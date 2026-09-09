/*
 * dvb_net.h
 *
 * Copyright (C) 2001 Ralph Metzler for convergence integrated media GmbH
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * as published by the Free Software Foundation; either version 2.1
 * of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

// C dependencies supplied by other translated headers.
use crate::{dvb_adapter, dvb_device, dmx_demux, mutex, net_device};

pub const DVB_NET_DEVICES_MAX: usize = 10;

// CONFIG_DVB_NET selects the full implementation; this translation preserves
// both branches as Rust cfg alternatives.

#[cfg(CONFIG_DVB_NET)]
#[repr(C)]
pub struct dvb_net {
    pub dvbdev: *mut dvb_device,
    pub device: [*mut net_device; DVB_NET_DEVICES_MAX],
    pub state: [::core::ffi::c_int; DVB_NET_DEVICES_MAX],
    pub exit: u32,
    pub demux: *mut dmx_demux,
    pub ioctl_mutex: mutex,
    pub remove_mutex: mutex,
}

#[cfg(CONFIG_DVB_NET)]
extern "C" {
    pub fn dvb_net_init(
        adap: *mut dvb_adapter,
        dvbnet: *mut dvb_net,
        dmxdemux: *mut dmx_demux,
    ) -> ::core::ffi::c_int;

    pub fn dvb_net_release(dvbnet: *mut dvb_net);
}

#[cfg(not(CONFIG_DVB_NET))]
#[repr(C)]
pub struct dvb_net {
    pub dvbdev: *mut dvb_device,
}

#[cfg(not(CONFIG_DVB_NET))]
#[inline]
pub unsafe fn dvb_net_release(_dvbnet: *mut dvb_net) {}

#[cfg(not(CONFIG_DVB_NET))]
#[inline]
pub unsafe fn dvb_net_init(
    _adap: *mut dvb_adapter,
    _dvbnet: *mut dvb_net,
    _dmx: *mut dmx_demux,
) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
