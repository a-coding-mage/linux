/*
 * Copyright 2020 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 *  and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding DCN implementation provide the
// register definitions, types, conversion helpers, and register-operation
// macros used below.

pub unsafe fn vpg3_update_generic_info_packet(
    vpg: *mut struct_vpg,
    packet_index: u32,
    info_packet: *const dc_info_packet,
    immediate_update: bool,
) {
    let vpg3: *mut dcn30_vpg = DCN30_VPG_FROM_VPG!(vpg);
    let mut i: u32;
    let max_retries: u32 = 50;

    if packet_index > 14 {
        ASSERT!(0);
    }

    // Poll dig_update_lock is not locked -> ASIC internal signal; assume OTG
    // master lock will unlock it. The original REG_WAIT is intentionally
    // disabled, as in the C implementation.

    // Check if HW is reading GSP memory.
    REG_WAIT!(VPG_GENERIC_STATUS, VPG_GENERIC_CONFLICT_OCCURED,
        0, 10, max_retries);

    // HW is not reading GSP memory; clear GSP memory access.
    REG_UPDATE!(VPG_GENERIC_STATUS, VPG_GENERIC_CONFLICT_CLR, 1);

    // Choose which generic packet to use.
    REG_UPDATE!(VPG_GENERIC_PACKET_ACCESS_CTRL,
        VPG_GENERIC_DATA_INDEX, packet_index * 9);

    // Write generic packet header (4th byte is for GENERIC0 only).
    REG_SET_4!(VPG_GENERIC_PACKET_DATA, 0,
        VPG_GENERIC_DATA_BYTE0, (*info_packet).hb0,
        VPG_GENERIC_DATA_BYTE1, (*info_packet).hb1,
        VPG_GENERIC_DATA_BYTE2, (*info_packet).hb2,
        VPG_GENERIC_DATA_BYTE3, (*info_packet).hb3);

    // Write generic packet contents; the last 4 bytes are never used.
    let mut content: *const u32 = (*info_packet).sb.as_ptr() as *const u32;
    i = 0;
    while i < 8 {
        REG_WRITE!(VPG_GENERIC_PACKET_DATA, *content);
        content = content.add(1);
        i += 1;
    }

    // Atomically update double-buffered GENERIC registers.
    if immediate_update {
        match packet_index {
            0 => REG_UPDATE!(VPG_GSP_IMMEDIATE_UPDATE_CTRL, VPG_GENERIC0_IMMEDIATE_UPDATE, 1),
            1 => REG_UPDATE!(VPG_GSP_IMMEDIATE_UPDATE_CTRL, VPG_GENERIC1_IMMEDIATE_UPDATE, 1),
            2 => REG_UPDATE!(VPG_GSP_IMMEDIATE_UPDATE_CTRL, VPG_GENERIC2_IMMEDIATE_UPDATE, 1),
            3 => REG_UPDATE!(VPG_GSP_IMMEDIATE_UPDATE_CTRL, VPG_GENERIC3_IMMEDIATE_UPDATE, 1),
            4 => REG_UPDATE!(VPG_GSP_IMMEDIATE_UPDATE_CTRL, VPG_GENERIC4_IMMEDIATE_UPDATE, 1),
            5 => REG_UPDATE!(VPG_GSP_IMMEDIATE_UPDATE_CTRL, VPG_GENERIC5_IMMEDIATE_UPDATE, 1),
            6 => REG_UPDATE!(VPG_GSP_IMMEDIATE_UPDATE_CTRL, VPG_GENERIC6_IMMEDIATE_UPDATE, 1),
            7 => REG_UPDATE!(VPG_GSP_IMMEDIATE_UPDATE_CTRL, VPG_GENERIC7_IMMEDIATE_UPDATE, 1),
            8 => REG_UPDATE!(VPG_GSP_IMMEDIATE_UPDATE_CTRL, VPG_GENERIC8_IMMEDIATE_UPDATE, 1),
            9 => REG_UPDATE!(VPG_GSP_IMMEDIATE_UPDATE_CTRL, VPG_GENERIC9_IMMEDIATE_UPDATE, 1),
            10 => REG_UPDATE!(VPG_GSP_IMMEDIATE_UPDATE_CTRL, VPG_GENERIC10_IMMEDIATE_UPDATE, 1),
            11 => REG_UPDATE!(VPG_GSP_IMMEDIATE_UPDATE_CTRL, VPG_GENERIC11_IMMEDIATE_UPDATE, 1),
            12 => REG_UPDATE!(VPG_GSP_IMMEDIATE_UPDATE_CTRL, VPG_GENERIC12_IMMEDIATE_UPDATE, 1),
            13 => REG_UPDATE!(VPG_GSP_IMMEDIATE_UPDATE_CTRL, VPG_GENERIC13_IMMEDIATE_UPDATE, 1),
            14 => REG_UPDATE!(VPG_GSP_IMMEDIATE_UPDATE_CTRL, VPG_GENERIC14_IMMEDIATE_UPDATE, 1),
            _ => {}
        }
    } else {
        match packet_index {
            0 => REG_UPDATE!(VPG_GSP_FRAME_UPDATE_CTRL, VPG_GENERIC0_FRAME_UPDATE, 1),
            1 => REG_UPDATE!(VPG_GSP_FRAME_UPDATE_CTRL, VPG_GENERIC1_FRAME_UPDATE, 1),
            2 => REG_UPDATE!(VPG_GSP_FRAME_UPDATE_CTRL, VPG_GENERIC2_FRAME_UPDATE, 1),
            3 => REG_UPDATE!(VPG_GSP_FRAME_UPDATE_CTRL, VPG_GENERIC3_FRAME_UPDATE, 1),
            4 => REG_UPDATE!(VPG_GSP_FRAME_UPDATE_CTRL, VPG_GENERIC4_FRAME_UPDATE, 1),
            5 => REG_UPDATE!(VPG_GSP_FRAME_UPDATE_CTRL, VPG_GENERIC5_FRAME_UPDATE, 1),
            6 => REG_UPDATE!(VPG_GSP_FRAME_UPDATE_CTRL, VPG_GENERIC6_FRAME_UPDATE, 1),
            7 => REG_UPDATE!(VPG_GSP_FRAME_UPDATE_CTRL, VPG_GENERIC7_FRAME_UPDATE, 1),
            8 => REG_UPDATE!(VPG_GSP_FRAME_UPDATE_CTRL, VPG_GENERIC8_FRAME_UPDATE, 1),
            9 => REG_UPDATE!(VPG_GSP_FRAME_UPDATE_CTRL, VPG_GENERIC9_FRAME_UPDATE, 1),
            10 => REG_UPDATE!(VPG_GSP_FRAME_UPDATE_CTRL, VPG_GENERIC10_FRAME_UPDATE, 1),
            11 => REG_UPDATE!(VPG_GSP_FRAME_UPDATE_CTRL, VPG_GENERIC11_FRAME_UPDATE, 1),
            12 => REG_UPDATE!(VPG_GSP_FRAME_UPDATE_CTRL, VPG_GENERIC12_FRAME_UPDATE, 1),
            13 => REG_UPDATE!(VPG_GSP_FRAME_UPDATE_CTRL, VPG_GENERIC13_FRAME_UPDATE, 1),
            14 => REG_UPDATE!(VPG_GSP_FRAME_UPDATE_CTRL, VPG_GENERIC14_FRAME_UPDATE, 1),
            _ => {}
        }
    }
}

static mut DCN30_VPG_FUNCS: vpg_funcs = vpg_funcs {
    update_generic_info_packet: Some(vpg3_update_generic_info_packet),
};

pub unsafe fn vpg3_construct(
    vpg3: *mut dcn30_vpg,
    ctx: *mut dc_context,
    inst: u32,
    vpg_regs: *const dcn30_vpg_registers,
    vpg_shift: *const dcn30_vpg_shift,
    vpg_mask: *const dcn30_vpg_mask,
) {
    (*vpg3).base.ctx = ctx;
    (*vpg3).base.inst = inst;
    (*vpg3).base.funcs = &raw const DCN30_VPG_FUNCS;
    (*vpg3).regs = vpg_regs;
    (*vpg3).vpg_shift = vpg_shift;
    (*vpg3).vpg_mask = vpg_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
