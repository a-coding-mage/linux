// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2013 Altera Corporation
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 *
 * Based on cpuinfo.c from microblaze
 */

// Kernel and architecture declarations are supplied by the surrounding build.

pub static mut cpuinfo: cpuinfo = unsafe { core::mem::zeroed() };

#[inline]
unsafe fn fcpu(cpu: *mut device_node, n: *const core::ffi::c_char) -> u32 {
    let mut val: u32 = 0;
    of_property_read_u32(cpu, n, &mut val);
    val
}

pub unsafe fn setup_cpuinfo() {
    let cpu: *mut device_node;
    let str_: *const core::ffi::c_char;
    let mut len: core::ffi::c_int;

    cpu = of_get_cpu_node(0, core::ptr::null());
    if cpu.is_null() {
        panic!("{}: No CPU found in devicetree!\n", "setup_cpuinfo");
    }

    if !of_property_read_bool(cpu, c"altr,has-initda".as_ptr()) {
        panic!("initda instruction is unimplemented. Please update your hardware system to have more than 4-byte line data cache\n");
    }

    cpuinfo.cpu_clock_freq = fcpu(cpu, c"clock-frequency".as_ptr());

    str_ = of_get_property(cpu, c"altr,implementation".as_ptr(), &mut len);
    strscpy(cpuinfo.cpu_impl.as_mut_ptr(), if str_.is_null() { c"<unknown>".as_ptr() } else { str_ });

    cpuinfo.has_div = of_property_read_bool(cpu, c"altr,has-div".as_ptr());
    cpuinfo.has_mul = of_property_read_bool(cpu, c"altr,has-mul".as_ptr());
    cpuinfo.has_mulx = of_property_read_bool(cpu, c"altr,has-mulx".as_ptr());
    cpuinfo.has_bmx = of_property_read_bool(cpu, c"altr,has-bmx".as_ptr());
    cpuinfo.has_cdx = of_property_read_bool(cpu, c"altr,has-cdx".as_ptr());
    cpuinfo.mmu = of_property_read_bool(cpu, c"altr,has-mmu".as_ptr());

    if cfg!(CONFIG_NIOS2_HW_DIV_SUPPORT) && !cpuinfo.has_div { pr_err!("ERROR: Nios II DIV different for kernel and DTS\n"); }
    if cfg!(CONFIG_NIOS2_HW_MUL_SUPPORT) && !cpuinfo.has_mul { pr_err!("ERROR: Nios II MUL different for kernel and DTS\n"); }
    if cfg!(CONFIG_NIOS2_HW_MULX_SUPPORT) && !cpuinfo.has_mulx { pr_err!("ERROR: Nios II MULX different for kernel and DTS\n"); }
    if cfg!(CONFIG_NIOS2_BMX_SUPPORT) && !cpuinfo.has_bmx { pr_err!("ERROR: Nios II BMX different for kernel and DTS\n"); }
    if cfg!(CONFIG_NIOS2_CDX_SUPPORT) && !cpuinfo.has_cdx { pr_err!("ERROR: Nios II CDX different for kernel and DTS\n"); }

    cpuinfo.tlb_num_ways = fcpu(cpu, c"altr,tlb-num-ways".as_ptr());
    if cpuinfo.tlb_num_ways == 0 { panic!("altr,tlb-num-ways can't be 0. Please check your hardware system\n"); }
    cpuinfo.icache_line_size = fcpu(cpu, c"icache-line-size".as_ptr());
    cpuinfo.icache_size = fcpu(cpu, c"icache-size".as_ptr());
    if CONFIG_NIOS2_ICACHE_SIZE != cpuinfo.icache_size { pr_warn!("Warning: icache size configuration mismatch (0x%x vs 0x%x) of CONFIG_NIOS2_ICACHE_SIZE vs device tree icache-size\n", CONFIG_NIOS2_ICACHE_SIZE, cpuinfo.icache_size); }
    cpuinfo.dcache_line_size = fcpu(cpu, c"dcache-line-size".as_ptr());
    if CONFIG_NIOS2_DCACHE_LINE_SIZE != cpuinfo.dcache_line_size { pr_warn!("Warning: dcache line size configuration mismatch (0x%x vs 0x%x) of CONFIG_NIOS2_DCACHE_LINE_SIZE vs device tree dcache-line-size\n", CONFIG_NIOS2_DCACHE_LINE_SIZE, cpuinfo.dcache_line_size); }
    cpuinfo.dcache_size = fcpu(cpu, c"dcache-size".as_ptr());
    if CONFIG_NIOS2_DCACHE_SIZE != cpuinfo.dcache_size { pr_warn!("Warning: dcache size configuration mismatch (0x%x vs 0x%x) of CONFIG_NIOS2_DCACHE_SIZE vs device tree dcache-size\n", CONFIG_NIOS2_DCACHE_SIZE, cpuinfo.dcache_size); }

    cpuinfo.tlb_pid_num_bits = fcpu(cpu, c"altr,pid-num-bits".as_ptr());
    cpuinfo.tlb_num_ways_log2 = ilog2(cpuinfo.tlb_num_ways);
    cpuinfo.tlb_num_entries = fcpu(cpu, c"altr,tlb-num-entries".as_ptr());
    cpuinfo.tlb_num_lines = cpuinfo.tlb_num_entries / cpuinfo.tlb_num_ways;
    cpuinfo.tlb_ptr_sz = fcpu(cpu, c"altr,tlb-ptr-sz".as_ptr());
    cpuinfo.reset_addr = fcpu(cpu, c"altr,reset-addr".as_ptr());
    cpuinfo.exception_addr = fcpu(cpu, c"altr,exception-addr".as_ptr());
    cpuinfo.fast_tlb_miss_exc_addr = fcpu(cpu, c"altr,fast-tlb-miss-addr".as_ptr());
    of_node_put(cpu);
}

// CONFIG_PROC_FS declarations and definitions are retained below when enabled.
#[cfg(CONFIG_PROC_FS)]
pub unsafe fn show_cpuinfo(m: *mut seq_file, _v: *mut core::ffi::c_void) -> core::ffi::c_int {
    seq_printf(m, c"CPU:\t\tNios II/%s\nREV:\t\t%i\nMMU:\t\t%s\nFPU:\t\tnone\nClocking:\t%u.%02u MHz\nBogoMips:\t%lu.%02lu\nCalibration:\t%lu loops\n".as_ptr(), cpuinfo.cpu_impl.as_ptr(), CONFIG_NIOS2_ARCH_REVISION, if cpuinfo.mmu { c"present".as_ptr() } else { c"none".as_ptr() }, cpuinfo.cpu_clock_freq / 1_000_000, (cpuinfo.cpu_clock_freq / 100_000) % 10, (loops_per_jiffy * HZ) / 500_000, ((loops_per_jiffy * HZ) / 5_000) % 100, loops_per_jiffy * HZ);
    seq_printf(m, c"HW:\n MUL:\t\t%s\n MULX:\t\t%s\n DIV:\t\t%s\n BMX:\t\t%s\n CDX:\t\t%s\n".as_ptr(), str_yes_no(cpuinfo.has_mul), str_yes_no(cpuinfo.has_mulx), str_yes_no(cpuinfo.has_div), str_yes_no(cpuinfo.has_bmx), str_yes_no(cpuinfo.has_cdx));
    seq_printf(m, c"Icache:\t\t%ukB, line length: %u\n".as_ptr(), cpuinfo.icache_size >> 10, cpuinfo.icache_line_size);
    seq_printf(m, c"Dcache:\t\t%ukB, line length: %u\n".as_ptr(), cpuinfo.dcache_size >> 10, cpuinfo.dcache_line_size);
    seq_printf(m, c"TLB:\t\t%u ways, %u entries, %u PID bits\n".as_ptr(), cpuinfo.tlb_num_ways, cpuinfo.tlb_num_entries, cpuinfo.tlb_pid_num_bits);
    0
}

#[cfg(CONFIG_PROC_FS)]
pub unsafe fn cpuinfo_start(_m: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let i = *pos as usize;
    if i < num_possible_cpus() { (i + 1) as *mut core::ffi::c_void } else { core::ptr::null_mut() }
}

#[cfg(CONFIG_PROC_FS)]
pub unsafe fn cpuinfo_next(m: *mut seq_file, _v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    *pos += 1;
    cpuinfo_start(m, pos)
}

#[cfg(CONFIG_PROC_FS)]
pub unsafe fn cpuinfo_stop(_m: *mut seq_file, _v: *mut core::ffi::c_void) {}

#[cfg(CONFIG_PROC_FS)]
pub static cpuinfo_op: seq_operations = seq_operations { start: Some(cpuinfo_start), next: Some(cpuinfo_next), stop: Some(cpuinfo_stop), show: Some(show_cpuinfo) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
