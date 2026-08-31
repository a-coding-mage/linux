// SPDX-License-Identifier: GPL-2.0
/*
 * builtin-list.c
 *
 * Builtin list command: list all event types
 *
 * Copyright (C) 2009, Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
 * Copyright (C) 2008-2009, Red Hat Inc, Ingo Molnar <mingo@redhat.com>
 * Copyright (C) 2011, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type FILE = c_void;
type Bool = bool;
type U32 = u32;

const PERF_TYPE_RAW: U32 = 4;
const PERF_TYPE_MAX: U32 = 6;
const PARSE_OPT_HIDDEN: c_int = 1;
const PARSE_OPT_STOP_AT_NON_OPTION: c_int = 2;

#[repr(C)]
pub struct strlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strbuf {
    pub buf: *mut c_char,
}

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct print_callbacks {
    pub print_start: Option<unsafe extern "C" fn(*mut c_void)>,
    pub print_end: Option<unsafe extern "C" fn(*mut c_void)>,
    pub print_event: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const c_char,
            *const c_char,
            U32,
            *const c_char,
            *const c_char,
            *const c_char,
            Bool,
            *const c_char,
            *const c_char,
            *const c_char,
            *const c_char,
        ),
    >,
    pub print_metric: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const c_char,
            *const c_char,
            *const c_char,
            *const c_char,
            *const c_char,
            *const c_char,
            *const c_char,
            *const c_char,
        ),
    >,
    pub skip_duplicate_pmus: Option<unsafe extern "C" fn(*mut c_void) -> Bool>,
}

/*
 * struct print_state - State and configuration passed to the default_print
 * functions.
 */
#[repr(C)]
pub struct print_state {
    /** @fp: File to write output to. */
    pub fp: *mut FILE,
    /**
     * @pmu_glob: Optionally restrict PMU and metric matching to PMU or
     * debugfs subsystem name.
     */
    pub pmu_glob: *mut c_char,
    /** @event_glob: Optional pattern matching glob. */
    pub event_glob: *mut c_char,
    /** @name_only: Print event or metric names only. */
    pub name_only: Bool,
    /** @desc: Print the event or metric description. */
    pub desc: Bool,
    /** @long_desc: Print longer event or metric description. */
    pub long_desc: Bool,
    /** @deprecated: Print deprecated events or metrics. */
    pub deprecated: Bool,
    /**
     * @detailed: Print extra information on the perf event such as names
     * and expressions used internally by events.
     */
    pub detailed: Bool,
    /** @metrics: Controls printing of metric and metric groups. */
    pub metrics: Bool,
    /** @metricgroups: Controls printing of metric and metric groups. */
    pub metricgroups: Bool,
    /** @exclude_abi: Exclude PMUs with types less than PERF_TYPE_MAX except PERF_TYPE_RAW. */
    pub exclude_abi: Bool,
    /** @last_topic: The last printed event topic. */
    pub last_topic: *mut c_char,
    /** @last_metricgroups: The last printed metric group. */
    pub last_metricgroups: *mut c_char,
    /** @visited_metrics: Metrics that are printed to avoid duplicates. */
    pub visited_metrics: *mut strlist,
}

#[repr(C)]
pub struct json_print_state {
    /** The shared print_state */
    pub common: print_state,
    /** Should a separator be printed prior to the next item? */
    pub need_sep: Bool,
}

enum FixArg {
    Raw(*const c_char),
    Escaped(*const c_char),
}

unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut verbose: c_int;

    fn pager_in_use() -> Bool;
    fn pager_get_columns() -> c_int;
    fn setup_pager();
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcspn(s: *const c_char, reject: *const c_char) -> usize;
    fn isspace(c: c_int) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn zfree(ptr: *mut *mut c_char);
    fn strglobmatch(str_: *const c_char, pat: *const c_char) -> Bool;
    fn strglobmatch_nocase(str_: *const c_char, pat: *const c_char) -> Bool;
    fn describe_metricgroup(group: *const c_char) -> *const c_char;
    fn strlist__has_entry(slist: *mut strlist, entry: *const c_char) -> Bool;
    fn strlist__add(slist: *mut strlist, entry: *const c_char) -> c_int;
    fn strlist__new(slist: *const c_char, dupstr: *const c_void) -> *mut strlist;
    fn strlist__delete(slist: *mut strlist);
    fn strbuf_init(sb: *mut strbuf, hint: usize);
    fn strbuf_setlen(sb: *mut strbuf, len: usize);
    fn strbuf_addstr(sb: *mut strbuf, s: *const c_char);
    fn strbuf_addch(sb: *mut strbuf, c: c_int);
    fn strbuf_release(sb: *mut strbuf);
    fn pr_err(fmt: *const c_char, ...);
    fn parse_options(
        argc: c_int,
        argv: *mut *const c_char,
        options: *mut option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn set_option_flag(options: *mut option, short_name: c_int, long_name: *const c_char, flag: c_int);
    fn perf_pmus__pmu_for_pmu_filter(name: *const c_char) -> *const perf_pmu;
    fn print_events(print_cb: *const print_callbacks, ps: *mut c_void);
    fn perf_pmus__print_pmu_events(print_cb: *const print_callbacks, ps: *mut c_void);
    fn print_sdt_events(print_cb: *const print_callbacks, ps: *mut c_void);
    fn metricgroup__print(print_cb: *const print_callbacks, ps: *mut c_void);
    fn print_libpfm_events(print_cb: *const print_callbacks, ps: *mut c_void);
}

unsafe fn cstr_or_empty(s: *const c_char) -> *const c_char {
    if s.is_null() { c"".as_ptr() } else { s }
}

unsafe extern "C" fn default_print_start(ps: *mut c_void) {
    let print_state = ps as *mut print_state;

    if !(*print_state).name_only && pager_in_use() {
        fprintf(
            (*print_state).fp,
            c"\nList of pre-defined events (to be used in -e or -M):\n\n".as_ptr(),
        );
    }
}

unsafe extern "C" fn default_print_end(_print_state: *mut c_void) {}

unsafe fn skip_spaces_or_commas(mut str_: *const c_char) -> *const c_char {
    while isspace(*str_ as c_int) != 0 || *str_ == b',' as c_char {
        str_ = str_.add(1);
    }
    str_
}

unsafe fn wordwrap(fp: *mut FILE, mut s: *const c_char, start: c_int, max: c_int, corr: c_int) {
    let mut column = start;
    let mut saw_newline = false;
    let mut comma = false;

    while *s != 0 {
        let wlen = strcspn(s, c" ,\t\n".as_ptr());
        let mut sep = if comma { c",".as_ptr() } else { c" ".as_ptr() };

        if (column + wlen as c_int >= max && column > start) || saw_newline {
            fprintf(
                fp,
                if comma { c",\n%*s".as_ptr() } else { c"\n%*s".as_ptr() },
                start,
                c"".as_ptr(),
            );
            column = start + corr;
        }
        if column <= start {
            sep = c"".as_ptr();
        }
        let n = fprintf(fp, c"%s%.*s".as_ptr(), sep, wlen as c_int, s);
        if n <= 0 {
            break;
        }
        saw_newline = *s.add(wlen) == b'\n' as c_char;
        s = s.add(wlen);
        comma = *s == b',' as c_char;
        column += n;
        s = skip_spaces_or_commas(s);
    }
}

unsafe extern "C" fn default_print_event(
    ps: *mut c_void,
    topic: *const c_char,
    pmu_name: *const c_char,
    pmu_type: U32,
    event_name: *const c_char,
    event_alias: *const c_char,
    _scale_unit: *const c_char,
    deprecated: Bool,
    event_type_desc: *const c_char,
    mut desc: *const c_char,
    long_desc: *const c_char,
    encoding_desc: *const c_char,
) {
    let print_state = ps as *mut print_state;
    let fp = (*print_state).fp;
    let mut pos: c_int;

    if deprecated && !(*print_state).deprecated {
        return;
    }
    if !(*print_state).pmu_glob.is_null()
        && (pmu_name.is_null() || !strglobmatch(pmu_name, (*print_state).pmu_glob))
    {
        return;
    }
    if (*print_state).exclude_abi && pmu_type < PERF_TYPE_MAX && pmu_type != PERF_TYPE_RAW {
        return;
    }
    if !(*print_state).event_glob.is_null()
        && (event_name.is_null() || !strglobmatch(event_name, (*print_state).event_glob))
        && (event_alias.is_null() || !strglobmatch(event_alias, (*print_state).event_glob))
        && (topic.is_null() || !strglobmatch_nocase(topic, (*print_state).event_glob))
    {
        return;
    }
    if (*print_state).name_only {
        if !event_alias.is_null() && strlen(event_alias) != 0 {
            fprintf(fp, c"%s ".as_ptr(), event_alias);
        } else {
            fprintf(fp, c"%s ".as_ptr(), event_name);
        }
        return;
    }
    if strcmp((*print_state).last_topic, cstr_or_empty(topic)) != 0 {
        if !topic.is_null() {
            fprintf(fp, c"\n%s:\n".as_ptr(), topic);
        }
        zfree(&mut (*print_state).last_topic);
        (*print_state).last_topic = strdup(cstr_or_empty(topic));
    }
    if !event_alias.is_null() && strlen(event_alias) != 0 {
        pos = fprintf(fp, c"  %s OR %s".as_ptr(), event_name, event_alias);
    } else {
        pos = fprintf(fp, c"  %s".as_ptr(), event_name);
    }
    if topic.is_null() && !event_type_desc.is_null() {
        while pos < 53 {
            fputc(b' ' as c_int, fp);
            pos += 1;
        }
        fprintf(fp, c"[%s]\n".as_ptr(), event_type_desc);
    } else {
        fputc(b'\n' as c_int, fp);
    }
    if !long_desc.is_null() && (*print_state).long_desc {
        desc = long_desc;
    }
    if !desc.is_null() && ((*print_state).desc || (*print_state).long_desc) {
        let mut desc_with_unit: *mut c_char = ptr::null_mut();
        let mut desc_len: c_int = -1;

        if !pmu_name.is_null() && strcmp(pmu_name, c"default_core".as_ptr()) != 0 {
            desc_len = strlen(desc) as c_int;
            desc_len = asprintf(
                &mut desc_with_unit,
                if desc_len > 0 && *desc.add((desc_len - 1) as usize) != b'.' as c_char {
                    c"%s. Unit: %s".as_ptr()
                } else {
                    c"%s Unit: %s".as_ptr()
                },
                desc,
                pmu_name,
            );
        }
        fprintf(fp, c"%*s".as_ptr(), 8, c"[".as_ptr());
        wordwrap(
            fp,
            if desc_len > 0 { desc_with_unit } else { desc as *mut c_char },
            8,
            pager_get_columns(),
            0,
        );
        fprintf(fp, c"]\n".as_ptr());
        free(desc_with_unit as *mut c_void);
    }
    if (*print_state).detailed && !encoding_desc.is_null() {
        fprintf(fp, c"%*s".as_ptr(), 8, c"".as_ptr());
        wordwrap(fp, encoding_desc, 8, pager_get_columns(), 0);
        fputc(b'\n' as c_int, fp);
    }
}

unsafe extern "C" fn default_print_metric(
    ps: *mut c_void,
    group: *const c_char,
    name: *const c_char,
    desc: *const c_char,
    long_desc: *const c_char,
    expr: *const c_char,
    threshold: *const c_char,
    _unit: *const c_char,
    _pmu_name: *const c_char,
) {
    let print_state = ps as *mut print_state;
    let fp = (*print_state).fp;

    if !(*print_state).event_glob.is_null()
        && (!(*print_state).metrics || name.is_null() || !strglobmatch(name, (*print_state).event_glob))
        && (!(*print_state).metricgroups || group.is_null() || !strglobmatch(group, (*print_state).event_glob))
    {
        return;
    }
    if !(*print_state).name_only && (*print_state).last_metricgroups.is_null() {
        if (*print_state).metricgroups {
            fprintf(fp, c"\nMetric Groups:\n".as_ptr());
            if !(*print_state).metrics {
                fputc(b'\n' as c_int, fp);
            }
        } else {
            fprintf(fp, c"\nMetrics:\n\n".as_ptr());
        }
    }
    if (*print_state).last_metricgroups.is_null()
        || strcmp((*print_state).last_metricgroups, cstr_or_empty(group)) != 0
    {
        if !group.is_null() && (*print_state).metricgroups {
            if (*print_state).name_only {
                fprintf(fp, c"%s ".as_ptr(), group);
            } else {
                let gdesc = if (*print_state).desc { describe_metricgroup(group) } else { ptr::null() };
                let mut print_colon = c"".as_ptr();

                if (*print_state).metrics {
                    print_colon = c":".as_ptr();
                    fputc(b'\n' as c_int, fp);
                }
                if !gdesc.is_null() {
                    fprintf(fp, c"%s%s [%s]\n".as_ptr(), group, print_colon, gdesc);
                } else {
                    fprintf(fp, c"%s%s\n".as_ptr(), group, print_colon);
                }
            }
        }
        zfree(&mut (*print_state).last_metricgroups);
        (*print_state).last_metricgroups = strdup(cstr_or_empty(group));
    }
    if !(*print_state).metrics {
        return;
    }
    if (*print_state).name_only {
        if (*print_state).metrics && !strlist__has_entry((*print_state).visited_metrics, name) {
            fprintf(fp, c"%s ".as_ptr(), name);
            strlist__add((*print_state).visited_metrics, name);
        }
        return;
    }
    fprintf(fp, c"  %s\n".as_ptr(), name);
    if !long_desc.is_null() && (*print_state).long_desc {
        fprintf(fp, c"%*s".as_ptr(), 8, c"[".as_ptr());
        wordwrap(fp, long_desc, 8, pager_get_columns(), 0);
        fprintf(fp, c"]\n".as_ptr());
    } else if !desc.is_null() && (*print_state).desc {
        fprintf(fp, c"%*s".as_ptr(), 8, c"[".as_ptr());
        wordwrap(fp, desc, 8, pager_get_columns(), 0);
        fprintf(fp, c"]\n".as_ptr());
    }
    if !expr.is_null() && (*print_state).detailed {
        fprintf(fp, c"%*s".as_ptr(), 8, c"[".as_ptr());
        wordwrap(fp, expr, 8, pager_get_columns(), 0);
        fprintf(fp, c"]\n".as_ptr());
    }
    if !threshold.is_null() && (*print_state).detailed {
        fprintf(fp, c"%*s".as_ptr(), 8, c"[".as_ptr());
        wordwrap(fp, threshold, 8, pager_get_columns(), 0);
        fprintf(fp, c"]\n".as_ptr());
    }
}

unsafe extern "C" fn json_print_start(ps: *mut c_void) {
    let print_state = ps as *mut json_print_state;
    let fp = (*print_state).common.fp;

    fprintf(fp, c"[\n".as_ptr());
}

unsafe extern "C" fn json_print_end(ps: *mut c_void) {
    let print_state = ps as *mut json_print_state;
    let fp = (*print_state).common.fp;

    fprintf(fp, c"%s]\n".as_ptr(), if (*print_state).need_sep { c"\n".as_ptr() } else { c"".as_ptr() });
}

unsafe fn fix_escape_fprintf(fp: *mut FILE, buf: *mut strbuf, fmt: *const c_char, args: &[FixArg]) {
    strbuf_setlen(buf, 0);
    let mut arg_pos = 0usize;
    let mut fmt_pos = 0usize;
    while fmt_pos < strlen(fmt) {
        let ch = *fmt.add(fmt_pos);
        match ch as u8 {
            b'%' => {
                fmt_pos += 1;
                match *fmt.add(fmt_pos) as u8 {
                    b's' => {
                        if let FixArg::Raw(s) = args[arg_pos] {
                            strbuf_addstr(buf, s);
                        }
                        arg_pos += 1;
                    }
                    b'S' => {
                        let s = match args[arg_pos] {
                            FixArg::Raw(s) | FixArg::Escaped(s) => s,
                        };
                        arg_pos += 1;
                        let mut s_pos = 0usize;
                        while s_pos < strlen(s) {
                            match *s.add(s_pos) as u8 {
                                b'\n' => strbuf_addstr(buf, c"\\n".as_ptr()),
                                b'\r' => strbuf_addstr(buf, c"\\r".as_ptr()),
                                b'\\' | b'"' => {
                                    strbuf_addch(buf, b'\\' as c_int);
                                    strbuf_addch(buf, *s.add(s_pos) as c_int);
                                }
                                _ => strbuf_addch(buf, *s.add(s_pos) as c_int),
                            }
                            s_pos += 1;
                        }
                    }
                    other => {
                        pr_err(c"Unexpected format character '%c'\n".as_ptr(), other as c_int);
                        strbuf_addch(buf, b'%' as c_int);
                        strbuf_addch(buf, other as c_int);
                    }
                }
            }
            _ => strbuf_addch(buf, ch as c_int),
        }
        fmt_pos += 1;
    }
    fputs((*buf).buf, fp);
}

unsafe extern "C" fn json_print_event(
    ps: *mut c_void,
    topic: *const c_char,
    pmu_name: *const c_char,
    pmu_type: U32,
    event_name: *const c_char,
    event_alias: *const c_char,
    scale_unit: *const c_char,
    deprecated: Bool,
    event_type_desc: *const c_char,
    desc: *const c_char,
    long_desc: *const c_char,
    encoding_desc: *const c_char,
) {
    let print_state = ps as *mut json_print_state;
    let mut need_sep = false;
    let fp = (*print_state).common.fp;
    let mut buf: strbuf = mem::zeroed();

    if deprecated && !(*print_state).common.deprecated {
        return;
    }
    if !(*print_state).common.pmu_glob.is_null()
        && (pmu_name.is_null() || !strglobmatch(pmu_name, (*print_state).common.pmu_glob))
    {
        return;
    }
    if (*print_state).common.exclude_abi && pmu_type < PERF_TYPE_MAX && pmu_type != PERF_TYPE_RAW {
        return;
    }
    if !(*print_state).common.event_glob.is_null()
        && (event_name.is_null() || !strglobmatch(event_name, (*print_state).common.event_glob))
        && (event_alias.is_null() || !strglobmatch(event_alias, (*print_state).common.event_glob))
        && (topic.is_null() || !strglobmatch_nocase(topic, (*print_state).common.event_glob))
    {
        return;
    }

    strbuf_init(&mut buf, 0);
    fprintf(fp, c"%s{\n".as_ptr(), if (*print_state).need_sep { c",\n".as_ptr() } else { c"".as_ptr() });
    (*print_state).need_sep = true;
    if !pmu_name.is_null() {
        fix_escape_fprintf(fp, &mut buf, c"\t\"Unit\": \"%S\"".as_ptr(), &[FixArg::Escaped(pmu_name)]);
        need_sep = true;
    }
    if !topic.is_null() {
        fix_escape_fprintf(fp, &mut buf, c"%s\t\"Topic\": \"%S\"".as_ptr(), &[FixArg::Raw(if need_sep { c",\n".as_ptr() } else { c"".as_ptr() }), FixArg::Escaped(topic)]);
        need_sep = true;
    }
    if !event_name.is_null() {
        fix_escape_fprintf(fp, &mut buf, c"%s\t\"EventName\": \"%S\"".as_ptr(), &[FixArg::Raw(if need_sep { c",\n".as_ptr() } else { c"".as_ptr() }), FixArg::Escaped(event_name)]);
        need_sep = true;
    }
    if !event_alias.is_null() && strlen(event_alias) != 0 {
        fix_escape_fprintf(fp, &mut buf, c"%s\t\"EventAlias\": \"%S\"".as_ptr(), &[FixArg::Raw(if need_sep { c",\n".as_ptr() } else { c"".as_ptr() }), FixArg::Escaped(event_alias)]);
        need_sep = true;
    }
    if !scale_unit.is_null() && strlen(scale_unit) != 0 {
        fix_escape_fprintf(fp, &mut buf, c"%s\t\"ScaleUnit\": \"%S\"".as_ptr(), &[FixArg::Raw(if need_sep { c",\n".as_ptr() } else { c"".as_ptr() }), FixArg::Escaped(scale_unit)]);
        need_sep = true;
    }
    if !event_type_desc.is_null() {
        fix_escape_fprintf(fp, &mut buf, c"%s\t\"EventType\": \"%S\"".as_ptr(), &[FixArg::Raw(if need_sep { c",\n".as_ptr() } else { c"".as_ptr() }), FixArg::Escaped(event_type_desc)]);
        need_sep = true;
    }
    if deprecated {
        fix_escape_fprintf(fp, &mut buf, c"%s\t\"Deprecated\": \"%S\"".as_ptr(), &[FixArg::Raw(if need_sep { c",\n".as_ptr() } else { c"".as_ptr() }), FixArg::Escaped(if deprecated { c"1".as_ptr() } else { c"0".as_ptr() })]);
        need_sep = true;
    }
    if !desc.is_null() {
        fix_escape_fprintf(fp, &mut buf, c"%s\t\"BriefDescription\": \"%S\"".as_ptr(), &[FixArg::Raw(if need_sep { c",\n".as_ptr() } else { c"".as_ptr() }), FixArg::Escaped(desc)]);
        need_sep = true;
    }
    if !long_desc.is_null() {
        fix_escape_fprintf(fp, &mut buf, c"%s\t\"PublicDescription\": \"%S\"".as_ptr(), &[FixArg::Raw(if need_sep { c",\n".as_ptr() } else { c"".as_ptr() }), FixArg::Escaped(long_desc)]);
        need_sep = true;
    }
    if !encoding_desc.is_null() {
        fix_escape_fprintf(fp, &mut buf, c"%s\t\"Encoding\": \"%S\"".as_ptr(), &[FixArg::Raw(if need_sep { c",\n".as_ptr() } else { c"".as_ptr() }), FixArg::Escaped(encoding_desc)]);
        need_sep = true;
    }
    fprintf(fp, c"%s}".as_ptr(), if need_sep { c"\n".as_ptr() } else { c"".as_ptr() });
    strbuf_release(&mut buf);
}

unsafe extern "C" fn json_print_metric(
    ps: *mut c_void,
    group: *const c_char,
    name: *const c_char,
    desc: *const c_char,
    long_desc: *const c_char,
    expr: *const c_char,
    threshold: *const c_char,
    unit: *const c_char,
    pmu_name: *const c_char,
) {
    let print_state = ps as *mut json_print_state;
    let mut need_sep = false;
    let fp = (*print_state).common.fp;
    let mut buf: strbuf = mem::zeroed();

    if !(*print_state).common.event_glob.is_null()
        && (!(*print_state).common.metrics || name.is_null() || !strglobmatch(name, (*print_state).common.event_glob))
        && (!(*print_state).common.metricgroups || group.is_null() || !strglobmatch(group, (*print_state).common.event_glob))
    {
        return;
    }

    strbuf_init(&mut buf, 0);
    fprintf(fp, c"%s{\n".as_ptr(), if (*print_state).need_sep { c",\n".as_ptr() } else { c"".as_ptr() });
    (*print_state).need_sep = true;
    if !group.is_null() {
        fix_escape_fprintf(fp, &mut buf, c"\t\"MetricGroup\": \"%S\"".as_ptr(), &[FixArg::Escaped(group)]);
        need_sep = true;
    }
    if !name.is_null() {
        fix_escape_fprintf(fp, &mut buf, c"%s\t\"MetricName\": \"%S\"".as_ptr(), &[FixArg::Raw(if need_sep { c",\n".as_ptr() } else { c"".as_ptr() }), FixArg::Escaped(name)]);
        need_sep = true;
    }
    if !expr.is_null() {
        fix_escape_fprintf(fp, &mut buf, c"%s\t\"MetricExpr\": \"%S\"".as_ptr(), &[FixArg::Raw(if need_sep { c",\n".as_ptr() } else { c"".as_ptr() }), FixArg::Escaped(expr)]);
        need_sep = true;
    }
    if !threshold.is_null() {
        fix_escape_fprintf(fp, &mut buf, c"%s\t\"MetricThreshold\": \"%S\"".as_ptr(), &[FixArg::Raw(if need_sep { c",\n".as_ptr() } else { c"".as_ptr() }), FixArg::Escaped(threshold)]);
        need_sep = true;
    }
    if !unit.is_null() {
        fix_escape_fprintf(fp, &mut buf, c"%s\t\"ScaleUnit\": \"%S\"".as_ptr(), &[FixArg::Raw(if need_sep { c",\n".as_ptr() } else { c"".as_ptr() }), FixArg::Escaped(unit)]);
        need_sep = true;
    }
    if !desc.is_null() {
        fix_escape_fprintf(fp, &mut buf, c"%s\t\"BriefDescription\": \"%S\"".as_ptr(), &[FixArg::Raw(if need_sep { c",\n".as_ptr() } else { c"".as_ptr() }), FixArg::Escaped(desc)]);
        need_sep = true;
    }
    if !long_desc.is_null() {
        fix_escape_fprintf(fp, &mut buf, c"%s\t\"PublicDescription\": \"%S\"".as_ptr(), &[FixArg::Raw(if need_sep { c",\n".as_ptr() } else { c"".as_ptr() }), FixArg::Escaped(long_desc)]);
        need_sep = true;
    }
    if !pmu_name.is_null() {
        fix_escape_fprintf(fp, &mut buf, c"%s\t\"Unit\": \"%S\"".as_ptr(), &[FixArg::Raw(if need_sep { c",\n".as_ptr() } else { c"".as_ptr() }), FixArg::Escaped(pmu_name)]);
        need_sep = true;
    }
    fprintf(fp, c"%s}".as_ptr(), if need_sep { c"\n".as_ptr() } else { c"".as_ptr() });
    strbuf_release(&mut buf);
}

unsafe extern "C" fn json_skip_duplicate_pmus(_ps: *mut c_void) -> Bool {
    false
}

unsafe extern "C" fn default_skip_duplicate_pmus(ps: *mut c_void) -> Bool {
    let print_state = ps as *mut print_state;

    !(*print_state).long_desc
}

// Option construction macros such as OPT_BOOLEAN/OPT_STRING/OPT_INCR/OPT_END
// are provided by parse-options.h in C; their Rust equivalents are external to
// this isolated translation.
unsafe fn opt_boolean(_: c_int, _: *const c_char, _: *mut Bool, _: *const c_char) -> option { mem::zeroed() }
unsafe fn opt_string(_: c_int, _: *const c_char, _: *mut *const c_char, _: *const c_char, _: *const c_char) -> option { mem::zeroed() }
unsafe fn opt_incr(_: c_int, _: *const c_char, _: *mut c_int, _: *const c_char) -> option { mem::zeroed() }
unsafe fn opt_end() -> option { mem::zeroed() }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_list(mut argc: c_int, argv: *mut *const c_char) -> c_int {
    let mut i: c_int;
    let mut ret: c_int = 0;
    let mut default_ps: print_state = mem::zeroed();
    default_ps.fp = stdout;
    default_ps.desc = true;
    let mut json_ps: json_print_state = mem::zeroed();
    json_ps.common.fp = stdout;
    let mut ps: *mut print_state = &mut default_ps;
    let mut print_cb = print_callbacks {
        print_start: Some(default_print_start),
        print_end: Some(default_print_end),
        print_event: Some(default_print_event),
        print_metric: Some(default_print_metric),
        skip_duplicate_pmus: Some(default_skip_duplicate_pmus),
    };
    let mut cputype: *const c_char = ptr::null();
    let mut unit_name: *const c_char = ptr::null();
    let mut output_path: *const c_char = ptr::null();
    let mut json = false;
    let mut list_options = [
        opt_boolean(0, c"raw-dump".as_ptr(), &mut default_ps.name_only, c"Dump raw events".as_ptr()),
        opt_boolean(b'j' as c_int, c"json".as_ptr(), &mut json, c"JSON encode events and metrics".as_ptr()),
        opt_boolean(b'd' as c_int, c"desc".as_ptr(), &mut default_ps.desc, c"Print extra event descriptions. --no-desc to not print.".as_ptr()),
        opt_boolean(b'v' as c_int, c"long-desc".as_ptr(), &mut default_ps.long_desc, c"Print longer event descriptions and all similar PMUs with alphanumeric suffixes.".as_ptr()),
        opt_boolean(0, c"details".as_ptr(), &mut default_ps.detailed, c"Print information on the perf event names and expressions used internally by events.".as_ptr()),
        opt_string(b'o' as c_int, c"output".as_ptr(), &mut output_path, c"file".as_ptr(), c"output file name".as_ptr()),
        opt_boolean(0, c"deprecated".as_ptr(), &mut default_ps.deprecated, c"Print deprecated events.".as_ptr()),
        opt_string(0, c"cputype".as_ptr(), &mut cputype, c"cpu type".as_ptr(), c"Limit PMU or metric printing to the given PMU (e.g. cpu, core or atom).".as_ptr()),
        opt_string(0, c"unit".as_ptr(), &mut unit_name, c"PMU name".as_ptr(), c"Limit PMU or metric printing to the specified PMU.".as_ptr()),
        opt_incr(0, c"debug".as_ptr(), &mut verbose, c"Enable debugging output".as_ptr()),
        opt_end(),
    ];
    /* HAVE_LIBPFM adds "|pfm" to this usage string in the C source. */
    let list_usage = [
        c"perf list [<options>] [hw|sw|cache|tracepoint|pmu|sdt|metric|metricgroup|event_glob]".as_ptr(),
        ptr::null(),
    ];

    set_option_flag(list_options.as_mut_ptr(), 0, c"raw-dump".as_ptr(), PARSE_OPT_HIDDEN);
    /* Hide hybrid flag for the more generic 'unit' flag. */
    set_option_flag(list_options.as_mut_ptr(), 0, c"cputype".as_ptr(), PARSE_OPT_HIDDEN);

    argc = parse_options(argc, argv, list_options.as_mut_ptr(), list_usage.as_ptr(), PARSE_OPT_STOP_AT_NON_OPTION);

    if json {
        ps = &mut json_ps.common;
    }
    if !output_path.is_null() {
        (*ps).fp = fopen(output_path, c"w".as_ptr());
    }

    setup_pager();
    if !default_ps.name_only {
        setup_pager();
    }

    if json {
        print_cb = print_callbacks {
            print_start: Some(json_print_start),
            print_end: Some(json_print_end),
            print_event: Some(json_print_event),
            print_metric: Some(json_print_metric),
            skip_duplicate_pmus: Some(json_skip_duplicate_pmus),
        };
    } else {
        (*ps).last_topic = strdup(c"".as_ptr());
        assert!(!(*ps).last_topic.is_null());
        (*ps).visited_metrics = strlist__new(ptr::null(), ptr::null());
        assert!(!(*ps).visited_metrics.is_null());
        if !unit_name.is_null() {
            (*ps).pmu_glob = strdup(unit_name);
        } else if !cputype.is_null() {
            let pmu = perf_pmus__pmu_for_pmu_filter(cputype);

            if pmu.is_null() {
                pr_err(c"ERROR: cputype is not supported!\n".as_ptr());
                ret = -1;
                goto_out(&mut print_cb, ps, output_path);
                return ret;
            }
            (*ps).pmu_glob = strdup((*pmu).name);
        }
    }
    (print_cb.print_start.unwrap())(ps as *mut c_void);

    if argc == 0 {
        if unit_name.is_null() {
            (*ps).metrics = true;
            (*ps).metricgroups = true;
        }
        print_events(&print_cb, ps as *mut c_void);
        goto_out(&mut print_cb, ps, output_path);
        return ret;
    }

    i = 0;
    while i < argc {
        let mut s: *mut c_char = ptr::null_mut();
        let arg = *argv.add(i as usize);

        if strcmp(arg, c"tracepoint".as_ptr()) == 0 {
            let old_pmu_glob = default_ps.pmu_glob;

            default_ps.pmu_glob = strdup(c"tracepoint".as_ptr());
            if default_ps.pmu_glob.is_null() {
                ret = -1;
                break;
            }
            perf_pmus__print_pmu_events(&print_cb, ps as *mut c_void);
            zfree(&mut default_ps.pmu_glob);
            default_ps.pmu_glob = old_pmu_glob;
        } else if strcmp(arg, c"hw".as_ptr()) == 0 || strcmp(arg, c"hardware".as_ptr()) == 0 {
            let old_event_glob = (*ps).event_glob;

            (*ps).event_glob = strdup(c"legacy hardware".as_ptr());
            if (*ps).event_glob.is_null() {
                ret = -1;
                break;
            }
            perf_pmus__print_pmu_events(&print_cb, ps as *mut c_void);
            zfree(&mut (*ps).event_glob);
            (*ps).event_glob = old_event_glob;
        } else if strcmp(arg, c"sw".as_ptr()) == 0 || strcmp(arg, c"software".as_ptr()) == 0 {
            let old_pmu_glob = (*ps).pmu_glob;
            let sw_globs = [c"software".as_ptr(), c"tool".as_ptr()];

            let mut j = 0usize;
            while j < sw_globs.len() {
                (*ps).pmu_glob = strdup(sw_globs[j]);
                if (*ps).pmu_glob.is_null() {
                    ret = -1;
                    break;
                }
                perf_pmus__print_pmu_events(&print_cb, ps as *mut c_void);
                zfree(&mut (*ps).pmu_glob);
                j += 1;
            }
            if ret != 0 {
                break;
            }
            (*ps).pmu_glob = old_pmu_glob;
        } else if strcmp(arg, c"cache".as_ptr()) == 0 || strcmp(arg, c"hwcache".as_ptr()) == 0 {
            let old_event_glob = (*ps).event_glob;

            (*ps).event_glob = strdup(c"legacy cache".as_ptr());
            if (*ps).event_glob.is_null() {
                ret = -1;
                break;
            }
            perf_pmus__print_pmu_events(&print_cb, ps as *mut c_void);
            zfree(&mut (*ps).event_glob);
            (*ps).event_glob = old_event_glob;
        } else if strcmp(arg, c"pmu".as_ptr()) == 0 {
            (*ps).exclude_abi = true;
            perf_pmus__print_pmu_events(&print_cb, ps as *mut c_void);
            (*ps).exclude_abi = false;
        } else if strcmp(arg, c"sdt".as_ptr()) == 0 {
            print_sdt_events(&print_cb, ps as *mut c_void);
        } else if strcmp(arg, c"metric".as_ptr()) == 0 || strcmp(arg, c"metrics".as_ptr()) == 0 {
            (*ps).metricgroups = false;
            (*ps).metrics = true;
            metricgroup__print(&print_cb, ps as *mut c_void);
        } else if strcmp(arg, c"metricgroup".as_ptr()) == 0 || strcmp(arg, c"metricgroups".as_ptr()) == 0 {
            (*ps).metricgroups = true;
            (*ps).metrics = false;
            metricgroup__print(&print_cb, ps as *mut c_void);
        } else if strcmp(arg, c"pfm".as_ptr()) == 0 {
            /* Translated from the HAVE_LIBPFM conditional branch. */
            print_libpfm_events(&print_cb, ps as *mut c_void);
        } else if !strchr(arg, b':' as c_int).is_null() {
            let old_pmu_glob = (*ps).pmu_glob;
            let old_event_glob = (*ps).event_glob;

            (*ps).event_glob = strdup(arg);
            if (*ps).event_glob.is_null() {
                ret = -1;
                break;
            }
            (*ps).pmu_glob = strdup(c"tracepoint".as_ptr());
            if (*ps).pmu_glob.is_null() {
                zfree(&mut (*ps).event_glob);
                ret = -1;
                break;
            }
            perf_pmus__print_pmu_events(&print_cb, ps as *mut c_void);
            zfree(&mut (*ps).pmu_glob);
            (*ps).pmu_glob = old_pmu_glob;
            print_sdt_events(&print_cb, ps as *mut c_void);
            (*ps).metrics = true;
            (*ps).metricgroups = true;
            metricgroup__print(&print_cb, ps as *mut c_void);
            zfree(&mut (*ps).event_glob);
            (*ps).event_glob = old_event_glob;
        } else {
            if asprintf(&mut s, c"*%s*".as_ptr(), arg) < 0 {
                printf(c"Critical: Not enough memory! Trying to continue...\n".as_ptr());
                i += 1;
                continue;
            }
            (*ps).event_glob = s;
            perf_pmus__print_pmu_events(&print_cb, ps as *mut c_void);
            print_sdt_events(&print_cb, ps as *mut c_void);
            (*ps).metrics = true;
            (*ps).metricgroups = true;
            metricgroup__print(&print_cb, ps as *mut c_void);
            free(s as *mut c_void);
        }
        i += 1;
    }

    goto_out(&mut print_cb, ps, output_path);
    ret
}

unsafe fn goto_out(print_cb: *mut print_callbacks, ps: *mut print_state, output_path: *const c_char) {
    ((*print_cb).print_end.unwrap())(ps as *mut c_void);
    free((*ps).pmu_glob as *mut c_void);
    free((*ps).last_topic as *mut c_void);
    free((*ps).last_metricgroups as *mut c_void);
    strlist__delete((*ps).visited_metrics);
    if !output_path.is_null() {
        fclose((*ps).fp);
    }
}
