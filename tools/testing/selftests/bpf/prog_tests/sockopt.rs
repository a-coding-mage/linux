// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/sockopt.c.
// C includes removed; the referenced Linux, libbpf, io_uring, libc, and
// selftest harness symbols are expected to be supplied by surrounding bindings.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type socklen_t = u32;

const PAGE_SIZE: usize = 4096;

static mut bpf_log_buf: [c_char; 4096] = [0; 4096];
static mut verbose: bool = false;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum sockopt_test_error {
    OK = 0,
    DENY_LOAD,
    DENY_ATTACH,
    EOPNOTSUPP_GETSOCKOPT,
    EPERM_GETSOCKOPT,
    EFAULT_GETSOCKOPT,
    EPERM_SETSOCKOPT,
    EFAULT_SETSOCKOPT,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sockopt_test {
    descr: *const c_char,
    insns: [bpf_insn; 64],
    prog_type: bpf_prog_type,
    attach_type: bpf_attach_type,
    expected_attach_type: bpf_attach_type,

    set_optname: c_int,
    set_level: c_int,
    set_optval: [c_char; 64],
    set_optlen: socklen_t,

    get_optname: c_int,
    get_level: c_int,
    get_optval: [c_char; 64],
    get_optlen: socklen_t,
    get_optlen_ret: socklen_t,

    error: sockopt_test_error,
    io_uring_support: bool,
}

impl sockopt_test {
    const fn zeroed() -> Self {
        Self {
            descr: ptr::null(),
            insns: [BPF_INSN_ZERO; 64],
            prog_type: 0,
            attach_type: 0,
            expected_attach_type: 0,
            set_optname: 0,
            set_level: 0,
            set_optval: [0; 64],
            set_optlen: 0,
            get_optname: 0,
            get_level: 0,
            get_optval: [0; 64],
            get_optlen: 0,
            get_optlen_ret: 0,
            error: sockopt_test_error::OK,
            io_uring_support: false,
        }
    }
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! cchars {
    () => {
        [0; 64]
    };
    ($($x:expr),+ $(,)?) => {{
        let mut a = [0 as c_char; 64];
        let mut i = 0usize;
        $(
            a[i] = ($x) as c_char;
            i += 1;
        )+
        a
    }};
}

macro_rules! insns {
    ($($x:expr),* $(,)?) => {{
        let mut a = [BPF_INSN_ZERO; 64];
        let mut i = 0usize;
        $(
            a[i] = $x;
            i += 1;
        )*
        a
    }};
}

macro_rules! sockopt_test {
    ($($field:ident : $value:expr),* $(,)?) => {{
        let mut t = sockopt_test::zeroed();
        $(t.$field = $value;)*
        t
    }};
}

// ==================== getsockopt ====================
static mut tests: [sockopt_test; 34] = [
    sockopt_test! {
        descr: cstr!("getsockopt: no expected_attach_type"),
        insns: insns![BPF_MOV64_IMM(BPF_REG_0, 1), BPF_EXIT_INSN()],
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: 0,
        error: sockopt_test_error::DENY_LOAD,
    },
    sockopt_test! {
        descr: cstr!("getsockopt: wrong expected_attach_type"),
        insns: insns![BPF_MOV64_IMM(BPF_REG_0, 1), BPF_EXIT_INSN()],
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        error: sockopt_test_error::DENY_ATTACH,
    },
    sockopt_test! {
        descr: cstr!("getsockopt: bypass bpf hook"),
        insns: insns![BPF_MOV64_IMM(BPF_REG_0, 1), BPF_EXIT_INSN()],
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        get_level: SOL_IP,
        set_level: SOL_IP,
        get_optname: IP_TOS,
        set_optname: IP_TOS,
        set_optval: cchars![1 << 3],
        set_optlen: 1,
        get_optval: cchars![1 << 3],
        get_optlen: 1,
    },
    sockopt_test! {
        descr: cstr!("getsockopt: return EPERM from bpf hook"),
        insns: insns![BPF_MOV64_IMM(BPF_REG_0, 0), BPF_EXIT_INSN()],
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        get_level: SOL_IP,
        get_optname: IP_TOS,
        get_optlen: 1,
        error: sockopt_test_error::EPERM_GETSOCKOPT,
    },
    sockopt_test! {
        descr: cstr!("getsockopt: no optval bounds check, deny loading"),
        insns: insns![
            BPF_LDX_MEM(BPF_DW, BPF_REG_6, BPF_REG_1, offset_of_bpf_sockopt_optval()),
            BPF_MOV64_IMM(BPF_REG_0, 0x80),
            BPF_STX_MEM(BPF_W, BPF_REG_6, BPF_REG_0, 0),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        error: sockopt_test_error::DENY_LOAD,
    },
    sockopt_test! {
        descr: cstr!("getsockopt: read ctx->level"),
        insns: insns![
            BPF_LDX_MEM(BPF_W, BPF_REG_6, BPF_REG_1, offset_of_bpf_sockopt_level()),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_6, 123, 4),
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_retval()),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_JMP_A(1),
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        get_level: 123,
        get_optlen: 1,
    },
    sockopt_test! {
        descr: cstr!("getsockopt: deny writing to ctx->level"),
        insns: insns![
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_level()),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        error: sockopt_test_error::DENY_LOAD,
    },
    sockopt_test! {
        descr: cstr!("getsockopt: read ctx->optname"),
        insns: insns![
            BPF_LDX_MEM(BPF_W, BPF_REG_6, BPF_REG_1, offset_of_bpf_sockopt_optname()),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_6, 123, 4),
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_retval()),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_JMP_A(1),
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        get_optname: 123,
        get_optlen: 1,
    },
    sockopt_test! {
        descr: cstr!("getsockopt: read ctx->retval"),
        insns: insns![
            BPF_LDX_MEM(BPF_W, BPF_REG_6, BPF_REG_1, offset_of_bpf_sockopt_retval()),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        get_level: SOL_IP,
        get_optname: IP_TOS,
        get_optlen: 1,
    },
    sockopt_test! {
        descr: cstr!("getsockopt: deny writing to ctx->optname"),
        insns: insns![
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_optname()),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        error: sockopt_test_error::DENY_LOAD,
    },
    sockopt_test! {
        descr: cstr!("getsockopt: read ctx->optlen"),
        insns: insns![
            BPF_LDX_MEM(BPF_W, BPF_REG_6, BPF_REG_1, offset_of_bpf_sockopt_optlen()),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_6, 64, 4),
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_retval()),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_JMP_A(1),
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        get_level: SOL_SOCKET,
        get_optlen: 64,
        io_uring_support: true,
    },
    sockopt_test! {
        descr: cstr!("getsockopt: deny bigger ctx->optlen"),
        insns: insns![
            BPF_MOV64_IMM(BPF_REG_0, 65),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_optlen()),
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_retval()),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        get_optlen: 64,
        error: sockopt_test_error::EFAULT_GETSOCKOPT,
        io_uring_support: true,
    },
    sockopt_test! {
        descr: cstr!("getsockopt: deny negative ctx->optlen in TCP_ZEROCOPY_RECEIVE"),
        insns: insns![
            BPF_MOV64_IMM(BPF_REG_0, -1),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_optlen()),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        get_level: IPPROTO_TCP,
        get_optname: TCP_ZEROCOPY_RECEIVE,
        get_optlen: size_of::<tcp_zerocopy_receive>() as socklen_t,
        error: sockopt_test_error::EFAULT_GETSOCKOPT,
    },
    sockopt_test! {
        descr: cstr!("getsockopt: ignore >PAGE_SIZE optlen"),
        insns: sockopt_rw_first_byte_insns!(0xFF, true),
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        get_level: 1234,
        get_optname: 5678,
        get_optval: cchars![],
        get_optlen: (PAGE_SIZE + 1) as socklen_t,
        error: sockopt_test_error::EOPNOTSUPP_GETSOCKOPT,
        io_uring_support: true,
    },
    sockopt_test! {
        descr: cstr!("getsockopt: support smaller ctx->optlen"),
        insns: insns![
            BPF_MOV64_IMM(BPF_REG_0, 32),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_optlen()),
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_retval()),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        get_level: SOL_SOCKET,
        get_optlen: 64,
        get_optlen_ret: 32,
        io_uring_support: true,
    },
    sockopt_test! {
        descr: cstr!("getsockopt: deny writing to ctx->optval"),
        insns: insns![
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_STX_MEM(BPF_DW, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_optval()),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        error: sockopt_test_error::DENY_LOAD,
    },
    sockopt_test! {
        descr: cstr!("getsockopt: deny writing to ctx->optval_end"),
        insns: insns![
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_STX_MEM(BPF_DW, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_optval_end()),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        error: sockopt_test_error::DENY_LOAD,
    },
    sockopt_test! {
        descr: cstr!("getsockopt: rewrite value"),
        insns: sockopt_rw_first_byte_insns!(0xF0, true),
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        get_level: SOL_IP,
        get_optname: IP_TOS,
        get_optval: cchars![0xF0],
        get_optlen: 1,
    },

    // ==================== setsockopt ====================
    sockopt_test! {
        descr: cstr!("setsockopt: no expected_attach_type"),
        insns: insns![BPF_MOV64_IMM(BPF_REG_0, 1), BPF_EXIT_INSN()],
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: 0,
        error: sockopt_test_error::DENY_LOAD,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: wrong expected_attach_type"),
        insns: insns![BPF_MOV64_IMM(BPF_REG_0, 1), BPF_EXIT_INSN()],
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        error: sockopt_test_error::DENY_ATTACH,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: bypass bpf hook"),
        insns: insns![BPF_MOV64_IMM(BPF_REG_0, 1), BPF_EXIT_INSN()],
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        get_level: SOL_IP,
        set_level: SOL_IP,
        get_optname: IP_TOS,
        set_optname: IP_TOS,
        set_optval: cchars![1 << 3],
        set_optlen: 1,
        get_optval: cchars![1 << 3],
        get_optlen: 1,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: return EPERM from bpf hook"),
        insns: insns![BPF_MOV64_IMM(BPF_REG_0, 0), BPF_EXIT_INSN()],
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        set_level: SOL_IP,
        set_optname: IP_TOS,
        set_optlen: 1,
        error: sockopt_test_error::EPERM_SETSOCKOPT,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: no optval bounds check, deny loading"),
        insns: insns![
            BPF_LDX_MEM(BPF_DW, BPF_REG_6, BPF_REG_1, offset_of_bpf_sockopt_optval()),
            BPF_LDX_MEM(BPF_W, BPF_REG_0, BPF_REG_6, 0),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        error: sockopt_test_error::DENY_LOAD,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: read ctx->level"),
        insns: sockopt_setsockopt_read_field_insns!(offset_of_bpf_sockopt_level(), 123),
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        set_level: 123,
        set_optlen: 1,
        io_uring_support: true,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: allow changing ctx->level"),
        insns: insns![
            BPF_MOV64_IMM(BPF_REG_0, SOL_IP),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_level()),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        get_level: SOL_IP,
        set_level: 234,
        get_optname: IP_TOS,
        set_optname: IP_TOS,
        set_optval: cchars![1 << 3],
        set_optlen: 1,
        get_optval: cchars![1 << 3],
        get_optlen: 1,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: read ctx->optname"),
        insns: sockopt_setsockopt_read_field_insns!(offset_of_bpf_sockopt_optname(), 123),
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        set_optname: 123,
        set_optlen: 1,
        io_uring_support: true,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: allow changing ctx->optname"),
        insns: insns![
            BPF_MOV64_IMM(BPF_REG_0, IP_TOS),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_optname()),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        get_level: SOL_IP,
        set_level: SOL_IP,
        get_optname: IP_TOS,
        set_optname: 456,
        set_optval: cchars![1 << 3],
        set_optlen: 1,
        get_optval: cchars![1 << 3],
        get_optlen: 1,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: read ctx->optlen"),
        insns: sockopt_setsockopt_read_field_insns!(offset_of_bpf_sockopt_optlen(), 64),
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        set_optlen: 64,
        io_uring_support: true,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: ctx->optlen == -1 is ok"),
        insns: insns![
            BPF_MOV64_IMM(BPF_REG_0, -1),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_optlen()),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        set_optlen: 64,
        io_uring_support: true,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: deny ctx->optlen < 0 (except -1)"),
        insns: insns![
            BPF_MOV64_IMM(BPF_REG_0, -2),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_optlen()),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        set_optlen: 4,
        error: sockopt_test_error::EFAULT_SETSOCKOPT,
        io_uring_support: true,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: deny ctx->optlen > input optlen"),
        insns: insns![
            BPF_MOV64_IMM(BPF_REG_0, 65),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_optlen()),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        set_optlen: 64,
        error: sockopt_test_error::EFAULT_SETSOCKOPT,
        io_uring_support: true,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: ignore >PAGE_SIZE optlen"),
        insns: sockopt_rw_first_byte_insns!(0xF0, false),
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        set_level: SOL_IP,
        set_optname: IP_TOS,
        set_optval: cchars![],
        set_optlen: (PAGE_SIZE + 1) as socklen_t,
        get_level: SOL_IP,
        get_optname: IP_TOS,
        get_optval: cchars![],
        get_optlen: 4,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: allow changing ctx->optlen within bounds"),
        insns: insns![
            BPF_LDX_MEM(BPF_DW, BPF_REG_6, BPF_REG_1, offset_of_bpf_sockopt_optval()),
            BPF_MOV64_REG(BPF_REG_2, BPF_REG_6),
            BPF_ALU64_IMM(BPF_ADD, BPF_REG_6, 1),
            BPF_LDX_MEM(BPF_DW, BPF_REG_7, BPF_REG_1, offset_of_bpf_sockopt_optval_end()),
            BPF_JMP_REG(BPF_JGT, BPF_REG_6, BPF_REG_7, 1),
            BPF_ST_MEM(BPF_B, BPF_REG_2, 0, 1 << 3),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_optlen()),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        get_level: SOL_IP,
        set_level: SOL_IP,
        get_optname: IP_TOS,
        set_optname: IP_TOS,
        set_optval: cchars![1, 1, 1, 1],
        set_optlen: 4,
        get_optval: cchars![1 << 3],
        get_optlen: 1,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: deny write ctx->retval"),
        insns: insns![
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_retval()),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        error: sockopt_test_error::DENY_LOAD,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: deny read ctx->retval"),
        insns: insns![
            BPF_LDX_MEM(BPF_W, BPF_REG_6, BPF_REG_1, offset_of_bpf_sockopt_retval()),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        error: sockopt_test_error::DENY_LOAD,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: deny writing to ctx->optval"),
        insns: insns![
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_STX_MEM(BPF_DW, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_optval()),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        error: sockopt_test_error::DENY_LOAD,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: deny writing to ctx->optval_end"),
        insns: insns![
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_STX_MEM(BPF_DW, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_optval_end()),
            BPF_EXIT_INSN(),
        ],
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        error: sockopt_test_error::DENY_LOAD,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: allow IP_TOS <= 128"),
        insns: sockopt_ip_tos_128_insns!(),
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        get_level: SOL_IP,
        set_level: SOL_IP,
        get_optname: IP_TOS,
        set_optname: IP_TOS,
        set_optval: cchars![0x80],
        set_optlen: 1,
        get_optval: cchars![0x80],
        get_optlen: 1,
    },
    sockopt_test! {
        descr: cstr!("setsockopt: deny IP_TOS > 128"),
        insns: sockopt_ip_tos_128_insns!(),
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        get_level: SOL_IP,
        set_level: SOL_IP,
        get_optname: IP_TOS,
        set_optname: IP_TOS,
        set_optval: cchars![0x81],
        set_optlen: 1,
        get_optval: cchars![0x00],
        get_optlen: 1,
        error: sockopt_test_error::EPERM_SETSOCKOPT,
    },

    // ==================== prog_type ====================
    sockopt_test! {
        descr: cstr!("can attach only BPF_CGROUP_SETSOCKOP"),
        insns: insns![BPF_MOV64_IMM(BPF_REG_0, 1), BPF_EXIT_INSN()],
        prog_type: BPF_PROG_TYPE_CGROUP_SKB,
        attach_type: BPF_CGROUP_SETSOCKOPT,
        expected_attach_type: 0,
        error: sockopt_test_error::DENY_ATTACH,
    },
    sockopt_test! {
        descr: cstr!("can attach only BPF_CGROUP_GETSOCKOP"),
        insns: insns![BPF_MOV64_IMM(BPF_REG_0, 1), BPF_EXIT_INSN()],
        prog_type: BPF_PROG_TYPE_CGROUP_SKB,
        attach_type: BPF_CGROUP_GETSOCKOPT,
        expected_attach_type: 0,
        error: sockopt_test_error::DENY_ATTACH,
    },
];

macro_rules! sockopt_rw_first_byte_insns {
    ($byte:expr, $with_retval:expr) => {
        insns![
            BPF_LDX_MEM(BPF_DW, BPF_REG_6, BPF_REG_1, offset_of_bpf_sockopt_optval()),
            BPF_MOV64_REG(BPF_REG_2, BPF_REG_6),
            BPF_ALU64_IMM(BPF_ADD, BPF_REG_6, 1),
            BPF_LDX_MEM(BPF_DW, BPF_REG_7, BPF_REG_1, offset_of_bpf_sockopt_optval_end()),
            BPF_JMP_REG(BPF_JGT, BPF_REG_6, BPF_REG_7, 1),
            BPF_ST_MEM(BPF_B, BPF_REG_2, 0, $byte),
            BPF_MOV64_IMM(BPF_REG_0, 5),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_retval()),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ]
    };
}

macro_rules! sockopt_setsockopt_read_field_insns {
    ($off:expr, $val:expr) => {
        insns![
            BPF_LDX_MEM(BPF_W, BPF_REG_6, BPF_REG_1, $off),
            BPF_JMP_IMM(BPF_JNE, BPF_REG_6, $val, 4),
            BPF_MOV64_IMM(BPF_REG_0, -1),
            BPF_STX_MEM(BPF_W, BPF_REG_1, BPF_REG_0, offset_of_bpf_sockopt_optlen()),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_JMP_A(1),
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_EXIT_INSN(),
        ]
    };
}

macro_rules! sockopt_ip_tos_128_insns {
    () => {
        insns![
            BPF_LDX_MEM(BPF_DW, BPF_REG_6, BPF_REG_1, offset_of_bpf_sockopt_optval()),
            BPF_MOV64_REG(BPF_REG_7, BPF_REG_6),
            BPF_ALU64_IMM(BPF_ADD, BPF_REG_7, 1),
            BPF_LDX_MEM(BPF_DW, BPF_REG_8, BPF_REG_1, offset_of_bpf_sockopt_optval_end()),
            BPF_JMP_REG(BPF_JGT, BPF_REG_7, BPF_REG_8, 4),
            BPF_LDX_MEM(BPF_B, BPF_REG_9, BPF_REG_6, 0),
            BPF_JMP_IMM(BPF_JGT, BPF_REG_9, 128, 2),
            BPF_MOV64_IMM(BPF_REG_0, 1),
            BPF_JMP_A(1),
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_EXIT_INSN(),
        ]
    };
}

unsafe fn load_prog(
    insns: *const bpf_insn,
    prog_type: bpf_prog_type,
    expected_attach_type: bpf_attach_type,
) -> c_int {
    let mut opts: bpf_prog_load_opts = core::mem::zeroed();
    opts.expected_attach_type = expected_attach_type;
    opts.log_level = 2;
    opts.log_buf = bpf_log_buf.as_mut_ptr();
    opts.log_size = size_of_val(&bpf_log_buf) as u32;

    let mut insns_cnt: c_int = 0;
    while (*insns.add(insns_cnt as usize)).code != (BPF_JMP | BPF_EXIT) {
        insns_cnt += 1;
    }
    insns_cnt += 1;

    let fd = bpf_prog_load(
        prog_type,
        ptr::null(),
        cstr!("GPL"),
        insns,
        insns_cnt,
        &mut opts,
    );
    if verbose && fd < 0 {
        fprintf(stderr, cstr!("%s\n"), bpf_log_buf.as_ptr());
    }

    fd
}

/* Core function that handles io_uring ring initialization,
 * sending SQE with sockopt command and waiting for the CQE.
 */
unsafe fn uring_sockopt(
    op: c_int,
    fd: c_int,
    level: c_int,
    optname: c_int,
    optval: *const c_void,
    optlen: socklen_t,
) -> c_int {
    let mut cqe: *mut io_uring_cqe = ptr::null_mut();
    let mut ring: io_uring = core::mem::zeroed();
    let mut err: c_int;

    err = io_uring_queue_init(1, &mut ring, 0);
    if !ASSERT_OK(err, cstr!("io_uring initialization")) {
        return err;
    }

    let sqe = io_uring_get_sqe(&mut ring);
    if !ASSERT_NEQ(sqe as *const c_void, ptr::null(), cstr!("Get an SQE")) {
        err = -1;
        goto_fail_uring_sockopt(&mut ring, err)
    } else {
        io_uring_prep_cmd(sqe, op, fd, level, optname, optval, optlen);

        err = io_uring_submit(&mut ring);
        if !ASSERT_EQ(err, 1, cstr!("Submit SQE")) {
            goto_fail_uring_sockopt(&mut ring, err)
        } else {
            err = io_uring_wait_cqe(&mut ring, &mut cqe);
            if !ASSERT_OK(err, cstr!("Wait for CQE")) {
                goto_fail_uring_sockopt(&mut ring, err)
            } else {
                err = (*cqe).res;
                goto_fail_uring_sockopt(&mut ring, err)
            }
        }
    }
}

unsafe fn goto_fail_uring_sockopt(ring: *mut io_uring, err: c_int) -> c_int {
    io_uring_queue_exit(ring);
    err
}

unsafe fn uring_setsockopt(
    fd: c_int,
    level: c_int,
    optname: c_int,
    optval: *const c_void,
    optlen: socklen_t,
) -> c_int {
    uring_sockopt(SOCKET_URING_OP_SETSOCKOPT, fd, level, optname, optval, optlen)
}

unsafe fn uring_getsockopt(
    fd: c_int,
    level: c_int,
    optname: c_int,
    optval: *mut c_void,
    optlen: *mut socklen_t,
) -> c_int {
    let ret = uring_sockopt(
        SOCKET_URING_OP_GETSOCKOPT,
        fd,
        level,
        optname,
        optval as *const c_void,
        *optlen,
    );
    if ret < 0 {
        return ret;
    }

    /* Populate optlen back to be compatible with systemcall interface,
     * and simplify the test.
     */
    *optlen = ret as socklen_t;

    0
}

/* Execute the setsocktopt operation */
unsafe fn call_setsockopt(
    use_io_uring: bool,
    fd: c_int,
    level: c_int,
    optname: c_int,
    optval: *const c_void,
    optlen: socklen_t,
) -> c_int {
    if use_io_uring {
        return uring_setsockopt(fd, level, optname, optval, optlen);
    }

    setsockopt(fd, level, optname, optval, optlen)
}

/* Execute the getsocktopt operation */
unsafe fn call_getsockopt(
    use_io_uring: bool,
    fd: c_int,
    level: c_int,
    optname: c_int,
    optval: *mut c_void,
    optlen: *mut socklen_t,
) -> c_int {
    if use_io_uring {
        return uring_getsockopt(fd, level, optname, optval, optlen);
    }

    getsockopt(fd, level, optname, optval, optlen)
}

unsafe fn run_test(
    cgroup_fd: c_int,
    test: *mut sockopt_test,
    use_io_uring: bool,
    use_link: bool,
) -> c_int {
    let mut prog_type = BPF_PROG_TYPE_CGROUP_SOCKOPT;
    let mut link_fd: c_int = -1;
    let mut optval: *mut c_void = ptr::null_mut();
    let mut ret: c_int = 0;

    if (*test).prog_type != 0 {
        prog_type = (*test).prog_type;
    }

    let prog_fd = load_prog((*test).insns.as_ptr(), prog_type, (*test).expected_attach_type);
    if prog_fd < 0 {
        if (*test).error == sockopt_test_error::DENY_LOAD {
            return 0;
        }

        log_err(cstr!("Failed to load BPF program"));
        return -1;
    }

    let mut err: c_int;
    if use_link {
        err = bpf_link_create(prog_fd, cgroup_fd, (*test).attach_type, ptr::null());
        link_fd = err;
    } else {
        err = bpf_prog_attach(prog_fd, cgroup_fd, (*test).attach_type, 0);
    }
    if err < 0 {
        if (*test).error == sockopt_test_error::DENY_ATTACH {
            close(prog_fd);
            return ret;
        }

        log_err(cstr!("Failed to attach BPF program"));
        ret = -1;
        close(prog_fd);
        return ret;
    }

    let sock_fd = socket(AF_INET, SOCK_STREAM, 0);
    if sock_fd < 0 {
        log_err(cstr!("Failed to create AF_INET socket"));
        ret = -1;
        detach_and_close(use_link, link_fd, prog_fd, cgroup_fd, (*test).attach_type);
        return ret;
    }

    if (*test).set_optlen != 0 {
        if (*test).set_optlen as usize >= PAGE_SIZE {
            let num_pages = (*test).set_optlen as usize / PAGE_SIZE;
            let remainder = (*test).set_optlen as usize % PAGE_SIZE;
            (*test).set_optlen = (num_pages as i64 * sysconf(_SC_PAGESIZE) + remainder as i64) as socklen_t;
        }

        err = call_setsockopt(
            use_io_uring,
            sock_fd,
            (*test).set_level,
            (*test).set_optname,
            (*test).set_optval.as_ptr() as *const c_void,
            (*test).set_optlen,
        );
        if err != 0 {
            if errno() == EPERM && (*test).error == sockopt_test_error::EPERM_SETSOCKOPT {
                close(sock_fd);
                detach_and_close(use_link, link_fd, prog_fd, cgroup_fd, (*test).attach_type);
                return ret;
            }
            if errno() == EFAULT && (*test).error == sockopt_test_error::EFAULT_SETSOCKOPT {
                free(optval);
                close(sock_fd);
                detach_and_close(use_link, link_fd, prog_fd, cgroup_fd, (*test).attach_type);
                return ret;
            }

            log_err(cstr!("Failed to call setsockopt"));
            ret = -1;
            close(sock_fd);
            detach_and_close(use_link, link_fd, prog_fd, cgroup_fd, (*test).attach_type);
            return ret;
        }
    }

    if (*test).get_optlen != 0 {
        if (*test).get_optlen as usize >= PAGE_SIZE {
            let num_pages = (*test).get_optlen as usize / PAGE_SIZE;
            let remainder = (*test).get_optlen as usize % PAGE_SIZE;
            (*test).get_optlen = (num_pages as i64 * sysconf(_SC_PAGESIZE) + remainder as i64) as socklen_t;
        }

        optval = malloc((*test).get_optlen as usize);
        memset(optval, 0, (*test).get_optlen as usize);
        let mut optlen: socklen_t = (*test).get_optlen;
        let expected_get_optlen: socklen_t = if (*test).get_optlen_ret != 0 {
            (*test).get_optlen_ret
        } else {
            (*test).get_optlen
        };

        err = call_getsockopt(
            use_io_uring,
            sock_fd,
            (*test).get_level,
            (*test).get_optname,
            optval,
            &mut optlen,
        );
        if err != 0 {
            if errno() == EOPNOTSUPP && (*test).error == sockopt_test_error::EOPNOTSUPP_GETSOCKOPT {
                free(optval);
                close(sock_fd);
                detach_and_close(use_link, link_fd, prog_fd, cgroup_fd, (*test).attach_type);
                return ret;
            }
            if errno() == EPERM && (*test).error == sockopt_test_error::EPERM_GETSOCKOPT {
                free(optval);
                close(sock_fd);
                detach_and_close(use_link, link_fd, prog_fd, cgroup_fd, (*test).attach_type);
                return ret;
            }
            if errno() == EFAULT && (*test).error == sockopt_test_error::EFAULT_GETSOCKOPT {
                free(optval);
                close(sock_fd);
                detach_and_close(use_link, link_fd, prog_fd, cgroup_fd, (*test).attach_type);
                return ret;
            }

            log_err(cstr!("Failed to call getsockopt"));
            ret = -1;
            free(optval);
            close(sock_fd);
            detach_and_close(use_link, link_fd, prog_fd, cgroup_fd, (*test).attach_type);
            return ret;
        }

        if optlen != expected_get_optlen {
            set_errno(0);
            log_err(cstr!("getsockopt returned unexpected optlen"));
            ret = -1;
            free(optval);
            close(sock_fd);
            detach_and_close(use_link, link_fd, prog_fd, cgroup_fd, (*test).attach_type);
            return ret;
        }

        if memcmp(optval, (*test).get_optval.as_ptr() as *const c_void, optlen as usize) != 0 {
            set_errno(0);
            log_err(cstr!("getsockopt returned unexpected optval"));
            ret = -1;
            free(optval);
            close(sock_fd);
            detach_and_close(use_link, link_fd, prog_fd, cgroup_fd, (*test).attach_type);
            return ret;
        }
    }

    ret = ((*test).error != sockopt_test_error::OK) as c_int;

    free(optval);
    close(sock_fd);
    detach_and_close(use_link, link_fd, prog_fd, cgroup_fd, (*test).attach_type);
    ret
}

unsafe fn detach_and_close(
    use_link: bool,
    link_fd: c_int,
    prog_fd: c_int,
    cgroup_fd: c_int,
    attach_type: bpf_attach_type,
) {
    if use_link {
        if link_fd >= 0 {
            close(link_fd);
        }
    } else {
        bpf_prog_detach2(prog_fd, cgroup_fd, attach_type);
    }
    close(prog_fd);
}

#[no_mangle]
pub unsafe extern "C" fn test_sockopt() {
    let cgroup_fd = test__join_cgroup(cstr!("/sockopt"));
    if !ASSERT_GE(cgroup_fd, 0, cstr!("join_cgroup")) {
        return;
    }

    let mut i = 0usize;
    while i < tests.len() {
        if !test__start_subtest(tests[i].descr) {
            i += 1;
            continue;
        }

        ASSERT_OK(run_test(cgroup_fd, &mut tests[i], false, false), tests[i].descr);
        ASSERT_OK(run_test(cgroup_fd, &mut tests[i], false, true), tests[i].descr);
        if tests[i].io_uring_support {
            ASSERT_OK(run_test(cgroup_fd, &mut tests[i], true, false), tests[i].descr);
        }
        i += 1;
    }

    close(cgroup_fd);
}

// External declarations and constants supplied by translated headers/bindings.
type bpf_prog_type = c_int;
type bpf_attach_type = c_int;

#[repr(C)]
#[derive(Clone, Copy)]
struct bpf_insn {
    code: u8,
    dst_src: u8,
    off: i16,
    imm: i32,
}

const BPF_INSN_ZERO: bpf_insn = bpf_insn { code: 0, dst_src: 0, off: 0, imm: 0 };

#[repr(C)]
struct bpf_prog_load_opts {
    expected_attach_type: bpf_attach_type,
    log_level: u32,
    log_buf: *mut c_char,
    log_size: u32,
}

#[repr(C)]
struct io_uring_cqe {
    res: c_int,
}
#[repr(C)]
struct io_uring_sqe {
    _private: [u8; 0],
}
#[repr(C)]
struct io_uring {
    _private: [u8; 0],
}
#[repr(C)]
struct tcp_zerocopy_receive {
    _private: [u8; 0],
}

extern "C" {
    static mut stderr: *mut c_void;

    fn bpf_prog_load(
        prog_type: bpf_prog_type,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: c_int,
        opts: *mut bpf_prog_load_opts,
    ) -> c_int;
    fn bpf_link_create(
        prog_fd: c_int,
        target_fd: c_int,
        attach_type: bpf_attach_type,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_prog_attach(
        prog_fd: c_int,
        target_fd: c_int,
        attach_type: bpf_attach_type,
        flags: u32,
    ) -> c_int;
    fn bpf_prog_detach2(prog_fd: c_int, target_fd: c_int, attach_type: bpf_attach_type) -> c_int;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn setsockopt(fd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
    fn getsockopt(fd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(ptr: *mut c_void, value: c_int, n: usize) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn sysconf(name: c_int) -> i64;
    fn io_uring_queue_init(entries: u32, ring: *mut io_uring, flags: u32) -> c_int;
    fn io_uring_get_sqe(ring: *mut io_uring) -> *mut io_uring_sqe;
    fn io_uring_prep_cmd(
        sqe: *mut io_uring_sqe,
        op: c_int,
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: socklen_t,
    );
    fn io_uring_submit(ring: *mut io_uring) -> c_int;
    fn io_uring_wait_cqe(ring: *mut io_uring, cqe_ptr: *mut *mut io_uring_cqe) -> c_int;
    fn io_uring_queue_exit(ring: *mut io_uring);
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn log_err(msg: *const c_char);
    fn errno() -> c_int;
    fn set_errno(value: c_int);
}

extern "Rust" {
    fn ASSERT_OK(value: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(a: *const c_void, b: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(a: c_int, b: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(a: c_int, b: c_int, name: *const c_char) -> bool;
    fn BPF_MOV64_IMM(dst: c_int, imm: c_int) -> bpf_insn;
    fn BPF_MOV64_REG(dst: c_int, src: c_int) -> bpf_insn;
    fn BPF_EXIT_INSN() -> bpf_insn;
    fn BPF_LDX_MEM(size: c_int, dst: c_int, src: c_int, off: usize) -> bpf_insn;
    fn BPF_STX_MEM(size: c_int, dst: c_int, src: c_int, off: usize) -> bpf_insn;
    fn BPF_ST_MEM(size: c_int, dst: c_int, off: c_int, imm: c_int) -> bpf_insn;
    fn BPF_JMP_IMM(op: c_int, dst: c_int, imm: c_int, off: c_int) -> bpf_insn;
    fn BPF_JMP_REG(op: c_int, dst: c_int, src: c_int, off: c_int) -> bpf_insn;
    fn BPF_ALU64_IMM(op: c_int, dst: c_int, imm: c_int) -> bpf_insn;
    fn BPF_JMP_A(off: c_int) -> bpf_insn;
    fn offset_of_bpf_sockopt_optval() -> usize;
    fn offset_of_bpf_sockopt_optval_end() -> usize;
    fn offset_of_bpf_sockopt_level() -> usize;
    fn offset_of_bpf_sockopt_optname() -> usize;
    fn offset_of_bpf_sockopt_optlen() -> usize;
    fn offset_of_bpf_sockopt_retval() -> usize;
    fn size_of_val<T>(value: &T) -> usize;
}

extern "Rust" {
    static BPF_REG_0: c_int;
    static BPF_REG_1: c_int;
    static BPF_REG_2: c_int;
    static BPF_REG_6: c_int;
    static BPF_REG_7: c_int;
    static BPF_REG_8: c_int;
    static BPF_REG_9: c_int;
    static BPF_DW: c_int;
    static BPF_W: c_int;
    static BPF_B: c_int;
    static BPF_JMP: u8;
    static BPF_EXIT: u8;
    static BPF_JNE: c_int;
    static BPF_JGT: c_int;
    static BPF_ADD: c_int;
    static BPF_CGROUP_GETSOCKOPT: bpf_attach_type;
    static BPF_CGROUP_SETSOCKOPT: bpf_attach_type;
    static BPF_PROG_TYPE_CGROUP_SKB: bpf_prog_type;
    static BPF_PROG_TYPE_CGROUP_SOCKOPT: bpf_prog_type;
    static SOL_IP: c_int;
    static SOL_SOCKET: c_int;
    static IP_TOS: c_int;
    static IPPROTO_TCP: c_int;
    static TCP_ZEROCOPY_RECEIVE: c_int;
    static SOCKET_URING_OP_SETSOCKOPT: c_int;
    static SOCKET_URING_OP_GETSOCKOPT: c_int;
    static AF_INET: c_int;
    static SOCK_STREAM: c_int;
    static _SC_PAGESIZE: c_int;
    static EPERM: c_int;
    static EFAULT: c_int;
    static EOPNOTSUPP: c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
