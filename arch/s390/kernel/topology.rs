// SPDX-License-Identifier: GPL-2.0
/*
 *    Copyright IBM Corp. 2007, 2011
 */

// Linux kernel dependencies and build-time macros are supplied externally.

const PTF_HORIZONTAL: c_ulong = 0;
const PTF_VERTICAL: c_ulong = 1;
const PTF_CHECK: c_ulong = 2;

enum TopologyMode {
    Hw,
    Single,
    Package,
    Uninitialized,
}

#[repr(C)]
struct MaskInfo {
    next: *mut MaskInfo,
    id: u8,
    mask: cpumask_t,
}

static mut topology_mode: TopologyMode = TopologyMode::Uninitialized;
static mut tl_info: *mut sysinfo_15_1_x = core::ptr::null_mut();
static mut cpu_management: c_int = 0;
static mut topology_work: work_struct = DECLARE_WORK!(topology_work_fn);

// Socket/Book linked lists and cpu_topology updates are protected by sched_domains_mutex.
static mut socket_info: MaskInfo = MaskInfo { next: core::ptr::null_mut(), id: 0, mask: cpumask_t::zeroed() };
static mut book_info: MaskInfo = MaskInfo { next: core::ptr::null_mut(), id: 0, mask: cpumask_t::zeroed() };
static mut drawer_info: MaskInfo = MaskInfo { next: core::ptr::null_mut(), id: 0, mask: cpumask_t::zeroed() };

extern "C" {
    static mut cpu_topology: [cpu_topology_s390; NR_CPUS];
}

unsafe fn cpu_group_map(dst: *mut cpumask_t, mut info: *mut MaskInfo, cpu: c_uint) {
    static mut mask: cpumask_t = cpumask_t::zeroed();
    cpumask_clear(&mut mask);
    if !cpumask_test_cpu(cpu, &cpu_setup_mask) { cpumask_copy(dst, &mask); return; }
    cpumask_set_cpu(cpu, &mut mask);
    match topology_mode {
        TopologyMode::Hw => {
            while !info.is_null() {
                if cpumask_test_cpu(cpu, &(*info).mask) { cpumask_copy(&mut mask, &(*info).mask); break; }
                info = (*info).next;
            }
        }
        TopologyMode::Package => cpumask_copy(&mut mask, cpu_present_mask),
        TopologyMode::Single | TopologyMode::Uninitialized => {}
    }
    cpumask_and(&mut mask, &mask, &cpu_setup_mask);
    cpumask_copy(dst, &mask);
}

unsafe fn cpu_thread_map(dst: *mut cpumask_t, mut cpu: c_uint) {
    static mut mask: cpumask_t = cpumask_t::zeroed();
    cpumask_clear(&mut mask);
    if !cpumask_test_cpu(cpu, &cpu_setup_mask) { cpumask_copy(dst, &mask); return; }
    cpumask_set_cpu(cpu, &mut mask);
    if !matches!(topology_mode, TopologyMode::Hw) { cpumask_copy(dst, &mask); return; }
    cpu -= cpu % (smp_cpu_mtid + 1);
    let max_cpu = core::cmp::min(cpu + smp_cpu_mtid, nr_cpu_ids - 1);
    while cpu <= max_cpu { if cpumask_test_cpu(cpu, &cpu_setup_mask) { cpumask_set_cpu(cpu, &mut mask); } cpu += 1; }
    cpumask_copy(dst, &mask);
}

const TOPOLOGY_CORE_BITS: usize = 64;

unsafe fn add_cpus_to_mask(tl_core: *mut topology_core, drawer: *mut MaskInfo, book: *mut MaskInfo, socket: *mut MaskInfo) {
    for core in for_each_set_bit(&(*tl_core).mask, TOPOLOGY_CORE_BITS) {
        let rcore = TOPOLOGY_CORE_BITS - 1 - core + (*tl_core).origin as usize;
        let mut cpu = smp_find_processor_id((rcore << smp_cpu_mt_shift) as c_int);
        if cpu < 0 { continue; }
        let max_cpu = core::cmp::min(cpu + smp_cpu_mtid as c_int, nr_cpu_ids as c_int - 1);
        while cpu <= max_cpu {
            let topo = &mut cpu_topology[cpu as usize];
            topo.drawer_id = (*drawer).id; topo.book_id = (*book).id; topo.socket_id = (*socket).id;
            topo.core_id = rcore as _; topo.thread_id = cpu as _; topo.dedicated = (*tl_core).d;
            cpumask_set_cpu(cpu as c_uint, &mut (*drawer).mask); cpumask_set_cpu(cpu as c_uint, &mut (*book).mask); cpumask_set_cpu(cpu as c_uint, &mut (*socket).mask);
            smp_cpu_set_polarization(cpu, (*tl_core).pp); smp_cpu_set_capacity(cpu, CPU_CAPACITY_HIGH); cpu += 1;
        }
    }
}

unsafe fn clear_masks() { let mut info = &mut socket_info as *mut MaskInfo; while !info.is_null() { cpumask_clear(&mut (*info).mask); info = (*info).next; } let mut info = &mut book_info as *mut MaskInfo; while !info.is_null() { cpumask_clear(&mut (*info).mask); info = (*info).next; } let mut info = &mut drawer_info as *mut MaskInfo; while !info.is_null() { cpumask_clear(&mut (*info).mask); info = (*info).next; } }

unsafe fn next_tle(tle: *mut topology_entry) -> *mut topology_entry { if (*tle).nl == 0 { (tle as *mut topology_core).add(1) as *mut topology_entry } else { (tle as *mut topology_container).add(1) as *mut topology_entry } }

unsafe fn tl_to_masks(info: *mut sysinfo_15_1_x) {
    let mut socket = &mut socket_info as *mut MaskInfo; let mut book = &mut book_info as *mut MaskInfo; let mut drawer = &mut drawer_info as *mut MaskInfo;
    clear_masks(); let mut tle = (*info).tle; let end = (info as *mut u8).add((*info).length as usize) as *mut topology_entry;
    while (tle as usize) < (end as usize) { match (*tle).nl { 3 => { drawer = (*drawer).next; (*drawer).id = (*tle).container.id - 1; }, 2 => { book = (*book).next; (*book).id = (*tle).container.id - 1; }, 1 => { socket = (*socket).next; (*socket).id = (*tle).container.id - 1; }, 0 => add_cpus_to_mask(&mut (*tle).cpu, drawer, book, socket), _ => { clear_masks(); return; } } tle = next_tle(tle); }
}

unsafe fn topology_update_polarization_simple() { for_each_possible_cpu(|cpu| smp_cpu_set_polarization(cpu, POLARIZATION_HRZ)); }

unsafe fn ptf(fc: c_ulong) -> c_int { let mut cc = 0; core::arch::asm!(".insn rre,0xb9a20000,{0},{0}", inout(reg) fc, lateout("cc") cc); CC_TRANSFORM(cc) }

pub unsafe fn topology_set_cpu_management(fc: c_int) -> c_int { if !cpu_has_topology() { return -EOPNOTSUPP; } let rc = ptf(if fc != 0 { PTF_VERTICAL } else { PTF_HORIZONTAL }); if rc != 0 { return -EBUSY; } for_each_possible_cpu(|cpu| smp_cpu_set_polarization(cpu, POLARIZATION_UNKNOWN)); rc }

pub unsafe fn update_cpu_masks() {
    for_each_possible_cpu(|cpu| { let topo = &mut cpu_topology[cpu as usize]; cpu_thread_map(&mut topo.thread_mask, cpu as _); cpu_group_map(&mut topo.core_mask, &mut socket_info, cpu as _); cpu_group_map(&mut topo.book_mask, &mut book_info, cpu as _); cpu_group_map(&mut topo.drawer_mask, &mut drawer_info, cpu as _); topo.booted_cores = 0; if !matches!(topology_mode, TopologyMode::Hw) { let id = if matches!(topology_mode, TopologyMode::Package) { 0 } else { cpu }; topo.thread_id = cpu; topo.core_id = cpu; topo.socket_id = id; topo.book_id = id; topo.drawer_id = id; } });
    hd_reset_state();
    for_each_online_cpu(|cpu| { let topo = &mut cpu_topology[cpu as usize]; let pkg_first = cpumask_first(&topo.core_mask); let topo_package = &mut cpu_topology[pkg_first as usize]; if cpu == pkg_first { for_each_cpu(&topo.core_mask, |sibling| { let topo_sibling = &cpu_topology[sibling as usize]; if sibling == cpumask_first(&topo_sibling.thread_mask) { topo_package.booted_cores += 1; hd_add_core(sibling); } }); } else { topo.booted_cores = topo_package.booted_cores; } });
}

pub unsafe fn store_topology(info: *mut sysinfo_15_1_x) { stsi(info, 15, 1, topology_mnest_limit()); }
unsafe fn __arch_update_dedicated_flag(_: *mut core::ffi::c_void) { if topology_cpu_dedicated(smp_processor_id()) { set_cpu_flag(CIF_DEDICATED_CPU); } else { clear_cpu_flag(CIF_DEDICATED_CPU); } }
unsafe fn __arch_update_cpu_topology() -> c_int { let info = tl_info; let mut rc = 0; let mut hd_status = 0; mutex_lock(&smp_cpu_state_mutex); if cpu_has_topology() { rc = 1; store_topology(info); tl_to_masks(info); } update_cpu_masks(); if !cpu_has_topology() { topology_update_polarization_simple(); } if cpu_management == 1 { hd_status = hd_enable_hiperdispatch(); } mutex_unlock(&smp_cpu_state_mutex); if hd_status == 0 { hd_disable_hiperdispatch(); } rc }
pub unsafe fn arch_update_cpu_topology() -> c_int { let rc = __arch_update_cpu_topology(); on_each_cpu(__arch_update_dedicated_flag, core::ptr::null_mut(), 0); rc }
unsafe fn topology_work_fn(_: *mut work_struct) { rebuild_sched_domains(); }
pub unsafe fn topology_schedule_update() { schedule_work(&mut topology_work); }
unsafe fn topology_flush_work() { flush_work(&mut topology_work); }
unsafe fn topology_timer_fn(_: *mut timer_list) { if ptf(PTF_CHECK) != 0 { topology_schedule_update(); } set_topology_timer(); }
static mut topology_timer: timer_list = timer_list::zeroed();
static mut topology_poll: atomic_t = ATOMIC_INIT!(0);
unsafe fn set_topology_timer() { if atomic_add_unless(&mut topology_poll, -1, 0) { mod_timer(&mut topology_timer, jiffies + msecs_to_jiffies(100)); } else { mod_timer(&mut topology_timer, jiffies + secs_to_jiffies(60)); } }
pub unsafe fn topology_expect_change() { if !cpu_has_topology() { return; } if atomic_read(&topology_poll) > 60 { return; } atomic_add(60, &mut topology_poll); set_topology_timer(); }

unsafe fn set_polarization(polarization: c_int) -> c_int { let mut rc = 0; cpus_read_lock(); mutex_lock(&smp_cpu_state_mutex); if cpu_management != polarization { rc = topology_set_cpu_management(polarization); if rc == 0 { cpu_management = polarization; topology_expect_change(); } } mutex_unlock(&smp_cpu_state_mutex); cpus_read_unlock(); rc }

unsafe fn dispatching_show(_: *mut device, _: *mut device_attribute, buf: *mut c_char) -> ssize_t { mutex_lock(&smp_cpu_state_mutex); let count = sysfs_emit(buf, "%d\n", cpu_management); mutex_unlock(&smp_cpu_state_mutex); count }
unsafe fn dispatching_store(_: *mut device, _: *mut device_attribute, buf: *const c_char, count: usize) -> ssize_t { let mut val = 0; let mut delim = 0; if sscanf(buf, "%d %c", &mut val, &mut delim) != 1 || (val != 0 && val != 1) { return -EINVAL as ssize_t; } let rc = set_polarization(val); if rc != 0 { rc as ssize_t } else { count as ssize_t } }

unsafe fn cpu_polarization_show(dev: *mut device, _: *mut device_attribute, buf: *mut c_char) -> ssize_t { let cpu = (*dev).id; mutex_lock(&smp_cpu_state_mutex); let count = match smp_cpu_get_polarization(cpu) { POLARIZATION_HRZ => sysfs_emit(buf, "horizontal\n"), POLARIZATION_VL => sysfs_emit(buf, "vertical:low\n"), POLARIZATION_VM => sysfs_emit(buf, "vertical:medium\n"), POLARIZATION_VH => sysfs_emit(buf, "vertical:high\n"), _ => sysfs_emit(buf, "unknown\n") }; mutex_unlock(&smp_cpu_state_mutex); count }
unsafe fn cpu_dedicated_show(dev: *mut device, _: *mut device_attribute, buf: *mut c_char) -> ssize_t { let cpu = (*dev).id; mutex_lock(&smp_cpu_state_mutex); let count = sysfs_emit(buf, "%d\n", topology_cpu_dedicated(cpu)); mutex_unlock(&smp_cpu_state_mutex); count }

pub unsafe fn topology_cpu_init(cpu: *mut cpu) -> c_int { let rc = sysfs_create_group(&mut (*cpu).dev.kobj, &topology_cpu_attr_group); if rc != 0 || !cpu_has_topology() { return rc; } let rc = sysfs_create_group(&mut (*cpu).dev.kobj, &topology_extra_cpu_attr_group); if rc != 0 { sysfs_remove_group(&mut (*cpu).dev.kobj, &topology_cpu_attr_group); } rc }
pub unsafe fn cpu_coregroup_mask(cpu: c_int) -> *const cpumask_t { &cpu_topology[cpu as usize].core_mask }
unsafe fn tl_book_mask(_: *mut sched_domain_topology_level, cpu: c_int) -> *const cpumask_t { &cpu_topology[cpu as usize].book_mask }
unsafe fn tl_drawer_mask(_: *mut sched_domain_topology_level, cpu: c_int) -> *const cpumask_t { &cpu_topology[cpu as usize].drawer_mask }
static mut s390_topology: [sched_domain_topology_level; 6] = [SDTL_INIT!(tl_smt_mask, cpu_smt_flags, SMT), SDTL_INIT!(tl_mc_mask, cpu_core_flags, MC), SDTL_INIT!(tl_book_mask, core::ptr::null_mut(), BOOK), SDTL_INIT!(tl_drawer_mask, core::ptr::null_mut(), DRAWER), SDTL_INIT!(tl_pkg_mask, core::ptr::null_mut(), PKG), sched_domain_topology_level::zeroed()];

unsafe fn alloc_masks(info: *mut sysinfo_15_1_x, mut mask: *mut MaskInfo, offset: c_int) { let mut nr = (*info).mag[(TOPOLOGY_NR_MAG - offset) as usize]; for i in 0..((*info).mnest - offset) { nr *= (*info).mag[(TOPOLOGY_NR_MAG - offset - 1 - i) as usize]; } nr = core::cmp::max(nr, 1); for _ in 0..nr { (*mask).next = memblock_alloc_or_panic(core::mem::size_of::<MaskInfo>(), 8) as *mut MaskInfo; mask = (*mask).next; } }
unsafe fn detect_polarization(mut tle: *mut topology_entry) -> c_int { while (*tle).nl != 0 { tle = next_tle(tle); } ((*((tle as *mut topology_core))).pp != POLARIZATION_HRZ) as c_int }
pub unsafe fn topology_init_early() { set_sched_topology(s390_topology.as_mut_ptr()); if matches!(topology_mode, TopologyMode::Uninitialized) { topology_mode = if cpu_has_topology() { TopologyMode::Hw } else { TopologyMode::Single }; } if cpu_has_topology() { tl_info = memblock_alloc_or_panic(PAGE_SIZE, PAGE_SIZE) as *mut sysinfo_15_1_x; store_topology(tl_info); cpu_management = detect_polarization((*tl_info).tle); alloc_masks(tl_info, &mut socket_info, 1); alloc_masks(tl_info, &mut book_info, 2); alloc_masks(tl_info, &mut drawer_info, 3); } cpumask_set_cpu(0, &mut cpu_setup_mask); __arch_update_cpu_topology(); __arch_update_dedicated_flag(core::ptr::null_mut()); }
unsafe fn topology_get_mode(enabled: c_int) -> TopologyMode { if enabled == 0 { TopologyMode::Single } else if cpu_has_topology() { TopologyMode::Hw } else { TopologyMode::Package } }
unsafe fn topology_is_enabled() -> c_int { (!matches!(topology_mode, TopologyMode::Single)) as c_int }
unsafe fn topology_setup(str_: *mut c_char) -> c_int { let mut enabled = false; let rc = kstrtobool(str_, &mut enabled); if rc != 0 { return rc; } topology_mode = topology_get_mode(enabled as c_int); 0 }
unsafe fn topology_init() -> c_int { timer_setup(&mut topology_timer, topology_timer_fn, TIMER_DEFERRABLE); if cpu_has_topology() { set_topology_timer(); } else { topology_update_polarization_simple(); } if IS_ENABLED!(CONFIG_SCHED_TOPOLOGY_VERTICAL) { set_polarization(1); } register_sysctl("s390", topology_ctl_table.as_ptr()); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
