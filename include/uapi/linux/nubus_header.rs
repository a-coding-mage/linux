/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
  nubus.h: various definitions and prototypes for NuBus drivers to use.

  Originally written by Alan Cox.

  Hacked to death by C. Scott Ananian and David Huggins-Daines.

  Some of the constants in here are from the corresponding
  NetBSD/OpenBSD header file, by Allen Briggs.  We figured out the
  rest of them on our own.
*/

/* C enums are represented as integer constants to preserve duplicate values. */
pub type NubusCategory = i32;
pub const NUBUS_CAT_BOARD: NubusCategory = 0x0001;
pub const NUBUS_CAT_DISPLAY: NubusCategory = 0x0003;
pub const NUBUS_CAT_NETWORK: NubusCategory = 0x0004;
pub const NUBUS_CAT_COMMUNICATIONS: NubusCategory = 0x0006;
pub const NUBUS_CAT_FONT: NubusCategory = 0x0009;
pub const NUBUS_CAT_CPU: NubusCategory = 0x000A;
/* For lack of a better name */
pub const NUBUS_CAT_DUODOCK: NubusCategory = 0x0020;

pub type NubusTypeNetwork = i32;
pub const NUBUS_TYPE_ETHERNET: NubusTypeNetwork = 0x0001;
pub const NUBUS_TYPE_RS232: NubusTypeNetwork = 0x0002;

pub type NubusTypeDisplay = i32;
pub const NUBUS_TYPE_VIDEO: NubusTypeDisplay = 0x0001;

pub type NubusTypeCpu = i32;
pub const NUBUS_TYPE_68020: NubusTypeCpu = 0x0003;
pub const NUBUS_TYPE_68030: NubusTypeCpu = 0x0004;
pub const NUBUS_TYPE_68040: NubusTypeCpu = 0x0005;

/* Known <Cat,Type,SW,HW> tuples: see the original header for the full list. */

/* DrSW: Uniquely identifies the software interface to a board. */
pub type NubusDrsw = i32;
pub const NUBUS_DRSW_APPLE: NubusDrsw = 0x0001;
pub const NUBUS_DRSW_APPLE_HIRES: NubusDrsw = 0x0013;
pub const NUBUS_DRSW_3COM: NubusDrsw = 0x0000;
pub const NUBUS_DRSW_CABLETRON: NubusDrsw = 0x0001;
pub const NUBUS_DRSW_SONIC_LC: NubusDrsw = 0x0001;
pub const NUBUS_DRSW_KINETICS: NubusDrsw = 0x0103;
pub const NUBUS_DRSW_ASANTE: NubusDrsw = 0x0104;
pub const NUBUS_DRSW_TECHWORKS: NubusDrsw = 0x0109;
pub const NUBUS_DRSW_DAYNA: NubusDrsw = 0x010b;
pub const NUBUS_DRSW_FARALLON: NubusDrsw = 0x010c;
pub const NUBUS_DRSW_APPLE_SN: NubusDrsw = 0x010f;
pub const NUBUS_DRSW_DAYNA2: NubusDrsw = 0x0115;
pub const NUBUS_DRSW_FOCUS: NubusDrsw = 0x011a;
pub const NUBUS_DRSW_ASANTE_CS: NubusDrsw = 0x011d;
pub const NUBUS_DRSW_DAYNA_LC: NubusDrsw = 0x011e;
pub const NUBUS_DRSW_NONE: NubusDrsw = 0x0000;

/* DrHW: Uniquely identifies the hardware interface to a board. */
pub type NubusDrhw = i32;
pub const NUBUS_DRHW_APPLE_TFB: NubusDrhw = 0x0001;
pub const NUBUS_DRHW_APPLE_WVC: NubusDrhw = 0x0006;
pub const NUBUS_DRHW_SIGMA_CLRMAX: NubusDrhw = 0x0007;
pub const NUBUS_DRHW_APPLE_SE30: NubusDrhw = 0x0009;
pub const NUBUS_DRHW_APPLE_HRVC: NubusDrhw = 0x0013;
pub const NUBUS_DRHW_APPLE_MVC: NubusDrhw = 0x0014;
pub const NUBUS_DRHW_APPLE_PVC: NubusDrhw = 0x0017;
pub const NUBUS_DRHW_APPLE_RBV1: NubusDrhw = 0x0018;
pub const NUBUS_DRHW_APPLE_MDC: NubusDrhw = 0x0019;
pub const NUBUS_DRHW_APPLE_VSC: NubusDrhw = 0x0020;
pub const NUBUS_DRHW_APPLE_SONORA: NubusDrhw = 0x0022;
pub const NUBUS_DRHW_APPLE_JET: NubusDrhw = 0x0029;
pub const NUBUS_DRHW_APPLE_24AC: NubusDrhw = 0x002b;
pub const NUBUS_DRHW_APPLE_VALKYRIE: NubusDrhw = 0x002e;
pub const NUBUS_DRHW_SMAC_GFX: NubusDrhw = 0x0105;
pub const NUBUS_DRHW_RASTER_CB264: NubusDrhw = 0x013B;
pub const NUBUS_DRHW_MICRON_XCEED: NubusDrhw = 0x0146;
pub const NUBUS_DRHW_RDIUS_GSC: NubusDrhw = 0x0153;
pub const NUBUS_DRHW_SMAC_SPEC8: NubusDrhw = 0x017B;
pub const NUBUS_DRHW_SMAC_SPEC24: NubusDrhw = 0x017C;
pub const NUBUS_DRHW_RASTER_CB364: NubusDrhw = 0x026F;
pub const NUBUS_DRHW_RDIUS_DCGX: NubusDrhw = 0x027C;
pub const NUBUS_DRHW_RDIUS_PC8: NubusDrhw = 0x0291;
pub const NUBUS_DRHW_LAPIS_PCS8: NubusDrhw = 0x0292;
pub const NUBUS_DRHW_RASTER_24XLI: NubusDrhw = 0x02A0;
pub const NUBUS_DRHW_RASTER_PBPGT: NubusDrhw = 0x02A5;
pub const NUBUS_DRHW_EMACH_FSX: NubusDrhw = 0x02AE;
pub const NUBUS_DRHW_RASTER_24XLTV: NubusDrhw = 0x02B7;
pub const NUBUS_DRHW_SMAC_THUND24: NubusDrhw = 0x02CB;
pub const NUBUS_DRHW_SMAC_THUNDLGHT: NubusDrhw = 0x03D9;
pub const NUBUS_DRHW_RDIUS_PC24XP: NubusDrhw = 0x0406;
pub const NUBUS_DRHW_RDIUS_PC24X: NubusDrhw = 0x040A;
pub const NUBUS_DRHW_RDIUS_PC8XJ: NubusDrhw = 0x040B;
pub const NUBUS_DRHW_INTERLAN: NubusDrhw = 0x0100;
pub const NUBUS_DRHW_SMC9194: NubusDrhw = 0x0101;
pub const NUBUS_DRHW_KINETICS: NubusDrhw = 0x0106;
pub const NUBUS_DRHW_CABLETRON: NubusDrhw = 0x0109;
pub const NUBUS_DRHW_ASANTE_LC: NubusDrhw = 0x010f;
pub const NUBUS_DRHW_SONIC: NubusDrhw = 0x0110;
pub const NUBUS_DRHW_TECHWORKS: NubusDrhw = 0x0112;
pub const NUBUS_DRHW_APPLE_SONIC_NB: NubusDrhw = 0x0118;
pub const NUBUS_DRHW_APPLE_SONIC_LC: NubusDrhw = 0x0119;
pub const NUBUS_DRHW_FOCUS: NubusDrhw = 0x011c;
pub const NUBUS_DRHW_SONNET: NubusDrhw = 0x011d;

pub type NubusResId = i32;
pub const NUBUS_RESID_TYPE: NubusResId = 0x0001;
pub const NUBUS_RESID_NAME: NubusResId = 0x0002;
pub const NUBUS_RESID_ICON: NubusResId = 0x0003;
pub const NUBUS_RESID_DRVRDIR: NubusResId = 0x0004;
pub const NUBUS_RESID_LOADREC: NubusResId = 0x0005;
pub const NUBUS_RESID_BOOTREC: NubusResId = 0x0006;
pub const NUBUS_RESID_FLAGS: NubusResId = 0x0007;
pub const NUBUS_RESID_HWDEVID: NubusResId = 0x0008;
pub const NUBUS_RESID_MINOR_BASEOS: NubusResId = 0x000a;
pub const NUBUS_RESID_MINOR_LENGTH: NubusResId = 0x000b;
pub const NUBUS_RESID_MAJOR_BASEOS: NubusResId = 0x000c;
pub const NUBUS_RESID_MAJOR_LENGTH: NubusResId = 0x000d;
pub const NUBUS_RESID_CICN: NubusResId = 0x000f;
pub const NUBUS_RESID_ICL8: NubusResId = 0x0010;
pub const NUBUS_RESID_ICL4: NubusResId = 0x0011;
pub const NUBUS_RESID_BOARDID: NubusResId = 0x0020;
pub const NUBUS_RESID_PRAMINITDATA: NubusResId = 0x0021;
pub const NUBUS_RESID_PRIMARYINIT: NubusResId = 0x0022;
pub const NUBUS_RESID_TIMEOUTCONST: NubusResId = 0x0023;
pub const NUBUS_RESID_VENDORINFO: NubusResId = 0x0024;
pub const NUBUS_RESID_BOARDFLAGS: NubusResId = 0x0025;
pub const NUBUS_RESID_SECONDINIT: NubusResId = 0x0026;
pub const NUBUS_RESID_VIDNAMES: NubusResId = 0x0041;
pub const NUBUS_RESID_VIDMODES: NubusResId = 0x007e;
pub const NUBUS_RESID_VEND_ID: NubusResId = 0x0001;
pub const NUBUS_RESID_VEND_SERIAL: NubusResId = 0x0002;
pub const NUBUS_RESID_VEND_REV: NubusResId = 0x0003;
pub const NUBUS_RESID_VEND_PART: NubusResId = 0x0004;
pub const NUBUS_RESID_VEND_DATE: NubusResId = 0x0005;
pub const NUBUS_RESID_MAC_ADDRESS: NubusResId = 0x0080;
pub const NUBUS_RESID_MEMINFO: NubusResId = 0x0081;
pub const NUBUS_RESID_ROMINFO: NubusResId = 0x0082;
pub const NUBUS_RESID_GAMMADIR: NubusResId = 0x0040;
pub const NUBUS_RESID_FIRSTMODE: NubusResId = 0x0080;
pub const NUBUS_RESID_SECONDMODE: NubusResId = 0x0081;
pub const NUBUS_RESID_THIRDMODE: NubusResId = 0x0082;
pub const NUBUS_RESID_FOURTHMODE: NubusResId = 0x0083;
pub const NUBUS_RESID_FIFTHMODE: NubusResId = 0x0084;
pub const NUBUS_RESID_SIXTHMODE: NubusResId = 0x0085;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
