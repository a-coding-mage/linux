/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Rust translation of include/linux/compiler.h.
 *
 * C include directives and header guards are intentionally not executable Rust.
 * This header originally depends on linux/compiler_types.h and linux/types.h.
 */

/*
 * __compiletime_error(message)
 *
 * C attribute-only macro. Rust has no file-local equivalent for attaching a
 * compiler diagnostic attribute to an arbitrary extern declaration.
 */

/*
 * __compiletime_assert(condition, msg, prefix, suffix)
 * _compiletime_assert(condition, msg, prefix, suffix)
 * compiletime_assert(condition, msg)
 *
 * The C implementation creates a uniquely named extern function annotated with
 * __compiletime_error and calls it when an optimized compile-time condition is
 * false. Rust cannot paste identifiers or attach that C attribute locally.
 */
#[macro_export]
macro_rules! compiletime_assert {
    ($condition:expr, $msg:expr $(,)?) => {
        const _: () = assert!($condition, $msg);
    };
}

/* Optimization barrier */
/* The "volatile" is due to gcc bugs */
#[inline(always)]
pub unsafe fn barrier() {
    core::arch::asm!("", options(nostack, preserves_flags));
}

/*
 * C declaration/definition attributes translated as marker macros. They carry
 * no executable Rust behavior at this header level:
 * __always_inline, __always_unused, __noreturn, noinline, __nocf_check,
 * __naked, __user, __rcu, __read_mostly, __attribute_const__,
 * __maybe_unused, __used, __packed, __force, __iomem, __weak.
 */

#[macro_export]
macro_rules! unreachable {
    () => {
        core::hint::unreachable_unchecked()
    };
}

/* Are two types/vars the same type (ignoring qualifiers)? */
/*
 * __same_type(a, b) used GCC typeof and __builtin_types_compatible_p. There is
 * no direct expression-level Rust equivalent in this isolated header.
 */

/*
 * This returns a constant expression while determining if an argument is
 * a constant expression, most importantly without evaluating the argument.
 * Glory to Martin Uecker <Martin.Uecker@med.uni-goettingen.de>
 */
/*
 * __is_constexpr(x) depends on C sizeof, pointer conditional typing, and typeof
 * behavior. It is preserved here as intent only.
 */

/*
 * Similar to statically_true() but produces a constant expression
 *
 * To be used in conjunction with macros, such as BUILD_BUG_ON_ZERO(),
 * which require their input to be a constant expression and for which
 * statically_true() would otherwise fail.
 *
 * This is a trade-off: const_true() requires all its operands to be
 * compile time constants. Else, it would always returns false even on
 * the most trivial cases like:
 *
 *   true || non_const_var
 *
 * On the opposite, statically_true() is able to fold more complex
 * tautologies and will return true on expressions such as:
 *
 *   !(non_const_var * 8 % 4)
 *
 * For the general case, statically_true() is better.
 */
#[macro_export]
macro_rules! const_true {
    ($x:expr) => {
        $x
    };
}

#[macro_export]
macro_rules! likely {
    ($x:expr) => {
        $x
    };
}

#[macro_export]
macro_rules! unlikely {
    ($x:expr) => {
        $x
    };
}

/*
 * Following functions are taken from kernel sources and
 * break aliasing rules in their original form.
 *
 * While kernel is compiled with -fno-strict-aliasing,
 * perf uses -Wstrict-aliasing=3 which makes build fail
 * under gcc 4.4.
 *
 * Using extra __may_alias__ type to allow aliasing
 * in this case.
 */
pub type __u8_alias_t = __u8;
pub type __u16_alias_t = __u16;
pub type __u32_alias_t = __u32;
pub type __u64_alias_t = __u64;

#[inline(always)]
pub unsafe fn __read_once_size(p: *const core::ffi::c_void, res: *mut core::ffi::c_void, size: i32) {
    match size {
        1 => {
            *(res as *mut __u8_alias_t) = core::ptr::read_volatile(p as *const __u8_alias_t);
        }
        2 => {
            *(res as *mut __u16_alias_t) = core::ptr::read_volatile(p as *const __u16_alias_t);
        }
        4 => {
            *(res as *mut __u32_alias_t) = core::ptr::read_volatile(p as *const __u32_alias_t);
        }
        8 => {
            *(res as *mut __u64_alias_t) = core::ptr::read_volatile(p as *const __u64_alias_t);
        }
        _ => {
            barrier();
            core::ptr::copy_nonoverlapping(p as *const u8, res as *mut u8, size as usize);
            barrier();
        }
    }
}

#[inline(always)]
pub unsafe fn __write_once_size(p: *mut core::ffi::c_void, res: *mut core::ffi::c_void, size: i32) {
    match size {
        1 => {
            core::ptr::write_volatile(p as *mut __u8_alias_t, *(res as *const __u8_alias_t));
        }
        2 => {
            core::ptr::write_volatile(p as *mut __u16_alias_t, *(res as *const __u16_alias_t));
        }
        4 => {
            core::ptr::write_volatile(p as *mut __u32_alias_t, *(res as *const __u32_alias_t));
        }
        8 => {
            core::ptr::write_volatile(p as *mut __u64_alias_t, *(res as *const __u64_alias_t));
        }
        _ => {
            barrier();
            core::ptr::copy_nonoverlapping(res as *const u8, p as *mut u8, size as usize);
            barrier();
        }
    }
}

/*
 * Prevent the compiler from merging or refetching reads or writes. The
 * compiler is also forbidden from reordering successive instances of
 * READ_ONCE and WRITE_ONCE, but only when the compiler is aware of some
 * particular ordering. One way to make the compiler aware of ordering is to
 * put the two invocations of READ_ONCE or WRITE_ONCE in different C
 * statements.
 *
 * These two macros will also work on aggregate data types like structs or
 * unions. If the size of the accessed data type exceeds the word size of
 * the machine (e.g., 32 bits or 64 bits) READ_ONCE() and WRITE_ONCE() will
 * fall back to memcpy and print a compile-time warning.
 *
 * Their two major use cases are: (1) Mediating communication between
 * process-level code and irq/NMI handlers, all running on the same CPU,
 * and (2) Ensuring that the compiler does not fold, spindle, or otherwise
 * mutilate accesses that either do not require ordering or that interact
 * with an explicit memory barrier or atomic instruction that provides the
 * required ordering.
 */
#[inline(always)]
pub unsafe fn READ_ONCE<T: Copy>(x: *const T) -> T {
    let mut __u = core::mem::MaybeUninit::<T>::zeroed();
    __read_once_size(
        x as *const core::ffi::c_void,
        __u.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of::<T>() as i32,
    );
    __u.assume_init()
}

#[inline(always)]
pub unsafe fn WRITE_ONCE<T: Copy>(x: *mut T, val: T) -> T {
    let mut __u = val;
    __write_once_size(
        x as *mut core::ffi::c_void,
        &mut __u as *mut T as *mut core::ffi::c_void,
        core::mem::size_of::<T>() as i32,
    );
    __u
}

/* Indirect macros required for expanded argument pasting, eg. __LINE__. */
/*
 * ___PASTE(a, b) and __PASTE(a, b) are C preprocessor token-pasting macros.
 * Rust macro_rules! cannot directly paste arbitrary identifiers without an
 * external helper.
 */

/*
 * OPTIMIZER_HIDE_VAR(var)
 *
 * Make the optimizer believe the variable can be manipulated arbitrarily.
 */
#[macro_export]
macro_rules! OPTIMIZER_HIDE_VAR {
    ($var:expr) => {
        core::arch::asm!("", inout(reg) $var, options(nostack, preserves_flags))
    };
}

/*
 * __BUILD_BUG_ON_ZERO_MSG(e, msg, ...)
 *
 * The C macro uses a negative bitfield width under clang or _Static_assert
 * otherwise, and evaluates to integer zero when the expression is false.
 */
#[macro_export]
macro_rules! __BUILD_BUG_ON_ZERO_MSG {
    ($e:expr, $msg:expr $(, $args:tt)*) => {{
        const _: () = assert!(!$e, $msg);
        0i32
    }};
}
