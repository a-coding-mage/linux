// SPDX-License-Identifier: GPL-2.0
/*
 * builtin-config.c
 *
 * Copyright (C) 2015, Taeung Song <treeze.taeung@gmail.com>
 *
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

// C dependencies originally included from:
// "builtin.h", "util/cache.h", <subcmd/parse-options.h>, "util/debug.h",
// "util/config.h", <linux/string.h>, <limits.h>, <stdio.h>, <stdlib.h>

const PATH_MAX: usize = 4096;
const ACTION_LIST: c_uint = 1;
const PARSE_OPT_STOP_AT_NON_OPTION: c_int = 1;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct perf_config_set {
    pub sections: list_head,
}

#[repr(C)]
pub struct perf_config_section {
    pub sections: list_head,
    pub items: list_head,
    pub name: *mut c_char,
    pub from_system_config: bool,
}

#[repr(C)]
pub struct perf_config_item {
    pub node: list_head,
    pub name: *mut c_char,
    pub value: *mut c_char,
    pub from_system_config: bool,
}

unsafe extern "C" {
    static mut config_exclusive_filename: *const c_char;

    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn getenv(name: *const c_char) -> *mut c_char;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;

    fn mkpath(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> *mut c_char;
    fn perf_etc_perfconfig() -> *const c_char;
    fn perf_config_set__new() -> *mut perf_config_set;
    fn perf_config_set__delete(set: *mut perf_config_set);
    fn perf_config_set__collect(
        set: *mut perf_config_set,
        file_name: *const c_char,
        var: *const c_char,
        value: *const c_char,
    ) -> c_int;
    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn parse_options_usage(
        usagestr: *const *const c_char,
        options: *const option,
        opt: *const c_char,
        short_opt: c_int,
    );
    fn pr_err(format: *const c_char, ...) -> c_int;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
}

static mut USE_SYSTEM_CONFIG: bool = false;
static mut USE_USER_CONFIG: bool = false;

static CONFIG_USAGE_0: &[u8] =
    b"perf config [<file-option>] [options] [section.name[=value] ...]\0";
static CONFIG_USAGE: [*const c_char; 2] = [CONFIG_USAGE_0.as_ptr() as *const c_char, ptr::null()];

static mut ACTIONS: c_uint = 0;

// Original C used OPT_SET_UINT, OPT_BOOLEAN, and OPT_END initializers from
// <subcmd/parse-options.h>. The concrete struct layout is supplied externally.
static mut CONFIG_OPTIONS: [option; 4] = [
    option { _private: [] },
    option { _private: [] },
    option { _private: [] },
    option { _private: [] },
];

unsafe fn list_entry<T>(ptr: *mut list_head, member_offset: usize) -> *mut T {
    (ptr as *mut u8).wrapping_sub(member_offset) as *mut T
}

unsafe fn offsetof_perf_config_section_sections() -> usize {
    core::mem::offset_of!(perf_config_section, sections)
}

unsafe fn offsetof_perf_config_item_node() -> usize {
    core::mem::offset_of!(perf_config_item, node)
}

unsafe fn for_each_section<F>(head: *mut list_head, mut f: F)
where
    F: FnMut(*mut perf_config_section),
{
    let mut pos = unsafe { (*head).next };
    while pos != head {
        let section = unsafe {
            list_entry::<perf_config_section>(pos, offsetof_perf_config_section_sections())
        };
        f(section);
        pos = unsafe { (*pos).next };
    }
}

unsafe fn for_each_item<F>(head: *mut list_head, mut f: F)
where
    F: FnMut(*mut perf_config_item),
{
    let mut pos = unsafe { (*head).next };
    while pos != head {
        let item = unsafe { list_entry::<perf_config_item>(pos, offsetof_perf_config_item_node()) };
        f(item);
        pos = unsafe { (*pos).next };
    }
}

unsafe fn set_config(set: *mut perf_config_set, file_name: *const c_char) -> c_int {
    let first_line = b"# this file is auto-generated.\0".as_ptr() as *const c_char;
    let mut fp: *mut FILE;

    if set.is_null() {
        return -1;
    }

    fp = unsafe { fopen(file_name, b"w\0".as_ptr() as *const c_char) };
    if fp.is_null() {
        return -1;
    }

    unsafe {
        fprintf(fp, b"%s\n\0".as_ptr() as *const c_char, first_line);
    }

    /* overwrite configvariables */
    unsafe {
        for_each_section(&mut (*set).sections, |section| {
            if !USE_SYSTEM_CONFIG && (*section).from_system_config {
                return;
            }
            fprintf(
                fp,
                b"[%s]\n\0".as_ptr() as *const c_char,
                (*section).name,
            );

            for_each_item(&mut (*section).items, |item| {
                if !USE_SYSTEM_CONFIG && (*item).from_system_config {
                    return;
                }
                if !(*item).value.is_null() {
                    fprintf(
                        fp,
                        b"\t%s = %s\n\0".as_ptr() as *const c_char,
                        (*item).name,
                        (*item).value,
                    );
                }
            });
        });
        fclose(fp);
    }

    0
}

unsafe fn show_spec_config(set: *mut perf_config_set, var: *const c_char) -> c_int {
    if set.is_null() {
        return -1;
    }

    unsafe {
        for_each_section(&mut (*set).sections, |section| {
            if !strstarts(var, (*section).name) {
                return;
            }

            for_each_item(&mut (*section).items, |item| {
                let name = var.add(strlen((*section).name) + 1);

                if strcmp(name, (*item).name) == 0 {
                    let value = (*item).value;

                    if !value.is_null() {
                        printf(b"%s=%s\n\0".as_ptr() as *const c_char, var, value);
                    }
                }
            });
        });
    }

    0
}

unsafe fn show_config(set: *mut perf_config_set) -> c_int {
    if set.is_null() {
        return -1;
    }

    unsafe {
        for_each_section(&mut (*set).sections, |section| {
            for_each_item(&mut (*section).items, |item| {
                let value = (*item).value;

                if !value.is_null() {
                    printf(
                        b"%s.%s=%s\n\0".as_ptr() as *const c_char,
                        (*section).name,
                        (*item).name,
                        value,
                    );
                }
            });
        });
    }

    0
}

unsafe fn parse_config_arg(
    mut arg: *mut c_char,
    var: *mut *mut c_char,
    value: *mut *mut c_char,
) -> c_int {
    let last_dot = unsafe { strchr(arg, b'.' as c_int) };

    /*
     * Since "var" actually contains the section name and the real
     * config variable name separated by a dot, we have to know where the dot is.
     */
    if last_dot.is_null() || last_dot == arg {
        unsafe {
            pr_err(
                b"The config variable does not contain a section name: %s\n\0".as_ptr()
                    as *const c_char,
                arg,
            );
        }
        return -1;
    }
    if unsafe { *last_dot.add(1) } == 0 {
        unsafe {
            pr_err(
                b"The config variable does not contain a variable name: %s\n\0".as_ptr()
                    as *const c_char,
                arg,
            );
        }
        return -1;
    }

    unsafe {
        *value = strchr(arg, b'=' as c_int);
    }
    if unsafe { (*value).is_null() } {
        unsafe {
            *var = arg;
        }
    } else if unsafe { strcmp(*value, b"=\0".as_ptr() as *const c_char) == 0 } {
        unsafe {
            pr_err(
                b"The config variable does not contain a value: %s\n\0".as_ptr() as *const c_char,
                arg,
            );
        }
        return -1;
    } else {
        unsafe {
            *value = (*value).add(1); /* excluding a first character '=' */
            *var = strsep(&mut arg, b"=\0".as_ptr() as *const c_char);
            if **var == 0 {
                pr_err(
                    b"invalid config variable: %s\n\0".as_ptr() as *const c_char,
                    arg,
                );
                return -1;
            }
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_config__set_variable(
    var: *const c_char,
    value: *const c_char,
) -> c_int {
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];
    let user_config = unsafe {
        mkpath(
            path.as_mut_ptr(),
            path.len(),
            b"%s/.perfconfig\0".as_ptr() as *const c_char,
            getenv(b"HOME\0".as_ptr() as *const c_char),
        )
    };
    let config_filename: *const c_char;
    let set: *mut perf_config_set;
    let mut ret: c_int = -1;

    unsafe {
        if USE_SYSTEM_CONFIG {
            config_exclusive_filename = perf_etc_perfconfig();
        } else if USE_USER_CONFIG {
            config_exclusive_filename = user_config;
        }

        if config_exclusive_filename.is_null() {
            config_filename = user_config;
        } else {
            config_filename = config_exclusive_filename;
        }

        set = perf_config_set__new();
        if set.is_null() {
            perf_config_set__delete(set);
            return ret;
        }

        if perf_config_set__collect(set, config_filename, var, value) < 0 {
            pr_err(
                b"Failed to add '%s=%s'\n\0".as_ptr() as *const c_char,
                var,
                value,
            );
            perf_config_set__delete(set);
            return ret;
        }

        if set_config(set, config_filename) < 0 {
            pr_err(
                b"Failed to set the configs on %s\n\0".as_ptr() as *const c_char,
                config_filename,
            );
            perf_config_set__delete(set);
            return ret;
        }

        ret = 0;
        perf_config_set__delete(set);
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_config(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut i: c_int;
    let mut ret: c_int = -1;
    let set: *mut perf_config_set;
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];
    let user_config = unsafe {
        mkpath(
            path.as_mut_ptr(),
            path.len(),
            b"%s/.perfconfig\0".as_ptr() as *const c_char,
            getenv(b"HOME\0".as_ptr() as *const c_char),
        )
    };
    let config_filename: *const c_char;
    let mut changed = false;
    let mut argc = argc;

    unsafe {
        argc = parse_options(
            argc,
            argv,
            CONFIG_OPTIONS.as_ptr(),
            CONFIG_USAGE.as_ptr(),
            PARSE_OPT_STOP_AT_NON_OPTION,
        );

        if USE_SYSTEM_CONFIG && USE_USER_CONFIG {
            pr_err(b"Error: only one config file at a time\n\0".as_ptr() as *const c_char);
            parse_options_usage(
                CONFIG_USAGE.as_ptr(),
                CONFIG_OPTIONS.as_ptr(),
                b"user\0".as_ptr() as *const c_char,
                0,
            );
            parse_options_usage(
                ptr::null(),
                CONFIG_OPTIONS.as_ptr(),
                b"system\0".as_ptr() as *const c_char,
                0,
            );
            return -1;
        }

        if USE_SYSTEM_CONFIG {
            config_exclusive_filename = perf_etc_perfconfig();
        } else if USE_USER_CONFIG {
            config_exclusive_filename = user_config;
        }

        if config_exclusive_filename.is_null() {
            config_filename = user_config;
        } else {
            config_filename = config_exclusive_filename;
        }

        /*
         * At only 'config' sub-command, individually use the config set
         * because of reinitializing with options config file location.
         */
        set = perf_config_set__new();
        if set.is_null() {
            perf_config_set__delete(set);
            return ret;
        }

        match ACTIONS {
            ACTION_LIST => {
                if argc != 0 {
                    pr_err(b"Error: takes no arguments\n\0".as_ptr() as *const c_char);
                    parse_options_usage(
                        CONFIG_USAGE.as_ptr(),
                        CONFIG_OPTIONS.as_ptr(),
                        b"l\0".as_ptr() as *const c_char,
                        1,
                    );
                } else if show_config(set) < 0 {
                    pr_err(
                        b"Nothing configured, please check your %s \n\0".as_ptr()
                            as *const c_char,
                        config_filename,
                    );
                    perf_config_set__delete(set);
                    return ret;
                }
            }
            _ => {
                if argc == 0 {
                    if show_config(set) < 0 {
                        pr_err(
                            b"Nothing configured, please check your %s \n\0".as_ptr()
                                as *const c_char,
                            config_filename,
                        );
                        perf_config_set__delete(set);
                        return ret;
                    }
                } else {
                    i = 0;
                    while !(*argv.add(i as usize)).is_null() {
                        let mut var: *mut c_char = ptr::null_mut();
                        let mut value: *mut c_char = ptr::null_mut();
                        let arg = strdup(*argv.add(i as usize));

                        if arg.is_null() {
                            pr_err(b"%s: strdup failed\n\0".as_ptr() as *const c_char, c"cmd_config".as_ptr());
                            perf_config_set__delete(set);
                            return ret;
                        }

                        if parse_config_arg(arg, &mut var, &mut value) < 0 {
                            free(arg as *mut c_void);
                            perf_config_set__delete(set);
                            return ret;
                        }

                        if value.is_null() {
                            if show_spec_config(set, var) < 0 {
                                pr_err(
                                    b"%s is not configured: %s\n\0".as_ptr() as *const c_char,
                                    var,
                                    config_filename,
                                );
                                free(arg as *mut c_void);
                                perf_config_set__delete(set);
                                return ret;
                            }
                        } else {
                            if perf_config_set__collect(set, config_filename, var, value) < 0 {
                                pr_err(
                                    b"Failed to add '%s=%s'\n\0".as_ptr() as *const c_char,
                                    var,
                                    value,
                                );
                                free(arg as *mut c_void);
                                perf_config_set__delete(set);
                                return ret;
                            }
                            changed = true;
                        }
                        free(arg as *mut c_void);
                        i += 1;
                    }

                    if changed && set_config(set, config_filename) < 0 {
                        pr_err(
                            b"Failed to set the configs on %s\n\0".as_ptr() as *const c_char,
                            config_filename,
                        );
                        perf_config_set__delete(set);
                        return ret;
                    }
                }
            }
        }

        ret = 0;
        perf_config_set__delete(set);
    }
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
