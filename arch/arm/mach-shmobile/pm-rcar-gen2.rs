// SPDX-License-Identifier: GPL-2.0
/*
 * R-Car Generation 2 Power management support
 *
 * Copyright (C) 2013 - 2015  Renesas Electronics Corporation
 * Copyright (C) 2011  Renesas Solutions Corp.
 * Copyright (C) 2011  Magnus Damm
 */

// Dependencies supplied by the surrounding kernel translation.

const RST: usize = 0xe6160000;

const CA15BAR: usize = 0x0020; // CA15 Boot Address Register
const CA7BAR: usize = 0x0030; // CA7 Boot Address Register
const CA15RESCNT: usize = 0x0040; // CA15 Reset Control Register
const CA7RESCNT: usize = 0x0044; // CA7 Reset Control Register

const SBAR_BAREN: u32 = 1 << 4; // SBAR is valid

const CA15RESCNT_CODE: u32 = 0xa5a5_0000;
const CA15RESCNT_CPUS: u32 = 0xf; // CPU0-3
const CA7RESCNT_CODE: u32 = 0x5a5a_0000;
const CA7RESCNT_CPUS: u32 = 0xf; // CPU0-3

const ICRAM1: usize = 0xe63c0000; // Inter Connect RAM1 (4 KiB)

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Resource {
    pub start: usize,
    pub end: usize,
    pub flags: usize,
}

unsafe extern "C" {
    fn request_mem_region(start: usize, size: usize, name: *const u8) -> *mut Resource;
    fn pr_err(format: *const u8, ...);
    fn of_device_is_compatible(np: *mut DeviceNode, compatible: *const u8) -> bool;
    fn of_find_compatible_node(from: *mut DeviceNode, typ: *const u8, compatible: *const u8) -> *mut DeviceNode;
    fn of_node_put(np: *mut DeviceNode);
    fn of_address_to_resource(np: *mut DeviceNode, index: usize, resource: *mut Resource) -> i32;
    fn ioremap(addr: usize, size: usize) -> *mut u8;
    fn iounmap(addr: *mut u8);
    fn memcpy_toio(dst: *mut u8, src: *const u8, size: usize);
    fn read_cpuid_mpidr() -> u32;
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn shmobile_smp_apmu_suspend_init();
}

unsafe extern "C" {
    static mut shmobile_boot_size: usize;
    static mut shmobile_boot_size_gen2: usize;
    static mut shmobile_boot_cpu_gen2: u32;
    static shmobile_boot_vector: u8;
    static shmobile_boot_vector_gen2: u8;
}

#[inline]
fn phys_to_sbar(addr: usize) -> u32 {
    ((addr >> 8) & 0xffff_fc00) as u32
}

pub unsafe fn rcar_gen2_pm_init() {
    let mut p: *mut u8;
    let mut bar: u32;
    let mut np: *mut DeviceNode;
    let mut has_a7 = false;
    let mut has_a15 = false;
    let mut res: Resource;
    let error: i32;

    if request_mem_region(0, 256 * 1024, b"Boot Area\0".as_ptr()).is_null() {
        pr_err(b"Failed to request boot area\n\0".as_ptr());
        return;
    }

    // for_each_of_cpu_node(np)
    // The device-tree CPU-node iterator is supplied by the surrounding kernel.
    // Its body is translated literally here:
    // if (of_device_is_compatible(np, "arm,cortex-a15")) has_a15 = true;
    // else if (of_device_is_compatible(np, "arm,cortex-a7")) has_a7 = true;

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"renesas,smp-sram\0".as_ptr());
    if np.is_null() {
        // No smp-sram in DT, fall back to hardcoded address
        res = Resource { start: ICRAM1, end: ICRAM1 + shmobile_boot_size - 1, flags: 0 };
    } else {
        error = of_address_to_resource(np, 0, &mut res);
        of_node_put(np);
        if error != 0 {
            pr_err(b"Failed to get smp-sram address: %d\n\0".as_ptr(), error);
            return;
        }
    }

    // RAM for jump stub, because BAR requires 256KB aligned address
    let resource_size = res.end.wrapping_sub(res.start).wrapping_add(1);
    if (res.start & (256 * 1024 - 1)) != 0 || resource_size < shmobile_boot_size {
        pr_err(b"Invalid smp-sram region\n\0".as_ptr());
        return;
    }

    p = ioremap(res.start, resource_size);
    if p.is_null() {
        return;
    }
    /*
     * install the reset vector, use the largest version if we have enough
     * memory available
     */
    if resource_size >= shmobile_boot_size_gen2 {
        shmobile_boot_cpu_gen2 = read_cpuid_mpidr();
        memcpy_toio(p, &shmobile_boot_vector_gen2, shmobile_boot_size_gen2);
    } else {
        memcpy_toio(p, &shmobile_boot_vector, shmobile_boot_size);
    }
    iounmap(p);

    // setup reset vectors
    p = ioremap(RST, 0x63);
    bar = phys_to_sbar(res.start);
    if has_a15 {
        writel_relaxed(bar, p.add(CA15BAR));
        writel_relaxed(bar | SBAR_BAREN, p.add(CA15BAR));
        // de-assert reset for CA15 CPUs
        writel_relaxed((readl_relaxed(p.add(CA15RESCNT)) & !CA15RESCNT_CPUS) | CA15RESCNT_CODE,
                       p.add(CA15RESCNT));
    }
    if has_a7 {
        writel_relaxed(bar, p.add(CA7BAR));
        writel_relaxed(bar | SBAR_BAREN, p.add(CA7BAR));
        // de-assert reset for CA7 CPUs
        writel_relaxed((readl_relaxed(p.add(CA7RESCNT)) & !CA7RESCNT_CPUS) | CA7RESCNT_CODE,
                       p.add(CA7RESCNT));
    }
    iounmap(p);

    shmobile_smp_apmu_suspend_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
