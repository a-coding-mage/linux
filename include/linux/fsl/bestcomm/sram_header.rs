/*
 * Handling of a sram zone for bestcomm
 *
 *
 * Copyright (C) 2007 Sylvain Munaut <tnt@246tNt.com>
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2. This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

// Dependencies supplied by the surrounding kernel environment:
// asm/rheap.h, asm/mmu.h, linux/spinlock.h

/* Structure used internally */
/* The internals are here for the inline functions
 * sake, certainly not for the user to mess with !
 */
#[repr(C)]
pub struct bcom_sram {
    pub base_phys: phys_addr_t,
    pub base_virt: *mut core::ffi::c_void,
    pub size: core::ffi::c_uint,
    pub rh: *mut rh_info_t,
    pub lock: spinlock_t,
}

extern "C" {
    pub static mut bcom_sram: *mut bcom_sram;
}

/* Public API */
extern "C" {
    pub fn bcom_sram_init(sram_node: *mut device_node, owner: *mut core::ffi::c_char)
        -> core::ffi::c_int;
    pub fn bcom_sram_cleanup();

    pub fn bcom_sram_alloc(
        size: core::ffi::c_int,
        align: core::ffi::c_int,
        phys: *mut phys_addr_t,
    ) -> *mut core::ffi::c_void;
    pub fn bcom_sram_free(ptr: *mut core::ffi::c_void);
}

pub unsafe fn bcom_sram_va2pa(va: *mut core::ffi::c_void) -> phys_addr_t {
    (*bcom_sram).base_phys
        + (va as *mut u8).offset_from((*bcom_sram).base_virt as *mut u8) as core::ffi::c_ulong
}

pub unsafe fn bcom_sram_pa2va(pa: phys_addr_t) -> *mut core::ffi::c_void {
    ((*bcom_sram).base_virt as *mut u8)
        .add((pa - (*bcom_sram).base_phys) as usize)
        as *mut core::ffi::c_void
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
