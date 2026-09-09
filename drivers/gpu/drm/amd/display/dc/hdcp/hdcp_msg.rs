/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies are supplied by the surrounding translation unit.

const HDCP14_KSV_SIZE: usize = 5;
const HDCP14_MAX_KSV_FIFO_SIZE: usize = 127 * HDCP14_KSV_SIZE;

static HDCP_CMD_IS_READ: [bool; HDCP_MESSAGE_ID_MAX as usize] = {
    let mut a = [false; HDCP_MESSAGE_ID_MAX as usize];
    a[HDCP_MESSAGE_ID_READ_BKSV as usize] = true;
    a[HDCP_MESSAGE_ID_READ_RI_R0 as usize] = true;
    a[HDCP_MESSAGE_ID_READ_PJ as usize] = true;
    a[HDCP_MESSAGE_ID_READ_VH_X as usize] = true;
    a[HDCP_MESSAGE_ID_READ_VH_0 as usize] = true;
    a[HDCP_MESSAGE_ID_READ_VH_1 as usize] = true;
    a[HDCP_MESSAGE_ID_READ_VH_2 as usize] = true;
    a[HDCP_MESSAGE_ID_READ_VH_3 as usize] = true;
    a[HDCP_MESSAGE_ID_READ_VH_4 as usize] = true;
    a[HDCP_MESSAGE_ID_READ_BCAPS as usize] = true;
    a[HDCP_MESSAGE_ID_READ_BSTATUS as usize] = true;
    a[HDCP_MESSAGE_ID_READ_KSV_FIFO as usize] = true;
    a[HDCP_MESSAGE_ID_READ_BINFO as usize] = true;
    a[HDCP_MESSAGE_ID_HDCP2VERSION as usize] = true;
    a[HDCP_MESSAGE_ID_RX_CAPS as usize] = true;
    a[HDCP_MESSAGE_ID_READ_AKE_SEND_CERT as usize] = true;
    a[HDCP_MESSAGE_ID_READ_AKE_SEND_H_PRIME as usize] = true;
    a[HDCP_MESSAGE_ID_READ_AKE_SEND_PAIRING_INFO as usize] = true;
    a[HDCP_MESSAGE_ID_READ_LC_SEND_L_PRIME as usize] = true;
    a[HDCP_MESSAGE_ID_READ_REPEATER_AUTH_SEND_RECEIVERID_LIST as usize] = true;
    a[HDCP_MESSAGE_ID_READ_REPEATER_AUTH_STREAM_READY as usize] = true;
    a[HDCP_MESSAGE_ID_READ_RXSTATUS as usize] = true;
    a
};

static HDCP_I2C_OFFSETS: [u8; HDCP_MESSAGE_ID_MAX as usize] = [0; HDCP_MESSAGE_ID_MAX as usize];
static HDCP_DPCD_ADDRS: [u32; HDCP_MESSAGE_ID_MAX as usize] = [0; HDCP_MESSAGE_ID_MAX as usize];

#[repr(C)]
struct protection_properties {
    supported: bool,
    process_transaction: Option<unsafe extern "C" fn(*mut dc_link, *mut hdcp_protection_message) -> bool>,
}

static NON_SUPPORTED_PROTECTION: protection_properties = protection_properties { supported: false, process_transaction: None };

unsafe fn hdmi_14_process_transaction(link: *mut dc_link, message_info: *mut hdcp_protection_message) -> bool {
    let mut buff: *mut u8 = core::ptr::null_mut();
    let hdcp_i2c_addr_link_primary: u8 = 0x3a;
    let hdcp_i2c_addr_link_secondary: u8 = 0x3b;
    let mut i2c_command: i2c_command = core::mem::zeroed();
    let offset = HDCP_I2C_OFFSETS[(*message_info).msg_id as usize];
    if (*message_info).msg_id == HDCP_MESSAGE_ID_INVALID { return false; }
    let mut payloads: [i2c_payload; 2] = [
        i2c_payload { write: true, address: 0, length: 1, data: &offset as *const u8 as *mut u8 },
        i2c_payload { write: false, address: 0, length: 0, data: core::ptr::null_mut() },
    ];
    let address = if (*message_info).link == HDCP_LINK_SECONDARY { hdcp_i2c_addr_link_secondary } else { hdcp_i2c_addr_link_primary };
    payloads[0].address = address; payloads[1].address = address;
    let is_read = HDCP_CMD_IS_READ[(*message_info).msg_id as usize];
    if is_read {
        payloads[1].write = false; payloads[1].length = (*message_info).length; payloads[1].data = (*message_info).data;
        i2c_command.number_of_payloads = 2;
    } else {
        buff = kzalloc((*message_info).length + 1, GFP_KERNEL);
        if buff.is_null() { return false; }
        *buff = offset; core::ptr::copy_nonoverlapping((*message_info).data, buff.add(1), (*message_info).length);
        payloads[0].length = (*message_info).length + 1; payloads[0].data = buff; i2c_command.number_of_payloads = 1;
    }
    i2c_command.payloads = payloads.as_mut_ptr(); i2c_command.engine = I2C_COMMAND_ENGINE_HW;
    i2c_command.speed = (*(*(*link).ddc).ctx).dc.caps.i2c_speed_in_khz;
    let result = if (*link).force_to_use_aux { dm_helpers_submit_i2c_over_aux((*link).ddc, hdcp_i2c_addr_link_primary, offset, (*message_info).data, (*message_info).length, is_read) } else { dm_helpers_submit_i2c((*link).ctx, link, &mut i2c_command) };
    kfree(buff); result
}

static HDMI_14_PROTECTION: protection_properties = protection_properties { supported: true, process_transaction: Some(hdmi_14_process_transaction) };

unsafe fn dpcd_access_helper(link: *mut dc_link, mut length: u32, data: *mut u8, dpcd_addr: u32, is_read: bool) -> bool {
    let mut offset = 0; let ksv_read_size = 0x6803b - 0x6802c;
    if dpcd_addr == 0x6802c {
        while length > 0 { let n = if length > ksv_read_size { ksv_read_size } else { length }; if core_link_read_dpcd(link, dpcd_addr + offset, data.add(offset as usize), n) != DC_OK { return false; } length -= n; offset += n; }
    } else {
        while length > 0 { let n = if length > DEFAULT_AUX_MAX_DATA_SIZE { DEFAULT_AUX_MAX_DATA_SIZE } else { length }; let s = if is_read { core_link_read_dpcd(link, dpcd_addr + offset, data.add(offset as usize), n) } else { core_link_write_dpcd(link, dpcd_addr + offset, data.add(offset as usize), n) }; if s != DC_OK { return false; } length -= n; offset += n; }
    } true
}

unsafe fn dp_11_process_transaction(link: *mut dc_link, message_info: *mut hdcp_protection_message) -> bool {
    if (*message_info).msg_id == HDCP_MESSAGE_ID_INVALID { return false; }
    dpcd_access_helper(link, (*message_info).length, (*message_info).data, HDCP_DPCD_ADDRS[(*message_info).msg_id as usize], HDCP_CMD_IS_READ[(*message_info).msg_id as usize])
}
static DP_11_PROTECTION: protection_properties = protection_properties { supported: true, process_transaction: Some(dp_11_process_transaction) };

unsafe fn get_protection_properties_by_signal(_link: *mut dc_link, _st: signal_type, _version: hdcp_version) -> *const protection_properties { &DP_11_PROTECTION }

pub unsafe fn dc_process_hdcp_msg(signal: signal_type, link: *mut dc_link, message_info: *mut hdcp_protection_message) -> hdcp_message_status {
    if message_info.is_null() || (*message_info).msg_id < HDCP_MESSAGE_ID_READ_BKSV || (*message_info).msg_id >= HDCP_MESSAGE_ID_MAX { return HDCP_MESSAGE_UNSUPPORTED; }
    let p = &*get_protection_properties_by_signal(link, signal, (*message_info).version);
    if !p.supported { return HDCP_MESSAGE_UNSUPPORTED; }
    if (p.process_transaction.unwrap())(link, message_info) { HDCP_MESSAGE_SUCCESS } else { let mut i = 0; while i < (*message_info).max_retries { if (p.process_transaction.unwrap())(link, message_info) { return HDCP_MESSAGE_SUCCESS; } i += 1; } HDCP_MESSAGE_FAILURE }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
