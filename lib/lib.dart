import 'dart:ffi' as ffi;
import 'dart:io' show Platform;
import 'package:ffi/ffi.dart' as pffi;

void execRust() {}

late ffi.DynamicLibrary dl;
typedef CopyDirFunc = ffi.Int32 Function(
  ffi.Pointer<ffi.Char> src,
  ffi.Pointer<ffi.Char> dst,
);
typedef CopyDir = int Function(
  ffi.Pointer<ffi.Char> src,
  ffi.Pointer<ffi.Char> dst,
);

const libPath = '/Users/agmacone/Projects/Dart/rusty_dart/libmain.so';
const source = '/Users/agmacone/Projects/Dart/rusty_dart/dummy/source';
const destination = '/Users/agmacone/Projects/Dart/rusty_dart/dummy/dest';

void main() {
  if (Platform.isMacOS || Platform.isLinux) {
    dl = ffi.DynamicLibrary.open(libPath);
  } else if (Platform.isWindows) {
    dl = ffi.DynamicLibrary.open("rust.dll");
  } else {
    print('Unsupported platform');
    return;
  }

  final copyDirPointer = dl.lookup<ffi.NativeFunction<CopyDirFunc>>('copy_dir');
  final copyDir = copyDirPointer.asFunction<CopyDir>();

  final src = source.toNativeUtf8();
  final dst = destination.toNativeUtf8();

  final result = copyDir(src.cast<ffi.Char>(), dst.cast<ffi.Char>());

  pffi.calloc.free(src);
  pffi.calloc.free(dst);

  print('Copy result: $result');
}
