/*
 * Based on linux/arch/mips/jmr3927/rbhma3100/irq.c,
 *          linux/arch/mips/tx4927/common/tx4927_irq.c,
 *          linux/arch/mips/tx4938/common/irq.c
 *
 * Copyright 2001, 2003-2005 MontaVista Software Inc.
 * Author: MontaVista Software, Inc.
 *        ahennessy@mvista.com
 *        source@mvista.com
 * Copyright (C) 2000-2001 Toshiba Corporation
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// C dependencies: linux/init.h, linux/interrupt.h, linux/types.h,
// linux/irq.h, and asm/txx9irq.h.

#[repr(C)]
struct txx9_irc_reg {
    cer: u32,
    cr: [u32; 2],
    unused0: u32,
    ilr: [u32; 8],
    unused1: [u32; 4],
    imr: u32,
    unused2: [u32; 7],
    scr: u32,
    unused3: [u32; 7],
    ssr: u32,
    unused4: [u32; 7],
    csr: u32,
}

const TXx9_IRCER_ICE: u32 = 0x00000001;
const TXx9_IRCR_LOW: u32 = 0x00000000;
const TXx9_IRCR_HIGH: u32 = 0x00000001;
const TXx9_IRCR_DOWN: u32 = 0x00000002;
const TXx9_IRCR_UP: u32 = 0x00000003;
const TXx9_IRSCR_EIClrE: u32 = 0x00000100;
const TXx9_IRSCR_EIClr_MASK: u32 = 0x0000000f;
const TXx9_IRCSR_IF: u32 = 0x00010000;
const TXx9_IRCSR_ILV_MASK: u32 = 0x00000700;
const TXx9_IRCSR_IVL_MASK: u32 = 0x0000001f;

const irc_dlevel: u32 = 0;
const irc_elevel: u32 = 1;

#[repr(C)]
struct irq_data { irq: u32 }
#[repr(C)]
struct irq_chip {
    name: *const u8,
    irq_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
    irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    irq_mask_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
    irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
    irq_set_type: Option<unsafe extern "C" fn(*mut irq_data, u32) -> i32>,
}

extern "C" {
    static mut txx9_ircptr: *mut txx9_irc_reg;
    fn __raw_readl(addr: *const u32) -> u32;
    fn __raw_writel(value: u32, addr: *mut u32);
    fn mmiowb();
    fn ioremap(baseaddr: usize, size: usize) -> *mut txx9_irc_reg;
    fn irq_set_chip_and_handler(irq: u32, chip: *mut irq_chip, handler: unsafe extern "C" fn());
    fn handle_level_irq();
}

const TXX9_IRQ_BASE: u32 = 0; // supplied by asm/txx9irq.h
const TXx9_MAX_IR: usize = 32; // supplied by asm/txx9irq.h
const IRQF_TRIGGER_PROBE: u32 = 1 << 6;
const IRQF_TRIGGER_MASK: u32 = 0x0000000f;
const IRQF_TRIGGER_RISING: u32 = 0x00000001;
const IRQF_TRIGGER_FALLING: u32 = 0x00000002;
const IRQF_TRIGGER_HIGH: u32 = 0x00000004;
const IRQF_TRIGGER_LOW: u32 = 0x00000008;

#[repr(C)]
struct txx9_irq_entry { level: u8, mode: u8 }
static mut txx9irq: [txx9_irq_entry; TXx9_MAX_IR] = [txx9_irq_entry { level: 0, mode: 0 }; TXx9_MAX_IR];

#[inline]
fn TXx9_IRCR_EDGE(cr: u32) -> u32 { cr & 0x00000002 }

unsafe extern "C" fn txx9_irq_unmask(d: *mut irq_data) {
    let irq_nr = (*d).irq.wrapping_sub(TXX9_IRQ_BASE);
    let ilrp = &mut (*txx9_ircptr).ilr[((irq_nr % 16) / 2) as usize];
    let ofs = irq_nr / 16 * 16 + (irq_nr & 1) * 8;
    __raw_writel((__raw_readl(ilrp) & !(0xffu32 << ofs)) | ((txx9irq[irq_nr as usize].level as u32) << ofs), ilrp);
}

unsafe extern "C" fn txx9_irq_mask(d: *mut irq_data) {
    let irq_nr = (*d).irq.wrapping_sub(TXX9_IRQ_BASE);
    let ilrp = &mut (*txx9_ircptr).ilr[((irq_nr % 16) / 2) as usize];
    let ofs = irq_nr / 16 * 16 + (irq_nr & 1) * 8;
    __raw_writel(__raw_readl(ilrp) & !(0xffu32 << ofs) | (irc_dlevel << ofs), ilrp);
    mmiowb();
}

unsafe extern "C" fn txx9_irq_mask_ack(d: *mut irq_data) {
    let irq_nr = (*d).irq.wrapping_sub(TXX9_IRQ_BASE);
    txx9_irq_mask(d);
    if TXx9_IRCR_EDGE(txx9irq[irq_nr as usize].mode as u32) != 0 {
        __raw_writel(TXx9_IRSCR_EIClrE | irq_nr, &mut (*txx9_ircptr).scr);
    }
}

unsafe extern "C" fn txx9_irq_set_type(d: *mut irq_data, flow_type: u32) -> i32 {
    let irq_nr = (*d).irq.wrapping_sub(TXX9_IRQ_BASE);
    if flow_type & IRQF_TRIGGER_PROBE != 0 { return 0; }
    let mode = match flow_type & IRQF_TRIGGER_MASK {
        IRQF_TRIGGER_RISING => TXx9_IRCR_UP,
        IRQF_TRIGGER_FALLING => TXx9_IRCR_DOWN,
        IRQF_TRIGGER_HIGH => TXx9_IRCR_HIGH,
        IRQF_TRIGGER_LOW => TXx9_IRCR_LOW,
        _ => return -22,
    };
    let crp = &mut (*txx9_ircptr).cr[(irq_nr / 8) as usize];
    let ofs = (irq_nr & 7) * 2;
    let mut cr = __raw_readl(crp) & !(0x3u32 << ofs);
    cr |= (mode & 0x3) << ofs;
    __raw_writel(cr, crp);
    txx9irq[irq_nr as usize].mode = mode as u8;
    0
}

static mut txx9_irq_chip: irq_chip = irq_chip {
    name: b"TXX9\0".as_ptr(), irq_ack: Some(txx9_irq_mask_ack), irq_mask: Some(txx9_irq_mask),
    irq_mask_ack: Some(txx9_irq_mask_ack), irq_unmask: Some(txx9_irq_unmask), irq_set_type: Some(txx9_irq_set_type),
};

pub unsafe extern "C" fn txx9_irq_init(baseaddr: usize) {
    txx9_ircptr = ioremap(baseaddr, core::mem::size_of::<txx9_irc_reg>());
    for i in 0..TXx9_MAX_IR { txx9irq[i].level = 4; txx9irq[i].mode = TXx9_IRCR_LOW as u8; irq_set_chip_and_handler(TXX9_IRQ_BASE + i as u32, &mut txx9_irq_chip, handle_level_irq); }
    __raw_writel(0, &mut (*txx9_ircptr).imr);
    for i in 0..8 { __raw_writel(0, &mut (*txx9_ircptr).ilr[i]); }
    for i in 0..2 { __raw_writel(0, &mut (*txx9_ircptr).cr[i]); }
    __raw_writel(TXx9_IRCER_ICE, &mut (*txx9_ircptr).cer);
    __raw_writel(irc_elevel, &mut (*txx9_ircptr).imr);
}

pub unsafe extern "C" fn txx9_irq_set_pri(irc_irq: i32, new_pri: i32) -> i32 {
    if irc_irq < 0 || irc_irq as usize >= TXx9_MAX_IR { return 0; }
    let old_pri = txx9irq[irc_irq as usize].level as i32;
    txx9irq[irc_irq as usize].level = new_pri as u8;
    old_pri
}

pub unsafe extern "C" fn txx9_irq() -> i32 {
    let csr = __raw_readl(&(*txx9_ircptr).csr);
    if csr & TXx9_IRCSR_IF == 0 { (TXX9_IRQ_BASE + (csr & (TXx9_MAX_IR as u32 - 1))) as i32 } else { -1 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
