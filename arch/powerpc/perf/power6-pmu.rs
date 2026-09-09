// SPDX-License-Identifier: GPL-2.0-or-later
/* Performance counter support for POWER6 processors. */

const PM_PMC_SH: u64 = 20;
const PM_PMC_MSK: u64 = 0x7;
const PM_PMC_MSKS: u64 = PM_PMC_MSK << PM_PMC_SH;
const PM_UNIT_SH: u64 = 16;
const PM_UNIT_MSK: u64 = 0xf;
const PM_UNIT_MSKS: u64 = PM_UNIT_MSK << PM_UNIT_SH;
const PM_LLAV: u64 = 0x8000;
const PM_LLA: u64 = 0x4000;
const PM_BYTE_SH: u64 = 12;
const PM_BYTE_MSK: u64 = 3;
const PM_SUBUNIT_SH: u64 = 8;
const PM_SUBUNIT_MSK: u64 = 7;
const PM_SUBUNIT_MSKS: u64 = PM_SUBUNIT_MSK << PM_SUBUNIT_SH;
const PM_PMCSEL_MSK: u64 = 0xff;
const PM_BUSEVENT_MSK: u64 = 0xf3700;
const MMCR1_TTM0SEL_SH: u64 = 60;
const MMCR1_NESTSEL_SH: u64 = 45;
const MMCR1_NESTSEL_MSK: u64 = 0x7;
const MMCR1_PMC1_LLA: u64 = 1u64 << 44;
const MMCR1_PMC1_LLA_VALUE: u64 = 1u64 << 39;
const MMCR1_PMC1_ADDR_SEL: u64 = 1u64 << 35;
const MMCR1_PMC1SEL_SH: u64 = 24;
const MMCR1_PMCSEL_MSK: u64 = 0xff;

const fn mmcr1_ttm_sel_sh(n: u64) -> u64 { MMCR1_TTM0SEL_SH - n * 4 }
const fn mmcr1_ttm_sel(m: u64, n: u64) -> u64 { (m >> mmcr1_ttm_sel_sh(n)) & 0xf }
const fn mmcr1_nestsel(m: u64) -> u64 { (m >> MMCR1_NESTSEL_SH) & MMCR1_NESTSEL_MSK }
const fn mmcr1_pmcsel_sh(n: u64) -> u64 { MMCR1_PMC1SEL_SH - n * 8 }

static DIRECT_EVENT_IS_MARKED: [u8; 48] = [
    0,0,0,0x07,0x04,0x06,0,0,0x02,0x08,0,0,0x0c,0x0f,0x01,0,
    0,0,0,0,0,0,0,0,0x15,0,0,0,0x4f,0x7f,0x4f,0x5f,
    0x6f,0x4f,0,0x08,0x1f,0x1f,0x1f,0x1f,0x1f,0x1f,0x1f,0x1f,
    0,0x05,0x1c,0x02,0,0,0,0
];
static MARKED_BUS_EVENTS: [u32; 16] = [0x01000000,0x00010000,0,0,0,0,0x00000088,0x000000c0,0x04010000,0xff010000,0,0x00000010,0,0x00000022,0,0];

unsafe fn power6_marked_instr_event(event: u64) -> i32 {
    let pmc = ((event >> PM_PMC_SH) & PM_PMC_MSK) as i32;
    let psel = ((event & PM_PMCSEL_MSK) >> 1) as usize;
    if pmc >= 5 { return 0; }
    let mut bit: i32 = -1;
    if psel < DIRECT_EVENT_IS_MARKED.len() {
        let mut ptype = DIRECT_EVENT_IS_MARKED[psel];
        if pmc == 0 || (ptype & (1 << (pmc - 1))) == 0 { return 0; }
        ptype >>= 4;
        if ptype == 0 { return 1; }
        bit = if ptype == 1 { 0 } else { (ptype ^ (pmc as u8 - 1)) as i32 };
    } else if (psel as u64 & 0x48) == 0x40 { bit = (psel & 7) as i32; }
    if (event & PM_BUSEVENT_MSK) == 0 || bit == -1 { return 0; }
    let byte = ((event >> PM_BYTE_SH) & PM_BYTE_MSK) as u32;
    let unit = ((event >> PM_UNIT_SH) & PM_UNIT_MSK) as usize;
    ((MARKED_BUS_EVENTS[unit] >> (byte * 8 + bit as u32)) & 1) as i32
}

unsafe fn p6_compute_mmcr(event: *const u64, n_ev: i32, hwc: *mut u32, mmcr: *mut mmcr_regs, _pevents: *mut *mut perf_event, _flags: u32) -> i32 {
    let mut mmcr1 = 0u64;
    let mut mmcra = MMCRA_SDAR_DCACHE_MISS | MMCRA_SDAR_ERAT_MISS;
    if n_ev > 6 { return -1; }
    let mut pmc_inuse = 0u32;
    for i in 0..n_ev as isize { let pmc = ((*event.offset(i) >> PM_PMC_SH) & PM_PMC_MSK) as u32; if pmc != 0 { if pmc_inuse & (1 << (pmc-1)) != 0 { return -1; } pmc_inuse |= 1 << (pmc-1); } }
    let mut ttmset = 0u32;
    for i in 0..n_ev as isize {
        let ev = *event.offset(i); let requested = ((ev >> PM_PMC_SH) & PM_PMC_MSK) as u32;
        let mut pmc = if requested != 0 { requested - 1 } else { let mut p=0; while p<4 && pmc_inuse & (1<<p)!=0 { p+=1; } if p>=4{return -1;} pmc_inuse|=1<<p; p };
        *hwc.offset(i) = pmc;
        let mut psel = ev & PM_PMCSEL_MSK;
        if ev & PM_BUSEVENT_MSK != 0 { let b=((ev>>PM_BYTE_SH)&PM_BYTE_MSK) as u32; let u=((ev>>PM_UNIT_SH)&PM_UNIT_MSK) as u64; if ttmset&(1<<b)!=0 && mmcr1_ttm_sel(mmcr1,b as u64)!=u{return -1;} mmcr1|=u<<mmcr1_ttm_sel_sh(b as u64); ttmset|=1<<b; if u==5 { let s=(ev>>PM_SUBUNIT_SH)&PM_SUBUNIT_MSK; if ttmset&0x10!=0 && mmcr1_nestsel(mmcr1)!=s{return -1;} ttmset|=0x10; mmcr1|=s<<MMCR1_NESTSEL_SH; } if psel>=0x30&&psel<=0x3d&&b>=2 {mmcr1|=MMCR1_PMC1_ADDR_SEL>>pmc;} if pmc>=2&&(psel&0x90)==0x80 {psel^=0x20;} }
        if ev&PM_LLA!=0 {mmcr1|=MMCR1_PMC1_LLA>>pmc; if ev&PM_LLAV!=0 {mmcr1|=MMCR1_PMC1_LLA_VALUE>>pmc;} }
        if power6_marked_instr_event(ev)!=0 {mmcra|=MMCRA_SAMPLE_ENABLE;} if pmc<4 {mmcr1|=psel<<mmcr1_pmcsel_sh(pmc as u64);}
    }
    (*mmcr).mmcr0=0; if pmc_inuse&1!=0 {(*mmcr).mmcr0=MMCR0_PMC1CE;} if pmc_inuse&0xe!=0 {(*mmcr).mmcr0|=MMCR0_PMCjCE;} (*mmcr).mmcr1=mmcr1; (*mmcr).mmcra=mmcra; 0
}

unsafe fn p6_get_constraint(event:u64, maskp:*mut u64, valp:*mut u64, _event_config1:u64)->i32 { let pmc=((event>>PM_PMC_SH)&PM_PMC_MSK) as u64; let mut mask=0; let mut value=0; if pmc!=0 {if pmc>4 && event!=0x500009 && event!=0x600005{return -1;} let sh=(pmc-1)*2; mask|=2<<sh; value|=1<<sh;} if event&PM_BUSEVENT_MSK!=0 {let byte=(event>>PM_BYTE_SH)&PM_BYTE_MSK; let sh=byte*4+(16-PM_UNIT_SH); mask|=PM_UNIT_MSKS<<sh; value|=(event&PM_UNIT_MSKS)<<sh; if event&PM_UNIT_MSKS==5<<PM_UNIT_SH {mask|=PM_SUBUNIT_MSK<<32; value|=((event>>PM_SUBUNIT_SH)&PM_SUBUNIT_MSK)<<32;}} if pmc<=4 {mask|=0x8000;value|=0x1000;} *maskp=mask;*valp=value;0 }
unsafe fn p6_limited_pmc_event(event:u64)->bool { let pmc=(event>>PM_PMC_SH)&PM_PMC_MSK; pmc==5||pmc==6 }
const MAX_ALT: usize=4;
static EVENT_ALTERNATIVES:[[u32;4];22]=[[0x0130e8,0x2000f6,0x3000fc,0],[0x080080,0x10000d,0x30000c,0x4000f0],[0x080088,0x200054,0x3000f0,0],[0x10000a,0x2000f4,0x600005,0],[0x10000b,0x2000f5,0,0],[0x10000e,0x400010,0,0],[0x100010,0x4000f8,0,0],[0x10001a,0x200010,0,0],[0x100026,0x3000f8,0,0],[0x100054,0x2000f0,0,0],[0x100056,0x2000fc,0,0],[0x1000f0,0x40000a,0,0],[0x1000f8,0x200008,0,0],[0x1000fc,0x400006,0,0],[0x20000e,0x400007,0,0],[0x200012,0x300012,0,0],[0x2000f2,0x3000f2,0,0],[0x2000f8,0x300010,0,0],[0x2000fe,0x300056,0,0],[0x2d0030,0x30001a,0,0],[0x30000a,0x400018,0,0],[0x3000f6,0x40000e,0,0],[0x3000fe,0x400056,0,0]];

unsafe fn find_alternatives_list(event:u64)->i32 { const T:[u32;51]=[0x0130e8,0x080080,0x080088,0x10000a,0x10000b,0x10000d,0x10000e,0x100010,0x10001a,0x100026,0x100054,0x100056,0x1000f0,0x1000f8,0x1000fc,0x200008,0x20000e,0x200010,0x200012,0x200054,0x2000f0,0x2000f2,0x2000f4,0x2000f5,0x2000f6,0x2000f8,0x2000fc,0x2000fe,0x2d0030,0x30000a,0x30000c,0x300010,0x300012,0x30001a,0x300056,0x3000f0,0x3000f2,0x3000f6,0x3000f8,0x3000fc,0x3000fe,0x400006,0x400007,0x40000a,0x40000e,0x400010,0x400018,0x400056,0x4000f0,0x4000f8,0x600005]; const I:[i32;51]=[0,1,2,3,4,1,5,6,7,8,9,10,11,12,13,12,14,7,15,2,9,16,3,4,0,17,10,18,19,20,1,17,15,19,18,2,16,21,8,0,22,13,14,11,21,5,20,22,1,6,3]; let(mut lo,mut hi)=(0i32,50i32); while lo<=hi {let m=lo+(hi-lo)/2; if T[m as usize]<event as u32{lo=m+1}else if T[m as usize]>event as u32{hi=m-1}else{return I[m as usize];}} -1 }

unsafe fn p6_get_alternatives(event:u64,flags:u32,alt:*mut u64)->i32 { *alt=event; let mut nalt=1i32; let mut nlim=if p6_limited_pmc_event(event){1}else{0}; let i=find_alternatives_list(event); if i>=0 {for a in EVENT_ALTERNATIVES[i as usize] {if a==0{break} if a as u64!=event {*alt.offset(nalt as isize)=a as u64;nalt+=1;} if p6_limited_pmc_event(a as u64){nlim+=1;}}} else {let psel=event&(PM_PMCSEL_MSK&!1);let pmc=(event>>PM_PMC_SH)&PM_PMC_MSK;if pmc!=0&&(psel==0x32||psel==0x34){*alt.offset(nalt as isize)=((event^6)&!PM_PMC_MSKS)|((5-pmc)<<PM_PMC_SH);nalt+=1;}if pmc!=0&&(psel==0x38||psel==0x3a){*alt.offset(nalt as isize)=((event^2)&!PM_PMC_MSKS)|((if pmc>2{pmc-2}else{pmc+2})<<PM_PMC_SH);nalt+=1;}} if flags&PPMU_ONLY_COUNT_RUN!=0 {let j=nalt; for k in 0..j {let v=*alt.offset(k as isize);let x=match v{0x1e=>0x600005,0x10000a=>0x1e,2=>0x500009,0x500009=>2,0x10000e=>0x4000f4,0x4000f4=>0x10000e,_=>continue};*alt.offset(nalt as isize)=x;nalt+=1;} } if flags&PPMU_LIMITED_PMC_OK==0&&nlim!=0 {let mut j=0;for k in 0..nalt{let v=*alt.offset(k as isize);if !p6_limited_pmc_event(v){*alt.offset(j as isize)=v;j+=1;}}nalt=j;}else if flags&PPMU_LIMITED_PMC_REQD!=0&&nlim<nalt {let mut j=0;for k in 0..nalt{let v=*alt.offset(k as isize);if p6_limited_pmc_event(v){*alt.offset(j as isize)=v;j+=1;}}nalt=j;}nalt}

unsafe fn p6_disable_pmc(pmc:u32,mmcr:*mut mmcr_regs){if pmc<=3{(*mmcr).mmcr1&=!(0xffu64<<mmcr1_pmcsel_sh(pmc as u64));}}

// External kernel types, constants, and registration symbols are supplied by other translated files.
static mut POWER6_GENERIC_EVENTS:[i32;6]=[0x1e,2,0x280030,0x30000c,0x410a0,0x400052];
static mut POWER6_CACHE_EVENTS:[[[u64;2];3];8]=[
    [[0x280030,0x80080],[0x180032,0x80088],[0x810a4,0]],
    [[0,0x100056],[-1i64 as u64,-1i64 as u64],[0x4008c,0]],
    [[0x150730,0x250532],[0x250432,0x150432],[0x810a6,0]],
    [[0,0x20000e],[-1i64 as u64,-1i64 as u64],[-1i64 as u64,-1i64 as u64]],
    [[0,0x420ce],[-1i64 as u64,-1i64 as u64],[-1i64 as u64,-1i64 as u64]],
    [[0x430e6,0x400052],[-1i64 as u64,-1i64 as u64],[-1i64 as u64,-1i64 as u64]],
    [[-1i64 as u64,-1i64 as u64],[-1i64 as u64,-1i64 as u64],[-1i64 as u64,-1i64 as u64]],
];
static mut POWER6_PMU: power_pmu = power_pmu { name:"POWER6", n_counter:6, max_alternatives:MAX_ALT, add_fields:0x1555, test_adder:0x3000, compute_mmcr:p6_compute_mmcr, get_constraint:p6_get_constraint, get_alternatives:p6_get_alternatives, disable_pmc:p6_disable_pmc, limited_pmc_event:p6_limited_pmc_event, flags:PPMU_LIMITED_PMC5_6|PPMU_ALT_SIPR, n_generic:6, generic_events:POWER6_GENERIC_EVENTS.as_ptr(), cache_events:POWER6_CACHE_EVENTS.as_ptr() };
unsafe fn init_power6_pmu()->i32 {let pvr=mfspr(SPRN_PVR);if PVR_VER(pvr)!=PVR_POWER6{return -ENODEV;}register_power_pmu(&mut POWER6_PMU)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
