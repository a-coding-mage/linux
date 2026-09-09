// SPDX-License-Identifier: GPL-2.0
/*
 * Precise Delay Loops for i386
 *
 * The __delay function must NOT be inlined as its execution time depends
 * wildly on alignment on many x86 processors.
 */

use core::cmp::min;

extern "C" {
    static mut loops_per_jiffy: usize;
    static mut cpu_tss_rw: core::ffi::c_void;
    static mut HZ: usize;
    fn preempt_disable();
    fn preempt_enable();
    fn smp_processor_id() -> i32;
    fn rdtsc_ordered() -> u64;
    fn rdtsc() -> u64;
    fn native_pause();
    fn __tpause(state: u32, edx: u32, eax: u32);
    fn __monitorx(addr: *mut core::ffi::c_void, ecx: u32, edx: u32);
    fn __mwaitx(ecx: u32, eax: u32, edx: u32);
    fn this_cpu_loops_per_jiffy() -> usize;
}

const TPAUSE_C02_STATE: u32 = 0;
const MWAITX_MAX_WAIT_CYCLES: u64 = 0xffff_ffff;
const MWAITX_DISABLE_CSTATES: u32 = 0x0f;
const MWAITX_ECX_TIMER_ENABLE: u32 = 1;

unsafe fn delay_loop(mut loops: u64) {
    let mut value = loops as usize;
    core::arch::asm!(
        "test {0}, {0}",
        "jz 3f",
        "jmp 1f",
        ".align 16",
        "1: jmp 2f",
        ".align 16",
        "2: dec {0}",
        "jnz 2b",
        "3: dec {0}",
        inout(reg) value,
        options(nostack)
    );
    loops = value as u64;
    let _ = loops;
}

unsafe fn delay_tsc(cycles: u64) {
    preempt_disable();
    let mut cpu = smp_processor_id();
    let mut bclock = rdtsc_ordered();
    loop {
        let now = rdtsc_ordered();
        if now.wrapping_sub(bclock) >= cycles { break; }
        preempt_enable();
        native_pause();
        preempt_disable();
        if cpu != smp_processor_id() {
            cycles = cycles.wrapping_sub(now.wrapping_sub(bclock));
            cpu = smp_processor_id();
            bclock = rdtsc_ordered();
        }
    }
    preempt_enable();
}

unsafe fn delay_halt_tpause(start: u64, cycles: u64) {
    let until = start.wrapping_add(cycles);
    __tpause(TPAUSE_C02_STATE, (until >> 32) as u32, until as u32);
}

unsafe fn delay_halt_mwaitx(_unused: u64, cycles: u64) {
    let delay = min(MWAITX_MAX_WAIT_CYCLES, cycles);
    __monitorx((&raw mut cpu_tss_rw).cast(), 0, 0);
    __mwaitx(MWAITX_DISABLE_CSTATES, delay as u32, MWAITX_ECX_TIMER_ENABLE);
}

unsafe fn delay_halt(mut cycles: u64) {
    if cycles == 0 { return; }
    let mut start = rdtsc_ordered();
    loop {
        if let Some(f) = DELAY_HALT_FN { f(start, cycles); }
        let end = rdtsc_ordered();
        let elapsed = end.wrapping_sub(start);
        if cycles <= elapsed { break; }
        cycles -= elapsed;
        start = end;
    }
}

type DelayFn = unsafe fn(u64);
type DelayHaltFn = unsafe fn(u64, u64);
static mut DELAY_FN: DelayFn = delay_loop;
static mut DELAY_HALT_FN: Option<DelayHaltFn> = None;

pub unsafe fn use_tsc_delay() { if core::ptr::fn_addr_eq(DELAY_FN, delay_loop) { DELAY_FN = delay_tsc; } }
pub unsafe fn use_tpause_delay() { DELAY_HALT_FN = Some(delay_halt_tpause); DELAY_FN = delay_halt; }
pub unsafe fn use_mwaitx_delay() { DELAY_HALT_FN = Some(delay_halt_mwaitx); DELAY_FN = delay_halt; }

pub unsafe fn delay_read_timer(timer_val: *mut usize) -> bool {
    if core::ptr::fn_addr_eq(DELAY_FN, delay_tsc) { *timer_val = rdtsc() as usize; return true; }
    false
}

pub unsafe fn __delay(loops: usize) { DELAY_FN(loops as u64); }

#[inline(never)]
pub unsafe fn __const_udelay(mut xloops: usize) {
    let lpj = { let local = this_cpu_loops_per_jiffy(); if local != 0 { local } else { loops_per_jiffy } };
    xloops = xloops.wrapping_mul(4);
    let product = (xloops as u128).wrapping_mul((lpj.wrapping_mul(HZ / 4)) as u128);
    __delay((product >> 32) as usize + 1);
}

pub unsafe fn __udelay(usecs: usize) { __const_udelay(usecs.wrapping_mul(0x0000_10c7)); }
pub unsafe fn __ndelay(nsecs: usize) { __const_udelay(nsecs.wrapping_mul(0x0000_0005)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
