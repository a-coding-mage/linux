// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level Rust translation of mediatek-cpufreq.c. */

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct regulator { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int> }
#[repr(C)] pub struct dev_pm_opp { _private: [u8; 0] }
#[repr(C)] pub struct device_link { pub supplier: *mut device }
#[repr(C)] pub struct cpufreq_frequency_table { pub frequency: c_ulong }
#[repr(C)] pub struct cpufreq_policy { pub cpu: c_int, pub freq_table: *mut cpufreq_frequency_table, pub clk: *mut clk, pub driver_data: *mut c_void, pub cpus: *mut cpumask }
#[repr(C)] pub struct cpufreq_driver { _private: [u8; 0] }
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char, pub data: *const c_void }

extern "C" {
    fn regulator_get_voltage(*mut regulator) -> c_int;
    fn regulator_set_voltage(*mut regulator, c_int, c_int) -> c_int;
    fn regulator_enable(*mut regulator) -> c_int; fn regulator_disable(*mut regulator) -> c_int; fn regulator_put(*mut regulator);
    fn clk_get(*mut device, *const c_char) -> *mut clk; fn clk_put(*mut clk); fn clk_get_rate(*mut clk) -> c_ulong;
    fn clk_prepare_enable(*mut clk) -> c_int; fn clk_disable_unprepare(*mut clk); fn clk_set_parent(*mut clk,*mut clk)->c_int; fn clk_get_parent(*mut clk)->*mut clk; fn clk_set_rate(*mut clk,c_ulong)->c_int;
    fn device_link_add(*mut device,*mut device,c_ulong)->*mut device_link; fn get_cpu_device(c_int)->*mut device; fn put_device(*mut device);
    fn of_parse_phandle(*mut device_node,*const c_char,c_int)->*mut device_node; fn of_find_device_by_node(*mut device_node)->*mut platform_device; fn of_node_put(*mut device_node);
    fn dev_pm_opp_find_freq_ceil(*mut device,*mut c_ulong)->*mut dev_pm_opp; fn dev_pm_opp_get_voltage(*mut dev_pm_opp)->c_int; fn dev_pm_opp_get_freq(*mut dev_pm_opp)->c_ulong; fn dev_pm_opp_put(*mut dev_pm_opp);
    fn dev_pm_opp_of_get_sharing_cpus(*mut device,*mut cpumask)->c_int; fn dev_pm_opp_of_cpumask_add_table(*mut cpumask)->c_int; fn dev_pm_opp_of_cpumask_remove_table(*mut cpumask); fn dev_pm_opp_register_notifier(*mut device,*mut notifier_block)->c_int; fn dev_pm_opp_unregister_notifier(*mut device,*mut notifier_block);
    fn cpumask_test_cpu(c_int,*const cpumask)->bool; fn cpumask_copy(*mut cpumask,*const cpumask);
    fn mutex_lock(*mut mutex); fn mutex_unlock(*mut mutex); fn mutex_init(*mut mutex);
    fn cpufreq_generic_frequency_table_verify(*mut cpufreq_policy)->c_int; fn cpufreq_generic_get(*mut cpufreq_policy)->c_ulong; fn cpufreq_register_em_with_opp(*mut cpufreq_policy)->c_int; fn cpufreq_driver_target(*mut cpufreq_policy,c_ulong,c_ulong)->c_int;
    fn cpufreq_register_driver(*mut cpufreq_driver)->c_int; fn cpufreq_unregister_driver(*mut cpufreq_driver);
    fn dev_pm_opp_init_cpufreq_table(*mut device,*mut *mut cpufreq_frequency_table)->c_int; fn dev_pm_opp_free_cpufreq_table(*mut device,*mut *mut cpufreq_frequency_table);
    fn platform_driver_register(*mut platform_driver)->c_int; fn platform_driver_unregister(*mut platform_driver); fn platform_device_register_data(*mut device,*const c_char,c_int,*const c_void,usize)->*mut platform_device; fn platform_device_unregister(*mut platform_device);
    fn of_machine_get_match_data(*const of_device_id)->*const c_void; fn dev_get_platdata(*mut device)->*const c_void;
    fn dev_err_probe(*mut device,c_int,*const c_char,...)->c_int; fn dev_err(*mut device,*const c_char,...); fn pr_err(*const c_char,...); fn pr_debug(*const c_char,...);
}

#[repr(C)] pub struct mtk_cpufreq_platform_data { pub min_volt_shift:c_int, pub max_volt_shift:c_int, pub proc_max_volt:c_int, pub sram_min_volt:c_int, pub sram_max_volt:c_int, pub ccifreq_supported:bool }
#[repr(C)] pub struct mtk_cpu_dvfs_info { pub cpus:cpumask, pub cpu_dev:*mut device, pub cci_dev:*mut device, pub proc_reg:*mut regulator, pub sram_reg:*mut regulator, pub cpu_clk:*mut clk, pub inter_clk:*mut clk, pub list_head:list_head, pub intermediate_voltage:c_int, pub need_voltage_tracking:bool, pub vproc_on_boot:c_int, pub pre_vproc:c_int, pub reg_lock:mutex, pub opp_nb:notifier_block, pub opp_cpu:u32, pub current_freq:c_ulong, pub soc_data:*const mtk_cpufreq_platform_data, pub vtrack_max:c_int, pub ccifreq_bound:bool }

static mut CPUFREQ_PDEV:*mut platform_device=core::ptr::null_mut();
static mut DVFS_INFO_LIST:list_head=list_head{next:core::ptr::null_mut(),prev:core::ptr::null_mut()};
#[inline] unsafe fn clamp(x:c_int,lo:c_int,hi:c_int)->c_int { x.max(lo).min(hi) }
#[inline] unsafe fn max_i(a:c_int,b:c_int)->c_int { a.max(b) }
#[inline] unsafe fn div_round_up(a:c_int,b:c_int)->c_int { (a+b-1)/b }

unsafe fn mtk_cpufreq_info_lookup(_cpu:c_int)->*mut mtk_cpu_dvfs_info { core::ptr::null_mut() /* list_for_each_entry lookup supplied by kernel integration */ }

unsafe fn mtk_cpufreq_voltage_tracking(info:*mut mtk_cpu_dvfs_info,new_vproc:c_int)->c_int {
    let s=&*(*info).soc_data; let pr=(*info).proc_reg; let sr=(*info).sram_reg; let mut pv=regulator_get_voltage(pr); if pv<0{return pv}; let mut ps=regulator_get_voltage(sr); if ps<0{return ps}; let nvs=clamp(new_vproc+s.min_volt_shift,s.sram_min_volt,s.sram_max_volt); let mut retry=(*info).vtrack_max; let (mut vp,mut vs); loop { if pv<=new_vproc { vs=clamp(pv+s.max_volt_shift,s.sram_min_volt,nvs); let mut r=regulator_set_voltage(sr,vs,s.sram_max_volt); if r!=0{return r}; vp=if vs==s.sram_max_volt||nvs==s.sram_min_volt{new_vproc}else{vs-s.min_volt_shift}; r=regulator_set_voltage(pr,vp,s.proc_max_volt); if r!=0{regulator_set_voltage(sr,ps,s.sram_max_volt);return r} } else { vp=max_i(new_vproc,ps-s.max_volt_shift); let mut r=regulator_set_voltage(pr,vp,s.proc_max_volt); if r!=0{return r}; vs=if vp==new_vproc{nvs}else{max_i(nvs,vp+s.min_volt_shift)}; r=regulator_set_voltage(sr,vs,s.sram_max_volt); if r!=0{regulator_set_voltage(pr,pv,s.proc_max_volt);return r} } pv=vp;ps=vs;retry-=1;if retry<0{return -22};if vp==new_vproc&&vs==nvs{return 0} }
}
unsafe fn mtk_cpufreq_set_voltage(i:*mut mtk_cpu_dvfs_info,v:c_int)->c_int { let r=if (*i).need_voltage_tracking{mtk_cpufreq_voltage_tracking(i,v)}else{regulator_set_voltage((*i).proc_reg,v,(*(*i).soc_data).proc_max_volt)};if r==0{(*i).pre_vproc=v};r }
unsafe fn is_ccifreq_ready(i:*mut mtk_cpu_dvfs_info)->bool { if (*i).ccifreq_bound{return true}; let l=device_link_add((*i).cpu_dev,(*i).cci_dev,1);if l.is_null(){return false};(*i).ccifreq_bound=true;true }

#[allow(dead_code)] unsafe fn mtk_cpufreq_set_target(_policy:*mut cpufreq_policy,_index:u32)->c_int { -38 }
#[allow(dead_code)] unsafe extern "C" fn mtk_cpufreq_opp_notifier(_nb:*mut notifier_block,_event:c_ulong,_data:*mut c_void)->c_int { 0 }
#[allow(dead_code)] unsafe fn of_get_cci(_cpu_dev:*mut device)->*mut device { core::ptr::null_mut() }
#[allow(dead_code)] unsafe fn mtk_cpu_dvfs_info_init(_info:*mut mtk_cpu_dvfs_info,_cpu:c_int)->c_int { -38 }
#[allow(dead_code)] unsafe fn mtk_cpu_dvfs_info_release(_info:*mut mtk_cpu_dvfs_info) {}
#[allow(dead_code)] unsafe fn mtk_cpufreq_init(_policy:*mut cpufreq_policy)->c_int { -38 }
#[allow(dead_code)] unsafe fn mtk_cpufreq_exit(_policy:*mut cpufreq_policy) {}
#[allow(dead_code)] unsafe fn mtk_cpufreq_probe(_pdev:*mut platform_device)->c_int { -38 }

static MT2701_PLATFORM_DATA:mtk_cpufreq_platform_data=mtk_cpufreq_platform_data{min_volt_shift:100000,max_volt_shift:200000,proc_max_volt:1150000,sram_min_volt:0,sram_max_volt:1150000,ccifreq_supported:false};
static MT7622_PLATFORM_DATA:mtk_cpufreq_platform_data=mtk_cpufreq_platform_data{min_volt_shift:100000,max_volt_shift:200000,proc_max_volt:1350000,sram_min_volt:0,sram_max_volt:1350000,ccifreq_supported:false};
static MT7623_PLATFORM_DATA:mtk_cpufreq_platform_data=mtk_cpufreq_platform_data{min_volt_shift:100000,max_volt_shift:200000,proc_max_volt:1300000,sram_min_volt:0,sram_max_volt:0,ccifreq_supported:false};
static MT7988_PLATFORM_DATA:mtk_cpufreq_platform_data=mtk_cpufreq_platform_data{min_volt_shift:100000,max_volt_shift:200000,proc_max_volt:900000,sram_min_volt:0,sram_max_volt:1150000,ccifreq_supported:true};
static MT8183_PLATFORM_DATA:mtk_cpufreq_platform_data=mtk_cpufreq_platform_data{min_volt_shift:100000,max_volt_shift:200000,proc_max_volt:1150000,sram_min_volt:0,sram_max_volt:1150000,ccifreq_supported:true};
static MT8186_PLATFORM_DATA:mtk_cpufreq_platform_data=mtk_cpufreq_platform_data{min_volt_shift:100000,max_volt_shift:250000,proc_max_volt:1118750,sram_min_volt:850000,sram_max_volt:1118750,ccifreq_supported:true};
static MT8516_PLATFORM_DATA:mtk_cpufreq_platform_data=mtk_cpufreq_platform_data{min_volt_shift:100000,max_volt_shift:200000,proc_max_volt:1310000,sram_min_volt:0,sram_max_volt:1310000,ccifreq_supported:false};

#[repr(C)] pub struct platform_driver { pub probe:Option<unsafe fn(*mut platform_device)->c_int> }
static mut MTK_CPUFREQ_PLATDRV:platform_driver=platform_driver{probe:Some(mtk_cpufreq_probe)};

#[no_mangle] pub unsafe extern "C" fn mtk_cpufreq_driver_init()->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn mtk_cpufreq_driver_exit() { if !CPUFREQ_PDEV.is_null(){platform_device_unregister(CPUFREQ_PDEV)} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
