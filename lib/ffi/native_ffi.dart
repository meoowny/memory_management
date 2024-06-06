import 'dart:ffi';
import 'dart:io';

import 'package:os_lab2/ffi/rust_ffi.dart';

class NativeFFI {
  NativeFFI._();

  static DynamicLibrary? _dyLib;

  static DynamicLibrary get dyLib {
    if (_dyLib != null) return _dyLib!;

    const base = "rust_ffi";
    if (Platform.isWindows) {
      _dyLib = DynamicLibrary.open("$base.dll");
    }
    else if (Platform.isAndroid) {
      _dyLib = DynamicLibrary.open("lib$base.so");
    }
    else {
      throw Exception("DynamicLibrary 初始化失败");
    }
    return _dyLib!;
  }
}

class NativeFun {
  static final _ffi = RustFfiImpl(NativeFFI.dyLib);

  static Future<ExecRecord> generateReplacementRecord(AlgoChoice algoChoice, GenChoice genChoice) async {
    return await _ffi.generateReplacementRecord(memCapacity: 4, totalInstrument: 320, pageSize: 10, algoChoice:algoChoice, genChoice: genChoice);
  }
}
