// SPDX-License-Identifier: GPL-2.0+
/* Translation of ipmi_si_platform.c. Kernel-provided types and functions are external dependencies. */

static mut PLATFORM_REGISTERED: bool = false;
static mut SI_TRYPLATFORM: bool = true;
// CONFIG_ACPI: static mut SI_TRYACPI: bool = true;
// CONFIG_OF: static mut SI_TRYOPENFIRMWARE: bool = true;
// CONFIG_DMI: static mut SI_TRYDMI: bool = true; otherwise false.

#[cfg(feature = "config_acpi")]
unsafe extern "C" {
    fn ipmi_si_irq_handler(irq: i32, data: *mut core::ffi::c_void);
    fn ipmi_irq_start_cleanup(io: *mut si_sm_io);
    fn acpi_remove_gpe_handler(device: *mut core::ffi::c_void, irq: u32, handler: unsafe extern "C" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> u32);
    fn acpi_install_gpe_handler(device: *mut core::ffi::c_void, irq: u32, trigger: u32, handler: unsafe extern "C" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> u32, context: *mut core::ffi::c_void) -> i32;
    fn ipmi_irq_finish_setup(io: *mut si_sm_io);
}

unsafe extern "C" {
    fn resource_type(r: *const resource) -> u64;
    fn platform_get_mem_or_io(pdev: *mut platform_device, index: u32) -> *mut resource;
    fn device_property_read_u8(dev: *mut device, name: *const u8, value: *mut u8) -> i32;
    fn platform_get_irq_optional(pdev: *mut platform_device, index: u32) -> i32;
    fn ipmi_addr_src_to_str(source: u8) -> *const u8;
    fn ipmi_si_add_smi(io: *mut si_sm_io) -> i32;
    fn ipmi_si_remove_by_dev(dev: *mut device);
    fn ipmi_std_irq_setup(io: *mut si_sm_io) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn bus_find_device(bus: *mut bus_type, start: *mut device, data: *const core::ffi::c_void, match_fn: unsafe extern "C" fn(*mut device, *const core::ffi::c_void) -> i32) -> *mut device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn put_device(dev: *mut device);
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn strcmp(a: *const u8, b: *const u8) -> i32;
}

#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct platform_device { pub dev: device, pub name: *const u8 }
#[repr(C)] pub struct resource { pub start: usize }
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct bus_type;
#[repr(C)] pub struct platform_driver;
#[repr(C)] pub struct si_info { pub type_: u32 }
#[repr(C)] pub struct acpi_info { pub acpi_handle: *mut core::ffi::c_void }
#[repr(C)] pub union addr_info_union { pub acpi_info: acpi_info }
#[repr(C)] pub struct si_sm_io {
    pub addr_source: u8, pub addr_space: u8, pub addr_data: usize,
    pub regspacing: u32, pub regsize: u32, pub regshift: u32,
    pub slave_addr: u8, pub irq: i32, pub dev: *mut device,
    pub si_info: *const si_info, pub irq_setup: Option<unsafe extern "C" fn(*mut si_sm_io) -> i32>,
    pub irq_cleanup: Option<unsafe extern "C" fn(*mut si_sm_io)>,
    pub irq_handler_data: *mut core::ffi::c_void, pub addr_info: addr_info_union,
}

const SI_PLATFORM: u8 = 0; const SI_SMBIOS: u8 = 1; const SI_HARDCODED: u8 = 2;
const SI_LAST: u8 = 3; const SI_KCS: u8 = 0; const SI_SMIC: u8 = 1; const SI_BT: u8 = 2;
const SI_TYPE_INVALID: u8 = 0xff; const SI_DEVICETREE: u8 = 3; const SI_ACPI: u8 = 4;
const IPMI_IO_ADDR_SPACE: u8 = 0; const IPMI_MEM_ADDR_SPACE: u8 = 1;
const IORESOURCE_IO: u64 = 0x100; const DEFAULT_REGSIZE: u32 = 1; const DEFAULT_REGSPACING: u32 = 1;

extern "C" { static ipmi_kcs_si_info: si_info; static ipmi_smic_si_info: si_info; static ipmi_bt_si_info: si_info; }

unsafe fn ipmi_set_addr_data_and_space(r: *mut resource, io: *mut si_sm_io) {
    (*io).addr_space = if resource_type(r) == IORESOURCE_IO { IPMI_IO_ADDR_SPACE } else { IPMI_MEM_ADDR_SPACE };
    (*io).addr_data = (*r).start;
}

unsafe fn ipmi_get_info_from_resources(pdev: *mut platform_device, io: *mut si_sm_io) -> *mut resource {
    let res = platform_get_mem_or_io(pdev, 0); if res.is_null() { return core::ptr::null_mut(); }
    ipmi_set_addr_data_and_space(res, io); (*io).regspacing = DEFAULT_REGSPACING;
    let second = platform_get_mem_or_io(pdev, 1);
    if !second.is_null() && resource_type(second) == resource_type(res) && (*second).start > (*io).addr_data { (*io).regspacing = ((*second).start - (*io).addr_data) as u32; }
    res
}

unsafe fn platform_ipmi_probe(pdev: *mut platform_device) -> i32 {
    let mut io: si_sm_io = core::mem::zeroed(); let mut source = 0u8; let mut ty = 0u8;
    if device_property_read_u8(&mut (*pdev).dev, b"addr-source\0".as_ptr(), &mut source) != 0 { source = SI_PLATFORM; }
    if source >= SI_LAST || (source == SI_SMBIOS && !SI_TRYDMI) || (source != SI_HARDCODED && source != SI_SMBIOS && !SI_TRYPLATFORM) { return -19; }
    if device_property_read_u8(&mut (*pdev).dev, b"ipmi-type\0".as_ptr(), &mut ty) != 0 { return -19; }
    io.addr_source = source; io.si_info = match ty { SI_KCS => &ipmi_kcs_si_info, SI_SMIC => &ipmi_smic_si_info, SI_BT => &ipmi_bt_si_info, SI_TYPE_INVALID => return -19, _ => return -22 };
    io.regsize = DEFAULT_REGSIZE; io.regshift = 0; let _ = ipmi_get_info_from_resources(pdev, &mut io); io.slave_addr = 0x20;
    io.irq = platform_get_irq_optional(pdev, 0); if io.irq <= 0 { io.irq = 0; } else { io.irq_setup = Some(ipmi_std_irq_setup); }
    io.dev = &mut (*pdev).dev; ipmi_si_add_smi(&mut io)
}

unsafe extern "C" fn ipmi_probe(pdev: *mut platform_device) -> i32 { platform_ipmi_probe(pdev) }
unsafe extern "C" fn ipmi_remove(pdev: *mut platform_device) { ipmi_si_remove_by_dev(&mut (*pdev).dev); }

pub unsafe fn ipmi_remove_platform_device_by_name(name: *mut u8) {
    loop { let dev = bus_find_device(core::ptr::null_mut(), core::ptr::null_mut(), name.cast(), pdev_match_name); if dev.is_null() { break; } platform_device_unregister(to_platform_device(dev)); put_device(dev); }
}
unsafe extern "C" fn pdev_match_name(dev: *mut device, data: *const core::ffi::c_void) -> i32 { let p = to_platform_device(dev); (strcmp((*p).name, data.cast()) == 0) as i32 }

#[no_mangle] pub unsafe extern "C" fn ipmi_si_platform_init() { let rv = platform_driver_register(core::ptr::null_mut()); if rv == 0 { PLATFORM_REGISTERED = true; } }
#[no_mangle] pub unsafe extern "C" fn ipmi_si_platform_shutdown() { if PLATFORM_REGISTERED { platform_driver_unregister(core::ptr::null_mut()); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
