// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/em_text.c  Textsearch ematch
 *
 * Authors: Thomas Graf <tgraf@suug.ch>
 */

// Linux kernel dependencies are supplied by the surrounding translation.

use core::ffi::c_void;

type U8 = u8;
type U16 = u16;
type CInt = i32;
type CLong = isize;
type ULong = usize;

#[repr(C)]
pub struct sk_buff {
    pub data: *mut U8,
}

#[repr(C)]
pub struct ts_config {
    pub ops: *mut ts_ops,
}

#[repr(C)]
pub struct ts_ops {
    pub name: *const u8,
}

#[repr(C)]
pub struct tcf_ematch {
    pub data: ULong,
    pub datalen: U16,
}

#[repr(C)]
pub struct tcf_pkt_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcf_em_text {
    pub algo: [u8; 32],
    pub from_offset: U16,
    pub to_offset: U16,
    pub from_layer: U8,
    pub to_layer: U8,
    pub pattern_len: U16,
    pub pad: U16,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct tcf_ematch_ops {
    pub kind: CInt,
    pub change: Option<unsafe extern "C" fn(*mut net, *mut c_void, CInt, *mut tcf_ematch) -> CInt>,
    pub match_: Option<unsafe extern "C" fn(*mut sk_buff, *mut tcf_ematch, *mut tcf_pkt_info) -> CInt>,
    pub destroy: Option<unsafe extern "C" fn(*mut tcf_ematch)>,
    pub dump: Option<unsafe extern "C" fn(*mut sk_buff, *mut tcf_ematch) -> CInt>,
    pub owner: *mut c_void,
    pub link: list_head,
}

#[repr(C)]
struct text_match {
    from_offset: U16,
    to_offset: U16,
    from_layer: U8,
    to_layer: U8,
    config: *mut ts_config,
}

// #define EM_TEXT_PRIV(m) ((struct text_match *) (m)->data)
#[inline]
unsafe fn em_text_priv(m: *mut tcf_ematch) -> *mut text_match {
    (*m).data as *mut text_match
}

extern "C" {
    fn tcf_get_base_ptr(skb: *mut sk_buff, layer: U8) -> *mut U8;
    fn skb_find_text(skb: *mut sk_buff, from: CInt, to: CInt, config: *mut ts_config) -> ULong;
    fn textsearch_prepare(algo: *const u8, pattern: *const U8, len: U16, gfp: CInt, flags: CInt) -> *mut ts_config;
    fn rtnl_lock();
    fn rtnl_unlock();
    fn textsearch_destroy(config: *mut ts_config);
    fn kmalloc(size: usize, flags: CInt) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn strscpy(dst: *mut u8, src: *const u8);
    fn nla_put_nohdr(skb: *mut sk_buff, len: usize, data: *const c_void) -> CInt;
    fn nla_append(skb: *mut sk_buff, len: U16, data: *const u8) -> CInt;
    fn textsearch_get_pattern_len(config: *mut ts_config) -> U16;
    fn textsearch_get_pattern(config: *mut ts_config) -> *const u8;
    fn tcf_em_register(ops: *mut tcf_ematch_ops) -> CInt;
    fn tcf_em_unregister(ops: *mut tcf_ematch_ops);
}

const GFP_KERNEL: CInt = 0;
const TS_AUTOLOAD: CInt = 1;
const ENOENT: CInt = 2;
const ENOBUFS: CInt = 105;
const EINVAL: CInt = 22;
const EAGAIN: CInt = 11;
const UINT_MAX: ULong = ULong::MAX;

unsafe extern "C" fn em_text_match(
    skb: *mut sk_buff,
    m: *mut tcf_ematch,
    _info: *mut tcf_pkt_info,
) -> CInt {
    let tm = em_text_priv(m);
    let mut ptr = tcf_get_base_ptr(skb, (*tm).from_layer);
    if ptr.is_null() {
        return 0;
    }
    let mut from = ptr.offset_from((*skb).data) as CInt;
    from += (*tm).from_offset as CInt;

    ptr = tcf_get_base_ptr(skb, (*tm).to_layer);
    if ptr.is_null() {
        return 0;
    }
    let mut to = ptr.offset_from((*skb).data) as CInt;
    to += (*tm).to_offset as CInt;

    (skb_find_text(skb, from, to, (*tm).config) != UINT_MAX) as CInt
}

unsafe extern "C" fn em_text_change(
    _net: *mut net,
    data: *mut c_void,
    len: CInt,
    m: *mut tcf_ematch,
) -> CInt {
    let conf = data as *mut tcf_em_text;
    let mut flags: CInt = 0;

    if len < core::mem::size_of::<tcf_em_text>() as CInt
        || len < (core::mem::size_of::<tcf_em_text>() + (*conf).pattern_len as usize) as CInt
    {
        return -EINVAL;
    }
    if (*conf).from_layer > (*conf).to_layer {
        return -EINVAL;
    }
    if (*conf).from_layer == (*conf).to_layer && (*conf).from_offset > (*conf).to_offset {
        return -EINVAL;
    }

    loop {
        let ts_conf = textsearch_prepare(
            (*conf).algo.as_ptr(),
            (data as *mut U8).add(core::mem::size_of::<tcf_em_text>()),
            (*conf).pattern_len,
            GFP_KERNEL,
            flags,
        );
        if flags & TS_AUTOLOAD != 0 {
            rtnl_lock();
        }
        if ts_conf as isize == -ENOENT as isize && flags & TS_AUTOLOAD == 0 {
            rtnl_unlock();
            flags |= TS_AUTOLOAD;
            continue;
        }
        if ts_conf as isize < 0 {
            return ts_conf as CInt;
        } else if flags & TS_AUTOLOAD != 0 {
            textsearch_destroy(ts_conf);
            return -EAGAIN;
        }

        let tm = kmalloc(core::mem::size_of::<text_match>(), GFP_KERNEL) as *mut text_match;
        if tm.is_null() {
            textsearch_destroy(ts_conf);
            return -ENOBUFS;
        }
        (*tm).from_offset = (*conf).from_offset;
        (*tm).to_offset = (*conf).to_offset;
        (*tm).from_layer = (*conf).from_layer;
        (*tm).to_layer = (*conf).to_layer;
        (*tm).config = ts_conf;
        (*m).datalen = core::mem::size_of::<text_match>() as U16;
        (*m).data = tm as ULong;
        return 0;
    }
}

unsafe extern "C" fn em_text_destroy(m: *mut tcf_ematch) {
    let tm = em_text_priv(m);
    if !tm.is_null() && !(*tm).config.is_null() {
        textsearch_destroy((*tm).config);
        kfree(tm as *mut c_void);
    }
}

unsafe extern "C" fn em_text_dump(skb: *mut sk_buff, m: *mut tcf_ematch) -> CInt {
    let tm = em_text_priv(m);
    let mut conf: tcf_em_text = core::mem::zeroed();
    strscpy(conf.algo.as_mut_ptr(), (*(*tm).config).ops.as_ref().unwrap().name);
    conf.from_offset = (*tm).from_offset;
    conf.to_offset = (*tm).to_offset;
    conf.from_layer = (*tm).from_layer;
    conf.to_layer = (*tm).to_layer;
    conf.pattern_len = textsearch_get_pattern_len((*tm).config);
    conf.pad = 0;
    if nla_put_nohdr(skb, core::mem::size_of::<tcf_em_text>(), &conf as *const _ as *const c_void) < 0 {
        return -1;
    }
    if nla_append(skb, conf.pattern_len, textsearch_get_pattern((*tm).config)) < 0 {
        return -1;
    }
    0
}

static mut em_text_ops: tcf_ematch_ops = tcf_ematch_ops {
    kind: 0,
    change: Some(em_text_change),
    match_: Some(em_text_match),
    destroy: Some(em_text_destroy),
    dump: Some(em_text_dump),
    owner: core::ptr::null_mut(),
    link: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() },
};

unsafe extern "C" fn init_em_text() -> CInt {
    tcf_em_register(&mut em_text_ops)
}

unsafe extern "C" fn exit_em_text() {
    tcf_em_unregister(&mut em_text_ops);
}

// MODULE_DESCRIPTION("ematch classifier for embedded text in skbs");
// MODULE_LICENSE("GPL");
// module_init(init_em_text);
// module_exit(exit_em_text);
// MODULE_ALIAS_TCF_EMATCH(TCF_EM_TEXT);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
