// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include "bpf_tracing_net.h"
// #include <bpf/bpf_core_read.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

extern "C" {
    fn bpf_rdonly_cast(obj: *const core::ffi::c_void, btf_id: u32) -> *mut core::ffi::c_void;
    fn bpf_core_type_id_kernel_tcp_sock() -> u32;
}

#[repr(C)]
pub struct sock {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct tcp_sock {
    pub snd_cwnd: u32,
}

#[repr(C)]
pub struct tcp_congestion_ops {
    pub init: *mut core::ffi::c_void,
    pub name: [u8; 16],
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn untrusted_btf_write_init(sk: *mut sock) {
    let mut tp: *mut tcp_sock;
    let mut v: i32 = 1;
    let mut p: *mut core::ffi::c_void;

    p = bpf_rdonly_cast(
        (&mut v as *mut i32).cast::<core::ffi::c_void>(),
        0,
    );
    tp = bpf_rdonly_cast(p, bpf_core_type_id_kernel_tcp_sock()).cast::<tcp_sock>();
    (*tp).snd_cwnd = 1;

    let _ = sk;
}

#[no_mangle]
#[link_section = ".struct_ops"]
pub static mut untrusted_btf_write: tcp_congestion_ops = tcp_congestion_ops {
    init: untrusted_btf_write_init as *mut core::ffi::c_void,
    name: [
        b'b', b'p', b'f', b'_', b'r', b'o', b'_', b'b', b't', b'f', 0, 0, 0, 0, 0, 0,
    ],
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
