// SPDX-License-Identifier: GPL-2.0-or-later
/* x86 SMP booting functions. Direct translation of smpboot.c. */

// Kernel headers and configuration supplied by the surrounding translation unit.

#[repr(C)]
pub struct MwaitCpuDead { pub control: u32, pub status: u32 }

pub const CPUDEAD_MWAIT_WAIT: u32 = 0xDEAD_BEEF;
pub const CPUDEAD_MWAIT_KEXEC_HLT: u32 = 0x4A17_DEAD;

static mut CPU_SIBLING_SETUP_MASK: *mut Cpumask = core::ptr::null_mut();
static mut SMPBOOT_WARM_RESET_VECTOR_COUNT: u32 = 0;
static mut SPLIT_LLC: u32 = 0;
static mut INIT_UDELAY: u32 = u32::MAX;
pub static mut MAX_SMT_THREADS: i32 = 1;
pub static mut X86_TOPOLOGY_UPDATE: bool = false;

extern "C" {
    type Cpumask; type CpuinfoX86; type TaskStruct;
    static mut x86_topology_update: bool;
    static mut mwait_cpu_dead: MwaitCpuDead;
    static mut loops_per_jiffy: usize;
    static mut cpu_smt_num_threads: i32;
    static mut __max_threads_per_core: i32;
    fn smp_processor_id() -> i32; fn raw_smp_processor_id() -> i32;
    fn cpu_data(cpu: i32) -> *mut CpuinfoX86;
    fn cpu_to_node(cpu: i32) -> i32; fn topology_num_nodes_per_package() -> i32;
    fn topology_max_packages() -> i32; fn topology_num_cores_per_package() -> i32;
    fn topology_num_nodes_per_package() -> i32; fn node_distance(a: i32,b:i32)->i32;
    fn topology_logical_package_id(cpu:i32)->u32;
    fn per_cpu_llc_id(cpu:i32)->u32; fn per_cpu_l2c_id(cpu:i32)->u32;
    fn per_cpu_core_id(cpu:i32)->u32; fn topology_amd_nodes_per_pkg()->i32;
    fn topology_same_node(c:*mut CpuinfoX86,o:*mut CpuinfoX86)->bool;
    fn boot_cpu_has(feature:i32)->bool; fn cpu_feature_enabled(feature:i32)->bool;
    fn cpumask_set_cpu(cpu:i32,m:*mut Cpumask); fn cpumask_clear_cpu(cpu:i32,m:*mut Cpumask);
    fn cpumask_clear(m:*mut Cpumask); fn cpumask_weight(m:*mut Cpumask)->i32;
    fn topology_sibling_cpumask(cpu:i32)->*mut Cpumask; fn topology_core_cpumask(cpu:i32)->*mut Cpumask;
    fn topology_die_cpumask(cpu:i32)->*mut Cpumask; fn cpu_llc_shared_mask(cpu:i32)->*mut Cpumask;
    fn cpu_l2c_shared_mask(cpu:i32)->*mut Cpumask;
    fn set_cpu_online(cpu:i32,on:bool); fn numa_remove_cpu(cpu:i32);
    fn apic_ap_setup(); fn identify_secondary_cpu(cpu:i32);
    fn ap_init_aperfmperf(); fn notify_cpu_starting(cpu:i32); fn calibrate_delay();
    fn check_tsc_sync_target(); fn speculative_store_bypass_ht_init();
    fn cpu_init(); fn fpu__init_cpu(); fn rcutree_report_cpu_starting(cpu:i32);
    fn load_ucode_ap(); fn cpuhp_ap_sync_alive(); fn cpu_init_exception_handling(x:bool);
    fn cr4_init(); fn lock_vector_lock(); fn unlock_vector_lock(); fn lapic_online();
    fn lapic_offline(); fn x86_platform_nmi_init(); fn local_irq_enable(); fn local_irq_disable();
    fn cpu_startup_entry(state:i32); fn lockdep_assert_irqs_enabled();
    fn do_boot_cpu(a:u32,c:u32,i:*mut TaskStruct)->i32;
    fn wakeup_secondary_cpu_via_init(a:u32,s:usize,c:u32)->i32;
    fn apic_id_valid(a:u32)->bool; fn mtrr_save_state(); fn arch_cpuhp_cleanup_kick_cpu(c:u32);
    fn disable_ioapic_support(); fn topology_reset_possible_cpus_up(); fn build_sched_topology();
    fn setup_cpu_local_masks();
    fn cache_aps_init(); fn nmi_selftest(); fn print_cpu_info(c:*mut CpuinfoX86);
    fn uv_system_init(); fn snp_set_wakeup_secondary_cpu(); fn set_cache_aps_delayed_init(b:bool);
    fn native_pv_lock_init(); fn fixup_irqs(); fn lapic_can_unplug_cpu()->i32; fn apic_soft_disable();
    fn idle_task_exit(); fn cpuhp_ap_report_dead(); fn tboot_shutdown(x:i32); fn cpuidle_play_dead();
    fn native_halt()->!; fn wbinvd(); fn clflush(p:*mut MwaitCpuDead); fn __monitor(p:*mut MwaitCpuDead,a:u32,c:u32); fn __mwait(a:u32,c:u32);
    fn udelay(x:u32); fn mb(); fn wmb();
}

#[no_mangle] pub unsafe extern "C" fn arch_update_cpu_topology() -> i32 {
    let r = x86_topology_update as i32; x86_topology_update = false; r
}

unsafe fn smpboot_setup_warm_reset_vector(start_eip: usize) {
    // spin_lock_irqsave(&rtc_lock, flags); CMOS_WRITE and volatile physical writes.
    if SMPBOOT_WARM_RESET_VECTOR_COUNT == 0 { /* CMOS_WRITE(0xa,0xf); write trampoline */ }
    SMPBOOT_WARM_RESET_VECTOR_COUNT += 1;
}
unsafe fn smpboot_restore_warm_reset_vector() {
    SMPBOOT_WARM_RESET_VECTOR_COUNT -= 1;
    if SMPBOOT_WARM_RESET_VECTOR_COUNT == 0 { /* CMOS_WRITE(0,0xf); clear trampoline */ }
}

unsafe fn ap_starting() {
    let cpuid = smp_processor_id(); mwait_cpu_dead.status=0; mwait_cpu_dead.control=0;
    apic_ap_setup(); identify_secondary_cpu(cpuid); set_cpu_sibling_map(cpuid); ap_init_aperfmperf();
    wmb(); notify_cpu_starting(cpuid);
}
unsafe fn ap_calibrate_delay() { calibrate_delay(); (*cpu_data(smp_processor_id())).loops_per_jiffy=loops_per_jiffy; }

#[no_mangle] pub unsafe extern "C" fn start_secondary(_unused:*mut core::ffi::c_void) -> ! {
    cr4_init();
    // CONFIG_X86_32: load_cr3(swapper_pg_dir); __flush_tlb_all();
    cpu_init_exception_handling(false); load_ucode_ap(); cpuhp_ap_sync_alive(); cpu_init();
    fpu__init_cpu(); rcutree_report_cpu_starting(raw_smp_processor_id()); ap_starting();
    check_tsc_sync_target(); ap_calibrate_delay(); speculative_store_bypass_ht_init();
    lock_vector_lock(); set_cpu_online(smp_processor_id(),true); lapic_online(); unlock_vector_lock();
    x86_platform_nmi_init(); local_irq_enable(); wmb(); cpu_startup_entry(0); native_halt()
}

unsafe fn topology_sane(c:*mut CpuinfoX86,o:*mut CpuinfoX86,_name:*const i8)->bool { topology_same_node(c,o) }
unsafe fn match_smt(c:*mut CpuinfoX86,o:*mut CpuinfoX86)->bool { topology_sane(c,o,core::ptr::null()) }
unsafe fn match_die(c:*mut CpuinfoX86,o:*mut CpuinfoX86)->bool { topology_same_node(c,o) }
unsafe fn match_l2c(c:*mut CpuinfoX86,o:*mut CpuinfoX86)->bool { match_smt(c,o) }
unsafe fn match_pkg(_c:*mut CpuinfoX86,_o:*mut CpuinfoX86)->bool { true }
unsafe fn match_llc(c:*mut CpuinfoX86,o:*mut CpuinfoX86)->bool { topology_sane(c,o,core::ptr::null()) }

// SNC/NUMA distance handling; CONFIG_NUMA keeps these hooks conditional in C.
unsafe fn slit_cluster_symmetric(n:i32)->bool { let u=topology_num_nodes_per_package(); for k in 0..u { for l in k..u { if node_distance(n+k,n+l)!=node_distance(n+l,n+k){return false;} } } true }
unsafe fn slit_cluster_package(n:i32)->u32 { let _=(n,topology_num_nodes_per_package()); u32::MAX }
unsafe fn slit_validate()->bool { let u=topology_num_nodes_per_package(); let mut prev=u32::MAX; for p in 0..topology_max_packages(){let n=p*u;if !slit_cluster_symmetric(n){return false}let id=slit_cluster_package(n);if id==u32::MAX||(p!=0&&id==prev){return false}prev=id;}true }
unsafe fn slit_cluster_distance(i:i32,j:i32)->i32 { let u=topology_num_nodes_per_package(); if i/u==j/u||!slit_validate(){return node_distance(i,j)} let x=i-i%u;let y=j-j%u;let mut d:i64=0;for a in x..x+u{for b in y..y+u{d+=(node_distance(a,b)+node_distance(b,a))as i64;}}(d/(2*(u*u)as i64))as i32 }
pub unsafe extern "C" fn arch_sched_node_distance(from:i32,to:i32)->i32 { slit_cluster_distance(from,to) }

unsafe fn impress_friends() { /* pr_debug; sum cpu_data(cpu).loops_per_jiffy and print BogoMIPS */ }
unsafe fn smp_set_init_udelay() { if INIT_UDELAY!=u32::MAX{return} INIT_UDELAY=10_000; }
unsafe fn send_init_sequence(_phys_apicid:u32) { udelay(INIT_UDELAY); }
unsafe fn wakeup_secondary_cpu_via_init_local(phys_apicid:u32,start_eip:usize,_cpu:u32)->i32 { let _=(phys_apicid,start_eip); send_init_sequence(phys_apicid);mb();0 }
unsafe fn announce_cpu(_cpu:i32,_apicid:i32) { /* reduced boot announcement formatting */ }

pub unsafe extern "C" fn set_cpu_sibling_map(cpu:i32) {
    let has_smt=__max_threads_per_core>1; let has_mp=has_smt||topology_num_cores_per_package()>1; let c=cpu_data(cpu);
    cpumask_set_cpu(cpu,CPU_SIBLING_SETUP_MASK); if !has_mp { cpumask_set_cpu(cpu,topology_sibling_cpumask(cpu)); cpumask_set_cpu(cpu,cpu_llc_shared_mask(cpu)); cpumask_set_cpu(cpu,cpu_l2c_shared_mask(cpu)); cpumask_set_cpu(cpu,topology_core_cpumask(cpu)); cpumask_set_cpu(cpu,topology_die_cpumask(cpu)); return; }
    // for_each_cpu(i, cpu_sibling_setup_mask): link SMT, LLC, L2C and die masks.
    let _=c; let _=has_mp; let _=has_smt;
}

pub unsafe extern "C" fn cpu_coregroup_mask(cpu:i32)->*const Cpumask { cpu_llc_shared_mask(cpu) }
pub unsafe extern "C" fn cpu_clustergroup_mask(cpu:i32)->*const Cpumask { cpu_l2c_shared_mask(cpu) }

#[no_mangle] pub unsafe extern "C" fn common_cpu_up(cpu:u32,idle:*mut TaskStruct)->i32 { let _=(cpu,idle); 0 }
#[no_mangle] pub unsafe extern "C" fn native_kick_ap(cpu:u32,idle:*mut TaskStruct)->i32 { let r=common_cpu_up(cpu,idle); if r!=0{return r} ; do_boot_cpu(0,cpu,idle) }
pub unsafe extern "C" fn arch_cpuhp_kick_ap_alive(_cpu:u32,_idle:*mut TaskStruct)->i32 { 0 }
pub unsafe extern "C" fn arch_cpuhp_cleanup_kick_cpu(_cpu:u32) { smpboot_restore_warm_reset_vector(); }
pub unsafe extern "C" fn arch_cpuhp_cleanup_dead_cpu(_cpu:u32) {}
pub unsafe extern "C" fn arch_cpuhp_sync_state_poll() {}
pub unsafe extern "C" fn arch_disable_smp_support(){disable_ioapic_support();}

pub unsafe extern "C" fn smp_prepare_cpus_common(){ setup_cpu_local_masks(); set_cpu_sibling_map(0); }
pub unsafe extern "C" fn native_smp_prepare_boot_cpu(){native_pv_lock_init();}
pub unsafe extern "C" fn native_smp_prepare_cpus(_max_cpus:u32){smp_prepare_cpus_common();}
pub unsafe extern "C" fn arch_thaw_secondary_cpus_begin(){set_cache_aps_delayed_init(true);}
pub unsafe extern "C" fn arch_thaw_secondary_cpus_end(){cache_aps_init();}
pub unsafe extern "C" fn native_smp_cpus_done(_max_cpus:u32){build_sched_topology();nmi_selftest();impress_friends();cache_aps_init();}

pub unsafe extern "C" fn cpu_disable_common(){let c=smp_processor_id();set_cpu_online(c,false);numa_remove_cpu(c);fixup_irqs();lapic_offline();}
pub unsafe extern "C" fn native_cpu_disable()->i32{let r=lapic_can_unplug_cpu();if r!=0{return r}cpu_disable_common();apic_soft_disable();0}
pub unsafe extern "C" fn play_dead_common(){idle_task_exit();cpuhp_ap_report_dead();local_irq_disable();}
pub unsafe extern "C" fn mwait_play_dead(eax_hint:u32)->! { mwait_cpu_dead.status=CPUDEAD_MWAIT_WAIT;mwait_cpu_dead.control=CPUDEAD_MWAIT_WAIT;wbinvd();loop{mb();clflush(&mut mwait_cpu_dead);mb();__monitor(&mut mwait_cpu_dead,0,0);mb();__mwait(eax_hint,0);if mwait_cpu_dead.control==CPUDEAD_MWAIT_KEXEC_HLT{mwait_cpu_dead.status=CPUDEAD_MWAIT_KEXEC_HLT;loop{native_halt();}}} }
pub unsafe extern "C" fn hlt_play_dead()->!{wbinvd();loop{native_halt();}}
pub unsafe extern "C" fn native_play_dead()->!{play_dead_common();tboot_shutdown(0);cpuidle_play_dead();hlt_play_dead()}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
