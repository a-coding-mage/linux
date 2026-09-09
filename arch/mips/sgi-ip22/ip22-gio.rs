// SPDX-License-Identifier: GPL-2.0-only
// Translated from ip22-gio.c. Linux kernel dependencies are supplied externally.

static GIO_BUS_TYPE: BusType;

static mut GIO_NAME_TABLE: [GioName; 7] = [
    GioName { name: "SGI Impact", id: 0x10 },
    GioName { name: "Phobos G160", id: 0x35 },
    GioName { name: "Phobos G130", id: 0x36 },
    GioName { name: "Phobos G100", id: 0x37 },
    GioName { name: "Set Engineering GFE", id: 0x38 },
    // fake IDs
    GioName { name: "SGI Newport", id: 0x7e },
    GioName { name: "SGI GR2/GR3", id: 0x7f },
];

struct GioName { name: &'static str, id: u8 }
static mut GIO_BUS: *mut Device = core::ptr::null_mut();

// gio_match_device - Tell if an of_device structure has a matching gio_match structure.
unsafe fn gio_match_device(match_: *const GioDeviceId, dev: *const GioDevice) -> *const GioDeviceId {
    let mut ids = match_;
    while (*ids).id != 0xff {
        if (*ids).id == (*dev).id.id { return ids; }
        ids = ids.add(1);
    }
    core::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn gio_dev_get(dev: *mut GioDevice) -> *mut GioDevice {
    if dev.is_null() { return core::ptr::null_mut(); }
    let tmp = get_device(&mut (*dev).dev);
    if !tmp.is_null() { to_gio_device(tmp) } else { core::ptr::null_mut() }
}

#[no_mangle]
pub unsafe extern "C" fn gio_dev_put(dev: *mut GioDevice) {
    if !dev.is_null() { put_device(&mut (*dev).dev); }
}

unsafe fn gio_release_dev(dev: *mut Device) {
    let giodev = to_gio_device(dev);
    kfree(giodev as *mut core::ffi::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn gio_device_register(giodev: *mut GioDevice) -> i32 {
    (*giodev).dev.bus = &GIO_BUS_TYPE as *const _ as *mut _;
    (*giodev).dev.parent = GIO_BUS;
    (*giodev).dev.release = Some(gio_release_dev);
    device_register(&mut (*giodev).dev)
}

#[no_mangle]
pub unsafe extern "C" fn gio_device_unregister(giodev: *mut GioDevice) {
    device_unregister(&mut (*giodev).dev);
}

unsafe fn gio_bus_match(dev: *mut Device, drv: *const DeviceDriver) -> i32 {
    let gio_dev = to_gio_device(dev);
    let gio_drv = to_gio_driver(drv as *mut _);
    if !gio_match_device((*gio_drv).id_table, gio_dev).is_null() { 1 } else { 0 }
}

unsafe fn gio_device_probe(dev: *mut Device) -> i32 {
    let mut error = -ENODEV;
    let drv = to_gio_driver((*dev).driver);
    let gio_dev = to_gio_device(dev);
    if (*drv).probe.is_none() { return error; }
    let m = gio_match_device((*drv).id_table, gio_dev);
    if !m.is_null() { error = ((*drv).probe.unwrap())(gio_dev, m); }
    error
}

unsafe fn gio_device_remove(dev: *mut Device) {
    let gio_dev = to_gio_device(dev);
    let drv = to_gio_driver((*dev).driver);
    if let Some(remove) = (*drv).remove { remove(gio_dev); }
}

unsafe fn gio_device_shutdown(dev: *mut Device) {
    let gio_dev = to_gio_device(dev);
    let drv = to_gio_driver((*dev).driver);
    if !(*dev).driver.is_null() { if let Some(shutdown) = (*drv).shutdown { shutdown(gio_dev); } }
}

unsafe fn modalias_show(dev: *mut Device, _a: *mut DeviceAttribute, buf: *mut i8) -> isize {
    let gio_dev = to_gio_device(dev);
    sysfs_emit(buf, "gio:%x\n", (*gio_dev).id.id)
}
static DEVICE_ATTR_MODALIAS: DeviceAttribute = device_attr_ro!(modalias);

unsafe fn name_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut i8) -> isize {
    let giodev = to_gio_device(dev);
    sysfs_emit(buf, "%s\n", (*giodev).name)
}
static DEVICE_ATTR_NAME: DeviceAttribute = device_attr_ro!(name);

unsafe fn id_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut i8) -> isize {
    let giodev = to_gio_device(dev);
    sysfs_emit(buf, "%x\n", (*giodev).id.id)
}
static DEVICE_ATTR_ID: DeviceAttribute = device_attr_ro!(id);

static GIO_DEV_ATTRS: [*mut Attribute; 4] = [
    &DEVICE_ATTR_MODALIAS.attr as *const _ as *mut _,
    &DEVICE_ATTR_NAME.attr as *const _ as *mut _,
    &DEVICE_ATTR_ID.attr as *const _ as *mut _,
    core::ptr::null_mut(),
];

unsafe fn gio_device_uevent(dev: *const Device, env: *mut KobjUeventEnv) -> i32 {
    let gio_dev = to_gio_device(dev as *mut _);
    add_uevent_var(env, "MODALIAS=gio:%x", (*gio_dev).id.id);
    0
}

#[no_mangle]
pub unsafe extern "C" fn gio_register_driver(drv: *mut GioDriver) -> i32 {
    if (*drv).driver.name.is_null() { (*drv).driver.name = (*drv).name; }
    if (*drv).driver.owner.is_null() { (*drv).driver.owner = (*drv).owner; }
    (*drv).driver.bus = &GIO_BUS_TYPE as *const _ as *mut _;
    driver_register(&mut (*drv).driver)
}

#[no_mangle]
pub unsafe extern "C" fn gio_unregister_driver(drv: *mut GioDriver) { driver_unregister(&mut (*drv).driver); }

#[no_mangle]
pub unsafe extern "C" fn gio_set_master(dev: *mut GioDevice) {
    let mut tmp = (*sgimc).giopar;
    match (*dev).slotno { 0 => tmp |= SGIMC_GIOPAR_MASTERGFX, 1 => tmp |= SGIMC_GIOPAR_MASTEREXP0, 2 => tmp |= SGIMC_GIOPAR_MASTEREXP1, _ => {} }
    (*sgimc).giopar = tmp;
}

unsafe fn ip22_gio_set_64bit(slotno: i32) {
    let mut tmp = (*sgimc).giopar;
    match slotno { 0 => tmp |= SGIMC_GIOPAR_GFX64, 1 => tmp |= SGIMC_GIOPAR_EXP064, 2 => tmp |= SGIMC_GIOPAR_EXP164, _ => {} }
    (*sgimc).giopar = tmp;
}

unsafe fn ip22_gio_id(addr: u64, res: *mut u32) -> i32 {
    let mut tmp8 = 0u8; let mut tmp16 = 0u16; let mut tmp32 = 0u32;
    let ptr32 = CKSEG1ADDR(addr) as *mut u32;
    if get_dbe(&mut tmp32, ptr32) == 0 {
        let ptr8 = CKSEG1ADDR(addr + 3) as *mut u8;
        if get_dbe(&mut tmp8, ptr8) != 0 { *res = tmp32; return 1; }
        let ptr16 = CKSEG1ADDR(addr + 2) as *mut u16;
        get_dbe(&mut tmp16, ptr16);
        if tmp8 as u32 == (tmp16 & 0xff) as u32 && tmp8 as u32 == (tmp32 & 0xff) && tmp16 as u32 == (tmp32 & 0xffff) { *res = tmp32; return 1; }
    }
    0
}

const HQ2_MYSTERY_OFFS: u64 = 0x6A07C;
const NEWPORT_USTATUS_OFFS: u64 = 0xF133C;

unsafe fn ip22_is_gr2(addr: u64) -> i32 {
    let mut tmp = 0u32;
    if get_dbe(&mut tmp, CKSEG1ADDR(addr + HQ2_MYSTERY_OFFS) as *mut u32) == 0 && tmp == 0xdeadbeef { 1 } else { 0 }
}

unsafe fn ip22_check_gio(slotno: i32, addr: u64, irq: i32) {
    let mut name: &'static str = "Unknown"; let mut tmp = 0u32; let mut gio_dev: *mut GioDevice;
    let id: u8;
    if ip22_is_gr2(addr) != 0 { tmp = 0x7f; }
    else if ip22_gio_id(addr, &mut tmp) == 0 { if ip22_gio_id(addr + NEWPORT_USTATUS_OFFS, &mut tmp) != 0 { tmp = 0x7e; } else { tmp = 0; } }
    if tmp != 0 {
        id = GIO_ID(tmp);
        if tmp & GIO_32BIT_ID != 0 && tmp & GIO_64BIT_IFACE != 0 { ip22_gio_set_64bit(slotno); }
        for entry in GIO_NAME_TABLE.iter() { if id == entry.id { name = entry.name; break; } }
        printk(KERN_INFO, "GIO: slot %d : %s (id %x)\n", slotno, name, id);
        gio_dev = kzalloc_obj::<GioDevice>(); if gio_dev.is_null() { return; }
        (*gio_dev).name = name; (*gio_dev).slotno = slotno; (*gio_dev).id.id = id;
        (*gio_dev).resource.start = addr; (*gio_dev).resource.end = addr + 0x3fffff; (*gio_dev).resource.flags = IORESOURCE_MEM; (*gio_dev).irq = irq;
        dev_set_name(&mut (*gio_dev).dev, "%d", slotno);
        if gio_device_register(gio_dev) != 0 { gio_dev_put(gio_dev); }
    } else { printk(KERN_INFO, "GIO: slot %d : Empty\n", slotno); }
}

static GIO_BUS_TYPE: BusType = BusType { name: "gio", dev_groups: gio_dev_groups, match_: Some(gio_bus_match), probe: Some(gio_device_probe), remove: Some(gio_device_remove), shutdown: Some(gio_device_shutdown), uevent: Some(gio_device_uevent) };
static mut GIO_BUS_RESOURCE: Resource = Resource { start: GIO_SLOT_GFX_BASE, end: GIO_SLOT_GFX_BASE + 0x9fffff, name: "GIO Bus", flags: IORESOURCE_MEM };

unsafe fn ip22_gio_init() -> i32 {
    let mut pbdma = 0u32;
    GIO_BUS = root_device_register("gio"); if IS_ERR(GIO_BUS) { return PTR_ERR(GIO_BUS); }
    let ret = bus_register(&GIO_BUS_TYPE);
    if ret == 0 {
        request_resource(&mut iomem_resource, &mut GIO_BUS_RESOURCE);
        printk(KERN_INFO, "GIO: Probing bus...\n");
        if ip22_is_fullhouse() {
            ip22_check_gio(0, GIO_SLOT_GFX_BASE, SGI_GIO_1_IRQ);
            ip22_check_gio(1, GIO_SLOT_EXP0_BASE, SGI_GIO_1_IRQ);
        } else {
            if get_dbe(&mut pbdma, &mut (*hpc3c1).pbdma[1]) != 0 { ip22_check_gio(0, GIO_SLOT_GFX_BASE, SGI_GIO_0_IRQ); }
            ip22_check_gio(1, GIO_SLOT_EXP0_BASE, SGI_GIOEXP0_IRQ); ip22_check_gio(2, GIO_SLOT_EXP1_BASE, SGI_GIOEXP1_IRQ);
        }
    } else { root_device_unregister(GIO_BUS); }
    ret
}

// subsys_initcall(ip22_gio_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
