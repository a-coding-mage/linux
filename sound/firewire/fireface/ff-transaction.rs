// SPDX-License-Identifier: GPL-2.0-only
/*
 * ff-transaction.c - a part of driver for RME Fireface series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto
 */

// Rust translation of the implementation originally depending on "ff.h".
// External kernel/firewire/ALSA types, constants, macros, and helpers are
// expected to be supplied by the surrounding translated crate.

unsafe fn finish_transmit_midi_msg(ff: *mut snd_ff, port: c_uint, rcode: c_int) {
    let substream: *mut snd_rawmidi_substream =
        READ_ONCE((*ff).rx_midi_substreams[port as usize]);

    if rcode_is_permanent_error(rcode) {
        (*ff).rx_midi_error[port as usize] = true;
        return;
    }

    if rcode != RCODE_COMPLETE {
        /* Transfer the message again, immediately. */
        (*ff).next_ktime[port as usize] = 0;
        schedule_work(&mut (*ff).rx_midi_work[port as usize]);
        return;
    }

    snd_rawmidi_transmit_ack(substream, (*ff).rx_bytes[port as usize]);
    (*ff).rx_bytes[port as usize] = 0;

    if !snd_rawmidi_transmit_empty(substream) {
        schedule_work(&mut (*ff).rx_midi_work[port as usize]);
    }
}

unsafe extern "C" fn finish_transmit_midi0_msg(
    card: *mut fw_card,
    rcode: c_int,
    data: *mut c_void,
    length: size_t,
    callback_data: *mut c_void,
) {
    let ff: *mut snd_ff = container_of!(callback_data, snd_ff, transactions[0]);
    finish_transmit_midi_msg(ff, 0, rcode);
}

unsafe extern "C" fn finish_transmit_midi1_msg(
    card: *mut fw_card,
    rcode: c_int,
    data: *mut c_void,
    length: size_t,
    callback_data: *mut c_void,
) {
    let ff: *mut snd_ff = container_of!(callback_data, snd_ff, transactions[1]);
    finish_transmit_midi_msg(ff, 1, rcode);
}

unsafe fn transmit_midi_msg(ff: *mut snd_ff, port: c_uint) {
    let substream: *mut snd_rawmidi_substream =
        READ_ONCE((*ff).rx_midi_substreams[port as usize]);
    let mut quad_count: c_int;

    let fw_dev: *mut fw_device = fw_parent_device((*ff).unit);
    let addr: c_ulonglong;
    let generation: c_int;
    let callback: fw_transaction_callback_t;
    let tcode: c_int;

    if substream.is_null() || snd_rawmidi_transmit_empty(substream) {
        return;
    }

    if (*ff).rx_bytes[port as usize] > 0 || (*ff).rx_midi_error[port as usize] {
        return;
    }

    /* Do it in next chance. */
    if ktime_after((*ff).next_ktime[port as usize], ktime_get()) {
        schedule_work(&mut (*ff).rx_midi_work[port as usize]);
        return;
    }

    quad_count = ((*(*(*ff).spec).protocol).fill_midi_msg)(ff, substream, port);
    if quad_count <= 0 {
        return;
    }

    if port == 0 {
        addr = (*(*ff).spec).midi_rx_addrs[0];
        callback = Some(finish_transmit_midi0_msg);
    } else {
        addr = (*(*ff).spec).midi_rx_addrs[1];
        callback = Some(finish_transmit_midi1_msg);
    }

    /* Set interval to next transaction. */
    (*ff).next_ktime[port as usize] = ktime_add_ns(
        ktime_get(),
        (*ff).rx_bytes[port as usize] * 8 * (NSEC_PER_SEC / 31250),
    );

    if quad_count == 1 {
        tcode = TCODE_WRITE_QUADLET_REQUEST;
    } else {
        tcode = TCODE_WRITE_BLOCK_REQUEST;
    }

    /*
     * In Linux FireWire core, when generation is updated with memory
     * barrier, node id has already been updated. In this module, After
     * this smp_rmb(), load/store instructions to memory are completed.
     * Thus, both of generation and node id are available with recent
     * values. This is a light-serialization solution to handle bus reset
     * events on IEEE 1394 bus.
     */
    generation = (*fw_dev).generation;
    smp_rmb();
    fw_send_request(
        (*fw_dev).card,
        &mut (*ff).transactions[port as usize],
        tcode,
        (*fw_dev).node_id,
        generation,
        (*fw_dev).max_speed,
        addr,
        &mut (*ff).msg_buf[port as usize] as *mut _ as *mut c_void,
        quad_count * 4,
        callback,
        &mut (*ff).transactions[port as usize] as *mut _ as *mut c_void,
    );
}

unsafe extern "C" fn transmit_midi0_msg(work: *mut work_struct) {
    let ff: *mut snd_ff = container_of!(work, snd_ff, rx_midi_work[0]);

    transmit_midi_msg(ff, 0);
}

unsafe extern "C" fn transmit_midi1_msg(work: *mut work_struct) {
    let ff: *mut snd_ff = container_of!(work, snd_ff, rx_midi_work[1]);

    transmit_midi_msg(ff, 1);
}

unsafe extern "C" fn handle_msg(
    card: *mut fw_card,
    request: *mut fw_request,
    tcode: c_int,
    destination: c_int,
    source: c_int,
    generation: c_int,
    mut offset: c_ulonglong,
    data: *mut c_void,
    length: size_t,
    callback_data: *mut c_void,
) {
    let ff: *mut snd_ff = callback_data as *mut snd_ff;
    let buf: *mut __le32 = data as *mut __le32;
    let tstamp: u32 = fw_request_get_timestamp(request);

    fw_send_response(card, request, RCODE_COMPLETE);

    offset -= (*ff).async_handler.offset;

    let _guard = spinlock_irqsave_guard(&mut (*ff).lock);
    ((*(*(*ff).spec).protocol).handle_msg)(ff, offset as c_uint, buf, length, tstamp);
}

unsafe fn allocate_own_address(ff: *mut snd_ff, i: c_int) -> c_int {
    let mut midi_msg_region: fw_address_region = core::mem::zeroed();
    let mut err: c_int;

    (*ff).async_handler.length = (*(*ff).spec).midi_addr_range;
    (*ff).async_handler.address_callback = Some(handle_msg);
    (*ff).async_handler.callback_data = ff as *mut c_void;

    midi_msg_region.start = 0x000100000000u64 * i as u64;
    midi_msg_region.end = midi_msg_region.start + (*ff).async_handler.length;

    err = fw_core_add_address_handler(&mut (*ff).async_handler, &mut midi_msg_region);
    if err >= 0 {
        /* Controllers are allowed to register this region. */
        if ((*ff).async_handler.offset & 0x0000ffffffffu64) != 0 {
            fw_core_remove_address_handler(&mut (*ff).async_handler);
            err = -EAGAIN;
        }
    }

    err
}

// Controllers are allowed to register higher 4 bytes of destination address to
// receive asynchronous transactions for MIDI messages, while the way to
// register lower 4 bytes of address is different depending on protocols. For
// details, please refer to comments in protocol implementations.
//
// This driver expects userspace applications to configure registers for the
// lower address because in most cases such registers has the other settings.
pub unsafe extern "C" fn snd_ff_transaction_reregister(ff: *mut snd_ff) -> c_int {
    let fw_card: *mut fw_card = (*fw_parent_device((*ff).unit)).card;
    let addr: u32;
    let mut reg: __le32;

    /*
     * Controllers are allowed to register its node ID and upper 2 byte of
     * local address to listen asynchronous transactions.
     */
    addr = ((*fw_card).node_id << 16) | (((*ff).async_handler.offset >> 32) as u32);
    reg = cpu_to_le32(addr);
    snd_fw_transaction(
        (*ff).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        (*(*ff).spec).midi_high_addr,
        &mut reg as *mut _ as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    )
}

pub unsafe extern "C" fn snd_ff_transaction_register(ff: *mut snd_ff) -> c_int {
    let mut i: c_int;
    let mut err: c_int = 0;

    /*
     * Allocate in Memory Space of IEC 13213, but lower 4 byte in LSB should
     * be zero due to device specification.
     */
    i = 0;
    while i < 0xffff {
        err = allocate_own_address(ff, i);
        if err != -EBUSY && err != -EAGAIN {
            break;
        }
        i += 1;
    }
    if err < 0 {
        return err;
    }

    err = snd_ff_transaction_reregister(ff);
    if err < 0 {
        return err;
    }

    INIT_WORK(&mut (*ff).rx_midi_work[0], Some(transmit_midi0_msg));
    INIT_WORK(&mut (*ff).rx_midi_work[1], Some(transmit_midi1_msg));

    0
}

pub unsafe extern "C" fn snd_ff_transaction_unregister(ff: *mut snd_ff) {
    let mut reg: __le32;

    if (*ff).async_handler.callback_data.is_null() {
        return;
    }
    (*ff).async_handler.callback_data = core::ptr::null_mut();

    /* Release higher 4 bytes of address. */
    reg = cpu_to_le32(0x00000000);
    snd_fw_transaction(
        (*ff).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        (*(*ff).spec).midi_high_addr,
        &mut reg as *mut _ as *mut c_void,
        core::mem::size_of_val(&reg),
        0,
    );

    fw_core_remove_address_handler(&mut (*ff).async_handler);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
