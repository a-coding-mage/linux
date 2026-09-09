/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by asm/smp.h in the original header.
extern "C" {
    pub static mut current_set: *mut thread_info;
    pub fn fpsave(
        fpregs: *mut ::core::ffi::c_ulong,
        fsr: *mut ::core::ffi::c_ulong,
        fpqueue: *mut ::core::ffi::c_void,
        fpqdepth: *mut ::core::ffi::c_ulong,
    );
    pub fn synchronize_user_stack();
}

// The following types and symbols are supplied by the surrounding kernel.
#[repr(C)]
pub struct thread_info {
    pub thread: thread_struct,
}

#[repr(C)]
pub struct thread_struct {
    pub float_regs: [::core::ffi::c_ulong; 0],
    pub fsr: ::core::ffi::c_ulong,
    pub fpqueue: [::core::ffi::c_ulong; 0],
    pub fpqdepth: ::core::ffi::c_ulong,
    pub kregs: *mut pt_regs,
}

#[repr(C)]
pub struct pt_regs {
    pub psr: ::core::ffi::c_ulong,
}

#[cfg(not(CONFIG_SMP))]
#[macro_export]
macro_rules! SWITCH_ENTER { ($prv:expr) => {}; }

#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! SWITCH_ENTER {
    ($prv:expr) => {{
        unsafe {
            if test_tsk_thread_flag($prv, TIF_USEDFPU) {
                put_psr(get_psr() | PSR_EF);
                fpsave(
                    (*$prv).thread.float_regs.as_mut_ptr(),
                    &mut (*$prv).thread.fsr,
                    (*$prv).thread.fpqueue.as_mut_ptr().cast(),
                    &mut (*$prv).thread.fpqdepth,
                );
                clear_tsk_thread_flag($prv, TIF_USEDFPU);
                (*(*$prv).thread.kregs).psr &= !PSR_EF;
            }
        }
    }};
}

#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! SWITCH_DO_LAZY_FPU { ($next:expr) => {}; }

#[cfg(not(CONFIG_SMP))]
#[macro_export]
macro_rules! SWITCH_DO_LAZY_FPU {
    ($nxt:expr) => {{
        unsafe {
            if last_task_used_math != $nxt {
                (*(*$nxt).thread.kregs).psr &= !PSR_EF;
            }
        }
    }};
}

#[macro_export]
macro_rules! prepare_arch_switch {
    ($next:expr) => {{
        // Original implementation is SPARC inline assembly that flushes
        // register windows; retain the operation as an external hook.
        unsafe { flush_patch_switch(); }
    }};
}

#[macro_export]
macro_rules! switch_to {
    ($prev:expr, $next:expr, $last:expr) => {{
        SWITCH_ENTER!($prev);
        SWITCH_DO_LAZY_FPU!($next);
        unsafe {
            cpumask_set_cpu(smp_processor_id(), mm_cpumask((*$next).active_mm));
            $last = sparc_switch_to($prev, $next);
        }
    }};
}

// External hooks corresponding to the original SPARC assembly and kernel APIs.
extern "C" {
    fn flush_patch_switch();
    fn sparc_switch_to(prev: *mut task_struct, next: *mut task_struct) -> *mut task_struct;
    fn test_tsk_thread_flag(task: *mut task_struct, flag: ::core::ffi::c_int) -> bool;
    fn clear_tsk_thread_flag(task: *mut task_struct, flag: ::core::ffi::c_int);
    fn put_psr(value: ::core::ffi::c_ulong);
    fn get_psr() -> ::core::ffi::c_ulong;
    fn cpumask_set_cpu(cpu: ::core::ffi::c_uint, mask: *mut cpumask);
    fn smp_processor_id() -> ::core::ffi::c_uint;
    fn mm_cpumask(mm: *mut mm_struct) -> *mut cpumask;
}

#[repr(C)] pub struct task_struct { pub thread: thread_struct, pub active_mm: *mut mm_struct }
#[repr(C)] pub struct mm_struct;
#[repr(C)] pub struct cpumask;

const TIF_USEDFPU: ::core::ffi::c_int = 0;
const PSR_EF: ::core::ffi::c_ulong = 0;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
