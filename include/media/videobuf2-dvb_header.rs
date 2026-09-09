/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by the surrounding translation unit:
// media/dvbdev.h, media/dmxdev.h, media/dvb_demux.h, media/dvb_net.h,
// media/dvb_frontend.h, and media/videobuf2-v4l2.h.

use core::ffi::{c_char, c_short, c_void};

// We don't actually need to include media-device.h here.
#[repr(C)]
pub struct media_device;

// TODO: This header file should be replaced with videobuf2-core.h.
// Currently, vb2_thread is not a stuff of videobuf2-core,
// since vb2_thread has many dependencies on videobuf2-v4l2.

#[repr(C)]
pub struct vb2_dvb {
    /* filling that the job of the driver */
    pub name: *mut c_char,
    pub frontend: *mut dvb_frontend,
    pub dvbq: vb2_queue,

    /* vb2-dvb state info */
    pub lock: mutex,
    pub nfeeds: core::ffi::c_int,

    /* vb2_dvb_(un)register manages this */
    pub demux: dvb_demux,
    pub dmxdev: dmxdev,
    pub fe_hw: dmx_frontend,
    pub fe_mem: dmx_frontend,
    pub net: dvb_net,
}

#[repr(C)]
pub struct vb2_dvb_frontend {
    pub felist: list_head,
    pub id: core::ffi::c_int,
    pub dvb: vb2_dvb,
}

#[repr(C)]
pub struct vb2_dvb_frontends {
    pub felist: list_head,
    pub lock: mutex,
    pub adapter: dvb_adapter,
    pub active_fe_id: core::ffi::c_int, /* Indicates which frontend in the felist is in use */
    pub gate: core::ffi::c_int, /* Frontend with gate control 0=!MFE,1=fe0,2=fe1 etc */
}

unsafe extern "C" {
    pub fn vb2_dvb_register_bus(
        f: *mut vb2_dvb_frontends,
        r#module: *mut module,
        adapter_priv: *mut c_void,
        device: *mut device,
        mdev: *mut media_device,
        adapter_nr: *mut c_short,
        mfe_shared: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn vb2_dvb_unregister_bus(f: *mut vb2_dvb_frontends);

    pub fn vb2_dvb_alloc_frontend(
        f: *mut vb2_dvb_frontends,
        id: core::ffi::c_int,
    ) -> *mut vb2_dvb_frontend;

    pub fn vb2_dvb_dealloc_frontends(f: *mut vb2_dvb_frontends);

    pub fn vb2_dvb_get_frontend(
        f: *mut vb2_dvb_frontends,
        id: core::ffi::c_int,
    ) -> *mut vb2_dvb_frontend;

    pub fn vb2_dvb_find_frontend(
        f: *mut vb2_dvb_frontends,
        p: *mut dvb_frontend,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
