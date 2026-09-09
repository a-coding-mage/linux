/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2009 Lemote, Inc.
 * Author: Wu Zhangjin <wuzhangjin@gmail.com>
 */

// C dependencies: linux/io.h, linux/init.h, linux/irq.h, boot_param.h

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoongsonFwInterface { LOONGSON_LEFI, LOONGSON_DTB }

#[repr(C)]
pub struct LoongsonSystemConfiguration {
    pub fw_interface: LoongsonFwInterface,
    pub nr_cpus: u32,
    pub nr_nodes: u32,
    pub cores_per_node: i32,
    pub cores_per_package: i32,
    pub boot_cpu_id: u16,
    pub reserved_cpus_mask: u16,
    pub cputype: LoongsonCpuType,
    pub bridgetype: LoongsonBridgeType,
    pub restart_addr: u64,
    pub poweroff_addr: u64,
    pub suspend_addr: u64,
    pub vgabios_addr: u64,
    pub dma_mask_bits: u32,
    pub workarounds: u64,
    pub early_config: Option<unsafe extern "C" fn()>,
}

// Supplied by boot_param.h.
pub type LoongsonCpuType = u32;
pub type LoongsonBridgeType = u32;
pub type PlatSmpOps = core::ffi::c_void;

unsafe extern "C" {
    pub fn mach_prepare_reboot();
    pub fn mach_prepare_shutdown();
    pub static mut cpu_clock_freq: u32;
    pub static mut memsize: u32;
    pub static mut highmemsize: u32;
    pub static loongson3_smp_ops: PlatSmpOps;
    pub fn prom_dtb_init_env();
    pub fn prom_lefi_init_env();
    pub fn szmem(node: core::ffi::c_uint);
    pub static mut loongson_fdt_blob: *mut core::ffi::c_void;
    pub fn mach_irq_dispatch(pending: core::ffi::c_uint);
    pub fn mach_i8259_irq() -> core::ffi::c_int;
    pub static mut loongson_sysconf: LoongsonSysconf;
}

#[repr(C)]
pub struct LoongsonSysconf { pub pci_io_base: u64 }

#[inline(always)]
pub unsafe fn delay() { for _x in 0..100000 { core::hint::spin_loop(); } }

pub const LOONGSON_FLASH_BASE: u64 = 0x1c000000;
pub const LOONGSON_FLASH_SIZE: u64 = 0x02000000;
pub const LOONGSON_FLASH_TOP: u64 = LOONGSON_FLASH_BASE + LOONGSON_FLASH_SIZE - 1;
pub const LOONGSON_LIO0_BASE: u64 = 0x1e000000;
pub const LOONGSON_LIO0_SIZE: u64 = 0x01C00000;
pub const LOONGSON_LIO0_TOP: u64 = LOONGSON_LIO0_BASE + LOONGSON_LIO0_SIZE - 1;
pub const LOONGSON_BOOT_BASE: u64 = 0x1fc00000;
pub const LOONGSON_BOOT_SIZE: u64 = 0x00100000;
pub const LOONGSON_BOOT_TOP: u64 = LOONGSON_BOOT_BASE + LOONGSON_BOOT_SIZE - 1;
pub const LOONGSON_REG_BASE: u64 = 0x1fe00000;
pub const LOONGSON_REG_SIZE: u64 = 0x00100000;
pub const LOONGSON_REG_TOP: u64 = LOONGSON_REG_BASE + LOONGSON_REG_SIZE - 1;
pub const LOONGSON3_REG_BASE: u64 = 0x3ff00000;
pub const LOONGSON3_REG_SIZE: u64 = 0x00100000;
pub const LOONGSON3_REG_TOP: u64 = LOONGSON3_REG_BASE + LOONGSON3_REG_SIZE - 1;
pub const LOONGSON_LIO1_BASE: u64 = 0x1ff00000;
pub const LOONGSON_LIO1_SIZE: u64 = 0x00100000;
pub const LOONGSON_LIO1_TOP: u64 = LOONGSON_LIO1_BASE + LOONGSON_LIO1_SIZE - 1;
pub const LOONGSON_PCILO0_BASE: u64 = 0x10000000;
pub const LOONGSON_PCILO1_BASE: u64 = 0x14000000;
pub const LOONGSON_PCILO2_BASE: u64 = 0x18000000;
pub const LOONGSON_PCILO_BASE: u64 = LOONGSON_PCILO0_BASE;
pub const LOONGSON_PCILO_SIZE: u64 = 0x0c000000;
pub const LOONGSON_PCILO_TOP: u64 = LOONGSON_PCILO0_BASE + LOONGSON_PCILO_SIZE - 1;
pub const LOONGSON_PCICFG_BASE: u64 = 0x1fe80000;
pub const LOONGSON_PCICFG_SIZE: u64 = 0x00000800;
pub const LOONGSON_PCICFG_TOP: u64 = LOONGSON_PCICFG_BASE + LOONGSON_PCICFG_SIZE - 1;
pub const LOONGSON_PCICONFIGBASE: u32 = 0x00;
pub const LOONGSON_REGBASE: u32 = 0x100;
pub const LOONGSON_PCIIO_SIZE: u64 = 0x00100000;
pub const LOONGSON_PCIIO_BASE: u64 = 0; // loongson_sysconf.pci_io_base
pub const LOONGSON_PCIIO_TOP: u64 = LOONGSON_PCIIO_BASE + LOONGSON_PCIIO_SIZE - 1;

// Address conversion helpers are supplied by the target architecture.
extern "C" { fn CKSEG1ADDR(x: u64) -> *mut u32; fn TO_UNCAC(x: u64) -> *mut u8; }

#[inline(always)] pub unsafe fn loongson_reg(x: u32) -> u32 { core::ptr::read_volatile(CKSEG1ADDR(LOONGSON_REG_BASE + x as u64)) }
#[inline(always)] pub unsafe fn loongson3_reg8(base: u64, x: u32) -> u8 { core::ptr::read_volatile(TO_UNCAC(base).add(x as usize)) }
#[inline(always)] pub unsafe fn loongson3_reg32(base: u64, x: u32) -> u32 { core::ptr::read_volatile(TO_UNCAC(base).add(x as usize) as *mut u32) }

pub const LOONGSON_PCI_REG_BASE: u32 = LOONGSON_PCICONFIGBASE;
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
#[inline(always)] pub unsafe fn LOONGSON_PCI_REG(x:u32)->u32 { loongson_reg(LOONGSON_PCICONFIGBASE+x) }
#[inline(always)] pub unsafe fn LOONGSON_REG(x:u32)->u32 { loongson_reg(x) }
#[inline(always)] pub unsafe fn LOONGSON3_REG8(base:u64,x:u32)->u8 { loongson3_reg8(base,x) }
#[inline(always)] pub unsafe fn LOONGSON3_REG32(base:u64,x:u32)->u32 { loongson3_reg32(base,x) }

pub const LOONGSON_PCICMD_PERR_CLR:u32=0x80000000; pub const LOONGSON_PCICMD_SERR_CLR:u32=0x40000000;
pub const LOONGSON_PCICMD_MABORT_CLR:u32=0x20000000; pub const LOONGSON_PCICMD_MTABORT_CLR:u32=0x10000000;
pub const LOONGSON_PCICMD_TABORT_CLR:u32=0x08000000; pub const LOONGSON_PCICMD_MPERR_CLR:u32=0x01000000;
pub const LOONGSON_PCICMD_PERRRESPEN:u32=0x40; pub const LOONGSON_PCICMD_ASTEPEN:u32=0x80;
pub const LOONGSON_PCICMD_SERREN:u32=0x100; pub const LOONGSON_PCILTIMER_BUSLATENCY:u32=0xff00;
pub const LOONGSON_PCILTIMER_BUSLATENCY_SHIFT:u32=8;

pub const LOONGSON_PCIMAP:u32=0x110; pub const LOONGSON_PCIMEMBASECFG:u32=0x114; pub const LOONGSON_PCIMAP_CFG:u32=0x118;
pub const LOONGSON_GPIODATA:u32=0x11c; pub const LOONGSON_GPIOIE:u32=0x120;
pub const LOONGSON_INTEDGE:u32=0x124; pub const LOONGSON_INTSTEER:u32=0x128; pub const LOONGSON_INTPOL:u32=0x12c;
pub const LOONGSON_INTENSET:u32=0x130; pub const LOONGSON_INTENCLR:u32=0x134; pub const LOONGSON_INTEN:u32=0x138; pub const LOONGSON_INTISR:u32=0x13c;
pub const LOONGSON_ICU_MBOXES:u32=0xf; pub const LOONGSON_ICU_MBOXES_SHIFT:u32=0; pub const LOONGSON_ICU_DMARDY:u32=0x10; pub const LOONGSON_ICU_DMAEMPTY:u32=0x20; pub const LOONGSON_ICU_COPYRDY:u32=0x40; pub const LOONGSON_ICU_COPYEMPTY:u32=0x80; pub const LOONGSON_ICU_COPYERR:u32=0x100; pub const LOONGSON_ICU_PCIIRQ:u32=0x200; pub const LOONGSON_ICU_MASTERERR:u32=0x400; pub const LOONGSON_ICU_SYSTEMERR:u32=0x800; pub const LOONGSON_ICU_DRAMPERR:u32=0x1000; pub const LOONGSON_ICU_RETRYERR:u32=0x2000; pub const LOONGSON_ICU_GPIOS:u32=0x01ff0000; pub const LOONGSON_ICU_GPIOS_SHIFT:u32=16; pub const LOONGSON_ICU_GPINS:u32=0x7e000000; pub const LOONGSON_ICU_GPINS_SHIFT:u32=25;
pub const LOONGSON_MEM_WIN_BASE_L:u32=0x140; pub const LOONGSON_MEM_WIN_BASE_H:u32=0x144; pub const LOONGSON_MEM_WIN_MASK_L:u32=0x148; pub const LOONGSON_MEM_WIN_MASK_H:u32=0x14c;
pub const LOONGSON_PCI_HIT0_SEL_L:u32=0x150; pub const LOONGSON_PCI_HIT0_SEL_H:u32=0x154; pub const LOONGSON_PCI_HIT1_SEL_L:u32=0x158; pub const LOONGSON_PCI_HIT1_SEL_H:u32=0x15c; pub const LOONGSON_PCI_HIT2_SEL_L:u32=0x160; pub const LOONGSON_PCI_HIT2_SEL_H:u32=0x164;
pub const LOONGSON_PXARB_CFG:u32=0x168; pub const LOONGSON_PXARB_STATUS:u32=0x16c;
pub const LOONGSON_LIO0_TOP_COMMENT: &str = "28M";

#[inline(always)] pub const fn loongson_icu_mbox(n:u32)->u32 { 1 << (LOONGSON_ICU_MBOXES_SHIFT+n) }
#[inline(always)] pub const fn loongson_icu_gpio(n:u32)->u32 { 1 << (LOONGSON_ICU_GPIOS_SHIFT+n) }
#[inline(always)] pub const fn loongson_icu_gpin(n:u32)->u32 { 1 << (LOONGSON_ICU_GPINS_SHIFT+n) }

pub const LOONGSON_GENCFG_OFFSET:u32=4;
pub const LOONGSON_GENCFG_DEBUGMODE:u32=1; pub const LOONGSON_GENCFG_SNOOPEN:u32=2; pub const LOONGSON_GENCFG_CPUSELFRESET:u32=4;
pub const LOONGSON_GENCFG_FORCE_IRQA:u32=8; pub const LOONGSON_GENCFG_IRQA_ISOUT:u32=0x10; pub const LOONGSON_GENCFG_IRQA_FROM_INT1:u32=0x20; pub const LOONGSON_GENCFG_BYTESWAP:u32=0x40;
pub const LOONGSON_GENCFG_UNCACHED:u32=0x80; pub const LOONGSON_GENCFG_PREFETCHEN:u32=0x100; pub const LOONGSON_GENCFG_WBEHINDEN:u32=0x200; pub const LOONGSON_GENCFG_CACHEALG:u32=0xc00; pub const LOONGSON_GENCFG_CACHEALG_SHIFT:u32=10; pub const LOONGSON_GENCFG_PCIQUEUE:u32=0x1000; pub const LOONGSON_GENCFG_CACHESTOP:u32=0x2000; pub const LOONGSON_GENCFG_MSTRBYTESWAP:u32=0x4000; pub const LOONGSON_GENCFG_BUSERREN:u32=0x8000; pub const LOONGSON_GENCFG_NORETRYTIMEOUT:u32=0x10000; pub const LOONGSON_GENCFG_SHORTCOPYTIMEOUT:u32=0x20000;

pub const MAX_PACKAGES: usize = 4;
pub const LOONGSON_PCIMAP_PCIMAP_LO0:u32=0x3f; pub const LOONGSON_PCIMAP_PCIMAP_LO0_SHIFT:u32=0;
pub const LOONGSON_PCIMAP_PCIMAP_LO1:u32=0xfc0; pub const LOONGSON_PCIMAP_PCIMAP_LO1_SHIFT:u32=6;
pub const LOONGSON_PCIMAP_PCIMAP_LO2:u32=0x3f000; pub const LOONGSON_PCIMAP_PCIMAP_LO2_SHIFT:u32=12; pub const LOONGSON_PCIMAP_PCIMAP_2:u32=0x40000;
pub unsafe fn loongson_pci_map_win(win:u32, addr:u32)->u32 { ((addr >> 26) & LOONGSON_PCIMAP_PCIMAP_LO0) << (win * 6) }

pub static mut loongson_chipcfg: [u64; MAX_PACKAGES] = [0; MAX_PACKAGES];
pub static mut loongson_chiptemp: [u64; MAX_PACKAGES] = [0; MAX_PACKAGES];
pub static mut loongson_freqctrl: [u64; MAX_PACKAGES] = [0; MAX_PACKAGES];
#[inline(always)] pub unsafe fn loongson_chipcfg_read(id: usize)->u32 { core::ptr::read_volatile(loongson_chipcfg[id] as *const u32) }
#[inline(always)] pub unsafe fn loongson_chiptemp_read(id: usize)->u32 { core::ptr::read_volatile(loongson_chiptemp[id] as *const u32) }
#[inline(always)] pub unsafe fn loongson_freqctrl_read(id: usize)->u32 { core::ptr::read_volatile(loongson_freqctrl[id] as *const u32) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
