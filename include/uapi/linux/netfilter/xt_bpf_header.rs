/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from xt_bpf.h.
// Dependency intent: struct sock_filter is supplied by <linux/filter.h>.

pub const XT_BPF_MAX_NUM_INSTR: usize = 64;
pub const XT_BPF_PATH_MAX: usize =
    XT_BPF_MAX_NUM_INSTR * core::mem::size_of::<sock_filter>();

#[repr(C)]
pub struct bpf_prog {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xt_bpf_info {
    pub bpf_program_num_elem: u16,
    pub bpf_program: [sock_filter; XT_BPF_MAX_NUM_INSTR],

    /* only used in the kernel */
    pub filter: *mut bpf_prog,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum xt_bpf_modes {
    XT_BPF_MODE_BYTECODE,
    XT_BPF_MODE_FD_PINNED,
    XT_BPF_MODE_FD_ELF,
}

pub const XT_BPF_MODE_PATH_PINNED: xt_bpf_modes = xt_bpf_modes::XT_BPF_MODE_FD_PINNED;

#[repr(C)]
pub union xt_bpf_info_v1_data {
    pub bpf_program: [sock_filter; XT_BPF_MAX_NUM_INSTR],
    pub path: [core::ffi::c_char; XT_BPF_PATH_MAX],
}

#[repr(C)]
pub struct xt_bpf_info_v1 {
    pub mode: u16,
    pub bpf_program_num_elem: u16,
    pub fd: i32,
    pub data: xt_bpf_info_v1_data,

    /* only used in the kernel */
    pub filter: *mut bpf_prog,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
