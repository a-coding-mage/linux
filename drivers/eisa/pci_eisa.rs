// SPDX-License-Identifier: GPL-2.0-only
/*
 * Minimalist driver for a generic PCI-to-EISA bridge.
 *
 * (C) 2003 Marc Zyngier <maz@wild-wind.fr.eu.org>
 *
 * Ivan Kokshaysky <ink@jurassic.park.msu.ru> :
 * Generalisation from i82375 to PCI_CLASS_BRIDGE_EISA.
 */

// Linux kernel dependencies supplied by other translation units.

/* There is only *one* pci_eisa device per machine, right ? */
static mut pci_eisa_root: eisa_root_device = eisa_root_device::default();

unsafe fn pci_eisa_init(pdev: *mut pci_dev) -> i32
{
    let mut res: *mut resource;
    let mut bus_res: *mut resource = core::ptr::null_mut();
    let mut rc: i32;

    rc = pci_enable_device(pdev);
    if rc != 0 {
        dev_err(&mut (*pdev).dev, "Could not enable device\n");
        return rc;
    }

    /*
     * The Intel 82375 PCI-EISA bridge is a subtractive-decode PCI
     * device, so the resources available on EISA are the same as those
     * available on the 82375 bus.  This works the same as a PCI-PCI
     * bridge in subtractive-decode mode (see pci_read_bridge_bases()).
     * We assume other PCI-EISA bridges are similar.
     *
     * eisa_root_register() can only deal with a single io port resource,
     * so we use the first valid io port resource.
     */
    pci_bus_for_each_resource((*pdev).bus, res) {
        if !res.is_null() && ((*res).flags & IORESOURCE_IO) != 0 {
            bus_res = res;
            break;
        }
    }

    if bus_res.is_null() {
        dev_err(&mut (*pdev).dev, "No resources available\n");
        return -1;
    }

    pci_eisa_root.dev = &mut (*pdev).dev;
    pci_eisa_root.res = bus_res;
    pci_eisa_root.bus_base_addr = (*bus_res).start;
    pci_eisa_root.slots = EISA_MAX_SLOTS;
    pci_eisa_root.dma_mask = (*pdev).dma_mask;
    dev_set_drvdata(pci_eisa_root.dev, &mut pci_eisa_root);

    if eisa_root_register(&mut pci_eisa_root) != 0 {
        dev_err(&mut (*pdev).dev, "Could not register EISA root\n");
        return -1;
    }

    0
}

/*
 * We have to call pci_eisa_init_early() before pnpacpi_init()/isapnp_init().
 *   Otherwise pnp resource will get enabled early and could prevent eisa
 *   to be initialized.
 * Also need to make sure pci_eisa_init_early() is called after
 * x86/pci_subsys_init().
 * So need to use subsys_initcall_sync with it.
 */
unsafe fn pci_eisa_init_early() -> i32
{
    let mut dev: *mut pci_dev = core::ptr::null_mut();
    let mut ret: i32;

    for_each_pci_dev(dev) {
        if ((*dev).class >> 8) == PCI_CLASS_BRIDGE_EISA {
            ret = pci_eisa_init(dev);
            if ret != 0 {
                return ret;
            }
        }
    }

    0
}

// Equivalent to: subsys_initcall_sync(pci_eisa_init_early);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
