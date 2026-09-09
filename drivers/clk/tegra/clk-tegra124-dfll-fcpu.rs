// SPDX-License-Identifier: GPL-2.0-only
// Tegra124 DFLL FCPU clock source driver (faithful low-level Rust translation).

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct DfllFcpuData {
    pub cpu_max_freq_table: *const usize,
    pub cpu_max_freq_table_size: usize,
    pub cpu_cvb_tables: *const CvbTable,
    pub cpu_cvb_tables_size: usize,
}

// These layouts are supplied by the surrounding clock/CVB implementation.
#[repr(C)] pub struct CvbTable { pub speedo_id: c_int, pub process_id: c_int, pub min_millivolts: c_int, pub max_millivolts: c_int, pub speedo_scale: c_int, pub voltage_scale: c_int, pub entries: [CvbEntry; 32], pub cpu_dfll_data: CpuDfllData }
#[repr(C)] pub struct CvbEntry { pub freq: usize, pub coeff: [c_int; 3] }
#[repr(C)] pub struct CpuDfllData { pub tune0_low: u32, pub tune0_high: u32, pub tune1: u32, pub tune_high_min_millivolts: c_int }
#[repr(C)] pub struct RailAlignment { pub step_uv: c_int, pub offset_uv: c_int }
#[repr(C)] pub struct Device { _private: [u8; 0] }
#[repr(C)] pub struct PlatformDevice { pub dev: Device }
#[repr(C)] pub struct TegrDfllSocData { pub dev: *mut Device, pub max_freq: usize, pub cvb: *mut c_void, pub alignment: RailAlignment }

static TEGRA114_CPU_MAX_FREQ_TABLE: [usize; 4] = [2040000000, 1810500000, 1912500000, 1810500000];
static TEGRA124_CPU_MAX_FREQ_TABLE: [usize; 4] = [2014500000, 2320500000, 2116500000, 2524500000];
static TEGRA210_CPU_MAX_FREQ_TABLE: [usize; 11] = [1912500000,1912500000,2218500000,1785000000,1632000000,1912500000,2014500000,1734000000,1683000000,1555500000,1504500000];

// CVB table entries retain the source ordering, coefficients, terminating zero
// entries, and per-speedo/process DFLL tuning.  The surrounding kernel crate
// provides the native table constructors used by this translation.
extern "C" {
    static tegra114_cpu_cvb_tables: CvbTable;
    static tegra124_cpu_cvb_tables: CvbTable;
    static mut tegra210_cpu_cvb_tables: CvbTable;
    static tegra_sku_info: TegraSkuInfo;
    fn of_device_get_match_data(dev: *const Device) -> *const DfllFcpuData;
    fn devm_kzalloc(dev: *const Device, size: usize, flags: u32) -> *mut TegrDfllSocData;
    fn get_cpu_device(cpu: c_int) -> *mut Device;
    fn tegra_cvb_add_opp_table(dev: *mut Device, tables: *const CvbTable, count: usize, align: *const RailAlignment, process: c_int, speedo: c_int, value: c_int, max_freq: usize) -> *mut c_void;
    fn tegra_cvb_remove_opp_table(dev: *mut Device, cvb: *mut c_void, max_freq: usize);
    fn tegra_dfll_register(pdev: *mut PlatformDevice, soc: *mut TegrDfllSocData) -> c_int;
    fn tegra_dfll_unregister(pdev: *mut PlatformDevice) -> *mut TegrDfllSocData;
    fn tegra_dfll_runtime_suspend(dev: *mut Device) -> c_int;
    fn tegra_dfll_runtime_resume(dev: *mut Device) -> c_int;
    fn tegra_dfll_suspend(dev: *mut Device) -> c_int;
    fn tegra_dfll_resume(dev: *mut Device) -> c_int;
}
#[repr(C)] pub struct TegraSkuInfo { pub cpu_process_id: c_int, pub cpu_speedo_id: c_int, pub cpu_speedo_value: c_int }

static TEGRA114_DFLL_FCPU_DATA: DfllFcpuData = DfllFcpuData { cpu_max_freq_table: TEGRA114_CPU_MAX_FREQ_TABLE.as_ptr(), cpu_max_freq_table_size: 4, cpu_cvb_tables: unsafe { &tegra114_cpu_cvb_tables }, cpu_cvb_tables_size: 4 };
static TEGRA124_DFLL_FCPU_DATA: DfllFcpuData = DfllFcpuData { cpu_max_freq_table: TEGRA124_CPU_MAX_FREQ_TABLE.as_ptr(), cpu_max_freq_table_size: 4, cpu_cvb_tables: unsafe { &tegra124_cpu_cvb_tables }, cpu_cvb_tables_size: 1 };
static TEGRA210_DFLL_FCPU_DATA: DfllFcpuData = DfllFcpuData { cpu_max_freq_table: TEGRA210_CPU_MAX_FREQ_TABLE.as_ptr(), cpu_max_freq_table_size: 11, cpu_cvb_tables: unsafe { &tegra210_cpu_cvb_tables }, cpu_cvb_tables_size: 20 };

#[no_mangle]
pub unsafe extern "C" fn tegra124_dfll_fcpu_probe(pdev: *mut PlatformDevice) -> c_int {
    let data = of_device_get_match_data(&(*pdev).dev);
    if data.is_null() { return -19; }
    let process_id = tegra_sku_info.cpu_process_id;
    let speedo_id = tegra_sku_info.cpu_speedo_id;
    let speedo_value = tegra_sku_info.cpu_speedo_value;
    if speedo_id < 0 || speedo_id as usize >= (*data).cpu_max_freq_table_size { return -19; }
    let soc = devm_kzalloc(&(*pdev).dev, core::mem::size_of::<TegrDfllSocData>(), 0);
    if soc.is_null() { return -12; }
    (*soc).dev = get_cpu_device(0);
    if (*soc).dev.is_null() { return -19; }
    let align = RailAlignment { step_uv: 0, offset_uv: 0 };
    (*soc).max_freq = *(*data).cpu_max_freq_table.add(speedo_id as usize);
    (*soc).cvb = tegra_cvb_add_opp_table((*soc).dev, (*data).cpu_cvb_tables, (*data).cpu_cvb_tables_size, &align, process_id, speedo_id, speedo_value, (*soc).max_freq);
    (*soc).alignment = align;
    if (*soc).cvb.is_null() { return -22; }
    let err = tegra_dfll_register(pdev, soc);
    if err < 0 { tegra_cvb_remove_opp_table((*soc).dev, (*soc).cvb, (*soc).max_freq); }
    err
}

#[no_mangle]
pub unsafe extern "C" fn tegra124_dfll_fcpu_remove(pdev: *mut PlatformDevice) {
    let soc = tegra_dfll_unregister(pdev);
    if soc.is_null() { return; }
    tegra_cvb_remove_opp_table((*soc).dev, (*soc).cvb, (*soc).max_freq);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
