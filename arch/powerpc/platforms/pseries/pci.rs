// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2001 Dave Engebretsen, IBM Corporation
 * Copyright (C) 2003 Anton Blanchard <anton@au.ibm.com>, IBM
 *
 * pSeries specific routines for PCI.
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[cfg(feature = "CONFIG_PCI_IOV")]
const MAX_VFS_FOR_MAP_PE: usize = 256;

#[cfg(feature = "CONFIG_PCI_IOV")]
#[repr(C)]
struct pe_map_bar_entry {
    bar: u64,       // __be64: Input: Virtual Function BAR
    rid: u16,       // __be16: Input: Virtual Function Router ID
    pe_num: u16,    // __be16: Output: Virtual Function PE Number
    reserved: u32,  // Reserved Space
}

#[cfg(feature = "CONFIG_PCI_IOV")]
unsafe fn pseries_send_map_pe(
    pdev: *mut pci_dev,
    num_vfs: u16,
    vf_pe_array: *mut pe_map_bar_entry,
) -> i32 {
    let pdn: *mut pci_dn;
    let mut rc: i32;
    let mut buid: usize;
    let mut addr: usize;
    let ibm_map_pes = rtas_function_token(RTAS_FN_IBM_OPEN_SRIOV_MAP_PE_NUMBER);

    if ibm_map_pes == RTAS_UNKNOWN_SERVICE {
        return -EINVAL;
    }

    pdn = pci_get_pdn(pdev);
    addr = rtas_config_addr((*pdn).busno, (*pdn).devfn, 0);
    buid = (*(*pdn).phb).buid;
    spin_lock(&mut rtas_data_buf_lock);
    memcpy(rtas_data_buf, vf_pe_array, RTAS_DATA_BUF_SIZE);
    rc = rtas_call(
        ibm_map_pes, 5, 1, core::ptr::null_mut(), addr,
        BUID_HI(buid), BUID_LO(buid), rtas_data_buf,
        (num_vfs as usize) * core::mem::size_of::<pe_map_bar_entry>(),
    );
    memcpy(vf_pe_array, rtas_data_buf, RTAS_DATA_BUF_SIZE);
    spin_unlock(&mut rtas_data_buf_lock);

    if rc != 0 {
        dev_err(&mut (*pdev).dev,
            "%s: Failed to associate pes PE#%lx, rc=%x\n", __func__, addr, rc);
    }
    rc
}

#[cfg(feature = "CONFIG_PCI_IOV")]
unsafe fn pseries_set_pe_num(pdev: *mut pci_dev, vf_index: u16, pe_num: u16) {
    let pdn = pci_get_pdn(pdev);
    (*pdn).pe_num_map[vf_index as usize] = be16_to_cpu(pe_num);
    dev_dbg(&mut (*pdev).dev, "VF %04x:%02x:%02x.%x associated with PE#%x\n",
        pci_domain_nr((*pdev).bus), (*(*pdev).bus).number,
        PCI_SLOT(pci_iov_virtfn_devfn(pdev, vf_index)),
        PCI_FUNC(pci_iov_virtfn_devfn(pdev, vf_index)),
        (*pdn).pe_num_map[vf_index as usize]);
}

#[cfg(feature = "CONFIG_PCI_IOV")]
unsafe fn pseries_associate_pes(pdev: *mut pci_dev, num_vfs: u16) -> i32 {
    let pdn;
    let mut i: i32;
    let mut rc: i32;
    let mut vf_index: u16;
    let vf_pe_array = kzalloc(RTAS_DATA_BUF_SIZE, GFP_KERNEL) as *mut pe_map_bar_entry;
    let mut res: *mut resource;
    let mut size: u64;

    if vf_pe_array.is_null() { return -ENOMEM; }
    pdn = pci_get_pdn(pdev);
    // create firmware structure to associate pes
    vf_index = 0;
    while vf_index < num_vfs {
        (*pdn).pe_num_map[vf_index as usize] = IODA_INVALID_PE;
        i = 0;
        while i < PCI_SRIOV_NUM_BARS {
            res = &mut (*pdev).resource[(i + PCI_IOV_RESOURCES) as usize];
            if (*res).parent.is_null() { i += 1; continue; }
            size = pcibios_iov_resource_alignment(pdev, i + PCI_IOV_RESOURCES);
            (*vf_pe_array.add(vf_index as usize)).bar =
                cpu_to_be64((*res).start + size * vf_index as u64);
            (*vf_pe_array.add(vf_index as usize)).rid = cpu_to_be16(
                ((pci_iov_virtfn_bus(pdev, vf_index) << 8) |
                 pci_iov_virtfn_devfn(pdev, vf_index)) as u16);
            (*vf_pe_array.add(vf_index as usize)).pe_num = cpu_to_be16(IODA_INVALID_PE);
            i += 1;
        }
        vf_index += 1;
    }
    rc = pseries_send_map_pe(pdev, num_vfs, vf_pe_array);
    // Only zero is success
    if rc == 0 {
        vf_index = 0;
        while vf_index < num_vfs {
            pseries_set_pe_num(pdev, vf_index, (*vf_pe_array.add(vf_index as usize)).pe_num);
            vf_index += 1;
        }
    }
    kfree(vf_pe_array as *mut core::ffi::c_void);
    rc
}

#[cfg(feature = "CONFIG_PCI_IOV")]
unsafe fn pseries_pci_sriov_enable(pdev: *mut pci_dev, num_vfs: u16) -> i32 {
    let pdn;
    let mut rc: i32;
    let max_vfs: *const i32;
    let max_config_vfs: i32;
    let dn = pci_device_to_OF_node(pdev);
    max_vfs = of_get_property(dn, "ibm,number-of-configurable-vfs\0".as_ptr() as *const i8, core::ptr::null_mut());
    if max_vfs.is_null() { return -EINVAL; }
    // First integer stores max config
    max_config_vfs = of_read_number(max_vfs, 1) as i32;
    if max_config_vfs < num_vfs as i32 || num_vfs as usize > MAX_VFS_FOR_MAP_PE {
        dev_err(&mut (*pdev).dev, "Num VFs %x > %x Configurable VFs\n", num_vfs,
            if num_vfs as usize > MAX_VFS_FOR_MAP_PE { MAX_VFS_FOR_MAP_PE } else { max_config_vfs });
        return -EINVAL;
    }
    pdn = pci_get_pdn(pdev);
    (*pdn).pe_num_map = kmalloc_objs((*pdn).pe_num_map, num_vfs);
    if (*pdn).pe_num_map.is_null() { return -ENOMEM; }
    rc = pseries_associate_pes(pdev, num_vfs);
    // Anything other than zero is failure
    if rc != 0 {
        dev_err(&mut (*pdev).dev, "Failure to enable sriov: %x\n", rc);
        kfree((*pdn).pe_num_map as *mut core::ffi::c_void);
    } else {
        pci_vf_drivers_autoprobe(pdev, false);
    }
    rc
}

#[cfg(feature = "CONFIG_PCI_IOV")]
unsafe fn pseries_pcibios_sriov_enable(pdev: *mut pci_dev, num_vfs: u16) -> i32 {
    // Allocate PCI data
    add_sriov_vf_pdns(pdev);
    pseries_pci_sriov_enable(pdev, num_vfs)
}

#[cfg(feature = "CONFIG_PCI_IOV")]
unsafe fn pseries_pcibios_sriov_disable(pdev: *mut pci_dev) -> i32 {
    let pdn = pci_get_pdn(pdev);
    // Releasing pe_num_map
    kfree((*pdn).pe_num_map as *mut core::ffi::c_void);
    // Release PCI data
    remove_sriov_vf_pdns(pdev);
    pci_vf_drivers_autoprobe(pdev, true);
    0
}

unsafe fn pSeries_request_regions() {
    if isa_io_base == 0 { return; }
    request_region(0x20, 0x20, "pic1\0".as_ptr() as *const i8);
    request_region(0xa0, 0x20, "pic2\0".as_ptr() as *const i8);
    request_region(0x00, 0x20, "dma1\0".as_ptr() as *const i8);
    request_region(0x40, 0x20, "timer\0".as_ptr() as *const i8);
    request_region(0x80, 0x10, "dma page reg\0".as_ptr() as *const i8);
    request_region(0xc0, 0x20, "dma2\0".as_ptr() as *const i8);
}

pub unsafe fn pSeries_final_fixup() {
    pSeries_request_regions();
    eeh_show_enabled();
    #[cfg(feature = "CONFIG_PCI_IOV")]
    {
        ppc_md.pcibios_sriov_enable = Some(pseries_pcibios_sriov_enable);
        ppc_md.pcibios_sriov_disable = Some(pseries_pcibios_sriov_disable);
    }
}

/* Assume the winbond 82c105 is the IDE controller on a p610/p615/p630. */
unsafe fn fixup_winbond_82c105(dev: *mut pci_dev) {
    let mut r: *mut resource;
    let mut reg: u32 = 0;
    if !machine_is(pseries) { return; }
    printk("Using INTC for W82c105 IDE controller.\n\0".as_ptr() as *const i8);
    pci_read_config_dword(dev, 0x40, &mut reg);
    // Enable LEGIRQ to use INTC instead of ISA interrupts
    pci_write_config_dword(dev, 0x40, reg | (1 << 11));
    pci_dev_for_each_resource(dev, |resource: *mut resource| {
        r = resource;
        if (*dev).bus.number == 0 && (*dev).devfn == 0x81 && (*r).flags & IORESOURCE_IO != 0 {
            (*r).flags &= !IORESOURCE_IO;
        }
        if (*r).start == 0 && (*r).end != 0 { (*r).flags = 0; (*r).end = 0; }
    });
}

// DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_WINBOND, PCI_DEVICE_ID_WINBOND_82C105,
//                          fixup_winbond_82c105);

unsafe fn prop_to_pci_speed(prop: u32) -> pci_bus_speed {
    match prop {
        0x01 => PCIE_SPEED_2_5GT,
        0x02 => PCIE_SPEED_5_0GT,
        0x04 => PCIE_SPEED_8_0GT,
        0x08 => PCIE_SPEED_16_0GT,
        0x10 => PCIE_SPEED_32_0GT,
        _ => { pr_debug("Unexpected PCI link speed property value\n\0".as_ptr() as *const i8); PCI_SPEED_UNKNOWN }
    }
}

pub unsafe fn pseries_root_bridge_prepare(bridge: *mut pci_host_bridge) -> i32 {
    let mut dn: *mut device_node;
    let mut pdn: *mut device_node;
    let bus = (*bridge).bus;
    let mut pcie_link_speed_stats = [0u32; 2];
    let mut rc: i32;
    // Rely on the pcibios_free_controller_deferred() callback.
    pci_set_host_bridge_release(bridge, Some(pcibios_free_controller_deferred),
        pci_bus_to_host(bus) as *mut core::ffi::c_void);
    dn = pcibios_get_phb_of_node(bus);
    if dn.is_null() { return 0; }
    pdn = dn;
    while !pdn.is_null() {
        rc = of_property_read_u32_array(pdn, "ibm,pcie-link-speed-stats\0".as_ptr() as *const i8,
            pcie_link_speed_stats.as_mut_ptr(), 2);
        if rc == 0 { break; }
        pdn = of_get_next_parent(pdn);
    }
    of_node_put(pdn);
    if rc != 0 { pr_debug("no ibm,pcie-link-speed-stats property\n\0".as_ptr() as *const i8); return 0; }
    (*bus).max_bus_speed = prop_to_pci_speed(pcie_link_speed_stats[0]);
    (*bus).cur_bus_speed = prop_to_pci_speed(pcie_link_speed_stats[1]);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
