// SPDX-License-Identifier: GPL-2.0-only
/*
 * builtin-kallsyms.c
 *
 * Builtin command: Look for a symbol in the running kernel and its modules
 *
 * Copyright (C) 2017, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

use std::ffi::{c_char, c_int, c_void};
use std::ptr;

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub start: u64,
    pub end: u64,
    pub namelen: u16,
    pub binding: u8,
    pub idle: u8,
    pub ignore: u8,
    pub inlined: u8,
    pub arch_sym: u8,
    pub annotate2: u8,
    pub name: [c_char; 0],
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol_conf_t {
    pub vmlinux_name: *const c_char,
    pub try_vmlinux_path: bool,
}

unsafe extern "C" {
    static mut verbose: c_int;
    static mut symbol_conf: symbol_conf_t;

    fn perf_env__init(env: *mut perf_env);
    fn perf_env__set_cmdline(env: *mut perf_env, argc: c_int, argv: *const *const c_char) -> c_int;
    fn perf_env__exit(env: *mut perf_env);

    fn machine__new_kallsyms(env: *mut perf_env) -> *mut machine;
    fn machine__delete(machine: *mut machine);
    fn machine__find_kernel_symbol_by_name(
        machine: *mut machine,
        name: *const c_char,
        mapp: *mut *mut map,
    ) -> *mut symbol;

    fn map__dso(map: *mut map) -> *const dso;
    fn map__unmap_ip(map: *mut map, ip: u64) -> u64;
    fn dso__short_name(dso: *const dso) -> *const c_char;
    fn dso__long_name(dso: *const dso) -> *const c_char;

    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option) -> !;
    fn symbol__init(arg: *mut c_void) -> c_int;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...) -> c_int;
}

unsafe fn __cmd_kallsyms(mut argc: c_int, argv: *const *const c_char) -> c_int {
    let mut i: c_int;
    let mut err: c_int;
    let mut host_env: perf_env = std::mem::zeroed();
    let mut machine: *mut machine = ptr::null_mut();

    perf_env__init(&mut host_env);
    err = perf_env__set_cmdline(&mut host_env, argc, argv);
    if err != 0 {
        goto_out(machine, &mut host_env);
        return err;
    }

    machine = machine__new_kallsyms(&mut host_env);
    if machine.is_null() {
        pr_err(c"Couldn't read /proc/kallsyms\n".as_ptr());
        err = -1;
        goto_out(machine, &mut host_env);
        return err;
    }

    i = 0;
    while i < argc {
        let mut map: *mut map = ptr::null_mut();
        let dso: *const dso;
        let symbol: *mut symbol =
            machine__find_kernel_symbol_by_name(machine, *argv.offset(i as isize), &mut map);

        if symbol.is_null() {
            printf(c"%s: not found\n".as_ptr(), *argv.offset(i as isize));
            i += 1;
            continue;
        }

        dso = map__dso(map);
        printf(
            c"%s: %s %s %#lx-%#lx (%#lx-%#lx)\n".as_ptr(),
            (*symbol).name.as_ptr(),
            dso__short_name(dso),
            dso__long_name(dso),
            map__unmap_ip(map, (*symbol).start),
            map__unmap_ip(map, (*symbol).end),
            (*symbol).start,
            (*symbol).end,
        );
        i += 1;
    }

    goto_out(machine, &mut host_env);
    err
}

unsafe fn goto_out(machine: *mut machine, host_env: *mut perf_env) {
    machine__delete(machine);
    perf_env__exit(host_env);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_kallsyms(mut argc: c_int, argv: *const *const c_char) -> c_int {
    /*
     * Original C:
     * const struct option options[] = {
     * OPT_INCR('v', "verbose", &verbose, "be more verbose (show counter open errors, etc)"),
     * OPT_END()
     * };
     *
     * The concrete struct option layout and OPT_* macro expansion are supplied
     * by <subcmd/parse-options.h>; preserve the local declaration site as an
     * external-layout dependency.
     */
    let options: [option; 0] = [];
    let kallsyms_usage: [*const c_char; 2] = [
        c"perf kallsyms [<options>] symbol_name".as_ptr(),
        ptr::null(),
    ];

    argc = parse_options(argc, argv, options.as_ptr(), kallsyms_usage.as_ptr(), 0);
    if argc < 1 {
        usage_with_options(kallsyms_usage.as_ptr(), options.as_ptr());
    }

    symbol_conf.try_vmlinux_path = symbol_conf.vmlinux_name.is_null();
    if symbol__init(ptr::null_mut()) < 0 {
        return -1;
    }

    __cmd_kallsyms(argc, argv)
}
