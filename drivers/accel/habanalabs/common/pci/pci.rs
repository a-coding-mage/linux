// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2016-2019 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

// Dependencies supplied by the surrounding driver and kernel bindings.

const HL_PLDM_PCI_ELBI_TIMEOUT_MSEC: u64 = HL_PCI_ELBI_TIMEOUT_MSEC * 100;

const IATU_REGION_CTRL_REGION_EN_MASK: u32 = 1u32 << 31;
const IATU_REGION_CTRL_MATCH_MODE_MASK: u32 = 1u32 << 30;
const IATU_REGION_CTRL_NUM_MATCH_EN_MASK: u32 = 1u32 << 19;
const IATU_REGION_CTRL_BAR_NUM_MASK: u32 = 0x7u32 << 8;

/** Map PCI BARs. */
pub unsafe fn hl_pci_bars_map(hdev: *mut hl_device, name: [*const c_char; 3], is_wc: [bool; 3]) -> c_int {
    let pdev = (*hdev).pdev;
    let mut rc = pci_request_regions(pdev, HL_NAME);
    if rc != 0 {
        dev_err((*hdev).dev, "Cannot obtain PCI resources\n");
        return rc;
    }

    for i in 0..3 {
        let bar = i * 2;
        (*hdev).pcie_bar[bar] = if is_wc[i] {
            pci_ioremap_wc_bar(pdev, bar as c_int)
        } else {
            pci_ioremap_bar(pdev, bar as c_int)
        };
        if (*hdev).pcie_bar[bar].is_null() {
            dev_err((*hdev).dev, "pci_ioremap%s_bar failed for %s\n", if is_wc[i] { "_wc" } else { "" }, name[i]);
            rc = -ENODEV;
            break;
        }
    }
    if rc == 0 { return 0; }

    for i in (0..3).rev() {
        let bar = i * 2;
        if !(*hdev).pcie_bar[bar].is_null() { iounmap((*hdev).pcie_bar[bar]); }
    }
    pci_release_regions(pdev);
    rc
}

/** Unmap PCI BARs. */
unsafe fn hl_pci_bars_unmap(hdev: *mut hl_device) {
    let pdev = (*hdev).pdev;
    for i in (0..3).rev() {
        iounmap((*hdev).pcie_bar[i * 2]);
    }
    pci_release_regions(pdev);
}

pub unsafe fn hl_pci_elbi_read(hdev: *mut hl_device, addr: u64, data: *mut u32) -> c_int {
    let pdev = (*hdev).pdev;
    let msec = if (*hdev).pldm { HL_PLDM_PCI_ELBI_TIMEOUT_MSEC } else { HL_PCI_ELBI_TIMEOUT_MSEC };
    let mut val: u32 = 0;
    pci_write_config_dword(pdev, mmPCI_CONFIG_ELBI_STS, 0);
    pci_write_config_dword(pdev, mmPCI_CONFIG_ELBI_ADDR, addr as u32);
    pci_write_config_dword(pdev, mmPCI_CONFIG_ELBI_CTRL, 0);
    let timeout = ktime_add_ms(ktime_get(), msec);
    loop {
        pci_read_config_dword(pdev, mmPCI_CONFIG_ELBI_STS, &mut val);
        if val & PCI_CONFIG_ELBI_STS_MASK != 0 { break; }
        if ktime_compare(ktime_get(), timeout) > 0 {
            pci_read_config_dword(pdev, mmPCI_CONFIG_ELBI_STS, &mut val);
            break;
        }
        usleep_range(300, 500);
    }
    if val & PCI_CONFIG_ELBI_STS_MASK == PCI_CONFIG_ELBI_STS_DONE {
        pci_read_config_dword(pdev, mmPCI_CONFIG_ELBI_DATA, data);
        if trace_habanalabs_elbi_read_enabled() { trace_habanalabs_elbi_read(&(*hdev).pdev.dev, addr as u32, val); }
        return 0;
    }
    if val & PCI_CONFIG_ELBI_STS_ERR != 0 { dev_err((*hdev).dev, "Error reading from ELBI\n"); return -EIO; }
    if val & PCI_CONFIG_ELBI_STS_MASK == 0 { dev_err((*hdev).dev, "ELBI read didn't finish in time\n"); return -EIO; }
    dev_err((*hdev).dev, "ELBI read has undefined bits in status\n");
    -EIO
}

/** Write through the ELBI interface. */
unsafe fn hl_pci_elbi_write(hdev: *mut hl_device, addr: u64, data: u32) -> c_int {
    let pdev = (*hdev).pdev;
    let msec = if (*hdev).pldm { HL_PLDM_PCI_ELBI_TIMEOUT_MSEC } else { HL_PCI_ELBI_TIMEOUT_MSEC };
    let mut val: u32 = 0;
    pci_write_config_dword(pdev, mmPCI_CONFIG_ELBI_STS, 0);
    pci_write_config_dword(pdev, mmPCI_CONFIG_ELBI_ADDR, addr as u32);
    pci_write_config_dword(pdev, mmPCI_CONFIG_ELBI_DATA, data);
    pci_write_config_dword(pdev, mmPCI_CONFIG_ELBI_CTRL, PCI_CONFIG_ELBI_CTRL_WRITE);
    let timeout = ktime_add_ms(ktime_get(), msec);
    loop {
        pci_read_config_dword(pdev, mmPCI_CONFIG_ELBI_STS, &mut val);
        if val & PCI_CONFIG_ELBI_STS_MASK != 0 { break; }
        if ktime_compare(ktime_get(), timeout) > 0 { pci_read_config_dword(pdev, mmPCI_CONFIG_ELBI_STS, &mut val); break; }
        usleep_range(300, 500);
    }
    if val & PCI_CONFIG_ELBI_STS_MASK == PCI_CONFIG_ELBI_STS_DONE {
        if trace_habanalabs_elbi_write_enabled() { trace_habanalabs_elbi_write(&(*hdev).pdev.dev, addr as u32, val); }
        return 0;
    }
    if val & PCI_CONFIG_ELBI_STS_ERR != 0 { return -EIO; }
    if val & PCI_CONFIG_ELBI_STS_MASK == 0 { dev_err((*hdev).dev, "ELBI write didn't finish in time\n"); return -EIO; }
    dev_err((*hdev).dev, "ELBI write has undefined bits in status\n");
    -EIO
}

/** iATU write routine. */
pub unsafe fn hl_pci_iatu_write(hdev: *mut hl_device, addr: u32, data: u32) -> c_int {
    let prop = &(*hdev).asic_prop;
    let dbi_offset = addr & 0xFFF;
    hl_pci_elbi_write(hdev, prop.pcie_aux_dbi_reg_addr, 0x00300000);
    if hl_pci_elbi_write(hdev, prop.pcie_dbi_base_address + dbi_offset as u64, data) != 0 { return -EIO; }
    0
}

/** Configure the iATU inbound region. */
pub unsafe fn hl_pci_set_inbound_region(hdev: *mut hl_device, region: u8, pci_region: *mut hl_inbound_pci_region) -> c_int {
    let prop = &(*hdev).asic_prop;
    let offset = 0x200u32 * region as u32 + 0x100;
    let mut rc = 0;
    if (*pci_region).mode == PCI_ADDRESS_MATCH_MODE {
        let bar_phys_base = (*hdev).pcie_bar_phys[(*pci_region).bar as usize];
        let region_base = bar_phys_base + (*pci_region).offset_in_bar;
        let end = region_base + (*pci_region).size - 1;
        rc |= hl_pci_iatu_write(hdev, offset + 8, lower_32_bits(region_base));
        rc |= hl_pci_iatu_write(hdev, offset + 0xC, upper_32_bits(region_base));
        rc |= hl_pci_iatu_write(hdev, offset + 0x10, lower_32_bits(end));
    }
    rc |= hl_pci_iatu_write(hdev, offset + 0x14, lower_32_bits((*pci_region).addr));
    rc |= hl_pci_iatu_write(hdev, offset + 0x18, upper_32_bits((*pci_region).addr));
    rc |= hl_pci_iatu_write(hdev, offset, 0);
    let mut ctrl = FIELD_PREP(IATU_REGION_CTRL_REGION_EN_MASK, 1) | FIELD_PREP(IATU_REGION_CTRL_MATCH_MODE_MASK, (*pci_region).mode as u32) | FIELD_PREP(IATU_REGION_CTRL_NUM_MATCH_EN_MASK, 1);
    if (*pci_region).mode == PCI_BAR_MATCH_MODE { ctrl |= FIELD_PREP(IATU_REGION_CTRL_BAR_NUM_MASK, (*pci_region).bar as u32); }
    rc |= hl_pci_iatu_write(hdev, offset + 4, ctrl);
    hl_pci_elbi_write(hdev, prop.pcie_aux_dbi_reg_addr, 0);
    if rc != 0 { dev_err((*hdev).dev, "failed to map bar %u to 0x%08llx\n", (*pci_region).bar, (*pci_region).addr); }
    rc
}

/** Configure outbound region 0. */
pub unsafe fn hl_pci_set_outbound_region(hdev: *mut hl_device, pci_region: *mut hl_outbound_pci_region) -> c_int {
    let prop = &(*hdev).asic_prop;
    let end = (*pci_region).addr + (*pci_region).size - 1;
    let mut rc = 0;
    rc |= hl_pci_iatu_write(hdev, 8, lower_32_bits((*pci_region).addr));
    rc |= hl_pci_iatu_write(hdev, 0xC, upper_32_bits((*pci_region).addr));
    rc |= hl_pci_iatu_write(hdev, 0x10, lower_32_bits(end));
    rc |= hl_pci_iatu_write(hdev, 0x14, 0); rc |= hl_pci_iatu_write(hdev, 0x18, 0);
    rc |= hl_pci_iatu_write(hdev, 0x20, upper_32_bits(end));
    rc |= hl_pci_iatu_write(hdev, 0, 0x00002000); rc |= hl_pci_iatu_write(hdev, 4, 0x80000000);
    hl_pci_elbi_write(hdev, prop.pcie_aux_dbi_reg_addr, 0);
    rc
}

pub unsafe fn hl_get_pci_memory_region(hdev: *mut hl_device, addr: u64) -> pci_region {
    for i in 0..PCI_REGION_NUMBER {
        let region = &(*hdev).pci_mem_region[i as usize];
        if region.used && addr >= region.region_base && addr < region.region_base + region.region_size { return i as pci_region; }
    }
    PCI_REGION_NUMBER
}

/** PCI initialization code. */
pub unsafe fn hl_pci_init(hdev: *mut hl_device) -> c_int {
    let prop = &(*hdev).asic_prop;
    let pdev = (*hdev).pdev;
    let mut rc = pci_enable_device_mem(pdev);
    if rc != 0 { dev_err((*hdev).dev, "can't enable PCI device\n"); return rc; }
    pci_set_master(pdev);
    rc = ((*hdev).asic_funcs).pci_bars_map(hdev);
    if rc != 0 { dev_err((*hdev).dev, "Failed to map PCI BAR addresses\n"); pci_disable_device(pdev); return rc; }
    rc = ((*hdev).asic_funcs).init_iatu(hdev);
    if rc != 0 { dev_err((*hdev).dev, "PCI controller was not initialized successfully\n"); hl_pci_bars_unmap(hdev); pci_disable_device(pdev); return rc; }
    if (*hdev).asic_prop.iatu_done_by_fw { usleep_range(2000, 3000); }
    rc = dma_set_mask_and_coherent(&mut (*pdev).dev, DMA_BIT_MASK(prop.dma_mask));
    if rc != 0 { dev_err((*hdev).dev, "Failed to set dma mask to %d bits, error %d\n", prop.dma_mask, rc); hl_pci_bars_unmap(hdev); pci_disable_device(pdev); return rc; }
    dma_set_max_seg_size(&mut (*pdev).dev, U32_MAX);
    0
}

/** PCI finalization code. */
pub unsafe fn hl_pci_fini(hdev: *mut hl_device) {
    hl_pci_bars_unmap(hdev);
    pci_disable_device((*hdev).pdev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
