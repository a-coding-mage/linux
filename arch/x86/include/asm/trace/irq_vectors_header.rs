/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of irq_vectors.h.
// The Linux tracepoint macros used by this header are supplied by external
// dependencies; the declarations below preserve their event data and names.

#[cfg(feature = "config_x86_local_apic")]
pub mod x86_local_apic {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct X86IrqVector {
        pub vector: ::core::ffi::c_int,
    }

    macro_rules! define_irq_vector_event {
        ($name:ident) => {
            pub const $name##_ENTRY: &str = concat!(stringify!($name), "_entry");
            pub const $name##_EXIT: &str = concat!(stringify!($name), "_exit");
        };
    }

    // local_timer - called when entering/exiting a local timer interrupt vector handler
    // spurious_apic - called when entering/exiting a spurious apic vector handler
    // error_apic - called when entering/exiting an error apic vector handler
    // x86_platform_ipi - called when entering/exiting a x86 platform ipi interrupt vector handler
    // The C macro invocations declare the corresponding tracepoints.
    pub const LOCAL_TIMER_ENTRY: &str = "local_timer_entry";
    pub const LOCAL_TIMER_EXIT: &str = "local_timer_exit";
    pub const SPURIOUS_APIC_ENTRY: &str = "spurious_apic_entry";
    pub const SPURIOUS_APIC_EXIT: &str = "spurious_apic_exit";
    pub const ERROR_APIC_ENTRY: &str = "error_apic_entry";
    pub const ERROR_APIC_EXIT: &str = "error_apic_exit";
    pub const X86_PLATFORM_IPI_ENTRY: &str = "x86_platform_ipi_entry";
    pub const X86_PLATFORM_IPI_EXIT: &str = "x86_platform_ipi_exit";

    #[cfg(feature = "config_irq_work")]
    pub const IRQ_WORK_ENTRY: &str = "irq_work_entry";
    #[cfg(feature = "config_irq_work")]
    pub const IRQ_WORK_EXIT: &str = "irq_work_exit";
    // irq_work_exit sampling is forbidden: sampling can itself generate irq_work.

    #[cfg(feature = "config_smp")]
    pub const RESCHEDULE_ENTRY: &str = "reschedule_entry";
    #[cfg(feature = "config_smp")]
    pub const RESCHEDULE_EXIT: &str = "reschedule_exit";
    #[cfg(feature = "config_smp")]
    pub const CALL_FUNCTION_ENTRY: &str = "call_function_entry";
    #[cfg(feature = "config_smp")]
    pub const CALL_FUNCTION_EXIT: &str = "call_function_exit";
    #[cfg(feature = "config_smp")]
    pub const CALL_FUNCTION_SINGLE_ENTRY: &str = "call_function_single_entry";
    #[cfg(feature = "config_smp")]
    pub const CALL_FUNCTION_SINGLE_EXIT: &str = "call_function_single_exit";

    #[cfg(feature = "config_x86_mce_threshold")]
    pub const THRESHOLD_APIC_ENTRY: &str = "threshold_apic_entry";
    #[cfg(feature = "config_x86_mce_threshold")]
    pub const THRESHOLD_APIC_EXIT: &str = "threshold_apic_exit";
    #[cfg(feature = "config_x86_mce_amd")]
    pub const DEFERRED_ERROR_APIC_ENTRY: &str = "deferred_error_apic_entry";
    #[cfg(feature = "config_x86_mce_amd")]
    pub const DEFERRED_ERROR_APIC_EXIT: &str = "deferred_error_apic_exit";
    #[cfg(feature = "config_x86_thermal_vector")]
    pub const THERMAL_APIC_ENTRY: &str = "thermal_apic_entry";
    #[cfg(feature = "config_x86_thermal_vector")]
    pub const THERMAL_APIC_EXIT: &str = "thermal_apic_exit";

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VectorConfig {
        pub irq: ::core::ffi::c_uint,
        pub vector: ::core::ffi::c_uint,
        pub cpu: ::core::ffi::c_uint,
        pub apicdest: ::core::ffi::c_uint,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VectorMod {
        pub irq: ::core::ffi::c_uint,
        pub vector: ::core::ffi::c_uint,
        pub cpu: ::core::ffi::c_uint,
        pub prev_vector: ::core::ffi::c_uint,
        pub prev_cpu: ::core::ffi::c_uint,
    }
    pub const VECTOR_UPDATE: &str = "vector_update";
    pub const VECTOR_CLEAR: &str = "vector_clear";

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VectorReserve {
        pub irq: ::core::ffi::c_uint,
        pub ret: ::core::ffi::c_int,
    }
    pub const VECTOR_RESERVE_MANAGED: &str = "vector_reserve_managed";
    pub const VECTOR_RESERVE: &str = "vector_reserve";

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VectorAlloc {
        pub irq: ::core::ffi::c_uint,
        pub vector: ::core::ffi::c_uint,
        pub reserved: bool,
        pub ret: ::core::ffi::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VectorAllocManaged {
        pub irq: ::core::ffi::c_uint,
        pub vector: ::core::ffi::c_uint,
        pub ret: ::core::ffi::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VectorActivate {
        pub irq: ::core::ffi::c_uint,
        pub is_managed: bool,
        pub can_reserve: bool,
        pub reserve: bool,
    }
    pub const VECTOR_ACTIVATE: &str = "vector_activate";
    pub const VECTOR_DEACTIVATE: &str = "vector_deactivate";

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VectorTeardown {
        pub irq: ::core::ffi::c_uint,
        pub is_managed: bool,
        pub has_reserved: bool,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VectorSetup {
        pub irq: ::core::ffi::c_uint,
        pub is_legacy: bool,
        pub ret: ::core::ffi::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VectorFreeMoved {
        pub irq: ::core::ffi::c_uint,
        pub cpu: ::core::ffi::c_uint,
        pub vector: ::core::ffi::c_uint,
        pub is_managed: bool,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
