/****************************************************************************\
* 
*  Module Name    displayobjectsoc15.h
*  Project        
*  Device         
*
*  Description    Contains the common definitions for display objects for SoC15 products.
*
*  Copyright 2014 Advanced Micro Devices, Inc.
*
* Permission is hereby granted, free of charge, to any person obtaining a copy of this software 
* and associated documentation files (the "Software"), to deal in the Software without restriction,
* including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
* and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so,
* subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all copies or substantial
* portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
* THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
* OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
* ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
* OTHER DEALINGS IN THE SOFTWARE.
*
\****************************************************************************/

/* #if defined(_X86_) : C struct packing was requested here; no structs occur in this header. */

#[repr(u32)]
pub enum display_object_type {
    DISPLAY_OBJECT_TYPE_NONE = 0x00,
    DISPLAY_OBJECT_TYPE_GPU = 0x01,
    DISPLAY_OBJECT_TYPE_ENCODER = 0x02,
    DISPLAY_OBJECT_TYPE_CONNECTOR = 0x03,
}

#[repr(u32)]
pub enum encoder_object_type {
    ENCODER_OBJECT_ID_NONE = 0x00,
    ENCODER_OBJECT_ID_INTERNAL_UNIPHY = 0x01,
    ENCODER_OBJECT_ID_INTERNAL_UNIPHY1 = 0x02,
    ENCODER_OBJECT_ID_INTERNAL_UNIPHY2 = 0x03,
}

#[repr(u32)]
pub enum connector_object_type {
    CONNECTOR_OBJECT_ID_NONE = 0x00,
    CONNECTOR_OBJECT_ID_SINGLE_LINK_DVI_D = 0x01,
    CONNECTOR_OBJECT_ID_DUAL_LINK_DVI_D = 0x02,
    CONNECTOR_OBJECT_ID_HDMI_TYPE_A = 0x03,
    CONNECTOR_OBJECT_ID_LVDS = 0x04,
    CONNECTOR_OBJECT_ID_DISPLAYPORT = 0x05,
    CONNECTOR_OBJECT_ID_eDP = 0x06,
    CONNECTOR_OBJECT_ID_OPM = 0x07,
}

// Protection Object ID definition: No need.

#[repr(u32)]
pub enum object_enum_id {
    OBJECT_ENUM_ID1 = 0x01,
    OBJECT_ENUM_ID2 = 0x02,
    OBJECT_ENUM_ID3 = 0x03,
    OBJECT_ENUM_ID4 = 0x04,
    OBJECT_ENUM_ID5 = 0x05,
    OBJECT_ENUM_ID6 = 0x06,
}

#[repr(u32)]
pub enum object_id_bit {
    OBJECT_ID_MASK = 0x00FF,
    ENUM_ID_MASK = 0x0F00,
    OBJECT_TYPE_MASK = 0xF000,
    OBJECT_ID_SHIFT = 0x00,
    ENUM_ID_SHIFT = 0x08,
    OBJECT_TYPE_SHIFT = 0x0C,
}

pub const GPU_ENUM_ID1: u32 =
    ((display_object_type::DISPLAY_OBJECT_TYPE_GPU as u32) << (object_id_bit::OBJECT_TYPE_SHIFT as u32))
    | ((object_enum_id::OBJECT_ENUM_ID1 as u32) << (object_id_bit::ENUM_ID_SHIFT as u32));

pub const ENCODER_INTERNAL_UNIPHY_ENUM_ID1: u32 =
    ((display_object_type::DISPLAY_OBJECT_TYPE_ENCODER as u32) << 12)
    | ((object_enum_id::OBJECT_ENUM_ID1 as u32) << 8)
    | (encoder_object_type::ENCODER_OBJECT_ID_INTERNAL_UNIPHY as u32);
pub const ENCODER_INTERNAL_UNIPHY_ENUM_ID2: u32 =
    ((display_object_type::DISPLAY_OBJECT_TYPE_ENCODER as u32) << 12)
    | ((object_enum_id::OBJECT_ENUM_ID2 as u32) << 8)
    | (encoder_object_type::ENCODER_OBJECT_ID_INTERNAL_UNIPHY as u32);
pub const ENCODER_INTERNAL_UNIPHY1_ENUM_ID1: u32 =
    ((display_object_type::DISPLAY_OBJECT_TYPE_ENCODER as u32) << 12)
    | ((object_enum_id::OBJECT_ENUM_ID1 as u32) << 8)
    | (encoder_object_type::ENCODER_OBJECT_ID_INTERNAL_UNIPHY1 as u32);
pub const ENCODER_INTERNAL_UNIPHY1_ENUM_ID2: u32 =
    ((display_object_type::DISPLAY_OBJECT_TYPE_ENCODER as u32) << 12)
    | ((object_enum_id::OBJECT_ENUM_ID2 as u32) << 8)
    | (encoder_object_type::ENCODER_OBJECT_ID_INTERNAL_UNIPHY1 as u32);
pub const ENCODER_INTERNAL_UNIPHY2_ENUM_ID1: u32 =
    ((display_object_type::DISPLAY_OBJECT_TYPE_ENCODER as u32) << 12)
    | ((object_enum_id::OBJECT_ENUM_ID1 as u32) << 8)
    | (encoder_object_type::ENCODER_OBJECT_ID_INTERNAL_UNIPHY2 as u32);
pub const ENCODER_INTERNAL_UNIPHY2_ENUM_ID2: u32 =
    ((display_object_type::DISPLAY_OBJECT_TYPE_ENCODER as u32) << 12)
    | ((object_enum_id::OBJECT_ENUM_ID2 as u32) << 8)
    | (encoder_object_type::ENCODER_OBJECT_ID_INTERNAL_UNIPHY2 as u32);

pub const CONNECTOR_LVDS_ENUM_ID1: u32 = (3 << 12) | (1 << 8) | 4;
pub const CONNECTOR_eDP_ENUM_ID1: u32 = (3 << 12) | (1 << 8) | 6;
pub const CONNECTOR_SINGLE_LINK_DVI_D_ENUM_ID1: u32 = (3 << 12) | (1 << 8) | 1;
pub const CONNECTOR_SINGLE_LINK_DVI_D_ENUM_ID2: u32 = (3 << 12) | (2 << 8) | 1;
pub const CONNECTOR_DUAL_LINK_DVI_D_ENUM_ID1: u32 = (3 << 12) | (1 << 8) | 2;
pub const CONNECTOR_DUAL_LINK_DVI_D_ENUM_ID2: u32 = (3 << 12) | (2 << 8) | 2;
pub const CONNECTOR_HDMI_TYPE_A_ENUM_ID1: u32 = (3 << 12) | (1 << 8) | 3;
pub const CONNECTOR_HDMI_TYPE_A_ENUM_ID2: u32 = (3 << 12) | (2 << 8) | 3;
pub const CONNECTOR_DISPLAYPORT_ENUM_ID1: u32 = (3 << 12) | (1 << 8) | 5;
pub const CONNECTOR_DISPLAYPORT_ENUM_ID2: u32 = (3 << 12) | (2 << 8) | 5;
pub const CONNECTOR_DISPLAYPORT_ENUM_ID3: u32 = (3 << 12) | (3 << 8) | 5;
pub const CONNECTOR_DISPLAYPORT_ENUM_ID4: u32 = (3 << 12) | (4 << 8) | 5;
pub const CONNECTOR_OPM_ENUM_ID1: u32 = (3 << 12) | (1 << 8) | 7; // Mapping to MXM_DP_A
pub const CONNECTOR_OPM_ENUM_ID2: u32 = (3 << 12) | (2 << 8) | 7; // Mapping to MXM_DP_B
pub const CONNECTOR_OPM_ENUM_ID3: u32 = (3 << 12) | (3 << 8) | 7; // Mapping to MXM_DP_C
pub const CONNECTOR_OPM_ENUM_ID4: u32 = (3 << 12) | (4 << 8) | 7; // Mapping to MXM_DP_D
pub const CONNECTOR_OPM_ENUM_ID5: u32 = (3 << 12) | (5 << 8) | 7; // Mapping to MXM_LVDS_TXxx
pub const CONNECTOR_OPM_ENUM_ID6: u32 = (3 << 12) | (6 << 8) | 7; // Mapping to MXM_LVDS_TXxx

// Router Object ID definition: No need; reserved for a future Atom firmware record.
// Protection Object ID definition: No need; all display paths are capable of protection now.
// Generic Object ID definition: No need; reserved for future objects such as GLsync.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
