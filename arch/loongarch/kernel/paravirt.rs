// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation.

static mut HAS_STEAL_CLOCK: i32 = 0;
static mut VIRT_PREEMPT_KEY: StaticKeyFalse = StaticKeyFalse;
static mut VIRT_SPIN_LOCK_KEY: StaticKeyFalse = StaticKeyFalse;
static mut STEAL_TIME: PerCpu<KvmStealTime> = PerCpu::new();

static mut STEAL_ACC: bool = true;

unsafe extern "C" fn parse_no_stealacc(_arg: *mut c_char) -> i32 {
    STEAL_ACC = false;
    0
}

unsafe fn paravt_steal_clock(cpu: i32) -> u64 {
    let src: *mut KvmStealTime = per_cpu(&raw mut STEAL_TIME, cpu);
    let mut version: i32;
    let mut steal: u64;
    loop {
        version = (*src).version;
        virt_rmb(); // Make sure that the version is read before the steal
        steal = (*src).steal;
        virt_rmb(); // Make sure that the steal is read before the next version
        if (version & 1) == 0 && version == (*src).version {
            return steal;
        }
    }
}

#[cfg(CONFIG_SMP)]
static mut NATIVE_OPS: SmpOps = SmpOps::zeroed();

#[cfg(CONFIG_SMP)]
unsafe fn pv_send_ipi_single(cpu: i32, action: u32) {
    if unlikely(action == ACTION_BOOT_CPU) {
        ((*(&raw mut NATIVE_OPS)).send_ipi_single)(cpu, action);
        return;
    }
    let info: *mut IrqCpustat = per_cpu(&raw mut IRQ_STAT, cpu);
    let old = atomic_fetch_or(&raw mut (*info).message, BIT(action));
    if old != 0 {
        return;
    }
    let min = cpu_logical_map(cpu);
    kvm_hypercall3(KVM_HCALL_FUNC_IPI, 1, 0, min);
}

const KVM_IPI_CLUSTER_SIZE: i32 = 2 * BITS_PER_LONG;

#[cfg(CONFIG_SMP)]
unsafe fn pv_send_ipi_mask(mask: *const Cpumask, mut action: u32) {
    let mut min = 0;
    let mut max = 0;
    let mut bitmap: u128 = 0;
    if cpumask_empty(mask) {
        return;
    }
    if unlikely(action == ACTION_BOOT_CPU) {
        ((*(&raw mut NATIVE_OPS)).send_ipi_mask)(mask, action);
        return;
    }
    action = BIT(action);
    for_each_cpu!(i, mask, {
        let info: *mut IrqCpustat = per_cpu(&raw mut IRQ_STAT, i);
        let old = atomic_fetch_or(&raw mut (*info).message, action);
        if old != 0 { continue; }
        let cpu = cpu_logical_map(i);
        if bitmap == 0 { min = cpu; max = cpu; }
        else if cpu < min && cpu > max - KVM_IPI_CLUSTER_SIZE {
            bitmap <<= (min - cpu) as u32; min = cpu;
        } else if cpu > min && cpu < min + KVM_IPI_CLUSTER_SIZE {
            max = if cpu > max { cpu } else { max };
        } else {
            kvm_hypercall3(KVM_HCALL_FUNC_IPI, bitmap as usize, (bitmap >> BITS_PER_LONG) as usize, min);
            min = cpu; max = cpu; bitmap = 0;
        }
        bitmap |= 1u128 << (cpu - min);
    });
    if bitmap != 0 {
        kvm_hypercall3(KVM_HCALL_FUNC_IPI, bitmap as usize, (bitmap >> BITS_PER_LONG) as usize, min);
    }
}

#[cfg(CONFIG_SMP)]
unsafe fn pv_ipi_interrupt(_irq: i32, _dev: *mut c_void) -> IrqReturn {
    clear_csr_estat(1 << INT_SWI0);
    let info: *mut IrqCpustat = this_cpu_ptr(&raw mut IRQ_STAT);
    let action = atomic_xchg(&raw mut (*info).message, 0);
    if action & SMP_RESCHEDULE != 0 { scheduler_ipi(); (*info).ipi_irqs[IPI_RESCHEDULE] += 1; }
    if action & SMP_CALL_FUNCTION != 0 { generic_smp_call_function_interrupt(); (*info).ipi_irqs[IPI_CALL_FUNCTION] += 1; }
    if action & SMP_IRQ_WORK != 0 { irq_work_run(); (*info).ipi_irqs[IPI_IRQ_WORK] += 1; }
    if action & SMP_CLEAR_VECTOR != 0 { complete_irq_moving(); (*info).ipi_irqs[IPI_CLEAR_VECTOR] += 1; }
    IRQ_HANDLED
}

#[cfg(CONFIG_SMP)]
unsafe fn pv_init_ipi() {
    ((*(&raw mut NATIVE_OPS)).init_ipi)();
    let swi = get_percpu_irq(INT_SWI0);
    if swi < 0 { panic!("SWI0 IRQ mapping failed\n"); }
    irq_set_percpu_devid(swi);
    if request_percpu_irq(swi, pv_ipi_interrupt, "SWI0-IPI", &raw mut IRQ_STAT) < 0 {
        panic!("SWI0 IRQ request failed\n");
    }
}

unsafe fn kvm_para_available() -> bool {
    static mut HYPERVISOR_TYPE: i32 = 0;
    if !CPU_HAS_HYPERVISOR { return false; }
    if HYPERVISOR_TYPE == 0 {
        let config = read_cpucfg(CPUCFG_KVM_SIG);
        if memcmp(&config as *const _ as *const c_void, KVM_SIGNATURE.as_ptr() as *const c_void, 4) == 0 {
            HYPERVISOR_TYPE = HYPERVISOR_KVM;
        }
    }
    HYPERVISOR_TYPE == HYPERVISOR_KVM
}

unsafe fn kvm_arch_para_features() -> u32 {
    static mut FEATURE: u32 = 0;
    if !kvm_para_available() { return 0; }
    if FEATURE == 0 { FEATURE = read_cpucfg(CPUCFG_KVM_FEATURE); }
    FEATURE
}

unsafe fn pv_ipi_init() -> i32 {
    if !kvm_para_has_feature(KVM_FEATURE_IPI) { return 0; }
    #[cfg(CONFIG_SMP)] {
        NATIVE_OPS = MP_OPS;
        MP_OPS.init_ipi = pv_init_ipi;
        MP_OPS.send_ipi_single = pv_send_ipi_single;
        MP_OPS.send_ipi_mask = pv_send_ipi_mask;
    }
    0
}

unsafe fn pv_enable_steal_time() -> i32 {
    let cpu = smp_processor_id();
    if HAS_STEAL_CLOCK == 0 { return -EPERM; }
    let st = per_cpu(&raw mut STEAL_TIME, cpu);
    let mut addr = per_cpu_ptr_to_phys(st);
    if PFN_DOWN(addr) != PFN_DOWN(addr + core::mem::size_of::<KvmStealTime>() as usize) {
        pr_warn!("Illegal PV steal time addr %lx\n", addr); return -EFAULT;
    }
    addr |= KVM_STEAL_PHYS_VALID;
    kvm_hypercall2(KVM_HCALL_FUNC_NOTIFY, BIT(KVM_FEATURE_STEAL_TIME), addr);
    0
}

unsafe fn pv_disable_steal_time() {
    if HAS_STEAL_CLOCK != 0 { kvm_hypercall2(KVM_HCALL_FUNC_NOTIFY, BIT(KVM_FEATURE_STEAL_TIME), 0); }
}

#[cfg(CONFIG_SMP)]
unsafe fn pv_time_cpu_online(_cpu: u32) -> i32 { let mut flags = 0; local_irq_save(&mut flags); pv_enable_steal_time(); local_irq_restore(flags); 0 }
#[cfg(CONFIG_SMP)]
unsafe fn pv_time_cpu_down_prepare(_cpu: u32) -> i32 { let mut flags = 0; local_irq_save(&mut flags); pv_disable_steal_time(); local_irq_restore(flags); 0 }

unsafe fn pv_cpu_reboot(_unused: *mut c_void) { pv_disable_steal_time(); }
unsafe fn pv_reboot_notify(_nb: *mut NotifierBlock, _code: usize, _unused: *mut c_void) -> i32 { on_each_cpu(pv_cpu_reboot, core::ptr::null_mut(), 1); NOTIFY_DONE }
static mut PV_REBOOT_NB: NotifierBlock = NotifierBlock { notifier_call: pv_reboot_notify };

unsafe fn pv_time_init() -> i32 {
    if !kvm_para_has_feature(KVM_FEATURE_STEAL_TIME) { return 0; }
    HAS_STEAL_CLOCK = 1;
    if pv_enable_steal_time() < 0 { HAS_STEAL_CLOCK = 0; return 0; }
    register_reboot_notifier(&raw mut PV_REBOOT_NB);
    #[cfg(CONFIG_SMP)] {
        let r = cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN, "loongarch/pv_time:online", pv_time_cpu_online, pv_time_cpu_down_prepare);
        if r < 0 { HAS_STEAL_CLOCK = 0; pr_err!("Failed to install cpu hotplug callbacks\n"); return r; }
        if kvm_para_has_feature(KVM_FEATURE_PREEMPT) { static_branch_enable(&raw mut VIRT_PREEMPT_KEY); }
    }
    static_call_update(PV_STEAL_CLOCK, paravt_steal_clock);
    static_key_slow_inc(&raw mut PARAVIRT_STEAL_ENABLED);
    #[cfg(CONFIG_PARAVIRT_TIME_ACCOUNTING)] if STEAL_ACC { static_key_slow_inc(&raw mut PARAVIRT_STEAL_RQ_ENABLED); }
    if static_key_enabled(&raw mut VIRT_PREEMPT_KEY) { pr_info!("Using paravirt steal-time with preempt enabled\n"); } else { pr_info!("Using paravirt steal-time with preempt disabled\n"); }
    0
}

unsafe fn pv_spinlock_init() -> i32 {
    if !CPU_HAS_HYPERVISOR { return 0; }
    static_branch_enable(&raw mut VIRT_SPIN_LOCK_KEY);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
