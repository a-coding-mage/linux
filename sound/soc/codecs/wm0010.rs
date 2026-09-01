// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm0010.c  --  WM0010 DSP Driver
 *
 * Copyright 2012 Wolfson Microelectronics PLC.
 *
 * Authors: Mark Brown <broonie@opensource.wolfsonmicro.com>
 *          Dimitris Papastamos <dp@opensource.wolfsonmicro.com>
 *          Scott Ling <sl@opensource.wolfsonmicro.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u32 = u32;
type u64 = u64;
type usize_t = usize;
type bool_t = bool;
type irqreturn_t = c_int;

const DEVICE_ID_WM0010: u8 = 10;

/* We only support v1 of the .dfw INFO record */
const INFO_VERSION: u8 = 1;

const DFW_CMD_FUSE: u8 = 0x01;
const DFW_CMD_CODE_HDR: u8 = 0x02;
const DFW_CMD_CODE_DATA: u8 = 0x03;
const DFW_CMD_PLL: u8 = 0x04;
const DFW_CMD_INFO: u8 = 0xff;

const GFP_KERNEL: c_uint = 0;
const GFP_DMA: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const ECANCELED: c_int = 125;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQ_NONE: irqreturn_t = 0;
const IRQF_TRIGGER_FALLING: c_uint = 0x00000002;
const IRQF_ONESHOT: c_uint = 0x00002000;
const GPIOD_OUT_HIGH: c_int = 1;
const SND_SOC_NOPM: c_int = 0;
const SND_SOC_BIAS_ON: snd_soc_bias_level = 0;
const SND_SOC_BIAS_PREPARE: snd_soc_bias_level = 1;
const SND_SOC_BIAS_STANDBY: snd_soc_bias_level = 2;
const SND_SOC_BIAS_OFF: snd_soc_bias_level = 3;
const SNDRV_PCM_RATE_44100: u32 = 1 << 0;
const SNDRV_PCM_RATE_48000: u32 = 1 << 1;
const SNDRV_PCM_FMTBIT_S8: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 3;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 4;

type snd_soc_bias_level = c_int;

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
struct spi_device {
    dev: device,
    irq: c_int,
    max_speed_hz: c_uint,
}

#[repr(C)]
struct firmware {
    size: usize_t,
    data: *const u8,
}

#[repr(C)]
struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
struct completion {
    _private: [u8; 0],
}

#[repr(C)]
struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
struct regulator {
    _private: [u8; 0],
}

#[repr(C)]
struct regulator_bulk_data {
    supply: *const c_char,
}

#[repr(C)]
struct wm0010_pdata {
    irq_flags: c_uint,
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct spi_message {
    status: c_int,
    complete: Option<unsafe extern "C" fn(*mut c_void)>,
    context: *mut c_void,
}

#[repr(C)]
struct spi_transfer {
    rx_buf: *mut c_void,
    tx_buf: *mut c_void,
    len: c_uint,
    bits_per_word: c_uint,
    speed_hz: c_uint,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    name: *const c_char,
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int>,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: u32,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
}

#[repr(C)]
struct driver_inner {
    name: *const c_char,
}

#[repr(C)]
struct spi_driver {
    driver: driver_inner,
    probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut spi_device)>,
}

/*
 * C stores length as a packed 24-bit bitfield followed by address and a
 * flexible data array. Rust has no direct stable representation for that exact
 * layout, so this source-level translation keeps the addressable header fields
 * used by the driver and models data as the byte immediately following them.
 */
#[repr(C, packed)]
struct dfw_binrec {
    command: u8,
    length: u32,
    address: u32,
}

#[repr(C)]
struct dfw_inforec {
    info_version: u8,
    tool_major_version: u8,
    tool_minor_version: u8,
    dsp_target: u8,
}

#[repr(C, packed)]
struct dfw_pllrec {
    command: u8,
    length: u32,
    address: u32,
    clkctrl1: u32,
    clkctrl2: u32,
    clkctrl3: u32,
    ldetctrl: u32,
    uart_div: u32,
    spi_div: u32,
}

#[repr(C)]
struct pll_clock_map {
    max_sysclk: c_int,
    max_pll_spi_speed: c_int,
    pll_clkctrl1: u32,
}

static mut pll_clock_map_array: [pll_clock_map; 6] = [
    pll_clock_map { max_sysclk: 22000000, max_pll_spi_speed: 26000000, pll_clkctrl1: 0x00201f11 }, /* 2,32,2  */
    pll_clock_map { max_sysclk: 18000000, max_pll_spi_speed: 26000000, pll_clkctrl1: 0x00203f21 }, /* 2,64,4  */
    pll_clock_map { max_sysclk: 14000000, max_pll_spi_speed: 26000000, pll_clkctrl1: 0x00202620 }, /* 1,39,4  */
    pll_clock_map { max_sysclk: 10000000, max_pll_spi_speed: 22000000, pll_clkctrl1: 0x00203120 }, /* 1,50,4  */
    pll_clock_map { max_sysclk:  6500000, max_pll_spi_speed: 22000000, pll_clkctrl1: 0x00204520 }, /* 1,70,4  */
    pll_clock_map { max_sysclk:  5500000, max_pll_spi_speed: 22000000, pll_clkctrl1: 0x00103f10 }, /* 1,64,2  */
];

type wm0010_state = c_int;
const WM0010_POWER_OFF: wm0010_state = 0;
const WM0010_OUT_OF_RESET: wm0010_state = 1;
const WM0010_BOOTROM: wm0010_state = 2;
const WM0010_STAGE2: wm0010_state = 3;
const WM0010_FIRMWARE: wm0010_state = 4;

#[repr(C)]
struct wm0010_priv {
    component: *mut snd_soc_component,
    lock: mutex,
    dev: *mut device,
    pdata: wm0010_pdata,
    reset: *mut gpio_desc,
    core_supplies: [regulator_bulk_data; 2],
    dbvdd: *mut regulator,
    sysclk: c_int,
    state: wm0010_state,
    boot_failed: bool_t,
    ready: bool_t,
    pll_running: bool_t,
    max_spi_freq: c_int,
    board_max_spi_speed: c_int,
    pll_clkctrl1: u32,
    irq_lock: spinlock_t,
    irq: c_int,
    boot_completion: completion,
}

static wm0010_dapm_widgets: [snd_soc_dapm_widget; 1] = [
    snd_soc_dapm_widget { name: c"CLKIN".as_ptr() },
];

static wm0010_dapm_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: c"SDI2 Capture".as_ptr(), control: ptr::null(), source: c"SDI1 Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"SDI1 Capture".as_ptr(), control: ptr::null(), source: c"SDI2 Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"SDI1 Capture".as_ptr(), control: ptr::null(), source: c"CLKIN".as_ptr() },
    snd_soc_dapm_route { sink: c"SDI2 Capture".as_ptr(), control: ptr::null(), source: c"CLKIN".as_ptr() },
    snd_soc_dapm_route { sink: c"SDI1 Playback".as_ptr(), control: ptr::null(), source: c"CLKIN".as_ptr() },
    snd_soc_dapm_route { sink: c"SDI2 Playback".as_ptr(), control: ptr::null(), source: c"CLKIN".as_ptr() },
];

unsafe fn wm0010_state_to_str(state: wm0010_state) -> *const c_char {
    static state_to_str: [*const c_char; 5] = [
        c"Power off".as_ptr(),
        c"Out of reset".as_ptr(),
        c"Boot ROM".as_ptr(),
        c"Stage2".as_ptr(),
        c"Firmware".as_ptr(),
    ];

    if state < 0 || state as usize >= state_to_str.len() {
        return c"null".as_ptr();
    }
    state_to_str[state as usize]
}

/* Called with wm0010->lock held */
unsafe fn wm0010_halt(component: *mut snd_soc_component) {
    let wm0010 = snd_soc_component_get_drvdata(component) as *mut wm0010_priv;
    let state: wm0010_state;

    spin_lock_irqsave(&mut (*wm0010).irq_lock, 0);
    state = (*wm0010).state;
    spin_unlock_irqrestore(&mut (*wm0010).irq_lock, 0);

    match state {
        WM0010_POWER_OFF => {
            /* If there's nothing to do, bail out */
            return;
        }
        WM0010_OUT_OF_RESET | WM0010_BOOTROM | WM0010_STAGE2 | WM0010_FIRMWARE => {
            /* Remember to put chip back into reset */
            gpiod_set_value_cansleep((*wm0010).reset, 1);
            /* Disable the regulators */
            regulator_disable((*wm0010).dbvdd);
            regulator_bulk_disable((*wm0010).core_supplies.len() as c_uint, (*wm0010).core_supplies.as_mut_ptr());
        }
        _ => {}
    }

    spin_lock_irqsave(&mut (*wm0010).irq_lock, 0);
    (*wm0010).state = WM0010_POWER_OFF;
    spin_unlock_irqrestore(&mut (*wm0010).irq_lock, 0);
}

#[repr(C)]
struct wm0010_boot_xfer {
    list: list_head,
    component: *mut snd_soc_component,
    done: *mut completion,
    m: spi_message,
    t: spi_transfer,
}

/* Called with wm0010->lock held */
unsafe fn wm0010_mark_boot_failure(wm0010: *mut wm0010_priv) {
    let state: wm0010_state;

    spin_lock_irqsave(&mut (*wm0010).irq_lock, 0);
    state = (*wm0010).state;
    spin_unlock_irqrestore(&mut (*wm0010).irq_lock, 0);

    dev_err((*wm0010).dev, c"Failed to transition from `%s' state to `%s' state\n".as_ptr(),
        wm0010_state_to_str(state), wm0010_state_to_str(state + 1));

    (*wm0010).boot_failed = true;
}

unsafe extern "C" fn wm0010_boot_xfer_complete(data: *mut c_void) {
    let xfer = data as *mut wm0010_boot_xfer;
    let component = (*xfer).component;
    let wm0010 = snd_soc_component_get_drvdata(component) as *mut wm0010_priv;
    let out32 = (*xfer).t.rx_buf as *mut u32;
    let mut i: c_int;

    if (*xfer).m.status != 0 {
        dev_err((*component).dev, c"SPI transfer failed: %d\n".as_ptr(), (*xfer).m.status);
        wm0010_mark_boot_failure(wm0010);
        if !(*xfer).done.is_null() {
            complete((*xfer).done);
        }
        return;
    }

    i = 0;
    while i < ((*xfer).t.len / 4) as c_int {
        dev_dbg((*component).dev, c"%d: %04x\n".as_ptr(), i, *out32.add(i as usize));

        match be32_to_cpu(*out32.add(i as usize)) {
            0xe0e0e0e0 => {
                dev_err((*component).dev, c"%d: ROM error reported in stage 2\n".as_ptr(), i);
                wm0010_mark_boot_failure(wm0010);
            }
            0x55555555 => {
                if (*wm0010).state < WM0010_STAGE2 {
                    i += 1;
                    continue;
                }
                dev_err((*component).dev, c"%d: ROM bootloader running in stage 2\n".as_ptr(), i);
                wm0010_mark_boot_failure(wm0010);
            }
            0x0fed0000 => dev_dbg((*component).dev, c"Stage2 loader running\n".as_ptr()),
            0x0fed0007 => dev_dbg((*component).dev, c"CODE_HDR packet received\n".as_ptr()),
            0x0fed0008 => dev_dbg((*component).dev, c"CODE_DATA packet received\n".as_ptr()),
            0x0fed0009 => dev_dbg((*component).dev, c"Download complete\n".as_ptr()),
            0x0fed000c => dev_dbg((*component).dev, c"Application start\n".as_ptr()),
            0x0fed000e => {
                dev_dbg((*component).dev, c"PLL packet received\n".as_ptr());
                (*wm0010).pll_running = true;
            }
            0x0fed0025 => {
                dev_err((*component).dev, c"Device reports image too long\n".as_ptr());
                wm0010_mark_boot_failure(wm0010);
            }
            0x0fed002c => {
                dev_err((*component).dev, c"Device reports bad SPI packet\n".as_ptr());
                wm0010_mark_boot_failure(wm0010);
            }
            0x0fed0031 => {
                dev_err((*component).dev, c"Device reports SPI read overflow\n".as_ptr());
                wm0010_mark_boot_failure(wm0010);
            }
            0x0fed0032 => {
                dev_err((*component).dev, c"Device reports SPI underclock\n".as_ptr());
                wm0010_mark_boot_failure(wm0010);
            }
            0x0fed0033 => {
                dev_err((*component).dev, c"Device reports bad header packet\n".as_ptr());
                wm0010_mark_boot_failure(wm0010);
            }
            0x0fed0034 => {
                dev_err((*component).dev, c"Device reports invalid packet type\n".as_ptr());
                wm0010_mark_boot_failure(wm0010);
            }
            0x0fed0035 => {
                dev_err((*component).dev, c"Device reports data before header error\n".as_ptr());
                wm0010_mark_boot_failure(wm0010);
            }
            0x0fed0038 => {
                dev_err((*component).dev, c"Device reports invalid PLL packet\n".as_ptr());
            }
            0x0fed003a => {
                dev_err((*component).dev, c"Device reports packet alignment error\n".as_ptr());
                wm0010_mark_boot_failure(wm0010);
            }
            _ => {
                dev_err((*component).dev, c"Unrecognised return 0x%x\n".as_ptr(), be32_to_cpu(*out32.add(i as usize)));
                wm0010_mark_boot_failure(wm0010);
            }
        }

        if (*wm0010).boot_failed {
            break;
        }
        i += 1;
    }

    if !(*xfer).done.is_null() {
        complete((*xfer).done);
    }
}

unsafe fn byte_swap_64(data_in: *mut u64, data_out: *mut u64, len: u32) {
    let mut i: c_int = 0;

    while i < (len / 8) as c_int {
        *data_out.add(i as usize) = swab64(*data_in.add(i as usize));
        i += 1;
    }
}

unsafe fn dfw_data(rec: *const dfw_binrec) -> *const u8 {
    (rec as *const u8).add(8)
}

unsafe fn wm0010_firmware_load(name: *const c_char, component: *mut snd_soc_component) -> c_int {
    let spi = to_spi_device((*component).dev);
    let wm0010 = snd_soc_component_get_drvdata(component) as *mut wm0010_priv;
    let mut xfer_list = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
    let mut xfer: *mut wm0010_boot_xfer;
    let mut ret: c_int;
    let mut done = completion { _private: [] };
    let mut rec: *const dfw_binrec;
    let inforec: *const dfw_inforec;
    let mut img: *mut u64;
    let mut out: *mut u8;
    let dsp: u8;
    let mut len: u32;
    let mut offset: u32;
    let mut fw: *const firmware = ptr::null();

    INIT_LIST_HEAD(&mut xfer_list);

    ret = request_firmware(&mut fw, name, (*component).dev);
    if ret != 0 {
        dev_err((*component).dev, c"Failed to request application(%s): %d\n".as_ptr(), name, ret);
        return ret;
    }

    rec = (*fw).data as *const dfw_binrec;
    inforec = dfw_data(rec) as *const dfw_inforec;
    offset = 0;
    dsp = (*inforec).dsp_target;
    (*wm0010).boot_failed = false;
    if WARN_ON(!list_empty(&mut xfer_list)) != 0 {
        release_firmware(fw);
        return -EINVAL;
    }

    /* First record should be INFO */
    if (*rec).command != DFW_CMD_INFO {
        dev_err((*component).dev, c"First record not INFO\r\n".as_ptr());
        release_firmware(fw);
        return -EINVAL;
    }

    if (*inforec).info_version != INFO_VERSION {
        dev_err((*component).dev, c"Unsupported version (%02d) of INFO record\r\n".as_ptr(), (*inforec).info_version as c_int);
        release_firmware(fw);
        return -EINVAL;
    }

    dev_dbg((*component).dev, c"Version v%02d INFO record found\r\n".as_ptr(), (*inforec).info_version as c_int);

    /* Check it's a DSP file */
    if dsp != DEVICE_ID_WM0010 {
        dev_err((*component).dev, c"Not a WM0010 firmware file.\r\n".as_ptr());
        release_firmware(fw);
        return -EINVAL;
    }

    /* Skip the info record as we don't need to send it */
    offset = offset.wrapping_add((*rec).length.wrapping_add(8));
    rec = dfw_data(rec).add((*rec).length as usize) as *const dfw_binrec;

    while offset < (*fw).size as u32 {
        dev_dbg((*component).dev, c"Packet: command %d, data length = 0x%x\r\n".as_ptr(), (*rec).command as c_int, (*rec).length);
        len = (*rec).length.wrapping_add(8);

        xfer = kzalloc_obj(size_of::<wm0010_boot_xfer>()) as *mut wm0010_boot_xfer;
        if xfer.is_null() {
            ret = -ENOMEM;
            goto_abort(&mut xfer_list, fw);
            return ret;
        }

        (*xfer).component = component;
        list_add_tail(&mut (*xfer).list, &mut xfer_list);

        out = kzalloc(len as usize, GFP_KERNEL | GFP_DMA) as *mut u8;
        if out.is_null() {
            ret = -ENOMEM;
            goto_abort(&mut xfer_list, fw);
            return ret;
        }
        (*xfer).t.rx_buf = out as *mut c_void;

        img = kzalloc(len as usize, GFP_KERNEL | GFP_DMA) as *mut u64;
        if img.is_null() {
            ret = -ENOMEM;
            goto_abort(&mut xfer_list, fw);
            return ret;
        }
        (*xfer).t.tx_buf = img as *mut c_void;

        byte_swap_64(&(*rec).command as *const u8 as *mut u64, img, len);

        spi_message_init(&mut (*xfer).m);
        (*xfer).m.complete = Some(wm0010_boot_xfer_complete);
        (*xfer).m.context = xfer as *mut c_void;
        (*xfer).t.len = len;
        (*xfer).t.bits_per_word = 8;

        if !(*wm0010).pll_running {
            (*xfer).t.speed_hz = ((*wm0010).sysclk / 6) as c_uint;
        } else {
            (*xfer).t.speed_hz = (*wm0010).max_spi_freq as c_uint;

            if (*wm0010).board_max_spi_speed != 0
                && (*wm0010).board_max_spi_speed < (*wm0010).max_spi_freq
            {
                (*xfer).t.speed_hz = (*wm0010).board_max_spi_speed as c_uint;
            }
        }

        /* Store max usable spi frequency for later use */
        (*wm0010).max_spi_freq = (*xfer).t.speed_hz as c_int;

        spi_message_add_tail(&mut (*xfer).t, &mut (*xfer).m);

        offset = offset.wrapping_add((*rec).length.wrapping_add(8));
        rec = dfw_data(rec).add((*rec).length as usize) as *const dfw_binrec;

        if offset >= (*fw).size as u32 {
            dev_dbg((*component).dev, c"All transfers scheduled\n".as_ptr());
            (*xfer).done = &mut done;
        }

        ret = spi_async(spi, &mut (*xfer).m);
        if ret != 0 {
            dev_err((*component).dev, c"Write failed: %d\n".as_ptr(), ret);
            goto_abort(&mut xfer_list, fw);
            return ret;
        }

        if (*wm0010).boot_failed {
            dev_dbg((*component).dev, c"Boot fail!\n".as_ptr());
            ret = -EINVAL;
            goto_abort(&mut xfer_list, fw);
            return ret;
        }
    }

    wait_for_completion(&mut done);

    ret = 0;
    goto_abort(&mut xfer_list, fw);
    ret
}

unsafe fn goto_abort(xfer_list: *mut list_head, fw: *const firmware) {
    while !list_empty(xfer_list) {
        let xfer = list_first_entry(xfer_list) as *mut wm0010_boot_xfer;
        kfree((*xfer).t.rx_buf);
        kfree((*xfer).t.tx_buf);
        list_del(&mut (*xfer).list);
        kfree(xfer as *mut c_void);
    }
    if !fw.is_null() {
        release_firmware(fw);
    }
}

unsafe fn wm0010_stage2_load(component: *mut snd_soc_component) -> c_int {
    let spi = to_spi_device((*component).dev);
    let wm0010 = snd_soc_component_get_drvdata(component) as *mut wm0010_priv;
    let mut m = spi_message { status: 0, complete: None, context: ptr::null_mut() };
    let mut t = spi_transfer { rx_buf: ptr::null_mut(), tx_buf: ptr::null_mut(), len: 0, bits_per_word: 0, speed_hz: 0 };
    let mut i: c_int;
    let mut ret: c_int = 0;
    let mut fw: *const firmware = ptr::null();

    ret = request_firmware(&mut fw, c"wm0010_stage2.bin".as_ptr(), (*component).dev);
    if ret != 0 {
        dev_err((*component).dev, c"Failed to request stage2 loader: %d\n".as_ptr(), ret);
        return ret;
    }

    dev_dbg((*component).dev, c"Downloading %zu byte stage 2 loader\n".as_ptr(), (*fw).size);

    /* Copy to local buffer first as vmalloc causes problems for dma */
    let img = kmemdup((*fw).data as *const c_void, (*fw).size, GFP_KERNEL | GFP_DMA) as *mut u32;
    if img.is_null() {
        release_firmware(fw);
        return -ENOMEM;
    }

    let out = kzalloc((*fw).size, GFP_KERNEL | GFP_DMA) as *mut u8;
    if out.is_null() {
        kfree(img as *mut c_void);
        release_firmware(fw);
        return -ENOMEM;
    }

    spi_message_init(&mut m);
    memset(&mut t as *mut _ as *mut c_void, 0, size_of::<spi_transfer>());
    t.rx_buf = out as *mut c_void;
    t.tx_buf = img as *mut c_void;
    t.len = (*fw).size as c_uint;
    t.bits_per_word = 8;
    t.speed_hz = ((*wm0010).sysclk / 10) as c_uint;
    spi_message_add_tail(&mut t, &mut m);

    dev_dbg((*component).dev, c"Starting initial download at %dHz\n".as_ptr(), t.speed_hz);

    ret = spi_sync(spi, &mut m);
    if ret != 0 {
        dev_err((*component).dev, c"Initial download failed: %d\n".as_ptr(), ret);
        kfree(out as *mut c_void);
        kfree(img as *mut c_void);
        release_firmware(fw);
        return ret;
    }

    /* Look for errors from the boot ROM */
    i = 0;
    while i < (*fw).size as c_int {
        if *out.add(i as usize) != 0x55 {
            dev_err((*component).dev, c"Boot ROM error: %x in %d\n".as_ptr(), *out.add(i as usize) as c_int, i);
            wm0010_mark_boot_failure(wm0010);
            kfree(out as *mut c_void);
            kfree(img as *mut c_void);
            release_firmware(fw);
            return -EBUSY;
        }
        i += 1;
    }

    kfree(out as *mut c_void);
    kfree(img as *mut c_void);
    release_firmware(fw);
    0
}

unsafe fn wm0010_boot(component: *mut snd_soc_component) -> c_int {
    let spi = to_spi_device((*component).dev);
    let wm0010 = snd_soc_component_get_drvdata(component) as *mut wm0010_priv;
    let mut flags: c_ulong = 0;
    let mut ret: c_int;
    let mut m = spi_message { status: 0, complete: None, context: ptr::null_mut() };
    let mut t = spi_transfer { rx_buf: ptr::null_mut(), tx_buf: ptr::null_mut(), len: 0, bits_per_word: 0, speed_hz: 0 };
    let mut pll_rec = dfw_pllrec {
        command: 0, length: 0, address: 0, clkctrl1: 0, clkctrl2: 0,
        clkctrl3: 0, ldetctrl: 0, uart_div: 0, spi_div: 0,
    };
    let mut p: *mut u32;
    let len: u32;
    let img_swap: *mut u64;
    let out: *mut u8;
    let mut i: c_int;

    spin_lock_irqsave(&mut (*wm0010).irq_lock, flags);
    if (*wm0010).state != WM0010_POWER_OFF {
        dev_warn((*wm0010).dev, c"DSP already powered up!\n".as_ptr());
    }
    spin_unlock_irqrestore(&mut (*wm0010).irq_lock, flags);

    if (*wm0010).sysclk > 26000000 {
        dev_err((*component).dev, c"Max DSP clock frequency is 26MHz\n".as_ptr());
        ret = -ECANCELED;
        return ret;
    }

    mutex_lock(&mut (*wm0010).lock);
    (*wm0010).pll_running = false;

    dev_dbg((*component).dev, c"max_spi_freq: %d\n".as_ptr(), (*wm0010).max_spi_freq);

    ret = regulator_bulk_enable((*wm0010).core_supplies.len() as c_uint, (*wm0010).core_supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(&mut (*spi).dev, c"Failed to enable core supplies: %d\n".as_ptr(), ret);
        mutex_unlock(&mut (*wm0010).lock);
        return ret;
    }

    ret = regulator_enable((*wm0010).dbvdd);
    if ret != 0 {
        dev_err(&mut (*spi).dev, c"Failed to enable DBVDD: %d\n".as_ptr(), ret);
        mutex_unlock(&mut (*wm0010).lock);
        regulator_bulk_disable((*wm0010).core_supplies.len() as c_uint, (*wm0010).core_supplies.as_mut_ptr());
        return ret;
    }

    /* Release reset */
    gpiod_set_value_cansleep((*wm0010).reset, 0);
    spin_lock_irqsave(&mut (*wm0010).irq_lock, flags);
    (*wm0010).state = WM0010_OUT_OF_RESET;
    spin_unlock_irqrestore(&mut (*wm0010).irq_lock, flags);

    if wait_for_completion_timeout(&mut (*wm0010).boot_completion, msecs_to_jiffies(20)) == 0 {
        dev_err((*component).dev, c"Failed to get interrupt from DSP\n".as_ptr());
    }

    spin_lock_irqsave(&mut (*wm0010).irq_lock, flags);
    (*wm0010).state = WM0010_BOOTROM;
    spin_unlock_irqrestore(&mut (*wm0010).irq_lock, flags);

    ret = wm0010_stage2_load(component);
    if ret != 0 {
        wm0010_halt(component);
        mutex_unlock(&mut (*wm0010).lock);
        return ret;
    }

    if wait_for_completion_timeout(&mut (*wm0010).boot_completion, msecs_to_jiffies(20)) == 0 {
        dev_err((*component).dev, c"Failed to get interrupt from DSP loader.\n".as_ptr());
    }

    spin_lock_irqsave(&mut (*wm0010).irq_lock, flags);
    (*wm0010).state = WM0010_STAGE2;
    spin_unlock_irqrestore(&mut (*wm0010).irq_lock, flags);

    /* Only initialise PLL if max_spi_freq initialised */
    if (*wm0010).max_spi_freq != 0 {
        /* Initialise a PLL record */
        memset(&mut pll_rec as *mut _ as *mut c_void, 0, size_of::<dfw_pllrec>());
        pll_rec.command = DFW_CMD_PLL;
        pll_rec.length = (size_of::<dfw_pllrec>() - 8) as u32;

        /* On wm0010 only the CLKCTRL1 value is used */
        pll_rec.clkctrl1 = (*wm0010).pll_clkctrl1;

        ret = -ENOMEM;
        len = pll_rec.length + 8;
        out = kzalloc(len as usize, GFP_KERNEL | GFP_DMA) as *mut u8;
        if out.is_null() {
            wm0010_halt(component);
            mutex_unlock(&mut (*wm0010).lock);
            return ret;
        }

        img_swap = kzalloc(len as usize, GFP_KERNEL | GFP_DMA) as *mut u64;
        if img_swap.is_null() {
            kfree(out as *mut c_void);
            wm0010_halt(component);
            mutex_unlock(&mut (*wm0010).lock);
            return ret;
        }

        /* We need to re-order for 0010 */
        byte_swap_64(&mut pll_rec as *mut _ as *mut u64, img_swap, len);

        spi_message_init(&mut m);
        memset(&mut t as *mut _ as *mut c_void, 0, size_of::<spi_transfer>());
        t.rx_buf = out as *mut c_void;
        t.tx_buf = img_swap as *mut c_void;
        t.len = len;
        t.bits_per_word = 8;
        t.speed_hz = ((*wm0010).sysclk / 6) as c_uint;
        spi_message_add_tail(&mut t, &mut m);

        ret = spi_sync(spi, &mut m);
        if ret != 0 {
            dev_err((*component).dev, c"First PLL write failed: %d\n".as_ptr(), ret);
            kfree(img_swap as *mut c_void);
            kfree(out as *mut c_void);
            wm0010_halt(component);
            mutex_unlock(&mut (*wm0010).lock);
            return ret;
        }

        /* Use a second send of the message to get the return status */
        ret = spi_sync(spi, &mut m);
        if ret != 0 {
            dev_err((*component).dev, c"Second PLL write failed: %d\n".as_ptr(), ret);
            kfree(img_swap as *mut c_void);
            kfree(out as *mut c_void);
            wm0010_halt(component);
            mutex_unlock(&mut (*wm0010).lock);
            return ret;
        }

        p = out as *mut u32;

        /* Look for PLL active code from the DSP */
        i = 0;
        while i < (len / 4) as c_int {
            if *p == 0x0e00ed0f {
                dev_dbg((*component).dev, c"PLL packet received\n".as_ptr());
                (*wm0010).pll_running = true;
                break;
            }
            p = p.add(1);
            i += 1;
        }

        kfree(img_swap as *mut c_void);
        kfree(out as *mut c_void);
    } else {
        dev_dbg((*component).dev, c"Not enabling DSP PLL.".as_ptr());
    }

    ret = wm0010_firmware_load(c"wm0010.dfw".as_ptr(), component);

    if ret != 0 {
        wm0010_halt(component);
        mutex_unlock(&mut (*wm0010).lock);
        return ret;
    }

    spin_lock_irqsave(&mut (*wm0010).irq_lock, flags);
    (*wm0010).state = WM0010_FIRMWARE;
    spin_unlock_irqrestore(&mut (*wm0010).irq_lock, flags);

    mutex_unlock(&mut (*wm0010).lock);

    0
}

unsafe extern "C" fn wm0010_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let wm0010 = snd_soc_component_get_drvdata(component) as *mut wm0010_priv;
    let dapm = snd_soc_component_to_dapm(component);

    match level {
        SND_SOC_BIAS_ON => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_PREPARE {
                wm0010_boot(component);
            }
        }
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_PREPARE {
                mutex_lock(&mut (*wm0010).lock);
                wm0010_halt(component);
                mutex_unlock(&mut (*wm0010).lock);
            }
        }
        SND_SOC_BIAS_OFF => {}
        _ => {}
    }

    0
}

unsafe extern "C" fn wm0010_set_sysclk(
    component: *mut snd_soc_component,
    _source: c_int,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let wm0010 = snd_soc_component_get_drvdata(component) as *mut wm0010_priv;
    let mut i: c_uint;

    (*wm0010).sysclk = freq as c_int;

    if freq < pll_clock_map_array[pll_clock_map_array.len() - 1].max_sysclk as c_uint {
        (*wm0010).max_spi_freq = 0;
    } else {
        i = 0;
        while (i as usize) < pll_clock_map_array.len() {
            if freq >= pll_clock_map_array[i as usize].max_sysclk as c_uint {
                (*wm0010).max_spi_freq = pll_clock_map_array[i as usize].max_pll_spi_speed;
                (*wm0010).pll_clkctrl1 = pll_clock_map_array[i as usize].pll_clkctrl1;
                break;
            }
            i += 1;
        }
    }

    0
}

unsafe extern "C" fn wm0010_probe(component: *mut snd_soc_component) -> c_int {
    let wm0010 = snd_soc_component_get_drvdata(component) as *mut wm0010_priv;

    (*wm0010).component = component;

    0
}

static soc_component_dev_wm0010: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm0010_probe),
    set_bias_level: Some(wm0010_set_bias_level),
    set_sysclk: Some(wm0010_set_sysclk),
    dapm_widgets: wm0010_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm0010_dapm_widgets.len() as c_uint,
    dapm_routes: wm0010_dapm_routes.as_ptr(),
    num_dapm_routes: wm0010_dapm_routes.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

const WM0010_RATES: u32 = SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000;
const WM0010_FORMATS: u64 = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

static mut wm0010_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"wm0010-sdi1".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"SDI1 Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: WM0010_RATES,
            formats: WM0010_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"SDI1 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: WM0010_RATES,
            formats: WM0010_FORMATS,
        },
    },
    snd_soc_dai_driver {
        name: c"wm0010-sdi2".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"SDI2 Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: WM0010_RATES,
            formats: WM0010_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"SDI2 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: WM0010_RATES,
            formats: WM0010_FORMATS,
        },
    },
];

unsafe extern "C" fn wm0010_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let wm0010 = data as *mut wm0010_priv;

    match (*wm0010).state {
        WM0010_OUT_OF_RESET | WM0010_BOOTROM | WM0010_STAGE2 => {
            spin_lock(&mut (*wm0010).irq_lock);
            complete(&mut (*wm0010).boot_completion);
            spin_unlock(&mut (*wm0010).irq_lock);
            return IRQ_HANDLED;
        }
        _ => {
            return IRQ_NONE;
        }
    }
}

unsafe extern "C" fn wm0010_spi_probe(spi: *mut spi_device) -> c_int {
    let mut ret: c_int;
    let mut trigger: c_uint;
    let irq: c_int;
    let wm0010: *mut wm0010_priv;

    wm0010 = devm_kzalloc(&mut (*spi).dev, size_of::<wm0010_priv>(), GFP_KERNEL) as *mut wm0010_priv;
    if wm0010.is_null() {
        return -ENOMEM;
    }

    mutex_init(&mut (*wm0010).lock);
    spin_lock_init(&mut (*wm0010).irq_lock);

    spi_set_drvdata(spi, wm0010 as *mut c_void);
    (*wm0010).dev = &mut (*spi).dev;

    if !dev_get_platdata(&mut (*spi).dev).is_null() {
        memcpy(
            &mut (*wm0010).pdata as *mut _ as *mut c_void,
            dev_get_platdata(&mut (*spi).dev),
            size_of::<wm0010_pdata>(),
        );
    }

    init_completion(&mut (*wm0010).boot_completion);

    (*wm0010).core_supplies[0].supply = c"AVDD".as_ptr();
    (*wm0010).core_supplies[1].supply = c"DCVDD".as_ptr();
    ret = devm_regulator_bulk_get(
        (*wm0010).dev,
        (*wm0010).core_supplies.len() as c_uint,
        (*wm0010).core_supplies.as_mut_ptr(),
    );
    if ret != 0 {
        dev_err((*wm0010).dev, c"Failed to obtain core supplies: %d\n".as_ptr(), ret);
        return ret;
    }

    (*wm0010).dbvdd = devm_regulator_get((*wm0010).dev, c"DBVDD".as_ptr());
    if IS_ERR((*wm0010).dbvdd as *const c_void) {
        ret = PTR_ERR((*wm0010).dbvdd as *const c_void);
        dev_err((*wm0010).dev, c"Failed to obtain DBVDD: %d\n".as_ptr(), ret);
        return ret;
    }

    (*wm0010).reset = devm_gpiod_get((*wm0010).dev, c"reset".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*wm0010).reset as *const c_void) {
        return dev_err_probe(
            (*wm0010).dev,
            PTR_ERR((*wm0010).reset as *const c_void),
            c"could not get RESET GPIO\n".as_ptr(),
        );
    }
    gpiod_set_consumer_name((*wm0010).reset, c"wm0010 reset".as_ptr());

    (*wm0010).state = WM0010_POWER_OFF;

    irq = (*spi).irq;
    if (*wm0010).pdata.irq_flags != 0 {
        trigger = (*wm0010).pdata.irq_flags;
    } else {
        trigger = IRQF_TRIGGER_FALLING;
    }
    trigger |= IRQF_ONESHOT;

    ret = request_threaded_irq(irq, None, Some(wm0010_irq), trigger, c"wm0010".as_ptr(), wm0010 as *mut c_void);
    if ret != 0 {
        dev_err((*wm0010).dev, c"Failed to request IRQ %d: %d\n".as_ptr(), irq, ret);
        return ret;
    }
    (*wm0010).irq = irq;

    ret = irq_set_irq_wake(irq, 1);
    if ret != 0 {
        dev_err((*wm0010).dev, c"Failed to set IRQ %d as wake source: %d\n".as_ptr(), irq, ret);
        if (*wm0010).irq != 0 {
            free_irq((*wm0010).irq, wm0010 as *mut c_void);
        }
        return ret;
    }

    if (*spi).max_speed_hz != 0 {
        (*wm0010).board_max_spi_speed = (*spi).max_speed_hz as c_int;
    } else {
        (*wm0010).board_max_spi_speed = 0;
    }

    ret = devm_snd_soc_register_component(
        &mut (*spi).dev,
        &soc_component_dev_wm0010,
        wm0010_dai.as_mut_ptr(),
        wm0010_dai.len() as c_uint,
    );
    if ret < 0 {
        irq_set_irq_wake((*wm0010).irq, 0);
        if (*wm0010).irq != 0 {
            free_irq((*wm0010).irq, wm0010 as *mut c_void);
        }
        return ret;
    }

    0
}

unsafe extern "C" fn wm0010_spi_remove(spi: *mut spi_device) {
    let wm0010 = spi_get_drvdata(spi) as *mut wm0010_priv;

    gpiod_set_value_cansleep((*wm0010).reset, 1);

    irq_set_irq_wake((*wm0010).irq, 0);

    if (*wm0010).irq != 0 {
        free_irq((*wm0010).irq, wm0010 as *mut c_void);
    }
}

static mut wm0010_spi_driver: spi_driver = spi_driver {
    driver: driver_inner {
        name: c"wm0010".as_ptr(),
    },
    probe: Some(wm0010_spi_probe),
    remove: Some(wm0010_spi_remove),
};

/* module_spi_driver(wm0010_spi_driver); */

/* MODULE_DESCRIPTION("ASoC WM0010 driver"); */
/* MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>"); */
/* MODULE_LICENSE("GPL"); */

/* MODULE_FIRMWARE("wm0010.dfw"); */
/* MODULE_FIRMWARE("wm0010_stage2.bin"); */

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn to_spi_device(dev: *mut device) -> *mut spi_device;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char);
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn regulator_bulk_disable(num: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_regulator_bulk_get(dev: *mut device, num: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn complete(done: *mut completion);
    fn init_completion(done: *mut completion);
    fn wait_for_completion(done: *mut completion);
    fn wait_for_completion_timeout(done: *mut completion, timeout: c_ulong) -> c_ulong;
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, device: *mut device) -> c_int;
    fn release_firmware(fw: *const firmware);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kzalloc_obj(size: usize) -> *mut c_void;
    fn kmemdup(src: *const c_void, len: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn swab64(x: u64) -> u64;
    fn be32_to_cpu(x: u32) -> u32;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_empty(head: *mut list_head) -> bool;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_first_entry(head: *mut list_head) -> *mut c_void;
    fn list_del(entry: *mut list_head);
    fn spi_message_init(m: *mut spi_message);
    fn spi_message_add_tail(t: *mut spi_transfer, m: *mut spi_message);
    fn spi_async(spi: *mut spi_device, m: *mut spi_message) -> c_int;
    fn spi_sync(spi: *mut spi_device, m: *mut spi_message) -> c_int;
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut c_void);
    fn spi_get_drvdata(spi: *mut spi_device) -> *mut c_void;
    fn dev_get_platdata(dev: *mut device) -> *const c_void;
    fn request_threaded_irq(
        irq: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        irqflags: c_uint,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn irq_set_irq_wake(irq: c_int, on: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_uint,
    ) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn WARN_ON(condition: bool) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
