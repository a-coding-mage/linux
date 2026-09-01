// SPDX-License-Identifier: GPL-2.0
/*
 * McBSP Sidetone support
 *
 * Copyright (C) 2004 Nokia Corporation
 * Author: Samuel Ortiz <samuel.ortiz@nokia.com>
 *
 * Contact: Jarkko Nikula <jarkko.nikula@bitmer.com>
 *          Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

// External kernel module and device management, interrupt, error handling, clock, delay, I/O, and memory allocation
// use linux_kernel::*;

// Sidetone support structs and definitions from separate modules
// use crate::omap_mcbsp::*;
// use crate::omap_mcbsp_priv::*;

/* OMAP3 sidetone control registers */
const OMAP_ST_REG_REV: u16 = 0x00;
const OMAP_ST_REG_SYSCONFIG: u16 = 0x10;
const OMAP_ST_REG_IRQSTATUS: u16 = 0x18;
const OMAP_ST_REG_IRQENABLE: u16 = 0x1C;
const OMAP_ST_REG_SGAINCR: u16 = 0x24;
const OMAP_ST_REG_SFIRCR: u16 = 0x28;
const OMAP_ST_REG_SSELCR: u16 = 0x2C;

/********************** McBSP SSELCR bit definitions ***********************/
const SIDETONEEN: u32 = 1 << 10;

/********************** McBSP Sidetone SYSCONFIG bit definitions ***********/
const ST_AUTOIDLE: u32 = 1 << 0;

/********************** McBSP Sidetone SGAINCR bit definitions *************/
fn ST_CH0GAIN(value: i16) -> u32 {
    (value as u32) & 0xffff
}

fn ST_CH1GAIN(value: i16) -> u32 {
    (((value as u32) & 0xffff) << 16)
}

/********************** McBSP Sidetone SFIRCR bit definitions **************/
fn ST_FIRCOEFF(value: i16) -> u32 {
    (value as u32) & 0xffff
}

/********************** McBSP Sidetone SSELCR bit definitions **************/
const ST_SIDETONEEN: u32 = 1 << 0;
const ST_COEFFWREN: u32 = 1 << 1;
const ST_COEFFWRDONE: u32 = 1 << 2;

// External types from omap_mcbsp and kernel
#[repr(C)]
pub struct OmapMcbspStData {
    io_base_st: *mut core::ffi::c_void,
    mcbsp_iclk: *mut core::ffi::c_void,
    running: bool,
    enabled: bool,
    taps: [i16; 128],
    nr_taps: i32,
    ch0gain: i16,
    ch1gain: i16,
}

// Opaque external types
pub struct OmapMcbsp {
    pub st_data: *mut OmapMcbspStData,
    pub pdata: *mut core::ffi::c_void,
    pub lock: core::ffi::c_void,
    pub dev: *mut core::ffi::c_void,
    pub free: i32,
}

pub struct PlatformDevice {
    _private: core::ffi::c_void,
}

unsafe fn omap_mcbsp_st_write(mcbsp: *const OmapMcbsp, reg: u16, val: u32) {
    let st_data = (*mcbsp).st_data;
    let addr = ((*st_data).io_base_st as usize + reg as usize) as *mut u32;
    core::ptr::write_volatile(addr, val);
}

unsafe fn omap_mcbsp_st_read(mcbsp: *const OmapMcbsp, reg: u16) -> u32 {
    let st_data = (*mcbsp).st_data;
    let addr = ((*st_data).io_base_st as usize + reg as usize) as *const u32;
    core::ptr::read_volatile(addr)
}

// Macro equivalents
macro_rules! MCBSP_ST_READ {
    ($mcbsp:expr, $reg:ident) => {
        omap_mcbsp_st_read($mcbsp, concat_idents!(OMAP_ST_REG_, $reg))
    };
}

macro_rules! MCBSP_ST_WRITE {
    ($mcbsp:expr, $reg:ident, $val:expr) => {
        omap_mcbsp_st_write($mcbsp, concat_idents!(OMAP_ST_REG_, $reg), $val)
    };
}

// External function declarations
extern "C" {
    fn MCBSP_READ(mcbsp: *const OmapMcbsp, reg: u16) -> u32;
    fn MCBSP_WRITE(mcbsp: *const OmapMcbsp, reg: u16, val: u32);
    fn omap_mcbsp_st_start(mcbsp: *mut OmapMcbsp) -> i32;
    fn omap_mcbsp_st_stop(mcbsp: *mut OmapMcbsp) -> i32;
    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
    fn dev_warn(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
    fn sysfs_emit_at(buf: *mut u8, offset: usize, fmt: *const u8, ...) -> i32;
    fn dev_get_drvdata(dev: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn sscanf(s: *const u8, format: *const u8, ...) -> i32;
    fn platform_get_drvdata(pdev: *const PlatformDevice) -> *mut core::ffi::c_void;
    fn platform_get_resource_byname(
        pdev: *const PlatformDevice,
        type_: u32,
        name: *const u8,
    ) -> *mut core::ffi::c_void;
    fn devm_kzalloc(dev: *mut core::ffi::c_void, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_clk_get(dev: *mut core::ffi::c_void, id: *const u8) -> *mut core::ffi::c_void;
    fn devm_ioremap(
        dev: *mut core::ffi::c_void,
        offset: usize,
        size: usize,
    ) -> *mut core::ffi::c_void;
    fn devm_device_add_group(
        dev: *mut core::ffi::c_void,
        grp: *const core::ffi::c_void,
    ) -> i32;
    fn snd_kcontrol_chip(kcontrol: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn snd_soc_dai_get_drvdata(dai: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn snd_soc_rtd_to_cpu(rtd: *const core::ffi::c_void, num: i32) -> *mut core::ffi::c_void;
    fn snd_soc_add_dai_controls(
        dai: *const core::ffi::c_void,
        controls: *const core::ffi::c_void,
        num_controls: i32,
    ) -> i32;
}

// Stub implementations for macros that would expand to variable declarations
const IORESOURCE_MEM: u32 = 1;
const GFP_KERNEL: u32 = 0;

unsafe fn omap_mcbsp_st_on(mcbsp: *mut OmapMcbsp) {
    let mut w: u32;

    let pdata = (*mcbsp).pdata;
    if !pdata.is_null() {
        let force_ick_on_ptr = pdata as *mut usize;
        let force_ick_on = *force_ick_on_ptr;
        if force_ick_on != 0 {
            let func: unsafe extern "C" fn(*mut core::ffi::c_void, bool) =
                core::mem::transmute(force_ick_on);
            func((*(*mcbsp).st_data).mcbsp_iclk, true);
        }
    }

    w = omap_mcbsp_st_read(mcbsp, OMAP_ST_REG_SYSCONFIG);
    omap_mcbsp_st_write(mcbsp, OMAP_ST_REG_SYSCONFIG, w & !(ST_AUTOIDLE));

    w = MCBSP_READ(mcbsp, OMAP_ST_REG_SSELCR);
    MCBSP_WRITE(mcbsp, OMAP_ST_REG_SSELCR, w | SIDETONEEN);

    w = omap_mcbsp_st_read(mcbsp, OMAP_ST_REG_SSELCR);
    omap_mcbsp_st_write(mcbsp, OMAP_ST_REG_SSELCR, w | ST_SIDETONEEN);
}

unsafe fn omap_mcbsp_st_off(mcbsp: *mut OmapMcbsp) {
    let mut w: u32;

    w = omap_mcbsp_st_read(mcbsp, OMAP_ST_REG_SSELCR);
    omap_mcbsp_st_write(mcbsp, OMAP_ST_REG_SSELCR, w & !(ST_SIDETONEEN));

    w = MCBSP_READ(mcbsp, OMAP_ST_REG_SSELCR);
    MCBSP_WRITE(mcbsp, OMAP_ST_REG_SSELCR, w & !(SIDETONEEN));

    w = omap_mcbsp_st_read(mcbsp, OMAP_ST_REG_SYSCONFIG);
    omap_mcbsp_st_write(mcbsp, OMAP_ST_REG_SYSCONFIG, w | ST_AUTOIDLE);

    let pdata = (*mcbsp).pdata;
    if !pdata.is_null() {
        let force_ick_on_ptr = pdata as *mut usize;
        let force_ick_on = *force_ick_on_ptr;
        if force_ick_on != 0 {
            let func: unsafe extern "C" fn(*mut core::ffi::c_void, bool) =
                core::mem::transmute(force_ick_on);
            func((*(*mcbsp).st_data).mcbsp_iclk, false);
        }
    }
}

unsafe fn omap_mcbsp_st_fir_write(mcbsp: *mut OmapMcbsp, fir: *const i16) {
    let mut val: u32;
    let mut i: u16;

    val = omap_mcbsp_st_read(mcbsp, OMAP_ST_REG_SSELCR);

    if (val & ST_COEFFWREN) != 0 {
        omap_mcbsp_st_write(mcbsp, OMAP_ST_REG_SSELCR, val & !(ST_COEFFWREN));
    }

    omap_mcbsp_st_write(mcbsp, OMAP_ST_REG_SSELCR, val | ST_COEFFWREN);

    i = 0;
    while i < 128 {
        omap_mcbsp_st_write(mcbsp, OMAP_ST_REG_SFIRCR, ST_FIRCOEFF(*fir.add(i as usize)));
        i = i.wrapping_add(1);
    }

    i = 0;

    val = omap_mcbsp_st_read(mcbsp, OMAP_ST_REG_SSELCR);
    while (val & ST_COEFFWRDONE) == 0 && i < 1000 {
        i = i.wrapping_add(1);
        val = omap_mcbsp_st_read(mcbsp, OMAP_ST_REG_SSELCR);
    }

    omap_mcbsp_st_write(mcbsp, OMAP_ST_REG_SSELCR, val & !(ST_COEFFWREN));

    if i == 1000 {
        dev_err((*mcbsp).dev, b"McBSP FIR load error!\n\0".as_ptr());
    }
}

unsafe fn omap_mcbsp_st_chgain(mcbsp: *mut OmapMcbsp) {
    let st_data = (*mcbsp).st_data;

    omap_mcbsp_st_write(
        mcbsp,
        OMAP_ST_REG_SGAINCR,
        ST_CH0GAIN((*st_data).ch0gain) | ST_CH1GAIN((*st_data).ch1gain),
    );
}

pub unsafe fn omap_mcbsp_st_set_chgain(mcbsp: *mut OmapMcbsp, channel: i32, chgain: i16) -> i32 {
    let st_data = (*mcbsp).st_data;
    let mut ret: i32 = 0;

    if st_data.is_null() {
        return -2; // -ENOENT
    }

    // guard(spinlock_irq)(&mcbsp->lock);
    if channel == 0 {
        (*st_data).ch0gain = chgain;
    } else if channel == 1 {
        (*st_data).ch1gain = chgain;
    } else {
        ret = -22; // -EINVAL
    }

    if (*st_data).enabled {
        omap_mcbsp_st_chgain(mcbsp);
    }

    ret
}

pub unsafe fn omap_mcbsp_st_get_chgain(
    mcbsp: *mut OmapMcbsp,
    channel: i32,
    chgain: *mut i16,
) -> i32 {
    let st_data = (*mcbsp).st_data;
    let mut ret: i32 = 0;

    if st_data.is_null() {
        return -2; // -ENOENT
    }

    // guard(spinlock_irq)(&mcbsp->lock);
    if channel == 0 {
        *chgain = (*st_data).ch0gain;
    } else if channel == 1 {
        *chgain = (*st_data).ch1gain;
    } else {
        ret = -22; // -EINVAL
    }

    ret
}

pub unsafe fn omap_mcbsp_st_enable(mcbsp: *mut OmapMcbsp) -> i32 {
    let st_data = (*mcbsp).st_data;

    if st_data.is_null() {
        return -19; // -ENODEV
    }

    // guard(spinlock_irq)(&mcbsp->lock);
    (*st_data).enabled = true;
    omap_mcbsp_st_start(mcbsp);

    0
}

pub unsafe fn omap_mcbsp_st_disable(mcbsp: *mut OmapMcbsp) -> i32 {
    let st_data = (*mcbsp).st_data;

    if st_data.is_null() {
        return -19; // -ENODEV
    }

    // guard(spinlock_irq)(&mcbsp->lock);
    omap_mcbsp_st_stop(mcbsp);
    (*st_data).enabled = false;

    0
}

pub unsafe fn omap_mcbsp_st_is_enabled(mcbsp: *mut OmapMcbsp) -> i32 {
    let st_data = (*mcbsp).st_data;

    if st_data.is_null() {
        return -19; // -ENODEV
    }

    (*st_data).enabled as i32
}

pub unsafe fn st_taps_show(
    dev: *mut core::ffi::c_void,
    attr: *mut core::ffi::c_void,
    buf: *mut u8,
) -> isize {
    let mcbsp = dev_get_drvdata(dev) as *mut OmapMcbsp;
    let st_data = (*mcbsp).st_data;
    let mut status: isize = 0;
    let mut i: i32;

    // guard(spinlock_irq)(&mcbsp->lock);
    i = 0;
    while i < (*st_data).nr_taps {
        status += sysfs_emit_at(
            buf,
            status as usize,
            if i == 0 {
                b"%d\0".as_ptr()
            } else {
                b", %d\0".as_ptr()
            },
            (*st_data).taps[i as usize] as i32,
        ) as isize;
        i += 1;
    }
    if i > 0 {
        status += sysfs_emit_at(buf, status as usize, b"\n\0".as_ptr()) as isize;
    }

    status
}

pub unsafe fn st_taps_store(
    dev: *mut core::ffi::c_void,
    attr: *mut core::ffi::c_void,
    buf: *const u8,
    size: usize,
) -> isize {
    let mcbsp = dev_get_drvdata(dev) as *mut OmapMcbsp;
    let st_data = (*mcbsp).st_data;
    let mut val: i32;
    let mut tmp: i32;
    let mut status: i32;
    let mut i: i32 = 0;
    let mut buf_iter = buf;

    // guard(spinlock_irq)(&mcbsp->lock);
    memset(
        (*st_data).taps.as_mut_ptr() as *mut core::ffi::c_void,
        0,
        core::mem::size_of_val(&(*st_data).taps),
    );
    (*st_data).nr_taps = 0;

    loop {
        status = sscanf(buf_iter, b"%d%n\0".as_ptr(), &mut val as *mut i32, &mut tmp as *mut i32);
        if status < 0 || status == 0 {
            return -22; // -EINVAL
        }
        if val < -32768 || val > 32767 {
            return -22; // -EINVAL
        }
        (*st_data).taps[i as usize] = val as i16;
        i += 1;
        buf_iter = buf_iter.add(tmp as usize);
        if *buf_iter != b',' as u8 {
            break;
        }
        buf_iter = buf_iter.add(1);
    }

    (*st_data).nr_taps = i;

    size as isize
}

pub unsafe fn omap_mcbsp_st_start(mcbsp: *mut OmapMcbsp) -> i32 {
    let st_data = (*mcbsp).st_data;

    if (*st_data).enabled && !(*st_data).running {
        omap_mcbsp_st_fir_write(mcbsp, (*st_data).taps.as_ptr());
        omap_mcbsp_st_chgain(mcbsp);

        if (*mcbsp).free == 0 {
            omap_mcbsp_st_on(mcbsp);
            (*st_data).running = true;
        }
    }

    0
}

pub unsafe fn omap_mcbsp_st_stop(mcbsp: *mut OmapMcbsp) -> i32 {
    let st_data = (*mcbsp).st_data;

    if (*st_data).running {
        if (*mcbsp).free == 0 {
            omap_mcbsp_st_off(mcbsp);
            (*st_data).running = false;
        }
    }

    0
}

pub unsafe fn omap_mcbsp_st_init(pdev: *mut PlatformDevice) -> i32 {
    let mcbsp = platform_get_drvdata(pdev as *const PlatformDevice) as *mut OmapMcbsp;
    let mut st_data: *mut OmapMcbspStData;
    let mut res: *mut core::ffi::c_void;

    res = platform_get_resource_byname(
        pdev as *const PlatformDevice,
        IORESOURCE_MEM,
        b"sidetone\0".as_ptr(),
    );
    if res.is_null() {
        return 0;
    }

    st_data = devm_kzalloc(
        (*mcbsp).dev,
        core::mem::size_of::<OmapMcbspStData>(),
        GFP_KERNEL,
    ) as *mut OmapMcbspStData;
    if st_data.is_null() {
        return -12; // -ENOMEM
    }

    (*st_data).mcbsp_iclk = devm_clk_get((*mcbsp).dev, b"ick\0".as_ptr());
    if ((*st_data).mcbsp_iclk as isize) < 0 {
        dev_warn(
            (*mcbsp).dev,
            b"Failed to get ick, sidetone might be broken\n\0".as_ptr(),
        );
        (*st_data).mcbsp_iclk = core::ptr::null_mut();
    }

    // Simplified: resource_size would need to read the resource structure
    let res_start = res as usize;
    let res_size = 256; // Placeholder size, would be read from resource_size(res)
    (*st_data).io_base_st = devm_ioremap((*mcbsp).dev, res_start, res_size);
    if (*st_data).io_base_st.is_null() {
        return -12; // -ENOMEM
    }

    // device_add_group would add sysfs attributes
    // Placeholder: return value from devm_device_add_group
    let ret = devm_device_add_group((*mcbsp).dev, core::ptr::null());
    if ret != 0 {
        return ret;
    }

    (*mcbsp).st_data = st_data;

    0
}

pub unsafe fn omap_mcbsp_st_info_volsw(
    kcontrol: *mut core::ffi::c_void,
    uinfo: *mut core::ffi::c_void,
) -> i32 {
    // struct soc_mixer_control *mc =
    //     (struct soc_mixer_control *)kcontrol->private_value;
    // Simplified: access to nested structure would require knowing its layout
    let _max = 4;
    let _min = 0;

    // uinfo->type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    // uinfo->count = 1;
    // uinfo->value.integer.min = min;
    // uinfo->value.integer.max = max;

    0
}

// Helper macro expansion for OMAP_MCBSP_ST_CHANNEL_VOLUME
unsafe fn omap_mcbsp_set_st_ch0_volume(
    kc: *mut core::ffi::c_void,
    uc: *mut core::ffi::c_void,
) -> i32 {
    let cpu_dai = snd_kcontrol_chip(kc);
    let mcbsp = snd_soc_dai_get_drvdata(cpu_dai) as *mut OmapMcbsp;
    let _max = 4;
    let _min = 0;
    // let val = uc->value.integer.value[0];

    // Placeholder: would extract val from uc structure
    let val = 0;

    if val < _min || val > _max {
        return -22; // -EINVAL
    }

    omap_mcbsp_st_set_chgain(mcbsp, 0, val as i16)
}

unsafe fn omap_mcbsp_get_st_ch0_volume(
    kc: *mut core::ffi::c_void,
    uc: *mut core::ffi::c_void,
) -> i32 {
    let cpu_dai = snd_kcontrol_chip(kc);
    let mcbsp = snd_soc_dai_get_drvdata(cpu_dai) as *mut OmapMcbsp;
    let mut chgain: i16 = 0;

    if omap_mcbsp_st_get_chgain(mcbsp, 0, &mut chgain) != 0 {
        return -11; // -EAGAIN
    }

    // uc->value.integer.value[0] = chgain;

    0
}

unsafe fn omap_mcbsp_set_st_ch1_volume(
    kc: *mut core::ffi::c_void,
    uc: *mut core::ffi::c_void,
) -> i32 {
    let cpu_dai = snd_kcontrol_chip(kc);
    let mcbsp = snd_soc_dai_get_drvdata(cpu_dai) as *mut OmapMcbsp;
    let _max = 4;
    let _min = 0;
    // let val = uc->value.integer.value[0];

    let val = 0;

    if val < _min || val > _max {
        return -22; // -EINVAL
    }

    omap_mcbsp_st_set_chgain(mcbsp, 1, val as i16)
}

unsafe fn omap_mcbsp_get_st_ch1_volume(
    kc: *mut core::ffi::c_void,
    uc: *mut core::ffi::c_void,
) -> i32 {
    let cpu_dai = snd_kcontrol_chip(kc);
    let mcbsp = snd_soc_dai_get_drvdata(cpu_dai) as *mut OmapMcbsp;
    let mut chgain: i16 = 0;

    if omap_mcbsp_st_get_chgain(mcbsp, 1, &mut chgain) != 0 {
        return -11; // -EAGAIN
    }

    // uc->value.integer.value[0] = chgain;

    0
}

pub unsafe fn omap_mcbsp_st_put_mode(
    kcontrol: *mut core::ffi::c_void,
    ucontrol: *mut core::ffi::c_void,
) -> i32 {
    let cpu_dai = snd_kcontrol_chip(kcontrol);
    let mcbsp = snd_soc_dai_get_drvdata(cpu_dai) as *mut OmapMcbsp;
    // u8 value = ucontrol->value.integer.value[0];

    let value = 0u8;

    if value as i32 == omap_mcbsp_st_is_enabled(mcbsp) {
        return 0;
    }

    if value != 0 {
        omap_mcbsp_st_enable(mcbsp);
    } else {
        omap_mcbsp_st_disable(mcbsp);
    }

    1
}

pub unsafe fn omap_mcbsp_st_get_mode(
    kcontrol: *mut core::ffi::c_void,
    ucontrol: *mut core::ffi::c_void,
) -> i32 {
    let cpu_dai = snd_kcontrol_chip(kcontrol);
    let mcbsp = snd_soc_dai_get_drvdata(cpu_dai) as *mut OmapMcbsp;

    // ucontrol->value.integer.value[0] = omap_mcbsp_st_is_enabled(mcbsp);

    0
}

extern "C" {
    static omap_mcbsp2_st_controls: core::ffi::c_void;
    static omap_mcbsp3_st_controls: core::ffi::c_void;

    fn ARRAY_SIZE(arr: *const core::ffi::c_void) -> i32;
}

pub unsafe fn omap_mcbsp_st_add_controls(
    rtd: *mut core::ffi::c_void,
    port_id: i32,
) -> i32 {
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mcbsp = snd_soc_dai_get_drvdata(cpu_dai) as *mut OmapMcbsp;

    if (*mcbsp).st_data.is_null() {
        dev_warn((*mcbsp).dev, b"No sidetone data for port\n\0".as_ptr());
        return 0;
    }

    match port_id {
        2 => {
            snd_soc_add_dai_controls(
                cpu_dai as *const core::ffi::c_void,
                &omap_mcbsp2_st_controls,
                ARRAY_SIZE(&omap_mcbsp2_st_controls),
            )
        }
        3 => {
            snd_soc_add_dai_controls(
                cpu_dai as *const core::ffi::c_void,
                &omap_mcbsp3_st_controls,
                ARRAY_SIZE(&omap_mcbsp3_st_controls),
            )
        }
        _ => {
            dev_err(
                (*mcbsp).dev,
                b"Port %d not supported\n\0".as_ptr(),
                port_id,
            );
            -22 // -EINVAL
        }
    }
}

// EXPORT_SYMBOL_GPL(omap_mcbsp_st_add_controls);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
