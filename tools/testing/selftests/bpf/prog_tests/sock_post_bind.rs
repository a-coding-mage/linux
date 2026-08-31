// SPDX-License-Identifier: GPL-2.0
// C dependencies translated from:
// <linux/bpf.h>, <test_progs.h>, "cgroup_helpers.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

const TEST_NS: *const c_char = c"sock_post_bind".as_ptr();

static mut bpf_log_buf: [c_char; 4096] = [0; 4096];

#[repr(C)]
struct sock_post_bind_test {
    descr: *const c_char,
    /* BPF prog properties */
    insns: [bpf_insn; 64],
    attach_type: bpf_attach_type,
    expected_attach_type: bpf_attach_type,
    /* Socket properties */
    domain: c_int,
    type_: c_int,
    /* Endpoint to bind() to */
    ip: *const c_char,
    port: u16,
    port_retry: u16,

    /* Expected test result */
    result: sock_post_bind_result,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum sock_post_bind_result {
    ATTACH_REJECT,
    BIND_REJECT,
    SUCCESS,
    RETRY_SUCCESS,
    RETRY_REJECT,
}

macro_rules! insns64 {
    ($($insn:expr),* $(,)?) => {{
        let mut insns: [bpf_insn; 64] = unsafe { zeroed() };
        let init = [$($insn),*];
        let mut i = 0usize;
        while i < init.len() {
            insns[i] = init[i];
            i += 1;
        }
        insns
    }};
}

static mut tests: [sock_post_bind_test; 13] = [
    sock_post_bind_test {
        descr: c"attach type mismatch bind4 vs bind6".as_ptr(),
        insns: insns64![
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET4_POST_BIND,
        attach_type: BPF_CGROUP_INET6_POST_BIND,
        domain: 0,
        type_: 0,
        ip: null(),
        port: 0,
        port_retry: 0,
        result: sock_post_bind_result::ATTACH_REJECT,
    },
    sock_post_bind_test {
        descr: c"attach type mismatch bind6 vs bind4".as_ptr(),
        insns: insns64![
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET6_POST_BIND,
        attach_type: BPF_CGROUP_INET4_POST_BIND,
        domain: 0,
        type_: 0,
        ip: null(),
        port: 0,
        port_retry: 0,
        result: sock_post_bind_result::ATTACH_REJECT,
    },
    sock_post_bind_test {
        descr: c"attach type mismatch default vs bind4".as_ptr(),
        insns: insns64![
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: 0,
        attach_type: BPF_CGROUP_INET4_POST_BIND,
        domain: 0,
        type_: 0,
        ip: null(),
        port: 0,
        port_retry: 0,
        result: sock_post_bind_result::ATTACH_REJECT,
    },
    sock_post_bind_test {
        descr: c"attach type mismatch bind6 vs sock_create".as_ptr(),
        insns: insns64![
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET6_POST_BIND,
        attach_type: BPF_CGROUP_INET_SOCK_CREATE,
        domain: 0,
        type_: 0,
        ip: null(),
        port: 0,
        port_retry: 0,
        result: sock_post_bind_result::ATTACH_REJECT,
    },
    sock_post_bind_test {
        descr: c"bind4 reject all".as_ptr(),
        insns: insns64![
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET4_POST_BIND,
        attach_type: BPF_CGROUP_INET4_POST_BIND,
        domain: AF_INET,
        type_: SOCK_STREAM,
        ip: c"0.0.0.0".as_ptr(),
        port: 0,
        port_retry: 0,
        result: sock_post_bind_result::BIND_REJECT,
    },
    sock_post_bind_test {
        descr: c"bind6 reject all".as_ptr(),
        insns: insns64![
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET6_POST_BIND,
        attach_type: BPF_CGROUP_INET6_POST_BIND,
        domain: AF_INET6,
        type_: SOCK_STREAM,
        ip: c"::".as_ptr(),
        port: 0,
        port_retry: 0,
        result: sock_post_bind_result::BIND_REJECT,
    },
    sock_post_bind_test {
        descr: c"bind6 deny specific IP & port".as_ptr(),
        insns: insns64![
            BPF_MOV64_REG(BPF_REG_6, BPF_REG_1),

            /* if (ip == expected && port == expected) */
            BPF_LDX_MEM(BPF_W, BPF_REG_7, BPF_REG_6,
                        offset_of!(bpf_sock, src_ip6[3]) as i16),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_7,
                        __bpf_constant_ntohl(0x00000001), 4),
            BPF_LDX_MEM(BPF_W, BPF_REG_7, BPF_REG_6,
                        offset_of!(bpf_sock, src_port) as i16),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_7, 0x2001, 2),

            /* return DENY; */
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_JMP_A(1),

            /* else return ALLOW; */
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET6_POST_BIND,
        attach_type: BPF_CGROUP_INET6_POST_BIND,
        domain: AF_INET6,
        type_: SOCK_STREAM,
        ip: c"::1".as_ptr(),
        port: 8193,
        port_retry: 0,
        result: sock_post_bind_result::BIND_REJECT,
    },
    sock_post_bind_test {
        descr: c"bind4 allow specific IP & port".as_ptr(),
        insns: insns64![
            BPF_MOV64_REG(BPF_REG_6, BPF_REG_1),

            /* if (ip == expected && port == expected) */
            BPF_LDX_MEM(BPF_W, BPF_REG_7, BPF_REG_6,
                        offset_of!(bpf_sock, src_ip4) as i16),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_7,
                        __bpf_constant_ntohl(0x7F000001), 4),
            BPF_LDX_MEM(BPF_W, BPF_REG_7, BPF_REG_6,
                        offset_of!(bpf_sock, src_port) as i16),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_7, 0x1002, 2),

            /* return ALLOW; */
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_JMP_A(1),

            /* else return DENY; */
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET4_POST_BIND,
        attach_type: BPF_CGROUP_INET4_POST_BIND,
        domain: AF_INET,
        type_: SOCK_STREAM,
        ip: c"127.0.0.1".as_ptr(),
        port: 4098,
        port_retry: 0,
        result: sock_post_bind_result::SUCCESS,
    },
    sock_post_bind_test {
        descr: c"bind4 deny specific IP & port of TCP, and retry".as_ptr(),
        insns: insns64![
            BPF_MOV64_REG(BPF_REG_6, BPF_REG_1),

            /* if (ip == expected && port == expected) */
            BPF_LDX_MEM(BPF_W, BPF_REG_7, BPF_REG_6,
                        offset_of!(bpf_sock, src_ip4) as i16),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_7,
                        __bpf_constant_ntohl(0x7F000001), 4),
            BPF_LDX_MEM(BPF_W, BPF_REG_7, BPF_REG_6,
                        offset_of!(bpf_sock, src_port) as i16),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_7, 0x1002, 2),

            /* return DENY; */
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_JMP_A(1),

            /* else return ALLOW; */
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET4_POST_BIND,
        attach_type: BPF_CGROUP_INET4_POST_BIND,
        domain: AF_INET,
        type_: SOCK_STREAM,
        ip: c"127.0.0.1".as_ptr(),
        port: 4098,
        port_retry: 5000,
        result: sock_post_bind_result::RETRY_SUCCESS,
    },
    sock_post_bind_test {
        descr: c"bind4 deny specific IP & port of UDP, and retry".as_ptr(),
        insns: insns64![
            BPF_MOV64_REG(BPF_REG_6, BPF_REG_1),

            /* if (ip == expected && port == expected) */
            BPF_LDX_MEM(BPF_W, BPF_REG_7, BPF_REG_6,
                        offset_of!(bpf_sock, src_ip4) as i16),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_7,
                        __bpf_constant_ntohl(0x7F000001), 4),
            BPF_LDX_MEM(BPF_W, BPF_REG_7, BPF_REG_6,
                        offset_of!(bpf_sock, src_port) as i16),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_7, 0x1002, 2),

            /* return DENY; */
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_JMP_A(1),

            /* else return ALLOW; */
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET4_POST_BIND,
        attach_type: BPF_CGROUP_INET4_POST_BIND,
        domain: AF_INET,
        type_: SOCK_DGRAM,
        ip: c"127.0.0.1".as_ptr(),
        port: 4098,
        port_retry: 5000,
        result: sock_post_bind_result::RETRY_SUCCESS,
    },
    sock_post_bind_test {
        descr: c"bind6 deny specific IP & port, and retry".as_ptr(),
        insns: insns64![
            BPF_MOV64_REG(BPF_REG_6, BPF_REG_1),

            /* if (ip == expected && port == expected) */
            BPF_LDX_MEM(BPF_W, BPF_REG_7, BPF_REG_6,
                        offset_of!(bpf_sock, src_ip6[3]) as i16),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_7,
                        __bpf_constant_ntohl(0x00000001), 4),
            BPF_LDX_MEM(BPF_W, BPF_REG_7, BPF_REG_6,
                        offset_of!(bpf_sock, src_port) as i16),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_7, 0x2001, 2),

            /* return DENY; */
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_JMP_A(1),

            /* else return ALLOW; */
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET6_POST_BIND,
        attach_type: BPF_CGROUP_INET6_POST_BIND,
        domain: AF_INET6,
        type_: SOCK_STREAM,
        ip: c"::1".as_ptr(),
        port: 8193,
        port_retry: 9000,
        result: sock_post_bind_result::RETRY_SUCCESS,
    },
    sock_post_bind_test {
        descr: c"bind4 allow all".as_ptr(),
        insns: insns64![
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET4_POST_BIND,
        attach_type: BPF_CGROUP_INET4_POST_BIND,
        domain: AF_INET,
        type_: SOCK_STREAM,
        ip: c"0.0.0.0".as_ptr(),
        port: 0,
        port_retry: 0,
        result: sock_post_bind_result::SUCCESS,
    },
    sock_post_bind_test {
        descr: c"bind6 allow all".as_ptr(),
        insns: insns64![
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        expected_attach_type: BPF_CGROUP_INET6_POST_BIND,
        attach_type: BPF_CGROUP_INET6_POST_BIND,
        domain: AF_INET6,
        type_: SOCK_STREAM,
        ip: c"::".as_ptr(),
        port: 0,
        port_retry: 0,
        result: sock_post_bind_result::SUCCESS,
    },
];

unsafe fn load_prog(insns: *const bpf_insn, expected_attach_type: bpf_attach_type) -> c_int {
    let mut opts: bpf_prog_load_opts = zeroed();
    opts.expected_attach_type = expected_attach_type;
    opts.log_level = 2;
    opts.log_buf = bpf_log_buf.as_mut_ptr();
    opts.log_size = size_of::<[c_char; 4096]>() as _;

    let mut insns_cnt: c_int = 0;

    while (*insns.add(insns_cnt as usize)).code != (BPF_JMP | BPF_EXIT) as _ {
        insns_cnt += 1;
    }
    insns_cnt += 1;

    let fd = bpf_prog_load(
        BPF_PROG_TYPE_CGROUP_SOCK,
        null(),
        c"GPL".as_ptr(),
        insns,
        insns_cnt,
        &mut opts,
    );
    if fd < 0 {
        fprintf(stderr, c"%s\n".as_ptr(), bpf_log_buf.as_ptr());
    }

    fd
}

unsafe fn bind_sock(
    domain: c_int,
    type_: c_int,
    ip: *const c_char,
    port: u16,
    port_retry: u16,
) -> c_int {
    let mut addr: sockaddr_storage = zeroed();
    let mut addr6: *mut sockaddr_in6 = null_mut();
    let mut addr4: *mut sockaddr_in = null_mut();
    let mut sockfd: c_int = -1;
    let len: socklen_t;
    let mut res: c_int = sock_post_bind_result::SUCCESS as c_int;

    sockfd = socket(domain, type_, 0);
    if sockfd < 0 {
        res = -1;
        close(sockfd);
        return res;
    }

    memset(
        &mut addr as *mut sockaddr_storage as *mut c_void,
        0,
        size_of::<sockaddr_storage>(),
    );

    if domain == AF_INET {
        len = size_of::<sockaddr_in>() as socklen_t;
        addr4 = &mut addr as *mut sockaddr_storage as *mut sockaddr_in;
        (*addr4).sin_family = domain as _;
        (*addr4).sin_port = htons(port);
        if inet_pton(domain, ip, &mut (*addr4).sin_addr as *mut _ as *mut c_void) != 1 {
            res = -1;
            close(sockfd);
            return res;
        }
    } else if domain == AF_INET6 {
        len = size_of::<sockaddr_in6>() as socklen_t;
        addr6 = &mut addr as *mut sockaddr_storage as *mut sockaddr_in6;
        (*addr6).sin6_family = domain as _;
        (*addr6).sin6_port = htons(port);
        if inet_pton(domain, ip, &mut (*addr6).sin6_addr as *mut _ as *mut c_void) != 1 {
            res = -1;
            close(sockfd);
            return res;
        }
    } else {
        res = -1;
        close(sockfd);
        return res;
    }

    if bind(sockfd, &addr as *const sockaddr_storage as *const sockaddr, len) == -1 {
        /* sys_bind() may fail for different reasons, errno has to be
         * checked to confirm that BPF program rejected it.
         */
        if *__errno_location() != EPERM {
            res = -1;
            close(sockfd);
            return res;
        }
        if port_retry != 0 {
            if domain == AF_INET {
                (*addr4).sin_port = htons(port_retry);
            } else {
                (*addr6).sin6_port = htons(port_retry);
            }
            if bind(sockfd, &addr as *const sockaddr_storage as *const sockaddr, len) == -1 {
                if *__errno_location() != EPERM {
                    res = -1;
                } else {
                    res = sock_post_bind_result::RETRY_REJECT as c_int;
                }
            } else {
                res = sock_post_bind_result::RETRY_SUCCESS as c_int;
            }
            close(sockfd);
            return res;
        }
        res = sock_post_bind_result::BIND_REJECT as c_int;
        close(sockfd);
        return res;
    }

    close(sockfd);
    res
}

unsafe fn run_test(cgroup_fd: c_int, test: *mut sock_post_bind_test) -> c_int {
    let mut ret: c_int = 0;

    let prog_fd = load_prog((*test).insns.as_ptr(), (*test).expected_attach_type);
    if prog_fd < 0 {
        ret = -1;
    } else {
        let err = bpf_prog_attach(prog_fd, cgroup_fd, (*test).attach_type, 0);
        if err < 0 {
            if (*test).result != sock_post_bind_result::ATTACH_REJECT {
                ret = -1;
            }
        } else {
            let res = bind_sock(
                (*test).domain,
                (*test).type_,
                (*test).ip,
                (*test).port,
                (*test).port_retry,
            );
            if !(res > 0 && (*test).result as c_int == res) {
                ret = -1;
            }
        }
    }

    /* Detaching w/o checking return code: best effort attempt. */
    if prog_fd != -1 {
        bpf_prog_detach(cgroup_fd, (*test).attach_type);
    }
    close(prog_fd);
    ret
}

pub unsafe extern "C" fn test_sock_post_bind() {
    let mut ns: *mut netns_obj;
    let cgroup_fd: c_int;

    cgroup_fd = test__join_cgroup(c"/post_bind".as_ptr());
    if !ASSERT_OK_FD(cgroup_fd, c"join_cgroup".as_ptr()) {
        return;
    }

    ns = netns_new(TEST_NS, true);
    if !ASSERT_OK_PTR(ns as *const c_void, c"netns_new".as_ptr()) {
        netns_free(ns);
        close(cgroup_fd);
        return;
    }

    let mut i = 0usize;
    while i < tests.len() {
        if !test__start_subtest(tests[i].descr) {
            i += 1;
            continue;
        }

        ASSERT_OK(run_test(cgroup_fd, &mut tests[i]), tests[i].descr);
        i += 1;
    }

    netns_free(ns);
    close(cgroup_fd);
}

