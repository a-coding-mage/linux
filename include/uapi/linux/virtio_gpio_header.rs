/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from <linux/types.h>; the external Linux integer types are
// intentionally referenced here rather than redefined.

/* Virtio GPIO Feature bits */
pub const VIRTIO_GPIO_F_IRQ: u32 = 0;

/* Virtio GPIO request types */
pub const VIRTIO_GPIO_MSG_GET_NAMES: u16 = 0x0001;
pub const VIRTIO_GPIO_MSG_GET_DIRECTION: u16 = 0x0002;
pub const VIRTIO_GPIO_MSG_SET_DIRECTION: u16 = 0x0003;
pub const VIRTIO_GPIO_MSG_GET_VALUE: u16 = 0x0004;
pub const VIRTIO_GPIO_MSG_SET_VALUE: u16 = 0x0005;
pub const VIRTIO_GPIO_MSG_IRQ_TYPE: u16 = 0x0006;

/* Possible values of the status field */
pub const VIRTIO_GPIO_STATUS_OK: u8 = 0x0;
pub const VIRTIO_GPIO_STATUS_ERR: u8 = 0x1;

/* Direction types */
pub const VIRTIO_GPIO_DIRECTION_NONE: u8 = 0x00;
pub const VIRTIO_GPIO_DIRECTION_OUT: u8 = 0x01;
pub const VIRTIO_GPIO_DIRECTION_IN: u8 = 0x02;

/* Virtio GPIO IRQ types */
pub const VIRTIO_GPIO_IRQ_TYPE_NONE: u8 = 0x00;
pub const VIRTIO_GPIO_IRQ_TYPE_EDGE_RISING: u8 = 0x01;
pub const VIRTIO_GPIO_IRQ_TYPE_EDGE_FALLING: u8 = 0x02;
pub const VIRTIO_GPIO_IRQ_TYPE_EDGE_BOTH: u8 = 0x03;
pub const VIRTIO_GPIO_IRQ_TYPE_LEVEL_HIGH: u8 = 0x04;
pub const VIRTIO_GPIO_IRQ_TYPE_LEVEL_LOW: u8 = 0x08;

#[repr(C)]
pub struct virtio_gpio_config {
    pub ngpio: __le16,
    pub padding: [__u8; 2],
    pub gpio_names_size: __le32,
}

/* Virtio GPIO Request / Response */
#[repr(C)]
pub struct virtio_gpio_request {
    pub type_: __le16,
    pub gpio: __le16,
    pub value: __le32,
}

#[repr(C)]
pub struct virtio_gpio_response {
    pub status: __u8,
    pub value: __u8,
}

#[repr(C)]
pub struct virtio_gpio_response_get_names {
    pub status: __u8,
    pub value: [__u8; 0],
}

/* Virtio GPIO IRQ Request / Response */
#[repr(C)]
pub struct virtio_gpio_irq_request {
    pub gpio: __le16,
}

#[repr(C)]
pub struct virtio_gpio_irq_response {
    pub status: __u8,
}

/* Possible values of the interrupt status field */
pub const VIRTIO_GPIO_IRQ_STATUS_INVALID: u8 = 0x0;
pub const VIRTIO_GPIO_IRQ_STATUS_VALID: u8 = 0x1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
