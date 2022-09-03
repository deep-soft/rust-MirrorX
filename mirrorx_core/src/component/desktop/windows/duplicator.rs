use super::{
    dx_math::{VERTEX_STRIDES, VERTICES},
    shader,
};
use crate::{
    api_error,
    component::{
        capture_frame::CaptureFrame,
        desktop::windows::dx_math::{BPP, VERTEX},
    },
    error::{CoreResult, MirrorXError},
    utility::wide_char::FromWide,
    windows_api_check,
};
use scopeguard::defer;
use std::{ffi::OsString, os::raw::c_void};
use tracing::info;
use windows::{
    core::{Interface, PCSTR},
    Win32::{
        Graphics::{
            Direct3D::*,
            Direct3D11::*,
            Dxgi::{Common::*, *},
            Gdi::*,
        },
        System::{StationsAndDesktops::*, SystemServices::*, WindowsProgramming::INFINITE},
        UI::WindowsAndMessaging::*,
    },
};

pub struct Duplicator {
    device: ID3D11Device,
    device_context: ID3D11DeviceContext,

    vertex_shader: ID3D11VertexShader,
    vertex_buffer: Option<ID3D11Buffer>,

    pixel_shader: ID3D11PixelShader,
    pixel_shader_lumina: ID3D11PixelShader,
    pixel_shader_chrominance: ID3D11PixelShader,

    duplication: IDXGIOutputDuplication,
    dxgi_outdupl_desc: DXGI_OUTDUPL_DESC,

    backend_texture: ID3D11Texture2D,
    backend_viewport: [D3D11_VIEWPORT; 1],
    backend_rtv: [Option<ID3D11RenderTargetView>; 1],

    lumina_render_texture: ID3D11Texture2D,
    lumina_staging_texture: ID3D11Texture2D,
    lumina_viewport: [D3D11_VIEWPORT; 1],
    lumina_rtv: [Option<ID3D11RenderTargetView>; 1],

    chrominance_render_texture: ID3D11Texture2D,
    chrominance_staging_texture: ID3D11Texture2D,
    chrominance_viewport: [D3D11_VIEWPORT; 1],
    chrominance_rtv: [Option<ID3D11RenderTargetView>; 1],

    sampler_state: [Option<ID3D11SamplerState>; 1],
    blend_state: ID3D11BlendState,
    input_layout: ID3D11InputLayout,
}

unsafe impl Send for Duplicator {}

impl Duplicator {
    pub fn new(monitor_id: Option<String>) -> CoreResult<Duplicator> {
        unsafe {
            prepare_desktop()?;

            let (device, device_context) = init_directx()?;

            let (
                vertex_shader,
                vertex_buffer,
                pixel_shader,
                pixel_shader_lumina,
                pixel_shader_chrominance,
            ) = init_shaders(&device)?;

            let duplication = init_output_duplication(&device, monitor_id)?;

            let mut dxgi_outdupl_desc = std::mem::zeroed();
            duplication.GetDesc(&mut dxgi_outdupl_desc);

            let (backend_texture, backend_rtv, backend_viewport) =
                init_backend_resources(&device, &dxgi_outdupl_desc)?;

            let (lumina_render_texture, lumina_staging_texture, lumina_viewport, lumina_rtv) =
                init_lumina_resources(&device, &dxgi_outdupl_desc)?;

            let (
                chrominance_render_texture,
                chrominance_staging_texture,
                chrominance_viewport,
                chrominance_rtv,
            ) = init_chrominance_resources(&device, &dxgi_outdupl_desc)?;

            let sampler_state = init_sampler_state(&device)?;

            let blend_state = init_blend_state(&device)?;

            let input_layout = init_input_layout(&device)?;

            Ok(Duplicator {
                device,
                device_context,
                vertex_shader,
                vertex_buffer: Some(vertex_buffer),
                pixel_shader,
                pixel_shader_lumina,
                pixel_shader_chrominance,
                duplication,
                dxgi_outdupl_desc,
                backend_texture,
                backend_viewport: [backend_viewport],
                backend_rtv: [Some(backend_rtv)],
                lumina_render_texture,
                lumina_staging_texture,
                lumina_viewport: [lumina_viewport],
                lumina_rtv: [Some(lumina_rtv)],
                chrominance_render_texture,
                chrominance_staging_texture,
                chrominance_viewport: [chrominance_viewport],
                chrominance_rtv: [Some(chrominance_rtv)],
                sampler_state: [Some(sampler_state)],
                blend_state,
                input_layout,
            })
        }
    }

    pub fn capture(&self) -> CoreResult<CaptureFrame> {
        unsafe {
            self.acquire_frame()?;
            self.draw_lumina_and_chrominance_texture()?;
            self.create_capture_frame()
        }
    }

    unsafe fn acquire_frame(&self) -> CoreResult<()> {
        let mut dxgi_resource = None;
        let mut dxgi_outdupl_frame_info = std::mem::zeroed();

        let mut failures = 0;
        while failures < 10 {
            let hr = match self.duplication.AcquireNextFrame(
                INFINITE,
                &mut dxgi_outdupl_frame_info,
                &mut dxgi_resource,
            ) {
                Ok(_) => break,
                Err(err) => {
                    failures += 1;
                    err.code()
                }
            };

            if failures > 10 {
                return Err(api_error!("too many failures on DXGI acquire frame"));
            }

            if hr == DXGI_ERROR_ACCESS_LOST {
                tracing::warn!("Duplication: IDXGIOutputDuplication::AcquireNextFrame returns DXGI_ERROR_ACCESS_LOST, re-init DXGIOutputDuplication");

                // todo

                // let _ = self.output_duplication.ReleaseFrame();

                // std::ptr::drop_in_place(&mut self.output_duplication);

                // let (dxgi_output_desc, dxgi_output_duplication) =
                //     init_output_duplication(&self.dx, 0)?;

                // self.output_duplication = dxgi_output_duplication;
                // self.output_desc = dxgi_output_desc;
            }
        }

        defer! {
            if let Err(err) = self.duplication.ReleaseFrame() {
                tracing::error!("DXGI release frame failed ({:?})", err.code());
            }
        }

        if let Some(resource) = dxgi_resource {
            let desktop_texture: ID3D11Texture2D = windows_api_check!(resource.cast());

            self.device_context
                .CopyResource(&self.backend_texture, desktop_texture);

            if dxgi_outdupl_frame_info.PointerPosition.Visible.as_bool() {
                self.draw_mouse(&dxgi_outdupl_frame_info)?;
            }

            Ok(())
        } else {
            Err(api_error!("DXGI frame resource is None"))
        }
    }

    unsafe fn draw_lumina_and_chrominance_texture(&self) -> CoreResult<()> {
        let mut backend_texture_desc = std::mem::zeroed();
        self.backend_texture.GetDesc(&mut backend_texture_desc);

        let shader_resouce_view_desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
            Format: backend_texture_desc.Format,
            ViewDimension: D3D11_SRV_DIMENSION_TEXTURE2D,
            Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture2D: D3D11_TEX2D_SRV {
                    MostDetailedMip: backend_texture_desc.MipLevels - 1,
                    MipLevels: backend_texture_desc.MipLevels,
                },
            },
        };

        let shader_resouce_view = windows_api_check!(self
            .device
            .CreateShaderResourceView(&self.backend_texture, &shader_resouce_view_desc));

        self.device_context
            .IASetVertexBuffers(0, 1, &self.vertex_buffer, &VERTEX_STRIDES, &0);

        self.device_context
            .IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

        self.device_context.IASetInputLayout(&self.input_layout);

        self.device_context
            .PSSetShaderResources(0, &vec![Some(shader_resouce_view)]);

        // draw lumina plane

        self.device_context
            .OMSetRenderTargets(&self.lumina_rtv, None);

        self.device_context
            .PSSetShader(&self.pixel_shader_lumina, &[]);

        self.device_context.RSSetViewports(&self.lumina_viewport);

        self.device_context.Draw(VERTICES.len() as u32, 0);

        // draw chrominance plane

        self.device_context
            .OMSetRenderTargets(&self.chrominance_rtv, None);

        self.device_context
            .PSSetShader(&self.pixel_shader_chrominance, &[]);

        self.device_context
            .RSSetViewports(&self.chrominance_viewport);

        self.device_context.Draw(VERTICES.len() as u32, 0);

        self.device_context.Flush();

        Ok(())
    }

    unsafe fn create_capture_frame(&self) -> CoreResult<CaptureFrame> {
        self.device_context
            .CopyResource(&self.lumina_staging_texture, &self.lumina_render_texture);

        self.device_context.CopyResource(
            &self.chrominance_staging_texture,
            &self.chrominance_render_texture,
        );

        let lumina_mapped_resource = windows_api_check!(self.device_context.Map(
            &self.lumina_staging_texture,
            0,
            D3D11_MAP_READ,
            0
        ));

        let lumina_stride = lumina_mapped_resource.RowPitch;

        let lumina_bytes = std::slice::from_raw_parts(
            lumina_mapped_resource.pData as *mut u8,
            (self.dxgi_outdupl_desc.ModeDesc.Height * lumina_stride) as usize,
        )
        .to_vec();

        self.device_context.Unmap(&self.lumina_staging_texture, 0);

        let chrominance_mapped_resource = windows_api_check!(self.device_context.Map(
            &self.chrominance_staging_texture,
            0,
            D3D11_MAP_READ,
            0
        ));

        let chrominance_stride = chrominance_mapped_resource.RowPitch;

        let chrominance_bytes = std::slice::from_raw_parts(
            chrominance_mapped_resource.pData as *mut u8,
            (self.dxgi_outdupl_desc.ModeDesc.Height / 2 * chrominance_stride) as usize,
        )
        .to_vec();

        self.device_context
            .Unmap(&self.chrominance_staging_texture, 0);

        Ok(CaptureFrame {
            width: self.dxgi_outdupl_desc.ModeDesc.Width,
            height: self.dxgi_outdupl_desc.ModeDesc.Height,
            lumina_bytes,
            lumina_stride,
            chrominance_bytes,
            chrominance_stride,
        })
    }

    unsafe fn draw_mouse(&self, desktop_frame_info: &DXGI_OUTDUPL_FRAME_INFO) -> CoreResult<()> {
        let mut cursor_shape_buffer =
            Vec::with_capacity(desktop_frame_info.PointerShapeBufferSize as usize);
        let mut cursor_shape_buffer_length = 0u32;
        let mut cursor_shape_info: DXGI_OUTDUPL_POINTER_SHAPE_INFO = std::mem::zeroed();

        windows_api_check!(self.duplication.GetFramePointerShape(
            desktop_frame_info.PointerShapeBufferSize,
            cursor_shape_buffer.as_mut_ptr() as *mut _,
            &mut cursor_shape_buffer_length,
            &mut cursor_shape_info,
        ));

        cursor_shape_buffer.set_len(desktop_frame_info.PointerShapeBufferSize as usize);

        let mut full_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
        self.backend_texture.GetDesc(&mut full_desc);

        let desktop_width = full_desc.Width;
        let desktop_height = full_desc.Height;

        let center_x = (desktop_width / 2) as i32;
        let center_y = (desktop_height / 2) as i32;

        let (mut pointer_width, mut pointer_height, mut pointer_left, mut pointer_top) =
            (0i32, 0i32, 0i32, 0i32);

        let mut pointer_box: D3D11_BOX = std::mem::zeroed();
        pointer_box.front = 0;
        pointer_box.back = 1;

        let mut pointer_texture_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
        pointer_texture_desc.MipLevels = 1;
        pointer_texture_desc.ArraySize = 1;
        pointer_texture_desc.Format = full_desc.Format;
        pointer_texture_desc.SampleDesc.Count = 1;
        pointer_texture_desc.SampleDesc.Quality = 0;
        pointer_texture_desc.Usage = D3D11_USAGE_DEFAULT;
        pointer_texture_desc.BindFlags = D3D11_BIND_SHADER_RESOURCE;

        let mut shader_resource_view_desc: D3D11_SHADER_RESOURCE_VIEW_DESC = std::mem::zeroed();
        shader_resource_view_desc.Format = pointer_texture_desc.Format;
        shader_resource_view_desc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURE2D;
        shader_resource_view_desc
            .Anonymous
            .Texture2D
            .MostDetailedMip = pointer_texture_desc.MipLevels - 1;
        shader_resource_view_desc.Anonymous.Texture2D.MipLevels = pointer_texture_desc.MipLevels;

        let mut init_buffer = std::ptr::null();

        match DXGI_OUTDUPL_POINTER_SHAPE_TYPE(cursor_shape_info.Type as i32) {
            DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR => {
                tracing::info!(
                    "DXGI_OUTDUPL_POINTER_SHAPE_INFO: DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR"
                );

                pointer_left = desktop_frame_info.PointerPosition.Position.x as i32;
                pointer_top = desktop_frame_info.PointerPosition.Position.y as i32;
                pointer_width = cursor_shape_info.Width as i32;
                pointer_height = cursor_shape_info.Height as i32;
            }
            DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME => {
                tracing::debug!(
                    "DXGI_OUTDUPL_POINTER_SHAPE_INFO: DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME"
                );

                let buffer = self.process_mono_mask(
                    true,
                    &full_desc,
                    &mut cursor_shape_info,
                    desktop_frame_info,
                    &mut cursor_shape_buffer,
                    &mut pointer_width,
                    &mut pointer_height,
                    &mut pointer_left,
                    &mut pointer_top,
                    &mut pointer_box,
                )?;

                init_buffer = buffer.as_ptr()
            }
            DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR => {
                tracing::debug!(
                    "DXGI_OUTDUPL_POINTER_SHAPE_INFO: DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR"
                );

                let buffer = self.process_mono_mask(
                    false,
                    &full_desc,
                    &mut cursor_shape_info,
                    desktop_frame_info,
                    &mut cursor_shape_buffer,
                    &mut pointer_width,
                    &mut pointer_height,
                    &mut pointer_left,
                    &mut pointer_top,
                    &mut pointer_box,
                )?;

                init_buffer = buffer.as_ptr()
            }
            _ => {}
        };

        let mut vertices = VERTICES.clone();
        vertices[0].pos.x = (pointer_left - center_x) as f32 / center_x as f32;
        vertices[0].pos.y =
            -1f32 * (pointer_top + pointer_height - center_y) as f32 / center_y as f32;

        vertices[1].pos.x = (pointer_left - center_x) as f32 / center_x as f32;
        vertices[1].pos.y = -1f32 * (pointer_top - center_y) as f32 / center_y as f32;

        vertices[2].pos.x = (pointer_left + pointer_width - center_x) as f32 / center_x as f32;
        vertices[2].pos.y =
            -1f32 * (pointer_top + pointer_height - center_y) as f32 / center_y as f32;

        vertices[3].pos.x = vertices[2].pos.x;
        vertices[3].pos.y = vertices[2].pos.y;

        vertices[4].pos.x = vertices[1].pos.x;
        vertices[4].pos.y = vertices[1].pos.y;

        vertices[5].pos.x = (pointer_left + pointer_width - center_x) as f32 / center_x as f32;
        vertices[5].pos.y = -1f32 * (pointer_top - center_y) as f32 / center_y as f32;

        pointer_texture_desc.Width = pointer_width as u32;
        pointer_texture_desc.Height = pointer_height as u32;

        let mut init_data: D3D11_SUBRESOURCE_DATA = std::mem::zeroed();
        init_data.pSysMem =
            if cursor_shape_info.Type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR.0 as u32 {
                cursor_shape_buffer.as_ptr() as *const _
            } else {
                init_buffer as *const _
            };
        init_data.SysMemPitch =
            if cursor_shape_info.Type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR.0 as u32 {
                cursor_shape_info.Pitch
            } else {
                pointer_width as u32 * BPP
            };
        init_data.SysMemSlicePitch = 0;

        let pointer_texture = windows_api_check!(self
            .device
            .CreateTexture2D(&pointer_texture_desc, &init_data));

        let shader_res = windows_api_check!(self
            .device
            .CreateShaderResourceView(&pointer_texture, &shader_resource_view_desc));

        let mut buffer_desc: D3D11_BUFFER_DESC = std::mem::zeroed();
        buffer_desc.Usage = D3D11_USAGE_DEFAULT;
        buffer_desc.ByteWidth = (std::mem::size_of::<VERTEX>() * VERTICES.len()) as u32;
        buffer_desc.BindFlags = D3D11_BIND_VERTEX_BUFFER.0;

        init_data = std::mem::zeroed();
        init_data.pSysMem = vertices.as_ptr() as *const _;

        let vertex_buffer = Some(windows_api_check!(self
            .device
            .CreateBuffer(&buffer_desc, &init_data)));

        let blend_factor = [0f32; 4];
        let stride = std::mem::size_of::<VERTEX>() as u32;
        let offset = 0;

        self.device_context
            .IASetVertexBuffers(0, 1, [vertex_buffer].as_ptr(), &stride, &offset);

        self.device_context
            .IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

        self.device_context.IASetInputLayout(&self.input_layout);

        self.device_context
            .OMSetBlendState(&self.blend_state, blend_factor.as_ptr(), 0xFFFFFFFF);

        self.device_context
            .OMSetRenderTargets(&self.backend_rtv, None);

        self.device_context.VSSetShader(&self.vertex_shader, &[]);

        self.device_context.PSSetShader(&self.pixel_shader, &[]);

        self.device_context
            .PSSetShaderResources(0, &[Some(shader_res)]);

        self.device_context.PSSetSamplers(0, &self.sampler_state);

        self.device_context.RSSetViewports(&self.backend_viewport);

        self.device_context.Draw(VERTICES.len() as u32, 0);

        self.device_context.Flush();

        Ok(())
    }

    unsafe fn process_mono_mask(
        &self,
        is_mono: bool,
        full_desc: &D3D11_TEXTURE2D_DESC,
        pointer_info: &mut DXGI_OUTDUPL_POINTER_SHAPE_INFO,
        frame_info: &DXGI_OUTDUPL_FRAME_INFO,
        pointer_shape_buffer: &mut [u8],
        pointer_width: &mut i32,
        pointer_height: &mut i32,
        pointer_left: &mut i32,
        pointer_top: &mut i32,
        pointer_box: &mut D3D11_BOX,
    ) -> CoreResult<Vec<u8>> {
        let desktop_width = full_desc.Width as i32;
        let desktop_height = full_desc.Height as i32;

        let given_left = frame_info.PointerPosition.Position.x;
        let given_top = frame_info.PointerPosition.Position.y;

        if given_left < 0 {
            *pointer_width = given_left + pointer_info.Width as i32;
        } else if given_left + (pointer_info.Width as i32) > desktop_width {
            *pointer_width = desktop_width - given_left;
        } else {
            *pointer_width = pointer_info.Width as i32;
        }

        if is_mono {
            pointer_info.Height /= 2;
        }

        if given_top < 0 {
            *pointer_height = given_top + pointer_info.Height as i32;
        } else if given_top + (pointer_info.Height as i32) > desktop_height {
            *pointer_height = desktop_height - given_top;
        } else {
            *pointer_height = pointer_info.Height as i32;
        }

        if is_mono {
            pointer_info.Height *= 2;
        }

        *pointer_left = given_left.max(0);
        *pointer_top = given_top.max(0);

        let mut copy_buffer_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
        copy_buffer_desc.Width = *pointer_width as u32;
        copy_buffer_desc.Height = *pointer_height as u32;
        copy_buffer_desc.MipLevels = 1;
        copy_buffer_desc.ArraySize = 1;
        copy_buffer_desc.Format = DXGI_FORMAT_B8G8R8A8_UNORM;
        copy_buffer_desc.SampleDesc.Count = 1;
        copy_buffer_desc.SampleDesc.Quality = 0;
        copy_buffer_desc.Usage = D3D11_USAGE_STAGING;
        copy_buffer_desc.BindFlags = D3D11_BIND_FLAG::default();
        copy_buffer_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
        copy_buffer_desc.MiscFlags = D3D11_RESOURCE_MISC_FLAG::default();

        let copy_buffer = windows_api_check!(self
            .device
            .CreateTexture2D(&copy_buffer_desc, std::ptr::null()));

        (*pointer_box).left = *pointer_left as u32;
        (*pointer_box).top = *pointer_top as u32;
        (*pointer_box).right = (*pointer_left + *pointer_width) as u32;
        (*pointer_box).bottom = (*pointer_top + *pointer_height) as u32;

        self.device_context.CopySubresourceRegion(
            &copy_buffer,
            0,
            0,
            0,
            0,
            &self.backend_texture,
            0,
            pointer_box,
        );

        let copy_surface: IDXGISurface = windows_api_check!(copy_buffer.cast());

        let mut mapped_surface: DXGI_MAPPED_RECT = std::mem::zeroed();
        windows_api_check!(copy_surface.Map(&mut mapped_surface, DXGI_MAP_READ));

        defer! {
            let _ = copy_surface.Unmap();
        }

        let mut init_buffer = Vec::new();
        init_buffer.resize(
            ((*pointer_width as u32) * (*pointer_height as u32) * BPP) as usize,
            0,
        );

        let init_buffer_32: *mut u32 = std::mem::transmute(init_buffer.as_mut_ptr());
        let desktop_32: *mut u32 = std::mem::transmute(mapped_surface.pBits);
        let desktop_pitch_in_pixels = mapped_surface.Pitch / 4;

        let skip_x = if given_left < 0 { -1 * given_left } else { 0 };
        let skip_y = if given_top < 0 { -1 * given_top } else { 0 };

        if is_mono {
            for row in 0..*pointer_height {
                let mut mask = 0x80u8;
                mask = mask.wrapping_shr((skip_x % 8) as u32);

                for col in 0..*pointer_width {
                    let and_mask: u8 = pointer_shape_buffer[((col + skip_x) / 8
                        + (row + skip_y) * (pointer_info.Pitch as i32))
                        as usize]
                        & mask;

                    let xor_mask: u8 = pointer_shape_buffer[((col + skip_x) / 8
                        + (row + skip_y + ((pointer_info.Height / 2) as i32))
                            * (pointer_info.Pitch as i32))
                        as usize]
                        & mask;

                    let and_mask_32: u32 = if and_mask > 0 { 0xFFFFFFFF } else { 0xFF000000 };
                    let xor_mask_32: u32 = if xor_mask > 0 { 0x00FFFFFF } else { 0x00000000 };

                    (*init_buffer_32.add(((row * *pointer_width) + col) as usize)) = (*desktop_32
                        .add(((row * desktop_pitch_in_pixels) + col) as usize)
                        & and_mask_32)
                        ^ xor_mask_32;

                    if mask == 0x01 {
                        mask = 0x80;
                    } else {
                        mask = mask.wrapping_shr(1);
                    }
                }
            }
        } else {
            let buffer_32: *mut u32 = std::mem::transmute(pointer_shape_buffer.as_mut_ptr());
            for row in 0..*pointer_height {
                for col in 0..*pointer_width {
                    let mask_val: u32 = 0xFF000000
                        & *buffer_32.add(
                            ((col + skip_x) + ((row + skip_y) * (pointer_info.Pitch as i32 / 4)))
                                as usize,
                        );

                    if mask_val > 0 {
                        // Mask was 0xFF
                        *buffer_32.add(((row * *pointer_width) + col) as usize) = (*desktop_32
                            .add(((row * desktop_pitch_in_pixels) + col) as usize)
                            ^ *buffer_32.add(
                                ((col + skip_x)
                                    + ((row + skip_y) * (pointer_info.Pitch as i32 / 4)))
                                    as usize,
                            ))
                            | 0xFF000000;
                    } else {
                        // Mask was 0x00
                        *buffer_32.add(((row * *pointer_width) + col) as usize) = *buffer_32.add(
                            ((col + skip_x) + ((row + skip_y) * (pointer_info.Pitch as i32 / 4)))
                                as usize,
                        ) | 0xFF000000;
                    }
                }
            }
        }

        Ok(init_buffer)
    }
}

unsafe fn prepare_desktop() -> CoreResult<()> {
    let current_desktop = windows_api_check!(OpenInputDesktop(0, false, GENERIC_ALL));

    defer! {
        let _ = CloseDesktop(current_desktop);
    }

    if !SetThreadDesktop(current_desktop).as_bool() {
        return Err(api_error!(
            "SetThreadDesktop returns false when prepare desktop"
        ));
    }

    Ok(())
}

unsafe fn init_directx() -> CoreResult<(ID3D11Device, ID3D11DeviceContext)> {
    let driver_types = [
        D3D_DRIVER_TYPE_HARDWARE,
        D3D_DRIVER_TYPE_WARP,
        D3D_DRIVER_TYPE_REFERENCE,
    ];

    let mut device = None;
    let mut device_context = None;
    let mut feature_level = std::mem::zeroed();

    for driver_type in driver_types {
        match D3D11CreateDevice(
            None,
            driver_type,
            None,
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            &[],
            D3D11_SDK_VERSION,
            &mut device,
            &mut feature_level,
            &mut device_context,
        ) {
            Ok(_) => {
                let driver_type_name = match driver_type {
                    D3D_DRIVER_TYPE_UNKNOWN => "D3D_DRIVER_TYPE_UNKNOWN",
                    D3D_DRIVER_TYPE_HARDWARE => "D3D_DRIVER_TYPE_HARDWARE",
                    D3D_DRIVER_TYPE_REFERENCE => "D3D_DRIVER_TYPE_REFERENCE",
                    D3D_DRIVER_TYPE_NULL => "D3D_DRIVER_TYPE_NULL",
                    D3D_DRIVER_TYPE_SOFTWARE => "D3D_DRIVER_TYPE_SOFTWARE",
                    D3D_DRIVER_TYPE_WARP => "D3D_DRIVER_TYPE_WARP",
                    _ => "Unknown",
                };

                let feature_level_name = match feature_level {
                    D3D_FEATURE_LEVEL_12_2 => "D3D_FEATURE_LEVEL_12_2",
                    D3D_FEATURE_LEVEL_12_1 => "D3D_FEATURE_LEVEL_12_1",
                    D3D_FEATURE_LEVEL_12_0 => "D3D_FEATURE_LEVEL_12_0",
                    D3D_FEATURE_LEVEL_11_1 => "D3D_FEATURE_LEVEL_11_1",
                    D3D_FEATURE_LEVEL_11_0 => "D3D_FEATURE_LEVEL_11_0",
                    D3D_FEATURE_LEVEL_10_1 => "D3D_FEATURE_LEVEL_10_1",
                    D3D_FEATURE_LEVEL_10_0 => "D3D_FEATURE_LEVEL_10_0",
                    D3D_FEATURE_LEVEL_9_3 => "D3D_FEATURE_LEVEL_9_3",
                    D3D_FEATURE_LEVEL_9_2 => "D3D_FEATURE_LEVEL_9_2",
                    D3D_FEATURE_LEVEL_9_1 => "D3D_FEATURE_LEVEL_9_1",
                    D3D_FEATURE_LEVEL_1_0_CORE => "D3D_FEATURE_LEVEL_1_0_CORE",
                    _ => "Unknown",
                };

                tracing::info!(
                    ?driver_type_name,
                    ?feature_level_name,
                    "create DirectX device successfully"
                );

                break;
            }
            Err(err) => {
                let driver_type_name = match driver_type {
                    D3D_DRIVER_TYPE_UNKNOWN => "D3D_DRIVER_TYPE_UNKNOWN",
                    D3D_DRIVER_TYPE_HARDWARE => "D3D_DRIVER_TYPE_HARDWARE",
                    D3D_DRIVER_TYPE_REFERENCE => "D3D_DRIVER_TYPE_REFERENCE",
                    D3D_DRIVER_TYPE_NULL => "D3D_DRIVER_TYPE_NULL",
                    D3D_DRIVER_TYPE_SOFTWARE => "D3D_DRIVER_TYPE_SOFTWARE",
                    D3D_DRIVER_TYPE_WARP => "D3D_DRIVER_TYPE_WARP",
                    _ => "Unknown",
                };

                tracing::info!(
                    ?driver_type_name,
                    ?err,
                    "create DirectX device failed, try next one"
                );
            }
        };
    }

    if let (Some(device), Some(device_context)) = (device, device_context) {
        Ok((device, device_context))
    } else {
        Err(MirrorXError::Other(anyhow::anyhow!(
            "create DirectX device failed, all driver types had tried"
        )))
    }
}

unsafe fn init_output_duplication(
    device: &ID3D11Device,
    monitor_id: Option<String>,
) -> CoreResult<IDXGIOutputDuplication> {
    let dxgi_device: IDXGIDevice = windows_api_check!(device.cast());

    let dxgi_adapter = windows_api_check!(dxgi_device.GetParent::<IDXGIAdapter>());

    let adapter_desc = windows_api_check!(dxgi_adapter.GetDesc());

    info!(
        name = ?OsString::from_wide_null(&adapter_desc.Description),
        "DXGI Adapter",
    );

    let mut output_index = 0;

    while let Ok(dxgi_output) = dxgi_adapter.EnumOutputs(output_index) {
        output_index += 1;

        let dxgi_output_desc = windows_api_check!(dxgi_output.GetDesc());

        if !dxgi_output_desc.AttachedToDesktop.as_bool() {
            continue;
        }

        let mut dev_index = 0u32;
        loop {
            let origin_device_name = OsString::from_wide_null(&dxgi_output_desc.DeviceName);

            let mut display_device: DISPLAY_DEVICEW = std::mem::zeroed();
            display_device.cb = std::mem::size_of::<DISPLAY_DEVICEW>() as u32;

            let success = EnumDisplayDevicesW(
                &*origin_device_name,
                dev_index,
                &mut display_device as *mut _,
                EDD_GET_DEVICE_INTERFACE_NAME,
            )
            .as_bool();

            dev_index += 1;

            if !success {
                break;
            }

            if (display_device.StateFlags & DISPLAY_DEVICE_ATTACHED_TO_DESKTOP) != 0 {
                let device_id = OsString::from_wide_null(&display_device.DeviceID)
                    .into_string()
                    .map_err(|_| MirrorXError::API {
                        message: String::from("convert OsString to String failed"),
                        file: file!().to_string(),
                        line: line!().to_string(),
                    })?;

                if let Some(ref id) = monitor_id {
                    if *id != device_id {
                        continue;
                    }
                }

                let dxgi_output1: IDXGIOutput1 = windows_api_check!(dxgi_output.cast());

                let dxgi_output_duplication =
                    windows_api_check!(dxgi_output1.DuplicateOutput(device));

                return Ok(dxgi_output_duplication);
            }
        }
    }

    Err(api_error!(
        "create IDXGIOutputDuplication failed, all Outputs had tried"
    ))
}

unsafe fn init_shaders(
    device: &ID3D11Device,
) -> CoreResult<(
    ID3D11VertexShader,
    ID3D11Buffer,
    ID3D11PixelShader,
    ID3D11PixelShader,
    ID3D11PixelShader,
)> {
    let vertex_shader =
        windows_api_check!(device.CreateVertexShader(shader::VERTEX_SHADER_BYTES, None));

    let vertex_buffer_desc = D3D11_BUFFER_DESC {
        ByteWidth: VERTEX_STRIDES * VERTICES.len() as u32,
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: D3D11_BIND_VERTEX_BUFFER.0,
        CPUAccessFlags: 0,
        MiscFlags: 0,
        StructureByteStride: 0,
    };

    let subresource_data = D3D11_SUBRESOURCE_DATA {
        pSysMem: &VERTICES as *const _ as *const c_void,
        SysMemPitch: 0,
        SysMemSlicePitch: 0,
    };

    let vertex_buffer =
        windows_api_check!(device.CreateBuffer(&vertex_buffer_desc, &subresource_data));

    let pixel_shader =
        windows_api_check!(device.CreatePixelShader(shader::PIXEL_SHADER_BYTES, None));

    let pixel_shader_lumina =
        windows_api_check!(device.CreatePixelShader(shader::PIXEL_SHADER_LUMINA_BYTES, None));

    let pixel_shader_chrominance =
        windows_api_check!(device.CreatePixelShader(shader::PIXEL_SHADER_CHROMINANCE_BYTES, None));

    Ok((
        vertex_shader,
        vertex_buffer,
        pixel_shader,
        pixel_shader_lumina,
        pixel_shader_chrominance,
    ))
}

unsafe fn init_backend_resources(
    device: &ID3D11Device,
    dxgi_outdupl_desc: &DXGI_OUTDUPL_DESC,
) -> CoreResult<(ID3D11Texture2D, ID3D11RenderTargetView, D3D11_VIEWPORT)> {
    let mut texture_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
    texture_desc.Width = dxgi_outdupl_desc.ModeDesc.Width;
    texture_desc.Height = dxgi_outdupl_desc.ModeDesc.Height;
    texture_desc.MipLevels = 1;
    texture_desc.ArraySize = 1;
    texture_desc.Format = DXGI_FORMAT_B8G8R8A8_UNORM;
    texture_desc.SampleDesc.Count = 1;
    texture_desc.SampleDesc.Quality = 0;
    texture_desc.Usage = D3D11_USAGE_DEFAULT;
    texture_desc.BindFlags = D3D11_BIND_RENDER_TARGET | D3D11_BIND_SHADER_RESOURCE;

    let texture = windows_api_check!(device.CreateTexture2D(&texture_desc, std::ptr::null()));

    texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
    texture_desc.Usage = D3D11_USAGE_STAGING;
    texture_desc.BindFlags = D3D11_BIND_FLAG::default();

    let backend_staging_texture =
        windows_api_check!(device.CreateTexture2D(&texture_desc, std::ptr::null()));

    let rtv = windows_api_check!(device.CreateRenderTargetView(&texture, std::ptr::null()));

    let viewport = D3D11_VIEWPORT {
        TopLeftX: 0.0,
        TopLeftY: 0.0,
        Width: texture_desc.Width as f32,
        Height: texture_desc.Height as f32,
        MinDepth: 0.0,
        MaxDepth: 1.0,
    };

    Ok((texture, rtv, viewport))
}

unsafe fn init_lumina_resources(
    device: &ID3D11Device,
    dxgi_outdupl_desc: &DXGI_OUTDUPL_DESC,
) -> CoreResult<(
    ID3D11Texture2D,
    ID3D11Texture2D,
    D3D11_VIEWPORT,
    ID3D11RenderTargetView,
)> {
    let mut texture_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
    texture_desc.Width = dxgi_outdupl_desc.ModeDesc.Width;
    texture_desc.Height = dxgi_outdupl_desc.ModeDesc.Height;
    texture_desc.MipLevels = 1;
    texture_desc.ArraySize = 1;
    texture_desc.Format = DXGI_FORMAT_R8_UNORM;
    texture_desc.SampleDesc.Count = 1;
    texture_desc.SampleDesc.Quality = 0;
    texture_desc.Usage = D3D11_USAGE_DEFAULT;
    texture_desc.BindFlags = D3D11_BIND_RENDER_TARGET;

    let render_texture =
        windows_api_check!(device.CreateTexture2D(&texture_desc, std::ptr::null()));

    texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
    texture_desc.Usage = D3D11_USAGE_STAGING;
    texture_desc.BindFlags = D3D11_BIND_FLAG::default();

    let staging_texture =
        windows_api_check!(device.CreateTexture2D(&texture_desc, std::ptr::null()));

    let viewport = D3D11_VIEWPORT {
        TopLeftX: 0.0,
        TopLeftY: 0.0,
        Width: dxgi_outdupl_desc.ModeDesc.Width as f32,
        Height: dxgi_outdupl_desc.ModeDesc.Height as f32,
        MinDepth: 0.0,
        MaxDepth: 1.0,
    };

    let rtv = windows_api_check!(device.CreateRenderTargetView(&render_texture, std::ptr::null()));

    Ok((render_texture, staging_texture, viewport, rtv))
}

unsafe fn init_chrominance_resources(
    device: &ID3D11Device,
    dxgi_outdupl_desc: &DXGI_OUTDUPL_DESC,
) -> CoreResult<(
    ID3D11Texture2D,
    ID3D11Texture2D,
    D3D11_VIEWPORT,
    ID3D11RenderTargetView,
)> {
    let mut texture_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
    texture_desc.Width = dxgi_outdupl_desc.ModeDesc.Width / 2;
    texture_desc.Height = dxgi_outdupl_desc.ModeDesc.Height / 2;
    texture_desc.MipLevels = 1;
    texture_desc.ArraySize = 1;
    texture_desc.Format = DXGI_FORMAT_R8G8_UNORM;
    texture_desc.SampleDesc.Count = 1;
    texture_desc.SampleDesc.Quality = 0;
    texture_desc.Usage = D3D11_USAGE_DEFAULT;
    texture_desc.BindFlags = D3D11_BIND_RENDER_TARGET;

    let render_texture =
        windows_api_check!(device.CreateTexture2D(&texture_desc, std::ptr::null()));

    texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
    texture_desc.Usage = D3D11_USAGE_STAGING;
    texture_desc.BindFlags = D3D11_BIND_FLAG::default();

    let staging_texture =
        windows_api_check!(device.CreateTexture2D(&texture_desc, std::ptr::null()));

    let viewport = D3D11_VIEWPORT {
        TopLeftX: 0.0,
        TopLeftY: 0.0,
        Width: (dxgi_outdupl_desc.ModeDesc.Width / 2) as f32,
        Height: (dxgi_outdupl_desc.ModeDesc.Height / 2) as f32,
        MinDepth: 0.0,
        MaxDepth: 1.0,
    };

    let rtv = windows_api_check!(device.CreateRenderTargetView(&render_texture, std::ptr::null()));

    Ok((render_texture, staging_texture, viewport, rtv))
}

unsafe fn init_sampler_state(device: &ID3D11Device) -> CoreResult<ID3D11SamplerState> {
    let mut sampler_desc: D3D11_SAMPLER_DESC = std::mem::zeroed();
    sampler_desc.Filter = D3D11_FILTER_MIN_MAG_MIP_LINEAR;
    sampler_desc.AddressU = D3D11_TEXTURE_ADDRESS_CLAMP;
    sampler_desc.AddressV = D3D11_TEXTURE_ADDRESS_CLAMP;
    sampler_desc.AddressW = D3D11_TEXTURE_ADDRESS_CLAMP;
    sampler_desc.ComparisonFunc = D3D11_COMPARISON_NEVER;
    sampler_desc.MinLOD = 0f32;
    sampler_desc.MaxLOD = D3D11_FLOAT32_MAX;

    let sampler_state = windows_api_check!(device.CreateSamplerState(&sampler_desc));

    Ok(sampler_state)
}

unsafe fn init_blend_state(device: &ID3D11Device) -> CoreResult<ID3D11BlendState> {
    let mut blend_desc: D3D11_BLEND_DESC = std::mem::zeroed();
    blend_desc.AlphaToCoverageEnable = true.into();
    blend_desc.IndependentBlendEnable = false.into();
    blend_desc.RenderTarget[0].BlendEnable = true.into();
    blend_desc.RenderTarget[0].SrcBlend = D3D11_BLEND_SRC_ALPHA;
    blend_desc.RenderTarget[0].DestBlend = D3D11_BLEND_INV_SRC_ALPHA;
    blend_desc.RenderTarget[0].BlendOp = D3D11_BLEND_OP_ADD;
    blend_desc.RenderTarget[0].SrcBlendAlpha = D3D11_BLEND_ONE;
    blend_desc.RenderTarget[0].DestBlendAlpha = D3D11_BLEND_ZERO;
    blend_desc.RenderTarget[0].BlendOpAlpha = D3D11_BLEND_OP_ADD;
    blend_desc.RenderTarget[0].RenderTargetWriteMask = D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8;

    let blend_state = windows_api_check!(device.CreateBlendState(&blend_desc));

    Ok(blend_state)
}

unsafe fn init_input_layout(device: &ID3D11Device) -> CoreResult<ID3D11InputLayout> {
    let input_element_desc_array = [
        D3D11_INPUT_ELEMENT_DESC {
            SemanticName: PCSTR(b"POSITION\0".as_ptr()),
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32B32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
        D3D11_INPUT_ELEMENT_DESC {
            SemanticName: PCSTR(b"TEXCOORD\0".as_ptr()),
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 12,
            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
    ];

    let input_layout = windows_api_check!(
        device.CreateInputLayout(&input_element_desc_array, shader::VERTEX_SHADER_BYTES)
    );

    Ok(input_layout)
}

#[test]
fn test_duplicator() {
    tracing_subscriber::fmt::init();

    let duplicator = Duplicator::new(None).unwrap();

    tracing::info!("begin capture frame");

    let capture_frame = duplicator.capture().unwrap();

    tracing::info!(
        width = capture_frame.width,
        height = capture_frame.height,
        "captured frame"
    );
}
