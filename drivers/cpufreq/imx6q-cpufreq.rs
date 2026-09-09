// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2013 Freescale Semiconductor, Inc. */

// Linux kernel headers and symbols are supplied by the surrounding repository.

const PU_SOC_VOLTAGE_NORMAL: u32 = 1250000;
const PU_SOC_VOLTAGE_HIGH: u32 = 1275000;
const FREQ_1P2_GHZ: u32 = 1200000000;

#[repr(C)] pub struct regulator { _p: [u8; 0] }
#[repr(C)] pub struct clk { _p: [u8; 0] }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { _p: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct dev_pm_opp { _p: [u8; 0] }
#[repr(C)] pub struct cpufreq_policy { pub clk: *mut clk, pub suspend_freq: u32 }
#[repr(C)] pub struct cpufreq_frequency_table { pub frequency: u32 }
#[repr(C)] pub struct property { pub length: u32, pub value: *const u32 }
#[repr(C)] pub struct regmap { _p: [u8; 0] }
#[repr(C)] pub struct cpufreq_driver { _p: [u8; 0] }
#[repr(C)] pub struct clk_bulk_data { pub clk: *mut clk, pub id: *const u8 }

extern "C" {
    fn get_cpu_device(cpu: u32) -> *mut device;
    fn of_node_get(n: *mut device_node) -> *mut device_node;
    fn of_node_put(n: *mut device_node);
    fn of_machine_is_compatible(s: *const u8) -> bool;
    fn clk_bulk_get(d: *mut device, n: i32, c: *mut clk_bulk_data) -> i32;
    fn clk_bulk_put(n: i32, c: *mut clk_bulk_data);
    fn clk_get_rate(c: *mut clk) -> u64;
    fn clk_set_rate(c: *mut clk, r: u64) -> i32;
    fn clk_set_parent(c: *mut clk, p: *mut clk) -> i32;
    fn clk_prepare_enable(c: *mut clk) -> i32;
    fn clk_disable_unprepare(c: *mut clk);
    fn regulator_get(d: *mut device, n: *const u8) -> *mut regulator;
    fn regulator_get_optional(d: *mut device, n: *const u8) -> *mut regulator;
    fn regulator_put(r: *mut regulator);
    fn regulator_get_voltage(r: *mut regulator) -> i64;
    fn regulator_set_voltage_tol(r: *mut regulator, v: u32, t: u32) -> i32;
    fn regulator_set_voltage_time(r: *mut regulator, a: u32, b: u32) -> i32;
    fn dev_pm_opp_find_freq_ceil(d: *mut device, f: *mut u64) -> *mut dev_pm_opp;
    fn dev_pm_opp_find_freq_exact(d: *mut device, f: u64, available: bool) -> *mut dev_pm_opp;
    fn dev_pm_opp_get_voltage(o: *mut dev_pm_opp) -> u32;
    fn dev_pm_opp_put(o: *mut dev_pm_opp);
    fn dev_pm_opp_disable(d: *mut device, f: u64) -> i32;
    fn dev_pm_opp_of_add_table(d: *mut device) -> i32;
    fn dev_pm_opp_of_remove_table(d: *mut device);
    fn dev_pm_opp_get_opp_count(d: *mut device) -> i32;
    fn dev_pm_opp_init_cpufreq_table(d: *mut device, t: *mut *mut cpufreq_frequency_table) -> i32;
    fn dev_pm_opp_free_cpufreq_table(d: *mut device, t: *mut *mut cpufreq_frequency_table);
    fn cpufreq_generic_init(p: *mut cpufreq_policy, t: *mut cpufreq_frequency_table, l: u32);
    fn cpufreq_register_driver(d: *mut cpufreq_driver) -> i32;
    fn cpufreq_unregister_driver(d: *mut cpufreq_driver);
    fn cpufreq_generic_frequency_table_verify(p: *mut cpufreq_policy) -> i32;
    fn cpufreq_generic_get(p: *mut cpufreq_policy) -> u32;
    fn cpufreq_generic_suspend(p: *mut cpufreq_policy) -> i32;
    fn cpufreq_register_em_with_opp(p: *mut cpufreq_policy) -> i32;
    fn nvmem_cell_read_u32(d: *mut device, n: *const u8, v: *mut u32) -> i32;
    fn syscon_regmap_lookup_by_compatible(s: *const u8) -> *mut regmap;
    fn regmap_read(r: *mut regmap, o: u32, v: *mut u32) -> i32;
    fn of_property_present(n: *mut device_node, s: *const u8) -> bool;
    fn of_find_property(n: *mut device_node, s: *const u8, l: *mut u32) -> *const property;
    fn of_property_read_u32(n: *mut device_node, s: *const u8, v: *mut u32) -> i32;
    fn devm_kcalloc(d: *mut device, n: usize, z: usize, g: u32) -> *mut u32;
}

static mut arm_reg: *mut regulator = core::ptr::null_mut();
static mut pu_reg: *mut regulator = core::ptr::null_mut();
static mut soc_reg: *mut regulator = core::ptr::null_mut();
#[repr(C)] enum IMX6_CPUFREQ_CLKS { ARM, PLL1_SYS, STEP, PLL1_SW, PLL2_PFD2_396M, PLL2_BUS, SECONDARY_SEL }
const IMX6Q_CPUFREQ_CLK_NUM: i32 = 5;
const IMX6UL_CPUFREQ_CLK_NUM: i32 = 7;
static mut num_clks: i32 = 0;
static mut clks: [clk_bulk_data; 7] = [
    clk_bulk_data { clk: core::ptr::null_mut(), id: b"arm\0".as_ptr() }, clk_bulk_data { clk: core::ptr::null_mut(), id: b"pll1_sys\0".as_ptr() },
    clk_bulk_data { clk: core::ptr::null_mut(), id: b"step\0".as_ptr() }, clk_bulk_data { clk: core::ptr::null_mut(), id: b"pll1_sw\0".as_ptr() },
    clk_bulk_data { clk: core::ptr::null_mut(), id: b"pll2_pfd2_396m\0".as_ptr() }, clk_bulk_data { clk: core::ptr::null_mut(), id: b"pll2_bus\0".as_ptr() },
    clk_bulk_data { clk: core::ptr::null_mut(), id: b"secondary_sel\0".as_ptr() }];
static mut cpu_dev: *mut device = core::ptr::null_mut();
static mut freq_table: *mut cpufreq_frequency_table = core::ptr::null_mut();
static mut max_freq: u32 = 0;
static mut transition_latency: u32 = 0;
static mut imx6_soc_volt: *mut u32 = core::ptr::null_mut();

unsafe fn imx6q_set_target(policy: *mut cpufreq_policy, index: u32) -> i32 {
    let new_freq = (*freq_table.add(index as usize)).frequency; let mut freq_hz = (new_freq as u64) * 1000;
    let old_freq = (clk_get_rate(clks[ARM as usize].clk) / 1000) as u32;
    let opp = dev_pm_opp_find_freq_ceil(cpu_dev, &mut freq_hz); if opp.is_null() { return -2; }
    let volt = dev_pm_opp_get_voltage(opp); dev_pm_opp_put(opp); let volt_old = regulator_get_voltage(arm_reg) as u32;
    if new_freq > old_freq {
        if !pu_reg.is_null() { let r = regulator_set_voltage_tol(pu_reg, *imx6_soc_volt.add(index as usize), 0); if r != 0 { return r; } }
        let r = regulator_set_voltage_tol(soc_reg, *imx6_soc_volt.add(index as usize), 0); if r != 0 { return r; }
        let r = regulator_set_voltage_tol(arm_reg, volt, 0); if r != 0 { return r; }
    }
    let ul = of_machine_is_compatible(b"fsl,imx6ul\0".as_ptr()) || of_machine_is_compatible(b"fsl,imx6ull\0".as_ptr());
    if ul {
        clk_set_rate(clks[ARM as usize].clk, ((old_freq >> 1) * 1000) as u64); clk_set_parent(clks[PLL1_SW as usize].clk, clks[PLL1_SYS as usize].clk);
        let p = if freq_hz > clk_get_rate(clks[PLL2_PFD2_396M as usize].clk) { clks[PLL2_BUS as usize].clk } else { clks[PLL2_PFD2_396M as usize].clk };
        clk_set_parent(clks[SECONDARY_SEL as usize].clk, p); clk_set_parent(clks[STEP as usize].clk, clks[SECONDARY_SEL as usize].clk); clk_set_parent(clks[PLL1_SW as usize].clk, clks[STEP as usize].clk);
        if freq_hz > clk_get_rate(clks[PLL2_BUS as usize].clk) { clk_set_rate(clks[PLL1_SYS as usize].clk, (new_freq as u64)*1000); clk_set_parent(clks[PLL1_SW as usize].clk, clks[PLL1_SYS as usize].clk); }
    } else { clk_set_parent(clks[STEP as usize].clk, clks[PLL2_PFD2_396M as usize].clk); clk_set_parent(clks[PLL1_SW as usize].clk, clks[STEP as usize].clk); if freq_hz > clk_get_rate(clks[PLL2_PFD2_396M as usize].clk) { clk_set_rate(clks[PLL1_SYS as usize].clk, (new_freq as u64)*1000); clk_set_parent(clks[PLL1_SW as usize].clk, clks[PLL1_SYS as usize].clk); } else { clk_prepare_enable(clks[PLL1_SYS as usize].clk); } }
    let ret = clk_set_rate(clks[ARM as usize].clk, (new_freq as u64)*1000); if ret != 0 { regulator_set_voltage_tol(arm_reg, volt_old, 0); return ret; }
    if new_freq < old_freq { regulator_set_voltage_tol(arm_reg, volt, 0); regulator_set_voltage_tol(soc_reg, *imx6_soc_volt.add(index as usize), 0); if !pu_reg.is_null() { regulator_set_voltage_tol(pu_reg, *imx6_soc_volt.add(index as usize), 0); } }
    0
}

unsafe fn imx6q_cpufreq_init(policy: *mut cpufreq_policy) -> i32 { (*policy).clk = clks[ARM as usize].clk; cpufreq_generic_init(policy, freq_table, transition_latency); (*policy).suspend_freq = max_freq; 0 }
unsafe fn imx6x_disable_freq_in_opp(dev: *mut device, freq: u64) { let _ = dev_pm_opp_disable(dev, freq); }
const OCOTP_CFG3_SPEED_SHIFT: u32 = 16; const OCOTP_CFG3_SPEED_1P2GHZ: u32 = 3; const OCOTP_CFG3_SPEED_996MHZ: u32 = 2; const OCOTP_CFG3_SPEED_852MHZ: u32 = 1;
unsafe fn imx6q_opp_check_speed_grading(dev: *mut device) -> i32 { let mut val=0; let mut ret=0; if of_property_present((*dev).of_node,b"nvmem-cells\0".as_ptr()) { ret=nvmem_cell_read_u32(dev,b"speed_grade\0".as_ptr(),&mut val); } else { let o=syscon_regmap_lookup_by_compatible(b"fsl,imx6q-ocotp\0".as_ptr()); if o.is_null(){return -2;} regmap_read(o,0x440,&mut val); } if ret!=0{return ret;} val=(val>>OCOTP_CFG3_SPEED_SHIFT)&3; if val<2{imx6x_disable_freq_in_opp(dev,996000000)} if of_machine_is_compatible(b"fsl,imx6q\0".as_ptr())||of_machine_is_compatible(b"fsl,imx6qp\0".as_ptr()){if val!=1{imx6x_disable_freq_in_opp(dev,852000000)} if val!=3{imx6x_disable_freq_in_opp(dev,1200000000)}} 0 }
unsafe fn imx6ul_opp_check_speed_grading(dev: *mut device) -> i32 { let mut val=0; let mut ret=0; if of_property_present((*dev).of_node,b"nvmem-cells\0".as_ptr()){ret=nvmem_cell_read_u32(dev,b"speed_grade\0".as_ptr(),&mut val)} else {let o=syscon_regmap_lookup_by_compatible(b"fsl,imx6ul-ocotp\0".as_ptr()); if o.is_null(){return -2} regmap_read(o,0x440,&mut val)} if ret!=0{return ret} val=(val>>16)&3; if of_machine_is_compatible(b"fsl,imx6ul\0".as_ptr())&&val!=2{imx6x_disable_freq_in_opp(dev,696000000)} if of_machine_is_compatible(b"fsl,imx6ull\0".as_ptr()){if val<2{imx6x_disable_freq_in_opp(dev,792000000)} if val!=3{imx6x_disable_freq_in_opp(dev,900000000)}} ret }

unsafe fn imx6q_cpufreq_probe(pdev: *mut platform_device) -> i32 {
    cpu_dev = get_cpu_device(0); if cpu_dev.is_null() { return -19; }
    let np = of_node_get((*cpu_dev).of_node); if np.is_null() { return -2; }
    num_clks = if of_machine_is_compatible(b"fsl,imx6ul\0".as_ptr()) || of_machine_is_compatible(b"fsl,imx6ull\0".as_ptr()) { IMX6UL_CPUFREQ_CLK_NUM } else { IMX6Q_CPUFREQ_CLK_NUM };
    let mut ret = clk_bulk_get(cpu_dev, num_clks, clks.as_mut_ptr()); if ret != 0 { of_node_put(np); return ret; }
    arm_reg=regulator_get(cpu_dev,b"arm\0".as_ptr()); pu_reg=regulator_get_optional(cpu_dev,b"pu\0".as_ptr()); soc_reg=regulator_get(cpu_dev,b"soc\0".as_ptr());
    ret=dev_pm_opp_of_add_table(cpu_dev); if ret<0 { clk_bulk_put(num_clks,clks.as_mut_ptr()); of_node_put(np); return ret; }
    ret=if of_machine_is_compatible(b"fsl,imx6ul\0".as_ptr())||of_machine_is_compatible(b"fsl,imx6ull\0".as_ptr()){imx6ul_opp_check_speed_grading(cpu_dev)}else{imx6q_opp_check_speed_grading(cpu_dev)}; if ret!=0{dev_pm_opp_of_remove_table(cpu_dev); clk_bulk_put(num_clks,clks.as_mut_ptr()); of_node_put(np); return ret;}
    let num=dev_pm_opp_get_opp_count(cpu_dev); if num<0{return num;}
    ret=dev_pm_opp_init_cpufreq_table(cpu_dev,&mut freq_table); if ret!=0{return ret;}
    imx6_soc_volt=devm_kcalloc(&mut (*pdev).dev,num as usize,core::mem::size_of::<u32>(),0); if imx6_soc_volt.is_null(){return -12;}
    for i in 0..num as usize {*imx6_soc_volt.add(i)=PU_SOC_VOLTAGE_NORMAL;} if (*freq_table.add(num as usize-1)).frequency*1000==FREQ_1P2_GHZ{*imx6_soc_volt.add(num as usize-1)=PU_SOC_VOLTAGE_HIGH;}
    max_freq=(*freq_table.add(num as usize-1)).frequency; cpufreq_generic_init(core::ptr::null_mut(),freq_table,transition_latency); of_node_put(np); 0
}
unsafe fn imx6q_cpufreq_remove(_pdev: *mut platform_device) { cpufreq_unregister_driver(core::ptr::null_mut()); imx6_soc_volt=core::ptr::null_mut(); dev_pm_opp_free_cpufreq_table(cpu_dev,&mut freq_table); dev_pm_opp_of_remove_table(cpu_dev); regulator_put(arm_reg); if !pu_reg.is_null(){regulator_put(pu_reg)} regulator_put(soc_reg); clk_bulk_put(num_clks,clks.as_mut_ptr()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
