// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor security identifier (secid) manipulation fns
 *
 * Copyright 2009-2017 Canonical Ltd.
 *
 * AppArmor allocates a unique secid for every label used. If a label
 * is replaced it receives the secid of the label it is replacing.
 */

// linux/errno.h
// linux/err.h
// linux/gfp.h
// linux/slab.h
// linux/spinlock.h
// linux/xarray.h

// include/cred.h
// include/lib.h
// include/secid.h
// include/label.h
// include/policy_ns.h

/*
 * secids - do not pin labels with a refcount. They rely on the label
 * properly updating/freeing them
 */
const AA_FIRST_SECID: u32 = 2;

// secids - do not pin labels with a refcount. They rely on the label
// properly updating/freeing them
// static DEFINE_XARRAY_FLAGS(aa_secids, XA_FLAGS_LOCK_IRQ | XA_FLAGS_TRACK_FREE);
static mut aa_secids: xa_struct = unsafe { core::mem::zeroed() };

pub static mut apparmor_display_secid_mode: i32 = 0;

/*
 * TODO: allow policy to reserve a secid range?
 * TODO: add secid pinning
 * TODO: use secid_update in label replace
 */

/*
 * see label for inverse aa_label_to_secid
 */
pub unsafe extern "C" fn aa_secid_to_label(secid: u32) -> *mut aa_label {
    xa_load(&aa_secids, secid as usize) as *mut aa_label
}

unsafe extern "C" fn apparmor_label_to_secctx(
    label: *mut aa_label,
    cp: *mut lsm_context,
) -> i32 {
    /* TODO: cache secctx and ref count so we don't have to recreate */
    let mut flags: i32 = FLAG_VIEW_SUBNS | FLAG_HIDDEN_UNCONFINED | FLAG_ABS_ROOT;
    let len: i32;

    if label.is_null() {
        return -22; // -EINVAL
    }

    if apparmor_display_secid_mode != 0 {
        flags |= FLAG_SHOW_MODE;
    }

    if !cp.is_null() {
        len = aa_label_asxprint(&mut (*cp).context, root_ns, label, flags, 0);
    } else {
        len = aa_label_snxprint(core::ptr::null_mut(), 0, root_ns, label, flags);
    }

    if len < 0 {
        return -12; // -ENOMEM
    }

    if !cp.is_null() {
        (*cp).len = len;
        (*cp).id = 1; // LSM_ID_APPARMOR
    }

    len
}

pub unsafe extern "C" fn apparmor_secid_to_secctx(secid: u32, cp: *mut lsm_context) -> i32 {
    let label = aa_secid_to_label(secid);
    apparmor_label_to_secctx(label, cp)
}

pub unsafe extern "C" fn apparmor_lsmprop_to_secctx(prop: *mut lsm_prop, cp: *mut lsm_context) -> i32 {
    let label: *mut aa_label = (*prop).apparmor.label;
    apparmor_label_to_secctx(label, cp)
}

pub unsafe extern "C" fn apparmor_secctx_to_secid(secdata: *const u8, seclen: u32, secid: *mut u32) -> i32 {
    let label = aa_label_strn_parse(&mut (*(*root_ns).unconfined).label, secdata, seclen, 0, 0, 0);
    if IS_ERR(label as *const ()) {
        return PTR_ERR(label as *const ());
    }
    *secid = (*label).secid;
    0
}

pub unsafe extern "C" fn apparmor_release_secctx(cp: *mut lsm_context) {
    if (*cp).id == 1 {
        kfree((*cp).context);
        (*cp).context = core::ptr::null_mut();
        (*cp).id = 0;
    }
}

/**
 * aa_alloc_secid - allocate a new secid for a profile
 * @label: the label to allocate a secid for
 * @gfp: memory allocation flags
 *
 * Returns: 0 with @label->secid initialized
 *          <0 returns error with @label->secid set to AA_SECID_INVALID
 */
pub unsafe extern "C" fn aa_alloc_secid(label: *mut aa_label, gfp: u32) -> i32 {
    let mut flags: u64 = 0;
    let ret = {
        xa_lock_irqsave(&mut aa_secids, &mut flags);
        let ret = __xa_alloc(&mut aa_secids, &mut (*label).secid, label as *const (), (AA_FIRST_SECID, i32::MAX as u32), gfp);
        xa_unlock_irqrestore(&mut aa_secids, flags);
        ret
    };
    if ret < 0 {
        (*label).secid = 0xffffffff;
        return ret;
    }
    0
}

/**
 * aa_free_secid - free a secid
 * @secid: secid to free
 */
pub unsafe extern "C" fn aa_free_secid(secid: u32) {
    let mut flags: u64 = 0;
    xa_lock_irqsave(&mut aa_secids, &mut flags);
    __xa_erase(&mut aa_secids, secid as usize);
    xa_unlock_irqrestore(&mut aa_secids, flags);
}

extern "C" {
    pub struct aa_label { pub secid: u32 }
    pub struct lsm_context { pub context: *mut u8, pub len: i32, pub id: i32 }
    pub struct lsm_prop { pub apparmor: AppArmorProp }
    pub struct AppArmorProp { pub label: *mut aa_label }
    pub struct xa_struct {}
    pub static mut root_ns: *mut ();
    pub fn xa_load(xa: *const xa_struct, index: usize) -> *const ();
    pub fn xa_lock_irqsave(xa: *mut xa_struct, flags: *mut u64);
    pub fn xa_unlock_irqrestore(xa: *mut xa_struct, flags: u64);
    pub fn __xa_alloc(xa: *mut xa_struct, index: *mut u32, entry: *const (), limit: (u32, u32), gfp: u32) -> i32;
    pub fn __xa_erase(xa: *mut xa_struct, index: usize);
    pub fn aa_label_asxprint(context: *mut *mut u8, ns: *mut (), label: *mut aa_label, flags: i32, gfp: u32) -> i32;
    pub fn aa_label_snxprint(buf: *mut u8, size: usize, ns: *mut (), label: *mut aa_label, flags: i32) -> i32;
    pub fn aa_label_strn_parse(base: *mut aa_label, str_: *const u8, len: u32, gfp: u32, create: i32, force_clear: i32) -> *mut aa_label;
    pub fn kfree(ptr: *const ());
    pub fn IS_ERR(ptr: *const ()) -> i32;
    pub fn PTR_ERR(ptr: *const ()) -> i32;
}

const FLAG_VIEW_SUBNS: i32 = 1;
const FLAG_HIDDEN_UNCONFINED: i32 = 2;
const FLAG_ABS_ROOT: i32 = 4;
const FLAG_SHOW_MODE: i32 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
