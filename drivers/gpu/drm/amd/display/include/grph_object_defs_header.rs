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
 */

// Dependency supplied by grph_object_id.h.

pub const MAX_CONNECTOR_NUMBER_PER_SLOT: u32 = 16;
pub const MAX_BOARD_SLOTS: u32 = 4;
pub const INVALID_CONNECTOR_INDEX: u32 = u32::MAX;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum hpd_source_id {
    HPD_SOURCEID1 = 0,
    HPD_SOURCEID2,
    HPD_SOURCEID3,
    HPD_SOURCEID4,
    HPD_SOURCEID5,
    HPD_SOURCEID6,
    HPD_SOURCEID_COUNT,
    HPD_SOURCEID_UNKNOWN,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum channel_id {
    CHANNEL_ID_UNKNOWN = 0,
    CHANNEL_ID_DDC1,
    CHANNEL_ID_DDC2,
    CHANNEL_ID_DDC3,
    CHANNEL_ID_DDC4,
    CHANNEL_ID_DDC5,
    CHANNEL_ID_DDC6,
    CHANNEL_ID_DDC_VGA,
    CHANNEL_ID_I2C_PAD,
    CHANNEL_ID_COUNT,
}

#[inline]
pub const fn DECODE_CHANNEL_ID(ch_id: channel_id) -> &'static str {
    match ch_id {
        channel_id::CHANNEL_ID_DDC1 => "CHANNEL_ID_DDC1",
        channel_id::CHANNEL_ID_DDC2 => "CHANNEL_ID_DDC2",
        channel_id::CHANNEL_ID_DDC3 => "CHANNEL_ID_DDC3",
        channel_id::CHANNEL_ID_DDC4 => "CHANNEL_ID_DDC4",
        channel_id::CHANNEL_ID_DDC5 => "CHANNEL_ID_DDC5",
        channel_id::CHANNEL_ID_DDC6 => "CHANNEL_ID_DDC6",
        channel_id::CHANNEL_ID_DDC_VGA => "CHANNEL_ID_DDC_VGA",
        channel_id::CHANNEL_ID_I2C_PAD => "CHANNEL_ID_I2C_PAD",
        _ => "Invalid",
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum transmitter {
    TRANSMITTER_UNKNOWN = -1,
    TRANSMITTER_UNIPHY_A,
    TRANSMITTER_UNIPHY_B,
    TRANSMITTER_UNIPHY_C,
    TRANSMITTER_UNIPHY_D,
    TRANSMITTER_UNIPHY_E,
    TRANSMITTER_UNIPHY_F,
    TRANSMITTER_NUTMEG_CRT,
    TRANSMITTER_TRAVIS_CRT,
    TRANSMITTER_TRAVIS_LCD,
    TRANSMITTER_UNIPHY_G,
    TRANSMITTER_COUNT,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum sync_source {
    SYNC_SOURCE_NONE = 0,
    SYNC_SOURCE_CONTROLLER0,
    SYNC_SOURCE_CONTROLLER1,
    SYNC_SOURCE_CONTROLLER2,
    SYNC_SOURCE_CONTROLLER3,
    SYNC_SOURCE_CONTROLLER4,
    SYNC_SOURCE_CONTROLLER5,
    SYNC_SOURCE_GSL_GROUP0,
    SYNC_SOURCE_GSL_GROUP1,
    SYNC_SOURCE_GSL_GROUP2,
    SYNC_SOURCE_GSL_IO_FIRST,
    SYNC_SOURCE_GSL_IO_GENLOCK_CLOCK = Self::SYNC_SOURCE_GSL_IO_FIRST as isize,
    SYNC_SOURCE_GSL_IO_GENLOCK_VSYNC,
    SYNC_SOURCE_GSL_IO_SWAPLOCK_A,
    SYNC_SOURCE_GSL_IO_SWAPLOCK_B,
    SYNC_SOURCE_GSL_IO_LAST = Self::SYNC_SOURCE_GSL_IO_SWAPLOCK_B as isize,
    SYNC_SOURCE_IO_FIRST,
    SYNC_SOURCE_IO_GENERIC_A = Self::SYNC_SOURCE_IO_FIRST as isize,
    SYNC_SOURCE_IO_GENERIC_B,
    SYNC_SOURCE_IO_GENERIC_C,
    SYNC_SOURCE_IO_GENERIC_D,
    SYNC_SOURCE_IO_GENERIC_E,
    SYNC_SOURCE_IO_GENERIC_F,
    SYNC_SOURCE_IO_HPD1,
    SYNC_SOURCE_IO_HPD2,
    SYNC_SOURCE_IO_HSYNC_A,
    SYNC_SOURCE_IO_VSYNC_A,
    SYNC_SOURCE_IO_HSYNC_B,
    SYNC_SOURCE_IO_VSYNC_B,
    SYNC_SOURCE_IO_LAST = Self::SYNC_SOURCE_IO_VSYNC_B as isize,
    SYNC_SOURCE_DUAL_GPU_PIN,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum tx_ffe_id {
    TX_FFE0 = 0,
    TX_FFE1,
    TX_FFE2,
    TX_FFE3,
    TX_FFE_DeEmphasis_Only,
    TX_FFE_PreShoot_Only,
    TX_FFE_No_FFE,
}

pub const CONNECTOR_SIZE_DVI: u32 = 40;
pub const CONNECTOR_SIZE_VGA: u32 = 32;
pub const CONNECTOR_SIZE_HDMI: u32 = 16;
pub const CONNECTOR_SIZE_DP: u32 = 16;
pub const CONNECTOR_SIZE_MINI_DP: u32 = 9;
pub const CONNECTOR_SIZE_UNKNOWN: u32 = 30;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum connector_layout_type {
    CONNECTOR_LAYOUT_TYPE_UNKNOWN,
    CONNECTOR_LAYOUT_TYPE_DVI_D,
    CONNECTOR_LAYOUT_TYPE_DVI_I,
    CONNECTOR_LAYOUT_TYPE_VGA,
    CONNECTOR_LAYOUT_TYPE_HDMI,
    CONNECTOR_LAYOUT_TYPE_DP,
    CONNECTOR_LAYOUT_TYPE_MINI_DP,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct connector_layout_info {
    pub connector_id: graphics_object_id,
    pub connector_type: connector_layout_type,
    pub length: u32,
    pub position: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct slot_layout_info {
    pub length: u32,
    pub width: u32,
    pub num_of_connectors: u32,
    pub connectors: [connector_layout_info; MAX_CONNECTOR_NUMBER_PER_SLOT as usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct board_layout_info {
    pub num_of_slots: u32,
    // C bit-fields; each member occupies one bit in the original u32 storage unit.
    pub is_number_of_slots_valid: u32,
    pub is_slots_size_valid: u32,
    pub is_connector_offsets_valid: u32,
    pub is_connector_lengths_valid: u32,
    pub slots: [slot_layout_info; MAX_BOARD_SLOTS as usize],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
