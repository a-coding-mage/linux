/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* CONFIG_SECRETMEM is a build-time configuration condition from the C header. */

#[cfg(CONFIG_SECRETMEM)]
extern "C" {
    pub static secretmem_aops: address_space_operations;

    pub fn vma_is_secretmem(vma: *mut vm_area_struct) -> bool;
    pub fn secretmem_active() -> bool;
}

#[cfg(CONFIG_SECRETMEM)]
#[inline]
pub unsafe fn secretmem_mapping(mapping: *mut address_space) -> bool {
    (*mapping).a_ops == &secretmem_aops as *const address_space_operations
}

#[cfg(not(CONFIG_SECRETMEM))]
#[inline]
pub unsafe fn vma_is_secretmem(_vma: *mut vm_area_struct) -> bool {
    false
}

#[cfg(not(CONFIG_SECRETMEM))]
#[inline]
pub unsafe fn secretmem_mapping(_mapping: *mut address_space) -> bool {
    false
}

#[cfg(not(CONFIG_SECRETMEM))]
#[inline]
pub fn secretmem_active() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
