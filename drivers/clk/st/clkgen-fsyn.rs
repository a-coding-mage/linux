// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014 STMicroelectronics R&D Ltd
 *
 * Authors:
 * Stephen Gallimore <stephen.gallimore@st.com>,
 * Pankaj Dev <pankaj.dev@st.com>.
 */

const QUADFS_NDIV_THRESHOLD: u64 = 30000000;
const PLL_BW_GOODREF: i64 = 0;
const PLL_BW_VBADREF: i64 = 1;
const PLL_BW_BADREF: i64 = 2;
const PLL_BW_VGOODREF: i64 = 3;
const QUADFS_MAX_CHAN: usize = 4;

#[repr(C)]
pub struct stm_fs { pub ndiv: usize, pub mdiv: usize, pub pe: usize, pub sdiv: usize, pub nsdiv: usize }

#[repr(C)]
pub struct clkgen_quadfs_data {
    pub reset_present: bool, pub bwfilter_present: bool, pub lockstatus_present: bool,
    pub powerup_polarity: bool, pub standby_polarity: bool, pub nsdiv_present: bool, pub nrst_present: bool,
    pub ndiv: clkgen_field, pub ref_bw: clkgen_field, pub nreset: clkgen_field, pub npda: clkgen_field,
    pub lock_status: clkgen_field, pub nrst: [clkgen_field; QUADFS_MAX_CHAN], pub nsb: [clkgen_field; QUADFS_MAX_CHAN],
    pub en: [clkgen_field; QUADFS_MAX_CHAN], pub mdiv: [clkgen_field; QUADFS_MAX_CHAN],
    pub pe: [clkgen_field; QUADFS_MAX_CHAN], pub sdiv: [clkgen_field; QUADFS_MAX_CHAN],
    pub nsdiv: [clkgen_field; QUADFS_MAX_CHAN], pub pll_ops: *const clk_ops,
    pub get_params: Option<unsafe extern "C" fn(usize, usize, *mut stm_fs) -> i32>,
    pub get_rate: Option<unsafe extern "C" fn(usize, *const stm_fs, *mut usize) -> i32>,
}
#[repr(C)] pub struct clkgen_clk_out { pub name: *const i8, pub flags: usize }
#[repr(C)] pub struct clkgen_quadfs_data_clks { pub data: *mut clkgen_quadfs_data, pub outputs: *const clkgen_clk_out }

extern "C" {
    static st_quadfs_pll_c32_ops: clk_ops;
    fn clk_fs660c32_dig_get_params(input: usize, output: usize, fs: *mut stm_fs) -> i32;
    fn clk_fs660c32_dig_get_rate(input: usize, fs: *const stm_fs, rate: *mut usize) -> i32;
}

// The following hardware descriptions directly mirror the C designated initializers.
static mut st_fs660c32_C: clkgen_quadfs_data = clkgen_quadfs_data {
    reset_present:true,bwfilter_present:false,lockstatus_present:true,powerup_polarity:true,standby_polarity:true,nsdiv_present:true,nrst_present:true,
    ndiv: CLKGEN_FIELD!(0x2f4,0x7,16), ref_bw: CLKGEN_FIELD!(0,0,0), nreset: CLKGEN_FIELD!(0,0,0), npda: CLKGEN_FIELD!(0x2f0,1,12), lock_status: CLKGEN_FIELD!(0x2f0,1,24),
    nrst:[CLKGEN_FIELD!(0x2f0,1,0),CLKGEN_FIELD!(0x2f0,1,1),CLKGEN_FIELD!(0x2f0,1,2),CLKGEN_FIELD!(0x2f0,1,3)],
    nsb:[CLKGEN_FIELD!(0x2f0,1,8),CLKGEN_FIELD!(0x2f0,1,9),CLKGEN_FIELD!(0x2f0,1,10),CLKGEN_FIELD!(0x2f0,1,11)],
    en:[CLKGEN_FIELD!(0x2fc,1,0),CLKGEN_FIELD!(0x2fc,1,1),CLKGEN_FIELD!(0x2fc,1,2),CLKGEN_FIELD!(0x2fc,1,3)],
    mdiv:[CLKGEN_FIELD!(0x304,0x1f,15),CLKGEN_FIELD!(0x308,0x1f,15),CLKGEN_FIELD!(0x30c,0x1f,15),CLKGEN_FIELD!(0x310,0x1f,15)],
    pe:[CLKGEN_FIELD!(0x304,0x7fff,0),CLKGEN_FIELD!(0x308,0x7fff,0),CLKGEN_FIELD!(0x30c,0x7fff,0),CLKGEN_FIELD!(0x310,0x7fff,0)],
    sdiv:[CLKGEN_FIELD!(0x304,0xf,20),CLKGEN_FIELD!(0x308,0xf,20),CLKGEN_FIELD!(0x30c,0xf,20),CLKGEN_FIELD!(0x310,0xf,20)],
    nsdiv:[CLKGEN_FIELD!(0x304,1,24),CLKGEN_FIELD!(0x308,1,24),CLKGEN_FIELD!(0x30c,1,24),CLKGEN_FIELD!(0x310,1,24)],
    pll_ops:unsafe { &st_quadfs_pll_c32_ops },get_params:Some(clk_fs660c32_dig_get_params),get_rate:Some(clk_fs660c32_dig_get_rate),
};

#[repr(C)] pub struct st_clk_quadfs_pll { pub hw: clk_hw, pub regs_base:*mut core::ffi::c_void, pub lock:*mut spinlock_t, pub data:*mut clkgen_quadfs_data, pub ndiv:u32 }
#[repr(C)] pub struct st_clk_quadfs_fsynth { pub hw:clk_hw,pub regs_base:*mut core::ffi::c_void,pub lock:*mut spinlock_t,pub data:*mut clkgen_quadfs_data,pub chan:u32,pub md:u32,pub pe:u32,pub sdiv:u32,pub nsdiv:u32 }

const P20: u64 = 1u64 << 20;

unsafe fn clk_fs660c32_vco_get_rate(input: usize, fs:*const stm_fs, rate:*mut usize)->i32 { *rate=input*((*fs).ndiv+16); 0 }
unsafe fn clk_fs660c32_vco_get_params(mut input:usize, mut output:usize, fs:*mut stm_fs)->i32 {
    if output<384000000 || output>660000000 || input>40000000 { return -22; }
    let pdiv=1usize; input/=1000; output/=1000; let mut n=output*pdiv/input; if n<16 {n=16;} (*fs).ndiv=n-16; 0
}
unsafe fn clk_fs660c32_dig_get_rate(input:usize,fs:*const stm_fs,rate:*mut usize)->i32 {
    let s=1usize<<(*fs).sdiv; let ns=if (*fs).nsdiv==1 {1} else {3}; let res=(P20*(32+(*fs).mdiv as u64)+32*(*fs).pe as u64)*s as u64*ns; *rate=((input as u64*P20*32)/res) as usize; 0
}
unsafe fn clk_fs660c32_dig_get_params(input:usize,output:usize,fs:*mut stm_fs)->i32 {
    let mut best=usize::MAX; let mut found=false;
    for si in 0..=8 { for m in 0..=31 { let val=(output as u64)<<si; let mut p=((input as u64*P20-(32+m as u64)*val*(P20/32))/val) as u64; if p>32767 {continue;} let t=stm_fs{ndiv:0,mdiv:m,pe:p as usize,sdiv:si,nsdiv:1}; let mut f=0; clk_fs660c32_dig_get_rate(input,&t,&mut f); let d=output.abs_diff(f); if d<best {best=d;found=true;(*fs)=t;} } }
    if !found {-1} else {0}
}

// Kernel-facing operations and registration entry points retain their original external interfaces.
extern "C" {
    fn quadfs_pll_enable(hw:*mut clk_hw)->i32; fn quadfs_pll_disable(hw:*mut clk_hw);
    fn quadfs_pll_is_enabled(hw:*mut clk_hw)->i32; fn quadfs_pll_fs660c32_recalc_rate(hw:*mut clk_hw,parent_rate:usize)->usize;
    fn quadfs_pll_fs660c32_determine_rate(hw:*mut clk_hw,req:*mut clk_rate_request)->i32;
    fn quadfs_pll_fs660c32_set_rate(hw:*mut clk_hw,rate:usize,parent_rate:usize)->i32;
    fn quadfs_fsynth_enable(hw:*mut clk_hw)->i32; fn quadfs_fsynth_disable(hw:*mut clk_hw);
    fn quadfs_fsynth_is_enabled(hw:*mut clk_hw)->i32; fn quadfs_determine_rate(hw:*mut clk_hw,req:*mut clk_rate_request)->i32;
    fn quadfs_set_rate(hw:*mut clk_hw,rate:usize,parent_rate:usize)->i32; fn quadfs_recalc_rate(hw:*mut clk_hw,parent_rate:usize)->usize;
    fn st_of_quadfs_setup(np:*mut device_node,datac:*mut clkgen_quadfs_data_clks);
}

// Device-tree declarations from the C source.
unsafe fn st_of_quadfs660C_setup(np:*mut device_node){st_of_quadfs_setup(np, &mut st_fs660c32_C_data as *mut _);}
static mut st_fs660c32_C_data:clkgen_quadfs_data_clks=clkgen_quadfs_data_clks{data:unsafe{&mut st_fs660c32_C},outputs:core::ptr::null()};
unsafe fn st_of_quadfs660D_setup(np:*mut device_node){st_of_quadfs_setup(np, core::ptr::null_mut());}
unsafe fn st_of_quadfs660D0_setup(np:*mut device_node){st_of_quadfs_setup(np, core::ptr::null_mut());}
unsafe fn st_of_quadfs660D2_setup(np:*mut device_node){st_of_quadfs_setup(np, core::ptr::null_mut());}
unsafe fn st_of_quadfs660D3_setup(np:*mut device_node){st_of_quadfs_setup(np, core::ptr::null_mut());}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
