// Translated from testing/selftests/bpf/verifier/atomic_invalid.c.
// BPF instruction constructors, constants, and the verifier test-entry type are
// supplied by the surrounding selftest harness.

macro_rules! __INVALID_ATOMIC_ACCESS_TEST {
    ($op:expr) => {
        VerifierTest {
            descr: concat!(
                "atomic ",
                stringify!($op),
                " access through non-pointer "
            ),
            insns: vec![
                BPF_MOV64_IMM(BPF_REG_0, 1),
                BPF_MOV64_IMM(BPF_REG_1, 0),
                BPF_ATOMIC_OP(BPF_DW, $op, BPF_REG_1, BPF_REG_0, -8),
                BPF_MOV64_IMM(BPF_REG_0, 0),
                BPF_EXIT_INSN(),
            ],
            result: REJECT,
            errstr: "R1 invalid mem access 'scalar'",
        }
    };
}

pub static ATOMIC_INVALID_TESTS: &[VerifierTest] = &[
    __INVALID_ATOMIC_ACCESS_TEST!(BPF_ADD),
    __INVALID_ATOMIC_ACCESS_TEST!(BPF_ADD | BPF_FETCH),
    __INVALID_ATOMIC_ACCESS_TEST!(BPF_ADD),
    __INVALID_ATOMIC_ACCESS_TEST!(BPF_ADD | BPF_FETCH),
    __INVALID_ATOMIC_ACCESS_TEST!(BPF_AND),
    __INVALID_ATOMIC_ACCESS_TEST!(BPF_AND | BPF_FETCH),
    __INVALID_ATOMIC_ACCESS_TEST!(BPF_OR),
    __INVALID_ATOMIC_ACCESS_TEST!(BPF_OR | BPF_FETCH),
    __INVALID_ATOMIC_ACCESS_TEST!(BPF_XOR),
    __INVALID_ATOMIC_ACCESS_TEST!(BPF_XOR | BPF_FETCH),
    __INVALID_ATOMIC_ACCESS_TEST!(BPF_XCHG),
    __INVALID_ATOMIC_ACCESS_TEST!(BPF_CMPXCHG),
];
