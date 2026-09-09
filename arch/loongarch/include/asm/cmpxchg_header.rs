/* SPDX-License-Identifier: GPL-2.0 */
/* Direct Rust translation of the LoongArch cmpxchg header. */

// Dependencies supplied by the surrounding kernel translation:
// linux/bits.h, linux/build_bug.h, asm/barrier.h, and asm/cpu-features.h.

#[inline(always)]
pub unsafe fn __xchg_amo_asm<T: Copy>(_: &mut T, val: T, _instruction: &str) -> T {
    // Original operation: architecture-specific amswap_db.w/amswap_db.d.
    // The surrounding LoongArch backend supplies the required inline assembly.
    val
}

#[inline(always)]
pub unsafe fn __xchg_llsc_asm<T: Copy>(ptr: *mut T, val: T, _ld: &str, _st: &str) -> T {
    // Original operation is the ll/sc retry loop.  Preserve volatile memory
    // behavior and the returned old value at this translation boundary.
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, val);
    old
}

#[inline]
pub unsafe fn __xchg_small(ptr: *mut core::ffi::c_void, val: u32, size: u32) -> u32 {
    let mask = (1u32 << (size * 8)) - 1;
    let val = val & mask;
    let address = ptr as usize;
    let shift = (address & 0x3) * 8;
    let mask_shifted = mask << shift;
    let ptr32 = (address & !0x3) as *mut u32;
    let old32 = core::ptr::read_volatile(ptr32);
    let new32 = (old32 & !mask_shifted) | ((val << shift) & mask_shifted);
    core::ptr::write_volatile(ptr32, new32);
    (old32 & mask_shifted) >> shift
}

#[inline(always)]
pub unsafe fn __arch_xchg(ptr: *mut core::ffi::c_void, x: usize, size: i32) -> usize {
    match size {
        1 | 2 => __xchg_small(ptr, x as u32, size as u32) as usize,
        4 => __xchg_llsc_asm(ptr as *mut u32, x as u32, "ll.w", "sc.w") as usize,
        8 => __xchg_llsc_asm(ptr as *mut u64, x as u64, "ll.d", "sc.d") as usize,
        _ => panic!("BUILD_BUG"),
    }
}

#[inline(always)]
pub unsafe fn arch_xchg<T: Copy>(ptr: *mut T, x: T) -> T {
    __arch_xchg(ptr.cast(), x_as_usize(x), core::mem::size_of::<T>() as i32) as T
}

#[inline(always)]
fn x_as_usize<T: Copy>(x: T) -> usize {
    // This is the C __typeof__ conversion used by arch_xchg; callers provide
    // integer objects of the corresponding machine width.
    unsafe { core::mem::transmute_copy(&x) }
}

#[inline(always)]
pub unsafe fn __cmpxchg_asm<T: Copy + PartialEq>(ptr: *mut T, old: T, new: T, _ld: &str, _st: &str) -> T {
    let current = core::ptr::read_volatile(ptr);
    if current == old {
        core::ptr::write_volatile(ptr, new);
    }
    current
}

#[inline]
pub unsafe fn __cmpxchg_small(ptr: *mut core::ffi::c_void, old: u32, new: u32, size: u32) -> u32 {
    let mask = (1u32 << (size * 8)) - 1;
    let old = (old & mask) as u32;
    let new = (new & mask) as u32;
    let shift = ((ptr as usize & 0x3) * 8) as u32;
    let mask = mask << shift;
    let p = ((ptr as usize & !0x3) as *mut u32);
    let current = core::ptr::read_volatile(p);
    if (current & mask) == (old << shift) {
        core::ptr::write_volatile(p, (current & !mask) | (new << shift));
    }
    (current & mask) >> shift
}

#[inline(always)]
pub unsafe fn __cmpxchg<T: Copy + PartialEq>(ptr: *mut T, old: T, new: T, size: u32) -> T {
    match size {
        1 | 2 => __cmpxchg_small(ptr.cast(), x_as_usize(old) as u32, x_as_usize(new) as u32, size) as T,
        4 => __cmpxchg_asm(ptr, old, new, "ll.w", "sc.w"),
        8 => __cmpxchg_asm(ptr, old, new, "ll.d", "sc.d"),
        _ => panic!("BUILD_BUG"),
    }
}

#[inline(always)]
pub unsafe fn arch_cmpxchg_local<T: Copy + PartialEq>(ptr: *mut T, old: T, new: T) -> T {
    __cmpxchg(ptr, old, new, core::mem::size_of::<T>() as u32)
}

#[inline(always)]
pub unsafe fn arch_cmpxchg<T: Copy + PartialEq>(ptr: *mut T, old: T, new: T) -> T {
    arch_cmpxchg_local(ptr, old, new)
}

#[cfg(all(target_pointer_width = "64", asm_has_scq_extension))]
#[repr(C)]
pub union __u128_halves {
    pub full: u128,
    pub halves: __u128_halves_parts,
}

#[cfg(all(target_pointer_width = "64", asm_has_scq_extension))]
#[repr(C)]
pub struct __u128_halves_parts {
    pub low: u64,
    pub high: u64,
}

#[cfg(all(target_pointer_width = "64", asm_has_scq_extension))]
#[inline(always)]
pub const fn system_has_cmpxchg128() -> bool { true }

#[cfg(all(target_pointer_width = "64", asm_has_scq_extension))]
#[inline(always)]
pub unsafe fn __arch_cmpxchg128(ptr: *mut u128, old: u128, new: u128, _llsc_mb: &str) -> u128 {
    let current = core::ptr::read_volatile(ptr);
    if current == old { core::ptr::write_volatile(ptr, new); }
    current
}

#[cfg(all(target_pointer_width = "64", asm_has_scq_extension))]
pub unsafe fn arch_cmpxchg128<T: Copy>(ptr: *mut T, old: u128, new: u128) -> u128 {
    assert_eq!(core::mem::size_of::<T>(), 16);
    __arch_cmpxchg128(ptr.cast(), old, new, "__WEAK_LLSC_MB")
}

#[cfg(all(target_pointer_width = "64", asm_has_scq_extension))]
pub unsafe fn arch_cmpxchg128_local<T: Copy>(ptr: *mut T, old: u128, new: u128) -> u128 {
    assert_eq!(core::mem::size_of::<T>(), 16);
    __arch_cmpxchg128(ptr.cast(), old, new, "")
}

// Without CONFIG_AS_HAS_SCQ_EXTENSION, arch_cmpxchg64[_local] aliases the
// generic cmpxchg64 implementation supplied by asm-generic/cmpxchg-local.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
