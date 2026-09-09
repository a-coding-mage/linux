// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Author: Alexander Shiyan <shc_work@mail.ru>, 2016
 */

// Declarations supplied by the Linux kernel headers are intentionally left as
// external dependencies of this translation.

const CLPS711X_VIRT_BASE: usize = 0xfeff_4000;
const CLPS711X_PHYS_BASE: usize = 0x8000_0000;
const SYSFLG1: usize = 0x0140;
const HALT: usize = 0x0800;
const UNIQID: usize = 0x2440;
const RANDID0: usize = 0x2700;
const RANDID1: usize = 0x2704;
const RANDID2: usize = 0x2708;
const RANDID3: usize = 0x270c;

// `struct map_desc`, `struct resource`, and the machine-description types are
// provided by the corresponding kernel headers.
extern "C" {
    fn iotable_init(io_desc: *const MapDesc, nr: usize);
    fn readl(addr: usize) -> u32;
    fn sysflg1_verid(value: u32) -> u32;
    fn add_device_randomness(buf: *const core::ffi::c_void, len: usize);
    fn platform_device_register_simple(
        name: *const core::ffi::c_char,
        id: isize,
        resource: *const Resource,
        num_resources: usize,
    );
    fn soft_restart(addr: usize) -> !;
    static mut system_rev: u32;
    static mut system_serial_low: u32;
}

#[repr(C)]
struct MapDesc {
    virtual_addr: usize,
    pfn: usize,
    length: usize,
    type_: usize,
}

#[repr(C)]
struct Resource {
    start: usize,
    end: usize,
    flags: usize,
}

// Values supplied by the kernel headers.
const MT_DEVICE: usize = 0;
const PLATFORM_DEVID_NONE: isize = -1;
const SZ_128: usize = 128;

static mut clps711x_io_desc: MapDesc = MapDesc {
    virtual_addr: CLPS711X_VIRT_BASE,
    pfn: CLPS711X_PHYS_BASE >> 12,
    length: 48 * 1024,
    type_: MT_DEVICE,
};

static clps711x_cpuidle_res: Resource = Resource {
    start: CLPS711X_PHYS_BASE + HALT,
    end: CLPS711X_PHYS_BASE + HALT + SZ_128 - 1,
    flags: 0x0000_0200, // IORESOURCE_MEM
};

unsafe fn clps711x_map_io() {
    iotable_init(&raw const clps711x_io_desc, 1);
}

unsafe fn clps711x_init() {
    let mut id = [0u32; 5];

    id[0] = readl(CLPS711X_VIRT_BASE + UNIQID);
    id[1] = readl(CLPS711X_VIRT_BASE + RANDID0);
    id[2] = readl(CLPS711X_VIRT_BASE + RANDID1);
    id[3] = readl(CLPS711X_VIRT_BASE + RANDID2);
    id[4] = readl(CLPS711X_VIRT_BASE + RANDID3);
    system_rev = sysflg1_verid(readl(CLPS711X_VIRT_BASE + SYSFLG1));

    add_device_randomness(id.as_ptr().cast(), core::mem::size_of_val(&id));

    system_serial_low = id[0];

    platform_device_register_simple(
        c"clps711x-cpuidle".as_ptr(),
        PLATFORM_DEVID_NONE,
        &raw const clps711x_cpuidle_res,
        1,
    );
}

#[repr(C)]
#[derive(Copy, Clone)]
enum RebootMode {
    Unknown,
}

unsafe fn clps711x_restart(_mode: RebootMode, _cmd: *const core::ffi::c_char) {
    soft_restart(0);
}

static clps711x_compat: [Option<&'static core::ffi::CStr>; 2] = [
    Some(c"cirrus,ep7209"),
    None,
];

// DT_MACHINE_START(CLPS711X_DT, "Cirrus Logic CLPS711X (Device Tree Support)")
//     .dt_compat = clps711x_compat,
//     .map_io = clps711x_map_io,
//     .init_late = clps711x_init,
//     .restart = clps711x_restart,
// MACHINE_END
// The registration macro and its containing machine-description type are
// supplied by the architecture headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
