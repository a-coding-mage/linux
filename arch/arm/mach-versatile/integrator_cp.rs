// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2003 Deep Blue Solutions Ltd
 */

// C dependencies supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct regmap;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct device_node;

#[repr(C)]
pub struct map_desc {
    pub virtual_: usize,
    pub pfn: usize,
    pub length: usize,
    pub type_: usize,
}

#[repr(C)]
pub struct mmci_platform_data {
    pub ocr_mask: u32,
    pub status: Option<unsafe extern "C" fn(*mut device) -> u32>,
}

#[repr(C)]
pub struct of_dev_auxdata {
    pub compatible: *const c_char,
    pub phys_addr: usize,
    pub name: *const c_char,
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

extern "C" {
    fn cm_init();
    fn irqchip_init();
    fn iotable_init(desc: *mut map_desc, size: usize);
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn regmap_read(map: *mut regmap, offset: u32, value: *mut u32) -> c_int;
    fn syscon_regmap_lookup_by_compatible(compatible: *const c_char) -> *mut regmap;
    fn is_err<T>(ptr: *mut T) -> bool;
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, rate: u32);
    fn of_find_matching_node(from: *mut device_node, matches: *const of_device_id) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: c_int) -> *mut c_void;
    fn of_platform_default_populate(
        root: *mut device_node,
        lookup: *mut of_dev_auxdata,
        parent: *mut device,
    ) -> c_int;
    fn integrator_reserve();
}

// Base address to the core module header
static mut cm_map: *mut regmap = core::ptr::null_mut();
// Base address to the CP controller
static mut intcp_con_base: *mut c_void = core::ptr::null_mut();

const CM_COUNTER_OFFSET: u32 = 0x28;

/*
 * Logical      Physical
 * f1400000    14000000    Interrupt controller
 * f1600000    16000000    UART 0
 * fca00000    ca000000    SIC
 */

static mut intcp_io_desc: [map_desc; 3] = [
    map_desc { virtual_: IO_ADDRESS(INTEGRATOR_IC_BASE), pfn: __phys_to_pfn(INTEGRATOR_IC_BASE), length: SZ_4K, type_: MT_DEVICE },
    map_desc { virtual_: IO_ADDRESS(INTEGRATOR_UART0_BASE), pfn: __phys_to_pfn(INTEGRATOR_UART0_BASE), length: SZ_4K, type_: MT_DEVICE },
    map_desc { virtual_: IO_ADDRESS(INTEGRATOR_CP_SIC_BASE), pfn: __phys_to_pfn(INTEGRATOR_CP_SIC_BASE), length: SZ_4K, type_: MT_DEVICE },
];

unsafe fn intcp_map_io() {
    iotable_init(intcp_io_desc.as_mut_ptr(), intcp_io_desc.len());
}

/*
 * It seems that the card insertion interrupt remains active after
 * we've acknowledged it.  We therefore ignore the interrupt, and
 * rely on reading it from the SIC.  This also means that we must
 * clear the latched interrupt.
 */
unsafe extern "C" fn mmc_status(_dev: *mut device) -> u32 {
    let status = readl(__io_address(0xca000000usize + 4) as *const c_void);
    writel(8, intcp_con_base.add(8));

    status & 8
}

static mut mmc_data: mmci_platform_data = mmci_platform_data {
    ocr_mask: MMC_VDD_32_33 | MMC_VDD_33_34,
    status: Some(mmc_status),
};

unsafe extern "C" fn intcp_read_sched_clock() -> u64 {
    let mut val = 0u32;

    // MMIO so discard return code
    regmap_read(cm_map, CM_COUNTER_OFFSET, &mut val);
    val as u64
}

unsafe fn intcp_init_irq_of() {
    cm_init();
    irqchip_init();
}

/*
 * For the Device Tree, add in the UART, MMC and CLCD specifics as AUXDATA
 * and enforce the bus names since these are used for clock lookups.
 */
static mut intcp_auxdata_lookup: [of_dev_auxdata; 2] = [
    of_dev_auxdata { compatible: c"arm,primecell", phys_addr: INTEGRATOR_CP_MMC_BASE, name: c"mmci", platform_data: unsafe { &mut mmc_data as *mut _ as *mut c_void } },
    of_dev_auxdata { compatible: core::ptr::null(), phys_addr: 0, name: core::ptr::null(), platform_data: core::ptr::null_mut() },
];

static intcp_syscon_match: [of_device_id; 2] = [
    of_device_id { compatible: c"arm,integrator-cp-syscon" },
    of_device_id { compatible: core::ptr::null() },
];

unsafe fn intcp_init_of() {
    let mut cpcon: *mut device_node;

    cm_map = syscon_regmap_lookup_by_compatible(c"arm,core-module-integrator");
    if !is_err(cm_map) {
        sched_clock_register(intcp_read_sched_clock, 32, 24000000);
    }

    cpcon = of_find_matching_node(core::ptr::null_mut(), intcp_syscon_match.as_ptr());
    if cpcon.is_null() {
        return;
    }

    intcp_con_base = of_iomap(cpcon, 0);
    if intcp_con_base.is_null() {
        return;
    }

    of_platform_default_populate(core::ptr::null_mut(), intcp_auxdata_lookup.as_mut_ptr(), core::ptr::null_mut());
}

static intcp_dt_board_compat: [*const c_char; 2] = [c"arm,integrator-cp", core::ptr::null()];

// DT_MACHINE_START(INTEGRATOR_CP_DT, "ARM Integrator/CP (Device Tree)")
//     .reserve = integrator_reserve,
//     .map_io = intcp_map_io,
//     .init_irq = intcp_init_irq_of,
//     .init_machine = intcp_init_of,
//     .dt_compat = intcp_dt_board_compat,
// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
