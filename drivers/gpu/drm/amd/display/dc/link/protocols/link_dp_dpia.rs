// SPDX-License-Identifier: MIT
/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding driver are intentionally external.

pub const DP_TUNNELING_CAPABILITIES_SUPPORT: u32 = 0xe000d;
pub const DP_IN_ADAPTER_INFO: u32 = 0xe000e;
pub const DP_USB4_DRIVER_ID: u32 = 0xe000f;
pub const DP_USB4_ROUTER_TOPOLOGY_ID: u32 = 0xe001b;

pub unsafe fn dpcd_get_tunneling_device_data(link: *mut dc_link) -> dc_status {
    let mut status: dc_status = DC_OK;
    let mut dpcd_dp_tun_data = [0u8; 3];
    let mut dpcd_topology_data = [0u8; DPCD_USB4_TOPOLOGY_ID_LEN as usize];
    let mut i: u8 = 0;

    status = core_link_read_dpcd(
        link,
        DP_TUNNELING_CAPABILITIES_SUPPORT,
        dpcd_dp_tun_data.as_mut_ptr(),
        dpcd_dp_tun_data.len(),
    );
    if status != DC_OK {
        return status;
    }

    (*link).dpcd_caps.usb4_dp_tun_info.dp_tun_cap.raw =
        dpcd_dp_tun_data[(DP_TUNNELING_CAPABILITIES_SUPPORT - DP_TUNNELING_CAPABILITIES_SUPPORT) as usize];

    if !(*link).dpcd_caps.usb4_dp_tun_info.dp_tun_cap.bits.dp_tunneling {
        return status;
    }

    (*link).dpcd_caps.usb4_dp_tun_info.dpia_info.raw =
        dpcd_dp_tun_data[(DP_IN_ADAPTER_INFO - DP_TUNNELING_CAPABILITIES_SUPPORT) as usize];
    (*link).dpcd_caps.usb4_dp_tun_info.usb4_driver_id =
        dpcd_dp_tun_data[(DP_USB4_DRIVER_ID - DP_TUNNELING_CAPABILITIES_SUPPORT) as usize] & 0x0f;

    if (*link).dpcd_caps.usb4_dp_tun_info.dp_tun_cap.bits.dpia_bw_alloc {
        status = core_link_read_dpcd(link, USB4_DRIVER_BW_CAPABILITY,
            dpcd_dp_tun_data.as_mut_ptr(), 2);
        if status != DC_OK {
            return status;
        }
        (*link).dpcd_caps.usb4_dp_tun_info.driver_bw_cap.raw =
            dpcd_dp_tun_data[0];
        (*link).dpcd_caps.usb4_dp_tun_info.dpia_tunnel_info.raw =
            dpcd_dp_tun_data[(DP_IN_ADAPTER_TUNNEL_INFO - USB4_DRIVER_BW_CAPABILITY) as usize];
    }

    DC_LOG_DEBUG!("{}: Link[{}] DP tunneling support (RouterId={} AdapterId={}) DPIA_BW_Alloc_support={} CM_BW_Alloc_support={}",
        "dpcd_get_tunneling_device_data", (*link).link_index,
        (*link).dpcd_caps.usb4_dp_tun_info.usb4_driver_id,
        (*link).dpcd_caps.usb4_dp_tun_info.dpia_info.bits.dpia_num,
        (*link).dpcd_caps.usb4_dp_tun_info.dp_tun_cap.bits.dpia_bw_alloc,
        (*link).dpcd_caps.usb4_dp_tun_info.driver_bw_cap.bits.driver_bw_alloc_support);

    status = core_link_read_dpcd(link, DP_USB4_ROUTER_TOPOLOGY_ID,
        dpcd_topology_data.as_mut_ptr(), dpcd_topology_data.len());
    if status != DC_OK {
        return status;
    }
    while i < DPCD_USB4_TOPOLOGY_ID_LEN {
        (*link).dpcd_caps.usb4_dp_tun_info.usb4_topology_id[i as usize] =
            dpcd_topology_data[i as usize];
        i += 1;
    }
    status
}

pub unsafe fn dpia_query_hpd_status(link: *mut dc_link) -> bool {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    let dmub_srv = (*link).ctx.dmub_srv;

    cmd.query_hpd.header.type_ = DMUB_CMD__QUERY_HPD_STATE;
    cmd.query_hpd.header.payload_bytes = core::mem::size_of_val(&cmd.query_hpd.data);
    cmd.query_hpd.data.instance = ((*link).link_id.enum_id - ENUM_ID_1) as u8;
    cmd.query_hpd.data.ch_type = AUX_CHANNEL_DPIA;

    if dc_wake_and_execute_dmub_cmd((*dmub_srv).ctx, &mut cmd,
        DM_DMUB_WAIT_TYPE_WAIT_WITH_REPLY) && cmd.query_hpd.data.status == AUX_RET_SUCCESS {
        DC_LOG_DEBUG!("{}: for link({}) dpia({}) success, current_hpd_status({}) new_hpd_status({})",
            "dpia_query_hpd_status", (*link).link_index,
            (*link).link_id.enum_id - ENUM_ID_1, (*link).hpd_status,
            cmd.query_hpd.data.result);
        (*link).hpd_status = cmd.query_hpd.data.result;
    } else {
        DC_LOG_ERROR!("{}: for link({}) dpia({}) failed with status({}), current_hpd_status({}) new_hpd_status(0)",
            "dpia_query_hpd_status", (*link).link_index,
            (*link).link_id.enum_id - ENUM_ID_1, cmd.query_hpd.data.status,
            (*link).hpd_status);
        (*link).hpd_status = false;
    }
    (*link).hpd_status
}

pub unsafe fn link_decide_dp_tunnel_settings(
    stream: *mut dc_stream_state,
    dp_tunnel_setting: *mut dc_tunnel_settings,
) {
    let link = (*stream).link;
    core::ptr::write_bytes(dp_tunnel_setting, 0, 1);

    if (*stream).signal == SIGNAL_TYPE_DISPLAY_PORT ||
        (*stream).signal == SIGNAL_TYPE_DISPLAY_PORT_MST {
        (*dp_tunnel_setting).should_enable_dp_tunneling =
            (*link).dpcd_caps.usb4_dp_tun_info.dp_tun_cap.bits.dp_tunneling;
        if (*link).dpcd_caps.usb4_dp_tun_info.dp_tun_cap.bits.dpia_bw_alloc &&
            (*link).dpcd_caps.usb4_dp_tun_info.driver_bw_cap.bits.driver_bw_alloc_support {
            (*dp_tunnel_setting).should_use_dp_bw_allocation = true;
            (*dp_tunnel_setting).cm_id = (*link).dpcd_caps.usb4_dp_tun_info.usb4_driver_id & 0x0f;
            (*dp_tunnel_setting).group_id = (*link).dpcd_caps.usb4_dp_tun_info.dpia_tunnel_info.bits.group_id;
            (*dp_tunnel_setting).estimated_bw = (*link).dpia_bw_alloc_config.estimated_bw;
            (*dp_tunnel_setting).allocated_bw = (*link).dpia_bw_alloc_config.allocated_bw;
            (*dp_tunnel_setting).bw_granularity = (*link).dpia_bw_alloc_config.bw_granularity;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
