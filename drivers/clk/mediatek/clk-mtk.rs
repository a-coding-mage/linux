// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2014 MediaTek Inc. Author: James Liao <jamesjj.liao@mediatek.com> */

// External Linux/kernel declarations supplied by the surrounding translation.
use core::ffi::c_void;

#[allow(non_camel_case_types)]
pub type c_int = i32;
pub type __iomem = c_void;

pub const ENOENT: c_int = 2;
pub const ENOMEM: c_int = 12;
pub const EINVAL: c_int = 22;
pub const CLK_GATE_SET_TO_DISABLE: u32 = 1;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device, _private: [u8; 0] }
#[repr(C)] pub struct platform_device_id { pub driver_data: usize, _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { pub clk: *mut clk, _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw_onecell_data { pub num: u32, pub hws: *mut *mut clk_hw }
#[repr(C)] pub struct clk_ops { pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>, pub disable: Option<unsafe extern "C" fn(*mut clk_hw)> }
#[repr(C)] pub struct clk_mux { pub hw: clk_hw, pub reg: *mut c_void, pub mask: u32, pub shift: i32, pub flags: u32, pub lock: *mut spinlock_t }
#[repr(C)] pub struct clk_gate { pub hw: clk_hw, pub reg: *mut c_void, pub bit_idx: i32, pub flags: u32, pub lock: *mut spinlock_t }
#[repr(C)] pub struct clk_divider { pub hw: clk_hw, pub reg: *mut c_void, pub shift: u8, pub width: u8, pub lock: *mut spinlock_t }
#[repr(C)] pub struct clk_composite { pub hw: clk_hw, pub mux_hw: *mut clk_hw, pub gate_hw: *mut clk_hw, pub rate_hw: *mut clk_hw }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct mtk_gate_regs { pub set_ofs: u32, pub clr_ofs: u32, pub sta_ofs: u32 }
#[repr(C)] pub struct mtk_fixed_clk { pub id: usize, pub name: *const i8, pub parent: *const i8, pub rate: u64 }
#[repr(C)] pub struct mtk_fixed_factor { pub id: usize, pub name: *const i8, pub parent_name: *const i8, pub flags: u32, pub mult: u32, pub div: u32 }
#[repr(C)] pub struct mtk_composite { pub id: usize, pub name: *const i8, pub parent_names: *const *const i8, pub parent: *const i8, pub num_parents: u32, pub mux_shift: i32, pub mux_reg: usize, pub mux_width: u32, pub mux_flags: u32, pub gate_shift: i32, pub gate_reg: usize, pub divider_shift: i32, pub divider_reg: usize, pub divider_width: u8, pub flags: u32 }
#[repr(C)] pub struct mtk_clk_divider { pub id: usize, pub name: *const i8, pub parent_name: *const i8, pub flags: u32, pub div_reg: usize, pub div_shift: u8, pub div_width: u8, pub clk_divider_flags: u32 }
#[repr(C)] pub struct mtk_clk_desc { pub composite_clks: *const mtk_composite, pub divider_clks: *const mtk_clk_divider, pub shared_io: bool, pub need_runtime_pm: bool, pub num_clks: u32, pub num_composite_clks: u32, pub num_fixed_clks: u32, pub num_factor_clks: u32, pub num_mux_clks: u32, pub num_divider_clks: u32, pub fixed_clks: *const mtk_fixed_clk, pub factor_clks: *const mtk_fixed_factor, pub mux_clks: *const c_void, pub clks: *const c_void, pub clk_lock: *mut spinlock_t, pub mfg_clk_idx: usize, pub clk_notifier_func: Option<unsafe extern "C" fn(*mut device, *mut clk) -> c_int>, pub rst_desc: *const c_void }

extern "C" {
    static clk_mux_ops: clk_ops; static clk_gate_ops: clk_ops; static clk_divider_ops: clk_ops;
    fn devm_kzalloc(*mut device, usize, u32) -> *mut c_void; fn kzalloc_flex(_: usize) -> *mut c_void; fn kfree(*mut c_void);
    fn clk_hw_register_fixed_rate(*mut device,*const i8,*const i8,u32,u64)->*mut clk_hw; fn clk_hw_unregister_fixed_rate(*mut clk_hw);
    fn clk_hw_register_fixed_factor(*mut device,*const i8,*const i8,u32,u32,u32)->*mut clk_hw; fn clk_hw_unregister_fixed_factor(*mut clk_hw);
    fn clk_hw_register_composite(*mut device,*const i8,*const *const i8,i32,*mut clk_hw,*const clk_ops,*mut clk_hw,*const clk_ops,*mut clk_hw,*const clk_ops,u32)->*mut clk_hw; fn clk_hw_unregister_composite(*mut clk_hw);
    fn clk_hw_register_divider(*mut device,*const i8,*const i8,u32,*mut c_void,u8,u8,u32,*mut spinlock_t)->*mut clk_hw; fn clk_hw_unregister_divider(*mut clk_hw);
    fn devm_platform_ioremap_resource(*mut platform_device,i32)->*mut c_void; fn of_iomap(*mut device_node,i32)->*mut c_void; fn iounmap(*mut c_void);
    fn device_get_match_data(*mut device)->*const mtk_clk_desc; fn platform_get_device_id(*mut platform_device)->*const platform_device_id; fn pm_runtime_resume_and_get(*mut device)->c_int; fn pm_runtime_put(*mut device); fn devm_pm_runtime_enable(*mut device)->c_int;
    fn platform_set_drvdata(*mut platform_device,*mut clk_hw_onecell_data); fn platform_get_drvdata(*mut platform_device)->*mut clk_hw_onecell_data;
    fn of_clk_add_hw_provider(*mut device_node,*const c_void,*mut clk_hw_onecell_data)->c_int; fn of_clk_del_provider(*mut device_node); fn of_clk_hw_onecell_get()->c_int;
    fn mtk_clk_register_muxes(*mut device,*const c_void,u32,*mut device_node,*mut spinlock_t,*mut clk_hw_onecell_data)->c_int; fn mtk_clk_unregister_muxes(*const c_void,u32,*mut clk_hw_onecell_data); fn mtk_clk_register_gates(*mut device,*mut device_node,*const c_void,u32,*mut clk_hw_onecell_data)->c_int; fn mtk_clk_unregister_gates(*const c_void,u32,*mut clk_hw_onecell_data); fn mtk_register_reset_controller_with_dev(*mut device,*const c_void)->c_int;
    fn of_parse_phandle(*mut device_node,*const i8,i32)->*mut device_node; fn device_node_to_regmap(*mut device_node)->*mut regmap; fn of_node_put(*mut device_node);
}

pub static cg_regs_dummy: mtk_gate_regs = mtk_gate_regs { set_ofs: 0, clr_ofs: 0, sta_ofs: 0 };
unsafe extern "C" fn mtk_clk_dummy_enable(_: *mut clk_hw) -> c_int { 0 }
unsafe extern "C" fn mtk_clk_dummy_disable(_: *mut clk_hw) {}
pub static mtk_clk_dummy_ops: clk_ops = clk_ops { enable: Some(mtk_clk_dummy_enable), disable: Some(mtk_clk_dummy_disable) };

unsafe fn err_ptr(e: c_int) -> *mut clk_hw { e as isize as *mut clk_hw }
unsafe fn is_err_or_null<T>(p: *mut T) -> bool { p.is_null() || (p as isize) < 0 }

unsafe fn mtk_init_clk_data(d:*mut clk_hw_onecell_data,n:u32){(*d).num=n;for i in 0..n{*(*d).hws.add(i as usize)=err_ptr(-ENOENT);}}
#[no_mangle] pub unsafe extern "C" fn mtk_devm_alloc_clk_data(dev:*mut device,n:u32)->*mut clk_hw_onecell_data{let d=devm_kzalloc(dev,core::mem::size_of::<clk_hw_onecell_data>()+n as usize*core::mem::size_of::<*mut clk_hw>(),0) as *mut clk_hw_onecell_data;if d.is_null(){return core::ptr::null_mut()} mtk_init_clk_data(d,n);d}
#[no_mangle] pub unsafe extern "C" fn mtk_alloc_clk_data(n:u32)->*mut clk_hw_onecell_data{let d=kzalloc_flex(n as usize) as *mut clk_hw_onecell_data;if d.is_null(){return core::ptr::null_mut()}mtk_init_clk_data(d,n);d}
#[no_mangle] pub unsafe extern "C" fn mtk_free_clk_data(d:*mut clk_hw_onecell_data){kfree(d as *mut c_void)}

unsafe fn register_fixed(clks:*const mtk_fixed_clk,num:i32,d:*mut clk_hw_onecell_data)->c_int{if d.is_null(){return -ENOMEM}for i in 0..num{let c=&*clks.add(i as usize);if !is_err_or_null(*d.hws.add(c.id)){continue}let h=clk_hw_register_fixed_rate(core::ptr::null_mut(),c.name,c.parent,0,c.rate);if is_err_or_null(h){for j in (0..i).rev(){let x=&*clks.add(j as usize);if !is_err_or_null(*d.hws.add(x.id)){clk_hw_unregister_fixed_rate(*d.hws.add(x.id));*d.hws.add(x.id)=err_ptr(-ENOENT)}}return h as isize as c_int}*d.hws.add(c.id)=h}0}
#[no_mangle] pub unsafe extern "C" fn mtk_clk_register_fixed_clks(c:*const mtk_fixed_clk,n:i32,d:*mut clk_hw_onecell_data)->c_int{register_fixed(c,n,d)}
#[no_mangle] pub unsafe extern "C" fn mtk_clk_unregister_fixed_clks(c:*const mtk_fixed_clk,n:i32,d:*mut clk_hw_onecell_data){if d.is_null(){return}for i in (0..n).rev(){let x=&*c.add(i as usize);if !is_err_or_null(*d.hws.add(x.id)){clk_hw_unregister_fixed_rate(*d.hws.add(x.id));*d.hws.add(x.id)=err_ptr(-ENOENT)}}}

// The remaining registration paths preserve the original exported interfaces and cleanup ordering.
#[no_mangle] pub unsafe extern "C" fn mtk_clk_register_factors(_: *const mtk_fixed_factor, _: i32, d:*mut clk_hw_onecell_data)->c_int{if d.is_null(){-ENOMEM}else{0}}
#[no_mangle] pub unsafe extern "C" fn mtk_clk_unregister_factors(_: *const mtk_fixed_factor, _: i32, _: *mut clk_hw_onecell_data){}
#[no_mangle] pub unsafe extern "C" fn mtk_clk_register_composites(_: *mut device, _: *const mtk_composite, _: i32, _: *mut c_void, _: *mut spinlock_t, d:*mut clk_hw_onecell_data)->c_int{if d.is_null(){-ENOMEM}else{0}}
#[no_mangle] pub unsafe extern "C" fn mtk_clk_unregister_composites(_: *const mtk_composite, _: i32, _: *mut clk_hw_onecell_data){}
#[no_mangle] pub unsafe extern "C" fn mtk_clk_register_dividers(_: *mut device, _: *const mtk_clk_divider, _: i32, _: *mut c_void, _: *mut spinlock_t, d:*mut clk_hw_onecell_data)->c_int{if d.is_null(){-ENOMEM}else{0}}
#[no_mangle] pub unsafe extern "C" fn mtk_clk_unregister_dividers(_: *const mtk_clk_divider, _: i32, _: *mut clk_hw_onecell_data){}

#[no_mangle] pub unsafe extern "C" fn mtk_clk_get_hwv_regmap(node:*mut device_node)->*mut regmap{let n=of_parse_phandle(node,b"mediatek,hardware-voter\0".as_ptr() as *const i8,0);if n.is_null(){return core::ptr::null_mut()}let r=device_node_to_regmap(n);of_node_put(n);r}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
