/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2013 ARM Ltd. */

// C dependencies: linux/preempt.h, asm/alternative.h, asm/cmpxchg.h,
// asm/stack_pointer.h, asm/sysreg.h, and asm-generic/percpu.h.

#[inline(always)]
pub unsafe fn set_my_cpu_offset(off: usize) {
    // C: ALTERNATIVE("msr tpidr_el1, %0", "msr tpidr_el2, %0", ARM64_HAS_VIRT_HOST_EXTN)
    // The selected TPIDR register is supplied by the target ARM build.
    core::arch::asm!("msr tpidr_el1, {0}", in(reg) off, options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn __hyp_my_cpu_offset() -> usize {
    let value: usize;
    core::arch::asm!("mrs {0}, tpidr_el2", out(reg) value, options(nostack, preserves_flags));
    value
}

#[inline(always)]
pub unsafe fn __kern_my_cpu_offset() -> usize {
    let value: usize;
    // The original fake stack read hazards this access against barrier().
    core::arch::asm!("mrs {0}, tpidr_el1", out(reg) value, options(nostack, preserves_flags));
    value
}

#[cfg(feature = "kvm_nvhe_hypervisor")]
#[inline(always)]
pub unsafe fn __my_cpu_offset() -> usize { __hyp_my_cpu_offset() }

#[cfg(not(feature = "kvm_nvhe_hypervisor"))]
#[inline(always)]
pub unsafe fn __my_cpu_offset() -> usize { __kern_my_cpu_offset() }

macro_rules! percpu_rw_ops {
    ($bits:literal, $ty:ty, $read:ident, $write:ident) => {
        #[inline(always)]
        pub unsafe fn $read(ptr: *mut core::ffi::c_void) -> usize {
            core::ptr::read_volatile(ptr as *const $ty) as usize
        }
        #[inline(always)]
        pub unsafe fn $write(ptr: *mut core::ffi::c_void, val: usize) {
            core::ptr::write_volatile(ptr as *mut $ty, val as $ty)
        }
    };
}

percpu_rw_ops!(8, u8, __percpu_read_8, __percpu_write_8);
percpu_rw_ops!(16, u16, __percpu_read_16, __percpu_write_16);
percpu_rw_ops!(32, u32, __percpu_read_32, __percpu_write_32);
percpu_rw_ops!(64, u64, __percpu_read_64, __percpu_write_64);

macro_rules! percpu_ops {
    ($bits:literal, $ty:ty, $add:ident, $andnot:ident, $or:ident,
     $add_ret:ident) => {
        #[inline(always)]
        pub unsafe fn $add(ptr: *mut core::ffi::c_void, val: usize) {
            let p = ptr as *mut $ty;
            let old = core::ptr::read_volatile(p);
            core::ptr::write_volatile(p, old.wrapping_add(val as $ty));
        }
        #[inline(always)]
        pub unsafe fn $andnot(ptr: *mut core::ffi::c_void, val: usize) {
            let p = ptr as *mut $ty;
            let old = core::ptr::read_volatile(p);
            core::ptr::write_volatile(p, old & !(val as $ty));
        }
        #[inline(always)]
        pub unsafe fn $or(ptr: *mut core::ffi::c_void, val: usize) {
            let p = ptr as *mut $ty;
            let old = core::ptr::read_volatile(p);
            core::ptr::write_volatile(p, old | val as $ty);
        }
        #[inline(always)]
        pub unsafe fn $add_ret(ptr: *mut core::ffi::c_void, val: usize) -> $ty {
            let p = ptr as *mut $ty;
            let old = core::ptr::read_volatile(p);
            let new = old.wrapping_add(val as $ty);
            core::ptr::write_volatile(p, new);
            new
        }
    };
}

percpu_ops!(8, u8, __percpu_add_case_8, __percpu_andnot_case_8, __percpu_or_case_8, __percpu_add_return_case_8);
percpu_ops!(16, u16, __percpu_add_case_16, __percpu_andnot_case_16, __percpu_or_case_16, __percpu_add_return_case_16);
percpu_ops!(32, u32, __percpu_add_case_32, __percpu_andnot_case_32, __percpu_or_case_32, __percpu_add_return_case_32);
percpu_ops!(64, u64, __percpu_add_case_64, __percpu_andnot_case_64, __percpu_or_case_64, __percpu_add_return_case_64);

// The following macros preserve the C per-CPU API and its preemption bracketing.
// raw_cpu_ptr, preempt_disable_notrace, preempt_enable_notrace, xchg_relaxed,
// cmpxchg_relaxed, and cmpxchg128_local are supplied by dependent headers.
#[macro_export]
macro_rules! _pcp_protect { ($op:path, $pcp:expr $(, $arg:expr)*) => {{
    unsafe { preempt_disable_notrace(); $op(raw_cpu_ptr(core::ptr::addr_of!($pcp)), $($arg),*); preempt_enable_notrace(); }
}} }
#[macro_export]
macro_rules! _pcp_protect_return { ($op:path, $pcp:expr $(, $arg:expr)*) => {{
    unsafe { preempt_disable_notrace(); let r = $op(raw_cpu_ptr(core::ptr::addr_of!($pcp)), $($arg),*); preempt_enable_notrace(); r }
}} }

extern "C" {
    fn preempt_disable_notrace();
    fn preempt_enable_notrace();
    fn raw_cpu_ptr(ptr: *const u8) -> *mut core::ffi::c_void;
    fn cmpxchg128_local(ptr: *mut core::ffi::c_void, old: u128, new: u128) -> u128;
    fn __hyp_per_cpu_offset(cpu: u32) -> usize;
}

#[cfg(feature = "kvm_nvhe_hypervisor")]
#[inline(always)]
pub unsafe fn per_cpu_offset(cpu: u32) -> usize { __hyp_per_cpu_offset(cpu) }

#[inline(always)]
pub unsafe fn this_cpu_cmpxchg128<T>(pcp: *mut T, old: u128, new: u128) -> u128 {
    preempt_disable_notrace();
    let ret = cmpxchg128_local(pcp.cast(), old, new);
    preempt_enable_notrace();
    ret
}

// asm-generic/percpu.h declarations and the DEBUG_PREEMPT nVHE macro
// redefinitions are intentionally provided by the generic dependency.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
