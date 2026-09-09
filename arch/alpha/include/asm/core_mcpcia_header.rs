/* SPDX-License-Identifier: GPL-2.0 */

/* MCPCIA_ONE_HAE_WINDOW is defined in the source header. */
pub const MCPCIA_ONE_HAE_WINDOW: usize = 1;

pub const MCPCIA_MAX_HOSES: usize = 4;

#[inline]
pub const fn MCPCIA_MID(m: usize) -> usize { m << 33 }

#[inline]
pub const fn MCPCIA_HOSE2MID(h: usize) -> usize { h + 4 }

pub const MCPCIA_MEM_MASK: usize = 0x07ffffff;

/* IDENT_ADDR is supplied by the surrounding Alpha platform code. */
macro_rules! MCPCIA_SPARSE { ($m:expr) => { IDENT_ADDR + 0xf000000000usize + MCPCIA_MID($m) }; }
macro_rules! MCPCIA_DENSE { ($m:expr) => { IDENT_ADDR + 0xf100000000usize + MCPCIA_MID($m) }; }
macro_rules! MCPCIA_IO { ($m:expr) => { IDENT_ADDR + 0xf180000000usize + MCPCIA_MID($m) }; }
macro_rules! MCPCIA_CONF { ($m:expr) => { IDENT_ADDR + 0xf1c0000000usize + MCPCIA_MID($m) }; }
macro_rules! MCPCIA_CSR { ($m:expr) => { IDENT_ADDR + 0xf1e0000000usize + MCPCIA_MID($m) }; }
macro_rules! MCPCIA_IO_IACK { ($m:expr) => { IDENT_ADDR + 0xf1f0000000usize + MCPCIA_MID($m) }; }
macro_rules! MCPCIA_DENSE_IO { ($m:expr) => { IDENT_ADDR + 0xe1fc000000usize + MCPCIA_MID($m) }; }
macro_rules! MCPCIA_DENSE_CONF { ($m:expr) => { IDENT_ADDR + 0xe1fe000000usize + MCPCIA_MID($m) }; }

macro_rules! MCPCIA_REG { ($name:ident, $off:expr) => {
    #[inline] pub const fn $name(m: usize) -> usize { MCPCIA_CSR!(m) + $off }
}; }

MCPCIA_REG!(MCPCIA_REV, 0x000);
MCPCIA_REG!(MCPCIA_WHOAMI, 0x040);
MCPCIA_REG!(MCPCIA_PCI_LAT, 0x080);
MCPCIA_REG!(MCPCIA_CAP_CTRL, 0x100);
MCPCIA_REG!(MCPCIA_HAE_MEM, 0x400);
MCPCIA_REG!(MCPCIA_HAE_IO, 0x440);
MCPCIA_REG!(_MCPCIA_IACK_SC, 0x480);
MCPCIA_REG!(MCPCIA_HAE_DENSE, 0x4C0);
MCPCIA_REG!(MCPCIA_INT_CTL, 0x500);
MCPCIA_REG!(MCPCIA_INT_REQ, 0x540);
MCPCIA_REG!(MCPCIA_INT_TARG, 0x580);
MCPCIA_REG!(MCPCIA_INT_ADR, 0x5C0);
MCPCIA_REG!(MCPCIA_INT_ADR_EXT, 0x600);
MCPCIA_REG!(MCPCIA_INT_MASK0, 0x640);
MCPCIA_REG!(MCPCIA_INT_MASK1, 0x680);
MCPCIA_REG!(MCPCIA_INT_ACK0, 0x10003f00);
MCPCIA_REG!(MCPCIA_INT_ACK1, 0x10003f40);
MCPCIA_REG!(MCPCIA_PERF_MON, 0x300);
MCPCIA_REG!(MCPCIA_PERF_CONT, 0x340);
MCPCIA_REG!(MCPCIA_CAP_DIAG, 0x700);
MCPCIA_REG!(MCPCIA_TOP_OF_MEM, 0x7C0);
MCPCIA_REG!(MCPCIA_MC_ERR0, 0x800);
MCPCIA_REG!(MCPCIA_MC_ERR1, 0x840);
MCPCIA_REG!(MCPCIA_CAP_ERR, 0x880);
MCPCIA_REG!(MCPCIA_PCI_ERR1, 0x1040);
MCPCIA_REG!(MCPCIA_MDPA_STAT, 0x4000);
MCPCIA_REG!(MCPCIA_MDPA_SYN, 0x4040);
MCPCIA_REG!(MCPCIA_MDPA_DIAG, 0x4080);
MCPCIA_REG!(MCPCIA_MDPB_STAT, 0x8000);
MCPCIA_REG!(MCPCIA_MDPB_SYN, 0x8040);
MCPCIA_REG!(MCPCIA_MDPB_DIAG, 0x8080);
MCPCIA_REG!(MCPCIA_SG_TBIA, 0x1300);
MCPCIA_REG!(MCPCIA_HBASE, 0x1340);
MCPCIA_REG!(MCPCIA_W0_BASE, 0x1400);
MCPCIA_REG!(MCPCIA_W0_MASK, 0x1440);
MCPCIA_REG!(MCPCIA_T0_BASE, 0x1480);
MCPCIA_REG!(MCPCIA_W1_BASE, 0x1500);
MCPCIA_REG!(MCPCIA_W1_MASK, 0x1540);
MCPCIA_REG!(MCPCIA_T1_BASE, 0x1580);
MCPCIA_REG!(MCPCIA_W2_BASE, 0x1600);
MCPCIA_REG!(MCPCIA_W2_MASK, 0x1640);
MCPCIA_REG!(MCPCIA_T2_BASE, 0x1680);
MCPCIA_REG!(MCPCIA_W3_BASE, 0x1700);
MCPCIA_REG!(MCPCIA_W3_MASK, 0x1740);
MCPCIA_REG!(MCPCIA_T3_BASE, 0x1780);

pub const MCPCIA_IACK_SC: usize = _MCPCIA_IACK_SC(4);
pub const MCPCIA_IO_BIAS: usize = MCPCIA_IO!(4);
pub const MCPCIA_MEM_BIAS: usize = MCPCIA_DENSE!(4);
pub const MCPCIA_DAC_OFFSET: usize = 1usize << 40;

#[repr(C)]
pub struct el_MCPCIA_uncorrected_frame_mcheck {
    pub header: el_common,
    pub procdata: el_common_EV5_uncorrectable_mcheck,
}

/* The following declarations correspond to the source's __KERNEL__ section. */

#[inline]
pub unsafe fn __mcpcia_is_mmio(addr: usize) -> i32 { if (addr & 0x80000000) == 0 { 1 } else { 0 } }

#[inline]
pub unsafe fn mcpcia_ioread8(xaddr: *const core::ffi::c_void) -> u8 {
    let addr = xaddr as usize & MCPCIA_MEM_MASK;
    let hose = xaddr as usize & !MCPCIA_MEM_MASK;
    let result = core::ptr::read_volatile(((addr << 5) + hose) as *const i32);
    __kernel_extbl(result as usize, addr & 3) as u8
}

#[inline]
pub unsafe fn mcpcia_iowrite8(b: u8, xaddr: *mut core::ffi::c_void) {
    let addr = xaddr as usize & MCPCIA_MEM_MASK;
    let hose = xaddr as usize & !MCPCIA_MEM_MASK;
    let w = __kernel_insbl(b as usize, addr & 3);
    core::ptr::write_volatile(((addr << 5) + hose) as *mut u32, w as u32);
}

#[inline]
pub unsafe fn mcpcia_ioread16(xaddr: *const core::ffi::c_void) -> u16 {
    let addr = xaddr as usize & MCPCIA_MEM_MASK;
    let hose = xaddr as usize & !MCPCIA_MEM_MASK;
    let result = core::ptr::read_volatile(((addr << 5) + hose + 0x08) as *const i32);
    __kernel_extwl(result as usize, addr & 3) as u16
}

#[inline]
pub unsafe fn mcpcia_iowrite16(b: u16, xaddr: *mut core::ffi::c_void) {
    let addr = xaddr as usize & MCPCIA_MEM_MASK;
    let hose = xaddr as usize & !MCPCIA_MEM_MASK;
    let w = __kernel_inswl(b as usize, addr & 3);
    core::ptr::write_volatile(((addr << 5) + hose + 0x08) as *mut u32, w as u32);
}

#[inline]
pub unsafe fn mcpcia_ioread32(xaddr: *const core::ffi::c_void) -> u32 {
    let mut addr = xaddr as usize;
    if __mcpcia_is_mmio(addr) == 0 { addr = ((addr & 0xffff) << 5) + (addr & !0xffffusize) + 0x18; }
    core::ptr::read_volatile(addr as *const u32)
}

#[inline]
pub unsafe fn mcpcia_iowrite32(b: u32, xaddr: *mut core::ffi::c_void) {
    let mut addr = xaddr as usize;
    if __mcpcia_is_mmio(addr) == 0 { addr = ((addr & 0xffff) << 5) + (addr & !0xffffusize) + 0x18; }
    core::ptr::write_volatile(addr as *mut u32, b);
}

#[inline]
pub unsafe fn mcpcia_ioread64(xaddr: *const core::ffi::c_void) -> u64 {
    let mut addr = xaddr as usize;
    if __mcpcia_is_mmio(addr) == 0 { addr = ((addr & 0xffff) << 5) + (addr & !0xffffusize) + 0x18; }
    core::ptr::read_volatile(addr as *const u64)
}

#[inline]
pub unsafe fn mcpcia_iowrite64(b: u64, xaddr: *mut core::ffi::c_void) {
    let mut addr = xaddr as usize;
    if __mcpcia_is_mmio(addr) == 0 { addr = ((addr & 0xffff) << 5) + (addr & !0xffffusize) + 0x18; }
    core::ptr::write_volatile(addr as *mut u64, b);
}

#[inline]
pub unsafe fn mcpcia_ioportmap(addr: usize) -> *mut core::ffi::c_void { (addr + MCPCIA_IO_BIAS) as *mut core::ffi::c_void }

#[inline]
pub unsafe fn mcpcia_ioremap(addr: usize, _size: usize) -> *mut core::ffi::c_void { (addr + MCPCIA_MEM_BIAS) as *mut core::ffi::c_void }

#[inline]
pub unsafe fn mcpcia_is_ioaddr(addr: usize) -> i32 { if addr >= MCPCIA_SPARSE!(0) { 1 } else { 0 } }

#[inline]
pub unsafe fn mcpcia_is_mmio(xaddr: *const core::ffi::c_void) -> i32 { __mcpcia_is_mmio(xaddr as usize) }

pub const mcpcia_trivial_rw_bw: usize = 2;
pub const mcpcia_trivial_rw_lq: usize = 1;
pub const mcpcia_trivial_io_bw: usize = 0;
pub const mcpcia_trivial_io_lq: usize = 0;
pub const mcpcia_trivial_iounmap: usize = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
