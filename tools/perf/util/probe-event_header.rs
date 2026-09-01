/* SPDX-License-Identifier: GPL-2.0 */

// Translated from perf/util/probe-event.h.
// Original C dependencies included linux/compiler.h and stdbool.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub type u64 = u64;
pub type size_t = usize;

#[repr(C)]
pub struct intlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nsinfo {
    _private: [u8; 0],
}

/* Probe related configurations */
#[repr(C)]
pub struct probe_conf {
    pub show_ext_vars: bool,
    pub show_location_range: bool,
    pub force_add: bool,
    pub no_inlines: bool,
    pub cache: bool,
    pub bootconfig: bool,
    pub max_probes: c_int,
    pub magic_num: c_ulong,
}

unsafe extern "C" {
    pub static mut probe_conf: probe_conf;
    pub static mut probe_event_dry_run: bool;
}

pub const DEFAULT_PROBE_MAGIC_NUM: c_ulong = 0xdeade12d; /* u32: 3735937325 */

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

/* kprobe-tracer and uprobe-tracer tracing point */
#[repr(C)]
pub struct probe_trace_point {
    pub realname: *mut c_char,       /* function real name (if needed) */
    pub symbol: *mut c_char,         /* Base symbol */
    pub module: *mut c_char,         /* Module name */
    pub offset: c_ulong,             /* Offset from symbol */
    pub ref_ctr_offset: c_ulong,     /* SDT reference counter offset */
    pub address: u64,                /* Actual address of the trace point */
    pub retprobe: bool,              /* Return probe flag */
}

/* probe-tracer tracing argument referencing offset */
#[repr(C)]
pub struct probe_trace_arg_ref {
    pub next: *mut probe_trace_arg_ref, /* Next reference */
    pub offset: isize,                  /* Offset value */
    pub user_access: bool,              /* User-memory access */
}

/* kprobe-tracer and uprobe-tracer tracing argument */
#[repr(C)]
pub struct probe_trace_arg {
    pub name: *mut c_char,              /* Argument name */
    pub value: *mut c_char,             /* Base value */
    pub type_: *mut c_char,             /* Type name */
    pub ref_: *mut probe_trace_arg_ref, /* Referencing offset */
}

/* kprobe-tracer and uprobe-tracer tracing event (point + arg) */
#[repr(C)]
pub struct probe_trace_event {
    pub event: *mut c_char,           /* Event name */
    pub group: *mut c_char,           /* Group name */
    pub point: probe_trace_point,     /* Trace point */
    pub nargs: c_int,                 /* Number of args */
    pub lang: c_int,                  /* Dwarf language code */
    pub uprobes: bool,                /* uprobes only */
    pub args: *mut probe_trace_arg,   /* Arguments */
}

/* Perf probe probing point */
#[repr(C)]
pub struct perf_probe_point {
    pub file: *mut c_char,          /* File path */
    pub function: *mut c_char,      /* Function name */
    pub line: c_int,                /* Line number */
    pub retprobe: bool,             /* Return probe flag */
    pub lazy_line: *mut c_char,     /* Lazy matching pattern */
    pub offset: c_ulong,            /* Offset from function entry */
    pub abs_address: u64,           /* Absolute address of the point */
}

/* Perf probe probing argument field chain */
#[repr(C)]
pub struct perf_probe_arg_field {
    pub next: *mut perf_probe_arg_field, /* Next field */
    pub name: *mut c_char,               /* Name of the field */
    pub index: isize,                    /* Array index number */
    pub ref_: bool,                      /* Referencing flag */
}

/* Perf probe probing argument */
#[repr(C)]
pub struct perf_probe_arg {
    pub name: *mut c_char,                  /* Argument name */
    pub var: *mut c_char,                   /* Variable name */
    pub type_: *mut c_char,                 /* Type name */
    pub field: *mut perf_probe_arg_field,   /* Structure fields */
    pub user_access: bool,                  /* User-memory access */
}

/* Perf probe probing event (point + arg) */
#[repr(C)]
pub struct perf_probe_event {
    pub event: *mut c_char,              /* Event name */
    pub group: *mut c_char,              /* Group name */
    pub point: perf_probe_point,         /* Probe point */
    pub nargs: c_int,                    /* Number of arguments */
    pub sdt: bool,                       /* SDT/cached event flag */
    pub uprobes: bool,                   /* Uprobe event flag */
    pub target: *mut c_char,             /* Target binary */
    pub args: *mut perf_probe_arg,       /* Arguments */
    pub tevs: *mut probe_trace_event,
    pub ntevs: c_int,
    pub nsi: *mut nsinfo,                /* Target namespace */
}

/* Line range */
#[repr(C)]
pub struct line_range {
    pub file: *mut c_char,          /* File name */
    pub function: *mut c_char,      /* Function name */
    pub start: c_int,               /* Start line number */
    pub end: c_int,                 /* End line number */
    pub offset: c_int,              /* Start line offset */
    pub path: *mut c_char,          /* Real path name */
    pub comp_dir: *mut c_char,      /* Compile directory */
    pub line_list: *mut intlist,    /* Visible lines */
}

#[repr(C)]
pub struct strlist {
    _private: [u8; 0],
}

/* List of variables */
#[repr(C)]
pub struct variable_list {
    pub point: probe_trace_point, /* Actual probepoint */
    pub vars: *mut strlist,       /* Available variables */
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strfilter {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn init_probe_symbol_maps(user_only: bool) -> c_int;
    pub fn exit_probe_symbol_maps();

    /* Command string to events */
    pub fn parse_perf_probe_command(cmd: *const c_char, pev: *mut perf_probe_event) -> c_int;
    pub fn parse_probe_trace_command(cmd: *const c_char, tev: *mut probe_trace_event) -> c_int;

    /* Events to command string */
    pub fn synthesize_perf_probe_command(pev: *mut perf_probe_event) -> *mut c_char;
    pub fn synthesize_probe_trace_command(tev: *mut probe_trace_event) -> *mut c_char;
    pub fn synthesize_perf_probe_arg(pa: *mut perf_probe_arg) -> *mut c_char;

    pub fn perf_probe_event__copy(
        dst: *mut perf_probe_event,
        src: *mut perf_probe_event,
    ) -> c_int;

    pub fn perf_probe_with_var(pev: *mut perf_probe_event) -> bool;

    /* Check the perf_probe_event needs debuginfo */
    pub fn perf_probe_event_need_dwarf(pev: *mut perf_probe_event) -> bool;

    /* Release event contents */
    pub fn clear_perf_probe_event(pev: *mut perf_probe_event);
    pub fn clear_probe_trace_event(tev: *mut probe_trace_event);

    /* Command string to line-range */
    pub fn parse_line_range_desc(cmd: *const c_char, lr: *mut line_range) -> c_int;

    /* Release line range members */
    pub fn line_range__clear(lr: *mut line_range);

    /* Initialize line range */
    pub fn line_range__init(lr: *mut line_range) -> c_int;

    pub fn convert_perf_probe_events(pevs: *mut perf_probe_event, npevs: c_int) -> c_int;
    pub fn apply_perf_probe_events(pevs: *mut perf_probe_event, npevs: c_int) -> c_int;
    pub fn show_probe_trace_events(pevs: *mut perf_probe_event, npevs: c_int) -> c_int;
    pub fn show_bootconfig_events(pevs: *mut perf_probe_event, npevs: c_int) -> c_int;
    pub fn cleanup_perf_probe_events(pevs: *mut perf_probe_event, npevs: c_int);

    pub fn show_perf_probe_event(
        group: *const c_char,
        event: *const c_char,
        pev: *mut perf_probe_event,
        module: *const c_char,
        use_stdout: bool,
    ) -> c_int;
    pub fn show_perf_probe_events(filter: *mut strfilter) -> c_int;
    pub fn show_line_range(
        lr: *mut line_range,
        module: *const c_char,
        nsi: *mut nsinfo,
        user: bool,
    ) -> c_int;
    pub fn show_available_vars(
        pevs: *mut perf_probe_event,
        npevs: c_int,
        filter: *mut strfilter,
    ) -> c_int;
    pub fn show_available_funcs(
        module: *const c_char,
        nsi: *mut nsinfo,
        filter: *mut strfilter,
        user: bool,
    ) -> c_int;
    pub fn arch__fix_tev_from_maps(
        pev: *mut perf_probe_event,
        tev: *mut probe_trace_event,
        map: *mut map,
        sym: *mut symbol,
    );

    /* If there is no space to write, returns -E2BIG. */
    /* C declaration used __printf(3, 4) format checking. */
    pub fn e_snprintf(str_: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;

    pub fn copy_to_probe_trace_arg(
        tvar: *mut probe_trace_arg,
        pvar: *mut perf_probe_arg,
    ) -> c_int;

    pub fn get_target_map(target: *const c_char, nsi: *mut nsinfo, user: bool) -> *mut map;

    pub fn arch__post_process_probe_trace_events(pev: *mut perf_probe_event, ntevs: c_int);
}

/* Maximum index number of event-name postfix */
pub const MAX_EVENT_INDEX: c_int = 1024;


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
