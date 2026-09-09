/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/bitops.h, linux/cpumask.h, asm/smp-ops.h

// The following items are conditional on CONFIG_SMP in the original header.
// Keep this condition when integrating into a configuration-aware build.

#[cfg(CONFIG_SMP)]
#[inline]
pub unsafe fn raw_smp_processor_id() -> i32 {
    (*current_thread_info()).cpu
}

/// Map from cpu id to sequential logical cpu number.
#[cfg(CONFIG_SMP)]
extern "C" {
    pub static mut __cpu_number_map: [i32; NR_CPUS];
}

#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! cpu_number_map {
    ($cpu:expr) => {
        unsafe { $crate::__cpu_number_map[$cpu as usize] }
    };
}

/// The reverse map from sequential logical cpu number to cpu id.
#[cfg(CONFIG_SMP)]
extern "C" {
    pub static mut __cpu_logical_map: [i32; NR_CPUS];
}

#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! cpu_logical_map {
    ($cpu:expr) => {
        unsafe { $crate::__cpu_logical_map[$cpu as usize] }
    };
}

#[cfg(CONFIG_SMP)]
#[repr(i32)]
pub enum SmpMsg {
    SMP_MSG_FUNCTION,
    SMP_MSG_RESCHEDULE,
    SMP_MSG_FUNCTION_SINGLE,
    SMP_MSG_TIMER,

    // must be last
    SMP_MSG_NR,
}

// DECLARE_PER_CPU(int, cpu_state)
#[cfg(CONFIG_SMP)]
extern "C" {
    pub static mut cpu_state: i32;
}

#[cfg(CONFIG_SMP)]
extern "C" {
    pub fn smp_message_recv(msg: u32);

    pub fn arch_send_call_function_single_ipi(cpu: i32);
    pub fn arch_send_call_function_ipi_mask(mask: *const cpumask);

    pub fn native_play_dead();
    pub fn native_cpu_die(cpu: u32);
    pub fn native_cpu_disable(cpu: u32) -> i32;
}

#[cfg(all(CONFIG_SMP, CONFIG_HOTPLUG_CPU))]
extern "C" {
    pub fn play_dead_common();
    pub fn __cpu_disable() -> i32;
}

#[cfg(all(CONFIG_SMP, CONFIG_HOTPLUG_CPU))]
#[inline]
pub unsafe fn __cpu_die(cpu: u32) {
    extern "C" {
        static mut mp_ops: *mut plat_smp_ops;
    }

    (*mp_ops).cpu_die(cpu);
}

#[cfg(CONFIG_SMP)]
#[inline]
pub unsafe fn hard_smp_processor_id() -> i32 {
    extern "C" {
        static mut mp_ops: *mut plat_smp_ops;
    }

    if mp_ops.is_null() {
        return 0; // boot CPU
    }

    (*mp_ops).smp_processor_id()
}

#[cfg(CONFIG_SMP)]
#[repr(C)]
pub struct of_cpu_method {
    pub method: *const core::ffi::c_char,
    pub ops: *mut plat_smp_ops,
}

// CPU_METHOD_OF_DECLARE(name, _method, _ops)
#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! CPU_METHOD_OF_DECLARE {
    ($name:ident, $method:expr, $ops:expr) => {
        #[used]
        #[link_section = "__cpu_method_of_table"]
        static $name: $crate::of_cpu_method = $crate::of_cpu_method {
            method: $method,
            ops: $ops,
        };
    };
}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub const fn hard_smp_processor_id() -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
