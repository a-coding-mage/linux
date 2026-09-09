// SPDX-License-Identifier: GPL-2.0-only
/*
 * Keystone2 based boards and SOC related code.
 *
 * Copyright 2013 Texas Instruments, Inc.
 *	Cyril Chemparathy <cyril@ti.com>
 *	Santosh Shilimkar <santosh.shillimkar@ti.com>
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const KEYSTONE_LOW_PHYS_START: u64 = 0x8000_0000;
const KEYSTONE_LOW_PHYS_SIZE: u64 = 0x8000_0000; // 2G
const KEYSTONE_LOW_PHYS_END: u64 =
    KEYSTONE_LOW_PHYS_START + KEYSTONE_LOW_PHYS_SIZE - 1;

const KEYSTONE_HIGH_PHYS_START: u64 = 0x8_0000_0000;
const KEYSTONE_HIGH_PHYS_SIZE: u64 = 0x4_0000_0000; // 16G
const KEYSTONE_HIGH_PHYS_END: u64 =
    KEYSTONE_HIGH_PHYS_START + KEYSTONE_HIGH_PHYS_SIZE - 1;

#[repr(C)]
pub struct DevPmDomain {
    pub ops: *const c_void,
}

#[repr(C)]
pub struct PmClkNotifierBlock {
    pub pm_domain: *mut DevPmDomain,
    pub con_ids: [*const c_char; 1],
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct NotifierBlock {
    pub notifier_call: Option<unsafe extern "C" fn(*mut NotifierBlock, c_ulong, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct Device {
    pub of_node: *mut DeviceNode,
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct BusType {
    _private: [u8; 0],
}

extern "C" {
    static mut platform_bus_type: BusType;
    static mut arch_phys_to_idmap_offset: c_long;

    fn of_find_matching_node(from: *mut DeviceNode, matches: *const OfDeviceId) -> *mut DeviceNode;
    fn pm_clk_add_notifier(bus: *mut BusType, block: *mut PmClkNotifierBlock) -> c_int;
    fn dma_direct_set_offset(dev: *mut Device, phys: u64, dma: u64, size: u64) -> c_int;
    fn bus_register_notifier(bus: *mut BusType, nb: *mut NotifierBlock) -> c_int;
    fn memblock_start_of_DRAM() -> u64;
    fn memblock_end_of_DRAM() -> u64;
    fn pr_crit(fmt: *const c_char, ...);
    fn dev_err(dev: *mut Device, fmt: *const c_char, ...);
}

static mut keystone_pm_domain: DevPmDomain = DevPmDomain {
    // USE_PM_CLK_RUNTIME_OPS and USE_PLATFORM_PM_SLEEP_OPS
    ops: core::ptr::null(),
};

static mut platform_domain_notifier: PmClkNotifierBlock = PmClkNotifierBlock {
    pm_domain: unsafe { &mut keystone_pm_domain },
    con_ids: [core::ptr::null()],
};

static of_keystone_table: [OfDeviceId; 4] = [
    OfDeviceId { compatible: b"ti,k2hk\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ti,k2e\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ti,k2l\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: core::ptr::null() },
];

unsafe extern "C" fn keystone_pm_runtime_init() -> c_int {
    let np = of_find_matching_node(core::ptr::null_mut(), of_keystone_table.as_ptr());
    if np.is_null() {
        return 0;
    }

    pm_clk_add_notifier(&mut platform_bus_type, &mut platform_domain_notifier);
    0
}

#[cfg(feature = "CONFIG_ARM_LPAE")]
const BUS_NOTIFY_ADD_DEVICE: c_ulong = 0x0001;

#[cfg(feature = "CONFIG_ARM_LPAE")]
unsafe extern "C" fn keystone_platform_notifier(
    _nb: *mut NotifierBlock,
    event: c_ulong,
    data: *mut c_void,
) -> c_int {
    const NOTIFY_DONE: c_int = 0x0000;
    const NOTIFY_BAD: c_int = 0x8002;
    const NOTIFY_OK: c_int = 0x0001;

    if event != BUS_NOTIFY_ADD_DEVICE {
        return NOTIFY_DONE;
    }

    let dev = data as *mut Device;
    if dev.is_null() {
        return NOTIFY_BAD;
    }

    if (*dev).of_node.is_null() {
        let ret = dma_direct_set_offset(
            dev,
            KEYSTONE_HIGH_PHYS_START,
            KEYSTONE_LOW_PHYS_START,
            KEYSTONE_HIGH_PHYS_SIZE,
        );
        // dev_err(dev, "set dma_offset%08llx%s\n", ...)
        let _ = ret;
    }
    NOTIFY_OK
}

#[cfg(feature = "CONFIG_ARM_LPAE")]
static mut platform_nb: NotifierBlock = NotifierBlock {
    notifier_call: Some(keystone_platform_notifier),
};

unsafe extern "C" fn keystone_init() {
    #[cfg(feature = "CONFIG_ARM_LPAE")]
    {
        // if (PHYS_OFFSET >= KEYSTONE_HIGH_PHYS_START)
        // PHYS_OFFSET is supplied by the architecture configuration.
        bus_register_notifier(&mut platform_bus_type, &mut platform_nb);
    }
    keystone_pm_runtime_init();
}

unsafe extern "C" fn keystone_pv_fixup() -> i64 {
    let mem_start = memblock_start_of_DRAM();
    let mem_end = memblock_end_of_DRAM();

    if mem_start >= KEYSTONE_LOW_PHYS_START && mem_end <= KEYSTONE_LOW_PHYS_END {
        return 0;
    }

    if mem_start < KEYSTONE_HIGH_PHYS_START || mem_end > KEYSTONE_HIGH_PHYS_END {
        // pr_crit("Invalid address space for memory (%08llx-%08llx)\n", ...)
        return 0;
    }

    let offset = KEYSTONE_HIGH_PHYS_START - KEYSTONE_LOW_PHYS_START;
    arch_phys_to_idmap_offset = -(offset as c_long);
    offset as i64
}

static keystone_match: [*const c_char; 6] = [
    b"ti,k2hk\0".as_ptr() as *const c_char,
    b"ti,k2e\0".as_ptr() as *const c_char,
    b"ti,k2l\0".as_ptr() as *const c_char,
    b"ti,k2g\0".as_ptr() as *const c_char,
    b"ti,keystone\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(KEYSTONE, "Keystone") ... MACHINE_END
// .dma_zone_size = SZ_2G when CONFIG_ZONE_DMA && CONFIG_ARM_LPAE
#[allow(dead_code)]
static _KEYSTONE_MACHINE: (*const c_char, unsafe extern "C" fn(), unsafe extern "C" fn() -> i64, *const *const c_char) =
    (b"Keystone\0".as_ptr() as *const c_char, keystone_init, keystone_pv_fixup, keystone_match.as_ptr());

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
