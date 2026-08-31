bpf_test {
	descr: "jset: functional",
	insns: &[
	BPF_DIRECT_PKT_R2,
	BPF_LDX_MEM(BPF_DW, BPF_REG_7, BPF_REG_2, 0),

	/* reg, bit 63 or bit 0 set, taken */
	BPF_LD_IMM64(BPF_REG_8, 0x8000000000000001u64 as i64),
	BPF_JMP_REG(BPF_JSET, BPF_REG_7, BPF_REG_8, 1),
	BPF_EXIT_INSN(),

	/* reg, bit 62, not taken */
	BPF_LD_IMM64(BPF_REG_8, 0x4000000000000000u64 as i64),
	BPF_JMP_REG(BPF_JSET, BPF_REG_7, BPF_REG_8, 1),
	BPF_JMP_IMM(BPF_JA, 0, 0, 1),
	BPF_EXIT_INSN(),

	/* imm, any bit set, taken */
	BPF_JMP_IMM(BPF_JSET, BPF_REG_7, -1, 1),
	BPF_EXIT_INSN(),

	/* imm, bit 31 set, taken */
	BPF_JMP_IMM(BPF_JSET, BPF_REG_7, 0x80000000u32 as i32, 1),
	BPF_EXIT_INSN(),

	/* all good - return r0 == 2 */
	BPF_MOV64_IMM(BPF_REG_0, 2),
	BPF_EXIT_INSN(),
	],
	prog_type: BPF_PROG_TYPE_SCHED_CLS,
	result: ACCEPT,
	runs: 7,
	retvals: &[
		bpf_test_run {
			retval: 2,
			data64: [ (1u64 << 63) | ((1u32 as u64) << 31) | ((1u32 as u64) << 0), ],
		},
		bpf_test_run {
			retval: 2,
			data64: [ (1u64 << 63) | ((1u32 as u64) << 31), ],
		},
		bpf_test_run {
			retval: 2,
			data64: [ ((1u32 as u64) << 31) | ((1u32 as u64) << 0), ],
		},
		bpf_test_run {
			retval: 2,
			data64: [ (-1i32 as u32) as u64, ],
		},
		bpf_test_run {
			retval: 2,
			data64: [ !0x4000000000000000u64, ],
		},
		bpf_test_run {
			retval: 0,
			data64: [ 0, ],
		},
		bpf_test_run {
			retval: 0,
			data64: [ !0u64, ],
		},
	],
	flags: F_NEEDS_EFFICIENT_UNALIGNED_ACCESS,
},
bpf_test {
	descr: "jset: sign-extend",
	insns: &[
	BPF_DIRECT_PKT_R2,
	BPF_LDX_MEM(BPF_DW, BPF_REG_7, BPF_REG_2, 0),

	BPF_JMP_IMM(BPF_JSET, BPF_REG_7, 0x80000000u32 as i32, 1),
	BPF_EXIT_INSN(),

	BPF_MOV64_IMM(BPF_REG_0, 2),
	BPF_EXIT_INSN(),
	],
	prog_type: BPF_PROG_TYPE_SCHED_CLS,
	result: ACCEPT,
	retval: 2,
	data: [ 1, 0, 0, 0, 0, 0, 0, 1, ],
	flags: F_NEEDS_EFFICIENT_UNALIGNED_ACCESS,
},
bpf_test {
	descr: "jset: known const compare",
	insns: &[
	BPF_MOV64_IMM(BPF_REG_0, 1),
	BPF_JMP_IMM(BPF_JSET, BPF_REG_0, 1, 1),
	/* unpriv: nospec (inserted to prevent "R9 !read_ok") */
	BPF_LDX_MEM(BPF_B, BPF_REG_8, BPF_REG_9, 0),
	BPF_EXIT_INSN(),
	],
	prog_type: BPF_PROG_TYPE_SOCKET_FILTER,
	retval: 1,
	result: ACCEPT,
},
bpf_test {
	descr: "jset: known const compare bad",
	insns: &[
	BPF_MOV64_IMM(BPF_REG_0, 0),
	BPF_JMP_IMM(BPF_JSET, BPF_REG_0, 1, 1),
	BPF_LDX_MEM(BPF_B, BPF_REG_8, BPF_REG_9, 0),
	BPF_EXIT_INSN(),
	],
	prog_type: BPF_PROG_TYPE_SOCKET_FILTER,
	errstr_unpriv: "!read_ok",
	result_unpriv: REJECT,
	errstr: "!read_ok",
	result: REJECT,
},
bpf_test {
	descr: "jset: unknown const compare taken",
	insns: &[
	BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_get_prandom_u32),
	BPF_JMP_IMM(BPF_JSET, BPF_REG_0, 1, 1),
	BPF_JMP_IMM(BPF_JA, 0, 0, 1),
	BPF_LDX_MEM(BPF_B, BPF_REG_8, BPF_REG_9, 0),
	BPF_EXIT_INSN(),
	],
	prog_type: BPF_PROG_TYPE_SOCKET_FILTER,
	errstr_unpriv: "!read_ok",
	result_unpriv: REJECT,
	errstr: "!read_ok",
	result: REJECT,
},
bpf_test {
	descr: "jset: unknown const compare not taken",
	insns: &[
	BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_get_prandom_u32),
	BPF_JMP_IMM(BPF_JSET, BPF_REG_0, 1, 1),
	BPF_LDX_MEM(BPF_B, BPF_REG_8, BPF_REG_9, 0),
	BPF_EXIT_INSN(),
	],
	prog_type: BPF_PROG_TYPE_SOCKET_FILTER,
	errstr_unpriv: "!read_ok",
	result_unpriv: REJECT,
	errstr: "!read_ok",
	result: REJECT,
},
bpf_test {
	descr: "jset: half-known const compare",
	insns: &[
	BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_get_prandom_u32),
	BPF_ALU64_IMM(BPF_OR, BPF_REG_0, 2),
	BPF_JMP_IMM(BPF_JSET, BPF_REG_0, 3, 1),
	/* unpriv: nospec (inserted to prevent "R9 !read_ok") */
	BPF_LDX_MEM(BPF_B, BPF_REG_8, BPF_REG_9, 0),
	BPF_MOV64_IMM(BPF_REG_0, 0),
	BPF_EXIT_INSN(),
	],
	prog_type: BPF_PROG_TYPE_SOCKET_FILTER,
	result: ACCEPT,
},
bpf_test {
	descr: "jset: range",
	insns: &[
	BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_get_prandom_u32),
	BPF_MOV64_REG(BPF_REG_1, BPF_REG_0),
	BPF_MOV64_IMM(BPF_REG_0, 0),
	BPF_ALU64_IMM(BPF_AND, BPF_REG_1, 0xff),
	BPF_JMP_IMM(BPF_JSET, BPF_REG_1, 0xf0, 3),
	BPF_JMP_IMM(BPF_JLT, BPF_REG_1, 0x10, 1),
	/* unpriv: nospec (inserted to prevent "R9 !read_ok") */
	BPF_LDX_MEM(BPF_B, BPF_REG_8, BPF_REG_9, 0),
	BPF_EXIT_INSN(),
	BPF_JMP_IMM(BPF_JSET, BPF_REG_1, 0x10, 1),
	BPF_EXIT_INSN(),
	BPF_JMP_IMM(BPF_JGE, BPF_REG_1, 0x10, 1),
	/* unpriv: nospec (inserted to prevent "R9 !read_ok") */
	BPF_LDX_MEM(BPF_B, BPF_REG_8, BPF_REG_9, 0),
	BPF_EXIT_INSN(),
	],
	prog_type: BPF_PROG_TYPE_SOCKET_FILTER,
	result: ACCEPT,
},
