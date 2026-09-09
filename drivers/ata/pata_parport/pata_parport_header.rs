/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *	pata_parport.h	(c) 1997-8  Grant R. Guenther <grant@torque.net>
 *				    Under the terms of the GPL.
 *
 * This file defines the interface for parallel port IDE adapter chip drivers.
 */

// Dependency supplied externally: linux/libata.h

#[repr(C)]
pub struct pi_adapter {
    pub dev: device,
    pub proto: *mut pi_protocol, /* adapter protocol */
    pub port: ::core::ffi::c_int, /* base address of parallel port */
    pub mode: ::core::ffi::c_int, /* transfer mode in use */
    pub delay: ::core::ffi::c_int, /* adapter delay setting */
    pub unit: ::core::ffi::c_int, /* unit number for chained adapters */
    pub saved_r0: ::core::ffi::c_int, /* saved port state */
    pub saved_r2: ::core::ffi::c_int, /* saved port state */
    pub private: ::core::ffi::c_ulong, /* for protocol module */
    pub pardev: *mut pardevice, /* pointer to pardevice */
}

/* registers are addressed as (cont,regr)
 *	cont: 0 for command register file, 1 for control register(s)
 *	regr: 0-7 for register number.
 */

/* macros and functions exported to the protocol modules */
#[macro_export]
macro_rules! delay_p {
    ($pi:expr) => {
        if $pi.delay != 0 { udelay($pi.delay as _) } else { () }
    };
}
#[macro_export]
macro_rules! out_p {
    ($pi:expr, $offs:expr, $byte:expr) => {{ outb($byte, $pi.port + $offs); delay_p!($pi); }};
}
#[macro_export]
macro_rules! in_p {
    ($pi:expr, $offs:expr) => {{ delay_p!($pi); inb($pi.port + $offs) }};
}
#[macro_export]
macro_rules! w0 { ($pi:expr, $byte:expr) => { out_p!($pi, 0, $byte) }; }
#[macro_export]
macro_rules! r0 { ($pi:expr) => { in_p!($pi, 0) }; }
#[macro_export]
macro_rules! w1 { ($pi:expr, $byte:expr) => { out_p!($pi, 1, $byte) }; }
#[macro_export]
macro_rules! r1 { ($pi:expr) => { in_p!($pi, 1) }; }
#[macro_export]
macro_rules! w2 { ($pi:expr, $byte:expr) => { out_p!($pi, 2, $byte) }; }
#[macro_export]
macro_rules! r2 { ($pi:expr) => { in_p!($pi, 2) }; }
#[macro_export]
macro_rules! w3 { ($pi:expr, $byte:expr) => { out_p!($pi, 3, $byte) }; }
#[macro_export]
macro_rules! w4 { ($pi:expr, $byte:expr) => { out_p!($pi, 4, $byte) }; }
#[macro_export]
macro_rules! r4 { ($pi:expr) => { in_p!($pi, 4) }; }
#[macro_export]
macro_rules! w4w { ($pi:expr, $data:expr) => {{ outw($data, $pi.port + 4); delay_p!($pi); }}; }
#[macro_export]
macro_rules! w4l { ($pi:expr, $data:expr) => {{ outl($data, $pi.port + 4); delay_p!($pi); }}; }
#[macro_export]
macro_rules! r4w { ($pi:expr) => {{ delay_p!($pi); inw($pi.port + 4) }}; }
#[macro_export]
macro_rules! r4l { ($pi:expr) => {{ delay_p!($pi); inl($pi.port + 4) }}; }

#[repr(C)]
pub struct pi_protocol {
    pub name: [::core::ffi::c_char; 8],
    pub max_mode: ::core::ffi::c_int,
    pub epp_first: ::core::ffi::c_int, /* modes >= this use 8 ports */
    pub default_delay: ::core::ffi::c_int,
    pub max_units: ::core::ffi::c_int, /* max chained units probed for */
    pub write_regr: Option<unsafe extern "C" fn(*mut pi_adapter, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int)>,
    pub read_regr: Option<unsafe extern "C" fn(*mut pi_adapter, ::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub write_block: Option<unsafe extern "C" fn(*mut pi_adapter, *mut ::core::ffi::c_char, ::core::ffi::c_int)>,
    pub read_block: Option<unsafe extern "C" fn(*mut pi_adapter, *mut ::core::ffi::c_char, ::core::ffi::c_int)>,
    pub connect: Option<unsafe extern "C" fn(*mut pi_adapter)>,
    pub disconnect: Option<unsafe extern "C" fn(*mut pi_adapter)>,
    pub test_port: Option<unsafe extern "C" fn(*mut pi_adapter) -> ::core::ffi::c_int>,
    pub probe_unit: Option<unsafe extern "C" fn(*mut pi_adapter) -> ::core::ffi::c_int>,
    pub test_proto: Option<unsafe extern "C" fn(*mut pi_adapter) -> ::core::ffi::c_int>,
    pub log_adapter: Option<unsafe extern "C" fn(*mut pi_adapter)>,
    pub init_proto: Option<unsafe extern "C" fn(*mut pi_adapter) -> ::core::ffi::c_int>,
    pub release_proto: Option<unsafe extern "C" fn(*mut pi_adapter)>,
    pub owner: *mut module,
    pub driver: device_driver,
    pub sht: scsi_host_template,
}

pub const PATA_PARPORT_SHT: _ = ATA_PIO_SHT;

unsafe extern "C" {
    pub fn pata_parport_register_driver(pr: *mut pi_protocol) -> ::core::ffi::c_int;
    pub fn pata_parport_unregister_driver(pr: *mut pi_protocol);
}

// module_pata_parport_driver() expands to module_driver with the registration callbacks.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
