/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2009 Lemote, Inc.
 * Author: Wu Zhangjin <wuzhangjin@gmail.com>
 */

/* Dependencies supplied by the surrounding kernel translation. */

extern "C" {
    pub fn bonito_irq_init();
    pub fn mach_prepare_reboot();
    pub fn mach_prepare_shutdown();
    pub fn mach_prom_init_machtype();
    pub static mut cpu_clock_freq: u32;
    pub static mut memsize: u32;
    pub static mut highmemsize: u32;
    pub fn prom_init_memory();
    pub fn prom_init_machtype();
    pub fn prom_init_env();
    pub fn bonito_irqdispatch();
    pub fn mach_init_irq();
    pub fn mach_irq_dispatch(pending: u32);
    pub fn mach_i8259_irq() -> i32;
    pub fn setup_wakeup_events();
    pub fn wakeup_loongson() -> i32;
    pub fn mach_suspend();
    pub fn mach_resume();
}

#[cfg(CONFIG_LOONGSON_UART_BASE)]
extern "C" {
    pub static mut _loongson_uart_base: usize;
    pub static mut loongson_uart_base: usize;
    pub fn prom_init_loongson_uart_base();
}

#[inline]
pub unsafe fn prom_init_uart_base() {
    #[cfg(CONFIG_LOONGSON_UART_BASE)]
    prom_init_loongson_uart_base();
}

#[inline]
pub unsafe fn delay() {
    let mut x: i32 = 0;
    while x < 100000 {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        x += 1;
    }
}

pub const LOONGSON_IRQ_BASE: u32 = 32;
pub const LOONGSON_FLASH_BASE: u32 = 0x1c000000;
pub const LOONGSON_FLASH_SIZE: u32 = 0x02000000;
pub const LOONGSON_FLASH_TOP: u32 = LOONGSON_FLASH_BASE + LOONGSON_FLASH_SIZE - 1;
pub const LOONGSON_LIO0_BASE: u32 = 0x1e000000;
pub const LOONGSON_LIO0_SIZE: u32 = 0x01c00000;
pub const LOONGSON_LIO0_TOP: u32 = LOONGSON_LIO0_BASE + LOONGSON_LIO0_SIZE - 1;
pub const LOONGSON_BOOT_BASE: u32 = 0x1fc00000;
pub const LOONGSON_BOOT_SIZE: u32 = 0x00100000;
pub const LOONGSON_BOOT_TOP: u32 = LOONGSON_BOOT_BASE + LOONGSON_BOOT_SIZE - 1;
pub const LOONGSON_REG_BASE: u32 = 0x1fe00000;
pub const LOONGSON_REG_SIZE: u32 = 0x00100000;
pub const LOONGSON_REG_TOP: u32 = LOONGSON_REG_BASE + LOONGSON_REG_SIZE - 1;
pub const LOONGSON_LIO1_BASE: u32 = 0x1ff00000;
pub const LOONGSON_LIO1_SIZE: u32 = 0x00100000;
pub const LOONGSON_LIO1_TOP: u32 = LOONGSON_LIO1_BASE + LOONGSON_LIO1_SIZE - 1;
pub const LOONGSON_PCILO0_BASE: u32 = 0x10000000;
pub const LOONGSON_PCILO1_BASE: u32 = 0x14000000;
pub const LOONGSON_PCILO2_BASE: u32 = 0x18000000;
pub const LOONGSON_PCILO_BASE: u32 = LOONGSON_PCILO0_BASE;
pub const LOONGSON_PCILO_SIZE: u32 = 0x0c000000;
pub const LOONGSON_PCILO_TOP: u32 = LOONGSON_PCILO0_BASE + LOONGSON_PCILO_SIZE - 1;
pub const LOONGSON_PCICFG_BASE: u32 = 0x1fe80000;
pub const LOONGSON_PCICFG_SIZE: u32 = 0x00000800;
pub const LOONGSON_PCICFG_TOP: u32 = LOONGSON_PCICFG_BASE + LOONGSON_PCICFG_SIZE - 1;
pub const LOONGSON_PCIIO_BASE: u32 = 0x1fd00000;
pub const LOONGSON_PCIIO_SIZE: u32 = 0x00100000;
pub const LOONGSON_PCIIO_TOP: u32 = LOONGSON_PCIIO_BASE + LOONGSON_PCIIO_SIZE - 1;

pub const LOONGSON_PCICONFIGBASE: u32 = 0x00;
pub const LOONGSON_REGBASE: u32 = 0x100;

#[macro_export]
macro_rules! LOONGSON_REG { ($x:expr) => { (($x as usize) as *mut u32) }; }
#[macro_export]
macro_rules! LOONGSON_PCI_REG { ($x:expr) => { LOONGSON_REG!(LOONGSON_PCICONFIGBASE + ($x)) }; }

pub const LOONGSON_PCIDID: u32 = 0x00;
pub const LOONGSON_PCICMD: u32 = 0x04;
pub const LOONGSON_PCICLASS: u32 = 0x08;
pub const LOONGSON_PCILTIMER: u32 = 0x0c;
pub const LOONGSON_PCIBASE0: u32 = 0x10;
pub const LOONGSON_PCIBASE1: u32 = 0x14;
pub const LOONGSON_PCIBASE2: u32 = 0x18;
pub const LOONGSON_PCIBASE3: u32 = 0x1c;
pub const LOONGSON_PCIBASE4: u32 = 0x20;
pub const LOONGSON_PCIEXPRBASE: u32 = 0x30;
pub const LOONGSON_PCIINT: u32 = 0x3c;
pub const LOONGSON_PCI_ISR4C: u32 = 0x4c;
pub const LOONGSON_PCICMD_PERR_CLR: u32 = 0x80000000;
pub const LOONGSON_PCICMD_SERR_CLR: u32 = 0x40000000;
pub const LOONGSON_PCICMD_MABORT_CLR: u32 = 0x20000000;
pub const LOONGSON_PCICMD_MTABORT_CLR: u32 = 0x10000000;
pub const LOONGSON_PCICMD_TABORT_CLR: u32 = 0x08000000;
pub const LOONGSON_PCICMD_MPERR_CLR: u32 = 0x01000000;
pub const LOONGSON_PCICMD_PERRRESPEN: u32 = 0x40;
pub const LOONGSON_PCICMD_ASTEPEN: u32 = 0x80;
pub const LOONGSON_PCICMD_SERREN: u32 = 0x100;
pub const LOONGSON_PCILTIMER_BUSLATENCY: u32 = 0xff00;
pub const LOONGSON_PCILTIMER_BUSLATENCY_SHIFT: u32 = 8;

pub const LOONGSON_GENCFG_OFFSET: u32 = 4;
#[macro_export] macro_rules! LOONGSON_GENCFG { () => { LOONGSON_REG!(LOONGSON_REGBASE + LOONGSON_GENCFG_OFFSET) }; }
pub const LOONGSON_GENCFG_DEBUGMODE: u32 = 1;
pub const LOONGSON_GENCFG_SNOOPEN: u32 = 2;
pub const LOONGSON_GENCFG_CPUSELFRESET: u32 = 4;
pub const LOONGSON_GENCFG_FORCE_IRQA: u32 = 8;
pub const LOONGSON_GENCFG_IRQA_ISOUT: u32 = 0x10;
pub const LOONGSON_GENCFG_IRQA_FROM_INT1: u32 = 0x20;
pub const LOONGSON_GENCFG_BYTESWAP: u32 = 0x40;
pub const LOONGSON_GENCFG_UNCACHED: u32 = 0x80;
pub const LOONGSON_GENCFG_PREFETCHEN: u32 = 0x100;
pub const LOONGSON_GENCFG_WBEHINDEN: u32 = 0x200;
pub const LOONGSON_GENCFG_CACHEALG: u32 = 0xc00;
pub const LOONGSON_GENCFG_CACHEALG_SHIFT: u32 = 10;
pub const LOONGSON_GENCFG_PCIQUEUE: u32 = 0x1000;
pub const LOONGSON_GENCFG_CACHESTOP: u32 = 0x2000;
pub const LOONGSON_GENCFG_MSTRBYTESWAP: u32 = 0x4000;
pub const LOONGSON_GENCFG_BUSERREN: u32 = 0x8000;
pub const LOONGSON_GENCFG_NORETRYTIMEOUT: u32 = 0x10000;
pub const LOONGSON_GENCFG_SHORTCOPYTIMEOUT: u32 = 0x20000;

#[macro_export] macro_rules! LOONGSON_PCIMAP { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x10) }; }
#[macro_export] macro_rules! LOONGSON_PCIMEMBASECFG { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x14) }; }
#[macro_export] macro_rules! LOONGSON_PCIMAP_CFG { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x18) }; }
#[macro_export] macro_rules! LOONGSON_GPIODATA { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x1c) }; }
#[macro_export] macro_rules! LOONGSON_GPIOIE { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x20) }; }
#[macro_export] macro_rules! LOONGSON_INTEDGE { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x24) }; }
#[macro_export] macro_rules! LOONGSON_INTSTEER { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x28) }; }
#[macro_export] macro_rules! LOONGSON_INTPOL { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x2c) }; }
#[macro_export] macro_rules! LOONGSON_INTENSET { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x30) }; }
#[macro_export] macro_rules! LOONGSON_INTENCLR { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x34) }; }
#[macro_export] macro_rules! LOONGSON_INTEN { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x38) }; }
#[macro_export] macro_rules! LOONGSON_INTISR { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x3c) }; }

#[macro_export] macro_rules! LOONGSON_MEM_WIN_BASE_L { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x40) }; }
#[macro_export] macro_rules! LOONGSON_MEM_WIN_BASE_H { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x44) }; }
#[macro_export] macro_rules! LOONGSON_MEM_WIN_MASK_L { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x48) }; }
#[macro_export] macro_rules! LOONGSON_MEM_WIN_MASK_H { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x4c) }; }
#[macro_export] macro_rules! LOONGSON_PCI_HIT0_SEL_L { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x50) }; }
#[macro_export] macro_rules! LOONGSON_PCI_HIT0_SEL_H { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x54) }; }
#[macro_export] macro_rules! LOONGSON_PCI_HIT1_SEL_L { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x58) }; }
#[macro_export] macro_rules! LOONGSON_PCI_HIT1_SEL_H { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x5c) }; }
#[macro_export] macro_rules! LOONGSON_PCI_HIT2_SEL_L { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x60) }; }
#[macro_export] macro_rules! LOONGSON_PCI_HIT2_SEL_H { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x64) }; }
#[macro_export] macro_rules! LOONGSON_PXARB_CFG { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x68) }; }
#[macro_export] macro_rules! LOONGSON_PXARB_STATUS { () => { LOONGSON_REG!(LOONGSON_REGBASE + 0x6c) }; }
pub const LOONGSON_CHIPCFG: *mut core::ffi::c_void = 0x1fc00180usize as *mut core::ffi::c_void;

pub const LOONGSON_PCIMAP_PCIMAP_LO0: u32 = 0x3f;
pub const LOONGSON_PCIMAP_PCIMAP_LO0_SHIFT: u32 = 0;
pub const LOONGSON_PCIMAP_PCIMAP_LO1: u32 = 0xfc0;
pub const LOONGSON_PCIMAP_PCIMAP_LO1_SHIFT: u32 = 6;
pub const LOONGSON_PCIMAP_PCIMAP_LO2: u32 = 0x3f000;
pub const LOONGSON_PCIMAP_PCIMAP_LO2_SHIFT: u32 = 12;
pub const LOONGSON_PCIMAP_PCIMAP_2: u32 = 0x40000;

pub const LOONGSON_ICU_MBOXES: u32 = 0x0000000f;
pub const LOONGSON_ICU_MBOXES_SHIFT: u32 = 0;
pub const LOONGSON_ICU_DMARDY: u32 = 0x10;
pub const LOONGSON_ICU_DMAEMPTY: u32 = 0x20;
pub const LOONGSON_ICU_COPYRDY: u32 = 0x40;
pub const LOONGSON_ICU_COPYEMPTY: u32 = 0x80;
pub const LOONGSON_ICU_COPYERR: u32 = 0x100;
pub const LOONGSON_ICU_PCIIRQ: u32 = 0x200;
pub const LOONGSON_ICU_MASTERERR: u32 = 0x400;
pub const LOONGSON_ICU_SYSTEMERR: u32 = 0x800;
pub const LOONGSON_ICU_DRAMPERR: u32 = 0x1000;
pub const LOONGSON_ICU_RETRYERR: u32 = 0x2000;
pub const LOONGSON_ICU_GPIOS: u32 = 0x01ff0000;
pub const LOONGSON_ICU_GPIOS_SHIFT: u32 = 16;
pub const LOONGSON_ICU_GPINS: u32 = 0x7e000000;
pub const LOONGSON_ICU_GPINS_SHIFT: u32 = 25;
#[macro_export] macro_rules! LOONGSON_ICU_MBOX { ($n:expr) => { 1u32 << (LOONGSON_ICU_MBOXES_SHIFT + ($n)) }; }
#[macro_export] macro_rules! LOONGSON_ICU_GPIO { ($n:expr) => { 1u32 << (LOONGSON_ICU_GPIOS_SHIFT + ($n)) }; }
#[macro_export] macro_rules! LOONGSON_ICU_GPIN { ($n:expr) => { 1u32 << (LOONGSON_ICU_GPINS_SHIFT + ($n)) }; }

#[macro_export] macro_rules! LOONGSON_PCIMAP_WIN { ($win:expr, $addr:expr) => { ((($addr >> 26) & 0x3f) << ($win * 6)) }; }

#[cfg(CONFIG_CPU_SUPPORTS_CPUFREQ)]
extern "C" {
    pub static mut loongson2_clockmod_table: *mut core::ffi::c_void;
    pub fn loongson2_cpu_set_rate(rate_khz: usize) -> i32;
}

#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
pub const LOONGSON_ADDRWINCFG_BASE: usize = 0x3ff00000;
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
pub const LOONGSON_ADDRWINCFG_SIZE: usize = 0x180;
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
extern "C" { pub static mut _loongson_addrwincfg_base: usize; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! LOONGSON_ADDRWINCFG { ($offset:expr) => { (_loongson_addrwincfg_base + ($offset)) as *mut u64 }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
pub const ADDRWIN_WIN0: usize = 0;
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
pub const ADDRWIN_WIN1: usize = 1;
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
pub const ADDRWIN_WIN2: usize = 2;
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
pub const ADDRWIN_WIN3: usize = 3;
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
pub const ADDRWIN_MAP_DST_DDR: usize = 0;
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
pub const ADDRWIN_MAP_DST_PCI: usize = 1;
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
pub const ADDRWIN_MAP_DST_LIO: usize = 1;
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! CPU_WIN0_BASE { () => { LOONGSON_ADDRWINCFG!(0x00) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! CPU_WIN1_BASE { () => { LOONGSON_ADDRWINCFG!(0x08) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! CPU_WIN2_BASE { () => { LOONGSON_ADDRWINCFG!(0x10) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! CPU_WIN3_BASE { () => { LOONGSON_ADDRWINCFG!(0x18) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! CPU_WIN0_MASK { () => { LOONGSON_ADDRWINCFG!(0x20) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! CPU_WIN1_MASK { () => { LOONGSON_ADDRWINCFG!(0x28) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! CPU_WIN2_MASK { () => { LOONGSON_ADDRWINCFG!(0x30) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! CPU_WIN3_MASK { () => { LOONGSON_ADDRWINCFG!(0x38) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! CPU_WIN0_MMAP { () => { LOONGSON_ADDRWINCFG!(0x40) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! CPU_WIN1_MMAP { () => { LOONGSON_ADDRWINCFG!(0x48) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! CPU_WIN2_MMAP { () => { LOONGSON_ADDRWINCFG!(0x50) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! CPU_WIN3_MMAP { () => { LOONGSON_ADDRWINCFG!(0x58) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! PCIDMA_WIN0_BASE { () => { LOONGSON_ADDRWINCFG!(0x60) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! PCIDMA_WIN1_BASE { () => { LOONGSON_ADDRWINCFG!(0x68) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! PCIDMA_WIN2_BASE { () => { LOONGSON_ADDRWINCFG!(0x70) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! PCIDMA_WIN3_BASE { () => { LOONGSON_ADDRWINCFG!(0x78) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! PCIDMA_WIN0_MASK { () => { LOONGSON_ADDRWINCFG!(0x80) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! PCIDMA_WIN1_MASK { () => { LOONGSON_ADDRWINCFG!(0x88) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! PCIDMA_WIN2_MASK { () => { LOONGSON_ADDRWINCFG!(0x90) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! PCIDMA_WIN3_MASK { () => { LOONGSON_ADDRWINCFG!(0x98) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! PCIDMA_WIN0_MMAP { () => { LOONGSON_ADDRWINCFG!(0xa0) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! PCIDMA_WIN1_MMAP { () => { LOONGSON_ADDRWINCFG!(0xa8) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! PCIDMA_WIN2_MMAP { () => { LOONGSON_ADDRWINCFG!(0xb0) }; }
#[cfg(CONFIG_CPU_SUPPORTS_ADDRWINCFG)]
#[macro_export] macro_rules! PCIDMA_WIN3_MMAP { () => { LOONGSON_ADDRWINCFG!(0xb8) }; }

#[cfg(CONFIG_PCI)]
extern "C" { pub fn loongson2ef_pcibios_init(); }
#[cfg(not(CONFIG_PCI))]
#[inline] pub unsafe fn loongson2ef_pcibios_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
