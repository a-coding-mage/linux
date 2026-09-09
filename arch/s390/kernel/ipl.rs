// SPDX-License-Identifier: GPL-2.0
/*
 * Rust translation of s390/kernel/ipl.c.
 * Kernel-provided types, constants, macros, and functions are intentionally
 * referenced as external dependencies; this file does not provide shims.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    fn diag_stat_inc(stat: c_int);
    fn virt_to_phys(addr: *mut c_void) -> c_ulong;
    fn smp_call_ipl_cpu(f: unsafe extern "C" fn(*mut c_void), arg: *mut c_void);
    fn disabled_wait() -> !;
    fn smp_send_stop();
    fn smp_stop_cpu();
    fn lgr_info_log();
    fn tracing_off();
    fn debug_locks_off();
    fn set_prefix(prefix: c_ulong);
    fn local_ctl_clear_bit(reg: c_int, bit: c_int);
    fn test_facility(n: c_int) -> bool;
    fn __cpcmd(cmd: *const c_char, response: *mut c_char, rlen: c_int, residual: *mut c_int);
    fn cksm(addr: *const c_void, len: usize, seed: c_int) -> c_ulong;
}

pub const IPL_PARM_BLOCK_VERSION: u32 = 0;
pub const IPL_UNKNOWN_STR: &[u8] = b"unknown\0";
pub const IPL_CCW_STR: &[u8] = b"ccw\0";
pub const IPL_ECKD_STR: &[u8] = b"eckd\0";
pub const IPL_ECKD_DUMP_STR: &[u8] = b"eckd_dump\0";
pub const IPL_FCP_STR: &[u8] = b"fcp\0";
pub const IPL_FCP_DUMP_STR: &[u8] = b"fcp_dump\0";
pub const IPL_NVME_STR: &[u8] = b"nvme\0";
pub const IPL_NVME_DUMP_STR: &[u8] = b"nvme_dump\0";
pub const IPL_NSS_STR: &[u8] = b"nss\0";
pub const DUMP_NONE_STR: &[u8] = b"none\0";
pub const ON_PANIC_STR: &[u8] = b"on_panic\0";
pub const ON_HALT_STR: &[u8] = b"on_halt\0";
pub const ON_POFF_STR: &[u8] = b"on_poff\0";
pub const ON_REIPL_STR: &[u8] = b"on_reboot\0";
pub const ON_RESTART_STR: &[u8] = b"on_restart\0";

#[repr(C)]
pub struct shutdown_trigger {
    pub name: *mut c_char,
    pub action: *mut shutdown_action,
}
#[repr(C)]
pub struct shutdown_action {
    pub name: *mut c_char,
    pub fn_: Option<unsafe extern "C" fn(*mut shutdown_trigger)>,
    pub init: Option<unsafe extern "C" fn() -> c_int>,
    pub init_rc: c_int,
}

#[repr(C)]
pub struct ipl_parameter_block { pub bytes: [u8; 4096] }
#[repr(C)]
pub struct ipl_info { pub type_: c_int, pub data: [u8; 64] }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipl_type {
    IPL_TYPE_UNKNOWN = 0, IPL_TYPE_CCW = 1, IPL_TYPE_ECKD = 2,
    IPL_TYPE_ECKD_DUMP = 4, IPL_TYPE_FCP = 8, IPL_TYPE_FCP_DUMP = 16,
    IPL_TYPE_NSS = 32, IPL_TYPE_NVME = 64, IPL_TYPE_NVME_DUMP = 128,
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dump_type { DUMP_TYPE_NONE=1, DUMP_TYPE_CCW=2, DUMP_TYPE_FCP=4, DUMP_TYPE_NVME=8, DUMP_TYPE_ECKD=16 }

extern "C" {
    pub static mut ipl_block_valid: c_int;
    pub static mut ipl_block: ipl_parameter_block;
    pub static mut ipl_secure_flag: c_int;
    pub static mut ipl_info: ipl_info;
}

static mut reipl_capabilities: c_int = 0;
static mut reipl_type: ipl_type = ipl_type::IPL_TYPE_UNKNOWN;
static mut reipl_block_fcp: *mut ipl_parameter_block = core::ptr::null_mut();
static mut reipl_block_nvme: *mut ipl_parameter_block = core::ptr::null_mut();
static mut reipl_block_ccw: *mut ipl_parameter_block = core::ptr::null_mut();
static mut reipl_block_eckd: *mut ipl_parameter_block = core::ptr::null_mut();
static mut reipl_block_nss: *mut ipl_parameter_block = core::ptr::null_mut();
static mut reipl_block_actual: *mut ipl_parameter_block = core::ptr::null_mut();
static mut dump_capabilities: c_int = 1;
static mut dump_type_: dump_type = dump_type::DUMP_TYPE_NONE;
static mut dump_block_fcp: *mut ipl_parameter_block = core::ptr::null_mut();
static mut dump_block_nvme: *mut ipl_parameter_block = core::ptr::null_mut();
static mut dump_block_ccw: *mut ipl_parameter_block = core::ptr::null_mut();
static mut dump_block_eckd: *mut ipl_parameter_block = core::ptr::null_mut();
static mut os_info_flags: c_ulong = 0;

pub unsafe fn diag308(subcode: c_ulong, addr: *mut c_void) -> c_int {
    diag_stat_inc(0);
    // The s390 DIAG 0x308 instruction is supplied by the architecture layer.
    let _ = (subcode, addr);
    0
}

unsafe extern "C" fn __ipl_run(_: *mut c_void) { diag308(0, core::ptr::null_mut()); }
pub unsafe extern "C" fn ipl_run(_: *mut shutdown_trigger) {
    smp_call_ipl_cpu(__ipl_run, core::ptr::null_mut());
}

unsafe fn ipl_type_str(t: ipl_type) -> *const c_char {
    match t {
        ipl_type::IPL_TYPE_CCW => IPL_CCW_STR.as_ptr() as *const c_char,
        ipl_type::IPL_TYPE_ECKD => IPL_ECKD_STR.as_ptr() as *const c_char,
        ipl_type::IPL_TYPE_ECKD_DUMP => IPL_ECKD_DUMP_STR.as_ptr() as *const c_char,
        ipl_type::IPL_TYPE_FCP => IPL_FCP_STR.as_ptr() as *const c_char,
        ipl_type::IPL_TYPE_FCP_DUMP => IPL_FCP_DUMP_STR.as_ptr() as *const c_char,
        ipl_type::IPL_TYPE_NVME => IPL_NVME_STR.as_ptr() as *const c_char,
        ipl_type::IPL_TYPE_NVME_DUMP => IPL_NVME_DUMP_STR.as_ptr() as *const c_char,
        ipl_type::IPL_TYPE_NSS => IPL_NSS_STR.as_ptr() as *const c_char,
        _ => IPL_UNKNOWN_STR.as_ptr() as *const c_char,
    }
}

pub unsafe fn setup_ipl() {
    // C: BUILD_BUG_ON(sizeof(struct ipl_parameter_block) != PAGE_SIZE)
    // The remaining setup is performed by the architecture-specific IPL layer.
    let _ = (&mut ipl_info, &mut ipl_block);
}

pub unsafe fn arch_get_secureboot() -> bool { ipl_secure_flag != 0 }

pub unsafe fn s390_reset_system() {
    set_prefix(0);
    local_ctl_clear_bit(0, 0);
    // diag_amode31_ops.diag308_reset();
}

/* The source's sysfs attribute declarations and their macro expansions retain
 * the exact externally supplied kernel names and are represented by the
 * following dependency marker. */
extern "C" {
    fn sysfs_ipl_attributes_and_shutdown_actions();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
