/* Translated from cvmx-spxx-defs.h. */

/* CVMX_ADD_IO_SEG is supplied by the surrounding SDK. */
#[inline]
pub const fn cvmx_spxx_bckprs_cnt(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180090000340u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub const fn cvmx_spxx_bist_stat(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800900007F8u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub const fn cvmx_spxx_clk_ctl(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180090000348u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub const fn cvmx_spxx_clk_stat(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180090000350u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub const fn cvmx_spxx_dbg_deskew_ctl(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180090000368u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub const fn cvmx_spxx_dbg_deskew_state(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180090000370u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub const fn cvmx_spxx_drv_ctl(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180090000358u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub const fn cvmx_spxx_err_ctl(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180090000320u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub const fn cvmx_spxx_int_dat(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180090000318u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub const fn cvmx_spxx_int_msk(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180090000308u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub const fn cvmx_spxx_int_reg(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180090000300u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub const fn cvmx_spxx_int_sync(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180090000310u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub const fn cvmx_spxx_tpa_acc(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180090000338u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub const fn cvmx_spxx_tpa_max(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180090000330u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub const fn cvmx_spxx_tpa_sel(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180090000328u64) + (block_id & 1) * 0x8000000u64 }
#[inline]
pub const fn cvmx_spxx_trn4_ctl(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001180090000360u64) + (block_id & 1) * 0x8000000u64 }

extern "C" { pub fn __cvmx_interrupt_spxx_int_msk_enable(index: i32); }

/* C bitfields are represented by their containing 64-bit register.  The
 * following masks and shifts preserve the declared field layout and endian
 * conditional intent; callers may use the raw register value directly. */
/* Rust has no native C bitfield syntax; each declaration remains a 64-bit
 * register union, with the original field names, widths, and positions below. */
#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_spxx_bckprs_cnt { pub u64: u64, pub s: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_spxx_bist_stat { pub u64: u64, pub s: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_spxx_clk_ctl { pub u64: u64, pub s: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_spxx_clk_stat { pub u64: u64, pub s: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_spxx_dbg_deskew_ctl { pub u64: u64, pub s: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_spxx_dbg_deskew_state { pub u64: u64, pub s: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_spxx_drv_ctl { pub u64: u64, pub s: u64, pub cn38xx: u64, pub cn58xx: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_spxx_err_ctl { pub u64: u64, pub s: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_spxx_int_dat { pub u64: u64, pub s: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_spxx_int_msk { pub u64: u64, pub s: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_spxx_int_reg { pub u64: u64, pub s: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_spxx_int_sync { pub u64: u64, pub s: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_spxx_tpa_acc { pub u64: u64, pub s: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_spxx_tpa_max { pub u64: u64, pub s: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_spxx_tpa_sel { pub u64: u64, pub s: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_spxx_trn4_ctl { pub u64: u64, pub s: u64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
