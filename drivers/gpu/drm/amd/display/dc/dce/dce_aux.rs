/* Faithful low-level translation of dce_aux.c.  Types, register helpers, and
 * external services are supplied by the surrounding display driver. */

const AUX_INVALID_REPLY_RETRY_COUNTER: u32 = 1;
const AUX_TIMED_OUT_RETRY_COUNTER: u32 = 2;
const AUX_DEFER_RETRY_COUNTER: u32 = 6;
const TIME_OUT_INCREMENT: u32 = 1016;
const TIME_OUT_MULTIPLIER_8: u32 = 8;
const TIME_OUT_MULTIPLIER_16: u32 = 16;
const TIME_OUT_MULTIPLIER_32: u32 = 32;
const TIME_OUT_MULTIPLIER_64: u32 = 64;
const MAX_TIMEOUT_LENGTH: u32 = 127;
const DEFAULT_AUX_ENGINE_MULT: u32 = 0;
const DEFAULT_AUX_ENGINE_LENGTH: u32 = 69;
const SW_CAN_ACCESS_AUX: u32 = 1;
const DMCU_CAN_ACCESS_AUX: u32 = 2;

unsafe fn release_engine(engine: *mut dce_aux) {
    let aux110 = FROM_AUX_ENGINE!(engine);
    dal_ddc_close((*engine).ddc);
    (*engine).ddc = core::ptr::null_mut();
    REG_UPDATE_2!(aux110, AUX_ARB_CONTROL, AUX_SW_DONE_USING_AUX_REG, 1,
                  AUX_SW_USE_AUX_REG_REQ, 0);
}

unsafe fn is_engine_available(engine: *mut dce_aux) -> bool {
    let aux110 = FROM_AUX_ENGINE!(engine);
    let value = REG_READ!(aux110, AUX_ARB_CONTROL);
    let field = get_reg_field_value(value, AUX_ARB_CONTROL, AUX_REG_RW_CNTL_STATUS);
    field != DMCU_CAN_ACCESS_AUX
}

unsafe fn acquire_engine(engine: *mut dce_aux) -> bool {
    let aux110 = FROM_AUX_ENGINE!(engine);
    let mut value = REG_READ!(aux110, AUX_ARB_CONTROL);
    let mut field = get_reg_field_value(value, AUX_ARB_CONTROL, AUX_REG_RW_CNTL_STATUS);
    if field == DMCU_CAN_ACCESS_AUX { return false; }
    value = REG_READ!(aux110, AUX_CONTROL);
    field = get_reg_field_value(value, AUX_CONTROL, AUX_EN);
    if field == 0 {
        set_reg_field_value!(value, 1, AUX_CONTROL, AUX_EN);
        if REG!(aux110, AUX_RESET_MASK) { set_reg_field_value!(value, 1, AUX_CONTROL, AUX_RESET); }
        REG_WRITE!(aux110, AUX_CONTROL, value);
        if REG!(aux110, AUX_RESET_MASK) {
            REG_WAIT!(aux110, AUX_CONTROL, AUX_RESET_DONE, 1, 1, 11);
            set_reg_field_value!(value, 0, AUX_CONTROL, AUX_RESET);
            REG_WRITE!(aux110, AUX_CONTROL, value);
            REG_WAIT!(aux110, AUX_CONTROL, AUX_RESET_DONE, 0, 1, 11);
        }
    }
    REG_UPDATE!(aux110, AUX_ARB_CONTROL, AUX_SW_USE_AUX_REG_REQ, 1);
    value = REG_READ!(aux110, AUX_ARB_CONTROL);
    field = get_reg_field_value(value, AUX_ARB_CONTROL, AUX_REG_RW_CNTL_STATUS);
    field == SW_CAN_ACCESS_AUX
}

unsafe fn submit_channel_request(engine: *mut dce_aux, request: *mut aux_request_transaction_data) {
    let aux110 = FROM_AUX_ENGINE!(engine);
    let is_write = ((*request).type_ == AUX_TRANSACTION_TYPE_DP && (*request).action == I2CAUX_TRANSACTION_ACTION_DP_WRITE) ||
        ((*request).type_ == AUX_TRANSACTION_TYPE_I2C && ((*request).action == I2CAUX_TRANSACTION_ACTION_I2C_WRITE || (*request).action == I2CAUX_TRANSACTION_ACTION_I2C_WRITE_MOT));
    if REG!(aux110, AUXN_IMPCAL) {
        REG_UPDATE_SEQ_2!(aux110, AUXN_IMPCAL, AUXN_CALOUT_ERROR_AK, 1, AUXN_CALOUT_ERROR_AK, 0);
        REG_UPDATE_SEQ_2!(aux110, AUXP_IMPCAL, AUXP_CALOUT_ERROR_AK, 1, AUXP_CALOUT_ERROR_AK, 0);
        REG_UPDATE_SEQ_2!(aux110, AUXN_IMPCAL, AUXN_IMPCAL_ENABLE, 1, AUXN_IMPCAL_OVERRIDE_ENABLE, 0);
        REG_UPDATE_SEQ_2!(aux110, AUXP_IMPCAL, AUXP_IMPCAL_OVERRIDE_ENABLE, 1, AUXP_IMPCAL_OVERRIDE_ENABLE, 0);
    }
    REG_UPDATE!(aux110, AUX_INTERRUPT_CONTROL, AUX_SW_DONE_ACK, 1);
    REG_WAIT!(aux110, AUX_SW_STATUS, AUX_SW_DONE, 0, 10, (*aux110).polling_timeout_period / 10);
    let mut length = if (*request).length != 0 { 4 } else { 3 };
    if is_write { length += (*request).length; }
    REG_UPDATE_2!(aux110, AUX_SW_CONTROL, AUX_SW_START_DELAY, (*request).delay, AUX_SW_WR_BYTES, length);
    let mut value = REG_UPDATE_4!(aux110, AUX_SW_DATA, AUX_SW_INDEX, 0, AUX_SW_DATA_RW, 0, AUX_SW_AUTOINCREMENT_DISABLE, 1, AUX_SW_DATA, ((*request).action | ((0xF0000 & (*request).address) >> 16)));
    value = REG_SET_2!(aux110, AUX_SW_DATA, value, AUX_SW_AUTOINCREMENT_DISABLE, 0, AUX_SW_DATA, ((0xFF00 & (*request).address) >> 8));
    value = REG_SET!(aux110, AUX_SW_DATA, value, AUX_SW_DATA, 0xFF & (*request).address);
    if (*request).length != 0 { value = REG_SET!(aux110, AUX_SW_DATA, value, AUX_SW_DATA, (*request).length - 1); }
    if is_write { for i in 0..(*request).length { value = REG_SET!(aux110, AUX_SW_DATA, value, AUX_SW_DATA, (*request).data.add(i).read()); } }
    REG_UPDATE!(aux110, AUX_SW_CONTROL, AUX_SW_GO, 1);
    if !(*engine).ddc.is_null() { EVENT_LOG_AUX_REQ!((*engine).ddc.pin_data.en, EVENT_LOG_AUX_ORIGIN_NATIVE, (*request).action, (*request).address, (*request).length, (*request).data); }
    else { EVENT_LOG_AUX_REQ!((*engine).inst, EVENT_LOG_AUX_ORIGIN_NATIVE, (*request).action, (*request).address, (*request).length, (*request).data); }
}

unsafe fn read_channel_reply(engine: *mut dce_aux, size: u32, buffer: *mut u8, reply_result: *mut u8, sw_status: *mut u32) -> i32 {
    let aux110 = FROM_AUX_ENGINE!(engine); let mut bytes_replied = 0; let mut result = 0;
    *sw_status = REG_GET!(aux110, AUX_SW_STATUS, AUX_SW_REPLY_BYTE_COUNT, &mut bytes_replied);
    if (*sw_status & AUX_SW_STATUS__AUX_SW_HPD_DISCON_MASK) != 0 || bytes_replied == 0 { return -1; }
    REG_UPDATE_SEQ_3!(aux110, AUX_SW_DATA, AUX_SW_INDEX, 0, AUX_SW_AUTOINCREMENT_DISABLE, 1, AUX_SW_DATA_RW, 1);
    REG_GET!(aux110, AUX_SW_DATA, AUX_SW_DATA, &mut result); result >>= 4;
    if !reply_result.is_null() { *reply_result = result as u8; }
    if result == 0 { bytes_replied -= 1; if bytes_replied > size { return -1; } for i in 0..bytes_replied { let mut v=0; REG_GET!(aux110,AUX_SW_DATA,AUX_SW_DATA,&mut v); buffer.add(i as usize).write(v as u8); } return bytes_replied as i32; }
    0
}

unsafe fn get_channel_status(engine: *mut dce_aux, returned_bytes: *mut u8) -> aux_return_code_type {
    let aux110=FROM_AUX_ENGINE!(engine); if returned_bytes.is_null() { ASSERT_CRITICAL!(false); return AUX_RET_ERROR_UNKNOWN; } *returned_bytes=0;
    REG_WAIT!(aux110,AUX_SW_STATUS,AUX_SW_DONE,1,10,(*aux110).polling_timeout_period/10); let value=REG_READ!(aux110,AUX_SW_STATUS);
    if value&AUX_SW_STATUS__AUX_SW_HPD_DISCON_MASK != 0 { return AUX_RET_ERROR_HPD_DISCON; }
    if value&AUX_SW_STATUS__AUX_SW_DONE_MASK != 0 { if value&(AUX_SW_STATUS__AUX_SW_RX_TIMEOUT_STATE_MASK|AUX_SW_STATUS__AUX_SW_RX_TIMEOUT_MASK)!=0{return AUX_RET_ERROR_TIMEOUT;} if value&(AUX_SW_STATUS__AUX_SW_RX_INVALID_STOP_MASK|AUX_SW_STATUS__AUX_SW_RX_RECV_NO_DET_MASK|AUX_SW_STATUS__AUX_SW_RX_RECV_INVALID_H_MASK|AUX_SW_STATUS__AUX_SW_RX_RECV_INVALID_L_MASK)!=0{return AUX_RET_ERROR_INVALID_REPLY;} *returned_bytes=get_reg_field_value(value,AUX_SW_STATUS,AUX_SW_REPLY_BYTE_COUNT) as u8; if *returned_bytes==0{return AUX_RET_ERROR_INVALID_REPLY;} *returned_bytes-=1; AUX_RET_SUCCESS } else { ASSERT_CRITICAL!(false); AUX_RET_ERROR_TIMEOUT }
}

// The remaining entry points are declared here; their definitions are supplied
// by the corresponding translation unit when the complete driver is assembled.
extern "C" {
    fn dce110_engine_destroy(engine: *mut *mut dce_aux);
    fn dce_aux_transfer_raw(ddc: *mut ddc_service, payload: *mut aux_payload,
                            operation_result: *mut aux_return_code_type) -> i32;
    fn dce_aux_transfer_raw_with_ddc_pin(ddc: *mut ddc_service, payload: *mut aux_payload,
                                         operation_result: *mut aux_return_code_type) -> i32;
    fn dce_aux_transfer_raw_without_ddc_pin(ddc: *mut ddc_service, payload: *mut aux_payload,
                                            operation_result: *mut aux_return_code_type) -> i32;
    fn dce_aux_transfer_dmub_raw(ddc: *mut ddc_service, payload: *mut aux_payload,
                                 operation_result: *mut aux_return_code_type) -> i32;
    fn dce_aux_transfer_with_retries(ddc: *mut ddc_service, payload: *mut aux_payload) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
