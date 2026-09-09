/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of newport.h. */

pub type NpiregT = u32;

#[repr(C)]
pub union Npfloat { pub flt: f32, pub word: NpiregT }
pub type NpfregT = Npfloat;

#[repr(C)]
pub union NpDcb {
    pub byword: NpiregT,
    pub byshort: [u16; 2],
    pub bybytes: [u8; 4],
}

#[repr(C)]
pub struct NewportRexregs {
    pub drawmode1: NpiregT, pub drawmode0: NpiregT, pub lsmode: NpiregT,
    pub lspattern: NpiregT, pub lspatsave: NpiregT, pub zpattern: NpiregT,
    pub colorback: NpiregT, pub colorvram: NpiregT, pub alpharef: NpiregT,
    pub pad0: u32, pub smask0x: NpiregT, pub smask0y: NpiregT,
    pub _setup: NpiregT, pub _stepz: NpiregT, pub _lsrestore: NpiregT, pub _lssave: NpiregT,
    pub _pad1: [u32; 0x30],
    pub _xstart: NpfregT, pub _ystart: NpfregT, pub _xend: NpfregT, pub _yend: NpfregT,
    pub xsave: NpiregT, pub xymove: NpiregT, pub bresd: NpfregT, pub bress1: NpfregT,
    pub bresoctinc1: NpfregT, pub bresrndinc2: i32, pub brese1: NpfregT, pub bress2: NpfregT,
    pub aweight0: NpiregT, pub aweight1: NpiregT, pub xstartf: NpfregT, pub ystartf: NpfregT,
    pub xendf: NpfregT, pub yendf: NpfregT, pub xstarti: NpiregT, pub xendf1: NpfregT,
    pub xystarti: NpiregT, pub xyendi: NpiregT, pub xstartendi: NpiregT,
    pub _unused2: [u32; 0x29],
    pub colorred: NpfregT, pub coloralpha: NpfregT, pub colorgrn: NpfregT, pub colorblue: NpfregT,
    pub slopered: NpfregT, pub slopealpha: NpfregT, pub slopegrn: NpfregT, pub slopeblue: NpfregT,
    pub wrmask: NpiregT, pub colori: NpiregT, pub colorx: NpfregT, pub slopered1: NpfregT,
    pub hostrw0: NpiregT, pub hostrw1: NpiregT, pub dcbmode: NpiregT, pub _unused3: u32,
    pub dcbdata0: NpDcb, pub dcbdata1: NpiregT,
}

#[repr(C)]
pub struct NewportCregs {
    pub smask1x: NpiregT, pub smask1y: NpiregT, pub smask2x: NpiregT, pub smask2y: NpiregT,
    pub smask3x: NpiregT, pub smask3y: NpiregT, pub smask4x: NpiregT, pub smask4y: NpiregT,
    pub topscan: NpiregT, pub xywin: NpiregT, pub clipmode: NpiregT, pub _unused0: u32,
    pub config: u32, pub _unused1: NpiregT, pub status: NpiregT, pub ustatus: NpiregT,
    pub dcbreset: NpiregT,
}

#[repr(C)]
pub struct NewportRegs {
    pub set: NewportRexregs, pub _unused0: [u32; 0x16e], pub go: NewportRexregs,
    pub _unused1: [u32; 0x22e], pub cset: NewportCregs, pub _unused2: [u32; 0x1ef], pub cgo: NewportCregs,
}

#[repr(C)]
pub struct NewportCtx {
    pub drawmode1:u32, pub drawmode0:u32, pub lsmode:u32, pub lspattern:u32, pub lspatsave:u32,
    pub zpattern:u32, pub colorback:u32, pub colorvram:u32, pub alpharef:u32, pub smask0x:u32, pub smask0y:u32,
    pub _xstart:u32, pub _ystart:u32, pub _xend:u32, pub _yend:u32, pub xsave:u32, pub xymove:u32,
    pub bresd:u32, pub bress1:u32, pub bresoctinc1:u32, pub bresrndinc2:u32, pub brese1:u32, pub bress2:u32,
    pub aweight0:u32, pub aweight1:u32, pub colorred:u32, pub coloralpha:u32, pub colorgrn:u32, pub colorblue:u32,
    pub slopered:u32, pub slopealpha:u32, pub slopegrn:u32, pub slopeblue:u32, pub wrmask:u32, pub hostrw0:u32, pub hostrw1:u32,
    pub smask1x:u32, pub smask1y:u32, pub smask2x:u32, pub smask2y:u32, pub smask3x:u32, pub smask3y:u32, pub smask4x:u32, pub smask4y:u32,
    pub topscan:u32, pub xywin:u32, pub clipmode:u32, pub config:u32, pub dcbmode:u32, pub dcbdata0:u32, pub dcbdata1:u32,
}

macro_rules! c { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: u32 = $v;)* }; }
c!(DM1_PLANES=0x7,DM1_NOPLANES=0,DM1_RGBPLANES=1,DM1_RGBAPLANES=2,DM1_OLAYPLANES=4,DM1_PUPPLANES=5,DM1_CIDPLANES=6,
NPORT_DMODE1_DDMASK=0x18,NPORT_DMODE1_DD4=0,NPORT_DMODE1_DD8=0x8,NPORT_DMODE1_DD12=0x10,NPORT_DMODE1_DD24=0x18,
NPORT_DMODE1_DSRC=0x20,NPORT_DMODE1_YFLIP=0x40,NPORT_DMODE1_RWPCKD=0x80,NPORT_DMODE1_HDMASK=0x300,NPORT_DMODE1_HD4=0,NPORT_DMODE1_HD8=0x100,NPORT_DMODE1_HD12=0x200,NPORT_DMODE1_HD32=0x300,NPORT_DMODE1_RWDBL=0x400,NPORT_DMODE1_ESWAP=0x800,NPORT_DMODE1_CCMASK=0x7000,NPORT_DMODE1_CCLT=0x1000,NPORT_DMODE1_CCEQ=0x2000,NPORT_DMODE1_CCGT=0x4000,NPORT_DMODE1_RGBMD=0x8000,NPORT_DMODE1_DENAB=0x10000,NPORT_DMODE1_FCLR=0x20000,NPORT_DMODE1_BENAB=0x40000,NPORT_DMODE1_SFMASK=0x380000,NPORT_DMODE1_SF0=0,NPORT_DMODE1_SF1=0x80000,NPORT_DMODE1_SFDC=0x100000,NPORT_DMODE1_SFMDC=0x180000,NPORT_DMODE1_SFSA=0x200000,NPORT_DMODE1_SFMSA=0x280000,NPORT_DMODE1_DFMASK=0x1c00000,NPORT_DMODE1_DF0=0,NPORT_DMODE1_DF1=0x400000,NPORT_DMODE1_DFSC=0x800000,NPORT_DMODE1_DFMSC=0xc00000,NPORT_DMODE1_DFSA=0x1000000,NPORT_DMODE1_DFMSA=0x1400000,NPORT_DMODE1_BBENAB=0x2000000,NPORT_DMODE1_PFENAB=0x4000000,NPORT_DMODE1_ABLEND=0x8000000,NPORT_DMODE1_LOMASK=0xf0000000,NPORT_DMODE1_LOZERO=0,NPORT_DMODE1_LOAND=0x10000000,NPORT_DMODE1_LOANDR=0x20000000,NPORT_DMODE1_LOSRC=0x30000000,NPORT_DMODE1_LOANDI=0x40000000,NPORT_DMODE1_LODST=0x50000000,NPORT_DMODE1_LOXOR=0x60000000,NPORT_DMODE1_LOOR=0x70000000,NPORT_DMODE1_LONOR=0x80000000,NPORT_DMODE1_LOXNOR=0x90000000,NPORT_DMODE1_LONDST=0xa0000000,NPORT_DMODE1_LOORR=0xb0000000,NPORT_DMODE1_LONSRC=0xc0000000,NPORT_DMODE1_LOORI=0xd0000000,NPORT_DMODE1_LONAND=0xe0000000,NPORT_DMODE1_LOONE=0xf0000000,
NPORT_DMODE0_OPMASK=3,NPORT_DMODE0_NOP=0,NPORT_DMODE0_RD=1,NPORT_DMODE0_DRAW=2,NPORT_DMODE0_S2S=3,NPORT_DMODE0_AMMASK=0x1c,NPORT_DMODE0_SPAN=0,NPORT_DMODE0_BLOCK=4,NPORT_DMODE0_ILINE=8,NPORT_DMODE0_FLINE=0xc,NPORT_DMODE0_ALINE=0x10,NPORT_DMODE0_TLINE=0x14,NPORT_DMODE0_BLINE=0x18,NPORT_DMODE0_DOSETUP=0x20,NPORT_DMODE0_CHOST=0x40,NPORT_DMODE0_AHOST=0x80,NPORT_DMODE0_STOPX=0x100,NPORT_DMODE0_STOPY=0x200,NPORT_DMODE0_SK1ST=0x400,NPORT_DMODE0_SKLST=0x800,NPORT_DMODE0_ZPENAB=0x1000,NPORT_DMODE0_LISPENAB=0x2000,NPORT_DMODE0_LISLST=0x4000,NPORT_DMODE0_L32=0x8000,NPORT_DMODE0_ZOPQ=0x10000,NPORT_DMODE0_LISOPQ=0x20000,NPORT_DMODE0_SHADE=0x40000,NPORT_DMODE0_LRONLY=0x80000,NPORT_DMODE0_XYOFF=0x100000,NPORT_DMODE0_CLAMP=0x200000,NPORT_DMODE0_ENDPF=0x400000,NPORT_DMODE0_YSTR=0x800000,
NPORT_DMODE_AVC2=0,NPORT_DMODE_ACMALL=0x80,NPORT_DMODE_W1=1,NPORT_DMODE_W2=2,NPORT_DMODE_W3=3,NPORT_DMODE_ECINC=8,NPORT_DMODE_EASACK=0x1000,NPORT_DMODE_SENDIAN=0x10000000);

c!(NPORT_DMODE_WMASK=3,NPORT_DMODE_W4=0,NPORT_DMODE_EDPACK=4,NPORT_DMODE_CMASK=0x70,NPORT_DMODE_AMASK=0x780,
NPORT_DMODE_ACM0=0x100,NPORT_DMODE_ACM1=0x180,NPORT_DMODE_AXMALL=0x200,NPORT_DMODE_AXM0=0x280,NPORT_DMODE_AXM1=0x300,NPORT_DMODE_ABT=0x380,NPORT_DMODE_AVCC1=0x400,NPORT_DMODE_AVAB1=0x480,NPORT_DMODE_ALG3V0=0x500,NPORT_DMODE_A1562=0x580,NPORT_DMODE_ESACK=0x800,NPORT_DMODE_CWMASK=0x3e000,NPORT_DMODE_CHMASK=0x7c0000,NPORT_DMODE_CSMASK=0xf800000,
NPORT_CMODE_SM0=1,NPORT_CMODE_SM1=2,NPORT_CMODE_SM2=4,NPORT_CMODE_SM3=8,NPORT_CMODE_SM4=0x10,NPORT_CMODE_CMSK=0x1e00,
NPORT_CFG_G32MD=1,NPORT_CFG_BWIDTH=2,NPORT_CFG_ERCVR=4,NPORT_CFG_BDMSK=0x78,NPORT_CFG_BFAINT=0x80,NPORT_CFG_GDMSK=0x1f80,NPORT_CFG_GD0=0x100,NPORT_CFG_GD1=0x200,NPORT_CFG_GD2=0x400,NPORT_CFG_GD3=0x800,NPORT_CFG_GD4=0x1000,NPORT_CFG_GFAINT=0x2000,NPORT_CFG_TOMSK=0x1c000,NPORT_CFG_VRMSK=0xe0000,NPORT_CFG_FBTYP=0x100000,
NPORT_STAT_VERS=7,NPORT_STAT_GBUSY=8,NPORT_STAT_BBUSY=0x10,NPORT_STAT_VRINT=0x20,NPORT_STAT_VIDINT=0x40,NPORT_STAT_GLMSK=0x1f80,NPORT_STAT_BLMSK=0x7e000,NPORT_STAT_BFIRQ=0x80000,NPORT_STAT_GFIRQ=0x100000,
VC2_REGADDR_INDEX=0,VC2_REGADDR_IREG=0x10,VC2_REGADDR_RAM=0x30,VC2_PROTOCOL=0x848000,VC2_VLINET_ADDR=0,VC2_VFRAMET_ADDR=0x400,VC2_CGLYPH_ADDR=0x500,
VC2_IREG_VENTRY=0,VC2_IREG_CENTRY=1,VC2_IREG_CURSX=2,VC2_IREG_CURSY=3,VC2_IREG_CCURSX=4,VC2_IREG_DENTRY=5,VC2_IREG_SLEN=6,VC2_IREG_RADDR=7,VC2_IREG_VFPTR=8,VC2_IREG_VLSPTR=9,VC2_IREG_VLIR=0xa,VC2_IREG_VLCTR=0xb,VC2_IREG_CTPTR=0xc,VC2_IREG_WCURSY=0xd,VC2_IREG_DFPTR=0xe,VC2_IREG_DLTPTR=0xf,VC2_IREG_CONTROL=0x10,VC2_IREG_CONFIG=0x20,
VC2_CTRL_EVIRQ=1,VC2_CTRL_EDISP=2,VC2_CTRL_EVIDEO=4,VC2_CTRL_EDIDS=8,VC2_CTRL_ECURS=0x10,VC2_CTRL_EGSYNC=0x20,VC2_CTRL_EILACE=0x40,VC2_CTRL_ECDISP=0x80,VC2_CTRL_ECCURS=0x100,VC2_CTRL_ECG64=0x200,VC2_CTRL_GLSEL=0x400,
NCMAP_REGADDR_AREG=0,NCMAP_REGADDR_ALO=0,NCMAP_REGADDR_AHI=0x10,NCMAP_REGADDR_PBUF=0x20,NCMAP_REGADDR_CREG=0x30,NCMAP_REGADDR_SREG=0x40,NCMAP_REGADDR_RREG=0x60,NCMAP_PROTOCOL=0x848000,
BUSY_TIMEOUT=100000,DCB_DATAWIDTH_4=0,DCB_DATAWIDTH_1=1,DCB_DATAWIDTH_2=2,DCB_DATAWIDTH_3=3,DCB_ENDATAPACK=4,DCB_ENCRSINC=8,DCB_CRS_SHIFT=4,DCB_ADDR_SHIFT=7,DCB_VC2=0,DCB_CMAP_ALL=0x80,DCB_CMAP0=0x100,DCB_CMAP1=0x180,DCB_XMAP_ALL=0x200,DCB_XMAP0=0x280,DCB_XMAP1=0x300,DCB_BT445=0x380,DCB_VCC1=0x400,DCB_VAB1=0x480,DCB_LG3_BDVERS0=0x500,DCB_LG3_ICS1562=0x580,DCB_RESERVED=0x780,DCB_ENSYNCACK=0x800,DCB_ENASYNCACK=0x1000,DCB_CSWIDTH_SHIFT=13,DCB_CSHOLD_SHIFT=18,DCB_CSSETUP_SHIFT=23,
XM9_CRS_CONFIG=0,XM9_PUPMODE=1,XM9_ODD_PIXEL=2,XM9_8_BITPLANES=4,XM9_SLOW_DCB=8,XM9_VIDEO_RGBMAP_MASK=0x30,XM9_EXPRESS_VIDEO=0x40,XM9_VIDEO_OPTION=0x80,XM9_CRS_REVISION=0x10,XM9_CRS_FIFO_AVAIL=0x20,XM9_FIFO_0_AVAIL=0,XM9_FIFO_1_AVAIL=1,XM9_FIFO_2_AVAIL=3,XM9_FIFO_3_AVAIL=2,XM9_FIFO_FULL=0,XM9_FIFO_EMPTY=2,XM9_CRS_CURS_CMAP_MSB=0x30,XM9_CRS_PUP_CMAP_MSB=0x40,XM9_CRS_MODE_REG_DATA=0x50,XM9_CRS_MODE_REG_INDEX=0x70,BT445_PROTOCOL=0x6180000,BT445_CSR_ADDR_REG=0,BT445_CSR_REVISION=0x20,BT445_REVISION_REG=1);

#[inline] pub unsafe fn newport_vc2_set(regs: *mut NewportRegs, vc2ireg: u8, val: u16) { (*regs).set.dcbmode = NPORT_DMODE_AVC2 | VC2_REGADDR_INDEX | NPORT_DMODE_W3 | NPORT_DMODE_ECINC | VC2_PROTOCOL; (*regs).set.dcbdata0.byword = ((vc2ireg as u32) << 24) | ((val as u32) << 8); }
#[inline] pub unsafe fn newport_vc2_get(regs: *mut NewportRegs, vc2ireg: u8) -> u16 { (*regs).set.dcbmode=NPORT_DMODE_AVC2|VC2_REGADDR_INDEX|NPORT_DMODE_W1|NPORT_DMODE_ECINC|VC2_PROTOCOL; (*regs).set.dcbdata0.bybytes[3]=vc2ireg; (*regs).set.dcbmode=NPORT_DMODE_AVC2|VC2_REGADDR_IREG|NPORT_DMODE_W2|NPORT_DMODE_ECINC|VC2_PROTOCOL; (*regs).set.dcbdata0.byshort[1] }
#[inline] pub unsafe fn newport_cmap_setaddr(regs: *mut NewportRegs, addr: u16) { (*regs).set.dcbmode=NPORT_DMODE_ACMALL|NCMAP_PROTOCOL|NPORT_DMODE_SENDIAN|NPORT_DMODE_ECINC|NCMAP_REGADDR_AREG|NPORT_DMODE_W2; (*regs).set.dcbdata0.byshort[1]=addr; (*regs).set.dcbmode=NPORT_DMODE_ACMALL|NCMAP_PROTOCOL|NCMAP_REGADDR_PBUF|NPORT_DMODE_W3; }
#[inline] pub unsafe fn newport_cmap_setrgb(regs: *mut NewportRegs, red:u8, green:u8, blue:u8) { (*regs).set.dcbdata0.byword=((red as u32)<<24)|((green as u32)<<16)|((blue as u32)<<8); }
#[inline] pub unsafe fn newport_wait(regs: *mut NewportRegs) -> i32 { let mut t=BUSY_TIMEOUT as i32; while { t-=1; t != 0 } { if (*regs).cset.status & NPORT_STAT_GBUSY == 0 { break; } } if t != 0 { 0 } else { 1 } }
#[inline] pub unsafe fn newport_bfwait(regs: *mut NewportRegs) -> i32 { let mut t=BUSY_TIMEOUT as i32; while { t-=1; t != 0 } { if (*regs).cset.status & NPORT_STAT_BBUSY == 0 { break; } } if t != 0 { 0 } else { 1 } }
#[inline] pub unsafe fn xmap9FIFOWait(rex:*mut NewportRegs) { (*rex).set.dcbmode=0x280|0x20|1|0x6180000; newport_bfwait(rex); while (*rex).set.dcbdata0.bybytes[3]&3 != XM9_FIFO_EMPTY {} }
#[inline] pub unsafe fn xmap9SetModeReg(rex:*mut NewportRegs, modereg:u32, data24:u32, cfreq:i32) { let p=if cfreq>119 {0x6180000} else if cfreq>59 {0xa500000} else {0x181800000}; (*rex).set.dcbmode=0x200|0x50|4|p; (*rex).set.dcbdata0.byword=(modereg<<24)|(data24&0xffffff); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
