/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2023 Arm Ltd.
 *
 * Based on arch/x86/include/asm/pkeys.h
 */

// #include dependencies are supplied by the surrounding kernel translation.

pub const ARCH_VM_PKEY_FLAGS: _ = VM_PKEY_BIT0 | VM_PKEY_BIT1 | VM_PKEY_BIT2;

#[inline]
pub const fn arch_max_pkey() -> i32 {
    8
}

extern "C" {
    pub fn arch_set_user_pkey_access(pkey: i32, init_val: ::core::ffi::c_ulong) -> i32;
}

#[inline]
pub unsafe fn arch_pkeys_enabled() -> bool {
    system_supports_poe()
}

#[inline]
pub unsafe fn vma_pkey(vma: *mut vm_area_struct) -> i32 {
    (((*vma).vm_flags & ARCH_VM_PKEY_FLAGS) >> VM_PKEY_SHIFT) as i32
}

#[inline]
pub unsafe fn arch_override_mprotect_pkey(
    vma: *mut vm_area_struct,
    _prot: i32,
    pkey: i32,
) -> i32 {
    if pkey != -1 {
        return pkey;
    }

    vma_pkey(vma)
}

#[inline]
pub unsafe fn execute_only_pkey(_mm: *mut mm_struct) -> i32 {
    // Execute-only mappings are handled by EPAN/FEAT_PAN3.
    -1
}

#[inline]
pub unsafe fn mm_pkey_allocation_map(mm: *mut mm_struct) -> _ {
    (*mm).context.pkey_allocation_map
}

#[macro_export]
macro_rules! mm_set_pkey_allocated {
    ($mm:expr, $pkey:expr) => {{
        (*$mm).context.pkey_allocation_map |= 1u32 << $pkey;
    }};
}

#[macro_export]
macro_rules! mm_set_pkey_free {
    ($mm:expr, $pkey:expr) => {{
        (*$mm).context.pkey_allocation_map &= !(1u32 << $pkey);
    }};
}

#[inline]
pub unsafe fn mm_pkey_is_allocated(mm: *mut mm_struct, pkey: i32) -> bool {
    /*
     * "Allocated" pkeys are those that have been returned
     * from pkey_alloc() or pkey 0 which is allocated
     * implicitly when the mm is created.
     */
    if pkey < 0 || pkey >= arch_max_pkey() {
        return false;
    }

    mm_pkey_allocation_map(mm) & (1u32 << pkey) != 0
}

/*
 * Returns a positive, 3-bit key on success, or -1 on failure.
 */
#[inline]
pub unsafe fn mm_pkey_alloc(mm: *mut mm_struct) -> i32 {
    /*
     * Note: this is the one and only place we make sure
     * that the pkey is valid as far as the hardware is
     * concerned.  The rest of the kernel trusts that
     * only good, valid pkeys come out of here.
     */
    let all_pkeys_mask: u8 = GENMASK(arch_max_pkey() - 1, 0);
    let ret: i32;

    if !arch_pkeys_enabled() {
        return -1;
    }

    /*
     * Are we out of pkeys?  We must handle this specially
     * because ffz() behavior is undefined if there are no
     * zeros.
     */
    if mm_pkey_allocation_map(mm) == all_pkeys_mask {
        return -1;
    }

    ret = ffz(mm_pkey_allocation_map(mm));

    mm_set_pkey_allocated!(mm, ret);

    ret
}

#[inline]
pub unsafe fn mm_pkey_free(mm: *mut mm_struct, pkey: i32) -> i32 {
    if !mm_pkey_is_allocated(mm, pkey) {
        return -EINVAL;
    }

    mm_set_pkey_free!(mm, pkey);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
