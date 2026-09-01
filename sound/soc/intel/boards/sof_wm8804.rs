// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2018-2020, Intel Corporation
//
// sof-wm8804.c - ASoC machine driver for Up and Up2 board
// based on WM8804/Hifiberry Digi+

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *mut c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub id: c_int,
    pub no_pcm: c_uint,
    pub ops: *const snd_soc_ops,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub id: *const c_char,
}

#[repr(C)]
pub struct acpi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dmi_system_id {
    pub callback: Option<unsafe extern "C" fn(*const dmi_system_id) -> c_int>,
    pub matches: [dmi_strmatch; 4],
    pub driver_data: *mut c_void,
}

#[repr(C)]
pub struct dmi_strmatch {
    pub slot: c_int,
    pub substr: *const c_char,
}

#[repr(C)]
pub struct gpiod_lookup {
    pub key: *const c_char,
    pub chip_hwnum: c_uint,
    pub con_id: *const c_char,
    pub idx: c_uint,
    pub flags: c_uint,
}

#[repr(C)]
pub struct gpiod_lookup_table {
    pub dev_id: *const c_char,
    pub table: [gpiod_lookup; 3],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
struct sof_card_private {
    gpio_44: *mut gpio_desc,
    gpio_48: *mut gpio_desc,
    sample_rate: c_int,
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_int = 0;
const GPIO_ACTIVE_HIGH: c_uint = 0;
const DMI_SYS_VENDOR: c_int = 0;
const DMI_PRODUCT_NAME: c_int = 1;
const SND_ACPI_I2C_ID_LEN: usize = 16;
const SND_SOC_CLOCK_OUT: c_int = 0;
const WM8804_MCLKDIV_256FS: c_int = 0;
const WM8804_MCLKDIV_128FS: c_int = 1;
const WM8804_MCLK_DIV: c_int = 0;
const WM8804_TX_CLKSRC_PLL: c_int = 0;
const WM8804_SPDTX4: c_uint = 0;
const SOF_WM8804_UP2_QUIRK: c_ulong = 1 << 0;

unsafe extern "C" {
    static THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn IS_ERR_OR_NULL(ptr: *const c_void) -> bool;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn snd_soc_dai_set_clkdiv(dai: *mut snd_soc_dai, div_id: c_int, div: c_int) -> c_int;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dmi_check_system(list: *const dmi_system_id) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn gpiod_add_lookup_table(table: *mut gpiod_lookup_table);
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn acpi_dev_get_first_match_dev(
        hid: *const c_char,
        uid: *const c_char,
        hrv: c_long,
    ) -> *mut acpi_device;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn acpi_dev_name(adev: *mut acpi_device) -> *const c_char;
    fn acpi_dev_put(adev: *mut acpi_device);
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn gpiod_remove_lookup_table(table: *mut gpiod_lookup_table);
}

static mut sof_wm8804_quirk: c_ulong = 0;

unsafe extern "C" fn sof_wm8804_quirk_cb(id: *const dmi_system_id) -> c_int {
    sof_wm8804_quirk = (*id).driver_data as c_ulong;
    1
}

const fn DMI_MATCH(slot: c_int, substr: *const c_char) -> dmi_strmatch {
    dmi_strmatch { slot, substr }
}

static mut sof_wm8804_quirk_table: [dmi_system_id; 2] = [
    dmi_system_id {
        callback: Some(sof_wm8804_quirk_cb),
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, c"AAEON".as_ptr()),
            DMI_MATCH(DMI_PRODUCT_NAME, c"UP-APL01".as_ptr()),
            DMI_MATCH(0, ptr::null()),
            DMI_MATCH(0, ptr::null()),
        ],
        driver_data: SOF_WM8804_UP2_QUIRK as *mut c_void,
    },
    dmi_system_id {
        callback: None,
        matches: [
            DMI_MATCH(0, ptr::null()),
            DMI_MATCH(0, ptr::null()),
            DMI_MATCH(0, ptr::null()),
            DMI_MATCH(0, ptr::null()),
        ],
        driver_data: ptr::null_mut(),
    },
];

unsafe extern "C" fn sof_wm8804_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let ctx = snd_soc_card_get_drvdata((*rtd).card) as *mut sof_card_private;
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let codec = (*codec_dai).component;
    let sysclk: c_int = 27000000; /* This is fixed on this board */
    let samplerate: c_int;
    let mclk_freq: c_long;
    let mclk_div: c_int;
    let sampling_freq: c_int;
    let clk_44: bool;
    let mut ret: c_int;

    samplerate = params_rate(params);
    if samplerate == (*ctx).sample_rate {
        return 0;
    }

    (*ctx).sample_rate = 0;

    if samplerate <= 96000 {
        mclk_freq = (samplerate * 256) as c_long;
        mclk_div = WM8804_MCLKDIV_256FS;
    } else {
        mclk_freq = (samplerate * 128) as c_long;
        mclk_div = WM8804_MCLKDIV_128FS;
    }

    match samplerate {
        32000 => {
            sampling_freq = 0x03;
        }
        44100 => {
            sampling_freq = 0x00;
        }
        48000 => {
            sampling_freq = 0x02;
        }
        88200 => {
            sampling_freq = 0x08;
        }
        96000 => {
            sampling_freq = 0x0a;
        }
        176400 => {
            sampling_freq = 0x0c;
        }
        192000 => {
            sampling_freq = 0x0e;
        }
        _ => {
            dev_err(
                (*(*rtd).card).dev,
                c"unsupported samplerate %d\n".as_ptr(),
                samplerate,
            );
            return -EINVAL;
        }
    }

    if samplerate % 16000 != 0 {
        clk_44 = true; /* use 44.1 kHz root frequency */
    } else {
        clk_44 = false;
    }

    if !(IS_ERR_OR_NULL((*ctx).gpio_44 as *const c_void)
        || IS_ERR_OR_NULL((*ctx).gpio_48 as *const c_void))
    {
        /*
         * ensure both GPIOs are LOW first, then drive the
         * relevant one to HIGH
         */
        if clk_44 {
            gpiod_set_value_cansleep((*ctx).gpio_48, (!clk_44) as c_int);
            gpiod_set_value_cansleep((*ctx).gpio_44, clk_44 as c_int);
        } else {
            gpiod_set_value_cansleep((*ctx).gpio_44, clk_44 as c_int);
            gpiod_set_value_cansleep((*ctx).gpio_48, (!clk_44) as c_int);
        }
    }

    snd_soc_dai_set_clkdiv(codec_dai, WM8804_MCLK_DIV, mclk_div);
    ret = snd_soc_dai_set_pll(codec_dai, 0, 0, sysclk as c_uint, mclk_freq as c_uint);
    if ret < 0 {
        dev_err((*(*rtd).card).dev, c"Failed to set WM8804 PLL\n".as_ptr());
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(
        codec_dai,
        WM8804_TX_CLKSRC_PLL,
        sysclk as c_uint,
        SND_SOC_CLOCK_OUT,
    );
    if ret < 0 {
        dev_err(
            (*(*rtd).card).dev,
            c"Failed to set WM8804 SYSCLK: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    /* set sampling frequency status bits */
    snd_soc_component_update_bits(codec, WM8804_SPDTX4, 0x0f, sampling_freq as c_uint);

    (*ctx).sample_rate = samplerate;

    0
}

/* machine stream operations */
static sof_wm8804_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(sof_wm8804_hw_params),
};

static mut ssp5_pin: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"SSP5 Pin".as_ptr() as *mut c_char,
    dai_name: ptr::null(),
}];

static mut ssp5_codec: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"i2c-1AEC8804:00".as_ptr() as *mut c_char,
    dai_name: c"wm8804-spdif".as_ptr(),
}];

static mut platform: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"0000:00:0e.0".as_ptr() as *mut c_char,
    dai_name: ptr::null(),
}];

static mut dailink: [snd_soc_dai_link; 1] = [
    /* back ends */
    snd_soc_dai_link {
        name: c"SSP5-Codec".as_ptr(),
        id: 0,
        no_pcm: 1,
        ops: &sof_wm8804_ops,
        cpus: unsafe { ssp5_pin.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { ssp5_codec.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { platform.as_mut_ptr() },
        num_platforms: 1,
    },
];

/* SoC card */
static mut sof_wm8804_card: snd_soc_card = snd_soc_card {
    name: c"wm8804".as_ptr(), /* sof- prefix added automatically */
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { dailink.as_mut_ptr() },
    num_links: 1,
    dev: ptr::null_mut(),
};

/* i2c-<HID>:00 with HID being 8 chars */
static mut codec_name: [c_char; SND_ACPI_I2C_ID_LEN] = [0; SND_ACPI_I2C_ID_LEN];

/*
 * to control the HifiBerry Digi+ PRO, it's required to toggle GPIO to
 * select the clock source. On the Up2 board, this means
 * Pin29/BCM5/Linux GPIO 430 and Pin 31/BCM6/ Linux GPIO 404.
 *
 * Using the ACPI device name is not very nice, but since we only use
 * the value for the Up2 board there is no risk of conflict with other
 * platforms.
 */

static mut up2_gpios_table: gpiod_lookup_table = gpiod_lookup_table {
    /* .dev_id is set during probe */
    dev_id: ptr::null(),
    table: [
        gpiod_lookup {
            key: c"INT3452:01".as_ptr(),
            chip_hwnum: 73,
            con_id: c"BCM-GPIO5".as_ptr(),
            idx: 0,
            flags: GPIO_ACTIVE_HIGH,
        },
        gpiod_lookup {
            key: c"INT3452:01".as_ptr(),
            chip_hwnum: 74,
            con_id: c"BCM-GPIO6".as_ptr(),
            idx: 0,
            flags: GPIO_ACTIVE_HIGH,
        },
        gpiod_lookup {
            key: ptr::null(),
            chip_hwnum: 0,
            con_id: ptr::null(),
            idx: 0,
            flags: 0,
        },
    ],
};

unsafe extern "C" fn sof_wm8804_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card;
    let mach: *mut snd_soc_acpi_mach;
    let ctx: *mut sof_card_private;
    let adev: *mut acpi_device;
    let mut dai_index: c_int = 0;
    let mut ret: c_int;
    let mut i: c_int;

    ctx = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<sof_card_private>(),
        GFP_KERNEL,
    ) as *mut sof_card_private;
    if ctx.is_null() {
        return -ENOMEM;
    }

    mach = (*pdev).dev.platform_data as *mut snd_soc_acpi_mach;
    card = &raw mut sof_wm8804_card;
    (*card).dev = &mut (*pdev).dev;

    dmi_check_system((&raw const sof_wm8804_quirk_table) as *const dmi_system_id);

    if sof_wm8804_quirk & SOF_WM8804_UP2_QUIRK != 0 {
        up2_gpios_table.dev_id = dev_name(&mut (*pdev).dev);
        gpiod_add_lookup_table(&raw mut up2_gpios_table);

        /*
         * The gpios are required for specific boards with
         * local oscillators, and optional in other cases.
         * Since we can't identify when they are needed, use
         * the GPIO as non-optional
         */

        (*ctx).gpio_44 = devm_gpiod_get(&mut (*pdev).dev, c"BCM-GPIO5".as_ptr(), GPIOD_OUT_LOW);
        if IS_ERR((*ctx).gpio_44 as *const c_void) {
            ret = PTR_ERR((*ctx).gpio_44 as *const c_void);
            dev_err(
                &mut (*pdev).dev,
                c"could not get BCM-GPIO5: %d\n".as_ptr(),
                ret,
            );
            return ret;
        }

        (*ctx).gpio_48 = devm_gpiod_get(&mut (*pdev).dev, c"BCM-GPIO6".as_ptr(), GPIOD_OUT_LOW);
        if IS_ERR((*ctx).gpio_48 as *const c_void) {
            ret = PTR_ERR((*ctx).gpio_48 as *const c_void);
            dev_err(
                &mut (*pdev).dev,
                c"could not get BCM-GPIO6: %d\n".as_ptr(),
                ret,
            );
            return ret;
        }
    }

    /* fix index of codec dai */
    i = 0;
    while i < dailink.len() as c_int {
        if strcmp(
            (*dailink[i as usize].codecs).name,
            c"i2c-1AEC8804:00".as_ptr(),
        ) == 0
        {
            dai_index = i;
            break;
        }
        i += 1;
    }

    /* fixup codec name based on HID */
    adev = acpi_dev_get_first_match_dev((*mach).id, ptr::null(), -1);
    if !adev.is_null() {
        snprintf(
            codec_name.as_mut_ptr(),
            codec_name.len(),
            c"%s%s".as_ptr(),
            c"i2c-".as_ptr(),
            acpi_dev_name(adev),
        );
        (*dailink[dai_index as usize].codecs).name = codec_name.as_mut_ptr();
    } else {
        dev_err(
            &mut (*pdev).dev,
            c"Error cannot find '%s' dev\n".as_ptr(),
            (*mach).id,
        );
        return -ENOENT;
    }

    acpi_dev_put(adev);

    snd_soc_card_set_drvdata(card, ctx as *mut c_void);

    devm_snd_soc_register_card(&mut (*pdev).dev, card)
}

unsafe extern "C" fn sof_wm8804_remove(_pdev: *mut platform_device) {
    if sof_wm8804_quirk & SOF_WM8804_UP2_QUIRK != 0 {
        gpiod_remove_lookup_table(&raw mut up2_gpios_table);
    }
}

static mut sof_wm8804_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: c"sof-wm8804".as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    probe: Some(sof_wm8804_probe),
    remove: Some(sof_wm8804_remove),
};

// module_platform_driver(sof_wm8804_driver);

// MODULE_DESCRIPTION("ASoC Intel(R) SOF + WM8804 Machine driver");
// MODULE_AUTHOR("Pierre-Louis Bossart");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:sof-wm8804");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
