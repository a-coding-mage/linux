/* SPDX-License-Identifier: GPL-2.0 */

// Under __KERNEL__, kernel_ulong_t is typedef'd as unsigned long.
pub type kernel_ulong_t = usize;

pub const HID_ANY_ID: u32 = !0u32;
pub const HID_BUS_ANY: u16 = 0xffffu16;
pub const HID_GROUP_ANY: u16 = 0x0000u16;

#[repr(C)]
pub struct hid_device_id {
    pub bus: u16,
    pub group: u16,
    pub vendor: u32,
    pub product: u32,
    pub driver_data: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
