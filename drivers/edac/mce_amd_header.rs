/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency intent: linux/notifier.h and asm/mce.h provide the referenced
 * notifier and machine-check declarations. */

#[inline]
pub const fn ec(x: u64) -> u64 { x & 0xffff }

#[inline]
pub const fn low_syndrome(x: u64) -> u64 { (x >> 15) & 0xff }

#[inline]
pub const fn high_syndrome(x: u64) -> u64 { (x >> 24) & 0xff }

#[inline]
pub const fn tlb_error(x: u64) -> bool { (x & 0xfff0) == 0x0010 }

#[inline]
pub const fn mem_error(x: u64) -> bool { (x & 0xff00) == 0x0100 }

#[inline]
pub const fn bus_error(x: u64) -> bool { (x & 0xf800) == 0x0800 }

#[inline]
pub const fn int_error(x: u64) -> bool { (x & 0xf4ff) == 0x0400 }

#[inline]
pub const fn tt(x: u64) -> usize { ((x >> 2) & 0x3) as usize }

#[macro_export]
macro_rules! tt_msg { ($x:expr) => { tt_msgs[tt($x)] }; }

#[inline]
pub const fn ii(x: u64) -> usize { ((x >> 2) & 0x3) as usize }

#[macro_export]
macro_rules! ii_msg { ($x:expr) => { ii_msgs[ii($x)] }; }

#[inline]
pub const fn ll(x: u64) -> usize { (x & 0x3) as usize }

#[macro_export]
macro_rules! ll_msg { ($x:expr) => { ll_msgs[ll($x)] }; }

#[inline]
pub const fn to(x: u64) -> usize { ((x >> 8) & 0x1) as usize }

#[macro_export]
macro_rules! to_msg { ($x:expr) => { to_msgs[to($x)] }; }

#[inline]
pub const fn pp(x: u64) -> usize { ((x >> 9) & 0x3) as usize }

#[macro_export]
macro_rules! pp_msg { ($x:expr) => { pp_msgs[pp($x)] }; }

#[inline]
pub const fn uu(x: u64) -> usize { ((x >> 8) & 0x3) as usize }

#[macro_export]
macro_rules! uu_msg { ($x:expr) => { uu_msgs[uu($x)] }; }

#[inline]
pub const fn r4(x: u64) -> usize { ((x >> 4) & 0xf) as usize }

#[macro_export]
macro_rules! r4_msg {
    ($x:expr) => {{
        let n = r4($x);
        if n < 9 { rrrr_msgs[n] } else { "Wrong R4!" }
    }};
}

extern "C" {
    pub static pp_msgs: *const *const core::ffi::c_char;
}

#[repr(u32)]
pub enum tt_ids {
    TT_INSTR = 0,
    TT_DATA,
    TT_GEN,
    TT_RESV,
}

#[repr(u32)]
pub enum ll_ids {
    LL_RESV = 0,
    LL_L1,
    LL_L2,
    LL_LG,
}

#[repr(u32)]
pub enum ii_ids {
    II_MEM = 0,
    II_RESV,
    II_IO,
    II_GEN,
}

#[repr(u32)]
pub enum rrrr_ids {
    R4_GEN = 0,
    R4_RD,
    R4_WR,
    R4_DRD,
    R4_DWR,
    R4_IRD,
    R4_PREF,
    R4_EVICT,
    R4_SNOOP,
}

/* per-family decoder ops */
#[repr(C)]
pub struct amd_decoder_ops {
    pub mc0_mce: Option<unsafe extern "C" fn(u16, u8) -> bool>,
    pub mc1_mce: Option<unsafe extern "C" fn(u16, u8) -> bool>,
    pub mc2_mce: Option<unsafe extern "C" fn(u16, u8) -> bool>,
}

#[repr(C)]
pub struct mce;

extern "C" {
    pub fn amd_register_ecc_decoder(f: Option<unsafe extern "C" fn(i32, *mut mce)>);
    pub fn amd_unregister_ecc_decoder(f: Option<unsafe extern "C" fn(i32, *mut mce)>);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
