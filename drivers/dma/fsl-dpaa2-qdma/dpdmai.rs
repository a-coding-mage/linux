// SPDX-License-Identifier: GPL-2.0
// Copyright 2019 NXP

// C dependencies supplied by the surrounding kernel bindings.

const DEST_TYPE_MASK: u32 = 0xF;

#[repr(C)]
struct dpdmai_rsp_get_attributes {
    id: __le32,
    num_of_priorities: u8,
    num_of_queues: u8,
    pad0: [u8; 2],
    major: __le16,
    minor: __le16,
}

#[repr(C, packed)]
struct dpdmai_cmd_queue {
    dest_id: __le32,
    dest_priority: u8,
    queue_or_pri: dpdmai_cmd_queue_queue_or_pri,
    dest_type: u8,
    queue_idx: u8,
    user_ctx: __le64,
    options_or_fqid: dpdmai_cmd_queue_options_or_fqid,
}

#[repr(C)]
union dpdmai_cmd_queue_queue_or_pri {
    queue: u8,
    pri: u8,
}

#[repr(C)]
union dpdmai_cmd_queue_options_or_fqid {
    options: __le32,
    fqid: __le32,
}

#[repr(C)]
struct dpdmai_rsp_get_tx_queue {
    pad: __le64,
    fqid: __le32,
}

#[repr(C, packed)]
struct dpdmai_cmd_open {
    dpdmai_id: __le32,
}

#[repr(C, packed)]
struct dpdmai_cmd_destroy {
    dpdmai_id: __le32,
}

extern "C" {
    fn mc_encode_cmd_header(cmd_id: u16, cmd_flags: u32, token: u16) -> u64;
    fn mc_send_command(mc_io: *mut fsl_mc_io, cmd: *mut fsl_mc_command) -> i32;
    fn mc_cmd_hdr_read_token(cmd: *const fsl_mc_command) -> u16;
}

/// dpdmai_open() - Open a control session for the specified object
pub unsafe extern "C" fn dpdmai_open(
    mc_io: *mut fsl_mc_io,
    cmd_flags: u32,
    dpdmai_id: i32,
    token: *mut u16,
) -> i32 {
    let cmd_params: *mut dpdmai_cmd_open;
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    let err: i32;

    cmd.header = mc_encode_cmd_header(DPDMAI_CMDID_OPEN, cmd_flags, 0);
    cmd_params = cmd.params.as_mut_ptr() as *mut dpdmai_cmd_open;
    (*cmd_params).dpdmai_id = cpu_to_le32(dpdmai_id as u32);

    err = mc_send_command(mc_io, &mut cmd);
    if err != 0 {
        return err;
    }
    *token = mc_cmd_hdr_read_token(&cmd);
    0
}

pub unsafe extern "C" fn dpdmai_close(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPDMAI_CMDID_CLOSE, cmd_flags, token);
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe extern "C" fn dpdmai_destroy(
    mc_io: *mut fsl_mc_io, cmd_flags: u32, dpdmai_id: u32, token: u16,
) -> i32 {
    let cmd_params: *mut dpdmai_cmd_destroy;
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPDMAI_CMDID_DESTROY, cmd_flags, token);
    cmd_params = cmd.params.as_mut_ptr() as *mut dpdmai_cmd_destroy;
    (*cmd_params).dpdmai_id = cpu_to_le32(dpdmai_id);
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe extern "C" fn dpdmai_enable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPDMAI_CMDID_ENABLE, cmd_flags, token);
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe extern "C" fn dpdmai_disable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPDMAI_CMDID_DISABLE, cmd_flags, token);
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe extern "C" fn dpdmai_reset(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPDMAI_CMDID_RESET, cmd_flags, token);
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe extern "C" fn dpdmai_get_attributes(
    mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, attr: *mut dpdmai_attr,
) -> i32 {
    let rsp_params: *mut dpdmai_rsp_get_attributes;
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    let err: i32;
    cmd.header = mc_encode_cmd_header(DPDMAI_CMDID_GET_ATTR, cmd_flags, token);
    err = mc_send_command(mc_io, &mut cmd);
    if err != 0 { return err; }
    rsp_params = cmd.params.as_mut_ptr() as *mut dpdmai_rsp_get_attributes;
    (*attr).id = le32_to_cpu((*rsp_params).id);
    (*attr).version.major = le16_to_cpu((*rsp_params).major);
    (*attr).version.minor = le16_to_cpu((*rsp_params).minor);
    (*attr).num_of_priorities = (*rsp_params).num_of_priorities;
    (*attr).num_of_queues = (*rsp_params).num_of_queues;
    0
}

pub unsafe extern "C" fn dpdmai_set_rx_queue(
    mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, queue_idx: u8, priority: u8,
    cfg: *const dpdmai_rx_queue_cfg,
) -> i32 {
    let cmd_params: *mut dpdmai_cmd_queue;
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPDMAI_CMDID_SET_RX_QUEUE, cmd_flags, token);
    cmd_params = cmd.params.as_mut_ptr() as *mut dpdmai_cmd_queue;
    (*cmd_params).dest_id = cpu_to_le32((*cfg).dest_cfg.dest_id);
    (*cmd_params).dest_priority = (*cfg).dest_cfg.priority;
    (*cmd_params).queue_or_pri.pri = priority;
    (*cmd_params).dest_type = (*cfg).dest_cfg.dest_type;
    (*cmd_params).user_ctx = cpu_to_le64((*cfg).user_ctx);
    (*cmd_params).options_or_fqid.options = cpu_to_le32((*cfg).options);
    (*cmd_params).queue_idx = queue_idx;
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe extern "C" fn dpdmai_get_rx_queue(
    mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, queue_idx: u8, priority: u8,
    attr: *mut dpdmai_rx_queue_attr,
) -> i32 {
    let cmd_params: *mut dpdmai_cmd_queue;
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    let err: i32;
    cmd.header = mc_encode_cmd_header(DPDMAI_CMDID_GET_RX_QUEUE, cmd_flags, token);
    cmd_params = cmd.params.as_mut_ptr() as *mut dpdmai_cmd_queue;
    (*cmd_params).queue_or_pri.queue = priority;
    (*cmd_params).queue_idx = queue_idx;
    err = mc_send_command(mc_io, &mut cmd);
    if err != 0 { return err; }
    (*attr).dest_cfg.dest_id = le32_to_cpu((*cmd_params).dest_id);
    (*attr).dest_cfg.priority = (*cmd_params).dest_priority;
    (*attr).dest_cfg.dest_type = FIELD_GET(DEST_TYPE_MASK, (*cmd_params).dest_type as u32) as u8;
    (*attr).user_ctx = le64_to_cpu((*cmd_params).user_ctx);
    (*attr).fqid = le32_to_cpu((*cmd_params).options_or_fqid.fqid);
    0
}

pub unsafe extern "C" fn dpdmai_get_tx_queue(
    mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, queue_idx: u8, priority: u8,
    attr: *mut dpdmai_tx_queue_attr,
) -> i32 {
    let rsp_params: *mut dpdmai_rsp_get_tx_queue;
    let cmd_params: *mut dpdmai_cmd_queue;
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    let err: i32;
    cmd.header = mc_encode_cmd_header(DPDMAI_CMDID_GET_TX_QUEUE, cmd_flags, token);
    cmd_params = cmd.params.as_mut_ptr() as *mut dpdmai_cmd_queue;
    (*cmd_params).queue_or_pri.queue = priority;
    (*cmd_params).queue_idx = queue_idx;
    err = mc_send_command(mc_io, &mut cmd);
    if err != 0 { return err; }
    rsp_params = cmd.params.as_mut_ptr() as *mut dpdmai_rsp_get_tx_queue;
    (*attr).fqid = le32_to_cpu((*rsp_params).fqid);
    0
}

// MODULE_DESCRIPTION("NXP DPAA2 QDMA driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
