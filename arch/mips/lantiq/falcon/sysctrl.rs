// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2011 Thomas Langer <thomas.langer@lantiq.com>
 * Copyright (C) 2011 John Crispin <john@phrozen.org>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

/* infrastructure control register */
const SYS1_INFRAC: u32 = 0x00bc;
/* Configuration fuses for drivers and pll */
const STATUS_CONFIG: u32 = 0x0040;
/* GPE frequency selection */
const GPPC_OFFSET: u32 = 24;
const GPEFREQ_MASK: u32 = 0x0000C00;
const GPEFREQ_OFFSET: u32 = 10;
/* Clock status register */
const SYSCTL_CLKS: u32 = 0x0000;
/* Clock enable register */
const SYSCTL_CLKEN: u32 = 0x0004;
/* Clock clear register */
const SYSCTL_CLKCLR: u32 = 0x0008;
/* Activation Status Register */
const SYSCTL_ACTS: u32 = 0x0020;
/* Activation Register */
const SYSCTL_ACT: u32 = 0x0024;
/* Deactivation Register */
const SYSCTL_DEACT: u32 = 0x0028;
/* reboot Register */
const SYSCTL_RBT: u32 = 0x002c;
/* CPU0 Clock Control Register */
const SYS1_CPU0CC: u32 = 0x0040;
/* HRST_OUT_N Control Register */
const SYS1_HRSTOUTC: u32 = 0x00c0;
/* clock divider bit */
const CPU0CC_CPUDIV: u32 = 0x0001;

/* Activation Status Register */
const ACTS_ASC0_ACT: u32 = 0x00001000;
const ACTS_SSC0: u32 = 0x00002000;
const ACTS_ASC1_ACT: u32 = 0x00000800;
const ACTS_I2C_ACT: u32 = 0x00004000;
const ACTS_P0: u32 = 0x00010000;
const ACTS_P1: u32 = 0x00010000;
const ACTS_P2: u32 = 0x00020000;
const ACTS_P3: u32 = 0x00020000;
const ACTS_P4: u32 = 0x00040000;
const ACTS_PADCTRL0: u32 = 0x00100000;
const ACTS_PADCTRL1: u32 = 0x00100000;
const ACTS_PADCTRL2: u32 = 0x00200000;
const ACTS_PADCTRL3: u32 = 0x00200000;
const ACTS_PADCTRL4: u32 = 0x00400000;

extern "C" {
    static mut sysctl_membase: [*mut c_void; 3];
    static mut status_membase: *mut c_void;
    pub static mut ltq_sys1_membase: *mut c_void;
    pub static mut ltq_ebu_membase: *mut c_void;

    fn ltq_w32(value: u32, address: *mut c_void);
    fn ltq_r32(address: *mut c_void) -> u32;
    fn pr_err(format: *const c_char, ...);
    fn udelay(usecs: c_uint);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn clkdev_add(clk: *mut clk_lookup);
    fn clkdev_add_static(cpu: c_uint, fpi: c_uint, io: c_uint, unused: c_uint);
    fn ltq_sys1_r32(offset: u32) -> u32;
    fn panic(format: *const c_char, ... ) -> !;
    fn of_find_compatible_node(from: *mut device_node, ty: *const c_char, compatible: *const c_char) -> *mut device_node;
    fn of_address_to_resource(node: *mut device_node, index: c_uint, resource: *mut resource) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn request_mem_region(start: u64, size: u64, name: *const c_char) -> *mut resource;
    fn ioremap(start: u64, size: u64) -> *mut c_void;
    fn resource_size(resource: *const resource) -> u64;
}

#[repr(C)]
pub struct clk_lookup {
    pub dev_id: *const c_char,
    pub con_id: *const c_char,
    pub clk: *mut clk,
}

#[repr(C)]
pub struct clk {
    pub cl: clk_lookup,
    pub module: u32,
    pub bits: u32,
    pub activate: Option<unsafe extern "C" fn(*mut clk) -> c_int>,
    pub deactivate: Option<unsafe extern "C" fn(*mut clk)>,
    pub enable: Option<unsafe extern "C" fn(*mut clk) -> c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut clk)>,
    pub reboot: Option<unsafe extern "C" fn(*mut clk)>,
}

#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct resource { pub start: u64, pub end: u64, pub name: *const c_char }

const SYSCTL_SYS1: u32 = 0;
const SYSCTL_SYSETH: u32 = 1;
const CLOCK_200M: u32 = 200_000_000;
const CLOCK_100M: u32 = 100_000_000;
const CLOCK_400M: u32 = 400_000_000;

#[inline]
unsafe fn sysctl_w32(m: u32, x: u32, y: u32) { ltq_w32(x, sysctl_membase[m as usize].add(y as usize)); }
#[inline]
unsafe fn sysctl_r32(m: u32, x: u32) -> u32 { ltq_r32(sysctl_membase[m as usize].add(x as usize)) }
#[inline]
unsafe fn sysctl_w32_mask(m: u32, clear: u32, set: u32, reg: u32) {
    sysctl_w32(m, (sysctl_r32(m, reg) & !clear) | set, reg);
}
#[inline]
unsafe fn status_w32(x: u32, y: u32) { ltq_w32(x, status_membase.add(y as usize)); }
#[inline]
unsafe fn status_r32(x: u32) -> u32 { ltq_r32(status_membase.add(x as usize)) }

unsafe extern "C" fn sysctl_wait(clk: *mut clk, test: u32, reg: u32) {
    let mut err: c_int = 1_000_000;
    while { err -= 1; err != 0 && (sysctl_r32((*clk).module, reg) & (*clk).bits) != test } {}
    if err == 0 {
        pr_err(b"module de/activation failed %d %08X %08X %08X\0".as_ptr() as *const c_char,
            (*clk).module, (*clk).bits, test, sysctl_r32((*clk).module, reg) & (*clk).bits);
    }
}

unsafe extern "C" fn sysctl_activate(clk: *mut clk) -> c_int {
    sysctl_w32((*clk).module, (*clk).bits, SYSCTL_CLKEN);
    sysctl_w32((*clk).module, (*clk).bits, SYSCTL_ACT);
    sysctl_wait(clk, (*clk).bits, SYSCTL_ACTS);
    0
}
unsafe extern "C" fn sysctl_deactivate(clk: *mut clk) {
    sysctl_w32((*clk).module, (*clk).bits, SYSCTL_CLKCLR);
    sysctl_w32((*clk).module, (*clk).bits, SYSCTL_DEACT);
    sysctl_wait(clk, 0, SYSCTL_ACTS);
}
unsafe extern "C" fn sysctl_clken(clk: *mut clk) -> c_int {
    sysctl_w32((*clk).module, (*clk).bits, SYSCTL_CLKEN);
    sysctl_w32((*clk).module, (*clk).bits, SYSCTL_ACT);
    sysctl_wait(clk, (*clk).bits, SYSCTL_CLKS);
    0
}
unsafe extern "C" fn sysctl_clkdis(clk: *mut clk) {
    sysctl_w32((*clk).module, (*clk).bits, SYSCTL_CLKCLR);
    sysctl_wait(clk, 0, SYSCTL_CLKS);
}
unsafe extern "C" fn sysctl_reboot(clk: *mut clk) {
    let act = sysctl_r32((*clk).module, SYSCTL_ACT);
    let bits = !act & (*clk).bits;
    if bits != 0 {
        sysctl_w32((*clk).module, bits, SYSCTL_CLKEN);
        sysctl_w32((*clk).module, bits, SYSCTL_ACT);
        sysctl_wait(clk, bits, SYSCTL_ACTS);
    }
    sysctl_w32((*clk).module, act & (*clk).bits, SYSCTL_RBT);
    sysctl_wait(clk, (*clk).bits, SYSCTL_ACTS);
}

/* enable the ONU core */
unsafe fn falcon_gpe_enable() {
    let status = sysctl_r32(SYSCTL_SYS1, SYS1_INFRAC);
    if status & (1 << (GPPC_OFFSET + 1)) != 0 { return; }
    let mut freq = (status_r32(STATUS_CONFIG) & GPEFREQ_MASK) >> GPEFREQ_OFFSET;
    if freq == 0 { freq = 1; /* use 625MHz on unfused chip */ }
    sysctl_w32_mask(SYSCTL_SYS1, 7 << (GPPC_OFFSET + 1), freq << (GPPC_OFFSET + 2), SYS1_INFRAC);
    udelay(1);
    sysctl_w32_mask(SYSCTL_SYS1, 0, 1 << (GPPC_OFFSET + 1), SYS1_INFRAC);
    udelay(1);
}

unsafe fn clkdev_add_sys(dev: *const c_char, module: u32, bits: u32) {
    let clk = kzalloc(core::mem::size_of::<clk>(), 0) as *mut clk;
    if clk.is_null() { return; }
    (*clk).cl.dev_id = dev; (*clk).cl.con_id = core::ptr::null(); (*clk).cl.clk = clk;
    (*clk).module = module; (*clk).bits = bits;
    (*clk).activate = Some(sysctl_activate); (*clk).deactivate = Some(sysctl_deactivate);
    (*clk).enable = Some(sysctl_clken); (*clk).disable = Some(sysctl_clkdis); (*clk).reboot = Some(sysctl_reboot);
    clkdev_add(&mut (*clk).cl);
}

pub unsafe extern "C" fn ltq_soc_init() {
    let np_status = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"lantiq,status-falcon\0".as_ptr() as *const c_char);
    let np_ebu = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"lantiq,ebu-falcon\0".as_ptr() as *const c_char);
    let np_sys1 = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"lantiq,sys1-falcon\0".as_ptr() as *const c_char);
    let np_syseth = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"lantiq,syseth-falcon\0".as_ptr() as *const c_char);
    let np_sysgpe = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"lantiq,sysgpe-falcon\0".as_ptr() as *const c_char);
    let mut res_status = resource { start: 0, end: 0, name: core::ptr::null() };
    let mut res_ebu = resource { start: 0, end: 0, name: core::ptr::null() };
    let mut res_sys = [resource { start: 0, end: 0, name: core::ptr::null() }; 3];
    if np_status.is_null() || np_ebu.is_null() || np_sys1.is_null() || np_syseth.is_null() || np_sysgpe.is_null() { panic(b"Failed to load core nodes from devicetree\0".as_ptr() as *const c_char); }
    if of_address_to_resource(np_status, 0, &mut res_status) != 0 || of_address_to_resource(np_ebu, 0, &mut res_ebu) != 0 || of_address_to_resource(np_sys1, 0, &mut res_sys[0]) != 0 || of_address_to_resource(np_syseth, 0, &mut res_sys[1]) != 0 || of_address_to_resource(np_sysgpe, 0, &mut res_sys[2]) != 0 { panic(b"Failed to get core resources\0".as_ptr() as *const c_char); }
    of_node_put(np_status); of_node_put(np_ebu); of_node_put(np_sys1); of_node_put(np_syseth); of_node_put(np_sysgpe);
    if request_mem_region(res_status.start, resource_size(&res_status), res_status.name).is_null() || request_mem_region(res_ebu.start, resource_size(&res_ebu), res_ebu.name).is_null() || request_mem_region(res_sys[0].start, resource_size(&res_sys[0]), res_sys[0].name).is_null() || request_mem_region(res_sys[1].start, resource_size(&res_sys[1]), res_sys[1].name).is_null() || request_mem_region(res_sys[2].start, resource_size(&res_sys[2]), res_sys[2].name).is_null() { pr_err(b"Failed to request core resources\0".as_ptr() as *const c_char); }
    status_membase = ioremap(res_status.start, resource_size(&res_status));
    ltq_ebu_membase = ioremap(res_ebu.start, resource_size(&res_ebu));
    if status_membase.is_null() || ltq_ebu_membase.is_null() { panic(b"Failed to remap core resources\0".as_ptr() as *const c_char); }
    for i in 0..3 { sysctl_membase[i] = ioremap(res_sys[i].start, resource_size(&res_sys[i])); if sysctl_membase[i].is_null() { panic(b"Failed to remap sysctrl resources\0".as_ptr() as *const c_char); } }
    ltq_sys1_membase = sysctl_membase[0];
    falcon_gpe_enable();
    if ltq_sys1_r32(SYS1_CPU0CC) & CPU0CC_CPUDIV != 0 { clkdev_add_static(CLOCK_200M, CLOCK_100M, CLOCK_200M, 0); }
    else { clkdev_add_static(CLOCK_400M, CLOCK_100M, CLOCK_200M, 0); }
    clkdev_add_sys(b"1d810000.gpio\0".as_ptr() as *const c_char, SYSCTL_SYSETH, ACTS_P0);
    clkdev_add_sys(b"1d810100.gpio\0".as_ptr() as *const c_char, SYSCTL_SYSETH, ACTS_P2);
    clkdev_add_sys(b"1e800100.gpio\0".as_ptr() as *const c_char, SYSCTL_SYS1, ACTS_P1);
    clkdev_add_sys(b"1e800200.gpio\0".as_ptr() as *const c_char, SYSCTL_SYS1, ACTS_P3);
    clkdev_add_sys(b"1e800300.gpio\0".as_ptr() as *const c_char, SYSCTL_SYS1, ACTS_P4);
    clkdev_add_sys(b"1db01000.pad\0".as_ptr() as *const c_char, SYSCTL_SYSETH, ACTS_PADCTRL0);
    clkdev_add_sys(b"1db02000.pad\0".as_ptr() as *const c_char, SYSCTL_SYSETH, ACTS_PADCTRL2);
    clkdev_add_sys(b"1e800400.pad\0".as_ptr() as *const c_char, SYSCTL_SYS1, ACTS_PADCTRL1);
    clkdev_add_sys(b"1e800500.pad\0".as_ptr() as *const c_char, SYSCTL_SYS1, ACTS_PADCTRL3);
    clkdev_add_sys(b"1e800600.pad\0".as_ptr() as *const c_char, SYSCTL_SYS1, ACTS_PADCTRL4);
    clkdev_add_sys(b"1e100b00.serial\0".as_ptr() as *const c_char, SYSCTL_SYS1, ACTS_ASC1_ACT);
    clkdev_add_sys(b"1e100c00.serial\0".as_ptr() as *const c_char, SYSCTL_SYS1, ACTS_ASC0_ACT);
    clkdev_add_sys(b"1e100d00.spi\0".as_ptr() as *const c_char, SYSCTL_SYS1, ACTS_SSC0);
    clkdev_add_sys(b"1e200000.i2c\0".as_ptr() as *const c_char, SYSCTL_SYS1, ACTS_I2C_ACT);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
