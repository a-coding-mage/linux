/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from alpha/include/asm/core_cia.h. */

/* CIA_ONE_HAE_WINDOW is a build-time experiment flag. */
pub const CIA_ONE_HAE_WINDOW: usize = 1;

pub const CIA_MEM_R1_MASK: u64 = 0x1fffffff;
pub const CIA_MEM_R2_MASK: u64 = 0x07ffffff;
pub const CIA_MEM_R3_MASK: u64 = 0x03ffffff;

pub const CIA_IOC_CIA_REV: u64 = IDENT_ADDR + 0x8740000080;
pub const CIA_REV_MASK: u64 = 0xff;
pub const CIA_IOC_PCI_LAT: u64 = IDENT_ADDR + 0x87400000c0;
pub const CIA_IOC_CIA_CTRL: u64 = IDENT_ADDR + 0x8740000100;
pub const CIA_CTRL_PCI_EN: u64 = 1 << 0;
pub const CIA_CTRL_PCI_LOCK_EN: u64 = 1 << 1;
pub const CIA_CTRL_PCI_LOOP_EN: u64 = 1 << 2;
pub const CIA_CTRL_FST_BB_EN: u64 = 1 << 3;
pub const CIA_CTRL_PCI_MST_EN: u64 = 1 << 4;
pub const CIA_CTRL_PCI_MEM_EN: u64 = 1 << 5;
pub const CIA_CTRL_PCI_REQ64_EN: u64 = 1 << 6;
pub const CIA_CTRL_PCI_ACK64_EN: u64 = 1 << 7;
pub const CIA_CTRL_ADDR_PE_EN: u64 = 1 << 8;
pub const CIA_CTRL_PERR_EN: u64 = 1 << 9;
pub const CIA_CTRL_FILL_ERR_EN: u64 = 1 << 10;
pub const CIA_CTRL_MCHK_ERR_EN: u64 = 1 << 11;
pub const CIA_CTRL_ECC_CHK_EN: u64 = 1 << 12;
pub const CIA_CTRL_ASSERT_IDLE_BC: u64 = 1 << 13;
pub const CIA_CTRL_COM_IDLE_BC: u64 = 1 << 14;
pub const CIA_CTRL_CSR_IOA_BYPASS: u64 = 1 << 15;
pub const CIA_CTRL_IO_FLUSHREQ_EN: u64 = 1 << 16;
pub const CIA_CTRL_CPU_FLUSHREQ_EN: u64 = 1 << 17;
pub const CIA_CTRL_ARB_CPU_EN: u64 = 1 << 18;
pub const CIA_CTRL_EN_ARB_LINK: u64 = 1 << 19;
pub const CIA_CTRL_RD_TYPE_SHIFT: u64 = 20;
pub const CIA_CTRL_RL_TYPE_SHIFT: u64 = 24;
pub const CIA_CTRL_RM_TYPE_SHIFT: u64 = 28;
pub const CIA_CTRL_EN_DMA_RD_PERF: u64 = 1 << 31;
pub const CIA_IOC_CIA_CNFG: u64 = IDENT_ADDR + 0x8740000140;
pub const CIA_CNFG_IOA_BWEN: u64 = 1 << 0;
pub const CIA_CNFG_PCI_MWEN: u64 = 1 << 4;
pub const CIA_CNFG_PCI_DWEN: u64 = 1 << 5;
pub const CIA_CNFG_PCI_WLEN: u64 = 1 << 8;
pub const CIA_IOC_FLASH_CTRL: u64 = IDENT_ADDR + 0x8740000200;
pub const CIA_IOC_HAE_MEM: u64 = IDENT_ADDR + 0x8740000400;
pub const CIA_IOC_HAE_IO: u64 = IDENT_ADDR + 0x8740000440;
pub const CIA_IOC_CFG: u64 = IDENT_ADDR + 0x8740000480;
pub const CIA_IOC_CACK_EN: u64 = IDENT_ADDR + 0x8740000600;
pub const CIA_CACK_EN_LOCK_EN: u64 = 1 << 0;
pub const CIA_CACK_EN_MB_EN: u64 = 1 << 1;
pub const CIA_CACK_EN_SET_DIRTY_EN: u64 = 1 << 2;
pub const CIA_CACK_EN_BC_VICTIM_EN: u64 = 1 << 3;

pub const CIA_IOC_CIA_DIAG: u64 = IDENT_ADDR + 0x8740002000;
pub const CIA_IOC_DIAG_CHECK: u64 = IDENT_ADDR + 0x8740003000;
pub const CIA_IOC_PERF_MONITOR: u64 = IDENT_ADDR + 0x8740004000;
pub const CIA_IOC_PERF_CONTROL: u64 = IDENT_ADDR + 0x8740004040;
pub const CIA_IOC_CPU_ERR0: u64 = IDENT_ADDR + 0x8740008000;
pub const CIA_IOC_CPU_ERR1: u64 = IDENT_ADDR + 0x8740008040;
pub const CIA_IOC_CIA_ERR: u64 = IDENT_ADDR + 0x8740008200;
pub const CIA_ERR_COR_ERR: u64 = 1 << 0;
pub const CIA_ERR_UN_COR_ERR: u64 = 1 << 1;
pub const CIA_ERR_CPU_PE: u64 = 1 << 2;
pub const CIA_ERR_MEM_NEM: u64 = 1 << 3;
pub const CIA_ERR_PCI_SERR: u64 = 1 << 4;
pub const CIA_ERR_PERR: u64 = 1 << 5;
pub const CIA_ERR_PCI_ADDR_PE: u64 = 1 << 6;
pub const CIA_ERR_RCVD_MAS_ABT: u64 = 1 << 7;
pub const CIA_ERR_RCVD_TAR_ABT: u64 = 1 << 8;
pub const CIA_ERR_PA_PTE_INV: u64 = 1 << 9;
pub const CIA_ERR_FROM_WRT_ERR: u64 = 1 << 10;
pub const CIA_ERR_IOA_TIMEOUT: u64 = 1 << 11;
pub const CIA_ERR_LOST_CORR_ERR: u64 = 1 << 16;
pub const CIA_ERR_LOST_UN_CORR_ERR: u64 = 1 << 17;
pub const CIA_ERR_LOST_CPU_PE: u64 = 1 << 18;
pub const CIA_ERR_LOST_MEM_NEM: u64 = 1 << 19;
pub const CIA_ERR_LOST_PERR: u64 = 1 << 21;
pub const CIA_ERR_LOST_PCI_ADDR_PE: u64 = 1 << 22;
pub const CIA_ERR_LOST_RCVD_MAS_ABT: u64 = 1 << 23;
pub const CIA_ERR_LOST_RCVD_TAR_ABT: u64 = 1 << 24;
pub const CIA_ERR_LOST_PA_PTE_INV: u64 = 1 << 25;
pub const CIA_ERR_LOST_FROM_WRT_ERR: u64 = 1 << 26;
pub const CIA_ERR_LOST_IOA_TIMEOUT: u64 = 1 << 27;
pub const CIA_ERR_VALID: u64 = 1 << 31;
pub const CIA_IOC_CIA_STAT: u64 = IDENT_ADDR + 0x8740008240;
pub const CIA_IOC_ERR_MASK: u64 = IDENT_ADDR + 0x8740008280;
pub const CIA_IOC_CIA_SYN: u64 = IDENT_ADDR + 0x8740008300;
pub const CIA_IOC_MEM_ERR0: u64 = IDENT_ADDR + 0x8740008400;
pub const CIA_IOC_MEM_ERR1: u64 = IDENT_ADDR + 0x8740008440;
pub const CIA_IOC_PCI_ERR0: u64 = IDENT_ADDR + 0x8740008800;
pub const CIA_IOC_PCI_ERR1: u64 = IDENT_ADDR + 0x8740008840;
pub const CIA_IOC_PCI_ERR3: u64 = IDENT_ADDR + 0x8740008880;

pub const CIA_IOC_MCR: u64 = IDENT_ADDR + 0x8750000000;
pub const CIA_IOC_MBA0: u64 = IDENT_ADDR + 0x8750000600;
pub const CIA_IOC_MBA2: u64 = IDENT_ADDR + 0x8750000680;
pub const CIA_IOC_MBA4: u64 = IDENT_ADDR + 0x8750000700;
pub const CIA_IOC_MBA6: u64 = IDENT_ADDR + 0x8750000780;
pub const CIA_IOC_MBA8: u64 = IDENT_ADDR + 0x8750000800;
pub const CIA_IOC_MBAA: u64 = IDENT_ADDR + 0x8750000880;
pub const CIA_IOC_MBAC: u64 = IDENT_ADDR + 0x8750000900;
pub const CIA_IOC_MBAE: u64 = IDENT_ADDR + 0x8750000980;
pub const CIA_IOC_TMG0: u64 = IDENT_ADDR + 0x8750000b00;
pub const CIA_IOC_TMG1: u64 = IDENT_ADDR + 0x8750000b40;
pub const CIA_IOC_TMG2: u64 = IDENT_ADDR + 0x8750000b80;

pub const CIA_IOC_PCI_TBIA: u64 = IDENT_ADDR + 0x8760000100;
pub const CIA_IOC_PCI_W0_BASE: u64 = IDENT_ADDR + 0x8760000400;
pub const CIA_IOC_PCI_W0_MASK: u64 = IDENT_ADDR + 0x8760000440;
pub const CIA_IOC_PCI_T0_BASE: u64 = IDENT_ADDR + 0x8760000480;
pub const CIA_IOC_PCI_W1_BASE: u64 = IDENT_ADDR + 0x8760000500;
pub const CIA_IOC_PCI_W1_MASK: u64 = IDENT_ADDR + 0x8760000540;
pub const CIA_IOC_PCI_T1_BASE: u64 = IDENT_ADDR + 0x8760000580;
pub const CIA_IOC_PCI_W2_BASE: u64 = IDENT_ADDR + 0x8760000600;
pub const CIA_IOC_PCI_W2_MASK: u64 = IDENT_ADDR + 0x8760000640;
pub const CIA_IOC_PCI_T2_BASE: u64 = IDENT_ADDR + 0x8760000680;
pub const CIA_IOC_PCI_W3_BASE: u64 = IDENT_ADDR + 0x8760000700;
pub const CIA_IOC_PCI_W3_MASK: u64 = IDENT_ADDR + 0x8760000740;
pub const CIA_IOC_PCI_T3_BASE: u64 = IDENT_ADDR + 0x8760000780;
pub const CIA_IOC_PCI_W_DAC: u64 = IDENT_ADDR + 0x87600007c0;
pub const CIA_IOC_TB_TAG_BASE: u64 = IDENT_ADDR + 0x8760000800;
pub const CIA_IOC_TB_PAGE_BASE: u64 = IDENT_ADDR + 0x8760001000;

macro_rules! cia_pci_wn_base { ($n:expr) => { IDENT_ADDR + 0x8760000400 + ($n as u64) * 0x100 }; }
macro_rules! cia_pci_wn_mask { ($n:expr) => { IDENT_ADDR + 0x8760000440 + ($n as u64) * 0x100 }; }
macro_rules! cia_pci_tn_base { ($n:expr) => { IDENT_ADDR + 0x8760000480 + ($n as u64) * 0x100 }; }
macro_rules! cia_tb_tagn { ($n:expr) => { IDENT_ADDR + 0x8760000800 + ($n as u64) * 0x40 }; }
macro_rules! cia_tbn_pagem { ($n:expr, $m:expr) => { IDENT_ADDR + 0x8760001000 + ($n as u64) * 0x100 + ($m as u64) * 0x40 }; }

pub const CIA_IACK_SC: u64 = IDENT_ADDR + 0x8720000000;
pub const CIA_CONF: u64 = IDENT_ADDR + 0x8700000000;
pub const CIA_IO: u64 = IDENT_ADDR + 0x8580000000;
pub const CIA_SPARSE_MEM: u64 = IDENT_ADDR + 0x8000000000;
pub const CIA_SPARSE_MEM_R2: u64 = IDENT_ADDR + 0x8400000000;
pub const CIA_SPARSE_MEM_R3: u64 = IDENT_ADDR + 0x8500000000;
pub const CIA_DENSE_MEM: u64 = IDENT_ADDR + 0x8600000000;
pub const CIA_BW_MEM: u64 = IDENT_ADDR + 0x8800000000;
pub const CIA_BW_IO: u64 = IDENT_ADDR + 0x8900000000;
pub const CIA_BW_CFG_0: u64 = IDENT_ADDR + 0x8a00000000;
pub const CIA_BW_CFG_1: u64 = IDENT_ADDR + 0x8b00000000;

pub const GRU_INT_REQ: u64 = IDENT_ADDR + 0x8780000000;
pub const GRU_INT_MASK: u64 = IDENT_ADDR + 0x8780000040;
pub const GRU_INT_EDGE: u64 = IDENT_ADDR + 0x8780000080;
pub const GRU_INT_HILO: u64 = IDENT_ADDR + 0x87800000c0;
pub const GRU_INT_CLEAR: u64 = IDENT_ADDR + 0x8780000100;
pub const GRU_CACHE_CNFG: u64 = IDENT_ADDR + 0x8780000200;
pub const GRU_SCR: u64 = IDENT_ADDR + 0x8780000300;
pub const GRU_LED: u64 = IDENT_ADDR + 0x8780000800;
pub const GRU_RESET: u64 = IDENT_ADDR + 0x8780000900;
pub const ALCOR_GRU_INT_REQ_BITS: u64 = 0x800fffff;
pub const XLT_GRU_INT_REQ_BITS: u64 = 0x80003fff;
/* GRU_INT_REQ_BITS expands to alpha_mv.sys.cia.gru_int_req_bits + 0. */

pub const PYXIS_INT_REQ: u64 = IDENT_ADDR + 0x87a0000000;
pub const PYXIS_INT_MASK: u64 = IDENT_ADDR + 0x87a0000040;
pub const PYXIS_INT_HILO: u64 = IDENT_ADDR + 0x87a00000c0;
pub const PYXIS_INT_ROUTE: u64 = IDENT_ADDR + 0x87a0000140;
pub const PYXIS_GPO: u64 = IDENT_ADDR + 0x87a0000180;
pub const PYXIS_INT_CNFG: u64 = IDENT_ADDR + 0x87a00001c0;
pub const PYXIS_RT_COUNT: u64 = IDENT_ADDR + 0x87a0000200;
pub const PYXIS_INT_TIME: u64 = IDENT_ADDR + 0x87a0000240;
pub const PYXIS_IIC_CTRL: u64 = IDENT_ADDR + 0x87a00002c0;
pub const PYXIS_RESET: u64 = IDENT_ADDR + 0x8780000900;
pub const PYXIS_DAC_OFFSET: u64 = 1 << 40;

#[repr(C)]
pub struct el_CIA_sysdata_mcheck {
    pub cpu_err0: c_ulong, pub cpu_err1: c_ulong, pub cia_err: c_ulong,
    pub cia_stat: c_ulong, pub err_mask: c_ulong, pub cia_syn: c_ulong,
    pub mem_err0: c_ulong, pub mem_err1: c_ulong, pub pci_err0: c_ulong,
    pub pci_err1: c_ulong, pub pci_err2: c_ulong,
}

/* The following helpers are the __KERNEL__ inline I/O implementation. */
#[inline(always)]
pub unsafe fn cia_ioread8(xaddr: *const core::ffi::c_void) -> u8 {
    let mut addr = xaddr as u64;
    let base = if addr >= CIA_DENSE_MEM { CIA_SPARSE_MEM } else { CIA_IO };
    addr &= CIA_MEM_R1_MASK;
    let result = core::ptr::read_volatile(((addr << 5) + base) as *const u32);
    __kernel_extbl(result as c_ulong, (addr & 3) as c_ulong) as u8
}
#[inline(always)]
pub unsafe fn cia_iowrite8(b: u8, xaddr: *mut core::ffi::c_void) {
    let mut addr = xaddr as u64;
    let base = if addr >= CIA_DENSE_MEM { CIA_SPARSE_MEM } else { CIA_IO };
    addr &= CIA_MEM_R1_MASK;
    let w = __kernel_insbl(b as c_ulong, (addr & 3) as c_ulong);
    core::ptr::write_volatile(((addr << 5) + base) as *mut u32, w as u32);
}
#[inline(always)]
pub unsafe fn cia_ioread16(xaddr: *const core::ffi::c_void) -> u16 {
    let mut addr = xaddr as u64;
    let base = if addr >= CIA_DENSE_MEM { CIA_SPARSE_MEM + 8 } else { CIA_IO + 8 };
    addr &= CIA_MEM_R1_MASK;
    let result = core::ptr::read_volatile(((addr << 5) + base) as *const u32);
    __kernel_extwl(result as c_ulong, (addr & 3) as c_ulong) as u16
}
#[inline(always)]
pub unsafe fn cia_iowrite16(b: u16, xaddr: *mut core::ffi::c_void) {
    let mut addr = xaddr as u64;
    let base = if addr >= CIA_DENSE_MEM { CIA_SPARSE_MEM + 8 } else { CIA_IO + 8 };
    addr &= CIA_MEM_R1_MASK;
    let w = __kernel_inswl(b as c_ulong, (addr & 3) as c_ulong);
    core::ptr::write_volatile(((addr << 5) + base) as *mut u32, w as u32);
}
#[inline(always)]
pub unsafe fn cia_ioread32(xaddr: *const core::ffi::c_void) -> u32 {
    let mut addr = xaddr as u64; if addr < CIA_DENSE_MEM { addr = ((addr - CIA_IO) << 5) + CIA_IO + 0x18; }
    core::ptr::read_volatile(addr as *const u32)
}
#[inline(always)]
pub unsafe fn cia_iowrite32(b: u32, xaddr: *mut core::ffi::c_void) {
    let mut addr = xaddr as u64; if addr < CIA_DENSE_MEM { addr = ((addr - CIA_IO) << 5) + CIA_IO + 0x18; }
    core::ptr::write_volatile(addr as *mut u32, b)
}
#[inline(always)]
pub unsafe fn cia_ioread64(xaddr: *const core::ffi::c_void) -> u64 {
    let mut addr = xaddr as u64; if addr < CIA_DENSE_MEM { addr = ((addr - CIA_IO) << 5) + CIA_IO + 0x18; }
    core::ptr::read_volatile(addr as *const u64)
}
#[inline(always)]
pub unsafe fn cia_iowrite64(b: u64, xaddr: *mut core::ffi::c_void) {
    let mut addr = xaddr as u64; if addr < CIA_DENSE_MEM { addr = ((addr - CIA_IO) << 5) + CIA_IO + 0x18; }
    core::ptr::write_volatile(addr as *mut u64, b)
}
pub unsafe fn cia_ioportmap(addr: u64) -> *mut core::ffi::c_void { (addr + CIA_IO) as *mut _ }
pub unsafe fn cia_ioremap(addr: u64, _size: u64) -> *mut core::ffi::c_void { (addr + CIA_DENSE_MEM) as *mut _ }
pub fn cia_is_ioaddr(addr: u64) -> i32 { (addr >= IDENT_ADDR + 0x8000000000) as i32 }
pub fn cia_is_mmio(addr: *const core::ffi::c_void) -> i32 { ((addr as u64) >= CIA_DENSE_MEM) as i32 }
pub unsafe fn cia_bwx_ioportmap(addr: u64) -> *mut core::ffi::c_void { (addr + CIA_BW_IO) as *mut _ }
pub unsafe fn cia_bwx_ioremap(addr: u64, _size: u64) -> *mut core::ffi::c_void { (addr + CIA_BW_MEM) as *mut _ }
pub fn cia_bwx_is_ioaddr(addr: u64) -> i32 { (addr >= IDENT_ADDR + 0x8000000000) as i32 }
pub fn cia_bwx_is_mmio(addr: *const core::ffi::c_void) -> i32 { ((addr as u64) < CIA_BW_IO) as i32 }

/* External Alpha/compiler symbols and c_ulong are supplied by other headers. */
extern "C" { fn __kernel_extbl(a: c_ulong, b: c_ulong) -> c_ulong; fn __kernel_insbl(a: c_ulong, b: c_ulong) -> c_ulong; fn __kernel_extwl(a: c_ulong, b: c_ulong) -> c_ulong; fn __kernel_inswl(a: c_ulong, b: c_ulong) -> c_ulong; }
/* asm/io_trivial.h is included twice in the C header to generate cia and cia_bwx families. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
