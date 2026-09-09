/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/mach-pxa/include/mach/irqs.h
 *
 *  Author: Nicolas Pitre
 *  Created: Jun 15, 2001
 *  Copyright: MontaVista Software Inc.
 */

// Dependency supplied by asm/irq.h in the original header.

macro_rules! PXA_ISA_IRQ {
    ($x:expr) => { $x };
}

macro_rules! PXA_IRQ {
    ($x:expr) => { NR_IRQS_LEGACY + ($x) };
}

pub const IRQ_SSP3: i32 = PXA_IRQ!(0); /* SSP3 service request */
pub const IRQ_MSL: i32 = PXA_IRQ!(1); /* MSL Interface interrupt */
pub const IRQ_USBH2: i32 = PXA_IRQ!(2); /* USB Host interrupt 1 (OHCI,PXA27x) */
pub const IRQ_USBH1: i32 = PXA_IRQ!(3); /* USB Host interrupt 2 (non-OHCI,PXA27x) */
pub const IRQ_KEYPAD: i32 = PXA_IRQ!(4); /* Key pad controller */
pub const IRQ_MEMSTK: i32 = PXA_IRQ!(5); /* Memory Stick interrupt (PXA27x) */
pub const IRQ_ACIPC0: i32 = PXA_IRQ!(5); /* AP-CP Communication (PXA930) */
pub const IRQ_PWRI2C: i32 = PXA_IRQ!(6); /* Power I2C interrupt */
pub const IRQ_HWUART: i32 = PXA_IRQ!(7); /* HWUART Transmit/Receive/Error (PXA26x) */
pub const IRQ_OST_4_11: i32 = PXA_IRQ!(7); /* OS timer 4-11 matches (PXA27x) */
pub const IRQ_GPIO0: i32 = PXA_IRQ!(8); /* GPIO0 Edge Detect */
pub const IRQ_GPIO1: i32 = PXA_IRQ!(9); /* GPIO1 Edge Detect */
pub const IRQ_GPIO_2_x: i32 = PXA_IRQ!(10); /* GPIO[2-x] Edge Detect */
pub const IRQ_USB: i32 = PXA_IRQ!(11); /* USB Service */
pub const IRQ_PMU: i32 = PXA_IRQ!(12); /* Performance Monitoring Unit */
pub const IRQ_I2S: i32 = PXA_IRQ!(13); /* I2S Interrupt (PXA27x) */
pub const IRQ_SSP4: i32 = PXA_IRQ!(13); /* SSP4 service request (PXA3xx) */
pub const IRQ_AC97: i32 = PXA_IRQ!(14); /* AC97 Interrupt */
pub const IRQ_ASSP: i32 = PXA_IRQ!(15); /* Audio SSP Service Request (PXA25x) */
pub const IRQ_USIM: i32 = PXA_IRQ!(15); /* Smart Card interface interrupt (PXA27x) */
pub const IRQ_NSSP: i32 = PXA_IRQ!(16); /* Network SSP Service Request (PXA25x) */
pub const IRQ_SSP2: i32 = PXA_IRQ!(16); /* SSP2 interrupt (PXA27x) */
pub const IRQ_LCD: i32 = PXA_IRQ!(17); /* LCD Controller Service Request */
pub const IRQ_I2C: i32 = PXA_IRQ!(18); /* I2C Service Request */
pub const IRQ_ICP: i32 = PXA_IRQ!(19); /* ICP Transmit/Receive/Error */
pub const IRQ_ACIPC2: i32 = PXA_IRQ!(19); /* AP-CP Communication (PXA930) */
pub const IRQ_STUART: i32 = PXA_IRQ!(20); /* STUART Transmit/Receive/Error */
pub const IRQ_BTUART: i32 = PXA_IRQ!(21); /* BTUART Transmit/Receive/Error */
pub const IRQ_FFUART: i32 = PXA_IRQ!(22); /* FFUART Transmit/Receive/Error*/
pub const IRQ_MMC: i32 = PXA_IRQ!(23); /* MMC Status/Error Detection */
pub const IRQ_SSP: i32 = PXA_IRQ!(24); /* SSP Service Request */
pub const IRQ_DMA: i32 = PXA_IRQ!(25); /* DMA Channel Service Request */
pub const IRQ_OST0: i32 = PXA_IRQ!(26); /* OS Timer match 0 */
pub const IRQ_OST1: i32 = PXA_IRQ!(27); /* OS Timer match 1 */
pub const IRQ_OST2: i32 = PXA_IRQ!(28); /* OS Timer match 2 */
pub const IRQ_OST3: i32 = PXA_IRQ!(29); /* OS Timer match 3 */
pub const IRQ_RTC1Hz: i32 = PXA_IRQ!(30); /* RTC HZ Clock Tick */
pub const IRQ_RTCAlrm: i32 = PXA_IRQ!(31); /* RTC Alarm */

pub const IRQ_TPM: i32 = PXA_IRQ!(32); /* TPM interrupt */
pub const IRQ_CAMERA: i32 = PXA_IRQ!(33); /* Camera Interface */
pub const IRQ_CIR: i32 = PXA_IRQ!(34); /* Consumer IR */
pub const IRQ_COMM_WDT: i32 = PXA_IRQ!(35); /* Comm WDT interrupt */
pub const IRQ_TSI: i32 = PXA_IRQ!(36); /* Touch Screen Interface (PXA320) */
pub const IRQ_ENHROT: i32 = PXA_IRQ!(37); /* Enhanced Rotary (PXA930) */
pub const IRQ_USIM2: i32 = PXA_IRQ!(38); /* USIM2 Controller */
pub const IRQ_GCU: i32 = PXA_IRQ!(39); /* Graphics Controller (PXA3xx) */
pub const IRQ_ACIPC1: i32 = PXA_IRQ!(40); /* AP-CP Communication (PXA930) */
pub const IRQ_MMC2: i32 = PXA_IRQ!(41); /* MMC2 Controller */
pub const IRQ_TRKBALL: i32 = PXA_IRQ!(43); /* Track Ball (PXA930) */
pub const IRQ_1WIRE: i32 = PXA_IRQ!(44); /* 1-Wire Controller */
pub const IRQ_NAND: i32 = PXA_IRQ!(45); /* NAND Controller */
pub const IRQ_USB2: i32 = PXA_IRQ!(46); /* USB 2.0 Device Controller */
pub const IRQ_WAKEUP0: i32 = PXA_IRQ!(49); /* EXT_WAKEUP0 */
pub const IRQ_WAKEUP1: i32 = PXA_IRQ!(50); /* EXT_WAKEUP1 */
pub const IRQ_DMEMC: i32 = PXA_IRQ!(51); /* Dynamic Memory Controller */
pub const IRQ_MMC3: i32 = PXA_IRQ!(55); /* MMC3 Controller (PXA310) */

pub const IRQ_U2O: i32 = PXA_IRQ!(64); /* USB OTG 2.0 Controller (PXA935) */
pub const IRQ_U2H: i32 = PXA_IRQ!(65); /* USB Host 2.0 Controller (PXA935) */
pub const IRQ_PXA935_MMC0: i32 = PXA_IRQ!(72); /* MMC0 Controller (PXA935) */
pub const IRQ_PXA935_MMC1: i32 = PXA_IRQ!(73); /* MMC1 Controller (PXA935) */
pub const IRQ_PXA935_MMC2: i32 = PXA_IRQ!(74); /* MMC2 Controller (PXA935) */
pub const IRQ_U2P: i32 = PXA_IRQ!(93); /* USB PHY D+/D- Lines (PXA935) */

pub const PXA_GPIO_IRQ_BASE: i32 = PXA_IRQ!(96);
pub const PXA_NR_BUILTIN_GPIO: i32 = 192;

macro_rules! PXA_GPIO_TO_IRQ {
    ($x:expr) => { PXA_GPIO_IRQ_BASE + ($x) };
}

/*
 * The following interrupts are for board specific purposes. Since
 * the kernel can only run on one machine at a time, we can re-use
 * these.
 * By default, no board IRQ is reserved. It should be finished in
 * custom board since sparse IRQ is already enabled.
 */
pub const IRQ_BOARD_START: i32 = PXA_GPIO_IRQ_BASE + PXA_NR_BUILTIN_GPIO;
pub const PXA_NR_IRQS: i32 = IRQ_BOARD_START;

// The following declarations are excluded from assembly in the C header.
#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn pxa_mask_irq(irq: *mut irq_data);
    pub fn pxa_unmask_irq(irq: *mut irq_data);
    pub fn icip_handle_irq(regs: *mut pt_regs);
    pub fn ichp_handle_irq(regs: *mut pt_regs);
    pub fn pxa_init_irq(irq_nr: i32, set_wake: Option<unsafe extern "C" fn(*mut irq_data, u32) -> i32>);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
