// SPDX-License-Identifier: GPL-2.0-only
/*
 * dice.h - a part of driver for Dice based devices
 *
 * Copyright (c) Clemens Ladisch
 * Copyright (c) 2014 Takashi Sakamoto
 */

/*
 * C header dependencies removed from executable Rust:
 * linux/compat.h, linux/completion.h, linux/delay.h, linux/device.h,
 * linux/firewire.h, linux/firewire-constants.h, linux/jiffies.h,
 * linux/module.h, linux/mutex.h, linux/slab.h, linux/spinlock.h,
 * linux/wait.h, linux/sched/signal.h, sound/control.h, sound/core.h,
 * sound/firewire.h, sound/hwdep.h, sound/info.h, sound/initval.h,
 * sound/pcm.h, sound/pcm_params.h, sound/rawmidi.h, ../amdtp-am824.h,
 * ../iso-resources.h, ../lib.h, and dice-interface.h.
 */

/*
 * This module support maximum 2 pairs of tx/rx isochronous streams for
 * our convinience.
 *
 * In documents for ASICs called with a name of 'DICE':
 *  - ASIC for DICE II:
 *   - Maximum 2 tx and 4 rx are supported.
 *   - A packet supports maximum 16 data channels.
 *  - TCD2210/2210-E (so-called 'Dice Mini'):
 *   - Maximum 2 tx and 2 rx are supported.
 *   - A packet supports maximum 16 data channels.
 *  - TCD2220/2220-E (so-called 'Dice Jr.')
 *   - 2 tx and 2 rx are supported.
 *   - A packet supports maximum 16 data channels.
 *  - TCD3070-CH (so-called 'Dice III')
 *   - Maximum 2 tx and 2 rx are supported.
 *   - A packet supports maximum 32 data channels.
 *
 * For the above, MIDI conformant data channel is just on the first isochronous
 * stream.
 */
pub const MAX_STREAMS: usize = 2;

pub const SND_DICE_RATE_MODE_LOW: snd_dice_rate_mode = 0;
pub const SND_DICE_RATE_MODE_MIDDLE: snd_dice_rate_mode = 1;
pub const SND_DICE_RATE_MODE_HIGH: snd_dice_rate_mode = 2;
pub const SND_DICE_RATE_MODE_COUNT: snd_dice_rate_mode = 3;
pub type snd_dice_rate_mode = ::std::os::raw::c_uint;

pub type snd_dice_detect_formats_t =
    Option<unsafe extern "C" fn(dice: *mut snd_dice) -> ::std::os::raw::c_int>;

#[repr(C)]
pub struct snd_dice {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,
    pub lock: spinlock_t,
    pub mutex: mutex,

    /* Offsets for sub-addresses */
    pub global_offset: ::std::os::raw::c_uint,
    pub rx_offset: ::std::os::raw::c_uint,
    pub tx_offset: ::std::os::raw::c_uint,
    pub sync_offset: ::std::os::raw::c_uint,
    pub rsrv_offset: ::std::os::raw::c_uint,

    pub clock_caps: ::std::os::raw::c_uint,
    pub tx_pcm_chs:
        [[::std::os::raw::c_uint; SND_DICE_RATE_MODE_COUNT as usize]; MAX_STREAMS],
    pub rx_pcm_chs:
        [[::std::os::raw::c_uint; SND_DICE_RATE_MODE_COUNT as usize]; MAX_STREAMS],
    pub tx_midi_ports: [::std::os::raw::c_uint; MAX_STREAMS],
    pub rx_midi_ports: [::std::os::raw::c_uint; MAX_STREAMS],

    pub notification_handler: fw_address_handler,
    pub owner_generation: ::std::os::raw::c_int,
    pub notification_bits: u32,

    /* For uapi */
    pub dev_lock_count: ::std::os::raw::c_int, /* > 0 driver, < 0 userspace */
    pub dev_lock_changed: bool,
    pub hwdep_wait: wait_queue_head_t,

    /* For streaming */
    pub tx_resources: [fw_iso_resources; MAX_STREAMS],
    pub rx_resources: [fw_iso_resources; MAX_STREAMS],
    pub tx_stream: [amdtp_stream; MAX_STREAMS],
    pub rx_stream: [amdtp_stream; MAX_STREAMS],
    /*
     * C bitfields:
     * bool global_enabled:1;
     * bool disable_double_pcm_frames:1;
     */
    pub global_enabled: bool,
    pub disable_double_pcm_frames: bool,
    pub clock_accepted: completion,
    pub substreams_counter: ::std::os::raw::c_uint,

    pub domain: amdtp_domain,
}

pub const SND_DICE_ADDR_TYPE_PRIVATE: snd_dice_addr_type = 0;
pub const SND_DICE_ADDR_TYPE_GLOBAL: snd_dice_addr_type = 1;
pub const SND_DICE_ADDR_TYPE_TX: snd_dice_addr_type = 2;
pub const SND_DICE_ADDR_TYPE_RX: snd_dice_addr_type = 3;
pub const SND_DICE_ADDR_TYPE_SYNC: snd_dice_addr_type = 4;
pub const SND_DICE_ADDR_TYPE_RSRV: snd_dice_addr_type = 5;
pub type snd_dice_addr_type = ::std::os::raw::c_uint;

unsafe extern "C" {
    pub fn snd_dice_transaction_write(
        dice: *mut snd_dice,
        type_: snd_dice_addr_type,
        offset: ::std::os::raw::c_uint,
        buf: *mut ::std::os::raw::c_void,
        len: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn snd_dice_transaction_read(
        dice: *mut snd_dice,
        type_: snd_dice_addr_type,
        offset: ::std::os::raw::c_uint,
        buf: *mut ::std::os::raw::c_void,
        len: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}

#[inline]
pub unsafe fn snd_dice_transaction_write_global(
    dice: *mut snd_dice,
    offset: ::std::os::raw::c_uint,
    buf: *mut ::std::os::raw::c_void,
    len: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { snd_dice_transaction_write(dice, SND_DICE_ADDR_TYPE_GLOBAL, offset, buf, len) }
}

#[inline]
pub unsafe fn snd_dice_transaction_read_global(
    dice: *mut snd_dice,
    offset: ::std::os::raw::c_uint,
    buf: *mut ::std::os::raw::c_void,
    len: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { snd_dice_transaction_read(dice, SND_DICE_ADDR_TYPE_GLOBAL, offset, buf, len) }
}

#[inline]
pub unsafe fn snd_dice_transaction_write_tx(
    dice: *mut snd_dice,
    offset: ::std::os::raw::c_uint,
    buf: *mut ::std::os::raw::c_void,
    len: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { snd_dice_transaction_write(dice, SND_DICE_ADDR_TYPE_TX, offset, buf, len) }
}

#[inline]
pub unsafe fn snd_dice_transaction_read_tx(
    dice: *mut snd_dice,
    offset: ::std::os::raw::c_uint,
    buf: *mut ::std::os::raw::c_void,
    len: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { snd_dice_transaction_read(dice, SND_DICE_ADDR_TYPE_TX, offset, buf, len) }
}

#[inline]
pub unsafe fn snd_dice_transaction_write_rx(
    dice: *mut snd_dice,
    offset: ::std::os::raw::c_uint,
    buf: *mut ::std::os::raw::c_void,
    len: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { snd_dice_transaction_write(dice, SND_DICE_ADDR_TYPE_RX, offset, buf, len) }
}

#[inline]
pub unsafe fn snd_dice_transaction_read_rx(
    dice: *mut snd_dice,
    offset: ::std::os::raw::c_uint,
    buf: *mut ::std::os::raw::c_void,
    len: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { snd_dice_transaction_read(dice, SND_DICE_ADDR_TYPE_RX, offset, buf, len) }
}

#[inline]
pub unsafe fn snd_dice_transaction_write_sync(
    dice: *mut snd_dice,
    offset: ::std::os::raw::c_uint,
    buf: *mut ::std::os::raw::c_void,
    len: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { snd_dice_transaction_write(dice, SND_DICE_ADDR_TYPE_SYNC, offset, buf, len) }
}

#[inline]
pub unsafe fn snd_dice_transaction_read_sync(
    dice: *mut snd_dice,
    offset: ::std::os::raw::c_uint,
    buf: *mut ::std::os::raw::c_void,
    len: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { snd_dice_transaction_read(dice, SND_DICE_ADDR_TYPE_SYNC, offset, buf, len) }
}

unsafe extern "C" {
    pub fn snd_dice_transaction_get_clock_source(
        dice: *mut snd_dice,
        source: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn snd_dice_transaction_get_rate(
        dice: *mut snd_dice,
        rate: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn snd_dice_transaction_set_enable(dice: *mut snd_dice) -> ::std::os::raw::c_int;
    pub fn snd_dice_transaction_clear_enable(dice: *mut snd_dice);
    pub fn snd_dice_transaction_init(dice: *mut snd_dice) -> ::std::os::raw::c_int;
    pub fn snd_dice_transaction_reinit(dice: *mut snd_dice) -> ::std::os::raw::c_int;
    pub fn snd_dice_transaction_destroy(dice: *mut snd_dice);
}

pub const SND_DICE_RATES_COUNT: usize = 7;

unsafe extern "C" {
    pub static snd_dice_rates: [::std::os::raw::c_uint; SND_DICE_RATES_COUNT];

    pub fn snd_dice_stream_get_rate_mode(
        dice: *mut snd_dice,
        rate: ::std::os::raw::c_uint,
        mode: *mut snd_dice_rate_mode,
    ) -> ::std::os::raw::c_int;
    pub fn snd_dice_stream_start_duplex(dice: *mut snd_dice) -> ::std::os::raw::c_int;
    pub fn snd_dice_stream_stop_duplex(dice: *mut snd_dice);
    pub fn snd_dice_stream_init_duplex(dice: *mut snd_dice) -> ::std::os::raw::c_int;
    pub fn snd_dice_stream_destroy_duplex(dice: *mut snd_dice);
    pub fn snd_dice_stream_reserve_duplex(
        dice: *mut snd_dice,
        rate: ::std::os::raw::c_uint,
        events_per_period: ::std::os::raw::c_uint,
        events_per_buffer: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn snd_dice_stream_update_duplex(dice: *mut snd_dice);
    pub fn snd_dice_stream_detect_current_formats(dice: *mut snd_dice) -> ::std::os::raw::c_int;

    pub fn snd_dice_stream_lock_try(dice: *mut snd_dice) -> ::std::os::raw::c_int;
    pub fn snd_dice_stream_lock_release(dice: *mut snd_dice);

    pub fn snd_dice_create_pcm(dice: *mut snd_dice) -> ::std::os::raw::c_int;

    pub fn snd_dice_create_hwdep(dice: *mut snd_dice) -> ::std::os::raw::c_int;

    pub fn snd_dice_create_proc(dice: *mut snd_dice);

    pub fn snd_dice_create_midi(dice: *mut snd_dice) -> ::std::os::raw::c_int;

    pub fn snd_dice_detect_tcelectronic_formats(dice: *mut snd_dice) -> ::std::os::raw::c_int;
    pub fn snd_dice_detect_alesis_formats(dice: *mut snd_dice) -> ::std::os::raw::c_int;
    pub fn snd_dice_detect_alesis_mastercontrol_formats(
        dice: *mut snd_dice,
    ) -> ::std::os::raw::c_int;
    pub fn snd_dice_detect_extension_formats(dice: *mut snd_dice) -> ::std::os::raw::c_int;
    pub fn snd_dice_detect_mytek_formats(dice: *mut snd_dice) -> ::std::os::raw::c_int;
    pub fn snd_dice_detect_presonus_formats(dice: *mut snd_dice) -> ::std::os::raw::c_int;
    pub fn snd_dice_detect_harman_formats(dice: *mut snd_dice) -> ::std::os::raw::c_int;
    pub fn snd_dice_detect_focusrite_pro40_tcd3070_formats(
        dice: *mut snd_dice,
    ) -> ::std::os::raw::c_int;
    pub fn snd_dice_detect_weiss_formats(dice: *mut snd_dice) -> ::std::os::raw::c_int;
    pub fn snd_dice_detect_teac_formats(dice: *mut snd_dice) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
