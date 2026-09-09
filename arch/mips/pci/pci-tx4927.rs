/*
 * Based on linux/arch/mips/txx9/rbtx4938/setup.c,
 *          and RBTX49xx patch from CELF patch archive.
 *
 * Copyright 2001, 2003-2005 MontaVista Software Inc.
 * Copyright (C) 2004 by Ralf Baechle (ralf@linux-mips.org)
 * (C) Copyright TOSHIBA CORPORATION 2000-2001, 2004-2007
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// External declarations and constants are supplied by the Linux/MIPS headers.

pub unsafe fn tx4927_report_pciclk() -> i32 {
    let mut pciclk: i32 = 0;

    pr_info(
        "PCIC --{} PCICLK:",
        if (__raw_readq(&(*tx4927_ccfgptr).ccfg) & TX4927_CCFG_PCI66) != 0 {
            " PCI66"
        } else {
            ""
        },
    );
    if (__raw_readq(&(*tx4927_ccfgptr).pcfg) & TX4927_PCFG_PCICLKEN_ALL) != 0 {
        let ccfg: u64 = __raw_readq(&(*tx4927_ccfgptr).ccfg);
        match (ccfg as usize) & TX4927_CCFG_PCIDIVMODE_MASK {
            TX4927_CCFG_PCIDIVMODE_2_5 => {
                pciclk = txx9_cpu_clock * 2 / 5;
            }
            TX4927_CCFG_PCIDIVMODE_3 => {
                pciclk = txx9_cpu_clock / 3;
            }
            TX4927_CCFG_PCIDIVMODE_5 => {
                pciclk = txx9_cpu_clock / 5;
            }
            TX4927_CCFG_PCIDIVMODE_6 => {
                pciclk = txx9_cpu_clock / 6;
            }
            _ => {}
        }
        pr_cont(
            "Internal({}.{}MHz)",
            (pciclk + 50000) / 1000000,
            ((pciclk + 50000) / 100000) % 10,
        );
    } else {
        pr_cont("External");
        pciclk = -1;
    }
    pr_cont("\n");
    pciclk
}

pub unsafe fn tx4927_pciclk66_setup() -> i32 {
    let pciclk: i32;

    /* Assert M66EN */
    tx4927_ccfg_set(TX4927_CCFG_PCI66);
    /* Double PCICLK (if possible) */
    if (__raw_readq(&(*tx4927_ccfgptr).pcfg) & TX4927_PCFG_PCICLKEN_ALL) != 0 {
        let mut pcidivmode: u32 = 0;
        let ccfg: u64 = __raw_readq(&(*tx4927_ccfgptr).ccfg);
        pcidivmode = (ccfg as usize & TX4927_CCFG_PCIDIVMODE_MASK) as u32;
        match pcidivmode {
            TX4927_CCFG_PCIDIVMODE_5 | TX4927_CCFG_PCIDIVMODE_2_5 => {
                pcidivmode = TX4927_CCFG_PCIDIVMODE_2_5;
                pciclk = txx9_cpu_clock * 2 / 5;
            }
            TX4927_CCFG_PCIDIVMODE_6 | TX4927_CCFG_PCIDIVMODE_3 | _ => {
                pcidivmode = TX4927_CCFG_PCIDIVMODE_3;
                pciclk = txx9_cpu_clock / 3;
            }
        }
        tx4927_ccfg_change(TX4927_CCFG_PCIDIVMODE_MASK, pcidivmode);
        pr_debug(
            "PCICLK: ccfg:{:08lx}\n",
            __raw_readq(&(*tx4927_ccfgptr).ccfg) as usize,
        );
    } else {
        pciclk = -1;
    }
    pciclk
}

pub unsafe fn tx4927_setup_pcierr_irq() {
    if request_irq(
        TXX9_IRQ_BASE + TX4927_IR_PCIERR,
        tx4927_pcierr_interrupt,
        0,
        "PCI error",
        TX4927_PCIC_REG as *mut core::ffi::c_void,
    ) != 0 {
        pr_warn("Failed to request irq for PCIERR\n");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
