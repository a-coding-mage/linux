/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright 2015-2017 Google, Inc */

// Translated from linux/usb/pd_vdo.h. Definitions from pd.h and bitfield.h
// remain external dependencies of this header translation.

pub const VDO_MAX_OBJECTS: u32 = 6;
pub const VDO_MAX_SIZE: u32 = VDO_MAX_OBJECTS + 1;

macro_rules! VDO { ($vid:expr, $ty:expr, $ver:expr, $custom:expr) => {
    (($vid << 16) | ($ty << 15) | ($ver << 13) | ($custom & 0x7fff))
} }
macro_rules! VDO_SVDM_TYPE { () => { 1 << 15 } }
macro_rules! VDO_SVDM_VERS { ($x:expr) => { $x << 13 } }
macro_rules! VDO_OPOS { ($x:expr) => { $x << 8 } }
macro_rules! VDO_CMDT { ($x:expr) => { $x << 6 } }
pub const VDO_SVDM_VERS_MASK: u32 = VDO_SVDM_VERS!(0x3);
pub const VDO_OPOS_MASK: u32 = VDO_OPOS!(0x7);
pub const VDO_CMDT_MASK: u32 = VDO_CMDT!(0x3);

pub const CMDT_INIT: u32 = 0;
pub const CMDT_RSP_ACK: u32 = 1;
pub const CMDT_RSP_NAK: u32 = 2;
pub const CMDT_RSP_BUSY: u32 = 3;
pub const VDO_SRC_INITIATOR: u32 = 0 << 5;
pub const VDO_SRC_RESPONDER: u32 = 1 << 5;
pub const CMD_DISCOVER_IDENT: u32 = 1;
pub const CMD_DISCOVER_SVID: u32 = 2;
pub const CMD_DISCOVER_MODES: u32 = 3;
pub const CMD_ENTER_MODE: u32 = 4;
pub const CMD_EXIT_MODE: u32 = 5;
pub const CMD_ATTENTION: u32 = 6;
macro_rules! VDO_CMD_VENDOR { ($x:expr) => { (0x10 + $x) & 0x1f } }
pub const VDO_CMD_VERSION: u32 = VDO_CMD_VENDOR!(0);
pub const VDO_CMD_SEND_INFO: u32 = VDO_CMD_VENDOR!(1);
pub const VDO_CMD_READ_INFO: u32 = VDO_CMD_VENDOR!(2);
pub const VDO_CMD_REBOOT: u32 = VDO_CMD_VENDOR!(5);
pub const VDO_CMD_FLASH_ERASE: u32 = VDO_CMD_VENDOR!(6);
pub const VDO_CMD_FLASH_WRITE: u32 = VDO_CMD_VENDOR!(7);
pub const VDO_CMD_ERASE_SIG: u32 = VDO_CMD_VENDOR!(8);
pub const VDO_CMD_PING_ENABLE: u32 = VDO_CMD_VENDOR!(10);
pub const VDO_CMD_CURRENT: u32 = VDO_CMD_VENDOR!(11);
pub const VDO_CMD_FLIP: u32 = VDO_CMD_VENDOR!(12);
pub const VDO_CMD_GET_LOG: u32 = VDO_CMD_VENDOR!(13);
pub const VDO_CMD_CCD_EN: u32 = VDO_CMD_VENDOR!(14);

macro_rules! PD_VDO_VID { ($v:expr) => { $v >> 16 } }
macro_rules! PD_VDO_SVDM { ($v:expr) => { ($v >> 15) & 1 } }
macro_rules! PD_VDO_SVDM_VER { ($v:expr) => { ($v >> 13) & 0x3 } }
macro_rules! PD_VDO_OPOS { ($v:expr) => { ($v >> 8) & 0x7 } }
macro_rules! PD_VDO_CMD { ($v:expr) => { $v & 0x1f } }
macro_rules! PD_VDO_CMDT { ($v:expr) => { ($v >> 6) & 0x3 } }

pub const VDO_INDEX_HDR: usize = 0;
pub const VDO_INDEX_IDH: usize = 1;
pub const VDO_INDEX_CSTAT: usize = 2;
pub const VDO_INDEX_CABLE: usize = 3;
pub const VDO_INDEX_PRODUCT: usize = 3;
pub const VDO_INDEX_AMA: usize = 4;
pub const VDO_INDEX_CABLE_1: usize = 4;
pub const VDO_INDEX_CABLE_2: usize = 5;

pub const IDH_PTYPE_UNDEF: u32 = 0;
pub const IDH_PTYPE_NOT_UFP: u32 = 0;
pub const IDH_PTYPE_HUB: u32 = 1;
pub const IDH_PTYPE_PERIPH: u32 = 2;
pub const IDH_PTYPE_PSD: u32 = 3;
pub const IDH_PTYPE_AMA: u32 = 5;
pub const IDH_PTYPE_NOT_CABLE: u32 = 0;
pub const IDH_PTYPE_PCABLE: u32 = 3;
pub const IDH_PTYPE_ACABLE: u32 = 4;
pub const IDH_PTYPE_VPD: u32 = 6;
pub const IDH_PTYPE_NOT_DFP: u32 = 0;
pub const IDH_PTYPE_DFP_HUB: u32 = 1;
pub const IDH_PTYPE_DFP_HOST: u32 = 2;
pub const IDH_PTYPE_DFP_PB: u32 = 3;
macro_rules! GENMASK { ($h:expr, $l:expr) => { (((1u32 << ($h - $l + 1)) - 1) << $l) } }
macro_rules! FIELD_GET { ($mask:expr, $v:expr) => { (($v & $mask) >> $mask.trailing_zeros()) } }
macro_rules! BIT { ($n:expr) => { 1u32 << $n } }
pub const IDH_DFP_MASK: u32 = GENMASK!(25, 23);
pub const IDH_CONN_MASK: u32 = GENMASK!(22, 21);
macro_rules! VDO_IDH { ($usbh:expr,$usbd:expr,$ufp:expr,$modal:expr,$dfp:expr,$conn:expr,$vid:expr) => {
    ($usbh << 31 | $usbd << 30 | ($ufp & 0x7) << 27 | $modal << 26 | ($dfp & 0x7) << 23 | ($conn & 0x3) << 21 | ($vid & 0xffff))
} }
macro_rules! PD_IDH_PTYPE { ($v:expr) => { ($v >> 27) & 0x7 } }
macro_rules! PD_IDH_VID { ($v:expr) => { $v & 0xffff } }
macro_rules! PD_IDH_MODAL_SUPP { ($v:expr) => { $v & (1 << 26) } }
macro_rules! PD_IDH_DFP_PTYPE { ($v:expr) => { ($v >> 23) & 0x7 } }
macro_rules! PD_IDH_CONN_TYPE { ($v:expr) => { ($v >> 21) & 0x3 } }
macro_rules! PD_IDH_HOST_SUPP { ($v:expr) => { $v & (1 << 31) } }
macro_rules! PD_CSTAT_XID { ($v:expr) => { $v } }
macro_rules! VDO_CERT { ($x:expr) => { $x & 0xffffffff } }
macro_rules! VDO_PRODUCT { ($pid:expr,$bcd:expr) => { (($pid & 0xffff) << 16 | ($bcd & 0xffff)) } }
macro_rules! PD_PRODUCT_PID { ($v:expr) => { ($v >> 16) & 0xffff } }

pub const UFP_VDO_VER1_2: u32 = 2;
pub const DEV_USB2_CAPABLE: u32 = BIT!(0);
pub const DEV_USB2_BILLBOARD: u32 = BIT!(1);
pub const DEV_USB3_CAPABLE: u32 = BIT!(2);
pub const DEV_USB4_CAPABLE: u32 = BIT!(3);
pub const UFP_RECEPTACLE: u32 = 2;
pub const UFP_CAPTIVE: u32 = 3;
pub const AMA_VCONN_PWR_1W: u32 = 0;
pub const AMA_VCONN_PWR_1W5: u32 = 1;
pub const AMA_VCONN_PWR_2W: u32 = 2;
pub const AMA_VCONN_PWR_3W: u32 = 3;
pub const AMA_VCONN_PWR_4W: u32 = 4;
pub const AMA_VCONN_PWR_5W: u32 = 5;
pub const AMA_VCONN_PWR_6W: u32 = 6;
pub const AMA_VCONN_NOT_REQ: u32 = 0;
pub const AMA_VCONN_REQ: u32 = 1;
pub const AMA_VBUS_REQ: u32 = 0;
pub const AMA_VBUS_NOT_REQ: u32 = 1;
pub const UFP_ALTMODE_NOT_SUPP: u32 = 0;
pub const UFP_ALTMODE_TBT3: u32 = BIT!(0);
pub const UFP_ALTMODE_RECFG: u32 = BIT!(1);
pub const UFP_ALTMODE_NO_RECFG: u32 = BIT!(2);
pub const UFP_USB2_ONLY: u32 = 0;
pub const UFP_USB32_GEN1: u32 = 1;
pub const UFP_USB32_4_GEN2: u32 = 2;
pub const UFP_USB4_GEN3: u32 = 3;
macro_rules! PD_VDO_UFP_DEVCAP { ($v:expr) => { FIELD_GET!(GENMASK!(27,24), $v) } }
macro_rules! VDO_UFP { ($ver:expr,$cap:expr,$conn:expr,$pwr:expr,$vcr:expr,$vbr:expr,$alt:expr,$spd:expr) => { (($ver & 0x7)<<29 | ($cap & 0xf)<<24 | ($conn & 0x3)<<22 | ($pwr & 0x7)<<8 | $vcr<<7 | $vbr<<6 | ($alt & 0x7)<<3 | ($spd & 0x7)) } }

pub const DFP_VDO_VER1_1: u32 = 1;
pub const HOST_USB2_CAPABLE: u32 = BIT!(0);
pub const HOST_USB3_CAPABLE: u32 = BIT!(1);
pub const HOST_USB4_CAPABLE: u32 = BIT!(2);
pub const DFP_RECEPTACLE: u32 = 2;
pub const DFP_CAPTIVE: u32 = 3;
macro_rules! PD_VDO_DFP_HOSTCAP { ($v:expr) => { FIELD_GET!(GENMASK!(26,24), $v) } }
macro_rules! VDO_DFP { ($ver:expr,$cap:expr,$conn:expr,$pnum:expr) => { (($ver&0x7)<<29 | ($cap&0x7)<<24 | ($conn&0x3)<<22 | ($pnum&0x1f)) } }

pub const CABLE_VDO_VER1_0: u32 = 0;
pub const CABLE_VDO_VER1_3: u32 = 3;
pub const CABLE_ATYPE: u32 = 0;
pub const CABLE_BTYPE: u32 = 1;
pub const CABLE_CTYPE: u32 = 2;
pub const CABLE_CAPTIVE: u32 = 3;
pub const CABLE_LATENCY_1M: u32 = 1;
pub const CABLE_LATENCY_2M: u32 = 2;
pub const CABLE_LATENCY_3M: u32 = 3;
pub const CABLE_LATENCY_4M: u32 = 4;
pub const CABLE_LATENCY_5M: u32 = 5;
pub const CABLE_LATENCY_6M: u32 = 6;
pub const CABLE_LATENCY_7M: u32 = 7;
pub const CABLE_LATENCY_7M_PLUS: u32 = 8;
pub const PCABLE_VCONN_NOT_REQ: u32 = 0;
pub const PCABLE_VCONN_REQ: u32 = 1;
pub const ACABLE_ONE_END: u32 = 2;
pub const ACABLE_BOTH_END: u32 = 3;
pub const CABLE_MAX_VBUS_20V: u32 = 0;
pub const CABLE_MAX_VBUS_30V: u32 = 1;
pub const CABLE_MAX_VBUS_40V: u32 = 2;
pub const CABLE_MAX_VBUS_50V: u32 = 3;
pub const ACABLE_SBU_SUPP: u32 = 0;
pub const ACABLE_SBU_NOT_SUPP: u32 = 1;
pub const ACABLE_SBU_PASSIVE: u32 = 0;
pub const ACABLE_SBU_ACTIVE: u32 = 1;
pub const CABLE_CURR_DEF: u32 = 0;
pub const CABLE_CURR_3A: u32 = 1;
pub const CABLE_CURR_5A: u32 = 2;
pub const CABLE_USBSS_U2_ONLY: u32 = 0;
pub const CABLE_USBSS_U31_GEN1: u32 = 1;
pub const CABLE_USBSS_U31_GEN2: u32 = 2;
pub const CABLE_USB2_ONLY: u32 = 0;
pub const CABLE_USB32_GEN1: u32 = 1;
pub const CABLE_USB32_4_GEN2: u32 = 2;
pub const CABLE_USB4_GEN3: u32 = 3;
macro_rules! VDO_CABLE { ($hw:expr,$fw:expr,$cbl:expr,$lat:expr,$term:expr,$tx1:expr,$tx2:expr,$rx1:expr,$rx2:expr,$cur:expr,$vps:expr,$sopp:expr,$usb:expr) => { (($hw&0x7)<<28|($fw&0x7)<<24|($cbl&0x3)<<18|($lat&0x7)<<13|($term&0x3)<<11|$tx1<<10|$tx2<<9|$rx1<<8|$rx2<<7|($cur&0x3)<<5|$vps<<4|$sopp<<3|($usb&0x7)) } }
macro_rules! VDO_PCABLE { ($hw:expr,$fw:expr,$ver:expr,$conn:expr,$lat:expr,$term:expr,$vbm:expr,$cur:expr,$spd:expr) => { (($hw&0xf)<<28|($fw&0xf)<<24|($ver&0x7)<<21|($conn&0x3)<<18|($lat&0xf)<<13|($term&0x3)<<11|($vbm&0x3)<<9|($cur&0x3)<<5|($spd&0x7)) } }
macro_rules! VDO_ACABLE1 { ($hw:expr,$fw:expr,$ver:expr,$conn:expr,$lat:expr,$term:expr,$vbm:expr,$sbu:expr,$sbut:expr,$cur:expr,$vbt:expr,$sopp:expr,$spd:expr) => { (($hw&0xf)<<28|($fw&0xf)<<24|($ver&0x7)<<21|($conn&0x3)<<18|($lat&0xf)<<13|($term&0x3)<<11|($vbm&0x3)<<9|$sbu<<8|$sbut<<7|($cur&0x3)<<5|$vbt<<4|$sopp<<3|($spd&0x7)) } }
macro_rules! VDO_TYPEC_CABLE_SPEED { ($v:expr) => { $v & 0x7 } }
macro_rules! VDO_TYPEC_CABLE_TYPE { ($v:expr) => { ($v >> 18) & 0x3 } }

pub const ACAB2_U3_CLD_10MW_PLUS: u32=0; pub const ACAB2_U3_CLD_10MW:u32=1; pub const ACAB2_U3_CLD_5MW:u32=2; pub const ACAB2_U3_CLD_1MW:u32=3; pub const ACAB2_U3_CLD_500UW:u32=4; pub const ACAB2_U3_CLD_200UW:u32=5; pub const ACAB2_U3_CLD_50UW:u32=6;
pub const ACAB2_U3U0_DIRECT:u32=0; pub const ACAB2_U3U0_U3S:u32=1; pub const ACAB2_PHY_COPPER:u32=0; pub const ACAB2_PHY_OPTICAL:u32=1; pub const ACAB2_REDRIVER:u32=0; pub const ACAB2_RETIMER:u32=1; pub const ACAB2_USB4_SUPP:u32=0; pub const ACAB2_USB4_NOT_SUPP:u32=1; pub const ACAB2_USB2_SUPP:u32=0; pub const ACAB2_USB2_NOT_SUPP:u32=1; pub const ACAB2_USB32_SUPP:u32=0; pub const ACAB2_USB32_NOT_SUPP:u32=1; pub const ACAB2_LANES_ONE:u32=0; pub const ACAB2_LANES_TWO:u32=1; pub const ACAB2_OPT_ISO_NO:u32=0; pub const ACAB2_OPT_ISO_YES:u32=1; pub const ACAB2_GEN_1:u32=0; pub const ACAB2_GEN_2_PLUS:u32=1;
macro_rules! VDO_ACABLE2 { ($m:expr,$s:expr,$u3:expr,$tr:expr,$phy:expr,$el:expr,$u4:expr,$h:expr,$u2:expr,$u32:expr,$lane:expr,$iso:expr,$gen:expr) => { (($m&0xff)<<24|($s&0xff)<<16|($u3&0x7)<<12|$tr<<11|$phy<<10|$el<<9|$u4<<8|($h&0x3)<<6|$u2<<5|$u32<<4|$lane<<3|$iso<<2|$gen) } }

macro_rules! VDO_AMA { ($hw:expr,$fw:expr,$tx1:expr,$tx2:expr,$rx1:expr,$rx2:expr,$pwr:expr,$vcr:expr,$vbr:expr,$usb:expr) => { (($hw&0x7)<<28|($fw&0x7)<<24|$tx1<<11|$tx2<<10|$rx1<<9|$rx2<<8|($pwr&0x7)<<5|$vcr<<4|$vbr<<3|($usb&0x7)) } }
macro_rules! PD_VDO_AMA_VCONN_REQ { ($v:expr) => { ($v >> 4) & 1 } }
macro_rules! PD_VDO_AMA_VBUS_REQ { ($v:expr) => { ($v >> 3) & 1 } }
pub const AMA_USBSS_U2_ONLY:u32=0; pub const AMA_USBSS_U31_GEN1:u32=1; pub const AMA_USBSS_U31_GEN2:u32=2; pub const AMA_USBSS_BBONLY:u32=3;
pub const VPD_VDO_VER1_0:u32=0; pub const VPD_MAX_VBUS_20V:u32=0; pub const VPD_MAX_VBUS_30V:u32=1; pub const VPD_MAX_VBUS_40V:u32=2; pub const VPD_MAX_VBUS_50V:u32=3; pub const VPDCT_CURR_3A:u32=0; pub const VPDCT_CURR_5A:u32=1; pub const VPDCT_NOT_SUPP:u32=0; pub const VPDCT_SUPP:u32=1;
macro_rules! VDO_VPD { ($hw:expr,$fw:expr,$ver:expr,$vbm:expr,$curr:expr,$vbi:expr,$gi:expr,$ct:expr) => { (($hw&0xf)<<28|($fw&0xf)<<24|($ver&0x7)<<21|($vbm&0x3)<<15|$curr<<14|($vbi&0x3f)<<7|($gi&0x3f)<<1|$ct) } }
macro_rules! VDO_SVID { ($a:expr,$b:expr) => { (($a&0xffff)<<16|($b&0xffff)) } }
macro_rules! PD_VDO_SVID_SVID0 { ($v:expr) => { $v >> 16 } }
macro_rules! PD_VDO_SVID_SVID1 { ($v:expr) => { $v & 0xffff } }
pub const USB_SID_PD:u32=0xff00; pub const USB_SID_DISPLAYPORT:u32=0xff01; pub const USB_SID_MHL:u32=0xff02;
pub const PD_T_VDM_UNSTRUCTURED:u32=500; pub const PD_T_VDM_BUSY:u32=100; pub const PD_T_VDM_WAIT_MODE_E:u32=100; pub const PD_T_VDM_SNDR_RSP:u32=30; pub const PD_T_VDM_E_MODE:u32=25; pub const PD_T_VDM_RCVR_RSP:u32=15;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
