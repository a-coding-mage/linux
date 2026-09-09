// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2025 AMD Corporation. All rights reserved. */

// Dependencies are supplied by the surrounding kernel translation.

pub unsafe fn cxl_dport_map_rch_aer(dport: *mut cxl_dport) {
    let aer_phys: resource_size_t;
    let host: *mut device;
    let aer_cap: u16;

    aer_cap = cxl_rcrb_to_aer((*dport).dport_dev, (*dport).rcrb.base);
    if aer_cap != 0 {
        host = (*dport).reg_map.host;
        aer_phys = (aer_cap as resource_size_t).wrapping_add((*dport).rcrb.base);
        (*dport).regs.dport_aer = devm_cxl_iomap_block(
            host,
            aer_phys,
            core::mem::size_of::<aer_capability_regs>(),
        );
    }
}

pub unsafe fn cxl_disable_rch_root_ints(dport: *mut cxl_dport) {
    let aer_base: *mut core::ffi::c_void = (*dport).regs.dport_aer;
    let aer_cmd_mask: u32;
    let mut aer_cmd: u32;

    if aer_base.is_null() {
        return;
    }

    /*
     * Disable RCH root port command interrupts.
     * CXL 3.0 12.2.1.1 - RCH Downstream Port-detected Errors
     *
     * This sequence may not be necessary. CXL spec states disabling
     * the root cmd register's interrupts is required. But, PCI spec
     * shows these are disabled by default on reset.
     */
    aer_cmd_mask = PCI_ERR_ROOT_CMD_COR_EN |
        PCI_ERR_ROOT_CMD_NONFATAL_EN |
        PCI_ERR_ROOT_CMD_FATAL_EN;
    aer_cmd = readl(aer_base.add(PCI_ERR_ROOT_COMMAND as usize));
    aer_cmd &= !aer_cmd_mask;
    writel(aer_cmd, aer_base.add(PCI_ERR_ROOT_COMMAND as usize));
}

/*
 * Copy the AER capability registers using 32 bit read accesses.
 * This is necessary because RCRB AER capability is MMIO mapped. Clear the
 * status after copying.
 *
 * @aer_base: base address of AER capability block in RCRB
 * @aer_regs: destination for copying AER capability
 */
unsafe fn cxl_rch_get_aer_info(
    aer_base: *mut core::ffi::c_void,
    aer_regs: *mut aer_capability_regs,
) -> bool {
    /*
     * Bound the copy to the physically-defined AER registers (header
     * through the 16-byte Header Log). struct aer_capability_regs is a
     * software layout whose embedded struct pcie_tlp_log is larger than
     * the on-wire AER capability; copying sizeof(*aer_regs) would
     * over-read the RCRB-mapped MMIO block.
     */
    let read_cnt: usize = ((PCI_ERR_HEADER_LOG + 16) as usize) / core::mem::size_of::<u32>();
    let aer_regs_buf = aer_regs as *mut u32;
    let mut n: usize;

    if aer_base.is_null() {
        return false;
    }

    /*
     * Zero the destination so the software-only tail fields
     * (e.g. header_log.header_len) are deterministic rather than
     * left as uninitialized stack, which could drive a bogus loop
     * length in pcie_print_tlp_log().
     */
    core::ptr::write_bytes(aer_regs as *mut u8, 0, core::mem::size_of::<aer_capability_regs>());

    /* Use readl() to guarantee 32-bit accesses */
    n = 0;
    while n < read_cnt {
        *aer_regs_buf.add(n) = readl(aer_base.add(n * core::mem::size_of::<u32>()));
        n += 1;
    }

    writel((*aer_regs).uncor_status, aer_base.add(PCI_ERR_UNCOR_STATUS as usize));
    writel((*aer_regs).cor_status, aer_base.add(PCI_ERR_COR_STATUS as usize));

    true
}

/* Get AER severity. Return false if there is no error. */
unsafe fn cxl_rch_get_aer_severity(
    aer_regs: *mut aer_capability_regs,
    severity: *mut i32,
) -> bool {
    let uncor_status = (*aer_regs).uncor_status & !(*aer_regs).uncor_mask;

    if uncor_status != 0 {
        *severity = if (uncor_status & (*aer_regs).uncor_severity) != 0 {
            AER_FATAL
        } else {
            AER_NONFATAL
        };
        return true;
    }

    if ((*aer_regs).cor_status & !(*aer_regs).cor_mask) != 0 {
        *severity = AER_CORRECTABLE;
        return true;
    }

    false
}

pub unsafe fn cxl_handle_rdport_errors(cxlds: *mut cxl_dev_state) {
    let pdev: *mut pci_dev = to_pci_dev((*cxlds).dev);
    let mut aer_regs: aer_capability_regs = core::mem::zeroed();
    let mut dport: *mut cxl_dport = core::ptr::null_mut();
    let mut severity: i32 = 0;

    let port: *mut cxl_port = cxl_pci_find_port(pdev, &mut dport);
    if port.is_null() {
        return;
    }

    if !cxl_rch_get_aer_info((*dport).regs.dport_aer, &mut aer_regs) {
        return;
    }

    if !cxl_rch_get_aer_severity(&mut aer_regs, &mut severity) {
        return;
    }

    pci_print_aer(pdev, severity, &mut aer_regs);
    if severity == AER_CORRECTABLE {
        cxl_handle_cor_ras(&mut (*(*cxlds).cxlmd).dev, (*dport).regs.ras);
    } else {
        cxl_handle_ras(&mut (*(*cxlds).cxlmd).dev, (*dport).regs.ras);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
