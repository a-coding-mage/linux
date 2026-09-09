// SPDX-License-Identifier: GPL-2.0-or-later
/* Performance counter support for POWER5 (not POWER5++) processors. */

// Kernel dependencies supplied by the surrounding translation unit.

const PM_PMC_SH: u64 = 20;
const PM_PMC_MSK: u64 = 0xf;
const PM_PMC_MSKS: u64 = PM_PMC_MSK << PM_PMC_SH;
const PM_UNIT_SH: u64 = 16;
const PM_UNIT_MSK: u64 = 0xf;
const PM_BYTE_SH: u64 = 12;
const PM_BYTE_MSK: u64 = 7;
const PM_GRS_SH: u64 = 8;
const PM_GRS_MSK: u64 = 7;
const PM_BUSEVENT_MSK: u64 = 0x80;
const PM_PMCSEL_MSK: u64 = 0x7f;

const PM_FPU: usize = 0;
const PM_ISU0: usize = 1;
const PM_IFU: usize = 2;
const PM_ISU1: usize = 3;
const PM_IDU: usize = 4;
const PM_ISU0_ALT: usize = 6;
const PM_GRS: usize = 7;
const PM_LSU0: usize = 8;
const PM_LSU1: usize = 0xc;
const PM_LASTUNIT: usize = 0xc;

const MMCR1_TTM0SEL_SH: u32 = 62;
const MMCR1_TTM1SEL_SH: u32 = 60;
const MMCR1_TTM2SEL_SH: u32 = 58;
const MMCR1_TTM3SEL_SH: u32 = 56;
const MMCR1_TD_CP_DBG0SEL_SH: u32 = 54;
const MMCR1_GRS_L2SEL_SH: u32 = 46;
const MMCR1_GRS_L3SEL_SH: u32 = 44;
const MMCR1_GRS_MCSEL_SH: u32 = 41;
const MMCR1_GRS_FABSEL_SH: u32 = 39;
const MMCR1_PMC1_ADDER_SEL_SH: u32 = 35;
const MMCR1_PMCSEL_MSK: u64 = 0x7f;

#[inline]
const fn mmcr1_pmcsel_sh(n: usize) -> u32 { 25 - (n as u32) * 8 }

static GRSEL_SHIFT: [u32; 8] = [
    MMCR1_GRS_L2SEL_SH, MMCR1_GRS_L2SEL_SH, MMCR1_GRS_L2SEL_SH,
    MMCR1_GRS_L3SEL_SH, MMCR1_GRS_L3SEL_SH, MMCR1_GRS_L3SEL_SH,
    MMCR1_GRS_MCSEL_SH, MMCR1_GRS_FABSEL_SH,
];

static UNIT_CONS: [[u64; 2]; PM_LASTUNIT + 1] = [
    [0xc0002000000000, 0x00001000000000],
    [0x00002000000000, 0x00000800000000],
    [0, 0],
    [0xc0002000000000, 0xc0001000000000],
    [0x30002000000000, 0x00000400000000],
    [0, 0],
    [0, 0],
    [0x30002000000000, 0x30000400000000],
    [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],
];

unsafe fn power5_get_constraint(event: u64, maskp: *mut u64, valp: *mut u64, _event_config1: u64) -> i32 {
    let mut pmc = ((event >> PM_PMC_SH) & PM_PMC_MSK) as usize;
    let mut mask = 0u64; let mut value = 0u64; let mut grp: i32 = -1;
    if pmc != 0 {
        if pmc > 6 { return -1; }
        let sh = (pmc - 1) * 2; mask |= 2 << sh; value |= 1 << sh;
        if pmc <= 4 { grp = ((pmc - 1) >> 1) as i32; }
        else if event != 0x500009 && event != 0x600005 { return -1; }
    }
    if event & PM_BUSEVENT_MSK != 0 {
        let mut unit = ((event >> PM_UNIT_SH) & PM_UNIT_MSK) as usize;
        if unit > PM_LASTUNIT { return -1; }
        if unit == PM_ISU0_ALT { unit = PM_ISU0; }
        mask |= UNIT_CONS[unit][0]; value |= UNIT_CONS[unit][1];
        let mut byte = ((event >> PM_BYTE_SH) & PM_BYTE_MSK) as usize;
        if byte >= 4 { if unit != PM_LSU1 { return -1; } unit += 1; byte &= 3; }
        if unit == PM_GRS {
            let bit = (event & 7) as usize; let fmask = if bit == 6 { 7 } else { 3 };
            let sh = GRSEL_SHIFT[bit]; mask |= (fmask as u64) << sh;
            value |= (((event >> PM_GRS_SH) & fmask) as u64) << sh;
        }
        if pmc == 0 { grp = (byte & 1) as i32; }
        mask |= 0xfu64 << (24 - 4 * byte); value |= (unit as u64) << (24 - 4 * byte);
    }
    if grp == 0 { mask |= 0x200000000; value |= 0x080000000; }
    else if grp == 1 { mask |= 0x40000000; value |= 0x10000000; }
    if pmc < 5 { mask |= 0x8000000000000; value |= 0x1000000000000; }
    *maskp = mask; *valp = value; 0
}

const MAX_ALT: usize = 3;
static EVENT_ALTERNATIVES: [[u32; MAX_ALT]; 5] = [
    [0x120e4, 0x400002, 0], [0x410c7, 0x441084, 0], [0x100005, 0x600005, 0],
    [0x100009, 0x200009, 0x500009], [0x300009, 0x400009, 0],
];

fn find_alternative(event: u64) -> i32 {
    for (i, row) in EVENT_ALTERNATIVES.iter().enumerate() {
        if event < row[0] as u64 { break; }
        for &v in row { if v != 0 && event == v as u64 { return i as i32; } }
    } -1
}

static BYTEDECODE_ALTERNATIVES: [[u8; 4]; 4] = [[0x21,0x23,0x25,0x27],[0x07,0x17,0x0e,0x1e],[0x20,0x22,0x24,0x26],[0x07,0x17,0x0e,0x1e]];

fn find_alternative_bdecode(event: u64) -> i64 {
    let pmc = ((event >> PM_PMC_SH) & PM_PMC_MSK) as usize;
    if pmc == 0 || pmc > 4 { return -1; }
    let altpmc = 5 - pmc; let pp = (event & PM_PMCSEL_MSK) as u8;
    for j in 0..4 { if BYTEDECODE_ALTERNATIVES[pmc-1][j] == pp {
        return ((event & !(PM_PMC_MSKS | PM_PMCSEL_MSK)) | ((altpmc as u64) << PM_PMC_SH) | BYTEDECODE_ALTERNATIVES[altpmc-1][j] as u64) as i64;
    }} -1
}

unsafe fn power5_get_alternatives(event: u64, _flags: u32, alt: *mut u64) -> i32 {
    *alt = event; let mut nalt = 1; let i = find_alternative(event);
    if i >= 0 { for &ae in &EVENT_ALTERNATIVES[i as usize] { if ae != 0 && ae as u64 != event { *alt.add(nalt as usize) = ae as u64; nalt += 1; } } }
    else { let ae = find_alternative_bdecode(event); if ae > 0 { *alt.add(nalt as usize) = ae as u64; nalt += 1; } }
    nalt
}

static DIRECT_EVENT_IS_MARKED: [u8; 0x28] = [0,0x1f,2,0xe,0,0x1c,0x80,0x80,0,0,0,0x18,0,0x80,0x80,0,0,0x14,0,0x10,0x1f,2,0x80,0x80,0,0,0,0,0,0x80,0x80,0,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80];

fn power5_marked_instr_event(event: u64) -> i32 {
    let pmc = ((event >> PM_PMC_SH) & PM_PMC_MSK) as usize; let psel = (event & PM_PMCSEL_MSK) as usize;
    if pmc >= 5 { return 0; }
    let mut bit: i32 = -1;
    if psel < DIRECT_EVENT_IS_MARKED.len() { let d = DIRECT_EVENT_IS_MARKED[psel]; if d & (1 << pmc) != 0 { return 1; } if d & 0x80 != 0 { bit = 4; } else if psel == 8 { bit = pmc as i32 - 1; } else if psel == 0x10 { bit = 4 - pmc as i32; } else if psel == 0x1b && (pmc == 1 || pmc == 3) { bit = 4; } }
    else if psel & 0x58 == 0x40 { bit = (psel & 7) as i32; }
    if event & PM_BUSEVENT_MSK == 0 { return 0; }
    let mut byte = ((event >> PM_BYTE_SH) & PM_BYTE_MSK) as usize; let unit = ((event >> PM_UNIT_SH) & PM_UNIT_MSK) as usize;
    let mask: u32; if unit == PM_LSU0 { mask = 0x5dff00; } else if unit == PM_LSU1 && byte >= 4 { byte -= 4; mask = 0x5f00c0aa; } else { return 0; }
    ((mask >> (byte * 8 + bit as usize)) & 1) as i32
}

// The remaining callback uses kernel-provided `mmcr_regs`, `perf_event`, PMU constants, and registration APIs.
unsafe fn power5_disable_pmc(pmc: u32, mmcr: *mut mmcr_regs) { if pmc <= 3 { (*mmcr).mmcr1 &= !(0x7f_u64 << mmcr1_pmcsel_sh(pmc as usize)); } }

unsafe fn power5_compute_mmcr(event: *mut u64, n_ev: i32, hwc: *mut u32, mmcr: *mut mmcr_regs, _pevents: *mut *mut perf_event, _flags: u32) -> i32 {
    let mut mmcr1 = 0u64;
    let mut mmcra = MMCRA_SDAR_DCACHE_MISS | MMCRA_SDAR_ERAT_MISS;
    let mut pmc_inuse = 0u32; let mut pmc_grp_use = [0u32; 2];
    let mut busbyte = [0u8; 4]; let mut unituse = [0u8; 16];
    if n_ev > 6 { return -1; }
    for i in 0..n_ev as usize {
        let e = *event.add(i); let mut pmc = ((e >> PM_PMC_SH) & PM_PMC_MSK) as usize;
        if pmc != 0 { if pmc > 6 || pmc_inuse & (1 << (pmc - 1)) != 0 { return -1; } pmc_inuse |= 1 << (pmc - 1); if pmc <= 4 { pmc_grp_use[(pmc-1)>>1] += 1; } }
        if e & PM_BUSEVENT_MSK != 0 {
            let mut unit = ((e >> PM_UNIT_SH) & PM_UNIT_MSK) as usize; let mut byte = ((e >> PM_BYTE_SH) & PM_BYTE_MSK) as usize;
            if unit > PM_LASTUNIT { return -1; } if unit == PM_ISU0_ALT { unit = PM_ISU0; }
            if byte >= 4 { if unit != PM_LSU1 { return -1; } unit += 1; byte &= 3; }
            if pmc == 0 { pmc_grp_use[byte & 1] += 1; }
            if busbyte[byte] != 0 && busbyte[byte] != unit as u8 { return -1; } busbyte[byte] = unit as u8; unituse[unit] = 1;
        }
    }
    if pmc_grp_use[0] > 2 || pmc_grp_use[1] > 2 { return -1; }
    if unituse[PM_ISU0] != 0 && (unituse[PM_FPU] | unituse[PM_IFU] | unituse[PM_ISU1]) != 0 { unituse[PM_ISU0_ALT] = 1; unituse[PM_ISU0] = 0; }
    let mut ttmuse = 0; for i in PM_FPU..=PM_ISU1 { if unituse[i] != 0 { if ttmuse != 0 { return -1; } ttmuse += 1; mmcr1 |= (i as u64) << MMCR1_TTM0SEL_SH; } }
    ttmuse = 0; for i in PM_IDU..=PM_GRS { if unituse[i] != 0 { if ttmuse != 0 { return -1; } ttmuse += 1; mmcr1 |= ((i & 3) as u64) << MMCR1_TTM1SEL_SH; } }
    for byte in 0..4 { let mut unit = busbyte[byte] as usize; if unit == 0 { continue; } if unit == PM_ISU0 && unituse[PM_ISU0_ALT] != 0 { unit = PM_ISU0_ALT; } else if unit == PM_LSU1 + 1 { mmcr1 |= 1u64 << (MMCR1_TTM3SEL_SH + 3 - byte); } mmcr1 |= ((unit >> 2) as u64) << (MMCR1_TD_CP_DBG0SEL_SH - 2 * byte); }
    for i in 0..n_ev as usize {
        let e = *event.add(i); let mut pmc = ((e >> PM_PMC_SH) & PM_PMC_MSK) as usize; let unit = ((e >> PM_UNIT_SH) & PM_UNIT_MSK) as usize; let byte = ((e >> PM_BYTE_SH) & PM_BYTE_MSK) as usize; let psel = e & PM_PMCSEL_MSK; let isbus = e & PM_BUSEVENT_MSK != 0;
        if pmc == 0 { for p in 0..4 { if pmc_inuse & (1 << p) == 0 && (!isbus || (p >> 1) == (byte & 1)) { pmc = p; break; } } pmc_inuse |= 1 << pmc; } else { pmc -= 1; }
        if isbus && unit == PM_GRS { let bit = (psel & 7) as usize; mmcr1 |= (((e >> PM_GRS_SH) & PM_GRS_MSK) as u64) << GRSEL_SHIFT[bit]; }
        if power5_marked_instr_event(e) != 0 { mmcra |= MMCRA_SAMPLE_ENABLE; } if pmc <= 3 { mmcr1 |= psel << mmcr1_pmcsel_sh(pmc); } *hwc.add(i) = pmc as u32;
    }
    (*mmcr).mmcr0 = if pmc_inuse & 1 != 0 { MMCR0_PMC1CE } else { 0 }; if pmc_inuse & 0x3e != 0 { (*mmcr).mmcr0 |= MMCR0_PMCjCE; } (*mmcr).mmcr1 = mmcr1; (*mmcr).mmcra = mmcra; 0
}

static POWER5_GENERIC_EVENTS: [i32; 6] = [0xf, 0x100009, 0x4c1090, 0x3c1088, 0x230e4, 0x230e5];

// Direct event tables and PMU registration are retained as declarations for surrounding kernel bindings.
extern "C" { static mut power5_pmu: power_pmu; fn mfspr(reg: u32) -> u32; fn register_power_pmu(pmu: *mut power_pmu) -> i32; }

#[no_mangle]
pub unsafe extern "C" fn init_power5_pmu() -> i32 {
    let pvr = mfspr(SPRN_PVR);
    if PVR_VER(pvr) != PVR_POWER5 { return -ENODEV; }
    register_power_pmu(&raw mut power5_pmu)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
