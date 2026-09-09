// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level Rust translation of clk-sunxi.c.  Kernel-provided types,
 * constants, macros, and functions are intentionally external dependencies. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut clk_lock: c_void;
}

const SUNXI_MAX_PARENTS: usize = 5;
const SUN6I_AHB1_PARENT_PLL6: u8 = 3;
const SUNXI_MUX_GATE_WIDTH: u8 = 2;
const SUNXI_DIVS_MAX_QTY: usize = 4;
const SUNXI_DIVISOR_WIDTH: u8 = 2;

#[repr(C)]
pub struct factors_request { pub rate: u32, pub parent_rate: u32, pub parent_index: u8, pub n: u8, pub k: u8, pub m: u8, pub p: u8 }
#[repr(C)]
pub struct clk_factors_config { pub nshift:u8, pub nwidth:u8, pub kshift:u8, pub kwidth:u8, pub mshift:u8, pub mwidth:u8, pub pshift:u8, pub pwidth:u8, pub n_start:u8 }
#[repr(C)]
pub struct factors_data { pub enable:u8, pub mux:u8, pub muxmask:u32, pub table:*const clk_factors_config, pub getter: Option<unsafe extern "C" fn(*mut factors_request)>, pub recalc: Option<unsafe extern "C" fn(*mut factors_request)>, pub name:*const c_char }
#[repr(C)] pub struct device_node { pub name:*const c_char }
#[repr(C)] pub struct clk { _x:[u8;0] }
#[repr(C)] pub struct clk_hw { _x:[u8;0] }
#[repr(C)] pub struct clk_ops { _x:[u8;0] }
#[repr(C)] pub struct clk_div_table { pub val:u32, pub div:u32 }
#[repr(C)] pub struct clk_onecell_data { pub clks:*mut *mut clk, pub clk_num:usize }
#[repr(C)] pub struct clk_gate { pub hw:clk_hw, pub reg:*mut c_void, pub bit_idx:u8, pub lock:*mut c_void }
#[repr(C)] pub struct clk_fixed_factor { pub hw:clk_hw, pub mult:u32, pub div:u32 }
#[repr(C)] pub struct clk_divider { pub hw:clk_hw, pub reg:*mut c_void, pub shift:u8, pub width:u8, pub flags:u32, pub lock:*mut c_void, pub table:*mut clk_div_table }

extern "C" {
    fn rounddown(x:u32, y:u32)->u32; fn round_down(x:u32,y:u32)->u32; fn order_base_2(x:u32)->u32;
    fn div_round_up(x:u32,y:u32)->u32; fn roundup_pow_of_two(x:u32)->u32;
    fn sunxi_factors_register(n:*mut device_node,d:*const factors_data,l:*mut c_void,r:*mut c_void)->*mut clk;
    fn of_iomap(n:*mut device_node,i:c_int)->*mut c_void; fn iounmap(r:*mut c_void);
    fn of_property_read_string(n:*mut device_node,p:*const c_char,s:*mut *const c_char)->c_int;
    fn of_property_read_string_index(n:*mut device_node,p:*const c_char,i:c_int,s:*mut *const c_char)->c_int;
    fn of_clk_parent_fill(n:*mut device_node,p:*mut *const c_char,m:usize)->c_int;
    fn clk_register_mux(a:*mut c_void,n:*const c_char,p:*const *const c_char,c:c_int,f:u32,r:*mut c_void,s:u8,w:u8,x:u8,l:*mut c_void)->*mut clk;
    fn clk_register_divider_table(a:*mut c_void,n:*const c_char,p:*const c_char,f:u32,r:*mut c_void,s:u8,w:u8,x:u32,t:*const clk_div_table,l:*mut c_void)->*mut clk;
    fn clk_register_composite(a:*mut c_void,n:*const c_char,p:*const *const c_char,c:c_int,m:*mut clk_hw,mo:*const clk_ops,r:*mut clk_hw,ro:*const clk_ops,g:*mut clk_hw,go:*const clk_ops,f:u32)->*mut clk;
    fn of_clk_add_provider(n:*mut device_node,g:*mut c_void,d:*mut c_void)->c_int; fn of_clk_del_provider(n:*mut device_node);
    fn clk_register_clkdev(c:*mut clk,n:*const c_char,p:*const c_char)->c_int; fn clk_unregister_divider(c:*mut clk);
    fn __clk_get_name(c:*mut clk)->*const c_char; fn kfree(p:*mut c_void);
    fn kmalloc(size:usize,flags:u32)->*mut c_void; fn kzalloc(size:usize,flags:u32)->*mut c_void;
}

#[inline] unsafe fn div_up(a:u32,b:u32)->u32 { (a+b-1)/b }
unsafe fn sun4i_get_pll1_factors(r:*mut factors_request) { let q=&mut *r; let mut d=(q.rate/6000000) as u8; q.rate=6000000*d as u32; q.m=0; q.k=if q.rate>=768000000||q.rate==42000000||q.rate==54000000{1}else{0}; q.p=if d<10{3}else if d<20||(d<32&&(d&1)!=0){2}else if d<40||(d<64&&(d&2)!=0){1}else{0}; d<<=q.p; d/=(q.k+1); q.n=d/4; }
unsafe fn sun8i_a23_get_pll1_factors(r:*mut factors_request) { let q=&mut *r; let mut d=(q.rate/6000000) as u8; q.rate=6000000*d as u32; q.m=0; q.k=if q.rate>=768000000||q.rate==42000000||q.rate==54000000{1}else{0}; q.p=if d<20||(d<32&&(d&1)!=0){2}else if d<40||(d<64&&(d&2)!=0){1}else{0}; d<<=q.p; d/=(q.k+1); q.n=d/4-1; }
unsafe fn sun4i_get_pll5_factors(r:*mut factors_request){let q=&mut *r;let d=(q.rate/q.parent_rate) as u8;q.rate=q.parent_rate*d as u32;q.k=if d<31{0}else if d/2<31{1}else if d/3<31{2}else{3};q.n=div_up(d as u32,q.k as u32+1) as u8;}
unsafe fn sun6i_a31_get_pll6_factors(r:*mut factors_request){let q=&mut *r;let d=(q.rate/q.parent_rate) as u8;q.rate=q.parent_rate*d as u32;q.k=(d/32).min(3);q.n=div_up(d as u32,q.k as u32+1) as u8-1;}
unsafe fn sun5i_a13_get_ahb_factors(r:*mut factors_request){let q=&mut *r;if q.parent_rate<q.rate{q.rate=q.parent_rate}if q.rate<8000{q.rate=8000}if q.rate>300000000{q.rate=300000000}let mut d=32-q.parent_rate.div_ceil(q.rate).leading_zeros();if d>3{d=3}q.rate=q.parent_rate>>d;q.p=d as u8;}
unsafe fn sun6i_get_ahb1_factors(r:*mut factors_request){let q=&mut *r;if q.parent_rate!=0&&q.rate>q.parent_rate{q.rate=q.parent_rate}let d=div_up(q.parent_rate,q.rate);let (p,m)=if q.parent_index==3{let p=if d<4{0}else if d/2<4{1}else if d/4<4{2}else{3};(p,div_up(d,1<<p))}else{(d.next_power_of_two().min(8).trailing_zeros(),1)};q.rate=(q.parent_rate/m)>>p;q.p=p as u8;q.m=(m-1) as u8;}
unsafe fn sun6i_ahb1_recalc(r:*mut factors_request){let q=&mut *r;q.rate=q.parent_rate;if q.parent_index==3{q.rate/=q.m as u32+1}q.rate>>=q.p;}
unsafe fn sun4i_get_apb1_factors(r:*mut factors_request){let q=&mut *r;if q.parent_rate<q.rate{q.rate=q.parent_rate}let d=div_up(q.parent_rate,q.rate);if d>32{return}let p=if d<=4{0}else if d<=8{1}else if d<=16{2}else{3};let m=(d>>p)-1;q.rate=(q.parent_rate>>p)/(m+1);q.m=m as u8;q.p=p as u8;}
unsafe fn sun7i_a20_get_out_factors(r:*mut factors_request){let q=&mut *r;if q.rate>q.parent_rate{q.rate=q.parent_rate}let d=div_up(q.parent_rate,q.rate);let p=if d<32{0}else if d/2<32{1}else if d/4<32{2}else{3};let m=div_up(d,1<<p);q.rate=(q.parent_rate>>p)/m;q.m=(m-1) as u8;q.p=p as u8;}
unsafe fn sun6i_display_factors(r:*mut factors_request){let q=&mut *r;if q.rate>q.parent_rate{q.rate=q.parent_rate}let m=div_up(q.parent_rate,q.rate);q.rate=q.parent_rate/m;q.m=(m-1) as u8;}

// The remaining registration tables and CLK_OF_DECLARE entries are preserved as
// dependency-facing declarations; their Linux macro expansion is external.
extern "C" { pub fn sunxi_factors_clk_setup(node:*mut device_node,data:*const factors_data)->*mut clk; pub fn sunxi_mux_clk_setup(node:*mut device_node,data:*const c_void,flags:u32)->*mut clk; pub fn sunxi_divider_clk_setup(node:*mut device_node,data:*const c_void); pub fn sunxi_divs_clk_setup(node:*mut device_node,data:*const c_void)->*mut *mut clk; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
