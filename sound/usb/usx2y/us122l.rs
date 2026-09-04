// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2007, 2008 Karsten Wiese <fzu@wemgehoertderstaat.de>
 */

// Dependencies from Linux kernel:
// #include <linux/slab.h>
// #include <linux/usb.h>
// #include <linux/usb/audio.h>
// #include <linux/module.h>
// #include <sound/core.h>
// #include <sound/hwdep.h>
// #include <sound/pcm.h>
// #include <sound/initval.h>
// #define MODNAME "US122L"
// #include "usb_stream.c"
// #include "../usbaudio.h"
// #include "../midi.h"
// #include "us122l.h"

const MODNAME: &str = "US122L";

// MODULE_AUTHOR("Karsten Wiese <fzu@wemgehoertderstaat.de>");
// MODULE_DESCRIPTION("TASCAM "NAME_ALLCAPS" Version 0.5");
// MODULE_LICENSE("GPL");

// External kernel types and functions - declarations only
extern "C" {
    type snd_card;
    type snd_hwdep;
    type usb_device;
    type usb_interface;
    type us122l;
    type usb_stream;
    type usb_stream_config;
    type snd_usb_midi_endpoint_info;
    type snd_usb_audio_quirk;
    type vm_fault;
    type vm_area_struct;
    type file;
    type page;
    type poll_table;
    type usb_device_id;
    type usb_driver;
    type pm_message_t;
    type list_head;

    // Constants and enums
    static SNDRV_CARDS: usize;
    static SNDRV_DEFAULT_IDX: [i32; 32];
    static SNDRV_DEFAULT_STR: [*const u8; 32];
    static SNDRV_DEFAULT_ENABLE_PNP: [bool; 32];
    static NAME_ALLCAPS: [u8; 8];
    static USB_ID_US122L: u16;
    static USB_ID_US144: u16;
    static USB_ID_US122MKII: u16;
    static QUIRK_MIDI_US122L: u32;
    static UAC_SET_CUR: u8;
    static UAC_EP_CS_ATTR_SAMPLE_RATE: u8;
    static USB_SPEED_HIGH: u8;
    static SNDRV_USB_STREAM_IOCTL_SET_PARAMS: u32;
    static USB_STREAM_INTERFACE_VERSION: u32;
    static USB_DIR_OUT: u8;
    static USB_TYPE_VENDOR: u8;
    static USB_RECIP_DEVICE: u8;
    static USB_TYPE_CLASS: u8;
    static USB_RECIP_ENDPOINT: u8;
    static USB_DIR_IN: u8;
    static USB_DEVICE_ID_MATCH_DEVICE: u32;
    static PAGE_SHIFT: usize;
    static VM_WRITE: u32;
    static VM_FAULT_SIGBUS: i32;
    static VM_DONTDUMP: u32;
    static VM_DONTEXPAND: u32;
    static EPOLLIN: u32;
    static EPOLLOUT: u32;
    static EPOLLWRNORM: u32;
    static EPOLLERR: u32;
    static SNDRV_HWDEP_IFACE_USB_STREAM: u32;
    static EBUSY: i32;
    static ENOTTY: i32;
    static EFAULT: i32;
    static ENXIO: i32;
    static EINVAL: i32;
    static EIO: i32;
    static ENODEV: i32;
    static EPERM: i32;
    static GFP_NOIO: u32;
    static THIS_MODULE: *mut ();
    static SNDRV_CTL_POWER_D3hot: u32;
    static SNDRV_CTL_POWER_D0: u32;

    fn usb_control_msg_send(
        dev: *mut usb_device,
        pipe: u32,
        request: u8,
        requesttype: u8,
        value: u16,
        index: u16,
        data: *mut u8,
        size: u16,
        timeout: i32,
        gfp_mask: u32,
    ) -> i32;

    fn usb_ifnum_to_if(dev: *mut usb_device, ifnum: i32) -> *mut usb_interface;
    fn snd_usbmidi_create(
        card: *mut snd_card,
        iface: *mut usb_interface,
        midi_list: *mut list_head,
        quirk: *const snd_usb_audio_quirk,
    ) -> i32;
    fn snd_usbmidi_input_stop(p: *mut list_head);
    fn usb_stream_stop(sk: *mut ());
    fn usb_stream_free(sk: *mut ());
    fn usb_stream_new(
        sk: *mut (),
        dev: *mut usb_device,
        in_pipe: u32,
        out_pipe: u32,
        rate: u32,
        use_packsize: u32,
        period_frames: u32,
        frame_size: u32,
    ) -> *mut usb_stream;
    fn usb_stream_start(sk: *mut ()) -> i32;
    fn snd_usbmidi_input_start(p: *mut list_head);
    fn snd_usbmidi_disconnect(p: *mut list_head);
    fn usb_set_interface(dev: *mut usb_device, ifnum: i32, alternate: i32) -> i32;
    fn usb_autopm_get_interface(intf: *mut usb_interface) -> i32;
    fn usb_autopm_put_interface(intf: *mut usb_interface);
    fn virt_to_page(addr: *const ()) -> *mut page;
    fn get_page(page: *mut page);
    fn mutex_trylock(lock: *mut ()) -> bool;
    fn mutex_unlock(lock: *mut ());
    fn mutex_init(lock: *mut ());
    fn init_waitqueue_head(wq: *mut ());
    fn poll_wait(filp: *mut file, wq: *mut (), wait: *mut poll_table);
    fn wake_up_all(wq: *mut ());
    fn snd_hwdep_new(
        card: *mut snd_card,
        id: *const u8,
        device: i32,
        rhwdep: *mut *mut snd_hwdep,
    ) -> i32;
    fn snd_card_new(
        parent: *mut (),
        idx: i32,
        xid: *const u8,
        module: *mut (),
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> i32;
    fn snd_card_free(card: *mut snd_card);
    fn snd_card_free_when_closed(card: *mut snd_card);
    fn snd_card_register(card: *mut snd_card) -> i32;
    fn snd_card_disconnect(card: *mut snd_card);
    fn snd_power_wait(card: *mut snd_card) -> i32;
    fn snd_power_change_state(card: *mut snd_card, power: u32);
    fn strscpy(dest: *mut u8, src: *const u8, count: usize) -> isize;
    fn sprintf(buf: *mut u8, fmt: *const u8, ...) -> i32;
    fn interface_to_usbdev(intf: *mut usb_interface) -> *mut usb_device;
    fn usb_get_intfdata(intf: *mut usb_interface) -> *mut ();
    fn usb_set_intfdata(intf: *mut usb_interface, data: *mut ());
    fn copy_from_user(to: *mut (), from: *const (), n: usize) -> usize;
    fn memcmp(cs: *const (), ct: *const (), count: usize) -> i32;
    fn vm_flags_set(vma: *mut vm_area_struct, flags: u32);
    fn le16_to_cpu(val: u16) -> u16;
    fn dev_err(dev: *const (), fmt: *const u8, ...);
    fn dev_warn(dev: *const (), fmt: *const u8, ...);
    fn list_for_each(pos: *mut *mut list_head, head: *const list_head);
}

// Static module parameters
static mut index: [i32; 32] = SNDRV_DEFAULT_IDX;
static mut id: [*const u8; 32] = SNDRV_DEFAULT_STR;
static mut enable: [bool; 32] = SNDRV_DEFAULT_ENABLE_PNP;

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for "NAME_ALLCAPS".");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for "NAME_ALLCAPS".");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable "NAME_ALLCAPS".");

const US122L_FLAG_US144: u32 = 1 << 0;

static mut snd_us122l_card_used: [i32; 32] = [0; 32];

const SND_USB_STREAM_ID: &[u8; 10] = b"USB STREAM";

unsafe fn us122l_create_usbmidi(card: *mut snd_card) -> i32 {
    #[repr(C)]
    struct SndUsbMidiEndpointInfo {
        out_ep: u8,
        in_ep: u8,
        out_cables: u16,
        in_cables: u16,
    }

    let quirk_data = SndUsbMidiEndpointInfo {
        out_ep: 4,
        in_ep: 3,
        out_cables: 0x001,
        in_cables: 0x001,
    };

    let quirk = snd_usb_audio_quirk {
        vendor_name: b"US122L\0".as_ptr(),
        product_name: NAME_ALLCAPS.as_ptr(),
        ifnum: 1,
        type_: QUIRK_MIDI_US122L,
        data: &quirk_data as *const _ as *const (),
    };

    let dev = (*(card as *const us122l)).dev;
    let iface = usb_ifnum_to_if(dev, 1);

    snd_usbmidi_create(
        card,
        iface,
        &mut (*(card as *mut us122l)).midi_list,
        &quirk,
    )
}

unsafe fn us144_create_usbmidi(card: *mut snd_card) -> i32 {
    #[repr(C)]
    struct SndUsbMidiEndpointInfo {
        out_ep: u8,
        in_ep: u8,
        out_cables: u16,
        in_cables: u16,
    }

    let quirk_data = SndUsbMidiEndpointInfo {
        out_ep: 4,
        in_ep: 3,
        out_cables: 0x001,
        in_cables: 0x001,
    };

    let quirk = snd_usb_audio_quirk {
        vendor_name: b"US144\0".as_ptr(),
        product_name: NAME_ALLCAPS.as_ptr(),
        ifnum: 0,
        type_: QUIRK_MIDI_US122L,
        data: &quirk_data as *const _ as *const (),
    };

    let dev = (*(card as *const us122l)).dev;
    let iface = usb_ifnum_to_if(dev, 0);

    snd_usbmidi_create(
        card,
        iface,
        &mut (*(card as *mut us122l)).midi_list,
        &quirk,
    )
}

unsafe fn pt_info_set(dev: *mut usb_device, v: u8) {
    usb_control_msg_send(
        dev,
        0,
        b'I' as u8,
        USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
        v as u16,
        0,
        std::ptr::null_mut(),
        0,
        1000,
        GFP_NOIO,
    );
}

unsafe extern "C" fn usb_stream_hwdep_vm_fault(vmf: *mut vm_fault) -> i32 {
    let offset: u64;
    let page: *mut page;
    let vaddr: *mut u8;
    let us122l = (*(vmf as *const vm_fault)).vma.vm_private_data as *mut us122l;
    let s: *mut usb_stream;

    // guard(mutex)(&us122l->mutex);
    s = (*(us122l)).sk.s;
    if s.is_null() {
        return VM_FAULT_SIGBUS;
    }

    offset = (*(vmf)).pgoff << PAGE_SHIFT as u32;
    if offset < ((*s).read_size as u64) {
        vaddr = (s as *mut u8).add(offset as usize);
    } else {
        let offset = offset - ((*s).read_size as u64);
        if offset >= ((*s).write_size as u64) {
            return VM_FAULT_SIGBUS;
        }
        vaddr = (*(us122l)).sk.write_page.add(offset as usize);
    }
    page = virt_to_page(vaddr as *const ());

    get_page(page);

    (*(vmf)).page = page;

    0
}

#[repr(C)]
struct VmOperationsStruct {
    fault: unsafe extern "C" fn(*mut vm_fault) -> i32,
}

static usb_stream_hwdep_vm_ops: VmOperationsStruct = VmOperationsStruct {
    fault: usb_stream_hwdep_vm_fault,
};

unsafe fn usb_stream_hwdep_open(hw: *mut snd_hwdep, file: *mut file) -> i32 {
    let us122l = (*(hw)).private_data as *mut us122l;
    let iface: *mut usb_interface;

    if (*(hw)).used >= 2 {
        return -EBUSY;
    }

    if (*(us122l)).first.is_null() {
        (*(us122l)).first = file;
    }

    if (*(us122l)).is_us144 != 0 {
        iface = usb_ifnum_to_if((*(us122l)).dev, 0);
        usb_autopm_get_interface(iface);
    }
    iface = usb_ifnum_to_if((*(us122l)).dev, 1);
    usb_autopm_get_interface(iface);
    0
}

unsafe fn usb_stream_hwdep_release(hw: *mut snd_hwdep, file: *mut file) -> i32 {
    let us122l = (*(hw)).private_data as *mut us122l;
    let iface: *mut usb_interface;

    if (*(us122l)).is_us144 != 0 {
        iface = usb_ifnum_to_if((*(us122l)).dev, 0);
        usb_autopm_put_interface(iface);
    }
    iface = usb_ifnum_to_if((*(us122l)).dev, 1);
    usb_autopm_put_interface(iface);
    if (*(us122l)).first == file {
        (*(us122l)).first = std::ptr::null_mut();
    }
    // guard(mutex)(&us122l->mutex);
    if (*(us122l)).master == file {
        (*(us122l)).master = (*(us122l)).slave;
    }

    (*(us122l)).slave = std::ptr::null_mut();
    0
}

unsafe fn usb_stream_hwdep_mmap(
    hw: *mut snd_hwdep,
    filp: *mut file,
    area: *mut vm_area_struct,
) -> i32 {
    let size: u64 = ((*area).vm_end - (*area).vm_start) as u64;
    let us122l = (*(hw)).private_data as *mut us122l;
    let offset: u64;
    let s: *mut usb_stream;
    let read: bool;

    offset = ((*area).vm_pgoff << PAGE_SHIFT as u32) as u64;
    // guard(mutex)(&us122l->mutex);
    s = (*(us122l)).sk.s;
    read = offset < (*s).read_size as u64;
    if read && ((*area).vm_flags & VM_WRITE) != 0 {
        return -EPERM;
    }
    // if userspace tries to mmap beyond end of our buffer, fail
    let max_size = if read {
        (*s).read_size
    } else {
        (*s).write_size
    } as u64;
    if size > (max_size + (PAGE_SHIFT - 1)) & !((PAGE_SHIFT - 1) as u64) {
        dev_warn(
            (*(hw)).card.dev,
            b"%s: size %lu > %u\n\0".as_ptr(),
            b"usb_stream_hwdep_mmap\0".as_ptr(),
            size,
            max_size as u32,
        );
        return -EINVAL;
    }

    ((*area).vm_ops as *mut VmOperationsStruct) = &usb_stream_hwdep_vm_ops as *const _ as *mut _;
    vm_flags_set(area, VM_DONTDUMP);
    if !read {
        vm_flags_set(area, VM_DONTEXPAND);
    }
    ((*area).vm_private_data) = us122l as *const ();
    0
}

unsafe fn usb_stream_hwdep_poll(
    hw: *mut snd_hwdep,
    file: *mut file,
    wait: *mut poll_table,
) -> u32 {
    let us122l = (*(hw)).private_data as *mut us122l;
    let polled: *mut u32;
    let mut mask: u32;

    poll_wait(file, &mut (*(us122l)).sk.sleep, wait);

    mask = EPOLLIN | EPOLLOUT | EPOLLWRNORM | EPOLLERR;
    if mutex_trylock(&mut (*(us122l)).mutex as *mut ()) {
        let s = (*(us122l)).sk.s;

        if !s.is_null()
            && (*s).state as i32
                == 1 /* usb_stream_ready - from usb_stream.h */
        {
            if (*(us122l)).first == file {
                polled = &mut (*s).periods_polled;
            } else {
                polled = &mut (*(us122l)).second_periods_polled;
            }
            if *polled != (*s).periods_done {
                *polled = (*s).periods_done;
                mask = EPOLLIN | EPOLLOUT | EPOLLWRNORM;
            } else {
                mask = 0;
            }
        }
        mutex_unlock(&mut (*(us122l)).mutex as *mut ());
    }
    mask
}

unsafe fn us122l_stop(us122l: *mut us122l) {
    let p: *mut list_head;

    // list_for_each(p, &us122l->midi_list)
    p = (*(us122l)).midi_list.next;
    while p != &mut (*(us122l)).midi_list {
        snd_usbmidi_input_stop(p);
        p = (*p).next;
    }

    usb_stream_stop(&mut (*(us122l)).sk as *mut () as *mut ());
    usb_stream_free(&mut (*(us122l)).sk as *mut () as *mut ());
}

unsafe fn us122l_set_sample_rate(dev: *mut usb_device, rate: i32) -> i32 {
    let _ep: u32 = 0x81;
    let mut data: [u8; 3] = [0; 3];

    data[0] = rate as u8;
    data[1] = (rate >> 8) as u8;
    data[2] = (rate >> 16) as u8;
    let err = usb_control_msg_send(
        dev,
        0,
        UAC_SET_CUR,
        USB_TYPE_CLASS | USB_RECIP_ENDPOINT | USB_DIR_OUT,
        (UAC_EP_CS_ATTR_SAMPLE_RATE as u16) << 8,
        0x81u16,
        data.as_mut_ptr(),
        3,
        1000,
        GFP_NOIO,
    );
    if err != 0 {
        dev_err(
            &(*dev).dev,
            b"%d: cannot set freq %d to ep 0x%x\n\0".as_ptr(),
            (*dev).devnum,
            rate,
            0x81,
        );
    }
    err
}

unsafe fn us122l_start(
    us122l: *mut us122l,
    rate: u32,
    period_frames: u32,
) -> bool {
    let mut p: *mut list_head;
    let mut err: i32;
    let mut use_packsize: u32 = 0;
    let mut success: bool = false;

    if (*((*us122l).dev)).speed == USB_SPEED_HIGH {
        // The us-122l's descriptor defaults to iso max_packsize 78,
        // which isn't needed for samplerates <= 48000.
        // Lets save some memory:
        match rate {
            44100 => {
                use_packsize = 36;
            }
            48000 => {
                use_packsize = 42;
            }
            88200 => {
                use_packsize = 72;
            }
            _ => {}
        }
    }
    if usb_stream_new(&mut (*(us122l)).sk as *mut () as *mut (), (*us122l).dev, 1, 2, rate, use_packsize, period_frames, 6).is_null() {
        return false;
    }

    err = us122l_set_sample_rate((*us122l).dev, rate as i32);
    if err < 0 {
        us122l_stop(us122l);
        dev_err(
            &(*(*us122l).dev).dev,
            b"us122l_set_sample_rate error\n\0".as_ptr(),
        );
        return false;
    }
    err = usb_stream_start(&mut (*(us122l)).sk as *mut () as *mut ());
    if err < 0 {
        us122l_stop(us122l);
        dev_err(
            &(*(*us122l).dev).dev,
            b"%s error %i\n\0".as_ptr(),
            b"us122l_start\0".as_ptr(),
            err,
        );
        return false;
    }
    // list_for_each(p, &us122l->midi_list)
    p = (*(us122l)).midi_list.next;
    while p != &mut (*(us122l)).midi_list {
        snd_usbmidi_input_start(p);
        p = (*p).next;
    }
    true
}

unsafe fn usb_stream_hwdep_ioctl(
    hw: *mut snd_hwdep,
    file: *mut file,
    cmd: u32,
    arg: u64,
) -> i32 {
    let mut cfg: usb_stream_config = std::mem::zeroed();
    let us122l = (*(hw)).private_data as *mut us122l;
    let s: *mut usb_stream;
    let mut min_period_frames: u32;
    let mut err: i32 = 0;
    let high_speed: bool;

    if cmd != SNDRV_USB_STREAM_IOCTL_SET_PARAMS {
        return -ENOTTY;
    }

    if copy_from_user(&mut cfg as *mut _ as *mut (), arg as *const (), std::mem::size_of::<usb_stream_config>()) != 0 {
        return -EFAULT;
    }

    if cfg.version != USB_STREAM_INTERFACE_VERSION {
        return -ENXIO;
    }

    high_speed = (*(*us122l).dev).speed == USB_SPEED_HIGH;
    if (cfg.sample_rate != 44100
        && cfg.sample_rate != 48000
        && (!high_speed
            || (cfg.sample_rate != 88200 && cfg.sample_rate != 96000)))
        || cfg.frame_size != 6
        || cfg.period_frames > 0x3000
    {
        return -EINVAL;
    }

    match cfg.sample_rate {
        44100 => {
            min_period_frames = 48;
        }
        48000 => {
            min_period_frames = 52;
        }
        _ => {
            min_period_frames = 104;
        }
    }
    if !high_speed {
        min_period_frames <<= 1;
    }
    if cfg.period_frames < min_period_frames {
        return -EINVAL;
    }

    snd_power_wait((*(hw)).card);

    // guard(mutex)(&us122l->mutex);
    s = (*(us122l)).sk.s;
    if (*(us122l)).master.is_null() {
        (*(us122l)).master = file;
    } else if (*(us122l)).master != file {
        if s.is_null()
            || memcmp(
                &cfg as *const _ as *const (),
                &(*s).cfg as *const _ as *const (),
                std::mem::size_of::<usb_stream_config>(),
            ) != 0
        {
            err = -EIO;
            // goto unlock;
        } else {
            (*(us122l)).slave = file;
        }
        if err != 0 {
            wake_up_all(&mut (*(us122l)).sk.sleep);
            return err;
        }
    }
    if s.is_null()
        || memcmp(
            &cfg as *const _ as *const (),
            &(*s).cfg as *const _ as *const (),
            std::mem::size_of::<usb_stream_config>(),
        ) != 0
        || (*s).state as i32 == 3 /* usb_stream_xrun */
    {
        us122l_stop(us122l);
        if !us122l_start(us122l, cfg.sample_rate, cfg.period_frames) {
            err = -EIO;
        } else {
            err = 1;
        }
    }
    // unlock:
    wake_up_all(&mut (*(us122l)).sk.sleep);
    err
}

unsafe fn usb_stream_hwdep_new(card: *mut snd_card) -> i32 {
    let mut err: i32;
    let hw: *mut snd_hwdep = std::ptr::null_mut();
    let dev = (*(card as *const us122l)).dev;

    err = snd_hwdep_new(
        card,
        SND_USB_STREAM_ID.as_ptr(),
        0,
        &hw as *const _ as *mut _,
    );
    if err < 0 {
        return err;
    }

    (*(hw)).iface = SNDRV_HWDEP_IFACE_USB_STREAM;
    (*(hw)).private_data = card as *const us122l as *const ();
    (*(hw)).ops.open = Some(usb_stream_hwdep_open);
    (*(hw)).ops.release = Some(usb_stream_hwdep_release);
    (*(hw)).ops.ioctl = Some(usb_stream_hwdep_ioctl);
    (*(hw)).ops.ioctl_compat = Some(usb_stream_hwdep_ioctl);
    (*(hw)).ops.mmap = Some(usb_stream_hwdep_mmap);
    (*(hw)).ops.poll = Some(usb_stream_hwdep_poll);

    sprintf(
        (*(hw)).name.as_mut_ptr(),
        b"/dev/bus/usb/%03d/%03d/hwdeppcm\0".as_ptr(),
        (*dev).bus.busnum,
        (*dev).devnum,
    );
    0
}

unsafe fn us122l_create_card(card: *mut snd_card) -> bool {
    let mut err: i32;
    let us122l = card as *mut us122l;

    if (*us122l).is_us144 != 0 {
        err = usb_set_interface((*us122l).dev, 0, 1);
        if err != 0 {
            dev_err(
                (*card).dev,
                b"usb_set_interface error\n\0".as_ptr(),
            );
            return false;
        }
    }
    err = usb_set_interface((*us122l).dev, 1, 1);
    if err != 0 {
        dev_err(
            (*card).dev,
            b"usb_set_interface error\n\0".as_ptr(),
        );
        return false;
    }

    pt_info_set((*us122l).dev, 0x11);
    pt_info_set((*us122l).dev, 0x10);

    if !us122l_start(us122l, 44100, 256) {
        return false;
    }

    if (*us122l).is_us144 != 0 {
        err = us144_create_usbmidi(card);
    } else {
        err = us122l_create_usbmidi(card);
    }
    if err < 0 {
        dev_err(
            (*card).dev,
            b"us122l_create_usbmidi error %i\n\0".as_ptr(),
            err,
        );
        // goto stop;
        us122l_stop(us122l);
        return false;
    }
    err = usb_stream_hwdep_new(card);
    if err < 0 {
        // release the midi resources
        let mut p: *mut list_head;

        // list_for_each(p, &us122l->midi_list)
        p = (*us122l).midi_list.next;
        while p != &mut (*us122l).midi_list {
            snd_usbmidi_disconnect(p);
            p = (*p).next;
        }

        // goto stop;
        us122l_stop(us122l);
        return false;
    }
    true
}

unsafe fn snd_us122l_free(card: *mut snd_card) {
    let us122l = card as *mut us122l;
    let index = (*us122l).card_index;

    if index >= 0 && index < 32 {
        snd_us122l_card_used[index as usize] = 0;
    }
}

unsafe fn usx2y_create_card(
    device: *mut usb_device,
    intf: *mut usb_interface,
    cardp: *mut *mut snd_card,
    flags: u64,
) -> i32 {
    let mut dev: i32;
    let mut card: *mut snd_card = std::ptr::null_mut();
    let mut err: i32;

    dev = 0;
    while (dev as usize) < 32 {
        if enable[dev as usize] && snd_us122l_card_used[dev as usize] == 0 {
            break;
        }
        dev += 1;
    }
    if (dev as usize) >= 32 {
        return -ENODEV;
    }
    err = snd_card_new(
        (*intf).dev,
        index[dev as usize],
        id[dev as usize],
        THIS_MODULE,
        std::mem::size_of::<us122l>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    let us122l_ptr = card as *mut us122l;
    snd_us122l_card_used[dev as usize] = 1;
    (*us122l_ptr).card_index = dev;
    (*card).private_free = Some(snd_us122l_free);
    (*us122l_ptr).dev = device;
    mutex_init(&mut (*us122l_ptr).mutex as *mut ());
    (*us122l_ptr).sk.dev = device;
    init_waitqueue_head(&mut (*us122l_ptr).sk.sleep as *mut ());
    (*us122l_ptr).is_us144 = if (flags & US122L_FLAG_US144 as u64) != 0 { 1 } else { 0 };
    // INIT_LIST_HEAD(&(*us122l_ptr).midi_list);
    strscpy(
        (*card).driver.as_mut_ptr(),
        b"USB \0".as_ptr().cast_mut(),
        32,
    );
    sprintf(
        (*card).shortname.as_mut_ptr(),
        b"TASCAM \0".as_ptr(),
    );
    sprintf(
        (*card).longname.as_mut_ptr(),
        b"%s (%x:%x if %d at %03d/%03d)\0".as_ptr(),
        (*card).shortname.as_ptr(),
        le16_to_cpu((*device).descriptor.idVendor),
        le16_to_cpu((*device).descriptor.idProduct),
        0,
        (*(*us122l_ptr).dev).bus.busnum,
        (*(*us122l_ptr).dev).devnum,
    );
    *cardp = card;
    0
}

unsafe fn us122l_usb_probe(
    intf: *mut usb_interface,
    device_id: *const usb_device_id,
    cardp: *mut *mut snd_card,
) -> i32 {
    let device = interface_to_usbdev(intf);
    let mut card: *mut snd_card = std::ptr::null_mut();
    let mut err: i32;

    err = usx2y_create_card(device, intf, &mut card, (*device_id).driver_info);
    if err < 0 {
        return err;
    }

    if !us122l_create_card(card) {
        snd_card_free(card);
        return -EINVAL;
    }

    err = snd_card_register(card);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    *cardp = card;
    0
}

unsafe fn snd_us122l_probe(intf: *mut usb_interface, id: *const usb_device_id) -> i32 {
    let device = interface_to_usbdev(intf);
    let mut card: *mut snd_card = std::ptr::null_mut();
    let mut err: i32;

    if ((*id).driver_info & US122L_FLAG_US144) != 0
        && (*device).speed == USB_SPEED_HIGH
    {
        dev_err(
            &(*device).dev,
            b"disable ehci-hcd to run US-144\n\0".as_ptr(),
        );
        return -ENODEV;
    }

    if (*(*intf).cur_altsetting).desc.bInterfaceNumber != 1 {
        return 0;
    }

    err = us122l_usb_probe(intf, id, &mut card);
    if err < 0 {
        return err;
    }

    usb_set_intfdata(intf, card as *const () as *mut ());
    0
}

unsafe fn snd_us122l_disconnect(intf: *mut usb_interface) {
    let mut card: *mut snd_card;
    let us122l: *mut us122l;
    let mut p: *mut list_head;

    card = usb_get_intfdata(intf) as *mut _;
    if card.is_null() {
        return;
    }

    snd_card_disconnect(card);

    us122l = card as *mut us122l;
    // scoped_guard(mutex, &us122l->mutex)
    {
        us122l_stop(us122l);
    }

    // release the midi resources
    p = (*us122l).midi_list.next;
    while p != &mut (*us122l).midi_list {
        snd_usbmidi_disconnect(p);
        p = (*p).next;
    }

    snd_card_free_when_closed(card);
}

unsafe fn snd_us122l_suspend(intf: *mut usb_interface, _message: pm_message_t) -> i32 {
    let mut card: *mut snd_card;
    let us122l: *mut us122l;
    let mut p: *mut list_head;

    card = usb_get_intfdata(intf) as *mut _;
    if card.is_null() {
        return 0;
    }
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);

    us122l = card as *mut us122l;
    if us122l.is_null() {
        return 0;
    }

    p = (*us122l).midi_list.next;
    while p != &mut (*us122l).midi_list {
        snd_usbmidi_input_stop(p);
        p = (*p).next;
    }

    // guard(mutex)(&us122l->mutex);
    usb_stream_stop(&mut (*us122l).sk as *mut () as *mut ());

    0
}

unsafe fn snd_us122l_resume(intf: *mut usb_interface) -> i32 {
    let mut card: *mut snd_card;
    let us122l: *mut us122l;
    let mut p: *mut list_head;
    let mut err: i32;

    card = usb_get_intfdata(intf) as *mut _;
    if card.is_null() {
        return 0;
    }

    us122l = card as *mut us122l;
    if us122l.is_null() {
        return 0;
    }

    // guard(mutex)(&us122l->mutex);
    // needed, doesn't restart without:
    if (*us122l).is_us144 != 0 {
        err = usb_set_interface((*us122l).dev, 0, 1);
        if err != 0 {
            dev_err(
                &(*(*us122l).dev).dev,
                b"usb_set_interface error\n\0".as_ptr(),
            );
            snd_power_change_state(card, SNDRV_CTL_POWER_D0);
            return err;
        }
    }
    err = usb_set_interface((*us122l).dev, 1, 1);
    if err != 0 {
        dev_err(
            &(*(*us122l).dev).dev,
            b"usb_set_interface error\n\0".as_ptr(),
        );
        snd_power_change_state(card, SNDRV_CTL_POWER_D0);
        return err;
    }

    pt_info_set((*us122l).dev, 0x11);
    pt_info_set((*us122l).dev, 0x10);

    err = us122l_set_sample_rate(
        (*us122l).dev,
        (*(*(*us122l).sk.s).cfg).sample_rate as i32,
    );
    if err < 0 {
        dev_err(
            &(*(*us122l).dev).dev,
            b"us122l_set_sample_rate error\n\0".as_ptr(),
        );
        snd_power_change_state(card, SNDRV_CTL_POWER_D0);
        return err;
    }
    err = usb_stream_start(&mut (*us122l).sk as *mut () as *mut ());
    if err != 0 {
        snd_power_change_state(card, SNDRV_CTL_POWER_D0);
        return err;
    }

    p = (*us122l).midi_list.next;
    while p != &mut (*us122l).midi_list {
        snd_usbmidi_input_start(p);
        p = (*p).next;
    }

    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    err
}

#[repr(C)]
struct SndUsbDeviceIdEntry {
    match_flags: u32,
    idVendor: u16,
    idProduct: u16,
    _pad1: u16,
    bcdDevice_lo: u16,
    bcdDevice_hi: u16,
    driver_info: u64,
}

static snd_us122l_usb_id_table: [SndUsbDeviceIdEntry; 4] = [
    SndUsbDeviceIdEntry {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: 0x0644,
        idProduct: USB_ID_US122L,
        _pad1: 0,
        bcdDevice_lo: 0,
        bcdDevice_hi: 0,
        driver_info: 0,
    },
    SndUsbDeviceIdEntry {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: 0x0644,
        idProduct: USB_ID_US144,
        _pad1: 0,
        bcdDevice_lo: 0,
        bcdDevice_hi: 0,
        driver_info: US122L_FLAG_US144 as u64,
    },
    SndUsbDeviceIdEntry {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: 0x0644,
        idProduct: USB_ID_US122MKII,
        _pad1: 0,
        bcdDevice_lo: 0,
        bcdDevice_hi: 0,
        driver_info: 0,
    },
    SndUsbDeviceIdEntry {
        match_flags: 0,
        idVendor: 0,
        idProduct: 0,
        _pad1: 0,
        bcdDevice_lo: 0,
        bcdDevice_hi: 0,
        driver_info: 0,
    },
];

// MODULE_DEVICE_TABLE(usb, snd_us122l_usb_id_table);

#[repr(C)]
struct UsbDriver {
    name: *const u8,
    probe: unsafe extern "C" fn(*mut usb_interface, *const usb_device_id) -> i32,
    disconnect: unsafe extern "C" fn(*mut usb_interface),
    suspend: unsafe extern "C" fn(*mut usb_interface, pm_message_t) -> i32,
    resume: unsafe extern "C" fn(*mut usb_interface) -> i32,
    reset_resume: unsafe extern "C" fn(*mut usb_interface) -> i32,
    id_table: *const SndUsbDeviceIdEntry,
    supports_autosuspend: i32,
}

static snd_us122l_usb_driver: UsbDriver = UsbDriver {
    name: b"snd-usb-us122l\0".as_ptr(),
    probe: snd_us122l_probe,
    disconnect: snd_us122l_disconnect,
    suspend: snd_us122l_suspend,
    resume: snd_us122l_resume,
    reset_resume: snd_us122l_resume,
    id_table: snd_us122l_usb_id_table.as_ptr(),
    supports_autosuspend: 1,
};

// module_usb_driver(snd_us122l_usb_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
