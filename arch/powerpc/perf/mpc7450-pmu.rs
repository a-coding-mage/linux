// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Performance counter support for MPC7450-family processors.
 *
 * Copyright 2008-2009 Paul Mackerras, IBM Corporation.
 */

// Dependencies supplied by the surrounding kernel translation.

const N_COUNTER: usize = 6; /* Number of hardware counters */
const MAX_ALT: usize = 3; /* Maximum number of event alternative codes */

/* Bits in event code for MPC7450 family */
const PM_THRMULT_MSKS: u32 = 0x40000;
const PM_THRESH_SH: u32 = 12;
const PM_THRESH_MSK: u32 = 0x3f;
const PM_PMC_SH: u32 = 8;
const PM_PMC_MSK: u32 = 7;
const PM_PMCSEL_MSK: u32 = 0x7f;
const N_CLASSES: usize = 5;

unsafe fn mpc7450_classify_event(mut event: u32) -> i32 {
    let pmc = (event >> PM_PMC_SH) & PM_PMC_MSK;
    if pmc != 0 {
        if pmc > N_COUNTER as u32 { return -1; }
        return 4;
    }
    event &= PM_PMCSEL_MSK;
    if event <= 1 { return 0; }
    if event <= 7 { return 1; }
    if event <= 13 { return 2; }
    if event <= 22 { return 3; }
    -1
}

/* Return use of threshold and threshold scale bits: 0 = neither, 1 = threshold, 2 = both. */
unsafe fn mpc7450_threshold_use(event: u32) -> i32 {
    let pmc = (event >> PM_PMC_SH) & PM_PMC_MSK;
    let sel = event & PM_PMCSEL_MSK;
    match pmc {
        1 => { if sel == 0x1e || sel == 0x1f { return 1; } if sel == 0x28 || sel == 0x2b { return 2; } }
        2 => { if sel == 0x20 { return 1; } }
        3 => { if sel == 0xc || sel == 0xd { return 1; } if sel == 0x11 { return 2; } }
        4 => { if sel == 0x10 { return 1; } }
        _ => {}
    }
    0
}

static mut pmcbits: [[u32; 2]; N_COUNTER] = [
    [0x00844002, 0x00111001], [0x00844008, 0x00111004],
    [0x00800020, 0x00100010], [0x00840080, 0x00110040],
    [0x00000200, 0x00000100], [0x00000800, 0x00000400],
];
static mut classbits: [[u32; 2]; N_CLASSES - 1] = [
    [0, 0], [0x00800000, 0x00100000], [0x00040000, 0x00010000], [0x00004000, 0x00001000],
];

unsafe fn mpc7450_get_constraint(event: u64, maskp: *mut usize, valp: *mut usize, _event_config1: u64) -> i32 {
    let class = mpc7450_classify_event(event as u32);
    if class < 0 { return -1; }
    let (mut mask, mut value): (u32, u32);
    if class == 4 {
        let pmc = ((event as u32) >> PM_PMC_SH) & PM_PMC_MSK;
        mask = pmcbits[(pmc - 1) as usize][0]; value = pmcbits[(pmc - 1) as usize][1];
    } else { mask = classbits[class as usize][0]; value = classbits[class as usize][1]; }
    let tuse = mpc7450_threshold_use(event as u32);
    if tuse != 0 {
        let thresh = ((event as u32) >> PM_THRESH_SH) & PM_THRESH_MSK;
        mask |= 0x3f << 24; value |= thresh << 24;
        if tuse == 2 { mask |= 0x40000000; if (event as u32) & PM_THRMULT_MSKS != 0 { value |= 0x40000000; } }
    }
    *maskp = mask as usize; *valp = value as usize; 0
}

static event_alternatives: [[u32; MAX_ALT]; 15] = [
    [0x217,0x317,0], [0x418,0x50f,0x60f], [0x502,0x602,0], [0x503,0x603,0],
    [0x504,0x604,0], [0x505,0x605,0], [0x506,0x606,0], [0x507,0x607,0],
    [0x50a,0x623,0], [0x50b,0x624,0], [0x50d,0x60d,0], [0x50e,0x60e,0],
    [0x512,0x612,0], [0x513,0x61d,0], [0x514,0x61e,0],
];

unsafe fn find_alternative(event: u32) -> i32 {
    for i in 0..event_alternatives.len() {
        if event < event_alternatives[i][0] { break; }
        for j in 0..MAX_ALT { if event_alternatives[i][j] == 0 { break; } if event == event_alternatives[i][j] { return i as i32; } }
    }
    -1
}

unsafe fn mpc7450_get_alternatives(event: u64, _flags: u32, alt: *mut u64) -> i32 {
    *alt = event; let i = find_alternative(event as u32); let mut nalt = 1;
    if i >= 0 { for j in 0..MAX_ALT { let ae = event_alternatives[i as usize][j]; if ae != 0 && ae != event as u32 { *alt.add(nalt as usize) = ae as u64; nalt += 1; } } }
    nalt
}

static classmap: [u8; N_CLASSES] = [0x3f, 0x0f, 0x0b, 0x03, 0];
static pmcsel_shift: [u32; N_COUNTER] = [6,0,27,22,17,11];
static pmcsel_mask: [u32; N_COUNTER] = [0x7f,0x3f,0x1f,0x1f,0x1f,0x3f];

/* Compute MMCR0/1/2 values for a set of events. */
unsafe fn mpc7450_compute_mmcr(event: *const u64, n_ev: i32, hwc: *mut u32, mmcr: *mut mmcr_regs, _pevents: *mut *mut perf_event, _flags: u32) -> i32 {
    if n_ev > N_COUNTER as i32 { return -1; }
    let mut event_index = [[0u8; N_COUNTER]; N_CLASSES]; let mut n_class = [0usize; N_CLASSES];
    for i in 0..n_ev as usize { let c = mpc7450_classify_event(*event.add(i)); if c < 0 { return -1; } let c = c as usize; let j = n_class[c]; n_class[c] += 1; event_index[c][j] = i as u8; }
    let mut pmc_inuse=0u32; let mut mmcr0=0u32; let mut mmcr1=0u32; let mut mmcr2=0u32;
    for class in (0..N_CLASSES).rev() { for k in 0..n_class[class] { let idx=event_index[class][k] as usize; let mut ev=*event.add(idx); let pmc;
        if class==4 { pmc=(ev as u32 >> PM_PMC_SH)&PM_PMC_MSK; if pmc_inuse & (1 << (pmc-1)) != 0 { return -1; } }
        else { let avail=(classmap[class] as u32)&!pmc_inuse; if avail==0 { return -1; } pmc=avail.trailing_zeros()+1; }
        pmc_inuse |= 1 << (pmc-1); let t=mpc7450_threshold_use(ev as u32); if t!=0 { mmcr0 |= (((ev as u32)>>PM_THRESH_SH)&PM_THRESH_MSK)<<16; if t==2 && (ev as u32)&PM_THRMULT_MSKS!=0 { mmcr2=0x80000000; } }
        ev=(ev as u32 & pmcsel_mask[(pmc-1) as usize]) as u64; ev <<= pmcsel_shift[(pmc-1) as usize]; if pmc<=2 { mmcr0 |= ev as u32; } else { mmcr1 |= ev as u32; } *hwc.add(idx)=pmc-1;
    }}
    if pmc_inuse&1!=0 { mmcr0|=MMCR0_PMC1CE; } if pmc_inuse&0x3e!=0 { mmcr0|=MMCR0_PMCnCE; }
    (*mmcr).mmcr0=mmcr0; (*mmcr).mmcr1=mmcr1; (*mmcr).mmcr2=mmcr2; (*mmcr).mmcra=mmcr2; 0
}

unsafe fn mpc7450_disable_pmc(pmc: u32, mmcr: *mut mmcr_regs) { if pmc<=1 { (*mmcr).mmcr0 &= !(pmcsel_mask[pmc as usize]<<pmcsel_shift[pmc as usize]); } else { (*mmcr).mmcr1 &= !(pmcsel_mask[pmc as usize]<<pmcsel_shift[pmc as usize]); } }

// Generalized event tables and PMU registration retain the kernel's externally supplied types and constants.
static mut mpc7450_generic_events: [i32; 5] = [1, 2, 0x217, 0x122, 0x41c];

/* Table of generalized cache-related events.  Zero means unsupported and -1 nonsensical. */
static mut mpc7450_cache_events: [[[u64; 2]; 3]; 7] = [
    [[0,0x225],[0,0x227],[0,0]],
    [[0x129,0x115],[-1i64 as u64,-1i64 as u64],[0x634,0]],
    [[0,0],[0,0],[0,0]],
    [[0,0x312],[-1i64 as u64,-1i64 as u64],[-1i64 as u64,-1i64 as u64]],
    [[0,0x223],[-1i64 as u64,-1i64 as u64],[-1i64 as u64,-1i64 as u64]],
    [[0x122,0x41c],[-1i64 as u64,-1i64 as u64],[-1i64 as u64,-1i64 as u64]],
    [[-1i64 as u64,-1i64 as u64],[-1i64 as u64,-1i64 as u64],[-1i64 as u64,-1i64 as u64]],
];

#[allow(non_upper_case_globals)]
static mut mpc7450_pmu: power_pmu = power_pmu {
    name: "MPC7450 family",
    n_counter: N_COUNTER as _, max_alternatives: MAX_ALT as _,
    add_fields: 0x00111555, test_adder: 0x00301000,
    compute_mmcr: Some(mpc7450_compute_mmcr), get_constraint: Some(mpc7450_get_constraint),
    get_alternatives: Some(mpc7450_get_alternatives), disable_pmc: Some(mpc7450_disable_pmc),
    n_generic: 5, generic_events: &mut mpc7450_generic_events,
    cache_events: &mut mpc7450_cache_events,
};

unsafe fn init_mpc7450_pmu() -> i32 {
    if !pvr_version_is(PVR_VER_7450) && !pvr_version_is(PVR_VER_7455) &&
       !pvr_version_is(PVR_VER_7447) && !pvr_version_is(PVR_VER_7447A) &&
       !pvr_version_is(PVR_VER_7448) { return -ENODEV; }
    register_power_pmu(&mut mpc7450_pmu)
}

early_initcall!(init_mpc7450_pmu);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
