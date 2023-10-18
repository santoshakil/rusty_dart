import 'dart:ffi';
import 'dart:isolate';

import 'package:ffi/ffi.dart';

void rustyDart() async {
  final dlib = DynamicLibrary.open('target/release/librusty_dart.dylib');
  final init = dlib.lookupFunction<Void Function(), void Function()>('init');
  final getData = dlib.lookupFunction<Pointer<Utf8> Function(), Pointer<Utf8> Function()>('get');
  init();

  await Isolate.spawn(
    (getData) {
      final data = getData();
      final str = data.toDartString();
      print(str);
    },
    getData,
  );

  while (true) {
    await Future.delayed(Duration(seconds: 1));
  }
}
