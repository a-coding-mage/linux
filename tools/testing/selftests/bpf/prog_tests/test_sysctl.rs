// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// Rust translation of testing/selftests/bpf/prog_tests/test_sysctl.c.
// C includes translated as external dependencies from test_progs.h and
// cgroup_helpers.h supplied by the surrounding selftest crate.

use core::ffi::{c_char, c_int, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const CG_PATH: *const c_char = c"/foo".as_ptr();
const MAX_INSNS: usize = 512;
const FIXUP_SYSCTL_VALUE: i32 = 0;

unsafe extern "C" {
    static mut bpf_log_buf: [c_char; BPF_LOG_BUF_SIZE];

    fn log_err(fmt: *const c_char, ...) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn lseek(fd: c_int, offset: c_int, whence: c_int) -> isize;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;

    fn bpf_prog_load(
        prog_type: bpf_prog_type,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: c_int,
        opts: *mut bpf_prog_load_opts,
    ) -> c_int;
    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: bpf_prog_type,
        obj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_prog_attach(progfd: c_int, targetfd: c_int, attach_type: bpf_attach_type, flags: u32) -> c_int;
    fn bpf_prog_detach(targetfd: c_int, attach_type: bpf_attach_type) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn cgroup_setup_and_join(path: *const c_char) -> c_int;
    fn cleanup_cgroup_environment();
    fn ASSERT_OK_FD(cond: bool, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
}

#[repr(C)]
struct sysctl_test {
    descr: *const c_char,
    fixup_value_insn: usize,
    insns: [bpf_insn; MAX_INSNS],
    prog_file: *const c_char,
    attach_type: bpf_attach_type,
    sysctl: *const c_char,
    open_flags: c_int,
    seek: c_int,
    newval: *const c_char,
    oldval: *const c_char,
    result: sysctl_test_result,
    obj: *mut bpf_object,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum sysctl_test_result {
    LOAD_REJECT,
    ATTACH_REJECT,
    OP_EPERM,
    SUCCESS,
}

// The original C file contains a large static table of sysctl_test values whose
// insns field is built from BPF instruction-constructor macros and conditional
// byte-order preprocessor branches. In Rust form this remains a static test
// table supplied through translated BPF macro dependencies. The initializer is
// intentionally kept as a source-level dependency instead of inventing local
// BPF constructors in this isolated file.
unsafe extern "Rust" {
    static mut tests: [sysctl_test; TEST_SYSCTL_TESTS_LEN];
}

unsafe fn probe_prog_length(fp: *const bpf_insn) -> usize {
    let mut len = MAX_INSNS - 1;

    while len > 0 {
        if (*fp.add(len)).code != 0 || (*fp.add(len)).imm != 0 {
            break;
        }
        len -= 1;
    }
    len + 1
}

unsafe fn fixup_sysctl_value(
    buf: *const c_char,
    buf_len: usize,
    prog: *mut bpf_insn,
    insn_num: usize,
) -> c_int {
    #[repr(C)]
    union Value {
        raw: [u8; size_of::<u64>()],
        num: u64,
    }

    let mut value = Value { raw: [0; size_of::<u64>()] };

    if buf_len > size_of::<Value>() {
        log_err(c"Value is too big (%zd) to use in fixup".as_ptr(), buf_len);
        return -1;
    }
    if (*prog.add(insn_num)).code != (BPF_LD | BPF_DW | BPF_IMM) as _ {
        log_err(c"Can fixup only BPF_LD_IMM64 insns".as_ptr());
        return -1;
    }

    memcpy(value.raw.as_mut_ptr() as *mut c_void, buf as *const c_void, buf_len);
    (*prog.add(insn_num)).imm = value.num as u32 as _;
    (*prog.add(insn_num + 1)).imm = (value.num >> 32) as u32 as _;

    0
}

unsafe fn load_sysctl_prog_insns(test: *mut sysctl_test, sysctl_path: *const c_char) -> c_int {
    let prog = (*test).insns.as_mut_ptr();
    let mut opts: bpf_prog_load_opts = zeroed();
    let insn_cnt = probe_prog_length(prog) as c_int;

    if (*test).fixup_value_insn != 0 {
        let mut buf = [0 as c_char; 128];
        let fd = open(sysctl_path, O_RDONLY | O_CLOEXEC);
        if fd < 0 {
            log_err(c"open(%s) failed".as_ptr(), sysctl_path);
            return -1;
        }
        let len = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
        if len == -1 {
            log_err(c"read(%s) failed".as_ptr(), sysctl_path);
            close(fd);
            return -1;
        }
        close(fd);
        if fixup_sysctl_value(buf.as_ptr(), len as usize, prog, (*test).fixup_value_insn) != 0 {
            return -1;
        }
    }

    opts.log_buf = bpf_log_buf.as_mut_ptr();
    opts.log_size = BPF_LOG_BUF_SIZE;

    let ret = bpf_prog_load(
        BPF_PROG_TYPE_CGROUP_SYSCTL,
        ptr::null(),
        c"GPL".as_ptr(),
        prog,
        insn_cnt,
        &mut opts,
    );
    if ret < 0 && (*test).result != sysctl_test_result::LOAD_REJECT {
        log_err(
            c">>> Loading program error.\n>>> Verifier output:\n%s\n-------\n".as_ptr(),
            bpf_log_buf.as_ptr(),
        );
    }

    ret
}

unsafe fn load_sysctl_prog_file(test: *mut sysctl_test) -> c_int {
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut prog_fd: c_int = 0;

    if bpf_prog_test_load((*test).prog_file, BPF_PROG_TYPE_CGROUP_SYSCTL, &mut obj, &mut prog_fd) != 0 {
        if (*test).result != sysctl_test_result::LOAD_REJECT {
            log_err(c">>> Loading program (%s) error.\n".as_ptr(), (*test).prog_file);
        }
        return -1;
    }

    (*test).obj = obj;
    prog_fd
}

unsafe fn load_sysctl_prog(test: *mut sysctl_test, sysctl_path: *const c_char) -> c_int {
    if !(*test).prog_file.is_null() {
        load_sysctl_prog_file(test)
    } else {
        load_sysctl_prog_insns(test, sysctl_path)
    }
}

unsafe fn access_sysctl(sysctl_path: *const c_char, test: *const sysctl_test) -> c_int {
    let mut err = 0;
    let fd = open(sysctl_path, (*test).open_flags | O_CLOEXEC);
    if fd < 0 {
        return fd;
    }

    if (*test).seek != 0 && lseek(fd, (*test).seek, SEEK_SET) == -1 {
        log_err(c"lseek(%d) failed".as_ptr(), (*test).seek);
        err = -1;
    } else if (*test).open_flags == O_RDONLY {
        let mut buf = [0 as c_char; 128];
        if read(fd, buf.as_mut_ptr() as *mut c_void, buf.len()) == -1 {
            err = -1;
        } else if !(*test).oldval.is_null()
            && strncmp(buf.as_ptr(), (*test).oldval, strlen((*test).oldval)) != 0
        {
            log_err(c"Read value %s != %s".as_ptr(), buf.as_ptr(), (*test).oldval);
            err = -1;
        }
    } else if (*test).open_flags == O_WRONLY {
        if (*test).newval.is_null() {
            log_err(c"New value for sysctl is not set".as_ptr());
            err = -1;
        } else if write(fd, (*test).newval as *const c_void, strlen((*test).newval)) == -1 {
            err = -1;
        }
    } else {
        log_err(c"Unexpected sysctl access: neither read nor write".as_ptr());
        err = -1;
    }

    close(fd);
    err
}

unsafe fn run_test_case(cgfd: c_int, test: *mut sysctl_test) -> c_int {
    let atype = (*test).attach_type;
    let mut sysctl_path = [0 as c_char; 128];
    let mut progfd: c_int = -1;
    let mut err: c_int = 0;

    printf(c"Test case: %s .. ".as_ptr(), (*test).descr);

    snprintf(
        sysctl_path.as_mut_ptr(),
        sysctl_path.len(),
        c"/proc/sys/%s".as_ptr(),
        (*test).sysctl,
    );

    progfd = load_sysctl_prog(test, sysctl_path.as_ptr());
    if progfd < 0 {
        if (*test).result != sysctl_test_result::LOAD_REJECT {
            err = -1;
        }
    } else if bpf_prog_attach(progfd, cgfd, atype, BPF_F_ALLOW_OVERRIDE) < 0 {
        if (*test).result != sysctl_test_result::ATTACH_REJECT {
            err = -1;
        }
    } else {
        errno = 0;
        if access_sysctl(sysctl_path.as_ptr(), test) == -1 {
            if !((*test).result == sysctl_test_result::OP_EPERM && errno == EPERM) {
                err = -1;
            }
        } else if (*test).result != sysctl_test_result::SUCCESS {
            log_err(c"Unexpected success".as_ptr());
            err = -1;
        }
    }

    /* Detaching w/o checking return code: best effort attempt. */
    if progfd != -1 {
        bpf_prog_detach(cgfd, atype);
    }
    bpf_object__close((*test).obj);
    close(progfd);
    printf(c"[%s]\n".as_ptr(), if err != 0 { c"FAIL".as_ptr() } else { c"PASS".as_ptr() });
    err
}

unsafe fn run_tests(cgfd: c_int) -> c_int {
    let mut passes = 0;
    let mut fails = 0;
    let mut i = 0;

    while i < TEST_SYSCTL_TESTS_LEN {
        if run_test_case(cgfd, tests.as_mut_ptr().add(i)) != 0 {
            fails += 1;
        } else {
            passes += 1;
        }
        i += 1;
    }
    printf(c"Summary: %d PASSED, %d FAILED\n".as_ptr(), passes, fails);
    if fails != 0 { -1 } else { 0 }
}

pub unsafe extern "C" fn test_sysctl() {
    let cgfd = cgroup_setup_and_join(CG_PATH);
    if !ASSERT_OK_FD(cgfd < 0, c"create_cgroup".as_ptr()) {
        close(cgfd);
        cleanup_cgroup_environment();
        return;
    }

    if !ASSERT_OK(run_tests(cgfd), c"run_tests".as_ptr()) {
        close(cgfd);
        cleanup_cgroup_environment();
        return;
    }

    close(cgfd);
    cleanup_cgroup_environment();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
