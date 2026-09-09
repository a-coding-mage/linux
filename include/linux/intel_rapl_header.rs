/* SPDX-License-Identifier: GPL-2.0 */
/* Data types and headers for RAPL support. */

// C dependencies: linux/types.h, linux/powercap.h, linux/cpuhotplug.h

#[repr(C)]
pub enum rapl_if_type {
    RAPL_IF_MSR,
    RAPL_IF_MMIO,
    RAPL_IF_TPMI,
}

#[repr(C)]
pub enum rapl_domain_type {
    RAPL_DOMAIN_PACKAGE,
    RAPL_DOMAIN_PP0,
    RAPL_DOMAIN_PP1,
    RAPL_DOMAIN_DRAM,
    RAPL_DOMAIN_PLATFORM,
    RAPL_DOMAIN_MAX,
}

#[repr(C)]
pub enum rapl_domain_reg_id {
    RAPL_DOMAIN_REG_LIMIT,
    RAPL_DOMAIN_REG_STATUS,
    RAPL_DOMAIN_REG_PERF,
    RAPL_DOMAIN_REG_POLICY,
    RAPL_DOMAIN_REG_INFO,
    RAPL_DOMAIN_REG_PL4,
    RAPL_DOMAIN_REG_UNIT,
    RAPL_DOMAIN_REG_PL2,
    RAPL_DOMAIN_REG_MAX,
}

#[repr(C)]
pub enum rapl_primitives {
    POWER_LIMIT1,
    POWER_LIMIT2,
    POWER_LIMIT4,
    ENERGY_COUNTER,
    FW_LOCK,
    FW_HIGH_LOCK,
    PL1_LOCK,
    PL2_LOCK,
    PL4_LOCK,
    PL1_ENABLE,
    PL1_CLAMP,
    PL2_ENABLE,
    PL2_CLAMP,
    PL4_ENABLE,
    TIME_WINDOW1,
    TIME_WINDOW2,
    THERMAL_SPEC_POWER,
    MAX_POWER,
    MIN_POWER,
    MAX_TIME_WINDOW,
    THROTTLED_TIME,
    PRIORITY_LEVEL,
    PSYS_POWER_LIMIT1,
    PSYS_POWER_LIMIT2,
    PSYS_PL1_ENABLE,
    PSYS_PL2_ENABLE,
    PSYS_TIME_WINDOW1,
    PSYS_TIME_WINDOW2,
    NR_RAPL_PRIMITIVES,
}

#[repr(C)]
pub struct rapl_domain_data {
    pub primitives: [u64; NR_RAPL_PRIMITIVES as usize],
    pub timestamp: usize,
}

pub const NR_POWER_LIMITS: usize = POWER_LIMIT4 as usize + 1;
pub const RAPL_DOMAIN_NAME_LENGTH: usize = 16;
pub const PACKAGE_DOMAIN_NAME_LENGTH: usize = 30;

#[repr(C)]
pub struct rapl_power_limit {
    pub constraint: *mut powercap_zone_constraint,
    pub domain: *mut rapl_domain,
    pub name: *const core::ffi::c_char,
    pub locked: bool,
    pub last_power_limit: u64,
}

#[repr(C)]
pub union rapl_reg {
    pub mmio: *mut core::ffi::c_void,
    pub msr: u32,
    pub val: u64,
}

#[repr(C)]
pub struct rapl_domain {
    pub name: [core::ffi::c_char; RAPL_DOMAIN_NAME_LENGTH],
    pub id: rapl_domain_type,
    pub regs: [rapl_reg; RAPL_DOMAIN_REG_MAX as usize],
    pub power_zone: powercap_zone,
    pub rdd: rapl_domain_data,
    pub rpl: [rapl_power_limit; NR_POWER_LIMITS],
    pub attr_map: u64,
    pub state: u32,
    pub power_unit: u32,
    pub energy_unit: u32,
    pub time_unit: u32,
    pub rp: *mut rapl_package,
}

#[repr(C)]
pub struct reg_action {
    pub reg: rapl_reg,
    pub mask: u64,
    pub value: u64,
    pub err: i32,
}

#[repr(C)]
pub struct rapl_defaults {
    pub floor_freq_reg_addr: u8,
    pub check_unit: Option<unsafe extern "C" fn(*mut rapl_domain) -> i32>,
    pub set_floor_freq: Option<unsafe extern "C" fn(*mut rapl_domain, bool)>,
    pub compute_time_window: Option<unsafe extern "C" fn(*mut rapl_domain, u64, bool) -> u64>,
    pub dram_domain_energy_unit: u32,
    pub psys_domain_energy_unit: u32,
    pub spr_psys_bits: bool,
    pub msr_pl4_support: bool,
    pub msr_pmu_support: bool,
}

#[macro_export]
macro_rules! PRIMITIVE_INFO_INIT {
    ($p:ident, $m:expr, $s:expr, $i:expr, $u:expr, $f:expr) => {
        rapl_primitive_info { name: concat!(stringify!($p), "\0").as_ptr() as *const core::ffi::c_char,
            mask: $m, shift: $s, id: $i, unit: $u, flag: $f }
    };
}

#[repr(C)]
pub enum unit_type { ARBITRARY_UNIT, POWER_UNIT, ENERGY_UNIT, TIME_UNIT }

#[repr(C)]
pub struct rapl_primitive_info {
    pub name: *const core::ffi::c_char,
    pub mask: u64,
    pub shift: i32,
    pub id: rapl_domain_reg_id,
    pub unit: unit_type,
    pub flag: u32,
}

#[repr(C)]
pub struct rapl_if_priv {
    pub type_: rapl_if_type,
    pub control_type: *mut powercap_control_type,
    pub pcap_rapl_online: cpuhp_state,
    pub reg_unit: rapl_reg,
    pub regs: [[rapl_reg; RAPL_DOMAIN_REG_MAX as usize]; RAPL_DOMAIN_MAX as usize],
    pub limits: [i32; RAPL_DOMAIN_MAX as usize],
    pub read_raw: Option<unsafe extern "C" fn(i32, *mut reg_action, bool) -> i32>,
    pub write_raw: Option<unsafe extern "C" fn(i32, *mut reg_action) -> i32>,
    pub defaults: *const rapl_defaults,
    pub rpi: *mut rapl_primitive_info,
}

#[repr(C)]
pub struct rapl_package {
    pub id: u32,
    pub nr_domains: u32,
    pub domain_map: usize,
    pub domains: *mut rapl_domain,
    pub power_zone: *mut powercap_zone,
    pub power_limit_irq: usize,
    pub plist: list_head,
    pub lead_cpu: i32,
    pub cpumask: cpumask,
    pub name: [core::ffi::c_char; PACKAGE_DOMAIN_NAME_LENGTH],
    pub priv_: *mut rapl_if_priv,
    // CONFIG_PERF_EVENTS fields are present when that build condition is enabled.
    pub has_pmu: bool,
    pub pmu_data: rapl_package_pmu_data,
}

// CONFIG_PERF_EVENTS declarations and fallback inline functions are preserved below.
#[repr(C)]
pub struct rapl_package_pmu_data {
    pub scale: [u64; RAPL_DOMAIN_MAX as usize],
    pub lock: raw_spinlock_t,
    pub n_active: i32,
    pub active_list: list_head,
    pub timer_interval: ktime_t,
    pub hrtimer: hrtimer,
}

extern "C" {
    pub fn rapl_find_package_domain_cpuslocked(id: i32, priv_: *mut rapl_if_priv, id_is_cpu: bool) -> *mut rapl_package;
    pub fn rapl_add_package_cpuslocked(id: i32, priv_: *mut rapl_if_priv, id_is_cpu: bool) -> *mut rapl_package;
    pub fn rapl_remove_package_cpuslocked(rp: *mut rapl_package);
    pub fn rapl_find_package_domain(id: i32, priv_: *mut rapl_if_priv, id_is_cpu: bool) -> *mut rapl_package;
    pub fn rapl_add_package(id: i32, priv_: *mut rapl_if_priv, id_is_cpu: bool) -> *mut rapl_package;
    pub fn rapl_remove_package(rp: *mut rapl_package);
    pub fn rapl_default_check_unit(rd: *mut rapl_domain) -> i32;
    pub fn rapl_default_set_floor_freq(rd: *mut rapl_domain, mode: bool);
    pub fn rapl_default_compute_time_window(rd: *mut rapl_domain, value: u64, to_raw: bool) -> u64;
    pub fn rapl_package_add_pmu(rp: *mut rapl_package) -> i32;
    pub fn rapl_package_add_pmu_locked(rp: *mut rapl_package) -> i32;
    pub fn rapl_package_remove_pmu(rp: *mut rapl_package);
    pub fn rapl_package_remove_pmu_locked(rp: *mut rapl_package);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
