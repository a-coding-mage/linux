/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2006, Johannes Berg <johannes@sipsolutions.net>
 */

// Dependencies supplied by the surrounding translation unit:
// linux/list.h, linux/spinlock.h, linux/leds.h, ieee80211_i.h

pub const MAC80211_BLINK_DELAY: u64 = 50; /* ms */

// CONFIG_MAC80211_LEDS is a build-time C condition; represented here by the
// corresponding Rust cfg feature.
#[inline]
pub unsafe fn ieee80211_led_rx(local: *mut ieee80211_local) {
    #[cfg(feature = "CONFIG_MAC80211_LEDS")]
    {
        if !atomic_read(&(*local).rx_led_active) {
            return;
        }
        led_trigger_blink_oneshot(
            &mut (*local).rx_led,
            MAC80211_BLINK_DELAY,
            MAC80211_BLINK_DELAY,
            0,
        );
    }
}

#[inline]
pub unsafe fn ieee80211_led_tx(local: *mut ieee80211_local) {
    #[cfg(feature = "CONFIG_MAC80211_LEDS")]
    {
        if !atomic_read(&(*local).tx_led_active) {
            return;
        }
        led_trigger_blink_oneshot(
            &mut (*local).tx_led,
            MAC80211_BLINK_DELAY,
            MAC80211_BLINK_DELAY,
            0,
        );
    }
}

#[cfg(feature = "CONFIG_MAC80211_LEDS")]
extern "C" {
    pub fn ieee80211_led_assoc(local: *mut ieee80211_local, associated: bool);
    pub fn ieee80211_led_radio(local: *mut ieee80211_local, enabled: bool);
    pub fn ieee80211_alloc_led_names(local: *mut ieee80211_local);
    pub fn ieee80211_free_led_names(local: *mut ieee80211_local);
    pub fn ieee80211_led_init(local: *mut ieee80211_local);
    pub fn ieee80211_led_exit(local: *mut ieee80211_local);
    pub fn ieee80211_mod_tpt_led_trig(
        local: *mut ieee80211_local,
        types_on: u32,
        types_off: u32,
    );
}

#[cfg(not(feature = "CONFIG_MAC80211_LEDS"))]
#[inline]
pub unsafe fn ieee80211_led_assoc(_local: *mut ieee80211_local, _associated: bool) {}

#[cfg(not(feature = "CONFIG_MAC80211_LEDS"))]
#[inline]
pub unsafe fn ieee80211_led_radio(_local: *mut ieee80211_local, _enabled: bool) {}

#[cfg(not(feature = "CONFIG_MAC80211_LEDS"))]
#[inline]
pub unsafe fn ieee80211_alloc_led_names(_local: *mut ieee80211_local) {}

#[cfg(not(feature = "CONFIG_MAC80211_LEDS"))]
#[inline]
pub unsafe fn ieee80211_free_led_names(_local: *mut ieee80211_local) {}

#[cfg(not(feature = "CONFIG_MAC80211_LEDS"))]
#[inline]
pub unsafe fn ieee80211_led_init(_local: *mut ieee80211_local) {}

#[cfg(not(feature = "CONFIG_MAC80211_LEDS"))]
#[inline]
pub unsafe fn ieee80211_led_exit(_local: *mut ieee80211_local) {}

#[cfg(not(feature = "CONFIG_MAC80211_LEDS"))]
#[inline]
pub unsafe fn ieee80211_mod_tpt_led_trig(
    _local: *mut ieee80211_local,
    _types_on: u32,
    _types_off: u32,
) {
}

#[inline]
pub unsafe fn ieee80211_tpt_led_trig_tx(local: *mut ieee80211_local, bytes: i32) {
    #[cfg(feature = "CONFIG_MAC80211_LEDS")]
    {
        if atomic_read(&(*local).tpt_led_active) {
            (*(*local).tpt_led_trigger).tx_bytes =
                (*(*local).tpt_led_trigger).tx_bytes.wrapping_add(bytes);
        }
    }
}

#[inline]
pub unsafe fn ieee80211_tpt_led_trig_rx(local: *mut ieee80211_local, bytes: i32) {
    #[cfg(feature = "CONFIG_MAC80211_LEDS")]
    {
        if atomic_read(&(*local).tpt_led_active) {
            (*(*local).tpt_led_trigger).rx_bytes =
                (*(*local).tpt_led_trigger).rx_bytes.wrapping_add(bytes);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
