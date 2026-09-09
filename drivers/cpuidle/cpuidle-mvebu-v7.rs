/*
 * Marvell Armada 370, 38x and XP SoC cpuidle driver
 *
 * Copyright (C) 2014 Marvell
 *
 * Nadav Haklai <nadavh@marvell.com>
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 *
 * Maintainer: Gregory CLEMENT <gregory.clement@free-electrons.com>
 */

// Dependencies supplied by the surrounding kernel environment.
use core::ffi::c_void;

const MVEBU_V7_FLAG_DEEP_IDLE: u32 = 0x10000;

#[repr(C)]
pub struct CpuidleDevice;
#[repr(C)]
pub struct PlatformDevice {
    pub id_entry: *const PlatformDeviceId,
    pub dev: Device,
}
#[repr(C)]
pub struct Device {
    pub platform_data: *mut c_void,
}
#[repr(C)]
pub struct PlatformDeviceId {
    pub name: *const u8,
    pub driver_data: usize,
}
#[repr(C)]
pub struct CpuidleState {
    pub enter: Option<unsafe extern "C" fn(*mut CpuidleDevice, *mut CpuidleDriver, i32) -> i32>,
    pub exit_latency: u32,
    pub power_usage: u32,
    pub target_residency: u32,
    pub flags: u32,
    pub name: *const u8,
    pub desc: *const u8,
}
#[repr(C)]
pub struct CpuidleDriver {
    pub name: *const u8,
    pub states: [CpuidleState; 3],
    pub state_count: u32,
}
#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
    pub name: *const u8,
    pub suppress_bind_attrs: bool,
    pub id_table: *const PlatformDeviceId,
}

extern "C" {
    static ARM_CPUIDLE_WFI_STATE: CpuidleState;
    static CPUIDLE_FLAG_RCU_IDLE: u32;
    fn cpu_pm_enter();
    fn cpu_pm_exit();
    fn ct_cpuidle_enter();
    fn ct_cpuidle_exit();
    fn cpuidle_register(driver: *mut CpuidleDriver, device: *mut c_void) -> i32;
}

static mut mvebu_v7_cpu_suspend: Option<unsafe extern "C" fn(i32) -> i32> = None;

unsafe extern "C" fn mvebu_v7_enter_idle(
    _dev: *mut CpuidleDevice,
    drv: *mut CpuidleDriver,
    index: i32,
) -> i32 {
    let mut ret: i32;
    let mut deepidle = false;
    cpu_pm_enter();

    if (*drv).states[index as usize].flags & MVEBU_V7_FLAG_DEEP_IDLE != 0 {
        deepidle = true;
    }

    ct_cpuidle_enter();
    ret = (mvebu_v7_cpu_suspend.unwrap())(deepidle as i32);
    ct_cpuidle_exit();

    cpu_pm_exit();

    if ret != 0 {
        return ret;
    }

    index
}

static mut armadaxp_idle_driver: CpuidleDriver = CpuidleDriver {
    name: b"armada_xp_idle\0".as_ptr(),
    states: [
        unsafe { core::mem::zeroed() },
        CpuidleState { enter: Some(mvebu_v7_enter_idle), exit_latency: 100, power_usage: 50, target_residency: 1000, flags: 0, name: b"MV CPU IDLE\0".as_ptr(), desc: b"CPU power down\0".as_ptr() },
        CpuidleState { enter: Some(mvebu_v7_enter_idle), exit_latency: 1000, power_usage: 5, target_residency: 10000, flags: MVEBU_V7_FLAG_DEEP_IDLE, name: b"MV CPU DEEP IDLE\0".as_ptr(), desc: b"CPU and L2 Fabric power down\0".as_ptr() },
    ],
    state_count: 3,
};

static mut armada370_idle_driver: CpuidleDriver = CpuidleDriver {
    name: b"armada_370_idle\0".as_ptr(),
    states: [unsafe { core::mem::zeroed() }, CpuidleState { enter: Some(mvebu_v7_enter_idle), exit_latency: 100, power_usage: 5, target_residency: 1000, flags: MVEBU_V7_FLAG_DEEP_IDLE, name: b"Deep Idle\0".as_ptr(), desc: b"CPU and L2 Fabric power down\0".as_ptr() }, unsafe { core::mem::zeroed() }],
    state_count: 2,
};

static mut armada38x_idle_driver: CpuidleDriver = CpuidleDriver {
    name: b"armada_38x_idle\0".as_ptr(),
    states: [unsafe { core::mem::zeroed() }, CpuidleState { enter: Some(mvebu_v7_enter_idle), exit_latency: 10, power_usage: 5, target_residency: 100, flags: 0, name: b"Idle\0".as_ptr(), desc: b"CPU and SCU power down\0".as_ptr() }, unsafe { core::mem::zeroed() }],
    state_count: 2,
};

unsafe extern "C" fn mvebu_v7_cpuidle_probe(pdev: *mut PlatformDevice) -> i32 {
    let id = (*pdev).id_entry;
    if id.is_null() {
        return -22; // -EINVAL
    }

    mvebu_v7_cpu_suspend = core::mem::transmute((*pdev).dev.platform_data);
    cpuidle_register((*id).driver_data as *mut CpuidleDriver, core::ptr::null_mut())
}

static mvebu_cpuidle_ids: [PlatformDeviceId; 4] = [
    PlatformDeviceId { name: b"cpuidle-armada-xp\0".as_ptr(), driver_data: unsafe { &armadaxp_idle_driver as *const _ as usize } },
    PlatformDeviceId { name: b"cpuidle-armada-370\0".as_ptr(), driver_data: unsafe { &armada370_idle_driver as *const _ as usize } },
    PlatformDeviceId { name: b"cpuidle-armada-38x\0".as_ptr(), driver_data: unsafe { &armada38x_idle_driver as *const _ as usize } },
    PlatformDeviceId { name: core::ptr::null(), driver_data: 0 },
];

static mut mvebu_cpuidle_driver: PlatformDriver = PlatformDriver {
    probe: Some(mvebu_v7_cpuidle_probe),
    name: b"cpuidle-mbevu\0".as_ptr(),
    suppress_bind_attrs: true,
    id_table: mvebu_cpuidle_ids.as_ptr(),
};

// Equivalent of builtin_platform_driver(mvebu_cpuidle_driver).

// MODULE_AUTHOR("Gregory CLEMENT <gregory.clement@free-electrons.com>");
// MODULE_DESCRIPTION("Marvell EBU v7 cpuidle driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
