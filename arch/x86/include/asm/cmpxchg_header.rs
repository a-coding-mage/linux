/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of the x86 cmpxchg header. */

extern "C" {
    pub fn __xchg_wrong_size() -> !;
    pub fn __cmpxchg_wrong_size() -> !;
    pub fn __xadd_wrong_size() -> !;
    pub fn __add_wrong_size() -> !;
}

pub const __X86_CASE_B: usize = 1;
pub const __X86_CASE_W: usize = 2;
pub const __X86_CASE_L: usize = 4;
#[cfg(target_pointer_width = "64")]
pub const __X86_CASE_Q: usize = 8;
#[cfg(not(target_pointer_width = "64"))]
pub const __X86_CASE_Q: isize = -1; /* sizeof will never return -1 */

/* LOCK_PREFIX and the architecture-specific cmpxchg implementations are supplied externally. */

#[macro_export]
macro_rules! __xchg_op {
    ($ptr:expr, $arg:expr, xchg, $lock:expr) => {{
        let mut __ret = $arg;
        unsafe {
            match core::mem::size_of_val(&*$ptr) {
                1 => core::arch::asm!(concat!($lock, "xchgb %b0, [{1}]") , inout(reg_byte) __ret, in(reg) $ptr, options(nostack, preserves_flags)),
                2 => core::arch::asm!(concat!($lock, "xchgw %w0, [{1}]") , inout(reg) __ret, in(reg) $ptr, options(nostack, preserves_flags)),
                4 => core::arch::asm!(concat!($lock, "xchgl %0, [{1}]") , inout(reg) __ret, in(reg) $ptr, options(nostack, preserves_flags)),
                8 => core::arch::asm!(concat!($lock, "xchgq %0, [{1}]") , inout(reg) __ret, in(reg) $ptr, options(nostack, preserves_flags)),
                _ => $crate::__xchg_wrong_size(),
            }
        }
        __ret
    }};
    ($ptr:expr, $arg:expr, xadd, $lock:expr) => {{
        let mut __ret = $arg;
        unsafe {
            match core::mem::size_of_val(&*$ptr) {
                1 => core::arch::asm!(concat!($lock, "xaddb %b0, [{1}]") , inout(reg_byte) __ret, in(reg) $ptr, options(nostack, preserves_flags)),
                2 => core::arch::asm!(concat!($lock, "xaddw %w0, [{1}]") , inout(reg) __ret, in(reg) $ptr, options(nostack, preserves_flags)),
                4 => core::arch::asm!(concat!($lock, "xaddl %0, [{1}]") , inout(reg) __ret, in(reg) $ptr, options(nostack, preserves_flags)),
                8 => core::arch::asm!(concat!($lock, "xaddq %0, [{1}]") , inout(reg) __ret, in(reg) $ptr, options(nostack, preserves_flags)),
                _ => $crate::__xadd_wrong_size(),
            }
        }
        __ret
    }};
}

#[macro_export]
macro_rules! arch_xchg { ($ptr:expr, $v:expr) => { $crate::__xchg_op!($ptr, $v, xchg, "") }; }

/* Atomic compare-and-exchange; the lock argument preserves the C macro's ordering intent. */
#[macro_export]
macro_rules! __raw_cmpxchg {
    ($ptr:expr, $old:expr, $new:expr, $size:expr, $lock:expr) => {{
        let mut __ret = $old;
        let __new = $new;
        unsafe {
            match $size {
                1 => core::arch::asm!(concat!($lock, "cmpxchgb {new}, [{ptr}]") , ptr = in(reg) $ptr, new = in(reg_byte) __new, inout("rax") __ret, options(nostack)),
                2 => core::arch::asm!(concat!($lock, "cmpxchgw {new}, [{ptr}]") , ptr = in(reg) $ptr, new = in(reg) __new, inout("rax") __ret, options(nostack)),
                4 => core::arch::asm!(concat!($lock, "cmpxchgl {new}, [{ptr}]") , ptr = in(reg) $ptr, new = in(reg) __new, inout("rax") __ret, options(nostack)),
                8 => core::arch::asm!(concat!($lock, "cmpxchgq {new}, [{ptr}]") , ptr = in(reg) $ptr, new = in(reg) __new, inout("rax") __ret, options(nostack)),
                _ => $crate::__cmpxchg_wrong_size(),
            }
        }
        __ret
    }};
}

#[macro_export]
macro_rules! __cmpxchg { ($ptr:expr, $old:expr, $new:expr, $size:expr) => { $crate::__raw_cmpxchg!($ptr, $old, $new, $size, "lock ") }; }
#[macro_export]
macro_rules! __sync_cmpxchg { ($ptr:expr, $old:expr, $new:expr, $size:expr) => { $crate::__raw_cmpxchg!($ptr, $old, $new, $size, "lock ") }; }
#[macro_export]
macro_rules! __cmpxchg_local { ($ptr:expr, $old:expr, $new:expr, $size:expr) => { $crate::__raw_cmpxchg!($ptr, $old, $new, $size, "") }; }
#[macro_export]
macro_rules! arch_cmpxchg { ($ptr:expr, $old:expr, $new:expr) => { $crate::__cmpxchg!($ptr, $old, $new, core::mem::size_of_val(&*$ptr)) }; }
#[macro_export]
macro_rules! arch_sync_cmpxchg { ($ptr:expr, $old:expr, $new:expr) => { $crate::__sync_cmpxchg!($ptr, $old, $new, core::mem::size_of_val(&*$ptr)) }; }
#[macro_export]
macro_rules! arch_cmpxchg_local { ($ptr:expr, $old:expr, $new:expr) => { $crate::__cmpxchg_local!($ptr, $old, $new, core::mem::size_of_val(&*$ptr)) }; }

/* try_cmpxchg variants preserve the C API's in-place old-value update. */
#[macro_export]
macro_rules! __raw_try_cmpxchg {
    ($ptr:expr, $pold:expr, $new:expr, $size:expr, $lock:expr) => {{
        let mut __old = unsafe { *$pold };
        let __new = $new;
        let __success = $crate::__raw_cmpxchg!($ptr, __old, __new, $size, $lock) == __old;
        if !__success { unsafe { *$pold = __old; } }
        __success
    }};
}
#[macro_export]
macro_rules! __try_cmpxchg { ($ptr:expr, $pold:expr, $new:expr, $size:expr) => { $crate::__raw_try_cmpxchg!($ptr, $pold, $new, $size, "lock ") }; }
#[macro_export]
macro_rules! __sync_try_cmpxchg { ($ptr:expr, $pold:expr, $new:expr, $size:expr) => { $crate::__raw_try_cmpxchg!($ptr, $pold, $new, $size, "lock ") }; }
#[macro_export]
macro_rules! __try_cmpxchg_local { ($ptr:expr, $pold:expr, $new:expr, $size:expr) => { $crate::__raw_try_cmpxchg!($ptr, $pold, $new, $size, "") }; }
#[macro_export]
macro_rules! arch_try_cmpxchg { ($ptr:expr, $pold:expr, $new:expr) => { $crate::__try_cmpxchg!($ptr, $pold, $new, core::mem::size_of_val(&*$ptr)) }; }
#[macro_export]
macro_rules! arch_sync_try_cmpxchg { ($ptr:expr, $pold:expr, $new:expr) => { $crate::__sync_try_cmpxchg!($ptr, $pold, $new, core::mem::size_of_val(&*$ptr)) }; }
#[macro_export]
macro_rules! arch_try_cmpxchg_local { ($ptr:expr, $pold:expr, $new:expr) => { $crate::__try_cmpxchg_local!($ptr, $pold, $new, core::mem::size_of_val(&*$ptr)) }; }

#[macro_export]
macro_rules! __xadd { ($ptr:expr, $inc:expr, $lock:expr) => { $crate::__xchg_op!($ptr, $inc, xadd, $lock) }; }
#[macro_export]
macro_rules! xadd { ($ptr:expr, $inc:expr) => { $crate::__xadd!($ptr, $inc, "lock ") }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
