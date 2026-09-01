// SPDX-License-Identifier: GPL-2.0-only
/*
 * Apple Onboard Audio driver for Toonie codec
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 *
 * This is a driver for the toonie codec chip. This chip is present
 * on the Mac Mini and is nothing but a DAC.
 */

// MODULE_AUTHOR("Johannes Berg <johannes@sipsolutions.net>");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("toonie codec driver for snd-aoa");

const PFX: &[u8] = b"snd-aoa-codec-toonie: \0";

const SNDRV_PCM_FMTBIT_S16_BE: u32 = 1 << 0;
const SNDRV_PCM_FMTBIT_S24_BE: u32 = 1 << 1;
const SNDRV_PCM_RATE_32000: u32 = 1 << 0;
const SNDRV_PCM_RATE_44100: u32 = 1 << 1;
const SNDRV_PCM_RATE_48000: u32 = 1 << 2;
const SNDRV_PCM_RATE_88200: u32 = 1 << 3;
const SNDRV_PCM_RATE_96000: u32 = 1 << 4;
const SNDRV_DEV_CODEC: i32 = 0;
const ENOTCONN: i32 = 107;
const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;

#[repr(C)]
pub struct snd_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct codec_info_item {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soundbus_dev {
    pub attach_codec: unsafe extern "C" fn(
        dev: *mut soundbus_dev,
        card: *mut snd_card,
        info: *mut codec_info,
        data: *mut toonie,
    ) -> i32,
    pub detach_codec: unsafe extern "C" fn(dev: *mut soundbus_dev, data: *mut toonie),
}

#[repr(C)]
pub struct aoa_codec {
    pub name: [u8; 32],
    pub owner: *mut module,
    pub init: Option<unsafe extern "C" fn(codec: *mut aoa_codec) -> i32>,
    pub exit: Option<unsafe extern "C" fn(codec: *mut aoa_codec)>,
    pub connected: i32,
    pub soundbus_dev: *mut soundbus_dev,
}

#[repr(C)]
pub struct toonie {
    pub codec: aoa_codec,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_register: Option<unsafe extern "C" fn(dev: *mut snd_device) -> i32>,
}

#[repr(C)]
pub struct transfer_info {
    pub formats: u32,
    pub rates: u32,
}

#[repr(C)]
pub struct codec_info {
    pub transfers: *mut transfer_info,
    pub sysclock_factor: i32,
    pub bus_factor: i32,
    pub owner: *mut module,
    pub usable: Option<
        unsafe extern "C" fn(
            cii: *mut codec_info_item,
            ti: *mut transfer_info,
            out: *mut transfer_info,
        ) -> i32,
    >,
    // Present in C only when CONFIG_PM is enabled.
    pub suspend: Option<unsafe extern "C" fn(cii: *mut codec_info_item, state: pm_message_t) -> i32>,
    pub resume: Option<unsafe extern "C" fn(cii: *mut codec_info_item) -> i32>,
}

#[allow(non_camel_case_types)]
pub type pm_message_t = i32;

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;

    fn aoa_snd_device_new(
        kind: i32,
        device_data: *mut toonie,
        ops: *const snd_device_ops,
    ) -> i32;
    fn aoa_get_card() -> *mut snd_card;
    fn snd_device_free(card: *mut snd_card, device_data: *mut toonie);
    fn aoa_codec_register(codec: *mut aoa_codec) -> i32;
    fn aoa_codec_unregister(codec: *mut aoa_codec);
    fn kfree(ptr: *mut toonie);
    fn kzalloc_obj_toonie() -> *mut toonie;
    fn strscpy(dest: *mut u8, src: *const u8) -> isize;
    fn printk(fmt: *const u8, ...);
}

#[inline]
unsafe fn codec_to_toonie(c: *mut aoa_codec) -> *mut toonie {
    c as *mut toonie
}

unsafe extern "C" fn toonie_dev_register(_dev: *mut snd_device) -> i32 {
    0
}

static OPS: snd_device_ops = snd_device_ops {
    dev_register: Some(toonie_dev_register),
};

static mut TOONIE_TRANSFERS: [transfer_info; 2] = [
    /*
     * This thing *only* has analog output,
     * the rates are taken from Info.plist
     * from Darwin.
     */
    transfer_info {
        formats: SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S24_BE,
        rates: SNDRV_PCM_RATE_32000
            | SNDRV_PCM_RATE_44100
            | SNDRV_PCM_RATE_48000
            | SNDRV_PCM_RATE_88200
            | SNDRV_PCM_RATE_96000,
    },
    transfer_info {
        formats: 0,
        rates: 0,
    },
];

unsafe extern "C" fn toonie_usable(
    _cii: *mut codec_info_item,
    _ti: *mut transfer_info,
    _out: *mut transfer_info,
) -> i32 {
    1
}

// CONFIG_PM: suspend and resume hooks are compiled in only when enabled.
unsafe extern "C" fn toonie_suspend(_cii: *mut codec_info_item, _state: pm_message_t) -> i32 {
    /* can we turn it off somehow? */
    0
}

unsafe extern "C" fn toonie_resume(_cii: *mut codec_info_item) -> i32 {
    0
}

static mut TOONIE_CODEC_INFO: codec_info = codec_info {
    transfers: unsafe { TOONIE_TRANSFERS.as_mut_ptr() },
    sysclock_factor: 256,
    bus_factor: 64,
    owner: unsafe { THIS_MODULE },
    usable: Some(toonie_usable),
    // CONFIG_PM fields.
    suspend: Some(toonie_suspend),
    resume: Some(toonie_resume),
};

unsafe extern "C" fn toonie_init_codec(codec: *mut aoa_codec) -> i32 {
    let toonie = unsafe { codec_to_toonie(codec) };

    /* nothing connected? what a joke! */
    if unsafe { (*toonie).codec.connected } != 1 {
        return -ENOTCONN;
    }

    if unsafe { aoa_snd_device_new(SNDRV_DEV_CODEC, toonie, &OPS) } != 0 {
        unsafe {
            printk(
                b"\x013snd-aoa-codec-toonie: failed to create toonie snd device!\n\0".as_ptr(),
            );
        }
        return -ENODEV;
    }

    if unsafe {
        ((*(*toonie).codec.soundbus_dev).attach_codec)(
            (*toonie).codec.soundbus_dev,
            aoa_get_card(),
            &raw mut TOONIE_CODEC_INFO,
            toonie,
        )
    } != 0
    {
        unsafe {
            printk(b"\x013snd-aoa-codec-toonie: error creating toonie pcm\n\0".as_ptr());
            snd_device_free(aoa_get_card(), toonie);
        }
        return -ENODEV;
    }

    0
}

unsafe extern "C" fn toonie_exit_codec(codec: *mut aoa_codec) {
    let toonie = unsafe { codec_to_toonie(codec) };

    if unsafe { (*toonie).codec.soundbus_dev.is_null() } {
        unsafe {
            printk(
                b"\x013snd-aoa-codec-toonie: toonie_exit_codec called without soundbus_dev!\n\0"
                    .as_ptr(),
            );
        }
        return;
    }
    unsafe {
        ((*(*toonie).codec.soundbus_dev).detach_codec)((*toonie).codec.soundbus_dev, toonie);
    }
}

static mut TOONIE: *mut toonie = core::ptr::null_mut();

unsafe extern "C" fn toonie_init() -> i32 {
    unsafe {
        TOONIE = kzalloc_obj_toonie();
    }

    if unsafe { TOONIE.is_null() } {
        return -ENOMEM;
    }

    unsafe {
        strscpy((*TOONIE).codec.name.as_mut_ptr(), b"toonie\0".as_ptr());
        (*TOONIE).codec.owner = THIS_MODULE;
        (*TOONIE).codec.init = Some(toonie_init_codec);
        (*TOONIE).codec.exit = Some(toonie_exit_codec);
    }

    if unsafe { aoa_codec_register(&raw mut (*TOONIE).codec) } != 0 {
        unsafe {
            kfree(TOONIE);
        }
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn toonie_exit() {
    unsafe {
        aoa_codec_unregister(&raw mut (*TOONIE).codec);
        kfree(TOONIE);
    }
}

// module_init(toonie_init);
// module_exit(toonie_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
