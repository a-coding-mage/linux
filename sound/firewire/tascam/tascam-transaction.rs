// SPDX-License-Identifier: GPL-2.0-only
/*
 * tascam-transaction.c - a part of driver for TASCAM FireWire series
 *
 * Copyright (c) 2015 Takashi Sakamoto
 */

// Dependencies from "tascam.h" are expected to be supplied by the surrounding crate.

/*
 * When return minus value, given argument is not MIDI status.
 * When return 0, given argument is a beginning of system exclusive.
 * When return the others, given argument is MIDI data.
 */
#[inline]
unsafe fn calculate_message_bytes(status: u8) -> i32 {
    match status {
        0xf6 => 1, /* Tune request. */
        0xf8 => 1, /* Timing clock. */
        0xfa => 1, /* Start. */
        0xfb => 1, /* Continue. */
        0xfc => 1, /* Stop. */
        0xfe => 1, /* Active sensing. */
        0xff => 1, /* System reset. */
        0xf1 => 2, /* MIDI time code quarter frame. */
        0xf3 => 2, /* Song select. */
        0xf2 => 3, /* Song position pointer. */
        0xf0 => 0, /* Exclusive. */
        0xf7 => -EINVAL, /* End of exclusive. */
        0xf4 => -EINVAL, /* Undefined. */
        0xf5 => -EINVAL, /* Undefined. */
        0xf9 => -EINVAL, /* Undefined. */
        0xfd => -EINVAL, /* Undefined. */
        _ => match status & 0xf0 {
            0x80 => 3, /* Note on. */
            0x90 => 3, /* Note off. */
            0xa0 => 3, /* Polyphonic key pressure. */
            0xb0 => 3, /* Control change and Mode change. */
            0xe0 => 3, /* Pitch bend change. */
            0xc0 => 2, /* Program change. */
            0xd0 => 2, /* Channel pressure. */
            _ => -EINVAL,
        },
    }
}

unsafe fn fill_message(
    port: *mut snd_fw_async_midi_port,
    substream: *mut snd_rawmidi_substream,
) -> i32 {
    let mut i: i32;
    let mut len: i32;
    let mut consume: i32;
    let label: *mut u8;
    let msg: *mut u8;
    let mut status: u8;

    /* The first byte is used for label, the rest for MIDI bytes. */
    label = (*port).buf.as_mut_ptr();
    msg = (*port).buf.as_mut_ptr().add(1);

    consume = snd_rawmidi_transmit_peek(substream, msg as *mut _, 3);
    if consume == 0 {
        return 0;
    }

    /* On exclusive message. */
    if (*port).on_sysex {
        /* Seek the end of exclusives. */
        i = 0;
        while i < consume {
            if *msg.add(i as usize) == 0xf7 {
                (*port).on_sysex = false;
                break;
            }
            i += 1;
        }

        /* At the end of exclusive message, use label 0x07. */
        if !(*port).on_sysex {
            consume = i + 1;
            *label = (((*substream).number << 4) | 0x07) as u8;
        /* During exclusive message, use label 0x04. */
        } else if consume == 3 {
            *label = (((*substream).number << 4) | 0x04) as u8;
        /* We need to fill whole 3 bytes. Go to next change. */
        } else {
            return 0;
        }

        len = consume;
    } else {
        /* The beginning of exclusives. */
        if *msg.add(0) == 0xf0 {
            /* Transfer it in next chance in another condition. */
            (*port).on_sysex = true;
            return 0;
        } else {
            /* On running-status. */
            if (*msg.add(0) & 0x80) != 0x80 {
                status = (*port).running_status;
            } else {
                status = *msg.add(0);
            }

            /* Calculate consume bytes. */
            len = calculate_message_bytes(status);
            if len <= 0 {
                return 0;
            }

            /* On running-status. */
            if (*msg.add(0) & 0x80) != 0x80 {
                /* Enough MIDI bytes were not retrieved. */
                if consume < len - 1 {
                    return 0;
                }
                consume = len - 1;

                *msg.add(2) = *msg.add(1);
                *msg.add(1) = *msg.add(0);
                *msg.add(0) = (*port).running_status;
            } else {
                /* Enough MIDI bytes were not retrieved. */
                if consume < len {
                    return 0;
                }
                consume = len;

                (*port).running_status = *msg.add(0);
            }
        }

        *label = (((*substream).number << 4) | ((*msg.add(0) >> 4) as i32)) as u8;
    }

    if len > 0 && len < 3 {
        memset(
            msg.add(len as usize) as *mut _,
            0,
            (3 - len) as usize,
        );
    }

    consume
}

unsafe extern "C" fn async_midi_port_callback(
    _card: *mut fw_card,
    rcode: i32,
    _data: *mut core::ffi::c_void,
    _length: usize,
    callback_data: *mut core::ffi::c_void,
) {
    let port = callback_data as *mut snd_fw_async_midi_port;
    let substream: *mut snd_rawmidi_substream = READ_ONCE((*port).substream);

    /* This port is closed. */
    if substream.is_null() {
        return;
    }

    if rcode == RCODE_COMPLETE {
        snd_rawmidi_transmit_ack(substream, (*port).consume_bytes);
    } else if !rcode_is_permanent_error(rcode) {
        /* To start next transaction immediately for recovery. */
        (*port).next_ktime = 0;
    } else {
        /* Don't continue processing. */
        (*port).error = true;
    }

    (*port).idling = true;

    if !snd_rawmidi_transmit_empty(substream) {
        schedule_work(&mut (*port).work);
    }
}

unsafe extern "C" fn midi_port_work(work: *mut work_struct) {
    let port: *mut snd_fw_async_midi_port =
        container_of!(work, snd_fw_async_midi_port, work);
    let substream: *mut snd_rawmidi_substream = READ_ONCE((*port).substream);
    let generation: i32;

    /* Under transacting or error state. */
    if !(*port).idling || (*port).error {
        return;
    }

    /* Nothing to do. */
    if substream.is_null() || snd_rawmidi_transmit_empty(substream) {
        return;
    }

    /* Do it in next chance. */
    if ktime_after((*port).next_ktime, ktime_get()) {
        schedule_work(&mut (*port).work);
        return;
    }

    /*
     * Fill the buffer. The callee must use snd_rawmidi_transmit_peek().
     * Later, snd_rawmidi_transmit_ack() is called.
     */
    memset((*port).buf.as_mut_ptr() as *mut _, 0, 4);
    (*port).consume_bytes = fill_message(port, substream);
    if (*port).consume_bytes <= 0 {
        /* Do it in next chance, immediately. */
        if (*port).consume_bytes == 0 {
            (*port).next_ktime = 0;
            schedule_work(&mut (*port).work);
        } else {
            /* Fatal error. */
            (*port).error = true;
        }
        return;
    }

    /* Set interval to next transaction. */
    (*port).next_ktime = ktime_add_ns(
        ktime_get(),
        ((*port).consume_bytes * 8 * (NSEC_PER_SEC / 31250)) as _,
    );

    /* Start this transaction. */
    (*port).idling = false;

    /*
     * In Linux FireWire core, when generation is updated with memory
     * barrier, node id has already been updated. In this module, After
     * this smp_rmb(), load/store instructions to memory are completed.
     * Thus, both of generation and node id are available with recent
     * values. This is a light-serialization solution to handle bus reset
     * events on IEEE 1394 bus.
     */
    generation = (*(*port).parent).generation;
    smp_rmb();

    fw_send_request(
        (*(*port).parent).card,
        &mut (*port).transaction,
        TCODE_WRITE_QUADLET_REQUEST,
        (*(*port).parent).node_id,
        generation,
        (*(*port).parent).max_speed,
        TSCM_ADDR_BASE + TSCM_OFFSET_MIDI_RX_QUAD,
        (*port).buf.as_mut_ptr() as *mut _,
        4,
        Some(async_midi_port_callback),
        port as *mut _,
    );
}

#[no_mangle]
pub unsafe extern "C" fn snd_fw_async_midi_port_init(port: *mut snd_fw_async_midi_port) {
    (*port).idling = true;
    (*port).error = false;
    (*port).running_status = 0;
    (*port).on_sysex = false;
}

unsafe extern "C" fn handle_midi_tx(
    card: *mut fw_card,
    request: *mut fw_request,
    _tcode: i32,
    _destination: i32,
    _source: i32,
    _generation: i32,
    offset: u64,
    data: *mut core::ffi::c_void,
    length: usize,
    callback_data: *mut core::ffi::c_void,
) {
    let tscm = callback_data as *mut snd_tscm;
    let buf = data as *mut u32;
    let messages: u32;
    let mut i: u32;
    let mut port: u32;
    let mut substream: *mut snd_rawmidi_substream;
    let mut b: *mut u8;
    let mut bytes: i32;

    if offset != (*tscm).async_handler.offset {
        goto_end(card, request);
        return;
    }

    messages = (length / 8) as u32;
    i = 0;
    while i < messages {
        b = buf.add((i * 2) as usize) as *mut u8;

        port = (*b.add(0) >> 4) as u32;
        /* TODO: support virtual MIDI ports. */
        if port >= (*(*tscm).spec).midi_capture_ports {
            goto_end(card, request);
            return;
        }

        /* Assume the message length. */
        bytes = calculate_message_bytes(*b.add(1));
        /* On MIDI data or exclusives. */
        if bytes <= 0 {
            /* Seek the end of exclusives. */
            bytes = 1;
            while bytes < 4 {
                if *b.add(bytes as usize) == 0xf7 {
                    break;
                }
                bytes += 1;
            }
            if bytes == 4 {
                bytes = 3;
            }
        }

        substream = READ_ONCE((*tscm).tx_midi_substreams[port as usize]);
        if !substream.is_null() {
            snd_rawmidi_receive(substream, b.add(1) as *mut _, bytes);
        }

        i += 1;
    }

    goto_end(card, request);
}

unsafe fn goto_end(card: *mut fw_card, request: *mut fw_request) {
    fw_send_response(card, request, RCODE_COMPLETE);
}

#[no_mangle]
pub unsafe extern "C" fn snd_tscm_transaction_register(tscm: *mut snd_tscm) -> i32 {
    static RESP_REGISTER_REGION: fw_address_region = fw_address_region {
        start: 0xffffe0000000u64,
        end: 0xffffe000ffffu64,
    };
    let mut i: u32;
    let mut err: i32;

    /*
     * Usually, two quadlets are transferred by one transaction. The first
     * quadlet has MIDI messages, the rest includes timestamp.
     * Sometimes, 8 set of the data is transferred by a block transaction.
     */
    (*tscm).async_handler.length = 8 * 8;
    (*tscm).async_handler.address_callback = Some(handle_midi_tx);
    (*tscm).async_handler.callback_data = tscm as *mut _;

    err = fw_core_add_address_handler(&mut (*tscm).async_handler, &RESP_REGISTER_REGION);
    if err < 0 {
        return err;
    }

    err = snd_tscm_transaction_reregister(tscm);
    if err < 0 {
        fw_core_remove_address_handler(&mut (*tscm).async_handler);
        (*tscm).async_handler.callback_data = core::ptr::null_mut();
        return err;
    }

    i = 0;
    while i < TSCM_MIDI_OUT_PORT_MAX {
        (*tscm).out_ports[i as usize].parent = fw_parent_device((*tscm).unit);
        (*tscm).out_ports[i as usize].next_ktime = 0;
        INIT_WORK(&mut (*tscm).out_ports[i as usize].work, Some(midi_port_work));
        i += 1;
    }

    err
}

/* At bus reset, these registers are cleared. */
#[no_mangle]
pub unsafe extern "C" fn snd_tscm_transaction_reregister(tscm: *mut snd_tscm) -> i32 {
    let device: *mut fw_device = fw_parent_device((*tscm).unit);
    let mut reg: __be32;
    let mut err: i32;

    /* Register messaging address. Block transaction is not allowed. */
    reg = cpu_to_be32((((*(*device).card).node_id << 16) as u64
        | ((*tscm).async_handler.offset >> 32)) as u32);
    err = snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_MIDI_TX_ADDR_HI,
        &mut reg as *mut _ as *mut _,
        core::mem::size_of_val(&reg),
        0,
    );
    if err < 0 {
        return err;
    }

    reg = cpu_to_be32((*tscm).async_handler.offset as u32);
    err = snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_MIDI_TX_ADDR_LO,
        &mut reg as *mut _ as *mut _,
        core::mem::size_of_val(&reg),
        0,
    );
    if err < 0 {
        return err;
    }

    /* Turn on messaging. */
    reg = cpu_to_be32(0x00000001);
    err = snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_MIDI_TX_ON,
        &mut reg as *mut _ as *mut _,
        core::mem::size_of_val(&reg),
        0,
    );
    if err < 0 {
        return err;
    }

    /* Turn on FireWire LED. */
    reg = cpu_to_be32(0x0001008e);
    snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_LED_POWER,
        &mut reg as *mut _ as *mut _,
        core::mem::size_of_val(&reg),
        0,
    )
}

#[no_mangle]
pub unsafe extern "C" fn snd_tscm_transaction_unregister(tscm: *mut snd_tscm) {
    let mut reg: __be32;

    if (*tscm).async_handler.callback_data.is_null() {
        return;
    }

    /* Turn off FireWire LED. */
    reg = cpu_to_be32(0x0000008e);
    snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_LED_POWER,
        &mut reg as *mut _ as *mut _,
        core::mem::size_of_val(&reg),
        0,
    );

    /* Turn off messaging. */
    reg = cpu_to_be32(0x00000000);
    snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_MIDI_TX_ON,
        &mut reg as *mut _ as *mut _,
        core::mem::size_of_val(&reg),
        0,
    );

    /* Unregister the address. */
    snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_MIDI_TX_ADDR_HI,
        &mut reg as *mut _ as *mut _,
        core::mem::size_of_val(&reg),
        0,
    );
    snd_fw_transaction(
        (*tscm).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        TSCM_ADDR_BASE + TSCM_OFFSET_MIDI_TX_ADDR_LO,
        &mut reg as *mut _ as *mut _,
        core::mem::size_of_val(&reg),
        0,
    );

    fw_core_remove_address_handler(&mut (*tscm).async_handler);
    (*tscm).async_handler.callback_data = core::ptr::null_mut();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
