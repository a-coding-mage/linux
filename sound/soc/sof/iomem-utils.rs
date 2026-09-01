// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018-2022 Intel Corporation
//
// Author: Keyon Jie <yang.jie@linux.intel.com>
//

// C dependencies:
// #include <linux/io-64-nonatomic-lo-hi.h>
// #include <linux/platform_device.h>
// #include <linux/unaligned.h>
// #include <sound/soc.h>
// #include <sound/sof.h>
// #include "sof-priv.h"
// #include "ops.h"

use core::ffi::c_void;

pub type u8 = core::ffi::c_uchar;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub type size_t = usize;

#[repr(C)]
pub struct snd_sof_dev {
    pub bar: [*mut c_void; 0],
    pub mailbox_bar: usize,
}

#[repr(C)]
pub enum snd_sof_fw_blk_type {
    __Incomplete,
}

extern "C" {
    fn writel(value: u32, addr: *mut c_void);
    fn readl(addr: *mut c_void) -> u32;
    fn writeq(value: u64, addr: *mut c_void);
    fn readq(addr: *mut c_void) -> u64;
    fn memcpy_toio(dest: *mut c_void, src: *const c_void, count: size_t);
    fn memcpy_fromio(dest: *mut c_void, src: *const c_void, count: size_t);
    fn snd_sof_dsp_get_bar_index(
        sdev: *mut snd_sof_dev,
        blk_type: snd_sof_fw_blk_type,
    ) -> core::ffi::c_int;
    fn __iowrite32_copy(to: *mut c_void, from: *const c_void, count: size_t);
    fn ioread32(addr: *const c_void) -> u32;
    fn iowrite32(value: u32, addr: *mut c_void);
}

/*
 * Register IO
 *
 * The sof_io_xyz() wrappers are typically referenced in snd_sof_dsp_ops
 * structures and cannot be inlined.
 */

#[no_mangle]
pub unsafe extern "C" fn sof_io_write(
    sdev: *mut snd_sof_dev,
    addr: *mut c_void,
    value: u32,
) {
    let _ = sdev;
    writel(value, addr);
}
// EXPORT_SYMBOL(sof_io_write);

#[no_mangle]
pub unsafe extern "C" fn sof_io_read(sdev: *mut snd_sof_dev, addr: *mut c_void) -> u32 {
    let _ = sdev;
    return readl(addr);
}
// EXPORT_SYMBOL(sof_io_read);

#[no_mangle]
pub unsafe extern "C" fn sof_io_write64(
    sdev: *mut snd_sof_dev,
    addr: *mut c_void,
    value: u64,
) {
    let _ = sdev;
    writeq(value, addr);
}
// EXPORT_SYMBOL(sof_io_write64);

#[no_mangle]
pub unsafe extern "C" fn sof_io_read64(sdev: *mut snd_sof_dev, addr: *mut c_void) -> u64 {
    let _ = sdev;
    return readq(addr);
}
// EXPORT_SYMBOL(sof_io_read64);

/*
 * IPC Mailbox IO
 */

#[no_mangle]
pub unsafe extern "C" fn sof_mailbox_write(
    sdev: *mut snd_sof_dev,
    offset: u32,
    message: *mut c_void,
    bytes: size_t,
) {
    let dest: *mut c_void =
        (*sdev).bar[(*sdev).mailbox_bar].cast::<u8>().add(offset as usize).cast::<c_void>();

    memcpy_toio(dest, message, bytes);
}
// EXPORT_SYMBOL(sof_mailbox_write);

#[no_mangle]
pub unsafe extern "C" fn sof_mailbox_read(
    sdev: *mut snd_sof_dev,
    offset: u32,
    message: *mut c_void,
    bytes: size_t,
) {
    let src: *mut c_void =
        (*sdev).bar[(*sdev).mailbox_bar].cast::<u8>().add(offset as usize).cast::<c_void>();

    memcpy_fromio(message, src, bytes);
}
// EXPORT_SYMBOL(sof_mailbox_read);

/*
 * Memory copy.
 */

#[no_mangle]
pub unsafe extern "C" fn sof_block_write(
    sdev: *mut snd_sof_dev,
    blk_type: snd_sof_fw_blk_type,
    offset: u32,
    src: *mut c_void,
    size: size_t,
) -> core::ffi::c_int {
    let bar: core::ffi::c_int = snd_sof_dsp_get_bar_index(sdev, blk_type);
    let src_byte: *const u8 = src.cast::<u8>();
    let dest: *mut c_void;
    let affected_mask: u32;
    let mut tmp: u32;
    let m: core::ffi::c_int;
    let n: core::ffi::c_int;

    if bar < 0 {
        return bar;
    }

    dest = (*sdev).bar[bar as usize]
        .cast::<u8>()
        .add(offset as usize)
        .cast::<c_void>();

    m = (size / 4) as core::ffi::c_int;
    n = (size % 4) as core::ffi::c_int;

    /* __iowrite32_copy use 32bit size values so divide by 4 */
    __iowrite32_copy(dest, src, m as size_t);

    if n != 0 {
        affected_mask = (1u32 << (8 * n as u32)).wrapping_sub(1);

        /* first read the 32bit data of dest, then change affected
         * bytes, and write back to dest. For unaffected bytes, it
         * should not be changed
         */
        tmp = ioread32(dest.cast::<u8>().add((m * 4) as usize).cast::<c_void>());
        tmp &= !affected_mask;

        tmp |= *(src_byte.add((m * 4) as usize).cast::<u32>()) & affected_mask;
        iowrite32(
            tmp,
            dest.cast::<u8>().add((m * 4) as usize).cast::<c_void>(),
        );
    }

    return 0;
}
// EXPORT_SYMBOL(sof_block_write);

#[no_mangle]
pub unsafe extern "C" fn sof_block_read(
    sdev: *mut snd_sof_dev,
    blk_type: snd_sof_fw_blk_type,
    offset: u32,
    dest: *mut c_void,
    size: size_t,
) -> core::ffi::c_int {
    let bar: core::ffi::c_int = snd_sof_dsp_get_bar_index(sdev, blk_type);

    if bar < 0 {
        return bar;
    }

    memcpy_fromio(
        dest,
        (*sdev).bar[bar as usize]
            .cast::<u8>()
            .add(offset as usize)
            .cast::<c_void>(),
        size,
    );

    return 0;
}
// EXPORT_SYMBOL(sof_block_read);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
