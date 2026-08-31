// Source-level translation of testing/selftests/bpf/verifier/ld_imm64.c.
// Harness types, constants, and BPF instruction helpers are provided externally.

pub static LD_IMM64_TESTS: &[Test] = &[
    Test {
        name: "test1 ld_imm64",
        insns: &[
            BPF_JMP_IMM(BPF_JEQ, BPF_REG_1, 0, 1),
            BPF_LD_IMM64(BPF_REG_0, 0),
            BPF_LD_IMM64(BPF_REG_0, 0),
            BPF_LD_IMM64(BPF_REG_0, 1),
            BPF_LD_IMM64(BPF_REG_0, 1),
            BPF_MOV64_IMM(BPF_REG_0, 2),
            BPF_EXIT_INSN(),
        ],
        errstr: Some("jump into the middle of ldimm64 insn 1"),
        errstr_unpriv: Some("jump into the middle of ldimm64 insn 1"),
        result: REJECT,
        ..Default::default()
    },
    Test {
        name: "test2 ld_imm64",
        insns: &[
            BPF_JMP_IMM(BPF_JEQ, BPF_REG_1, 0, 1),
            BPF_LD_IMM64(BPF_REG_0, 0),
            BPF_LD_IMM64(BPF_REG_0, 0),
            BPF_LD_IMM64(BPF_REG_0, 1),
            BPF_LD_IMM64(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        errstr: Some("jump into the middle of ldimm64 insn 1"),
        errstr_unpriv: Some("jump into the middle of ldimm64 insn 1"),
        result: REJECT,
        ..Default::default()
    },
    Test {
        name: "test3 ld_imm64",
        insns: &[
            BPF_JMP_IMM(BPF_JEQ, BPF_REG_1, 0, 1),
            BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, 0, 0, 0),
            BPF_LD_IMM64(BPF_REG_0, 0),
            BPF_LD_IMM64(BPF_REG_0, 0),
            BPF_LD_IMM64(BPF_REG_0, 1),
            BPF_LD_IMM64(BPF_REG_0, 1),
            BPF_EXIT_INSN(),
        ],
        errstr: Some("invalid bpf_ld_imm64 insn"),
        result: REJECT,
        ..Default::default()
    },
    Test {
        name: "test4 ld_imm64",
        insns: &[
            BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, 0, 0, 0),
            BPF_EXIT_INSN(),
        ],
        errstr: Some("invalid bpf_ld_imm64 insn"),
        result: REJECT,
        ..Default::default()
    },
    Test {
        name: "test6 ld_imm64",
        insns: &[
            BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, 0, 0, 0),
            BPF_RAW_INSN(0, 0, 0, 0, 0),
            BPF_EXIT_INSN(),
        ],
        result: ACCEPT,
        ..Default::default()
    },
    Test {
        name: "test7 ld_imm64",
        insns: &[
            BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, 0, 0, 1),
            BPF_RAW_INSN(0, 0, 0, 0, 1),
            BPF_EXIT_INSN(),
        ],
        result: ACCEPT,
        retval: 1,
        ..Default::default()
    },
    Test {
        name: "test8 ld_imm64",
        insns: &[
            BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, 0, 1, 1),
            BPF_RAW_INSN(0, 0, 0, 0, 1),
            BPF_EXIT_INSN(),
        ],
        errstr: Some("uses reserved fields"),
        result: REJECT,
        ..Default::default()
    },
    Test {
        name: "test9 ld_imm64",
        insns: &[
            BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, 0, 0, 1),
            BPF_RAW_INSN(0, 0, 0, 1, 1),
            BPF_EXIT_INSN(),
        ],
        errstr: Some("invalid bpf_ld_imm64 insn"),
        result: REJECT,
        ..Default::default()
    },
    Test {
        name: "test10 ld_imm64",
        insns: &[
            BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, 0, 0, 1),
            BPF_RAW_INSN(0, BPF_REG_1, 0, 0, 1),
            BPF_EXIT_INSN(),
        ],
        errstr: Some("invalid bpf_ld_imm64 insn"),
        result: REJECT,
        ..Default::default()
    },
    Test {
        name: "test11 ld_imm64",
        insns: &[
            BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, 0, 0, 1),
            BPF_RAW_INSN(0, 0, BPF_REG_1, 0, 1),
            BPF_EXIT_INSN(),
        ],
        errstr: Some("invalid bpf_ld_imm64 insn"),
        result: REJECT,
        ..Default::default()
    },
    Test {
        name: "test12 ld_imm64",
        insns: &[
            BPF_MOV64_IMM(BPF_REG_1, 0),
            BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, BPF_REG_1, 0, 1),
            BPF_RAW_INSN(0, 0, 0, 0, 0),
            BPF_EXIT_INSN(),
        ],
        errstr: Some("not pointing to valid bpf_map"),
        result: REJECT,
        ..Default::default()
    },
    Test {
        name: "test13 ld_imm64",
        insns: &[
            BPF_MOV64_IMM(BPF_REG_1, 0),
            BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, 0, BPF_REG_1, 0, 1),
            BPF_RAW_INSN(0, 0, BPF_REG_1, 0, 1),
            BPF_EXIT_INSN(),
        ],
        errstr: Some("invalid bpf_ld_imm64 insn"),
        result: REJECT,
        ..Default::default()
    },
    Test {
        name: "test14 ld_imm64: reject 2nd imm != 0",
        insns: &[
            BPF_MOV64_IMM(BPF_REG_0, 0),
            BPF_RAW_INSN(BPF_LD | BPF_IMM | BPF_DW, BPF_REG_1, BPF_PSEUDO_MAP_FD, 0, 0),
            BPF_RAW_INSN(0, 0, 0, 0, 0xfefefe),
            BPF_EXIT_INSN(),
        ],
        fixup_map_hash_48b: &[1],
        errstr: Some("unrecognized bpf_ld_imm64 insn"),
        result: REJECT,
        ..Default::default()
    },
];
