/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright 2015-2017 Google, Inc */

// Linux dependencies: bitfield/kernel/types/usb typec.

#[repr(u32)]
pub enum pd_ctrl_msg_type { PD_CTRL_GOOD_CRC=1, PD_CTRL_GOTO_MIN, PD_CTRL_ACCEPT, PD_CTRL_REJECT, PD_CTRL_PING, PD_CTRL_PS_RDY, PD_CTRL_GET_SOURCE_CAP, PD_CTRL_GET_SINK_CAP, PD_CTRL_DR_SWAP, PD_CTRL_PR_SWAP, PD_CTRL_VCONN_SWAP, PD_CTRL_WAIT, PD_CTRL_SOFT_RESET, PD_CTRL_NOT_SUPP=16, PD_CTRL_GET_SOURCE_CAP_EXT, PD_CTRL_GET_STATUS, PD_CTRL_FR_SWAP, PD_CTRL_GET_PPS_STATUS, PD_CTRL_GET_COUNTRY_CODES, PD_CTRL_GET_SINK_CAP_EXT, PD_CTRL_GET_REVISION=24 }
#[repr(u32)]
pub enum pd_data_msg_type { PD_DATA_SOURCE_CAP=1, PD_DATA_REQUEST, PD_DATA_BIST, PD_DATA_SINK_CAP, PD_DATA_BATT_STATUS, PD_DATA_ALERT, PD_DATA_GET_COUNTRY_INFO, PD_DATA_ENTER_USB, PD_DATA_REVISION=12, PD_DATA_VENDOR_DEF=15 }
#[repr(u32)]
pub enum pd_ext_msg_type { PD_EXT_SOURCE_CAP_EXT=1, PD_EXT_STATUS, PD_EXT_GET_BATT_CAP, PD_EXT_GET_BATT_STATUS, PD_EXT_BATT_CAP, PD_EXT_GET_MANUFACTURER_INFO, PD_EXT_MANUFACTURER_INFO, PD_EXT_SECURITY_REQUEST, PD_EXT_SECURITY_RESPONSE, PD_EXT_FW_UPDATE_REQUEST, PD_EXT_FW_UPDATE_RESPONSE, PD_EXT_PPS_STATUS, PD_EXT_COUNTRY_INFO, PD_EXT_COUNTRY_CODES, PD_EXT_SINK_CAP_EXT }

pub const PD_REV10:u32=0; pub const PD_REV20:u32=1; pub const PD_REV30:u32=2; pub const PD_MAX_REV:u32=PD_REV30;
pub const PD_HEADER_EXT_HDR:u16=1<<15; pub const PD_HEADER_CNT_SHIFT:u32=12; pub const PD_HEADER_CNT_MASK:u16=7; pub const PD_HEADER_ID_SHIFT:u32=9; pub const PD_HEADER_ID_MASK:u16=7; pub const PD_HEADER_PWR_ROLE:u16=1<<8; pub const PD_HEADER_REV_SHIFT:u32=6; pub const PD_HEADER_REV_MASK:u16=3; pub const PD_HEADER_DATA_ROLE:u16=1<<5; pub const PD_HEADER_TYPE_SHIFT:u32=0; pub const PD_HEADER_TYPE_MASK:u16=0x1f;
pub const fn pd_header(type_:u16,pwr:u32,data:u32,rev:u16,id:u16,cnt:u16,ext_hdr:bool)->u16 { ((type_&PD_HEADER_TYPE_MASK)<<PD_HEADER_TYPE_SHIFT)|((data==1) as u16*PD_HEADER_DATA_ROLE)|((rev)<<PD_HEADER_REV_SHIFT)|((id&PD_HEADER_ID_MASK)<<PD_HEADER_ID_SHIFT)|((cnt&PD_HEADER_CNT_MASK)<<PD_HEADER_CNT_SHIFT)|((ext_hdr as u16)*PD_HEADER_EXT_HDR) }
pub const fn pd_header_le(type_:u16,pwr:u32,data:u32,rev:u16,id:u16,cnt:u16)->u16 { pd_header(type_,pwr,data,rev,id,cnt,false).to_le() }
pub const fn pd_header_ext_le(type_:u16,pwr:u32,data:u32,rev:u16,id:u16,cnt:u16)->u16 { pd_header(type_,pwr,data,rev,id,cnt,true).to_le() }
pub const fn pd_header_cnt(h:u16)->u32 { ((h as u32)>>PD_HEADER_CNT_SHIFT)&PD_HEADER_CNT_MASK as u32 }
pub const fn pd_header_cnt_le(h:u16)->u32 { pd_header_cnt(u16::from_le(h)) }
pub const fn pd_header_type(h:u16)->u32 { ((h as u32)>>PD_HEADER_TYPE_SHIFT)&PD_HEADER_TYPE_MASK as u32 }
pub const fn pd_header_type_le(h:u16)->u32 { pd_header_type(u16::from_le(h)) }
pub const fn pd_header_msgid(h:u16)->u32 { ((h as u32)>>PD_HEADER_ID_SHIFT)&PD_HEADER_ID_MASK as u32 }
pub const fn pd_header_msgid_le(h:u16)->u32 { pd_header_msgid(u16::from_le(h)) }
pub const fn pd_header_rev(h:u16)->u32 { ((h as u32)>>PD_HEADER_REV_SHIFT)&PD_HEADER_REV_MASK as u32 }
pub const fn pd_header_rev_le(h:u16)->u32 { pd_header_rev(u16::from_le(h)) }

pub const PD_EXT_HDR_CHUNKED:u16=1<<15; pub const PD_EXT_HDR_CHUNK_NUM_SHIFT:u32=11; pub const PD_EXT_HDR_CHUNK_NUM_MASK:u16=0xf; pub const PD_EXT_HDR_REQ_CHUNK:u16=1<<10; pub const PD_EXT_HDR_DATA_SIZE_SHIFT:u32=0; pub const PD_EXT_HDR_DATA_SIZE_MASK:u16=0x1ff;
pub const fn pd_ext_hdr(data_size:u16,req_chunk:bool,chunk_num:u16,chunked:bool)->u16 { ((data_size&PD_EXT_HDR_DATA_SIZE_MASK)<<PD_EXT_HDR_DATA_SIZE_SHIFT)|((req_chunk as u16)*PD_EXT_HDR_REQ_CHUNK)|((chunk_num&PD_EXT_HDR_CHUNK_NUM_MASK)<<PD_EXT_HDR_CHUNK_NUM_SHIFT)|((chunked as u16)*PD_EXT_HDR_CHUNKED) }
pub const fn pd_ext_hdr_le(a:u16,b:bool,c:u16,d:bool)->u16 { pd_ext_hdr(a,b,c,d).to_le() }
pub const fn pd_ext_header_chunk_num(h:u16)->u32 { ((h as u32)>>11)&0xf }
pub const fn pd_ext_header_data_size(h:u16)->u32 { (h as u32)&0x1ff }
pub const fn pd_ext_header_data_size_le(h:u16)->u32 { pd_ext_header_data_size(u16::from_le(h)) }

pub const PD_MAX_PAYLOAD:usize=7; pub const PD_EXT_MAX_CHUNK_DATA:usize=26;
#[repr(C,packed)] pub struct pd_chunked_ext_message_data { pub header:u16, pub data:[u8;PD_EXT_MAX_CHUNK_DATA] }
#[repr(C)] pub union pd_message_body { pub payload:[u32;PD_MAX_PAYLOAD], pub ext_msg:pd_chunked_ext_message_data }
#[repr(C,packed)] pub struct pd_message { pub header:u16, pub body:pd_message_body }
pub const fn count_chunked_data_objs(size:u32)->u8 { let s=size+2; ((s/4)+(if s%4!=0 {1}else{0})) as u8 }
#[repr(C,packed)] pub struct batt_cap_ext_msg { pub vid:u16,pub pid:u16,pub batt_design_cap:u16,pub batt_last_chg_cap:u16,pub batt_type:u8 }
pub const BATT_CAP_BATT_TYPE_INVALID_REF:u32=1;
pub const SKEDB_VER_1_0:u32=1; pub const SINK_MODE_PPS:u32=1; pub const SINK_MODE_VBUS:u32=2; pub const SINK_MODE_AC_SUPPLY:u32=4; pub const SINK_MODE_BATT:u32=8; pub const SINK_MODE_BATT_UL:u32=16; pub const SINK_MODE_AVS:u32=32;
#[repr(C,packed)] pub struct sink_caps_ext_msg { pub vid:u16,pub pid:u16,pub xid:u32,pub fw:u8,pub hw:u8,pub skedb_ver:u8,pub load_step:u8,pub load_char:u16,pub compliance:u8,pub touch_temp:u8,pub batt_info:u8,pub modes:u8,pub spr_min_pdp:u8,pub spr_op_pdp:u8,pub spr_max_pdp:u8,pub epr_min_pdp:u8,pub epr_op_pdp:u8,pub epr_max_pdp:u8 }

pub const PDO_MAX_OBJECTS:usize=7;
#[repr(u32)] pub enum pd_pdo_type { PDO_TYPE_FIXED=0, PDO_TYPE_BATT=1, PDO_TYPE_VAR=2, PDO_TYPE_APDO=3 }
pub const PDO_TYPE_SHIFT:u32=30; pub const PDO_TYPE_MASK:u32=3; pub const fn PDO_TYPE(t:u32)->u32{t<<30}
pub const PDO_VOLT_MASK:u32=0x3ff; pub const PDO_CURR_MASK:u32=0x3ff; pub const PDO_PWR_MASK:u32=0x3ff;
pub const PDO_FIXED_DUAL_ROLE:u32=1<<29; pub const PDO_FIXED_SUSPEND:u32=1<<28; pub const PDO_FIXED_HIGHER_CAP:u32=1<<28; pub const PDO_FIXED_EXTPOWER:u32=1<<27; pub const PDO_FIXED_USB_COMM:u32=1<<26; pub const PDO_FIXED_DATA_SWAP:u32=1<<25; pub const PDO_FIXED_UNCHUNK_EXT:u32=1<<24; pub const PDO_FIXED_FRS_CURR_MASK:u32=(1<<24)|(1<<23); pub const PDO_FIXED_FRS_CURR_SHIFT:u32=23; pub const PDO_FIXED_PEAK_CURR_SHIFT:u32=20; pub const PDO_FIXED_VOLT_SHIFT:u32=10; pub const PDO_FIXED_CURR_SHIFT:u32=0;
pub const fn PDO_FIXED_VOLT(mv:u32)->u32{((mv/50)&PDO_VOLT_MASK)<<10} pub const fn PDO_FIXED_CURR(ma:u32)->u32{((ma/10)&PDO_CURR_MASK)} pub const fn PDO_FIXED(mv:u32,ma:u32,flags:u32)->u32{PDO_TYPE(0)|flags|PDO_FIXED_VOLT(mv)|PDO_FIXED_CURR(ma)} pub const VSAFE5V:u32=5000;
pub const PDO_BATT_MAX_VOLT_SHIFT:u32=20; pub const PDO_BATT_MIN_VOLT_SHIFT:u32=10; pub const PDO_BATT_MAX_PWR_SHIFT:u32=0;
pub const fn PDO_BATT_MIN_VOLT(mv:u32)->u32{((mv/50)&PDO_VOLT_MASK)<<10} pub const fn PDO_BATT_MAX_VOLT(mv:u32)->u32{((mv/50)&PDO_VOLT_MASK)<<20} pub const fn PDO_BATT_MAX_POWER(mw:u32)->u32{((mw/250)&PDO_PWR_MASK)} pub const fn PDO_BATT(a:u32,b:u32,c:u32)->u32{PDO_TYPE(1)|PDO_BATT_MIN_VOLT(a)|PDO_BATT_MAX_VOLT(b)|PDO_BATT_MAX_POWER(c)}
pub const PDO_VAR_MIN_VOLT_SHIFT:u32=10; pub const PDO_VAR_MAX_VOLT_SHIFT:u32=20; pub const fn PDO_VAR_MIN_VOLT(mv:u32)->u32{((mv/50)&PDO_VOLT_MASK)<<10} pub const fn PDO_VAR_MAX_VOLT(mv:u32)->u32{((mv/50)&PDO_VOLT_MASK)<<20} pub const fn PDO_VAR_MAX_CURR(ma:u32)->u32{(ma/10)&PDO_CURR_MASK} pub const fn PDO_VAR(a:u32,b:u32,c:u32)->u32{PDO_TYPE(2)|PDO_VAR_MIN_VOLT(a)|PDO_VAR_MAX_VOLT(b)|PDO_VAR_MAX_CURR(c)}
#[repr(u32)] pub enum pd_apdo_type { APDO_TYPE_PPS=0, APDO_TYPE_EPR_AVS=1, APDO_TYPE_SPR_AVS=2 }
pub const PDO_APDO_TYPE_SHIFT:u32=28; pub const PDO_APDO_TYPE_MASK:u32=3; pub const PDO_PPS_APDO_MIN_VOLT_SHIFT:u32=8; pub const PDO_PPS_APDO_MAX_VOLT_SHIFT:u32=17; pub const PDO_PPS_APDO_MAX_CURR_SHIFT:u32=0; pub const PDO_PPS_APDO_VOLT_MASK:u32=0xff; pub const PDO_PPS_APDO_CURR_MASK:u32=0x7f;
pub const fn PDO_APDO_TYPE(t:u32)->u32{t<<28} pub const fn PDO_PPS_APDO_MIN_VOLT(mv:u32)->u32{((mv/100)&255)<<8} pub const fn PDO_PPS_APDO_MAX_VOLT(mv:u32)->u32{((mv/100)&255)<<17} pub const fn PDO_PPS_APDO_MAX_CURR(ma:u32)->u32{((ma/50)&127)} pub const fn PDO_PPS_APDO(a:u32,b:u32,c:u32)->u32{PDO_TYPE(3)|PDO_APDO_TYPE(0)|PDO_PPS_APDO_MIN_VOLT(a)|PDO_PPS_APDO_MAX_VOLT(b)|PDO_PPS_APDO_MAX_CURR(c)}

pub const PDO_EPR_AVS_APDO_PEAK_CURRENT:u32=0x0c000000; pub const PDO_EPR_AVS_APDO_MAX_VOLT:u32=0x03fe0000; pub const PDO_EPR_AVS_APDO_MIN_VOLT:u32=0x0000ff00; pub const PDO_EPR_AVS_APDO_PDP:u32=0xff; pub const PDO_SPR_AVS_APDO_PEAK_CURRENT:u32=0x0c000000; pub const PDO_SPR_AVS_APDO_9V_TO_15V_MAX_CURR:u32=0x000ffc00; pub const PDO_SPR_AVS_APDO_15V_TO_20V_MAX_CURR:u32=0x3ff;
pub const SPR_AVS_TIER1_MIN_VOLT_MV:u32=9000; pub const SPR_AVS_TIER1_MAX_VOLT_MV:u32=15000; pub const SPR_AVS_TIER2_MAX_VOLT_MV:u32=20000; pub const SPR_AVS_AVS_SMALL_STEP_V:u32=1; pub const SPR_AVS_VOLT_MV_STEP:u32=100; pub const RDO_SPR_AVS_CURR_MA_STEP:u32=50; pub const RDO_SPR_AVS_OUT_VOLT_MV_STEP:u32=25; pub const RDO_SPR_AVS_VOLT:u32=0x001ffe00; pub const RDO_SPR_AVS_CURR:u32=0x7f;
pub const fn field_prep(mask:u32,v:u32)->u32{(v<<mask.trailing_zeros())&mask} pub const fn field_get(mask:u32,v:u32)->u32{(v&mask)>>mask.trailing_zeros()}
pub const fn RDO_SPR_AVS_OUT_VOLT(mv:u32)->u32{field_prep(RDO_SPR_AVS_VOLT,mv/25)} pub const fn RDO_SPR_AVS_OP_CURR(ma:u32)->u32{field_prep(RDO_SPR_AVS_CURR,ma/50)}
pub const fn pdo_type(p:u32)->u32{(p>>30)&3} pub const fn pdo_fixed_voltage(p:u32)->u32{((p>>10)&1023)*50} pub const fn pdo_fixed_current(p:u32)->u32{p&1023*10} pub const fn pdo_min_voltage(p:u32)->u32{((p>>10)&1023)*50} pub const fn pdo_max_voltage(p:u32)->u32{((p>>20)&1023)*50} pub const fn pdo_max_current(p:u32)->u32{(p&1023)*10} pub const fn pdo_max_power(p:u32)->u32{(p&1023)*250} pub const fn pdo_apdo_type(p:u32)->u32{(p>>28)&3}
pub const RDO_OBJ_POS_SHIFT:u32=28; pub const RDO_OBJ_POS_MASK:u32=7; pub const RDO_GIVE_BACK:u32=1<<27; pub const RDO_CAP_MISMATCH:u32=1<<26; pub const RDO_USB_COMM:u32=1<<25; pub const RDO_NO_SUSPEND:u32=1<<24; pub const RDO_PWR_MASK:u32=1023; pub const RDO_CURR_MASK:u32=1023; pub const RDO_FIXED_OP_CURR_SHIFT:u32=10; pub const RDO_FIXED_MAX_CURR_SHIFT:u32=0;
pub const fn RDO_OBJ(i:u32)->u32{(i&7)<<28} pub const fn PDO_FIXED_OP_CURR(ma:u32)->u32{((ma/10)&1023)<<10} pub const fn PDO_FIXED_MAX_CURR(ma:u32)->u32{(ma/10)&1023} pub const fn RDO_FIXED(i:u32,a:u32,b:u32,f:u32)->u32{RDO_OBJ(i)|f|PDO_FIXED_OP_CURR(a)|PDO_FIXED_MAX_CURR(b)} pub const RDO_BATT_OP_PWR_SHIFT:u32=10; pub const RDO_BATT_MAX_PWR_SHIFT:u32=0; pub const fn RDO_BATT_OP_PWR(m:u32)->u32{((m/250)&1023)<<10} pub const fn RDO_BATT_MAX_PWR(m:u32)->u32{(m/250)&1023} pub const fn RDO_BATT(i:u32,a:u32,b:u32,f:u32)->u32{RDO_OBJ(i)|f|RDO_BATT_OP_PWR(a)|RDO_BATT_MAX_PWR(b)}
pub const fn rdo_index(r:u32)->u32{(r>>28)&7} pub const fn rdo_op_current(r:u32)->u32{((r>>10)&1023)*10} pub const fn rdo_max_current(r:u32)->u32{(r&1023)*10} pub const fn rdo_op_power(r:u32)->u32{((r>>10)&1023)*250} pub const fn rdo_max_power(r:u32)->u32{(r&1023)*250}
pub const fn RMDO(a:u32,b:u32,c:u32,d:u32)->u32{(a&15)<<28|(b&15)<<24|(c&15)<<20|(d&15)<<16} pub const PD_N_HARD_RESET_COUNT:u32=2; pub const PD_P_SNK_STDBY_MW:u32=2500; pub const PD_I_SNK_STBY_MA:u32=500;
pub const BSDO_PRESENT_CAPACITY:u32=0xffff0000; pub const BSDO_CHG_STATUS:u32=0xc00; pub const BSDO_BATTERY_PRESENT:u32=1<<9; pub const BSDO_INVALID_BATTERY_REFERENCE:u32=1<<8; pub const BSDO_BATTERY_INFO_CHARGING:u32=0; pub const BSDO_BATTERY_INFO_DISCHARGING:u32=1; pub const BSDO_BATTERY_INFO_IDLE:u32=2; pub const BSDO_BATTERY_INFO_RSVD:u32=3; pub const fn BSDO(a:u32,b:u32,c:bool,d:bool)->u32{field_prep(BSDO_PRESENT_CAPACITY,a)|field_prep(BSDO_CHG_STATUS,b)|((c as u32)<<9)|((d as u32)<<8)}
pub const PD_T_NO_RESPONSE:u32=5000; pub const PD_T_DB_DETECT:u32=10000; pub const PD_T_SEND_SOURCE_CAP:u32=150; pub const PD_T_SENDER_RESPONSE:u32=60; pub const PD_T_RECEIVER_RESPONSE:u32=15; pub const PD_T_SOURCE_ACTIVITY:u32=45; pub const PD_T_SINK_ACTIVITY:u32=135; pub const PD_T_SINK_WAIT_CAP:u32=310; pub const PD_T_PS_TRANSITION:u32=500; pub const PD_T_SRC_TRANSITION:u32=35; pub const PD_T_DRP_SNK:u32=40; pub const PD_T_DRP_SRC:u32=30; pub const PD_T_PS_SOURCE_OFF:u32=920; pub const PD_T_PS_SOURCE_ON:u32=480; pub const PD_T_PS_SOURCE_ON_PRS:u32=450; pub const PD_T_PS_HARD_RESET:u32=30; pub const PD_T_SRC_RECOVER:u32=760; pub const PD_T_SRC_RECOVER_MAX:u32=1000; pub const PD_T_SRC_TURN_ON:u32=275; pub const PD_T_SAFE_0V:u32=650; pub const PD_T_VCONN_SOURCE_ON:u32=100; pub const PD_T_SINK_REQUEST:u32=100; pub const PD_T_ERROR_RECOVERY:u32=100; pub const PD_T_SRCSWAPSTDBY:u32=625; pub const PD_T_NEWSRC:u32=250; pub const PD_T_SWAP_SRC_START:u32=20; pub const PD_T_BIST_CONT_MODE:u32=50; pub const PD_T_SINK_TX:u32=16; pub const PD_T_CHUNK_NOT_SUPP:u32=42; pub const PD_T_VCONN_STABLE:u32=50; pub const PD_T_DRP_TRY:u32=100; pub const PD_T_DRP_TRYWAIT:u32=600; pub const PD_T_CC_DEBOUNCE:u32=200; pub const PD_T_PD_DEBOUNCE:u32=20; pub const PD_T_TRY_CC_DEBOUNCE:u32=15; pub const PD_N_CAPS_COUNT:u32=PD_T_NO_RESPONSE/PD_T_SEND_SOURCE_CAP; pub const PD_T_AVS_SRC_TRANS_SMALL:u32=50; pub const PD_T_AVS_SRC_TRANS_LARGE:u32=700;
// CONFIG_TYPEC conditional declarations are intentionally represented as external interfaces.
#[repr(C)] pub struct usb_power_delivery; #[repr(C)] pub struct usb_power_delivery_capabilities; #[repr(C)] pub struct device;
#[repr(C)] pub struct usb_power_delivery_desc { pub revision:u16, pub version:u16 }
#[repr(C)] pub struct usb_power_delivery_capabilities_desc { pub pdo:[u32;PDO_MAX_OBJECTS], pub role:u32 }
extern "C" { pub fn usb_power_delivery_register_capabilities(pd:*mut usb_power_delivery, desc:*mut usb_power_delivery_capabilities_desc)->*mut usb_power_delivery_capabilities; pub fn usb_power_delivery_unregister_capabilities(cap:*mut usb_power_delivery_capabilities); pub fn usb_power_delivery_register(parent:*mut device,desc:*mut usb_power_delivery_desc)->*mut usb_power_delivery; pub fn usb_power_delivery_unregister(pd:*mut usb_power_delivery); pub fn usb_power_delivery_link_device(pd:*mut usb_power_delivery,dev:*mut device)->i32; pub fn usb_power_delivery_unlink_device(pd:*mut usb_power_delivery,dev:*mut device); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
