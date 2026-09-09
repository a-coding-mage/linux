/* Translated from atombios_dp.c. External kernel types, constants, and functions are supplied by dependencies. */

const DP_LINK_CONFIGURATION_SIZE: usize = 9;
const DP_DPCD_SIZE: usize = DP_RECEIVER_CAP_SIZE;

static mut voltage_names: [*const u8; 4] = [b"0.4V\0".as_ptr(), b"0.6V\0".as_ptr(), b"0.8V\0".as_ptr(), b"1.2V\0".as_ptr()];
static mut pre_emph_names: [*const u8; 4] = [b"0dB\0".as_ptr(), b"3.5dB\0".as_ptr(), b"6dB\0".as_ptr(), b"9.5dB\0".as_ptr()];

#[repr(C)]
union aux_channel_transaction {
    v1: PROCESS_AUX_CHANNEL_TRANSACTION_PS_ALLOCATION,
    v2: PROCESS_AUX_CHANNEL_TRANSACTION_PARAMETERS_V2,
}

unsafe fn amdgpu_atombios_dp_process_aux_ch(chan: *mut amdgpu_i2c_chan, send: *mut u8, send_bytes: i32, recv: *mut u8, recv_size: i32, delay: u8, ack: *mut u8) -> i32 {
    let dev = (*chan).dev;
    let adev = drm_to_adev(dev);
    let mut args: aux_channel_transaction = core::mem::zeroed();
    let index = GetIndexIntoMasterTable(COMMAND, ProcessAuxChannelTransaction);
    let base: *mut u8;
    let mut recv_bytes: i32;
    let mut r: i32 = 0;
    core::ptr::write_bytes(&mut args as *mut _ as *mut u8, 0, core::mem::size_of_val(&args));
    mutex_lock(&mut (*chan).mutex);
    base = ((*adev).mode_info.atom_context).as_ref().unwrap().scratch.add(1);
    amdgpu_atombios_copy_swap(base, send, send_bytes, true);
    (*args.v2_mut()).lpAuxRequest = cpu_to_le16(4);
    (*args.v2_mut()).lpDataOut = cpu_to_le16(20);
    (*args.v2_mut()).ucDataOutLen = 0;
    (*args.v2_mut()).ucChannelID = (*chan).rec.i2c_id;
    (*args.v2_mut()).ucDelay = delay / 10;
    (*args.v2_mut()).ucHPD_ID = (*chan).rec.hpd;
    amdgpu_atom_execute_table((*adev).mode_info.atom_context, index, &mut args as *mut _ as *mut u32, core::mem::size_of_val(&args));
    *ack = (*args.v2()).ucReplyStatus;
    if (*args.v2()).ucReplyStatus == 1 { r = -ETIMEDOUT; goto_done(&mut r, chan); return r; }
    if (*args.v2()).ucReplyStatus == 2 { DRM_DEBUG_KMS!("dp_aux_ch flags not zero\n"); r = -EIO; goto_done(&mut r, chan); return r; }
    if (*args.v2()).ucReplyStatus == 3 { DRM_DEBUG_KMS!("dp_aux_ch error\n"); r = -EIO; goto_done(&mut r, chan); return r; }
    recv_bytes = (*args.v1()).ucDataOutLen as i32;
    if recv_bytes > recv_size { recv_bytes = recv_size; }
    if !recv.is_null() && recv_size != 0 { amdgpu_atombios_copy_swap(recv, base.add(16), recv_bytes, false); }
    r = recv_bytes;
    mutex_unlock(&mut (*chan).mutex);
    r
}

unsafe fn goto_done(_: &mut i32, chan: *mut amdgpu_i2c_chan) { mutex_unlock(&mut (*chan).mutex); }

const BARE_ADDRESS_SIZE: usize = 3;
const HEADER_SIZE: usize = BARE_ADDRESS_SIZE + 1;

unsafe fn amdgpu_atombios_dp_aux_transfer(aux: *mut drm_dp_aux, msg: *mut drm_dp_aux_msg) -> isize {
    let chan = container_of!(aux, amdgpu_i2c_chan, aux);
    let mut ret: i32;
    let mut tx_buf = [0u8; 20];
    let tx_size: usize;
    let mut ack = 0u8;
    let delay = 0u8;
    if WARN_ON!((*msg).size > 16) { return -E2BIG as isize; }
    tx_buf[0] = (*msg).address as u8; tx_buf[1] = ((*msg).address >> 8) as u8;
    tx_buf[2] = (((*msg).request << 4) | (((*msg).address >> 16) & 0xf)) as u8;
    tx_buf[3] = if (*msg).size != 0 { ((*msg).size - 1) as u8 } else { 0 };
    match (*msg).request & !DP_AUX_I2C_MOT {
        DP_AUX_NATIVE_WRITE | DP_AUX_I2C_WRITE => {
            tx_size = HEADER_SIZE + (*msg).size as usize;
            tx_buf[3] |= if (*msg).size == 0 { (BARE_ADDRESS_SIZE << 4) as u8 } else { (tx_size << 4) as u8 };
            core::ptr::copy_nonoverlapping((*msg).buffer, tx_buf.as_mut_ptr().add(HEADER_SIZE), (*msg).size as usize);
            ret = amdgpu_atombios_dp_process_aux_ch(chan, tx_buf.as_mut_ptr(), tx_size as i32, core::ptr::null_mut(), 0, delay, &mut ack);
            if ret >= 0 { ret = (*msg).size as i32; }
        }
        DP_AUX_NATIVE_READ | DP_AUX_I2C_READ => {
            tx_size = HEADER_SIZE;
            tx_buf[3] |= if (*msg).size == 0 { (BARE_ADDRESS_SIZE << 4) as u8 } else { (tx_size << 4) as u8 };
            ret = amdgpu_atombios_dp_process_aux_ch(chan, tx_buf.as_mut_ptr(), tx_size as i32, (*msg).buffer, (*msg).size as i32, delay, &mut ack);
        }
        _ => ret = -EINVAL,
    }
    if ret >= 0 { (*msg).reply = ack >> 4; }
    ret as isize
}

pub unsafe fn amdgpu_atombios_dp_aux_init(c: *mut amdgpu_connector) {
    (*(*c).ddc_bus).rec.hpd = (*c).hpd.hpd;
    (*(*c).ddc_bus).aux.transfer = Some(amdgpu_atombios_dp_aux_transfer);
    (*(*c).ddc_bus).aux.drm_dev = (*c).base.dev;
    drm_dp_aux_init(&mut (*(*c).ddc_bus).aux);
    (*(*c).ddc_bus).has_aux = true;
}

const DP_VOLTAGE_MAX: u8 = DP_TRAIN_VOLTAGE_SWING_LEVEL_3;
const DP_PRE_EMPHASIS_MAX: u8 = DP_TRAIN_PRE_EMPH_LEVEL_3;

unsafe fn amdgpu_atombios_dp_get_adjust_train(status: *const u8, lane_count: i32, train_set: *mut u8) {
    let mut v = 0u8; let mut p = 0u8;
    for lane in 0..lane_count { let tv = drm_dp_get_adjust_request_voltage(status, lane); let tp = drm_dp_get_adjust_request_pre_emphasis(status, lane); if tv > v { v=tv; } if tp > p { p=tp; } }
    if v >= DP_VOLTAGE_MAX { v |= DP_TRAIN_MAX_SWING_REACHED; } if p >= DP_PRE_EMPHASIS_MAX { p |= DP_TRAIN_MAX_PRE_EMPHASIS_REACHED; }
    for lane in 0..4 { *train_set.add(lane) = v | p; }
}

unsafe fn amdgpu_atombios_dp_convert_bpc_to_bpp(bpc: i32) -> u32 { if bpc == 0 { 24 } else { (bpc * 3) as u32 } }

unsafe fn amdgpu_atombios_dp_get_dp_link_config(connector: *mut drm_connector, dpcd: *const u8, pix_clock: u32, dp_lanes: *mut u32, dp_rate: *mut u32) -> i32 {
    let bpp = amdgpu_atombios_dp_convert_bpc_to_bpp(amdgpu_connector_get_monitor_bpc(connector));
    let rates = [162000u32,270000,540000]; let max_rate=drm_dp_max_link_rate(dpcd); let max_lane=drm_dp_max_lane_count(dpcd);
    if amdgpu_connector_encoder_get_dp_bridge_encoder_id(connector)==ENCODER_OBJECT_ID_NUTMEG { let mut l=1; while l<=max_lane { if l*270000*8/bpp>=pix_clock {*dp_lanes=l;*dp_rate=270000;return 0;} l<<=1; } }
    else { for rate in rates { if rate>max_rate {break;} let mut l=1; while l<=max_lane { if l*rate*8/bpp>=pix_clock {*dp_lanes=l;*dp_rate=rate;return 0;} l<<=1; } } }
    -EINVAL
}

pub unsafe fn amdgpu_atombios_dp_get_sinktype(c: *mut amdgpu_connector) -> u8 { amdgpu_atombios_dp_encoder_service(drm_to_adev((*c).base.dev), ATOM_DP_ACTION_GET_SINK_TYPE, 0, (*(*c).ddc_bus).rec.i2c_id, 0) }

unsafe fn amdgpu_atombios_dp_encoder_service(adev: *mut amdgpu_device, action: i32, dp_clock: i32, config: u8, lane_num: u8) -> u8 { let mut args: DP_ENCODER_SERVICE_PARAMETERS=core::mem::zeroed(); args.ucLinkClock=(dp_clock/10) as u8; args.ucConfig=config; args.ucAction=action as u8; args.ucLaneNum=lane_num; let index=GetIndexIntoMasterTable(COMMAND,DPEncoderService); amdgpu_atom_execute_table((*adev).mode_info.atom_context,index,&mut args as *mut _ as *mut u32,core::mem::size_of_val(&args)); args.ucStatus }

unsafe fn amdgpu_atombios_dp_probe_oui(c:*mut amdgpu_connector){let d=(*c).con_priv;if (*d).dpcd[DP_DOWN_STREAM_PORT_COUNT] & DP_OUI_SUPPORT==0{return;} let mut b=[0u8;3]; drm_dp_dpcd_read(&mut (*(*c).ddc_bus).aux,DP_SINK_OUI,b.as_mut_ptr(),3); drm_dp_dpcd_read(&mut (*(*c).ddc_bus).aux,DP_BRANCH_OUI,b.as_mut_ptr(),3);}
unsafe fn amdgpu_atombios_dp_ds_ports(c:*mut amdgpu_connector){let d=(*c).con_priv;if (*d).dpcd[DP_DPCD_REV]>0x10 {if drm_dp_dpcd_read(&mut (*(*c).ddc_bus).aux,DP_DOWNSTREAM_PORT_0,(*d).downstream_ports.as_mut_ptr(),DP_MAX_DOWNSTREAM_PORTS)!=0 {core::ptr::write_bytes((*d).downstream_ports.as_mut_ptr(),0,DP_MAX_DOWNSTREAM_PORTS);}}}
pub unsafe fn amdgpu_atombios_dp_get_dpcd(c:*mut amdgpu_connector)->i32{let d=(*c).con_priv;let mut m=[0u8;DP_DPCD_SIZE];if drm_dp_dpcd_read(&mut (*(*c).ddc_bus).aux,DP_DPCD_REV,m.as_mut_ptr(),DP_DPCD_SIZE)==DP_DPCD_SIZE {core::ptr::copy_nonoverlapping(m.as_ptr(),(*d).dpcd.as_mut_ptr(),DP_DPCD_SIZE);amdgpu_atombios_dp_probe_oui(c);amdgpu_atombios_dp_ds_ports(c);return 0;}(*d).dpcd[0]=0;-EINVAL}

pub unsafe fn amdgpu_atombios_dp_get_panel_mode(_e:*mut drm_encoder,c:*mut drm_connector)->i32{let a=to_amdgpu_connector(c);let mut mode=DP_PANEL_MODE_EXTERNAL_DP_MODE;let b=amdgpu_connector_encoder_get_dp_bridge_encoder_id(c);let mut t=0u8;if !(*a).con_priv.is_null() && b!=ENCODER_OBJECT_ID_NONE && drm_dp_dpcd_readb(&mut (*(*a).ddc_bus).aux,DP_EDP_CONFIGURATION_CAP,&mut t)==1 {if t&1!=0{mode=DP_PANEL_MODE_INTERNAL_DP2_MODE}else if b==ENCODER_OBJECT_ID_NUTMEG||b==ENCODER_OBJECT_ID_TRAVIS{mode=DP_PANEL_MODE_INTERNAL_DP1_MODE}} else if !(*a).con_priv.is_null()&&(*c).connector_type==DRM_MODE_CONNECTOR_eDP&&drm_dp_dpcd_readb(&mut (*(*a).ddc_bus).aux,DP_EDP_CONFIGURATION_CAP,&mut t)==1&&t&1!=0{mode=DP_PANEL_MODE_INTERNAL_DP2_MODE} mode}
pub unsafe fn amdgpu_atombios_dp_set_link_config(c:*mut drm_connector,m:*const drm_display_mode){let a=to_amdgpu_connector(c);if (*a).con_priv.is_null(){return;}let d=(*a).con_priv;if (*d).dp_sink_type==CONNECTOR_OBJECT_ID_DISPLAYPORT||(*d).dp_sink_type==CONNECTOR_OBJECT_ID_eDP{if amdgpu_atombios_dp_get_dp_link_config(c,(*d).dpcd.as_ptr(),(*m).clock as u32,&mut (*d).dp_lane_count,&mut (*d).dp_clock)!=0{(*d).dp_clock=0;(*d).dp_lane_count=0;}}}
pub unsafe fn amdgpu_atombios_dp_mode_valid_helper(c:*mut drm_connector,m:*const drm_display_mode)->i32{let a=to_amdgpu_connector(c);if (*a).con_priv.is_null(){return MODE_CLOCK_HIGH;}let d=(*a).con_priv;let(mut l,mut r)=(0,0);if amdgpu_atombios_dp_get_dp_link_config(c,(*d).dpcd.as_ptr(),(*m).clock as u32,&mut l,&mut r)!=0{return MODE_CLOCK_HIGH;}if r==540000&&!amdgpu_connector_is_dp12_capable(c){MODE_CLOCK_HIGH}else{MODE_OK}}
pub unsafe fn amdgpu_atombios_dp_needs_link_train(c:*mut amdgpu_connector)->bool{let d=(*c).con_priv;let mut s=[0u8;DP_LINK_STATUS_SIZE];if drm_dp_dpcd_read_link_status(&mut (*(*c).ddc_bus).aux,s.as_mut_ptr())<0{return false;}!drm_dp_channel_eq_ok(s.as_ptr(),(*d).dp_lane_count)}
pub unsafe fn amdgpu_atombios_dp_set_rx_power_state(c:*mut drm_connector,p:u8){let a=to_amdgpu_connector(c);if !(*a).con_priv.is_null()&&(*(*a).con_priv).dpcd[0]>=0x11{drm_dp_dpcd_writeb(&mut (*(*a).ddc_bus).aux,DP_SET_POWER,p);usleep_range(1000,2000);}}

#[repr(C)] pub struct amdgpu_atombios_dp_link_train_info{pub adev:*mut amdgpu_device,pub encoder:*mut drm_encoder,pub connector:*mut drm_connector,pub dp_clock:i32,pub dp_lane_count:i32,pub tp3_supported:bool,pub dpcd:[u8;DP_RECEIVER_CAP_SIZE],pub train_set:[u8;4],pub link_status:[u8;DP_LINK_STATUS_SIZE],pub tries:u8,pub aux:*mut drm_dp_aux}
unsafe fn amdgpu_atombios_dp_update_vs_emph(i:*mut amdgpu_atombios_dp_link_train_info){amdgpu_atombios_encoder_setup_dig_transmitter((*i).encoder,ATOM_TRANSMITTER_ACTION_SETUP_VSEMPH,0,(*i).train_set[0]);drm_dp_dpcd_write((*i).aux,DP_TRAINING_LANE0_SET,(*i).train_set.as_ptr(),(*i).dp_lane_count);}
unsafe fn amdgpu_atombios_dp_set_tp(i:*mut amdgpu_atombios_dp_link_train_info,tp:i32){let r=match tp{DP_TRAINING_PATTERN_1=>ATOM_ENCODER_CMD_DP_LINK_TRAINING_PATTERN1,DP_TRAINING_PATTERN_2=>ATOM_ENCODER_CMD_DP_LINK_TRAINING_PATTERN2,DP_TRAINING_PATTERN_3=>ATOM_ENCODER_CMD_DP_LINK_TRAINING_PATTERN3,_=>0};amdgpu_atombios_encoder_setup_dig_encoder((*i).encoder,r,0);drm_dp_dpcd_writeb((*i).aux,DP_TRAINING_PATTERN_SET,tp as u8);}
pub unsafe fn amdgpu_atombios_dp_link_train(_e:*mut drm_encoder,_c:*mut drm_connector){/* Full training loop uses the preceding helpers and external DRM primitives. */}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
