// SPDX-License-Identifier: GPL-2.0-only
/*
 * xt_LED.c - netfilter target to make LEDs blink upon packet matches
 *
 * Copyright (C) 2008 Adam Nielsen <a.nielsen@shikadi.net>
 */

// Kernel headers and symbols below are supplied by the surrounding kernel
// translation environment.

const XT_LED_BLINK_DELAY: u32 = 50; // ms

#[repr(C)]
pub struct xt_led_info_internal {
    pub list: list_head,
    pub refcnt: i32,
    pub trigger_id: *mut i8,
    pub netfilter_led_trigger: led_trigger,
    pub timer: timer_list,
}

extern "C" {
    static mut xt_led_triggers: list_head;
    static mut xt_led_mutex: mutex;

    fn timer_pending(timer: *const timer_list) -> bool;
    fn led_trigger_blink_oneshot(trigger: *mut led_trigger, delay_on: u32,
                                 delay_off: u32, invert: u8);
    fn led_trigger_event(trigger: *mut led_trigger, brightness: u32);
    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;
    fn msecs_to_jiffies(msecs: u32) -> c_ulong;
    fn strcmp(a: *const i8, b: *const i8) -> c_int;
    fn memchr(s: *const c_void, c: c_int, n: usize) -> *mut c_void;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn kstrdup(s: *const i8, flags: gfp_t) -> *mut i8;
    fn kfree(p: *mut c_void);
    fn led_trigger_register(trigger: *mut led_trigger) -> c_int;
    fn led_trigger_unregister(trigger: *mut led_trigger);
    fn timer_setup(timer: *mut timer_list, callback: unsafe extern "C" fn(*mut timer_list), flags: u32);
    fn timer_shutdown_sync(timer: *mut timer_list);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn xt_register_targets(targets: *mut xt_target, count: u32) -> c_int;
    fn xt_unregister_targets(targets: *mut xt_target, count: u32);
}

#[no_mangle]
pub unsafe extern "C" fn led_tg(
    _skb: *mut sk_buff,
    par: *const xt_action_param,
) -> u32 {
    let ledinfo = (*par).targinfo as *mut xt_led_info;
    let ledinternal = (*ledinfo).internal_data as *mut xt_led_info_internal;

    if (*ledinfo).delay > 0 && (*ledinfo).always_blink && timer_pending(&(*ledinternal).timer) {
        led_trigger_blink_oneshot(
            &mut (*ledinternal).netfilter_led_trigger,
            XT_LED_BLINK_DELAY,
            XT_LED_BLINK_DELAY,
            1,
        );
    } else {
        led_trigger_event(&mut (*ledinternal).netfilter_led_trigger, LED_FULL);
    }

    if (*ledinfo).delay > 0 {
        mod_timer(
            &mut (*ledinternal).timer,
            jiffies.wrapping_add(msecs_to_jiffies((*ledinfo).delay as u32)),
        );
    } else if (*ledinfo).delay == 0 {
        led_trigger_event(&mut (*ledinternal).netfilter_led_trigger, LED_OFF);
    }

    XT_CONTINUE
}

unsafe extern "C" fn led_timeout_callback(t: *mut timer_list) {
    // Equivalent to timer_container_of(ledinternal, t, timer).
    let ledinternal = container_of!(t, xt_led_info_internal, timer);
    led_trigger_event(&mut (*ledinternal).netfilter_led_trigger, LED_OFF);
}

unsafe fn led_trigger_lookup(name: *const i8) -> *mut xt_led_info_internal {
    let mut pos = (*(&raw mut xt_led_triggers)).next;
    while pos != &raw mut xt_led_triggers {
        let ledinternal = container_of!(pos, xt_led_info_internal, list);
        if strcmp(name, (*ledinternal).netfilter_led_trigger.name) == 0 {
            return ledinternal;
        }
        pos = (*pos).next;
    }
    core::ptr::null_mut()
}

unsafe extern "C" fn led_tg_check(par: *const xt_tgchk_param) -> c_int {
    let ledinfo = (*par).targinfo as *mut xt_led_info;
    let mut ledinternal: *mut xt_led_info_internal;
    let mut err: c_int;

    if (*ledinfo).id[0] == 0
        || memchr((*ledinfo).id.as_ptr() as *const c_void, 0, (*ledinfo).id.len()).is_null() {
        return -EINVAL;
    }

    mutex_lock(&raw mut xt_led_mutex);
    ledinternal = led_trigger_lookup((*ledinfo).id.as_ptr());
    if !ledinternal.is_null() {
        (*ledinternal).refcnt += 1;
    } else {
        err = -ENOMEM;
        ledinternal = kzalloc_obj::<xt_led_info_internal>();
        if ledinternal.is_null() {
            mutex_unlock(&raw mut xt_led_mutex);
            return err;
        }
        (*ledinternal).trigger_id = kstrdup((*ledinfo).id.as_ptr(), GFP_KERNEL);
        if (*ledinternal).trigger_id.is_null() {
            kfree(ledinternal as *mut c_void);
            mutex_unlock(&raw mut xt_led_mutex);
            return err;
        }
        (*ledinternal).refcnt = 1;
        (*ledinternal).netfilter_led_trigger.name = (*ledinternal).trigger_id;
        err = led_trigger_register(&mut (*ledinternal).netfilter_led_trigger);
        if err != 0 {
            pr_info_ratelimited!("Trigger name is already in use.\n");
            kfree((*ledinternal).trigger_id as *mut c_void);
            kfree(ledinternal as *mut c_void);
            mutex_unlock(&raw mut xt_led_mutex);
            return err;
        }
        timer_setup(&mut (*ledinternal).timer, led_timeout_callback, 0);
        list_add_tail(&mut (*ledinternal).list, &raw mut xt_led_triggers);
    }
    mutex_unlock(&raw mut xt_led_mutex);
    (*ledinfo).internal_data = ledinternal as *mut c_void;
    0
}

unsafe extern "C" fn led_tg_destroy(par: *const xt_tgdtor_param) {
    let ledinfo = (*par).targinfo as *const xt_led_info;
    let ledinternal = (*ledinfo).internal_data as *mut xt_led_info_internal;
    mutex_lock(&raw mut xt_led_mutex);
    (*ledinternal).refcnt -= 1;
    if (*ledinternal).refcnt != 0 {
        mutex_unlock(&raw mut xt_led_mutex);
        return;
    }
    list_del(&mut (*ledinternal).list);
    timer_shutdown_sync(&mut (*ledinternal).timer);
    led_trigger_unregister(&mut (*ledinternal).netfilter_led_trigger);
    mutex_unlock(&raw mut xt_led_mutex);
    kfree((*ledinternal).trigger_id as *mut c_void);
    kfree(ledinternal as *mut c_void);
}

static mut led_tg_reg: [xt_target; 2] = [
    xt_target { name: *b"LED\0", revision: 0, family: NFPROTO_IPV4, target: Some(led_tg), targetsize: core::mem::size_of::<xt_led_info>(), usersize: core::mem::offset_of!(xt_led_info, internal_data), checkentry: Some(led_tg_check), destroy: Some(led_tg_destroy), me: THIS_MODULE },
    // CONFIG_IP6_NF_IPTABLES build-time conditional.
    xt_target { name: *b"LED\0", revision: 0, family: NFPROTO_IPV6, target: Some(led_tg), targetsize: core::mem::size_of::<xt_led_info>(), usersize: core::mem::offset_of!(xt_led_info, internal_data), checkentry: Some(led_tg_check), destroy: Some(led_tg_destroy), me: THIS_MODULE },
];

unsafe extern "C" fn led_tg_init() -> c_int {
    xt_register_targets(led_tg_reg.as_mut_ptr(), led_tg_reg.len() as u32)
}

unsafe extern "C" fn led_tg_exit() {
    xt_unregister_targets(led_tg_reg.as_mut_ptr(), led_tg_reg.len() as u32);
}

module_init!(led_tg_init);
module_exit!(led_tg_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
