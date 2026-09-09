// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2015 - 2021 Intel Corporation */

// Linux headers and local headers from the C translation unit provide the
// types, constants, macros, and external functions referenced below.

const ADF_PFVF_MSG_COLLISION_DETECT_DELAY: u32 = 10;
const ADF_PFVF_MSG_ACK_DELAY: u32 = 2;
const ADF_PFVF_MSG_ACK_MAX_RETRY: u32 = 100;
const ADF_PFVF_MSG_RESP_RETRIES: u32 = 5;
const ADF_PFVF_MSG_RESP_TIMEOUT: u32 = ADF_PFVF_MSG_ACK_DELAY * ADF_PFVF_MSG_ACK_MAX_RETRY
    + ADF_PFVF_MSG_COLLISION_DETECT_DELAY;

pub unsafe fn adf_send_vf2pf_msg(
    accel_dev: *mut adf_accel_dev,
    msg: pfvf_message,
) -> i32 {
    let pfvf_ops = GET_PFVF_OPS(accel_dev);
    let pfvf_offset = ((*pfvf_ops).get_vf2pf_offset)(0);

    ((*pfvf_ops).send_msg)(
        accel_dev,
        msg,
        pfvf_offset,
        &mut (*accel_dev).vf.vf2pf_lock,
    )
}

unsafe fn adf_recv_pf2vf_msg(accel_dev: *mut adf_accel_dev) -> pfvf_message {
    let pfvf_ops = GET_PFVF_OPS(accel_dev);
    let pfvf_offset = ((*pfvf_ops).get_pf2vf_offset)(0);

    ((*pfvf_ops).recv_msg)(accel_dev, pfvf_offset, (*accel_dev).vf.pf_compat_ver)
}

pub unsafe fn adf_send_vf2pf_req(
    accel_dev: *mut adf_accel_dev,
    msg: pfvf_message,
    resp: *mut pfvf_message,
) -> i32 {
    let timeout = msecs_to_jiffies(ADF_PFVF_MSG_RESP_TIMEOUT);
    let mut retries = ADF_PFVF_MSG_RESP_RETRIES;

    reinit_completion(&mut (*accel_dev).vf.msg_received);

    loop {
        let ret = adf_send_vf2pf_msg(accel_dev, msg);
        if ret != 0 {
            dev_err(&GET_DEV(accel_dev), "Failed to send request msg to PF\n");
            return ret;
        }

        let ret = wait_for_completion_timeout(&mut (*accel_dev).vf.msg_received, timeout);
        if ret != 0 {
            if !resp.is_null() {
                *resp = (*accel_dev).vf.response;
            }
            (*accel_dev).vf.response.type_ = 0;
            return 0;
        }

        dev_err(&GET_DEV(accel_dev), "PFVF response message timeout\n");
        retries -= 1;
        if retries == 0 {
            break;
        }
    }

    -EIO
}

unsafe fn adf_vf2pf_blkmsg_data_req(
    accel_dev: *mut adf_accel_dev,
    crc: bool,
    type_: *mut u8,
    data: *mut u8,
) -> i32 {
    let mut req: pfvf_message = core::mem::zeroed();
    let mut resp: pfvf_message = core::mem::zeroed();
    let (msg_type, blk_type, blk_byte, max_data);

    if *type_ <= ADF_VF2PF_SMALL_BLOCK_TYPE_MAX {
        msg_type = ADF_VF2PF_MSGTYPE_SMALL_BLOCK_REQ;
        blk_type = FIELD_PREP(ADF_VF2PF_SMALL_BLOCK_TYPE_MASK, *type_);
        blk_byte = FIELD_PREP(ADF_VF2PF_SMALL_BLOCK_BYTE_MASK, *data);
        max_data = ADF_VF2PF_SMALL_BLOCK_BYTE_MAX;
    } else if *type_ <= ADF_VF2PF_MEDIUM_BLOCK_TYPE_MAX {
        msg_type = ADF_VF2PF_MSGTYPE_MEDIUM_BLOCK_REQ;
        blk_type = FIELD_PREP(ADF_VF2PF_MEDIUM_BLOCK_TYPE_MASK,
                              *type_ - ADF_VF2PF_SMALL_BLOCK_TYPE_MAX);
        blk_byte = FIELD_PREP(ADF_VF2PF_MEDIUM_BLOCK_BYTE_MASK, *data);
        max_data = ADF_VF2PF_MEDIUM_BLOCK_BYTE_MAX;
    } else if *type_ <= ADF_VF2PF_LARGE_BLOCK_TYPE_MAX {
        msg_type = ADF_VF2PF_MSGTYPE_LARGE_BLOCK_REQ;
        blk_type = FIELD_PREP(ADF_VF2PF_LARGE_BLOCK_TYPE_MASK,
                              *type_ - ADF_VF2PF_MEDIUM_BLOCK_TYPE_MAX);
        blk_byte = FIELD_PREP(ADF_VF2PF_LARGE_BLOCK_BYTE_MASK, *data);
        max_data = ADF_VF2PF_LARGE_BLOCK_BYTE_MAX;
    } else {
        dev_err(&GET_DEV(accel_dev), "Invalid message type %u\n", *type_);
        return -EINVAL;
    }

    if *data > max_data {
        dev_err(&GET_DEV(accel_dev), "Invalid byte %s %u for message type %u\n",
                if crc { "count" } else { "index" }, *data, *type_);
        return -EINVAL;
    }

    req.type_ = msg_type;
    req.data = blk_type | blk_byte | FIELD_PREP(ADF_VF2PF_BLOCK_CRC_REQ_MASK, crc);

    let err = adf_send_vf2pf_req(accel_dev, req, &mut resp);
    if err != 0 {
        return err;
    }

    *type_ = FIELD_GET(ADF_PF2VF_BLKMSG_RESP_TYPE_MASK, resp.data);
    *data = FIELD_GET(ADF_PF2VF_BLKMSG_RESP_DATA_MASK, resp.data);
    0
}

unsafe fn adf_vf2pf_blkmsg_get_byte(
    accel_dev: *mut adf_accel_dev, type_: u8, index: u8, data: *mut u8,
) -> i32 {
    let mut type_ = type_;
    let mut index = index;
    let ret = adf_vf2pf_blkmsg_data_req(accel_dev, false, &mut type_, &mut index);
    if ret < 0 { return ret; }
    if type_ != ADF_PF2VF_BLKMSG_RESP_TYPE_DATA {
        dev_err(&GET_DEV(accel_dev), "Unexpected BLKMSG response type %u, byte 0x%x\n", type_, index);
        return -EFAULT;
    }
    *data = index;
    0
}

unsafe fn adf_vf2pf_blkmsg_get_crc(
    accel_dev: *mut adf_accel_dev, type_: u8, mut bytes: u8, crc: *mut u8,
) -> i32 {
    bytes = bytes.wrapping_sub(1);
    let mut type_ = type_;
    let ret = adf_vf2pf_blkmsg_data_req(accel_dev, true, &mut type_, &mut bytes);
    if ret < 0 { return ret; }
    if type_ != ADF_PF2VF_BLKMSG_RESP_TYPE_CRC {
        dev_err(&GET_DEV(accel_dev), "Unexpected CRC BLKMSG response type %u, crc 0x%x\n", type_, bytes);
        return -EFAULT;
    }
    *crc = bytes;
    0
}

pub unsafe fn adf_send_vf2pf_blkmsg_req(
    accel_dev: *mut adf_accel_dev, type_: u8, buffer: *mut u8, buffer_len: *mut u32,
) -> i32 {
    if type_ > ADF_VF2PF_LARGE_BLOCK_TYPE_MAX { return -EINVAL; }
    if *buffer_len < ADF_PFVF_BLKMSG_HEADER_SIZE { return -EINVAL; }

    let mut ret = adf_vf2pf_blkmsg_get_byte(accel_dev, type_, ADF_PFVF_BLKMSG_VER_BYTE,
                                             buffer.add(ADF_PFVF_BLKMSG_VER_BYTE as usize));
    if ret != 0 { return ret; }
    if *buffer.add(ADF_PFVF_BLKMSG_VER_BYTE as usize) == 0 { return -EFAULT; }

    ret = adf_vf2pf_blkmsg_get_byte(accel_dev, type_, ADF_PFVF_BLKMSG_LEN_BYTE,
                                     buffer.add(ADF_PFVF_BLKMSG_LEN_BYTE as usize));
    if ret != 0 { return ret; }
    if *buffer.add(ADF_PFVF_BLKMSG_LEN_BYTE as usize) == 0 { return -EFAULT; }

    let mut msg_len = ADF_PFVF_BLKMSG_HEADER_SIZE +
        *buffer.add(ADF_PFVF_BLKMSG_LEN_BYTE as usize) as u32;
    msg_len = core::cmp::min(*buffer_len, msg_len);
    for index in ADF_PFVF_BLKMSG_HEADER_SIZE..msg_len {
        ret = adf_vf2pf_blkmsg_get_byte(accel_dev, type_, index as u8, buffer.add(index as usize));
        if ret != 0 { return ret; }
    }

    let mut remote_crc = 0u8;
    ret = adf_vf2pf_blkmsg_get_crc(accel_dev, type_, msg_len as u8, &mut remote_crc);
    if ret != 0 { return ret; }
    let local_crc = adf_pfvf_calc_blkmsg_crc(buffer, msg_len);
    if local_crc != remote_crc { return -EIO; }
    *buffer_len = msg_len;
    0
}

unsafe fn adf_handle_pf2vf_msg(accel_dev: *mut adf_accel_dev, msg: pfvf_message) -> bool {
    match msg.type_ {
        ADF_PF2VF_MSGTYPE_RESTARTING => { adf_pf2vf_handle_pf_restarting(accel_dev); false }
        ADF_PF2VF_MSGTYPE_RESTARTED | ADF_PF2VF_MSGTYPE_FATAL_ERROR => true,
        ADF_PF2VF_MSGTYPE_VERSION_RESP | ADF_PF2VF_MSGTYPE_BLKMSG_RESP |
        ADF_PF2VF_MSGTYPE_RP_RESET_RESP => {
            (*accel_dev).vf.response = msg;
            complete(&mut (*accel_dev).vf.msg_received);
            true
        }
        _ => false,
    }
}

pub unsafe fn adf_recv_and_handle_pf2vf_msg(accel_dev: *mut adf_accel_dev) -> bool {
    let msg = adf_recv_pf2vf_msg(accel_dev);
    if msg.type_ != 0 { return adf_handle_pf2vf_msg(accel_dev, msg); }
    true
}

pub unsafe fn adf_enable_vf2pf_comms(accel_dev: *mut adf_accel_dev) -> i32 {
    adf_pfvf_crc_init();
    adf_enable_pf2vf_interrupts(accel_dev);
    let mut ret = adf_vf2pf_request_version(accel_dev);
    if ret != 0 { return ret; }
    ret = adf_vf2pf_get_capabilities(accel_dev);
    if ret != 0 { return ret; }
    adf_vf2pf_get_ring_to_svc(accel_dev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
