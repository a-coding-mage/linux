// SPDX-License-Identifier: GPL-2.0
//
// C dependencies intentionally preserved as external declarations:
// <error.h>, <errno.h>, <getopt.h>, <stdio.h>, <stdlib.h>, <string.h>,
// <sys/stat.h>, <fcntl.h>, <unistd.h>, <bpf/bpf.h>, <bpf/libbpf.h>,
// and "flow_dissector_load.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

pub const LIBBPF_STRICT_ALL: c_uint = !0;
pub const BPF_FLOW_DISSECTOR: c_int = 7;

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut errno: c_int;

    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn system(command: *const c_char) -> c_int;

    fn libbpf_set_strict_mode(mode: c_uint) -> c_int;
    fn bpf_flow_load(
        obj: *mut *mut bpf_object,
        path: *const c_char,
        section: *const c_char,
        map_name: *const c_char,
        keys: *mut c_void,
        prog_fd: *mut c_int,
        map_fd: *mut c_void,
    ) -> c_int;
    fn bpf_prog_attach(
        prog_fd: c_int,
        target_fd: c_int,
        attach_type: c_int,
        attach_flags: c_uint,
    ) -> c_int;
    fn bpf_object__pin(obj: *mut bpf_object, path: *const c_char) -> c_int;
    fn bpf_prog_detach(target_fd: c_int, attach_type: c_int) -> c_int;
}

static mut cfg_pin_path: *const c_char = b"/sys/fs/bpf/flow_dissector\0".as_ptr() as *const c_char;
static mut cfg_map_name: *const c_char = b"jmp_table\0".as_ptr() as *const c_char;
static mut cfg_attach: bool = true;
static mut cfg_prog_name: *mut c_char = ptr::null_mut();
static mut cfg_path_name: *mut c_char = ptr::null_mut();

unsafe fn load_and_attach_program() {
    let mut prog_fd: c_int = 0;
    let mut ret: c_int;
    let mut obj: *mut bpf_object = ptr::null_mut();

    /* Use libbpf 1.0 API mode */
    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);

    ret = bpf_flow_load(
        &mut obj,
        cfg_path_name,
        cfg_prog_name,
        cfg_map_name,
        ptr::null_mut(),
        &mut prog_fd,
        ptr::null_mut(),
    );
    if ret != 0 {
        error(
            1,
            0,
            b"bpf_flow_load %s\0".as_ptr() as *const c_char,
            cfg_path_name,
        );
    }

    ret = bpf_prog_attach(prog_fd, 0 /* Ignore */, BPF_FLOW_DISSECTOR, 0);
    if ret != 0 {
        error(
            1,
            0,
            b"bpf_prog_attach %s\0".as_ptr() as *const c_char,
            cfg_path_name,
        );
    }

    ret = bpf_object__pin(obj, cfg_pin_path);
    if ret != 0 {
        error(
            1,
            0,
            b"bpf_object__pin %s\0".as_ptr() as *const c_char,
            cfg_pin_path,
        );
    }
}

unsafe fn detach_program() {
    let mut command: [c_char; 64] = [0; 64];
    let mut ret: c_int;

    ret = bpf_prog_detach(0, BPF_FLOW_DISSECTOR);
    if ret != 0 {
        error(1, 0, b"bpf_prog_detach\0".as_ptr() as *const c_char);
    }

    /* To unpin, it is necessary and sufficient to just remove this dir */
    sprintf(
        command.as_mut_ptr(),
        b"rm -r %s\0".as_ptr() as *const c_char,
        cfg_pin_path,
    );
    ret = system(command.as_ptr());
    if ret != 0 {
        error(
            1,
            errno,
            b"%s\0".as_ptr() as *const c_char,
            command.as_ptr(),
        );
    }
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let mut attach: bool = false;
    let mut detach: bool = false;
    let mut c: c_int;

    loop {
        c = getopt(argc, argv, b"adp:s:\0".as_ptr() as *const c_char);
        if c == -1 {
            break;
        }

        match c {
            c if c == b'a' as c_int => {
                if detach {
                    error(
                        1,
                        0,
                        b"attach/detach are exclusive\0".as_ptr() as *const c_char,
                    );
                }
                attach = true;
            }
            c if c == b'd' as c_int => {
                if attach {
                    error(
                        1,
                        0,
                        b"attach/detach are exclusive\0".as_ptr() as *const c_char,
                    );
                }
                detach = true;
            }
            c if c == b'p' as c_int => {
                if !cfg_path_name.is_null() {
                    error(
                        1,
                        0,
                        b"only one path can be given\0".as_ptr() as *const c_char,
                    );
                }

                cfg_path_name = optarg;
            }
            c if c == b's' as c_int => {
                if !cfg_prog_name.is_null() {
                    error(
                        1,
                        0,
                        b"only one prog can be given\0".as_ptr() as *const c_char,
                    );
                }

                cfg_prog_name = optarg;
            }
            _ => {}
        }
    }

    if detach {
        cfg_attach = false;
    }

    if cfg_attach && cfg_path_name.is_null() {
        error(
            1,
            0,
            b"must provide a path to the BPF program\0".as_ptr() as *const c_char,
        );
    }

    if cfg_attach && cfg_prog_name.is_null() {
        error(
            1,
            0,
            b"must provide a section name\0".as_ptr() as *const c_char,
        );
    }
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    parse_opts(argc, argv);
    if cfg_attach {
        load_and_attach_program();
    } else {
        detach_program();
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
