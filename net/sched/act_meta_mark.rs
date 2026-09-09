// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/act_meta_mark.c IFE skb->mark metadata module
 *
 * copyright Jamal Hadi Salim (2015)
 */

// Kernel and IFE declarations are supplied by the surrounding translation unit.

unsafe extern "C" {
    fn ife_encode_meta_u32(
        value: u32,
        skbdata: *mut core::ffi::c_void,
        e: *mut tcf_meta_info,
    ) -> i32;
    fn ntohl(value: u32) -> u32;
    fn ife_check_meta_u32(value: u32, e: *mut tcf_meta_info) -> i32;
    fn ife_get_meta_u32();
    fn ife_alloc_meta_u32();
    fn ife_release_meta_gen();
    fn ife_validate_meta_u32();
    fn register_ife_op(ops: *mut tcf_meta_ops) -> i32;
    fn unregister_ife_op(ops: *mut tcf_meta_ops);
}

#[repr(C)]
pub struct sk_buff {
    pub mark: u32,
}

#[repr(C)]
pub struct tcf_meta_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcf_meta_ops {
    pub metaid: u16,
    pub metatype: u16,
    pub name: *const core::ffi::c_char,
    pub synopsis: *const core::ffi::c_char,
    pub check_presence: unsafe extern "C" fn(*mut sk_buff, *mut tcf_meta_info) -> i32,
    pub encode: unsafe extern "C" fn(
        *mut sk_buff,
        *mut core::ffi::c_void,
        *mut tcf_meta_info,
    ) -> i32,
    pub decode: unsafe extern "C" fn(*mut sk_buff, *mut core::ffi::c_void, u16) -> i32,
    pub get: unsafe extern "C" fn(),
    pub alloc: unsafe extern "C" fn(),
    pub release: unsafe extern "C" fn(),
    pub validate: unsafe extern "C" fn(),
    pub owner: *mut core::ffi::c_void,
}

const IFE_META_SKBMARK: u16 = 0;
const NLA_U32: u16 = 10;

unsafe extern "C" fn skbmark_encode(
    skb: *mut sk_buff,
    skbdata: *mut core::ffi::c_void,
    e: *mut tcf_meta_info,
) -> i32 {
    let ifemark: u32 = (*skb).mark;

    ife_encode_meta_u32(ifemark, skbdata, e)
}

unsafe extern "C" fn skbmark_decode(
    skb: *mut sk_buff,
    data: *mut core::ffi::c_void,
    _len: u16,
) -> i32 {
    let ifemark: u32 = *(data as *const u32);

    (*skb).mark = ntohl(ifemark);
    0
}

unsafe extern "C" fn skbmark_check(skb: *mut sk_buff, e: *mut tcf_meta_info) -> i32 {
    ife_check_meta_u32((*skb).mark, e)
}

static mut ife_skbmark_ops: tcf_meta_ops = tcf_meta_ops {
    metaid: IFE_META_SKBMARK,
    metatype: NLA_U32,
    name: b"skbmark\0".as_ptr() as *const core::ffi::c_char,
    synopsis: b"skb mark 32 bit metadata\0".as_ptr() as *const core::ffi::c_char,
    check_presence: skbmark_check,
    encode: skbmark_encode,
    decode: skbmark_decode,
    get: ife_get_meta_u32,
    alloc: ife_alloc_meta_u32,
    release: ife_release_meta_gen,
    validate: ife_validate_meta_u32,
    owner: core::ptr::null_mut(),
};

unsafe extern "C" fn ifemark_init_module() -> i32 {
    register_ife_op(&raw mut ife_skbmark_ops)
}

unsafe extern "C" fn ifemark_cleanup_module() {
    unregister_ife_op(&raw mut ife_skbmark_ops);
}

// module_init(ifemark_init_module);
// module_exit(ifemark_cleanup_module);
// MODULE_AUTHOR("Jamal Hadi Salim(2015)");
// MODULE_DESCRIPTION("Inter-FE skb mark metadata module");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_IFE_META("skbmark");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
