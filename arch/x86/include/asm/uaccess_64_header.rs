/* SPDX-License-Identifier: GPL-2.0 */

/*
 * User space memory access functions
 */
// Dependencies: linux/compiler.h, linux/lockdep.h, linux/kasan-checks.h,
// asm/alternative.h, asm/cpufeatures.h, asm/page.h, asm/percpu.h,
// asm/runtime-const.h (not for MODULE).


extern "C" {
    pub static USER_PTR_MAX: core::ffi::c_ulong;
}

/// `runtime_const_ptr(USER_PTR_MAX)`: a boot-patched immediate in the kernel
/// image, a plain load in modules (which cannot use runtime constants).
#[inline(always)]
pub unsafe fn __user_ptr_max() -> core::ffi::c_ulong {
    #[cfg(MODULE)]
    {
        USER_PTR_MAX
    }
    #[cfg(not(MODULE))]
    {
        runtime_const_ptr!(USER_PTR_MAX)
    }
}

/*
 * Mask out tag bits from the address.
 */
#[cfg(CONFIG_ADDRESS_MASKING)]
#[inline]
pub unsafe fn __untagged_addr(mut addr: core::ffi::c_ulong) -> core::ffi::c_ulong {
    core::arch::asm!(
        ALTERNATIVE!("", "and %gs:{mask}(%rip), {addr}", "ft"),
        addr = inout(reg) addr,
        mask = sym tlbstate_untag_mask,
        ft = const X86_FEATURE_LAM,
        options(att_syntax, nostack, readonly),
    );
    addr
}

#[cfg(CONFIG_ADDRESS_MASKING)]
#[macro_export]
macro_rules! untagged_addr {
    ($addr:expr) => {
        __untagged_addr(($addr) as core::ffi::c_ulong) as _
    };
}

#[cfg(CONFIG_ADDRESS_MASKING)]
#[inline]
pub unsafe fn __untagged_addr_remote(mm: *mut mm_struct, addr: core::ffi::c_ulong) -> core::ffi::c_ulong {
    mmap_assert_locked(mm);
    addr & (*mm).context.untag_mask
}

#[cfg(CONFIG_ADDRESS_MASKING)]
#[macro_export]
macro_rules! untagged_addr_remote {
    ($mm:expr, $addr:expr) => {
        __untagged_addr_remote($mm, ($addr) as core::ffi::c_ulong) as _
    };
}

#[macro_export]
macro_rules! valid_user_address {
    ($x:expr) => {
        likely(($x) as core::ffi::c_ulong <= __user_ptr_max())
    };
}

/*
 * Masking the user address is an alternative to a conditional
 * user_access_begin that can avoid the fencing. This only works
 * for dense accesses starting at the address.
 */
#[inline]
pub unsafe fn mask_user_address(ptr: *const core::ffi::c_void) -> *mut core::ffi::c_void {
    let mut ret = ptr as *mut core::ffi::c_void;
    core::arch::asm!(
        "cmp {max}, {ret}",
        "cmova {max}, {ret}",
        ret = inout(reg) ret,
        max = in(reg) __user_ptr_max(),
        options(att_syntax, pure, nomem, nostack),
    );
    ret
}

#[macro_export]
macro_rules! masked_user_access_begin {
    ($x:expr) => {{
        let __masked_ptr = mask_user_address(($x).cast()).cast();
        __uaccess_begin();
        __masked_ptr
    }};
}

/*
 * User pointers can have tag bits on x86-64.  This scheme tolerates
 * arbitrary values in those bits rather then masking them off.
 *
 * Enforce two rules:
 * 1. 'ptr' must be in the user part of the address space
 * 2. 'ptr+size' must not overflow into kernel addresses
 *
 * Note that we always have at least one guard page between the
 * max user address and the non-canonical gap, allowing us to
 * ignore small sizes entirely.
 *
 * In fact, we could probably remove the size check entirely, since
 * any kernel accesses will be in increasing address order starting
 * at 'ptr'.
 *
 * That's a separate optimization, for now just handle the small
 * constant case.
 */
#[inline]
pub unsafe fn __access_ok(ptr: *const core::ffi::c_void, size: core::ffi::c_ulong) -> bool {
    // C takes the short path only for constant sizes; thanks to the guard
    // page it is equally valid for any size up to PAGE_SIZE.
    if size <= PAGE_SIZE as core::ffi::c_ulong {
        valid_user_address!(ptr)
    } else {
        let sum = size.wrapping_add(ptr as core::ffi::c_ulong);

        valid_user_address!(sum) && sum >= ptr as core::ffi::c_ulong
    }
}

/*
 * Copy To/From Userspace
 */

extern "C" {
    /* Handles exceptions in both to and from, but doesn't do access_ok */
    pub fn rep_movs_alternative(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, len: core::ffi::c_uint) -> core::ffi::c_ulong;
}

#[inline(always)]
#[must_use]
pub unsafe fn copy_user_generic(mut to: *mut core::ffi::c_void, mut from: *const core::ffi::c_void, mut len: core::ffi::c_ulong) -> core::ffi::c_ulong {
    stac();
    /*
     * If CPU has FSRM feature, use 'rep movs'.
     * Otherwise, use rep_movs_alternative.
     */
    core::arch::asm!(
        "1:",
        ALTERNATIVE!("rep movsb", "call rep_movs_alternative", "ft"),
        "2:",
        ".pushsection __ex_table, \"aM\", @progbits, 12",
        ".balign 4",
        ".long 1b - .",
        ".long 2b - .",
        ".long 3", /* EX_TYPE_UACCESS */
        ".popsection",
        ft = const ALT_NOT(X86_FEATURE_FSRM),
        inout("rcx") len,
        inout("rdi") to,
        inout("rsi") from,
        out("rax") _,
        options(att_syntax),
    );
    let _ = (to, from);
    clac();
    len
}

#[inline(always)]
#[must_use]
pub unsafe fn raw_copy_from_user(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: core::ffi::c_ulong) -> core::ffi::c_ulong {
    copy_user_generic(dst, src, size)
}

#[inline(always)]
#[must_use]
pub unsafe fn raw_copy_to_user(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: core::ffi::c_ulong) -> core::ffi::c_ulong {
    copy_user_generic(dst, src, size)
}

extern "C" {
    pub fn copy_to_nontemporal(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize) -> usize;
    pub fn copy_user_flushcache(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize) -> usize;
}

#[inline]
pub unsafe fn copy_from_user_inatomic_nontemporal(dst: *mut core::ffi::c_void, mut src: *const core::ffi::c_void, size: core::ffi::c_uint) -> core::ffi::c_int {
    kasan_check_write(dst, size as _);
    src = mask_user_address(src);
    stac();
    let ret = copy_to_nontemporal(dst, src, size as usize) as core::ffi::c_long;
    clac();
    ret as core::ffi::c_int
}

#[inline]
pub unsafe fn copy_from_user_flushcache(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize) -> usize {
    kasan_check_write(dst, size as _);
    copy_user_flushcache(dst, src, size)
}

/*
 * Zero Userspace.
 */

extern "C" {
    pub fn rep_stos_alternative(addr: *mut core::ffi::c_void, len: core::ffi::c_ulong) -> core::ffi::c_ulong;
}

#[inline(always)]
#[must_use]
pub unsafe fn __clear_user(mut addr: *mut core::ffi::c_void, mut size: core::ffi::c_ulong) -> core::ffi::c_ulong {
    might_fault();
    stac();

    /*
     * No memory constraint because it doesn't change any memory gcc
     * knows about.
     */
    core::arch::asm!(
        "1:",
        ALTERNATIVE!("rep stosb", "call rep_stos_alternative", "ft"),
        "2:",
        ".pushsection __ex_table, \"aM\", @progbits, 12",
        ".balign 4",
        ".long 1b - .",
        ".long 2b - .",
        ".long 3", /* EX_TYPE_UACCESS */
        ".popsection",
        ft = const ALT_NOT(X86_FEATURE_FSRS),
        inout("rcx") size,
        inout("rdi") addr,
        in("rax") 0u64,
        options(att_syntax),
    );
    let _ = addr;

    clac();

    size
}

#[inline(always)]
pub unsafe fn clear_user(to: *mut core::ffi::c_void, n: core::ffi::c_ulong) -> core::ffi::c_ulong {
    if __access_ok(to, n) {
        return __clear_user(to, n);
    }
    n
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
