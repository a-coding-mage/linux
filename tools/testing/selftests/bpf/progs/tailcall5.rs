// SPDX-License-Identifier: GPL-2.0
// C source included <linux/bpf.h> and <bpf/bpf_helpers.h>.

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

pub const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;

#[repr(C)]
pub struct jmp_table {
    pub type_: *mut [u32; BPF_MAP_TYPE_PROG_ARRAY as usize],
    pub max_entries: *mut [u32; 3],
    pub key_size: *mut [u32; core::mem::size_of::<u32>()],
    pub value_size: *mut [u32; core::mem::size_of::<u32>()],
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut jmp_table: jmp_table = jmp_table {
    type_: core::ptr::null_mut(),
    max_entries: core::ptr::null_mut(),
    key_size: core::ptr::null_mut(),
    value_size: core::ptr::null_mut(),
};

#[unsafe(no_mangle)]
pub static mut selector: i32 = 0;

unsafe extern "C" {
    pub fn bpf_tail_call(ctx: *mut __sk_buff, prog_array_map: *mut jmp_table, index: i32) -> i32;
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn classifier_0(_skb: *mut __sk_buff) -> i32 {
    return 0;
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn classifier_1(_skb: *mut __sk_buff) -> i32 {
    return 1;
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn classifier_2(_skb: *mut __sk_buff) -> i32 {
    return 2;
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn entry(skb: *mut __sk_buff) -> i32 {
    let mut idx: i32 = 0;

    if selector == 1234 {
        idx = 1;
    } else if selector == 5678 {
        idx = 2;
    }

    bpf_tail_call(skb, &raw mut jmp_table, idx);
    return 3;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut __license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
