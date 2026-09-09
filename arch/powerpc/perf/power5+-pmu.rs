// SPDX-License-Identifier: GPL-2.0-or-later
/* Performance counter support for POWER5+/++ (not POWER5) processors. */

// Kernel dependencies supplied externally.

const PM_PMC_SH: u32 = 20; const PM_PMC_MSK: u64 = 0xf;
const PM_PMC_MSKS: u64 = PM_PMC_MSK << PM_PMC_SH; const PM_UNIT_SH: u32 = 16;
const PM_UNIT_MSK: u64 = 0xf; const PM_BYTE_SH: u32 = 12; const PM_BYTE_MSK: u64 = 7;
const PM_GRS_SH: u32 = 8; const PM_GRS_MSK: u64 = 7; const PM_BUSEVENT_MSK: u64 = 0x80;
const PM_PMCSEL_MSK: u64 = 0x7f;
const PM_FPU: usize=0; const PM_ISU0: usize=1; const PM_IFU: usize=2; const PM_ISU1: usize=3;
const PM_IDU: usize=4; const PM_ISU0_ALT: usize=6; const PM_GRS: usize=7; const PM_LSU0: usize=8;
const PM_LSU1: usize=12; const PM_LASTUNIT: usize=12;
const MMCR1_TTM0SEL_SH:u32=62; const MMCR1_TTM1SEL_SH:u32=60; const MMCR1_TTMSEL_MSK:u64=3;
const MMCR1_TTM2SEL_SH:u32=58; const MMCR1_TTM3SEL_SH:u32=56; const MMCR1_TD_CP_DBG0SEL_SH:u32=54;
const MMCR1_GRS_L2SEL_SH:u32=46; const MMCR1_GRS_L3SEL_SH:u32=44; const MMCR1_GRS_MCSEL_SH:u32=41;
const MMCR1_GRS_FABSEL_SH:u32=39; const MMCR1_PMC1_ADDER_SEL_SH:u32=35;
const MMCR1_PMCSEL_SH: fn(usize)->u32 = |n| 25 - (n as u32)*8;

static GRSEL_SHIFT:[u32;8]=[46,46,46,44,44,44,41,39];
static UNIT_CONS:[[u64;2];13]=[[0;2],[0x0200000000,0x0080000000],[0x3200000000,0x2100000000],[0x3200000000,0x3100000000],[0x0e00000000,0x0040000000],[0;2],[0;2],[0x0e00000000,0x0c40000000],[0;2],[0;2],[0;2],[0;2],[0;2]];

unsafe fn power5p_get_constraint(event:u64, maskp:*mut u64, valp:*mut u64, _event_config1:u64)->i32 {
 let mut pmc=((event>>PM_PMC_SH)&PM_PMC_MSK) as usize; let mut mask=0u64; let mut value=0u64;
 if pmc!=0 { if pmc>6{return -1}; let sh=(pmc-1)*2; mask|=2u64<<sh; value|=1u64<<sh; if pmc>=5 && event!=0x500009 && event!=0x600005{return -1}; }
 if event&PM_BUSEVENT_MSK!=0 { let mut unit=((event>>PM_UNIT_SH)&PM_UNIT_MSK) as usize; if unit>PM_LASTUNIT{return -1}; if unit==PM_ISU0_ALT{unit=PM_ISU0}; mask|=UNIT_CONS[unit][0]; value|=UNIT_CONS[unit][1]; let mut byte=((event>>PM_BYTE_SH)&PM_BYTE_MSK) as usize; if byte>=4 {if unit!=PM_LSU1{return -1}; unit+=1; byte&=3;} if unit==PM_GRS {let bit=(event&7) as usize; let fmask=if bit==6{7}else{3}; mask|=(fmask as u64)<<GRSEL_SHIFT[bit]; value|=((event>>PM_GRS_SH)&fmask)<<GRSEL_SHIFT[bit];} mask|=0xfu64<<(24-4*byte); value|=(unit as u64)<<(24-4*byte); }
 if pmc<5 {mask|=0x8000000000000; value|=0x1000000000000;} *maskp=mask; *valp=value; 0
}
unsafe fn power5p_limited_pmc_event(event:u64)->bool {let pmc=(event>>PM_PMC_SH)&PM_PMC_MSK; pmc==5||pmc==6}
const MAX_ALT:usize=3;
static EVENT_ALTERNATIVES:[[u32;3];11]=[[0x100c0,0x40001f,0],[0x120e4,0x400002,0],[0x230e2,0x323087,0],[0x230e3,0x223087,0x3230a0],[0x410c7,0x441084,0],[0x800c4,0xc20e0,0],[0xc50c6,0xc60e0,0],[0x100005,0x600005,0],[0x100009,0x200009,0],[0x200015,0x300015,0],[0x300009,0x400009,0]];
fn find_alternative(event:u32)->i32 {for (i,a) in EVENT_ALTERNATIVES.iter().enumerate(){if event<a[0]{break} for &x in a.iter(){if x!=0&&event==x{return i as i32}}} -1}
static BYTEDECODE:[[u8;4];4]=[[0x21,0x23,0x25,0x27],[7,0x17,0xe,0x1e],[0x20,0x22,0x24,0x26],[7,0x17,0xe,0x1e]];
unsafe fn find_alternative_bdecode(event:u64)->i64 {let pmc=((event>>PM_PMC_SH)&PM_PMC_MSK) as usize; if pmc==0||pmc>4{return -1}; let altpmc=5-pmc; let pp=event&PM_PMCSEL_MSK; for j in 0..4 {if BYTEDECODE[pmc-1][j] as u64==pp{return ((event&!(PM_PMC_MSKS|PM_PMCSEL_MSK))|((altpmc as u64)<<PM_PMC_SH)|(BYTEDECODE[altpmc-1][j] as u64)) as i64;}} if pmc==1&&(pp==0xd||pp==0xe){return (event+(2<<PM_PMC_SH)+(0x2e-0xd)) as i64} if pmc==3&&(pp==0x2e||pp==0x2f){return (event-(2<<PM_PMC_SH)-(0x2e-0xd)) as i64} if pp==0x10||pp==0x28{return (((event^(0x10^0x28))&!PM_PMC_MSKS)|((altpmc as u64)<<PM_PMC_SH)) as i64} -1}
unsafe fn power5p_get_alternatives(event:u64, flags:u32, alt:*mut u64)->i32 { *alt=event; let mut n=1; let mut nlim=power5p_limited_pmc_event(event) as i32; let i=find_alternative(event as u32); if i>=0 {for &ae in EVENT_ALTERNATIVES[i as usize].iter(){if ae!=0&&ae as u64!=event{*alt.add(n as usize)=ae as u64;n+=1;} nlim+=power5p_limited_pmc_event(ae as u64) as i32;}} else {let ae=find_alternative_bdecode(event);if ae>0{*alt.add(n as usize)=ae as u64;n+=1;}} if flags&PPMU_ONLY_COUNT_RUN!=0 {let mut j=n;for k in 0..n {match *alt.add(k as usize){0xf=>{*alt.add(j as usize)=0x600005;j+=1;nlim+=1},0x600005=>{*alt.add(j as usize)=0xf;j+=1},0x100009=>{*alt.add(j as usize)=0x500009;j+=1;nlim+=1},0x500009=>{*alt.add(j as usize)=0x100009;j+=1;*alt.add(j as usize)=0x200009;j+=1},_=>{}}}n=j;} if flags&PPMU_LIMITED_PMC_OK==0&&nlim!=0 {let mut j=0;for k in 0..n{if !power5p_limited_pmc_event(*alt.add(k as usize)){*alt.add(j as usize)=*alt.add(k as usize);j+=1}}n=j;} else if flags&PPMU_LIMITED_PMC_REQD!=0&&nlim<n {let mut j=0;for k in 0..n{if power5p_limited_pmc_event(*alt.add(k as usize)){*alt.add(j as usize)=*alt.add(k as usize);j+=1}}n=j;} n}

// The remaining PMU integration items depend on kernel types/constants declared in external headers.
// Their C declarations are preserved here as external interfaces.
extern "C" { fn register_power_pmu(pmu:*mut power_pmu)->i32; }
#[repr(C)] pub struct power_pmu { pub name:*const u8, pub n_counter:u32, pub max_alternatives:u32, pub add_fields:u64, pub test_adder:u64, pub compute_mmcr:Option<unsafe extern "C" fn()>, pub get_constraint:Option<unsafe extern "C" fn()>, pub get_alternatives:Option<unsafe extern "C" fn()>, pub disable_pmc:Option<unsafe extern "C" fn()>, pub limited_pmc_event:Option<unsafe extern "C" fn()>, pub flags:u32, pub n_generic:usize, pub generic_events:*mut i32, pub cache_events:*mut u64 }
static mut POWER5P_PMU:power_pmu=power_pmu{name:b"POWER5+/++\0".as_ptr(),n_counter:6,max_alternatives:MAX_ALT as u32,add_fields:0x7000000000055,test_adder:0x3000040000000,compute_mmcr:None,get_constraint:None,get_alternatives:None,disable_pmc:None,limited_pmc_event:None,flags:0,n_generic:0,generic_events:std::ptr::null_mut(),cache_events:std::ptr::null_mut()};
pub unsafe fn init_power5p_pmu()->i32 { register_power_pmu(&mut POWER5P_PMU) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
