// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file defines data structures and functions used in Machine
 * Driver for Intel platforms with Cirrus Logic Codecs.
 *
 * Copyright 2022 Intel Corporation.
 */

// C includes translated as external dependency intent:
// #include <linux/module.h>
// #include <sound/sof.h>
// #include "../../codecs/cs35l41.h"
// #include "sof_cirrus_common.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const CS35L41_HID: *const c_char = b"CSC3541\0".as_ptr() as *const c_char;
const CS35L41_MAX_AMPS: usize = 4;
const EINVAL: c_int = 22;

extern "C" {
    static CS35L41_CODEC_DAI: *const c_char;
    static CS35L41_CLKID_SCLK: c_int;
    static SND_SOC_CLOCK_IN: c_uint;

    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget,
        num: c_int,
    ) -> c_int;
    fn snd_soc_add_card_controls(
        card: *mut snd_soc_card,
        controls: *const snd_kcontrol_new,
        num: c_int,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn sof_dai_get_bclk(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_uint,
    ) -> c_int;
    fn snd_soc_component_set_sysclk(
        component: *mut snd_soc_component,
        clk_id: c_int,
        source: c_int,
        freq: c_uint,
        dir: c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_channel_map(
        dai: *mut snd_soc_dai,
        tx_num: c_uint,
        tx_slot: *mut c_uint,
        rx_num: c_uint,
        rx_slot: *mut c_uint,
    ) -> c_int;
    fn acpi_dev_get_first_match_dev(
        hid: *const c_char,
        uid: *const c_char,
        hrv: c_int,
    ) -> *mut acpi_device;
    fn acpi_get_first_physical_node(adev: *mut acpi_device) -> *mut device;
    fn acpi_dev_put(adev: *mut acpi_device);
    fn get_device(dev: *mut device) -> *mut device;
    fn dev_name(dev: *mut device) -> *const c_char;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pr_devel(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
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
pub struct snd_kcontrol_new {
    name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    name: *const c_char,
    event: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_codec_conf_dlc {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_codec_conf {
    pub dlc: snd_soc_codec_conf_dlc,
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_card {
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num_configs: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
    pub num_codecs: c_int,
    pub codec_dais: *mut *mut snd_soc_dai,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub num_codecs: c_int,
    pub codecs: *mut snd_soc_dai_link_component,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub ops: *const snd_soc_ops,
}

/*
 * Cirrus Logic CS35L41/CS35L53
 */
// SOC_DAPM_PIN_SWITCH("...") translated as the local data it contributes here.
static CS35L41_KCONTROLS: [snd_kcontrol_new; CS35L41_MAX_AMPS] = [
    snd_kcontrol_new {
        name: b"WL Spk\0".as_ptr() as *const c_char,
    },
    snd_kcontrol_new {
        name: b"WR Spk\0".as_ptr() as *const c_char,
    },
    snd_kcontrol_new {
        name: b"TL Spk\0".as_ptr() as *const c_char,
    },
    snd_kcontrol_new {
        name: b"TR Spk\0".as_ptr() as *const c_char,
    },
];

// SND_SOC_DAPM_SPK("...", NULL) translated as the local data it contributes here.
static CS35L41_DAPM_WIDGETS: [snd_soc_dapm_widget; CS35L41_MAX_AMPS] = [
    snd_soc_dapm_widget {
        name: b"WL Spk\0".as_ptr() as *const c_char,
        event: ptr::null_mut(),
    },
    snd_soc_dapm_widget {
        name: b"WR Spk\0".as_ptr() as *const c_char,
        event: ptr::null_mut(),
    },
    snd_soc_dapm_widget {
        name: b"TL Spk\0".as_ptr() as *const c_char,
        event: ptr::null_mut(),
    },
    snd_soc_dapm_widget {
        name: b"TR Spk\0".as_ptr() as *const c_char,
        event: ptr::null_mut(),
    },
];

static CS35L41_DAPM_ROUTES: [snd_soc_dapm_route; CS35L41_MAX_AMPS] = [
    /* speaker */
    snd_soc_dapm_route {
        sink: b"WL Spk\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"WL SPK\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"WR Spk\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"WR SPK\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"TL Spk\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"TL SPK\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"TR Spk\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"TR SPK\0".as_ptr() as *const c_char,
    },
];

static mut CS35L41_COMPONENTS: [snd_soc_dai_link_component; CS35L41_MAX_AMPS] =
    [snd_soc_dai_link_component {
        name: ptr::null(),
        dai_name: ptr::null(),
    }; CS35L41_MAX_AMPS];

/*
 * Mapping between ACPI instance id and speaker position.
 */
static mut CS35L41_CODEC_CONF: [snd_soc_codec_conf; CS35L41_MAX_AMPS] = [snd_soc_codec_conf {
    dlc: snd_soc_codec_conf_dlc { name: ptr::null() },
    name_prefix: ptr::null(),
}; CS35L41_MAX_AMPS];

unsafe extern "C" fn cs35l41_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    ret = snd_soc_dapm_new_controls(
        dapm,
        CS35L41_DAPM_WIDGETS.as_ptr(),
        CS35L41_DAPM_WIDGETS.len() as c_int,
    );
    if ret != 0 {
        dev_err(
            (*rtd).dev,
            b"fail to add dapm controls, ret %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = snd_soc_add_card_controls(
        card,
        CS35L41_KCONTROLS.as_ptr(),
        CS35L41_KCONTROLS.len() as c_int,
    );
    if ret != 0 {
        dev_err(
            (*rtd).dev,
            b"fail to add card controls, ret %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = snd_soc_dapm_add_routes(
        dapm,
        CS35L41_DAPM_ROUTES.as_ptr(),
        CS35L41_DAPM_ROUTES.len() as c_int,
    );

    if ret != 0 {
        dev_err(
            (*rtd).dev,
            b"fail to add dapm routes, ret %d\n\0".as_ptr() as *const c_char,
            ret,
        );
    }

    ret
}

/*
 * Channel map:
 *
 * TL/WL: ASPRX1 on slot 0, ASPRX2 on slot 1 (default)
 * TR/WR: ASPRX1 on slot 1, ASPRX2 on slot 0
 */
#[repr(C)]
struct cs35l41_channel_map_entry {
    rx: [c_uint; 2],
}

static CS35L41_CHANNEL_MAP: [cs35l41_channel_map_entry; CS35L41_MAX_AMPS] = [
    cs35l41_channel_map_entry { rx: [0, 1] }, /* WL */
    cs35l41_channel_map_entry { rx: [1, 0] }, /* WR */
    cs35l41_channel_map_entry { rx: [0, 1] }, /* TL */
    cs35l41_channel_map_entry { rx: [1, 0] }, /* TR */
];

unsafe extern "C" fn cs35l41_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let mut codec_dai: *mut snd_soc_dai;
    let clk_freq: c_int;
    let mut i: c_int;
    let mut ret: c_int;

    clk_freq = sof_dai_get_bclk(rtd); /* BCLK freq */

    if clk_freq <= 0 {
        dev_err(
            (*rtd).dev,
            b"fail to get bclk freq, ret %d\n\0".as_ptr() as *const c_char,
            clk_freq,
        );
        return -EINVAL;
    }

    i = 0;
    while i < (*rtd).num_codecs {
        codec_dai = *(*rtd).codec_dais.offset(i as isize);

        /* call dai driver's set_sysclk() callback */
        ret = snd_soc_dai_set_sysclk(
            codec_dai,
            CS35L41_CLKID_SCLK,
            clk_freq as c_uint,
            SND_SOC_CLOCK_IN,
        );
        if ret < 0 {
            dev_err(
                (*codec_dai).dev,
                b"fail to set sysclk, ret %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        /* call component driver's set_sysclk() callback */
        ret = snd_soc_component_set_sysclk(
            (*codec_dai).component,
            CS35L41_CLKID_SCLK,
            0,
            clk_freq as c_uint,
            SND_SOC_CLOCK_IN,
        );
        if ret < 0 {
            dev_err(
                (*codec_dai).dev,
                b"fail to set component sysclk, ret %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        /* setup channel map */
        ret = snd_soc_dai_set_channel_map(
            codec_dai,
            0,
            ptr::null_mut(),
            CS35L41_CHANNEL_MAP[i as usize].rx.len() as c_uint,
            CS35L41_CHANNEL_MAP[i as usize].rx.as_ptr() as *mut c_uint,
        );
        if ret < 0 {
            dev_err(
                (*codec_dai).dev,
                b"fail to set channel map, ret %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        i += 1;
    }

    0
}

static CS35L41_OPS: snd_soc_ops = snd_soc_ops {
    hw_params: Some(cs35l41_hw_params),
};

static CS35L41_NAME_PREFIXES: [*const c_char; CS35L41_MAX_AMPS] = [
    b"WL\0".as_ptr() as *const c_char,
    b"WR\0".as_ptr() as *const c_char,
    b"TL\0".as_ptr() as *const c_char,
    b"TR\0".as_ptr() as *const c_char,
];

/*
 * Expected UIDs are integers (stored as strings).
 * UID Mapping is fixed:
 * UID 0x0 -> WL
 * UID 0x1 -> WR
 * UID 0x2 -> TL
 * UID 0x3 -> TR
 * Note: If there are less than 4 Amps, UIDs still map to WL/WR/TL/TR. Dynamic code will only create
 * dai links for UIDs which exist, and ignore non-existant ones. Only 2 or 4 amps are expected.
 * Return number of codecs found.
 */
unsafe fn cs35l41_compute_codec_conf() -> c_int {
    static UID_STRINGS: [*const c_char; CS35L41_MAX_AMPS] = [
        b"0\0".as_ptr() as *const c_char,
        b"1\0".as_ptr() as *const c_char,
        b"2\0".as_ptr() as *const c_char,
        b"3\0".as_ptr() as *const c_char,
    ];
    let mut uid: c_uint;
    let mut sz: c_uint = 0;
    let mut adev: *mut acpi_device;
    let mut physdev: *mut device;

    uid = 0;
    while uid < CS35L41_MAX_AMPS as c_uint {
        adev = acpi_dev_get_first_match_dev(CS35L41_HID, UID_STRINGS[uid as usize], -1);
        if adev.is_null() {
            pr_devel(
                b"Cannot find match for HID %s UID %u (%s)\n\0".as_ptr() as *const c_char,
                CS35L41_HID,
                uid,
                CS35L41_NAME_PREFIXES[uid as usize],
            );
            uid += 1;
            continue;
        }
        physdev = get_device(acpi_get_first_physical_node(adev));
        acpi_dev_put(adev);
        if physdev.is_null() {
            pr_devel(
                b"Cannot find physical node for HID %s UID %u (%s)\n\0".as_ptr() as *const c_char,
                CS35L41_HID,
                uid,
                CS35L41_NAME_PREFIXES[uid as usize],
            );
            return 0;
        }
        CS35L41_COMPONENTS[sz as usize].name = dev_name(physdev);
        CS35L41_COMPONENTS[sz as usize].dai_name = CS35L41_CODEC_DAI;
        CS35L41_CODEC_CONF[sz as usize].dlc.name = dev_name(physdev);
        CS35L41_CODEC_CONF[sz as usize].name_prefix = CS35L41_NAME_PREFIXES[uid as usize];
        sz += 1;
        uid += 1;
    }

    if sz != 2 && sz != 4 {
        pr_warn(
            b"Invalid number of cs35l41 amps found: %d, expected 2 or 4\n\0".as_ptr()
                as *const c_char,
            sz,
        );
    }
    sz as c_int
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_set_dai_link(link: *mut snd_soc_dai_link) {
    (*link).num_codecs = cs35l41_compute_codec_conf();
    (*link).codecs = CS35L41_COMPONENTS.as_mut_ptr();
    (*link).init = Some(cs35l41_init);
    (*link).ops = &CS35L41_OPS;
}
// EXPORT_SYMBOL_NS(cs35l41_set_dai_link, "SND_SOC_INTEL_SOF_CIRRUS_COMMON");

#[no_mangle]
pub unsafe extern "C" fn cs35l41_set_codec_conf(card: *mut snd_soc_card) {
    (*card).codec_conf = CS35L41_CODEC_CONF.as_mut_ptr();
    (*card).num_configs = CS35L41_CODEC_CONF.len() as c_int;
}
// EXPORT_SYMBOL_NS(cs35l41_set_codec_conf, "SND_SOC_INTEL_SOF_CIRRUS_COMMON");

// MODULE_DESCRIPTION("ASoC Intel SOF Cirrus Logic helpers");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
