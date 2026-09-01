// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * builtin-probe.c
 *
 * Builtin probe command: Set up probe events by C expression
 *
 * Written by Masami Hiramatsu <mhiramat@redhat.com>
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::size_of;
use core::ptr;

const DEFAULT_VAR_FILTER: &[u8] = b"!__k???tab_* & !__crc_*\0";
const DEFAULT_FUNC_FILTER: &[u8] = b"!_* & !*@plt\0";
const DEFAULT_LIST_FILTER: &[u8] = b"*\0";

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const MAX_PROBES: usize = 128;
const STRERR_BUFSIZE: usize = 128;
const PF_FL_RW: c_int = 0;
const PARSE_OPT_EXCLUSIVE: c_int = 1;
const PARSE_OPT_STOP_AT_NON_OPTION: c_int = 2;

#[repr(C)]
pub struct perf_probe_event {
    pub uprobes: bool,
    pub target: *mut c_char,
    pub nsi: *mut nsinfo,
    pub nargs: c_int,
    pub ntevs: c_int,
    pub tevs: *mut probe_trace_event,
}

#[repr(C)]
pub struct line_range {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strfilter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nsinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    pub short_name: c_int,
    pub long_name: *const c_char,
}

#[repr(C)]
pub struct probe_trace_event {
    pub event: *const c_char,
    pub group: *const c_char,
    pub point: probe_trace_point,
}

#[repr(C)]
pub struct probe_trace_point {
    pub module: *const c_char,
}

#[repr(C)]
pub struct probe_cache {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct str_node {
    pub s: *const c_char,
}

#[repr(C)]
pub struct probe_conf_t {
    pub bootconfig: bool,
    pub cache: bool,
    pub force_add: bool,
    pub show_ext_vars: bool,
    pub show_location_range: bool,
    pub no_inlines: bool,
    pub max_probes: c_int,
}

#[repr(C)]
pub struct symbol_conf_t {
    pub vmlinux_name: *mut c_char,
    pub source_prefix: *mut c_char,
    pub demangle: bool,
    pub demangle_kernel: bool,
    pub try_vmlinux_path: bool,
    pub ignore_vmlinux_buildid: bool,
}

#[repr(C)]
struct params_t {
    command: c_int, /* Command short_name */
    list_events: bool,
    uprobes: bool,
    target_used: bool,
    nevents: c_int,
    events: [perf_probe_event; MAX_PROBES],
    line_range: line_range,
    target: *mut c_char,
    filter: *mut strfilter,
    nsi: *mut nsinfo,
}

static mut params: *mut params_t = ptr::null_mut();

unsafe extern "C" {
    static mut errno: c_int;
    static mut verbose: c_int;
    static mut quiet: bool;
    static mut probe_conf: probe_conf_t;
    static mut symbol_conf: symbol_conf_t;
    static mut probe_event_dry_run: bool;

    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;

    fn zalloc(size: usize) -> *mut c_void;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;

    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);

    fn nsinfo__get(nsi: *mut nsinfo) -> *mut nsinfo;
    fn nsinfo__put(nsi: *mut nsinfo);
    fn nsinfo__new(pid: c_int) -> *mut nsinfo;
    fn nsinfo__need_setns(nsi: *mut nsinfo) -> bool;
    fn nsinfo__realpath(path: *const c_char, nsi: *mut nsinfo) -> *mut c_char;

    fn parse_perf_probe_command(str_: *const c_char, pev: *mut perf_probe_event) -> c_int;
    fn parse_line_range_desc(str_: *const c_char, range: *mut line_range) -> c_int;
    fn line_range__init(range: *mut line_range) -> c_int;
    fn line_range__clear(range: *mut line_range);
    fn clear_perf_probe_event(pev: *mut perf_probe_event);
    fn strfilter__new(str_: *const c_char, err: *mut *const c_char) -> *mut strfilter;
    fn strfilter__or(filter: *mut strfilter, str_: *const c_char, err: *mut *const c_char) -> c_int;
    fn strfilter__delete(filter: *mut strfilter);
    fn strfilter__string(filter: *mut strfilter) -> *mut c_char;
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> *const c_char;

    fn init_probe_symbol_maps(uprobes: bool) -> c_int;
    fn convert_perf_probe_events(pevs: *mut perf_probe_event, npevs: c_int) -> c_int;
    fn show_bootconfig_events(pevs: *mut perf_probe_event, npevs: c_int) -> c_int;
    fn show_probe_trace_events(pevs: *mut perf_probe_event, npevs: c_int) -> c_int;
    fn apply_perf_probe_events(pevs: *mut perf_probe_event, npevs: c_int) -> c_int;
    fn show_perf_probe_event(
        group: *const c_char,
        event: *const c_char,
        pev: *mut perf_probe_event,
        module: *const c_char,
        use_stdout: bool,
    );
    fn cleanup_perf_probe_events(pevs: *mut perf_probe_event, npevs: c_int);
    fn exit_probe_symbol_maps();

    fn build_id_cache__list_all(validonly: bool) -> *mut strlist;
    fn strlist__new(list: *const c_char, dupstr: *const c_void) -> *mut strlist;
    fn strlist__delete(slist: *mut strlist);
    fn strlist__next(slist: *mut strlist, prev: *mut str_node) -> *mut str_node;
    fn probe_cache__new(build_id: *const c_char, target: *const c_char) -> *mut probe_cache;
    fn probe_cache__filter_purge(cache: *mut probe_cache, filter: *mut strfilter) -> c_int;
    fn probe_cache__commit(cache: *mut probe_cache) -> c_int;
    fn probe_cache__delete(cache: *mut probe_cache);
    fn probe_file__open_both(kfd: *mut c_int, ufd: *mut c_int, flag: c_int) -> c_int;
    fn probe_file__get_events(fd: c_int, filter: *mut strfilter, list: *mut strlist) -> c_int;
    fn probe_file__del_strlist(fd: c_int, list: *mut strlist) -> c_int;
    fn show_perf_probe_events(filter: *mut strfilter) -> c_int;
    fn show_available_funcs(
        target: *mut c_char,
        nsi: *mut nsinfo,
        filter: *mut strfilter,
        uprobes: bool,
    ) -> c_int;
    fn show_line_range(
        range: *mut line_range,
        target: *mut c_char,
        nsi: *mut nsinfo,
        uprobes: bool,
    ) -> c_int;
    fn show_available_vars(
        events: *mut perf_probe_event,
        nevents: c_int,
        filter: *mut strfilter,
    ) -> c_int;

    fn set_option_flag(options: *mut option, s: c_int, l: *const c_char, flags: c_int);
    fn set_option_nobuild(
        options: *mut option,
        s: c_int,
        l: *const c_char,
        reason: *const c_char,
        can_skip: bool,
    );
    fn parse_options(
        argc: c_int,
        argv: *mut *const c_char,
        options: *mut option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options_msg(usagestr: *const *const c_char, options: *mut option, fmt: *const c_char, ...);
    fn usage_with_options(usagestr: *const *const c_char, options: *mut option);
    fn parse_options_usage(usagestr: *const *const c_char, options: *mut option, optstr: *const c_char, short_opt: bool);
    fn symbol__validate_sym_arguments() -> c_int;
    fn symbol__config_symfs(opt: *const option, str_: *const c_char, unset: c_int) -> c_int;
}

unsafe fn zfree<T>(ptrp: *mut *mut T) {
    if !(*ptrp).is_null() {
        free(*ptrp as *mut c_void);
        *ptrp = ptr::null_mut();
    }
}

/* Parse an event definition. Note that any error must die. */
unsafe fn parse_probe_event(str_: *const c_char) -> c_int {
    let pev = (*params).events.as_mut_ptr().add((*params).nevents as usize);
    let ret: c_int;

    pr_debug(
        c"probe-definition(%d): %s\n".as_ptr(),
        (*params).nevents,
        str_,
    );
    (*params).nevents += 1;
    if (*params).nevents as usize == MAX_PROBES {
        pr_err(c"Too many probes (> %d) were specified.".as_ptr(), MAX_PROBES as c_int);
        return -1;
    }

    (*pev).uprobes = (*params).uprobes;
    if !(*params).target.is_null() {
        (*pev).target = strdup((*params).target);
        if (*pev).target.is_null() {
            return -ENOMEM;
        }
        (*params).target_used = true;
    }

    (*pev).nsi = nsinfo__get((*params).nsi);

    /* Parse a perf-probe command into event */
    ret = parse_perf_probe_command(str_, pev);
    pr_debug(c"%d arguments\n".as_ptr(), (*pev).nargs);

    ret
}

unsafe fn params_add_filter(str_: *const c_char) -> c_int {
    let mut err: *const c_char = ptr::null();
    let mut ret: c_int = 0;

    pr_debug2(c"Add filter: %s\n".as_ptr(), str_);
    if (*params).filter.is_null() {
        (*params).filter = strfilter__new(str_, &mut err);
        if (*params).filter.is_null() {
            ret = if !err.is_null() { -EINVAL } else { -ENOMEM };
        }
    } else {
        ret = strfilter__or((*params).filter, str_, &mut err);
    }

    if ret == -EINVAL {
        let pos = err.offset_from(str_) + 1;
        pr_err(c"Filter parse error at %td.\n".as_ptr(), pos);
        pr_err(c"Source: \"%s\"\n".as_ptr(), str_);
        pr_err(c"         %*c\n".as_ptr(), pos as c_int, '^' as c_int);
    }

    ret
}

unsafe fn set_target(ptr_: *const c_char) -> c_int {
    let mut found: c_int = 0;
    let buf: *const c_char;

    /*
     * The first argument after options can be an absolute path
     * to an executable / library or kernel module.
     *
     * TODO: Support relative path, and $PATH, $LD_LIBRARY_PATH,
     * short module name.
     */
    if (*params).target.is_null() && !ptr_.is_null() && *ptr_ == b'/' as c_char {
        (*params).target = strdup(ptr_);
        if (*params).target.is_null() {
            return -ENOMEM;
        }
        (*params).target_used = false;

        found = 1;
        buf = ptr_.add(strlen(ptr_) - 3);

        if strcmp(buf, c".ko".as_ptr()) != 0 {
            (*params).uprobes = true;
        }
    }

    found
}

unsafe fn parse_probe_event_argv(argc: c_int, argv: *mut *const c_char) -> c_int {
    let mut i: c_int;
    let mut len: c_int;
    let ret: c_int;
    let found_target: c_int;
    let buf: *mut c_char;

    found_target = set_target(*argv.add(0));
    if found_target < 0 {
        return found_target;
    }

    if found_target != 0 && argc == 1 {
        return 0;
    }

    /* Bind up rest arguments */
    len = 0;
    i = 0;
    while i < argc {
        if !(i == 0 && found_target != 0) {
            len += strlen(*argv.add(i as usize)) as c_int + 1;
        }
        i += 1;
    }
    buf = zalloc((len + 1) as usize) as *mut c_char;
    if buf.is_null() {
        return -ENOMEM;
    }
    len = 0;
    i = 0;
    while i < argc {
        if !(i == 0 && found_target != 0) {
            len += sprintf(buf.add(len as usize), c"%s ".as_ptr(), *argv.add(i as usize));
        }
        i += 1;
    }
    ret = parse_probe_event(buf);
    free(buf as *mut c_void);
    ret
}

unsafe extern "C" fn opt_set_target(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let mut ret = -ENOENT;
    let tmp: *mut c_char;

    if !str_.is_null() {
        if strcmp((*opt).long_name, c"exec".as_ptr()) == 0 {
            (*params).uprobes = true;
        } else if strcmp((*opt).long_name, c"module".as_ptr()) == 0 {
            (*params).uprobes = false;
        } else {
            return ret;
        }

        /* Expand given path to absolute path, except for modulename */
        if (*params).uprobes || !strchr(str_, '/' as c_int).is_null() {
            tmp = nsinfo__realpath(str_, (*params).nsi);
            if tmp.is_null() {
                pr_warning(c"Failed to get the absolute path of %s: %m\n".as_ptr(), str_);
                return ret;
            }
        } else {
            tmp = strdup(str_);
            if tmp.is_null() {
                return -ENOMEM;
            }
        }
        free((*params).target as *mut c_void);
        (*params).target = tmp;
        (*params).target_used = false;
        ret = 0;
    }

    ret
}

unsafe extern "C" fn opt_set_target_ns(_opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let mut ret = -ENOENT;
    let ns_pid: c_int;
    let nsip: *mut nsinfo;

    if !str_.is_null() {
        errno = 0;
        ns_pid = strtol(str_, ptr::null_mut(), 10) as c_int;
        if errno != 0 {
            ret = -errno;
            pr_warning(c"Failed to parse %s as a pid: %m\n".as_ptr(), str_);
            return ret;
        }
        nsip = nsinfo__new(ns_pid);
        if !nsip.is_null() && nsinfo__need_setns(nsip) {
            (*params).nsi = nsinfo__get(nsip);
        }
        nsinfo__put(nsip);

        ret = 0;
    }

    ret
}

/* Command option callbacks */

/* #ifdef HAVE_LIBDW_SUPPORT */
unsafe extern "C" fn opt_show_lines(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let mut ret = 0;

    if str_.is_null() {
        return 0;
    }

    if (*params).command == 'L' as c_int {
        pr_warning(c"Warning: more than one --line options are detected. Only the first one is valid.\n".as_ptr());
        return 0;
    }

    (*params).command = (*opt).short_name;
    ret = parse_line_range_desc(str_, &mut (*params).line_range);

    ret
}

unsafe extern "C" fn opt_show_vars(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let pev = (*params).events.as_mut_ptr().add((*params).nevents as usize);
    let ret: c_int;

    if str_.is_null() {
        return 0;
    }

    ret = parse_probe_event(str_);
    if ret == 0 && (*pev).nargs != 0 {
        pr_err(c"  Error: '--vars' doesn't accept arguments.\n".as_ptr());
        return -EINVAL;
    }
    (*params).command = (*opt).short_name;

    ret
}
/* #else: opt_show_lines and opt_show_vars are NULL when HAVE_LIBDW_SUPPORT is unavailable. */

unsafe extern "C" fn opt_add_probe_event(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    if !str_.is_null() {
        (*params).command = (*opt).short_name;
        return parse_probe_event(str_);
    }

    0
}

unsafe extern "C" fn opt_set_filter_with_command(opt: *const option, str_: *const c_char, unset: c_int) -> c_int {
    if unset == 0 {
        (*params).command = (*opt).short_name;
    }

    if !str_.is_null() {
        return params_add_filter(str_);
    }

    0
}

unsafe extern "C" fn opt_set_filter(_opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    if !str_.is_null() {
        return params_add_filter(str_);
    }

    0
}

unsafe fn init_params() -> c_int {
    let ret: c_int;

    params = calloc(1, size_of::<params_t>()) as *mut params_t;
    if params.is_null() {
        return -ENOMEM;
    }

    ret = line_range__init(&mut (*params).line_range);
    if ret != 0 {
        zfree(&mut params);
    }
    ret
}

unsafe fn cleanup_params() {
    let mut i: c_int = 0;

    while i < (*params).nevents {
        clear_perf_probe_event((*params).events.as_mut_ptr().add(i as usize));
        i += 1;
    }
    line_range__clear(&mut (*params).line_range);
    zfree(&mut (*params).target);
    strfilter__delete((*params).filter);
    nsinfo__put((*params).nsi);
    zfree(&mut params);
}

unsafe fn pr_err_with_code(msg: *const c_char, err: c_int) {
    let mut sbuf = [0 as c_char; STRERR_BUFSIZE];

    pr_err(c"%s".as_ptr(), msg);
    pr_debug(
        c" Reason: %s (Code: %d)".as_ptr(),
        str_error_r(-err, sbuf.as_mut_ptr(), sbuf.len()),
        err,
    );
    pr_err(c"\n".as_ptr());
}

unsafe fn perf_add_probe_events(pevs: *mut perf_probe_event, npevs: c_int) -> c_int {
    let mut ret: c_int;
    let mut i: c_int;
    let mut k: c_int;
    let mut event: *const c_char = ptr::null();
    let mut group: *const c_char = ptr::null();

    ret = init_probe_symbol_maps((*pevs).uprobes);
    if ret < 0 {
        return ret;
    }

    ret = convert_perf_probe_events(pevs, npevs);
    if ret < 0 {
        cleanup_perf_probe_events(pevs, npevs);
        exit_probe_symbol_maps();
        return ret;
    }

    if (*params).command == 'D' as c_int {
        /* it shows definition */
        if probe_conf.bootconfig {
            ret = show_bootconfig_events(pevs, npevs);
        } else {
            ret = show_probe_trace_events(pevs, npevs);
        }
        cleanup_perf_probe_events(pevs, npevs);
        exit_probe_symbol_maps();
        return ret;
    }

    ret = apply_perf_probe_events(pevs, npevs);
    if ret < 0 {
        cleanup_perf_probe_events(pevs, npevs);
        exit_probe_symbol_maps();
        return ret;
    }

    i = 0;
    k = 0;
    while i < npevs {
        k += (*pevs.add(i as usize)).ntevs;
        i += 1;
    }

    pr_info(c"Added new event%s\n".as_ptr(), if k > 1 { c"s:".as_ptr() } else { c":".as_ptr() });
    i = 0;
    while i < npevs {
        let pev = pevs.add(i as usize);
        k = 0;
        while k < (*pev).ntevs {
            let tev = (*pev).tevs.add(k as usize);
            /* Skipped events have no event name */
            if !(*tev).event.is_null() {
                /* We use tev's name for showing new events */
                show_perf_probe_event((*tev).group, (*tev).event, pev, (*tev).point.module, false);

                /* Save the last valid name */
                event = (*tev).event;
                group = (*tev).group;
            }
            k += 1;
        }
        i += 1;
    }

    /* Note that it is possible to skip all events because of blacklist */
    if !event.is_null() {
        /* #ifndef HAVE_LIBTRACEEVENT */
        pr_info(c"\nperf is not linked with libtraceevent, to use the new probe you can use tracefs:\n\n".as_ptr());
        pr_info(c"\tcd /sys/kernel/tracing/\n".as_ptr());
        pr_info(c"\techo 1 > events/%s/%s/enable\n".as_ptr(), group, event);
        pr_info(c"\techo 1 > tracing_on\n".as_ptr());
        pr_info(c"\tcat trace_pipe\n".as_ptr());
        pr_info(c"\tBefore removing the probe, echo 0 > events/%s/%s/enable\n".as_ptr(), group, event);
        /* #else
         * Show how to use the event.
         * pr_info("\nYou can now use it in all perf tools, such as:\n\n");
         * pr_info("\tperf record -e %s:%s -aR sleep 1\n\n", group, event);
         * #endif
         */
    }

    cleanup_perf_probe_events(pevs, npevs);
    exit_probe_symbol_maps();
    ret
}

unsafe fn del_perf_probe_caches(filter: *mut strfilter) -> c_int {
    let mut cache: *mut probe_cache;
    let bidlist: *mut strlist;
    let mut nd: *mut str_node;
    let ret: c_int;

    bidlist = build_id_cache__list_all(false);
    if bidlist.is_null() {
        ret = -errno;
        pr_debug(c"Failed to get buildids: %d\n".as_ptr(), ret);
        return if ret != 0 { ret } else { -ENOMEM };
    }

    nd = strlist__next(bidlist, ptr::null_mut());
    while !nd.is_null() {
        cache = probe_cache__new((*nd).s, ptr::null());
        if !cache.is_null() {
            if probe_cache__filter_purge(cache, filter) < 0 || probe_cache__commit(cache) < 0 {
                pr_warning(c"Failed to remove entries for %s\n".as_ptr(), (*nd).s);
            }
            probe_cache__delete(cache);
        }
        nd = strlist__next(bidlist, nd);
    }
    0
}

unsafe fn perf_del_probe_events(filter: *mut strfilter) -> c_int {
    let mut ret: c_int;
    let mut ret2: c_int;
    let mut ufd: c_int = -1;
    let mut kfd: c_int = -1;
    let str_ = strfilter__string(filter);
    let mut klist: *mut strlist = ptr::null_mut();
    let mut ulist: *mut strlist = ptr::null_mut();
    let mut ent: *mut str_node;

    if str_.is_null() {
        return -EINVAL;
    }

    pr_debug(c"Delete filter: '%s'\n".as_ptr(), str_);

    if probe_conf.cache {
        return del_perf_probe_caches(filter);
    }

    /* Get current event names */
    ret = probe_file__open_both(&mut kfd, &mut ufd, PF_FL_RW);
    if ret < 0 {
        strlist__delete(klist);
        strlist__delete(ulist);
        free(str_ as *mut c_void);
        return ret;
    }

    klist = strlist__new(ptr::null(), ptr::null());
    ulist = strlist__new(ptr::null(), ptr::null());
    if klist.is_null() || ulist.is_null() {
        ret = -ENOMEM;
        if kfd >= 0 {
            close(kfd);
        }
        if ufd >= 0 {
            close(ufd);
        }
        strlist__delete(klist);
        strlist__delete(ulist);
        free(str_ as *mut c_void);
        return ret;
    }

    ret = probe_file__get_events(kfd, filter, klist);
    if ret == 0 {
        ent = strlist__next(klist, ptr::null_mut());
        while !ent.is_null() {
            pr_info(c"Removed event: %s\n".as_ptr(), (*ent).s);
            ent = strlist__next(klist, ent);
        }

        ret = probe_file__del_strlist(kfd, klist);
        if ret < 0 {
            if kfd >= 0 {
                close(kfd);
            }
            if ufd >= 0 {
                close(ufd);
            }
            strlist__delete(klist);
            strlist__delete(ulist);
            free(str_ as *mut c_void);
            return ret;
        }
    } else if ret == -ENOMEM {
        if kfd >= 0 {
            close(kfd);
        }
        if ufd >= 0 {
            close(ufd);
        }
        strlist__delete(klist);
        strlist__delete(ulist);
        free(str_ as *mut c_void);
        return ret;
    }

    ret2 = probe_file__get_events(ufd, filter, ulist);
    if ret2 == 0 {
        ent = strlist__next(ulist, ptr::null_mut());
        while !ent.is_null() {
            pr_info(c"Removed event: %s\n".as_ptr(), (*ent).s);
            ent = strlist__next(ulist, ent);
        }

        ret2 = probe_file__del_strlist(ufd, ulist);
        if ret2 < 0 {
            ret = ret2;
            if kfd >= 0 {
                close(kfd);
            }
            if ufd >= 0 {
                close(ufd);
            }
            strlist__delete(klist);
            strlist__delete(ulist);
            free(str_ as *mut c_void);
            return ret;
        }
    } else if ret2 == -ENOMEM {
        ret = ret2;
        if kfd >= 0 {
            close(kfd);
        }
        if ufd >= 0 {
            close(ufd);
        }
        strlist__delete(klist);
        strlist__delete(ulist);
        free(str_ as *mut c_void);
        return ret;
    }

    if ret == -ENOENT && ret2 == -ENOENT {
        pr_warning(c"\"%s\" does not hit any event.\n".as_ptr(), str_);
    } else {
        ret = 0;
    }

    if kfd >= 0 {
        close(kfd);
    }
    if ufd >= 0 {
        close(ufd);
    }
    strlist__delete(klist);
    strlist__delete(ulist);
    free(str_ as *mut c_void);

    ret
}

/* #ifdef HAVE_LIBDW_SUPPORT */
const PROBEDEF_STR: &[u8] =
    b"[EVENT=]FUNC[@SRC][+OFF|%return|:RL|;PT]|SRC:AL|SRC;PT [[NAME=]ARG ...]\0";
/* #else
 * const PROBEDEF_STR: &[u8] = b"[EVENT=]FUNC[+OFF|%return] [[NAME=]ARG ...]\0";
 * #endif
 */

unsafe fn __cmd_probe(mut argc: c_int, argv: *mut *const c_char) -> c_int {
    let probe_usage: [*const c_char; 8] = [
        c"perf probe [<options>] 'PROBEDEF' ['PROBEDEF' ...]".as_ptr(),
        c"perf probe [<options>] --add 'PROBEDEF' [--add 'PROBEDEF' ...]".as_ptr(),
        c"perf probe [<options>] --del '[GROUP:]EVENT' ...".as_ptr(),
        c"perf probe --list [GROUP:]EVENT ...".as_ptr(),
        c"perf probe [<options>] --line 'LINEDESC'".as_ptr(),
        c"perf probe [<options>] --vars 'PROBEPOINT'".as_ptr(),
        c"perf probe [<options>] --funcs".as_ptr(),
        ptr::null(),
    ];
    /*
     * The C source builds a rich parse-options table with OPT_* macros. In this
     * isolated Rust translation those macro-expanded initializers are external
     * dependency detail, so this zeroed table preserves the local variable and
     * call structure while leaving actual option construction to the imported
     * parse-options layer.
     */
    let mut options: [option; 1] = [option {
        short_name: 0,
        long_name: ptr::null(),
    }];
    let mut ret: c_int;

    set_option_flag(options.as_mut_ptr(), 'a' as c_int, c"add".as_ptr(), PARSE_OPT_EXCLUSIVE);
    set_option_flag(options.as_mut_ptr(), 'd' as c_int, c"del".as_ptr(), PARSE_OPT_EXCLUSIVE);
    set_option_flag(options.as_mut_ptr(), 'D' as c_int, c"definition".as_ptr(), PARSE_OPT_EXCLUSIVE);
    set_option_flag(options.as_mut_ptr(), 'l' as c_int, c"list".as_ptr(), PARSE_OPT_EXCLUSIVE);
    /* #ifdef HAVE_LIBDW_SUPPORT */
    set_option_flag(options.as_mut_ptr(), 'L' as c_int, c"line".as_ptr(), PARSE_OPT_EXCLUSIVE);
    set_option_flag(options.as_mut_ptr(), 'V' as c_int, c"vars".as_ptr(), PARSE_OPT_EXCLUSIVE);
    /* #else: set_option_nobuild(..., "NO_LIBDW=1", ...) for line, vars, externs, range, vmlinux, source, no-inlines. */
    set_option_flag(options.as_mut_ptr(), 'F' as c_int, c"funcs".as_ptr(), PARSE_OPT_EXCLUSIVE);

    argc = parse_options(
        argc,
        argv,
        options.as_mut_ptr(),
        probe_usage.as_ptr(),
        PARSE_OPT_STOP_AT_NON_OPTION,
    );

    if quiet {
        if verbose != 0 {
            pr_err(c"  Error: -v and -q are exclusive.\n".as_ptr());
            return -EINVAL;
        }
        verbose = -1;
    }

    if argc > 0 {
        if strcmp(*argv.add(0), c"-".as_ptr()) == 0 {
            usage_with_options_msg(
                probe_usage.as_ptr(),
                options.as_mut_ptr(),
                c"'-' is not supported.\n".as_ptr(),
            );
        }
        if (*params).command != 0 && (*params).command != 'a' as c_int {
            usage_with_options_msg(
                probe_usage.as_ptr(),
                options.as_mut_ptr(),
                c"another command except --add is set.\n".as_ptr(),
            );
        }
        ret = parse_probe_event_argv(argc, argv);
        if ret < 0 {
            pr_err_with_code(c"  Error: Command Parse Error.".as_ptr(), ret);
            return ret;
        }
        (*params).command = 'a' as c_int;
    }

    ret = symbol__validate_sym_arguments();
    if ret != 0 {
        return ret;
    }

    if probe_conf.max_probes == 0 {
        probe_conf.max_probes = MAX_PROBES as c_int;
    }

    /*
     * Only consider the user's kernel image path if given.
     */
    symbol_conf.try_vmlinux_path = symbol_conf.vmlinux_name.is_null();

    /*
     * Except for --list, --del and --add, other command doesn't depend
     * nor change running kernel. So if user gives offline vmlinux,
     * ignore its buildid.
     */
    if strchr(c"lda".as_ptr(), (*params).command).is_null() && !symbol_conf.vmlinux_name.is_null() {
        symbol_conf.ignore_vmlinux_buildid = true;
    }

    match (*params).command {
        x if x == 'l' as c_int => {
            if (*params).uprobes {
                pr_err(c"  Error: Don't use --list with --exec.\n".as_ptr());
                parse_options_usage(probe_usage.as_ptr(), options.as_mut_ptr(), c"l".as_ptr(), true);
                parse_options_usage(ptr::null(), options.as_mut_ptr(), c"x".as_ptr(), true);
                return -EINVAL;
            }
            ret = show_perf_probe_events((*params).filter);
            if ret < 0 {
                pr_err_with_code(c"  Error: Failed to show event list.".as_ptr(), ret);
            }
            return ret;
        }
        x if x == 'F' as c_int => {
            ret = show_available_funcs((*params).target, (*params).nsi, (*params).filter, (*params).uprobes);
            if ret < 0 {
                pr_err_with_code(c"  Error: Failed to show functions.".as_ptr(), ret);
            }
            return ret;
        }
        x if x == 'L' as c_int => {
            ret = show_line_range(&mut (*params).line_range, (*params).target, (*params).nsi, (*params).uprobes);
            if ret < 0 {
                pr_err_with_code(c"  Error: Failed to show lines.".as_ptr(), ret);
            }
            return ret;
        }
        x if x == 'V' as c_int => {
            if (*params).filter.is_null() {
                (*params).filter = strfilter__new(DEFAULT_VAR_FILTER.as_ptr() as *const c_char, ptr::null_mut());
            }

            ret = show_available_vars((*params).events.as_mut_ptr(), (*params).nevents, (*params).filter);
            if ret < 0 {
                pr_err_with_code(c"  Error: Failed to show vars.".as_ptr(), ret);
            }
            return ret;
        }
        x if x == 'd' as c_int => {
            ret = perf_del_probe_events((*params).filter);
            if ret < 0 {
                pr_err_with_code(c"  Error: Failed to delete events.".as_ptr(), ret);
                return ret;
            }
        }
        x if x == 'D' as c_int => {
            if probe_conf.bootconfig && (*params).uprobes {
                pr_err(c"  Error: --bootconfig doesn't support uprobes.\n".as_ptr());
                return -EINVAL;
            }
            /* fallthrough */
            if (*params).target_used == false && !(*params).target.is_null() {
                pr_err(c"  Error: -x/-m must follow the probe definitions.\n".as_ptr());
                parse_options_usage(probe_usage.as_ptr(), options.as_mut_ptr(), c"m".as_ptr(), true);
                parse_options_usage(ptr::null(), options.as_mut_ptr(), c"x".as_ptr(), true);
                return -EINVAL;
            }

            ret = perf_add_probe_events((*params).events.as_mut_ptr(), (*params).nevents);
            if ret < 0 {
                /*
                 * When perf_add_probe_events() fails it calls
                 * cleanup_perf_probe_events(pevs, npevs), i.e.
                 * cleanup_perf_probe_events(params->events, params->nevents), which
                 * will call clear_perf_probe_event(), so set nevents to zero
                 * to avoid cleanup_params() to call clear_perf_probe_event() again
                 * on the same pevs.
                 */
                (*params).nevents = 0;
                pr_err_with_code(c"  Error: Failed to add events.".as_ptr(), ret);
                return ret;
            }
        }
        x if x == 'a' as c_int => {
            /* Ensure the last given target is used */
            if (*params).target_used == false && !(*params).target.is_null() {
                pr_err(c"  Error: -x/-m must follow the probe definitions.\n".as_ptr());
                parse_options_usage(probe_usage.as_ptr(), options.as_mut_ptr(), c"m".as_ptr(), true);
                parse_options_usage(ptr::null(), options.as_mut_ptr(), c"x".as_ptr(), true);
                return -EINVAL;
            }

            ret = perf_add_probe_events((*params).events.as_mut_ptr(), (*params).nevents);
            if ret < 0 {
                /*
                 * When perf_add_probe_events() fails it calls
                 * cleanup_perf_probe_events(pevs, npevs), i.e.
                 * cleanup_perf_probe_events(params->events, params->nevents), which
                 * will call clear_perf_probe_event(), so set nevents to zero
                 * to avoid cleanup_params() to call clear_perf_probe_event() again
                 * on the same pevs.
                 */
                (*params).nevents = 0;
                pr_err_with_code(c"  Error: Failed to add events.".as_ptr(), ret);
                return ret;
            }
        }
        _ => {
            usage_with_options(probe_usage.as_ptr(), options.as_mut_ptr());
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_probe(argc: c_int, argv: *mut *const c_char) -> c_int {
    let mut ret: c_int;

    ret = init_params();
    if ret == 0 {
        ret = __cmd_probe(argc, argv);
        cleanup_params();
    }

    if ret < 0 { ret } else { 0 }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
