/* Direct low-level Rust translation of dce_i2c_hw.c. External declarations,
 * register helpers, and constants are provided by the surrounding sources. */

unsafe fn execute_transaction(dce_i2c_hw: *mut dce_i2c_hw) {
    REG_UPDATE_N!(SETUP, 5, FN!(DC_I2C_DDC1_SETUP, DC_I2C_DDC1_DATA_DRIVE_EN), 0, FN!(DC_I2C_DDC1_SETUP, DC_I2C_DDC1_CLK_DRIVE_EN), 0, FN!(DC_I2C_DDC1_SETUP, DC_I2C_DDC1_DATA_DRIVE_SEL), 0, FN!(DC_I2C_DDC1_SETUP, DC_I2C_DDC1_INTRA_TRANSACTION_DELAY), 0, FN!(DC_I2C_DDC1_SETUP, DC_I2C_DDC1_INTRA_BYTE_DELAY), 0);
    REG_UPDATE_5!(DC_I2C_CONTROL, DC_I2C_SOFT_RESET, 0, DC_I2C_SW_STATUS_RESET, 0, DC_I2C_SEND_RESET, 0, DC_I2C_GO, 0, DC_I2C_TRANSACTION_COUNT, (*dce_i2c_hw).transaction_count - 1);
    REG_UPDATE!(DC_I2C_CONTROL, DC_I2C_GO, 1);
    (*dce_i2c_hw).transaction_count = 0;
    (*dce_i2c_hw).buffer_used_bytes = 0;
}

unsafe fn get_channel_status(dce_i2c_hw: *mut dce_i2c_hw, _returned_bytes: *mut u8) -> i2c_channel_operation_result {
    let mut status = 0u32;
    let value = REG_GET!(DC_I2C_SW_STATUS, DC_I2C_SW_STATUS, &mut status);
    if status == DC_I2C_STATUS__DC_I2C_STATUS_USED_BY_SW { I2C_CHANNEL_OPERATION_ENGINE_BUSY }
    else if value & (*(*dce_i2c_hw).masks).DC_I2C_SW_STOPPED_ON_NACK != 0 { I2C_CHANNEL_OPERATION_NO_RESPONSE }
    else if value & (*(*dce_i2c_hw).masks).DC_I2C_SW_TIMEOUT != 0 { I2C_CHANNEL_OPERATION_TIMEOUT }
    else if value & (*(*dce_i2c_hw).masks).DC_I2C_SW_ABORTED != 0 { I2C_CHANNEL_OPERATION_FAILED }
    else { I2C_CHANNEL_OPERATION_SUCCEEDED }
}

unsafe fn get_hw_buffer_available_size(dce_i2c_hw: *const dce_i2c_hw) -> u32 { (*dce_i2c_hw).buffer_size - (*dce_i2c_hw).buffer_used_bytes }

unsafe fn process_channel_reply(dce_i2c_hw: *mut dce_i2c_hw, reply: *mut i2c_payload) {
    let mut length = (*reply).length; let mut buffer = (*reply).data;
    REG_SET_3!(DC_I2C_DATA, 0, DC_I2C_INDEX, (*dce_i2c_hw).buffer_used_write, DC_I2C_DATA_RW, 1, DC_I2C_INDEX_WRITE, 1);
    while length != 0 { let mut data=0u32; REG_GET!(DC_I2C_DATA, DC_I2C_DATA, &mut data); *buffer=data as u8; buffer=buffer.add(1); length-=1; }
}

unsafe fn is_engine_available(dce_i2c_hw: *mut dce_i2c_hw) -> bool {
    let mut status=0u32; REG_GET!(HW_STATUS, DC_I2C_DDC1_HW_STATUS, &mut status); if status == DC_I2C_STATUS__DC_I2C_STATUS_USED_BY_HW { return false; }
    let mut arbitrate=0u32; REG_GET!(DC_I2C_ARBITRATION, DC_I2C_REG_RW_CNTL_STATUS, &mut arbitrate); arbitrate != DC_I2C_REG_RW_CNTL_STATUS_DMCU_ONLY
}
unsafe fn is_hw_busy(dce_i2c_hw: *mut dce_i2c_hw) -> bool { let mut s=0u32; REG_GET!(DC_I2C_SW_STATUS, DC_I2C_SW_STATUS, &mut s); s != DC_I2C_STATUS__DC_I2C_STATUS_IDLE && !is_engine_available(dce_i2c_hw) }

unsafe fn process_transaction(dce_i2c_hw: *mut dce_i2c_hw, request: *mut i2c_request_transaction_data) -> bool {
    let mut length=(*request).length; let mut buffer=(*request).data;
    if is_hw_busy(dce_i2c_hw) { (*request).status=I2C_CHANNEL_OPERATION_ENGINE_BUSY; return false; }
    let last=(*dce_i2c_hw).transaction_count==3 || (*request).action==DCE_I2C_TRANSACTION_ACTION_I2C_WRITE || (*request).action & DCE_I2C_TRANSACTION_ACTION_I2C_READ != 0;
    match (*dce_i2c_hw).transaction_count { 0=>REG_UPDATE_5!(DC_I2C_TRANSACTION0,DC_I2C_STOP_ON_NACK0,1,DC_I2C_START0,1,DC_I2C_RW0,(*request).action&DCE_I2C_TRANSACTION_ACTION_I2C_READ!=0,DC_I2C_COUNT0,length,DC_I2C_STOP0,if last{1}else{0}), 1=>REG_UPDATE_5!(DC_I2C_TRANSACTION1,DC_I2C_STOP_ON_NACK0,1,DC_I2C_START0,1,DC_I2C_RW0,(*request).action&DCE_I2C_TRANSACTION_ACTION_I2C_READ!=0,DC_I2C_COUNT0,length,DC_I2C_STOP0,if last{1}else{0}), 2=>REG_UPDATE_5!(DC_I2C_TRANSACTION2,DC_I2C_STOP_ON_NACK0,1,DC_I2C_START0,1,DC_I2C_RW0,(*request).action&DCE_I2C_TRANSACTION_ACTION_I2C_READ!=0,DC_I2C_COUNT0,length,DC_I2C_STOP0,if last{1}else{0}), 3=>REG_UPDATE_5!(DC_I2C_TRANSACTION3,DC_I2C_STOP_ON_NACK0,1,DC_I2C_START0,1,DC_I2C_RW0,(*request).action&DCE_I2C_TRANSACTION_ACTION_I2C_READ!=0,DC_I2C_COUNT0,length,DC_I2C_STOP0,if last{1}else{0}), _=>() }
    let value=if (*dce_i2c_hw).transaction_count==0 { let v=REG_SET_4!(DC_I2C_DATA,0,DC_I2C_DATA_RW,false,DC_I2C_DATA,(*request).address,DC_I2C_INDEX,0,DC_I2C_INDEX_WRITE,1); (*dce_i2c_hw).buffer_used_write=0; v } else { REG_SET_2!(DC_I2C_DATA,0,DC_I2C_DATA_RW,false,DC_I2C_DATA,(*request).address) };
    (*dce_i2c_hw).buffer_used_write+=1;
    if (*request).action & DCE_I2C_TRANSACTION_ACTION_I2C_READ == 0 { while length!=0 { REG_SET_2!(DC_I2C_DATA,value,DC_I2C_INDEX_WRITE,0,DC_I2C_DATA,*buffer); buffer=buffer.add(1); (*dce_i2c_hw).buffer_used_write+=1; length-=1; } }
    (*dce_i2c_hw).transaction_count+=1; (*dce_i2c_hw).buffer_used_bytes+=length+1; last
}

unsafe fn reset_hw_engine(_dce_i2c_hw: *mut dce_i2c_hw) { REG_UPDATE_2!(DC_I2C_CONTROL,DC_I2C_SW_STATUS_RESET,1,DC_I2C_SW_STATUS_RESET,1); }

unsafe fn set_speed(dce_i2c_hw:*mut dce_i2c_hw,speed:u32) { if speed==0{return;} let(mut xtal,mut base)=(0u32,0u32); REG_GET_2!(MICROSECOND_TIME_BASE_DIV,MICROSECOND_TIME_BASE_DIV,&mut base,XTAL_REF_DIV,&mut xtal); if xtal==0{xtal=2;} let clock=if base==0{(*dce_i2c_hw).reference_frequency*2}else{base*1000}; let pre=(clock/xtal)/speed; if (*(*dce_i2c_hw).masks).DC_I2C_DDC1_START_STOP_TIMING_CNTL!=0{REG_UPDATE_N!(SPEED,3,FN!(DC_I2C_DDC1_SPEED,DC_I2C_DDC1_PRESCALE),pre,FN!(DC_I2C_DDC1_SPEED,DC_I2C_DDC1_THRESHOLD),2,FN!(DC_I2C_DDC1_SPEED,DC_I2C_DDC1_START_STOP_TIMING_CNTL),if speed>50{2}else{1});}else{REG_UPDATE_N!(SPEED,2,FN!(DC_I2C_DDC1_SPEED,DC_I2C_DDC1_PRESCALE),pre,FN!(DC_I2C_DDC1_SPEED,DC_I2C_DDC1_THRESHOLD),2);} }

unsafe fn acquire_engine(dce_i2c_hw:*mut dce_i2c_hw)->bool{let mut a=0u32;REG_GET!(DC_I2C_ARBITRATION,DC_I2C_REG_RW_CNTL_STATUS,&mut a);if a==DC_I2C_STATUS__DC_I2C_STATUS_USED_BY_SW{return true;}if a==DC_I2C_STATUS__DC_I2C_STATUS_USED_BY_HW{return false;}REG_UPDATE!(DC_I2C_ARBITRATION,DC_I2C_SW_USE_I2C_REG_REQ,true);REG_GET!(DC_I2C_ARBITRATION,DC_I2C_REG_RW_CNTL_STATUS,&mut a);a==DC_I2C_STATUS__DC_I2C_STATUS_USED_BY_SW}

unsafe fn setup_engine(dce_i2c_hw:*mut dce_i2c_hw)->bool{REG_UPDATE!(DC_I2C_CONTROL,DC_I2C_SOFT_RESET,false);let mut limit=I2C_SETUP_TIME_LIMIT_DCE;if (*dce_i2c_hw).setup_limit!=0{limit=(*dce_i2c_hw).setup_limit;}if !acquire_engine(dce_i2c_hw){return false;}set_speed(dce_i2c_hw,(*(*(*dce_i2c_hw).ctx).dc).caps.i2c_speed_in_khz);REG_UPDATE_5!(DC_I2C_CONTROL,DC_I2C_GO,0,DC_I2C_SEND_RESET,0,DC_I2C_SW_STATUS_RESET,1,DC_I2C_TRANSACTION_COUNT,0,DC_I2C_DDC_SELECT,(*dce_i2c_hw).engine_id);if (*dce_i2c_hw).send_reset_length==0{REG_UPDATE_N!(SETUP,2,FN!(DC_I2C_DDC1_SETUP,DC_I2C_DDC1_TIME_LIMIT),limit,FN!(DC_I2C_DDC1_SETUP,DC_I2C_DDC1_ENABLE),1);}else{REG_UPDATE_N!(SETUP,3,FN!(DC_I2C_DDC1_SETUP,DC_I2C_DDC1_TIME_LIMIT),limit,FN!(DC_I2C_DDC1_SETUP,DC_I2C_DDC1_SEND_RESET_LENGTH),(*dce_i2c_hw).send_reset_length,FN!(DC_I2C_DDC1_SETUP,DC_I2C_DDC1_ENABLE),1);}REG_UPDATE!(DC_I2C_ARBITRATION,DC_I2C_NO_QUEUED_SW_GO,0);true}

unsafe fn cntl_stuck_hw_workaround(dce_i2c_hw:*mut dce_i2c_hw){let mut a=0u32;REG_GET!(DC_I2C_ARBITRATION,DC_I2C_REG_RW_CNTL_STATUS,&mut a);if a!=DC_I2C_STATUS__DC_I2C_STATUS_USED_BY_SW{return;}REG_UPDATE!(DC_I2C_ARBITRATION,DC_I2C_SW_DONE_USING_I2C_REG,true);REG_GET!(DC_I2C_ARBITRATION,DC_I2C_REG_RW_CNTL_STATUS,&mut a);ASSERT!(a!=DC_I2C_STATUS__DC_I2C_STATUS_USED_BY_SW);}

unsafe fn release_engine(dce_i2c_hw:*mut dce_i2c_hw){let mut s=0u32;REG_GET!(DC_I2C_SW_STATUS,DC_I2C_SW_STATUS,&mut s);if s==1{REG_UPDATE_2!(DC_I2C_CONTROL,DC_I2C_SOFT_RESET,1,DC_I2C_SW_STATUS_RESET,1);}else{REG_UPDATE!(DC_I2C_CONTROL,DC_I2C_SW_STATUS_RESET,1);}if (*dce_i2c_hw).engine_keep_power_up_count==0{REG_UPDATE_N!(SETUP,1,FN!(SETUP,DC_I2C_DDC1_ENABLE),0);}set_speed(dce_i2c_hw,(*(*(*dce_i2c_hw).ctx).dc).caps.i2c_speed_in_khz_hdcp);REG_UPDATE!(DC_I2C_ARBITRATION,DC_I2C_SW_DONE_USING_I2C_REG,true);cntl_stuck_hw_workaround(dce_i2c_hw);}

// Public acquisition, submission, and constructor functions correspond to the
// remaining C definitions and use the same external structures and constants.
unsafe fn dce_i2c_hw_construct(h:*mut dce_i2c_hw,ctx:*mut dc_context,id:u32,regs:*const dce_i2c_registers,shifts:*const dce_i2c_shift,masks:*const dce_i2c_mask){(*h).ctx=ctx;(*h).engine_id=id;(*h).reference_frequency=(*(*ctx).dc_bios).fw_info.pll_info.crystal_frequency>>1;(*h).regs=regs;(*h).shifts=shifts;(*h).masks=masks;(*h).buffer_used_bytes=0;(*h).transaction_count=0;(*h).engine_keep_power_up_count=1;(*h).default_speed=DEFAULT_I2C_HW_SPEED;(*h).send_reset_length=0;(*h).setup_limit=I2C_SETUP_TIME_LIMIT_DCE;(*h).buffer_size=I2C_HW_BUFFER_SIZE_DCE;}
unsafe fn dce100_i2c_hw_construct(h:*mut dce_i2c_hw,c:*mut dc_context,id:u32,r:*const dce_i2c_registers,s:*const dce_i2c_shift,m:*const dce_i2c_mask){dce_i2c_hw_construct(h,c,id,r,s,m);(*h).buffer_size=I2C_HW_BUFFER_SIZE_DCE100;}
unsafe fn dce112_i2c_hw_construct(h:*mut dce_i2c_hw,c:*mut dc_context,id:u32,r:*const dce_i2c_registers,s:*const dce_i2c_shift,m:*const dce_i2c_mask){dce100_i2c_hw_construct(h,c,id,r,s,m);(*h).default_speed=DEFAULT_I2C_HW_SPEED_100KHZ;}
unsafe fn dcn1_i2c_hw_construct(h:*mut dce_i2c_hw,c:*mut dc_context,id:u32,r:*const dce_i2c_registers,s:*const dce_i2c_shift,m:*const dce_i2c_mask){dce112_i2c_hw_construct(h,c,id,r,s,m);(*h).setup_limit=I2C_SETUP_TIME_LIMIT_DCN;}
unsafe fn dcn2_i2c_hw_construct(h:*mut dce_i2c_hw,c:*mut dc_context,id:u32,r:*const dce_i2c_registers,s:*const dce_i2c_shift,m:*const dce_i2c_mask){dcn1_i2c_hw_construct(h,c,id,r,s,m);(*h).send_reset_length=I2C_SEND_RESET_LENGTH_9;if (*(*c).dc).debug.scl_reset_length10{(*h).send_reset_length=I2C_SEND_RESET_LENGTH_10;}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
