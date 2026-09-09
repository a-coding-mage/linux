// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 Advanced Micro Devices, Inc.
 */

/* External kernel and CXL declarations are supplied by the surrounding tree. */

/*
 * PRM Address Translation - CXL DPA to System Physical Address
 *
 * Reference:
 *
 * AMD Family 1Ah Models 00h–0Fh and Models 10h–1Fh
 * ACPI v6.5 Porting Guide, Publication # 58088
 */

static prm_cxl_dpa_spa_guid: guid_t = GUID_INIT!(
    0xee41b397, 0x25d4, 0x452c, 0xad, 0x54, 0x48, 0xc6, 0xe3,
    0x48, 0x0b, 0x94
);

#[repr(C, packed)]
struct prm_cxl_dpa_spa_data {
    dpa: u64,
    reserved: u8,
    devfn: u8,
    bus: u8,
    segment: u8,
    spa: *mut u64,
}

unsafe fn prm_cxl_dpa_spa(pci_dev: *mut pci_dev, dpa: u64) -> u64 {
    let mut spa: u64 = 0;
    let data = prm_cxl_dpa_spa_data {
        dpa,
        reserved: 0,
        devfn: (*pci_dev).devfn,
        bus: (*(*pci_dev).bus).number,
        segment: pci_domain_nr((*pci_dev).bus),
        spa: &mut spa,
    };

    let rc = acpi_call_prm_handler(prm_cxl_dpa_spa_guid, &data as *const _ as *mut _);
    if rc != 0 {
        pci_dbg!(pci_dev, "failed to get SPA for %#llx: %d\n", dpa, rc);
        return ULLONG_MAX;
    }

    pci_dbg!(pci_dev, "PRM address translation: DPA -> SPA: %#llx -> %#llx\n", dpa, spa);
    spa
}

unsafe fn cxl_prm_setup_root(cxl_root: *mut cxl_root, data: *mut core::ffi::c_void) -> i32 {
    let ctx = data as *mut cxl_region_context;
    let cxled = (*ctx).cxled;
    let cxld = &mut (*cxled).cxld;
    let cxlmd = cxled_to_memdev(cxled);
    let mut hpa_range = (*ctx).hpa_range;
    let mut pci_dev: *mut pci_dev;
    let mut spa_len: u64;
    let mut len: u64;
    let mut addr: u64;
    let mut base_spa: u64;
    let base: u64;
    let mut ways: i32;
    let mut gran: i32;

    if hpa_range.start != (*(*cxled).dpa_res).start { return 0; }
    if (*ctx).interleave_ways != 1 {
        dev_dbg!(&cxld.dev, "unexpected interleaving config: ways: %d granularity: %d\n", (*ctx).interleave_ways, (*ctx).interleave_granularity);
        return -ENXIO;
    }
    if cxlmd.is_null() || !dev_is_pci((*cxlmd).dev.parent) {
        dev_dbg!(&cxld.dev, "No endpoint found: %s, range %#llx-%#llx\n", dev_name((*cxld).dev.parent), hpa_range.start, hpa_range.end);
        return -ENXIO;
    }
    pci_dev = to_pci_dev((*cxlmd).dev.parent);

    base = hpa_range.start;
    hpa_range.start = prm_cxl_dpa_spa(pci_dev, hpa_range.start);
    hpa_range.end = prm_cxl_dpa_spa(pci_dev, hpa_range.end);
    base_spa = hpa_range.start;
    if hpa_range.start == ULLONG_MAX || hpa_range.end == ULLONG_MAX { return -ENXIO; }

    hpa_range.start = ALIGN_DOWN!(hpa_range.start, SZ_256M);
    hpa_range.end = ALIGN!(hpa_range.end, SZ_256M) - 1;
    len = range_len(&(*ctx).hpa_range);
    spa_len = range_len(&hpa_range);
    if len == 0 || spa_len == 0 || spa_len % len != 0 { return -ENXIO; }
    ways = (spa_len / len) as i32;
    gran = SZ_256;
    if ways > 1 {
        while gran <= SZ_16M {
            addr = prm_cxl_dpa_spa(pci_dev, base.wrapping_add(gran as u64));
            if addr != base_spa.wrapping_add(gran as u64) { break; }
            gran <<= 1;
        }
    }
    if gran > SZ_16M { return -ENXIO; }

    cxld.flags |= CXL_DECODER_F_LOCK;
    cxld.flags |= CXL_DECODER_F_NORMALIZED_ADDRESSING;
    (*ctx).hpa_range = hpa_range;
    (*ctx).interleave_ways = ways;
    (*ctx).interleave_granularity = gran;
    0
}

pub unsafe fn cxl_setup_prm_address_translation(cxl_root: *mut cxl_root) {
    let host = (*cxl_root).port.uport_dev;
    let mut spa: u64 = 0;
    let data = prm_cxl_dpa_spa_data { dpa: 0, reserved: 0, devfn: 0, bus: 0, segment: 0, spa: &mut spa };
    if !acpi_match_device((*(*host).driver).acpi_match_table, host) { return; }
    let rc = acpi_call_prm_handler(prm_cxl_dpa_spa_guid, &data as *const _ as *mut _);
    if rc == -EOPNOTSUPP || rc == -ENODEV { return; }
    (*cxl_root).ops.translation_setup_root = Some(cxl_prm_setup_root);
}

EXPORT_SYMBOL_NS_GPL!(cxl_setup_prm_address_translation, "CXL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
