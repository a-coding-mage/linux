// SPDX-License-Identifier: GPL-2.0-only
/*
 * netfilter module to enforce network quotas
 *
 * Sam Johnston <samj@samj.net>
 */

use core::ffi::c_void;

// Linux kernel dependencies supplied by the surrounding translation.
extern "C" {
    fn spin_lock_bh(lock: *mut spinlock_t);
    fn spin_unlock_bh(lock: *mut spinlock_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn kfree(ptr: *mut c_void);
    fn xt_register_match(m: *mut xt_match) -> i32;
    fn xt_unregister_match(m: *mut xt_match);
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    pub len: u32,
}

#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *mut c_void,
}

#[repr(C)]
pub struct xt_mtchk_param {
    pub matchinfo: *mut c_void,
}

#[repr(C)]
pub struct xt_mtdtor_param {
    pub matchinfo: *const c_void,
}

#[repr(C)]
pub struct xt_quota_info {
    pub flags: u8,
    pub quota: u64,
    pub master: *mut xt_quota_priv,
}

#[repr(C)]
pub struct xt_quota_priv {
    pub lock: spinlock_t,
    pub quota: u64,
}

#[repr(C)]
pub struct xt_match {
    pub name: *const u8,
    pub revision: u8,
    pub family: u16,
    pub match_fn: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    pub destroy: Option<unsafe extern "C" fn(*const xt_mtdtor_param)>,
    pub matchsize: usize,
    pub usersize: usize,
    pub me: *mut c_void,
}

pub const XT_QUOTA_INVERT: u8 = 1;
pub const XT_QUOTA_MASK: u8 = XT_QUOTA_INVERT;
pub const NFPROTO_UNSPEC: u16 = 0;

unsafe extern "C" fn quota_mt(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let q = (*par).matchinfo as *mut xt_quota_info;
    let priv_ = (*q).master;
    let mut ret = ((*q).flags & XT_QUOTA_INVERT) != 0;

    spin_lock_bh(&mut (*priv_).lock);
    if (*priv_).quota >= (*skb).len as u64 {
        (*priv_).quota = (*priv_).quota.wrapping_sub((*skb).len as u64);
        ret = !ret;
    } else {
        /* we do not allow even small packets from now on */
        (*priv_).quota = 0;
    }
    spin_unlock_bh(&mut (*priv_).lock);

    ret
}

unsafe extern "C" fn quota_mt_check(par: *const xt_mtchk_param) -> i32 {
    let q = (*par).matchinfo as *mut xt_quota_info;

    if ((*q).flags & !XT_QUOTA_MASK) != 0 {
        return -22;
    }

    // Equivalent of kmalloc_obj(*q->master), supplied by the kernel environment.
    (*q).master = kmalloc_obj::<xt_quota_priv>();
    if (*q).master.is_null() {
        return -12;
    }

    spin_lock_init(&mut (*(*q).master).lock);
    (*(*q).master).quota = (*q).quota;
    0
}

unsafe extern "C" fn quota_mt_destroy(par: *const xt_mtdtor_param) {
    let q = (*par).matchinfo as *const xt_quota_info;
    kfree((*q).master as *mut c_void);
}

static mut quota_mt_reg: xt_match = xt_match {
    name: b"quota\0".as_ptr(),
    revision: 0,
    family: NFPROTO_UNSPEC,
    match_fn: Some(quota_mt),
    checkentry: Some(quota_mt_check),
    destroy: Some(quota_mt_destroy),
    matchsize: core::mem::size_of::<xt_quota_info>(),
    usersize: core::mem::offset_of!(xt_quota_info, master),
    me: core::ptr::null_mut(),
};

unsafe extern "C" fn quota_mt_init() -> i32 {
    xt_register_match(&mut quota_mt_reg)
}

unsafe extern "C" fn quota_mt_exit() {
    xt_unregister_match(&mut quota_mt_reg);
}

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Sam Johnston <samj@samj.net>");
// MODULE_DESCRIPTION("Xtables: countdown quota match");
// MODULE_ALIAS("ipt_quota");
// MODULE_ALIAS("ip6t_quota");
// module_init(quota_mt_init);
// module_exit(quota_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
