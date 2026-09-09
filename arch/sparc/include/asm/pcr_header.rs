/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct pcr_ops {
    pub read_pcr: Option<unsafe extern "C" fn(unsigned_long: ::core::ffi::c_ulong) -> u64>,
    pub write_pcr:
        Option<unsafe extern "C" fn(unsigned_long: ::core::ffi::c_ulong, value: u64)>,
    pub read_pic: Option<unsafe extern "C" fn(unsigned_long: ::core::ffi::c_ulong) -> u64>,
    pub write_pic:
        Option<unsafe extern "C" fn(unsigned_long: ::core::ffi::c_ulong, value: u64)>,
    pub nmi_picl_value: Option<unsafe extern "C" fn(nmi_hz: ::core::ffi::c_uint) -> u64>,
    pub pcr_nmi_enable: u64,
    pub pcr_nmi_disable: u64,
}

// External dependency supplied by the surrounding translation unit.
extern "C" {
    pub static pcr_ops: *const pcr_ops;

    pub fn deferred_pcr_work_irq(irq: ::core::ffi::c_int, regs: *mut pt_regs);
    pub fn schedule_deferred_pcr_work();

    pub fn pcr_arch_init() -> ::core::ffi::c_int;
}

// External dependency supplied by the surrounding translation unit.
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

pub const PCR_PIC_PRIV: u64 = 0x00000001; // PIC access is privileged
pub const PCR_STRACE: u64 = 0x00000002; // Trace supervisor events
pub const PCR_UTRACE: u64 = 0x00000004; // Trace user events
pub const PCR_N2_HTRACE: u64 = 0x00000008; // Trace hypervisor events
pub const PCR_N2_TOE_OV0: u64 = 0x00000010; // Trap if PIC 0 overflows
pub const PCR_N2_TOE_OV1: u64 = 0x00000020; // Trap if PIC 1 overflows
pub const PCR_N2_MASK0: u64 = 0x00003fc0;
pub const PCR_N2_MASK0_SHIFT: u64 = 6;
pub const PCR_N2_SL0: u64 = 0x0003c000;
pub const PCR_N2_SL0_SHIFT: u64 = 14;
pub const PCR_N2_OV0: u64 = 0x00040000;
pub const PCR_N2_MASK1: u64 = 0x07f80000;
pub const PCR_N2_MASK1_SHIFT: u64 = 19;
pub const PCR_N2_SL1: u64 = 0x78000000;
pub const PCR_N2_SL1_SHIFT: u64 = 27;
pub const PCR_N2_OV1: u64 = 0x80000000;

pub const PCR_N4_OV: u64 = 0x00000001; // PIC overflow
pub const PCR_N4_TOE: u64 = 0x00000002; // Trap On Event
pub const PCR_N4_UTRACE: u64 = 0x00000004; // Trace user events
pub const PCR_N4_STRACE: u64 = 0x00000008; // Trace supervisor events
pub const PCR_N4_HTRACE: u64 = 0x00000010; // Trace hypervisor events
pub const PCR_N4_MASK: u64 = 0x000007e0; // Event mask
pub const PCR_N4_MASK_SHIFT: u64 = 5;
pub const PCR_N4_SL: u64 = 0x0000f800; // Event Select
pub const PCR_N4_SL_SHIFT: u64 = 11;
pub const PCR_N4_PICNPT: u64 = 0x00010000; // PIC non-privileged trap
pub const PCR_N4_PICNHT: u64 = 0x00020000; // PIC non-hypervisor trap
pub const PCR_N4_NTC: u64 = 0x00040000; // Next-To-Commit wrap

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
