// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2015 - 2021 Intel Corporation */

type Pf2vfBlkmsgDataGetterFn = unsafe extern "C" fn(*const u8, u8) -> u8;

static PF2VF_BLKMSG_PROVIDERS: [Option<adf_pf2vf_blkmsg_provider>; 4] = [
    None, /* no message type defined for value 0 */
    None, /* no message type defined for value 1 */
    Some(adf_pf_capabilities_msg_provider), /* ADF_VF2PF_BLKMSG_REQ_CAP_SUMMARY */
    Some(adf_pf_ring_to_svc_msg_provider),  /* ADF_VF2PF_BLKMSG_REQ_RING_SVC_MAP */
];

/**
 * adf_send_pf2vf_msg() - send PF to VF message
 * @accel_dev: Pointer to acceleration device
 * @vf_nr: VF number to which the message will be sent
 * @msg: Message to send
 *
 * This function allows the PF to send a message to a specific VF.
 *
 * Return: 0 on success, error code otherwise.
 */
pub unsafe fn adf_send_pf2vf_msg(
    accel_dev: *mut adf_accel_dev,
    vf_nr: u8,
    msg: pfvf_message,
) -> i32 {
    let pfvf_ops = GET_PFVF_OPS(accel_dev);
    let pfvf_offset = ((*pfvf_ops).get_pf2vf_offset)(vf_nr);

    ((*pfvf_ops).send_msg)(
        accel_dev,
        msg,
        pfvf_offset,
        &mut (*accel_dev).pf.vf_info[vf_nr as usize].pf2vf_lock,
    )
}

/**
 * adf_recv_vf2pf_msg() - receive a VF to PF message
 * @accel_dev: Pointer to acceleration device
 * @vf_nr: Number of the VF from where the message will be received
 *
 * This function allows the PF to receive a message from a specific VF.
 *
 * Return: a valid message on success, zero otherwise.
 */
unsafe fn adf_recv_vf2pf_msg(
    accel_dev: *mut adf_accel_dev,
    vf_nr: u8,
) -> pfvf_message {
    let vf_info = &mut (*accel_dev).pf.vf_info[vf_nr as usize];
    let pfvf_ops = GET_PFVF_OPS(accel_dev);
    let pfvf_offset = ((*pfvf_ops).get_vf2pf_offset)(vf_nr);

    ((*pfvf_ops).recv_msg)(accel_dev, pfvf_offset, vf_info.vf_compat_ver)
}

unsafe fn get_blkmsg_response_provider(
    msg_type: u8,
) -> Option<adf_pf2vf_blkmsg_provider> {
    if (msg_type as usize) >= PF2VF_BLKMSG_PROVIDERS.len() {
        return None;
    }

    PF2VF_BLKMSG_PROVIDERS[msg_type as usize]
}

/* Byte pf2vf_blkmsg_data_getter_fn callback */
unsafe extern "C" fn adf_pf2vf_blkmsg_get_byte(blkmsg: *const u8, index: u8) -> u8 {
    *blkmsg.add(index as usize)
}

/* CRC pf2vf_blkmsg_data_getter_fn callback */
unsafe extern "C" fn adf_pf2vf_blkmsg_get_crc(blkmsg: *const u8, count: u8) -> u8 {
    /* count is 0-based, turn it into a length */
    adf_pfvf_calc_blkmsg_crc(blkmsg, count.wrapping_add(1))
}

unsafe fn adf_pf2vf_blkmsg_get_data(
    vf_info: *mut adf_accel_vf_info,
    msg_type: u8,
    byte: u8,
    max_size: u8,
    data: *mut u8,
    data_getter: Pf2vfBlkmsgDataGetterFn,
) -> i32 {
    let mut blkmsg = [0u8; ADF_PFVF_BLKMSG_MSG_MAX_SIZE as usize];
    let accel_dev = (*vf_info).accel_dev;
    let provider = get_blkmsg_response_provider(msg_type);

    let provider = match provider {
        Some(provider) => provider,
        None => {
            pr_err!("QAT: No registered provider for message %d\n", msg_type);
            *data = ADF_PF2VF_INVALID_BLOCK_TYPE;
            return -EINVAL;
        }
    };

    if provider(accel_dev, blkmsg.as_mut_ptr(), (*vf_info).vf_compat_ver) != 0 {
        pr_err!("QAT: unknown error from provider for message %d\n", msg_type);
        *data = ADF_PF2VF_UNSPECIFIED_ERROR;
        return -EINVAL;
    }

    let msg_size = ADF_PFVF_BLKMSG_HEADER_SIZE + blkmsg[ADF_PFVF_BLKMSG_LEN_BYTE as usize];

    if msg_size >= max_size {
        pr_err!("QAT: Invalid size %d provided for message type %d\n", msg_size, msg_type);
        *data = ADF_PF2VF_PAYLOAD_TRUNCATED;
        return -EINVAL;
    }

    if byte >= msg_size {
        pr_err!("QAT: Out-of-bound byte number %d (msg size %d)\n", byte, msg_size);
        *data = ADF_PF2VF_INVALID_BYTE_NUM_REQ;
        return -EINVAL;
    }

    *data = data_getter(blkmsg.as_ptr(), byte);
    0
}

unsafe fn handle_blkmsg_req(
    vf_info: *mut adf_accel_vf_info,
    req: pfvf_message,
) -> pfvf_message {
    let mut resp_type = ADF_PF2VF_BLKMSG_RESP_TYPE_ERROR;
    let mut resp = pfvf_message { type_: 0, data: 0 };
    let mut resp_data = 0u8;
    let (blk_type, blk_byte, byte_max) = match req.type_ {
        ADF_VF2PF_MSGTYPE_LARGE_BLOCK_REQ => (
            FIELD_GET(ADF_VF2PF_LARGE_BLOCK_TYPE_MASK, req.data)
                + ADF_VF2PF_MEDIUM_BLOCK_TYPE_MAX + 1,
            FIELD_GET(ADF_VF2PF_LARGE_BLOCK_BYTE_MASK, req.data),
            ADF_VF2PF_LARGE_BLOCK_BYTE_MAX,
        ),
        ADF_VF2PF_MSGTYPE_MEDIUM_BLOCK_REQ => (
            FIELD_GET(ADF_VF2PF_MEDIUM_BLOCK_TYPE_MASK, req.data)
                + ADF_VF2PF_SMALL_BLOCK_TYPE_MAX + 1,
            FIELD_GET(ADF_VF2PF_MEDIUM_BLOCK_BYTE_MASK, req.data),
            ADF_VF2PF_MEDIUM_BLOCK_BYTE_MAX,
        ),
        ADF_VF2PF_MSGTYPE_SMALL_BLOCK_REQ => (
            FIELD_GET(ADF_VF2PF_SMALL_BLOCK_TYPE_MASK, req.data),
            FIELD_GET(ADF_VF2PF_SMALL_BLOCK_BYTE_MASK, req.data),
            ADF_VF2PF_SMALL_BLOCK_BYTE_MAX,
        ),
        _ => {
            dev_err!(&GET_DEV((*vf_info).accel_dev), "Invalid BlockMsg type 0x%.4x received from VF%u\n", req.type_, (*vf_info).vf_nr);
            return pfvf_message {
                type_: ADF_PF2VF_MSGTYPE_BLKMSG_RESP,
                data: FIELD_PREP(ADF_PF2VF_BLKMSG_RESP_TYPE_MASK, ADF_PF2VF_BLKMSG_RESP_TYPE_ERROR)
                    | FIELD_PREP(ADF_PF2VF_BLKMSG_RESP_DATA_MASK, ADF_PF2VF_UNSPECIFIED_ERROR),
            };
        }
    };

    if FIELD_GET(ADF_VF2PF_BLOCK_CRC_REQ_MASK, req.data) != 0 {
        dev_dbg!(&GET_DEV((*vf_info).accel_dev), "BlockMsg of type %d for CRC over %d bytes received from VF%d\n", blk_type, blk_byte + 1, (*vf_info).vf_nr);
        if adf_pf2vf_blkmsg_get_data(vf_info, blk_type, blk_byte, byte_max, &mut resp_data, adf_pf2vf_blkmsg_get_crc) == 0 {
            resp_type = ADF_PF2VF_BLKMSG_RESP_TYPE_CRC;
        }
    } else {
        dev_dbg!(&GET_DEV((*vf_info).accel_dev), "BlockMsg of type %d for data byte %d received from VF%d\n", blk_type, blk_byte, (*vf_info).vf_nr);
        if adf_pf2vf_blkmsg_get_data(vf_info, blk_type, blk_byte, byte_max, &mut resp_data, adf_pf2vf_blkmsg_get_byte) == 0 {
            resp_type = ADF_PF2VF_BLKMSG_RESP_TYPE_DATA;
        }
    }

    resp.type_ = ADF_PF2VF_MSGTYPE_BLKMSG_RESP;
    resp.data = FIELD_PREP(ADF_PF2VF_BLKMSG_RESP_TYPE_MASK, resp_type)
        | FIELD_PREP(ADF_PF2VF_BLKMSG_RESP_DATA_MASK, resp_data);
    resp
}

unsafe fn handle_rp_reset_req(
    accel_dev: *mut adf_accel_dev,
    vf_nr: u8,
    req: pfvf_message,
) -> pfvf_message {
    let hw_data = (*accel_dev).hw_device;
    let mut resp = pfvf_message { type_: ADF_PF2VF_MSGTYPE_RP_RESET_RESP, data: RPRESET_SUCCESS };
    let mut bank_number = FIELD_GET(ADF_VF2PF_RNG_RESET_RP_MASK, req.data);
    let rsvd_field = FIELD_GET(ADF_VF2PF_RNG_RESET_RSVD_MASK, req.data);

    dev_dbg!(&GET_DEV(accel_dev), "Ring Pair Reset Message received from VF%d for bank 0x%x\n", vf_nr, bank_number);
    if (*hw_data).ring_pair_reset.is_none() || rsvd_field != 0 {
        dev_dbg!(&GET_DEV(accel_dev), "Ring Pair Reset for VF%d is not supported\n", vf_nr);
        resp.data = RPRESET_NOT_SUPPORTED;
        return resp;
    }
    if bank_number >= (*hw_data).num_banks_per_vf {
        dev_err!(&GET_DEV(accel_dev), "Invalid bank number (0x%x) from VF%d for Ring Reset\n", bank_number, vf_nr);
        resp.data = RPRESET_INVAL_BANK;
        return resp;
    }
    bank_number = vf_nr as u32 * (*hw_data).num_banks_per_vf + bank_number;
    if ((*hw_data).ring_pair_reset.unwrap())(accel_dev, bank_number) != 0 {
        dev_dbg!(&GET_DEV(accel_dev), "Ring pair reset for VF%d failure\n", vf_nr);
        resp.data = RPRESET_TIMEOUT;
        return resp;
    }
    dev_dbg!(&GET_DEV(accel_dev), "Ring pair reset for VF%d successfully\n", vf_nr);
    resp
}

unsafe fn adf_handle_vf2pf_msg(
    accel_dev: *mut adf_accel_dev,
    vf_nr: u8,
    msg: pfvf_message,
    resp: *mut pfvf_message,
) -> i32 {
    let vf_info = &mut (*accel_dev).pf.vf_info[vf_nr as usize];
    match msg.type_ {
        ADF_VF2PF_MSGTYPE_COMPAT_VER_REQ => {
            let vf_compat_ver = msg.data;
            let compat = adf_vf_compat_checker(vf_compat_ver);
            dev_dbg!(&GET_DEV(accel_dev), "VersionRequest received from VF%d (vers %d) to PF (vers %d)\n", vf_nr, vf_compat_ver, ADF_PFVF_COMPAT_THIS_VERSION);
            vf_info.vf_compat_ver = vf_compat_ver;
            (*resp).type_ = ADF_PF2VF_MSGTYPE_VERSION_RESP;
            (*resp).data = FIELD_PREP(ADF_PF2VF_VERSION_RESP_VERS_MASK, ADF_PFVF_COMPAT_THIS_VERSION) | FIELD_PREP(ADF_PF2VF_VERSION_RESP_RESULT_MASK, compat);
        }
        ADF_VF2PF_MSGTYPE_VERSION_REQ => {
            dev_dbg!(&GET_DEV(accel_dev), "Legacy VersionRequest received from VF%d to PF (vers 1.1)\n", vf_nr);
            vf_info.vf_compat_ver = 0;
            (*resp).type_ = ADF_PF2VF_MSGTYPE_VERSION_RESP;
            (*resp).data = FIELD_PREP(ADF_PF2VF_VERSION_RESP_VERS_MASK, 0x11) | FIELD_PREP(ADF_PF2VF_VERSION_RESP_RESULT_MASK, ADF_PF2VF_VF_COMPATIBLE);
        }
        ADF_VF2PF_MSGTYPE_INIT => { dev_dbg!(&GET_DEV(accel_dev), "Init message received from VF%d\n", vf_nr); vf_info.init = true; }
        ADF_VF2PF_MSGTYPE_SHUTDOWN => { dev_dbg!(&GET_DEV(accel_dev), "Shutdown message received from VF%d\n", vf_nr); vf_info.init = false; }
        ADF_VF2PF_MSGTYPE_RESTARTING_COMPLETE => { dev_dbg!(&GET_DEV(accel_dev), "Restarting Complete received from VF%d\n", vf_nr); vf_info.restarting = false; vf_info.init = false; }
        ADF_VF2PF_MSGTYPE_LARGE_BLOCK_REQ | ADF_VF2PF_MSGTYPE_MEDIUM_BLOCK_REQ | ADF_VF2PF_MSGTYPE_SMALL_BLOCK_REQ => *resp = handle_blkmsg_req(vf_info, msg),
        ADF_VF2PF_MSGTYPE_RP_RESET => *resp = handle_rp_reset_req(accel_dev, vf_nr, msg),
        _ => { dev_dbg!(&GET_DEV(accel_dev), "Unknown message from VF%d (type 0x%.4x, data: 0x%.4x)\n", vf_nr, msg.type_, msg.data); return -ENOMSG; }
    }
    0
}

pub unsafe fn adf_recv_and_handle_vf2pf_msg(accel_dev: *mut adf_accel_dev, vf_nr: u32) -> bool {
    let req = adf_recv_vf2pf_msg(accel_dev, vf_nr as u8);
    if req.type_ == 0 { return true; }
    let mut resp = pfvf_message { type_: 0, data: 0 };
    if adf_handle_vf2pf_msg(accel_dev, vf_nr as u8, req, &mut resp) != 0 { return false; }
    if resp.type_ != 0 && adf_send_pf2vf_msg(accel_dev, vf_nr as u8, resp) != 0 {
        dev_err!(&GET_DEV(accel_dev), "Failed to send response to VF%d\n", vf_nr);
    }
    true
}

/**
 * adf_enable_pf2vf_comms() - Function enables communication from pf to vf
 *
 * @accel_dev: Pointer to acceleration device virtual function.
 *
 * This function carries out the necessary steps to setup and start the PFVF
 * communication channel, if any.
 *
 * Return: 0 on success, error code otherwise.
 */
pub unsafe fn adf_enable_pf2vf_comms(accel_dev: *mut adf_accel_dev) -> i32 {
    adf_pfvf_crc_init();
    spin_lock_init(&mut (*accel_dev).pf.vf2pf_ints_lock);
    0
}

// EXPORT_SYMBOL_GPL(adf_enable_pf2vf_comms);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
