// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Regents of the University of California
 */

// Linux and RISC-V header dependencies are supplied by the surrounding tree.

pub unsafe fn arch_match_cpu_phys_id(cpu: i32, phys_id: u64) -> bool {
    phys_id == cpuid_to_hartid_map(cpu)
}

/*
 * Returns the hart ID of the given device tree node, or -ENODEV if the node
 * isn't an enabled and valid RISC-V hart node.
 */
pub unsafe fn riscv_of_processor_hartid(
    node: *mut device_node,
    hart: *mut libc::c_ulong,
) -> i32 {
    let cpu: i32;

    *hart = of_get_cpu_hwid(node, 0) as libc::c_ulong;
    if *hart == !0 as libc::c_ulong {
        pr_warn!("Found CPU without hart ID\n");
        return -ENODEV;
    }

    cpu = riscv_hartid_to_cpuid(*hart);
    if cpu < 0 {
        return cpu;
    }

    if !cpu_possible(cpu) {
        return -ENODEV;
    }

    0
}

pub unsafe fn riscv_early_of_processor_hartid(
    node: *mut device_node,
    hart: *mut libc::c_ulong,
) -> i32 {
    let isa: *const libc::c_char;

    if !of_device_is_compatible(node, c"riscv".as_ptr()) {
        pr_warn!("Found incompatible CPU\n");
        return -ENODEV;
    }

    *hart = of_get_cpu_hwid(node, 0) as libc::c_ulong;
    if *hart == !0 as libc::c_ulong {
        pr_warn!("Found CPU without hart ID\n");
        return -ENODEV;
    }

    if !of_device_is_available(node) {
        return -ENODEV;
    }

    if of_property_read_string(node, c"riscv,isa-base".as_ptr(), &mut isa) != 0 {
        return riscv_early_of_processor_hartid_old_interface(node, hart);
    }

    if cfg!(CONFIG_32BIT) && strncasecmp(isa, c"rv32i".as_ptr(), 5) != 0 {
        pr_warn!("CPU with hartid=%lu does not support rv32i", *hart);
        return -ENODEV;
    }
    if cfg!(CONFIG_64BIT) && strncasecmp(isa, c"rv64i".as_ptr(), 5) != 0 {
        pr_warn!("CPU with hartid=%lu does not support rv64i", *hart);
        return -ENODEV;
    }
    if !of_property_present(node, c"riscv,isa-extensions".as_ptr()) {
        return -ENODEV;
    }
    if of_property_match_string(node, c"riscv,isa-extensions".as_ptr(), c"i".as_ptr()) < 0
        || of_property_match_string(node, c"riscv,isa-extensions".as_ptr(), c"m".as_ptr()) < 0
        || of_property_match_string(node, c"riscv,isa-extensions".as_ptr(), c"a".as_ptr()) < 0
    {
        pr_warn!("CPU with hartid=%lu does not support ima", *hart);
        return -ENODEV;
    }
    0
}

unsafe fn riscv_early_of_processor_hartid_old_interface(
    node: *mut device_node,
    hart: *mut libc::c_ulong,
) -> i32 {
    let isa: *const libc::c_char;
    if !riscv_isa_fallback {
        pr_warn!("CPU with hartid=%lu is invalid: this kernel does not parse \"riscv,isa\"", *hart);
        return -ENODEV;
    }
    if of_property_read_string(node, c"riscv,isa".as_ptr(), &mut isa) != 0 {
        pr_warn!("CPU with hartid=%lu has no \"riscv,isa-base\" or \"riscv,isa\" property\n", *hart);
        return -ENODEV;
    }
    if cfg!(CONFIG_32BIT) && strncasecmp(isa, c"rv32ima".as_ptr(), 7) != 0 {
        pr_warn!("CPU with hartid=%lu does not support rv32ima", *hart);
        return -ENODEV;
    }
    if cfg!(CONFIG_64BIT) && strncasecmp(isa, c"rv64ima".as_ptr(), 7) != 0 {
        pr_warn!("CPU with hartid=%lu does not support rv64ima", *hart);
        return -ENODEV;
    }
    0
}

pub unsafe fn riscv_of_parent_hartid(
    mut node: *mut device_node,
    hartid: *mut libc::c_ulong,
) -> i32 {
    while !node.is_null() {
        if of_device_is_compatible(node, c"riscv".as_ptr()) {
            *hartid = of_get_cpu_hwid(node, 0) as libc::c_ulong;
            if *hartid == !0 as libc::c_ulong {
                pr_warn!("Found CPU without hart ID\n");
                return -ENODEV;
            }
            return 0;
        }
        node = (*node).parent;
    }
    -1
}

pub unsafe fn riscv_get_marchid() -> libc::c_ulong {
    let ci = this_cpu_ptr(&mut riscv_cpuinfo);
    #[cfg(CONFIG_RISCV_SBI)]
    { (*ci).marchid = if sbi_spec_is_0_1() { 0 } else { sbi_get_marchid() }; }
    #[cfg(all(not(CONFIG_RISCV_SBI), CONFIG_RISCV_M_MODE))]
    { (*ci).marchid = csr_read(CSR_MARCHID); }
    #[cfg(not(any(CONFIG_RISCV_SBI, CONFIG_RISCV_M_MODE)))]
    { (*ci).marchid = 0; }
    (*ci).marchid
}

pub unsafe fn riscv_get_mvendorid() -> libc::c_ulong {
    let ci = this_cpu_ptr(&mut riscv_cpuinfo);
    #[cfg(CONFIG_RISCV_SBI)]
    { (*ci).mvendorid = if sbi_spec_is_0_1() { 0 } else { sbi_get_mvendorid() }; }
    #[cfg(all(not(CONFIG_RISCV_SBI), CONFIG_RISCV_M_MODE))]
    { (*ci).mvendorid = csr_read(CSR_MVENDORID); }
    #[cfg(not(any(CONFIG_RISCV_SBI, CONFIG_RISCV_M_MODE)))]
    { (*ci).mvendorid = 0; }
    (*ci).mvendorid
}

pub static mut riscv_cpuinfo: riscv_cpuinfo = riscv_cpuinfo::default();

pub unsafe fn riscv_cached_mvendorid(cpu_id: u32) -> libc::c_ulong {
    (*per_cpu_ptr(&mut riscv_cpuinfo, cpu_id)).mvendorid
}

pub unsafe fn riscv_cached_marchid(cpu_id: u32) -> libc::c_ulong {
    (*per_cpu_ptr(&mut riscv_cpuinfo, cpu_id)).marchid
}

pub unsafe fn riscv_cached_mimpid(cpu_id: u32) -> libc::c_ulong {
    (*per_cpu_ptr(&mut riscv_cpuinfo, cpu_id)).mimpid
}

unsafe fn riscv_cpuinfo_starting(_cpu: u32) -> i32 {
    let ci = this_cpu_ptr(&mut riscv_cpuinfo);
    #[cfg(CONFIG_RISCV_SBI)]
    {
        if (*ci).mvendorid == 0 { (*ci).mvendorid = if sbi_spec_is_0_1() { 0 } else { sbi_get_mvendorid() }; }
        if (*ci).marchid == 0 { (*ci).marchid = if sbi_spec_is_0_1() { 0 } else { sbi_get_marchid() }; }
        (*ci).mimpid = if sbi_spec_is_0_1() { 0 } else { sbi_get_mimpid() };
    }
    #[cfg(all(not(CONFIG_RISCV_SBI), CONFIG_RISCV_M_MODE))]
    {
        if (*ci).mvendorid == 0 { (*ci).mvendorid = csr_read(CSR_MVENDORID); }
        if (*ci).marchid == 0 { (*ci).marchid = csr_read(CSR_MARCHID); }
        (*ci).mimpid = csr_read(CSR_MIMPID);
    }
    #[cfg(not(any(CONFIG_RISCV_SBI, CONFIG_RISCV_M_MODE)))]
    { (*ci).mvendorid = 0; (*ci).marchid = 0; (*ci).mimpid = 0; }
    0
}

unsafe fn riscv_cpuinfo_init() -> i32 {
    let ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, c"riscv/cpuinfo:starting".as_ptr(), riscv_cpuinfo_starting, None);
    if ret < 0 {
        pr_err!("cpuinfo: failed to register hotplug callbacks.\n");
        return ret;
    }
    0
}

// arch_initcall(riscv_cpuinfo_init);

#[cfg(CONFIG_PROC_FS)]
mod proc_fs {
    use super::*;
    const ALL_CPUS: i32 = -1;

    unsafe fn print_vendor_isa(f: *mut seq_file, cpu: i32) {
        for i in 0..riscv_isa_vendor_ext_list_size {
            let ext_list = riscv_isa_vendor_ext_list[i];
            let vendor_bitmap = if cpu == ALL_CPUS { &(*ext_list).all_harts_isa_bitmap } else { &(*ext_list).per_hart_isa_bitmap[cpu as usize] };
            for j in 0..(*ext_list).ext_data_count {
                let ext_data = (*ext_list).ext_data.add(j);
                if !__riscv_isa_extension_available(vendor_bitmap.isa, (*ext_data).id) { continue; }
                seq_printf(f, c"_%s".as_ptr(), (*ext_data).name);
            }
        }
    }

    unsafe fn print_isa(f: *mut seq_file, isa_bitmap: *const libc::c_ulong, cpu: i32) {
        if cfg!(CONFIG_32BIT) { seq_write(f, c"rv32".as_ptr(), 4); } else { seq_write(f, c"rv64".as_ptr(), 4); }
        for i in 0..riscv_isa_ext_count {
            if !__riscv_isa_extension_available(isa_bitmap, riscv_isa_ext[i].id) { continue; }
            if strnlen(riscv_isa_ext[i].name, 2) != 1 { seq_puts(f, c"_".as_ptr()); }
            seq_printf(f, c"%s".as_ptr(), riscv_isa_ext[i].name);
        }
        print_vendor_isa(f, cpu);
        seq_puts(f, c"\n".as_ptr());
    }

    unsafe fn print_mmu(f: *mut seq_file) {
        let sv_type = if !cfg!(CONFIG_MMU) { c"none".as_ptr() } else if cfg!(CONFIG_32BIT) { c"sv32".as_ptr() } else if pgtable_l5_enabled { c"sv57".as_ptr() } else if pgtable_l4_enabled { c"sv48".as_ptr() } else { c"sv39".as_ptr() };
        seq_printf(f, c"mmu\t\t: %s\n".as_ptr(), sv_type);
    }

    unsafe fn c_start(_m: *mut seq_file, pos: *mut loff_t) -> *mut libc::c_void {
        if *pos == nr_cpu_ids { return core::ptr::null_mut(); }
        *pos = cpumask_next(*pos - 1, cpu_online_mask);
        if *pos < nr_cpu_ids { (1 + *pos as usize) as *mut libc::c_void } else { core::ptr::null_mut() }
    }
    unsafe fn c_next(m: *mut seq_file, _v: *mut libc::c_void, pos: *mut loff_t) -> *mut libc::c_void { *pos += 1; c_start(m, pos) }
    unsafe fn c_stop(_m: *mut seq_file, _v: *mut libc::c_void) {}

    unsafe fn c_show(m: *mut seq_file, v: *mut libc::c_void) -> i32 {
        let cpu_id = v as usize - 1;
        let ci = per_cpu_ptr(&mut riscv_cpuinfo, cpu_id as u32);
        seq_printf(m, c"processor\t: %lu\n".as_ptr(), cpu_id);
        seq_printf(m, c"hart\t\t: %lu\n".as_ptr(), cpuid_to_hartid_map(cpu_id as i32));
        seq_puts(m, c"isa\t\t: ".as_ptr()); print_isa(m, core::ptr::null(), ALL_CPUS); print_mmu(m);
        if acpi_disabled {
            let node = of_get_cpu_node(cpu_id as u32, core::ptr::null_mut());
            let mut compat = core::ptr::null();
            if !of_property_read_string(node, c"compatible".as_ptr(), &mut compat) && strcmp(compat, c"riscv".as_ptr()) != 0 { seq_printf(m, c"uarch\t\t: %s\n".as_ptr(), compat); }
            of_node_put(node);
        }
        seq_printf(m, c"mvendorid\t: 0x%lx\n".as_ptr(), (*ci).mvendorid);
        seq_printf(m, c"marchid\t\t: 0x%lx\n".as_ptr(), (*ci).marchid);
        seq_printf(m, c"mimpid\t\t: 0x%lx\n".as_ptr(), (*ci).mimpid);
        seq_puts(m, c"hart isa\t: ".as_ptr()); print_isa(m, hart_isa[cpu_id].isa, cpu_id as i32); seq_puts(m, c"\n".as_ptr());
        0
    }

    // const struct seq_operations cpuinfo_op = { .start = c_start, .next = c_next, .stop = c_stop, .show = c_show };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
