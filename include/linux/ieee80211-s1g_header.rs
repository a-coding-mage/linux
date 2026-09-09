/* SPDX-License-Identifier: GPL-2.0-only */
/* IEEE 802.11 S1G definitions. */

// Kernel dependencies: linux/types.h, linux/if_ether.h

pub const IEEE80211_S1G_BCN_NEXT_TBTT: u16 = 0x100;
pub const IEEE80211_S1G_BCN_CSSID: u16 = 0x200;
pub const IEEE80211_S1G_BCN_ANO: u16 = 0x400;
pub const IEEE80211_S1G_1MHZ_NDP_BITS: usize = 25;
pub const IEEE80211_S1G_1MHZ_NDP_BYTES: usize = 4;
pub const IEEE80211_S1G_2MHZ_NDP_BITS: usize = 37;
pub const IEEE80211_S1G_2MHZ_NDP_BYTES: usize = 5;

#[inline]
pub fn ieee80211_is_s1g_beacon(fc: __le16) -> bool {
    (fc & cpu_to_le16(IEEE80211_FCTL_FTYPE | IEEE80211_FCTL_STYPE)) ==
        cpu_to_le16(IEEE80211_FTYPE_EXT | IEEE80211_STYPE_S1G_BEACON)
}

#[inline]
pub fn ieee80211_s1g_has_next_tbtt(fc: __le16) -> bool {
    ieee80211_is_s1g_beacon(fc) && (fc & cpu_to_le16(IEEE80211_S1G_BCN_NEXT_TBTT)) != 0
}
#[inline]
pub fn ieee80211_s1g_has_ano(fc: __le16) -> bool {
    ieee80211_is_s1g_beacon(fc) && (fc & cpu_to_le16(IEEE80211_S1G_BCN_ANO)) != 0
}
#[inline]
pub fn ieee80211_s1g_has_cssid(fc: __le16) -> bool {
    ieee80211_is_s1g_beacon(fc) && (fc & cpu_to_le16(IEEE80211_S1G_BCN_CSSID)) != 0
}

#[repr(u8)]
pub enum ieee80211_s1g_chanwidth { IEEE80211_S1G_CHANWIDTH_1MHZ=0, IEEE80211_S1G_CHANWIDTH_2MHZ=1, IEEE80211_S1G_CHANWIDTH_4MHZ=3, IEEE80211_S1G_CHANWIDTH_8MHZ=7, IEEE80211_S1G_CHANWIDTH_16MHZ=15 }
#[repr(u8)]
pub enum ieee80211_s1g_pri_chanwidth { IEEE80211_S1G_PRI_CHANWIDTH_2MHZ=0, IEEE80211_S1G_PRI_CHANWIDTH_1MHZ=1 }

#[repr(C, packed)]
pub struct ieee80211_s1g_bcn_compat_ie { pub compat_info: __le16, pub beacon_int: __le16, pub tsf_completion: __le32 }
#[repr(C, packed)]
pub struct ieee80211_s1g_oper_ie { pub ch_width:u8, pub oper_class:u8, pub primary_ch:u8, pub oper_ch:u8, pub basic_mcs_nss:__le16 }
#[repr(C, packed)]
pub struct ieee80211_aid_response_ie { pub aid:__le16, pub switch_count:u8, pub response_int:__le16 }
#[repr(C, packed)]
pub struct ieee80211_s1g_cap { pub capab_info:[u8;10], pub supp_mcs_nss:[u8;5] }

#[inline]
pub fn ieee80211_s1g_optional_len(fc: __le16) -> usize {
    let mut len=0; if ieee80211_s1g_has_next_tbtt(fc){len+=3;} if ieee80211_s1g_has_cssid(fc){len+=4;} if ieee80211_s1g_has_ano(fc){len+=1;} len
}

pub const IEEE80211_S1G_CAPABILITY_LEN:usize=15;
pub const S1G_CAP0_S1G_LONG:u8=1<<0; pub const S1G_CAP0_SGI_1MHZ:u8=1<<1; pub const S1G_CAP0_SGI_2MHZ:u8=1<<2; pub const S1G_CAP0_SGI_4MHZ:u8=1<<3; pub const S1G_CAP0_SGI_8MHZ:u8=1<<4; pub const S1G_CAP0_SGI_16MHZ:u8=1<<5; pub const S1G_CAP0_SUPP_CH_WIDTH:u8=0xc0;
pub const S1G_SUPP_CH_WIDTH_2:u8=0; pub const S1G_SUPP_CH_WIDTH_4:u8=1; pub const S1G_SUPP_CH_WIDTH_8:u8=2; pub const S1G_SUPP_CH_WIDTH_16:u8=3;
#[inline] pub fn S1G_SUPP_CH_WIDTH_MAX(cap:&[u8]) -> u8 { (1u8 << ((cap[0] & S1G_CAP0_SUPP_CH_WIDTH) >> 6)) << 1 }
pub const S1G_CAP1_RX_LDPC:u8=1; pub const S1G_CAP1_TX_STBC:u8=2; pub const S1G_CAP1_RX_STBC:u8=4; pub const S1G_CAP1_SU_BFER:u8=8; pub const S1G_CAP1_SU_BFEE:u8=16; pub const S1G_CAP1_BFEE_STS:u8=0xe0;
pub const S1G_CAP2_SOUNDING_DIMENSIONS:u8=7; pub const S1G_CAP2_MU_BFER:u8=8; pub const S1G_CAP2_MU_BFEE:u8=16; pub const S1G_CAP2_PLUS_HTC_VHT:u8=32; pub const S1G_CAP2_TRAVELING_PILOT:u8=0xc0;
pub const S1G_CAP3_RD_RESPONDER:u8=1; pub const S1G_CAP3_HT_DELAYED_BA:u8=2; pub const S1G_CAP3_MAX_MPDU_LEN:u8=4; pub const S1G_CAP3_MAX_AMPDU_LEN_EXP:u8=0x18; pub const S1G_CAP3_MIN_MPDU_START:u8=0xe0;
pub const S1G_CAP4_UPLINK_SYNC:u8=1; pub const S1G_CAP4_DYNAMIC_AID:u8=2; pub const S1G_CAP4_BAT:u8=4; pub const S1G_CAP4_TIME_ADE:u8=8; pub const S1G_CAP4_NON_TIM:u8=16; pub const S1G_CAP4_GROUP_AID:u8=32; pub const S1G_CAP4_STA_TYPE:u8=0xc0;
pub const S1G_CAP5_CENT_AUTH_CONTROL:u8=1; pub const S1G_CAP5_DIST_AUTH_CONTROL:u8=2; pub const S1G_CAP5_AMSDU:u8=4; pub const S1G_CAP5_AMPDU:u8=8; pub const S1G_CAP5_ASYMMETRIC_BA:u8=16; pub const S1G_CAP5_FLOW_CONTROL:u8=32; pub const S1G_CAP5_SECTORIZED_BEAM:u8=0xc0;
pub const S1G_CAP6_OBSS_MITIGATION:u8=1; pub const S1G_CAP6_FRAGMENT_BA:u8=2; pub const S1G_CAP6_NDP_PS_POLL:u8=4; pub const S1G_CAP6_RAW_OPERATION:u8=8; pub const S1G_CAP6_PAGE_SLICING:u8=16; pub const S1G_CAP6_TXOP_SHARING_IMP_ACK:u8=32; pub const S1G_CAP6_VHT_LINK_ADAPT:u8=0xc0;
pub const S1G_CAP7_TACK_AS_PS_POLL:u8=1; pub const S1G_CAP7_DUP_1MHZ:u8=2; pub const S1G_CAP7_MCS_NEGOTIATION:u8=4; pub const S1G_CAP7_1MHZ_CTL_RESPONSE_PREAMBLE:u8=8; pub const S1G_CAP7_NDP_BFING_REPORT_POLL:u8=16; pub const S1G_CAP7_UNSOLICITED_DYN_AID:u8=32; pub const S1G_CAP7_SECTOR_TRAINING_OPERATION:u8=64; pub const S1G_CAP7_TEMP_PS_MODE_SWITCH:u8=128;
pub const S1G_CAP8_TWT_GROUPING:u8=1; pub const S1G_CAP8_BDT:u8=2; pub const S1G_CAP8_COLOR:u8=0x1c; pub const S1G_CAP8_TWT_REQUEST:u8=32; pub const S1G_CAP8_TWT_RESPOND:u8=64; pub const S1G_CAP8_PV1_FRAME:u8=128; pub const S1G_CAP9_LINK_ADAPT_PER_CONTROL_RESPONSE:u8=1;
pub const S1G_OPER_CH_WIDTH_PRIMARY:u8=1; pub const S1G_OPER_CH_WIDTH_OPER:u8=0x1e; pub const S1G_OPER_CH_PRIMARY_LOCATION:u8=0x20; pub const S1G_2M_PRIMARY_LOCATION_LOWER:u8=0; pub const S1G_2M_PRIMARY_LOCATION_UPPER:u8=1;
pub const LISTEN_INT_USF:u16=0xc000; pub const LISTEN_INT_UI:u16=0x3fff; pub const IEEE80211_MAX_USF:u16=3; pub const IEEE80211_MAX_UI:u16=0x3fff;
pub const IEEE80211_S1G_TIM_ENC_MODE_BLOCK:u8=0; pub const IEEE80211_S1G_TIM_ENC_MODE_SINGLE:u8=1; pub const IEEE80211_S1G_TIM_ENC_MODE_OLB:u8=2;

#[repr(u8)] pub enum ieee80211_s1g_actioncode { WLAN_S1G_AID_SWITCH_REQUEST=0, WLAN_S1G_AID_SWITCH_RESPONSE, WLAN_S1G_SYNC_CONTROL, WLAN_S1G_STA_INFO_ANNOUNCE, WLAN_S1G_EDCA_PARAM_SET, WLAN_S1G_EL_OPERATION, WLAN_S1G_TWT_SETUP, WLAN_S1G_TWT_TEARDOWN, WLAN_S1G_SECT_GROUP_ID_LIST, WLAN_S1G_SECT_ID_FEEDBACK, WLAN_S1G_TWT_INFORMATION=11 }
#[repr(C)] pub struct s1g_tim_aid { pub aid:u16, pub target_blk:u8, pub target_subblk:u8, pub target_subblk_bit:u8 }
#[repr(C)] pub struct s1g_tim_enc_block { pub enc_mode:u8, pub inverse:bool, pub ptr:*const u8, pub len:u8, pub olb_blk_offset:u8 }

#[inline] pub unsafe fn ieee80211_is_s1g_short_beacon(fc:__le16, variable:*const u8, variable_len:usize)->bool { if !ieee80211_is_s1g_beacon(fc){return false;} if variable_len<2{return true;} *variable != WLAN_EID_S1G_BCN_COMPAT }
#[inline] pub unsafe fn ieee80211_s1g_len_bitmap(ptr:*const u8,end:*const u8)->i32 { if ptr>=end{return -EINVAL;} let n=hweight8(*ptr); if ptr.add(1+n as usize)>end{return -EINVAL;} 1+n as i32 }
#[inline] pub unsafe fn ieee80211_s1g_len_single(ptr:*const u8,end:*const u8)->i32 { if ptr.add(1)>end{-EINVAL}else{1} }
#[inline] pub unsafe fn ieee80211_s1g_len_olb(ptr:*const u8,end:*const u8)->i32 { if ptr>=end{return -EINVAL;} let n=*ptr as usize; if ptr.add(1+n)>end{-EINVAL}else{1+n as i32} }

// The remaining inline TIM parser uses kernel pointer arithmetic and helpers.
#[inline] pub unsafe fn ieee80211_s1g_find_target_block(enc:&mut s1g_tim_enc_block,aid:&s1g_tim_aid,mut ptr:*const u8,end:*const u8)->i32 { while ptr.add(1)<=end { let ctrl=*ptr; ptr=ptr.add(1); let mode=ctrl&3; let inverse=ctrl&4!=0; let blk_off=ctrl>>3; let len=match mode {0=>ieee80211_s1g_len_bitmap(ptr,end),1=>ieee80211_s1g_len_single(ptr,end),2=>ieee80211_s1g_len_olb(ptr,end),_=>return -EOPNOTSUPP}; if len<0{return len;} let span=if mode==2 {(len as usize-1+7)/8}else{0}; let contains=if mode==2 {aid.target_blk>=blk_off&& (aid.target_blk as usize)<blk_off as usize+span}else{blk_off==aid.target_blk}; if contains {enc.enc_mode=mode;enc.inverse=inverse;enc.ptr=ptr;enc.len=len as u8;enc.olb_blk_offset=blk_off;return 0;} ptr=ptr.add(len as usize);} -ENOENT }
#[inline] pub unsafe fn ieee80211_s1g_parse_bitmap(enc:&s1g_tim_enc_block,aid:&s1g_tim_aid)->bool { let mut p=enc.ptr;let map=*p;p=p.add(1);if map&(1<<aid.target_subblk)==0{return enc.inverse;} if aid.target_subblk!=0{p=p.add((hweight8(map&((1<<aid.target_subblk)-1))) as usize);} ((*p&(1<<aid.target_subblk_bit))!=0)^enc.inverse }
#[inline] pub unsafe fn ieee80211_s1g_parse_single(enc:&s1g_tim_enc_block,aid:&s1g_tim_aid)->bool { ((*enc.ptr&0x3f)==(aid.aid as u8&0x3f))^enc.inverse }
#[inline] pub unsafe fn ieee80211_s1g_parse_olb(enc:&s1g_tim_enc_block,aid:&s1g_tim_aid)->bool { let n=*enc.ptr;let off=aid.target_blk-enc.olb_blk_offset;let idx=off as usize*8+aid.target_subblk as usize;if idx>=n as usize{return enc.inverse;} (*enc.ptr.add(1+idx)&(1<<aid.target_subblk_bit)!=0)^enc.inverse }
#[inline] pub unsafe fn ieee80211_s1g_check_tim(tim:*const ieee80211_tim_ie,tim_len:u8,aid:u16)->bool { if tim_len<3{return false;} let target=s1g_tim_aid{aid,target_blk:((aid>>6)&0x1f) as u8,target_subblk:((aid>>3)&7) as u8,target_subblk_bit:(aid&7) as u8}; let mut enc=s1g_tim_enc_block{enc_mode:0,inverse:false,ptr:std::ptr::null(),len:0,olb_blk_offset:0}; let err=ieee80211_s1g_find_target_block(&mut enc,&target,(*tim).virtual_map,(tim as *const u8).add(tim_len as usize)); if err!=0{return false;} match enc.enc_mode {0=>ieee80211_s1g_parse_bitmap(&enc,&target),1=>ieee80211_s1g_parse_single(&enc,&target),2=>ieee80211_s1g_parse_olb(&enc,&target),_=>false} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
