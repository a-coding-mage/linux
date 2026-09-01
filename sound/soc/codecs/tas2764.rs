// SPDX-License-Identifier: GPL-2.0
//
// Driver for the Texas Instruments TAS2764 CODEC
// Copyright (C) 2020 Texas Instruments Inc.

// Rust translation of tas2764.c. External Linux/ASoC/regmap symbols and
// tas2764.h / tas2764-quirks.h definitions are expected to be supplied by the
// surrounding repository bindings.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tas2764_devid {
    DEVID_TAS2764 = 0,
    DEVID_SN012776 = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tas2764_idle_slot_config {
    pub tx_mode: c_int,
    pub tx_mask: c_uint,
}

#[repr(C)]
pub struct tas2764_priv {
    pub component: *mut snd_soc_component,
    pub reset_gpio: *mut gpio_desc,
    pub sdz_gpio: *mut gpio_desc,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub irq: c_int,
    pub devid: tas2764_devid,

    pub v_sense_slot: c_int,
    pub i_sense_slot: c_int,

    pub dac_powered: bool,
    pub unmuted: bool,

    pub idle_slot_config: tas2764_idle_slot_config,
}

unsafe extern "C" {
    static tas2764_quirk_init_sequences: [tas2764_quirk_init_sequence; 0];
    static ENABLED_APPLE_QUIRKS: c_uint;

    fn tas2764_do_quirky_pwr_ctrl_change(tas2764: *mut tas2764_priv, val: c_uint) -> c_int;

    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_reinit_cache(map: *mut regmap, config: *const regmap_config) -> c_int;
    fn regmap_multi_reg_write(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn msleep(msecs: c_uint);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn dev_crit_ratelimited(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_ratelimited(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_request_threaded_irq(
        dev: *mut device,
        irq: c_int,
        handler: *mut c_void,
        thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        irqflags: c_ulong,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn fwnode_property_read_u32(fwnode: *mut fwnode_handle, propname: *const c_char, val: *mut c_int) -> c_int;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_hwmon_device_register_with_info(
        dev: *mut device,
        name: *const c_char,
        drvdata: *mut c_void,
        chip: *const hwmon_chip_info,
        groups: *mut c_void,
    ) -> *mut device;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

const fn BIT(nr: c_int) -> c_uint {
    1u32 << nr
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

unsafe fn __ffs(word: c_uint) -> c_int {
    word.trailing_zeros() as c_int
}

static tas2764_int_ltch0_msgs: [*const c_char; 8] = [
    c"fault: over temperature".as_ptr(), /* INT_LTCH0 & BIT(0) */
    c"fault: over current".as_ptr(),
    c"fault: bad TDM clock".as_ptr(),
    c"limiter active".as_ptr(),
    c"fault: PVDD below limiter inflection point".as_ptr(),
    c"fault: limiter max attenuation".as_ptr(),
    c"fault: BOP infinite hold".as_ptr(),
    c"fault: BOP mute".as_ptr(), /* INT_LTCH0 & BIT(7) */
];

static tas2764_int_readout_regs: [c_uint; 6] = [
    TAS2764_INT_LTCH0,
    TAS2764_INT_LTCH1,
    TAS2764_INT_LTCH1_0,
    TAS2764_INT_LTCH2,
    TAS2764_INT_LTCH3,
    TAS2764_INT_LTCH4,
];

unsafe extern "C" fn tas2764_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    let tas2764 = data as *mut tas2764_priv;
    let mut latched: [u8; 6] = [0, 0, 0, 0, 0, 0];
    let mut ret = IRQ_NONE;
    let mut i: c_int;

    i = 0;
    while (i as usize) < latched.len() {
        latched[i as usize] = snd_soc_component_read(
            (*tas2764).component,
            tas2764_int_readout_regs[i as usize],
        ) as u8;
        i += 1;
    }

    i = 0;
    while i < 8 {
        if (latched[0] as c_uint & BIT(i)) != 0 {
            dev_crit_ratelimited((*tas2764).dev, c"%s\n".as_ptr(), tas2764_int_ltch0_msgs[i as usize]);
            ret = IRQ_HANDLED;
        }
        i += 1;
    }

    if latched[0] != 0 {
        dev_err_ratelimited(
            (*tas2764).dev,
            c"other context to the fault: %02x,%02x,%02x,%02x,%02x".as_ptr(),
            latched[1] as c_int,
            latched[2] as c_int,
            latched[3] as c_int,
            latched[4] as c_int,
            latched[5] as c_int,
        );
        snd_soc_component_update_bits(
            (*tas2764).component,
            TAS2764_INT_CLK_CFG,
            TAS2764_INT_CLK_CFG_IRQZ_CLR,
            TAS2764_INT_CLK_CFG_IRQZ_CLR,
        );
    }

    ret
}

unsafe fn tas2764_reset(tas2764: *mut tas2764_priv) {
    if !(*tas2764).reset_gpio.is_null() {
        gpiod_set_value_cansleep((*tas2764).reset_gpio, 0);
        msleep(20);
        gpiod_set_value_cansleep((*tas2764).reset_gpio, 1);
        usleep_range(1000, 2000);
    }

    snd_soc_component_write((*tas2764).component, TAS2764_SW_RST, TAS2764_RST);
    usleep_range(1000, 2000);
}

unsafe fn tas2764_update_pwr_ctrl(tas2764: *mut tas2764_priv) -> c_int {
    let component = (*tas2764).component;
    let val: c_uint;
    let ret: c_int;

    if (*tas2764).dac_powered {
        val = if (*tas2764).unmuted {
            TAS2764_PWR_CTRL_ACTIVE
        } else {
            TAS2764_PWR_CTRL_MUTE
        };
    } else {
        val = TAS2764_PWR_CTRL_SHUTDOWN;
    }

    if (ENABLED_APPLE_QUIRKS & TAS2764_SHUTDOWN_DANCE) != 0 {
        return tas2764_do_quirky_pwr_ctrl_change(tas2764, val);
    }

    ret = snd_soc_component_update_bits(component, TAS2764_PWR_CTRL, TAS2764_PWR_CTRL_MASK, val);
    if ret < 0 {
        return ret;
    }

    0
}

/* CONFIG_PM: suspend/resume hooks are conditionally compiled in C. */
unsafe extern "C" fn tas2764_codec_suspend(component: *mut snd_soc_component) -> c_int {
    let tas2764 = snd_soc_component_get_drvdata(component) as *mut tas2764_priv;
    let ret: c_int;

    ret = snd_soc_component_update_bits(
        component,
        TAS2764_PWR_CTRL,
        TAS2764_PWR_CTRL_MASK,
        TAS2764_PWR_CTRL_SHUTDOWN,
    );

    if ret < 0 {
        return ret;
    }

    if !(*tas2764).sdz_gpio.is_null() {
        gpiod_set_value_cansleep((*tas2764).sdz_gpio, 0);
    }

    regcache_cache_only((*tas2764).regmap, true);
    regcache_mark_dirty((*tas2764).regmap);

    usleep_range(6000, 7000);

    0
}

unsafe extern "C" fn tas2764_codec_resume(component: *mut snd_soc_component) -> c_int {
    let tas2764 = snd_soc_component_get_drvdata(component) as *mut tas2764_priv;
    let ret: c_int;

    if !(*tas2764).sdz_gpio.is_null() {
        gpiod_set_value_cansleep((*tas2764).sdz_gpio, 1);
        usleep_range(1000, 2000);
    }

    ret = tas2764_update_pwr_ctrl(tas2764);

    if ret < 0 {
        return ret;
    }

    regcache_cache_only((*tas2764).regmap, false);

    regcache_sync((*tas2764).regmap)
}

static tas2764_ASI1_src: [*const c_char; 4] = [
    c"I2C offset".as_ptr(),
    c"Left".as_ptr(),
    c"Right".as_ptr(),
    c"LeftRightDiv2".as_ptr(),
];

static tas2764_ASI1_src_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL(TAS2764_TDM_CFG2, TAS2764_TDM_CFG2_SCFG_SHIFT, &tas2764_ASI1_src);

static tas2764_asi1_mux: snd_kcontrol_new = SOC_DAPM_ENUM(c"ASI1 Source".as_ptr(), &tas2764_ASI1_src_enum);

static isense_switch: snd_kcontrol_new =
    SOC_DAPM_SINGLE(c"Switch".as_ptr(), TAS2764_PWR_CTRL, TAS2764_ISENSE_POWER_EN, 1, 1);
static vsense_switch: snd_kcontrol_new =
    SOC_DAPM_SINGLE(c"Switch".as_ptr(), TAS2764_PWR_CTRL, TAS2764_VSENSE_POWER_EN, 1, 1);

static tas2764_dapm_widgets: [snd_soc_dapm_widget; 8] = [
    SND_SOC_DAPM_AIF_IN(c"ASI1".as_ptr(), c"ASI1 Playback".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_MUX(c"ASI1 Sel".as_ptr(), SND_SOC_NOPM, 0, 0, &tas2764_asi1_mux),
    SND_SOC_DAPM_SWITCH(c"ISENSE".as_ptr(), TAS2764_PWR_CTRL, TAS2764_ISENSE_POWER_EN, 1, &isense_switch),
    SND_SOC_DAPM_SWITCH(c"VSENSE".as_ptr(), TAS2764_PWR_CTRL, TAS2764_VSENSE_POWER_EN, 1, &vsense_switch),
    SND_SOC_DAPM_DAC(c"DAC".as_ptr(), ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_OUTPUT(c"OUT".as_ptr()),
    SND_SOC_DAPM_SIGGEN(c"VMON".as_ptr()),
    SND_SOC_DAPM_SIGGEN(c"IMON".as_ptr()),
];

static tas2764_audio_map: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route { sink: c"ASI1 Sel".as_ptr(), control: c"I2C offset".as_ptr(), source: c"ASI1".as_ptr() },
    snd_soc_dapm_route { sink: c"ASI1 Sel".as_ptr(), control: c"Left".as_ptr(), source: c"ASI1".as_ptr() },
    snd_soc_dapm_route { sink: c"ASI1 Sel".as_ptr(), control: c"Right".as_ptr(), source: c"ASI1".as_ptr() },
    snd_soc_dapm_route { sink: c"ASI1 Sel".as_ptr(), control: c"LeftRightDiv2".as_ptr(), source: c"ASI1".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC".as_ptr(), control: ptr::null(), source: c"ASI1 Sel".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT".as_ptr(), control: ptr::null(), source: c"DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"ISENSE".as_ptr(), control: c"Switch".as_ptr(), source: c"IMON".as_ptr() },
    snd_soc_dapm_route { sink: c"VSENSE".as_ptr(), control: c"Switch".as_ptr(), source: c"VMON".as_ptr() },
];

unsafe extern "C" fn tas2764_mute(dai: *mut snd_soc_dai, mute: c_int, direction: c_int) -> c_int {
    let tas2764 = snd_soc_component_get_drvdata((*dai).component) as *mut tas2764_priv;
    let mut ret: c_int;

    if mute == 0 {
        (*tas2764).dac_powered = true;
        ret = tas2764_update_pwr_ctrl(tas2764);
        if ret != 0 {
            return ret;
        }
    }

    (*tas2764).unmuted = mute == 0;
    ret = tas2764_update_pwr_ctrl(tas2764);
    if ret != 0 {
        return ret;
    }

    if mute != 0 {
        /* Wait for ramp-down */
        usleep_range(6000, 7000);

        (*tas2764).dac_powered = false;
        ret = tas2764_update_pwr_ctrl(tas2764);
        if ret != 0 {
            return ret;
        }

        /* Wait a bit after shutdown */
        usleep_range(2000, 3000);
    }

    0
}

unsafe fn tas2764_set_bitwidth(tas2764: *mut tas2764_priv, bitwidth: c_int) -> c_int {
    let component = (*tas2764).component;
    let mut sense_en: c_int;
    let val: c_int;
    let mut ret: c_int;

    match bitwidth {
        SNDRV_PCM_FORMAT_S16_LE => {
            ret = snd_soc_component_update_bits(
                component,
                TAS2764_TDM_CFG2,
                TAS2764_TDM_CFG2_RXW_MASK,
                TAS2764_TDM_CFG2_RXW_16BITS,
            );
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            ret = snd_soc_component_update_bits(
                component,
                TAS2764_TDM_CFG2,
                TAS2764_TDM_CFG2_RXW_MASK,
                TAS2764_TDM_CFG2_RXW_24BITS,
            );
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            ret = snd_soc_component_update_bits(
                component,
                TAS2764_TDM_CFG2,
                TAS2764_TDM_CFG2_RXW_MASK,
                TAS2764_TDM_CFG2_RXW_32BITS,
            );
        }
        _ => return -EINVAL,
    }

    if ret < 0 {
        return ret;
    }

    val = snd_soc_component_read((*tas2764).component, TAS2764_PWR_CTRL);
    if val < 0 {
        return val;
    }

    if (val & (1 << TAS2764_VSENSE_POWER_EN)) != 0 {
        sense_en = 0;
    } else {
        sense_en = TAS2764_TDM_CFG5_VSNS_ENABLE as c_int;
    }

    ret = snd_soc_component_update_bits(
        (*tas2764).component,
        TAS2764_TDM_CFG5,
        TAS2764_TDM_CFG5_VSNS_ENABLE,
        sense_en as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    if (val & (1 << TAS2764_ISENSE_POWER_EN)) != 0 {
        sense_en = 0;
    } else {
        sense_en = TAS2764_TDM_CFG6_ISNS_ENABLE as c_int;
    }

    ret = snd_soc_component_update_bits(
        (*tas2764).component,
        TAS2764_TDM_CFG6,
        TAS2764_TDM_CFG6_ISNS_ENABLE,
        sense_en as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe fn tas2764_set_samplerate(tas2764: *mut tas2764_priv, samplerate: c_int) -> c_int {
    let component = (*tas2764).component;
    let ramp_rate_val: c_int;
    let ret: c_int;

    match samplerate {
        48000 => ramp_rate_val = (TAS2764_TDM_CFG0_SMP_48KHZ | TAS2764_TDM_CFG0_44_1_48KHZ) as c_int,
        44100 => ramp_rate_val = (TAS2764_TDM_CFG0_SMP_44_1KHZ | TAS2764_TDM_CFG0_44_1_48KHZ) as c_int,
        96000 => ramp_rate_val = (TAS2764_TDM_CFG0_SMP_48KHZ | TAS2764_TDM_CFG0_88_2_96KHZ) as c_int,
        88200 => ramp_rate_val = (TAS2764_TDM_CFG0_SMP_44_1KHZ | TAS2764_TDM_CFG0_88_2_96KHZ) as c_int,
        _ => return -EINVAL,
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2764_TDM_CFG0,
        TAS2764_TDM_CFG0_SMP_MASK | TAS2764_TDM_CFG0_MASK,
        ramp_rate_val as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn tas2764_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let tas2764 = snd_soc_component_get_drvdata(component) as *mut tas2764_priv;
    let ret: c_int;

    ret = tas2764_set_bitwidth(tas2764, params_format(params));
    if ret < 0 {
        return ret;
    }

    tas2764_set_samplerate(tas2764, params_rate(params))
}

unsafe extern "C" fn tas2764_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let tas2764 = snd_soc_component_get_drvdata(component) as *mut tas2764_priv;
    let mut tdm_rx_start_slot: u8 = 0;
    let mut asi_cfg_0: u8 = 0;
    let mut asi_cfg_1: u8 = 0;
    let mut asi_cfg_4: u8 = 0;
    let mut ret: c_int;

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_IF => {
            asi_cfg_0 ^= TAS2764_TDM_CFG0_FRAME_START as u8;
            asi_cfg_1 = TAS2764_TDM_CFG1_RX_RISING as u8;
            asi_cfg_4 = TAS2764_TDM_CFG4_TX_FALLING as u8;
        }
        SND_SOC_DAIFMT_NB_NF => {
            asi_cfg_1 = TAS2764_TDM_CFG1_RX_RISING as u8;
            asi_cfg_4 = TAS2764_TDM_CFG4_TX_FALLING as u8;
        }
        SND_SOC_DAIFMT_IB_IF => {
            asi_cfg_0 ^= TAS2764_TDM_CFG0_FRAME_START as u8;
            asi_cfg_1 = TAS2764_TDM_CFG1_RX_FALLING as u8;
            asi_cfg_4 = TAS2764_TDM_CFG4_TX_RISING as u8;
        }
        SND_SOC_DAIFMT_IB_NF => {
            asi_cfg_1 = TAS2764_TDM_CFG1_RX_FALLING as u8;
            asi_cfg_4 = TAS2764_TDM_CFG4_TX_RISING as u8;
        }
        _ => {}
    }

    ret = snd_soc_component_update_bits(component, TAS2764_TDM_CFG1, TAS2764_TDM_CFG1_RX_MASK, asi_cfg_1 as c_uint);
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_component_update_bits(component, TAS2764_TDM_CFG4, TAS2764_TDM_CFG4_TX_MASK, asi_cfg_4 as c_uint);
    if ret < 0 {
        return ret;
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            asi_cfg_0 ^= TAS2764_TDM_CFG0_FRAME_START as u8;
            tdm_rx_start_slot = 1;
        }
        SND_SOC_DAIFMT_DSP_A => {
            tdm_rx_start_slot = 1;
        }
        SND_SOC_DAIFMT_DSP_B | SND_SOC_DAIFMT_LEFT_J => {
            tdm_rx_start_slot = 0;
        }
        _ => {
            dev_err((*tas2764).dev, c"DAI Format is not found, fmt=0x%x\n".as_ptr(), fmt);
            return -EINVAL;
        }
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2764_TDM_CFG0,
        TAS2764_TDM_CFG0_FRAME_START,
        asi_cfg_0 as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2764_TDM_CFG1,
        TAS2764_TDM_CFG1_MASK,
        (tdm_rx_start_slot as c_uint) << TAS2764_TDM_CFG1_51_SHIFT,
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn tas2764_set_dai_tdm_slot(
    dai: *mut snd_soc_dai,
    mut tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let tas2764 = snd_soc_component_get_drvdata(component) as *mut tas2764_priv;
    let left_slot: c_int;
    let right_slot: c_int;
    let slots_cfg: c_int;
    let slot_size: c_int;
    let mut ret: c_int;

    if tx_mask == 0 || rx_mask != 0 {
        return -EINVAL;
    }

    left_slot = __ffs(tx_mask);
    tx_mask &= !(1 << left_slot);
    if tx_mask == 0 {
        right_slot = left_slot;
    } else {
        right_slot = __ffs(tx_mask);
        tx_mask &= !(1 << right_slot);
    }

    if tx_mask != 0 || left_slot >= slots || right_slot >= slots {
        return -EINVAL;
    }

    slots_cfg = (right_slot << TAS2764_TDM_CFG3_RXS_SHIFT) | left_slot;

    ret = snd_soc_component_write(component, TAS2764_TDM_CFG3, slots_cfg as c_uint);
    if ret != 0 {
        return ret;
    }

    match slot_width {
        16 => slot_size = TAS2764_TDM_CFG2_RXS_16BITS as c_int,
        24 => slot_size = TAS2764_TDM_CFG2_RXS_24BITS as c_int,
        32 => slot_size = TAS2764_TDM_CFG2_RXS_32BITS as c_int,
        _ => return -EINVAL,
    }

    ret = snd_soc_component_update_bits(component, TAS2764_TDM_CFG2, TAS2764_TDM_CFG2_RXS_MASK, slot_size as c_uint);
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2764_TDM_CFG5,
        TAS2764_TDM_CFG5_50_MASK,
        (*tas2764).v_sense_slot as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2764_TDM_CFG6,
        TAS2764_TDM_CFG6_50_MASK,
        (*tas2764).i_sense_slot as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe fn tas2764_write_sdout_idle_mask(tas2764: *mut tas2764_priv, mask: u32) -> c_int {
    let component = (*tas2764).component;
    let mut i: c_int;
    let mut ret: c_int;

    /* Hardware supports up to 64 slots, but we don't */
    i = 0;
    while i < 4 {
        ret = snd_soc_component_write(component, TAS2764_SDOUT_HIZ_1 + i as c_uint, (mask >> (i * 8)) & 0xff);
        if ret < 0 {
            return ret;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn tas2764_set_dai_tdm_idle(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    tx_mode: c_int,
    rx_mode: c_int,
) -> c_int {
    let component = (*dai).component;
    let tas2764 = snd_soc_component_get_drvdata(component) as *mut tas2764_priv;
    let mut ret: c_int;

    /* We don't support setting anything on SDIN */
    if rx_mode != 0 {
        return -EOPNOTSUPP;
    }

    if (*tas2764).idle_slot_config.tx_mask == tx_mask && (*tas2764).idle_slot_config.tx_mode == tx_mode {
        return 0;
    }

    match tx_mode {
        SND_SOC_DAI_TDM_IDLE_ZERO => {
            if tx_mask == 0 {
                return -EINVAL;
            }

            ret = tas2764_write_sdout_idle_mask(tas2764, tx_mask);
            if ret < 0 {
                return ret;
            }

            ret = snd_soc_component_update_bits(
                component,
                TAS2764_SDOUT_HIZ_9,
                TAS2764_SDOUT_HIZ_9_FORCE_0_EN,
                TAS2764_SDOUT_HIZ_9_FORCE_0_EN,
            );
            if ret < 0 {
                return ret;
            }

            (*tas2764).idle_slot_config.tx_mask = tx_mask;
            (*tas2764).idle_slot_config.tx_mode = tx_mode;
        }
        SND_SOC_DAI_TDM_IDLE_HIZ | SND_SOC_DAI_TDM_IDLE_OFF => {
            /* HiZ mode does not support a slot mask */
            ret = tas2764_write_sdout_idle_mask(tas2764, 0);
            if ret < 0 {
                return ret;
            }

            ret = snd_soc_component_update_bits(
                component,
                TAS2764_SDOUT_HIZ_9,
                TAS2764_SDOUT_HIZ_9_FORCE_0_EN,
                0,
            );
            if ret < 0 {
                return ret;
            }

            (*tas2764).idle_slot_config.tx_mask = 0;
            (*tas2764).idle_slot_config.tx_mode = tx_mode;
        }
        _ => return -EOPNOTSUPP,
    }

    0
}

/* The SDOUT idle slot mask must be cropped based on the BCLK ratio */
unsafe extern "C" fn tas2764_set_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int {
    let tas2764 = snd_soc_component_get_drvdata((*dai).component) as *mut tas2764_priv;

    if (*tas2764).idle_slot_config.tx_mask == 0 {
        return 0;
    }

    (*tas2764).idle_slot_config.tx_mask &= GENMASK((ratio / 8) - 1, 0);

    tas2764_write_sdout_idle_mask(tas2764, (*tas2764).idle_slot_config.tx_mask)
}

static tas2764_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    mute_stream: Some(tas2764_mute),
    hw_params: Some(tas2764_hw_params),
    set_fmt: Some(tas2764_set_fmt),
    set_bclk_ratio: Some(tas2764_set_bclk_ratio),
    set_tdm_slot: Some(tas2764_set_dai_tdm_slot),
    set_tdm_idle: Some(tas2764_set_dai_tdm_idle),
    no_capture_mute: 1,
};

const TAS2764_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

const TAS2764_RATES: c_uint =
    SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_88200;

static mut tas2764_dai_driver: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"tas2764 ASI1".as_ptr(),
    id: 0,
    playback: snd_soc_pcm_stream {
        stream_name: c"ASI1 Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: TAS2764_RATES,
        formats: TAS2764_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"ASI1 Capture".as_ptr(),
        channels_min: 0,
        channels_max: 2,
        rates: TAS2764_RATES,
        formats: TAS2764_FORMATS,
    },
    ops: &tas2764_dai_ops,
    symmetric_rate: 1,
}];

static mut sn012776_bop_presets: [u8; 24] = [
    0x01, 0x32, 0x02, 0x22, 0x83, 0x2d, 0x80, 0x02, 0x06, 0x32, 0x46, 0x30, 0x02, 0x06, 0x38,
    0x40, 0x30, 0x02, 0x06, 0x3e, 0x37, 0x30, 0xff, 0xe6,
];

unsafe fn tas2764_apply_init_quirks(tas2764: *mut tas2764_priv) -> c_int {
    let mut ret: c_int;
    let mut i: c_int;

    i = 0;
    while (i as usize) < tas2764_quirk_init_sequences.len() {
        let init_seq: *const tas2764_quirk_init_sequence = &tas2764_quirk_init_sequences[i as usize];

        if (*init_seq).seq.is_null() {
            i += 1;
            continue;
        }

        if (BIT(i) & ENABLED_APPLE_QUIRKS) == 0 {
            i += 1;
            continue;
        }

        ret = regmap_multi_reg_write((*tas2764).regmap, (*init_seq).seq, (*init_seq).len);

        if ret < 0 {
            return ret;
        }
        i += 1;
    }

    0
}

unsafe fn tas2764_read_die_temp(tas2764: *mut tas2764_priv, result: *mut c_long) -> c_int {
    let ret: c_int;
    let mut reg: c_int = 0;

    ret = regmap_read((*tas2764).regmap, TAS2764_TEMP, &mut reg);
    if ret != 0 {
        return ret;
    }
    /*
     * As per datasheet, subtract 93 from raw value to get degrees
     * Celsius. hwmon wants millidegrees.
     *
     * NOTE: The TAS2764 datasheet mentions initialising TAS2764_TEMP
     * such that the temperature is 2.6 *C, however the register
     * is actually initialised to 0. The ADC is also powered down during
     * software shutdown. The last sampled temperature will persist
     * in the register while the amp is in this power state.
     */
    if reg == 0 {
        return -ENODATA;
    }

    *result = ((reg - 93) * 1000) as c_long;
    0
}

unsafe fn tas2764_hwmon_is_fault(tas2764: *mut tas2764_priv, result: *mut c_long) -> c_int {
    let ret: c_int;
    let mut temp: c_long = 0;

    ret = tas2764_read_die_temp(tas2764, &mut temp);
    if ret == -ENODATA {
        *result = true as c_long;
        return 0;
    }

    ret
}

unsafe extern "C" fn tas2764_hwmon_is_visible(
    data: *const c_void,
    type_: hwmon_sensor_types,
    attr: u32,
    channel: c_int,
) -> umode_t {
    if type_ != hwmon_sensor_types::hwmon_temp {
        return 0;
    }

    match attr {
        hwmon_temp_input | hwmon_temp_fault => return 0o444,
        _ => {}
    }

    0
}

unsafe extern "C" fn tas2764_hwmon_read(
    dev: *mut device,
    type_: hwmon_sensor_types,
    attr: u32,
    channel: c_int,
    val: *mut c_long,
) -> c_int {
    let tas2764 = dev_get_drvdata(dev) as *mut tas2764_priv;
    let ret: c_int;

    match attr {
        hwmon_temp_input => {
            ret = tas2764_read_die_temp(tas2764, val);
        }
        hwmon_temp_fault => {
            ret = tas2764_hwmon_is_fault(tas2764, val);
        }
        _ => {
            ret = -EOPNOTSUPP;
        }
    }

    ret
}

static tas2764_hwmon_info: [*const hwmon_channel_info; 2] = [
    HWMON_CHANNEL_INFO(HWMON_SENSOR_TYPE_TEMP, HWMON_T_INPUT | HWMON_T_FAULT),
    ptr::null(),
];

static tas2764_hwmon_ops: hwmon_ops = hwmon_ops {
    is_visible: Some(tas2764_hwmon_is_visible),
    read: Some(tas2764_hwmon_read),
};

static tas2764_hwmon_chip_info: hwmon_chip_info = hwmon_chip_info {
    ops: &tas2764_hwmon_ops,
    info: tas2764_hwmon_info.as_ptr(),
};

unsafe extern "C" fn tas2764_codec_probe(component: *mut snd_soc_component) -> c_int {
    let tas2764 = snd_soc_component_get_drvdata(component) as *mut tas2764_priv;
    let mut ret: c_int;
    let mut i: c_int;

    (*tas2764).component = component;

    if !(*tas2764).sdz_gpio.is_null() {
        gpiod_set_value_cansleep((*tas2764).sdz_gpio, 1);
        usleep_range(1000, 2000);
    }

    tas2764_reset(tas2764);
    regcache_reinit_cache((*tas2764).regmap, &tas2764_i2c_regmap);

    if (*tas2764).irq != 0 {
        ret = snd_soc_component_write((*tas2764).component, TAS2764_INT_MASK0, 0x00);
        if ret < 0 {
            return ret;
        }

        ret = snd_soc_component_write((*tas2764).component, TAS2764_INT_MASK1, 0xff);
        if ret < 0 {
            return ret;
        }

        ret = snd_soc_component_write((*tas2764).component, TAS2764_INT_MASK2, 0xff);
        if ret < 0 {
            return ret;
        }

        ret = snd_soc_component_write((*tas2764).component, TAS2764_INT_MASK3, 0xff);
        if ret < 0 {
            return ret;
        }

        ret = snd_soc_component_write((*tas2764).component, TAS2764_INT_MASK4, 0xff);
        if ret < 0 {
            return ret;
        }

        ret = devm_request_threaded_irq(
            (*tas2764).dev,
            (*tas2764).irq,
            ptr::null_mut(),
            tas2764_irq,
            IRQF_ONESHOT | IRQF_SHARED | IRQF_TRIGGER_LOW,
            c"tas2764".as_ptr(),
            tas2764 as *mut c_void,
        );
        if ret != 0 {
            dev_warn((*tas2764).dev, c"failed to request IRQ: %d\n".as_ptr(), ret);
        }
    }

    ret = snd_soc_component_update_bits(
        (*tas2764).component,
        TAS2764_TDM_CFG5,
        TAS2764_TDM_CFG5_VSNS_ENABLE,
        0,
    );
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_component_update_bits(
        (*tas2764).component,
        TAS2764_TDM_CFG6,
        TAS2764_TDM_CFG6_ISNS_ENABLE,
        0,
    );
    if ret < 0 {
        return ret;
    }

    match (*tas2764).devid {
        tas2764_devid::DEVID_SN012776 => {
            ret = snd_soc_component_update_bits(
                component,
                TAS2764_PWR_CTRL,
                TAS2764_PWR_CTRL_BOP_SRC,
                TAS2764_PWR_CTRL_BOP_SRC,
            );
            if ret < 0 {
                return ret;
            }

            i = 0;
            while (i as usize) < sn012776_bop_presets.len() {
                ret = snd_soc_component_write(
                    component,
                    TAS2764_BOP_CFG0 + i as c_uint,
                    sn012776_bop_presets[i as usize] as c_uint,
                );

                if ret < 0 {
                    return ret;
                }
                i += 1;
            }

            /* Apply all enabled Apple quirks */
            ret = tas2764_apply_init_quirks(tas2764);

            if ret < 0 {
                return ret;
            }
        }
        _ => {}
    }

    0
}

static tas2764_digital_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE(1100, 50, 0);
static tas2764_playback_volume: [c_uint; 4] = DECLARE_TLV_DB_SCALE(-10050, 50, 1);

static tas2764_hpf_texts: [*const c_char; 7] = [
    c"Disabled".as_ptr(),
    c"2 Hz".as_ptr(),
    c"50 Hz".as_ptr(),
    c"100 Hz".as_ptr(),
    c"200 Hz".as_ptr(),
    c"400 Hz".as_ptr(),
    c"800 Hz".as_ptr(),
];

static tas2764_hpf_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL(TAS2764_DC_BLK0, TAS2764_DC_BLK0_HPF_FREQ_PB_SHIFT, &tas2764_hpf_texts);

static tas2764_oce_texts: [*const c_char; 2] = [c"Disable".as_ptr(), c"Retry".as_ptr()];

static tas2764_oce_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL(TAS2764_MISC_CFG1, TAS2764_MISC_CFG1_OCE_RETRY_SHIFT, &tas2764_oce_texts);

static tas2764_snd_controls: [snd_kcontrol_new; 4] = [
    SOC_SINGLE_TLV(
        c"Speaker Volume".as_ptr(),
        TAS2764_DVC,
        0,
        TAS2764_DVC_MAX,
        1,
        tas2764_playback_volume.as_ptr(),
    ),
    SOC_SINGLE_TLV(
        c"Amp Gain Volume".as_ptr(),
        TAS2764_CHNL_0,
        1,
        0x14,
        0,
        tas2764_digital_tlv.as_ptr(),
    ),
    SOC_ENUM(c"HPF Corner Frequency".as_ptr(), &tas2764_hpf_enum),
    SOC_ENUM(c"OCE Handling".as_ptr(), &tas2764_oce_enum),
];

static soc_component_driver_tas2764: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(tas2764_codec_probe),
    suspend: Some(tas2764_codec_suspend),
    resume: Some(tas2764_codec_resume),
    controls: tas2764_snd_controls.as_ptr(),
    num_controls: tas2764_snd_controls.len() as c_uint,
    dapm_widgets: tas2764_dapm_widgets.as_ptr(),
    num_dapm_widgets: tas2764_dapm_widgets.len() as c_uint,
    dapm_routes: tas2764_audio_map.as_ptr(),
    num_dapm_routes: tas2764_audio_map.len() as c_uint,
    idle_bias_on: 1,
    endianness: 1,
};

static tas2764_reg_defaults: [reg_default; 11] = [
    reg_default { reg: TAS2764_PAGE, def: 0x00 },
    reg_default { reg: TAS2764_SW_RST, def: 0x00 },
    reg_default { reg: TAS2764_PWR_CTRL, def: 0x1a },
    reg_default { reg: TAS2764_CHNL_0, def: 0x28 },
    reg_default { reg: TAS2764_TDM_CFG0, def: 0x09 },
    reg_default { reg: TAS2764_TDM_CFG1, def: 0x02 },
    reg_default { reg: TAS2764_TDM_CFG2, def: 0x0a },
    reg_default { reg: TAS2764_TDM_CFG3, def: 0x10 },
    reg_default { reg: TAS2764_TDM_CFG5, def: 0x42 },
    reg_default { reg: TAS2764_DVC, def: 0x00 },
    reg_default { reg: TAS2764_INT_CLK_CFG, def: 0x19 },
];

static tas2764_regmap_ranges: [regmap_range_cfg; 1] = [regmap_range_cfg {
    range_min: 0,
    range_max: 0xffff,
    selector_reg: TAS2764_PAGE,
    selector_mask: 0xff,
    selector_shift: 0,
    window_start: 0,
    window_len: 128,
}];

unsafe extern "C" fn tas2764_volatile_register(dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TAS2764_SW_RST | TAS2764_TEMP | TAS2764_INT_CLK_CFG => true,
        TAS2764_INT_LTCH0..=TAS2764_INT_LTCH4 => true,
        x if x >= TAS2764_REG(0xf0, 0x0) && x <= TAS2764_REG(0xff, 0x0) => {
            /* TI's undocumented registers for the application of quirks */
            true
        }
        _ => false,
    }
}

static tas2764_i2c_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    volatile_reg: Some(tas2764_volatile_register),
    reg_defaults: tas2764_reg_defaults.as_ptr(),
    num_reg_defaults: tas2764_reg_defaults.len() as c_uint,
    cache_type: REGCACHE_RBTREE,
    ranges: tas2764_regmap_ranges.as_ptr(),
    num_ranges: tas2764_regmap_ranges.len() as c_uint,
    max_register: 0xffff,
};

unsafe fn tas2764_parse_dt(dev: *mut device, tas2764: *mut tas2764_priv) -> c_int {
    let mut ret: c_int = 0;

    (*tas2764).reset_gpio = devm_gpiod_get_optional((*tas2764).dev, c"reset".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*tas2764).reset_gpio as *const c_void) {
        if PTR_ERR((*tas2764).reset_gpio as *const c_void) == -EPROBE_DEFER {
            (*tas2764).reset_gpio = ptr::null_mut();
            return -EPROBE_DEFER;
        }
    }

    (*tas2764).sdz_gpio = devm_gpiod_get_optional(dev, c"shutdown".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*tas2764).sdz_gpio as *const c_void) {
        if PTR_ERR((*tas2764).sdz_gpio as *const c_void) == -EPROBE_DEFER {
            return -EPROBE_DEFER;
        }

        (*tas2764).sdz_gpio = ptr::null_mut();
    }

    ret = fwnode_property_read_u32((*dev).fwnode, c"ti,imon-slot-no".as_ptr(), &mut (*tas2764).i_sense_slot);
    if ret != 0 {
        (*tas2764).i_sense_slot = 0;
    }

    ret = fwnode_property_read_u32((*dev).fwnode, c"ti,vmon-slot-no".as_ptr(), &mut (*tas2764).v_sense_slot);
    if ret != 0 {
        (*tas2764).v_sense_slot = 2;
    }

    0
}

unsafe extern "C" fn tas2764_i2c_probe(client: *mut i2c_client) -> c_int {
    let tas2764: *mut tas2764_priv;
    let mut result: c_int;

    tas2764 = devm_kzalloc(&mut (*client).dev, core::mem::size_of::<tas2764_priv>(), GFP_KERNEL) as *mut tas2764_priv;
    if tas2764.is_null() {
        return -ENOMEM;
    }

    (*tas2764).devid = core::mem::transmute::<c_ulong, tas2764_devid>(of_device_get_match_data(&mut (*client).dev) as c_ulong);

    (*tas2764).dev = &mut (*client).dev;
    (*tas2764).irq = (*client).irq;
    i2c_set_clientdata(client, tas2764 as *mut c_void);
    dev_set_drvdata(&mut (*client).dev, tas2764 as *mut c_void);

    (*tas2764).regmap = devm_regmap_init_i2c(client, &tas2764_i2c_regmap);
    if IS_ERR((*tas2764).regmap as *const c_void) {
        result = PTR_ERR((*tas2764).regmap as *const c_void);
        dev_err(&mut (*client).dev, c"Failed to allocate register map: %d\n".as_ptr(), result);
        return result;
    }

    if !(*client).dev.of_node.is_null() {
        result = tas2764_parse_dt(&mut (*client).dev, tas2764);
        if result != 0 {
            dev_err((*tas2764).dev, c"%s: Failed to parse devicetree\n".as_ptr(), c"tas2764_i2c_probe".as_ptr());
            return result;
        }
    }

    if IS_REACHABLE(CONFIG_HWMON) {
        let hwmon: *mut device;

        hwmon = devm_hwmon_device_register_with_info(
            &mut (*client).dev,
            c"tas2764".as_ptr(),
            tas2764 as *mut c_void,
            &tas2764_hwmon_chip_info,
            ptr::null_mut(),
        );
        if IS_ERR(hwmon as *const c_void) {
            return dev_err_probe(
                &mut (*client).dev,
                PTR_ERR(hwmon as *const c_void),
                c"Failed to register temp sensor\n".as_ptr(),
            );
        }
    }

    devm_snd_soc_register_component(
        (*tas2764).dev,
        &soc_component_driver_tas2764,
        tas2764_dai_driver.as_mut_ptr(),
        tas2764_dai_driver.len() as c_int,
    )
}

static tas2764_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: c"tas2764".as_ptr(), driver_data: 0 },
    i2c_device_id { name: ptr::null(), driver_data: 0 },
];

/* MODULE_DEVICE_TABLE(i2c, tas2764_i2c_id); */

/* CONFIG_OF */
static tas2764_of_match: [of_device_id; 3] = [
    of_device_id {
        compatible: c"ti,tas2764".as_ptr(),
        data: tas2764_devid::DEVID_TAS2764 as usize as *const c_void,
    },
    of_device_id {
        compatible: c"ti,sn012776".as_ptr(),
        data: tas2764_devid::DEVID_SN012776 as usize as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, tas2764_of_match); */

static mut tas2764_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"tas2764".as_ptr(),
        of_match_table: of_match_ptr(tas2764_of_match.as_ptr()),
    },
    probe: Some(tas2764_i2c_probe),
    id_table: tas2764_i2c_id.as_ptr(),
};

module_i2c_driver!(tas2764_i2c_driver);

MODULE_AUTHOR!("Dan Murphy <dmurphy@ti.com>");
MODULE_DESCRIPTION!("TAS2764 I2C Smart Amplifier driver");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
