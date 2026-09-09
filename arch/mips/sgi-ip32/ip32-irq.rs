/*
 * Code to handle IP32 IRQs
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2000 Harald Koerfgen
 * Copyright (C) 2001 Keith M Wesolowski
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

unsafe fn flush_crime_bus() { let _ = crime.control; }
unsafe fn flush_mace_bus() { let _ = mace.perif.ctrl.misc; }

static mut crime_mask: u64 = 0;

unsafe fn crime_enable_irq(d: *mut irq_data) {
    let bit = (*d).irq - CRIME_IRQ_BASE;
    crime_mask |= 1u64 << bit;
    crime.imask = crime_mask;
}

unsafe fn crime_disable_irq(d: *mut irq_data) {
    let bit = (*d).irq - CRIME_IRQ_BASE;
    crime_mask &= !(1u64 << bit);
    crime.imask = crime_mask;
    flush_crime_bus();
}

static mut crime_level_interrupt: irq_chip = irq_chip {
    name: "IP32 CRIME\0".as_ptr() as *const i8,
    irq_mask: Some(crime_disable_irq), irq_unmask: Some(crime_enable_irq), ..irq_chip::zeroed()
};

unsafe fn crime_edge_mask_and_ack_irq(d: *mut irq_data) {
    let bit = (*d).irq - CRIME_IRQ_BASE;
    let mut crime_int = crime.hard_int;
    crime_int &= !(1u64 << bit);
    crime.hard_int = crime_int;
    crime_disable_irq(d);
}

static mut crime_edge_interrupt: irq_chip = irq_chip {
    name: "IP32 CRIME\0".as_ptr() as *const i8,
    irq_ack: Some(crime_edge_mask_and_ack_irq), irq_mask: Some(crime_disable_irq),
    irq_mask_ack: Some(crime_edge_mask_and_ack_irq), irq_unmask: Some(crime_enable_irq), ..irq_chip::zeroed()
};

static mut macepci_mask: c_ulong = 0;
unsafe fn enable_macepci_irq(d: *mut irq_data) {
    macepci_mask |= MACEPCI_CONTROL_INT((*d).irq - MACEPCI_SCSI0_IRQ);
    mace.pci.control = macepci_mask;
    crime_mask |= 1u64 << ((*d).irq - CRIME_IRQ_BASE);
    crime.imask = crime_mask;
}
unsafe fn disable_macepci_irq(d: *mut irq_data) {
    crime_mask &= !(1u64 << ((*d).irq - CRIME_IRQ_BASE)); crime.imask = crime_mask; flush_crime_bus();
    macepci_mask &= !MACEPCI_CONTROL_INT((*d).irq - MACEPCI_SCSI0_IRQ); mace.pci.control = macepci_mask; flush_mace_bus();
}
static mut ip32_macepci_interrupt: irq_chip = irq_chip { name: "IP32 MACE PCI\0".as_ptr() as *const i8, irq_mask: Some(disable_macepci_irq), irq_unmask: Some(enable_macepci_irq), ..irq_chip::zeroed() };

const MACEISA_AUDIO_INT: c_ulong = MACEISA_AUDIO_SW_INT | MACEISA_AUDIO_SC_INT | MACEISA_AUDIO1_DMAT_INT | MACEISA_AUDIO1_OF_INT | MACEISA_AUDIO2_DMAT_INT | MACEISA_AUDIO2_MERR_INT | MACEISA_AUDIO3_DMAT_INT | MACEISA_AUDIO3_MERR_INT;
const MACEISA_MISC_INT: c_ulong = MACEISA_RTC_INT | MACEISA_KEYB_INT | MACEISA_KEYB_POLL_INT | MACEISA_MOUSE_INT | MACEISA_MOUSE_POLL_INT | MACEISA_TIMER0_INT | MACEISA_TIMER1_INT | MACEISA_TIMER2_INT;
const MACEISA_SUPERIO_INT: c_ulong = MACEISA_PARALLEL_INT | MACEISA_PAR_CTXA_INT | MACEISA_PAR_CTXB_INT | MACEISA_PAR_MERR_INT | MACEISA_SERIAL1_INT | MACEISA_SERIAL1_TDMAT_INT | MACEISA_SERIAL1_TDMAPR_INT | MACEISA_SERIAL1_TDMAME_INT | MACEISA_SERIAL1_RDMAT_INT | MACEISA_SERIAL1_RDMAOR_INT | MACEISA_SERIAL2_INT | MACEISA_SERIAL2_TDMAT_INT | MACEISA_SERIAL2_TDMAPR_INT | MACEISA_SERIAL2_TDMAME_INT | MACEISA_SERIAL2_RDMAT_INT | MACEISA_SERIAL2_RDMAOR_INT;
static mut maceisa_mask: c_ulong = 0;

unsafe fn enable_maceisa_irq(d: *mut irq_data) {
    let mut crime_int = 0;
    pr_debug!("maceisa enable: %u\n", (*d).irq);
    match (*d).irq {
        MACEISA_AUDIO_SW_IRQ..=MACEISA_AUDIO3_MERR_IRQ => crime_int = MACE_AUDIO_INT,
        MACEISA_RTC_IRQ..=MACEISA_TIMER2_IRQ => crime_int = MACE_MISC_INT,
        MACEISA_PARALLEL_IRQ..=MACEISA_SERIAL2_RDMAOR_IRQ => crime_int = MACE_SUPERIO_INT,
        _ => {}
    }
    pr_debug!("crime_int %08x enabled\n", crime_int); crime_mask |= crime_int; crime.imask = crime_mask;
    maceisa_mask |= 1 << ((*d).irq - MACEISA_AUDIO_SW_IRQ); mace.perif.ctrl.imask = maceisa_mask;
}
unsafe fn disable_maceisa_irq(d: *mut irq_data) {
    let mut crime_int = 0; maceisa_mask &= !(1 << ((*d).irq - MACEISA_AUDIO_SW_IRQ));
    if maceisa_mask & MACEISA_AUDIO_INT == 0 { crime_int |= MACE_AUDIO_INT; }
    if maceisa_mask & MACEISA_MISC_INT == 0 { crime_int |= MACE_MISC_INT; }
    if maceisa_mask & MACEISA_SUPERIO_INT == 0 { crime_int |= MACE_SUPERIO_INT; }
    crime_mask &= !crime_int; crime.imask = crime_mask; flush_crime_bus(); mace.perif.ctrl.imask = maceisa_mask; flush_mace_bus();
}
unsafe fn mask_and_ack_maceisa_irq(d: *mut irq_data) { let mut mace_int = mace.perif.ctrl.istat; mace_int &= !(1 << ((*d).irq - MACEISA_AUDIO_SW_IRQ)); mace.perif.ctrl.istat = mace_int; disable_maceisa_irq(d); }
static mut ip32_maceisa_level_interrupt: irq_chip = irq_chip { name: "IP32 MACE ISA\0".as_ptr() as *const i8, irq_mask: Some(disable_maceisa_irq), irq_unmask: Some(enable_maceisa_irq), ..irq_chip::zeroed() };
static mut ip32_maceisa_edge_interrupt: irq_chip = irq_chip { name: "IP32 MACE ISA\0".as_ptr() as *const i8, irq_ack: Some(mask_and_ack_maceisa_irq), irq_mask: Some(disable_maceisa_irq), irq_mask_ack: Some(mask_and_ack_maceisa_irq), irq_unmask: Some(enable_maceisa_irq), ..irq_chip::zeroed() };

unsafe fn enable_mace_irq(d: *mut irq_data) { let bit = (*d).irq - CRIME_IRQ_BASE; crime_mask |= 1u64 << bit; crime.imask = crime_mask; }
unsafe fn disable_mace_irq(d: *mut irq_data) { let bit = (*d).irq - CRIME_IRQ_BASE; crime_mask &= !(1u64 << bit); crime.imask = crime_mask; flush_crime_bus(); }
static mut ip32_mace_interrupt: irq_chip = irq_chip { name: "IP32 MACE\0".as_ptr() as *const i8, irq_mask: Some(disable_mace_irq), irq_unmask: Some(enable_mace_irq), ..irq_chip::zeroed() };

unsafe fn ip32_unknown_interrupt() -> ! {
    printk!("Unknown interrupt occurred!\n"); printk!("cp0_status: %08x\n", read_c0_status()); printk!("cp0_cause: %08x\n", read_c0_cause());
    printk!("CRIME intr mask: %016lx\n", crime.imask); printk!("CRIME intr status: %016lx\n", crime.istat); printk!("CRIME hardware intr register: %016lx\n", crime.hard_int);
    printk!("MACE ISA intr mask: %08lx\n", mace.perif.ctrl.imask); printk!("MACE ISA intr status: %08lx\n", mace.perif.ctrl.istat); printk!("MACE PCI control register: %08x\n", mace.pci.control);
    printk!("Register dump:\n"); show_regs(get_irq_regs()); printk!("Please mail this report to linux-mips@vger.kernel.org\n"); printk!("Spinning..."); loop {}
}

unsafe fn ip32_irq0() { let crime_int = crime.istat & crime_mask; if crime_int == 0 { return; } let mut irq = MACE_VID_IN1_IRQ + __ffs(crime_int); if crime_int & CRIME_MACEISA_INT_MASK != 0 { let mace_int = mace.perif.ctrl.istat; irq = __ffs(mace_int & maceisa_mask) + MACEISA_AUDIO_SW_IRQ; } pr_debug!("*irq %u*\n", irq); do_IRQ(irq); }
unsafe fn ip32_irq1() { ip32_unknown_interrupt(); } unsafe fn ip32_irq2() { ip32_unknown_interrupt(); } unsafe fn ip32_irq3() { ip32_unknown_interrupt(); } unsafe fn ip32_irq4() { ip32_unknown_interrupt(); }
unsafe fn ip32_irq5() { do_IRQ(MIPS_CPU_IRQ_BASE + 7); }

pub unsafe fn plat_irq_dispatch() {
    let pending = read_c0_status() & read_c0_cause();
    if pending & IE_IRQ0 != 0 { ip32_irq0(); } else if pending & IE_IRQ1 != 0 { ip32_irq1(); } else if pending & IE_IRQ2 != 0 { ip32_irq2(); } else if pending & IE_IRQ3 != 0 { ip32_irq3(); } else if pending & IE_IRQ4 != 0 { ip32_irq4(); } else if pending & IE_IRQ5 != 0 { ip32_irq5(); }
}

pub unsafe fn arch_init_irq() {
    crime.imask = 0; crime.hard_int = 0; crime.soft_int = 0; mace.perif.ctrl.istat = 0; mace.perif.ctrl.imask = 0; mips_cpu_irq_init();
    for irq in CRIME_IRQ_BASE..=IP32_IRQ_MAX {
        match irq {
            MACE_VID_IN1_IRQ..=MACE_PCI_BRIDGE_IRQ => irq_set_chip_and_handler_name(irq, &mut ip32_mace_interrupt, handle_level_irq, "level"),
            MACEPCI_SCSI0_IRQ..=MACEPCI_SHARED2_IRQ => irq_set_chip_and_handler_name(irq, &mut ip32_macepci_interrupt, handle_level_irq, "level"),
            CRIME_CPUERR_IRQ | CRIME_MEMERR_IRQ => irq_set_chip_and_handler_name(irq, &mut crime_level_interrupt, handle_level_irq, "level"),
            CRIME_GBE0_IRQ..=CRIME_GBE3_IRQ | CRIME_RE_EMPTY_E_IRQ..=CRIME_RE_IDLE_E_IRQ | CRIME_SOFT0_IRQ..=CRIME_SOFT2_IRQ | CRIME_VICE_IRQ => irq_set_chip_and_handler_name(irq, &mut crime_edge_interrupt, handle_edge_irq, "edge"),
            MACEISA_PARALLEL_IRQ | MACEISA_SERIAL1_TDMAPR_IRQ | MACEISA_SERIAL2_TDMAPR_IRQ => irq_set_chip_and_handler_name(irq, &mut ip32_maceisa_edge_interrupt, handle_edge_irq, "edge"),
            _ => irq_set_chip_and_handler_name(irq, &mut ip32_maceisa_level_interrupt, handle_level_irq, "level"),
        }
    }
    if request_irq(CRIME_MEMERR_IRQ, crime_memerr_intr, 0, "CRIME memory error", core::ptr::null_mut()) != 0 { pr_err!("Failed to register CRIME memory error interrupt\n"); }
    if request_irq(CRIME_CPUERR_IRQ, crime_cpuerr_intr, 0, "CRIME CPU error", core::ptr::null_mut()) != 0 { pr_err!("Failed to register CRIME CPU error interrupt\n"); }
    const ALLINTS: u32 = IE_IRQ0 | IE_IRQ1 | IE_IRQ2 | IE_IRQ3 | IE_IRQ4 | IE_IRQ5; change_c0_status(ST0_IM, ALLINTS);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
