// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports

import 'dart:convert';
import 'dart:typed_data';

import 'dart:convert';
import 'dart:typed_data';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'dart:ffi' as ffi;

abstract class MirrorXCore {
  Future<void> init(
      {required String osType,
      required String osVersion,
      required String configDir,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kInitConstMeta;

  Future<String?> configReadDeviceId({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kConfigReadDeviceIdConstMeta;

  Future<void> configSaveDeviceId({required String deviceId, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kConfigSaveDeviceIdConstMeta;

  Future<int?> configReadDeviceIdExpiration({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kConfigReadDeviceIdExpirationConstMeta;

  Future<void> configSaveDeviceIdExpiration(
      {required int timeStamp, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kConfigSaveDeviceIdExpirationConstMeta;

  Future<String?> configReadDevicePassword({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kConfigReadDevicePasswordConstMeta;

  Future<void> configSaveDevicePassword(
      {required String devicePassword, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kConfigSaveDevicePasswordConstMeta;

  Future<bool> signalingConnect({required String remoteDeviceId, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kSignalingConnectConstMeta;

  Future<void> signalingConnectionKeyExchange(
      {required String remoteDeviceId, required String password, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kSignalingConnectionKeyExchangeConstMeta;

  Future<GetDisplayInfoResponse> endpointGetDisplayInfo(
      {required String remoteDeviceId, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kEndpointGetDisplayInfoConstMeta;

  Future<StartMediaTransmissionResponse> endpointStartMediaTransmission(
      {required String remoteDeviceId,
      required String displayId,
      required int textureId,
      required int videoTexturePtr,
      required int updateFrameCallbackPtr,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kEndpointStartMediaTransmissionConstMeta;
}

class DisplayInfo {
  final String id;
  final String name;
  final String refreshRate;
  final int width;
  final int height;
  final bool isPrimary;
  final Uint8List screenShot;

  DisplayInfo({
    required this.id,
    required this.name,
    required this.refreshRate,
    required this.width,
    required this.height,
    required this.isPrimary,
    required this.screenShot,
  });
}

class GetDisplayInfoResponse {
  final List<DisplayInfo> displays;

  GetDisplayInfoResponse({
    required this.displays,
  });
}

class StartMediaTransmissionResponse {
  final String osName;
  final String osVersion;
  final int screenWidth;
  final int screenHeight;
  final String videoType;
  final String audioType;

  StartMediaTransmissionResponse({
    required this.osName,
    required this.osVersion,
    required this.screenWidth,
    required this.screenHeight,
    required this.videoType,
    required this.audioType,
  });
}

class MirrorXCoreImpl extends FlutterRustBridgeBase<MirrorXCoreWire>
    implements MirrorXCore {
  factory MirrorXCoreImpl(ffi.DynamicLibrary dylib) =>
      MirrorXCoreImpl.raw(MirrorXCoreWire(dylib));

  MirrorXCoreImpl.raw(MirrorXCoreWire inner) : super(inner);

  Future<void> init(
          {required String osType,
          required String osVersion,
          required String configDir,
          dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_init(port_, _api2wire_String(osType),
            _api2wire_String(osVersion), _api2wire_String(configDir)),
        parseSuccessData: _wire2api_unit,
        constMeta: kInitConstMeta,
        argValues: [osType, osVersion, configDir],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kInitConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "init",
        argNames: ["osType", "osVersion", "configDir"],
      );

  Future<String?> configReadDeviceId({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_config_read_device_id(port_),
        parseSuccessData: _wire2api_opt_String,
        constMeta: kConfigReadDeviceIdConstMeta,
        argValues: [],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kConfigReadDeviceIdConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "config_read_device_id",
        argNames: [],
      );

  Future<void> configSaveDeviceId({required String deviceId, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) =>
            inner.wire_config_save_device_id(port_, _api2wire_String(deviceId)),
        parseSuccessData: _wire2api_unit,
        constMeta: kConfigSaveDeviceIdConstMeta,
        argValues: [deviceId],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kConfigSaveDeviceIdConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "config_save_device_id",
        argNames: ["deviceId"],
      );

  Future<int?> configReadDeviceIdExpiration({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_config_read_device_id_expiration(port_),
        parseSuccessData: _wire2api_opt_box_autoadd_u32,
        constMeta: kConfigReadDeviceIdExpirationConstMeta,
        argValues: [],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kConfigReadDeviceIdExpirationConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "config_read_device_id_expiration",
        argNames: [],
      );

  Future<void> configSaveDeviceIdExpiration(
          {required int timeStamp, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_config_save_device_id_expiration(
            port_, _api2wire_i32(timeStamp)),
        parseSuccessData: _wire2api_unit,
        constMeta: kConfigSaveDeviceIdExpirationConstMeta,
        argValues: [timeStamp],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kConfigSaveDeviceIdExpirationConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "config_save_device_id_expiration",
        argNames: ["timeStamp"],
      );

  Future<String?> configReadDevicePassword({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_config_read_device_password(port_),
        parseSuccessData: _wire2api_opt_String,
        constMeta: kConfigReadDevicePasswordConstMeta,
        argValues: [],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kConfigReadDevicePasswordConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "config_read_device_password",
        argNames: [],
      );

  Future<void> configSaveDevicePassword(
          {required String devicePassword, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_config_save_device_password(
            port_, _api2wire_String(devicePassword)),
        parseSuccessData: _wire2api_unit,
        constMeta: kConfigSaveDevicePasswordConstMeta,
        argValues: [devicePassword],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kConfigSaveDevicePasswordConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "config_save_device_password",
        argNames: ["devicePassword"],
      );

  Future<bool> signalingConnect(
          {required String remoteDeviceId, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_signaling_connect(
            port_, _api2wire_String(remoteDeviceId)),
        parseSuccessData: _wire2api_bool,
        constMeta: kSignalingConnectConstMeta,
        argValues: [remoteDeviceId],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kSignalingConnectConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "signaling_connect",
        argNames: ["remoteDeviceId"],
      );

  Future<void> signalingConnectionKeyExchange(
          {required String remoteDeviceId,
          required String password,
          dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_signaling_connection_key_exchange(port_,
            _api2wire_String(remoteDeviceId), _api2wire_String(password)),
        parseSuccessData: _wire2api_unit,
        constMeta: kSignalingConnectionKeyExchangeConstMeta,
        argValues: [remoteDeviceId, password],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kSignalingConnectionKeyExchangeConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "signaling_connection_key_exchange",
        argNames: ["remoteDeviceId", "password"],
      );

  Future<GetDisplayInfoResponse> endpointGetDisplayInfo(
          {required String remoteDeviceId, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_endpoint_get_display_info(
            port_, _api2wire_String(remoteDeviceId)),
        parseSuccessData: _wire2api_get_display_info_response,
        constMeta: kEndpointGetDisplayInfoConstMeta,
        argValues: [remoteDeviceId],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kEndpointGetDisplayInfoConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "endpoint_get_display_info",
        argNames: ["remoteDeviceId"],
      );

  Future<StartMediaTransmissionResponse> endpointStartMediaTransmission(
          {required String remoteDeviceId,
          required String displayId,
          required int textureId,
          required int videoTexturePtr,
          required int updateFrameCallbackPtr,
          dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_endpoint_start_media_transmission(
            port_,
            _api2wire_String(remoteDeviceId),
            _api2wire_String(displayId),
            _api2wire_i64(textureId),
            _api2wire_i64(videoTexturePtr),
            _api2wire_i64(updateFrameCallbackPtr)),
        parseSuccessData: _wire2api_start_media_transmission_response,
        constMeta: kEndpointStartMediaTransmissionConstMeta,
        argValues: [
          remoteDeviceId,
          displayId,
          textureId,
          videoTexturePtr,
          updateFrameCallbackPtr
        ],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kEndpointStartMediaTransmissionConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "endpoint_start_media_transmission",
        argNames: [
          "remoteDeviceId",
          "displayId",
          "textureId",
          "videoTexturePtr",
          "updateFrameCallbackPtr"
        ],
      );

  // Section: api2wire
  ffi.Pointer<wire_uint_8_list> _api2wire_String(String raw) {
    return _api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  int _api2wire_i32(int raw) {
    return raw;
  }

  int _api2wire_i64(int raw) {
    return raw;
  }

  int _api2wire_u8(int raw) {
    return raw;
  }

  ffi.Pointer<wire_uint_8_list> _api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  // Section: api_fill_to_wire

}

// Section: wire2api
String _wire2api_String(dynamic raw) {
  return raw as String;
}

bool _wire2api_bool(dynamic raw) {
  return raw as bool;
}

int _wire2api_box_autoadd_u32(dynamic raw) {
  return raw as int;
}

DisplayInfo _wire2api_display_info(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 7)
    throw Exception('unexpected arr length: expect 7 but see ${arr.length}');
  return DisplayInfo(
    id: _wire2api_String(arr[0]),
    name: _wire2api_String(arr[1]),
    refreshRate: _wire2api_String(arr[2]),
    width: _wire2api_u16(arr[3]),
    height: _wire2api_u16(arr[4]),
    isPrimary: _wire2api_bool(arr[5]),
    screenShot: _wire2api_uint_8_list(arr[6]),
  );
}

GetDisplayInfoResponse _wire2api_get_display_info_response(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return GetDisplayInfoResponse(
    displays: _wire2api_list_display_info(arr[0]),
  );
}

List<DisplayInfo> _wire2api_list_display_info(dynamic raw) {
  return (raw as List<dynamic>).map(_wire2api_display_info).toList();
}

String? _wire2api_opt_String(dynamic raw) {
  return raw == null ? null : _wire2api_String(raw);
}

int? _wire2api_opt_box_autoadd_u32(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_u32(raw);
}

StartMediaTransmissionResponse _wire2api_start_media_transmission_response(
    dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 6)
    throw Exception('unexpected arr length: expect 6 but see ${arr.length}');
  return StartMediaTransmissionResponse(
    osName: _wire2api_String(arr[0]),
    osVersion: _wire2api_String(arr[1]),
    screenWidth: _wire2api_u16(arr[2]),
    screenHeight: _wire2api_u16(arr[3]),
    videoType: _wire2api_String(arr[4]),
    audioType: _wire2api_String(arr[5]),
  );
}

int _wire2api_u16(dynamic raw) {
  return raw as int;
}

int _wire2api_u32(dynamic raw) {
  return raw as int;
}

int _wire2api_u8(dynamic raw) {
  return raw as int;
}

Uint8List _wire2api_uint_8_list(dynamic raw) {
  return raw as Uint8List;
}

void _wire2api_unit(dynamic raw) {
  return;
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.

/// generated by flutter_rust_bridge
class MirrorXCoreWire implements FlutterRustBridgeWireBase {
  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  MirrorXCoreWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  MirrorXCoreWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void wire_init(
    int port_,
    ffi.Pointer<wire_uint_8_list> os_type,
    ffi.Pointer<wire_uint_8_list> os_version,
    ffi.Pointer<wire_uint_8_list> config_dir,
  ) {
    return _wire_init(
      port_,
      os_type,
      os_version,
      config_dir,
    );
  }

  late final _wire_initPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>)>>('wire_init');
  late final _wire_init = _wire_initPtr.asFunction<
      void Function(int, ffi.Pointer<wire_uint_8_list>,
          ffi.Pointer<wire_uint_8_list>, ffi.Pointer<wire_uint_8_list>)>();

  void wire_config_read_device_id(
    int port_,
  ) {
    return _wire_config_read_device_id(
      port_,
    );
  }

  late final _wire_config_read_device_idPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_config_read_device_id');
  late final _wire_config_read_device_id =
      _wire_config_read_device_idPtr.asFunction<void Function(int)>();

  void wire_config_save_device_id(
    int port_,
    ffi.Pointer<wire_uint_8_list> device_id,
  ) {
    return _wire_config_save_device_id(
      port_,
      device_id,
    );
  }

  late final _wire_config_save_device_idPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_uint_8_list>)>>('wire_config_save_device_id');
  late final _wire_config_save_device_id = _wire_config_save_device_idPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_config_read_device_id_expiration(
    int port_,
  ) {
    return _wire_config_read_device_id_expiration(
      port_,
    );
  }

  late final _wire_config_read_device_id_expirationPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_config_read_device_id_expiration');
  late final _wire_config_read_device_id_expiration =
      _wire_config_read_device_id_expirationPtr
          .asFunction<void Function(int)>();

  void wire_config_save_device_id_expiration(
    int port_,
    int time_stamp,
  ) {
    return _wire_config_save_device_id_expiration(
      port_,
      time_stamp,
    );
  }

  late final _wire_config_save_device_id_expirationPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int32)>>(
          'wire_config_save_device_id_expiration');
  late final _wire_config_save_device_id_expiration =
      _wire_config_save_device_id_expirationPtr
          .asFunction<void Function(int, int)>();

  void wire_config_read_device_password(
    int port_,
  ) {
    return _wire_config_read_device_password(
      port_,
    );
  }

  late final _wire_config_read_device_passwordPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_config_read_device_password');
  late final _wire_config_read_device_password =
      _wire_config_read_device_passwordPtr.asFunction<void Function(int)>();

  void wire_config_save_device_password(
    int port_,
    ffi.Pointer<wire_uint_8_list> device_password,
  ) {
    return _wire_config_save_device_password(
      port_,
      device_password,
    );
  }

  late final _wire_config_save_device_passwordPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>(
      'wire_config_save_device_password');
  late final _wire_config_save_device_password =
      _wire_config_save_device_passwordPtr
          .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_signaling_connect(
    int port_,
    ffi.Pointer<wire_uint_8_list> remote_device_id,
  ) {
    return _wire_signaling_connect(
      port_,
      remote_device_id,
    );
  }

  late final _wire_signaling_connectPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_uint_8_list>)>>('wire_signaling_connect');
  late final _wire_signaling_connect = _wire_signaling_connectPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_signaling_connection_key_exchange(
    int port_,
    ffi.Pointer<wire_uint_8_list> remote_device_id,
    ffi.Pointer<wire_uint_8_list> password,
  ) {
    return _wire_signaling_connection_key_exchange(
      port_,
      remote_device_id,
      password,
    );
  }

  late final _wire_signaling_connection_key_exchangePtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>,
                  ffi.Pointer<wire_uint_8_list>)>>(
      'wire_signaling_connection_key_exchange');
  late final _wire_signaling_connection_key_exchange =
      _wire_signaling_connection_key_exchangePtr.asFunction<
          void Function(int, ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>)>();

  void wire_endpoint_get_display_info(
    int port_,
    ffi.Pointer<wire_uint_8_list> remote_device_id,
  ) {
    return _wire_endpoint_get_display_info(
      port_,
      remote_device_id,
    );
  }

  late final _wire_endpoint_get_display_infoPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>(
      'wire_endpoint_get_display_info');
  late final _wire_endpoint_get_display_info =
      _wire_endpoint_get_display_infoPtr
          .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_endpoint_start_media_transmission(
    int port_,
    ffi.Pointer<wire_uint_8_list> remote_device_id,
    ffi.Pointer<wire_uint_8_list> display_id,
    int texture_id,
    int video_texture_ptr,
    int update_frame_callback_ptr,
  ) {
    return _wire_endpoint_start_media_transmission(
      port_,
      remote_device_id,
      display_id,
      texture_id,
      video_texture_ptr,
      update_frame_callback_ptr,
    );
  }

  late final _wire_endpoint_start_media_transmissionPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Int64,
              ffi.Int64,
              ffi.Int64)>>('wire_endpoint_start_media_transmission');
  late final _wire_endpoint_start_media_transmission =
      _wire_endpoint_start_media_transmissionPtr.asFunction<
          void Function(int, ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>, int, int, int)>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list(
    int len,
  ) {
    return _new_uint_8_list(
      len,
    );
  }

  late final _new_uint_8_listPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_uint_8_list> Function(
              ffi.Int32)>>('new_uint_8_list');
  late final _new_uint_8_list = _new_uint_8_listPtr
      .asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  void free_WireSyncReturnStruct(
    WireSyncReturnStruct val,
  ) {
    return _free_WireSyncReturnStruct(
      val,
    );
  }

  late final _free_WireSyncReturnStructPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturnStruct)>>(
          'free_WireSyncReturnStruct');
  late final _free_WireSyncReturnStruct = _free_WireSyncReturnStructPtr
      .asFunction<void Function(WireSyncReturnStruct)>();

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>(
          'store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr
      .asFunction<void Function(DartPostCObjectFnType)>();
}

class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<ffi.Uint8 Function(DartPort, ffi.Pointer<ffi.Void>)>>;
typedef DartPort = ffi.Int64;
