/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Parallel SCSI (SPI) transport specific attributes exported to sysfs.
 *
 * Copyright (c) 2003 Silicon Graphics, Inc.  All rights reserved.
 */

/* Dependencies supplied by the surrounding kernel translation. */

use core::ffi::{c_int, c_ulong, c_uchar, c_void};

pub enum scsi_transport_template {}
pub enum scsi_target {}
pub enum scsi_device {}
pub enum Scsi_Host {}
pub enum scsi_cmnd {}
pub enum mutex {}

#[repr(C)]
pub struct spi_transport_attrs {
    pub period: c_int,             /* value in the PPR/SDTR command */
    pub min_period: c_int,
    pub offset: c_int,
    pub max_offset: c_int,
    pub width: u32,                /* unsigned int width:1; 0 - narrow, 1 - wide */
    pub max_width: u32,
    pub iu: u32,                   /* Information Units enabled */
    pub max_iu: u32,
    pub dt: u32,                   /* DT clocking enabled */
    pub qas: u32,                  /* Quick Arbitration and Selection enabled */
    pub max_qas: u32,
    pub wr_flow: u32,              /* Write Flow control enabled */
    pub rd_strm: u32,              /* Read streaming enabled */
    pub rti: u32,                  /* Retain Training Information */
    pub pcomp_en: u32,             /* Precompensation enabled */
    pub hold_mcs: u32,             /* Hold Margin Control Settings */
    pub initial_dv: u32,           /* DV done to this target yet */
    pub flags: c_ulong,            /* flags field for drivers to use */
    /* Device Properties fields */
    pub support_sync: u32,         /* synchronous support */
    pub support_wide: u32,         /* wide support */
    pub support_dt: u32,           /* allows DT phases */
    pub support_dt_only: u32,      /* disallows ST phases */
    pub support_ius: u32,          /* support Information Units */
    pub support_qas: u32,          /* supports quick arbitration and selection */
    /* Private Fields */
    pub dv_pending: u32,           /* Internal flag: DV Requested */
    pub dv_in_progress: u32,       /* Internal: DV started */
    pub dv_mutex: mutex,           /* semaphore to serialise dv */
}

#[repr(C)]
#[derive Copy, Clone, PartialEq, Eq)]
pub enum spi_signal_type {
    SPI_SIGNAL_UNKNOWN = 1,
    SPI_SIGNAL_SE,
    SPI_SIGNAL_LVD,
    SPI_SIGNAL_HVD,
}

#[repr(C)]
pub struct spi_host_attrs {
    pub signalling: spi_signal_type,
}

/* Accessor functions corresponding to the C accessor macros. */
#[inline]
pub unsafe fn spi_period(x: *mut scsi_target) -> c_int {
    (*(x as *mut spi_transport_attrs)).period
}
#[inline]
pub unsafe fn spi_min_period(x: *mut scsi_target) -> c_int { (*(x as *mut spi_transport_attrs)).min_period }
#[inline]
pub unsafe fn spi_offset(x: *mut scsi_target) -> c_int { (*(x as *mut spi_transport_attrs)).offset }
#[inline]
pub unsafe fn spi_max_offset(x: *mut scsi_target) -> c_int { (*(x as *mut spi_transport_attrs)).max_offset }

macro_rules! spi_attr_accessor {
    ($name:ident, $field:ident) => {
        #[inline]
        pub unsafe fn $name(x: *mut scsi_target) -> u32 {
            (*(x as *mut spi_transport_attrs)).$field
        }
    };
}

spi_attr_accessor!(spi_width, width);
spi_attr_accessor!(spi_max_width, max_width);
spi_attr_accessor!(spi_iu, iu);
spi_attr_accessor!(spi_max_iu, max_iu);
spi_attr_accessor!(spi_dt, dt);
spi_attr_accessor!(spi_qas, qas);
spi_attr_accessor!(spi_max_qas, max_qas);
spi_attr_accessor!(spi_wr_flow, wr_flow);
spi_attr_accessor!(spi_rd_strm, rd_strm);
spi_attr_accessor!(spi_rti, rti);
spi_attr_accessor!(spi_pcomp_en, pcomp_en);
spi_attr_accessor!(spi_hold_mcs, hold_mcs);
spi_attr_accessor!(spi_initial_dv, initial_dv);
spi_attr_accessor!(spi_dv_pending, dv_pending);
spi_attr_accessor!(spi_support_sync, support_sync);
spi_attr_accessor!(spi_support_wide, support_wide);
spi_attr_accessor!(spi_support_dt, support_dt);
spi_attr_accessor!(spi_support_dt_only, support_dt_only);
spi_attr_accessor!(spi_support_ius, support_ius);
spi_attr_accessor!(spi_support_qas, support_qas);

#[inline]
pub unsafe fn spi_flags(x: *mut scsi_target) -> c_ulong {
    (*(x as *mut spi_transport_attrs)).flags
}

#[inline]
pub unsafe fn spi_signalling(h: *mut Scsi_Host) -> spi_signal_type {
    (*(h as *mut spi_host_attrs)).signalling
}

/* The functions by which the transport class and the driver communicate */
#[repr(C)]
pub struct spi_function_template {
    pub get_period: Option<unsafe extern "C" fn(*mut scsi_target)>,
    pub set_period: Option<unsafe extern "C" fn(*mut scsi_target, c_int)>,
    pub get_offset: Option<unsafe extern "C" fn(*mut scsi_target)>,
    pub set_offset: Option<unsafe extern "C" fn(*mut scsi_target, c_int)>,
    pub get_width: Option<unsafe extern "C" fn(*mut scsi_target)>,
    pub set_width: Option<unsafe extern "C" fn(*mut scsi_target, c_int)>,
    pub get_iu: Option<unsafe extern "C" fn(*mut scsi_target)>,
    pub set_iu: Option<unsafe extern "C" fn(*mut scsi_target, c_int)>,
    pub get_dt: Option<unsafe extern "C" fn(*mut scsi_target)>,
    pub set_dt: Option<unsafe extern "C" fn(*mut scsi_target, c_int)>,
    pub get_qas: Option<unsafe extern "C" fn(*mut scsi_target)>,
    pub set_qas: Option<unsafe extern "C" fn(*mut scsi_target, c_int)>,
    pub get_wr_flow: Option<unsafe extern "C" fn(*mut scsi_target)>,
    pub set_wr_flow: Option<unsafe extern "C" fn(*mut scsi_target, c_int)>,
    pub get_rd_strm: Option<unsafe extern "C" fn(*mut scsi_target)>,
    pub set_rd_strm: Option<unsafe extern "C" fn(*mut scsi_target, c_int)>,
    pub get_rti: Option<unsafe extern "C" fn(*mut scsi_target)>,
    pub set_rti: Option<unsafe extern "C" fn(*mut scsi_target, c_int)>,
    pub get_pcomp_en: Option<unsafe extern "C" fn(*mut scsi_target)>,
    pub set_pcomp_en: Option<unsafe extern "C" fn(*mut scsi_target, c_int)>,
    pub get_hold_mcs: Option<unsafe extern "C" fn(*mut scsi_target)>,
    pub set_hold_mcs: Option<unsafe extern "C" fn(*mut scsi_target, c_int)>,
    pub get_signalling: Option<unsafe extern "C" fn(*mut Scsi_Host)>,
    pub set_signalling: Option<unsafe extern "C" fn(*mut Scsi_Host, spi_signal_type)>,
    pub deny_binding: Option<unsafe extern "C" fn(*mut scsi_target) -> c_int>,
    /* The driver sets these to tell the transport class it wants the
     * attributes displayed in sysfs. If the show_ flag is not set,
     * the attribute will be private to the transport class. */
    pub show_period: u64,
    pub show_offset: u64,
    pub show_width: u64,
    pub show_iu: u64,
    pub show_dt: u64,
    pub show_qas: u64,
    pub show_wr_flow: u64,
    pub show_rd_strm: u64,
    pub show_rti: u64,
    pub show_pcomp_en: u64,
    pub show_hold_mcs: u64,
}

extern "C" {
    pub fn spi_attach_transport(ft: *mut spi_function_template) -> *mut scsi_transport_template;
    pub fn spi_release_transport(t: *mut scsi_transport_template);
    pub fn spi_schedule_dv_device(sdev: *mut scsi_device);
    pub fn spi_dv_device(sdev: *mut scsi_device);
    pub fn spi_display_xfer_agreement(starget: *mut scsi_target);
    pub fn spi_print_msg(msg: *const c_uchar) -> c_int;
    pub fn spi_populate_width_msg(msg: *mut c_uchar, width: c_int) -> c_int;
    pub fn spi_populate_sync_msg(msg: *mut c_uchar, period: c_int, offset: c_int) -> c_int;
    pub fn spi_populate_ppr_msg(msg: *mut c_uchar, period: c_int, offset: c_int, width: c_int, options: c_int) -> c_int;
    pub fn spi_populate_tag_msg(msg: *mut c_uchar, cmd: *mut scsi_cmnd) -> c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
