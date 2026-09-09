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

// External declarations and constants are supplied by the translated dependency files.

static const DP_VGA_DONGLE_BRANCH_DEV_NAME: &[u8] = b"DpVga\0";
static const DP_DVI_CONVERTER_ID_4: &[u8] = b"m2DVIa\0";
static const DP_DVI_CONVERTER_ID_5: &[u8] = b"3393N2\0";

#[repr(C)]
struct i2c_payloads { payloads: vector }

unsafe fn i2c_payloads_create(ctx: *mut dc_context, payloads: *mut i2c_payloads, count: u32) -> bool {
    if dal_vector_construct(&mut (*payloads).payloads, ctx, count, core::mem::size_of::<i2c_payload>()) { true } else { false }
}

unsafe fn i2c_payloads_get(p: *mut i2c_payloads) -> *mut i2c_payload { (*p).payloads.container as *mut i2c_payload }
unsafe fn i2c_payloads_get_count(p: *mut i2c_payloads) -> u32 { (*p).payloads.count }
unsafe fn i2c_payloads_destroy(p: *mut i2c_payloads) { if !p.is_null() { dal_vector_destruct(&mut (*p).payloads); } }

#[inline] unsafe fn ddc_min<T: Ord>(a: T, b: T) -> T { if a < b { a } else { b } }

unsafe fn i2c_payloads_add(payloads: *mut i2c_payloads, address: u32, len: u32, data: *mut u8, write: bool) {
    let payload_size: u32 = EDID_SEGMENT_SIZE;
    let mut pos = 0;
    while pos < len {
        let payload = i2c_payload { write, address: address as u8, length: ddc_min(payload_size, len - pos), data: data.add(pos as usize) };
        dal_vector_append(&mut (*payloads).payloads, &payload);
        pos += payload_size;
    }
}

unsafe fn ddc_service_construct(ddc_service: *mut ddc_service, init_data: *mut ddc_service_init_data) {
    let connector_id = dal_graphics_object_id_get_connector_id((*init_data).id);
    let gpio_service = (*(*init_data).ctx).gpio_service;
    let mut i2c_info: graphics_object_i2c_info = core::mem::zeroed();
    let mut hw_info: gpio_ddc_hw_info = core::mem::zeroed();
    let dcb = (*(*init_data).ctx).dc_bios;
    (*ddc_service).link = (*init_data).link;
    (*ddc_service).ctx = (*init_data).ctx;
    if !(*ddc_service).link.is_null() && (*(*ddc_service).link).force_to_use_aux {
        if (*(*dcb).funcs).get_connector_aux_info(dcb, (*init_data).id, &mut i2c_info) == BP_RESULT_OK { (*(*ddc_service).link).aux_hw_inst = i2c_info.i2c_line as u8; }
    } else if (*init_data).is_dpia_link || (*(*dcb).funcs).get_i2c_info(dcb, (*init_data).id, &mut i2c_info) != BP_RESULT_OK {
        (*ddc_service).ddc_pin = core::ptr::null_mut();
    } else {
        hw_info.ddc_channel = i2c_info.i2c_line;
        hw_info.hw_supported = if !(*ddc_service).link.is_null() { i2c_info.i2c_hw_assist } else { false };
        (*ddc_service).ddc_pin = dal_gpio_create_ddc(gpio_service, i2c_info.gpio_info.clk_a_register_index, 1u32 << i2c_info.gpio_info.clk_a_shift, &mut hw_info);
    }
    (*ddc_service).flags.EDID_QUERY_DONE_ONCE = false;
    (*ddc_service).flags.FORCE_READ_REPEATED_START = false;
    (*ddc_service).flags.EDID_STRESS_READ = false;
    (*ddc_service).flags.IS_INTERNAL_DISPLAY = connector_id == CONNECTOR_ID_EDP || connector_id == CONNECTOR_ID_LVDS;
    (*ddc_service).wa.raw = 0;
}

pub unsafe fn link_create_ddc_service(init_data: *mut ddc_service_init_data) -> *mut ddc_service {
    let ddc_service = kzalloc_obj::<ddc_service>();
    if ddc_service.is_null() { return core::ptr::null_mut(); }
    ddc_service_construct(ddc_service, init_data); ddc_service
}
unsafe fn ddc_service_destruct(ddc: *mut ddc_service) { if !(*ddc).ddc_pin.is_null() { dal_gpio_destroy_ddc(&mut (*ddc).ddc_pin); } }
pub unsafe fn link_destroy_ddc_service(ddc: *mut *mut ddc_service) { if ddc.is_null() || (*ddc).is_null() { BREAK_TO_DEBUGGER!(); return; } ddc_service_destruct(*ddc); kfree(*ddc); *ddc = core::ptr::null_mut(); }
pub unsafe fn set_ddc_transaction_type(ddc: *mut ddc_service, ty: ddc_transaction_type) { (*ddc).transaction_type = ty; }
pub unsafe fn link_is_in_aux_transaction_mode(ddc: *mut ddc_service) -> bool { match (*ddc).transaction_type { DDC_TRANSACTION_TYPE_I2C_OVER_AUX | DDC_TRANSACTION_TYPE_I2C_OVER_AUX_WITH_DEFER | DDC_TRANSACTION_TYPE_I2C_OVER_AUX_RETRY_DEFER => true, _ => false } }
pub unsafe fn set_dongle_type(ddc: *mut ddc_service, dongle_type: display_dongle_type) { (*ddc).dongle_type = dongle_type; }

unsafe fn defer_delay_converter_wa(ddc: *mut ddc_service, defer_delay: u32) -> u32 {
    let link = (*ddc).link; let caps = &(*link).dpcd_caps;
    if caps.dongle_type == DISPLAY_DONGLE_DP_VGA_CONVERTER && caps.branch_dev_id == DP_BRANCH_DEVICE_ID_0080E1 && (caps.branch_fw_revision[0] < 1 || (caps.branch_fw_revision[0] == 1 && caps.branch_fw_revision[1] < 0x40)) && !memcmp(caps.branch_dev_name.as_ptr(), DP_VGA_DONGLE_BRANCH_DEV_NAME.as_ptr(), core::mem::size_of_val(&caps.branch_dev_name)) { return if defer_delay > DPVGA_DONGLE_AUX_DEFER_WA_DELAY { defer_delay } else { DPVGA_DONGLE_AUX_DEFER_WA_DELAY }; }
    if caps.branch_dev_id == DP_BRANCH_DEVICE_ID_0080E1 && !memcmp(caps.branch_dev_name.as_ptr(), DP_DVI_CONVERTER_ID_4.as_ptr(), core::mem::size_of_val(&caps.branch_dev_name)) { return if defer_delay > I2C_OVER_AUX_DEFER_WA_DELAY { defer_delay } else { I2C_OVER_AUX_DEFER_WA_DELAY }; }
    if caps.branch_dev_id == DP_BRANCH_DEVICE_ID_006037 && !memcmp(caps.branch_dev_name.as_ptr(), DP_DVI_CONVERTER_ID_5.as_ptr(), core::mem::size_of_val(&caps.branch_dev_name)) { return if defer_delay > I2C_OVER_AUX_DEFER_WA_DELAY_1MS { I2C_OVER_AUX_DEFER_WA_DELAY_1MS } else { defer_delay }; }
    defer_delay
}

const DP_TRANSLATOR_DELAY: u32 = 5;

pub unsafe fn link_get_ddc_aux_inst(link: *const dc_link) -> u8 { if (*link).force_to_use_aux { return (*link).aux_hw_inst; } ASSERT!((*(*link).ddc).ddc_pin.hw_info.ddc_channel <= 0xFF); (*(*link).ddc).ddc_pin.hw_info.ddc_channel as u8 }
pub unsafe fn link_get_aux_defer_delay(ddc: *mut ddc_service) -> u32 { match (*ddc).transaction_type { DDC_TRANSACTION_TYPE_I2C_OVER_AUX => if (*ddc).dongle_type == DISPLAY_DONGLE_DP_VGA_CONVERTER || (*ddc).dongle_type == DISPLAY_DONGLE_DP_DVI_CONVERTER || (*ddc).dongle_type == DISPLAY_DONGLE_DP_HDMI_CONVERTER { defer_delay_converter_wa(ddc, DP_TRANSLATOR_DELAY) } else { 0 }, DDC_TRANSACTION_TYPE_I2C_OVER_AUX_WITH_DEFER => DP_TRANSLATOR_DELAY, _ => 0 } }

unsafe fn submit_aux_command(ddc: *mut ddc_service, payload: *mut aux_payload) -> bool {
    if ddc.is_null() || payload.is_null() { return false; }
    let mut retrieved = 0; let mut ret;
    loop {
        let end = retrieved + DEFAULT_AUX_MAX_DATA_SIZE >= (*payload).length;
        let length = if end { (*payload).length - retrieved } else { DEFAULT_AUX_MAX_DATA_SIZE };
        let mut current = aux_payload { address: (*payload).address, data: (*payload).data.add(retrieved as usize), defer_delay: (*payload).defer_delay, i2c_over_aux: (*payload).i2c_over_aux, length, mot: if end { (*payload).mot } else { true }, write_status_update: false, reply: (*payload).reply, write: (*payload).write };
        ret = link_aux_transfer_with_retries_no_mutex(ddc, &mut current); retrieved += length;
        if !(retrieved < (*payload).length && ret) { break; }
    } ret
}

pub unsafe fn link_query_ddc_data(ddc: *mut ddc_service, address: u32, write_buf: *mut u8, write_size: u32, read_buf: *mut u8, read_size: u32) -> bool {
    let mut success = true; let payload_size = if link_is_in_aux_transaction_mode(ddc) { DEFAULT_AUX_MAX_DATA_SIZE } else { EDID_SEGMENT_SIZE };
    let write_payloads = (write_size + payload_size - 1) / payload_size; let read_payloads = (read_size + payload_size - 1) / payload_size; let payloads_num = write_payloads + read_payloads;
    if payloads_num == 0 { return false; }
    if link_is_in_aux_transaction_mode(ddc) {
        let mut payload: aux_payload = core::mem::zeroed(); payload.i2c_over_aux = true; payload.address = address; payload.defer_delay = link_get_aux_defer_delay(ddc);
        if write_size != 0 { payload.write = true; payload.mot = read_size != 0; payload.length = write_size; payload.data = write_buf; success = submit_aux_command(ddc, &mut payload); }
        if read_size != 0 && success { payload.write = false; payload.mot = false; payload.length = read_size; payload.data = read_buf; success = submit_aux_command(ddc, &mut payload); }
    } else {
        let mut payloads: i2c_payloads = core::mem::zeroed(); if !i2c_payloads_create((*ddc).ctx, &mut payloads, payloads_num) { return false; }
        let mut command: i2c_command = core::mem::zeroed(); command.payloads = i2c_payloads_get(&mut payloads); command.engine = DDC_I2C_COMMAND_ENGINE; command.speed = (*(*(*ddc).ctx).dc).caps.i2c_speed_in_khz;
        i2c_payloads_add(&mut payloads, address, write_size, write_buf, true); i2c_payloads_add(&mut payloads, address, read_size, read_buf, false); command.number_of_payloads = i2c_payloads_get_count(&mut payloads) as u8;
        success = dm_helpers_submit_i2c((*ddc).ctx, (*ddc).link, &mut command); i2c_payloads_destroy(&mut payloads);
    } success
}

pub unsafe fn link_aux_transfer_raw(ddc: *mut ddc_service, payload: *mut aux_payload, operation_result: *mut aux_return_code_type) -> i32 { if (*(*ddc).ctx).dc.config.dp_connector_no_native_i2c && (*(*ddc).link).no_ddc_pin { if (*(*(*ddc).ctx).dc).debug.enable_dmub_aux_for_legacy_ddc { dce_aux_transfer_dmub_raw(ddc, payload, operation_result) } else { dce_aux_transfer_raw_without_ddc_pin(ddc, payload, operation_result) } } else { dce_aux_transfer_raw(ddc, payload, operation_result) } }

pub unsafe fn link_get_fixed_vs_pe_retimer_write_address(link: *mut dc_link) -> u32 { let mut address = 0xF004F; let offset = match (*link).dpcd_caps.lttpr_caps.phy_repeater_cnt { 0x80=>1,0x40=>2,0x20=>3,0x10=>4,0x08=>5,0x04=>6,0x02=>7,0x01=>8,_=>0xFF }; if offset != 0xFF { address += DP_REPEATER_CONFIGURATION_AND_STATUS_SIZE * (offset - 1); } address }
pub unsafe fn link_get_fixed_vs_pe_retimer_read_address(link: *mut dc_link) -> u32 { link_get_fixed_vs_pe_retimer_write_address(link) + 4 }

pub unsafe fn link_configure_fixed_vs_pe_retimer(ddc: *mut ddc_service, data: *const u8, length: u32) -> bool { let mut p = aux_payload { i2c_over_aux:false, write:true, address:link_get_fixed_vs_pe_retimer_write_address((*ddc).link), length, data:data as *mut u8, reply:core::ptr::null_mut(), mot:I2C_MOT_UNDEF, write_status_update:false, defer_delay:0 }; link_aux_transfer_with_retries_no_mutex(ddc, &mut p) }
pub unsafe fn link_query_fixed_vs_pe_retimer(ddc: *mut ddc_service, data: *mut u8, length: u32) -> bool { let mut p = aux_payload { i2c_over_aux:false, write:false, address:link_get_fixed_vs_pe_retimer_read_address((*ddc).link), length, data, reply:core::ptr::null_mut(), mot:I2C_MOT_UNDEF, write_status_update:false, defer_delay:0 }; link_aux_transfer_with_retries_no_mutex(ddc, &mut p) }
pub unsafe fn link_aux_transfer_with_retries_no_mutex(ddc: *mut ddc_service, payload: *mut aux_payload) -> bool { dce_aux_transfer_with_retries(ddc, payload) }

pub unsafe fn try_to_configure_aux_timeout(ddc: *mut ddc_service, mut timeout: u32) -> bool { let mut result=false; let ddc_pin=(*ddc).ddc_pin; if ((*(*ddc).link).chip_caps & AMD_EXT_DISPLAY_PATH_CAPS__EXT_CHIP_MASK)==AMD_EXT_DISPLAY_PATH_CAPS__DP_FIXED_VS_EN && !(*(*(*ddc).link).dc).debug.disable_fixed_vs_aux_timeout_wa && (*(*ddc).ctx).dce_version==DCN_VERSION_3_1 { let data=[1,0x22,0x63,0xc]; core_link_write_dpcd((*ddc).link,0xF004F,data.as_ptr(),data.len()); timeout=3072; } if (*(*ddc).link).ep_type != DISPLAY_ENDPOINT_PHY { return true; } if (*(*ddc).link).force_to_use_aux { if !(*(*(*(*ddc).ctx).dc).res_pool.engines[(*(*ddc).link).aux_hw_inst as usize]).funcs.configure_timeout.is_none() { (*(*(*(*ddc).ctx).dc).res_pool.engines[(*(*ddc).link).aux_hw_inst as usize]).funcs.configure_timeout.unwrap()(ddc,timeout); result=true; } } else if !(*(*(*(*ddc).ctx).dc).res_pool.engines[(*ddc_pin).pin_data.en as usize]).funcs.configure_timeout.is_none() { (*(*(*(*ddc).ctx).dc).res_pool.engines[(*ddc_pin).pin_data.en as usize]).funcs.configure_timeout.unwrap()(ddc,timeout); result=true; } result }
pub unsafe fn get_ddc_pin(ddc_service: *mut ddc_service) -> *mut ddc { (*ddc_service).ddc_pin }

pub unsafe fn write_scdc_data(ddc: *mut ddc_service, pix_clk:u32, lte_340_scramble:bool) { let over=pix_clk>340000; let slave=HDMI_SCDC_ADDRESS; let mut offset=HDMI_SCDC_SINK_VERSION; let mut version=0; let mut buf=[0u8;2]; if !(*ddc).link.is_null() && (*(*ddc).link).local_sink != core::ptr::null_mut() && ((*(*(*ddc).link).local_sink).edid_caps.panel_patch.skip_scdc_overwrite || !(*(*(*ddc).link).local_sink).edid_caps.scdc_present) { return; } hdmi_frl_LTS_clear_Link_Setting(ddc); hdmi_frl_LTS_clear_Update_flag(ddc); link_query_ddc_data(ddc,slave,&mut offset,1,&mut version,1); if version==1 { buf[0]=HDMI_SCDC_SOURCE_VERSION; buf[1]=1; link_query_ddc_data(ddc,slave,buf.as_mut_ptr(),2,core::ptr::null_mut(),0); } buf[0]=HDMI_SCDC_TMDS_CONFIG; buf[1]=if over {3} else if lte_340_scramble {1} else {0}; link_query_ddc_data(ddc,slave,buf.as_mut_ptr(),2,core::ptr::null_mut(),0); }
pub unsafe fn read_scdc_data(ddc:*mut ddc_service) { let slave=HDMI_SCDC_ADDRESS; let mut offset=HDMI_SCDC_TMDS_CONFIG; let mut config=0; if !(*ddc).link.is_null() && (*(*ddc).link).local_sink != core::ptr::null_mut() && (*(*(*ddc).link).local_sink).edid_caps.panel_patch.skip_scdc_overwrite { return; } link_query_ddc_data(ddc,slave,&mut offset,1,&mut config,1); if config&1 != 0 { let mut status=hdmi_scdc_status_flags_data{byte:0}; let mut scramble=0; offset=HDMI_SCDC_SCRAMBLER_STATUS; link_query_ddc_data(ddc,slave,&mut offset,1,&mut scramble,1); offset=HDMI_SCDC_STATUS_FLAGS; link_query_ddc_data(ddc,slave,&mut offset,1,&mut status.byte,1); } }

pub unsafe fn write_idcc_data(ddc:*mut ddc_service, scope:hdmi_idcc_scope, write_buf:*mut u8, offset:u8, write_len:u8) { let slave=HDMI_IDCC_ADDRESS; let mut header=[0u8;5]; let mut dummy=0; let mut checksum=0u8; header[0]=HDMI_IDCC_MARKER0; header[1]=HDMI_IDCC_MARKER1; header[2]=HDMI_IDCC_MARKER2|scope as u8; header[3]=offset; header[4]=write_len; for i in 0..5 { link_query_ddc_data(ddc,slave,&mut header[i],1,&mut dummy,1); checksum=checksum.wrapping_add(header[i]); } for i in 0..write_len as usize { link_query_ddc_data(ddc,slave,write_buf.add(i),1,&mut dummy,1); checksum=checksum.wrapping_add(*write_buf.add(i)); } if write_len>0 { checksum=0xffu8.wrapping_sub(checksum).wrapping_add(1); link_query_ddc_data(ddc,slave,&mut checksum,1,&mut dummy,1); } }

pub unsafe fn read_idcc_data(ddc:*mut ddc_service, scope:hdmi_idcc_scope, read_buf:*mut u8, offset:u8, mut read_len:u8) -> i32 { let slave=HDMI_IDCC_ADDRESS; let mut header=[0u8;5]; let mut dummy=0; let mut local=[0u8;6]; let mut checksum=0u8; header[0]=HDMI_IDCC_MARKER0; header[1]=HDMI_IDCC_MARKER1; header[2]=HDMI_IDCC_MARKER2|scope as u8; header[3]=offset; header[4]=read_len; for i in 0..5 { link_query_ddc_data(ddc,slave,&mut header[i],1,&mut dummy,1); checksum=checksum.wrapping_add(header[i]); } if read_len>0 { dummy=1; if read_len>5 { read_len=5; } link_query_ddc_data(ddc,slave,&mut dummy,1,local.as_mut_ptr(),read_len as u32+1); core::ptr::copy_nonoverlapping(local.as_ptr(),read_buf,read_len as usize); checksum=local[read_len as usize]; for i in 0..5 { checksum=checksum.wrapping_add(header[i]); } for i in 0..read_len as usize { checksum=checksum.wrapping_add(local[i]); } if checksum!=0 { return -1; } } read_len as i32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
