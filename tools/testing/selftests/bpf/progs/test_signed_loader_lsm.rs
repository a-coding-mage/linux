// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C includes:
// "vmlinux.h", <bpf/bpf_helpers.h>, and <bpf/bpf_tracing.h>.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u32 = u32;

#[repr(C)]
pub struct bpf_prog {
    pub aux: *mut bpf_prog_aux,
}

#[repr(C)]
pub struct bpf_prog_aux {
    pub sig: bpf_prog_aux_sig,
}

#[repr(C)]
pub struct bpf_prog_aux_sig {
    pub keyring_serial: i32,
    pub keyring_type: i32,
    pub verdict: i32,
}

#[repr(C)]
pub struct bpf_attr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_token {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut monitored_tid: __u32 = 0;

#[no_mangle]
pub static mut sig_keyring_serial: i32 = 0;
#[no_mangle]
pub static mut sig_keyring_type: i32 = 0;
#[no_mangle]
pub static mut sig_verdict: i32 = 0;
#[no_mangle]
pub static mut seen: i32 = 0;

#[no_mangle]
#[link_section = "lsm/bpf_prog_load"]
pub unsafe extern "C" fn inspect_prog_load(
    prog: *mut bpf_prog,
    attr: *mut bpf_attr,
    token: *mut bpf_token,
    kernel: bool,
) -> i32 {
    let _ = attr;
    let _ = token;
    let _ = kernel;

    let tid: __u32 = (bpf_get_current_pid_tgid() & 0xffffffff) as __u32;

    if monitored_tid == 0 || tid != monitored_tid {
        return 0;
    }

    seen += 1;
    sig_keyring_serial = (*(*prog).aux).sig.keyring_serial;
    sig_keyring_type = (*(*prog).aux).sig.keyring_type;
    sig_verdict = (*(*prog).aux).sig.verdict;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
