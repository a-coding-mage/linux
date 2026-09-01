// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble VT1724 (Envy24HT)
 *
 *   Lowlevel functions for Philips PSC724 Ultimate Edge
 *
 *	Copyright (c) 2012 Ondrej Zary <linux@rainbow-software.org>
 */

/*
 * C dependencies:
 * <linux/delay.h>, <linux/init.h>, <linux/slab.h>, <sound/core.h>,
 * "ice1712.h", "envy24ht.h", "psc724.h", "wm8766.h", "wm8776.h"
 */

use core::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong};
use core::mem::{size_of, zeroed};
use core::ptr::null;

#[repr(C)]
struct psc724_spec {
    wm8766: snd_wm8766,
    wm8776: snd_wm8776,
    mute_all: bool,
    jack_detect: bool,
    ice: *mut snd_ice1712,
    hp_work: delayed_work,
    hp_connected: bool,
}

/****************************************************************************/
/*  PHILIPS PSC724 ULTIMATE EDGE                                            */
/****************************************************************************/
/*
 *  VT1722 (Envy24GT) - 6 outputs, 4 inputs (only 2 used), 24-bit/96kHz
 *
 *  system configuration ICE_EEP2_SYSCONF=0x42
 *    XIN1 49.152MHz
 *    no MPU401
 *    one stereo ADC, no S/PDIF receiver
 *    three stereo DACs (FRONT, REAR, CENTER+LFE)
 *
 *  AC-Link configuration ICE_EEP2_ACLINK=0x80
 *    use I2S, not AC97
 *
 *  I2S converters feature ICE_EEP2_I2S=0x30
 *    I2S codec has no volume/mute control feature (bug!)
 *    I2S codec does not support 96KHz or 192KHz (bug!)
 *    I2S codec 24bits
 *
 *  S/PDIF configuration ICE_EEP2_SPDIF=0xc1
 *    Enable integrated S/PDIF transmitter
 *    internal S/PDIF out implemented
 *    No S/PDIF input
 *    External S/PDIF out implemented
 *
 *
 * ** connected chips **
 *
 *  WM8776
 *     2-channel DAC used for main output and stereo ADC (with 10-channel MUX)
 *     AIN1: LINE IN, AIN2: CD/VIDEO, AIN3: AUX, AIN4: Front MIC, AIN5: Rear MIC
 *     Controlled by I2C using VT1722 I2C interface:
 *          MODE (pin16) -- GND
 *          CE   (pin17) -- GND  I2C mode (address=0x34)
 *          DI   (pin18) -- SDA  (VT1722 pin70)
 *          CL   (pin19) -- SCLK (VT1722 pin71)
 *
 *  WM8766
 *      6-channel DAC used for rear & center/LFE outputs (only 4 channels used)
 *      Controlled by SPI using VT1722 GPIO pins:
 *          MODE   (pin 1) -- GPIO19 (VT1722 pin99)
 *          ML/I2S (pin11) -- GPIO18 (VT1722 pin98)
 *          MC/IWL (pin12) -- GPIO17 (VT1722 pin97)
 *          MD/DM  (pin13) -- GPIO16 (VT1722 pin96)
 *          MUTE   (pin14) -- GPIO20 (VT1722 pin101)
 *
 *  GPIO14 is used as input for headphone jack detection (1 = connected)
 *  GPIO22 is used as MUTE ALL output, grounding all 6 channels
 *
 * ** output pins and device names **
 *
 *   5.1ch name -- output connector color -- device (-D option)
 *
 *      FRONT 2ch                  -- green  -- plughw:0,0
 *      CENTER(Lch) SUBWOOFER(Rch) -- orange -- plughw:0,2,0
 *      REAR 2ch                   -- black  -- plughw:0,2,1
 */

/* codec access low-level functions */

const GPIO_HP_JACK: c_uint = 1 << 14;
const GPIO_MUTE_SUR: c_uint = 1 << 20;
const GPIO_MUTE_ALL: c_uint = 1 << 22;

const JACK_INTERVAL: c_uint = 1000;

const PSC724_SPI_DELAY: c_uint = 1;

const PSC724_SPI_DATA: c_uint = 1 << 16;
const PSC724_SPI_CLK: c_uint = 1 << 17;
const PSC724_SPI_LOAD: c_uint = 1 << 18;
const PSC724_SPI_MASK: c_uint = PSC724_SPI_DATA | PSC724_SPI_CLK | PSC724_SPI_LOAD;

unsafe fn psc724_wm8766_write(wm: *mut snd_wm8766, addr: u16, data: u16) {
    let spec: *mut psc724_spec = container_of!(wm, psc724_spec, wm8766);
    let ice: *mut snd_ice1712 = (*spec).ice;
    let mut st: u32;
    let mut bits: u32;
    let mut i: c_int;

    snd_ice1712_save_gpio_status(ice);

    st = (((addr & 0x7f) as u32) << 9) | ((data & 0x1ff) as u32);
    snd_ice1712_gpio_set_dir(ice, (*ice).gpio.direction | PSC724_SPI_MASK);
    snd_ice1712_gpio_set_mask(ice, (*ice).gpio.write_mask & !PSC724_SPI_MASK);
    bits = snd_ice1712_gpio_read(ice) & !PSC724_SPI_MASK;
    snd_ice1712_gpio_write(ice, bits);

    i = 0;
    while i < 16 {
        udelay(PSC724_SPI_DELAY);
        bits &= !PSC724_SPI_CLK;
        /* MSB first */
        st <<= 1;
        if (st & 0x10000) != 0 {
            bits |= PSC724_SPI_DATA;
        } else {
            bits &= !PSC724_SPI_DATA;
        }
        snd_ice1712_gpio_write(ice, bits);
        /* CLOCK high */
        udelay(PSC724_SPI_DELAY);
        bits |= PSC724_SPI_CLK;
        snd_ice1712_gpio_write(ice, bits);
        i += 1;
    }
    /* LOAD high */
    udelay(PSC724_SPI_DELAY);
    bits |= PSC724_SPI_LOAD;
    snd_ice1712_gpio_write(ice, bits);
    /* LOAD low, DATA and CLOCK high */
    udelay(PSC724_SPI_DELAY);
    bits |= PSC724_SPI_DATA | PSC724_SPI_CLK;
    snd_ice1712_gpio_write(ice, bits);

    snd_ice1712_restore_gpio_status(ice);
}

unsafe fn psc724_wm8776_write(wm: *mut snd_wm8776, addr: u8, data: u8) {
    let spec: *mut psc724_spec = container_of!(wm, psc724_spec, wm8776);

    snd_vt1724_write_i2c((*spec).ice, 0x34, addr, data);
}

/* mute all */

unsafe fn psc724_set_master_switch(ice: *mut snd_ice1712, on: bool) {
    let mut bits: c_uint = snd_ice1712_gpio_read(ice);
    let spec: *mut psc724_spec = (*ice).spec as *mut psc724_spec;

    (*spec).mute_all = !on;
    if on {
        bits &= !(GPIO_MUTE_ALL | GPIO_MUTE_SUR);
    } else {
        bits |= GPIO_MUTE_ALL | GPIO_MUTE_SUR;
    }
    snd_ice1712_gpio_write(ice, bits);
}

unsafe fn psc724_get_master_switch(ice: *mut snd_ice1712) -> bool {
    let spec: *mut psc724_spec = (*ice).spec as *mut psc724_spec;

    !(*spec).mute_all
}

/* jack detection */

unsafe fn psc724_set_jack_state(ice: *mut snd_ice1712, hp_connected: bool) {
    let spec: *mut psc724_spec = (*ice).spec as *mut psc724_spec;
    let mut kctl: *mut snd_kcontrol;
    let mut power: u16 = (*spec).wm8776.regs[WM8776_REG_PWRDOWN] & !(WM8776_PWR_HPPD as u16);

    psc724_set_master_switch(ice, !hp_connected);
    if !hp_connected {
        power |= WM8776_PWR_HPPD as u16;
    }
    snd_wm8776_set_power(&mut (*spec).wm8776, power);
    (*spec).hp_connected = hp_connected;
    /* notify about master speaker mute change */
    kctl = snd_ctl_find_id_mixer((*ice).card, c"Master Speakers Playback Switch".as_ptr());
    if !kctl.is_null() {
        snd_ctl_notify((*ice).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*kctl).id);
    }
    /* and headphone mute change */
    kctl = snd_ctl_find_id_mixer(
        (*ice).card,
        (*spec).wm8776.ctl[WM8776_CTL_HP_SW].name,
    );
    if !kctl.is_null() {
        snd_ctl_notify((*ice).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*kctl).id);
    }
}

unsafe fn psc724_update_hp_jack_state(work: *mut work_struct) {
    let spec: *mut psc724_spec = container_of!(work, psc724_spec, hp_work.work);
    let ice: *mut snd_ice1712 = (*spec).ice;
    let hp_connected: bool = (snd_ice1712_gpio_read(ice) & GPIO_HP_JACK) != 0;

    schedule_delayed_work(&mut (*spec).hp_work, msecs_to_jiffies(JACK_INTERVAL));
    if hp_connected == (*spec).hp_connected {
        return;
    }
    psc724_set_jack_state(ice, hp_connected);
}

unsafe fn psc724_set_jack_detection(ice: *mut snd_ice1712, on: bool) {
    let spec: *mut psc724_spec = (*ice).spec as *mut psc724_spec;

    if (*spec).jack_detect == on {
        return;
    }

    (*spec).jack_detect = on;
    if on {
        let hp_connected: bool = (snd_ice1712_gpio_read(ice) & GPIO_HP_JACK) != 0;
        psc724_set_jack_state(ice, hp_connected);
        schedule_delayed_work(&mut (*spec).hp_work, msecs_to_jiffies(JACK_INTERVAL));
    } else {
        cancel_delayed_work_sync(&mut (*spec).hp_work);
    }
}

unsafe fn psc724_get_jack_detection(ice: *mut snd_ice1712) -> bool {
    let spec: *mut psc724_spec = (*ice).spec as *mut psc724_spec;

    (*spec).jack_detect
}

/* mixer controls */

#[repr(C)]
struct psc724_control {
    name: *const c_char,
    set: unsafe fn(*mut snd_ice1712, bool),
    get: unsafe fn(*mut snd_ice1712) -> bool,
}

static psc724_cont: [psc724_control; 2] = [
    psc724_control {
        name: c"Master Speakers Playback Switch".as_ptr(),
        set: psc724_set_master_switch,
        get: psc724_get_master_switch,
    },
    psc724_control {
        name: c"Headphone Jack Detection Playback Switch".as_ptr(),
        set: psc724_set_jack_detection,
        get: psc724_get_jack_detection,
    },
];

unsafe fn psc724_ctl_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let n: c_int = (*kcontrol).private_value as c_int;

    (*ucontrol).value.integer.value[0] = (psc724_cont[n as usize].get)(ice) as c_long;

    0
}

unsafe fn psc724_ctl_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let n: c_int = (*kcontrol).private_value as c_int;

    (psc724_cont[n as usize].set)(ice, (*ucontrol).value.integer.value[0] != 0);

    0
}

static front_volume: *const c_char = c"Front Playback Volume".as_ptr();
static front_switch: *const c_char = c"Front Playback Switch".as_ptr();
static front_zc: *const c_char = c"Front Zero Cross Detect Playback Switch".as_ptr();
static front_izd: *const c_char = c"Front Infinite Zero Detect Playback Switch".as_ptr();
static front_phase: *const c_char = c"Front Phase Invert Playback Switch".as_ptr();
static front_deemph: *const c_char = c"Front Deemphasis Playback Switch".as_ptr();
static ain1_switch: *const c_char = c"Line Capture Switch".as_ptr();
static ain2_switch: *const c_char = c"CD Capture Switch".as_ptr();
static ain3_switch: *const c_char = c"AUX Capture Switch".as_ptr();
static ain4_switch: *const c_char = c"Front Mic Capture Switch".as_ptr();
static ain5_switch: *const c_char = c"Rear Mic Capture Switch".as_ptr();
static rear_volume: *const c_char = c"Surround Playback Volume".as_ptr();
static clfe_volume: *const c_char = c"CLFE Playback Volume".as_ptr();
static rear_switch: *const c_char = c"Surround Playback Switch".as_ptr();
static clfe_switch: *const c_char = c"CLFE Playback Switch".as_ptr();
static rear_phase: *const c_char = c"Surround Phase Invert Playback Switch".as_ptr();
static clfe_phase: *const c_char = c"CLFE Phase Invert Playback Switch".as_ptr();
static rear_deemph: *const c_char = c"Surround Deemphasis Playback Switch".as_ptr();
static clfe_deemph: *const c_char = c"CLFE Deemphasis Playback Switch".as_ptr();
static rear_clfe_izd: *const c_char = c"Rear Infinite Zero Detect Playback Switch".as_ptr();
static rear_clfe_zc: *const c_char = c"Rear Zero Cross Detect Playback Switch".as_ptr();

unsafe fn psc724_add_controls(ice: *mut snd_ice1712) -> c_int {
    let mut cont: snd_kcontrol_new;
    let mut ctl: *mut snd_kcontrol;
    let mut err: c_int;
    let mut i: c_int;
    let spec: *mut psc724_spec = (*ice).spec as *mut psc724_spec;

    (*spec).wm8776.ctl[WM8776_CTL_DAC_VOL].name = front_volume;
    (*spec).wm8776.ctl[WM8776_CTL_DAC_SW].name = front_switch;
    (*spec).wm8776.ctl[WM8776_CTL_DAC_ZC_SW].name = front_zc;
    (*spec).wm8776.ctl[WM8776_CTL_AUX_SW].name = null();
    (*spec).wm8776.ctl[WM8776_CTL_DAC_IZD_SW].name = front_izd;
    (*spec).wm8776.ctl[WM8776_CTL_PHASE_SW].name = front_phase;
    (*spec).wm8776.ctl[WM8776_CTL_DEEMPH_SW].name = front_deemph;
    (*spec).wm8776.ctl[WM8776_CTL_INPUT1_SW].name = ain1_switch;
    (*spec).wm8776.ctl[WM8776_CTL_INPUT2_SW].name = ain2_switch;
    (*spec).wm8776.ctl[WM8776_CTL_INPUT3_SW].name = ain3_switch;
    (*spec).wm8776.ctl[WM8776_CTL_INPUT4_SW].name = ain4_switch;
    (*spec).wm8776.ctl[WM8776_CTL_INPUT5_SW].name = ain5_switch;
    snd_wm8776_build_controls(&mut (*spec).wm8776);
    (*spec).wm8766.ctl[WM8766_CTL_CH1_VOL].name = rear_volume;
    (*spec).wm8766.ctl[WM8766_CTL_CH2_VOL].name = clfe_volume;
    (*spec).wm8766.ctl[WM8766_CTL_CH3_VOL].name = null();
    (*spec).wm8766.ctl[WM8766_CTL_CH1_SW].name = rear_switch;
    (*spec).wm8766.ctl[WM8766_CTL_CH2_SW].name = clfe_switch;
    (*spec).wm8766.ctl[WM8766_CTL_CH3_SW].name = null();
    (*spec).wm8766.ctl[WM8766_CTL_PHASE1_SW].name = rear_phase;
    (*spec).wm8766.ctl[WM8766_CTL_PHASE2_SW].name = clfe_phase;
    (*spec).wm8766.ctl[WM8766_CTL_PHASE3_SW].name = null();
    (*spec).wm8766.ctl[WM8766_CTL_DEEMPH1_SW].name = rear_deemph;
    (*spec).wm8766.ctl[WM8766_CTL_DEEMPH2_SW].name = clfe_deemph;
    (*spec).wm8766.ctl[WM8766_CTL_DEEMPH3_SW].name = null();
    (*spec).wm8766.ctl[WM8766_CTL_IZD_SW].name = rear_clfe_izd;
    (*spec).wm8766.ctl[WM8766_CTL_ZC_SW].name = rear_clfe_zc;
    snd_wm8766_build_controls(&mut (*spec).wm8766);

    cont = zeroed();
    cont.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    i = 0;
    while i < psc724_cont.len() as c_int {
        cont.private_value = i as c_ulong;
        cont.name = psc724_cont[i as usize].name;
        cont.access = SNDRV_CTL_ELEM_ACCESS_READWRITE;
        cont.info = Some(snd_ctl_boolean_mono_info);
        cont.get = Some(psc724_ctl_get);
        cont.put = Some(psc724_ctl_put);
        ctl = snd_ctl_new1(&mut cont, ice as *mut _);
        if ctl.is_null() {
            return -ENOMEM;
        }
        err = snd_ctl_add((*ice).card, ctl);
        if err < 0 {
            return err;
        }
        i += 1;
    }

    0
}

unsafe fn psc724_set_pro_rate(ice: *mut snd_ice1712, _rate: c_uint) {
    let spec: *mut psc724_spec = (*ice).spec as *mut psc724_spec;
    /* restore codec volume settings after rate change (PMCLK stop) */
    snd_wm8776_volume_restore(&mut (*spec).wm8776);
    snd_wm8766_volume_restore(&mut (*spec).wm8766);
}

/* power management */

/* CONFIG_PM_SLEEP */
unsafe fn psc724_resume(ice: *mut snd_ice1712) -> c_int {
    let spec: *mut psc724_spec = (*ice).spec as *mut psc724_spec;

    snd_wm8776_resume(&mut (*spec).wm8776);
    snd_wm8766_resume(&mut (*spec).wm8766);

    0
}

/* init */

unsafe fn psc724_init(ice: *mut snd_ice1712) -> c_int {
    let mut spec: *mut psc724_spec;

    spec = kzalloc_obj::<psc724_spec>();
    if spec.is_null() {
        return -ENOMEM;
    }
    (*ice).spec = spec as *mut _;
    (*spec).ice = ice;

    (*ice).num_total_dacs = 6;
    (*ice).num_total_adcs = 2;
    (*spec).wm8776.ops.write = Some(psc724_wm8776_write);
    (*spec).wm8776.card = (*ice).card;
    snd_wm8776_init(&mut (*spec).wm8776);
    (*spec).wm8766.ops.write = Some(psc724_wm8766_write);
    (*spec).wm8766.card = (*ice).card;
    /* CONFIG_PM_SLEEP */
    (*ice).pm_resume = Some(psc724_resume);
    (*ice).pm_suspend_enabled = 1;
    snd_wm8766_init(&mut (*spec).wm8766);
    snd_wm8766_set_if(
        &mut (*spec).wm8766,
        WM8766_IF_FMT_I2S | WM8766_IF_IWL_24BIT,
    );
    (*ice).gpio.set_pro_rate = Some(psc724_set_pro_rate);
    INIT_DELAYED_WORK(&mut (*spec).hp_work, Some(psc724_update_hp_jack_state));
    psc724_set_jack_detection(ice, true);
    0
}

unsafe fn psc724_exit(ice: *mut snd_ice1712) {
    let spec: *mut psc724_spec = (*ice).spec as *mut psc724_spec;

    cancel_delayed_work_sync(&mut (*spec).hp_work);
}

/* PSC724 has buggy EEPROM (no 96&192kHz, all FFh GPIOs), so override it here */
static psc724_eeprom: [c_uchar; 32] = {
    let mut data = [0 as c_uchar; 32];
    data[ICE_EEP2_SYSCONF] = 0x42; /* 49.152MHz, 1 ADC, 3 DACs */
    data[ICE_EEP2_ACLINK] = 0x80; /* I2S */
    data[ICE_EEP2_I2S] = 0xf0; /* I2S volume, 96kHz, 24bit */
    data[ICE_EEP2_SPDIF] = 0xc1; /* spdif out-en, out-int, no input */
    /* GPIO outputs */
    data[ICE_EEP2_GPIO_DIR2] = 0x5f; /* MUTE_ALL,WM8766 MUTE/MODE/ML/MC/MD */
    /* GPIO write enable */
    data[ICE_EEP2_GPIO_MASK] = 0xff; /* read-only */
    data[ICE_EEP2_GPIO_MASK1] = 0xff; /* read-only */
    data[ICE_EEP2_GPIO_MASK2] = 0xa0; /* MUTE_ALL,WM8766 MUTE/MODE/ML/MC/MD */
    /* GPIO initial state */
    data[ICE_EEP2_GPIO_STATE2] = 0x20; /* unmuted, all WM8766 pins low */
    data
};

#[no_mangle]
pub static mut snd_vt1724_psc724_cards: [snd_ice1712_card_info; 2] = [
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_PSC724,
        name: c"Philips PSC724 Ultimate Edge".as_ptr(),
        model: c"psc724".as_ptr(),
        chip_init: Some(psc724_init),
        chip_exit: Some(psc724_exit),
        build_controls: Some(psc724_add_controls),
        eeprom_size: size_of::<[c_uchar; 32]>(),
        eeprom_data: psc724_eeprom.as_ptr(),
        ..unsafe { zeroed() }
    },
    unsafe { zeroed() }, /*terminator*/
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
