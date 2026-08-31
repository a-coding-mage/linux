// SPDX-License-Identifier: GPL-2.0
// C dependencies: "vmlinux.h" and <bpf/bpf_helpers.h>.

const BPF_MAP_TYPE_SOCKMAP: u32 = 15;
const SK_PASS: i32 = 1;

const POP_START: u32 = 0x48a3;
const POP_LEN: u32 = 0xfffffffd;

#[repr(C)]
pub struct sk_msg_md {
    pub data: *mut core::ffi::c_void,
    pub data_end: *mut core::ffi::c_void,
    pub family: u32,
    pub remote_ip4: u32,
    pub local_ip4: u32,
    pub remote_ip6: [u32; 4],
    pub local_ip6: [u32; 4],
    pub remote_port: u32,
    pub local_port: u32,
    pub size: u32,
}

#[repr(C)]
pub struct sock_map {
    // Original C declaration used libbpf BTF map-definition macros:
    // __uint(type, BPF_MAP_TYPE_SOCKMAP);
    // __uint(max_entries, 1);
    // __type(key, int);
    // __type(value, int);
    pub type_: u32,
    pub max_entries: u32,
    pub key: i32,
    pub value: i32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut sock_map: sock_map = sock_map {
    type_: BPF_MAP_TYPE_SOCKMAP,
    max_entries: 1,
    key: 0,
    value: 0,
};

#[no_mangle]
pub static mut pop_data_ret: i64 = 1;

extern "C" {
    fn bpf_msg_pop_data(msg: *mut sk_msg_md, start: u32, len: u32, flags: u64) -> i64;
}

#[link_section = "sk_msg"]
#[no_mangle]
pub unsafe extern "C" fn prog_msg_pop_data(msg: *mut sk_msg_md) -> i32 {
    if (*msg).size <= POP_START {
        return SK_PASS;
    }

    pop_data_ret = bpf_msg_pop_data(msg, POP_START, POP_LEN, 0);
    SK_PASS
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
