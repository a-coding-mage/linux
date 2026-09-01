// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2020 Facebook

// C dependencies from the original source:
// errno.h, unistd.h, linux/err.h, bpf/libbpf.h, and "main.h".
// The original file defines _GNU_SOURCE when not already defined.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_attach_opts {
    pub sz: usize,
    pub link_info: *mut bpf_iter_link_info,
    pub link_info_len: u32,
}

#[repr(C)]
pub struct bpf_iter_link_info_map {
    pub map_fd: c_uint,
}

#[repr(C)]
pub union bpf_iter_link_info {
    pub map: bpf_iter_link_info_map,
}

#[repr(C)]
pub struct cmd {
    pub cmd: *const c_char,
    pub func: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
}

const BPF_F_RDONLY: c_int = 8;

unsafe extern "C" {
    static mut errno: c_int;
    static mut bin_name: *const c_char;

    fn usage() -> !;
    fn is_prefix(str_: *const c_char, prefix: *const c_char) -> bool;
    fn map_parse_fd(argc: *mut c_int, argv: *mut *mut *mut c_char, flags: c_int) -> c_int;
    fn mount_bpffs_for_file(file: *const c_char) -> c_int;
    fn cmd_select(
        cmds: *const cmd,
        argc: c_int,
        argv: *mut *mut c_char,
        help: unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int,
    ) -> c_int;

    fn bpf_object__open(path: *const c_char) -> *mut bpf_object;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn bpf_object__next_program(
        obj: *const bpf_object,
        prev: *mut bpf_program,
    ) -> *mut bpf_program;
    fn bpf_program__attach_iter(
        prog: *const bpf_program,
        opts: *const bpf_iter_attach_opts,
    ) -> *mut bpf_link;
    fn bpf_program__name(prog: *const bpf_program) -> *const c_char;
    fn bpf_link__pin(link: *mut bpf_link, path: *const c_char) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn close(fd: c_int) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn p_err(format: *const c_char, ...);

    static mut stderr: *mut c_void;
}

unsafe fn req_args(argc: c_int, required: c_int) -> bool {
    argc >= required
}

unsafe fn get_arg(argc: &mut c_int, argv: &mut *mut *mut c_char) -> *mut c_char {
    let arg = **argv;
    *argv = (*argv).add(1);
    *argc -= 1;
    arg
}

unsafe fn next_arg(argc: &mut c_int, argv: &mut *mut *mut c_char) {
    *argv = (*argv).add(1);
    *argc -= 1;
}

unsafe extern "C" fn do_pin(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut iter_opts = bpf_iter_attach_opts {
        sz: core::mem::size_of::<bpf_iter_attach_opts>(),
        link_info: core::ptr::null_mut(),
        link_info_len: 0,
    };
    let mut linfo = core::mem::MaybeUninit::<bpf_iter_link_info>::uninit();
    let objfile: *const c_char;
    let path: *const c_char;
    let mut prog: *mut bpf_program;
    let mut obj: *mut bpf_object;
    let mut link: *mut bpf_link;
    let mut err: c_int = -1;
    let mut map_fd: c_int = -1;

    if !req_args(argc, 2) {
        usage();
    }

    objfile = get_arg(&mut argc, &mut argv);
    path = get_arg(&mut argc, &mut argv);

    /* optional arguments */
    if argc != 0 {
        if is_prefix(*argv, c"map".as_ptr()) {
            next_arg(&mut argc, &mut argv);

            if !req_args(argc, 2) {
                p_err(c"incorrect map spec".as_ptr());
                return -1;
            }

            map_fd = map_parse_fd(&mut argc, &mut argv, BPF_F_RDONLY);
            if map_fd < 0 {
                return -1;
            }

            memset(
                linfo.as_mut_ptr() as *mut c_void,
                0,
                core::mem::size_of::<bpf_iter_link_info>(),
            );
            (*linfo.as_mut_ptr()).map.map_fd = map_fd as c_uint;
            iter_opts.link_info = linfo.as_mut_ptr();
            iter_opts.link_info_len = core::mem::size_of::<bpf_iter_link_info>() as u32;
        }
    }

    obj = bpf_object__open(objfile);
    if obj.is_null() {
        err = -errno;
        p_err(c"can't open objfile %s".as_ptr(), objfile);
        if map_fd >= 0 {
            close(map_fd);
        }
        return err;
    }

    err = bpf_object__load(obj);
    if err != 0 {
        p_err(c"can't load objfile %s".as_ptr(), objfile);
        bpf_object__close(obj);
        if map_fd >= 0 {
            close(map_fd);
        }
        return err;
    }

    prog = bpf_object__next_program(obj, core::ptr::null_mut());
    if prog.is_null() {
        err = -errno;
        p_err(c"can't find bpf program in objfile %s".as_ptr(), objfile);
        bpf_object__close(obj);
        if map_fd >= 0 {
            close(map_fd);
        }
        return err;
    }

    link = bpf_program__attach_iter(prog, &iter_opts);
    if link.is_null() {
        err = -errno;
        p_err(
            c"attach_iter failed for program %s".as_ptr(),
            bpf_program__name(prog),
        );
        bpf_object__close(obj);
        if map_fd >= 0 {
            close(map_fd);
        }
        return err;
    }

    err = mount_bpffs_for_file(path);
    if err == 0 {
        err = bpf_link__pin(link, path);
        if err != 0 {
            p_err(
                c"pin_iter failed for program %s to path %s".as_ptr(),
                bpf_program__name(prog),
                path,
            );
        }
    }

    bpf_link__destroy(link);
    bpf_object__close(obj);
    if map_fd >= 0 {
        close(map_fd);
    }
    err
}

unsafe extern "C" fn do_help(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    fprintf(
        stderr,
        c"Usage: %1$s %2$s pin OBJ PATH [map MAP]\n       %1$s %2$s help\n\n       %s\n       %s }\n".as_ptr(),
        bin_name,
        c"iter".as_ptr(),
        c"HELP_SPEC_MAP".as_ptr(),
        c"HELP_SPEC_OPTIONS".as_ptr(),
    );

    0
}

static CMDS: [cmd; 3] = [
    cmd {
        cmd: c"help".as_ptr(),
        func: Some(do_help),
    },
    cmd {
        cmd: c"pin".as_ptr(),
        func: Some(do_pin),
    },
    cmd {
        cmd: core::ptr::null(),
        func: None,
    },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_iter(argc: c_int, argv: *mut *mut c_char) -> c_int {
    cmd_select(CMDS.as_ptr(), argc, argv, do_help)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
