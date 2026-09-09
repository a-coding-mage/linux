/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Interfaces for vfio-ccw
 *
 * Copyright IBM Corp. 2017
 *
 * Author(s): Dong Jia Shi <bjsdjshi@linux.vnet.ibm.com>
 */

/* C header guard: _VFIO_CCW_H_ */
/* Dependency: linux/types.h */

/* used for START SUBCHANNEL, always present */
#[repr(C, packed)]
pub struct ccw_io_region {
    pub orb_area: [__u8; ORB_AREA_SIZE],
    pub scsw_area: [__u8; SCSW_AREA_SIZE],
    pub irb_area: [__u8; IRB_AREA_SIZE],
    pub ret_code: __u32,
}

pub const ORB_AREA_SIZE: usize = 12;
pub const SCSW_AREA_SIZE: usize = 12;
pub const IRB_AREA_SIZE: usize = 96;

/*
 * used for processing commands that trigger asynchronous actions
 * Note: this is controlled by a capability
 */
pub const VFIO_CCW_ASYNC_CMD_HSCH: __u32 = 1 << 0;
pub const VFIO_CCW_ASYNC_CMD_CSCH: __u32 = 1 << 1;

#[repr(C, packed)]
pub struct ccw_cmd_region {
    pub command: __u32,
    pub ret_code: __u32,
}

/*
 * Used for processing commands that read the subchannel-information block
 * Reading this region triggers a stsch() to hardware
 * Note: this is controlled by a capability
 */
#[repr(C, packed)]
pub struct ccw_schib_region {
    pub schib_area: [__u8; SCHIB_AREA_SIZE],
}

pub const SCHIB_AREA_SIZE: usize = 52;

/*
 * Used for returning a Channel Report Word to userspace.
 * Note: this is controlled by a capability
 */
#[repr(C, packed)]
pub struct ccw_crw_region {
    pub crw: __u32,
    pub pad: __u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
