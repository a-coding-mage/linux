/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ciscode.h
 *
 * The initial developer of the original code is David A. Hinds
 * <dahinds@users.sourceforge.net>.  Portions created by David A. Hinds
 * are Copyright (C) 1999 David A. Hinds.  All Rights Reserved.
 *
 * (C) 1999\t\tDavid A. Hinds
 */

/* Manufacturer and Product ID codes */

pub const MANFID_3COM: u16 = 0x0101;
pub const PRODID_3COM_3CXEM556: u16 = 0x0035;
pub const PRODID_3COM_3CCFEM556: u16 = 0x0556;
pub const PRODID_3COM_3C562: u16 = 0x0562;

pub const MANFID_ACCTON: u16 = 0x01bf;
pub const PRODID_ACCTON_EN2226: u16 = 0x010a;

pub const MANFID_ADAPTEC: u16 = 0x012f;
pub const PRODID_ADAPTEC_SCSI: u16 = 0x0001;

pub const MANFID_ATT: u16 = 0xffff;
pub const PRODID_ATT_KIT: u16 = 0x0100;

pub const MANFID_CONTEC: u16 = 0xc001;

pub const MANFID_FUJITSU: u16 = 0x0004;
pub const PRODID_FUJITSU_MBH10302: u16 = 0x0004;
pub const PRODID_FUJITSU_MBH10304: u16 = 0x1003;
pub const PRODID_FUJITSU_LA501: u16 = 0x2000;

pub const MANFID_IBM: u16 = 0x00a4;
pub const PRODID_IBM_HOME_AND_AWAY: u16 = 0x002e;

pub const MANFID_INTEL: u16 = 0x0089;
pub const PRODID_INTEL_DUAL_RS232: u16 = 0x0301;
pub const PRODID_INTEL_2PLUS: u16 = 0x8422;

pub const MANFID_KME: u16 = 0x0032;
pub const PRODID_KME_KXLC005_A: u16 = 0x0704;
pub const PRODID_KME_KXLC005_B: u16 = 0x2904;

pub const MANFID_LINKSYS: u16 = 0x0143;
pub const PRODID_LINKSYS_PCMLM28: u16 = 0xc0ab;
pub const PRODID_LINKSYS_3400: u16 = 0x3341;

pub const MANFID_MEGAHERTZ: u16 = 0x0102;
pub const PRODID_MEGAHERTZ_VARIOUS: u16 = 0x0000;
pub const PRODID_MEGAHERTZ_EM3288: u16 = 0x0006;

pub const MANFID_MACNICA: u16 = 0xc00b;

pub const MANFID_MOTOROLA: u16 = 0x0109;
pub const PRODID_MOTOROLA_MARINER: u16 = 0x0501;

pub const MANFID_NATINST: u16 = 0x010b;
pub const PRODID_NATINST_QUAD_RS232: u16 = 0xd180;

pub const MANFID_NEW_MEDIA: u16 = 0x0057;

pub const MANFID_NOKIA: u16 = 0x0124;
pub const PRODID_NOKIA_CARDPHONE: u16 = 0x0900;

pub const MANFID_OLICOM: u16 = 0x0121;
pub const PRODID_OLICOM_OC2231: u16 = 0x3122;
pub const PRODID_OLICOM_OC2232: u16 = 0x3222;

pub const MANFID_OMEGA: u16 = 0x0137;
pub const PRODID_OMEGA_QSP_100: u16 = 0x0025;

pub const MANFID_OSITECH: u16 = 0x0140;
pub const PRODID_OSITECH_JACK_144: u16 = 0x0001;
pub const PRODID_OSITECH_JACK_288: u16 = 0x0002;
pub const PRODID_OSITECH_JACK_336: u16 = 0x0007;
pub const PRODID_OSITECH_SEVEN: u16 = 0x0008;

pub const MANFID_OXSEMI: u16 = 0x0279;

pub const MANFID_PIONEER: u16 = 0x000b;

pub const MANFID_PSION: u16 = 0x016c;
pub const PRODID_PSION_NET100: u16 = 0x0023;

pub const MANFID_QUATECH: u16 = 0x0137;
pub const PRODID_QUATECH_SPP100: u16 = 0x0003;
pub const PRODID_QUATECH_DUAL_RS232: u16 = 0x0012;
pub const PRODID_QUATECH_DUAL_RS232_D1: u16 = 0x0007;
pub const PRODID_QUATECH_DUAL_RS232_D2: u16 = 0x0052;
pub const PRODID_QUATECH_DUAL_RS232_G: u16 = 0x004d;
pub const PRODID_QUATECH_QUAD_RS232: u16 = 0x001b;
pub const PRODID_QUATECH_DUAL_RS422: u16 = 0x000e;
pub const PRODID_QUATECH_QUAD_RS422: u16 = 0x0045;

pub const MANFID_SMC: u16 = 0x0108;
pub const PRODID_SMC_ETHER: u16 = 0x0105;

pub const MANFID_SOCKET: u16 = 0x0104;
pub const PRODID_SOCKET_DUAL_RS232: u16 = 0x0006;
pub const PRODID_SOCKET_EIO: u16 = 0x000a;
pub const PRODID_SOCKET_LPE: u16 = 0x000d;
pub const PRODID_SOCKET_LPE_CF: u16 = 0x0075;

pub const MANFID_SUNDISK: u16 = 0x0045;

pub const MANFID_TDK: u16 = 0x0105;
pub const PRODID_TDK_CF010: u16 = 0x0900;
pub const PRODID_TDK_NP9610: u16 = 0x0d0a;
pub const PRODID_TDK_MN3200: u16 = 0x0e0a;
pub const PRODID_TDK_GN3410: u16 = 0x4815;

pub const MANFID_TOSHIBA: u16 = 0x0098;

pub const MANFID_UNGERMANN: u16 = 0x02c0;

pub const MANFID_XIRCOM: u16 = 0x0105;

pub const MANFID_POSSIO: u16 = 0x030c;
pub const PRODID_POSSIO_GCC: u16 = 0x0003;

pub const MANFID_NEC: u16 = 0x0010;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
