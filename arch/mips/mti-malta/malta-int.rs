/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Carsten Langgaard, carstenl@mips.com
 * Copyright (C) 2000, 2001, 2004 MIPS Technologies, Inc.
 * Copyright (C) 2001 Ralf Baechle
 * Copyright (C) 2013 Imagination Technologies Ltd.
 *
 * Routines for generic manipulation of the interrupts found on the MIPS
 * Malta board. The interrupt controller is located in the South Bridge
 * a PIIX4 device with two internal 82C95 interrupt controllers.
 */

// Linux and architecture headers from the C source provide the referenced
// constants, types, globals, functions, and low-level register operations.

unsafe fn mips_pcibios_iack() -> i32 {
    let mut irq: i32;
    match mips_revision_sconid {
        MIPS_REVISION_SCON_SOCIT
        | MIPS_REVISION_SCON_ROCIT
        | MIPS_REVISION_SCON_SOCITSC
        | MIPS_REVISION_SCON_SOCITSCP => {
            MSC_READ(MSC01_PCI_IACK, irq);
            irq &= 0xff;
        }
        MIPS_REVISION_SCON_GT64120 => {
            irq = GT_READ(GT_PCI0_IACK_OFS);
            irq &= 0xff;
        }
        MIPS_REVISION_SCON_BONITO => {
            // Generate a PCI IACK cycle on the Bonito controller.
            BONITO_PCIMAP_CFG = 0x20000;
            let _ = BONITO_PCIMAP_CFG;
            iob();
            irq = __raw_readl(_pcictrl_bonito_pcicfg as *mut u32) as i32;
            iob();
            irq &= 0xff;
            BONITO_PCIMAP_CFG = 0;
        }
        _ => {
            pr_emerg!("Unknown system controller.\n");
            return -1;
        }
    }
    irq
}

unsafe fn corehi_irqdispatch() {
    let (mut intedge, mut intsteer, mut pcicmd, mut pcibadaddr): (u32, u32, u32, u32);
    let (mut pcimstat, mut intisr, mut inten, mut intpol): (u32, u32, u32, u32);
    let (mut intrcause, mut datalo, mut datahi): (u32, u32, u32);
    let regs = get_irq_regs();

    pr_emerg!("CoreHI interrupt, shouldn't happen, we die here!\n");
    pr_emerg!("epc\t : %08lx\nStatus: %08lx\nCause : %08lx\nbadVaddr : %08lx\n",
        (*regs).cp0_epc, (*regs).cp0_status, (*regs).cp0_cause, (*regs).cp0_badvaddr);

    match mips_revision_sconid {
        MIPS_REVISION_SCON_SOCIT
        | MIPS_REVISION_SCON_ROCIT
        | MIPS_REVISION_SCON_SOCITSC
        | MIPS_REVISION_SCON_SOCITSCP => ll_msc_irq(),
        MIPS_REVISION_SCON_GT64120 => {
            intrcause = GT_READ(GT_INTRCAUSE_OFS);
            datalo = GT_READ(GT_CPUERR_ADDRLO_OFS);
            datahi = GT_READ(GT_CPUERR_ADDRHI_OFS);
            pr_emerg!("GT_INTRCAUSE = %08x\n", intrcause);
            pr_emerg!("GT_CPUERR_ADDR = %02x%08x\n", datahi, datalo);
        }
        MIPS_REVISION_SCON_BONITO => {
            pcibadaddr = BONITO_PCIBADADDR;
            pcimstat = BONITO_PCIMSTAT;
            intisr = BONITO_INTISR;
            inten = BONITO_INTEN;
            intpol = BONITO_INTPOL;
            intedge = BONITO_INTEDGE;
            intsteer = BONITO_INTSTEER;
            pcicmd = BONITO_PCICMD;
            pr_emerg!("BONITO_INTISR = %08x\n", intisr);
            pr_emerg!("BONITO_INTEN = %08x\n", inten);
            pr_emerg!("BONITO_INTPOL = %08x\n", intpol);
            pr_emerg!("BONITO_INTEDGE = %08x\n", intedge);
            pr_emerg!("BONITO_INTSTEER = %08x\n", intsteer);
            pr_emerg!("BONITO_PCICMD = %08x\n", pcicmd);
            pr_emerg!("BONITO_PCIBADADDR = %08x\n", pcibadaddr);
            pr_emerg!("BONITO_PCIMSTAT = %08x\n", pcimstat);
        }
        _ => {}
    }
    die!("CoreHi interrupt", regs);
}

unsafe extern "C" fn corehi_handler(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    corehi_irqdispatch();
    IRQ_HANDLED
}

static mut MSC_IRQMAP: [msc_irqmap_t; 2] = [
    msc_irqmap_t { intr: MSC01C_INT_TMR, level: MSC01_IRQ_EDGE, flags: 0 },
    msc_irqmap_t { intr: MSC01C_INT_PCI, level: MSC01_IRQ_LEVEL, flags: 0 },
];
static mut MSC_NR_IRQS: i32 = 2;

static mut MSC_EICIRQMAP: [msc_irqmap_t; 10] = [
    msc_irqmap_t { intr: MSC01E_INT_SW0, level: MSC01_IRQ_LEVEL, flags: 0 },
    msc_irqmap_t { intr: MSC01E_INT_SW1, level: MSC01_IRQ_LEVEL, flags: 0 },
    msc_irqmap_t { intr: MSC01E_INT_I8259A, level: MSC01_IRQ_LEVEL, flags: 0 },
    msc_irqmap_t { intr: MSC01E_INT_SMI, level: MSC01_IRQ_LEVEL, flags: 0 },
    msc_irqmap_t { intr: MSC01E_INT_COREHI, level: MSC01_IRQ_LEVEL, flags: 0 },
    msc_irqmap_t { intr: MSC01E_INT_CORELO, level: MSC01_IRQ_LEVEL, flags: 0 },
    msc_irqmap_t { intr: MSC01E_INT_TMR, level: MSC01_IRQ_EDGE, flags: 0 },
    msc_irqmap_t { intr: MSC01E_INT_PCI, level: MSC01_IRQ_LEVEL, flags: 0 },
    msc_irqmap_t { intr: MSC01E_INT_PERFCTR, level: MSC01_IRQ_LEVEL, flags: 0 },
    msc_irqmap_t { intr: MSC01E_INT_CPUCTR, level: MSC01_IRQ_LEVEL, flags: 0 },
];
static mut MSC_NR_EICIRQS: i32 = 10;

unsafe fn arch_init_irq() {
    let corehi_irq: i32;
    WARN!(irq_alloc_descs(I8259A_IRQ_BASE, I8259A_IRQ_BASE, 16, numa_node_id()) < 0,
        "Cannot reserve i8259 virqs at IRQ%d\n", I8259A_IRQ_BASE);
    i8259_set_poll(mips_pcibios_iack);
    irqchip_init();

    match mips_revision_sconid {
        MIPS_REVISION_SCON_SOCIT | MIPS_REVISION_SCON_ROCIT => {
            if cpu_has_veic {
                init_msc_irqs(MIPS_MSC01_IC_REG_BASE, MSC01E_INT_BASE,
                    MSC_EICIRQMAP.as_ptr(), MSC_NR_EICIRQS);
            } else {
                init_msc_irqs(MIPS_MSC01_IC_REG_BASE, MSC01C_INT_BASE,
                    MSC_IRQMAP.as_ptr(), MSC_NR_IRQS);
            }
        }
        MIPS_REVISION_SCON_SOCITSC | MIPS_REVISION_SCON_SOCITSCP => {
            if cpu_has_veic {
                init_msc_irqs(MIPS_SOCITSC_IC_REG_BASE, MSC01E_INT_BASE,
                    MSC_EICIRQMAP.as_ptr(), MSC_NR_EICIRQS);
            } else {
                init_msc_irqs(MIPS_SOCITSC_IC_REG_BASE, MSC01C_INT_BASE,
                    MSC_IRQMAP.as_ptr(), MSC_NR_IRQS);
            }
        }
        _ => {}
    }

    if mips_gic_present() {
        corehi_irq = MIPS_CPU_IRQ_BASE + MIPSCPU_INT_COREHI;
    } else if cpu_has_veic {
        set_vi_handler(MSC01E_INT_COREHI, corehi_irqdispatch);
        corehi_irq = MSC01E_INT_BASE + MSC01E_INT_COREHI;
    } else {
        corehi_irq = MIPS_CPU_IRQ_BASE + MIPSCPU_INT_COREHI;
    }

    if request_irq(corehi_irq, corehi_handler, IRQF_NO_THREAD, "CoreHi", core::ptr::null_mut()) != 0 {
        pr_err!("Failed to request irq %d (CoreHi)\n", corehi_irq);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
