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

fn object_type_from_bios_object_id(bios_object_id: u32) -> object_type {
    let bios_object_type = (bios_object_id & OBJECT_TYPE_MASK) >> OBJECT_TYPE_SHIFT;
    match bios_object_type {
        GRAPH_OBJECT_TYPE_GPU => OBJECT_TYPE_GPU,
        GRAPH_OBJECT_TYPE_ENCODER => OBJECT_TYPE_ENCODER,
        GRAPH_OBJECT_TYPE_CONNECTOR => OBJECT_TYPE_CONNECTOR,
        GRAPH_OBJECT_TYPE_ROUTER => OBJECT_TYPE_ROUTER,
        GRAPH_OBJECT_TYPE_GENERIC => OBJECT_TYPE_GENERIC,
        _ => OBJECT_TYPE_UNKNOWN,
    }
}

fn enum_id_from_bios_object_id(bios_object_id: u32) -> object_enum_id {
    let bios_enum_id = (bios_object_id & ENUM_ID_MASK) >> ENUM_ID_SHIFT;
    match bios_enum_id {
        GRAPH_OBJECT_ENUM_ID1 => ENUM_ID_1,
        GRAPH_OBJECT_ENUM_ID2 => ENUM_ID_2,
        GRAPH_OBJECT_ENUM_ID3 => ENUM_ID_3,
        GRAPH_OBJECT_ENUM_ID4 => ENUM_ID_4,
        GRAPH_OBJECT_ENUM_ID5 => ENUM_ID_5,
        GRAPH_OBJECT_ENUM_ID6 => ENUM_ID_6,
        GRAPH_OBJECT_ENUM_ID7 => ENUM_ID_7,
        _ => ENUM_ID_UNKNOWN,
    }
}

fn gpu_id_from_bios_object_id(bios_object_id: u32) -> u32 {
    (bios_object_id & OBJECT_ID_MASK) >> OBJECT_ID_SHIFT
}

fn encoder_id_from_bios_object_id(bios_object_id: u32) -> encoder_id {
    let bios_encoder_id = gpu_id_from_bios_object_id(bios_object_id);
    match bios_encoder_id {
        ENCODER_OBJECT_ID_INTERNAL_LVDS => ENCODER_ID_INTERNAL_LVDS,
        ENCODER_OBJECT_ID_INTERNAL_TMDS1 => ENCODER_ID_INTERNAL_TMDS1,
        ENCODER_OBJECT_ID_INTERNAL_TMDS2 => ENCODER_ID_INTERNAL_TMDS2,
        ENCODER_OBJECT_ID_INTERNAL_DAC1 => ENCODER_ID_INTERNAL_DAC1,
        ENCODER_OBJECT_ID_INTERNAL_DAC2 => ENCODER_ID_INTERNAL_DAC2,
        ENCODER_OBJECT_ID_INTERNAL_LVTM1 => ENCODER_ID_INTERNAL_LVTM1,
        ENCODER_OBJECT_ID_HDMI_INTERNAL => ENCODER_ID_INTERNAL_HDMI,
        ENCODER_OBJECT_ID_INTERNAL_KLDSCP_TMDS1 => ENCODER_ID_INTERNAL_KLDSCP_TMDS1,
        ENCODER_OBJECT_ID_INTERNAL_KLDSCP_DAC1 => ENCODER_ID_INTERNAL_KLDSCP_DAC1,
        ENCODER_OBJECT_ID_INTERNAL_KLDSCP_DAC2 => ENCODER_ID_INTERNAL_KLDSCP_DAC2,
        ENCODER_OBJECT_ID_MVPU_FPGA => ENCODER_ID_EXTERNAL_MVPU_FPGA,
        ENCODER_OBJECT_ID_INTERNAL_DDI => ENCODER_ID_INTERNAL_DDI,
        ENCODER_OBJECT_ID_INTERNAL_UNIPHY => ENCODER_ID_INTERNAL_UNIPHY,
        ENCODER_OBJECT_ID_INTERNAL_KLDSCP_LVTMA => ENCODER_ID_INTERNAL_KLDSCP_LVTMA,
        ENCODER_OBJECT_ID_INTERNAL_UNIPHY1 => ENCODER_ID_INTERNAL_UNIPHY1,
        ENCODER_OBJECT_ID_INTERNAL_UNIPHY2 => ENCODER_ID_INTERNAL_UNIPHY2,
        ENCODER_OBJECT_ID_ALMOND /* ENCODER_OBJECT_ID_NUTMEG */ => {
            ENCODER_ID_EXTERNAL_NUTMEG
        }
        ENCODER_OBJECT_ID_TRAVIS => ENCODER_ID_EXTERNAL_TRAVIS,
        ENCODER_OBJECT_ID_INTERNAL_UNIPHY3 => ENCODER_ID_INTERNAL_UNIPHY3,
        _ => {
            ASSERT(0);
            ENCODER_ID_UNKNOWN
        }
    }
}

fn connector_id_from_bios_object_id(bios_object_id: u32) -> connector_id {
    let bios_connector_id = gpu_id_from_bios_object_id(bios_object_id);
    match bios_connector_id {
        CONNECTOR_OBJECT_ID_SINGLE_LINK_DVI_I => CONNECTOR_ID_SINGLE_LINK_DVII,
        CONNECTOR_OBJECT_ID_DUAL_LINK_DVI_I => CONNECTOR_ID_DUAL_LINK_DVII,
        CONNECTOR_OBJECT_ID_SINGLE_LINK_DVI_D => CONNECTOR_ID_SINGLE_LINK_DVID,
        CONNECTOR_OBJECT_ID_DUAL_LINK_DVI_D => CONNECTOR_ID_DUAL_LINK_DVID,
        CONNECTOR_OBJECT_ID_VGA => CONNECTOR_ID_VGA,
        CONNECTOR_OBJECT_ID_HDMI_TYPE_A => CONNECTOR_ID_HDMI_TYPE_A,
        CONNECTOR_OBJECT_ID_LVDS => CONNECTOR_ID_LVDS,
        CONNECTOR_OBJECT_ID_PCIE_CONNECTOR => CONNECTOR_ID_PCIE,
        CONNECTOR_OBJECT_ID_HARDCODE_DVI => CONNECTOR_ID_HARDCODE_DVI,
        CONNECTOR_OBJECT_ID_DISPLAYPORT => CONNECTOR_ID_DISPLAY_PORT,
        CONNECTOR_OBJECT_ID_eDP => CONNECTOR_ID_EDP,
        CONNECTOR_OBJECT_ID_MXM => CONNECTOR_ID_MXM,
        CONNECTOR_OBJECT_ID_USBC => CONNECTOR_ID_USBC,
        _ => CONNECTOR_ID_UNKNOWN,
    }
}

fn generic_id_from_bios_object_id(bios_object_id: u32) -> generic_id {
    let bios_generic_id = gpu_id_from_bios_object_id(bios_object_id);
    match bios_generic_id {
        GENERIC_OBJECT_ID_MXM_OPM => GENERIC_ID_MXM_OPM,
        GENERIC_OBJECT_ID_GLSYNC => GENERIC_ID_GLSYNC,
        GENERIC_OBJECT_ID_STEREO_PIN => GENERIC_ID_STEREO,
        _ => GENERIC_ID_UNKNOWN,
    }
}

fn id_from_bios_object_id(object_type: object_type, bios_object_id: u32) -> u32 {
    match object_type {
        OBJECT_TYPE_GPU => gpu_id_from_bios_object_id(bios_object_id),
        OBJECT_TYPE_ENCODER => encoder_id_from_bios_object_id(bios_object_id) as u32,
        OBJECT_TYPE_CONNECTOR => connector_id_from_bios_object_id(bios_object_id) as u32,
        OBJECT_TYPE_GENERIC => generic_id_from_bios_object_id(bios_object_id) as u32,
        _ => 0,
    }
}

pub fn object_id_from_bios_object_id(bios_object_id: u32) -> graphics_object_id {
    let object_type = object_type_from_bios_object_id(bios_object_id);
    let mut go_id: graphics_object_id = unsafe { core::mem::zeroed() };

    if OBJECT_TYPE_UNKNOWN == object_type {
        return go_id;
    }

    let enum_id = enum_id_from_bios_object_id(bios_object_id);
    if ENUM_ID_UNKNOWN == enum_id {
        return go_id;
    }

    go_id = dal_graphics_object_id_init(
        id_from_bios_object_id(object_type, bios_object_id),
        enum_id,
        object_type,
    );
    go_id
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
