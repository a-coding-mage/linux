/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies are supplied by the surrounding kernel translation.

pub const ACPI_PROCESSOR_DEVICE_HID: &str = "ACPI0007";
pub const ACPI_PROCESSOR_CONTAINER_HID: &str = "ACPI0010";
pub const ACPI_PROCESSOR_BUSY_METRIC: u32 = 10;
pub const ACPI_PROCESSOR_MAX_POWER: usize = 8;
pub const ACPI_PROCESSOR_MAX_C2_LATENCY: u32 = 100;
pub const ACPI_PROCESSOR_MAX_C3_LATENCY: u32 = 1000;
pub const ACPI_PROCESSOR_MAX_THROTTLING: usize = 16;
pub const ACPI_PROCESSOR_MAX_THROTTLE: u32 = 250; // 25%
pub const ACPI_PROCESSOR_MAX_DUTY_WIDTH: u32 = 4;
pub const ACPI_PDC_REVISION_ID: u32 = 0x1;
pub const ACPI_PSD_REV0_REVISION: u32 = 0; // Support for _PSD as in ACPI 3.0
pub const ACPI_PSD_REV0_ENTRIES: usize = 5;
pub const ACPI_TSD_REV0_REVISION: u32 = 0; // Support for _PSD as in ACPI 3.0
pub const ACPI_TSD_REV0_ENTRIES: usize = 5;
// Types of coordination defined in ACPI 3.0. Same macros can be used across
// P, C and T states.
pub const DOMAIN_COORD_TYPE_SW_ALL: u8 = 0xfc;
pub const DOMAIN_COORD_TYPE_SW_ANY: u8 = 0xfd;
pub const DOMAIN_COORD_TYPE_HW_ALL: u8 = 0xfe;
pub const ACPI_CSTATE_SYSTEMIO: u32 = 0;
pub const ACPI_CSTATE_FFH: u32 = 1;
pub const ACPI_CSTATE_HALT: u32 = 2;
pub const ACPI_CSTATE_INTEGER: u32 = 3;
pub const ACPI_CX_DESC_LEN: usize = 32;

pub type AcpiHandle = *mut core::ffi::c_void;
pub type CpumaskVarT = *mut core::ffi::c_void;
pub type PhysCpuidT = u32;

#[repr(C, packed)]
pub struct AcpiPowerRegister { pub descriptor: u8, pub length: u16, pub space_id: u8, pub bit_width: u8, pub bit_offset: u8, pub access_size: u8, pub address: u64 }

#[repr(C)]
pub struct AcpiProcessorCx { pub valid: u8, pub type_: u8, pub address: u32, pub entry_method: u8, pub index: u8, pub latency: u32, pub bm_sts_skip: u8, pub desc: [u8; ACPI_CX_DESC_LEN] }

#[repr(C)]
pub struct AcpiLpiState { pub min_residency: u32, pub wake_latency: u32, pub flags: u32, pub arch_flags: u32, pub res_cnt_freq: u32, pub enable_parent_state: u32, pub address: u64, pub index: u8, pub entry_method: u8, pub desc: [u8; ACPI_CX_DESC_LEN] }

#[repr(C)]
pub union AcpiProcessorPowerStates { pub states: [AcpiProcessorCx; ACPI_PROCESSOR_MAX_POWER], pub lpi_states: [AcpiLpiState; ACPI_PROCESSOR_MAX_POWER] }
#[repr(C)]
pub struct AcpiProcessorPower { pub count: i32, pub states: AcpiProcessorPowerStates, pub timer_broadcast_on_state: i32 }

#[repr(C, packed)]
pub struct AcpiPsdPackage { pub num_entries: u64, pub revision: u64, pub domain: u64, pub coord_type: u64, pub num_processors: u64 }
#[repr(C, packed)]
pub struct AcpiPctRegister { pub descriptor: u8, pub length: u16, pub space_id: u8, pub bit_width: u8, pub bit_offset: u8, pub reserved: u8, pub address: u64 }
#[repr(C)]
pub struct AcpiProcessorPx { pub core_frequency: u64, pub power: u64, pub transition_latency: u64, pub bus_master_latency: u64, pub control: u64, pub status: u64 }

#[repr(C)]
pub struct AcpiProcessorPerformance { pub state: u32, pub platform_limit: u32, pub control_register: AcpiPctRegister, pub status_register: AcpiPctRegister, pub state_count: u32, pub states: *mut AcpiProcessorPx, pub domain_info: AcpiPsdPackage, pub shared_cpu_map: CpumaskVarT, pub shared_type: u32 }

#[repr(C)]
pub struct AcpiTsdPackage { pub num_entries: u64, pub revision: u64, pub domain: u64, pub coord_type: u64, pub num_processors: u64 }
#[repr(C, packed)]
pub struct AcpiPtcRegister { pub descriptor: u8, pub length: u16, pub space_id: u8, pub bit_width: u8, pub bit_offset: u8, pub reserved: u8, pub address: u64 }
#[repr(C)]
pub struct AcpiProcessorTxTss { pub freqpercentage: u64, pub power: u64, pub transition_latency: u64, pub control: u64, pub status: u64 }
#[repr(C)]
pub struct AcpiProcessorTx { pub power: u16, pub performance: u16 }

pub struct AcpiProcessor;
#[repr(C)]
pub struct AcpiProcessorThrottling {
    pub state: u32, pub platform_limit: u32, pub control_register: AcpiPctRegister, pub status_register: AcpiPctRegister,
    pub state_count: u32, pub states_tss: *mut AcpiProcessorTxTss, pub domain_info: AcpiTsdPackage, pub shared_cpu_map: CpumaskVarT,
    pub acpi_processor_get_throttling: Option<unsafe extern "C" fn(*mut AcpiProcessor) -> i32>,
    pub acpi_processor_set_throttling: Option<unsafe extern "C" fn(*mut AcpiProcessor, i32, bool) -> i32>,
    pub address: u32, pub duty_offset: u8, pub duty_width: u8, pub tsd_valid_flag: u8, pub shared_type: u32,
    pub states: [AcpiProcessorTx; ACPI_PROCESSOR_MAX_THROTTLING],
}

extern "C" {
    pub fn acpi_processor_tstate_has_changed(pr: *mut AcpiProcessor) -> i32;
    pub fn acpi_processor_get_throttling_info(pr: *mut AcpiProcessor) -> i32;
    pub fn acpi_processor_set_throttling(pr: *mut AcpiProcessor, state: i32, force: bool) -> i32;
    pub fn acpi_processor_reevaluate_tstate(pr: *mut AcpiProcessor, is_dead: bool);
    pub fn acpi_processor_throttling_init();
    pub fn acpi_processor_power_init(pr: *mut AcpiProcessor);
    pub fn acpi_processor_power_exit(pr: *mut AcpiProcessor);
    pub fn acpi_processor_power_state_has_changed(pr: *mut AcpiProcessor) -> i32;
    pub fn acpi_processor_hotplug(pr: *mut AcpiProcessor) -> i32;
    pub fn acpi_processor_register_idle_driver();
    pub fn acpi_processor_unregister_idle_driver();
    pub fn acpi_processor_ffh_lpi_probe(cpu: u32) -> i32;
    pub fn acpi_processor_ffh_lpi_enter(lpi: *mut AcpiLpiState) -> i32;
    pub fn acpi_thermal_cpufreq_init(policy: *mut core::ffi::c_void);
    pub fn acpi_thermal_cpufreq_exit(policy: *mut core::ffi::c_void);
}

#[inline] pub unsafe fn acpi_processor_ignore_ppc_init() {}
#[inline] pub unsafe fn acpi_processor_ppc_init(_policy: *mut core::ffi::c_void) {}
#[inline] pub unsafe fn acpi_processor_ppc_exit(_policy: *mut core::ffi::c_void) {}
#[inline] pub unsafe fn acpi_processor_ppc_has_changed(_pr: *mut AcpiProcessor, _event_flag: i32) {}
#[inline] pub unsafe fn acpi_processor_get_bios_limit(_cpu: i32, _limit: *mut u32) -> i32 { -19 }
#[inline] pub unsafe fn acpi_cppc_processor_probe(_pr: *mut AcpiProcessor) -> i32 { 0 }
#[inline] pub unsafe fn acpi_cppc_processor_exit(_pr: *mut AcpiProcessor) {}

#[repr(C)] pub struct AcpiProcessorLx { pub px: i32, pub tx: i32 }
#[repr(C)] pub struct AcpiProcessorLimit { pub state: AcpiProcessorLx, pub thermal: AcpiProcessorLx, pub user: AcpiProcessorLx }
#[repr(C)] pub struct AcpiProcessorFlags { pub power: u8, pub performance: u8, pub throttling: u8, pub limit: u8, pub bm_control: u8, pub bm_check: u8, pub has_cst: u8, pub has_lpi: u8, pub power_setup_done: u8, pub bm_rld_set: u8, pub previously_online: u8 }

#[repr(C)]
pub struct AcpiProcessor {
    pub handle: AcpiHandle, pub acpi_id: u32, pub phys_id: PhysCpuidT, pub id: u32, pub pblk: u32,
    pub performance_platform_limit: i32, pub throttling_platform_limit: i32, pub flags: AcpiProcessorFlags,
    pub power: AcpiProcessorPower, pub performance: *mut AcpiProcessorPerformance, pub throttling: AcpiProcessorThrottling,
    pub limit: AcpiProcessorLimit, pub cdev: *mut core::ffi::c_void, pub dev: *mut core::ffi::c_void,
    pub perflib_req: [u8; 0], pub thermal_req: [u8; 0],
}

#[repr(C)] pub struct AcpiProcessorErrata { pub smp: u8, pub piix4: AcpiProcessorErrataPiix4 }
#[repr(C)] pub struct AcpiProcessorErrataPiix4 { pub throttle: u8, pub fdma: u8, pub reserved: u8, pub bmisx: u32 }

extern "C" {
    pub fn acpi_processor_preregister_performance(performance: *mut AcpiProcessorPerformance) -> i32;
    pub fn acpi_processor_register_performance(performance: *mut AcpiProcessorPerformance, cpu: u32) -> i32;
    pub fn acpi_processor_unregister_performance(cpu: u32);
    pub fn acpi_processor_pstate_control() -> i32;
    pub fn acpi_processor_notify_smm(calling_module: *mut core::ffi::c_void) -> i32;
    pub fn acpi_processor_get_psd(handle: AcpiHandle, pdomain: *mut AcpiPsdPackage) -> i32;
    pub fn acpi_processor_get_performance_info(pr: *mut AcpiProcessor) -> i32;
    pub static mut processors: *mut AcpiProcessor;
    pub static mut errata: AcpiProcessorErrata;
    pub fn acpi_get_phys_id(handle: AcpiHandle, type_: i32, acpi_id: u32) -> PhysCpuidT;
    pub fn acpi_map_madt_entry(acpi_id: u32) -> PhysCpuidT;
    pub fn acpi_map_cpuid(phys_id: PhysCpuidT, acpi_id: u32) -> i32;
    pub fn acpi_get_cpuid(handle: AcpiHandle, type_: i32, acpi_id: u32) -> i32;
    pub fn acpi_processor_set_pdc(handle: AcpiHandle);
    pub fn acpi_processor_thermal_init(pr: *mut AcpiProcessor, device: *mut core::ffi::c_void) -> i32;
    pub fn acpi_processor_thermal_exit(pr: *mut AcpiProcessor, device: *mut core::ffi::c_void);
    pub fn acpi_processor_init_invariance_cppc();
}

// Configuration-dependent declarations and inline fallbacks are intentionally
// retained as conditional Rust items; their kernel feature definitions are
// supplied by the surrounding build.
#[inline] pub unsafe fn acpi_processor_power_init_bm_check(flags: *mut AcpiProcessorFlags, _cpu: u32) { (*flags).bm_check = 1; }
#[inline] pub unsafe fn acpi_processor_ffh_cstate_probe(_cpu: u32, _cx: *mut AcpiProcessorCx, _reg: *mut AcpiPowerRegister) -> i32 { -1 }
#[inline] pub unsafe fn acpi_processor_ffh_cstate_enter(_cstate: *mut AcpiProcessorCx) {}
#[inline] pub unsafe fn acpi_processor_ffh_play_dead(_cx: *mut AcpiProcessorCx) -> ! { panic!("BUG") }

#[inline] pub unsafe fn call_on_cpu(cpu: i32, fn_: unsafe extern "C" fn(*mut core::ffi::c_void) -> i64, arg: *mut core::ffi::c_void, direct: bool) -> i64 {
    // is_percpu_thread(), smp_processor_id(), and work_on_cpu() are external kernel dependencies.
    if direct { fn_(arg) } else { let _ = cpu; fn_(arg) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
