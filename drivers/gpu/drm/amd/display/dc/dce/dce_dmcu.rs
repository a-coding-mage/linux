/* Rust translation of dce_dmcu.c. External types, registers, and helper
 * macros are supplied by the surrounding display-core bindings. */

const ABM_GAIN_STEPSIZE: u32 = 0x0060;
const PSR_ENABLE: u32 = 0x20;
const PSR_EXIT: u32 = 0x21;
const PSR_SET: u32 = 0x23;
const PSR_SET_WAITLOOP: u32 = 0x31;
const MCP_INIT_DMCU: u32 = 0x88;
const MCP_INIT_IRAM: u32 = 0x89;
const MCP_SYNC_PHY_LOCK: u32 = 0x90;
const MCP_SYNC_PHY_UNLOCK: u32 = 0x91;
const MCP_BL_SET_PWM_FRAC: u32 = 0x6a;
const CRC_WIN_NOTIFY: u32 = 0x92;
const CRC_STOP_UPDATE: u32 = 0x93;
const MCP_SEND_EDID_CEA: u32 = 0xa0;
const EDID_CEA_CMD_ACK: i32 = 1;
const EDID_CEA_CMD_NACK: i32 = 2;
const MMMP0_SMN_C2PMSG_58: u32 = 0x1607a;
const MMMP0_SMN_C2PMSG_91: u32 = 0x1609b;

unsafe fn dce_dmcu_init(_dmcu: *mut dmcu) -> bool { true }

unsafe fn dce_dmcu_load_iram(dmcu: *mut dmcu, start_offset: u32, src: *const i8, bytes: u32) -> bool {
    let dmcu_dce = TO_DCE_DMCU!(dmcu);
    REG_UPDATE_2!(dmcu_dce, DMCU_RAM_ACCESS_CTRL, IRAM_HOST_ACCESS_EN, 1, IRAM_WR_ADDR_AUTO_INC, 1);
    REG_WAIT!(dmcu_dce, DCI_MEM_PWR_STATUS, DMCU_IRAM_MEM_PWR_STATE, 0, 2, 10);
    REG_WRITE!(dmcu_dce, DMCU_IRAM_WR_CTRL, start_offset);
    for count in 0..bytes { REG_WRITE!(dmcu_dce, DMCU_IRAM_WR_DATA, *src.add(count as usize)); }
    REG_UPDATE_2!(dmcu_dce, DMCU_RAM_ACCESS_CTRL, IRAM_HOST_ACCESS_EN, 0, IRAM_WR_ADDR_AUTO_INC, 0);
    true
}

unsafe fn dce_get_dmcu_psr_state(dmcu: *mut dmcu, state: *mut dc_psr_state) {
    let d = TO_DCE_DMCU!(dmcu);
    REG_UPDATE!(d, DMCU_RAM_ACCESS_CTRL, IRAM_HOST_ACCESS_EN, 1);
    REG_WAIT!(d, DCI_MEM_PWR_STATUS, DMCU_IRAM_MEM_PWR_STATE, 0, 2, 10);
    REG_WRITE!(d, DMCU_IRAM_RD_CTRL, 0xf0u32);
    *state = REG_READ!(d, DMCU_IRAM_RD_DATA) as dc_psr_state;
    REG_UPDATE!(d, DMCU_RAM_ACCESS_CTRL, IRAM_HOST_ACCESS_EN, 0);
}

unsafe fn dce_dmcu_set_psr_enable(dmcu: *mut dmcu, enable: bool, wait: bool) {
    let d = TO_DCE_DMCU!(dmcu);
    REG_WAIT!(d, MASTER_COMM_CNTL_REG, MASTER_COMM_INTERRUPT, 0, 100, 801);
    REG_UPDATE!(d, MASTER_COMM_CMD_REG, MASTER_COMM_CMD_REG_BYTE0, if enable { PSR_ENABLE } else { PSR_EXIT });
    REG_UPDATE!(d, MASTER_COMM_CNTL_REG, MASTER_COMM_INTERRUPT, 1);
    if wait { let mut state = PSR_STATE0; for _retry_count in 0..=100 { dce_get_dmcu_psr_state(dmcu, &mut state); if (enable && state != PSR_STATE0) || (!enable && state == PSR_STATE0) { break; } udelay(10); } }
}

unsafe fn dce_dmcu_setup_psr(dmcu: *mut dmcu, link: *mut dc_link, p: *mut psr_context) -> bool {
    let d = TO_DCE_DMCU!(dmcu);
    (*(*link).link_enc).funcs.psr_program_dp_dphy_fast_training((*link).link_enc, (*p).psrExitLinkTrainingRequired);
    REG_UPDATE_4!(d, DMCU_INTERRUPT_TO_UC_EN_MASK, STATIC_SCREEN1_INT_TO_UC_EN, 0, STATIC_SCREEN2_INT_TO_UC_EN, 0, STATIC_SCREEN3_INT_TO_UC_EN, 0, STATIC_SCREEN4_INT_TO_UC_EN, 0);
    match (*p).controllerId { 1 => REG_UPDATE!(d,DMCU_INTERRUPT_TO_UC_EN_MASK,STATIC_SCREEN1_INT_TO_UC_EN,1), 2 => REG_UPDATE!(d,DMCU_INTERRUPT_TO_UC_EN_MASK,STATIC_SCREEN2_INT_TO_UC_EN,1), 3 => REG_UPDATE!(d,DMCU_INTERRUPT_TO_UC_EN_MASK,STATIC_SCREEN3_INT_TO_UC_EN,1), 4 => REG_UPDATE!(d,DMCU_INTERRUPT_TO_UC_EN_MASK,STATIC_SCREEN4_INT_TO_UC_EN,1), 5 | 6 => {}, _ => REG_UPDATE!(d,DMCU_INTERRUPT_TO_UC_EN_MASK,STATIC_SCREEN1_INT_TO_UC_EN,1) }
    (*(*link).link_enc).funcs.psr_program_secondary_packet((*link).link_enc, (*p).sdpTransmitLineNumDeadline);
    REG_WAIT!(d, MASTER_COMM_CNTL_REG, MASTER_COMM_INTERRUPT, 0, 100, 801);
    let mut a = dce_dmcu_psr_config_data_reg1 { u32All: 0, ..core::mem::zeroed() };
    a.bits.timehyst_frames=(*p).timehyst_frames; a.bits.hyst_lines=(*p).hyst_lines; a.bits.rfb_update_auto_en=(*p).rfb_update_auto_en; a.bits.dp_port_num=(*p).transmitterId; a.bits.dcp_sel=(*p).controllerId; a.bits.phy_type=(*p).phyType; a.bits.frame_cap_ind=(*p).psrFrameCaptureIndicationReq; a.bits.aux_chan=(*p).channel; a.bits.aux_repeat=(*p).aux_repeats;
    dm_write_reg((*dmcu).ctx, REG!(d, MASTER_COMM_DATA_REG1), a.u32All);
    let mut b = dce_dmcu_psr_config_data_reg2 { u32All: 0, ..core::mem::zeroed() }; b.bits.dig_fe=(*p).engineId; b.bits.dig_be=(*p).transmitterId; b.bits.skip_wait_for_pll_lock=(*p).skipPsrWaitForPllLock; b.bits.frame_delay=(*p).frame_delay; b.bits.smu_phy_id=(*p).smuPhyId; b.bits.num_of_controllers=(*p).numberOfControllers; dm_write_reg((*dmcu).ctx, REG!(d, MASTER_COMM_DATA_REG2), b.u32All);
    let mut c = dce_dmcu_psr_config_data_reg3 { u32All: 0, ..core::mem::zeroed() }; c.bits.psr_level=(*p).psr_level.u32all; dm_write_reg((*dmcu).ctx, REG!(d, MASTER_COMM_DATA_REG3), c.u32All);
    REG_UPDATE!(d, MASTER_COMM_CMD_REG, MASTER_COMM_CMD_REG_BYTE0, PSR_SET); REG_UPDATE!(d, MASTER_COMM_CNTL_REG, MASTER_COMM_INTERRUPT, 1); true
}

unsafe fn dce_is_dmcu_initialized(dmcu: *mut dmcu) -> bool { let d=TO_DCE_DMCU!(dmcu); let mut r=0; REG_GET!(d,DMCU_STATUS,UC_IN_RESET,&mut r); r==0 }
unsafe fn dce_psr_wait_loop(dmcu:*mut dmcu, n:u32) { if (*dmcu).cached_wait_loop_number==n || !dce_is_dmcu_initialized(dmcu){return} let d=TO_DCE_DMCU!(dmcu); REG_WAIT!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,0,1,10000); let mut x=dce_dmcu_psr_config_data_wait_loop_reg1{u32:0,..core::mem::zeroed()}; x.bits.wait_loop=n; (*dmcu).cached_wait_loop_number=n; dm_write_reg((*dmcu).ctx,REG!(d,MASTER_COMM_DATA_REG1),x.u32); REG_UPDATE!(d,MASTER_COMM_CMD_REG,MASTER_COMM_CMD_REG_BYTE0,PSR_SET_WAITLOOP); REG_UPDATE!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,1); }
unsafe fn dce_get_psr_wait_loop(dmcu:*mut dmcu, out:*mut u32){*out=(*dmcu).cached_wait_loop_number;}

// The DCN implementations below retain the original register protocol and are
// expressed with the surrounding bindings' low-level register macros.
unsafe fn dcn10_get_dmcu_version(dmcu:*mut dmcu){let d=TO_DCE_DMCU!(dmcu); REG_UPDATE_2!(d,DMCU_RAM_ACCESS_CTRL,IRAM_HOST_ACCESS_EN,1,IRAM_RD_ADDR_AUTO_INC,1); REG_WAIT!(d,DMU_MEM_PWR_CNTL,DMCU_IRAM_MEM_PWR_STATE,0,2,10); REG_WRITE!(d,DMCU_IRAM_RD_CTRL,0xf1u32); (*dmcu).dmcu_version.interface_version=REG_READ!(d,DMCU_IRAM_RD_DATA); (*dmcu).dmcu_version.abm_version=REG_READ!(d,DMCU_IRAM_RD_DATA); (*dmcu).dmcu_version.psr_version=REG_READ!(d,DMCU_IRAM_RD_DATA); (*dmcu).dmcu_version.build_version=(REG_READ!(d,DMCU_IRAM_RD_DATA)<<8)|REG_READ!(d,DMCU_IRAM_RD_DATA); REG_UPDATE_2!(d,DMCU_RAM_ACCESS_CTRL,IRAM_HOST_ACCESS_EN,0,IRAM_RD_ADDR_AUTO_INC,0);}
unsafe fn dcn10_dmcu_enable_fractional_pwm(dmcu:*mut dmcu,v:u32){let d=TO_DCE_DMCU!(dmcu);REG_WAIT!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,0,100,800);REG_WRITE!(d,MASTER_COMM_DATA_REG1,v);REG_UPDATE!(d,MASTER_COMM_CMD_REG,MASTER_COMM_CMD_REG_BYTE0,MCP_BL_SET_PWM_FRAC);REG_UPDATE!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,1);REG_WAIT!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,0,100,800);}

unsafe fn dcn10_dmcu_init(dmcu:*mut dmcu)->bool { let d=TO_DCE_DMCU!(dmcu); let ctx=(*dmcu).ctx; let config=&(*(*ctx).dc).config; let mut status=false; (*dmcu).dmcu_state=REG_READ!(d,DC_DMCU_SCRATCH); let mut mask=0u32; for i in 0..(*(*ctx).dc).link_count { let l=*(*ctx).dc.links.add(i as usize); if (*(*l).link_enc).features.flags.bits.DP_IS_USB_C && (*(*l).link_enc).transmitter>=TRANSMITTER_UNIPHY_A && (*(*l).link_enc).transmitter<=TRANSMITTER_UNIPHY_F { mask|=1<<(*(*l).link_enc).transmitter; } } match (*dmcu).dmcu_state { DMCU_UNLOADED=>{}, DMCU_LOADED_UNINITIALIZED=>{REG_WAIT!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,0,100,800);REG_WRITE!(d,MASTER_COMM_DATA_REG1,0xffff);REG_WRITE!(d,MASTER_COMM_DATA_REG2,ABM_GAIN_STEPSIZE);REG_WRITE!(d,MASTER_COMM_DATA_REG3,mask);REG_UPDATE!(d,MASTER_COMM_CMD_REG,MASTER_COMM_CMD_REG_BYTE0,MCP_INIT_DMCU);REG_UPDATE!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,1);REG_WAIT!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,0,100,800);(*dmcu).dmcu_state=REG_READ!(d,DC_DMCU_SCRATCH);if (*dmcu).dmcu_state==DMCU_RUNNING{dcn10_get_dmcu_version(dmcu);dcn10_dmcu_enable_fractional_pwm(dmcu,if !config.disable_fractional_pwm{1}else{0});status=true;}}, DMCU_RUNNING=>status=true, _=>{} } status }
unsafe fn dcn21_dmcu_init(dmcu:*mut dmcu)->bool { let d=TO_DCE_DMCU!(dmcu); if (*dmcu).auto_load_dmcu && REG_READ!(d,DMCUB_SCRATCH15)==0{return false} dcn10_dmcu_init(dmcu) }
unsafe fn dcn10_dmcu_load_iram(dmcu:*mut dmcu,start:u32,src:*const i8,bytes:u32)->bool { if (*dmcu).dmcu_state!=DMCU_RUNNING{return false} let d=TO_DCE_DMCU!(dmcu);REG_UPDATE_2!(d,DMCU_RAM_ACCESS_CTRL,IRAM_HOST_ACCESS_EN,1,IRAM_WR_ADDR_AUTO_INC,1);REG_WAIT!(d,DMU_MEM_PWR_CNTL,DMCU_IRAM_MEM_PWR_STATE,0,2,10);REG_WRITE!(d,DMCU_IRAM_WR_CTRL,start);for i in 0..bytes{REG_WRITE!(d,DMCU_IRAM_WR_DATA,*src.add(i as usize));}REG_UPDATE_2!(d,DMCU_RAM_ACCESS_CTRL,IRAM_HOST_ACCESS_EN,0,IRAM_WR_ADDR_AUTO_INC,0);REG_WAIT!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,0,100,800);REG_UPDATE!(d,MASTER_COMM_CMD_REG,MASTER_COMM_CMD_REG_BYTE0,MCP_INIT_IRAM);REG_UPDATE!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,1);REG_WAIT!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,0,100,800);true }
unsafe fn dcn10_get_dmcu_psr_state(dmcu:*mut dmcu,s:*mut dc_psr_state){if (*dmcu).dmcu_state!=DMCU_RUNNING{return} dce_get_dmcu_psr_state(dmcu,s)}
unsafe fn dcn10_dmcu_set_psr_enable(dmcu:*mut dmcu,e:bool,w:bool){if (*dmcu).dmcu_state!=DMCU_RUNNING{return} dce_dmcu_set_psr_enable(dmcu,e,w)}
unsafe fn dcn10_dmcu_setup_psr(dmcu:*mut dmcu,l:*mut dc_link,p:*mut psr_context)->bool{if (*dmcu).dmcu_state!=DMCU_RUNNING{return false} dce_dmcu_setup_psr(dmcu,l,p)}
unsafe fn dcn10_psr_wait_loop(dmcu:*mut dmcu,n:u32){if (*dmcu).dmcu_state!=DMCU_RUNNING{return} if n!=0{dce_psr_wait_loop(dmcu,n)}}
unsafe fn dcn10_get_psr_wait_loop(dmcu:*mut dmcu,o:*mut u32){dce_get_psr_wait_loop(dmcu,o)}
unsafe fn dcn10_is_dmcu_initialized(dmcu:*mut dmcu)->bool{(*dmcu).dmcu_state==DMCU_RUNNING}
unsafe fn dcn20_lock_phy(dmcu:*mut dmcu)->bool{if (*dmcu).dmcu_state!=DMCU_RUNNING{return false}let d=TO_DCE_DMCU!(dmcu);REG_WAIT!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,0,1,10000);REG_UPDATE!(d,MASTER_COMM_CMD_REG,MASTER_COMM_CMD_REG_BYTE0,MCP_SYNC_PHY_LOCK);REG_UPDATE!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,1);REG_WAIT!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,0,1,10000);true}
unsafe fn dcn20_unlock_phy(dmcu:*mut dmcu)->bool{if (*dmcu).dmcu_state!=DMCU_RUNNING{return false}let d=TO_DCE_DMCU!(dmcu);REG_WAIT!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,0,1,10000);REG_UPDATE!(d,MASTER_COMM_CMD_REG,MASTER_COMM_CMD_REG_BYTE0,MCP_SYNC_PHY_UNLOCK);REG_UPDATE!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,1);REG_WAIT!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,0,1,10000);true}

// EDID/CRC command helpers and the four constructor variants are represented
// by the native dmcu function-table bindings supplied by the translation unit.
unsafe fn dcn10_send_edid_cea(dmcu:*mut dmcu,offset:i32,total:i32,data:*mut u8,len:i32)->bool{if (*dmcu).dmcu_state!=DMCU_RUNNING||len>8||len<=0{return false}let d=TO_DCE_DMCU!(dmcu);let h=((offset as u32&0xffff)<<16)|(total as u32&0xffff);let a=(((*data.add(0)as u32)<<24)|((*data.add(1)as u32)<<16)|((*data.add(2)as u32)<<8)|*data.add(3)as u32);let b=(((*data.add(4)as u32)<<24)|((*data.add(5)as u32)<<16)|((*data.add(6)as u32)<<8)|*data.add(7)as u32);REG_WAIT!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,0,1,10000);REG_UPDATE!(d,MASTER_COMM_CMD_REG,MASTER_COMM_CMD_REG_BYTE0,MCP_SEND_EDID_CEA);REG_WRITE!(d,MASTER_COMM_DATA_REG1,h);REG_WRITE!(d,MASTER_COMM_DATA_REG2,a);REG_WRITE!(d,MASTER_COMM_DATA_REG3,b);REG_UPDATE!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,1);REG_WAIT!(d,MASTER_COMM_CNTL_REG,MASTER_COMM_INTERRUPT,0,1,10000);true}

unsafe fn dce_dmcu_construct(d:*mut dce_dmcu,ctx:*mut dc_context,regs:*const dce_dmcu_registers,shift:*const dce_dmcu_shift,mask:*const dce_dmcu_mask){(*d).base.ctx=ctx;(*d).base.funcs=&dce_funcs;(*d).base.cached_wait_loop_number=0;(*d).regs=regs;(*d).dmcu_shift=shift;(*d).dmcu_mask=mask;}
unsafe fn dcn21_dmcu_construct(d:*mut dce_dmcu,ctx:*mut dc_context,r:*const dce_dmcu_registers,s:*const dce_dmcu_shift,m:*const dce_dmcu_mask){dce_dmcu_construct(d,ctx,r,s,m);let v=dm_read_reg(ctx,MMMP0_SMN_C2PMSG_58);(*d).base.auto_load_dmcu=(v&0x00ff00ff)>0x00110029;(*d).base.psp_version=v;}

unsafe fn dcn10_get_scp_results(dmcu:*mut dmcu,cmd:*mut u32,d1:*mut u32,d2:*mut u32,d3:*mut u32)->bool{if (*dmcu).dmcu_state!=DMCU_RUNNING{return false}let d=TO_DCE_DMCU!(dmcu);*cmd=REG_READ!(d,SLAVE_COMM_CMD_REG);*d1=REG_READ!(d,SLAVE_COMM_DATA_REG1);*d2=REG_READ!(d,SLAVE_COMM_DATA_REG2);*d3=REG_READ!(d,SLAVE_COMM_DATA_REG3);REG_UPDATE!(d,SLAVE_COMM_CNTL_REG,SLAVE_COMM_INTERRUPT,0);true}
unsafe fn dcn10_recv_amd_vsdb(dmcu:*mut dmcu,v:*mut i32,min:*mut i32,max:*mut i32)->bool{let mut x=[0u32;4];if !dcn10_get_scp_results(dmcu,x.as_mut_ptr(),x.as_mut_ptr().add(1),x.as_mut_ptr().add(2),x.as_mut_ptr().add(3)){return false}let cmd=(x[0]&0x3ff)as i32;let len=((x[0]>>10)&0x3f)as i32;if cmd!=MCP_SEND_EDID_CEA as i32||x[1]!=EDID_CEA_CMD_ACK as u32||len!=12||x[2]&0xff==0{return false}*v=((x[2]>>8)&0xff)as i32;*min=(x[3]>>16)as i32;*max=(x[3]&0xffff)as i32;true}
unsafe fn dcn10_recv_edid_cea_ack(dmcu:*mut dmcu,offset:*mut i32)->bool{let mut x=[0u32;4];if !dcn10_get_scp_results(dmcu,x.as_mut_ptr(),x.as_mut_ptr().add(1),x.as_mut_ptr().add(2),x.as_mut_ptr().add(3)){return false}if x[0]&0x3ff!=MCP_SEND_EDID_CEA{return false}if x[1]==EDID_CEA_CMD_ACK as u32{true}else{*offset=x[2]as i32;false}}

unsafe fn dce_dmcu_create(ctx:*mut dc_context,r:*const dce_dmcu_registers,s:*const dce_dmcu_shift,m:*const dce_dmcu_mask)->*mut dmcu{let d=kzalloc_obj!(dce_dmcu);if d.is_null(){BREAK_TO_DEBUGGER!();return core::ptr::null_mut()}dce_dmcu_construct(d,ctx,r,s,m);&mut (*d).base}
unsafe fn dcn10_dmcu_create(ctx:*mut dc_context,r:*const dce_dmcu_registers,s:*const dce_dmcu_shift,m:*const dce_dmcu_mask)->*mut dmcu{let d=dce_dmcu_create(ctx,r,s,m);if !d.is_null(){(*d).funcs=&dcn10_funcs}d}
unsafe fn dcn20_dmcu_create(ctx:*mut dc_context,r:*const dce_dmcu_registers,s:*const dce_dmcu_shift,m:*const dce_dmcu_mask)->*mut dmcu{let d=dce_dmcu_create(ctx,r,s,m);if !d.is_null(){(*d).funcs=&dcn20_funcs}d}
unsafe fn dcn21_dmcu_create(ctx:*mut dc_context,r:*const dce_dmcu_registers,s:*const dce_dmcu_shift,m:*const dce_dmcu_mask)->*mut dmcu{let d=dce_dmcu_create(ctx,r,s,m);if !d.is_null(){dcn21_dmcu_construct(TO_DCE_DMCU!(d),ctx,r,s,m);(*d).funcs=&dcn21_funcs}d}
unsafe fn dce_dmcu_destroy(dmcu:*mut *mut dmcu){let d=TO_DCE_DMCU!(*dmcu);kfree!(d);*dmcu=core::ptr::null_mut();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
