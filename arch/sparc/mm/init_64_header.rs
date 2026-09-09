/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding translation unit: asm/page.h

/* Most of the symbols in this file are defined in init.c and
 * marked non-static so that assembler code can get at them.
 */

pub const MAX_PHYS_ADDRESS: usize = 1usize << MAX_PHYS_ADDRESS_BITS;

unsafe extern "C" {
    pub static mut kern_linear_pte_xor: [core::ffi::c_ulong; 4];
    pub static mut sparc64_highest_unlocked_tlb_ent: core::ffi::c_uint;
    pub static mut sparc64_kern_pri_context: core::ffi::c_ulong;
    pub static mut sparc64_kern_pri_nuc_bits: core::ffi::c_ulong;
    pub static mut sparc64_kern_sec_context: core::ffi::c_ulong;
    pub fn mmu_info(m: *mut seq_file);
}

#[repr(C)]
pub struct linux_prom_translation {
    pub virt: core::ffi::c_ulong,
    pub size: core::ffi::c_ulong,
    pub data: core::ffi::c_ulong,
}

/* Exported for kernel TLB miss handling in ktlb.S */
unsafe extern "C" {
    pub static mut prom_trans: [linux_prom_translation; 512];
    pub static mut prom_trans_ents: core::ffi::c_uint;
}

/* Exported for SMP bootup purposes. */
unsafe extern "C" {
    pub static mut kern_locked_tte_data: core::ffi::c_ulong;
}

unsafe extern "C" {
    pub fn prom_world(enter: core::ffi::c_int);
}

// Opaque declaration corresponding to struct seq_file from the kernel headers.
#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
