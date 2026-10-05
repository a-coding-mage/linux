// SPDX-License-Identifier: GPL-2.0
// Continued initializers/default topology from topology.c:1786–2107, excluding
// the separate allocation and NUMA state families; baseline 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
/// Parses the boot-time scheduler-domain relaxation level.
///
/// # Safety
/// `str` must be a live NUL-terminated parameter string supplied during the
/// serialized setup phase, before init memory is discarded.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_topology_setup_relax_domain_level(str: *mut c_char) -> c_int {
    // SAFETY: The setup parser owns the string through this call and serializes
    // mutation of the native default relaxation setting.
    unsafe {
        if lupos_topology_relax_parse(str) != 0 { lupos_topology_relax_warn(); }
        1
    }
}

unsafe fn set_domain_attribute(sd: *mut sched_domain, attr: *mut sched_domain_attr) {
    // SAFETY: The build path exclusively owns sd and supplies a null attribute
    // pointer or a live attribute object for the duration of this update.
    unsafe {
        let request;
        if attr.is_null() || (*attr).relax_domain_level < 0 {
            let default_level = lupos_topology_relax_level();
            if default_level < 0 { return; }
            request = default_level;
        } else { request = (*attr).relax_domain_level; }
        if (*sd).level >= request { (*sd).flags &= !(SD_BALANCE_WAKE | SD_BALANCE_NEWIDLE); }
    }
}

unsafe fn sd_init(tl: *mut sched_domain_topology_level, cpu_map: *const cpumask,
                  child: *mut sched_domain, cpu: c_int) -> *mut sched_domain {
    // SAFETY: The serialized build owns the topology level and its allocated CPU
    // slot, keeps the input masks and optional child live, and has initialized
    // NUMA distances. Native zeroing stays inside sizeof(*sd), before its span.
    unsafe {
        let sdd = &raw mut (*tl).data;
        let sd = lupos_topology_init_sd(sdd, cpu);
        let now = lupos_topology_init_sched_clock();
        let sd_span = lupos_topology_domain_span(sd);
        lupos_topology_mask_and(sd_span, cpu_map, lupos_topology_level_mask(tl, cpu));
        let sd_weight = lupos_topology_mask_weight(sd_span) as c_int;
        // The oracle computes sd_id here although it does not subsequently use it.
        let _sd_id = lupos_topology_mask_first(sd_span) as c_int;
        let mut sd_flags = 0;
        if lupos_topology_level_has_flags(tl) { sd_flags = lupos_topology_level_flags(tl); }
        if lupos_topology_init_warn_flags(sd_flags & !(LUPOS_TOPOLOGY_ALLOWED_FLAGS as c_int) != 0) {
            sd_flags &= LUPOS_TOPOLOGY_ALLOWED_FLAGS as c_int;
        }
        sd_flags |= asym_cpu_capacity_classify(sd_span, cpu_map);
        // Native struct compound-zeroing preserves the cpumask outside sizeof(*sd).
        lupos_topology_init_domain_zero(sd);
        (*sd).min_interval = sd_weight as c_ulong;
        (*sd).max_interval = sd_weight.wrapping_mul(2) as c_ulong;
        (*sd).busy_factor = 16;
        (*sd).imbalance_pct = 117;
        (*sd).cache_nice_tries = 0;
        (*sd).flags = SD_BALANCE_NEWIDLE | SD_BALANCE_EXEC | SD_BALANCE_FORK
            | SD_WAKE_AFFINE | SD_PREFER_SIBLING | sd_flags;
        (*sd).last_balance = lupos_topology_init_jiffies();
        (*sd).balance_interval = sd_weight as c_uint;
        (*sd).newidle_call = 512;
        (*sd).newidle_success = 256;
        (*sd).newidle_ratio = 512;
        (*sd).newidle_stamp = now;
        (*sd).max_newidle_lb_cost = 0;
        (*sd).last_decay_max_lb_cost = lupos_topology_init_jiffies();
        (*sd).child = child;
        (*sd).name = (*tl).name;
        lupos_topology_init_warn_smt_asym((*sd).flags & (SD_SHARE_CPUCAPACITY | SD_ASYM_CPUCAPACITY)
            == (SD_SHARE_CPUCAPACITY | SD_ASYM_CPUCAPACITY));
        if (*sd).flags & SD_SHARE_CPUCAPACITY != 0 {
            (*sd).imbalance_pct = 110;
        } else if (*sd).flags & SD_SHARE_LLC != 0 {
            (*sd).imbalance_pct = 117;
            (*sd).cache_nice_tries = 1;
        } else {
            #[cfg(CONFIG_NUMA)]
            if (*sd).flags & SD_NUMA != 0 {
                (*sd).cache_nice_tries = 2;
                (*sd).flags &= !SD_PREFER_SIBLING;
                (*sd).flags |= SD_SERIALIZE;
                if lupos_topology_numa_domain_distance((*tl).numa_level) > lupos_topology_init_reclaim_distance() {
                    (*sd).flags &= !(SD_BALANCE_EXEC | SD_BALANCE_FORK | SD_WAKE_AFFINE);
                }
            } else { (*sd).cache_nice_tries = 1; }
            #[cfg(not(CONFIG_NUMA))]
            { (*sd).cache_nice_tries = 1; }
        }
        lupos_topology_domain_private_set(sd, sdd);
        sd
    }
}

/// Supplies the native SMT topology flags.
///
/// # Safety
/// This is a native scheduler-topology callback; it has no pointer preconditions.
#[cfg(CONFIG_SCHED_SMT)]
#[no_mangle]
pub unsafe extern "C" fn cpu_smt_flags() -> c_int { SD_SHARE_CPUCAPACITY | SD_SHARE_LLC }
/// Supplies the configured SMT sibling mask.
///
/// # Safety
/// `cpu` must identify an initialized CPU, and the caller must keep the native
/// topology mask stable while using the returned pointer. `_tl` is not read.
#[cfg(CONFIG_SCHED_SMT)]
#[no_mangle]
pub unsafe extern "C" fn tl_smt_mask(_tl: *mut sched_domain_topology_level, cpu: c_int) -> *const cpumask {
    // SAFETY: The caller provides a valid CPU and preserves native mask lifetime.
    unsafe { lupos_topology_init_smt_mask(cpu) }
}
/// Supplies the native cluster topology flags.
///
/// # Safety
/// This is a native scheduler-topology callback; it has no pointer preconditions.
#[cfg(CONFIG_SCHED_CLUSTER)]
#[no_mangle]
pub unsafe extern "C" fn cpu_cluster_flags() -> c_int { SD_CLUSTER | SD_SHARE_LLC }
/// Supplies the configured cluster mask.
///
/// # Safety
/// `cpu` must identify an initialized CPU, and the caller must keep the native
/// topology mask stable while using the returned pointer. `_tl` is not read.
#[cfg(CONFIG_SCHED_CLUSTER)]
#[no_mangle]
pub unsafe extern "C" fn tl_cls_mask(_tl: *mut sched_domain_topology_level, cpu: c_int) -> *const cpumask {
    // SAFETY: The caller provides a valid CPU and preserves native mask lifetime.
    unsafe { lupos_topology_init_cluster_mask(cpu) }
}
/// Supplies the native multicore topology flags.
///
/// # Safety
/// This is a native scheduler-topology callback; it has no pointer preconditions.
#[cfg(CONFIG_SCHED_MC)]
#[no_mangle]
pub unsafe extern "C" fn cpu_core_flags() -> c_int { SD_SHARE_LLC }
/// Supplies the configured core-group mask.
///
/// # Safety
/// `cpu` must identify an initialized CPU, and the caller must keep the native
/// topology mask stable while using the returned pointer. `_tl` is not read.
#[cfg(CONFIG_SCHED_MC)]
#[no_mangle]
pub unsafe extern "C" fn tl_mc_mask(_tl: *mut sched_domain_topology_level, cpu: c_int) -> *const cpumask {
    // SAFETY: The caller provides a valid CPU and preserves native mask lifetime.
    unsafe { lupos_topology_init_core_mask(cpu) }
}
/// Supplies the configured node/package mask.
///
/// # Safety
/// `cpu` must identify an initialized CPU, and the caller must keep the native
/// topology mask stable while using the returned pointer. `_tl` is not read.
#[no_mangle]
pub unsafe extern "C" fn tl_pkg_mask(_tl: *mut sched_domain_topology_level, cpu: c_int) -> *const cpumask {
    // SAFETY: The caller provides a valid CPU and preserves native mask lifetime.
    unsafe { lupos_topology_init_node_mask(cpu) }
}
/// Installs the architecture's boot-time scheduler topology description.
///
/// # Safety
/// Call in serialized boot setup before init memory is discarded. `tl` must
/// describe a sentinel-terminated native topology table whose storage and mask
/// callbacks remain valid for subsequent scheduler-domain construction.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_topology_set_sched_topology(tl: *mut sched_domain_topology_level) {
    // SAFETY: The boot caller provides persistent native table storage. The
    // original SMP-initialization warning guards replacement of active topology.
    unsafe {
        if lupos_topology_init_warn_smp() { return; }
        lupos_topology_numa_set_topology(tl);
        lupos_topology_numa_set_saved_topology(null_mut());
    }
}
