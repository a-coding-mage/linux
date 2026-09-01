// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

// C dependencies: sched.h, sys/wait.h, setjmp.h, signal.h, stdio.h, stdlib.h,
// string.h, sys/ioctl.h, trace.h, ebb.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

type u64 = u64;
type u32 = u32;
type uint32_t = u32;
type uint64_t = u64;

#[repr(C)]
pub struct event_attr {
    pub config: u64,
    pub exclusive: u64,
    pub pinned: u64,
    pub exclude_kernel: u64,
    pub exclude_hv: u64,
    pub exclude_idle: u64,
}

#[repr(C)]
pub struct event {
    pub fd: c_int,
    pub attr: event_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pipe {
    pub read_fd: c_int,
    pub write_fd: c_int,
}

#[repr(C)]
pub struct ebb_stats {
    pub ebb_count: c_int,
    pub spurious: c_int,
    pub negative: c_int,
    pub no_overflow: c_int,
    pub pmc_count: [u64; 6],
}

#[repr(C)]
pub struct ebb_state {
    pub stats: ebb_stats,
    pub pmc_enable: [c_int; 6],
    pub trace: *mut c_void,
}

#[repr(C)]
pub struct opd {
    pub entry: u64,
    pub toc: u64,
}

#[repr(C)]
pub struct sigaction {
    pub sa_handler: Option<unsafe extern "C" fn(c_int)>,
}

#[repr(C)]
pub struct jmp_buf {
    _private: [c_ulong; 64],
}

unsafe extern "C" {
    static SPRN_MMCR0: c_int;
    static SPRN_BESCRR: c_int;
    static SPRN_BESCRS: c_int;
    static SPRN_BESCR: c_int;
    static SPRN_EBBHR: c_int;
    static SPRN_MMCR2: c_int;
    static SPRN_PMC1: c_int;
    static SPRN_PMC2: c_int;
    static SPRN_PMC3: c_int;
    static SPRN_PMC4: c_int;
    static SPRN_PMC5: c_int;
    static SPRN_PMC6: c_int;
    static SPRN_SIAR: c_int;
    static SPRN_MMCRA: c_int;

    static MMCR0_PMAE: u64;
    static MMCR0_PMAO: u64;
    static MMCR0_FC: u64;
    static BESCR_PMEO: u64;
    static BESCR_PME: u64;
    static PERF_EVENT_IOC_ENABLE: c_ulong;
    static COUNTER_OVERFLOW: u64;
    static SIGILL: c_int;
    static SIGTERM: c_int;

    fn mfspr(spr: c_int) -> u64;
    fn mtspr(spr: c_int, val: u64);
    fn mb();
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn abort() -> !;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn setjmp(env: *mut jmp_buf) -> c_int;
    fn longjmp(env: *mut jmp_buf, val: c_int) -> !;

    fn trace_log_counter(trace: *mut c_void, value: c_int);
    fn trace_log_reg(trace: *mut c_void, reg: c_int, value: u64);
    fn trace_buffer_print(trace: *mut c_void);
    fn trace_buffer_allocate(size: usize) -> *mut c_void;

    fn pmc_sample_period(sample_period: uint32_t) -> uint32_t;
    fn event_read(e: *mut event) -> c_int;
    fn event_init_named(e: *mut event, config: u64, name: *const c_char);
    fn event_open(e: *mut event) -> c_int;
    fn event_enable(e: *mut event) -> c_int;
    fn event_close(e: *mut event);
    fn wait_for_parent(read_pipe: pipe) -> c_int;
    fn notify_parent(write_pipe: pipe) -> c_int;
    fn notify_parent_of_error(write_pipe: pipe);
    fn ebb_enable_pmc_counting(pmc: c_int);
    fn core_busy_loop() -> c_int;
    fn have_hwcap2(feature: c_ulong) -> bool;
    fn ebb_handler();
}

const fn PMC_INDEX(pmc: c_int) -> usize {
    (pmc - 1) as usize
}

unsafe fn FAIL_IF(expr: c_int) {
    if expr != 0 {
        abort();
    }
}

#[unsafe(no_mangle)]
pub static mut ebb_user_func: Option<unsafe extern "C" fn()> = None;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ebb_hook() {
    if let Some(func) = unsafe { ebb_user_func } {
        unsafe { func() };
    }
}

#[unsafe(no_mangle)]
pub static mut ebb_state: ebb_state = ebb_state {
    stats: ebb_stats {
        ebb_count: 0,
        spurious: 0,
        negative: 0,
        no_overflow: 0,
        pmc_count: [0; 6],
    },
    pmc_enable: [0; 6],
    trace: core::ptr::null_mut(),
};

#[unsafe(no_mangle)]
pub static mut sample_period: u64 = 0x40000000_u64;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_ebb_with_clear_mask(mmcr0_clear_mask: c_ulong) {
    let val: u64;

    /* 2) clear MMCR0[PMAO] - docs say BESCR[PMEO] should do this */
    /* 3) set MMCR0[PMAE]   - docs say BESCR[PME] should do this */
    val = unsafe { mfspr(SPRN_MMCR0) };
    unsafe { mtspr(SPRN_MMCR0, (val & !(mmcr0_clear_mask as u64)) | MMCR0_PMAE) };

    /* 4) clear BESCR[PMEO] */
    unsafe { mtspr(SPRN_BESCRR, BESCR_PMEO) };

    /* 5) set BESCR[PME] */
    unsafe { mtspr(SPRN_BESCRS, BESCR_PME) };

    /* 6) rfebb 1 - done in our caller */
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_ebb() {
    unsafe { reset_ebb_with_clear_mask((MMCR0_PMAO | MMCR0_FC) as c_ulong) };
}

/* Called outside of the EBB handler to check MMCR0 is sane */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ebb_check_mmcr0() -> c_int {
    let val: u64;

    val = unsafe { mfspr(SPRN_MMCR0) };
    if (val & (MMCR0_FC | MMCR0_PMAO)) == MMCR0_FC {
        /* It's OK if we see FC & PMAO, but not FC by itself */
        unsafe { printf(c"Outside of loop, only FC set 0x%llx\n".as_ptr(), val) };
        return 1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ebb_check_count(pmc: c_int, sample_period: u64, fudge: c_int) -> bool {
    let count: u64;
    let upper: u64;
    let lower: u64;

    count = unsafe { ebb_state.stats.pmc_count[PMC_INDEX(pmc)] };

    lower = unsafe { (ebb_state.stats.ebb_count as u64).wrapping_mul(sample_period.wrapping_sub(fudge as u64)) };

    if count < lower {
        unsafe {
            printf(
                c"PMC%d count (0x%llx) below lower limit 0x%llx (-0x%llx)\n".as_ptr(),
                pmc,
                count,
                lower,
                lower - count,
            )
        };
        return false;
    }

    upper = unsafe { (ebb_state.stats.ebb_count as u64).wrapping_mul(sample_period.wrapping_add(fudge as u64)) };

    if count > upper {
        unsafe {
            printf(
                c"PMC%d count (0x%llx) above upper limit 0x%llx (+0x%llx)\n".as_ptr(),
                pmc,
                count,
                upper,
                count - upper,
            )
        };
        return false;
    }

    unsafe {
        printf(
            c"PMC%d count (0x%llx) is between 0x%llx and 0x%llx delta +0x%llx/-0x%llx\n".as_ptr(),
            pmc,
            count,
            lower,
            upper,
            count - lower,
            upper - count,
        )
    };

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn standard_ebb_callee() {
    let mut found: c_int;
    let mut i: c_int;
    let mut val: u64;

    val = unsafe { mfspr(SPRN_BESCR) };
    if (val & BESCR_PMEO) == 0 {
        unsafe { ebb_state.stats.spurious += 1 };
        unsafe { reset_ebb() };
        return;
    }

    unsafe { ebb_state.stats.ebb_count += 1 };
    unsafe { trace_log_counter(ebb_state.trace, ebb_state.stats.ebb_count) };

    val = unsafe { mfspr(SPRN_MMCR0) };
    unsafe { trace_log_reg(ebb_state.trace, SPRN_MMCR0, val) };

    found = 0;
    i = 1;
    while i <= 6 {
        if unsafe { ebb_state.pmc_enable[PMC_INDEX(i)] } != 0 {
            found += unsafe { count_pmc(i, sample_period as uint32_t) };
        }
        i += 1;
    }

    if found == 0 {
        unsafe { ebb_state.stats.no_overflow += 1 };
    }

    unsafe { reset_ebb() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_ebb_handler(callee: Option<unsafe extern "C" fn()>) {
    let entry: u64;

    // C conditional preserved:
    // #if defined(_CALL_ELF) && _CALL_ELF == 2
    //     entry = (u64)ebb_handler;
    // #else
    //     opd = (struct opd *)ebb_handler;
    //     entry = opd->entry;
    // #endif
    #[cfg(all(target_arch = "powerpc64", target_env = "musl"))]
    {
        entry = ebb_handler as usize as u64;
    }
    #[cfg(not(all(target_arch = "powerpc64", target_env = "musl")))]
    {
        let opd: *mut opd;
        opd = ebb_handler as *mut opd;
        entry = unsafe { (*opd).entry };
    }

    unsafe { printf(c"EBB Handler is at %#llx\n".as_ptr(), entry) };

    unsafe { ebb_user_func = callee };

    /* Ensure ebb_user_func is set before we set the handler */
    unsafe { mb() };
    unsafe { mtspr(SPRN_EBBHR, entry) };

    /* Make sure the handler is set before we return */
    unsafe { mb() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clear_ebb_stats() {
    unsafe {
        memset(
            core::ptr::addr_of_mut!(ebb_state.stats) as *mut c_void,
            0,
            core::mem::size_of_val(&raw const ebb_state.stats),
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_summary_ebb_state() {
    unsafe {
        printf(
            c"ebb_state:\n  ebb_count    = %d\n  spurious     = %d\n  negative     = %d\n  no_overflow  = %d\n  pmc[1] count = 0x%llx\n  pmc[2] count = 0x%llx\n  pmc[3] count = 0x%llx\n  pmc[4] count = 0x%llx\n  pmc[5] count = 0x%llx\n  pmc[6] count = 0x%llx\n".as_ptr(),
            ebb_state.stats.ebb_count,
            ebb_state.stats.spurious,
            ebb_state.stats.negative,
            ebb_state.stats.no_overflow,
            ebb_state.stats.pmc_count[0],
            ebb_state.stats.pmc_count[1],
            ebb_state.stats.pmc_count[2],
            ebb_state.stats.pmc_count[3],
            ebb_state.stats.pmc_count[4],
            ebb_state.stats.pmc_count[5],
        )
    };
}

static mut DECODE_MMCR0_BUF: [c_char; 16] = [0; 16];

unsafe fn decode_mmcr0(value: u32) -> *mut c_char {
    unsafe { DECODE_MMCR0_BUF[0] = 0 };

    if (value & (1 << 31)) != 0 {
        unsafe { strcat(core::ptr::addr_of_mut!(DECODE_MMCR0_BUF).cast::<c_char>(), c"FC ".as_ptr()) };
    }
    if (value & (1 << 26)) != 0 {
        unsafe { strcat(core::ptr::addr_of_mut!(DECODE_MMCR0_BUF).cast::<c_char>(), c"PMAE ".as_ptr()) };
    }
    if (value & (1 << 7)) != 0 {
        unsafe { strcat(core::ptr::addr_of_mut!(DECODE_MMCR0_BUF).cast::<c_char>(), c"PMAO ".as_ptr()) };
    }

    core::ptr::addr_of_mut!(DECODE_MMCR0_BUF).cast::<c_char>()
}

static mut DECODE_BESCR_BUF: [c_char; 16] = [0; 16];

unsafe fn decode_bescr(value: u64) -> *mut c_char {
    unsafe { DECODE_BESCR_BUF[0] = 0 };

    if (value & (1_u64 << 63)) != 0 {
        unsafe { strcat(core::ptr::addr_of_mut!(DECODE_BESCR_BUF).cast::<c_char>(), c"GE ".as_ptr()) };
    }
    if (value & (1_u64 << 32)) != 0 {
        unsafe { strcat(core::ptr::addr_of_mut!(DECODE_BESCR_BUF).cast::<c_char>(), c"PMAE ".as_ptr()) };
    }
    if (value & 1) != 0 {
        unsafe { strcat(core::ptr::addr_of_mut!(DECODE_BESCR_BUF).cast::<c_char>(), c"PMAO ".as_ptr()) };
    }

    core::ptr::addr_of_mut!(DECODE_BESCR_BUF).cast::<c_char>()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_ebb_hw_state() {
    let bescr: u64;
    let mmcr0: u32;

    mmcr0 = unsafe { mfspr(SPRN_MMCR0) as u32 };
    bescr = unsafe { mfspr(SPRN_BESCR) };

    unsafe {
        printf(
            c"HW state:\nMMCR0 0x%016x %s\nMMCR2 0x%016lx\nEBBHR 0x%016lx\nBESCR 0x%016llx %s\nPMC1  0x%016lx\nPMC2  0x%016lx\nPMC3  0x%016lx\nPMC4  0x%016lx\nPMC5  0x%016lx\nPMC6  0x%016lx\nSIAR  0x%016lx\n".as_ptr(),
            mmcr0,
            decode_mmcr0(mmcr0),
            mfspr(SPRN_MMCR2) as c_ulong,
            mfspr(SPRN_EBBHR) as c_ulong,
            bescr,
            decode_bescr(bescr),
            mfspr(SPRN_PMC1) as c_ulong,
            mfspr(SPRN_PMC2) as c_ulong,
            mfspr(SPRN_PMC3) as c_ulong,
            mfspr(SPRN_PMC4) as c_ulong,
            mfspr(SPRN_PMC5) as c_ulong,
            mfspr(SPRN_PMC6) as c_ulong,
            mfspr(SPRN_SIAR) as c_ulong,
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_ebb_state() {
    unsafe { dump_summary_ebb_state() };

    unsafe { dump_ebb_hw_state() };

    unsafe { trace_buffer_print(ebb_state.trace) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn count_pmc(pmc: c_int, sample_period: uint32_t) -> c_int {
    let start_value: uint32_t;
    let val: u64;

    /* 0) Read PMC */
    start_value = unsafe { pmc_sample_period(sample_period) };

    val = unsafe { read_pmc(pmc) };
    if val < start_value as u64 {
        unsafe { ebb_state.stats.negative += 1 };
    } else {
        unsafe {
            ebb_state.stats.pmc_count[PMC_INDEX(pmc)] =
                ebb_state.stats.pmc_count[PMC_INDEX(pmc)].wrapping_add(val - start_value as u64);
        }
    }

    unsafe { trace_log_reg(ebb_state.trace, SPRN_PMC1 + pmc - 1, val) };

    /* 1) Reset PMC */
    unsafe { write_pmc(pmc, start_value as u64) };

    /* Report if we overflowed */
    (val >= unsafe { COUNTER_OVERFLOW }) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ebb_event_enable(e: *mut event) -> c_int {
    let mut rc: c_int;

    /* Ensure any SPR writes are ordered vs us */
    unsafe { mb() };

    rc = unsafe { ioctl((*e).fd, PERF_EVENT_IOC_ENABLE) };
    if rc != 0 {
        return rc;
    }

    rc = unsafe { event_read(e) };

    /* Ditto */
    unsafe { mb() };

    rc
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ebb_freeze_pmcs() {
    unsafe { mtspr(SPRN_MMCR0, mfspr(SPRN_MMCR0) | MMCR0_FC) };
    unsafe { mb() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ebb_unfreeze_pmcs() {
    /* Unfreeze counters */
    unsafe { mtspr(SPRN_MMCR0, mfspr(SPRN_MMCR0) & !MMCR0_FC) };
    unsafe { mb() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ebb_global_enable() {
    /* Enable EBBs globally and PMU EBBs */
    unsafe { mtspr(SPRN_BESCR, 0x8000000100000000_u64) };
    unsafe { mb() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ebb_global_disable() {
    /* Disable EBBs & freeze counters, events are still scheduled */
    unsafe { mtspr(SPRN_BESCRR, BESCR_PME) };
    unsafe { mb() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ebb_is_supported() -> bool {
    // C conditional preserved:
    // #ifdef PPC_FEATURE2_EBB
    //     return have_hwcap2(PPC_FEATURE2_EBB);
    // #else
    //     return false;
    // #endif
    #[cfg(any())]
    {
        unsafe extern "C" {
            static PPC_FEATURE2_EBB: c_ulong;
        }
        return unsafe { have_hwcap2(PPC_FEATURE2_EBB) };
    }
    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_ebb_init(e: *mut event) {
    unsafe { (*e).attr.config |= 1_u64 << 63 };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_bhrb_init(e: *mut event, ifm: c_uint) {
    unsafe { (*e).attr.config |= (1_u64 << 62) | ((ifm as u64) << 60) };
}

type c_uint = u32;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_leader_ebb_init(e: *mut event) {
    unsafe { event_ebb_init(e) };

    unsafe {
        (*e).attr.exclusive = 1;
        (*e).attr.pinned = 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ebb_child(read_pipe: pipe, write_pipe: pipe) -> c_int {
    let mut event: event = unsafe { core::mem::zeroed() };
    let mut val: uint64_t;

    unsafe { FAIL_IF(wait_for_parent(read_pipe)) };

    unsafe { event_init_named(&mut event, 0x1001e, c"cycles".as_ptr()) };
    unsafe { event_leader_ebb_init(&mut event) };

    event.attr.exclude_kernel = 1;
    event.attr.exclude_hv = 1;
    event.attr.exclude_idle = 1;

    unsafe { FAIL_IF(event_open(&mut event)) };

    unsafe { ebb_enable_pmc_counting(1) };
    unsafe { setup_ebb_handler(Some(standard_ebb_callee)) };
    unsafe { ebb_global_enable() };

    unsafe { FAIL_IF(event_enable(&mut event)) };

    if unsafe { event_read(&mut event) } != 0 {
        /*
         * Some tests expect to fail here, so don't report an error on
         * this line, and return a distinguisable error code. Tell the
         * parent an error happened.
         */
        unsafe { notify_parent_of_error(write_pipe) };
        return 2;
    }

    unsafe { mtspr(SPRN_PMC1, pmc_sample_period(sample_period as uint32_t) as u64) };

    unsafe { FAIL_IF(notify_parent(write_pipe)) };
    unsafe { FAIL_IF(wait_for_parent(read_pipe)) };
    unsafe { FAIL_IF(notify_parent(write_pipe)) };

    while unsafe { ebb_state.stats.ebb_count } < 20 {
        unsafe { FAIL_IF(core_busy_loop()) };

        /* To try and hit SIGILL case */
        val = unsafe { mfspr(SPRN_MMCRA) };
        val |= unsafe { mfspr(SPRN_MMCR2) };
        val |= unsafe { mfspr(SPRN_MMCR0) };
        let _ = val;
    }

    unsafe { ebb_global_disable() };
    unsafe { ebb_freeze_pmcs() };

    unsafe { dump_ebb_state() };

    unsafe { event_close(&mut event) };

    unsafe { FAIL_IF((ebb_state.stats.ebb_count == 0) as c_int) };

    0
}

static mut setjmp_env: jmp_buf = jmp_buf { _private: [0; 64] };

unsafe extern "C" fn sigill_handler(_signal: c_int) {
    unsafe { printf(c"Took sigill\n".as_ptr()) };
    unsafe { longjmp(core::ptr::addr_of_mut!(setjmp_env), 1) };
}

static mut sigill_action: sigaction = sigaction {
    sa_handler: Some(sigill_handler),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn catch_sigill(func: Option<unsafe extern "C" fn()>) -> c_int {
    if unsafe { sigaction(SIGILL, core::ptr::addr_of!(sigill_action), core::ptr::null_mut()) } != 0 {
        unsafe { perror(c"sigaction".as_ptr()) };
        return 1;
    }

    if unsafe { setjmp(core::ptr::addr_of_mut!(setjmp_env)) } == 0 {
        if let Some(func) = func {
            unsafe { func() };
        }
        return 1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_pmc1() {
    unsafe { mtspr(SPRN_PMC1, 0) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_pmc(pmc: c_int, value: u64) {
    match pmc {
        1 => unsafe { mtspr(SPRN_PMC1, value) },
        2 => unsafe { mtspr(SPRN_PMC2, value) },
        3 => unsafe { mtspr(SPRN_PMC3, value) },
        4 => unsafe { mtspr(SPRN_PMC4, value) },
        5 => unsafe { mtspr(SPRN_PMC5, value) },
        6 => unsafe { mtspr(SPRN_PMC6, value) },
        _ => {}
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_pmc(pmc: c_int) -> u64 {
    match pmc {
        1 => unsafe { mfspr(SPRN_PMC1) },
        2 => unsafe { mfspr(SPRN_PMC2) },
        3 => unsafe { mfspr(SPRN_PMC3) },
        4 => unsafe { mfspr(SPRN_PMC4) },
        5 => unsafe { mfspr(SPRN_PMC5) },
        6 => unsafe { mfspr(SPRN_PMC6) },
        _ => 0,
    }
}

unsafe extern "C" fn term_handler(_signal: c_int) {
    unsafe { dump_summary_ebb_state() };
    unsafe { dump_ebb_hw_state() };
    unsafe { abort() };
}

#[unsafe(no_mangle)]
pub static mut term_action: sigaction = sigaction {
    sa_handler: Some(term_handler),
};

#[used]
#[cfg_attr(any(target_os = "linux", target_os = "android"), unsafe(link_section = ".init_array"))]
static EBB_INIT_ARRAY: unsafe extern "C" fn() = ebb_init;

unsafe extern "C" fn ebb_init() {
    unsafe { clear_ebb_stats() };

    if unsafe { sigaction(SIGTERM, core::ptr::addr_of!(term_action), core::ptr::null_mut()) } != 0 {
        unsafe { perror(c"sigaction".as_ptr()) };
    }

    unsafe { ebb_state.trace = trace_buffer_allocate(1 * 1024 * 1024) };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
