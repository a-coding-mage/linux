// SPDX-License-Identifier: GPL-2.0-only
/*
 * Versatile Express Serial Power Controller (SPC) support
 *
 * Copyright (C) 2013 ARM Ltd.
 *
 * Authors: Sudeep KarkadaNagesha <sudeep.karkadanagesha@arm.com>
 *          Achin Gupta           <achin.gupta@arm.com>
 *          Lorenzo Pieralisi     <lorenzo.pieralisi@arm.com>
 */

const SPCLOG: &str = "vexpress-spc: ";
const PERF_LVL_A15: usize = 0x00;
const PERF_REQ_A15: usize = 0x04;
const PERF_LVL_A7: usize = 0x08;
const PERF_REQ_A7: usize = 0x0c;
const COMMS: usize = 0x10;
const COMMS_REQ: usize = 0x14;
const PWC_STATUS: usize = 0x18;
const PWC_FLAG: usize = 0x1c;
const WAKE_INT_MASK: usize = 0x24;
const WAKE_INT_RAW: usize = 0x28;
const WAKE_INT_STAT: usize = 0x2c;
const A15_PWRDN_EN: usize = 0x30;
const A7_PWRDN_EN: usize = 0x34;
const A15_BX_ADDR0: usize = 0x68;
const A7_BX_ADDR0: usize = 0x78;
const STANDBYWFI_STAT: usize = 0x3c;
const SYSCFG_WDATA: usize = 0x70;
const SYSCFG_RDATA: usize = 0x74;
const A15_PERFVAL_BASE: u32 = 0xC10;
const A7_PERFVAL_BASE: u32 = 0xC30;
const SYSCFG_START: u32 = 1 << 31;
const SYSCFG_SCC: u32 = 6 << 20;
const SYSCFG_STAT: u32 = 14 << 20;
const GBL_WAKEUP_INT_MSK: u32 = 0x3 << 10;
const MAX_CLUSTERS: usize = 2;
const TIMEOUT_US: u32 = 20000;
const MAX_OPPS: usize = 8;
const CA15_DVFS: usize = 0;
const CA7_DVFS: usize = 1;
const SPC_SYS_CFG: usize = 2;
const MULT_FACTOR: u32 = 20;
const VOLT_SHIFT: u32 = 20;
const FREQ_MASK: u32 = 0xFFFFF;

#[repr(C)]
pub struct ve_spc_opp { pub freq: u64, pub u_volt: u64 }

#[repr(C)]
pub struct ve_spc_drvdata {
    pub baseaddr: *mut u8,
    pub a15_clusid: u32,
    pub cur_rsp_mask: u32,
    pub cur_rsp_stat: u32,
    pub sem: semaphore,
    pub done: completion,
    pub opps: [*mut ve_spc_opp; MAX_CLUSTERS],
    pub num_opps: [i32; MAX_CLUSTERS],
}

#[repr(C)] pub struct semaphore { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
pub type irqreturn_t = i32;
static mut info: *mut ve_spc_drvdata = core::ptr::null_mut();

extern "C" {
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn writel(value: u32, addr: *mut u8);
    fn wait_for_completion_interruptible_timeout(c: *mut completion, timeout: u32) -> i32;
    fn usecs_to_jiffies(usecs: u32) -> u32;
    fn init_completion(c: *mut completion);
    fn complete(c: *mut completion);
    fn down_timeout(s: *mut semaphore, timeout: u32) -> i32;
    fn up(s: *mut semaphore);
    fn sema_init(s: *mut semaphore, value: i32);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t, flags: u32, name: *const u8, data: *mut core::ffi::c_void) -> i32;
    fn topology_physical_package_id(id: i32) -> i32;
    fn dev_pm_opp_add(dev: *mut device, freq: u64, volt: u64) -> i32;
    fn sync_cache_w(ptr: *mut core::ffi::c_void);
}

const fn stat_complete(t: usize) -> u32 { 1 << (t << 2) }
const fn stat_err(t: usize) -> u32 { 1 << ((t << 2) + 1) }
const fn response_mask(t: usize) -> u32 { stat_complete(t) | stat_err(t) }
unsafe fn reg(base: *mut u8, off: usize) -> *mut u8 { base.add(off) }
unsafe fn cluster_is_a15(cluster: u32) -> bool { cluster == (*info).a15_clusid }

pub unsafe fn ve_spc_global_wakeup_irq(set: bool) {
    let mut r = readl_relaxed(reg((*info).baseaddr, WAKE_INT_MASK));
    if set { r |= GBL_WAKEUP_INT_MSK; } else { r &= !GBL_WAKEUP_INT_MSK; }
    writel_relaxed(r, reg((*info).baseaddr, WAKE_INT_MASK));
}

pub unsafe fn ve_spc_cpu_wakeup_irq(cluster: u32, cpu: u32, set: bool) {
    if cluster >= MAX_CLUSTERS as u32 { return; }
    let mut mask = 1u32 << cpu;
    if !cluster_is_a15(cluster) { mask <<= 4; }
    let mut r = readl_relaxed(reg((*info).baseaddr, WAKE_INT_MASK));
    if set { r |= mask; } else { r &= !mask; }
    writel_relaxed(r, reg((*info).baseaddr, WAKE_INT_MASK));
}

pub unsafe fn ve_spc_set_resume_addr(cluster: u32, cpu: u32, addr: u32) {
    if cluster >= MAX_CLUSTERS as u32 { return; }
    let off = if cluster_is_a15(cluster) { A15_BX_ADDR0 } else { A7_BX_ADDR0 };
    writel_relaxed(addr, reg((*info).baseaddr, off + (cpu << 2) as usize));
}

pub unsafe fn ve_spc_powerdown(cluster: u32, enable: bool) {
    if cluster >= MAX_CLUSTERS as u32 { return; }
    let off = if cluster_is_a15(cluster) { A15_PWRDN_EN } else { A7_PWRDN_EN };
    writel_relaxed(enable as u32, reg((*info).baseaddr, off));
}

unsafe fn standbywfi_cpu_mask(cpu: u32, cluster: u32) -> u32 {
    if cluster_is_a15(cluster) { 1 << cpu } else { 1 << (3 + cpu) }
}

pub unsafe fn ve_spc_cpu_in_wfi(cpu: u32, cluster: u32) -> i32 {
    let mask = standbywfi_cpu_mask(cpu, cluster);
    if cluster >= MAX_CLUSTERS as u32 { return 1; }
    (readl_relaxed(reg((*info).baseaddr, STANDBYWFI_STAT)) & mask) as i32
}

unsafe fn ve_spc_get_performance(cluster: usize, freq: *mut u32) -> i32 {
    let off = if cluster_is_a15(cluster as u32) { PERF_LVL_A15 } else { PERF_LVL_A7 };
    let perf = readl_relaxed(reg((*info).baseaddr, off));
    if perf >= (*info).num_opps[cluster] as u32 { return -22; }
    *freq = (*(*info).opps[cluster].add(perf as usize)).freq as u32;
    0
}

unsafe fn ve_spc_round_performance(cluster: usize, mut freq: u32) -> i32 {
    let max_opp = (*info).num_opps[cluster];
    let opps = (*info).opps[cluster];
    let mut fmin = 0u32; let mut fmax = !0u32;
    freq /= 1000;
    for idx in 0..max_opp as usize {
        let f = (*opps.add(idx)).freq as u32;
        if f >= freq { if f <= fmax { fmax = f; } } else if f >= fmin { fmin = f; }
    }
    if fmax != !0u32 { (fmax * 1000) as i32 } else { (fmin * 1000) as i32 }
}

unsafe fn ve_spc_find_performance_index(cluster: usize, freq: u32) -> i32 {
    let max = (*info).num_opps[cluster] as usize;
    for idx in 0..max { if (*(*info).opps[cluster].add(idx)).freq as u32 == freq { return idx as i32; } }
    -22
}

unsafe fn ve_spc_waitforcompletion(req_type: usize) -> i32 {
    let mut ret = wait_for_completion_interruptible_timeout(&mut (*info).done, usecs_to_jiffies(TIMEOUT_US));
    if ret == 0 { ret = -110; } else if ret > 0 { ret = if (*info).cur_rsp_stat & stat_complete(req_type) != 0 { 0 } else { -5 }; }
    ret
}

unsafe fn ve_spc_set_performance(cluster: usize, freq: u32) -> i32 {
    let (req_type, off) = if cluster_is_a15(cluster as u32) { (CA15_DVFS, PERF_LVL_A15) } else { (CA7_DVFS, PERF_LVL_A7) };
    let perf = ve_spc_find_performance_index(cluster, freq);
    if perf < 0 { return perf; }
    if down_timeout(&mut (*info).sem, usecs_to_jiffies(TIMEOUT_US)) != 0 { return -62; }
    init_completion(&mut (*info).done); (*info).cur_rsp_mask = response_mask(req_type);
    writel(perf as u32, reg((*info).baseaddr, off));
    let ret = ve_spc_waitforcompletion(req_type);
    (*info).cur_rsp_mask = 0; up(&mut (*info).sem); ret
}

unsafe fn ve_spc_read_sys_cfg(func: u32, offset: u32, data: *mut u32) -> i32 {
    if down_timeout(&mut (*info).sem, usecs_to_jiffies(TIMEOUT_US)) != 0 { return -62; }
    init_completion(&mut (*info).done); (*info).cur_rsp_mask = response_mask(SPC_SYS_CFG);
    writel(SYSCFG_START | func | (offset >> 2), reg((*info).baseaddr, COMMS));
    let ret = ve_spc_waitforcompletion(SPC_SYS_CFG);
    if ret == 0 { *data = readl_relaxed(reg((*info).baseaddr, SYSCFG_RDATA)); }
    (*info).cur_rsp_mask = 0; up(&mut (*info).sem); ret
}

pub unsafe extern "C" fn ve_spc_irq_handler(_irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let drv = data as *mut ve_spc_drvdata;
    let status = readl_relaxed(reg((*drv).baseaddr, PWC_STATUS));
    if (*info).cur_rsp_mask & status != 0 { (*info).cur_rsp_stat = status; complete(&mut (*drv).done); }
    1
}

unsafe fn ve_spc_populate_opps(cluster: u32) -> i32 {
    let opps = libc::calloc(MAX_OPPS, core::mem::size_of::<ve_spc_opp>()) as *mut ve_spc_opp;
    if opps.is_null() { return -12; }
    (*info).opps[cluster as usize] = opps;
    let mut off = if cluster_is_a15(cluster) { A15_PERFVAL_BASE } else { A7_PERFVAL_BASE };
    let mut data = 0u32; let mut idx = 0usize; let mut ret = 0;
    while idx < MAX_OPPS {
        ret = ve_spc_read_sys_cfg(SYSCFG_SCC, off, &mut data);
        if ret != 0 { break; }
        (*opps.add(idx)).freq = ((data & FREQ_MASK) * MULT_FACTOR) as u64;
        (*opps.add(idx)).u_volt = ((data >> VOLT_SHIFT) * 1000) as u64;
        idx += 1; off += 4;
    }
    (*info).num_opps[cluster as usize] = idx as i32; ret
}

pub unsafe fn ve_spc_init(baseaddr: *mut u8, a15_clusid: u32, irq: i32) -> i32 {
    let p = libc::calloc(1, core::mem::size_of::<ve_spc_drvdata>()) as *mut ve_spc_drvdata;
    if p.is_null() { return -12; }
    info = p; (*info).baseaddr = baseaddr; (*info).a15_clusid = a15_clusid;
    if irq <= 0 { libc::free(info as *mut _); return -22; }
    init_completion(&mut (*info).done); readl_relaxed(reg(baseaddr, PWC_STATUS));
    let ret = request_irq(irq, ve_spc_irq_handler, 1 << 2, b"vexpress-spc\0".as_ptr(), info as *mut _);
    if ret != 0 { libc::free(info as *mut _); return -19; }
    sema_init(&mut (*info).sem, 1); sync_cache_w(info as *mut _); sync_cache_w(&mut info as *mut _ as *mut _); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
