/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2005-2007 Cavium Networks
 */

// Linux and Octeon dependencies are supplied by the surrounding translation.

pub static mut cache_err_dcache: [u64; NR_CPUS as usize] = [0; NR_CPUS as usize];

unsafe fn octeon_flush_data_cache_page(_addr: usize) {}

#[inline]
unsafe fn octeon_local_flush_icache() {
    core::arch::asm!("synci 0($0)");
}

unsafe fn local_octeon_flush_icache_range(_start: usize, _end: usize) {
    octeon_local_flush_icache();
}

unsafe fn octeon_flush_icache_all_cores(vma: *mut vm_area_struct) {
    unsafe extern "C" {
        fn octeon_send_ipi_single(cpu: i32, action: u32);
    }

    mb();
    octeon_local_flush_icache();

    #[cfg(CONFIG_SMP)]
    {
        preempt_disable();
        let cpu = smp_processor_id();
        let mut mask: cpumask_t;
        if !vma.is_null() {
            mask = *mm_cpumask((*vma).vm_mm);
        } else {
            mask = *cpu_online_mask;
        }
        cpumask_clear_cpu(cpu, &mut mask);
        #[cfg(CONFIG_CAVIUM_OCTEON_SOC)]
        for_each_cpu!(cpu, &mask, {
            octeon_send_ipi_single(cpu, SMP_ICACHE_FLUSH);
        });
        #[cfg(not(CONFIG_CAVIUM_OCTEON_SOC))]
        smp_call_function_many(&mask, octeon_local_flush_icache, core::ptr::null_mut(), 1);
        preempt_enable();
    }
}

unsafe fn octeon_flush_icache_all() {
    octeon_flush_icache_all_cores(core::ptr::null_mut());
}

unsafe fn octeon_flush_cache_mm(_mm: *mut mm_struct) {}

unsafe fn octeon_flush_icache_range(_start: usize, _end: usize) {
    octeon_flush_icache_all_cores(core::ptr::null_mut());
}

unsafe fn octeon_flush_cache_range(vma: *mut vm_area_struct, _start: usize, _end: usize) {
    if (*vma).vm_flags & VM_EXEC != 0 {
        octeon_flush_icache_all_cores(vma);
    }
}

unsafe fn octeon_flush_cache_page(vma: *mut vm_area_struct, _page: usize, _pfn: usize) {
    if (*vma).vm_flags & VM_EXEC != 0 {
        octeon_flush_icache_all_cores(vma);
    }
}

unsafe fn octeon_flush_kernel_vmap_range(_vaddr: usize, _size: i32) {
    BUG();
}

unsafe fn probe_octeon() {
    let mut icache_size: usize;
    let mut dcache_size: usize;
    let config1 = read_c0_config1();
    let c = &mut current_cpu_data;
    let cputype = current_cpu_type();

    match cputype {
        CPU_CAVIUM_OCTEON | CPU_CAVIUM_OCTEON_PLUS => {
            c.icache.linesz = 2 << ((config1 >> 19) & 7);
            c.icache.sets = 64 << ((config1 >> 22) & 7);
            c.icache.ways = 1 + ((config1 >> 16) & 7);
            c.icache.flags |= MIPS_CACHE_VTAG;
            icache_size = c.icache.sets * c.icache.ways * c.icache.linesz;
            c.icache.waybit = ffs(icache_size / c.icache.ways) - 1;
            c.dcache.linesz = 128;
            c.dcache.sets = if cputype == CPU_CAVIUM_OCTEON_PLUS { 2 } else { 1 };
            c.dcache.ways = 64;
            dcache_size = c.dcache.sets * c.dcache.ways * c.dcache.linesz;
            c.dcache.waybit = ffs(dcache_size / c.dcache.ways) - 1;
            c.options |= MIPS_CPU_PREFETCH;
        }
        CPU_CAVIUM_OCTEON2 => {
            c.icache.linesz = 2 << ((config1 >> 19) & 7);
            c.icache.sets = 8; c.icache.ways = 37; c.icache.flags |= MIPS_CACHE_VTAG;
            icache_size = c.icache.sets * c.icache.ways * c.icache.linesz;
            c.dcache.linesz = 128; c.dcache.ways = 32; c.dcache.sets = 8;
            dcache_size = c.dcache.sets * c.dcache.ways * c.dcache.linesz;
            c.options |= MIPS_CPU_PREFETCH;
        }
        CPU_CAVIUM_OCTEON3 => {
            c.icache.linesz = 128; c.icache.sets = 16; c.icache.ways = 39;
            c.icache.flags |= MIPS_CACHE_VTAG;
            icache_size = c.icache.sets * c.icache.ways * c.icache.linesz;
            c.dcache.linesz = 128; c.dcache.ways = 32; c.dcache.sets = 8;
            dcache_size = c.dcache.sets * c.dcache.ways * c.dcache.linesz;
            c.options |= MIPS_CPU_PREFETCH;
        }
        _ => panic!("Unsupported Cavium Networks CPU type"),
    }
    c.icache.waysize = icache_size / c.icache.ways;
    c.dcache.waysize = dcache_size / c.dcache.ways;
    c.icache.sets = icache_size / (c.icache.linesz * c.icache.ways);
    c.dcache.sets = dcache_size / (c.dcache.linesz * c.dcache.ways);
    if smp_processor_id() == 0 {
        pr_info!("Primary instruction cache %ldkB, %s, %d way, %d sets, linesize %d bytes.\n", icache_size >> 10, if cpu_has_vtag_icache { "virtually tagged" } else { "physically tagged" }, c.icache.ways, c.icache.sets, c.icache.linesz);
        pr_info!("Primary data cache %ldkB, %d-way, %d sets, linesize %d bytes.\n", dcache_size >> 10, c.dcache.ways, c.dcache.sets, c.dcache.linesz);
    }
}

unsafe fn octeon_cache_error_setup() {
    unsafe extern "C" { static mut except_vec2_octeon: u8; }
    set_handler(0x100, &raw mut except_vec2_octeon, 0x80);
}

pub unsafe fn octeon_cache_init() {
    probe_octeon();
    shm_align_mask = PAGE_SIZE - 1;
    flush_cache_all = octeon_flush_icache_all;
    __flush_cache_all = octeon_flush_icache_all;
    flush_cache_mm = octeon_flush_cache_mm;
    flush_cache_page = octeon_flush_cache_page;
    flush_cache_range = octeon_flush_cache_range;
    flush_icache_all = octeon_flush_icache_all;
    flush_data_cache_page = octeon_flush_data_cache_page;
    flush_icache_range = octeon_flush_icache_range;
    local_flush_icache_range = local_octeon_flush_icache_range;
    __flush_icache_user_range = octeon_flush_icache_range;
    __local_flush_icache_user_range = local_octeon_flush_icache_range;
    __flush_kernel_vmap_range = octeon_flush_kernel_vmap_range;
    build_clear_page();
    build_copy_page();
    board_cache_error_setup = octeon_cache_error_setup;
}

static mut co_cache_error_chain: RAW_NOTIFIER_HEAD = RAW_NOTIFIER_HEAD;

pub unsafe fn register_co_cache_error_notifier(nb: *mut notifier_block) -> i32 {
    raw_notifier_chain_register(&raw mut co_cache_error_chain, nb)
}

pub unsafe fn unregister_co_cache_error_notifier(nb: *mut notifier_block) -> i32 {
    raw_notifier_chain_unregister(&raw mut co_cache_error_chain, nb)
}

unsafe fn co_cache_error_call_notifiers(val: usize) {
    let rv = raw_notifier_call_chain(&raw mut co_cache_error_chain, val, core::ptr::null_mut());
    if rv & !NOTIFY_STOP_MASK != NOTIFY_OK {
        let coreid = cvmx_get_core_num();
        let icache_err = read_octeon_c0_icacheerr();
        let dcache_err = if val != 0 { let e = cache_err_dcache[coreid]; cache_err_dcache[coreid] = 0; e } else { read_octeon_c0_dcacheerr() };
        pr_err!("Core%lu: Cache error exception:\n", coreid);
        pr_err!("cp0_errorepc == %lx\n", read_c0_errorepc());
        if icache_err & 1 != 0 { pr_err!("CacheErr (Icache) == %llx\n", icache_err); write_octeon_c0_icacheerr(0); }
        if dcache_err & 1 != 0 { pr_err!("CacheErr (Dcache) == %llx\n", dcache_err); }
    }
}

pub unsafe extern "C" fn cache_parity_error_octeon_recoverable() { co_cache_error_call_notifiers(0); }

pub unsafe extern "C" fn cache_parity_error_octeon_non_recoverable() {
    co_cache_error_call_notifiers(1);
    panic!("Can't handle cache error: nested exception");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
