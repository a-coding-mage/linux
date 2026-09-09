// SPDX-License-Identifier: GPL-2.0-only
/* String matching match for iptables
 *
 * (C) 2005 Pablo Neira Ayuso <pablo@eurodev.net>
 */

// Kernel headers and module infrastructure are supplied by external dependencies.

const XT_STRING_FLAG_IGNORECASE: u32 = 1 << 0;
const XT_STRING_FLAG_INVERT: u32 = 1 << 1;
const XT_STRING_MAX_ALGO_NAME_SIZE: usize = 16;
const XT_STRING_MAX_PATTERN_SIZE: usize = 128;
const NFPROTO_UNSPEC: u16 = 0;
const TS_AUTOLOAD: i32 = 1 << 0;
const TS_IGNORECASE: i32 = 1 << 1;
const GFP_KERNEL: u32 = 0;
const UINT_MAX: u32 = u32::MAX;
const EINVAL: i32 = 22;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ts_config {
    _private: [u8; 0],
}

#[repr(C)]
pub union xt_string_info_u {
    pub v1: xt_string_info_v1,
}

#[repr(C)]
pub struct xt_string_info_v1 {
    pub flags: u32,
}

#[repr(C)]
pub struct xt_string_info {
    pub from_offset: u16,
    pub to_offset: u16,
    pub algo: [u8; XT_STRING_MAX_ALGO_NAME_SIZE],
    pub pattern: [u8; XT_STRING_MAX_PATTERN_SIZE],
    pub patlen: u8,
    pub u: xt_string_info_u,
    pub config: *mut ts_config,
}

#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *const core::ffi::c_void,
}

#[repr(C)]
pub struct xt_mtchk_param {
    pub matchinfo: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct xt_mtdtor_param {
    pub matchinfo: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct xt_match {
    pub name: *const u8,
    pub revision: u8,
    pub family: u16,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    pub match_: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(*const xt_mtdtor_param)>,
    pub matchsize: usize,
    pub usersize: usize,
    pub me: *mut core::ffi::c_void,
}

extern "C" {
    fn skb_find_text(
        skb: *mut sk_buff,
        from: u16,
        to: u16,
        config: *mut ts_config,
    ) -> u32;
    fn textsearch_prepare(
        algo: *const u8,
        pattern: *const u8,
        len: u8,
        gfp_mask: u32,
        flags: i32,
    ) -> *mut ts_config;
    fn textsearch_destroy(config: *mut ts_config);
    fn xt_register_match(m: *mut xt_match) -> i32;
    fn xt_unregister_match(m: *mut xt_match);
    fn ptr_err<T>(ptr: *mut T) -> i32;
}

unsafe fn string_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let conf = (*par).matchinfo as *const xt_string_info;
    let invert = ((*conf).u.v1.flags & XT_STRING_FLAG_INVERT) != 0;

    (skb_find_text(
        skb as *mut sk_buff,
        (*conf).from_offset,
        (*conf).to_offset,
        (*conf).config,
    ) != UINT_MAX) ^ invert
}

unsafe fn string_mt_check(par: *const xt_mtchk_param) -> i32 {
    let conf = (*par).matchinfo as *mut xt_string_info;
    let mut flags = TS_AUTOLOAD;

    /* Damn, can't handle this case properly with iptables... */
    if (*conf).from_offset > (*conf).to_offset {
        return -EINVAL;
    }
    if (*conf).algo[XT_STRING_MAX_ALGO_NAME_SIZE - 1] != 0 {
        return -EINVAL;
    }
    if (*conf).patlen as usize > XT_STRING_MAX_PATTERN_SIZE {
        return -EINVAL;
    }
    if ((*conf).u.v1.flags & !(XT_STRING_FLAG_IGNORECASE | XT_STRING_FLAG_INVERT)) != 0 {
        return -EINVAL;
    }
    if ((*conf).u.v1.flags & XT_STRING_FLAG_IGNORECASE) != 0 {
        flags |= TS_IGNORECASE;
    }
    let ts_conf = textsearch_prepare(
        (*conf).algo.as_ptr(),
        (*conf).pattern.as_ptr(),
        (*conf).patlen,
        GFP_KERNEL,
        flags,
    );
    if (ts_conf.is_null()) {
        return ptr_err(ts_conf);
    }

    (*conf).config = ts_conf;
    0
}

unsafe fn string_mt_destroy(par: *const xt_mtdtor_param) {
    let conf = (*par).matchinfo as *mut xt_string_info;
    textsearch_destroy((*conf).config);
}

static mut xt_string_mt_reg: xt_match = xt_match {
    name: b"string\0".as_ptr(),
    revision: 1,
    family: NFPROTO_UNSPEC,
    checkentry: Some(string_mt_check),
    match_: Some(string_mt),
    destroy: Some(string_mt_destroy),
    matchsize: core::mem::size_of::<xt_string_info>(),
    usersize: core::mem::offset_of!(xt_string_info, config),
    me: core::ptr::null_mut(),
};

unsafe extern "C" fn string_mt_init() -> i32 {
    xt_register_match(&raw mut xt_string_mt_reg)
}

unsafe extern "C" fn string_mt_exit() {
    xt_unregister_match(&raw mut xt_string_mt_reg);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
