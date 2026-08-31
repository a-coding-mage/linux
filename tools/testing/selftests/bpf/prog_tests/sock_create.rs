// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/sock_create.c
// Dependencies from <linux/bpf.h>, <test_progs.h>, and "cgroup_helpers.h"
// are expected to be supplied by the surrounding test harness.

static mut bpf_log_buf: [::std::os::raw::c_char; 4096] = [0; 4096];
static mut verbose: bool = false;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum sock_create_test_error {
    OK = 0,
    DENY_CREATE,
}

#[repr(C)]
struct sock_create_test {
    descr: *const ::std::os::raw::c_char,
    insns: [bpf_insn; 64],
    attach_type: bpf_attach_type,
    expected_attach_type: bpf_attach_type,

    domain: ::std::os::raw::c_int,
    type_: ::std::os::raw::c_int,
    protocol: ::std::os::raw::c_int,

    optname: ::std::os::raw::c_int,
    optval: ::std::os::raw::c_int,
    error: sock_create_test_error,
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const ::std::os::raw::c_char
    };
}

macro_rules! insns64 {
    ($($insn:expr),* $(,)?) => {{
        let mut insns: [bpf_insn; 64] = unsafe { ::std::mem::zeroed() };
        let init = [$($insn),*];
        let mut i = 0;
        while i < init.len() {
            insns[i] = init[i];
            i += 1;
        }
        insns
    }};
}

static mut tests: [sock_create_test; 9] = [
    sock_create_test {
        descr: c_str!("AF_INET set priority"),
        insns: insns64![
            /* r3 = 123 (priority) */
            BPF_MOV64_IMM(BPF_REG_3, 123),
            BPF_STX_MEM(
                BPF_W,
                BPF_REG_1,
                BPF_REG_3,
                offset_of!(bpf_sock, priority) as ::std::os::raw::c_int,
            ),

            /* return 1 */
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET_SOCK_CREATE,
        attach_type: BPF_CGROUP_INET_SOCK_CREATE,

        domain: AF_INET,
        type_: SOCK_DGRAM,
        protocol: 0,

        optname: SO_PRIORITY,
        optval: 123,
        error: sock_create_test_error::OK,
    },
    sock_create_test {
        descr: c_str!("AF_INET6 set priority"),
        insns: insns64![
            /* r3 = 123 (priority) */
            BPF_MOV64_IMM(BPF_REG_3, 123),
            BPF_STX_MEM(
                BPF_W,
                BPF_REG_1,
                BPF_REG_3,
                offset_of!(bpf_sock, priority) as ::std::os::raw::c_int,
            ),

            /* return 1 */
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET_SOCK_CREATE,
        attach_type: BPF_CGROUP_INET_SOCK_CREATE,

        domain: AF_INET6,
        type_: SOCK_DGRAM,
        protocol: 0,

        optname: SO_PRIORITY,
        optval: 123,
        error: sock_create_test_error::OK,
    },
    sock_create_test {
        descr: c_str!("AF_INET set mark"),
        insns: insns64![
            BPF_MOV64_REG(BPF_REG_6, BPF_REG_1),

            /* get uid of process */
            BPF_EMIT_CALL(BPF_FUNC_get_current_uid_gid),
            BPF_ALU64_IMM(BPF_AND, BPF_REG_0, 0xffffffff),

            /* if uid is 0, use given mark(666), else use uid as the mark */
            BPF_MOV64_REG(BPF_REG_3, BPF_REG_0),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_0, 0, 1),
            BPF_MOV64_IMM(BPF_REG_3, 666),

            BPF_MOV64_REG(BPF_REG_1, BPF_REG_6),
            BPF_STX_MEM(
                BPF_W,
                BPF_REG_1,
                BPF_REG_3,
                offset_of!(bpf_sock, mark) as ::std::os::raw::c_int,
            ),

            /* return 1 */
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET_SOCK_CREATE,
        attach_type: BPF_CGROUP_INET_SOCK_CREATE,

        domain: AF_INET,
        type_: SOCK_DGRAM,
        protocol: 0,

        optname: SO_MARK,
        optval: 666,
        error: sock_create_test_error::OK,
    },
    sock_create_test {
        descr: c_str!("AF_INET6 set mark"),
        insns: insns64![
            BPF_MOV64_REG(BPF_REG_6, BPF_REG_1),

            /* get uid of process */
            BPF_EMIT_CALL(BPF_FUNC_get_current_uid_gid),
            BPF_ALU64_IMM(BPF_AND, BPF_REG_0, 0xffffffff),

            /* if uid is 0, use given mark(666), else use uid as the mark */
            BPF_MOV64_REG(BPF_REG_3, BPF_REG_0),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_0, 0, 1),
            BPF_MOV64_IMM(BPF_REG_3, 666),

            BPF_MOV64_REG(BPF_REG_1, BPF_REG_6),
            BPF_STX_MEM(
                BPF_W,
                BPF_REG_1,
                BPF_REG_3,
                offset_of!(bpf_sock, mark) as ::std::os::raw::c_int,
            ),

            /* return 1 */
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET_SOCK_CREATE,
        attach_type: BPF_CGROUP_INET_SOCK_CREATE,

        domain: AF_INET6,
        type_: SOCK_DGRAM,
        protocol: 0,

        optname: SO_MARK,
        optval: 666,
        error: sock_create_test_error::OK,
    },
    sock_create_test {
        descr: c_str!("AF_INET bound to iface"),
        insns: insns64![
            /* r3 = 1 (lo interface) */
            BPF_MOV64_IMM(BPF_REG_3, 1),
            BPF_STX_MEM(
                BPF_W,
                BPF_REG_1,
                BPF_REG_3,
                offset_of!(bpf_sock, bound_dev_if) as ::std::os::raw::c_int,
            ),

            /* return 1 */
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET_SOCK_CREATE,
        attach_type: BPF_CGROUP_INET_SOCK_CREATE,

        domain: AF_INET,
        type_: SOCK_DGRAM,
        protocol: 0,

        optname: SO_BINDTOIFINDEX,
        optval: 1,
        error: sock_create_test_error::OK,
    },
    sock_create_test {
        descr: c_str!("AF_INET6 bound to iface"),
        insns: insns64![
            /* r3 = 1 (lo interface) */
            BPF_MOV64_IMM(BPF_REG_3, 1),
            BPF_STX_MEM(
                BPF_W,
                BPF_REG_1,
                BPF_REG_3,
                offset_of!(bpf_sock, bound_dev_if) as ::std::os::raw::c_int,
            ),

            /* return 1 */
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET_SOCK_CREATE,
        attach_type: BPF_CGROUP_INET_SOCK_CREATE,

        domain: AF_INET6,
        type_: SOCK_DGRAM,
        protocol: 0,

        optname: SO_BINDTOIFINDEX,
        optval: 1,
        error: sock_create_test_error::OK,
    },
    sock_create_test {
        descr: c_str!("block AF_INET, SOCK_DGRAM, IPPROTO_ICMP socket"),
        insns: insns64![
            BPF_MOV64_IMM(BPF_REG_0, 1), /* r0 = verdict */

            /* sock->family == AF_INET */
            BPF_LDX_MEM(
                BPF_H,
                BPF_REG_2,
                BPF_REG_1,
                offset_of!(bpf_sock, family) as ::std::os::raw::c_int,
            ),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_2, AF_INET, 5),

            /* sock->type == SOCK_DGRAM */
            BPF_LDX_MEM(
                BPF_H,
                BPF_REG_2,
                BPF_REG_1,
                offset_of!(bpf_sock, type_) as ::std::os::raw::c_int,
            ),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_2, SOCK_DGRAM, 3),

            /* sock->protocol == IPPROTO_ICMP */
            BPF_LDX_MEM(
                BPF_H,
                BPF_REG_2,
                BPF_REG_1,
                offset_of!(bpf_sock, protocol) as ::std::os::raw::c_int,
            ),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_2, IPPROTO_ICMP, 1),

            /* return 0 (block) */
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET_SOCK_CREATE,
        attach_type: BPF_CGROUP_INET_SOCK_CREATE,

        domain: AF_INET,
        type_: SOCK_DGRAM,
        protocol: IPPROTO_ICMP,

        optname: 0,
        optval: 0,
        error: sock_create_test_error::DENY_CREATE,
    },
    sock_create_test {
        descr: c_str!("block AF_INET6, SOCK_DGRAM, IPPROTO_ICMPV6 socket"),
        insns: insns64![
            BPF_MOV64_IMM(BPF_REG_0, 1), /* r0 = verdict */

            /* sock->family == AF_INET6 */
            BPF_LDX_MEM(
                BPF_H,
                BPF_REG_2,
                BPF_REG_1,
                offset_of!(bpf_sock, family) as ::std::os::raw::c_int,
            ),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_2, AF_INET6, 5),

            /* sock->type == SOCK_DGRAM */
            BPF_LDX_MEM(
                BPF_H,
                BPF_REG_2,
                BPF_REG_1,
                offset_of!(bpf_sock, type_) as ::std::os::raw::c_int,
            ),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_2, SOCK_DGRAM, 3),

            /* sock->protocol == IPPROTO_ICMPV6 */
            BPF_LDX_MEM(
                BPF_H,
                BPF_REG_2,
                BPF_REG_1,
                offset_of!(bpf_sock, protocol) as ::std::os::raw::c_int,
            ),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_2, IPPROTO_ICMPV6, 1),

            /* return 0 (block) */
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET_SOCK_CREATE,
        attach_type: BPF_CGROUP_INET_SOCK_CREATE,

        domain: AF_INET,
        type_: SOCK_DGRAM,
        protocol: IPPROTO_ICMPV6,

        optname: 0,
        optval: 0,
        error: sock_create_test_error::DENY_CREATE,
    },
    sock_create_test {
        descr: c_str!("load w/o expected_attach_type (compat mode)"),
        insns: insns64![
            /* return 1 */
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: 0,
        attach_type: BPF_CGROUP_INET_SOCK_CREATE,

        domain: AF_INET,
        type_: SOCK_STREAM,
        protocol: 0,

        optname: 0,
        optval: 0,
        error: sock_create_test_error::OK,
    },
];

unsafe fn load_prog(
    insns: *const bpf_insn,
    expected_attach_type: bpf_attach_type,
) -> ::std::os::raw::c_int {
    let mut opts = bpf_prog_load_opts {
        expected_attach_type,
        log_level: 2,
        log_buf: bpf_log_buf.as_mut_ptr(),
        log_size: ::std::mem::size_of_val(&bpf_log_buf) as _,
        ..::std::mem::zeroed()
    };
    let mut insns_cnt: ::std::os::raw::c_int = 0;

    while (*insns.offset(insns_cnt as isize)).code != (BPF_JMP | BPF_EXIT) {
        insns_cnt += 1;
    }
    insns_cnt += 1;

    let fd = bpf_prog_load(
        BPF_PROG_TYPE_CGROUP_SOCK,
        ::std::ptr::null(),
        c_str!("GPL"),
        insns,
        insns_cnt,
        &mut opts,
    );
    if verbose && fd < 0 {
        fprintf(stderr, c_str!("%s\n"), bpf_log_buf.as_ptr());
    }

    fd
}

unsafe fn run_test(
    cgroup_fd: ::std::os::raw::c_int,
    test: *mut sock_create_test,
) -> ::std::os::raw::c_int {
    let mut err: ::std::os::raw::c_int;
    let mut optval: ::std::os::raw::c_int = 0;
    let mut ret: ::std::os::raw::c_int = -1;
    let mut optlen: socklen_t = ::std::mem::size_of_val(&optval) as socklen_t;

    let prog_fd = load_prog(
        (*test).insns.as_ptr(),
        (*test).expected_attach_type,
    );
    if prog_fd < 0 {
        log_err(c_str!("Failed to load BPF program"));
        return -1;
    }

    err = bpf_prog_attach(prog_fd, cgroup_fd, (*test).attach_type, 0);
    if err < 0 {
        log_err(c_str!("Failed to attach BPF program"));
        close(prog_fd);
        return ret;
    }

    let sock_fd = socket((*test).domain, (*test).type_, (*test).protocol);
    if sock_fd < 0 {
        if (*test).error == sock_create_test_error::DENY_CREATE {
            ret = 0;
        } else {
            log_err(c_str!("Failed to create socket"));
        }

        bpf_prog_detach2(prog_fd, cgroup_fd, (*test).attach_type);
        close(prog_fd);
        return ret;
    }

    if (*test).optname != 0 {
        err = getsockopt(
            sock_fd,
            SOL_SOCKET,
            (*test).optname,
            &mut optval as *mut _ as *mut ::std::os::raw::c_void,
            &mut optlen,
        );
        if err != 0 {
            log_err(c_str!("Failed to call getsockopt"));
            close(sock_fd);
            bpf_prog_detach2(prog_fd, cgroup_fd, (*test).attach_type);
            close(prog_fd);
            return ret;
        }

        if optval != (*test).optval {
            errno = 0;
            log_err(c_str!("getsockopt returned unexpected optval"));
            close(sock_fd);
            bpf_prog_detach2(prog_fd, cgroup_fd, (*test).attach_type);
            close(prog_fd);
            return ret;
        }
    }

    ret = ((*test).error != sock_create_test_error::OK) as ::std::os::raw::c_int;

    close(sock_fd);
    bpf_prog_detach2(prog_fd, cgroup_fd, (*test).attach_type);
    close(prog_fd);
    ret
}

pub unsafe extern "C" fn test_sock_create() {
    let cgroup_fd = test__join_cgroup(c_str!("/sock_create"));
    if !ASSERT_GE(cgroup_fd, 0, c_str!("join_cgroup")) {
        return;
    }

    let mut i: ::std::os::raw::c_int = 0;
    while (i as usize) < tests.len() {
        if !test__start_subtest(tests[i as usize].descr) {
            i += 1;
            continue;
        }

        ASSERT_OK(
            run_test(cgroup_fd, &mut tests[i as usize]),
            tests[i as usize].descr,
        );
        i += 1;
    }

    close(cgroup_fd);
}
