/*
 * Cobalt Qube/Raq PCI support
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 1996, 1997, 2002, 2003 by Ralf Baechle
 * Copyright (C) 2001, 2002, 2003 by Liam Davies (ldavies@agile.tv)
 */

/* External kernel and platform types, constants, functions, and fixup
 * registration facilities are supplied by the surrounding kernel build. */

const COBALT_PCICONF_CPU: usize = 0x06;
const COBALT_PCICONF_ETH0: usize = 0x07;
const COBALT_PCICONF_RAQSCSI: usize = 0x08;
const COBALT_PCICONF_VIA: usize = 0x09;
const COBALT_PCICONF_PCISLOT: usize = 0x0A;
const COBALT_PCICONF_ETH1: usize = 0x0C;

const VIA_COBALT_BRD_ID_REG: u16 = 0x94;

#[inline]
const fn via_cobalt_brd_reg_to_id(reg: u8) -> u8 {
    reg >> 4
}

/*
 * Default value of PCI Class Code on GT64111 is PCI_CLASS_MEMORY_OTHER (0x0580)
 * instead of PCI_CLASS_BRIDGE_HOST (0x0600). Galileo explained this choice in
 * document "GT-64111 System Controller for RC4640, RM523X and VR4300 CPUs",
 * section "6.5.3 PCI Autoconfiguration at RESET":
 *
 *   Some PCs refuse to configure host bridges if they are found plugged into
 *   a PCI slot (ask the BIOS vendors why...). The "Memory Controller" Class
 *   Code does not cause a problem for these non-compliant BIOSes, so we used
 *   this as the default in the GT-64111.
 *
 * So fix the incorrect default value of PCI Class Code. More details are on:
 * https://lore.kernel.org/r/20211102154831.xtrlgrmrizl5eidl@pali/
 * https://lore.kernel.org/r/20211102150201.GA11675@alpha.franken.de/
 */
unsafe fn qube_raq_galileo_early_fixup(dev: *mut pci_dev) {
    if (*dev).devfn == PCI_DEVFN(0, 0)
        && ((*dev).class >> 8) == PCI_CLASS_MEMORY_OTHER
    {
        (*dev).class = (PCI_CLASS_BRIDGE_HOST << 8) | ((*dev).class & 0xff);
        printk(KERN_INFO, "Galileo: fixed bridge class\n");
    }
}

DECLARE_PCI_FIXUP_EARLY!(PCI_VENDOR_ID_MARVELL, PCI_DEVICE_ID_MARVELL_GT64111,
    qube_raq_galileo_early_fixup);

unsafe fn qube_raq_via_bmIDE_fixup(dev: *mut pci_dev) {
    let mut cfgword: u16 = 0;
    let mut lt: u8 = 0;

    /* Enable Bus Mastering and fast back to back. */
    pci_read_config_word(dev, PCI_COMMAND, &mut cfgword);
    cfgword |= PCI_COMMAND_FAST_BACK | PCI_COMMAND_MASTER;
    pci_write_config_word(dev, PCI_COMMAND, cfgword);

    /* Enable both ide interfaces. ROM only enables primary one.  */
    pci_write_config_byte(dev, 0x40, 0xb);

    /* Set latency timer to reasonable value. */
    pci_read_config_byte(dev, PCI_LATENCY_TIMER, &mut lt);
    if lt < 64 {
        pci_write_config_byte(dev, PCI_LATENCY_TIMER, 64);
    }
    pci_write_config_byte(dev, PCI_CACHE_LINE_SIZE, 8);
}

DECLARE_PCI_FIXUP_HEADER!(PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_82C586_1,
    qube_raq_via_bmIDE_fixup);

unsafe fn qube_raq_galileo_fixup(dev: *mut pci_dev) {
    if (*dev).devfn != PCI_DEVFN(0, 0) {
        return;
    }

    /* Fix PCI latency-timer and cache-line-size values in Galileo
     * host bridge.
     */
    pci_write_config_byte(dev, PCI_LATENCY_TIMER, 64);
    pci_write_config_byte(dev, PCI_CACHE_LINE_SIZE, 8);

    /*
     * The code described by the comment below has been removed
     * as it causes bus mastering by the Ethernet controllers
     * to break under any kind of network load. We always set
     * the retry timeouts to their maximum.
     *
     * --x--x--x--x--x--x--x--x--x--x--x--x--x--x--x--x--x--x--x--x--
     *
     * On all machines prior to Q2, we had the STOP line disconnected
     * from Galileo to VIA on PCI. The new Galileo does not function
     * correctly unless we have it connected.
     *
     * Therefore we must set the disconnect/retry cycle values to
     * something sensible when using the new Galileo.
     */

    printk(KERN_INFO, "Galileo: revision %u\n", (*dev).revision);

    /* The original #if 0 conditional is intentionally disabled. */
    {
        let _timeo: i32;
        /* XXX WE MUST DO THIS ELSE GALILEO LOCKS UP! -DaveM */
        _timeo = GT_READ(GT_PCI0_TOR_OFS);
        /* Old Galileo, assumes PCI STOP line to VIA is disconnected. */
        GT_WRITE(GT_PCI0_TOR_OFS,
            (0xff << 16) |
            (0xff << 8) |
            0xff);

        /* enable PCI retry exceeded interrupt */
        GT_WRITE(GT_INTRMASK_OFS,
            GT_INTR_RETRYCTR0_MSK | GT_READ(GT_INTRMASK_OFS));
    }
}

DECLARE_PCI_FIXUP_HEADER!(PCI_VENDOR_ID_MARVELL, PCI_DEVICE_ID_MARVELL_GT64111,
    qube_raq_galileo_fixup);

static mut cobalt_board_id: i32 = 0;

unsafe fn qube_raq_via_board_id_fixup(dev: *mut pci_dev) {
    let mut id: u8 = 0;
    let retval: i32 = pci_read_config_byte(dev, VIA_COBALT_BRD_ID_REG, &mut id);
    if retval != 0 {
        panic!("Cannot read board ID");
        return;
    }

    cobalt_board_id = via_cobalt_brd_reg_to_id(id) as i32;
    printk(KERN_INFO, "Cobalt board ID: %d\n", cobalt_board_id);
}

DECLARE_PCI_FIXUP_HEADER!(PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_82C586_0,
    qube_raq_via_board_id_fixup);

static mut irq_tab_qube1: [i8; 13] = [
    0, 0, 0, 0, 0, 0, 0, QUBE1_ETH0_IRQ, SCSI_IRQ, 0, PCISLOT_IRQ, 0, 0,
];

static mut irq_tab_cobalt: [i8; 13] = [
    0, 0, 0, 0, 0, 0, 0, ETH0_IRQ, SCSI_IRQ, 0, PCISLOT_IRQ, 0, ETH1_IRQ,
];

static mut irq_tab_raq2: [i8; 13] = [
    0, 0, 0, 0, 0, 0, 0, ETH0_IRQ, RAQ2_SCSI_IRQ, 0, PCISLOT_IRQ, 0, ETH1_IRQ,
];

unsafe fn pcibios_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    let _ = (dev, pin);
    if cobalt_board_id <= COBALT_BRD_ID_QUBE1 {
        return irq_tab_qube1[slot as usize] as i32;
    }

    if cobalt_board_id == COBALT_BRD_ID_RAQ2 {
        return irq_tab_raq2[slot as usize] as i32;
    }

    irq_tab_cobalt[slot as usize] as i32
}

/* Do platform specific device initialization at pci_enable_device() time */
unsafe fn pcibios_plat_dev_init(dev: *mut pci_dev) -> i32 {
    let _ = dev;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
