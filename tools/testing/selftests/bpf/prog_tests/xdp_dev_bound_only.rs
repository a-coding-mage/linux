// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C source:
// #include <net/if.h>
// #include <test_progs.h>
// #include <network_helpers.h>

use core::ffi::{c_char, c_int};
use core::mem;
use core::ptr;

const LOCAL_NETNS: &[u8] = b"xdp_dev_bound_only_netns\0";

type __u32 = u32;

const BPF_REG_0: c_int = 0;
const BPF_PROG_TYPE_XDP: c_int = 6;
const BPF_F_XDP_DEV_BOUND_ONLY: __u32 = 1 << 6;
const EINVAL: c_int = 22;

#[repr(C)]
pub struct bpf_insn {
    pub code: u8,
    pub dst_reg_src_reg: u8,
    pub off: i16,
    pub imm: i32,
}

#[repr(C)]
pub struct bpf_prog_load_opts {
    pub sz: usize,
    pub prog_flags: __u32,
    pub prog_ifindex: __u32,
}

#[repr(C)]
pub struct nstoken {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_prog_load(
        prog_type: c_int,
        prog_name: *mut c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: usize,
        opts: *const bpf_prog_load_opts,
    ) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> __u32;
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(tok: *mut nstoken);
    fn close(fd: c_int) -> c_int;
    fn perror(s: *const c_char);

    fn ASSERT_OK_PTR(ptr: *const nstoken, name: *const c_char) -> bool;
    fn ASSERT_NEQ(left: __u32, right: __u32, name: *const c_char) -> bool;
    fn ASSERT_GE(left: c_int, right: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: c_int, right: c_int, name: *const c_char) -> bool;
    fn SYS(label: *const c_char, cmd: *const c_char) -> c_int;
    fn SYS_NOFAIL(cmd: *const c_char) -> c_int;
}

const fn bpf_mov64_imm(dst: c_int, imm: i32) -> bpf_insn {
    bpf_insn {
        code: 0xb7,
        dst_reg_src_reg: dst as u8,
        off: 0,
        imm,
    }
}

const fn bpf_exit_insn() -> bpf_insn {
    bpf_insn {
        code: 0x95,
        dst_reg_src_reg: 0,
        off: 0,
        imm: 0,
    }
}

unsafe fn load_dummy_prog(name: *mut c_char, ifindex: __u32, flags: __u32) -> c_int {
    let insns = [bpf_mov64_imm(BPF_REG_0, 0), bpf_exit_insn()];
    let mut opts: bpf_prog_load_opts = mem::zeroed();

    opts.sz = mem::size_of::<bpf_prog_load_opts>();
    opts.prog_flags = flags;
    opts.prog_ifindex = ifindex;
    bpf_prog_load(
        BPF_PROG_TYPE_XDP,
        name,
        b"GPL\0".as_ptr() as *const c_char,
        insns.as_ptr(),
        insns.len(),
        &opts,
    )
}

/* A test case for bpf_offload_netdev->offload handling bug:
 * - create a veth device (does not support offload);
 * - create a device bound XDP program with BPF_F_XDP_DEV_BOUND_ONLY flag
 *   (such programs are not offloaded);
 * - create a device bound XDP program without flags (such programs are offloaded).
 * This might lead to 'BUG: kernel NULL pointer dereference'.
 */
#[no_mangle]
pub unsafe extern "C" fn test_xdp_dev_bound_only_offdev() {
    let mut tok: *mut nstoken = ptr::null_mut();
    let ifindex: __u32;
    let mut fd1: c_int = -1;
    let mut fd2: c_int = -1;

    SYS(
        b"out\0".as_ptr() as *const c_char,
        b"ip netns add xdp_dev_bound_only_netns\0".as_ptr() as *const c_char,
    );
    tok = open_netns(LOCAL_NETNS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(tok, b"open_netns\0".as_ptr() as *const c_char) {
        goto_out(fd1, fd2, tok);
        return;
    }
    SYS(
        b"out\0".as_ptr() as *const c_char,
        b"ip link add eth42 type veth\0".as_ptr() as *const c_char,
    );
    ifindex = if_nametoindex(b"eth42\0".as_ptr() as *const c_char);
    if !ASSERT_NEQ(ifindex, 0, b"if_nametoindex\0".as_ptr() as *const c_char) {
        perror(b"if_nametoindex\0".as_ptr() as *const c_char);
        goto_out(fd1, fd2, tok);
        return;
    }
    fd1 = load_dummy_prog(
        b"dummy1\0".as_ptr() as *mut c_char,
        ifindex,
        BPF_F_XDP_DEV_BOUND_ONLY,
    );
    if !ASSERT_GE(fd1, 0, b"load_dummy_prog #1\0".as_ptr() as *const c_char) {
        perror(b"load_dummy_prog #1\0".as_ptr() as *const c_char);
        goto_out(fd1, fd2, tok);
        return;
    }
    /* Program with ifindex is considered offloaded, however veth
     * does not support offload => error should be reported.
     */
    fd2 = load_dummy_prog(b"dummy2\0".as_ptr() as *mut c_char, ifindex, 0);
    ASSERT_EQ(
        fd2,
        -EINVAL,
        b"load_dummy_prog #2 (offloaded)\0".as_ptr() as *const c_char,
    );

    goto_out(fd1, fd2, tok);
}

unsafe fn goto_out(fd1: c_int, fd2: c_int, tok: *mut nstoken) {
    close(fd1);
    close(fd2);
    close_netns(tok);
    /* eth42 was added inside netns, removing the netns will
     * also remove eth42 veth pair.
     */
    SYS_NOFAIL(b"ip netns del xdp_dev_bound_only_netns\0".as_ptr() as *const c_char);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
