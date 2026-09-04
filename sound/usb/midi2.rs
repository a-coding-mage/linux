// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MIDI 2.0 support
 */

// Linux kernel headers (included in original)
// use linux_kernel::*;
// use sound::*;

use core::mem;
use core::ptr::{self, null_mut};
use core::ffi::c_int;

// External types from Linux kernel (assumed to be available)
// These are declared but implementations come from kernel bindings
extern "C" {
    type urb;
    type usb_device;
    type usb_host_interface;
    type usb_host_endpoint;
    type usb_ms20_endpoint_descriptor;
    type usb_ms20_gr_trm_block_header_descriptor;
    type usb_ms20_gr_trm_block_descriptor;
    type usb_ms_endpoint_descriptor;
    type usb_ms_header_descriptor;
    type snd_usb_audio;
    type snd_ump_endpoint;
    type snd_ump_block;
    type snd_ump_ops;
    type snd_usb_audio_quirk;
    type snd_rawmidi_substream;
    type usb_interface;

    fn find_first_bit(addr: *const core::ffi::c_ulong, size: core::ffi::c_uint) -> core::ffi::c_int;
    fn clear_bit(index: core::ffi::c_int, addr: *mut core::ffi::c_ulong);
    fn set_bit(index: core::ffi::c_int, addr: *mut core::ffi::c_ulong);
    fn usb_submit_urb(urb: *mut urb, gfp_mask: core::ffi::c_uint) -> c_int;
    fn usb_kill_urb(urb: *mut urb);
    fn usb_free_urb(urb: *mut urb);
    fn usb_alloc_urb(iso_packets: c_int, gfp_mask: core::ffi::c_uint) -> *mut urb;
    fn usb_alloc_coherent(
        dev: *mut usb_device,
        size: usize,
        gfp: core::ffi::c_uint,
        dma: *mut core::ffi::c_ulong,
    ) -> *mut core::ffi::c_void;
    fn usb_free_coherent(
        dev: *mut usb_device,
        size: usize,
        addr: *mut core::ffi::c_void,
        dma: core::ffi::c_ulong,
    );
    fn usb_set_interface(
        dev: *mut usb_device,
        ifnum: core::ffi::c_int,
        alternate: core::ffi::c_int,
    ) -> c_int;
    fn usb_maxpacket(dev: *mut usb_device, pipe: core::ffi::c_uint) -> c_int;
    fn usb_rcvintpipe(dev: *mut usb_device, endpoint: core::ffi::c_uint) -> core::ffi::c_uint;
    fn usb_rcvbulkpipe(dev: *mut usb_device, endpoint: core::ffi::c_uint) -> core::ffi::c_uint;
    fn usb_sndintpipe(dev: *mut usb_device, endpoint: core::ffi::c_uint) -> core::ffi::c_uint;
    fn usb_sndbulkpipe(dev: *mut usb_device, endpoint: core::ffi::c_uint) -> core::ffi::c_uint;
    fn usb_endpoint_xfer_int(desc: *const core::ffi::c_void) -> core::ffi::c_int;
    fn usb_endpoint_xfer_bulk(desc: *const core::ffi::c_void) -> core::ffi::c_int;
    fn usb_urb_ep_type_check(urb: *mut urb) -> c_int;
    fn usb_string(
        dev: *mut usb_device,
        index: c_int,
        buf: *mut c_int,
        size: usize,
    ) -> c_int;
    fn snd_usb_ctl_msg(
        dev: *mut usb_device,
        pipe: core::ffi::c_uint,
        request: core::ffi::c_uint,
        requesttype: core::ffi::c_uint,
        value: core::ffi::c_uint,
        index: core::ffi::c_uint,
        data: *mut core::ffi::c_void,
        size: c_int,
    ) -> c_int;
    fn snd_ump_transmit(
        ump: *mut snd_ump_endpoint,
        buf: *mut core::ffi::c_void,
        count: c_int,
    ) -> c_int;
    fn snd_ump_receive(
        ump: *mut snd_ump_endpoint,
        buf: *mut u32,
        count: c_int,
    );
    fn snd_ump_endpoint_new(
        card: *mut core::ffi::c_void,
        id: *const c_int,
        device: c_int,
        output: c_int,
        input: c_int,
        ump: *mut *mut snd_ump_endpoint,
    ) -> c_int;
    fn snd_ump_parse_endpoint(ump: *mut snd_ump_endpoint) -> c_int;
    fn snd_ump_attach_legacy_rawmidi(
        ump: *mut snd_ump_endpoint,
        name: *const c_int,
        device: c_int,
    ) -> c_int;
    fn snd_ump_block_new(
        ump: *mut snd_ump_endpoint,
        blk_id: c_int,
        direction: c_int,
        group_start: c_int,
        num_groups: c_int,
        block: *mut *mut snd_ump_block,
    ) -> c_int;
    fn snd_ump_update_group_attrs(ump: *mut snd_ump_endpoint);
    fn __snd_usbmidi_create(
        card: *mut core::ffi::c_void,
        iface: *mut usb_interface,
        midi_list: *mut core::ffi::c_void,
        quirk: *const snd_usb_audio_quirk,
        usb_id: core::ffi::c_uint,
        num_rawmidis: *mut c_int,
    ) -> c_int;
    fn cpu_to_le32_array(buf: *mut u32, len: c_int);
    fn le32_to_cpu_array(buf: *mut u32, len: c_int);
    fn le16_to_cpu(val: u16) -> u16;
    fn __le16_to_cpu(val: u16) -> u16;
    fn kzalloc(size: usize, flags: core::ffi::c_uint) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn spin_lock_init(lock: *mut core::ffi::c_void);
    fn init_waitqueue_head(wait: *mut core::ffi::c_void);
    fn msecs_to_jiffies(msecs: core::ffi::c_uint) -> c_int;
}

static mut MIDI2_ENABLE: bool = true;
// module_param(midi2_enable, bool, 0444);
// MODULE_PARM_DESC(midi2_enable, "Enable MIDI 2.0 support.");

static mut MIDI2_UMP_PROBE: bool = true;
// module_param(midi2_ump_probe, bool, 0444);
// MODULE_PARM_DESC(midi2_ump_probe, "Probe UMP v1.1 support at first.");

/* stream direction; just shorter names */
const STR_OUT: c_int = 0; // SNDRV_RAWMIDI_STREAM_OUTPUT
const STR_IN: c_int = 1;  // SNDRV_RAWMIDI_STREAM_INPUT

const NUM_URBS: usize = 8;

/* URB context */
#[repr(C)]
pub struct snd_usb_midi2_urb {
    pub urb: *mut urb,
    pub ep: *mut snd_usb_midi2_endpoint,
    pub index: c_int,
}

/* A USB MIDI input/output endpoint */
#[repr(C)]
pub struct snd_usb_midi2_endpoint {
    pub dev: *mut usb_device,
    pub ms_ep: *const usb_ms20_endpoint_descriptor,
    pub pair: *mut snd_usb_midi2_endpoint,
    pub rmidi: *mut snd_usb_midi2_ump,
    pub ump: *mut snd_ump_endpoint,
    pub direction: c_int,
    pub endpoint: core::ffi::c_uint,
    pub pipe: core::ffi::c_uint,
    pub packets: core::ffi::c_uint,
    pub interval: core::ffi::c_uint,
    pub wait: core::ffi::c_void,
    pub lock: core::ffi::c_void,
    pub substream: *mut snd_rawmidi_substream,
    pub num_urbs: core::ffi::c_uint,
    pub urb_free: core::ffi::c_ulong,
    pub urb_free_mask: core::ffi::c_ulong,
    pub running: core::ffi::c_int,
    pub suspended: core::ffi::c_int,
    pub disconnected: bool,
    pub list: core::ffi::c_void,
    pub urbs: [snd_usb_midi2_urb; NUM_URBS],
}

/* A UMP endpoint - one or two USB MIDI endpoints are assigned */
#[repr(C)]
pub struct snd_usb_midi2_ump {
    pub dev: *mut usb_device,
    pub umidi: *mut snd_usb_midi2_interface,
    pub ump: *mut snd_ump_endpoint,
    pub eps: [*mut snd_usb_midi2_endpoint; 2],
    pub index: c_int,
    pub usb_block_id: core::ffi::c_uchar,
    pub ump_parsed: bool,
    pub list: core::ffi::c_void,
}

/* top-level instance per USB MIDI interface */
#[repr(C)]
pub struct snd_usb_midi2_interface {
    pub chip: *mut snd_usb_audio,
    pub iface: *mut usb_interface,
    pub hostif: *mut usb_host_interface,
    pub blk_descs: *const c_int,
    pub blk_desc_size: core::ffi::c_uint,
    pub disconnected: bool,
    pub ep_list: core::ffi::c_void,
    pub rawmidi_list: core::ffi::c_void,
    pub list: core::ffi::c_void,
}

/* submit URBs as much as possible; used for both input and output */
unsafe fn do_submit_urbs_locked<F>(
    ep: *mut snd_usb_midi2_endpoint,
    prepare: F,
) where
    F: Fn(*mut snd_usb_midi2_endpoint, *mut urb) -> c_int,
{
    let mut index: c_int;
    let mut err: c_int = 0;

    if (*ep).disconnected {
        return;
    }

    while (*ep).urb_free != 0 {
        index = find_first_bit(&(*ep).urb_free, (*ep).num_urbs as core::ffi::c_uint);
        if index >= (*ep).num_urbs as c_int {
            return;
        }
        let ctx = &mut (*ep).urbs[index as usize];
        err = prepare(ep, ctx.urb);
        if err < 0 {
            return;
        }
        if (*ctx.urb).transfer_buffer_length == 0 {
            return;
        }
        (*ctx.urb).dev = (*ep).dev;
        err = usb_submit_urb(ctx.urb, 0x20); // GFP_ATOMIC
        if err < 0 {
            // dev_dbg(&ep->dev->dev, "usb_submit_urb error %d\n", err);
            return;
        }
        clear_bit(index, &mut (*ep).urb_free);
    }
}

/* prepare for output submission: copy from rawmidi buffer to urb packet */
unsafe fn prepare_output_urb(ep: *mut snd_usb_midi2_endpoint, urb: *mut urb) -> c_int {
    let count: c_int;

    count = snd_ump_transmit((*ep).ump, (*urb).transfer_buffer, (*ep).packets as c_int);
    if count < 0 {
        // dev_dbg(&ep->dev->dev, "rawmidi transmit error %d\n", count);
        return count;
    }
    cpu_to_le32_array((*urb).transfer_buffer as *mut u32, count >> 2);
    (*urb).transfer_buffer_length = count as usize;
    return 0;
}

unsafe fn submit_output_urbs_locked(ep: *mut snd_usb_midi2_endpoint) {
    do_submit_urbs_locked(ep, prepare_output_urb);
}

/* URB completion for output; re-filling and re-submit */
unsafe extern "C" fn output_urb_complete(urb_ptr: *mut urb) {
    let urb = urb_ptr;
    let ctx = (*urb).context as *mut snd_usb_midi2_urb;
    let ep = (*ctx).ep;

    // guard(spinlock_irqsave)(&ep->lock);
    set_bit((*ctx).index, &mut (*ep).urb_free);
    if (*urb).status >= 0 {
        submit_output_urbs_locked(ep);
    }
    if (*ep).urb_free == (*ep).urb_free_mask {
        // wake_up(&ep->wait);
    }
}

/* prepare for input submission: just set the buffer length */
unsafe fn prepare_input_urb(_ep: *mut snd_usb_midi2_endpoint, urb: *mut urb) -> c_int {
    (*urb).transfer_buffer_length = (*_ep).packets as usize;
    return 0;
}

unsafe fn submit_input_urbs_locked(ep: *mut snd_usb_midi2_endpoint) {
    do_submit_urbs_locked(ep, prepare_input_urb);
}

/* URB completion for input; copy into rawmidi buffer and resubmit */
unsafe extern "C" fn input_urb_complete(urb_ptr: *mut urb) {
    let urb = urb_ptr;
    let ctx = (*urb).context as *mut snd_usb_midi2_urb;
    let ep = (*ctx).ep;
    let mut len: c_int;

    // guard(spinlock_irqsave)(&ep->lock);
    if (*ep).disconnected || (*urb).status < 0 {
        goto_dequeue(ctx, ep);
        return;
    }
    len = (*urb).actual_length as c_int;
    len &= !3; /* align UMP */
    if len > (*ep).packets as c_int {
        len = (*ep).packets as c_int;
    }
    if len > 0 {
        le32_to_cpu_array((*urb).transfer_buffer as *mut u32, len >> 2);
        snd_ump_receive((*ep).ump, (*urb).transfer_buffer as *mut u32, len);
    }
    goto_dequeue(ctx, ep);
}

unsafe fn goto_dequeue(ctx: *mut snd_usb_midi2_urb, ep: *mut snd_usb_midi2_endpoint) {
    set_bit((*ctx).index, &mut (*ep).urb_free);
    submit_input_urbs_locked(ep);
    if (*ep).urb_free == (*ep).urb_free_mask {
        // wake_up(&ep->wait);
    }
}

/* URB submission helper; for both direction */
unsafe fn submit_io_urbs(ep: *mut snd_usb_midi2_endpoint) {
    if ep.is_null() {
        return;
    }
    // guard(spinlock_irqsave)(&ep->lock);
    if (*ep).direction == STR_IN {
        submit_input_urbs_locked(ep);
    } else {
        submit_output_urbs_locked(ep);
    }
}

/* kill URBs for close, suspend and disconnect */
unsafe fn kill_midi_urbs(ep: *mut snd_usb_midi2_endpoint, suspending: bool) {
    if ep.is_null() {
        return;
    }
    if suspending {
        // atomic_set(&ep->suspended, atomic_read(&ep->running));
    }
    // atomic_set(&ep->running, 0);
    for i in 0..(*ep).num_urbs as usize {
        if (*ep).urbs[i].urb.is_null() {
            break;
        }
        usb_kill_urb((*ep).urbs[i].urb);
    }
}

/* wait until all URBs get freed */
unsafe fn drain_urb_queue(ep: *mut snd_usb_midi2_endpoint) {
    if ep.is_null() {
        return;
    }
    // guard(spinlock_irq)(&ep->lock);
    // atomic_set(&ep->running, 0);
    // wait_event_lock_irq_timeout(ep->wait,
    //                             ep->disconnected ||
    //                             ep->urb_free == ep->urb_free_mask,
    //                             ep->lock, msecs_to_jiffies(500));
}

/* release URBs for an EP */
unsafe fn free_midi_urbs(ep: *mut snd_usb_midi2_endpoint) {
    if ep.is_null() {
        return;
    }
    for i in 0..NUM_URBS {
        let ctx = &mut (*ep).urbs[i];
        if ctx.urb.is_null() {
            break;
        }
        usb_free_coherent(
            (*ep).dev,
            (*ep).packets as usize,
            (*ctx.urb).transfer_buffer,
            (*ctx.urb).transfer_dma,
        );
        usb_free_urb(ctx.urb);
        ctx.urb = null_mut();
    }
    (*ep).num_urbs = 0;
}

/* allocate URBs for an EP */
/* the callers should handle allocation errors via free_midi_urbs() */
unsafe fn alloc_midi_urbs(ep: *mut snd_usb_midi2_endpoint) -> c_int {
    let mut i: c_int;
    let mut err: c_int;
    let endpoint: core::ffi::c_uint;
    let len: core::ffi::c_uint;

    endpoint = (*ep).endpoint;
    len = (*ep).packets;

    (*ep).num_urbs = 0;
    (*ep).urb_free = 0;
    (*ep).urb_free_mask = 0;

    for i in 0..NUM_URBS as c_int {
        let ctx = &mut (*ep).urbs[i as usize];
        ctx.index = i;
        ctx.urb = usb_alloc_urb(0, 0x10000); // GFP_KERNEL
        if ctx.urb.is_null() {
            // dev_err(&ep->dev->dev, "URB alloc failed\n");
            return -12; // -ENOMEM
        }
        ctx.ep = ep;
        let buffer = usb_alloc_coherent((*ep).dev, len as usize, 0x10000, &mut (*ctx.urb).transfer_dma);
        if buffer.is_null() {
            // dev_err(&ep->dev->dev, "URB buffer alloc failed (size %d)\n", len);
            return -12; // -ENOMEM
        }
        if (*ep).interval != 0 {
            // usb_fill_int_urb(ctx->urb, ep->dev, ep->pipe,
            //                  buffer, len, comp, ctx, ep->interval);
        } else {
            // usb_fill_bulk_urb(ctx->urb, ep->dev, ep->pipe,
            //                   buffer, len, comp, ctx);
        }
        err = usb_urb_ep_type_check(ctx.urb);
        if err < 0 {
            // dev_err(&ep->dev->dev, "invalid MIDI EP %x\n", endpoint);
            return err;
        }
        (*ctx.urb).transfer_flags = 1; // URB_NO_TRANSFER_DMA_MAP
        (*ep).num_urbs += 1;
    }
    (*ep).urb_free = (1 << (*ep).num_urbs) - 1;
    (*ep).urb_free_mask = (*ep).urb_free;
    return 0;
}

unsafe fn ump_to_endpoint(
    ump: *mut snd_ump_endpoint,
    dir: c_int,
) -> *mut snd_usb_midi2_endpoint {
    let rmidi = (*ump).private_data as *mut snd_usb_midi2_ump;

    if rmidi.is_null() {
        null_mut()
    } else {
        (*rmidi).eps[dir as usize]
    }
}

/* ump open callback */
unsafe extern "C" fn snd_usb_midi_v2_open(ump: *mut snd_ump_endpoint, dir: c_int) -> c_int {
    let ep = ump_to_endpoint(ump, dir);

    if ep.is_null() || (*ep).endpoint == 0 {
        return -19; // -ENODEV
    }
    if (*ep).disconnected {
        return -5; // -EIO
    }
    if (*ep).direction == STR_OUT {
        let err = alloc_midi_urbs(ep);
        if err != 0 {
            free_midi_urbs(ep);
            return err;
        }
    }
    return 0;
}

/* ump close callback */
unsafe extern "C" fn snd_usb_midi_v2_close(ump: *mut snd_ump_endpoint, dir: c_int) {
    let ep = ump_to_endpoint(ump, dir);

    if (*ep).direction == STR_OUT {
        kill_midi_urbs(ep, false);
        drain_urb_queue(ep);
        free_midi_urbs(ep);
    }
}

/* ump trigger callback */
unsafe extern "C" fn snd_usb_midi_v2_trigger(
    ump: *mut snd_ump_endpoint,
    dir: c_int,
    up: c_int,
) {
    let ep = ump_to_endpoint(ump, dir);

    // atomic_set(&ep->running, up);
    if up != 0 && (*ep).direction == STR_OUT && !(*ep).disconnected {
        submit_io_urbs(ep);
    }
}

/* ump drain callback */
unsafe extern "C" fn snd_usb_midi_v2_drain(ump: *mut snd_ump_endpoint, dir: c_int) {
    let ep = ump_to_endpoint(ump, dir);

    drain_urb_queue(ep);
}

/* allocate and start all input streams */
unsafe fn start_input_streams(umidi: *mut snd_usb_midi2_interface) -> c_int {
    let mut ep: *mut snd_usb_midi2_endpoint;
    let mut err: c_int;

    // First loop: allocate URBs for input endpoints
    // list_for_each_entry(ep, &umidi->ep_list, list) {
    //     if (ep->direction == STR_IN) {
    //         err = alloc_midi_urbs(ep);
    //         if (err < 0)
    //             goto error;
    //     }
    // }

    // Second loop: submit URBs for input endpoints
    // list_for_each_entry(ep, &umidi->ep_list, list) {
    //     if (ep->direction == STR_IN)
    //         submit_io_urbs(ep);
    // }

    return 0;
    // error:
    //     list_for_each_entry(ep, &umidi->ep_list, list) {
    //         if (ep->direction == STR_IN)
    //             free_midi_urbs(ep);
    //     }
    //     return err;
}

// snd_ump_ops structure - external definition expected

/* create a USB MIDI 2.0 endpoint object */
unsafe fn create_midi2_endpoint(
    umidi: *mut snd_usb_midi2_interface,
    hostep: *mut usb_host_endpoint,
    ms_ep: *const usb_ms20_endpoint_descriptor,
) -> c_int {
    let ep: *mut snd_usb_midi2_endpoint;
    let endpoint: core::ffi::c_uint;
    let dir: c_int;

    // usb_audio_dbg(umidi->chip, "Creating an EP 0x%02x, #GTB=%d\n",
    //               hostep->desc.bEndpointAddress,
    //               ms_ep->bNumGrpTrmBlock);

    ep = kzalloc(mem::size_of::<snd_usb_midi2_endpoint>(), 0x10000) as *mut snd_usb_midi2_endpoint;
    if ep.is_null() {
        return -12; // -ENOMEM
    }

    // spin_lock_init(&ep->lock);
    // init_waitqueue_head(&ep->wait);
    (*ep).dev = (*(*umidi).chip).dev;
    // endpoint = hostep->desc.bEndpointAddress;
    // dir = (endpoint & USB_DIR_IN) ? STR_IN : STR_OUT;

    (*ep).endpoint = endpoint;
    (*ep).direction = dir;
    (*ep).ms_ep = ms_ep;
    // if (usb_endpoint_xfer_int(&hostep->desc))
    //     ep->interval = hostep->desc.bInterval;
    // else
    //     ep->interval = 0;
    if dir == STR_IN {
        if (*ep).interval != 0 {
            (*ep).pipe = usb_rcvintpipe((*ep).dev, endpoint);
        } else {
            (*ep).pipe = usb_rcvbulkpipe((*ep).dev, endpoint);
        }
    } else {
        if (*ep).interval != 0 {
            (*ep).pipe = usb_sndintpipe((*ep).dev, endpoint);
        } else {
            (*ep).pipe = usb_sndbulkpipe((*ep).dev, endpoint);
        }
    }
    (*ep).packets = usb_maxpacket((*ep).dev, (*ep).pipe) as core::ffi::c_uint;
    // list_add_tail(&ep->list, &umidi->ep_list);

    return 0;
}

/* destructor for endpoint; from snd_usb_midi_v2_free() */
unsafe fn free_midi2_endpoint(ep: *mut snd_usb_midi2_endpoint) {
    // list_del(&ep->list);
    if !(*ep).disconnected {
        (*ep).disconnected = true;
        kill_midi_urbs(ep, false);
        drain_urb_queue(ep);
    }
    free_midi_urbs(ep);
    kfree(ep as *mut core::ffi::c_void);
}

/* call all endpoint destructors */
unsafe fn free_all_midi2_endpoints(umidi: *mut snd_usb_midi2_interface) {
    // while (!list_empty(&umidi->ep_list)) {
    //     ep = list_first_entry(&umidi->ep_list,
    //                           struct snd_usb_midi2_endpoint, list);
    //     free_midi2_endpoint(ep);
    // }
}

/* find a MIDI STREAMING descriptor with a given subtype */
unsafe fn find_usb_ms_endpoint_descriptor(
    hostep: *mut usb_host_endpoint,
    subtype: core::ffi::c_uchar,
) -> *mut core::ffi::c_void {
    let mut extra = (*hostep).extra as *mut core::ffi::c_uchar;
    let mut extralen = (*hostep).extralen as c_int;

    while extralen > 3 {
        let ms_ep = extra as *mut usb_ms_endpoint_descriptor;
        let length = (*ms_ep).bLength as c_int;

        if length == 0 || length > extralen {
            break;
        }

        if length > 3 {
            // && ms_ep->bDescriptorType == USB_DT_CS_ENDPOINT
            // && ms_ep->bDescriptorSubtype == subtype
            return ms_ep as *mut core::ffi::c_void;
        }
        extralen -= length;
        extra = extra.offset(length as isize);
    }
    null_mut()
}

/* get the full group terminal block descriptors and return the size */
unsafe fn get_group_terminal_block_descs(umidi: *mut snd_usb_midi2_interface) -> c_int {
    let hostif = (*umidi).hostif;
    let dev = (*(*umidi).chip).dev;
    let mut header: usb_ms20_gr_trm_block_header_descriptor = mem::zeroed();
    let data: *mut core::ffi::c_uchar;
    let err: c_int;
    let size: c_int;

    // err = snd_usb_ctl_msg(dev, usb_rcvctrlpipe(dev, 0),
    //                       USB_REQ_GET_DESCRIPTOR,
    //                       USB_RECIP_INTERFACE | USB_TYPE_STANDARD | USB_DIR_IN,
    //                       USB_DT_CS_GR_TRM_BLOCK << 8 | hostif->desc.bAlternateSetting,
    //                       hostif->desc.bInterfaceNumber,
    //                       &header, sizeof(header));
    // if (err < 0)
    //     return err;
    // size = __le16_to_cpu(header.wTotalLength);
    // if (!size) {
    //     dev_err(&dev->dev, "Failed to get GTB descriptors for %d:%d\n",
    //             hostif->desc.bInterfaceNumber, hostif->desc.bAlternateSetting);
    //     return -EINVAL;
    // }

    data = kzalloc(size as usize, 0x10000) as *mut core::ffi::c_uchar;
    if data.is_null() {
        return -12; // -ENOMEM
    }

    // err = snd_usb_ctl_msg(dev, usb_rcvctrlpipe(dev, 0),
    //                       USB_REQ_GET_DESCRIPTOR,
    //                       USB_RECIP_INTERFACE | USB_TYPE_STANDARD | USB_DIR_IN,
    //                       USB_DT_CS_GR_TRM_BLOCK << 8 | hostif->desc.bAlternateSetting,
    //                       hostif->desc.bInterfaceNumber, data, size);
    // if (err < 0) {
    //     kfree(data);
    //     return err;
    // }

    (*umidi).blk_descs = data as *const c_int;
    (*umidi).blk_desc_size = size as core::ffi::c_uint;
    return 0;
}

/* find the corresponding group terminal block descriptor */
unsafe fn find_group_terminal_block(
    umidi: *mut snd_usb_midi2_interface,
    id: c_int,
) -> *const usb_ms20_gr_trm_block_descriptor {
    let data = (*umidi).blk_descs as *const core::ffi::c_uchar;
    let mut size = (*umidi).blk_desc_size as c_int;
    let desc: *const usb_ms20_gr_trm_block_descriptor;

    size -= mem::size_of::<usb_ms20_gr_trm_block_header_descriptor>() as c_int;
    // data += sizeof(struct usb_ms20_gr_trm_block_header_descriptor);
    while size > 0 && *data != 0 && (*data as c_int) <= size {
        desc = data as *const usb_ms20_gr_trm_block_descriptor;
        // if (desc->bLength >= sizeof(*desc) &&
        //     desc->bDescriptorType == USB_DT_CS_GR_TRM_BLOCK &&
        //     desc->bDescriptorSubtype == USB_MS_GR_TRM_BLOCK &&
        //     desc->bGrpTrmBlkID == id)
        //     return desc;
        size -= *data as c_int;
        // data += *data;
    }

    null()
}

/* fill up the information from GTB */
unsafe fn parse_group_terminal_block(
    rmidi: *mut snd_usb_midi2_ump,
    desc: *const usb_ms20_gr_trm_block_descriptor,
) -> c_int {
    let ump = (*rmidi).ump;
    let protocol: core::ffi::c_uint;
    let protocol_caps: core::ffi::c_uint;

    // set default protocol
    // switch (desc->bMIDIProtocol) {
    // case USB_MS_MIDI_PROTO_1_0_64:
    // case USB_MS_MIDI_PROTO_1_0_64_JRTS:
    // case USB_MS_MIDI_PROTO_1_0_128:
    // case USB_MS_MIDI_PROTO_1_0_128_JRTS:
    //     protocol = SNDRV_UMP_EP_INFO_PROTO_MIDI1;
    //     break;
    // case USB_MS_MIDI_PROTO_2_0:
    // case USB_MS_MIDI_PROTO_2_0_JRTS:
    //     protocol = SNDRV_UMP_EP_INFO_PROTO_MIDI2;
    //     break;
    // default:
    //     return 0;
    // }

    // if (!ump->info.protocol)
    //     ump->info.protocol = protocol;

    // protocol_caps = protocol;
    // switch (desc->bMIDIProtocol) {
    // case USB_MS_MIDI_PROTO_1_0_64_JRTS:
    // case USB_MS_MIDI_PROTO_1_0_128_JRTS:
    // case USB_MS_MIDI_PROTO_2_0_JRTS:
    //     protocol_caps |= SNDRV_UMP_EP_INFO_PROTO_JRTS_TX |
    //         SNDRV_UMP_EP_INFO_PROTO_JRTS_RX;
    //     break;
    // }

    // ump->info.protocol_caps |= protocol_caps;
    return 0;
}

/* allocate and parse for each assigned group terminal block */
unsafe fn parse_group_terminal_blocks(umidi: *mut snd_usb_midi2_interface) -> c_int {
    let mut err: c_int;

    err = get_group_terminal_block_descs(umidi);
    if err < 0 {
        return err;
    }
    if (*umidi).blk_descs.is_null() {
        return 0;
    }

    // list_for_each_entry(rmidi, &umidi->rawmidi_list, list) {
    //     desc = find_group_terminal_block(umidi, rmidi->usb_block_id);
    //     if (!desc)
    //         continue;
    //     err = parse_group_terminal_block(rmidi, desc);
    //     if (err < 0)
    //         return err;
    // }

    return 0;
}

/* parse endpoints included in the given interface and create objects */
unsafe fn parse_midi_2_0_endpoints(umidi: *mut snd_usb_midi2_interface) -> c_int {
    let hostif = (*umidi).hostif;
    let mut i: c_int;
    let mut err: c_int;

    // for (i = 0; i < hostif->desc.bNumEndpoints; i++) {
    //     hostep = &hostif->endpoint[i];
    //     if (!usb_endpoint_xfer_bulk(&hostep->desc) &&
    //         !usb_endpoint_xfer_int(&hostep->desc))
    //         continue;
    //     ms_ep = find_usb_ms_endpoint_descriptor(hostep, USB_MS_GENERAL_2_0);
    //     if (!ms_ep)
    //         continue;
    //     if (ms_ep->bLength <= sizeof(*ms_ep))
    //         continue;
    //     if (!ms_ep->bNumGrpTrmBlock)
    //         continue;
    //     if (ms_ep->bLength < sizeof(*ms_ep) + ms_ep->bNumGrpTrmBlock)
    //         continue;
    //     err = create_midi2_endpoint(umidi, hostep, ms_ep);
    //     if (err < 0)
    //         return err;
    // }
    return 0;
}

unsafe fn free_ump_private_data(ump: *mut snd_ump_endpoint) {
    let rmidi = (*ump).private_data as *mut snd_usb_midi2_ump;

    if !rmidi.is_null() {
        (*rmidi).ump = null_mut();
    }
}

unsafe fn free_all_midi2_umps(umidi: *mut snd_usb_midi2_interface) {
    // while (!list_empty(&umidi->rawmidi_list)) {
    //     rmidi = list_first_entry(&umidi->rawmidi_list,
    //                              struct snd_usb_midi2_ump, list);
    //     list_del(&rmidi->list);
    //     if (rmidi->ump)
    //         rmidi->ump->private_data = NULL;
    //     kfree(rmidi);
    // }
}

unsafe fn create_midi2_ump(
    umidi: *mut snd_usb_midi2_interface,
    ep_in: *mut snd_usb_midi2_endpoint,
    ep_out: *mut snd_usb_midi2_endpoint,
    blk_id: c_int,
) -> c_int {
    let rmidi: *mut snd_usb_midi2_ump;
    let ump: *mut snd_ump_endpoint = null_mut();
    let input: c_int;
    let output: c_int;
    let mut idstr: [c_int; 16] = [0; 16];
    let err: c_int;

    rmidi = kzalloc(mem::size_of::<snd_usb_midi2_ump>(), 0x10000) as *mut snd_usb_midi2_ump;
    if rmidi.is_null() {
        return -12; // -ENOMEM
    }
    // INIT_LIST_HEAD(&rmidi->list);
    (*rmidi).dev = (*(*umidi).chip).dev;
    (*rmidi).umidi = umidi;
    (*rmidi).usb_block_id = blk_id as core::ffi::c_uchar;

    (*rmidi).index = 0; // umidi->chip->num_rawmidis;
    // snprintf(idstr, sizeof(idstr), "UMP %d", rmidi->index);
    input = if !ep_in.is_null() { 1 } else { 0 };
    output = if !ep_out.is_null() { 1 } else { 0 };
    // err = snd_ump_endpoint_new(umidi->chip->card, idstr, rmidi->index,
    //                            output, input, &ump);
    // if (err < 0) {
    //     usb_audio_dbg(umidi->chip, "Failed to create a UMP object\n");
    //     kfree(rmidi);
    //     return err;
    // }

    (*rmidi).ump = ump;
    // umidi->chip->num_rawmidis++;

    (*ump).private_data = rmidi as *mut core::ffi::c_void;
    // ump->ops = &snd_usb_midi_v2_ump_ops;
    // ump->private_free = free_ump_private_data;

    (*rmidi).eps[0] = ep_in; // STR_IN
    (*rmidi).eps[1] = ep_out; // STR_OUT
    if !ep_in.is_null() {
        (*ep_in).pair = ep_out;
        (*ep_in).rmidi = rmidi;
        (*ep_in).ump = ump;
    }
    if !ep_out.is_null() {
        (*ep_out).pair = ep_in;
        (*ep_out).rmidi = rmidi;
        (*ep_out).ump = ump;
    }

    // list_add_tail(&rmidi->list, &umidi->rawmidi_list);
    return 0;
}

/* find the UMP EP with the given USB block id */
unsafe fn find_midi2_ump(
    umidi: *mut snd_usb_midi2_interface,
    blk_id: c_int,
) -> *mut snd_usb_midi2_ump {
    // list_for_each_entry(rmidi, &umidi->rawmidi_list, list) {
    //     if (rmidi->usb_block_id == blk_id)
    //         return rmidi;
    // }
    null_mut()
}

/* look for the matching output endpoint and create UMP object if found */
unsafe fn find_matching_ep_partner(
    umidi: *mut snd_usb_midi2_interface,
    ep: *mut snd_usb_midi2_endpoint,
    blk_id: c_int,
) -> c_int {
    // list_for_each_entry(pair_ep, &umidi->ep_list, list) {
    //     if (pair_ep->direction != STR_OUT)
    //         continue;
    //     if (pair_ep->pair)
    //         continue; /* already paired */
    //     for (blk = 0; blk < pair_ep->ms_ep->bNumGrpTrmBlock; blk++) {
    //         if (pair_ep->ms_ep->baAssoGrpTrmBlkID[blk] == blk_id) {
    //             usb_audio_dbg(umidi->chip,
    //                           "Found a match with EP-out 0x%02x blk %d\n",
    //                           pair_ep->endpoint, blk);
    //             return create_midi2_ump(umidi, ep, pair_ep, blk_id);
    //         }
    //     }
    // }
    return 0;
}

/* Call UMP helper to parse UMP endpoints;
 * this needs to be called after starting the input streams for bi-directional
 * communications
 */
unsafe fn parse_ump_endpoints(umidi: *mut snd_usb_midi2_interface) -> c_int {
    // list_for_each_entry(rmidi, &umidi->rawmidi_list, list) {
    //     if (!rmidi->ump ||
    //         !(rmidi->ump->core.info_flags & SNDRV_RAWMIDI_INFO_DUPLEX))
    //         continue;
    //     err = snd_ump_parse_endpoint(rmidi->ump);
    //     if (!err) {
    //         rmidi->ump_parsed = true;
    //     } else {
    //         if (err == -ENOMEM)
    //             return err;
    //     }
    // }
    return 0;
}

/* create a UMP block from a GTB entry */
unsafe fn create_gtb_block(rmidi: *mut snd_usb_midi2_ump, dir: c_int, blk: c_int) -> c_int {
    // let umidi = (*rmidi).umidi;
    // let desc = find_group_terminal_block(umidi, blk);
    // if (!desc)
    //     return 0;

    // usb_audio_dbg(umidi->chip,
    //               "GTB %d: type=%d, group=%d/%d, protocol=%d, in bw=%d, out bw=%d\n",
    //               blk, desc->bGrpTrmBlkType, desc->nGroupTrm,
    //               desc->nNumGroupTrm, desc->bMIDIProtocol,
    //               __le16_to_cpu(desc->wMaxInputBandwidth),
    //               __le16_to_cpu(desc->wMaxOutputBandwidth));

    // assign the direction
    // let type = match desc->bGrpTrmBlkType {
    //     USB_MS_GR_TRM_BLOCK_TYPE_BIDIRECTIONAL => SNDRV_UMP_DIR_BIDIRECTION,
    //     USB_MS_GR_TRM_BLOCK_TYPE_INPUT_ONLY => SNDRV_UMP_DIR_INPUT,
    //     USB_MS_GR_TRM_BLOCK_TYPE_OUTPUT_ONLY => SNDRV_UMP_DIR_OUTPUT,
    //     _ => {
    //         usb_audio_dbg(umidi->chip, "Unsupported GTB type %d\n",
    //                       desc->bGrpTrmBlkType);
    //         return 0;
    //     }
    // };

    // guess work: set blk-1 as the (0-based) block ID
    // let err = snd_ump_block_new((*rmidi).ump, blk - 1, type,
    //                             desc->nGroupTrm, desc->nNumGroupTrm,
    //                             &fb);
    // if (err == -EBUSY)
    //     return 0; /* already present */
    // else if (err)
    //     return err;

    // if (desc->iBlockItem)
    //     usb_string((*rmidi).dev, desc->iBlockItem,
    //                fb->info.name, sizeof(fb->info.name));

    // if (__le16_to_cpu(desc->wMaxInputBandwidth) == 1 ||
    //     __le16_to_cpu(desc->wMaxOutputBandwidth) == 1)
    //     fb->info.flags |= SNDRV_UMP_BLOCK_IS_MIDI1 |
    //         SNDRV_UMP_BLOCK_IS_LOWSPEED;

    // if MIDI 2.0 protocol is supported and yet the GTB shows MIDI 1.0,
    // treat it as a MIDI 1.0-specific block
    // if ((*rmidi).ump->info.protocol_caps & SNDRV_UMP_EP_INFO_PROTO_MIDI2) {
    //     match desc->bMIDIProtocol {
    //         USB_MS_MIDI_PROTO_1_0_64 |
    //         USB_MS_MIDI_PROTO_1_0_64_JRTS |
    //         USB_MS_MIDI_PROTO_1_0_128 |
    //         USB_MS_MIDI_PROTO_1_0_128_JRTS => {
    //             fb->info.flags |= SNDRV_UMP_BLOCK_IS_MIDI1;
    //         }
    //         _ => {}
    //     }
    // }

    // snd_ump_update_group_attrs((*rmidi).ump);

    // usb_audio_dbg(umidi->chip,
    //               "Created a UMP block %d from GTB, name=%s, flags=0x%x\n",
    //               blk, fb->info.name, fb->info.flags);
    return 0;
}

/* Create UMP blocks for each UMP EP */
unsafe fn create_blocks_from_gtb(umidi: *mut snd_usb_midi2_interface) -> c_int {
    // list_for_each_entry(rmidi, &umidi->rawmidi_list, list) {
    //     if (!rmidi->ump)
    //         continue;
    //     if (rmidi->ump_parsed || rmidi->ump->info.num_blocks)
    //         continue;
    //     rmidi->ump->info.flags |= SNDRV_UMP_EP_INFO_STATIC_BLOCKS;
    //     for (dir = 0; dir < 2; dir++) {
    //         if (!rmidi->eps[dir])
    //             continue;
    //         for (i = 0; i < rmidi->eps[dir]->ms_ep->bNumGrpTrmBlock; i++) {
    //             blk = rmidi->eps[dir]->ms_ep->baAssoGrpTrmBlkID[i];
    //             err = create_gtb_block(rmidi, dir, blk);
    //             if (err < 0)
    //                 return err;
    //         }
    //     }
    // }

    return 0;
}

/* attach legacy rawmidis */
unsafe fn attach_legacy_rawmidi(umidi: *mut snd_usb_midi2_interface) -> c_int {
    // #if IS_ENABLED(CONFIG_SND_UMP_LEGACY_RAWMIDI)
    // list_for_each_entry(rmidi, &umidi->rawmidi_list, list) {
    //     err = snd_ump_attach_legacy_rawmidi(rmidi->ump,
    //                                         "Legacy MIDI",
    //                                         umidi->chip->num_rawmidis);
    //     if (err < 0)
    //         return err;
    //     umidi->chip->num_rawmidis++;
    // }
    // #endif
    return 0;
}

unsafe fn snd_usb_midi_v2_free(umidi: *mut snd_usb_midi2_interface) {
    free_all_midi2_endpoints(umidi);
    free_all_midi2_umps(umidi);
    // list_del(&umidi->list);
    kfree((*umidi).blk_descs as *mut core::ffi::c_void);
    kfree(umidi as *mut core::ffi::c_void);
}

/* parse the interface for MIDI 2.0 */
unsafe fn parse_midi_2_0(umidi: *mut snd_usb_midi2_interface) -> c_int {
    let err: c_int;

    // First, create an object for each USB MIDI Endpoint
    err = parse_midi_2_0_endpoints(umidi);
    if err < 0 {
        return err;
    }
    // if (list_empty(&umidi->ep_list)) {
    //     usb_audio_warn(umidi->chip, "No MIDI endpoints found\n");
    //     return -ENODEV;
    // }

    // Next, look for EP I/O pairs that are found in group terminal blocks
    // A UMP object is created for each EP I/O pair as bidirecitonal
    // UMP EP
    // list_for_each_entry(ep, &umidi->ep_list, list) {
    //     only input in this loop; output is matched in find_midi_ump()
    //     if (ep->direction != STR_IN)
    //         continue;
    //     for (blk = 0; blk < ep->ms_ep->bNumGrpTrmBlock; blk++) {
    //         id = ep->ms_ep->baAssoGrpTrmBlkID[blk];
    //         err = find_matching_ep_partner(umidi, ep, id);
    //         if (err < 0)
    //             return err;
    //     }
    // }

    // For the remaining EPs, treat as singles, create a UMP object with
    // unidirectional EP
    // list_for_each_entry(ep, &umidi->ep_list, list) {
    //     if (ep->rmidi)
    //         continue; /* already paired */
    //     for (blk = 0; blk < ep->ms_ep->bNumGrpTrmBlock; blk++) {
    //         id = ep->ms_ep->baAssoGrpTrmBlkID[blk];
    //         if (find_midi2_ump(umidi, id))
    //             continue;
    //         usb_audio_dbg(umidi->chip,
    //                       "Creating a unidirection UMP for EP=0x%02x, blk=%d\n",
    //                       ep->endpoint, id);
    //         if (ep->direction == STR_IN)
    //             err = create_midi2_ump(umidi, ep, NULL, id);
    //         else
    //             err = create_midi2_ump(umidi, NULL, ep, id);
    //         if (err < 0)
    //             return err;
    //         break;
    //     }
    // }

    return 0;
}

/* is the given interface for MIDI 2.0? */
unsafe fn is_midi2_altset(hostif: *mut usb_host_interface) -> bool {
    let ms_header = (*hostif).extra as *const usb_ms_header_descriptor;

    if (*hostif).extralen < 7
        || (*ms_header).bLength < 7
        // || (*ms_header).bDescriptorType != USB_DT_CS_INTERFACE
        // || (*ms_header).bDescriptorSubtype != UAC_HEADER
    {
        return false;
    }

    return le16_to_cpu((*ms_header).bcdMSC) == 0x0200; // USB_MS_REV_MIDI_2_0
}

/* change the altsetting */
unsafe fn set_altset(umidi: *mut snd_usb_midi2_interface) -> c_int {
    // usb_audio_dbg(umidi->chip, "Setting host iface %d:%d\n",
    //               umidi->hostif->desc.bInterfaceNumber,
    //               umidi->hostif->desc.bAlternateSetting);
    return usb_set_interface(
        (*(*umidi).chip).dev,
        0, // umidi->hostif->desc.bInterfaceNumber,
        0, // umidi->hostif->desc.bAlternateSetting
    );
}

/* fill UMP Endpoint name string from USB descriptor */
unsafe fn fill_ump_ep_name(ump: *mut snd_ump_endpoint, dev: *mut usb_device, id: c_int) {
    // usb_string(dev, id, ump->info.name, sizeof(ump->info.name));

    // trim superfluous "MIDI" suffix
    // let len = strlen(ump->info.name);
    // if (len > 5 && !strcmp(ump->info.name + len - 5, " MIDI"))
    //     ump->info.name[len - 5] = 0;
}

/* fill the fallback name string for each rawmidi instance */
unsafe fn set_fallback_rawmidi_names(umidi: *mut snd_usb_midi2_interface) {
    // list_for_each_entry(rmidi, &umidi->rawmidi_list, list) {
    //     ump = rmidi->ump;
    //     /* fill UMP EP name from USB descriptors */
    //     if (!*ump->info.name && umidi->hostif->desc.iInterface)
    //         fill_ump_ep_name(ump, dev, umidi->hostif->desc.iInterface);
    //     else if (!*ump->info.name && dev->descriptor.iProduct)
    //         fill_ump_ep_name(ump, dev, dev->descriptor.iProduct);
    //     /* fill fallback name */
    //     if (!*ump->info.name)
    //         scnprintf(ump->info.name, sizeof(ump->info.name),
    //                   "USB MIDI %d", rmidi->index);
    //     /* copy as rawmidi name if not set */
    //     if (!*ump->core.name)
    //         strscpy(ump->core.name, ump->info.name,
    //                 sizeof(ump->core.name));
    //     /* use serial number string as unique UMP product id */
    //     if (!*ump->info.product_id && dev->serial && *dev->serial)
    //         strscpy(ump->info.product_id, dev->serial);
    // }
}

/* create MIDI interface; fallback to MIDI 1.0 if needed */
pub unsafe extern "C" fn snd_usb_midi_v2_create(
    chip: *mut snd_usb_audio,
    iface: *mut usb_interface,
    quirk: *const snd_usb_audio_quirk,
    usb_id: core::ffi::c_uint,
) -> c_int {
    let umidi: *mut snd_usb_midi2_interface;
    let hostif: *mut usb_host_interface;
    let err: c_int;

    // usb_audio_dbg(chip, "Parsing interface %d...\n",
    //               iface->altsetting[0].desc.bInterfaceNumber);

    /* fallback to MIDI 1.0? */
    if !MIDI2_ENABLE {
        // usb_audio_info(chip, "Falling back to MIDI 1.0 by module option\n");
        goto_fallback_to_midi1(chip, iface, quirk, usb_id);
    }
    // if ((quirk && quirk->type != QUIRK_MIDI_STANDARD_INTERFACE) ||
    //     iface->num_altsetting < 2) {
    //     usb_audio_info(chip, "Quirk or no altset; falling back to MIDI 1.0\n");
    //     goto fallback_to_midi1;
    // }
    hostif = null_mut(); // &iface->altsetting[1];
    if !is_midi2_altset(hostif) {
        // usb_audio_info(chip, "No MIDI 2.0 at altset 1, falling back to MIDI 1.0\n");
        goto_fallback_to_midi1(chip, iface, quirk, usb_id);
    }
    // if (!hostif->desc.bNumEndpoints) {
    //     usb_audio_info(chip, "No endpoint at altset 1, falling back to MIDI 1.0\n");
    //     goto fallback_to_midi1;
    // }

    // usb_audio_dbg(chip, "Creating a MIDI 2.0 instance for %d:%d\n",
    //               hostif->desc.bInterfaceNumber,
    //               hostif->desc.bAlternateSetting);

    umidi = kzalloc(mem::size_of::<snd_usb_midi2_interface>(), 0x10000) as *mut snd_usb_midi2_interface;
    if umidi.is_null() {
        return -12; // -ENOMEM
    }
    (*umidi).chip = chip;
    (*umidi).iface = iface;
    (*umidi).hostif = hostif;
    // INIT_LIST_HEAD(&umidi->rawmidi_list);
    // INIT_LIST_HEAD(&umidi->ep_list);

    // list_add_tail(&umidi->list, &chip->midi_v2_list);

    err = set_altset(umidi);
    if err < 0 {
        // usb_audio_err(chip, "Failed to set altset\n");
        goto_error(umidi, err);
    }

    /* assume only altset 1 corresponding to MIDI 2.0 interface */
    err = parse_midi_2_0(umidi);
    if err < 0 {
        // usb_audio_err(chip, "Failed to parse MIDI 2.0 interface\n");
        goto_error(umidi, err);
    }

    /* parse USB group terminal blocks */
    err = parse_group_terminal_blocks(umidi);
    if err < 0 {
        // usb_audio_err(chip, "Failed to parse GTB\n");
        goto_error(umidi, err);
    }

    err = start_input_streams(umidi);
    if err < 0 {
        // usb_audio_err(chip, "Failed to start input streams\n");
        goto_error(umidi, err);
    }

    if MIDI2_UMP_PROBE {
        err = parse_ump_endpoints(umidi);
        if err < 0 {
            // usb_audio_err(chip, "Failed to parse UMP endpoint\n");
            goto_error(umidi, err);
        }
    }

    err = create_blocks_from_gtb(umidi);
    if err < 0 {
        // usb_audio_err(chip, "Failed to create GTB blocks\n");
        goto_error(umidi, err);
    }

    set_fallback_rawmidi_names(umidi);

    err = attach_legacy_rawmidi(umidi);
    if err < 0 {
        // usb_audio_err(chip, "Failed to create legacy rawmidi\n");
        goto_error(umidi, err);
    }

    return 0;
}

unsafe fn goto_error(umidi: *mut snd_usb_midi2_interface, err: c_int) -> c_int {
    snd_usb_midi_v2_free(umidi);
    return err;
}

unsafe fn goto_fallback_to_midi1(
    chip: *mut snd_usb_audio,
    iface: *mut usb_interface,
    quirk: *const snd_usb_audio_quirk,
    usb_id: core::ffi::c_uint,
) -> c_int {
    return __snd_usbmidi_create(
        null_mut(), // chip->card
        iface,
        null_mut(), // &chip->midi_list
        quirk,
        usb_id,
        null_mut(), // &chip->num_rawmidis
    );
}

unsafe fn suspend_midi2_endpoint(ep: *mut snd_usb_midi2_endpoint) {
    kill_midi_urbs(ep, true);
    drain_urb_queue(ep);
}

pub unsafe extern "C" fn snd_usb_midi_v2_suspend_all(chip: *mut snd_usb_audio) {
    // list_for_each_entry(umidi, &chip->midi_v2_list, list) {
    //     list_for_each_entry(ep, &umidi->ep_list, list)
    //         suspend_midi2_endpoint(ep);
    // }
}

unsafe fn resume_midi2_endpoint(ep: *mut snd_usb_midi2_endpoint) {
    // atomic_set(&ep->running, atomic_read(&ep->suspended));
    // atomic_set(&ep->suspended, 0);

    if (*ep).direction == STR_IN {
        submit_io_urbs(ep);
    }
}

pub unsafe extern "C" fn snd_usb_midi_v2_resume_all(chip: *mut snd_usb_audio) {
    // list_for_each_entry(umidi, &chip->midi_v2_list, list) {
    //     set_altset(umidi);
    //     list_for_each_entry(ep, &umidi->ep_list, list)
    //         resume_midi2_endpoint(ep);
    // }
}

pub unsafe extern "C" fn snd_usb_midi_v2_disconnect_all(chip: *mut snd_usb_audio) {
    // list_for_each_entry(umidi, &chip->midi_v2_list, list) {
    //     umidi->disconnected = 1;
    //     list_for_each_entry(ep, &umidi->ep_list, list) {
    //         ep->disconnected = 1;
    //         kill_midi_urbs(ep, false);
    //         drain_urb_queue(ep);
    //     }
    // }
}

/* release the MIDI instance */
pub unsafe extern "C" fn snd_usb_midi_v2_free_all(chip: *mut snd_usb_audio) {
    // list_for_each_entry_safe(umidi, next, &chip->midi_v2_list, list)
    //     snd_usb_midi_v2_free(umidi);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
