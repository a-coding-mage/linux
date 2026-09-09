// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/em_nbyte.c N-Byte ematch
 *
 * Authors: Thomas Graf <tgraf@suug.ch>
 */

use core::ffi::{c_char, c_int, c_uchar, c_ulong, c_void};

// Types and functions supplied by the surrounding kernel headers.
#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcf_pkt_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcf_em_nbyte {
    pub layer: u16,
    pub off: u16,
    pub len: u16,
}

#[repr(C)]
pub struct tcf_ematch {
    pub datalen: usize,
    pub data: c_ulong,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct tcf_ematch_ops {
    pub kind: c_int,
    pub change: Option<unsafe extern "C" fn(*mut net, *mut c_void, c_int, *mut tcf_ematch) -> c_int>,
    pub match_: Option<unsafe extern "C" fn(*mut sk_buff, *mut tcf_ematch, *mut tcf_pkt_info) -> c_int>,
    pub owner: *mut c_void,
    pub link: list_head,
}

extern "C" {
    static THIS_MODULE: *mut c_void;
    static TCF_EM_NBYTE: c_int;

    fn kmemdup(src: *const c_void, len: usize, flags: c_int) -> *mut c_void;
    fn tcf_get_base_ptr(skb: *mut sk_buff, layer: u16) -> *mut c_uchar;
    fn tcf_valid_offset(skb: *mut sk_buff, ptr: *const c_uchar, len: u16) -> c_int;
    fn tcf_em_register(ops: *mut tcf_ematch_ops) -> c_int;
    fn tcf_em_unregister(ops: *mut tcf_ematch_ops);
}

const GFP_KERNEL: c_int = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

#[repr(C)]
struct nbyte_data {
    hdr: tcf_em_nbyte,
    pattern: [c_char; 0],
}

unsafe extern "C" fn em_nbyte_change(
    _net: *mut net,
    data: *mut c_void,
    data_len: c_int,
    em: *mut tcf_ematch,
) -> c_int {
    let nbyte = data as *mut tcf_em_nbyte;

    if (data_len as usize) < core::mem::size_of::<tcf_em_nbyte>()
        || (data_len as usize)
            < core::mem::size_of::<tcf_em_nbyte>() + (*nbyte).len as usize
    {
        return -EINVAL;
    }

    (*em).datalen = core::mem::size_of::<tcf_em_nbyte>() + (*nbyte).len as usize;
    (*em).data = kmemdup(data, (*em).datalen, GFP_KERNEL) as c_ulong;
    if (*em).data == 0 {
        return -ENOMEM;
    }

    0
}

unsafe extern "C" fn em_nbyte_match(
    skb: *mut sk_buff,
    em: *mut tcf_ematch,
    _info: *mut tcf_pkt_info,
) -> c_int {
    let nbyte = (*em).data as *mut nbyte_data;
    let mut ptr = tcf_get_base_ptr(skb, (*nbyte).hdr.layer);

    if ptr.is_null() {
        return 0;
    }
    ptr = ptr.add((*nbyte).hdr.off as usize);

    if tcf_valid_offset(skb, ptr, (*nbyte).hdr.len) == 0 {
        return 0;
    }

    let pattern = (nbyte as *mut u8).add(core::mem::size_of::<tcf_em_nbyte>());
    for i in 0..(*nbyte).hdr.len as usize {
        if *ptr.add(i) != *pattern.add(i) {
            return 0;
        }
    }
    1
}

static mut em_nbyte_ops: tcf_ematch_ops = tcf_ematch_ops {
    kind: 0,
    change: Some(em_nbyte_change),
    match_: Some(em_nbyte_match),
    owner: core::ptr::null_mut(),
    link: list_head {
        next: core::ptr::null_mut(),
        prev: core::ptr::null_mut(),
    },
};

unsafe extern "C" fn init_em_nbyte() -> c_int {
    em_nbyte_ops.kind = TCF_EM_NBYTE;
    em_nbyte_ops.owner = THIS_MODULE;
    em_nbyte_ops.link.next = &mut em_nbyte_ops.link;
    em_nbyte_ops.link.prev = &mut em_nbyte_ops.link;
    tcf_em_register(&mut em_nbyte_ops)
}

unsafe extern "C" fn exit_em_nbyte() {
    tcf_em_unregister(&mut em_nbyte_ops);
}

// MODULE_DESCRIPTION("ematch classifier for arbitrary skb multi-bytes");
// MODULE_LICENSE("GPL");
// module_init(init_em_nbyte);
// module_exit(exit_em_nbyte);
// MODULE_ALIAS_TCF_EMATCH(TCF_EM_NBYTE);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
