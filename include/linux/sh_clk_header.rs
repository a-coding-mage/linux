/* SPDX-License-Identifier: GPL-2.0 */
// C dependencies supplied by other translated units are intentionally external.

use core::ffi::c_void;

pub type PhysAddr = usize;
pub type CInt = i32;
pub type CLong = isize;

#[repr(C)]
pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)] pub struct Kref { pub refcount: CInt }
#[repr(C)] pub struct CpufreqFrequencyTable { _private: [u8; 0] }

#[repr(C)]
pub struct ClkMapping {
    pub phys: PhysAddr,
    pub base: *mut c_void,
    pub len: usize,
    pub ref_: Kref,
}

#[repr(C)]
pub struct ClkOps {
    #[cfg(feature = "CONFIG_SH_CLK_CPG_LEGACY")]
    pub init: Option<unsafe extern "C" fn(*mut Clk)>,
    pub enable: Option<unsafe extern "C" fn(*mut Clk) -> CInt>,
    pub disable: Option<unsafe extern "C" fn(*mut Clk)>,
    pub recalc: Option<unsafe extern "C" fn(*mut Clk) -> usize>,
    pub set_rate: Option<unsafe extern "C" fn(*mut Clk, usize) -> CInt>,
    pub set_parent: Option<unsafe extern "C" fn(*mut Clk, *mut Clk) -> CInt>,
    pub round_rate: Option<unsafe extern "C" fn(*mut Clk, usize) -> CLong>,
}

#[inline] pub const fn sh_clk_div_msk(div: u32) -> u32 { (1u32 << div).wrapping_sub(1) }
pub const SH_CLK_DIV4_MSK: u32 = sh_clk_div_msk(4);
pub const SH_CLK_DIV6_MSK: u32 = sh_clk_div_msk(6);

#[repr(C)]
pub struct Clk {
    pub node: ListHead,
    pub parent: *mut Clk,
    pub parent_table: *mut *mut Clk,
    pub parent_num: u16,
    pub src_shift: u8,
    pub src_width: u8,
    pub ops: *mut ClkOps,
    pub children: ListHead,
    pub sibling: ListHead,
    pub usecount: CInt,
    pub rate: usize,
    pub flags: usize,
    pub enable_reg: *mut c_void,
    pub status_reg: *mut c_void,
    pub enable_bit: u32,
    pub mapped_reg: *mut c_void,
    pub div_mask: u32,
    pub arch_flags: usize,
    pub priv_: *mut c_void,
    pub mapping: *mut ClkMapping,
    pub freq_table: *mut CpufreqFrequencyTable,
    pub nr_freqs: u32,
}

pub const CLK_ENABLE_ON_INIT: usize = 1 << 0;
pub const CLK_ENABLE_REG_32BIT: usize = 1 << 1;
pub const CLK_ENABLE_REG_16BIT: usize = 1 << 2;
pub const CLK_ENABLE_REG_8BIT: usize = 1 << 3;
pub const CLK_MASK_DIV_ON_DISABLE: usize = 1 << 4;
pub const CLK_ENABLE_REG_MASK: usize = CLK_ENABLE_REG_32BIT | CLK_ENABLE_REG_16BIT | CLK_ENABLE_REG_8BIT;

unsafe extern "C" {
    pub fn followparent_recalc(clk: *mut Clk) -> usize;
    pub fn recalculate_root_clocks();
    pub fn propagate_rate(clk: *mut Clk);
    pub fn clk_reparent(child: *mut Clk, parent: *mut Clk) -> CInt;
    pub fn clk_register(clk: *mut Clk) -> CInt;
    pub fn clk_unregister(clk: *mut Clk);
    pub fn clk_enable_init_clocks();
}

#[repr(C)] pub struct ClkDivMultTable { pub divisors: *mut u32, pub nr_divisors: u32, pub multipliers: *mut u32, pub nr_multipliers: u32 }
unsafe extern "C" {
    pub fn clk_rate_table_build(clk: *mut Clk, freq_table: *mut CpufreqFrequencyTable, nr_freqs: CInt, src_table: *mut ClkDivMultTable, bitmap: *mut usize);
    pub fn clk_rate_table_round(clk: *mut Clk, freq_table: *mut CpufreqFrequencyTable, rate: usize) -> CLong;
    pub fn clk_rate_table_find(clk: *mut Clk, freq_table: *mut CpufreqFrequencyTable, rate: usize) -> CInt;
    pub fn clk_rate_div_range_round(clk: *mut Clk, div_min: u32, div_max: u32, rate: usize) -> CLong;
    pub fn clk_rate_mult_range_round(clk: *mut Clk, mult_min: u32, mult_max: u32, rate: usize) -> CLong;
    pub fn sh_clk_mstp_register(clks: *mut Clk, nr: CInt) -> CInt;
}

#[macro_export] macro_rules! SH_CLK_MSTP { ($p:expr, $r:expr, $b:expr, $s:expr, $f:expr) => { Clk { parent: $p, enable_reg: $r as *mut c_void, enable_bit: $b, status_reg: $s, flags: $f, ..unsafe { core::mem::zeroed() } } }; }
#[macro_export] macro_rules! SH_CLK_MSTP32 { ($p:expr,$r:expr,$b:expr,$f:expr) => { SH_CLK_MSTP!($p,$r,$b,core::ptr::null_mut(),$f | CLK_ENABLE_REG_32BIT) }; }
#[macro_export] macro_rules! SH_CLK_MSTP32_STS { ($p:expr,$r:expr,$b:expr,$s:expr,$f:expr) => { SH_CLK_MSTP!($p,$r,$b,$s,$f | CLK_ENABLE_REG_32BIT) }; }
#[macro_export] macro_rules! SH_CLK_MSTP16 { ($p:expr,$r:expr,$b:expr,$f:expr) => { SH_CLK_MSTP!($p,$r,$b,core::ptr::null_mut(),$f | CLK_ENABLE_REG_16BIT) }; }
#[macro_export] macro_rules! SH_CLK_MSTP8 { ($p:expr,$r:expr,$b:expr,$f:expr) => { SH_CLK_MSTP!($p,$r,$b,core::ptr::null_mut(),$f | CLK_ENABLE_REG_8BIT) }; }

// MSTP registration never really cared about access size; clock definitions provide it.
#[inline] pub unsafe fn sh_clk_mstp32_register(clks: *mut Clk, nr: CInt) -> CInt { sh_clk_mstp_register(clks, nr) }

#[repr(C)] pub struct ClkDivTable { pub div_mult_table: *mut ClkDivMultTable, pub kick: Option<unsafe extern "C" fn(*mut Clk)> }
pub type ClkDiv4Table = ClkDivTable;
unsafe extern "C" { pub fn sh_clk_div4_register(clks: *mut Clk, nr: CInt, table: *mut ClkDiv4Table) -> CInt; pub fn sh_clk_div4_enable_register(clks: *mut Clk, nr: CInt, table: *mut ClkDiv4Table) -> CInt; pub fn sh_clk_div4_reparent_register(clks: *mut Clk, nr: CInt, table: *mut ClkDiv4Table) -> CInt; pub fn sh_clk_div6_register(clks: *mut Clk, nr: CInt) -> CInt; pub fn sh_clk_div6_reparent_register(clks: *mut Clk, nr: CInt) -> CInt; pub fn sh_clk_fsidiv_register(clks: *mut Clk, nr: CInt) -> CInt; }

#[macro_export] macro_rules! SH_CLK_DIV4 { ($p:expr,$r:expr,$s:expr,$bm:expr,$f:expr) => { Clk { parent:$p, enable_reg:$r as *mut c_void, enable_bit:$s, arch_flags:$bm, div_mask:SH_CLK_DIV4_MSK, flags:$f, ..unsafe { core::mem::zeroed() } } }; }
#[macro_export] macro_rules! SH_CLK_DIV6 { ($p:expr,$r:expr,$f:expr) => { Clk { parent:$p, enable_reg:$r as *mut c_void, div_mask:SH_CLK_DIV6_MSK, flags:$f | CLK_MASK_DIV_ON_DISABLE, ..unsafe { core::mem::zeroed() } } }; }
#[macro_export] macro_rules! SH_CLK_DIV6_EXT { ($r:expr,$f:expr,$parents:expr,$num:expr,$shift:expr,$width:expr) => { Clk { enable_reg:$r as *mut c_void, parent_table:$parents, parent_num:$num, src_shift:$shift, src_width:$width, div_mask:SH_CLK_DIV6_MSK, flags:$f | CLK_MASK_DIV_ON_DISABLE, ..unsafe { core::mem::zeroed() } } }; }
#[repr(C)] pub struct ClkLookup { pub con_id: *const core::ffi::c_char, pub dev_id: *const core::ffi::c_char, pub clk: *mut Clk }
#[macro_export] macro_rules! CLKDEV_CON_ID { ($id:expr,$clk:expr) => { ClkLookup { con_id:$id, dev_id:core::ptr::null(), clk:$clk } }; }
#[macro_export] macro_rules! CLKDEV_DEV_ID { ($id:expr,$clk:expr) => { ClkLookup { con_id:core::ptr::null(), dev_id:$id, clk:$clk } }; }
#[macro_export] macro_rules! CLKDEV_ICK_ID { ($cid:expr,$did:expr,$clk:expr) => { ClkLookup { con_id:$cid, dev_id:$did, clk:$clk } }; }
#[macro_export] macro_rules! SH_CLK_FSIDIV { ($r:expr,$p:expr) => { Clk { enable_reg:$r as *mut c_void, parent:$p, ..unsafe { core::mem::zeroed() } } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
