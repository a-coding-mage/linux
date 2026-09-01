// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for C-Media's CMI8330 and CMI8329 soundcards.
 *  Copyright (c) by George Talusan <gstalusan@uwaterloo.ca>
 *    http://www.undergrad.math.uwaterloo.ca/~gstalusa
 */

/*
 * NOTES
 *
 *  The extended registers contain mixer settings which are largely
 *  untapped for the time being.
 *
 *  MPU401 and SPDIF are not supported yet.  I don't have the hardware
 *  to aid in coding and testing, so I won't bother.
 *
 *  To quickly load the module,
 *
 *  modprobe -a snd-cmi8330 sbport=0x220 sbirq=5 sbdma8=1
 *    sbdma16=5 wssport=0x530 wssirq=11 wssdma=0 fmport=0x388
 *
 *  This card has two mixers and two PCM devices.  I've cheesed it such
 *  that recording and playback can be done through the same device.
 *  The driver "magically" routes the capturing to the AD1848 codec,
 *  and playback to the SB16 codec.  This allows for full-duplex mode
 *  to some extent.
 *  The utilities in alsa-utils are aware of both devices, so passing
 *  the appropriate parameters to amixer and alsactl will give you
 *  full control over both mixers.
 */

/* C includes removed: linux/init.h, linux/err.h, linux/isa.h, linux/pnp.h,
 * linux/module.h, sound/core.h, sound/wss.h, sound/opl3.h, sound/mpu401.h,
 * sound/sb.h, sound/initval.h.
 */

/*
 */
/* ENABLE_SB_MIXER intentionally disabled in the original source. */
const PLAYBACK_ON_SB: bool = true;

/*
 */
// MODULE_AUTHOR("George Talusan <gstalusan@uwaterloo.ca>");
// MODULE_DESCRIPTION("C-Media CMI8330/CMI8329");
// MODULE_LICENSE("GPL");

type CInt = i32;
type CUInt = u32;
type CLong = isize;
type CUChar = u8;
type Bool = bool;

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [CInt; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut i8; SNDRV_CARDS] = [core::ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_ISAPNP: [Bool; SNDRV_CARDS] = [false; SNDRV_CARDS];
const SNDRV_DEFAULT_PORT: [CLong; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_IRQ: [CInt; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_DMA: [CInt; SNDRV_CARDS] = [0; SNDRV_CARDS];

static mut index: [CInt; SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
static mut id: [*mut i8; SNDRV_CARDS] = SNDRV_DEFAULT_STR;
static mut enable: [Bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_ISAPNP;
#[cfg(CONFIG_PNP)]
static mut isapnp: [Bool; SNDRV_CARDS] = [true; SNDRV_CARDS];
static mut sbport: [CLong; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;
static mut sbirq: [CInt; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ;
static mut sbdma8: [CInt; SNDRV_CARDS] = SNDRV_DEFAULT_DMA;
static mut sbdma16: [CInt; SNDRV_CARDS] = SNDRV_DEFAULT_DMA;
static mut wssport: [CLong; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;
static mut wssirq: [CInt; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ;
static mut wssdma: [CInt; SNDRV_CARDS] = SNDRV_DEFAULT_DMA;
static mut fmport: [CLong; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;
static mut mpuport: [CLong; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;
static mut mpuirq: [CInt; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ;

// module_param_array/module_param_hw_array and MODULE_PARM_DESC declarations
// are kernel module metadata in C.

#[cfg(CONFIG_PNP)]
static mut isa_registered: CInt = 0;
#[cfg(CONFIG_PNP)]
static mut pnp_registered: CInt = 0;

const CMI8330_RMUX3D: CInt = 16;
const CMI8330_MUTEMUX: CInt = 17;
const CMI8330_OUTPUTVOL: CInt = 18;
const CMI8330_MASTVOL: CInt = 19;
const CMI8330_LINVOL: CInt = 20;
const CMI8330_CDINVOL: CInt = 21;
const CMI8330_WAVVOL: CInt = 22;
const CMI8330_RECMUX: CInt = 23;
const CMI8330_WAVGAIN: CInt = 24;
const CMI8330_LINGAIN: CInt = 25;
const CMI8330_CDINGAIN: CInt = 26;

static snd_cmi8330_image: [CUChar; ((CMI8330_CDINGAIN - 16) + 1) as usize] = [
    0x40, /* 16 - recording mux (SB-mixer-enabled) */
    0x0,  /* 17 - mute mux */
    0x0,  /* 18 - vol */
    0x0,  /* 19 - master volume */
    0x0,  /* 20 - line-in volume */
    0x0,  /* 21 - cd-in volume */
    0x0,  /* 22 - wave volume */
    0x0,  /* 23 - mute/rec mux */
    0x0,  /* 24 - wave rec gain */
    0x0,  /* 25 - line-in rec gain */
    0x0,  /* 26 - cd-in rec gain */
];

type snd_pcm_open_callback_t = unsafe extern "C" fn(*mut snd_pcm_substream) -> CInt;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum card_type {
    CMI8330,
    CMI8329,
}

#[repr(C)]
struct snd_cmi8330_stream {
    ops: snd_pcm_ops,
    open: Option<snd_pcm_open_callback_t>,
    private_data: *mut core::ffi::c_void, /* sb or wss */
}

#[repr(C)]
struct snd_cmi8330 {
    #[cfg(CONFIG_PNP)]
    cap: *mut pnp_dev,
    #[cfg(CONFIG_PNP)]
    play: *mut pnp_dev,
    #[cfg(CONFIG_PNP)]
    mpu: *mut pnp_dev,
    card: *mut snd_card,
    wss: *mut snd_wss,
    sb: *mut snd_sb,
    pcm: *mut snd_pcm,
    streams: [snd_cmi8330_stream; 2],
    type_: card_type,
}

#[cfg(CONFIG_PNP)]
static snd_cmi8330_pnpids: [pnp_card_device_id; 3] = [
    pnp_card_device_id { id: *b"CMI0001\0", devs: [pnp_id::new(*b"@X@0001\0"), pnp_id::new(*b"@@@0001\0"), pnp_id::new(*b"@H@0001\0"), pnp_id::new(*b"A@@0001\0")] },
    pnp_card_device_id { id: *b"CMI0001\0", devs: [pnp_id::new(*b"@@@0001\0"), pnp_id::new(*b"@X@0001\0"), pnp_id::new(*b"@H@0001\0"), pnp_id::new(*b"\0\0\0\0\0\0\0\0")] },
    pnp_card_device_id { id: *b"\0\0\0\0\0\0\0\0", devs: [pnp_id::new(*b"\0\0\0\0\0\0\0\0"), pnp_id::new(*b"\0\0\0\0\0\0\0\0"), pnp_id::new(*b"\0\0\0\0\0\0\0\0"), pnp_id::new(*b"\0\0\0\0\0\0\0\0")] },
];

// MODULE_DEVICE_TABLE(pnp_card, snd_cmi8330_pnpids);

static snd_cmi8330_controls: [snd_kcontrol_new; 26] = [
    WSS_DOUBLE!("Master Playback Volume", 0, CMI8330_MASTVOL, CMI8330_MASTVOL, 4, 0, 15, 0),
    WSS_SINGLE!("Loud Playback Switch", 0, CMI8330_MUTEMUX, 6, 1, 1),
    WSS_DOUBLE!("PCM Playback Switch", 0, CS4231_LEFT_OUTPUT, CS4231_RIGHT_OUTPUT, 7, 7, 1, 1),
    WSS_DOUBLE!("PCM Playback Volume", 0, CS4231_LEFT_OUTPUT, CS4231_RIGHT_OUTPUT, 0, 0, 63, 1),
    WSS_DOUBLE!("Line Playback Switch", 0, CMI8330_MUTEMUX, CMI8330_MUTEMUX, 4, 3, 1, 0),
    WSS_DOUBLE!("Line Playback Volume", 0, CMI8330_LINVOL, CMI8330_LINVOL, 4, 0, 15, 0),
    WSS_DOUBLE!("Line Capture Switch", 0, CMI8330_RMUX3D, CMI8330_RMUX3D, 2, 1, 1, 0),
    WSS_DOUBLE!("Line Capture Volume", 0, CMI8330_LINGAIN, CMI8330_LINGAIN, 4, 0, 15, 0),
    WSS_DOUBLE!("CD Playback Switch", 0, CMI8330_MUTEMUX, CMI8330_MUTEMUX, 2, 1, 1, 0),
    WSS_DOUBLE!("CD Capture Switch", 0, CMI8330_RMUX3D, CMI8330_RMUX3D, 4, 3, 1, 0),
    WSS_DOUBLE!("CD Playback Volume", 0, CMI8330_CDINVOL, CMI8330_CDINVOL, 4, 0, 15, 0),
    WSS_DOUBLE!("CD Capture Volume", 0, CMI8330_CDINGAIN, CMI8330_CDINGAIN, 4, 0, 15, 0),
    WSS_SINGLE!("Mic Playback Switch", 0, CMI8330_MUTEMUX, 0, 1, 0),
    WSS_SINGLE!("Mic Playback Volume", 0, CMI8330_OUTPUTVOL, 0, 7, 0),
    WSS_SINGLE!("Mic Capture Switch", 0, CMI8330_RMUX3D, 0, 1, 0),
    WSS_SINGLE!("Mic Capture Volume", 0, CMI8330_OUTPUTVOL, 5, 7, 0),
    WSS_DOUBLE!("Wavetable Playback Switch", 0, CMI8330_RECMUX, CMI8330_RECMUX, 1, 0, 1, 0),
    WSS_DOUBLE!("Wavetable Playback Volume", 0, CMI8330_WAVVOL, CMI8330_WAVVOL, 4, 0, 15, 0),
    WSS_DOUBLE!("Wavetable Capture Switch", 0, CMI8330_RECMUX, CMI8330_RECMUX, 5, 4, 1, 0),
    WSS_DOUBLE!("Wavetable Capture Volume", 0, CMI8330_WAVGAIN, CMI8330_WAVGAIN, 4, 0, 15, 0),
    WSS_SINGLE!("3D Control - Switch", 0, CMI8330_RMUX3D, 5, 1, 1),
    WSS_SINGLE!("Beep Playback Volume", 0, CMI8330_OUTPUTVOL, 3, 3, 0),
    WSS_DOUBLE!("FM Playback Switch", 0, CS4231_AUX2_LEFT_INPUT, CS4231_AUX2_RIGHT_INPUT, 7, 7, 1, 1),
    WSS_DOUBLE!("FM Playback Volume", 0, CS4231_AUX2_LEFT_INPUT, CS4231_AUX2_RIGHT_INPUT, 0, 0, 31, 1),
    WSS_SINGLE!(SNDRV_CTL_NAME_IEC958!("Input ", CAPTURE, SWITCH), 0, CMI8330_RMUX3D, 7, 1, 1),
    WSS_SINGLE!(SNDRV_CTL_NAME_IEC958!("Input ", PLAYBACK, SWITCH), 0, CMI8330_MUTEMUX, 7, 1, 1),
];

/* ENABLE_SB_MIXER block preserved but inactive, matching the original source. */
#[cfg(ENABLE_SB_MIXER)]
static cmi8330_sb_mixers: [sbmix_elem; 15] = [
    SB_DOUBLE!("SB Master Playback Volume", SB_DSP4_MASTER_DEV, SB_DSP4_MASTER_DEV + 1, 3, 3, 31),
    SB_DOUBLE!("Tone Control - Bass", SB_DSP4_BASS_DEV, SB_DSP4_BASS_DEV + 1, 4, 4, 15),
    SB_DOUBLE!("Tone Control - Treble", SB_DSP4_TREBLE_DEV, SB_DSP4_TREBLE_DEV + 1, 4, 4, 15),
    SB_DOUBLE!("SB PCM Playback Volume", SB_DSP4_PCM_DEV, SB_DSP4_PCM_DEV + 1, 3, 3, 31),
    SB_DOUBLE!("SB Synth Playback Volume", SB_DSP4_SYNTH_DEV, SB_DSP4_SYNTH_DEV + 1, 3, 3, 31),
    SB_DOUBLE!("SB CD Playback Switch", SB_DSP4_OUTPUT_SW, SB_DSP4_OUTPUT_SW, 2, 1, 1),
    SB_DOUBLE!("SB CD Playback Volume", SB_DSP4_CD_DEV, SB_DSP4_CD_DEV + 1, 3, 3, 31),
    SB_DOUBLE!("SB Line Playback Switch", SB_DSP4_OUTPUT_SW, SB_DSP4_OUTPUT_SW, 4, 3, 1),
    SB_DOUBLE!("SB Line Playback Volume", SB_DSP4_LINE_DEV, SB_DSP4_LINE_DEV + 1, 3, 3, 31),
    SB_SINGLE!("SB Mic Playback Switch", SB_DSP4_OUTPUT_SW, 0, 1),
    SB_SINGLE!("SB Mic Playback Volume", SB_DSP4_MIC_DEV, 3, 31),
    SB_SINGLE!("SB Beep Volume", SB_DSP4_SPEAKER_DEV, 6, 3),
    SB_DOUBLE!("SB Capture Volume", SB_DSP4_IGAIN_DEV, SB_DSP4_IGAIN_DEV + 1, 6, 6, 3),
    SB_DOUBLE!("SB Playback Volume", SB_DSP4_OGAIN_DEV, SB_DSP4_OGAIN_DEV + 1, 6, 6, 3),
    SB_SINGLE!("SB Mic Auto Gain", SB_DSP4_MIC_AGC, 0, 1),
];

#[cfg(ENABLE_SB_MIXER)]
static cmi8330_sb_init_values: [[CUChar; 2]; 10] = [
    [SB_DSP4_MASTER_DEV + 0, 0],
    [SB_DSP4_MASTER_DEV + 1, 0],
    [SB_DSP4_PCM_DEV + 0, 0],
    [SB_DSP4_PCM_DEV + 1, 0],
    [SB_DSP4_SYNTH_DEV + 0, 0],
    [SB_DSP4_SYNTH_DEV + 1, 0],
    [SB_DSP4_INPUT_LEFT, 0],
    [SB_DSP4_INPUT_RIGHT, 0],
    [SB_DSP4_OUTPUT_SW, 0],
    [SB_DSP4_SPEAKER_DEV, 0],
];

#[cfg(ENABLE_SB_MIXER)]
unsafe extern "C" fn cmi8330_add_sb_mixers(chip: *mut snd_sb) -> CInt {
    let mut idx: CInt;
    let mut err: CInt;

    snd_sbmixer_write(chip, 0x00, 0x00); /* mixer reset */

    /* mute and zero volume channels */
    idx = 0;
    while (idx as usize) < cmi8330_sb_init_values.len() {
        snd_sbmixer_write(
            chip,
            cmi8330_sb_init_values[idx as usize][0],
            cmi8330_sb_init_values[idx as usize][1],
        );
        idx += 1;
    }

    idx = 0;
    while (idx as usize) < cmi8330_sb_mixers.len() {
        err = snd_sbmixer_add_ctl_elem(chip, &cmi8330_sb_mixers[idx as usize]);
        if err < 0 {
            return err;
        }
        idx += 1;
    }
    0
}

unsafe extern "C" fn snd_cmi8330_mixer(card: *mut snd_card, acard: *mut snd_cmi8330) -> CInt {
    let mut idx: CUInt;
    let mut err: CInt;

    strscpy((*card).mixername.as_mut_ptr(), if (*acard).type_ == card_type::CMI8329 { c"CMI8329".as_ptr() } else { c"CMI8330/C3D".as_ptr() });

    idx = 0;
    while (idx as usize) < snd_cmi8330_controls.len() {
        err = snd_ctl_add(card, snd_ctl_new1(&snd_cmi8330_controls[idx as usize], (*acard).wss as *mut core::ffi::c_void));
        if err < 0 {
            return err;
        }
        idx += 1;
    }

    #[cfg(ENABLE_SB_MIXER)]
    {
        err = cmi8330_add_sb_mixers((*acard).sb);
        if err < 0 {
            return err;
        }
    }
    0
}

#[cfg(CONFIG_PNP)]
unsafe extern "C" fn snd_cmi8330_pnp(
    dev: CInt,
    acard: *mut snd_cmi8330,
    card: *mut pnp_card_link,
    id: *const pnp_card_device_id,
) -> CInt {
    let mut pdev: *mut pnp_dev;
    let mut err: CInt;

    /* CMI8329 has a device with ID A@@0001, CMI8330 does not */
    (*acard).type_ = if (*id).devs[3].id[0] != 0 { card_type::CMI8329 } else { card_type::CMI8330 };

    (*acard).cap = pnp_request_card_device(card, (*id).devs[0].id.as_ptr(), core::ptr::null_mut());
    if (*acard).cap.is_null() {
        return -EBUSY;
    }

    (*acard).play = pnp_request_card_device(card, (*id).devs[1].id.as_ptr(), core::ptr::null_mut());
    if (*acard).play.is_null() {
        return -EBUSY;
    }

    (*acard).mpu = pnp_request_card_device(card, (*id).devs[2].id.as_ptr(), core::ptr::null_mut());
    if (*acard).mpu.is_null() {
        return -EBUSY;
    }

    pdev = (*acard).cap;

    err = pnp_activate_dev(pdev);
    if err < 0 {
        dev_err(&mut (*pdev).dev, c"AD1848 PnP configure failure\n".as_ptr());
        return -EBUSY;
    }
    wssport[dev as usize] = pnp_port_start(pdev, 0) as CLong;
    wssdma[dev as usize] = pnp_dma(pdev, 0);
    wssirq[dev as usize] = pnp_irq(pdev, 0);
    if pnp_port_start(pdev, 1) != 0 {
        fmport[dev as usize] = pnp_port_start(pdev, 1) as CLong;
    }

    /* allocate SB16 resources */
    pdev = (*acard).play;

    err = pnp_activate_dev(pdev);
    if err < 0 {
        dev_err(&mut (*pdev).dev, c"SB16 PnP configure failure\n".as_ptr());
        return -EBUSY;
    }
    sbport[dev as usize] = pnp_port_start(pdev, 0) as CLong;
    sbdma8[dev as usize] = pnp_dma(pdev, 0);
    sbdma16[dev as usize] = pnp_dma(pdev, 1);
    sbirq[dev as usize] = pnp_irq(pdev, 0);
    /* On CMI8239, the OPL3 port might be present in SB16 PnP resources */
    if fmport[dev as usize] == SNDRV_AUTO_PORT {
        if pnp_port_start(pdev, 1) != 0 {
            fmport[dev as usize] = pnp_port_start(pdev, 1) as CLong;
        } else {
            fmport[dev as usize] = 0x388; /* Or hardwired */
        }
    }

    /* allocate MPU-401 resources */
    pdev = (*acard).mpu;

    err = pnp_activate_dev(pdev);
    if err < 0 {
        dev_err(&mut (*pdev).dev, c"MPU-401 PnP configure failure: will be disabled\n".as_ptr());
    } else {
        mpuport[dev as usize] = pnp_port_start(pdev, 0) as CLong;
        mpuirq[dev as usize] = pnp_irq(pdev, 0);
    }
    0
}

/*
 * PCM interface
 *
 * since we call the different chip interfaces for playback and capture
 * directions, we need a trick.
 *
 * - copy the ops for each direction into a local record.
 * - replace the open callback with the new one, which replaces the
 *   substream->private_data with the corresponding chip instance
 *   and calls again the original open callback of the chip.
 *
 */

const CMI_SB_STREAM: usize = SNDRV_PCM_STREAM_PLAYBACK as usize;
const CMI_AD_STREAM: usize = SNDRV_PCM_STREAM_CAPTURE as usize;

unsafe extern "C" fn snd_cmi8330_playback_open(substream: *mut snd_pcm_substream) -> CInt {
    let chip: *mut snd_cmi8330 = snd_pcm_substream_chip(substream) as *mut snd_cmi8330;

    /* replace the private_data and call the original open callback */
    (*substream).private_data = (*chip).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].private_data;
    ((*chip).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].open.unwrap())(substream)
}

unsafe extern "C" fn snd_cmi8330_capture_open(substream: *mut snd_pcm_substream) -> CInt {
    let chip: *mut snd_cmi8330 = snd_pcm_substream_chip(substream) as *mut snd_cmi8330;

    /* replace the private_data and call the original open callback */
    (*substream).private_data = (*chip).streams[SNDRV_PCM_STREAM_CAPTURE as usize].private_data;
    ((*chip).streams[SNDRV_PCM_STREAM_CAPTURE as usize].open.unwrap())(substream)
}

unsafe extern "C" fn snd_cmi8330_pcm(card: *mut snd_card, chip: *mut snd_cmi8330) -> CInt {
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let mut ops: *const snd_pcm_ops;
    let mut err: CInt;
    static cmi_open_callbacks: [snd_pcm_open_callback_t; 2] = [
        snd_cmi8330_playback_open,
        snd_cmi8330_capture_open,
    ];

    err = snd_pcm_new(card, if (*chip).type_ == card_type::CMI8329 { c"CMI8329".as_ptr() } else { c"CMI8330".as_ptr() }, 0, 1, 1, &mut pcm);
    if err < 0 {
        return err;
    }
    strscpy((*pcm).name.as_mut_ptr(), if (*chip).type_ == card_type::CMI8329 { c"CMI8329".as_ptr() } else { c"CMI8330".as_ptr() });
    (*pcm).private_data = chip as *mut core::ffi::c_void;

    /* SB16 */
    ops = snd_sb16dsp_get_pcm_ops(CMI_SB_STREAM as CInt);
    (*chip).streams[CMI_SB_STREAM].ops = *ops;
    (*chip).streams[CMI_SB_STREAM].open = (*ops).open;
    (*chip).streams[CMI_SB_STREAM].ops.open = Some(cmi_open_callbacks[CMI_SB_STREAM]);
    (*chip).streams[CMI_SB_STREAM].private_data = (*chip).sb as *mut core::ffi::c_void;

    /* AD1848 */
    ops = snd_wss_get_pcm_ops(CMI_AD_STREAM as CInt);
    (*chip).streams[CMI_AD_STREAM].ops = *ops;
    (*chip).streams[CMI_AD_STREAM].open = (*ops).open;
    (*chip).streams[CMI_AD_STREAM].ops.open = Some(cmi_open_callbacks[CMI_AD_STREAM]);
    (*chip).streams[CMI_AD_STREAM].private_data = (*chip).wss as *mut core::ffi::c_void;

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &mut (*chip).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &mut (*chip).streams[SNDRV_PCM_STREAM_CAPTURE as usize].ops);

    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, (*card).dev, 64 * 1024, 128 * 1024);
    (*chip).pcm = pcm;

    0
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn snd_cmi8330_suspend(card: *mut snd_card) -> CInt {
    let acard: *mut snd_cmi8330 = (*card).private_data as *mut snd_cmi8330;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    ((*(*acard).wss).suspend.unwrap())((*acard).wss);
    snd_sbmixer_suspend((*acard).sb);
    0
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn snd_cmi8330_resume(card: *mut snd_card) -> CInt {
    let acard: *mut snd_cmi8330 = (*card).private_data as *mut snd_cmi8330;

    snd_sbdsp_reset((*acard).sb);
    snd_sbmixer_suspend((*acard).sb);
    ((*(*acard).wss).resume.unwrap())((*acard).wss);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

/*
 */

#[cfg(CONFIG_PNP)]
unsafe fn is_isapnp_selected(dev: usize) -> Bool {
    isapnp[dev]
}

#[cfg(not(CONFIG_PNP))]
unsafe fn is_isapnp_selected(_dev: usize) -> CInt {
    0
}

unsafe extern "C" fn snd_cmi8330_card_new(pdev: *mut device, dev: CInt, cardp: *mut *mut snd_card) -> CInt {
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut acard: *mut snd_cmi8330;
    let mut err: CInt;

    err = snd_devm_card_new(
        pdev,
        index[dev as usize],
        id[dev as usize],
        THIS_MODULE,
        core::mem::size_of::<snd_cmi8330>(),
        &mut card,
    );
    if err < 0 {
        dev_err(pdev, c"could not get a new card\n".as_ptr());
        return err;
    }
    acard = (*card).private_data as *mut snd_cmi8330;
    (*acard).card = card;
    *cardp = card;
    0
}

unsafe extern "C" fn snd_cmi8330_probe(card: *mut snd_card, dev: CInt) -> CInt {
    let mut acard: *mut snd_cmi8330;
    let mut i: CInt;
    let mut err: CInt;
    let mut opl3: *mut snd_opl3 = core::ptr::null_mut();

    acard = (*card).private_data as *mut snd_cmi8330;
    err = snd_wss_create(
        card,
        wssport[dev as usize] + 4,
        -1,
        wssirq[dev as usize],
        wssdma[dev as usize],
        -1,
        WSS_HW_DETECT,
        0,
        &mut (*acard).wss,
    );
    if err < 0 {
        dev_err((*card).dev, c"AD1848 device busy??\n".as_ptr());
        return err;
    }
    if (*(*acard).wss).hardware != WSS_HW_CMI8330 {
        dev_err((*card).dev, c"AD1848 not found during probe\n".as_ptr());
        return -ENODEV;
    }

    err = snd_sbdsp_create(
        card,
        sbport[dev as usize],
        sbirq[dev as usize],
        Some(snd_sb16dsp_interrupt),
        sbdma8[dev as usize],
        sbdma16[dev as usize],
        SB_HW_AUTO,
        &mut (*acard).sb,
    );
    if err < 0 {
        dev_err((*card).dev, c"SB16 device busy??\n".as_ptr());
        return err;
    }
    if (*(*acard).sb).hardware != SB_HW_16 {
        dev_err((*card).dev, c"SB16 not found during probe\n".as_ptr());
        return -ENODEV;
    }

    snd_wss_out((*acard).wss, CS4231_MISC_INFO, 0x40); /* switch on MODE2 */
    i = CMI8330_RMUX3D;
    while i <= CMI8330_CDINGAIN {
        snd_wss_out((*acard).wss, i, snd_cmi8330_image[(i - CMI8330_RMUX3D) as usize]);
        i += 1;
    }

    err = snd_cmi8330_mixer(card, acard);
    if err < 0 {
        dev_err((*card).dev, c"failed to create mixers\n".as_ptr());
        return err;
    }

    err = snd_cmi8330_pcm(card, acard);
    if err < 0 {
        dev_err((*card).dev, c"failed to create pcms\n".as_ptr());
        return err;
    }
    if fmport[dev as usize] != SNDRV_AUTO_PORT {
        if snd_opl3_create(card, fmport[dev as usize], fmport[dev as usize] + 2, OPL3_HW_AUTO, 0, &mut opl3) < 0 {
            dev_err((*card).dev, c"no OPL device at 0x%lx-0x%lx ?\n".as_ptr(), fmport[dev as usize], fmport[dev as usize] + 2);
        } else {
            err = snd_opl3_hwdep_new(opl3, 0, 1, core::ptr::null_mut());
            if err < 0 {
                return err;
            }
        }
    }

    if mpuport[dev as usize] != SNDRV_AUTO_PORT {
        if snd_mpu401_uart_new(card, 0, MPU401_HW_MPU401, mpuport[dev as usize], 0, mpuirq[dev as usize], core::ptr::null_mut()) < 0 {
            dev_err((*card).dev, c"no MPU-401 device at 0x%lx.\n".as_ptr(), mpuport[dev as usize]);
        }
    }

    strscpy((*card).driver.as_mut_ptr(), if (*acard).type_ == card_type::CMI8329 { c"CMI8329".as_ptr() } else { c"CMI8330/C3D".as_ptr() });
    strscpy((*card).shortname.as_mut_ptr(), if (*acard).type_ == card_type::CMI8329 { c"C-Media CMI8329".as_ptr() } else { c"C-Media CMI8330/C3D".as_ptr() });
    sprintf(
        (*card).longname.as_mut_ptr(),
        c"%s at 0x%lx, irq %d, dma %d".as_ptr(),
        (*card).shortname.as_ptr(),
        (*(*acard).wss).port,
        wssirq[dev as usize],
        wssdma[dev as usize],
    );

    snd_card_register(card)
}

unsafe extern "C" fn snd_cmi8330_isa_match(pdev: *mut device, dev: CUInt) -> CInt {
    if !enable[dev as usize] || is_isapnp_selected(dev as usize) != 0 {
        return 0;
    }
    if wssport[dev as usize] == SNDRV_AUTO_PORT {
        dev_err(pdev, c"specify wssport\n".as_ptr());
        return 0;
    }
    if sbport[dev as usize] == SNDRV_AUTO_PORT {
        dev_err(pdev, c"specify sbport\n".as_ptr());
        return 0;
    }
    1
}

unsafe extern "C" fn snd_cmi8330_isa_probe(pdev: *mut device, dev: CUInt) -> CInt {
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut err: CInt;

    err = snd_cmi8330_card_new(pdev, dev as CInt, &mut card);
    if err < 0 {
        return err;
    }
    err = snd_cmi8330_probe(card, dev as CInt);
    if err < 0 {
        return err;
    }
    dev_set_drvdata(pdev, card as *mut core::ffi::c_void);
    0
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn snd_cmi8330_isa_suspend(dev: *mut device, n: CUInt, state: pm_message_t) -> CInt {
    snd_cmi8330_suspend(dev_get_drvdata(dev) as *mut snd_card)
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn snd_cmi8330_isa_resume(dev: *mut device, n: CUInt) -> CInt {
    snd_cmi8330_resume(dev_get_drvdata(dev) as *mut snd_card)
}

const DEV_NAME: &[u8] = b"cmi8330\0";

static mut snd_cmi8330_driver: isa_driver = isa_driver {
    match_: Some(snd_cmi8330_isa_match),
    probe: Some(snd_cmi8330_isa_probe),
    #[cfg(CONFIG_PM)]
    suspend: Some(snd_cmi8330_isa_suspend),
    #[cfg(CONFIG_PM)]
    resume: Some(snd_cmi8330_isa_resume),
    driver: device_driver {
        name: DEV_NAME.as_ptr() as *const i8,
    },
};

#[cfg(CONFIG_PNP)]
unsafe extern "C" fn snd_cmi8330_pnp_detect(pcard: *mut pnp_card_link, pid: *const pnp_card_device_id) -> CInt {
    static mut dev: CInt = 0;
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut res: CInt;

    while dev < SNDRV_CARDS as CInt {
        if enable[dev as usize] && isapnp[dev as usize] {
            break;
        }
        dev += 1;
    }
    if dev >= SNDRV_CARDS as CInt {
        return -ENODEV;
    }

    res = snd_cmi8330_card_new(&mut (*(*pcard).card).dev, dev, &mut card);
    if res < 0 {
        return res;
    }
    res = snd_cmi8330_pnp(dev, (*card).private_data as *mut snd_cmi8330, pcard, pid);
    if res < 0 {
        dev_err((*card).dev, c"PnP detection failed\n".as_ptr());
        return res;
    }
    res = snd_cmi8330_probe(card, dev);
    if res < 0 {
        return res;
    }
    pnp_set_card_drvdata(pcard, card as *mut core::ffi::c_void);
    dev += 1;
    0
}

#[cfg(all(CONFIG_PNP, CONFIG_PM))]
unsafe extern "C" fn snd_cmi8330_pnp_suspend(pcard: *mut pnp_card_link, state: pm_message_t) -> CInt {
    snd_cmi8330_suspend(pnp_get_card_drvdata(pcard) as *mut snd_card)
}

#[cfg(all(CONFIG_PNP, CONFIG_PM))]
unsafe extern "C" fn snd_cmi8330_pnp_resume(pcard: *mut pnp_card_link) -> CInt {
    snd_cmi8330_resume(pnp_get_card_drvdata(pcard) as *mut snd_card)
}

#[cfg(CONFIG_PNP)]
static mut cmi8330_pnpc_driver: pnp_card_driver = pnp_card_driver {
    flags: PNP_DRIVER_RES_DISABLE,
    name: b"cmi8330\0".as_ptr() as *const i8,
    id_table: snd_cmi8330_pnpids.as_ptr(),
    probe: Some(snd_cmi8330_pnp_detect),
    #[cfg(CONFIG_PM)]
    suspend: Some(snd_cmi8330_pnp_suspend),
    #[cfg(CONFIG_PM)]
    resume: Some(snd_cmi8330_pnp_resume),
};

unsafe extern "C" fn alsa_card_cmi8330_init() -> CInt {
    let mut err: CInt;

    err = isa_register_driver(&mut snd_cmi8330_driver, SNDRV_CARDS as CUInt);
    #[cfg(CONFIG_PNP)]
    {
        if err == 0 {
            isa_registered = 1;
        }

        err = pnp_register_card_driver(&mut cmi8330_pnpc_driver);
        if err == 0 {
            pnp_registered = 1;
        }

        if isa_registered != 0 {
            err = 0;
        }
    }
    err
}

unsafe extern "C" fn alsa_card_cmi8330_exit() {
    #[cfg(CONFIG_PNP)]
    {
        if pnp_registered != 0 {
            pnp_unregister_card_driver(&mut cmi8330_pnpc_driver);
        }

        if isa_registered != 0 {
            isa_unregister_driver(&mut snd_cmi8330_driver);
        }
    }
    #[cfg(not(CONFIG_PNP))]
    {
        isa_unregister_driver(&mut snd_cmi8330_driver);
    }
}

// module_init(alsa_card_cmi8330_init)
// module_exit(alsa_card_cmi8330_exit)

extern "C" {
    static THIS_MODULE: *mut module;

    fn strscpy(dst: *mut i8, src: *const i8) -> isize;
    fn sprintf(dst: *mut i8, fmt: *const i8, ...) -> CInt;
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn snd_ctl_add(card: *mut snd_card, elem: *mut snd_kcontrol) -> CInt;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut core::ffi::c_void) -> *mut snd_kcontrol;
    fn snd_pcm_new(card: *mut snd_card, id: *const i8, device: CInt, playback_count: CInt, capture_count: CInt, rpcm: *mut *mut snd_pcm) -> CInt;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut core::ffi::c_void;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: CInt, ops: *mut snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: CInt, data: *mut device, min: usize, max: usize);
    fn snd_sb16dsp_get_pcm_ops(direction: CInt) -> *const snd_pcm_ops;
    fn snd_wss_get_pcm_ops(direction: CInt) -> *const snd_pcm_ops;
    fn snd_wss_create(card: *mut snd_card, port: CLong, cport: CLong, irq: CInt, dma1: CInt, dma2: CInt, hardware: CInt, flags: CInt, rchip: *mut *mut snd_wss) -> CInt;
    fn snd_wss_out(chip: *mut snd_wss, reg: CInt, val: CUChar);
    fn snd_sbdsp_create(card: *mut snd_card, port: CLong, irq: CInt, irq_handler: Option<unsafe extern "C" fn()>, dma8: CInt, dma16: CInt, hardware: CInt, r_chip: *mut *mut snd_sb) -> CInt;
    fn snd_sb16dsp_interrupt();
    fn snd_opl3_create(card: *mut snd_card, l_port: CLong, r_port: CLong, hardware: CInt, integrated: CInt, ropl3: *mut *mut snd_opl3) -> CInt;
    fn snd_opl3_hwdep_new(opl3: *mut snd_opl3, device: CInt, seq_device: CInt, rrawmidi: *mut core::ffi::c_void) -> CInt;
    fn snd_mpu401_uart_new(card: *mut snd_card, device: CInt, hardware: CInt, port: CLong, integrated: CInt, irq: CInt, rrawmidi: *mut core::ffi::c_void) -> CInt;
    fn snd_card_register(card: *mut snd_card) -> CInt;
    fn snd_devm_card_new(dev: *mut device, idx: CInt, xid: *mut i8, module: *mut module, extra_size: usize, card_ret: *mut *mut snd_card) -> CInt;
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn isa_register_driver(driver: *mut isa_driver, ndev: CUInt) -> CInt;
    fn isa_unregister_driver(driver: *mut isa_driver);

    #[cfg(ENABLE_SB_MIXER)]
    fn snd_sbmixer_write(chip: *mut snd_sb, reg: CUChar, value: CUChar);
    #[cfg(ENABLE_SB_MIXER)]
    fn snd_sbmixer_add_ctl_elem(chip: *mut snd_sb, elem: *const sbmix_elem) -> CInt;
    #[cfg(CONFIG_PM)]
    fn snd_power_change_state(card: *mut snd_card, state: CInt);
    #[cfg(CONFIG_PM)]
    fn snd_sbmixer_suspend(chip: *mut snd_sb);
    #[cfg(CONFIG_PM)]
    fn snd_sbdsp_reset(chip: *mut snd_sb);

    #[cfg(CONFIG_PNP)]
    fn pnp_request_card_device(card: *mut pnp_card_link, id: *const u8, from: *mut pnp_dev) -> *mut pnp_dev;
    #[cfg(CONFIG_PNP)]
    fn pnp_activate_dev(dev: *mut pnp_dev) -> CInt;
    #[cfg(CONFIG_PNP)]
    fn pnp_port_start(dev: *mut pnp_dev, bar: CUInt) -> CLong;
    #[cfg(CONFIG_PNP)]
    fn pnp_dma(dev: *mut pnp_dev, n: CUInt) -> CInt;
    #[cfg(CONFIG_PNP)]
    fn pnp_irq(dev: *mut pnp_dev, n: CUInt) -> CInt;
    #[cfg(CONFIG_PNP)]
    fn pnp_set_card_drvdata(pcard: *mut pnp_card_link, data: *mut core::ffi::c_void);
    #[cfg(CONFIG_PNP)]
    fn pnp_get_card_drvdata(pcard: *mut pnp_card_link) -> *mut core::ffi::c_void;
    #[cfg(CONFIG_PNP)]
    fn pnp_register_card_driver(driver: *mut pnp_card_driver) -> CInt;
    #[cfg(CONFIG_PNP)]
    fn pnp_unregister_card_driver(driver: *mut pnp_card_driver);
}

#[repr(C)]
struct module {
    _private: [u8; 0],
}
#[repr(C)]
struct device {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
struct sbmix_elem {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_opl3 {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_card {
    dev: *mut device,
    private_data: *mut core::ffi::c_void,
    mixername: [i8; 80],
    driver: [i8; 16],
    shortname: [i8; 32],
    longname: [i8; 80],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_pcm_ops {
    open: Option<snd_pcm_open_callback_t>,
}
#[repr(C)]
struct snd_pcm {
    name: [i8; 80],
    private_data: *mut core::ffi::c_void,
}
#[repr(C)]
struct snd_pcm_substream {
    private_data: *mut core::ffi::c_void,
}
#[repr(C)]
struct snd_wss {
    hardware: CInt,
    port: CLong,
    suspend: Option<unsafe extern "C" fn(*mut snd_wss)>,
    resume: Option<unsafe extern "C" fn(*mut snd_wss)>,
}
#[repr(C)]
struct snd_sb {
    hardware: CInt,
}
#[cfg(CONFIG_PNP)]
#[repr(C)]
struct pnp_dev {
    dev: device,
}
#[cfg(CONFIG_PNP)]
#[repr(C)]
struct pnp_card {
    dev: device,
}
#[cfg(CONFIG_PNP)]
#[repr(C)]
struct pnp_card_link {
    card: *mut pnp_card,
}
#[cfg(CONFIG_PNP)]
#[repr(C)]
struct pnp_id {
    id: [u8; 8],
}
#[cfg(CONFIG_PNP)]
impl pnp_id {
    const fn new(id: [u8; 8]) -> Self {
        Self { id }
    }
}
#[cfg(CONFIG_PNP)]
#[repr(C)]
struct pnp_card_device_id {
    id: [u8; 8],
    devs: [pnp_id; 4],
}
#[repr(C)]
struct device_driver {
    name: *const i8,
}
#[repr(C)]
struct isa_driver {
    match_: Option<unsafe extern "C" fn(*mut device, CUInt) -> CInt>,
    probe: Option<unsafe extern "C" fn(*mut device, CUInt) -> CInt>,
    #[cfg(CONFIG_PM)]
    suspend: Option<unsafe extern "C" fn(*mut device, CUInt, pm_message_t) -> CInt>,
    #[cfg(CONFIG_PM)]
    resume: Option<unsafe extern "C" fn(*mut device, CUInt) -> CInt>,
    driver: device_driver,
}
#[cfg(CONFIG_PNP)]
#[repr(C)]
struct pnp_card_driver {
    flags: CInt,
    name: *const i8,
    id_table: *const pnp_card_device_id,
    probe: Option<unsafe extern "C" fn(*mut pnp_card_link, *const pnp_card_device_id) -> CInt>,
    #[cfg(CONFIG_PM)]
    suspend: Option<unsafe extern "C" fn(*mut pnp_card_link, pm_message_t) -> CInt>,
    #[cfg(CONFIG_PM)]
    resume: Option<unsafe extern "C" fn(*mut pnp_card_link) -> CInt>,
}

#[cfg(CONFIG_PM)]
type pm_message_t = CInt;

extern "Rust" {
    static CS4231_LEFT_OUTPUT: CInt;
    static CS4231_RIGHT_OUTPUT: CInt;
    static CS4231_AUX2_LEFT_INPUT: CInt;
    static CS4231_AUX2_RIGHT_INPUT: CInt;
    static CS4231_MISC_INFO: CInt;
    static SNDRV_PCM_STREAM_PLAYBACK: CInt;
    static SNDRV_PCM_STREAM_CAPTURE: CInt;
    static SNDRV_DMA_TYPE_DEV: CInt;
    static SNDRV_AUTO_PORT: CLong;
    static WSS_HW_DETECT: CInt;
    static WSS_HW_CMI8330: CInt;
    static SB_HW_AUTO: CInt;
    static SB_HW_16: CInt;
    static OPL3_HW_AUTO: CInt;
    static MPU401_HW_MPU401: CInt;
    static EBUSY: CInt;
    static ENODEV: CInt;
    #[cfg(CONFIG_PM)]
    static SNDRV_CTL_POWER_D3hot: CInt;
    #[cfg(CONFIG_PM)]
    static SNDRV_CTL_POWER_D0: CInt;
    #[cfg(CONFIG_PNP)]
    static PNP_DRIVER_RES_DISABLE: CInt;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
