// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependencies are supplied by the surrounding translation unit.

unsafe fn op_i2c_convert(
    cmd: *mut union dmub_rb_cmd,
    op: *const mod_hdcp_atomic_op_i2c,
    type_: enum dmub_cmd_fused_request_type,
    ddc_line: u32,
    over_aux: bool,
) -> bool {
    let req = &mut (*cmd).fused_io.request;
    let loc = &mut req.u.i2c;

    if op.is_null() || (*op).size > core::mem::size_of_val(&req.buffer) {
        return false;
    }

    req.type_ = type_;
    loc.is_aux = false;
    loc.ddc_line = ddc_line as u8;
    loc.over_aux = over_aux;
    loc.address = (*op).address;
    loc.offset = (*op).offset;
    loc.length = (*op).size as u8;
    core::ptr::copy_nonoverlapping((*op).data, req.buffer.as_mut_ptr(), (*op).size);

    true
}

unsafe fn op_aux_convert(
    cmd: *mut union dmub_rb_cmd,
    op: *const mod_hdcp_atomic_op_aux,
    type_: enum dmub_cmd_fused_request_type,
    ddc_line: u32,
) -> bool {
    let req = &mut (*cmd).fused_io.request;
    let loc = &mut req.u.aux;

    if op.is_null() || (*op).size > core::mem::size_of_val(&req.buffer) {
        return false;
    }

    req.type_ = type_;
    loc.is_aux = true;
    loc.ddc_line = ddc_line;
    loc.address = (*op).address;
    loc.length = (*op).size;
    core::ptr::copy_nonoverlapping((*op).data, req.buffer.as_mut_ptr(), (*op).size);

    true
}

unsafe fn atomic_write_poll_read(
    link: *mut dc_link,
    commands: *mut [union dmub_rb_cmd; 3],
    poll_timeout_us: u32,
    poll_mask_msb: u8,
) -> bool {
    let count: u8 = 3;
    let timeout_per_request_us: u32 = 10000;
    let timeout_per_aux_transaction_us: u32 = 10000;
    let mut timeout_us: u64 = 0;

    (*commands)[1].fused_io.request.poll_mask_msb = poll_mask_msb;
    (*commands)[1].fused_io.request.timeout_us = poll_timeout_us;

    for i in 0..count {
        let io = &mut (*commands)[i as usize].fused_io;

        io.header.type_ = DMUB_CMD__FUSED_IO;
        io.header.sub_type = DMUB_CMD__FUSED_IO_EXECUTE;
        io.header.multi_cmd_pending = i != count - 1;
        io.header.payload_bytes = core::mem::size_of_val(io) - core::mem::size_of_val(&io.header);

        timeout_us += timeout_per_request_us as u64 + io.request.timeout_us as u64;
        if io.request.timeout_us == 0 && io.request.u.aux.is_aux {
            timeout_us += (timeout_per_aux_transaction_us as u64)
                * (io.request.u.aux.length as u64 / 16);
        }
    }

    if !dm_helpers_execute_fused_io((*link).ctx, link, commands, count, timeout_us as u32) {
        return false;
    }

    (*commands)[0].fused_io.request.status == FUSED_REQUEST_STATUS_SUCCESS
}

pub unsafe fn dm_atomic_write_poll_read_i2c(
    link: *mut dc_link,
    write: *const mod_hdcp_atomic_op_i2c,
    poll: *const mod_hdcp_atomic_op_i2c,
    read: *mut mod_hdcp_atomic_op_i2c,
    poll_timeout_us: u32,
    poll_mask_msb: u8,
) -> bool {
    if link.is_null() {
        return false;
    }

    let over_aux = (*link).no_ddc_pin;
    let ddc_line = if over_aux {
        (*link).aux_hw_inst
    } else {
        (*(*(*link).ddc).ddc_pin).pin_data.en
    };

    let mut commands: [union dmub_rb_cmd; 3] = core::mem::zeroed();
    let converted = op_i2c_convert(&mut commands[0], write, FUSED_REQUEST_WRITE, ddc_line, over_aux)
        && op_i2c_convert(&mut commands[1], poll, FUSED_REQUEST_POLL, ddc_line, over_aux)
        && op_i2c_convert(&mut commands[2], read, FUSED_REQUEST_READ, ddc_line, over_aux);

    if !converted {
        return false;
    }

    let result = atomic_write_poll_read(link, &mut commands, poll_timeout_us, poll_mask_msb);

    core::ptr::copy_nonoverlapping(commands[0].fused_io.request.buffer.as_ptr(), (*read).data, (*read).size);
    result
}

pub unsafe fn dm_atomic_write_poll_read_aux(
    link: *mut dc_link,
    write: *const mod_hdcp_atomic_op_aux,
    poll: *const mod_hdcp_atomic_op_aux,
    read: *mut mod_hdcp_atomic_op_aux,
    poll_timeout_us: u32,
    poll_mask_msb: u8,
) -> bool {
    if link.is_null() {
        return false;
    }

    let ddc_line = (*(*(*link).ddc).ddc_pin).pin_data.en;
    let mut commands: [union dmub_rb_cmd; 3] = core::mem::zeroed();
    let converted = op_aux_convert(&mut commands[0], write, FUSED_REQUEST_WRITE, ddc_line)
        && op_aux_convert(&mut commands[1], poll, FUSED_REQUEST_POLL, ddc_line)
        && op_aux_convert(&mut commands[2], read, FUSED_REQUEST_READ, ddc_line);

    if !converted {
        return false;
    }

    let result = atomic_write_poll_read(link, &mut commands, poll_timeout_us, poll_mask_msb);

    core::ptr::copy_nonoverlapping(commands[0].fused_io.request.buffer.as_ptr(), (*read).data, (*read).size);
    result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
