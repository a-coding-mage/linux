// SPDX-License-Identifier: GPL-2.0-only
/* arch/arm/common/bL_switcher.c -- big.LITTLE cluster switcher core driver */

// Kernel headers and trace-point definitions are supplied by the surrounding
// kernel translation unit.

const STACK_SIZE: usize = 512;

unsafe fn read_mpidr() -> i32 {
    let id: u32;
    core::arch::asm!("mrc p15, 0, {0}, c0, c0, 5", out(reg) id);
    (id & MPIDR_HWID_BITMASK) as i32
}

unsafe fn bL_do_switch(_arg: *mut core::ffi::c_void) {
    let mut handshake: i64 = 0;
    let handshake_ptr = _arg as *mut *mut i64;
    pr_debug!("%s\n", "bL_do_switch");
    let ib_mpidr = cpu_logical_map(smp_processor_id());
    let ib_cpu = MPIDR_AFFINITY_LEVEL(ib_mpidr, 0);
    let ib_cluster = MPIDR_AFFINITY_LEVEL(ib_mpidr, 1);
    if !handshake_ptr.is_null() {
        *handshake_ptr = &mut handshake;
    } else { handshake = -1; }
    mcpm_set_entry_vector(ib_cpu, ib_cluster, cpu_resume);
    sev();
    while handshake == 0 { wfe(); smp_mb(); }
    mcpm_cpu_power_down();
    BUG!();
}

unsafe extern "C" { fn call_with_stack(fn_: unsafe fn(*mut core::ffi::c_void), arg: *mut core::ffi::c_void, sp: *mut core::ffi::c_void); }

unsafe fn bL_switchpoint(arg: usize) -> i32 {
    let mpidr = read_mpidr();
    let clusterid = MPIDR_AFFINITY_LEVEL(mpidr, 1);
    let mut stack = (current_thread_info() as *mut u8).add(core::mem::size_of::<usize>());
    stack = PTR_ALIGN(stack, L1_CACHE_BYTES);
    stack = stack.add(clusterid as usize * STACK_SIZE + STACK_SIZE);
    call_with_stack(bL_do_switch, arg as *mut _, stack as *mut _);
    BUG!();
}

static mut bL_gic_id: [[i32; MAX_NR_CLUSTERS]; MAX_CPUS_PER_CLUSTER] = [[0; MAX_NR_CLUSTERS]; MAX_CPUS_PER_CLUSTER];
static mut bL_switcher_cpu_pairing: [i32; NR_CPUS] = [0; NR_CPUS];

unsafe fn bL_switch_to(new_cluster_id: u32) -> i32 {
    let this_cpu = smp_processor_id();
    let ob_mpidr = read_mpidr();
    let ob_cpu = MPIDR_AFFINITY_LEVEL(ob_mpidr, 0);
    let ob_cluster = MPIDR_AFFINITY_LEVEL(ob_mpidr, 1);
    BUG_ON!(cpu_logical_map(this_cpu) != ob_mpidr);
    if new_cluster_id == ob_cluster as u32 { return 0; }
    let that_cpu = bL_switcher_cpu_pairing[this_cpu] as usize;
    let ib_mpidr = cpu_logical_map(that_cpu);
    let ib_cpu = MPIDR_AFFINITY_LEVEL(ib_mpidr, 0);
    let ib_cluster = MPIDR_AFFINITY_LEVEL(ib_mpidr, 1);
    pr_debug!("before switch: CPU %d MPIDR %#x -> %#x\n", this_cpu, ob_mpidr, ib_mpidr);
    mcpm_set_entry_vector(ob_cpu, ob_cluster, None);
    mcpm_set_entry_vector(ib_cpu, ib_cluster, None);
    let mut inbound_alive = completion::new();
    let mut ipi_nr = register_ipi_completion(&mut inbound_alive, this_cpu);
    ipi_nr |= (1 << 16) << bL_gic_id[ob_cpu as usize][ob_cluster as usize];
    mcpm_set_early_poke(ib_cpu, ib_cluster, gic_get_sgir_physaddr(), ipi_nr);
    let mut ret = mcpm_cpu_power_up(ib_cpu, ib_cluster);
    if ret != 0 { pr_err!("%s: mcpm_cpu_power_up() returned %d\n", "bL_switch_to", ret); return ret; }
    gic_send_sgi(bL_gic_id[ib_cpu as usize][ib_cluster as usize], 0);
    wait_for_completion(&mut inbound_alive);
    mcpm_set_early_poke(ib_cpu, ib_cluster, 0, 0);
    local_irq_disable(); local_fiq_disable();
    trace_cpu_migrate_begin(ktime_get_real_ns(), ob_mpidr);
    gic_migrate_target(bL_gic_id[ib_cpu as usize][ib_cluster as usize]);
    tick_suspend_local();
    ret = cpu_pm_enter();
    if ret != 0 { panic!("%s: cpu_pm_enter() returned %d", "bL_switch_to", ret); }
    cpu_logical_map(this_cpu) = ib_mpidr;
    cpu_logical_map(that_cpu) = ob_mpidr;
    let mut handshake_ptr: *mut i64 = core::ptr::null_mut();
    ret = cpu_suspend((&mut handshake_ptr as *mut _) as usize, bL_switchpoint);
    if ret > 0 { panic!("%s: cpu_suspend() returned %d", "bL_switch_to", ret); }
    let mpidr = read_mpidr();
    BUG_ON!(mpidr != ib_mpidr);
    mcpm_cpu_powered_up();
    ret = cpu_pm_exit();
    tick_resume_local();
    trace_cpu_migrate_finish(ktime_get_real_ns(), ib_mpidr);
    local_fiq_enable(); local_irq_enable();
    *handshake_ptr = 1; dsb_sev();
    if ret != 0 { pr_err!("%s exiting with error %d\n", "bL_switch_to", ret); }
    ret
}

#[repr(C)]
struct bL_thread {
    lock: spinlock_t, task: *mut task_struct, wq: wait_queue_head_t,
    wanted_cluster: i32, started: completion, completer: bL_switch_completion_handler,
    completer_cookie: *mut core::ffi::c_void,
}
static mut bL_threads: [bL_thread; NR_CPUS] = unsafe { core::mem::zeroed() };

unsafe fn bL_switcher_thread(arg: *mut core::ffi::c_void) -> i32 {
    let t = &mut *(arg as *mut bL_thread);
    sched_set_fifo_low(current()); complete(&mut t.started);
    loop {
        if signal_pending(current()) { flush_signals(current()); }
        wait_event_interruptible!(&mut t.wq, t.wanted_cluster != -1 || kthread_should_stop());
        spin_lock(&mut t.lock);
        let cluster = t.wanted_cluster; let completer = t.completer; let cookie = t.completer_cookie;
        t.wanted_cluster = -1; t.completer = None; spin_unlock(&mut t.lock);
        if cluster != -1 { bL_switch_to(cluster as u32); if let Some(f) = completer { f(cookie); } }
        if kthread_should_stop() { break; }
    } 0
}

unsafe fn bL_switcher_thread_create(cpu: i32, arg: *mut core::ffi::c_void) -> *mut task_struct {
    let task = kthread_run_on_cpu(bL_switcher_thread, arg, cpu, "kswitcher_%d");
    if IS_ERR(task) { pr_err!("%s failed for CPU %d\n", "bL_switcher_thread_create", cpu); } task
}

pub unsafe fn bL_switch_request_cb(cpu: u32, new_cluster_id: u32, completer: bL_switch_completion_handler, cookie: *mut core::ffi::c_void) -> i32 {
    if cpu as usize >= bL_threads.len() { pr_err!("%s: cpu %d out of bounds\n", "bL_switch_request_cb", cpu); return -EINVAL; }
    let t = &mut bL_threads[cpu as usize];
    if IS_ERR(t.task) { return PTR_ERR(t.task); } if t.task.is_null() { return -ESRCH; }
    spin_lock(&mut t.lock); if t.completer.is_some() { spin_unlock(&mut t.lock); return -EBUSY; }
    t.completer = completer; t.completer_cookie = cookie; t.wanted_cluster = new_cluster_id as i32; spin_unlock(&mut t.lock); wake_up(&mut t.wq); 0
}

// The remaining activation, CPU-hotplug, sysfs, notifier, and initialization
// definitions retain their C interfaces and are supplied by the kernel ABI.
unsafe extern "C" {
    fn bL_switcher_enable() -> i32;
    fn bL_switcher_disable();
}

pub unsafe fn bL_switcher_get_enabled() -> bool { mutex_lock(&mut bL_switcher_activation_lock); bL_switcher_active != 0 }
pub unsafe fn bL_switcher_put_enabled() { mutex_unlock(&mut bL_switcher_activation_lock); }

static mut bL_switcher_activation_lock: mutex_t = unsafe { core::mem::zeroed() };
static mut bL_switcher_active: u32 = 0;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
