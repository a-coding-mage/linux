/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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
/*********************************************************************/
// USB4 DPIA BANDWIDTH ALLOCATION LOGIC
/*********************************************************************/

// Dependencies supplied by the surrounding translation unit.

const Kbps_TO_Gbps: u32 = 1000 * 1000;
const MST_TIME_SLOT_COUNT: u32 = 64;
const BW_GRANULARITY_0: u8 = 4; // 0.25 Gbps
const BW_GRANULARITY_1: u8 = 2; // 0.5 Gbps
const BW_GRANULARITY_2: u8 = 1; // 1 Gbps

unsafe fn link_dp_is_bw_alloc_available(link: *mut dc_link) -> bool {
    !link.is_null()
        && (*link).dpcd_caps.usb4_dp_tun_info.dp_tun_cap.bits.dp_tunneling
        && (*link).dpcd_caps.usb4_dp_tun_info.dp_tun_cap.bits.dpia_bw_alloc
        && (*link).dpcd_caps.usb4_dp_tun_info.driver_bw_cap.bits.driver_bw_alloc_support
}

unsafe fn reset_bw_alloc_struct(link: *mut dc_link) {
    (*link).dpia_bw_alloc_config.bw_alloc_enabled = false;
    (*link).dpia_bw_alloc_config.link_verified_bw = 0;
    (*link).dpia_bw_alloc_config.link_max_bw = 0;
    (*link).dpia_bw_alloc_config.allocated_bw = 0;
    (*link).dpia_bw_alloc_config.estimated_bw = 0;
    (*link).dpia_bw_alloc_config.bw_granularity = 0;
    (*link).dpia_bw_alloc_config.dp_overhead = 0;
    (*link).dpia_bw_alloc_config.nrd_max_lane_count = 0;
    (*link).dpia_bw_alloc_config.nrd_max_link_rate = 0;
    for i in 0..MAX_SINKS_PER_LINK {
        (*link).dpia_bw_alloc_config.remote_sink_req_bw[i] = 0;
    }
    DC_LOG_DEBUG("reset usb4 bw alloc of link(%d)\n", (*link).link_index);
}

unsafe fn get_bw_granularity(link: *mut dc_link) -> u8 {
    let mut bw_granularity: u8 = 0;
    core_link_read_dpcd(link, DP_BW_GRANULALITY, &mut bw_granularity, core::mem::size_of::<u8>());
    bw_granularity = match bw_granularity & 0x3 {
        0 => BW_GRANULARITY_0,
        1 => BW_GRANULARITY_1,
        _ => BW_GRANULARITY_2,
    };
    bw_granularity
}

unsafe fn get_estimated_bw(link: *mut dc_link) -> i32 {
    let mut bw_estimated_bw: u8 = 0;
    core_link_read_dpcd(link, ESTIMATED_BW, &mut bw_estimated_bw, core::mem::size_of::<u8>());
    bw_estimated_bw as i32 * (Kbps_TO_Gbps / (*link).dpia_bw_alloc_config.bw_granularity as u32) as i32
}

unsafe fn get_non_reduced_max_link_rate(link: *mut dc_link) -> u8 {
    let mut value: u8 = 0;
    core_link_read_dpcd(link, DP_TUNNELING_MAX_LINK_RATE, &mut value, core::mem::size_of::<u8>());
    value
}

unsafe fn get_non_reduced_max_lane_count(link: *mut dc_link) -> u8 {
    let mut value: u8 = 0;
    core_link_read_dpcd(link, DP_TUNNELING_MAX_LANE_COUNT, &mut value, core::mem::size_of::<u8>());
    value
}

unsafe fn retrieve_usb4_dp_bw_allocation_info(link: *mut dc_link) {
    reset_bw_alloc_struct(link);
    (*link).dpia_bw_alloc_config.bw_granularity = get_bw_granularity(link);
    (*link).dpia_bw_alloc_config.estimated_bw = get_estimated_bw(link);
    (*link).dpia_bw_alloc_config.nrd_max_link_rate = get_non_reduced_max_link_rate(link);
    (*link).dpia_bw_alloc_config.nrd_max_lane_count = get_non_reduced_max_lane_count(link);
    DC_LOG_DEBUG!("%s: bw_granularity(%d), estimated_bw(%d)\n", __func__, (*link).dpia_bw_alloc_config.bw_granularity, (*link).dpia_bw_alloc_config.estimated_bw);
    DC_LOG_DEBUG!("%s: nrd_max_link_rate(%d), nrd_max_lane_count(%d)\n", __func__, (*link).dpia_bw_alloc_config.nrd_max_link_rate, (*link).dpia_bw_alloc_config.nrd_max_lane_count);
}

unsafe fn dpia_bw_alloc_unplug(link: *mut dc_link) {
    if !link.is_null() { DC_LOG_DEBUG!("%s: resetting BW alloc config for link(%d)\n", __func__, (*link).link_index); reset_bw_alloc_struct(link); }
}

unsafe fn link_dpia_send_bw_alloc_request(link: *mut dc_link, mut req_bw: i32) {
    if (*link).dpia_bw_alloc_config.bw_granularity == 0 { DC_LOG_ERROR!("%s:  Link[%d]:  bw_granularity is zero!", __func__, (*link).link_index); return; }
    let mut temp = req_bw as u32 * (*link).dpia_bw_alloc_config.bw_granularity as u32;
    let mut request_reg_val = temp / Kbps_TO_Gbps;
    if temp % Kbps_TO_Gbps != 0 { request_reg_val += 1; }
    let request_bw = request_reg_val * (Kbps_TO_Gbps / (*link).dpia_bw_alloc_config.bw_granularity as u32);
    if request_bw > (*link).dpia_bw_alloc_config.estimated_bw as u32 {
        DC_LOG_ERROR!("%s:  Link[%d]:  Request BW (%d --> %d) > Estimated BW (%d)... Set to Estimated BW!", __func__, (*link).link_index, req_bw, request_bw, (*link).dpia_bw_alloc_config.estimated_bw);
        req_bw = (*link).dpia_bw_alloc_config.estimated_bw;
        temp = req_bw as u32 * (*link).dpia_bw_alloc_config.bw_granularity as u32;
        request_reg_val = temp / Kbps_TO_Gbps;
        if temp % Kbps_TO_Gbps != 0 { request_reg_val += 1; }
    }
    (*link).dpia_bw_alloc_config.allocated_bw = request_bw as i32;
    DC_LOG_DC!("%s:  Link[%d]:  Request BW:  %d", __func__, (*link).link_index, request_bw);
    let mut requested_bw_dpcd = request_reg_val as u8;
    core_link_write_dpcd(link, REQUESTED_BW, &mut requested_bw_dpcd, core::mem::size_of::<u8>());
}

pub unsafe fn link_dpia_enable_usb4_dp_bw_alloc_mode(link: *mut dc_link) -> bool {
    let mut ret = false;
    let mut val = DPTX_BW_ALLOC_MODE_ENABLE | DPTX_BW_ALLOC_UNMASK_IRQ;
    if core_link_write_dpcd(link, DPTX_BW_ALLOCATION_MODE_CONTROL, &mut val, core::mem::size_of::<u8>()) == DC_OK {
        DC_LOG_DEBUG!("%s:  link[%d] DPTX BW allocation mode enabled", __func__, (*link).link_index);
        retrieve_usb4_dp_bw_allocation_info(link);
        if (*link).dpia_bw_alloc_config.nrd_max_link_rate != 0 && (*link).dpia_bw_alloc_config.nrd_max_lane_count != 0 { (*link).reported_link_cap.link_rate = (*link).dpia_bw_alloc_config.nrd_max_link_rate; (*link).reported_link_cap.lane_count = (*link).dpia_bw_alloc_config.nrd_max_lane_count; }
        (*link).dpia_bw_alloc_config.bw_alloc_enabled = true;
        ret = true;
        if (*link).dc.debug.dpia_debug.bits.enable_usb4_bw_zero_alloc_patch { link_dp_dpia_allocate_usb4_bandwidth_for_stream(link, 0); }
    } else { DC_LOG_DEBUG!("%s:  link[%d] failed to enable DPTX BW allocation mode", __func__, (*link).link_index); }
    ret
}

pub unsafe fn link_dp_dpia_handle_bw_alloc_status(link: *mut dc_link, status: u8) {
    if status & DP_TUNNELING_BW_REQUEST_SUCCEEDED != 0 { DC_LOG_DEBUG!("%s: BW Allocation request succeeded on link(%d)", __func__, (*link).link_index); }
    if status & DP_TUNNELING_BW_REQUEST_FAILED != 0 { DC_LOG_DEBUG!("%s: BW Allocation request failed on link(%d)  allocated/estimated BW=%d", __func__, (*link).link_index, (*link).dpia_bw_alloc_config.estimated_bw); link_dpia_send_bw_alloc_request(link, (*link).dpia_bw_alloc_config.estimated_bw); }
    if status & DP_TUNNELING_BW_ALLOC_CAP_CHANGED != 0 { (*link).dpia_bw_alloc_config.bw_granularity = get_bw_granularity(link); DC_LOG_DEBUG!("%s: Granularity changed on link(%d)  new granularity=%d", __func__, (*link).link_index, (*link).dpia_bw_alloc_config.bw_granularity); }
    if status & DP_TUNNELING_ESTIMATED_BW_CHANGED != 0 { (*link).dpia_bw_alloc_config.estimated_bw = get_estimated_bw(link); DC_LOG_DEBUG!("%s: Estimated BW changed on link(%d)  new estimated BW=%d", __func__, (*link).link_index, (*link).dpia_bw_alloc_config.estimated_bw); }
    core_link_write_dpcd(link, DP_TUNNELING_STATUS, &mut (status as u8), core::mem::size_of::<u8>());
}

pub unsafe fn dpia_handle_usb4_bandwidth_allocation_for_link(link: *mut dc_link, peak_bw: i32) {
    if !link.is_null() && (*link).dpcd_caps.usb4_dp_tun_info.dp_tun_cap.bits.dp_tunneling && (*link).dpia_bw_alloc_config.bw_alloc_enabled { if peak_bw > 0 { (*link).dpia_bw_alloc_config.link_max_bw = peak_bw; link_dpia_send_bw_alloc_request(link, peak_bw); } else { dpia_bw_alloc_unplug(link); } }
}

pub unsafe fn link_dp_dpia_allocate_usb4_bandwidth_for_stream(link: *mut dc_link, req_bw: i32) {
    (*link).dpia_bw_alloc_config.estimated_bw = get_estimated_bw(link);
    DC_LOG_DEBUG!("%s: ENTER: link[%d] hpd(%d)  Allocated_BW: %d  Estimated_BW: %d  Req_BW: %d", __func__, (*link).link_index, (*link).hpd_status, (*link).dpia_bw_alloc_config.allocated_bw, (*link).dpia_bw_alloc_config.estimated_bw, req_bw);
    if link_dp_is_bw_alloc_available(link) { link_dpia_send_bw_alloc_request(link, req_bw); } else { DC_LOG_DEBUG!("%s:  BW Allocation mode not available", __func__); }
}

pub unsafe fn link_dpia_get_dp_overhead(link: *const dc_link) -> u32 {
    let mut overhead = 0;
    if (*link).type_ == dc_connection_mst_branch && !(*link).dpcd_caps.channel_coding_cap.bits.DP_128b_132b_SUPPORTED {
        let link_cap = dc_link_get_link_cap(link);
        if !link_cap.is_null() { let bw = link_cap.link_rate as u32 * link_cap.lane_count as u32 * LINK_RATE_REF_FREQ_IN_KHZ * 8; overhead = bw / MST_TIME_SLOT_COUNT + if bw % MST_TIME_SLOT_COUNT != 0 { 1 } else { 0 }; }
    }
    overhead
}

pub unsafe fn link_dpia_validate_dp_tunnel_bandwidth(dpia_link_sets: *const dc_validation_dpia_set, count: u8) -> bool {
    let mut router_sets: [usb4_router_validation_set; MAX_HOST_ROUTERS_NUM] = [core::mem::zeroed(); MAX_HOST_ROUTERS_NUM];
    let mut success = true;
    let mut router_count: u8 = 0;
    if dpia_link_sets.is_null() || count == 0 { return success; }
    for i in 0..count as usize {
        let link = (*dpia_link_sets.add(i)).link;
        let mut required = (*dpia_link_sets.add(i)).required_bw;
        let settings = (*dpia_link_sets.add(i)).tunnel_settings;
        if link.is_null() || settings.is_null() || (*settings).bw_granularity == 0 { break; }
        if (*link).type_ == dc_connection_mst_branch { required += link_dpia_get_dp_overhead(link); }
        let granularity = Kbps_TO_Gbps / (*settings).bw_granularity;
        let link_bw = (required / granularity) * granularity + if required % granularity != 0 { granularity } else { 0 };
        for j in 0..MAX_HOST_ROUTERS_NUM { if !router_sets[j].is_valid { router_sets[j].is_valid = true; router_sets[j].cm_id = (*settings).cm_id; router_count += 1; } if router_sets[j].cm_id == (*settings).cm_id { let remaining = (*settings).estimated_bw - (*settings).allocated_bw; router_sets[j].allocated_bw += (*settings).allocated_bw; if remaining > router_sets[j].remaining_bw { router_sets[j].remaining_bw = remaining; } if (*settings).estimated_bw > router_sets[j].estimated_bw { router_sets[j].estimated_bw = (*settings).estimated_bw; } router_sets[j].required_bw += link_bw; router_sets[j].dpia_count += 1; break; } }
    }
    for i in 0..router_count as usize { if !router_sets[i].is_valid { break; } let total = if router_sets[i].dpia_count == 1 || router_sets[i].allocated_bw == 0 { router_sets[i].estimated_bw } else { router_sets[i].allocated_bw + router_sets[i].remaining_bw }; if router_sets[i].required_bw > total { success = false; break; } }
    success
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
