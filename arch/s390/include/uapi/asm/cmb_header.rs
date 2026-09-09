/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by the surrounding UAPI translation: linux/types.h.

/**
 * struct cmbdata - channel measurement block data for user space
 * @size: size of the stored data
 * @elapsed_time: time since last sampling
 * @ssch_rsch_count: number of ssch and rsch
 * @sample_count: number of samples
 * @device_connect_time: time of device connect
 * @function_pending_time: time of function pending
 * @device_disconnect_time: time of device disconnect
 * @control_unit_queuing_time: time of control unit queuing
 * @device_active_only_time: time of device active only
 * @device_busy_time: time of device busy (ext. format)
 * @initial_command_response_time: initial command response time (ext. format)
 *
 * All values are stored as 64 bit for simplicity, especially
 * in 32 bit emulation mode. All time values are normalized to
 * nanoseconds.
 * Currently, two formats are known, which differ by the size of
 * this structure, i.e. the last two members are only set when
 * the extended channel measurement facility (first shipped in
 * z990 machines) is activated.
 * Potentially, more fields could be added, which would result in a
 * new ioctl number.
 */
#[repr(C)]
pub struct cmbdata {
    pub size: u64,
    pub elapsed_time: u64,
    /* basic and extended format: */
    pub ssch_rsch_count: u64,
    pub sample_count: u64,
    pub device_connect_time: u64,
    pub function_pending_time: u64,
    pub device_disconnect_time: u64,
    pub control_unit_queuing_time: u64,
    pub device_active_only_time: u64,
    /* extended format only: */
    pub device_busy_time: u64,
    pub initial_command_response_time: u64,
}

/* enable channel measurement */
// #define BIODASDCMFENABLE _IO(DASD_IOCTL_LETTER, 32)
pub const BIODASDCMFENABLE: u32 = crate::_IO(DASD_IOCTL_LETTER, 32);

/* enable channel measurement */
// #define BIODASDCMFDISABLE _IO(DASD_IOCTL_LETTER, 33)
pub const BIODASDCMFDISABLE: u32 = crate::_IO(DASD_IOCTL_LETTER, 33);

/* read channel measurement data */
// #define BIODASDREADALLCMB _IOWR(DASD_IOCTL_LETTER, 33, struct cmbdata)
pub const BIODASDREADALLCMB: u32 = crate::_IOWR(DASD_IOCTL_LETTER, 33, cmbdata);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
