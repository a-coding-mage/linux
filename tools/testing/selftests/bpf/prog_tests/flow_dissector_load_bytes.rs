// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, <network_helpers.h>

use core::ffi::c_void;

unsafe extern "C" {
    static pkt_v4: c_void;

    fn bpf_test_load_program(
        prog_type: u32,
        insns: *const bpf_insn,
        insn_cnt: usize,
        license: *const i8,
        kern_version: u32,
        log_buf: *mut c_void,
        log_buf_sz: u32,
    ) -> i32;
    fn bpf_prog_test_run_opts(fd: i32, opts: *mut bpf_test_run_opts) -> i32;
    fn close(fd: i32) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_flow_dissector_load_bytes() {
    let mut flow_keys: bpf_flow_keys = unsafe { core::mem::zeroed() };
    let prog: [bpf_insn; 10] = [
        // BPF_REG_1 - 1st argument: context
        // BPF_REG_2 - 2nd argument: offset, start at first byte
        BPF_MOV64_IMM!(BPF_REG_2, 0),
        // BPF_REG_3 - 3rd argument: destination, reserve byte on stack
        BPF_ALU64_REG!(BPF_MOV, BPF_REG_3, BPF_REG_10),
        BPF_ALU64_IMM!(BPF_ADD, BPF_REG_3, -1),
        // BPF_REG_4 - 4th argument: copy one byte
        BPF_MOV64_IMM!(BPF_REG_4, 1),
        // bpf_skb_load_bytes(ctx, sizeof(pkt_v4), ptr, 1)
        BPF_RAW_INSN!(
            BPF_JMP | BPF_CALL,
            0,
            0,
            0,
            BPF_FUNC_skb_load_bytes
        ),
        BPF_JMP_IMM!(BPF_JNE, BPF_REG_0, 0, 2),
        // if (ret == 0) return BPF_DROP (2)
        BPF_MOV64_IMM!(BPF_REG_0, BPF_DROP),
        BPF_EXIT_INSN!(),
        // if (ret != 0) return BPF_OK (0)
        BPF_MOV64_IMM!(BPF_REG_0, BPF_OK),
        BPF_EXIT_INSN!(),
    ];
    let mut fd: i32;
    let mut err: i32;
    let mut topts = LIBBPF_OPTS!(
        bpf_test_run_opts,
        data_in = unsafe { &pkt_v4 as *const _ as *const c_void },
        data_size_in = core::mem::size_of_val(unsafe { &pkt_v4 }),
        data_out = &mut flow_keys as *mut _ as *mut c_void,
        data_size_out = core::mem::size_of::<bpf_flow_keys>(),
        repeat = 1,
    );

    /* make sure bpf_skb_load_bytes is not allowed from skb-less context
     */
    fd = unsafe {
        bpf_test_load_program(
            BPF_PROG_TYPE_FLOW_DISSECTOR,
            prog.as_ptr(),
            ARRAY_SIZE!(prog),
            b"GPL\0".as_ptr() as *const i8,
            0,
            core::ptr::null_mut(),
            0,
        )
    };
    ASSERT_GE!(fd, 0, "bpf_test_load_program good fd");

    err = unsafe { bpf_prog_test_run_opts(fd, &mut topts) };
    ASSERT_OK!(err, "test_run");
    ASSERT_EQ!(
        topts.data_size_out,
        core::mem::size_of::<bpf_flow_keys>(),
        "test_run data_size_out"
    );
    ASSERT_EQ!(topts.retval, BPF_OK, "test_run retval");

    if fd >= -1 {
        unsafe {
            close(fd);
        }
    }
}
