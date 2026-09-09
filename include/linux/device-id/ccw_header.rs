/* SPDX-License-Identifier: GPL-2.0 */

// Under __KERNEL__, this header includes <linux/types.h> and defines
// kernel_ulong_t as unsigned long. The corresponding Rust dependencies are
// supplied externally.

pub const CCW_DEVICE_ID_MATCH_CU_TYPE: u16 = 0x01;
pub const CCW_DEVICE_ID_MATCH_CU_MODEL: u16 = 0x02;
pub const CCW_DEVICE_ID_MATCH_DEVICE_TYPE: u16 = 0x04;
pub const CCW_DEVICE_ID_MATCH_DEVICE_MODEL: u16 = 0x08;

/* s390 CCW devices */
#[repr(C)]
pub struct ccw_device_id {
    pub match_flags: u16, /* which fields to match against */

    pub cu_type: u16, /* control unit type     */
    pub dev_type: u16, /* device type           */
    pub cu_model: u8, /* control unit model    */
    pub dev_model: u8, /* device model          */

    pub driver_info: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
