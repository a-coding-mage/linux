/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *
 * envctrl.h: Definitions for access to the i2c environment
 *            monitoring on Ultrasparc systems.
 *
 * Copyright (C) 1998  Eddie C. Dost  (ecd@skynet.be)
 * Copyright (C) 2000  Vinh Truong  (vinh.truong@eng.sun.com)
 * VT - Add all ioctl commands and environment status definitions
 * VT - Add application note
 */

// Application note:
//
// The driver supports 4 operations: open(), close(), ioctl(), read()
// The device name is /dev/envctrl.
// Below is sample usage:
//
//     fd = open("/dev/envtrl", O_RDONLY);
//     if (ioctl(fd, ENVCTRL_READ_SHUTDOWN_TEMPERATURE, 0) < 0)
//             printf("error\n");
//     ret = read(fd, buf, 10);
//     close(fd);
//
// Notice in the case of cpu voltage and temperature, the default is
// cpu0.  If we need to know the info of cpu1, cpu2, cpu3, we need to
// pass in cpu number in ioctl() last parameter.  For example, to
// get the voltage of cpu2:
//
//     ioctlbuf[0] = 2;
//     if (ioctl(fd, ENVCTRL_READ_CPU_VOLTAGE, ioctlbuf) < 0)
//             printf("error\n");
//     ret = read(fd, buf, 10);
//
// All the return values are in ascii.  So check read return value
// and do appropriate conversions in your application.

// IOCTL commands

// Note: these commands reflect possible monitor features.
// Some boards choose to support some of the features only.
// `_IOR` is supplied by the Linux ioctl dependency.
pub const ENVCTRL_RD_CPU_TEMPERATURE: u32 = _IOR('p', 0x40, core::ffi::c_int);
pub const ENVCTRL_RD_CPU_VOLTAGE: u32 = _IOR('p', 0x41, core::ffi::c_int);
pub const ENVCTRL_RD_FAN_STATUS: u32 = _IOR('p', 0x42, core::ffi::c_int);
pub const ENVCTRL_RD_WARNING_TEMPERATURE: u32 = _IOR('p', 0x43, core::ffi::c_int);
pub const ENVCTRL_RD_SHUTDOWN_TEMPERATURE: u32 = _IOR('p', 0x44, core::ffi::c_int);
pub const ENVCTRL_RD_VOLTAGE_STATUS: u32 = _IOR('p', 0x45, core::ffi::c_int);
pub const ENVCTRL_RD_SCSI_TEMPERATURE: u32 = _IOR('p', 0x46, core::ffi::c_int);
pub const ENVCTRL_RD_ETHERNET_TEMPERATURE: u32 = _IOR('p', 0x47, core::ffi::c_int);
pub const ENVCTRL_RD_MTHRBD_TEMPERATURE: u32 = _IOR('p', 0x48, core::ffi::c_int);

pub const ENVCTRL_RD_GLOBALADDRESS: u32 = _IOR('p', 0x49, core::ffi::c_int);

// Read return values for a voltage status request.
pub const ENVCTRL_VOLTAGE_POWERSUPPLY_GOOD: u8 = 0x01;
pub const ENVCTRL_VOLTAGE_BAD: u8 = 0x02;
pub const ENVCTRL_POWERSUPPLY_BAD: u8 = 0x03;
pub const ENVCTRL_VOLTAGE_POWERSUPPLY_BAD: u8 = 0x04;

// Read return values for a fan status request.
// A failure match means either the fan fails or
// the fan is not connected.  Some boards have optional
// connectors to connect extra fans.
//
// There are maximum 8 monitor fans.  Some are cpu fans
// some are system fans.  The mask below only indicates
// fan by order number.
// Below is a sample application:
//
//     if (ioctl(fd, ENVCTRL_READ_FAN_STATUS, 0) < 0) {
//             printf("ioctl fan failed\n");
//     }
//     if (read(fd, rslt, 1) <= 0) {
//             printf("error or fan not monitored\n");
//     } else {
//             if (rslt[0] == ENVCTRL_ALL_FANS_GOOD) {
//                     printf("all fans good\n");
//             } else if (rslt[0] == ENVCTRL_ALL_FANS_BAD) {
//             printf("all fans bad\n");
//             } else {
//             if (rslt[0] & ENVCTRL_FAN0_FAILURE_MASK) {
//                     printf("fan 0 failed or not connected\n");
//             }
//     }
//     ......

pub const ENVCTRL_ALL_FANS_GOOD: u8 = 0x00;
pub const ENVCTRL_FAN0_FAILURE_MASK: u8 = 0x01;
pub const ENVCTRL_FAN1_FAILURE_MASK: u8 = 0x02;
pub const ENVCTRL_FAN2_FAILURE_MASK: u8 = 0x04;
pub const ENVCTRL_FAN3_FAILURE_MASK: u8 = 0x08;
pub const ENVCTRL_FAN4_FAILURE_MASK: u8 = 0x10;
pub const ENVCTRL_FAN5_FAILURE_MASK: u8 = 0x20;
pub const ENVCTRL_FAN6_FAILURE_MASK: u8 = 0x40;
pub const ENVCTRL_FAN7_FAILURE_MASK: u8 = 0x80;
pub const ENVCTRL_ALL_FANS_BAD: u8 = 0xFF;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
