// SPDX-License-Identifier: GPL-2.0-only
/* OMAP3/OMAP4 Voltage Management Routines */

// Linux headers and the declarations supplied by the surrounding OMAP code
// are intentionally external dependencies of this translation.

use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    static mut voltdm_list: list_head;
    fn list_empty(head: *const list_head) -> bool;
    fn list_first_voltdm(head: *const list_head) -> *mut voltagedomain;
    fn list_next_voltdm(item: *mut voltagedomain) -> *mut voltagedomain;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_notice(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn clk_get(dev: *mut c_void, name: *const c_char) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_put(clk: *mut clk);
    fn omap_vc_bypass_scale(v: *mut voltagedomain, volt: c_ulong) -> c_int;
    fn omap_vc_init_channel(v: *mut voltagedomain);
    fn omap_vp_forceupdate_scale(v: *mut voltagedomain, volt: c_ulong) -> c_int;
    fn omap_vp_init(v: *mut voltagedomain);
    fn is_err_or_null<T>(p: *const T) -> bool;
    fn err_ptr<T>(err: c_int) -> *mut T;
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct clk;
#[repr(C)] pub struct omap_volt_data { pub volt_nominal: c_ulong }
#[repr(C)] pub struct omap_voltdm_pmic;
#[repr(C)] pub struct clock_data { pub name: *const c_char, pub rate: c_ulong }
#[repr(C)] pub struct voltagedomain {
    pub node: list_head,
    pub name: *const c_char,
    pub nominal_volt: c_ulong,
    pub scale: Option<unsafe extern "C" fn(*mut voltagedomain, c_ulong) -> c_int>,
    pub volt_data: *mut omap_volt_data,
    pub pmic: *mut omap_voltdm_pmic,
    pub scalable: bool,
    pub sys_clk: clock_data,
    pub vc: *mut c_void,
    pub vp: *mut c_void,
}

pub unsafe fn voltdm_get_voltage(voltdm: *mut voltagedomain) -> c_ulong {
    if is_err_or_null(voltdm) { pr_warn(c"%s: VDD specified does not exist!\n".as_ptr(), c"voltdm_get_voltage".as_ptr()); return 0; }
    (*voltdm).nominal_volt
}

unsafe fn voltdm_scale(voltdm: *mut voltagedomain, target_volt: c_ulong) -> c_int {
    if is_err_or_null(voltdm) { return -22; }
    if (*voltdm).scale.is_none() { return -61; }
    if (*voltdm).volt_data.is_null() { return -61; }
    let mut i = 0usize; let mut volt = 0;
    while (*(*voltdm).volt_data.add(i)).volt_nominal != 0 {
        if (*(*voltdm).volt_data.add(i)).volt_nominal >= target_volt { volt = (*(*voltdm).volt_data.add(i)).volt_nominal; break; }
        i += 1;
    }
    if volt == 0 { return -22; }
    let ret = ((*voltdm).scale.unwrap())(voltdm, volt);
    if ret == 0 { (*voltdm).nominal_volt = volt; }
    ret
}

pub unsafe fn voltdm_reset(voltdm: *mut voltagedomain) {
    let target_volt = voltdm_get_voltage(voltdm); if target_volt != 0 { voltdm_scale(voltdm, target_volt); }
}

pub unsafe fn omap_voltage_get_volttable(voltdm: *mut voltagedomain, volt_data: *mut *mut omap_volt_data) {
    if is_err_or_null(voltdm) { return; } *volt_data = (*voltdm).volt_data;
}

pub unsafe fn omap_voltage_get_voltdata(voltdm: *mut voltagedomain, volt: c_ulong) -> *mut omap_volt_data {
    if is_err_or_null(voltdm) { return err_ptr(-22); }
    if (*voltdm).volt_data.is_null() { return err_ptr(-61); }
    let mut i = 0usize; while (*(*voltdm).volt_data.add(i)).volt_nominal != 0 { if (*(*voltdm).volt_data.add(i)).volt_nominal == volt { return (*voltdm).volt_data.add(i); } i += 1; }
    err_ptr(-61)
}

pub unsafe fn omap_voltage_register_pmic(voltdm: *mut voltagedomain, pmic: *mut omap_voltdm_pmic) -> c_int {
    if is_err_or_null(voltdm) { return -22; } (*voltdm).pmic = pmic; 0
}

pub unsafe fn omap_voltage_late_init() -> c_int {
    if list_empty(&voltdm_list) { return -22; }
    let mut v = list_first_voltdm(&voltdm_list);
    while !v.is_null() { if (*v).scalable { let ck = clk_get(core::ptr::null_mut(), (*v).sys_clk.name); if ck.is_null() { return -22; } (*v).sys_clk.rate = clk_get_rate(ck); clk_put(ck); if !(*v).vc.is_null() { (*v).scale = Some(omap_vc_bypass_scale); omap_vc_init_channel(v); } if !(*v).vp.is_null() { (*v).scale = Some(omap_vp_forceupdate_scale); omap_vp_init(v); } } v = list_next_voltdm(v); } 0
}

unsafe fn _voltdm_lookup(name: *const c_char) -> *mut voltagedomain { let mut v = list_first_voltdm(&voltdm_list); while !v.is_null() { if strcmp(name, (*v).name) == 0 { return v; } v = list_next_voltdm(v); } core::ptr::null_mut() }
unsafe fn _voltdm_register(v: *mut voltagedomain) -> c_int { if v.is_null() || (*v).name.is_null() { return -22; } list_add(&mut (*v).node, &mut voltdm_list); 0 }
pub unsafe fn voltdm_lookup(name: *const c_char) -> *mut voltagedomain { if name.is_null() { core::ptr::null_mut() } else { _voltdm_lookup(name) } }
pub unsafe fn voltdm_init(voltdms: *mut *mut voltagedomain) { if !voltdms.is_null() { let mut v = voltdms; while !(*v).is_null() { _voltdm_register(*v); v = v.add(1); } } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
