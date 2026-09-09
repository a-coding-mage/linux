// SPDX-License-Identifier: GPL-2.0
/* net/atm/resources.c - Statically allocated resources */
/* Written 1995-2000 by Werner Almesberger, EPFL LRC/ICA */
/* Fixes
 * Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 * 2002/01 - don't free the whole struct sock on sk->destruct time,
 *           use the default destruct function initialized by sock_init_data
 */

// Dependencies are supplied by the surrounding kernel translation unit.

#[allow(non_upper_case_globals)]
pub static mut atm_devs: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
pub static mut atm_dev_mutex: mutex = mutex::new();

unsafe fn __alloc_atm_dev(type_: *const c_char) -> *mut atm_dev {
    let dev = kzalloc_obj::<atm_dev>();
    if dev.is_null() { return core::ptr::null_mut(); }
    (*dev).type_ = type_;
    (*dev).signal = ATM_PHY_SIG_UNKNOWN;
    (*dev).link_rate = ATM_OC3_PCR;
    dev
}

unsafe fn __atm_dev_lookup(number: c_int) -> *mut atm_dev {
    let mut dev: *mut atm_dev = core::ptr::null_mut();
    list_for_each_entry!(dev, &mut atm_devs, dev_list) {
        if (*dev).number == number {
            atm_dev_hold(dev);
            return dev;
        }
    }
    core::ptr::null_mut()
}

pub unsafe fn atm_dev_lookup(number: c_int) -> *mut atm_dev {
    let dev;
    mutex_lock(&mut atm_dev_mutex);
    dev = __atm_dev_lookup(number);
    mutex_unlock(&mut atm_dev_mutex);
    dev
}

pub unsafe fn atm_dev_register(type_: *const c_char, parent: *mut device,
    ops: *const atmdev_ops, number: c_int, flags: *mut c_ulong) -> *mut atm_dev {
    let dev = __alloc_atm_dev(type_);
    if dev.is_null() { pr_err!("no space for dev %s\n", type_); return core::ptr::null_mut(); }
    mutex_lock(&mut atm_dev_mutex);
    if number != -1 {
        let inuse = __atm_dev_lookup(number);
        if !inuse.is_null() {
            atm_dev_put(inuse); mutex_unlock(&mut atm_dev_mutex); kfree(dev); return core::ptr::null_mut();
        }
        (*dev).number = number;
    } else {
        (*dev).number = 0;
        loop {
            let inuse = __atm_dev_lookup((*dev).number);
            if inuse.is_null() { break; }
            atm_dev_put(inuse); (*dev).number += 1;
        }
    }
    (*dev).ops = ops;
    if !flags.is_null() { (*dev).flags = *flags; } else { memset(&mut (*dev).flags as *mut _, 0, core::mem::size_of_val(&(*dev).flags)); }
    memset(&mut (*dev).stats as *mut _, 0, core::mem::size_of_val(&(*dev).stats));
    refcount_set(&mut (*dev).refcnt, 1);
    if atm_proc_dev_register(dev) < 0 { pr_err!("atm_proc_dev_register failed for dev %s\n", type_); mutex_unlock(&mut atm_dev_mutex); kfree(dev); return core::ptr::null_mut(); }
    if atm_register_sysfs(dev, parent) < 0 { pr_err!("atm_register_sysfs failed for dev %s\n", type_); atm_proc_dev_deregister(dev); put_device(&mut (*dev).class_dev); mutex_unlock(&mut atm_dev_mutex); return core::ptr::null_mut(); }
    list_add_tail(&mut (*dev).dev_list, &mut atm_devs);
    mutex_unlock(&mut atm_dev_mutex);
    dev
}

pub unsafe fn atm_dev_deregister(dev: *mut atm_dev) {
    BUG_ON!(test_bit(ATM_DF_REMOVED, &(*dev).flags));
    set_bit(ATM_DF_REMOVED, &mut (*dev).flags);
    mutex_lock(&mut atm_dev_mutex);
    list_del(&mut (*dev).dev_list);
    atm_dev_release_vccs(dev);
    atm_unregister_sysfs(dev);
    atm_proc_dev_deregister(dev);
    mutex_unlock(&mut atm_dev_mutex);
    atm_dev_put(dev);
}

unsafe fn copy_aal_stats(from: *mut k_atm_aal_stats, to: *mut atm_aal_stats) {
    aal_stat_items!(|i| { (*to).i = atomic_read(&mut (*from).i); });
}

unsafe fn subtract_aal_stats(from: *mut k_atm_aal_stats, to: *mut atm_aal_stats) {
    aal_stat_items!(|i| { atomic_sub((*to).i, &mut (*from).i); });
}

unsafe fn fetch_stats(dev: *mut atm_dev, arg: *mut atm_dev_stats, zero: bool) -> c_int {
    let mut tmp: atm_dev_stats = core::mem::zeroed();
    copy_aal_stats(&mut (*dev).stats.aal0, &mut tmp.aal0);
    copy_aal_stats(&mut (*dev).stats.aal34, &mut tmp.aal34);
    copy_aal_stats(&mut (*dev).stats.aal5, &mut tmp.aal5);
    let error = if !arg.is_null() { copy_to_user(arg, &tmp, core::mem::size_of_val(&tmp)) } else { 0 };
    if zero && error == 0 { subtract_aal_stats(&mut (*dev).stats.aal0, &mut tmp.aal0); subtract_aal_stats(&mut (*dev).stats.aal34, &mut tmp.aal34); subtract_aal_stats(&mut (*dev).stats.aal5, &mut tmp.aal5); }
    if error != 0 { -EFAULT } else { 0 }
}

pub unsafe fn atm_getnames(buf: *mut c_void, iobuf_len: *mut c_int) -> c_int {
    let len = get_user(iobuf_len); let mut size = 0usize;
    mutex_lock(&mut atm_dev_mutex);
    list_for_each!(p, &atm_devs) { size += core::mem::size_of::<c_int>(); }
    if size > len as usize { mutex_unlock(&mut atm_dev_mutex); return -E2BIG; }
    let tmp_buf = kmalloc(size, GFP_ATOMIC) as *mut c_int;
    if tmp_buf.is_null() { mutex_unlock(&mut atm_dev_mutex); return -ENOMEM; }
    let mut tmp_p = tmp_buf;
    list_for_each_entry!(dev, &mut atm_devs, dev_list) { *tmp_p = (*dev).number; tmp_p = tmp_p.add(1); }
    mutex_unlock(&mut atm_dev_mutex);
    let error = if copy_to_user(buf, tmp_buf, size) != 0 || put_user(size as c_int, iobuf_len) != 0 { -EFAULT } else { 0 };
    kfree(tmp_buf as *mut c_void); error
}

pub unsafe fn atm_dev_ioctl(cmd: c_uint, buf: *mut c_void, sioc_len: *mut c_int, number: c_int, compat: c_int) -> c_int {
    let dev = try_then_request_module!(atm_dev_lookup(number), "atm-device-%d", number);
    if dev.is_null() { return -ENODEV; }
    let mut error = 0; let mut size = 0;
    match cmd {
        ATM_GETTYPE => { size = strlen((*dev).type_) + 1; if copy_to_user(buf, (*dev).type_, size) != 0 { error = -EFAULT; } }
        ATM_GETESI => { size = ESI_LEN; if copy_to_user(buf, (*dev).esi.as_ptr(), size) != 0 { error = -EFAULT; } }
        ATM_SETESI | ATM_SETESIF => { if !capable(CAP_NET_ADMIN) { error = -EPERM; } else { let mut esi = [0u8; ESI_LEN]; if copy_from_user(esi.as_mut_ptr(), buf, ESI_LEN) != 0 { error = -EFAULT; } else { (*dev).esi.copy_from_slice(&esi); error = ESI_LEN as c_int; } } }
        ATM_GETSTATZ | ATM_GETSTAT => { if cmd == ATM_GETSTATZ && !capable(CAP_NET_ADMIN) { error = -EPERM; } else { size = core::mem::size_of::<atm_dev_stats>(); error = fetch_stats(dev, buf as *mut atm_dev_stats, cmd == ATM_GETSTATZ); } }
        ATM_GETCIRANGE => { size = core::mem::size_of::<atm_cirange>(); if copy_to_user(buf, &(*dev).ci_range, size) != 0 { error = -EFAULT; } }
        ATM_GETLINKRATE => { size = core::mem::size_of::<c_int>(); if copy_to_user(buf, &(*dev).link_rate, size) != 0 { error = -EFAULT; } }
        ATM_SETLOOP | ATM_SETCIRANGE => { if !capable(CAP_NET_ADMIN) { error = -EPERM; } else { error = dispatch_ioctl(dev, cmd, buf, compat); } }
        _ => { error = dispatch_ioctl(dev, cmd, buf, compat); }
    }
    if error == 0 && size != 0 { error = if put_user(size, sioc_len) != 0 { -EFAULT } else { 0 }; }
    atm_dev_put(dev); error
}

#[cfg(CONFIG_PROC_FS)]
pub unsafe fn atm_dev_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void { mutex_lock(&mut atm_dev_mutex); seq_list_start_head(&mut atm_devs, *pos) }
#[cfg(CONFIG_PROC_FS)]
pub unsafe fn atm_dev_seq_stop(_seq: *mut seq_file, _v: *mut c_void) { mutex_unlock(&mut atm_dev_mutex); }
#[cfg(CONFIG_PROC_FS)]
pub unsafe fn atm_dev_seq_next(_seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void { seq_list_next(v, &mut atm_devs, pos) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
