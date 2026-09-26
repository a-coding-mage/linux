/* SPDX-License-Identifier: GPL-2.0 */
/* User space memory access functions. C includes and header guards omitted. */

/* CONFIG_X86_32 / CONFIG_CC_HAS_ASM_GOTO_OUTPUT and related build conditions
 * are preserved below as cfg comments because their definitions are external. */

unsafe extern "C" {
    pub fn __get_user_1() -> i32;
    pub fn __get_user_2() -> i32;
    pub fn __get_user_4() -> i32;
    pub fn __get_user_8() -> i32;
    pub fn __get_user_nocheck_1() -> i32;
    pub fn __get_user_nocheck_2() -> i32;
    pub fn __get_user_nocheck_4() -> i32;
    pub fn __get_user_nocheck_8() -> i32;
    pub fn __get_user_bad() -> i32;
    pub fn __put_user_bad();
    pub fn __put_user_1();
    pub fn __put_user_2();
    pub fn __put_user_4();
    pub fn __put_user_8();
    pub fn __put_user_nocheck_1();
    pub fn __put_user_nocheck_2();
    pub fn __put_user_nocheck_4();
    pub fn __put_user_nocheck_8();
    pub fn copy_from_user_nmi(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    pub fn strncpy_from_user(dst: *mut i8, src: *const i8, count: isize) -> isize;
    pub fn strnlen_user(string: *const i8, n: isize) -> isize;
    pub fn __try_cmpxchg_user_wrong_size();
}

#[inline(always)]
pub unsafe fn __uaccess_begin() { stac(); }
#[inline(always)]
pub unsafe fn __uaccess_end() { clac(); }
#[inline(always)]
pub unsafe fn __uaccess_begin_nospec() { stac(); barrier_nospec(); }

/* External architecture/compiler primitives. */
unsafe extern "C" {
    fn stac();
    fn clac();
    fn barrier_nospec();
    fn might_fault();
    fn access_ok(ptr: *const core::ffi::c_void, len: usize) -> bool;
    fn smap_save() -> usize;
    fn smap_restore(value: usize);
}

#[repr(C)]
pub struct __large_struct { pub buf: [usize; 100] }

#[repr(C)]
pub struct movsl_mask { pub mask: i32 }

pub const ARCH_HAS_NONTEMPORAL_UACCESS: i32 = 1;

#[inline(always)]
pub unsafe fn user_access_begin(ptr: *const core::ffi::c_void, len: usize) -> bool {
    if !access_ok(ptr, len) { return false; }
    __uaccess_begin_nospec();
    true
}
#[inline(always)]
pub unsafe fn user_access_end() { __uaccess_end(); }
#[inline(always)]
pub unsafe fn user_access_save() -> usize { smap_save() }
#[inline(always)]
pub unsafe fn user_access_restore(value: usize) { smap_restore(value); }

/*
 * Integer types moved across the user boundary by a single access; this is
 * the Rust form of the C `switch (sizeof(*(ptr)))` dispatch and `__inttype()`.
 */
pub trait UserInt: Copy {
    const SIZE: usize;
    fn into_bits(self) -> u64;
    fn from_bits(bits: u64) -> Self;
}

macro_rules! impl_user_int {
    ($($t:ty),*) => {$(
        impl UserInt for $t {
            const SIZE: usize = core::mem::size_of::<$t>();
            #[inline(always)]
            fn into_bits(self) -> u64 {
                self as u64
            }
            #[inline(always)]
            fn from_bits(bits: u64) -> Self {
                bits as $t
            }
        }
    )*};
}
impl_user_int!(u8, u16, u32, u64, i8, i16, i32, i64, usize, isize);

/*
 * Strange magic calling convention: pointer in %rax, value returned in %rdx,
 * error in %eax (see arch/x86/lib/getuser.S).
 */
#[cfg(CONFIG_X86_64)]
macro_rules! call_get_user {
    ($sym:ident, $ptr:expr) => {{
        let ret: u64;
        let val: u64;
        // SAFETY: the out-of-line helper range-checks the pointer and
        // handles faults through its own exception table entries.
        unsafe {
            core::arch::asm!(
                "call {f}",
                f = sym $sym,
                inlateout("rax") $ptr as u64 => ret,
                lateout("rdx") val,
                options(att_syntax),
            );
        }
        (ret as core::ffi::c_int, val)
    }};
}

/// `do_get_user_call()`: returns the helper's error and the loaded value.
#[cfg(CONFIG_X86_64)]
#[inline(always)]
pub unsafe fn __do_get_user_call<T: UserInt>(ptr: *const T, check: bool) -> (core::ffi::c_int, T) {
    let (ret, val) = match (T::SIZE, check) {
        (1, true) => call_get_user!(__get_user_1, ptr),
        (2, true) => call_get_user!(__get_user_2, ptr),
        (4, true) => call_get_user!(__get_user_4, ptr),
        (8, true) => call_get_user!(__get_user_8, ptr),
        (1, false) => call_get_user!(__get_user_nocheck_1, ptr),
        (2, false) => call_get_user!(__get_user_nocheck_2, ptr),
        (4, false) => call_get_user!(__get_user_nocheck_4, ptr),
        (8, false) => call_get_user!(__get_user_nocheck_8, ptr),
        _ => (__get_user_bad(), 0),
    };
    (ret, T::from_bits(val))
}

/*
 * Pointer in %rcx, value in %rax, error returned in %ecx; the helper
 * clobbers %rbx (see arch/x86/lib/putuser.S), which Rust cannot name as an
 * operand, so it is preserved around the call.
 */
#[cfg(CONFIG_X86_64)]
macro_rules! call_put_user {
    ($sym:ident, $ptr:expr, $val:expr) => {{
        let ret: u64;
        // SAFETY: the out-of-line helper range-checks the pointer and
        // handles faults through its own exception table entries.
        unsafe {
            core::arch::asm!(
                "mov %rbx, {saved}",
                "call {f}",
                "mov {saved}, %rbx",
                f = sym $sym,
                saved = out(reg) _,
                inlateout("rcx") $ptr as u64 => ret,
                in("rax") $val,
                options(att_syntax),
            );
        }
        ret as core::ffi::c_int
    }};
}

/// `do_put_user_call()`: returns the helper's error.
#[cfg(CONFIG_X86_64)]
#[inline(always)]
pub unsafe fn __do_put_user_call<T: UserInt>(x: T, ptr: *mut T, check: bool) -> core::ffi::c_int {
    let val = x.into_bits();
    match (T::SIZE, check) {
        (1, true) => call_put_user!(__put_user_1, ptr, val),
        (2, true) => call_put_user!(__put_user_2, ptr, val),
        (4, true) => call_put_user!(__put_user_4, ptr, val),
        (8, true) => call_put_user!(__put_user_8, ptr, val),
        (1, false) => call_put_user!(__put_user_nocheck_1, ptr, val),
        (2, false) => call_put_user!(__put_user_nocheck_2, ptr, val),
        (4, false) => call_put_user!(__put_user_nocheck_4, ptr, val),
        (8, false) => call_put_user!(__put_user_nocheck_8, ptr, val),
        _ => {
            __put_user_bad();
            0
        }
    }
}

/*
 * get_user - Get a simple variable from user space.
 * Returns zero on success, or -EFAULT on error; on error @x is set to zero.
 */
#[macro_export]
macro_rules! get_user {
    ($x:expr, $ptr:expr) => {{
        might_fault();
        let (__ret_gu, __val_gu) = __do_get_user_call($ptr, true);
        $x = __val_gu;
        __ret_gu
    }};
}
#[macro_export]
macro_rules! __get_user {
    ($x:expr, $ptr:expr) => {{
        let (__ret_gu, __val_gu) = __do_get_user_call($ptr, false);
        $x = __val_gu;
        __ret_gu
    }};
}
/*
 * put_user - Write a simple value into user space.
 * Returns zero on success, or -EFAULT on error.
 */
#[macro_export]
macro_rules! put_user {
    ($x:expr, $ptr:expr) => {{
        might_fault();
        __do_put_user_call($x, $ptr, true)
    }};
}
#[macro_export]
macro_rules! __put_user {
    ($x:expr, $ptr:expr) => {
        __do_put_user_call($x, $ptr, false)
    };
}

/*
 * Exception-table entry types (asm/extable_fixup_types.h) used by the
 * inline user accessors below.
 */
pub const __UACCESS_EX_TYPE_UACCESS: u32 = 3;
/// `EX_TYPE_EFAULT_REG | EX_FLAG_CLEAR_AX` with %rdx (register 2) as the
/// error register: on fault %rdx = -EFAULT and %rax = 0.
pub const __UACCESS_EX_EFAULT_RDX_CLEAR_AX: u32 =
    17 | ((-14i32 as u32) << 16) | (1 << 12) | (2 << 8);

/*
 * `__get_user_size()` without asm-goto outputs: a faulting load leaves
 * -EFAULT in the error register and a zeroed value.
 */
#[cfg(CONFIG_X86_64)]
#[inline(always)]
pub unsafe fn __get_user_size<T: UserInt>(ptr: *const T) -> Result<T, core::ffi::c_int> {
    let err: u64;
    let val: u64;
    macro_rules! load {
        ($insn:literal, $dst:literal) => {
            core::arch::asm!(
                concat!("1: ", $insn, " ({p}), ", $dst),
                "2:",
                ".pushsection __ex_table, \"aM\", @progbits, 12",
                ".balign 4",
                ".long 1b - .",
                ".long 2b - .",
                ".long {ty}",
                ".popsection",
                p = in(reg) ptr,
                ty = const __UACCESS_EX_EFAULT_RDX_CLEAR_AX,
                inout("rdx") 0u64 => err,
                out("rax") val,
                options(att_syntax, nostack, readonly),
            )
        };
    }
    match T::SIZE {
        1 => load!("movzbl", "%eax"),
        2 => load!("movzwl", "%eax"),
        4 => load!("movl", "%eax"),
        8 => load!("movq", "%rax"),
        _ => return Err(__get_user_bad()),
    }
    if err != 0 {
        Err(err as core::ffi::c_int)
    } else {
        Ok(T::from_bits(val))
    }
}

/// `__put_user_size()`/`__put_user_goto()`: one `mov` with an
/// `EX_TYPE_UACCESS` fixup that branches to the fault path.
#[cfg(CONFIG_X86_64)]
#[inline(always)]
pub unsafe fn __put_user_size<T: UserInt>(x: T, ptr: *mut T) -> Result<(), ()> {
    let v = x.into_bits();
    macro_rules! store {
        ($insn:literal, $reg:literal) => {
            core::arch::asm!(
                concat!("1: ", $insn, " {v", $reg, "}, ({p})"),
                ".pushsection __ex_table, \"aM\", @progbits, 12",
                ".balign 4",
                ".long 1b - .",
                ".long {fault} - .",
                ".long {ty}",
                ".popsection",
                v = in(reg) v,
                p = in(reg) ptr,
                ty = const __UACCESS_EX_TYPE_UACCESS,
                fault = label { return Err(()); },
                options(att_syntax, nostack),
            )
        };
    }
    match T::SIZE {
        1 => store!("movb", ":l"),
        2 => store!("movw", ":x"),
        4 => store!("movl", ":e"),
        8 => store!("movq", ""),
        _ => __put_user_bad(),
    }
    Ok(())
}

/*
 * The unsafe_{get,put}_user() accessors require user_access_begin(); on a
 * fault they break out to @label, as the C versions `goto` it.
 */
#[macro_export]
macro_rules! arch_unsafe_get_user {
    ($x:expr, $ptr:expr, $label:lifetime) => {
        match __get_user_size($ptr) {
            Ok(__gu_val) => $x = __gu_val,
            Err(_) => break $label,
        }
    };
}
#[macro_export]
macro_rules! arch_unsafe_put_user {
    ($x:expr, $ptr:expr, $label:lifetime) => {
        if __put_user_size($x, $ptr).is_err() {
            break $label;
        }
    };
}
#[macro_export]
macro_rules! unsafe_get_user {
    ($x:expr, $ptr:expr, $label:lifetime) => {
        arch_unsafe_get_user!($x, $ptr, $label)
    };
}
#[macro_export]
macro_rules! unsafe_put_user {
    ($x:expr, $ptr:expr, $label:lifetime) => {
        arch_unsafe_put_user!($x, $ptr, $label)
    };
}

#[macro_export]
macro_rules! unsafe_copy_loop {
    ($dst:ident, $src:ident, $len:ident, $ty:ty, $label:lifetime) => {
        while $len >= core::mem::size_of::<$ty>() {
            unsafe_put_user!(
                core::ptr::read_unaligned($src as *const $ty),
                $dst as *mut $ty,
                $label
            );
            $dst = $dst.wrapping_add(core::mem::size_of::<$ty>());
            $src = $src.wrapping_add(core::mem::size_of::<$ty>());
            $len -= core::mem::size_of::<$ty>();
        }
    };
}

#[macro_export]
macro_rules! unsafe_copy_to_user {
    ($dst:expr, $src:expr, $len:expr, $label:lifetime) => {{
        let mut __ucu_dst: *mut u8 = ($dst).cast();
        let mut __ucu_src: *const u8 = ($src).cast();
        let mut __ucu_len: usize = $len;
        unsafe_copy_loop!(__ucu_dst, __ucu_src, __ucu_len, u64, $label);
        unsafe_copy_loop!(__ucu_dst, __ucu_src, __ucu_len, u32, $label);
        unsafe_copy_loop!(__ucu_dst, __ucu_src, __ucu_len, u16, $label);
        unsafe_copy_loop!(__ucu_dst, __ucu_src, __ucu_len, u8, $label);
    }};
}

// CONFIG_X86_32: the 32-bit forms (u64 in %edx:%eax, `__get_user_8` returning
// %ecx:%edx) are not translated yet; only CONFIG_X86_64 accessors exist here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
