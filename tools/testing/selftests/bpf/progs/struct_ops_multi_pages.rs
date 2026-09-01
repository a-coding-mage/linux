// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

use core::ffi::c_void;

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "../test_kmods/bpf_testmod.h"

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = "struct_ops/tramp_1"]
pub extern "C" fn tramp_1(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_2"]
pub extern "C" fn tramp_2(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_3"]
pub extern "C" fn tramp_3(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_4"]
pub extern "C" fn tramp_4(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_5"]
pub extern "C" fn tramp_5(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_6"]
pub extern "C" fn tramp_6(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_7"]
pub extern "C" fn tramp_7(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_8"]
pub extern "C" fn tramp_8(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_9"]
pub extern "C" fn tramp_9(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_10"]
pub extern "C" fn tramp_10(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_11"]
pub extern "C" fn tramp_11(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_12"]
pub extern "C" fn tramp_12(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_13"]
pub extern "C" fn tramp_13(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_14"]
pub extern "C" fn tramp_14(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_15"]
pub extern "C" fn tramp_15(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_16"]
pub extern "C" fn tramp_16(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_17"]
pub extern "C" fn tramp_17(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_18"]
pub extern "C" fn tramp_18(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_19"]
pub extern "C" fn tramp_19(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_20"]
pub extern "C" fn tramp_20(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_21"]
pub extern "C" fn tramp_21(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_22"]
pub extern "C" fn tramp_22(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_23"]
pub extern "C" fn tramp_23(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_24"]
pub extern "C" fn tramp_24(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_25"]
pub extern "C" fn tramp_25(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_26"]
pub extern "C" fn tramp_26(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_27"]
pub extern "C" fn tramp_27(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_28"]
pub extern "C" fn tramp_28(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_29"]
pub extern "C" fn tramp_29(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_30"]
pub extern "C" fn tramp_30(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_31"]
pub extern "C" fn tramp_31(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_32"]
pub extern "C" fn tramp_32(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_33"]
pub extern "C" fn tramp_33(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_34"]
pub extern "C" fn tramp_34(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_35"]
pub extern "C" fn tramp_35(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_36"]
pub extern "C" fn tramp_36(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_37"]
pub extern "C" fn tramp_37(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_38"]
pub extern "C" fn tramp_38(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_39"]
pub extern "C" fn tramp_39(a: i32) -> i32 { a }
#[no_mangle]
#[link_section = "struct_ops/tramp_40"]
pub extern "C" fn tramp_40(a: i32) -> i32 { a }

#[no_mangle]
#[link_section = ".struct_ops.link"]
pub static mut multi_pages: bpf_testmod_ops = bpf_testmod_ops {
    tramp_1: tramp_1 as *mut c_void,
    tramp_2: tramp_2 as *mut c_void,
    tramp_3: tramp_3 as *mut c_void,
    tramp_4: tramp_4 as *mut c_void,
    tramp_5: tramp_5 as *mut c_void,
    tramp_6: tramp_6 as *mut c_void,
    tramp_7: tramp_7 as *mut c_void,
    tramp_8: tramp_8 as *mut c_void,
    tramp_9: tramp_9 as *mut c_void,
    tramp_10: tramp_10 as *mut c_void,
    tramp_11: tramp_11 as *mut c_void,
    tramp_12: tramp_12 as *mut c_void,
    tramp_13: tramp_13 as *mut c_void,
    tramp_14: tramp_14 as *mut c_void,
    tramp_15: tramp_15 as *mut c_void,
    tramp_16: tramp_16 as *mut c_void,
    tramp_17: tramp_17 as *mut c_void,
    tramp_18: tramp_18 as *mut c_void,
    tramp_19: tramp_19 as *mut c_void,
    tramp_20: tramp_20 as *mut c_void,
    tramp_21: tramp_21 as *mut c_void,
    tramp_22: tramp_22 as *mut c_void,
    tramp_23: tramp_23 as *mut c_void,
    tramp_24: tramp_24 as *mut c_void,
    tramp_25: tramp_25 as *mut c_void,
    tramp_26: tramp_26 as *mut c_void,
    tramp_27: tramp_27 as *mut c_void,
    tramp_28: tramp_28 as *mut c_void,
    tramp_29: tramp_29 as *mut c_void,
    tramp_30: tramp_30 as *mut c_void,
    tramp_31: tramp_31 as *mut c_void,
    tramp_32: tramp_32 as *mut c_void,
    tramp_33: tramp_33 as *mut c_void,
    tramp_34: tramp_34 as *mut c_void,
    tramp_35: tramp_35 as *mut c_void,
    tramp_36: tramp_36 as *mut c_void,
    tramp_37: tramp_37 as *mut c_void,
    tramp_38: tramp_38 as *mut c_void,
    tramp_39: tramp_39 as *mut c_void,
    tramp_40: tramp_40 as *mut c_void,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
