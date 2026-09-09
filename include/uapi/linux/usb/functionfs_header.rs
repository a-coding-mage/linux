/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const FUNCTIONFS_DESCRIPTORS_MAGIC: u32 = 1;
pub const FUNCTIONFS_STRINGS_MAGIC: u32 = 2;
pub const FUNCTIONFS_DESCRIPTORS_MAGIC_V2: u32 = 3;

#[repr(u32)]
pub enum functionfs_flags {
    FUNCTIONFS_HAS_FS_DESC = 1,
    FUNCTIONFS_HAS_HS_DESC = 2,
    FUNCTIONFS_HAS_SS_DESC = 4,
    FUNCTIONFS_HAS_MS_OS_DESC = 8,
    FUNCTIONFS_VIRTUAL_ADDR = 16,
    FUNCTIONFS_EVENTFD = 32,
    FUNCTIONFS_ALL_CTRL_RECIP = 64,
    FUNCTIONFS_CONFIG0_SETUP = 128,
    FUNCTIONFS_RW_PROXY_EPS = 256,
}

#[repr(C, packed)]
pub struct usb_endpoint_descriptor_no_audio {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bEndpointAddress: __u8,
    pub bmAttributes: __u8,
    pub wMaxPacketSize: __le16,
    pub bInterval: __u8,
}

#[repr(C, packed)]
pub struct usb_dfu_functional_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bmAttributes: __u8,
    pub wDetachTimeOut: __le16,
    pub wTransferSize: __le16,
    pub bcdDFUVersion: __le16,
}

pub const DFU_FUNC_ATT_CAN_DOWNLOAD: u32 = 1 << 0;
pub const DFU_FUNC_ATT_CAN_UPLOAD: u32 = 1 << 1;
pub const DFU_FUNC_ATT_MANIFEST_TOLERANT: u32 = 1 << 2;
pub const DFU_FUNC_ATT_WILL_DETACH: u32 = 1 << 3;

#[repr(C, packed)]
pub struct usb_functionfs_descs_head_v2 {
    pub magic: __le32,
    pub length: __le32,
    pub flags: __le32,
}

#[repr(C, packed)]
pub struct usb_functionfs_descs_head {
    pub magic: __le32,
    pub length: __le32,
    pub fs_count: __le32,
    pub hs_count: __le32,
}

#[repr(C, packed)]
pub struct usb_os_desc_header {
    pub interface: __u8,
    pub dwLength: __le32,
    pub bcdVersion: __le16,
    pub wIndex: __le16,
    pub u: usb_os_desc_header__u,
}

#[repr(C)]
pub union usb_os_desc_header__u {
    pub bCount_Reserved: usb_os_desc_header__u__bCount_Reserved,
    pub wCount: __le16,
}

#[repr(C, packed)]
pub struct usb_os_desc_header__u__bCount_Reserved {
    pub bCount: __u8,
    pub Reserved: __u8,
}

#[repr(C)]
pub struct usb_ext_compat_desc {
    pub bFirstInterfaceNumber: __u8,
    pub Reserved1: __u8,
    pub CompatibleID: [__u8; 8],
    pub SubCompatibleID: [__u8; 8],
    pub Reserved2: [__u8; 6],
}

#[repr(C, packed)]
pub struct usb_ext_prop_desc {
    pub dwSize: __le32,
    pub dwPropertyDataType: __le32,
    pub wPropertyNameLength: __le16,
}

pub const USB_FFS_DMABUF_TRANSFER_MASK: u32 = 0x0;

#[repr(C, packed)]
pub struct usb_ffs_dmabuf_transfer_req {
    pub fd: i32,
    pub flags: __u32,
    pub length: __u64,
}

#[repr(C, packed)]
pub struct usb_functionfs_strings_head {
    pub magic: __le32,
    pub length: __le32,
    pub str_count: __le32,
    pub lang_count: __le32,
}

#[repr(u32)]
pub enum usb_functionfs_event_type {
    FUNCTIONFS_BIND,
    FUNCTIONFS_UNBIND,
    FUNCTIONFS_ENABLE,
    FUNCTIONFS_DISABLE,
    FUNCTIONFS_SETUP,
    FUNCTIONFS_SUSPEND,
    FUNCTIONFS_RESUME,
}

#[repr(C, packed)]
pub struct usb_functionfs_event {
    pub u: usb_functionfs_event__u,
    pub r#type: __u8,
    pub _pad: [__u8; 3],
}

#[repr(C, packed)]
pub union usb_functionfs_event__u {
    pub setup: usb_ctrlrequest,
}

/* Ioctl values depend on the external Linux ioctl encoding definitions. */
pub const FUNCTIONFS_FIFO_STATUS: u32 = _IO('g' as u32, 1);
pub const FUNCTIONFS_FIFO_FLUSH: u32 = _IO('g' as u32, 2);
pub const FUNCTIONFS_CLEAR_HALT: u32 = _IO('g' as u32, 3);
pub const FUNCTIONFS_INTERFACE_REVMAP: u32 = _IO('g' as u32, 128);
pub const FUNCTIONFS_ENDPOINT_REVMAP: u32 = _IO('g' as u32, 129);
pub const FUNCTIONFS_ENDPOINT_DESC: u32 = _IOR('g' as u32, 130, usb_endpoint_descriptor);
pub const FUNCTIONFS_DMABUF_ATTACH: u32 = _IOW('g' as u32, 131, i32);
pub const FUNCTIONFS_DMABUF_DETACH: u32 = _IOW('g' as u32, 132, i32);
pub const FUNCTIONFS_DMABUF_TRANSFER: u32 = _IOW('g' as u32, 133, usb_ffs_dmabuf_transfer_req);
pub const FUNCTIONFS_ENDPOINT_ENABLE_ZLP: u32 = _IOW('g' as u32, 134, __u32);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
