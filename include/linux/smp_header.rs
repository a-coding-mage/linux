/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Generic SMP support
 *		Alan Cox. <alan@redhat.com>
 *
 * C header dependencies are supplied by other translated units.
 */

use core::ffi::c_void;

pub type smp_call_func_t = unsafe extern "C" fn(info: *mut c_void);
pub type smp_cond_func_t = unsafe extern "C" fn(cpu: i32, info: *mut c_void) -> bool;

/* structure shares (partial) layout with struct irq_work */
#[repr(C)]
pub struct __call_single_data {
    pub node: __call_single_node,
    pub func: smp_call_func_t,
    pub info: *mut c_void,
}

/* Use aligned() to avoid using 2 cache lines for 1 csd. */
#[repr(C, align(16))]
pub struct call_single_data_t {
    pub node: __call_single_node,
    pub func: smp_call_func_t,
    pub info: *mut c_void,
}

#[inline]
pub unsafe fn CSD_INIT(_func: smp_call_func_t, _info: *mut c_void) -> __call_single_data {
    __call_single_data { node: core::mem::zeroed(), func: _func, info: _info }
}

#[inline]
pub unsafe fn INIT_CSD(_csd: *mut __call_single_data, _func: smp_call_func_t, _info: *mut c_void) {
    *_csd = CSD_INIT(_func, _info);
}

extern "C" {
    pub fn __smp_call_single_queue(cpu: i32, node: *mut llist_node);
    pub static mut total_cpus: u32;

    pub fn smp_call_function_single(cpuid: i32, func: smp_call_func_t, info: *mut c_void, wait: bool) -> i32;
    pub fn on_each_cpu_cond_mask(cond_func: Option<smp_cond_func_t>, func: smp_call_func_t,
                                 info: *mut c_void, wait: bool, mask: *const cpumask);
    pub fn smp_call_function_single_async(cpu: i32, csd: *mut call_single_data_t) -> i32;

    pub fn panic_smp_self_stop() -> !;
    pub fn nmi_panic_self_stop(regs: *mut pt_regs) -> !;
    pub fn crash_smp_send_stop();
    pub fn panic_smp_redirect_cpu(target_cpu: i32, msg: *mut c_void) -> i32;
    pub fn smp_prepare_boot_cpu();
}

#[inline]
pub unsafe fn on_each_cpu(func: smp_call_func_t, info: *mut c_void, wait: i32) {
    on_each_cpu_cond_mask(None, func, info, wait != 0, cpu_online_mask);
}

#[inline]
pub unsafe fn on_each_cpu_mask(mask: *const cpumask, func: smp_call_func_t, info: *mut c_void, wait: bool) {
    on_each_cpu_cond_mask(None, func, info, wait, mask);
}

#[inline]
pub unsafe fn on_each_cpu_cond(cond_func: smp_cond_func_t, func: smp_call_func_t, info: *mut c_void, wait: bool) {
    on_each_cpu_cond_mask(Some(cond_func), func, info, wait, cpu_online_mask);
}

/* CONFIG_SMP declarations and implementations. */
#[cfg(CONFIG_SMP)]
extern "C" {
    pub fn smp_send_stop();
    pub fn arch_smp_send_reschedule(cpu: i32);
    pub fn smp_prepare_cpus(max_cpus: u32);
    pub fn __cpu_up(cpunum: u32, tidle: *mut task_struct) -> i32;
    pub fn smp_cpus_done(max_cpus: u32);
    pub fn smp_call_function(func: smp_call_func_t, info: *mut c_void, wait: i32);
    pub fn smp_call_function_many(mask: *const cpumask, func: smp_call_func_t, info: *mut c_void, wait: bool);
    pub fn smp_call_function_any(mask: *const cpumask, func: smp_call_func_t, info: *mut c_void, wait: i32) -> i32;
    pub fn kick_all_cpus_sync();
    pub fn wake_up_all_idle_cpus();
    pub fn cpus_peek_for_pending_ipi(mask: *const cpumask) -> bool;
    pub fn call_function_init();
    pub fn generic_smp_call_function_single_interrupt();
    pub static mut setup_max_cpus: u32;
    pub fn setup_nr_cpu_ids();
    pub fn smp_init();
    pub static mut __boot_cpu_id: i32;
}

#[cfg(CONFIG_SMP)]
#[inline]
pub unsafe fn get_boot_cpu_id() -> i32 { __boot_cpu_id }

/* !CONFIG_SMP: these fold SMP functionality into a single CPU system. */
#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn smp_send_stop() {}

#[cfg(not(CONFIG_SMP))]
pub const fn raw_smp_processor_id() -> i32 { 0 }

#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn up_smp_call_function(_func: smp_call_func_t, _info: *mut c_void) {}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn smp_send_reschedule(_cpu: i32) {}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn smp_call_function_many(_mask: *const cpumask, func: smp_call_func_t, info: *mut c_void, _wait: bool) {
    up_smp_call_function(func, info);
}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn call_function_init() {}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn smp_call_function_any(mask: *const cpumask, func: smp_call_func_t, info: *mut c_void, wait: i32) -> i32 {
    smp_call_function_single(0, func, info, wait != 0)
}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn kick_all_cpus_sync() {}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn wake_up_all_idle_cpus() {}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn cpus_peek_for_pending_ipi(_mask: *const cpumask) -> bool { false }

#[cfg(not(CONFIG_SMP))]
pub const setup_max_cpus: u32 = 0;

#[cfg(all(not(CONFIG_SMP), CONFIG_UP_LATE_INIT))]
extern "C" { pub fn up_late_init(); }

#[cfg(all(not(CONFIG_SMP), CONFIG_UP_LATE_INIT))]
#[inline]
pub unsafe fn smp_init() { up_late_init(); }

#[cfg(all(not(CONFIG_SMP), not(CONFIG_UP_LATE_INIT)))]
#[inline]
pub unsafe fn smp_init() {}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub const fn get_boot_cpu_id() -> i32 { 0 }

/* External types and globals supplied by the included Linux headers. */
extern "C" {
    static cpu_online_mask: *const cpumask;
}
pub enum __call_single_node {}
pub enum llist_node {}
pub enum cpumask {}
pub enum pt_regs {}
pub enum task_struct {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
