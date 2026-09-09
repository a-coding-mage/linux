// SPDX-License-Identifier: GPL-2.0-only
/*
 * Spin Table SMP initialisation
 *
 * Copyright (C) 2013 ARM Ltd.
 */

// Linux and architecture dependencies supplied by other translation units.

extern "C" {
    fn secondary_holding_pen();
}

#[no_mangle]
#[link_section = ".mmuoff.data.read"]
pub static mut secondary_holding_pen_release: ::core::ffi::c_ulong = INVALID_HWID;

static mut cpu_release_addr: [phys_addr_t; NR_CPUS] = [0; NR_CPUS];

/*
 * Write secondary_holding_pen_release in a way that is guaranteed to be
 * visible to all observers, irrespective of whether they're taking part
 * in coherency or not.  This is necessary for the hotplug code to work
 * reliably.
 */
unsafe fn write_pen_release(val: u64) {
    let start = &secondary_holding_pen_release as *const _ as *mut core::ffi::c_void;
    let size = core::mem::size_of::<core::ffi::c_ulong>();

    secondary_holding_pen_release = val as core::ffi::c_ulong;
    dcache_clean_inval_poc(start as core::ffi::c_ulong,
                            start as core::ffi::c_ulong + size as core::ffi::c_ulong);
}

unsafe fn smp_spin_table_cpu_init(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let mut dn: *mut device_node;
    let mut ret: ::core::ffi::c_int;

    dn = of_get_cpu_node(cpu, core::ptr::null_mut());
    if dn.is_null() {
        return -ENODEV;
    }

    /*
     * Determine the address from which the CPU is polling.
     */
    ret = of_property_read_u64(
        dn,
        b"cpu-release-addr\0".as_ptr() as *const core::ffi::c_char,
        &mut cpu_release_addr[cpu as usize],
    );
    if ret != 0 {
        pr_err(b"CPU %d: missing or invalid cpu-release-addr property\n\0".as_ptr() as *const core::ffi::c_char, cpu);
    }

    of_node_put(dn);

    ret
}

unsafe fn smp_spin_table_cpu_prepare(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let release_addr: *mut u64;
    let pa_holding_pen: phys_addr_t = __pa_symbol(secondary_holding_pen as *const ());

    if cpu_release_addr[cpu as usize] == 0 {
        return -ENODEV;
    }

    /*
     * The cpu-release-addr may or may not be inside the linear mapping.
     * As ioremap_cache will either give us a new mapping or reuse the
     * existing linear mapping, we can use it to cover both cases. In
     * either case the memory will be MT_NORMAL.
     */
    release_addr = ioremap_cache(
        cpu_release_addr[cpu as usize],
        core::mem::size_of::<u64>(),
    ) as *mut u64;
    if release_addr.is_null() {
        return -ENOMEM;
    }

    /*
     * We write the release address as LE regardless of the native
     * endianness of the kernel. Therefore, any boot-loaders that
     * read this address need to convert this address to the
     * boot-loader's endianness before jumping. This is mandated by
     * the boot protocol.
     */
    writeq_relaxed(pa_holding_pen, release_addr);
    dcache_clean_inval_poc(
        release_addr as ::core::ffi::c_ulong,
        release_addr as ::core::ffi::c_ulong + core::mem::size_of::<u64>() as ::core::ffi::c_ulong,
    );

    /*
     * Send an event to wake up the secondary CPU.
     */
    sev();

    iounmap(release_addr as *mut core::ffi::c_void);

    0
}

unsafe fn smp_spin_table_cpu_boot(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    /*
     * Update the pen release flag.
     */
    write_pen_release(cpu_logical_map(cpu));

    /*
     * Send an event, causing the secondaries to read pen_release.
     */
    sev();

    0
}

#[repr(C)]
pub struct cpu_operations {
    pub name: *const core::ffi::c_char,
    pub cpu_init: Option<unsafe fn(::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub cpu_prepare: Option<unsafe fn(::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub cpu_boot: Option<unsafe fn(::core::ffi::c_uint) -> ::core::ffi::c_int>,
}

#[no_mangle]
pub static smp_spin_table_ops: cpu_operations = cpu_operations {
    name: b"spin-table\0".as_ptr() as *const core::ffi::c_char,
    cpu_init: Some(smp_spin_table_cpu_init),
    cpu_prepare: Some(smp_spin_table_cpu_prepare),
    cpu_boot: Some(smp_spin_table_cpu_boot),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
