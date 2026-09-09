/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of arch/arm/include/asm/mcpm.h. */

pub const MAX_CPUS_PER_CLUSTER: usize = 4;

/* CONFIG_MCPM_QUAD_CLUSTER selects four clusters; otherwise there are two. */
#[cfg(feature = "CONFIG_MCPM_QUAD_CLUSTER")]
pub const MAX_NR_CLUSTERS: usize = 4;
#[cfg(not(feature = "CONFIG_MCPM_QUAD_CLUSTER"))]
pub const MAX_NR_CLUSTERS: usize = 2;

extern "C" {
    /* Entry location used when processors are released from reset. */
    pub fn mcpm_entry_point();

    pub fn mcpm_set_entry_vector(cpu: u32, cluster: u32, ptr: *mut core::ffi::c_void);
    pub fn mcpm_set_early_poke(
        cpu: u32,
        cluster: u32,
        poke_phys_addr: usize,
        poke_val: usize,
    );

    pub fn mcpm_is_available() -> bool;
    pub fn mcpm_cpu_power_up(cpu: u32, cluster: u32) -> i32;
    pub fn mcpm_cpu_power_down();
    pub fn mcpm_wait_for_cpu_powerdown(cpu: u32, cluster: u32) -> i32;
    pub fn mcpm_cpu_suspend();
    pub fn mcpm_cpu_powered_up() -> i32;

    pub fn mcpm_platform_register(ops: *const mcpm_platform_ops) -> i32;
    pub fn mcpm_sync_init(power_up_setup: Option<unsafe extern "C" fn(affinity_level: u32)>) -> i32;
    pub fn mcpm_loopback(cache_disable: Option<unsafe extern "C" fn()>) -> i32;
    pub fn mcpm_smp_set_ops();
}

#[repr(C)]
pub struct mcpm_platform_ops {
    pub cpu_powerup: Option<unsafe extern "C" fn(cpu: u32, cluster: u32) -> i32>,
    pub cluster_powerup: Option<unsafe extern "C" fn(cluster: u32) -> i32>,
    pub cpu_suspend_prepare: Option<unsafe extern "C" fn(cpu: u32, cluster: u32)>,
    pub cpu_powerdown_prepare: Option<unsafe extern "C" fn(cpu: u32, cluster: u32)>,
    pub cluster_powerdown_prepare: Option<unsafe extern "C" fn(cluster: u32)>,
    pub cpu_cache_disable: Option<unsafe extern "C" fn()>,
    pub cluster_cache_disable: Option<unsafe extern "C" fn()>,
    pub cpu_is_up: Option<unsafe extern "C" fn(cpu: u32, cluster: u32)>,
    pub cluster_is_up: Option<unsafe extern "C" fn(cluster: u32)>,
    pub wait_for_powerdown: Option<unsafe extern "C" fn(cpu: u32, cluster: u32) -> i32>,
}

/* Synchronisation state shared between C and assembly. */
#[repr(C)]
pub struct mcpm_sync_struct {
    /* The C fields are aligned to __CACHE_WRITEBACK_GRANULE. */
    pub cpus: [mcpm_cpu_state; MAX_CPUS_PER_CLUSTER],
    pub cluster: i8,
    pub inbound: i8,
}

#[repr(C)]
pub struct mcpm_cpu_state {
    pub cpu: i8,
}

#[repr(C)]
pub struct sync_struct {
    pub clusters: [mcpm_sync_struct; MAX_NR_CLUSTERS],
}

pub const CPU_DOWN: u8 = 0x11;
pub const CPU_COMING_UP: u8 = 0x12;
pub const CPU_UP: u8 = 0x13;
pub const CPU_GOING_DOWN: u8 = 0x14;

pub const CLUSTER_DOWN: u8 = 0x21;
pub const CLUSTER_UP: u8 = 0x22;
pub const CLUSTER_GOING_DOWN: u8 = 0x23;

pub const INBOUND_NOT_COMING_UP: u8 = 0x31;
pub const INBOUND_COMING_UP: u8 = 0x32;

/* Assembly offsets; cache-line size is supplied by the target headers. */
pub const MCPM_SYNC_CLUSTER_CPUS: usize = 0;
pub const MCPM_SYNC_CPU_SIZE: usize = __CACHE_WRITEBACK_GRANULE;
pub const MCPM_SYNC_CLUSTER_CLUSTER: usize =
    MCPM_SYNC_CLUSTER_CPUS + MCPM_SYNC_CPU_SIZE * MAX_CPUS_PER_CLUSTER;
pub const MCPM_SYNC_CLUSTER_INBOUND: usize =
    MCPM_SYNC_CLUSTER_CLUSTER + __CACHE_WRITEBACK_GRANULE;
pub const MCPM_SYNC_CLUSTER_SIZE: usize =
    MCPM_SYNC_CLUSTER_INBOUND + __CACHE_WRITEBACK_GRANULE;

/* Supplied by asm/cacheflush.h (or asm/asm-offsets.h for assembly builds). */
extern "C" {
    static __CACHE_WRITEBACK_GRANULE: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
