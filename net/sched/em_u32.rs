// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/em_u32.c\tU32 Ematch
 *
 * Authors:\tThomas Graf <tgraf@suug.ch>
 *\t\tAlexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 *
 * Based on net/sched/cls_u32.c
 */

use core::ffi::c_void;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tc_u32_key {
    pub mask: u32,
    pub val: u32,
    pub off: i32,
    pub offmask: i32,
}

#[repr(C)]
pub struct tcf_ematch {
    pub data: *mut c_void,
}

#[repr(C)]
pub struct tcf_pkt_info {
    pub ptr: *const u8,
    pub nexthdr: u32,
}

#[repr(C)]
pub struct tcf_ematch_ops {
    pub kind: u32,
    pub datalen: usize,
    pub match_: Option<unsafe extern "C" fn(*mut sk_buff, *mut tcf_ematch, *mut tcf_pkt_info) -> i32>,
    pub owner: *mut c_void,
    pub link: list_head,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

extern "C" {
    fn skb_network_header(skb: *mut sk_buff) -> *const u8;
    fn tcf_valid_offset(skb: *mut sk_buff, ptr: *const u8, len: usize) -> bool;
    fn tcf_em_register(ops: *mut tcf_ematch_ops) -> i32;
    fn tcf_em_unregister(ops: *mut tcf_ematch_ops);
    static THIS_MODULE: *mut c_void;
}

pub const TCF_EM_U32: u32 = 0;

unsafe extern "C" fn em_u32_match(
    skb: *mut sk_buff,
    em: *mut tcf_ematch,
    info: *mut tcf_pkt_info,
) -> i32 {
    let key = (*em).data as *const tc_u32_key;
    let mut ptr = skb_network_header(skb);

    if !info.is_null() {
        if !(*info).ptr.is_null() {
            ptr = (*info).ptr;
        }
        ptr = ptr.add(((*info).nexthdr & (*key).offmask) as usize);
    }

    ptr = ptr.offset((*key).off as isize);

    if !tcf_valid_offset(skb, ptr, core::mem::size_of::<u32>()) {
        return 0;
    }

    let value = u32::from_be(core::ptr::read_unaligned(ptr as *const u32));
    if ((value ^ (*key).val) & (*key).mask) == 0 { 1 } else { 0 }
}

static mut em_u32_ops: tcf_ematch_ops = tcf_ematch_ops {
    kind: TCF_EM_U32,
    datalen: core::mem::size_of::<tc_u32_key>(),
    match_: Some(em_u32_match),
    owner: core::ptr::null_mut(),
    link: list_head {
        next: core::ptr::null_mut(),
        prev: core::ptr::null_mut(),
    },
};

unsafe extern "C" fn init_em_u32() -> i32 {
    tcf_em_register(&mut em_u32_ops)
}

unsafe extern "C" fn exit_em_u32() {
    tcf_em_unregister(&mut em_u32_ops);
}

// MODULE_DESCRIPTION("ematch skb classifier using 32 bit chunks of data");
// MODULE_LICENSE("GPL");
// module_init(init_em_u32);
// module_exit(exit_em_u32);
// MODULE_ALIAS_TCF_EMATCH(TCF_EM_U32);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
