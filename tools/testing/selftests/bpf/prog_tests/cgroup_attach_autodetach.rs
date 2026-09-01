// SPDX-License-Identifier: GPL-2.0

// Dependencies from test_progs.h and cgroup_helpers.h are declared externally.

use core::ffi::{c_char, c_int, c_uint, c_void};

const PING_CMD: &[u8] = b"ping -q -c1 -w1 127.0.0.1 > /dev/null\0";

static mut bpf_log_buf: [c_char; BPF_LOG_BUF_SIZE as usize] =
    [0; BPF_LOG_BUF_SIZE as usize];

#[repr(C)]
pub struct bpf_insn {
    _private: [u8; 0],
}

type __u32 = u32;

extern "C" {
    static mut errno: c_int;

    static BPF_LOG_BUF_SIZE: usize;
    static BPF_PROG_TYPE_CGROUP_SKB: c_uint;
    static BPF_REG_0: c_uint;
    static BPF_CGROUP_INET_EGRESS: c_uint;
    static BPF_F_ALLOW_MULTI: c_uint;

    fn BPF_MOV64_IMM(dst: c_uint, imm: c_int) -> bpf_insn;
    fn BPF_EXIT_INSN() -> bpf_insn;

    fn bpf_test_load_program(
        type_: c_uint,
        insns: *const bpf_insn,
        insns_cnt: usize,
        license: *const c_char,
        kern_version: c_uint,
        log_buf: *mut c_char,
        log_buf_sz: usize,
    ) -> c_int;
    fn setup_cgroup_environment() -> c_int;
    fn create_and_get_cgroup(path: *const c_char) -> c_int;
    fn join_cgroup(path: *const c_char) -> c_int;
    fn bpf_prog_attach(
        prog_fd: c_int,
        target_fd: c_int,
        attach_type: c_uint,
        attach_flags: c_uint,
    ) -> c_int;
    fn bpf_prog_query(
        target_fd: c_int,
        attach_type: c_uint,
        query_flags: c_uint,
        attach_flags: *mut __u32,
        prog_ids: *mut __u32,
        prog_cnt: *mut __u32,
    ) -> c_int;
    fn bpf_prog_get_fd_by_id(id: __u32) -> c_int;
    fn cleanup_cgroup_environment();

    fn CHECK(condition: bool, name: *const c_char, format: *const c_char, ...) -> bool;
    fn CHECK_FAIL(condition: bool) -> bool;

    fn system(command: *const c_char) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
}

unsafe fn prog_load() -> c_int {
    let prog = [
        BPF_MOV64_IMM(BPF_REG_0, 1), /* r0 = 1 */
        BPF_EXIT_INSN(),
    ];
    let insns_cnt = prog.len();

    bpf_test_load_program(
        BPF_PROG_TYPE_CGROUP_SKB,
        prog.as_ptr(),
        insns_cnt,
        b"GPL\0".as_ptr() as *const c_char,
        0,
        bpf_log_buf.as_mut_ptr(),
        BPF_LOG_BUF_SIZE,
    )
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_cgroup_attach_autodetach() {
    let mut duration: __u32 = 0;
    let mut prog_cnt: __u32 = 4;
    let mut attach_flags: __u32 = 0;
    let mut allow_prog: [c_int; 2] = [-1; 2];
    let mut prog_ids: [__u32; 2] = [0; 2];
    let mut ptr: *mut c_void = core::ptr::null_mut();
    let mut cg: c_int = 0;
    let mut i: c_int;
    let mut attempts: c_int;

    'err: loop {
        i = 0;
        while (i as usize) < allow_prog.len() {
            allow_prog[i as usize] = prog_load();
            if CHECK(
                allow_prog[i as usize] < 0,
                b"prog_load\0".as_ptr() as *const c_char,
                b"verifier output:\n%s\n-------\n\0".as_ptr() as *const c_char,
                bpf_log_buf.as_ptr(),
            ) {
                break 'err;
            }
            i += 1;
        }

        if CHECK_FAIL(setup_cgroup_environment() != 0) {
            break 'err;
        }

        /* create a cgroup, attach two programs and remember their ids */
        cg = create_and_get_cgroup(b"/cg_autodetach\0".as_ptr() as *const c_char);
        if CHECK_FAIL(cg < 0) {
            break 'err;
        }

        if CHECK_FAIL(join_cgroup(b"/cg_autodetach\0".as_ptr() as *const c_char) != 0) {
            break 'err;
        }

        i = 0;
        while (i as usize) < allow_prog.len() {
            if CHECK(
                bpf_prog_attach(
                    allow_prog[i as usize],
                    cg,
                    BPF_CGROUP_INET_EGRESS,
                    BPF_F_ALLOW_MULTI,
                ) != 0,
                b"prog_attach\0".as_ptr() as *const c_char,
                b"prog[%d], errno=%d\n\0".as_ptr() as *const c_char,
                i,
                errno,
            ) {
                break 'err;
            }
            i += 1;
        }

        /* make sure that programs are attached and run some traffic */
        if CHECK(
            bpf_prog_query(
                cg,
                BPF_CGROUP_INET_EGRESS,
                0,
                &mut attach_flags,
                prog_ids.as_mut_ptr(),
                &mut prog_cnt,
            ) != 0,
            b"prog_query\0".as_ptr() as *const c_char,
            b"errno=%d\n\0".as_ptr() as *const c_char,
            errno,
        ) {
            break 'err;
        }
        if CHECK_FAIL(system(PING_CMD.as_ptr() as *const c_char) != 0) {
            break 'err;
        }

        /* allocate some memory (4Mb) to pin the original cgroup */
        ptr = malloc(4 * (1 << 20));
        if CHECK_FAIL(ptr.is_null()) {
            break 'err;
        }

        /* close programs and cgroup fd */
        i = 0;
        while (i as usize) < allow_prog.len() {
            close(allow_prog[i as usize]);
            allow_prog[i as usize] = -1;
            i += 1;
        }

        close(cg);
        cg = 0;

        /* leave the cgroup and remove it. don't detach programs */
        cleanup_cgroup_environment();

        /* wait for the asynchronous auto-detachment.
         * wait for no more than 5 sec and give up.
         */
        i = 0;
        while (i as usize) < prog_ids.len() {
            attempts = 5;
            while attempts >= 0 {
                let fd = bpf_prog_get_fd_by_id(prog_ids[i as usize]);

                if fd < 0 {
                    break;
                }

                /* don't leave the fd open */
                close(fd);

                if CHECK_FAIL(attempts == 0) {
                    break 'err;
                }

                sleep(1);
                attempts -= 1;
            }
            i += 1;
        }

        break;
    }

    i = 0;
    while (i as usize) < allow_prog.len() {
        if allow_prog[i as usize] >= 0 {
            close(allow_prog[i as usize]);
        }
        i += 1;
    }
    if cg != 0 {
        close(cg);
    }
    free(ptr);
    cleanup_cgroup_environment();

    let _ = duration;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
