/* Copyright (c) 2017 Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// C dependencies translated as external Rust dependencies:
// <linux/bpf.h>, <linux/version.h>, <bpf/bpf_helpers.h>

#[unsafe(link_section = "cgroup/dev")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut bpf_cgroup_dev_ctx) -> i32 {
    let type_: i16 = ((*ctx).access_type & 0xFFFF) as i16;

    // C conditional preserved: #ifdef DEBUG
    #[cfg(DEBUG)]
    {
        let access: i16 = ((*ctx).access_type >> 16) as i16;
        let mut fmt: [u8; 13] = *b"  %d:%d    \n\0";

        match type_ as i32 {
            BPF_DEVCG_DEV_BLOCK => {
                fmt[0] = b'b';
            }
            BPF_DEVCG_DEV_CHAR => {
                fmt[0] = b'c';
            }
            _ => {
                fmt[0] = b'?';
            }
        }

        if (access as i32 & BPF_DEVCG_ACC_READ) != 0 {
            fmt[8] = b'r';
        }

        if (access as i32 & BPF_DEVCG_ACC_WRITE) != 0 {
            fmt[9] = b'w';
        }

        if (access as i32 & BPF_DEVCG_ACC_MKNOD) != 0 {
            fmt[10] = b'm';
        }

        bpf_trace_printk(
            fmt.as_mut_ptr() as *mut i8,
            fmt.len() as u32,
            (*ctx).major,
            (*ctx).minor,
        );
    }

    /* Allow access to /dev/null and /dev/urandom.
     * Forbid everything else.
     */
    if (*ctx).major != 1 || type_ as i32 != BPF_DEVCG_DEV_CHAR {
        return 0;
    }

    match (*ctx).minor {
        3 => {
            /* 1:3 /dev/null */
            return 1;
        }
        9 => {
            /* 1:9 /dev/urandom */
            return 1;
        }
        _ => {}
    }

    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";
