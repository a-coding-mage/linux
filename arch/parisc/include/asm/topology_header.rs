// Translation of parisc/include/asm/topology.h.
// The original header conditionally includes Linux topology dependencies when
// CONFIG_GENERIC_ARCH_TOPOLOGY is enabled.

#[cfg(not(feature = "CONFIG_GENERIC_ARCH_TOPOLOGY"))]
#[inline]
pub fn init_cpu_topology() {}

#[cfg(not(feature = "CONFIG_GENERIC_ARCH_TOPOLOGY"))]
#[inline]
pub fn store_cpu_topology(cpuid: core::ffi::c_uint) {
    let _ = cpuid;
}

#[cfg(not(feature = "CONFIG_GENERIC_ARCH_TOPOLOGY"))]
#[inline]
pub fn reset_cpu_topology() {}

// When CONFIG_GENERIC_ARCH_TOPOLOGY is enabled, init_cpu_topology,
// store_cpu_topology, and reset_cpu_topology are supplied by the generic
// architecture topology dependencies (linux/cpumask.h and
// linux/arch_topology.h in the original source).

// Dependency equivalent of <asm-generic/topology.h>.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
