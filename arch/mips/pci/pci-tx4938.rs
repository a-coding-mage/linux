/*
 * Based on linux/arch/mips/txx9/rbtx4938/setup.c,
 *            and RBTX49xx patch from CELF patch archive.
 *
 * Copyright 2001, 2003-2005 MontaVista Software Inc.
 * Copyright (C) 2004 by Ralf Baechle (ralf@linux-mips.org)
 * (C) Copyright TOSHIBA CORPORATION 2000-2001, 2004-2007
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    static mut tx4938_ccfgptr: *mut tx4938_ccfg;
    static mut tx4938_pcic1ptr: *mut core::ffi::c_void;
    static mut txx9_cpu_clock: i32;
    static mut txx9_gbus_clock: u32;

    fn __raw_readq(addr: *const u64) -> u64;
    fn tx4938_ccfg_set(bits: u64);
    fn tx4938_ccfg_change(mask: u64, value: u32);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn pr_cont(fmt: *const core::ffi::c_char, ...);
    fn pr_debug(fmt: *const core::ffi::c_char, ...);
    fn pr_warn(fmt: *const core::ffi::c_char, ...);
    fn get_tx4927_pcicptr(sysdata: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn request_irq(irq: u32, handler: unsafe extern "C" fn(), flags: u32,
                   name: *const core::ffi::c_char, dev: *mut core::ffi::c_void) -> i32;
    fn tx4927_pcierr_interrupt();
}

#[repr(C)]
struct tx4938_ccfg {
    ccfg: u64,
    pcfg: u64,
}

#[repr(C)]
struct pci_bus {
    sysdata: *mut core::ffi::c_void,
}

#[repr(C)]
struct pci_dev {
    bus: *mut pci_bus,
}

// Constants and macros are provided by the translated platform headers.
extern "C" {
    static TX4938_CCFG_PCI66: u64;
    static TX4938_PCFG_PCICLKEN_ALL: u64;
    static TX4938_CCFG_PCIDIVMODE_MASK: u64;
    static TX4938_CCFG_PCIDIVMODE_4: u64;
    static TX4938_CCFG_PCIDIVMODE_4_5: u64;
    static TX4938_CCFG_PCIDIVMODE_5: u64;
    static TX4938_CCFG_PCIDIVMODE_5_5: u64;
    static TX4938_CCFG_PCIDIVMODE_8: u64;
    static TX4938_CCFG_PCIDIVMODE_9: u64;
    static TX4938_CCFG_PCIDIVMODE_10: u64;
    static TX4938_CCFG_PCIDIVMODE_11: u64;
    static TX4938_CCFG_PCI1DMD: u64;
    static TX4938_CCFG_PCI1_66: u64;
    static TX4938_PCFG_ETH0_SEL: u64;
    static TX4938_PCFG_ETH1_SEL: u64;
    static TX4927_PCIC_REG: *mut core::ffi::c_void;
    static TXX9_IRQ_BASE: u32;
    static TX4938_IR_PCIERR: u32;
    static TX4938_IR_ETH0: u32;
    static TX4938_IR_ETH1: u32;
}

pub unsafe extern "C" fn tx4938_report_pciclk() -> i32 {
    let mut pciclk: i32 = 0;

    pr_info(b"PCIC --%s PCICLK:\0".as_ptr() as _,
        if __raw_readq(&(*tx4938_ccfgptr).ccfg) & TX4938_CCFG_PCI66 != 0 {
            b" PCI66\0".as_ptr() as _
        } else { b"\0".as_ptr() as _ });
    if __raw_readq(&(*tx4938_ccfgptr).pcfg) & TX4938_PCFG_PCICLKEN_ALL != 0 {
        let ccfg = __raw_readq(&(*tx4938_ccfgptr).ccfg);
        match ccfg & TX4938_CCFG_PCIDIVMODE_MASK {
            TX4938_CCFG_PCIDIVMODE_4 => pciclk = txx9_cpu_clock / 4,
            TX4938_CCFG_PCIDIVMODE_4_5 => pciclk = txx9_cpu_clock * 2 / 9,
            TX4938_CCFG_PCIDIVMODE_5 => pciclk = txx9_cpu_clock / 5,
            TX4938_CCFG_PCIDIVMODE_5_5 => pciclk = txx9_cpu_clock * 2 / 11,
            TX4938_CCFG_PCIDIVMODE_8 => pciclk = txx9_cpu_clock / 8,
            TX4938_CCFG_PCIDIVMODE_9 => pciclk = txx9_cpu_clock / 9,
            TX4938_CCFG_PCIDIVMODE_10 => pciclk = txx9_cpu_clock / 10,
            TX4938_CCFG_PCIDIVMODE_11 => pciclk = txx9_cpu_clock / 11,
            _ => (),
        }
        pr_cont(b"Internal(%u.%uMHz)\0".as_ptr() as _,
            (pciclk + 50000) / 1000000, ((pciclk + 50000) / 100000) % 10);
    } else { pr_cont(b"External\0".as_ptr() as _); pciclk = -1; }
    pr_cont(b"\n\0".as_ptr() as _);
    pciclk
}

pub unsafe extern "C" fn tx4938_report_pci1clk() {
    let ccfg = __raw_readq(&(*tx4938_ccfgptr).ccfg);
    let pciclk = txx9_gbus_clock / if ccfg & TX4938_CCFG_PCI1DMD != 0 { 4 } else { 2 };
    pr_info(b"PCIC1 -- %sPCICLK:%u.%uMHz\n\0".as_ptr() as _,
        if ccfg & TX4938_CCFG_PCI1_66 != 0 { b"PCI66 \0".as_ptr() } else { b"\0".as_ptr() },
        (pciclk + 50000) / 1000000, ((pciclk + 50000) / 100000) % 10);
}

pub unsafe extern "C" fn tx4938_pciclk66_setup() -> i32 {
    tx4938_ccfg_set(TX4938_CCFG_PCI66);
    if __raw_readq(&(*tx4938_ccfgptr).pcfg) & TX4938_PCFG_PCICLKEN_ALL != 0 {
        let mut pcidivmode;
        let ccfg = __raw_readq(&(*tx4938_ccfgptr).ccfg);
        let pciclk = match ccfg & TX4938_CCFG_PCIDIVMODE_MASK {
            TX4938_CCFG_PCIDIVMODE_8 | TX4938_CCFG_PCIDIVMODE_4 => { pcidivmode = TX4938_CCFG_PCIDIVMODE_4; txx9_cpu_clock / 4 }
            TX4938_CCFG_PCIDIVMODE_9 | TX4938_CCFG_PCIDIVMODE_4_5 => { pcidivmode = TX4938_CCFG_PCIDIVMODE_4_5; txx9_cpu_clock * 2 / 9 }
            TX4938_CCFG_PCIDIVMODE_10 | TX4938_CCFG_PCIDIVMODE_5 => { pcidivmode = TX4938_CCFG_PCIDIVMODE_5; txx9_cpu_clock / 5 }
            TX4938_CCFG_PCIDIVMODE_11 | TX4938_CCFG_PCIDIVMODE_5_5 | _ => { pcidivmode = TX4938_CCFG_PCIDIVMODE_5_5; txx9_cpu_clock * 2 / 11 }
        };
        tx4938_ccfg_change(TX4938_CCFG_PCIDIVMODE_MASK, pcidivmode as u32);
        pr_debug(b"PCICLK: ccfg:%08lx\n\0".as_ptr() as _, __raw_readq(&(*tx4938_ccfgptr).ccfg));
        pciclk
    } else { -1 }
}

pub unsafe extern "C" fn tx4938_pcic1_map_irq(dev: *const pci_dev, slot: u8) -> i32 {
    if get_tx4927_pcicptr((*(*dev).bus).sysdata) == tx4938_pcic1ptr {
        match slot {
            31 => if __raw_readq(&(*tx4938_ccfgptr).pcfg) & TX4938_PCFG_ETH0_SEL != 0 { return (TXX9_IRQ_BASE + TX4938_IR_ETH0) as i32 },
            30 => if __raw_readq(&(*tx4938_ccfgptr).pcfg) & TX4938_PCFG_ETH1_SEL != 0 { return (TXX9_IRQ_BASE + TX4938_IR_ETH1) as i32 },
            _ => (),
        }
        return 0;
    }
    -1
}

pub unsafe extern "C" fn tx4938_setup_pcierr_irq() {
    if request_irq(TXX9_IRQ_BASE + TX4938_IR_PCIERR, tx4927_pcierr_interrupt, 0,
                   b"PCI error\0".as_ptr() as _, TX4927_PCIC_REG) != 0 {
        pr_warn(b"Failed to request irq for PCIERR\n\0".as_ptr() as _);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
