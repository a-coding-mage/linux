// SPDX-License-Identifier: GPL-2.0-only
/* Industrialio event test code.
 *
 * Copyright (c) 2011-2012 Lars-Peter Clausen <lars@metafoo.de>
 *
 * This program is primarily intended as an example application.
 * Reads the current buffer setup from sysfs and starts a short capture
 * from the specified device, pretty printing the result after appropriate
 * conversion.
 *
 * Usage:
 *	iio_event_monitor <device_name>
 */

use std::ffi::CStr;
use std::mem;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

/* C includes translated as external dependencies:
 * unistd.h, stdlib.h, dirent.h, stdbool.h, stdio.h, errno.h, string.h,
 * poll.h, fcntl.h, sys/ioctl.h, iio_utils.h, linux/iio/events.h,
 * linux/iio/types.h
 */
use crate::*;

unsafe fn iio_chan_type_name_spec(idx: iio_chan_type) -> *const c_char {
    match idx {
        IIO_VOLTAGE => c"voltage".as_ptr(),
        IIO_CURRENT => c"current".as_ptr(),
        IIO_POWER => c"power".as_ptr(),
        IIO_ACCEL => c"accel".as_ptr(),
        IIO_ANGL_VEL => c"anglvel".as_ptr(),
        IIO_MAGN => c"magn".as_ptr(),
        IIO_LIGHT => c"illuminance".as_ptr(),
        IIO_INTENSITY => c"intensity".as_ptr(),
        IIO_PROXIMITY => c"proximity".as_ptr(),
        IIO_TEMP => c"temp".as_ptr(),
        IIO_INCLI => c"incli".as_ptr(),
        IIO_ROT => c"rot".as_ptr(),
        IIO_ANGL => c"angl".as_ptr(),
        IIO_TIMESTAMP => c"timestamp".as_ptr(),
        IIO_CAPACITANCE => c"capacitance".as_ptr(),
        IIO_ALTVOLTAGE => c"altvoltage".as_ptr(),
        IIO_CCT => c"cct".as_ptr(),
        IIO_PRESSURE => c"pressure".as_ptr(),
        IIO_HUMIDITYRELATIVE => c"humidityrelative".as_ptr(),
        IIO_ACTIVITY => c"activity".as_ptr(),
        IIO_STEPS => c"steps".as_ptr(),
        IIO_ENERGY => c"energy".as_ptr(),
        IIO_DISTANCE => c"distance".as_ptr(),
        IIO_VELOCITY => c"velocity".as_ptr(),
        IIO_CONCENTRATION => c"concentration".as_ptr(),
        IIO_RESISTANCE => c"resistance".as_ptr(),
        IIO_PH => c"ph".as_ptr(),
        IIO_UVINDEX => c"uvindex".as_ptr(),
        IIO_GRAVITY => c"gravity".as_ptr(),
        IIO_POSITIONRELATIVE => c"positionrelative".as_ptr(),
        IIO_PHASE => c"phase".as_ptr(),
        IIO_MASSCONCENTRATION => c"massconcentration".as_ptr(),
        IIO_DELTA_ANGL => c"deltaangl".as_ptr(),
        IIO_DELTA_VELOCITY => c"deltavelocity".as_ptr(),
        IIO_COLORTEMP => c"colortemp".as_ptr(),
        IIO_CHROMATICITY => c"chromaticity".as_ptr(),
        IIO_ATTENTION => c"attention".as_ptr(),
        IIO_ALTCURRENT => c"altcurrent".as_ptr(),
        IIO_COVERAGE => c"coverage".as_ptr(),
        IIO_VOLUMEFLOW => c"volumeflow".as_ptr(),
        _ => ptr::null(),
    }
}

unsafe fn iio_ev_type_text(idx: iio_event_type) -> *const c_char {
    match idx {
        IIO_EV_TYPE_THRESH => c"thresh".as_ptr(),
        IIO_EV_TYPE_MAG => c"mag".as_ptr(),
        IIO_EV_TYPE_ROC => c"roc".as_ptr(),
        IIO_EV_TYPE_THRESH_ADAPTIVE => c"thresh_adaptive".as_ptr(),
        IIO_EV_TYPE_MAG_ADAPTIVE => c"mag_adaptive".as_ptr(),
        IIO_EV_TYPE_CHANGE => c"change".as_ptr(),
        IIO_EV_TYPE_MAG_REFERENCED => c"mag_referenced".as_ptr(),
        IIO_EV_TYPE_GESTURE => c"gesture".as_ptr(),
        IIO_EV_TYPE_FAULT => c"fault".as_ptr(),
        _ => ptr::null(),
    }
}

unsafe fn iio_ev_dir_text(idx: iio_event_direction) -> *const c_char {
    match idx {
        IIO_EV_DIR_EITHER => c"either".as_ptr(),
        IIO_EV_DIR_RISING => c"rising".as_ptr(),
        IIO_EV_DIR_FALLING => c"falling".as_ptr(),
        IIO_EV_DIR_SINGLETAP => c"singletap".as_ptr(),
        IIO_EV_DIR_DOUBLETAP => c"doubletap".as_ptr(),
        IIO_EV_DIR_FAULT_OPENWIRE => c"openwire".as_ptr(),
        _ => ptr::null(),
    }
}

unsafe fn iio_modifier_names(idx: iio_modifier) -> *const c_char {
    match idx {
        IIO_MOD_X => c"x".as_ptr(),
        IIO_MOD_Y => c"y".as_ptr(),
        IIO_MOD_Z => c"z".as_ptr(),
        IIO_MOD_X_AND_Y => c"x&y".as_ptr(),
        IIO_MOD_X_AND_Z => c"x&z".as_ptr(),
        IIO_MOD_Y_AND_Z => c"y&z".as_ptr(),
        IIO_MOD_X_AND_Y_AND_Z => c"x&y&z".as_ptr(),
        IIO_MOD_X_OR_Y => c"x|y".as_ptr(),
        IIO_MOD_X_OR_Z => c"x|z".as_ptr(),
        IIO_MOD_Y_OR_Z => c"y|z".as_ptr(),
        IIO_MOD_X_OR_Y_OR_Z => c"x|y|z".as_ptr(),
        IIO_MOD_LIGHT_BOTH => c"both".as_ptr(),
        IIO_MOD_LIGHT_IR => c"ir".as_ptr(),
        IIO_MOD_ROOT_SUM_SQUARED_X_Y => c"sqrt(x^2+y^2)".as_ptr(),
        IIO_MOD_SUM_SQUARED_X_Y_Z => c"x^2+y^2+z^2".as_ptr(),
        IIO_MOD_LIGHT_CLEAR => c"clear".as_ptr(),
        IIO_MOD_LIGHT_RED => c"red".as_ptr(),
        IIO_MOD_LIGHT_GREEN => c"green".as_ptr(),
        IIO_MOD_LIGHT_BLUE => c"blue".as_ptr(),
        IIO_MOD_LIGHT_UV => c"uv".as_ptr(),
        IIO_MOD_LIGHT_UVA => c"uva".as_ptr(),
        IIO_MOD_LIGHT_UVB => c"uvb".as_ptr(),
        IIO_MOD_LIGHT_DUV => c"duv".as_ptr(),
        IIO_MOD_QUATERNION => c"quaternion".as_ptr(),
        IIO_MOD_TEMP_AMBIENT => c"ambient".as_ptr(),
        IIO_MOD_TEMP_OBJECT => c"object".as_ptr(),
        IIO_MOD_NORTH_MAGN => c"from_north_magnetic".as_ptr(),
        IIO_MOD_NORTH_TRUE => c"from_north_true".as_ptr(),
        IIO_MOD_NORTH_MAGN_TILT_COMP => c"from_north_magnetic_tilt_comp".as_ptr(),
        IIO_MOD_NORTH_TRUE_TILT_COMP => c"from_north_true_tilt_comp".as_ptr(),
        IIO_MOD_RUNNING => c"running".as_ptr(),
        IIO_MOD_JOGGING => c"jogging".as_ptr(),
        IIO_MOD_WALKING => c"walking".as_ptr(),
        IIO_MOD_STILL => c"still".as_ptr(),
        IIO_MOD_ROOT_SUM_SQUARED_X_Y_Z => c"sqrt(x^2+y^2+z^2)".as_ptr(),
        IIO_MOD_I => c"i".as_ptr(),
        IIO_MOD_Q => c"q".as_ptr(),
        IIO_MOD_CO2 => c"co2".as_ptr(),
        IIO_MOD_ETHANOL => c"ethanol".as_ptr(),
        IIO_MOD_H2 => c"h2".as_ptr(),
        IIO_MOD_VOC => c"voc".as_ptr(),
        IIO_MOD_PM1 => c"pm1".as_ptr(),
        IIO_MOD_PM2P5 => c"pm2p5".as_ptr(),
        IIO_MOD_PM4 => c"pm4".as_ptr(),
        IIO_MOD_PM10 => c"pm10".as_ptr(),
        IIO_MOD_O2 => c"o2".as_ptr(),
        IIO_MOD_LINEAR_X => c"linear_x".as_ptr(),
        IIO_MOD_LINEAR_Y => c"linear_y".as_ptr(),
        IIO_MOD_LINEAR_Z => c"linear_z".as_ptr(),
        IIO_MOD_PITCH => c"pitch".as_ptr(),
        IIO_MOD_YAW => c"yaw".as_ptr(),
        IIO_MOD_ROLL => c"roll".as_ptr(),
        IIO_MOD_RMS => c"rms".as_ptr(),
        IIO_MOD_ACTIVE => c"active".as_ptr(),
        IIO_MOD_REACTIVE => c"reactive".as_ptr(),
        IIO_MOD_APPARENT => c"apparent".as_ptr(),
        IIO_MOD_QUATERNION_AXIS => c"quaternionaxis".as_ptr(),
        _ => ptr::null(),
    }
}

unsafe fn event_is_known(event: *mut iio_event_data) -> bool {
    let type_: iio_chan_type = IIO_EVENT_CODE_EXTRACT_CHAN_TYPE((*event).id);
    let mod_: iio_modifier = IIO_EVENT_CODE_EXTRACT_MODIFIER((*event).id);
    let ev_type: iio_event_type = IIO_EVENT_CODE_EXTRACT_TYPE((*event).id);
    let dir: iio_event_direction = IIO_EVENT_CODE_EXTRACT_DIR((*event).id);

    match type_ {
        IIO_VOLTAGE | IIO_CURRENT | IIO_POWER | IIO_ACCEL | IIO_ANGL_VEL | IIO_MAGN
        | IIO_LIGHT | IIO_INTENSITY | IIO_PROXIMITY | IIO_TEMP | IIO_INCLI | IIO_ROT
        | IIO_ANGL | IIO_TIMESTAMP | IIO_CAPACITANCE | IIO_ALTVOLTAGE | IIO_CCT
        | IIO_PRESSURE | IIO_HUMIDITYRELATIVE | IIO_ACTIVITY | IIO_STEPS | IIO_ENERGY
        | IIO_DISTANCE | IIO_VELOCITY | IIO_CONCENTRATION | IIO_RESISTANCE | IIO_PH
        | IIO_UVINDEX | IIO_GRAVITY | IIO_POSITIONRELATIVE | IIO_PHASE
        | IIO_MASSCONCENTRATION | IIO_DELTA_ANGL | IIO_DELTA_VELOCITY | IIO_COLORTEMP
        | IIO_CHROMATICITY | IIO_ATTENTION | IIO_ALTCURRENT | IIO_COVERAGE
        | IIO_VOLUMEFLOW => {}
        _ => return false,
    }

    match mod_ {
        IIO_NO_MOD | IIO_MOD_X | IIO_MOD_Y | IIO_MOD_Z | IIO_MOD_X_AND_Y
        | IIO_MOD_X_AND_Z | IIO_MOD_Y_AND_Z | IIO_MOD_X_AND_Y_AND_Z | IIO_MOD_X_OR_Y
        | IIO_MOD_X_OR_Z | IIO_MOD_Y_OR_Z | IIO_MOD_X_OR_Y_OR_Z | IIO_MOD_LIGHT_BOTH
        | IIO_MOD_LIGHT_IR | IIO_MOD_ROOT_SUM_SQUARED_X_Y | IIO_MOD_SUM_SQUARED_X_Y_Z
        | IIO_MOD_LIGHT_CLEAR | IIO_MOD_LIGHT_RED | IIO_MOD_LIGHT_GREEN | IIO_MOD_LIGHT_BLUE
        | IIO_MOD_LIGHT_UV | IIO_MOD_LIGHT_DUV | IIO_MOD_QUATERNION | IIO_MOD_TEMP_AMBIENT
        | IIO_MOD_TEMP_OBJECT | IIO_MOD_NORTH_MAGN | IIO_MOD_NORTH_TRUE
        | IIO_MOD_NORTH_MAGN_TILT_COMP | IIO_MOD_NORTH_TRUE_TILT_COMP | IIO_MOD_RUNNING
        | IIO_MOD_JOGGING | IIO_MOD_WALKING | IIO_MOD_STILL | IIO_MOD_ROOT_SUM_SQUARED_X_Y_Z
        | IIO_MOD_I | IIO_MOD_Q | IIO_MOD_CO2 | IIO_MOD_ETHANOL | IIO_MOD_H2 | IIO_MOD_VOC
        | IIO_MOD_PM1 | IIO_MOD_PM2P5 | IIO_MOD_PM4 | IIO_MOD_PM10 | IIO_MOD_O2
        | IIO_MOD_RMS | IIO_MOD_ACTIVE | IIO_MOD_REACTIVE | IIO_MOD_APPARENT => {}
        _ => return false,
    }

    match ev_type {
        IIO_EV_TYPE_THRESH | IIO_EV_TYPE_MAG | IIO_EV_TYPE_ROC | IIO_EV_TYPE_THRESH_ADAPTIVE
        | IIO_EV_TYPE_MAG_ADAPTIVE | IIO_EV_TYPE_CHANGE | IIO_EV_TYPE_GESTURE
        | IIO_EV_TYPE_FAULT => {}
        _ => return false,
    }

    match dir {
        IIO_EV_DIR_EITHER | IIO_EV_DIR_RISING | IIO_EV_DIR_FALLING | IIO_EV_DIR_SINGLETAP
        | IIO_EV_DIR_DOUBLETAP | IIO_EV_DIR_FAULT_OPENWIRE | IIO_EV_DIR_NONE => {}
        _ => return false,
    }

    true
}

unsafe fn print_event(event: *mut iio_event_data) {
    let type_: iio_chan_type = IIO_EVENT_CODE_EXTRACT_CHAN_TYPE((*event).id);
    let mod_: iio_modifier = IIO_EVENT_CODE_EXTRACT_MODIFIER((*event).id);
    let ev_type: iio_event_type = IIO_EVENT_CODE_EXTRACT_TYPE((*event).id);
    let dir: iio_event_direction = IIO_EVENT_CODE_EXTRACT_DIR((*event).id);
    let chan: c_int = IIO_EVENT_CODE_EXTRACT_CHAN((*event).id);
    let chan2: c_int = IIO_EVENT_CODE_EXTRACT_CHAN2((*event).id);
    let diff: bool = IIO_EVENT_CODE_EXTRACT_DIFF((*event).id);

    if !event_is_known(event) {
        fprintf(
            stderr,
            c"Unknown event: time: %lld, id: %llx\n".as_ptr(),
            (*event).timestamp,
            (*event).id,
        );

        return;
    }

    printf(
        c"Event: time: %lld, type: %s".as_ptr(),
        (*event).timestamp,
        iio_chan_type_name_spec(type_),
    );

    if mod_ != IIO_NO_MOD {
        printf(c"(%s)".as_ptr(), iio_modifier_names(mod_));
    }

    if chan >= 0 {
        printf(c", channel: %d".as_ptr(), chan);
        if diff && chan2 >= 0 {
            printf(c"-%d".as_ptr(), chan2);
        }
    }

    printf(c", evtype: %s".as_ptr(), iio_ev_type_text(ev_type));

    if dir != IIO_EV_DIR_NONE {
        printf(c", direction: %s".as_ptr(), iio_ev_dir_text(dir));
    }

    printf(c"\n".as_ptr());
    fflush(stdout);
}

/* Enable or disable events in sysfs if the knob is available */
unsafe fn enable_events(dev_dir: *mut c_char, enable: c_int) {
    let mut ent: *const dirent;
    let mut evdir: [c_char; 256] = [0; 256];
    let mut ret: c_int;
    let mut dp: *mut DIR;

    snprintf(
        evdir.as_mut_ptr(),
        mem::size_of_val(&evdir),
        FORMAT_EVENTS_DIR,
        dev_dir,
    );
    evdir[mem::size_of_val(&evdir) - 1] = 0;

    dp = opendir(evdir.as_ptr());
    if dp.is_null() {
        fprintf(
            stderr,
            c"Enabling/disabling events: can't open %s\n".as_ptr(),
            evdir.as_ptr(),
        );
        return;
    }

    loop {
        ent = readdir(dp);
        if ent.is_null() {
            break;
        }
        if iioutils_check_suffix((*ent).d_name.as_ptr(), c"_en".as_ptr()) {
            printf(
                c"%sabling: %s\n".as_ptr(),
                if enable != 0 {
                    c"En".as_ptr()
                } else {
                    c"Dis".as_ptr()
                },
                (*ent).d_name.as_ptr(),
            );
            ret = write_sysfs_int((*ent).d_name.as_ptr(), evdir.as_ptr(), enable);
            if ret < 0 {
                fprintf(
                    stderr,
                    c"Failed to enable/disable %s\n".as_ptr(),
                    (*ent).d_name.as_ptr(),
                );
            }
        }
    }

    if closedir(dp) == -1 {
        perror(c"Enabling/disabling channels: Failed to close directory".as_ptr());
        return;
    }
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut event: iio_event_data = mem::zeroed();
    let mut device_name: *const c_char;
    let mut dev_dir_name: *mut c_char = ptr::null_mut();
    let mut chrdev_name: *mut c_char;
    let mut ret: c_int;
    let mut dev_num: c_int;
    let mut fd: c_int;
    let mut event_fd: c_int = 0;
    let mut all_events: bool = false;

    if argc == 2 {
        device_name = *argv.add(1);
    } else if argc == 3 {
        device_name = *argv.add(2);
        if strcmp(*argv.add(1), c"-a".as_ptr()) == 0 {
            all_events = true;
        }
    } else {
        fprintf(
            stderr,
            c"Usage: iio_event_monitor [options] <device_name>\nListen and display events from IIO devices\n  -a         Auto-activate all available events\n".as_ptr(),
        );
        return -1;
    }

    dev_num = find_type_by_name(device_name, c"iio:device".as_ptr());
    if dev_num >= 0 {
        printf(
            c"Found IIO device with name %s with device number %d\n".as_ptr(),
            device_name,
            dev_num,
        );
        ret = asprintf(
            &mut chrdev_name,
            c"/dev/iio:device%d".as_ptr(),
            dev_num,
        );
        if ret < 0 {
            return -ENOMEM;
        }
        /* Look up sysfs dir as well if we can */
        ret = asprintf(
            &mut dev_dir_name,
            c"%siio:device%d".as_ptr(),
            iio_dir,
            dev_num,
        );
        if ret < 0 {
            return -ENOMEM;
        }
    } else {
        /*
         * If we can't find an IIO device by name assume device_name is
         * an IIO chrdev
         */
        chrdev_name = strdup(device_name);
        if chrdev_name.is_null() {
            return -ENOMEM;
        }
    }

    if all_events && !dev_dir_name.is_null() {
        enable_events(dev_dir_name, 1);
    }

    fd = open(chrdev_name, 0);
    if fd == -1 {
        ret = -errno();
        fprintf(stderr, c"Failed to open %s\n".as_ptr(), chrdev_name);
        goto_error_free_chrdev_name(ret, chrdev_name, dev_dir_name, all_events);
        return ret;
    }

    ret = ioctl(fd, IIO_GET_EVENT_FD_IOCTL, &mut event_fd);
    if ret == -1 || event_fd == -1 {
        ret = -errno();
        if ret == -ENODEV {
            fprintf(stderr, c"This device does not support events\n".as_ptr());
        } else {
            fprintf(stderr, c"Failed to retrieve event fd\n".as_ptr());
        }
        if close(fd) == -1 {
            perror(c"Failed to close character device file".as_ptr());
        }

        goto_error_free_chrdev_name(ret, chrdev_name, dev_dir_name, all_events);
        return ret;
    }

    if close(fd) == -1 {
        ret = -errno();
        goto_error_free_chrdev_name(ret, chrdev_name, dev_dir_name, all_events);
        return ret;
    }

    loop {
        ret = read(
            event_fd,
            &mut event as *mut iio_event_data as *mut c_void,
            mem::size_of_val(&event),
        ) as c_int;
        if ret == -1 {
            if errno() == EAGAIN {
                fprintf(stderr, c"nothing available\n".as_ptr());
                continue;
            } else {
                ret = -errno();
                perror(c"Failed to read event from device".as_ptr());
                break;
            }
        }

        if ret as usize != mem::size_of_val(&event) {
            fprintf(stderr, c"Reading event failed!\n".as_ptr());
            ret = -EIO;
            break;
        }

        print_event(&mut event);
    }

    if close(event_fd) == -1 {
        perror(c"Failed to close event file".as_ptr());
    }

    /* Disable events after use */
    if all_events && !dev_dir_name.is_null() {
        enable_events(dev_dir_name, 0);
    }

    free(chrdev_name as *mut c_void);
    free(dev_dir_name as *mut c_void);

    ret
}

unsafe fn goto_error_free_chrdev_name(
    ret: c_int,
    chrdev_name: *mut c_char,
    dev_dir_name: *mut c_char,
    all_events: bool,
) {
    /* Disable events after use */
    if all_events && !dev_dir_name.is_null() {
        enable_events(dev_dir_name, 0);
    }

    free(chrdev_name as *mut c_void);
    free(dev_dir_name as *mut c_void);
}
