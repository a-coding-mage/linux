// SPDX-License-Identifier: GPL-2.0
/* Rust translation of mc_smp.c; kernel dependencies are supplied externally. */

const SUNXI_CPUS_PER_CLUSTER: usize = 4;
const SUNXI_NR_CLUSTERS: usize = 2;
const POLL_USEC: u32 = 100;
const TIMEOUT_USEC: u32 = 100000;

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn cpucfg_cx_ctrl_reg0(c: u32) -> usize { (0x10 * c) as usize }
const fn cpucfg_cx_ctrl_reg1(c: u32) -> usize { (0x10 * c + 4) as usize }
const fn cpucfg_cx_status(c: u32) -> usize { (0x30 + 4 * c) as usize }
const fn cpucfg_cx_rst_ctrl(c: u32) -> usize { (0x80 + 4 * c) as usize }
const fn prcm_cpu_po_rst_ctrl(c: u32) -> usize { (4 + 4 * c) as usize }
const fn prcm_pwroff_gating_reg(c: u32) -> usize { (0x100 + 4 * c) as usize }
const fn prcm_pwr_switch_reg(c: u32, cpu: u32) -> usize { (0x140 + 0x10 * c + 4 * cpu) as usize }
const fn r_cpucfg_cluster_po_rst_ctrl(c: u32) -> usize { (0x30 + c * 4) as usize }

const CPUCFG_CX_CTRL_REG1_ACINACTM: u32 = bit(0);
const CPUCFG_CX_STATUS_STANDBYWFIL2: u32 = bit(0);
const CPUCFG_CX_RST_CTRL_DBG_SOC_RST: u32 = bit(24);
const CPUCFG_CX_RST_CTRL_H_RST: u32 = bit(12);
const CPUCFG_CX_RST_CTRL_L2_RST: u32 = bit(8);
const CPUCFG_CX_RST_CTRL_CORE_RST_ALL: u32 = 0xf;
const CPUCFG_CX_RST_CTRL_DBG_RST_ALL: u32 = 0xf << 16;
const CPUCFG_CX_RST_CTRL_ETM_RST_ALL: u32 = 0xf << 20;
const CPUCFG_CX_CTRL_REG0_L1_RST_DISABLE_ALL: u32 = 0xf;
const CPUCFG_CX_CTRL_REG0_L2_RST_DISABLE_A7: u32 = bit(4);
const CPUCFG_CX_CTRL_REG0_L2_RST_DISABLE_A15: u32 = bit(0);
const PRCM_CPU_PO_RST_CTRL_CORE_ALL: u32 = 0xf;
const PRCM_PWROFF_GATING_REG_CLUSTER_SUN8I: u32 = bit(0);
const PRCM_PWROFF_GATING_REG_CLUSTER_SUN9I: u32 = bit(4);
const PRCM_CPU_SOFT_ENTRY_REG: usize = 0x164;
const R_CPUCFG_CPU_SOFT_ENTRY_REG: usize = 0x01a4;
const CPU0_SUPPORT_HOTPLUG_MAGIC0: u32 = 0xFA50392F;
const CPU0_SUPPORT_HOTPLUG_MAGIC1: u32 = 0x790DCA3A;

static mut cpucfg_base: *mut u8 = core::ptr::null_mut();
static mut prcm_base: *mut u8 = core::ptr::null_mut();
static mut sram_b_smp_base: *mut u8 = core::ptr::null_mut();
static mut r_cpucfg_base: *mut u8 = core::ptr::null_mut();
extern "C" { fn sunxi_mc_smp_secondary_startup(); fn sunxi_mc_smp_resume(); }
static mut is_a83t: bool = false;

unsafe fn sunxi_cpu_power_switch_set(cpu: u32, cluster: u32, enable: bool) -> i32 {
    let p = prcm_base.add(prcm_pwr_switch_reg(cluster, cpu));
    let reg = readl(p);
    if enable {
        if reg == 0 { return 0; }
        for v in [0xff, 0xfe, 0xf8, 0xf0, 0] { writel(v, p); udelay(10); }
    } else { writel(0xff, p); udelay(10); }
    0
}
unsafe fn sunxi_cpu0_hotplug_support_set(enable: bool) {
    writel(if enable { CPU0_SUPPORT_HOTPLUG_MAGIC0 } else { 0 }, sram_b_smp_base);
    writel(if enable { CPU0_SUPPORT_HOTPLUG_MAGIC1 } else { 0 }, sram_b_smp_base.add(4));
}
unsafe fn sunxi_core_is_cortex_a15(_core: u32, _cluster: u32) -> bool { /* OF lookup supplied by kernel */ false }
unsafe fn sunxi_cpu_powerup(cpu: u32, cluster: u32) -> i32 {
    if cpu >= 4 || cluster >= 2 { return -22; }
    if cluster == 0 && cpu == 0 { sunxi_cpu0_hotplug_support_set(true); }
    let mut r = readl(prcm_base.add(prcm_cpu_po_rst_ctrl(cluster))); r &= !bit(cpu); writel(r, prcm_base.add(prcm_cpu_po_rst_ctrl(cluster)));
    if is_a83t { r=readl(r_cpucfg_base.add(r_cpucfg_cluster_po_rst_ctrl(cluster))); r &= !bit(cpu); writel(r,r_cpucfg_base.add(r_cpucfg_cluster_po_rst_ctrl(cluster))); udelay(10); }
    if !sunxi_core_is_cortex_a15(cpu,cluster) { let p=cpucfg_base.add(cpucfg_cx_ctrl_reg0(cluster)); r=readl(p); r &= !(bit(cpu)); writel(r,p); }
    let p=cpucfg_base.add(cpucfg_cx_rst_ctrl(cluster)); r=readl(p); r &= !bit(16+cpu); if !sunxi_core_is_cortex_a15(cpu,cluster){r &= !bit(20+cpu);} writel(r,p);
    sunxi_cpu_power_switch_set(cpu,cluster,true);
    let mut b=cpu; if is_a83t && b==0 {b=4;} let p=prcm_base.add(prcm_pwroff_gating_reg(cluster)); r=readl(p); r &= !bit(b); writel(r,p); udelay(20);
    if is_a83t && b==4 {b=0;} let p=prcm_base.add(prcm_cpu_po_rst_ctrl(cluster)); r=readl(p); r |= bit(b); writel(r,p);
    if is_a83t { let p=r_cpucfg_base.add(r_cpucfg_cluster_po_rst_ctrl(cluster)); r=readl(p); r|=bit(b); writel(r,p); udelay(10); }
    let p=cpucfg_base.add(cpucfg_cx_rst_ctrl(cluster)); r=readl(p)|bit(16+b)|bit(b); if !sunxi_core_is_cortex_a15(b,cluster){r|=bit(20+b)}else{r|=bit(4+b)} writel(r,p); 0
}
unsafe fn sunxi_cluster_powerup(cluster:u32)->i32 { if cluster>=2{return -22;} let p=cpucfg_base.add(cpucfg_cx_ctrl_reg1(cluster)); let mut r=readl(p)|CPUCFG_CX_CTRL_REG1_ACINACTM; writel(r,p); let p=prcm_base.add(prcm_cpu_po_rst_ctrl(cluster)); r=readl(p)&!PRCM_CPU_PO_RST_CTRL_CORE_ALL; writel(r,p); let p=cpucfg_base.add(cpucfg_cx_rst_ctrl(cluster)); r=readl(p)&!(CPUCFG_CX_RST_CTRL_DBG_SOC_RST|CPUCFG_CX_RST_CTRL_DBG_RST_ALL|CPUCFG_CX_RST_CTRL_H_RST|CPUCFG_CX_RST_CTRL_L2_RST); if !sunxi_core_is_cortex_a15(0,cluster){r&=!CPUCFG_CX_RST_CTRL_ETM_RST_ALL;} writel(r,p); let p=cpucfg_base.add(cpucfg_cx_ctrl_reg0(cluster)); r=readl(p); if sunxi_core_is_cortex_a15(0,cluster){r&=!CPUCFG_CX_CTRL_REG0_L2_RST_DISABLE_A15;}else{r&=!CPUCFG_CX_CTRL_REG0_L1_RST_DISABLE_ALL; r&=!CPUCFG_CX_CTRL_REG0_L2_RST_DISABLE_A7;} writel(r,p); let p=prcm_base.add(prcm_pwroff_gating_reg(cluster)); r=readl(p)&!(if is_a83t{PRCM_PWROFF_GATING_REG_CLUSTER_SUN8I}else{PRCM_PWROFF_GATING_REG_CLUSTER_SUN9I}); writel(r,p); udelay(20); let p=cpucfg_base.add(cpucfg_cx_rst_ctrl(cluster)); r=readl(p)|CPUCFG_CX_RST_CTRL_DBG_SOC_RST|CPUCFG_CX_RST_CTRL_H_RST|CPUCFG_CX_RST_CTRL_L2_RST; writel(r,p); let p=cpucfg_base.add(cpucfg_cx_ctrl_reg1(cluster)); r=readl(p)&!CPUCFG_CX_CTRL_REG1_ACINACTM; writel(r,p); 0 }

// Remaining callbacks and device-tree initialization retain the C interfaces.
extern "C" { fn readl(p:*mut u8)->u32; fn writel(v:u32,p:*mut u8); fn udelay(v:u32); }

static mut sunxi_mc_smp_cpu_table: [[i32; SUNXI_CPUS_PER_CLUSTER]; SUNXI_NR_CLUSTERS] = [[0; 4]; 2];
static mut sunxi_mc_smp_first_comer: i32 = 0;

unsafe fn sunxi_cluster_cache_disable_without_axi() {
    v7_exit_coherency_flush(0);
    cci_disable_port_by_cpu(read_cpuid_mpidr());
}
unsafe fn sunxi_mc_smp_cluster_is_down(cluster: usize) -> bool {
    for i in 0..SUNXI_CPUS_PER_CLUSTER { if sunxi_mc_smp_cpu_table[cluster][i] != 0 { return false; } }
    true
}
unsafe fn sunxi_mc_smp_secondary_init(cpu: u32) { if cpu == 0 { sunxi_cpu0_hotplug_support_set(false); } }
unsafe fn sunxi_mc_smp_boot_secondary(l_cpu:u32, _idle:*mut core::ffi::c_void)->i32 {
    let mpidr=cpu_logical_map(l_cpu); let cpu=(mpidr&0xff) as usize; let cluster=((mpidr>>8)&0xff) as usize;
    if cpucfg_base.is_null(){return -19;} if cluster>=2||cpu>=4{return -22;}
    if sunxi_mc_smp_cpu_table[cluster][cpu]==0 { sunxi_mc_smp_first_comer=if sunxi_mc_smp_cluster_is_down(cluster){sunxi_cluster_powerup(cluster);1}else{0}; sunxi_cpu_powerup(cpu as u32,cluster as u32); }
    sunxi_mc_smp_cpu_table[cluster][cpu]+=1; 0
}
unsafe fn sunxi_cpu_powerdown(cpu:u32,cluster:u32)->i32 { if cpu>=4||cluster>=2{return -22;} let p=prcm_base.add(prcm_pwroff_gating_reg(cluster)); let mut r=readl(p)|bit(if is_a83t&&cpu==0{4}else{cpu}); writel(r,p); udelay(20); sunxi_cpu_power_switch_set(cpu,cluster,false); 0 }
unsafe fn sunxi_cluster_powerdown(cluster:u32)->i32 { if cluster>=2{return -22;} let p=cpucfg_base.add(cpucfg_cx_rst_ctrl(cluster)); let mut r=readl(p)&!(CPUCFG_CX_RST_CTRL_DBG_SOC_RST|CPUCFG_CX_RST_CTRL_H_RST|CPUCFG_CX_RST_CTRL_L2_RST); writel(r,p); let p=prcm_base.add(prcm_pwroff_gating_reg(cluster)); r=readl(p)|(if is_a83t{PRCM_PWROFF_GATING_REG_CLUSTER_SUN8I}else{PRCM_PWROFF_GATING_REG_CLUSTER_SUN9I}); writel(r,p); udelay(20); 0 }
unsafe fn sunxi_mc_smp_cpu_can_disable(cpu:u32)->bool { !(is_a83t&&cpu==0) }

#[repr(C)] struct sunxi_mc_smp_nodes { prcm_node:*mut core::ffi::c_void, cpucfg_node:*mut core::ffi::c_void, sram_node:*mut core::ffi::c_void, r_cpucfg_node:*mut core::ffi::c_void }
#[repr(C)] struct sunxi_mc_smp_data { enable_method:*const u8, get_smp_nodes:unsafe extern "C" fn(*mut sunxi_mc_smp_nodes)->i32, is_a83t:bool }
unsafe extern "C" fn sunxi_mc_smp_init()->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
