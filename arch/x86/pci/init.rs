// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the kernel headers and other translation units:
// linux/pci.h, linux/init.h, asm/pci_x86.h, asm/x86_init.h, asm/irqdomain.h

/* arch_initcall has too random ordering, so call the initializers
   in the right sequence from here. */
unsafe fn pci_arch_init() -> i32 {
    let mut pcbios: i32 = 1;

    let type_: i32 = pci_direct_probe();

    if (pci_probe & PCI_PROBE_NOEARLY) == 0 {
        pci_mmcfg_early_init();
    }

    if x86_init.pci.arch_init.is_some() {
        pcbios = (x86_init.pci.arch_init.unwrap())();
    }

    /*
     * Must happen after x86_init.pci.arch_init(). Xen sets up the
     * x86_init.irqs.create_pci_msi_domain there.
     */
    x86_create_pci_msi_domain();

    if pcbios == 0 {
        return 0;
    }

    pci_pcbios_init();

    /*
     * don't check for raw_pci_ops here because we want pcbios as last
     * fallback, yet it's needed to run first to set pcibios_last_bus
     * in case legacy PCI probing is used. otherwise detecting peer busses
     * fails.
     */
    pci_direct_init(type_);

    if raw_pci_ops.is_null() && raw_pci_ext_ops.is_null() {
        printk(
            KERN_ERR,
            "PCI: Fatal: No config space access function found\n",
        );
    }

    dmi_check_pciprobe();

    dmi_check_skip_isa_align();

    0
}

// Equivalent to: arch_initcall(pci_arch_init);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
