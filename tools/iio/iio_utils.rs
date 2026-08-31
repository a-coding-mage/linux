// SPDX-License-Identifier: GPL-2.0-only
/* IIO - useful set of util functionality
 *
 * Copyright (c) 2008 Jonathan Cameron
 */

use libc::{
    c_char, c_float, c_int, c_uint, c_ulong, c_void, uint64_t, DIR, FILE, EINVAL, EIO, ENODEV,
    ENODATA, ENOENT, ENOMEM,
};
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_channel_info {
    pub name: *mut c_char,
    pub generic_name: *mut c_char,
    pub scale: c_float,
    pub offset: c_float,
    pub index: c_uint,
    pub bytes: c_uint,
    pub bits_used: c_uint,
    pub shift: c_uint,
    pub mask: uint64_t,
    pub be: c_uint,
    pub format: c_char,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn strlen(s: *const c_char) -> c_ulong;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: c_ulong) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strndup(s: *const c_char, n: c_ulong) -> *mut c_char;
    fn strtok(s: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn sprintf(str: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn fscanf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn sscanf(str: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn free(ptr: *mut c_void);
    fn malloc(size: c_ulong) -> *mut c_void;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn rewinddir(dirp: *mut DIR);
    fn closedir(dirp: *mut DIR) -> c_int;
    fn perror(s: *const c_char);
    fn isdigit(c: c_int) -> c_int;
}

#[repr(C)]
pub struct dirent {
    pub d_ino: libc::ino_t,
    pub d_off: libc::off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [c_char; 256],
}

unsafe extern "C" {
    static mut stderr: *mut FILE;
}

#[unsafe(no_mangle)]
pub static mut iio_dir: *const c_char = b"/sys/bus/iio/devices/\0".as_ptr() as *const c_char;

static mut iio_direction: [*const c_char; 2] = [
    b"in\0".as_ptr() as *const c_char,
    b"out\0".as_ptr() as *const c_char,
];

/* FORMAT_SCAN_ELEMENTS_DIR, FORMAT_TYPE_FILE, and IIO_MAX_NAME_LENGTH are
 * supplied by iio_utils.h in the original C source.
 */
const FORMAT_SCAN_ELEMENTS_DIR: *const c_char = b"%s/buffer%d\0".as_ptr() as *const c_char;
const FORMAT_TYPE_FILE: *const c_char = b"%s_type\0".as_ptr() as *const c_char;
const IIO_MAX_NAME_LENGTH: usize = 30;

/**
 * iioutils_break_up_name() - extract generic name from full channel name
 * @full_name: the full channel name
 * @generic_name: the output generic channel name
 *
 * Returns 0 on success, or a negative error code if string extraction failed.
 **/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iioutils_break_up_name(
    full_name: *const c_char,
    generic_name: *mut *mut c_char,
) -> c_int {
    let mut current: *mut c_char;
    let mut w: *mut c_char;
    let mut r: *mut c_char;
    let mut working: *mut c_char;
    let mut prefix: *const c_char = b"\0".as_ptr() as *const c_char;
    let mut ret: c_int;

    for i in 0..iio_direction.len() {
        if strncmp(full_name, iio_direction[i], strlen(iio_direction[i])) == 0 {
            prefix = iio_direction[i];
            break;
        }
    }

    current = strdup(full_name.add(strlen(prefix) as usize + 1));
    if current.is_null() {
        return -ENOMEM;
    }

    working = strtok(current, b"_\0".as_ptr() as *const c_char);
    if working.is_null() {
        free(current as *mut c_void);
        return -EINVAL;
    }

    w = working;
    r = working;

    while *r != 0 {
        if isdigit(*r as c_int) == 0 {
            *w = *r;
            w = w.add(1);
        }

        r = r.add(1);
    }
    *w = 0;
    ret = asprintf(generic_name, b"%s_%s\0".as_ptr() as *const c_char, prefix, working);
    free(current as *mut c_void);

    if ret == -1 {
        -ENOMEM
    } else {
        0
    }
}

/**
 * iioutils_get_type() - find and process _type attribute data
 * @format: output channel format
 * @bytes: output how many bytes the channel storage occupies
 * @bits_used: output number of valid bits of data
 * @shift: output amount of bits to shift right data before applying bit mask
 * @mask: output a bit mask for the raw data
 * @be: output if data in big endian
 * @device_dir: the IIO device directory
 * @buffer_idx: the IIO buffer index
 * @name: the channel name
 * @generic_name: the channel type name
 *
 * Returns a value >= 0 on success, otherwise a negative error code.
 **/
unsafe fn iioutils_get_type(
    format: *mut c_char,
    bytes: *mut c_uint,
    bits_used: *mut c_uint,
    shift: *mut c_uint,
    mask: *mut uint64_t,
    be: *mut c_uint,
    device_dir: *const c_char,
    buffer_idx: c_int,
    name: *const c_char,
    generic_name: *const c_char,
) -> c_int {
    let mut sysfsfp: *mut FILE = ptr::null_mut();
    let mut ret: c_int;
    let mut dp: *mut DIR;
    let mut scan_el_dir: *mut c_char = ptr::null_mut();
    let mut builtname: *mut c_char = ptr::null_mut();
    let mut builtname_generic: *mut c_char = ptr::null_mut();
    let mut filename: *mut c_char = ptr::null_mut();
    let mut formatchar: c_char = 0;
    let mut endianchar: c_char = 0;
    let mut padint: c_uint = 0;
    let mut ent: *mut dirent;

    ret = asprintf(&mut scan_el_dir, FORMAT_SCAN_ELEMENTS_DIR, device_dir, buffer_idx);
    if ret < 0 {
        return -ENOMEM;
    }

    ret = asprintf(&mut builtname, FORMAT_TYPE_FILE, name);
    if ret < 0 {
        ret = -ENOMEM;
        goto_free_scan_el_dir(scan_el_dir);
        return ret;
    }
    ret = asprintf(&mut builtname_generic, FORMAT_TYPE_FILE, generic_name);
    if ret < 0 {
        ret = -ENOMEM;
        free(builtname as *mut c_void);
        goto_free_scan_el_dir(scan_el_dir);
        return ret;
    }

    dp = opendir(scan_el_dir);
    if dp.is_null() {
        ret = -errno;
        free(builtname_generic as *mut c_void);
        free(builtname as *mut c_void);
        goto_free_scan_el_dir(scan_el_dir);
        return ret;
    }

    ret = -ENOENT;
    loop {
        ent = readdir(dp);
        if ent.is_null() {
            break;
        }
        let d_name = (*ent).d_name.as_ptr();
        if strcmp(builtname, d_name) == 0 || strcmp(builtname_generic, d_name) == 0 {
            ret = asprintf(
                &mut filename,
                b"%s/%s\0".as_ptr() as *const c_char,
                scan_el_dir,
                d_name,
            );
            if ret < 0 {
                ret = -ENOMEM;
                break;
            }

            sysfsfp = fopen(filename, b"r\0".as_ptr() as *const c_char);
            if sysfsfp.is_null() {
                ret = -errno;
                fprintf(stderr, b"failed to open %s\n\0".as_ptr() as *const c_char, filename);
                break;
            }

            ret = fscanf(
                sysfsfp,
                b"%ce:%c%u/%u>>%u\0".as_ptr() as *const c_char,
                &mut endianchar,
                &mut formatchar,
                bits_used,
                &mut padint,
                shift,
            );
            if ret < 0 {
                ret = -errno;
                fprintf(
                    stderr,
                    b"failed to pass scan type description\n\0".as_ptr() as *const c_char,
                );
                break;
            } else if ret != 5 {
                ret = -EIO;
                fprintf(
                    stderr,
                    b"scan type description didn't match\n\0".as_ptr() as *const c_char,
                );
                break;
            }

            *be = (endianchar == b'b' as c_char) as c_uint;
            *bytes = padint / 8;
            if *bits_used == 64 {
                *mask = !0u64;
            } else {
                *mask = (1u64 << *bits_used) - 1u64;
            }

            *format = formatchar;
            if fclose(sysfsfp) != 0 {
                ret = -errno;
                fprintf(stderr, b"Failed to close %s\n\0".as_ptr() as *const c_char, filename);
                sysfsfp = ptr::null_mut();
                break;
            }

            sysfsfp = ptr::null_mut();
            free(filename as *mut c_void);
            filename = ptr::null_mut();

            /*
             * Avoid having a more generic entry overwriting
             * the settings.
             */
            if strcmp(builtname, d_name) == 0 {
                break;
            }
        }
    }

    if !sysfsfp.is_null() && fclose(sysfsfp) != 0 {
        perror(b"iioutils_get_type(): Failed to close file\0".as_ptr() as *const c_char);
    }
    if !filename.is_null() {
        free(filename as *mut c_void);
    }
    if closedir(dp) == -1 {
        perror(b"iioutils_get_type(): Failed to close directory\0".as_ptr() as *const c_char);
    }
    free(builtname_generic as *mut c_void);
    free(builtname as *mut c_void);
    free(scan_el_dir as *mut c_void);

    ret
}

unsafe fn goto_free_scan_el_dir(scan_el_dir: *mut c_char) {
    free(scan_el_dir as *mut c_void);
}

/**
 * iioutils_get_param_float() - read a float value from a channel parameter
 * @output: output the float value
 * @param_name: the parameter name to read
 * @device_dir: the IIO device directory in sysfs
 * @name: the channel name
 * @generic_name: the channel type name
 *
 * Returns a value >= 0 on success, otherwise a negative error code.
 **/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iioutils_get_param_float(
    output: *mut c_float,
    param_name: *const c_char,
    device_dir: *const c_char,
    name: *const c_char,
    generic_name: *const c_char,
) -> c_int {
    let mut sysfsfp: *mut FILE;
    let mut ret: c_int;
    let mut dp: *mut DIR;
    let mut builtname: *mut c_char = ptr::null_mut();
    let mut builtname_generic: *mut c_char = ptr::null_mut();
    let mut filename: *mut c_char = ptr::null_mut();
    let mut ent: *mut dirent;

    ret = asprintf(&mut builtname, b"%s_%s\0".as_ptr() as *const c_char, name, param_name);
    if ret < 0 {
        return -ENOMEM;
    }

    ret = asprintf(
        &mut builtname_generic,
        b"%s_%s\0".as_ptr() as *const c_char,
        generic_name,
        param_name,
    );
    if ret < 0 {
        ret = -ENOMEM;
        free(builtname as *mut c_void);
        return ret;
    }

    dp = opendir(device_dir);
    if dp.is_null() {
        ret = -errno;
        free(builtname_generic as *mut c_void);
        free(builtname as *mut c_void);
        return ret;
    }

    ret = -ENOENT;
    loop {
        ent = readdir(dp);
        if ent.is_null() {
            break;
        }
        let d_name = (*ent).d_name.as_ptr();
        if strcmp(builtname, d_name) == 0 || strcmp(builtname_generic, d_name) == 0 {
            ret = asprintf(&mut filename, b"%s/%s\0".as_ptr() as *const c_char, device_dir, d_name);
            if ret < 0 {
                ret = -ENOMEM;
                break;
            }

            sysfsfp = fopen(filename, b"r\0".as_ptr() as *const c_char);
            if sysfsfp.is_null() {
                ret = -errno;
                break;
            }

            errno = 0;
            if fscanf(sysfsfp, b"%f\0".as_ptr() as *const c_char, output) != 1 {
                ret = if errno != 0 { -errno } else { -ENODATA };
            }

            fclose(sysfsfp);
            break;
        }
    }
    if !filename.is_null() {
        free(filename as *mut c_void);
    }
    if closedir(dp) == -1 {
        perror(b"iioutils_get_param_float(): Failed to close directory\0".as_ptr() as *const c_char);
    }
    free(builtname_generic as *mut c_void);
    free(builtname as *mut c_void);

    ret
}

/**
 * bsort_channel_array_by_index() - sort the array in index order
 * @ci_array: the iio_channel_info array to be sorted
 * @cnt: the amount of array elements
 **/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bsort_channel_array_by_index(
    ci_array: *mut iio_channel_info,
    cnt: c_int,
) {
    let mut temp: iio_channel_info;

    for _x in 0..cnt {
        for y in 0..(cnt - 1) {
            let y = y as isize;
            if (*ci_array.offset(y)).index > (*ci_array.offset(y + 1)).index {
                temp = *ci_array.offset(y + 1);
                *ci_array.offset(y + 1) = *ci_array.offset(y);
                *ci_array.offset(y) = temp;
            }
        }
    }
}

/**
 * build_channel_array() - function to figure out what channels are present
 * @device_dir: the IIO device directory in sysfs
 * @buffer_idx: the IIO buffer for this channel array
 * @ci_array: output the resulting array of iio_channel_info
 * @counter: output the amount of array elements
 *
 * Returns 0 on success, otherwise a negative error code.
 **/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_channel_array(
    device_dir: *const c_char,
    buffer_idx: c_int,
    ci_array: *mut *mut iio_channel_info,
    counter: *mut c_int,
) -> c_int {
    let mut dp: *mut DIR;
    let mut sysfsfp: *mut FILE;
    let mut count: c_int = 0;
    let mut current: *mut iio_channel_info;
    let mut ret: c_int;
    let mut ent: *mut dirent;
    let mut scan_el_dir: *mut c_char = ptr::null_mut();
    let mut filename: *mut c_char = ptr::null_mut();

    *counter = 0;
    ret = asprintf(&mut scan_el_dir, FORMAT_SCAN_ELEMENTS_DIR, device_dir, buffer_idx);
    if ret < 0 {
        return -ENOMEM;
    }

    dp = opendir(scan_el_dir);
    if dp.is_null() {
        ret = -errno;
        free(scan_el_dir as *mut c_void);
        return ret;
    }

    loop {
        ent = readdir(dp);
        if ent.is_null() {
            break;
        }
        let d_name = (*ent).d_name.as_ptr();
        if strcmp(d_name.add(strlen(d_name) as usize - strlen(b"_en\0".as_ptr() as *const c_char) as usize), b"_en\0".as_ptr() as *const c_char) == 0 {
            ret = asprintf(&mut filename, b"%s/%s\0".as_ptr() as *const c_char, scan_el_dir, d_name);
            if ret < 0 {
                ret = -ENOMEM;
                goto_build_error_close_dir(dp, scan_el_dir);
                return ret;
            }

            sysfsfp = fopen(filename, b"r\0".as_ptr() as *const c_char);
            free(filename as *mut c_void);
            if sysfsfp.is_null() {
                ret = -errno;
                goto_build_error_close_dir(dp, scan_el_dir);
                return ret;
            }

            errno = 0;
            if fscanf(sysfsfp, b"%i\0".as_ptr() as *const c_char, &mut ret) != 1 {
                ret = if errno != 0 { -errno } else { -ENODATA };
                if fclose(sysfsfp) != 0 {
                    perror(b"build_channel_array(): Failed to close file\0".as_ptr() as *const c_char);
                }
                goto_build_error_close_dir(dp, scan_el_dir);
                return ret;
            }
            if ret == 1 {
                *counter += 1;
            }

            if fclose(sysfsfp) != 0 {
                ret = -errno;
                goto_build_error_close_dir(dp, scan_el_dir);
                return ret;
            }
        }
    }

    *ci_array = malloc((mem::size_of::<iio_channel_info>() * (*counter as usize)) as c_ulong)
        as *mut iio_channel_info;
    if (*ci_array).is_null() {
        ret = -ENOMEM;
        goto_build_error_close_dir(dp, scan_el_dir);
        return ret;
    }

    rewinddir(dp);
    loop {
        ent = readdir(dp);
        if ent.is_null() {
            break;
        }
        let d_name = (*ent).d_name.as_ptr();
        if strcmp(d_name.add(strlen(d_name) as usize - strlen(b"_en\0".as_ptr() as *const c_char) as usize), b"_en\0".as_ptr() as *const c_char) == 0 {
            let mut current_enabled: c_int = 0;

            current = (*ci_array).offset(count as isize);
            count += 1;
            ret = asprintf(&mut filename, b"%s/%s\0".as_ptr() as *const c_char, scan_el_dir, d_name);
            if ret < 0 {
                ret = -ENOMEM;
                count -= 1;
                break;
            }

            sysfsfp = fopen(filename, b"r\0".as_ptr() as *const c_char);
            free(filename as *mut c_void);
            if sysfsfp.is_null() {
                ret = -errno;
                count -= 1;
                break;
            }

            errno = 0;
            if fscanf(sysfsfp, b"%i\0".as_ptr() as *const c_char, &mut current_enabled) != 1 {
                ret = if errno != 0 { -errno } else { -ENODATA };
                count -= 1;
                break;
            }

            if fclose(sysfsfp) != 0 {
                ret = -errno;
                count -= 1;
                break;
            }

            if current_enabled == 0 {
                count -= 1;
                continue;
            }

            (*current).scale = 1.0;
            (*current).offset = 0.0;
            (*current).name = strndup(d_name, strlen(d_name) - strlen(b"_en\0".as_ptr() as *const c_char));
            if (*current).name.is_null() {
                ret = -ENOMEM;
                count -= 1;
                break;
            }

            /* Get the generic and specific name elements */
            ret = iioutils_break_up_name((*current).name, &mut (*current).generic_name);
            if ret != 0 {
                free((*current).name as *mut c_void);
                count -= 1;
                break;
            }

            ret = asprintf(
                &mut filename,
                b"%s/%s_index\0".as_ptr() as *const c_char,
                scan_el_dir,
                (*current).name,
            );
            if ret < 0 {
                ret = -ENOMEM;
                break;
            }

            sysfsfp = fopen(filename, b"r\0".as_ptr() as *const c_char);
            free(filename as *mut c_void);
            if sysfsfp.is_null() {
                ret = -errno;
                fprintf(
                    stderr,
                    b"failed to open %s/%s_index\n\0".as_ptr() as *const c_char,
                    scan_el_dir,
                    (*current).name,
                );
                break;
            }

            errno = 0;
            if fscanf(sysfsfp, b"%u\0".as_ptr() as *const c_char, &mut (*current).index) != 1 {
                ret = if errno != 0 { -errno } else { -ENODATA };
                if fclose(sysfsfp) != 0 {
                    perror(b"build_channel_array(): Failed to close file\0".as_ptr() as *const c_char);
                }
                break;
            }

            if fclose(sysfsfp) != 0 {
                ret = -errno;
                break;
            }

            /* Find the scale */
            ret = iioutils_get_param_float(
                &mut (*current).scale,
                b"scale\0".as_ptr() as *const c_char,
                device_dir,
                (*current).name,
                (*current).generic_name,
            );
            if ret < 0 && ret != -ENOENT {
                break;
            }

            ret = iioutils_get_param_float(
                &mut (*current).offset,
                b"offset\0".as_ptr() as *const c_char,
                device_dir,
                (*current).name,
                (*current).generic_name,
            );
            if ret < 0 && ret != -ENOENT {
                break;
            }

            ret = iioutils_get_type(
                &mut (*current).format,
                &mut (*current).bytes,
                &mut (*current).bits_used,
                &mut (*current).shift,
                &mut (*current).mask,
                &mut (*current).be,
                device_dir,
                buffer_idx,
                (*current).name,
                (*current).generic_name,
            );
            if ret < 0 {
                break;
            }
        }
    }

    if ent.is_null() {
        if closedir(dp) == -1 {
            ret = -errno;
            cleanup_channel_array(ci_array, counter, count);
            free(scan_el_dir as *mut c_void);
            return ret;
        }

        free(scan_el_dir as *mut c_void);
        /* reorder so that the array is in index order */
        bsort_channel_array_by_index(*ci_array, *counter);

        return 0;
    }

    cleanup_channel_array(ci_array, counter, count);
    if !dp.is_null() && closedir(dp) == -1 {
        perror(b"build_channel_array(): Failed to close dir\0".as_ptr() as *const c_char);
    }
    free(scan_el_dir as *mut c_void);
    ret
}

unsafe fn goto_build_error_close_dir(dp: *mut DIR, scan_el_dir: *mut c_char) {
    if !dp.is_null() && closedir(dp) == -1 {
        perror(b"build_channel_array(): Failed to close dir\0".as_ptr() as *const c_char);
    }
    free(scan_el_dir as *mut c_void);
}

unsafe fn cleanup_channel_array(
    ci_array: *mut *mut iio_channel_info,
    counter: *mut c_int,
    count: c_int,
) {
    let mut i = count - 1;
    while i >= 0 {
        free((*(*ci_array).offset(i as isize)).name as *mut c_void);
        free((*(*ci_array).offset(i as isize)).generic_name as *mut c_void);
        i -= 1;
    }
    free(*ci_array as *mut c_void);
    *ci_array = ptr::null_mut();
    *counter = 0;
}

unsafe fn calc_digits(mut num: c_int) -> c_int {
    let mut count: c_int = 0;

    /* It takes a digit to represent zero */
    if num == 0 {
        return 1;
    }

    while num != 0 {
        num /= 10;
        count += 1;
    }

    count
}

/**
 * find_type_by_name() - function to match top level types by name
 * @name: top level type instance name
 * @type: the type of top level instance being searched
 *
 * Returns the device number of a matched IIO device on success, otherwise a
 * negative error code.
 * Typical types this is used for are device and trigger.
 **/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_type_by_name(name: *const c_char, type_: *const c_char) -> c_int {
    let mut ent: *mut dirent;
    let mut number: c_int = 0;
    let mut numstrlen: c_int;
    let mut ret: c_int;

    let mut namefp: *mut FILE;
    let mut dp: *mut DIR;
    let mut thisname: [c_char; IIO_MAX_NAME_LENGTH] = [0; IIO_MAX_NAME_LENGTH];
    let mut filename: *mut c_char;

    dp = opendir(iio_dir);
    if dp.is_null() {
        fprintf(stderr, b"No industrialio devices available\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    loop {
        ent = readdir(dp);
        if ent.is_null() {
            break;
        }
        let d_name = (*ent).d_name.as_ptr();
        if strcmp(d_name, b".\0".as_ptr() as *const c_char) != 0
            && strcmp(d_name, b"..\0".as_ptr() as *const c_char) != 0
            && strlen(d_name) > strlen(type_)
            && strncmp(d_name, type_, strlen(type_)) == 0
        {
            errno = 0;
            ret = sscanf(
                d_name.add(strlen(type_) as usize),
                b"%d\0".as_ptr() as *const c_char,
                &mut number,
            );
            if ret < 0 {
                ret = -errno;
                fprintf(stderr, b"failed to read element number\n\0".as_ptr() as *const c_char);
                if closedir(dp) == -1 {
                    perror(b"find_type_by_name(): Failed to close directory\0".as_ptr() as *const c_char);
                }
                return ret;
            } else if ret != 1 {
                ret = -EIO;
                fprintf(stderr, b"failed to match element number\n\0".as_ptr() as *const c_char);
                if closedir(dp) == -1 {
                    perror(b"find_type_by_name(): Failed to close directory\0".as_ptr() as *const c_char);
                }
                return ret;
            }

            numstrlen = calc_digits(number);
            /* verify the next character is not a colon */
            if strncmp(
                d_name.add(strlen(type_) as usize + numstrlen as usize),
                b":\0".as_ptr() as *const c_char,
                1,
            ) != 0
            {
                filename = malloc(strlen(iio_dir) + strlen(type_) + numstrlen as c_ulong + 6)
                    as *mut c_char;
                if filename.is_null() {
                    ret = -ENOMEM;
                    if closedir(dp) == -1 {
                        perror(b"find_type_by_name(): Failed to close directory\0".as_ptr() as *const c_char);
                    }
                    return ret;
                }

                ret = sprintf(
                    filename,
                    b"%s%s%d/name\0".as_ptr() as *const c_char,
                    iio_dir,
                    type_,
                    number,
                );
                if ret < 0 {
                    free(filename as *mut c_void);
                    if closedir(dp) == -1 {
                        perror(b"find_type_by_name(): Failed to close directory\0".as_ptr() as *const c_char);
                    }
                    return ret;
                }

                namefp = fopen(filename, b"r\0".as_ptr() as *const c_char);
                if namefp.is_null() {
                    free(filename as *mut c_void);
                    continue;
                }

                free(filename as *mut c_void);
                errno = 0;
                if fscanf(namefp, b"%s\0".as_ptr() as *const c_char, thisname.as_mut_ptr()) != 1 {
                    ret = if errno != 0 { -errno } else { -ENODATA };
                    if closedir(dp) == -1 {
                        perror(b"find_type_by_name(): Failed to close directory\0".as_ptr() as *const c_char);
                    }
                    return ret;
                }

                if fclose(namefp) != 0 {
                    ret = -errno;
                    if closedir(dp) == -1 {
                        perror(b"find_type_by_name(): Failed to close directory\0".as_ptr() as *const c_char);
                    }
                    return ret;
                }

                if strcmp(name, thisname.as_ptr()) == 0 {
                    if closedir(dp) == -1 {
                        return -errno;
                    }

                    return number;
                }
            }
        }
    }
    if closedir(dp) == -1 {
        return -errno;
    }

    -ENODEV
}

unsafe fn _write_sysfs_int(
    filename: *const c_char,
    basedir: *const c_char,
    val: c_int,
    verify: c_int,
) -> c_int {
    let mut ret: c_int = 0;
    let mut sysfsfp: *mut FILE;
    let mut test: c_int = 0;
    let temp = malloc(strlen(basedir) + strlen(filename) + 2) as *mut c_char;

    if temp.is_null() {
        return -ENOMEM;
    }

    ret = sprintf(temp, b"%s/%s\0".as_ptr() as *const c_char, basedir, filename);
    if ret < 0 {
        free(temp as *mut c_void);
        return ret;
    }

    sysfsfp = fopen(temp, b"w\0".as_ptr() as *const c_char);
    if sysfsfp.is_null() {
        ret = -errno;
        fprintf(stderr, b"failed to open %s\n\0".as_ptr() as *const c_char, temp);
        free(temp as *mut c_void);
        return ret;
    }

    ret = fprintf(sysfsfp, b"%d\0".as_ptr() as *const c_char, val);
    if ret < 0 {
        if fclose(sysfsfp) != 0 {
            perror(b"_write_sysfs_int(): Failed to close dir\0".as_ptr() as *const c_char);
        }
        free(temp as *mut c_void);
        return ret;
    }

    if fclose(sysfsfp) != 0 {
        ret = -errno;
        free(temp as *mut c_void);
        return ret;
    }

    if verify != 0 {
        sysfsfp = fopen(temp, b"r\0".as_ptr() as *const c_char);
        if sysfsfp.is_null() {
            ret = -errno;
            fprintf(stderr, b"failed to open %s\n\0".as_ptr() as *const c_char, temp);
            free(temp as *mut c_void);
            return ret;
        }

        if fscanf(sysfsfp, b"%d\0".as_ptr() as *const c_char, &mut test) != 1 {
            ret = if errno != 0 { -errno } else { -ENODATA };
            if fclose(sysfsfp) != 0 {
                perror(b"_write_sysfs_int(): Failed to close dir\0".as_ptr() as *const c_char);
            }
            free(temp as *mut c_void);
            return ret;
        }

        if fclose(sysfsfp) != 0 {
            ret = -errno;
            free(temp as *mut c_void);
            return ret;
        }

        if test != val {
            fprintf(
                stderr,
                b"Possible failure in int write %d to %s/%s\n\0".as_ptr() as *const c_char,
                val,
                basedir,
                filename,
            );
            ret = -1;
        }
    }

    free(temp as *mut c_void);
    ret
}

/**
 * write_sysfs_int() - write an integer value to a sysfs file
 * @filename: name of the file to write to
 * @basedir: the sysfs directory in which the file is to be found
 * @val: integer value to write to file
 *
 * Returns a value >= 0 on success, otherwise a negative error code.
 **/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_sysfs_int(
    filename: *const c_char,
    basedir: *const c_char,
    val: c_int,
) -> c_int {
    _write_sysfs_int(filename, basedir, val, 0)
}

/**
 * write_sysfs_int_and_verify() - write an integer value to a sysfs file
 *                                and verify
 * @filename: name of the file to write to
 * @basedir: the sysfs directory in which the file is to be found
 * @val: integer value to write to file
 *
 * Returns a value >= 0 on success, otherwise a negative error code.
 **/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_sysfs_int_and_verify(
    filename: *const c_char,
    basedir: *const c_char,
    val: c_int,
) -> c_int {
    _write_sysfs_int(filename, basedir, val, 1)
}

unsafe fn _write_sysfs_string(
    filename: *const c_char,
    basedir: *const c_char,
    val: *const c_char,
    verify: c_int,
) -> c_int {
    let mut ret: c_int = 0;
    let mut sysfsfp: *mut FILE;
    let temp = malloc(strlen(basedir) + strlen(filename) + 2) as *mut c_char;

    if temp.is_null() {
        fprintf(stderr, b"Memory allocation failed\n\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }

    ret = sprintf(temp, b"%s/%s\0".as_ptr() as *const c_char, basedir, filename);
    if ret < 0 {
        free(temp as *mut c_void);
        return ret;
    }

    sysfsfp = fopen(temp, b"w\0".as_ptr() as *const c_char);
    if sysfsfp.is_null() {
        ret = -errno;
        fprintf(stderr, b"Could not open %s\n\0".as_ptr() as *const c_char, temp);
        free(temp as *mut c_void);
        return ret;
    }

    ret = fprintf(sysfsfp, b"%s\0".as_ptr() as *const c_char, val);
    if ret < 0 {
        if fclose(sysfsfp) != 0 {
            perror(b"_write_sysfs_string(): Failed to close dir\0".as_ptr() as *const c_char);
        }
        free(temp as *mut c_void);
        return ret;
    }

    if fclose(sysfsfp) != 0 {
        ret = -errno;
        free(temp as *mut c_void);
        return ret;
    }

    if verify != 0 {
        sysfsfp = fopen(temp, b"r\0".as_ptr() as *const c_char);
        if sysfsfp.is_null() {
            ret = -errno;
            fprintf(stderr, b"Could not open file to verify\n\0".as_ptr() as *const c_char);
            free(temp as *mut c_void);
            return ret;
        }

        if fscanf(sysfsfp, b"%s\0".as_ptr() as *const c_char, temp) != 1 {
            ret = if errno != 0 { -errno } else { -ENODATA };
            if fclose(sysfsfp) != 0 {
                perror(b"_write_sysfs_string(): Failed to close dir\0".as_ptr() as *const c_char);
            }
            free(temp as *mut c_void);
            return ret;
        }

        if fclose(sysfsfp) != 0 {
            ret = -errno;
            free(temp as *mut c_void);
            return ret;
        }

        if strcmp(temp, val) != 0 {
            fprintf(
                stderr,
                b"Possible failure in string write of %s Should be %s written to %s/%s\n\0"
                    .as_ptr() as *const c_char,
                temp,
                val,
                basedir,
                filename,
            );
            ret = -1;
        }
    }

    free(temp as *mut c_void);

    ret
}

/**
 * write_sysfs_string_and_verify() - string write, readback and verify
 * @filename: name of file to write to
 * @basedir: the sysfs directory in which the file is to be found
 * @val: the string to write
 *
 * Returns a value >= 0 on success, otherwise a negative error code.
 **/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_sysfs_string_and_verify(
    filename: *const c_char,
    basedir: *const c_char,
    val: *const c_char,
) -> c_int {
    _write_sysfs_string(filename, basedir, val, 1)
}

/**
 * write_sysfs_string() - write string to a sysfs file
 * @filename: name of file to write to
 * @basedir: the sysfs directory in which the file is to be found
 * @val: the string to write
 *
 * Returns a value >= 0 on success, otherwise a negative error code.
 **/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_sysfs_string(
    filename: *const c_char,
    basedir: *const c_char,
    val: *const c_char,
) -> c_int {
    _write_sysfs_string(filename, basedir, val, 0)
}

/**
 * read_sysfs_posint() - read an integer value from file
 * @filename: name of file to read from
 * @basedir: the sysfs directory in which the file is to be found
 *
 * Returns the read integer value >= 0 on success, otherwise a negative error
 * code.
 **/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_sysfs_posint(
    filename: *const c_char,
    basedir: *const c_char,
) -> c_int {
    let mut ret: c_int;
    let mut sysfsfp: *mut FILE;
    let temp = malloc(strlen(basedir) + strlen(filename) + 2) as *mut c_char;

    if temp.is_null() {
        fprintf(stderr, b"Memory allocation failed\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }

    ret = sprintf(temp, b"%s/%s\0".as_ptr() as *const c_char, basedir, filename);
    if ret < 0 {
        free(temp as *mut c_void);
        return ret;
    }

    sysfsfp = fopen(temp, b"r\0".as_ptr() as *const c_char);
    if sysfsfp.is_null() {
        ret = -errno;
        free(temp as *mut c_void);
        return ret;
    }

    errno = 0;
    if fscanf(sysfsfp, b"%d\n\0".as_ptr() as *const c_char, &mut ret) != 1 {
        ret = if errno != 0 { -errno } else { -ENODATA };
        if fclose(sysfsfp) != 0 {
            perror(b"read_sysfs_posint(): Failed to close dir\0".as_ptr() as *const c_char);
        }
        free(temp as *mut c_void);
        return ret;
    }

    if fclose(sysfsfp) != 0 {
        ret = -errno;
    }

    free(temp as *mut c_void);

    ret
}

/**
 * read_sysfs_float() - read a float value from file
 * @filename: name of file to read from
 * @basedir: the sysfs directory in which the file is to be found
 * @val: output the read float value
 *
 * Returns a value >= 0 on success, otherwise a negative error code.
 **/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_sysfs_float(
    filename: *const c_char,
    basedir: *const c_char,
    val: *mut c_float,
) -> c_int {
    let mut ret: c_int = 0;
    let mut sysfsfp: *mut FILE;
    let temp = malloc(strlen(basedir) + strlen(filename) + 2) as *mut c_char;

    if temp.is_null() {
        fprintf(stderr, b"Memory allocation failed\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }

    ret = sprintf(temp, b"%s/%s\0".as_ptr() as *const c_char, basedir, filename);
    if ret < 0 {
        free(temp as *mut c_void);
        return ret;
    }

    sysfsfp = fopen(temp, b"r\0".as_ptr() as *const c_char);
    if sysfsfp.is_null() {
        ret = -errno;
        free(temp as *mut c_void);
        return ret;
    }

    errno = 0;
    if fscanf(sysfsfp, b"%f\n\0".as_ptr() as *const c_char, val) != 1 {
        ret = if errno != 0 { -errno } else { -ENODATA };
        if fclose(sysfsfp) != 0 {
            perror(b"read_sysfs_float(): Failed to close dir\0".as_ptr() as *const c_char);
        }
        free(temp as *mut c_void);
        return ret;
    }

    if fclose(sysfsfp) != 0 {
        ret = -errno;
    }

    free(temp as *mut c_void);

    ret
}

/**
 * read_sysfs_string() - read a string from file
 * @filename: name of file to read from
 * @basedir: the sysfs directory in which the file is to be found
 * @str: output the read string
 *
 * Returns a value >= 0 on success, otherwise a negative error code.
 **/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_sysfs_string(
    filename: *const c_char,
    basedir: *const c_char,
    str_: *mut c_char,
) -> c_int {
    let mut ret: c_int = 0;
    let mut sysfsfp: *mut FILE;
    let temp = malloc(strlen(basedir) + strlen(filename) + 2) as *mut c_char;

    if temp.is_null() {
        fprintf(stderr, b"Memory allocation failed\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }

    ret = sprintf(temp, b"%s/%s\0".as_ptr() as *const c_char, basedir, filename);
    if ret < 0 {
        free(temp as *mut c_void);
        return ret;
    }

    sysfsfp = fopen(temp, b"r\0".as_ptr() as *const c_char);
    if sysfsfp.is_null() {
        ret = -errno;
        free(temp as *mut c_void);
        return ret;
    }

    errno = 0;
    if fscanf(sysfsfp, b"%s\n\0".as_ptr() as *const c_char, str_) != 1 {
        ret = if errno != 0 { -errno } else { -ENODATA };
        if fclose(sysfsfp) != 0 {
            perror(b"read_sysfs_string(): Failed to close dir\0".as_ptr() as *const c_char);
        }
        free(temp as *mut c_void);
        return ret;
    }

    if fclose(sysfsfp) != 0 {
        ret = -errno;
    }

    free(temp as *mut c_void);

    ret
}
