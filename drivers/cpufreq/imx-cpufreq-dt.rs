// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2019 NXP
 */

// Translated from the Linux kernel implementation. External kernel types and
// functions are supplied by the surrounding build environment.

const OCOTP_CFG3_SPEED_GRADE_SHIFT: u32 = 8;
const OCOTP_CFG3_SPEED_GRADE_MASK: u32 = 0x3 << 8;
const IMX8MN_OCOTP_CFG3_SPEED_GRADE_MASK: u32 = 0xf << 8;
const OCOTP_CFG3_MKT_SEGMENT_SHIFT: u32 = 6;
const OCOTP_CFG3_MKT_SEGMENT_MASK: u32 = 0x3 << 6;
const IMX8MP_OCOTP_CFG3_MKT_SEGMENT_SHIFT: u32 = 5;
const IMX8MP_OCOTP_CFG3_MKT_SEGMENT_MASK: u32 = 0x3 << 5;

const IMX7ULP_MAX_RUN_FREQ: u32 = 528000;

static mut cpufreq_dt_pdev: *mut platform_device = core::ptr::null_mut();
static mut cpu_dev: *mut device = core::ptr::null_mut();
static mut cpufreq_opp_token: i32 = 0;

#[repr(usize)]
enum IMX7ULP_CPUFREQ_CLKS {
    ARM,
    CORE,
    SCS_SEL,
    HSRUN_CORE,
    HSRUN_SCS_SEL,
    FIRC,
}

static mut imx7ulp_clks: [clk_bulk_data; 6] = [
    clk_bulk_data { id: "arm", clk: core::ptr::null_mut() },
    clk_bulk_data { id: "core", clk: core::ptr::null_mut() },
    clk_bulk_data { id: "scs_sel", clk: core::ptr::null_mut() },
    clk_bulk_data { id: "hsrun_core", clk: core::ptr::null_mut() },
    clk_bulk_data { id: "hsrun_scs_sel", clk: core::ptr::null_mut() },
    clk_bulk_data { id: "firc", clk: core::ptr::null_mut() },
];

unsafe extern "C" {
    fn clk_get_rate(clk: *mut clk) -> u32;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> i32;
    fn clk_bulk_get(dev: *mut device, num_clks: usize, clks: *mut clk_bulk_data) -> i32;
    fn clk_bulk_put(num_clks: usize, clks: *mut clk_bulk_data);
    fn get_cpu_device(cpu: u32) -> *mut device;
    fn of_property_present(node: *mut device_node, name: *const u8) -> bool;
    fn of_machine_is_compatible(name: *const u8) -> bool;
    fn nvmem_cell_read_u32(dev: *mut device, name: *const u8, value: *mut u32) -> i32;
    fn platform_device_register_data(parent: *mut device, name: *const u8, id: i32, data: *const core::ffi::c_void, size: usize) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn dev_pm_opp_set_supported_hw(dev: *mut device, hw: *const u32, count: usize) -> i32;
    fn dev_pm_opp_put_supported_hw(token: i32);
    fn ptr_is_err<T>(ptr: *mut T) -> bool;
    fn ptr_err<T>(ptr: *mut T) -> i32;
}

#[repr(C)]
struct clk_bulk_data { id: &'static str, clk: *mut clk }
#[repr(C)] struct clk;
#[repr(C)] struct device { of_node: *mut device_node }
#[repr(C)] struct device_node;
#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct cpufreq_policy { freq_table: *mut cpufreq_frequency_table }
#[repr(C)] struct cpufreq_frequency_table { frequency: u32 }
#[repr(C)] struct cpufreq_dt_platform_data {
    target_intermediate: Option<unsafe extern "C" fn(*mut cpufreq_policy, u32) -> i32>,
    get_intermediate: Option<unsafe extern "C" fn(*mut cpufreq_policy, u32) -> u32>,
}

unsafe extern "C" fn imx7ulp_get_intermediate(_policy: *mut cpufreq_policy, _index: u32) -> u32 {
    clk_get_rate(imx7ulp_clks[IMX7ULP_CPUFREQ_CLKS::FIRC as usize].clk)
}

unsafe extern "C" fn imx7ulp_target_intermediate(policy: *mut cpufreq_policy, index: u32) -> i32 {
    let newfreq = (*(*policy).freq_table.add(index as usize)).frequency;
    clk_set_parent(imx7ulp_clks[IMX7ULP_CPUFREQ_CLKS::SCS_SEL as usize].clk, imx7ulp_clks[IMX7ULP_CPUFREQ_CLKS::FIRC as usize].clk);
    clk_set_parent(imx7ulp_clks[IMX7ULP_CPUFREQ_CLKS::HSRUN_SCS_SEL as usize].clk, imx7ulp_clks[IMX7ULP_CPUFREQ_CLKS::FIRC as usize].clk);
    if newfreq > IMX7ULP_MAX_RUN_FREQ {
        clk_set_parent(imx7ulp_clks[IMX7ULP_CPUFREQ_CLKS::ARM as usize].clk, imx7ulp_clks[IMX7ULP_CPUFREQ_CLKS::HSRUN_CORE as usize].clk);
    } else {
        clk_set_parent(imx7ulp_clks[IMX7ULP_CPUFREQ_CLKS::ARM as usize].clk, imx7ulp_clks[IMX7ULP_CPUFREQ_CLKS::CORE as usize].clk);
    }
    0
}

static mut imx7ulp_data: cpufreq_dt_platform_data = cpufreq_dt_platform_data {
    target_intermediate: Some(imx7ulp_target_intermediate),
    get_intermediate: Some(imx7ulp_get_intermediate),
};

unsafe extern "C" fn imx_cpufreq_dt_probe(pdev: *mut platform_device) -> i32 {
    let mut dt_pdev: *mut platform_device;
    let mut cell_value: u32 = 0;
    let mut supported_hw = [0u32; 2];
    let mut speed_grade: i32;
    let mut mkt_segment: i32;
    let mut ret: i32;
    cpu_dev = get_cpu_device(0);
    if !of_property_present((*cpu_dev).of_node, b"cpu-supply\0".as_ptr()) { return -19; }
    if of_machine_is_compatible(b"fsl,imx7ulp\0".as_ptr()) {
        ret = clk_bulk_get(cpu_dev, imx7ulp_clks.len(), imx7ulp_clks.as_mut_ptr());
        if ret != 0 { return ret; }
        dt_pdev = platform_device_register_data(core::ptr::null_mut(), b"cpufreq-dt\0".as_ptr(), -1, &imx7ulp_data as *const _ as *const _, core::mem::size_of::<cpufreq_dt_platform_data>());
        if ptr_is_err(dt_pdev) { clk_bulk_put(imx7ulp_clks.len(), imx7ulp_clks.as_mut_ptr()); return ptr_err(dt_pdev); }
        cpufreq_dt_pdev = dt_pdev;
        return 0;
    }
    ret = nvmem_cell_read_u32(cpu_dev, b"speed_grade\0".as_ptr(), &mut cell_value);
    if ret != 0 { return ret; }
    if of_machine_is_compatible(b"fsl,imx8mn\0".as_ptr()) || of_machine_is_compatible(b"fsl,imx8mp\0".as_ptr()) { speed_grade = ((cell_value & IMX8MN_OCOTP_CFG3_SPEED_GRADE_MASK) >> OCOTP_CFG3_SPEED_GRADE_SHIFT) as i32; } else { speed_grade = ((cell_value & OCOTP_CFG3_SPEED_GRADE_MASK) >> OCOTP_CFG3_SPEED_GRADE_SHIFT) as i32; }
    if of_machine_is_compatible(b"fsl,imx8mp\0".as_ptr()) { mkt_segment = ((cell_value & IMX8MP_OCOTP_CFG3_MKT_SEGMENT_MASK) >> IMX8MP_OCOTP_CFG3_MKT_SEGMENT_SHIFT) as i32; } else { mkt_segment = ((cell_value & OCOTP_CFG3_MKT_SEGMENT_MASK) >> OCOTP_CFG3_MKT_SEGMENT_SHIFT) as i32; }
    // Early samples without fuses written report "0 0"; clamp to the minimum OPP.
    if mkt_segment == 0 && speed_grade == 0 {
        if of_machine_is_compatible(b"fsl,imx8mm\0".as_ptr()) || of_machine_is_compatible(b"fsl,imx8mq\0".as_ptr()) { speed_grade = 1; }
        if of_machine_is_compatible(b"fsl,imx8mn\0".as_ptr()) || of_machine_is_compatible(b"fsl,imx8mp\0".as_ptr()) { speed_grade = 0xb; }
    }
    supported_hw[0] = 1u32 << speed_grade; supported_hw[1] = 1u32 << mkt_segment;
    cpufreq_opp_token = dev_pm_opp_set_supported_hw(cpu_dev, supported_hw.as_ptr(), 2);
    if cpufreq_opp_token < 0 { return cpufreq_opp_token; }
    cpufreq_dt_pdev = platform_device_register_data(&mut (*pdev).dev, b"cpufreq-dt\0".as_ptr(), -1, core::ptr::null(), 0);
    if ptr_is_err(cpufreq_dt_pdev) { dev_pm_opp_put_supported_hw(cpufreq_opp_token); return ptr_err(cpufreq_dt_pdev); }
    0
}

unsafe extern "C" fn imx_cpufreq_dt_remove(_pdev: *mut platform_device) {
    platform_device_unregister(cpufreq_dt_pdev);
    if !of_machine_is_compatible(b"fsl,imx7ulp\0".as_ptr()) { dev_pm_opp_put_supported_hw(cpufreq_opp_token); } else { clk_bulk_put(imx7ulp_clks.len(), imx7ulp_clks.as_mut_ptr()); }
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: driver,
}
#[repr(C)] struct driver { name: &'static str }

static mut imx_cpufreq_dt_driver: platform_driver = platform_driver {
    probe: Some(imx_cpufreq_dt_probe),
    remove: Some(imx_cpufreq_dt_remove),
    driver: driver { name: "imx-cpufreq-dt" },
};

// Equivalent of module_platform_driver(imx_cpufreq_dt_driver).
// MODULE_ALIAS("platform:imx-cpufreq-dt");
// MODULE_DESCRIPTION("Freescale i.MX cpufreq speed grading driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
