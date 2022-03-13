import 'package:flutter/material.dart';
import 'package:get/get.dart';

void popupAskDialog({required String content, Function()? yesAction}) {
  Get.defaultDialog(
      title: "MirrorX",
      content: Text(content),
      barrierDismissible: false,
      radius: 12,
      actions: [
        TextButton(
            onPressed: () {
              if (yesAction != null) {
                yesAction();
              }
              Get.back(closeOverlays: true);
            },
            child: Text("dialog.yes".tr)),
        TextButton(
            onPressed: () {
              Get.back(closeOverlays: true);
            },
            child: Text("dialog.no".tr))
      ]);
}

void popupErrorDialog({required String content}) {
  Get.defaultDialog(
    title: "MirrorX Error",
    content: Text(content),
    barrierDismissible: false,
    textCancel: "No",
    textConfirm: "Yes",
  );
}
