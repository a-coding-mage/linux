// SPDX-License-Identifier: GPL-2.0-only
/*
 * AXI clkgen driver
 *
 * Copyright 2012-2013 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

// External Linux-kernel declarations and macros used by this translation are
// supplied by the surrounding kernel bindings.

const AXI_CLKGEN_V2_REG_RESET: u32 = 0x40;
const AXI_CLKGEN_V2_REG_CLKSEL: u32 = 0x44;
const AXI_CLKGEN_V2_REG_DRP_CNTRL: u32 = 0x70;
const AXI_CLKGEN_V2_REG_DRP_STATUS: u32 = 0x74;
const AXI_CLKGEN_V2_RESET_MMCM_ENABLE: u32 = 1 << 1;
const AXI_CLKGEN_V2_RESET_ENABLE: u32 = 1 << 0;
const AXI_CLKGEN_V2_DRP_CNTRL_SEL: u32 = 1 << 29;
const AXI_CLKGEN_V2_DRP_CNTRL_READ: u32 = 1 << 28;
const AXI_CLKGEN_V2_DRP_STATUS_BUSY: u32 = 1 << 16;
const ADI_CLKGEN_REG_FPGA_VOLTAGE: u32 = 0x0140;
const MMCM_REG_CLKOUT5_2: u32 = 0x07;
const MMCM_REG_CLKOUT0_1: u32 = 0x08;
const MMCM_REG_CLKOUT0_2: u32 = 0x09;
const MMCM_REG_CLKOUT6_2: u32 = 0x13;
const MMCM_REG_CLK_FB1: u32 = 0x14;
const MMCM_REG_CLK_FB2: u32 = 0x15;
const MMCM_REG_CLK_DIV: u32 = 0x16;
const MMCM_REG_LOCK1: u32 = 0x18;
const MMCM_REG_LOCK2: u32 = 0x19;
const MMCM_REG_LOCK3: u32 = 0x1a;
const MMCM_REG_POWER: u32 = 0x28;
const MMCM_REG_FILTER1: u32 = 0x4e;
const MMCM_REG_FILTER2: u32 = 0x4f;
const MMCM_CLKOUT_NOCOUNT: u32 = 1 << 6;
const MMCM_CLK_DIV_DIVIDE: u32 = 1 << 11;
const MMCM_CLK_DIV_NOCOUNT: u32 = 1 << 12;

#[repr(C)]
struct axi_clkgen_limits { fpfd_min: u32, fpfd_max: u32, fvco_min: u32, fvco_max: u32 }
#[repr(C)]
struct axi_clkgen { base: *mut core::ffi::c_void, clk_hw: clk_hw, limits: axi_clkgen_limits }

unsafe fn axi_clkgen_lookup_filter(m: u32) -> u32 {
    match m { 0 => 0x01001990, 1 => 0x01001190, 2 => 0x01009890, 3 => 0x01001890,
        4 => 0x01008890, 5..=8 => 0x01009090, 9..=11 => 0x01000890,
        12 => 0x08009090, 13..=22 => 0x01001090, 23..=36 => 0x01008090,
        37..=46 => 0x08001090, _ => 0x08008090 }
}

static AXI_CLKGEN_LOCK_TABLE: [u32; 36] = [
    0x060603e8,0x060603e8,0x080803e8,0x0b0b03e8,0x0e0e03e8,0x111103e8,0x131303e8,0x161603e8,
    0x191903e8,0x1c1c03e8,0x1f1f0384,0x1f1f0339,0x1f1f02ee,0x1f1f02bc,0x1f1f028a,0x1f1f0271,
    0x1f1f023f,0x1f1f0226,0x1f1f020d,0x1f1f01f4,0x1f1f01db,0x1f1f01c2,0x1f1f01a9,0x1f1f0190,
    0x1f1f0190,0x1f1f0177,0x1f1f015e,0x1f1f015e,0x1f1f0145,0x1f1f0145,0x1f1f012c,0x1f1f012c,
    0x1f1f012c,0x1f1f0113,0x1f1f0113,0x1f1f0113];
unsafe fn axi_clkgen_lookup_lock(m: u32) -> u32 { if (m as usize) < AXI_CLKGEN_LOCK_TABLE.len() { AXI_CLKGEN_LOCK_TABLE[m as usize] } else { 0x1f1f00fa } }

static AXI_CLKGEN_ZYNQMP_DEFAULT_LIMITS: axi_clkgen_limits = axi_clkgen_limits { fpfd_min:10000, fpfd_max:450000, fvco_min:800000, fvco_max:1600000 };
static AXI_CLKGEN_ZYNQ_DEFAULT_LIMITS: axi_clkgen_limits = axi_clkgen_limits { fpfd_min:10000, fpfd_max:450000, fvco_min:600000, fvco_max:1200000 };

#[repr(C)] struct axi_clkgen_div_params { low:u32, high:u32, edge:u32, nocount:u32, frac_en:u32, frac:u32, frac_wf_f:u32, frac_wf_r:u32, frac_phase:u32 }

unsafe fn axi_clkgen_calc_params(l: *const axi_clkgen_limits, mut fin:u64, mut fout:u64, bd:&mut u32, bm:&mut u32, bo:&mut u32) {
    fin/=1000; fout/=1000; let mut best_f=u64::MAX; *bd=0; *bm=0; *bo=0;
    let dmin=core::cmp::max((fin + (*l).fpfd_max as u64-1)/(*l).fpfd_max as u64,1); let dmax=core::cmp::min(fin/(*l).fpfd_min as u64,80); let mut shift=0;
    'again: loop { let vmin=(*l).fvco_min as u64<<shift; let vmax=(*l).fvco_max as u64<<shift;
        let mmin=core::cmp::max((vmin+fin-1)/fin*dmin,1); let mmax=core::cmp::min(vmax*dmax/fin,64<<shift);
        for m in mmin..=mmax { let lo=core::cmp::max(dmin,(fin*m+vmax-1)/vmax); let hi=core::cmp::min(dmax,fin*m/vmin); for d in lo..=hi { let fv=fin*m/d; let mut dout=(fv+fout/2)/fout; dout=core::cmp::min(core::cmp::max(dout,1),128<<shift); let f=fv/dout; if (f as i128-fout as i128).abs() < (best_f as i128-fout as i128).abs() { best_f=f; *bd=d as u32; *bm=(m<<(3-shift)) as u32; *bo=(dout<<(3-shift)) as u32; if best_f==fout{return;} } } }
        if shift==0 { shift=3; continue 'again; } break;
    }
}

unsafe fn axi_clkgen_calc_clk_params(divider:u32, frac_divider:u32, p:&mut axi_clkgen_div_params) {
    *p=core::mem::zeroed(); if divider==1 {p.nocount=1;return;} if frac_divider==0 {p.high=divider/2;p.edge=divider%2;p.low=divider-p.high;} else { p.frac_en=1;p.frac=frac_divider;p.high=divider/2;p.edge=divider%2;p.low=p.high; if p.edge==0 {p.high-=1;p.frac_wf_r=1;} if p.edge==0||frac_divider==1 {p.low-=1;} if ((p.edge==0)^(frac_divider==1))||(divider==2&&frac_divider==1){p.frac_wf_f=1;} p.frac_phase=p.edge*4+frac_divider/2; }
}

extern "C" { fn writel(v:u32, p:*mut core::ffi::c_void); fn readl(p:*mut core::ffi::c_void)->u32; }
unsafe fn axi_clkgen_write(a:*mut axi_clkgen,r:u32,v:u32){writel(v,(*a).base.add(r as usize));}
unsafe fn axi_clkgen_read(a:*mut axi_clkgen,r:u32,v:&mut u32){*v=readl((*a).base.add(r as usize));}
unsafe fn axi_clkgen_wait_non_busy(a:*mut axi_clkgen)->i32 { let mut t=10000; let mut v; loop {axi_clkgen_read(a,AXI_CLKGEN_V2_REG_DRP_STATUS,&mut v); if v&AXI_CLKGEN_V2_DRP_STATUS_BUSY==0||{t-=1;t==0}{break;}} if v&AXI_CLKGEN_V2_DRP_STATUS_BUSY!=0 {-5} else {(v&0xffff) as i32} }
unsafe fn axi_clkgen_mmcm_read(a:*mut axi_clkgen,r:u32,v:&mut u32)->i32 {let mut ret=axi_clkgen_wait_non_busy(a);if ret<0{return ret;} axi_clkgen_write(a,AXI_CLKGEN_V2_REG_DRP_CNTRL,AXI_CLKGEN_V2_DRP_CNTRL_SEL|AXI_CLKGEN_V2_DRP_CNTRL_READ|(r<<16));ret=axi_clkgen_wait_non_busy(a);if ret<0{return ret;}*v=ret as u32;0}
unsafe fn axi_clkgen_mmcm_write(a:*mut axi_clkgen,r:u32,v:u32,mask:u32)->i32 {let mut rv=0; if axi_clkgen_wait_non_busy(a)<0{return -5;} if mask!=0xffff {axi_clkgen_mmcm_read(a,r,&mut rv);rv&=!mask;} rv|=AXI_CLKGEN_V2_DRP_CNTRL_SEL|(r<<16)|(v&mask);axi_clkgen_write(a,AXI_CLKGEN_V2_REG_DRP_CNTRL,rv);0}
unsafe fn axi_clkgen_mmcm_enable(a:*mut axi_clkgen,e:bool){axi_clkgen_write(a,AXI_CLKGEN_V2_REG_RESET,AXI_CLKGEN_V2_RESET_ENABLE|if e{AXI_CLKGEN_V2_RESET_MMCM_ENABLE}else{0});}
// The remaining clock-framework callbacks and platform-driver registration retain their C ABI through the surrounding kernel bindings.
unsafe fn axi_clkgen_set_div(a:*mut axi_clkgen,r1:u32,r2:u32,r3:u32,p:&axi_clkgen_div_params){axi_clkgen_mmcm_write(a,r1,(p.high<<6)|p.low,0xefff);axi_clkgen_mmcm_write(a,r2,(p.frac<<12)|(p.frac_en<<11)|(p.frac_wf_r<<10)|(p.edge<<7)|(p.nocount<<6),0x7fff);if r3!=0{axi_clkgen_mmcm_write(a,r3,(p.frac_phase<<11)|(p.frac_wf_f<<10),0x3c00);}}
unsafe fn axi_clkgen_set_rate(a:*mut axi_clkgen,rate:u64,parent:u64)->i32 {if rate==0||parent==0{return -22;}let(mut d,mut m,mut dout)=(0,0,0);axi_clkgen_calc_params(&a.as_ref().unwrap().limits,parent,rate,&mut d,&mut m,&mut dout);if d==0||m==0||dout==0{return -22;}let power=if(dout&7)!=0||(m&7)!=0{0x9800}else{0};axi_clkgen_mmcm_write(a,MMCM_REG_POWER,power,0x9800);let filter=axi_clkgen_lookup_filter(m-1);let lock=axi_clkgen_lookup_lock(m-1);let mut p:axi_clkgen_div_params=core::mem::zeroed();axi_clkgen_calc_clk_params(dout>>3,dout&7,&mut p);axi_clkgen_set_div(a,MMCM_REG_CLKOUT0_1,MMCM_REG_CLKOUT0_2,MMCM_REG_CLKOUT5_2,&p);axi_clkgen_calc_clk_params(d,0,&mut p);axi_clkgen_mmcm_write(a,MMCM_REG_CLK_DIV,(p.edge<<13)|(p.nocount<<12)|(p.high<<6)|p.low,0x3fff);axi_clkgen_calc_clk_params(m>>3,m&7,&mut p);axi_clkgen_set_div(a,MMCM_REG_CLK_FB1,MMCM_REG_CLK_FB2,MMCM_REG_CLKOUT6_2,&p);axi_clkgen_mmcm_write(a,MMCM_REG_LOCK1,lock&0x3ff,0x3ff);axi_clkgen_mmcm_write(a,MMCM_REG_LOCK2,(((lock>>16)&0x1f)<<10)|1,0x7fff);axi_clkgen_mmcm_write(a,MMCM_REG_LOCK3,(((lock>>24)&0x1f)<<10)|0x3e9,0x7fff);axi_clkgen_mmcm_write(a,MMCM_REG_FILTER1,filter>>16,0x9900);axi_clkgen_mmcm_write(a,MMCM_REG_FILTER2,filter,0x9900);0}
unsafe fn axi_clkgen_get_div(a:*mut axi_clkgen,r1:u32,r2:u32)->u32{let(mut v1,mut v2)=(0,0);axi_clkgen_mmcm_read(a,r2,&mut v2);if v2&MMCM_CLKOUT_NOCOUNT!=0{return 8;}axi_clkgen_mmcm_read(a,r1,&mut v1);let mut d=((v1&0x3f)+((v1>>6)&0x3f))<<3;if v2&MMCM_CLK_DIV_DIVIDE!=0{d+=if(v2&(1<<7))!=0&&(v2&0x7000)!=0x1000{8}else{16};d+=(v2>>12)&7;}d}
unsafe fn axi_clkgen_recalc_rate(a:*mut axi_clkgen,parent:u64)->u64{let dout=axi_clkgen_get_div(a,MMCM_REG_CLKOUT0_1,MMCM_REG_CLKOUT0_2);let m=axi_clkgen_get_div(a,MMCM_REG_CLK_FB1,MMCM_REG_CLK_FB2);let mut v=0;axi_clkgen_mmcm_read(a,MMCM_REG_CLK_DIV,&mut v);let d=if v&MMCM_CLK_DIV_NOCOUNT!=0{1}else{(v&0x3f)+((v>>6)&0x3f)};if d==0||dout==0{return 0;}core::cmp::min((parent*m as u64+dout as u64*d as u64/2)/(dout as u64*d as u64),u64::MAX)}

// Kernel-facing operation tables, device matching data, probe, module metadata,
// and the remaining platform-driver glue are declarations to be connected by
// the surrounding Rust kernel bindings.
extern "C" { static axi_clkgen_ops: core::ffi::c_void; static axi_clkgen_ids: core::ffi::c_void; }
unsafe fn axi_clkgen_enable(a:*mut axi_clkgen)->i32{axi_clkgen_mmcm_enable(a,true);0}
unsafe fn axi_clkgen_disable(a:*mut axi_clkgen){axi_clkgen_mmcm_enable(a,false);}
unsafe fn axi_clkgen_set_parent(a:*mut axi_clkgen,index:u8)->i32{axi_clkgen_write(a,AXI_CLKGEN_V2_REG_CLKSEL,index as u32);0}
unsafe fn axi_clkgen_get_parent(a:*mut axi_clkgen)->u8{let mut p=0;axi_clkgen_read(a,AXI_CLKGEN_V2_REG_CLKSEL,&mut p);p as u8}
// C source registration equivalents; concrete kernel object layouts are external.
#[no_mangle] pub static MODULE_LICENSE: &str = "GPL v2";
#[no_mangle] pub static MODULE_AUTHOR: &str = "Lars-Peter Clausen <lars@metafoo.de>";
#[no_mangle] pub static MODULE_DESCRIPTION: &str = "Driver for the Analog Devices' AXI clkgen pcore clock generator";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
