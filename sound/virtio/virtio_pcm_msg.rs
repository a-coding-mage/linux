// SPDX-License-Identifier: GPL-2.0+
/*
 * virtio-snd: Virtio sound device
 * Copyright (C) 2021 OpenSynergy GmbH
 */
// Includes: <sound/pcm_params.h>, "virtio_card.h"

#[repr(C)]
pub struct VirtioPcmMsg {
    pub substream: *mut VirtioPcmSubstream,
    pub xfer: VirtioSndPcmXfer,
    pub status: VirtioSndPcmStatus,
    pub length: usize,
    pub sgs: [Scatterlist; 0],
}

/// Index values for the virtio_pcm_msg->sgs field in an I/O message.
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum PcmMsgSgIndex {
    /// Element containing a virtio_snd_pcm_xfer structure.
    Xfer = 0,
    /// Element containing a virtio_snd_pcm_status structure.
    Status = 1,
    /// The first element containing a data buffer.
    Data = 2,
}

/// Count the number of sg-elements required to represent vmalloc'ed buffer.
///
/// # Arguments
/// * `data` - Pointer to vmalloc'ed buffer
/// * `length` - Buffer size
///
/// # Context
/// Any context.
///
/// # Returns
/// Number of physically contiguous parts in the data
unsafe fn virtsnd_pcm_sg_num(data: *mut u8, length: u32) -> i32 {
    let mut sg_address: usize = 0;
    let mut sg_length: u32 = 0;
    let mut num = 0;
    let mut curr_data = data;
    let mut curr_length = length;

    while curr_length > 0 {
        let pg = vmalloc_to_page(curr_data);
        let pg_address = page_to_phys(pg);
        let mut pg_length = PAGE_SIZE - offset_in_page(curr_data);
        if pg_length > curr_length as usize {
            pg_length = curr_length as usize;
        }

        if num == 0 || sg_address + sg_length as usize != pg_address {
            sg_address = pg_address;
            sg_length = pg_length as u32;
            num += 1;
        } else {
            sg_length += pg_length as u32;
        }

        curr_data = curr_data.add(pg_length);
        curr_length -= pg_length as u32;
    }

    num
}

/// Build sg-list from vmalloc'ed buffer.
///
/// Splits the buffer into physically contiguous parts and makes an sg-list of
/// such parts.
///
/// # Arguments
/// * `sgs` - Preallocated sg-list to populate
/// * `nsgs` - The maximum number of elements in the sgs
/// * `data` - Pointer to vmalloc'ed buffer
/// * `length` - Buffer size
///
/// # Context
/// Any context.
unsafe fn virtsnd_pcm_sg_from(
    sgs: *mut Scatterlist,
    nsgs: i32,
    data: *mut u8,
    length: u32,
) {
    let mut idx = -1i32;
    let mut curr_data = data;
    let mut curr_length = length;

    while curr_length > 0 {
        let pg = vmalloc_to_page(curr_data);
        let mut pg_length = PAGE_SIZE - offset_in_page(curr_data);
        if pg_length > curr_length as usize {
            pg_length = curr_length as usize;
        }

        if idx == -1
            || sg_phys(&*sgs.add(idx as usize)) as usize + (*sgs.add(idx as usize)).length
                != page_to_phys(pg)
        {
            if idx + 1 == nsgs {
                break;
            }
            idx += 1;
            sg_set_page(
                &mut *sgs.add(idx as usize),
                pg,
                pg_length as u32,
                offset_in_page(curr_data) as u32,
            );
        } else {
            (*sgs.add(idx as usize)).length += pg_length as u32;
        }

        curr_data = curr_data.add(pg_length);
        curr_length -= pg_length as u32;
    }

    sg_mark_end(&mut *sgs.add(idx as usize));
}

/// Allocate I/O messages.
///
/// The function slices the buffer into periods parts (each with the size of
/// period_bytes), and creates periods corresponding I/O messages.
///
/// # Arguments
/// * `vss` - VirtIO PCM substream
/// * `periods` - Current number of periods
/// * `period_bytes` - Current period size in bytes
///
/// # Context
/// Any context that permits to sleep.
///
/// # Returns
/// 0 on success, -ENOMEM on failure.
pub fn virtsnd_pcm_msg_alloc(
    vss: *mut VirtioPcmSubstream,
    periods: u32,
    period_bytes: u32,
) -> i32 {
    unsafe {
        let runtime = (*(*vss).substream).runtime;

        (*vss).msgs = kzalloc_objs((*vss).msgs, periods);
        if (*vss).msgs.is_null() {
            return -12; // ENOMEM
        }

        (*vss).nmsgs = periods;

        let mut i = 0;
        while i < periods {
            let data = (*runtime).dma_area.add((period_bytes * i) as usize) as *mut u8;
            let sg_num = virtsnd_pcm_sg_num(data, period_bytes);

            let msg = kzalloc_flex(core::mem::size_of::<VirtioPcmMsg>(), sg_num + 2);
            if msg.is_null() {
                return -12; // ENOMEM
            }

            let msg = msg as *mut VirtioPcmMsg;
            (*msg).substream = vss;
            sg_init_one(
                &mut (*msg).sgs[PcmMsgSgIndex::Xfer as usize] as *mut Scatterlist as *mut u8,
                &mut (*msg).xfer as *mut VirtioSndPcmXfer as *mut u8,
                core::mem::size_of::<VirtioSndPcmXfer>(),
            );
            sg_init_one(
                &mut (*msg).sgs[PcmMsgSgIndex::Status as usize] as *mut Scatterlist as *mut u8,
                &mut (*msg).status as *mut VirtioSndPcmStatus as *mut u8,
                core::mem::size_of::<VirtioSndPcmStatus>(),
            );
            virtsnd_pcm_sg_from(
                &mut (*msg).sgs[PcmMsgSgIndex::Data as usize],
                sg_num,
                data,
                period_bytes,
            );

            *(*vss).msgs.add(i as usize) = msg;
            i += 1;
        }

        0
    }
}

/// Free all allocated I/O messages.
///
/// # Arguments
/// * `vss` - VirtIO PCM substream
///
/// # Context
/// Any context.
pub fn virtsnd_pcm_msg_free(vss: *mut VirtioPcmSubstream) {
    unsafe {
        let mut i = 0;

        while !(*vss).msgs.is_null() && i < (*vss).nmsgs {
            kfree(*(*vss).msgs.add(i as usize) as *mut u8);
            i += 1;
        }
        kfree((*vss).msgs as *mut u8);

        (*vss).msgs = core::ptr::null_mut();
        (*vss).nmsgs = 0;
    }
}

/// Send asynchronous I/O messages.
///
/// All messages are organized in an ordered circular list. Each time the
/// function is called, all currently non-enqueued messages are added to the
/// virtqueue. For this, the function uses offset and bytes to calculate the
/// messages that need to be added.
///
/// # Arguments
/// * `vss` - VirtIO PCM substream
/// * `offset` - starting position that has been updated
/// * `bytes` - number of bytes that has been updated
///
/// # Context
/// Any context. Expects the tx/rx queue and the VirtIO substream
/// spinlocks to be held by caller.
///
/// # Returns
/// 0 on success, -errno on failure.
pub fn virtsnd_pcm_msg_send(
    vss: *mut VirtioPcmSubstream,
    offset: usize,
    bytes: usize,
) -> i32 {
    unsafe {
        let snd = (*vss).snd;
        let vdev = (*snd).vdev;
        let vqueue = virtsnd_pcm_queue(vss).vqueue;
        let period_bytes = snd_pcm_lib_period_bytes((*vss).substream);
        let start = offset / period_bytes;
        let end = (offset + bytes - 1) / period_bytes;
        let mut i = start;
        let msg_count = (*vss).msg_count;
        let mut notify = false;
        let mut curr_offset = offset;
        let mut curr_bytes = bytes;
        let mut rc = 0;

        while i <= end {
            let msg = *(*vss).msgs.add(i);
            let mut psgs: [*mut Scatterlist; 3] = [
                &mut (*msg).sgs[PcmMsgSgIndex::Xfer as usize],
                &mut (*msg).sgs[PcmMsgSgIndex::Data as usize],
                &mut (*msg).sgs[PcmMsgSgIndex::Status as usize],
            ];

            let mut n = period_bytes - (curr_offset % period_bytes);
            if n > curr_bytes {
                n = curr_bytes;
            }

            (*msg).length += n;
            if (*msg).length == period_bytes {
                (*msg).xfer.stream_id = cpu_to_le32((*vss).sid);
                core::ptr::write_bytes(
                    &mut (*msg).status as *mut VirtioSndPcmStatus as *mut u8,
                    0,
                    core::mem::size_of::<VirtioSndPcmStatus>(),
                );

                if (*vss).direction == SNDRV_PCM_STREAM_PLAYBACK {
                    rc = virtqueue_add_sgs(vqueue, &mut psgs, 2, 1, msg as *mut u8, 0x20);
                } else {
                    rc = virtqueue_add_sgs(vqueue, &mut psgs, 1, 2, msg as *mut u8, 0x20);
                }

                if rc != 0 {
                    dev_err(
                        &(*vdev).dev as *const u8 as *mut u8,
                        b"SID %u: failed to send I/O message\n" as *const u8,
                        (*vss).sid,
                    );
                    return rc;
                }

                (*vss).msg_count += 1;
            }

            curr_offset = 0;
            curr_bytes -= n;
            i += 1;
        }

        if msg_count == (*vss).msg_count {
            return 0;
        }

        if ((*vss).features & (1u32 << 0)) == 0 {
            notify = virtqueue_kick_prepare(vqueue) != 0;
        }

        if notify {
            virtqueue_notify(vqueue);
        }

        0
    }
}

/// Returns the number of pending I/O messages.
///
/// # Arguments
/// * `vss` - VirtIO substream
///
/// # Context
/// Any context.
///
/// # Returns
/// Number of messages
pub fn virtsnd_pcm_msg_pending_num(vss: *mut VirtioPcmSubstream) -> u32 {
    unsafe {
        let _guard = spinlock_irqsave(&mut (*vss).lock);
        (*vss).msg_count
    }
}

/// Complete an I/O message.
///
/// Completion of the message means the elapsed period. If transmission is
/// allowed, then each completed message is immediately placed back at the end
/// of the queue.
///
/// For the playback substream, written_bytes is equal to sizeof(msg->status).
///
/// For the capture substream, written_bytes is equal to sizeof(msg->status)
/// plus the number of captured bytes.
///
/// # Arguments
/// * `msg` - I/O message
/// * `written_bytes` - Number of bytes written to the message
///
/// # Context
/// Interrupt context. Takes and releases the VirtIO substream spinlock.
unsafe fn virtsnd_pcm_msg_complete(msg: *mut VirtioPcmMsg, written_bytes: usize) {
    let vss = (*msg).substream;

    let _guard = spinlock(&mut (*vss).lock);

    if (*vss).direction == SNDRV_PCM_STREAM_PLAYBACK
        || written_bytes <= core::mem::size_of::<VirtioSndPcmStatus>()
    {
        (*vss).hw_ptr += (*msg).length;
    } else {
        (*vss).hw_ptr += written_bytes - core::mem::size_of::<VirtioSndPcmStatus>();
    }

    if (*vss).hw_ptr >= (*vss).buffer_bytes {
        (*vss).hw_ptr -= (*vss).buffer_bytes;
    }

    (*msg).length = 0;

    (*vss).xfer_xrun = false;
    (*vss).msg_count -= 1;

    if (*vss).xfer_enabled {
        let runtime = (*(*vss).substream).runtime;

        (*runtime).delay = bytes_to_frames(
            runtime,
            le32_to_cpu((*msg).status.latency_bytes),
        );

        schedule_work(&mut (*vss).elapsed_period as *mut u8);
    } else if (*vss).msg_count == 0 {
        wake_up_all(&mut (*vss).msg_empty as *mut u8);
    }
}

/// Process all completed I/O messages.
///
/// # Arguments
/// * `queue` - Underlying tx/rx virtqueue
///
/// # Context
/// Interrupt context. Takes and releases the tx/rx queue spinlock.
unsafe fn virtsnd_pcm_notify_cb(queue: *mut VirtioSndQueue) {
    let mut written_bytes: u32 = 0;

    let _guard = spinlock_irqsave(&mut (*queue).lock);
    loop {
        virtqueue_disable_cb((*queue).vqueue);
        loop {
            let msg = virtqueue_get_buf((*queue).vqueue, &mut written_bytes);
            if msg.is_null() {
                break;
            }
            virtsnd_pcm_msg_complete(msg as *mut VirtioPcmMsg, written_bytes as usize);
        }
        if virtqueue_enable_cb((*queue).vqueue) == 0 {
            break;
        }
    }
}

/// Process all completed TX messages.
///
/// # Arguments
/// * `vqueue` - Underlying tx virtqueue
///
/// # Context
/// Interrupt context.
#[no_mangle]
pub extern "C" fn virtsnd_pcm_tx_notify_cb(vqueue: *mut Virtqueue) {
    unsafe {
        let snd = (*(*vqueue).vdev).priv as *mut VirtioSnd;

        virtsnd_pcm_notify_cb(virtsnd_tx_queue(snd));
    }
}

/// Process all completed RX messages.
///
/// # Arguments
/// * `vqueue` - Underlying rx virtqueue
///
/// # Context
/// Interrupt context.
#[no_mangle]
pub extern "C" fn virtsnd_pcm_rx_notify_cb(vqueue: *mut Virtqueue) {
    unsafe {
        let snd = (*(*vqueue).vdev).priv as *mut VirtioSnd;

        virtsnd_pcm_notify_cb(virtsnd_rx_queue(snd));
    }
}

/// Allocate and initialize the PCM device control message for the specified substream.
///
/// # Arguments
/// * `vss` - VirtIO PCM substream
/// * `command` - Control request code (VIRTIO_SND_R_PCM_XXX)
/// * `gfp` - Kernel flags for memory allocation
///
/// # Context
/// Any context. May sleep if gfp flags permit.
///
/// # Returns
/// Allocated message on success, NULL on failure.
pub fn virtsnd_pcm_ctl_msg_alloc(
    vss: *mut VirtioPcmSubstream,
    command: u32,
    gfp: u32,
) -> *mut VirtioSndMsg {
    unsafe {
        let mut request_size = core::mem::size_of::<VirtioSndPcmHdr>();
        let response_size = core::mem::size_of::<VirtioSndHdr>();

        match command {
            1 => {
                request_size = core::mem::size_of::<VirtioSndPcmSetParams>();
            }
            _ => {}
        }

        let msg = virtsnd_ctl_msg_alloc(request_size, response_size, gfp);
        if !msg.is_null() {
            let hdr = virtsnd_ctl_msg_request(msg) as *mut VirtioSndPcmHdr;

            (*hdr).hdr.code = cpu_to_le32(command);
            (*hdr).stream_id = cpu_to_le32((*vss).sid);
        }

        msg
    }
}

// External declarations
extern "C" {
    pub type VirtioPcmSubstream;
    pub type VirtioSndPcmXfer;
    pub type VirtioSndPcmStatus;
    pub type Scatterlist;
    pub type VirtioDevice;
    pub type Virtqueue;
    pub type VirtioSnd;
    pub type VirtioSndQueue;
    pub type VirtioSndMsg;
    pub type VirtioSndPcmHdr;
    pub type VirtioSndHdr;
    pub type VirtioSndPcmSetParams;
    pub type SndPcmRuntime;
    pub type SndPcmSubstream;
    pub type Page;

    pub static PAGE_SIZE: usize;

    pub fn vmalloc_to_page(addr: *mut u8) -> *mut Page;
    pub fn page_to_phys(page: *mut Page) -> usize;
    pub fn offset_in_page(addr: *mut u8) -> usize;
    pub fn sg_init_one(sg: *mut u8, buf: *mut u8, buflen: usize);
    pub fn sg_set_page(sg: *mut Scatterlist, page: *mut Page, len: u32, offset: u32);
    pub fn sg_mark_end(sg: *mut Scatterlist);
    pub fn sg_phys(sg: *const Scatterlist) -> usize;
    pub fn kzalloc_objs(ptr: *mut *mut VirtioPcmMsg, count: u32) -> *mut *mut VirtioPcmMsg;
    pub fn kzalloc_flex(size: usize, flex_count: i32) -> *mut u8;
    pub fn kfree(ptr: *mut u8);
    pub fn dev_err(dev: *mut u8, fmt: *const u8, ...);
    pub fn virtsnd_pcm_queue(vss: *mut VirtioPcmSubstream) -> *mut VirtioSndQueue;
    pub fn snd_pcm_lib_period_bytes(substream: *mut SndPcmSubstream) -> usize;
    pub fn cpu_to_le32(val: u32) -> u32;
    pub fn le32_to_cpu(val: u32) -> u32;
    pub fn virtqueue_add_sgs(
        vqueue: *mut Virtqueue,
        sgs: *mut *mut Scatterlist,
        out_sgs: usize,
        in_sgs: usize,
        data: *mut u8,
        gfp: u32,
    ) -> i32;
    pub fn virtqueue_disable_cb(vqueue: *mut Virtqueue);
    pub fn virtqueue_get_buf(vqueue: *mut Virtqueue, written_bytes: *mut u32) -> *mut u8;
    pub fn virtqueue_enable_cb(vqueue: *mut Virtqueue) -> i32;
    pub fn virtqueue_kick_prepare(vqueue: *mut Virtqueue) -> bool;
    pub fn virtqueue_notify(vqueue: *mut Virtqueue);
    pub fn spinlock_irqsave(lock: *mut u8) -> *mut u8;
    pub fn spinlock(lock: *mut u8) -> *mut u8;
    pub fn virtsnd_tx_queue(snd: *mut VirtioSnd) -> *mut VirtioSndQueue;
    pub fn virtsnd_rx_queue(snd: *mut VirtioSnd) -> *mut VirtioSndQueue;
    pub fn bytes_to_frames(runtime: *mut SndPcmRuntime, bytes: u32) -> u32;
    pub fn schedule_work(work: *mut u8);
    pub fn wake_up_all(wq: *mut u8);
    pub fn virtsnd_ctl_msg_alloc(
        request_size: usize,
        response_size: usize,
        gfp: u32,
    ) -> *mut VirtioSndMsg;
    pub fn virtsnd_ctl_msg_request(msg: *mut VirtioSndMsg) -> *mut u8;

    pub const SNDRV_PCM_STREAM_PLAYBACK: u32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
