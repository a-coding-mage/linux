// SPDX-License-Identifier: GPL-2.0

// External opaque types referenced from other headers
#[repr(C)]
pub struct usb_device {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct usb_stream_kernel {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct file {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct us122l {
    pub dev: *mut usb_device,
    pub card_index: i32,
    pub stride: i32,
    pub sk: usb_stream_kernel,
    pub mutex: mutex,
    pub first: *mut file,
    pub second_periods_polled: u32,
    pub master: *mut file,
    pub slave: *mut file,
    pub midi_list: list_head,
    pub is_us144: bool,
}

// C macro: #define US122L(c) ((struct us122l *)(c)->private_data)
// Casts the private_data field of a structure to us122l pointer
// Requires c to point to a structure with a void* private_data field (typically snd_card)
#[inline]
pub unsafe fn US122L(c: *mut core::ffi::c_void) -> *mut us122l {
    // TODO: Requires external structure definition with private_data field
    // Placeholder that treats c as pointing to a structure with private_data at known offset
    (*(c as *const *mut us122l))
}

pub const NAME_ALLCAPS: &str = "US-122L";

pub const USB_ID_US122L: u32 = 0x800E;
pub const USB_ID_US144: u32 = 0x800F;
pub const USB_ID_US122MKII: u32 = 0x8021;
pub const USB_ID_US144MKII: u32 = 0x8020;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
