// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sysfs.c sysfs ABI access functions for TMON program
 *
 * Copyright (C) 2013 Intel Corporation. All rights reserved.
 *
 * Author: Jacob Pan <jacob.jun.pan@linux.intel.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type FILE = c_void;
type DIR = c_void;

#[repr(C)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct dirent {
    pub d_ino: c_ulong,
    pub d_off: c_long,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct thermal_trip_point {
    pub type_: c_int,
    pub temp: c_ulong,
}

#[repr(C)]
pub struct tz_info {
    pub type_: [c_char; 256],
    pub instance: c_int,
    pub nr_cdev: c_int,
    pub nr_trip_pts: c_int,
    pub cdev_binding: c_ulong,
    pub trip_binding: *mut c_ulong,
    pub tp: *mut thermal_trip_point,
}

#[repr(C)]
pub struct cdev_info {
    pub type_: [c_char; 256],
    pub max_state: c_ulong,
    pub cur_state: c_ulong,
    pub instance: c_int,
    pub flag: c_int,
}

#[repr(C)]
pub struct tmon_platform_data {
    pub tzi: *mut tz_info,
    pub cdi: *mut cdev_info,
    pub nr_tz_sensor: c_int,
    pub nr_cooling_dev: c_int,
    pub max_tz_instance: c_int,
    pub max_cdev_instance: c_int,
}

#[repr(C)]
pub struct thermal_data_record {
    pub tv: timeval,
    pub temp: *mut c_ulong,
}

#[repr(C)]
pub struct pid_param {
    pub t_target: f32,
}

const PATH_MAX: usize = 4096;
const NR_THERMAL_RECORDS: usize = 3;

const LOG_ERR: c_int = 3;
const LOG_INFO: c_int = 6;
const LOG_DEBUG: c_int = 7;
const DT_LNK: u8 = 10;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;

/* Constants and globals supplied by tmon.h in the original program. */
extern "C" {
    static THERMAL_SYSFS: *const c_char;
    static TZONE: *const c_char;
    static CDEV: *const c_char;
    static NR_THERMAL_TRIP_TYPE: c_int;
    static MAX_NR_TRIP: c_int;
    static MAX_TEMP_KC: c_ulong;
    static CDEV_FLAG_IN_CONTROL: c_int;
    static mut ctrl_cdev: *mut c_char;
    static mut target_thermal_zone: c_int;
    static mut tmon_log: *mut FILE;
    static mut p_param: pid_param;
    static mut no_control: c_int;

    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn syslog(priority: c_int, format: *const c_char, ...);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strtok(str: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn atol(nptr: *const c_char) -> c_long;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn scandir(
        dirp: *const c_char,
        namelist: *mut *mut *mut dirent,
        filter: Option<unsafe extern "C" fn(*const dirent) -> c_int>,
        compar: Option<unsafe extern "C" fn(*const *const dirent, *const *const dirent) -> c_int>,
    ) -> c_int;
    fn alphasort(a: *const *const dirent, b: *const *const dirent) -> c_int;
    fn free(ptr: *mut c_void);
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn readlink(pathname: *const c_char, buf: *mut c_char, bufsiz: usize) -> isize;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    static mut stderr: *mut FILE;
    fn fflush(stream: *mut FILE) -> c_int;
}

#[no_mangle]
pub static mut ptdata: tmon_platform_data = tmon_platform_data {
    tzi: ptr::null_mut(),
    cdi: ptr::null_mut(),
    nr_tz_sensor: 0,
    nr_cooling_dev: 0,
    max_tz_instance: 0,
    max_cdev_instance: 0,
};

#[no_mangle]
pub static trip_type_name: [*const c_char; 4] = [
    b"critical\0".as_ptr() as *const c_char,
    b"hot\0".as_ptr() as *const c_char,
    b"passive\0".as_ptr() as *const c_char,
    b"active\0".as_ptr() as *const c_char,
];

#[no_mangle]
pub static mut trec: [thermal_data_record; NR_THERMAL_RECORDS] = [
    thermal_data_record { tv: timeval { tv_sec: 0, tv_usec: 0 }, temp: ptr::null_mut() },
    thermal_data_record { tv: timeval { tv_sec: 0, tv_usec: 0 }, temp: ptr::null_mut() },
    thermal_data_record { tv: timeval { tv_sec: 0, tv_usec: 0 }, temp: ptr::null_mut() },
];

#[no_mangle]
pub static mut cur_thermal_record: c_int = 0; /* index to the trec array */

#[no_mangle]
pub unsafe extern "C" fn sysfs_set_ulong(
    path: *mut c_char,
    filename: *mut c_char,
    val: c_ulong,
) -> c_int {
    let mut ret: c_int = -1;
    let mut filepath = [0 as c_char; PATH_MAX + 2]; /* NUL and '/' */

    snprintf(filepath.as_mut_ptr(), filepath.len(), b"%s/%s\0".as_ptr() as *const c_char, path, filename);

    let fd = fopen(filepath.as_ptr(), b"w\0".as_ptr() as *const c_char);
    if fd.is_null() {
        syslog(LOG_ERR, b"Err: open %s: %s\n\0".as_ptr() as *const c_char, b"sysfs_set_ulong\0".as_ptr() as *const c_char, filepath.as_ptr());
        return ret;
    }
    ret = fprintf(fd, b"%lu\0".as_ptr() as *const c_char, val);
    fclose(fd);

    0
}

unsafe extern "C" fn sysfs_get_ulong(
    path: *mut c_char,
    filename: *mut c_char,
    p_ulong: *mut c_ulong,
) -> c_int {
    let mut ret: c_int = -1;
    let mut filepath = [0 as c_char; PATH_MAX + 2]; /* NUL and '/' */

    snprintf(filepath.as_mut_ptr(), filepath.len(), b"%s/%s\0".as_ptr() as *const c_char, path, filename);

    let fd = fopen(filepath.as_ptr(), b"r\0".as_ptr() as *const c_char);
    if fd.is_null() {
        syslog(LOG_ERR, b"Err: open %s: %s\n\0".as_ptr() as *const c_char, b"sysfs_get_ulong\0".as_ptr() as *const c_char, filepath.as_ptr());
        return ret;
    }
    ret = fscanf(fd, b"%lu\0".as_ptr() as *const c_char, p_ulong);
    fclose(fd);

    0
}

unsafe extern "C" fn sysfs_get_string(
    path: *mut c_char,
    filename: *mut c_char,
    str_: *mut c_char,
) -> c_int {
    let mut ret: c_int = -1;
    let mut filepath = [0 as c_char; PATH_MAX + 2]; /* NUL and '/' */

    snprintf(filepath.as_mut_ptr(), filepath.len(), b"%s/%s\0".as_ptr() as *const c_char, path, filename);

    let fd = fopen(filepath.as_ptr(), b"r\0".as_ptr() as *const c_char);
    if fd.is_null() {
        syslog(LOG_ERR, b"Err: open %s: %s\n\0".as_ptr() as *const c_char, b"sysfs_get_string\0".as_ptr() as *const c_char, filepath.as_ptr());
        return ret;
    }
    ret = fscanf(fd, b"%256s\0".as_ptr() as *const c_char, str_);
    fclose(fd);

    ret
}

/* get states of the cooling device instance */
unsafe extern "C" fn probe_cdev(cdi: *mut cdev_info, path: *mut c_char) -> c_int {
    sysfs_get_string(path, b"type\0".as_ptr() as *mut c_char, (*cdi).type_.as_mut_ptr());
    sysfs_get_ulong(path, b"max_state\0".as_ptr() as *mut c_char, &mut (*cdi).max_state);
    sysfs_get_ulong(path, b"cur_state\0".as_ptr() as *mut c_char, &mut (*cdi).cur_state);

    syslog(
        LOG_INFO,
        b"%s: %s: type %s, max %lu, curr %lu inst %d\n\0".as_ptr() as *const c_char,
        b"probe_cdev\0".as_ptr() as *const c_char,
        path,
        (*cdi).type_.as_ptr(),
        (*cdi).max_state,
        (*cdi).cur_state,
        (*cdi).instance,
    );

    0
}

unsafe extern "C" fn str_to_trip_type(name: *mut c_char) -> c_int {
    let mut i: c_int = 0;

    while i < NR_THERMAL_TRIP_TYPE {
        if strcmp(name, trip_type_name[i as usize]) == 0 {
            return i;
        }
        i += 1;
    }

    -ENOENT
}

/* scan and fill in trip point info for a thermal zone and trip point id */
unsafe extern "C" fn get_trip_point_data(tz_path: *mut c_char, tzid: c_int, tpid: c_int) -> c_int {
    let mut filename = [0 as c_char; 256];
    let mut temp_str = [0 as c_char; 256];
    let trip_type: c_int;

    if tpid >= MAX_NR_TRIP {
        return -EINVAL;
    }
    /* check trip point type */
    snprintf(filename.as_mut_ptr(), filename.len(), b"trip_point_%d_type\0".as_ptr() as *const c_char, tpid);
    sysfs_get_string(tz_path, filename.as_mut_ptr(), temp_str.as_mut_ptr());
    trip_type = str_to_trip_type(temp_str.as_mut_ptr());
    if trip_type < 0 {
        syslog(LOG_ERR, b"%s:%s no matching type\n\0".as_ptr() as *const c_char, b"get_trip_point_data\0".as_ptr() as *const c_char, temp_str.as_ptr());
        return -ENOENT;
    }
    (*(*ptdata.tzi.add(tzid as usize)).tp.add(tpid as usize)).type_ = trip_type;
    syslog(
        LOG_INFO,
        b"%s:tz:%d tp:%d:type:%s type id %d\n\0".as_ptr() as *const c_char,
        b"get_trip_point_data\0".as_ptr() as *const c_char,
        tzid,
        tpid,
        temp_str.as_ptr(),
        trip_type,
    );

    /* TODO: check attribute */

    0
}

/* return instance id for file format such as trip_point_4_temp */
unsafe extern "C" fn get_instance_id(name: *mut c_char, pos: c_int, skip: c_int) -> c_int {
    let mut ch: *mut c_char;
    let mut i: c_int = 0;

    ch = strtok(name, b"_\0".as_ptr() as *const c_char);
    while !ch.is_null() {
        i += 1;
        syslog(LOG_INFO, b"%s:%s:%s:%d\0".as_ptr() as *const c_char, b"get_instance_id\0".as_ptr() as *const c_char, name, ch, i);
        ch = strtok(ptr::null_mut(), b"_\0".as_ptr() as *const c_char);
        if pos == i {
            return atol(ch.add(skip as usize)) as c_int;
        }
    }

    -1
}

/* Find trip point info of a thermal zone */
unsafe extern "C" fn find_tzone_tp(
    tz_name: *mut c_char,
    d_name: *mut c_char,
    tzi: *mut tz_info,
    tz_id: c_int,
) -> c_int {
    let mut tp_id: c_int;
    let mut temp_ulong: c_ulong = 0;

    if !strstr(d_name, b"trip_point\0".as_ptr() as *const c_char).is_null()
        && !strstr(d_name, b"temp\0".as_ptr() as *const c_char).is_null()
    {
        /* check if trip point temp is non-zero
         * ignore 0/invalid trip points
         */
        sysfs_get_ulong(tz_name, d_name, &mut temp_ulong);
        if temp_ulong < MAX_TEMP_KC {
            (*tzi).nr_trip_pts += 1;
            /* found a valid trip point */
            tp_id = get_instance_id(d_name, 2, 0);
            syslog(
                LOG_DEBUG,
                b"tzone %s trip %d temp %lu tpnode %s\0".as_ptr() as *const c_char,
                tz_name,
                tp_id,
                temp_ulong,
                d_name,
            );
            if tp_id < 0 || tp_id >= MAX_NR_TRIP {
                syslog(LOG_ERR, b"Failed to find TP inst %s\n\0".as_ptr() as *const c_char, d_name);
                return -1;
            }
            get_trip_point_data(tz_name, tz_id, tp_id);
            (*(*tzi).tp.add(tp_id as usize)).temp = temp_ulong;
        }
    }

    0
}

/* check cooling devices for binding info. */
unsafe extern "C" fn find_tzone_cdev(
    nl: *mut dirent,
    tz_name: *mut c_char,
    tzi: *mut tz_info,
    tz_id: c_int,
    cid: c_int,
) -> c_int {
    let mut trip_instance: c_ulong = 0;
    let mut cdev_name_linked = [0 as c_char; 256];
    let mut cdev_name = [0 as c_char; PATH_MAX];
    let mut cdev_trip_name = [0 as c_char; PATH_MAX];
    let cdev_id: c_int;

    if (*nl).d_type == DT_LNK {
        syslog(LOG_DEBUG, b"TZ%d: cdev: %s cid %d\n\0".as_ptr() as *const c_char, tz_id, (*nl).d_name.as_ptr(), cid);
        (*tzi).nr_cdev += 1;
        if (*tzi).nr_cdev > ptdata.nr_cooling_dev {
            syslog(LOG_ERR, b"Err: Too many cdev? %d\n\0".as_ptr() as *const c_char, (*tzi).nr_cdev);
            return -EINVAL;
        }
        /* find the link to real cooling device record binding */
        snprintf(cdev_name.as_mut_ptr(), cdev_name.len() - 2, b"%s/%s\0".as_ptr() as *const c_char, tz_name, (*nl).d_name.as_ptr());
        memset(cdev_name_linked.as_mut_ptr() as *mut c_void, 0, cdev_name_linked.len());
        if readlink(cdev_name.as_ptr(), cdev_name_linked.as_mut_ptr(), cdev_name_linked.len() - 1) != -1 {
            cdev_id = get_instance_id(cdev_name_linked.as_mut_ptr(), 1, (size_of::<[c_char; 7]>() - 1) as c_int);
            syslog(LOG_DEBUG, b"cdev %s linked to %s : %d\n\0".as_ptr() as *const c_char, cdev_name.as_ptr(), cdev_name_linked.as_ptr(), cdev_id);
            (*tzi).cdev_binding |= (1 as c_ulong) << cdev_id;

            /* find the trip point in which the cdev is binded to
             * in this tzone
             */
            snprintf(cdev_trip_name.as_mut_ptr(), cdev_trip_name.len() - 1, b"%s%s\0".as_ptr() as *const c_char, (*nl).d_name.as_ptr(), b"_trip_point\0".as_ptr() as *const c_char);
            sysfs_get_ulong(tz_name, cdev_trip_name.as_mut_ptr(), &mut trip_instance);
            /* validate trip point range, e.g. trip could return -1
             * when passive is enabled
             */
            if trip_instance > MAX_NR_TRIP as c_ulong {
                trip_instance = 0;
            }
            *(*tzi).trip_binding.add(cdev_id as usize) |= (1 as c_ulong) << trip_instance;
            syslog(
                LOG_DEBUG,
                b"cdev %s -> trip:%lu: 0x%lx %d\n\0".as_ptr() as *const c_char,
                cdev_name.as_ptr(),
                trip_instance,
                *(*tzi).trip_binding.add(cdev_id as usize),
                cdev_id,
            );
        }
        return 0;
    }

    -ENODEV
}

/*****************************************************************************
 * Before calling scan_tzones, thermal sysfs must be probed to determine
 * the number of thermal zones and cooling devices.
 * We loop through each thermal zone and fill in tz_info struct, i.e.
 * ptdata.tzi[]
root@jacob-chiefriver:~# tree -d /sys/class/thermal/thermal_zone0
/sys/class/thermal/thermal_zone0
|-- cdev0 -> ../cooling_device4
|-- cdev1 -> ../cooling_device3
|-- cdev10 -> ../cooling_device7
|-- cdev11 -> ../cooling_device6
|-- cdev12 -> ../cooling_device5
|-- cdev2 -> ../cooling_device2
|-- cdev3 -> ../cooling_device1
|-- cdev4 -> ../cooling_device0
|-- cdev5 -> ../cooling_device12
|-- cdev6 -> ../cooling_device11
|-- cdev7 -> ../cooling_device10
|-- cdev8 -> ../cooling_device9
|-- cdev9 -> ../cooling_device8
|-- device -> ../../../LNXSYSTM:00/device:62/LNXTHERM:00
|-- power
`-- subsystem -> ../../../../class/thermal
*****************************************************************************/
unsafe extern "C" fn scan_tzones() -> c_int {
    let mut namelist: *mut *mut dirent = ptr::null_mut();
    let mut tz_name = [0 as c_char; 256];
    let mut i: c_int;
    let mut j: c_int;
    let mut n: c_int;
    let mut k: c_int = 0;

    if ptdata.nr_tz_sensor == 0 {
        return -1;
    }

    i = 0;
    while i <= ptdata.max_tz_instance {
        memset(tz_name.as_mut_ptr() as *mut c_void, 0, tz_name.len());
        snprintf(tz_name.as_mut_ptr(), 256, b"%s/%s%d\0".as_ptr() as *const c_char, THERMAL_SYSFS, TZONE, i);

        let dir = opendir(tz_name.as_ptr());
        if dir.is_null() {
            syslog(LOG_INFO, b"Thermal zone %s skipped\n\0".as_ptr() as *const c_char, tz_name.as_ptr());
            i += 1;
            continue;
        }
        /* keep track of valid tzones */
        n = scandir(tz_name.as_ptr(), &mut namelist, None, Some(alphasort));
        if n < 0 {
            syslog(LOG_ERR, b"scandir failed in %s\0".as_ptr() as *const c_char, tz_name.as_ptr());
        } else {
            sysfs_get_string(tz_name.as_mut_ptr(), b"type\0".as_ptr() as *mut c_char, (*ptdata.tzi.add(k as usize)).type_.as_mut_ptr());
            (*ptdata.tzi.add(k as usize)).instance = i;
            /* detect trip points and cdev attached to this tzone */
            j = 0; /* index for cdev */
            (*ptdata.tzi.add(k as usize)).nr_cdev = 0;
            (*ptdata.tzi.add(k as usize)).nr_trip_pts = 0;
            while n != 0 {
                let temp_str: *mut c_char;
                n -= 1;

                if find_tzone_tp(tz_name.as_mut_ptr(), (**namelist.add(n as usize)).d_name.as_mut_ptr(), ptdata.tzi.add(k as usize), k) != 0 {
                    break;
                }
                temp_str = strstr((**namelist.add(n as usize)).d_name.as_ptr(), b"cdev\0".as_ptr() as *const c_char);
                if temp_str.is_null() {
                    free(*namelist.add(n as usize) as *mut c_void);
                    continue;
                }
                if find_tzone_cdev(*namelist.add(n as usize), tz_name.as_mut_ptr(), ptdata.tzi.add(k as usize), i, j) == 0 {
                    j += 1; /* increment cdev index */
                }
                free(*namelist.add(n as usize) as *mut c_void);
            }
            free(namelist as *mut c_void);
        }
        /*TODO: reverse trip points */
        closedir(dir);
        syslog(LOG_INFO, b"TZ %d has %d cdev\n\0".as_ptr() as *const c_char, i, (*ptdata.tzi.add(k as usize)).nr_cdev);
        k += 1;
        i += 1;
    }

    0
}

unsafe extern "C" fn scan_cdevs() -> c_int {
    let mut namelist: *mut *mut dirent = ptr::null_mut();
    let mut cdev_name = [0 as c_char; 256];
    let mut i: c_int;
    let mut n: c_int;
    let mut k: c_int = 0;

    if ptdata.nr_cooling_dev == 0 {
        fprintf(stderr, b"No cooling devices found\n\0".as_ptr() as *const c_char);
        return 0;
    }
    i = 0;
    while i <= ptdata.max_cdev_instance {
        memset(cdev_name.as_mut_ptr() as *mut c_void, 0, cdev_name.len());
        snprintf(cdev_name.as_mut_ptr(), 256, b"%s/%s%d\0".as_ptr() as *const c_char, THERMAL_SYSFS, CDEV, i);

        let dir = opendir(cdev_name.as_ptr());
        if dir.is_null() {
            syslog(LOG_INFO, b"Cooling dev %s skipped\n\0".as_ptr() as *const c_char, cdev_name.as_ptr());
            /* there is a gap in cooling device id, check again
             * for the same index.
             */
            i += 1;
            continue;
        }

        n = scandir(cdev_name.as_ptr(), &mut namelist, None, Some(alphasort));
        if n < 0 {
            syslog(LOG_ERR, b"scandir failed in %s\0".as_ptr() as *const c_char, cdev_name.as_ptr());
        } else {
            sysfs_get_string(cdev_name.as_mut_ptr(), b"type\0".as_ptr() as *mut c_char, (*ptdata.cdi.add(k as usize)).type_.as_mut_ptr());
            (*ptdata.cdi.add(k as usize)).instance = i;
            if !strstr((*ptdata.cdi.add(k as usize)).type_.as_ptr(), ctrl_cdev).is_null() {
                (*ptdata.cdi.add(k as usize)).flag |= CDEV_FLAG_IN_CONTROL;
                syslog(LOG_DEBUG, b"control cdev id %d\n\0".as_ptr() as *const c_char, i);
            }
            while n != 0 {
                n -= 1;
                free(*namelist.add(n as usize) as *mut c_void);
            }
            free(namelist as *mut c_void);
        }
        closedir(dir);
        k += 1;
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn probe_thermal_sysfs() -> c_int {
    let mut namelist: *mut *mut dirent = ptr::null_mut();
    let mut n: c_int;

    let dir = opendir(THERMAL_SYSFS);
    if dir.is_null() {
        fprintf(stderr, b"\nNo thermal sysfs, exit\n\0".as_ptr() as *const c_char);
        return -1;
    }
    n = scandir(THERMAL_SYSFS, &mut namelist, None, Some(alphasort));
    if n < 0 {
        syslog(LOG_ERR, b"scandir failed in thermal sysfs\0".as_ptr() as *const c_char);
    } else {
        /* detect number of thermal zones and cooling devices */
        while n != 0 {
            let inst: c_int;
            n -= 1;

            if !strstr((**namelist.add(n as usize)).d_name.as_ptr(), CDEV).is_null() {
                inst = get_instance_id((**namelist.add(n as usize)).d_name.as_mut_ptr(), 1, (size_of::<[c_char; 7]>() - 1) as c_int);
                /* keep track of the max cooling device since
                 * there may be gaps.
                 */
                if inst > ptdata.max_cdev_instance {
                    ptdata.max_cdev_instance = inst;
                }

                syslog(
                    LOG_DEBUG,
                    b"found cdev: %s %d %d\n\0".as_ptr() as *const c_char,
                    (**namelist.add(n as usize)).d_name.as_ptr(),
                    ptdata.nr_cooling_dev,
                    ptdata.max_cdev_instance,
                );
                ptdata.nr_cooling_dev += 1;
            } else if !strstr((**namelist.add(n as usize)).d_name.as_ptr(), TZONE).is_null() {
                inst = get_instance_id((**namelist.add(n as usize)).d_name.as_mut_ptr(), 1, (size_of::<[c_char; 5]>() - 1) as c_int);
                if inst > ptdata.max_tz_instance {
                    ptdata.max_tz_instance = inst;
                }

                syslog(
                    LOG_DEBUG,
                    b"found tzone: %s %d %d\n\0".as_ptr() as *const c_char,
                    (**namelist.add(n as usize)).d_name.as_ptr(),
                    ptdata.nr_tz_sensor,
                    ptdata.max_tz_instance,
                );
                ptdata.nr_tz_sensor += 1;
            }
            free(*namelist.add(n as usize) as *mut c_void);
        }
        free(namelist as *mut c_void);
    }
    syslog(
        LOG_INFO,
        b"found %d tzone(s), %d cdev(s), target zone %d\n\0".as_ptr() as *const c_char,
        ptdata.nr_tz_sensor,
        ptdata.nr_cooling_dev,
        target_thermal_zone,
    );
    closedir(dir);

    if ptdata.nr_tz_sensor == 0 {
        fprintf(stderr, b"\nNo thermal zones found, exit\n\n\0".as_ptr() as *const c_char);
        return -1;
    }

    ptdata.tzi = calloc((ptdata.max_tz_instance + 1) as usize, size_of::<tz_info>()) as *mut tz_info;
    if ptdata.tzi.is_null() {
        fprintf(stderr, b"Err: allocate tz_info\n\0".as_ptr() as *const c_char);
        return -1;
    }

    /* we still show thermal zone information if there is no cdev */
    if ptdata.nr_cooling_dev != 0 {
        ptdata.cdi = calloc((ptdata.max_cdev_instance + 1) as usize, size_of::<cdev_info>()) as *mut cdev_info;
        if ptdata.cdi.is_null() {
            free(ptdata.tzi as *mut c_void);
            fprintf(stderr, b"Err: allocate cdev_info\n\0".as_ptr() as *const c_char);
            return -1;
        }
    }

    /* now probe tzones */
    if scan_tzones() != 0 {
        return -1;
    }
    if scan_cdevs() != 0 {
        return -1;
    }
    0
}

/* convert sysfs zone instance to zone array index */
#[no_mangle]
pub unsafe extern "C" fn zone_instance_to_index(zone_inst: c_int) -> c_int {
    let mut i: c_int = 0;

    while i < ptdata.nr_tz_sensor {
        if (*ptdata.tzi.add(i as usize)).instance == zone_inst {
            return i;
        }
        i += 1;
    }
    -ENOENT
}

/* read temperature of all thermal zones */
#[no_mangle]
pub unsafe extern "C" fn update_thermal_data() -> c_int {
    let mut i: c_int;
    let mut next_thermal_record: c_int = cur_thermal_record + 1;
    let mut tz_name = [0 as c_char; 256];
    static mut samples: c_ulong = 0;

    if ptdata.nr_tz_sensor == 0 {
        syslog(LOG_ERR, b"No thermal zones found!\n\0".as_ptr() as *const c_char);
        return -1;
    }

    /* circular buffer for keeping historic data */
    if next_thermal_record >= NR_THERMAL_RECORDS as c_int {
        next_thermal_record = 0;
    }
    gettimeofday(&mut trec[next_thermal_record as usize].tv, ptr::null_mut());
    if !tmon_log.is_null() {
        samples += 1;
        fprintf(tmon_log, b"%lu \0".as_ptr() as *const c_char, samples);
        fprintf(tmon_log, b"%3.1f \0".as_ptr() as *const c_char, p_param.t_target as f64);
    }
    i = 0;
    while i < ptdata.nr_tz_sensor {
        memset(tz_name.as_mut_ptr() as *mut c_void, 0, tz_name.len());
        snprintf(
            tz_name.as_mut_ptr(),
            256,
            b"%s/%s%d\0".as_ptr() as *const c_char,
            THERMAL_SYSFS,
            TZONE,
            (*ptdata.tzi.add(i as usize)).instance,
        );
        sysfs_get_ulong(
            tz_name.as_mut_ptr(),
            b"temp\0".as_ptr() as *mut c_char,
            trec[next_thermal_record as usize].temp.add(i as usize),
        );
        if !tmon_log.is_null() {
            fprintf(
                tmon_log,
                b"%lu \0".as_ptr() as *const c_char,
                *trec[next_thermal_record as usize].temp.add(i as usize) / 1000,
            );
        }
        i += 1;
    }
    cur_thermal_record = next_thermal_record;
    i = 0;
    while i < ptdata.nr_cooling_dev {
        let mut cdev_name = [0 as c_char; 256];
        let mut val: c_ulong;

        snprintf(
            cdev_name.as_mut_ptr(),
            256,
            b"%s/%s%d\0".as_ptr() as *const c_char,
            THERMAL_SYSFS,
            CDEV,
            (*ptdata.cdi.add(i as usize)).instance,
        );
        probe_cdev(ptdata.cdi.add(i as usize), cdev_name.as_mut_ptr());
        val = (*ptdata.cdi.add(i as usize)).cur_state;
        if val > 1000000 {
            val = 0;
        }
        if !tmon_log.is_null() {
            fprintf(tmon_log, b"%lu \0".as_ptr() as *const c_char, val);
        }
        i += 1;
    }

    if !tmon_log.is_null() {
        fprintf(tmon_log, b"\n\0".as_ptr() as *const c_char);
        fflush(tmon_log);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn set_ctrl_state(state: c_ulong) {
    let mut ctrl_cdev_path = [0 as c_char; 256];
    let mut i: c_int;
    let cdev_state: c_ulong;

    if no_control != 0 {
        return;
    }
    /* set all ctrl cdev to the same state */
    i = 0;
    while i < ptdata.nr_cooling_dev {
        if ((*ptdata.cdi.add(i as usize)).flag & CDEV_FLAG_IN_CONTROL) != 0 {
            if (*ptdata.cdi.add(i as usize)).max_state < 10 {
                strcpy(ctrl_cdev, b"None.\0".as_ptr() as *const c_char);
                return;
            }
            /* scale to percentage of max_state */
            cdev_state = state * (*ptdata.cdi.add(i as usize)).max_state / 100;
            syslog(
                LOG_DEBUG,
                b"ctrl cdev %d set state %lu scaled to %lu\n\0".as_ptr() as *const c_char,
                (*ptdata.cdi.add(i as usize)).instance,
                state,
                cdev_state,
            );
            snprintf(
                ctrl_cdev_path.as_mut_ptr(),
                256,
                b"%s/%s%d\0".as_ptr() as *const c_char,
                THERMAL_SYSFS,
                CDEV,
                (*ptdata.cdi.add(i as usize)).instance,
            );
            syslog(LOG_DEBUG, b"ctrl cdev path %s\0".as_ptr() as *const c_char, ctrl_cdev_path.as_ptr());
            sysfs_set_ulong(ctrl_cdev_path.as_mut_ptr(), b"cur_state\0".as_ptr() as *mut c_char, cdev_state);
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_ctrl_state(state: *mut c_ulong) {
    let mut ctrl_cdev_path = [0 as c_char; 256];
    let mut ctrl_cdev_id: c_int = -1;
    let mut i: c_int;

    /* TODO: take average of all ctrl types. also consider change based on
     * uevent. Take the first reading for now.
     */
    i = 0;
    while i < ptdata.nr_cooling_dev {
        if ((*ptdata.cdi.add(i as usize)).flag & CDEV_FLAG_IN_CONTROL) != 0 {
            ctrl_cdev_id = (*ptdata.cdi.add(i as usize)).instance;
            syslog(LOG_INFO, b"ctrl cdev %d get state\n\0".as_ptr() as *const c_char, (*ptdata.cdi.add(i as usize)).instance);
            break;
        }
        i += 1;
    }
    if ctrl_cdev_id == -1 {
        *state = 0;
        return;
    }
    snprintf(
        ctrl_cdev_path.as_mut_ptr(),
        256,
        b"%s/%s%d\0".as_ptr() as *const c_char,
        THERMAL_SYSFS,
        CDEV,
        ctrl_cdev_id,
    );
    sysfs_get_ulong(ctrl_cdev_path.as_mut_ptr(), b"cur_state\0".as_ptr() as *mut c_char, state);
}

#[no_mangle]
pub unsafe extern "C" fn free_thermal_data() {
    free(ptdata.tzi as *mut c_void);
    free(ptdata.cdi as *mut c_void);
}
