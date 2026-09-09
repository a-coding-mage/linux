// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2014-2015 Broadcom Corporation
 * Copyright 2014 Linaro Limited
 */

/* Kernel dependencies are supplied by the surrounding tree. */

const CORTEX_A9_SCU_SIZE: usize = 0x58;
const SECONDARY_TIMEOUT_NS: u64 = NSEC_PER_MSEC;
const BOOT_ADDR_CPUID_MASK: u32 = 0x3;
const OF_SECONDARY_BOOT: *const u8 = b"secondary-boot-reg\0".as_ptr();
const MPIDR_CPUID_BITMASK: u32 = 0x3;

const CDC_CMD: u32 = 6;
const CDC_CMD_OFFSET: usize = 0;

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct TaskStruct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CpuMask {
    pub bits: [usize; 1],
}

#[repr(C)]
pub struct SmpOperations {
    pub smp_prepare_cpus: Option<unsafe extern "C" fn(max_cpus: u32)>,
    pub smp_boot_secondary:
        Option<unsafe extern "C" fn(cpu: u32, idle: *mut TaskStruct) -> i32>,
}

extern "C" {
    static mut secondary_startup: u8;
    static mut NSEC_PER_MSEC: u64;
    static mut CPU_BITS_CPU0: usize;
    static mut LOCAL_MAILBOX3_SET0: usize;

    fn scu_a9_has_base() -> bool;
    fn scu_a9_get_base() -> usize;
    fn ioremap(addr: usize, size: usize) -> *mut u8;
    fn iounmap(addr: *mut u8);
    fn scu_enable(base: *mut u8);
    fn pr_err(fmt: *const u8, ...);
    fn pr_warn(fmt: *const u8, ...);
    fn of_get_cpu_node(cpu: u32, thread: *mut u32) -> *mut DeviceNode;
    fn of_property_read_u32(node: *mut DeviceNode, name: *const u8, value: *mut u32) -> i32;
    fn of_node_put(node: *mut DeviceNode);
    fn __pa_symbol(symbol: *const u8) -> usize;
    fn BUG_ON(condition: bool);
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn smp_wmb();
    fn cpu_logical_map(cpu: u32) -> u32;
    fn sev();
    fn local_clock() -> u64;
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn of_find_compatible_node(from: *mut DeviceNode, typ: *const u8, compatible: *const u8) -> *mut DeviceNode;
    fn of_iomap(node: *mut DeviceNode, index: i32) -> *mut u8;
    fn arch_send_wakeup_ipi_mask(mask: *const CpuMask);
    fn cpumask_of(cpu: u32) -> *const CpuMask;
    fn writel(value: u32, addr: *mut u8);
    fn dsb(option: u32);
    fn virt_to_phys(addr: *const u8) -> u32;
    fn init_cpu_present(mask: *const CpuMask);
}

const ENXIO: i32 = 6;
const ENOENT: i32 = 2;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;
const ENODEV: i32 = 19;
const SY: u32 = 0;

unsafe extern "C" fn scu_a9_enable() -> i32 {
    let config_base: usize;
    let scu_base: *mut u8;

    if !scu_a9_has_base() {
        pr_err(b"no configuration base address register!\n\0".as_ptr());
        return -ENXIO;
    }

    config_base = scu_a9_get_base();
    if config_base == 0 {
        pr_err(b"hardware reports only one core\n\0".as_ptr());
        return -ENOENT;
    }

    scu_base = ioremap(config_base, CORTEX_A9_SCU_SIZE);
    if scu_base.is_null() {
        pr_err(b"failed to remap config base (%lu/%u) for SCU\n\0".as_ptr(), config_base, CORTEX_A9_SCU_SIZE);
        return -ENOMEM;
    }

    scu_enable(scu_base);
    iounmap(scu_base);
    0
}

unsafe extern "C" fn secondary_boot_addr_for(cpu: u32) -> u32 {
    let mut secondary_boot_addr: u32 = 0;
    let cpu_node = of_get_cpu_node(cpu, core::ptr::null_mut());
    if cpu_node.is_null() {
        pr_err(b"Failed to find device tree node for CPU%u\n\0".as_ptr(), cpu);
        return 0;
    }
    if of_property_read_u32(cpu_node, OF_SECONDARY_BOOT, &mut secondary_boot_addr) != 0 {
        pr_err(b"required secondary boot register not specified for CPU%u\n\0".as_ptr(), cpu);
    }
    of_node_put(cpu_node);
    secondary_boot_addr
}

unsafe extern "C" fn nsp_write_lut(cpu: u32) -> i32 {
    let secondary_boot_addr = secondary_boot_addr_for(cpu);
    if secondary_boot_addr == 0 { return -EINVAL; }
    let sku_rom_lut = ioremap(secondary_boot_addr as usize, core::mem::size_of::<usize>());
    if sku_rom_lut.is_null() {
        pr_warn(b"unable to ioremap SKU-ROM LUT register for cpu %u\n\0".as_ptr(), cpu);
        return -ENOMEM;
    }
    let secondary_startup_phy = __pa_symbol(&secondary_startup as *const u8);
    BUG_ON(secondary_startup_phy > u32::MAX as usize);
    writel_relaxed(secondary_startup_phy as u32, sku_rom_lut);
    smp_wmb();
    iounmap(sku_rom_lut);
    0
}

unsafe extern "C" fn bcm_smp_prepare_cpus(_max_cpus: u32) {
    let only_cpu_0 = CpuMask { bits: [CPU_BITS_CPU0] };
    if scu_a9_enable() != 0 {
        pr_warn(b"failed to enable A9 SCU - disabling SMP\n\0".as_ptr());
        init_cpu_present(&only_cpu_0);
    }
}

unsafe extern "C" fn kona_boot_secondary(cpu: u32, _idle: *mut TaskStruct) -> i32 {
    let secondary_boot_addr = secondary_boot_addr_for(cpu);
    let cpu_id = cpu_logical_map(cpu);
    if cpu_id & !BOOT_ADDR_CPUID_MASK != 0 {
        pr_err(b"bad cpu id (%u > %u)\n\0".as_ptr(), cpu_id, BOOT_ADDR_CPUID_MASK);
        return -EINVAL;
    }
    if secondary_boot_addr == 0 { return -EINVAL; }
    let boot_reg = ioremap(secondary_boot_addr as usize, core::mem::size_of::<usize>());
    if boot_reg.is_null() {
        pr_err(b"unable to map boot register for cpu %u\n\0".as_ptr(), cpu_id);
        return -ENOMEM;
    }
    let boot_func = __pa_symbol(&secondary_startup as *const u8);
    BUG_ON(boot_func & BOOT_ADDR_CPUID_MASK as usize != 0);
    BUG_ON(boot_func > u32::MAX as usize);
    let boot_val = boot_func as u32 | cpu_id;
    writel_relaxed(boot_val, boot_reg);
    sev();
    let start_clock = local_clock();
    let mut timeout = false;
    while !timeout && readl_relaxed(boot_reg) == boot_val {
        timeout = local_clock().wrapping_sub(start_clock) > SECONDARY_TIMEOUT_NS;
    }
    iounmap(boot_reg);
    if !timeout { return 0; }
    pr_err(b"timeout waiting for cpu %u to start\n\0".as_ptr(), cpu_id);
    -ENXIO
}

const fn cdc_cmd_reg(cpu: usize) -> usize { CDC_CMD_OFFSET + 4 * cpu }

unsafe extern "C" fn bcm23550_boot_secondary(cpu: u32, idle: *mut TaskStruct) -> i32 {
    let name = b"brcm,bcm23550-cdc\0";
    let dn = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), name.as_ptr());
    if dn.is_null() { pr_err(b"unable to find cdc node\n\0".as_ptr()); return -ENODEV; }
    let cdc_base = of_iomap(dn, 0);
    of_node_put(dn);
    if cdc_base.is_null() { pr_err(b"unable to remap cdc base register\n\0".as_ptr()); return -ENOMEM; }
    let ret = kona_boot_secondary(cpu, idle);
    if ret == 0 { writel_relaxed(CDC_CMD, cdc_base.add(cdc_cmd_reg(cpu as usize))); }
    iounmap(cdc_base);
    ret
}

unsafe extern "C" fn nsp_boot_secondary(cpu: u32, _idle: *mut TaskStruct) -> i32 {
    let ret = nsp_write_lut(cpu);
    if ret != 0 { pr_err(b"unable to write startup addr to SKU ROM LUT\n\0".as_ptr()); return ret; }
    arch_send_wakeup_ipi_mask(cpumask_of(cpu));
    ret
}

unsafe extern "C" fn bcm2836_boot_secondary(cpu: u32, _idle: *mut TaskStruct) -> i32 {
    let name = b"brcm,bcm2836-l1-intc\0";
    let dn = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), name.as_ptr());
    if dn.is_null() { pr_err(b"unable to find intc node\n\0".as_ptr()); return -ENODEV; }
    let intc_base = of_iomap(dn, 0);
    of_node_put(dn);
    if intc_base.is_null() { pr_err(b"unable to remap intc base register\n\0".as_ptr()); return -ENOMEM; }
    writel(virt_to_phys(&secondary_startup as *const u8), intc_base.add(LOCAL_MAILBOX3_SET0 + 16 * cpu as usize));
    dsb(SY);
    sev();
    iounmap(intc_base);
    0
}

#[used]
static kona_smp_ops: SmpOperations = SmpOperations { smp_prepare_cpus: Some(bcm_smp_prepare_cpus), smp_boot_secondary: Some(kona_boot_secondary) };
#[used]
static bcm23550_smp_ops: SmpOperations = SmpOperations { smp_prepare_cpus: None, smp_boot_secondary: Some(bcm23550_boot_secondary) };
#[used]
static nsp_smp_ops: SmpOperations = SmpOperations { smp_prepare_cpus: Some(bcm_smp_prepare_cpus), smp_boot_secondary: Some(nsp_boot_secondary) };
#[used]
pub static bcm2836_smp_ops: SmpOperations = SmpOperations { smp_prepare_cpus: None, smp_boot_secondary: Some(bcm2836_boot_secondary) };

// CPU_METHOD_OF_DECLARE registrations:
// bcm_smp_bcm281xx: "brcm,bcm11351-cpu-method" -> kona_smp_ops
// bcm_smp_bcm23550: "brcm,bcm23550" -> bcm23550_smp_ops
// bcm_smp_nsp: "brcm,bcm-nsp-smp" -> nsp_smp_ops
// bcm_smp_bcm2836: "brcm,bcm2836-smp" -> bcm2836_smp_ops

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
