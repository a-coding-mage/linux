/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * soundbus generic definitions
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 */

use core::ffi::c_void;
use std::os::raw::{c_char, c_int};

/* Dependencies from the original C header:
 * <linux/platform_device.h>, <sound/pcm.h>, and <linux/list.h>.
 * The following external C types are expected to be supplied by the
 * surrounding translated repository bindings.
 */

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}

pub type pm_message_t = c_int;

/* When switching from master to slave or the other way around,
 * you don't want to have the codec chip acting as clock source
 * while the bus still is.
 * More importantly, while switch from slave to master, you need
 * to turn off the chip's master function first, but then there's
 * no clock for a while and other chips might reset, so we notify
 * their drivers after having switched.
 * The constants here are codec-point of view, so when we switch
 * the soundbus to master we tell the codec we're going to switch
 * and give it CLOCK_SWITCH_PREPARE_SLAVE!
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum clock_switch {
    CLOCK_SWITCH_PREPARE_SLAVE = 0,
    CLOCK_SWITCH_PREPARE_MASTER = 1,
    CLOCK_SWITCH_SLAVE = 2,
    CLOCK_SWITCH_MASTER = 3,
    CLOCK_SWITCH_NOTIFY = 4,
}

/* information on a transfer the codec can take */
#[repr(C)]
pub struct transfer_info {
    pub formats: u64,        /* SNDRV_PCM_FMTBIT_* */
    pub rates: u32,          /* SNDRV_PCM_RATE_* */
    /* flags */
    pub flags: u32,
    /* for codecs to distinguish among their TIs */
    pub tag: c_int,
}

pub const TRANSFER_INFO_TRANSFER_IN: u32 = 1 << 0; /* input = 1, output = 0 */
pub const TRANSFER_INFO_MUST_BE_CLOCK_SOURCE: u32 = 1 << 1;

#[repr(C)]
pub struct codec_info_item {
    pub codec: *mut codec_info,
    pub codec_data: *mut c_void,
    pub sdev: *mut soundbus_dev,
    /* internal, to be used by the soundbus provider */
    pub list: list_head,
}

/* for prepare, where the codecs need to know
 * what we're going to drive the bus with
 */
#[repr(C)]
pub struct bus_info {
    /* see below */
    pub sysclock_factor: c_int,
    pub bus_factor: c_int,
}

/* information on the codec itself, plus function pointers */
#[repr(C)]
pub struct codec_info {
    /* the module this lives in */
    pub owner: *mut module,

    /* supported transfer possibilities, array terminated by
     * formats or rates being 0.
     */
    pub transfers: *mut transfer_info,

    /* Master clock speed factor
     * to be used (master clock speed = sysclock_factor * sampling freq)
     * Unused if the soundbus provider has no such notion.
     */
    pub sysclock_factor: c_int,

    /* Bus factor, bus clock speed = bus_factor * sampling freq)
     * Unused if the soundbus provider has no such notion.
     */
    pub bus_factor: c_int,

    /* operations */
    /* clock switching, see above */
    pub switch_clock:
        Option<unsafe extern "C" fn(cii: *mut codec_info_item, clock: clock_switch) -> c_int>,

    /* called for each transfer_info when the user
     * opens the pcm device to determine what the
     * hardware can support at this point in time.
     * That can depend on other user-switchable controls.
     * Return 1 if usable, 0 if not.
     * out points to another instance of a transfer_info
     * which is initialised to the values in *ti, and
     * it's format and rate values can be modified by
     * the callback if it is necessary to further restrict
     * the formats that can be used at the moment, for
     * example when one codec has multiple logical codec
     * info structs for multiple inputs.
     */
    pub usable: Option<
        unsafe extern "C" fn(
            cii: *mut codec_info_item,
            ti: *mut transfer_info,
            out: *mut transfer_info,
        ) -> c_int,
    >,

    /* called when pcm stream is opened, probably not implemented
     * most of the time since it isn't too useful
     */
    pub open: Option<
        unsafe extern "C" fn(
            cii: *mut codec_info_item,
            substream: *mut snd_pcm_substream,
        ) -> c_int,
    >,

    /* called when the pcm stream is closed, at this point
     * the user choices can all be unlocked (see below)
     */
    pub close: Option<
        unsafe extern "C" fn(
            cii: *mut codec_info_item,
            substream: *mut snd_pcm_substream,
        ) -> c_int,
    >,

    /* if the codec must forbid some user choices because
     * they are not valid with the substream/transfer info,
     * it must do so here. Example: no digital output for
     * incompatible framerate, say 8KHz, on Onyx.
     * If the selected stuff in the substream is NOT
     * compatible, you have to reject this call!
     */
    pub prepare: Option<
        unsafe extern "C" fn(
            cii: *mut codec_info_item,
            bi: *mut bus_info,
            substream: *mut snd_pcm_substream,
        ) -> c_int,
    >,

    /* start() is called before data is pushed to the codec.
     * Note that start() must be atomic!
     */
    pub start: Option<
        unsafe extern "C" fn(
            cii: *mut codec_info_item,
            substream: *mut snd_pcm_substream,
        ) -> c_int,
    >,

    /* stop() is called after data is no longer pushed to the codec.
     * Note that stop() must be atomic!
     */
    pub stop: Option<
        unsafe extern "C" fn(
            cii: *mut codec_info_item,
            substream: *mut snd_pcm_substream,
        ) -> c_int,
    >,

    pub suspend: Option<unsafe extern "C" fn(cii: *mut codec_info_item, state: pm_message_t) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(cii: *mut codec_info_item) -> c_int>,
}

/* information on a soundbus device */
#[repr(C)]
pub struct soundbus_dev {
    /* the bus it belongs to */
    pub onbuslist: list_head,

    /* the of device it represents */
    pub ofdev: platform_device,

    /* what modules go by */
    pub modalias: [c_char; 32],

    /* These fields must be before attach_codec can be called.
     * They should be set by the owner of the alsa card object
     * that is needed, and whoever sets them must make sure
     * that they are unique within that alsa card object.
     */
    pub pcmname: *mut c_char,
    pub pcmid: c_int,

    /* this is assigned by the soundbus provider in attach_codec */
    pub pcm: *mut snd_pcm,

    /* operations */
    /* attach a codec to this soundbus, give the alsa
     * card object the PCMs for this soundbus should be in.
     * The 'data' pointer must be unique, it is used as the
     * key for detach_codec().
     */
    pub attach_codec: Option<
        unsafe extern "C" fn(
            dev: *mut soundbus_dev,
            card: *mut snd_card,
            ci: *mut codec_info,
            data: *mut c_void,
        ) -> c_int,
    >,
    pub detach_codec: Option<unsafe extern "C" fn(dev: *mut soundbus_dev, data: *mut c_void)>,
    /* TODO: suspend/resume */

    /* private for the soundbus provider */
    pub codec_list: list_head,
    pub flags: u32,
}

pub const SOUNDBUS_DEV_HAVE_OUT: u32 = 1 << 0;
pub const SOUNDBUS_DEV_HAVE_IN: u32 = 1 << 1;

/* #define to_soundbus_device(d) container_of(d, struct soundbus_dev, ofdev.dev)
 * #define of_to_soundbus_device(d) container_of(d, struct soundbus_dev, ofdev)
 */

unsafe extern "C" {
    pub fn soundbus_add_one(dev: *mut soundbus_dev) -> c_int;
    pub fn soundbus_remove_one(dev: *mut soundbus_dev);

    pub fn soundbus_dev_get(dev: *mut soundbus_dev) -> *mut soundbus_dev;
    pub fn soundbus_dev_put(dev: *mut soundbus_dev);
}

#[repr(C)]
pub struct soundbus_driver {
    pub name: *mut c_char,
    pub owner: *mut module,

    /* we don't implement any matching at all */

    pub probe: Option<unsafe extern "C" fn(dev: *mut soundbus_dev) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(dev: *mut soundbus_dev)>,

    pub shutdown: Option<unsafe extern "C" fn(dev: *mut soundbus_dev) -> c_int>,

    pub driver: device_driver,
}

/* #define to_soundbus_driver(drv) container_of(drv,struct soundbus_driver, driver) */

unsafe extern "C" {
    pub fn soundbus_register_driver(drv: *mut soundbus_driver) -> c_int;
    pub fn soundbus_unregister_driver(drv: *mut soundbus_driver);

    pub static mut soundbus_dev_attrs: *mut *mut attribute;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
