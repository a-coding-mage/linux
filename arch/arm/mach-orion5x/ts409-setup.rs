// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * QNAP TS-409 Board Setup
 *
 * Maintainer: Sylver Bruneau <sylver.bruneau@gmail.com>
 *
 * Copyright (C) 2008  Sylver Bruneau <sylver.bruneau@gmail.com>
 * Copyright (C) 2008  Martin Michlmayr <tbm@cyrius.com>
 */

// Kernel and board dependencies supplied by the surrounding translation.

const QNAP_TS409_NOR_BOOT_BASE: usize = 0xff800000;
const QNAP_TS409_NOR_BOOT_SIZE: usize = 8 * 1024 * 1024;

#[repr(C)]
struct MtdPartition { name: *const u8, size: usize, offset: usize, mask_flags: u32 }
#[repr(C)]
struct PhysmapFlashData { width: u32, parts: *mut MtdPartition, nr_parts: usize }
#[repr(C)]
struct Resource { flags: u64, start: usize, end: usize }
#[repr(C)]
struct Device { platform_data: *mut core::ffi::c_void }
#[repr(C)]
struct PlatformDevice { name: *const u8, id: i32, dev: Device, num_resources: usize, resource: *mut Resource }
#[repr(C)]
struct PciDev;
#[repr(C)]
struct HwPci { nr_controllers: u32, setup: Option<unsafe extern "C" fn()>, scan: Option<unsafe extern "C" fn()>, map_irq: Option<unsafe extern "C" fn(*const PciDev, u8, u8) -> i32> }
#[repr(C)]
struct I2cBoardInfo { type_name: [u8; 32], addr: u16, irq: i32 }
#[repr(C)]
struct GpioLed { name: *const u8 }
#[repr(C)]
struct GpioLookupTable;
#[repr(C)]
struct GpioLedPlatformData { leds: *mut GpioLed, num_leds: usize }
#[repr(C)]
struct GpioKeysButton { code: u16, gpio: u32, desc: *const u8, active_low: u8 }
#[repr(C)]
struct GpioKeysPlatformData { buttons: *mut GpioKeysButton, nbuttons: usize }

const MTD_WRITEABLE: u32 = 0x400;
const IORESOURCE_MEM: u64 = 0x00000200;
const TS409_RTC_GPIO: u32 = 10;
const QNAP_TS409_GPIO_KEY_RESET: u32 = 14;
const QNAP_TS409_GPIO_KEY_MEDIA: u32 = 15;

static mut qnap_ts409_partitions: [MtdPartition; 6] = [
    MtdPartition { name: b"U-Boot\0".as_ptr(), size: 0x00080000, offset: 0x00780000, mask_flags: MTD_WRITEABLE },
    MtdPartition { name: b"Kernel\0".as_ptr(), size: 0x00200000, offset: 0, mask_flags: 0 },
    MtdPartition { name: b"RootFS1\0".as_ptr(), size: 0x00400000, offset: 0x00200000, mask_flags: 0 },
    MtdPartition { name: b"RootFS2\0".as_ptr(), size: 0x00100000, offset: 0x00600000, mask_flags: 0 },
    MtdPartition { name: b"U-Boot Config\0".as_ptr(), size: 0x00020000, offset: 0x00760000, mask_flags: 0 },
    MtdPartition { name: b"NAS Config\0".as_ptr(), size: 0x00060000, offset: 0x00700000, mask_flags: MTD_WRITEABLE },
];

static mut qnap_ts409_nor_flash_data: PhysmapFlashData = PhysmapFlashData { width: 1, parts: unsafe { qnap_ts409_partitions.as_mut_ptr() }, nr_parts: 6 };
static mut qnap_ts409_nor_flash_resource: Resource = Resource { flags: IORESOURCE_MEM, start: QNAP_TS409_NOR_BOOT_BASE, end: QNAP_TS409_NOR_BOOT_BASE + QNAP_TS409_NOR_BOOT_SIZE - 1 };
static mut qnap_ts409_nor_flash: PlatformDevice = PlatformDevice { name: b"physmap-flash\0".as_ptr(), id: 0, dev: Device { platform_data: unsafe { &mut qnap_ts409_nor_flash_data as *mut _ as *mut _ } }, num_resources: 1, resource: unsafe { &mut qnap_ts409_nor_flash_resource } };

extern "C" {
    fn orion5x_pci_map_irq(dev: *const PciDev, slot: u8, pin: u8) -> i32;
    fn pci_common_init(pci: *mut HwPci);
    fn machine_is_ts409() -> bool;
}

unsafe extern "C" fn qnap_ts409_pci_map_irq(dev: *const PciDev, slot: u8, pin: u8) -> i32 {
    let irq = orion5x_pci_map_irq(dev, slot, pin);
    if irq != -1 { return irq; }
    -1
}

static mut qnap_ts409_pci: HwPci = HwPci { nr_controllers: 2, setup: None, scan: None, map_irq: Some(qnap_ts409_pci_map_irq) };

unsafe extern "C" fn qnap_ts409_pci_init() -> i32 {
    if machine_is_ts409() { pci_common_init(&mut qnap_ts409_pci); }
    0
}

// subsys_initcall(qnap_ts409_pci_init);

static mut qnap_ts409_i2c_rtc: I2cBoardInfo = I2cBoardInfo { type_name: *b"s35390a\0", addr: 0x30, irq: 0 };
static mut ts409_led_pins: [GpioLed; 4] = [
    GpioLed { name: b"ts409:red:sata1\0".as_ptr() }, GpioLed { name: b"ts409:red:sata2\0".as_ptr() },
    GpioLed { name: b"ts409:red:sata3\0".as_ptr() }, GpioLed { name: b"ts409:red:sata4\0".as_ptr() },
];
static mut ts409_leds_gpio_table: GpioLookupTable = GpioLookupTable;
static mut ts409_led_data: GpioLedPlatformData = GpioLedPlatformData { leds: unsafe { ts409_led_pins.as_mut_ptr() }, num_leds: 4 };
static mut ts409_leds: PlatformDevice = PlatformDevice { name: b"leds-gpio\0".as_ptr(), id: -1, dev: Device { platform_data: unsafe { &mut ts409_led_data as *mut _ as *mut _ } }, num_resources: 0, resource: core::ptr::null_mut() };
static mut qnap_ts409_buttons: [GpioKeysButton; 2] = [
    GpioKeysButton { code: 0x198, gpio: QNAP_TS409_GPIO_KEY_RESET, desc: b"Reset Button\0".as_ptr(), active_low: 1 },
    GpioKeysButton { code: 0x1a6, gpio: QNAP_TS409_GPIO_KEY_MEDIA, desc: b"USB Copy Button\0".as_ptr(), active_low: 1 },
];
static mut qnap_ts409_button_data: GpioKeysPlatformData = GpioKeysPlatformData { buttons: unsafe { qnap_ts409_buttons.as_mut_ptr() }, nbuttons: 2 };
static mut qnap_ts409_button_device: PlatformDevice = PlatformDevice { name: b"gpio-keys\0".as_ptr(), id: -1, dev: Device { platform_data: unsafe { &mut qnap_ts409_button_data as *mut _ as *mut _ } }, num_resources: 0, resource: core::ptr::null_mut() };

static mut ts409_mpp_modes: [u32; 21] = [0,0,0,0,1,1,1,1,0,0,1,0,0,0,1,1,2,2,0,0,0];

unsafe extern "C" fn qnap_ts409_init() {
    // Setup basic Orion functions. Need to be called early.
    orion5x_init();
    orion5x_mpp_conf(ts409_mpp_modes.as_ptr());
    mvebu_mbus_add_window_by_id(0, 0, QNAP_TS409_NOR_BOOT_BASE, QNAP_TS409_NOR_BOOT_SIZE);
    platform_device_register(&mut qnap_ts409_nor_flash);
    orion5x_ehci0_init();
    qnap_tsx09_find_mac_addr(QNAP_TS409_NOR_BOOT_BASE + qnap_ts409_partitions[5].offset, qnap_ts409_partitions[5].size);
    orion5x_eth_init(); orion5x_i2c_init(); orion5x_uart0_init(); orion5x_uart1_init();
    platform_device_register(&mut qnap_ts409_button_device);
    if gpio_request(TS409_RTC_GPIO, b"rtc\0".as_ptr()) == 0 {
        if gpio_direction_input(TS409_RTC_GPIO) == 0 { qnap_ts409_i2c_rtc.irq = gpio_to_irq(TS409_RTC_GPIO); }
        else { gpio_free(TS409_RTC_GPIO); }
    }
    if qnap_ts409_i2c_rtc.irq == 0 { pr_warn(b"qnap_ts409_init: failed to get RTC IRQ\n\0".as_ptr()); }
    i2c_register_board_info(0, &mut qnap_ts409_i2c_rtc, 1);
    gpiod_add_lookup_table(&mut ts409_leds_gpio_table);
    platform_device_register(&mut ts409_leds);
    register_platform_power_off(qnap_tsx09_power_off);
}

// MACHINE_START(TS409, "QNAP TS-409")
// Maintainer: Sylver Bruneau <sylver.bruneau@gmail.com>
#[repr(C)]
struct MachineDesc {
    name: *const u8,
    atag_offset: u32,
    nr_irqs: u32,
    init_machine: Option<unsafe extern "C" fn()>,
    map_io: Option<unsafe extern "C" fn()>,
    init_early: Option<unsafe extern "C" fn()>,
    init_irq: Option<unsafe extern "C" fn()>,
    init_time: Option<unsafe extern "C" fn()>,
    fixup: Option<unsafe extern "C" fn()>,
    restart: Option<unsafe extern "C" fn()>,
}

// MACHINE_END
static mut TS409_MACHINE: MachineDesc = MachineDesc {
    name: b"QNAP TS-409\0".as_ptr(),
    atag_offset: 0x100,
    nr_irqs: 0,
    init_machine: Some(qnap_ts409_init),
    map_io: None,
    init_early: None,
    init_irq: None,
    init_time: None,
    fixup: None,
    restart: None,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
