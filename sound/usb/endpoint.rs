// SPDX-License-Identifier: GPL-2.0-or-later

use core::ptr;

#[repr(u32)]
enum EpState {
    Stopped = 0,
    Running = 1,
    Stopping = 2,
}

#[repr(C)]
struct SndUsbIfaceRef {
    iface: u8,
    need_setup: bool,
    opened: i32,
    altset: i32,
    list: ListHead,
}

#[repr(C)]
struct SndUsbClockRef {
    clock: u8,
    locked: AtomicT,
    opened: i32,
    rate: i32,
    need_setup: bool,
    list: ListHead,
}

#[inline]
fn get_usb_full_speed_rate(rate: u32) -> u32 {
    ((rate << 13) + 62) / 125
}

#[inline]
fn get_usb_high_speed_rate(rate: u32) -> u32 {
    ((rate << 10) + 62) / 125
}

unsafe fn release_urb_ctx(u: *mut SndUrbCtx) {
    if !(*u).urb.is_null() && (*u).buffer_size != 0 {
        usb_free_coherent(
            (*(*u).ep).chip as *mut _,
            (*u).buffer_size,
            (*(*u).urb).transfer_buffer,
            (*(*u).urb).transfer_dma,
        );
    }
    usb_free_urb((*u).urb);
    (*u).urb = ptr::null_mut();
    (*u).buffer_size = 0;
}

fn usb_error_string(err: i32) -> &'static str {
    match err {
        -libc::ENODEV => "no device",
        -libc::ENOENT => "endpoint not enabled",
        -libc::EPIPE => "endpoint stalled",
        -libc::ENOSPC => "not enough bandwidth",
        -libc::ESHUTDOWN => "device disabled",
        -libc::EHOSTUNREACH => "device suspended",
        -libc::EINVAL | -libc::EAGAIN | -libc::EFBIG | -libc::EMSGSIZE => "internal error",
        _ => "unknown error",
    }
}

#[inline]
fn ep_state_running(ep: *const SndUsbEndpoint) -> bool {
    unsafe { atomic_read(&(*ep).state) == EP_STATE_RUNNING as i32 }
}

#[inline]
fn ep_state_update(ep: *mut SndUsbEndpoint, old: i32, new: i32) -> bool {
    unsafe { atomic_try_cmpxchg(&mut (*ep).state, &mut (old as i32), new) }
}

pub extern "C" fn snd_usb_endpoint_implicit_feedback_sink(ep: *const SndUsbEndpoint) -> i32 {
    unsafe {
        if (*ep).implicit_fb_sync != 0 && usb_pipeout((*ep).pipe) != 0 {
            return 1;
        }
        0
    }
}

fn synced_next_packet_size(ep: *mut SndUsbEndpoint, avail: u32) -> i32 {
    unsafe {
        let mut phase: u32;
        let mut ret: i32;

        if (*ep).fill_max != 0 {
            return (*ep).maxframesize as i32;
        }

        // guard(spinlock_irqsave)(&ep->lock)
        spin_lock_irqsave(&(*ep).lock);
        phase = ((*ep).phase & 0xffff) + ((*ep).freqm << (*ep).datainterval);
        ret = (phase >> 16) as i32;
        if ret > (*ep).maxframesize as i32 {
            ret = (*ep).maxframesize as i32;
        }
        if avail != 0 && ret as u32 >= avail {
            ret = -libc::EAGAIN as i32;
        } else {
            (*ep).phase = phase;
        }
        spin_unlock_irqrestore(&(*ep).lock);

        ret
    }
}

fn next_packet_size(ep: *mut SndUsbEndpoint, avail: u32) -> i32 {
    unsafe {
        let mut sample_accum: u32;
        let mut ret: i32;

        if (*ep).fill_max != 0 {
            return (*ep).maxframesize as i32;
        }

        sample_accum = (*ep).sample_accum + (*ep).sample_rem;
        if sample_accum >= (*ep).pps {
            sample_accum -= (*ep).pps;
            ret = (*ep).packsize[1] as i32;
        } else {
            ret = (*ep).packsize[0] as i32;
        }
        if avail != 0 && ret as u32 >= avail {
            ret = -libc::EAGAIN as i32;
        } else {
            (*ep).sample_accum = sample_accum;
        }

        ret
    }
}

pub extern "C" fn snd_usb_endpoint_next_packet_size(
    ep: *mut SndUsbEndpoint,
    ctx: *mut SndUrbCtx,
    idx: i32,
    avail: u32,
) -> i32 {
    unsafe {
        let mut packet: u32 = (*ctx).packet_size[idx as usize];
        if packet != 0 {
            if packet > (*ep).maxframesize as u32 {
                packet = (*ep).maxframesize as u32;
            }
            if avail != 0 && packet >= avail {
                return -libc::EAGAIN as i32;
            }
            return packet as i32;
        }

        if !(*ep).sync_source.is_null() {
            synced_next_packet_size(ep, avail)
        } else {
            next_packet_size(ep, avail)
        }
    }
}

unsafe fn call_retire_callback(ep: *mut SndUsbEndpoint, urb: *mut Urb) {
    let data_subs = read_once((*ep).data_subs as *const _);
    if !data_subs.is_null() && !(*ep).retire_data_urb.is_none() {
        if let Some(retire) = (*ep).retire_data_urb {
            retire(data_subs as *mut _, urb);
        }
    }
}

unsafe fn retire_outbound_urb(ep: *mut SndUsbEndpoint, urb_ctx: *mut SndUrbCtx) {
    call_retire_callback(ep, (*urb_ctx).urb);
}

extern "C" {
    fn snd_usb_handle_sync_urb(
        ep: *mut SndUsbEndpoint,
        sender: *mut SndUsbEndpoint,
        urb: *const Urb,
    );
}

unsafe fn retire_inbound_urb(ep: *mut SndUsbEndpoint, urb_ctx: *mut SndUrbCtx) {
    let urb = (*urb_ctx).urb;

    if (*ep).skip_packets > 0 {
        (*ep).skip_packets -= 1;
        return;
    }

    let sync_sink = read_once((*ep).sync_sink as *const _);
    if !sync_sink.is_null() {
        snd_usb_handle_sync_urb(sync_sink as *mut _, ep, urb);
    }

    call_retire_callback(ep, urb);
}

#[inline]
fn has_tx_length_quirk(chip: *const SndUsbAudio) -> bool {
    unsafe { ((*chip).quirk_flags & QUIRK_FLAG_TX_LENGTH) != 0 }
}

unsafe fn prepare_silent_urb(ep: *mut SndUsbEndpoint, ctx: *mut SndUrbCtx) -> i32 {
    let urb = (*ctx).urb;
    let mut offs: u32 = 0;
    let mut extra: u32 = 0;
    let packet_length: u32;
    let mut i: i32;

    if has_tx_length_quirk((*ep).chip) {
        extra = core::mem::size_of::<u32>() as u32;
    }

    i = 0;
    while i < (*ctx).packets {
        let mut length: i32 = snd_usb_endpoint_next_packet_size(ep, ctx, i, 0);
        if length < 0 {
            return length;
        }
        length *= (*ep).stride as i32;
        if offs + length as u32 + extra > (*ctx).buffer_size {
            break;
        }
        (*urb).iso_frame_desc[i as usize].offset = offs as u32;
        (*urb).iso_frame_desc[i as usize].length = length as u32 + extra;
        if extra != 0 {
            packet_length = cpu_to_le32(length as u32);
            memcpy(
                ((*urb).transfer_buffer as *mut u8).add(offs as usize) as *mut _,
                &packet_length as *const _ as *const _,
                core::mem::size_of::<u32>(),
            );
            offs += extra;
        }
        memset(
            ((*urb).transfer_buffer as *mut u8).add(offs as usize) as *mut _,
            (*ep).silence_value as i32,
            length as usize,
        );
        offs += length as u32;
        i += 1;
    }

    if offs == 0 {
        return -libc::EPIPE as i32;
    }

    (*urb).number_of_packets = i as u32;
    (*urb).transfer_buffer_length = offs;
    (*ctx).queued = 0;
    0
}

unsafe fn prepare_outbound_urb(
    ep: *mut SndUsbEndpoint,
    ctx: *mut SndUrbCtx,
    in_stream_lock: bool,
) -> i32 {
    let urb = (*ctx).urb;
    let cp = (*urb).transfer_buffer as *mut u8;

    (*urb).dev = (*(*ep).chip).dev;

    match (*ep).type_ {
        SND_USB_ENDPOINT_TYPE_DATA => {
            let data_subs = read_once((*ep).data_subs as *const _);
            if !data_subs.is_null() && !(*ep).prepare_data_urb.is_none() {
                if let Some(prepare) = (*ep).prepare_data_urb {
                    return prepare(data_subs as *mut _, urb, in_stream_lock);
                }
            }
            prepare_silent_urb(ep, ctx)
        }
        SND_USB_ENDPOINT_TYPE_SYNC => {
            if snd_usb_get_speed((*(*ep).chip).dev) >= USB_SPEED_HIGH {
                (*urb).iso_frame_desc[0].length = 4;
                (*urb).iso_frame_desc[0].offset = 0;
                *cp.offset(0) = (*ep).freqn as u8;
                *cp.offset(1) = ((*ep).freqn >> 8) as u8;
                *cp.offset(2) = ((*ep).freqn >> 16) as u8;
                *cp.offset(3) = ((*ep).freqn >> 24) as u8;
            } else {
                (*urb).iso_frame_desc[0].length = 3;
                (*urb).iso_frame_desc[0].offset = 0;
                *cp.offset(0) = ((*ep).freqn >> 2) as u8;
                *cp.offset(1) = ((*ep).freqn >> 10) as u8;
                *cp.offset(2) = ((*ep).freqn >> 18) as u8;
            }
            0
        }
        _ => 0,
    }
}

unsafe fn prepare_inbound_urb(ep: *mut SndUsbEndpoint, urb_ctx: *mut SndUrbCtx) -> i32 {
    let mut i: i32;
    let mut offs: u32 = 0;
    let urb = (*urb_ctx).urb;

    (*urb).dev = (*(*ep).chip).dev;

    match (*ep).type_ {
        SND_USB_ENDPOINT_TYPE_DATA => {
            i = 0;
            while i < (*urb_ctx).packets {
                if offs + (*ep).curpacksize > (*urb_ctx).buffer_size {
                    break;
                }
                (*urb).iso_frame_desc[i as usize].offset = offs;
                (*urb).iso_frame_desc[i as usize].length = (*ep).curpacksize;
                offs += (*ep).curpacksize;
                i += 1;
            }

            (*urb).transfer_buffer_length = offs;
            (*urb).number_of_packets = i as u32;
        }
        SND_USB_ENDPOINT_TYPE_SYNC => {
            let max_len = if 4 < (*ep).syncmaxsize { 4 } else { (*ep).syncmaxsize };
            (*urb).iso_frame_desc[0].length = max_len as u32;
            (*urb).iso_frame_desc[0].offset = 0;
        }
        _ => {}
    }
    0
}

unsafe fn notify_xrun(ep: *mut SndUsbEndpoint) -> bool {
    let data_subs = read_once((*ep).data_subs as *const _);
    if data_subs.is_null() {
        return false;
    }
    let psubs = (*data_subs).pcm_substream;
    if !psubs.is_null() && !(*psubs).runtime.is_null() {
        if (*(*psubs).runtime).state == SNDRV_PCM_STATE_RUNNING {
            snd_pcm_stop_xrun(psubs);
            return true;
        }
    }
    false
}

unsafe fn next_packet_fifo_enqueue(ep: *mut SndUsbEndpoint) -> *mut SndUsbPacketInfo {
    let idx = ((*ep).next_packet_head + (*ep).next_packet_queued) % ARRAY_SIZE_NEXT_PACKET;
    let p = (*ep).next_packet.offset(idx as isize);
    (*ep).next_packet_queued += 1;
    p
}

unsafe fn next_packet_fifo_dequeue(ep: *mut SndUsbEndpoint) -> *mut SndUsbPacketInfo {
    let p = (*ep).next_packet.offset((*ep).next_packet_head as isize);
    (*ep).next_packet_head += 1;
    (*ep).next_packet_head %= ARRAY_SIZE_NEXT_PACKET;
    (*ep).next_packet_queued -= 1;
    p
}

unsafe fn push_back_to_ready_list(ep: *mut SndUsbEndpoint, ctx: *mut SndUrbCtx) {
    spin_lock_irqsave(&(*ep).lock);
    list_add_tail(&mut (*ctx).ready_list, &mut (*ep).ready_playback_urbs);
    spin_unlock_irqrestore(&(*ep).lock);
}

pub extern "C" fn snd_usb_queue_pending_output_urbs(
    ep: *mut SndUsbEndpoint,
    in_stream_lock: bool,
) -> i32 {
    unsafe {
        let implicit_fb = snd_usb_endpoint_implicit_feedback_sink(ep) != 0;

        while ep_state_running(ep) {
            let mut packet: *mut SndUsbPacketInfo = ptr::null_mut();
            let mut ctx: *mut SndUrbCtx = ptr::null_mut();
            let mut err: i32;

            spin_lock_irqsave(&(*ep).lock);
            if (!implicit_fb || (*ep).next_packet_queued > 0)
                && !list_empty(&(*ep).ready_playback_urbs)
            {
                ctx = list_first_entry(&(*ep).ready_playback_urbs, SndUrbCtx, ready_list);
                list_del_init(&(*ctx).ready_list);
                if implicit_fb {
                    packet = next_packet_fifo_dequeue(ep);
                }
            }
            spin_unlock_irqrestore(&(*ep).lock);

            if ctx.is_null() {
                break;
            }

            if implicit_fb {
                (*ctx).packets = (*packet).packets;
                memcpy(
                    (*ctx).packet_size.as_mut_ptr() as *mut _,
                    (*packet).packet_size.as_ptr() as *const _,
                    ((*packet).packets as usize) * core::mem::size_of::<u32>(),
                );
            }

            err = prepare_outbound_urb(ep, ctx, in_stream_lock);
            if !ep_state_running(ep) {
                break;
            }
            if err < 0 {
                if err == -libc::EAGAIN as i32 {
                    push_back_to_ready_list(ep, ctx);
                    break;
                }

                if !in_stream_lock {
                    notify_xrun(ep);
                }
                return -libc::EPIPE as i32;
            }

            if atomic_read(&(*(*ep).chip).shutdown) == 0 {
                err = usb_submit_urb((*ctx).urb, GFP_ATOMIC);
            } else {
                err = -libc::ENODEV as i32;
            }
            if err < 0 {
                if atomic_read(&(*(*ep).chip).shutdown) == 0 {
                    usb_audio_err(
                        (*ep).chip,
                        "Unable to submit urb #%d: %d at %s\n\0".as_ptr() as *const _,
                        (*ctx).index,
                        err,
                        "snd_usb_queue_pending_output_urbs\0".as_ptr(),
                    );
                    if !in_stream_lock {
                        notify_xrun(ep);
                    }
                }
                return -libc::EPIPE as i32;
            }

            set_bit((*ctx).index, &mut (*ep).active_mask);
            atomic_inc(&mut (*ep).submitted_urbs);
        }

        0
    }
}

extern "C" fn snd_complete_urb(urb: *mut Urb) {
    unsafe {
        let ctx = (*urb).context as *mut SndUrbCtx;
        let ep = (*ctx).ep;
        let mut err: i32;

        if ((*urb).status == -libc::ENOENT as i32)
            || ((*urb).status == -libc::ENODEV as i32)
            || ((*urb).status == -libc::ECONNRESET as i32)
            || ((*urb).status == -libc::ESHUTDOWN as i32)
        {
            goto_exit_clear(ep, ctx);
            return;
        }
        if atomic_read(&(*(*ep).chip).shutdown) != 0 {
            goto_exit_clear(ep, ctx);
            return;
        }

        if !ep_state_running(ep) {
            goto_exit_clear(ep, ctx);
            return;
        }

        if usb_pipeout((*ep).pipe) != 0 {
            retire_outbound_urb(ep, ctx);
            if !ep_state_running(ep) {
                goto_exit_clear(ep, ctx);
                return;
            }

            if (*ep).lowlatency_playback != 0 || snd_usb_endpoint_implicit_feedback_sink(ep) != 0
            {
                push_back_to_ready_list(ep, ctx);
                clear_bit((*ctx).index, &mut (*ep).active_mask);
                snd_usb_queue_pending_output_urbs(ep, false);
                if atomic_dec_and_test(&mut (*ep).submitted_urbs) {
                    if snd_usb_endpoint_implicit_feedback_sink(ep) == 0 {
                        notify_xrun(ep);
                    }
                }
                return;
            }

            prepare_outbound_urb(ep, ctx, false);
            if !ep_state_running(ep) {
                goto_exit_clear(ep, ctx);
                return;
            }
        } else {
            retire_inbound_urb(ep, ctx);
            if !ep_state_running(ep) {
                goto_exit_clear(ep, ctx);
                return;
            }

            prepare_inbound_urb(ep, ctx);
        }

        if atomic_read(&(*(*ep).chip).shutdown) == 0 {
            err = usb_submit_urb(urb, GFP_ATOMIC);
        } else {
            err = -libc::ENODEV as i32;
        }
        if err == 0 {
            return;
        }

        if atomic_read(&(*(*ep).chip).shutdown) == 0 {
            if notify_xrun(ep) {
                usb_audio_err(
                    (*ep).chip,
                    "cannot submit urb (err = %d)\n\0".as_ptr() as *const _,
                    err,
                );
            }
        }

        goto_exit_clear(ep, ctx);
    }
}

unsafe fn goto_exit_clear(ep: *mut SndUsbEndpoint, ctx: *mut SndUrbCtx) {
    clear_bit((*ctx).index, &mut (*ep).active_mask);
    atomic_dec(&mut (*ep).submitted_urbs);
}

unsafe fn iface_ref_find(chip: *mut SndUsbAudio, iface: i32) -> *mut SndUsbIfaceRef {
    let mut ip: *mut SndUsbIfaceRef;

    let mut pos = (*chip).iface_ref_list;
    loop {
        if pos.is_null() {
            break;
        }
        ip = container_of(pos, SndUsbIfaceRef, list) as *mut _;
        if (*ip).iface as i32 == iface {
            return ip;
        }
        pos = (*pos).next;
    }

    ip = kzalloc(core::mem::size_of::<SndUsbIfaceRef>()) as *mut _;
    if ip.is_null() {
        return ptr::null_mut();
    }
    (*ip).iface = iface as u8;
    list_add_tail(&mut (*ip).list, &mut (*chip).iface_ref_list);
    ip
}

unsafe fn clock_ref_find(chip: *mut SndUsbAudio, clock: i32) -> *mut SndUsbClockRef {
    let mut ref_: *mut SndUsbClockRef;

    let mut pos = (*chip).clock_ref_list;
    loop {
        if pos.is_null() {
            break;
        }
        ref_ = container_of(pos, SndUsbClockRef, list) as *mut _;
        if (*ref_).clock as i32 == clock {
            return ref_;
        }
        pos = (*pos).next;
    }

    ref_ = kzalloc(core::mem::size_of::<SndUsbClockRef>()) as *mut _;
    if ref_.is_null() {
        return ptr::null_mut();
    }
    (*ref_).clock = clock as u8;
    atomic_set(&mut (*ref_).locked, 0);
    list_add_tail(&mut (*ref_).list, &mut (*chip).clock_ref_list);
    ref_
}

pub extern "C" fn snd_usb_get_endpoint(
    chip: *mut SndUsbAudio,
    ep_num: i32,
) -> *mut SndUsbEndpoint {
    unsafe {
        let mut ep: *mut SndUsbEndpoint;

        let mut pos = (*chip).ep_list;
        loop {
            if pos.is_null() {
                break;
            }
            ep = container_of(pos, SndUsbEndpoint, list) as *mut _;
            if (*ep).ep_num == ep_num {
                return ep;
            }
            pos = (*pos).next;
        }

        ptr::null_mut()
    }
}

fn ep_type_name(ep_type: i32) -> &'static str {
    if ep_type == SND_USB_ENDPOINT_TYPE_DATA {
        "data"
    } else {
        "sync"
    }
}

pub extern "C" fn snd_usb_add_endpoint(chip: *mut SndUsbAudio, ep_num: i32, ep_type: i32) -> i32 {
    unsafe {
        let ep = snd_usb_get_endpoint(chip, ep_num);
        if !ep.is_null() {
            return 0;
        }

        usb_audio_dbg(
            chip,
            "Creating new %s endpoint #%x\n\0".as_ptr() as *const _,
            ep_type_name(ep_type).as_ptr(),
            ep_num,
        );
        let ep = kzalloc(core::mem::size_of::<SndUsbEndpoint>()) as *mut SndUsbEndpoint;
        if ep.is_null() {
            return -libc::ENOMEM as i32;
        }

        (*ep).chip = chip;
        spin_lock_init(&mut (*ep).lock);
        (*ep).type_ = ep_type;
        (*ep).ep_num = ep_num;
        init_list_head(&mut (*ep).ready_playback_urbs);
        atomic_set(&mut (*ep).submitted_urbs, 0);

        let is_playback = ((ep_num & USB_ENDPOINT_DIR_MASK) == USB_DIR_OUT);
        let ep_num_masked = ep_num & USB_ENDPOINT_NUMBER_MASK;
        if is_playback {
            (*ep).pipe = usb_sndisocpipe((*chip).dev, ep_num_masked);
        } else {
            (*ep).pipe = usb_rcvisocpipe((*chip).dev, ep_num_masked);
        }

        list_add_tail(&mut (*ep).list, &mut (*chip).ep_list);
        0
    }
}

unsafe fn endpoint_set_syncinterval(chip: *mut SndUsbAudio, ep: *mut SndUsbEndpoint) {
    let alts = snd_usb_get_host_interface(chip, (*ep).iface, (*ep).altsetting);
    if alts.is_null() {
        return;
    }

    let desc = get_endpoint(alts, (*ep).ep_idx);
    if !desc.is_null() {
        if (*desc).bLength >= USB_DT_ENDPOINT_AUDIO_SIZE as u8
            && (*desc).bRefresh >= 1
            && (*desc).bRefresh <= 9
        {
            (*ep).syncinterval = (*desc).bRefresh as i32;
        } else if snd_usb_get_speed((*chip).dev) == USB_SPEED_FULL {
            (*ep).syncinterval = 1;
        } else if (*desc).bInterval >= 1 && (*desc).bInterval <= 16 {
            (*ep).syncinterval = ((*desc).bInterval - 1) as i32;
        } else {
            (*ep).syncinterval = 3;
        }

        (*ep).syncmaxsize = le16_to_cpu((*desc).wMaxPacketSize) as u32;
    }
}

unsafe fn endpoint_compatible(
    ep: *const SndUsbEndpoint,
    fp: *const AudioFormat,
    params: *const SndPcmHwParams,
) -> bool {
    if (*ep).opened == 0 {
        return false;
    }
    if (*ep).cur_audiofmt as *const _ != fp {
        return false;
    }
    if (*ep).cur_rate != params_rate(params)
        || (*ep).cur_format != params_format(params)
        || (*ep).cur_period_frames != params_period_size(params)
        || (*ep).cur_buffer_periods != params_periods(params)
    {
        return false;
    }
    true
}

pub extern "C" fn snd_usb_endpoint_compatible(
    chip: *mut SndUsbAudio,
    ep: *const SndUsbEndpoint,
    fp: *const AudioFormat,
    params: *const SndPcmHwParams,
) -> bool {
    unsafe {
        mutex_lock(&mut (*chip).mutex);
        let result = endpoint_compatible(ep, fp, params);
        mutex_unlock(&mut (*chip).mutex);
        result
    }
}

pub extern "C" fn snd_usb_endpoint_open(
    chip: *mut SndUsbAudio,
    fp: *const AudioFormat,
    params: *const SndPcmHwParams,
    is_sync_ep: bool,
    fixed_rate: bool,
) -> *mut SndUsbEndpoint {
    unsafe {
        let ep_num = if is_sync_ep {
            (*fp).sync_ep
        } else {
            (*fp).endpoint
        };

        mutex_lock(&mut (*chip).mutex);
        let mut ep = snd_usb_get_endpoint(chip, ep_num);
        if ep.is_null() {
            usb_audio_err(
                chip,
                "Cannot find EP 0x%x to open\n\0".as_ptr() as *const _,
                ep_num,
            );
            mutex_unlock(&mut (*chip).mutex);
            return ptr::null_mut();
        }

        if (*ep).opened == 0 {
            if is_sync_ep {
                (*ep).iface = (*fp).sync_iface;
                (*ep).altsetting = (*fp).sync_altsetting;
                (*ep).ep_idx = (*fp).sync_ep_idx;
            } else {
                (*ep).iface = (*fp).iface;
                (*ep).altsetting = (*fp).altsetting;
                (*ep).ep_idx = (*fp).ep_idx;
            }
            usb_audio_dbg(
                chip,
                "Open EP 0x%x, iface=%d:%d, idx=%d\n\0".as_ptr() as *const _,
                ep_num,
                (*ep).iface,
                (*ep).altsetting,
                (*ep).ep_idx,
            );

            (*ep).iface_ref = iface_ref_find(chip, (*ep).iface as i32);
            if (*ep).iface_ref.is_null() {
                mutex_unlock(&mut (*chip).mutex);
                return ptr::null_mut();
            }

            if (*fp).protocol != UAC_VERSION_1 {
                (*ep).clock_ref = clock_ref_find(chip, (*fp).clock);
                if (*ep).clock_ref.is_null() {
                    mutex_unlock(&mut (*chip).mutex);
                    return ptr::null_mut();
                }
                (*(*ep).clock_ref).opened += 1;
            }

            (*ep).cur_audiofmt = fp;
            (*ep).cur_channels = (*fp).channels;
            (*ep).cur_rate = params_rate(params);
            (*ep).cur_format = params_format(params);
            (*ep).cur_frame_bytes = (snd_pcm_format_physical_width((*ep).cur_format)
                * (*ep).cur_channels
                / 8) as u32;
            (*ep).cur_period_frames = params_period_size(params);
            (*ep).cur_period_bytes = (*ep).cur_period_frames * (*ep).cur_frame_bytes;
            (*ep).cur_buffer_periods = params_periods(params);

            if (*ep).type_ == SND_USB_ENDPOINT_TYPE_SYNC {
                endpoint_set_syncinterval(chip, ep);
            }

            (*ep).implicit_fb_sync = (*fp).implicit_fb;
            (*ep).need_setup = true;
            (*ep).need_prepare = true;
            (*ep).fixed_rate = fixed_rate;

            usb_audio_dbg(
                chip,
                "  channels=%d, rate=%d, format=%s, period_bytes=%d, periods=%d, implicit_fb=%d\n\0"
                    .as_ptr() as *const _,
                (*ep).cur_channels,
                (*ep).cur_rate,
                snd_pcm_format_name((*ep).cur_format).as_ptr(),
                (*ep).cur_period_bytes,
                (*ep).cur_buffer_periods,
                if (*ep).implicit_fb_sync { 1 } else { 0 },
            );
        } else {
            if (*ep).iface_ref.is_null() {
                mutex_unlock(&mut (*chip).mutex);
                return ptr::null_mut();
            }

            if !endpoint_compatible(ep, fp, params) {
                usb_audio_err(
                    chip,
                    "Incompatible EP setup for 0x%x\n\0".as_ptr() as *const _,
                    ep_num,
                );
                mutex_unlock(&mut (*chip).mutex);
                return ptr::null_mut();
            }

            usb_audio_dbg(
                chip,
                "Reopened EP 0x%x (count %d)\n\0".as_ptr() as *const _,
                ep_num,
                (*ep).opened,
            );
        }

        if (*(*ep).iface_ref).opened == 0 {
            (*(*ep).iface_ref).need_setup = true;
        }
        (*(*ep).iface_ref).opened += 1;

        (*ep).opened += 1;
        mutex_unlock(&mut (*chip).mutex);
        ep
    }
}

pub extern "C" fn snd_usb_endpoint_set_sync(
    chip: *mut SndUsbAudio,
    data_ep: *mut SndUsbEndpoint,
    sync_ep: *mut SndUsbEndpoint,
) {
    unsafe {
        (*data_ep).sync_source = sync_ep;
    }
}

pub extern "C" fn snd_usb_endpoint_set_callback(
    ep: *mut SndUsbEndpoint,
    prepare: Option<
        unsafe extern "C" fn(*mut SndUsbSubstream, *mut Urb, bool) -> i32,
    >,
    retire: Option<unsafe extern "C" fn(*mut SndUsbSubstream, *mut Urb)>,
    data_subs: *mut SndUsbSubstream,
) {
    unsafe {
        (*ep).prepare_data_urb = prepare;
        (*ep).retire_data_urb = retire;
        if !data_subs.is_null() {
            (*ep).lowlatency_playback = (*data_subs).lowlatency_playback;
        } else {
            (*ep).lowlatency_playback = false;
        }
        write_once(data_subs, &mut (*ep).data_subs as *mut _);
    }
}

unsafe fn endpoint_set_interface(chip: *mut SndUsbAudio, ep: *mut SndUsbEndpoint, set: bool) -> i32 {
    let altset = if set { (*ep).altsetting } else { 0 };
    let mut err: i32;
    let mut retries: i32 = 0;
    const MAX_RETRIES: i32 = 5;

    if (*(*ep).iface_ref).altset == altset {
        return 0;
    }
    if atomic_read(&(*chip).shutdown) != 0 {
        return -libc::ENODEV as i32;
    }

    usb_audio_dbg(
        chip,
        "Setting usb interface %d:%d for EP 0x%x\n\0".as_ptr() as *const _,
        (*ep).iface,
        altset,
        (*ep).ep_num,
    );
    loop {
        err = usb_set_interface((*chip).dev, (*ep).iface, altset);
        if err < 0 {
            if err == -libc::EPROTO as i32 && retries < MAX_RETRIES {
                retries += 1;
                msleep(5 * (1 << (retries - 1)));
                continue;
            }
            usb_audio_err_ratelimited(
                chip,
                "%d:%d: usb_set_interface failed (%d)\n\0".as_ptr() as *const _,
                (*ep).iface,
                altset,
                err,
            );
            return err;
        }
        break;
    }

    if ((*chip).quirk_flags & QUIRK_FLAG_IFACE_DELAY) != 0 {
        msleep(50);
    }
    (*(*ep).iface_ref).altset = altset;
    0
}

pub extern "C" fn snd_usb_endpoint_close(chip: *mut SndUsbAudio, ep: *mut SndUsbEndpoint) {
    unsafe {
        mutex_lock(&mut (*chip).mutex);
        usb_audio_dbg(
            chip,
            "Closing EP 0x%x (count %d)\n\0".as_ptr() as *const _,
            (*ep).ep_num,
            (*ep).opened,
        );

        (*(*ep).iface_ref).opened -= 1;
        if (*(*ep).iface_ref).opened == 0
            && ((*chip).quirk_flags & QUIRK_FLAG_IFACE_SKIP_CLOSE) == 0
        {
            endpoint_set_interface(chip, ep, false);
        }

        (*ep).opened -= 1;
        if (*ep).opened == 0 {
            if !(*ep).clock_ref.is_null() {
                (*(*ep).clock_ref).opened -= 1;
                if (*(*ep).clock_ref).opened == 0 {
                    (*(*ep).clock_ref).rate = 0;
                }
            }
            (*ep).iface = 0;
            (*ep).altsetting = 0;
            (*ep).cur_audiofmt = ptr::null();
            (*ep).cur_rate = 0;
            (*ep).iface_ref = ptr::null_mut();
            (*ep).clock_ref = ptr::null_mut();
            usb_audio_dbg(
                chip,
                "EP 0x%x closed\n\0".as_ptr() as *const _,
                (*ep).ep_num,
            );
        }
        mutex_unlock(&mut (*chip).mutex);
    }
}

pub extern "C" fn snd_usb_endpoint_suspend(ep: *mut SndUsbEndpoint) {
    unsafe {
        (*ep).need_prepare = true;
        if !(*ep).iface_ref.is_null() {
            (*(*ep).iface_ref).need_setup = true;
        }
        if !(*ep).clock_ref.is_null() {
            (*(*ep).clock_ref).rate = 0;
        }
    }
}

unsafe fn wait_clear_urbs(ep: *mut SndUsbEndpoint) -> i32 {
    let end_time = jiffies() + msecs_to_jiffies(1000);
    let mut alive: i32;

    if atomic_read(&(*ep).state) != EP_STATE_STOPPING as i32 {
        return 0;
    }

    loop {
        alive = atomic_read(&(*ep).submitted_urbs);
        if alive == 0 {
            break;
        }

        schedule_timeout_uninterruptible(1);
        if !time_before(jiffies(), end_time) {
            break;
        }
    }

    if alive != 0 {
        usb_audio_err(
            (*ep).chip,
            "timeout: still %d active urbs on EP #%x\n\0".as_ptr() as *const _,
            alive,
            (*ep).ep_num,
        );
    }

    if ep_state_update(ep, EP_STATE_STOPPING as i32, EP_STATE_STOPPED as i32) {
        (*ep).sync_sink = ptr::null_mut();
        snd_usb_endpoint_set_callback(ep, None, None, ptr::null_mut());
    }

    0
}

pub extern "C" fn snd_usb_endpoint_sync_pending_stop(ep: *mut SndUsbEndpoint) {
    unsafe {
        if !ep.is_null() {
            wait_clear_urbs(ep);
        }
    }
}

unsafe fn stop_urbs(ep: *mut SndUsbEndpoint, force: bool, keep_pending: bool) -> i32 {
    if !force && atomic_read(&(*ep).running) != 0 {
        return -libc::EBUSY as i32;
    }

    if !ep_state_update(ep, EP_STATE_RUNNING as i32, EP_STATE_STOPPING as i32) {
        return 0;
    }

    spin_lock_irqsave(&(*ep).lock);
    init_list_head(&mut (*ep).ready_playback_urbs);
    (*ep).next_packet_head = 0;
    (*ep).next_packet_queued = 0;
    spin_unlock_irqrestore(&(*ep).lock);

    if keep_pending {
        return 0;
    }

    for i in 0..(*ep).nurbs {
        if test_bit(i, &(*ep).active_mask) != 0 {
            if test_and_set_bit(i, &mut (*ep).unlink_mask) == 0 {
                let u = (*ep).urb[i].urb;
                usb_unlink_urb(u);
            }
        }
    }

    0
}

unsafe fn release_urbs(ep: *mut SndUsbEndpoint, force: bool) -> i32 {
    let mut err: i32;

    snd_usb_endpoint_set_callback(ep, None, None, ptr::null_mut());

    err = stop_urbs(ep, force, false);
    if err != 0 {
        return err;
    }

    wait_clear_urbs(ep);

    for i in 0..(*ep).nurbs {
        release_urb_ctx(&mut (*ep).urb[i]);
    }

    usb_free_coherent(
        (*(*ep).chip).dev,
        (SYNC_URBS * 4) as usize,
        (*ep).syncbuf,
        (*ep).sync_dma,
    );

    (*ep).syncbuf = ptr::null_mut();
    (*ep).nurbs = 0;
    0
}

unsafe fn data_ep_set_params(ep: *mut SndUsbEndpoint) -> i32 {
    let chip = (*ep).chip;
    let fmt = (*ep).cur_audiofmt;
    let mut frame_bits = ((*ep).cur_frame_bytes * 8) as i32;
    let tx_length_quirk = has_tx_length_quirk(chip) && usb_pipeout((*ep).pipe) != 0;

    usb_audio_dbg(
        chip,
        "Setting params for data EP 0x%x, pipe 0x%x\n\0".as_ptr() as *const _,
        (*ep).ep_num,
        (*ep).pipe,
    );

    if (*ep).cur_format == SNDRV_PCM_FORMAT_DSD_U16_LE && (*fmt).dsd_dop {
        frame_bits += ((*ep).cur_channels << 3) as i32;
    }

    (*ep).datainterval = (*fmt).datainterval;
    (*ep).stride = (frame_bits >> 3) as u32;

    match (*ep).cur_format {
        SNDRV_PCM_FORMAT_U8 => {
            (*ep).silence_value = 0x80;
        }
        SNDRV_PCM_FORMAT_DSD_U8
        | SNDRV_PCM_FORMAT_DSD_U16_LE
        | SNDRV_PCM_FORMAT_DSD_U32_LE
        | SNDRV_PCM_FORMAT_DSD_U16_BE
        | SNDRV_PCM_FORMAT_DSD_U32_BE => {
            (*ep).silence_value = 0x69;
        }
        _ => {
            (*ep).silence_value = 0;
        }
    }

    (*ep).freqmax = (*ep).freqn + ((*ep).freqn >> 1);
    let mut maxsize = ((((*ep).freqmax << (*ep).datainterval) + 0xffff) >> 16) * (frame_bits >> 3);
    if tx_length_quirk {
        maxsize += core::mem::size_of::<u32>() as i32;
    }
    if (*ep).maxpacksize != 0 && ((*ep).maxpacksize as i32) < maxsize {
        let mut data_maxsize = (*ep).maxpacksize as i32;
        maxsize = data_maxsize;

        if tx_length_quirk {
            data_maxsize -= core::mem::size_of::<u32>() as i32;
        }
        (*ep).freqmax = ((data_maxsize / (frame_bits >> 3)) << (16 - (*ep).datainterval)) as u32;
    }

    if (*ep).fill_max != 0 {
        (*ep).curpacksize = (*ep).maxpacksize;
        maxsize = (*ep).maxpacksize as i32;
    } else {
        (*ep).curpacksize = maxsize as u32;
    }

    let mut packs_per_ms: u32;
    let mut max_packs_per_urb: u32;
    if snd_usb_get_speed((*chip).dev) != USB_SPEED_FULL {
        packs_per_ms = 8 >> (*ep).datainterval;
        max_packs_per_urb = MAX_PACKS_HS;
    } else {
        packs_per_ms = 1;
        max_packs_per_urb = MAX_PACKS;
    }
    if !(*ep).sync_source.is_null() && (*(*ep).sync_source).implicit_fb_sync == 0 {
        let temp = 1u32 << (*(*ep).sync_source).syncinterval;
        if max_packs_per_urb > temp {
            max_packs_per_urb = temp;
        }
    }
    let temp = max_packs_per_urb >> (*ep).datainterval;
    max_packs_per_urb = if temp > 0 { temp } else { 1 };

    let mut urb_packs: u32;
    if usb_pipein((*ep).pipe) != 0 || (*ep).implicit_fb_sync != 0 {
        urb_packs = if max_packs_per_urb > packs_per_ms {
            packs_per_ms
        } else {
            max_packs_per_urb
        };
        while urb_packs > 1 && urb_packs * (maxsize as u32) >= (*ep).cur_period_bytes {
            urb_packs >>= 1;
        }
        (*ep).nurbs = MAX_URBS as i32;
    } else {
        let mut minsize = ((*ep).freqn >> (16 - (*ep).datainterval)) * (frame_bits >> 3);
        if !(*ep).sync_source.is_null() {
            minsize -= minsize >> 3;
        }
        minsize = if minsize > 1 { minsize } else { 1 };

        let max_packs_per_period = div_round_up((*ep).cur_period_bytes, minsize as u32);
        let urbs_per_period = div_round_up(max_packs_per_period, max_packs_per_urb);
        urb_packs = div_round_up(max_packs_per_period, urbs_per_period);

        (*ep).max_urb_frames = div_round_up((*ep).cur_period_frames, urbs_per_period);

        let max_urbs = if MAX_URBS < (MAX_QUEUE * packs_per_ms / urb_packs) as usize {
            MAX_URBS
        } else {
            (MAX_QUEUE * packs_per_ms / urb_packs) as usize
        };
        if ((*chip).quirk_flags & QUIRK_FLAG_PLAYBACK_URB_FIXUP) != 0 {
            (*ep).nurbs = MAX_URBS as i32;
        } else {
            let temp = (urbs_per_period as usize) * ((*ep).cur_buffer_periods as usize);
            (*ep).nurbs = if max_urbs < temp { max_urbs as i32 } else { temp as i32 };
        }
    }

    for i in 0..((*ep).nurbs as usize) {
        let u = &mut (*ep).urb[i];
        u.index = i as i32;
        u.ep = ep;
        u.packets = urb_packs as i32;

        if (*fmt).fmt_type == UAC_FORMAT_TYPE_II {
            u.packets += 1;
        }
        u.buffer_size = (maxsize as u32) * u.packets as u32;
        u.urb = usb_alloc_urb(u.packets as u32, GFP_KERNEL);
        if u.urb.is_null() {
            release_urbs(ep, false);
            return -libc::ENOMEM as i32;
        }

        (*u.urb).transfer_buffer = usb_alloc_coherent(
            (*chip).dev,
            u.buffer_size as usize,
            GFP_KERNEL,
            &mut (*u.urb).transfer_dma,
        );
        if (*u.urb).transfer_buffer.is_null() {
            release_urbs(ep, false);
            return -libc::ENOMEM as i32;
        }
        (*u.urb).pipe = (*ep).pipe;
        (*u.urb).transfer_flags = URB_NO_TRANSFER_DMA_MAP;
        if ((*chip).quirk_flags & QUIRK_FLAG_PLAYBACK_URB_FIXUP) != 0 {
            (*u.urb).transfer_flags |= URB_ISO_ASAP;
        }
        (*u.urb).interval = 1 << (*ep).datainterval;
        (*u.urb).context = u as *mut _ as *mut _;
        (*u.urb).complete = Some(snd_complete_urb);
        init_list_head(&mut u.ready_list);
    }

    0
}

unsafe fn sync_ep_set_params(ep: *mut SndUsbEndpoint) -> i32 {
    let chip = (*ep).chip;

    usb_audio_dbg(
        chip,
        "Setting params for sync EP 0x%x, pipe 0x%x\n\0".as_ptr() as *const _,
        (*ep).ep_num,
        (*ep).pipe,
    );

    (*ep).syncbuf = usb_alloc_coherent(
        (*chip).dev,
        (SYNC_URBS * 4) as usize,
        GFP_KERNEL,
        &mut (*ep).sync_dma,
    );
    if (*ep).syncbuf.is_null() {
        return -libc::ENOMEM as i32;
    }

    (*ep).nurbs = SYNC_URBS as i32;
    for i in 0..(SYNC_URBS as usize) {
        let u = &mut (*ep).urb[i];
        u.index = i as i32;
        u.ep = ep;
        u.packets = 1;
        u.urb = usb_alloc_urb(1, GFP_KERNEL);
        if u.urb.is_null() {
            release_urbs(ep, false);
            return -libc::ENOMEM as i32;
        }
        (*u.urb).transfer_buffer = ((*ep).syncbuf as *mut u8).add(i * 4) as *mut _;
        (*u.urb).transfer_dma = (*ep).sync_dma + (i * 4) as u64;
        (*u.urb).transfer_buffer_length = 4;
        (*u.urb).pipe = (*ep).pipe;
        (*u.urb).transfer_flags = URB_NO_TRANSFER_DMA_MAP;
        (*u.urb).number_of_packets = 1;
        (*u.urb).interval = 1 << (*ep).syncinterval;
        (*u.urb).context = u as *mut _ as *mut _;
        (*u.urb).complete = Some(snd_complete_urb);
    }

    0
}

unsafe fn update_clock_ref_rate(chip: *mut SndUsbAudio, ep: *mut SndUsbEndpoint) -> i32 {
    let clock = (*ep).clock_ref;
    let rate = (*ep).cur_rate;

    if clock.is_null() || (*clock).rate == rate {
        return rate;
    }
    if (*clock).rate != 0 {
        if atomic_read(&(*clock).locked) != 0 {
            return (*clock).rate;
        }
        if (*clock).rate != rate {
            usb_audio_err(
                chip,
                "Mismatched sample rate %d vs %d for EP 0x%x\n\0".as_ptr() as *const _,
                (*clock).rate,
                rate,
                (*ep).ep_num,
            );
            return (*clock).rate;
        }
    }
    (*clock).rate = rate;
    (*clock).need_setup = true;
    rate
}

pub extern "C" fn snd_usb_endpoint_set_params(
    chip: *mut SndUsbAudio,
    ep: *mut SndUsbEndpoint,
) -> i32 {
    unsafe {
        let fmt = (*ep).cur_audiofmt;
        let mut err: i32;

        mutex_lock(&mut (*chip).mutex);
        if !(*ep).need_setup {
            mutex_unlock(&mut (*chip).mutex);
            return 0;
        }

        err = release_urbs(ep, false);
        if err < 0 {
            mutex_unlock(&mut (*chip).mutex);
            return err;
        }

        (*ep).datainterval = (*fmt).datainterval;
        (*ep).maxpacksize = (*fmt).maxpacksize;
        (*ep).fill_max = ((*fmt).attributes & UAC_EP_CS_ATTR_FILL_MAX) != 0;

        if snd_usb_get_speed((*chip).dev) == USB_SPEED_FULL {
            (*ep).freqn = get_usb_full_speed_rate((*ep).cur_rate);
            (*ep).pps = 1000 >> (*ep).datainterval;
        } else {
            (*ep).freqn = get_usb_high_speed_rate((*ep).cur_rate);
            (*ep).pps = 8000 >> (*ep).datainterval;
        }

        (*ep).sample_rem = (*ep).cur_rate % (*ep).pps;
        (*ep).packsize[0] = (*ep).cur_rate / (*ep).pps;
        (*ep).packsize[1] = ((*ep).cur_rate + ((*ep).pps - 1)) / (*ep).pps;
        if (*ep).packsize[1] > (*ep).maxpacksize {
            usb_audio_dbg(
                chip,
                "Too small maxpacksize %u for rate %u / pps %u\n\0".as_ptr() as *const _,
                (*ep).maxpacksize,
                (*ep).cur_rate,
                (*ep).pps,
            );
            mutex_unlock(&mut (*chip).mutex);
            return -libc::EINVAL as i32;
        }

        (*ep).freqm = (*ep).freqn;
        (*ep).freqshift = i32::MIN;

        (*ep).phase = 0;

        match (*ep).type_ {
            SND_USB_ENDPOINT_TYPE_DATA => {
                err = data_ep_set_params(ep);
            }
            SND_USB_ENDPOINT_TYPE_SYNC => {
                err = sync_ep_set_params(ep);
            }
            _ => {
                err = -libc::EINVAL as i32;
            }
        }

        usb_audio_dbg(
            chip,
            "Set up %d URBS, ret=%d\n\0".as_ptr() as *const _,
            (*ep).nurbs,
            err,
        );

        if err < 0 {
            mutex_unlock(&mut (*chip).mutex);
            return err;
        }

        (*ep).maxframesize = (*ep).maxpacksize / (*ep).cur_frame_bytes;
        (*ep).curframesize = (*ep).curpacksize / (*ep).cur_frame_bytes;

        if (*ep).packsize[0] > (*ep).maxframesize {
            (*ep).packsize[0] = (*ep).maxframesize;
        }
        if (*ep).packsize[1] > (*ep).maxframesize {
            (*ep).packsize[1] = (*ep).maxframesize;
        }

        let result = update_clock_ref_rate(chip, ep);
        if result >= 0 {
            (*ep).need_setup = false;
            err = 0;
        } else {
            err = result;
        }

        mutex_unlock(&mut (*chip).mutex);
        err
    }
}

unsafe fn init_sample_rate(chip: *mut SndUsbAudio, ep: *mut SndUsbEndpoint) -> i32 {
    let clock = (*ep).clock_ref;
    let mut rate: i32;

    rate = update_clock_ref_rate(chip, ep);
    if rate < 0 {
        return rate;
    }
    if !clock.is_null() && !(*clock).need_setup {
        return 0;
    }

    if !(*ep).fixed_rate {
        let err = snd_usb_init_sample_rate(chip, (*ep).cur_audiofmt, rate);
        if err < 0 {
            if !clock.is_null() {
                (*clock).rate = 0;
            }
            return err;
        }
    }

    if !clock.is_null() {
        (*clock).need_setup = false;
    }
    0
}

pub extern "C" fn snd_usb_endpoint_prepare(
    chip: *mut SndUsbAudio,
    ep: *mut SndUsbEndpoint,
) -> i32 {
    unsafe {
        let mut iface_first: bool;
        let mut err: i32 = 0;

        mutex_lock(&mut (*chip).mutex);
        if (*ep).iface_ref.is_null() {
            mutex_unlock(&mut (*chip).mutex);
            return 0;
        }
        if !(*ep).need_prepare {
            mutex_unlock(&mut (*chip).mutex);
            return 0;
        }

        if !(*(*ep).iface_ref).need_setup {
            if (*(*ep).cur_audiofmt).protocol == UAC_VERSION_1 {
                err = init_sample_rate(chip, ep);
                if err < 0 {
                    mutex_unlock(&mut (*chip).mutex);
                    return err;
                }
            }
            goto_done(ep);
            mutex_unlock(&mut (*chip).mutex);
            return 1;
        }

        endpoint_set_interface(chip, ep, false);

        iface_first = (*(*ep).cur_audiofmt).protocol == UAC_VERSION_1;
        if ((*chip).quirk_flags & QUIRK_FLAG_SET_IFACE_FIRST) != 0 {
            iface_first = true;
        }
        if iface_first {
            err = endpoint_set_interface(chip, ep, true);
            if err < 0 {
                mutex_unlock(&mut (*chip).mutex);
                return err;
            }
        }

        err = snd_usb_select_mode_quirk(chip, (*ep).cur_audiofmt);
        if err < 0 {
            mutex_unlock(&mut (*chip).mutex);
            return err;
        }

        err = snd_usb_init_pitch(chip, (*ep).cur_audiofmt);
        if err < 0 {
            mutex_unlock(&mut (*chip).mutex);
            return err;
        }

        err = init_sample_rate(chip, ep);
        if err < 0 {
            mutex_unlock(&mut (*chip).mutex);
            return err;
        }

        if !iface_first {
            err = endpoint_set_interface(chip, ep, true);
            if err < 0 {
                mutex_unlock(&mut (*chip).mutex);
                return err;
            }
        }

        (*(*ep).iface_ref).need_setup = false;

        goto_done(ep);
        mutex_unlock(&mut (*chip).mutex);
        1
    }
}

unsafe fn goto_done(ep: *mut SndUsbEndpoint) {
    (*ep).need_prepare = false;
}

pub extern "C" fn snd_usb_endpoint_get_clock_rate(chip: *mut SndUsbAudio, clock: i32) -> i32 {
    unsafe {
        let mut rate: i32 = 0;

        if clock == 0 {
            return 0;
        }
        mutex_lock(&mut (*chip).mutex);
        let mut pos = (*chip).clock_ref_list;
        loop {
            if pos.is_null() {
                break;
            }
            let ref_ = container_of(pos, SndUsbClockRef, list) as *mut SndUsbClockRef;
            if (*ref_).clock as i32 == clock {
                rate = (*ref_).rate;
                break;
            }
            pos = (*pos).next;
        }
        mutex_unlock(&mut (*chip).mutex);
        rate
    }
}

pub extern "C" fn snd_usb_endpoint_start(ep: *mut SndUsbEndpoint) -> i32 {
    unsafe {
        let is_playback = usb_pipeout((*ep).pipe) != 0;
        let mut err: i32;

        if atomic_read(&(*(*ep).chip).shutdown) != 0 {
            return -libc::EBADFD as i32;
        }

        if !(*ep).sync_source.is_null() {
            write_once(
                (*ep) as *mut _,
                &(*ep).sync_source as *const _ as *mut _,
            );
        }

        usb_audio_dbg(
            (*ep).chip,
            "Starting %s EP 0x%x (running %d)\n\0".as_ptr() as *const _,
            ep_type_name((*ep).type_).as_ptr(),
            (*ep).ep_num,
            atomic_read(&(*ep).running),
        );

        if atomic_inc_return(&mut (*ep).running) != 1 {
            return 0;
        }

        if !(*ep).clock_ref.is_null() {
            atomic_inc(&mut (*(*ep).clock_ref).locked);
        }

        (*ep).active_mask = 0;
        (*ep).unlink_mask = 0;
        (*ep).phase = 0;
        (*ep).sample_accum = 0;

        snd_usb_endpoint_start_quirk(ep);

        if !ep_state_update(ep, EP_STATE_STOPPED as i32, EP_STATE_RUNNING as i32) {
            snd_usb_endpoint_stop(ep, false);
            return -libc::EPIPE as i32;
        }

        if snd_usb_endpoint_implicit_feedback_sink(ep) != 0
            && ((*(*ep).chip).quirk_flags & QUIRK_FLAG_PLAYBACK_FIRST) == 0
        {
            usb_audio_dbg(
                (*ep).chip,
                "No URB submission due to implicit fb sync\n\0".as_ptr() as *const _,
            );
            let i = 0;
            fill_rest(ep, is_playback, i);
            return 0;
        }

        let mut i = 0;
        while (i as i32) < (*ep).nurbs {
            let urb = (*ep).urb[i].urb;

            if urb.is_null() {
                snd_usb_endpoint_stop(ep, false);
                return -libc::EPIPE as i32;
            }

            if is_playback {
                err = prepare_outbound_urb(ep, (*urb).context as *mut _, true);
            } else {
                err = prepare_inbound_urb(ep, (*urb).context as *mut _);
            }
            if err < 0 {
                if err == -libc::EAGAIN as i32 {
                    break;
                }
                usb_audio_dbg(
                    (*ep).chip,
                    "EP 0x%x: failed to prepare urb: %d\n\0".as_ptr() as *const _,
                    (*ep).ep_num,
                    err,
                );
                snd_usb_endpoint_stop(ep, false);
                return -libc::EPIPE as i32;
            }

            if atomic_read(&(*(*ep).chip).shutdown) == 0 {
                err = usb_submit_urb(urb, GFP_ATOMIC);
            } else {
                err = -libc::ENODEV as i32;
            }
            if err < 0 {
                if atomic_read(&(*(*ep).chip).shutdown) == 0 {
                    usb_audio_err(
                        (*ep).chip,
                        "cannot submit urb %d, error %d: %s\n\0".as_ptr() as *const _,
                        i,
                        err,
                        usb_error_string(err).as_ptr(),
                    );
                }
                snd_usb_endpoint_stop(ep, false);
                return -libc::EPIPE as i32;
            }
            set_bit(i, &mut (*ep).active_mask);
            atomic_inc(&mut (*ep).submitted_urbs);
            i += 1;
        }

        if i == 0 {
            usb_audio_dbg(
                (*ep).chip,
                "XRUN at starting EP 0x%x\n\0".as_ptr() as *const _,
                (*ep).ep_num,
            );
            snd_usb_endpoint_stop(ep, false);
            return -libc::EPIPE as i32;
        }

        usb_audio_dbg(
            (*ep).chip,
            "%d URBs submitted for EP 0x%x\n\0".as_ptr() as *const _,
            i,
            (*ep).ep_num,
        );

        fill_rest(ep, is_playback, i);
        0
    }
}

unsafe fn fill_rest(ep: *mut SndUsbEndpoint, is_playback: bool, i: usize) {
    if is_playback {
        let mut j = i;
        while j < (*ep).nurbs as usize {
            push_back_to_ready_list(ep, &mut (*ep).urb[j]);
            j += 1;
        }
    }
}

pub extern "C" fn snd_usb_endpoint_stop(ep: *mut SndUsbEndpoint, keep_pending: bool) {
    unsafe {
        if ep.is_null() {
            return;
        }

        usb_audio_dbg(
            (*ep).chip,
            "Stopping %s EP 0x%x (running %d)\n\0".as_ptr() as *const _,
            ep_type_name((*ep).type_).as_ptr(),
            (*ep).ep_num,
            atomic_read(&(*ep).running),
        );

        if atomic_read(&(*ep).running) == 0 {
            return;
        }

        if atomic_dec_return(&mut (*ep).running) == 0 {
            if !(*ep).sync_source.is_null() {
                write_once(
                    ptr::null_mut::<SndUsbEndpoint>(),
                    &(*(*ep).sync_source).sync_sink as *const _ as *mut _,
                );
            }
            stop_urbs(ep, false, keep_pending);
            if !(*ep).clock_ref.is_null() {
                atomic_dec(&mut (*(*ep).clock_ref).locked);
            }

            if ((*(*ep).chip).quirk_flags & QUIRK_FLAG_FORCE_IFACE_RESET) != 0
                && usb_pipeout((*ep).pipe) != 0
            {
                (*ep).need_prepare = true;
                if !(*ep).iface_ref.is_null() {
                    (*(*ep).iface_ref).need_setup = true;
                }
            }
        }
    }
}

pub extern "C" fn snd_usb_endpoint_release(ep: *mut SndUsbEndpoint) {
    unsafe {
        release_urbs(ep, true);
    }
}

pub extern "C" fn snd_usb_endpoint_free_all(chip: *mut SndUsbAudio) {
    unsafe {
        let mut pos = (*chip).ep_list;
        while !pos.is_null() {
            let next = (*pos).next;
            let ep = container_of(pos, SndUsbEndpoint, list);
            kfree(ep as *mut _);
            pos = next;
        }

        pos = (*chip).iface_ref_list;
        while !pos.is_null() {
            let next = (*pos).next;
            let ip = container_of(pos, SndUsbIfaceRef, list);
            kfree(ip as *mut _);
            pos = next;
        }

        pos = (*chip).clock_ref_list;
        while !pos.is_null() {
            let next = (*pos).next;
            let cp = container_of(pos, SndUsbClockRef, list);
            kfree(cp as *mut _);
            pos = next;
        }
    }
}

unsafe fn snd_usb_handle_sync_urb(
    ep: *mut SndUsbEndpoint,
    sender: *mut SndUsbEndpoint,
    urb: *const Urb,
) {
    let mut shift: i32;
    let mut f: u32;

    if snd_usb_endpoint_implicit_feedback_sink(ep) != 0 && atomic_read(&(*ep).running) != 0 {
        let mut bytes: u32 = 0;
        let in_ctx = (*urb).context as *mut SndUrbCtx;

        for i in 0..(*in_ctx).packets {
            if (*urb).iso_frame_desc[i as usize].status == 0 {
                bytes += (*urb).iso_frame_desc[i as usize].actual_length;
            }
        }

        if bytes == 0
            && ((*(*ep).chip).quirk_flags & QUIRK_FLAG_IFB_SILENCE_ON_EMPTY) == 0
        {
            return;
        }

        spin_lock_irqsave(&(*ep).lock);
        if (*ep).next_packet_queued >= ARRAY_SIZE_NEXT_PACKET {
            spin_unlock_irqrestore(&(*ep).lock);
            if notify_xrun(ep) {
                usb_audio_err(
                    (*ep).chip,
                    "next packet FIFO overflow EP 0x%x\n\0".as_ptr() as *const _,
                    (*ep).ep_num,
                );
            }
            return;
        }

        let out_packet = next_packet_fifo_enqueue(ep);

        (*out_packet).packets = (*in_ctx).packets;
        for i in 0..(*in_ctx).packets {
            if (*urb).iso_frame_desc[i as usize].status == 0 {
                let frames =
                    (*urb).iso_frame_desc[i as usize].actual_length / (*sender).stride;
                (*out_packet).packet_size[i as usize] = if frames > (*ep).maxframesize as u32 {
                    (*ep).maxframesize as u32
                } else {
                    frames
                };
            } else {
                (*out_packet).packet_size[i as usize] = 0;
            }
        }

        spin_unlock_irqrestore(&(*ep).lock);
        snd_usb_queue_pending_output_urbs(ep, false);

        return;
    }

    if (*urb).iso_frame_desc[0].status != 0
        || (*urb).iso_frame_desc[0].actual_length < 3
    {
        return;
    }

    f = le32_to_cpup((*urb).transfer_buffer as *const u32);
    if (*urb).iso_frame_desc[0].actual_length == 3 {
        f &= 0x00ffffff;
    } else {
        f &= 0x0fffffff;
    }

    if f == 0 {
        return;
    }

    if (*sender).tenor_fb_quirk {
        if f < (*ep).freqn - 0x8000 {
            f += 0xf000;
        } else if f > (*ep).freqn + 0x8000 {
            f -= 0xf000;
        }
    } else if (*ep).freqshift == i32::MIN {
        shift = 0;
        while f < (*ep).freqn - (*ep).freqn / 4 {
            f <<= 1;
            shift += 1;
        }
        while f > (*ep).freqn + (*ep).freqn / 2 {
            f >>= 1;
            shift -= 1;
        }
        (*ep).freqshift = shift;
    } else if (*ep).freqshift >= 0 {
        f <<= (*ep).freqshift;
    } else {
        f >>= -(*ep).freqshift;
    }

    if f >= (*ep).freqn - (*ep).freqn / 8 && f <= (*ep).freqmax {
        spin_lock_irqsave(&(*ep).lock);
        (*ep).freqm = f;
        spin_unlock_irqrestore(&(*ep).lock);
    } else {
        (*ep).freqshift = i32::MIN;
    }
}

// External function declarations
extern "C" {
    fn usb_free_coherent(dev: *mut _, size: usize, addr: *mut _, dma: u64);
    fn usb_free_urb(urb: *mut Urb);
    fn spin_lock_irqsave(lock: *mut _) -> u64;
    fn spin_unlock_irqrestore(lock: *mut _, flags: u64);
    fn spin_lock_init(lock: *mut _);
    fn memcpy(dest: *mut _, src: *const _, n: usize);
    fn memset(s: *mut _, c: i32, n: usize);
    fn cpu_to_le32(x: u32) -> u32;
    fn le32_to_cpup(x: *const u32) -> u32;
    fn le16_to_cpu(x: u16) -> u16;
    fn usb_pipeout(pipe: u32) -> i32;
    fn usb_pipein(pipe: u32) -> i32;
    fn usb_sndisocpipe(dev: *mut _, ep: i32) -> u32;
    fn usb_rcvisocpipe(dev: *mut _, ep: i32) -> u32;
    fn list_add_tail(new_node: *mut ListHead, head: *mut ListHead);
    fn list_del_init(entry: *mut ListHead);
    fn list_empty(head: *const ListHead) -> bool;
    fn list_first_entry(ptr: *const ListHead, ty: _, member: _) -> *mut _;
    fn atomic_read(v: *const AtomicT) -> i32;
    fn atomic_set(v: *mut AtomicT, i: i32);
    fn atomic_inc(v: *mut AtomicT);
    fn atomic_dec(v: *mut AtomicT);
    fn atomic_inc_return(v: *mut AtomicT) -> i32;
    fn atomic_dec_return(v: *mut AtomicT) -> i32;
    fn atomic_dec_and_test(v: *mut AtomicT) -> bool;
    fn atomic_try_cmpxchg(v: *mut AtomicT, old: *mut i32, new: i32) -> bool;
    fn read_once(x: *const _) -> *const _;
    fn write_once(val: *const _, ptr: *mut *const _);
    fn set_bit(nr: usize, addr: *mut u64);
    fn clear_bit(nr: usize, addr: *mut u64);
    fn test_bit(nr: usize, addr: *const u64) -> bool;
    fn test_and_set_bit(nr: usize, addr: *mut u64) -> bool;
    fn init_list_head(list: *mut ListHead);
    fn container_of(ptr: *const _, ty: _, member: _) -> *const _;
    fn kzalloc(size: usize) -> *mut u8;
    fn kfree(ptr: *mut _);
    fn usb_alloc_urb(iso_packets: u32, mem_flags: u32) -> *mut Urb;
    fn usb_alloc_coherent(dev: *mut _, size: usize, mem_flags: u32, dma: *mut u64) -> *mut u8;
    fn usb_submit_urb(urb: *mut Urb, mem_flags: u32) -> i32;
    fn usb_unlink_urb(urb: *mut Urb);
    fn usb_set_interface(dev: *mut _, iface: i32, altset: i32) -> i32;
    fn usb_audio_dbg(chip: *const _, fmt: *const u8, ...);
    fn usb_audio_err(chip: *const _, fmt: *const u8, ...);
    fn usb_audio_err_ratelimited(chip: *const _, fmt: *const u8, ...);
    fn snd_pcm_format_name(format: i32) -> *const u8;
    fn snd_pcm_format_physical_width(format: i32) -> i32;
    fn snd_pcm_stop_xrun(substream: *const _);
    fn snd_BUG_ON(cond: bool);
    fn snd_usb_get_speed(dev: *const _) -> i32;
    fn snd_usb_get_host_interface(chip: *const _, iface: i32, altsetting: i32) -> *const _;
    fn get_endpoint(alts: *const _, ep_idx: i32) -> *const _;
    fn snd_usb_init_sample_rate(chip: *const _, fmt: *const _, rate: i32) -> i32;
    fn snd_usb_select_mode_quirk(chip: *mut _, fmt: *const _) -> i32;
    fn snd_usb_init_pitch(chip: *mut _, fmt: *const _) -> i32;
    fn snd_usb_endpoint_start_quirk(ep: *const _);
    fn mutex_lock(m: *mut _);
    fn mutex_unlock(m: *mut _);
    fn schedule_timeout_uninterruptible(timeout: u64);
    fn jiffies() -> u64;
    fn msecs_to_jiffies(msecs: u64) -> u64;
    fn time_before(a: u64, b: u64) -> bool;
    fn msleep(msecs: u32);
}

// Type placeholders for external types
#[repr(C)]
struct ListHead;

#[repr(C)]
struct AtomicT;

#[repr(C)]
struct Urb;

#[repr(C)]
struct SndUsbEndpoint;

#[repr(C)]
struct SndUrbCtx;

#[repr(C)]
struct SndUsbSubstream;

#[repr(C)]
struct SndUsbAudio;

#[repr(C)]
struct AudioFormat;

#[repr(C)]
struct SndPcmHwParams;

#[repr(C)]
struct SndUsbPacketInfo;

const EP_STATE_STOPPED: i32 = 0;
const EP_STATE_RUNNING: i32 = 1;
const EP_STATE_STOPPING: i32 = 2;

const SND_USB_ENDPOINT_TYPE_DATA: i32 = 0;
const SND_USB_ENDPOINT_TYPE_SYNC: i32 = 1;

const USB_ENDPOINT_DIR_MASK: i32 = 0x80;
const USB_ENDPOINT_NUMBER_MASK: i32 = 0x0f;
const USB_DIR_OUT: i32 = 0x00;

const USB_SPEED_FULL: i32 = 1;
const USB_SPEED_HIGH: i32 = 3;

const USB_DT_ENDPOINT_AUDIO_SIZE: i32 = 9;

const UAC_VERSION_1: i32 = 0x0100;
const UAC_FORMAT_TYPE_II: i32 = 2;
const UAC_EP_CS_ATTR_FILL_MAX: u16 = 0x80;

const SNDRV_PCM_FORMAT_U8: i32 = 1;
const SNDRV_PCM_FORMAT_DSD_U8: i32 = 48;
const SNDRV_PCM_FORMAT_DSD_U16_LE: i32 = 49;
const SNDRV_PCM_FORMAT_DSD_U32_LE: i32 = 50;
const SNDRV_PCM_FORMAT_DSD_U16_BE: i32 = 51;
const SNDRV_PCM_FORMAT_DSD_U32_BE: i32 = 52;

const SNDRV_PCM_STATE_RUNNING: i32 = 2;

const QUIRK_FLAG_TX_LENGTH: u32 = 0x00000001;
const QUIRK_FLAG_IFACE_DELAY: u32 = 0x00000040;
const QUIRK_FLAG_IFACE_SKIP_CLOSE: u32 = 0x00000100;
const QUIRK_FLAG_PLAYBACK_URB_FIXUP: u32 = 0x00000400;
const QUIRK_FLAG_SET_IFACE_FIRST: u32 = 0x00002000;
const QUIRK_FLAG_FORCE_IFACE_RESET: u32 = 0x00004000;
const QUIRK_FLAG_PLAYBACK_FIRST: u32 = 0x00008000;
const QUIRK_FLAG_IFB_SILENCE_ON_EMPTY: u32 = 0x00020000;

const GFP_ATOMIC: u32 = 0x20;
const GFP_KERNEL: u32 = 0xd0;

const URB_NO_TRANSFER_DMA_MAP: u32 = 0x0004;
const URB_ISO_ASAP: u32 = 0x0002;

const MAX_PACKS: u32 = 6;
const MAX_PACKS_HS: u32 = 34;
const MAX_URBS: usize = 8;
const MAX_QUEUE: u32 = 24;
const SYNC_URBS: usize = 4;
const ARRAY_SIZE_NEXT_PACKET: usize = 8;

fn div_round_up(n: u32, d: u32) -> u32 {
    (n + d - 1) / d
}

fn params_rate(params: *const SndPcmHwParams) -> u32 {
    // Placeholder, returns rate from params
    0
}

fn params_format(params: *const SndPcmHwParams) -> i32 {
    // Placeholder, returns format from params
    0
}

fn params_period_size(params: *const SndPcmHwParams) -> u32 {
    // Placeholder, returns period size from params
    0
}

fn params_periods(params: *const SndPcmHwParams) -> u32 {
    // Placeholder, returns periods from params
    0
}

fn snd_pcm_format_name(format: i32) -> *const u8 {
    // Placeholder
    b"\0".as_ptr()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
