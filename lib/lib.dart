import 'dart:ffi' as ffi;
import 'dart:io' show Platform;

late ffi.DynamicLibrary dl;
typedef CopyDirFunc = ffi.Int32 Function(
  ffi.Pointer<ffi.Char> src,
  ffi.Pointer<ffi.Char> dst,
);
typedef CopyDir = int Function(
  ffi.Pointer<ffi.Char> src,
  ffi.Pointer<ffi.Char> dst,
);

typedef AddFunc = ffi.Int32 Function(ffi.Int32 a, ffi.Int32 b);
typedef Add = int Function(int a, int b);

const libPath =
    '/Users/agmacone/Projects/Dart/rusty_dart/rusty_lib/target/aarch64-apple-darwin/release/librusty_lib.dylib';
const source = '/Users/agmacone/Projects/Dart/rusty_dart/dummy/source';
const destination = '/Users/agmacone/Projects/Dart/rusty_dart/dummy/dest';

void execRust() {
  if (Platform.isAndroid || Platform.isLinux) {
    dl = ffi.DynamicLibrary.open(libPath);
  } else {
    dl = ffi.DynamicLibrary.open(libPath);
  }

  // final copyDirPointer = dl.lookup<ffi.NativeFunction<CopyDirFunc>>('copy_dir');
  // final copyDir = copyDirPointer.asFunction<CopyDir>();

  // final src = source.toNativeUtf8();
  // final dst = destination.toNativeUtf8();

  // final result = copyDir(src.cast<ffi.Char>(), dst.cast<ffi.Char>());

  // pffi.calloc.free(src);
  // pffi.calloc.free(dst);

  // print('Copy result: $result');

  final addPointer = dl.lookup<ffi.NativeFunction<AddFunc>>('add');
  final add = addPointer.asFunction<Add>();
  final result = add(2, 3);

  print('Copy result: $result');
}
