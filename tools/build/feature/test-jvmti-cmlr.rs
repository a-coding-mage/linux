// SPDX-License-Identifier: GPL-2.0
// C dependencies: <jvmti.h>, <jvmticmlr.h>

extern "C" {
    type jvmtiCompiledMethodLoadInlineRecord;
    type jvmtiCompiledMethodLoadRecordHeader;
    type PCStackInfo;
}

fn main() -> i32 {
    let rec: *mut jvmtiCompiledMethodLoadInlineRecord = core::ptr::null_mut();
    let hdr: *mut jvmtiCompiledMethodLoadRecordHeader = core::ptr::null_mut();
    let p: *mut PCStackInfo = core::ptr::null_mut();
    let _ = (rec, hdr, p);
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
