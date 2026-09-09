// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Performance counter support for PPC970-family processors.
 *
 * Copyright 2008-2009 Paul Mackerras, IBM Corporation.
 */

// Kernel dependencies supplied externally: linux string/perf_event, asm reg/cputable, internal.

const PM_PMC_SH: u32 = 12;
const PM_PMC_MSK: u64 = 0xf;
const PM_UNIT_SH: u32 = 8;
const PM_UNIT_MSK: u64 = 0xf;
const PM_SPCSEL_SH: u32 = 6;
const PM_SPCSEL_MSK: u64 = 3;
const PM_BYTE_SH: u32 = 4;
const PM_BYTE_MSK: u64 = 3;
const PM_PMCSEL_MSK: u64 = 0xf;

const PM_NONE: usize = 0;
const PM_FPU: usize = 1;
const PM_VPU: usize = 2;
const PM_ISU: usize = 3;
const PM_IFU: usize = 4;
const PM_IDU: usize = 5;
const PM_STS: usize = 6;
const PM_LSU0: usize = 7;
const PM_LSU1U: usize = 8;
const PM_LSU1L: usize = 9;
const PM_LASTUNIT: usize = 9;

const MMCR0_PMC1SEL_SH: u32 = 8;
const MMCR0_PMC2SEL_SH: u32 = 1;
const MMCR_PMCSEL_MSK: u64 = 0x1f;
const MMCR1_TTM0SEL_SH: u32 = 62;
const MMCR1_TTM1SEL_SH: u32 = 59;
const MMCR1_TTM3SEL_SH: u32 = 53;
const MMCR1_TTMSEL_MSK: u64 = 3;
const MMCR1_TD_CP_DBG0SEL_SH: u32 = 50;
const MMCR1_TD_CP_DBG1SEL_SH: u32 = 48;
const MMCR1_TD_CP_DBG2SEL_SH: u32 = 46;
const MMCR1_TD_CP_DBG3SEL_SH: u32 = 44;
const MMCR1_PMC1_ADDER_SEL_SH: u32 = 39;
const MMCR1_PMC2_ADDER_SEL_SH: u32 = 38;
const MMCR1_PMC6_ADDER_SEL_SH: u32 = 37;
const MMCR1_PMC5_ADDER_SEL_SH: u32 = 36;
const MMCR1_PMC8_ADDER_SEL_SH: u32 = 35;
const MMCR1_PMC7_ADDER_SEL_SH: u32 = 34;
const MMCR1_PMC3_ADDER_SEL_SH: u32 = 33;
const MMCR1_PMC4_ADDER_SEL_SH: u32 = 32;
const MMCR1_PMC3SEL_SH: u32 = 27;
const MMCR1_PMC4SEL_SH: u32 = 22;
const MMCR1_PMC5SEL_SH: u32 = 17;
const MMCR1_PMC6SEL_SH: u32 = 12;
const MMCR1_PMC7SEL_SH: u32 = 7;
const MMCR1_PMC8SEL_SH: u32 = 2;

static mut mmcr1_adder_bits: [i16; 8] = [39, 38, 33, 32, 36, 37, 34, 35];

static mut direct_marked_event: [u8; 8] = [
    (1 << 2) | (1 << 3), (1 << 3) | (1 << 5), (1 << 3) | (1 << 5),
    (1 << 4) | (1 << 5), (1 << 4) | (1 << 5), (1 << 3) | (1 << 4) | (1 << 5),
    (1 << 4) | (1 << 5), 1 << 4,
];

unsafe fn p970_marked_instr_event(event: u64) -> i32 {
    let pmc = ((event >> PM_PMC_SH) & PM_PMC_MSK) as usize;
    let psel = event & PM_PMCSEL_MSK;
    let bit: usize;
    if pmc != 0 {
        if direct_marked_event[pmc - 1] & (1u8 << psel) != 0 { return 1; }
        if psel == 0 { bit = if pmc <= 4 { pmc - 1 } else { 8 - pmc }; }
        else if psel == 7 || psel == 13 { bit = 4; }
        else { return 0; }
    } else { bit = psel as usize; }
    let byte = ((event >> PM_BYTE_SH) & PM_BYTE_MSK) as usize;
    let unit = ((event >> PM_UNIT_SH) & PM_UNIT_MSK) as usize;
    let mut mask: u32 = 0;
    match unit { PM_VPU => mask = 0x4c, PM_LSU0 => mask = 0x085dff00,
        PM_LSU1L => mask = 0x50 << 24, _ => {} }
    ((mask >> (byte * 8 + bit)) & 1) as i32
}

static mut unit_cons: [[u64; 2]; PM_LASTUNIT + 1] = [
    [0, 0], [0xc80000000000, 0x040000000000], [0xc80000000000, 0xc40000000000],
    [0x080000000000, 0x020000000000], [0xc80000000000, 0x840000000000],
    [0x380000000000, 0x010000000000], [0x380000000000, 0x310000000000],
    [0, 0], [0, 0], [0, 0],
];

unsafe fn p970_get_constraint(event: u64, maskp: *mut u64, valp: *mut u64, _event_config1: u64) -> i32 {
    let pmc = ((event >> PM_PMC_SH) & PM_PMC_MSK) as usize;
    let mut mask = 0u64; let mut value = 0u64; let mut grp: i32 = -1;
    if pmc != 0 { if pmc > 8 { return -1; } let sh = (pmc - 1) * 2; mask |= 2 << sh; value |= 1 << sh; grp = (((pmc - 1) >> 1) & 1) as i32; }
    let unit = ((event >> PM_UNIT_SH) & PM_UNIT_MSK) as usize;
    if unit != 0 { if unit > PM_LASTUNIT { return -1; } mask |= unit_cons[unit][0]; value |= unit_cons[unit][1]; let byte = ((event >> PM_BYTE_SH) & PM_BYTE_MSK) as usize; if pmc == 0 { grp = (byte & 1) as i32; } mask |= 0xfu64 << (28 - 4 * byte); value |= (unit as u64) << (28 - 4 * byte); }
    if grp == 0 { mask |= 0x8000000000; value |= 0x1000000000; } else if grp == 1 { mask |= 0x800000000; value |= 0x100000000; }
    let spcsel = (event >> PM_SPCSEL_SH) & PM_SPCSEL_MSK;
    if spcsel != 0 { mask |= 3u64 << 48; value |= spcsel << 48; }
    *maskp = mask; *valp = value; 0
}

unsafe fn p970_get_alternatives(event: u64, _flags: u32, alt: *mut u64) -> i32 {
    *alt = event;
    if event == 0x2002 || event == 0x3002 { *alt.add(1) = event ^ 0x1000; return 2; }
    1
}

unsafe extern "C" fn p970_compute_mmcr(event: *mut u64, n_ev: i32, hwc: *mut u32, mmcr: *mut mmcr_regs, _pevents: *mut *mut perf_event, _flags: u32) -> i32 {
    let mut mmcr0=0u64; let mut mmcr1=0u64; let mut mmcra=0u64; let mut pmc_inuse=0u32; let mut pmc_grp_use=[0u32;2];
    let mut busbyte=[0u8;4]; let mut unituse=[0u8;16]; let mut unitmap=[0u8,0,24,8,16,4,7,0,0,0]; let mut ttmuse=[0u8;2]; let mut pmcsel=[8u8;8];
    if n_ev > 8 { return -1; }
    for i in 0..n_ev as usize { let e=*event.add(i); let pmc=((e>>PM_PMC_SH)&PM_PMC_MSK) as usize; if pmc!=0 { if pmc_inuse&(1<<(pmc-1))!=0{return -1;} pmc_inuse|=1<<(pmc-1); pmc_grp_use[((pmc-1)>>1)&1]+=1; } let unit=((e>>PM_UNIT_SH)&PM_UNIT_MSK) as usize; let byte=((e>>PM_BYTE_SH)&PM_BYTE_MSK) as usize; if unit!=0 { if unit>PM_LASTUNIT{return -1;} if pmc==0 {pmc_grp_use[byte&1]+=1;} if busbyte[byte]!=0&&busbyte[byte]!=unit as u8{return -1;} busbyte[byte]=unit as u8; unituse[unit]=1; } }
    if pmc_grp_use[0]>4||pmc_grp_use[1]>4{return -1;}
    if unituse[PM_ISU]!=0&&(unituse[PM_FPU]!=0||unituse[PM_IFU]!=0||unituse[PM_VPU]!=0){unitmap[PM_ISU]=6;}
    for i in PM_FPU..=PM_STS {if unituse[i]!=0 {let t=unitmap[i]; ttmuse[((t>>2)&1) as usize]+=1; mmcr1|=((t&!4) as u64)<<MMCR1_TTM1SEL_SH;}}
    if ttmuse[0]>1||ttmuse[1]>1{return -1;}
    for byte in 0..4 {let unit=busbyte[byte] as usize;if unit==0{continue;}let t=if unit<=PM_STS{((unitmap[unit]>>2)&1) as u64}else if unit==PM_LSU0{2}else{if unit==PM_LSU1L&&byte>=2{mmcr1|=1u64<<(MMCR1_TTM3SEL_SH+3-byte as u32);}3};mmcr1<<=0;mmcr1|=t<<(MMCR1_TD_CP_DBG0SEL_SH-2*byte as u32);}
    for i in 0..n_ev as usize {let e=*event.add(i);let mut pmc=((e>>PM_PMC_SH)&PM_PMC_MSK) as usize;let unit=((e>>PM_UNIT_SH)&PM_UNIT_MSK) as usize;let byte=((e>>PM_BYTE_SH)&PM_BYTE_MSK) as usize;let mut psel=(e&PM_PMCSEL_MSK) as u8;if pmc==0 {if unit!=0{psel|=0x10|((byte&2)<<2) as u8;}else{psel|=8;}for p in 0..8 {if pmc_inuse&(1<<p)!=0{continue;}let g=(p>>1)&1;if unit!=0 {if g==(byte&1){pmc=p;break;}}else if pmc_grp_use[g]<4{pmc_grp_use[g]+=1;pmc=p;break;}}pmc_inuse|=1<<pmc;}else{pmc-=1;if psel==0&&(byte&2)!=0{mmcr1|=1u64<<(mmcr1_adder_bits[pmc] as u32);}}pmcsel[pmc]=psel;*hwc.add(i)=pmc as u32;mmcr1|=((e>>PM_SPCSEL_SH)&PM_SPCSEL_MSK);if p970_marked_instr_event(e)!=0{mmcra|=MMCRA_SAMPLE_ENABLE as u64;}}
    for pmc in 0..2 {mmcr0|=(pmcsel[pmc] as u64)<<(MMCR0_PMC1SEL_SH-7*pmc as u32);}for pmc in 2..8{mmcr1|=(pmcsel[pmc] as u64)<<(MMCR1_PMC3SEL_SH-5*(pmc-2) as u32);}if pmc_inuse&1!=0{mmcr0|=MMCR0_PMC1CE as u64;}if pmc_inuse&0xfe!=0{mmcr0|=MMCR0_PMCjCE as u64;}mmcra|=0x2000;(*mmcr).mmcr0=mmcr0;(*mmcr).mmcr1=mmcr1;(*mmcr).mmcra=mmcra;0
}

// Remaining declarations retain the C ABI and external kernel types.
extern "C" {
    fn register_power_pmu(pmu: *mut power_pmu) -> i32;
    fn mfspr(spr: u32) -> u32;
}

#[repr(C)] pub struct mmcr_regs { pub mmcr0: u64, pub mmcr1: u64, pub mmcra: u64 }
#[repr(C)] pub struct perf_event;
#[repr(C)] pub struct power_pmu {
    pub name: *const u8, pub n_counter: i32, pub max_alternatives: i32,
    pub add_fields: u64, pub test_adder: u64,
    pub compute_mmcr: Option<unsafe extern "C" fn(*mut u64, i32, *mut u32, *mut mmcr_regs, *mut *mut perf_event, u32) -> i32>,
    pub get_constraint: Option<unsafe extern "C" fn(u64, *mut u64, *mut u64, u64) -> i32>,
    pub get_alternatives: Option<unsafe extern "C" fn(u64, u32, *mut u64) -> i32>,
    pub disable_pmc: Option<unsafe extern "C" fn(u32, *mut mmcr_regs)>,
    pub n_generic: usize, pub generic_events: *mut i32, pub cache_events: *const u64, pub flags: u32,
}

unsafe extern "C" fn p970_disable_pmc(pmc: u32, mmcr: *mut mmcr_regs) {
    let shift: u32;
    if pmc <= 1 { shift = MMCR0_PMC1SEL_SH - 7 * pmc; (*mmcr).mmcr0 = ((*mmcr).mmcr0 & !(0x1f_u64 << shift)) | (0x08_u64 << shift); }
    else { shift = MMCR1_PMC3SEL_SH - 5 * (pmc - 2); (*mmcr).mmcr1 = ((*mmcr).mmcr1 & !(0x1f_u64 << shift)) | (0x08_u64 << shift); }
}

static mut ppc970_generic_events: [i32; 6] = [7, 1, 0x8810, 0x3810, 0x431, 0x327];
static mut ppc970_cache_events: [u64; 7 * 3 * 2] = [0; 7 * 3 * 2];

static mut ppc970_pmu: power_pmu = power_pmu {
    name: b"PPC970/FX/MP\0".as_ptr(), n_counter: 8, max_alternatives: 2,
    add_fields: 0x001100005555, test_adder: 0x013300000000,
    compute_mmcr: Some(p970_compute_mmcr), get_constraint: Some(p970_get_constraint), get_alternatives: Some(p970_get_alternatives),
    disable_pmc: Some(p970_disable_pmc), n_generic: 6, generic_events: ppc970_generic_events.as_mut_ptr(),
    cache_events: ppc970_cache_events.as_ptr(), flags: 0,
};

pub unsafe fn init_ppc970_pmu() -> i32 {
    let pvr = mfspr(0);
    if PVR_VER(pvr) != PVR_970 && PVR_VER(pvr) != PVR_970MP && PVR_VER(pvr) != PVR_970FX && PVR_VER(pvr) != PVR_970GX { return -19; }
    register_power_pmu(&mut ppc970_pmu)
}

extern "C" { fn PVR_VER(pvr: u32) -> u32; }
extern "C" { static PVR_970: u32; static PVR_970MP: u32; static PVR_970FX: u32; static PVR_970GX: u32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
