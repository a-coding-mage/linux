// SPDX-License-Identifier: GPL-2.0-only
/*
 * Kernel module for testing dynamic_debug
 *
 * Authors:
 *      Jim Cromie <jim.cromie@gmail.com>
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct KernelParamOps {
    pub set: Option<unsafe extern "C" fn(*const c_char, *const KernelParam) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut c_char, *const KernelParam) -> c_int>,
}
#[repr(C)]
pub struct KernelParam;
#[repr(C)]
pub struct DdebugClassParam {
    pub bits: *mut c_ulong,
    pub flags: *const c_char,
    pub map: *const c_void,
}

extern "C" {
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn __pr_debug_cls(sym: c_int, msg: *const c_char);
    static param_ops_dyndbg_classes: KernelParamOps;
}

unsafe extern "C" fn param_set_do_prints(_instr: *const c_char, _kp: *const KernelParam) -> c_int {
    do_prints();
    0
}
unsafe extern "C" fn param_get_do_prints(buffer: *mut c_char, _kp: *const KernelParam) -> c_int {
    do_prints();
    scnprintf(buffer, 4096, b"did do_prints\n\0".as_ptr() as *const c_char)
}
static PARAM_OPS_DO_PRINTS: KernelParamOps =
    KernelParamOps { set: Some(param_set_do_prints), get: Some(param_get_do_prints) };
// module_param_cb(do_prints, &param_ops_do_prints, NULL, 0600)

/*
 * Using the CLASSMAP api:
 * - classmaps must have corresponding enum
 * - enum symbols must match/correlate with class-name strings in the map.
 * - base must equal enum's 1st value
 * - multiple maps must set their base to share the 0-30 class_id space !!
 *   (build-bug-on tips welcome)
 * Additionally, here:
 * - tie together sysname, mapname, bitsname, flagsname
 */
// DECLARE_DYNDBG_CLASSMAP and DD_SYS_WRAP are kernel-provided declarations.

#[repr(i32)]
pub enum CatDisjointBits { D2Core = 0, D2Driver, D2Kms, D2Prime, D2Atomic, D2Vbl, D2State, D2Lease, D2Dp, D2Drmres }
#[repr(i32)]
pub enum CatDisjointNames { Low = 11, Mid, Hi }
#[repr(i32)]
pub enum CatLevelNum { V0 = 14, V1, V2, V3, V4, V5, V6, V7 }
#[repr(i32)]
pub enum CatLevelNames { L0 = 22, L1, L2, L3, L4, L5, L6, L7 }

// DECLARE_DYNDBG_CLASSMAP(map_disjoint_bits, DD_CLASS_TYPE_DISJOINT_BITS, 0, "D2_CORE", "D2_DRIVER", "D2_KMS", "D2_PRIME", "D2_ATOMIC", "D2_VBL", "D2_STATE", "D2_LEASE", "D2_DP", "D2_DRMRES");
// DD_SYS_WRAP(disjoint_bits, p); DD_SYS_WRAP(disjoint_bits, T);
// DECLARE_DYNDBG_CLASSMAP(map_disjoint_names, DD_CLASS_TYPE_DISJOINT_NAMES, 10, "LOW", "MID", "HI");
// DD_SYS_WRAP(disjoint_names, p); DD_SYS_WRAP(disjoint_names, T);
// DECLARE_DYNDBG_CLASSMAP(map_level_num, DD_CLASS_TYPE_LEVEL_NUM, 14, "V0", "V1", "V2", "V3", "V4", "V5", "V6", "V7");
// DD_SYS_WRAP(level_num, p); DD_SYS_WRAP(level_num, T);
// DECLARE_DYNDBG_CLASSMAP(map_level_names, DD_CLASS_TYPE_LEVEL_NAMES, 22, "L0", "L1", "L2", "L3", "L4", "L5", "L6", "L7");
// DD_SYS_WRAP(level_names, p); DD_SYS_WRAP(level_names, T);

unsafe fn prdbg(sym: c_int, msg: &'static [u8]) {
    __pr_debug_cls(sym, msg.as_ptr() as *const c_char);
}
unsafe fn do_cats() {
    prdbg(0, b"doing categories\n\0");
    prdbg(CatDisjointNames::Low as c_int, b"LOW msg\n\0");
    prdbg(CatDisjointNames::Mid as c_int, b"MID msg\n\0");
    prdbg(CatDisjointNames::Hi as c_int, b"HI msg\n\0");
    prdbg(CatDisjointBits::D2Core as c_int, b"D2_CORE msg\n\0");
    prdbg(CatDisjointBits::D2Driver as c_int, b"D2_DRIVER msg\n\0");
    prdbg(CatDisjointBits::D2Kms as c_int, b"D2_KMS msg\n\0");
    prdbg(CatDisjointBits::D2Prime as c_int, b"D2_PRIME msg\n\0");
    prdbg(CatDisjointBits::D2Atomic as c_int, b"D2_ATOMIC msg\n\0");
    prdbg(CatDisjointBits::D2Vbl as c_int, b"D2_VBL msg\n\0");
    prdbg(CatDisjointBits::D2State as c_int, b"D2_STATE msg\n\0");
    prdbg(CatDisjointBits::D2Lease as c_int, b"D2_LEASE msg\n\0");
    prdbg(CatDisjointBits::D2Dp as c_int, b"D2_DP msg\n\0");
    prdbg(CatDisjointBits::D2Drmres as c_int, b"D2_DRMRES msg\n\0");
}
unsafe fn do_levels() {
    prdbg(0, b"doing levels\n\0");
    prdbg(CatLevelNum::V1 as c_int, b"V1 msg\n\0");
    prdbg(CatLevelNum::V2 as c_int, b"V2 msg\n\0");
    prdbg(CatLevelNum::V3 as c_int, b"V3 msg\n\0");
    prdbg(CatLevelNum::V4 as c_int, b"V4 msg\n\0");
    prdbg(CatLevelNum::V5 as c_int, b"V5 msg\n\0");
    prdbg(CatLevelNum::V6 as c_int, b"V6 msg\n\0");
    prdbg(CatLevelNum::V7 as c_int, b"V7 msg\n\0");
    prdbg(CatLevelNames::L1 as c_int, b"L1 msg\n\0");
    prdbg(CatLevelNames::L2 as c_int, b"L2 msg\n\0");
    prdbg(CatLevelNames::L3 as c_int, b"L3 msg\n\0");
    prdbg(CatLevelNames::L4 as c_int, b"L4 msg\n\0");
    prdbg(CatLevelNames::L5 as c_int, b"L5 msg\n\0");
    prdbg(CatLevelNames::L6 as c_int, b"L6 msg\n\0");
    prdbg(CatLevelNames::L7 as c_int, b"L7 msg\n\0");
}
unsafe fn do_prints() { do_cats(); do_levels(); }

pub unsafe extern "C" fn test_dynamic_debug_init() -> c_int {
    prdbg(0, b"init start\n\0");
    do_prints();
    prdbg(0, b"init done\n\0");
    0
}
pub unsafe extern "C" fn test_dynamic_debug_exit() {
    prdbg(0, b"exited\n\0");
}
// module_init(test_dynamic_debug_init); module_exit(test_dynamic_debug_exit);
// MODULE_AUTHOR("Jim Cromie <jim.cromie@gmail.com>");
// MODULE_DESCRIPTION("Kernel module for testing dynamic_debug");
// MODULE_LICENSE("GPL");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
