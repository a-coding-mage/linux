macro_rules! BPF_SOCK_ADDR_STORE {
    ($field:ident, $off:expr, $res:expr, $err:expr, $flgs:expr) => {
        {
            test_val {
                descr: concat!(
                    "wide store to bpf_sock_addr.",
                    stringify!($field),
                    "[",
                    stringify!($off),
                    "]",
                ),
                insns: [
                    BPF_MOV64_IMM(BPF_REG_0, 1),
                    BPF_STX_MEM(
                        BPF_DW,
                        BPF_REG_1,
                        BPF_REG_0,
                        offsetof!(bpf_sock_addr, $field[$off]),
                    ),
                    BPF_EXIT_INSN(),
                ],
                result: $res,
                prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
                expected_attach_type: BPF_CGROUP_UDP6_SENDMSG,
                errstr: $err,
                flags: $flgs,
            }
        }
    };
}

/* user_ip6[0] is u64 aligned */
BPF_SOCK_ADDR_STORE!(user_ip6, 0, ACCEPT, None, 0),
BPF_SOCK_ADDR_STORE!(
    user_ip6,
    1,
    REJECT,
    Some("invalid bpf_context access off=12 size=8"),
    F_NEEDS_EFFICIENT_UNALIGNED_ACCESS,
),
BPF_SOCK_ADDR_STORE!(user_ip6, 2, ACCEPT, None, 0),
BPF_SOCK_ADDR_STORE!(
    user_ip6,
    3,
    REJECT,
    Some("invalid bpf_context access off=20 size=8"),
    F_NEEDS_EFFICIENT_UNALIGNED_ACCESS,
),

/* msg_src_ip6[0] is _not_ u64 aligned */
BPF_SOCK_ADDR_STORE!(
    msg_src_ip6,
    0,
    REJECT,
    Some("invalid bpf_context access off=44 size=8"),
    F_NEEDS_EFFICIENT_UNALIGNED_ACCESS,
),
BPF_SOCK_ADDR_STORE!(msg_src_ip6, 1, ACCEPT, None, 0),
BPF_SOCK_ADDR_STORE!(
    msg_src_ip6,
    2,
    REJECT,
    Some("invalid bpf_context access off=52 size=8"),
    F_NEEDS_EFFICIENT_UNALIGNED_ACCESS,
),
BPF_SOCK_ADDR_STORE!(
    msg_src_ip6,
    3,
    REJECT,
    Some("invalid bpf_context access off=56 size=8"),
    0,
),

macro_rules! BPF_SOCK_ADDR_LOAD {
    ($field:ident, $off:expr, $res:expr, $err:expr, $flgs:expr) => {
        {
            test_val {
                descr: concat!(
                    "wide load from bpf_sock_addr.",
                    stringify!($field),
                    "[",
                    stringify!($off),
                    "]",
                ),
                insns: [
                    BPF_LDX_MEM(
                        BPF_DW,
                        BPF_REG_0,
                        BPF_REG_1,
                        offsetof!(bpf_sock_addr, $field[$off]),
                    ),
                    BPF_MOV64_IMM(BPF_REG_0, 1),
                    BPF_EXIT_INSN(),
                ],
                result: $res,
                prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
                expected_attach_type: BPF_CGROUP_UDP6_SENDMSG,
                errstr: $err,
                flags: $flgs,
            }
        }
    };
}

/* user_ip6[0] is u64 aligned */
BPF_SOCK_ADDR_LOAD!(user_ip6, 0, ACCEPT, None, 0),
BPF_SOCK_ADDR_LOAD!(
    user_ip6,
    1,
    REJECT,
    Some("invalid bpf_context access off=12 size=8"),
    F_NEEDS_EFFICIENT_UNALIGNED_ACCESS,
),
BPF_SOCK_ADDR_LOAD!(user_ip6, 2, ACCEPT, None, 0),
BPF_SOCK_ADDR_LOAD!(
    user_ip6,
    3,
    REJECT,
    Some("invalid bpf_context access off=20 size=8"),
    F_NEEDS_EFFICIENT_UNALIGNED_ACCESS,
),

/* msg_src_ip6[0] is _not_ u64 aligned */
BPF_SOCK_ADDR_LOAD!(
    msg_src_ip6,
    0,
    REJECT,
    Some("invalid bpf_context access off=44 size=8"),
    F_NEEDS_EFFICIENT_UNALIGNED_ACCESS,
),
BPF_SOCK_ADDR_LOAD!(msg_src_ip6, 1, ACCEPT, None, 0),
BPF_SOCK_ADDR_LOAD!(
    msg_src_ip6,
    2,
    REJECT,
    Some("invalid bpf_context access off=52 size=8"),
    F_NEEDS_EFFICIENT_UNALIGNED_ACCESS,
),
BPF_SOCK_ADDR_LOAD!(
    msg_src_ip6,
    3,
    REJECT,
    Some("invalid bpf_context access off=56 size=8"),
    0,
),

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
