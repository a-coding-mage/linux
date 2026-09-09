// SPDX-License-Identifier: GPL-2.0-only
/*
 * Broadcom STB CPU SMP and hotplug support for ARM
 *
 * Copyright (C) 2013-2014 Broadcom Corporation
 */

// Linux and ARM dependencies are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct DeviceNode { _private: [u8; 0] }
#[repr(C)]
pub struct TaskStruct { _private: [u8; 0] }

#[repr(C)]
pub struct SmpOperations {
    pub smp_prepare_cpus: Option<unsafe extern "C" fn(max_cpus: u32)>,
    pub smp_boot_secondary: Option<unsafe extern "C" fn(cpu: u32, idle: *mut TaskStruct) -> i32>,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub cpu_kill: Option<unsafe extern "C" fn(cpu: u32) -> i32>,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub cpu_die: Option<unsafe extern "C" fn(cpu: u32)>,
}

const ZONE_MAN_CLKEN_MASK: u32 = 1 << 0;
const ZONE_MAN_RESET_CNTL_MASK: u32 = 1 << 1;
const ZONE_MAN_MEM_PWR_MASK: u32 = 1 << 4;
const ZONE_RESERVED_1_MASK: u32 = 1 << 5;
const ZONE_MAN_ISO_CNTL_MASK: u32 = 1 << 6;
const ZONE_MANUAL_CONTROL_MASK: u32 = 1 << 7;
const ZONE_PWR_DN_REQ_MASK: u32 = 1 << 9;
const ZONE_PWR_UP_REQ_MASK: u32 = 1 << 10;
const ZONE_BLK_RST_ASSERT_MASK: u32 = 1 << 12;
const ZONE_PWR_OFF_STATE_MASK: u32 = 1 << 25;
const ZONE_PWR_ON_STATE_MASK: u32 = 1 << 26;
const ZONE_DPG_PWR_STATE_MASK: u32 = 1 << 28;
const ZONE_MEM_PWR_STATE_MASK: u32 = 1 << 29;
const ZONE_RESET_STATE_MASK: u32 = 1 << 31;
const CPU0_PWR_ZONE_CTRL_REG: u32 = 1;
const CPU_RESET_CONFIG_REG: u32 = 2;

static mut CPUBIUCTRL_BLOCK: *mut u8 = core::ptr::null_mut();
static mut HIF_CONT_BLOCK: *mut u8 = core::ptr::null_mut();
static mut CPU0_PWR_ZONE_CTRL_REG_VALUE: u32 = 0;
static mut CPU_RST_CFG_REG: u32 = 0;
static mut HIF_CONT_REG: u32 = 0;

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
static mut PER_CPU_SW_STATE: i32 = 0;

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn per_cpu_sw_state_rd(cpu: u32) -> i32 {
    sync_cache_r(&raw mut PER_CPU_SW_STATE, cpu);
    per_cpu(PER_CPU_SW_STATE, cpu)
}

unsafe fn per_cpu_sw_state_wr(cpu: u32, val: i32) {
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    {
        dmb();
        per_cpu_set(PER_CPU_SW_STATE, cpu, val);
        sync_cache_w(&raw mut PER_CPU_SW_STATE, cpu);
    }
    #[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
    { let _ = (cpu, val); }
}

unsafe fn pwr_ctrl_get_base(cpu: u32) -> *mut u8 {
    let mut base = CPUBIUCTRL_BLOCK.add(CPU0_PWR_ZONE_CTRL_REG_VALUE as usize);
    base = base.add((cpu_logical_map(cpu) * 4) as usize);
    base
}

unsafe fn pwr_ctrl_rd(cpu: u32) -> u32 { readl_relaxed(pwr_ctrl_get_base(cpu)) }

unsafe fn pwr_ctrl_set(cpu: u32, val: u32, mask: u32) {
    let base = pwr_ctrl_get_base(cpu);
    writel((readl(base) & mask) | val, base);
}

unsafe fn pwr_ctrl_clr(cpu: u32, val: u32, mask: u32) {
    let base = pwr_ctrl_get_base(cpu);
    writel((readl(base) & mask) & !val, base);
}

const POLL_TMOUT_MS: u32 = 500;
unsafe fn pwr_ctrl_wait_tmout(cpu: u32, set: u32, mask: u32) -> i32 {
    let timeo = jiffies().wrapping_add(msecs_to_jiffies(POLL_TMOUT_MS));
    let mut tmp;
    loop {
        tmp = pwr_ctrl_rd(cpu) & mask;
        if (set == 0) == (tmp == 0) { return 0; }
        if !time_before(jiffies(), timeo) { break; }
    }
    tmp = pwr_ctrl_rd(cpu) & mask;
    if (set == 0) == (tmp == 0) { return 0; }
    -ETIMEDOUT
}

unsafe fn cpu_rst_cfg_set(cpu: u32, set: i32) {
    let mut val = readl_relaxed(CPUBIUCTRL_BLOCK.add(CPU_RST_CFG_REG as usize));
    if set != 0 { val |= 1 << cpu_logical_map(cpu); }
    else { val &= !(1 << cpu_logical_map(cpu)); }
    writel_relaxed(val, CPUBIUCTRL_BLOCK.add(CPU_RST_CFG_REG as usize));
}

unsafe fn cpu_set_boot_addr(cpu: u32, boot_addr: usize) {
    let reg_ofs = (cpu_logical_map(cpu) * 8) as usize;
    writel_relaxed(0, HIF_CONT_BLOCK.add((HIF_CONT_REG as usize) + reg_ofs));
    writel_relaxed(boot_addr as u32, HIF_CONT_BLOCK.add((HIF_CONT_REG as usize) + 4 + reg_ofs));
}

unsafe fn brcmstb_cpu_boot(cpu: u32) {
    per_cpu_sw_state_wr(cpu, 1);
    cpu_set_boot_addr(cpu, __pa_symbol(secondary_startup));
    cpu_rst_cfg_set(cpu, 0);
}

unsafe fn brcmstb_cpu_power_on(cpu: u32) {
    pwr_ctrl_set(cpu, ZONE_MAN_ISO_CNTL_MASK, 0xffffff00);
    pwr_ctrl_set(cpu, ZONE_MANUAL_CONTROL_MASK, u32::MAX);
    pwr_ctrl_set(cpu, ZONE_RESERVED_1_MASK, u32::MAX);
    pwr_ctrl_set(cpu, ZONE_MAN_MEM_PWR_MASK, u32::MAX);
    if pwr_ctrl_wait_tmout(cpu, 1, ZONE_MEM_PWR_STATE_MASK) != 0 { panic!("ZONE_MEM_PWR_STATE_MASK set timeout"); }
    pwr_ctrl_set(cpu, ZONE_MAN_CLKEN_MASK, u32::MAX);
    if pwr_ctrl_wait_tmout(cpu, 1, ZONE_DPG_PWR_STATE_MASK) != 0 { panic!("ZONE_DPG_PWR_STATE_MASK set timeout"); }
    pwr_ctrl_clr(cpu, ZONE_MAN_ISO_CNTL_MASK, u32::MAX);
    pwr_ctrl_set(cpu, ZONE_MAN_RESET_CNTL_MASK, u32::MAX);
}

unsafe fn brcmstb_cpu_get_power_state(cpu: u32) -> i32 {
    if (pwr_ctrl_rd(cpu) & ZONE_RESET_STATE_MASK) != 0 { 0 } else { 1 }
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn brcmstb_cpu_die(cpu: u32) {
    v7_exit_coherency_flush(all);
    per_cpu_sw_state_wr(cpu, 0);
    wfi();
    loop { core::hint::spin_loop(); }
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn brcmstb_cpu_kill(cpu: u32) -> i32 {
    if cpu == 0 { pr_warn!("SMP: refusing to power off CPU0\n"); return 1; }
    while per_cpu_sw_state_rd(cpu) != 0 { core::hint::spin_loop(); }
    pwr_ctrl_set(cpu, ZONE_MANUAL_CONTROL_MASK, u32::MAX);
    pwr_ctrl_clr(cpu, ZONE_MAN_RESET_CNTL_MASK, u32::MAX);
    pwr_ctrl_clr(cpu, ZONE_MAN_CLKEN_MASK, u32::MAX);
    pwr_ctrl_set(cpu, ZONE_MAN_ISO_CNTL_MASK, u32::MAX);
    pwr_ctrl_clr(cpu, ZONE_MAN_MEM_PWR_MASK, u32::MAX);
    if pwr_ctrl_wait_tmout(cpu, 0, ZONE_MEM_PWR_STATE_MASK) != 0 { panic!("ZONE_MEM_PWR_STATE_MASK clear timeout"); }
    pwr_ctrl_clr(cpu, ZONE_RESERVED_1_MASK, u32::MAX);
    if pwr_ctrl_wait_tmout(cpu, 0, ZONE_DPG_PWR_STATE_MASK) != 0 { panic!("ZONE_DPG_PWR_STATE_MASK clear timeout"); }
    mb();
    cpu_rst_cfg_set(cpu, 1);
    1
}

// Device-tree setup and SMP registration retain their C interfaces through the surrounding kernel bindings.
unsafe fn setup_hifcpubiuctrl_regs(np: *mut DeviceNode) -> i32 { setup_hifcpubiuctrl_regs_impl(np) }
unsafe fn setup_hifcont_regs(np: *mut DeviceNode) -> i32 { setup_hifcont_regs_impl(np) }

unsafe fn brcmstb_cpu_ctrl_setup(max_cpus: u32) {
    let _ = max_cpus;
    let name = "brcm,brcmstb-smpboot";
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), name);
    if np.is_null() { pr_err!("can't find compatible node %s\n", name); return; }
    let rc = setup_hifcpubiuctrl_regs(np);
    if rc == 0 { let _ = setup_hifcont_regs(np); }
    of_node_put(np);
}

unsafe fn brcmstb_boot_secondary(cpu: u32, _idle: *mut TaskStruct) -> i32 {
    if CPUBIUCTRL_BLOCK.is_null() || HIF_CONT_BLOCK.is_null() { return -ENODEV; }
    if brcmstb_cpu_get_power_state(cpu) == 0 { brcmstb_cpu_power_on(cpu); }
    brcmstb_cpu_boot(cpu);
    0
}

// CONFIG_HOTPLUG_CPU conditionally supplies cpu_kill and cpu_die fields.
#[used]
static BRCMSTB_SMP_OPS: SmpOperations = SmpOperations {
    smp_prepare_cpus: Some(brcmstb_cpu_ctrl_setup),
    smp_boot_secondary: Some(brcmstb_boot_secondary),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_kill: Some(brcmstb_cpu_kill),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_die: Some(brcmstb_cpu_die),
};

// CPU_METHOD_OF_DECLARE(brcmstb_smp, "brcm,brahma-b15", &brcmstb_smp_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
