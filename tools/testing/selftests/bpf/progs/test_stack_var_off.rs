// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// SEC(...) is provided by the BPF build environment in the original source.

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
}

#[no_mangle]
pub static mut probe_res: i32 = 0;

#[no_mangle]
pub static mut input: [u8; 4] = [0; 4];

#[no_mangle]
pub static mut test_pid: i32 = 0;

// SEC("tracepoint/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn probe(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    /* This BPF program performs variable-offset reads and writes on a
     * stack-allocated buffer.
     */
    let mut stack_buf: [u8; 16] = [0; 16];
    let len: usize;
    let last: usize;

    if ((bpf_get_current_pid_tgid() >> 32) as i32) != test_pid {
        return 0;
    }

    /* Copy the input to the stack. */
    core::ptr::copy_nonoverlapping(input.as_ptr(), stack_buf.as_mut_ptr(), 4);

    /* The first byte in the buffer indicates the length. */
    len = (stack_buf[0] & 0xf) as usize;
    last = len.wrapping_sub(1) & 0xf;

    /* Append something to the buffer. The offset where we write is not
     * statically known; this is a variable-offset stack write.
     */
    stack_buf[len] = 42;

    /* Index into the buffer at an unknown offset. This is a
     * variable-offset stack read.
     *
     * Note that if it wasn't for the preceding variable-offset write, this
     * read would be rejected because the stack slot cannot be verified as
     * being initialized. With the preceding variable-offset write, the
     * stack slot still cannot be verified, but the write inhibits the
     * respective check on the reasoning that, if there was a
     * variable-offset to a higher-or-equal spot, we're probably reading
     * what we just wrote.
     */
    probe_res = stack_buf[last] as i32;
    return 0;
}

// char _license[] SEC("license") = "GPL";
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
