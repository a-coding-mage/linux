// SPDX-License-Identifier: GPL-2.0
/*
 * Builtin evlist command: Show the list of event selectors present
 * in a perf.data file.
 */

// C dependencies translated as external declarations:
// "builtin.h", <linux/list.h>, "util/evlist.h", "util/evsel.h",
// "util/evsel_fprintf.h", "util/parse-events.h",
// <subcmd/parse-options.h>, "util/session.h", "util/data.h",
// "util/debug.h", <linux/err.h>, "util/tool.h", "util/util.h".

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct perf_tool {
    pub attr: Option<unsafe extern "C" fn(*mut perf_tool, *mut perf_event, *mut evlist) -> c_int>,
    pub feature: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
}

#[repr(C)]
pub struct perf_session {
    pub evlist: *mut evlist,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: c_uint,
}

#[repr(C)]
pub union perf_event {
    _bindgen_union_align: u64,
}

#[repr(C)]
pub struct perf_attr_details {
    pub freq: bool,
    pub verbose: bool,
    pub event_group: bool,
    pub force: bool,
    pub trace_fields: bool,
}

#[repr(C)]
pub struct perf_data {
    pub path: *const c_char,
    pub mode: c_int,
    pub force: bool,
    pub is_pipe: bool,
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

pub const PERF_DATA_MODE_READ: c_int = 0;
pub const PERF_TYPE_TRACEPOINT: c_uint = 2;

unsafe extern "C" {
    static mut session_done: c_int;
    static mut input_name: *const c_char;
    static mut stdout: *mut c_void;

    fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool);
    fn perf_event__process_attr(
        tool: *mut perf_tool,
        event: *mut perf_event,
        evlist: *mut evlist,
    ) -> c_int;
    fn perf_session__new(data: *mut perf_data, tool: *mut perf_tool) -> *mut perf_session;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn perf_session__process_events(session: *mut perf_session) -> c_int;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next(evlist: *mut evlist, pos: *mut evsel) -> *mut evsel;
    fn evsel__fprintf(pos: *mut evsel, details: *mut perf_attr_details, fp: *mut c_void) -> c_int;
    fn evsel__is_group_leader(pos: *mut evsel) -> bool;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn perf_session__delete(session: *mut perf_session);
    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option) -> !;
    fn usage_with_options_msg(
        usagestr: *const *const c_char,
        options: *const option,
        fmt: *const c_char,
        ...
    ) -> !;
}

// Direct Rust declarations for subcmd option-construction macros used below.
unsafe extern "C" {
    fn OPT_STRING(
        short_name: c_char,
        long_name: *const c_char,
        value: *mut *const c_char,
        argh: *const c_char,
        help: *const c_char,
    ) -> option;
    fn OPT_BOOLEAN(
        short_name: c_char,
        long_name: *const c_char,
        value: *mut bool,
        help: *const c_char,
    ) -> option;
    fn OPT_END() -> option;
}

unsafe extern "C" fn process_header_feature(
    _tool: *const perf_tool,
    _session: *mut perf_session,
    _event: *mut perf_event,
) -> c_int {
    session_done = 1;
    0
}

unsafe fn __cmd_evlist(file_name: *const c_char, details: *mut perf_attr_details) -> c_int {
    let mut data = perf_data {
        path: file_name,
        mode: PERF_DATA_MODE_READ,
        force: (*details).force,
        is_pipe: false,
    };
    let mut tool = core::mem::MaybeUninit::<perf_tool>::uninit();
    let mut has_tracepoint = false;
    let mut has_group = false;

    perf_tool__init(tool.as_mut_ptr(), false);
    let mut tool = tool.assume_init();

    /* only needed for pipe mode */
    tool.attr = Some(perf_event__process_attr);
    tool.feature = Some(process_header_feature);
    let session = perf_session__new(&mut data, &mut tool);
    if IS_ERR(session as *const c_void) {
        return PTR_ERR(session as *const c_void);
    }

    if data.is_pipe {
        perf_session__process_events(session);
    }

    let mut pos = evlist__first((*session).evlist);
    while !pos.is_null() {
        evsel__fprintf(pos, details, stdout);

        if (*pos).core.attr.type_ == PERF_TYPE_TRACEPOINT {
            has_tracepoint = true;
        }

        if !evsel__is_group_leader(pos) {
            has_group = true;
        }

        pos = evlist__next((*session).evlist, pos);
    }

    if has_tracepoint && !(*details).trace_fields {
        printf(
            b"# Tip: use 'perf evlist --trace-fields' to show fields for tracepoint events\n\0"
                .as_ptr() as *const c_char,
        );
    }

    if has_group && !(*details).event_group {
        printf(
            b"# Tip: use 'perf evlist -g' to show group information\n\0".as_ptr()
                as *const c_char,
        );
    }

    perf_session__delete(session);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_evlist(mut argc: c_int, argv: *const *const c_char) -> c_int {
    let mut details = perf_attr_details {
        freq: false,
        verbose: false,
        event_group: false,
        force: false,
        trace_fields: false,
    };

    let options = [
        OPT_STRING(
            b'i' as c_char,
            b"input\0".as_ptr() as *const c_char,
            &raw mut input_name,
            b"file\0".as_ptr() as *const c_char,
            b"Input file name\0".as_ptr() as *const c_char,
        ),
        OPT_BOOLEAN(
            b'F' as c_char,
            b"freq\0".as_ptr() as *const c_char,
            &mut details.freq,
            b"Show the sample frequency\0".as_ptr() as *const c_char,
        ),
        OPT_BOOLEAN(
            b'v' as c_char,
            b"verbose\0".as_ptr() as *const c_char,
            &mut details.verbose,
            b"Show all event attr details\0".as_ptr() as *const c_char,
        ),
        OPT_BOOLEAN(
            b'g' as c_char,
            b"group\0".as_ptr() as *const c_char,
            &mut details.event_group,
            b"Show event group information\0".as_ptr() as *const c_char,
        ),
        OPT_BOOLEAN(
            b'f' as c_char,
            b"force\0".as_ptr() as *const c_char,
            &mut details.force,
            b"don't complain, do it\0".as_ptr() as *const c_char,
        ),
        OPT_BOOLEAN(
            0,
            b"trace-fields\0".as_ptr() as *const c_char,
            &mut details.trace_fields,
            b"Show tracepoint fields\0".as_ptr() as *const c_char,
        ),
        OPT_END(),
    ];
    let evlist_usage = [
        b"perf evlist [<options>]\0".as_ptr() as *const c_char,
        core::ptr::null(),
    ];

    argc = parse_options(argc, argv, options.as_ptr(), evlist_usage.as_ptr(), 0);
    if argc != 0 {
        usage_with_options(evlist_usage.as_ptr(), options.as_ptr());
    }

    if details.event_group && (details.verbose || details.freq) {
        usage_with_options_msg(
            evlist_usage.as_ptr(),
            options.as_ptr(),
            b"--group option is not compatible with other options\n\0".as_ptr() as *const c_char,
        );
    }

    __cmd_evlist(input_name, &mut details)
}
