/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes <linux/mm.h> and conditionally includes
// <asm/pkeys.h>. Those declarations are supplied by other translated files.

pub const ARCH_DEFAULT_PKEY: i32 = 0;

// CONFIG_ARCH_HAS_PKEYS selects the architecture-specific implementation.
// The following items are the !CONFIG_ARCH_HAS_PKEYS fallback definitions.
pub const PKEY_DEDICATED_EXECUTE_ONLY: i32 = 0;
pub const ARCH_VM_PKEY_FLAGS: i32 = 0;

#[inline]
pub unsafe fn arch_max_pkey() -> i32 {
    1
}

#[inline]
pub unsafe fn execute_only_pkey(_mm: *mut mm_struct) -> i32 {
    0
}

#[inline]
pub unsafe fn arch_override_mprotect_pkey(
    _vma: *mut vm_area_struct,
    _prot: i32,
    _pkey: i32,
) -> i32 {
    0
}

#[inline]
pub unsafe fn vma_pkey(_vma: *mut vm_area_struct) -> i32 {
    0
}

#[inline]
pub unsafe fn mm_pkey_is_allocated(_mm: *mut mm_struct, pkey: i32) -> bool {
    pkey == 0
}

#[inline]
pub unsafe fn mm_pkey_alloc(_mm: *mut mm_struct) -> i32 {
    -1
}

#[inline]
pub unsafe fn mm_pkey_free(_mm: *mut mm_struct, _pkey: i32) -> i32 {
    -EINVAL
}

#[inline]
pub unsafe fn arch_set_user_pkey_access(
    _tsk: *mut task_struct,
    _pkey: i32,
    _init_val: c_ulong,
) -> i32 {
    0
}

#[inline]
pub unsafe fn arch_pkeys_enabled() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
