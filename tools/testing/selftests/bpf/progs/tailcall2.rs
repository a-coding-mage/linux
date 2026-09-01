// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h> and <bpf/bpf_helpers.h>.

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

pub const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;

#[repr(C)]
pub struct JmpTable {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut jmp_table: JmpTable = JmpTable {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 5,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u32>() as u32,
};

extern "C" {
    pub fn bpf_tail_call_static(skb: *mut __sk_buff, map: *mut JmpTable, index: u32);
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn classifier_0(skb: *mut __sk_buff) -> i32 {
    bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table), 1);
    return 0;
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn classifier_1(skb: *mut __sk_buff) -> i32 {
    bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table), 2);
    return 1;
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn classifier_2(_skb: *mut __sk_buff) -> i32 {
    return 2;
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn classifier_3(skb: *mut __sk_buff) -> i32 {
    bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table), 4);
    return 3;
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn classifier_4(skb: *mut __sk_buff) -> i32 {
    bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table), 3);
    return 4;
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn entry(skb: *mut __sk_buff) -> i32 {
    bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table), 0);
    /* Check multi-prog update. */
    bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table), 2);
    /* Check tail call limit. */
    bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table), 3);
    return 3;
}

#[no_mangle]
#[link_section = "license"]
pub static __license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
