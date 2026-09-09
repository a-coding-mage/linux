// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2004, 2005 Oracle.  All rights reserved.
 */

// Linux kernel dependencies and "masklog.h" are supplied externally.

pub static mut mlog_and_bits: mlog_bits = MLOG_BITS_RHS(MLOG_INITIAL_AND_MASK);
// EXPORT_SYMBOL_GPL(mlog_and_bits);
pub static mut mlog_not_bits: mlog_bits = MLOG_BITS_RHS(0);
// EXPORT_SYMBOL_GPL(mlog_not_bits);

unsafe fn mlog_mask_show(mask: u64, buf: *mut c_char) -> ssize_t {
    let state: *const c_char;

    if __mlog_test_u64(mask, mlog_and_bits) {
        state = b"allow\0".as_ptr() as *const c_char;
    } else if __mlog_test_u64(mask, mlog_not_bits) {
        state = b"deny\0".as_ptr() as *const c_char;
    } else {
        state = b"off\0".as_ptr() as *const c_char;
    }

    snprintf(buf, PAGE_SIZE, b"%s\n\0".as_ptr() as *const c_char, state)
}

unsafe fn mlog_mask_store(mask: u64, buf: *const c_char, count: size_t) -> ssize_t {
    if !strncasecmp(buf, b"allow\0".as_ptr() as *const c_char, 5) {
        __mlog_set_u64(mask, mlog_and_bits);
        __mlog_clear_u64(mask, mlog_not_bits);
    } else if !strncasecmp(buf, b"deny\0".as_ptr() as *const c_char, 4) {
        __mlog_set_u64(mask, mlog_not_bits);
        __mlog_clear_u64(mask, mlog_and_bits);
    } else if !strncasecmp(buf, b"off\0".as_ptr() as *const c_char, 3) {
        __mlog_clear_u64(mask, mlog_not_bits);
        __mlog_clear_u64(mask, mlog_and_bits);
    } else {
        return -EINVAL;
    }

    count as ssize_t
}

pub unsafe fn __mlog_printk(
    mask: *const u64,
    func: *const c_char,
    line: c_int,
    fmt: *const c_char,
    mut _args: ...,
) {
    // struct va_format and kernel varargs/printk behavior are supplied externally.
    if !__mlog_test_u64(*mask, mlog_and_bits) || __mlog_test_u64(*mask, mlog_not_bits) {
        return;
    }

    let (level, prefix): (*const c_char, *const c_char);
    if *mask & ML_ERROR != 0 {
        level = KERN_ERR;
        prefix = b"ERROR: \0".as_ptr() as *const c_char;
    } else if *mask & ML_NOTICE != 0 {
        level = KERN_NOTICE;
        prefix = b"\0".as_ptr() as *const c_char;
    } else {
        level = KERN_INFO;
        prefix = b"\0".as_ptr() as *const c_char;
    }

    // va_start/va_format/printk are C kernel interfaces; preserve the call contract.
    printk(level, current, func, line, prefix, fmt);
}
// EXPORT_SYMBOL_GPL(__mlog_printk);

#[repr(C)]
pub struct mlog_attribute {
    pub attr: attribute,
    pub mask: u64,
}

macro_rules! define_mask {
    ($name:ident) => {
        mlog_attribute {
            attr: attribute { name: stringify!($name), mode: S_IRUGO | S_IWUSR },
            mask: concat_idents!(ML_, $name),
        }
    };
}

static mut mlog_attrs: [mlog_attribute; MLOG_MAX_BITS] = [
    define_mask!(TCP), define_mask!(MSG), define_mask!(SOCKET), define_mask!(HEARTBEAT),
    define_mask!(HB_BIO), define_mask!(DLMFS), define_mask!(DLM), define_mask!(DLM_DOMAIN),
    define_mask!(DLM_THREAD), define_mask!(DLM_MASTER), define_mask!(DLM_RECOVERY),
    define_mask!(DLM_GLUE), define_mask!(VOTE), define_mask!(CONN), define_mask!(QUORUM),
    define_mask!(BASTS), define_mask!(CLUSTER), define_mask!(ERROR), define_mask!(NOTICE),
    define_mask!(KTHREAD),
];

static mut mlog_default_attrs: [*mut attribute; MLOG_MAX_BITS] = [core::ptr::null_mut(); MLOG_MAX_BITS];

unsafe fn mlog_show(_obj: *mut kobject, attr: *mut attribute, buf: *mut c_char) -> ssize_t {
    let mlog_attr = container_of!(attr, mlog_attribute, attr);
    mlog_mask_show((*mlog_attr).mask, buf)
}

unsafe fn mlog_store(_obj: *mut kobject, attr: *mut attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let mlog_attr = container_of!(attr, mlog_attribute, attr);
    mlog_mask_store((*mlog_attr).mask, buf, count)
}

static mlog_attr_ops: sysfs_ops = sysfs_ops { show: Some(mlog_show), store: Some(mlog_store) };
static mut mlog_ktype: kobj_type = kobj_type { default_groups: mlog_default_groups, sysfs_ops: &mlog_attr_ops };
static mut mlog_kset: kset = kset { kobj: kobject { ktype: &mlog_ktype } };

pub unsafe fn mlog_sys_init(o2cb_kset: *mut kset) -> c_int {
    let mut i = 0;
    while (*mlog_attrs.as_ptr().add(i)).attr.mode != 0 {
        mlog_default_attrs[i] = &mut (*mlog_attrs.as_mut_ptr().add(i)).attr;
        i += 1;
    }
    mlog_default_attrs[i] = core::ptr::null_mut();
    kobject_set_name(&mut mlog_kset.kobj, b"logmask\0".as_ptr() as *const c_char);
    mlog_kset.kobj.kset = o2cb_kset;
    kset_register(&mut mlog_kset)
}

pub unsafe fn mlog_sys_shutdown() {
    kset_unregister(&mut mlog_kset);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
