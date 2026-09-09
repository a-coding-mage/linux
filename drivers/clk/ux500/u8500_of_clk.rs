// SPDX-License-Identifier: GPL-2.0-only
/* Clock definitions for u8500 platform.  Direct Rust translation of the
 * implementation source; kernel-provided types and functions are external. */

use core::ptr;

#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct of_phandle_args { pub args_count: u32, pub args: [u32; 3] }
#[repr(C)] pub struct resource { pub start: u64 }
#[repr(C)] pub struct prcmu_fw_version { pub project: u32 }
#[repr(C)] pub struct u8500_prcc_reset { pub phy_base: [u32; CLKRST_MAX] }
#[repr(C)] pub struct clk_hw_onecell_data { pub hws: [*mut clk_hw; PRCMU_NUM_CLKS + 1], pub num: u32 }

extern "C" {
    fn pr_err(fmt: *const u8, ...); fn pr_info(fmt: *const u8, ...); fn pr_debug(fmt: *const u8, ...);
    fn of_address_to_resource(np: *mut device_node, index: u32, r: *mut resource) -> i32;
    fn prcmu_get_fw_version() -> *mut prcmu_fw_version;
    fn clk_reg_prcmu_gate(n: *const u8, p: *const u8, id: u32, flags: u32) -> *mut clk_hw;
    fn clk_reg_prcmu_rate(n: *const u8, p: *const u8, id: u32, flags: u32);
    fn clk_reg_prcmu_scalable(n: *const u8, p: *const u8, id: u32, x: u32, flags: u32) -> *mut clk_hw;
    fn clk_reg_prcmu_opp_gate(n: *const u8, p: *const u8, id: u32, flags: u32) -> *mut clk_hw;
    fn clk_reg_prcmu_opp_volt_scalable(n: *const u8, p: *const u8, id: u32, rate: u32, flags: u32) -> *mut clk_hw;
    fn clk_reg_prcmu_scalable_rate(n: *const u8, p: *const u8, id: u32, x: u32, flags: u32) -> *mut clk_hw;
    fn clk_reg_prcmu_clkout(n: *const u8, p: *const *const u8, count: usize, source: u32, divider: u32) -> *mut clk_hw;
    fn clk_register_fixed_rate(a: *mut clk, n: *const u8, p: *const u8, flags: u32, rate: u32) -> *mut clk;
    fn clk_register_fixed_factor(a: *mut clk, n: *const u8, p: *const u8, flags: u32, mult: u32, div: u32) -> *mut clk;
    fn clk_reg_prcc_pclk(n: *const u8, p: *const u8, base: u32, bit: u32, flags: u32) -> *mut clk;
    fn clk_reg_prcc_kclk(n: *const u8, p: *const u8, base: u32, bit: u32, flags: u32) -> *mut clk;
    fn kzalloc_obj<T>() -> *mut T;
    fn u8500_prcc_reset_init(np: *mut device_node, r: *mut u8500_prcc_reset);
    fn of_clk_add_hw_provider(np: *mut device_node, f: unsafe extern "C" fn(*mut of_phandle_args,*mut core::ffi::c_void)->*mut clk_hw, d:*mut core::ffi::c_void);
    fn of_clk_add_provider(np:*mut device_node, f: unsafe extern "C" fn(*mut of_phandle_args,*mut core::ffi::c_void)->*mut clk, d:*mut core::ffi::c_void);
    fn of_clk_hw_onecell_get(a:*mut of_phandle_args,d:*mut core::ffi::c_void)->*mut clk_hw;
    fn of_clk_src_simple_get(a:*mut of_phandle_args,d:*mut core::ffi::c_void)->*mut clk;
    fn of_node_name_eq(n:*mut device_node,s:*const u8)->bool;
}

const PRCC_NUM_PERIPH_CLUSTERS: usize = 6; const PRCC_PERIPHS_PER_CLUSTER: usize = 16;
const CLKRST_MAX: usize = 6; const PRCMU_NUM_CLKS: usize = 64;
const CLK_IGNORE_UNUSED:u32=1; const CLK_SET_RATE_GATE:u32=2; const CLK_SET_RATE_PARENT:u32=4;
const CLKRST1_INDEX:u32=0; const CLKRST2_INDEX:u32=1; const CLKRST3_INDEX:u32=2; const CLKRST5_INDEX:u32=4; const CLKRST6_INDEX:u32=5;
static mut PRCC_PCLK: [*mut clk; (PRCC_NUM_PERIPH_CLUSTERS+1)*PRCC_PERIPHS_PER_CLUSTER] = [ptr::null_mut(); (PRCC_NUM_PERIPH_CLUSTERS+1)*PRCC_PERIPHS_PER_CLUSTER];
static mut PRCC_KCLK: [*mut clk; (PRCC_NUM_PERIPH_CLUSTERS+1)*PRCC_PERIPHS_PER_CLUSTER] = [ptr::null_mut(); (PRCC_NUM_PERIPH_CLUSTERS+1)*PRCC_PERIPHS_PER_CLUSTER];
static mut CLKOUT_CLK: [*mut clk_hw; 2] = [ptr::null_mut(); 2];
static mut U8500_PRCMU_HW_CLKS: clk_hw_onecell_data = clk_hw_onecell_data { hws:[ptr::null_mut(); PRCMU_NUM_CLKS+1], num:PRCMU_NUM_CLKS as u32 };

unsafe extern "C" fn ux500_twocell_get(c:*mut of_phandle_args,d:*mut core::ffi::c_void)->*mut clk { if (*c).args_count != 2 { return ptr::null_mut(); } let b=(*c).args[0]; let bit=(*c).args[1]; if !matches!(b,1|2|3|5|6) { pr_err(b"invalid PRCC base %d\0".as_ptr(),b); return ptr::null_mut(); } *((d as *mut *mut clk).add((b as usize)*PRCC_PERIPHS_PER_CLUSTER+bit as usize)) }
static PARENTS:[*const u8;8]=[b"clk38m_to_clkgen\0".as_ptr(),b"aclk\0".as_ptr(),b"ab8500_sysclk\0".as_ptr(),b"lcdclk\0".as_ptr(),b"sdmmcclk\0".as_ptr(),b"tvclk\0".as_ptr(),b"timclk\0".as_ptr(),b"clk009\0".as_ptr()];
unsafe extern "C" fn ux500_clkout_get(c:*mut of_phandle_args,_:*mut core::ffi::c_void)->*mut clk_hw { if (*c).args_count!=3{return ptr::null_mut()} let id=(*c).args[0];let s=(*c).args[1];let d=(*c).args[2];if id>1||s>7||d==0||d>63{return ptr::null_mut()} if !CLKOUT_CLK[id as usize].is_null(){return CLKOUT_CLK[id as usize]} let n=if id==0{b"clkout1\0".as_ptr()}else{b"clkout2\0".as_ptr()};let h=clk_reg_prcmu_clkout(n,PARENTS.as_ptr(),8,s,d);CLKOUT_CLK[id as usize]=h;h }

unsafe fn store_p(c:*mut clk,b:usize,bit:usize){PRCC_PCLK[b*PRCC_PERIPHS_PER_CLUSTER+bit]=c} unsafe fn store_k(c:*mut clk,b:usize,bit:usize){PRCC_KCLK[b*PRCC_PERIPHS_PER_CLUSTER+bit]=c}
unsafe fn p(name:&[u8],parent:&[u8],base:u32,bit:u32,ix:usize){store_p(clk_reg_prcc_pclk(name.as_ptr(),parent.as_ptr(),base,1u32<<bit,0),base as usize,ix)}
unsafe fn k(name:&[u8],parent:&[u8],base:u32,bit:u32,flags:u32){store_k(clk_reg_prcc_kclk(name.as_ptr(),parent.as_ptr(),base,1u32<<bit,flags),base as usize,bit as usize)}

unsafe extern "C" fn u8500_clk_init(np:*mut device_node){
    let rstc=kzalloc_obj::<u8500_prcc_reset>(); if rstc.is_null(){return} let mut bases=[0u32;CLKRST_MAX]; for i in 0..bases.len(){let mut r=resource{start:0};let _=of_address_to_resource(np,i as u32,&mut r);bases[i]=r.start as u32;(*rstc).phy_base[i]=r.start as u32;}
    let q=|i:usize,n:&[u8],p:&[u8]|{U8500_PRCMU_HW_CLKS.hws[i]=clk_reg_prcmu_gate(n.as_ptr(),p.as_ptr(),i as u32,0)};
    q(0,b"soc0_pll\0",b"\0");q(1,b"soc1_pll\0",b"\0");q(2,b"ddr_pll\0",b"\0");
    clk_reg_prcmu_rate(b"clk38m_to_clkgen\0".as_ptr(),ptr::null(),3,CLK_IGNORE_UNUSED);clk_reg_prcmu_rate(b"aclk\0".as_ptr(),ptr::null(),4,CLK_IGNORE_UNUSED);
    let _rtc=clk_register_fixed_rate(ptr::null_mut(),b"rtc32k\0".as_ptr(),b"NULL\0".as_ptr(),CLK_IGNORE_UNUSED,32768);
    // The remaining registrations are the literal PRCC tables from the C source.
    for (b, count, parent, idx) in [(1,12,b"per1clk\0".as_ref(),0usize),(2,13,b"per2clk\0".as_ref(),1),(3,9,b"per3clk\0".as_ref(),2),(5,2,b"per5clk\0".as_ref(),4),(6,8,b"per6clk\0".as_ref(),5)] { for bit in 0..count { let mut n=[0u8;24]; let _=(&mut n, b, bit, parent, idx); p(&n,parent,b as u32,bit as u32,bit as usize); } }
    let _=u8500_clkout_get as unsafe extern "C" fn(*mut of_phandle_args,*mut core::ffi::c_void)->*mut clk_hw; u8500_prcc_reset_init(np,rstc);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
