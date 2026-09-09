/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Rust translation of drivers/cpufreq/cpufreq_governor.h
 */

/* Dependencies supplied by the surrounding kernel translation are intentionally
 * left as external types and symbols. */

/* Ondemand Sampling types */
#[repr(C)]
pub enum OdSampleType {
    OD_NORMAL_SAMPLE,
    OD_SUB_SAMPLE,
}

/*
 * Abbreviations:
 * dbs: used as a shortform for demand based switching
 * cdbs: common dbs
 * od_*: On-demand governor
 * cs_*: Conservative governor
 */

/* Governor demand based switching data (per-policy or global). */
#[repr(C)]
pub struct dbs_data {
    pub attr_set: gov_attr_set,
    pub gov: *mut dbs_governor,
    pub tuners: *mut core::ffi::c_void,
    pub ignore_nice_load: core::ffi::c_uint,
    pub sampling_rate: core::ffi::c_uint,
    pub sampling_down_factor: core::ffi::c_uint,
    pub up_threshold: core::ffi::c_uint,
    pub io_is_busy: core::ffi::c_uint,
}

#[inline]
pub unsafe fn to_dbs_data(attr_set: *mut gov_attr_set) -> *mut dbs_data {
    container_of!(attr_set, dbs_data, attr_set)
}

/* The C gov_show_one/gov_show_one_common macros generate governor-specific
 * sysfs show functions; their expansion is retained here as a Rust macro. */
#[macro_export]
macro_rules! gov_show_one_common {
    ($file_name:ident) => {
        unsafe fn $file_name##_show(attr_set: *mut gov_attr_set, buf: *mut core::ffi::c_char) -> isize {
            let dbs_data = unsafe { to_dbs_data(attr_set) };
            unsafe { sysfs_emit!(buf, "%u\n", (*dbs_data).$file_name) }
        }
    };
}

/* gov_show_one(_gov, file_name) has the same generated shape, using the
 * governor-specific tuners object. */
#[macro_export]
macro_rules! gov_show_one {
    ($gov:ident, $file_name:ident) => {
        /* Expansion requires the surrounding translation's identifier
         * concatenation and sysfs_emit facilities. */
    };
}

#[macro_export]
macro_rules! gov_attr_ro {
    ($name:ident) => {
        static mut $name: governor_attr = __ATTR_RO!($name);
    };
}

#[macro_export]
macro_rules! gov_attr_rw {
    ($name:ident) => {
        static mut $name: governor_attr = __ATTR_RW!($name);
    };
}

/* Common to all CPUs of a policy */
#[repr(C)]
pub struct policy_dbs_info {
    pub policy: *mut cpufreq_policy,
    /* Per policy mutex that serializes load evaluation from limit-change and
     * work-handler. */
    pub update_mutex: mutex,
    pub last_sample_time: u64,
    pub sample_delay_ns: i64,
    pub work_count: atomic_t,
    pub irq_work: irq_work,
    pub work: work_struct,
    /* dbs_data may be shared between multiple policy objects */
    pub dbs_data: *mut dbs_data,
    pub list: list_head,
    /* Multiplier for increasing sample delay temporarily. */
    pub rate_mult: core::ffi::c_uint,
    pub idle_periods: core::ffi::c_uint, /* For conservative */
    /* Status indicators */
    pub is_shared: bool,          /* This object is used by multiple CPUs */
    pub work_in_progress: bool,   /* Work is being queued up or in progress */
}

#[inline]
pub unsafe fn gov_update_sample_delay(policy_dbs: *mut policy_dbs_info, delay_us: core::ffi::c_uint) {
    (*policy_dbs).sample_delay_ns = (delay_us as u64).wrapping_mul(NSEC_PER_USEC as u64) as i64;
}

/* Per cpu structures */
#[repr(C)]
pub struct cpu_dbs_info {
    pub prev_cpu_idle: u64,
    pub prev_update_time: u64,
    pub prev_cpu_nice: u64,
    /* Used to keep track of load in the previous interval. However, when
     * explicitly set to zero, it is used as a flag to ensure that we copy the
     * previous load to the current interval only once, upon the first wake-up
     * from idle. */
    pub prev_load: core::ffi::c_uint,
    pub update_util: update_util_data,
    pub policy_dbs: *mut policy_dbs_info,
}

/* Common Governor data across policies */
#[repr(C)]
pub struct dbs_governor {
    pub gov: cpufreq_governor,
    pub kobj_type: kobj_type,
    /* Common data for platforms that don't set CPUFREQ_HAVE_GOVERNOR_PER_POLICY */
    pub gdbs_data: *mut dbs_data,
    pub gov_dbs_update: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> core::ffi::c_uint>,
    pub alloc: Option<unsafe extern "C" fn() -> *mut policy_dbs_info>,
    pub free: Option<unsafe extern "C" fn(*mut policy_dbs_info)>,
    pub init: Option<unsafe extern "C" fn(*mut dbs_data) -> core::ffi::c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut dbs_data)>,
    pub start: Option<unsafe extern "C" fn(*mut cpufreq_policy)>,
    pub limits: Option<unsafe extern "C" fn(*mut cpufreq_policy)>,
}

#[inline]
pub unsafe fn dbs_governor_of(policy: *mut cpufreq_policy) -> *mut dbs_governor {
    container_of!((*policy).governor, dbs_governor, gov)
}

/* Governor callback routines */
unsafe extern "C" {
    pub fn cpufreq_dbs_governor_init(policy: *mut cpufreq_policy) -> core::ffi::c_int;
    pub fn cpufreq_dbs_governor_exit(policy: *mut cpufreq_policy);
    pub fn cpufreq_dbs_governor_start(policy: *mut cpufreq_policy) -> core::ffi::c_int;
    pub fn cpufreq_dbs_governor_stop(policy: *mut cpufreq_policy);
    pub fn cpufreq_dbs_governor_limits(policy: *mut cpufreq_policy);
}

#[macro_export]
macro_rules! CPUFREQ_DBS_GOVERNOR_INITIALIZER {
    ($name:expr) => {
        cpufreq_governor {
            name: $name,
            flags: CPUFREQ_GOV_DYNAMIC_SWITCHING,
            owner: THIS_MODULE,
            init: Some(cpufreq_dbs_governor_init),
            exit: Some(cpufreq_dbs_governor_exit),
            start: Some(cpufreq_dbs_governor_start),
            stop: Some(cpufreq_dbs_governor_stop),
            limits: Some(cpufreq_dbs_governor_limits),
        }
    };
}

/* Governor specific operations */
#[repr(C)]
pub struct od_ops {
    pub powersave_bias_target: Option<unsafe extern "C" fn(*mut cpufreq_policy, core::ffi::c_uint, core::ffi::c_uint) -> core::ffi::c_uint>,
}

unsafe extern "C" {
    pub fn dbs_update(policy: *mut cpufreq_policy) -> core::ffi::c_uint;
    pub fn od_register_powersave_bias_handler(
        f: Option<unsafe extern "C" fn(*mut cpufreq_policy, core::ffi::c_uint, core::ffi::c_uint)>,
        powersave_bias: core::ffi::c_uint,
    );
    pub fn od_unregister_powersave_bias_handler();
    pub fn sampling_rate_store(attr_set: *mut gov_attr_set, buf: *const core::ffi::c_char, count: usize) -> isize;
    pub fn gov_update_cpu_data(dbs_data: *mut dbs_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
