/* Translated from cvmx-rst-defs.h. */

pub const CVMX_RST_BOOT: u64 = cvmx_add_io_seg(0x0001180006001600u64);
pub const CVMX_RST_CFG: u64 = cvmx_add_io_seg(0x0001180006001610u64);
pub const CVMX_RST_CKILL: u64 = cvmx_add_io_seg(0x0001180006001638u64);
#[inline] pub const fn CVMX_RST_CTLX(offset: u64) -> u64 { cvmx_add_io_seg(0x0001180006001640u64) + (offset & 3) * 8 }
pub const CVMX_RST_DELAY: u64 = cvmx_add_io_seg(0x0001180006001608u64);
pub const CVMX_RST_ECO: u64 = cvmx_add_io_seg(0x00011800060017B8u64);
pub const CVMX_RST_INT: u64 = cvmx_add_io_seg(0x0001180006001628u64);
pub const CVMX_RST_OCX: u64 = cvmx_add_io_seg(0x0001180006001618u64);
pub const CVMX_RST_POWER_DBG: u64 = cvmx_add_io_seg(0x0001180006001708u64);
pub const CVMX_RST_PP_POWER: u64 = cvmx_add_io_seg(0x0001180006001700u64);
#[inline] pub const fn CVMX_RST_SOFT_PRSTX(offset: u64) -> u64 { cvmx_add_io_seg(0x00011800060016C0u64) + (offset & 3) * 8 }
pub const CVMX_RST_SOFT_RST: u64 = cvmx_add_io_seg(0x0001180006001680u64);

extern "C" { fn cvmx_add_io_seg(value: u64) -> u64; }

/* C bitfields are represented by their containing register.  Accessors use
 * the little-endian bit numbering used by the original non-big-endian branch.
 * Define CVMX_BIG_ENDIAN_BITFIELD to select the corresponding reversed layout. */
macro_rules! reg_union { ($u:ident, $s:ident) => {
    #[repr(C)] pub union $u { pub u64: u64, pub s: $s }
}; }
macro_rules! reg_struct { ($s:ident) => { #[repr(C)] #[derive(Copy, Clone)] pub struct $s { pub bits: u64 } }; }

reg_struct!(cvmx_rst_boot_s); reg_union!(cvmx_rst_boot, cvmx_rst_boot_s);
reg_struct!(cvmx_rst_cfg_s); reg_union!(cvmx_rst_cfg, cvmx_rst_cfg_s);
reg_struct!(cvmx_rst_ckill_s); reg_union!(cvmx_rst_ckill, cvmx_rst_ckill_s);
reg_struct!(cvmx_rst_ctlx_s); reg_union!(cvmx_rst_ctlx, cvmx_rst_ctlx_s);
reg_struct!(cvmx_rst_delay_s); reg_union!(cvmx_rst_delay, cvmx_rst_delay_s);
reg_struct!(cvmx_rst_eco_s); reg_union!(cvmx_rst_eco, cvmx_rst_eco_s);
reg_struct!(cvmx_rst_int_s);
#[repr(C)] pub union cvmx_rst_int { pub u64: u64, pub s: cvmx_rst_int_s, pub cn70xx: cvmx_rst_int_cn70xx }
reg_struct!(cvmx_rst_ocx_s); reg_union!(cvmx_rst_ocx, cvmx_rst_ocx_s);
reg_struct!(cvmx_rst_power_dbg_s); reg_union!(cvmx_rst_power_dbg, cvmx_rst_power_dbg_s);
reg_struct!(cvmx_rst_pp_power_s);
#[repr(C)] pub union cvmx_rst_pp_power { pub u64: u64, pub s: cvmx_rst_pp_power_s, pub cn70xx: cvmx_rst_pp_power_cn70xx }
reg_struct!(cvmx_rst_soft_prstx_s); reg_union!(cvmx_rst_soft_prstx, cvmx_rst_soft_prstx_s);
reg_struct!(cvmx_rst_soft_rst_s); reg_union!(cvmx_rst_soft_rst, cvmx_rst_soft_rst_s);

#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_rst_int_cn70xx { pub bits: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_rst_pp_power_cn70xx { pub bits: u64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
