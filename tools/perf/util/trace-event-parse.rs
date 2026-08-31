// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2009, Steven Rostedt <srostedt@redhat.com>
 */

use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
use std::mem;
use std::ptr;

/* C dependencies: debug.h, trace-event.h, linux/ctype.h, linux/kernel.h, event-parse.h */

const TEP_PRINT_FIELD: c_int = 0;
const TEP_PRINT_FLAGS: c_int = 1;
const TEP_PRINT_OP: c_int = 2;
const TEP_PRINT_INFO: c_int = 0;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scripting_context {
    pub pevent: *mut tep_handle,
    pub event_data: *mut c_void,
}

#[repr(C)]
pub struct tep_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_event {
    pub tep: *mut tep_handle,
    pub print_fmt: tep_print_fmt,
}

#[repr(C)]
pub struct tep_print_fmt {
    pub args: *mut tep_print_arg,
}

#[repr(C)]
pub struct tep_format_field {
    pub event: *mut tep_event,
    pub offset: c_int,
    pub size: c_int,
}

#[repr(C)]
pub struct tep_record {
    pub cpu: c_int,
    pub size: c_int,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct trace_seq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_print_flag_sym {
    pub next: *mut tep_print_flag_sym,
    pub value: *const c_char,
    pub str_: *const c_char,
}

#[repr(C)]
pub struct tep_print_arg {
    pub next: *mut tep_print_arg,
    pub type_: c_int,
    pub field: tep_print_arg_field,
    pub op: tep_print_arg_op,
    pub flags: tep_print_arg_flags,
}

#[repr(C)]
pub struct tep_print_arg_field {
    pub field: *mut tep_format_field,
}

#[repr(C)]
pub struct tep_print_arg_op {
    pub left: *mut tep_print_arg,
    pub right: *mut tep_print_arg,
}

#[repr(C)]
pub struct tep_print_arg_flags {
    pub field: *mut tep_print_arg,
    pub flags: *mut tep_print_flag_sym,
}

#[repr(C)]
pub struct flag {
    pub name: *const c_char,
    pub value: c_ulonglong,
}

unsafe extern "C" {
    fn tep_get_first_event(pevent: *mut tep_handle) -> *mut tep_event;
    fn tep_find_common_field(event: *mut tep_event, type_: *const c_char) -> *mut tep_format_field;
    fn tep_read_number(pevent: *mut tep_handle, ptr: *const c_void, size: c_int) -> c_ulonglong;
    fn tep_find_any_field(event: *mut tep_event, name: *const c_char) -> *mut tep_format_field;
    fn tep_read_number_field(field: *mut tep_format_field, data: *mut c_void, val: *mut c_ulonglong);
    fn tep_print_event(
        pevent: *mut tep_handle,
        s: *mut trace_seq,
        record: *mut tep_record,
        fmt: *const c_char,
        print_type: c_int,
    );
    fn trace_seq_init(s: *mut trace_seq);
    fn trace_seq_do_fprintf(s: *mut trace_seq, fp: *mut FILE);
    fn trace_seq_destroy(s: *mut trace_seq);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn pr_warning(fmt: *const c_char, ...);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn tep_register_print_string(pevent: *mut tep_handle, str_: *mut c_char, addr: c_ulonglong) -> c_int;
    fn free(ptr: *mut c_void);
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn tep_register_comm(pevent: *mut tep_handle, comm: *const c_char, pid: c_int) -> c_int;
    fn tep_parse_event(
        pevent: *mut tep_handle,
        buf: *mut c_char,
        size: c_ulong,
        sys: *const c_char,
    ) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

unsafe fn get_common_field(
    context: *mut scripting_context,
    offset: *mut c_int,
    size: *mut c_int,
    type_: *const c_char,
) -> c_int {
    let pevent: *mut tep_handle = (*context).pevent;
    let mut event: *mut tep_event;
    let mut field: *mut tep_format_field;

    if *size == 0 {
        event = tep_get_first_event(pevent);
        if event.is_null() {
            return 0;
        }

        field = tep_find_common_field(event, type_);
        if field.is_null() {
            return 0;
        }
        *offset = (*field).offset;
        *size = (*field).size;
    }

    tep_read_number(
        pevent,
        ((*context).event_data as *mut u8).offset(*offset as isize) as *const c_void,
        *size,
    ) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn common_lock_depth(context: *mut scripting_context) -> c_int {
    static mut OFFSET: c_int = 0;
    static mut SIZE: c_int = 0;
    let ret: c_int;

    ret = get_common_field(
        context,
        &raw mut SIZE,
        &raw mut OFFSET,
        c"common_lock_depth".as_ptr(),
    );
    if ret < 0 {
        return -1;
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn common_flags(context: *mut scripting_context) -> c_int {
    static mut OFFSET: c_int = 0;
    static mut SIZE: c_int = 0;
    let ret: c_int;

    ret = get_common_field(
        context,
        &raw mut SIZE,
        &raw mut OFFSET,
        c"common_flags".as_ptr(),
    );
    if ret < 0 {
        return -1;
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn common_pc(context: *mut scripting_context) -> c_int {
    static mut OFFSET: c_int = 0;
    static mut SIZE: c_int = 0;
    let ret: c_int;

    ret = get_common_field(
        context,
        &raw mut SIZE,
        &raw mut OFFSET,
        c"common_preempt_count".as_ptr(),
    );
    if ret < 0 {
        return -1;
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn raw_field_value(
    event: *mut tep_event,
    name: *const c_char,
    data: *mut c_void,
) -> c_ulonglong {
    let mut field: *mut tep_format_field;
    let mut val: c_ulonglong = 0;

    field = tep_find_any_field(event, name);
    if field.is_null() {
        return 0_u64;
    }

    tep_read_number_field(field, data, &mut val);

    val
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_size(event: *mut tep_event, ptr: *mut c_void, size: c_int) -> c_ulonglong {
    tep_read_number((*event).tep, ptr, size)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_format__fprintf(
    event: *const tep_event,
    cpu: c_int,
    data: *mut c_void,
    size: c_int,
    fp: *mut FILE,
) {
    let mut record: tep_record = mem::zeroed();
    let mut s: trace_seq = mem::zeroed();

    memset(
        &mut record as *mut tep_record as *mut c_void,
        0,
        mem::size_of::<tep_record>(),
    );
    record.cpu = cpu;
    record.size = size;
    record.data = data;

    trace_seq_init(&mut s);
    tep_print_event((*event).tep, &mut s, &mut record, c"%s".as_ptr(), TEP_PRINT_INFO);
    trace_seq_do_fprintf(&mut s, fp);
    trace_seq_destroy(&mut s);
}

/*
 * prev_state is of size long, which is 32 bits on 32 bit architectures.
 * As it needs to have the same bits for both 32 bit and 64 bit architectures
 * we can just assume that the flags we care about will all be within
 * the 32 bits.
 */
const MAX_STATE_BITS: usize = 32;

unsafe fn convert_sym(mut sym: *mut tep_print_flag_sym) -> *const c_char {
    static mut SAVE_STATES: [c_char; MAX_STATE_BITS + 1] = [0; MAX_STATE_BITS + 1];

    memset(
        (&raw mut SAVE_STATES) as *mut c_void,
        0,
        mem::size_of::<[c_char; MAX_STATE_BITS + 1]>(),
    );

    /* This is the flags for the prev_state_field, now make them into a string */
    while !sym.is_null() {
        let mut bitmask: c_long = strtoul((*sym).value, ptr::null_mut(), 0) as c_long;
        let mut i: c_int;

        i = 0;
        while (bitmask & 1) == 0 {
            i += 1;
            bitmask >>= 1;
        }

        if i >= MAX_STATE_BITS as c_int {
            sym = (*sym).next;
            continue;
        }

        SAVE_STATES[i as usize] = *(*sym).str_;
        sym = (*sym).next;
    }

    (&raw const SAVE_STATES) as *const c_char
}

unsafe fn find_arg_field(
    prev_state_field: *mut tep_format_field,
    arg: *mut tep_print_arg,
) -> *mut tep_print_arg_field {
    let mut field: *mut tep_print_arg_field;

    if arg.is_null() {
        return ptr::null_mut();
    }

    if (*arg).type_ == TEP_PRINT_FIELD {
        return &mut (*arg).field;
    }

    if (*arg).type_ == TEP_PRINT_OP {
        field = find_arg_field(prev_state_field, (*arg).op.left);
        if !field.is_null() && (*field).field == prev_state_field {
            return field;
        }
        field = find_arg_field(prev_state_field, (*arg).op.right);
        if !field.is_null() && (*field).field == prev_state_field {
            return field;
        }
    }
    ptr::null_mut()
}

unsafe fn test_flags(
    prev_state_field: *mut tep_format_field,
    arg: *mut tep_print_arg,
) -> *mut tep_print_flag_sym {
    let mut field: *mut tep_print_arg_field;

    field = find_arg_field(prev_state_field, (*arg).flags.field);
    if field.is_null() {
        return ptr::null_mut();
    }

    (*arg).flags.flags
}

unsafe fn search_op(
    prev_state_field: *mut tep_format_field,
    arg: *mut tep_print_arg,
) -> *mut tep_print_flag_sym {
    let mut sym: *mut tep_print_flag_sym = ptr::null_mut();

    if arg.is_null() {
        return ptr::null_mut();
    }

    if (*arg).type_ == TEP_PRINT_OP {
        sym = search_op(prev_state_field, (*arg).op.left);
        if !sym.is_null() {
            return sym;
        }

        sym = search_op(prev_state_field, (*arg).op.right);
        if !sym.is_null() {
            return sym;
        }
    } else if (*arg).type_ == TEP_PRINT_FLAGS {
        sym = test_flags(prev_state_field, arg);
    }

    sym
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_task_states(state_field: *mut tep_format_field) -> *const c_char {
    let mut sym: *mut tep_print_flag_sym;
    let mut arg: *mut tep_print_arg;
    let event: *mut tep_event;

    event = (*state_field).event;

    /*
     * Look at the event format fields, and search for where
     * the prev_state is parsed via the format flags.
     */
    arg = (*event).print_fmt.args;
    while !arg.is_null() {
        /*
         * Currently, the __print_flags() for the prev_state
         * is embedded in operations, so they too must be
         * searched.
         */
        sym = search_op(state_field, arg);
        if !sym.is_null() {
            return convert_sym(sym);
        }
        arg = (*arg).next;
    }
    ptr::null()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_ftrace_printk(
    pevent: *mut tep_handle,
    file: *mut c_char,
    _size: c_uint,
) {
    let mut addr: c_ulonglong;
    let mut printk: *mut c_char;
    let mut line: *mut c_char;
    let mut next: *mut c_char = ptr::null_mut();
    let mut addr_str: *mut c_char;
    let mut fmt: *mut c_char = ptr::null_mut();

    line = strtok_r(file, c"\n".as_ptr(), &mut next);
    while !line.is_null() {
        addr_str = strtok_r(line, c":".as_ptr(), &mut fmt);
        if addr_str.is_null() {
            pr_warning(c"printk format with empty entry".as_ptr());
            break;
        }
        addr = strtoull(addr_str, ptr::null_mut(), 16);
        /* fmt still has a space, skip it */
        printk = strdup(fmt.offset(1));
        line = strtok_r(ptr::null_mut(), c"\n".as_ptr(), &mut next);
        tep_register_print_string(pevent, printk, addr);
        free(printk as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_saved_cmdline(
    pevent: *mut tep_handle,
    file: *mut c_char,
    _size: c_uint,
) {
    let mut comm: [c_char; 17] = [0; 17]; /* Max comm length in the kernel is 16. */
    let mut line: *mut c_char;
    let mut next: *mut c_char = ptr::null_mut();
    let mut pid: c_int = 0;

    line = strtok_r(file, c"\n".as_ptr(), &mut next);
    while !line.is_null() {
        if sscanf(line, c"%d %16s".as_ptr(), &mut pid, comm.as_mut_ptr()) == 2 {
            tep_register_comm(pevent, comm.as_ptr(), pid);
        }
        line = strtok_r(ptr::null_mut(), c"\n".as_ptr(), &mut next);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_ftrace_file(
    pevent: *mut tep_handle,
    buf: *mut c_char,
    size: c_ulong,
) -> c_int {
    tep_parse_event(pevent, buf, size, c"ftrace".as_ptr())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_event_file(
    pevent: *mut tep_handle,
    buf: *mut c_char,
    size: c_ulong,
    sys: *mut c_char,
) -> c_int {
    tep_parse_event(pevent, buf, size, sys)
}

static FLAGS: [flag; 12] = [
    flag { name: c"HI_SOFTIRQ".as_ptr(), value: 0 },
    flag { name: c"TIMER_SOFTIRQ".as_ptr(), value: 1 },
    flag { name: c"NET_TX_SOFTIRQ".as_ptr(), value: 2 },
    flag { name: c"NET_RX_SOFTIRQ".as_ptr(), value: 3 },
    flag { name: c"BLOCK_SOFTIRQ".as_ptr(), value: 4 },
    flag { name: c"IRQ_POLL_SOFTIRQ".as_ptr(), value: 5 },
    flag { name: c"TASKLET_SOFTIRQ".as_ptr(), value: 6 },
    flag { name: c"SCHED_SOFTIRQ".as_ptr(), value: 7 },
    flag { name: c"HRTIMER_SOFTIRQ".as_ptr(), value: 8 },
    flag { name: c"RCU_SOFTIRQ".as_ptr(), value: 9 },
    flag { name: c"HRTIMER_NORESTART".as_ptr(), value: 0 },
    flag { name: c"HRTIMER_RESTART".as_ptr(), value: 1 },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn eval_flag(flag: *const c_char) -> c_ulonglong {
    let mut i: c_int;

    /*
     * Some flags in the format files do not get converted.
     * If the flag is not numeric, see if it is something that
     * we already know about.
     */
    if *flag >= b'0' as c_char && *flag <= b'9' as c_char {
        return strtoull(flag, ptr::null_mut(), 0);
    }

    i = 0;
    while i < FLAGS.len() as c_int {
        if strcmp(FLAGS[i as usize].name, flag) == 0 {
            return FLAGS[i as usize].value;
        }
        i += 1;
    }

    0
}
