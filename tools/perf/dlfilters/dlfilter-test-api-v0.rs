// SPDX-License-Identifier: GPL-2.0
/*
 * dlfilter-test-api-v0.c: test original (v0) API for perf --dlfilter shared object
 * Copyright (c) 2021, Intel Corporation.
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem;
use core::ptr;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type __s32 = i32;

/* Copy original (v0) API instead of including current API */
/* Rust translation references the Linux perf ABI types/constants used here. */

const PERF_RECORD_MISC_USER: __u16 = 1 << 0;
const PERF_TYPE_HARDWARE: __u32 = 0;
const PERF_COUNT_HW_BRANCH_INSTRUCTIONS: __u64 = 4;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
}

#[repr(C)]
pub struct perf_branch_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: __u32,
    pub size: __u32,
    pub config: __u64,
}

/* Definitions for perf_dlfilter_sample flags */
pub const PERF_DLFILTER_FLAG_BRANCH: __u64 = 1u64 << 0;
pub const PERF_DLFILTER_FLAG_CALL: __u64 = 1u64 << 1;
pub const PERF_DLFILTER_FLAG_RETURN: __u64 = 1u64 << 2;
pub const PERF_DLFILTER_FLAG_CONDITIONAL: __u64 = 1u64 << 3;
pub const PERF_DLFILTER_FLAG_SYSCALLRET: __u64 = 1u64 << 4;
pub const PERF_DLFILTER_FLAG_ASYNC: __u64 = 1u64 << 5;
pub const PERF_DLFILTER_FLAG_INTERRUPT: __u64 = 1u64 << 6;
pub const PERF_DLFILTER_FLAG_TX_ABORT: __u64 = 1u64 << 7;
pub const PERF_DLFILTER_FLAG_TRACE_BEGIN: __u64 = 1u64 << 8;
pub const PERF_DLFILTER_FLAG_TRACE_END: __u64 = 1u64 << 9;
pub const PERF_DLFILTER_FLAG_IN_TX: __u64 = 1u64 << 10;
pub const PERF_DLFILTER_FLAG_VMENTRY: __u64 = 1u64 << 11;
pub const PERF_DLFILTER_FLAG_VMEXIT: __u64 = 1u64 << 12;

/*
 * perf sample event information (as per perf script and <linux/perf_event.h>)
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_dlfilter_sample {
    pub size: __u32, /* Size of this structure (for compatibility checking) */
    pub ins_lat: __u16, /* Refer PERF_SAMPLE_WEIGHT_TYPE in <linux/perf_event.h> */
    pub p_stage_cyc: __u16, /* Refer PERF_SAMPLE_WEIGHT_TYPE in <linux/perf_event.h> */
    pub ip: __u64,
    pub pid: __s32,
    pub tid: __s32,
    pub time: __u64,
    pub addr: __u64,
    pub id: __u64,
    pub stream_id: __u64,
    pub period: __u64,
    pub weight: __u64, /* Refer PERF_SAMPLE_WEIGHT_TYPE in <linux/perf_event.h> */
    pub transaction: __u64, /* Refer PERF_SAMPLE_TRANSACTION in <linux/perf_event.h> */
    pub insn_cnt: __u64, /* For instructions-per-cycle (IPC) */
    pub cyc_cnt: __u64, /* For instructions-per-cycle (IPC) */
    pub cpu: __s32,
    pub flags: __u32, /* Refer PERF_DLFILTER_FLAG_* above */
    pub data_src: __u64, /* Refer PERF_SAMPLE_DATA_SRC in <linux/perf_event.h> */
    pub phys_addr: __u64, /* Refer PERF_SAMPLE_PHYS_ADDR in <linux/perf_event.h> */
    pub data_page_size: __u64, /* Refer PERF_SAMPLE_DATA_PAGE_SIZE in <linux/perf_event.h> */
    pub code_page_size: __u64, /* Refer PERF_SAMPLE_CODE_PAGE_SIZE in <linux/perf_event.h> */
    pub cgroup: __u64, /* Refer PERF_SAMPLE_CGROUP in <linux/perf_event.h> */
    pub cpumode: __u8, /* Refer CPUMODE_MASK etc in <linux/perf_event.h> */
    pub addr_correlates_sym: __u8, /* True => resolve_addr() can be called */
    pub misc: __u16, /* Refer perf_event_header in <linux/perf_event.h> */
    pub raw_size: __u32, /* Refer PERF_SAMPLE_RAW in <linux/perf_event.h> */
    pub raw_data: *const c_void, /* Refer PERF_SAMPLE_RAW in <linux/perf_event.h> */
    pub brstack_nr: __u64, /* Number of brstack entries */
    pub brstack: *const perf_branch_entry, /* Refer <linux/perf_event.h> */
    pub raw_callchain_nr: __u64, /* Number of raw_callchain entries */
    pub raw_callchain: *const __u64, /* Refer <linux/perf_event.h> */
    pub event: *const c_char,
}

/*
 * Address location (as per perf script)
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_dlfilter_al {
    pub size: __u32, /* Size of this structure (for compatibility checking) */
    pub symoff: __u32,
    pub sym: *const c_char,
    pub addr: __u64, /* Mapped address (from dso) */
    pub sym_start: __u64,
    pub sym_end: __u64,
    pub dso: *const c_char,
    pub sym_binding: __u8, /* STB_LOCAL, STB_GLOBAL or STB_WEAK, refer <elf.h> */
    pub is_64_bit: __u8, /* Only valid if dso is not NULL */
    pub is_kernel_ip: __u8, /* True if in kernel space */
    pub buildid_size: __u32,
    pub buildid: *mut __u8,
    /* Below members are only populated by resolve_ip() */
    pub filtered: __u8, /* True if this sample event will be filtered out */
    pub comm: *const c_char,
}

#[repr(C)]
pub struct perf_dlfilter_fns {
    /* Return information about ip */
    pub resolve_ip: Option<unsafe extern "C" fn(ctx: *mut c_void) -> *const perf_dlfilter_al>,
    /* Return information about addr (if addr_correlates_sym) */
    pub resolve_addr: Option<unsafe extern "C" fn(ctx: *mut c_void) -> *const perf_dlfilter_al>,
    /* Return arguments from --dlarg option */
    pub args: Option<unsafe extern "C" fn(ctx: *mut c_void, dlargc: *mut c_int) -> *mut *mut c_char>,
    /*
     * Return information about address (al->size must be set before
     * calling). Returns 0 on success, -1 otherwise.
     */
    pub resolve_address: Option<
        unsafe extern "C" fn(
            ctx: *mut c_void,
            address: __u64,
            al: *mut perf_dlfilter_al,
        ) -> __s32,
    >,
    /* Return instruction bytes and length */
    pub insn: Option<unsafe extern "C" fn(ctx: *mut c_void, length: *mut __u32) -> *const __u8>,
    /* Return source file name and line number */
    pub srcline: Option<unsafe extern "C" fn(ctx: *mut c_void, line_number: *mut __u32) -> *const c_char>,
    /* Return perf_event_attr, refer <linux/perf_event.h> */
    pub attr: Option<unsafe extern "C" fn(ctx: *mut c_void) -> *mut perf_event_attr>,
    /* Read object code, return numbers of bytes read */
    pub object_code:
        Option<unsafe extern "C" fn(ctx: *mut c_void, ip: __u64, buf: *mut c_void, len: __u32) -> __s32>,
    /* Reserved */
    pub reserved: [Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>; 120],
}

#[unsafe(no_mangle)]
pub static mut perf_dlfilter_fns: perf_dlfilter_fns = perf_dlfilter_fns {
    resolve_ip: None,
    resolve_addr: None,
    args: None,
    resolve_address: None,
    insn: None,
    srcline: None,
    attr: None,
    object_code: None,
    reserved: [None; 120],
};

static mut verbose: c_int = 0;

macro_rules! pr_debug {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        if verbose > 0 {
            fprintf(stderr, $fmt.as_ptr() as *const c_char $(, $arg)*);
        }
    }};
}

unsafe fn test_fail(msg: *const c_char) -> c_int {
    pr_debug!(b"%s\n\0", msg);
    -1
}

macro_rules! check {
    ($x:expr, $msg:expr) => {{
        if !($x) {
            return test_fail($msg.as_ptr() as *const c_char);
        }
    }};
}

#[repr(C)]
struct filter_data {
    ip: __u64,
    addr: __u64,
    do_early: c_int,
    early_filter_cnt: c_int,
    filter_cnt: c_int,
}

static mut filt_dat: *mut filter_data = ptr::null_mut();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn start(data: *mut *mut c_void, ctx: *mut c_void) -> c_int {
    let mut dlargc: c_int = 0;
    let dlargv: *mut *mut c_char;
    let d: *mut filter_data;
    static mut called: bool = false;

    verbose = 1;

    check!(!filt_dat.is_null() == false && !called, b"Check '!filt_dat && !called' failed\n\0");
    called = true;

    d = calloc(1, mem::size_of::<filter_data>()) as *mut filter_data;
    if d.is_null() {
        test_fail(b"Failed to allocate memory\0".as_ptr() as *const c_char);
    }
    filt_dat = d;
    *data = d as *mut c_void;

    dlargv = (perf_dlfilter_fns.args.unwrap())(ctx, &mut dlargc);

    check!(dlargc == 6, b"Check 'dlargc == 6' failed\n\0");
    check!(
        strcmp(*dlargv.add(0), b"first\0".as_ptr() as *const c_char) == 0,
        b"Check '!strcmp(dlargv[0], \"first\")' failed\n\0"
    );
    verbose = strtol(*dlargv.add(1), ptr::null_mut(), 0) as c_int;
    (*d).ip = strtoull(*dlargv.add(2), ptr::null_mut(), 0) as __u64;
    (*d).addr = strtoull(*dlargv.add(3), ptr::null_mut(), 0) as __u64;
    (*d).do_early = strtol(*dlargv.add(4), ptr::null_mut(), 0) as c_int;
    check!(
        strcmp(*dlargv.add(5), b"last\0".as_ptr() as *const c_char) == 0,
        b"Check '!strcmp(dlargv[5], \"last\")' failed\n\0"
    );

    pr_debug!(b"%s API\n\0", b"start\0".as_ptr() as *const c_char);

    0
}

macro_rules! check_sample {
    ($sample:expr, $expected:expr, $field:ident, $msg:expr) => {{
        if (*$sample).$field != $expected.$field {
            return test_fail($msg.as_ptr() as *const c_char);
        }
    }};
}

unsafe fn check_sample(d: *mut filter_data, sample: *const perf_dlfilter_sample) -> c_int {
    let mut expected: perf_dlfilter_sample = mem::zeroed();

    expected.ip = (*d).ip;
    expected.pid = 12345;
    expected.tid = 12346;
    expected.time = 1234567890;
    expected.addr = (*d).addr;
    expected.id = 99;
    expected.stream_id = 101;
    expected.period = 543212345;
    expected.cpu = 31;
    expected.cpumode = PERF_RECORD_MISC_USER as __u8;
    expected.addr_correlates_sym = 1;
    expected.misc = PERF_RECORD_MISC_USER;

    check!(
        (*sample).size as usize >= mem::size_of::<perf_dlfilter_sample>(),
        b"Check 'sample->size >= sizeof(struct perf_dlfilter_sample)' failed\n\0"
    );

    check_sample!(sample, expected, ip, b"'ip' not expected value\n\0");
    check_sample!(sample, expected, pid, b"'pid' not expected value\n\0");
    check_sample!(sample, expected, tid, b"'tid' not expected value\n\0");
    check_sample!(sample, expected, time, b"'time' not expected value\n\0");
    check_sample!(sample, expected, addr, b"'addr' not expected value\n\0");
    check_sample!(sample, expected, id, b"'id' not expected value\n\0");
    check_sample!(sample, expected, stream_id, b"'stream_id' not expected value\n\0");
    check_sample!(sample, expected, period, b"'period' not expected value\n\0");
    check_sample!(sample, expected, cpu, b"'cpu' not expected value\n\0");
    check_sample!(sample, expected, cpumode, b"'cpumode' not expected value\n\0");
    check_sample!(
        sample,
        expected,
        addr_correlates_sym,
        b"'addr_correlates_sym' not expected value\n\0"
    );
    check_sample!(sample, expected, misc, b"'misc' not expected value\n\0");

    check!((*sample).raw_data.is_null(), b"Check '!sample->raw_data' failed\n\0");
    check_sample!(sample, expected, brstack_nr, b"'brstack_nr' not expected value\n\0");
    check!((*sample).brstack.is_null(), b"Check '!sample->brstack' failed\n\0");
    check_sample!(
        sample,
        expected,
        raw_callchain_nr,
        b"'raw_callchain_nr' not expected value\n\0"
    );
    check!(
        (*sample).raw_callchain.is_null(),
        b"Check '!sample->raw_callchain' failed\n\0"
    );

    const EVENT_NAME: &[u8] = b"branches\0";
    check!(
        strncmp(
            (*sample).event,
            EVENT_NAME.as_ptr() as *const c_char,
            strlen(EVENT_NAME.as_ptr() as *const c_char),
        ) == 0,
        b"Check '!strncmp(sample->event, EVENT_NAME, strlen(EVENT_NAME))' failed\n\0"
    );

    0
}

unsafe fn check_al(ctx: *mut c_void) -> c_int {
    let al: *const perf_dlfilter_al;

    al = (perf_dlfilter_fns.resolve_ip.unwrap())(ctx);
    if al.is_null() {
        return test_fail(b"resolve_ip() failed\0".as_ptr() as *const c_char);
    }

    check!(
        !(*al).sym.is_null() && strcmp(b"foo\0".as_ptr() as *const c_char, (*al).sym) == 0,
        b"Check 'al->sym && !strcmp(\"foo\", al->sym)' failed\n\0"
    );
    check!((*al).symoff == 0, b"Check '!al->symoff' failed\n\0");

    0
}

unsafe fn check_addr_al(ctx: *mut c_void) -> c_int {
    let addr_al: *const perf_dlfilter_al;

    addr_al = (perf_dlfilter_fns.resolve_addr.unwrap())(ctx);
    if addr_al.is_null() {
        return test_fail(b"resolve_addr() failed\0".as_ptr() as *const c_char);
    }

    check!(
        !(*addr_al).sym.is_null()
            && strcmp(b"bar\0".as_ptr() as *const c_char, (*addr_al).sym) == 0,
        b"Check 'addr_al->sym && !strcmp(\"bar\", addr_al->sym)' failed\n\0"
    );
    check!((*addr_al).symoff == 0, b"Check '!addr_al->symoff' failed\n\0");

    0
}

unsafe fn check_address_al(ctx: *mut c_void, sample: *const perf_dlfilter_sample) -> c_int {
    let mut address_al: perf_dlfilter_al = mem::zeroed();
    let al: *const perf_dlfilter_al;

    al = (perf_dlfilter_fns.resolve_ip.unwrap())(ctx);
    if al.is_null() {
        return test_fail(b"resolve_ip() failed\0".as_ptr() as *const c_char);
    }

    address_al.size = mem::size_of_val(&address_al) as __u32;
    if (perf_dlfilter_fns.resolve_address.unwrap())(ctx, (*sample).ip, &mut address_al) != 0 {
        return test_fail(b"resolve_address() failed\0".as_ptr() as *const c_char);
    }

    check!(
        !address_al.sym.is_null() && !(*al).sym.is_null(),
        b"Check 'address_al.sym && al->sym' failed\n\0"
    );
    check!(
        strcmp(address_al.sym, (*al).sym) == 0,
        b"Check '!strcmp(address_al.sym, al->sym)' failed\n\0"
    );
    check!(
        address_al.addr == (*al).addr,
        b"Check 'address_al.addr == al->addr' failed\n\0"
    );
    check!(
        address_al.sym_start == (*al).sym_start,
        b"Check 'address_al.sym_start == al->sym_start' failed\n\0"
    );
    check!(
        address_al.sym_end == (*al).sym_end,
        b"Check 'address_al.sym_end == al->sym_end' failed\n\0"
    );
    check!(
        !address_al.dso.is_null() && !(*al).dso.is_null(),
        b"Check 'address_al.dso && al->dso' failed\n\0"
    );
    check!(
        strcmp(address_al.dso, (*al).dso) == 0,
        b"Check '!strcmp(address_al.dso, al->dso)' failed\n\0"
    );

    0
}

unsafe fn check_attr(ctx: *mut c_void) -> c_int {
    let attr: *mut perf_event_attr = (perf_dlfilter_fns.attr.unwrap())(ctx);

    check!(!attr.is_null(), b"Check 'attr' failed\n\0");
    check!(
        (*attr).type_ == PERF_TYPE_HARDWARE,
        b"Check 'attr->type == PERF_TYPE_HARDWARE' failed\n\0"
    );
    check!(
        (*attr).config == PERF_COUNT_HW_BRANCH_INSTRUCTIONS,
        b"Check 'attr->config == PERF_COUNT_HW_BRANCH_INSTRUCTIONS' failed\n\0"
    );

    0
}

unsafe fn check_object_code(ctx: *mut c_void, sample: *const perf_dlfilter_sample) -> c_int {
    let mut buf: [__u8; 15] = [0; 15];

    check!(
        (perf_dlfilter_fns.object_code.unwrap())(
            ctx,
            (*sample).ip,
            buf.as_mut_ptr() as *mut c_void,
            mem::size_of_val(&buf) as __u32,
        ) > 0,
        b"Check 'perf_dlfilter_fns.object_code(ctx, sample->ip, buf, sizeof(buf)) > 0' failed\n\0"
    );

    0
}

unsafe fn do_checks(
    data: *mut c_void,
    sample: *const perf_dlfilter_sample,
    ctx: *mut c_void,
    early: bool,
) -> c_int {
    let d: *mut filter_data = data as *mut filter_data;

    check!(
        !data.is_null() && filt_dat == data as *mut filter_data,
        b"Check 'data && filt_dat == data' failed\n\0"
    );

    if early {
        check!(
            (*d).early_filter_cnt == 0,
            b"Check '!d->early_filter_cnt' failed\n\0"
        );
        (*d).early_filter_cnt += 1;
    } else {
        check!((*d).filter_cnt == 0, b"Check '!d->filter_cnt' failed\n\0");
        check!(
            (*d).early_filter_cnt != 0,
            b"Check 'd->early_filter_cnt' failed\n\0"
        );
        check!((*d).do_early != 2, b"Check 'd->do_early != 2' failed\n\0");
        (*d).filter_cnt += 1;
    }

    if check_sample(data as *mut filter_data, sample) != 0 {
        return -1;
    }

    if check_attr(ctx) != 0 {
        return -1;
    }

    if early && (*d).do_early == 0 {
        return 0;
    }

    if check_al(ctx) != 0
        || check_addr_al(ctx) != 0
        || check_address_al(ctx, sample) != 0
        || check_object_code(ctx, sample) != 0
    {
        return -1;
    }

    if early {
        return ((*d).do_early == 2) as c_int;
    }

    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn filter_event_early(
    data: *mut c_void,
    sample: *const perf_dlfilter_sample,
    ctx: *mut c_void,
) -> c_int {
    pr_debug!(
        b"%s API\n\0",
        b"filter_event_early\0".as_ptr() as *const c_char
    );

    do_checks(data, sample, ctx, true)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn filter_event(
    data: *mut c_void,
    sample: *const perf_dlfilter_sample,
    ctx: *mut c_void,
) -> c_int {
    pr_debug!(b"%s API\n\0", b"filter_event\0".as_ptr() as *const c_char);

    do_checks(data, sample, ctx, false)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn stop(data: *mut c_void, _ctx: *mut c_void) -> c_int {
    static mut called: bool = false;

    pr_debug!(b"%s API\n\0", b"stop\0".as_ptr() as *const c_char);

    check!(
        !data.is_null() && filt_dat == data as *mut filter_data && !called,
        b"Check 'data && filt_dat == data && !called' failed\n\0"
    );
    called = true;

    free(data);
    filt_dat = ptr::null_mut();
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn filter_description(
    long_description: *mut *const c_char,
) -> *const c_char {
    *long_description = b"Filter used by the 'dlfilter C API' perf test\0".as_ptr() as *const c_char;
    b"dlfilter to test v0 C API\0".as_ptr() as *const c_char
}
