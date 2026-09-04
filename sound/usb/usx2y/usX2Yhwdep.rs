// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Tascam US-X2Y USB soundcards
 *
 * FPGA Loader + ALSA Startup
 *
 * Copyright (c) 2003 by Karsten Wiese <annabellesgarden@yahoo.de>
 */

// Extern crate and module dependencies from Linux kernel:
// linux/interrupt.h, linux/slab.h, linux/usb.h
// sound/core.h, sound/memalloc.h, sound/pcm.h, sound/hwdep.h
// "usx2y.h", "usbusx2y.h", "usX2Yhwdep.h"

use core::ffi::CStr;
use core::mem;
use core::ptr;

// Types and constants from external dependencies
// These declarations reference types/constants defined elsewhere
type vm_fault_t = i32;
type __poll_t = u32;

// Constants
const PAGE_SHIFT: usize = 12; // Typical page size shift (4096 bytes)
const VM_FAULT_SIGBUS: vm_fault_t = 1;
const EPOLLHUP: __poll_t = 0x010;
const EPOLLIN: __poll_t = 0x001;
const US428_SHAREDMEM_PAGES: usize = 4096; // Example page count
const GFP_KERNEL: u32 = 0xd0;
const SNDRV_HWDEP_IFACE_USX2Y: i32 = 12;
const SND_USX2Y_LOADER_ID: *const i8 = b"USX2Y Loader\0".as_ptr() as *const i8;
const NAME_ALLCAPS: *const i8 = b"US-X2Y\0".as_ptr() as *const i8;

// Error codes
const EBUSY: i32 = -16;
const EINVAL: i32 = -22;
const ENODEV: i32 = -19;
const ENOMEM: i32 = -12;

// USB Product IDs
const USB_ID_US122: u16 = 0x8006;
const USB_ID_US224: u16 = 0x8007;
const USB_ID_US428: u16 = 0x8008;

// Device status flags
const USX2Y_STAT_CHIP_INIT: u32 = 0x01;
const USX2Y_STAT_CHIP_HUP: u32 = 0x02;

// MIDI quirk types
const QUIRK_MIDI_FIXED_ENDPOINT: i32 = 3;

const USX2Y_DRIVER_VERSION: u32 = 0x000124;
const USX2Y_TYPE_NUMS: usize = 3;
const USX2Y_TYPE_122: usize = 0;
const USX2Y_TYPE_224: usize = 1;
const USX2Y_TYPE_428: usize = 2;

// External types (from kernel/ALSA)
#[repr(C)]
pub struct page {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct vm_fault {
    pub pgoff: usize,
    pub vma: *mut vm_area_struct,
    pub page: *mut page,
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_start: usize,
    pub vm_end: usize,
    pub vm_ops: *const vm_operations_struct,
    pub vm_private_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct vm_operations_struct {
    pub fault: Option<unsafe extern "C" fn(*mut vm_fault) -> vm_fault_t>,
}

#[repr(C)]
pub struct snd_usb_midi_endpoint_info {
    pub out_ep: u8,
    pub in_ep: u8,
    pub out_cables: u16,
    pub in_cables: u16,
}

#[repr(C)]
pub struct snd_usb_audio_quirk {
    pub vendor_name: *const i8,
    pub product_name: *const i8,
    pub ifnum: i32,
    pub quirk_type: i32,
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct snd_hwdep_dsp_status {
    pub id: [i8; 64],
    pub num_dsps: u32,
    pub chip_ready: u32,
    pub version: u32,
}

#[repr(C)]
pub struct snd_hwdep_dsp_image {
    pub index: u32,
    pub length: usize,
    pub image: *const core::ffi::c_void,
}

#[repr(C)]
pub struct us428ctls_sharedmem {
    pub ctl_snapshot_last: i32,
    pub ctl_snapshot_red: i32,
    // Additional fields not shown
}

#[repr(C)]
pub struct usx2ydev {
    pub dev: *mut usb_device,
    pub chip_status: u32,
    pub us428ctls_sharedmem: *mut us428ctls_sharedmem,
    pub us428ctls_wait_queue_head: wait_queue_head_t,
    pub midi_list: core::ffi::c_void,
}

#[repr(C)]
pub struct usb_device {
    pub descriptor: usb_device_descriptor,
    pub bus: *mut usb_bus,
    pub devnum: i32,
}

#[repr(C)]
pub struct usb_device_descriptor {
    pub idProduct: u16,
}

#[repr(C)]
pub struct usb_bus {
    pub busnum: i32,
}

#[repr(C)]
pub struct usb_interface {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct device {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_hwdep {
    pub private_data: *mut core::ffi::c_void,
    pub iface: i32,
    pub card: *mut snd_card,
    pub name: [i8; 80],
    pub exclusive: i32,
    pub ops: hwdep_ops,
}

#[repr(C)]
pub struct hwdep_ops {
    pub dsp_status: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut snd_hwdep_dsp_status) -> i32>,
    pub dsp_load: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut snd_hwdep_dsp_image) -> i32>,
    pub mmap: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file, *mut vm_area_struct) -> i32>,
    pub poll: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file, *mut poll_table) -> __poll_t>,
}

#[repr(C)]
pub struct file {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct poll_table {
    _opaque: [u8; 0],
}

pub struct wait_queue_head_t {
    _opaque: [u8; 0],
}

// External functions (to be implemented elsewhere)
extern "C" {
    pub fn virt_to_page(addr: *const core::ffi::c_void) -> *mut page;
    pub fn get_page(page: *mut page);
    pub fn memdup_user(from: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    pub fn IS_ERR(ptr: *const core::ffi::c_void) -> i32;
    pub fn PTR_ERR(ptr: *const core::ffi::c_void) -> i32;
    pub fn kfree(ptr: *mut core::ffi::c_void);
    pub fn dev_dbg(dev: *mut device, fmt: *const i8, ...);
    pub fn dev_err(dev: *mut device, fmt: *const i8, ...);
    pub fn poll_wait(filp: *mut file, wait_address: *mut wait_queue_head_t, p: *mut poll_table);
    pub fn vm_flags_set(vma: *mut vm_area_struct, flags: u32);
    pub fn usb_set_interface(dev: *mut usb_device, ifnum: i32, alternate: i32) -> i32;
    pub fn usb_sndbulkpipe(dev: *mut usb_device, endpoint: u32) -> u32;
    pub fn usb_bulk_msg(
        usb_dev: *mut usb_device,
        pipe: u32,
        data: *mut core::ffi::c_void,
        len: usize,
        actual_length: *mut i32,
        timeout: i32,
    ) -> i32;
    pub fn msleep(msecs: u32);
    pub fn usx2y_async_seq04_init(dev: *mut usx2ydev) -> i32;
    pub fn usx2y_in04_init(dev: *mut usx2ydev) -> i32;
    pub fn usx2y_audio_create(card: *mut snd_card) -> i32;
    pub fn usx2y_hwdep_pcm_new(card: *mut snd_card) -> i32;
    pub fn snd_card_register(card: *mut snd_card) -> i32;
    pub fn snd_hwdep_new(
        card: *mut snd_card,
        id: *const i8,
        device: i32,
        rhwdep: *mut *mut snd_hwdep,
    ) -> i32;
    pub fn snd_usbmidi_create(
        card: *mut snd_card,
        iface: *mut usb_interface,
        midi_list: *mut core::ffi::c_void,
        quirk: *const snd_usb_audio_quirk,
    ) -> i32;
    pub fn usb_ifnum_to_if(dev: *mut usb_device, ifnum: i32) -> *mut usb_interface;
    pub fn alloc_pages_exact(size: usize, gfp_mask: u32) -> *mut core::ffi::c_void;
    pub fn le16_to_cpu(val: u16) -> u16;
    pub fn strscpy(dest: *mut i8, src: *const i8, ...) -> usize;
    pub fn sprintf(s: *mut i8, format: *const i8, ...) -> i32;
}

// Helper macro replacements
fn usx2y(card: *mut snd_card) -> *mut usx2ydev {
    // Assuming card->private_data is usx2ydev pointer
    unsafe { ((*card).dev as *mut usx2ydev) }
}

// VM fault handler
unsafe extern "C" fn snd_us428ctls_vm_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let mut offset: usize;
    let mut page: *mut page;
    let mut vaddr: *mut core::ffi::c_void;

    offset = (*vmf).pgoff << PAGE_SHIFT;
    if offset >= US428_SHAREDMEM_PAGES {
        return VM_FAULT_SIGBUS;
    }
    vaddr = ((*vmf).vma as *mut u8).add(offset) as *mut core::ffi::c_void;
    let dev_ptr = ((*(*vmf).vma).vm_private_data as *mut usx2ydev);
    vaddr = ((*dev_ptr).us428ctls_sharedmem as *mut u8).add(offset) as *mut core::ffi::c_void;
    page = virt_to_page(vaddr);
    get_page(page);
    (*vmf).page = page;

    return 0;
}

// VM operations structure
static us428ctls_vm_ops: vm_operations_struct = vm_operations_struct {
    fault: Some(snd_us428ctls_vm_fault),
};

// mmap handler for hwdep device
unsafe extern "C" fn snd_us428ctls_mmap(
    hw: *mut snd_hwdep,
    filp: *mut file,
    area: *mut vm_area_struct,
) -> i32 {
    let size: usize = ((*area).vm_end - (*area).vm_start);
    let us428: *mut usx2ydev = (*hw).private_data as *mut usx2ydev;

    // Device must be fully initialized before allowing mmap
    if ((*us428).chip_status & USX2Y_STAT_CHIP_INIT) == 0 {
        return EBUSY;
    }

    // Check if userspace tries to mmap beyond end of buffer
    if size > US428_SHAREDMEM_PAGES {
        dev_dbg(
            (*(*hw).card).dev,
            b"%s: mmap size %lu > %lu\n\0".as_ptr() as *const i8,
            b"snd_us428ctls_mmap\0".as_ptr() as *const i8,
            size,
            US428_SHAREDMEM_PAGES,
        );
        return EINVAL;
    }

    (*area).vm_ops = &us428ctls_vm_ops;
    vm_flags_set(area, 0x00000400 | 0x00001000); // VM_DONTEXPAND | VM_DONTDUMP
    (*area).vm_private_data = (*hw).private_data;
    return 0;
}

// Poll handler for hwdep device
unsafe extern "C" fn snd_us428ctls_poll(
    hw: *mut snd_hwdep,
    file: *mut file,
    wait: *mut poll_table,
) -> __poll_t {
    let mut mask: __poll_t = 0;
    let us428: *mut usx2ydev = (*hw).private_data as *mut usx2ydev;
    let shm: *mut us428ctls_sharedmem = (*us428).us428ctls_sharedmem;

    if ((*us428).chip_status & USX2Y_STAT_CHIP_HUP) != 0 {
        return EPOLLHUP;
    }

    poll_wait(file, &mut (*us428).us428ctls_wait_queue_head, wait);

    if !shm.is_null() && (*shm).ctl_snapshot_last != (*shm).ctl_snapshot_red {
        mask |= EPOLLIN;
    }

    return mask;
}

// DSP status handler
unsafe extern "C" fn snd_usx2y_hwdep_dsp_status(
    hw: *mut snd_hwdep,
    info: *mut snd_hwdep_dsp_status,
) -> i32 {
    const TYPE_IDS: [*const i8; USX2Y_TYPE_NUMS] = [
        b"us122\0".as_ptr() as *const i8,
        b"us224\0".as_ptr() as *const i8,
        b"us428\0".as_ptr() as *const i8,
    ];

    let us428: *mut usx2ydev = (*hw).private_data as *mut usx2ydev;
    let mut id: i32 = -1;

    let product_id: u16 = le16_to_cpu((*(*us428).dev).descriptor.idProduct);
    match product_id {
        USB_ID_US122 => id = USX2Y_TYPE_122 as i32,
        USB_ID_US224 => id = USX2Y_TYPE_224 as i32,
        USB_ID_US428 => id = USX2Y_TYPE_428 as i32,
        _ => {}
    }

    if id < 0 {
        return ENODEV;
    }

    strscpy(
        (*info).id.as_mut_ptr(),
        TYPE_IDS[id as usize],
    );
    (*info).num_dsps = 2; // 0: Prepad Data, 1: FPGA Code
    if ((*us428).chip_status & USX2Y_STAT_CHIP_INIT) != 0 {
        (*info).chip_ready = 1;
    }
    (*info).version = USX2Y_DRIVER_VERSION;
    return 0;
}

// Create USB MIDI device
unsafe extern "C" fn usx2y_create_usbmidi(card: *mut snd_card) -> i32 {
    static QUIRK_DATA_1: snd_usb_midi_endpoint_info = snd_usb_midi_endpoint_info {
        out_ep: 0x06,
        in_ep: 0x06,
        out_cables: 0x001,
        in_cables: 0x001,
    };

    static QUIRK_1: snd_usb_audio_quirk = snd_usb_audio_quirk {
        vendor_name: b"TASCAM\0".as_ptr() as *const i8,
        product_name: NAME_ALLCAPS,
        ifnum: 0,
        quirk_type: QUIRK_MIDI_FIXED_ENDPOINT,
        data: &QUIRK_DATA_1 as *const _ as *const core::ffi::c_void,
    };

    static QUIRK_DATA_2: snd_usb_midi_endpoint_info = snd_usb_midi_endpoint_info {
        out_ep: 0x06,
        in_ep: 0x06,
        out_cables: 0x003,
        in_cables: 0x003,
    };

    static QUIRK_2: snd_usb_audio_quirk = snd_usb_audio_quirk {
        vendor_name: b"TASCAM\0".as_ptr() as *const i8,
        product_name: b"US428\0".as_ptr() as *const i8,
        ifnum: 0,
        quirk_type: QUIRK_MIDI_FIXED_ENDPOINT,
        data: &QUIRK_DATA_2 as *const _ as *const core::ffi::c_void,
    };

    let dev: *mut usb_device = usx2y(card).as_mut().unwrap().dev;
    let iface: *mut usb_interface = usb_ifnum_to_if(dev, 0);
    let quirk: *const snd_usb_audio_quirk = if le16_to_cpu((*dev).descriptor.idProduct) == USB_ID_US428 {
        &QUIRK_2
    } else {
        &QUIRK_1
    };

    return snd_usbmidi_create(
        card,
        iface,
        &mut usx2y(card).as_mut().unwrap().midi_list as *mut _ as *mut core::ffi::c_void,
        quirk,
    );
}

// Create ALSA devices
unsafe extern "C" fn usx2y_create_alsa_devices(card: *mut snd_card) -> i32 {
    let mut err: i32;

    err = usx2y_create_usbmidi(card);
    if err < 0 {
        dev_err(
            (*card).dev,
            b"%s: usx2y_create_usbmidi error %i\n\0".as_ptr() as *const i8,
            b"usx2y_create_alsa_devices\0".as_ptr() as *const i8,
            err,
        );
        return err;
    }

    err = usx2y_audio_create(card);
    if err < 0 {
        return err;
    }

    err = usx2y_hwdep_pcm_new(card);
    if err < 0 {
        return err;
    }

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    return 0;
}

// DSP firmware load handler
unsafe extern "C" fn snd_usx2y_hwdep_dsp_load(
    hw: *mut snd_hwdep,
    dsp: *mut snd_hwdep_dsp_image,
) -> i32 {
    let priv_data: *mut usx2ydev = (*hw).private_data as *mut usx2ydev;
    let dev: *mut usb_device = (*priv_data).dev;
    let mut lret: i32 = 0;
    let mut err: i32;
    let buf: *mut core::ffi::c_void;

    buf = memdup_user((*dsp).image, (*dsp).length);
    if IS_ERR(buf) != 0 {
        return PTR_ERR(buf);
    }

    err = usb_set_interface(dev, 0, 1);
    if err != 0 {
        dev_err(
            &mut (*dev).dev as *mut device,
            b"usb_set_interface error\n\0".as_ptr() as *const i8,
        );
    } else {
        err = usb_bulk_msg(dev, usb_sndbulkpipe(dev, 2), buf, (*dsp).length, &mut lret, 6000);
    }

    kfree(buf);
    if err != 0 {
        return err;
    }

    if (*dsp).index == 1 {
        msleep(250); // give the device some time

        err = usx2y_async_seq04_init(priv_data);
        if err != 0 {
            dev_err(
                &mut (*dev).dev as *mut device,
                b"usx2y_async_seq04_init error\n\0".as_ptr() as *const i8,
            );
            return err;
        }

        err = usx2y_in04_init(priv_data);
        if err != 0 {
            dev_err(
                &mut (*dev).dev as *mut device,
                b"usx2y_in04_init error\n\0".as_ptr() as *const i8,
            );
            return err;
        }

        err = usx2y_create_alsa_devices((*hw).card);
        if err != 0 {
            dev_err(
                &mut (*dev).dev as *mut device,
                b"usx2y_create_alsa_devices error %i\n\0".as_ptr() as *const i8,
                err,
            );
            return err;
        }

        (*priv_data).chip_status |= USX2Y_STAT_CHIP_INIT;
    }

    return err;
}

// Create new hwdep device
pub unsafe extern "C" fn usx2y_hwdep_new(
    card: *mut snd_card,
    device: *mut usb_device,
) -> i32 {
    let mut err: i32;
    let mut hw: *mut snd_hwdep = ptr::null_mut();
    let us428: *mut usx2ydev = usx2y(card);

    err = snd_hwdep_new(card, SND_USX2Y_LOADER_ID, 0, &mut hw);
    if err < 0 {
        return err;
    }

    (*hw).iface = SNDRV_HWDEP_IFACE_USX2Y;
    (*hw).private_data = us428 as *mut core::ffi::c_void;
    (*hw).ops.dsp_status = Some(snd_usx2y_hwdep_dsp_status);
    (*hw).ops.dsp_load = Some(snd_usx2y_hwdep_dsp_load);
    (*hw).ops.mmap = Some(snd_us428ctls_mmap);
    (*hw).ops.poll = Some(snd_us428ctls_poll);
    (*hw).exclusive = 1;

    sprintf(
        (*hw).name.as_mut_ptr(),
        b"/dev/bus/usb/%03d/%03d\0".as_ptr() as *const i8,
        (*(*device).bus).busnum,
        (*device).devnum,
    );

    (*us428).us428ctls_sharedmem = alloc_pages_exact(US428_SHAREDMEM_PAGES, GFP_KERNEL) as *mut us428ctls_sharedmem;
    if (*us428).us428ctls_sharedmem.is_null() {
        return ENOMEM;
    }

    // Clear memory with pattern -1 (all bits set)
    ptr::write_bytes((*us428).us428ctls_sharedmem as *mut u8, 0xFF, US428_SHAREDMEM_PAGES);
    (*(*us428).us428ctls_sharedmem).ctl_snapshot_last = -2;

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
