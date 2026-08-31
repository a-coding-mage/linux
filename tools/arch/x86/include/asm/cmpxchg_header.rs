/* SPDX-License-Identifier: GPL-2.0 */

// Original C header dependency: <linux/compiler.h>

/*
 * Non-existant functions to indicate usage errors at link time
 * (or compile-time if the compiler implements __compiletime_error().
 */
extern "C" {
    #[link_name = "__cmpxchg_wrong_size"]
    pub fn __cmpxchg_wrong_size() -> !;
}

/*
 * Constants for operation sizes. On 32-bit, the 64-bit size it set to
 * -1 because sizeof will never return -1, thereby making those switch
 * case statements guaranteeed dead code which the compiler will
 * eliminate, and allowing the "missing symbol in the default case" to
 * indicate a usage error.
 */
pub const __X86_CASE_B: isize = 1;
pub const __X86_CASE_W: isize = 2;
pub const __X86_CASE_L: isize = 4;

// C preprocessor condition preserved: #ifdef __x86_64__
#[cfg(target_arch = "x86_64")]
pub const __X86_CASE_Q: isize = 8;
#[cfg(not(target_arch = "x86_64"))]
pub const __X86_CASE_Q: isize = -1; /* sizeof will never return -1 */

/*
 * Atomic compare and exchange.  Compare OLD with MEM, if identical,
 * store NEW in MEM.  Return the initial value in MEM.  Success is
 * indicated by comparing RETURN with OLD.
 */
#[macro_export]
macro_rules! __raw_cmpxchg {
    ($ptr:expr, $old:expr, $new:expr, $size:expr, $lock:expr) => {{
        let mut __ret;
        let __old = $old;
        let __new = $new;

        match $size as isize {
            $crate::__X86_CASE_B => {
                let __ptr = $ptr as *mut u8;
                __ret = __old;
                unsafe {
                    core::arch::asm!(
                        concat!($lock, "cmpxchgb {new}, [{ptr}]"),
                        ptr = in(reg) __ptr,
                        new = in(reg_byte) __new as u8,
                        inout("al") __ret,
                        options(nostack, preserves_flags),
                    );
                }
            }
            $crate::__X86_CASE_W => {
                let __ptr = $ptr as *mut u16;
                __ret = __old;
                unsafe {
                    core::arch::asm!(
                        concat!($lock, "cmpxchgw {new:x}, [{ptr}]"),
                        ptr = in(reg) __ptr,
                        new = in(reg) __new as u16,
                        inout("ax") __ret,
                        options(nostack, preserves_flags),
                    );
                }
            }
            $crate::__X86_CASE_L => {
                let __ptr = $ptr as *mut u32;
                __ret = __old;
                unsafe {
                    core::arch::asm!(
                        concat!($lock, "cmpxchgl {new:e}, [{ptr}]"),
                        ptr = in(reg) __ptr,
                        new = in(reg) __new as u32,
                        inout("eax") __ret,
                        options(nostack, preserves_flags),
                    );
                }
            }
            $crate::__X86_CASE_Q => {
                let __ptr = $ptr as *mut u64;
                __ret = __old;
                unsafe {
                    core::arch::asm!(
                        concat!($lock, "cmpxchgq {new}, [{ptr}]"),
                        ptr = in(reg) __ptr,
                        new = in(reg) __new as u64,
                        inout("rax") __ret,
                        options(nostack, preserves_flags),
                    );
                }
            }
            _ => unsafe {
                $crate::__cmpxchg_wrong_size();
            },
        }

        __ret
    }};
}

#[macro_export]
macro_rules! __cmpxchg {
    ($ptr:expr, $old:expr, $new:expr, $size:expr) => {
        $crate::__raw_cmpxchg!(($ptr), ($old), ($new), ($size), LOCK_PREFIX)
    };
}

#[macro_export]
macro_rules! cmpxchg {
    ($ptr:expr, $old:expr, $new:expr) => {
        $crate::__cmpxchg!(($ptr), ($old), ($new), core::mem::size_of_val(&*($ptr)))
    };
}
