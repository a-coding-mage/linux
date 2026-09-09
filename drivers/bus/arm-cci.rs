/*
 * CCI cache coherent interconnect driver
 *
 * Copyright (C) 2013 ARM Ltd.
 * Author: Lorenzo Pieralisi <lorenzo.pieralisi@arm.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation.
 */

// Dependencies supplied by the surrounding kernel translation unit.

static mut cci_ctrl_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut cci_ctrl_phys: libc::c_ulong = 0;

#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
#[repr(C)]
struct cci_nb_ports { nb_ace: u32, nb_ace_lite: u32 }

#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
static cci400_ports: cci_nb_ports = cci_nb_ports { nb_ace: 2, nb_ace_lite: 3 };

#[repr(C)]
struct of_device_id { compatible: *const core::ffi::c_char, data: *const core::ffi::c_void }
#[repr(C)]
struct of_dev_auxdata { compatible: *const core::ffi::c_char, phys_addr: libc::c_ulong, name: *const core::ffi::c_char, platform_data: *mut *mut core::ffi::c_void }

static arm_cci_matches: [of_device_id; 4] = [
    of_device_id { compatible: b"arm,cci-400\0" as *const u8 as *const _, data: core::ptr::null() },
    of_device_id { compatible: b"arm,cci-500\0" as *const u8 as *const _, data: core::ptr::null() },
    of_device_id { compatible: b"arm,cci-550\0" as *const u8 as *const _, data: core::ptr::null() },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];
static arm_cci_auxdata: [of_dev_auxdata; 6] = [
    of_dev_auxdata { compatible: b"arm,cci-400-pmu\0" as *const u8 as *const _, phys_addr: 0, name: core::ptr::null(), platform_data: unsafe { &mut cci_ctrl_base } },
    of_dev_auxdata { compatible: b"arm,cci-400-pmu,r0\0" as *const u8 as *const _, phys_addr: 0, name: core::ptr::null(), platform_data: unsafe { &mut cci_ctrl_base } },
    of_dev_auxdata { compatible: b"arm,cci-400-pmu,r1\0" as *const u8 as *const _, phys_addr: 0, name: core::ptr::null(), platform_data: unsafe { &mut cci_ctrl_base } },
    of_dev_auxdata { compatible: b"arm,cci-500-pmu,r0\0" as *const u8 as *const _, phys_addr: 0, name: core::ptr::null(), platform_data: unsafe { &mut cci_ctrl_base } },
    of_dev_auxdata { compatible: b"arm,cci-550-pmu,r0\0" as *const u8 as *const _, phys_addr: 0, name: core::ptr::null(), platform_data: unsafe { &mut cci_ctrl_base } },
    of_dev_auxdata { compatible: core::ptr::null(), phys_addr: 0, name: core::ptr::null(), platform_data: core::ptr::null_mut() },
];

const DRIVER_NAME: &[u8] = b"ARM-CCI\0";

#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
pub enum device_node {}
#[repr(C)] struct platform_driver { driver: driver, probe: Option<unsafe extern "C" fn(*mut platform_device) -> libc::c_int> }
#[repr(C)] struct driver { name: *const libc::c_char, of_match_table: *const of_device_id }

extern "C" {
    fn cci_probed() -> bool;
    fn of_platform_populate(np: *mut device_node, matches: *const of_device_id, aux: *const of_dev_auxdata, parent: *mut device) -> libc::c_int;
    fn platform_driver_register(drv: *mut platform_driver) -> libc::c_int;
}

unsafe extern "C" fn cci_platform_probe(pdev: *mut platform_device) -> libc::c_int {
    if !cci_probed() { return -19; }
    of_platform_populate((*pdev).dev.of_node, core::ptr::null(), arm_cci_auxdata.as_ptr(), &mut (*pdev).dev)
}
static mut cci_platform_driver: platform_driver = platform_driver { driver: driver { name: DRIVER_NAME.as_ptr() as *const _, of_match_table: arm_cci_matches.as_ptr() }, probe: Some(cci_platform_probe) };
unsafe fn cci_platform_init() -> libc::c_int { platform_driver_register(&mut cci_platform_driver) }

#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
const CCI_PORT_CTRL: usize = 0x0;
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
const CCI_CTRL_STATUS: usize = 0xc;
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
const CCI_ENABLE_SNOOP_REQ: u32 = 0x1;
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
const CCI_ENABLE_DVM_REQ: u32 = 0x2;
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
const CCI_ENABLE_REQ: u32 = CCI_ENABLE_SNOOP_REQ | CCI_ENABLE_DVM_REQ;

#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
#[repr(C)] #[derive(Copy, Clone, PartialEq)] enum cci_ace_port_type { ACE_INVALID_PORT = 0, ACE_PORT, ACE_LITE_PORT }
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
#[repr(C)] struct cci_ace_port { base: *mut u8, phys: libc::c_ulong, port_type: cci_ace_port_type, dn: *mut device_node }
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
static mut ports: *mut cci_ace_port = core::ptr::null_mut();
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
static mut nb_cci_ports: u32 = 0;
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
#[repr(C)] #[derive(Copy, Clone)] struct cpu_port { mpidr: u64, port: u32 }
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
const PORT_VALID_SHIFT: u32 = 31;
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
const PORT_VALID: u32 = 1u32 << PORT_VALID_SHIFT;
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
static mut cpu_port_array: [cpu_port; 256] = [cpu_port { mpidr: 0, port: 0 }; 256];

#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
#[inline] unsafe fn init_cpu_port(port: *mut cpu_port, index: u32, mpidr: u64) { (*port).port = PORT_VALID | index; (*port).mpidr = mpidr; }
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
#[inline] unsafe fn cpu_port_is_valid(port: *mut cpu_port) -> bool { ((*port).port & PORT_VALID) != 0 }
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
#[inline] unsafe fn cpu_port_match(port: *mut cpu_port, mpidr: u64) -> bool { (*port).mpidr == (mpidr & 0xff00_ffff) }

#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
unsafe fn __cci_ace_get_port(dn: *mut device_node, port_type: i32) -> i32 {
    let cci_portn = of_parse_phandle(dn, b"cci-control-port\0".as_ptr() as *const _, 0);
    for i in 0..nb_cci_ports { let p = ports.add(i as usize); if (*p).port_type as i32 == port_type && cci_portn == (*p).dn { return i as i32; } }
    -19
}
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
pub unsafe fn cci_ace_get_port(dn: *mut device_node) -> i32 { __cci_ace_get_port(dn, cci_ace_port_type::ACE_LITE_PORT as i32) }

#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
unsafe fn cci_port_control(port: u32, enable: bool) { writel_relaxed(if enable { CCI_ENABLE_REQ } else { 0 }, ports.add(port as usize).as_ref().unwrap().base.add(CCI_PORT_CTRL)); while readl_relaxed(cci_ctrl_base.add(CCI_CTRL_STATUS)) & 1 != 0 {} }
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
pub unsafe fn cci_disable_port_by_cpu(mpidr: u64) -> i32 { for cpu in 0..256 { if cpu_port_is_valid(&mut cpu_port_array[cpu]) && cpu_port_match(&mut cpu_port_array[cpu], mpidr) { cci_port_control(cpu_port_array[cpu].port, false); return 0; } } -19 }
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
pub unsafe fn cci_enable_port_for_self() -> ! { loop { core::arch::asm!("wfi\n wfe", options(noreturn)); } }
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
pub unsafe fn __cci_control_port_by_device(dn: *mut device_node, enable: bool) -> i32 { if dn.is_null() { return -19; } let port = __cci_ace_get_port(dn, cci_ace_port_type::ACE_LITE_PORT as i32); if port < 0 { return -19; } cci_port_control(port as u32, enable); 0 }
#[cfg(feature = "CONFIG_ARM_CCI400_PORT_CTRL")]
pub unsafe fn __cci_control_port_by_index(port: u32, enable: bool) -> i32 { if port >= nb_cci_ports || (*ports.add(port as usize)).port_type == cci_ace_port_type::ACE_INVALID_PORT { return -19; } if (*ports.add(port as usize)).port_type == cci_ace_port_type::ACE_PORT { return -1; } cci_port_control(port, enable); 0 }

extern "C" {
    fn of_parse_phandle(dn: *mut device_node, name: *const libc::c_char, index: libc::c_int) -> *mut device_node;
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn of_device_is_available(np: *mut device_node) -> bool;
    fn of_find_matching_node(from: *mut device_node, matches: *const of_device_id) -> *mut device_node;
    fn cci_probe_ports(np: *mut device_node) -> libc::c_int;
    fn of_address_to_resource(np: *mut device_node, index: libc::c_int, res: *mut resource) -> libc::c_int;
    fn ioremap(start: libc::c_ulong, size: libc::c_ulong) -> *mut core::ffi::c_void;
    fn resource_size(res: *const resource) -> libc::c_ulong;
    fn mutex_lock(m: *mut core::ffi::c_void); fn mutex_unlock(m: *mut core::ffi::c_void);
}
#[repr(C)] struct resource { start: libc::c_ulong }

unsafe fn cci_probe() -> libc::c_int { let np = of_find_matching_node(core::ptr::null_mut(), arm_cci_matches.as_ptr()); if !of_device_is_available(np) { return -19; } let mut res = resource { start: 0 }; let ret = of_address_to_resource(np, 0, &mut res); if ret == 0 { cci_ctrl_base = ioremap(res.start, resource_size(&res)); cci_ctrl_phys = res.start; } if ret != 0 || cci_ctrl_base.is_null() { return -6; } cci_probe_ports(np) }
static mut cci_init_status: libc::c_int = -11;
static mut cci_probing: core::ffi::c_void = core::mem::zeroed();
unsafe fn cci_init() -> libc::c_int { if cci_init_status != -11 { return cci_init_status; } mutex_lock(&mut cci_probing); if cci_init_status == -11 { cci_init_status = cci_probe(); } mutex_unlock(&mut cci_probing); cci_init_status }
pub unsafe fn cci_probed() -> bool { cci_init() == 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
