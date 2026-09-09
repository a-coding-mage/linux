// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
/*
 * Copyright 2013-2016 Freescale Semiconductor Inc.
 * Copyright 2017-2018 NXP
 */

// Dependencies supplied by the surrounding repository: fsl_mc types,
// dpseci declarations, command structures, encoding helpers, and constants.

pub unsafe fn dpseci_open(mc_io: *mut fsl_mc_io, cmd_flags: u32, dpseci_id: i32,
                          token: *mut u16) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPSECI_CMDID_OPEN, cmd_flags, 0);
    let cmd_params = cmd.params.as_mut_ptr() as *mut dpseci_cmd_open;
    (*cmd_params).dpseci_id = cpu_to_le32(dpseci_id as u32);
    let err = mc_send_command(mc_io, &mut cmd);
    if err != 0 { return err; }
    *token = mc_cmd_hdr_read_token(&cmd);
    0
}

pub unsafe fn dpseci_close(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPSECI_CMDID_CLOSE, cmd_flags, token);
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe fn dpseci_enable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPSECI_CMDID_ENABLE, cmd_flags, token);
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe fn dpseci_disable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPSECI_CMDID_DISABLE, cmd_flags, token);
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe fn dpseci_reset(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPSECI_CMDID_RESET, cmd_flags, token);
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe fn dpseci_is_enabled(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16,
                                en: *mut i32) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPSECI_CMDID_IS_ENABLED, cmd_flags, token);
    let err = mc_send_command(mc_io, &mut cmd);
    if err != 0 { return err; }
    let rsp = &*(cmd.params.as_ptr() as *const dpseci_rsp_is_enabled);
    *en = dpseci_get_field(rsp.is_enabled, ENABLE) as i32;
    0
}

pub unsafe fn dpseci_get_attributes(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16,
                                    attr: *mut dpseci_attr) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPSECI_CMDID_GET_ATTR, cmd_flags, token);
    let err = mc_send_command(mc_io, &mut cmd);
    if err != 0 { return err; }
    let rsp = &*(cmd.params.as_ptr() as *const dpseci_rsp_get_attributes);
    (*attr).id = le32_to_cpu(rsp.id);
    (*attr).num_tx_queues = rsp.num_tx_queues;
    (*attr).num_rx_queues = rsp.num_rx_queues;
    (*attr).options = le32_to_cpu(rsp.options);
    0
}

pub unsafe fn dpseci_set_rx_queue(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16,
                                  queue: u8, cfg: *const dpseci_rx_queue_cfg) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPSECI_CMDID_SET_RX_QUEUE, cmd_flags, token);
    let p = &mut *(cmd.params.as_mut_ptr() as *mut dpseci_cmd_queue);
    p.dest_id = cpu_to_le32((*cfg).dest_cfg.dest_id);
    p.priority = (*cfg).dest_cfg.priority; p.queue = queue;
    dpseci_set_field(&mut p.dest_type, DEST_TYPE, (*cfg).dest_cfg.dest_type);
    p.user_ctx = cpu_to_le64((*cfg).user_ctx); p.options = cpu_to_le32((*cfg).options);
    dpseci_set_field(&mut p.order_preservation_en, ORDER_PRESERVATION, (*cfg).order_preservation_en);
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe fn dpseci_get_rx_queue(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16,
                                  queue: u8, attr: *mut dpseci_rx_queue_attr) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPSECI_CMDID_GET_RX_QUEUE, cmd_flags, token);
    let p = &mut *(cmd.params.as_mut_ptr() as *mut dpseci_cmd_queue); p.queue = queue;
    let err = mc_send_command(mc_io, &mut cmd); if err != 0 { return err; }
    (*attr).dest_cfg.dest_id = le32_to_cpu(p.dest_id);
    (*attr).dest_cfg.priority = p.priority;
    (*attr).dest_cfg.dest_type = dpseci_get_field(p.dest_type, DEST_TYPE);
    (*attr).user_ctx = le64_to_cpu(p.user_ctx); (*attr).fqid = le32_to_cpu(p.fqid);
    (*attr).order_preservation_en = dpseci_get_field(p.order_preservation_en, ORDER_PRESERVATION);
    0
}

pub unsafe fn dpseci_get_tx_queue(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16,
                                  queue: u8, attr: *mut dpseci_tx_queue_attr) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPSECI_CMDID_GET_TX_QUEUE, cmd_flags, token);
    let p = &mut *(cmd.params.as_mut_ptr() as *mut dpseci_cmd_queue); p.queue = queue;
    let err = mc_send_command(mc_io, &mut cmd); if err != 0 { return err; }
    let r = &*(cmd.params.as_ptr() as *const dpseci_rsp_get_tx_queue);
    (*attr).fqid = le32_to_cpu(r.fqid); (*attr).priority = r.priority; 0
}

pub unsafe fn dpseci_get_sec_attr(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16,
                                  attr: *mut dpseci_sec_attr) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPSECI_CMDID_GET_SEC_ATTR, cmd_flags, token);
    let err = mc_send_command(mc_io, &mut cmd); if err != 0 { return err; }
    let r = &*(cmd.params.as_ptr() as *const dpseci_rsp_get_sec_attr);
    (*attr).ip_id = le16_to_cpu(r.ip_id); (*attr).major_rev = r.major_rev; (*attr).minor_rev = r.minor_rev;
    (*attr).era = r.era; (*attr).deco_num = r.deco_num; (*attr).zuc_auth_acc_num = r.zuc_auth_acc_num;
    (*attr).zuc_enc_acc_num = r.zuc_enc_acc_num; (*attr).snow_f8_acc_num = r.snow_f8_acc_num;
    (*attr).snow_f9_acc_num = r.snow_f9_acc_num; (*attr).crc_acc_num = r.crc_acc_num;
    (*attr).pk_acc_num = r.pk_acc_num; (*attr).kasumi_acc_num = r.kasumi_acc_num; (*attr).rng_acc_num = r.rng_acc_num;
    (*attr).md_acc_num = r.md_acc_num; (*attr).arc4_acc_num = r.arc4_acc_num; (*attr).des_acc_num = r.des_acc_num;
    (*attr).aes_acc_num = r.aes_acc_num; (*attr).ccha_acc_num = r.ccha_acc_num; (*attr).ptha_acc_num = r.ptha_acc_num; 0
}

pub unsafe fn dpseci_get_api_version(mc_io: *mut fsl_mc_io, cmd_flags: u32, major_ver: *mut u16, minor_ver: *mut u16) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed(); cmd.header = mc_encode_cmd_header(DPSECI_CMDID_GET_API_VERSION, cmd_flags, 0);
    let err = mc_send_command(mc_io, &mut cmd); if err != 0 { return err; }
    let r = &*(cmd.params.as_ptr() as *const dpseci_rsp_get_api_version); *major_ver = le16_to_cpu(r.major); *minor_ver = le16_to_cpu(r.minor); 0
}

pub unsafe fn dpseci_set_congestion_notification(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, cfg: *const dpseci_congestion_notification_cfg) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed(); cmd.header = mc_encode_cmd_header(DPSECI_CMDID_SET_CONGESTION_NOTIFICATION, cmd_flags, token);
    let p = &mut *(cmd.params.as_mut_ptr() as *mut dpseci_cmd_congestion_notification);
    p.dest_id = cpu_to_le32((*cfg).dest_cfg.dest_id); p.notification_mode = cpu_to_le16((*cfg).notification_mode); p.priority = (*cfg).dest_cfg.priority;
    dpseci_set_field(&mut p.options, CGN_DEST_TYPE, (*cfg).dest_cfg.dest_type); dpseci_set_field(&mut p.options, CGN_UNITS, (*cfg).units);
    p.message_iova = cpu_to_le64((*cfg).message_iova); p.message_ctx = cpu_to_le64((*cfg).message_ctx); p.threshold_entry = cpu_to_le32((*cfg).threshold_entry); p.threshold_exit = cpu_to_le32((*cfg).threshold_exit);
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe fn dpseci_get_congestion_notification(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, cfg: *mut dpseci_congestion_notification_cfg) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed(); cmd.header = mc_encode_cmd_header(DPSECI_CMDID_GET_CONGESTION_NOTIFICATION, cmd_flags, token);
    let err = mc_send_command(mc_io, &mut cmd); if err != 0 { return err; }
    let r = &*(cmd.params.as_ptr() as *const dpseci_cmd_congestion_notification);
    (*cfg).dest_cfg.dest_id = le32_to_cpu(r.dest_id); (*cfg).notification_mode = le16_to_cpu(r.notification_mode); (*cfg).dest_cfg.priority = r.priority;
    (*cfg).dest_cfg.dest_type = dpseci_get_field(r.options, CGN_DEST_TYPE); (*cfg).units = dpseci_get_field(r.options, CGN_UNITS);
    (*cfg).message_iova = le64_to_cpu(r.message_iova); (*cfg).message_ctx = le64_to_cpu(r.message_ctx); (*cfg).threshold_entry = le32_to_cpu(r.threshold_entry); (*cfg).threshold_exit = le32_to_cpu(r.threshold_exit); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
