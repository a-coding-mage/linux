// SPDX-License-Identifier: GPL-2.0+
/* CPU frequency scaling support for Armada 37xx platform. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Linux kernel headers and cpufreq-dt.h supply these declarations.
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct cpufreq_policy { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct cpufreq_dt_platform_data { pub suspend: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> c_int>, pub resume: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> c_int> }

extern "C" {
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> c_int;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> c_int;
    fn syscon_regmap_lookup_by_compatible(s: *const c_char) -> *mut regmap;
    fn get_cpu_device(cpu: c_uint) -> *mut device;
    fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_get_parent(clk: *mut clk) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_put(clk: *mut clk);
    fn dev_pm_opp_add(dev: *mut device, freq: c_ulong, volt: c_ulong) -> c_int;
    fn dev_pm_opp_remove(dev: *mut device, freq: c_ulong);
    fn platform_device_register_data(parent: *mut device, name: *const c_char, id: c_int, data: *mut c_void, size: usize) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut armada_37xx_dvfs);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut armada_37xx_dvfs;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_pm_opp_remove_all_dynamic(dev: *mut device);
}

const ARMADA_37XX_CLK_TBG_SEL: u32 = 0;
const ARMADA_37XX_CLK_TBG_SEL_CPU_OFF: u32 = 22;
const ARMADA_37XX_NB_L0L1: u32 = 0x18;
const ARMADA_37XX_NB_L2L3: u32 = 0x1c;
const ARMADA_37XX_NB_TBG_DIV_OFF: u32 = 13;
const ARMADA_37XX_NB_TBG_DIV_MASK: u32 = 0x7;
const ARMADA_37XX_NB_CLK_SEL_OFF: u32 = 11;
const ARMADA_37XX_NB_CLK_SEL_MASK: u32 = 0x1;
const ARMADA_37XX_NB_CLK_SEL_TBG: u32 = 0x1;
const ARMADA_37XX_NB_TBG_SEL_OFF: u32 = 9;
const ARMADA_37XX_NB_TBG_SEL_MASK: u32 = 0x3;
const ARMADA_37XX_NB_VDD_SEL_OFF: u32 = 6;
const ARMADA_37XX_NB_VDD_SEL_MASK: u32 = 0x3;
const ARMADA_37XX_NB_CONFIG_SHIFT: u32 = 16;
const ARMADA_37XX_NB_DYN_MOD: u32 = 0x24;
const ARMADA_37XX_NB_CLK_SEL_EN: u32 = 1 << 26;
const ARMADA_37XX_NB_TBG_EN: u32 = 1 << 28;
const ARMADA_37XX_NB_DIV_EN: u32 = 1 << 29;
const ARMADA_37XX_NB_VDD_EN: u32 = 1 << 30;
const ARMADA_37XX_NB_DFS_EN: u32 = 1 << 31;
const ARMADA_37XX_NB_CPU_LOAD: u32 = 0x30;
const ARMADA_37XX_NB_CPU_LOAD_MASK: u32 = 0x3;
const ARMADA_37XX_AVS_CTL0: u32 = 0;
const ARMADA_37XX_AVS_ENABLE: u32 = 1 << 30;
const ARMADA_37XX_AVS_HIGH_VDD_LIMIT: u32 = 16;
const ARMADA_37XX_AVS_LOW_VDD_LIMIT: u32 = 22;
const ARMADA_37XX_AVS_VDD_MASK: u32 = 0x3f;
const ARMADA_37XX_AVS_CTL2: u32 = 8;
const ARMADA_37XX_AVS_LOW_VDD_EN: u32 = 1 << 6;
const LOAD_LEVEL_NR: usize = 4;
const MIN_VOLT_MV: i32 = 1000;
const MIN_VOLT_MV_FOR_L1_1000MHZ: i32 = 1108;
const MIN_VOLT_MV_FOR_L1_1200MHZ: i32 = 1155;
const ARMADA_37XX_DVFS_LOAD_0: i32 = 0;
const ARMADA_37XX_DVFS_LOAD_1: i32 = 1;
const ARMADA_37XX_DVFS_LOAD_2: i32 = 2;

static mut avs_map: [i32; 43] = [747,758,770,782,793,805,817,828,840,852,863,875,887,898,910,922,933,945,957,968,980,992,1003,1015,1027,1038,1050,1062,1073,1085,1097,1108,1120,1132,1143,1155,1167,1178,1190,1202,1213,1225,1237,1248,1260,1272,1283,1295,1307,1318,1330,1342];

#[repr(C)] struct armada37xx_cpufreq_state { pdev: *mut platform_device, cpu_dev: *mut device, regmap: *mut regmap, nb_l0l1: u32, nb_l2l3: u32, nb_dyn_mod: u32, nb_cpu_load: u32 }
static mut armada37xx_cpufreq_state: *mut armada37xx_cpufreq_state = core::ptr::null_mut();
#[repr(C)] struct armada_37xx_dvfs { cpu_freq_max: u32, divider: [u8; LOAD_LEVEL_NR], avs: [u32; LOAD_LEVEL_NR] }
static mut armada_37xx_dvfs: [armada_37xx_dvfs; 4] = [
    armada_37xx_dvfs { cpu_freq_max: 1200*1000*1000, divider: [1,2,4,6], avs: [0;4] },
    armada_37xx_dvfs { cpu_freq_max: 1000*1000*1000, divider: [1,2,4,5], avs: [0;4] },
    armada_37xx_dvfs { cpu_freq_max: 800*1000*1000, divider: [1,2,3,4], avs: [0;4] },
    armada_37xx_dvfs { cpu_freq_max: 600*1000*1000, divider: [2,4,5,6], avs: [0;4] },
];

unsafe fn armada_37xx_cpu_freq_info_get(freq: u32) -> *mut armada_37xx_dvfs { for i in 0..4 { if freq == armada_37xx_dvfs[i].cpu_freq_max { return &mut armada_37xx_dvfs[i]; } } core::ptr::null_mut() }
unsafe fn armada_37xx_avs_val_match(target_vm: i32) -> u32 { let mut avs=0; while avs < avs_map.len() && avs_map[avs] < target_vm { avs+=1; } if avs == avs_map.len() { avs-=1; } avs as u32 }

unsafe fn armada37xx_cpufreq_dvfs_setup(base:*mut regmap, clk_base:*mut regmap, divider:*const u8) { let mut cpu_tbg_sel=0; regmap_read(clk_base,ARMADA_37XX_CLK_TBG_SEL,&mut cpu_tbg_sel); cpu_tbg_sel=(cpu_tbg_sel>>ARMADA_37XX_CLK_TBG_SEL_CPU_OFF)&ARMADA_37XX_NB_TBG_SEL_MASK; for load_lvl in 0..LOAD_LEVEL_NR { let reg=if load_lvl<=1 {ARMADA_37XX_NB_L0L1} else {ARMADA_37XX_NB_L2L3}; let offset=if load_lvl==0||load_lvl==2 {ARMADA_37XX_NB_CONFIG_SHIFT} else {0}; let mut val=cpu_tbg_sel<<ARMADA_37XX_NB_TBG_SEL_OFF; let mut mask=ARMADA_37XX_NB_TBG_SEL_MASK<<ARMADA_37XX_NB_TBG_SEL_OFF; val|=(*divider.add(load_lvl) as u32)<<ARMADA_37XX_NB_TBG_DIV_OFF; mask|=ARMADA_37XX_NB_TBG_DIV_MASK<<ARMADA_37XX_NB_TBG_DIV_OFF; val|=(load_lvl as u32)<<ARMADA_37XX_NB_VDD_SEL_OFF; mask|=ARMADA_37XX_NB_VDD_SEL_MASK<<ARMADA_37XX_NB_VDD_SEL_OFF; regmap_update_bits(base,reg,mask<<offset,val<<offset); } }

unsafe fn armada37xx_cpufreq_avs_configure(base:*mut regmap,dvfs:*mut armada_37xx_dvfs) { if base.is_null(){return} let mut l0=0; regmap_read(base,ARMADA_37XX_AVS_CTL0,&mut l0); let l0=((l0>>ARMADA_37XX_AVS_LOW_VDD_LIMIT)&ARMADA_37XX_AVS_VDD_MASK) as usize; if l0>=avs_map.len(){return} (*dvfs).avs[0]=l0 as u32; if avs_map[l0]<=MIN_VOLT_MV { let mut a=armada_37xx_avs_val_match(MIN_VOLT_MV); for i in 1..LOAD_LEVEL_NR {(*dvfs).avs[i]=a;} if (*dvfs).cpu_freq_max>=1000*1000*1000 {a=armada_37xx_avs_val_match(if (*dvfs).cpu_freq_max>=1200*1000*1000 {MIN_VOLT_MV_FOR_L1_1200MHZ}else{MIN_VOLT_MV_FOR_L1_1000MHZ});(*dvfs).avs[0]=a;(*dvfs).avs[1]=a;} return; } let t1=core::cmp::max(avs_map[l0]-100,MIN_VOLT_MV); (*dvfs).avs[1]=armada_37xx_avs_val_match(t1); let t2=core::cmp::max(avs_map[l0]-150,MIN_VOLT_MV); (*dvfs).avs[2]=armada_37xx_avs_val_match(t2);(*dvfs).avs[3]=(*dvfs).avs[2]; if (*dvfs).cpu_freq_max>=1000*1000*1000 {let mut a=armada_37xx_avs_val_match(if (*dvfs).cpu_freq_max>=1200*1000*1000 {MIN_VOLT_MV_FOR_L1_1200MHZ}else{MIN_VOLT_MV_FOR_L1_1000MHZ});if a>(*dvfs).avs[0]{a=(*dvfs).avs[0];}if (*dvfs).avs[1]<a{(*dvfs).avs[1]=a;}} }

unsafe fn armada37xx_cpufreq_avs_setup(base:*mut regmap,dvfs:*mut armada_37xx_dvfs) { if base.is_null(){return} regmap_update_bits(base,ARMADA_37XX_AVS_CTL0,ARMADA_37XX_AVS_ENABLE,0);regmap_update_bits(base,ARMADA_37XX_AVS_CTL2,ARMADA_37XX_AVS_LOW_VDD_EN,ARMADA_37XX_AVS_LOW_VDD_EN);for l in 1..LOAD_LEVEL_NR {let a=(*dvfs).avs[l];let reg=0x1c+4*((l-1) as u32);let mask=(ARMADA_37XX_AVS_VDD_MASK<<ARMADA_37XX_AVS_HIGH_VDD_LIMIT)|(ARMADA_37XX_AVS_VDD_MASK<<ARMADA_37XX_AVS_LOW_VDD_LIMIT);regmap_update_bits(base,reg,mask,(a<<ARMADA_37XX_AVS_HIGH_VDD_LIMIT)|(a<<ARMADA_37XX_AVS_LOW_VDD_LIMIT));}regmap_update_bits(base,ARMADA_37XX_AVS_CTL0,ARMADA_37XX_AVS_ENABLE,ARMADA_37XX_AVS_ENABLE); }
unsafe fn armada37xx_cpufreq_disable_dvfs(base:*mut regmap){regmap_update_bits(base,ARMADA_37XX_NB_DYN_MOD,ARMADA_37XX_NB_DFS_EN,0);}
unsafe fn armada37xx_cpufreq_enable_dvfs(base:*mut regmap){regmap_update_bits(base,ARMADA_37XX_NB_CPU_LOAD,ARMADA_37XX_NB_CPU_LOAD_MASK,0);let m=ARMADA_37XX_NB_CLK_SEL_EN|ARMADA_37XX_NB_TBG_EN|ARMADA_37XX_NB_DIV_EN|ARMADA_37XX_NB_VDD_EN|ARMADA_37XX_NB_DFS_EN;regmap_update_bits(base,ARMADA_37XX_NB_DYN_MOD,m,m);}

unsafe extern "C" fn armada37xx_cpufreq_suspend(_: *mut cpufreq_policy)->c_int {let s=&mut *armada37xx_cpufreq_state;regmap_read(s.regmap,ARMADA_37XX_NB_L0L1,&mut s.nb_l0l1);regmap_read(s.regmap,ARMADA_37XX_NB_L2L3,&mut s.nb_l2l3);regmap_read(s.regmap,ARMADA_37XX_NB_CPU_LOAD,&mut s.nb_cpu_load);regmap_read(s.regmap,ARMADA_37XX_NB_DYN_MOD,&mut s.nb_dyn_mod);0}
unsafe extern "C" fn armada37xx_cpufreq_resume(_: *mut cpufreq_policy)->c_int {let s=&mut *armada37xx_cpufreq_state;armada37xx_cpufreq_disable_dvfs(s.regmap);regmap_write(s.regmap,ARMADA_37XX_NB_L0L1,s.nb_l0l1);regmap_write(s.regmap,ARMADA_37XX_NB_L2L3,s.nb_l2l3);regmap_write(s.regmap,ARMADA_37XX_NB_CPU_LOAD,s.nb_cpu_load);regmap_write(s.regmap,ARMADA_37XX_NB_DYN_MOD,s.nb_dyn_mod);0}

// The init/exit entry points retain the source lifecycle; kernel registration macros are represented as comments.
unsafe fn armada37xx_cpufreq_driver_init()->c_int {
    // late_initcall: loaded after the A37xx clock driver.
    let nb_clk_base=syscon_regmap_lookup_by_compatible(b"marvell,armada-3700-periph-clock-nb\0".as_ptr() as *const c_char); if nb_clk_base.is_null(){return -19;}
    let nb_pm_base=syscon_regmap_lookup_by_compatible(b"marvell,armada-3700-nb-pm\0".as_ptr() as *const c_char); if nb_pm_base.is_null(){return -19;}
    let avs_base=syscon_regmap_lookup_by_compatible(b"marvell,armada-3700-avs\0".as_ptr() as *const c_char); armada37xx_cpufreq_disable_dvfs(nb_pm_base);
    let cpu_dev=get_cpu_device(0); if cpu_dev.is_null(){return -19;} let clk=clk_get(cpu_dev,core::ptr::null()); if clk.is_null(){return -19;} let parent=clk_get_parent(clk); if parent.is_null(){clk_put(clk);return -19;}
    let base_frequency=clk_get_rate(parent) as u32; if base_frequency==0{clk_put(clk);return -22;} let dvfs=armada_37xx_cpu_freq_info_get(base_frequency); if dvfs.is_null(){clk_put(clk);return -22;}
    armada37xx_cpufreq_state=kmalloc(core::mem::size_of::<armada37xx_cpufreq_state>(),0) as *mut armada37xx_cpufreq_state; if armada37xx_cpufreq_state.is_null(){clk_put(clk);return -12;} (*armada37xx_cpufreq_state).regmap=nb_pm_base;
    armada37xx_cpufreq_avs_configure(if avs_base.is_null(){core::ptr::null_mut()}else{avs_base},dvfs); armada37xx_cpufreq_avs_setup(avs_base,dvfs); armada37xx_cpufreq_dvfs_setup(nb_pm_base,nb_clk_base,(*dvfs).divider.as_ptr()); clk_put(clk);
    for l in 0..LOAD_LEVEL_NR {let freq=(base_frequency/(*dvfs).divider[l] as u32) as c_ulong;let volt=(avs_map[(*dvfs).avs[l] as usize]*1000) as c_ulong;if dev_pm_opp_add(cpu_dev,freq,volt)!=0{for j in (0..l).rev(){dev_pm_opp_remove(cpu_dev,(base_frequency/(*dvfs).divider[j] as u32) as c_ulong);}kfree(armada37xx_cpufreq_state as *mut c_void);return -12;}}
    armada37xx_cpufreq_enable_dvfs(nb_pm_base); let mut pdata=cpufreq_dt_platform_data{suspend:Some(armada37xx_cpufreq_suspend),resume:Some(armada37xx_cpufreq_resume)}; let pdev=platform_device_register_data(core::ptr::null_mut(),b"cpufreq-dt\0".as_ptr() as *const c_char,-1,&mut pdata as *mut _ as *mut c_void,core::mem::size_of_val(&pdata)); if pdev.is_null(){armada37xx_cpufreq_disable_dvfs(nb_pm_base);return -12;} (*armada37xx_cpufreq_state).cpu_dev=cpu_dev;(*armada37xx_cpufreq_state).pdev=pdev;platform_set_drvdata(pdev,dvfs);0
}
unsafe fn armada37xx_cpufreq_driver_exit() {let s=&mut *armada37xx_cpufreq_state;let pdev=s.pdev;let dvfs=platform_get_drvdata(pdev);platform_device_unregister(pdev);armada37xx_cpufreq_disable_dvfs(s.regmap);for l in 0..LOAD_LEVEL_NR{dev_pm_opp_remove(s.cpu_dev,((*dvfs).cpu_freq_max/(*dvfs).divider[l] as u32) as c_ulong);}kfree(armada37xx_cpufreq_state as *mut c_void);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
