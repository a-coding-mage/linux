/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from arch/x86/include/asm/cmpxchg_64.h. */

/* BUILD_BUG_ON(sizeof(*(ptr)) != 8), checked on the pointee type. */
#[inline(always)]
pub const fn __cmpxchg64_check_size<T>(_ptr: *const T) {
    const { assert!(core::mem::size_of::<T>() == 8, "BUILD_BUG_ON failed: sizeof(*(ptr)) != 8") }
}

#[macro_export]
macro_rules! arch_cmpxchg64 {
    ($ptr:expr, $o:expr, $n:expr) => {{
        let __ptr = $ptr;
        __cmpxchg64_check_size(__ptr);
        arch_cmpxchg!(__ptr, $o, $n)
    }};
}

#[macro_export]
macro_rules! arch_cmpxchg64_local {
    ($ptr:expr, $o:expr, $n:expr) => {{
        let __ptr = $ptr;
        __cmpxchg64_check_size(__ptr);
        arch_cmpxchg_local!(__ptr, $o, $n)
    }};
}

#[macro_export]
macro_rules! arch_try_cmpxchg64 {
    ($ptr:expr, $po:expr, $n:expr) => {{
        let __ptr = $ptr;
        __cmpxchg64_check_size(__ptr);
        arch_try_cmpxchg!(__ptr, $po, $n)
    }};
}

#[macro_export]
macro_rules! arch_try_cmpxchg64_local {
    ($ptr:expr, $po:expr, $n:expr) => {{
        let __ptr = $ptr;
        __cmpxchg64_check_size(__ptr);
        arch_try_cmpxchg_local!(__ptr, $po, $n)
    }};
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct __u128_halves_s {
    pub low: u64,
    pub high: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union __u128_halves {
    pub full: u128,
    pub s: __u128_halves_s,
}

/*
 * cmpxchg16b takes the new value in %rbx:%rcx.  LLVM reserves %rbx, so the
 * low half travels in a scratch register and is swapped into %rbx around the
 * instruction.
 */
#[inline(always)]
pub unsafe fn __arch_cmpxchg128<const LOCK: bool>(ptr: *mut u128, old: u128, new: u128) -> u128 {
    let o = __u128_halves { full: old };
    let n = __u128_halves { full: new };
    let (mut low, mut high) = unsafe { (o.s.low, o.s.high) };

    unsafe {
        if LOCK {
            core::arch::asm!(
                "xchgq {nlow}, %rbx",
                "lock cmpxchg16b ({ptr})",
                "movq {nlow}, %rbx",
                ptr = in(reg) ptr,
                nlow = inout(reg) n.s.low => _,
                in("rcx") n.s.high,
                inout("rax") low,
                inout("rdx") high,
                options(att_syntax, nostack)
            );
        } else {
            core::arch::asm!(
                "xchgq {nlow}, %rbx",
                "cmpxchg16b ({ptr})",
                "movq {nlow}, %rbx",
                ptr = in(reg) ptr,
                nlow = inout(reg) n.s.low => _,
                in("rcx") n.s.high,
                inout("rax") low,
                inout("rdx") high,
                options(att_syntax, nostack)
            );
        }
    }

    unsafe { __u128_halves { s: __u128_halves_s { low, high } }.full }
}

#[inline(always)]
pub unsafe fn arch_cmpxchg128(ptr: *mut u128, old: u128, new: u128) -> u128 {
    unsafe { __arch_cmpxchg128::<{ cfg!(CONFIG_SMP) }>(ptr, old, new) }
}
#[macro_export]
macro_rules! arch_cmpxchg128 {
    ($($args:tt)*) => {
        arch_cmpxchg128($($args)*)
    };
}

#[inline(always)]
pub unsafe fn arch_cmpxchg128_local(ptr: *mut u128, old: u128, new: u128) -> u128 {
    unsafe { __arch_cmpxchg128::<false>(ptr, old, new) }
}
#[macro_export]
macro_rules! arch_cmpxchg128_local {
    ($($args:tt)*) => {
        arch_cmpxchg128_local($($args)*)
    };
}

#[inline(always)]
pub unsafe fn __arch_try_cmpxchg128<const LOCK: bool>(ptr: *mut u128, oldp: *mut u128, new: u128) -> bool {
    let o = __u128_halves { full: unsafe { *oldp } };
    let n = __u128_halves { full: new };
    let (mut low, mut high) = unsafe { (o.s.low, o.s.high) };
    let ret: u8;

    unsafe {
        if LOCK {
            core::arch::asm!(
                "xchgq {nlow}, %rbx",
                "lock cmpxchg16b ({ptr})",
                "sete {ret}",
                "movq {nlow}, %rbx",
                ptr = in(reg) ptr,
                nlow = inout(reg) n.s.low => _,
                ret = out(reg_byte) ret,
                in("rcx") n.s.high,
                inout("rax") low,
                inout("rdx") high,
                options(att_syntax, nostack)
            );
        } else {
            core::arch::asm!(
                "xchgq {nlow}, %rbx",
                "cmpxchg16b ({ptr})",
                "sete {ret}",
                "movq {nlow}, %rbx",
                ptr = in(reg) ptr,
                nlow = inout(reg) n.s.low => _,
                ret = out(reg_byte) ret,
                in("rcx") n.s.high,
                inout("rax") low,
                inout("rdx") high,
                options(att_syntax, nostack)
            );
        }
    }

    if unlikely!(ret == 0) {
        unsafe { *oldp = __u128_halves { s: __u128_halves_s { low, high } }.full };
    }

    likely!(ret != 0)
}

#[inline(always)]
pub unsafe fn arch_try_cmpxchg128(ptr: *mut u128, oldp: *mut u128, new: u128) -> bool {
    unsafe { __arch_try_cmpxchg128::<{ cfg!(CONFIG_SMP) }>(ptr, oldp, new) }
}
#[macro_export]
macro_rules! arch_try_cmpxchg128 {
    ($($args:tt)*) => {
        arch_try_cmpxchg128($($args)*)
    };
}

#[inline(always)]
pub unsafe fn arch_try_cmpxchg128_local(ptr: *mut u128, oldp: *mut u128, new: u128) -> bool {
    unsafe { __arch_try_cmpxchg128::<false>(ptr, oldp, new) }
}
#[macro_export]
macro_rules! arch_try_cmpxchg128_local {
    ($($args:tt)*) => {
        arch_try_cmpxchg128_local($($args)*)
    };
}

#[macro_export]
macro_rules! system_has_cmpxchg128 {
    () => {
        boot_cpu_has!(X86_FEATURE_CX16)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
