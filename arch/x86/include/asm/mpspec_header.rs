/* SPDX-License-Identifier: GPL-2.0 */

// Translated from asm/mpspec.h.  The following names are supplied by the
// corresponding Linux/Rust dependencies.

use core::ffi::c_int;

extern "C" {
    pub static mut pic_mode: c_int;
}

#[cfg(all(target_arch = "x86", not(target_pointer_width = "64"), feature = "config_base_small"))]
pub const MAX_MP_BUSSES: usize = 32;
#[cfg(all(target_arch = "x86", not(target_pointer_width = "64"), not(feature = "config_base_small")))]
pub const MAX_MP_BUSSES: usize = 260;

#[cfg(all(target_arch = "x86", not(target_pointer_width = "64")))]
pub const MAX_IRQ_SOURCES: usize = 256;

#[cfg(target_arch = "x86_64")]
pub const MAX_MP_BUSSES: usize = 256;
#[cfg(target_arch = "x86_64")]
pub const MAX_IRQ_SOURCES: usize = MAX_MP_BUSSES * 4;

#[cfg(feature = "config_eisa")]
extern "C" {
    pub static mut mp_bus_id_to_type: [c_int; MAX_MP_BUSSES];
}

// DECLARE_BITMAP(mp_bus_not_pci, MAX_MP_BUSSES)
extern "C" {
    pub static mut mp_bus_not_pci: [usize; (MAX_MP_BUSSES + usize::BITS as usize - 1) / usize::BITS as usize];
    pub static mut boot_cpu_physical_apicid: u32;
    pub static mut boot_cpu_apic_version: u8;
}

#[cfg(feature = "config_x86_local_apic")]
extern "C" {
    pub static mut smp_found_config: c_int;
}
#[cfg(not(feature = "config_x86_local_apic"))]
pub const smp_found_config: c_int = 0;

#[cfg(feature = "config_x86_mpparse")]
extern "C" {
    pub fn e820__memblock_alloc_reserved_mpc_new();
    pub static mut enable_update_mptable: c_int;
    pub fn mpparse_find_mptable();
    pub fn mpparse_parse_early_smp_config();
    pub fn mpparse_parse_smp_config();
}

#[cfg(not(feature = "config_x86_mpparse"))]
#[inline]
pub unsafe fn e820__memblock_alloc_reserved_mpc_new() {}

#[cfg(not(feature = "config_x86_mpparse"))]
pub const enable_update_mptable: c_int = 0;

#[cfg(not(feature = "config_x86_mpparse"))]
#[inline]
pub unsafe fn mpparse_find_mptable() {
    // C macro alias: x86_init_noop
}

#[cfg(not(feature = "config_x86_mpparse"))]
#[inline]
pub unsafe fn mpparse_parse_early_smp_config() {
    // C macro alias: x86_init_noop
}

#[cfg(not(feature = "config_x86_mpparse"))]
#[inline]
pub unsafe fn mpparse_parse_smp_config() {
    // C macro alias: x86_init_noop
}

// DECLARE_BITMAP(phys_cpu_present_map, MAX_LOCAL_APIC)
extern "C" {
    pub static mut phys_cpu_present_map: [usize; (MAX_LOCAL_APIC + usize::BITS as usize - 1) / usize::BITS as usize];
}

#[inline]
pub unsafe fn reset_phys_cpu_present_map(apicid: u32) {
    // bitmap_zero(phys_cpu_present_map, MAX_LOCAL_APIC);
    for word in core::slice::from_raw_parts_mut(
        core::ptr::addr_of_mut!(phys_cpu_present_map) as *mut usize,
        (MAX_LOCAL_APIC + usize::BITS as usize - 1) / usize::BITS as usize,
    ) {
        core::ptr::write_volatile(word, 0);
    }
    // set_bit(apicid, phys_cpu_present_map);
    let index = (apicid as usize) / usize::BITS as usize;
    let shift = (apicid as usize) % usize::BITS as usize;
    (*core::ptr::addr_of_mut!(phys_cpu_present_map).cast::<usize>().add(index)) |= 1usize << shift;
}

#[inline]
pub unsafe fn copy_phys_cpu_present_map(dst: *mut usize) {
    // bitmap_copy(dst, phys_cpu_present_map, MAX_LOCAL_APIC);
    core::ptr::copy_nonoverlapping(
        core::ptr::addr_of!(phys_cpu_present_map).cast::<usize>(),
        dst,
        (MAX_LOCAL_APIC + usize::BITS as usize - 1) / usize::BITS as usize,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
