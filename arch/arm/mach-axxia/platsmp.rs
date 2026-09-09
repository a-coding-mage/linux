// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-axxia/platsmp.c
 *
 * Copyright (C) 2012 LSI Corporation
 */

// Kernel dependencies supplied by other translation units.

/* Syscon register offsets for releasing cores from reset */
const SC_CRIT_WRITE_KEY: usize = 0x1000;
const SC_RST_CPU_HOLD: usize = 0x1010;

extern "C" {
    static secondary_startup: unsafe extern "C" fn();
    fn phys_to_virt(release_phys: u32) -> *mut u32;
    fn __pa_symbol(symbol: unsafe extern "C" fn()) -> u32;
    fn writel_relaxed(value: u32, address: *mut u32);
    fn smp_wmb();
    fn __cpuc_flush_dcache_area(address: *mut core::ffi::c_void, size: usize);

    fn of_find_compatible_node(
        from: *mut device_node,
        typ: *const core::ffi::c_char,
        compatible: *const core::ffi::c_char,
    ) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: i32) -> *mut u8;
    fn of_node_put(node: *mut device_node);
    fn readl(address: *mut u8) -> u32;
    fn writel(value: u32, address: *mut u8);
    fn of_get_cpu_node(cpu: i32, thread: *mut i32) -> *mut device_node;
    fn of_property_read_u32(node: *mut device_node, name: *const core::ffi::c_char, value: *mut u32) -> i32;
    fn set_cpu_present(cpu: i32, present: bool);
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct smp_operations {
    pub smp_prepare_cpus: Option<unsafe extern "C" fn(max_cpus: u32)>,
    pub smp_boot_secondary:
        Option<unsafe extern "C" fn(cpu: u32, idle: *mut task_struct) -> i32>,
}

const ENOENT: i32 = 2;
const ENOMEM: i32 = 12;

/*
 * Write the kernel entry point for secondary CPUs to the specified address
 */
unsafe extern "C" fn write_release_addr(release_phys: u32) {
    let virt: *mut u32 = phys_to_virt(release_phys);
    writel_relaxed(__pa_symbol(secondary_startup), virt);
    /* Make sure this store is visible to other CPUs */
    smp_wmb();
    __cpuc_flush_dcache_area(virt.cast(), core::mem::size_of::<u32>());
}

unsafe extern "C" fn axxia_boot_secondary(
    cpu: u32,
    _idle: *mut task_struct,
) -> i32 {
    let syscon_np: *mut device_node = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        c"lsi,axxia-syscon".as_ptr(),
    );
    if syscon_np.is_null() {
        return -ENOENT;
    }

    let syscon: *mut u8 = of_iomap(syscon_np, 0);
    of_node_put(syscon_np);
    if syscon.is_null() {
        return -ENOMEM;
    }

    let mut tmp: u32 = readl(syscon.add(SC_RST_CPU_HOLD));
    writel(0xab, syscon.add(SC_CRIT_WRITE_KEY));
    tmp &= !(1u32 << cpu);
    writel(tmp, syscon.add(SC_RST_CPU_HOLD));

    0
}

unsafe extern "C" fn axxia_smp_prepare_cpus(max_cpus: u32) {
    let mut cpu_count: i32 = 0;
    let mut cpu: i32 = 0;

    /*
     * Initialise the present map, which describes the set of CPUs actually
     * populated at the present time.
     */
    // for_each_possible_cpu(cpu)
    while cpu < num_possible_cpus() {
        let np: *mut device_node;
        let mut release_phys: u32 = 0;

        np = of_get_cpu_node(cpu, core::ptr::null_mut());
        if np.is_null() {
            cpu += 1;
            continue;
        }
        if of_property_read_u32(np, c"cpu-release-addr".as_ptr(), &mut release_phys) != 0 {
            cpu += 1;
            continue;
        }

        if cpu_count < max_cpus as i32 {
            set_cpu_present(cpu, true);
            cpu_count += 1;
        }

        if release_phys != 0 {
            write_release_addr(release_phys);
        }
        cpu += 1;
    }
}

extern "C" {
    fn num_possible_cpus() -> i32;
}

static axxia_smp_ops: smp_operations = smp_operations {
    smp_prepare_cpus: Some(axxia_smp_prepare_cpus),
    smp_boot_secondary: Some(axxia_boot_secondary),
};

// CPU_METHOD_OF_DECLARE(axxia_smp, "lsi,syscon-release", &axxia_smp_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
