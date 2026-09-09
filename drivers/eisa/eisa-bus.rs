// SPDX-License-Identifier: GPL-2.0-only
/* EISA bus support functions for sysfs. */

// Linux kernel dependencies supplied by other translated units.

const EISA_MAX_FORCED_DEV: usize = 16;

#[repr(C)]
pub struct EisaDeviceInfo {
    pub id: EisaDeviceId,
    pub name: [c_char; EISA_DEVICE_INFO_NAME_SIZE],
}

static mut ENABLE_DEV: [c_int; EISA_MAX_FORCED_DEV] = [0; EISA_MAX_FORCED_DEV];
static mut ENABLE_DEV_COUNT: c_uint = 0;
static mut DISABLE_DEV: [c_int; EISA_MAX_FORCED_DEV] = [0; EISA_MAX_FORCED_DEV];
static mut DISABLE_DEV_COUNT: c_uint = 0;

#[inline]
unsafe fn slot_address(r: *mut EisaRootDevice, n: c_int) -> c_ulong {
    (*r).bus_base_addr.wrapping_add(0x1000u64.wrapping_mul(n as u64))
}

unsafe fn is_forced_dev(
    forced_tab: *const c_int,
    forced_count: c_int,
    root: *mut EisaRootDevice,
    edev: *mut EisaDevice,
) -> c_int {
    let mut i = 0;
    while i < forced_count {
        let x = (((*root).bus_nr as c_int) << 8) | (*edev).slot;
        if *forced_tab.add(i as usize) == x { return 1; }
        i += 1;
    }
    0
}

unsafe fn eisa_name_device(edev: *mut EisaDevice) {
    #[cfg(CONFIG_EISA_NAMES)]
    {
        let mut i = 0;
        while i < EISA_INFOS {
            if strcmp((*edev).id.sig.as_ptr(), EISA_TABLE[i].id.sig.as_ptr()) == 0 {
                strscpy((*edev).pretty_name.as_mut_ptr(), EISA_TABLE[i].name.as_ptr(), (*edev).pretty_name.len());
                return;
            }
            i += 1;
        }
        sprintf((*edev).pretty_name.as_mut_ptr(), b"EISA device %.7s\0".as_ptr() as *const c_char, (*edev).id.sig.as_ptr());
    }
}

unsafe fn decode_eisa_sig(addr: c_ulong) -> *mut c_char {
    static mut SIG_STR: [c_char; EISA_SIG_LEN] = [0; EISA_SIG_LEN];
    let mut sig = [0u8; 4];
    let mut i = 0;
    while i < 4 {
        #[cfg(CONFIG_EISA_VLB_PRIMING)]
        outb(0x80 + i as u8, addr);
        sig[i] = inb(addr + i as c_ulong);
        if i == 0 && (sig[0] & 0x80) != 0 { return core::ptr::null_mut(); }
        i += 1;
    }
    SIG_STR[0] = (((sig[0] >> 2) & 0x1f) + (b'A' - 1)) as c_char;
    SIG_STR[1] = ((((sig[0] & 3) << 3) | (sig[1] >> 5)) + (b'A' - 1)) as c_char;
    SIG_STR[2] = ((sig[1] & 0x1f) + (b'A' - 1)) as c_char;
    let rev = ((sig[2] as u16) << 8) | sig[3] as u16;
    sprintf(SIG_STR.as_mut_ptr().add(3), b"%04X\0".as_ptr() as *const c_char, rev as c_uint);
    SIG_STR.as_mut_ptr()
}

unsafe fn eisa_bus_match(dev: *mut Device, drv: *const DeviceDriver) -> c_int {
    let edev = to_eisa_device(dev);
    let edrv = to_eisa_driver(drv);
    let mut eids = (*edrv).id_table;
    if eids.is_null() { return 0; }
    while strlen((*eids).sig.as_ptr()) != 0 {
        if strcmp((*eids).sig.as_ptr(), (*edev).id.sig.as_ptr()) == 0 && ((*edev).state & EISA_CONFIG_ENABLED) != 0 {
            (*edev).id.driver_data = (*eids).driver_data;
            return 1;
        }
        eids = eids.add(1);
    }
    0
}

unsafe fn eisa_bus_uevent(dev: *const Device, env: *mut KobjUeventEnv) -> c_int {
    let edev = to_eisa_device(dev as *mut Device);
    add_uevent_var(env, b"MODALIAS=" EISA_DEVICE_MODALIAS_FMT.as_bytes(), (*edev).id.sig.as_ptr());
    0
}

pub static mut EISA_BUS_TYPE: BusType = BusType { name: b"eisa\0".as_ptr() as *const c_char, match_: Some(eisa_bus_match), uevent: Some(eisa_bus_uevent) };

pub unsafe fn eisa_driver_register(edrv: *mut EisaDriver) -> c_int {
    (*edrv).driver.bus = &raw mut EISA_BUS_TYPE;
    driver_register(&mut (*edrv).driver)
}

pub unsafe fn eisa_driver_unregister(edrv: *mut EisaDriver) { driver_unregister(&mut (*edrv).driver); }

unsafe fn signature_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut c_char) -> ssize_t {
    sprintf(buf, b"%s\n\0".as_ptr() as *const c_char, (*to_eisa_device(dev)).id.sig.as_ptr())
}
unsafe fn enabled_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut c_char) -> ssize_t {
    sprintf(buf, b"%d\n\0".as_ptr() as *const c_char, (*to_eisa_device(dev)).state & EISA_CONFIG_ENABLED)
}
unsafe fn modalias_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut c_char) -> ssize_t {
    sprintf(buf, b"" EISA_DEVICE_MODALIAS_FMT "\n\0".as_ptr() as *const c_char, (*to_eisa_device(dev)).id.sig.as_ptr())
}

unsafe fn eisa_init_device(root: *mut EisaRootDevice, edev: *mut EisaDevice, slot: c_int) -> c_int {
    let sig = decode_eisa_sig(slot_address(root, slot) + EISA_VENDOR_ID_OFFSET);
    if sig.is_null() { return -1; }
    memcpy((*edev).id.sig.as_mut_ptr(), sig, EISA_SIG_LEN);
    (*edev).slot = slot;
    (*edev).state = inb(slot_address(root, slot) + EISA_CONFIG_OFFSET) as c_uint & EISA_CONFIG_ENABLED;
    (*edev).base_addr = slot_address(root, slot);
    (*edev).dma_mask = (*root).dma_mask;
    eisa_name_device(edev);
    (*edev).dev.parent = (*root).dev;
    (*edev).dev.bus = &raw mut EISA_BUS_TYPE;
    (*edev).dev.dma_mask = &mut (*edev).dma_mask;
    (*edev).dev.coherent_dma_mask = (*edev).dma_mask;
    dev_set_name(&mut (*edev).dev, b"%02X:%02X\0".as_ptr() as *const c_char, (*root).bus_nr, slot);
    for i in 0..EISA_MAX_RESOURCES { (*edev).res[i].name = (*edev).id.sig.as_mut_ptr(); }
    if is_forced_dev(ENABLE_DEV.as_ptr(), ENABLE_DEV_COUNT as c_int, root, edev) != 0 { (*edev).state = EISA_CONFIG_ENABLED | EISA_CONFIG_FORCED; }
    if is_forced_dev(DISABLE_DEV.as_ptr(), DISABLE_DEV_COUNT as c_int, root, edev) != 0 { (*edev).state = EISA_CONFIG_FORCED; }
    0
}

// The remaining registration/probing routines retain the kernel call ordering and
// are declared with their external kernel data types supplied by other units.
unsafe fn eisa_register_device(edev: *mut EisaDevice) -> c_int {
    let mut rc = device_register(&mut (*edev).dev);
    if rc != 0 { put_device(&mut (*edev).dev); return rc; }
    rc = device_create_file(&mut (*edev).dev, &mut DEV_ATTR_SIGNATURE);
    if rc != 0 { device_unregister(&mut (*edev).dev); return rc; }
    rc = device_create_file(&mut (*edev).dev, &mut DEV_ATTR_ENABLED);
    if rc != 0 { device_remove_file(&mut (*edev).dev, &mut DEV_ATTR_SIGNATURE); device_unregister(&mut (*edev).dev); return rc; }
    rc = device_create_file(&mut (*edev).dev, &mut DEV_ATTR_MODALIAS);
    if rc != 0 { device_remove_file(&mut (*edev).dev, &mut DEV_ATTR_ENABLED); device_remove_file(&mut (*edev).dev, &mut DEV_ATTR_SIGNATURE); device_unregister(&mut (*edev).dev); }
    rc
}

unsafe fn eisa_request_resources(root: *mut EisaRootDevice, edev: *mut EisaDevice, slot: c_int) -> c_int {
    let mut i: c_int = 0;
    while i < EISA_MAX_RESOURCES as c_int {
        if slot == 0 && i > 0 { (*edev).res[i as usize].start = 0; (*edev).res[i as usize].end = 0; i += 1; continue; }
        (*edev).res[i as usize].name = core::ptr::null_mut();
        if slot != 0 {
            (*edev).res[i as usize].start = slot_address(root, slot) + (i as c_ulong * 0x400);
            (*edev).res[i as usize].end = (*edev).res[i as usize].start + 0xff;
            (*edev).res[i as usize].flags = IORESOURCE_IO;
        } else {
            (*edev).res[i as usize].start = slot_address(root, slot) + EISA_VENDOR_ID_OFFSET;
            (*edev).res[i as usize].end = (*edev).res[i as usize].start + 3;
            (*edev).res[i as usize].flags = IORESOURCE_IO | IORESOURCE_BUSY;
        }
        if request_resource((*root).res, &mut (*edev).res[i as usize]) != 0 {
            while i > 0 { i -= 1; release_resource(&mut (*edev).res[i as usize]); }
            return -1;
        }
        i += 1;
    }
    0
}

unsafe fn eisa_release_resources(edev: *mut EisaDevice) {
    for i in 0..EISA_MAX_RESOURCES { if (*edev).res[i].start != 0 || (*edev).res[i].end != 0 { release_resource(&mut (*edev).res[i]); } }
}

unsafe fn eisa_probe(root: *mut EisaRootDevice) -> c_int {
    let mut edev = kzalloc_eisa_device();
    if edev.is_null() { return -ENOMEM; }
    if eisa_request_resources(root, edev, 0) != 0 || eisa_init_device(root, edev, 0) != 0 {
        eisa_release_resources(edev); kfree(edev);
        if !(*root).force_probe { return -ENODEV; }
    } else if eisa_register_device(edev) != 0 { eisa_release_resources(edev); kfree(edev); }
    let mut c = 0;
    for slot in 1..=(*root).slots {
        edev = kzalloc_eisa_device();
        if edev.is_null() { continue; }
        if eisa_request_resources(root, edev, slot) != 0 || eisa_init_device(root, edev, slot) != 0 {
            eisa_release_resources(edev); kfree(edev); continue;
        }
        c += 1;
        if eisa_register_device(edev) != 0 { eisa_release_resources(edev); kfree(edev); }
    }
    dev_info((*root).dev, b"EISA: Detected %d cards\n\0".as_ptr() as *const c_char, c);
    0
}

static mut EISA_ROOT_RES: Resource = Resource { name: b"EISA root resource\0".as_ptr() as *const c_char, start: 0, end: 0xffff_ffff, flags: IORESOURCE_IO };
static mut EISA_BUS_COUNT: c_int = 0;

pub unsafe fn eisa_root_register(root: *mut EisaRootDevice) -> c_int {
    (*root).eisa_root_res.name = EISA_ROOT_RES.name;
    (*root).eisa_root_res.start = (*(*root).res).start;
    (*root).eisa_root_res.end = (*(*root).res).end;
    (*root).eisa_root_res.flags = IORESOURCE_BUSY;
    let err = request_resource(&mut EISA_ROOT_RES, &mut (*root).eisa_root_res);
    if err != 0 { return err; }
    (*root).bus_nr = EISA_BUS_COUNT;
    EISA_BUS_COUNT += 1;
    let err = eisa_probe(root);
    if err != 0 { release_resource(&mut (*root).eisa_root_res); }
    err
}

unsafe fn eisa_init() -> c_int { let r = bus_register(&mut EISA_BUS_TYPE); if r != 0 { return r; } printk(b"EISA bus registered\0".as_ptr() as *const c_char); 0 }

pub static mut EISA_BUS: c_int = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
