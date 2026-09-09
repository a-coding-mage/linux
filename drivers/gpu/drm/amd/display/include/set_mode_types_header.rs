/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 *
 */

// Dependencies supplied by the surrounding translation unit: dc_types.h and linux/hdmi.h.

/* Info frame packet status */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InfoFrameFlag {
    INFO_PACKET_PACKET_INVALID = 0,
    INFO_PACKET_PACKET_VALID = 1,
    INFO_PACKET_PACKET_RESET = 2,
    INFO_PACKET_PACKET_UPDATE_SCAN_TYPE = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdmiInfoFrameHeader {
    pub info_frame_type: u8,
    pub version: u8,
    pub length: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct InfoPacketRawData {
    pub hb0: u8,
    pub hb1: u8,
    pub hb2: u8,
    pub sb: [u8; 28], /* sb0~sb27 */
}

/* C bit-fields are represented as their declared byte storage here; callers
 * must apply the corresponding masks when interoperating with packed HDMI data. */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct AviInfoFrame {
    pub header: HdmiInfoFrameHeader,

    pub CHECK_SUM: u8,

    pub S0_S1: u8,
    pub B0_B1: u8,
    pub A0: u8,
    pub Y0_Y1_Y2: u8,

    pub R0_R3: u8,
    pub M0_M1: u8,
    pub C0_C1: u8,

    pub SC0_SC1: u8,
    pub Q0_Q1: u8,
    pub EC0_EC2: u8,
    pub ITC: u8,

    pub VIC0_VIC7: u8,

    pub PR0_PR3: u8,
    pub CN0_CN1: u8,
    pub YQ0_YQ1: u8,

    pub bar_top: u16,
    pub bar_bottom: u16,
    pub bar_left: u16,
    pub bar_right: u16,

    pub FR0_FR3: u8,
    pub ACE0_ACE3: u8,

    pub RID0_RID5: u8,
    pub FR4: u8,
    pub F157: u8,

    pub reserved: [u8; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union HdmiInfoPacket {
    pub bits: AviInfoFrame,
    pub packet_raw_data: InfoPacketRawData,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
