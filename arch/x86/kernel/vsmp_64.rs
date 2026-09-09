// SPDX-License-Identifier: GPL-2.0-only
/*
 * vSMPowered(tm) systems specific initialization
 * Copyright (C) 2005 ScaleMP Inc.
 *
 * Ravikiran Thirumalai <kiran@scalemp.com>,
 * Shai Fultheim <shai@scalemp.com>
 * Paravirt ops integration: Glauber de Oliveira Costa <gcosta@redhat.com>,
 *                             Ravikiran Thirumalai <kiran@scalemp.com>
 */

// C dependencies supplied by the surrounding kernel translation.

const TOPOLOGY_REGISTER_OFFSET: u32 = 0x10;

// CONFIG_PCI
unsafe fn set_vsmp_ctl() {
    let address: *mut core::ffi::c_void;
    let mut cap: u32;
    let mut ctl: u32;
    let cfg: u32;

    /* set vSMP magic bits to indicate vSMP capable kernel */
    cfg = read_pci_config(0, 0x1f, 0, PCI_BASE_ADDRESS_0);
    address = early_ioremap(cfg as usize, 8);
    cap = readl(address);
    ctl = readl(address.add(4));
    printk!(KERN_INFO, "vSMP CTL: capabilities:0x{:08x}  control:0x{:08x}\n", cap, ctl);

    /* If possible, let the vSMP foundation route the interrupt optimally */
    // CONFIG_SMP
    if cap & ctl & BIT(8) != 0 {
        ctl &= !BIT(8);

        // CONFIG_PROC_FS
        /* Don't let users change irq affinity via procfs */
        no_irq_affinity = 1;
    }

    writel(ctl, address.add(4));
    ctl = readl(address.add(4));
    pr_info!("vSMP CTL: control set to:0x{:08x}\n", ctl);

    early_iounmap(address, 8);
}

static mut is_vsmp: i32 = -1;

unsafe fn detect_vsmp_box() {
    is_vsmp = 0;

    if !early_pci_allowed() {
        return;
    }

    /* Check if we are running on a ScaleMP vSMPowered box */
    if read_pci_config(0, 0x1f, 0, PCI_VENDOR_ID)
        == (PCI_VENDOR_ID_SCALEMP | (PCI_DEVICE_ID_SCALEMP_VSMP_CTL << 16))
    {
        is_vsmp = 1;
    }
}

unsafe fn is_vsmp_box() -> i32 {
    if is_vsmp != -1 {
        is_vsmp
    } else {
        WARN_ON_ONCE!(1);
        0
    }
}

unsafe fn vsmp_cap_cpus() {
    // Active only when !CONFIG_X86_VSMP && CONFIG_SMP && CONFIG_PCI.
    let address: *mut core::ffi::c_void;
    let cfg: u32;
    let topology: u32;
    let mut node_shift: u32;
    let maxcpus: u32;

    /*
     * CONFIG_X86_VSMP is not configured, so limit the number CPUs to the
     * ones present in the first board, unless explicitly overridden by
     * setup_max_cpus
     */
    if setup_max_cpus != NR_CPUS {
        return;
    }

    /* Read the vSMP Foundation topology register */
    cfg = read_pci_config(0, 0x1f, 0, PCI_BASE_ADDRESS_0);
    address = early_ioremap((cfg + TOPOLOGY_REGISTER_OFFSET) as usize, 4);
    if WARN_ON!(address.is_null()) {
        return;
    }

    topology = readl(address);
    node_shift = (topology >> 16) & 0x7;
    if node_shift == 0 {
        /* The value 0 should be decoded as 8 */
        node_shift = 8;
    }
    maxcpus = (topology & ((1u32 << node_shift) - 1)) + 1;

    pr_info!("vSMP CTL: Capping CPUs to {} (CONFIG_X86_VSMP is unset)\n", maxcpus);
    setup_max_cpus = maxcpus;
    early_iounmap(address, 4);
}

pub unsafe fn vsmp_init() {
    detect_vsmp_box();
    if is_vsmp_box() == 0 {
        return;
    }

    vsmp_cap_cpus();

    set_vsmp_ctl();
    return;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
