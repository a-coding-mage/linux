/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * kexec for arm64
 *
 * Copyright (C) Linaro.
 * Copyright (C) Huawei Futurewei Technologies.
 */

/* Maximum physical address we can use pages from */
pub const KEXEC_SOURCE_MEMORY_LIMIT: usize = usize::MAX;

/* Maximum address we can reach in physical address mode */
pub const KEXEC_DESTINATION_MEMORY_LIMIT: usize = usize::MAX;

/* Maximum address we can use for the control code buffer */
pub const KEXEC_CONTROL_MEMORY_LIMIT: usize = usize::MAX;

pub const KEXEC_CONTROL_PAGE_SIZE: usize = 4096;

/* Requires the externally supplied KEXEC_ARCH_AARCH64 constant. */
pub const KEXEC_ARCH: u32 = KEXEC_ARCH_AARCH64;

/**
 * crash_setup_regs() - save registers for the panic kernel
 *
 * @newregs: registers are saved here
 * @oldregs: registers to be saved (may be %NULL)
 */
pub unsafe fn crash_setup_regs(newregs: *mut pt_regs, oldregs: *mut pt_regs) {
    if !oldregs.is_null() {
        core::ptr::copy_nonoverlapping(
            oldregs,
            newregs,
            core::mem::size_of::<pt_regs>(),
        );
    } else {
        let mut tmp1: u64;
        let mut tmp2: u64;

        core::arch::asm!(
            "stp x0, x1, [{newregs}, #16 * 0]",
            "stp x2, x3, [{newregs}, #16 * 1]",
            "stp x4, x5, [{newregs}, #16 * 2]",
            "stp x6, x7, [{newregs}, #16 * 3]",
            "stp x8, x9, [{newregs}, #16 * 4]",
            "stp x10, x11, [{newregs}, #16 * 5]",
            "stp x12, x13, [{newregs}, #16 * 6]",
            "stp x14, x15, [{newregs}, #16 * 7]",
            "stp x16, x17, [{newregs}, #16 * 8]",
            "stp x18, x19, [{newregs}, #16 * 9]",
            "stp x20, x21, [{newregs}, #16 * 10]",
            "stp x22, x23, [{newregs}, #16 * 11]",
            "stp x24, x25, [{newregs}, #16 * 12]",
            "stp x26, x27, [{newregs}, #16 * 13]",
            "stp x28, x29, [{newregs}, #16 * 14]",
            "mov {tmp1}, sp",
            "stp x30, {tmp1}, [{newregs}, #16 * 15]",
            "mrs {tmp1}, CurrentEL",
            "mrs {tmp2}, SPSEL",
            "orr {tmp1}, {tmp1}, {tmp2}",
            "mrs {tmp2}, DAIF",
            "orr {tmp1}, {tmp1}, {tmp2}",
            "mrs {tmp2}, NZCV",
            "orr {tmp1}, {tmp1}, {tmp2}",
            "adr {tmp2}, 1f",
            "1:",
            "stp {tmp2}, {tmp1}, [{newregs}, #16 * 16]",
            newregs = in(reg) newregs,
            tmp1 = lateout(reg) tmp1,
            tmp2 = lateout(reg) tmp2,
            options(nostack),
        );
    }
}

/* These declarations are enabled when CONFIG_CRASH_DUMP and CONFIG_HIBERNATION are set. */
#[cfg(all(feature = "CONFIG_CRASH_DUMP", feature = "CONFIG_HIBERNATION"))]
unsafe extern "C" {
    pub fn crash_is_nosave(pfn: c_ulong) -> bool;
    pub fn crash_prepare_suspend();
    pub fn crash_post_resume();
    pub fn crash_free_reserved_phys_range(begin: c_ulong, end: c_ulong);
}

#[cfg(not(all(feature = "CONFIG_CRASH_DUMP", feature = "CONFIG_HIBERNATION")))]
pub unsafe fn crash_is_nosave(_pfn: c_ulong) -> bool { false }
#[cfg(not(all(feature = "CONFIG_CRASH_DUMP", feature = "CONFIG_HIBERNATION")))]
pub unsafe fn crash_prepare_suspend() {}
#[cfg(not(all(feature = "CONFIG_CRASH_DUMP", feature = "CONFIG_HIBERNATION")))]
pub unsafe fn crash_post_resume() {}

#[repr(C)]
pub struct kimage;

/* Enabled when CONFIG_KEXEC_CORE is set. */
#[cfg(feature = "CONFIG_KEXEC_CORE")]
unsafe extern "C" {
    pub fn cpu_soft_restart(el2_switch: c_ulong, entry: c_ulong, arg0: c_ulong,
                            arg1: c_ulong, arg2: c_ulong);
    pub fn machine_kexec_post_load(image: *mut kimage) -> c_int;
}

pub const ARCH_HAS_KIMAGE_ARCH: bool = true;

#[repr(C)]
pub struct kimage_arch {
    pub dtb: *mut core::ffi::c_void,
    pub dtb_mem: phys_addr_t,
    pub kern_reloc: phys_addr_t,
    pub el2_vectors: phys_addr_t,
    pub ttbr0: phys_addr_t,
    pub ttbr1: phys_addr_t,
    pub zero_page: phys_addr_t,
    pub phys_offset: c_ulong,
    pub t0sz: c_ulong,
}

/* Enabled when CONFIG_KEXEC_FILE is set. */
#[cfg(feature = "CONFIG_KEXEC_FILE")]
unsafe extern "C" {
    pub static kexec_image_ops: kexec_file_ops;
    pub fn arch_kimage_file_post_load_cleanup(image: *mut kimage) -> c_int;
    pub fn load_other_segments(image: *mut kimage, kernel_load_addr: c_ulong,
                               kernel_size: c_ulong, initrd: *mut c_char,
                               initrd_len: c_ulong, cmdline: *mut c_char) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
