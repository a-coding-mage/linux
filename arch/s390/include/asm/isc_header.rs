/* SPDX-License-Identifier: GPL-2.0 */

/*
 * I/O interruption subclasses used by drivers.
 * Please add all used iscs here so that it is possible to distribute
 * isc usage between drivers.
 * Reminder: 0 is highest priority, 7 lowest.
 */
pub const MAX_ISC: u32 = 7;

/* Regular I/O interrupts. */
pub const IO_SCH_ISC: u32 = 3; /* regular I/O subchannels */
pub const CONSOLE_ISC: u32 = 1; /* console I/O subchannel */
pub const EADM_SCH_ISC: u32 = 4; /* EADM subchannels */
pub const CHSC_SCH_ISC: u32 = 7; /* CHSC subchannels */
pub const VFIO_CCW_ISC: u32 = IO_SCH_ISC; /* VFIO-CCW I/O subchannels */

/* Adapter interrupts. */
pub const QDIO_AIRQ_ISC: u32 = IO_SCH_ISC; /* I/O subchannel in qdio mode */
pub const PCI_ISC: u32 = 2; /* PCI I/O subchannels */
pub const GAL_ISC: u32 = 5; /* GIB alert */
pub const AP_ISC: u32 = 6; /* adjunct processor (crypto) devices */

/* Functions for registration of I/O interruption subclasses */
unsafe extern "C" {
    pub fn isc_register(isc: u32);
    pub fn isc_unregister(isc: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
