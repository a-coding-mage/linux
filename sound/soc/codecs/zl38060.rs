// SPDX-License-Identifier: GPL-2.0-only
//
// Codec driver for Microsemi ZL38060 Connected Home Audio Processor.
//
// Copyright(c) 2020 Sven Van Asbroeck

// The ZL38060 is very flexible and configurable. This driver implements only a
// tiny subset of the chip's possible configurations:
//
// - DSP block bypassed: DAI        routed straight to DACs
//                       microphone routed straight to DAI
// - chip's internal clock is driven by a 12 MHz external crystal
// - chip's DAI connected to CPU is I2S, and bit + frame clock master
// - chip must be strapped for "host boot": in this mode, firmware will be
//   provided by this driver.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type size_t = usize;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type __be16 = u16;
type __be32 = u32;

const fn BIT(n: c_uint) -> c_uint {
    1u32 << n
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_HIGH: c_int = 1;
const THIS_MODULE: *mut module = ptr::null_mut();

const SNDRV_PCM_RATE_8000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_16000: c_uint = 1 << 1;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 2;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 2;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;

const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x00f0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0xf000;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0x1000;

const DRV_NAME: &[u8] = b"zl38060\0";

const ZL38_RATES: c_uint =
    SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000;
const ZL38_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE;

const HBI_FIRMWARE_PAGE: c_uint = 0xFF;
const ZL38_MAX_RAW_XFER: usize = 0x100;

const REG_TDMA_CFG_CLK: c_uint = 0x0262;
const CFG_CLK_PCLK_SHIFT: c_uint = 4;
const CFG_CLK_PCLK_MASK: c_uint = 0x7ff << CFG_CLK_PCLK_SHIFT;
const fn CFG_CLK_PCLK(bits: c_uint) -> c_uint {
    (bits - 1) << CFG_CLK_PCLK_SHIFT
}
const CFG_CLK_MASTER: c_uint = BIT(15);
const CFG_CLK_FSRATE_MASK: c_uint = 0x7;
const CFG_CLK_FSRATE_8KHZ: c_uint = 0x1;
const CFG_CLK_FSRATE_16KHZ: c_uint = 0x2;
const CFG_CLK_FSRATE_48KHZ: c_uint = 0x6;

const REG_CLK_CFG: c_uint = 0x0016;
const CLK_CFG_SOURCE_XTAL: c_uint = BIT(15);

const REG_CLK_STATUS: c_uint = 0x0014;
const CLK_STATUS_HWRST: c_uint = BIT(0);

const REG_PARAM_RESULT: c_uint = 0x0034;
const PARAM_RESULT_READY: c_uint = 0xD3D3;

const REG_PG255_BASE_HI: c_uint = 0x000C;
const fn REG_PG255_OFFS(addr: u32) -> c_uint {
    ((HBI_FIRMWARE_PAGE << 8) | (addr & 0xFF)) as c_uint
}
const REG_FWR_EXEC: c_uint = 0x012C;

const REG_CMD: c_uint = 0x0032;
const REG_HW_REV: c_uint = 0x0020;
const REG_FW_PROD: c_uint = 0x0022;
const REG_FW_REV: c_uint = 0x0024;

const REG_SEMA_FLAGS: c_uint = 0x0006;
const SEMA_FLAGS_BOOT_CMD: c_uint = BIT(0);
const SEMA_FLAGS_APP_REBOOT: c_uint = BIT(1);

const REG_GPIO_DIR: c_uint = 0x02DC;
const REG_GPIO_DAT: c_uint = 0x02DA;

const BOOTCMD_LOAD_COMPLETE: u16 = 0x000D;
const BOOTCMD_FW_GO: u16 = 0x0008;

const FIRMWARE_MAJOR: c_int = 2;
const FIRMWARE_MINOR: c_int = 2;

#[repr(C)]
struct device {
    _private: [u8; 0],
}
#[repr(C)]
struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
struct module {
    _private: [u8; 0],
}
#[repr(C)]
struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct spi_device {
    dev: device,
}

#[repr(C)]
struct snd_pcm_substream {
    stream: c_int,
}

#[repr(C)]
struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
struct firmware {
    size: size_t,
    data: *const u8,
}

#[repr(C)]
struct ihex_binrec {
    addr: __be32,
    len: __be16,
    data: [u8; 0],
}

#[repr(C)]
struct zl38_codec_priv {
    dev: *mut device,
    regmap: *mut regmap,
    is_stream_in_use: [bool_; 2],
    gpio_chip: *mut gpio_chip,
}

#[repr(C)]
struct reg_sequence {
    reg: c_uint,
    def: c_uint,
}

#[repr(C)]
struct snd_soc_dai_ops {
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: c_uint,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
    symmetric_sample_bits: c_uint,
    symmetric_channels: c_uint,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    name: *const c_char,
    id: c_int,
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_component_driver {
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    endianness: c_uint,
}

#[repr(C)]
struct gpio_chip {
    owner: *mut module,
    label: *const c_char,
    base: c_int,
    ngpio: u16,
    direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    set: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    can_sleep: bool_,
    parent: *mut device,
}

#[repr(C)]
struct regmap_bus {
    read: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const c_void,
            size_t,
            *mut c_void,
            size_t,
        ) -> c_int,
    >,
    write: Option<unsafe extern "C" fn(*mut c_void, *const c_void, size_t) -> c_int>,
    max_raw_write: size_t,
    max_raw_read: size_t,
}

#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    reg_stride: c_uint,
    use_single_read: bool_,
    use_single_write: bool_,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct spi_device_id {
    name: *const c_char,
    driver_data: c_ulong,
}

#[repr(C)]
struct driver_private {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct spi_driver {
    driver: driver_private,
    probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
    id_table: *const spi_device_id,
}

unsafe extern "C" {
    fn regmap_read(regmap: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(regmap: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(regmap: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_raw_write(
        regmap: *mut regmap,
        reg: c_uint,
        val: *const c_void,
        val_len: size_t,
    ) -> c_int;
    fn regmap_multi_reg_write(
        regmap: *mut regmap,
        regs: *const reg_sequence,
        num_regs: c_int,
    ) -> c_int;
    fn request_ihex_firmware(
        fw: *mut *const firmware,
        name: *const c_char,
        device: *mut device,
    ) -> c_int;
    fn ihex_next_binrec(rec: *const ihex_binrec) -> *const ihex_binrec;
    fn msleep(msecs: c_uint);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut c_void;
    fn spi_write_then_read(
        spi: *mut spi_device,
        txbuf: *const c_void,
        n_tx: c_uint,
        rxbuf: *mut c_void,
        n_rx: c_uint,
    ) -> c_int;
    fn spi_write(spi: *mut spi_device, buf: *const c_void, len: size_t) -> c_int;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_int,
    ) -> *mut gpio_desc;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn devm_kzalloc(dev: *mut device, size: size_t, gfp: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_regmap_init(
        dev: *mut device,
        bus: *const regmap_bus,
        bus_context: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_kmemdup(
        dev: *mut device,
        src: *const c_void,
        len: size_t,
        gfp: c_uint,
    ) -> *mut c_void;
    fn devm_gpiochip_add_data(
        dev: *mut device,
        gc: *mut gpio_chip,
        data: *mut c_void,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn of_match_ptr(ptr: *const of_device_id) -> *const of_device_id;
}

unsafe fn regmap_read_poll_timeout_reg_sema_flags_boot_cmd(
    regmap: *mut regmap,
    reg: c_uint,
) -> c_int {
    let mut val: c_uint = 0;
    loop {
        let err = regmap_read(regmap, reg, &mut val);
        if err != 0 {
            return err;
        }
        if (val & SEMA_FLAGS_BOOT_CMD) == 0 {
            return 0;
        }
        usleep_range(10000, 10000);
        break;
    }
    loop {
        let err = regmap_read(regmap, reg, &mut val);
        if err != 0 {
            return err;
        }
        if (val & SEMA_FLAGS_BOOT_CMD) == 0 {
            return 0;
        }
        usleep_range(10000, 10000);
        break;
    }
    -EINVAL
}

unsafe fn regmap_read_poll_timeout_reg_cmd_zero(regmap: *mut regmap, reg: c_uint) -> c_int {
    let mut val: c_uint = 0;
    loop {
        let err = regmap_read(regmap, reg, &mut val);
        if err != 0 {
            return err;
        }
        if val == 0 {
            return 0;
        }
        usleep_range(10000, 10000);
        break;
    }
    loop {
        let err = regmap_read(regmap, reg, &mut val);
        if err != 0 {
            return err;
        }
        if val == 0 {
            return 0;
        }
        usleep_range(10000, 10000);
        break;
    }
    -EINVAL
}

unsafe fn regmap_read_poll_timeout_param_ready(regmap: *mut regmap, reg: c_uint) -> c_int {
    let mut val: c_uint = 0;
    loop {
        let err = regmap_read(regmap, reg, &mut val);
        if err != 0 {
            return err;
        }
        if val == PARAM_RESULT_READY {
            return 0;
        }
        usleep_range(1000, 1000);
        break;
    }
    -EINVAL
}

unsafe fn regmap_read_poll_timeout_app_reboot_clear(regmap: *mut regmap, reg: c_uint) -> c_int {
    let mut val: c_uint = 0;
    loop {
        let err = regmap_read(regmap, reg, &mut val);
        if err != 0 {
            return err;
        }
        if (val & SEMA_FLAGS_APP_REBOOT) == 0 {
            return 0;
        }
        usleep_range(10000, 10000);
        break;
    }
    loop {
        let err = regmap_read(regmap, reg, &mut val);
        if err != 0 {
            return err;
        }
        if (val & SEMA_FLAGS_APP_REBOOT) == 0 {
            return 0;
        }
        usleep_range(10000, 10000);
        break;
    }
    -EINVAL
}

unsafe extern "C" fn zl38_fw_issue_command(regmap: *mut regmap, cmd: u16) -> c_int {
    let mut err: c_int;

    err = regmap_read_poll_timeout_reg_sema_flags_boot_cmd(regmap, REG_SEMA_FLAGS);
    if err != 0 {
        return err;
    }
    err = regmap_write(regmap, REG_CMD, cmd as c_uint);
    if err != 0 {
        return err;
    }
    err = regmap_update_bits(
        regmap,
        REG_SEMA_FLAGS,
        SEMA_FLAGS_BOOT_CMD,
        SEMA_FLAGS_BOOT_CMD,
    );
    if err != 0 {
        return err;
    }

    regmap_read_poll_timeout_reg_cmd_zero(regmap, REG_CMD)
}

unsafe extern "C" fn zl38_fw_go(regmap: *mut regmap) -> c_int {
    let mut err: c_int;

    err = zl38_fw_issue_command(regmap, BOOTCMD_LOAD_COMPLETE);
    if err != 0 {
        return err;
    }

    zl38_fw_issue_command(regmap, BOOTCMD_FW_GO)
}

unsafe extern "C" fn zl38_fw_enter_boot_mode(regmap: *mut regmap) -> c_int {
    let err: c_int;

    err = regmap_update_bits(
        regmap,
        REG_CLK_STATUS,
        CLK_STATUS_HWRST,
        CLK_STATUS_HWRST,
    );
    if err != 0 {
        return err;
    }

    regmap_read_poll_timeout_param_ready(regmap, REG_PARAM_RESULT)
}

unsafe extern "C" fn zl38_fw_send_data(
    regmap: *mut regmap,
    addr: u32,
    data: *const c_void,
    len: u16,
) -> c_int {
    let addr_base: __be32 = (addr & !0xFF).to_be();
    let err: c_int;

    err = regmap_raw_write(
        regmap,
        REG_PG255_BASE_HI,
        &addr_base as *const __be32 as *const c_void,
        size_of::<__be32>(),
    );
    if err != 0 {
        return err;
    }
    regmap_raw_write(regmap, REG_PG255_OFFS(addr), data, len as size_t)
}

unsafe extern "C" fn zl38_fw_send_xaddr(regmap: *mut regmap, data: *const c_void) -> c_int {
    /* execution address from ihex: 32-bit little endian.
     * device register expects 32-bit big endian.
     */
    let addr: u32 = u32::from_le(ptr::read_unaligned(data as *const u32));
    let baddr: __be32 = addr.to_be();

    regmap_raw_write(
        regmap,
        REG_FWR_EXEC,
        &baddr as *const __be32 as *const c_void,
        size_of::<__be32>(),
    )
}

unsafe extern "C" fn zl38_load_firmware(dev: *mut device, regmap: *mut regmap) -> c_int {
    let mut rec: *const ihex_binrec;
    let mut fw: *const firmware = ptr::null();
    let mut addr: u32;
    let mut len: u16;
    let mut err: c_int;

    /* how to get this firmware:
     * 1. request and download chip firmware from Microsemi
     *    (provided by Microsemi in srec format)
     * 2. convert downloaded firmware from srec to ihex. Simple tool:
     *    https://gitlab.com/TheSven73/s3-to-irec
     * 3. convert ihex to binary (.fw) using ihex2fw tool which is included
     *    with the Linux kernel sources
     */
    err = request_ihex_firmware(&mut fw, c"zl38060.fw".as_ptr(), dev);
    if err != 0 {
        return err;
    }
    err = zl38_fw_enter_boot_mode(regmap);
    if err != 0 {
        return err;
    }
    rec = (*fw).data as *const ihex_binrec;
    while !rec.is_null() {
        addr = u32::from_be((*rec).addr);
        len = u16::from_be((*rec).len);
        if addr != 0 {
            /* regular data ihex record */
            err = zl38_fw_send_data(regmap, addr, (*rec).data.as_ptr() as *const c_void, len);
        } else if len == 4 {
            /* execution address ihex record */
            err = zl38_fw_send_xaddr(regmap, (*rec).data.as_ptr() as *const c_void);
        } else {
            err = -EINVAL;
        }
        if err != 0 {
            return err;
        }
        /* next ! */
        rec = ihex_next_binrec(rec);
    }

    zl38_fw_go(regmap)
}

unsafe extern "C" fn zl38_software_reset(regmap: *mut regmap) -> c_int {
    let err: c_int;

    err = regmap_update_bits(
        regmap,
        REG_SEMA_FLAGS,
        SEMA_FLAGS_APP_REBOOT,
        SEMA_FLAGS_APP_REBOOT,
    );
    if err != 0 {
        return err;
    }

    /* wait for host bus interface to settle.
     * Not sure if this is required: Microsemi's vendor driver does this,
     * but the firmware manual does not mention it. Leave it in, there's
     * little downside, apart from a slower reset.
     */
    msleep(50);

    regmap_read_poll_timeout_app_reboot_clear(regmap, REG_SEMA_FLAGS)
}

unsafe extern "C" fn zl38_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let priv_: *mut zl38_codec_priv = snd_soc_dai_get_drvdata(dai) as *mut zl38_codec_priv;
    let err: c_int;

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            /* firmware default is normal i2s */
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {
            /* firmware default is normal bitclock and frame */
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            /* always 32 bits per frame (= 16 bits/channel, 2 channels) */
            err = regmap_update_bits(
                (*priv_).regmap,
                REG_TDMA_CFG_CLK,
                CFG_CLK_MASTER | CFG_CLK_PCLK_MASK,
                CFG_CLK_MASTER | CFG_CLK_PCLK(32),
            );
            if err != 0 {
                return err;
            }
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn zl38_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_: *mut zl38_codec_priv = snd_soc_dai_get_drvdata(dai) as *mut zl38_codec_priv;
    let tx: bool_ = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let fsrate: c_uint;
    let mut err: c_int;

    /* We cannot change hw_params while the dai is already in use - the
     * software reset will corrupt the audio. However, this is not required,
     * as the chip's TDM buses are fully symmetric, which mandates identical
     * rates, channels, and samplebits for record and playback.
     */
    if (*priv_).is_stream_in_use[(!tx) as usize] {
        (*priv_).is_stream_in_use[tx as usize] = true;
        return 0;
    }

    match params_rate(params) {
        8000 => fsrate = CFG_CLK_FSRATE_8KHZ,
        16000 => fsrate = CFG_CLK_FSRATE_16KHZ,
        48000 => fsrate = CFG_CLK_FSRATE_48KHZ,
        _ => return -EINVAL,
    }

    err = regmap_update_bits(
        (*priv_).regmap,
        REG_TDMA_CFG_CLK,
        CFG_CLK_FSRATE_MASK,
        fsrate,
    );
    if err != 0 {
        return err;
    }

    /* chip requires a software reset to apply audio register changes */
    err = zl38_software_reset((*priv_).regmap);
    if err != 0 {
        return err;
    }

    (*priv_).is_stream_in_use[tx as usize] = true;

    0
}

unsafe extern "C" fn zl38_hw_free(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_: *mut zl38_codec_priv = snd_soc_dai_get_drvdata(dai) as *mut zl38_codec_priv;
    let tx: bool_ = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;

    (*priv_).is_stream_in_use[tx as usize] = false;

    0
}

/* stereo bypass with no AEC */
static cp_config_stereo_bypass: [reg_sequence; 7] = [
    /* interconnects must be programmed first */
    reg_sequence { reg: 0x0210, def: 0x0005 }, /* DAC1   in <= I2S1-L */
    reg_sequence { reg: 0x0212, def: 0x0006 }, /* DAC2   in <= I2S1-R */
    reg_sequence { reg: 0x0214, def: 0x0001 }, /* I2S1-L in <= MIC1   */
    reg_sequence { reg: 0x0216, def: 0x0001 }, /* I2S1-R in <= MIC1   */
    reg_sequence { reg: 0x0224, def: 0x0000 }, /* AEC-S  in <= n/a    */
    reg_sequence { reg: 0x0226, def: 0x0000 }, /* AEC-R  in <= n/a    */
    /* output enables must be programmed next */
    reg_sequence { reg: 0x0202, def: 0x000F }, /* enable I2S1 + DAC   */
];

static zl38_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(zl38_set_fmt),
    hw_params: Some(zl38_hw_params),
    hw_free: Some(zl38_hw_free),
};

static mut zl38_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"zl38060-tdma".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: ZL38_RATES,
        formats: ZL38_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: ZL38_RATES,
        formats: ZL38_FORMATS,
    },
    ops: &zl38_dai_ops,
    symmetric_rate: 1,
    symmetric_sample_bits: 1,
    symmetric_channels: 1,
};

const fn SND_SOC_DAPM_OUTPUT(name: *const c_char) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget { name, id: 0 }
}

const fn SND_SOC_DAPM_INPUT(name: *const c_char) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget { name, id: 1 }
}

static zl38_dapm_widgets: [snd_soc_dapm_widget; 3] = [
    SND_SOC_DAPM_OUTPUT(c"DAC1".as_ptr()),
    SND_SOC_DAPM_OUTPUT(c"DAC2".as_ptr()),
    SND_SOC_DAPM_INPUT(c"DMICL".as_ptr()),
];

static zl38_dapm_routes: [snd_soc_dapm_route; 3] = [
    snd_soc_dapm_route {
        sink: c"DAC1".as_ptr(),
        control: ptr::null(),
        source: c"Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"DAC2".as_ptr(),
        control: ptr::null(),
        source: c"Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Capture".as_ptr(),
        control: ptr::null(),
        source: c"DMICL".as_ptr(),
    },
];

static zl38_component_dev: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: zl38_dapm_widgets.as_ptr(),
    num_dapm_widgets: zl38_dapm_widgets.len() as c_uint,
    dapm_routes: zl38_dapm_routes.as_ptr(),
    num_dapm_routes: zl38_dapm_routes.len() as c_uint,
    endianness: 1,
};

unsafe extern "C" fn chip_gpio_set(c: *mut gpio_chip, offset: c_uint, val: c_int) -> c_int {
    let regmap: *mut regmap = gpiochip_get_data(c) as *mut regmap;
    let mask: c_uint = BIT(offset);

    regmap_update_bits(regmap, REG_GPIO_DAT, mask, if val != 0 { mask } else { 0 })
}

unsafe extern "C" fn chip_gpio_get(c: *mut gpio_chip, offset: c_uint) -> c_int {
    let regmap: *mut regmap = gpiochip_get_data(c) as *mut regmap;
    let mask: c_uint = BIT(offset);
    let mut val: c_uint = 0;
    let err: c_int;

    err = regmap_read(regmap, REG_GPIO_DAT, &mut val);
    if err != 0 {
        return err;
    }

    ((val & mask) != 0) as c_int
}

unsafe extern "C" fn chip_direction_input(c: *mut gpio_chip, offset: c_uint) -> c_int {
    let regmap: *mut regmap = gpiochip_get_data(c) as *mut regmap;
    let mask: c_uint = BIT(offset);

    regmap_update_bits(regmap, REG_GPIO_DIR, mask, 0)
}

unsafe extern "C" fn chip_direction_output(
    c: *mut gpio_chip,
    offset: c_uint,
    val: c_int,
) -> c_int {
    let regmap: *mut regmap = gpiochip_get_data(c) as *mut regmap;
    let mask: c_uint = BIT(offset);
    let ret: c_int;

    ret = chip_gpio_set(c, offset, val);
    if ret != 0 {
        return ret;
    }

    regmap_update_bits(regmap, REG_GPIO_DIR, mask, mask)
}

static template_chip: gpio_chip = gpio_chip {
    owner: THIS_MODULE,
    label: DRV_NAME.as_ptr() as *const c_char,

    base: -1,
    ngpio: 14,
    direction_input: Some(chip_direction_input),
    direction_output: Some(chip_direction_output),
    get: Some(chip_gpio_get),
    set: Some(chip_gpio_set),

    can_sleep: true,
    parent: ptr::null_mut(),
};

unsafe extern "C" fn zl38_check_revision(dev: *mut device, regmap: *mut regmap) -> c_int {
    let mut hwrev: c_uint = 0;
    let mut fwprod: c_uint = 0;
    let mut fwrev: c_uint = 0;
    let fw_major: c_int;
    let fw_minor: c_int;
    let fw_micro: c_int;
    let mut err: c_int;

    err = regmap_read(regmap, REG_HW_REV, &mut hwrev);
    if err != 0 {
        return err;
    }
    err = regmap_read(regmap, REG_FW_PROD, &mut fwprod);
    if err != 0 {
        return err;
    }
    err = regmap_read(regmap, REG_FW_REV, &mut fwrev);
    if err != 0 {
        return err;
    }

    fw_major = ((fwrev >> 12) & 0xF) as c_int;
    fw_minor = ((fwrev >> 8) & 0xF) as c_int;
    fw_micro = (fwrev & 0xFF) as c_int;
    dev_info(
        dev,
        c"hw rev 0x%x, fw product code %d, firmware rev %d.%d.%d".as_ptr(),
        hwrev & 0x1F,
        fwprod,
        fw_major,
        fw_minor,
        fw_micro,
    );

    if fw_major != FIRMWARE_MAJOR || fw_minor < FIRMWARE_MINOR {
        dev_err(
            dev,
            c"unsupported firmware. driver supports %d.%d".as_ptr(),
            FIRMWARE_MAJOR,
            FIRMWARE_MINOR,
        );
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn zl38_bus_read(
    context: *mut c_void,
    reg_buf: *const c_void,
    reg_size: size_t,
    val_buf: *mut c_void,
    val_size: size_t,
) -> c_int {
    let spi: *mut spi_device = context as *mut spi_device;
    let reg_buf8: *const u8 = reg_buf as *const u8;
    let mut len: size_t = 0;
    let offs: u8;
    let page: u8;
    let mut txbuf: [u8; 4] = [0; 4];

    if reg_size != 2 || val_size > ZL38_MAX_RAW_XFER {
        return -EINVAL;
    }

    offs = *reg_buf8.add(1) >> 1;
    page = *reg_buf8.add(0);

    if page != 0 {
        txbuf[len] = 0xFE;
        len += 1;
        txbuf[len] = if page as c_uint == HBI_FIRMWARE_PAGE {
            0xFF
        } else {
            page - 1
        };
        len += 1;
        txbuf[len] = offs;
        len += 1;
        txbuf[len] = (val_size / 2 - 1) as u8;
        len += 1;
    } else {
        txbuf[len] = offs | 0x80;
        len += 1;
        txbuf[len] = (val_size / 2 - 1) as u8;
        len += 1;
    }

    spi_write_then_read(spi, txbuf.as_ptr() as *const c_void, len as c_uint, val_buf, val_size as c_uint)
}

unsafe extern "C" fn zl38_bus_write(
    context: *mut c_void,
    data: *const c_void,
    count: size_t,
) -> c_int {
    let spi: *mut spi_device = context as *mut spi_device;
    let mut buf: [u8; 4 + ZL38_MAX_RAW_XFER] = [0; 4 + ZL38_MAX_RAW_XFER];
    let val_len: size_t;
    let mut len: size_t = 0;
    let data8: *const u8 = data as *const u8;
    let offs: u8;
    let page: u8;

    if count > (2 + ZL38_MAX_RAW_XFER) || count < 4 {
        return -EINVAL;
    }
    val_len = count - 2;
    offs = *data8.add(1) >> 1;
    page = *data8.add(0);

    if page != 0 {
        buf[len] = 0xFE;
        len += 1;
        buf[len] = if page as c_uint == HBI_FIRMWARE_PAGE {
            0xFF
        } else {
            page - 1
        };
        len += 1;
        buf[len] = offs;
        len += 1;
        buf[len] = ((val_len / 2 - 1) | 0x80) as u8;
        len += 1;
    } else {
        buf[len] = offs | 0x80;
        len += 1;
        buf[len] = ((val_len / 2 - 1) | 0x80) as u8;
        len += 1;
    }
    ptr::copy_nonoverlapping(data8.add(2), buf.as_mut_ptr().add(len), val_len);
    len += val_len;

    spi_write(spi, buf.as_ptr() as *const c_void, len)
}

static zl38_regmap_bus: regmap_bus = regmap_bus {
    read: Some(zl38_bus_read),
    write: Some(zl38_bus_write),
    max_raw_write: ZL38_MAX_RAW_XFER,
    max_raw_read: ZL38_MAX_RAW_XFER,
};

static zl38_regmap_conf: regmap_config = regmap_config {
    reg_bits: 16,
    val_bits: 16,
    reg_stride: 2,
    use_single_read: true,
    use_single_write: true,
};

unsafe extern "C" fn zl38_spi_probe(spi: *mut spi_device) -> c_int {
    let dev: *mut device = &mut (*spi).dev;
    let mut priv_: *mut zl38_codec_priv;
    let reset_gpio: *mut gpio_desc;
    let mut err: c_int;

    /* get the chip to a known state by putting it in reset */
    reset_gpio = devm_gpiod_get_optional(dev, c"reset".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR(reset_gpio as *const c_void) {
        return PTR_ERR(reset_gpio as *const c_void);
    }
    if !reset_gpio.is_null() {
        /* datasheet: need > 10us for a digital + analog reset */
        usleep_range(15, 50);
        /* take the chip out of reset */
        gpiod_set_value_cansleep(reset_gpio, 0);
        /* datasheet: need > 3ms for digital section to become stable */
        usleep_range(3000, 10000);
    }

    priv_ = devm_kzalloc(dev, size_of::<zl38_codec_priv>(), GFP_KERNEL) as *mut zl38_codec_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*priv_).dev = dev;
    dev_set_drvdata(dev, priv_ as *mut c_void);
    (*priv_).regmap = devm_regmap_init(
        dev,
        &zl38_regmap_bus,
        spi as *mut c_void,
        &zl38_regmap_conf,
    );
    if IS_ERR((*priv_).regmap as *const c_void) {
        return PTR_ERR((*priv_).regmap as *const c_void);
    }

    err = zl38_load_firmware(dev, (*priv_).regmap);
    if err != 0 {
        return err;
    }

    err = zl38_check_revision(dev, (*priv_).regmap);
    if err != 0 {
        return err;
    }

    (*priv_).gpio_chip = devm_kmemdup(
        dev,
        &template_chip as *const gpio_chip as *const c_void,
        size_of::<gpio_chip>(),
        GFP_KERNEL,
    ) as *mut gpio_chip;
    if (*priv_).gpio_chip.is_null() {
        return -ENOMEM;
    }
    (*(*priv_).gpio_chip).parent = dev;
    err = devm_gpiochip_add_data(dev, (*priv_).gpio_chip, (*priv_).regmap as *mut c_void);
    if err != 0 {
        return err;
    }

    /* setup the cross-point switch for stereo bypass */
    err = regmap_multi_reg_write(
        (*priv_).regmap,
        cp_config_stereo_bypass.as_ptr(),
        cp_config_stereo_bypass.len() as c_int,
    );
    if err != 0 {
        return err;
    }
    /* setup for 12MHz crystal connected to the chip */
    err = regmap_update_bits(
        (*priv_).regmap,
        REG_CLK_CFG,
        CLK_CFG_SOURCE_XTAL,
        CLK_CFG_SOURCE_XTAL,
    );
    if err != 0 {
        return err;
    }

    devm_snd_soc_register_component(dev, &zl38_component_dev, &mut zl38_dai, 1)
}

static zl38_dt_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: c"mscc,zl38060".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, zl38_dt_ids); */

static zl38_spi_ids: [spi_device_id; 2] = [
    spi_device_id {
        name: c"zl38060".as_ptr(),
        driver_data: 0,
    },
    spi_device_id {
        name: ptr::null(),
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(spi, zl38_spi_ids); */

static mut zl38060_spi_driver: spi_driver = spi_driver {
    driver: driver_private {
        name: DRV_NAME.as_ptr() as *const c_char,
        of_match_table: zl38_dt_ids.as_ptr(),
    },
    probe: Some(zl38_spi_probe),
    id_table: zl38_spi_ids.as_ptr(),
};
/* module_spi_driver(zl38060_spi_driver); */

/* MODULE_DESCRIPTION("ASoC ZL38060 driver"); */
/* MODULE_AUTHOR("Sven Van Asbroeck <TheSven73@gmail.com>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
