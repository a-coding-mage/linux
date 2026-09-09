// SPDX-License-Identifier: GPL-2.0
// External declarations corresponding to the Linux, libc, and BPF headers used by the C source.

use std::ffi::{c_char, c_int, c_long, c_void, CString};
use std::ptr;

const MAX_INDEX: usize = 64;
const MAX_STARS: usize = 38;

unsafe extern "C" {
    fn bpf_num_possible_cpus() -> u32;
    fn bpf_obj_get(pathname: *const c_char) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *const c_char;
    static mut stderr: *mut c_void;
    fn __errno_location() -> *mut c_int;
}

unsafe fn stars(str_: *mut c_char, val: c_long, max: c_long, width: c_int) {
    let mut i: c_int = 0;

    while i < (width * val / max) - 1 && i < width - 1 {
        *str_.offset(i as isize) = b'*' as c_char;
        i += 1;
    }
    if val > max {
        *str_.offset((i - 1) as isize) = b'+' as c_char;
    }
    *str_.offset(i as isize) = 0;
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let nr_cpus = bpf_num_possible_cpus() as usize;
    let map_filename = CString::new("/sys/fs/bpf/tc/globals/lwt_len_hist_map").unwrap();
    let mut values = vec![0u64; nr_cpus];
    let mut sum: u64;
    let mut max_value: u64 = 0;
    let mut data = [0u64; MAX_INDEX];
    let mut key: u64 = 0;
    let mut next_key: u64 = 0;
    let mut max_key: u64 = 0;
    let mut starstr = [0 as c_char; MAX_STARS];
    let map_fd = bpf_obj_get(map_filename.as_ptr());

    if map_fd < 0 {
        let errno = *__errno_location();
        let format = CString::new("bpf_obj_get(%s): %s(%d)\n").unwrap();
        fprintf(
            stderr,
            format.as_ptr(),
            map_filename.as_ptr(),
            strerror(errno),
            errno,
        );
        return -1;
    }

    while bpf_map_get_next_key(
        map_fd,
        &key as *const u64 as *const c_void,
        &mut next_key as *mut u64 as *mut c_void,
    ) == 0
    {
        if next_key >= MAX_INDEX as u64 {
            let format = CString::new("Key %lu out of bounds\n").unwrap();
            fprintf(stderr, format.as_ptr(), next_key);
            continue;
        }

        bpf_map_lookup_elem(
            map_fd,
            &next_key as *const u64 as *const c_void,
            values.as_mut_ptr() as *mut c_void,
        );

        sum = 0;
        for i in 0..nr_cpus {
            sum += values[i];
        }

        data[next_key as usize] = sum;
        if sum != 0 && next_key > max_key {
            max_key = next_key;
        }

        if sum > max_value {
            max_value = sum;
        }

        key = next_key;
    }

    let mut i: u64 = 1;
    while i <= max_key + 1 {
        stars(
            starstr.as_mut_ptr(),
            data[(i - 1) as usize] as c_long,
            max_value as c_long,
            MAX_STARS as c_int,
        );
        let format = CString::new("%8ld -> %-8ld : %-8ld |%-*s|\n").unwrap();
        printf(
            format.as_ptr(),
            ((1u64 << i) >> 1) as c_long,
            ((1u64 << i) - 1) as c_long,
            data[(i - 1) as usize] as c_long,
            MAX_STARS as c_int,
            starstr.as_ptr(),
        );
        i += 1;
    }

    close(map_fd);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
