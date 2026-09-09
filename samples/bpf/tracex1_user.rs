// SPDX-License-Identifier: GPL-2.0

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::ptr;

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> i64;
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__close(obj: *mut bpf_object);
    fn popen(command: *const c_char, mode: *const c_char) -> *mut FILE;
    fn read_trace_pipe();
}

pub unsafe fn main(ac: c_int, argv: *mut *mut c_char) -> c_int {
    let mut link: *mut bpf_link = ptr::null_mut();
    let prog: *mut bpf_program;
    let obj: *mut bpf_object;
    let mut filename = [0u8; 256];
    let f: *mut FILE;

    let argv0 = if ac > 0 && !argv.is_null() && !(*argv).is_null() {
        CStr::from_ptr(*argv).to_bytes()
    } else {
        b""
    };
    let suffix = b".bpf.o\0";
    let copy_len = std::cmp::min(
        argv0.len().saturating_add(suffix.len() - 1),
        filename.len() - 1,
    );
    let argv_copy_len = std::cmp::min(argv0.len(), copy_len);
    filename[..argv_copy_len].copy_from_slice(&argv0[..argv_copy_len]);
    let suffix_copy_len = std::cmp::min(
        suffix.len() - 1,
        filename.len() - 1 - argv_copy_len,
    );
    filename[argv_copy_len..argv_copy_len + suffix_copy_len]
        .copy_from_slice(&suffix[..suffix_copy_len]);
    filename[argv_copy_len + suffix_copy_len] = 0;

    obj = bpf_object__open_file(filename.as_ptr() as *const c_char, ptr::null());
    if libbpf_get_error(obj as *const c_void) != 0 {
        eprintln!("ERROR: opening BPF object file failed");
        return 0;
    }

    let program_name = CString::new("bpf_prog1").unwrap();
    prog = bpf_object__find_program_by_name(obj, program_name.as_ptr());
    if prog.is_null() {
        eprintln!("ERROR: finding a prog in obj file failed");
        goto_cleanup(link, obj);
        return 0;
    }

    /* load BPF program */
    if bpf_object__load(obj) != 0 {
        eprintln!("ERROR: loading BPF object file failed");
        goto_cleanup(link, obj);
        return 0;
    }

    link = bpf_program__attach(prog);
    if libbpf_get_error(link as *const c_void) != 0 {
        eprintln!("ERROR: bpf_program__attach failed");
        link = ptr::null_mut();
        goto_cleanup(link, obj);
        return 0;
    }

    let command = CString::new("taskset 1 ping -c5 localhost").unwrap();
    let mode = CString::new("r").unwrap();
    f = popen(command.as_ptr(), mode.as_ptr());
    let _ = f;

    read_trace_pipe();

    goto_cleanup(link, obj);
    0
}

unsafe fn goto_cleanup(link: *mut bpf_link, obj: *mut bpf_object) {
    bpf_link__destroy(link);
    bpf_object__close(obj);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
