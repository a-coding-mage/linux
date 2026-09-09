// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Create default crypto algorithm instances.
 *
 * Copyright (c) 2006 Herbert Xu <herbert@gondor.apana.org.au>
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct cryptomgr_param {
    pub tb: [*mut rtattr; CRYPTO_MAX_ATTRS + 2],
    pub r#type: rtattr_type,
    pub attrs: [crypto_attr_alg_type; CRYPTO_MAX_ATTRS],
    pub template: [c_char; CRYPTO_MAX_ALG_NAME],
    pub larval: *mut crypto_larval,
    pub otype: u32,
    pub omask: u32,
}

#[repr(C)]
pub struct rtattr_type {
    pub attr: rtattr,
    pub data: crypto_attr_type,
}

#[repr(C)]
pub struct crypto_attr_alg_type {
    pub attr: rtattr,
    pub data: crypto_attr_alg,
}

#[repr(C)]
pub struct crypto_test_param {
    pub driver: [c_char; CRYPTO_MAX_ALG_NAME],
    pub alg: [c_char; CRYPTO_MAX_ALG_NAME],
    pub r#type: u32,
}

unsafe fn cryptomgr_probe(data: *mut c_void) -> c_int {
    let param = data as *mut cryptomgr_param;
    let mut tmpl: *mut crypto_template;
    let mut err: c_int = -ENOENT;

    tmpl = crypto_lookup_template((*param).template.as_ptr());
    if tmpl.is_null() { goto_out(param, err); }

    loop {
        err = ((*tmpl).create)(tmpl, (*param).tb.as_mut_ptr());
        if !(err == -EAGAIN && !signal_pending(current)) { break; }
    }
    crypto_tmpl_put(tmpl);

    goto_out(param, err);
    module_put_and_kthread_exit(0);
}

unsafe fn goto_out(param: *mut cryptomgr_param, err: c_int) {
    (*param).larval.as_mut().unwrap().adult = ERR_PTR(err);
    (*param).larval.as_mut().unwrap().alg.cra_flags |= CRYPTO_ALG_DEAD;
    complete_all(&mut (*param).larval.as_mut().unwrap().completion);
    crypto_alg_put(&mut (*param).larval.as_mut().unwrap().alg);
    kfree(param as *mut c_void);
}

unsafe fn cryptomgr_schedule_probe(larval: *mut crypto_larval) -> c_int {
    let mut thread: *mut task_struct;
    let param = kzalloc_obj::<cryptomgr_param>();
    if !try_module_get(THIS_MODULE) || param.is_null() { return NOTIFY_OK; }

    let mut p = (*larval).alg.cra_name;
    let start = p;
    while isalnum(*p) || *p == b'-' as c_char || *p == b'_' as c_char { p = p.add(1); }
    let len = p.offset_from(start) as usize;
    if len == 0 || *p != b'(' as c_char { kfree(param as *mut c_void); module_put(THIS_MODULE); return NOTIFY_OK; }
    ptr::copy_nonoverlapping(start, (*param).template.as_mut_ptr(), len);

    let mut i = 0usize;
    loop {
        let name = p.add(1);
        p = name;
        while isalnum(*p) || *p == b'-' as c_char || *p == b'_' as c_char { p = p.add(1); }
        if *p == b'(' as c_char {
            let mut recursion = 0;
            loop {
                p = p.add(1);
                if *p == 0 { kfree(param as *mut c_void); module_put(THIS_MODULE); return NOTIFY_OK; }
                if *p == b'(' as c_char { recursion += 1; }
                else if *p == b')' as c_char && recursion == 0 { break; }
                else if *p == b')' as c_char { recursion -= 1; }
            }
            p = p.add(1);
        }
        let len = p.offset_from(name) as usize;
        if len == 0 { kfree(param as *mut c_void); module_put(THIS_MODULE); return NOTIFY_OK; }
        (*param).attrs[i].attr.rta_len = size_of::<crypto_attr_alg_type>() as _;
        (*param).attrs[i].attr.rta_type = CRYPTOA_ALG;
        ptr::copy_nonoverlapping(name, (*param).attrs[i].data.name.as_mut_ptr(), len);
        (*param).tb[i + 1] = &mut (*param).attrs[i].attr;
        i += 1;
        if i >= CRYPTO_MAX_ATTRS || *p != b')' as c_char && *p != b',' as c_char { kfree(param as *mut c_void); module_put(THIS_MODULE); return NOTIFY_OK; }
        if *p == b')' as c_char { break; }
    }
    (*param).tb[i + 1] = ptr::null_mut();
    (*param).type.attr.rta_len = size_of::<rtattr_type>() as _;
    (*param).type.attr.rta_type = CRYPTOA_TYPE;
    (*param).type.data.r#type = (*larval).alg.cra_flags & !CRYPTO_ALG_TESTED;
    (*param).type.data.mask = (*larval).mask & !CRYPTO_ALG_TESTED;
    (*param).tb[0] = &mut (*param).type.attr;
    (*param).otype = (*larval).alg.cra_flags;
    (*param).omask = (*larval).mask;
    crypto_alg_get(&mut (*larval).alg);
    (*param).larval = larval;
    thread = kthread_run(cryptomgr_probe, param as *mut c_void, b"cryptomgr_probe\0".as_ptr() as _);
    if IS_ERR(thread) { crypto_alg_put(&mut (*larval).alg); kfree(param as *mut c_void); module_put(THIS_MODULE); return NOTIFY_OK; }
    NOTIFY_STOP
}

unsafe fn cryptomgr_test(data: *mut c_void) -> c_int {
    let param = data as *mut crypto_test_param;
    let err = alg_test((*param).driver.as_ptr(), (*param).alg.as_ptr(), (*param).r#type, CRYPTO_ALG_TESTED);
    crypto_alg_tested((*param).driver.as_ptr(), err);
    kfree(param as *mut c_void); module_put_and_kthread_exit(0);
}

unsafe fn cryptomgr_schedule_test(alg: *mut crypto_alg) -> c_int {
    if !IS_ENABLED(CONFIG_CRYPTO_SELFTESTS) { return NOTIFY_DONE; }
    if !try_module_get(THIS_MODULE) { return NOTIFY_OK; }
    let param = kzalloc_obj::<crypto_test_param>();
    if param.is_null() { module_put(THIS_MODULE); return NOTIFY_OK; }
    ptr::copy_nonoverlapping((*alg).cra_driver_name, (*param).driver.as_mut_ptr(), (*param).driver.len());
    ptr::copy_nonoverlapping((*alg).cra_name, (*param).alg.as_mut_ptr(), (*param).alg.len());
    (*param).r#type = (*alg).cra_flags;
    let thread = kthread_run(cryptomgr_test, param as *mut c_void, b"cryptomgr_test\0".as_ptr() as _);
    if IS_ERR(thread) { kfree(param as *mut c_void); module_put(THIS_MODULE); return NOTIFY_OK; }
    NOTIFY_STOP
}

unsafe fn cryptomgr_notify(_this: *mut notifier_block, msg: c_ulong, data: *mut c_void) -> c_int {
    match msg {
        CRYPTO_MSG_ALG_REQUEST => cryptomgr_schedule_probe(data as *mut crypto_larval),
        CRYPTO_MSG_ALG_REGISTER => cryptomgr_schedule_test(data as *mut crypto_alg),
        CRYPTO_MSG_ALG_LOADED => NOTIFY_DONE,
        _ => NOTIFY_DONE,
    }
}

static mut cryptomgr_notifier: notifier_block = notifier_block { notifier_call: Some(cryptomgr_notify) };

unsafe fn cryptomgr_init() -> c_int { crypto_register_notifier(&mut cryptomgr_notifier) }

unsafe fn cryptomgr_exit() {
    let err = crypto_unregister_notifier(&mut cryptomgr_notifier);
    BUG_ON(err);
}

module_init!(cryptomgr_init);
module_exit!(cryptomgr_exit);
module_license!("GPL");
module_description!("Crypto Algorithm Manager");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
