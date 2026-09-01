// SPDX-License-Identifier: GPL-2.0
//
// TAS2781 HDA SPI driver
//
// Copyright 2024 - 2026 Texas Instruments, Inc.
//
// Author: Baojun Xu <baojun.xu@ti.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type size_t = usize;
type u32 = u32;

const TASDEVICE_RANGE_MAX_SIZE: c_uint = 256 * 128;
const TASDEVICE_WIN_LEN: usize = 128;
const TAS2781_SPI_MAX_FREQ: c_uint = 4 * HZ_PER_MHZ;

/* System Reset Check Register */
const TAS2781_REG_CLK_CONFIG: c_uint = TASDEVICE_REG(0x0, 0x0, 0x5c);
const TAS2781_REG_CLK_CONFIG_RESET: c_uint = 0x19;

#[repr(C)]
struct tas2781_hda_spi_priv {
    snd_ctls: [*mut snd_kcontrol; 3],
}

#[repr(C)]
struct regmap_range_cfg {
    range_min: c_uint,
    range_max: c_uint,
    selector_reg: c_uint,
    selector_mask: c_uint,
    selector_shift: c_uint,
    window_start: c_uint,
    window_len: c_uint,
}

#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    zero_flag_mask: bool_,
    read_flag_mask: c_uint,
    reg_shift: c_int,
    cache_type: c_uint,
    ranges: *const regmap_range_cfg,
    num_ranges: c_uint,
    max_register: c_uint,
}

#[repr(C)]
struct snd_kcontrol_new {
    iface: c_uint,
    name: *const c_char,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    private_value: usize,
    tlv: *const c_uint,
}

#[repr(C)]
struct component_ops {
    bind: Option<unsafe extern "C" fn(*mut device, *mut device, *mut c_void) -> c_int>,
    unbind: Option<unsafe extern "C" fn(*mut device, *mut device, *mut c_void)>,
}

#[repr(C)]
struct dev_pm_ops {
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
struct spi_device_id {
    name: [c_char; 32],
    driver_data: usize,
}

#[repr(C)]
struct acpi_device_id {
    id: [c_char; 16],
    driver_data: usize,
}

#[repr(C)]
struct driver_inner {
    name: *const c_char,
    acpi_match_table: *const acpi_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct spi_driver {
    driver: driver_inner,
    id_table: *const spi_device_id,
    probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut spi_device)>,
}

#[repr(C)]
struct integer_value {
    value: [c_int; 128],
}

#[repr(C)]
union elem_value_union {
    integer: integer_value,
}

#[repr(C)]
struct snd_ctl_elem_value {
    value: elem_value_union,
}

#[repr(C)]
struct soc_mixer_control {
    reg: c_uint,
    rreg: c_uint,
    shift: c_uint,
    rshift: c_uint,
    max: c_int,
    platform_max: c_int,
    invert: c_uint,
}

#[repr(C)]
struct tasdevice {
    cur_book: c_int,
    cur_conf: c_int,
    cur_prog: c_int,
}

#[repr(C)]
struct firmware {
    _private: [u8; 0],
}

#[repr(C)]
struct tasdevice_fmw {
    nr_programs: c_int,
    nr_configurations: c_int,
}

#[repr(C)]
struct tasdevice_rcabin {
    profile_cfg_id: c_int,
}

#[repr(C)]
struct tasdevice_priv {
    dev: *mut device,
    codec: *mut hda_codec,
    regmap: *mut regmap,
    reset: *mut gpio_desc,
    tasdevice: *mut tasdevice,
    index: c_uint,
    ndev: size_t,
    isspi: bool_,
    dev_name: [c_char; 64],
    rca_binaryname: [c_char; 64],
    coef_binaryname: [c_char; 64],
    crc8_lkp_tbl: [u8; 256],
    codec_lock: mutex,
    update_bits: Option<unsafe extern "C" fn(*mut tasdevice_priv, u16, c_uint, c_uint, c_uint) -> c_int>,
    change_chn_book: Option<unsafe extern "C" fn(*mut tasdevice_priv, u16, c_int) -> c_int>,
    dev_read: Option<unsafe extern "C" fn(*mut tasdevice_priv, u16, c_uint, *mut c_uint) -> c_int>,
    dev_bulk_read: Option<unsafe extern "C" fn(*mut tasdevice_priv, u16, c_uint, *mut u8, c_uint) -> c_int>,
    force_fwload_status: bool_,
    fw_state: c_int,
    playback_started: bool_,
    irq: c_int,
    chip_id: c_int,
    rcabin: tasdevice_rcabin,
    fmw: *mut tasdevice_fmw,
}

#[repr(C)]
struct tas2781_hda {
    priv_: *mut tasdevice_priv,
    hda_priv: *mut c_void,
    dsp_prog_ctl: *mut snd_kcontrol,
    dsp_conf_ctl: *mut snd_kcontrol,
    prof_ctl: *mut snd_kcontrol,
    catlog_id: c_int,
}

#[repr(C)]
struct hda_codec_core {
    subsystem_id: c_uint,
}

#[repr(C)]
struct hda_codec {
    card: *mut snd_card,
    core: hda_codec_core,
}

#[repr(C)]
struct hda_component {
    dev: *mut device,
    name: [c_char; 64],
    playback_hook: Option<unsafe extern "C" fn(*mut device, c_int)>,
}

#[repr(C)]
struct hda_component_parent {
    codec: *mut hda_codec,
}

#[repr(C)]
struct spi_device {
    dev: device,
    max_speed_hz: c_uint,
    irq: c_int,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}
#[repr(C)]
struct acpi_device {
    _private: [u8; 0],
}
#[repr(C)]
struct module {
    _private: [u8; 0],
}
#[repr(C)]
struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_kcontrol {
    private_value: usize,
}
#[repr(C)]
struct snd_ctl_elem_info {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_card {
    _private: [u8; 0],
}
#[repr(C)]
struct gpio_desc {
    _private: [u8; 0],
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

const fn TASDEVICE_REG(book: c_uint, page: c_uint, reg: c_uint) -> c_uint {
    (book << 16) | (page << 8) | reg
}

const fn TASDEVICE_BOOK_ID(reg: c_uint) -> c_uint {
    (reg >> 16) & 0xff
}

const fn TASDEVICE_PAGE_ID(reg: c_uint) -> c_uint {
    (reg >> 8) & 0xff
}

const fn TASDEVICE_PAGE_REG(reg: c_uint) -> c_uint {
    reg & 0xff
}

const fn lower_16_bits(value: c_uint) -> c_uint {
    value & 0xffff
}

fn clamp(val: c_int, min: c_int, max: c_int) -> c_int {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

fn rounddown_pow_of_two(mut x: c_int) -> u8 {
    if x <= 0 {
        return 0;
    }
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    ((x + 1) >> 1) as u8
}

static tasdevice_ranges: [regmap_range_cfg; 1] = [regmap_range_cfg {
    range_min: 0,
    range_max: TASDEVICE_RANGE_MAX_SIZE,
    selector_reg: TASDEVICE_PAGE_SELECT,
    selector_mask: GENMASK(7, 0),
    selector_shift: 0,
    window_start: 0,
    window_len: TASDEVICE_WIN_LEN as c_uint,
}];

static tasdevice_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    zero_flag_mask: true,
    read_flag_mask: 0x01,
    reg_shift: -1,
    cache_type: REGCACHE_NONE,
    ranges: tasdevice_ranges.as_ptr(),
    num_ranges: ARRAY_SIZE(&tasdevice_ranges),
    max_register: TASDEVICE_RANGE_MAX_SIZE,
};

unsafe extern "C" fn tasdevice_spi_dev_read(
    tas_priv: *mut tasdevice_priv,
    chn: u16,
    reg: c_uint,
    val: *mut c_uint,
) -> c_int {
    let ret: c_int;

    /*
     * In our TAS2781 SPI mode, if read from other book (not book 0),
     * or read from page number larger than 1 in book 0, one more byte
     * read is needed, and first byte is a dummy byte, need to be ignored.
     */
    if TASDEVICE_BOOK_ID(reg) > 0 || TASDEVICE_PAGE_ID(reg) > 1 {
        let mut data = [0u8; 2];

        ret = tasdevice_dev_bulk_read(tas_priv, chn, reg, data.as_mut_ptr(), data.len() as c_uint);
        *val = data[1] as c_uint;
    } else {
        ret = tasdevice_dev_read(tas_priv, chn, reg, val);
    }
    if ret < 0 {
        dev_err((*tas_priv).dev, c"%s, E=%d\n".as_ptr(), c"tasdevice_spi_dev_read".as_ptr(), ret);
    }

    ret
}

unsafe extern "C" fn tasdevice_spi_dev_bulk_read(
    tas_priv: *mut tasdevice_priv,
    chn: u16,
    reg: c_uint,
    data: *mut u8,
    len: c_uint,
) -> c_int {
    let ret: c_int;

    /*
     * In our TAS2781 SPI mode, if read from other book (not book 0),
     * or read from page number larger than 1 in book 0, one more byte
     * read is needed, and first byte is a dummy byte, need to be ignored.
     */
    if TASDEVICE_BOOK_ID(reg) > 0 || TASDEVICE_PAGE_ID(reg) > 1 {
        let mut buf = [0u8; TASDEVICE_WIN_LEN + 1];

        ret = tasdevice_dev_bulk_read(tas_priv, chn, reg, buf.as_mut_ptr(), len + 1);
        memcpy(data as *mut c_void, buf.as_ptr().add(1) as *const c_void, len as size_t);
    } else {
        ret = tasdevice_dev_bulk_read(tas_priv, chn, reg, data, len);
    }
    if ret < 0 {
        dev_err((*tas_priv).dev, c"%s, E=%d\n".as_ptr(), c"tasdevice_spi_dev_bulk_read".as_ptr(), ret);
    }

    ret
}

unsafe extern "C" fn tasdevice_spi_dev_update_bits(
    tas_priv: *mut tasdevice_priv,
    chn: u16,
    reg: c_uint,
    mask: c_uint,
    value: c_uint,
) -> c_int {
    let ret: c_int;
    let mut val: c_int;

    /*
     * In TAS2781 SPI mode, when accessing non-book-zero or page numbers
     * greater than 1 in book 0, an additional byte must be read. The
     * first byte in such cases is a dummy byte and should be ignored.
     */
    if TASDEVICE_BOOK_ID(reg) > 0 || TASDEVICE_PAGE_ID(reg) > 1 {
        let mut buf = [0u8; 2];

        ret = tasdevice_dev_bulk_read(tas_priv, chn, reg, buf.as_mut_ptr(), 2);
        val = buf[1] as c_int;
    } else {
        ret = tasdevice_dev_read(tas_priv, chn, reg, &mut val as *mut c_int as *mut c_uint);
    }
    if ret < 0 {
        dev_err((*tas_priv).dev, c"%s, E=%d\n".as_ptr(), c"tasdevice_spi_dev_update_bits".as_ptr(), ret);
        return ret;
    }

    let ret = tasdevice_dev_write(
        tas_priv,
        chn,
        TASDEVICE_PAGE_REG(reg),
        ((val as c_uint) & !mask) | (mask & value),
    );
    if ret < 0 {
        dev_err((*tas_priv).dev, c"%s, E=%d\n".as_ptr(), c"tasdevice_spi_dev_update_bits".as_ptr(), ret);
    }

    ret
}

unsafe extern "C" fn tasdevice_spi_change_chn_book(
    p: *mut tasdevice_priv,
    chn: u16,
    book: c_int,
) -> c_int {
    let mut ret: c_int = 0;

    if chn as c_uint == (*p).index {
        let tasdev = (*p).tasdevice.add(chn as usize);
        let map = (*p).regmap;

        if (*tasdev).cur_book != book {
            ret = regmap_write(map, TASDEVICE_BOOKCTL_REG, book as c_uint);
            if ret < 0 {
                dev_err((*p).dev, c"%s, E=%d\n".as_ptr(), c"tasdevice_spi_change_chn_book".as_ptr(), ret);
            } else {
                (*tasdev).cur_book = book;
            }
        }
    } else {
        ret = -EXDEV;
        dev_dbg(
            (*p).dev,
            c"Not error, %s ignore channel(%d)\n".as_ptr(),
            c"tasdevice_spi_change_chn_book".as_ptr(),
            chn as c_int,
        );
    }

    ret
}

unsafe extern "C" fn tas2781_spi_reset(tas_dev: *mut tasdevice_priv) {
    let ret: c_int;

    if !(*tas_dev).reset.is_null() {
        gpiod_set_value_cansleep((*tas_dev).reset, 0);
        fsleep(800);
        gpiod_set_value_cansleep((*tas_dev).reset, 1);
    }

    ret = tasdevice_dev_write(
        tas_dev,
        (*tas_dev).index as u16,
        TASDEVICE_REG_SWRESET,
        TASDEVICE_REG_SWRESET_RESET,
    );
    if ret < 0 {
        dev_err((*tas_dev).dev, c"dev sw-reset fail, %d\n".as_ptr(), ret);
        return;
    }
    fsleep(1000);
}

unsafe extern "C" fn tascodec_spi_init(
    tas_priv: *mut tasdevice_priv,
    codec: *mut c_void,
    module: *mut module,
    cont: Option<unsafe extern "C" fn(*const firmware, *mut c_void)>,
) -> c_int {
    let ret: c_int;

    /*
     * Codec Lock Hold to ensure that codec_probe and firmware parsing and
     * loading do not simultaneously execute.
     */
    mutex_lock(&mut (*tas_priv).codec_lock);

    scnprintf(
        (*tas_priv).rca_binaryname.as_mut_ptr(),
        size_of::<[c_char; 64]>(),
        c"%sRCA%d.bin".as_ptr(),
        (*tas_priv).dev_name.as_ptr(),
        (*tas_priv).ndev as c_int,
    );
    crc8_populate_msb((*tas_priv).crc8_lkp_tbl.as_mut_ptr(), TASDEVICE_CRC8_POLYNOMIAL);
    (*tas_priv).codec = codec as *mut hda_codec;
    ret = request_firmware_nowait(
        module,
        FW_ACTION_UEVENT,
        (*tas_priv).rca_binaryname.as_ptr(),
        (*tas_priv).dev,
        GFP_KERNEL,
        tas_priv as *mut c_void,
        cont,
    );
    if ret != 0 {
        dev_err((*tas_priv).dev, c"request_firmware_nowait err:0x%08x\n".as_ptr(), ret);
    }

    mutex_unlock(&mut (*tas_priv).codec_lock);
    ret
}

unsafe extern "C" fn tasdevice_spi_init(tas_priv: *mut tasdevice_priv) {
    (*(*tas_priv).tasdevice.add((*tas_priv).index as usize)).cur_book = -1;
    (*(*tas_priv).tasdevice.add((*tas_priv).index as usize)).cur_conf = -1;
    (*(*tas_priv).tasdevice.add((*tas_priv).index as usize)).cur_prog = -1;

    (*tas_priv).isspi = true;

    (*tas_priv).update_bits = Some(tasdevice_spi_dev_update_bits);
    (*tas_priv).change_chn_book = Some(tasdevice_spi_change_chn_book);
    (*tas_priv).dev_read = Some(tasdevice_spi_dev_read);
    (*tas_priv).dev_bulk_read = Some(tasdevice_spi_dev_bulk_read);

    mutex_init(&mut (*tas_priv).codec_lock);
}

unsafe extern "C" fn tasdevice_spi_amp_putvol(
    tas_priv: *mut tasdevice_priv,
    ucontrol: *mut snd_ctl_elem_value,
    mc: *mut soc_mixer_control,
) -> c_int {
    let invert = (*mc).invert;
    let mut mask: u8;
    let max = (*mc).max;
    let val: c_int;

    mask = rounddown_pow_of_two(max);
    mask <<= (*mc).shift;
    val = clamp(
        if invert != 0 {
            max - (*ucontrol).value.integer.value[0]
        } else {
            (*ucontrol).value.integer.value[0]
        },
        0,
        max,
    );

    let ret = tasdevice_spi_dev_update_bits(
        tas_priv,
        (*tas_priv).index as u16,
        (*mc).reg,
        mask as c_uint,
        (val << (*mc).shift) as c_uint,
    );
    if ret != 0 {
        dev_err((*tas_priv).dev, c"set AMP vol error in dev %d\n".as_ptr(), (*tas_priv).index as c_int);
    }

    ret
}

unsafe extern "C" fn tasdevice_spi_amp_getvol(
    tas_priv: *mut tasdevice_priv,
    ucontrol: *mut snd_ctl_elem_value,
    mc: *mut soc_mixer_control,
) -> c_int {
    let invert = (*mc).invert;
    let mut mask: u8 = 0;
    let max = (*mc).max;
    let mut val: c_int = 0;

    let ret = tasdevice_spi_dev_read(
        tas_priv,
        (*tas_priv).index as u16,
        (*mc).reg,
        &mut val as *mut c_int as *mut c_uint,
    );
    if ret != 0 {
        dev_err((*tas_priv).dev, c"%s, get AMP vol error\n".as_ptr(), c"tasdevice_spi_amp_getvol".as_ptr());
        return ret;
    }

    mask = rounddown_pow_of_two(max);
    mask <<= (*mc).shift;
    val = ((val & mask as c_int) >> (*mc).shift) as c_int;
    val = clamp(if invert != 0 { max - val } else { val }, 0, max);
    (*ucontrol).value.integer.value[0] = val;

    ret
}

unsafe extern "C" fn tasdevice_spi_digital_putvol(
    p: *mut tasdevice_priv,
    ucontrol: *mut snd_ctl_elem_value,
    mc: *mut soc_mixer_control,
) -> c_int {
    let invert = (*mc).invert;
    let max = (*mc).max;

    let val = clamp(
        if invert != 0 {
            max - (*ucontrol).value.integer.value[0]
        } else {
            (*ucontrol).value.integer.value[0]
        },
        0,
        max,
    );
    let ret = tasdevice_dev_write(p, (*p).index as u16, (*mc).reg, val as c_uint);
    if ret != 0 {
        dev_err((*p).dev, c"set digital vol err in dev %d\n".as_ptr(), (*p).index as c_int);
    }

    ret
}

unsafe extern "C" fn tasdevice_spi_digital_getvol(
    p: *mut tasdevice_priv,
    ucontrol: *mut snd_ctl_elem_value,
    mc: *mut soc_mixer_control,
) -> c_int {
    let invert = (*mc).invert;
    let max = (*mc).max;
    let mut val: c_int = 0;

    let ret = tasdevice_spi_dev_read(p, (*p).index as u16, (*mc).reg, &mut val as *mut c_int as *mut c_uint);
    if ret != 0 {
        dev_err((*p).dev, c"%s, get digital vol err\n".as_ptr(), c"tasdevice_spi_digital_getvol".as_ptr());
        return ret;
    }

    val = clamp(if invert != 0 { max - val } else { val }, 0, max);
    (*ucontrol).value.integer.value[0] = val;

    ret
}

unsafe extern "C" fn tas2781_read_acpi(tas_hda: *mut tas2781_hda, hid: *const c_char, id: c_int) -> c_int {
    let p = (*tas_hda).priv_;
    let mut values = [0u32; HDA_MAX_COMPONENTS as usize];
    let property: *const c_char;
    let nval: size_t;
    let mut ret: c_int;

    let adev = acpi_dev_get_first_match_dev(hid, ptr::null(), -1);
    if adev.is_null() {
        dev_err((*p).dev, c"Failed to find ACPI device: %s\n".as_ptr(), hid);
        return -ENODEV;
    }

    strscpy((*p).dev_name.as_mut_ptr(), hid, size_of::<[c_char; 64]>());

    let physdev = get_device(acpi_get_first_physical_node(adev));
    acpi_dev_put(adev);
    if physdev.is_null() {
        return -ENODEV;
    }

    property = c"ti,dev-index".as_ptr();
    ret = device_property_count_u32(physdev, property);
    if ret <= 0 || ret as c_uint > ARRAY_SIZE(&values) {
        ret = -EINVAL;
        goto_err(p, ret);
        put_device(physdev);
        return ret;
    }
    nval = ret as size_t;
    (*p).ndev = nval;

    ret = device_property_read_u32_array(physdev, property, values.as_mut_ptr(), nval);
    if ret != 0 {
        goto_err(p, ret);
        put_device(physdev);
        return ret;
    }

    (*p).index = U8_MAX as c_uint;
    for i in 0..nval {
        if values[i] == id as u32 {
            (*p).index = i as c_uint;
            break;
        }
    }
    if (*p).index == U8_MAX as c_uint {
        dev_dbg((*p).dev, c"No index found in %s\n".as_ptr(), property);
        ret = -ENODEV;
        goto_err(p, ret);
        put_device(physdev);
        return ret;
    }

    if (*p).index == 0 {
        /* All of amps share same RESET pin. */
        (*p).reset = devm_gpiod_get_index_optional(physdev, c"reset".as_ptr(), (*p).index as c_uint, GPIOD_OUT_LOW);
        if IS_ERR((*p).reset as *const c_void) {
            ret = PTR_ERR((*p).reset as *const c_void);
            dev_err_probe((*p).dev, ret, c"Failed on reset GPIO\n".as_ptr());
            goto_err(p, ret);
            put_device(physdev);
            return ret;
        }
    }

    put_device(physdev);
    0
}

unsafe fn goto_err(p: *mut tasdevice_priv, ret: c_int) {
    dev_err((*p).dev, c"read acpi error, ret: %d\n".as_ptr(), ret);
}

unsafe extern "C" fn tas2781_hda_playback_hook(dev: *mut device, action: c_int) {
    let tas_hda = dev_get_drvdata(dev) as *mut tas2781_hda;
    let tas_priv = (*tas_hda).priv_;

    if action == HDA_GEN_PCM_ACT_OPEN {
        pm_runtime_get_sync(dev);
        mutex_lock(&mut (*tas_priv).codec_lock);
        if (*tas_priv).fw_state == TASDEVICE_DSP_FW_ALL_OK {
            tasdevice_tuning_switch((*tas_hda).priv_, 0, false);
        }
        mutex_unlock(&mut (*tas_priv).codec_lock);
    } else if action == HDA_GEN_PCM_ACT_CLOSE {
        mutex_lock(&mut (*tas_priv).codec_lock);
        if (*tas_priv).fw_state == TASDEVICE_DSP_FW_ALL_OK {
            tasdevice_tuning_switch(tas_priv, 1, false);
        }
        mutex_unlock(&mut (*tas_priv).codec_lock);
        pm_runtime_put_autosuspend(dev);
    }
}

/*
 * tas2781_digital_getvol - get the volum control
 * @kcontrol: control pointer
 * @ucontrol: User data
 *
 * Customer Kcontrol for tas2781 is primarily for regmap booking, paging
 * depends on internal regmap mechanism.
 * tas2781 contains book and page two-level register map, especially
 * book switching will set the register BXXP00R7F, after switching to the
 * correct book, then leverage the mechanism for paging to access the
 * register.
 *
 * Return 0 if succeeded.
 */
unsafe extern "C" fn tas2781_digital_getvol(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let tas_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;

    mutex_lock(&mut (*tas_priv).codec_lock);
    let ret = tasdevice_spi_digital_getvol(tas_priv, ucontrol, mc);
    mutex_unlock(&mut (*tas_priv).codec_lock);
    ret
}

unsafe extern "C" fn tas2781_amp_getvol(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let tas_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;

    mutex_lock(&mut (*tas_priv).codec_lock);
    let ret = tasdevice_spi_amp_getvol(tas_priv, ucontrol, mc);
    mutex_unlock(&mut (*tas_priv).codec_lock);
    ret
}

unsafe extern "C" fn tas2781_digital_putvol(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let tas_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;

    mutex_lock(&mut (*tas_priv).codec_lock);
    let ret = tasdevice_spi_digital_putvol(tas_priv, ucontrol, mc);
    mutex_unlock(&mut (*tas_priv).codec_lock);
    ret
}

unsafe extern "C" fn tas2781_amp_putvol(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let tas_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;

    mutex_lock(&mut (*tas_priv).codec_lock);
    let ret = tasdevice_spi_amp_putvol(tas_priv, ucontrol, mc);
    mutex_unlock(&mut (*tas_priv).codec_lock);
    ret
}

unsafe extern "C" fn tas2781_force_fwload_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let tas_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;

    (*ucontrol).value.integer.value[0] = (*tas_priv).force_fwload_status as c_int;
    dev_dbg(
        (*tas_priv).dev,
        c"%s : Force FWload %s\n".as_ptr(),
        c"tas2781_force_fwload_get".as_ptr(),
        str_on_off((*tas_priv).force_fwload_status),
    );

    0
}

unsafe extern "C" fn tas2781_force_fwload_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let tas_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;
    let val = (*ucontrol).value.integer.value[0] != 0;
    let change: bool_;

    if (*tas_priv).force_fwload_status == val {
        change = false;
    } else {
        change = true;
        (*tas_priv).force_fwload_status = val;
    }
    dev_dbg(
        (*tas_priv).dev,
        c"%s : Force FWload %s\n".as_ptr(),
        c"tas2781_force_fwload_put".as_ptr(),
        str_on_off((*tas_priv).force_fwload_status),
    );

    change as c_int
}

/* ACARD_SINGLE_RANGE_EXT_TLV and ACARD_SINGLE_BOOL_EXT macro expansion depends on ALSA definitions. */
static mut tas2781_snd_ctls: [snd_kcontrol_new; 3] = [
    snd_kcontrol_new {
        iface: 0,
        name: ptr::null(),
        info: None,
        get: Some(tas2781_amp_getvol),
        put: Some(tas2781_amp_putvol),
        private_value: TAS2781_AMP_LEVEL as usize,
        tlv: tas2781_amp_tlv,
    },
    snd_kcontrol_new {
        iface: 0,
        name: ptr::null(),
        info: None,
        get: Some(tas2781_digital_getvol),
        put: Some(tas2781_digital_putvol),
        private_value: TAS2781_DVC_LVL as usize,
        tlv: tas2781_dvc_tlv,
    },
    snd_kcontrol_new {
        iface: 0,
        name: ptr::null(),
        info: None,
        get: Some(tas2781_force_fwload_get),
        put: Some(tas2781_force_fwload_put),
        private_value: 0,
        tlv: ptr::null(),
    },
];

static mut tas2781_prof_ctl: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_CARD,
    name: ptr::null(),
    info: Some(tasdevice_info_profile),
    get: Some(tasdevice_get_profile_id),
    put: Some(tasdevice_set_profile_id),
    private_value: 0,
    tlv: ptr::null(),
};

static mut tas2781_dsp_ctls: [snd_kcontrol_new; 2] = [
    /* Speaker Program */
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_CARD,
        name: ptr::null(),
        info: Some(tasdevice_info_programs),
        get: Some(tasdevice_program_get),
        put: Some(tasdevice_program_put),
        private_value: 0,
        tlv: ptr::null(),
    },
    /* Speaker Config */
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_CARD,
        name: ptr::null(),
        info: Some(tasdevice_info_config),
        get: Some(tasdevice_config_get),
        put: Some(tasdevice_config_put),
        private_value: 0,
        tlv: ptr::null(),
    },
];

unsafe extern "C" fn tas2781_hda_remove_controls(tas_hda: *mut tas2781_hda) {
    let codec = (*(*tas_hda).priv_).codec;
    let h_priv = (*tas_hda).hda_priv as *mut tas2781_hda_spi_priv;

    snd_ctl_remove((*codec).card, (*tas_hda).dsp_prog_ctl);

    snd_ctl_remove((*codec).card, (*tas_hda).dsp_conf_ctl);

    for i in (0..ARRAY_SIZE(&(*h_priv).snd_ctls) as isize).rev() {
        snd_ctl_remove((*codec).card, (*h_priv).snd_ctls[i as usize]);
    }

    snd_ctl_remove((*codec).card, (*tas_hda).prof_ctl);
}

unsafe extern "C" fn tas2781_hda_spi_prf_ctl(h: *mut tas2781_hda) -> c_int {
    let p = (*h).priv_;
    let c = (*p).codec;
    let mut name = [0 as c_char; 64];

    snprintf(name.as_mut_ptr(), size_of::<[c_char; 64]>(), c"Speaker-%d Profile Id".as_ptr(), (*p).index as c_int);
    tas2781_prof_ctl.name = name.as_ptr();
    (*h).prof_ctl = snd_ctl_new1(&raw const tas2781_prof_ctl, p as *mut c_void);
    let rc = snd_ctl_add((*c).card, (*h).prof_ctl);
    if rc != 0 {
        dev_err((*p).dev, c"Failed to add KControl: %s, rc = %d\n".as_ptr(), tas2781_prof_ctl.name, rc);
    }
    rc
}

unsafe extern "C" fn tas2781_hda_spi_snd_ctls(h: *mut tas2781_hda) -> c_int {
    let h_priv = (*h).hda_priv as *mut tas2781_hda_spi_priv;
    let p = (*h).priv_;
    let c = (*p).codec;
    let mut name = [0 as c_char; 64];
    let mut i: usize = 0;

    snprintf(name.as_mut_ptr(), size_of::<[c_char; 64]>(), c"Speaker-%d Analog Volume".as_ptr(), (*p).index as c_int);
    tas2781_snd_ctls[i].name = name.as_ptr();
    (*h_priv).snd_ctls[i] = snd_ctl_new1(&raw const tas2781_snd_ctls[i], p as *mut c_void);
    let mut rc = snd_ctl_add((*c).card, (*h_priv).snd_ctls[i]);
    if rc != 0 {
        dev_err((*p).dev, c"Failed to add KControl: %s, rc = %d\n".as_ptr(), tas2781_snd_ctls[i].name, rc);
        return rc;
    }
    i += 1;
    snprintf(name.as_mut_ptr(), size_of::<[c_char; 64]>(), c"Speaker-%d Digital Volume".as_ptr(), (*p).index as c_int);
    tas2781_snd_ctls[i].name = name.as_ptr();
    (*h_priv).snd_ctls[i] = snd_ctl_new1(&raw const tas2781_snd_ctls[i], p as *mut c_void);
    rc = snd_ctl_add((*c).card, (*h_priv).snd_ctls[i]);
    if rc != 0 {
        dev_err((*p).dev, c"Failed to add KControl: %s, rc = %d\n".as_ptr(), tas2781_snd_ctls[i].name, rc);
        return rc;
    }
    i += 1;
    snprintf(name.as_mut_ptr(), size_of::<[c_char; 64]>(), c"Force Speaker-%d FW Load".as_ptr(), (*p).index as c_int);
    tas2781_snd_ctls[i].name = name.as_ptr();
    (*h_priv).snd_ctls[i] = snd_ctl_new1(&raw const tas2781_snd_ctls[i], p as *mut c_void);
    rc = snd_ctl_add((*c).card, (*h_priv).snd_ctls[i]);
    if rc != 0 {
        dev_err((*p).dev, c"Failed to add KControl: %s, rc = %d\n".as_ptr(), tas2781_snd_ctls[i].name, rc);
    }
    rc
}

unsafe extern "C" fn tas2781_hda_spi_dsp_ctls(h: *mut tas2781_hda) -> c_int {
    let p = (*h).priv_;
    let c = (*p).codec;
    let mut name = [0 as c_char; 64];
    let mut i: usize = 0;

    snprintf(name.as_mut_ptr(), size_of::<[c_char; 64]>(), c"Speaker-%d Program Id".as_ptr(), (*p).index as c_int);
    tas2781_dsp_ctls[i].name = name.as_ptr();
    (*h).dsp_prog_ctl = snd_ctl_new1(&raw const tas2781_dsp_ctls[i], p as *mut c_void);
    let mut rc = snd_ctl_add((*c).card, (*h).dsp_prog_ctl);
    if rc != 0 {
        dev_err((*p).dev, c"Failed to add KControl: %s, rc = %d\n".as_ptr(), tas2781_dsp_ctls[i].name, rc);
        return rc;
    }
    i += 1;
    snprintf(name.as_mut_ptr(), size_of::<[c_char; 64]>(), c"Speaker-%d Config Id".as_ptr(), (*p).index as c_int);
    tas2781_dsp_ctls[i].name = name.as_ptr();
    (*h).dsp_conf_ctl = snd_ctl_new1(&raw const tas2781_dsp_ctls[i], p as *mut c_void);
    rc = snd_ctl_add((*c).card, (*h).dsp_conf_ctl);
    if rc != 0 {
        dev_err((*p).dev, c"Failed to add KControl: %s, rc = %d\n".as_ptr(), tas2781_dsp_ctls[i].name, rc);
    }

    rc
}

unsafe extern "C" fn tasdev_fw_ready(fmw: *const firmware, context: *mut c_void) {
    let tas_priv = context as *mut tasdevice_priv;
    let tas_hda = dev_get_drvdata((*tas_priv).dev) as *mut tas2781_hda;
    let codec = (*tas_priv).codec;

    pm_runtime_active_auto_enter((*tas_priv).dev);
    mutex_lock(&mut (*tas_priv).codec_lock);

    let mut ret = tasdevice_rca_parser(tas_priv, fmw);
    if ret != 0 {
        mutex_unlock(&mut (*tas_priv).codec_lock);
        pm_runtime_active_auto_exit((*tas_priv).dev);
        release_firmware(fmw);
        return;
    }

    /* Add control one time only. */
    ret = tas2781_hda_spi_prf_ctl(tas_hda);
    if ret != 0 {
        mutex_unlock(&mut (*tas_priv).codec_lock);
        pm_runtime_active_auto_exit((*tas_priv).dev);
        release_firmware(fmw);
        return;
    }

    ret = tas2781_hda_spi_snd_ctls(tas_hda);
    if ret != 0 {
        mutex_unlock(&mut (*tas_priv).codec_lock);
        pm_runtime_active_auto_exit((*tas_priv).dev);
        release_firmware(fmw);
        return;
    }

    tasdevice_dsp_remove(tas_priv);

    (*tas_priv).fw_state = TASDEVICE_DSP_FW_PENDING;
    scnprintf(
        (*tas_priv).coef_binaryname.as_mut_ptr(),
        64,
        c"TAS2XXX%04X-%01d.bin".as_ptr(),
        lower_16_bits((*codec).core.subsystem_id),
        (*tas_priv).index as c_int,
    );
    ret = tasdevice_dsp_parser(tas_priv);
    if ret != 0 {
        dev_err((*tas_priv).dev, c"dspfw load %s error\n".as_ptr(), (*tas_priv).coef_binaryname.as_ptr());
        (*tas_priv).fw_state = TASDEVICE_DSP_FW_FAIL;
        mutex_unlock(&mut (*tas_priv).codec_lock);
        pm_runtime_active_auto_exit((*tas_priv).dev);
        release_firmware(fmw);
        return;
    }

    ret = tas2781_hda_spi_dsp_ctls(tas_hda);
    if ret != 0 {
        mutex_unlock(&mut (*tas_priv).codec_lock);
        pm_runtime_active_auto_exit((*tas_priv).dev);
        release_firmware(fmw);
        return;
    }
    /* Perform AMP reset before firmware download. */
    tas2781_spi_reset(tas_priv);
    (*tas_priv).rcabin.profile_cfg_id = 0;

    (*tas_priv).fw_state = TASDEVICE_DSP_FW_ALL_OK;

    ret = tasdevice_prmg_load(tas_priv, 0);
    if ret < 0 {
        dev_err((*tas_priv).dev, c"FW download failed = %d\n".as_ptr(), ret);
        mutex_unlock(&mut (*tas_priv).codec_lock);
        pm_runtime_active_auto_exit((*tas_priv).dev);
        release_firmware(fmw);
        return;
    }
    (*tas_priv).fw_state = TASDEVICE_DSP_FW_ALL_OK;

    if (*(*tas_priv).fmw).nr_programs > 0 {
        (*(*tas_priv).tasdevice.add((*tas_priv).index as usize)).cur_prog = 0;
    }
    if (*(*tas_priv).fmw).nr_configurations > 0 {
        (*(*tas_priv).tasdevice.add((*tas_priv).index as usize)).cur_conf = 0;
    }

    /*
     * If calibrated data occurs error, dsp will still works with default
     * calibrated data inside algo.
     */
    tas2781_save_calibration(tas_hda);
    mutex_unlock(&mut (*tas_priv).codec_lock);
    pm_runtime_active_auto_exit((*tas_priv).dev);
    release_firmware(fmw);
}

unsafe extern "C" fn tas2781_hda_bind(dev: *mut device, _master: *mut device, master_data: *mut c_void) -> c_int {
    let tas_hda = dev_get_drvdata(dev) as *mut tas2781_hda;
    let parent = master_data as *mut hda_component_parent;
    let comp: *mut hda_component;
    let codec: *mut hda_codec;

    comp = hda_component_from_index(parent, (*(*tas_hda).priv_).index as c_int);
    if comp.is_null() {
        return -EINVAL;
    }

    if !(*comp).dev.is_null() {
        return -EBUSY;
    }

    codec = (*parent).codec;

    pm_runtime_active_auto_enter(dev);

    (*comp).dev = dev;

    strscpy((*comp).name.as_mut_ptr(), dev_name(dev), size_of::<[c_char; 64]>());

    let ret = tascodec_spi_init((*tas_hda).priv_, codec as *mut c_void, THIS_MODULE, Some(tasdev_fw_ready));
    if ret == 0 {
        (*comp).playback_hook = Some(tas2781_hda_playback_hook);
    }

    /* Only HP Laptop support SPI-based TAS2781 */
    (*tas_hda).catlog_id = HP;

    pm_runtime_active_auto_exit(dev);
    ret
}

unsafe extern "C" fn tas2781_hda_unbind(dev: *mut device, _master: *mut device, master_data: *mut c_void) {
    let tas_hda = dev_get_drvdata(dev) as *mut tas2781_hda;
    let parent = master_data as *mut hda_component_parent;
    let tas_priv = (*tas_hda).priv_;

    let comp = hda_component_from_index(parent, (*tas_priv).index as c_int);
    if !comp.is_null() && (*comp).dev == dev {
        (*comp).dev = ptr::null_mut();
        memset((*comp).name.as_mut_ptr() as *mut c_void, 0, size_of::<[c_char; 64]>());
        (*comp).playback_hook = None;
    }

    request_firmware_nowait_cancel((*tas_priv).dev, tas_priv as *mut c_void, Some(tasdev_fw_ready));

    tas2781_hda_remove_controls(tas_hda);

    tasdevice_config_info_remove(tas_priv);
    tasdevice_dsp_remove(tas_priv);

    (*(*tas_hda).priv_).fw_state = TASDEVICE_DSP_FW_PENDING;
}

static tas2781_hda_comp_ops: component_ops = component_ops {
    bind: Some(tas2781_hda_bind),
    unbind: Some(tas2781_hda_unbind),
};

unsafe extern "C" fn tas2781_hda_spi_probe(spi: *mut spi_device) -> c_int {
    let hda_priv: *mut tas2781_hda_spi_priv;
    let tas_priv: *mut tasdevice_priv;
    let tas_hda: *mut tas2781_hda;
    let device_name: *const c_char;
    let mut ret: c_int = 0;

    tas_hda = devm_kzalloc(&mut (*spi).dev, size_of::<tas2781_hda>(), GFP_KERNEL) as *mut tas2781_hda;
    if tas_hda.is_null() {
        return -ENOMEM;
    }

    hda_priv = devm_kzalloc(&mut (*spi).dev, size_of::<tas2781_hda_spi_priv>(), GFP_KERNEL) as *mut tas2781_hda_spi_priv;
    if hda_priv.is_null() {
        return -ENOMEM;
    }

    (*tas_hda).hda_priv = hda_priv as *mut c_void;
    (*spi).max_speed_hz = TAS2781_SPI_MAX_FREQ;

    tas_priv = devm_kzalloc(&mut (*spi).dev, size_of::<tasdevice_priv>(), GFP_KERNEL) as *mut tasdevice_priv;
    if tas_priv.is_null() {
        return -ENOMEM;
    }
    (*tas_priv).dev = &mut (*spi).dev;
    (*tas_hda).priv_ = tas_priv;
    (*tas_priv).regmap = devm_regmap_init_spi(spi, &tasdevice_regmap);
    if IS_ERR((*tas_priv).regmap as *const c_void) {
        ret = PTR_ERR((*tas_priv).regmap as *const c_void);
        dev_err((*tas_priv).dev, c"Failed to allocate regmap: %d\n".as_ptr(), ret);
        return ret;
    }
    if !strstr(dev_name(&mut (*spi).dev), c"TXNW2781".as_ptr()).is_null() {
        device_name = c"TXNW2781".as_ptr();
        (*(*tas_hda).priv_).chip_id = TAS2781;
    } else {
        dev_err((*tas_priv).dev, c"Unmatched spi dev %s\n".as_ptr(), dev_name(&mut (*spi).dev));
        return -ENODEV;
    }

    (*tas_priv).irq = (*spi).irq;
    dev_set_drvdata(&mut (*spi).dev, tas_hda as *mut c_void);
    ret = tas2781_read_acpi(tas_hda, device_name, spi_get_chipselect(spi, 0));
    if ret != 0 {
        return dev_err_probe((*tas_priv).dev, ret, c"Platform not supported\n".as_ptr());
    }

    tasdevice_spi_init(tas_priv);

    pm_runtime_set_autosuspend_delay((*tas_priv).dev, 3000);
    pm_runtime_use_autosuspend((*tas_priv).dev);
    pm_runtime_set_active((*tas_priv).dev);
    pm_runtime_get_noresume((*tas_priv).dev);
    pm_runtime_enable((*tas_priv).dev);

    pm_runtime_put_autosuspend((*tas_priv).dev);

    ret = component_add((*tas_priv).dev, &tas2781_hda_comp_ops);
    if ret != 0 {
        dev_err((*tas_priv).dev, c"Register component fail: %d\n".as_ptr(), ret);
        pm_runtime_disable((*tas_priv).dev);
        tas2781_hda_remove(&mut (*spi).dev, &tas2781_hda_comp_ops);
    }

    ret
}

unsafe extern "C" fn tas2781_hda_spi_remove(spi: *mut spi_device) {
    tas2781_hda_remove(&mut (*spi).dev, &tas2781_hda_comp_ops);
}

unsafe extern "C" fn tas2781_runtime_suspend(dev: *mut device) -> c_int {
    let tas_hda = dev_get_drvdata(dev) as *mut tas2781_hda;
    let tas_priv = (*tas_hda).priv_;

    mutex_lock(&mut (*tas_priv).codec_lock);

    if (*tas_priv).fw_state == TASDEVICE_DSP_FW_ALL_OK && (*tas_priv).playback_started {
        tasdevice_tuning_switch(tas_priv, 1, false);
    }

    (*(*tas_priv).tasdevice.add((*tas_priv).index as usize)).cur_book = -1;
    (*(*tas_priv).tasdevice.add((*tas_priv).index as usize)).cur_conf = -1;

    mutex_unlock(&mut (*tas_priv).codec_lock);
    0
}

unsafe extern "C" fn tas2781_runtime_resume(dev: *mut device) -> c_int {
    let tas_hda = dev_get_drvdata(dev) as *mut tas2781_hda;
    let tas_priv = (*tas_hda).priv_;

    mutex_lock(&mut (*tas_priv).codec_lock);

    if (*tas_priv).fw_state == TASDEVICE_DSP_FW_ALL_OK && (*tas_priv).playback_started {
        tasdevice_tuning_switch(tas_priv, 0, false);
    }

    mutex_unlock(&mut (*tas_priv).codec_lock);
    0
}

unsafe extern "C" fn tas2781_system_suspend(dev: *mut device) -> c_int {
    let tas_hda = dev_get_drvdata(dev) as *mut tas2781_hda;
    let tas_priv = (*tas_hda).priv_;

    let ret = pm_runtime_force_suspend(dev);
    if ret != 0 {
        return ret;
    }

    /* Shutdown chip before system suspend */
    if (*tas_priv).fw_state == TASDEVICE_DSP_FW_ALL_OK && (*tas_priv).playback_started {
        tasdevice_tuning_switch(tas_priv, 1, false);
    }

    0
}

unsafe extern "C" fn tas2781_system_resume(dev: *mut device) -> c_int {
    let tas_hda = dev_get_drvdata(dev) as *mut tas2781_hda;
    let tas_priv = (*tas_hda).priv_;
    let mut val: c_int = 0;

    let mut ret = pm_runtime_force_resume(dev);
    if ret != 0 {
        return ret;
    }

    mutex_lock(&mut (*tas_priv).codec_lock);
    ret = ((*tas_priv).dev_read.unwrap())(
        tas_priv,
        (*tas_priv).index as u16,
        TAS2781_REG_CLK_CONFIG,
        &mut val as *mut c_int as *mut c_uint,
    );
    if ret < 0 {
        mutex_unlock(&mut (*tas_priv).codec_lock);
        return ret;
    }

    if val as c_uint == TAS2781_REG_CLK_CONFIG_RESET {
        (*(*tas_priv).tasdevice.add((*tas_priv).index as usize)).cur_book = -1;
        (*(*tas_priv).tasdevice.add((*tas_priv).index as usize)).cur_conf = -1;
        (*(*tas_priv).tasdevice.add((*tas_priv).index as usize)).cur_prog = -1;

        ret = tasdevice_prmg_load(tas_priv, 0);
        if ret < 0 {
            dev_err((*tas_priv).dev, c"FW download failed = %d\n".as_ptr(), ret);
            mutex_unlock(&mut (*tas_priv).codec_lock);
            return ret;
        }
        (*tas_priv).fw_state = TASDEVICE_DSP_FW_ALL_OK;

        if (*tas_priv).playback_started {
            tasdevice_tuning_switch(tas_priv, 0, false);
        }
    }

    mutex_unlock(&mut (*tas_priv).codec_lock);
    ret
}

static tas2781_hda_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(tas2781_runtime_suspend),
    runtime_resume: Some(tas2781_runtime_resume),
    suspend: Some(tas2781_system_suspend),
    resume: Some(tas2781_system_resume),
};

static tas2781_hda_spi_id: [spi_device_id; 2] = [
    spi_device_id {
        name: *b"tas2781-hda\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0" as [u8; 32] as [c_char; 32],
        driver_data: 0,
    },
    spi_device_id {
        name: [0; 32],
        driver_data: 0,
    },
];

static tas2781_acpi_hda_match: [acpi_device_id; 2] = [
    acpi_device_id {
        id: *b"TXNW2781\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16],
        driver_data: 0,
    },
    acpi_device_id {
        id: [0; 16],
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(acpi, tas2781_acpi_hda_match); */

static mut tas2781_hda_spi_driver: spi_driver = spi_driver {
    driver: driver_inner {
        name: c"tas2781-hda".as_ptr(),
        acpi_match_table: tas2781_acpi_hda_match.as_ptr(),
        pm: &tas2781_hda_pm_ops,
    },
    id_table: tas2781_hda_spi_id.as_ptr(),
    probe: Some(tas2781_hda_spi_probe),
    remove: Some(tas2781_hda_spi_remove),
};
/* module_spi_driver(tas2781_hda_spi_driver); */

/* MODULE_DESCRIPTION("TAS2781 HDA SPI Driver"); */
/* MODULE_AUTHOR("Baojun, Xu, <baojun.xug@ti.com>"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_IMPORT_NS("SND_SOC_TAS2781_FMWLIB"); */
/* MODULE_IMPORT_NS("SND_HDA_SCODEC_TAS2781"); */

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static tas2781_amp_tlv: *const c_uint;
    static tas2781_dvc_tlv: *const c_uint;

    fn tasdevice_dev_bulk_read(
        tas_priv: *mut tasdevice_priv,
        chn: u16,
        reg: c_uint,
        data: *mut u8,
        len: c_uint,
    ) -> c_int;
    fn tasdevice_dev_read(
        tas_priv: *mut tasdevice_priv,
        chn: u16,
        reg: c_uint,
        val: *mut c_uint,
    ) -> c_int;
    fn tasdevice_dev_write(
        tas_priv: *mut tasdevice_priv,
        chn: u16,
        reg: c_uint,
        val: c_uint,
    ) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn fsleep(usecs: c_uint);
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn crc8_populate_msb(table: *mut u8, polynomial: u8);
    fn request_firmware_nowait(
        module: *mut module,
        uevent: c_int,
        name: *const c_char,
        device: *mut device,
        gfp: c_uint,
        context: *mut c_void,
        cont: Option<unsafe extern "C" fn(*const firmware, *mut c_void)>,
    ) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn acpi_dev_get_first_match_dev(hid: *const c_char, uid: *const c_char, hrv: c_int) -> *mut acpi_device;
    fn acpi_get_first_physical_node(adev: *mut acpi_device) -> *mut device;
    fn acpi_dev_put(adev: *mut acpi_device);
    fn get_device(dev: *mut device) -> *mut device;
    fn put_device(dev: *mut device);
    fn strscpy(dest: *mut c_char, src: *const c_char, count: size_t) -> size_t;
    fn device_property_count_u32(dev: *mut device, property: *const c_char) -> c_int;
    fn device_property_read_u32_array(dev: *mut device, property: *const c_char, values: *mut u32, nval: size_t) -> c_int;
    fn devm_gpiod_get_index_optional(dev: *mut device, con_id: *const c_char, idx: c_uint, flags: c_int) -> *mut gpio_desc;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn tasdevice_tuning_switch(tas_priv: *mut tasdevice_priv, state: c_int, profile: bool_);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn str_on_off(v: bool_) -> *const c_char;
    fn tasdevice_info_profile(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn tasdevice_get_profile_id(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn tasdevice_set_profile_id(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn tasdevice_info_programs(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn tasdevice_program_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn tasdevice_program_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn tasdevice_info_config(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn tasdevice_config_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn tasdevice_config_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_ctl_remove(card: *mut snd_card, kcontrol: *mut snd_kcontrol);
    fn snd_ctl_new1(kcontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn tasdevice_rca_parser(tas_priv: *mut tasdevice_priv, fmw: *const firmware) -> c_int;
    fn tasdevice_dsp_remove(tas_priv: *mut tasdevice_priv);
    fn tasdevice_dsp_parser(tas_priv: *mut tasdevice_priv) -> c_int;
    fn tasdevice_prmg_load(tas_priv: *mut tasdevice_priv, prm_no: c_int) -> c_int;
    fn tas2781_save_calibration(tas_hda: *mut tas2781_hda);
    fn release_firmware(fmw: *const firmware);
    fn hda_component_from_index(parent: *mut hda_component_parent, index: c_int) -> *mut hda_component;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn request_firmware_nowait_cancel(
        device: *mut device,
        context: *mut c_void,
        cont: Option<unsafe extern "C" fn(*const firmware, *mut c_void)>,
    );
    fn tasdevice_config_info_remove(tas_priv: *mut tasdevice_priv);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn spi_get_chipselect(spi: *mut spi_device, idx: c_uint) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn component_add(dev: *mut device, ops: *const component_ops) -> c_int;
    fn pm_runtime_disable(dev: *mut device);
    fn tas2781_hda_remove(dev: *mut device, ops: *const component_ops);
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn pm_runtime_active_auto_enter(dev: *mut device);
    fn pm_runtime_active_auto_exit(dev: *mut device);
}

extern "Rust" {
    static HZ_PER_MHZ: c_uint;
    static TASDEVICE_PAGE_SELECT: c_uint;
    static REGCACHE_NONE: c_uint;
    static TASDEVICE_BOOKCTL_REG: c_uint;
    static EXDEV: c_int;
    static TASDEVICE_REG_SWRESET: c_uint;
    static TASDEVICE_REG_SWRESET_RESET: c_uint;
    static TASDEVICE_CRC8_POLYNOMIAL: u8;
    static FW_ACTION_UEVENT: c_int;
    static GFP_KERNEL: c_uint;
    static HDA_MAX_COMPONENTS: c_uint;
    static ENODEV: c_int;
    static EINVAL: c_int;
    static U8_MAX: u8;
    static GPIOD_OUT_LOW: c_int;
    static HDA_GEN_PCM_ACT_OPEN: c_int;
    static HDA_GEN_PCM_ACT_CLOSE: c_int;
    static TASDEVICE_DSP_FW_ALL_OK: c_int;
    static TAS2781_AMP_LEVEL: c_uint;
    static TAS2781_DVC_LVL: c_uint;
    static SNDRV_CTL_ELEM_IFACE_CARD: c_uint;
    static TASDEVICE_DSP_FW_PENDING: c_int;
    static TASDEVICE_DSP_FW_FAIL: c_int;
    static EBUSY: c_int;
    static HP: c_int;
    static ENOMEM: c_int;
    static TAS2781: c_int;
}

fn IS_ERR(ptr: *const c_void) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

fn PTR_ERR(ptr: *const c_void) -> c_int {
    ptr as isize as c_int
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
