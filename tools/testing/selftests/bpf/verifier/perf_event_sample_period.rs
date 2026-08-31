Test {
    name: "check bpf_perf_event_data->sample_period byte load permitted",
    insns: &[
        BPF_MOV64_IMM!(BPF_REG_0, 0),
        #[cfg(target_endian = "little")]
        BPF_LDX_MEM!(
            BPF_B,
            BPF_REG_0,
            BPF_REG_1,
            offset_of!(bpf_perf_event_data, sample_period)
        ),
        #[cfg(target_endian = "big")]
        BPF_LDX_MEM!(
            BPF_B,
            BPF_REG_0,
            BPF_REG_1,
            offset_of!(bpf_perf_event_data, sample_period) + 7
        ),
        BPF_EXIT_INSN!(),
    ],
    result: ACCEPT,
    prog_type: BPF_PROG_TYPE_PERF_EVENT,
},
Test {
    name: "check bpf_perf_event_data->sample_period half load permitted",
    insns: &[
        BPF_MOV64_IMM!(BPF_REG_0, 0),
        #[cfg(target_endian = "little")]
        BPF_LDX_MEM!(
            BPF_H,
            BPF_REG_0,
            BPF_REG_1,
            offset_of!(bpf_perf_event_data, sample_period)
        ),
        #[cfg(target_endian = "big")]
        BPF_LDX_MEM!(
            BPF_H,
            BPF_REG_0,
            BPF_REG_1,
            offset_of!(bpf_perf_event_data, sample_period) + 6
        ),
        BPF_EXIT_INSN!(),
    ],
    result: ACCEPT,
    prog_type: BPF_PROG_TYPE_PERF_EVENT,
},
Test {
    name: "check bpf_perf_event_data->sample_period word load permitted",
    insns: &[
        BPF_MOV64_IMM!(BPF_REG_0, 0),
        #[cfg(target_endian = "little")]
        BPF_LDX_MEM!(
            BPF_W,
            BPF_REG_0,
            BPF_REG_1,
            offset_of!(bpf_perf_event_data, sample_period)
        ),
        #[cfg(target_endian = "big")]
        BPF_LDX_MEM!(
            BPF_W,
            BPF_REG_0,
            BPF_REG_1,
            offset_of!(bpf_perf_event_data, sample_period) + 4
        ),
        BPF_EXIT_INSN!(),
    ],
    result: ACCEPT,
    prog_type: BPF_PROG_TYPE_PERF_EVENT,
},
Test {
    name: "check bpf_perf_event_data->sample_period dword load permitted",
    insns: &[
        BPF_MOV64_IMM!(BPF_REG_0, 0),
        BPF_LDX_MEM!(
            BPF_DW,
            BPF_REG_0,
            BPF_REG_1,
            offset_of!(bpf_perf_event_data, sample_period)
        ),
        BPF_EXIT_INSN!(),
    ],
    result: ACCEPT,
    prog_type: BPF_PROG_TYPE_PERF_EVENT,
},
