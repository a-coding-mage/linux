/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SB1250 LDT constants.  The C header included sb1250_defs.h; its
 * _SB_MAKEMASK/_SB_MAKEVALUE/_SB_GETVALUE helpers are represented below. */

pub const K_LDT_VENDOR_SIBYTE: u32 = 0x166d;
pub const K_LDT_DEVICE_SB1250: u32 = 0x0002;

macro_rules! mask1 { ($b:expr) => { 1u32 << $b }; }
macro_rules! mask { ($n:expr, $s:expr) => { (((1u32 << $n) - 1) << $s) }; }
macro_rules! value { ($x:expr, $s:expr) => { (($x as u32) << $s) }; }
macro_rules! get { ($x:expr, $s:expr, $m:expr) => { (($x as u32 & $m) >> $s) }; }

pub const R_LDT_TYPE1_DEVICEID:u32=0x0000; pub const R_LDT_TYPE1_CMDSTATUS:u32=0x0004;
pub const R_LDT_TYPE1_CLASSREV:u32=0x0008; pub const R_LDT_TYPE1_DEVHDR:u32=0x000c;
pub const R_LDT_TYPE1_BAR0:u32=0x0010; pub const R_LDT_TYPE1_BAR1:u32=0x0014;
pub const R_LDT_TYPE1_BUSID:u32=0x0018; pub const R_LDT_TYPE1_SECSTATUS:u32=0x001c;
pub const R_LDT_TYPE1_MEMLIMIT:u32=0x0020; pub const R_LDT_TYPE1_PREFETCH:u32=0x0024;
pub const R_LDT_TYPE1_PREF_BASE:u32=0x0028; pub const R_LDT_TYPE1_PREF_LIMIT:u32=0x002c;
pub const R_LDT_TYPE1_IOLIMIT:u32=0x0030; pub const R_LDT_TYPE1_CAPPTR:u32=0x0034;
pub const R_LDT_TYPE1_ROMADDR:u32=0x0038; pub const R_LDT_TYPE1_BRCTL:u32=0x003c;
pub const R_LDT_TYPE1_CMD:u32=0x0040; pub const R_LDT_TYPE1_LINKCTRL:u32=0x0044;
pub const R_LDT_TYPE1_LINKFREQ:u32=0x0048; pub const R_LDT_TYPE1_RESERVED1:u32=0x004c;
pub const R_LDT_TYPE1_SRICMD:u32=0x0050; pub const R_LDT_TYPE1_SRITXNUM:u32=0x0054;
pub const R_LDT_TYPE1_SRIRXNUM:u32=0x0058; pub const R_LDT_TYPE1_ERRSTATUS:u32=0x0068;
pub const R_LDT_TYPE1_SRICTRL:u32=0x006c; pub const R_LDT_TYPE1_ADDSTATUS:u32=0x0070;
pub const R_LDT_TYPE1_TXBUFCNT:u32=0x00c8; pub const R_LDT_TYPE1_EXPCRC:u32=0x00dc;
pub const R_LDT_TYPE1_RXCRC:u32=0x00f0;

pub const S_LDT_DEVICEID_VENDOR:u32=0; pub const M_LDT_DEVICEID_VENDOR:u32=mask!(16,0);
pub const S_LDT_DEVICEID_DEVICEID:u32=16; pub const M_LDT_DEVICEID_DEVICEID:u32=mask!(16,16);
pub const S_LDT_CLASSREV_REV:u32=0; pub const M_LDT_CLASSREV_REV:u32=mask!(8,0);
pub const S_LDT_CLASSREV_CLASS:u32=8; pub const M_LDT_CLASSREV_CLASS:u32=mask!(24,8);
pub const K_LDT_REV:u32=1; pub const K_LDT_CLASS:u32=0x060000;
pub const S_LDT_DEVHDR_CLINESZ:u32=0; pub const M_LDT_DEVHDR_CLINESZ:u32=mask!(8,0);
pub const S_LDT_DEVHDR_LATTMR:u32=8; pub const M_LDT_DEVHDR_LATTMR:u32=mask!(8,8);
pub const S_LDT_DEVHDR_HDRTYPE:u32=16; pub const M_LDT_DEVHDR_HDRTYPE:u32=mask!(8,16);
pub const K_LDT_DEVHDR_HDRTYPE_TYPE1:u32=1; pub const S_LDT_DEVHDR_BIST:u32=24;
pub const M_LDT_DEVHDR_BIST:u32=mask!(8,24);

pub const M_LDT_CMD_IOSPACE_EN:u32=mask1!(0); pub const M_LDT_CMD_MEMSPACE_EN:u32=mask1!(1);
pub const M_LDT_CMD_MASTER_EN:u32=mask1!(2); pub const M_LDT_CMD_SPECCYC_EN:u32=mask1!(3);
pub const M_LDT_CMD_MEMWRINV_EN:u32=mask1!(4); pub const M_LDT_CMD_VGAPALSNP_EN:u32=mask1!(5);
pub const M_LDT_CMD_PARERRRESP:u32=mask1!(6); pub const M_LDT_CMD_WAITCYCCTRL:u32=mask1!(7);
pub const M_LDT_CMD_SERR_EN:u32=mask1!(8); pub const M_LDT_CMD_FASTB2B_EN:u32=mask1!(9);

pub const M_LDT_STATUS_CAPLIST:u32=mask1!(20); pub const M_LDT_STATUS_66MHZCAP:u32=mask1!(21);
pub const M_LDT_STATUS_RESERVED2:u32=mask1!(22); pub const M_LDT_STATUS_FASTB2BCAP:u32=mask1!(23);
pub const M_LDT_STATUS_MSTRDPARERR:u32=mask1!(24); pub const S_LDT_STATUS_DEVSELTIMING:u32=25;
pub const M_LDT_STATUS_DEVSELTIMING:u32=mask!(2,25); pub const M_LDT_STATUS_SIGDTGTABORT:u32=mask1!(27);
pub const M_LDT_STATUS_RCVDTGTABORT:u32=mask1!(28); pub const M_LDT_STATUS_RCVDMSTRABORT:u32=mask1!(29);
pub const M_LDT_STATUS_SIGDSERR:u32=mask1!(30); pub const M_LDT_STATUS_DETPARERR:u32=mask1!(31);

pub const M_LDT_BRCTL_PARERRRESP_EN:u32=mask1!(16); pub const M_LDT_BRCTL_SERR_EN:u32=mask1!(17);
pub const M_LDT_BRCTL_ISA_EN:u32=mask1!(18); pub const M_LDT_BRCTL_VGA_EN:u32=mask1!(19);
pub const M_LDT_BRCTL_MSTRABORTMODE:u32=mask1!(21); pub const M_LDT_BRCTL_SECBUSRESET:u32=mask1!(22);
pub const M_LDT_BRCTL_FASTB2B_EN:u32=mask1!(23); pub const M_LDT_BRCTL_PRIDISCARD:u32=mask1!(24);
pub const M_LDT_BRCTL_SECDISCARD:u32=mask1!(25); pub const M_LDT_BRCTL_DISCARDSTAT:u32=mask1!(26);
pub const M_LDT_BRCTL_DISCARDSERR_EN:u32=mask1!(27);

pub const M_LDT_CMD_WARMRESET:u32=mask1!(16); pub const M_LDT_CMD_DOUBLEENDED:u32=mask1!(17);
pub const S_LDT_CMD_CAPTYPE:u32=29; pub const M_LDT_CMD_CAPTYPE:u32=mask!(3,29);

pub const M_LDT_LINKCTRL_CAPSYNCFLOOD_EN:u32=mask1!(1); pub const M_LDT_LINKCTRL_CRCSTARTTEST:u32=mask1!(2);
pub const M_LDT_LINKCTRL_CRCFORCEERR:u32=mask1!(3); pub const M_LDT_LINKCTRL_LINKFAIL:u32=mask1!(4);
pub const M_LDT_LINKCTRL_INITDONE:u32=mask1!(5); pub const M_LDT_LINKCTRL_EOC:u32=mask1!(6);
pub const M_LDT_LINKCTRL_XMITOFF:u32=mask1!(7); pub const S_LDT_LINKCTRL_CRCERR:u32=8;
pub const M_LDT_LINKCTRL_CRCERR:u32=mask!(4,8); pub const S_LDT_LINKCTRL_MAXIN:u32=16;
pub const M_LDT_LINKCTRL_MAXIN:u32=mask!(3,16); pub const M_LDT_LINKCTRL_DWFCLN:u32=mask1!(19);
pub const S_LDT_LINKCTRL_MAXOUT:u32=20; pub const M_LDT_LINKCTRL_MAXOUT:u32=mask!(3,20);
pub const M_LDT_LINKCTRL_DWFCOUT:u32=mask1!(23); pub const S_LDT_LINKCTRL_WIDTHIN:u32=24;
pub const M_LDT_LINKCTRL_WIDTHIN:u32=mask!(3,24); pub const M_LDT_LINKCTRL_DWFCLIN_EN:u32=mask1!(27);
pub const S_LDT_LINKCTRL_WIDTHOUT:u32=28; pub const M_LDT_LINKCTRL_WIDTHOUT:u32=mask!(3,28);
pub const M_LDT_LINKCTRL_DWFCOUT_EN:u32=mask1!(31);

pub const S_LDT_LINKFREQ_FREQ:u32=8; pub const M_LDT_LINKFREQ_FREQ:u32=mask!(4,8);
pub const K_LDT_LINKFREQ_200MHZ:u32=0; pub const K_LDT_LINKFREQ_300MHZ:u32=1;
pub const K_LDT_LINKFREQ_400MHZ:u32=2; pub const K_LDT_LINKFREQ_500MHZ:u32=3;
pub const K_LDT_LINKFREQ_600MHZ:u32=4; pub const K_LDT_LINKFREQ_800MHZ:u32=5;
pub const K_LDT_LINKFREQ_1000MHZ:u32=6;

pub const M_LDT_SRICMD_SIPREADY:u32=mask1!(16); pub const M_LDT_SRICMD_SYNCPTRCTL:u32=mask1!(17);
pub const M_LDT_SRICMD_REDUCESYNCZERO:u32=mask1!(18); pub const M_LDT_SRICMD_DISSTARVATIONCNT:u32=mask1!(19);
pub const M_LDT_SRICMD_DISMULTTXVLD:u32=mask1!(19); pub const M_LDT_SRICMD_EXPENDIAN:u32=mask1!(26);
pub const S_LDT_SRICMD_RXMARGIN:u32=20; pub const M_LDT_SRICMD_RXMARGIN:u32=mask!(5,20);
pub const M_LDT_SRICMD_LDTPLLCOMPAT:u32=mask1!(25); pub const S_LDT_SRICMD_TXINITIALOFFSET:u32=28;
pub const M_LDT_SRICMD_TXINITIALOFFSET:u32=mask!(3,28); pub const M_LDT_SRICMD_LINKFREQDIRECT:u32=mask1!(31);

pub const M_LDT_ERRCTL_PROTFATAL_EN:u32=mask1!(0); pub const M_LDT_ERRCTL_PROTNONFATAL_EN:u32=mask1!(1);
pub const M_LDT_ERRCTL_PROTSYNCFLOOD_EN:u32=mask1!(2); pub const M_LDT_ERRCTL_OVFFATAL_EN:u32=mask1!(3);
pub const M_LDT_ERRCTL_OVFNONFATAL_EN:u32=mask1!(4); pub const M_LDT_ERRCTL_OVFSYNCFLOOD_EN:u32=mask1!(5);
pub const M_LDT_ERRCTL_EOCNXAFATAL_EN:u32=mask1!(6); pub const M_LDT_ERRCTL_EOCNXANONFATAL_EN:u32=mask1!(7);
pub const M_LDT_ERRCTL_EOCNXASYNCFLOOD_EN:u32=mask1!(8); pub const M_LDT_ERRCTL_CRCFATAL_EN:u32=mask1!(9);
pub const M_LDT_ERRCTL_CRCNONFATAL_EN:u32=mask1!(10); pub const M_LDT_ERRCTL_SERRFATAL_EN:u32=mask1!(11);
pub const M_LDT_ERRCTL_SRCTAGFATAL_EN:u32=mask1!(12); pub const M_LDT_ERRCTL_SRCTAGNONFATAL_EN:u32=mask1!(13);
pub const M_LDT_ERRCTL_SRCTAGSYNCFLOOD_EN:u32=mask1!(14); pub const M_LDT_ERRCTL_MAPNXAFATAL_EN:u32=mask1!(15);
pub const M_LDT_ERRCTL_MAPNXANONFATAL_EN:u32=mask1!(16); pub const M_LDT_ERRCTL_MAPNXASYNCFLOOD_EN:u32=mask1!(17);
pub const M_LDT_ERRCTL_PROTOERR:u32=mask1!(24); pub const M_LDT_ERRCTL_OVFERR:u32=mask1!(25);
pub const M_LDT_ERRCTL_EOCNXAERR:u32=mask1!(26); pub const M_LDT_ERRCTL_SRCTAGERR:u32=mask1!(27);
pub const M_LDT_ERRCTL_MAPNXAERR:u32=mask1!(28);

pub const S_LDT_SRICTRL_NEEDRESP:u32=0; pub const M_LDT_SRICTRL_NEEDRESP:u32=mask!(2,0);
pub const S_LDT_SRICTRL_NEEDNPREQ:u32=2; pub const M_LDT_SRICTRL_NEEDNPREQ:u32=mask!(2,2);
pub const S_LDT_SRICTRL_NEEDPREQ:u32=4; pub const M_LDT_SRICTRL_NEEDPREQ:u32=mask!(2,4);
pub const S_LDT_SRICTRL_WANTRESP:u32=8; pub const M_LDT_SRICTRL_WANTRESP:u32=mask!(2,8);
pub const S_LDT_SRICTRL_WANTNPREQ:u32=10; pub const M_LDT_SRICTRL_WANTNPREQ:u32=mask!(2,10);
pub const S_LDT_SRICTRL_WANTPREQ:u32=12; pub const M_LDT_SRICTRL_WANTPREQ:u32=mask!(2,12);
pub const S_LDT_SRICTRL_BUFRELSPACE:u32=16; pub const M_LDT_SRICTRL_BUFRELSPACE:u32=mask!(4,16);

pub const S_LDT_TXBUFCNT_PCMD:u32=0; pub const M_LDT_TXBUFCNT_PCMD:u32=mask!(4,0);
pub const S_LDT_TXBUFCNT_PDATA:u32=4; pub const M_LDT_TXBUFCNT_PDATA:u32=mask!(4,4);
pub const S_LDT_TXBUFCNT_NPCMD:u32=8; pub const M_LDT_TXBUFCNT_NPCMD:u32=mask!(4,8);
pub const S_LDT_TXBUFCNT_NPDATA:u32=12; pub const M_LDT_TXBUFCNT_NPDATA:u32=mask!(4,12);
pub const S_LDT_TXBUFCNT_RCMD:u32=16; pub const M_LDT_TXBUFCNT_RCMD:u32=mask!(4,16);
pub const S_LDT_TXBUFCNT_RDATA:u32=20; pub const M_LDT_TXBUFCNT_RDATA:u32=mask!(4,20);

pub const S_LDT_ADDSTATUS_TGTDONE:u32=0; pub const M_LDT_ADDSTATUS_TGTDONE:u32=mask!(8,0);

/* C field-value/accessor macros, retained as Rust functions. */
pub const fn ldt_make_value(x:u32, shift:u32)->u32 { x << shift }
pub const fn ldt_get_value(x:u32, shift:u32, mask:u32)->u32 { (x & mask) >> shift }
pub const fn V_LDT_DEVICEID_VENDOR(x:u32)->u32 { value!(x,0) }
pub const fn G_LDT_DEVICEID_VENDOR(x:u32)->u32 { get!(x,0,M_LDT_DEVICEID_VENDOR) }
pub const fn V_LDT_DEVICEID_DEVICEID(x:u32)->u32 { value!(x,16) }
pub const fn G_LDT_DEVICEID_DEVICEID(x:u32)->u32 { get!(x,16,M_LDT_DEVICEID_DEVICEID) }
pub const fn V_LDT_CLASSREV_REV(x:u32)->u32 { value!(x,0) }
pub const fn G_LDT_CLASSREV_REV(x:u32)->u32 { get!(x,0,M_LDT_CLASSREV_REV) }
pub const fn V_LDT_CLASSREV_CLASS(x:u32)->u32 { value!(x,8) }
pub const fn G_LDT_CLASSREV_CLASS(x:u32)->u32 { get!(x,8,M_LDT_CLASSREV_CLASS) }
pub const fn V_LDT_DEVHDR_CLINESZ(x:u32)->u32 { value!(x,0) }
pub const fn G_LDT_DEVHDR_CLINESZ(x:u32)->u32 { get!(x,0,M_LDT_DEVHDR_CLINESZ) }
pub const fn V_LDT_DEVHDR_LATTMR(x:u32)->u32 { value!(x,8) }
pub const fn G_LDT_DEVHDR_LATTMR(x:u32)->u32 { get!(x,8,M_LDT_DEVHDR_LATTMR) }
pub const fn V_LDT_DEVHDR_HDRTYPE(x:u32)->u32 { value!(x,16) }
pub const fn G_LDT_DEVHDR_HDRTYPE(x:u32)->u32 { get!(x,16,M_LDT_DEVHDR_HDRTYPE) }
pub const fn V_LDT_DEVHDR_BIST(x:u32)->u32 { value!(x,24) }
pub const fn G_LDT_DEVHDR_BIST(x:u32)->u32 { get!(x,24,M_LDT_DEVHDR_BIST) }
pub const fn V_LDT_STATUS_DEVSELTIMING(x:u32)->u32 { value!(x,25) }
pub const fn G_LDT_STATUS_DEVSELTIMING(x:u32)->u32 { get!(x,25,M_LDT_STATUS_DEVSELTIMING) }
pub const fn V_LDT_CMD_CAPTYPE(x:u32)->u32 { value!(x,29) }
pub const fn G_LDT_CMD_CAPTYPE(x:u32)->u32 { get!(x,29,M_LDT_CMD_CAPTYPE) }
pub const fn V_LDT_LINKCTRL_CRCERR(x:u32)->u32 { value!(x,8) }
pub const fn G_LDT_LINKCTRL_CRCERR(x:u32)->u32 { get!(x,8,M_LDT_LINKCTRL_CRCERR) }
pub const fn V_LDT_LINKCTRL_MAXIN(x:u32)->u32 { value!(x,16) }
pub const fn G_LDT_LINKCTRL_MAXIN(x:u32)->u32 { get!(x,16,M_LDT_LINKCTRL_MAXIN) }
pub const fn V_LDT_LINKCTRL_MAXOUT(x:u32)->u32 { value!(x,20) }
pub const fn G_LDT_LINKCTRL_MAXOUT(x:u32)->u32 { get!(x,20,M_LDT_LINKCTRL_MAXOUT) }
pub const fn V_LDT_LINKCTRL_WIDTHIN(x:u32)->u32 { value!(x,24) }
pub const fn G_LDT_LINKCTRL_WIDTHIN(x:u32)->u32 { get!(x,24,M_LDT_LINKCTRL_WIDTHIN) }
pub const fn V_LDT_LINKCTRL_WIDTHOUT(x:u32)->u32 { value!(x,28) }
pub const fn G_LDT_LINKCTRL_WIDTHOUT(x:u32)->u32 { get!(x,28,M_LDT_LINKCTRL_WIDTHOUT) }
pub const fn V_LDT_LINKFREQ_FREQ(x:u32)->u32 { value!(x,8) }
pub const fn G_LDT_LINKFREQ_FREQ(x:u32)->u32 { get!(x,8,M_LDT_LINKFREQ_FREQ) }
pub const fn V_LDT_SRICMD_RXMARGIN(x:u32)->u32 { value!(x,20) }
pub const fn G_LDT_SRICMD_RXMARGIN(x:u32)->u32 { get!(x,20,M_LDT_SRICMD_RXMARGIN) }
pub const fn V_LDT_SRICMD_TXINITIALOFFSET(x:u32)->u32 { value!(x,28) }
pub const fn G_LDT_SRICMD_TXINITIALOFFSET(x:u32)->u32 { get!(x,28,M_LDT_SRICMD_TXINITIALOFFSET) }
pub const fn V_LDT_SRICTRL_NEEDRESP(x:u32)->u32 { value!(x,0) }
pub const fn G_LDT_SRICTRL_NEEDRESP(x:u32)->u32 { get!(x,0,M_LDT_SRICTRL_NEEDRESP) }
pub const fn V_LDT_SRICTRL_NEEDNPREQ(x:u32)->u32 { value!(x,2) }
pub const fn G_LDT_SRICTRL_NEEDNPREQ(x:u32)->u32 { get!(x,2,M_LDT_SRICTRL_NEEDNPREQ) }
pub const fn V_LDT_SRICTRL_NEEDPREQ(x:u32)->u32 { value!(x,4) }
pub const fn G_LDT_SRICTRL_NEEDPREQ(x:u32)->u32 { get!(x,4,M_LDT_SRICTRL_NEEDPREQ) }
pub const fn V_LDT_SRICTRL_WANTRESP(x:u32)->u32 { value!(x,8) }
pub const fn G_LDT_SRICTRL_WANTRESP(x:u32)->u32 { get!(x,8,M_LDT_SRICTRL_WANTRESP) }
pub const fn V_LDT_SRICTRL_WANTNPREQ(x:u32)->u32 { value!(x,10) }
pub const fn G_LDT_SRICTRL_WANTNPREQ(x:u32)->u32 { get!(x,10,M_LDT_SRICTRL_WANTNPREQ) }
pub const fn V_LDT_SRICTRL_WANTPREQ(x:u32)->u32 { value!(x,12) }
pub const fn G_LDT_SRICTRL_WANTPREQ(x:u32)->u32 { get!(x,12,M_LDT_SRICTRL_WANTPREQ) }
pub const fn V_LDT_SRICTRL_BUFRELSPACE(x:u32)->u32 { value!(x,16) }
pub const fn G_LDT_SRICTRL_BUFRELSPACE(x:u32)->u32 { get!(x,16,M_LDT_SRICTRL_BUFRELSPACE) }
pub const fn V_LDT_TXBUFCNT_PCMD(x:u32)->u32 { value!(x,0) }
pub const fn G_LDT_TXBUFCNT_PCMD(x:u32)->u32 { get!(x,0,M_LDT_TXBUFCNT_PCMD) }
pub const fn V_LDT_TXBUFCNT_PDATA(x:u32)->u32 { value!(x,4) }
pub const fn G_LDT_TXBUFCNT_PDATA(x:u32)->u32 { get!(x,4,M_LDT_TXBUFCNT_PDATA) }
pub const fn V_LDT_TXBUFCNT_NPCMD(x:u32)->u32 { value!(x,8) }
pub const fn G_LDT_TXBUFCNT_NPCMD(x:u32)->u32 { get!(x,8,M_LDT_TXBUFCNT_NPCMD) }
pub const fn V_LDT_TXBUFCNT_NPDATA(x:u32)->u32 { value!(x,12) }
pub const fn G_LDT_TXBUFCNT_NPDATA(x:u32)->u32 { get!(x,12,M_LDT_TXBUFCNT_NPDATA) }
pub const fn V_LDT_TXBUFCNT_RCMD(x:u32)->u32 { value!(x,16) }
pub const fn G_LDT_TXBUFCNT_RCMD(x:u32)->u32 { get!(x,16,M_LDT_TXBUFCNT_RCMD) }
pub const fn V_LDT_TXBUFCNT_RDATA(x:u32)->u32 { value!(x,20) }
pub const fn G_LDT_TXBUFCNT_RDATA(x:u32)->u32 { get!(x,20,M_LDT_TXBUFCNT_RDATA) }
pub const fn V_LDT_ADDSTATUS_TGTDONE(x:u32)->u32 { value!(x,0) }
pub const fn G_LDT_ADDSTATUS_TGTDONE(x:u32)->u32 { get!(x,0,M_LDT_ADDSTATUS_TGTDONE) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
