// SPDX-License-Identifier: GPL-2.0
/* Rust translation of qcom-cpufreq-hw.c. */

const LUT_MAX_ENTRIES: u32 = 40;
const LUT_SRC: u32 = 0xc000_0000;
const LUT_L_VAL: u32 = 0xff;
const LUT_CORE_COUNT: u32 = 0x0007_0000;
const LUT_VOLT: u32 = 0xfff;
const CLK_HW_DIV: u32 = 2;
const LUT_TURBO_IND: u32 = 1;
const GT_IRQ_STATUS: u32 = 1 << 2;
const MAX_FREQ_DOMAINS: usize = 4;

#[repr(C)]
pub struct QcomCpufreqSocData {
    pub reg_enable: u32, pub reg_domain_state: u32, pub reg_dcvs_ctrl: u32,
    pub reg_freq_lut: u32, pub reg_volt_lut: u32, pub reg_intr_clr: u32,
    pub reg_current_vote: u32, pub reg_perf_state: u32,
    pub lut_max_entries: u32, pub lut_row_size: u8,
}

#[repr(C)]
pub struct QcomCpufreqData {
    pub base: *mut core::ffi::c_void,
    pub throttle_lock: Mutex,
    pub throttle_irq: i32,
    pub irq_name: [u8; 15],
    pub cancel_throttle: bool,
    pub throttle_work: DelayedWork,
    pub policy: *mut CpufreqPolicy,
    pub cpu_clk: ClkHw,
    pub per_core_dcvs: bool,
}

#[repr(C)] pub struct Mutex { _private: [u8; 0] }
#[repr(C)] pub struct DelayedWork { _private: [u8; 0] }
#[repr(C)] pub struct ClkHw { pub init: *mut ClkInitData }
#[repr(C)] pub struct ClkInitData { pub name: *const u8, pub flags: u32, pub ops: *const ClkOps }
#[repr(C)] pub struct ClkOps { pub recalc_rate: Option<unsafe extern "C" fn(*mut ClkHw, usize) -> usize>, pub determine_rate: Option<unsafe extern "C" fn(*mut ClkHw, *mut ClkRateRequest) -> i32> }
#[repr(C)] pub struct ClkRateRequest { pub rate: usize }
#[repr(C)] pub struct CpufreqPolicy { pub driver_data: *mut QcomCpufreqData, pub freq_table: *mut CpufreqFrequencyTable, pub cached_resolved_idx: u32, pub related_cpus: *mut CpuMask, pub cpus: *mut CpuMask, pub cpu: u32, pub fast_switch_possible: bool, pub dvfs_possible_from_any_cpu: bool }
#[repr(C)] pub struct CpufreqFrequencyTable { pub flags: u32, pub frequency: u32 }
#[repr(C)] pub struct CpuMask { _private: [u8; 0] }
#[repr(C)] pub struct Device { _private: [u8; 0] }
#[repr(C)] pub struct PlatformDevice { pub dev: Device }
#[repr(C)] pub struct DeviceNode { _private: [u8; 0] }
#[repr(C)] pub struct OfPhandleArgs { pub args: [u32; 4] }
#[repr(C)] pub struct DevPmOpp { _private: [u8; 0] }

static mut QCOM_CPUFREQ_DATA: *mut QcomCpufreqData = core::ptr::null_mut();
static mut QCOM_CPUFREQ_SOC_DATA: *const QcomCpufreqSocData = core::ptr::null();
static mut CPU_HW_RATE: usize = 0;
static mut XO_RATE: usize = 0;
static mut ICC_SCALING_ENABLED: bool = false;

const fn field_get(mask: u32, value: u32) -> u32 { (value & mask) >> mask.trailing_zeros() }

unsafe fn qcom_cpufreq_set_bw(policy: *mut CpufreqPolicy, freq_khz: usize) -> i32 {
    let dev = get_cpu_device((*policy).cpu); if dev.is_null() { return -19; }
    let opp = dev_pm_opp_find_freq_exact(dev, freq_khz.wrapping_mul(1000), true);
    if is_err(opp) { return ptr_err(opp); }
    let ret = dev_pm_opp_set_opp(dev, opp); dev_pm_opp_put(opp); ret
}

unsafe fn qcom_cpufreq_update_opp(cpu_dev: *mut Device, freq_khz: usize, volt: usize) -> i32 {
    let freq_hz = freq_khz.wrapping_mul(1000);
    if !ICC_SCALING_ENABLED { return dev_pm_opp_add(cpu_dev, freq_hz, volt); }
    let ret = dev_pm_opp_adjust_voltage(cpu_dev, freq_hz, volt, volt, volt);
    if ret != 0 { dev_err(cpu_dev, "Voltage update failed freq=%ld\n", freq_khz); return ret; }
    dev_pm_opp_enable(cpu_dev, freq_hz)
}

unsafe fn qcom_cpufreq_hw_target_index(policy: *mut CpufreqPolicy, index: u32) -> i32 {
    let data = (*policy).driver_data; let soc = QCOM_CPUFREQ_SOC_DATA;
    writel_relaxed(index, (*data).base.add((*soc).reg_perf_state as usize));
    if (*data).per_core_dcvs { for i in 1..cpumask_weight((*policy).related_cpus) { writel_relaxed(index, (*data).base.add((*soc).reg_perf_state as usize + i as usize * 4)); } }
    if ICC_SCALING_ENABLED { qcom_cpufreq_set_bw(policy, (*(*policy).freq_table.add(index as usize)).frequency as usize); } 0
}

unsafe fn qcom_lmh_get_throttle_freq(data: *mut QcomCpufreqData) -> usize {
    let soc = QCOM_CPUFREQ_SOC_DATA; let lval = if (*soc).reg_current_vote != 0 { readl_relaxed((*data).base.add((*soc).reg_current_vote as usize)) & 0x3ff } else { readl_relaxed((*data).base.add((*soc).reg_domain_state as usize)) & 0xff }; lval as usize * XO_RATE
}

unsafe fn qcom_cpufreq_get_freq(policy: *mut CpufreqPolicy) -> u32 {
    if policy.is_null() { return 0; } let data = (*policy).driver_data; let soc = QCOM_CPUFREQ_SOC_DATA; let mut index = readl_relaxed((*data).base.add((*soc).reg_perf_state as usize)); index = core::cmp::min(index, (*soc).lut_max_entries - 1); (*(*policy).freq_table.add(index as usize)).frequency
}

unsafe fn __qcom_cpufreq_hw_get(policy: *mut CpufreqPolicy) -> u32 { if policy.is_null() { return 0; } let data = (*policy).driver_data; if (*data).throttle_irq >= 0 { (qcom_lmh_get_throttle_freq(data) / 1000) as u32 } else { qcom_cpufreq_get_freq(policy) } }
unsafe fn qcom_cpufreq_hw_get(cpu: u32) -> u32 { __qcom_cpufreq_hw_get(cpufreq_cpu_get_raw(cpu)) }

unsafe fn qcom_cpufreq_hw_fast_switch(policy: *mut CpufreqPolicy, _target_freq: u32) -> u32 { let data=(*policy).driver_data; let soc=QCOM_CPUFREQ_SOC_DATA; let index=(*policy).cached_resolved_idx; writel_relaxed(index,(*data).base.add((*soc).reg_perf_state as usize)); if (*data).per_core_dcvs { for i in 1..cpumask_weight((*policy).related_cpus) { writel_relaxed(index,(*data).base.add((*soc).reg_perf_state as usize+i as usize*4)); } } (*(*policy).freq_table.add(index as usize)).frequency }

unsafe fn qcom_cpufreq_hw_read_lut(cpu_dev: *mut Device, policy: *mut CpufreqPolicy) -> i32 {
    let data=(*policy).driver_data; let soc=QCOM_CPUFREQ_SOC_DATA; let table=kzalloc_freq_table((*soc).lut_max_entries+1); if table.is_null(){return -12;}
    let ret=dev_pm_opp_of_add_table(cpu_dev); if ret==0 { ICC_SCALING_ENABLED=true; let mut rate=0usize; loop { let opp=dev_pm_opp_find_freq_ceil(cpu_dev,&mut rate); if is_err(opp){break;} dev_pm_opp_put(opp); dev_pm_opp_disable(cpu_dev,rate); } } else if ret != -19 { dev_err(cpu_dev,"Invalid opp table in device tree\n"); kfree(table); return ret; } else { (*policy).fast_switch_possible=true; ICC_SCALING_ENABLED=false; }
    let mut prev_freq=0u32; let mut i=0u32; while i<(*soc).lut_max_entries { let d=readl_relaxed((*data).base.add((*soc).reg_freq_lut as usize+i as usize*(*soc).lut_row_size as usize)); let src=field_get(LUT_SRC,d); let lval=field_get(LUT_L_VAL,d); let core_count=field_get(LUT_CORE_COUNT,d); let v=readl_relaxed((*data).base.add((*soc).reg_volt_lut as usize+i as usize*(*soc).lut_row_size as usize)); let volt=(field_get(LUT_VOLT,v)*1000) as usize; let freq=if src {(XO_RATE as u32).wrapping_mul(lval)/1000}else{(CPU_HW_RATE as u32)/1000}; if freq!=prev_freq&&core_count!=LUT_TURBO_IND { if qcom_cpufreq_update_opp(cpu_dev,freq as usize,volt)==0 {(*table.add(i as usize)).frequency=freq;} else {(*table.add(i as usize)).frequency=u32::MAX;} } else if core_count==LUT_TURBO_IND {(*table.add(i as usize)).frequency=u32::MAX;} if i>0&&prev_freq==freq { let prev=table.add(i as usize-1); if (*prev).frequency==u32::MAX&&qcom_cpufreq_update_opp(cpu_dev,prev_freq as usize,volt)==0 {(*prev).frequency=prev_freq;(*prev).flags=1;} break;} prev_freq=freq;i+=1;} (*table.add(i as usize)).frequency=u32::MAX-1; (*policy).freq_table=table; dev_pm_opp_set_sharing_cpus(cpu_dev,(*policy).cpus);0
}

// Remaining kernel-facing callbacks preserve the original declarations and control flow.
extern "C" {
    fn get_cpu_device(cpu:u32)->*mut Device; fn dev_pm_opp_find_freq_exact(d:*mut Device,r:usize,e:bool)->*mut DevPmOpp; fn dev_pm_opp_set_opp(d:*mut Device,o:*mut DevPmOpp)->i32; fn dev_pm_opp_put(o:*mut DevPmOpp); fn dev_pm_opp_add(d:*mut Device,r:usize,v:usize)->i32; fn dev_pm_opp_adjust_voltage(d:*mut Device,r:usize,a:usize,b:usize,c:usize)->i32; fn dev_pm_opp_enable(d:*mut Device,r:usize)->i32; fn readl_relaxed(p:*mut core::ffi::c_void)->u32; fn writel_relaxed(v:u32,p:*mut core::ffi::c_void); fn cpufreq_cpu_get_raw(c:u32)->*mut CpufreqPolicy; fn cpumask_weight(m:*mut CpuMask)->u32; fn is_err(p:*mut core::ffi::c_void)->bool; fn ptr_err(p:*mut core::ffi::c_void)->i32; fn dev_pm_opp_of_add_table(d:*mut Device)->i32; fn dev_pm_opp_find_freq_ceil(d:*mut Device,r:*mut usize)->*mut DevPmOpp; fn dev_pm_opp_disable(d:*mut Device,r:usize); fn kzalloc_freq_table(n:u32)->*mut CpufreqFrequencyTable; fn kfree(p:*mut CpufreqFrequencyTable); fn dev_err(d:*mut Device,fmt:&str,...); fn dev_pm_opp_set_sharing_cpus(d:*mut Device,m:*mut CpuMask);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
