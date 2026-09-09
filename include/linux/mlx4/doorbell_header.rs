/*
 * Copyright (c) 2004 Topspin Communications.  All rights reserved.
 * Copyright (c) 2005 Sun Microsystems, Inc. All rights reserved.
 * Copyright (c) 2005 Mellanox Technologies. All rights reserved.
 *
 * This software is available under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the OpenIB.org BSD
 * license below:
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

// Dependencies supplied by the surrounding Linux translation.
pub type __be32 = u32;
pub enum spinlock_t {}

pub const MLX4_SEND_DOORBELL: u32 = 0x14;
pub const MLX4_CQ_DOORBELL: u32 = 0x20;

#[cfg(target_pointer_width = "64")]
pub macro MLX4_DECLARE_DOORBELL_LOCK($name:ident) {}

#[cfg(target_pointer_width = "64")]
pub macro MLX4_INIT_DOORBELL_LOCK($ptr:expr) {{ }}

#[cfg(target_pointer_width = "64")]
pub macro MLX4_GET_DOORBELL_LOCK($ptr:expr) { core::ptr::null_mut::<spinlock_t>() };

#[cfg(target_pointer_width = "64")]
pub unsafe fn mlx4_write64(
    val: *const __be32,
    dest: *mut core::ffi::c_void,
    _doorbell_lock: *mut spinlock_t,
) {
    unsafe { __raw_writeq((val as *const u64).read(), dest) };
}

#[cfg(target_pointer_width = "32")]
pub macro MLX4_DECLARE_DOORBELL_LOCK($name:ident) { spinlock_t $name; }

#[cfg(target_pointer_width = "32")]
pub macro MLX4_INIT_DOORBELL_LOCK($ptr:expr) { spin_lock_init($ptr) }

#[cfg(target_pointer_width = "32")]
pub macro MLX4_GET_DOORBELL_LOCK($ptr:expr) { $ptr };

#[cfg(target_pointer_width = "32")]
pub unsafe fn mlx4_write64(
    val: *const __be32,
    dest: *mut core::ffi::c_void,
    doorbell_lock: *mut spinlock_t,
) {
    let mut flags: usize = 0;

    unsafe {
        spin_lock_irqsave(doorbell_lock, &mut flags);
        __raw_writel(val.read(), dest);
        __raw_writel(val.add(1).read(), (dest as *mut u8).add(4) as *mut core::ffi::c_void);
        spin_unlock_irqrestore(doorbell_lock, flags);
    }
}

unsafe extern "C" {
    fn __raw_writeq(value: u64, dest: *mut core::ffi::c_void);
    fn __raw_writel(value: u32, dest: *mut core::ffi::c_void);
    fn spin_lock_init(ptr: *mut spinlock_t);
    fn spin_lock_irqsave(ptr: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(ptr: *mut spinlock_t, flags: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
