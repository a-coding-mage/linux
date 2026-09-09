/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: linux/arch_topology.h
// When CONFIG_NUMA is enabled, dependency: asm/numa.h

/* Replace task scheduler's default frequency-invariant accounting */
pub use topology_scale_freq_tick as arch_scale_freq_tick;
pub use topology_set_freq_scale as arch_set_freq_scale;
pub use topology_get_freq_scale as arch_scale_freq_capacity;
pub use topology_scale_freq_invariant as arch_scale_freq_invariant;
pub use topology_get_freq_ref as arch_scale_freq_ref;

/* Replace task scheduler's default cpu-invariant accounting */
pub use topology_get_cpu_scale as arch_scale_cpu_capacity;

/* Enable topology flag updates */
pub use topology_update_cpu_topology as arch_update_cpu_topology;

// Dependency: asm-generic/topology.h

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
