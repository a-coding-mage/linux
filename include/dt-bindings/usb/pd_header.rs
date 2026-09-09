/* SPDX-License-Identifier: GPL-2.0 */

/* Power delivery Power Data Object definitions */
pub const PDO_TYPE_FIXED: u32 = 0;
pub const PDO_TYPE_BATT: u32 = 1;
pub const PDO_TYPE_VAR: u32 = 2;
pub const PDO_TYPE_APDO: u32 = 3;
pub const PDO_TYPE_SHIFT: u32 = 30;
pub const PDO_TYPE_MASK: u32 = 0x3;
pub const PDO_VOLT_MASK: u32 = 0x3ff;
pub const PDO_CURR_MASK: u32 = 0x3ff;
pub const PDO_PWR_MASK: u32 = 0x3ff;
pub const PDO_FIXED_DUAL_ROLE: u32 = 1 << 29;
pub const PDO_FIXED_SUSPEND: u32 = 1 << 28;
pub const PDO_FIXED_HIGHER_CAP: u32 = 1 << 28;
pub const PDO_FIXED_EXTPOWER: u32 = 1 << 27;
pub const PDO_FIXED_USB_COMM: u32 = 1 << 26;
pub const PDO_FIXED_DATA_SWAP: u32 = 1 << 25;
pub const PDO_FIXED_VOLT_SHIFT: u32 = 10;
pub const PDO_FIXED_CURR_SHIFT: u32 = 0;
pub const VSAFE5V: u32 = 5000;

pub const PDO_BATT_MAX_VOLT_SHIFT: u32 = 20;
pub const PDO_BATT_MIN_VOLT_SHIFT: u32 = 10;
pub const PDO_BATT_MAX_PWR_SHIFT: u32 = 0;
pub const PDO_VAR_MAX_VOLT_SHIFT: u32 = 20;
pub const PDO_VAR_MIN_VOLT_SHIFT: u32 = 10;
pub const PDO_VAR_MAX_CURR_SHIFT: u32 = 0;

#[inline] pub const fn PDO_TYPE(t: u32) -> u32 { t << PDO_TYPE_SHIFT }
#[inline] pub const fn PDO_FIXED_VOLT(mv: u32) -> u32 { ((mv / 50) & PDO_VOLT_MASK) << PDO_FIXED_VOLT_SHIFT }
#[inline] pub const fn PDO_FIXED_CURR(ma: u32) -> u32 { ((ma / 10) & PDO_CURR_MASK) << PDO_FIXED_CURR_SHIFT }
#[inline] pub const fn PDO_FIXED(mv: u32, ma: u32, flags: u32) -> u32 { PDO_TYPE(PDO_TYPE_FIXED) | flags | PDO_FIXED_VOLT(mv) | PDO_FIXED_CURR(ma) }
#[inline] pub const fn PDO_BATT_MIN_VOLT(mv: u32) -> u32 { ((mv / 50) & PDO_VOLT_MASK) << PDO_BATT_MIN_VOLT_SHIFT }
#[inline] pub const fn PDO_BATT_MAX_VOLT(mv: u32) -> u32 { ((mv / 50) & PDO_VOLT_MASK) << PDO_BATT_MAX_VOLT_SHIFT }
#[inline] pub const fn PDO_BATT_MAX_POWER(mw: u32) -> u32 { ((mw / 250) & PDO_PWR_MASK) << PDO_BATT_MAX_PWR_SHIFT }
#[inline] pub const fn PDO_BATT(min_mv: u32, max_mv: u32, max_mw: u32) -> u32 { PDO_TYPE(PDO_TYPE_BATT) | PDO_BATT_MIN_VOLT(min_mv) | PDO_BATT_MAX_VOLT(max_mv) | PDO_BATT_MAX_POWER(max_mw) }
#[inline] pub const fn PDO_VAR_MIN_VOLT(mv: u32) -> u32 { ((mv / 50) & PDO_VOLT_MASK) << PDO_VAR_MIN_VOLT_SHIFT }
#[inline] pub const fn PDO_VAR_MAX_VOLT(mv: u32) -> u32 { ((mv / 50) & PDO_VOLT_MASK) << PDO_VAR_MAX_VOLT_SHIFT }
#[inline] pub const fn PDO_VAR_MAX_CURR(ma: u32) -> u32 { ((ma / 10) & PDO_CURR_MASK) << PDO_VAR_MAX_CURR_SHIFT }
#[inline] pub const fn PDO_VAR(min_mv: u32, max_mv: u32, max_ma: u32) -> u32 { PDO_TYPE(PDO_TYPE_VAR) | PDO_VAR_MIN_VOLT(min_mv) | PDO_VAR_MAX_VOLT(max_mv) | PDO_VAR_MAX_CURR(max_ma) }

pub const APDO_TYPE_PPS: u32 = 0;
pub const APDO_TYPE_SPR_AVS: u32 = 2;
pub const PDO_APDO_TYPE_SHIFT: u32 = 28;
pub const PDO_APDO_TYPE_MASK: u32 = 0x3;
pub const PDO_PPS_APDO_MAX_VOLT_SHIFT: u32 = 17;
pub const PDO_PPS_APDO_MIN_VOLT_SHIFT: u32 = 8;
pub const PDO_PPS_APDO_MAX_CURR_SHIFT: u32 = 0;
pub const PDO_PPS_APDO_VOLT_MASK: u32 = 0xff;
pub const PDO_PPS_APDO_CURR_MASK: u32 = 0x7f;
#[inline] pub const fn PDO_APDO_TYPE(t: u32) -> u32 { t << PDO_APDO_TYPE_SHIFT }
#[inline] pub const fn PDO_PPS_APDO_MIN_VOLT(mv: u32) -> u32 { ((mv / 100) & PDO_PPS_APDO_VOLT_MASK) << PDO_PPS_APDO_MIN_VOLT_SHIFT }
#[inline] pub const fn PDO_PPS_APDO_MAX_VOLT(mv: u32) -> u32 { ((mv / 100) & PDO_PPS_APDO_VOLT_MASK) << PDO_PPS_APDO_MAX_VOLT_SHIFT }
#[inline] pub const fn PDO_PPS_APDO_MAX_CURR(ma: u32) -> u32 { ((ma / 50) & PDO_PPS_APDO_CURR_MASK) << PDO_PPS_APDO_MAX_CURR_SHIFT }
#[inline] pub const fn PDO_PPS_APDO(min_mv: u32, max_mv: u32, max_ma: u32) -> u32 { PDO_TYPE(PDO_TYPE_APDO) | PDO_APDO_TYPE(APDO_TYPE_PPS) | PDO_PPS_APDO_MIN_VOLT(min_mv) | PDO_PPS_APDO_MAX_VOLT(max_mv) | PDO_PPS_APDO_MAX_CURR(max_ma) }
pub const PDO_SPR_AVS_APDO_9V_TO_15V_MAX_CURR_SHIFT: u32 = 10;
pub const PDO_SPR_AVS_APDO_15V_TO_20V_MAX_CURR_SHIFT: u32 = 0;
pub const PDO_SPR_AVS_APDO_MAX_CURR_MASK: u32 = 0x3ff;
#[inline] pub const fn PDO_SPR_AVS_APDO_9V_TO_15V_MAX_CURR(ma: u32) -> u32 { ((ma / 10) & PDO_SPR_AVS_APDO_MAX_CURR_MASK) << PDO_SPR_AVS_APDO_9V_TO_15V_MAX_CURR_SHIFT }
#[inline] pub const fn PDO_SPR_AVS_APDO_15V_TO_20V_MAX_CURR(ma: u32) -> u32 { ((ma / 10) & PDO_SPR_AVS_APDO_MAX_CURR_MASK) << PDO_SPR_AVS_APDO_15V_TO_20V_MAX_CURR_SHIFT }
#[inline] pub const fn PDO_SPR_AVS_SNK_APDO(a: u32, b: u32) -> u32 { PDO_TYPE(PDO_TYPE_APDO) | PDO_APDO_TYPE(APDO_TYPE_SPR_AVS) | PDO_SPR_AVS_APDO_9V_TO_15V_MAX_CURR(a) | PDO_SPR_AVS_APDO_15V_TO_20V_MAX_CURR(b) }

pub const FRS_DEFAULT_POWER: u32 = 1;
pub const FRS_5V_1P5A: u32 = 2;
pub const FRS_5V_3A: u32 = 3;
pub const IDH_PTYPE_UNDEF: u32 = 0;
pub const IDH_PTYPE_NOT_UFP: u32 = 0; pub const IDH_PTYPE_HUB: u32 = 1; pub const IDH_PTYPE_PERIPH: u32 = 2; pub const IDH_PTYPE_PSD: u32 = 3; pub const IDH_PTYPE_AMA: u32 = 5;
pub const IDH_PTYPE_NOT_CABLE: u32 = 0; pub const IDH_PTYPE_PCABLE: u32 = 3; pub const IDH_PTYPE_ACABLE: u32 = 4; pub const IDH_PTYPE_VPD: u32 = 6;
pub const IDH_PTYPE_NOT_DFP: u32 = 0; pub const IDH_PTYPE_DFP_HUB: u32 = 1; pub const IDH_PTYPE_DFP_HOST: u32 = 2; pub const IDH_PTYPE_DFP_PB: u32 = 3;
#[inline] pub const fn VDO_IDH(usbh:u32, usbd:u32, ufp_cable:u32, is_modal:u32, dfp:u32, conn:u32, vid:u32)->u32 { usbh<<31 | usbd<<30 | (ufp_cable&7)<<27 | is_modal<<26 | (dfp&7)<<23 | (conn&3)<<21 | (vid&0xffff) }
#[inline] pub const fn VDO_CERT(xid:u32)->u32 { xid & 0xffff_ffff }
#[inline] pub const fn VDO_PRODUCT(pid:u32,bcd:u32)->u32 { (pid&0xffff)<<16 | (bcd&0xffff) }

pub const UFP_VDO_VER1_2:u32=2; pub const DEV_USB2_CAPABLE:u32=1<<0; pub const DEV_USB2_BILLBOARD:u32=1<<1; pub const DEV_USB3_CAPABLE:u32=1<<2; pub const DEV_USB4_CAPABLE:u32=1<<3;
pub const UFP_RECEPTACLE:u32=2; pub const UFP_CAPTIVE:u32=3;
pub const AMA_VCONN_PWR_1W:u32=0; pub const AMA_VCONN_PWR_1W5:u32=1; pub const AMA_VCONN_PWR_2W:u32=2; pub const AMA_VCONN_PWR_3W:u32=3; pub const AMA_VCONN_PWR_4W:u32=4; pub const AMA_VCONN_PWR_5W:u32=5; pub const AMA_VCONN_PWR_6W:u32=6;
pub const AMA_VCONN_NOT_REQ:u32=0; pub const AMA_VCONN_REQ:u32=1; pub const AMA_VBUS_REQ:u32=0; pub const AMA_VBUS_NOT_REQ:u32=1;
pub const UFP_ALTMODE_NOT_SUPP:u32=0; pub const UFP_ALTMODE_TBT3:u32=1<<0; pub const UFP_ALTMODE_RECFG:u32=1<<1; pub const UFP_ALTMODE_NO_RECFG:u32=1<<2;
pub const UFP_USB2_ONLY:u32=0; pub const UFP_USB32_GEN1:u32=1; pub const UFP_USB32_4_GEN2:u32=2; pub const UFP_USB4_GEN3:u32=3;
#[inline] pub const fn VDO_UFP(ver:u32,cap:u32,conn:u32,vcpwr:u32,vcr:u32,vbr:u32,alt:u32,spd:u32)->u32 { (ver&7)<<29 | (cap&0xf)<<24 | (conn&3)<<22 | (vcpwr&7)<<8 | vcr<<7 | vbr<<6 | (alt&7)<<3 | (spd&7) }
pub const DFP_VDO_VER1_1:u32=1; pub const HOST_USB2_CAPABLE:u32=1; pub const HOST_USB3_CAPABLE:u32=2; pub const HOST_USB4_CAPABLE:u32=4; pub const DFP_RECEPTACLE:u32=2; pub const DFP_CAPTIVE:u32=3;
#[inline] pub const fn VDO_DFP(ver:u32,cap:u32,conn:u32,pnum:u32)->u32 { (ver&7)<<29 | (cap&7)<<24 | (conn&3)<<22 | (pnum&0x1f) }

pub const CABLE_VDO_VER1_0:u32=0; pub const CABLE_VDO_VER1_3:u32=3; pub const CABLE_ATYPE:u32=0; pub const CABLE_BTYPE:u32=1; pub const CABLE_CTYPE:u32=2; pub const CABLE_CAPTIVE:u32=3;
pub const CABLE_LATENCY_1M:u32=1; pub const CABLE_LATENCY_2M:u32=2; pub const CABLE_LATENCY_3M:u32=3; pub const CABLE_LATENCY_4M:u32=4; pub const CABLE_LATENCY_5M:u32=5; pub const CABLE_LATENCY_6M:u32=6; pub const CABLE_LATENCY_7M:u32=7; pub const CABLE_LATENCY_7M_PLUS:u32=8;
pub const PCABLE_VCONN_NOT_REQ:u32=0; pub const PCABLE_VCONN_REQ:u32=1; pub const ACABLE_ONE_END:u32=2; pub const ACABLE_BOTH_END:u32=3;
pub const CABLE_MAX_VBUS_20V:u32=0; pub const CABLE_MAX_VBUS_30V:u32=1; pub const CABLE_MAX_VBUS_40V:u32=2; pub const CABLE_MAX_VBUS_50V:u32=3;
pub const ACABLE_SBU_SUPP:u32=0; pub const ACABLE_SBU_NOT_SUPP:u32=1; pub const ACABLE_SBU_PASSIVE:u32=0; pub const ACABLE_SBU_ACTIVE:u32=1; pub const CABLE_CURR_DEF:u32=0; pub const CABLE_CURR_3A:u32=1; pub const CABLE_CURR_5A:u32=2;
pub const CABLE_USBSS_U2_ONLY:u32=0; pub const CABLE_USBSS_U31_GEN1:u32=1; pub const CABLE_USBSS_U31_GEN2:u32=2; pub const CABLE_USB2_ONLY:u32=0; pub const CABLE_USB32_GEN1:u32=1; pub const CABLE_USB32_4_GEN2:u32=2; pub const CABLE_USB4_GEN3:u32=3;
#[inline] pub const fn VDO_CABLE(hw:u32,fw:u32,cbl:u32,lat:u32,term:u32,tx1d:u32,tx2d:u32,rx1d:u32,rx2d:u32,cur:u32,vps:u32,sopp:u32,usbss:u32)->u32 { (hw&7)<<28 | (fw&7)<<24 | (cbl&3)<<18 | (lat&7)<<13 | (term&3)<<11 | tx1d<<10 | tx2d<<9 | rx1d<<8 | rx2d<<7 | (cur&3)<<5 | vps<<4 | sopp<<3 | (usbss&7) }
#[inline] pub const fn VDO_PCABLE(hw:u32,fw:u32,ver:u32,conn:u32,lat:u32,term:u32,vbm:u32,cur:u32,spd:u32)->u32 { (hw&15)<<28 | (fw&15)<<24 | (ver&7)<<21 | (conn&3)<<18 | (lat&15)<<13 | (term&3)<<11 | (vbm&3)<<9 | (cur&3)<<5 | (spd&7) }
#[inline] pub const fn VDO_ACABLE1(hw:u32,fw:u32,ver:u32,conn:u32,lat:u32,term:u32,vbm:u32,sbu:u32,sbut:u32,cur:u32,vbt:u32,sopp:u32,spd:u32)->u32 { (hw&15)<<28 | (fw&15)<<24 | (ver&7)<<21 | (conn&3)<<18 | (lat&15)<<13 | (term&3)<<11 | (vbm&3)<<9 | sbu<<8 | sbut<<7 | (cur&3)<<5 | vbt<<4 | sopp<<3 | (spd&7) }

pub const ACAB2_U3_CLD_10MW_PLUS:u32=0; pub const ACAB2_U3_CLD_10MW:u32=1; pub const ACAB2_U3_CLD_5MW:u32=2; pub const ACAB2_U3_CLD_1MW:u32=3; pub const ACAB2_U3_CLD_500UW:u32=4; pub const ACAB2_U3_CLD_200UW:u32=5; pub const ACAB2_U3_CLD_50UW:u32=6;
pub const ACAB2_U3U0_DIRECT:u32=0; pub const ACAB2_U3U0_U3S:u32=1; pub const ACAB2_PHY_COPPER:u32=0; pub const ACAB2_PHY_OPTICAL:u32=1; pub const ACAB2_REDRIVER:u32=0; pub const ACAB2_RETIMER:u32=1; pub const ACAB2_USB4_SUPP:u32=0; pub const ACAB2_USB4_NOT_SUPP:u32=1; pub const ACAB2_USB2_SUPP:u32=0; pub const ACAB2_USB2_NOT_SUPP:u32=1; pub const ACAB2_USB32_SUPP:u32=0; pub const ACAB2_USB32_NOT_SUPP:u32=1; pub const ACAB2_LANES_ONE:u32=0; pub const ACAB2_LANES_TWO:u32=1; pub const ACAB2_OPT_ISO_NO:u32=0; pub const ACAB2_OPT_ISO_YES:u32=1; pub const ACAB2_GEN_1:u32=0; pub const ACAB2_GEN_2_PLUS:u32=1;
#[inline] pub const fn VDO_ACABLE2(mtemp:u32,stemp:u32,u3p:u32,trans:u32,phy:u32,ele:u32,u4:u32,hops:u32,u2:u32,u32v:u32,lane:u32,iso:u32,gen:u32)->u32 { (mtemp&0xff)<<24 | (stemp&0xff)<<16 | (u3p&7)<<12 | trans<<11 | phy<<10 | ele<<9 | u4<<8 | (hops&3)<<6 | u2<<5 | u32v<<4 | lane<<3 | iso<<2 | gen }
#[inline] pub const fn VDO_AMA(hw:u32,fw:u32,tx1d:u32,tx2d:u32,rx1d:u32,rx2d:u32,vcpwr:u32,vcr:u32,vbr:u32,usbss:u32)->u32 { (hw&7)<<28 | (fw&7)<<24 | tx1d<<11 | tx2d<<10 | rx1d<<9 | rx2d<<8 | (vcpwr&7)<<5 | vcr<<4 | vbr<<3 | (usbss&7) }
#[inline] pub const fn PD_VDO_AMA_VCONN_REQ(vdo:u32)->u32 { (vdo>>4)&1 }
#[inline] pub const fn PD_VDO_AMA_VBUS_REQ(vdo:u32)->u32 { (vdo>>3)&1 }
pub const AMA_USBSS_U2_ONLY:u32=0; pub const AMA_USBSS_U31_GEN1:u32=1; pub const AMA_USBSS_U31_GEN2:u32=2; pub const AMA_USBSS_BBONLY:u32=3;
pub const VPD_VDO_VER1_0:u32=0; pub const VPD_MAX_VBUS_20V:u32=0; pub const VPD_MAX_VBUS_30V:u32=1; pub const VPD_MAX_VBUS_40V:u32=2; pub const VPD_MAX_VBUS_50V:u32=3; pub const VPDCT_CURR_3A:u32=0; pub const VPDCT_CURR_5A:u32=1; pub const VPDCT_NOT_SUPP:u32=0; pub const VPDCT_SUPP:u32=1;
#[inline] pub const fn VDO_VPD(hw:u32,fw:u32,ver:u32,vbm:u32,curr:u32,vbi:u32,gi:u32,ct:u32)->u32 { (hw&15)<<28 | (fw&15)<<24 | (ver&7)<<21 | (vbm&3)<<15 | curr<<14 | (vbi&0x3f)<<7 | (gi&0x3f)<<1 | ct }
#[inline] pub const fn SINK_LOAD_CHAR(vdroop:u32,duty_cycle:u32,period:u32,percent_ol:u32)->u32 { (vdroop&1)<<15 | (duty_cycle&15)<<11 | (period&0x3f)<<5 | (percent_ol&0x1f) }
pub const COMPLIANCE_LPS:u32=1<<0; pub const COMPLIANCE_PS1:u32=1<<1; pub const COMPLIANCE_PS2:u32=1<<2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
