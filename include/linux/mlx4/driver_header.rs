/*
 * Copyright (c) 2006 Cisco Systems, Inc.  All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
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

// C dependencies supplied by other translation units:
// <net/devlink.h>, <linux/auxiliary_bus.h>, <linux/notifier.h>,
// and <linux/mlx4/device.h>.

pub const MLX4_ADEV_NAME: &str = "mlx4_core";

#[repr(C)]
pub struct mlx4_dev {
    _private: [u8; 0],
}

pub const MLX4_MAC_MASK: u64 = 0xffffffffffff;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mlx4_dev_event {
    MLX4_DEV_EVENT_CATASTROPHIC_ERROR,
    MLX4_DEV_EVENT_PORT_UP,
    MLX4_DEV_EVENT_PORT_DOWN,
    MLX4_DEV_EVENT_PORT_REINIT,
    MLX4_DEV_EVENT_PORT_MGMT_CHANGE,
    MLX4_DEV_EVENT_SLAVE_INIT,
    MLX4_DEV_EVENT_SLAVE_SHUTDOWN,
}

pub const MLX4_INTFF_BONDING: i32 = 1 << 0;

#[repr(C)]
pub struct mlx4_adrv {
    pub adrv: auxiliary_driver,
    pub protocol: mlx4_protocol,
    pub flags: core::ffi::c_int,
}

extern "C" {
    pub fn mlx4_register_auxiliary_driver(madrv: *mut mlx4_adrv) -> core::ffi::c_int;
    pub fn mlx4_unregister_auxiliary_driver(madrv: *mut mlx4_adrv);

    pub fn mlx4_register_event_notifier(
        dev: *mut mlx4_dev,
        nb: *mut notifier_block,
    ) -> core::ffi::c_int;
    pub fn mlx4_unregister_event_notifier(
        dev: *mut mlx4_dev,
        nb: *mut notifier_block,
    ) -> core::ffi::c_int;

    pub fn mlx4_get_devlink_port(
        dev: *mut mlx4_dev,
        port: core::ffi::c_int,
    ) -> *mut devlink_port;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
