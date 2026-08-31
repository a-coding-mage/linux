// SPDX-License-Identifier: GPL-2.0
// C dependencies: <asm/unistd.h>, <linux/bpf.h>, <unistd.h>

use core::ffi::{c_int, c_long, c_ulong, c_void};

#[cfg(target_arch = "x86")]
const __NR_bpf: c_long = 357;
#[cfg(target_arch = "x86_64")]
const __NR_bpf: c_long = 321;
#[cfg(target_arch = "aarch64")]
const __NR_bpf: c_long = 280;
#[cfg(target_arch = "sparc")]
const __NR_bpf: c_long = 349;
#[cfg(target_arch = "s390x")]
const __NR_bpf: c_long = 351;
#[cfg(all(target_arch = "mips", target_pointer_width = "32"))]
const __NR_bpf: c_long = 4355;
#[cfg(all(target_arch = "mips64", target_pointer_width = "64"))]
const __NR_bpf: c_long = 5315;
#[cfg(target_arch = "loongarch64")]
const __NR_bpf: c_long = 280;

const BPF_PROG_LOAD: c_int = 5;
const BPF_PROG_TYPE_KPROBE: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_attr {
    prog_type: u32,
    insn_cnt: u32,
    insns: u64,
    license: u64,
    log_level: u32,
    log_size: u32,
    log_buf: u64,
    kern_version: u32,
    prog_flags: u32,
}

unsafe extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
}

fn main() -> c_int {
    let mut attr = bpf_attr {
        prog_type: 0,
        insn_cnt: 0,
        insns: 0,
        license: 0,
        log_level: 0,
        log_size: 0,
        log_buf: 0,
        kern_version: 0,
        prog_flags: 0,
    };

    /* Check fields in attr */
    attr.prog_type = BPF_PROG_TYPE_KPROBE;
    attr.insn_cnt = 0;
    attr.insns = 0;
    attr.license = 0;
    attr.log_buf = 0;
    attr.log_size = 0;
    attr.log_level = 0;
    attr.kern_version = 0;
    attr.prog_flags = 0;

    /*
     * Test existence of __NR_bpf and BPF_PROG_LOAD.
     * This call should fail if we run the testcase.
     */
    (unsafe {
        syscall(
            __NR_bpf,
            BPF_PROG_LOAD,
            &mut attr as *mut bpf_attr as *mut c_void,
            core::mem::size_of::<bpf_attr>() as c_ulong,
        )
    } == 0) as c_int
}
