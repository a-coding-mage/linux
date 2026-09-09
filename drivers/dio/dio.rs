// SPDX-License-Identifier: GPL-2.0
/* Code to support devices on the DIO and DIO-II bus
 * Copyright (C) 05/1998 Peter Maydell <pmaydell@chiark.greenend.org.uk>
 * Copyright (C) 2004 Jochen Friedrich <jochen@scram.de>
 */

// The C source includes Linux kernel headers. Their types, constants,
// macros, and functions are supplied by the surrounding translation unit.

use crate::*;

pub static mut dio_bus: struct_dio_bus = struct_dio_bus {
    resources: [
        resource { name: b"DIO mem\0".as_ptr() as *const i8, start: 0x00600000, end: 0x007fffff },
        resource { name: b"DIO-II mem\0".as_ptr() as *const i8, start: 0x01000000, end: 0x1fffffff },
    ],
    name: b"DIO bus\0".as_ptr() as *const i8,
    ..unsafe { core::mem::zeroed() }
};

/* not a real config option yet! */

#[cfg(feature = "CONFIG_DIO_CONSTANTS")]
struct dioname {
    id: i32,
    name: *const i8,
}

#[cfg(feature = "CONFIG_DIO_CONSTANTS")]
static mut names: [dioname; 47] = [
    dioname { id: DIO_ID_DCA0, name: DIO_DESC_DCA0 }, dioname { id: DIO_ID_DCA0REM, name: DIO_DESC_DCA0REM },
    dioname { id: DIO_ID_DCA1, name: DIO_DESC_DCA1 }, dioname { id: DIO_ID_DCA1REM, name: DIO_DESC_DCA1REM },
    dioname { id: DIO_ID_DCM, name: DIO_DESC_DCM }, dioname { id: DIO_ID_DCMREM, name: DIO_DESC_DCMREM },
    dioname { id: DIO_ID_LAN, name: DIO_DESC_LAN }, dioname { id: DIO_ID_FHPIB, name: DIO_DESC_FHPIB },
    dioname { id: DIO_ID_NHPIB, name: DIO_DESC_NHPIB }, dioname { id: DIO_ID_SCSI0, name: DIO_DESC_SCSI0 },
    dioname { id: DIO_ID_SCSI1, name: DIO_DESC_SCSI1 }, dioname { id: DIO_ID_SCSI2, name: DIO_DESC_SCSI2 },
    dioname { id: DIO_ID_SCSI3, name: DIO_DESC_SCSI3 }, dioname { id: DIO_ID_FBUFFER, name: DIO_DESC_FBUFFER },
    dioname { id: DIO_ID_PARALLEL, name: DIO_DESC_PARALLEL }, dioname { id: DIO_ID_VME, name: DIO_DESC_VME },
    dioname { id: DIO_ID_DCL, name: DIO_DESC_DCL }, dioname { id: DIO_ID_DCLREM, name: DIO_DESC_DCLREM },
    dioname { id: DIO_ID_MISC0, name: DIO_DESC_MISC0 }, dioname { id: DIO_ID_MISC1, name: DIO_DESC_MISC1 },
    dioname { id: DIO_ID_MISC2, name: DIO_DESC_MISC2 }, dioname { id: DIO_ID_MISC3, name: DIO_DESC_MISC3 },
    dioname { id: DIO_ID_MISC4, name: DIO_DESC_MISC4 }, dioname { id: DIO_ID_MISC5, name: DIO_DESC_MISC5 },
    dioname { id: DIO_ID_MISC6, name: DIO_DESC_MISC6 }, dioname { id: DIO_ID_MISC7, name: DIO_DESC_MISC7 },
    dioname { id: DIO_ID_MISC8, name: DIO_DESC_MISC8 }, dioname { id: DIO_ID_MISC9, name: DIO_DESC_MISC9 },
    dioname { id: DIO_ID_MISC10, name: DIO_DESC_MISC10 }, dioname { id: DIO_ID_MISC11, name: DIO_DESC_MISC11 },
    dioname { id: DIO_ID_MISC12, name: DIO_DESC_MISC12 }, dioname { id: DIO_ID_MISC13, name: DIO_DESC_MISC13 },
    dioname { id: DIO_ENCODE_ID(DIO_ID_FBUFFER, DIO_ID2_GATORBOX), name: DIO_DESC2_GATORBOX },
    dioname { id: DIO_ENCODE_ID(DIO_ID_FBUFFER, DIO_ID2_TOPCAT), name: DIO_DESC2_TOPCAT },
    dioname { id: DIO_ENCODE_ID(DIO_ID_FBUFFER, DIO_ID2_RENAISSANCE), name: DIO_DESC2_RENAISSANCE },
    dioname { id: DIO_ENCODE_ID(DIO_ID_FBUFFER, DIO_ID2_LRCATSEYE), name: DIO_DESC2_LRCATSEYE },
    dioname { id: DIO_ENCODE_ID(DIO_ID_FBUFFER, DIO_ID2_HRCCATSEYE), name: DIO_DESC2_HRCCATSEYE },
    dioname { id: DIO_ENCODE_ID(DIO_ID_FBUFFER, DIO_ID2_HRMCATSEYE), name: DIO_DESC2_HRMCATSEYE },
    dioname { id: DIO_ENCODE_ID(DIO_ID_FBUFFER, DIO_ID2_DAVINCI), name: DIO_DESC2_DAVINCI },
    dioname { id: DIO_ENCODE_ID(DIO_ID_FBUFFER, DIO_ID2_XXXCATSEYE), name: DIO_DESC2_XXXCATSEYE },
    dioname { id: DIO_ENCODE_ID(DIO_ID_FBUFFER, DIO_ID2_HYPERION), name: DIO_DESC2_HYPERION },
    dioname { id: DIO_ENCODE_ID(DIO_ID_FBUFFER, DIO_ID2_XGENESIS), name: DIO_DESC2_XGENESIS },
    dioname { id: DIO_ENCODE_ID(DIO_ID_FBUFFER, DIO_ID2_TIGER), name: DIO_DESC2_TIGER },
    dioname { id: DIO_ENCODE_ID(DIO_ID_FBUFFER, DIO_ID2_YGENESIS), name: DIO_DESC2_YGENESIS },
];

#[cfg(feature = "CONFIG_DIO_CONSTANTS")]
static unknowndioname: &[u8] = b"unknown DIO board, please email linux-m68k@lists.linux-m68k.org\0";

#[cfg(feature = "CONFIG_DIO_CONSTANTS")]
unsafe fn dio_getname(id: i32) -> *const i8 {
    for name in names.iter() {
        if name.id == id { return name.name; }
    }
    unknowndioname.as_ptr() as *const i8
}

#[cfg(not(feature = "CONFIG_DIO_CONSTANTS"))]
static mut dio_no_name: [i8; 1] = [0];

#[cfg(not(feature = "CONFIG_DIO_CONSTANTS"))]
unsafe fn dio_getname(_id: i32) -> *const i8 { dio_no_name.as_ptr() }

unsafe fn dio_dev_release(dev: *mut device) {
    let ddev = container_of!(dev, dio_dev, dev);
    kfree(ddev as *mut core::ffi::c_void);
}

pub unsafe fn dio_find(deviceid: i32) -> i32 {
    let mut scode: i32;
    let mut id: i32;
    let mut prid: u8;
    let mut secid: u8;
    let mut i: u8 = 0;
    scode = 0;
    while scode < DIO_SCMAX {
        if DIO_SCINHOLE(scode) { scode += 1; continue; }
        let pa = dio_scodetophysaddr(scode);
        if pa == 0 { scode += 1; continue; }
        let va = if scode < DIOII_SCBASE { (pa + DIO_VIRADDRBASE) as *mut core::ffi::c_void } else { ioremap(pa, PAGE_SIZE) };
        if copy_from_kernel_nofault(&mut i as *mut u8 as *mut core::ffi::c_void, (va as *mut u8).add(DIO_IDOFF) as *const core::ffi::c_void, 1) != 0 {
            if scode >= DIOII_SCBASE { iounmap(va); }
            scode += 1; continue;
        }
        prid = DIO_ID(va);
        if DIO_NEEDSSECID(prid) { secid = DIO_SECID(va); id = DIO_ENCODE_ID(prid, secid); } else { id = prid as i32; }
        if id == deviceid { if scode >= DIOII_SCBASE { iounmap(va); } return scode; }
        scode += 1;
    }
    -1
}

/* This is the function that scans the DIO space and works out what
 * hardware is actually present.
 */
unsafe fn dio_init() -> i32 {
    if !MACH_IS_HP300 { return 0; }
    pr_info!("Scanning for DIO devices...\n");
    INIT_LIST_HEAD(&mut dio_bus.devices);
    dev_set_name(&mut dio_bus.dev, b"dio\0".as_ptr() as *const i8);
    let mut error = device_register(&mut dio_bus.dev);
    if error != 0 { pr_err!("DIO: Error registering dio_bus\n"); return error; }
    dio_bus.num_resources = if hp300_model == HP_320 { 1 } else { 2 };
    for i in 0..dio_bus.num_resources { request_resource(&mut iomem_resource, &mut dio_bus.resources[i as usize]); }
    for scode in 0..DIO_SCMAX {
        let mut prid: u8;
        let mut secid: u8 = 0;
        let mut i: i32 = 0;
        if DIO_SCINHOLE(scode) { continue; }
        let pa = dio_scodetophysaddr(scode);
        if pa == 0 { continue; }
        let va = if scode < DIOII_SCBASE { (pa + DIO_VIRADDRBASE) as *mut u8 } else { ioremap(pa, PAGE_SIZE) as *mut u8 };
        if copy_from_kernel_nofault(&mut i as *mut i32 as *mut core::ffi::c_void, va.add(DIO_IDOFF) as *const core::ffi::c_void, 1) != 0 {
            if scode >= DIOII_SCBASE { iounmap(va as *mut core::ffi::c_void); }
            continue;
        }
        let dev = kzalloc_obj!(dio_dev);
        if dev.is_null() { if scode >= DIOII_SCBASE { iounmap(va as *mut core::ffi::c_void); } return -ENOMEM; }
        (*dev).bus = &mut dio_bus;
        (*dev).dev.parent = &mut dio_bus.dev;
        (*dev).dev.bus = &dio_bus_type;
        (*dev).dev.release = Some(dio_dev_release);
        (*dev).scode = scode;
        (*dev).resource.start = pa;
        (*dev).resource.end = pa + DIO_SIZE(scode, va);
        dev_set_name(&mut (*dev).dev, b"%02x\0".as_ptr() as *const i8, scode);
        prid = DIO_ID(va as *mut core::ffi::c_void);
        if DIO_NEEDSSECID(prid) { secid = DIO_SECID(va as *mut core::ffi::c_void); (*dev).id = DIO_ENCODE_ID(prid, secid); } else { (*dev).id = prid as i32; }
        (*dev).ipl = DIO_IPL(va as *mut core::ffi::c_void);
        strscpy((*dev).name.as_mut_ptr(), dio_getname((*dev).id));
        if scode >= DIOII_SCBASE { iounmap(va as *mut core::ffi::c_void); }
        error = device_register(&mut (*dev).dev);
        if error != 0 { pr_err!("DIO: Error registering device %s\n", (*dev).name.as_ptr()); put_device(&mut (*dev).dev); continue; }
        error = dio_create_sysfs_dev_files(dev);
        if error != 0 { dev_err!(&mut (*dev).dev, "Error creating sysfs files\n"); }
    }
    0
}

subsys_initcall!(dio_init);

pub unsafe fn dio_scodetophysaddr(scode: i32) -> u64 {
    if scode >= DIOII_SCBASE { DIOII_BASE + (scode - 132) as u64 * DIOII_DEVSIZE }
    else if scode > DIO_SCMAX || scode < 0 || DIO_SCINHOLE(scode) { 0 }
    else { DIO_BASE + scode as u64 * DIO_DEVSIZE }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
