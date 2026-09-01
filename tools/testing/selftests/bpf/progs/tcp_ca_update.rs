// SPDX-License-Identifier: GPL-2.0

// Depends on declarations from "bpf_tracing_net.h", <bpf/bpf_helpers.h>,
// and <bpf/bpf_tracing.h>.

use core::ffi::c_void;

extern "C" {
    fn tcp_sk(sk: *mut sock) -> *mut tcp_sock;
}

#[repr(C)]
pub struct sock {
    _data: [u8; 0],
}

#[repr(C)]
pub struct rate_sample {
    _data: [u8; 0],
}

#[repr(C)]
pub struct tcp_sock {
    pub snd_cwnd: u32,
    pub snd_ssthresh: u32,
}

#[repr(C)]
pub struct tcp_congestion_ops {
    pub init: *mut c_void,
    pub cong_control: *mut c_void,
    pub ssthresh: *mut c_void,
    pub undo_cwnd: *mut c_void,
    pub name: [u8; 16],
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut ca1_cnt: i32 = 0;

#[no_mangle]
pub static mut ca2_cnt: i32 = 0;

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn ca_update_1_init(sk: *mut sock) {
    let _ = sk;
    ca1_cnt += 1;
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn ca_update_2_init(sk: *mut sock) {
    let _ = sk;
    ca2_cnt += 1;
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn ca_update_cong_control(sk: *mut sock, rs: *const rate_sample) {
    let _ = sk;
    let _ = rs;
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn ca_update_ssthresh(sk: *mut sock) -> u32 {
    (*tcp_sk(sk)).snd_ssthresh
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn ca_update_undo_cwnd(sk: *mut sock) -> u32 {
    (*tcp_sk(sk)).snd_cwnd
}

#[no_mangle]
#[link_section = ".struct_ops.link"]
pub static mut ca_update_1: tcp_congestion_ops = tcp_congestion_ops {
    init: ca_update_1_init as *mut c_void,
    cong_control: ca_update_cong_control as *mut c_void,
    ssthresh: ca_update_ssthresh as *mut c_void,
    undo_cwnd: ca_update_undo_cwnd as *mut c_void,
    name: *b"tcp_ca_update\0\0\0",
};

#[no_mangle]
#[link_section = ".struct_ops.link"]
pub static mut ca_update_2: tcp_congestion_ops = tcp_congestion_ops {
    init: ca_update_2_init as *mut c_void,
    cong_control: ca_update_cong_control as *mut c_void,
    ssthresh: ca_update_ssthresh as *mut c_void,
    undo_cwnd: ca_update_undo_cwnd as *mut c_void,
    name: *b"tcp_ca_update\0\0\0",
};

#[no_mangle]
#[link_section = ".struct_ops.link"]
pub static mut ca_wrong: tcp_congestion_ops = tcp_congestion_ops {
    init: core::ptr::null_mut(),
    cong_control: ca_update_cong_control as *mut c_void,
    ssthresh: ca_update_ssthresh as *mut c_void,
    undo_cwnd: ca_update_undo_cwnd as *mut c_void,
    name: *b"tcp_ca_wrong\0\0\0\0",
};

#[no_mangle]
#[link_section = ".struct_ops"]
pub static mut ca_no_link: tcp_congestion_ops = tcp_congestion_ops {
    init: core::ptr::null_mut(),
    cong_control: ca_update_cong_control as *mut c_void,
    ssthresh: ca_update_ssthresh as *mut c_void,
    undo_cwnd: ca_update_undo_cwnd as *mut c_void,
    name: *b"tcp_ca_no_link\0",
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
