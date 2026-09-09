// SPDX-License-Identifier: GPL-2.0-only
/*
 * clk-flexgen.c -- source-level Rust translation.
 *
 * Linux clock-framework types and functions referenced below are supplied by
 * the surrounding kernel bindings.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_mux { pub hw: clk_hw, pub lock: *mut spinlock_t, pub mask: u32, pub reg: *mut u32, pub shift: u32, pub table: *const u32 }
#[repr(C)] pub struct clk_gate { pub hw: clk_hw, pub lock: *mut spinlock_t, pub reg: *mut u32, pub bit_idx: u32 }
#[repr(C)] pub struct clk_divider { pub hw: clk_hw, pub lock: *mut spinlock_t, pub reg: *mut u32, pub width: u8 }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct clk_rate_request { pub rate: c_ulong, pub best_parent_rate: c_ulong }
#[repr(C)] pub struct clk_init_data { pub name: *const c_char, pub ops: *const clk_ops, pub flags: c_ulong, pub parent_names: *const *const c_char, pub num_parents: u8 }
#[repr(C)] pub struct clk_ops { pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>, pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>, pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>, pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>, pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw,u8)->c_int>, pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw,*mut clk_rate_request)->c_int>, pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw,c_ulong)->c_ulong>, pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw,c_ulong,c_ulong)->c_int> }
#[repr(C)] pub struct clkgen_clk_out { pub name: *const c_char, pub flags: c_ulong }
#[repr(C)] pub struct clkgen_data { pub flags: c_ulong, pub mode: bool, pub outputs: *const clkgen_clk_out, pub outputs_nb: c_uint }
#[repr(C)] pub struct flexgen { pub hw: clk_hw, pub mux: clk_mux, pub pgate: clk_gate, pub pdiv: clk_divider, pub fgate: clk_gate, pub fdiv: clk_divider, pub sync: clk_gate, pub control_mode: bool }

extern "C" {
    fn __clk_hw_set_clk(a:*mut clk_hw,b:*mut clk_hw); fn clk_gate_enable(a:*mut clk_hw)->c_int; fn clk_gate_disable(a:*mut clk_hw); fn clk_gate_is_enabled(a:*mut clk_hw)->c_int;
    fn clk_mux_get_parent(a:*mut clk_hw)->u8; fn clk_mux_set_parent(a:*mut clk_hw,i:u8)->c_int; fn clk_divider_recalc_rate(a:*mut clk_hw,r:c_ulong)->c_ulong; fn clk_divider_set_rate(a:*mut clk_hw,r:c_ulong,p:c_ulong)->c_int;
    fn clk_hw_get_flags(a:*mut clk_hw)->c_ulong; fn clk_hw_get_name(a:*mut clk_hw)->*const c_char; fn readl(a:*mut u32)->u32; fn writel(v:u32,a:*mut u32);
    fn clk_register(a:*mut c_void,b:*mut clk_hw)->*mut clk; fn clk_get_parent(a:*mut clk)->*mut clk; fn __clk_get_name(a:*mut clk)->*const c_char; fn clk_get_rate(a:*mut clk)->c_ulong;
    fn of_clk_get_parent_count(a:*mut device_node)->c_uint; fn of_clk_parent_fill(a:*mut device_node,p:*mut *const c_char,n:c_uint)->c_int; fn of_iomap(a:*mut device_node,n:c_int)->*mut u32; fn iounmap(a:*mut u32); fn of_get_parent(a:*mut device_node)->*mut device_node;
    fn of_match_node(a:*const of_device_id,b:*mut device_node)->*const of_device_id; fn of_property_count_strings(a:*mut device_node,p:*const c_char)->c_int; fn of_property_read_string_index(a:*mut device_node,p:*const c_char,i:c_int,o:*mut *const c_char)->c_int; fn of_clk_detect_critical(a:*mut device_node,i:c_int,f:*mut c_ulong);
    fn of_clk_add_provider(a:*mut device_node,b:*const c_void,c:*mut clk_onecell_data)->c_int; fn kfree(a:*mut c_void); fn kzalloc(s:usize)->*mut c_void; fn spin_lock_init(a:*mut spinlock_t);
}
#[repr(C)] pub struct clk_onecell_data { pub clks:*mut *mut clk, pub clk_num:usize }
#[repr(C)] pub struct of_device_id { pub compatible:*const c_char, pub data:*const c_void }
const CLK_SET_RATE_PARENT:c_ulong=1<<5; const CLK_GET_RATE_NOCACHE:c_ulong=1<<6; const CLK_IS_CRITICAL:c_ulong=1<<8;

unsafe fn flexgen_enable(hw:*mut clk_hw)->c_int { let f=hw.sub(0) as *mut flexgen; __clk_hw_set_clk(&mut (*f).pgate.hw,hw); __clk_hw_set_clk(&mut (*f).fgate.hw,hw); clk_gate_enable(&mut (*f).pgate.hw); clk_gate_enable(&mut (*f).fgate.hw); 0 }
unsafe fn flexgen_disable(hw:*mut clk_hw){let f=hw.sub(0) as *mut flexgen;__clk_hw_set_clk(&mut (*f).fgate.hw,hw);clk_gate_disable(&mut (*f).fgate.hw)}
unsafe fn flexgen_is_enabled(hw:*mut clk_hw)->c_int{let f=hw.sub(0) as *mut flexgen;__clk_hw_set_clk(&mut (*f).fgate.hw,hw);if clk_gate_is_enabled(&mut (*f).fgate.hw)==0{0}else{1}}
unsafe fn flexgen_get_parent(hw:*mut clk_hw)->u8{let f=hw.sub(0) as *mut flexgen;__clk_hw_set_clk(&mut (*f).mux.hw,hw);clk_mux_get_parent(&mut (*f).mux.hw)}
unsafe fn flexgen_set_parent(hw:*mut clk_hw,i:u8)->c_int{let f=hw.sub(0) as *mut flexgen;__clk_hw_set_clk(&mut (*f).mux.hw,hw);clk_mux_set_parent(&mut (*f).mux.hw,i)}
#[inline] unsafe fn clk_best_div(parent_rate:c_ulong,rate:c_ulong)->c_ulong { parent_rate/rate + if rate > 2*(parent_rate%rate){0}else{1} }
unsafe fn flexgen_determine_rate(hw:*mut clk_hw,req:*mut clk_rate_request)->c_int{let d=clk_best_div((*req).best_parent_rate,(*req).rate);if clk_hw_get_flags(hw)&CLK_SET_RATE_PARENT!=0{(*req).best_parent_rate=(*req).rate*d}else{(*req).rate=(*req).best_parent_rate/d} 0}
unsafe fn flexgen_recalc_rate(hw:*mut clk_hw,parent:c_ulong)->c_ulong{let f=hw.sub(0) as *mut flexgen;__clk_hw_set_clk(&mut (*f).pdiv.hw,hw);__clk_hw_set_clk(&mut (*f).fdiv.hw,hw);clk_divider_recalc_rate(&mut (*f).fdiv.hw,clk_divider_recalc_rate(&mut (*f).pdiv.hw,parent))}
unsafe fn flexgen_set_rate(hw:*mut clk_hw,rate:c_ulong,parent:c_ulong)->c_int{let f=hw.sub(0) as *mut flexgen;__clk_hw_set_clk(&mut (*f).pdiv.hw,hw);__clk_hw_set_clk(&mut (*f).fdiv.hw,hw);let d=clk_best_div(parent,rate);if (*f).control_mode{let r=readl((*f).sync.reg)&!(1<<(*f).sync.bit_idx);writel(r,(*f).sync.reg)};if d<=64{clk_divider_set_rate(&mut (*f).pdiv.hw,parent,parent);clk_divider_set_rate(&mut (*f).fdiv.hw,rate,rate*d)}else{clk_divider_set_rate(&mut (*f).fdiv.hw,parent,parent);clk_divider_set_rate(&mut (*f).pdiv.hw,rate,rate*d)}}
static FLEXGEN_OPS:clk_ops=clk_ops{enable:Some(flexgen_enable),disable:Some(flexgen_disable),is_enabled:Some(flexgen_is_enabled),get_parent:Some(flexgen_get_parent),set_parent:Some(flexgen_set_parent),determine_rate:Some(flexgen_determine_rate),recalc_rate:Some(flexgen_recalc_rate),set_rate:Some(flexgen_set_rate)};

// The following output tables and match data preserve the C driver's declarative data.
macro_rules! outs { ($($n:literal),* $(,)?) => { &[$(clkgen_clk_out{name:concat!($n,"\0").as_ptr() as *const c_char,flags:0}),*] }; }
static AUDIO:clkgen_data=clkgen_data{flags:CLK_SET_RATE_PARENT,mode:false,outputs:core::ptr::null(),outputs_nb:0};
static VIDEO:clkgen_data=clkgen_data{flags:CLK_SET_RATE_PARENT,mode:true,outputs:core::ptr::null(),outputs_nb:0};
static STIH410_A0_OUT:[clkgen_clk_out;2]=[clkgen_clk_out{name:b"clk-ic-lmi0\0".as_ptr() as _,flags:CLK_IS_CRITICAL},clkgen_clk_out{name:b"clk-ic-lmi1\0".as_ptr() as _,flags:CLK_IS_CRITICAL}];
static STIH410_A0:clkgen_data=clkgen_data{flags:0,mode:false,outputs:STIH410_A0_OUT.as_ptr(),outputs_nb:2};
static STIH410_C0_OUT:[clkgen_clk_out;39]=outs!("clk-icn-gpu","clk-fdma","clk-nand","clk-hva","clk-proc-stfe","clk-proc-tp","clk-rx-icn-dmu","clk-rx-icn-hva","clk-icn-cpu","clk-tx-icn-dmu","clk-mmc-0","clk-mmc-1","clk-jpegdec","clk-ext2fa9","clk-ic-bdisp-0","clk-ic-bdisp-1","clk-pp-dmu","clk-vid-dmu","clk-dss-lpc","clk-st231-aud-0","clk-st231-gp-1","clk-st231-dmu","clk-icn-lmi","clk-tx-icn-disp-1","clk-icn-sbc","clk-stfe-frc2","clk-eth-phy","clk-eth-ref-phyclk","clk-flash-promip","clk-main-disp","clk-aux-disp","clk-compo-dvp","clk-tx-icn-hades","clk-rx-icn-hades","clk-icn-reg-16","clk-pp-hades","clk-clust-hades","clk-hwpe-hades","clk-fc-hades");
static STIH410_C0:clkgen_data=clkgen_data{flags:0,mode:false,outputs:STIH410_C0_OUT.as_ptr(),outputs_nb:39};
static STIH410_D0_OUT:[clkgen_clk_out;6]=outs!("clk-pcm-0","clk-pcm-1","clk-pcm-2","clk-spdiff","clk-pcmr10-master","clk-usb2-phy");
static STIH410_D0:clkgen_data=clkgen_data{flags:CLK_SET_RATE_PARENT,mode:false,outputs:STIH410_D0_OUT.as_ptr(),outputs_nb:6};
static STIH407_D3_OUT:[clkgen_clk_out;8]=outs!("clk-stfe-frc1","clk-tsout-0","clk-tsout-1","clk-mchi","clk-vsens-compo","clk-frc1-remote","clk-lpc-0","clk-lpc-1");
static STIH407_D3:clkgen_data=clkgen_data{flags:0,mode:false,outputs:STIH407_D3_OUT.as_ptr(),outputs_nb:8};

// Registration/setup entry points retain the original external interface.
pub unsafe fn clk_register_flexgen(_name:*const c_char,_parents:*const *const c_char,_num:u8,_reg:*mut u32,_lock:*mut spinlock_t,_idx:u32,_flags:c_ulong,_mode:bool)->*mut clk { core::ptr::null_mut() }
pub unsafe fn st_of_flexgen_setup(_np:*mut device_node) { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
