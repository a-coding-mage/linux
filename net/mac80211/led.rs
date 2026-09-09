// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2006, Johannes Berg <johannes@sipsolutions.net>
 */

/* Dependencies supplied by the surrounding kernel translation. */

pub unsafe fn ieee80211_led_assoc(local: *mut ieee80211_local, associated: bool) {
    if atomic_read(&(*local).assoc_led_active) == 0 {
        return;
    }
    if associated {
        led_trigger_event(&mut (*local).assoc_led, LED_FULL);
    } else {
        led_trigger_event(&mut (*local).assoc_led, LED_OFF);
    }
}

pub unsafe fn ieee80211_led_radio(local: *mut ieee80211_local, enabled: bool) {
    if atomic_read(&(*local).radio_led_active) == 0 {
        return;
    }
    if enabled {
        led_trigger_event(&mut (*local).radio_led, LED_FULL);
    } else {
        led_trigger_event(&mut (*local).radio_led, LED_OFF);
    }
}

pub unsafe fn ieee80211_alloc_led_names(local: *mut ieee80211_local) {
    (*local).rx_led.name = kasprintf(GFP_KERNEL, "%srx", wiphy_name((*local).hw.wiphy));
    (*local).tx_led.name = kasprintf(GFP_KERNEL, "%stx", wiphy_name((*local).hw.wiphy));
    (*local).assoc_led.name = kasprintf(GFP_KERNEL, "%sassoc", wiphy_name((*local).hw.wiphy));
    (*local).radio_led.name = kasprintf(GFP_KERNEL, "%sradio", wiphy_name((*local).hw.wiphy));
}

pub unsafe fn ieee80211_free_led_names(local: *mut ieee80211_local) {
    kfree((*local).rx_led.name);
    kfree((*local).tx_led.name);
    kfree((*local).assoc_led.name);
    kfree((*local).radio_led.name);
}

unsafe fn ieee80211_tx_led_activate(led_cdev: *mut led_classdev) -> i32 {
    let local = container_of((*led_cdev).trigger, ieee80211_local, tx_led);
    atomic_inc(&mut (*local).tx_led_active);
    0
}

unsafe fn ieee80211_tx_led_deactivate(led_cdev: *mut led_classdev) {
    let local = container_of((*led_cdev).trigger, ieee80211_local, tx_led);
    atomic_dec(&mut (*local).tx_led_active);
}

unsafe fn ieee80211_rx_led_activate(led_cdev: *mut led_classdev) -> i32 {
    let local = container_of((*led_cdev).trigger, ieee80211_local, rx_led);
    atomic_inc(&mut (*local).rx_led_active);
    0
}

unsafe fn ieee80211_rx_led_deactivate(led_cdev: *mut led_classdev) {
    let local = container_of((*led_cdev).trigger, ieee80211_local, rx_led);
    atomic_dec(&mut (*local).rx_led_active);
}

unsafe fn ieee80211_assoc_led_activate(led_cdev: *mut led_classdev) -> i32 {
    let local = container_of((*led_cdev).trigger, ieee80211_local, assoc_led);
    atomic_inc(&mut (*local).assoc_led_active);
    0
}

unsafe fn ieee80211_assoc_led_deactivate(led_cdev: *mut led_classdev) {
    let local = container_of((*led_cdev).trigger, ieee80211_local, assoc_led);
    atomic_dec(&mut (*local).assoc_led_active);
}

unsafe fn ieee80211_radio_led_activate(led_cdev: *mut led_classdev) -> i32 {
    let local = container_of((*led_cdev).trigger, ieee80211_local, radio_led);
    atomic_inc(&mut (*local).radio_led_active);
    0
}

unsafe fn ieee80211_radio_led_deactivate(led_cdev: *mut led_classdev) {
    let local = container_of((*led_cdev).trigger, ieee80211_local, radio_led);
    atomic_dec(&mut (*local).radio_led_active);
}

unsafe fn ieee80211_tpt_led_activate(led_cdev: *mut led_classdev) -> i32 {
    let local = container_of((*led_cdev).trigger, ieee80211_local, tpt_led);
    atomic_inc(&mut (*local).tpt_led_active);
    0
}

unsafe fn ieee80211_tpt_led_deactivate(led_cdev: *mut led_classdev) {
    let local = container_of((*led_cdev).trigger, ieee80211_local, tpt_led);
    atomic_dec(&mut (*local).tpt_led_active);
}

pub unsafe fn ieee80211_led_init(local: *mut ieee80211_local) {
    atomic_set(&mut (*local).rx_led_active, 0);
    (*local).rx_led.activate = Some(ieee80211_rx_led_activate);
    (*local).rx_led.deactivate = Some(ieee80211_rx_led_deactivate);
    if !(*local).rx_led.name.is_null() && led_trigger_register(&mut (*local).rx_led) != 0 {
        kfree((*local).rx_led.name); (*local).rx_led.name = core::ptr::null_mut();
    }
    atomic_set(&mut (*local).tx_led_active, 0);
    (*local).tx_led.activate = Some(ieee80211_tx_led_activate);
    (*local).tx_led.deactivate = Some(ieee80211_tx_led_deactivate);
    if !(*local).tx_led.name.is_null() && led_trigger_register(&mut (*local).tx_led) != 0 {
        kfree((*local).tx_led.name); (*local).tx_led.name = core::ptr::null_mut();
    }
    atomic_set(&mut (*local).assoc_led_active, 0);
    (*local).assoc_led.activate = Some(ieee80211_assoc_led_activate);
    (*local).assoc_led.deactivate = Some(ieee80211_assoc_led_deactivate);
    if !(*local).assoc_led.name.is_null() && led_trigger_register(&mut (*local).assoc_led) != 0 {
        kfree((*local).assoc_led.name); (*local).assoc_led.name = core::ptr::null_mut();
    }
    atomic_set(&mut (*local).radio_led_active, 0);
    (*local).radio_led.activate = Some(ieee80211_radio_led_activate);
    (*local).radio_led.deactivate = Some(ieee80211_radio_led_deactivate);
    if !(*local).radio_led.name.is_null() && led_trigger_register(&mut (*local).radio_led) != 0 {
        kfree((*local).radio_led.name); (*local).radio_led.name = core::ptr::null_mut();
    }
    atomic_set(&mut (*local).tpt_led_active, 0);
    if !(*local).tpt_led_trigger.is_null() {
        (*local).tpt_led.activate = Some(ieee80211_tpt_led_activate);
        (*local).tpt_led.deactivate = Some(ieee80211_tpt_led_deactivate);
        if led_trigger_register(&mut (*local).tpt_led) != 0 {
            kfree((*local).tpt_led_trigger); (*local).tpt_led_trigger = core::ptr::null_mut();
        }
    }
}

pub unsafe fn ieee80211_led_exit(local: *mut ieee80211_local) {
    if !(*local).radio_led.name.is_null() { led_trigger_unregister(&mut (*local).radio_led); }
    if !(*local).assoc_led.name.is_null() { led_trigger_unregister(&mut (*local).assoc_led); }
    if !(*local).tx_led.name.is_null() { led_trigger_unregister(&mut (*local).tx_led); }
    if !(*local).rx_led.name.is_null() { led_trigger_unregister(&mut (*local).rx_led); }
    if !(*local).tpt_led_trigger.is_null() {
        led_trigger_unregister(&mut (*local).tpt_led);
        kfree((*local).tpt_led_trigger);
    }
}

pub unsafe fn __ieee80211_get_radio_led_name(hw: *mut ieee80211_hw) -> *const i8 {
    let local = hw_to_local(hw); (*local).radio_led.name
}

pub unsafe fn __ieee80211_get_assoc_led_name(hw: *mut ieee80211_hw) -> *const i8 {
    let local = hw_to_local(hw); (*local).assoc_led.name
}

pub unsafe fn __ieee80211_get_tx_led_name(hw: *mut ieee80211_hw) -> *const i8 {
    let local = hw_to_local(hw); (*local).tx_led.name
}

pub unsafe fn __ieee80211_get_rx_led_name(hw: *mut ieee80211_hw) -> *const i8 {
    let local = hw_to_local(hw); (*local).rx_led.name
}

unsafe fn tpt_trig_traffic(_local: *mut ieee80211_local, tpt_trig: *mut tpt_led_trigger) -> c_ulong {
    let traffic = (*tpt_trig).tx_bytes + (*tpt_trig).rx_bytes;
    let delta = traffic - (*tpt_trig).prev_traffic;
    (*tpt_trig).prev_traffic = traffic;
    (delta + (1024 / 8) - 1) / (1024 / 8)
}

unsafe fn tpt_trig_timer(t: *mut timer_list) {
    let tpt_trig = timer_container_of(t, tpt_led_trigger, timer);
    let local = (*tpt_trig).local;
    if !(*tpt_trig).running { return; }
    mod_timer(&mut (*tpt_trig).timer, round_jiffies(jiffies() + HZ));
    let tpt = tpt_trig_traffic(local, tpt_trig);
    let mut on = 1; let mut off = 0;
    let mut i = (*tpt_trig).blink_table_len as isize - 1;
    while i >= 0 {
        let entry = &*(*tpt_trig).blink_table.offset(i);
        if entry.throughput < 0 || tpt > entry.throughput as c_ulong {
            off = entry.blink_time / 2; on = entry.blink_time - off; break;
        }
        i -= 1;
    }
    led_trigger_blink(&mut (*local).tpt_led, on, off);
}

pub unsafe fn __ieee80211_create_tpt_led_trigger(hw: *mut ieee80211_hw, flags: c_uint, blink_table: *const ieee80211_tpt_blink, blink_table_len: c_uint) -> *const i8 {
    let local = hw_to_local(hw);
    if WARN_ON(!(*local).tpt_led_trigger.is_null()) { return core::ptr::null(); }
    let tpt_trig = kzalloc_obj::<tpt_led_trigger>();
    if tpt_trig.is_null() { return core::ptr::null(); }
    snprintf((*tpt_trig).name.as_mut_ptr(), (*tpt_trig).name.len(), "%stpt", wiphy_name((*local).hw.wiphy));
    (*local).tpt_led.name = (*tpt_trig).name.as_mut_ptr();
    (*tpt_trig).blink_table = blink_table; (*tpt_trig).blink_table_len = blink_table_len;
    (*tpt_trig).want = flags; (*tpt_trig).local = local;
    timer_setup(&mut (*tpt_trig).timer, tpt_trig_timer, 0);
    (*local).tpt_led_trigger = tpt_trig;
    (*tpt_trig).name.as_ptr()
}

unsafe fn ieee80211_start_tpt_led_trig(local: *mut ieee80211_local) {
    let t = (*local).tpt_led_trigger;
    if (*t).running { return; }
    tpt_trig_traffic(local, t); (*t).running = true;
    tpt_trig_timer(&mut (*t).timer); mod_timer(&mut (*t).timer, round_jiffies(jiffies() + HZ));
}

unsafe fn ieee80211_stop_tpt_led_trig(local: *mut ieee80211_local) {
    let t = (*local).tpt_led_trigger;
    if !(*t).running { return; }
    (*t).running = false; timer_delete_sync(&mut (*t).timer);
    led_trigger_event(&mut (*local).tpt_led, LED_OFF);
}

pub unsafe fn ieee80211_mod_tpt_led_trig(local: *mut ieee80211_local, types_on: c_uint, types_off: c_uint) {
    let t = (*local).tpt_led_trigger;
    WARN_ON(types_on & types_off != 0);
    if t.is_null() { return; }
    (*t).active &= !types_off; (*t).active |= types_on;
    let allowed = (*t).active & IEEE80211_TPT_LEDTRIG_FL_RADIO != 0;
    if !allowed || (*t).active & (*t).want == 0 { ieee80211_stop_tpt_led_trig(local); }
    else { ieee80211_start_tpt_led_trig(local); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
