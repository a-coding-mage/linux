/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of perf/util/parse-events.h.
 *
 * Dependency intent from the original header:
 * linux/list.h, linux/types.h, linux/perf_event.h, sys/types.h, string.h.
 */

use core::ffi::{c_char, c_int, c_void};

pub type size_t = usize;
pub type uid_t = u32;
pub type u8 = u8;
pub type u32 = u32;
pub type u64 = u64;
pub type __u64 = u64;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct evsel {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct option {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct strbuf {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn event_type(type_: size_t) -> *const c_char;
}

/* Arguments encoded in opt->value. */
#[repr(C)]
pub struct parse_events_option_args {
    pub evlistp: *mut *mut evlist,
    pub pmu_filter: *const c_char,
    pub cputype_filter: bool,
}

unsafe extern "C" {
    pub fn parse_events_option(opt: *const option, str_: *const c_char, unset: c_int) -> c_int;
    pub fn parse_events_option_new_evlist(
        opt: *const option,
        str_: *const c_char,
        unset: c_int,
    ) -> c_int;
    pub fn __parse_events(
        evlist: *mut evlist,
        str_: *const c_char,
        pmu_filter: *const c_char,
        cputype_filter: bool,
        error: *mut parse_events_error,
        fake_pmu: bool,
        warn_if_reordered: bool,
        fake_tp: bool,
    ) -> c_int;
}

#[inline]
pub unsafe fn parse_events(
    evlist: *mut evlist,
    str_: *const c_char,
    err: *mut parse_events_error,
) -> c_int {
    unsafe {
        __parse_events(
            evlist,
            str_,
            core::ptr::null(),
            false,
            err,
            false,
            true,
            false,
        )
    }
}

unsafe extern "C" {
    pub fn parse_event(evlist: *mut evlist, str_: *const c_char) -> c_int;

    pub fn parse_filter(opt: *const option, str_: *const c_char, unset: c_int) -> c_int;
    pub fn parse_uid_filter(evlist: *mut evlist, uid: uid_t) -> c_int;
    pub fn exclude_perf(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum parse_events__term_val_type {
    PARSE_EVENTS__TERM_TYPE_NUM,
    PARSE_EVENTS__TERM_TYPE_STR,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum parse_events__term_type {
    PARSE_EVENTS__TERM_TYPE_USER,
    PARSE_EVENTS__TERM_TYPE_CONFIG,
    PARSE_EVENTS__TERM_TYPE_CONFIG1,
    PARSE_EVENTS__TERM_TYPE_CONFIG2,
    PARSE_EVENTS__TERM_TYPE_CONFIG3,
    PARSE_EVENTS__TERM_TYPE_CONFIG4,
    PARSE_EVENTS__TERM_TYPE_NAME,
    PARSE_EVENTS__TERM_TYPE_SAMPLE_PERIOD,
    PARSE_EVENTS__TERM_TYPE_SAMPLE_FREQ,
    PARSE_EVENTS__TERM_TYPE_BRANCH_SAMPLE_TYPE,
    PARSE_EVENTS__TERM_TYPE_TIME,
    PARSE_EVENTS__TERM_TYPE_CALLGRAPH,
    PARSE_EVENTS__TERM_TYPE_STACKSIZE,
    PARSE_EVENTS__TERM_TYPE_NOINHERIT,
    PARSE_EVENTS__TERM_TYPE_INHERIT,
    PARSE_EVENTS__TERM_TYPE_MAX_STACK,
    PARSE_EVENTS__TERM_TYPE_MAX_EVENTS,
    PARSE_EVENTS__TERM_TYPE_NOOVERWRITE,
    PARSE_EVENTS__TERM_TYPE_OVERWRITE,
    PARSE_EVENTS__TERM_TYPE_DRV_CFG,
    PARSE_EVENTS__TERM_TYPE_PERCORE,
    PARSE_EVENTS__TERM_TYPE_AUX_OUTPUT,
    PARSE_EVENTS__TERM_TYPE_AUX_ACTION,
    PARSE_EVENTS__TERM_TYPE_AUX_SAMPLE_SIZE,
    PARSE_EVENTS__TERM_TYPE_METRIC_ID,
    PARSE_EVENTS__TERM_TYPE_RAW,
    PARSE_EVENTS__TERM_TYPE_CPU,
    PARSE_EVENTS__TERM_TYPE_RATIO_TO_PREV,
    PARSE_EVENTS__TERM_TYPE_LEGACY_HARDWARE_CONFIG,
    PARSE_EVENTS__TERM_TYPE_LEGACY_CACHE_CONFIG,
}

pub const __PARSE_EVENTS__TERM_TYPE_NR: c_int =
    parse_events__term_type::PARSE_EVENTS__TERM_TYPE_LEGACY_CACHE_CONFIG as c_int + 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub union parse_events_term_val {
    pub str_: *mut c_char,
    pub num: u64,
}

#[repr(C)]
pub struct parse_events_term {
    /** @list: The term list the term is a part of. */
    pub list: list_head,
    /**
     * @config: The left-hand side of a term assignment, so the term
     * "event=8" would have the config be "event"
     */
    pub config: *const c_char,
    /**
     * @val: The right-hand side of a term assignment that can either be a
     * string or a number depending on type_val.
     */
    pub val: parse_events_term_val,
    /** @type_val: The union variable in val to be used for the term. */
    pub type_val: parse_events__term_val_type,
    /**
     * @type_term: A predefined term type or PARSE_EVENTS__TERM_TYPE_USER
     * when not inbuilt.
     */
    pub type_term: parse_events__term_type,
    /**
     * @err_term: The column index of the term from parsing, used during
     * error output.
     */
    pub err_term: c_int,
    /**
     * @err_val: The column index of the val from parsing, used during error
     * output.
     */
    pub err_val: c_int,
    /** @used: Was the term used during parameterized-eval. */
    pub used: bool,
    /**
     * @weak: A term from the sysfs or json encoding of an event that
     * shouldn't override terms coming from the command line.
     */
    pub weak: bool,
    /**
     * @no_value: Is there no value. If a numeric term has no value then the
     * value is assumed to be 1. An event name also has no value.
     */
    pub no_value: bool,
}

#[repr(C)]
pub struct parse_events_error {
    /** @list: The head of a list of errors. */
    pub list: list_head,
}

/* A wrapper around a list of terms for the sake of better type safety. */
#[repr(C)]
pub struct parse_events_terms {
    pub terms: list_head,
}

#[repr(C)]
pub struct parse_events_state {
    /* The list parsed events are placed on. */
    pub list: list_head,
    /* The updated index used by entries as they are added. */
    pub idx: c_int,
    /* Error information. */
    pub error: *mut parse_events_error,
    /* Holds returned terms for term parsing. */
    pub terms: *mut parse_events_terms,
    /* Start token. */
    pub stoken: c_int,
    /* Use the fake PMU marker for testing. */
    pub fake_pmu: bool,
    /* Skip actual tracepoint processing for testing. */
    pub fake_tp: bool,
    /* If non-null, when wildcard matching only match the given PMU. */
    pub pmu_filter: *const c_char,
    /* If true, the pmu_filter was set by --cputype option. */
    pub cputype_filter: bool,
    /* Should PE_LEGACY_NAME tokens be generated for config terms? */
    pub match_legacy_cache_terms: bool,
    /* Were multiple PMUs scanned to find events? */
    pub wild_card_pmus: bool,
}

unsafe extern "C" {
    pub fn parse_events__term_type_str(
        term_type: parse_events__term_type,
    ) -> *const c_char;

    pub fn parse_events__filter_pmu(
        parse_state: *const parse_events_state,
        pmu: *const perf_pmu,
    ) -> bool;
    pub fn parse_events__shrink_config_terms();
    pub fn parse_events__is_hardcoded_term(term: *mut parse_events_term) -> c_int;
    pub fn parse_events_term__num(
        term: *mut *mut parse_events_term,
        type_term: parse_events__term_type,
        config: *const c_char,
        num: u64,
        novalue: bool,
        loc_term: *mut c_void,
        loc_val: *mut c_void,
    ) -> c_int;
    pub fn parse_events_term__str(
        term: *mut *mut parse_events_term,
        type_term: parse_events__term_type,
        config: *mut c_char,
        str_: *mut c_char,
        loc_term: *mut c_void,
        loc_val: *mut c_void,
    ) -> c_int;
    pub fn parse_events_term__term(
        term: *mut *mut parse_events_term,
        term_lhs: parse_events__term_type,
        term_rhs: parse_events__term_type,
        loc_term: *mut c_void,
        loc_val: *mut c_void,
    ) -> c_int;
    pub fn parse_events_term__clone(
        new: *mut *mut parse_events_term,
        term: *const parse_events_term,
    ) -> c_int;
    pub fn parse_events_term__delete(term: *mut parse_events_term);

    pub fn parse_events_terms__delete(terms: *mut parse_events_terms);
    pub fn parse_events_terms__init(terms: *mut parse_events_terms);
    pub fn parse_events_terms__exit(terms: *mut parse_events_terms);
    pub fn parse_events_terms(terms: *mut parse_events_terms, str_: *const c_char) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct parse_events_modifier {
    pub precise: u8, /* Number of repeated 'p' for precision. */
    /*
     * Original C uses bool bit-fields for the following 14 one-bit flags.
     * Rust has no native C-compatible bit-field syntax, so this byte preserves
     * the storage intent for translation-only use.
     */
    pub flags: u16,
}

pub const PARSE_EVENTS_MODIFIER_PRECISE_MAX: u16 = 1 << 0; /* 'P' */
pub const PARSE_EVENTS_MODIFIER_NON_IDLE: u16 = 1 << 1; /* 'I' */
pub const PARSE_EVENTS_MODIFIER_SAMPLE_READ: u16 = 1 << 2; /* 'S' */
pub const PARSE_EVENTS_MODIFIER_PINNED: u16 = 1 << 3; /* 'D' */
pub const PARSE_EVENTS_MODIFIER_EXCLUSIVE: u16 = 1 << 4; /* 'e' */
pub const PARSE_EVENTS_MODIFIER_WEAK: u16 = 1 << 5; /* 'W' */
pub const PARSE_EVENTS_MODIFIER_BPF: u16 = 1 << 6; /* 'b' */
pub const PARSE_EVENTS_MODIFIER_USER: u16 = 1 << 7; /* 'u' */
pub const PARSE_EVENTS_MODIFIER_KERNEL: u16 = 1 << 8; /* 'k' */
pub const PARSE_EVENTS_MODIFIER_HYPERVISOR: u16 = 1 << 9; /* 'h' */
pub const PARSE_EVENTS_MODIFIER_GUEST: u16 = 1 << 10; /* 'G' */
pub const PARSE_EVENTS_MODIFIER_HOST: u16 = 1 << 11; /* 'H' */
pub const PARSE_EVENTS_MODIFIER_RETIRE_LAT: u16 = 1 << 12; /* 'R' */
pub const PARSE_EVENTS_MODIFIER_DONT_REGROUP: u16 = 1 << 13; /* 'X' */

unsafe extern "C" {
    pub fn parse_events__modifier_event(
        parse_state: *mut parse_events_state,
        loc: *mut c_void,
        list: *mut list_head,
        mod_: parse_events_modifier,
    ) -> c_int;
    pub fn parse_events__modifier_group(
        parse_state: *mut parse_events_state,
        loc: *mut c_void,
        list: *mut list_head,
        mod_: parse_events_modifier,
    ) -> c_int;
    pub fn parse_events__set_default_name(list: *mut list_head, name: *mut c_char) -> c_int;
    pub fn parse_events_add_tracepoint(
        parse_state: *mut parse_events_state,
        list: *mut list_head,
        sys: *const c_char,
        event: *const c_char,
        error: *mut parse_events_error,
        head_config: *mut parse_events_terms,
        loc: *mut c_void,
    ) -> c_int;
    pub fn parse_events_add_numeric(
        parse_state: *mut parse_events_state,
        list: *mut list_head,
        type_: u32,
        config: u64,
        head_config: *const parse_events_terms,
        wildcard: bool,
    ) -> c_int;
    pub fn parse_events__decode_legacy_cache(
        name: *const c_char,
        pmu_type: c_int,
        config: *mut __u64,
    ) -> c_int;
    pub fn parse_events_add_breakpoint(
        parse_state: *mut parse_events_state,
        list: *mut list_head,
        addr: u64,
        type_: *mut c_char,
        len: u64,
        head_config: *mut parse_events_terms,
    ) -> c_int;

    pub fn parse_events__add_event(
        idx: c_int,
        attr: *mut perf_event_attr,
        name: *const c_char,
        metric_id: *const c_char,
        pmu: *mut perf_pmu,
    ) -> *mut evsel;

    pub fn parse_events_multi_pmu_add(
        parse_state: *mut parse_events_state,
        event_name: *const c_char,
        const_parsed_terms: *const parse_events_terms,
        listp: *mut *mut list_head,
        loc: *mut c_void,
    ) -> c_int;

    pub fn parse_events_multi_pmu_add_or_add_pmu(
        parse_state: *mut parse_events_state,
        event_or_pmu: *const c_char,
        const_parsed_terms: *const parse_events_terms,
        listp: *mut *mut list_head,
        loc_: *mut c_void,
    ) -> c_int;

    pub fn parse_events__set_leader(name: *mut c_char, list: *mut list_head);
}

#[repr(C)]
pub struct event_symbol {
    pub symbol: *const c_char,
    pub alias: *const c_char,
}

unsafe extern "C" {
    pub fn parse_events_formats_error_string(additional_terms: *mut c_char) -> *mut c_char;

    pub fn parse_events_error__init(err: *mut parse_events_error);
    pub fn parse_events_error__exit(err: *mut parse_events_error);
    pub fn parse_events_error__handle(
        err: *mut parse_events_error,
        idx: c_int,
        str_: *mut c_char,
        help: *mut c_char,
    );
    pub fn parse_events_error__print(err: *const parse_events_error, event: *const c_char);
    pub fn parse_events_error__contains(
        err: *const parse_events_error,
        needle: *const c_char,
    ) -> bool;
}

/*
 * HAVE_LIBELF_SUPPORT conditional:
 * If the probe point starts with '%',
 * or starts with "sdt_" and has a ':' but no '=',
 * then it should be a SDT/cached probe point.
 *
 * The C header provides an alternate inline implementation returning false
 * when HAVE_LIBELF_SUPPORT is absent.
 */
#[cfg(HAVE_LIBELF_SUPPORT)]
#[inline]
pub unsafe fn is_sdt_event(str_: *mut c_char) -> bool {
    unsafe {
        *str_ == b'%' as c_char
            || (strncmp(str_, c"sdt_".as_ptr(), 4) == 0
                && !strchr(str_, b':' as c_int).is_null()
                && strchr(str_, b'=' as c_int).is_null())
    }
}

#[cfg(not(HAVE_LIBELF_SUPPORT))]
#[inline]
pub unsafe fn is_sdt_event(_str_: *mut c_char) -> bool {
    false
}

unsafe extern "C" {
    pub fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    pub fn strchr(s: *const c_char, c: c_int) -> *mut c_char;

    pub fn default_breakpoint_len() -> size_t;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
