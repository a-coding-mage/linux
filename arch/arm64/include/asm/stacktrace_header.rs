/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/percpu.h, linux/sched.h, linux/sched/task_stack.h, linux/llist.h,
// asm/memory.h, asm/pointer_auth.h, asm/ptrace.h, asm/sdei.h,
// and asm/stacktrace/common.h.

extern "C" {
    pub fn dump_backtrace(regs: *mut pt_regs, tsk: *mut task_struct, loglvl: *const ::core::ffi::c_char);
}

extern "C" {
    pub static mut irq_stack_ptr: *mut ::core::ffi::c_ulong;
}

#[inline]
pub unsafe fn stackinfo_get_irq() -> stack_info {
    let low: ::core::ffi::c_ulong = raw_cpu_read(irq_stack_ptr) as ::core::ffi::c_ulong;
    let high = low.wrapping_add(IRQ_STACK_SIZE as ::core::ffi::c_ulong);

    stack_info { low, high }
}

#[inline]
pub unsafe fn on_irq_stack(sp: ::core::ffi::c_ulong, size: ::core::ffi::c_ulong) -> bool {
    let info = stackinfo_get_irq();
    stackinfo_on_stack(&info, sp, size)
}

#[inline]
pub unsafe fn stackinfo_get_task(tsk: *const task_struct) -> stack_info {
    let low: ::core::ffi::c_ulong = task_stack_page(tsk) as ::core::ffi::c_ulong;
    let high = low.wrapping_add(THREAD_SIZE as ::core::ffi::c_ulong);

    stack_info { low, high }
}

#[inline]
pub unsafe fn on_task_stack(
    tsk: *const task_struct,
    sp: ::core::ffi::c_ulong,
    size: ::core::ffi::c_ulong,
) -> bool {
    let info = stackinfo_get_task(tsk);
    stackinfo_on_stack(&info, sp, size)
}

#[macro_export]
macro_rules! on_thread_stack {
    () => {
        $crate::on_task_stack(current, current_stack_pointer, 1)
    };
}

extern "C" {
    pub static mut overflow_stack: [::core::ffi::c_ulong; OVERFLOW_STACK_SIZE / ::core::mem::size_of::<::core::ffi::c_ulong>()];
}

#[inline]
pub unsafe fn stackinfo_get_overflow() -> stack_info {
    let low: ::core::ffi::c_ulong = raw_cpu_ptr(overflow_stack) as ::core::ffi::c_ulong;
    let high = low.wrapping_add(OVERFLOW_STACK_SIZE as ::core::ffi::c_ulong);

    stack_info { low, high }
}

// CONFIG_ARM_SDE_INTERFACE
#[cfg(feature = "CONFIG_ARM_SDE_INTERFACE")]
extern "C" {
    pub static mut sdei_stack_normal_ptr: *mut ::core::ffi::c_ulong;
    pub static mut sdei_stack_critical_ptr: *mut ::core::ffi::c_ulong;
}

#[cfg(feature = "CONFIG_ARM_SDE_INTERFACE")]
#[inline]
pub unsafe fn stackinfo_get_sdei_normal() -> stack_info {
    let low: ::core::ffi::c_ulong = raw_cpu_read(sdei_stack_normal_ptr) as ::core::ffi::c_ulong;
    let high = low.wrapping_add(SDEI_STACK_SIZE as ::core::ffi::c_ulong);

    stack_info { low, high }
}

#[cfg(feature = "CONFIG_ARM_SDE_INTERFACE")]
#[inline]
pub unsafe fn stackinfo_get_sdei_critical() -> stack_info {
    let low: ::core::ffi::c_ulong = raw_cpu_read(sdei_stack_critical_ptr) as ::core::ffi::c_ulong;
    let high = low.wrapping_add(SDEI_STACK_SIZE as ::core::ffi::c_ulong);

    stack_info { low, high }
}

#[cfg(not(feature = "CONFIG_ARM_SDE_INTERFACE"))]
#[inline]
pub unsafe fn stackinfo_get_sdei_normal() -> stack_info {
    stackinfo_get_unknown()
}

#[cfg(not(feature = "CONFIG_ARM_SDE_INTERFACE"))]
#[inline]
pub unsafe fn stackinfo_get_sdei_critical() -> stack_info {
    stackinfo_get_unknown()
}

// CONFIG_EFI
#[cfg(feature = "CONFIG_EFI")]
extern "C" {
    pub static mut efi_rt_stack_top: *mut u64;
}

#[cfg(feature = "CONFIG_EFI")]
#[inline]
pub unsafe fn stackinfo_get_efi() -> stack_info {
    let high: ::core::ffi::c_ulong = efi_rt_stack_top as ::core::ffi::c_ulong;
    let low = high.wrapping_sub(THREAD_SIZE as ::core::ffi::c_ulong);

    stack_info { low, high }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
