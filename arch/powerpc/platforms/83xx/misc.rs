// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * misc setup functions for MPC83xx
 *
 * Maintainer: Kumar Gala <galak@kernel.crashing.org>
 */

// Dependencies supplied by the surrounding kernel translation unit.

static mut restart_reg_base: *mut u32 = core::ptr::null_mut();

unsafe extern "C" {
    fn get_immrbase() -> usize;
    fn ioremap(addr: usize, size: usize) -> *mut u32;
    fn iounmap(addr: *mut u32);
    fn in_be32(addr: *mut u32) -> u32;
    fn out_be32(addr: *mut u32, value: u32);
    fn local_irq_disable();
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn of_find_compatible_node(from: *mut device_node, typ: *const core::ffi::c_char, compatible: *const core::ffi::c_char) -> *mut device_node;
    fn of_find_node_by_type(from: *mut device_node, typ: *const core::ffi::c_char) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn ipic_init(node: *mut device_node, flags: i32);
    fn ipic_set_default_priority();
    fn of_platform_bus_probe(node: *mut device_node, ids: *const of_device_id, parent: *mut core::ffi::c_void) -> i32;
    fn fix_to_virt(index: i32) -> usize;
    fn setbat(index: i32, virt: usize, phys: usize, size: usize, flags: usize);
    fn update_bats();
    fn machine_check_generic(regs: *mut pt_regs) -> i32;
    fn ipic_get_mcp_status() -> u32;
    fn ipic_clear_mcp_status(mask: u32);
    fn debugger_fault_handler(regs: *mut pt_regs) -> i32;
    fn die(msg: *const core::ffi::c_char, regs: *mut pt_regs, err: i32);
    fn mpc83xx_add_bridge(node: *mut device_node);
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    pub msr: u32,
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub name: *const core::ffi::c_char,
    pub type_: *const core::ffi::c_char,
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

extern "C" {
    static mut ppc_md_progress: Option<unsafe extern "C" fn(*const core::ffi::c_char, u32)>;
}

unsafe fn mpc83xx_restart_init() -> i32 {
    // map reset restart_register space
    restart_reg_base = ioremap(get_immrbase() + 0x900, 0xff);
    0
}

// Equivalent of arch_initcall(mpc83xx_restart_init).

pub unsafe extern "C" fn mpc83xx_restart(_cmd: *mut core::ffi::c_char) -> ! {
    const RST_PROT_REG: usize = 0x00000018;
    const RST_CTRL_REG: usize = 0x0000001c;

    local_irq_disable();

    if !restart_reg_base.is_null() {
        // enable software reset "RSTE"
        out_be32(restart_reg_base.add(RST_PROT_REG >> 2), 0x52535445);

        // set software hard reset
        out_be32(restart_reg_base.add(RST_CTRL_REG >> 2), 0x2);
    } else {
        // KERN_EMERG
        printk(b"Error: Restart registers not mapped, spinning!\0".as_ptr() as *const _);
    }

    loop {}
}

pub unsafe extern "C" fn mpc83xx_time_init() -> isize {
    const SPCR_OFFSET: usize = 0x00000110;
    const SPCR_TBEN: u32 = 0x00400000;
    let spcr = ioremap(get_immrbase() + SPCR_OFFSET, 4);
    let tmp = in_be32(spcr);
    out_be32(spcr, tmp | SPCR_TBEN);
    iounmap(spcr);
    0
}

pub unsafe extern "C" fn mpc83xx_ipic_init_IRQ() {
    let mut np: *mut device_node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"fsl,ipic\0".as_ptr() as *const _);
    if np.is_null() {
        np = of_find_node_by_type(core::ptr::null_mut(), b"ipic\0".as_ptr() as *const _);
    }
    if np.is_null() {
        return;
    }
    ipic_init(np, 0);
    of_node_put(np);
    ipic_set_default_priority();
}

static OF_BUS_IDS: [of_device_id; 9] = [
    of_device_id { name: core::ptr::null(), type_: b"soc\0".as_ptr() as *const _, compatible: core::ptr::null(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: b"soc\0".as_ptr() as *const _, data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: b"simple-bus\0".as_ptr() as *const _, data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: b"gianfar\0".as_ptr() as *const _, data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: b"gpio-leds\0".as_ptr() as *const _, data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: b"qe\0".as_ptr() as *const _, compatible: core::ptr::null(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: b"fsl,qe\0".as_ptr() as *const _, data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: core::ptr::null(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: core::ptr::null(), data: core::ptr::null() },
];

pub unsafe extern "C" fn mpc83xx_declare_of_platform_devices() -> i32 {
    of_platform_bus_probe(core::ptr::null_mut(), OF_BUS_IDS.as_ptr(), core::ptr::null_mut());
    0
}

pub unsafe extern "C" fn mpc83xx_setup_pci() {
    // CONFIG_PCI conditional retained from the source; compatible-node iteration is external.
}

pub unsafe extern "C" fn mpc83xx_setup_arch() {
    let immrbase = get_immrbase();
    let immrsize = if immrbase % (2 * 1024 * 1024) == 0 { 2 * 1024 * 1024 } else { 1024 * 1024 };
    let va = fix_to_virt(0);
    if let Some(progress) = ppc_md_progress {
        progress(b"mpc83xx_setup_arch()\0".as_ptr() as *const _, 0);
    }
    setbat(-1, va, immrbase, immrsize, 0);
    update_bats();
}

pub unsafe extern "C" fn machine_check_83xx(regs: *mut pt_regs) -> i32 {
    const IPIC_MCP_WDT: u32 = 0;
    const SRR1_MCE_MCP: u32 = 1 << 31;
    let mask = 1u32 << (31 - IPIC_MCP_WDT);
    if ((*regs).msr & SRR1_MCE_MCP) == 0 || (ipic_get_mcp_status() & mask) == 0 {
        return machine_check_generic(regs);
    }
    ipic_clear_mcp_status(mask);
    if debugger_fault_handler(regs) != 0 {
        return 1;
    }
    die(b"Watchdog NMI Reset\0".as_ptr() as *const _, regs, 0);
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
