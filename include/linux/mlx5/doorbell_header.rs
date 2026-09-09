/*
 * Copyright (c) 2013-2015, Mellanox Technologies. All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses. You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

pub const MLX5_BF_OFFSET: usize = 0x800;
pub const MLX5_CQ_DOORBELL: usize = 0x20;

/* Assume that we can just write a 64-bit doorbell atomically. s390
 * actually doesn't have writeq() but S/390 systems don't even have
 * PCI so we won't worry about it.
 *
 * Note that the write is not atomic on 32-bit systems! In contrast to 64-bit
 * ones, it requires proper locking. mlx5_write64 doesn't do any locking, so use
 * it at your own discretion, protected by some kind of lock on 32 bits.
 *
 * TODO: use write{q,l}_relaxed()
 */

unsafe extern "C" {
    fn __raw_writeq(val: u64, dest: *mut core::ffi::c_void);
    fn __raw_writel(val: u32, dest: *mut core::ffi::c_void);
}

#[inline]
pub unsafe fn mlx5_write64(val: *const u32, dest: *mut core::ffi::c_void) {
    #[cfg(target_pointer_width = "64")]
    {
        // C source: __raw_writeq(*(u64 *)val, dest)
        let value = core::ptr::read_unaligned(val as *const u64);
        __raw_writeq(value, dest);
    }

    #[cfg(not(target_pointer_width = "64"))]
    {
        // C source: __raw_writel((__force u32) val[0], dest)
        __raw_writel(core::ptr::read(val), dest);
        // C source: __raw_writel((__force u32) val[1], dest + 4)
        __raw_writel(core::ptr::read(val.add(1)), dest.byte_add(4));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
