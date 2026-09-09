// SPDX-License-Identifier: GPL-2.0+

// Dependencies supplied by the Linux PCI and Loongson headers are referenced
// here as external names.

unsafe fn pci_fixup_video(pdev: *mut pci_dev) {
    let res: *mut resource = &mut (*pdev).resource[PCI_ROM_RESOURCE as usize];

    if (*res).start != 0 {
        return;
    }

    if loongson_sysconf.vgabios_addr == 0 {
        return;
    }

    pci_disable_rom(pdev);
    if !(*res).parent.is_null() {
        release_resource(res);
    }

    (*res).start = virt_to_phys(loongson_sysconf.vgabios_addr as *mut core::ffi::c_void);
    (*res).end = (*res).start + 256 * 1024 - 1;
    (*res).flags = IORESOURCE_MEM | IORESOURCE_ROM_SHADOW | IORESOURCE_PCI_FIXED;

    dev_info(&(*pdev).dev, "Video device with shadowed ROM at %pR\n", res);
}

// DECLARE_PCI_FIXUP_CLASS_HEADER(PCI_VENDOR_ID_ATI, 0x9615,
//                                PCI_CLASS_DISPLAY_VGA, 8, pci_fixup_video);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
