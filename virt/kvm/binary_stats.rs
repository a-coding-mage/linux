// SPDX-License-Identifier: GPL-2.0-only
/*
 * KVM binary statistics interface implementation
 *
 * Copyright 2021 Google LLC
 */

/*
 * Dependencies from:
 * - <linux/kvm_host.h>
 * - <linux/kvm.h>
 * - <linux/errno.h>
 * - <linux/uaccess.h>
 */

use core::ffi::{c_char, c_void};

pub type ssize_t = isize;
pub type size_t = usize;
pub type loff_t = i64;

extern "C" {
    pub static KVM_STATS_NAME_SIZE: size_t;
    pub static EFAULT: i32;

    pub fn copy_to_user(to: *mut c_char, from: *const c_void, n: size_t) -> usize;
}

#[repr(C)]
pub struct kvm_stats_header {
    pub flags: u32,
    pub name_size: u32,
    pub num_desc: u32,
    pub id_offset: u32,
    pub desc_offset: u32,
    pub data_offset: u32,
}

#[repr(C)]
pub struct kvm_stats_desc {
    pub flags: u32,
    pub exponent: i16,
    pub size: u16,
    pub offset: u32,
    pub bucket_size: u32,
    pub name: [c_char; 0],
}

/**
 * kvm_stats_read() - Common function to read from the binary statistics
 * file descriptor.
 *
 * @id: identification string of the stats
 * @header: stats header for a vm or a vcpu
 * @desc: start address of an array of stats descriptors for a vm or a vcpu
 * @stats: start address of stats data block for a vm or a vcpu
 * @size_stats: the size of stats data block pointed by @stats
 * @user_buffer: start address of userspace buffer
 * @size: requested read size from userspace
 * @offset: the start position from which the content will be read for the
 *          corresponding vm or vcp file descriptor
 *
 * The file content of a vm/vcpu file descriptor is now defined as below:
 * +-------------+
 * |   Header    |
 * +-------------+
 * |  id string  |
 * +-------------+
 * | Descriptors |
 * +-------------+
 * | Stats Data  |
 * +-------------+
 * Although this function allows userspace to read any amount of data (as long
 * as in the limit) from any position, the typical usage would follow below
 * steps:
 * 1. Read header from offset 0. Get the offset of descriptors and stats data
 *    and some other necessary information. This is a one-time work for the
 *    lifecycle of the corresponding vm/vcpu stats fd.
 * 2. Read id string from its offset. This is a one-time work for the lifecycle
 *    of the corresponding vm/vcpu stats fd.
 * 3. Read descriptors from its offset and discover all the stats by parsing
 *    descriptors. This is a one-time work for the lifecycle of the
 *    corresponding vm/vcpu stats fd.
 * 4. Periodically read stats data from its offset using pread.
 *
 * Return: the number of bytes that has been successfully read
 */
#[no_mangle]
pub unsafe extern "C" fn kvm_stats_read(
    id: *mut c_char,
    header: *const kvm_stats_header,
    desc: *const kvm_stats_desc,
    stats: *mut c_void,
    size_stats: size_t,
    user_buffer: *mut c_char,
    size: size_t,
    offset: *mut loff_t,
) -> ssize_t {
    let mut len: ssize_t;
    let mut copylen: ssize_t;
    let mut remain: ssize_t = size as ssize_t;
    let size_desc: size_t;
    let size_header: size_t;
    let mut src: *mut c_void;
    let mut pos: loff_t = *offset;
    let mut dest: *mut c_char = user_buffer;

    size_header = core::mem::size_of::<kvm_stats_header>();
    size_desc = (*header).num_desc as size_t * core::mem::size_of::<kvm_stats_desc>();

    len = KVM_STATS_NAME_SIZE as ssize_t
        + size_header as ssize_t
        + size_desc as ssize_t
        + size_stats as ssize_t
        - pos as ssize_t;
    len = core::cmp::min(len, remain);
    if len <= 0 {
        return 0;
    }
    remain = len;

    /*
     * Copy kvm stats header.
     * The header is the first block of content userspace usually read out.
     * The pos is 0 and the copylen and remain would be the size of header.
     * The copy of the header would be skipped if offset is larger than the
     * size of header. That usually happens when userspace reads stats
     * descriptors and stats data.
     */
    copylen = size_header as ssize_t - pos as ssize_t;
    copylen = core::cmp::min(copylen, remain);
    if copylen > 0 {
        src = (header as *mut c_void).byte_add(pos as usize);
        if copy_to_user(dest, src as *const c_void, copylen as size_t) != 0 {
            return -(EFAULT as ssize_t);
        }
        remain -= copylen;
        pos += copylen as loff_t;
        dest = dest.add(copylen as usize);
    }

    /*
     * Copy kvm stats header id string.
     * The id string is unique for every vm/vcpu, which is stored in kvm
     * and kvm_vcpu structure.
     * The id string is part of the stat header from the perspective of
     * userspace, it is usually read out together with previous constant
     * header part and could be skipped for later descriptors and stats
     * data readings.
     */
    copylen = (*header).id_offset as ssize_t + KVM_STATS_NAME_SIZE as ssize_t - pos as ssize_t;
    copylen = core::cmp::min(copylen, remain);
    if copylen > 0 {
        src = id
            .offset(pos as isize - (*header).id_offset as isize)
            as *mut c_void;
        if copy_to_user(dest, src as *const c_void, copylen as size_t) != 0 {
            return -(EFAULT as ssize_t);
        }
        remain -= copylen;
        pos += copylen as loff_t;
        dest = dest.add(copylen as usize);
    }

    /*
     * Copy kvm stats descriptors.
     * The descriptors copy would be skipped in the typical case that
     * userspace periodically read stats data, since the pos would be
     * greater than the end address of descriptors
     * (header->header.desc_offset + size_desc) causing copylen <= 0.
     */
    copylen = (*header).desc_offset as ssize_t + size_desc as ssize_t - pos as ssize_t;
    copylen = core::cmp::min(copylen, remain);
    if copylen > 0 {
        src = (desc as *mut c_void).byte_offset(pos as isize - (*header).desc_offset as isize);
        if copy_to_user(dest, src as *const c_void, copylen as size_t) != 0 {
            return -(EFAULT as ssize_t);
        }
        remain -= copylen;
        pos += copylen as loff_t;
        dest = dest.add(copylen as usize);
    }

    /* Copy kvm stats values */
    copylen = (*header).data_offset as ssize_t + size_stats as ssize_t - pos as ssize_t;
    copylen = core::cmp::min(copylen, remain);
    if copylen > 0 {
        src = stats.byte_offset(pos as isize - (*header).data_offset as isize);
        if copy_to_user(dest, src as *const c_void, copylen as size_t) != 0 {
            return -(EFAULT as ssize_t);
        }
        pos += copylen as loff_t;
    }

    *offset = pos;
    len
}
