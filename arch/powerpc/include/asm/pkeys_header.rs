/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * PowerPC Memory Protection Keys management
 *
 * Copyright 2017, Ram Pai, IBM Corporation.
 */

/* Dependencies supplied by the surrounding kernel translation. */

pub static mut num_pkey: i32 = 0;
pub static mut reserved_allocation_mask: u32 = 0; /* bits set for reserved keys */

pub const ARCH_VM_PKEY_FLAGS: vm_flags_t =
    VM_PKEY_BIT0 | VM_PKEY_BIT1 | VM_PKEY_BIT2 | VM_PKEY_BIT3 | VM_PKEY_BIT4;

/* Override any generic PKEY permission defines. */
pub const PKEY_DISABLE_EXECUTE: u32 = 0x4;
pub const PKEY_ACCESS_MASK: u32 = PKEY_DISABLE_ACCESS | PKEY_DISABLE_WRITE | PKEY_DISABLE_EXECUTE;

#[inline]
pub fn pkey_to_vmflag_bits(pkey: u16) -> vm_flags_t {
    (((pkey as vm_flags_t) << VM_PKEY_SHIFT) & ARCH_VM_PKEY_FLAGS)
}

#[inline]
pub unsafe fn vma_pkey(vma: *mut vm_area_struct) -> i32 {
    if !mmu_has_feature(MMU_FTR_PKEY) {
        return 0;
    }
    ((*vma).vm_flags & ARCH_VM_PKEY_FLAGS) >> VM_PKEY_SHIFT
}

#[inline]
pub fn arch_max_pkey() -> i32 {
    unsafe { num_pkey }
}

#[macro_export]
macro_rules! pkey_alloc_mask {
    ($pkey:expr) => { 0x1u32 << ($pkey) };
}

#[macro_export]
macro_rules! mm_pkey_allocation_map {
    ($mm:expr) => { ($mm).context.pkey_allocation_map };
}

#[macro_export]
macro_rules! __mm_pkey_allocated {
    ($mm:expr, $pkey:expr) => {{
        mm_pkey_allocation_map!($mm) |= pkey_alloc_mask!($pkey);
    }};
}

#[macro_export]
macro_rules! __mm_pkey_free {
    ($mm:expr, $pkey:expr) => {{
        mm_pkey_allocation_map!($mm) &= !pkey_alloc_mask!($pkey);
    }};
}

#[macro_export]
macro_rules! __mm_pkey_is_allocated {
    ($mm:expr, $pkey:expr) => {
        mm_pkey_allocation_map!($mm) & pkey_alloc_mask!($pkey)
    };
}

#[macro_export]
macro_rules! __mm_pkey_is_reserved {
    ($pkey:expr) => {
        unsafe { reserved_allocation_mask } & pkey_alloc_mask!($pkey)
    };
}

#[inline]
pub unsafe fn mm_pkey_is_allocated(mm: *mut mm_struct, pkey: i32) -> bool {
    if pkey < 0 || pkey >= arch_max_pkey() {
        return false;
    }
    /* Reserved keys are never allocated. */
    if __mm_pkey_is_reserved!(pkey) != 0 {
        return false;
    }
    __mm_pkey_is_allocated!((*mm), pkey) != 0
}

/*
 * Returns a positive, 5-bit key on success, or -1 on failure.
 * Relies on the mmap_lock to protect against concurrency in mm_pkey_alloc() and
 * mm_pkey_free().
 */
#[inline]
pub unsafe fn mm_pkey_alloc(mm: *mut mm_struct) -> i32 {
    /* This is the one place where the hardware-valid pkey is established. */
    let all_pkeys_mask: u32 = !0u32;

    if !mmu_has_feature(MMU_FTR_PKEY) {
        return -1;
    }
    /* ffz() is undefined when there are no zero bits. */
    if mm_pkey_allocation_map!((*mm)) == all_pkeys_mask {
        return -1;
    }

    let ret = ffz(mm_pkey_allocation_map!((*mm)));
    __mm_pkey_allocated!((*mm), ret);
    ret
}

#[inline]
pub unsafe fn mm_pkey_free(mm: *mut mm_struct, pkey: i32) -> i32 {
    if !mmu_has_feature(MMU_FTR_PKEY) {
        return -1;
    }
    if !mm_pkey_is_allocated(mm, pkey) {
        return -EINVAL;
    }
    __mm_pkey_free!((*mm), pkey);
    0
}

/* Try to dedicate one protection key for execute-only protection. */
extern "C" {
    pub fn execute_only_pkey(mm: *mut mm_struct) -> i32;
    pub fn __arch_override_mprotect_pkey(vma: *mut vm_area_struct, prot: i32, pkey: i32) -> i32;
}

#[inline]
pub unsafe fn arch_override_mprotect_pkey(vma: *mut vm_area_struct, prot: i32, pkey: i32) -> i32 {
    if !mmu_has_feature(MMU_FTR_PKEY) {
        return 0;
    }
    /* Never override a value supplied by the user to mprotect_pkey(). */
    if pkey != -1 {
        return pkey;
    }
    __arch_override_mprotect_pkey(vma, prot, pkey)
}

extern "C" {
    pub fn __arch_set_user_pkey_access(pkey: i32, init_val: c_ulong) -> i32;
}

#[inline]
pub unsafe fn arch_set_user_pkey_access(pkey: i32, init_val: c_ulong) -> i32 {
    if !mmu_has_feature(MMU_FTR_PKEY) {
        return -EINVAL;
    }
    /* Userspace must not change pkey-0 permissions. */
    if pkey == 0 {
        return if init_val != 0 { -EINVAL } else { 0 };
    }
    __arch_set_user_pkey_access(pkey, init_val)
}

#[inline]
pub fn arch_pkeys_enabled() -> bool {
    mmu_has_feature(MMU_FTR_PKEY)
}

extern "C" {
    pub fn pkey_mm_init(mm: *mut mm_struct);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
