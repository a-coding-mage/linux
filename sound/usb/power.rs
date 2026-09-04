// SPDX-License-Identifier: GPL-2.0
/*
 *   UAC3 Power Domain state management functions
 */

// Include dependencies:
// - linux/slab.h
// - linux/usb.h
// - linux/usb/audio.h
// - linux/usb/audio-v2.h
// - linux/usb/audio-v3.h
// - usbaudio.h
// - helper.h
// - power.h

use std::ffi::c_void;
use std::ptr;

extern "C" {
    type snd_usb_power_domain;
    type usb_host_interface;
    type usb_device;
    type snd_usb_audio;
    type uac3_power_domain_descriptor;

    fn kzalloc_obj(obj: *const ()) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snd_usb_find_csint_desc(
        extra: *const u8,
        extralen: usize,
        p: *mut c_void,
        desctype: u32,
    ) -> *mut c_void;
    fn snd_usb_validate_audio_desc(desc: *const c_void, version: u32) -> i32;
    fn snd_usb_ctrl_intf(iface: *const usb_host_interface) -> u32;
    fn snd_usb_ctl_msg(
        dev: *mut usb_device,
        pipe: u32,
        request: u8,
        requesttype: u8,
        value: u16,
        index: u16,
        data: *mut u8,
        size: usize,
    ) -> i32;
    fn usb_rcvctrlpipe(dev: *mut usb_device, endpoint: u32) -> u32;
    fn usb_sndctrlpipe(dev: *mut usb_device, endpoint: u32) -> u32;
    fn dev_err(dev: *const c_void, fmt: *const i8, ...);
    fn dev_dbg(dev: *const c_void, fmt: *const i8, ...);
    fn udelay(usecs: u32);
}

// External constants (from linux/usb/audio-v3.h and other headers)
const UAC3_POWER_DOMAIN: u32 = 0x04; // UAC3_POWER_DOMAIN descriptor type
const UAC3_AC_POWER_DOMAIN_CONTROL: u32 = 0x09;
const UAC3_PD_STATE_D0: u8 = 0x00;
const UAC3_PD_STATE_D1: u8 = 0x01;
const UAC3_PD_STATE_D2: u8 = 0x02;
const UAC2_CS_CUR: u8 = 0x01;
const USB_RECIP_INTERFACE: u8 = 0x01;
const USB_TYPE_CLASS: u8 = 0x20;
const USB_DIR_IN: u8 = 0x80;
const USB_DIR_OUT: u8 = 0x00;
const EINVAL: i32 = 22;

// Offset definitions (from struct snd_usb_power_domain layout)
// These are used to access fields in the opaque snd_usb_power_domain type
const PD_ID_OFFSET: usize = 0;
const PD_D1D0_REC_OFFSET: usize = 4;
const PD_D2D0_REC_OFFSET: usize = 6;
const CTRL_IFACE_OFFSET: usize = 8;

// Helper to access struct fields via raw pointers
unsafe fn get_pd_id(pd: *mut snd_usb_power_domain) -> u8 {
    *(pd as *const u8).add(PD_ID_OFFSET)
}

unsafe fn set_pd_id(pd: *mut snd_usb_power_domain, value: u8) {
    *(pd as *mut u8).add(PD_ID_OFFSET) = value;
}

unsafe fn get_pd_d1d0_rec(pd: *const snd_usb_power_domain) -> u16 {
    *(pd as *const u16).add(PD_D1D0_REC_OFFSET / 2)
}

unsafe fn set_pd_d1d0_rec(pd: *mut snd_usb_power_domain, value: u16) {
    *(pd as *mut u16).add(PD_D1D0_REC_OFFSET / 2) = value;
}

unsafe fn get_pd_d2d0_rec(pd: *const snd_usb_power_domain) -> u16 {
    *(pd as *const u16).add(PD_D2D0_REC_OFFSET / 2)
}

unsafe fn set_pd_d2d0_rec(pd: *mut snd_usb_power_domain, value: u16) {
    *(pd as *mut u16).add(PD_D2D0_REC_OFFSET / 2) = value;
}

unsafe fn get_ctrl_iface(pd: *const snd_usb_power_domain) -> *const usb_host_interface {
    *(pd as *const *const usb_host_interface).add(CTRL_IFACE_OFFSET / 8)
}

unsafe fn set_ctrl_iface(pd: *mut snd_usb_power_domain, value: *const usb_host_interface) {
    *(pd as *mut *const usb_host_interface).add(CTRL_IFACE_OFFSET / 8) = value;
}

pub unsafe extern "C" fn snd_usb_find_power_domain(
    ctrl_iface: *mut usb_host_interface,
    id: u8,
) -> *mut snd_usb_power_domain {
    let mut pd: *mut snd_usb_power_domain;
    let mut p: *mut c_void;

    pd = kzalloc_obj(ptr::null()) as *mut snd_usb_power_domain;
    if pd.is_null() {
        return ptr::null_mut();
    }

    p = ptr::null_mut();
    loop {
        p = snd_usb_find_csint_desc(
            (*ctrl_iface).extra as *const u8,
            (*ctrl_iface).extralen,
            p,
            UAC3_POWER_DOMAIN,
        );
        if p.is_null() {
            break;
        }

        if snd_usb_validate_audio_desc(p, 3) == 0 {
            continue;
        }

        let pd_desc = p as *const uac3_power_domain_descriptor;
        let mut i: i32;

        i = 0;
        while i < (*pd_desc).bNrEntities as i32 {
            if (*pd_desc).baEntityID[i as usize] == id {
                set_pd_id(pd, (*pd_desc).bPowerDomainID);
                set_pd_d1d0_rec(pd, le16_to_cpu((*pd_desc).waRecoveryTime1));
                set_pd_d2d0_rec(pd, le16_to_cpu((*pd_desc).waRecoveryTime2));
                set_ctrl_iface(pd, ctrl_iface as *const usb_host_interface);
                return pd;
            }
            i += 1;
        }
    }

    kfree(pd as *mut c_void);
    ptr::null_mut()
}

pub unsafe extern "C" fn snd_usb_power_domain_set(
    chip: *mut snd_usb_audio,
    pd: *mut snd_usb_power_domain,
    state: u8,
) -> i32 {
    let dev: *mut usb_device = (*chip).dev;
    let mut current_state: u8;
    let mut err: i32;
    let idx: u32;

    idx = snd_usb_ctrl_intf((*pd).ctrl_iface as *const usb_host_interface)
        | ((get_pd_id(pd) as u32) << 8);

    err = snd_usb_ctl_msg(
        (*chip).dev,
        usb_rcvctrlpipe((*chip).dev, 0),
        UAC2_CS_CUR,
        USB_RECIP_INTERFACE | USB_TYPE_CLASS | USB_DIR_IN,
        (UAC3_AC_POWER_DOMAIN_CONTROL as u16) << 8,
        idx as u16,
        &mut current_state as *mut u8,
        std::mem::size_of::<u8>(),
    );
    if err < 0 {
        dev_err(
            &(*dev).dev as *const c_void,
            b"Can't get UAC3 power state for id %d\n\0".as_ptr() as *const i8,
            get_pd_id(pd),
        );
        return err;
    }

    if current_state == state {
        dev_dbg(
            &(*dev).dev as *const c_void,
            b"UAC3 power domain id %d already in state %d\n\0".as_ptr() as *const i8,
            get_pd_id(pd),
            state,
        );
        return 0;
    }

    err = snd_usb_ctl_msg(
        (*chip).dev,
        usb_sndctrlpipe((*chip).dev, 0),
        UAC2_CS_CUR,
        USB_RECIP_INTERFACE | USB_TYPE_CLASS | USB_DIR_OUT,
        (UAC3_AC_POWER_DOMAIN_CONTROL as u16) << 8,
        idx as u16,
        &state as *const u8 as *mut u8,
        std::mem::size_of::<u8>(),
    );
    if err < 0 {
        dev_err(
            &(*dev).dev as *const c_void,
            b"Can't set UAC3 power state to %d for id %d\n\0".as_ptr() as *const i8,
            state,
            get_pd_id(pd),
        );
        return err;
    }

    if state == UAC3_PD_STATE_D0 {
        match current_state {
            UAC3_PD_STATE_D2 => {
                udelay(get_pd_d2d0_rec(pd).wrapping_mul(50));
            }
            UAC3_PD_STATE_D1 => {
                udelay(get_pd_d1d0_rec(pd).wrapping_mul(50));
            }
            _ => {
                return -EINVAL;
            }
        }
    }

    dev_dbg(
        &(*dev).dev as *const c_void,
        b"UAC3 power domain id %d change to state %d\n\0".as_ptr() as *const i8,
        get_pd_id(pd),
        state,
    );

    0
}

// Helper function for le16_to_cpu (assuming little-endian conversion)
// This is typically a macro/inline that converts from little-endian to CPU byte order
unsafe fn le16_to_cpu(value: u16) -> u16 {
    value.to_le()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
