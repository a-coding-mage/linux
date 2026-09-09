// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
/*
 * Copyright 2013-2016 Freescale Semiconductor Inc.
 * Copyright 2020 NXP
 */

// Dependencies supplied by the surrounding kernel translation.

static mut dprc_major_ver: u16 = 0;
static mut dprc_minor_ver: u16 = 0;

pub unsafe fn dprc_open(mc_io: *mut fsl_mc_io, cmd_flags: u32, container_id: i32, token: *mut u16) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    let cmd_params = cmd.params.as_mut_ptr() as *mut dprc_cmd_open;
    let mut err: i32;
    cmd.header = mc_encode_cmd_header(DPRC_CMDID_OPEN, cmd_flags, 0);
    (*cmd_params).container_id = cpu_to_le32(container_id as u32);
    err = mc_send_command(mc_io, &mut cmd);
    if err != 0 { return err; }
    *token = mc_cmd_hdr_read_token(&cmd);
    0
}

pub unsafe fn dprc_close(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    cmd.header = mc_encode_cmd_header(DPRC_CMDID_CLOSE, cmd_flags, token);
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe fn dprc_reset_container(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, child_container_id: i32, options: u32) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    let cmd_params: *mut dprc_cmd_reset_container;
    let mut cmdid = DPRC_CMDID_RESET_CONT;
    let err: i32;
    if dprc_major_ver == 0 && dprc_minor_ver == 0 {
        err = dprc_get_api_version(mc_io, 0, &mut dprc_major_ver, &mut dprc_minor_ver);
        if err != 0 { return err; }
    }
    if dprc_major_ver > 6 || (dprc_major_ver == 6 && dprc_minor_ver >= 5) { cmdid = DPRC_CMDID_RESET_CONT_V2; }
    cmd.header = mc_encode_cmd_header(cmdid, cmd_flags, token);
    cmd_params = cmd.params.as_mut_ptr() as *mut dprc_cmd_reset_container;
    (*cmd_params).child_container_id = cpu_to_le32(child_container_id as u32);
    (*cmd_params).options = cpu_to_le32(options);
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe fn dprc_set_irq(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, irq_index: u8, irq_cfg: *const dprc_irq_cfg) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    let p = cmd.params.as_mut_ptr() as *mut dprc_cmd_set_irq;
    cmd.header = mc_encode_cmd_header(DPRC_CMDID_SET_IRQ, cmd_flags, token);
    (*p).irq_val = cpu_to_le32((*irq_cfg).val); (*p).irq_index = irq_index;
    (*p).irq_addr = cpu_to_le64((*irq_cfg).paddr); (*p).irq_num = cpu_to_le32((*irq_cfg).irq_num);
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe fn dprc_set_irq_enable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, irq_index: u8, en: u8) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    let p = cmd.params.as_mut_ptr() as *mut dprc_cmd_set_irq_enable;
    cmd.header = mc_encode_cmd_header(DPRC_CMDID_SET_IRQ_ENABLE, cmd_flags, token);
    (*p).enable = en & DPRC_ENABLE; (*p).irq_index = irq_index;
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe fn dprc_set_irq_mask(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, irq_index: u8, mask: u32) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    let p = cmd.params.as_mut_ptr() as *mut dprc_cmd_set_irq_mask;
    cmd.header = mc_encode_cmd_header(DPRC_CMDID_SET_IRQ_MASK, cmd_flags, token);
    (*p).mask = cpu_to_le32(mask); (*p).irq_index = irq_index;
    mc_send_command(mc_io, &mut cmd)
}

pub unsafe fn dprc_get_irq_status(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, irq_index: u8, status: *mut u32) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed();
    let p = cmd.params.as_mut_ptr() as *mut dprc_cmd_get_irq_status;
    cmd.header = mc_encode_cmd_header(DPRC_CMDID_GET_IRQ_STATUS, cmd_flags, token);
    (*p).status = cpu_to_le32(*status); (*p).irq_index = irq_index;
    let err = mc_send_command(mc_io, &mut cmd); if err != 0 { return err; }
    let r = cmd.params.as_ptr() as *const dprc_rsp_get_irq_status; *status = le32_to_cpu((*r).status); 0
}

pub unsafe fn dprc_clear_irq_status(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, irq_index: u8, status: u32) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed(); let p = cmd.params.as_mut_ptr() as *mut dprc_cmd_clear_irq_status;
    cmd.header = mc_encode_cmd_header(DPRC_CMDID_CLEAR_IRQ_STATUS, cmd_flags, token); (*p).status = cpu_to_le32(status); (*p).irq_index = irq_index; mc_send_command(mc_io, &mut cmd)
}

pub unsafe fn dprc_get_attributes(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, attr: *mut dprc_attributes) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed(); cmd.header = mc_encode_cmd_header(DPRC_CMDID_GET_ATTR, cmd_flags, token);
    let err = mc_send_command(mc_io, &mut cmd); if err != 0 { return err; }
    let r = cmd.params.as_ptr() as *const dprc_rsp_get_attributes; (*attr).container_id = le32_to_cpu((*r).container_id); (*attr).icid = le32_to_cpu((*r).icid); (*attr).options = le32_to_cpu((*r).options); (*attr).portal_id = le32_to_cpu((*r).portal_id); 0
}

pub unsafe fn dprc_get_obj_count(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, obj_count: *mut i32) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed(); cmd.header = mc_encode_cmd_header(DPRC_CMDID_GET_OBJ_COUNT, cmd_flags, token);
    let err = mc_send_command(mc_io, &mut cmd); if err != 0 { return err; } let r = cmd.params.as_ptr() as *const dprc_rsp_get_obj_count; *obj_count = le32_to_cpu((*r).obj_count) as i32; 0
}

pub unsafe fn dprc_get_obj(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, obj_index: i32, obj_desc: *mut fsl_mc_obj_desc) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed(); let p = cmd.params.as_mut_ptr() as *mut dprc_cmd_get_obj;
    cmd.header = mc_encode_cmd_header(DPRC_CMDID_GET_OBJ, cmd_flags, token); (*p).obj_index = cpu_to_le32(obj_index as u32);
    let err = mc_send_command(mc_io, &mut cmd); if err != 0 { return err; } let r = cmd.params.as_ptr() as *const dprc_rsp_get_obj;
    (*obj_desc).id = le32_to_cpu((*r).id); (*obj_desc).vendor = le16_to_cpu((*r).vendor); (*obj_desc).irq_count = (*r).irq_count; (*obj_desc).region_count = (*r).region_count; (*obj_desc).state = le32_to_cpu((*r).state); (*obj_desc).ver_major = le16_to_cpu((*r).version_major); (*obj_desc).ver_minor = le16_to_cpu((*r).version_minor); (*obj_desc).flags = le16_to_cpu((*r).flags); strscpy_pad((*obj_desc).type.as_mut_ptr(), (*r).type.as_ptr(), 16); strscpy_pad((*obj_desc).label.as_mut_ptr(), (*r).label.as_ptr(), 16); 0
}

pub unsafe fn dprc_set_obj_irq(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, obj_type: *mut i8, obj_id: i32, irq_index: u8, irq_cfg: *const dprc_irq_cfg) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed(); let p = cmd.params.as_mut_ptr() as *mut dprc_cmd_set_obj_irq;
    cmd.header = mc_encode_cmd_header(DPRC_CMDID_SET_OBJ_IRQ, cmd_flags, token); (*p).irq_val = cpu_to_le32((*irq_cfg).val); (*p).irq_index = irq_index; (*p).irq_addr = cpu_to_le64((*irq_cfg).paddr); (*p).irq_num = cpu_to_le32((*irq_cfg).irq_num); (*p).obj_id = cpu_to_le32(obj_id as u32); strscpy((*p).obj_type.as_mut_ptr(), obj_type); mc_send_command(mc_io, &mut cmd)
}

pub unsafe fn dprc_get_obj_region(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, obj_type: *mut i8, obj_id: i32, region_index: u8, region_desc: *mut dprc_region_desc) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed(); let p: *mut dprc_cmd_get_obj_region; let err: i32;
    if dprc_major_ver == 0 && dprc_minor_ver == 0 { err = dprc_get_api_version(mc_io, 0, &mut dprc_major_ver, &mut dprc_minor_ver); if err != 0 { return err; } }
    let id = if dprc_major_ver > 6 || (dprc_major_ver == 6 && dprc_minor_ver >= 6) { DPRC_CMDID_GET_OBJ_REG_V3 } else if dprc_major_ver == 6 && dprc_minor_ver >= 3 { DPRC_CMDID_GET_OBJ_REG_V2 } else { DPRC_CMDID_GET_OBJ_REG };
    cmd.header = mc_encode_cmd_header(id, cmd_flags, token); p = cmd.params.as_mut_ptr() as *mut dprc_cmd_get_obj_region; (*p).obj_id = cpu_to_le32(obj_id as u32); (*p).region_index = region_index; strscpy((*p).obj_type.as_mut_ptr(), obj_type);
    err = mc_send_command(mc_io, &mut cmd); if err != 0 { return err; } let r = cmd.params.as_ptr() as *const dprc_rsp_get_obj_region;
    (*region_desc).base_offset = le64_to_cpu((*r).base_offset); (*region_desc).size = le32_to_cpu((*r).size); (*region_desc).type_ = (*r).type_; (*region_desc).flags = le32_to_cpu((*r).flags); (*region_desc).base_address = if dprc_major_ver > 6 || (dprc_major_ver == 6 && dprc_minor_ver >= 3) { le64_to_cpu((*r).base_addr) } else { 0 }; 0
}

pub unsafe fn dprc_get_api_version(mc_io: *mut fsl_mc_io, cmd_flags: u32, major_ver: *mut u16, minor_ver: *mut u16) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed(); cmd.header = mc_encode_cmd_header(DPRC_CMDID_GET_API_VERSION, cmd_flags, 0); let err = mc_send_command(mc_io, &mut cmd); if err != 0 { return err; } mc_cmd_read_api_version(&cmd, major_ver, minor_ver); 0
}

pub unsafe fn dprc_get_container_id(mc_io: *mut fsl_mc_io, cmd_flags: u32, container_id: *mut i32) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed(); cmd.header = mc_encode_cmd_header(DPRC_CMDID_GET_CONT_ID, cmd_flags, 0); let err = mc_send_command(mc_io, &mut cmd); if err != 0 { return err; } *container_id = mc_cmd_read_object_id(&cmd) as i32; 0
}

pub unsafe fn dprc_get_connection(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, endpoint1: *const dprc_endpoint, endpoint2: *mut dprc_endpoint, state: *mut i32) -> i32 {
    let mut cmd: fsl_mc_command = core::mem::zeroed(); let p = cmd.params.as_mut_ptr() as *mut dprc_cmd_get_connection;
    cmd.header = mc_encode_cmd_header(DPRC_CMDID_GET_CONNECTION, cmd_flags, token); (*p).ep1_id = cpu_to_le32((*endpoint1).id); (*p).ep1_interface_id = cpu_to_le16((*endpoint1).if_id); for i in 0..16 { (*p).ep1_type[i] = (*endpoint1).type_[i]; }
    if mc_send_command(mc_io, &mut cmd) != 0 { return -ENOTCONN; } let r = cmd.params.as_ptr() as *const dprc_rsp_get_connection; (*endpoint2).id = le32_to_cpu((*r).ep2_id); (*endpoint2).if_id = le16_to_cpu((*r).ep2_interface_id); *state = le32_to_cpu((*r).state) as i32; for i in 0..16 { (*endpoint2).type_[i] = (*r).ep2_type[i]; } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
