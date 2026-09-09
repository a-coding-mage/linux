// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015 Carlo Caione <carlo@endlessm.com>
 * Copyright (C) 2017 Martin Blumenstingl <martin.blumenstingl@googlemail.com>
 */

// Dependencies are supplied by the surrounding kernel translation.

const MESON_SMP_SRAM_CPU_CTRL_REG: usize = 0x00;
const MESON_SMP_SRAM_CPU_CTRL_ADDR_REG: usize = 0x04;
const MESON_CPU_AO_RTI_PWR_A9_CNTL0: u32 = 0x00;
const MESON_CPU_AO_RTI_PWR_A9_CNTL1: u32 = 0x04;
const MESON_CPU_AO_RTI_PWR_A9_MEM_PD0: u32 = 0x14;

static mut sram_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut scu_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut pmu: *mut regmap = core::ptr::null_mut();

#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct reset_control { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct smp_operations {
    pub smp_prepare_cpus: Option<unsafe extern "C" fn(u32)>,
    pub smp_boot_secondary: Option<unsafe extern "C" fn(u32, *mut task_struct) -> i32>,
    #[cfg(CONFIG_HOTPLUG_CPU)] pub cpu_die: Option<unsafe extern "C" fn(u32)>,
    #[cfg(CONFIG_HOTPLUG_CPU)] pub cpu_kill: Option<unsafe extern "C" fn(u32) -> i32>,
}

extern "C" {
    fn of_get_cpu_node(cpu: i32, thread: i32) -> *mut device_node;
    fn of_reset_control_get_exclusive(np: *mut device_node, id: *const i8) -> *mut reset_control;
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn of_find_compatible_node(from: *mut device_node, ty: *const i8, compat: *const i8) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn of_node_put(node: *mut device_node);
    fn syscon_regmap_lookup_by_compatible(compat: *const i8) -> *mut regmap;
    fn scu_enable(base: *mut core::ffi::c_void);
    fn __pa_symbol(symbol: unsafe extern "C" fn());
    fn secondary_startup();
    fn scu_cpu_power_enable(base: *mut core::ffi::c_void, cpu: u32);
    fn reset_control_assert(rstc: *mut reset_control) -> i32;
    fn reset_control_deassert(rstc: *mut reset_control) -> i32;
    fn reset_control_put(rstc: *mut reset_control);
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn regmap_read_poll_timeout(map: *mut regmap, reg: u32, val: *mut u32, cond: bool, delay: u32, timeout: u32) -> i32;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn udelay(usecs: u32);
    fn usleep_range(min: u32, max: u32);
    fn msleep(msecs: u32);
    fn jiffies() -> u64;
    fn time_before(a: u64, b: u64) -> bool;
    fn v7_exit_coherency_flush(louis: u32);
    fn scu_power_mode(base: *mut core::ffi::c_void, mode: u32);
    fn scu_get_cpu_power_mode(base: *mut core::ffi::c_void, cpu: u32) -> i32;
    fn dsb();
    fn wfi();
}

const MESON_CPU_PWR_A9_CNTL0_M: fn(u32) -> u32 = |c| 0x03u32 << ((c * 2) + 16);
const MESON_CPU_PWR_A9_CNTL1_M: fn(u32) -> u32 = |c| 0x03u32 << ((c + 1) << 1);
const MESON_CPU_PWR_A9_MEM_PD0_M: fn(u32) -> u32 = |c| 0x0fu32 << (32 - (c * 4));
const MESON_CPU_PWR_A9_CNTL1_ST: fn(u32) -> u32 = |c| 0x01u32 << (c + 16);

unsafe fn meson_smp_get_core_reset(cpu: i32) -> *mut reset_control {
    of_reset_control_get_exclusive(of_get_cpu_node(cpu, 0), core::ptr::null())
}

unsafe fn meson_smp_set_cpu_ctrl(cpu: u32, on_off: bool) {
    let addr = (sram_base as *mut u8).add(MESON_SMP_SRAM_CPU_CTRL_REG);
    let mut val = readl(addr);
    if on_off { val |= 1u32 << cpu; } else { val &= !(1u32 << cpu); }
    val |= 1;
    writel(val, addr);
}

unsafe extern "C" fn meson_smp_prepare_cpus(scu_compatible: *const i8, pmu_compatible: *const i8, sram_compatible: *const i8) {
    let mut node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), sram_compatible);
    if node.is_null() { return; }
    sram_base = of_iomap(node, 0); of_node_put(node);
    if sram_base.is_null() { return; }
    pmu = syscon_regmap_lookup_by_compatible(pmu_compatible);
    if pmu.is_null() { return; }
    node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), scu_compatible);
    if node.is_null() { return; }
    scu_base = of_iomap(node, 0); of_node_put(node);
    if scu_base.is_null() { return; }
    scu_enable(scu_base);
}

unsafe extern "C" fn meson8b_smp_prepare_cpus(_: u32) { meson_smp_prepare_cpus(b"arm,cortex-a5-scu\0".as_ptr() as _, b"amlogic,meson8b-pmu\0".as_ptr() as _, b"amlogic,meson8b-smp-sram\0".as_ptr() as _); }
unsafe extern "C" fn meson8_smp_prepare_cpus(_: u32) { meson_smp_prepare_cpus(b"arm,cortex-a9-scu\0".as_ptr() as _, b"amlogic,meson8-pmu\0".as_ptr() as _, b"amlogic,meson8-smp-sram\0".as_ptr() as _); }

unsafe fn meson_smp_begin_secondary_boot(cpu: u32) {
    writel(__pa_symbol(secondary_startup) as u32, (sram_base as *mut u8).add(MESON_SMP_SRAM_CPU_CTRL_ADDR_REG + ((cpu - 1) << 2) as usize));
    scu_cpu_power_enable(scu_base, cpu);
}

unsafe fn meson_smp_finalize_secondary_boot(cpu: u32) -> i32 {
    let addr = (sram_base as *mut u8).add(MESON_SMP_SRAM_CPU_CTRL_ADDR_REG + ((cpu - 1) << 2) as usize);
    while readl(addr) != 0 { }
    writel(__pa_symbol(secondary_startup) as u32, addr); meson_smp_set_cpu_ctrl(cpu, true); 0
}

unsafe extern "C" fn meson8_smp_boot_secondary(cpu: u32, _: *mut task_struct) -> i32 {
    let rstc = meson_smp_get_core_reset(cpu as i32); if rstc.is_null() { return -1; }
    meson_smp_begin_secondary_boot(cpu);
    if reset_control_assert(rstc) != 0 { reset_control_put(rstc); return 0; }
    if regmap_update_bits(pmu, MESON_CPU_AO_RTI_PWR_A9_CNTL1, MESON_CPU_PWR_A9_CNTL1_M(cpu), 0) < 0 { reset_control_put(rstc); return 0; }
    udelay(10);
    if regmap_update_bits(pmu, MESON_CPU_AO_RTI_PWR_A9_CNTL0, 1 << cpu, 0) < 0 { reset_control_put(rstc); return 0; }
    if reset_control_deassert(rstc) != 0 { reset_control_put(rstc); return 0; }
    let _ = meson_smp_finalize_secondary_boot(cpu); reset_control_put(rstc); 0
}

unsafe extern "C" fn meson8b_smp_boot_secondary(cpu: u32, _: *mut task_struct) -> i32 {
    let rstc = meson_smp_get_core_reset(cpu as i32); if rstc.is_null() { return -1; }
    meson_smp_begin_secondary_boot(cpu);
    if regmap_update_bits(pmu, MESON_CPU_AO_RTI_PWR_A9_CNTL0, MESON_CPU_PWR_A9_CNTL0_M(cpu), 0) < 0 { reset_control_put(rstc); return 0; }
    udelay(5);
    if reset_control_assert(rstc) != 0 || regmap_update_bits(pmu, MESON_CPU_AO_RTI_PWR_A9_MEM_PD0, MESON_CPU_PWR_A9_MEM_PD0_M(cpu), 0) < 0 { reset_control_put(rstc); return 0; }
    if regmap_update_bits(pmu, MESON_CPU_AO_RTI_PWR_A9_CNTL1, MESON_CPU_PWR_A9_CNTL1_M(cpu), 0) < 0 { reset_control_put(rstc); return 0; }
    udelay(10); if reset_control_deassert(rstc) != 0 { reset_control_put(rstc); return 0; }
    let _ = meson_smp_finalize_secondary_boot(cpu); reset_control_put(rstc); 0
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe extern "C" fn meson8_smp_cpu_die(cpu: u32) { meson_smp_set_cpu_ctrl(cpu, false); v7_exit_coherency_flush(0); scu_power_mode(scu_base, 0); dsb(); wfi(); }

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe extern "C" fn meson8_smp_cpu_kill(cpu: u32) -> i32 {
    let mut power_mode;
    let mut count = 5000;
    loop { power_mode = scu_get_cpu_power_mode(scu_base, cpu); if power_mode == 0 { break; } udelay(10); count -= 1; if count == 0 { break; } }
    if power_mode != 0 { return -110; }
    msleep(30);
    if regmap_update_bits(pmu, MESON_CPU_AO_RTI_PWR_A9_CNTL0, 1 << cpu, 3) < 0 { return -1; }
    udelay(10);
    if regmap_update_bits(pmu, MESON_CPU_AO_RTI_PWR_A9_CNTL1, MESON_CPU_PWR_A9_CNTL1_M(cpu), 3) < 0 { return -1; }
    1
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe extern "C" fn meson8b_smp_cpu_kill(cpu: u32) -> i32 {
    let mut power_mode;
    let mut count = 5000;
    loop { power_mode = scu_get_cpu_power_mode(scu_base, cpu); if power_mode == 0 { break; } udelay(10); count -= 1; if count == 0 { break; } }
    if power_mode != 0 { return -110; }
    udelay(10);
    if regmap_update_bits(pmu, MESON_CPU_AO_RTI_PWR_A9_CNTL0, MESON_CPU_PWR_A9_CNTL0_M(cpu), 3) < 0 { return -1; }
    if regmap_update_bits(pmu, MESON_CPU_AO_RTI_PWR_A9_CNTL0, 1 << cpu, 3) < 0 { return -1; }
    udelay(10);
    if regmap_update_bits(pmu, MESON_CPU_AO_RTI_PWR_A9_CNTL1, MESON_CPU_PWR_A9_CNTL1_M(cpu), 3) < 0 { return -1; }
    if regmap_update_bits(pmu, MESON_CPU_AO_RTI_PWR_A9_MEM_PD0, MESON_CPU_PWR_A9_MEM_PD0_M(cpu), 0xf) < 0 { return -1; }
    1
}

static mut meson8_smp_ops: smp_operations = smp_operations { smp_prepare_cpus: Some(meson8_smp_prepare_cpus), smp_boot_secondary: Some(meson8_smp_boot_secondary), #[cfg(CONFIG_HOTPLUG_CPU)] cpu_die: Some(meson8_smp_cpu_die), #[cfg(CONFIG_HOTPLUG_CPU)] cpu_kill: Some(meson8_smp_cpu_kill) };
static mut meson8b_smp_ops: smp_operations = smp_operations { smp_prepare_cpus: Some(meson8b_smp_prepare_cpus), smp_boot_secondary: Some(meson8b_smp_boot_secondary), #[cfg(CONFIG_HOTPLUG_CPU)] cpu_die: Some(meson8_smp_cpu_die), #[cfg(CONFIG_HOTPLUG_CPU)] cpu_kill: Some(meson8b_smp_cpu_kill) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
