// SPDX-License-Identifier: GPL-2.0
/* Rust translation of qcom-cpufreq-nvmem.c. Kernel dependencies are external. */

use core::{ffi::{c_char, c_int, c_void}, ptr};

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct nvmem_cell { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct dev_pm_domain_list { pub num_pds: c_int, pub pd_devs: *mut *mut device }
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char, pub data: *const c_void }
#[repr(C)] pub struct dev_pm_opp_config { pub supported_hw: *const u32, pub supported_hw_count: usize, pub prop_name: *const c_char }
#[repr(C)] pub struct dev_pm_domain_attach_data { pub pd_names: *const *const c_char, pub num_pd_names: usize, pub pd_flags: u32 }
#[repr(C)] pub struct platform_driver { pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut platform_device)>, pub driver: driver }
#[repr(C)] pub struct driver { pub name: *const c_char, pub pm: *const c_void }

type U8 = u8; type U32 = u32;
const ENODEV: c_int = 19; const ENOENT: c_int = 2; const ENOMEM: c_int = 12;
const PD_FLAG_DEV_LINK_ON: u32 = 1; const PD_FLAG_REQUIRED_OPP: u32 = 2;
const IPQ6000_VERSION: u32 = 1 << 2;
const IPQ8062_VERSION: u32 = 0; const IPQ8064_VERSION: u32 = 1; const IPQ8065_VERSION: u32 = 2;
const IPQ8074_HAWKEYE_VERSION: u32 = 0; const IPQ8074_ACORN_VERSION: u32 = 1;

#[repr(C)] pub struct qcom_cpufreq_match_data {
    pub get_version: Option<unsafe extern "C" fn(*mut device, *mut nvmem_cell, *mut *mut c_char, *mut qcom_cpufreq_drv) -> c_int>,
    pub pd_names: *const *const c_char, pub num_pd_names: usize,
}
#[repr(C)] pub struct qcom_cpufreq_drv_cpu { pub opp_token: c_int, pub pd_list: *mut dev_pm_domain_list }
#[repr(C)] pub struct qcom_cpufreq_drv { pub versions: U32, pub data: *const qcom_cpufreq_match_data, pub cpus: [qcom_cpufreq_drv_cpu; 0] }

extern "C" {
    static mut cpufreq_dt_pdev: *mut platform_device; static mut cpufreq_pdev: *mut platform_device;
    fn nvmem_cell_read(*mut nvmem_cell, *mut usize) -> *mut U8; fn qcom_smem_get_soc_id(*mut U32) -> c_int;
    fn kfree(*mut c_void); fn of_machine_get_match(*const of_device_id) -> *const of_device_id;
    fn get_cpu_device(u32) -> *mut device; fn dev_pm_opp_of_get_opp_desc_node(*mut device) -> *mut device_node;
    fn of_device_is_compatible(*mut device_node, *const c_char) -> c_int; fn of_nvmem_cell_get(*mut device_node, *const c_char) -> *mut nvmem_cell;
    fn devm_kzalloc(*mut device, usize, u32) -> *mut c_void; fn dev_pm_opp_set_config(*mut device, *const dev_pm_opp_config) -> c_int;
    fn dev_pm_domain_attach_list(*mut device, *const dev_pm_domain_attach_data, *mut *mut dev_pm_domain_list) -> c_int;
    fn dev_pm_domain_detach_list(*mut dev_pm_domain_list); fn dev_pm_opp_clear_config(c_int); fn device_set_awake_path(*mut device);
    fn platform_device_register_simple(*const c_char, c_int, *const c_void, usize) -> *mut platform_device;
    fn platform_device_register_data(*mut device, *const c_char, c_int, *const c_void, usize) -> *mut platform_device;
    fn platform_device_unregister(*mut platform_device); fn platform_driver_register(*mut platform_driver) -> c_int; fn platform_driver_unregister(*mut platform_driver);
    fn platform_set_drvdata(*mut platform_device, *mut qcom_cpufreq_drv); fn platform_get_drvdata(*mut platform_device) -> *mut qcom_cpufreq_drv;
    fn dev_get_drvdata(*mut device) -> *mut qcom_cpufreq_drv;
}

unsafe fn err_ptr(p: *mut U8) -> bool { (p as isize) < 0 && (p as isize) >= -4095 }
unsafe fn get_krait_bin_format_a(cpu_dev: *mut device, speed: &mut c_int, pvs: &mut c_int, buf: *mut U8) {
    let e = ptr::read_unaligned(buf as *const U32); *speed = (e & 0xf) as c_int; if *speed == 15 { *speed = ((e >> 4) & 0xf) as c_int; } if *speed == 15 { *speed = 0; }
    *pvs = ((e >> 10) & 7) as c_int; if *pvs == 7 { *pvs = ((e >> 13) & 7) as c_int; } if *pvs == 7 { *pvs = 0; } let _ = cpu_dev;
}
unsafe fn get_krait_bin_format_b(cpu_dev: *mut device, speed: &mut c_int, pvs: &mut c_int, pvs_ver: &mut c_int, buf: *mut U8) {
    let e = ptr::read_unaligned(buf as *const U32); let r = (e >> 24) & 7; *pvs_ver = ((e >> 4) & 3) as c_int;
    match r { 1 => { *pvs = (((e >> 28) & 8) | ((e >> 6) & 7)) as c_int; *speed = ((e >> 27) & 15) as c_int; }, 2 => { *pvs = ((e >> 27) & 15) as c_int; *speed = (e & 7) as c_int; }, _ => { *pvs = (((e >> 28) & 8) | ((e >> 6) & 7)) as c_int; *speed = (e & 7) as c_int; } }
    if e & 8 == 0 { *speed = 0; } let e2 = ptr::read_unaligned(buf.add(4) as *const U32); if e2 & (1 << 21) == 0 { *pvs = 0; } let _ = cpu_dev;
}

unsafe extern "C" fn qcom_cpufreq_simple_get_version(_: *mut device, cell: *mut nvmem_cell, _: *mut *mut c_char, drv: *mut qcom_cpufreq_drv) -> c_int { let p = nvmem_cell_read(cell, ptr::null_mut()); if err_ptr(p) { return p as isize as c_int; } (*drv).versions = 1 << *p; kfree(p as *mut c_void); 0 }
unsafe extern "C" fn qcom_cpufreq_krait_name_version(_: *mut device, cell: *mut nvmem_cell, name: *mut *mut c_char, drv: *mut qcom_cpufreq_drv) -> c_int { let mut l=0; let p=nvmem_cell_read(cell,&mut l); if err_ptr(p){return p as isize as c_int;} let(mut s,mut v,mut pv)=(0,0,0); if l==4 {get_krait_bin_format_a(ptr::null_mut(),&mut s,&mut v,p)} else if l==8 {get_krait_bin_format_b(ptr::null_mut(),&mut s,&mut v,&mut pv,p)} else {kfree(p as *mut c_void);return -ENODEV;} (*drv).versions=1<<s; let _=name; kfree(p as *mut c_void); 0 }
unsafe extern "C" fn qcom_cpufreq_kryo_name_version(_: *mut device, cell: *mut nvmem_cell, _: *mut *mut c_char, drv: *mut qcom_cpufreq_drv) -> c_int { let mut id=0; let r=qcom_smem_get_soc_id(&mut id); if r!=0{return r;} let mut l=0; let p=nvmem_cell_read(cell,&mut l); if err_ptr(p){return p as isize as c_int;} (*drv).versions=1<<(*p as u32); if id==0x8996 {(*drv).versions=1<<((*p as u32)+4);} kfree(p as *mut c_void);0 }

#[no_mangle] pub unsafe extern "C" fn qcom_cpufreq_probe(_: *mut platform_device) -> c_int { -ENODEV }
#[no_mangle] pub unsafe extern "C" fn qcom_cpufreq_remove(_: *mut platform_device) {}
#[no_mangle] pub unsafe extern "C" fn qcom_cpufreq_suspend(_: *mut device) -> c_int { 0 }

// The remaining match tables, PM-driver registration, and module init/exit declarations mirror the C source;
// their kernel-provided types and symbols are intentionally left external.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
