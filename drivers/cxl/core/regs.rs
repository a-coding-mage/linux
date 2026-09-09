// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2020 Intel Corporation. */
// CXL register probing and mapping implementation.

pub unsafe fn cxl_probe_component_regs(dev: *mut device, mut base: *mut core::ffi::c_void,
                                       map: *mut cxl_component_reg_map) {
    *map = core::mem::zeroed();
    base = base.add(CXL_CM_OFFSET as usize);
    let cap_array = readl(base.add(CXL_CM_CAP_HDR_OFFSET as usize));
    if field_get(CXL_CM_CAP_HDR_ID_MASK, cap_array) != CM_CAP_HDR_CAP_ID {
        dev_dbg(dev, "Couldn't locate the CXL.cache and CXL.mem capability array header.\n");
        return;
    }
    let cap_count = field_get(CXL_CM_CAP_HDR_ARRAY_SIZE_MASK, cap_array);
    for cap in 1..=cap_count {
        let mut hdr = readl(base.add((cap * 0x4) as usize));
        let cap_id = field_get(CXL_CM_CAP_HDR_ID_MASK, hdr) as u16;
        let offset = field_get(CXL_CM_CAP_PTR_MASK, hdr) as u16;
        let register_block = base.add(offset as usize);
        hdr = readl(register_block);
        let (rmap, length): (*mut cxl_reg_map, u32) = match cap_id {
            CXL_CM_CAP_CAP_ID_HDM => {
                dev_dbg(dev, "found HDM decoder capability (0x%x)\n", offset);
                (&mut (*map).hdm_decoder, 0x20 * cxl_hdm_decoder_count(hdr) + 0x10)
            }
            CXL_CM_CAP_CAP_ID_RAS => {
                dev_dbg(dev, "found RAS capability (0x%x)\n", offset);
                (&mut (*map).ras, CXL_RAS_CAPABILITY_LENGTH)
            }
            _ => { dev_dbg(dev, "Unknown CM cap ID: %d (0x%x)\n", cap_id, offset); (core::ptr::null_mut(), 0) }
        };
        if rmap.is_null() { continue; }
        (*rmap).valid = true;
        (*rmap).id = cap_id;
        (*rmap).offset = CXL_CM_OFFSET + offset as u32;
        (*rmap).size = length;
    }
}

pub unsafe fn cxl_probe_device_regs(dev: *mut device, base: *mut core::ffi::c_void,
                                    map: *mut cxl_device_reg_map) {
    *map = core::mem::zeroed();
    let cap_array = readq(base.add(CXLDEV_CAP_ARRAY_OFFSET as usize));
    if field_get(CXLDEV_CAP_ARRAY_ID_MASK, cap_array) != CXLDEV_CAP_ARRAY_CAP_ID { return; }
    let cap_count = field_get(CXLDEV_CAP_ARRAY_COUNT_MASK, cap_array);
    for cap in 1..=cap_count {
        let cap_id = field_get(CXLDEV_CAP_HDR_CAP_ID_MASK, readl(base.add((cap * 0x10) as usize))) as u16;
        let offset = readl(base.add((cap * 0x10 + 0x4) as usize));
        let length = readl(base.add((cap * 0x10 + 0x8) as usize));
        let rmap = match cap_id {
            CXLDEV_CAP_CAP_ID_DEVICE_STATUS => { dev_dbg(dev, "found Status capability (0x%x)\n", offset); &mut (*map).status }
            CXLDEV_CAP_CAP_ID_PRIMARY_MAILBOX => { dev_dbg(dev, "found Mailbox capability (0x%x)\n", offset); &mut (*map).mbox }
            CXLDEV_CAP_CAP_ID_SECONDARY_MAILBOX => { dev_dbg(dev, "found Secondary Mailbox capability (0x%x)\n", offset); core::ptr::null_mut() }
            CXLDEV_CAP_CAP_ID_MEMDEV => { dev_dbg(dev, "found Memory Device capability (0x%x)\n", offset); &mut (*map).memdev }
            _ => { if cap_id >= 0x8000 { dev_dbg(dev, "Vendor cap ID: %#x offset: %#x\n", cap_id, offset); } else { dev_dbg(dev, "Unknown cap ID: %#x offset: %#x\n", cap_id, offset); } core::ptr::null_mut() }
        };
        if rmap.is_null() { continue; }
        (*rmap).valid = true; (*rmap).id = cap_id; (*rmap).offset = offset; (*rmap).size = length;
    }
}

pub unsafe fn devm_cxl_iomap_block(dev: *mut device, addr: resource_size_t, length: resource_size_t) -> *mut core::ffi::c_void {
    if warn_on_once(addr == CXL_RESOURCE_NONE) { return core::ptr::null_mut(); }
    let res = devm_request_mem_region(dev, addr, length, dev_name(dev));
    if res.is_null() { let end = addr + length - 1; dev_err(dev, "Failed to request region %pa-%pa\n", &addr, &end); return core::ptr::null_mut(); }
    let ret = devm_ioremap(dev, addr, length);
    if ret.is_null() { dev_err(dev, "Failed to map region %pr\n", res); }
    ret
}

pub unsafe fn cxl_map_component_regs(map: *const cxl_register_map, regs: *mut cxl_component_regs, map_mask: c_ulong) -> c_int {
    let host = (*map).host;
    let info = [(&(*map).component_map.hdm_decoder, &mut (*regs).hdm_decoder), (&(*map).component_map.ras, &mut (*regs).ras)];
    for (rmap, addr) in info { if !rmap.valid || !test_bit(rmap.id, &map_mask) { continue; } *addr = devm_cxl_iomap_block(host, (*map).resource + rmap.offset as resource_size_t, rmap.size as resource_size_t); if (*addr).is_null() { return -ENOMEM; } }
    0
}

pub unsafe fn cxl_map_device_regs(map: *const cxl_register_map, regs: *mut cxl_device_regs) -> c_int {
    let info = [(&(*map).device_map.status, &mut (*regs).status), (&(*map).device_map.mbox, &mut (*regs).mbox), (&(*map).device_map.memdev, &mut (*regs).memdev)];
    for (rmap, addr) in info { if !rmap.valid { continue; } *addr = devm_cxl_iomap_block((*map).host, (*map).resource + rmap.offset as resource_size_t, rmap.size as resource_size_t); if (*addr).is_null() { return -ENOMEM; } }
    0
}

unsafe fn cxl_decode_regblock(pdev: *mut pci_dev, reg_lo: u32, reg_hi: u32, map: *mut cxl_register_map) -> bool {
    let reg_type = field_get(PCI_DVSEC_CXL_REG_LOCATOR_BLOCK_ID, reg_lo) as u8;
    let bar = field_get(PCI_DVSEC_CXL_REG_LOCATOR_BIR, reg_lo) as i32;
    let offset = ((reg_hi as u64) << 32) | (reg_lo & PCI_DVSEC_CXL_REG_LOCATOR_BLOCK_OFF_LOW) as u64;
    if offset > pci_resource_len(pdev, bar) { dev_warn(&mut (*pdev).dev, "BAR%d too small\n", bar); return false; }
    (*map).reg_type = reg_type; (*map).resource = pci_resource_start(pdev, bar) + offset; (*map).max_size = pci_resource_len(pdev, bar) - offset; true
}

unsafe fn __cxl_find_regblock_instance(pdev: *mut pci_dev, typ: cxl_regloc_type, map: *mut cxl_register_map, index: c_int) -> c_int {
    *map = cxl_register_map { host: &mut (*pdev).dev, resource: CXL_RESOURCE_NONE, ..core::mem::zeroed() };
    let mut regloc = pci_find_dvsec_capability(pdev, PCI_VENDOR_ID_CXL, PCI_DVSEC_CXL_REG_LOCATOR); if regloc == 0 { return -ENXIO; }
    let mut size = 0; pci_read_config_dword(pdev, regloc + PCI_DVSEC_HEADER1, &mut size); let blocks = (PCI_DVSEC_HEADER1_LEN(size) - PCI_DVSEC_CXL_REG_LOCATOR_BLOCK1) / 8; regloc += PCI_DVSEC_CXL_REG_LOCATOR_BLOCK1;
    let mut instance = 0; for _ in 0..blocks { let (mut lo, mut hi) = (0, 0); pci_read_config_dword(pdev, regloc, &mut lo); pci_read_config_dword(pdev, regloc + 4, &mut hi); regloc += 8; if !cxl_decode_regblock(pdev, lo, hi, map) { continue; } if (*map).reg_type == typ { if index == instance { return 0; } instance += 1; } }
    (*map).resource = CXL_RESOURCE_NONE; if index == CXL_INSTANCES_COUNT { instance } else { -ENODEV }
}

pub unsafe fn cxl_find_regblock_instance(p: *mut pci_dev, t: cxl_regloc_type, m: *mut cxl_register_map, i: c_uint) -> c_int { __cxl_find_regblock_instance(p,t,m,i as c_int) }
pub unsafe fn cxl_find_regblock(p: *mut pci_dev, t: cxl_regloc_type, m: *mut cxl_register_map) -> c_int { __cxl_find_regblock_instance(p,t,m,0) }
pub unsafe fn cxl_count_regblock(p: *mut pci_dev, t: cxl_regloc_type) -> c_int { let mut m = core::mem::zeroed(); __cxl_find_regblock_instance(p,t,&mut m,CXL_INSTANCES_COUNT) }

pub unsafe fn cxl_map_pmu_regs(map: *mut cxl_register_map, regs: *mut cxl_pmu_regs) -> c_int { (*regs).pmu = devm_cxl_iomap_block((*map).host, (*map).resource, CXL_PMU_REGMAP_SIZE); if (*regs).pmu.is_null() {-ENOMEM} else {0} }
pub unsafe fn cxl_setup_regs(map: *mut cxl_register_map) -> c_int { (*map).base = ioremap((*map).resource, (*map).max_size); if (*map).base.is_null() { return -ENOMEM; } let rc = cxl_probe_regs(map); iounmap((*map).base); (*map).base = core::ptr::null_mut(); rc }

unsafe fn cxl_probe_regs(map: *mut cxl_register_map) -> c_int {
    match (*map).reg_type {
        CXL_REGLOC_RBI_COMPONENT => { cxl_probe_component_regs((*map).host, (*map).base, &mut (*map).component_map); dev_dbg((*map).host, "Set up component registers\n"); }
        CXL_REGLOC_RBI_MEMDEV => { cxl_probe_device_regs((*map).host, (*map).base, &mut (*map).device_map); let d=&(*map).device_map; if !d.status.valid || !d.mbox.valid || !d.memdev.valid { dev_err((*map).host,"registers not found: %s%s%s\n",if !d.status.valid{"status "}else{""},if !d.mbox.valid{"mbox "}else{""},if !d.memdev.valid{"memdev "}else{""}); return -ENXIO; } dev_dbg((*map).host,"Probing device registers...\n"); }
        _ => {}
    } 0
}

unsafe fn cxl_rcrb_to_linkcap(dev: *mut device, dport: *mut cxl_dport) -> resource_size_t {
    let rcrb=(*dport).rcrb.base; if request_mem_region(rcrb,SZ_4K,b"CXL RCRB\0".as_ptr()).is_null(){return CXL_RESOURCE_NONE;} let addr=ioremap(rcrb,SZ_4K); if addr.is_null(){dev_err(dev,"Failed to map region %pr\n",addr);release_mem_region(rcrb,SZ_4K);return CXL_RESOURCE_NONE;} let mut off=field_get(PCI_RCRB_CAP_LIST_ID_MASK,readw(addr.add(PCI_CAPABILITY_LIST as usize))) as u16; let mut hdr=readl(addr.add(off as usize)); while field_get(PCI_RCRB_CAP_HDR_ID_MASK,hdr)!=PCI_CAP_ID_EXP {off=field_get(PCI_RCRB_CAP_HDR_NEXT_MASK,hdr) as u16;if off==0||off>SZ_4K as u16{off=0;break;}hdr=readl(addr.add(off as usize));} iounmap(addr);release_mem_region(rcrb,SZ_4K);if off==0{CXL_RESOURCE_NONE}else{off as resource_size_t}
}
pub unsafe fn cxl_dport_map_rcd_linkcap(pdev:*mut pci_dev,dport:*mut cxl_dport)->c_int{let pos=cxl_rcrb_to_linkcap(&mut (*pdev).dev,dport);if pos==CXL_RESOURCE_NONE{return -ENXIO;}(*dport).regs.rcd_pcie_cap=devm_cxl_iomap_block(&mut (*pdev).dev,(*dport).rcrb.base+pos,PCI_CAP_EXP_SIZEOF);0}

pub unsafe fn __rcrb_to_component(dev:*mut device,ri:*mut cxl_rcrb_info,which:cxl_rcrb)->resource_size_t{let mut r=(*ri).base;if which==CXL_RCRB_UPSTREAM{r+=SZ_4K;}if request_mem_region(r,SZ_4K,b"CXL RCRB\0".as_ptr()).is_null(){return CXL_RESOURCE_NONE;}let a=ioremap(r,SZ_4K);if a.is_null(){release_mem_region(r,SZ_4K);return CXL_RESOURCE_NONE;}let id=readl(a.add(PCI_VENDOR_ID as usize));let b0=readl(a.add(PCI_BASE_ADDRESS_0 as usize));let b1=readl(a.add(PCI_BASE_ADDRESS_1 as usize));iounmap(a);release_mem_region(r,SZ_4K);if id==u32::MAX||b0&(PCI_BASE_ADDRESS_MEM_TYPE_1M|PCI_BASE_ADDRESS_SPACE_IO)!=0{return CXL_RESOURCE_NONE;}let mut phys=(b0&PCI_BASE_ADDRESS_MEM_MASK) as u64;if b0&PCI_BASE_ADDRESS_MEM_TYPE_64!=0{phys|=(b1 as u64)<<32;}if phys==0||!is_aligned(phys,CXL_COMPONENT_REG_BLOCK_SIZE){return CXL_RESOURCE_NONE;}phys}

// The remaining helpers preserve the original RCRB capability-walk behavior.
pub unsafe fn cxl_rcrb_to_aer(dev: *mut device, rcrb: resource_size_t) -> u16 { if warn_on_once(rcrb == CXL_RESOURCE_NONE) || request_mem_region(rcrb,SZ_4K,dev_name(dev)).is_null() { return 0; } let addr=ioremap(rcrb,SZ_4K); if addr.is_null(){release_mem_region(rcrb,SZ_4K);return 0;} let mut off=0u16; let mut hdr=readl(addr); while pci_ext_cap_id(hdr)!=PCI_EXT_CAP_ID_ERR { off=pci_ext_cap_next(hdr); if off==0{break;} hdr=readl(addr.add(off as usize)); } iounmap(addr); release_mem_region(rcrb,SZ_4K); off }

pub unsafe fn cxl_rcd_component_reg_phys(dev: *mut device, dport: *mut cxl_dport) -> resource_size_t { if (*dport).rch.is_null(){CXL_RESOURCE_NONE}else{__rcrb_to_component(dev,&mut (*dport).rcrb,CXL_RCRB_UPSTREAM)} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
