bpf_test {
	descr: "junk insn",
	insns: [
		BPF_RAW_INSN!(0, 0, 0, 0, 0),
		BPF_EXIT_INSN!(),
	],
	errstr: "unknown opcode 00",
	result: REJECT,
},
bpf_test {
	descr: "junk insn2",
	insns: [
		BPF_RAW_INSN!(BPF_LDX | BPF_MEM | BPF_W, 0, 0, 0, 1),
		BPF_EXIT_INSN!(),
	],
	errstr: "BPF_LDX uses reserved fields",
	result: REJECT,
},
bpf_test {
	descr: "junk insn3",
	insns: [
		BPF_RAW_INSN!(-1, 0, 0, 0, 0),
		BPF_EXIT_INSN!(),
	],
	errstr: "unknown opcode ff",
	result: REJECT,
},
bpf_test {
	descr: "junk insn4",
	insns: [
		BPF_RAW_INSN!(-1, 0, 0, -1, -1),
		BPF_EXIT_INSN!(),
	],
	errstr: "unknown opcode ff",
	result: REJECT,
},
bpf_test {
	descr: "junk insn5",
	insns: [
		BPF_RAW_INSN!(0x7f, 0, 0, -1, -1),
		BPF_EXIT_INSN!(),
	],
	errstr: "BPF_ALU uses reserved fields",
	result: REJECT,
},

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
