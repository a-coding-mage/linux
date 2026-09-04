// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * usbusx2y.rs - ALSA USB US-428 Driver
 *
2005-04-14 Karsten Wiese
	Version 0.8.7.2:
	Call snd_card_free() instead of snd_card_free_in_thread() to prevent oops with dead keyboard symptom.
	Tested ok with kernel 2.6.12-rc2.

2004-12-14 Karsten Wiese
	Version 0.8.7.1:
	snd_pcm_open for rawusb pcm-devices now returns -EBUSY if called without rawusb's hwdep device being open.

2004-12-02 Karsten Wiese
	Version 0.8.7:
	Use macro usb_maxpacket() for portability.

2004-10-26 Karsten Wiese
	Version 0.8.6:
	wake_up() process waiting in usx2y_urbs_start() on error.

2004-10-21 Karsten Wiese
	Version 0.8.5:
	nrpacks is runtime or compiletime configurable now with tested values from 1 to 4.

2004-10-03 Karsten Wiese
	Version 0.8.2:
	Avoid any possible racing while in prepare callback.

2004-09-30 Karsten Wiese
	Version 0.8.0:
	Simplified things and made ohci work again.

2004-09-20 Karsten Wiese
	Version 0.7.3:
	Use usb_kill_urb() instead of deprecated (kernel 2.6.9) usb_unlink_urb().

2004-07-13 Karsten Wiese
	Version 0.7.1:
	Don't sleep in START/STOP callbacks anymore.
	us428 channels C/D not handled just for this version, sorry.

2004-06-21 Karsten Wiese
	Version 0.6.4:
	Temporarely suspend midi input
	to sanely call usb_set_interface() when setting format.

2004-06-12 Karsten Wiese
	Version 0.6.3:
	Made it thus the following rule is enforced:
	"All pcm substreams of one usx2y have to operate at the same rate & format."

2004-04-06 Karsten Wiese
	Version 0.6.0:
	Runs on 2.6.5 kernel without any "--with-debug=" things.
	us224 reported running.

2004-01-14 Karsten Wiese
	Version 0.5.1:
	Runs with 2.6.1 kernel.

2003-12-30 Karsten Wiese
	Version 0.4.1:
	Fix 24Bit 4Channel capturing for the us428.

2003-11-27 Karsten Wiese, Martin Langer
	Version 0.4:
	us122 support.
	us224 could be tested by uncommenting the sections containing USB_ID_US224

2003-11-03 Karsten Wiese
	Version 0.3:
	24Bit support.
	"arecord -D hw:1 -c 2 -r 48000 -M -f S24_3LE|aplay -D hw:1 -c 2 -r 48000 -M -f S24_3LE" works.

2003-08-22 Karsten Wiese
	Version 0.0.8:
	Removed EZUSB Firmware. First Stage Firmwaredownload is now done by tascam-firmware downloader.
	See:
	http://usb-midi-fw.sourceforge.net/tascam-firmware.tar.gz

2003-06-18 Karsten Wiese
	Version 0.0.5:
	changed to compile with kernel 2.4.21 and alsa 0.9.4

2002-10-16 Karsten Wiese
	Version 0.0.4:
	compiles again with alsa-current.
	USB_ISO_ASAP not used anymore (most of the time), instead
	urb->start_frame is calculated here now, some calls inside usb-driver don't need to happen anymore.

	To get the best out of this:
	Disable APM-support in the kernel as APM-BIOS calls (once each second) hard disable interrupt for many precious milliseconds.
	This helped me much on my slowish PII 400 & PIII 500.
	ACPI yet untested but might cause the same bad behaviour.
	Use a kernel with lowlatency and preemptiv patches applied.
	To autoload snd-usb-midi append a line
		post-install snd-usb-us428 modprobe snd-usb-midi
	to /etc/modules.conf.

	known problems:
	sliders, knobs, lights not yet handled except MASTER Volume slider.
	"pcm -c 2" doesn't work. "pcm -c 2 -m direct_interleaved" does.
	KDE3: "Enable full duplex operation" deadlocks.

2002-08-31 Karsten Wiese
	Version 0.0.3: audio also simplex;
	simplifying: iso urbs only 1 packet, melted structs.
	ASYNC_UNLINK not used anymore: no more crashes so far.....
	for alsa 0.9 rc3.

2002-08-09 Karsten Wiese
	Version 0.0.2: midi works with snd-usb-midi, audio (only fullduplex now) with i.e. bristol.
	The firmware has been sniffed from win2k us-428 driver 3.09.

 *   Copyright (c) 2002 - 2004 Karsten Wiese
 */

// Dependencies (from linux kernel):
// - #include <linux/init.h>
// - #include <linux/module.h>
// - #include <linux/moduleparam.h>
// - #include <linux/slab.h>
// - #include <linux/interrupt.h>
// - #include <linux/usb.h>
// - #include <sound/core.h>
// - #include <sound/initval.h>
// - #include <sound/pcm.h>
// - #include <sound/rawmidi.h>
// - "usx2y.h"
// - "usbusx2y.h"
// - "usX2Yhwdep.h"

use core::ptr;
use core::ffi::c_char;

// TODO: External dependencies from linux kernel headers
extern {
    type urb;
    type usb_device;
    type usb_interface;
    type snd_card;
    type usx2ydev;
    type us428ctls_sharedmem;
    type us428_p4out;
    type us428_lights;
    type snd_usx2y_async_seq;
}

// Module metadata
// MODULE_AUTHOR("Karsten Wiese <annabellesgarden@yahoo.de>");
// MODULE_DESCRIPTION("TASCAM "NAME_ALLCAPS" Version 0.8.7.2");
// MODULE_LICENSE("GPL");

const SNDRV_CARDS: usize = 32; // Standard ALSA constant

// Module parameters
static mut INDEX: [i32; SNDRV_CARDS] = [0; SNDRV_CARDS];
// SNDRV_DEFAULT_IDX initialization would be set by kernel
static mut ID: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
// SNDRV_DEFAULT_STR initialization would be set by kernel
static mut ENABLE: [bool; SNDRV_CARDS] = [true; SNDRV_CARDS];
// SNDRV_DEFAULT_ENABLE_PNP initialization would be set by kernel

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for "NAME_ALLCAPS".");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for "NAME_ALLCAPS".");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable "NAME_ALLCAPS".");

static mut SND_USX2Y_CARD_USED: [i32; SNDRV_CARDS] = [0; SNDRV_CARDS];

extern "C" {
    fn snd_usx2y_card_private_free(card: *mut snd_card);
}

extern "C" {
    fn usx2y_unlinkseq(s: *mut snd_usx2y_async_seq);
}

// #ifdef USX2Y_NRPACKS_VARIABLE
// int nrpacks = USX2Y_NRPACKS; /* number of packets per urb */
// module_param(nrpacks, int, 0444);
// MODULE_PARM_DESC(nrpacks, "Number of packets per URB.");
// #endif

// External function declarations from kernel
extern "C" {
    fn dev_dbg(dev: *mut core::ffi::c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const c_char, ...);
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn usb_submit_urb(urb: *mut urb, mem_flags: u32) -> i32;
    fn usb_alloc_urb(iso_packets: i32, mem_flags: u32) -> *mut urb;
    fn usb_free_urb(urb: *mut urb);
    fn usb_kill_urb(urb: *mut urb);
    fn usb_fill_bulk_urb(
        urb: *mut urb,
        dev: *mut usb_device,
        pipe: u32,
        transfer_buffer: *mut core::ffi::c_void,
        transfer_buffer_length: i32,
        complete: extern "C" fn(*mut urb),
        context: *mut core::ffi::c_void,
    );
    fn usb_sndbulkpipe(dev: *mut usb_device, endpoint: u8) -> u32;
    fn usb_rcvintpipe(dev: *mut usb_device, endpoint: u8) -> u32;
    fn usb_fill_int_urb(
        urb: *mut urb,
        dev: *mut usb_device,
        pipe: u32,
        transfer_buffer: *mut core::ffi::c_void,
        transfer_buffer_length: i32,
        complete: extern "C" fn(*mut urb),
        context: *mut usb_device,
        interval: i32,
    );
    fn usb_urb_ep_type_check(urb: *mut urb) -> i32;
    fn kmalloc_array(n: usize, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn init_waitqueue_head(wq: *mut core::ffi::c_void);
    fn wake_up(wq: *mut core::ffi::c_void);
    fn snd_card_new(
        parent: *mut core::ffi::c_void,
        idx: i32,
        xid: *const c_char,
        module: *mut core::ffi::c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> i32;
    fn snd_card_register(card: *mut snd_card) -> i32;
    fn snd_card_disconnect(card: *mut snd_card);
    fn snd_card_free(card: *mut snd_card);
    fn snd_card_free_when_closed(card: *mut snd_card);
    fn snd_usbmidi_disconnect(p: *mut core::ffi::c_void);
    fn snd_card_free_pages_exact(ptr: *mut core::ffi::c_void, bytes: usize);
    fn strscpy(dest: *mut c_char, src: *const c_char, n: usize) -> usize;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> i32;
    fn le16_to_cpu(x: u16) -> u16;
    fn usb_get_intfdata(intf: *mut usb_interface) -> *mut core::ffi::c_void;
    fn dev_set_drvdata(dev: *mut core::ffi::c_void, data: *mut core::ffi::c_void);
    fn interface_to_usbdev(intf: *mut usb_interface) -> *mut usb_device;
    fn usx2y_hwdep_new(card: *mut snd_card, device: *mut usb_device) -> i32;
    fn usx2y(card: *mut snd_card) -> *mut usx2ydev;
    fn mutex_init(lock: *mut core::ffi::c_void);
    fn INIT_LIST_HEAD(list: *mut core::ffi::c_void);
}

const URBS_ASYNC_SEQ: usize = 10;
const URB_DATA_LEN_ASYNC_SEQ: usize = 32;
const N_US428_CTL_BUFS: usize = 16;
const N_US428_P4OUT_BUFS: usize = 16;
const ELT_LIGHT: i32 = 1;
const USX2Y_STAT_CHIP_HUP: i32 = -1;

const USB_ID_US428: u16 = 0x8001;
const USB_ID_US122: u16 = 0x8007;
const USB_ID_US224: u16 = 0x8008;

const GFP_KERNEL: u32 = 0x00;
const GFP_ATOMIC: u32 = 0x01;

// extern "C" fn i_usx2y_out04_int(urb: *mut urb) {
//     #[cfg(CONFIG_SND_DEBUG)]
//     {
//         if unsafe { (*urb).status } != 0 {
//             let mut i = 0;
//             let usx2y = unsafe { (*urb).context as *mut usx2ydev };
//             while i < 10 && unsafe { (*usx2y).as04.urb[i] } != urb {
//                 i += 1;
//             }
//             unsafe {
//                 dev_dbg(&mut (*(*urb).dev).dev as *mut _, b"%s urb %i status=%i\n".as_ptr() as *const c_char, i, (*urb).status);
//             }
//         }
//     }
// }

/*
 * pipe 4 is used for switching the lamps, setting samplerate, volumes ....
 */
#[cfg(CONFIG_SND_DEBUG)]
unsafe extern "C" fn i_usx2y_out04_int(urb: *mut urb) {
    if (*urb).status != 0 {
        let mut i = 0;
        let usx2y = (*urb).context as *mut usx2ydev;
        while i < 10 && (*usx2y).as04.urb[i] != urb {
            i += 1;
        }
        dev_dbg(
            &mut (*(*urb).dev).dev as *mut _ as *mut core::ffi::c_void,
            b"%s urb %i status=%i\n".as_ptr() as *const c_char,
        );
    }
}

#[cfg(not(CONFIG_SND_DEBUG))]
unsafe extern "C" fn i_usx2y_out04_int(_urb: *mut urb) {}

unsafe extern "C" fn i_usx2y_in04_int(urb: *mut urb) {
    let mut err: i32 = 0;
    let usx2y = (*urb).context as *mut usx2ydev;
    let us428ctls = (*usx2y).us428ctls_sharedmem;
    let mut p4out: *mut us428_p4out;
    let mut i: i32;
    let mut j: i32;
    let mut n: i32;
    let mut diff: i32;
    let mut send: i32;
    let mut len: i32;

    (*usx2y).in04_int_calls += 1;

    if (*urb).status != 0 {
        dev_dbg(
            &mut (*(*urb).dev).dev as *mut _ as *mut core::ffi::c_void,
            b"Interrupt Pipe 4 came back with status=%i\n".as_ptr() as *const c_char,
        );
        return;
    }

    if !us428ctls.is_null() {
        diff = -1;
        if (*us428ctls).ctl_snapshot_last == -2 {
            diff = 0;
            memcpy(
                (*usx2y).in04_last.as_mut_ptr() as *mut core::ffi::c_void,
                (*usx2y).in04_buf as *const core::ffi::c_void,
                core::mem::size_of_val(&(*usx2y).in04_last),
            );
            (*us428ctls).ctl_snapshot_last = -1;
        } else {
            i = 0;
            while i < 21 {
                if (*usx2y).in04_last[i as usize] != ((*usx2y).in04_buf as *mut c_char)[i as usize]
                {
                    if diff < 0 {
                        diff = i;
                    }
                    (*usx2y).in04_last[i as usize] =
                        ((*usx2y).in04_buf as *mut c_char)[i as usize];
                }
                i += 1;
            }
        }
        if diff >= 0 {
            n = (*us428ctls).ctl_snapshot_last + 1;
            if n >= N_US428_CTL_BUFS as i32 || n < 0 {
                n = 0;
            }
            memcpy(
                ((*us428ctls).ctl_snapshot as *mut u8).add(n as usize * core::mem::size_of::<[u8; 21]>())
                    as *mut core::ffi::c_void,
                (*usx2y).in04_buf as *const core::ffi::c_void,
                core::mem::size_of_val(&(*us428ctls).ctl_snapshot[0]),
            );
            (*us428ctls).ctl_snapshot_differs_at[n as usize] = diff;
            (*us428ctls).ctl_snapshot_last = n;
            wake_up(&(*usx2y).us428ctls_wait_queue_head as *const _ as *mut core::ffi::c_void);
        }
    }

    if !(*usx2y).us04.is_null() {
        if (*(*usx2y).us04).submitted == 0 {
            loop {
                err = usb_submit_urb(
                    (*(*usx2y).us04).urb[(*(*usx2y).us04).submitted as usize],
                    GFP_ATOMIC,
                );
                (*(*usx2y).us04).submitted += 1;
                if err != 0 || (*(*usx2y).us04).submitted >= (*(*usx2y).us04).len {
                    break;
                }
            }
        }
    } else {
        while !us428ctls.is_null()
            && (*us428ctls).p4out_last >= 0
            && (*us428ctls).p4out_last < N_US428_P4OUT_BUFS as i32
            && (*us428ctls).p4out_last != (*us428ctls).p4out_sent
        {
            j = 0;
            while j < URBS_ASYNC_SEQ as i32 && err == 0 {
                if (*(*usx2y).as04.urb[j as usize]).status == 0 {
                    send = (*us428ctls).p4out_sent + 1;
                    if send >= N_US428_P4OUT_BUFS as i32 {
                        send = 0;
                    }

                    p4out = (*us428ctls).p4out.add(send as usize);
                    len = if (*p4out).type_ == ELT_LIGHT {
                        core::mem::size_of::<us428_lights>() as i32
                    } else {
                        5
                    };
                    memcpy(
                        (*(*usx2y).as04.urb[j as usize]).transfer_buffer,
                        &(*p4out).val.vol as *const _ as *const core::ffi::c_void,
                        len as usize,
                    );
                    (*(*usx2y).as04.urb[j as usize]).transfer_buffer_length = len;
                    err = usb_submit_urb((*usx2y).as04.urb[j as usize], GFP_ATOMIC);
                    if err == 0 {
                        (*us428ctls).p4out_sent = send;
                    }

                    break;
                }
                j += 1;
            }
            if j >= URBS_ASYNC_SEQ as i32 || err != 0 {
                break;
            }
        }
    }

    if err != 0 {
        dev_err(
            &mut (*(*urb).dev).dev as *mut _ as *mut core::ffi::c_void,
            b"in04_int() usb_submit_urb err=%i\n".as_ptr() as *const c_char,
        );
    }

    (*urb).dev = (*usx2y).dev;
    usb_submit_urb(urb, GFP_ATOMIC);
}

/*
 * Prepare some urbs
 */
pub unsafe extern "C" fn usx2y_async_seq04_init(usx2y: *mut usx2ydev) -> i32 {
    let mut err: i32 = 0;
    let mut i: i32;

    if !(*usx2y).as04.buffer.is_null() {
        return -5; // -EBUSY
    }

    (*usx2y).as04.buffer = kmalloc_array(URBS_ASYNC_SEQ, URB_DATA_LEN_ASYNC_SEQ, GFP_KERNEL);
    if (*usx2y).as04.buffer.is_null() {
        err = -12; // -ENOMEM
    } else {
        i = 0;
        while i < URBS_ASYNC_SEQ as i32 {
            (*usx2y).as04.urb[i as usize] = usb_alloc_urb(0, GFP_KERNEL);
            if (*usx2y).as04.urb[i as usize].is_null() {
                err = -12; // -ENOMEM
                break;
            }
            usb_fill_bulk_urb(
                (*usx2y).as04.urb[i as usize],
                (*usx2y).dev,
                usb_sndbulkpipe((*usx2y).dev, 0x04),
                ((*usx2y).as04.buffer as *mut u8).add((URB_DATA_LEN_ASYNC_SEQ * i as usize) as usize)
                    as *mut core::ffi::c_void,
                0,
                i_usx2y_out04_int,
                usx2y as *mut core::ffi::c_void,
            );
            err = usb_urb_ep_type_check((*usx2y).as04.urb[i as usize]);
            if err < 0 {
                break;
            }
            i += 1;
        }
    }
    if err != 0 {
        usx2y_unlinkseq(&mut (*usx2y).as04);
    }
    err
}

pub unsafe extern "C" fn usx2y_in04_init(usx2y: *mut usx2ydev) -> i32 {
    let mut err: i32;

    if !(*usx2y).in04_urb.is_null() {
        return -5; // -EBUSY
    }

    (*usx2y).in04_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (*usx2y).in04_urb.is_null() {
        err = -12; // -ENOMEM
        // goto error
    } else {
        (*usx2y).in04_buf = kmalloc(21, GFP_KERNEL);
        if (*usx2y).in04_buf.is_null() {
            err = -12; // -ENOMEM
            // goto error
        } else {
            init_waitqueue_head(&mut (*usx2y).in04_wait_queue as *mut _ as *mut core::ffi::c_void);
            usb_fill_int_urb(
                (*usx2y).in04_urb,
                (*usx2y).dev,
                usb_rcvintpipe((*usx2y).dev, 0x4),
                (*usx2y).in04_buf,
                21,
                i_usx2y_in04_int,
                (*usx2y).dev,
                10,
            );
            if usb_urb_ep_type_check((*usx2y).in04_urb) != 0 {
                err = -22; // -EINVAL
                // goto error
            } else {
                return usb_submit_urb((*usx2y).in04_urb, GFP_KERNEL);
            }
        }
    }

    // error:
    kfree((*usx2y).in04_buf);
    usb_free_urb((*usx2y).in04_urb);
    (*usx2y).in04_buf = ptr::null_mut();
    (*usx2y).in04_urb = ptr::null_mut();
    err
}

unsafe extern "C" fn usx2y_unlinkseq_impl(s: *mut snd_usx2y_async_seq) {
    let mut i: i32;

    i = 0;
    while i < URBS_ASYNC_SEQ as i32 {
        if !(*s).urb[i as usize].is_null() {
            usb_kill_urb((*s).urb[i as usize]);
            usb_free_urb((*s).urb[i as usize]);
            (*s).urb[i as usize] = ptr::null_mut();
        }
        i += 1;
    }
    kfree((*s).buffer);
    (*s).buffer = ptr::null_mut();
}

#[repr(C)]
pub struct usb_device_id {
    pub match_flags: u16,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice_lo: u16,
    pub bcdDevice_hi: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
    pub bInterfaceNumber: u8,
    pub driver_info: usize,
}

const USB_DEVICE_ID_MATCH_DEVICE: u16 = 0x0001;

static SND_USX2Y_USB_ID_TABLE: [usb_device_id; 4] = [
    usb_device_id {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: 0x1604,
        idProduct: USB_ID_US428,
        bcdDevice_lo: 0,
        bcdDevice_hi: 0,
        bDeviceClass: 0,
        bDeviceSubClass: 0,
        bDeviceProtocol: 0,
        bInterfaceClass: 0,
        bInterfaceSubClass: 0,
        bInterfaceProtocol: 0,
        bInterfaceNumber: 0,
        driver_info: 0,
    },
    usb_device_id {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: 0x1604,
        idProduct: USB_ID_US122,
        bcdDevice_lo: 0,
        bcdDevice_hi: 0,
        bDeviceClass: 0,
        bDeviceSubClass: 0,
        bDeviceProtocol: 0,
        bInterfaceClass: 0,
        bInterfaceSubClass: 0,
        bInterfaceProtocol: 0,
        bInterfaceNumber: 0,
        driver_info: 0,
    },
    usb_device_id {
        match_flags: USB_DEVICE_ID_MATCH_DEVICE,
        idVendor: 0x1604,
        idProduct: USB_ID_US224,
        bcdDevice_lo: 0,
        bcdDevice_hi: 0,
        bDeviceClass: 0,
        bDeviceSubClass: 0,
        bDeviceProtocol: 0,
        bInterfaceClass: 0,
        bInterfaceSubClass: 0,
        bInterfaceProtocol: 0,
        bInterfaceNumber: 0,
        driver_info: 0,
    },
    usb_device_id {
        match_flags: 0,
        idVendor: 0,
        idProduct: 0,
        bcdDevice_lo: 0,
        bcdDevice_hi: 0,
        bDeviceClass: 0,
        bDeviceSubClass: 0,
        bDeviceProtocol: 0,
        bInterfaceClass: 0,
        bInterfaceSubClass: 0,
        bInterfaceProtocol: 0,
        bInterfaceNumber: 0,
        driver_info: 0,
    },
];

unsafe extern "C" fn usx2y_create_card(
    device: *mut usb_device,
    intf: *mut usb_interface,
    cardp: *mut *mut snd_card,
) -> i32 {
    let mut dev: i32;
    let mut card: *mut snd_card;
    let mut err: i32;

    dev = 0;
    while dev < SNDRV_CARDS as i32 {
        if ENABLE[dev as usize] && SND_USX2Y_CARD_USED[dev as usize] == 0 {
            break;
        }
        dev += 1;
    }
    if dev >= SNDRV_CARDS as i32 {
        return -19; // -ENODEV
    }

    err = snd_card_new(
        &(*intf).dev as *const _ as *mut core::ffi::c_void,
        INDEX[dev as usize],
        ID[dev as usize],
        ptr::null_mut(),
        core::mem::size_of::<usx2ydev>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }

    let usx2y_dev = usx2y(card);
    let card_idx = dev;
    SND_USX2Y_CARD_USED[card_idx as usize] = 1;
    (*card).private_free = Some(snd_usx2y_card_private_free);
    (*usx2y_dev).dev = device;
    init_waitqueue_head(&mut (*usx2y_dev).prepare_wait_queue as *mut _ as *mut core::ffi::c_void);
    init_waitqueue_head(
        &mut (*usx2y_dev).us428ctls_wait_queue_head as *mut _ as *mut core::ffi::c_void,
    );
    mutex_init(&mut (*usx2y_dev).pcm_mutex as *mut _ as *mut core::ffi::c_void);
    INIT_LIST_HEAD(&mut (*usx2y_dev).midi_list as *mut _ as *mut core::ffi::c_void);

    strscpy(
        (*card).driver.as_mut_ptr(),
        b"USB US428\0".as_ptr() as *const c_char,
        256,
    );
    sprintf(
        (*card).shortname.as_mut_ptr(),
        b"TASCAM US428\0".as_ptr() as *const c_char,
    );
    sprintf(
        (*card).longname.as_mut_ptr(),
        b"%s (%x:%x if %d at %03d/%03d)\0".as_ptr() as *const c_char,
    );

    *cardp = card;
    0
}

unsafe extern "C" fn snd_usx2y_card_private_free_impl(card: *mut snd_card) {
    let usx2y = usx2y(card);

    kfree((*usx2y).in04_buf);
    usb_free_urb((*usx2y).in04_urb);
    if !(*usx2y).us428ctls_sharedmem.is_null() {
        snd_card_free_pages_exact((*usx2y).us428ctls_sharedmem, 2); // US428_SHAREDMEM_PAGES = 2
    }
    if (*usx2y).card_index >= 0 && (*usx2y).card_index < SNDRV_CARDS as i32 {
        SND_USX2Y_CARD_USED[(*usx2y).card_index as usize] = 0;
    }
}

unsafe extern "C" fn snd_usx2y_disconnect(intf: *mut usb_interface) {
    let mut card: *mut snd_card;
    let mut usx2y: *mut usx2ydev;
    let mut p: *mut core::ffi::c_void;

    card = usb_get_intfdata(intf) as *mut snd_card;
    if card.is_null() {
        return;
    }
    usx2y = usx2y(card);
    (*usx2y).chip_status = USX2Y_STAT_CHIP_HUP;
    usx2y_unlinkseq(&mut (*usx2y).as04);
    usb_kill_urb((*usx2y).in04_urb);
    snd_card_disconnect(card);

    // release the midi resources
    // list_for_each(p, &usx2y->midi_list) {
    //     snd_usbmidi_disconnect(p);
    // }
    if !(*usx2y).us428ctls_sharedmem.is_null() {
        wake_up(&(*usx2y).us428ctls_wait_queue_head as *const _ as *mut core::ffi::c_void);
    }
    snd_card_free_when_closed(card);
}

unsafe extern "C" fn snd_usx2y_probe(intf: *mut usb_interface, id: *const usb_device_id) -> i32 {
    let device = interface_to_usbdev(intf);
    let mut card: *mut snd_card = ptr::null_mut();
    let mut err: i32;

    // #ifdef USX2Y_NRPACKS_VARIABLE
    // if (nrpacks < 0 || nrpacks > USX2Y_NRPACKS_MAX)
    //     return -EINVAL;
    // #endif

    if le16_to_cpu((*(*device).descriptor).idVendor) != 0x1604
        || (le16_to_cpu((*(*device).descriptor).idProduct) != USB_ID_US122
            && le16_to_cpu((*(*device).descriptor).idProduct) != USB_ID_US224
            && le16_to_cpu((*(*device).descriptor).idProduct) != USB_ID_US428)
    {
        return -22; // -EINVAL
    }

    err = usx2y_create_card(device, intf, &mut card);
    if err < 0 {
        return err;
    }
    err = usx2y_hwdep_new(card, device);
    if err < 0 {
        // goto error
        snd_card_free(card);
        return err;
    }
    err = snd_card_register(card);
    if err < 0 {
        // goto error
        snd_card_free(card);
        return err;
    }

    dev_set_drvdata(&(*intf).dev as *const _ as *mut core::ffi::c_void, card as *mut core::ffi::c_void);
    0
}

#[repr(C)]
pub struct usb_driver {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut usb_interface, *const usb_device_id) -> i32>,
    pub disconnect: Option<unsafe extern "C" fn(*mut usb_interface)>,
    pub id_table: *const usb_device_id,
}

static SND_USX2Y_USB_DRIVER: usb_driver = usb_driver {
    name: b"snd-usb-usx2y\0".as_ptr() as *const c_char,
    probe: Some(snd_usx2y_probe),
    disconnect: Some(snd_usx2y_disconnect),
    id_table: SND_USX2Y_USB_ID_TABLE.as_ptr(),
};

// module_usb_driver(snd_usx2y_usb_driver);
// TODO: kernel module initialization via module_usb_driver macro

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
