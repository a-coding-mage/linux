// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Performance event support - PPC 8xx
 *
 * Copyright 2016 Christophe Leroy, CS Systemes d'Information
 */

// Linux and PPC dependencies supplied by other translation units.

const PERF_8XX_ID_CPU_CYCLES: i32 = 1;
const PERF_8XX_ID_HW_INSTRUCTIONS: i32 = 2;
const PERF_8XX_ID_ITLB_LOAD_MISS: i32 = 3;
const PERF_8XX_ID_DTLB_LOAD_MISS: i32 = 4;

const DTLB_LOAD_MISS: u64 = PERF_COUNT_HW_CACHE_DTLB
    | (PERF_COUNT_HW_CACHE_OP_READ << 8)
    | (PERF_COUNT_HW_CACHE_RESULT_MISS << 16);
const ITLB_LOAD_MISS: u64 = PERF_COUNT_HW_CACHE_ITLB
    | (PERF_COUNT_HW_CACHE_OP_READ << 8)
    | (PERF_COUNT_HW_CACHE_RESULT_MISS << 16);

extern "C" {
    static mut itlb_miss_counter: ::core::ffi::c_ulong;
    static mut dtlb_miss_counter: ::core::ffi::c_ulong;
    static mut instruction_counter: atomic_t;
    static mut patch__itlbmiss_perf: u8;
    static mut patch__itlbmiss_exit_1: u8;
    static mut patch__dtlbmiss_perf: u8;
    static mut patch__dtlbmiss_exit_1: u8;

    fn atomic_read(v: *const atomic_t) -> i32;
    fn atomic_inc_return(v: *mut atomic_t) -> i32;
    fn atomic_dec_return(v: *mut atomic_t) -> i32;
    fn mfspr(spr: i32) -> ::core::ffi::c_ulong;
    fn mtspr(spr: i32, value: ::core::ffi::c_ulong);
    fn get_tb() -> i64;
    fn patch_site_addr(site: *mut u8) -> ::core::ffi::c_ulong;
    fn patch_branch_site(site: *mut u8, target: ::core::ffi::c_ulong, ctx: i32);
    fn patch_instruction_site(site: *mut u8, insn: ppc_inst_t);
    fn ppc_inst(raw: u32) -> ppc_inst_t;
    fn PPC_RAW_MFSPR(rt: i32, spr: i32) -> u32;
    fn perf_pmu_register(pmu: *mut pmu, name: *const u8, ty: i32) -> i32;
    fn local64_set(ptr: *mut local64_t, value: i64);
    fn local64_read(ptr: *const local64_t) -> i64;
    fn local64_cmpxchg(ptr: *mut local64_t, old: i64, new: i64) -> i64;
    fn local64_add(value: i64, ptr: *mut local64_t);
}

static mut insn_ctr_ref: atomic_t = atomic_t { counter: 0 };
static mut itlb_miss_ref: atomic_t = atomic_t { counter: 0 };
static mut dtlb_miss_ref: atomic_t = atomic_t { counter: 0 };

unsafe fn get_insn_ctr() -> i64 {
    let mut ctr: i32;
    let mut counta: ::core::ffi::c_ulong;
    loop {
        ctr = atomic_read(&instruction_counter);
        counta = mfspr(SPRN_COUNTA);
        if ctr == atomic_read(&instruction_counter) {
            break;
        }
    }
    ((ctr as i64) << 16) | ((counta >> 16) as i64)
}

unsafe fn event_type(event: *mut perf_event) -> i32 {
    match (*event).attr.type_ {
        PERF_TYPE_HARDWARE => {
            if (*event).attr.config == PERF_COUNT_HW_CPU_CYCLES { return PERF_8XX_ID_CPU_CYCLES; }
            if (*event).attr.config == PERF_COUNT_HW_INSTRUCTIONS { return PERF_8XX_ID_HW_INSTRUCTIONS; }
        }
        PERF_TYPE_HW_CACHE => {
            if (*event).attr.config == ITLB_LOAD_MISS { return PERF_8XX_ID_ITLB_LOAD_MISS; }
            if (*event).attr.config == DTLB_LOAD_MISS { return PERF_8XX_ID_DTLB_LOAD_MISS; }
        }
        PERF_TYPE_RAW => {}
        _ => return -ENOENT,
    }
    -EOPNOTSUPP
}

unsafe fn mpc8xx_pmu_event_init(event: *mut perf_event) -> i32 {
    let ty = event_type(event);
    if ty < 0 { return ty; }
    0
}

unsafe fn mpc8xx_pmu_add(event: *mut perf_event, _flags: i32) -> i32 {
    let ty = event_type(event);
    let mut val: i64 = 0;
    if ty < 0 { return ty; }
    match ty {
        PERF_8XX_ID_CPU_CYCLES => val = get_tb(),
        PERF_8XX_ID_HW_INSTRUCTIONS => {
            if atomic_inc_return(&mut insn_ctr_ref) == 1 { mtspr(SPRN_ICTRL, 0xc0080007); }
            val = get_insn_ctr();
        }
        PERF_8XX_ID_ITLB_LOAD_MISS => {
            if atomic_inc_return(&mut itlb_miss_ref) == 1 {
                let target = patch_site_addr(&mut patch__itlbmiss_perf);
                patch_branch_site(&mut patch__itlbmiss_exit_1, target, 0);
            }
            val = itlb_miss_counter as i64;
        }
        PERF_8XX_ID_DTLB_LOAD_MISS => {
            if atomic_inc_return(&mut dtlb_miss_ref) == 1 {
                let target = patch_site_addr(&mut patch__dtlbmiss_perf);
                patch_branch_site(&mut patch__dtlbmiss_exit_1, target, 0);
            }
            val = dtlb_miss_counter as i64;
        }
        _ => {}
    }
    local64_set(&mut (*event).hw.prev_count, val);
    0
}

unsafe fn mpc8xx_pmu_read(event: *mut perf_event) {
    let ty = event_type(event);
    let (mut prev, mut val, mut delta): (i64, i64, i64);
    if ty < 0 { return; }
    loop {
        prev = local64_read(&(*event).hw.prev_count);
        val = 0; delta = 0;
        match ty {
            PERF_8XX_ID_CPU_CYCLES => { val = get_tb(); delta = 16i64.wrapping_mul(val.wrapping_sub(prev)); }
            PERF_8XX_ID_HW_INSTRUCTIONS => { val = get_insn_ctr(); delta = prev.wrapping_sub(val); if delta < 0 { delta = delta.wrapping_add(0x1000000000000); } }
            PERF_8XX_ID_ITLB_LOAD_MISS => { val = itlb_miss_counter as i64; delta = (val as i32).wrapping_sub(prev as i32) as i64; }
            PERF_8XX_ID_DTLB_LOAD_MISS => { val = dtlb_miss_counter as i64; delta = (val as i32).wrapping_sub(prev as i32) as i64; }
            _ => {}
        }
        if local64_cmpxchg(&mut (*event).hw.prev_count, prev, val) == prev { break; }
    }
    local64_add(delta, &mut (*event).count);
}

unsafe fn mpc8xx_pmu_del(event: *mut perf_event, _flags: i32) {
    let insn = ppc_inst(PPC_RAW_MFSPR(10, SPRN_SPRG_SCRATCH2));
    mpc8xx_pmu_read(event);
    match event_type(event) {
        PERF_8XX_ID_CPU_CYCLES => {}
        PERF_8XX_ID_HW_INSTRUCTIONS => if atomic_dec_return(&mut insn_ctr_ref) == 0 { mtspr(SPRN_ICTRL, 7); },
        PERF_8XX_ID_ITLB_LOAD_MISS => if atomic_dec_return(&mut itlb_miss_ref) == 0 { patch_instruction_site(&mut patch__itlbmiss_exit_1, insn); },
        PERF_8XX_ID_DTLB_LOAD_MISS => if atomic_dec_return(&mut dtlb_miss_ref) == 0 { patch_instruction_site(&mut patch__dtlbmiss_exit_1, insn); },
        _ => {}
    }
}

static mut mpc8xx_pmu: pmu = pmu {
    event_init: Some(mpc8xx_pmu_event_init),
    add: Some(mpc8xx_pmu_add),
    del: Some(mpc8xx_pmu_del),
    read: Some(mpc8xx_pmu_read),
    capabilities: PERF_PMU_CAP_NO_INTERRUPT | PERF_PMU_CAP_NO_NMI,
};

unsafe fn init_mpc8xx_pmu() -> i32 {
    mtspr(SPRN_ICTRL, 7);
    mtspr(SPRN_CMPA, 0);
    mtspr(SPRN_COUNTA, 0xffff);
    perf_pmu_register(&mut mpc8xx_pmu, b"cpu\0".as_ptr(), PERF_TYPE_RAW)
}

// early_initcall(init_mpc8xx_pmu);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
