/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from alpha/include/asm/core_t2.h. */
/* Dependencies supplied by the surrounding kernel translation are intentionally external. */

pub const T2_ONE_HAE_WINDOW: usize = 1;
pub const T2_MEM_R1_MASK: usize = 0x07ffffff;
pub const _GAMMA_BIAS: usize = 0x8000000000;
pub const GAMMA_BIAS: usize = _GAMMA_BIAS;

pub const T2_CONF: usize = IDENT_ADDR + GAMMA_BIAS + 0x390000000;
pub const T2_IO: usize = IDENT_ADDR + GAMMA_BIAS + 0x3a0000000;
pub const T2_SPARSE_MEM: usize = IDENT_ADDR + GAMMA_BIAS + 0x200000000;
pub const T2_DENSE_MEM: usize = IDENT_ADDR + GAMMA_BIAS + 0x3c0000000;
pub const T2_IOCSR: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000000;
pub const T2_CERR1: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000020;
pub const T2_CERR2: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000040;
pub const T2_CERR3: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000060;
pub const T2_PERR1: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000080;
pub const T2_PERR2: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e0000a0;
pub const T2_PSCR: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e0000c0;
pub const T2_HAE_1: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e0000e0;
pub const T2_HAE_2: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000100;
pub const T2_HBASE: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000120;
pub const T2_WBASE1: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000140;
pub const T2_WMASK1: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000160;
pub const T2_TBASE1: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000180;
pub const T2_WBASE2: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e0001a0;
pub const T2_WMASK2: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e0001c0;
pub const T2_TBASE2: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e0001e0;
pub const T2_TLBBR: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000200;
pub const T2_IVR: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000220;
pub const T2_HAE_3: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000240;
pub const T2_HAE_4: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000260;
pub const T2_WBASE3: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000280;
pub const T2_WMASK3: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e0002a0;
pub const T2_TBASE3: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e0002c0;
pub const T2_TDR0: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000300;
pub const T2_TDR1: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000320;
pub const T2_TDR2: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000340;
pub const T2_TDR3: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000360;
pub const T2_TDR4: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000380;
pub const T2_TDR5: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e0003a0;
pub const T2_TDR6: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e0003c0;
pub const T2_TDR7: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e0003e0;
pub const T2_WBASE4: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000400;
pub const T2_WMASK4: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000420;
pub const T2_TBASE4: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000440;
pub const T2_AIR: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000460;
pub const T2_VAR: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e000480;
pub const T2_DIR: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e0004a0;
pub const T2_ICE: usize = IDENT_ADDR + GAMMA_BIAS + 0x38e0004c0;

pub const T2_CPU0_BASE: usize = IDENT_ADDR + GAMMA_BIAS + 0x380000000;
pub const T2_CPU1_BASE: usize = IDENT_ADDR + GAMMA_BIAS + 0x381000000;
pub const T2_CPU2_BASE: usize = IDENT_ADDR + GAMMA_BIAS + 0x382000000;
pub const T2_CPU3_BASE: usize = IDENT_ADDR + GAMMA_BIAS + 0x383000000;
#[inline] pub const fn T2_CPUn_BASE(n: usize) -> usize { T2_CPU0_BASE + ((n & 3) * 0x001000000) }
pub const T2_MEM0_BASE: usize = IDENT_ADDR + GAMMA_BIAS + 0x388000000;
pub const T2_MEM1_BASE: usize = IDENT_ADDR + GAMMA_BIAS + 0x389000000;
pub const T2_MEM2_BASE: usize = IDENT_ADDR + GAMMA_BIAS + 0x38a000000;
pub const T2_MEM3_BASE: usize = IDENT_ADDR + GAMMA_BIAS + 0x38b000000;

#[repr(C)]
pub struct sable_cpu_csr {
    pub bcc: ::kernel::ffi::c_ulong, pub fill_00: [::kernel::ffi::c_long; 3],
    pub bcce: ::kernel::ffi::c_ulong, pub fill_01: [::kernel::ffi::c_long; 3],
    pub bccea: ::kernel::ffi::c_ulong, pub fill_02: [::kernel::ffi::c_long; 3],
    pub bcue: ::kernel::ffi::c_ulong, pub fill_03: [::kernel::ffi::c_long; 3],
    pub bcuea: ::kernel::ffi::c_ulong, pub fill_04: [::kernel::ffi::c_long; 3],
    pub dter: ::kernel::ffi::c_ulong, pub fill_05: [::kernel::ffi::c_long; 3],
    pub cbctl: ::kernel::ffi::c_ulong, pub fill_06: [::kernel::ffi::c_long; 3],
    pub cbe: ::kernel::ffi::c_ulong, pub fill_07: [::kernel::ffi::c_long; 3],
    pub cbeal: ::kernel::ffi::c_ulong, pub fill_08: [::kernel::ffi::c_long; 3],
    pub cbeah: ::kernel::ffi::c_ulong, pub fill_09: [::kernel::ffi::c_long; 3],
    pub pmbx: ::kernel::ffi::c_ulong, pub fill_10: [::kernel::ffi::c_long; 3],
    pub ipir: ::kernel::ffi::c_ulong, pub fill_11: [::kernel::ffi::c_long; 3],
    pub sic: ::kernel::ffi::c_ulong, pub fill_12: [::kernel::ffi::c_long; 3],
    pub adlk: ::kernel::ffi::c_ulong, pub fill_13: [::kernel::ffi::c_long; 3],
    pub madrl: ::kernel::ffi::c_ulong, pub fill_14: [::kernel::ffi::c_long; 3],
    pub rev: ::kernel::ffi::c_ulong, pub fill_15: [::kernel::ffi::c_long; 3],
}

#[repr(C)] pub struct el_t2_frame_header { pub elcf_fid: u32, pub elcf_size: u32 }
#[repr(C)] pub struct el_t2_procdata_mcheck {
    pub elfmc_paltemp: [::kernel::ffi::c_ulong; 32], pub elfmc_exc_addr: ::kernel::ffi::c_ulong,
    pub elfmc_exc_sum: ::kernel::ffi::c_ulong, pub elfmc_exc_mask: ::kernel::ffi::c_ulong,
    pub elfmc_iccsr: ::kernel::ffi::c_ulong, pub elfmc_pal_base: ::kernel::ffi::c_ulong,
    pub elfmc_hier: ::kernel::ffi::c_ulong, pub elfmc_hirr: ::kernel::ffi::c_ulong,
    pub elfmc_mm_csr: ::kernel::ffi::c_ulong, pub elfmc_dc_stat: ::kernel::ffi::c_ulong,
    pub elfmc_dc_addr: ::kernel::ffi::c_ulong, pub elfmc_abox_ctl: ::kernel::ffi::c_ulong,
    pub elfmc_biu_stat: ::kernel::ffi::c_ulong, pub elfmc_biu_addr: ::kernel::ffi::c_ulong,
    pub elfmc_biu_ctl: ::kernel::ffi::c_ulong, pub elfmc_fill_syndrome: ::kernel::ffi::c_ulong,
    pub elfmc_fill_addr: ::kernel::ffi::c_ulong, pub elfmc_va: ::kernel::ffi::c_ulong,
    pub elfmc_bc_tag: ::kernel::ffi::c_ulong,
}
#[repr(C)] pub struct el_t2_logout_header {
    pub elfl_size: u32, /* elfl_sbz1:31 and elfl_retry:1 share this storage word. */
    pub elfl_sbz1: u32, pub elfl_retry: u32, pub elfl_procoffset: u32,
    pub elfl_sysoffset: u32, pub elfl_error_type: u32, pub elfl_frame_rev: u32,
}
#[repr(C)] pub struct el_t2_sysdata_mcheck { pub elcmc_bcc: ::kernel::ffi::c_ulong, pub elcmc_bcce: ::kernel::ffi::c_ulong, pub elcmc_bccea: ::kernel::ffi::c_ulong, pub elcmc_bcue: ::kernel::ffi::c_ulong, pub elcmc_bcuea: ::kernel::ffi::c_ulong, pub elcmc_dter: ::kernel::ffi::c_ulong, pub elcmc_cbctl: ::kernel::ffi::c_ulong, pub elcmc_cbe: ::kernel::ffi::c_ulong, pub elcmc_cbeal: ::kernel::ffi::c_ulong, pub elcmc_cbeah: ::kernel::ffi::c_ulong, pub elcmc_pmbx: ::kernel::ffi::c_ulong, pub elcmc_ipir: ::kernel::ffi::c_ulong, pub elcmc_sic: ::kernel::ffi::c_ulong, pub elcmc_adlk: ::kernel::ffi::c_ulong, pub elcmc_madrl: ::kernel::ffi::c_ulong, pub elcmc_crrev4: ::kernel::ffi::c_ulong }
#[repr(C)] pub struct el_t2_data_memory { pub elcm_hdr: el_t2_frame_header, pub elcm_module: u32, pub elcm_res04: u32, pub elcm_merr: ::kernel::ffi::c_ulong, pub elcm_mcmd1: ::kernel::ffi::c_ulong, pub elcm_mcmd2: ::kernel::ffi::c_ulong, pub elcm_mconf: ::kernel::ffi::c_ulong, pub elcm_medc1: ::kernel::ffi::c_ulong, pub elcm_medc2: ::kernel::ffi::c_ulong, pub elcm_medcc: ::kernel::ffi::c_ulong, pub elcm_msctl: ::kernel::ffi::c_ulong, pub elcm_mref: ::kernel::ffi::c_ulong, pub elcm_filter: ::kernel::ffi::c_ulong }
#[repr(C)] pub struct el_t2_data_other_cpu { pub elco_cpuid: i16, pub elco_res02: [i16; 3], pub elco_bcc: ::kernel::ffi::c_ulong, pub elco_bcce: ::kernel::ffi::c_ulong, pub elco_bccea: ::kernel::ffi::c_ulong, pub elco_bcue: ::kernel::ffi::c_ulong, pub elco_bcuea: ::kernel::ffi::c_ulong, pub elco_dter: ::kernel::ffi::c_ulong, pub elco_cbctl: ::kernel::ffi::c_ulong, pub elco_cbe: ::kernel::ffi::c_ulong, pub elco_cbeal: ::kernel::ffi::c_ulong, pub elco_cbeah: ::kernel::ffi::c_ulong, pub elco_pmbx: ::kernel::ffi::c_ulong, pub elco_ipir: ::kernel::ffi::c_ulong, pub elco_sic: ::kernel::ffi::c_ulong, pub elco_adlk: ::kernel::ffi::c_ulong, pub elco_madrl: ::kernel::ffi::c_ulong, pub elco_crrev4: ::kernel::ffi::c_ulong }
#[repr(C)] pub struct el_t2_data_t2 { pub elct_hdr: el_t2_frame_header, pub elct_iocsr: ::kernel::ffi::c_ulong, pub elct_cerr1: ::kernel::ffi::c_ulong, pub elct_cerr2: ::kernel::ffi::c_ulong, pub elct_cerr3: ::kernel::ffi::c_ulong, pub elct_perr1: ::kernel::ffi::c_ulong, pub elct_perr2: ::kernel::ffi::c_ulong, pub elct_hae0_1: ::kernel::ffi::c_ulong, pub elct_hae0_2: ::kernel::ffi::c_ulong, pub elct_hbase: ::kernel::ffi::c_ulong, pub elct_wbase1: ::kernel::ffi::c_ulong, pub elct_wmask1: ::kernel::ffi::c_ulong, pub elct_tbase1: ::kernel::ffi::c_ulong, pub elct_wbase2: ::kernel::ffi::c_ulong, pub elct_wmask2: ::kernel::ffi::c_ulong, pub elct_tbase2: ::kernel::ffi::c_ulong, pub elct_tdr0: ::kernel::ffi::c_ulong, pub elct_tdr1: ::kernel::ffi::c_ulong, pub elct_tdr2: ::kernel::ffi::c_ulong, pub elct_tdr3: ::kernel::ffi::c_ulong, pub elct_tdr4: ::kernel::ffi::c_ulong, pub elct_tdr5: ::kernel::ffi::c_ulong, pub elct_tdr6: ::kernel::ffi::c_ulong, pub elct_tdr7: ::kernel::ffi::c_ulong }
#[repr(C)] pub struct el_t2_data_corrected { pub elcpb_biu_stat: ::kernel::ffi::c_ulong, pub elcpb_biu_addr: ::kernel::ffi::c_ulong, pub elcpb_biu_ctl: ::kernel::ffi::c_ulong, pub elcpb_fill_syndrome: ::kernel::ffi::c_ulong, pub elcpb_fill_addr: ::kernel::ffi::c_ulong, pub elcpb_bc_tag: ::kernel::ffi::c_ulong }
#[repr(C)] pub struct el_t2_frame_mcheck { pub elfmc_header: el_t2_frame_header, pub elfmc_hdr: el_t2_logout_header, pub elfmc_procdata: el_t2_procdata_mcheck, pub elfmc_sysdata: el_t2_sysdata_mcheck, pub elfmc_t2data: el_t2_data_t2, pub elfmc_memdata: [el_t2_data_memory; 4], pub elfmc_footer: el_t2_frame_header }
#[repr(C)] pub struct el_t2_frame_corrected { pub elfcc_header: el_t2_frame_header, pub elfcc_hdr: el_t2_logout_header, pub elfcc_procdata: el_t2_data_corrected, pub elfcc_footer: el_t2_frame_header }

/* The original kernel-only inline I/O implementation is retained below in Rust form. */
#[inline] pub unsafe fn t2_inb(addr: usize) -> u8 { let result = core::ptr::read_volatile((addr.wrapping_shl(5).wrapping_add(T2_IO)) as *const i32); __kernel_extbl(result as ::kernel::ffi::c_long, (addr & 3) as ::kernel::ffi::c_ulong) as u8 }
#[inline] pub unsafe fn t2_outb(b: u8, addr: usize) { let w = __kernel_insbl(b as ::kernel::ffi::c_ulong, (addr & 3) as ::kernel::ffi::c_ulong); core::ptr::write_volatile((addr.wrapping_shl(5).wrapping_add(T2_IO)) as *mut u32, w as u32); mb(); }
#[inline] pub unsafe fn t2_inw(addr: usize) -> u16 { let result = core::ptr::read_volatile((addr.wrapping_shl(5).wrapping_add(T2_IO).wrapping_add(8)) as *const i32); __kernel_extwl(result as ::kernel::ffi::c_long, (addr & 3) as ::kernel::ffi::c_ulong) as u16 }
#[inline] pub unsafe fn t2_outw(b: u16, addr: usize) { let w = __kernel_inswl(b as ::kernel::ffi::c_ulong, (addr & 3) as ::kernel::ffi::c_ulong); core::ptr::write_volatile((addr.wrapping_shl(5).wrapping_add(T2_IO).wrapping_add(8)) as *mut u32, w as u32); mb(); }
#[inline] pub unsafe fn t2_inl(addr: usize) -> u32 { core::ptr::read_volatile((addr.wrapping_shl(5).wrapping_add(T2_IO).wrapping_add(0x18)) as *const u32) }
#[inline] pub unsafe fn t2_outl(b: u32, addr: usize) { core::ptr::write_volatile((addr.wrapping_shl(5).wrapping_add(T2_IO).wrapping_add(0x18)) as *mut u32, b); mb(); }
#[inline] pub unsafe fn t2_inq(addr: usize) -> u64 { core::ptr::read_volatile((addr.wrapping_shl(5).wrapping_add(T2_IO).wrapping_add(0x18)) as *const u64) }
#[inline] pub unsafe fn t2_outq(b: u64, addr: usize) { core::ptr::write_volatile((addr.wrapping_shl(5).wrapping_add(T2_IO).wrapping_add(0x18)) as *mut u64, b); mb(); }

/* T2_ONE_HAE_WINDOW is defined here, so the original t2_set_hae macro is empty. */
#[inline] pub unsafe fn t2_readb(xaddr: *const kernel::ffi::c_void) -> u8 { let addr = (xaddr as usize).wrapping_sub(T2_DENSE_MEM); let result = core::ptr::read_volatile((addr.wrapping_shl(5).wrapping_add(T2_SPARSE_MEM)) as *const i32); __kernel_extbl(result as ::kernel::ffi::c_long, (addr & 3) as ::kernel::ffi::c_ulong) as u8 }
#[inline] pub unsafe fn t2_readw(xaddr: *const kernel::ffi::c_void) -> u16 { let addr = (xaddr as usize).wrapping_sub(T2_DENSE_MEM); let result = core::ptr::read_volatile((addr.wrapping_shl(5).wrapping_add(T2_SPARSE_MEM).wrapping_add(8)) as *const u32); __kernel_extwl(result as ::kernel::ffi::c_long, (addr & 3) as ::kernel::ffi::c_ulong) as u16 }
#[inline] pub unsafe fn t2_readl(xaddr: *const kernel::ffi::c_void) -> u32 { let addr = (xaddr as usize).wrapping_sub(T2_DENSE_MEM); core::ptr::read_volatile((addr.wrapping_shl(5).wrapping_add(T2_SPARSE_MEM).wrapping_add(0x18)) as *const u32) }
#[inline] pub unsafe fn t2_readq(xaddr: *const kernel::ffi::c_void) -> u64 { let addr = (xaddr as usize).wrapping_sub(T2_DENSE_MEM); let work = addr.wrapping_shl(5).wrapping_add(T2_SPARSE_MEM).wrapping_add(0x18); let r0 = core::ptr::read_volatile(work as *const u32) as u64; let r1 = core::ptr::read_volatile(work.wrapping_add(4 << 5) as *const u32) as u64; r1.wrapping_shl(32) | r0 }
#[inline] pub unsafe fn t2_writeb(b: u8, xaddr: *mut kernel::ffi::c_void) { let addr = (xaddr as usize).wrapping_sub(T2_DENSE_MEM); let w = __kernel_insbl(b as ::kernel::ffi::c_ulong, (addr & 3) as ::kernel::ffi::c_ulong); core::ptr::write_volatile(addr.wrapping_shl(5).wrapping_add(T2_SPARSE_MEM) as *mut u32, w as u32); }
#[inline] pub unsafe fn t2_writew(b: u16, xaddr: *mut kernel::ffi::c_void) { let addr = (xaddr as usize).wrapping_sub(T2_DENSE_MEM); let w = __kernel_inswl(b as ::kernel::ffi::c_ulong, (addr & 3) as ::kernel::ffi::c_ulong); core::ptr::write_volatile(addr.wrapping_shl(5).wrapping_add(T2_SPARSE_MEM).wrapping_add(8) as *mut u32, w as u32); }
#[inline] pub unsafe fn t2_writel(b: u32, xaddr: *mut kernel::ffi::c_void) { let addr = (xaddr as usize).wrapping_sub(T2_DENSE_MEM); core::ptr::write_volatile(addr.wrapping_shl(5).wrapping_add(T2_SPARSE_MEM).wrapping_add(0x18) as *mut u32, b); }
#[inline] pub unsafe fn t2_writeq(b: u64, xaddr: *mut kernel::ffi::c_void) { let addr = (xaddr as usize).wrapping_sub(T2_DENSE_MEM); let work = addr.wrapping_shl(5).wrapping_add(T2_SPARSE_MEM).wrapping_add(0x18); core::ptr::write_volatile(work as *mut u32, b as u32); core::ptr::write_volatile(work.wrapping_add(4 << 5) as *mut u32, (b >> 32) as u32); }
#[inline] pub unsafe fn t2_ioportmap(addr: usize) -> *mut kernel::ffi::c_void { (addr.wrapping_add(T2_IO)) as *mut kernel::ffi::c_void }
#[inline] pub unsafe fn t2_ioremap(addr: usize, _size: usize) -> *mut kernel::ffi::c_void { (addr.wrapping_add(T2_DENSE_MEM)) as *mut kernel::ffi::c_void }
#[inline] pub fn t2_is_ioaddr(addr: usize) -> i32 { (addr as isize >= 0) as i32 }
#[inline] pub fn t2_is_mmio(addr: *const kernel::ffi::c_void) -> i32 { ((addr as usize) >= T2_DENSE_MEM) as i32 }

#[inline] pub unsafe fn t2_ioread8(xaddr: *const kernel::ffi::c_void) -> u8 { if t2_is_mmio(xaddr) != 0 { t2_readb(xaddr) } else { t2_inb((xaddr as usize).wrapping_sub(T2_IO)) } }
#[inline] pub unsafe fn t2_iowrite8(b: u8, xaddr: *mut kernel::ffi::c_void) { if t2_is_mmio(xaddr) != 0 { t2_writeb(b, xaddr) } else { t2_outb(b, (xaddr as usize).wrapping_sub(T2_IO)) } }
#[inline] pub unsafe fn t2_ioread16(xaddr: *const kernel::ffi::c_void) -> u16 { if t2_is_mmio(xaddr) != 0 { t2_readw(xaddr) } else { t2_inw((xaddr as usize).wrapping_sub(T2_IO)) } }
#[inline] pub unsafe fn t2_iowrite16(b: u16, xaddr: *mut kernel::ffi::c_void) { if t2_is_mmio(xaddr) != 0 { t2_writew(b, xaddr) } else { t2_outw(b, (xaddr as usize).wrapping_sub(T2_IO)) } }
#[inline] pub unsafe fn t2_ioread32(xaddr: *const kernel::ffi::c_void) -> u32 { if t2_is_mmio(xaddr) != 0 { t2_readl(xaddr) } else { t2_inl((xaddr as usize).wrapping_sub(T2_IO)) } }
#[inline] pub unsafe fn t2_iowrite32(b: u32, xaddr: *mut kernel::ffi::c_void) { if t2_is_mmio(xaddr) != 0 { t2_writel(b, xaddr) } else { t2_outl(b, (xaddr as usize).wrapping_sub(T2_IO)) } }
#[inline] pub unsafe fn t2_ioread64(xaddr: *const kernel::ffi::c_void) -> u64 { if t2_is_mmio(xaddr) != 0 { t2_readq(xaddr) } else { t2_inq((xaddr as usize).wrapping_sub(T2_IO)) } }
#[inline] pub unsafe fn t2_iowrite64(b: u64, xaddr: *mut kernel::ffi::c_void) { if t2_is_mmio(xaddr) != 0 { t2_writeq(b, xaddr) } else { t2_outq(b, (xaddr as usize).wrapping_sub(T2_IO)) } }

/* Original configuration: t2_trivial_rw_bw=0, t2_trivial_rw_lq=0,
 * t2_trivial_io_bw=0, t2_trivial_io_lq=0, t2_trivial_iounmap=1. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
