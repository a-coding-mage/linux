// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_void};

type u64 = u64;
type u32 = u32;
type size_t = usize;

const EINVAL: c_int = 22;
const EOPNOTSUPP: c_int = 95;
const PERF_RECORD_TIME_CONV: u32 = 80;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub struct perf_event_mmap_page {
    pub lock: u32,
    pub index: u32,
    pub offset: i64,
    pub time_enabled: u64,
    pub time_running: u64,
    pub capabilities: u64,
    pub pmc_width: u16,
    pub time_shift: u16,
    pub time_mult: u32,
    pub time_offset: u64,
    pub time_zero: u64,
    pub size: u32,
    pub __reserved_1: u32,
    pub time_cycles: u64,
    pub time_mask: u64,
    pub __reserved: [u8; 116 * 8],
    pub data_head: u64,
    pub data_tail: u64,
    pub data_offset: u64,
    pub data_size: u64,
    pub aux_head: u64,
    pub aux_tail: u64,
    pub aux_offset: u64,
    pub aux_size: u64,
}

impl perf_event_mmap_page {
    unsafe fn cap_user_time_zero(&self) -> u16 {
        ((self.capabilities >> 3) & 1) as u16
    }

    unsafe fn cap_user_time_short(&self) -> u16 {
        ((self.capabilities >> 6) & 1) as u16
    }
}

#[repr(C)]
pub struct perf_tsc_conversion {
    pub time_mult: u32,
    pub time_shift: u16,
    pub time_zero: u64,
    pub time_cycles: u64,
    pub time_mask: u64,
    pub cap_user_time_zero: u16,
    pub cap_user_time_short: u16,
}

#[repr(C)]
pub struct perf_tool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_time_conv {
    pub header: perf_event_header,
    pub time_shift: u64,
    pub time_mult: u64,
    pub time_zero: u64,
    pub time_cycles: u64,
    pub time_mask: u64,
    pub cap_user_time_zero: i32,
    pub cap_user_time_short: i32,
}

#[repr(C)]
pub union perf_event {
    pub time_conv: perf_record_time_conv,
}

type perf_event__handler_t = unsafe extern "C" fn(
    *const perf_tool,
    *mut perf_event,
    *mut c_void,
    *mut machine,
) -> c_int;

unsafe extern "C" {
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
}

#[inline]
unsafe fn rmb() {
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
}

unsafe fn event_contains(_tc: perf_record_time_conv, _field: EventContainsField) -> bool {
    true
}

enum EventContainsField {
    cap_user_time_short,
}

pub unsafe extern "C" fn perf_time_to_tsc(ns: u64, tc: *mut perf_tsc_conversion) -> u64 {
    let t: u64;
    let quot: u64;
    let rem: u64;

    t = ns.wrapping_sub((*tc).time_zero);
    quot = t / (*tc).time_mult as u64;
    rem = t % (*tc).time_mult as u64;
    return (quot << (*tc).time_shift)
        + ((rem << (*tc).time_shift) / (*tc).time_mult as u64);
}

pub unsafe extern "C" fn tsc_to_perf_time(mut cyc: u64, tc: *mut perf_tsc_conversion) -> u64 {
    let quot: u64;
    let rem: u64;

    if (*tc).cap_user_time_short != 0 {
        cyc = (*tc).time_cycles
            + ((cyc.wrapping_sub((*tc).time_cycles)) & (*tc).time_mask);
    }

    quot = cyc >> (*tc).time_shift;
    rem = cyc & (((1 as u64) << (*tc).time_shift) - 1);
    return (*tc).time_zero
        + quot.wrapping_mul((*tc).time_mult as u64)
        + ((rem.wrapping_mul((*tc).time_mult as u64)) >> (*tc).time_shift);
}

pub unsafe extern "C" fn perf_read_tsc_conversion(
    pc: *const perf_event_mmap_page,
    tc: *mut perf_tsc_conversion,
) -> c_int {
    let mut seq: u32;
    let mut i: c_int = 0;

    loop {
        seq = (*pc).lock;
        rmb();
        (*tc).time_mult = (*pc).time_mult;
        (*tc).time_shift = (*pc).time_shift;
        (*tc).time_zero = (*pc).time_zero;
        (*tc).time_cycles = (*pc).time_cycles;
        (*tc).time_mask = (*pc).time_mask;
        (*tc).cap_user_time_zero = (*pc).cap_user_time_zero();
        (*tc).cap_user_time_short = (*pc).cap_user_time_short();
        rmb();
        if (*pc).lock == seq && (seq & 1) == 0 {
            break;
        }
        i += 1;
        if i > 10000 {
            pr_debug(c"failed to get perf_event_mmap_page lock\n".as_ptr());
            return -EINVAL;
        }
    }

    if (*tc).cap_user_time_zero == 0 {
        return -EOPNOTSUPP;
    }

    return 0;
}

pub unsafe extern "C" fn perf_event__synth_time_conv(
    pc: *const perf_event_mmap_page,
    tool: *const perf_tool,
    process: perf_event__handler_t,
    machine: *mut machine,
) -> c_int {
    let mut event = perf_event {
        time_conv: perf_record_time_conv {
            header: perf_event_header {
                type_: PERF_RECORD_TIME_CONV,
                misc: 0,
                size: core::mem::size_of::<perf_record_time_conv>() as u16,
            },
            time_shift: 0,
            time_mult: 0,
            time_zero: 0,
            time_cycles: 0,
            time_mask: 0,
            cap_user_time_zero: 0,
            cap_user_time_short: 0,
        },
    };
    let mut tc = perf_tsc_conversion {
        time_mult: 0,
        time_shift: 0,
        time_zero: 0,
        time_cycles: 0,
        time_mask: 0,
        cap_user_time_zero: 0,
        cap_user_time_short: 0,
    };
    let err: c_int;

    if pc.is_null() {
        return 0;
    }
    err = perf_read_tsc_conversion(pc, &mut tc);
    if err == -EOPNOTSUPP {
        return 0;
    }
    if err != 0 {
        return err;
    }

    pr_debug2(c"Synthesizing TSC conversion information\n".as_ptr());

    event.time_conv.time_mult = tc.time_mult as u64;
    event.time_conv.time_shift = tc.time_shift as u64;
    event.time_conv.time_zero = tc.time_zero;
    event.time_conv.time_cycles = tc.time_cycles;
    event.time_conv.time_mask = tc.time_mask;
    event.time_conv.cap_user_time_zero = tc.cap_user_time_zero as i32;
    event.time_conv.cap_user_time_short = tc.cap_user_time_short as i32;

    return process(tool, &mut event, core::ptr::null_mut(), machine);
}

pub unsafe extern "C" fn rdtsc() -> u64 {
    return 0;
}

pub unsafe extern "C" fn perf_event__fprintf_time_conv(
    event: *mut perf_event,
    fp: *mut FILE,
) -> size_t {
    let tc: *mut perf_record_time_conv = event as *mut perf_record_time_conv;
    let mut ret: size_t;

    ret = fprintf(
        fp,
        c"\n... Time Shift      %llu\n".as_ptr(),
        (*tc).time_shift,
    ) as size_t;
    ret += fprintf(
        fp,
        c"... Time Multiplier %llu\n".as_ptr(),
        (*tc).time_mult,
    ) as size_t;
    ret += fprintf(fp, c"... Time Zero       %llu\n".as_ptr(), (*tc).time_zero) as size_t;

    /*
     * The event TIME_CONV was extended for the fields from "time_cycles"
     * when supported cap_user_time_short, for backward compatibility,
     * prints the extended fields only if they are contained in the event.
     */
    if event_contains(*tc, EventContainsField::cap_user_time_short) {
        ret += fprintf(
            fp,
            c"... Time Cycles     %llu\n".as_ptr(),
            (*tc).time_cycles,
        ) as size_t;
        ret += fprintf(fp, c"... Time Mask       %#llx\n".as_ptr(), (*tc).time_mask) as size_t;
        ret += fprintf(
            fp,
            c"... Cap Time Zero   %d\n".as_ptr(),
            (*tc).cap_user_time_zero,
        ) as size_t;
        ret += fprintf(
            fp,
            c"... Cap Time Short  %d\n".as_ptr(),
            (*tc).cap_user_time_short,
        ) as size_t;
    }

    return ret;
}
