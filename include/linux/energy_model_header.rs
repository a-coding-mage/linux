/* SPDX-License-Identifier: GPL-2.0 */
/* C header dependencies are supplied by other translated units. */

#[repr(C)]
pub struct em_perf_state {
    pub performance: ::core::ffi::c_ulong,
    pub frequency: ::core::ffi::c_ulong,
    pub power: ::core::ffi::c_ulong,
    pub cost: ::core::ffi::c_ulong,
    pub flags: ::core::ffi::c_ulong,
}

pub const EM_PERF_STATE_INEFFICIENT: ::core::ffi::c_ulong = 1 << 0;

#[repr(C)]
pub struct em_perf_table {
    pub rcu: rcu_head,
    pub kref: kref,
    pub state: [em_perf_state; 0],
}

#[repr(C)]
pub struct em_perf_domain {
    pub em_table: *mut em_perf_table,
    pub node: list_head,
    pub id: ::core::ffi::c_int,
    pub nr_perf_states: ::core::ffi::c_int,
    pub min_perf_state: ::core::ffi::c_int,
    pub max_perf_state: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_ulong,
    pub cpus: [::core::ffi::c_ulong; 0],
}

pub const EM_PERF_DOMAIN_MICROWATTS: ::core::ffi::c_ulong = 1 << 0;
pub const EM_PERF_DOMAIN_SKIP_INEFFICIENCIES: ::core::ffi::c_ulong = 1 << 1;
pub const EM_PERF_DOMAIN_ARTIFICIAL: ::core::ffi::c_ulong = 1 << 2;
pub const EM_MAX_POWER: ::core::ffi::c_ulong = 64000000;
pub const EM_MAX_NUM_CPUS: usize = 4096; /* CONFIG_64BIT; otherwise 16 */

#[repr(C)]
pub struct em_data_callback {
    pub active_power: Option<unsafe extern "C" fn(*mut device, *mut ::core::ffi::c_ulong, *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
    pub get_cost: Option<unsafe extern "C" fn(*mut device, ::core::ffi::c_ulong, *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
}

#[macro_export]
macro_rules! EM_SET_ACTIVE_POWER_CB { ($em_cb:expr, $cb:expr) => { $em_cb.active_power = Some($cb) }; }
#[macro_export]
macro_rules! EM_ADV_DATA_CB { ($active_power_cb:expr, $cost_cb:expr) => { em_data_callback { active_power: Some($active_power_cb), get_cost: Some($cost_cb) } }; }
#[macro_export]
macro_rules! EM_DATA_CB { ($active_power_cb:expr) => { EM_ADV_DATA_CB!($active_power_cb, core::compile_error!("NULL callback requires an external C-compatible function pointer")) }; }

/* CONFIG_ENERGY_MODEL disabled declarations and stubs are build-time alternatives
 * to the declarations above; their C header semantics are preserved here. */

pub const fn em_span_cpus(em: *mut em_perf_domain) -> *mut ::core::ffi::c_ulong { unsafe { (*em).cpus.as_mut_ptr() } }
pub unsafe fn em_is_artificial(em: *const em_perf_domain) -> bool { ((*em).flags & EM_PERF_DOMAIN_ARTIFICIAL) != 0 }

extern "C" {
    pub fn em_cpu_get(cpu: ::core::ffi::c_int) -> *mut em_perf_domain;
    pub fn em_pd_get(dev: *mut device) -> *mut em_perf_domain;
    pub fn em_dev_update_perf_domain(dev: *mut device, new_table: *mut em_perf_table) -> ::core::ffi::c_int;
    pub fn em_dev_register_perf_domain(dev: *mut device, nr_states: ::core::ffi::c_uint, cb: *const em_data_callback, cpus: *const cpumask_t, microwatts: bool) -> ::core::ffi::c_int;
    pub fn em_dev_register_pd_no_update(dev: *mut device, nr_states: ::core::ffi::c_uint, cb: *const em_data_callback, cpus: *const cpumask_t, microwatts: bool) -> ::core::ffi::c_int;
    pub fn em_dev_unregister_perf_domain(dev: *mut device);
    pub fn em_table_alloc(pd: *mut em_perf_domain) -> *mut em_perf_table;
    pub fn em_table_free(table: *mut em_perf_table);
    pub fn em_dev_compute_costs(dev: *mut device, table: *mut em_perf_state, nr_states: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn em_dev_update_chip_binning(dev: *mut device) -> ::core::ffi::c_int;
    pub fn em_update_performance_limits(pd: *mut em_perf_domain, freq_min_khz: ::core::ffi::c_ulong, freq_max_khz: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn em_adjust_cpu_capacity(cpu: ::core::ffi::c_uint);
    pub fn em_rebuild_sched_domains();
}

pub unsafe fn em_pd_get_efficient_state(table: *mut em_perf_state, pd: *mut em_perf_domain, max_util: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    let pd_flags = (*pd).flags;
    let min_ps = (*pd).min_perf_state;
    let max_ps = (*pd).max_perf_state;
    let mut i = min_ps;
    while i <= max_ps {
        let ps = &*table.add(i as usize);
        if ps.performance >= max_util {
            if (pd_flags & EM_PERF_DOMAIN_SKIP_INEFFICIENCIES) != 0 && (ps.flags & EM_PERF_STATE_INEFFICIENT) != 0 { i += 1; continue; }
            return i;
        }
        i += 1;
    }
    max_ps
}

pub unsafe fn em_cpu_energy(pd: *mut em_perf_domain, max_util: ::core::ffi::c_ulong, sum_util: ::core::ffi::c_ulong, allowed_cpu_cap: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    if sum_util == 0 { return 0; }
    let max_util = core::cmp::min(max_util, allowed_cpu_cap);
    let em_table = (*pd).em_table;
    let i = em_pd_get_efficient_state((*em_table).state.as_mut_ptr(), pd, max_util);
    (*em_table).state[i as usize].cost.wrapping_mul(sum_util)
}

pub unsafe fn em_pd_nr_perf_states(pd: *mut em_perf_domain) -> ::core::ffi::c_int { (*pd).nr_perf_states }
pub unsafe fn em_perf_state_from_pd(pd: *mut em_perf_domain) -> *mut em_perf_state { (*pd).em_table.as_mut().unwrap().state.as_mut_ptr() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
