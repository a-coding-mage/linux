/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * Structure holding cacheline aligned fields on SMP builds.
 * Each field or group should have an ____cacheline_aligned_in_smp
 * attribute to ensure no accidental false sharing can happen.
 *
 * The atomic types and cacheline-alignment configuration are supplied by
 * other translated dependencies.
 */
#[repr(C)]
pub struct net_aligned_data {
    /* ____cacheline_aligned_in_smp */
    pub net_cookie: atomic64_t,
    #[cfg(CONFIG_INET)]
    /* ____cacheline_aligned_in_smp */
    pub tcp_memory_allocated: atomic_long_t,
    #[cfg(CONFIG_INET)]
    /* ____cacheline_aligned_in_smp */
    pub udp_memory_allocated: atomic_long_t,
}

extern "C" {
    pub static mut net_aligned_data: net_aligned_data;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
