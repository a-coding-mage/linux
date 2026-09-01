// SPDX-License-Identifier: GPL-2.0+
//
// Machine driver for AMD ACP Audio engine using ES8336 codec.
//
// Copyright 2023 Marian Postevca <posteuca@mutex.one>
//
// Translated from C implementation source. C include dependencies are expected
// to be supplied by the surrounding kernel bindings.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const DUAL_CHANNEL: c_uint = 2;

const ES83XX_ENABLE_DMIC: c_ulong = 1 << 4;
const ES83XX_48_MHZ_MCLK: c_ulong = 1 << 5;

const ES83XX_12288_KHZ_MCLK_FREQ: c_uint = 48000 * 256;
const ES83XX_48_MHZ_MCLK_FREQ: c_uint = 48000 * 1000;

#[repr(C)]
pub struct acp3x_es83xx_private {
    speaker_on: bool,
    headphone_on: bool,
    quirk: c_ulong,
    codec: *mut snd_soc_component,
    codec_dev: *mut device,
    gpio_speakers: *mut gpio_desc,
    gpio_headphone: *mut gpio_desc,
    enable_spk_gpio: acpi_gpio_params,
    enable_hp_gpio: acpi_gpio_params,
    gpio_mapping: [acpi_gpio_mapping; 3],
    mic_map: [snd_soc_dapm_route; 2],
}

static channels: [c_uint; 1] = [DUAL_CHANNEL];

static constraints_channels: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: channels.len() as c_uint,
    list: channels.as_ptr(),
    mask: 0,
};

static mut es83xx_jack: snd_soc_jack = unsafe { core::mem::zeroed() };

static mut es83xx_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

// C macro initializers: SND_SOC_DAPM_SPK/HP/MIC/SUPPLY.
static acp3x_es83xx_widgets: [snd_soc_dapm_widget; 6] = [
    SND_SOC_DAPM_SPK(c"Speaker".as_ptr(), ptr::null()),
    SND_SOC_DAPM_HP(c"Headphone".as_ptr(), ptr::null()),
    SND_SOC_DAPM_MIC(c"Headset Mic".as_ptr(), ptr::null()),
    SND_SOC_DAPM_MIC(c"Internal Mic".as_ptr(), ptr::null()),
    SND_SOC_DAPM_SUPPLY(
        c"Headphone Power".as_ptr(),
        SND_SOC_NOPM,
        0,
        0,
        Some(acp3x_es83xx_headphone_power_event),
        SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU,
    ),
    SND_SOC_DAPM_SUPPLY(
        c"Speaker Power".as_ptr(),
        SND_SOC_NOPM,
        0,
        0,
        Some(acp3x_es83xx_speaker_power_event),
        SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU,
    ),
];

static acp3x_es83xx_audio_map: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route {
        sink: c"Headphone".as_ptr(),
        control: ptr::null(),
        source: c"HPOL".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headphone".as_ptr(),
        control: ptr::null(),
        source: c"HPOR".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headphone".as_ptr(),
        control: ptr::null(),
        source: c"Headphone Power".as_ptr(),
    },

    /*
     * There is no separate speaker output instead the speakers are muxed to
     * the HP outputs. The mux is controlled Speaker and/or headphone switch.
     */
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: ptr::null(),
        source: c"HPOL".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: ptr::null(),
        source: c"HPOR".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: ptr::null(),
        source: c"Speaker Power".as_ptr(),
    },
];

// C macro initializers: SOC_DAPM_PIN_SWITCH.
static acp3x_es83xx_controls: [snd_kcontrol_new; 4] = [
    SOC_DAPM_PIN_SWITCH(c"Speaker".as_ptr()),
    SOC_DAPM_PIN_SWITCH(c"Headphone".as_ptr()),
    SOC_DAPM_PIN_SWITCH(c"Headset Mic".as_ptr()),
    SOC_DAPM_PIN_SWITCH(c"Internal Mic".as_ptr()),
];

unsafe fn get_mach_priv(card: *mut snd_soc_card) -> *mut acp3x_es83xx_private {
    (*(acp_get_drvdata(card))).mach_priv as *mut acp3x_es83xx_private
}

unsafe extern "C" fn acp3x_es83xx_codec_startup(
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime: *mut snd_pcm_runtime;
    let rtd: *mut snd_soc_pcm_runtime;
    let codec_dai: *mut snd_soc_dai;
    let priv_: *mut acp3x_es83xx_private;
    let freq: c_uint;
    let ret: c_int;

    runtime = (*substream).runtime;
    rtd = snd_soc_substream_to_rtd(substream);
    codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    priv_ = get_mach_priv((*rtd).card);

    if (*priv_).quirk & ES83XX_48_MHZ_MCLK != 0 {
        dev_dbg((*priv_).codec_dev, c"using a 48Mhz MCLK\n".as_ptr());
        freq = ES83XX_48_MHZ_MCLK_FREQ;
    } else {
        dev_dbg((*priv_).codec_dev, c"using a 12.288Mhz MCLK\n".as_ptr());
        freq = ES83XX_12288_KHZ_MCLK_FREQ;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, 0, freq, SND_SOC_CLOCK_OUT);
    if ret < 0 {
        dev_err((*rtd).dev, c"can't set codec sysclk: %d\n".as_ptr(), ret);
        return ret;
    }

    (*runtime).hw.channels_max = DUAL_CHANNEL;
    snd_pcm_hw_constraint_list(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        &constraints_channels,
    );

    0
}

unsafe extern "C" fn acp3x_es83xx_configure_widgets(card: *mut snd_soc_card) -> c_int {
    (*card).dapm_widgets = acp3x_es83xx_widgets.as_ptr();
    (*card).num_dapm_widgets = acp3x_es83xx_widgets.len() as c_int;
    (*card).controls = acp3x_es83xx_controls.as_ptr();
    (*card).num_controls = acp3x_es83xx_controls.len() as c_int;
    (*card).dapm_routes = acp3x_es83xx_audio_map.as_ptr();
    (*card).num_dapm_routes = acp3x_es83xx_audio_map.len() as c_int;

    0
}

unsafe extern "C" fn acp3x_es83xx_headphone_power_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card: *mut snd_soc_card = snd_soc_dapm_to_card((*w).dapm);
    let priv_: *mut acp3x_es83xx_private = get_mach_priv(card);

    dev_dbg((*priv_).codec_dev, c"headphone power event = %d\n".as_ptr(), event);
    if SND_SOC_DAPM_EVENT_ON(event) {
        (*priv_).headphone_on = true;
    } else {
        (*priv_).headphone_on = false;
    }

    gpiod_set_value_cansleep((*priv_).gpio_speakers, (*priv_).speaker_on as c_int);
    gpiod_set_value_cansleep((*priv_).gpio_headphone, (*priv_).headphone_on as c_int);

    0
}

unsafe extern "C" fn acp3x_es83xx_speaker_power_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card: *mut snd_soc_card = snd_soc_dapm_to_card((*w).dapm);
    let priv_: *mut acp3x_es83xx_private = get_mach_priv(card);

    dev_dbg((*priv_).codec_dev, c"speaker power event: %d\n".as_ptr(), event);
    if SND_SOC_DAPM_EVENT_ON(event) {
        (*priv_).speaker_on = true;
    } else {
        (*priv_).speaker_on = false;
    }

    gpiod_set_value_cansleep((*priv_).gpio_speakers, (*priv_).speaker_on as c_int);
    gpiod_set_value_cansleep((*priv_).gpio_headphone, (*priv_).headphone_on as c_int);

    0
}

unsafe extern "C" fn acp3x_es83xx_suspend_pre(card: *mut snd_soc_card) -> c_int {
    let priv_: *mut acp3x_es83xx_private = get_mach_priv(card);

    /* We need to disable the jack in the machine driver suspend
     * callback so that the CODEC suspend callback actually gets
     * called. Without doing it, the CODEC suspend/resume
     * callbacks do not get called if headphones are plugged in.
     * This is because plugging in headphones keeps some supplies
     * active, this in turn means that the lowest bias level
     * that the CODEC can go to is SND_SOC_BIAS_STANDBY.
     * If components do not set idle_bias_on to true then
     * their suspend/resume callbacks do not get called.
     */
    dev_dbg((*priv_).codec_dev, c"card suspend\n".as_ptr());
    snd_soc_component_set_jack((*priv_).codec, ptr::null_mut(), ptr::null_mut());
    0
}

unsafe extern "C" fn acp3x_es83xx_resume_post(card: *mut snd_soc_card) -> c_int {
    let priv_: *mut acp3x_es83xx_private = get_mach_priv(card);

    /* We disabled jack detection in suspend callback,
     * enable it back.
     */
    dev_dbg((*priv_).codec_dev, c"card resume\n".as_ptr());
    snd_soc_component_set_jack((*priv_).codec, &raw mut es83xx_jack, ptr::null_mut());
    0
}

unsafe fn acp3x_es83xx_configure_gpios(priv_: *mut acp3x_es83xx_private) -> c_int {
    (*priv_).enable_spk_gpio.crs_entry_index = 0;
    (*priv_).enable_hp_gpio.crs_entry_index = 1;

    (*priv_).enable_spk_gpio.active_low = false;
    (*priv_).enable_hp_gpio.active_low = false;

    (*priv_).gpio_mapping[0].name = c"speakers-enable-gpios".as_ptr();
    (*priv_).gpio_mapping[0].data = &raw mut (*priv_).enable_spk_gpio;
    (*priv_).gpio_mapping[0].size = 1;
    (*priv_).gpio_mapping[0].quirks = ACPI_GPIO_QUIRK_ONLY_GPIOIO;

    (*priv_).gpio_mapping[1].name = c"headphone-enable-gpios".as_ptr();
    (*priv_).gpio_mapping[1].data = &raw mut (*priv_).enable_hp_gpio;
    (*priv_).gpio_mapping[1].size = 1;
    (*priv_).gpio_mapping[1].quirks = ACPI_GPIO_QUIRK_ONLY_GPIOIO;

    dev_info(
        (*priv_).codec_dev,
        c"speaker gpio %d active %s, headphone gpio %d active %s\n".as_ptr(),
        (*priv_).enable_spk_gpio.crs_entry_index,
        str_low_high((*priv_).enable_spk_gpio.active_low),
        (*priv_).enable_hp_gpio.crs_entry_index,
        str_low_high((*priv_).enable_hp_gpio.active_low),
    );
    0
}

unsafe fn acp3x_es83xx_configure_mics(priv_: *mut acp3x_es83xx_private) -> c_int {
    let mut num_routes: c_int = 0;
    let mut i: c_int;

    if (*priv_).quirk & ES83XX_ENABLE_DMIC == 0 {
        (*priv_).mic_map[num_routes as usize].sink = c"MIC1".as_ptr();
        (*priv_).mic_map[num_routes as usize].source = c"Internal Mic".as_ptr();
        num_routes += 1;
    }

    (*priv_).mic_map[num_routes as usize].sink = c"MIC2".as_ptr();
    (*priv_).mic_map[num_routes as usize].source = c"Headset Mic".as_ptr();
    num_routes += 1;

    i = 0;
    while i < num_routes {
        dev_info(
            (*priv_).codec_dev,
            c"%s is %s\n".as_ptr(),
            (*priv_).mic_map[i as usize].source,
            (*priv_).mic_map[i as usize].sink,
        );
        i += 1;
    }

    num_routes
}

unsafe extern "C" fn acp3x_es83xx_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let codec: *mut snd_soc_component = (*snd_soc_rtd_to_codec(runtime, 0)).component;
    let card: *mut snd_soc_card = (*runtime).card;
    let priv_: *mut acp3x_es83xx_private = get_mach_priv(card);
    let mut ret: c_int = 0;
    let num_routes: c_int;

    ret = snd_soc_card_jack_new_pins(
        card,
        c"Headset".as_ptr(),
        SND_JACK_HEADSET | SND_JACK_BTN_0,
        &raw mut es83xx_jack,
        es83xx_jack_pins.as_mut_ptr(),
        es83xx_jack_pins.len() as c_uint,
    );
    if ret != 0 {
        dev_err((*card).dev, c"jack creation failed %d\n".as_ptr(), ret);
        return ret;
    }

    snd_jack_set_key((*es83xx_jack.jack), SND_JACK_BTN_0, KEY_PLAYPAUSE);

    snd_soc_component_set_jack(codec, &raw mut es83xx_jack, ptr::null_mut());

    (*priv_).codec = codec;
    acp3x_es83xx_configure_gpios(priv_);

    ret = devm_acpi_dev_add_driver_gpios((*priv_).codec_dev, (*priv_).gpio_mapping.as_ptr());
    if ret != 0 {
        dev_warn((*priv_).codec_dev, c"failed to add speaker gpio\n".as_ptr());
    }

    (*priv_).gpio_speakers = gpiod_get_optional(
        (*priv_).codec_dev,
        c"speakers-enable".as_ptr(),
        if (*priv_).enable_spk_gpio.active_low {
            GPIOD_OUT_LOW
        } else {
            GPIOD_OUT_HIGH
        },
    );
    if IS_ERR((*priv_).gpio_speakers as *const c_void) {
        dev_err(
            (*priv_).codec_dev,
            c"could not get speakers-enable GPIO\n".as_ptr(),
        );
        return PTR_ERR((*priv_).gpio_speakers as *const c_void) as c_int;
    }

    (*priv_).gpio_headphone = gpiod_get_optional(
        (*priv_).codec_dev,
        c"headphone-enable".as_ptr(),
        if (*priv_).enable_hp_gpio.active_low {
            GPIOD_OUT_LOW
        } else {
            GPIOD_OUT_HIGH
        },
    );
    if IS_ERR((*priv_).gpio_headphone as *const c_void) {
        dev_err(
            (*priv_).codec_dev,
            c"could not get headphone-enable GPIO\n".as_ptr(),
        );
        return PTR_ERR((*priv_).gpio_headphone as *const c_void) as c_int;
    }

    num_routes = acp3x_es83xx_configure_mics(priv_);
    if num_routes > 0 {
        let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);

        ret = snd_soc_dapm_add_routes(dapm, (*priv_).mic_map.as_ptr(), num_routes);
        if ret != 0 {
            device_remove_software_node((*priv_).codec_dev);
        }
    }

    ret
}

static acp3x_es83xx_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(acp3x_es83xx_codec_startup),
};

// C macro expansion of:
// SND_SOC_DAILINK_DEF(codec,
//     DAILINK_COMP_ARRAY(COMP_CODEC("i2c-ESSX8336:00", "ES8316 HiFi")));
static mut codec: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"i2c-ESSX8336:00".as_ptr(),
    dai_name: c"ES8316 HiFi".as_ptr(),
}];

static acp3x_es83xx_dmi_table: [dmi_system_id; 8] = [
    dmi_system_id {
        matches: [
            DMI_EXACT_MATCH(DMI_BOARD_VENDOR, c"HUAWEI".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_NAME, c"KLVL-WXXW".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_VERSION, c"M1010".as_ptr()),
            dmi_strmatch::default(),
        ],
        driver_data: ES83XX_ENABLE_DMIC as *mut c_void,
    },
    dmi_system_id {
        matches: [
            DMI_EXACT_MATCH(DMI_BOARD_VENDOR, c"HUAWEI".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_NAME, c"KLVL-WXX9".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_VERSION, c"M1010".as_ptr()),
            dmi_strmatch::default(),
        ],
        driver_data: ES83XX_ENABLE_DMIC as *mut c_void,
    },
    dmi_system_id {
        matches: [
            DMI_EXACT_MATCH(DMI_BOARD_VENDOR, c"HUAWEI".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_NAME, c"BOM-WXX9".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_VERSION, c"M1010".as_ptr()),
            dmi_strmatch::default(),
        ],
        driver_data: (ES83XX_ENABLE_DMIC | ES83XX_48_MHZ_MCLK) as *mut c_void,
    },
    dmi_system_id {
        matches: [
            DMI_EXACT_MATCH(DMI_BOARD_VENDOR, c"HUAWEI".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_NAME, c"HVY-WXX9".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_VERSION, c"M1010".as_ptr()),
            dmi_strmatch::default(),
        ],
        driver_data: ES83XX_ENABLE_DMIC as *mut c_void,
    },
    dmi_system_id {
        matches: [
            DMI_EXACT_MATCH(DMI_BOARD_VENDOR, c"HUAWEI".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_NAME, c"HVY-WXX9".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_VERSION, c"M1020".as_ptr()),
            dmi_strmatch::default(),
        ],
        driver_data: ES83XX_ENABLE_DMIC as *mut c_void,
    },
    dmi_system_id {
        matches: [
            DMI_EXACT_MATCH(DMI_BOARD_VENDOR, c"HUAWEI".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_NAME, c"HVY-WXX9".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_VERSION, c"M1040".as_ptr()),
            dmi_strmatch::default(),
        ],
        driver_data: ES83XX_ENABLE_DMIC as *mut c_void,
    },
    dmi_system_id {
        matches: [
            DMI_EXACT_MATCH(DMI_BOARD_VENDOR, c"HUAWEI".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_NAME, c"HVY-WXX9".as_ptr()),
            DMI_EXACT_MATCH(DMI_PRODUCT_VERSION, c"M1060".as_ptr()),
            dmi_strmatch::default(),
        ],
        driver_data: ES83XX_ENABLE_DMIC as *mut c_void,
    },
    dmi_system_id::default(),
];

unsafe extern "C" fn acp3x_es83xx_configure_link(
    _card: *mut snd_soc_card,
    link: *mut snd_soc_dai_link,
) -> c_int {
    (*link).codecs = codec.as_mut_ptr();
    (*link).num_codecs = codec.len() as c_int;
    (*link).init = Some(acp3x_es83xx_init);
    (*link).ops = &acp3x_es83xx_ops;
    (*link).dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP;

    0
}

unsafe extern "C" fn acp3x_es83xx_probe(card: *mut snd_soc_card) -> c_int {
    let mut ret: c_int = 0;
    let dev: *mut device = (*card).dev;
    let dmi_id: *const dmi_system_id;

    dmi_id = dmi_first_match(acp3x_es83xx_dmi_table.as_ptr());
    if !dmi_id.is_null() && !(*dmi_id).driver_data.is_null() {
        let priv_: *mut acp3x_es83xx_private;
        let acp_drvdata: *mut acp_card_drvdata;
        let adev: *mut acpi_device;
        let codec_dev: *mut device;

        acp_drvdata = (*card).drvdata as *mut acp_card_drvdata;

        dev_info(
            dev,
            c"matched DMI table with this system, trying to register sound card\n".as_ptr(),
        );

        adev = acpi_dev_get_first_match_dev((*(*acp_drvdata).acpi_mach).id, ptr::null(), -1);
        if adev.is_null() {
            dev_err(
                dev,
                c"Error cannot find '%s' dev\n".as_ptr(),
                (*(*acp_drvdata).acpi_mach).id,
            );
            return -ENXIO;
        }

        codec_dev = acpi_get_first_physical_node(adev);
        acpi_dev_put(adev);
        if codec_dev.is_null() {
            dev_warn(
                dev,
                c"Error cannot find codec device, will defer probe\n".as_ptr(),
            );
            return -EPROBE_DEFER;
        }

        priv_ = devm_kzalloc(dev, size_of::<acp3x_es83xx_private>(), GFP_KERNEL)
            as *mut acp3x_es83xx_private;
        if priv_.is_null() {
            put_device(codec_dev);
            return -ENOMEM;
        }

        (*priv_).codec_dev = codec_dev;
        (*priv_).quirk = (*dmi_id).driver_data as c_ulong;
        (*acp_drvdata).mach_priv = priv_ as *mut c_void;
        dev_info(dev, c"successfully probed the sound card\n".as_ptr());
    } else {
        ret = -ENODEV;
        dev_warn(
            dev,
            c"this system has a ES83xx codec defined in ACPI, but the driver doesn't have this system registered in DMI table\n".as_ptr(),
        );
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn acp3x_es83xx_init_ops(ops: *mut acp_mach_ops) {
    (*ops).probe = Some(acp3x_es83xx_probe);
    (*ops).configure_widgets = Some(acp3x_es83xx_configure_widgets);
    (*ops).configure_link = Some(acp3x_es83xx_configure_link);
    (*ops).suspend_pre = Some(acp3x_es83xx_suspend_pre);
    (*ops).resume_post = Some(acp3x_es83xx_resume_post);
}

extern "C" {
    fn acp_get_drvdata(card: *mut snd_soc_card) -> *mut acp_card_drvdata;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        l: *const snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn str_low_high(v: bool) -> *const c_char;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int);
    fn devm_acpi_dev_add_driver_gpios(
        dev: *mut device,
        gpios: *const acpi_gpio_mapping,
    ) -> c_int;
    fn gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_int,
    ) -> *mut gpio_desc;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn device_remove_software_node(dev: *mut device);
    fn dmi_first_match(ids: *const dmi_system_id) -> *const dmi_system_id;
    fn acpi_dev_get_first_match_dev(
        hid: *const c_char,
        uid: *const c_char,
        hrv: c_long,
    ) -> *mut acpi_device;
    fn acpi_dev_put(adev: *mut acpi_device);
    fn acpi_get_first_physical_node(adev: *mut acpi_device) -> *mut device;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn put_device(dev: *mut device);

    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
}

extern "Rust" {
    fn SND_SOC_DAPM_SPK(name: *const c_char, event: *const c_void) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_HP(name: *const c_char, event: *const c_void) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_MIC(name: *const c_char, event: *const c_void) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_SUPPLY(
        name: *const c_char,
        reg: c_int,
        shift: c_int,
        invert: c_int,
        event: Option<
            unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int,
        >,
        event_flags: c_int,
    ) -> snd_soc_dapm_widget;
    fn SOC_DAPM_PIN_SWITCH(name: *const c_char) -> snd_kcontrol_new;
    fn DMI_EXACT_MATCH(slot: c_int, substr: *const c_char) -> dmi_strmatch;
    fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> bool;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
}

extern "C" {
    static SND_SOC_CLOCK_OUT: c_int;
    static SNDRV_PCM_HW_PARAM_CHANNELS: c_int;
    static SND_JACK_HEADPHONE: c_int;
    static SND_JACK_MICROPHONE: c_int;
    static SND_JACK_HEADSET: c_int;
    static SND_JACK_BTN_0: c_int;
    static KEY_PLAYPAUSE: c_int;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static ACPI_GPIO_QUIRK_ONLY_GPIOIO: c_uint;
    static GPIOD_OUT_LOW: c_int;
    static GPIOD_OUT_HIGH: c_int;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static DMI_BOARD_VENDOR: c_int;
    static DMI_PRODUCT_NAME: c_int;
    static DMI_PRODUCT_VERSION: c_int;
    static GFP_KERNEL: c_uint;
    static ENXIO: c_int;
    static EPROBE_DEFER: c_int;
    static ENOMEM: c_int;
    static ENODEV: c_int;
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpio_params {
    pub crs_entry_index: c_uint,
    pub active_low: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpio_mapping {
    pub name: *const c_char,
    pub data: *mut acpi_gpio_params,
    pub size: c_uint,
    pub quirks: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
    pub mask: c_ulong,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub drvdata: *mut c_void,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_int,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub ops: *const snd_soc_ops,
    pub dai_fmt: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmi_strmatch {
    _private: [u8; 0],
}

impl dmi_strmatch {
    pub const fn default() -> Self {
        Self { _private: [] }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmi_system_id {
    pub matches: [dmi_strmatch; 4],
    pub driver_data: *mut c_void,
}

impl dmi_system_id {
    pub const fn default() -> Self {
        Self {
            matches: [dmi_strmatch::default(); 4],
            driver_data: ptr::null_mut(),
        }
    }
}

#[repr(C)]
pub struct acp_card_drvdata {
    pub mach_priv: *mut c_void,
    pub acpi_mach: *mut snd_soc_acpi_mach,
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
pub struct acp_mach_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub configure_widgets: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub configure_link:
        Option<unsafe extern "C" fn(*mut snd_soc_card, *mut snd_soc_dai_link) -> c_int>,
    pub suspend_pre: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub resume_post: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
