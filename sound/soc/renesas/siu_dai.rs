// SPDX-License-Identifier: GPL-2.0+
//
// siu_dai.c - ALSA SoC driver for Renesas SH7343, SH7722 SIU peripheral.
//
// Copyright (C) 2009-2010 Guennadi Liakhovetski <g.liakhovetski@gmx.de>
// Copyright (C) 2006 Carlos Munoz <carlos@kenati.com>

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

// Dependencies originally included from Linux, ALSA SoC, asm/siu.h, and "siu.h".

type u32 = u32;

const SIU_MAX_VOLUME: u32 = 0x7fff;

const PRAM_SIZE: usize = 0x2000;
const XRAM_SIZE: usize = 0x800;
const YRAM_SIZE: usize = 0x800;

const XRAM_OFFSET: usize = 0x4000;
const YRAM_OFFSET: usize = 0x6000;
const REG_OFFSET: usize = 0xc000;

const PLAYBACK_ENABLED: u64 = 1;
const CAPTURE_ENABLED: u64 = 2;

const VOLUME_CAPTURE: c_long = 0;
const VOLUME_PLAYBACK: c_long = 1;
const DFLT_VOLUME_LEVEL: u32 = 0x08000800;

#[repr(C)]
struct format_flag {
    i2s: u32,
    pcm: u32,
    spdif: u32,
    mask: u32,
}

#[repr(C)]
struct port_flag {
    playback: format_flag,
    capture: format_flag,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    start: usize,
}

#[repr(C)]
pub struct platform_device {
    dev: device,
    name: *const c_char,
}

#[repr(C)]
pub struct firmware {
    data: *const c_void,
    size: usize,
}

#[repr(C)]
pub struct snd_card {
    dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm {
    card: *mut snd_card,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    channels: c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    stream: c_int,
    pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_soc_dai {
    dev: *mut device,
}

#[repr(C)]
pub struct snd_kcontrol {
    private_value: c_long,
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    min: c_long,
    max: c_long,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    type_: c_uint,
    count: c_uint,
    value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    value: [c_long; 2],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    iface: c_uint,
    name: *const c_char,
    index: c_uint,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    private_value: c_long,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    info: u32,
    formats: u32,
    rates: u32,
    rate_min: u32,
    rate_max: u32,
    channels_min: u32,
    channels_max: u32,
    buffer_bytes_max: usize,
    period_bytes_min: usize,
    period_bytes_max: usize,
    periods_min: u32,
    periods_max: u32,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    channels_min: u32,
    channels_max: u32,
    formats: u32,
    rates: u32,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct driver_inner {
    name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    driver: driver_inner,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
pub struct spbpar_entry {
    ab1a: u32,
    ab0a: u32,
    dir: u32,
    event: u32,
    stfifo: u32,
    trdat: u32,
}

#[repr(C)]
pub struct siu_firmware {
    yram0: *mut u32,
    pram0: *mut u32,
    pram1: *mut u32,
    yram_fir_coeff: *mut u32,
    spbpar: *mut spbpar_entry,
}

#[repr(C)]
pub struct siu_stream {
    volume: u32,
    rw_flg: c_int,
}

#[repr(C)]
pub struct siu_port {
    pcm: *mut snd_pcm,
    playback: siu_stream,
    capture: siu_stream,
    stfifo: u32,
    trdat: u32,
    play_cap: u64,
}

#[repr(C)]
pub struct siu_info {
    reg: *mut u32,
    pram: *mut u32,
    xram: *mut u32,
    yram: *mut u32,
    fw: siu_firmware,
    port_id: c_int,
    dev: *mut device,
}

unsafe extern "C" {
    static siu_component: c_void;

    fn siu_write32(addr: *mut u32, val: u32);
    fn siu_read32(addr: *mut u32) -> u32;
    fn udelay(usecs: c_uint);
    fn cpu_relax();
    fn snd_kcontrol_chip(kctrl: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_set_runtime_hwparams(substream: *mut snd_pcm_substream, hw: *const snd_pcm_hardware);
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_uint) -> c_int;
    fn siu_port_info(substream: *mut snd_pcm_substream) -> *mut siu_port;
    fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_put(clk: *mut clk);
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn devm_kmalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, device: *mut device) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn platform_get_resource(pdev: *mut platform_device, ty: c_uint, num: c_uint) -> *mut resource;
    fn resource_size(res: *mut resource) -> usize;
    fn devm_request_mem_region(
        dev: *mut device,
        start: usize,
        n: usize,
        name: *const c_char,
    ) -> *mut resource;
    fn devm_ioremap(dev: *mut device, offset: usize, size: usize) -> *mut u32;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const c_void,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn kfree(ptr: *mut c_void);
    fn kzalloc_obj(size: usize) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn WARN_ON(condition: c_int) -> bool;
}

unsafe fn dev_dbg(_dev: *mut device, _fmt: *const c_char) {}
unsafe fn dev_err(_dev: *mut device, _fmt: *const c_char) {}

unsafe extern "C" {
    static SIU_PORT_NUM: usize;
}

const SIU_PORT_A: usize = 0;
const SIU_PORT_B: usize = 1;
const SIU_SRCTL: usize = 0;
const SIU_CKCTL: usize = 0;
const SIU_BRGASEL: usize = 0;
const SIU_BRRA: usize = 0;
const SIU_BRGBSEL: usize = 0;
const SIU_BRRB: usize = 0;
const SIU_IFCTL: usize = 0;
const SIU_SFORM: usize = 0;
const SIU_SBDVCA: usize = 0;
const SIU_SBDVCB: usize = 0;
const SIU_DPAK: usize = 0;
const SIU_TRDAT: usize = 0;
const SIU_SBACTIV: usize = 0;
const SIU_SBCTL: usize = 0;
const SIU_SBPSET: usize = 0;
const PRAM0_SIZE: c_int = 0;
const PRAM1_SIZE: c_int = 0;
const XRAM0_SIZE: c_int = 0;
const XRAM1_SIZE: c_int = 0;
const XRAM2_SIZE: c_int = 0;
const YRAM_DEF_SIZE: c_int = 0;
const YRAM_FIR_SIZE: c_int = 0;
const YRAM_IIR_SIZE: c_int = 0;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 0;
const SNDRV_PCM_FMTBIT_S16: u32 = 0;
const SNDRV_PCM_RATE_8000_48000: u32 = 0;
const SIU_BUFFER_BYTES_MAX: usize = 0;
const SIU_PERIOD_BYTES_MIN: usize = 0;
const SIU_PERIOD_BYTES_MAX: usize = 0;
const SIU_PERIODS_MIN: u32 = 0;
const SIU_PERIODS_MAX: u32 = 0;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_HW_PARAM_PERIODS: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SIU_CLKA_PLL: c_int = 0;
const SIU_CLKA_EXT: c_int = 0;
const SIU_CLKB_PLL: c_int = 0;
const SIU_CLKB_EXT: c_int = 0;
const IORESOURCE_MEM: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;

static mut siu_i2s_data: *mut siu_info = ptr::null_mut();

static mut siu_flags: [port_flag; 2] = [
    port_flag {
        playback: format_flag {
            i2s: 0x50000000,
            pcm: 0x40000000,
            spdif: 0x80000000, /* not on all SIU versions */
            mask: 0xd0000000,
        },
        capture: format_flag {
            i2s: 0x05000000,
            pcm: 0x04000000,
            spdif: 0x08000000,
            mask: 0x0d000000,
        },
    },
    port_flag {
        playback: format_flag {
            i2s: 0x00500000,
            pcm: 0x00400000,
            spdif: 0, /* impossible - turn off */
            mask: 0x00500000,
        },
        capture: format_flag {
            i2s: 0x00050000,
            pcm: 0x00040000,
            spdif: 0, /* impossible - turn off */
            mask: 0x00050000,
        },
    },
];

unsafe extern "C" fn siu_dai_start(port_info: *mut siu_port) {
    let info = siu_i2s_data;
    let base = (*info).reg;

    dev_dbg((*(*(*port_info).pcm).card).dev, c"%s\n".as_ptr());

    /* Issue software reset to siu */
    siu_write32(base.add(SIU_SRCTL), 0);

    /* Wait for the reset to take effect */
    udelay(1);

    (*port_info).stfifo = 0;
    (*port_info).trdat = 0;

    /* portA, portB, SIU operate */
    siu_write32(base.add(SIU_SRCTL), 0x301);

    /* portA=256fs, portB=256fs */
    siu_write32(base.add(SIU_CKCTL), 0x40400000);

    /* portA's BRG does not divide SIUCKA */
    siu_write32(base.add(SIU_BRGASEL), 0);
    siu_write32(base.add(SIU_BRRA), 0);

    /* portB's BRG divides SIUCKB by half */
    siu_write32(base.add(SIU_BRGBSEL), 1);
    siu_write32(base.add(SIU_BRRB), 0);

    siu_write32(base.add(SIU_IFCTL), 0x44440000);

    /* portA: 32 bit/fs, master; portB: 32 bit/fs, master */
    siu_write32(base.add(SIU_SFORM), 0x0c0c0000);

    /*
     * Volume levels: looks like the DSP firmware implements volume controls
     * differently from what's described in the datasheet
     */
    siu_write32(base.add(SIU_SBDVCA), (*port_info).playback.volume);
    siu_write32(base.add(SIU_SBDVCB), (*port_info).capture.volume);
}

unsafe extern "C" fn siu_dai_stop(_port_info: *mut siu_port) {
    let info = siu_i2s_data;
    let base = (*info).reg;

    /* SIU software reset */
    siu_write32(base.add(SIU_SRCTL), 0);
}

unsafe extern "C" fn siu_dai_spbAselect(port_info: *mut siu_port) {
    let info = siu_i2s_data;
    let fw = &mut (*info).fw;
    let ydef = fw.yram0;
    let idx: u32;

    /* path A use */
    if (*info).port_id == 0 {
        idx = 1; /* portA */
    } else {
        idx = 2; /* portB */
    }

    *ydef.add(0) = ((*fw.spbpar.add(idx as usize)).ab1a << 16)
        | ((*fw.spbpar.add(idx as usize)).ab0a << 8)
        | ((*fw.spbpar.add(idx as usize)).dir << 7)
        | 3;
    *ydef.add(1) = *fw.yram0.add(1); /* 0x03000300 */
    *ydef.add(2) = (16 / 2) << 24;
    *ydef.add(3) = *fw.yram0.add(3); /* 0 */
    *ydef.add(4) = *fw.yram0.add(4); /* 0 */
    *ydef.add(7) = (*fw.spbpar.add(idx as usize)).event;
    (*port_info).stfifo |= (*fw.spbpar.add(idx as usize)).stfifo;
    (*port_info).trdat |= (*fw.spbpar.add(idx as usize)).trdat;
}

unsafe extern "C" fn siu_dai_spbBselect(port_info: *mut siu_port) {
    let info = siu_i2s_data;
    let fw = &mut (*info).fw;
    let ydef = fw.yram0;
    let idx: u32;

    /* path B use */
    if (*info).port_id == 0 {
        idx = 7; /* portA */
    } else {
        idx = 8; /* portB */
    }

    *ydef.add(5) = ((*fw.spbpar.add(idx as usize)).ab1a << 16)
        | ((*fw.spbpar.add(idx as usize)).ab0a << 8)
        | 1;
    *ydef.add(6) = (*fw.spbpar.add(idx as usize)).event;
    (*port_info).stfifo |= (*fw.spbpar.add(idx as usize)).stfifo;
    (*port_info).trdat |= (*fw.spbpar.add(idx as usize)).trdat;
}

unsafe extern "C" fn siu_dai_open(_siu_stream: *mut siu_stream) {
    let info = siu_i2s_data;
    let base = (*info).reg;
    let mut srctl: u32;
    let mut ifctl: u32;

    srctl = siu_read32(base.add(SIU_SRCTL));
    ifctl = siu_read32(base.add(SIU_IFCTL));

    match (*info).port_id as usize {
        SIU_PORT_A => {
            /* portA operates */
            srctl |= 0x200;
            ifctl &= !0xc2;
        }
        SIU_PORT_B => {
            /* portB operates */
            srctl |= 0x100;
            ifctl &= !0x31;
        }
        _ => {}
    }

    siu_write32(base.add(SIU_SRCTL), srctl);
    /* Unmute and configure portA */
    siu_write32(base.add(SIU_IFCTL), ifctl);
}

/*
 * At the moment only fixed Left-upper, Left-lower, Right-upper, Right-lower
 * packing is supported
 */
unsafe extern "C" fn siu_dai_pcmdatapack(_siu_stream: *mut siu_stream) {
    let info = siu_i2s_data;
    let base = (*info).reg;
    let mut dpak: u32;

    dpak = siu_read32(base.add(SIU_DPAK));

    match (*info).port_id as usize {
        SIU_PORT_A => {
            dpak &= !0xc0000000;
        }
        SIU_PORT_B => {
            dpak &= !0x00c00000;
        }
        _ => {}
    }

    siu_write32(base.add(SIU_DPAK), dpak);
}

unsafe extern "C" fn siu_dai_spbstart(port_info: *mut siu_port) -> c_int {
    let info = siu_i2s_data;
    let base = (*info).reg;
    let fw = &mut (*info).fw;
    let ydef = fw.yram0;
    let mut cnt: c_int;
    let mut add: *mut u32;
    let mut ptr: *mut u32;

    /* Load SPB Program in PRAM */
    ptr = fw.pram0;
    add = (*info).pram;
    cnt = 0;
    while cnt < PRAM0_SIZE {
        siu_write32(add, *ptr);
        cnt += 1;
        add = add.add(1);
        ptr = ptr.add(1);
    }

    ptr = fw.pram1;
    add = (*info).pram.add(0x0100 / size_of::<u32>());
    cnt = 0;
    while cnt < PRAM1_SIZE {
        siu_write32(add, *ptr);
        cnt += 1;
        add = add.add(1);
        ptr = ptr.add(1);
    }

    /* XRAM initialization */
    add = (*info).xram;
    cnt = 0;
    while cnt < XRAM0_SIZE + XRAM1_SIZE + XRAM2_SIZE {
        siu_write32(add, 0);
        cnt += 1;
        add = add.add(1);
    }

    /* YRAM variable area initialization */
    add = (*info).yram;
    cnt = 0;
    while cnt < YRAM_DEF_SIZE {
        siu_write32(add, *ydef.add(cnt as usize));
        cnt += 1;
        add = add.add(1);
    }

    /* YRAM FIR coefficient area initialization */
    add = (*info).yram.add(0x0200 / size_of::<u32>());
    cnt = 0;
    while cnt < YRAM_FIR_SIZE {
        siu_write32(add, *fw.yram_fir_coeff.add(cnt as usize));
        cnt += 1;
        add = add.add(1);
    }

    /* YRAM IIR coefficient area initialization */
    add = (*info).yram.add(0x0600 / size_of::<u32>());
    cnt = 0;
    while cnt < YRAM_IIR_SIZE {
        siu_write32(add, 0);
        cnt += 1;
        add = add.add(1);
    }

    siu_write32(base.add(SIU_TRDAT), (*port_info).trdat);
    (*port_info).trdat = 0x0;

    /* SPB start condition: software */
    siu_write32(base.add(SIU_SBACTIV), 0);
    /* Start SPB */
    siu_write32(base.add(SIU_SBCTL), 0xc0000000);
    /* Wait for program to halt */
    cnt = 0x10000;
    loop {
        cnt -= 1;
        if !(cnt != 0 && siu_read32(base.add(SIU_SBCTL)) != 0x80000000) {
            break;
        }
        cpu_relax();
    }

    if cnt == 0 {
        return -EBUSY;
    }

    /* SPB program start address setting */
    siu_write32(base.add(SIU_SBPSET), 0x00400000);
    /* SPB hardware start(FIFOCTL source) */
    siu_write32(base.add(SIU_SBACTIV), 0xc0000000);

    0
}

unsafe extern "C" fn siu_dai_spbstop(port_info: *mut siu_port) {
    let info = siu_i2s_data;
    let base = (*info).reg;

    siu_write32(base.add(SIU_SBACTIV), 0);
    /* SPB stop */
    siu_write32(base.add(SIU_SBCTL), 0);

    (*port_info).stfifo = 0;
}

/*		API functions		*/

/* Playback and capture hardware properties are identical */
static siu_dai_pcm_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED,
    formats: SNDRV_PCM_FMTBIT_S16,
    rates: SNDRV_PCM_RATE_8000_48000,
    rate_min: 8000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: SIU_BUFFER_BYTES_MAX,
    period_bytes_min: SIU_PERIOD_BYTES_MIN,
    period_bytes_max: SIU_PERIOD_BYTES_MAX,
    periods_min: SIU_PERIODS_MIN,
    periods_max: SIU_PERIODS_MAX,
};

unsafe extern "C" fn siu_dai_info_volume(
    kctrl: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let port_info = snd_kcontrol_chip(kctrl) as *mut siu_port;

    dev_dbg((*(*(*port_info).pcm).card).dev, c"%s\n".as_ptr());

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = SIU_MAX_VOLUME as c_long;

    0
}

unsafe extern "C" fn siu_dai_get_volume(
    kctrl: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let port_info = snd_kcontrol_chip(kctrl) as *mut siu_port;
    let dev = (*(*(*port_info).pcm).card).dev;
    let vol: u32;

    dev_dbg(dev, c"%s\n".as_ptr());

    match (*kctrl).private_value {
        VOLUME_PLAYBACK => {
            /* Playback is always on port 0 */
            vol = (*port_info).playback.volume;
            (*ucontrol).value.integer.value[0] = (vol & 0xffff) as c_long;
            (*ucontrol).value.integer.value[1] = ((vol >> 16) & 0xffff) as c_long;
        }
        VOLUME_CAPTURE => {
            /* Capture is always on port 1 */
            vol = (*port_info).capture.volume;
            (*ucontrol).value.integer.value[0] = (vol & 0xffff) as c_long;
            (*ucontrol).value.integer.value[1] = ((vol >> 16) & 0xffff) as c_long;
        }
        _ => {
            dev_err(dev, c"%s() invalid private_value=%ld\n".as_ptr());
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn siu_dai_put_volume(
    kctrl: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let port_info = snd_kcontrol_chip(kctrl) as *mut siu_port;
    let dev = (*(*(*port_info).pcm).card).dev;
    let info = siu_i2s_data;
    let base = (*info).reg;
    let new_vol: u32;
    let cur_vol: u32;

    dev_dbg(dev, c"%s\n".as_ptr());

    if (*ucontrol).value.integer.value[0] < 0
        || (*ucontrol).value.integer.value[0] > SIU_MAX_VOLUME as c_long
        || (*ucontrol).value.integer.value[1] < 0
        || (*ucontrol).value.integer.value[1] > SIU_MAX_VOLUME as c_long
    {
        return -EINVAL;
    }

    new_vol = ((*ucontrol).value.integer.value[0] as u32)
        | (((*ucontrol).value.integer.value[1] as u32) << 16);

    /* See comment above - DSP firmware implementation */
    match (*kctrl).private_value {
        VOLUME_PLAYBACK => {
            /* Playback is always on port 0 */
            cur_vol = (*port_info).playback.volume;
            siu_write32(base.add(SIU_SBDVCA), new_vol);
            (*port_info).playback.volume = new_vol;
        }
        VOLUME_CAPTURE => {
            /* Capture is always on port 1 */
            cur_vol = (*port_info).capture.volume;
            siu_write32(base.add(SIU_SBDVCB), new_vol);
            (*port_info).capture.volume = new_vol;
        }
        _ => {
            dev_err(dev, c"%s() invalid private_value=%ld\n".as_ptr());
            return -EINVAL;
        }
    }

    if cur_vol != new_vol {
        return 1;
    }

    0
}

static playback_controls: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"PCM Playback Volume".as_ptr(),
    index: 0,
    info: Some(siu_dai_info_volume),
    get: Some(siu_dai_get_volume),
    put: Some(siu_dai_put_volume),
    private_value: VOLUME_PLAYBACK,
};

static capture_controls: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"PCM Capture Volume".as_ptr(),
    index: 0,
    info: Some(siu_dai_info_volume),
    get: Some(siu_dai_get_volume),
    put: Some(siu_dai_put_volume),
    private_value: VOLUME_CAPTURE,
};

#[no_mangle]
pub unsafe extern "C" fn siu_init_port(
    port: c_int,
    port_info: *mut *mut siu_port,
    card: *mut snd_card,
) -> c_int {
    let dev = (*card).dev;
    let mut kctrl: *mut snd_kcontrol;
    let mut ret: c_int;

    *port_info = kzalloc_obj(size_of::<siu_port>()) as *mut siu_port;
    if (*port_info).is_null() {
        return -ENOMEM;
    }

    dev_dbg(dev, c"%s: port #%d@%p\n".as_ptr());

    (**port_info).playback.volume = DFLT_VOLUME_LEVEL;
    (**port_info).capture.volume = DFLT_VOLUME_LEVEL;

    /*
     * Add mixer support. The SPB is used to change the volume. Both
     * ports use the same SPB. Therefore, we only register one
     * control instance since it will be used by both channels.
     * In error case we continue without controls.
     */
    kctrl = snd_ctl_new1(&playback_controls, *port_info as *mut c_void);
    ret = snd_ctl_add(card, kctrl);
    if ret < 0 {
        dev_err(dev, c"failed to add playback controls %p port=%d err=%d\n".as_ptr());
    }

    kctrl = snd_ctl_new1(&capture_controls, *port_info as *mut c_void);
    ret = snd_ctl_add(card, kctrl);
    if ret < 0 {
        dev_err(dev, c"failed to add capture controls %p port=%d err=%d\n".as_ptr());
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn siu_free_port(port_info: *mut siu_port) {
    kfree(port_info as *mut c_void);
}

unsafe extern "C" fn siu_dai_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let info = snd_soc_dai_get_drvdata(dai) as *mut siu_info;
    let rt = (*substream).runtime;
    let port_info = siu_port_info(substream);
    let ret: c_int;

    dev_dbg((*(*(*substream).pcm).card).dev, c"%s: port=%d@%p\n".as_ptr());

    snd_soc_set_runtime_hwparams(substream, &siu_dai_pcm_hw);

    ret = snd_pcm_hw_constraint_integer(rt, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        return ret;
    }

    siu_dai_start(port_info);

    0
}

unsafe extern "C" fn siu_dai_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let info = snd_soc_dai_get_drvdata(dai) as *mut siu_info;
    let port_info = siu_port_info(substream);

    dev_dbg((*(*(*substream).pcm).card).dev, c"%s: port=%d@%p\n".as_ptr());

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*port_info).play_cap &= !PLAYBACK_ENABLED;
    } else {
        (*port_info).play_cap &= !CAPTURE_ENABLED;
    }

    /* Stop the siu if the other stream is not using it */
    if (*port_info).play_cap == 0 {
        /* during stmread or stmwrite ? */
        if WARN_ON(((*port_info).playback.rw_flg != 0 || (*port_info).capture.rw_flg != 0) as c_int) {
            return;
        }
        siu_dai_spbstop(port_info);
        siu_dai_stop(port_info);
    }
}

/* PCM part of siu_dai_playback_prepare() / siu_dai_capture_prepare() */
unsafe extern "C" fn siu_dai_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let info = snd_soc_dai_get_drvdata(dai) as *mut siu_info;
    let rt = (*substream).runtime;
    let port_info = siu_port_info(substream);
    let siu_stream: *mut siu_stream;
    let self_: c_int;
    let ret: c_int;

    dev_dbg((*(*(*substream).pcm).card).dev, c"%s: port %d, active streams %lx, %d channels\n".as_ptr());

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        self_ = PLAYBACK_ENABLED as c_int;
        siu_stream = &mut (*port_info).playback;
    } else {
        self_ = CAPTURE_ENABLED as c_int;
        siu_stream = &mut (*port_info).capture;
    }

    /* Set up the siu if not already done */
    if (*port_info).play_cap == 0 {
        (*siu_stream).rw_flg = 0; /* stream-data transfer flag */

        siu_dai_spbAselect(port_info);
        siu_dai_spbBselect(port_info);

        siu_dai_open(siu_stream);

        siu_dai_pcmdatapack(siu_stream);

        ret = siu_dai_spbstart(port_info);
        if ret < 0 {
            return ret;
        }
    } else {
        ret = 0;
    }

    (*port_info).play_cap |= self_ as u64;

    ret
}

/*
 * SIU can set bus format to I2S / PCM / SPDIF independently for playback and
 * capture, however, the current API sets the bus format globally for a DAI.
 */
unsafe extern "C" fn siu_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let info = snd_soc_dai_get_drvdata(dai) as *mut siu_info;
    let base = (*info).reg;
    let mut ifctl: u32;

    dev_dbg((*dai).dev, c"%s: fmt 0x%x on port %d\n".as_ptr());

    if (*info).port_id < 0 {
        return -ENODEV;
    }

    /* Here select between I2S / PCM / SPDIF */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            ifctl = siu_flags[(*info).port_id as usize].playback.i2s
                | siu_flags[(*info).port_id as usize].capture.i2s;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            ifctl = siu_flags[(*info).port_id as usize].playback.pcm
                | siu_flags[(*info).port_id as usize].capture.pcm;
        }
        /* SPDIF disabled - see comment at the top */
        _ => {
            return -EINVAL;
        }
    }

    ifctl |= !(siu_flags[(*info).port_id as usize].playback.mask
        | siu_flags[(*info).port_id as usize].capture.mask)
        & siu_read32(base.add(SIU_IFCTL));
    siu_write32(base.add(SIU_IFCTL), ifctl);

    0
}

unsafe extern "C" fn siu_dai_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    _freq: c_uint,
    dir: c_int,
) -> c_int {
    let siu_clk: *mut clk;
    let parent_clk: *mut clk;
    let siu_name: *const c_char;
    let parent_name: *const c_char;
    let mut ret: c_int;

    if dir != SND_SOC_CLOCK_IN {
        return -EINVAL;
    }

    dev_dbg((*dai).dev, c"%s: using clock %d\n".as_ptr());

    match clk_id {
        SIU_CLKA_PLL => {
            siu_name = c"siua_clk".as_ptr();
            parent_name = c"pll_clk".as_ptr();
        }
        SIU_CLKA_EXT => {
            siu_name = c"siua_clk".as_ptr();
            parent_name = c"siumcka_clk".as_ptr();
        }
        SIU_CLKB_PLL => {
            siu_name = c"siub_clk".as_ptr();
            parent_name = c"pll_clk".as_ptr();
        }
        SIU_CLKB_EXT => {
            siu_name = c"siub_clk".as_ptr();
            parent_name = c"siumckb_clk".as_ptr();
        }
        _ => {
            return -EINVAL;
        }
    }

    siu_clk = clk_get((*dai).dev, siu_name);
    if IS_ERR(siu_clk as *const c_void) {
        dev_err((*dai).dev, c"%s: cannot get a SIU clock: %ld\n".as_ptr());
        return PTR_ERR(siu_clk as *const c_void) as c_int;
    }

    parent_clk = clk_get((*dai).dev, parent_name);
    if IS_ERR(parent_clk as *const c_void) {
        ret = PTR_ERR(parent_clk as *const c_void) as c_int;
        dev_err((*dai).dev, c"cannot get a SIU clock parent: %d\n".as_ptr());
        clk_put(siu_clk);
        return ret;
    }

    ret = clk_set_parent(siu_clk, parent_clk);
    if ret < 0 {
        dev_err((*dai).dev, c"cannot reparent the SIU clock: %d\n".as_ptr());
        clk_put(parent_clk);
        clk_put(siu_clk);
        return ret;
    }

    ret = clk_set_rate(siu_clk, _freq);
    if ret < 0 {
        dev_err((*dai).dev, c"cannot set SIU clock rate: %d\n".as_ptr());
    }

    /* TODO: when clkdev gets reference counting we'll move these to siu_dai_shutdown() */
    clk_put(parent_clk);
    clk_put(siu_clk);

    ret
}

static siu_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(siu_dai_startup),
    shutdown: Some(siu_dai_shutdown),
    prepare: Some(siu_dai_prepare),
    set_sysclk: Some(siu_dai_set_sysclk),
    set_fmt: Some(siu_dai_set_fmt),
};

static mut siu_i2s_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"siu-i2s-dai".as_ptr(),
    playback: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        formats: SNDRV_PCM_FMTBIT_S16,
        rates: SNDRV_PCM_RATE_8000_48000,
    },
    capture: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        formats: SNDRV_PCM_FMTBIT_S16,
        rates: SNDRV_PCM_RATE_8000_48000,
    },
    ops: &siu_dai_ops,
};

unsafe extern "C" fn siu_probe(pdev: *mut platform_device) -> c_int {
    let mut res: *mut resource;
    let region: *mut resource;
    let info: *mut siu_info;
    let mut ret: c_int;

    info = devm_kmalloc(&mut (*pdev).dev, size_of::<siu_info>(), GFP_KERNEL) as *mut siu_info;
    if info.is_null() {
        return -ENOMEM;
    }
    siu_i2s_data = info;
    (*info).dev = &mut (*pdev).dev;

    let mut fw_entry: *const firmware = ptr::null();
    ret = request_firmware(&mut fw_entry, c"siu_spb.bin".as_ptr(), &mut (*pdev).dev);
    if ret != 0 {
        return ret;
    }

    /*
     * Loaded firmware is "const" - read only, but we have to modify it in
     * snd_siu_sh7343_spbAselect() and snd_siu_sh7343_spbBselect()
     */
    memcpy(
        &mut (*info).fw as *mut _ as *mut c_void,
        (*fw_entry).data,
        (*fw_entry).size,
    );

    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        return -ENODEV;
    }

    region = devm_request_mem_region(&mut (*pdev).dev, (*res).start, resource_size(res), (*pdev).name);
    if region.is_null() {
        dev_err(&mut (*pdev).dev, c"SIU region already claimed\n".as_ptr());
        return -EBUSY;
    }

    (*info).pram = devm_ioremap(&mut (*pdev).dev, (*res).start, PRAM_SIZE);
    if (*info).pram.is_null() {
        return -ENOMEM;
    }
    (*info).xram = devm_ioremap(&mut (*pdev).dev, (*res).start + XRAM_OFFSET, XRAM_SIZE);
    if (*info).xram.is_null() {
        return -ENOMEM;
    }
    (*info).yram = devm_ioremap(&mut (*pdev).dev, (*res).start + YRAM_OFFSET, YRAM_SIZE);
    if (*info).yram.is_null() {
        return -ENOMEM;
    }
    (*info).reg = devm_ioremap(
        &mut (*pdev).dev,
        (*res).start + REG_OFFSET,
        resource_size(res) - REG_OFFSET,
    );
    if (*info).reg.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(&mut (*pdev).dev, info as *mut c_void);

    /* register using ARRAY version so we can keep dai name */
    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &siu_component as *const _,
        &mut siu_i2s_dai,
        1,
    );
    if ret < 0 {
        return ret;
    }

    pm_runtime_enable(&mut (*pdev).dev);

    0
}

unsafe extern "C" fn siu_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

static mut siu_driver: platform_driver = platform_driver {
    driver: driver_inner {
        name: c"siu-pcm-audio".as_ptr(),
    },
    probe: Some(siu_probe),
    remove: Some(siu_remove),
};

// module_platform_driver(siu_driver);
// MODULE_AUTHOR("Carlos Munoz <carlos@kenati.com>");
// MODULE_DESCRIPTION("ALSA SoC SH7722 SIU driver");
// MODULE_LICENSE("GPL");
// MODULE_FIRMWARE("siu_spb.bin");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
