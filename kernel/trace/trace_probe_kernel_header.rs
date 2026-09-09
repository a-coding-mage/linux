/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This depends on trace_probe.h, but can not include it due to
 * the way trace_probe_tmpl.h is used by trace_kprobe.c and trace_eprobe.c.
 * Which means that any other user must include trace_probe.h before including
 * this file.
 */

/* Return the length of string -- including null terminal byte */
#[inline]
pub unsafe fn fetch_store_strlen_user(addr: usize) -> i32 {
    let uaddr = addr as *const core::ffi::c_void;
    strnlen_user_nofault(uaddr, MAX_STRING_SIZE)
}

/* Return the length of string -- including null terminal byte */
#[inline]
pub unsafe fn fetch_store_strlen(addr: usize) -> i32 {
    let mut ret: i32;
    let mut len: i32 = 0;
    let mut c: u8 = 0;

    #[cfg(CONFIG_ARCH_HAS_NON_OVERLAPPING_ADDRESS_SPACE)]
    {
        if addr < TASK_SIZE {
            return fetch_store_strlen_user(addr);
        }
    }

    loop {
        ret = copy_from_kernel_nofault(
            &mut c as *mut u8 as *mut core::ffi::c_void,
            (addr as *mut u8).add(len as usize) as *mut core::ffi::c_void,
            1,
        );
        len += 1;
        if !(c != 0 && ret == 0 && len < MAX_STRING_SIZE) {
            break;
        }
    }

    if ret < 0 { ret } else { len }
}

#[inline]
pub unsafe fn set_data_loc(ret: i32, dest: *mut core::ffi::c_void,
                           dest_data: *mut core::ffi::c_void,
                           base: *mut core::ffi::c_void) {
    let ret = if ret < 0 { 0 } else { ret };
    *(dest as *mut u32) = make_data_loc(
        ret,
        (dest_data as *mut u8).offset_from(base as *mut u8) as usize,
    );
}

/*
 * Fetch a null-terminated string from user. Caller MUST set *(u32 *)buf
 * with max length and relative data location.
 */
#[inline]
pub unsafe fn fetch_store_string_user(addr: usize, dest: *mut core::ffi::c_void,
                                      base: *mut core::ffi::c_void) -> i64 {
    let uaddr = addr as *const core::ffi::c_void;
    let maxlen = get_loc_len(*(dest as *mut u32));
    let dest_data: *mut core::ffi::c_void;

    if unlikely(maxlen == 0) {
        return -ENOMEM as i64;
    }

    dest_data = get_loc_data(dest, base);
    let ret = strncpy_from_user_nofault(dest_data, uaddr, maxlen);
    set_data_loc(ret as i32, dest, dest_data, base);
    ret
}

/*
 * Fetch a null-terminated string. Caller MUST set *(u32 *)buf with max
 * length and relative data location.
 */
#[inline]
pub unsafe fn fetch_store_string(addr: usize, dest: *mut core::ffi::c_void,
                                  base: *mut core::ffi::c_void) -> i64 {
    let maxlen = get_loc_len(*(dest as *mut u32));
    let dest_data: *mut core::ffi::c_void;

    #[cfg(CONFIG_ARCH_HAS_NON_OVERLAPPING_ADDRESS_SPACE)]
    {
        if addr < TASK_SIZE {
            return fetch_store_string_user(addr, dest, base);
        }
    }

    if unlikely(maxlen == 0) {
        return -ENOMEM as i64;
    }

    dest_data = get_loc_data(dest, base);

    /*
     * Try to get string again, since the string can be changed while
     * probing.
     */
    let ret = strncpy_from_kernel_nofault(
        dest_data,
        addr as *mut core::ffi::c_void,
        maxlen,
    );
    set_data_loc(ret as i32, dest, dest_data, base);
    ret
}

#[inline]
pub unsafe fn probe_mem_read_user(dest: *mut core::ffi::c_void,
                                  src: *mut core::ffi::c_void,
                                  size: usize) -> i32 {
    let uaddr = src as *const core::ffi::c_void;
    copy_from_user_nofault(dest, uaddr, size)
}

#[inline]
pub unsafe fn probe_mem_read(dest: *mut core::ffi::c_void,
                             src: *mut core::ffi::c_void,
                             size: usize) -> i32 {
    #[cfg(CONFIG_ARCH_HAS_NON_OVERLAPPING_ADDRESS_SPACE)]
    {
        if src as usize < TASK_SIZE {
            return probe_mem_read_user(dest, src, size);
        }
    }
    copy_from_kernel_nofault(dest, src, size)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
