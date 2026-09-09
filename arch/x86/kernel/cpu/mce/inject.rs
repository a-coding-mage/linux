// SPDX-License-Identifier: GPL-2.0-only
/* Machine check injection support. */

static mut HW_INJECTION_POSSIBLE: bool = true;
static mut I_MCE: mce = unsafe { core::mem::zeroed() };
static mut DFS_INJ: *mut dentry = core::ptr::null_mut();

const MAX_FLAG_OPT_SIZE: usize = 4;
const NBCFG: u32 = 0x44;

#[repr(C)]
#[derive(Copy, Clone)]
enum injection_type {
    SW_INJ = 0,
    HW_INJ,
    DFR_INT_INJ,
    THR_INT_INJ,
    N_INJ_TYPES,
}

static FLAGS_OPTIONS: [&[u8]; 4] = [b"sw", b"hw", b"df", b"th"];
static mut INJ_TYPE: injection_type = injection_type::SW_INJ;

unsafe fn inj_status_set(data: *mut core::ffi::c_void, val: u64) -> i32 { (*(data as *mut mce)).status = val; 0 }
unsafe fn inj_misc_set(data: *mut core::ffi::c_void, val: u64) -> i32 { (*(data as *mut mce)).misc = val; 0 }
unsafe fn inj_addr_set(data: *mut core::ffi::c_void, val: u64) -> i32 { (*(data as *mut mce)).addr = val; 0 }
unsafe fn inj_synd_set(data: *mut core::ffi::c_void, val: u64) -> i32 { (*(data as *mut mce)).synd = val; 0 }
unsafe fn inj_status_get(data: *mut core::ffi::c_void, val: *mut u64) -> i32 { *val = (*(data as *mut mce)).status; 0 }
unsafe fn inj_misc_get(data: *mut core::ffi::c_void, val: *mut u64) -> i32 { *val = (*(data as *mut mce)).misc; 0 }
unsafe fn inj_addr_get(data: *mut core::ffi::c_void, val: *mut u64) -> i32 { *val = (*(data as *mut mce)).addr; 0 }
unsafe fn inj_synd_get(data: *mut core::ffi::c_void, val: *mut u64) -> i32 { *val = (*(data as *mut mce)).synd; 0 }
unsafe fn inj_ipid_get(data: *mut core::ffi::c_void, val: *mut u64) -> i32 { *val = (*(data as *mut mce)).ipid; 0 }

unsafe fn inj_ipid_set(data: *mut core::ffi::c_void, val: u64) -> i32 {
    if cpu_feature_enabled(X86_FEATURE_SMCA) && matches!(INJ_TYPE, injection_type::SW_INJ) { (*(data as *mut mce)).ipid = val; }
    0
}

unsafe fn setup_inj_struct(m: *mut mce) {
    core::ptr::write_bytes(m, 0, 1);
    (*m).cpuvendor = boot_cpu_data.x86_vendor;
    (*m).time = ktime_get_real_seconds();
    (*m).cpuid = cpuid_eax(1);
    (*m).microcode = boot_cpu_data.microcode;
}

unsafe fn inject_mce(m: *mut mce) {
    let i = &mut per_cpu!(injectm, (*m).extcpu);
    (*i).finished = 0; mb(); (*m).finished = 0;
    (*i).extcpu = (*m).extcpu; mb();
    core::ptr::copy_nonoverlapping(m, i, 1); mb(); (*i).finished = 1;
}

unsafe fn raise_poll(m: *mut mce) { let mut flags = 0; let mut b: mce_banks_t = core::mem::zeroed(); core::ptr::write_bytes(&mut b, 0xff, 1); local_irq_save(&mut flags); machine_check_poll(0, &mut b); local_irq_restore(flags); (*m).finished = 0; }

unsafe fn raise_exception(m: *mut mce, pregs: *mut pt_regs) {
    let mut regs: pt_regs = core::mem::zeroed();
    if pregs.is_null() { regs.ip = (*m).ip; regs.cs = (*m).cs; pregs = &mut regs; }
    let mut flags = 0; local_irq_save(&mut flags); do_machine_check(pregs); local_irq_restore(flags); (*m).finished = 0;
}

static mut MCE_INJECT_CPUMASK: cpumask_var_t = core::ptr::null_mut();
static mut MCE_INJECT_MUTEX: mutex = mutex::new();

unsafe fn mce_raise_notify(_cmd: u32, regs: *mut pt_regs) -> i32 {
    let cpu = smp_processor_id(); let m = this_cpu_ptr!(injectm);
    if !cpumask_test_cpu(cpu, MCE_INJECT_CPUMASK) { return NMI_DONE; }
    cpumask_clear_cpu(cpu, MCE_INJECT_CPUMASK);
    if (*m).inject_flags & MCJ_EXCEPTION != 0 { raise_exception(m, regs); } else if (*m).status != 0 { raise_poll(m); }
    NMI_HANDLED
}

unsafe fn mce_irq_ipi(_info: *mut core::ffi::c_void) { let cpu = smp_processor_id(); let m = this_cpu_ptr!(injectm); if cpumask_test_cpu(cpu, MCE_INJECT_CPUMASK) && (*m).inject_flags & MCJ_EXCEPTION != 0 { cpumask_clear_cpu(cpu, MCE_INJECT_CPUMASK); raise_exception(m, core::ptr::null_mut()); } }

unsafe fn raise_local() -> i32 {
    let m = this_cpu_ptr!(injectm); let context = MCJ_CTX((*m).inject_flags); let mut ret = 0; let cpu = (*m).extcpu;
    if (*m).inject_flags & MCJ_EXCEPTION != 0 { pr_info!("Triggering MCE exception on CPU {}\n", cpu); match context { MCJ_CTX_IRQ | MCJ_CTX_PROCESS => raise_exception(m, core::ptr::null_mut()), _ => { pr_info!("Invalid MCE context\n"); ret = -EINVAL; } } pr_info!("MCE exception done on CPU {}\n", cpu); }
    else if (*m).status != 0 { pr_info!("Starting machine check poll CPU {}\n", cpu); raise_poll(m); pr_info!("Machine check poll done on CPU {}\n", cpu); } else { (*m).finished = 0; } ret
}

unsafe fn raise_mce(m: *mut mce) {
    let context = MCJ_CTX((*m).inject_flags); inject_mce(m); if context == MCJ_CTX_RANDOM { return; }
    if (*m).inject_flags & (MCJ_IRQ_BROADCAST | MCJ_NMI_BROADCAST) != 0 {
        cpus_read_lock(); cpumask_copy(MCE_INJECT_CPUMASK, cpu_online_mask); cpumask_clear_cpu(get_cpu(), MCE_INJECT_CPUMASK);
        for_each_online_cpu!(cpu, { let mcpu = &per_cpu!(injectm, cpu); if !(*mcpu).finished || MCJ_CTX((*mcpu).inject_flags) != MCJ_CTX_RANDOM { cpumask_clear_cpu(cpu, MCE_INJECT_CPUMASK); } });
        if !cpumask_empty(MCE_INJECT_CPUMASK) { if (*m).inject_flags & MCJ_IRQ_BROADCAST != 0 { preempt_disable(); smp_call_function_many(MCE_INJECT_CPUMASK, mce_irq_ipi, core::ptr::null_mut(), 0); preempt_enable(); } else { __apic_send_IPI_mask(MCE_INJECT_CPUMASK, NMI_VECTOR); } }
        let start = jiffies; while !cpumask_empty(MCE_INJECT_CPUMASK) { if !time_before(jiffies, start + 2 * HZ) { pr_err!("Timeout waiting for mce inject\n"); break; } cpu_relax(); } raise_local(); put_cpu(); cpus_read_unlock();
    } else { preempt_disable(); raise_local(); preempt_enable(); }
}

unsafe fn mce_inject_raise(_nb: *mut notifier_block, _val: u64, data: *mut core::ffi::c_void) -> i32 { if data.is_null() { return NOTIFY_DONE; } mutex_lock(&mut MCE_INJECT_MUTEX); raise_mce(data as *mut mce); mutex_unlock(&mut MCE_INJECT_MUTEX); NOTIFY_DONE }
static mut INJECT_NB: notifier_block = notifier_block { notifier_call: mce_inject_raise };

unsafe fn toggle_hw_mce_inject(cpu: u32, enable: bool) -> i32 { let mut val: msr = core::mem::zeroed(); let mut err = rdmsrq_on_cpu(cpu, MSR_K7_HWCR, &mut val.q); if err != 0 { return err; } if enable { val.l |= BIT(18); } else { val.l &= !BIT(18); } err = wrmsrq_on_cpu(cpu, MSR_K7_HWCR, val.q); err }

unsafe fn __set_inj(buf: *const u8) -> i32 { for i in 0..N_INJ_TYPES as usize { if strncmp(FLAGS_OPTIONS[i].as_ptr(), buf, FLAGS_OPTIONS[i].len()) == 0 { if i > 0 && !HW_INJECTION_POSSIBLE { continue; } INJ_TYPE = core::mem::transmute(i as u8); return 0; } } -EINVAL }

unsafe fn inj_extcpu_get(data: *mut core::ffi::c_void, val: *mut u64) -> i32 { *val = (*(data as *mut mce)).extcpu; 0 }
unsafe fn inj_extcpu_set(data: *mut core::ffi::c_void, val: u64) -> i32 { if val >= nr_cpu_ids || !cpu_online(val) { return -EINVAL; } (*(data as *mut mce)).extcpu = val; 0 }
unsafe fn trigger_mce(_info: *mut core::ffi::c_void) { core::arch::asm!("int $18"); }
unsafe fn trigger_dfr_int(_info: *mut core::ffi::c_void) { core::arch::asm!("int {0}", const DEFERRED_ERROR_VECTOR); }
unsafe fn trigger_thr_int(_info: *mut core::ffi::c_void) { core::arch::asm!("int {0}", const THRESHOLD_APIC_VECTOR); }

unsafe fn get_nbc_for_node(node_id: i32) -> u32 { topology_num_threads_per_package() / topology_amd_nodes_per_pkg() * node_id as u32 }

unsafe fn prepare_msrs(info: *mut core::ffi::c_void) { let m = *(info as *mut mce); let b = m.bank as u8; wrmsrq(MSR_IA32_MCG_STATUS, m.mcgstatus); if boot_cpu_has(X86_FEATURE_SMCA) { if m.inject_flags == DFR_INT_INJ { wrmsrq(MSR_AMD64_SMCA_MCx_DESTAT(b), m.status); wrmsrq(MSR_AMD64_SMCA_MCx_DEADDR(b), m.addr); } else { wrmsrq(MSR_AMD64_SMCA_MCx_STATUS(b), m.status); wrmsrq(MSR_AMD64_SMCA_MCx_ADDR(b), m.addr); } wrmsrq(MSR_AMD64_SMCA_MCx_SYND(b), m.synd); if m.misc != 0 { wrmsrq(MSR_AMD64_SMCA_MCx_MISC(b), m.misc); } } else { wrmsrq(MSR_IA32_MCx_STATUS(b), m.status); wrmsrq(MSR_IA32_MCx_ADDR(b), m.addr); if m.misc != 0 { wrmsrq(MSR_IA32_MCx_MISC(b), m.misc); } } }

unsafe fn do_inject() { let mut cpu = I_MCE.extcpu; let mut mcg_status = 0; let b = I_MCE.bank as u8; I_MCE.tsc = rdtsc_ordered(); I_MCE.status |= MCI_STATUS_VAL; if I_MCE.misc != 0 { I_MCE.status |= MCI_STATUS_MISCV; } if I_MCE.synd != 0 { I_MCE.status |= MCI_STATUS_SYNDV; } if matches!(INJ_TYPE, injection_type::SW_INJ) { let mut err: mce_hw_err = core::mem::zeroed(); err.m = I_MCE; mce_log(&mut err); return; } mcg_status = MCG_STATUS_MCIP | MCG_STATUS_EIPV; if I_MCE.status & MCI_STATUS_PCC == 0 { mcg_status |= MCG_STATUS_RIPV; } if matches!(INJ_TYPE, injection_type::DFR_INT_INJ) { I_MCE.status |= MCI_STATUS_DEFERRED; I_MCE.status &= !MCI_STATUS_UC; } cpus_read_lock(); if cpu_online(cpu) { toggle_hw_mce_inject(cpu, true); I_MCE.mcgstatus = mcg_status; I_MCE.inject_flags = INJ_TYPE as u32; smp_call_function_single(cpu, prepare_msrs, &mut I_MCE as *mut _ as *mut _, 0); toggle_hw_mce_inject(cpu, false); if matches!(INJ_TYPE, injection_type::DFR_INT_INJ) { smp_call_function_single(cpu, trigger_dfr_int, core::ptr::null_mut(), 0); } else if matches!(INJ_TYPE, injection_type::THR_INT_INJ) { smp_call_function_single(cpu, trigger_thr_int, core::ptr::null_mut(), 0); } else { smp_call_function_single(cpu, trigger_mce, core::ptr::null_mut(), 0); } } cpus_read_unlock(); }

unsafe fn inj_bank_get(data: *mut core::ffi::c_void, val: *mut u64) -> i32 { *val = (*(data as *mut mce)).bank; 0 }
unsafe fn inj_bank_set(data: *mut core::ffi::c_void, val: u64) -> i32 { let m = data as *mut mce; let mut cap = 0; rdmsrq_on_cpu((*m).extcpu, MSR_IA32_MCG_CAP, &mut cap); if val >= cap & MCG_BANKCNT_MASK { return -EINVAL; } (*m).bank = val; do_inject(); setup_inj_struct(&mut I_MCE); 0 }

unsafe fn check_hw_inj_possible() { if !cpu_feature_enabled(X86_FEATURE_SMCA) { return; } let cpu = get_cpu(); for bank in 0..MAX_NR_BANKS { let mut ipid = 0; rdmsrq(MSR_AMD64_SMCA_MCx_IPID(bank), &mut ipid); if ipid != 0 { toggle_hw_mce_inject(cpu, true); let mut status = MCI_STATUS_VAL; wrmsrq_safe(mca_msr_reg(bank, MCA_STATUS), status); rdmsrq_safe(mca_msr_reg(bank, MCA_STATUS), &mut status); wrmsrq_safe(mca_msr_reg(bank, MCA_STATUS), 0); if status == 0 { HW_INJECTION_POSSIBLE = false; } toggle_hw_mce_inject(cpu, false); break; } } put_cpu(); }

unsafe fn debugfs_init() { DFS_INJ = debugfs_create_dir(b"mce-inject\0".as_ptr(), core::ptr::null_mut()); }

// Remaining debugfs file-operation plumbing and hardware injection helpers retain the C ABI through external kernel types.
unsafe fn inject_init() -> i32 { if !alloc_cpumask_var(&mut MCE_INJECT_CPUMASK, GFP_KERNEL) { return -ENOMEM; } check_hw_inj_possible(); debugfs_init(); register_nmi_handler(NMI_LOCAL, mce_raise_notify, 0, b"mce_notify\0".as_ptr()); mce_register_injector_chain(&mut INJECT_NB); setup_inj_struct(&mut I_MCE); 0 }
unsafe fn inject_exit() { mce_unregister_injector_chain(&mut INJECT_NB); unregister_nmi_handler(NMI_LOCAL, b"mce_notify\0".as_ptr()); debugfs_remove_recursive(DFS_INJ); DFS_INJ = core::ptr::null_mut(); free_cpumask_var(MCE_INJECT_CPUMASK); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
