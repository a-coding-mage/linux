/* SPDX-License-Identifier: GPL-2.0-only */
/* Direct Rust translation of linux/include/linux/cpufreq.h. */

pub const CPUFREQ_DEFAULT_TRANSITION_LATENCY_NS: u64 = NSEC_PER_MSEC;
pub const CPUFREQ_NAME_LEN: usize = 16;
pub const CPUFREQ_NAME_PLEN: usize = CPUFREQ_NAME_LEN + 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum cpufreq_table_sorting { CPUFREQ_TABLE_UNSORTED, CPUFREQ_TABLE_SORTED_ASCENDING, CPUFREQ_TABLE_SORTED_DESCENDING }

#[repr(C)]
pub struct cpufreq_cpuinfo { pub max_freq: u32, pub min_freq: u32, pub transition_latency: u32 }

#[repr(C)]
pub struct cpufreq_policy {
    pub cpus: cpumask_var_t, pub related_cpus: cpumask_var_t, pub real_cpus: cpumask_var_t,
    pub shared_type: u32, pub cpu: u32, pub clk: *mut clk, pub cpuinfo: cpufreq_cpuinfo,
    pub min: u32, pub max: u32, pub cur: u32, pub suspend_freq: u32, pub policy: u32,
    pub last_policy: u32, pub governor: *mut cpufreq_governor, pub governor_data: *mut core::ffi::c_void,
    pub last_governor: [core::ffi::c_char; CPUFREQ_NAME_LEN], pub update: work_struct,
    pub constraints: freq_constraints, pub min_freq_req: freq_qos_request, pub max_freq_req: freq_qos_request,
    pub boost_freq_req: freq_qos_request, pub freq_table: *mut cpufreq_frequency_table,
    pub freq_table_sorted: cpufreq_table_sorting, pub policy_list: list_head, pub kobj: kobject,
    pub kobj_unregister: completion, pub rwsem: rw_semaphore, pub fast_switch_possible: bool,
    pub fast_switch_enabled: bool, pub strict_target: bool, pub efficiencies_available: bool,
    pub transition_delay_us: u32, pub dvfs_possible_from_any_cpu: bool, pub boost_enabled: bool,
    pub boost_supported: bool, pub update_limits: bool, pub cached_target_freq: u32,
    pub cached_resolved_idx: u32, pub transition_ongoing: bool, pub transition_lock: spinlock_t,
    pub transition_wait: wait_queue_head_t, pub transition_task: *mut task_struct,
    pub stats: *mut cpufreq_stats, pub driver_data: *mut core::ffi::c_void,
    pub cdev: *mut thermal_cooling_device, pub nb_min: notifier_block, pub nb_max: notifier_block,
}

#[repr(C)] pub struct cpufreq_policy_data { pub cpuinfo: cpufreq_cpuinfo, pub freq_table: *mut cpufreq_frequency_table, pub cpu: u32, pub min: u32, pub max: u32 }
#[repr(C)] pub struct cpufreq_freqs { pub policy: *mut cpufreq_policy, pub old: u32, pub new: u32, pub flags: u8 }

pub const CPUFREQ_SHARED_TYPE_NONE: u32=0; pub const CPUFREQ_SHARED_TYPE_HW:u32=1; pub const CPUFREQ_SHARED_TYPE_ALL:u32=2; pub const CPUFREQ_SHARED_TYPE_ANY:u32=3;
pub const CPUFREQ_RELATION_L:u32=0; pub const CPUFREQ_RELATION_H:u32=1; pub const CPUFREQ_RELATION_C:u32=2;
pub const CPUFREQ_RELATION_E:u32=BIT(2); pub const CPUFREQ_RELATION_LE:u32=CPUFREQ_RELATION_L|CPUFREQ_RELATION_E; pub const CPUFREQ_RELATION_HE:u32=CPUFREQ_RELATION_H|CPUFREQ_RELATION_E; pub const CPUFREQ_RELATION_CE:u32=CPUFREQ_RELATION_C|CPUFREQ_RELATION_E;

#[repr(C)] pub struct freq_attr { pub attr: attribute, pub show: Option<unsafe extern "C" fn(*mut cpufreq_policy,*mut core::ffi::c_char)->isize>, pub store: Option<unsafe extern "C" fn(*mut cpufreq_policy,*const core::ffi::c_char,usize)->isize> }
#[repr(C)] pub struct cpufreq_driver {
 pub name:[core::ffi::c_char;CPUFREQ_NAME_LEN], pub flags:u16, pub driver_data:*mut core::ffi::c_void,
 pub init:Option<unsafe extern "C" fn(*mut cpufreq_policy)->i32>, pub verify:Option<unsafe extern "C" fn(*mut cpufreq_policy_data)->i32>,
 pub setpolicy:Option<unsafe extern "C" fn(*mut cpufreq_policy)->i32>, pub target:Option<unsafe extern "C" fn(*mut cpufreq_policy,u32,u32)->i32>, pub target_index:Option<unsafe extern "C" fn(*mut cpufreq_policy,u32)->i32>, pub fast_switch:Option<unsafe extern "C" fn(*mut cpufreq_policy,u32)->u32>,
 pub adjust_perf:Option<unsafe extern "C" fn(*mut cpufreq_policy,usize,usize,usize,usize)>, pub get_intermediate:Option<unsafe extern "C" fn(*mut cpufreq_policy,u32)->u32>, pub target_intermediate:Option<unsafe extern "C" fn(*mut cpufreq_policy,u32)->i32>, pub get:Option<unsafe extern "C" fn(u32)->u32>, pub update_limits:Option<unsafe extern "C" fn(*mut cpufreq_policy)>, pub bios_limit:Option<unsafe extern "C" fn(i32,*mut u32)->i32>, pub online:Option<unsafe extern "C" fn(*mut cpufreq_policy)->i32>, pub offline:Option<unsafe extern "C" fn(*mut cpufreq_policy)->i32>, pub exit:Option<unsafe extern "C" fn(*mut cpufreq_policy)>, pub suspend:Option<unsafe extern "C" fn(*mut cpufreq_policy)->i32>, pub resume:Option<unsafe extern "C" fn(*mut cpufreq_policy)->i32>, pub ready:Option<unsafe extern "C" fn(*mut cpufreq_policy)>, pub attr:*mut *mut freq_attr, pub boost_enabled:bool, pub set_boost:Option<unsafe extern "C" fn(*mut cpufreq_policy,i32)->i32>, pub register_em:Option<unsafe extern "C" fn(*mut cpufreq_policy)>
}

pub const CPUFREQ_NEED_UPDATE_LIMITS:u16=BIT(0); pub const CPUFREQ_CONST_LOOPS:u16=BIT(1); pub const CPUFREQ_IS_COOLING_DEV:u16=BIT(2); pub const CPUFREQ_HAVE_GOVERNOR_PER_POLICY:u16=BIT(3); pub const CPUFREQ_ASYNC_NOTIFICATION:u16=BIT(4); pub const CPUFREQ_NEED_INITIAL_FREQ_CHECK:u16=BIT(5); pub const CPUFREQ_NO_AUTO_DYNAMIC_SWITCHING:u16=BIT(6);
pub const CPUFREQ_TRANSITION_NOTIFIER:u32=0; pub const CPUFREQ_POLICY_NOTIFIER:u32=1; pub const CPUFREQ_PRECHANGE:u32=0; pub const CPUFREQ_POSTCHANGE:u32=1; pub const CPUFREQ_CREATE_POLICY:u32=0; pub const CPUFREQ_REMOVE_POLICY:u32=1;
pub const CPUFREQ_POLICY_UNKNOWN:u32=0; pub const CPUFREQ_POLICY_POWERSAVE:u32=1; pub const CPUFREQ_POLICY_PERFORMANCE:u32=2;
pub const CPUFREQ_GOV_DYNAMIC_SWITCHING:u8=BIT(0); pub const CPUFREQ_GOV_STRICT_TARGET:u8=BIT(1);
pub const CPUFREQ_ENTRY_INVALID:u32=!0; pub const CPUFREQ_TABLE_END:u32=!1; pub const CPUFREQ_BOOST_FREQ:u32=1; pub const CPUFREQ_INEFFICIENT_FREQ:u32=2;
#[repr(C)] pub struct cpufreq_frequency_table { pub flags:u32, pub driver_data:u32, pub frequency:u32 }

extern "C" {
 pub fn cpufreq_cpu_get_raw(cpu:u32)->*mut cpufreq_policy; pub fn cpufreq_cpu_policy(cpu:u32)->*mut cpufreq_policy; pub fn cpufreq_cpu_get(cpu:u32)->*mut cpufreq_policy; pub fn cpufreq_cpu_put(policy:*mut cpufreq_policy);
 pub fn cpufreq_register_driver(driver_data:*mut cpufreq_driver)->i32; pub fn cpufreq_unregister_driver(driver_data:*mut cpufreq_driver);
 pub fn cpufreq_frequency_table_cpuinfo(policy:*mut cpufreq_policy)->i32; pub fn cpufreq_frequency_table_verify(policy:*mut cpufreq_policy_data)->i32; pub fn cpufreq_generic_frequency_table_verify(policy:*mut cpufreq_policy_data)->i32;
 pub fn cpufreq_table_index_unsorted(policy:*mut cpufreq_policy,target_freq:u32,min:u32,max:u32,relation:u32)->i32; pub fn cpufreq_frequency_table_get_index(policy:*mut cpufreq_policy,freq:u32)->i32;
}

#[inline] pub unsafe fn policy_is_inactive(policy:*mut cpufreq_policy)->bool { cpumask_empty((*policy).cpus) }
#[inline] pub unsafe fn policy_is_shared(policy:*mut cpufreq_policy)->bool { cpumask_nth(1,(*policy).cpus)<nr_cpumask_bits }
#[inline] pub unsafe fn cpufreq_verify_within_limits(p:*mut cpufreq_policy_data,min:u32,max:u32) { (*p).max=clamp((*p).max,min,max); (*p).min=clamp((*p).min,min,(*p).max); }
#[inline] pub unsafe fn cpufreq_verify_within_cpu_limits(p:*mut cpufreq_policy_data) { cpufreq_verify_within_limits(p,(*p).cpuinfo.min_freq,(*p).cpuinfo.max_freq); }

/* The remaining declarations depend on Linux types and macros supplied by other headers. */
extern "C" { pub fn cpufreq_get(cpu:u32)->u32; pub fn cpufreq_quick_get(cpu:u32)->u32; pub fn cpufreq_quick_get_max(cpu:u32)->u32; pub fn cpufreq_get_hw_max_freq(cpu:u32)->u32; pub fn disable_cpufreq(); pub fn cpufreq_suspend(); pub fn cpufreq_resume(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
