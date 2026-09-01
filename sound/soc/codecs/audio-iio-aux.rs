// SPDX-License-Identifier: GPL-2.0-only
//
// ALSA SoC glue to use IIO devices as audio components
//
// Copyright 2023 CS GROUP France
//
// Author: Herve Codina <herve.codina@bootlin.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type bool_ = bool;
type u32 = c_uint;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 0;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SND_SOC_NOPM: c_int = 0;

#[repr(C)]
pub struct iio_channel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: i64,
    pub max: i64,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct audio_iio_aux_chan {
    pub iio_chan: *mut iio_channel,
    pub name: *const c_char,
    pub max: c_int,
    pub min: c_int,
    pub is_invert_range: bool_,
}

#[repr(C)]
pub struct audio_iio_aux {
    pub dev: *mut device,
    pub num_chans: c_uint,
    pub chans: [audio_iio_aux_chan; 0],
}

unsafe extern "C" {
    fn iio_read_channel_raw(chan: *mut iio_channel, val: *mut c_int) -> c_int;
    fn iio_write_channel_raw(chan: *mut iio_channel, val: c_int) -> c_int;
    fn iio_read_max_channel_raw(chan: *mut iio_channel, val: *mut c_int) -> c_int;
    fn iio_read_min_channel_raw(chan: *mut iio_channel, val: *mut c_int) -> c_int;
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widgets: *const snd_soc_dapm_widget,
        num: c_int,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        routes: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn kasprintf(gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn kfree(ptr: *const c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn kcalloc(n: usize, size: usize, gfp: c_uint) -> *mut c_void;
    fn device_property_string_array_count(dev: *mut device, propname: *const c_char) -> c_int;
    fn device_property_read_string_array(
        dev: *mut device,
        propname: *const c_char,
        val: *mut *const c_char,
        nval: usize,
    ) -> c_int;
    fn device_property_count_u32(dev: *mut device, propname: *const c_char) -> c_int;
    fn device_property_read_u32_array(
        dev: *mut device,
        propname: *const c_char,
        val: *mut u32,
        nval: usize,
    ) -> c_int;
    fn devm_iio_channel_get(dev: *mut device, consumer_channel: *const c_char) -> *mut iio_channel;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
}

unsafe extern "C" fn audio_iio_aux_info_volsw(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let chan = (*kcontrol).private_value as *mut audio_iio_aux_chan;

    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = ((*chan).max - (*chan).min) as i64;
    (*uinfo).type_ = if (*uinfo).value.integer.max == 1 {
        SNDRV_CTL_ELEM_TYPE_BOOLEAN
    } else {
        SNDRV_CTL_ELEM_TYPE_INTEGER
    };
    0
}

unsafe extern "C" fn audio_iio_aux_get_volsw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chan = (*kcontrol).private_value as *mut audio_iio_aux_chan;
    let max = (*chan).max;
    let min = (*chan).min;
    let invert_range = (*chan).is_invert_range;
    let mut val: c_int = 0;

    let ret = iio_read_channel_raw((*chan).iio_chan, &mut val);
    if ret < 0 {
        return ret;
    }

    (*ucontrol).value.integer.value[0] = (val - min) as i64;
    if invert_range {
        (*ucontrol).value.integer.value[0] = (max as i64) - (*ucontrol).value.integer.value[0];
    }

    0
}

unsafe extern "C" fn audio_iio_aux_put_volsw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chan = (*kcontrol).private_value as *mut audio_iio_aux_chan;
    let max = (*chan).max;
    let min = (*chan).min;
    let invert_range = (*chan).is_invert_range;
    let mut tmp: c_int = 0;

    let mut val = (*ucontrol).value.integer.value[0] as c_int;
    if val < 0 {
        return -EINVAL;
    }
    if val > max - min {
        return -EINVAL;
    }

    val = val + min;
    if invert_range {
        val = max - val;
    }

    let mut ret = iio_read_channel_raw((*chan).iio_chan, &mut tmp);
    if ret < 0 {
        return ret;
    }

    if tmp == val {
        return 0;
    }

    ret = iio_write_channel_raw((*chan).iio_chan, val);
    if ret != 0 {
        return ret;
    }

    1 /* The value changed */
}

unsafe extern "C" fn audio_iio_aux_add_controls(
    component: *mut snd_soc_component,
    chan: *mut audio_iio_aux_chan,
) -> c_int {
    let control = snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: (*chan).name,
        info: Some(audio_iio_aux_info_volsw),
        get: Some(audio_iio_aux_get_volsw),
        put: Some(audio_iio_aux_put_volsw),
        private_value: chan as c_ulong,
    };

    snd_soc_add_component_controls(component, &control, 1)
}

/*
 * These data could be on stack but they are pretty big.
 * As ASoC internally copy them and protect them against concurrent accesses
 * (snd_soc_bind_card() protects using client_mutex), keep them in the global
 * data area.
 */
static mut widgets: [snd_soc_dapm_widget; 3] = [snd_soc_dapm_widget { name: ptr::null() }; 3];
static mut routes: [snd_soc_dapm_route; 2] = [snd_soc_dapm_route {
    sink: ptr::null(),
    control: ptr::null(),
    source: ptr::null(),
}; 2];

const _: [(); 3] = [(); mem::size_of::<[snd_soc_dapm_widget; 3]>() / mem::size_of::<snd_soc_dapm_widget>()];
const _: [(); 2] = [(); mem::size_of::<[snd_soc_dapm_route; 2]>() / mem::size_of::<snd_soc_dapm_route>()];

unsafe fn SND_SOC_DAPM_INPUT(name: *const c_char) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget { name }
}

unsafe fn SND_SOC_DAPM_OUTPUT(name: *const c_char) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget { name }
}

unsafe fn SND_SOC_DAPM_PGA(
    name: *const c_char,
    _reg: c_int,
    _shift: c_int,
    _invert: c_int,
    _controls: *const c_void,
    _num_controls: c_int,
) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget { name }
}

unsafe extern "C" fn audio_iio_aux_add_dapms(
    component: *mut snd_soc_component,
    chan: *mut audio_iio_aux_chan,
) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);

    /* Allocated names are not needed afterwards (duplicated in ASoC internals) */
    let input_name = kasprintf(GFP_KERNEL, c"%s IN".as_ptr(), (*chan).name);
    if input_name.is_null() {
        return -ENOMEM;
    }

    let output_name = kasprintf(GFP_KERNEL, c"%s OUT".as_ptr(), (*chan).name);
    if output_name.is_null() {
        kfree(input_name as *const c_void);
        return -ENOMEM;
    }

    let pga_name = kasprintf(GFP_KERNEL, c"%s PGA".as_ptr(), (*chan).name);
    if pga_name.is_null() {
        kfree(output_name as *const c_void);
        kfree(input_name as *const c_void);
        return -ENOMEM;
    }

    widgets[0] = SND_SOC_DAPM_INPUT(input_name);
    widgets[1] = SND_SOC_DAPM_OUTPUT(output_name);
    widgets[2] = SND_SOC_DAPM_PGA(pga_name, SND_SOC_NOPM, 0, 0, ptr::null(), 0);
    let ret = snd_soc_dapm_new_controls(dapm, widgets.as_ptr(), 3);
    if ret != 0 {
        kfree(pga_name as *const c_void);
        kfree(output_name as *const c_void);
        kfree(input_name as *const c_void);
        return ret;
    }

    routes[0].sink = pga_name;
    routes[0].control = ptr::null();
    routes[0].source = input_name;
    routes[1].sink = output_name;
    routes[1].control = ptr::null();
    routes[1].source = pga_name;

    let ret = snd_soc_dapm_add_routes(dapm, routes.as_ptr(), 2);
    kfree(pga_name as *const c_void);
    kfree(output_name as *const c_void);
    kfree(input_name as *const c_void);
    ret
}

unsafe extern "C" fn audio_iio_aux_component_probe(component: *mut snd_soc_component) -> c_int {
    let iio_aux = snd_soc_component_get_drvdata(component) as *mut audio_iio_aux;
    let mut i: c_int = 0;

    while i < (*iio_aux).num_chans as c_int {
        let chan = (*iio_aux).chans.as_mut_ptr().offset(i as isize);

        let mut ret = iio_read_max_channel_raw((*chan).iio_chan, &mut (*chan).max);
        if ret != 0 {
            return dev_err_probe(
                (*component).dev,
                ret,
                c"chan[%d] %s: Cannot get max raw value\n".as_ptr(),
                i,
                (*chan).name,
            );
        }

        ret = iio_read_min_channel_raw((*chan).iio_chan, &mut (*chan).min);
        if ret != 0 {
            return dev_err_probe(
                (*component).dev,
                ret,
                c"chan[%d] %s: Cannot get min raw value\n".as_ptr(),
                i,
                (*chan).name,
            );
        }

        if (*chan).min > (*chan).max {
            /*
             * This should never happen but to avoid any check
             * later, just swap values here to ensure that the
             * minimum value is lower than the maximum value.
             */
            dev_dbg(
                (*component).dev,
                c"chan[%d] %s: Swap min and max\n".as_ptr(),
                i,
                (*chan).name,
            );
            mem::swap(&mut (*chan).min, &mut (*chan).max);
        }

        /* Set initial value */
        ret = iio_write_channel_raw(
            (*chan).iio_chan,
            if (*chan).is_invert_range {
                (*chan).max
            } else {
                (*chan).min
            },
        );
        if ret != 0 {
            return dev_err_probe(
                (*component).dev,
                ret,
                c"chan[%d] %s: Cannot set initial value\n".as_ptr(),
                i,
                (*chan).name,
            );
        }

        ret = audio_iio_aux_add_controls(component, chan);
        if ret != 0 {
            return ret;
        }

        ret = audio_iio_aux_add_dapms(component, chan);
        if ret != 0 {
            return ret;
        }

        dev_dbg(
            (*component).dev,
            c"chan[%d]: Added %s (min=%d, max=%d, invert=%s)\n".as_ptr(),
            i,
            (*chan).name,
            (*chan).min,
            (*chan).max,
            str_on_off((*chan).is_invert_range),
        );

        i += 1;
    }

    0
}

static audio_iio_aux_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(audio_iio_aux_component_probe),
};

unsafe fn struct_size_audio_iio_aux_chans(count: c_int) -> usize {
    mem::size_of::<audio_iio_aux>()
        + (count as usize).wrapping_mul(mem::size_of::<audio_iio_aux_chan>())
}

unsafe fn min_t_unsigned_int(a: c_int, b: c_uint) -> c_int {
    let aa = a as c_uint;
    let bb = b;
    if aa < bb {
        aa as c_int
    } else {
        bb as c_int
    }
}

unsafe fn str_on_off(v: bool_) -> *const c_char {
    if v {
        c"on".as_ptr()
    } else {
        c"off".as_ptr()
    }
}

unsafe extern "C" fn audio_iio_aux_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;

    let mut count = device_property_string_array_count(dev, c"io-channel-names".as_ptr());
    if count < 0 {
        return dev_err_probe(
            dev,
            count,
            c"failed to count io-channel-names\n".as_ptr(),
        );
    }

    let iio_aux = devm_kzalloc(dev, struct_size_audio_iio_aux_chans(count), GFP_KERNEL)
        as *mut audio_iio_aux;
    if iio_aux.is_null() {
        return -ENOMEM;
    }

    (*iio_aux).dev = dev;

    (*iio_aux).num_chans = count as c_uint;

    let names = kcalloc(
        (*iio_aux).num_chans as usize,
        mem::size_of::<*const c_char>(),
        GFP_KERNEL,
    ) as *mut *const c_char;
    if names.is_null() {
        return -ENOMEM;
    }

    let invert_ranges = kcalloc(
        (*iio_aux).num_chans as usize,
        mem::size_of::<u32>(),
        GFP_KERNEL,
    ) as *mut u32;
    if invert_ranges.is_null() {
        kfree(names as *const c_void);
        return -ENOMEM;
    }

    let mut ret = device_property_read_string_array(
        dev,
        c"io-channel-names".as_ptr(),
        names,
        (*iio_aux).num_chans as usize,
    );
    if ret < 0 {
        kfree(invert_ranges as *const c_void);
        kfree(names as *const c_void);
        return dev_err_probe(dev, ret, c"failed to read io-channel-names\n".as_ptr());
    }

    /*
     * snd-control-invert-range is optional and can contain fewer items
     * than the number of channels. Unset values default to 0.
     */
    count = device_property_count_u32(dev, c"snd-control-invert-range".as_ptr());
    if count > 0 {
        count = min_t_unsigned_int(count, (*iio_aux).num_chans);
        ret = device_property_read_u32_array(
            dev,
            c"snd-control-invert-range".as_ptr(),
            invert_ranges,
            count as usize,
        );
        if ret < 0 {
            kfree(invert_ranges as *const c_void);
            kfree(names as *const c_void);
            return dev_err_probe(
                dev,
                ret,
                c"failed to read snd-control-invert-range\n".as_ptr(),
            );
        }
    }

    let mut i: c_int = 0;
    while i < (*iio_aux).num_chans as c_int {
        let iio_aux_chan = (*iio_aux).chans.as_mut_ptr().offset(i as isize);
        (*iio_aux_chan).name = *names.offset(i as isize);
        (*iio_aux_chan).is_invert_range = *invert_ranges.offset(i as isize) != 0;

        (*iio_aux_chan).iio_chan = devm_iio_channel_get(dev, (*iio_aux_chan).name);
        if IS_ERR((*iio_aux_chan).iio_chan as *const c_void) {
            let err = PTR_ERR((*iio_aux_chan).iio_chan as *const c_void);
            kfree(invert_ranges as *const c_void);
            kfree(names as *const c_void);
            return dev_err_probe(
                dev,
                err,
                c"get IIO channel '%s' failed\n".as_ptr(),
                (*iio_aux_chan).name,
            );
        }

        i += 1;
    }

    platform_set_drvdata(pdev, iio_aux as *mut c_void);

    kfree(invert_ranges as *const c_void);
    kfree(names as *const c_void);

    devm_snd_soc_register_component(
        dev,
        &audio_iio_aux_component_driver,
        ptr::null_mut(),
        0,
    )
}

static audio_iio_aux_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: c"audio-iio-aux".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, audio_iio_aux_ids); */

static mut audio_iio_aux_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"audio-iio-aux".as_ptr(),
        of_match_table: audio_iio_aux_ids.as_ptr(),
    },
    probe: Some(audio_iio_aux_probe),
};
/* module_platform_driver(audio_iio_aux_driver); */

/* MODULE_AUTHOR("Herve Codina <herve.codina@bootlin.com>"); */
/* MODULE_DESCRIPTION("IIO ALSA SoC aux driver"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_IMPORT_NS("IIO_CONSUMER"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
