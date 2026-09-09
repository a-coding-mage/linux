/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright (C) 2000 Deep Blue Solutions Ltd.
 *
 * Translated from the Linux UAPI header <linux/serial_core.h>.
 * The constants below complement the values defined in <linux/serial.h>.
 */

/*
 * The type definitions. These are from Ted Ts'o's serial.h. By historical
 * reasons the values from 0 to 13 are defined in linux/serial.h and are not
 * defined here. Values 0 to 19 are used by setserial from busybox and must
 * never be modified.
 */
pub const PORT_NS16550A: i32 = 14;
pub const PORT_XSCALE: i32 = 15;
pub const PORT_RM9000: i32 = 16; // PMC-Sierra RM9xxx internal UART
pub const PORT_OCTEON: i32 = 17; // Cavium OCTEON internal UART
pub const PORT_AR7: i32 = 18; // Texas Instruments AR7 internal UART
pub const PORT_U6_16550A: i32 = 19; // ST-Ericsson U6xxx internal UART
pub const PORT_TEGRA: i32 = 20; // NVIDIA Tegra internal UART
pub const PORT_XR17D15X: i32 = 21; // Exar XR17D15x UART
pub const PORT_LPC3220: i32 = 22; // NXP LPC32xx SoC "Standard" UART
pub const PORT_8250_CIR: i32 = 23; // CIR infrared port, has its own driver
pub const PORT_XR17V35X: i32 = 24; // Exar XR17V35x UARTs
pub const PORT_BRCM_TRUMANAGE: i32 = 25;
pub const PORT_ALTR_16550_F32: i32 = 26; // Altera 16550 UART with 32 FIFOs
pub const PORT_ALTR_16550_F64: i32 = 27; // Altera 16550 UART with 64 FIFOs
pub const PORT_ALTR_16550_F128: i32 = 28; // Altera 16550 UART with 128 FIFOs
pub const PORT_RT2880: i32 = 29; // Ralink RT2880 internal UART
pub const PORT_16550A_FSL64: i32 = 30; // Freescale 16550 UART with 64 FIFOs

/* ARM-specific type numbers; these are not currently guaranteed to be implemented. */
pub const PORT_PXA: i32 = 31;
pub const PORT_AMBA: i32 = 32;
pub const PORT_CLPS711X: i32 = 33;
pub const PORT_SA1100: i32 = 34;
pub const PORT_UART00: i32 = 35;
pub const PORT_OWL: i32 = 36;
pub const PORT_21285: i32 = 37;

/* Sparc type numbers. */
pub const PORT_SUNZILOG: i32 = 38;
pub const PORT_SUNSAB: i32 = 39;
pub const PORT_NPCM: i32 = 40; // Nuvoton UART
pub const PORT_TEGRA_TCU: i32 = 41; // NVIDIA Tegra Combined UART
pub const PORT_ASPEED_VUART: i32 = 42; // ASPEED AST2x00 virtual UART
pub const PORT_PCH_8LINE: i32 = 44; // Intel EG20
pub const PORT_PCH_2LINE: i32 = 45;
pub const PORT_DZ: i32 = 46; // DEC
pub const PORT_ZS: i32 = 47;
pub const PORT_MUX: i32 = 48; // Parisc type numbers
pub const PORT_ATMEL: i32 = 49; // Atmel AT91 SoC
pub const PORT_MAC_ZILOG: i32 = 50; // m68k: not yet implemented
pub const PORT_PMAC_ZILOG: i32 = 51;
pub const PORT_SCI: i32 = 52; // SH-SCI
pub const PORT_SCIF: i32 = 53;
pub const PORT_IRDA: i32 = 54;
pub const PORT_IP22ZILOG: i32 = 56; // SGI IP22 aka Indy / Challenge S / Indigo 2
pub const PORT_CPM: i32 = 58; // PPC CPM type number
pub const PORT_MPC52xx: i32 = 59; // MPC52xx (and MPC512x) type numbers
pub const PORT_ICOM: i32 = 60; // IBM icom
pub const PORT_IMX: i32 = 62; // Motorola i.MX SoC
pub const PORT_TXX9: i32 = 64; // TXX9 type number
pub const PORT_MUEX50: i32 = 65; // Moxa MUEx50 UART
pub const PORT_JSM: i32 = 69; // Digi jsm
pub const PORT_SUNHV: i32 = 72; // SUN4V Hypervisor Console
pub const PORT_UARTLITE: i32 = 74; // Xilinx uartlite
pub const PORT_BCM7271: i32 = 76; // Broadcom BCM7271 UART
pub const PORT_SB1250_DUART: i32 = 77; // Broadcom SB1250, etc. SOC
pub const PORT_MCF: i32 = 78; // Freescale ColdFire
pub const PORT_SC26XX: i32 = 82;
pub const PORT_SCIFA: i32 = 83; // SH-SCI
pub const PORT_S3C6400: i32 = 84;
pub const PORT_MAX3100: i32 = 86; // MAX3100
pub const PORT_TIMBUART: i32 = 87; // Timberdale UART
pub const PORT_MSM: i32 = 88; // Qualcomm MSM SoCs
pub const PORT_BCM63XX: i32 = 89; // BCM63xx family SoCs
pub const PORT_APBUART: i32 = 90; // Aeroflex Gaisler GRLIB APBUART
pub const PORT_ALTERA_JTAGUART: i32 = 91; // Altera UARTs
pub const PORT_ALTERA_UART: i32 = 92;
pub const PORT_SCIFB: i32 = 93; // SH-SCI
pub const PORT_MAX310X: i32 = 94; // MAX310X
pub const PORT_DA830: i32 = 95; // TI DA8xx/66AK2x
pub const PORT_OMAP: i32 = 96; // TI OMAP-UART
pub const PORT_VT8500: i32 = 97; // VIA VT8500 SoC
pub const PORT_XUARTPS: i32 = 98; // Cadence (Xilinx Zynq) UART
pub const PORT_AR933X: i32 = 99; // Atheros AR933X SoC
pub const PORT_MCHP16550A: i32 = 100; // MCHP 16550A UART with 256 byte FIFOs
pub const PORT_ARC: i32 = 101; // ARC (Synopsys) on-chip UART
pub const PORT_RP2: i32 = 102; // Rocketport EXPRESS/INFINITY
pub const PORT_LPUART: i32 = 103; // Freescale lpuart
pub const PORT_HSCIF: i32 = 104; // SH-SCI
pub const PORT_ASC: i32 = 105; // ST ASC type numbers
pub const PORT_MEN_Z135: i32 = 107; // MEN 16z135 UART
pub const PORT_SC16IS7XX: i32 = 108; // SC16IS7xx
pub const PORT_MESON: i32 = 109; // MESON
pub const PORT_DIGICOLOR: i32 = 110; // Conexant Digicolor
pub const PORT_SPRD: i32 = 111; // SPRD SERIAL
pub const PORT_STM32: i32 = 113; // STM32 USART
pub const PORT_MVEBU: i32 = 114; // MVEBU UART
pub const PORT_PIC32: i32 = 115; // Microchip PIC32 UART
pub const PORT_MPS2UART: i32 = 116; // MPS2 UART
pub const PORT_MTK_BTIF: i32 = 117; // MediaTek BTIF
pub const PORT_RDA: i32 = 118; // RDA UART
pub const PORT_MLB_USIO: i32 = 119; // Socionext Milbeaut UART
pub const PORT_SIFIVE_V0: i32 = 120; // SiFive UART
pub const PORT_SUNIX: i32 = 121; // Sunix UART
pub const PORT_LINFLEXUART: i32 = 122; // Freescale LINFlexD UART
pub const PORT_SUNPLUS: i32 = 123; // Sunplus UART

/* Generic type identifier for ports whose type is not important to userspace. */
pub const PORT_GENERIC: i32 = -1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
