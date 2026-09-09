// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/common/mcpm_entry.c -- entry point for multi-cluster PM
 *
 * Created by:  Nicolas Pitre, March 2012
 * Copyright:   (C) 2012-2013  Linaro Limited
 */

// C dependencies: linux/export.h, linux/kernel.h, linux/init.h,
// linux/irqflags.h, linux/cpu_pm.h, asm/mcpm.h, asm/cacheflush.h,
// asm/idmap.h, asm/cputype.h, and asm/suspend.h.

extern "C" {
    #[repr(C)]
    pub struct McpmCpuSync { pub cpu: i32 }
    #[repr(C)]
    pub struct McpmSyncStruct {
        pub cluster: i32,
        pub inbound: i32,
        pub cpus: [McpmCpuSync; MAX_CPUS_PER_CLUSTER as usize],
    }
    #[repr(C)]
    pub struct SyncStruct {
        pub clusters: [McpmSyncStruct; MAX_NR_CLUSTERS as usize],
    }

    pub static mut mcpm_sync: SyncStruct;
    pub static mut mcpm_entry_vectors: [[u64; MAX_CPUS_PER_CLUSTER as usize]; MAX_NR_CLUSTERS as usize];
    pub static mut mcpm_entry_early_pokes: [[[u64; 2]; MAX_CPUS_PER_CLUSTER as usize]; MAX_NR_CLUSTERS as usize];
    pub static mut mcpm_power_up_setup_phys: u64;

    pub fn sync_cache_w<T>(ptr: *mut T);
    pub fn sync_cache_r<T>(ptr: *const T);
    pub fn __sync_cache_range_w(ptr: *mut u64, size: usize);
    pub fn dmb(); pub fn sev(); pub fn wfe(); pub fn wfi();
    pub fn read_cpuid_mpidr() -> u32;
    pub fn setup_mm_for_reboot();
    pub fn local_irq_disable(); pub fn local_irq_enable();
    pub fn local_fiq_disable(); pub fn local_fiq_enable();
    pub fn arch_spin_lock(lock: *mut ArchSpinlock);
    pub fn arch_spin_unlock(lock: *mut ArchSpinlock);
    pub fn local_irq_save(flags: *mut usize);
    pub fn local_irq_restore(flags: usize);
    pub fn cpu_reset(entry: u64, arg: bool) -> !;
    pub fn mcpm_entry_point();
    pub fn cpu_resume_no_hyp();
    pub fn cpu_pm_enter() -> i32; pub fn cpu_pm_exit();
    pub fn cpu_suspend(arg: usize, fnptr: unsafe extern "C" fn(usize) -> i32) -> i32;
    pub fn __pa_symbol<T>(ptr: *const T) -> u64;
    pub fn might_sleep();
}

// Build-time constants and kernel helpers are supplied by the surrounding ARM environment.
extern "C" {
    static mut platform_ops: *const McpmPlatformOps;
}
const MAX_NR_CLUSTERS: u32 = 0;
const MAX_CPUS_PER_CLUSTER: u32 = 0;
const CPU_GOING_DOWN: i32 = 0;
const CPU_DOWN: i32 = 0;
const CPU_UP: i32 = 0;
const CLUSTER_GOING_DOWN: i32 = 0;
const CLUSTER_UP: i32 = 0;
const CLUSTER_DOWN: i32 = 0;
const INBOUND_COMING_UP: i32 = 0;
const INBOUND_NOT_COMING_UP: i32 = 0;
const EBUSY: i32 = 16;
const EUNATCH: i32 = 49;

#[repr(C)] pub struct ArchSpinlock { pub lock: u32 }
#[repr(C)] pub struct McpmPlatformOps {
    pub cluster_powerup: Option<unsafe extern "C" fn(u32) -> i32>,
    pub cpu_powerup: Option<unsafe extern "C" fn(u32, u32) -> i32>,
    pub cpu_powerdown_prepare: Option<unsafe extern "C" fn(u32, u32)>,
    pub cluster_powerdown_prepare: Option<unsafe extern "C" fn(u32)>,
    pub cluster_cache_disable: Option<unsafe extern "C" fn()>,
    pub cpu_cache_disable: Option<unsafe extern "C" fn()>,
    pub wait_for_powerdown: Option<unsafe extern "C" fn(u32, u32) -> i32>,
    pub cpu_suspend_prepare: Option<unsafe extern "C" fn(u32, u32)>,
    pub cluster_is_up: Option<unsafe extern "C" fn(u32)>,
    pub cpu_is_up: Option<unsafe extern "C" fn(u32, u32)>,
}

static mut mcpm_lock: ArchSpinlock = ArchSpinlock { lock: 0 };
static mut mcpm_cpu_use_count: [[i32; MAX_CPUS_PER_CLUSTER as usize]; MAX_NR_CLUSTERS as usize] = [[0; MAX_CPUS_PER_CLUSTER as usize]; MAX_NR_CLUSTERS as usize];

unsafe fn __mcpm_cpu_going_down(cpu: u32, cluster: u32) { mcpm_sync.clusters[cluster as usize].cpus[cpu as usize].cpu = CPU_GOING_DOWN; sync_cache_w(&mut mcpm_sync.clusters[cluster as usize].cpus[cpu as usize].cpu); }
unsafe fn __mcpm_cpu_down(cpu: u32, cluster: u32) { dmb(); mcpm_sync.clusters[cluster as usize].cpus[cpu as usize].cpu = CPU_DOWN; sync_cache_w(&mut mcpm_sync.clusters[cluster as usize].cpus[cpu as usize].cpu); sev(); }
unsafe fn __mcpm_outbound_leave_critical(cluster: u32, state: i32) { dmb(); mcpm_sync.clusters[cluster as usize].cluster = state; sync_cache_w(&mut mcpm_sync.clusters[cluster as usize].cluster); sev(); }
unsafe fn __mcpm_outbound_enter_critical(cpu: u32, cluster: u32) -> bool {
    let c = &mut mcpm_sync.clusters[cluster as usize]; c.cluster = CLUSTER_GOING_DOWN; sync_cache_w(&mut c.cluster); sync_cache_r(&c.inbound);
    if c.inbound == INBOUND_COMING_UP { __mcpm_outbound_leave_critical(cluster, CLUSTER_UP); return false; }
    sync_cache_r(&c.cpus[0].cpu as *const _);
    for i in 0..MAX_CPUS_PER_CLUSTER { if i == cpu { continue; } let mut state; loop { state = c.cpus[i as usize].cpu; if state != CPU_GOING_DOWN { break; } wfe(); sync_cache_r(&c.cpus[i as usize].cpu); } if state != CPU_DOWN { __mcpm_outbound_leave_critical(cluster, CLUSTER_UP); return false; } }
    true
}
unsafe fn __mcpm_cluster_state(cluster: u32) -> i32 { sync_cache_r(&mcpm_sync.clusters[cluster as usize].cluster); mcpm_sync.clusters[cluster as usize].cluster }

pub unsafe extern "C" fn mcpm_set_entry_vector(cpu: u32, cluster: u32, ptr: *const core::ffi::c_void) { mcpm_entry_vectors[cluster as usize][cpu as usize] = if ptr.is_null() { 0 } else { __pa_symbol(ptr) }; sync_cache_w(&mut mcpm_entry_vectors[cluster as usize][cpu as usize]); }
pub unsafe extern "C" fn mcpm_set_early_poke(cpu: u32, cluster: u32, addr: u64, val: u64) { let p = mcpm_entry_early_pokes[cluster as usize][cpu as usize].as_mut_ptr(); *p = addr; *p.add(1) = val; __sync_cache_range_w(p, 2 * core::mem::size_of::<u64>()); }
pub unsafe extern "C" fn mcpm_platform_register(ops: *const McpmPlatformOps) -> i32 { if !platform_ops.is_null() { return -EBUSY; } platform_ops = ops; 0 }
pub unsafe extern "C" fn mcpm_is_available() -> bool { !platform_ops.is_null() }

unsafe fn mcpm_cluster_unused(cluster: u32) -> bool { let mut cnt = 0; for i in 0..MAX_CPUS_PER_CLUSTER { cnt |= mcpm_cpu_use_count[cluster as usize][i as usize]; } cnt == 0 }

pub unsafe extern "C" fn mcpm_cpu_power_up(cpu: u32, cluster: u32) -> i32 { if platform_ops.is_null() { return -EUNATCH; } might_sleep(); local_irq_disable(); arch_spin_lock(&mut mcpm_lock); let down = mcpm_cpu_use_count[cluster as usize][cpu as usize] == 0; let cluster_down = mcpm_cluster_unused(cluster); mcpm_cpu_use_count[cluster as usize][cpu as usize] += 1; let p = &*platform_ops; let mut ret = 0; if cluster_down { if let Some(f)=p.cluster_powerup { ret=f(cluster); } } if down && ret == 0 { if let Some(f)=p.cpu_powerup { ret=f(cpu,cluster); } } arch_spin_unlock(&mut mcpm_lock); local_irq_enable(); ret }

pub unsafe extern "C" fn mcpm_cpu_power_down() { let mpidr=read_cpuid_mpidr(); let cpu=mpidr&0xff; let cluster=(mpidr>>8)&0xff; if platform_ops.is_null(){return;} setup_mm_for_reboot(); __mcpm_cpu_going_down(cpu,cluster); arch_spin_lock(&mut mcpm_lock); mcpm_cpu_use_count[cluster as usize][cpu as usize]-=1; let down=mcpm_cpu_use_count[cluster as usize][cpu as usize]==0; let last=mcpm_cluster_unused(cluster); let p=&*platform_ops; if last && __mcpm_outbound_enter_critical(cpu,cluster) { if let Some(f)=p.cpu_powerdown_prepare{f(cpu,cluster)} if let Some(f)=p.cluster_powerdown_prepare{f(cluster)} arch_spin_unlock(&mut mcpm_lock); if let Some(f)=p.cluster_cache_disable{f()} __mcpm_outbound_leave_critical(cluster,CLUSTER_DOWN); } else { if down {if let Some(f)=p.cpu_powerdown_prepare{f(cpu,cluster)}} arch_spin_unlock(&mut mcpm_lock); if let Some(f)=p.cpu_cache_disable{f()} } __mcpm_cpu_down(cpu,cluster); if down{wfi();} cpu_reset(__pa_symbol(mcpm_entry_point),false); }

pub unsafe extern "C" fn mcpm_wait_for_cpu_powerdown(cpu:u32,cluster:u32)->i32 { if platform_ops.is_null(){return -EUNATCH;} let p=&*platform_ops; let Some(f)=p.wait_for_powerdown else{return -EUNATCH}; f(cpu,cluster) }
pub unsafe extern "C" fn mcpm_cpu_suspend(){ mcpm_cpu_power_down(); }
pub unsafe extern "C" fn mcpm_cpu_powered_up()->i32 { if platform_ops.is_null(){return -EUNATCH;} let m=read_cpuid_mpidr(); let cpu=m&0xff; let cluster=(m>>8)&0xff; let mut flags=0; local_irq_save(&mut flags); arch_spin_lock(&mut mcpm_lock); if mcpm_cpu_use_count[cluster as usize][cpu as usize]==0 {mcpm_cpu_use_count[cluster as usize][cpu as usize]=1;} let p=&*platform_ops; if let Some(f)=p.cpu_is_up{f(cpu,cluster)} arch_spin_unlock(&mut mcpm_lock); local_irq_restore(flags); 0 }

pub unsafe extern "C" fn mcpm_sync_init(_power_up_setup: Option<unsafe extern "C" fn(u32)>) -> i32 { for c in 0..MAX_NR_CLUSTERS { mcpm_sync.clusters[c as usize].cluster=CLUSTER_DOWN; mcpm_sync.clusters[c as usize].inbound=INBOUND_NOT_COMING_UP; for cpu in 0..MAX_CPUS_PER_CLUSTER {mcpm_sync.clusters[c as usize].cpus[cpu as usize].cpu=CPU_DOWN;} } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
