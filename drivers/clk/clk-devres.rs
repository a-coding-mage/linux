// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk_bulk_data {
    _private: [u8; 0],
}

extern "C" {
    fn devres_alloc(release: unsafe extern "C" fn(*mut device, *mut c_void), size: usize, gfp: c_ulong) -> *mut c_void;
    fn devres_free(res: *mut c_void);
    fn devres_add(dev: *mut device, res: *mut c_void);
    fn devres_release(dev: *mut device, release: unsafe extern "C" fn(*mut device, *mut c_void), match_: unsafe extern "C" fn(*mut device, *mut c_void, *mut c_void), data: *mut c_void) -> c_int;
    fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_put(clk: *mut clk);
    fn clk_prepare(clk: *mut clk) -> c_int;
    fn clk_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_bulk_get(dev: *mut device, num_clks: c_int, clks: *mut clk_bulk_data) -> c_int;
    fn clk_bulk_get_optional(dev: *mut device, num_clks: c_int, clks: *mut clk_bulk_data) -> c_int;
    fn clk_bulk_put(num_clks: c_int, clks: *mut clk_bulk_data);
    fn clk_bulk_put_all(num_clks: c_int, clks: *mut clk_bulk_data);
    fn clk_bulk_prepare_enable(num_clks: c_int, clks: *mut clk_bulk_data) -> c_int;
    fn clk_bulk_disable_unprepare(num_clks: c_int, clks: *mut clk_bulk_data);
    fn devm_add_action_or_reset(dev: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> c_int;
    fn devm_clk_put(dev: *mut device, clk: *mut clk);
    fn of_clk_get_by_name(np: *mut device_node, con_id: *const c_char) -> *mut clk;
    fn warn_on(condition: bool) -> bool;
}

const GFP_KERNEL: c_ulong = 0;

#[repr(C)]
struct devm_clk_state {
    clk: *mut clk,
    exit: Option<unsafe extern "C" fn(*mut clk)>,
}

unsafe extern "C" fn devm_clk_release(_dev: *mut device, res: *mut c_void) {
    let state = res as *mut devm_clk_state;
    if let Some(exit) = (*state).exit {
        exit((*state).clk);
    }
    clk_put((*state).clk);
}

unsafe fn __devm_clk_get(
    dev: *mut device,
    id: *const c_char,
    get: unsafe extern "C" fn(*mut device, *const c_char) -> *mut clk,
    init: Option<unsafe extern "C" fn(*mut clk) -> c_int>,
    exit: Option<unsafe extern "C" fn(*mut clk)>,
) -> *mut clk {
    let state = devres_alloc(devm_clk_release, core::mem::size_of::<devm_clk_state>(), GFP_KERNEL) as *mut devm_clk_state;
    if state.is_null() { return (-12isize) as *mut clk; }
    let clk = get(dev, id);
    if (clk as isize) < 0 && (clk as isize) >= -4095 { devres_free(state as *mut c_void); return clk; }
    if let Some(init_fn) = init {
        let ret = init_fn(clk);
        if ret != 0 { clk_put(clk); devres_free(state as *mut c_void); return (ret as isize) as *mut clk; }
    }
    (*state).clk = clk;
    (*state).exit = exit;
    devres_add(dev, state as *mut c_void);
    clk
}

pub unsafe fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk { __devm_clk_get(dev, id, clk_get, None, None) }
pub unsafe fn devm_clk_get_prepared(dev: *mut device, id: *const c_char) -> *mut clk { __devm_clk_get(dev, id, clk_get, Some(clk_prepare), Some(clk_unprepare)) }
pub unsafe fn devm_clk_get_enabled(dev: *mut device, id: *const c_char) -> *mut clk { __devm_clk_get(dev, id, clk_get, Some(clk_prepare_enable), Some(clk_disable_unprepare)) }
pub unsafe fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk { __devm_clk_get(dev, id, clk_get_optional, None, None) }
pub unsafe fn devm_clk_get_optional_prepared(dev: *mut device, id: *const c_char) -> *mut clk { __devm_clk_get(dev, id, clk_get_optional, Some(clk_prepare), Some(clk_unprepare)) }
pub unsafe fn devm_clk_get_optional_enabled(dev: *mut device, id: *const c_char) -> *mut clk { __devm_clk_get(dev, id, clk_get_optional, Some(clk_prepare_enable), Some(clk_disable_unprepare)) }

unsafe extern "C" fn devm_clk_disable_unprepare(data: *mut c_void) { clk_disable_unprepare(data as *mut clk); }

pub unsafe fn devm_clk_get_optional_enabled_with_rate(dev: *mut device, id: *const c_char, rate: c_ulong) -> *mut clk {
    let clk = devm_clk_get_optional(dev, id);
    if (clk as isize) < 0 && (clk as isize) >= -4095 { return clk; }
    let mut ret = clk_set_rate(clk, rate);
    if ret == 0 { ret = clk_prepare_enable(clk); }
    if ret == 0 { ret = devm_add_action_or_reset(dev, devm_clk_disable_unprepare, clk as *mut c_void); }
    if ret != 0 { devm_clk_put(dev, clk); return (ret as isize) as *mut clk; }
    clk
}

#[repr(C)] struct clk_bulk_devres { clks: *mut clk_bulk_data, num_clks: c_int }
unsafe extern "C" fn devm_clk_bulk_release(_dev: *mut device, res: *mut c_void) { let d = res as *mut clk_bulk_devres; clk_bulk_put((*d).num_clks, (*d).clks); }
unsafe fn __devm_clk_bulk_get(dev: *mut device, n: c_int, clks: *mut clk_bulk_data, optional: bool) -> c_int {
    let d = devres_alloc(devm_clk_bulk_release, core::mem::size_of::<clk_bulk_devres>(), GFP_KERNEL) as *mut clk_bulk_devres; if d.is_null() { return -12; }
    let ret = if optional { clk_bulk_get_optional(dev,n,clks) } else { clk_bulk_get(dev,n,clks) }; if ret == 0 { (*d).clks=clks; (*d).num_clks=n; devres_add(dev,d as *mut c_void); } else { devres_free(d as *mut c_void); } ret
}
pub unsafe fn devm_clk_bulk_get(dev:*mut device,n:c_int,c:*mut clk_bulk_data)->c_int { __devm_clk_bulk_get(dev,n,c,false) }
pub unsafe fn devm_clk_bulk_get_optional(dev:*mut device,n:c_int,c:*mut clk_bulk_data)->c_int { __devm_clk_bulk_get(dev,n,c,true) }

unsafe extern "C" fn devm_clk_bulk_release_enable(_dev:*mut device,res:*mut c_void){let d=res as *mut clk_bulk_devres;clk_bulk_disable_unprepare((*d).num_clks,(*d).clks);clk_bulk_put((*d).num_clks,(*d).clks);}
unsafe fn __devm_clk_bulk_get_enable(dev:*mut device,n:c_int,c:*mut clk_bulk_data,o:bool)->c_int{let d=devres_alloc(devm_clk_bulk_release_enable,core::mem::size_of::<clk_bulk_devres>(),GFP_KERNEL)as*mut clk_bulk_devres;if d.is_null(){return -12}let mut r=if o{clk_bulk_get_optional(dev,n,c)}else{clk_bulk_get(dev,n,c)};if r!=0{devres_free(d as*mut c_void);return r}r=clk_bulk_prepare_enable(n,c);if r!=0{clk_bulk_put(n,c);devres_free(d as*mut c_void);return r}(*d).clks=c;(*d).num_clks=n;devres_add(dev,d as*mut c_void);0}
pub unsafe fn devm_clk_bulk_get_enable(d:*mut device,n:c_int,c:*mut clk_bulk_data)->c_int{__devm_clk_bulk_get_enable(d,n,c,false)}
pub unsafe fn devm_clk_bulk_get_optional_enable(d:*mut device,n:c_int,c:*mut clk_bulk_data)->c_int{__devm_clk_bulk_get_enable(d,n,c,true)}

unsafe extern "C" fn devm_clk_bulk_release_all(_dev:*mut device,res:*mut c_void){let d=res as*mut clk_bulk_devres;clk_bulk_put_all((*d).num_clks,(*d).clks);}
pub unsafe fn devm_clk_bulk_get_all(dev:*mut device,clks:*mut *mut clk_bulk_data)->c_int{let d=devres_alloc(devm_clk_bulk_release_all,core::mem::size_of::<clk_bulk_devres>(),GFP_KERNEL)as*mut clk_bulk_devres;if d.is_null(){return -12}let r=clk_bulk_get_all(dev,&mut (*d).clks);if r>0{*clks=(*d).clks;(*d).num_clks=r;devres_add(dev,d as*mut c_void)}else{devres_free(d as*mut c_void)}r}
extern "C"{fn clk_bulk_get_all(dev:*mut device,clks:*mut *mut clk_bulk_data)->c_int;}
unsafe extern "C" fn devm_clk_bulk_release_all_enable(_dev:*mut device,res:*mut c_void){let d=res as*mut clk_bulk_devres;clk_bulk_disable_unprepare((*d).num_clks,(*d).clks);clk_bulk_put_all((*d).num_clks,(*d).clks);}
pub unsafe fn devm_clk_bulk_get_all_enabled(dev:*mut device,clks:*mut *mut clk_bulk_data)->c_int{let d=devres_alloc(devm_clk_bulk_release_all_enable,core::mem::size_of::<clk_bulk_devres>(),GFP_KERNEL)as*mut clk_bulk_devres;if d.is_null(){return -12}let r=clk_bulk_get_all(dev,&mut (*d).clks);if r<=0{devres_free(d as*mut c_void);return r}*clks=(*d).clks;(*d).num_clks=r;let e=clk_bulk_prepare_enable(r,*clks);if e!=0{clk_bulk_put_all(r,(*d).clks);devres_free(d as*mut c_void);return e}devres_add(dev,d as*mut c_void);(*d).num_clks}

unsafe extern "C" fn devm_clk_match(_dev:*mut device,res:*mut c_void,data:*mut c_void)->c_int{let c=res as*mut *mut clk;if c.is_null()||(*c).is_null(){return 0}if *c==data as*mut clk{1}else{0}}
pub unsafe fn devm_clk_put(dev:*mut device,clk:*mut clk){let r=devres_release(dev,devm_clk_release,devm_clk_match,clk as*mut c_void);let _=warn_on(r!=0);}
pub unsafe fn devm_get_clk_from_child(dev:*mut device,np:*mut device_node,con_id:*const c_char)->*mut clk{let s=devres_alloc(devm_clk_release,core::mem::size_of::<devm_clk_state>(),GFP_KERNEL)as*mut devm_clk_state;if s.is_null(){return(-12isize)as*mut clk}let c=of_clk_get_by_name(np,con_id);if !((c as isize)<0&&(c as isize)>=-4095){(*s).clk=c;devres_add(dev,s as*mut c_void)}else{devres_free(s as*mut c_void)}c}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
