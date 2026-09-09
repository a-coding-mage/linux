/* SPDX-License-Identifier: BSD-3-Clause */

// Translated from the UAPI header <linux/virtio_types.h>.

/* Feature bits */
pub const VIRTIO_BT_F_VND_HCI: u32 = 0; /* Indicates vendor command support */
pub const VIRTIO_BT_F_MSFT_EXT: u32 = 1; /* Indicates MSFT vendor support */
pub const VIRTIO_BT_F_AOSP_EXT: u32 = 2; /* Indicates AOSP vendor support */
pub const VIRTIO_BT_F_CONFIG_V2: u32 = 3; /* Use second version configuration */

#[repr(i32)]
pub enum virtio_bt_config_type {
    VIRTIO_BT_CONFIG_TYPE_PRIMARY = 0,
}

#[repr(i32)]
pub enum virtio_bt_config_vendor {
    VIRTIO_BT_CONFIG_VENDOR_NONE = 0,
    VIRTIO_BT_CONFIG_VENDOR_ZEPHYR = 1,
    VIRTIO_BT_CONFIG_VENDOR_INTEL = 2,
    VIRTIO_BT_CONFIG_VENDOR_REALTEK = 3,
}

#[repr(C, packed)]
pub struct virtio_bt_config {
    pub type_: u8,
    pub vendor: u16,
    pub msft_opcode: u16,
}

#[repr(C)]
pub struct virtio_bt_config_v2 {
    pub type_: u8,
    pub alignment: u8,
    pub vendor: u16,
    pub msft_opcode: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
