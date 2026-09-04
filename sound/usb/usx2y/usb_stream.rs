// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2007, 2008 Karsten Wiese <fzu@wemgehoertderstaat.de>
 */

// External dependencies from linux/usb.h, linux/gfp.h, usb_stream.h
extern "C" {
    type urb;
    type usb_device;
    type usb_stream;
    type usb_stream_kernel;
    type usb_stream_packet;
    type usb_iso_packet_descriptor;

    fn usb_maxpacket(dev: *mut usb_device, pipe: i32) -> i32;
    fn usb_rcvisocpipe(dev: *mut usb_device, endpoint: u32) -> i32;
    fn usb_sndisocpipe(dev: *mut usb_device, endpoint: u32) -> i32;
    fn usb_pipeout(pipe: i32) -> i32;
    fn usb_urb_ep_type_check(urb: *mut urb) -> i32;
    fn alloc_pages_exact(size: usize, gfp_mask: u32) -> *mut u8;
    fn free_pages_exact(addr: *mut u8, size: usize);
    fn usb_alloc_urb(iso_packets: i32, mem_flags: u32) -> *mut urb;
    fn usb_free_urb(urb: *mut urb);
    fn usb_submit_urb(urb: *mut urb, mem_flags: u32) -> i32;
    fn dev_warn(dev: *const u8, fmt: *const u8, ...);
    fn dev_err(dev: *const u8, fmt: *const u8, ...);
    fn dev_dbg(dev: *const u8, fmt: *const u8, ...);
    fn usb_get_current_frame_number(dev: *mut usb_device) -> i32;
    fn usb_kill_urb(urb: *mut urb);
    fn msleep(msecs: u32);
    fn wake_up_all(wait_queue: *mut u8);
    fn singen_6pack(buf: *mut u8, len: i32);

    // Constant access functions (these may be macros or functions in the actual kernel)
    static USB_STREAM_NURBS: i32;
    static USB_STREAM_URBDEPTH: i32;
    static PAGE_SIZE: usize;
    static GFP_KERNEL: u32;
    static __GFP_ZERO: u32;
    static __GFP_NOWARN: u32;
    static GFP_ATOMIC: u32;
    static USB_STREAM_INTERFACE_VERSION: u32;
    static USB_SPEED_HIGH: i32;
    static USB_SPEED_FULL: i32;
}

// Setup section

unsafe fn usb_stream_next_packet_size(sk: *mut usb_stream_kernel) -> u32 {
    let s = (*sk).s;
    (*sk).out_phase_peeked = ((*sk).out_phase & 0xffff).wrapping_add((*sk).freqn);
    (((*sk).out_phase_peeked >> 16) as u32).wrapping_mul((*s).cfg.frame_size as u32)
}

unsafe fn playback_prep_freqn(sk: *mut usb_stream_kernel, urb: *mut urb) {
    let s = (*sk).s;
    let mut pack: i32 = 0;
    let mut lb: i32 = 0;

    while pack < (*sk).n_o_ps {
        let l: i32 = usb_stream_next_packet_size(sk) as i32;

        if (*s).idle_outsize + lb + l > (*s).period_size {
            break;
        }

        (*sk).out_phase = (*sk).out_phase_peeked;
        let iso_frame_desc = (*urb).iso_frame_desc as *mut usb_iso_packet_descriptor;
        (*iso_frame_desc.add(pack as usize)).offset = lb;
        (*iso_frame_desc.add(pack as usize)).length = l;
        lb += l;
        pack += 1;
    }

    let iso_frame_desc = (*urb).iso_frame_desc as *mut usb_iso_packet_descriptor;
    (*urb).number_of_packets = pack;
    (*urb).transfer_buffer_length = lb;
    (*s).idle_outsize += lb - (*s).period_size;
}

unsafe fn init_pipe_urbs(
    sk: *mut usb_stream_kernel,
    use_packsize: u32,
    urbs: *mut *mut urb,
    transfer: *mut u8,
    dev: *mut usb_device,
    pipe: i32,
) -> i32 {
    let mut u: i32 = 0;
    let maxpacket: i32 = if use_packsize != 0 {
        use_packsize as i32
    } else {
        usb_maxpacket(dev, pipe)
    };
    let transfer_length: i32 = maxpacket * (*sk).n_o_ps;

    let mut current_transfer = transfer;

    while u < USB_STREAM_NURBS {
        let urb = *urbs.add(u as usize);
        let desc: *mut usb_iso_packet_descriptor;

        (*urb).transfer_buffer = current_transfer as *mut u8;
        (*urb).dev = dev;
        (*urb).pipe = pipe;
        (*urb).number_of_packets = (*sk).n_o_ps;
        (*urb).context = sk as *mut u8;
        (*urb).interval = 1;

        if usb_pipeout(pipe) != 0 {
            current_transfer = current_transfer.add(transfer_length as usize);
            u += 1;
            continue;
        }

        if usb_urb_ep_type_check(urb) != 0 {
            return -22; // -EINVAL
        }

        (*urb).transfer_buffer_length = transfer_length;
        desc = (*urb).iso_frame_desc as *mut usb_iso_packet_descriptor;
        (*desc).offset = 0;
        (*desc).length = maxpacket;

        let mut p: i32 = 1;
        while p < (*sk).n_o_ps {
            (*desc.add(p as usize)).offset = (*desc.add((p - 1) as usize)).offset + maxpacket;
            (*desc.add(p as usize)).length = maxpacket;
            p += 1;
        }

        current_transfer = current_transfer.add(transfer_length as usize);
        u += 1;
    }

    0
}

unsafe fn init_urbs(
    sk: *mut usb_stream_kernel,
    use_packsize: u32,
    dev: *mut usb_device,
    in_pipe: i32,
    out_pipe: i32,
) -> i32 {
    let s = (*sk).s;
    let indata: *mut u8 = (s as *mut u8).add(
        core::mem::size_of::<usb_stream>()
            + core::mem::size_of::<usb_stream_packet>() * (*s).inpackets as usize,
    );
    let mut u: i32 = 0;

    while u < USB_STREAM_NURBS {
        let inurb = usb_alloc_urb((*sk).n_o_ps, GFP_KERNEL);
        if inurb.is_null() {
            return -12; // -ENOMEM
        }
        *(*sk).inurb.add(u as usize) = inurb;

        let outurb = usb_alloc_urb((*sk).n_o_ps, GFP_KERNEL);
        if outurb.is_null() {
            return -12; // -ENOMEM
        }
        *(*sk).outurb.add(u as usize) = outurb;

        u += 1;
    }

    if init_pipe_urbs(sk, use_packsize, (*sk).inurb, indata, dev, in_pipe) != 0
        || init_pipe_urbs(
            sk,
            use_packsize,
            (*sk).outurb,
            (*sk).write_page,
            dev,
            out_pipe,
        ) != 0
    {
        return -22; // -EINVAL
    }

    0
}

// Convert a sampling rate into our full speed format (fs/1000 in Q16.16)
// this will overflow at approx 524 kHz
#[inline]
unsafe fn get_usb_full_speed_rate(rate: u32) -> u32 {
    ((rate << 13).wrapping_add(62)) / 125
}

// Convert a sampling rate into USB high speed format (fs/8000 in Q16.16)
// this will overflow at approx 4 MHz
#[inline]
unsafe fn get_usb_high_speed_rate(rate: u32) -> u32 {
    ((rate << 10).wrapping_add(62)) / 125
}

#[no_mangle]
pub unsafe extern "C" fn usb_stream_free(sk: *mut usb_stream_kernel) {
    let mut u: u32 = 0;

    while u < USB_STREAM_NURBS as u32 {
        usb_free_urb(*(*sk).inurb.add(u as usize));
        *(*sk).inurb.add(u as usize) = core::ptr::null_mut();
        usb_free_urb(*(*sk).outurb.add(u as usize));
        *(*sk).outurb.add(u as usize) = core::ptr::null_mut();
        u += 1;
    }

    let s = (*sk).s;
    if s.is_null() {
        return;
    }

    if !(*sk).write_page.is_null() {
        free_pages_exact((*sk).write_page, (*s).write_size as usize);
        (*sk).write_page = core::ptr::null_mut();
    }

    free_pages_exact(s as *mut u8, (*s).read_size as usize);
    (*sk).s = core::ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn usb_stream_new(
    sk: *mut usb_stream_kernel,
    dev: *mut usb_device,
    in_endpoint: u32,
    out_endpoint: u32,
    sample_rate: u32,
    use_packsize: u32,
    period_frames: u32,
    frame_size: u32,
) -> *mut usb_stream {
    let mut packets: i32;
    let mut max_packsize: i32;
    let in_pipe: i32 = usb_rcvisocpipe(dev, in_endpoint);
    let out_pipe: i32 = usb_sndisocpipe(dev, out_endpoint);

    let mut read_size: i32 = core::mem::size_of::<usb_stream>() as i32;
    let write_size: i32;
    let usb_frames: i32 = if (*dev).speed == USB_SPEED_HIGH { 8000 } else { 1000 };

    max_packsize = if use_packsize != 0 {
        use_packsize as i32
    } else {
        usb_maxpacket(dev, in_pipe)
    };

    packets = ((period_frames as i32 * usb_frames) / sample_rate as i32) + 1;

    if (*dev).speed == USB_SPEED_HIGH {
        packets = (packets + 7) & !7;
    }

    read_size += packets
        * USB_STREAM_URBDEPTH
        * (max_packsize + core::mem::size_of::<usb_stream_packet>() as i32);

    max_packsize = usb_maxpacket(dev, out_pipe);
    write_size = max_packsize * packets * USB_STREAM_URBDEPTH;

    if read_size >= 256 * PAGE_SIZE as i32 || write_size >= 256 * PAGE_SIZE as i32 {
        dev_warn(
            &(*dev).dev as *const _ as *const u8,
            b"%s: a size exceeds 128*PAGE_SIZE\n\0".as_ptr(),
        );
        return core::ptr::null_mut();
    }

    let s = alloc_pages_exact(read_size as usize, GFP_KERNEL | __GFP_ZERO | __GFP_NOWARN);
    if s.is_null() {
        dev_warn(
            &(*dev).dev as *const _ as *const u8,
            b"us122l: couldn't allocate read buffer\n\0".as_ptr(),
        );
        return core::ptr::null_mut();
    }

    (*sk).s = s as *mut usb_stream;
    let s = (*sk).s;

    // Access version field through pointer; assuming offset-based field access
    // cfg is at some offset; we'll use it as if the struct is properly defined
    (*s).read_size = read_size;

    (*s).cfg.sample_rate = sample_rate;
    (*s).cfg.frame_size = frame_size;
    (*sk).n_o_ps = packets;
    (*s).inpackets = (packets * USB_STREAM_URBDEPTH) as u32;
    (*s).cfg.period_frames = period_frames;
    (*s).period_size = (frame_size as i32 * period_frames as i32) as i32;

    (*s).write_size = write_size;

    (*sk).write_page = alloc_pages_exact(write_size as usize, GFP_KERNEL | __GFP_ZERO | __GFP_NOWARN);
    if (*sk).write_page.is_null() {
        dev_warn(
            &(*dev).dev as *const _ as *const u8,
            b"us122l: couldn't allocate write buffer\n\0".as_ptr(),
        );
        usb_stream_free(sk);
        return core::ptr::null_mut();
    }

    // calculate the frequency in 16.16 format
    if (*dev).speed == USB_SPEED_FULL {
        (*sk).freqn = get_usb_full_speed_rate(sample_rate);
    } else {
        (*sk).freqn = get_usb_high_speed_rate(sample_rate);
    }

    if init_urbs(sk, use_packsize, dev, in_pipe, out_pipe) < 0 {
        usb_stream_free(sk);
        return core::ptr::null_mut();
    }

    (*s).state = 0; // usb_stream_stopped

    (*sk).s
}

// Start section

unsafe fn balance_check(sk: *mut usb_stream_kernel, urb: *mut urb) -> bool {
    if (*urb).status != 0 {
        if (*urb).status != -108 && (*urb).status != -2 {
            // -ESHUTDOWN (-108), -ENOENT (-2)
            dev_warn(
                &(*(*sk).dev).dev as *const _ as *const u8,
                b"%s: status=%i\n\0".as_ptr(),
                (*urb).status,
            );
        }
        (*sk).iso_frame_balance = 0x7FFFFFFF;
        return false;
    }
    let r = (*sk).iso_frame_balance == 0;
    if !r {
        (*sk).i_urb = urb;
    }
    r
}

unsafe fn balance_playback(sk: *mut usb_stream_kernel, urb: *mut urb) -> bool {
    (*sk).iso_frame_balance += (*urb).number_of_packets;
    balance_check(sk, urb)
}

unsafe fn balance_capture(sk: *mut usb_stream_kernel, urb: *mut urb) -> bool {
    (*sk).iso_frame_balance -= (*urb).number_of_packets;
    balance_check(sk, urb)
}

unsafe fn subs_set_complete(urbs: *mut *mut urb, complete: unsafe extern "C" fn(*mut urb)) {
    let mut u: i32 = 0;

    while u < USB_STREAM_NURBS {
        let urb = *urbs.add(u as usize);
        (*urb).complete = core::mem::transmute(complete);
        u += 1;
    }
}

unsafe fn usb_stream_prepare_playback(sk: *mut usb_stream_kernel, inurb: *mut urb) -> i32 {
    let s = (*sk).s;
    let io = (*sk).idle_outurb;
    let od = (*io).iso_frame_desc as *mut usb_iso_packet_descriptor;

    let mut p: i32 = 0;
    let mut lb: i32 = 0;
    let mut l: i32 = 0;

    while (*s).sync_packet < 0 {
        let ii = (*sk).completed_inurb;
        let id = ((*ii).iso_frame_desc as *mut usb_iso_packet_descriptor)
            .add(((*ii).number_of_packets as i32 + (*s).sync_packet) as usize);
        l = (*id).actual_length;

        (*od.add(p as usize)).length = l;
        (*od.add(p as usize)).offset = lb;
        lb += l;
        p += 1;
        (*s).sync_packet += 1;
    }

    while (*s).sync_packet < (*inurb).number_of_packets && p < (*sk).n_o_ps {
        let id = ((*inurb).iso_frame_desc as *mut usb_iso_packet_descriptor)
            .add((*s).sync_packet as usize);
        l = (*id).actual_length;

        if (*s).idle_outsize + lb + l > (*s).period_size {
            break;
        }

        (*od.add(p as usize)).length = l;
        (*od.add(p as usize)).offset = lb;
        lb += l;
        p += 1;
        (*s).sync_packet += 1;
    }

    (*s).sync_packet -= (*inurb).number_of_packets;

    if (*s).sync_packet < -2 || (*s).sync_packet > 0 {
        dev_warn(
            &(*(*sk).dev).dev as *const _ as *const u8,
            b"%s: invalid sync_packet = %i; p=%i nop=%i %i %x %x %x > %x\n\0".as_ptr(),
            (*s).sync_packet,
            p,
            (*inurb).number_of_packets,
            (*s).idle_outsize + lb + l,
            (*s).idle_outsize,
            lb,
            l,
            (*s).period_size,
        );
        return -1;
    }

    if lb % (*s).cfg.frame_size as i32 != 0 {
        dev_warn(
            &(*(*sk).dev).dev as *const _ as *const u8,
            b"%s: invalid outsize = %i\n\0".as_ptr(),
            lb,
        );
        return -1;
    }

    (*s).idle_outsize += lb - (*s).period_size;
    (*io).number_of_packets = p;
    (*io).transfer_buffer_length = lb;

    if (*s).idle_outsize <= 0 {
        return 0;
    }

    dev_warn(
        &(*(*sk).dev).dev as *const _ as *const u8,
        b"%s: idle=%i\n\0".as_ptr(),
        (*s).idle_outsize,
    );
    -1
}

unsafe fn prepare_inurb(number_of_packets: i32, iu: *mut urb) {
    (*iu).number_of_packets = number_of_packets;
    let id = (*iu).iso_frame_desc as *mut usb_iso_packet_descriptor;
    (*id).offset = 0;

    let mut p: i32 = 0;
    while p < (*iu).number_of_packets - 1 {
        (*id.add((p + 1) as usize)).offset =
            (*id.add(p as usize)).offset + (*id.add(p as usize)).length;
        p += 1;
    }

    (*iu).transfer_buffer_length = (*id).length * (*iu).number_of_packets;
}

unsafe fn submit_urbs(sk: *mut usb_stream_kernel, inurb: *mut urb, outurb: *mut urb) -> i32 {
    prepare_inurb((*(*sk).idle_outurb).number_of_packets, (*sk).idle_inurb);
    let err = usb_submit_urb((*sk).idle_inurb, GFP_ATOMIC);
    if err < 0 {
        dev_err(
            &(*(*sk).dev).dev as *const _ as *const u8,
            b"%s error: %i\n\0".as_ptr(),
            err,
        );
        return err;
    }

    (*sk).idle_inurb = (*sk).completed_inurb;
    (*sk).completed_inurb = inurb;

    let err = usb_submit_urb((*sk).idle_outurb, GFP_ATOMIC);
    if err < 0 {
        dev_err(
            &(*(*sk).dev).dev as *const _ as *const u8,
            b"%s error: %i\n\0".as_ptr(),
            err,
        );
        return err;
    }

    (*sk).idle_outurb = (*sk).completed_outurb;
    (*sk).completed_outurb = outurb;
    0
}

// Conditionally compiled: DEBUG_LOOP_BACK
#[cfg(feature = "debug_loop_back")]
unsafe fn loop_back(s: *mut usb_stream) {
    // This loop_back() shows how to read/write the period data.
    // Note: This requires sk to be accessible; in actual implementation
    // this would need to be refactored or use thread-local storage
    // TODO: loop_back implementation requires access to 'sk' which isn't available
}

#[cfg(not(feature = "debug_loop_back"))]
unsafe fn loop_back(_s: *mut usb_stream) {}

unsafe fn stream_idle(sk: *mut usb_stream_kernel, inurb: *mut urb, outurb: *mut urb) {
    let s = (*sk).s;
    let mut l: i32;
    let mut p: i32;
    let mut insize: i32 = (*s).idle_insize;
    let mut urb_size: i32 = 0;

    (*s).inpacket_split = (*s).next_inpacket_split;
    (*s).inpacket_split_at = (*s).next_inpacket_split_at;
    (*s).next_inpacket_split = -1;
    (*s).next_inpacket_split_at = 0;

    p = 0;
    while p < (*inurb).number_of_packets {
        let id = (*inurb).iso_frame_desc as *mut usb_iso_packet_descriptor;

        l = (*id.add(p as usize)).actual_length;
        if l == 0 || (*id.add(p as usize)).status != 0 {
            dev_warn(
                &(*(*sk).dev).dev as *const _ as *const u8,
                b"%s: underrun, status=%u\n\0".as_ptr(),
                (*id.add(p as usize)).status,
            );
            (*s).state = 4; // usb_stream_xrun
            wake_up_all(&(*sk).sleep as *mut _ as *mut u8);
            return;
        }

        (*s).inpacket_head += 1;
        (*s).inpacket_head %= (*s).inpackets as i32;

        if (*s).inpacket_split == -1 {
            (*s).inpacket_split = (*s).inpacket_head;
        }

        let inpacket = (s as *mut u8).add(
            core::mem::size_of::<usb_stream>()
                + ((*s).inpacket_head as usize) * core::mem::size_of::<usb_stream_packet>(),
        ) as *mut usb_stream_packet;

        (*inpacket).offset =
            (*id.add(p as usize)).offset + ((*inurb).transfer_buffer as i32 - s as i32) as i32;
        (*inpacket).length = l;

        if insize + l > (*s).period_size && (*s).next_inpacket_split == -1 {
            (*s).next_inpacket_split = (*s).inpacket_head;
            (*s).next_inpacket_split_at = (*s).period_size - insize;
        }

        insize += l;
        urb_size += l;
        p += 1;
    }

    (*s).idle_insize += urb_size - (*s).period_size;

    if (*s).idle_insize < 0 {
        dev_warn(
            &(*(*sk).dev).dev as *const _ as *const u8,
            b"%s error: %i\n\0".as_ptr(),
            (*s).idle_insize / (*s).cfg.frame_size as i32,
        );
        (*s).state = 4; // usb_stream_xrun
        wake_up_all(&(*sk).sleep as *mut _ as *mut u8);
        return;
    }

    (*s).insize_done += urb_size;

    l = (*s).idle_outsize;
    let outpacket0 = (s as *mut u8).add(
        core::mem::size_of::<usb_stream>()
            + ((*s).inpackets as usize) * core::mem::size_of::<usb_stream_packet>(),
    ) as *mut usb_stream_packet;

    (*outpacket0).offset =
        ((*(*sk).idle_outurb).transfer_buffer as i32 - (*sk).write_page as i32) - l;

    if usb_stream_prepare_playback(sk, inurb) < 0 {
        (*s).state = 4; // usb_stream_xrun
        wake_up_all(&(*sk).sleep as *mut _ as *mut u8);
        return;
    }

    (*outpacket0).length = (*(*sk).idle_outurb).transfer_buffer_length + l;

    let outpacket1 = (s as *mut u8).add(
        core::mem::size_of::<usb_stream>()
            + ((*s).inpackets as usize) * core::mem::size_of::<usb_stream_packet>()
            + core::mem::size_of::<usb_stream_packet>(),
    ) as *mut usb_stream_packet;

    (*outpacket1).offset = (*(*sk).completed_outurb).transfer_buffer as i32 - (*sk).write_page as i32;

    if submit_urbs(sk, inurb, outurb) < 0 {
        (*s).state = 4; // usb_stream_xrun
        wake_up_all(&(*sk).sleep as *mut _ as *mut u8);
        return;
    }

    loop_back(s);
    (*s).periods_done += 1;
    wake_up_all(&(*sk).sleep as *mut _ as *mut u8);
}

unsafe extern "C" fn i_capture_idle(urb: *mut urb) {
    let sk = (*urb).context as *mut usb_stream_kernel;

    if balance_capture(sk, urb) {
        stream_idle(sk, urb, (*sk).i_urb);
    }
}

unsafe extern "C" fn i_playback_idle(urb: *mut urb) {
    let sk = (*urb).context as *mut usb_stream_kernel;

    if balance_playback(sk, urb) {
        stream_idle(sk, (*sk).i_urb, urb);
    }
}

unsafe fn stream_start(sk: *mut usb_stream_kernel, inurb: *mut urb, outurb: *mut urb) {
    let s = (*sk).s;

    if (*s).state >= 2 {
        // usb_stream_sync1 = 2
        let mut l: i32;
        let mut p: i32;
        let mut max_diff: i32;
        let max_diff_0: i32;
        let mut urb_size: i32 = 0;
        let mut frames_per_packet: u32;
        let mut min_frames: u32 = 0;

        frames_per_packet = ((*s).period_size - (*s).idle_insize) as u32;
        frames_per_packet <<= 8;
        frames_per_packet /=
            ((*s).cfg.frame_size as u32).wrapping_mul((*inurb).number_of_packets as u32);
        frames_per_packet += 1;

        max_diff_0 = (*s).cfg.frame_size as i32;
        max_diff = max_diff_0;

        if (*s).cfg.period_frames >= 256 {
            max_diff <<= 1;
        }
        if (*s).cfg.period_frames >= 1024 {
            max_diff <<= 1;
        }

        p = 0;
        while p < (*inurb).number_of_packets {
            let id = ((*inurb).iso_frame_desc as *mut usb_iso_packet_descriptor)
                .add(p as usize);
            l = (*id).actual_length;
            urb_size += l;

            min_frames += frames_per_packet;
            let diff = urb_size - ((min_frames >> 8) as i32) * (*s).cfg.frame_size as i32;
            if diff < max_diff {
                max_diff = diff;
            }
            p += 1;
        }

        (*s).idle_insize -= max_diff - max_diff_0;
        (*s).idle_insize += urb_size - (*s).period_size;

        if (*s).idle_insize < 0 {
            dev_warn(
                &(*(*sk).dev).dev as *const _ as *const u8,
                b"%s idle_insize: %i %i %i\n\0".as_ptr(),
                (*s).idle_insize,
                urb_size,
                (*s).period_size,
            );
            return;
        } else if (*s).idle_insize == 0 {
            (*s).next_inpacket_split = ((*s).inpacket_head + 1) % (*s).inpackets as i32;
            (*s).next_inpacket_split_at = 0;
        } else {
            let mut split: i32 = (*s).inpacket_head;
            l = (*s).idle_insize;

            loop {
                let inpacket = (s as *mut u8).add(
                    core::mem::size_of::<usb_stream>()
                        + (split as usize) * core::mem::size_of::<usb_stream_packet>(),
                ) as *mut usb_stream_packet;

                if l <= (*inpacket).length {
                    break;
                }

                l -= (*inpacket).length;

                if split == 0 {
                    split = (*s).inpackets as i32 - 1;
                } else {
                    split -= 1;
                }
            }

            (*s).next_inpacket_split = split;

            let inpacket = (s as *mut u8).add(
                core::mem::size_of::<usb_stream>()
                    + (split as usize) * core::mem::size_of::<usb_stream_packet>(),
            ) as *mut usb_stream_packet;

            (*s).next_inpacket_split_at = (*inpacket).length - l;
        }

        (*s).insize_done += urb_size;

        if usb_stream_prepare_playback(sk, inurb) < 0 {
            return;
        }
    } else {
        playback_prep_freqn(sk, (*sk).idle_outurb);
    }

    if submit_urbs(sk, inurb, outurb) < 0 {
        return;
    }

    if (*s).state == 1 && (*s).insize_done > 360000 {
        // usb_stream_sync1 = 1
        (*s).state = 3; // usb_stream_ready = 3
        subs_set_complete((*sk).inurb, i_capture_idle);
        subs_set_complete((*sk).outurb, i_playback_idle);
    }
}

unsafe extern "C" fn i_capture_start(urb: *mut urb) {
    let id = (*urb).iso_frame_desc as *mut usb_iso_packet_descriptor;
    let sk = (*urb).context as *mut usb_stream_kernel;
    let s = (*sk).s;
    let mut p: i32;
    let mut empty: i32 = 0;

    if (*urb).status != 0 {
        dev_warn(
            &(*(*sk).dev).dev as *const _ as *const u8,
            b"%s: status=%i\n\0".as_ptr(),
            (*urb).status,
        );
        return;
    }

    p = 0;
    while p < (*urb).number_of_packets {
        let l = (*id.add(p as usize)).actual_length;

        if l < (*s).cfg.frame_size as i32 {
            empty += 1;
            if (*s).state >= 1 {
                // usb_stream_sync0 = 1
                dev_warn(
                    &(*(*sk).dev).dev as *const _ as *const u8,
                    b"%s: length %i\n\0".as_ptr(),
                    l,
                );
                return;
            }
        }

        (*s).inpacket_head += 1;
        (*s).inpacket_head %= (*s).inpackets as i32;

        let inpacket = (s as *mut u8).add(
            core::mem::size_of::<usb_stream>()
                + ((*s).inpacket_head as usize) * core::mem::size_of::<usb_stream_packet>(),
        ) as *mut usb_stream_packet;

        (*inpacket).offset =
            (*id.add(p as usize)).offset + ((*urb).transfer_buffer as i32 - s as i32) as i32;
        (*inpacket).length = l;

        p += 1;
    }

    if empty == 0 && (*s).state < 2 {
        // usb_stream_sync1 = 2
        (*s).state += 1;
    }

    if balance_capture(sk, urb) {
        stream_start(sk, urb, (*sk).i_urb);
    }
}

unsafe extern "C" fn i_playback_start(urb: *mut urb) {
    let sk = (*urb).context as *mut usb_stream_kernel;

    if balance_playback(sk, urb) {
        stream_start(sk, (*sk).i_urb, urb);
    }
}

#[no_mangle]
pub unsafe extern "C" fn usb_stream_start(sk: *mut usb_stream_kernel) -> i32 {
    let s = (*sk).s;

    if (*s).state != 0 {
        // usb_stream_stopped = 0
        return -11; // -EAGAIN
    }

    subs_set_complete((*sk).inurb, i_capture_start);
    subs_set_complete((*sk).outurb, i_playback_start);
    core::ptr::write_bytes((*sk).write_page, 0, (*s).write_size as usize);

    let mut frame: i32 = 0;
    let mut u: i32;
    let mut err: i32;
    let mut try_count: i32 = 0;

    loop {
        (*s).insize_done = 0;
        (*s).idle_insize = 0;
        (*s).idle_outsize = 0;
        (*s).sync_packet = -1;
        (*s).inpacket_head = -1;
        (*sk).iso_frame_balance = 0;
        try_count += 1;

        u = 0;
        loop {
            if u >= 2 {
                break;
            }

            let inurb = *(*sk).inurb.add(u as usize);
            let outurb = *(*sk).outurb.add(u as usize);

            playback_prep_freqn(sk, outurb);
            (*inurb).number_of_packets = (*outurb).number_of_packets;
            (*inurb).transfer_buffer_length =
                (*inurb).number_of_packets * (*inurb).iso_frame_desc[0].length;

            if u == 0 {
                let dev = (*inurb).dev;

                frame = usb_get_current_frame_number(dev);
                loop {
                    let now = usb_get_current_frame_number(dev);
                    if !(now > -1 && now == frame) {
                        break;
                    }
                }
            }

            err = usb_submit_urb(inurb, GFP_ATOMIC);
            if err < 0 {
                dev_err(
                    &(*(*sk).dev).dev as *const _ as *const u8,
                    b"%s: usb_submit_urb(sk->inurb[%i]) returned %i\n\0".as_ptr(),
                    u,
                    err,
                );
                return err;
            }

            err = usb_submit_urb(outurb, GFP_ATOMIC);
            if err < 0 {
                dev_err(
                    &(*(*sk).dev).dev as *const _ as *const u8,
                    b"%s: usb_submit_urb(sk->outurb[%i]) returned %i\n\0".as_ptr(),
                    u,
                    err,
                );
                return err;
            }

            if (*inurb).start_frame != (*outurb).start_frame {
                dev_dbg(
                    &(*(*sk).dev).dev as *const _ as *const u8,
                    b"%s: u[%i] start_frames differ in:%u out:%u\n\0".as_ptr(),
                    u,
                    (*inurb).start_frame,
                    (*outurb).start_frame,
                );
                break;
            }

            u += 1;
        }

        if u < 2 {
            try_count = 0;
            break;
        }

        if try_count < 5 {
            usb_stream_stop(sk);
            msleep(1500);
            dev_dbg(
                &(*(*sk).dev).dev as *const _ as *const u8,
                b"goto dotry;\n\0".as_ptr(),
            );
            continue;
        }

        usb_stream_stop(sk);
        dev_warn(
            &(*(*sk).dev).dev as *const _ as *const u8,
            b"%s: couldn't start all urbs on the same start_frame.\n\0".as_ptr(),
        );
        return -14; // -EFAULT
    }

    (*sk).idle_inurb = *(*sk).inurb.add((USB_STREAM_NURBS - 2) as usize);
    (*sk).idle_outurb = *(*sk).outurb.add((USB_STREAM_NURBS - 2) as usize);
    (*sk).completed_inurb = *(*sk).inurb.add((USB_STREAM_NURBS - 1) as usize);
    (*sk).completed_outurb = *(*sk).outurb.add((USB_STREAM_NURBS - 1) as usize);

    // wait, check
    let mut wait_ms: i32 = 3000;
    while (*s).state != 3 && wait_ms > 0 {
        // usb_stream_ready = 3
        msleep(200);
        wait_ms -= 200;
    }

    if (*s).state == 3 {
        0
    } else {
        -14 // -EFAULT
    }
}

// Stop section

#[no_mangle]
pub unsafe extern "C" fn usb_stream_stop(sk: *mut usb_stream_kernel) {
    if (*sk).s.is_null() {
        return;
    }

    let mut u: i32 = 0;
    while u < USB_STREAM_NURBS {
        usb_kill_urb(*(*sk).inurb.add(u as usize));
        usb_kill_urb(*(*sk).outurb.add(u as usize));
        u += 1;
    }

    (*(*sk).s).state = 0; // usb_stream_stopped
    msleep(400);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
