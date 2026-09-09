// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2013-2014 Linaro Ltd. */
/* Copyright (c) 2013-2014 HiSilicon Limited. */

// Linux and architecture headers supplying the names used below are external dependencies.

const HIP04_MAX_CLUSTERS: usize = 4;
const HIP04_MAX_CPUS_PER_CLUSTER: usize = 4;
const POLL_MSEC: u32 = 10;
const TIMEOUT_MSEC: u32 = 1000;

const fn core_reset_bit(x: u32) -> u32 { 1 << x }
const fn neon_reset_bit(x: u32) -> u32 { 1 << (x + 4) }
const fn core_debug_reset_bit(x: u32) -> u32 { 1 << (x + 9) }
const CLUSTER_L2_RESET_BIT: u32 = 1 << 8;
const CLUSTER_DEBUG_RESET_BIT: u32 = 1 << 13;
const fn core_reset_status(x: u32) -> u32 { 1 << x }
const fn neon_reset_status(x: u32) -> u32 { 1 << (x + 4) }
const fn core_debug_reset_status(x: u32) -> u32 { 1 << (x + 9) }
const CLUSTER_L2_RESET_STATUS: u32 = 1 << 8;
const CLUSTER_DEBUG_RESET_STATUS: u32 = 1 << 13;
const fn core_wfi_status(x: u32) -> u32 { 1 << (x + 16) }
const fn core_wfe_status(x: u32) -> u32 { 1 << (x + 20) }
const fn core_debug_ack(x: u32) -> u32 { 1 << (x + 24) }
const fn sc_cpu_reset_req(x: u32) -> usize { 0x520 + ((x as usize) << 3) }
const fn sc_cpu_reset_dreq(x: u32) -> usize { 0x524 + ((x as usize) << 3) }
const fn sc_cpu_reset_status(x: u32) -> usize { 0x1520 + ((x as usize) << 3) }
const FAB_SF_MODE: usize = 0x0c;
const FAB_SF_INVLD: usize = 0x10;
const FB_SF_INVLD_START: u32 = 1 << 8;

static mut sysctrl: *mut core::ffi::c_void = core::ptr::null_mut();
static mut fabric: *mut core::ffi::c_void = core::ptr::null_mut();
static mut hip04_cpu_table: [[i32; HIP04_MAX_CPUS_PER_CLUSTER]; HIP04_MAX_CLUSTERS] = [[0; HIP04_MAX_CPUS_PER_CLUSTER]; HIP04_MAX_CLUSTERS];
static mut boot_lock: spinlock_t = spinlock_t::new();
static mut fabric_phys_addr: u32 = 0;
static mut hip04_boot_method: [u32; 4] = [0; 4];

unsafe fn hip04_cluster_is_down(cluster: usize) -> bool {
    for i in 0..HIP04_MAX_CPUS_PER_CLUSTER { if hip04_cpu_table[cluster][i] != 0 { return false; } }
    true
}

unsafe fn hip04_set_snoop_filter(cluster: u32, on: u32) {
    if fabric.is_null() { BUG(); }
    let mut data = readl_relaxed((fabric as *mut u8).add(FAB_SF_MODE) as _);
    if on != 0 { data |= 1 << cluster; } else { data &= !(1 << cluster); }
    writel_relaxed(data, (fabric as *mut u8).add(FAB_SF_MODE) as _);
    loop { cpu_relax(); if data == readl_relaxed((fabric as *mut u8).add(FAB_SF_MODE) as _) { break; } }
}

unsafe fn hip04_boot_secondary(l_cpu: u32, _idle: *mut task_struct) -> i32 {
    let mpidr = cpu_logical_map(l_cpu); let cpu = MPIDR_AFFINITY_LEVEL(mpidr, 0); let cluster = MPIDR_AFFINITY_LEVEL(mpidr, 1);
    if sysctrl.is_null() { return -ENODEV; }
    if cluster >= HIP04_MAX_CLUSTERS as u32 || cpu >= HIP04_MAX_CPUS_PER_CLUSTER as u32 { return -EINVAL; }
    spin_lock_irq(&mut boot_lock);
    if hip04_cpu_table[cluster as usize][cpu as usize] != 0 { hip04_cpu_table[cluster as usize][cpu as usize] += 1; spin_unlock_irq(&mut boot_lock); return 0; }
    let sys_dreq = (sysctrl as *mut u8).add(sc_cpu_reset_dreq(cluster)); let sys_status = (sysctrl as *mut u8).add(sc_cpu_reset_status(cluster));
    if hip04_cluster_is_down(cluster as usize) { writel_relaxed(CLUSTER_DEBUG_RESET_BIT, sys_dreq as _); loop { cpu_relax(); if readl_relaxed(sys_status as _) & CLUSTER_DEBUG_RESET_STATUS == 0 { break; } } hip04_set_snoop_filter(cluster, 1); }
    let data = core_reset_bit(cpu) | neon_reset_bit(cpu) | core_debug_reset_bit(cpu); writel_relaxed(data, sys_dreq as _);
    loop { cpu_relax(); if data != readl_relaxed(sys_status as _) { break; } }
    udelay(20); arch_send_wakeup_ipi_mask(cpumask_of(l_cpu));
    hip04_cpu_table[cluster as usize][cpu as usize] += 1; spin_unlock_irq(&mut boot_lock); 0
}

// CONFIG_HOTPLUG_CPU conditional: the following callbacks are present in hotplug builds.
#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn hip04_cpu_die(l_cpu: u32) { let mpidr=cpu_logical_map(l_cpu); let cpu=MPIDR_AFFINITY_LEVEL(mpidr,0) as usize; let cluster=MPIDR_AFFINITY_LEVEL(mpidr,1) as usize; spin_lock(&mut boot_lock); hip04_cpu_table[cluster][cpu]-=1; if hip04_cpu_table[cluster][cpu]==1 { spin_unlock(&mut boot_lock); return; } if hip04_cpu_table[cluster][cpu]>1 { pr_err!("Cluster %d CPU%d boots multiple times\n",cluster,cpu); BUG(); } let last=hip04_cluster_is_down(cluster); spin_unlock(&mut boot_lock); if last { v7_exit_coherency_flush(all); } else { v7_exit_coherency_flush(louis); } loop { wfi(); } }

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn hip04_cpu_kill(l_cpu: u32) -> i32 { let mpidr=cpu_logical_map(l_cpu); let cpu=MPIDR_AFFINITY_LEVEL(mpidr,0); let cluster=MPIDR_AFFINITY_LEVEL(mpidr,1); BUG_ON(cluster>=4 || cpu>=4); let count=TIMEOUT_MSEC/POLL_MSEC; spin_lock_irq(&mut boot_lock); for _ in 0..count { if hip04_cpu_table[cluster as usize][cpu as usize]!=0 { spin_unlock_irq(&mut boot_lock); return 0; } cpu_relax(); if readl_relaxed((sysctrl as *mut u8).add(sc_cpu_reset_status(cluster)) as _) & core_wfi_status(cpu)!=0 { break; } spin_unlock_irq(&mut boot_lock); msleep(POLL_MSEC); spin_lock_irq(&mut boot_lock); } let data=core_reset_bit(cpu)|neon_reset_bit(cpu)|core_debug_reset_bit(cpu); writel_relaxed(data,(sysctrl as *mut u8).add(sc_cpu_reset_req(cluster)) as _); spin_unlock_irq(&mut boot_lock); 1 }

#[repr(C)]
struct smp_operations { smp_boot_secondary: unsafe fn(u32, *mut task_struct) -> i32 }
static hip04_smp_ops: smp_operations = smp_operations { smp_boot_secondary: hip04_boot_secondary };

unsafe fn hip04_cpu_table_init() -> bool {
    let mpidr=read_cpuid_mpidr(); let cpu=MPIDR_AFFINITY_LEVEL(mpidr,0); let cluster=MPIDR_AFFINITY_LEVEL(mpidr,1);
    if cluster>=HIP04_MAX_CLUSTERS as u32 || cpu>=HIP04_MAX_CPUS_PER_CLUSTER as u32 { pr_err!("%s: boot CPU is out of bound!\n", "hip04_cpu_table_init"); return false; }
    hip04_set_snoop_filter(cluster,1); hip04_cpu_table[cluster as usize][cpu as usize]=1; true
}

unsafe fn hip04_smp_init() -> i32 {
    let mut ret = -ENODEV;
    let np=of_find_compatible_node(core::ptr::null_mut(),core::ptr::null_mut(),b"hisilicon,hip04-bootwrapper\0".as_ptr() as _); if np.is_null(){return ret;}
    ret=of_property_read_u32_array(np,b"boot-method\0".as_ptr() as _,hip04_boot_method.as_mut_ptr(),4); if ret!=0{return ret;}
    let np_sctl=of_find_compatible_node(core::ptr::null_mut(),core::ptr::null_mut(),b"hisilicon,sysctrl\0".as_ptr() as _); if np_sctl.is_null(){return -ENODEV;}
    let np_fab=of_find_compatible_node(core::ptr::null_mut(),core::ptr::null_mut(),b"hisilicon,hip04-fabric\0".as_ptr() as _); if np_fab.is_null(){return -ENODEV;}
    ret=memblock_reserve(hip04_boot_method[0],hip04_boot_method[1]); if ret!=0{return ret;}
    let relocation=ioremap(hip04_boot_method[2],hip04_boot_method[3]); if relocation.is_null(){pr_err!("failed to map relocation space\n");return -ENOMEM;}
    sysctrl=of_iomap(np_sctl,0); if sysctrl.is_null(){pr_err!("failed to get sysctrl base\n");return -ENOMEM;}
    fabric=of_iomap(np_fab,0); if fabric.is_null(){pr_err!("failed to get fabric base\n");return -ENOMEM;}
    if !hip04_cpu_table_init(){return -EINVAL;}
    writel_relaxed(hip04_boot_method[0],relocation); writel_relaxed(0xa5a5a5a5,(relocation as *mut u8).add(4) as _); writel_relaxed(__pa_symbol(secondary_startup),(relocation as *mut u8).add(8) as _); writel_relaxed(0,(relocation as *mut u8).add(12) as _); iounmap(relocation); smp_set_ops(&hip04_smp_ops); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
