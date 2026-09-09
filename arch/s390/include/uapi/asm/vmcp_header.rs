/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright IBM Corp. 2004, 2005
 * Interface implementation for communication with the z/VM control program
 * Version 1.0
 * Author(s): Christian Borntraeger <cborntra@de.ibm.com>
 *
 *
 * z/VMs CP offers the possibility to issue commands via the diagnose code 8
 * this driver implements a character device that issues these commands and
 * returns the answer of CP.
 *
 * The idea of this driver is based on cpint from Neale Ferguson
 */

// The C header includes <linux/ioctl.h>. The constants below are the direct
// Linux ioctl encodings of _IOR/_IOW with an int payload.

pub const VMCP_GETCODE: u32 = 0x8004_1001;
pub const VMCP_SETBUF: u32 = 0x4004_1002;
pub const VMCP_GETSIZE: u32 = 0x8004_1003;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
