/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ni_labpc ISA DMA support.
 */

// The C header's IS_ENABLED(CONFIG_COMEDI_NI_LABPC_ISADMA) condition is
// represented here by the corresponding Rust configuration feature.

use core::ffi::c_uint;

#[repr(C)]
pub struct comedi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct comedi_subdevice {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_COMEDI_NI_LABPC_ISADMA")]
extern "C" {
    pub fn labpc_init_dma_chan(dev: *mut comedi_device, dma_chan: c_uint);
    pub fn labpc_free_dma_chan(dev: *mut comedi_device);
    pub fn labpc_setup_dma(dev: *mut comedi_device, s: *mut comedi_subdevice);
    pub fn labpc_drain_dma(dev: *mut comedi_device);
    pub fn labpc_handle_dma_status(dev: *mut comedi_device);
}

#[cfg(not(feature = "CONFIG_COMEDI_NI_LABPC_ISADMA"))]
#[inline]
pub unsafe fn labpc_init_dma_chan(_dev: *mut comedi_device, _dma_chan: c_uint) {}

#[cfg(not(feature = "CONFIG_COMEDI_NI_LABPC_ISADMA"))]
#[inline]
pub unsafe fn labpc_free_dma_chan(_dev: *mut comedi_device) {}

#[cfg(not(feature = "CONFIG_COMEDI_NI_LABPC_ISADMA"))]
#[inline]
pub unsafe fn labpc_setup_dma(_dev: *mut comedi_device, _s: *mut comedi_subdevice) {}

#[cfg(not(feature = "CONFIG_COMEDI_NI_LABPC_ISADMA"))]
#[inline]
pub unsafe fn labpc_drain_dma(_dev: *mut comedi_device) {}

#[cfg(not(feature = "CONFIG_COMEDI_NI_LABPC_ISADMA"))]
#[inline]
pub unsafe fn labpc_handle_dma_status(_dev: *mut comedi_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
