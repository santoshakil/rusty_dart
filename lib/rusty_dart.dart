import 'dart:ffi';
import 'dart:isolate';

import 'package:ffi/ffi.dart';

typedef StrFn = Pointer<Utf8> Function();
typedef FnStrsF = Void Function(Pointer<Utf8>, Pointer<Utf8>);
typedef FnStrsD = void Function(Pointer<Utf8>, Pointer<Utf8>);

const path = 'target/release/librusty_dart.dylib';

void rustyDart() async {
  final dlib = DynamicLibrary.open(path);
  final getData = dlib.lookupFunction<StrFn, StrFn>('get');
  final setData = dlib.lookupFunction<FnStrsF, FnStrsD>('set');
  final key = 'key'.toNativeUtf8();
  final value = 'value'.toNativeUtf8();
  setData(key, value);
  calloc.free(key);
  calloc.free(value);
  print("Dart: ${getData().toDartString()}");

  await Isolate.spawn(
    (_) {
      final dlib = DynamicLibrary.open(path);
      final getData = dlib.lookupFunction<StrFn, StrFn>('get');
      final setData = dlib.lookupFunction<FnStrsF, FnStrsD>('set');
      final key = 'key2'.toNativeUtf8();
      final value = 'value2'.toNativeUtf8();
      setData(key, value);
      calloc.free(key);
      calloc.free(value);
      print("Dart Isolate: ${getData().toDartString()}");
    },
    null,
  );

  while (true) {
    await Future.delayed(Duration(seconds: 1));
    print("Dart: ${getData().toDartString()}");
  }
}
