/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: symbols from <asm/mach-bcm63xx/bcm63xx_cpu.h> are supplied
// by the surrounding translation unit.

/*
 * Physical memory map, RAM is mapped at 0x0.
 *
 * Note that size MUST be a power of two.
 */
pub const BCM_PCMCIA_COMMON_BASE_PA: usize = 0x20000000;
pub const BCM_PCMCIA_COMMON_SIZE: usize = 16 * 1024 * 1024;
pub const BCM_PCMCIA_COMMON_END_PA: usize = BCM_PCMCIA_COMMON_BASE_PA + BCM_PCMCIA_COMMON_SIZE - 1;

pub const BCM_PCMCIA_ATTR_BASE_PA: usize = 0x21000000;
pub const BCM_PCMCIA_ATTR_SIZE: usize = 16 * 1024 * 1024;
pub const BCM_PCMCIA_ATTR_END_PA: usize = BCM_PCMCIA_ATTR_BASE_PA + BCM_PCMCIA_ATTR_SIZE - 1;

pub const BCM_PCMCIA_IO_BASE_PA: usize = 0x22000000;
pub const BCM_PCMCIA_IO_SIZE: usize = 64 * 1024;
pub const BCM_PCMCIA_IO_END_PA: usize = BCM_PCMCIA_IO_BASE_PA + BCM_PCMCIA_IO_SIZE - 1;

pub const BCM_PCI_MEM_BASE_PA: usize = 0x30000000;
pub const BCM_PCI_MEM_SIZE: usize = 128 * 1024 * 1024;
pub const BCM_PCI_MEM_END_PA: usize = BCM_PCI_MEM_BASE_PA + BCM_PCI_MEM_SIZE - 1;

pub const BCM_PCI_IO_BASE_PA: usize = 0x08000000;
pub const BCM_PCI_IO_SIZE: usize = 64 * 1024;
pub const BCM_PCI_IO_END_PA: usize = BCM_PCI_IO_BASE_PA + BCM_PCI_IO_SIZE - 1;
pub const BCM_PCI_IO_HALF_PA: usize = BCM_PCI_IO_BASE_PA + (BCM_PCI_IO_SIZE / 2) - 1;

pub const BCM_CB_MEM_BASE_PA: usize = 0x38000000;
pub const BCM_CB_MEM_SIZE: usize = 128 * 1024 * 1024;
pub const BCM_CB_MEM_END_PA: usize = BCM_CB_MEM_BASE_PA + BCM_CB_MEM_SIZE - 1;

pub const BCM_PCIE_MEM_BASE_PA: usize = 0x10f00000;
pub const BCM_PCIE_MEM_SIZE: usize = 16 * 1024 * 1024;
pub const BCM_PCIE_MEM_END_PA: usize = BCM_PCIE_MEM_BASE_PA + BCM_PCIE_MEM_SIZE - 1;

/* Internal registers are accessed through KSEG3. */
#[macro_export]
macro_rules! BCM_REGS_VA { ($x:expr) => { $x as *mut core::ffi::c_void }; }

#[macro_export]
macro_rules! bcm_readb { ($a:expr) => { unsafe { core::ptr::read_volatile(BCM_REGS_VA!($a) as *const u8) } }; }
#[macro_export]
macro_rules! bcm_readw { ($a:expr) => { unsafe { core::ptr::read_volatile(BCM_REGS_VA!($a) as *const u16) } }; }
#[macro_export]
macro_rules! bcm_readl { ($a:expr) => { unsafe { core::ptr::read_volatile(BCM_REGS_VA!($a) as *const u32) } }; }
#[macro_export]
macro_rules! bcm_readq { ($a:expr) => { unsafe { core::ptr::read_volatile(BCM_REGS_VA!($a) as *const u64) } }; }
#[macro_export]
macro_rules! bcm_writeb { ($v:expr, $a:expr) => { unsafe { core::ptr::write_volatile(BCM_REGS_VA!($a) as *mut u8, $v as u8) } }; }
#[macro_export]
macro_rules! bcm_writew { ($v:expr, $a:expr) => { unsafe { core::ptr::write_volatile(BCM_REGS_VA!($a) as *mut u16, $v as u16) } }; }
#[macro_export]
macro_rules! bcm_writel { ($v:expr, $a:expr) => { unsafe { core::ptr::write_volatile(BCM_REGS_VA!($a) as *mut u32, $v as u32) } }; }
#[macro_export]
macro_rules! bcm_writeq { ($v:expr, $a:expr) => { unsafe { core::ptr::write_volatile(BCM_REGS_VA!($a) as *mut u64, $v as u64) } }; }

/* IO helpers to access register set for current CPU. */
#[macro_export] macro_rules! bcm_rset_readb { ($s:expr, $o:expr) => { bcm_readb!(bcm63xx_regset_address($s) + ($o)) }; }
#[macro_export] macro_rules! bcm_rset_readw { ($s:expr, $o:expr) => { bcm_readw!(bcm63xx_regset_address($s) + ($o)) }; }
#[macro_export] macro_rules! bcm_rset_readl { ($s:expr, $o:expr) => { bcm_readl!(bcm63xx_regset_address($s) + ($o)) }; }
#[macro_export] macro_rules! bcm_rset_writeb { ($s:expr, $v:expr, $o:expr) => { bcm_writeb!($v, bcm63xx_regset_address($s) + ($o)) }; }
#[macro_export] macro_rules! bcm_rset_writew { ($s:expr, $v:expr, $o:expr) => { bcm_writew!($v, bcm63xx_regset_address($s) + ($o)) }; }
#[macro_export] macro_rules! bcm_rset_writel { ($s:expr, $v:expr, $o:expr) => { bcm_writel!($v, bcm63xx_regset_address($s) + ($o)) }; }

/* Helpers for frequently used register sets. */
#[macro_export] macro_rules! bcm_perf_readl { ($o:expr) => { bcm_rset_readl!(RSET_PERF, $o) }; }
#[macro_export] macro_rules! bcm_perf_writel { ($v:expr, $o:expr) => { bcm_rset_writel!(RSET_PERF, $v, $o) }; }
#[macro_export] macro_rules! bcm_timer_readl { ($o:expr) => { bcm_rset_readl!(RSET_TIMER, $o) }; }
#[macro_export] macro_rules! bcm_timer_writel { ($v:expr, $o:expr) => { bcm_rset_writel!(RSET_TIMER, $v, $o) }; }
#[macro_export] macro_rules! bcm_wdt_readl { ($o:expr) => { bcm_rset_readl!(RSET_WDT, $o) }; }
#[macro_export] macro_rules! bcm_wdt_writel { ($v:expr, $o:expr) => { bcm_rset_writel!(RSET_WDT, $v, $o) }; }
#[macro_export] macro_rules! bcm_gpio_readl { ($o:expr) => { bcm_rset_readl!(RSET_GPIO, $o) }; }
#[macro_export] macro_rules! bcm_gpio_writel { ($v:expr, $o:expr) => { bcm_rset_writel!(RSET_GPIO, $v, $o) }; }
#[macro_export] macro_rules! bcm_uart0_readl { ($o:expr) => { bcm_rset_readl!(RSET_UART0, $o) }; }
#[macro_export] macro_rules! bcm_uart0_writel { ($v:expr, $o:expr) => { bcm_rset_writel!(RSET_UART0, $v, $o) }; }
#[macro_export] macro_rules! bcm_mpi_readl { ($o:expr) => { bcm_rset_readl!(RSET_MPI, $o) }; }
#[macro_export] macro_rules! bcm_mpi_writel { ($v:expr, $o:expr) => { bcm_rset_writel!(RSET_MPI, $v, $o) }; }
#[macro_export] macro_rules! bcm_pcmcia_readl { ($o:expr) => { bcm_rset_readl!(RSET_PCMCIA, $o) }; }
#[macro_export] macro_rules! bcm_pcmcia_writel { ($v:expr, $o:expr) => { bcm_rset_writel!(RSET_PCMCIA, $v, $o) }; }
#[macro_export] macro_rules! bcm_pcie_readl { ($o:expr) => { bcm_rset_readl!(RSET_PCIE, $o) }; }
#[macro_export] macro_rules! bcm_pcie_writel { ($v:expr, $o:expr) => { bcm_rset_writel!(RSET_PCIE, $v, $o) }; }
#[macro_export] macro_rules! bcm_sdram_readl { ($o:expr) => { bcm_rset_readl!(RSET_SDRAM, $o) }; }
#[macro_export] macro_rules! bcm_sdram_writel { ($v:expr, $o:expr) => { bcm_rset_writel!(RSET_SDRAM, $v, $o) }; }
#[macro_export] macro_rules! bcm_memc_readl { ($o:expr) => { bcm_rset_readl!(RSET_MEMC, $o) }; }
#[macro_export] macro_rules! bcm_memc_writel { ($v:expr, $o:expr) => { bcm_rset_writel!(RSET_MEMC, $v, $o) }; }
#[macro_export] macro_rules! bcm_ddr_readl { ($o:expr) => { bcm_rset_readl!(RSET_DDR, $o) }; }
#[macro_export] macro_rules! bcm_ddr_writel { ($v:expr, $o:expr) => { bcm_rset_writel!(RSET_DDR, $v, $o) }; }
#[macro_export] macro_rules! bcm_misc_readl { ($o:expr) => { bcm_rset_readl!(RSET_MISC, $o) }; }
#[macro_export] macro_rules! bcm_misc_writel { ($v:expr, $o:expr) => { bcm_rset_writel!(RSET_MISC, $v, $o) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
