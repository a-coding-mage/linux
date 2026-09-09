/* SPDX-License-Identifier: GPL-2.0 */

// The CONFIG_ARM_CPU_TOPOLOGY build-time condition is preserved with Rust cfg.
#[cfg(feature = "CONFIG_ARM_CPU_TOPOLOGY")]
mod arm_cpu_topology {
    // big.LITTLE switcher is incompatible with frequency invariance.
    // The following macro aliases correspond to the C preprocessor mappings
    // when CONFIG_BL_SWITCHER is not enabled.
    #[cfg(not(feature = "CONFIG_BL_SWITCHER"))]
    macro_rules! arch_set_freq_scale {
        ($($args:tt)*) => { topology_set_freq_scale!($($args)*) };
    }

    #[cfg(not(feature = "CONFIG_BL_SWITCHER"))]
    macro_rules! arch_scale_freq_capacity {
        ($($args:tt)*) => { topology_get_freq_scale!($($args)*) };
    }

    #[cfg(not(feature = "CONFIG_BL_SWITCHER"))]
    macro_rules! arch_scale_freq_invariant {
        ($($args:tt)*) => { topology_scale_freq_invariant!($($args)*) };
    }

    #[cfg(not(feature = "CONFIG_BL_SWITCHER"))]
    macro_rules! arch_scale_freq_ref {
        ($($args:tt)*) => { topology_get_freq_ref!($($args)*) };
    }

    // Replace task scheduler's default cpu-invariant accounting.
    macro_rules! arch_scale_cpu_capacity {
        ($($args:tt)*) => { topology_get_cpu_scale!($($args)*) };
    }

    // Enable topology flag updates.
    macro_rules! arch_update_cpu_topology {
        ($($args:tt)*) => { topology_update_cpu_topology!($($args)*) };
    }

    // Replace task scheduler's default HW pressure API.
    macro_rules! arch_scale_hw_pressure {
        ($($args:tt)*) => { topology_get_hw_pressure!($($args)*) };
    }

    macro_rules! arch_update_hw_pressure {
        ($($args:tt)*) => { topology_update_hw_pressure!($($args)*) };
    }
}

#[cfg(not(feature = "CONFIG_ARM_CPU_TOPOLOGY"))]
#[inline]
pub fn init_cpu_topology() {}

#[cfg(not(feature = "CONFIG_ARM_CPU_TOPOLOGY"))]
#[inline]
pub fn store_cpu_topology(_cpuid: core::ffi::c_uint) {}

// Declaration supplied by asm-generic/topology.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
