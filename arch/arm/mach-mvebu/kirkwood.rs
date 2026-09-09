// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2012 (C), Jason Cooper <jason@lakedaemon.net>
 *
 * arch/arm/mach-mvebu/kirkwood.c
 *
 * Flattened Device Tree board initialization
 */

// Linux kernel and board headers supply the external types, constants, macros,
// and functions referenced below.

#[repr(C)]
pub struct resource {
    pub start: usize,
    pub end: usize,
    pub flags: usize,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const u8,
    pub id: i32,
    pub num_resources: usize,
    pub resource: *mut resource,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct property {
    pub name: *mut u8,
    pub length: usize,
    pub value: *mut u8,
}
#[repr(C)]
pub struct of_dev_auxdata {
    pub compatible: *const u8,
    pub phys_addr: usize,
    pub name: *const u8,
    pub platform_data: *mut core::ffi::c_void,
}

const IORESOURCE_MEM: usize = 0x0000_0200;
const MV643XX_ETH_MAC_ADDR_LOW: usize = 0x0414;
const MV643XX_ETH_MAC_ADDR_HIGH: usize = 0x0418;

extern "C" {
    static CPU_CONTROL_PHYS: usize;
    static DDR_OPERATION_BASE: usize;
    static CPU_CONFIG_PHYS: usize;
    static CPU_CONFIG_ERROR_PROP: u32;

    fn platform_device_register(dev: *mut platform_device) -> i32;
    fn for_each_compatible_node(np: *mut *mut device_node, ty: *const core::ffi::c_void, compatible: *const u8);
    fn of_get_parent(np: *mut device_node) -> *mut device_node;
    fn of_device_is_available(np: *mut device_node) -> bool;
    fn of_get_mac_address(np: *mut device_node, mac: *mut u8) -> bool;
    fn of_clk_get(np: *mut device_node, index: i32) -> *mut clk;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut u8;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn kzalloc(size: usize, flags: usize) -> *mut property;
    fn kstrdup(s: *const u8, flags: usize) -> *mut u8;
    fn kfree(p: *mut property);
    fn iounmap(addr: *mut u8);
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_put(clk: *mut clk);
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn of_update_property(np: *mut device_node, prop: *mut property) -> i32;
    fn ioremap(phys: usize, size: usize) -> *mut u8;
    fn mvebu_mbus_dt_init(sync: bool) -> i32;
    fn feroceon_of_init();
    fn kirkwood_pm_init();
    fn of_platform_default_populate(root: *mut device_node, auxdata: *mut of_dev_auxdata, parent: *mut core::ffi::c_void);
    fn mvebu_restart();
}

static mut KIRKWOOD_CPUFREQ_RESOURCES: [resource; 1] = [resource {
    start: 0,
    end: 0,
    flags: IORESOURCE_MEM,
}];

static mut KIRKWOOD_CPUFREQ_DEVICE: platform_device = platform_device {
    name: b"kirkwood-cpufreq\0".as_ptr(),
    id: -1,
    num_resources: 1,
    resource: core::ptr::null_mut(),
};

unsafe fn kirkwood_cpufreq_init() {
    platform_device_register(&raw mut KIRKWOOD_CPUFREQ_DEVICE);
}

static mut KIRKWOOD_CPUIDLE_RESOURCE: [resource; 1] = [resource {
    flags: IORESOURCE_MEM,
    start: 0,
    end: 0,
}];

static mut KIRKWOOD_CPUIDLE: platform_device = platform_device {
    name: b"kirkwood_cpuidle\0".as_ptr(),
    id: -1,
    resource: core::ptr::null_mut(),
    num_resources: 1,
};

unsafe fn kirkwood_cpuidle_init() {
    platform_device_register(&raw mut KIRKWOOD_CPUIDLE);
}

unsafe fn kirkwood_dt_eth_fixup() {
    // The ethernet interfaces forget the MAC address assigned by u-boot if the
    // clocks are turned off. Update the port node from the hardware registers
    // when no valid MAC address is set.
    let mut np: *mut device_node = core::ptr::null_mut();
    // for_each_compatible_node is a kernel iterator macro; its body is shown
    // literally here and is expected to be provided by the surrounding port.
    while !np.is_null() {
        let pnp = of_get_parent(np);
        if pnp.is_null() {
            continue;
        }
        let mut tmpmac = [0u8; 6];
        if !of_device_is_available(pnp) || !of_get_mac_address(np, tmpmac.as_mut_ptr()) {
            of_node_put(pnp);
            continue;
        }
        let clk = of_clk_get(pnp, 0);
        if clk.is_null() {
            of_node_put(pnp);
            continue;
        }
        let io = of_iomap(pnp, 0);
        if io.is_null() {
            clk_put(clk);
            of_node_put(pnp);
            continue;
        }
        clk_prepare_enable(clk);
        let pmac = kzalloc(core::mem::size_of::<property>() + 6, 0);
        if pmac.is_null() {
            iounmap(io);
            clk_disable_unprepare(clk);
            clk_put(clk);
            of_node_put(pnp);
            continue;
        }
        (*pmac).value = (*pmac as *mut u8).add(1 + core::mem::size_of::<property>() - 1);
        (*pmac).length = 6;
        (*pmac).name = kstrdup(b"local-mac-address\0".as_ptr(), 0);
        if (*pmac).name.is_null() {
            kfree(pmac);
            iounmap(io);
            clk_disable_unprepare(clk);
            clk_put(clk);
            of_node_put(pnp);
            continue;
        }
        let macaddr = (*pmac).value;
        let reg = readl(io.add(MV643XX_ETH_MAC_ADDR_HIGH));
        *macaddr.add(0) = (reg >> 24) as u8;
        *macaddr.add(1) = (reg >> 16) as u8;
        *macaddr.add(2) = (reg >> 8) as u8;
        *macaddr.add(3) = reg as u8;
        let reg = readl(io.add(MV643XX_ETH_MAC_ADDR_LOW));
        *macaddr.add(4) = (reg >> 8) as u8;
        *macaddr.add(5) = reg as u8;
        of_update_property(np, pmac);
        iounmap(io);
        clk_disable_unprepare(clk);
        clk_put(clk);
        of_node_put(pnp);
        break;
    }
}

unsafe fn kirkwood_disable_mbus_error_propagation() {
    let cpu_config = ioremap(CPU_CONFIG_PHYS, 4);
    writel(readl(cpu_config) & !CPU_CONFIG_ERROR_PROP, cpu_config);
}

static mut AUXDATA: [of_dev_auxdata; 2] = [
    of_dev_auxdata {
        compatible: b"marvell,kirkwood-audio\0".as_ptr(),
        phys_addr: 0xf10a0000,
        name: b"mvebu-audio\0".as_ptr(),
        platform_data: core::ptr::null_mut(),
    },
    of_dev_auxdata { compatible: core::ptr::null(), phys_addr: 0, name: core::ptr::null(), platform_data: core::ptr::null_mut() },
];

unsafe fn kirkwood_dt_init() {
    kirkwood_disable_mbus_error_propagation();
    if mvebu_mbus_dt_init(false) != 0 { panic!("BUG_ON"); }
    // CONFIG_CACHE_FEROCEON_L2: feroceon_of_init();
    kirkwood_cpufreq_init();
    kirkwood_cpuidle_init();
    kirkwood_pm_init();
    kirkwood_dt_eth_fixup();
    of_platform_default_populate(core::ptr::null_mut(), &raw mut AUXDATA[0], core::ptr::null_mut());
}

static KIRKWOOD_DT_BOARD_COMPAT: [*const u8; 2] = [b"marvell,kirkwood\0".as_ptr(), core::ptr::null()];

// DT_MACHINE_START(KIRKWOOD_DT, "Marvell Kirkwood (Flattened Device Tree)")
// Maintainer: Jason Cooper <jason@lakedaemon.net>
// .init_machine = kirkwood_dt_init, .restart = mvebu_restart,
// .dt_compat = kirkwood_dt_board_compat; MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
