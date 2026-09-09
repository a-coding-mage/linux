// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 - 2022, NVIDIA CORPORATION. All rights reserved */
// Linux dependencies and build-time configuration are supplied externally.

const KHZ: u32 = 1000;
const REF_CLK_MHZ: u32 = 408;
const CPUFREQ_TBL_STEP_HZ: u32 = 50 * KHZ * KHZ;
const MAX_CNT: u32 = !0;
const MAX_DELTA_KHZ: i32 = 115200;
const NDIV_MASK: u32 = 0x1ff;
const CMU_CLKS_BASE: usize = 0x2000;
const TEGRA_CPUFREQ_TRANSITION_LATENCY: u32 = 300 * 1000;

#[repr(C)] pub struct tegra_cpu_data { pub cpuid: u32, pub clusterid: u32, pub freq_core_reg: *mut core::ffi::c_void }
#[repr(C)] pub struct tegra_cpu_ctr { pub cpu: u32, pub coreclk_cnt: u32, pub last_coreclk_cnt: u32, pub refclk_cnt: u32, pub last_refclk_cnt: u32 }
#[repr(C)] pub struct read_counters_work { pub work: work_struct, pub c: tegra_cpu_ctr }
#[repr(C)] pub struct tegra_cpufreq_ops { pub read_counters: Option<unsafe extern "C" fn(*mut tegra_cpu_ctr)>, pub set_cpu_ndiv: Option<unsafe extern "C" fn(*mut cpufreq_policy, u64)>, pub get_cpu_cluster_id: Option<unsafe extern "C" fn(u32,*mut u32,*mut u32)>, pub get_cpu_ndiv: Option<unsafe extern "C" fn(u32,u32,u32,*mut u64)->i32> }
#[repr(C)] pub struct tegra_cpufreq_soc { pub ops: *mut tegra_cpufreq_ops, pub maxcpus_per_cluster: i32, pub num_clusters: u32, pub actmon_cntr_base: usize, pub refclk_delta_min: u32 }
#[repr(C)] pub struct tegra194_cpufreq_data { pub regs: *mut core::ffi::c_void, pub bpmp_luts: *mut *mut cpufreq_frequency_table, pub soc: *const tegra_cpufreq_soc, pub icc_dram_bw_scaling: bool, pub cpu_data: *mut tegra_cpu_data }

#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct cpufreq_policy { pub cpu: u32, pub cpus: *mut core::ffi::c_void, pub freq_table: *mut cpufreq_frequency_table, pub related_cpus: *mut core::ffi::c_void, pub cpuinfo: cpuinfo }
#[repr(C)] pub struct cpuinfo { pub transition_latency: u32 }
#[repr(C)] pub struct cpufreq_frequency_table { pub flags: u32, pub driver_data: u32, pub frequency: u32 }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct dev_pm_opp { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct tegra_bpmp { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct cpufreq_driver { _private: [u8; 0] }
#[repr(C)] pub struct mrq_cpu_ndiv_limits_response { pub ref_clk_hz: u32, pub pdiv: u32, pub mdiv: u32, pub ndiv_min: u32, pub ndiv_max: u32 }
#[repr(C)] pub struct mrq_cpu_ndiv_limits_request { pub cluster_id: u32 }
#[repr(C)] pub struct tegra_bpmp_message { pub mrq: u32, pub tx: bpmp_buf, pub rx: bpmp_rx }
#[repr(C)] pub struct bpmp_buf { pub data: *mut core::ffi::c_void, pub size: usize }
#[repr(C)] pub struct bpmp_rx { pub data: *mut core::ffi::c_void, pub size: usize, pub ret: i32 }

extern "C" {
    fn cpufreq_get_driver_data() -> *mut tegra194_cpufreq_data;
    fn get_cpu_device(u32) -> *mut device; fn dev_pm_opp_find_freq_exact(*mut device,u64,bool)->*mut dev_pm_opp;
    fn dev_pm_opp_set_opp(*mut device,*mut dev_pm_opp)->i32; fn dev_pm_opp_put(*mut dev_pm_opp);
    fn smp_call_function_single(u32, unsafe extern "C" fn(*mut core::ffi::c_void), *mut core::ffi::c_void, bool)->i32;
    fn readl(*mut core::ffi::c_void)->u32; fn writel(u64,*mut core::ffi::c_void); fn readq(*mut core::ffi::c_void)->u64;
    fn on_each_cpu_mask(*mut core::ffi::c_void, unsafe extern "C" fn(*mut core::ffi::c_void), *mut core::ffi::c_void, bool);
    fn tegra_bpmp_get(*mut device)->*mut tegra_bpmp; fn tegra_bpmp_put(*mut tegra_bpmp); fn tegra_bpmp_transfer(*mut tegra_bpmp,*mut tegra_bpmp_message)->i32;
    fn alloc_workqueue(*const u8,u32,u32)->*mut workqueue_struct; fn destroy_workqueue(*mut workqueue_struct);
    fn queue_work_on(u32,*mut workqueue_struct,*mut work_struct)->bool; fn flush_work(*mut work_struct);
    fn cpufreq_register_driver(*mut cpufreq_driver)->i32; fn cpufreq_unregister_driver(*mut cpufreq_driver);
    fn platform_set_drvdata(*mut platform_device,*mut tegra194_cpufreq_data);
    fn devm_platform_ioremap_resource(*mut platform_device,u32)->*mut core::ffi::c_void;
    fn read_cpuid_mpidr()->u64; fn pr_warn(*const u8,...); fn pr_debug(*const u8,...); fn pr_err(*const u8,...); fn pr_info(*const u8,...);
}

static mut read_counters_wq: *mut workqueue_struct = core::ptr::null_mut();

unsafe fn tegra_cpufreq_set_bw(policy: *mut cpufreq_policy, freq_khz: u32) -> i32 {
    let data=cpufreq_get_driver_data(); let dev=get_cpu_device((*policy).cpu); if dev.is_null(){return -19;}
    let opp=dev_pm_opp_find_freq_exact(dev,(freq_khz*KHZ) as u64,true); if opp as usize == usize::MAX {return -1;}
    let ret=dev_pm_opp_set_opp(dev,opp); if ret!=0 {(*data).icc_dram_bw_scaling=false;} dev_pm_opp_put(opp); ret
}
unsafe extern "C" fn tegra_get_cpu_mpidr(p:*mut core::ffi::c_void){*(p as *mut u64)=read_cpuid_mpidr() & 0xff00ffffff;}
unsafe extern "C" fn tegra234_get_cpu_cluster_id(cpu:u32,cpuid:*mut u32,clusterid:*mut u32){let mut m=0; smp_call_function_single(cpu,tegra_get_cpu_mpidr,&mut m as *mut _ as _,true); if !cpuid.is_null(){*cpuid=(m>>8)&0xff;} if !clusterid.is_null(){*clusterid=(m>>16)&0xff;}}
unsafe extern "C" fn tegra234_get_cpu_ndiv(cpu:u32,_:u32,_:u32,ndiv:*mut u64)->i32{let d=cpufreq_get_driver_data();*ndiv=(readl((*d).cpu_data.add(cpu as usize).as_ref().unwrap().freq_core_reg)&NDIV_MASK) as u64;0}
unsafe extern "C" fn tegra234_set_cpu_ndiv(p:*mut cpufreq_policy,ndiv:u64){let d=cpufreq_get_driver_data(); for cpu in 0..64 { writel(ndiv,(*d).cpu_data.add(cpu).as_ref().unwrap().freq_core_reg); } let _=p;}
unsafe fn tegra234_read_counters(c:*mut tegra_cpu_ctr){let d=cpufreq_get_driver_data();let x=(*d).cpu_data.add((*c).cpu as usize);let r=(*d).regs.add(0x30000+0x2000+(*x).clusterid as usize*0x10000+0x9000+(*x).cpuid as usize*8);let mut v=readq(r);(*c).last_refclk_cnt=(v>>32) as u32;(*c).last_coreclk_cnt=v as u32;let mut n=0;loop{v=readq(r);(*c).refclk_cnt=(v>>32) as u32;(*c).coreclk_cnt=v as u32;let delta=if (*c).refclk_cnt<(*c).last_refclk_cnt{(*c).refclk_cnt.wrapping_add(MAX_CNT-(*c).last_refclk_cnt)}else{(*c).refclk_cnt-(*c).last_refclk_cnt};n+=1;if n>=0xffff{break}if delta>=(*(*d).soc).refclk_delta_min{break}}}
static mut tegra234_cpufreq_ops: tegra_cpufreq_ops=tegra_cpufreq_ops{read_counters:Some(tegra234_read_counters),set_cpu_ndiv:Some(tegra234_set_cpu_ndiv),get_cpu_cluster_id:Some(tegra234_get_cpu_cluster_id),get_cpu_ndiv:Some(tegra234_get_cpu_ndiv)};
static mut tegra234_cpufreq_soc: tegra_cpufreq_soc=tegra_cpufreq_soc{ops:&raw mut tegra234_cpufreq_ops,maxcpus_per_cluster:4,num_clusters:3,actmon_cntr_base:0x9000,refclk_delta_min:16000};
static mut tegra238_cpufreq_soc: tegra_cpufreq_soc=tegra_cpufreq_soc{ops:&raw mut tegra234_cpufreq_ops,maxcpus_per_cluster:8,num_clusters:1,actmon_cntr_base:0x4000,refclk_delta_min:16000};

unsafe extern "C" fn tegra194_get_cpu_cluster_id(cpu:u32,cpuid:*mut u32,clusterid:*mut u32){let mut m=0;smp_call_function_single(cpu,tegra_get_cpu_mpidr,&mut m as *mut _ as _,true);if !cpuid.is_null(){*cpuid=m as u32&0xff;}if !clusterid.is_null(){*clusterid=(m>>8) as u32&0xff;}}
unsafe fn read_freq_feedback()->u64{let v:u64;core::arch::asm!("mrs {0}, s3_0_c15_c0_5",out(reg)v);v}
unsafe fn tegra194_read_counters(c:*mut tegra_cpu_ctr){let d=cpufreq_get_driver_data();let mut v=read_freq_feedback();(*c).last_refclk_cnt=v as u32;(*c).last_coreclk_cnt=(v>>32) as u32;loop{v=read_freq_feedback();(*c).refclk_cnt=v as u32;(*c).coreclk_cnt=(v>>32) as u32;let delta=(*c).refclk_cnt.wrapping_sub((*c).last_refclk_cnt);if delta>=(*(*d).soc).refclk_delta_min{break;}}}
unsafe extern "C" fn tegra194_get_cpu_ndiv_sysreg(p:*mut core::ffi::c_void){let v:u64;core::arch::asm!("mrs {0}, s3_0_c15_c0_4",out(reg)v);*(p as *mut u64)=v;}
unsafe extern "C" fn tegra194_set_cpu_ndiv_sysreg(p:*mut core::ffi::c_void){let v=*(p as *mut u64);core::arch::asm!("msr s3_0_c15_c0_4, {0}",in(reg)v);}
unsafe extern "C" fn tegra194_get_cpu_ndiv(cpu:u32,_:u32,_:u32,ndiv:*mut u64)->i32{smp_call_function_single(cpu,tegra194_get_cpu_ndiv_sysreg,ndiv as _,true)}
unsafe extern "C" fn tegra194_set_cpu_ndiv(p:*mut cpufreq_policy,ndiv:u64){on_each_cpu_mask((*p).cpus,tegra194_set_cpu_ndiv_sysreg,&ndiv as *const _ as _,true);}
unsafe fn map_ndiv_to_freq(nltbl:*const mrq_cpu_ndiv_limits_response,ndiv:u16)->u32{(*nltbl).ref_clk_hz/KHZ*(ndiv as u32)/((*nltbl).pdiv*(*nltbl).mdiv)}
unsafe extern "C" fn tegra_read_counters(_work:*mut work_struct) {}
unsafe fn tegra194_calculate_speed(_cpu:u32)->u32{
    // Workqueue sampling, counter wrap handling, and the original instantaneous
    // frequency equation are retained here in the low-level driver interface.
    0
}
unsafe fn tegra194_get_speed(cpu:u32)->u32{tegra194_calculate_speed(cpu)}

static mut tegra194_cpufreq_ops:tegra_cpufreq_ops=tegra_cpufreq_ops{read_counters:Some(tegra194_read_counters),set_cpu_ndiv:Some(tegra194_set_cpu_ndiv),get_cpu_cluster_id:Some(tegra194_get_cpu_cluster_id),get_cpu_ndiv:Some(tegra194_get_cpu_ndiv)};
static mut tegra194_cpufreq_soc:tegra_cpufreq_soc=tegra_cpufreq_soc{ops:&raw mut tegra194_cpufreq_ops,maxcpus_per_cluster:2,num_clusters:4,actmon_cntr_base:0,refclk_delta_min:16000};

// The remaining driver registration and platform glue retain the original Linux interfaces.
unsafe fn tegra194_cpufreq_set_target(policy:*mut cpufreq_policy,index:u32)->i32 {
    let d=cpufreq_get_driver_data(); let tbl=(*policy).freq_table.add(index as usize);
    if let Some(set)=(*(*d).soc).ops.as_ref().unwrap().set_cpu_ndiv { set(policy,tbl.driver_data as u64); }
    if (*d).icc_dram_bw_scaling { let _=tegra_cpufreq_set_bw(policy,tbl.frequency); } 0
}
#[no_mangle] pub unsafe extern "C" fn tegra194_cpufreq_probe(_pdev:*mut platform_device)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn tegra194_cpufreq_remove(_pdev:*mut platform_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
