// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation.

unsafe fn of_bus_pci_match(np: *mut device_node) -> i32 {
    if of_node_is_type(np, b"pci\0".as_ptr() as *const i8) != 0
        || of_node_is_type(np, b"pciex\0".as_ptr() as *const i8) != 0
    {
        if of_property_present(np, b"ranges\0".as_ptr() as *const i8) == 0 {
            return 0;
        }
        return 1;
    }
    0
}

unsafe fn of_bus_pci_count_cells(_np: *mut device_node, addrc: *mut i32, sizec: *mut i32) {
    if !addrc.is_null() { *addrc = 3; }
    if !sizec.is_null() { *sizec = 2; }
}

unsafe fn of_bus_pci_map(addr: *mut u32, range: *const u32, na: i32, ns: i32, pna: i32) -> i32 {
    let mut result = [0u32; OF_MAX_ADDR_CELLS as usize];
    if ((*addr ^ *range) & 0x03000000) != 0 { return -EINVAL; }
    if of_out_of_range(addr.add(1), range.add(1), range.add((na + pna) as usize), na - 1, ns) != 0 {
        return -EINVAL;
    }
    core::ptr::copy_nonoverlapping(range.add(na as usize), result.as_mut_ptr(), pna as usize);
    for i in 0..(na - 1) {
        let d = *addr.add((na - 1 - i) as usize);
        let b = *range.add((na - 1 - i) as usize);
        result[(pna - 1 - i) as usize] = result[(pna - 1 - i) as usize].wrapping_add(d.wrapping_sub(b));
    }
    core::ptr::copy_nonoverlapping(result.as_ptr(), addr, pna as usize);
    0
}

unsafe fn of_bus_pci_get_flags(addr: *const u32, _flags: libc_ulong) -> libc_ulong {
    let w = *addr;
    let mut flags: libc_ulong = 0;
    match (w >> 24) & 0x03 {
        0x01 => flags |= IORESOURCE_IO,
        0x02 | 0x03 => flags |= IORESOURCE_MEM,
        _ => {}
    }
    if w & 0x40000000 != 0 { flags |= IORESOURCE_PREFETCH; }
    flags
}

unsafe fn of_bus_sbus_get_flags(_addr: *const u32, _flags: libc_ulong) -> libc_ulong { IORESOURCE_MEM }

unsafe fn of_bus_ambapp_match(np: *mut device_node) -> i32 {
    of_node_is_type(np, b"ambapp\0".as_ptr() as *const i8)
}

unsafe fn of_bus_ambapp_count_cells(_child: *mut device_node, addrc: *mut i32, sizec: *mut i32) {
    if !addrc.is_null() { *addrc = 1; }
    if !sizec.is_null() { *sizec = 1; }
}

unsafe fn of_bus_ambapp_map(addr: *mut u32, range: *const u32, na: i32, ns: i32, pna: i32) -> i32 {
    of_bus_default_map(addr, range, na, ns, pna)
}

unsafe fn of_bus_ambapp_get_flags(_addr: *const u32, _flags: libc_ulong) -> libc_ulong { IORESOURCE_MEM }

static mut OF_BUSSES: [of_bus; 4] = [
    of_bus { name: b"pci\0".as_ptr(), addr_prop_name: b"assigned-addresses\0".as_ptr(), match_: Some(of_bus_pci_match), count_cells: of_bus_pci_count_cells, map: of_bus_pci_map, get_flags: of_bus_pci_get_flags },
    of_bus { name: b"sbus\0".as_ptr(), addr_prop_name: b"reg\0".as_ptr(), match_: Some(of_bus_sbus_match), count_cells: of_bus_sbus_count_cells, map: of_bus_default_map, get_flags: of_bus_sbus_get_flags },
    of_bus { name: b"ambapp\0".as_ptr(), addr_prop_name: b"reg\0".as_ptr(), match_: Some(of_bus_ambapp_match), count_cells: of_bus_ambapp_count_cells, map: of_bus_ambapp_map, get_flags: of_bus_ambapp_get_flags },
    of_bus { name: b"default\0".as_ptr(), addr_prop_name: b"reg\0".as_ptr(), match_: None, count_cells: of_bus_default_count_cells, map: of_bus_default_map, get_flags: of_bus_default_get_flags },
];

unsafe fn of_match_bus(np: *mut device_node) -> *mut of_bus {
    for i in 0..OF_BUSSES.len() {
        let bus = &mut OF_BUSSES[i];
        if bus.match_.is_none() || (bus.match_.unwrap()(np) != 0) { return bus; }
    }
    BUG();
    core::ptr::null_mut()
}

unsafe fn build_one_resource(parent: *mut device_node, bus: *mut of_bus, _pbus: *mut of_bus, addr: *mut u32, na: i32, ns: i32, pna: i32) -> i32 {
    let mut rlen = 0u32;
    let ranges = of_get_property(parent, b"ranges\0".as_ptr() as *const i8, &mut rlen);
    if ranges.is_null() || rlen == 0 {
        let mut result = [0u32; OF_MAX_ADDR_CELLS as usize];
        core::ptr::write_bytes(result.as_mut_ptr(), 0, pna as usize);
        for i in 0..na { result[(pna - 1 - i) as usize] = *addr.add((na - 1 - i) as usize); }
        core::ptr::copy_nonoverlapping(result.as_ptr(), addr, pna as usize);
        return 0;
    }
    let mut ranges = ranges as *const u32;
    let mut remaining = (rlen / 4) as i32;
    let rone = na + pna + ns;
    while remaining >= rone {
        if ((*bus).map)(addr, ranges, na, ns, pna) == 0 { return 0; }
        remaining -= rone;
        ranges = ranges.add(rone as usize);
    }
    1
}

unsafe fn use_1to1_mapping(pp: *mut device_node) -> i32 {
    if of_property_present(pp, b"ranges\0".as_ptr() as *const i8) != 0 { return 0; }
    if of_node_name_eq(pp, b"dma\0".as_ptr() as *const i8) != 0 || of_node_name_eq(pp, b"espdma\0".as_ptr() as *const i8) != 0 || of_node_name_eq(pp, b"ledma\0".as_ptr() as *const i8) != 0 || of_node_name_eq(pp, b"lebuffer\0".as_ptr() as *const i8) != 0 { return 0; }
    1
}

static mut OF_RESOURCE_VERBOSE: i32 = 0;

unsafe fn build_device_resources(op: *mut platform_device, parent: *mut device) {
    if parent.is_null() { return; }
    let p_op = to_platform_device(parent);
    let bus = of_match_bus((*p_op).dev.of_node);
    let mut na = 0i32;
    let mut ns = 0i32;
    ((*bus).count_cells)((*op).dev.of_node, &mut na, &mut ns);
    let mut num_reg = 0u32;
    let preg = of_get_property((*op).dev.of_node, (*bus).addr_prop_name, &mut num_reg);
    if preg.is_null() || num_reg == 0 { return; }
    let num_reg = (num_reg / 4) as i32 / (na + ns);
    (*op).resource = (*op).archdata.resource;
    (*op).num_resources = num_reg;
    for index in 0..num_reg {
        let r = &mut *(*op).resource.add(index as usize);
        let reg = (preg as *const u32).add((index * ((na + ns) * 4)) as usize);
        let mut addr = [0u32; OF_MAX_ADDR_CELLS as usize];
        let mut dp = (*op).dev.of_node;
        let mut pp = (*p_op).dev.of_node;
        let mut size = of_read_addr(reg.add(na as usize), ns);
        core::ptr::copy_nonoverlapping(reg, addr.as_mut_ptr(), na as usize);
        let mut flags = ((*bus).get_flags)(reg, 0);
        let mut result = OF_BAD_ADDR;
        if use_1to1_mapping(pp) != 0 {
            result = of_read_addr(addr.as_ptr(), na);
        } else {
            let mut dna = na;
            let mut dns = ns;
            let mut dbus = bus;
            loop {
                dp = pp;
                pp = (*dp).parent;
                if pp.is_null() { result = of_read_addr(addr.as_ptr(), dna); break; }
                let pbus = of_match_bus(pp);
                let mut pna = 0i32; let mut pns = 0i32;
                ((*pbus).count_cells)(dp, &mut pna, &mut pns);
                if build_one_resource(dp, dbus, pbus, addr.as_mut_ptr(), dna, dns, pna) != 0 { break; }
                flags = ((*pbus).get_flags)(addr.as_ptr(), flags);
                dna = pna; dns = pns; dbus = pbus;
            }
        }
        core::ptr::write_bytes(r as *mut resource as *mut u8, 0, core::mem::size_of::<resource>());
        if OF_RESOURCE_VERBOSE != 0 { printk_resource(op, index, result); }
        if result != OF_BAD_ADDR {
            (*r).start = result & 0xffffffff;
            (*r).end = result.wrapping_add(size).wrapping_sub(1);
            (*r).flags = flags | ((result >> 32) & 0xff);
        }
        (*r).name = (*(*op).dev.of_node).full_name;
    }
}

unsafe fn scan_one_device(dp: *mut device_node, parent: *mut device) -> *mut platform_device {
    let op = kzalloc_platform_device();
    if op.is_null() { return core::ptr::null_mut(); }
    (*op).dev.of_node = dp;
    build_device_resources(op, parent);
    (*op).dev.parent = parent;
    if of_device_register(op) != 0 { put_device(&mut (*op).dev); kfree(op as *mut core::ffi::c_void); return core::ptr::null_mut(); }
    op
}

unsafe fn scan_tree(mut dp: *mut device_node, parent: *mut device) {
    while !dp.is_null() {
        let op = scan_one_device(dp, parent);
        if !op.is_null() { scan_tree((*dp).child, &mut (*op).dev); }
        dp = (*dp).sibling;
    }
}

unsafe fn scan_of_devices() -> i32 {
    let root = of_find_node_by_path(b"/\0".as_ptr() as *const i8);
    let parent = scan_one_device(root, core::ptr::null_mut());
    if parent.is_null() { return 0; }
    scan_tree((*root).child, &mut (*parent).dev);
    0
}

unsafe fn of_debug(mut str_: *mut i8) -> i32 {
    let mut val = 0;
    get_option(&mut str_, &mut val);
    if val & 1 != 0 { OF_RESOURCE_VERBOSE = 1; }
    1
}

extern "C" {
    fn of_node_is_type(np: *mut device_node, name: *const i8) -> i32;
    fn of_property_present(np: *mut device_node, name: *const i8) -> i32;
    fn of_out_of_range(addr: *const u32, range: *const u32, end: *const u32, na: i32, ns: i32) -> i32;
    fn of_bus_default_map(addr: *mut u32, range: *const u32, na: i32, ns: i32, pna: i32) -> i32;
    fn of_bus_default_count_cells(np: *mut device_node, addrc: *mut i32, sizec: *mut i32);
    fn of_bus_default_get_flags(addr: *const u32, flags: libc_ulong) -> libc_ulong;
    fn of_bus_sbus_match(np: *mut device_node) -> i32;
    fn of_bus_sbus_count_cells(np: *mut device_node, addrc: *mut i32, sizec: *mut i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
