// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * RajeshwarR: Dec 11, 2007
 *   -- Added support for Inter Processor Interrupts
 *
 * Vineetg: Nov 1st, 2007
 *    -- Initial Write (Borrowed heavily from ARM)
 */

// Linux and ARC headers provide the external types, constants, functions, and macros used below.

#[cfg(not(feature = "CONFIG_ARC_HAS_LLSC"))]
#[no_mangle]
pub static mut smp_atomic_ops_lock: arch_spinlock_t = __ARCH_SPIN_LOCK_UNLOCKED;

#[no_mangle]
pub static mut plat_smp_ops: plat_smp_ops = unsafe { core::mem::zeroed() };

/* XXX: per cpu ? Only needed once in early secondary boot */
#[no_mangle]
pub static mut secondary_idle_tsk: *mut task_struct = core::ptr::null_mut();

unsafe fn arc_get_cpu_map(name: *const core::ffi::c_char, cpumask: *mut cpumask) -> i32 {
    let dt_root: c_ulong = of_get_flat_dt_root();
    let mut len: i32 = 0;
    let buf = of_get_flat_dt_prop(dt_root, name, &mut len);

    if buf.is_null() || memchr(buf as *const core::ffi::c_void, 0, len as usize).is_null() {
        return -EINVAL;
    }

    if cpulist_parse(buf, cpumask) != 0 {
        return -EINVAL;
    }

    0
}

unsafe fn arc_init_cpu_possible() {
    let mut cpumask: cpumask = core::mem::zeroed();

    if arc_get_cpu_map(c"possible-cpus".as_ptr(), &mut cpumask) != 0 {
        pr_warn!("Failed to get possible-cpus from dtb, pretending all %u cpus exist\n", NR_CPUS);
        cpumask_setall(&mut cpumask);
    }

    if !cpumask_test_cpu(0, &cpumask) {
        panic!("Master cpu (cpu[0]) is missed in cpu possible mask!");
    }

    init_cpu_possible(&cpumask);
}

#[no_mangle]
pub unsafe fn smp_init_cpus() {
    arc_init_cpu_possible();

    if let Some(init_early_smp) = plat_smp_ops.init_early_smp {
        init_early_smp();
    }
}

#[no_mangle]
pub unsafe fn smp_prepare_cpus(_max_cpus: u32) {
    if num_present_cpus() <= 1 {
        init_cpu_present(cpu_possible_mask);
    }
}

#[no_mangle]
pub unsafe fn smp_cpus_done(_max_cpus: u32) {}

static mut wake_flag: core::cell::UnsafeCell<i32> = core::cell::UnsafeCell::new(0);

#[cfg(feature = "CONFIG_ISA_ARCOMPACT")]
unsafe fn boot_read(f: *mut i32) -> i32 { *f }
#[cfg(feature = "CONFIG_ISA_ARCOMPACT")]
unsafe fn boot_write(f: *mut i32, v: i32) { *f = v; }
#[cfg(not(feature = "CONFIG_ISA_ARCOMPACT"))]
unsafe fn boot_read(f: *mut i32) -> i32 { arc_read_uncached_32(f) }
#[cfg(not(feature = "CONFIG_ISA_ARCOMPACT"))]
unsafe fn boot_write(f: *mut i32, v: i32) { arc_write_uncached_32(f, v); }

unsafe fn arc_default_smp_cpu_kick(cpu: i32, _pc: c_ulong) {
    BUG_ON!(cpu == 0);
    boot_write(wake_flag.get(), cpu);
}

#[no_mangle]
pub unsafe fn arc_platform_smp_wait_to_boot(cpu: i32) {
    if IS_ENABLED!(CONFIG_ARC_SMP_HALT_ON_RESET) { return; }
    while boot_read(wake_flag.get()) != cpu {}
    boot_write(wake_flag.get(), 0);
}

#[no_mangle]
pub unsafe fn arc_platform_smp_cpuinfo() -> *const core::ffi::c_char {
    plat_smp_ops.info.unwrap_or(c"".as_ptr())
}

#[no_mangle]
pub unsafe fn start_kernel_secondary() {
    let mm = &mut init_mm as *mut mm_struct;
    let cpu = smp_processor_id();
    setup_processor();
    mmget(mm);
    mmgrab(mm);
    (*current).active_mm = mm;
    cpumask_set_cpu(cpu, mm_cpumask(mm));
    if let Some(init_per_cpu) = plat_smp_ops.init_per_cpu { init_per_cpu(cpu); }
    if let Some(init_per_cpu) = (*machine_desc).init_per_cpu { init_per_cpu(cpu); }
    notify_cpu_starting(cpu);
    set_cpu_online(cpu, true);
    pr_info!("## CPU%u LIVE ##: Executing Code...\n", cpu);
    local_irq_enable();
    cpu_startup_entry(CPUHP_AP_ONLINE_IDLE);
}

#[no_mangle]
pub unsafe fn __cpu_up(cpu: u32, idle: *mut task_struct) -> i32 {
    secondary_idle_tsk = idle;
    pr_info!("Idle Task [%d] %p", cpu, idle);
    pr_info!("Trying to bring up CPU%u ...\n", cpu);
    if let Some(cpu_kick) = plat_smp_ops.cpu_kick {
        cpu_kick(cpu, first_lines_of_secondary as c_ulong);
    } else {
        arc_default_smp_cpu_kick(cpu as i32, 0);
    }
    let wait_till = jiffies + HZ;
    while time_before(jiffies, wait_till) {
        if cpu_online(cpu) { break; }
    }
    if !cpu_online(cpu) {
        pr_info!("Timeout: CPU%u FAILED to come up !!!\n", cpu);
        return -1;
    }
    secondary_idle_tsk = core::ptr::null_mut();
    0
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ipi_msg_type { IPI_EMPTY = 0, IPI_RESCHEDULE = 1, IPI_CALL_FUNC, IPI_CPU_STOP }

static mut ipi_data: [core::sync::atomic::AtomicUsize; NR_CPUS as usize] = [const { core::sync::atomic::AtomicUsize::new(0) }; NR_CPUS as usize];

unsafe fn ipi_send_msg_one(cpu: i32, msg: ipi_msg_type) {
    pr_debug!("%d Sending msg [%d] to %d\n", smp_processor_id(), msg as i32, cpu);
    let data = &ipi_data[cpu as usize];
    local_irq_save!();
    let bit = 1usize << msg as usize;
    let mut old = data.load(core::sync::atomic::Ordering::Relaxed);
    loop {
        let new = old | bit;
        match data.compare_exchange(old, new, core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::SeqCst) {
            Ok(_) => { if plat_smp_ops.ipi_send.is_some() && old == 0 { plat_smp_ops.ipi_send.unwrap()(cpu); } break; }
            Err(value) => old = value,
        }
    }
    local_irq_restore!();
}

unsafe fn ipi_send_msg(callmap: *const cpumask, msg: ipi_msg_type) {
    for_each_cpu!(cpu, callmap) { ipi_send_msg_one(cpu, msg); }
}

#[no_mangle] pub unsafe fn arch_smp_send_reschedule(cpu: i32) { ipi_send_msg_one(cpu, ipi_msg_type::IPI_RESCHEDULE); }
#[no_mangle] pub unsafe fn smp_send_stop() { let mut targets = core::mem::zeroed(); cpumask_copy(&mut targets, cpu_online_mask); cpumask_clear_cpu(smp_processor_id(), &mut targets); ipi_send_msg(&targets, ipi_msg_type::IPI_CPU_STOP); }
#[no_mangle] pub unsafe fn arch_send_call_function_single_ipi(cpu: i32) { ipi_send_msg_one(cpu, ipi_msg_type::IPI_CALL_FUNC); }
#[no_mangle] pub unsafe fn arch_send_call_function_ipi_mask(mask: *const cpumask) { ipi_send_msg(mask, ipi_msg_type::IPI_CALL_FUNC); }

unsafe fn ipi_cpu_stop() { machine_halt(); }

unsafe fn __do_IPI(msg: c_ulong) -> i32 {
    match msg {
        x if x == ipi_msg_type::IPI_RESCHEDULE as c_ulong => scheduler_ipi(),
        x if x == ipi_msg_type::IPI_CALL_FUNC as c_ulong => generic_smp_call_function_interrupt(),
        x if x == ipi_msg_type::IPI_CPU_STOP as c_ulong => ipi_cpu_stop(),
        _ => return 1,
    }; 0
}

unsafe fn do_IPI(irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let copy = ipi_data[smp_processor_id() as usize].swap(0, core::sync::atomic::Ordering::SeqCst);
    pr_debug!("IPI [%ld] received on cpu %d\n", copy, smp_processor_id());
    if let Some(ipi_clear) = plat_smp_ops.ipi_clear { ipi_clear(irq); }
    let mut pending = copy;
    while pending != 0 {
        let msg = pending.trailing_zeros() as c_ulong;
        if __do_IPI(msg) != 0 { pr_info!("IPI with bogus msg %ld in %ld\n", msg, copy); }
        pending &= !(1usize << msg);
    }
    IRQ_HANDLED
}

static mut ipi_dev: [i32; NR_CPUS as usize] = [0; NR_CPUS as usize];

#[no_mangle]
pub unsafe fn smp_ipi_irq_setup(cpu: i32, hwirq: irq_hw_number_t) -> i32 {
    let dev = &mut ipi_dev[cpu as usize] as *mut i32;
    let virq = irq_find_mapping(core::ptr::null_mut(), hwirq);
    if virq == 0 { panic!("Cannot find virq for root domain and hwirq=%lu", hwirq); }
    if cpu == 0 {
        let rc = request_percpu_irq(virq, do_IPI, c"IPI Interrupt".as_ptr(), dev);
        if rc != 0 { panic!("Percpu IRQ request failed for %u\n", virq); }
    }
    enable_percpu_irq(virq, 0);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
