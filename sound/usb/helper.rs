// SPDX-License-Identifier: GPL-2.0-or-later
/*
 */

// Linux kernel USB audio helper functions
// Requires: linux/init.h, linux/slab.h, linux/usb.h, usbaudio.h, helper.h, quirks.h

extern "C" {
    fn combine_word(bytes: *const u8) -> u32;
    fn combine_triple(bytes: *const u8) -> u32;
    fn combine_quad(bytes: *const u8) -> u32;
    fn usb_pipe_type_check(dev: *mut usb_device, pipe: u32) -> i32;
    fn kmemdup(src: *const core::ffi::c_void, len: usize, flags: u32) -> *mut core::ffi::c_void;
    fn usb_control_msg(
        dev: *mut usb_device,
        pipe: u32,
        request: u8,
        requesttype: u8,
        value: u16,
        index: u16,
        data: *mut core::ffi::c_void,
        size: u16,
        timeout: i32,
    ) -> i32;
    fn kfree(objp: *const core::ffi::c_void);
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn snd_usb_ctl_msg_quirk(
        dev: *mut usb_device,
        pipe: u32,
        request: u8,
        requesttype: u8,
        value: u16,
        index: u16,
        data: *mut core::ffi::c_void,
        size: u16,
    );
    fn snd_usb_find_desc(
        descstart: *mut core::ffi::c_void,
        desclen: i32,
        after: *mut core::ffi::c_void,
        dtype: u8,
    ) -> *mut core::ffi::c_void;
    fn snd_usb_get_speed(dev: *mut usb_device) -> i32;
    fn get_endpoint(alts: *mut usb_host_interface, ep: i32) -> *mut usb_endpoint_descriptor;
    fn usb_ifnum_to_if(dev: *mut usb_device, ifnum: i32) -> *mut usb_interface;
    fn usb_altnum_to_altsetting(
        intf: *mut usb_interface,
        alt: i32,
    ) -> *mut usb_host_interface;
}

// Type declarations - defined in kernel headers
#[repr(C)]
pub struct usb_device {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct usb_interface {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct usb_host_interface {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct usb_endpoint_descriptor {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_usb_audio {
    pub dev: *mut usb_device,
    pub ctrl_intf: *mut usb_host_interface,
    pub num_intf_to_ctrl: i32,
    pub intf_to_ctrl: [IntfToCtrl; 128], // MAX_CARD_INTERFACES
}

#[repr(C)]
pub struct IntfToCtrl {
    pub interface: i32,
    pub ctrl_intf: *mut usb_host_interface,
}

// Constants from kernel headers
const USB_DIR_IN: u8 = 0x80;
const USB_DT_CS_INTERFACE: u8 = 0x24;
const USB_SPEED_HIGH: i32 = 3;
const USB_SPEED_SUPER: i32 = 4;
const USB_SPEED_SUPER_PLUS: i32 = 5;
const USB_CTRL_GET_TIMEOUT: i32 = 3000;
const USB_CTRL_SET_TIMEOUT: i32 = 3000;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const GFP_KERNEL: u32 = 0xd0;
const MAX_CARD_INTERFACES: i32 = 128;

// combine bytes and get an integer value
pub fn snd_usb_combine_bytes(bytes: *const u8, size: i32) -> u32 {
    unsafe {
        match size {
            1 => *bytes as u32,
            2 => combine_word(bytes),
            3 => combine_triple(bytes),
            4 => combine_quad(bytes),
            _ => 0,
        }
    }
}

// parse descriptor buffer and return the pointer starting the given
// descriptor type.
pub fn snd_usb_find_desc(
    descstart: *mut core::ffi::c_void,
    desclen: i32,
    after: *mut core::ffi::c_void,
    dtype: u8,
) -> *mut core::ffi::c_void {
    unsafe {
        let mut p = descstart as *const u8;
        let end = (descstart as *const u8).offset(desclen as isize);

        while p < end {
            if *p < 2 {
                return core::ptr::null_mut();
            }
            let next = p.offset(*p as isize);
            if next > end {
                return core::ptr::null_mut();
            }
            if *(p.offset(1)) == dtype && (after.is_null() || (p as *const u8) > (after as *const u8)) {
                return p as *mut core::ffi::c_void;
            }
            p = next;
        }
        core::ptr::null_mut()
    }
}

// find a class-specified interface descriptor with the given subtype.
pub fn snd_usb_find_csint_desc(
    buffer: *mut core::ffi::c_void,
    buflen: i32,
    after: *mut core::ffi::c_void,
    dsubtype: u8,
) -> *mut core::ffi::c_void {
    unsafe {
        let mut p = after;

        loop {
            p = snd_usb_find_desc(buffer, buflen, p, USB_DT_CS_INTERFACE);
            if p.is_null() {
                break;
            }
            let p_slice = p as *const u8;
            if *p_slice >= 3 && *(p_slice.offset(2)) == dsubtype {
                return p;
            }
        }
        core::ptr::null_mut()
    }
}

// Wrapper for usb_control_msg().
// Allocates a temp buffer to prevent dmaing from/to the stack.
pub fn snd_usb_ctl_msg(
    dev: *mut usb_device,
    pipe: u32,
    request: u8,
    requesttype: u8,
    value: u16,
    index: u16,
    data: *mut core::ffi::c_void,
    size: u16,
) -> i32 {
    unsafe {
        let mut err: i32;
        let mut buf: *mut core::ffi::c_void = core::ptr::null_mut();
        let timeout: i32;

        if usb_pipe_type_check(dev, pipe) != 0 {
            return -EINVAL;
        }

        if size as i32 > 0 {
            buf = kmemdup(data, size as usize, GFP_KERNEL);
            if buf.is_null() {
                return -ENOMEM;
            }
        }

        if (requesttype & USB_DIR_IN) != 0 {
            timeout = USB_CTRL_GET_TIMEOUT;
        } else {
            timeout = USB_CTRL_SET_TIMEOUT;
        }

        err = usb_control_msg(dev, pipe, request, requesttype, value, index, buf, size, timeout);

        if size as i32 > 0 {
            memcpy(data, buf, size as usize);
            kfree(buf);
        }

        snd_usb_ctl_msg_quirk(dev, pipe, request, requesttype, value, index, data, size);

        err
    }
}

pub fn snd_usb_parse_datainterval(
    chip: *const snd_usb_audio,
    alts: *mut usb_host_interface,
) -> u8 {
    unsafe {
        match snd_usb_get_speed((*chip).dev) {
            USB_SPEED_HIGH | USB_SPEED_SUPER | USB_SPEED_SUPER_PLUS => {
                let endpoint = get_endpoint(alts, 0);
                let binterval = (*endpoint).bInterval;
                if binterval >= 1 && binterval <= 4 {
                    return (binterval - 1) as u8;
                }
            }
            _ => {}
        }
        0
    }
}

pub fn snd_usb_get_host_interface(
    chip: *const snd_usb_audio,
    ifnum: i32,
    altsetting: i32,
) -> *mut usb_host_interface {
    unsafe {
        let iface = usb_ifnum_to_if((*chip).dev, ifnum);
        if iface.is_null() {
            return core::ptr::null_mut();
        }
        usb_altnum_to_altsetting(iface, altsetting)
    }
}

pub fn snd_usb_add_ctrl_interface_link(
    chip: *mut snd_usb_audio,
    ifnum: i32,
    ctrlif: i32,
) -> i32 {
    unsafe {
        let dev = (*chip).dev;

        if (*chip).num_intf_to_ctrl >= MAX_CARD_INTERFACES {
            // dev_info(&dev->dev, "Too many interfaces assigned to the single USB-audio card\n");
            return -EINVAL;
        }

        // find audiocontrol interface
        let iface = usb_ifnum_to_if(dev, ctrlif);
        let host_iface = &mut (*(iface as *mut usb_interface)).altsetting as *mut usb_host_interface;

        (*chip).intf_to_ctrl[(*chip).num_intf_to_ctrl as usize].interface = ifnum;
        (*chip).intf_to_ctrl[(*chip).num_intf_to_ctrl as usize].ctrl_intf = host_iface;
        (*chip).num_intf_to_ctrl += 1;

        0
    }
}

pub fn snd_usb_find_ctrl_interface(
    chip: *const snd_usb_audio,
    ifnum: i32,
) -> *mut usb_host_interface {
    unsafe {
        let mut i = 0;
        while i < (*chip).num_intf_to_ctrl {
            if (*chip).intf_to_ctrl[i as usize].interface == ifnum {
                return (*chip).intf_to_ctrl[i as usize].ctrl_intf;
            }
            i += 1;
        }

        // Fallback to first audiocontrol interface
        (*chip).ctrl_intf
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
