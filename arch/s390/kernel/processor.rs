// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright IBM Corp. 2008
 *  Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
 */

// Dependencies are supplied by the surrounding kernel translation unit.

pub static mut elf_hwcap: c_ulong = 0;
pub static mut elf_platform: [c_char; ELF_PLATFORM_SIZE] = [0; ELF_PLATFORM_SIZE];

#[repr(C)]
pub struct cpu_info {
    pub cpu_mhz_dynamic: c_uint,
    pub cpu_mhz_static: c_uint,
    pub cpu_id: cpuid,
}

static mut cpu_info_per_cpu: PerCpu<cpu_info> = DEFINE_PER_CPU();
static mut cpu_relax_retry: PerCpu<c_int> = DEFINE_PER_CPU();
static mut machine_has_cpu_mhz: bool = false;

pub unsafe extern "C" fn cpu_detect_mhz_feature() {
    if test_facility(34) && __ecag(ECAG_CPU_ATTRIBUTE, 0) != !0UL {
        machine_has_cpu_mhz = true;
    }
}

unsafe extern "C" fn update_cpu_mhz(_arg: *mut c_void) {
    let mhz: c_ulong = __ecag(ECAG_CPU_ATTRIBUTE, 0);
    let c: *mut cpu_info = this_cpu_ptr(&raw mut cpu_info_per_cpu);
    (*c).cpu_mhz_dynamic = (mhz >> 32) as c_uint;
    (*c).cpu_mhz_static = (mhz & 0xffff_ffff) as c_uint;
}

pub unsafe extern "C" fn s390_update_cpu_mhz() {
    s390_adjust_jiffies();
    if machine_has_cpu_mhz {
        on_each_cpu(Some(update_cpu_mhz), core::ptr::null_mut(), 0);
    }
}

pub unsafe extern "C" fn stop_machine_yield(cpumask: *const cpumask) {
    let this_cpu = smp_processor_id();
    if __this_cpu_inc_return(&raw mut cpu_relax_retry) >= spin_retry {
        __this_cpu_write(&raw mut cpu_relax_retry, 0);
        let cpu = cpumask_next_wrap(this_cpu, cpumask);
        if cpu >= nr_cpu_ids { return; }
        if arch_vcpu_is_preempted(cpu) { smp_yield_cpu(cpu); }
    }
}

unsafe extern "C" fn do_sync_core(_info: *mut c_void) { sync_core(); }

pub unsafe extern "C" fn text_poke_sync() {
    on_each_cpu(Some(do_sync_core), core::ptr::null_mut(), 1);
}

pub unsafe extern "C" fn text_poke_sync_lock() {
    cpus_read_lock(); text_poke_sync(); cpus_read_unlock();
}

pub unsafe extern "C" fn cpu_init() {
    let id: *mut cpuid = &mut (*this_cpu_ptr(&raw mut cpu_info_per_cpu)).cpu_id;
    get_cpu_id(id);
    if machine_has_cpu_mhz { update_cpu_mhz(core::ptr::null_mut()); }
    mmgrab(&raw mut init_mm);
    (*current).active_mm = &raw mut init_mm;
    BUG_ON((*current).mm != core::ptr::null_mut());
    enter_lazy_tlb(&raw mut init_mm, current);
}

unsafe fn show_facilities(m: *mut seq_file) {
    let mut bit: c_uint = 0;
    seq_puts(m, "facilities      :");
    for_each_set_bit_inv(bit, &raw const stfle_fac_list as *const _, MAX_FACILITY_BIT) {
        seq_printf(m, " %d", bit);
    }
    seq_putc(m, b'\n' as c_int);
}

unsafe fn show_cpu_summary(m: *mut seq_file, _v: *mut c_void) {
    static hwcap_str: [*const c_char; HWCAP_NR_MAX] = [
        [HWCAP_NR_ESAN3] = c"esan3".as_ptr(), [HWCAP_NR_ZARCH] = c"zarch".as_ptr(),
        [HWCAP_NR_STFLE] = c"stfle".as_ptr(), [HWCAP_NR_MSA] = c"msa".as_ptr(),
        [HWCAP_NR_LDISP] = c"ldisp".as_ptr(), [HWCAP_NR_EIMM] = c"eimm".as_ptr(),
        [HWCAP_NR_DFP] = c"dfp".as_ptr(), [HWCAP_NR_HPAGE] = c"edat".as_ptr(),
        [HWCAP_NR_ETF3EH] = c"etf3eh".as_ptr(), [HWCAP_NR_HIGH_GPRS] = c"highgprs".as_ptr(),
        [HWCAP_NR_TE] = c"te".as_ptr(), [HWCAP_NR_VXRS] = c"vx".as_ptr(),
        [HWCAP_NR_VXRS_BCD] = c"vxd".as_ptr(), [HWCAP_NR_VXRS_EXT] = c"vxe".as_ptr(),
        [HWCAP_NR_GS] = c"gs".as_ptr(), [HWCAP_NR_VXRS_EXT2] = c"vxe2".as_ptr(),
        [HWCAP_NR_VXRS_PDE] = c"vxp".as_ptr(), [HWCAP_NR_SORT] = c"sort".as_ptr(),
        [HWCAP_NR_DFLT] = c"dflt".as_ptr(), [HWCAP_NR_VXRS_PDE2] = c"vxp2".as_ptr(),
        [HWCAP_NR_NNPA] = c"nnpa".as_ptr(), [HWCAP_NR_PCI_MIO] = c"pcimio".as_ptr(),
        [HWCAP_NR_SIE] = c"sie".as_ptr(),
    ];
    BUILD_BUG_ON(hwcap_str.len() != HWCAP_NR_MAX);
    seq_printf(m, c"vendor_id       : IBM/S390\n# processors    : %i\nbogomips per cpu: %lu.%02lu\n", num_online_cpus(), loops_per_jiffy / (500000 / HZ), (loops_per_jiffy / (5000 / HZ)) % 100);
    seq_printf(m, c"max thread id   : %d\n", smp_cpu_mtid);
    seq_puts(m, c"features\t: ");
    for i in 0..hwcap_str.len() { if !hwcap_str[i].is_null() && (elf_hwcap & (1UL << i)) != 0 { seq_printf(m, c"%s ", hwcap_str[i]); } }
    seq_puts(m, c"\n"); show_facilities(m); show_cacheinfo(m);
    for_each_online_cpu(cpu) { let id = &(*per_cpu_ptr(&raw mut cpu_info_per_cpu, cpu)).cpu_id; seq_printf(m, c"processor %d: version = %02X,  identification = %06X,  machine = %04X\n", cpu, id.version, id.ident, id.machine); }
}

unsafe extern "C" fn setup_hwcaps() -> c_int {
    elf_hwcap |= HWCAP_ESAN3 | HWCAP_ZARCH | HWCAP_EIMM | HWCAP_HIGH_GPRS;
    if test_facility(7) { elf_hwcap |= HWCAP_STFLE; } if test_facility(17) { elf_hwcap |= HWCAP_MSA; }
    if test_facility(19) { elf_hwcap |= HWCAP_LDISP; } if test_facility(22) && test_facility(30) { elf_hwcap |= HWCAP_ETF3EH; }
    if test_facility(42) && test_facility(44) { elf_hwcap |= HWCAP_DFP; } if cpu_has_edat1() { elf_hwcap |= HWCAP_HPAGE; }
    if machine_has_tx() { elf_hwcap |= HWCAP_TE; }
    if test_facility(129) { elf_hwcap |= HWCAP_VXRS; if test_facility(134) { elf_hwcap |= HWCAP_VXRS_BCD; } if test_facility(135) { elf_hwcap |= HWCAP_VXRS_EXT; } if test_facility(148) { elf_hwcap |= HWCAP_VXRS_EXT2; } if test_facility(152) { elf_hwcap |= HWCAP_VXRS_PDE; } if test_facility(192) { elf_hwcap |= HWCAP_VXRS_PDE2; } }
    if test_facility(150) { elf_hwcap |= HWCAP_SORT; } if test_facility(151) { elf_hwcap |= HWCAP_DFLT; } if test_facility(165) { elf_hwcap |= HWCAP_NNPA; }
    if cpu_has_gs() { elf_hwcap |= HWCAP_GS; } if test_machine_feature(MFEATURE_PCI_MIO) { elf_hwcap |= HWCAP_PCI_MIO; } if sclp.has_sief2 { elf_hwcap |= HWCAP_SIE; } 0
}
arch_initcall!(setup_hwcaps);

unsafe extern "C" fn setup_elf_platform() -> c_int {
    let mut cpu_id = cpuid::default(); get_cpu_id(&mut cpu_id); add_device_randomness(&cpu_id as *const _ as *const c_void, core::mem::size_of::<cpuid>());
    elf_platform = [0; ELF_PLATFORM_SIZE]; let s = match cpu_id.machine { 0x2817 | 0x2818 => c"z196", 0x2827 | 0x2828 => c"zEC12", 0x2964 | 0x2965 => c"z13", 0x3906 | 0x3907 => c"z14", 0x8561 | 0x8562 => c"z15", 0x3931 | 0x3932 => c"z16", 0x9175 | 0x9176 => c"z17", _ => c"z10" }; strscpy(elf_platform.as_mut_ptr(), s.as_ptr()); 0
}
arch_initcall!(setup_elf_platform);

unsafe fn show_cpu_topology(m: *mut seq_file, n: c_ulong) {
    #[cfg(CONFIG_SCHED_TOPOLOGY)] { seq_printf(m, c"physical id     : %d\n", topology_physical_package_id(n)); seq_printf(m, c"core id         : %d\n", topology_core_id(n)); seq_printf(m, c"book id         : %d\n", topology_book_id(n)); seq_printf(m, c"drawer id       : %d\n", topology_drawer_id(n)); seq_printf(m, c"dedicated       : %d\n", topology_cpu_dedicated(n)); seq_printf(m, c"address         : %d\n", smp_cpu_get_cpu_address(n)); seq_printf(m, c"siblings        : %d\n", cpumask_weight(topology_core_cpumask(n))); seq_printf(m, c"cpu cores       : %d\n", topology_booted_cores(n)); }
}
unsafe fn show_cpu_ids(m: *mut seq_file, n: c_ulong) { let id = &(*per_cpu_ptr(&raw mut cpu_info_per_cpu, n)).cpu_id; seq_printf(m, c"version         : %02X\n", id.version); seq_printf(m, c"identification  : %06X\n", id.ident); seq_printf(m, c"machine         : %04X\n", id.machine); }
unsafe fn show_cpu_mhz(m: *mut seq_file, n: c_ulong) { let c = per_cpu_ptr(&raw mut cpu_info_per_cpu, n); if !machine_has_cpu_mhz { return; } seq_printf(m, c"cpu MHz dynamic : %d\n", (*c).cpu_mhz_dynamic); seq_printf(m, c"cpu MHz static  : %d\n", (*c).cpu_mhz_static); }
unsafe extern "C" fn show_cpuinfo(m: *mut seq_file, v: *mut c_void) -> c_int { let n = v as c_ulong - 1; let first = cpumask_first(cpu_online_mask); if n == first { show_cpu_summary(m, v); } seq_printf(m, c"\ncpu number      : %ld\n", n); show_cpu_topology(m, n); show_cpu_ids(m, n); show_cpu_mhz(m, n); 0 }
unsafe fn c_update(pos: *mut loff_t) -> *mut c_void { if *pos != 0 { *pos = cpumask_next(*pos - 1, cpu_online_mask); } else { *pos = cpumask_first(cpu_online_mask); } if *pos < nr_cpu_ids { (*pos + 1) as *mut c_void } else { core::ptr::null_mut() } }
unsafe extern "C" fn c_start(_m: *mut seq_file, pos: *mut loff_t) -> *mut c_void { cpus_read_lock(); c_update(pos) }
unsafe extern "C" fn c_next(_m: *mut seq_file, _v: *mut c_void, pos: *mut loff_t) -> *mut c_void { *pos += 1; c_update(pos) }
unsafe extern "C" fn c_stop(_m: *mut seq_file, _v: *mut c_void) { cpus_read_unlock(); }

#[no_mangle]
pub static cpuinfo_op: seq_operations = seq_operations { start: Some(c_start), next: Some(c_next), stop: Some(c_stop), show: Some(show_cpuinfo) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
