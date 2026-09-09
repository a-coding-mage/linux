/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2020-2022 Loongson Technology Corporation Limited */

// The original header includes <asm/loongarch.h>; its symbols are supplied by
// the surrounding kernel translation.

/*
 * The "address" (in fact, offset from $r21) of a per-CPU variable is close to
 * the loading address of main kernel image, but far from where the modules are
 * loaded.  Preserve the explicit-relocation/model conditional here.
 */
#[cfg(all(feature = "MODULE", feature = "CONFIG_AS_HAS_EXPLICIT_RELOCS", feature = "CONFIG_64BIT"))]
// PER_CPU_ATTRIBUTES is the C compiler model("extreme") attribute.
pub const PER_CPU_ATTRIBUTES: &str = "model(\"extreme\")";

/* Use r21 for fast access. */
#[no_mangle]
pub static mut __my_cpu_offset: usize = 0;

extern "C" {
    fn csr_write(value: usize, reg: usize);
    fn BUILD_BUG() -> !;
    fn preempt_disable_notrace();
    fn preempt_enable_notrace();
    fn raw_cpu_ptr<T>(ptr: *mut T) -> *mut T;
    fn cmpxchg_local<T: Copy>(ptr: *mut T, old: T, new: T) -> T;
    fn __arch_xchg<T: Copy>(ptr: *mut T, value: T) -> T;
}

// Supplied by asm/loongarch.h in the original source.
extern "C" {
    static PERCPU_BASE_KS: usize;
}

#[inline]
pub unsafe fn set_my_cpu_offset(off: usize) {
    __my_cpu_offset = off;
    csr_write(off, PERCPU_BASE_KS);
}

#[inline]
pub unsafe fn __my_cpu_offset_value() -> usize {
    // The C expression uses a volatile register constraint to keep r21 live.
    core::ptr::read_volatile(&__my_cpu_offset)
}

#[cfg(feature = "CONFIG_CPU_HAS_AMO")]
#[inline(always)]
pub unsafe fn __percpu_add(ptr: *mut u8, val: usize, size: i32) -> usize {
    match size {
        4 => {
            let p = ptr as *mut u32;
            let old = core::ptr::read_volatile(p);
            core::ptr::write_volatile(p, old.wrapping_add(val as u32));
            old as usize
        }
        8 => {
            let p = ptr as *mut u64;
            let old = core::ptr::read_volatile(p);
            core::ptr::write_volatile(p, old.wrapping_add(val as u64));
            old as usize
        }
        _ => { BUILD_BUG(); }
    }
}

#[cfg(feature = "CONFIG_CPU_HAS_AMO")]
#[inline(always)]
pub unsafe fn __percpu_and(ptr: *mut u8, val: usize, size: i32) -> usize {
    match size {
        4 => { let p = ptr as *mut u32; let old = core::ptr::read_volatile(p); core::ptr::write_volatile(p, old & val as u32); old as usize }
        8 => { let p = ptr as *mut u64; let old = core::ptr::read_volatile(p); core::ptr::write_volatile(p, old & val as u64); old as usize }
        _ => { BUILD_BUG(); }
    }
}

#[cfg(feature = "CONFIG_CPU_HAS_AMO")]
#[inline(always)]
pub unsafe fn __percpu_or(ptr: *mut u8, val: usize, size: i32) -> usize {
    match size {
        4 => { let p = ptr as *mut u32; let old = core::ptr::read_volatile(p); core::ptr::write_volatile(p, old | val as u32); old as usize }
        8 => { let p = ptr as *mut u64; let old = core::ptr::read_volatile(p); core::ptr::write_volatile(p, old | val as u64); old as usize }
        _ => { BUILD_BUG(); }
    }
}

#[cfg(feature = "CONFIG_64BIT")]
#[inline]
pub unsafe fn _percpu_read<T: Copy>(pcp: *const T) -> T {
    core::ptr::read_volatile(pcp)
}

#[cfg(feature = "CONFIG_64BIT")]
#[inline]
pub unsafe fn _percpu_write<T>(pcp: *mut T, val: T) {
    core::ptr::write_volatile(pcp, val);
}

// __percpu_xchg is __arch_xchg in the original architecture header.
pub use __arch_xchg as __percpu_xchg;

#[inline]
pub unsafe fn _protect_cmpxchg_local<T: Copy>(pcp: *mut T, old: T, new: T) -> T {
    preempt_disable_notrace();
    let ret = cmpxchg_local(raw_cpu_ptr(pcp), old, new);
    preempt_enable_notrace();
    ret
}

#[inline]
pub unsafe fn _pcp_protect<T: Copy>(operation: unsafe extern "C" fn(*mut u8, usize, usize) -> usize, pcp: *mut T, val: usize) -> T {
    preempt_disable_notrace();
    let ret = operation(raw_cpu_ptr(pcp) as *mut u8, val, core::mem::size_of::<T>());
    preempt_enable_notrace();
    core::mem::transmute_copy(&ret)
}

#[cfg(feature = "CONFIG_CPU_HAS_AMO")]
macro_rules! _percpu_add { ($pcp:expr, $val:expr) => { unsafe { __percpu_add($pcp as *mut _ as *mut u8, $val as usize, core::mem::size_of_val(&$pcp) as i32) } }; }
#[cfg(feature = "CONFIG_CPU_HAS_AMO")]
macro_rules! _percpu_add_return { ($pcp:expr, $val:expr) => { _percpu_add!($pcp, $val) }; }
#[cfg(feature = "CONFIG_CPU_HAS_AMO")]
macro_rules! _percpu_and { ($pcp:expr, $val:expr) => { unsafe { __percpu_and($pcp as *mut _ as *mut u8, $val as usize, core::mem::size_of_val(&$pcp) as i32) } }; }
#[cfg(feature = "CONFIG_CPU_HAS_AMO")]
macro_rules! _percpu_or { ($pcp:expr, $val:expr) => { unsafe { __percpu_or($pcp as *mut _ as *mut u8, $val as usize, core::mem::size_of_val(&$pcp) as i32) } }; }

#[cfg(feature = "CONFIG_64BIT")]
macro_rules! this_cpu_read_1 { ($pcp:expr) => { unsafe { _percpu_read($pcp as *const _) } }; }
#[cfg(feature = "CONFIG_64BIT")]
macro_rules! this_cpu_read_2 { ($pcp:expr) => { unsafe { _percpu_read($pcp as *const _) } }; }
#[cfg(feature = "CONFIG_64BIT")]
macro_rules! this_cpu_read_4 { ($pcp:expr) => { unsafe { _percpu_read($pcp as *const _) } }; }
#[cfg(feature = "CONFIG_64BIT")]
macro_rules! this_cpu_read_8 { ($pcp:expr) => { unsafe { _percpu_read($pcp as *const _) } }; }

#[cfg(feature = "CONFIG_64BIT")]
macro_rules! this_cpu_write_1 { ($pcp:expr, $val:expr) => { unsafe { _percpu_write($pcp as *mut _, $val) } }; }
#[cfg(feature = "CONFIG_64BIT")]
macro_rules! this_cpu_write_2 { ($pcp:expr, $val:expr) => { unsafe { _percpu_write($pcp as *mut _, $val) } }; }
#[cfg(feature = "CONFIG_64BIT")]
macro_rules! this_cpu_write_4 { ($pcp:expr, $val:expr) => { unsafe { _percpu_write($pcp as *mut _, $val) } }; }
#[cfg(feature = "CONFIG_64BIT")]
macro_rules! this_cpu_write_8 { ($pcp:expr, $val:expr) => { unsafe { _percpu_write($pcp as *mut _, $val) } }; }

macro_rules! _percpu_xchg { ($pcp:expr, $val:expr) => { unsafe { __percpu_xchg($pcp as *mut _, $val) } }; }
macro_rules! this_cpu_xchg_1 { ($pcp:expr, $val:expr) => { _percpu_xchg!($pcp, $val) }; }
macro_rules! this_cpu_xchg_2 { ($pcp:expr, $val:expr) => { _percpu_xchg!($pcp, $val) }; }
macro_rules! this_cpu_xchg_4 { ($pcp:expr, $val:expr) => { _percpu_xchg!($pcp, $val) }; }
macro_rules! this_cpu_xchg_8 { ($pcp:expr, $val:expr) => { _percpu_xchg!($pcp, $val) }; }

macro_rules! this_cpu_cmpxchg_1 { ($ptr:expr, $o:expr, $n:expr) => { unsafe { _protect_cmpxchg_local($ptr as *mut _, $o, $n) } }; }
macro_rules! this_cpu_cmpxchg_2 { ($ptr:expr, $o:expr, $n:expr) => { unsafe { _protect_cmpxchg_local($ptr as *mut _, $o, $n) } }; }
macro_rules! this_cpu_cmpxchg_4 { ($ptr:expr, $o:expr, $n:expr) => { unsafe { _protect_cmpxchg_local($ptr as *mut _, $o, $n) } }; }
macro_rules! this_cpu_cmpxchg_8 { ($ptr:expr, $o:expr, $n:expr) => { unsafe { _protect_cmpxchg_local($ptr as *mut _, $o, $n) } }; }

// The original header also includes <asm-generic/percpu.h>; its declarations
// remain supplied by the surrounding translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
