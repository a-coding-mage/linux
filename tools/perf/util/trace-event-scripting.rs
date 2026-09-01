// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * trace-event-scripting.  Scripting engine common and initialization code.
 *
 * Copyright (C) 2009-2010 Tom Zanussi <tzanussi@gmail.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type size_t = usize;
type u32 = u32;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct scripting_context {
    pub pevent: *mut tep_handle,
    pub event_data: *mut c_void,
    pub event: *mut perf_event,
    pub sample: *mut perf_sample,
    pub al: *mut addr_location,
    pub addr_al: *mut addr_location,
}

#[repr(C)]
pub struct scripting_ops {
    pub name: *const c_char,
    pub dirname: *const c_char,
    pub start_script: Option<
        unsafe extern "C" fn(
            script: *const c_char,
            argc: c_int,
            argv: *const *const c_char,
            session: *mut perf_session,
        ) -> c_int,
    >,
    pub flush_script: Option<unsafe extern "C" fn() -> c_int>,
    pub stop_script: Option<unsafe extern "C" fn() -> c_int>,
    pub process_event: Option<
        unsafe extern "C" fn(
            event: *mut perf_event,
            sample: *mut perf_sample,
            al: *mut addr_location,
            addr_al: *mut addr_location,
        ),
    >,
    pub generate_script:
        Option<unsafe extern "C" fn(pevent: *mut tep_handle, outfile: *const c_char) -> c_int>,
}

#[repr(C)]
pub struct perf_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_event {
    pub tep: *mut tep_handle,
}

#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    pub evsel: *mut evsel,
    pub raw_data: *mut c_void,
}

#[repr(C)]
pub struct addr_location {
    _private: [u8; 0],
}

#[repr(C)]
struct script_spec {
    node: list_head,
    ops: *mut scripting_ops,
    spec: [c_char; 0],
}

#[repr(C)]
struct flag_name {
    flags: u32,
    name: *const c_char,
}

unsafe extern "C" {
    fn malloc(size: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;

    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn zfree(ptr: *mut *mut scripting_context);
    fn pr_err(format: *const c_char, ...);

    /* Provided by trace-event/perf headers when HAVE_LIBTRACEEVENT is enabled. */
    fn evsel__tp_format(evsel: *mut evsel) -> *const tep_event;
}

unsafe impl Sync for scripting_ops {}
unsafe impl Sync for flag_name {}

unsafe extern "C" {
    static PERF_MAX_STACK_DEPTH: c_uint;
    static PERF_IP_FLAG_BRANCH: u32;
    static PERF_IP_FLAG_CALL: u32;
    static PERF_IP_FLAG_RETURN: u32;
    static PERF_IP_FLAG_CONDITIONAL: u32;
    static PERF_IP_FLAG_INTERRUPT: u32;
    static PERF_IP_FLAG_SYSCALLRET: u32;
    static PERF_IP_FLAG_ASYNC: u32;
    static PERF_IP_FLAG_TX_ABORT: u32;
    static PERF_IP_FLAG_TRACE_BEGIN: u32;
    static PERF_IP_FLAG_TRACE_END: u32;
    static PERF_IP_FLAG_VMENTRY: u32;
    static PERF_IP_FLAG_VMEXIT: u32;
    static PERF_IP_FLAG_BRANCH_MISS: u32;
    static PERF_IP_FLAG_NOT_TAKEN: u32;
    static PERF_ADDITIONAL_STATE_MASK: u32;
    static PERF_IP_FLAG_BRANCH_EVENT_MASK: u32;
    static PERF_IP_FLAG_IN_TX: u32;
    static PERF_IP_FLAG_INTR_DISABLE: u32;
    static PERF_IP_FLAG_INTR_TOGGLE: u32;
    static SAMPLE_FLAGS_STR_ALIGNED_SIZE: size_t;
    static PERF_IP_FLAG_CHARS: [c_char; 0];
}

#[unsafe(no_mangle)]
pub static mut scripting_max_stack: c_uint = unsafe { PERF_MAX_STACK_DEPTH };

#[unsafe(no_mangle)]
pub static mut scripting_context: *mut scripting_context = core::ptr::null_mut();

static mut script_specs: list_head = list_head {
    next: core::ptr::null_mut(),
    prev: core::ptr::null_mut(),
};

unsafe fn script_specs_head() -> *mut list_head {
    if script_specs.next.is_null() {
        script_specs.next = &raw mut script_specs;
        script_specs.prev = &raw mut script_specs;
    }
    &raw mut script_specs
}

unsafe fn script_spec_spec(s: *mut script_spec) -> *mut c_char {
    (&raw mut (*s).spec) as *mut c_char
}

unsafe fn script_spec__new(spec: *const c_char, ops: *mut scripting_ops) -> *mut script_spec {
    let s = malloc(core::mem::size_of::<script_spec>() + strlen(spec) + 1) as *mut script_spec;

    if !s.is_null() {
        strcpy(script_spec_spec(s), spec);
        (*s).ops = ops;
    }

    s
}

unsafe fn script_spec__add(s: *mut script_spec) {
    list_add_tail(&raw mut (*s).node, script_specs_head());
}

unsafe fn script_spec__find(spec: *const c_char) -> *mut script_spec {
    let head = script_specs_head();
    let mut pos = (*head).next;

    while pos != head {
        let s = pos as *mut script_spec;
        if strcasecmp(script_spec_spec(s), spec) == 0 {
            return s;
        }
        pos = (*pos).next;
    }

    core::ptr::null_mut()
}

unsafe fn script_spec_register(spec: *const c_char, ops: *mut scripting_ops) -> c_int {
    let mut s = script_spec__find(spec);
    if !s.is_null() {
        return -1;
    }

    s = script_spec__new(spec, ops);
    if s.is_null() {
        return -1;
    }

    script_spec__add(s);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn script_spec__lookup(spec: *const c_char) -> *mut scripting_ops {
    let s = script_spec__find(spec);

    if s.is_null() {
        return core::ptr::null_mut();
    }

    (*s).ops
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn script_spec__for_each(
    cb: Option<unsafe extern "C" fn(ops: *mut scripting_ops, spec: *const c_char) -> c_int>,
) -> c_int {
    let head = script_specs_head();
    let mut pos = (*head).next;
    let mut ret = 0;

    while pos != head {
        let s = pos as *mut script_spec;
        ret = cb.expect("non-null callback")((*s).ops, script_spec_spec(s));
        if ret != 0 {
            break;
        }
        pos = (*pos).next;
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn scripting_context__update(
    c: *mut scripting_context,
    event: *mut perf_event,
    sample: *mut perf_sample,
    al: *mut addr_location,
    addr_al: *mut addr_location,
) {
    /*
     * Original C uses HAVE_LIBTRACEEVENT:
     * const struct tep_event *tp_format = evsel__tp_format(sample->evsel);
     * c->pevent = tp_format ? tp_format->tep : NULL;
     * Otherwise c->pevent = NULL.
     */
    #[cfg(HAVE_LIBTRACEEVENT)]
    {
        let tp_format = evsel__tp_format((*sample).evsel);
        (*c).pevent = if !tp_format.is_null() {
            (*tp_format).tep
        } else {
            core::ptr::null_mut()
        };
    }
    #[cfg(not(HAVE_LIBTRACEEVENT))]
    {
        (*c).pevent = core::ptr::null_mut();
    }

    (*c).event_data = (*sample).raw_data;
    (*c).event = event;
    (*c).sample = sample;
    (*c).al = al;
    (*c).addr_al = addr_al;
}

unsafe extern "C" fn flush_script_unsupported() -> c_int {
    0
}

unsafe extern "C" fn stop_script_unsupported() -> c_int {
    0
}

unsafe extern "C" fn process_event_unsupported(
    _event: *mut perf_event,
    _sample: *mut perf_sample,
    _al: *mut addr_location,
    _addr_al: *mut addr_location,
) {
}

unsafe fn print_python_unsupported_msg() {
    fprintf(
        stderr,
        b"Python scripting not supported.  Install libpython and rebuild perf to enable it.\nFor example:\n  # apt-get install python-dev (ubuntu)\n  # yum install python-devel (Fedora)\n  etc.\n\0"
            .as_ptr() as *const c_char,
    );
}

unsafe extern "C" fn python_start_script_unsupported(
    _script: *const c_char,
    _argc: c_int,
    _argv: *const *const c_char,
    _session: *mut perf_session,
) -> c_int {
    print_python_unsupported_msg();

    -1
}

unsafe extern "C" fn python_generate_script_unsupported(
    _pevent: *mut tep_handle,
    _outfile: *const c_char,
) -> c_int {
    print_python_unsupported_msg();

    -1
}

#[unsafe(no_mangle)]
pub static python_scripting_unsupported_ops: scripting_ops = scripting_ops {
    name: b"Python\0".as_ptr() as *const c_char,
    dirname: b"python\0".as_ptr() as *const c_char,
    start_script: Some(python_start_script_unsupported),
    flush_script: Some(flush_script_unsupported),
    stop_script: Some(stop_script_unsupported),
    process_event: Some(process_event_unsupported),
    generate_script: Some(python_generate_script_unsupported),
};

unsafe fn register_python_scripting(scripting_ops: *mut scripting_ops) {
    if scripting_context.is_null() {
        scripting_context = malloc(core::mem::size_of::<scripting_context>()) as *mut scripting_context;
    }

    if scripting_context.is_null()
        || script_spec_register(b"Python\0".as_ptr() as *const c_char, scripting_ops) != 0
        || script_spec_register(b"py\0".as_ptr() as *const c_char, scripting_ops) != 0
    {
        pr_err(b"Error registering Python script extension: disabling it\n\0".as_ptr() as *const c_char);
        zfree(&raw mut scripting_context);
    }
}

/*
 * Original C condition:
 * #ifndef HAVE_LIBPYTHON_SUPPORT use python_scripting_unsupported_ops,
 * otherwise use extern struct scripting_ops python_scripting_ops.
 */
unsafe extern "C" {
    static mut python_scripting_ops: scripting_ops;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_python_scripting() {
    #[cfg(not(HAVE_LIBPYTHON_SUPPORT))]
    {
        register_python_scripting(&python_scripting_unsupported_ops as *const scripting_ops as *mut scripting_ops);
    }
    #[cfg(HAVE_LIBPYTHON_SUPPORT)]
    {
        register_python_scripting(&raw mut python_scripting_ops);
    }
}

/*
 * Original C encloses all Perl scripting support in #ifdef HAVE_LIBTRACEEVENT.
 */
#[cfg(HAVE_LIBTRACEEVENT)]
unsafe fn print_perl_unsupported_msg() {
    fprintf(
        stderr,
        b"Perl scripting not supported.  Install libperl and rebuild perf to enable it.\nFor example:\n  # apt-get install libperl-dev (ubuntu)\n  # yum install 'perl(ExtUtils::Embed)' (Fedora)\n  etc.\n\0"
            .as_ptr() as *const c_char,
    );
}

#[cfg(HAVE_LIBTRACEEVENT)]
unsafe extern "C" fn perl_start_script_unsupported(
    _script: *const c_char,
    _argc: c_int,
    _argv: *const *const c_char,
    _session: *mut perf_session,
) -> c_int {
    print_perl_unsupported_msg();

    -1
}

#[cfg(HAVE_LIBTRACEEVENT)]
unsafe extern "C" fn perl_generate_script_unsupported(
    _pevent: *mut tep_handle,
    _outfile: *const c_char,
) -> c_int {
    print_perl_unsupported_msg();

    -1
}

#[cfg(HAVE_LIBTRACEEVENT)]
#[unsafe(no_mangle)]
pub static perl_scripting_unsupported_ops: scripting_ops = scripting_ops {
    name: b"Perl\0".as_ptr() as *const c_char,
    dirname: b"perl\0".as_ptr() as *const c_char,
    start_script: Some(perl_start_script_unsupported),
    flush_script: Some(flush_script_unsupported),
    stop_script: Some(stop_script_unsupported),
    process_event: Some(process_event_unsupported),
    generate_script: Some(perl_generate_script_unsupported),
};

#[cfg(HAVE_LIBTRACEEVENT)]
unsafe fn register_perl_scripting(scripting_ops: *mut scripting_ops) {
    if scripting_context.is_null() {
        scripting_context = malloc(core::mem::size_of::<scripting_context>()) as *mut scripting_context;
    }

    if scripting_context.is_null()
        || script_spec_register(b"Perl\0".as_ptr() as *const c_char, scripting_ops) != 0
        || script_spec_register(b"pl\0".as_ptr() as *const c_char, scripting_ops) != 0
    {
        pr_err(b"Error registering Perl script extension: disabling it\n\0".as_ptr() as *const c_char);
        zfree(&raw mut scripting_context);
    }
}

#[cfg(HAVE_LIBTRACEEVENT)]
unsafe extern "C" {
    static mut perl_scripting_ops: scripting_ops;
}

#[cfg(HAVE_LIBTRACEEVENT)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_perl_scripting() {
    #[cfg(not(HAVE_LIBPERL_SUPPORT))]
    {
        register_perl_scripting(&perl_scripting_unsupported_ops as *const scripting_ops as *mut scripting_ops);
    }
    #[cfg(HAVE_LIBPERL_SUPPORT)]
    {
        register_perl_scripting(&raw mut perl_scripting_ops);
    }
}

static sample_flags: [flag_name; 16] = [
    flag_name { flags: unsafe { PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL }, name: b"call\0".as_ptr() as *const c_char },
    flag_name { flags: unsafe { PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN }, name: b"return\0".as_ptr() as *const c_char },
    flag_name { flags: unsafe { PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CONDITIONAL }, name: b"jcc\0".as_ptr() as *const c_char },
    flag_name { flags: unsafe { PERF_IP_FLAG_BRANCH }, name: b"jmp\0".as_ptr() as *const c_char },
    flag_name { flags: unsafe { PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_INTERRUPT }, name: b"int\0".as_ptr() as *const c_char },
    flag_name { flags: unsafe { PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN | PERF_IP_FLAG_INTERRUPT }, name: b"iret\0".as_ptr() as *const c_char },
    flag_name { flags: unsafe { PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_SYSCALLRET }, name: b"syscall\0".as_ptr() as *const c_char },
    flag_name { flags: unsafe { PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN | PERF_IP_FLAG_SYSCALLRET }, name: b"sysret\0".as_ptr() as *const c_char },
    flag_name { flags: unsafe { PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_ASYNC }, name: b"async\0".as_ptr() as *const c_char },
    flag_name { flags: unsafe { PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_ASYNC | PERF_IP_FLAG_INTERRUPT }, name: b"hw int\0".as_ptr() as *const c_char },
    flag_name { flags: unsafe { PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_TX_ABORT }, name: b"tx abrt\0".as_ptr() as *const c_char },
    flag_name { flags: unsafe { PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_TRACE_BEGIN }, name: b"tr strt\0".as_ptr() as *const c_char },
    flag_name { flags: unsafe { PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_TRACE_END }, name: b"tr end\0".as_ptr() as *const c_char },
    flag_name { flags: unsafe { PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_VMENTRY }, name: b"vmentry\0".as_ptr() as *const c_char },
    flag_name { flags: unsafe { PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_VMEXIT }, name: b"vmexit\0".as_ptr() as *const c_char },
    flag_name { flags: 0, name: core::ptr::null() },
];

static branch_events: [flag_name; 3] = [
    flag_name { flags: unsafe { PERF_IP_FLAG_BRANCH_MISS }, name: b"miss\0".as_ptr() as *const c_char },
    flag_name { flags: unsafe { PERF_IP_FLAG_NOT_TAKEN }, name: b"not_taken\0".as_ptr() as *const c_char },
    flag_name { flags: 0, name: core::ptr::null() },
];

unsafe fn sample_flags_to_name(mut flags: u32, str_: *mut c_char, size: size_t) -> c_int {
    let mut i: c_int;
    let prefix: *const c_char;
    let mut pos: c_int = 0;
    let mut ret: c_int;
    let mut ev_idx: c_int = 0;
    let xf: u32 = flags & PERF_ADDITIONAL_STATE_MASK;
    let types: u32;
    let events: u32;
    let mut xs: [c_char; 16] = [0; 16];

    /* Clear additional state bits */
    flags &= !PERF_ADDITIONAL_STATE_MASK;

    if (flags & PERF_IP_FLAG_TRACE_BEGIN) != 0 {
        prefix = b"tr strt \0".as_ptr() as *const c_char;
    } else if (flags & PERF_IP_FLAG_TRACE_END) != 0 {
        prefix = b"tr end  \0".as_ptr() as *const c_char;
    } else {
        prefix = b"\0".as_ptr() as *const c_char;
    }

    ret = snprintf(str_.add(pos as usize), size.wrapping_sub(pos as usize), b"%s\0".as_ptr() as *const c_char, prefix);
    if ret < 0 {
        return ret;
    }
    pos += ret;

    flags &= !(PERF_IP_FLAG_TRACE_BEGIN | PERF_IP_FLAG_TRACE_END);

    types = flags & !PERF_IP_FLAG_BRANCH_EVENT_MASK;
    i = 0;
    while !sample_flags[i as usize].name.is_null() {
        if sample_flags[i as usize].flags != types {
            i += 1;
            continue;
        }

        ret = snprintf(
            str_.add(pos as usize),
            size.wrapping_sub(pos as usize),
            b"%s\0".as_ptr() as *const c_char,
            sample_flags[i as usize].name,
        );
        if ret < 0 {
            return ret;
        }
        pos += ret;
        break;
    }

    events = flags & PERF_IP_FLAG_BRANCH_EVENT_MASK;
    i = 0;
    while !branch_events[i as usize].name.is_null() {
        if (branch_events[i as usize].flags & events) == 0 {
            i += 1;
            continue;
        }

        ret = snprintf(
            str_.add(pos as usize),
            size.wrapping_sub(pos as usize),
            if ev_idx == 0 { b"/%s\0".as_ptr() as *const c_char } else { b",%s\0".as_ptr() as *const c_char },
            branch_events[i as usize].name,
        );
        if ret < 0 {
            return ret;
        }
        pos += ret;
        ev_idx += 1;
        i += 1;
    }

    /* Add an end character '/' for events */
    if ev_idx != 0 {
        ret = snprintf(str_.add(pos as usize), size.wrapping_sub(pos as usize), b"/\0".as_ptr() as *const c_char);
        if ret < 0 {
            return ret;
        }
        pos += ret;
    }

    if xf == 0 {
        return pos;
    }

    snprintf(
        xs.as_mut_ptr(),
        xs.len(),
        b"(%s%s%s)\0".as_ptr() as *const c_char,
        if (flags & PERF_IP_FLAG_IN_TX) != 0 { b"x\0".as_ptr() as *const c_char } else { b"\0".as_ptr() as *const c_char },
        if (flags & PERF_IP_FLAG_INTR_DISABLE) != 0 { b"D\0".as_ptr() as *const c_char } else { b"\0".as_ptr() as *const c_char },
        if (flags & PERF_IP_FLAG_INTR_TOGGLE) != 0 { b"t\0".as_ptr() as *const c_char } else { b"\0".as_ptr() as *const c_char },
    );

    /* Right align the string if its length is less than the limit */
    if ((pos as usize) + strlen(xs.as_ptr())) < SAMPLE_FLAGS_STR_ALIGNED_SIZE {
        ret = snprintf(
            str_.add(pos as usize),
            size.wrapping_sub(pos as usize),
            b"%*s\0".as_ptr() as *const c_char,
            (SAMPLE_FLAGS_STR_ALIGNED_SIZE as c_int) - ret,
            xs.as_ptr(),
        );
    } else {
        ret = snprintf(
            str_.add(pos as usize),
            size.wrapping_sub(pos as usize),
            b" %s\0".as_ptr() as *const c_char,
            xs.as_ptr(),
        );
    }
    if ret < 0 {
        return ret;
    }

    pos + ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_sample__sprintf_flags(flags: u32, str_: *mut c_char, sz: size_t) -> c_int {
    let chars = PERF_IP_FLAG_CHARS.as_ptr();
    let n: size_t = strlen(PERF_IP_FLAG_CHARS.as_ptr());
    let mut i: size_t = 0;
    let mut pos: size_t = 0;
    let mut flags = flags;
    let mut ret: c_int;

    ret = sample_flags_to_name(flags, str_, sz);
    if ret > 0 {
        return ret;
    }

    while i < n {
        if (flags & 1) != 0 && pos < sz {
            *str_.add(pos) = *chars.add(i);
            pos += 1;
        }
        i += 1;
        flags >>= 1;
    }
    while i < 32 {
        if (flags & 1) != 0 && pos < sz {
            *str_.add(pos) = b'?' as c_char;
            pos += 1;
        }
        i += 1;
        flags >>= 1;
    }
    if pos < sz {
        *str_.add(pos) = 0;
    }

    pos as c_int
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
