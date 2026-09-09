/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <uapi/asm/hw_breakpoint.h>
pub const __ARCH_HW_BREAKPOINT_H: bool = true;

/*
 * The name should probably be something dealt in
 * a higher level. While dealing with the user
 * (display/resolving)
 */
#[repr(C)]
pub struct arch_hw_breakpoint {
    pub address: core::ffi::c_ulong,
    pub mask: core::ffi::c_ulong,
    pub len: u8,
    pub r#type: u8,
}

// Dependencies: <linux/kdebug.h>, <linux/percpu.h>, <linux/list.h>

/* Available HW breakpoint length encodings */
pub const X86_BREAKPOINT_LEN_X: u32 = 0x40;
pub const X86_BREAKPOINT_LEN_1: u32 = 0x40;
pub const X86_BREAKPOINT_LEN_2: u32 = 0x44;
pub const X86_BREAKPOINT_LEN_4: u32 = 0x4c;

#[cfg(target_arch = "x86_64")]
pub const X86_BREAKPOINT_LEN_8: u32 = 0x48;

/* Available HW breakpoint type encodings */

/* trigger on instruction execute */
pub const X86_BREAKPOINT_EXECUTE: u32 = 0x80;
/* trigger on memory write */
pub const X86_BREAKPOINT_WRITE: u32 = 0x81;
/* trigger on memory read or write */
pub const X86_BREAKPOINT_RW: u32 = 0x83;

/* Total number of available HW breakpoint registers */
pub const HBP_NUM: i32 = 4;

#[macro_export]
macro_rules! hw_breakpoint_slots {
    ($type:expr) => { $crate::HBP_NUM };
}

#[repr(C)]
pub struct perf_event_attr {
    _private: [u8; 0],
}
#[repr(C)]
pub struct perf_event {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pmu {
    _private: [u8; 0],
}
#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

extern "C" {
    pub fn arch_check_bp_in_kernelspace(hw: *mut arch_hw_breakpoint) -> core::ffi::c_int;
    pub fn hw_breakpoint_arch_parse(
        bp: *mut perf_event,
        attr: *const perf_event_attr,
        hw: *mut arch_hw_breakpoint,
    ) -> core::ffi::c_int;
    pub fn hw_breakpoint_exceptions_notify(
        unused: *mut notifier_block,
        val: core::ffi::c_ulong,
        data: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;

    pub fn arch_install_hw_breakpoint(bp: *mut perf_event) -> core::ffi::c_int;
    pub fn arch_uninstall_hw_breakpoint(bp: *mut perf_event);
    pub fn hw_breakpoint_pmu_read(bp: *mut perf_event);
    pub fn hw_breakpoint_pmu_unthrottle(bp: *mut perf_event);

    pub fn arch_fill_perf_breakpoint(bp: *mut perf_event);

    pub fn encode_dr7(
        drnum: core::ffi::c_int,
        len: core::ffi::c_uint,
        r#type: core::ffi::c_uint,
    ) -> core::ffi::c_ulong;
    pub fn decode_dr7(
        dr7: core::ffi::c_ulong,
        bpnum: core::ffi::c_int,
        len: *mut core::ffi::c_uint,
        r#type: *mut core::ffi::c_uint,
    ) -> core::ffi::c_int;

    pub fn arch_bp_generic_fields(
        x86_len: core::ffi::c_int,
        x86_type: core::ffi::c_int,
        gen_len: *mut core::ffi::c_int,
        gen_type: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub static mut perf_ops_bp: pmu;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
