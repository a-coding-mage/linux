// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram miXart soundcards
 *
 * low level interface with interrupt handling and mail box implementation
 *
 * Copyright (c) 2003 by Digigram <alsa@digigram.com>
 */

// C dependencies removed from executable Rust:
// <linux/interrupt.h>, <linux/mutex.h>, <linux/pci.h>, <linux/io.h>,
// <sound/core.h>, "mixart.h", "mixart_hwdep.h", "mixart_core.h".

pub const MSG_TIMEOUT_JIFFIES: c_long = (400 * HZ) / 1000; /* 400 ms */

pub const MSG_DESCRIPTOR_SIZE: u32 = 0x24;
pub const MSG_HEADER_SIZE: u32 = MSG_DESCRIPTOR_SIZE + 4;

pub const MSG_TYPE_MASK: u32 = 0x00000003; /* mask for following types */
pub const MSG_TYPE_NOTIFY: u32 = 0; /* embedded -> driver (only notification, do not get_msg() !) */
pub const MSG_TYPE_COMMAND: u32 = 1; /* driver <-> embedded (a command has no answer) */
pub const MSG_TYPE_REQUEST: u32 = 2; /* driver -> embedded (request will get an answer back) */
pub const MSG_TYPE_ANSWER: u32 = 3; /* embedded -> driver */
pub const MSG_CANCEL_NOTIFY_MASK: u32 = 0x80000000; /* this bit is set for a notification that has been canceled */

unsafe fn retrieve_msg_frame(mgr: *mut mixart_mgr, msg_frame: *mut u32) -> c_int {
    /* read the message frame fifo */
    let mut headptr: u32;
    let mut tailptr: u32;

    tailptr = readl_be(MIXART_MEM(mgr, MSG_OUTBOUND_POST_TAIL));
    headptr = readl_be(MIXART_MEM(mgr, MSG_OUTBOUND_POST_HEAD));

    if tailptr == headptr {
        return 0; /* no message posted */
    }

    if tailptr < MSG_OUTBOUND_POST_STACK {
        return 0; /* error */
    }
    if tailptr >= MSG_OUTBOUND_POST_STACK + MSG_BOUND_STACK_SIZE {
        return 0; /* error */
    }

    *msg_frame = readl_be(MIXART_MEM(mgr, tailptr));

    /* increment the tail index */
    tailptr = tailptr.wrapping_add(4);
    if tailptr >= MSG_OUTBOUND_POST_STACK + MSG_BOUND_STACK_SIZE {
        tailptr = MSG_OUTBOUND_POST_STACK;
    }
    writel_be(tailptr, MIXART_MEM(mgr, MSG_OUTBOUND_POST_TAIL));

    1
}

unsafe fn get_msg(mgr: *mut mixart_mgr, resp: *mut mixart_msg, msg_frame_address: u32) -> c_int {
    let mut headptr: u32;
    let mut size: u32;
    let mut err: c_int;

    err = 0;

    /* copy message descriptor from miXart to driver */
    size = readl_be(MIXART_MEM(mgr, msg_frame_address)); /* size of descriptor + response */
    (*resp).message_id = readl_be(MIXART_MEM(mgr, msg_frame_address + 4)); /* dwMessageID */
    (*resp).uid.object_id = readl_be(MIXART_MEM(mgr, msg_frame_address + 8)); /* uidDest */
    (*resp).uid.desc = readl_be(MIXART_MEM(mgr, msg_frame_address + 12)); /* */

    if size < MSG_DESCRIPTOR_SIZE || (*resp).size < size - MSG_DESCRIPTOR_SIZE {
        err = -EINVAL;
        dev_err(
            &mut (*(*mgr).pci).dev,
            c"problem with response size = %d\n".as_ptr(),
            size,
        );
        return err;
    }
    size -= MSG_DESCRIPTOR_SIZE;

    memcpy_fromio(
        (*resp).data as *mut c_void,
        MIXART_MEM(mgr, msg_frame_address + MSG_HEADER_SIZE),
        size as usize,
    );
    (*resp).size = size;

    /* swap if necessary */
    #[cfg(not(target_endian = "big"))]
    {
        size /= 4; /* u32 size */
        let mut i: c_uint = 0;
        while i < size {
            *((*resp).data as *mut u32).add(i as usize) =
                be32_to_cpu(*(((*resp).data as *mut __be32).add(i as usize)));
            i += 1;
        }
    }

    /*
     * free message frame address
     */
    headptr = readl_be(MIXART_MEM(mgr, MSG_OUTBOUND_FREE_HEAD));

    if headptr < MSG_OUTBOUND_FREE_STACK
        || headptr >= MSG_OUTBOUND_FREE_STACK + MSG_BOUND_STACK_SIZE
    {
        err = -EINVAL;
        return err;
    }

    /* give address back to outbound fifo */
    writel_be(msg_frame_address, MIXART_MEM(mgr, headptr));

    /* increment the outbound free head */
    headptr = headptr.wrapping_add(4);
    if headptr >= MSG_OUTBOUND_FREE_STACK + MSG_BOUND_STACK_SIZE {
        headptr = MSG_OUTBOUND_FREE_STACK;
    }

    writel_be(headptr, MIXART_MEM(mgr, MSG_OUTBOUND_FREE_HEAD));

    err
}

/*
 * send a message to miXart. return: the msg_frame used for this message
 */
/* call with mgr->msg_lock held! */
unsafe fn send_msg(
    mgr: *mut mixart_mgr,
    msg: *mut mixart_msg,
    max_answersize: c_int,
    mark_pending: c_int,
    msg_event: *mut u32,
) -> c_int {
    let mut headptr: u32;
    let mut tailptr: u32;
    let mut msg_frame_address: u32;
    let mut i: c_int;

    if snd_BUG_ON(((*msg).size % 4) != 0) != 0 {
        return -EINVAL;
    }

    /* get message frame address */
    tailptr = readl_be(MIXART_MEM(mgr, MSG_INBOUND_FREE_TAIL));
    headptr = readl_be(MIXART_MEM(mgr, MSG_INBOUND_FREE_HEAD));

    if tailptr == headptr {
        dev_err(
            &mut (*(*mgr).pci).dev,
            c"error: no message frame available\n".as_ptr(),
        );
        return -EBUSY;
    }

    if tailptr < MSG_INBOUND_FREE_STACK || tailptr >= MSG_INBOUND_FREE_STACK + MSG_BOUND_STACK_SIZE {
        return -EINVAL;
    }

    msg_frame_address = readl_be(MIXART_MEM(mgr, tailptr));
    writel(0, MIXART_MEM(mgr, tailptr)); /* set address to zero on this fifo position */

    /* increment the inbound free tail */
    tailptr = tailptr.wrapping_add(4);
    if tailptr >= MSG_INBOUND_FREE_STACK + MSG_BOUND_STACK_SIZE {
        tailptr = MSG_INBOUND_FREE_STACK;
    }

    writel_be(tailptr, MIXART_MEM(mgr, MSG_INBOUND_FREE_TAIL));

    /* TODO : use memcpy_toio() with intermediate buffer to copy the message */

    /* copy message descriptor to card memory */
    writel_be((*msg).size + MSG_DESCRIPTOR_SIZE, MIXART_MEM(mgr, msg_frame_address)); /* size of descriptor + request */
    writel_be((*msg).message_id, MIXART_MEM(mgr, msg_frame_address + 4)); /* dwMessageID */
    writel_be((*msg).uid.object_id, MIXART_MEM(mgr, msg_frame_address + 8)); /* uidDest */
    writel_be((*msg).uid.desc, MIXART_MEM(mgr, msg_frame_address + 12)); /* */
    writel_be(MSG_DESCRIPTOR_SIZE, MIXART_MEM(mgr, msg_frame_address + 16)); /* SizeHeader */
    writel_be(MSG_DESCRIPTOR_SIZE, MIXART_MEM(mgr, msg_frame_address + 20)); /* OffsetDLL_T16 */
    writel_be((*msg).size, MIXART_MEM(mgr, msg_frame_address + 24)); /* SizeDLL_T16 */
    writel_be(MSG_DESCRIPTOR_SIZE, MIXART_MEM(mgr, msg_frame_address + 28)); /* OffsetDLL_DRV */
    writel_be(0, MIXART_MEM(mgr, msg_frame_address + 32)); /* SizeDLL_DRV */
    writel_be(
        MSG_DESCRIPTOR_SIZE + max_answersize as u32,
        MIXART_MEM(mgr, msg_frame_address + 36),
    ); /* dwExpectedAnswerSize */

    /* copy message data to card memory */
    i = 0;
    while i < (*msg).size as c_int {
        writel_be(
            *((*msg).data.add(i as usize) as *mut u32),
            MIXART_MEM(mgr, MSG_HEADER_SIZE + msg_frame_address + i as u32),
        );
        i += 4;
    }

    if mark_pending != 0 {
        if *msg_event != 0 {
            /* the pending event is the notification we wait for ! */
            (*mgr).pending_event = *msg_event;
        } else {
            /* the pending event is the answer we wait for (same address than the request)! */
            (*mgr).pending_event = msg_frame_address;

            /* copy address back to caller */
            *msg_event = msg_frame_address;
        }
    }

    /* mark the frame as a request (will have an answer) */
    msg_frame_address |= MSG_TYPE_REQUEST;

    /* post the frame */
    headptr = readl_be(MIXART_MEM(mgr, MSG_INBOUND_POST_HEAD));

    if headptr < MSG_INBOUND_POST_STACK || headptr >= MSG_INBOUND_POST_STACK + MSG_BOUND_STACK_SIZE {
        return -EINVAL;
    }

    writel_be(msg_frame_address, MIXART_MEM(mgr, headptr));

    /* increment the inbound post head */
    headptr = headptr.wrapping_add(4);
    if headptr >= MSG_INBOUND_POST_STACK + MSG_BOUND_STACK_SIZE {
        headptr = MSG_INBOUND_POST_STACK;
    }

    writel_be(headptr, MIXART_MEM(mgr, MSG_INBOUND_POST_HEAD));

    0
}

pub unsafe extern "C" fn snd_mixart_send_msg(
    mgr: *mut mixart_mgr,
    request: *mut mixart_msg,
    max_resp_size: c_int,
    resp_data: *mut c_void,
) -> c_int {
    let mut resp: mixart_msg = core::mem::zeroed();
    let mut msg_frame: u32 = 0; /* set to 0, so it's no notification to wait for, but the answer */
    let mut err: c_int;
    let mut wait: wait_queue_entry_t = core::mem::zeroed();
    let mut timeout: c_long;

    init_waitqueue_entry(&mut wait, current);

    mutex_lock(&mut (*mgr).msg_lock);
    /* send the message */
    err = send_msg(mgr, request, max_resp_size, 1, &mut msg_frame); /* send and mark the answer pending */
    if err != 0 {
        mutex_unlock(&mut (*mgr).msg_lock);
        return err;
    }

    set_current_state(TASK_UNINTERRUPTIBLE);
    add_wait_queue(&mut (*mgr).msg_sleep, &mut wait);
    mutex_unlock(&mut (*mgr).msg_lock);

    timeout = schedule_timeout(MSG_TIMEOUT_JIFFIES);
    remove_wait_queue(&mut (*mgr).msg_sleep, &mut wait);

    if timeout == 0 {
        /* error - no ack */
        dev_err(
            &mut (*(*mgr).pci).dev,
            c"error: no response on msg %x\n".as_ptr(),
            msg_frame,
        );
        return -EIO;
    }

    /* retrieve the answer into the same struct mixart_msg */
    resp.message_id = 0;
    resp.uid = mixart_uid { object_id: 0, desc: 0 };
    resp.data = resp_data as *mut u32;
    resp.size = max_resp_size as u32;

    mutex_lock(&mut (*mgr).msg_lock);
    err = get_msg(mgr, &mut resp, msg_frame);
    mutex_unlock(&mut (*mgr).msg_lock);

    if (*request).message_id != resp.message_id {
        dev_err(&mut (*(*mgr).pci).dev, c"RESPONSE ERROR!\n".as_ptr());
    }

    err
}

pub unsafe extern "C" fn snd_mixart_send_msg_wait_notif(
    mgr: *mut mixart_mgr,
    request: *mut mixart_msg,
    mut notif_event: u32,
) -> c_int {
    let mut err: c_int;
    let mut wait: wait_queue_entry_t = core::mem::zeroed();
    let mut timeout: c_long;

    if snd_BUG_ON(notif_event == 0) != 0 {
        return -EINVAL;
    }
    if snd_BUG_ON((notif_event & MSG_TYPE_MASK) != MSG_TYPE_NOTIFY) != 0 {
        return -EINVAL;
    }
    if snd_BUG_ON((notif_event & MSG_CANCEL_NOTIFY_MASK) != 0) != 0 {
        return -EINVAL;
    }

    init_waitqueue_entry(&mut wait, current);

    mutex_lock(&mut (*mgr).msg_lock);
    /* send the message */
    err = send_msg(mgr, request, MSG_DEFAULT_SIZE as c_int, 1, &mut notif_event); /* send and mark the notification event pending */
    if err != 0 {
        mutex_unlock(&mut (*mgr).msg_lock);
        return err;
    }

    set_current_state(TASK_UNINTERRUPTIBLE);
    add_wait_queue(&mut (*mgr).msg_sleep, &mut wait);
    mutex_unlock(&mut (*mgr).msg_lock);

    timeout = schedule_timeout(MSG_TIMEOUT_JIFFIES);
    remove_wait_queue(&mut (*mgr).msg_sleep, &mut wait);

    if timeout == 0 {
        /* error - no ack */
        dev_err(
            &mut (*(*mgr).pci).dev,
            c"error: notification %x not received\n".as_ptr(),
            notif_event,
        );
        return -EIO;
    }

    0
}

pub unsafe extern "C" fn snd_mixart_send_msg_nonblock(
    mgr: *mut mixart_mgr,
    request: *mut mixart_msg,
) -> c_int {
    let mut message_frame: u32 = core::mem::zeroed();
    let err: c_int;

    /* just send the message (do not mark it as a pending one) */
    mutex_lock(&mut (*mgr).msg_lock);
    err = send_msg(mgr, request, MSG_DEFAULT_SIZE as c_int, 0, &mut message_frame);
    mutex_unlock(&mut (*mgr).msg_lock);

    /* the answer will be handled by snd_struct mixart_msgasklet()  */
    atomic_inc(&mut (*mgr).msg_processed);

    err
}

/* common buffer of interrupt to send/receive messages */
static mut mixart_msg_data: [u32; (MSG_DEFAULT_SIZE / 4) as usize] =
    [0; (MSG_DEFAULT_SIZE / 4) as usize];

unsafe fn snd_mixart_process_msg(mgr: *mut mixart_mgr) {
    let mut resp: mixart_msg = core::mem::zeroed();
    let mut msg: u32;
    let mut addr: u32;
    let mut type_: u32;
    let mut err: c_int;

    while (*mgr).msg_fifo_readptr != (*mgr).msg_fifo_writeptr {
        msg = (*mgr).msg_fifo[(*mgr).msg_fifo_readptr as usize];
        (*mgr).msg_fifo_readptr += 1;
        (*mgr).msg_fifo_readptr %= MSG_FIFO_SIZE;

        /* process the message ... */
        addr = msg & !MSG_TYPE_MASK;
        type_ = msg & MSG_TYPE_MASK;

        match type_ {
            MSG_TYPE_ANSWER => {
                /* answer to a message on that we did not wait for (send_msg_nonblock) */
                resp.message_id = 0;
                resp.data = mixart_msg_data.as_mut_ptr();
                resp.size = core::mem::size_of_val(&mixart_msg_data) as u32;
                err = get_msg(mgr, &mut resp, addr);
                if err < 0 {
                    dev_err(
                        &mut (*(*mgr).pci).dev,
                        c"error(%d) reading mf %x\n".as_ptr(),
                        err,
                        msg,
                    );
                    atomic_dec(&mut (*mgr).msg_processed);
                    continue;
                }

                match resp.message_id {
                    MSG_STREAM_START_INPUT_STAGE_PACKET
                    | MSG_STREAM_START_OUTPUT_STAGE_PACKET
                    | MSG_STREAM_STOP_INPUT_STAGE_PACKET
                    | MSG_STREAM_STOP_OUTPUT_STAGE_PACKET => {
                        if mixart_msg_data[0] != 0 {
                            dev_err(
                                &mut (*(*mgr).pci).dev,
                                c"error MSG_STREAM_ST***_***PUT_STAGE_PACKET status=%x\n".as_ptr(),
                                mixart_msg_data[0],
                            );
                        }
                    }
                    _ => {
                        dev_dbg(
                            &mut (*(*mgr).pci).dev,
                            c"received mf(%x) : msg_id(%x) uid(%x, %x) size(%zd)\n".as_ptr(),
                            msg,
                            resp.message_id,
                            resp.uid.object_id,
                            resp.uid.desc,
                            resp.size,
                        );
                    }
                }
            }
            MSG_TYPE_NOTIFY => {
                /* msg contains no address ! do not get_msg() ! */
                dev_err(
                    &mut (*(*mgr).pci).dev,
                    c"doesn't know what to do with message %x\n".as_ptr(),
                    msg,
                );
            }
            MSG_TYPE_COMMAND | _ => {
                /* get_msg() necessary */
                dev_err(
                    &mut (*(*mgr).pci).dev,
                    c"doesn't know what to do with message %x\n".as_ptr(),
                    msg,
                );
            }
        } /* switch type */

        /* decrement counter */
        atomic_dec(&mut (*mgr).msg_processed);
    } /* while there is a msg in fifo */
}

pub unsafe extern "C" fn snd_mixart_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let mgr: *mut mixart_mgr = dev_id as *mut mixart_mgr;
    let mut it_reg: u32;

    it_reg = readl_le(MIXART_REG(mgr, MIXART_PCI_OMISR_OFFSET));
    if (it_reg & MIXART_OIDI) == 0 {
        /* this device did not cause the interrupt */
        return IRQ_NONE;
    }

    /* mask all interrupts */
    writel_le(
        MIXART_HOST_ALL_INTERRUPT_MASKED,
        MIXART_REG(mgr, MIXART_PCI_OMIMR_OFFSET),
    );

    /* outdoorbell register clear */
    it_reg = readl(MIXART_REG(mgr, MIXART_PCI_ODBR_OFFSET));
    writel(it_reg, MIXART_REG(mgr, MIXART_PCI_ODBR_OFFSET));

    /* clear interrupt */
    writel_le(MIXART_OIDI, MIXART_REG(mgr, MIXART_PCI_OMISR_OFFSET));

    IRQ_WAKE_THREAD
}

pub unsafe extern "C" fn snd_mixart_threaded_irq(
    irq: c_int,
    dev_id: *mut c_void,
) -> irqreturn_t {
    let mgr: *mut mixart_mgr = dev_id as *mut mixart_mgr;
    let mut err: c_int;
    let mut resp: mixart_msg = core::mem::zeroed();
    let mut msg: u32 = 0;

    mutex_lock(&mut (*mgr).lock);
    /* process interrupt */
    while retrieve_msg_frame(mgr, &mut msg) != 0 {
        match msg & MSG_TYPE_MASK {
            MSG_TYPE_COMMAND => {
                resp.message_id = 0;
                resp.data = mixart_msg_data.as_mut_ptr();
                resp.size = core::mem::size_of_val(&mixart_msg_data) as u32;
                err = get_msg(mgr, &mut resp, msg & !MSG_TYPE_MASK);
                if err < 0 {
                    dev_err(
                        &mut (*(*mgr).pci).dev,
                        c"interrupt: error(%d) reading mf %x\n".as_ptr(),
                        err,
                        msg,
                    );
                    continue;
                }

                if resp.message_id == MSG_SERVICES_TIMER_NOTIFY {
                    let mut i: c_int;
                    let notify: *mut mixart_timer_notify;
                    notify = mixart_msg_data.as_mut_ptr() as *mut mixart_timer_notify;

                    BUILD_BUG_ON(core::mem::size_of_val(&notify) > core::mem::size_of_val(&mixart_msg_data));
                    if snd_BUG_ON((*notify).stream_count > ARRAY_SIZE((*notify).streams.as_ptr())) != 0 {
                        continue;
                    }
                    i = 0;
                    while i < (*notify).stream_count as c_int {
                        let buffer_id: u32 = (*notify).streams[i as usize].buffer_id;
                        let chip_number: c_uint =
                            ((buffer_id & MIXART_NOTIFY_CARD_MASK) >> MIXART_NOTIFY_CARD_OFFSET) as c_uint; /* card0 to 3 */
                        let pcm_number: c_uint =
                            ((buffer_id & MIXART_NOTIFY_PCM_MASK) >> MIXART_NOTIFY_PCM_OFFSET) as c_uint; /* pcm0 to 3  */
                        let sub_number: c_uint =
                            (buffer_id & MIXART_NOTIFY_SUBS_MASK) as c_uint; /* 0 to MIXART_PLAYBACK_STREAMS */
                        let is_capture: c_uint =
                            ((buffer_id & MIXART_NOTIFY_CAPT_MASK) != 0) as c_uint; /* playback == 0 / capture == 1 */

                        let chip: *mut snd_mixart = (*mgr).chip[chip_number as usize];
                        let stream: *mut mixart_stream;

                        if chip_number >= (*mgr).num_cards
                            || pcm_number >= MIXART_PCM_TOTAL
                            || sub_number >= MIXART_PLAYBACK_STREAMS
                        {
                            dev_err(
                                &mut (*(*mgr).pci).dev,
                                c"error MSG_SERVICES_TIMER_NOTIFY buffer_id (%x) pos(%d)\n".as_ptr(),
                                buffer_id,
                                (*notify).streams[i as usize].sample_pos_low_part,
                            );
                            break;
                        }

                        if is_capture != 0 {
                            stream = &mut (*chip).capture_stream[pcm_number as usize];
                        } else {
                            stream = &mut (*chip).playback_stream[pcm_number as usize][sub_number as usize];
                        }

                        if !(*stream).substream.is_null()
                            && (*stream).status == MIXART_STREAM_STATUS_RUNNING
                        {
                            let runtime: *mut snd_pcm_runtime = (*(*stream).substream).runtime;
                            let mut elapsed: c_int = 0;
                            let mut sample_count: u64 =
                                ((*notify).streams[i as usize].sample_pos_high_part as u64) << 32;
                            sample_count |= (*notify).streams[i as usize].sample_pos_low_part as u64;

                            loop {
                                let new_elapse_pos: u64 =
                                    (*stream).abs_period_elapsed + (*runtime).period_size;

                                if new_elapse_pos > sample_count {
                                    break; /* while */
                                } else {
                                    elapsed = 1;
                                    (*stream).buf_periods += 1;
                                    if (*stream).buf_periods >= (*runtime).periods {
                                        (*stream).buf_periods = 0;
                                    }

                                    (*stream).abs_period_elapsed = new_elapse_pos;
                                }
                            }
                            (*stream).buf_period_frag =
                                (sample_count - (*stream).abs_period_elapsed) as u32;

                            if elapsed != 0 {
                                mutex_unlock(&mut (*mgr).lock);
                                snd_pcm_period_elapsed((*stream).substream);
                                mutex_lock(&mut (*mgr).lock);
                            }
                        }
                        i += 1;
                    }
                    continue;
                }
                if resp.message_id == MSG_SERVICES_REPORT_TRACES {
                    if resp.size > 1 {
                        #[cfg(not(target_endian = "big"))]
                        {
                            /* Traces are text: the swapped msg_data has to be swapped back ! */
                            let mut i: c_int = 0;
                            while i < (resp.size / 4) as c_int {
                                *(mixart_msg_data.as_mut_ptr() as *mut __be32).add(i as usize) =
                                    cpu_to_be32(mixart_msg_data[i as usize]);
                                i += 1;
                            }
                        }
                        *(mixart_msg_data.as_mut_ptr() as *mut c_char).add(resp.size as usize - 1) = 0;
                        dev_dbg(
                            &mut (*(*mgr).pci).dev,
                            c"MIXART TRACE : %s\n".as_ptr(),
                            mixart_msg_data.as_mut_ptr() as *mut c_char,
                        );
                    }
                    continue;
                }

                dev_dbg(
                    &mut (*(*mgr).pci).dev,
                    c"command %x not handled\n".as_ptr(),
                    resp.message_id,
                );
            }
            MSG_TYPE_NOTIFY => {
                if (msg & MSG_CANCEL_NOTIFY_MASK) != 0 {
                    msg &= !MSG_CANCEL_NOTIFY_MASK;
                    dev_err(
                        &mut (*(*mgr).pci).dev,
                        c"canceled notification %x !\n".as_ptr(),
                        msg,
                    );
                }
                /* fallthrough */
                mutex_lock(&mut (*mgr).msg_lock);
                if (msg & !MSG_TYPE_MASK) == (*mgr).pending_event {
                    wake_up(&mut (*mgr).msg_sleep);
                    (*mgr).pending_event = 0;
                } else {
                    (*mgr).msg_fifo[(*mgr).msg_fifo_writeptr as usize] = msg;
                    (*mgr).msg_fifo_writeptr += 1;
                    (*mgr).msg_fifo_writeptr %= MSG_FIFO_SIZE;
                    snd_mixart_process_msg(mgr);
                }
                mutex_unlock(&mut (*mgr).msg_lock);
            }
            MSG_TYPE_ANSWER => {
                /* answer or notification to a message we are waiting for*/
                mutex_lock(&mut (*mgr).msg_lock);
                if (msg & !MSG_TYPE_MASK) == (*mgr).pending_event {
                    wake_up(&mut (*mgr).msg_sleep);
                    (*mgr).pending_event = 0;
                }
                /* answer to a message we did't want to wait for */
                else {
                    (*mgr).msg_fifo[(*mgr).msg_fifo_writeptr as usize] = msg;
                    (*mgr).msg_fifo_writeptr += 1;
                    (*mgr).msg_fifo_writeptr %= MSG_FIFO_SIZE;
                    snd_mixart_process_msg(mgr);
                }
                mutex_unlock(&mut (*mgr).msg_lock);
            }
            MSG_TYPE_REQUEST | _ => {
                dev_dbg(
                    &mut (*(*mgr).pci).dev,
                    c"interrupt received request %x\n".as_ptr(),
                    msg,
                );
                /* TODO : are there things to do here ? */
            }
        } /* switch on msg type */
    } /* while there are msgs */

    /* allow interrupt again */
    writel_le(
        MIXART_ALLOW_OUTBOUND_DOORBELL,
        MIXART_REG(mgr, MIXART_PCI_OMIMR_OFFSET),
    );

    mutex_unlock(&mut (*mgr).lock);
    IRQ_HANDLED
}

pub unsafe extern "C" fn snd_mixart_init_mailbox(mgr: *mut mixart_mgr) {
    writel(0, MIXART_MEM(mgr, MSG_HOST_RSC_PROTECTION));
    writel(0, MIXART_MEM(mgr, MSG_AGENT_RSC_PROTECTION));

    /* allow outbound messagebox to generate interrupts */
    if (*mgr).irq >= 0 {
        writel_le(
            MIXART_ALLOW_OUTBOUND_DOORBELL,
            MIXART_REG(mgr, MIXART_PCI_OMIMR_OFFSET),
        );
    }
    return;
}

pub unsafe extern "C" fn snd_mixart_exit_mailbox(mgr: *mut mixart_mgr) {
    /* no more interrupts on outbound messagebox */
    writel_le(
        MIXART_HOST_ALL_INTERRUPT_MASKED,
        MIXART_REG(mgr, MIXART_PCI_OMIMR_OFFSET),
    );
    return;
}

pub unsafe extern "C" fn snd_mixart_reset_board(mgr: *mut mixart_mgr) {
    /* reset miXart */
    writel_be(1, MIXART_REG(mgr, MIXART_BA1_BRUTAL_RESET_OFFSET));
    return;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
