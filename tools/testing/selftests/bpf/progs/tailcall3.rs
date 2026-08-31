// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

// Map definition equivalent to:
// struct {
//     __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
//     __uint(max_entries, 2);
//     __uint(key_size, sizeof(__u32));
//     __uint(value_size, sizeof(__u32));
// } jmp_table SEC(".maps");
#[repr(C)]
pub struct jmp_table {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

pub const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static jmp_table: jmp_table = jmp_table {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 2,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u32>() as u32,
};

unsafe extern "C" {
    fn bpf_tail_call_static(ctx: *mut __sk_buff, map: *const jmp_table, index: u32);
}

#[unsafe(no_mangle)]
pub static mut count: i32 = 0;

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn classifier_0(skb: *mut __sk_buff) -> i32 {
    unsafe {
        count += 1;
        bpf_tail_call_static(skb, &jmp_table, 0);
    }
    1
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn entry(skb: *mut __sk_buff) -> i32 {
    unsafe {
        /* prog == NULL case */
        bpf_tail_call_static(skb, &jmp_table, 1);

        bpf_tail_call_static(skb, &jmp_table, 0);
    }
    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static __license: [u8; 4] = *b"GPL\0";
