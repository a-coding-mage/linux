/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from arch/x86/include/asm/cmpxchg.h. */

/* Depends on: linux/compiler.h, asm/cpufeatures.h,
 * asm/alternative.h (Provides LOCK_PREFIX) */

/*
 * Non-existent functions to indicate usage errors at link time
 * (or compile-time if the compiler implements __compiletime_error().
 */
extern "C" {
    pub fn __xchg_wrong_size();
    pub fn __cmpxchg_wrong_size();
    pub fn __xadd_wrong_size();
    pub fn __add_wrong_size();
}

/*
 * Constants for operation sizes. On 32-bit, the 64-bit size it set to
 * -1 because sizeof will never return -1, thereby making those switch
 * case statements guaranteed dead code which the compiler will
 * eliminate, and allowing the "missing symbol in the default case" to
 * indicate a usage error.
 */
pub const __X86_CASE_B: isize = 1;
pub const __X86_CASE_W: isize = 2;
pub const __X86_CASE_L: isize = 4;
#[cfg(CONFIG_64BIT)]
pub const __X86_CASE_Q: isize = 8;
#[cfg(not(CONFIG_64BIT))]
pub const __X86_CASE_Q: isize = -1; /* sizeof will never return -1 */

/*
 * The C macros switch on sizeof(*(ptr)) and pick the b/w/l/q instruction.
 * In Rust the operand type selects the implementation: each size case below
 * is one impl of this trait, and an unsupported size is a type error instead
 * of a link error against __*_wrong_size().  `LOCK` is the lock prefix
 * argument ("" or "lock ").
 */
pub trait __x86_cmpxchg_type: Copy {
    unsafe fn __xchg_op_xchg<const LOCK: bool>(ptr: *mut Self, arg: Self) -> Self;
    unsafe fn __xchg_op_xadd<const LOCK: bool>(ptr: *mut Self, arg: Self) -> Self;
    unsafe fn __raw_cmpxchg<const LOCK: bool>(ptr: *mut Self, old: Self, new: Self) -> Self;
    unsafe fn __raw_try_cmpxchg<const LOCK: bool>(ptr: *mut Self, pold: *mut Self, new: Self) -> bool;
}

macro_rules! __x86_cmpxchg_case {
    ($ty:ty, $sfx:literal, $class:ident, $acc:tt, $m:literal) => {
        impl __x86_cmpxchg_type for $ty {
            #[inline(always)]
            unsafe fn __xchg_op_xchg<const LOCK: bool>(ptr: *mut Self, arg: Self) -> Self {
                let mut __ret = arg;
                if LOCK {
                    unsafe {
                        core::arch::asm!(
                            concat!("lock xchg", $sfx, " {ret", $m, "}, ({ptr})"),
                            ret = inout($class) __ret,
                            ptr = in(reg) ptr,
                            options(att_syntax, nostack)
                        )
                    }
                } else {
                    unsafe {
                        core::arch::asm!(
                            concat!("xchg", $sfx, " {ret", $m, "}, ({ptr})"),
                            ret = inout($class) __ret,
                            ptr = in(reg) ptr,
                            options(att_syntax, nostack)
                        )
                    }
                }
                __ret
            }

            #[inline(always)]
            unsafe fn __xchg_op_xadd<const LOCK: bool>(ptr: *mut Self, arg: Self) -> Self {
                let mut __ret = arg;
                if LOCK {
                    unsafe {
                        core::arch::asm!(
                            concat!("lock xadd", $sfx, " {ret", $m, "}, ({ptr})"),
                            ret = inout($class) __ret,
                            ptr = in(reg) ptr,
                            options(att_syntax, nostack)
                        )
                    }
                } else {
                    unsafe {
                        core::arch::asm!(
                            concat!("xadd", $sfx, " {ret", $m, "}, ({ptr})"),
                            ret = inout($class) __ret,
                            ptr = in(reg) ptr,
                            options(att_syntax, nostack)
                        )
                    }
                }
                __ret
            }

            #[inline(always)]
            unsafe fn __raw_cmpxchg<const LOCK: bool>(ptr: *mut Self, old: Self, new: Self) -> Self {
                let __ret: Self;
                if LOCK {
                    unsafe {
                        core::arch::asm!(
                            concat!("lock cmpxchg", $sfx, " {new", $m, "}, ({ptr})"),
                            new = in($class) new,
                            ptr = in(reg) ptr,
                            inout($acc) old => __ret,
                            options(att_syntax, nostack)
                        )
                    }
                } else {
                    unsafe {
                        core::arch::asm!(
                            concat!("cmpxchg", $sfx, " {new", $m, "}, ({ptr})"),
                            new = in($class) new,
                            ptr = in(reg) ptr,
                            inout($acc) old => __ret,
                            options(att_syntax, nostack)
                        )
                    }
                }
                __ret
            }

            #[inline(always)]
            unsafe fn __raw_try_cmpxchg<const LOCK: bool>(
                ptr: *mut Self,
                pold: *mut Self,
                new: Self,
            ) -> bool {
                let mut __old: Self = unsafe { *pold };
                let success: u8;
                if LOCK {
                    unsafe {
                        core::arch::asm!(
                            concat!("lock cmpxchg", $sfx, " {new", $m, "}, ({ptr})"),
                            "sete {success}",
                            new = in($class) new,
                            ptr = in(reg) ptr,
                            success = out(reg_byte) success,
                            inout($acc) __old,
                            options(att_syntax, nostack)
                        )
                    }
                } else {
                    unsafe {
                        core::arch::asm!(
                            concat!("cmpxchg", $sfx, " {new", $m, "}, ({ptr})"),
                            "sete {success}",
                            new = in($class) new,
                            ptr = in(reg) ptr,
                            success = out(reg_byte) success,
                            inout($acc) __old,
                            options(att_syntax, nostack)
                        )
                    }
                }
                if unlikely!(success == 0) {
                    unsafe { *pold = __old };
                }
                likely!(success != 0)
            }
        }
    };
}

/* __X86_CASE_B */
__x86_cmpxchg_case!(u8, "b", reg_byte, "al", "");
__x86_cmpxchg_case!(i8, "b", reg_byte, "al", "");
/* __X86_CASE_W */
__x86_cmpxchg_case!(u16, "w", reg, "ax", ":x");
__x86_cmpxchg_case!(i16, "w", reg, "ax", ":x");
/* __X86_CASE_L */
__x86_cmpxchg_case!(u32, "l", reg, "eax", ":e");
__x86_cmpxchg_case!(i32, "l", reg, "eax", ":e");
/* __X86_CASE_Q */
#[cfg(CONFIG_64BIT)]
__x86_cmpxchg_case!(u64, "q", reg, "rax", ":r");
#[cfg(CONFIG_64BIT)]
__x86_cmpxchg_case!(i64, "q", reg, "rax", ":r");
#[cfg(CONFIG_64BIT)]
__x86_cmpxchg_case!(usize, "q", reg, "rax", ":r");
#[cfg(CONFIG_64BIT)]
__x86_cmpxchg_case!(isize, "q", reg, "rax", ":r");

/* Pointers are exchanged as unsigned long. */
impl<T> __x86_cmpxchg_type for *mut T {
    #[inline(always)]
    unsafe fn __xchg_op_xchg<const LOCK: bool>(ptr: *mut Self, arg: Self) -> Self {
        unsafe { usize::__xchg_op_xchg::<LOCK>(ptr.cast(), arg as usize) as Self }
    }
    #[inline(always)]
    unsafe fn __xchg_op_xadd<const LOCK: bool>(ptr: *mut Self, arg: Self) -> Self {
        unsafe { usize::__xchg_op_xadd::<LOCK>(ptr.cast(), arg as usize) as Self }
    }
    #[inline(always)]
    unsafe fn __raw_cmpxchg<const LOCK: bool>(ptr: *mut Self, old: Self, new: Self) -> Self {
        unsafe { usize::__raw_cmpxchg::<LOCK>(ptr.cast(), old as usize, new as usize) as Self }
    }
    #[inline(always)]
    unsafe fn __raw_try_cmpxchg<const LOCK: bool>(ptr: *mut Self, pold: *mut Self, new: Self) -> bool {
        unsafe { usize::__raw_try_cmpxchg::<LOCK>(ptr.cast(), pold.cast(), new as usize) }
    }
}

impl<T> __x86_cmpxchg_type for *const T {
    #[inline(always)]
    unsafe fn __xchg_op_xchg<const LOCK: bool>(ptr: *mut Self, arg: Self) -> Self {
        unsafe { usize::__xchg_op_xchg::<LOCK>(ptr.cast(), arg as usize) as Self }
    }
    #[inline(always)]
    unsafe fn __xchg_op_xadd<const LOCK: bool>(ptr: *mut Self, arg: Self) -> Self {
        unsafe { usize::__xchg_op_xadd::<LOCK>(ptr.cast(), arg as usize) as Self }
    }
    #[inline(always)]
    unsafe fn __raw_cmpxchg<const LOCK: bool>(ptr: *mut Self, old: Self, new: Self) -> Self {
        unsafe { usize::__raw_cmpxchg::<LOCK>(ptr.cast(), old as usize, new as usize) as Self }
    }
    #[inline(always)]
    unsafe fn __raw_try_cmpxchg<const LOCK: bool>(ptr: *mut Self, pold: *mut Self, new: Self) -> bool {
        unsafe { usize::__raw_try_cmpxchg::<LOCK>(ptr.cast(), pold.cast(), new as usize) }
    }
}

/* C _Bool is one byte holding 0 or 1. */
impl __x86_cmpxchg_type for bool {
    #[inline(always)]
    unsafe fn __xchg_op_xchg<const LOCK: bool>(ptr: *mut Self, arg: Self) -> Self {
        unsafe { u8::__xchg_op_xchg::<LOCK>(ptr.cast(), arg as u8) != 0 }
    }
    #[inline(always)]
    unsafe fn __xchg_op_xadd<const LOCK: bool>(ptr: *mut Self, arg: Self) -> Self {
        unsafe { u8::__xchg_op_xadd::<LOCK>(ptr.cast(), arg as u8) != 0 }
    }
    #[inline(always)]
    unsafe fn __raw_cmpxchg<const LOCK: bool>(ptr: *mut Self, old: Self, new: Self) -> Self {
        unsafe { u8::__raw_cmpxchg::<LOCK>(ptr.cast(), old as u8, new as u8) != 0 }
    }
    #[inline(always)]
    unsafe fn __raw_try_cmpxchg<const LOCK: bool>(ptr: *mut Self, pold: *mut Self, new: Self) -> bool {
        unsafe { u8::__raw_try_cmpxchg::<LOCK>(ptr.cast(), pold.cast(), new as u8) }
    }
}

/*
 * An exchange-type operation, which takes a value and a pointer, and
 * returns the old value.
 */
#[macro_export]
macro_rules! __xchg_op {
    ($ptr:expr, $arg:expr, xchg, "") => {
        __x86_cmpxchg_type::__xchg_op_xchg::<false>($ptr, $arg)
    };
    ($ptr:expr, $arg:expr, xchg, "lock ") => {
        __x86_cmpxchg_type::__xchg_op_xchg::<true>($ptr, $arg)
    };
    ($ptr:expr, $arg:expr, xadd, "") => {
        __x86_cmpxchg_type::__xchg_op_xadd::<false>($ptr, $arg)
    };
    ($ptr:expr, $arg:expr, xadd, "lock ") => {
        __x86_cmpxchg_type::__xchg_op_xadd::<true>($ptr, $arg)
    };
    ($ptr:expr, $arg:expr, xadd, LOCK_PREFIX) => {
        __x86_cmpxchg_type::__xchg_op_xadd::<{ cfg!(CONFIG_SMP) }>($ptr, $arg)
    };
}

/*
 * Note: no "lock" prefix even on SMP: xchg always implies lock anyway.
 * Since this is generally used to protect other memory information, we
 * use "asm volatile" and "memory" clobbers to prevent gcc from moving
 * information around.
 */
#[macro_export]
macro_rules! arch_xchg {
    ($ptr:expr, $v:expr) => {
        __xchg_op!($ptr, $v, xchg, "")
    };
}

/*
 * Atomic compare and exchange.  Compare OLD with MEM, if identical,
 * store NEW in MEM.  Return the initial value in MEM.  Success is
 * indicated by comparing RETURN with OLD.
 */
#[macro_export]
macro_rules! __raw_cmpxchg {
    ($ptr:expr, $old:expr, $new:expr, $size:expr, "") => {
        __x86_cmpxchg_type::__raw_cmpxchg::<false>($ptr, $old, $new)
    };
    ($ptr:expr, $old:expr, $new:expr, $size:expr, "lock ") => {
        __x86_cmpxchg_type::__raw_cmpxchg::<true>($ptr, $old, $new)
    };
    ($ptr:expr, $old:expr, $new:expr, $size:expr, LOCK_PREFIX) => {
        __x86_cmpxchg_type::__raw_cmpxchg::<{ cfg!(CONFIG_SMP) }>($ptr, $old, $new)
    };
}

#[macro_export]
macro_rules! __cmpxchg {
    ($ptr:expr, $old:expr, $new:expr, $size:expr) => {
        __raw_cmpxchg!($ptr, $old, $new, $size, LOCK_PREFIX)
    };
}

#[macro_export]
macro_rules! __sync_cmpxchg {
    ($ptr:expr, $old:expr, $new:expr, $size:expr) => {
        __raw_cmpxchg!($ptr, $old, $new, $size, "lock ")
    };
}

#[macro_export]
macro_rules! __cmpxchg_local {
    ($ptr:expr, $old:expr, $new:expr, $size:expr) => {
        __raw_cmpxchg!($ptr, $old, $new, $size, "")
    };
}

/* Depends on: asm/cmpxchg_32.h (CONFIG_X86_32) or asm/cmpxchg_64.h */

#[macro_export]
macro_rules! arch_cmpxchg {
    ($ptr:expr, $old:expr, $new:expr) => {
        __cmpxchg!($ptr, $old, $new, ())
    };
}

#[macro_export]
macro_rules! arch_sync_cmpxchg {
    ($ptr:expr, $old:expr, $new:expr) => {
        __sync_cmpxchg!($ptr, $old, $new, ())
    };
}

#[macro_export]
macro_rules! arch_cmpxchg_local {
    ($ptr:expr, $old:expr, $new:expr) => {
        __cmpxchg_local!($ptr, $old, $new, ())
    };
}

#[macro_export]
macro_rules! __raw_try_cmpxchg {
    ($ptr:expr, $pold:expr, $new:expr, $size:expr, "") => {
        __x86_cmpxchg_type::__raw_try_cmpxchg::<false>($ptr, $pold, $new)
    };
    ($ptr:expr, $pold:expr, $new:expr, $size:expr, "lock ") => {
        __x86_cmpxchg_type::__raw_try_cmpxchg::<true>($ptr, $pold, $new)
    };
    ($ptr:expr, $pold:expr, $new:expr, $size:expr, LOCK_PREFIX) => {
        __x86_cmpxchg_type::__raw_try_cmpxchg::<{ cfg!(CONFIG_SMP) }>($ptr, $pold, $new)
    };
}

#[macro_export]
macro_rules! __try_cmpxchg {
    ($ptr:expr, $pold:expr, $new:expr, $size:expr) => {
        __raw_try_cmpxchg!($ptr, $pold, $new, $size, LOCK_PREFIX)
    };
}

#[macro_export]
macro_rules! __sync_try_cmpxchg {
    ($ptr:expr, $pold:expr, $new:expr, $size:expr) => {
        __raw_try_cmpxchg!($ptr, $pold, $new, $size, "lock ")
    };
}

#[macro_export]
macro_rules! __try_cmpxchg_local {
    ($ptr:expr, $pold:expr, $new:expr, $size:expr) => {
        __raw_try_cmpxchg!($ptr, $pold, $new, $size, "")
    };
}

#[macro_export]
macro_rules! arch_try_cmpxchg {
    ($ptr:expr, $pold:expr, $new:expr) => {
        __try_cmpxchg!($ptr, $pold, $new, ())
    };
}

#[macro_export]
macro_rules! arch_sync_try_cmpxchg {
    ($ptr:expr, $pold:expr, $new:expr) => {
        __sync_try_cmpxchg!($ptr, $pold, $new, ())
    };
}

#[macro_export]
macro_rules! arch_try_cmpxchg_local {
    ($ptr:expr, $pold:expr, $new:expr) => {
        __try_cmpxchg_local!($ptr, $pold, $new, ())
    };
}

/*
 * xadd() adds "inc" to "*ptr" and atomically returns the previous
 * value of "*ptr".
 *
 * xadd() is locked when multiple CPUs are online
 */
#[macro_export]
macro_rules! __xadd {
    ($ptr:expr, $inc:expr, $lock:tt) => {
        __xchg_op!($ptr, $inc, xadd, $lock)
    };
}

#[macro_export]
macro_rules! xadd {
    ($ptr:expr, $inc:expr) => {
        __xadd!($ptr, $inc, LOCK_PREFIX)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
