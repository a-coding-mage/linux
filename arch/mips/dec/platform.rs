// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * DEC platform devices.
 *
 * Copyright (c) 2014 Maciej W. Rozycki
 */

// Linux kernel declarations supplied by the corresponding C headers.

#[repr(C)]
struct Resource {
    name: *const u8,
    start: usize,
    end: usize,
    flags: usize,
}

#[repr(C)]
struct CmosRtcBoardInfo {
    flags: usize,
    address_space: usize,
}

#[repr(C)]
struct PlatformDevice {
    name: *const u8,
    id: i32,
    dev_platform_data: *mut CmosRtcBoardInfo,
    resource: *mut Resource,
    num_resources: usize,
}

extern "C" {
    static mut dec_kn_slot_size: usize;
    static mut dec_kn_slot_base: usize;
    static mut dec_interrupt: [i32; 64];
    static mut mips_machtype: i32;

    fn platform_add_devices(devices: *mut *mut PlatformDevice, count: usize) -> i32;
}

const IORESOURCE_MEM: usize = 0x0000_0200;
const IORESOURCE_IRQ: usize = 0x0000_0400;
const CMOS_RTC_FLAGS_NOFREQ: usize = 1;
const PLATFORM_DEVID_NONE: i32 = -1;
const DEC_IRQ_DZ11: usize = 0;
const DEC_IRQ_SCC0: usize = 1;
const DEC_IRQ_SCC1: usize = 2;
const MACH_DS23100: i32 = 0;
const MACH_DS5100: i32 = 1;
const KN01_DZ11: usize = 0;
const KN02_DZ11: usize = 0;
const IOASIC_SCC0: usize = 0;
const IOASIC_SCC1: usize = 0;

extern "C" {
    fn RTC_PORT(port: usize) -> usize;
}

static mut dec_rtc_resources: [Resource; 1] = [Resource {
    name: b"rtc\0".as_ptr(),
    start: 0,
    end: 0,
    flags: IORESOURCE_MEM,
}];

static mut dec_rtc_info: CmosRtcBoardInfo = CmosRtcBoardInfo {
    flags: CMOS_RTC_FLAGS_NOFREQ,
    address_space: 64,
};

static mut dec_rtc_device: PlatformDevice = PlatformDevice {
    name: b"rtc_cmos\0".as_ptr(),
    id: PLATFORM_DEVID_NONE,
    dev_platform_data: unsafe { &raw mut dec_rtc_info },
    resource: unsafe { &raw mut dec_rtc_resources[0] },
    num_resources: 1,
};

static mut dec_rtc_devices: [*mut PlatformDevice; 1] = [unsafe { &raw mut dec_rtc_device }];

static mut dec_dz_resources: [Resource; 2] = [
    Resource { name: b"dz\0".as_ptr(), start: 0, end: 0, flags: IORESOURCE_MEM },
    Resource { name: b"dz\0".as_ptr(), start: 0, end: 0, flags: IORESOURCE_IRQ },
];

static mut dec_dz_device: PlatformDevice = PlatformDevice {
    name: b"dz\0".as_ptr(),
    id: PLATFORM_DEVID_NONE,
    dev_platform_data: core::ptr::null_mut(),
    resource: unsafe { &raw mut dec_dz_resources[0] },
    num_resources: 2,
};

static mut dec_dz_devices: [*mut PlatformDevice; 1] = [unsafe { &raw mut dec_dz_device }];

static mut dec_zs_resources: [[Resource; 2]; 2] = [
    [
        Resource { name: b"scc0\0".as_ptr(), start: 0, end: 0, flags: IORESOURCE_MEM },
        Resource { name: b"scc0\0".as_ptr(), start: 0, end: 0, flags: IORESOURCE_IRQ },
    ],
    [
        Resource { name: b"scc1\0".as_ptr(), start: 0, end: 0, flags: IORESOURCE_MEM },
        Resource { name: b"scc1\0".as_ptr(), start: 0, end: 0, flags: IORESOURCE_IRQ },
    ],
];

static mut dec_zs_device: [PlatformDevice; 2] = [
    PlatformDevice {
        name: b"zs\0".as_ptr(), id: 0, dev_platform_data: core::ptr::null_mut(),
        resource: unsafe { &raw mut dec_zs_resources[0][0] }, num_resources: 2,
    },
    PlatformDevice {
        name: b"zs\0".as_ptr(), id: 1, dev_platform_data: core::ptr::null_mut(),
        resource: unsafe { &raw mut dec_zs_resources[1][0] }, num_resources: 2,
    },
];

#[no_mangle]
pub unsafe extern "C" fn dec_add_devices() -> i32 {
    let mut dec_zs_devices: [*mut PlatformDevice; 2] = [core::ptr::null_mut(); 2];
    let (mut ret1, mut ret2, mut ret3): (i32, i32, i32);
    let (mut num_dz, mut num_zs): (usize, usize);
    let (mut irq, mut i): (i32, usize);

    dec_rtc_resources[0].start = RTC_PORT(0);
    dec_rtc_resources[0].end = RTC_PORT(0) + dec_kn_slot_size - 1;

    i = 0;
    irq = dec_interrupt[DEC_IRQ_DZ11];
    if irq >= 0 {
        let base = if mips_machtype == MACH_DS23100 || mips_machtype == MACH_DS5100 {
            dec_kn_slot_base + KN01_DZ11
        } else {
            dec_kn_slot_base + KN02_DZ11
        };
        dec_dz_device.resource.add(0).write(Resource { name: (*dec_dz_device.resource).name, start: base, end: base + dec_kn_slot_size - 1, flags: IORESOURCE_MEM });
        (*dec_dz_device.resource.add(1)).start = irq as usize;
        (*dec_dz_device.resource.add(1)).end = irq as usize;
        i += 1;
    }
    num_dz = i;

    i = 0;
    irq = dec_interrupt[DEC_IRQ_SCC0];
    if irq >= 0 {
        let base = dec_kn_slot_base + IOASIC_SCC0;
        (*dec_zs_device[i].resource).start = base;
        (*dec_zs_device[i].resource).end = base + dec_kn_slot_size - 1;
        (*dec_zs_device[i].resource.add(1)).start = irq as usize;
        (*dec_zs_device[i].resource.add(1)).end = irq as usize;
        dec_zs_devices[i] = &raw mut dec_zs_device[i];
        i += 1;
    }
    irq = dec_interrupt[DEC_IRQ_SCC1];
    if irq >= 0 {
        let base = dec_kn_slot_base + IOASIC_SCC1;
        (*dec_zs_device[i].resource).start = base;
        (*dec_zs_device[i].resource).end = base + dec_kn_slot_size - 1;
        (*dec_zs_device[i].resource.add(1)).start = irq as usize;
        (*dec_zs_device[i].resource.add(1)).end = irq as usize;
        dec_zs_devices[i] = &raw mut dec_zs_device[i];
        i += 1;
    }
    num_zs = i;

    ret1 = platform_add_devices(dec_rtc_devices.as_mut_ptr(), 1);
    ret2 = platform_add_devices(dec_dz_devices.as_mut_ptr(), num_dz);
    ret3 = platform_add_devices(dec_zs_devices.as_mut_ptr(), num_zs);
    if ret1 != 0 { ret1 } else if ret2 != 0 { ret2 } else { ret3 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
