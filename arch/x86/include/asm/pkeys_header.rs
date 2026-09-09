/* SPDX-License-Identifier: GPL-2.0 */

/*
 * If more than 16 keys are ever supported, a thorough audit
 * will be necessary to ensure that the types that store key
 * numbers and masks have sufficient capacity.
 */
#[inline]
pub unsafe fn arch_max_pkey() -> i32 {
    if cpu_feature_enabled(X86_FEATURE_OSPKE) { 16 } else { 1 }
}

extern "C" {
    pub fn arch_set_user_pkey_access(pkey: i32, init_val: ::core::ffi::c_ulong) -> i32;
}

#[inline]
pub unsafe fn arch_pkeys_enabled() -> bool {
    cpu_feature_enabled(X86_FEATURE_OSPKE)
}

/*
 * Try to dedicate one of the protection keys to be used as an
 * execute-only protection key.
 */
extern "C" {
    pub fn __execute_only_pkey(mm: *mut mm_struct) -> i32;
}

#[inline]
pub unsafe fn execute_only_pkey(mm: *mut mm_struct) -> i32 {
    if !cpu_feature_enabled(X86_FEATURE_OSPKE) {
        return ARCH_DEFAULT_PKEY;
    }

    __execute_only_pkey(mm)
}

extern "C" {
    pub fn __arch_override_mprotect_pkey(
        vma: *mut vm_area_struct,
        prot: i32,
        pkey: i32,
    ) -> i32;
}

#[inline]
pub unsafe fn arch_override_mprotect_pkey(
    vma: *mut vm_area_struct,
    prot: i32,
    pkey: i32,
) -> i32 {
    if !cpu_feature_enabled(X86_FEATURE_OSPKE) {
        return 0;
    }

    __arch_override_mprotect_pkey(vma, prot, pkey)
}

pub const ARCH_VM_PKEY_FLAGS: ::core::ffi::c_ulong =
    VM_PKEY_BIT0 | VM_PKEY_BIT1 | VM_PKEY_BIT2 | VM_PKEY_BIT3;

#[inline]
pub unsafe fn mm_pkey_allocation_map(mm: *mut mm_struct) -> u32 {
    (*mm).context.pkey_allocation_map
}

#[inline]
pub unsafe fn mm_set_pkey_allocated(mm: *mut mm_struct, pkey: i32) {
    (*mm).context.pkey_allocation_map |= 1u32 << (pkey as u32);
}

#[inline]
pub unsafe fn mm_set_pkey_free(mm: *mut mm_struct, pkey: i32) {
    (*mm).context.pkey_allocation_map &= !(1u32 << (pkey as u32));
}

#[inline]
pub unsafe fn mm_pkey_is_allocated(mm: *mut mm_struct, pkey: i32) -> bool {
    /*
     * "Allocated" pkeys are those that have been returned
     * from pkey_alloc() or pkey 0 which is allocated
     * implicitly when the mm is created.
     */
    if pkey < 0 {
        return false;
    }
    if pkey >= arch_max_pkey() {
        return false;
    }
    /*
     * The exec-only pkey is set in the allocation map, but
     * is not available to any of the user interfaces like
     * mprotect_pkey().
     */
    if pkey == (*mm).context.execute_only_pkey {
        return false;
    }

    mm_pkey_allocation_map(mm) & (1u32 << (pkey as u32)) != 0
}

/*
 * Returns a positive, 4-bit key on success, or -1 on failure.
 */
#[inline]
pub unsafe fn mm_pkey_alloc(mm: *mut mm_struct) -> i32 {
    /*
     * Note: this is the one and only place we make sure
     * that the pkey is valid as far as the hardware is
     * concerned.  The rest of the kernel trusts that
     * only good, valid pkeys come out of here.
     */
    let all_pkeys_mask: u16 = ((1u32 << (arch_max_pkey() as u32)) - 1) as u16;

    if !arch_pkeys_enabled() {
        return -1;
    }

    /*
     * Are we out of pkeys?  We must handle this specially
     * because ffz() behavior is undefined if there are no
     * zeros.
     */
    if mm_pkey_allocation_map(mm) as u16 == all_pkeys_mask {
        return -1;
    }

    let ret = ffz(mm_pkey_allocation_map(mm));

    mm_set_pkey_allocated(mm, ret);

    ret
}

#[inline]
pub unsafe fn mm_pkey_free(mm: *mut mm_struct, pkey: i32) -> i32 {
    if !mm_pkey_is_allocated(mm, pkey) {
        return -EINVAL;
    }

    mm_set_pkey_free(mm, pkey);

    0
}

#[inline]
pub unsafe fn vma_pkey(vma: *mut vm_area_struct) -> i32 {
    let vma_pkey_mask: ::core::ffi::c_ulong =
        VM_PKEY_BIT0 | VM_PKEY_BIT1 | VM_PKEY_BIT2 | VM_PKEY_BIT3;

    (((*vma).vm_flags & vma_pkey_mask) >> VM_PKEY_SHIFT) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
