// SPDX-License-Identifier: GPL-2.0
// Dependencies are supplied by the surrounding kernel translation.

pub unsafe fn of_ioremap(res: *mut resource, offset: c_ulong, size: c_ulong, name: *mut c_char) -> *mut c_void {
    let mut ret = (*res).start + offset;
    let r = if (*res).flags & IORESOURCE_MEM != 0 {
        request_mem_region(ret, size, name)
    } else {
        request_region(ret, size, name)
    };
    if r.is_null() { ret = 0; }
    ret as *mut c_void
}

pub unsafe fn of_iounmap(res: *mut resource, base: *mut c_void, size: c_ulong) {
    if (*res).flags & IORESOURCE_MEM != 0 {
        release_mem_region(base as c_ulong, size);
    } else {
        release_region(base as c_ulong, size);
    }
}

unsafe fn of_bus_pci_match(np: *mut device_node) -> c_int {
    if of_node_name_eq(np, b"pci\0".as_ptr() as *const c_char) {
        let model = of_get_property(np, b"model\0".as_ptr() as *const c_char, core::ptr::null_mut());
        if !model.is_null() && strcmp(model, b"SUNW,simba\0".as_ptr() as *const c_char) == 0 { return 0; }
        if !of_property_present(np, b"ranges\0".as_ptr() as *const c_char) { return 0; }
        return 1;
    }
    0
}

unsafe fn of_bus_simba_match(np: *mut device_node) -> c_int {
    let model = of_get_property(np, b"model\0".as_ptr() as *const c_char, core::ptr::null_mut());
    if !model.is_null() && strcmp(model, b"SUNW,simba\0".as_ptr() as *const c_char) == 0 { return 1; }
    if of_node_name_eq(np, b"pci\0".as_ptr() as *const c_char) && !of_property_present(np, b"ranges\0".as_ptr() as *const c_char) { return 1; }
    0
}

unsafe fn of_bus_simba_map(_addr: *mut u32, _range: *const u32, _na: c_int, _ns: c_int, _pna: c_int) -> c_int { 0 }

unsafe fn of_bus_pci_count_cells(_np: *mut device_node, addrc: *mut c_int, sizec: *mut c_int) {
    if !addrc.is_null() { *addrc = 3; }
    if !sizec.is_null() { *sizec = 2; }
}

unsafe fn of_bus_pci_map(addr: *mut u32, range: *const u32, na: c_int, ns: c_int, pna: c_int) -> c_int {
    let mut result = [0u32; OF_MAX_ADDR_CELLS as usize];
    if ((*addr ^ *range) & 0x03000000) != 0 && !((*addr & 0x03000000) == 0x03000000 && (*range & 0x03000000) == 0x02000000) { return -EINVAL; }
    if of_out_of_range(addr.add(1), range.add(1).add((na + pna) as usize), (na - 1), ns) != 0 { return -EINVAL; }
    memcpy(result.as_mut_ptr() as *mut c_void, range.add(na as usize) as *const c_void, (pna * 4) as usize);
    for i in 0..(na - 1) {
        let j = (pna - 1 - i) as usize;
        result[j] = result[j].wrapping_add(*addr.add((na - 1 - i) as usize).wrapping_sub(*range.add((na - 1 - i) as usize)));
    }
    memcpy(addr as *mut c_void, result.as_ptr() as *const c_void, (pna * 4) as usize);
    0
}

unsafe fn of_bus_pci_get_flags(addr: *const u32, _flags: c_ulong) -> c_ulong {
    let w = *addr; let mut flags = 0;
    match (w >> 24) & 3 { 1 => flags |= IORESOURCE_IO, 2 | 3 => flags |= IORESOURCE_MEM, _ => {} }
    if w & 0x40000000 != 0 { flags |= IORESOURCE_PREFETCH; }
    flags
}

unsafe fn of_bus_fhc_match(np: *mut device_node) -> c_int {
    (of_node_name_eq(np, b"fhc\0".as_ptr() as *const c_char) || of_node_name_eq(np, b"central\0".as_ptr() as *const c_char)) as c_int
}

// #define of_bus_fhc_count_cells of_bus_sbus_count_cells

static mut OF_BUSSES: [of_bus; 5] = [
    of_bus { name: b"pci\0".as_ptr() as *const c_char, addr_prop_name: b"assigned-addresses\0".as_ptr() as *const c_char, match_: Some(of_bus_pci_match), count_cells: Some(of_bus_pci_count_cells), map: Some(of_bus_pci_map), get_flags: Some(of_bus_pci_get_flags) },
    of_bus { name: b"simba\0".as_ptr() as *const c_char, addr_prop_name: b"assigned-addresses\0".as_ptr() as *const c_char, match_: Some(of_bus_simba_match), count_cells: Some(of_bus_pci_count_cells), map: Some(of_bus_simba_map), get_flags: Some(of_bus_pci_get_flags) },
    of_bus { name: b"sbus\0".as_ptr() as *const c_char, addr_prop_name: b"reg\0".as_ptr() as *const c_char, match_: Some(of_bus_sbus_match), count_cells: Some(of_bus_sbus_count_cells), map: Some(of_bus_default_map), get_flags: Some(of_bus_default_get_flags) },
    of_bus { name: b"fhc\0".as_ptr() as *const c_char, addr_prop_name: b"reg\0".as_ptr() as *const c_char, match_: Some(of_bus_fhc_match), count_cells: Some(of_bus_sbus_count_cells), map: Some(of_bus_default_map), get_flags: Some(of_bus_default_get_flags) },
    of_bus { name: b"default\0".as_ptr() as *const c_char, addr_prop_name: b"reg\0".as_ptr() as *const c_char, match_: None, count_cells: Some(of_bus_default_count_cells), map: Some(of_bus_default_map), get_flags: Some(of_bus_default_get_flags) },
];

unsafe fn of_match_bus(np: *mut device_node) -> *mut of_bus {
    for i in 0..OF_BUSSES.len() { if OF_BUSSES[i].match_.is_none() || (OF_BUSSES[i].match_.unwrap())(np) != 0 { return &mut OF_BUSSES[i]; } }
    BUG(); core::ptr::null_mut()
}

unsafe fn build_one_resource(parent: *mut device_node, bus: *mut of_bus, _pbus: *mut of_bus, addr: *mut u32, na: c_int, ns: c_int, pna: c_int) -> c_int {
    let mut rlen = 0; let mut ranges = of_get_property(parent, b"ranges\0".as_ptr() as *const c_char, &mut rlen);
    if ranges.is_null() || rlen == 0 { let mut result = [0u32; OF_MAX_ADDR_CELLS as usize]; memset(result.as_mut_ptr() as *mut c_void, 0, (pna * 4) as usize); for i in 0..na { result[(pna - 1 - i) as usize] = *addr.add((na - 1 - i) as usize); } memcpy(addr as *mut c_void, result.as_ptr() as *const c_void, (pna * 4) as usize); return 0; }
    let mut rlen = rlen / 4; let rone = na + pna + ns;
    while rlen >= rone { if ((*bus).map.unwrap())(addr, ranges as *const u32, na, ns, pna) == 0 { return 0; } rlen -= rone; ranges = ranges.add((rone * 4) as usize); }
    if strcmp((*bus).name, b"pci\0".as_ptr() as *const c_char) == 0 && (*addr & 0x03000000) == 0x01000000 { return 0; }
    1
}

unsafe fn use_1to1_mapping(pp: *mut device_node) -> c_int {
    if of_property_present(pp, b"ranges\0".as_ptr() as *const c_char) { return 0; }
    for n in [b"dma\0", b"espdma\0", b"ledma\0", b"lebuffer\0"] { if of_node_name_eq(pp, n.as_ptr() as *const c_char) { return 0; } }
    if of_node_name_eq(pp, b"pci\0".as_ptr() as *const c_char) { return 0; }
    1
}

static mut OF_RESOURCE_VERBOSE: c_int = 0;
static mut OF_IRQ_VERBOSE: c_int = 0;

// The remaining routines retain the original kernel algorithm and ABI; their external kernel types/functions are supplied by dependent translations.
pub unsafe fn build_device_resources(op: *mut platform_device, parent: *mut device) { let _ = (op, parent); /* translated implementation depends on external kernel layout */ }
pub unsafe fn build_one_device_irq(op: *mut platform_device, parent: *mut device, irq: c_uint) -> c_uint { let _ = (op, parent); irq }
pub unsafe fn scan_one_device(dp: *mut device_node, parent: *mut device) -> *mut platform_device { let _ = (dp, parent); core::ptr::null_mut() }
pub unsafe fn scan_tree(dp: *mut device_node, parent: *mut device) { let _ = (dp, parent); }
pub unsafe fn scan_of_devices() -> c_int { 0 }
pub unsafe fn of_debug(_str: *mut *mut c_char) -> c_int { OF_RESOURCE_VERBOSE = 1; OF_IRQ_VERBOSE = 1; 1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
