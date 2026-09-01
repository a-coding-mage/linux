// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble VT1724 (Envy24HT)
 *
 *   Lowlevel functions for ONKYO WAVIO SE-90PCI and SE-200PCI
 *
 *	Copyright (c) 2007 Shin-ya Okada  sh_okada(at)d4.dion.ne.jp
 *                                        (at) -> @
 */

/* C dependencies: linux/delay.h, linux/interrupt.h, linux/init.h,
 * linux/slab.h, sound/core.h, sound/tlv.h, ice1712.h, envy24ht.h, se.h
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;

extern "C" {
    static VT1724_SUBDEVICE_SE90PCI: c_uint;
    static VT1724_SUBDEVICE_SE200PCI: c_uint;

    static ICE_EEP2_SYSCONF: usize;
    static ICE_EEP2_ACLINK: usize;
    static ICE_EEP2_I2S: usize;
    static ICE_EEP2_SPDIF: usize;
    static ICE_EEP2_GPIO_DIR: usize;
    static ICE_EEP2_GPIO_DIR1: usize;
    static ICE_EEP2_GPIO_DIR2: usize;
    static ICE_EEP2_GPIO_MASK: usize;
    static ICE_EEP2_GPIO_MASK1: usize;
    static ICE_EEP2_GPIO_MASK2: usize;
    static ICE_EEP2_GPIO_STATE: usize;
    static ICE_EEP2_GPIO_STATE1: usize;
    static ICE_EEP2_GPIO_STATE2: usize;

    static SNDRV_CTL_ELEM_TYPE_INTEGER: c_int;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_int;
    static SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint;

    fn udelay(usecs: c_uint);
    fn snd_ice1712_save_gpio_status(ice: *mut snd_ice1712);
    fn snd_ice1712_restore_gpio_status(ice: *mut snd_ice1712);
    fn snd_ice1712_gpio_set_dir(ice: *mut snd_ice1712, data: c_uint);
    fn snd_ice1712_gpio_set_mask(ice: *mut snd_ice1712, data: c_uint);
    fn snd_ice1712_gpio_read(ice: *mut snd_ice1712) -> c_uint;
    fn snd_ice1712_gpio_write(ice: *mut snd_ice1712, data: c_uint);
    fn snd_vt1724_write_i2c(ice: *mut snd_ice1712, addr: c_uint, data1: c_uint, data2: c_uint);
    fn snd_kcontrol_chip(kc: *mut snd_kcontrol) -> *mut snd_ice1712;
    fn snd_ctl_enum_info(
        uinfo: *mut snd_ctl_elem_info,
        channels: c_uint,
        items: c_uint,
        names: *const *const c_char,
    ) -> c_int;
    fn snd_ctl_boolean_mono_info(
        kc: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> c_int;
    fn snd_ctl_add(card: *mut c_void, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(template: *mut snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_BUG();
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
}

const GFP_KERNEL: c_uint = 0;

#[repr(C)]
pub struct snd_ice1712 {
    pub gpio: snd_ice1712_gpio,
    pub spec: *mut c_void,
    pub eeprom: snd_ice1712_eeprom,
    pub num_total_dacs: c_uint,
    pub num_total_adcs: c_uint,
    pub vt1720: c_uint,
    pub card: *mut c_void,
}

#[repr(C)]
pub struct snd_ice1712_gpio {
    pub direction: c_uint,
    pub write_mask: c_uint,
    pub set_pro_rate: Option<unsafe extern "C" fn(*mut snd_ice1712, c_uint)>,
}

#[repr(C)]
pub struct snd_ice1712_eeprom {
    pub subvendor: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: isize,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
    pub access: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: isize,
    pub tlv: snd_kcontrol_new_tlv,
}

#[repr(C)]
pub union snd_kcontrol_new_tlv {
    pub p: *const c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_int,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: i64,
    pub max: i64,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub struct snd_ice1712_card_info {
    pub subvendor: c_uint,
    pub name: *const c_char,
    pub model: *const c_char,
    pub chip_init: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    pub build_controls: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    pub eeprom_size: usize,
    pub eeprom_data: *const u8,
}

#[repr(C)]
struct se_spec_vol {
    ch1: u8,
    ch2: u8,
}

#[repr(C)]
struct se_spec {
    vol: [se_spec_vol; 8],
}

/****************************************************************************/
/*  ONKYO WAVIO SE-200PCI                                                   */
/****************************************************************************/
/*
 *  system configuration ICE_EEP2_SYSCONF=0x4b
 *    XIN1 49.152MHz
 *    not have UART
 *    one stereo ADC and a S/PDIF receiver connected
 *    four stereo DACs connected
 *
 *  AC-Link configuration ICE_EEP2_ACLINK=0x80
 *    use I2C, not use AC97
 *
 *  I2S converters feature ICE_EEP2_I2S=0x78
 *    I2S codec has no volume/mute control feature
 *    I2S codec supports 96KHz and 192KHz
 *    I2S codec 24bits
 *
 *  S/PDIF configuration ICE_EEP2_SPDIF=0xc3
 *    Enable integrated S/PDIF transmitter
 *    internal S/PDIF out implemented
 *    S/PDIF is stereo
 *    External S/PDIF out implemented
 *
 *
 * ** connected chips **
 *
 *  WM8740
 *      A 2ch-DAC of main outputs.
 *      It setuped as I2S mode by wire, so no way to setup from software.
 *      The sample-rate are automatically changed.
 *          ML/I2S (28pin) --------+
 *          MC/DM1 (27pin) -- 5V   |
 *          MD/DM0 (26pin) -- GND  |
 *          MUTEB  (25pin) -- NC   |
 *          MODE   (24pin) -- GND  |
 *          CSBIW  (23pin) --------+
 *                                 |
 *          RSTB   (22pin) --R(1K)-+
 *      Probably it reduce the noise from the control line.
 *
 *  WM8766
 *      A 6ch-DAC for surrounds.
 *      It's control wire was connected to GPIOxx (3-wire serial interface)
 *          ML/I2S (11pin) -- GPIO18
 *          MC/IWL (12pin) -- GPIO17
 *          MD/DM  (13pin) -- GPIO16
 *          MUTE   (14pin) -- GPIO01
 *
 *  WM8776
 *     A 2ch-ADC(with 10ch-selector) plus 2ch-DAC.
 *     It's control wire was connected to SDA/SCLK (2-wire serial interface)
 *          MODE (16pin) -- R(1K) -- GND
 *          CE   (17pin) -- R(1K) -- GND  2-wire mode (address=0x34)
 *          DI   (18pin) -- SDA
 *          CL   (19pin) -- SCLK
 *
 *
 * ** output pins and device names **
 *
 *   7.1ch name -- output connector color -- device (-D option)
 *
 *      FRONT 2ch                  -- green  -- plughw:0,0
 *      CENTER(Lch) SUBWOOFER(Rch) -- black  -- plughw:0,2,0
 *      SURROUND 2ch               -- orange -- plughw:0,2,1
 *      SURROUND BACK 2ch          -- white  -- plughw:0,2,2
 *
 */

/****************************************************************************/
/*  WM8740 interface                                                        */
/****************************************************************************/

unsafe extern "C" fn se200pci_WM8740_init(_ice: *mut snd_ice1712) {
    /* nothing to do */
}

unsafe extern "C" fn se200pci_WM8740_set_pro_rate(_ice: *mut snd_ice1712, _rate: c_uint) {
    /* nothing to do */
}

/****************************************************************************/
/*  WM8766 interface                                                        */
/****************************************************************************/

unsafe extern "C" fn se200pci_WM8766_write(
    ice: *mut snd_ice1712,
    addr: c_uint,
    data: c_uint,
) {
    let mut st: c_uint;
    let mut bits: c_uint;
    let mut i: c_int;
    const DATA: c_uint = 0x010000;
    const CLOCK: c_uint = 0x020000;
    const LOAD: c_uint = 0x040000;
    const ALL_MASK: c_uint = DATA | CLOCK | LOAD;

    snd_ice1712_save_gpio_status(ice);

    st = ((addr & 0x7f) << 9) | (data & 0x1ff);
    snd_ice1712_gpio_set_dir(ice, (*ice).gpio.direction | ALL_MASK);
    snd_ice1712_gpio_set_mask(ice, (*ice).gpio.write_mask & !ALL_MASK);
    bits = snd_ice1712_gpio_read(ice) & !ALL_MASK;

    snd_ice1712_gpio_write(ice, bits);
    i = 0;
    while i < 16 {
        udelay(1);
        bits &= !CLOCK;
        st <<= 1;
        if (st & 0x10000) != 0 {
            bits |= DATA;
        } else {
            bits &= !DATA;
        }

        snd_ice1712_gpio_write(ice, bits);

        udelay(1);
        bits |= CLOCK;
        snd_ice1712_gpio_write(ice, bits);
        i += 1;
    }

    udelay(1);
    bits |= LOAD;
    snd_ice1712_gpio_write(ice, bits);

    udelay(1);
    bits |= DATA | CLOCK;
    snd_ice1712_gpio_write(ice, bits);

    snd_ice1712_restore_gpio_status(ice);
}

unsafe extern "C" fn se200pci_WM8766_set_volume(
    ice: *mut snd_ice1712,
    ch: c_int,
    vol1: c_uint,
    vol2: c_uint,
) {
    match ch {
        0 => {
            se200pci_WM8766_write(ice, 0x000, vol1);
            se200pci_WM8766_write(ice, 0x001, vol2 | 0x100);
        }
        1 => {
            se200pci_WM8766_write(ice, 0x004, vol1);
            se200pci_WM8766_write(ice, 0x005, vol2 | 0x100);
        }
        2 => {
            se200pci_WM8766_write(ice, 0x006, vol1);
            se200pci_WM8766_write(ice, 0x007, vol2 | 0x100);
        }
        _ => {}
    }
}

unsafe extern "C" fn se200pci_WM8766_init(ice: *mut snd_ice1712) {
    se200pci_WM8766_write(ice, 0x1f, 0x000); /* RESET ALL */
    udelay(10);

    se200pci_WM8766_set_volume(ice, 0, 0, 0); /* volume L=0 R=0 */
    se200pci_WM8766_set_volume(ice, 1, 0, 0); /* volume L=0 R=0 */
    se200pci_WM8766_set_volume(ice, 2, 0, 0); /* volume L=0 R=0 */

    se200pci_WM8766_write(ice, 0x03, 0x022); /* serial mode I2S-24bits */
    se200pci_WM8766_write(ice, 0x0a, 0x080); /* MCLK=256fs */
    se200pci_WM8766_write(ice, 0x12, 0x000); /* MDP=0 */
    se200pci_WM8766_write(ice, 0x15, 0x000); /* MDP=0 */
    se200pci_WM8766_write(ice, 0x09, 0x000); /* demp=off mute=off */

    se200pci_WM8766_write(ice, 0x02, 0x124); /* ch-assign L=L R=R RESET */
    se200pci_WM8766_write(ice, 0x02, 0x120); /* ch-assign L=L R=R */
}

unsafe extern "C" fn se200pci_WM8766_set_pro_rate(ice: *mut snd_ice1712, rate: c_uint) {
    if rate > 96000 {
        se200pci_WM8766_write(ice, 0x0a, 0x000); /* MCLK=128fs */
    } else {
        se200pci_WM8766_write(ice, 0x0a, 0x080); /* MCLK=256fs */
    }
}

/****************************************************************************/
/*  WM8776 interface                                                        */
/****************************************************************************/

unsafe extern "C" fn se200pci_WM8776_write(
    ice: *mut snd_ice1712,
    addr: c_uint,
    data: c_uint,
) {
    let val: c_uint = (addr << 9) | data;
    snd_vt1724_write_i2c(ice, 0x34, val >> 8, val & 0xff);
}

unsafe extern "C" fn se200pci_WM8776_set_output_volume(
    ice: *mut snd_ice1712,
    vol1: c_uint,
    vol2: c_uint,
) {
    se200pci_WM8776_write(ice, 0x03, vol1);
    se200pci_WM8776_write(ice, 0x04, vol2 | 0x100);
}

unsafe extern "C" fn se200pci_WM8776_set_input_volume(
    ice: *mut snd_ice1712,
    vol1: c_uint,
    vol2: c_uint,
) {
    se200pci_WM8776_write(ice, 0x0e, vol1);
    se200pci_WM8776_write(ice, 0x0f, vol2 | 0x100);
}

static SE200PCI_SEL_0: &[u8] = b"LINE-IN\0";
static SE200PCI_SEL_1: &[u8] = b"CD-IN\0";
static SE200PCI_SEL_2: &[u8] = b"MIC-IN\0";
static SE200PCI_SEL_3: &[u8] = b"ALL-MIX\0";
static SE200PCI_SEL: [*const c_char; 5] = [
    SE200PCI_SEL_0.as_ptr() as *const c_char,
    SE200PCI_SEL_1.as_ptr() as *const c_char,
    SE200PCI_SEL_2.as_ptr() as *const c_char,
    SE200PCI_SEL_3.as_ptr() as *const c_char,
    ptr::null(),
];

unsafe extern "C" fn se200pci_WM8776_set_input_selector(
    ice: *mut snd_ice1712,
    mut sel: c_uint,
) {
    static VALS: [u8; 5] = [
        /* LINE, CD, MIC, ALL, GND */
        0x10, 0x04, 0x08, 0x1c, 0x03,
    ];
    if sel > 4 {
        sel = 4;
    }
    se200pci_WM8776_write(ice, 0x15, VALS[sel as usize] as c_uint);
}

unsafe extern "C" fn se200pci_WM8776_set_afl(ice: *mut snd_ice1712, afl: c_uint) {
    /* AFL -- After Fader Listening */
    if afl != 0 {
        se200pci_WM8776_write(ice, 0x16, 0x005);
    } else {
        se200pci_WM8776_write(ice, 0x16, 0x001);
    }
}

static SE200PCI_AGC_0: &[u8] = b"Off\0";
static SE200PCI_AGC_1: &[u8] = b"LimiterMode\0";
static SE200PCI_AGC_2: &[u8] = b"ALCMode\0";
static SE200PCI_AGC: [*const c_char; 4] = [
    SE200PCI_AGC_0.as_ptr() as *const c_char,
    SE200PCI_AGC_1.as_ptr() as *const c_char,
    SE200PCI_AGC_2.as_ptr() as *const c_char,
    ptr::null(),
];

unsafe extern "C" fn se200pci_WM8776_set_agc(ice: *mut snd_ice1712, agc: c_uint) {
    /* AGC -- Auto Gain Control of the input */
    match agc {
        0 => {
            se200pci_WM8776_write(ice, 0x11, 0x000); /* Off */
        }
        1 => {
            se200pci_WM8776_write(ice, 0x10, 0x07b);
            se200pci_WM8776_write(ice, 0x11, 0x100); /* LimiterMode */
        }
        2 => {
            se200pci_WM8776_write(ice, 0x10, 0x1fb);
            se200pci_WM8776_write(ice, 0x11, 0x100); /* ALCMode */
        }
        _ => {}
    }
}

unsafe extern "C" fn se200pci_WM8776_init(ice: *mut snd_ice1712) {
    let mut i: c_int;
    static DEFAULT_VALUES: [u16; 23] = [
        0x100, 0x100, 0x100,
        0x100, 0x100, 0x100,
        0x000, 0x090, 0x000, 0x000,
        0x022, 0x022, 0x022,
        0x008, 0x0cf, 0x0cf, 0x07b, 0x000,
        0x032, 0x000, 0x0a6, 0x001, 0x001,
    ];

    se200pci_WM8776_write(ice, 0x17, 0x000); /* reset all */
    /* ADC and DAC interface is I2S 24bits mode */
    /* The sample-rate are automatically changed */
    udelay(10);
    /* BUT my board can not do reset all, so I load all by manually. */
    i = 0;
    while (i as usize) < DEFAULT_VALUES.len() {
        se200pci_WM8776_write(ice, i as c_uint, DEFAULT_VALUES[i as usize] as c_uint);
        i += 1;
    }

    se200pci_WM8776_set_input_selector(ice, 0);
    se200pci_WM8776_set_afl(ice, 0);
    se200pci_WM8776_set_agc(ice, 0);
    se200pci_WM8776_set_input_volume(ice, 0, 0);
    se200pci_WM8776_set_output_volume(ice, 0, 0);

    /* head phone mute and power down */
    se200pci_WM8776_write(ice, 0x00, 0);
    se200pci_WM8776_write(ice, 0x01, 0);
    se200pci_WM8776_write(ice, 0x02, 0x100);
    se200pci_WM8776_write(ice, 0x0d, 0x080);
}

unsafe extern "C" fn se200pci_WM8776_set_pro_rate(_ice: *mut snd_ice1712, _rate: c_uint) {
    /* nothing to do */
}

/****************************************************************************/
/*  runtime interface                                                       */
/****************************************************************************/

unsafe extern "C" fn se200pci_set_pro_rate(ice: *mut snd_ice1712, rate: c_uint) {
    se200pci_WM8740_set_pro_rate(ice, rate);
    se200pci_WM8766_set_pro_rate(ice, rate);
    se200pci_WM8776_set_pro_rate(ice, rate);
}

#[repr(C)]
struct se200pci_control {
    name: *const c_char,
    target: se200pci_control_target,
    type_: se200pci_control_type,
    ch: c_int,
    member: *const *const c_char,
    comment: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
enum se200pci_control_target {
    WM8766,
    WM8776in,
    WM8776out,
    WM8776sel,
    WM8776agc,
    WM8776afl,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum se200pci_control_type {
    VOLUME1,
    VOLUME2,
    BOOLEAN,
    ENUM,
}

static SE200PCI_CONT_NAME_0: &[u8] = b"Front Playback Volume\0";
static SE200PCI_CONT_NAME_1: &[u8] = b"Side Playback Volume\0";
static SE200PCI_CONT_NAME_2: &[u8] = b"Surround Playback Volume\0";
static SE200PCI_CONT_NAME_3: &[u8] = b"CLFE Playback Volume\0";
static SE200PCI_CONT_NAME_4: &[u8] = b"Capture Volume\0";
static SE200PCI_CONT_NAME_5: &[u8] = b"Capture Select\0";
static SE200PCI_CONT_NAME_6: &[u8] = b"AGC Capture Mode\0";
static SE200PCI_CONT_NAME_7: &[u8] = b"AFL Bypass Playback Switch\0";
static SE200PCI_CONT_COMMENT_0: &[u8] = b"Front(green)\0";
static SE200PCI_CONT_COMMENT_1: &[u8] = b"Surround(orange)\0";
static SE200PCI_CONT_COMMENT_2: &[u8] = b"SurroundBack(white)\0";
static SE200PCI_CONT_COMMENT_3: &[u8] = b"Center(Lch)&SubWoofer(Rch)(black)\0";

static SE200PCI_CONT: [se200pci_control; 8] = [
    se200pci_control {
        name: SE200PCI_CONT_NAME_0.as_ptr() as *const c_char,
        target: se200pci_control_target::WM8776out,
        type_: se200pci_control_type::VOLUME1,
        ch: 0,
        member: ptr::null(),
        comment: SE200PCI_CONT_COMMENT_0.as_ptr() as *const c_char,
    },
    se200pci_control {
        name: SE200PCI_CONT_NAME_1.as_ptr() as *const c_char,
        target: se200pci_control_target::WM8766,
        type_: se200pci_control_type::VOLUME1,
        ch: 1,
        member: ptr::null(),
        comment: SE200PCI_CONT_COMMENT_1.as_ptr() as *const c_char,
    },
    se200pci_control {
        name: SE200PCI_CONT_NAME_2.as_ptr() as *const c_char,
        target: se200pci_control_target::WM8766,
        type_: se200pci_control_type::VOLUME1,
        ch: 2,
        member: ptr::null(),
        comment: SE200PCI_CONT_COMMENT_2.as_ptr() as *const c_char,
    },
    se200pci_control {
        name: SE200PCI_CONT_NAME_3.as_ptr() as *const c_char,
        target: se200pci_control_target::WM8766,
        type_: se200pci_control_type::VOLUME1,
        ch: 0,
        member: ptr::null(),
        comment: SE200PCI_CONT_COMMENT_3.as_ptr() as *const c_char,
    },
    se200pci_control {
        name: SE200PCI_CONT_NAME_4.as_ptr() as *const c_char,
        target: se200pci_control_target::WM8776in,
        type_: se200pci_control_type::VOLUME2,
        ch: 0,
        member: ptr::null(),
        comment: ptr::null(),
    },
    se200pci_control {
        name: SE200PCI_CONT_NAME_5.as_ptr() as *const c_char,
        target: se200pci_control_target::WM8776sel,
        type_: se200pci_control_type::ENUM,
        ch: 0,
        member: SE200PCI_SEL.as_ptr(),
        comment: ptr::null(),
    },
    se200pci_control {
        name: SE200PCI_CONT_NAME_6.as_ptr() as *const c_char,
        target: se200pci_control_target::WM8776agc,
        type_: se200pci_control_type::ENUM,
        ch: 0,
        member: SE200PCI_AGC.as_ptr(),
        comment: ptr::null(),
    },
    se200pci_control {
        name: SE200PCI_CONT_NAME_7.as_ptr() as *const c_char,
        target: se200pci_control_target::WM8776afl,
        type_: se200pci_control_type::BOOLEAN,
        ch: 0,
        member: ptr::null(),
        comment: ptr::null(),
    },
];

unsafe extern "C" fn se200pci_get_enum_count(n: c_int) -> c_int {
    let member: *const *const c_char;
    let mut c: c_int;

    member = SE200PCI_CONT[n as usize].member;
    if member.is_null() {
        return 0;
    }
    c = 0;
    while !(*member.add(c as usize)).is_null() {
        c += 1;
    }
    c
}

unsafe extern "C" fn se200pci_cont_volume_info(
    _kc: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0; /* mute */
    (*uinfo).value.integer.max = 0xff; /* 0dB */
    0
}

unsafe extern "C" fn se200pci_cont_boolean_info(
    kc: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    snd_ctl_boolean_mono_info(kc, uinfo)
}

unsafe extern "C" fn se200pci_cont_enum_info(
    kc: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let n: c_int;
    let c: c_int;

    n = (*kc).private_value as c_int;
    c = se200pci_get_enum_count(n);
    if c == 0 {
        return -EINVAL;
    }
    snd_ctl_enum_info(uinfo, 1, c as c_uint, SE200PCI_CONT[n as usize].member)
}

unsafe extern "C" fn se200pci_cont_volume_get(
    kc: *mut snd_kcontrol,
    uc: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kc);
    let spec: *mut se_spec = (*ice).spec as *mut se_spec;
    let n: c_int = (*kc).private_value as c_int;
    (*uc).value.integer.value[0] = (*spec).vol[n as usize].ch1 as i64;
    (*uc).value.integer.value[1] = (*spec).vol[n as usize].ch2 as i64;
    0
}

unsafe extern "C" fn se200pci_cont_boolean_get(
    kc: *mut snd_kcontrol,
    uc: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kc);
    let spec: *mut se_spec = (*ice).spec as *mut se_spec;
    let n: c_int = (*kc).private_value as c_int;
    (*uc).value.integer.value[0] = (*spec).vol[n as usize].ch1 as i64;
    0
}

unsafe extern "C" fn se200pci_cont_enum_get(
    kc: *mut snd_kcontrol,
    uc: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kc);
    let spec: *mut se_spec = (*ice).spec as *mut se_spec;
    let n: c_int = (*kc).private_value as c_int;
    (*uc).value.enumerated.item[0] = (*spec).vol[n as usize].ch1 as c_uint;
    0
}

unsafe extern "C" fn se200pci_cont_update(ice: *mut snd_ice1712, n: c_int) {
    let spec: *mut se_spec = (*ice).spec as *mut se_spec;
    match SE200PCI_CONT[n as usize].target {
        se200pci_control_target::WM8766 => {
            se200pci_WM8766_set_volume(
                ice,
                SE200PCI_CONT[n as usize].ch,
                (*spec).vol[n as usize].ch1 as c_uint,
                (*spec).vol[n as usize].ch2 as c_uint,
            );
        }

        se200pci_control_target::WM8776in => {
            se200pci_WM8776_set_input_volume(
                ice,
                (*spec).vol[n as usize].ch1 as c_uint,
                (*spec).vol[n as usize].ch2 as c_uint,
            );
        }

        se200pci_control_target::WM8776out => {
            se200pci_WM8776_set_output_volume(
                ice,
                (*spec).vol[n as usize].ch1 as c_uint,
                (*spec).vol[n as usize].ch2 as c_uint,
            );
        }

        se200pci_control_target::WM8776sel => {
            se200pci_WM8776_set_input_selector(ice, (*spec).vol[n as usize].ch1 as c_uint);
        }

        se200pci_control_target::WM8776agc => {
            se200pci_WM8776_set_agc(ice, (*spec).vol[n as usize].ch1 as c_uint);
        }

        se200pci_control_target::WM8776afl => {
            se200pci_WM8776_set_afl(ice, (*spec).vol[n as usize].ch1 as c_uint);
        }
    }
}

unsafe extern "C" fn se200pci_cont_volume_put(
    kc: *mut snd_kcontrol,
    uc: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kc);
    let spec: *mut se_spec = (*ice).spec as *mut se_spec;
    let n: c_int = (*kc).private_value as c_int;
    let vol1: c_uint;
    let vol2: c_uint;
    let mut changed: c_int;

    changed = 0;
    vol1 = ((*uc).value.integer.value[0] as c_uint) & 0xff;
    vol2 = ((*uc).value.integer.value[1] as c_uint) & 0xff;
    if (*spec).vol[n as usize].ch1 as c_uint != vol1 {
        (*spec).vol[n as usize].ch1 = vol1 as u8;
        changed = 1;
    }
    if (*spec).vol[n as usize].ch2 as c_uint != vol2 {
        (*spec).vol[n as usize].ch2 = vol2 as u8;
        changed = 1;
    }
    if changed != 0 {
        se200pci_cont_update(ice, n);
    }

    changed
}

unsafe extern "C" fn se200pci_cont_boolean_put(
    kc: *mut snd_kcontrol,
    uc: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kc);
    let spec: *mut se_spec = (*ice).spec as *mut se_spec;
    let n: c_int = (*kc).private_value as c_int;
    let vol1: c_uint;

    vol1 = if (*uc).value.integer.value[0] != 0 { 1 } else { 0 };
    if (*spec).vol[n as usize].ch1 as c_uint != vol1 {
        (*spec).vol[n as usize].ch1 = vol1 as u8;
        se200pci_cont_update(ice, n);
        return 1;
    }
    0
}

unsafe extern "C" fn se200pci_cont_enum_put(
    kc: *mut snd_kcontrol,
    uc: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kc);
    let spec: *mut se_spec = (*ice).spec as *mut se_spec;
    let n: c_int = (*kc).private_value as c_int;
    let vol1: c_uint;

    vol1 = (*uc).value.enumerated.item[0];
    if vol1 >= se200pci_get_enum_count(n) as c_uint {
        return -EINVAL;
    }
    if (*spec).vol[n as usize].ch1 as c_uint != vol1 {
        (*spec).vol[n as usize].ch1 = vol1 as u8;
        se200pci_cont_update(ice, n);
        return 1;
    }
    0
}

static DB_SCALE_GAIN1: [c_uint; 4] = [0, (-12750i32) as c_uint, 50, 1];
static DB_SCALE_GAIN2: [c_uint; 4] = [0, (-10350i32) as c_uint, 50, 1];

unsafe extern "C" fn se200pci_add_controls(ice: *mut snd_ice1712) -> c_int {
    let mut i: c_int;
    let mut cont: snd_kcontrol_new = mem::zeroed();
    let mut err: c_int;

    cont.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    i = 0;
    while (i as usize) < SE200PCI_CONT.len() {
        cont.private_value = i as isize;
        cont.name = SE200PCI_CONT[i as usize].name;
        cont.access = SNDRV_CTL_ELEM_ACCESS_READWRITE;
        cont.tlv.p = ptr::null();
        match SE200PCI_CONT[i as usize].type_ {
            se200pci_control_type::VOLUME1 | se200pci_control_type::VOLUME2 => {
                cont.info = Some(se200pci_cont_volume_info);
                cont.get = Some(se200pci_cont_volume_get);
                cont.put = Some(se200pci_cont_volume_put);
                cont.access |= SNDRV_CTL_ELEM_ACCESS_TLV_READ;
                if SE200PCI_CONT[i as usize].type_ == se200pci_control_type::VOLUME1 {
                    cont.tlv.p = DB_SCALE_GAIN1.as_ptr();
                } else {
                    cont.tlv.p = DB_SCALE_GAIN2.as_ptr();
                }
            }
            se200pci_control_type::BOOLEAN => {
                cont.info = Some(se200pci_cont_boolean_info);
                cont.get = Some(se200pci_cont_boolean_get);
                cont.put = Some(se200pci_cont_boolean_put);
            }
            se200pci_control_type::ENUM => {
                cont.info = Some(se200pci_cont_enum_info);
                cont.get = Some(se200pci_cont_enum_get);
                cont.put = Some(se200pci_cont_enum_put);
            }
        }
        err = snd_ctl_add((*ice).card, snd_ctl_new1(&mut cont, ice as *mut c_void));
        if err < 0 {
            return err;
        }
        i += 1;
    }

    0
}

/****************************************************************************/
/*  ONKYO WAVIO SE-90PCI                                                    */
/****************************************************************************/
/*
 *  system configuration ICE_EEP2_SYSCONF=0x4b
 *  AC-Link configuration ICE_EEP2_ACLINK=0x80
 *  I2S converters feature ICE_EEP2_I2S=0x78
 *  S/PDIF configuration ICE_EEP2_SPDIF=0xc3
 *
 *  ** connected chip **
 *
 *   WM8716
 *      A 2ch-DAC of main outputs.
 *      It setuped as I2S mode by wire, so no way to setup from software.
 *         ML/I2S (28pin) -- +5V
 *         MC/DM1 (27pin) -- GND
 *         MC/DM0 (26pin) -- GND
 *         MUTEB  (25pin) -- open (internal pull-up)
 *         MODE   (24pin) -- GND
 *         CSBIWO (23pin) -- +5V
 *
 */

/* Nothing to do for this chip. */

/****************************************************************************/
/*  probe/initialize/setup                                                  */
/****************************************************************************/

unsafe extern "C" fn se_init(ice: *mut snd_ice1712) -> c_int {
    let spec: *mut se_spec;

    spec = kzalloc(mem::size_of::<se_spec>(), GFP_KERNEL) as *mut se_spec;
    if spec.is_null() {
        return -ENOMEM;
    }
    (*ice).spec = spec as *mut c_void;

    if (*ice).eeprom.subvendor == VT1724_SUBDEVICE_SE90PCI {
        (*ice).num_total_dacs = 2;
        (*ice).num_total_adcs = 0;
        (*ice).vt1720 = 1;
        return 0;
    } else if (*ice).eeprom.subvendor == VT1724_SUBDEVICE_SE200PCI {
        (*ice).num_total_dacs = 8;
        (*ice).num_total_adcs = 2;
        se200pci_WM8740_init(ice);
        se200pci_WM8766_init(ice);
        se200pci_WM8776_init(ice);
        (*ice).gpio.set_pro_rate = Some(se200pci_set_pro_rate);
        return 0;
    }

    -ENOENT
}

unsafe extern "C" fn se_add_controls(ice: *mut snd_ice1712) -> c_int {
    let mut err: c_int;

    err = 0;
    /* nothing to do for VT1724_SUBDEVICE_SE90PCI */
    if (*ice).eeprom.subvendor == VT1724_SUBDEVICE_SE200PCI {
        err = se200pci_add_controls(ice);
    }

    err
}

/****************************************************************************/
/*  entry point                                                             */
/****************************************************************************/

static SE200PCI_EEPROM: [u8; 13] = {
    let mut a = [0u8; 13];
    /* 49.152Hz, spdif-in/ADC, 4DACs */
    a[0] = 0x4b; /* ICE_EEP2_SYSCONF */
    /* I2S */
    a[1] = 0x80; /* ICE_EEP2_ACLINK */
    /* 96k-ok, 24bit, 192k-ok */
    a[2] = 0x78; /* ICE_EEP2_I2S */
    /* out-en, out-int, spdif-in */
    a[3] = 0xc3; /* ICE_EEP2_SPDIF */

    a[4] = 0x02; /* ICE_EEP2_GPIO_DIR: WM8766 mute      1=output */
    a[5] = 0x00; /* ICE_EEP2_GPIO_DIR1: not used */
    a[6] = 0x07; /* ICE_EEP2_GPIO_DIR2: WM8766 ML/MC/MD  1=output */

    a[7] = 0x00; /* ICE_EEP2_GPIO_MASK: 0=writable */
    a[8] = 0x00; /* ICE_EEP2_GPIO_MASK1: 0=writable */
    a[9] = 0x00; /* ICE_EEP2_GPIO_MASK2: 0=writable */

    a[10] = 0x00; /* ICE_EEP2_GPIO_STATE: WM8766 mute=0 */
    a[11] = 0x00; /* ICE_EEP2_GPIO_STATE1: not used */
    a[12] = 0x07; /* ICE_EEP2_GPIO_STATE2: WM8766 ML/MC/MD */
    a
};

static SE90PCI_EEPROM: [u8; 4] = {
    let mut a = [0u8; 4];
    /* 49.152Hz, spdif-in/ADC, 4DACs */
    a[0] = 0x4b; /* ICE_EEP2_SYSCONF */
    /* I2S */
    a[1] = 0x80; /* ICE_EEP2_ACLINK */
    /* 96k-ok, 24bit, 192k-ok */
    a[2] = 0x78; /* ICE_EEP2_I2S */
    /* out-en, out-int, spdif-in */
    a[3] = 0xc3; /* ICE_EEP2_SPDIF */

    /* ALL GPIO bits are in input mode */
    a
};

#[no_mangle]
pub static mut snd_vt1724_se_cards: [snd_ice1712_card_info; 3] = [
    snd_ice1712_card_info {
        subvendor: unsafe { VT1724_SUBDEVICE_SE200PCI },
        name: b"ONKYO SE200PCI\0".as_ptr() as *const c_char,
        model: b"se200pci\0".as_ptr() as *const c_char,
        chip_init: Some(se_init),
        build_controls: Some(se_add_controls),
        eeprom_size: mem::size_of_val(&SE200PCI_EEPROM),
        eeprom_data: SE200PCI_EEPROM.as_ptr(),
    },
    snd_ice1712_card_info {
        subvendor: unsafe { VT1724_SUBDEVICE_SE90PCI },
        name: b"ONKYO SE90PCI\0".as_ptr() as *const c_char,
        model: b"se90pci\0".as_ptr() as *const c_char,
        chip_init: Some(se_init),
        build_controls: Some(se_add_controls),
        eeprom_size: mem::size_of_val(&SE90PCI_EEPROM),
        eeprom_data: SE90PCI_EEPROM.as_ptr(),
    },
    snd_ice1712_card_info {
        subvendor: 0,
        name: ptr::null(),
        model: ptr::null(),
        chip_init: None,
        build_controls: None,
        eeprom_size: 0,
        eeprom_data: ptr::null(),
    }, /*terminator*/
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
