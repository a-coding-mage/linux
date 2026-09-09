/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Linux/PA-RISC floating-point emulation declarations and bitfield helpers. */

/* C includes and LOCORE are build-context dependencies supplied elsewhere. */

/* Single precision. */
macro_rules! Sall { ($object:expr) => { $object }; }
macro_rules! Ssign { ($object:expr) => { Bitfield_extract(0, 1, $object) }; }
macro_rules! Ssignedsign { ($object:expr) => { Bitfield_signed_extract(0, 1, $object) }; }
macro_rules! Sexponent { ($object:expr) => { Bitfield_extract(1, 8, $object) }; }
macro_rules! Smantissa { ($object:expr) => { Bitfield_mask(9, 23, $object) }; }
macro_rules! Ssignaling { ($object:expr) => { Bitfield_extract(9, 1, $object) }; }
macro_rules! Ssignalingnan { ($object:expr) => { Bitfield_extract(1, 9, $object) }; }
macro_rules! Shigh2mantissa { ($object:expr) => { Bitfield_extract(9, 2, $object) }; }
macro_rules! Sexponentmantissa { ($object:expr) => { Bitfield_mask(1, 31, $object) }; }
macro_rules! Ssignexponent { ($object:expr) => { Bitfield_extract(0, 9, $object) }; }
macro_rules! Shidden { ($object:expr) => { Bitfield_extract(8, 1, $object) }; }
macro_rules! Shiddenoverflow { ($object:expr) => { Bitfield_extract(7, 1, $object) }; }
macro_rules! Shiddenhigh7mantissa { ($object:expr) => { Bitfield_extract(8, 8, $object) }; }
macro_rules! Shiddenhigh3mantissa { ($object:expr) => { Bitfield_extract(8, 4, $object) }; }
macro_rules! Slow { ($object:expr) => { Bitfield_mask(31, 1, $object) }; }
macro_rules! Slow4 { ($object:expr) => { Bitfield_mask(28, 4, $object) }; }
macro_rules! Slow31 { ($object:expr) => { Bitfield_mask(1, 31, $object) }; }
macro_rules! Shigh31 { ($object:expr) => { Bitfield_extract(0, 31, $object) }; }
macro_rules! Ssignedhigh31 { ($object:expr) => { Bitfield_signed_extract(0, 31, $object) }; }
macro_rules! Shigh4 { ($object:expr) => { Bitfield_extract(0, 4, $object) }; }
macro_rules! Sbit24 { ($object:expr) => { Bitfield_extract(24, 1, $object) }; }
macro_rules! Sbit28 { ($object:expr) => { Bitfield_extract(28, 1, $object) }; }
macro_rules! Sbit29 { ($object:expr) => { Bitfield_extract(29, 1, $object) }; }
macro_rules! Sbit30 { ($object:expr) => { Bitfield_extract(30, 1, $object) }; }
macro_rules! Sbit31 { ($object:expr) => { Bitfield_mask(31, 1, $object) }; }

macro_rules! Deposit_ssign { ($object:expr, $value:expr) => { Bitfield_deposit($value, 0, 1, $object) }; }
macro_rules! Deposit_sexponent { ($object:expr, $value:expr) => { Bitfield_deposit($value, 1, 8, $object) }; }
macro_rules! Deposit_smantissa { ($object:expr, $value:expr) => { Bitfield_deposit($value, 9, 23, $object) }; }
macro_rules! Deposit_shigh2mantissa { ($object:expr, $value:expr) => { Bitfield_deposit($value, 9, 2, $object) }; }
macro_rules! Deposit_sexponentmantissa { ($object:expr, $value:expr) => { Bitfield_deposit($value, 1, 31, $object) }; }
macro_rules! Deposit_ssignexponent { ($object:expr, $value:expr) => { Bitfield_deposit($value, 0, 9, $object) }; }
macro_rules! Deposit_slow { ($object:expr, $value:expr) => { Bitfield_deposit($value, 31, 1, $object) }; }
macro_rules! Deposit_shigh4 { ($object:expr, $value:expr) => { Bitfield_deposit($value, 0, 4, $object) }; }
macro_rules! Is_ssign { ($object:expr) => { Bitfield_mask(0, 1, $object) }; }
macro_rules! Is_ssignaling { ($object:expr) => { Bitfield_mask(9, 1, $object) }; }
macro_rules! Is_shidden { ($object:expr) => { Bitfield_mask(8, 1, $object) }; }
macro_rules! Is_shiddenoverflow { ($object:expr) => { Bitfield_mask(7, 1, $object) }; }
macro_rules! Is_slow { ($object:expr) => { Bitfield_mask(31, 1, $object) }; }
macro_rules! Is_sbit24 { ($object:expr) => { Bitfield_mask(24, 1, $object) }; }
macro_rules! Is_sbit28 { ($object:expr) => { Bitfield_mask(28, 1, $object) }; }
macro_rules! Is_sbit29 { ($object:expr) => { Bitfield_mask(29, 1, $object) }; }
macro_rules! Is_sbit30 { ($object:expr) => { Bitfield_mask(30, 1, $object) }; }
macro_rules! Is_sbit31 { ($object:expr) => { Bitfield_mask(31, 1, $object) }; }

/* Double precision words. */
macro_rules! Dallp1 { ($object:expr) => { $object }; }
macro_rules! Dsign { ($object:expr) => { Bitfield_extract(0, 1, $object) }; }
macro_rules! Dsignedsign { ($object:expr) => { Bitfield_signed_extract(0, 1, $object) }; }
macro_rules! Dexponent { ($object:expr) => { Bitfield_extract(1, 11, $object) }; }
macro_rules! Dmantissap1 { ($object:expr) => { Bitfield_mask(12, 20, $object) }; }
macro_rules! Dsignaling { ($object:expr) => { Bitfield_extract(12, 1, $object) }; }
macro_rules! Dsignalingnan { ($object:expr) => { Bitfield_extract(1, 12, $object) }; }
macro_rules! Dhigh2mantissa { ($object:expr) => { Bitfield_extract(12, 2, $object) }; }
macro_rules! Dexponentmantissap1 { ($object:expr) => { Bitfield_mask(1, 31, $object) }; }
macro_rules! Dsignexponent { ($object:expr) => { Bitfield_extract(0, 12, $object) }; }
macro_rules! Dhidden { ($object:expr) => { Bitfield_extract(11, 1, $object) }; }
macro_rules! Dhiddenoverflow { ($object:expr) => { Bitfield_extract(10, 1, $object) }; }
macro_rules! Dhiddenhigh7mantissa { ($object:expr) => { Bitfield_extract(11, 8, $object) }; }
macro_rules! Dhiddenhigh3mantissa { ($object:expr) => { Bitfield_extract(11, 4, $object) }; }
macro_rules! Dlowp1 { ($object:expr) => { Bitfield_mask(31, 1, $object) }; }
macro_rules! Dlow31p1 { ($object:expr) => { Bitfield_mask(1, 31, $object) }; }
macro_rules! Dhighp1 { ($object:expr) => { Bitfield_extract(0, 1, $object) }; }
macro_rules! Dhigh4p1 { ($object:expr) => { Bitfield_extract(0, 4, $object) }; }
macro_rules! Dhigh31p1 { ($object:expr) => { Bitfield_extract(0, 31, $object) }; }
macro_rules! Dsignedhigh31p1 { ($object:expr) => { Bitfield_signed_extract(0, 31, $object) }; }
macro_rules! Dbit3p1 { ($object:expr) => { Bitfield_extract(3, 1, $object) }; }
macro_rules! Deposit_dsign { ($object:expr, $value:expr) => { Bitfield_deposit($value, 0, 1, $object) }; }
macro_rules! Deposit_dexponent { ($object:expr, $value:expr) => { Bitfield_deposit($value, 1, 11, $object) }; }
macro_rules! Deposit_dmantissap1 { ($object:expr, $value:expr) => { Bitfield_deposit($value, 12, 20, $object) }; }
macro_rules! Deposit_dhigh2mantissa { ($object:expr, $value:expr) => { Bitfield_deposit($value, 12, 2, $object) }; }
macro_rules! Deposit_dexponentmantissap1 { ($object:expr, $value:expr) => { Bitfield_deposit($value, 1, 31, $object) }; }
macro_rules! Deposit_dsignexponent { ($object:expr, $value:expr) => { Bitfield_deposit($value, 0, 12, $object) }; }
macro_rules! Deposit_dlowp1 { ($object:expr, $value:expr) => { Bitfield_deposit($value, 31, 1, $object) }; }
macro_rules! Deposit_dhigh4p1 { ($object:expr, $value:expr) => { Bitfield_deposit($value, 0, 4, $object) }; }
macro_rules! Is_dsign { ($object:expr) => { Bitfield_mask(0, 1, $object) }; }
macro_rules! Is_dsignaling { ($object:expr) => { Bitfield_mask(12, 1, $object) }; }
macro_rules! Is_dhidden { ($object:expr) => { Bitfield_mask(11, 1, $object) }; }
macro_rules! Is_dhiddenoverflow { ($object:expr) => { Bitfield_mask(10, 1, $object) }; }
macro_rules! Is_dlowp1 { ($object:expr) => { Bitfield_mask(31, 1, $object) }; }
macro_rules! Is_dhighp1 { ($object:expr) => { Bitfield_mask(0, 1, $object) }; }
macro_rules! Is_dbit3p1 { ($object:expr) => { Bitfield_mask(3, 1, $object) }; }

macro_rules! Dallp2 { ($object:expr) => { $object }; }
macro_rules! Dmantissap2 { ($object:expr) => { $object }; }
macro_rules! Dlowp2 { ($object:expr) => { Bitfield_mask(31, 1, $object) }; }
macro_rules! Dlow4p2 { ($object:expr) => { Bitfield_mask(28, 4, $object) }; }
macro_rules! Dlow31p2 { ($object:expr) => { Bitfield_mask(1, 31, $object) }; }
macro_rules! Dhighp2 { ($object:expr) => { Bitfield_extract(0, 1, $object) }; }
macro_rules! Dhigh31p2 { ($object:expr) => { Bitfield_extract(0, 31, $object) }; }
macro_rules! Dbit2p2 { ($object:expr) => { Bitfield_extract(2, 1, $object) }; }
macro_rules! Dbit3p2 { ($object:expr) => { Bitfield_extract(3, 1, $object) }; }
macro_rules! Dbit21p2 { ($object:expr) => { Bitfield_extract(21, 1, $object) }; }
macro_rules! Dbit28p2 { ($object:expr) => { Bitfield_extract(28, 1, $object) }; }
macro_rules! Dbit29p2 { ($object:expr) => { Bitfield_extract(29, 1, $object) }; }
macro_rules! Dbit30p2 { ($object:expr) => { Bitfield_extract(30, 1, $object) }; }
macro_rules! Dbit31p2 { ($object:expr) => { Bitfield_mask(31, 1, $object) }; }
macro_rules! Deposit_dlowp2 { ($object:expr, $value:expr) => { Bitfield_deposit($value, 31, 1, $object) }; }
macro_rules! Is_dlowp2 { ($object:expr) => { Bitfield_mask(31, 1, $object) }; }
macro_rules! Is_dhighp2 { ($object:expr) => { Bitfield_mask(0, 1, $object) }; }
macro_rules! Is_dbit2p2 { ($object:expr) => { Bitfield_mask(2, 1, $object) }; }
macro_rules! Is_dbit3p2 { ($object:expr) => { Bitfield_mask(3, 1, $object) }; }
macro_rules! Is_dbit21p2 { ($object:expr) => { Bitfield_mask(21, 1, $object) }; }
macro_rules! Is_dbit28p2 { ($object:expr) => { Bitfield_mask(28, 1, $object) }; }
macro_rules! Is_dbit29p2 { ($object:expr) => { Bitfield_mask(29, 1, $object) }; }
macro_rules! Is_dbit30p2 { ($object:expr) => { Bitfield_mask(30, 1, $object) }; }
macro_rules! Is_dbit31p2 { ($object:expr) => { Bitfield_mask(31, 1, $object) }; }

#[repr(C)]
pub union QuadU1 { pub qallp1: u32 }
#[repr(C)]
pub union QuadU2 { pub qallp2: u32 }
#[repr(C)]
pub union QuadU3 { pub qallp3: u32 }
#[repr(C)]
pub union QuadU4 { pub qallp4: u32 }
#[repr(C)]
pub struct QuadFloatingPoint { pub quad_u1: QuadU1, pub quad_u2: QuadU2, pub quad_u3: QuadU3, pub quad_u4: QuadU4 }

macro_rules! Extall { ($object:expr) => { $object }; }
macro_rules! Extsign { ($object:expr) => { Bitfield_extract(0, 1, $object) }; }
macro_rules! Exthigh31 { ($object:expr) => { Bitfield_extract(0, 31, $object) }; }
macro_rules! Extlow31 { ($object:expr) => { Bitfield_extract(1, 31, $object) }; }
macro_rules! Extlow { ($object:expr) => { Bitfield_extract(31, 1, $object) }; }
macro_rules! Sextallp1 { ($object:expr) => { $object }; }
macro_rules! Sextallp2 { ($object:expr) => { $object }; }
macro_rules! Sextlowp1 { ($object:expr) => { Bitfield_extract(31, 1, $object) }; }
macro_rules! Sexthighp2 { ($object:expr) => { Bitfield_extract(0, 1, $object) }; }
macro_rules! Sextlow31p2 { ($object:expr) => { Bitfield_extract(1, 31, $object) }; }
macro_rules! Sexthiddenoverflow { ($object:expr) => { Bitfield_extract(4, 1, $object) }; }
macro_rules! Is_sexthiddenoverflow { ($object:expr) => { Bitfield_mask(4, 1, $object) }; }
macro_rules! Dextallp1 { ($object:expr) => { $object }; }
macro_rules! Dextallp2 { ($object:expr) => { $object }; }
macro_rules! Dextallp3 { ($object:expr) => { $object }; }
macro_rules! Dextallp4 { ($object:expr) => { $object }; }
macro_rules! Dextlowp2 { ($object:expr) => { Bitfield_extract(31, 1, $object) }; }
macro_rules! Dexthighp3 { ($object:expr) => { Bitfield_extract(0, 1, $object) }; }
macro_rules! Dextlow31p3 { ($object:expr) => { Bitfield_extract(1, 31, $object) }; }
macro_rules! Dexthiddenoverflow { ($object:expr) => { Bitfield_extract(10, 1, $object) }; }
macro_rules! Is_dexthiddenoverflow { ($object:expr) => { Bitfield_mask(10, 1, $object) }; }
macro_rules! Deposit_dextlowp4 { ($object:expr, $value:expr) => { Bitfield_deposit($value, 31, 1, $object) }; }

pub type SglInteger = i32;
#[repr(C)] pub struct Dint { pub wd0: i32, pub wd1: u32 }
#[repr(C)] pub struct Dblwd { pub wd0: u32, pub wd1: u32 }
#[repr(C)] pub struct Quadwd { pub wd0: i32, pub wd1: u32, pub wd2: u32, pub wd3: u32 }
pub type QuadInteger = Quadwd;
pub type SglFloatingPoint = u32;
pub type DblFloatingPoint = Dblwd;
pub type DblInteger = Dint;
pub type DblUnsigned = Dblwd;

pub const SGL_BITLENGTH: i32 = 32; pub const SGL_EMAX: i32 = 127; pub const SGL_EMIN: i32 = -126; pub const SGL_BIAS: i32 = 127; pub const SGL_WRAP: i32 = 192; pub const SGL_INFINITY_EXPONENT: i32 = SGL_EMAX + SGL_BIAS + 1; pub const SGL_THRESHOLD: i32 = 32; pub const SGL_EXP_LENGTH: i32 = 8; pub const SGL_P: i32 = 24;
pub const DBL_BITLENGTH: i32 = 64; pub const DBL_EMAX: i32 = 1023; pub const DBL_EMIN: i32 = -1022; pub const DBL_BIAS: i32 = 1023; pub const DBL_WRAP: i32 = 1536; pub const DBL_INFINITY_EXPONENT: i32 = DBL_EMAX + DBL_BIAS + 1; pub const DBL_THRESHOLD: i32 = 64; pub const DBL_EXP_LENGTH: i32 = 11; pub const DBL_P: i32 = 53;
pub const QUAD_BITLENGTH: i32 = 128; pub const QUAD_EMAX: i32 = 16383; pub const QUAD_EMIN: i32 = -16382; pub const QUAD_BIAS: i32 = 16383; pub const QUAD_WRAP: i32 = 24576; pub const QUAD_INFINITY_EXPONENT: i32 = QUAD_EMAX + QUAD_BIAS + 1; pub const QUAD_P: i32 = 113;
pub const FALSE: i32 = 0; pub const TRUE: i32 = 1; pub const NULL: i32 = 0; pub const NIL: i32 = 0; pub const SGL: i32 = 0; pub const DBL: i32 = 1; pub const BADFMT: i32 = 2; pub const QUAD: i32 = 3;
pub type Boolean = i32; pub type Format = i32; pub type Void = i32;

macro_rules! Cbit { ($object:expr) => { Bitfield_extract(5, 1, $object) }; }
macro_rules! Tbit { ($object:expr) => { Bitfield_extract(25, 1, $object) }; }
macro_rules! Roundingmode { ($object:expr) => { Bitfield_extract(21, 2, $object) }; }
macro_rules! Invalidtrap { ($object:expr) => { Bitfield_extract(27, 1, $object) }; }
macro_rules! Divisionbyzerotrap { ($object:expr) => { Bitfield_extract(28, 1, $object) }; }
macro_rules! Overflowtrap { ($object:expr) => { Bitfield_extract(29, 1, $object) }; }
macro_rules! Underflowtrap { ($object:expr) => { Bitfield_extract(30, 1, $object) }; }
macro_rules! Inexacttrap { ($object:expr) => { Bitfield_extract(31, 1, $object) }; }
macro_rules! Invalidflag { ($object:expr) => { Bitfield_extract(0, 1, $object) }; }
macro_rules! Divisionbyzeroflag { ($object:expr) => { Bitfield_extract(1, 1, $object) }; }
macro_rules! Overflowflag { ($object:expr) => { Bitfield_extract(2, 1, $object) }; }
macro_rules! Underflowflag { ($object:expr) => { Bitfield_extract(3, 1, $object) }; }
macro_rules! Inexactflag { ($object:expr) => { Bitfield_extract(4, 1, $object) }; }
macro_rules! Allflags { ($object:expr) => { Bitfield_extract(0, 5, $object) }; }
pub const ROUNDNEAREST: i32 = 0; pub const ROUNDZERO: i32 = 1; pub const ROUNDPLUS: i32 = 2; pub const ROUNDMINUS: i32 = 3;
pub const NOEXCEPTION: i32 = 0x0; pub const INVALIDEXCEPTION: i32 = 0x20; pub const DIVISIONBYZEROEXCEPTION: i32 = 0x10; pub const OVERFLOWEXCEPTION: i32 = 0x08; pub const UNDERFLOWEXCEPTION: i32 = 0x04; pub const INEXACTEXCEPTION: i32 = 0x02; pub const UNIMPLEMENTEDEXCEPTION: i32 = 0x01;
pub const OPC_2E_INVALIDEXCEPTION: i32 = 0x30; pub const OPC_2E_OVERFLOWEXCEPTION: i32 = 0x18; pub const OPC_2E_UNDERFLOWEXCEPTION: i32 = 0x0c; pub const OPC_2E_INEXACTEXCEPTION: i32 = 0x12;
macro_rules! Allexception { ($object:expr) => { $object }; }
macro_rules! Exceptiontype { ($object:expr) => { Bitfield_extract(0, 6, $object) }; }
macro_rules! Instructionfield { ($object:expr) => { Bitfield_mask(6, 26, $object) }; }
macro_rules! Parmfield { ($object:expr) => { Bitfield_extract(23, 3, $object) }; }
macro_rules! Rabit { ($object:expr) => { Bitfield_extract(24, 1, $object) }; }
macro_rules! Ibit { ($object:expr) => { Bitfield_extract(25, 1, $object) }; }
macro_rules! Set_exceptiontype { ($object:expr, $value:expr) => { Bitfield_deposit($value, 0, 6, $object) }; }
macro_rules! Set_parmfield { ($object:expr, $value:expr) => { Bitfield_deposit($value, 23, 3, $object) }; }
macro_rules! Set_exceptiontype_and_instr_field { ($exception:expr, $instruction:expr, $object:expr) => { $object = ($exception << 26) | $instruction }; }
macro_rules! Greaterthanbit { ($object:expr) => { Bitfield_extract(27, 1, $object) }; }
macro_rules! Lessthanbit { ($object:expr) => { Bitfield_extract(28, 1, $object) }; }
macro_rules! Equalbit { ($object:expr) => { Bitfield_extract(29, 1, $object) }; }
macro_rules! Unorderedbit { ($object:expr) => { Bitfield_extract(30, 1, $object) }; }
macro_rules! Exceptionbit { ($object:expr) => { Bitfield_extract(31, 1, $object) }; }

/* The C header's Fpustatus_register is the externally supplied `*status`. */
macro_rules! Fpustatus_register { () => { *status }; }
macro_rules! Rounding_mode { () => { Roundingmode!(Fpustatus_register!()) }; }
macro_rules! Is_rounding_mode { ($rmode:expr) => { Roundingmode!(Fpustatus_register!()) == $rmode }; }
macro_rules! Set_rounding_mode { ($value:expr) => { Bitfield_deposit($value, 21, 2, Fpustatus_register!()) }; }
macro_rules! Is_invalidtrap_enabled { () => { Invalidtrap!(Fpustatus_register!()) }; }
macro_rules! Is_divisionbyzerotrap_enabled { () => { Divisionbyzerotrap!(Fpustatus_register!()) }; }
macro_rules! Is_overflowtrap_enabled { () => { Overflowtrap!(Fpustatus_register!()) }; }
macro_rules! Is_underflowtrap_enabled { () => { Underflowtrap!(Fpustatus_register!()) }; }
macro_rules! Is_inexacttrap_enabled { () => { Inexacttrap!(Fpustatus_register!()) }; }
macro_rules! Set_invalidflag { () => { Bitfield_deposit(1, 0, 1, Fpustatus_register!()) }; }
macro_rules! Set_divisionbyzeroflag { () => { Bitfield_deposit(1, 1, 1, Fpustatus_register!()) }; }
macro_rules! Set_overflowflag { () => { Bitfield_deposit(1, 2, 1, Fpustatus_register!()) }; }
macro_rules! Set_underflowflag { () => { Bitfield_deposit(1, 3, 1, Fpustatus_register!()) }; }
macro_rules! Set_inexactflag { () => { Bitfield_deposit(1, 4, 1, Fpustatus_register!()) }; }
macro_rules! Clear_all_flags { () => { Bitfield_deposit(0, 0, 5, Fpustatus_register!()) }; }
macro_rules! Set_tbit { () => { Bitfield_deposit(1, 25, 1, Fpustatus_register!()) }; }
macro_rules! Clear_tbit { () => { Bitfield_deposit(0, 25, 1, Fpustatus_register!()) }; }
macro_rules! Is_tbit_set { () => { Tbit!(Fpustatus_register!()) }; }
macro_rules! Is_cbit_set { () => { Cbit!(Fpustatus_register!()) }; }
macro_rules! Set_status_cbit { ($value:expr) => { Bitfield_deposit($value, 5, 1, Fpustatus_register!()) }; }

macro_rules! Unordered { ($cond:expr) => { Unorderedbit($cond) }; }
macro_rules! Equal { ($cond:expr) => { Equalbit($cond) }; }
macro_rules! Lessthan { ($cond:expr) => { Lessthanbit($cond) }; }
macro_rules! Greaterthan { ($cond:expr) => { Greaterthanbit($cond) }; }
macro_rules! Exception { ($cond:expr) => { Exceptionbit($cond) }; }
macro_rules! Ext_isone_sign { ($extent:expr) => { Extsign($extent) }; }
macro_rules! Ext_isnotzero { ($extent:expr) => { Extall($extent) }; }
macro_rules! Ext_isnotzero_lower { ($extent:expr) => { Extlow31($extent) }; }
macro_rules! Ext_leftshiftby1 { ($extent:expr) => { $extent <<= 1 }; }
macro_rules! Ext_negate { ($extent:expr) => { $extent = 0i32.wrapping_sub($extent as i32) }; }
macro_rules! Ext_setone_low { ($extent:expr) => { Bitfield_deposit(1, 31, 1, $extent) }; }
macro_rules! Ext_setzero { ($extent:expr) => { $extent = 0 }; }

pub type Operation = i32;
pub const NONE: i32 = 0; pub const UNDEFFPINST: i32 = 1;
pub const FTEST: i32 = (1 << 2) | 0; pub const FCPY: i32 = (2 << 2) | 0; pub const FABS: i32 = (3 << 2) | 0; pub const FSQRT: i32 = (4 << 2) | 0; pub const FRND: i32 = (5 << 2) | 0;
pub const FCNVFF: i32 = (0 << 2) | 1; pub const FCNVXF: i32 = (1 << 2) | 1; pub const FCNVFX: i32 = (2 << 2) | 1; pub const FCNVFXT: i32 = (3 << 2) | 1;
pub const FCMP: i32 = (0 << 2) | 2;
pub const FADD: i32 = (0 << 2) | 3; pub const FSUB: i32 = (1 << 2) | 3; pub const FMPY: i32 = (2 << 2) | 3; pub const FDIV: i32 = (3 << 2) | 3; pub const FREM: i32 = (4 << 2) | 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
