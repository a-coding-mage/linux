// SPDX-License-Identifier: GPL-2.0
// C dependencies: stdio.h, stdlib.h, unistd.h, linux/filter.h,
// linux/seccomp.h, sys/prctl.h, bpf/bpf.h, bpf/libbpf.h,
// trace_helpers.h, and bpf_util.h.

use std::ffi::{c_char, c_int, c_void, CStr};

#[repr(C)]
struct SockFilter {
    code: u16,
    jt: u8,
    jf: u8,
    k: u32,
}

#[repr(C)]
struct SockFprog {
    len: u16,
    filter: *mut SockFilter,
}

#[repr(C)]
struct BpfLink;
#[repr(C)]
struct BpfProgram;
#[repr(C)]
struct BpfObject;
#[repr(C)]
struct File;

extern "C" {
    fn prctl(option: c_int, ...) -> c_int;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut File, format: *const c_char, ...);
    fn printf(format: *const c_char, ...);
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ... ) -> c_int;
    fn popen(command: *const c_char, mode: *const c_char) -> *mut File;
    fn read_trace_pipe();

    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut BpfObject;
    fn libbpf_get_error(ptr: *const c_void) -> isize;
    fn bpf_object__find_program_by_name(obj: *mut BpfObject, name: *const c_char) -> *mut BpfProgram;
    fn bpf_object__load(obj: *mut BpfObject) -> c_int;
    fn bpf_program__attach(prog: *mut BpfProgram) -> *mut BpfLink;
    fn bpf_object__find_map_fd_by_name(obj: *mut BpfObject, name: *const c_char) -> c_int;
    fn bpf_object__next_program(obj: *mut BpfObject, prev: *mut BpfProgram) -> *mut BpfProgram;
    fn bpf_program__section_name(prog: *mut BpfProgram) -> *const c_char;
    fn bpf_program__fd(prog: *mut BpfProgram) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_link__destroy(link: *mut BpfLink) -> c_int;
    fn bpf_object__close(obj: *mut BpfObject);
}

#[cfg(target_arch = "mips64")]
const MAX_ENTRIES: usize = 6000; // MIPS n64 syscalls start at 5000
#[cfg(not(target_arch = "mips64"))]
const MAX_ENTRIES: usize = 1024;

unsafe fn install_accept_all_seccomp() {
    // BPF_STMT(BPF_RET+BPF_K, SECCOMP_RET_ALLOW)
    let mut filter = [SockFilter {
        code: 0x06,
        jt: 0,
        jf: 0,
        k: 0x7fff0000,
    }];
    let prog = SockFprog {
        len: filter.len() as u16,
        filter: filter.as_mut_ptr(),
    };
    // PR_SET_SECCOMP = 22, SECCOMP_MODE_FILTER = 2
    if prctl(22, 2, &prog as *const SockFprog) != 0 {
        perror(b"prctl\0".as_ptr() as *const c_char);
    }
}

fn main() {
    unsafe {
        let args: Vec<*const c_char> = std::env::args_os()
            .map(|arg| std::ffi::CString::new(arg.as_encoded_bytes()).unwrap().into_raw() as *const c_char)
            .collect();
        let ac = args.len() as c_int;
        let argv = args.as_ptr() as *mut *mut c_char;
        let _ = ac;

        let mut link: *mut BpfLink = std::ptr::null_mut();
        let mut prog: *mut BpfProgram;
        let obj: *mut BpfObject;
        let mut key: c_int = 0;
        let mut fd: c_int;
        let progs_fd: c_int;
        let mut filename = [0i8; 256];
        let f: *mut File;

        snprintf(filename.as_mut_ptr(), filename.len(), b"%s.bpf.o\0".as_ptr() as *const c_char, *argv);
        obj = bpf_object__open_file(filename.as_ptr(), std::ptr::null());
        if libbpf_get_error(obj as *const c_void) != 0 {
            fprintf(std::ptr::null_mut(), b"ERROR: opening BPF object file failed\n\0".as_ptr() as *const c_char);
            return;
        }

        prog = bpf_object__find_program_by_name(obj, b"bpf_prog1\0".as_ptr() as *const c_char);
        if prog.is_null() {
            printf(b"finding a prog in obj file failed\n\0".as_ptr() as *const c_char);
            bpf_object__close(obj);
            return;
        }
        if bpf_object__load(obj) != 0 {
            fprintf(std::ptr::null_mut(), b"ERROR: loading BPF object file failed\n\0".as_ptr() as *const c_char);
            bpf_link__destroy(link);
            bpf_object__close(obj);
            return;
        }
        link = bpf_program__attach(prog);
        if libbpf_get_error(link as *const c_void) != 0 {
            fprintf(std::ptr::null_mut(), b"ERROR: bpf_program__attach failed\n\0".as_ptr() as *const c_char);
            link = std::ptr::null_mut();
            bpf_object__close(obj);
            return;
        }
        progs_fd = bpf_object__find_map_fd_by_name(obj, b"progs\0".as_ptr() as *const c_char);
        if progs_fd < 0 {
            fprintf(std::ptr::null_mut(), b"ERROR: finding a map in obj file failed\n\0".as_ptr() as *const c_char);
            bpf_link__destroy(link);
            bpf_object__close(obj);
            return;
        }

        prog = std::ptr::null_mut();
        while !(prog = bpf_object__next_program(obj, prog)).is_null() {
            let section = bpf_program__section_name(prog);
            if section.is_null() { continue; }
            let section = CStr::from_ptr(section).to_bytes();
            if !section.starts_with(b"kprobe/") || section[7..].parse::<c_int>().map(|v| key = v).is_err() { continue; }
            fd = bpf_program__fd(prog);
            bpf_map_update_elem(progs_fd, &key as *const c_int as *const c_void, &fd as *const c_int as *const c_void, 0);
        }
        install_accept_all_seccomp();
        f = popen(b"dd if=/dev/zero of=/dev/null count=5\0".as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
        let _ = f;
        read_trace_pipe();
        bpf_link__destroy(link);
        bpf_object__close(obj);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
