/* SPDX-License-Identifier: GPL-2.0 */

/*
 * User space memory access functions
 *
 * C header includes and build-time configuration are supplied by surrounding
 * translation units.  The `MODULE` and `CONFIG_ADDRESS_MASKING` conditions
 * are preserved below as Rust configuration conditions where applicable.
 */

#[cfg(not(feature = "module"))]
extern "C" {
    pub static mut USER_PTR_MAX: ::core::ffi::c_ulong;
}

#[cfg(feature = "module")]
#[inline(always)]
unsafe fn runtime_const_ptr<T>(sym: T) -> T {
    sym
}

#[cfg(feature = "address_masking")]
#[inline(always)]
pub unsafe fn __untagged_addr(mut addr: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    // Original C uses architecture-specific alternative inline assembly to
    // apply the per-CPU TLB untag mask.
    core::arch::asm!(
        "/* ALTERNATIVE: and per-CPU tlbstate_untag_mask, {addr} */",
        addr = inout(reg) addr,
        options(nostack, preserves_flags)
    );
    addr
}

#[cfg(feature = "address_masking")]
#[inline(always)]
pub unsafe fn untagged_addr<T>(addr: T) -> T {
    __untagged_addr(addr as ::core::ffi::c_ulong) as T
}

#[cfg(feature = "address_masking")]
#[inline(always)]
pub unsafe fn __untagged_addr_remote(
    mm: *mut mm_struct,
    addr: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    // mmap_assert_locked(mm);
    (*mm).context.untag_mask & addr
}

#[cfg(feature = "address_masking")]
#[inline(always)]
pub unsafe fn untagged_addr_remote<T>(mm: *mut mm_struct, addr: T) -> T {
    __untagged_addr_remote(mm, addr as ::core::ffi::c_ulong) as T
}

#[inline(always)]
pub unsafe fn valid_user_address<T>(x: T) -> bool {
    (x as ::core::ffi::c_ulong)
        <= #[cfg(feature = "module")] runtime_const_ptr(USER_PTR_MAX)
           #[cfg(not(feature = "module"))] USER_PTR_MAX
}

/*
 * Masking the user address is an alternative to a conditional
 * user_access_begin that can avoid the fencing. This only works
 * for dense accesses starting at the address.
 */
#[inline(always)]
pub unsafe fn mask_user_address(ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    let mut ret = ptr as *mut ::core::ffi::c_void;
    // Original C uses cmp/cmov assembly against USER_PTR_MAX.
    core::arch::asm!(
        "cmp {max}, {ret}",
        "cmova {max}, {ret}",
        max = in(reg) USER_PTR_MAX,
        ret = inout(reg) ret,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline(always)]
pub unsafe fn __access_ok(ptr: *const ::core::ffi::c_void, size: ::core::ffi::c_ulong) -> bool {
    if size <= PAGE_SIZE {
        valid_user_address(ptr)
    } else {
        let sum = size.wrapping_add(ptr as ::core::ffi::c_ulong);
        valid_user_address(sum) && sum >= ptr as ::core::ffi::c_ulong
    }
}

/* Copy To/From Userspace */

extern "C" {
    pub fn rep_movs_alternative(
        to: *mut ::core::ffi::c_void,
        from: *const ::core::ffi::c_void,
        len: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_ulong;
}

#[inline(always)]
pub unsafe fn copy_user_generic(
    mut to: *mut ::core::ffi::c_void,
    mut from: *const ::core::ffi::c_void,
    mut len: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    // stac();
    core::arch::asm!(
        "1:",
        "/* ALTERNATIVE: rep movsb / call rep_movs_alternative */",
        "2:",
        "+c"(len),
        inout("rdi") to,
        inout("rsi") from,
        lateout("rax") _,
        options(nostack)
    );
    // clac();
    len
}

#[inline(always)]
pub unsafe fn raw_copy_from_user(
    dst: *mut ::core::ffi::c_void,
    src: *const ::core::ffi::c_void,
    size: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    copy_user_generic(dst, src, size)
}

#[inline(always)]
pub unsafe fn raw_copy_to_user(
    dst: *mut ::core::ffi::c_void,
    src: *const ::core::ffi::c_void,
    size: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    copy_user_generic(dst, src, size)
}

extern "C" {
    pub fn copy_to_nontemporal(
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        size: usize,
    ) -> usize;
    pub fn copy_user_flushcache(
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        size: usize,
    ) -> usize;
}

#[inline(always)]
pub unsafe fn copy_from_user_inatomic_nontemporal(
    dst: *mut ::core::ffi::c_void,
    src: *const ::core::ffi::c_void,
    size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    // kasan_check_write(dst, size);
    let src = mask_user_address(src);
    // stac();
    let ret = copy_to_nontemporal(dst, src, size as usize) as ::core::ffi::c_long;
    // clac();
    ret as ::core::ffi::c_int
}

#[inline(always)]
pub unsafe fn copy_from_user_flushcache(
    dst: *mut ::core::ffi::c_void,
    src: *const ::core::ffi::c_void,
    size: usize,
) -> usize {
    // kasan_check_write(dst, size);
    copy_user_flushcache(dst, src, size)
}

/* Zero Userspace. */

extern "C" {
    pub fn rep_stos_alternative(
        addr: *mut ::core::ffi::c_void,
        len: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong;
}

#[inline(always)]
pub unsafe fn __clear_user(
    mut addr: *mut ::core::ffi::c_void,
    mut size: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    // might_fault(); stac();
    core::arch::asm!(
        "1:",
        "/* ALTERNATIVE: rep stosb / call rep_stos_alternative */",
        "2:",
        "+c"(size),
        inout("rdi") addr,
        in("rax") 0usize,
        options(nostack)
    );
    // clac();
    size
}

#[inline(always)]
pub unsafe fn clear_user(
    to: *mut ::core::ffi::c_void,
    n: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    if __access_ok(to, n) { __clear_user(to, n) } else { n }
}

// External symbols/types supplied by the surrounding translated kernel.
extern "C" {
    pub static PAGE_SIZE: ::core::ffi::c_ulong;
}
#[allow(non_camel_case_types)]
pub struct mm_struct {
    pub context: mm_context,
}
#[repr(C)]
pub struct mm_context {
    pub untag_mask: ::core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
