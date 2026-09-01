Test {
	name: "scale: scale test 1",
	insns: &[],
	data: &[],
	fill_helper: Some(bpf_fill_scale),
	prog_type: BPF_PROG_TYPE_SCHED_CLS,
	result: ACCEPT,
	retval: 1,
},
Test {
	name: "scale: scale test 2",
	insns: &[],
	data: &[],
	fill_helper: Some(bpf_fill_scale),
	prog_type: BPF_PROG_TYPE_SCHED_CLS,
	result: ACCEPT,
	retval: 2,
},

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
