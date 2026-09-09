// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 *  Copyright (C) 2010 John Crispin <john@phrozen.org>
 */

// Linux and platform dependencies are supplied by the surrounding kernel bindings.

const PCI_CR_FCI_ADDR_MAP0: u32 = 0x00C0;
const PCI_CR_FCI_ADDR_MAP1: u32 = 0x00C4;
const PCI_CR_FCI_ADDR_MAP2: u32 = 0x00C8;
const PCI_CR_FCI_ADDR_MAP3: u32 = 0x00CC;
const PCI_CR_FCI_ADDR_MAP4: u32 = 0x00D0;
const PCI_CR_FCI_ADDR_MAP5: u32 = 0x00D4;
const PCI_CR_FCI_ADDR_MAP6: u32 = 0x00D8;
const PCI_CR_FCI_ADDR_MAP7: u32 = 0x00DC;
const PCI_CR_CLK_CTRL: u32 = 0x0000;
const PCI_CR_PCI_MOD: u32 = 0x0030;
const PCI_CR_PC_ARB: u32 = 0x0080;
const PCI_CR_FCI_ADDR_MAP11HG: u32 = 0x00E4;
const PCI_CR_BAR11MASK: u32 = 0x0044;
const PCI_CR_BAR12MASK: u32 = 0x0048;
const PCI_CR_BAR13MASK: u32 = 0x004C;
const PCI_CS_BASE_ADDR1: u32 = 0x0010;
const PCI_CR_PCI_ADDR_MAP11: u32 = 0x0064;
const PCI_CR_FCI_BURST_LENGTH: u32 = 0x00E8;
const PCI_CR_PCI_EOI: u32 = 0x002C;
const PCI_CS_STS_CMD: u32 = 0x0004;

const PCI_MASTER0_REQ_MASK_2BITS: u32 = 8;
const PCI_MASTER1_REQ_MASK_2BITS: u32 = 10;
const PCI_MASTER2_REQ_MASK_2BITS: u32 = 12;
const INTERNAL_ARB_ENABLE_BIT: u32 = 0;
const LTQ_CGU_IFCCR: u32 = 0x0018;
const LTQ_CGU_PCICR: u32 = 0x0034;

#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct resource { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct pci_ops { pub read: Option<unsafe extern "C" fn()>, pub write: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct pci_controller {
    pub pci_ops: *mut pci_ops,
    pub mem_resource: *mut resource,
    pub mem_offset: usize,
    pub io_resource: *mut resource,
    pub io_offset: usize,
}
#[repr(C)] pub struct of_device_id { pub compatible: *const u8 }
#[repr(C)] pub struct platform_driver { _private: [u8; 0] }

extern "C" {
    static mut ltq_pci_mapped_cfg: *mut u8;
    static mut LTQ_EBU_PCC_CON: u32;
    static mut LTQ_EBU_PCC_IEN: u32;
    static mut ltq_pci_driver: platform_driver;
    fn ltq_pci_read_config_dword();
    fn ltq_pci_write_config_dword();
    fn ltq_w32(value: u32, address: *mut u8);
    fn ltq_r32(address: *mut u8) -> u32;
    fn ltq_ebu_w32(value: u32, address: u32);
    fn ltq_ebu_r32(address: u32) -> u32;
    fn get_num_physpages() -> usize;
    fn fls(value: u32) -> u32;
    fn wmb();
    fn clk_get(dev: *mut device, name: *const u8) -> *mut clk;
    fn clk_put(clock: *mut clk);
    fn clk_set_rate(clock: *mut clk, rate: u32);
    fn clk_enable(clock: *mut clk);
    fn clk_disable(clock: *mut clk);
    fn of_get_property(node: *mut device_node, name: *const u8, length: *mut usize) -> *const u32;
    fn of_property_read_bool(node: *mut device_node, name: *const u8) -> bool;
    fn devm_gpiod_get_optional(dev: *mut device, name: *const u8, flags: u32) -> *mut gpio_desc;
    fn ptr_err_or_zero(ptr: *mut gpio_desc) -> i32;
    fn gpiod_set_consumer_name(gpio: *mut gpio_desc, name: *const u8);
    fn gpiod_set_value_cansleep(gpio: *mut gpio_desc, value: i32);
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: i32, resource: *mut *mut resource) -> *mut u8;
    fn ptr_err(ptr: *mut u8) -> i32;
    fn pci_clear_flags(flags: u32);
    fn pci_load_of_ranges(controller: *mut pci_controller, node: *mut device_node);
    fn register_pci_controller(controller: *mut pci_controller);
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn pr_info(message: *const u8);
}

static mut ltq_pci_membase: *mut u8 = core::ptr::null_mut();
static mut reset_gpio: *mut gpio_desc = core::ptr::null_mut();
static mut clk_pci: *mut clk = core::ptr::null_mut();
static mut clk_external: *mut clk = core::ptr::null_mut();
static mut pci_io_resource: resource = resource { _private: [] };
static mut pci_mem_resource: resource = resource { _private: [] };
static mut pci_ops_instance: pci_ops = pci_ops { read: Some(ltq_pci_read_config_dword), write: Some(ltq_pci_write_config_dword) };
static mut pci_controller_instance: pci_controller = pci_controller {
    pci_ops: &raw mut pci_ops_instance,
    mem_resource: &raw mut pci_mem_resource,
    mem_offset: 0,
    io_resource: &raw mut pci_io_resource,
    io_offset: 0,
};

#[inline]
unsafe fn ltq_pci_w32(x: u32, y: u32) { ltq_w32(x, ltq_pci_membase.add(y as usize)); }
#[inline]
unsafe fn ltq_pci_r32(x: u32) -> u32 { ltq_r32(ltq_pci_membase.add(x as usize)) }
#[inline]
unsafe fn ltq_pci_cfg_w32(x: u32, y: u32) { ltq_w32(x, ltq_pci_mapped_cfg.add(y as usize)); }
#[inline]
unsafe fn ltq_pci_cfg_r32(x: u32) -> u32 { ltq_r32(ltq_pci_mapped_cfg.add(x as usize)) }

unsafe fn ltq_calc_bar11mask() -> u32 {
    let mem = (get_num_physpages() as u32).wrapping_mul(4096);
    (0x0ffffff0 & !((1u32 << (fls(mem).wrapping_sub(1))) - 1)) | 8
}

unsafe fn ltq_pci_startup(pdev: *mut platform_device) -> i32 {
    let node = (*pdev).dev.of_node;
    let mut temp_buffer: u32;
    let error: i32;
    clk_pci = clk_get(&mut (*pdev).dev, core::ptr::null());
    if clk_pci.is_null() { return -1; }
    clk_external = clk_get(&mut (*pdev).dev, b"external\0".as_ptr());
    if clk_external.is_null() { clk_put(clk_pci); return -1; }
    let bus_clk = of_get_property(node, b"lantiq,bus-clock\0".as_ptr(), core::ptr::null_mut());
    if !bus_clk.is_null() { clk_set_rate(clk_pci, *bus_clk); }
    clk_enable(clk_pci);
    if of_property_read_bool(node, b"lantiq,external-clock\0".as_ptr()) { clk_enable(clk_external); } else { clk_disable(clk_external); }
    reset_gpio = devm_gpiod_get_optional(&mut (*pdev).dev, b"reset\0".as_ptr(), 1);
    error = ptr_err_or_zero(reset_gpio);
    if error != 0 { return error; }
    gpiod_set_consumer_name(reset_gpio, b"pci_reset\0".as_ptr());
    ltq_pci_w32(0xa, PCI_CR_CLK_CTRL);
    ltq_pci_w32(ltq_pci_r32(PCI_CR_PCI_MOD) & !(1 << 24), PCI_CR_PCI_MOD); wmb();
    ltq_pci_cfg_w32(ltq_pci_cfg_r32(PCI_CS_STS_CMD) | 7, PCI_CS_STS_CMD);
    temp_buffer = ltq_pci_r32(PCI_CR_PC_ARB);
    let req_mask = of_get_property(node, b"req-mask\0".as_ptr(), core::ptr::null_mut());
    if !req_mask.is_null() { temp_buffer &= !((*req_mask & 0xf) << 16); } else { temp_buffer &= !0xf0000; }
    temp_buffer |= 1 << INTERNAL_ARB_ENABLE_BIT;
    temp_buffer &= !(3 << PCI_MASTER0_REQ_MASK_2BITS);
    temp_buffer &= !(3 << PCI_MASTER1_REQ_MASK_2BITS);
    temp_buffer &= !(3 << PCI_MASTER2_REQ_MASK_2BITS);
    ltq_pci_w32(temp_buffer, PCI_CR_PC_ARB); wmb();
    ltq_pci_w32(0x18000000, PCI_CR_FCI_ADDR_MAP0); ltq_pci_w32(0x18400000, PCI_CR_FCI_ADDR_MAP1);
    ltq_pci_w32(0x18800000, PCI_CR_FCI_ADDR_MAP2); ltq_pci_w32(0x18c00000, PCI_CR_FCI_ADDR_MAP3);
    ltq_pci_w32(0x19000000, PCI_CR_FCI_ADDR_MAP4); ltq_pci_w32(0x19400000, PCI_CR_FCI_ADDR_MAP5);
    ltq_pci_w32(0x19800000, PCI_CR_FCI_ADDR_MAP6); ltq_pci_w32(0x19c00000, PCI_CR_FCI_ADDR_MAP7);
    ltq_pci_w32(0x1ae00000, PCI_CR_FCI_ADDR_MAP11HG); ltq_pci_w32(ltq_calc_bar11mask(), PCI_CR_BAR11MASK);
    ltq_pci_w32(0, PCI_CR_PCI_ADDR_MAP11); ltq_pci_w32(0, PCI_CS_BASE_ADDR1);
    ltq_pci_w32(ltq_pci_r32(PCI_CR_PCI_EOI) | 3, PCI_CR_PCI_EOI); wmb();
    ltq_pci_w32(ltq_pci_r32(PCI_CR_BAR12MASK) | 0x80000000, PCI_CR_BAR12MASK);
    ltq_pci_w32(ltq_pci_r32(PCI_CR_BAR13MASK) | 0x80000000, PCI_CR_BAR13MASK);
    ltq_pci_w32(0x303, PCI_CR_FCI_BURST_LENGTH); ltq_pci_w32(ltq_pci_r32(PCI_CR_PCI_MOD) | (1 << 24), PCI_CR_PCI_MOD); wmb();
    ltq_ebu_w32(ltq_ebu_r32(LTQ_EBU_PCC_CON) | 0xc, LTQ_EBU_PCC_CON);
    ltq_ebu_w32(ltq_ebu_r32(LTQ_EBU_PCC_IEN) | 0x10, LTQ_EBU_PCC_IEN);
    if !reset_gpio.is_null() { gpiod_set_value_cansleep(reset_gpio, 1); wmb(); /* mdelay(1); */ gpiod_set_value_cansleep(reset_gpio, 0); }
    0
}

unsafe fn ltq_pci_probe(pdev: *mut platform_device) -> i32 {
    pci_clear_flags(1);
    ltq_pci_membase = devm_platform_get_and_ioremap_resource(pdev, 1, core::ptr::null_mut());
    if ltq_pci_membase.is_null() { return ptr_err(ltq_pci_membase); }
    ltq_pci_mapped_cfg = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if ltq_pci_mapped_cfg.is_null() { return ptr_err(ltq_pci_mapped_cfg); }
    ltq_pci_startup(pdev);
    pci_load_of_ranges(&raw mut pci_controller_instance, (*pdev).dev.of_node);
    register_pci_controller(&raw mut pci_controller_instance); 0
}

// The platform-driver match table and registration retain the source driver's external kernel integration.
unsafe fn pcibios_init() -> i32 {
    let ret = platform_driver_register(&raw mut ltq_pci_driver);
    if ret != 0 { pr_info(b"pci-xway: Error registering platform driver!\0".as_ptr()); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
