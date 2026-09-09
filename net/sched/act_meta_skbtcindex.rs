// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/act_meta_tc_index.c IFE skb->tc_index metadata module
 *
 * copyright Jamal Hadi Salim (2016)
 */

// C dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    fn ife_encode_meta_u16(
        value: u32,
        skbdata: *mut core::ffi::c_void,
        e: *mut tcf_meta_info,
    ) -> core::ffi::c_int;
    fn ife_check_meta_u16(value: u16, e: *mut tcf_meta_info) -> core::ffi::c_int;
    fn ntohs(value: u16) -> u16;
    fn ife_get_meta_u16() -> *mut core::ffi::c_void;
    fn ife_alloc_meta_u16() -> *mut core::ffi::c_void;
    fn ife_release_meta_gen() -> *mut core::ffi::c_void;
    fn ife_validate_meta_u16() -> *mut core::ffi::c_void;
    fn register_ife_op(ops: *mut tcf_meta_ops) -> core::ffi::c_int;
    fn unregister_ife_op(ops: *mut tcf_meta_ops);
}

#[repr(C)]
pub struct sk_buff {
    pub tc_index: u16,
}

#[repr(C)]
pub struct tcf_meta_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcf_meta_ops {
    pub metaid: u32,
    pub metatype: u32,
    pub name: *const core::ffi::c_char,
    pub synopsis: *const core::ffi::c_char,
    pub check_presence:
        Option<unsafe extern "C" fn(*mut sk_buff, *mut tcf_meta_info) -> core::ffi::c_int>,
    pub encode: Option<unsafe extern "C" fn(
        *mut sk_buff,
        *mut core::ffi::c_void,
        *mut tcf_meta_info,
    ) -> core::ffi::c_int>,
    pub decode: Option<unsafe extern "C" fn(
        *mut sk_buff,
        *mut core::ffi::c_void,
        u16,
    ) -> core::ffi::c_int>,
    pub get: *const core::ffi::c_void,
    pub alloc: *const core::ffi::c_void,
    pub release: *const core::ffi::c_void,
    pub validate: *const core::ffi::c_void,
    pub owner: *const core::ffi::c_void,
}

const IFE_META_TCINDEX: u32 = 0;
const NLA_U16: u32 = 0;
const THIS_MODULE: *const core::ffi::c_void = core::ptr::null();

unsafe extern "C" fn skbtcindex_encode(
    skb: *mut sk_buff,
    skbdata: *mut core::ffi::c_void,
    e: *mut tcf_meta_info,
) -> core::ffi::c_int {
    let ifetc_index: u32 = (*skb).tc_index as u32;

    ife_encode_meta_u16(ifetc_index, skbdata, e)
}

unsafe extern "C" fn skbtcindex_decode(
    skb: *mut sk_buff,
    data: *mut core::ffi::c_void,
    _len: u16,
) -> core::ffi::c_int {
    let ifetc_index: u16 = *(data as *const u16);

    (*skb).tc_index = ntohs(ifetc_index);
    0
}

unsafe extern "C" fn skbtcindex_check(
    skb: *mut sk_buff,
    e: *mut tcf_meta_info,
) -> core::ffi::c_int {
    ife_check_meta_u16((*skb).tc_index, e)
}

static mut ife_skbtcindex_ops: tcf_meta_ops = tcf_meta_ops {
    metaid: IFE_META_TCINDEX,
    metatype: NLA_U16,
    name: b"tc_index\0".as_ptr() as *const core::ffi::c_char,
    synopsis: b"skb tc_index 16 bit metadata\0".as_ptr() as *const core::ffi::c_char,
    check_presence: Some(skbtcindex_check),
    encode: Some(skbtcindex_encode),
    decode: Some(skbtcindex_decode),
    get: ife_get_meta_u16 as *const core::ffi::c_void,
    alloc: ife_alloc_meta_u16 as *const core::ffi::c_void,
    release: ife_release_meta_gen as *const core::ffi::c_void,
    validate: ife_validate_meta_u16 as *const core::ffi::c_void,
    owner: THIS_MODULE,
};

unsafe extern "C" fn ifetc_index_init_module() -> core::ffi::c_int {
    register_ife_op(&raw mut ife_skbtcindex_ops)
}

unsafe extern "C" fn ifetc_index_cleanup_module() {
    unregister_ife_op(&raw mut ife_skbtcindex_ops);
}

// module_init(ifetc_index_init_module);
// module_exit(ifetc_index_cleanup_module);
// MODULE_AUTHOR("Jamal Hadi Salim(2016)");
// MODULE_DESCRIPTION("Inter-FE skb tc_index metadata module");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_IFE_META("tcindex");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
