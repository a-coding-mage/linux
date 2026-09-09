// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/net/netfilter/xt_IDLETIMER.c
 *
 * Netfilter module to trigger a timer when packet matches.
 * After timer expires a kevent will be sent.
 *
 * Copyright (C) 2004, 2010 Nokia Corporation
 * Written by Timo Teras <ext-timo.teras@nokia.com>
 *
 * Converted to x_tables and reworked for upstream inclusion
 * by Luciano Coelho <luciano.coelho@nokia.com>
 *
 * Contact: Luciano Coelho <luciano.coelho@nokia.com>
 */

// Dependencies supplied by the kernel and other translation units are intentionally external.

#[repr(C)]
struct idletimer_tg {
    entry: list_head,
    alarm: alarm,
    timer: timer_list,
    work: work_struct,
    kobj: *mut kobject,
    attr: device_attribute,
    refcnt: c_uint,
    timer_type: u8,
}

static mut idletimer_tg_list: list_head = LIST_HEAD_INIT(idletimer_tg_list);
static mut list_mutex: mutex = DEFINE_MUTEX_INIT(list_mutex);
static mut idletimer_tg_kobj: *mut kobject = core::ptr::null_mut();

unsafe fn __idletimer_tg_find_by_label(label: *const c_char) -> *mut idletimer_tg {
    let mut entry: *mut idletimer_tg;
    list_for_each_entry!(entry, &mut idletimer_tg_list, entry) {
        if strcmp(label, (*entry).attr.attr.name) == 0 {
            return entry;
        }
    }
    core::ptr::null_mut()
}

unsafe extern "C" fn idletimer_tg_show(
    _dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let mut timer: *mut idletimer_tg;
    let mut expires: c_ulong = 0;
    let mut ktimespec: timespec64 = core::mem::zeroed();
    let mut time_diff: c_long = 0;

    mutex_lock(&mut list_mutex);
    timer = __idletimer_tg_find_by_label((*attr).attr.name);
    if !timer.is_null() {
        if (*timer).timer_type & XT_IDLETIMER_ALARM != 0 {
            let expires_alarm: ktime_t = alarm_expires_remaining(&(*timer).alarm);
            ktimespec = ktime_to_timespec64(expires_alarm);
            time_diff = ktimespec.tv_sec;
        } else {
            expires = (*timer).timer.expires;
            time_diff = jiffies_to_msecs(expires.wrapping_sub(jiffies)) / 1000;
        }
    }
    mutex_unlock(&mut list_mutex);

    if time_after(expires, jiffies) || ktimespec.tv_sec > 0 {
        return sysfs_emit(buf, "%ld\n", time_diff);
    }
    sysfs_emit(buf, "0\n")
}

unsafe extern "C" fn idletimer_tg_work(work: *mut work_struct) {
    let timer = container_of!(work, idletimer_tg, work);
    sysfs_notify(idletimer_tg_kobj, core::ptr::null(), (*timer).attr.attr.name);
}

unsafe extern "C" fn idletimer_tg_expired(t: *mut timer_list) {
    let timer = timer_container_of!(t, idletimer_tg, timer);
    schedule_work(&mut (*timer).work);
}

unsafe extern "C" fn idletimer_tg_alarmproc(alarm: *mut alarm, _now: ktime_t) {
    let timer = (*alarm).data as *mut idletimer_tg;
    schedule_work(&mut (*timer).work);
}

unsafe fn idletimer_start_alarm_ktime(timer: *mut idletimer_tg, timeout: ktime_t) {
    /* The timer should always be queued as @tout it should be least one second,
     * but handle it correctly in any case. Virt will manage! */
    if !alarm_start_timer(&mut (*timer).alarm, timeout, true) {
        schedule_work(&mut (*timer).work);
    }
}

unsafe fn idletimer_start_alarm_sec(timer: *mut idletimer_tg, seconds: c_uint) {
    idletimer_start_alarm_ktime(timer, ktime_set(seconds as c_long, 0));
}

unsafe fn idletimer_check_sysfs_name(name: *const c_char, size: c_uint) -> c_int {
    let ret = xt_check_proc_name(name, size);
    if ret < 0 { return ret; }
    if strcmp(name, c"power".as_ptr()) == 0 || strcmp(name, c"subsystem".as_ptr()) == 0 || strcmp(name, c"uevent".as_ptr()) == 0 {
        return -EINVAL;
    }
    0
}

unsafe fn idletimer_tg_create(info: *mut idletimer_tg_info) -> c_int {
    let mut ret: c_int;
    (*info).timer = kzalloc_obj!((*info).timer);
    if (*info).timer.is_null() { return -ENOMEM; }
    ret = idletimer_check_sysfs_name((*info).label.as_ptr(), core::mem::size_of_val(&(*info).label) as c_uint);
    if ret < 0 { goto!(out_free_timer); }
    sysfs_attr_init(&mut (*(*info).timer).attr.attr);
    (*(*info).timer).attr.attr.name = kstrdup((*info).label.as_ptr(), GFP_KERNEL);
    if (*(*info).timer).attr.attr.name.is_null() { ret = -ENOMEM; goto!(out_free_timer); }
    (*(*info).timer).attr.attr.mode = 0o444;
    (*(*info).timer).attr.show = Some(idletimer_tg_show);
    ret = sysfs_create_file(idletimer_tg_kobj, &(*(*info).timer).attr.attr);
    if ret < 0 { pr_info_ratelimited!("couldn't add file to sysfs\n"); goto!(out_free_attr); }
    list_add(&mut (*(*info).timer).entry, &mut idletimer_tg_list);
    timer_setup(&mut (*(*info).timer).timer, Some(idletimer_tg_expired), 0);
    (*(*info).timer).refcnt = 1;
    INIT_WORK(&mut (*(*info).timer).work, Some(idletimer_tg_work));
    mod_timer(&mut (*(*info).timer).timer, secs_to_jiffies((*info).timeout) + jiffies);
    return 0;
out_free_attr:
    kfree((*(*info).timer).attr.attr.name as *mut c_void);
out_free_timer:
    kfree((*info).timer as *mut c_void);
    ret
}

unsafe fn idletimer_tg_create_v1(info: *mut idletimer_tg_info_v1) -> c_int {
    let mut ret: c_int;
    (*info).timer = kmalloc_obj!((*info).timer);
    if (*info).timer.is_null() { return -ENOMEM; }
    ret = idletimer_check_sysfs_name((*info).label.as_ptr(), core::mem::size_of_val(&(*info).label) as c_uint);
    if ret < 0 { goto!(out_free_timer); }
    sysfs_attr_init(&mut (*(*info).timer).attr.attr);
    (*(*info).timer).attr.attr.name = kstrdup((*info).label.as_ptr(), GFP_KERNEL);
    if (*(*info).timer).attr.attr.name.is_null() { ret = -ENOMEM; goto!(out_free_timer); }
    (*(*info).timer).attr.attr.mode = 0o444;
    (*(*info).timer).attr.show = Some(idletimer_tg_show);
    ret = sysfs_create_file(idletimer_tg_kobj, &(*(*info).timer).attr.attr);
    if ret < 0 { pr_info_ratelimited!("couldn't add file to sysfs\n"); goto!(out_free_attr); }
    kobject_uevent(idletimer_tg_kobj, KOBJ_ADD);
    list_add(&mut (*(*info).timer).entry, &mut idletimer_tg_list);
    (*(*info).timer).timer_type = (*info).timer_type;
    (*(*info).timer).refcnt = 1;
    INIT_WORK(&mut (*(*info).timer).work, Some(idletimer_tg_work));
    if (*(*info).timer).timer_type & XT_IDLETIMER_ALARM != 0 {
        alarm_init(&mut (*(*info).timer).alarm, ALARM_BOOTTIME, Some(idletimer_tg_alarmproc));
        (*(*info).timer).alarm.data = (*info).timer as *mut c_void;
        idletimer_start_alarm_sec((*info).timer, (*info).timeout);
    } else {
        timer_setup(&mut (*(*info).timer).timer, Some(idletimer_tg_expired), 0);
        mod_timer(&mut (*(*info).timer).timer, secs_to_jiffies((*info).timeout) + jiffies);
    }
    return 0;
out_free_attr:
    kfree((*(*info).timer).attr.attr.name as *mut c_void);
out_free_timer:
    kfree((*info).timer as *mut c_void);
    ret
}

unsafe extern "C" fn idletimer_tg_target(_skb: *mut sk_buff, par: *const xt_action_param) -> c_uint {
    let info = (*par).targinfo as *const idletimer_tg_info;
    mod_timer(&mut (*(*info).timer).timer, secs_to_jiffies((*info).timeout) + jiffies);
    XT_CONTINUE
}

unsafe extern "C" fn idletimer_tg_target_v1(_skb: *mut sk_buff, par: *const xt_action_param) -> c_uint {
    let info = (*par).targinfo as *const idletimer_tg_info_v1;
    if (*(*info).timer).timer_type & XT_IDLETIMER_ALARM != 0 {
        idletimer_start_alarm_sec((*info).timer, (*info).timeout);
    } else {
        mod_timer(&mut (*(*info).timer).timer, secs_to_jiffies((*info).timeout) + jiffies);
    }
    XT_CONTINUE
}

unsafe fn idletimer_tg_helper(info: *mut idletimer_tg_info) -> c_int {
    if (*info).timeout == 0 { pr_info_ratelimited!("timeout value is zero\n"); return -EINVAL; }
    if (*info).timeout >= INT_MAX / 1000 { pr_info_ratelimited!("timeout value is too big\n"); return -EINVAL; }
    if (*info).label[0] == 0 || strnlen((*info).label.as_ptr(), MAX_IDLETIMER_LABEL_SIZE) == MAX_IDLETIMER_LABEL_SIZE { pr_info_ratelimited!("label is empty or not nul-terminated\n"); return -EINVAL; }
    0
}

unsafe extern "C" fn idletimer_tg_checkentry(par: *const xt_tgchk_param) -> c_int {
    let info = (*par).targinfo as *mut idletimer_tg_info;
    let mut ret = idletimer_tg_helper(info);
    if ret < 0 { return -EINVAL; }
    mutex_lock(&mut list_mutex);
    (*info).timer = __idletimer_tg_find_by_label((*info).label.as_ptr());
    if !(*info).timer.is_null() {
        if (*(*info).timer).timer_type & XT_IDLETIMER_ALARM != 0 { mutex_unlock(&mut list_mutex); pr_info_ratelimited!("Adding/Replacing rule with same label and different timer type is not allowed\n"); return -EINVAL; }
        (*(*info).timer).refcnt += 1;
        mod_timer(&mut (*(*info).timer).timer, secs_to_jiffies((*info).timeout) + jiffies);
    } else {
        ret = idletimer_tg_create(info);
        if ret < 0 { mutex_unlock(&mut list_mutex); return ret; }
    }
    mutex_unlock(&mut list_mutex);
    0
}

unsafe extern "C" fn idletimer_tg_checkentry_v1(par: *const xt_tgchk_param) -> c_int {
    let info = (*par).targinfo as *mut idletimer_tg_info_v1;
    if (*info).send_nl_msg { return -EOPNOTSUPP; }
    let ret = idletimer_tg_helper(info as *mut idletimer_tg_info);
    if ret < 0 || (*info).timer_type > XT_IDLETIMER_ALARM { return -EINVAL; }
    mutex_lock(&mut list_mutex);
    (*info).timer = __idletimer_tg_find_by_label((*info).label.as_ptr());
    if !(*info).timer.is_null() {
        if (*(*info).timer).timer_type != (*info).timer_type { mutex_unlock(&mut list_mutex); pr_info_ratelimited!("Adding/Replacing rule with same label and different timer type is not allowed\n"); return -EINVAL; }
        (*(*info).timer).refcnt += 1;
        if (*info).timer_type & XT_IDLETIMER_ALARM != 0 {
            let tout = alarm_expires_remaining(&(*(*info).timer).alarm);
            let ktimespec = ktime_to_timespec64(tout);
            if ktimespec.tv_sec > 0 { idletimer_start_alarm_ktime((*info).timer, tout); }
        } else { mod_timer(&mut (*(*info).timer).timer, secs_to_jiffies((*info).timeout) + jiffies); }
    } else {
        let ret = idletimer_tg_create_v1(info);
        if ret < 0 { mutex_unlock(&mut list_mutex); return ret; }
    }
    mutex_unlock(&mut list_mutex);
    0
}

unsafe extern "C" fn idletimer_tg_destroy(par: *const xt_tgdtor_param) {
    let info = (*par).targinfo as *const idletimer_tg_info;
    mutex_lock(&mut list_mutex);
    (*(*info).timer).refcnt -= 1;
    if (*(*info).timer).refcnt > 0 { mutex_unlock(&mut list_mutex); return; }
    list_del(&mut (*(*info).timer).entry);
    mutex_unlock(&mut list_mutex);
    timer_shutdown_sync(&mut (*(*info).timer).timer);
    cancel_work_sync(&mut (*(*info).timer).work);
    sysfs_remove_file(idletimer_tg_kobj, &(*(*info).timer).attr.attr);
    kfree((*(*info).timer).attr.attr.name as *mut c_void);
    kfree((*info).timer as *mut c_void);
}

unsafe extern "C" fn idletimer_tg_destroy_v1(par: *const xt_tgdtor_param) {
    let info = (*par).targinfo as *const idletimer_tg_info_v1;
    mutex_lock(&mut list_mutex);
    (*(*info).timer).refcnt -= 1;
    if (*(*info).timer).refcnt > 0 { mutex_unlock(&mut list_mutex); return; }
    list_del(&mut (*(*info).timer).entry);
    mutex_unlock(&mut list_mutex);
    if (*(*info).timer).timer_type & XT_IDLETIMER_ALARM != 0 { alarm_cancel(&mut (*(*info).timer).alarm); } else { timer_shutdown_sync(&mut (*(*info).timer).timer); }
    cancel_work_sync(&mut (*(*info).timer).work);
    sysfs_remove_file(idletimer_tg_kobj, &(*(*info).timer).attr.attr);
    kfree((*(*info).timer).attr.attr.name as *mut c_void);
    kfree((*info).timer as *mut c_void);
}

// The xt_target table, module initialization/cleanup, and metadata retain the
// same externally visible registrations as the C implementation. The target
// structure types and registration functions are supplied by kernel bindings.
static mut idletimer_tg: [xt_target; 4] = [
    xt_target { name: *b"IDLETIMER\0", family: NFPROTO_IPV4, revision: 0, target: Some(idletimer_tg_target), targetsize: core::mem::size_of::<idletimer_tg_info>(), usersize: core::mem::offset_of!(idletimer_tg_info, timer), checkentry: Some(idletimer_tg_checkentry), destroy: Some(idletimer_tg_destroy), me: THIS_MODULE },
    xt_target { name: *b"IDLETIMER\0", family: NFPROTO_IPV4, revision: 1, target: Some(idletimer_tg_target_v1), targetsize: core::mem::size_of::<idletimer_tg_info_v1>(), usersize: core::mem::offset_of!(idletimer_tg_info_v1, timer), checkentry: Some(idletimer_tg_checkentry_v1), destroy: Some(idletimer_tg_destroy_v1), me: THIS_MODULE },
    // CONFIG_IP6_NF_IPTABLES conditional: IPv6 registrations are preserved by the bindings when enabled.
    xt_target { name: *b"IDLETIMER\0", family: NFPROTO_IPV6, revision: 0, target: Some(idletimer_tg_target), targetsize: core::mem::size_of::<idletimer_tg_info>(), usersize: core::mem::offset_of!(idletimer_tg_info, timer), checkentry: Some(idletimer_tg_checkentry), destroy: Some(idletimer_tg_destroy), me: THIS_MODULE },
    xt_target { name: *b"IDLETIMER\0", family: NFPROTO_IPV6, revision: 1, target: Some(idletimer_tg_target_v1), targetsize: core::mem::size_of::<idletimer_tg_info_v1>(), usersize: core::mem::offset_of!(idletimer_tg_info_v1, timer), checkentry: Some(idletimer_tg_checkentry_v1), destroy: Some(idletimer_tg_destroy_v1), me: THIS_MODULE },
];

static mut idletimer_tg_class: *mut class = core::ptr::null_mut();
static mut idletimer_tg_device: *mut device = core::ptr::null_mut();

unsafe extern "C" fn idletimer_tg_init() -> c_int {
    idletimer_tg_class = class_create(c"xt_idletimer".as_ptr());
    let mut err = PTR_ERR(idletimer_tg_class);
    if IS_ERR(idletimer_tg_class) { pr_err!("couldn't register device class\n"); return err; }
    idletimer_tg_device = device_create(idletimer_tg_class, core::ptr::null_mut(), MKDEV(0, 0), core::ptr::null_mut(), c"timers".as_ptr());
    err = PTR_ERR(idletimer_tg_device);
    if IS_ERR(idletimer_tg_device) { pr_err!("couldn't register system device\n"); class_destroy(idletimer_tg_class); return err; }
    idletimer_tg_kobj = &mut (*idletimer_tg_device).kobj;
    err = xt_register_targets(idletimer_tg.as_mut_ptr(), idletimer_tg.len());
    if err < 0 { pr_err!("couldn't register xt target\n"); device_destroy(idletimer_tg_class, MKDEV(0, 0)); class_destroy(idletimer_tg_class); return err; }
    0
}

unsafe extern "C" fn idletimer_tg_exit() {
    xt_unregister_targets(idletimer_tg.as_mut_ptr(), idletimer_tg.len());
    device_destroy(idletimer_tg_class, MKDEV(0, 0));
    class_destroy(idletimer_tg_class);
}

module_init!(idletimer_tg_init);
module_exit!(idletimer_tg_exit);
module_author!("Timo Teras <ext-timo.teras@nokia.com>");
module_author!("Luciano Coelho <luciano.coelho@nokia.com>");
module_description!("Xtables: idle time monitor");
module_license!("GPL v2");
module_alias!("ipt_IDLETIMER");
module_alias!("ip6t_IDLETIMER");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
