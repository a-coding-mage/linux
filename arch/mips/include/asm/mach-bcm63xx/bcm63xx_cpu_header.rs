// Faithful Rust translation of bcm63xx_cpu.h
#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

/* SPDX-License-Identifier: GPL-2.0 */
// conditional header guard: BCM63XX_CPU_H_
// header guard

// dependency: <linux/types.h>
// dependency: <linux/init.h>

/*
 * Macro to fetch bcm63xx cpu id and revision, should be optimized at
 * compile time if only one CPU support is enabled (idea stolen from
 * arm mach-types)
 */
pub const BCM3368_CPU_ID: u64 = 0x3368;
pub const BCM6328_CPU_ID: u64 = 0x6328;
pub const BCM6338_CPU_ID: u64 = 0x6338;
pub const BCM6345_CPU_ID: u64 = 0x6345;
pub const BCM6348_CPU_ID: u64 = 0x6348;
pub const BCM6358_CPU_ID: u64 = 0x6358;
pub const BCM6362_CPU_ID: u64 = 0x6362;
pub const BCM6368_CPU_ID: u64 = 0x6368;

extern "C" { pub fn bcm63xx_cpu_init(); }
extern "C" { pub fn bcm63xx_get_cpu_rev() -> u8; }
extern "C" { pub fn bcm63xx_get_cpu_freq() -> u32; }

pub unsafe fn __bcm63xx_get_cpu_id(cpu_id: u16) -> u16
{
	match cpu_id {
// build-time condition: CONFIG_BCM63XX_CPU_3368
		BCM3368_CPU_ID => {}
// end build-time condition

// build-time condition: CONFIG_BCM63XX_CPU_6328
		BCM6328_CPU_ID => {}
// end build-time condition

// build-time condition: CONFIG_BCM63XX_CPU_6338
		BCM6338_CPU_ID => {}
// end build-time condition

// build-time condition: CONFIG_BCM63XX_CPU_6345
		BCM6345_CPU_ID => {}
// end build-time condition

// build-time condition: CONFIG_BCM63XX_CPU_6348
		BCM6348_CPU_ID => {}
// end build-time condition

// build-time condition: CONFIG_BCM63XX_CPU_6358
		BCM6358_CPU_ID => {}
// end build-time condition

// build-time condition: CONFIG_BCM63XX_CPU_6362
		BCM6362_CPU_ID => {}
// end build-time condition

// build-time condition: CONFIG_BCM63XX_CPU_6368
		BCM6368_CPU_ID => {}
// end build-time condition
		return cpu_id;
	_ => {
		core::hint::unreachable_unchecked();
	}

	cpu_id
}

extern "C" { pub static mut bcm63xx_cpu_id: u16; }

pub unsafe fn bcm63xx_get_cpu_id() -> u16
{
	let cpu_id: u16 = bcm63xx_cpu_id;

	return __bcm63xx_get_cpu_id(cpu_id);
}

pub const fn BCMCPU_IS_3368(: usize) -> usize { (bcm63xx_get_cpu_id() == BCM3368_CPU_ID) }
pub const fn BCMCPU_IS_6328(: usize) -> usize { (bcm63xx_get_cpu_id() == BCM6328_CPU_ID) }
pub const fn BCMCPU_IS_6338(: usize) -> usize { (bcm63xx_get_cpu_id() == BCM6338_CPU_ID) }
pub const fn BCMCPU_IS_6345(: usize) -> usize { (bcm63xx_get_cpu_id() == BCM6345_CPU_ID) }
pub const fn BCMCPU_IS_6348(: usize) -> usize { (bcm63xx_get_cpu_id() == BCM6348_CPU_ID) }
pub const fn BCMCPU_IS_6358(: usize) -> usize { (bcm63xx_get_cpu_id() == BCM6358_CPU_ID) }
pub const fn BCMCPU_IS_6362(: usize) -> usize { (bcm63xx_get_cpu_id() == BCM6362_CPU_ID) }
pub const fn BCMCPU_IS_6368(: usize) -> usize { (bcm63xx_get_cpu_id() == BCM6368_CPU_ID) }

/*
 * While registers sets are (mostly) the same across 63xx CPU, base
 * address of these sets do change.
 */
#[repr(C)]\npub bcm63xx_regs_set {
	RSET_DSL_LMEM = 0,
	RSET_PERF,
	RSET_TIMER,
	RSET_WDT,
	RSET_UART0,
	RSET_UART1,
	RSET_GPIO,
	RSET_SPI,
	RSET_HSSPI,
	RSET_UDC0,
	RSET_OHCI0,
	RSET_OHCI_PRIV,
	RSET_USBH_PRIV,
	RSET_USBD,
	RSET_USBDMA,
	RSET_MPI,
	RSET_PCMCIA,
	RSET_PCIE,
	RSET_DSL,
	RSET_ENET0,
	RSET_ENET1,
	RSET_ENETDMA,
	RSET_ENETDMAC,
	RSET_ENETDMAS,
	RSET_ENETSW,
	RSET_EHCI0,
	RSET_SDRAM,
	RSET_MEMC,
	RSET_DDR,
	RSET_M2M,
	RSET_ATM,
	RSET_XTM,
	RSET_XTMDMA,
	RSET_XTMDMAC,
	RSET_XTMDMAS,
	RSET_PCM,
	RSET_PCMDMA,
	RSET_PCMDMAC,
	RSET_PCMDMAS,
	RSET_RNG,
	RSET_MISC
};

pub const RSET_DSL_LMEM_SIZE: u64 = (64 * 1024 * 4);
pub const RSET_DSL_SIZE: u64 = 4096;
pub const RSET_WDT_SIZE: u64 = 12;
pub const BCM_6338_RSET_SPI_SIZE: u64 = 64;
pub const BCM_6348_RSET_SPI_SIZE: u64 = 64;
pub const BCM_6358_RSET_SPI_SIZE: u64 = 1804;
pub const BCM_6368_RSET_SPI_SIZE: u64 = 1804;
pub const RSET_ENET_SIZE: u64 = 2048;
pub const RSET_ENETDMA_SIZE: u64 = 256;
pub const RSET_6345_ENETDMA_SIZE: u64 = 64;
pub const fn RSET_ENETDMAC_SIZE(chans: usize) -> usize { (16 * (chans)) }
pub const fn RSET_ENETDMAS_SIZE(chans: usize) -> usize { (16 * (chans)) }
pub const RSET_ENETSW_SIZE: u64 = 65536;
pub const RSET_UART_SIZE: u64 = 24;
pub const RSET_HSSPI_SIZE: u64 = 1536;
pub const RSET_UDC_SIZE: u64 = 256;
pub const RSET_OHCI_SIZE: u64 = 256;
pub const RSET_EHCI_SIZE: u64 = 256;
pub const RSET_USBD_SIZE: u64 = 256;
pub const RSET_USBDMA_SIZE: u64 = 1280;
pub const RSET_PCMCIA_SIZE: u64 = 12;
pub const RSET_M2M_SIZE: u64 = 256;
pub const RSET_ATM_SIZE: u64 = 4096;
pub const RSET_XTM_SIZE: u64 = 10240;
pub const RSET_XTMDMA_SIZE: u64 = 256;
pub const fn RSET_XTMDMAC_SIZE(chans: usize) -> usize { (16 * (chans)) }
pub const fn RSET_XTMDMAS_SIZE(chans: usize) -> usize { (16 * (chans)) }
pub const RSET_RNG_SIZE: u64 = 20;

/*
 * 3368 register sets base address
 */
pub const BCM_3368_DSL_LMEM_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_PERF_BASE: u64 = (0xfff8c000);
pub const BCM_3368_TIMER_BASE: u64 = (0xfff8c040);
pub const BCM_3368_WDT_BASE: u64 = (0xfff8c080);
pub const BCM_3368_UART0_BASE: u64 = (0xfff8c100);
pub const BCM_3368_UART1_BASE: u64 = (0xfff8c120);
pub const BCM_3368_GPIO_BASE: u64 = (0xfff8c080);
pub const BCM_3368_SPI_BASE: u64 = (0xfff8c800);
pub const BCM_3368_HSSPI_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_UDC0_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_USBDMA_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_OHCI0_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_OHCI_PRIV_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_USBH_PRIV_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_USBD_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_MPI_BASE: u64 = (0xfff80000);
pub const BCM_3368_PCMCIA_BASE: u64 = (0xfff80054);
pub const BCM_3368_PCIE_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_SDRAM_REGS_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_DSL_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_UBUS_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_ENET0_BASE: u64 = (0xfff98000);
pub const BCM_3368_ENET1_BASE: u64 = (0xfff98800);
pub const BCM_3368_ENETDMA_BASE: u64 = (0xfff99800);
pub const BCM_3368_ENETDMAC_BASE: u64 = (0xfff99900);
pub const BCM_3368_ENETDMAS_BASE: u64 = (0xfff99a00);
pub const BCM_3368_ENETSW_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_EHCI0_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_SDRAM_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_MEMC_BASE: u64 = (0xfff84000);
pub const BCM_3368_DDR_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_M2M_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_ATM_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_XTM_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_XTMDMA_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_XTMDMAC_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_XTMDMAS_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_PCM_BASE: u64 = (0xfff9c200);
pub const BCM_3368_PCMDMA_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_PCMDMAC_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_PCMDMAS_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_RNG_BASE: u64 = (0xdeadbeef);
pub const BCM_3368_MISC_BASE: u64 = (0xdeadbeef);

/*
 * 6328 register sets base address
 */
pub const BCM_6328_DSL_LMEM_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_PERF_BASE: u64 = (0xb0000000);
pub const BCM_6328_TIMER_BASE: u64 = (0xb0000040);
pub const BCM_6328_WDT_BASE: u64 = (0xb000005c);
pub const BCM_6328_UART0_BASE: u64 = (0xb0000100);
pub const BCM_6328_UART1_BASE: u64 = (0xb0000120);
pub const BCM_6328_GPIO_BASE: u64 = (0xb0000080);
pub const BCM_6328_SPI_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_HSSPI_BASE: u64 = (0xb0001000);
pub const BCM_6328_UDC0_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_USBDMA_BASE: u64 = (0xb000c000);
pub const BCM_6328_OHCI0_BASE: u64 = (0xb0002600);
pub const BCM_6328_OHCI_PRIV_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_USBH_PRIV_BASE: u64 = (0xb0002700);
pub const BCM_6328_USBD_BASE: u64 = (0xb0002400);
pub const BCM_6328_MPI_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_PCMCIA_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_PCIE_BASE: u64 = (0xb0e40000);
pub const BCM_6328_SDRAM_REGS_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_DSL_BASE: u64 = (0xb0001900);
pub const BCM_6328_UBUS_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_ENET0_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_ENET1_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_ENETDMA_BASE: u64 = (0xb000d800);
pub const BCM_6328_ENETDMAC_BASE: u64 = (0xb000da00);
pub const BCM_6328_ENETDMAS_BASE: u64 = (0xb000dc00);
pub const BCM_6328_ENETSW_BASE: u64 = (0xb0e00000);
pub const BCM_6328_EHCI0_BASE: u64 = (0xb0002500);
pub const BCM_6328_SDRAM_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_MEMC_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_DDR_BASE: u64 = (0xb0003000);
pub const BCM_6328_M2M_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_ATM_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_XTM_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_XTMDMA_BASE: u64 = (0xb000b800);
pub const BCM_6328_XTMDMAC_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_XTMDMAS_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_PCM_BASE: u64 = (0xb000a800);
pub const BCM_6328_PCMDMA_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_PCMDMAC_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_PCMDMAS_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_RNG_BASE: u64 = (0xdeadbeef);
pub const BCM_6328_MISC_BASE: u64 = (0xb0001800);
pub const BCM_6328_OTP_BASE: u64 = (0xb0000600);

/*
 * 6338 register sets base address
 */
pub const BCM_6338_DSL_LMEM_BASE: u64 = (0xfff00000);
pub const BCM_6338_PERF_BASE: u64 = (0xfffe0000);
pub const BCM_6338_BB_BASE: u64 = (0xfffe0100);
pub const BCM_6338_TIMER_BASE: u64 = (0xfffe0200);
pub const BCM_6338_WDT_BASE: u64 = (0xfffe021c);
pub const BCM_6338_UART0_BASE: u64 = (0xfffe0300);
pub const BCM_6338_UART1_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_GPIO_BASE: u64 = (0xfffe0400);
pub const BCM_6338_SPI_BASE: u64 = (0xfffe0c00);
pub const BCM_6338_HSSPI_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_UDC0_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_USBDMA_BASE: u64 = (0xfffe2400);
pub const BCM_6338_OHCI0_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_OHCI_PRIV_BASE: u64 = (0xfffe3000);
pub const BCM_6338_USBH_PRIV_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_USBD_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_MPI_BASE: u64 = (0xfffe3160);
pub const BCM_6338_PCMCIA_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_PCIE_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_SDRAM_REGS_BASE: u64 = (0xfffe3100);
pub const BCM_6338_DSL_BASE: u64 = (0xfffe1000);
pub const BCM_6338_UBUS_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_ENET0_BASE: u64 = (0xfffe2800);
pub const BCM_6338_ENET1_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_ENETDMA_BASE: u64 = (0xfffe2400);
pub const BCM_6338_ENETDMAC_BASE: u64 = (0xfffe2500);
pub const BCM_6338_ENETDMAS_BASE: u64 = (0xfffe2600);
pub const BCM_6338_ENETSW_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_EHCI0_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_SDRAM_BASE: u64 = (0xfffe3100);
pub const BCM_6338_MEMC_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_DDR_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_M2M_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_ATM_BASE: u64 = (0xfffe2000);
pub const BCM_6338_XTM_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_XTMDMA_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_XTMDMAC_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_XTMDMAS_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_PCM_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_PCMDMA_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_PCMDMAC_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_PCMDMAS_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_RNG_BASE: u64 = (0xdeadbeef);
pub const BCM_6338_MISC_BASE: u64 = (0xdeadbeef);

/*
 * 6345 register sets base address
 */
pub const BCM_6345_DSL_LMEM_BASE: u64 = (0xfff00000);
pub const BCM_6345_PERF_BASE: u64 = (0xfffe0000);
pub const BCM_6345_BB_BASE: u64 = (0xfffe0100);
pub const BCM_6345_TIMER_BASE: u64 = (0xfffe0200);
pub const BCM_6345_WDT_BASE: u64 = (0xfffe021c);
pub const BCM_6345_UART0_BASE: u64 = (0xfffe0300);
pub const BCM_6345_UART1_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_GPIO_BASE: u64 = (0xfffe0400);
pub const BCM_6345_SPI_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_HSSPI_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_UDC0_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_USBDMA_BASE: u64 = (0xfffe2800);
pub const BCM_6345_ENET0_BASE: u64 = (0xfffe1800);
pub const BCM_6345_ENETDMA_BASE: u64 = (0xfffe2800);
pub const BCM_6345_ENETDMAC_BASE: u64 = (0xfffe2840);
pub const BCM_6345_ENETDMAS_BASE: u64 = (0xfffe2a00);
pub const BCM_6345_ENETSW_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_PCMCIA_BASE: u64 = (0xfffe2028);
pub const BCM_6345_MPI_BASE: u64 = (0xfffe2000);
pub const BCM_6345_PCIE_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_OHCI0_BASE: u64 = (0xfffe2100);
pub const BCM_6345_OHCI_PRIV_BASE: u64 = (0xfffe2200);
pub const BCM_6345_USBH_PRIV_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_USBD_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_SDRAM_REGS_BASE: u64 = (0xfffe2300);
pub const BCM_6345_DSL_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_UBUS_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_ENET1_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_EHCI0_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_SDRAM_BASE: u64 = (0xfffe2300);
pub const BCM_6345_MEMC_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_DDR_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_M2M_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_ATM_BASE: u64 = (0xfffe4000);
pub const BCM_6345_XTM_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_XTMDMA_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_XTMDMAC_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_XTMDMAS_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_PCM_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_PCMDMA_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_PCMDMAC_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_PCMDMAS_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_RNG_BASE: u64 = (0xdeadbeef);
pub const BCM_6345_MISC_BASE: u64 = (0xdeadbeef);

/*
 * 6348 register sets base address
 */
pub const BCM_6348_DSL_LMEM_BASE: u64 = (0xfff00000);
pub const BCM_6348_PERF_BASE: u64 = (0xfffe0000);
pub const BCM_6348_TIMER_BASE: u64 = (0xfffe0200);
pub const BCM_6348_WDT_BASE: u64 = (0xfffe021c);
pub const BCM_6348_UART0_BASE: u64 = (0xfffe0300);
pub const BCM_6348_UART1_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_GPIO_BASE: u64 = (0xfffe0400);
pub const BCM_6348_SPI_BASE: u64 = (0xfffe0c00);
pub const BCM_6348_HSSPI_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_UDC0_BASE: u64 = (0xfffe1000);
pub const BCM_6348_USBDMA_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_OHCI0_BASE: u64 = (0xfffe1b00);
pub const BCM_6348_OHCI_PRIV_BASE: u64 = (0xfffe1c00);
pub const BCM_6348_USBH_PRIV_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_USBD_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_MPI_BASE: u64 = (0xfffe2000);
pub const BCM_6348_PCMCIA_BASE: u64 = (0xfffe2054);
pub const BCM_6348_PCIE_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_SDRAM_REGS_BASE: u64 = (0xfffe2300);
pub const BCM_6348_M2M_BASE: u64 = (0xfffe2800);
pub const BCM_6348_DSL_BASE: u64 = (0xfffe3000);
pub const BCM_6348_ENET0_BASE: u64 = (0xfffe6000);
pub const BCM_6348_ENET1_BASE: u64 = (0xfffe6800);
pub const BCM_6348_ENETDMA_BASE: u64 = (0xfffe7000);
pub const BCM_6348_ENETDMAC_BASE: u64 = (0xfffe7100);
pub const BCM_6348_ENETDMAS_BASE: u64 = (0xfffe7200);
pub const BCM_6348_ENETSW_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_EHCI0_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_SDRAM_BASE: u64 = (0xfffe2300);
pub const BCM_6348_MEMC_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_DDR_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_ATM_BASE: u64 = (0xfffe4000);
pub const BCM_6348_XTM_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_XTMDMA_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_XTMDMAC_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_XTMDMAS_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_PCM_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_PCMDMA_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_PCMDMAC_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_PCMDMAS_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_RNG_BASE: u64 = (0xdeadbeef);
pub const BCM_6348_MISC_BASE: u64 = (0xdeadbeef);

/*
 * 6358 register sets base address
 */
pub const BCM_6358_DSL_LMEM_BASE: u64 = (0xfff00000);
pub const BCM_6358_PERF_BASE: u64 = (0xfffe0000);
pub const BCM_6358_TIMER_BASE: u64 = (0xfffe0040);
pub const BCM_6358_WDT_BASE: u64 = (0xfffe005c);
pub const BCM_6358_UART0_BASE: u64 = (0xfffe0100);
pub const BCM_6358_UART1_BASE: u64 = (0xfffe0120);
pub const BCM_6358_GPIO_BASE: u64 = (0xfffe0080);
pub const BCM_6358_SPI_BASE: u64 = (0xfffe0800);
pub const BCM_6358_HSSPI_BASE: u64 = (0xdeadbeef);
pub const BCM_6358_UDC0_BASE: u64 = (0xfffe0800);
pub const BCM_6358_USBDMA_BASE: u64 = (0xdeadbeef);
pub const BCM_6358_OHCI0_BASE: u64 = (0xfffe1400);
pub const BCM_6358_OHCI_PRIV_BASE: u64 = (0xdeadbeef);
pub const BCM_6358_USBH_PRIV_BASE: u64 = (0xfffe1500);
pub const BCM_6358_USBD_BASE: u64 = (0xdeadbeef);
pub const BCM_6358_MPI_BASE: u64 = (0xfffe1000);
pub const BCM_6358_PCMCIA_BASE: u64 = (0xfffe1054);
pub const BCM_6358_PCIE_BASE: u64 = (0xdeadbeef);
pub const BCM_6358_SDRAM_REGS_BASE: u64 = (0xfffe2300);
pub const BCM_6358_M2M_BASE: u64 = (0xdeadbeef);
pub const BCM_6358_DSL_BASE: u64 = (0xfffe3000);
pub const BCM_6358_ENET0_BASE: u64 = (0xfffe4000);
pub const BCM_6358_ENET1_BASE: u64 = (0xfffe4800);
pub const BCM_6358_ENETDMA_BASE: u64 = (0xfffe5000);
pub const BCM_6358_ENETDMAC_BASE: u64 = (0xfffe5100);
pub const BCM_6358_ENETDMAS_BASE: u64 = (0xfffe5200);
pub const BCM_6358_ENETSW_BASE: u64 = (0xdeadbeef);
pub const BCM_6358_EHCI0_BASE: u64 = (0xfffe1300);
pub const BCM_6358_SDRAM_BASE: u64 = (0xdeadbeef);
pub const BCM_6358_MEMC_BASE: u64 = (0xfffe1200);
pub const BCM_6358_DDR_BASE: u64 = (0xfffe12a0);
pub const BCM_6358_ATM_BASE: u64 = (0xfffe2000);
pub const BCM_6358_XTM_BASE: u64 = (0xdeadbeef);
pub const BCM_6358_XTMDMA_BASE: u64 = (0xdeadbeef);
pub const BCM_6358_XTMDMAC_BASE: u64 = (0xdeadbeef);
pub const BCM_6358_XTMDMAS_BASE: u64 = (0xdeadbeef);
pub const BCM_6358_PCM_BASE: u64 = (0xfffe1600);
pub const BCM_6358_PCMDMA_BASE: u64 = (0xfffe1800);
pub const BCM_6358_PCMDMAC_BASE: u64 = (0xfffe1900);
pub const BCM_6358_PCMDMAS_BASE: u64 = (0xfffe1a00);
pub const BCM_6358_RNG_BASE: u64 = (0xdeadbeef);
pub const BCM_6358_MISC_BASE: u64 = (0xdeadbeef);


/*
 * 6362 register sets base address
 */
pub const BCM_6362_DSL_LMEM_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_PERF_BASE: u64 = (0xb0000000);
pub const BCM_6362_TIMER_BASE: u64 = (0xb0000040);
pub const BCM_6362_WDT_BASE: u64 = (0xb000005c);
pub const BCM_6362_UART0_BASE: u64 = (0xb0000100);
pub const BCM_6362_UART1_BASE: u64 = (0xb0000120);
pub const BCM_6362_GPIO_BASE: u64 = (0xb0000080);
pub const BCM_6362_SPI_BASE: u64 = (0xb0000800);
pub const BCM_6362_HSSPI_BASE: u64 = (0xb0001000);
pub const BCM_6362_UDC0_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_USBDMA_BASE: u64 = (0xb000c000);
pub const BCM_6362_OHCI0_BASE: u64 = (0xb0002600);
pub const BCM_6362_OHCI_PRIV_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_USBH_PRIV_BASE: u64 = (0xb0002700);
pub const BCM_6362_USBD_BASE: u64 = (0xb0002400);
pub const BCM_6362_MPI_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_PCMCIA_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_PCIE_BASE: u64 = (0xb0e40000);
pub const BCM_6362_SDRAM_REGS_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_DSL_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_UBUS_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_ENET0_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_ENET1_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_ENETDMA_BASE: u64 = (0xb000d800);
pub const BCM_6362_ENETDMAC_BASE: u64 = (0xb000da00);
pub const BCM_6362_ENETDMAS_BASE: u64 = (0xb000dc00);
pub const BCM_6362_ENETSW_BASE: u64 = (0xb0e00000);
pub const BCM_6362_EHCI0_BASE: u64 = (0xb0002500);
pub const BCM_6362_SDRAM_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_MEMC_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_DDR_BASE: u64 = (0xb0003000);
pub const BCM_6362_M2M_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_ATM_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_XTM_BASE: u64 = (0xb0007800);
pub const BCM_6362_XTMDMA_BASE: u64 = (0xb000b800);
pub const BCM_6362_XTMDMAC_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_XTMDMAS_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_PCM_BASE: u64 = (0xb000a800);
pub const BCM_6362_PCMDMA_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_PCMDMAC_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_PCMDMAS_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_RNG_BASE: u64 = (0xdeadbeef);
pub const BCM_6362_MISC_BASE: u64 = (0xb0001800);

pub const BCM_6362_NAND_REG_BASE: u64 = (0xb0000200);
pub const BCM_6362_NAND_CACHE_BASE: u64 = (0xb0000600);
pub const BCM_6362_LED_BASE: u64 = (0xb0001900);
pub const BCM_6362_IPSEC_BASE: u64 = (0xb0002800);
pub const BCM_6362_IPSEC_DMA_BASE: u64 = (0xb000d000);
pub const BCM_6362_WLAN_CHIPCOMMON_BASE: u64 = (0xb0004000);
pub const BCM_6362_WLAN_D11_BASE: u64 = (0xb0005000);
pub const BCM_6362_WLAN_SHIM_BASE: u64 = (0xb0007000);

/*
 * 6368 register sets base address
 */
pub const BCM_6368_DSL_LMEM_BASE: u64 = (0xdeadbeef);
pub const BCM_6368_PERF_BASE: u64 = (0xb0000000);
pub const BCM_6368_TIMER_BASE: u64 = (0xb0000040);
pub const BCM_6368_WDT_BASE: u64 = (0xb000005c);
pub const BCM_6368_UART0_BASE: u64 = (0xb0000100);
pub const BCM_6368_UART1_BASE: u64 = (0xb0000120);
pub const BCM_6368_GPIO_BASE: u64 = (0xb0000080);
pub const BCM_6368_SPI_BASE: u64 = (0xb0000800);
pub const BCM_6368_HSSPI_BASE: u64 = (0xdeadbeef);
pub const BCM_6368_UDC0_BASE: u64 = (0xdeadbeef);
pub const BCM_6368_USBDMA_BASE: u64 = (0xb0004800);
pub const BCM_6368_OHCI0_BASE: u64 = (0xb0001600);
pub const BCM_6368_OHCI_PRIV_BASE: u64 = (0xdeadbeef);
pub const BCM_6368_USBH_PRIV_BASE: u64 = (0xb0001700);
pub const BCM_6368_USBD_BASE: u64 = (0xb0001400);
pub const BCM_6368_MPI_BASE: u64 = (0xb0001000);
pub const BCM_6368_PCMCIA_BASE: u64 = (0xb0001054);
pub const BCM_6368_PCIE_BASE: u64 = (0xdeadbeef);
pub const BCM_6368_SDRAM_REGS_BASE: u64 = (0xdeadbeef);
pub const BCM_6368_M2M_BASE: u64 = (0xdeadbeef);
pub const BCM_6368_DSL_BASE: u64 = (0xdeadbeef);
pub const BCM_6368_ENET0_BASE: u64 = (0xdeadbeef);
pub const BCM_6368_ENET1_BASE: u64 = (0xdeadbeef);
pub const BCM_6368_ENETDMA_BASE: u64 = (0xb0006800);
pub const BCM_6368_ENETDMAC_BASE: u64 = (0xb0006a00);
pub const BCM_6368_ENETDMAS_BASE: u64 = (0xb0006c00);
pub const BCM_6368_ENETSW_BASE: u64 = (0xb0f00000);
pub const BCM_6368_EHCI0_BASE: u64 = (0xb0001500);
pub const BCM_6368_SDRAM_BASE: u64 = (0xdeadbeef);
pub const BCM_6368_MEMC_BASE: u64 = (0xb0001200);
pub const BCM_6368_DDR_BASE: u64 = (0xb0001280);
pub const BCM_6368_ATM_BASE: u64 = (0xdeadbeef);
pub const BCM_6368_XTM_BASE: u64 = (0xb0001800);
pub const BCM_6368_XTMDMA_BASE: u64 = (0xb0005000);
pub const BCM_6368_XTMDMAC_BASE: u64 = (0xb0005200);
pub const BCM_6368_XTMDMAS_BASE: u64 = (0xb0005400);
pub const BCM_6368_PCM_BASE: u64 = (0xb0004000);
pub const BCM_6368_PCMDMA_BASE: u64 = (0xb0005800);
pub const BCM_6368_PCMDMAC_BASE: u64 = (0xb0005a00);
pub const BCM_6368_PCMDMAS_BASE: u64 = (0xb0005c00);
pub const BCM_6368_RNG_BASE: u64 = (0xb0004180);
pub const BCM_6368_MISC_BASE: u64 = (0xdeadbeef);


extern "C" { pub static bcm63xx_regs_base: *const u64; }

pub const fn /* __GEN_CPU_REGS_TABLE token-pasting initializer preserved from C */ -> usize { \ }
	[RSET_DSL_LMEM]		= BCM_## __cpu ##_DSL_LMEM_BASE,	\
	[RSET_PERF]		= BCM_## __cpu ##_PERF_BASE,		\
	[RSET_TIMER]		= BCM_## __cpu ##_TIMER_BASE,		\
	[RSET_WDT]		= BCM_## __cpu ##_WDT_BASE,		\
	[RSET_UART0]		= BCM_## __cpu ##_UART0_BASE,		\
	[RSET_UART1]		= BCM_## __cpu ##_UART1_BASE,		\
	[RSET_GPIO]		= BCM_## __cpu ##_GPIO_BASE,		\
	[RSET_SPI]		= BCM_## __cpu ##_SPI_BASE,		\
	[RSET_HSSPI]		= BCM_## __cpu ##_HSSPI_BASE,		\
	[RSET_UDC0]		= BCM_## __cpu ##_UDC0_BASE,		\
	[RSET_OHCI0]		= BCM_## __cpu ##_OHCI0_BASE,		\
	[RSET_OHCI_PRIV]	= BCM_## __cpu ##_OHCI_PRIV_BASE,	\
	[RSET_USBH_PRIV]	= BCM_## __cpu ##_USBH_PRIV_BASE,	\
	[RSET_USBD]		= BCM_## __cpu ##_USBD_BASE,		\
	[RSET_USBDMA]		= BCM_## __cpu ##_USBDMA_BASE,		\
	[RSET_MPI]		= BCM_## __cpu ##_MPI_BASE,		\
	[RSET_PCMCIA]		= BCM_## __cpu ##_PCMCIA_BASE,		\
	[RSET_PCIE]		= BCM_## __cpu ##_PCIE_BASE,		\
	[RSET_DSL]		= BCM_## __cpu ##_DSL_BASE,		\
	[RSET_ENET0]		= BCM_## __cpu ##_ENET0_BASE,		\
	[RSET_ENET1]		= BCM_## __cpu ##_ENET1_BASE,		\
	[RSET_ENETDMA]		= BCM_## __cpu ##_ENETDMA_BASE,		\
	[RSET_ENETDMAC]		= BCM_## __cpu ##_ENETDMAC_BASE,	\
	[RSET_ENETDMAS]		= BCM_## __cpu ##_ENETDMAS_BASE,	\
	[RSET_ENETSW]		= BCM_## __cpu ##_ENETSW_BASE,		\
	[RSET_EHCI0]		= BCM_## __cpu ##_EHCI0_BASE,		\
	[RSET_SDRAM]		= BCM_## __cpu ##_SDRAM_BASE,		\
	[RSET_MEMC]		= BCM_## __cpu ##_MEMC_BASE,		\
	[RSET_DDR]		= BCM_## __cpu ##_DDR_BASE,		\
	[RSET_M2M]		= BCM_## __cpu ##_M2M_BASE,		\
	[RSET_ATM]		= BCM_## __cpu ##_ATM_BASE,		\
	[RSET_XTM]		= BCM_## __cpu ##_XTM_BASE,		\
	[RSET_XTMDMA]		= BCM_## __cpu ##_XTMDMA_BASE,		\
	[RSET_XTMDMAC]		= BCM_## __cpu ##_XTMDMAC_BASE,		\
	[RSET_XTMDMAS]		= BCM_## __cpu ##_XTMDMAS_BASE,		\
	[RSET_PCM]		= BCM_## __cpu ##_PCM_BASE,		\
	[RSET_PCMDMA]		= BCM_## __cpu ##_PCMDMA_BASE,		\
	[RSET_PCMDMAC]		= BCM_## __cpu ##_PCMDMAC_BASE,		\
	[RSET_PCMDMAS]		= BCM_## __cpu ##_PCMDMAS_BASE,		\
	[RSET_RNG]		= BCM_## __cpu ##_RNG_BASE,		\
	[RSET_MISC]		= BCM_## __cpu ##_MISC_BASE,		\


pub unsafe fn bcm63xx_regset_address(set: bcm63xx_regs_set) -> u64
{
	return *bcm63xx_regs_base.add(set as usize);
}

/*
 * IRQ number changes across CPU too
 */
#[repr(C)]\npub bcm63xx_irq {
	IRQ_TIMER = 0,
	IRQ_SPI,
	IRQ_UART0,
	IRQ_UART1,
	IRQ_DSL,
	IRQ_ENET0,
	IRQ_ENET1,
	IRQ_ENET_PHY,
	IRQ_HSSPI,
	IRQ_OHCI0,
	IRQ_EHCI0,
	IRQ_USBD,
	IRQ_USBD_RXDMA0,
	IRQ_USBD_TXDMA0,
	IRQ_USBD_RXDMA1,
	IRQ_USBD_TXDMA1,
	IRQ_USBD_RXDMA2,
	IRQ_USBD_TXDMA2,
	IRQ_ENET0_RXDMA,
	IRQ_ENET0_TXDMA,
	IRQ_ENET1_RXDMA,
	IRQ_ENET1_TXDMA,
	IRQ_PCI,
	IRQ_PCMCIA,
	IRQ_ATM,
	IRQ_ENETSW_RXDMA0,
	IRQ_ENETSW_RXDMA1,
	IRQ_ENETSW_RXDMA2,
	IRQ_ENETSW_RXDMA3,
	IRQ_ENETSW_TXDMA0,
	IRQ_ENETSW_TXDMA1,
	IRQ_ENETSW_TXDMA2,
	IRQ_ENETSW_TXDMA3,
	IRQ_XTM,
	IRQ_XTM_DMA0,
};

/*
 * 3368 irqs
 */
pub const BCM_3368_TIMER_IRQ: u64 = (IRQ_INTERNAL_BASE + 0);
pub const BCM_3368_SPI_IRQ: u64 = (IRQ_INTERNAL_BASE + 1);
pub const BCM_3368_UART0_IRQ: u64 = (IRQ_INTERNAL_BASE + 2);
pub const BCM_3368_UART1_IRQ: u64 = (IRQ_INTERNAL_BASE + 3);
pub const BCM_3368_DSL_IRQ: u64 = 0;
pub const BCM_3368_UDC0_IRQ: u64 = 0;
pub const BCM_3368_OHCI0_IRQ: u64 = 0;
pub const BCM_3368_ENET0_IRQ: u64 = (IRQ_INTERNAL_BASE + 8);
pub const BCM_3368_ENET1_IRQ: u64 = (IRQ_INTERNAL_BASE + 6);
pub const BCM_3368_ENET_PHY_IRQ: u64 = (IRQ_INTERNAL_BASE + 9);
pub const BCM_3368_ENET0_RXDMA_IRQ: u64 = (IRQ_INTERNAL_BASE + 15);
pub const BCM_3368_ENET0_TXDMA_IRQ: u64 = (IRQ_INTERNAL_BASE + 16);
pub const BCM_3368_HSSPI_IRQ: u64 = 0;
pub const BCM_3368_EHCI0_IRQ: u64 = 0;
pub const BCM_3368_USBD_IRQ: u64 = 0;
pub const BCM_3368_USBD_RXDMA0_IRQ: u64 = 0;
pub const BCM_3368_USBD_TXDMA0_IRQ: u64 = 0;
pub const BCM_3368_USBD_RXDMA1_IRQ: u64 = 0;
pub const BCM_3368_USBD_TXDMA1_IRQ: u64 = 0;
pub const BCM_3368_USBD_RXDMA2_IRQ: u64 = 0;
pub const BCM_3368_USBD_TXDMA2_IRQ: u64 = 0;
pub const BCM_3368_ENET1_RXDMA_IRQ: u64 = (IRQ_INTERNAL_BASE + 17);
pub const BCM_3368_ENET1_TXDMA_IRQ: u64 = (IRQ_INTERNAL_BASE + 18);
pub const BCM_3368_PCI_IRQ: u64 = (IRQ_INTERNAL_BASE + 31);
pub const BCM_3368_PCMCIA_IRQ: u64 = 0;
pub const BCM_3368_ATM_IRQ: u64 = 0;
pub const BCM_3368_ENETSW_RXDMA0_IRQ: u64 = 0;
pub const BCM_3368_ENETSW_RXDMA1_IRQ: u64 = 0;
pub const BCM_3368_ENETSW_RXDMA2_IRQ: u64 = 0;
pub const BCM_3368_ENETSW_RXDMA3_IRQ: u64 = 0;
pub const BCM_3368_ENETSW_TXDMA0_IRQ: u64 = 0;
pub const BCM_3368_ENETSW_TXDMA1_IRQ: u64 = 0;
pub const BCM_3368_ENETSW_TXDMA2_IRQ: u64 = 0;
pub const BCM_3368_ENETSW_TXDMA3_IRQ: u64 = 0;
pub const BCM_3368_XTM_IRQ: u64 = 0;
pub const BCM_3368_XTM_DMA0_IRQ: u64 = 0;

pub const BCM_3368_EXT_IRQ0: u64 = (IRQ_INTERNAL_BASE + 25);
pub const BCM_3368_EXT_IRQ1: u64 = (IRQ_INTERNAL_BASE + 26);
pub const BCM_3368_EXT_IRQ2: u64 = (IRQ_INTERNAL_BASE + 27);
pub const BCM_3368_EXT_IRQ3: u64 = (IRQ_INTERNAL_BASE + 28);


/*
 * 6328 irqs
 */
pub const BCM_6328_HIGH_IRQ_BASE: u64 = (IRQ_INTERNAL_BASE + 32);

pub const BCM_6328_TIMER_IRQ: u64 = (IRQ_INTERNAL_BASE + 31);
pub const BCM_6328_SPI_IRQ: u64 = 0;
pub const BCM_6328_UART0_IRQ: u64 = (IRQ_INTERNAL_BASE + 28);
pub const BCM_6328_UART1_IRQ: u64 = (BCM_6328_HIGH_IRQ_BASE + 7);
pub const BCM_6328_DSL_IRQ: u64 = (IRQ_INTERNAL_BASE + 4);
pub const BCM_6328_UDC0_IRQ: u64 = 0;
pub const BCM_6328_ENET0_IRQ: u64 = 0;
pub const BCM_6328_ENET1_IRQ: u64 = 0;
pub const BCM_6328_ENET_PHY_IRQ: u64 = (IRQ_INTERNAL_BASE + 12);
pub const BCM_6328_HSSPI_IRQ: u64 = (IRQ_INTERNAL_BASE + 29);
pub const BCM_6328_OHCI0_IRQ: u64 = (BCM_6328_HIGH_IRQ_BASE + 9);
pub const BCM_6328_EHCI0_IRQ: u64 = (BCM_6328_HIGH_IRQ_BASE + 10);
pub const BCM_6328_USBD_IRQ: u64 = (IRQ_INTERNAL_BASE + 4);
pub const BCM_6328_USBD_RXDMA0_IRQ: u64 = (IRQ_INTERNAL_BASE + 5);
pub const BCM_6328_USBD_TXDMA0_IRQ: u64 = (IRQ_INTERNAL_BASE + 6);
pub const BCM_6328_USBD_RXDMA1_IRQ: u64 = (IRQ_INTERNAL_BASE + 7);
pub const BCM_6328_USBD_TXDMA1_IRQ: u64 = (IRQ_INTERNAL_BASE + 8);
pub const BCM_6328_USBD_RXDMA2_IRQ: u64 = (IRQ_INTERNAL_BASE + 9);
pub const BCM_6328_USBD_TXDMA2_IRQ: u64 = (IRQ_INTERNAL_BASE + 10);
pub const BCM_6328_PCMCIA_IRQ: u64 = 0;
pub const BCM_6328_ENET0_RXDMA_IRQ: u64 = 0;
pub const BCM_6328_ENET0_TXDMA_IRQ: u64 = 0;
pub const BCM_6328_ENET1_RXDMA_IRQ: u64 = 0;
pub const BCM_6328_ENET1_TXDMA_IRQ: u64 = 0;
pub const BCM_6328_PCI_IRQ: u64 = (IRQ_INTERNAL_BASE + 23);
pub const BCM_6328_ATM_IRQ: u64 = 0;
pub const BCM_6328_ENETSW_RXDMA0_IRQ: u64 = (BCM_6328_HIGH_IRQ_BASE + 0);
pub const BCM_6328_ENETSW_RXDMA1_IRQ: u64 = (BCM_6328_HIGH_IRQ_BASE + 1);
pub const BCM_6328_ENETSW_RXDMA2_IRQ: u64 = (BCM_6328_HIGH_IRQ_BASE + 2);
pub const BCM_6328_ENETSW_RXDMA3_IRQ: u64 = (BCM_6328_HIGH_IRQ_BASE + 3);
pub const BCM_6328_ENETSW_TXDMA0_IRQ: u64 = 0;
pub const BCM_6328_ENETSW_TXDMA1_IRQ: u64 = 0;
pub const BCM_6328_ENETSW_TXDMA2_IRQ: u64 = 0;
pub const BCM_6328_ENETSW_TXDMA3_IRQ: u64 = 0;
pub const BCM_6328_XTM_IRQ: u64 = (BCM_6328_HIGH_IRQ_BASE + 31);
pub const BCM_6328_XTM_DMA0_IRQ: u64 = (BCM_6328_HIGH_IRQ_BASE + 11);

pub const BCM_6328_PCM_DMA0_IRQ: u64 = (IRQ_INTERNAL_BASE + 2);
pub const BCM_6328_PCM_DMA1_IRQ: u64 = (IRQ_INTERNAL_BASE + 3);
pub const BCM_6328_EXT_IRQ0: u64 = (IRQ_INTERNAL_BASE + 24);
pub const BCM_6328_EXT_IRQ1: u64 = (IRQ_INTERNAL_BASE + 25);
pub const BCM_6328_EXT_IRQ2: u64 = (IRQ_INTERNAL_BASE + 26);
pub const BCM_6328_EXT_IRQ3: u64 = (IRQ_INTERNAL_BASE + 27);

/*
 * 6338 irqs
 */
pub const BCM_6338_TIMER_IRQ: u64 = (IRQ_INTERNAL_BASE + 0);
pub const BCM_6338_SPI_IRQ: u64 = (IRQ_INTERNAL_BASE + 1);
pub const BCM_6338_UART0_IRQ: u64 = (IRQ_INTERNAL_BASE + 2);
pub const BCM_6338_UART1_IRQ: u64 = 0;
pub const BCM_6338_DSL_IRQ: u64 = (IRQ_INTERNAL_BASE + 5);
pub const BCM_6338_ENET0_IRQ: u64 = (IRQ_INTERNAL_BASE + 8);
pub const BCM_6338_ENET1_IRQ: u64 = 0;
pub const BCM_6338_ENET_PHY_IRQ: u64 = (IRQ_INTERNAL_BASE + 9);
pub const BCM_6338_HSSPI_IRQ: u64 = 0;
pub const BCM_6338_OHCI0_IRQ: u64 = 0;
pub const BCM_6338_EHCI0_IRQ: u64 = 0;
pub const BCM_6338_USBD_IRQ: u64 = 0;
pub const BCM_6338_USBD_RXDMA0_IRQ: u64 = 0;
pub const BCM_6338_USBD_TXDMA0_IRQ: u64 = 0;
pub const BCM_6338_USBD_RXDMA1_IRQ: u64 = 0;
pub const BCM_6338_USBD_TXDMA1_IRQ: u64 = 0;
pub const BCM_6338_USBD_RXDMA2_IRQ: u64 = 0;
pub const BCM_6338_USBD_TXDMA2_IRQ: u64 = 0;
pub const BCM_6338_ENET0_RXDMA_IRQ: u64 = (IRQ_INTERNAL_BASE + 15);
pub const BCM_6338_ENET0_TXDMA_IRQ: u64 = (IRQ_INTERNAL_BASE + 16);
pub const BCM_6338_ENET1_RXDMA_IRQ: u64 = 0;
pub const BCM_6338_ENET1_TXDMA_IRQ: u64 = 0;
pub const BCM_6338_PCI_IRQ: u64 = 0;
pub const BCM_6338_PCMCIA_IRQ: u64 = 0;
pub const BCM_6338_ATM_IRQ: u64 = 0;
pub const BCM_6338_ENETSW_RXDMA0_IRQ: u64 = 0;
pub const BCM_6338_ENETSW_RXDMA1_IRQ: u64 = 0;
pub const BCM_6338_ENETSW_RXDMA2_IRQ: u64 = 0;
pub const BCM_6338_ENETSW_RXDMA3_IRQ: u64 = 0;
pub const BCM_6338_ENETSW_TXDMA0_IRQ: u64 = 0;
pub const BCM_6338_ENETSW_TXDMA1_IRQ: u64 = 0;
pub const BCM_6338_ENETSW_TXDMA2_IRQ: u64 = 0;
pub const BCM_6338_ENETSW_TXDMA3_IRQ: u64 = 0;
pub const BCM_6338_XTM_IRQ: u64 = 0;
pub const BCM_6338_XTM_DMA0_IRQ: u64 = 0;

/*
 * 6345 irqs
 */
pub const BCM_6345_TIMER_IRQ: u64 = (IRQ_INTERNAL_BASE + 0);
pub const BCM_6345_SPI_IRQ: u64 = 0;
pub const BCM_6345_UART0_IRQ: u64 = (IRQ_INTERNAL_BASE + 2);
pub const BCM_6345_UART1_IRQ: u64 = 0;
pub const BCM_6345_DSL_IRQ: u64 = (IRQ_INTERNAL_BASE + 3);
pub const BCM_6345_ENET0_IRQ: u64 = (IRQ_INTERNAL_BASE + 8);
pub const BCM_6345_ENET1_IRQ: u64 = 0;
pub const BCM_6345_ENET_PHY_IRQ: u64 = (IRQ_INTERNAL_BASE + 12);
pub const BCM_6345_HSSPI_IRQ: u64 = 0;
pub const BCM_6345_OHCI0_IRQ: u64 = 0;
pub const BCM_6345_EHCI0_IRQ: u64 = 0;
pub const BCM_6345_USBD_IRQ: u64 = 0;
pub const BCM_6345_USBD_RXDMA0_IRQ: u64 = 0;
pub const BCM_6345_USBD_TXDMA0_IRQ: u64 = 0;
pub const BCM_6345_USBD_RXDMA1_IRQ: u64 = 0;
pub const BCM_6345_USBD_TXDMA1_IRQ: u64 = 0;
pub const BCM_6345_USBD_RXDMA2_IRQ: u64 = 0;
pub const BCM_6345_USBD_TXDMA2_IRQ: u64 = 0;
pub const BCM_6345_ENET0_RXDMA_IRQ: u64 = (IRQ_INTERNAL_BASE + 13 + 1);
pub const BCM_6345_ENET0_TXDMA_IRQ: u64 = (IRQ_INTERNAL_BASE + 13 + 2);
pub const BCM_6345_ENET1_RXDMA_IRQ: u64 = 0;
pub const BCM_6345_ENET1_TXDMA_IRQ: u64 = 0;
pub const BCM_6345_PCI_IRQ: u64 = 0;
pub const BCM_6345_PCMCIA_IRQ: u64 = 0;
pub const BCM_6345_ATM_IRQ: u64 = 0;
pub const BCM_6345_ENETSW_RXDMA0_IRQ: u64 = 0;
pub const BCM_6345_ENETSW_RXDMA1_IRQ: u64 = 0;
pub const BCM_6345_ENETSW_RXDMA2_IRQ: u64 = 0;
pub const BCM_6345_ENETSW_RXDMA3_IRQ: u64 = 0;
pub const BCM_6345_ENETSW_TXDMA0_IRQ: u64 = 0;
pub const BCM_6345_ENETSW_TXDMA1_IRQ: u64 = 0;
pub const BCM_6345_ENETSW_TXDMA2_IRQ: u64 = 0;
pub const BCM_6345_ENETSW_TXDMA3_IRQ: u64 = 0;
pub const BCM_6345_XTM_IRQ: u64 = 0;
pub const BCM_6345_XTM_DMA0_IRQ: u64 = 0;

/*
 * 6348 irqs
 */
pub const BCM_6348_TIMER_IRQ: u64 = (IRQ_INTERNAL_BASE + 0);
pub const BCM_6348_SPI_IRQ: u64 = (IRQ_INTERNAL_BASE + 1);
pub const BCM_6348_UART0_IRQ: u64 = (IRQ_INTERNAL_BASE + 2);
pub const BCM_6348_UART1_IRQ: u64 = 0;
pub const BCM_6348_DSL_IRQ: u64 = (IRQ_INTERNAL_BASE + 4);
pub const BCM_6348_ENET0_IRQ: u64 = (IRQ_INTERNAL_BASE + 8);
pub const BCM_6348_ENET1_IRQ: u64 = (IRQ_INTERNAL_BASE + 7);
pub const BCM_6348_ENET_PHY_IRQ: u64 = (IRQ_INTERNAL_BASE + 9);
pub const BCM_6348_HSSPI_IRQ: u64 = 0;
pub const BCM_6348_OHCI0_IRQ: u64 = (IRQ_INTERNAL_BASE + 12);
pub const BCM_6348_EHCI0_IRQ: u64 = 0;
pub const BCM_6348_USBD_IRQ: u64 = 0;
pub const BCM_6348_USBD_RXDMA0_IRQ: u64 = 0;
pub const BCM_6348_USBD_TXDMA0_IRQ: u64 = 0;
pub const BCM_6348_USBD_RXDMA1_IRQ: u64 = 0;
pub const BCM_6348_USBD_TXDMA1_IRQ: u64 = 0;
pub const BCM_6348_USBD_RXDMA2_IRQ: u64 = 0;
pub const BCM_6348_USBD_TXDMA2_IRQ: u64 = 0;
pub const BCM_6348_ENET0_RXDMA_IRQ: u64 = (IRQ_INTERNAL_BASE + 20);
pub const BCM_6348_ENET0_TXDMA_IRQ: u64 = (IRQ_INTERNAL_BASE + 21);
pub const BCM_6348_ENET1_RXDMA_IRQ: u64 = (IRQ_INTERNAL_BASE + 22);
pub const BCM_6348_ENET1_TXDMA_IRQ: u64 = (IRQ_INTERNAL_BASE + 23);
pub const BCM_6348_PCI_IRQ: u64 = (IRQ_INTERNAL_BASE + 24);
pub const BCM_6348_PCMCIA_IRQ: u64 = (IRQ_INTERNAL_BASE + 24);
pub const BCM_6348_ATM_IRQ: u64 = (IRQ_INTERNAL_BASE + 5);
pub const BCM_6348_ENETSW_RXDMA0_IRQ: u64 = 0;
pub const BCM_6348_ENETSW_RXDMA1_IRQ: u64 = 0;
pub const BCM_6348_ENETSW_RXDMA2_IRQ: u64 = 0;
pub const BCM_6348_ENETSW_RXDMA3_IRQ: u64 = 0;
pub const BCM_6348_ENETSW_TXDMA0_IRQ: u64 = 0;
pub const BCM_6348_ENETSW_TXDMA1_IRQ: u64 = 0;
pub const BCM_6348_ENETSW_TXDMA2_IRQ: u64 = 0;
pub const BCM_6348_ENETSW_TXDMA3_IRQ: u64 = 0;
pub const BCM_6348_XTM_IRQ: u64 = 0;
pub const BCM_6348_XTM_DMA0_IRQ: u64 = 0;

/*
 * 6358 irqs
 */
pub const BCM_6358_TIMER_IRQ: u64 = (IRQ_INTERNAL_BASE + 0);
pub const BCM_6358_SPI_IRQ: u64 = (IRQ_INTERNAL_BASE + 1);
pub const BCM_6358_UART0_IRQ: u64 = (IRQ_INTERNAL_BASE + 2);
pub const BCM_6358_UART1_IRQ: u64 = (IRQ_INTERNAL_BASE + 3);
pub const BCM_6358_DSL_IRQ: u64 = (IRQ_INTERNAL_BASE + 29);
pub const BCM_6358_ENET0_IRQ: u64 = (IRQ_INTERNAL_BASE + 8);
pub const BCM_6358_ENET1_IRQ: u64 = (IRQ_INTERNAL_BASE + 6);
pub const BCM_6358_ENET_PHY_IRQ: u64 = (IRQ_INTERNAL_BASE + 9);
pub const BCM_6358_HSSPI_IRQ: u64 = 0;
pub const BCM_6358_OHCI0_IRQ: u64 = (IRQ_INTERNAL_BASE + 5);
pub const BCM_6358_EHCI0_IRQ: u64 = (IRQ_INTERNAL_BASE + 10);
pub const BCM_6358_USBD_IRQ: u64 = 0;
pub const BCM_6358_USBD_RXDMA0_IRQ: u64 = 0;
pub const BCM_6358_USBD_TXDMA0_IRQ: u64 = 0;
pub const BCM_6358_USBD_RXDMA1_IRQ: u64 = 0;
pub const BCM_6358_USBD_TXDMA1_IRQ: u64 = 0;
pub const BCM_6358_USBD_RXDMA2_IRQ: u64 = 0;
pub const BCM_6358_USBD_TXDMA2_IRQ: u64 = 0;
pub const BCM_6358_ENET0_RXDMA_IRQ: u64 = (IRQ_INTERNAL_BASE + 15);
pub const BCM_6358_ENET0_TXDMA_IRQ: u64 = (IRQ_INTERNAL_BASE + 16);
pub const BCM_6358_ENET1_RXDMA_IRQ: u64 = (IRQ_INTERNAL_BASE + 17);
pub const BCM_6358_ENET1_TXDMA_IRQ: u64 = (IRQ_INTERNAL_BASE + 18);
pub const BCM_6358_PCI_IRQ: u64 = (IRQ_INTERNAL_BASE + 31);
pub const BCM_6358_PCMCIA_IRQ: u64 = (IRQ_INTERNAL_BASE + 24);
pub const BCM_6358_ATM_IRQ: u64 = (IRQ_INTERNAL_BASE + 19);
pub const BCM_6358_ENETSW_RXDMA0_IRQ: u64 = 0;
pub const BCM_6358_ENETSW_RXDMA1_IRQ: u64 = 0;
pub const BCM_6358_ENETSW_RXDMA2_IRQ: u64 = 0;
pub const BCM_6358_ENETSW_RXDMA3_IRQ: u64 = 0;
pub const BCM_6358_ENETSW_TXDMA0_IRQ: u64 = 0;
pub const BCM_6358_ENETSW_TXDMA1_IRQ: u64 = 0;
pub const BCM_6358_ENETSW_TXDMA2_IRQ: u64 = 0;
pub const BCM_6358_ENETSW_TXDMA3_IRQ: u64 = 0;
pub const BCM_6358_XTM_IRQ: u64 = 0;
pub const BCM_6358_XTM_DMA0_IRQ: u64 = 0;

pub const BCM_6358_PCM_DMA0_IRQ: u64 = (IRQ_INTERNAL_BASE + 23);
pub const BCM_6358_PCM_DMA1_IRQ: u64 = (IRQ_INTERNAL_BASE + 24);
pub const BCM_6358_EXT_IRQ0: u64 = (IRQ_INTERNAL_BASE + 25);
pub const BCM_6358_EXT_IRQ1: u64 = (IRQ_INTERNAL_BASE + 26);
pub const BCM_6358_EXT_IRQ2: u64 = (IRQ_INTERNAL_BASE + 27);
pub const BCM_6358_EXT_IRQ3: u64 = (IRQ_INTERNAL_BASE + 28);

/*
 * 6362 irqs
 */
pub const BCM_6362_HIGH_IRQ_BASE: u64 = (IRQ_INTERNAL_BASE + 32);

pub const BCM_6362_TIMER_IRQ: u64 = (IRQ_INTERNAL_BASE + 0);
pub const BCM_6362_SPI_IRQ: u64 = (IRQ_INTERNAL_BASE + 2);
pub const BCM_6362_UART0_IRQ: u64 = (IRQ_INTERNAL_BASE + 3);
pub const BCM_6362_UART1_IRQ: u64 = (IRQ_INTERNAL_BASE + 4);
pub const BCM_6362_DSL_IRQ: u64 = (IRQ_INTERNAL_BASE + 28);
pub const BCM_6362_UDC0_IRQ: u64 = 0;
pub const BCM_6362_ENET0_IRQ: u64 = 0;
pub const BCM_6362_ENET1_IRQ: u64 = 0;
pub const BCM_6362_ENET_PHY_IRQ: u64 = (IRQ_INTERNAL_BASE + 14);
pub const BCM_6362_HSSPI_IRQ: u64 = (IRQ_INTERNAL_BASE + 5);
pub const BCM_6362_OHCI0_IRQ: u64 = (IRQ_INTERNAL_BASE + 9);
pub const BCM_6362_EHCI0_IRQ: u64 = (IRQ_INTERNAL_BASE + 10);
pub const BCM_6362_USBD_IRQ: u64 = (IRQ_INTERNAL_BASE + 11);
pub const BCM_6362_USBD_RXDMA0_IRQ: u64 = (IRQ_INTERNAL_BASE + 20);
pub const BCM_6362_USBD_TXDMA0_IRQ: u64 = (IRQ_INTERNAL_BASE + 21);
pub const BCM_6362_USBD_RXDMA1_IRQ: u64 = (IRQ_INTERNAL_BASE + 22);
pub const BCM_6362_USBD_TXDMA1_IRQ: u64 = (IRQ_INTERNAL_BASE + 23);
pub const BCM_6362_USBD_RXDMA2_IRQ: u64 = (IRQ_INTERNAL_BASE + 24);
pub const BCM_6362_USBD_TXDMA2_IRQ: u64 = (IRQ_INTERNAL_BASE + 25);
pub const BCM_6362_PCMCIA_IRQ: u64 = 0;
pub const BCM_6362_ENET0_RXDMA_IRQ: u64 = 0;
pub const BCM_6362_ENET0_TXDMA_IRQ: u64 = 0;
pub const BCM_6362_ENET1_RXDMA_IRQ: u64 = 0;
pub const BCM_6362_ENET1_TXDMA_IRQ: u64 = 0;
pub const BCM_6362_PCI_IRQ: u64 = (IRQ_INTERNAL_BASE + 30);
pub const BCM_6362_ATM_IRQ: u64 = 0;
pub const BCM_6362_ENETSW_RXDMA0_IRQ: u64 = (BCM_6362_HIGH_IRQ_BASE + 0);
pub const BCM_6362_ENETSW_RXDMA1_IRQ: u64 = (BCM_6362_HIGH_IRQ_BASE + 1);
pub const BCM_6362_ENETSW_RXDMA2_IRQ: u64 = (BCM_6362_HIGH_IRQ_BASE + 2);
pub const BCM_6362_ENETSW_RXDMA3_IRQ: u64 = (BCM_6362_HIGH_IRQ_BASE + 3);
pub const BCM_6362_ENETSW_TXDMA0_IRQ: u64 = 0;
pub const BCM_6362_ENETSW_TXDMA1_IRQ: u64 = 0;
pub const BCM_6362_ENETSW_TXDMA2_IRQ: u64 = 0;
pub const BCM_6362_ENETSW_TXDMA3_IRQ: u64 = 0;
pub const BCM_6362_XTM_IRQ: u64 = 0;
pub const BCM_6362_XTM_DMA0_IRQ: u64 = (BCM_6362_HIGH_IRQ_BASE + 12);

pub const BCM_6362_RING_OSC_IRQ: u64 = (IRQ_INTERNAL_BASE + 1);
pub const BCM_6362_WLAN_GPIO_IRQ: u64 = (IRQ_INTERNAL_BASE + 6);
pub const BCM_6362_WLAN_IRQ: u64 = (IRQ_INTERNAL_BASE + 7);
pub const BCM_6362_IPSEC_IRQ: u64 = (IRQ_INTERNAL_BASE + 8);
pub const BCM_6362_NAND_IRQ: u64 = (IRQ_INTERNAL_BASE + 12);
pub const BCM_6362_PCM_IRQ: u64 = (IRQ_INTERNAL_BASE + 13);
pub const BCM_6362_DG_IRQ: u64 = (IRQ_INTERNAL_BASE + 15);
pub const BCM_6362_EPHY_ENERGY0_IRQ: u64 = (IRQ_INTERNAL_BASE + 16);
pub const BCM_6362_EPHY_ENERGY1_IRQ: u64 = (IRQ_INTERNAL_BASE + 17);
pub const BCM_6362_EPHY_ENERGY2_IRQ: u64 = (IRQ_INTERNAL_BASE + 18);
pub const BCM_6362_EPHY_ENERGY3_IRQ: u64 = (IRQ_INTERNAL_BASE + 19);
pub const BCM_6362_IPSEC_DMA0_IRQ: u64 = (IRQ_INTERNAL_BASE + 26);
pub const BCM_6362_IPSEC_DMA1_IRQ: u64 = (IRQ_INTERNAL_BASE + 27);
pub const BCM_6362_FAP0_IRQ: u64 = (IRQ_INTERNAL_BASE + 29);
pub const BCM_6362_PCM_DMA0_IRQ: u64 = (BCM_6362_HIGH_IRQ_BASE + 4);
pub const BCM_6362_PCM_DMA1_IRQ: u64 = (BCM_6362_HIGH_IRQ_BASE + 5);
pub const BCM_6362_DECT0_IRQ: u64 = (BCM_6362_HIGH_IRQ_BASE + 6);
pub const BCM_6362_DECT1_IRQ: u64 = (BCM_6362_HIGH_IRQ_BASE + 7);
pub const BCM_6362_EXT_IRQ0: u64 = (BCM_6362_HIGH_IRQ_BASE + 8);
pub const BCM_6362_EXT_IRQ1: u64 = (BCM_6362_HIGH_IRQ_BASE + 9);
pub const BCM_6362_EXT_IRQ2: u64 = (BCM_6362_HIGH_IRQ_BASE + 10);
pub const BCM_6362_EXT_IRQ3: u64 = (BCM_6362_HIGH_IRQ_BASE + 11);

/*
 * 6368 irqs
 */
pub const BCM_6368_HIGH_IRQ_BASE: u64 = (IRQ_INTERNAL_BASE + 32);

pub const BCM_6368_TIMER_IRQ: u64 = (IRQ_INTERNAL_BASE + 0);
pub const BCM_6368_SPI_IRQ: u64 = (IRQ_INTERNAL_BASE + 1);
pub const BCM_6368_UART0_IRQ: u64 = (IRQ_INTERNAL_BASE + 2);
pub const BCM_6368_UART1_IRQ: u64 = (IRQ_INTERNAL_BASE + 3);
pub const BCM_6368_DSL_IRQ: u64 = (IRQ_INTERNAL_BASE + 4);
pub const BCM_6368_ENET0_IRQ: u64 = 0;
pub const BCM_6368_ENET1_IRQ: u64 = 0;
pub const BCM_6368_ENET_PHY_IRQ: u64 = (IRQ_INTERNAL_BASE + 15);
pub const BCM_6368_HSSPI_IRQ: u64 = 0;
pub const BCM_6368_OHCI0_IRQ: u64 = (IRQ_INTERNAL_BASE + 5);
pub const BCM_6368_EHCI0_IRQ: u64 = (IRQ_INTERNAL_BASE + 7);
pub const BCM_6368_USBD_IRQ: u64 = (IRQ_INTERNAL_BASE + 8);
pub const BCM_6368_USBD_RXDMA0_IRQ: u64 = (IRQ_INTERNAL_BASE + 26);
pub const BCM_6368_USBD_TXDMA0_IRQ: u64 = (IRQ_INTERNAL_BASE + 27);
pub const BCM_6368_USBD_RXDMA1_IRQ: u64 = (IRQ_INTERNAL_BASE + 28);
pub const BCM_6368_USBD_TXDMA1_IRQ: u64 = (IRQ_INTERNAL_BASE + 29);
pub const BCM_6368_USBD_RXDMA2_IRQ: u64 = (IRQ_INTERNAL_BASE + 30);
pub const BCM_6368_USBD_TXDMA2_IRQ: u64 = (IRQ_INTERNAL_BASE + 31);
pub const BCM_6368_PCMCIA_IRQ: u64 = 0;
pub const BCM_6368_ENET0_RXDMA_IRQ: u64 = 0;
pub const BCM_6368_ENET0_TXDMA_IRQ: u64 = 0;
pub const BCM_6368_ENET1_RXDMA_IRQ: u64 = 0;
pub const BCM_6368_ENET1_TXDMA_IRQ: u64 = 0;
pub const BCM_6368_PCI_IRQ: u64 = (IRQ_INTERNAL_BASE + 13);
pub const BCM_6368_ATM_IRQ: u64 = 0;
pub const BCM_6368_ENETSW_RXDMA0_IRQ: u64 = (BCM_6368_HIGH_IRQ_BASE + 0);
pub const BCM_6368_ENETSW_RXDMA1_IRQ: u64 = (BCM_6368_HIGH_IRQ_BASE + 1);
pub const BCM_6368_ENETSW_RXDMA2_IRQ: u64 = (BCM_6368_HIGH_IRQ_BASE + 2);
pub const BCM_6368_ENETSW_RXDMA3_IRQ: u64 = (BCM_6368_HIGH_IRQ_BASE + 3);
pub const BCM_6368_ENETSW_TXDMA0_IRQ: u64 = (BCM_6368_HIGH_IRQ_BASE + 4);
pub const BCM_6368_ENETSW_TXDMA1_IRQ: u64 = (BCM_6368_HIGH_IRQ_BASE + 5);
pub const BCM_6368_ENETSW_TXDMA2_IRQ: u64 = (BCM_6368_HIGH_IRQ_BASE + 6);
pub const BCM_6368_ENETSW_TXDMA3_IRQ: u64 = (BCM_6368_HIGH_IRQ_BASE + 7);
pub const BCM_6368_XTM_IRQ: u64 = (IRQ_INTERNAL_BASE + 11);
pub const BCM_6368_XTM_DMA0_IRQ: u64 = (BCM_6368_HIGH_IRQ_BASE + 8);

pub const BCM_6368_PCM_DMA0_IRQ: u64 = (BCM_6368_HIGH_IRQ_BASE + 30);
pub const BCM_6368_PCM_DMA1_IRQ: u64 = (BCM_6368_HIGH_IRQ_BASE + 31);
pub const BCM_6368_EXT_IRQ0: u64 = (IRQ_INTERNAL_BASE + 20);
pub const BCM_6368_EXT_IRQ1: u64 = (IRQ_INTERNAL_BASE + 21);
pub const BCM_6368_EXT_IRQ2: u64 = (IRQ_INTERNAL_BASE + 22);
pub const BCM_6368_EXT_IRQ3: u64 = (IRQ_INTERNAL_BASE + 23);
pub const BCM_6368_EXT_IRQ4: u64 = (IRQ_INTERNAL_BASE + 24);
pub const BCM_6368_EXT_IRQ5: u64 = (IRQ_INTERNAL_BASE + 25);

extern "C" { pub static bcm63xx_irqs: *const i32; }

pub const fn /* __GEN_CPU_IRQ_TABLE token-pasting initializer preserved from C */ -> usize { \ }
	[IRQ_TIMER]		= BCM_## __cpu ##_TIMER_IRQ,		\
	[IRQ_SPI]		= BCM_## __cpu ##_SPI_IRQ,		\
	[IRQ_UART0]		= BCM_## __cpu ##_UART0_IRQ,		\
	[IRQ_UART1]		= BCM_## __cpu ##_UART1_IRQ,		\
	[IRQ_DSL]		= BCM_## __cpu ##_DSL_IRQ,		\
	[IRQ_ENET0]		= BCM_## __cpu ##_ENET0_IRQ,		\
	[IRQ_ENET1]		= BCM_## __cpu ##_ENET1_IRQ,		\
	[IRQ_ENET_PHY]		= BCM_## __cpu ##_ENET_PHY_IRQ,		\
	[IRQ_HSSPI]		= BCM_## __cpu ##_HSSPI_IRQ,		\
	[IRQ_OHCI0]		= BCM_## __cpu ##_OHCI0_IRQ,		\
	[IRQ_EHCI0]		= BCM_## __cpu ##_EHCI0_IRQ,		\
	[IRQ_USBD]		= BCM_## __cpu ##_USBD_IRQ,		\
	[IRQ_USBD_RXDMA0]	= BCM_## __cpu ##_USBD_RXDMA0_IRQ,	\
	[IRQ_USBD_TXDMA0]	= BCM_## __cpu ##_USBD_TXDMA0_IRQ,	\
	[IRQ_USBD_RXDMA1]	= BCM_## __cpu ##_USBD_RXDMA1_IRQ,	\
	[IRQ_USBD_TXDMA1]	= BCM_## __cpu ##_USBD_TXDMA1_IRQ,	\
	[IRQ_USBD_RXDMA2]	= BCM_## __cpu ##_USBD_RXDMA2_IRQ,	\
	[IRQ_USBD_TXDMA2]	= BCM_## __cpu ##_USBD_TXDMA2_IRQ,	\
	[IRQ_ENET0_RXDMA]	= BCM_## __cpu ##_ENET0_RXDMA_IRQ,	\
	[IRQ_ENET0_TXDMA]	= BCM_## __cpu ##_ENET0_TXDMA_IRQ,	\
	[IRQ_ENET1_RXDMA]	= BCM_## __cpu ##_ENET1_RXDMA_IRQ,	\
	[IRQ_ENET1_TXDMA]	= BCM_## __cpu ##_ENET1_TXDMA_IRQ,	\
	[IRQ_PCI]		= BCM_## __cpu ##_PCI_IRQ,		\
	[IRQ_PCMCIA]		= BCM_## __cpu ##_PCMCIA_IRQ,		\
	[IRQ_ATM]		= BCM_## __cpu ##_ATM_IRQ,		\
	[IRQ_ENETSW_RXDMA0]	= BCM_## __cpu ##_ENETSW_RXDMA0_IRQ,	\
	[IRQ_ENETSW_RXDMA1]	= BCM_## __cpu ##_ENETSW_RXDMA1_IRQ,	\
	[IRQ_ENETSW_RXDMA2]	= BCM_## __cpu ##_ENETSW_RXDMA2_IRQ,	\
	[IRQ_ENETSW_RXDMA3]	= BCM_## __cpu ##_ENETSW_RXDMA3_IRQ,	\
	[IRQ_ENETSW_TXDMA0]	= BCM_## __cpu ##_ENETSW_TXDMA0_IRQ,	\
	[IRQ_ENETSW_TXDMA1]	= BCM_## __cpu ##_ENETSW_TXDMA1_IRQ,	\
	[IRQ_ENETSW_TXDMA2]	= BCM_## __cpu ##_ENETSW_TXDMA2_IRQ,	\
	[IRQ_ENETSW_TXDMA3]	= BCM_## __cpu ##_ENETSW_TXDMA3_IRQ,	\
	[IRQ_XTM]		= BCM_## __cpu ##_XTM_IRQ,		\
	[IRQ_XTM_DMA0]		= BCM_## __cpu ##_XTM_DMA0_IRQ,		\

pub unsafe fn bcm63xx_get_irq_number(irq: bcm63xx_irq) -> i32
{
	return *bcm63xx_irqs.add(irq as usize);
}

/*
 * return installed memory size
 */
extern "C" { pub fn bcm63xx_get_memory_size() -> u32; }

extern "C" { pub fn bcm63xx_machine_halt(); }

extern "C" { pub fn bcm63xx_machine_reboot(); }

#endif /* !BCM63XX_CPU_H_ */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
