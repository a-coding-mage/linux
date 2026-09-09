// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies are supplied by other translation units.

static mut SYSTEM_ROM_RESOURCE: struct resource = struct resource {
    name: "System ROM",
    start: 0xf0000,
    end: 0xfffff,
    flags: IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM,
};

static mut EXTENSION_ROM_RESOURCE: struct resource = struct resource {
    name: "Extension ROM",
    start: 0xe0000,
    end: 0xeffff,
    flags: IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM,
};

static mut ADAPTER_ROM_RESOURCES: [struct resource; 6] = [
    struct resource { name: "Adapter ROM", start: 0xc8000, end: 0, flags: IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM },
    struct resource { name: "Adapter ROM", start: 0, end: 0, flags: IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM },
    struct resource { name: "Adapter ROM", start: 0, end: 0, flags: IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM },
    struct resource { name: "Adapter ROM", start: 0, end: 0, flags: IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM },
    struct resource { name: "Adapter ROM", start: 0, end: 0, flags: IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM },
    struct resource { name: "Adapter ROM", start: 0, end: 0, flags: IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM },
];

static mut VIDEO_ROM_RESOURCE: struct resource = struct resource {
    name: "Video ROM",
    start: 0xc0000,
    end: 0xc7fff,
    flags: IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM,
};

/* does this oprom support the given pci device, or any of the devices
 * that the driver supports?
 */
unsafe fn match_id(pdev: *mut struct pci_dev, vendor: u16, device: u16) -> bool {
    let drv: *mut struct pci_driver = to_pci_driver((*pdev).dev.driver);
    let mut id: *const struct pci_device_id;

    if (*pdev).vendor == vendor && (*pdev).device == device { return true; }

    id = if !drv.is_null() { (*drv).id_table } else { core::ptr::null() };
    while !id.is_null() && (*id).vendor != 0 {
        if (*id).vendor == vendor && (*id).device == device { break; }
        id = id.add(1);
    }
    !id.is_null() && (*id).vendor != 0
}

unsafe fn probe_list(pdev: *mut struct pci_dev, vendor: u16, mut rom_list: *const core::ffi::c_void) -> bool {
    let mut device: u16;
    loop {
        if get_kernel_nofault(&mut device, rom_list) != 0 { device = 0; }
        if device != 0 && match_id(pdev, vendor, device) { break; }
        rom_list = (rom_list as *const u8).add(2) as *const core::ffi::c_void;
        if device == 0 { break; }
    }
    device != 0
}

unsafe fn find_oprom(pdev: *mut struct pci_dev) -> *mut struct resource {
    let mut oprom: *mut struct resource = core::ptr::null_mut();
    let mut i = 0;
    while i < ADAPTER_ROM_RESOURCES.len() {
        let res = &mut ADAPTER_ROM_RESOURCES[i] as *mut struct resource;
        let (mut offset, mut vendor, mut device, mut list, mut rev): (u16, u16, u16, u16, u16);
        let rom: *const u8;
        if (*res).end == 0 { break; }
        rom = isa_bus_to_virt((*res).start) as *const u8;
        if get_kernel_nofault(&mut offset, rom.add(0x18)) != 0 { i += 1; continue; }
        if get_kernel_nofault(&mut vendor, rom.add(offset as usize + 0x4)) != 0 { i += 1; continue; }
        if get_kernel_nofault(&mut device, rom.add(offset as usize + 0x6)) != 0 { i += 1; continue; }
        if match_id(pdev, vendor, device) { oprom = res; break; }
        if get_kernel_nofault(&mut list, rom.add(offset as usize + 0x8)) == 0 &&
           get_kernel_nofault(&mut rev, rom.add(offset as usize + 0xc)) == 0 &&
           rev >= 3 && list != 0 && probe_list(pdev, vendor, rom.add(offset as usize + list as usize) as *const core::ffi::c_void) {
            oprom = res; break;
        }
        i += 1;
    }
    oprom
}

pub unsafe fn pci_map_biosrom(pdev: *mut struct pci_dev) -> *mut core::ffi::c_void {
    let oprom = find_oprom(pdev);
    if oprom.is_null() { return core::ptr::null_mut(); }
    ioremap((*oprom).start, resource_size(oprom))
}

pub unsafe fn pci_unmap_biosrom(image: *mut core::ffi::c_void) { iounmap(image); }

pub unsafe fn pci_biosrom_size(pdev: *mut struct pci_dev) -> usize {
    let oprom = find_oprom(pdev);
    if !oprom.is_null() { resource_size(oprom) } else { 0 }
}

const ROMSIGNATURE: u16 = 0xaa55;

unsafe fn romsignature(rom: *const u8) -> i32 {
    let mut sig: u16 = 0;
    (get_kernel_nofault(&mut sig, rom as *const u16) == 0 && sig == ROMSIGNATURE) as i32
}

unsafe fn romchecksum(mut rom: *const u8, mut length: usize) -> i32 {
    let mut sum: u8 = 0;
    let mut c: u8 = 0;
    while length != 0 && get_kernel_nofault(&mut c, rom) == 0 {
        sum = sum.wrapping_add(c); rom = rom.add(1); length -= 1;
    }
    (length == 0 && sum == 0) as i32
}

pub unsafe fn probe_roms() {
    let mut start: usize;
    let mut length: usize;
    let mut upper: usize;
    let mut rom: *const u8;
    let mut c: u8 = 0;
    let mut i: usize;

    upper = ADAPTER_ROM_RESOURCES[0].start;
    start = VIDEO_ROM_RESOURCE.start;
    while start < upper {
        rom = isa_bus_to_virt(start) as *const u8;
        if romsignature(rom) == 0 { start += 2048; continue; }
        VIDEO_ROM_RESOURCE.start = start;
        if get_kernel_nofault(&mut c, rom.add(2)) != 0 { start += 2048; continue; }
        length = (c as usize) * 512;
        if length != 0 && romchecksum(rom, length) != 0 { VIDEO_ROM_RESOURCE.end = start + length - 1; }
        request_resource(&mut iomem_resource, &mut VIDEO_ROM_RESOURCE);
        break;
    }
    start = (VIDEO_ROM_RESOURCE.end + 1 + 2047) & !2047usize;
    if start < upper { start = upper; }

    request_resource(&mut iomem_resource, &mut SYSTEM_ROM_RESOURCE);
    upper = SYSTEM_ROM_RESOURCE.start;
    rom = isa_bus_to_virt(EXTENSION_ROM_RESOURCE.start) as *const u8;
    if romsignature(rom) != 0 {
        length = resource_size(&EXTENSION_ROM_RESOURCE);
        if romchecksum(rom, length) != 0 {
            request_resource(&mut iomem_resource, &mut EXTENSION_ROM_RESOURCE);
            upper = EXTENSION_ROM_RESOURCE.start;
        }
    }

    i = 0;
    while i < ADAPTER_ROM_RESOURCES.len() && start < upper {
        rom = isa_bus_to_virt(start) as *const u8;
        if romsignature(rom) == 0 { start += 2048; continue; }
        if get_kernel_nofault(&mut c, rom.add(2)) != 0 { start += 2048; continue; }
        length = (c as usize) * 512;
        if length == 0 || start + length > upper || romchecksum(rom, length) == 0 { start += 2048; continue; }
        ADAPTER_ROM_RESOURCES[i].start = start;
        ADAPTER_ROM_RESOURCES[i].end = start + length - 1;
        request_resource(&mut iomem_resource, &mut ADAPTER_ROM_RESOURCES[i]);
        start = ADAPTER_ROM_RESOURCES[i].end & !2047usize;
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
