// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MTX-1 platform devices registration (Au1500)
 *
 * Copyright (C) 2007-2009, Florian Fainelli <florian@openwrt.org>
 */

// Linux and MIPS platform dependencies supplied by other translation units.

extern "C" {
    fn alchemy_uart_putchar(addr: usize, c: i8);
    fn alchemy_gpio_direction_output(gpio: i32, value: i32);
    fn alchemy_wrsys(value: u32, reg: u32);
    fn alchemy_gpio_set_value(gpio: i32, value: i32);
    fn udelay(usecs: u32);
    fn software_node_register_node_group(nodes: *const *const software_node) -> i32;
    fn software_node_fwnode(node: *const software_node) -> *mut core::ffi::c_void;
    fn platform_device_register_full(info: *const platform_device_info) -> *mut platform_device;
    fn irq_set_irq_type(irq: i32, irq_type: u32);
    fn au1xxx_override_eth_cfg(index: i32, data: *mut au1000_eth_platform_data);
    fn platform_add_devices(devices: *const *mut platform_device, count: usize) -> i32;
    fn printk(fmt: *const i8, ...);
    fn pr_err(fmt: *const i8, ...);
    fn ptr_err_or_zero(ptr: *mut platform_device) -> i32;
}

#[repr(C)]
pub struct software_node { pub name: *const i8, pub parent: *const software_node, pub properties: *const property_entry }
#[repr(C)]
pub struct property_entry { pub opaque: usize }
#[repr(C)]
pub struct platform_device_info { pub name: *const i8, pub id: i32, pub fwnode: *mut core::ffi::c_void, pub properties: *const property_entry, pub swnode: *const software_node }
#[repr(C)]
pub struct platform_device { pub name: *const i8, pub id: i32, pub dev: device, pub num_resources: usize, pub resource: *mut resource }
#[repr(C)] pub struct device { pub platform_data: *mut core::ffi::c_void }
#[repr(C)] pub struct resource { pub start: usize, pub end: usize, pub flags: u32 }
#[repr(C)] pub struct mtd_partition { pub name: *const i8, pub size: usize, pub offset: usize, pub mask_flags: u32 }
#[repr(C)] pub struct physmap_flash_data { pub width: u32, pub nr_parts: u32, pub parts: *mut mtd_partition }
#[repr(C)] pub struct pci_dev { pub opaque: usize }
#[repr(C)] pub struct alchemy_pci_platdata { pub board_map_irq: Option<unsafe extern "C" fn(*const pci_dev, u8, u8) -> i32>, pub board_pci_idsel: Option<unsafe extern "C" fn(u32, i32) -> i32>, pub pci_cfg_set: u32 }
#[repr(C)] pub struct au1000_eth_platform_data { pub phy_search_highest_addr: i32, pub phy1_search_mac0: i32 }

static mut pm_power_off: Option<unsafe extern "C" fn()> = None;
static mut _machine_halt: Option<unsafe extern "C" fn()> = None;
static mut _machine_restart: Option<unsafe extern "C" fn(*mut i8)> = None;

pub unsafe extern "C" fn get_system_type() -> *const i8 { b"MTX-1\0".as_ptr() as *const i8 }

pub unsafe extern "C" fn prom_putchar(c: i8) { alchemy_uart_putchar(AU1000_UART0_PHYS_ADDR, c); }

unsafe extern "C" fn mtx1_reset(_c: *mut i8) {
    // Jump to the reset vector
    core::arch::asm!("jr {target}", target = in(reg) 0xbfc00000usize);
}

unsafe extern "C" fn mtx1_power_off() -> ! {
    loop { core::arch::asm!(".set mips32", "wait", ".set mips0"); }
}

pub unsafe extern "C" fn board_setup() {
    // Build-time CONFIG_USB_OHCI_HCD condition is preserved here.
    #[cfg(feature = "CONFIG_USB_OHCI_HCD")] { alchemy_gpio_direction_output(204, 0); }
    alchemy_wrsys(SYS_PF_NI2, AU1000_SYS_PINFUNC);
    alchemy_wrsys(!0, AU1000_SYS_TRIOUTCLR);
    alchemy_gpio_direction_output(0, 0);
    alchemy_gpio_direction_output(3, 1);
    alchemy_gpio_direction_output(1, 1);
    alchemy_gpio_direction_output(5, 0);
    alchemy_gpio_direction_output(211, 1);
    alchemy_gpio_direction_output(212, 0);
    pm_power_off = Some(mtx1_power_off);
    _machine_halt = Some(mtx1_power_off);
    _machine_restart = Some(mtx1_reset);
    printk(b"4G Systems MTX-1 Board\n\0".as_ptr() as *const i8);
}

static mtx1_gpio_keys_node: software_node = software_node { name: b"mtx1-gpio-keys\0".as_ptr() as *const i8, parent: core::ptr::null(), properties: core::ptr::null() };
static mtx1_button_props: [property_entry; 1] = [property_entry { opaque: 0 }];
static mtx1_button_node: software_node = software_node { name: core::ptr::null(), parent: &mtx1_gpio_keys_node, properties: mtx1_button_props.as_ptr() };
static mtx1_gpio_keys_swnodes: [*const software_node; 3] = [&mtx1_gpio_keys_node, &mtx1_button_node, core::ptr::null()];

unsafe extern "C" fn mtx1_keys_init() {
    let mut keys_info = platform_device_info { name: b"gpio-keys\0".as_ptr() as *const i8, id: -1, fwnode: core::ptr::null_mut(), properties: core::ptr::null(), swnode: core::ptr::null() };
    let err = software_node_register_node_group(mtx1_gpio_keys_swnodes.as_ptr());
    if err != 0 { pr_err(b"failed to register gpio-keys software nodes: %d\n\0".as_ptr() as *const i8, err); return; }
    keys_info.fwnode = software_node_fwnode(&mtx1_gpio_keys_node);
    let pd = platform_device_register_full(&keys_info);
    let err = ptr_err_or_zero(pd);
    if err != 0 { pr_err(b"failed to create gpio-keys device: %d\n\0".as_ptr() as *const i8, err); }
}

static mtx1_wdt_props: [property_entry; 1] = [property_entry { opaque: 0 }];
static mtx1_wdt_info: platform_device_info = platform_device_info { name: b"mtx1-wdt\0".as_ptr() as *const i8, id: 0, fwnode: core::ptr::null_mut(), properties: mtx1_wdt_props.as_ptr(), swnode: core::ptr::null() };
unsafe extern "C" fn mtx1_wdt_init() { let pd = platform_device_register_full(&mtx1_wdt_info); let err = ptr_err_or_zero(pd); if err != 0 { pr_err(b"failed to create watchdog device: %d\n\0".as_ptr() as *const i8, err); } }

static mtx1_gpio_leds_node: software_node = software_node { name: b"mtx1-leds\0".as_ptr() as *const i8, parent: core::ptr::null(), properties: core::ptr::null() };
static mtx1_green_led_props: [property_entry; 1] = [property_entry { opaque: 0 }];
static mtx1_green_led_node: software_node = software_node { name: b"mtx1:green\0".as_ptr() as *const i8, parent: &mtx1_gpio_leds_node, properties: mtx1_green_led_props.as_ptr() };
static mtx1_red_led_props: [property_entry; 1] = [property_entry { opaque: 0 }];
static mtx1_red_led_node: software_node = software_node { name: b"mtx1:red\0".as_ptr() as *const i8, parent: &mtx1_gpio_leds_node, properties: mtx1_red_led_props.as_ptr() };
static mtx1_gpio_leds_swnodes: [*const software_node; 4] = [&mtx1_gpio_leds_node, &mtx1_green_led_node, &mtx1_red_led_node, core::ptr::null()];

unsafe extern "C" fn mtx1_leds_init() {
    let pdevinfo = platform_device_info { name: b"leds-gpio\0".as_ptr() as *const i8, id: -1, fwnode: core::ptr::null_mut(), properties: core::ptr::null(), swnode: &mtx1_gpio_leds_node };
    let err = software_node_register_node_group(mtx1_gpio_leds_swnodes.as_ptr());
    if err != 0 { pr_err(b"failed to register LED software nodes: %d\n\0".as_ptr() as *const i8, err); return; }
    let led_dev = platform_device_register_full(&pdevinfo); let err = ptr_err_or_zero(led_dev);
    if err != 0 { pr_err(b"failed to create LED device: %d\n\0".as_ptr() as *const i8, err); }
}

static mut mtx1_mtd_partitions: [mtd_partition; 4] = [
    mtd_partition { name: b"filesystem\0".as_ptr() as *const i8, size: 0x01c00000, offset: 0, mask_flags: 0 },
    mtd_partition { name: b"yamon\0".as_ptr() as *const i8, size: 0x00100000, offset: MTDPART_OFS_APPEND, mask_flags: MTD_WRITEABLE },
    mtd_partition { name: b"kernel\0".as_ptr() as *const i8, size: 0x002c0000, offset: MTDPART_OFS_APPEND, mask_flags: 0 },
    mtd_partition { name: b"yamon env\0".as_ptr() as *const i8, size: 0x00040000, offset: MTDPART_OFS_APPEND, mask_flags: 0 },
];
static mut mtx1_flash_data: physmap_flash_data = physmap_flash_data { width: 4, nr_parts: 4, parts: core::ptr::null_mut() };
static mut mtx1_mtd_resource: resource = resource { start: 0x1e000000, end: 0x1fffffff, flags: IORESOURCE_MEM };
static mut mtx1_mtd: platform_device = platform_device { name: b"physmap-flash\0".as_ptr() as *const i8, id: 0, dev: device { platform_data: core::ptr::null_mut() }, num_resources: 1, resource: core::ptr::null_mut() };
static mut alchemy_pci_host_res: [resource; 1] = [resource { start: AU1500_PCI_PHYS_ADDR, end: AU1500_PCI_PHYS_ADDR + 0xfff, flags: IORESOURCE_MEM }];

unsafe extern "C" fn mtx1_pci_idsel(devsel: u32, assert: i32) -> i32 { udelay(1); if assert != 0 && devsel != 0 { alchemy_gpio_set_value(1, 0); } else { alchemy_gpio_set_value(1, 1); } udelay(1); 1 }
static mtx1_irqtab: [[i32; 5]; 8] = [[-1, AU1500_PCI_INTA, AU1500_PCI_INTA, 0xff, 0xff], [-1, AU1500_PCI_INTB, AU1500_PCI_INTA, 0xff, 0xff], [-1, AU1500_PCI_INTC, AU1500_PCI_INTD, 0xff, 0xff], [-1, AU1500_PCI_INTD, AU1500_PCI_INTC, 0xff, 0xff], [-1, AU1500_PCI_INTA, AU1500_PCI_INTB, 0xff, 0xff], [-1, AU1500_PCI_INTB, AU1500_PCI_INTA, 0xff, 0xff], [-1, AU1500_PCI_INTC, AU1500_PCI_INTD, 0xff, 0xff], [-1, AU1500_PCI_INTD, AU1500_PCI_INTC, 0xff, 0xff]];
unsafe extern "C" fn mtx1_map_pci_irq(_d: *const pci_dev, slot: u8, pin: u8) -> i32 { mtx1_irqtab[slot as usize][pin as usize] }
static mut mtx1_pci_pd: alchemy_pci_platdata = alchemy_pci_platdata { board_map_irq: Some(mtx1_map_pci_irq), board_pci_idsel: Some(mtx1_pci_idsel), pci_cfg_set: PCI_CONFIG_AEN | PCI_CONFIG_R2H | PCI_CONFIG_R1H | PCI_CONFIG_CH };
static mut mtx1_pci_host: platform_device = platform_device { name: b"alchemy-pci\0".as_ptr() as *const i8, id: 0, dev: device { platform_data: core::ptr::null_mut() }, num_resources: 1, resource: core::ptr::null_mut() };
static mut mtx1_devs: [*mut platform_device; 2] = [core::ptr::null_mut(), core::ptr::null_mut()];
static mut mtx1_au1000_eth0_pdata: au1000_eth_platform_data = au1000_eth_platform_data { phy_search_highest_addr: 1, phy1_search_mac0: 1 };

unsafe extern "C" fn mtx1_register_devices() -> i32 {
    irq_set_irq_type(AU1500_GPIO204_INT, IRQ_TYPE_LEVEL_HIGH); irq_set_irq_type(AU1500_GPIO201_INT, IRQ_TYPE_LEVEL_LOW); irq_set_irq_type(AU1500_GPIO202_INT, IRQ_TYPE_LEVEL_LOW); irq_set_irq_type(AU1500_GPIO203_INT, IRQ_TYPE_LEVEL_LOW); irq_set_irq_type(AU1500_GPIO205_INT, IRQ_TYPE_LEVEL_LOW);
    au1xxx_override_eth_cfg(0, &mut mtx1_au1000_eth0_pdata);
    let rc = platform_add_devices(mtx1_devs.as_ptr(), 2); if rc != 0 { return rc; }
    mtx1_leds_init(); mtx1_wdt_init(); mtx1_keys_init(); 0
}

// arch_initcall(mtx1_register_devices)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
