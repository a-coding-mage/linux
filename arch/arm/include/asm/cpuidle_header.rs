/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <asm/proc-fns.h>

#[cfg(CONFIG_CPU_IDLE)]
extern "C" {
    pub fn arm_cpuidle_simple_enter(
        dev: *mut cpuidle_device,
        drv: *mut cpuidle_driver,
        index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_CPU_IDLE))]
#[inline]
pub unsafe fn arm_cpuidle_simple_enter(
    _dev: *mut cpuidle_device,
    _drv: *mut cpuidle_driver,
    _index: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    -19 // -ENODEV
}

// __cpuidle_method_section is __used __section("__cpuidle_method_of_table")
// when CONFIG_CPU_IDLE is enabled, and __maybe_unused otherwise.

/* Common ARM WFI state */
#[macro_export]
macro_rules! ARM_CPUIDLE_WFI_STATE_PWR {
    ($p:expr) => {
        {
            .enter = Some($crate::arm_cpuidle_simple_enter),
            .exit_latency = 1,
            .target_residency = 1,
            .power_usage = $p,
            .name = "WFI",
            .desc = "ARM WFI",
        }
    };
}

/*
 * in case power_specified == 1, give a default WFI power value needed
 * by some governors
 */
#[macro_export]
macro_rules! ARM_CPUIDLE_WFI_STATE {
    () => {
        $crate::ARM_CPUIDLE_WFI_STATE_PWR!(::core::primitive::u32::MAX)
    };
}

#[repr(C)]
pub struct device_node;

#[repr(C)]
pub struct cpuidle_ops {
    pub suspend: Option<unsafe extern "C" fn(arg: ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
    pub init: Option<unsafe extern "C" fn(
        node: *mut device_node,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct of_cpuidle_method {
    pub method: *const ::core::ffi::c_char,
    pub ops: *const cpuidle_ops,
}

// CPUIDLE_METHOD_OF_DECLARE(name, _method, _ops)
// The C macro creates a static table entry in the linker section
// "__cpuidle_method_of_table".
#[macro_export]
macro_rules! CPUIDLE_METHOD_OF_DECLARE {
    ($name:ident, $method:expr, $ops:expr) => {
        #[allow(non_upper_case_globals)]
        #[cfg_attr(CONFIG_CPU_IDLE, link_section = "__cpuidle_method_of_table")]
        static $name: $crate::of_cpuidle_method = $crate::of_cpuidle_method {
            method: $method,
            ops: $ops,
        };
    };
}

extern "C" {
    pub fn arm_cpuidle_suspend(index: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn arm_cpuidle_init(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct arm_cpuidle_irq_context {}

#[macro_export]
macro_rules! arm_cpuidle_save_irq_context {
    ($c:expr) => {{
        let _ = &$c;
    }};
}

#[macro_export]
macro_rules! arm_cpuidle_restore_irq_context {
    ($c:expr) => {{
        let _ = &$c;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
