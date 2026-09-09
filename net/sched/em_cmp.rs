// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/em_cmp.c\tSimple packet data comparison ematch
 *
 * Authors:\tThomas Graf <tgraf@suug.ch>
 */

// Dependencies supplied by the surrounding kernel translation unit.

use core::ffi::c_void;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcf_pkt_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcf_em_cmp {
    pub val: u32,
    pub mask: u32,
    pub off: u32,
    pub layer: u8,
    pub flags: u8,
    pub align: u8,
    pub opnd: u8,
}

#[repr(C)]
pub struct tcf_ematch {
    pub data: *mut c_void,
}

#[repr(C)]
pub struct tcf_ematch_ops {
    pub kind: u32,
    pub datalen: usize,
    pub match_: Option<unsafe extern "C" fn(*mut sk_buff, *mut tcf_ematch, *mut tcf_pkt_info) -> i32>,
    pub owner: *mut c_void,
    pub link: *mut c_void,
}

unsafe extern "C" {
    fn tcf_get_base_ptr(skb: *mut sk_buff, layer: u8) -> *mut u8;
    fn tcf_valid_offset(skb: *mut sk_buff, ptr: *mut u8, align: u8) -> bool;
    fn tcf_em_register(ops: *mut tcf_ematch_ops) -> i32;
    fn tcf_em_unregister(ops: *mut tcf_ematch_ops);
}

const TCF_EM_CMP_TRANS: u8 = 1;
const TCF_EM_ALIGN_U8: u8 = 1;
const TCF_EM_ALIGN_U16: u8 = 2;
const TCF_EM_ALIGN_U32: u8 = 4;
const TCF_EM_OPND_EQ: u8 = 1;
const TCF_EM_OPND_LT: u8 = 2;
const TCF_EM_OPND_GT: u8 = 3;
const TCF_EM_CMP: u32 = 1;

#[inline]
unsafe fn cmp_needs_transformation(cmp: *mut tcf_em_cmp) -> bool {
    ((*cmp).flags & TCF_EM_CMP_TRANS) != 0
}

#[inline]
unsafe fn get_unaligned_be16(ptr: *const u8) -> u32 {
    u16::from_be(core::ptr::read_unaligned(ptr as *const u16)) as u32
}

#[inline]
unsafe fn get_unaligned_be32(ptr: *const u8) -> u32 {
    u32::from_be(core::ptr::read_unaligned(ptr as *const u32))
}

unsafe fn em_cmp_match(
    skb: *mut sk_buff,
    em: *mut tcf_ematch,
    _info: *mut tcf_pkt_info,
) -> i32 {
    let cmp = (*em).data as *mut tcf_em_cmp;
    let mut ptr = tcf_get_base_ptr(skb, (*cmp).layer);
    let mut val: u32 = 0;

    if ptr.is_null() {
        return 0;
    }
    ptr = ptr.add((*cmp).off as usize);
    if !tcf_valid_offset(skb, ptr, (*cmp).align) {
        return 0;
    }

    match (*cmp).align {
        TCF_EM_ALIGN_U8 => {
            val = *ptr as u32;
        }
        TCF_EM_ALIGN_U16 => {
            val = get_unaligned_be16(ptr);
            if cmp_needs_transformation(cmp) {
                val = u16::from_be(val as u16) as u32;
            }
        }
        TCF_EM_ALIGN_U32 => {
            /* Worth checking boundaries? The branching seems
             * to get worse. Visit again.
             */
            val = get_unaligned_be32(ptr);
            if cmp_needs_transformation(cmp) {
                val = u32::from_be(val);
            }
        }
        _ => return 0,
    }

    if (*cmp).mask != 0 {
        val &= (*cmp).mask;
    }

    match (*cmp).opnd {
        TCF_EM_OPND_EQ => (val == (*cmp).val) as i32,
        TCF_EM_OPND_LT => (val < (*cmp).val) as i32,
        TCF_EM_OPND_GT => (val > (*cmp).val) as i32,
        _ => 0,
    }
}

static mut em_cmp_ops: tcf_ematch_ops = tcf_ematch_ops {
    kind: TCF_EM_CMP,
    datalen: core::mem::size_of::<tcf_em_cmp>(),
    match_: Some(em_cmp_match),
    owner: core::ptr::null_mut(),
    link: core::ptr::null_mut(),
};

unsafe fn init_em_cmp() -> i32 {
    tcf_em_register(&raw mut em_cmp_ops)
}

unsafe fn exit_em_cmp() {
    tcf_em_unregister(&raw mut em_cmp_ops);
}

// MODULE_DESCRIPTION("ematch classifier for basic data types(8/16/32 bit) against skb data");
// MODULE_LICENSE("GPL");
// module_init(init_em_cmp);
// module_exit(exit_em_cmp);
// MODULE_ALIAS_TCF_EMATCH(TCF_EM_CMP);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
