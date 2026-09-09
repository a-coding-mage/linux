// SPDX-License-Identifier: GPL-2.0
/* leon_smp.c: Sparc-Leon SMP support. */

// C header dependencies are supplied by the surrounding kernel translation.

extern "C" {
    static mut srmmu_ctx_table_phys: *mut ctxd_t;
    static mut cpu_callin_map: [core::ffi::c_ulong; NR_CPUS];
    static mut smp_commenced_mask: cpumask_t;
    fn leon_configure_cache_smp();
    fn hard_smp_processor_id() -> i32;
    fn smp_processor_id() -> i32;
    fn sparc_leon3_get_dcachecfg() -> core::ffi::c_ulong;
    fn sparc_leon3_disable_cache();
    fn sparc_leon3_enable_snooping();
    fn leon_smp_nrcpus() -> i32;
    fn leon_enable_irq_cpu(irq: i32, cpu: i32);
    fn udelay(usecs: u32);
    fn task_thread_info(idle: *mut task_struct) -> *mut thread_info;
    fn mmgrab(mm: *mut mm_struct);
    fn cpumask_test_cpu(cpu: i32, mask: *const cpumask_t) -> bool;
    fn mb();
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn prom_printf(fmt: *const core::ffi::c_char, ...);
    fn cpu_online(cpu: i32) -> bool;
    fn cpu_present(cpu: i32) -> bool;
    fn free_reserved_page(page: *mut page);
    fn virt_to_page(addr: *const core::ffi::c_void) -> *mut page;
    fn of_find_node_by_path(path: *const core::ffi::c_char) -> *mut device_node;
    fn of_find_property(node: *mut device_node, name: *const core::ffi::c_char, len: *mut i32) -> *mut property;
    fn local_irq_save(flags: *mut core::ffi::c_ulong);
    fn local_irq_restore(flags: core::ffi::c_ulong);
    fn smp_call_function_single_interrupt();
    fn smp_call_function_interrupt();
    fn smp_resched_interrupt();
    fn leon_get_irqmask(level: i32) -> core::ffi::c_ulong;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut core::ffi::c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: core::ffi::c_ulong);
    fn cpumask_clear_cpu(cpu: i32, mask: *mut cpumask_t);
    fn cpumask_and(dst: *mut cpumask_t, src1: *const cpumask_t, src2: *const cpumask_t);
    fn barrier();
}

extern "C" {
    static mut smp_penguin_ctable: linux_prom_registers;
    static mut leon3_irqctrl_regs: *mut leon3_irqctrl_regs;
    static mut local_ops: *mut sparc32_cachetlb_ops;
    static mut current: *mut task_struct;
    static mut init_mm: mm_struct;
    static mut current_set: [*mut thread_info; NR_CPUS];
    static mut sparc_ttable: [tt_entry; 256];
    static mut t_nmi: [usize; 2];
    static mut sparc32_ipi_ops: *const sparc32_ipi_ops;
    static mut linux_trap_ipi15_leon: usize;
    static mut linux_trap_ipi15_sun4m: usize;
    static mut real_irq_entry: usize;
    static mut smpleon_ipi: usize;
}

static mut smp_processors_ready: i32 = 0;
pub static mut leon_ipi_irq: i32 = LEON3_IRQ_IPI_DEFAULT;

unsafe fn do_swap(ptr: *mut core::ffi::c_ulong, mut val: core::ffi::c_ulong) -> core::ffi::c_ulong {
    // swapa [%ptr] ASI_LEON_DCACHE_MISS, val; exact target-specific atomic exchange.
    core::ptr::read_volatile(ptr).swap_bytes();
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, val);
    val = old;
    val
}

#[no_mangle]
pub unsafe extern "C" fn leon_cpu_pre_starting(_arg: *mut core::ffi::c_void) { leon_configure_cache_smp(); }

#[no_mangle]
pub unsafe extern "C" fn leon_cpu_pre_online(_arg: *mut core::ffi::c_void) {
    let cpuid = hard_smp_processor_id();
    do_swap(&mut cpu_callin_map[cpuid as usize], 1);
    (*local_ops).cache_all(); (*local_ops).tlb_all();
    current_set[cpuid as usize] = core::ptr::read_volatile(&current_set[cpuid as usize]);
    mmgrab(&mut init_mm); (*current).active_mm = &mut init_mm;
    while !cpumask_test_cpu(cpuid, &smp_commenced_mask) { mb(); }
}

#[no_mangle]
pub unsafe extern "C" fn leon_configure_cache_smp() {
    let cfg = sparc_leon3_get_dcachecfg(); let me = smp_processor_id();
    if ASI_LEON3_SYSCTRL_CFG_SSIZE(cfg) > 4 { printk(b"Note: SMP with snooping only works on 4k cache, found %dk(0x%x) on cpu %d, disabling caches\0".as_ptr() as _, ASI_LEON3_SYSCTRL_CFG_SSIZE(cfg), cfg, me); sparc_leon3_disable_cache(); }
    else if cfg & ASI_LEON3_SYSCTRL_CFG_SNOOPING != 0 { sparc_leon3_enable_snooping(); }
    else { printk(b"Note: You have to enable snooping in the vhdl model cpu %d, disabling caches\0".as_ptr() as _, me); sparc_leon3_disable_cache(); }
    (*local_ops).cache_all(); (*local_ops).tlb_all();
}

unsafe fn leon_smp_setbroadcast(mask: u32) {
    let broadcast = (LEON3_BYPASS_LOAD_PA(&(*leon3_irqctrl_regs).mpstatus) >> LEON3_IRQMPSTATUS_BROADCAST) & 1;
    if broadcast == 0 { prom_printf(b"######## !!!! The irqmp-ctrl must have broadcast enabled, smp wont work !!!!! ####### nr cpus: %d\n\0".as_ptr() as _, leon_smp_nrcpus()); if leon_smp_nrcpus() > 1 { BUG!(); } else { prom_printf(b"continue anyway\n\0".as_ptr() as _); return; } }
    LEON_BYPASS_STORE_PA(&mut (*leon3_irqctrl_regs).mpbroadcast, mask);
}

#[no_mangle]
pub unsafe extern "C" fn leon_smp_nrcpus() -> i32 { (((LEON3_BYPASS_LOAD_PA(&(*leon3_irqctrl_regs).mpstatus) >> LEON3_IRQMPSTATUS_CPUNR) & 0xf) + 1) as i32 }

#[no_mangle]
pub unsafe extern "C" fn leon_boot_cpus() {
    leon_ipi_init(); let me = smp_processor_id(); let nrcpu = leon_smp_nrcpus();
    printk(b"%d:(%d:%d) cpus mpirq at 0x%x\0".as_ptr() as _, me, nrcpu, NR_CPUS, &(*leon3_irqctrl_regs).mpstatus);
    leon_enable_irq_cpu(LEON3_IRQ_CROSS_CALL, me); leon_enable_irq_cpu(LEON3_IRQ_TICKER, me); leon_enable_irq_cpu(leon_ipi_irq, me);
    leon_smp_setbroadcast(1 << LEON3_IRQ_TICKER); leon_configure_cache_smp(); (*local_ops).cache_all();
}

#[no_mangle]
pub unsafe extern "C" fn leon_boot_one_cpu(i: i32, idle: *mut task_struct) -> i32 {
    current_set[i as usize] = task_thread_info(idle); smp_penguin_ctable.which_io=0; smp_penguin_ctable.phys_addr=srmmu_ctx_table_phys as u32; smp_penguin_ctable.reg_size=0;
    (*local_ops).cache_all(); LEON_BYPASS_STORE_PA(&mut (*leon3_irqctrl_regs).mask[i as usize], 0); LEON_BYPASS_STORE_PA(&mut (*leon3_irqctrl_regs).mpstatus, 1 << i);
    for _timeout in 0..10000 { if cpu_callin_map[i as usize] != 0 { break; } udelay(200); }
    if cpu_callin_map[i as usize] == 0 { printk(b"Processor %d is stuck.\0".as_ptr() as _, i); return -ENODEV; }
    leon_enable_irq_cpu(LEON3_IRQ_CROSS_CALL,i); leon_enable_irq_cpu(LEON3_IRQ_TICKER,i); leon_enable_irq_cpu(leon_ipi_irq,i); (*local_ops).cache_all(); 0
}

#[no_mangle] pub unsafe extern "C" fn leon_smp_done() {
    let mut first=0; let mut prev=&mut first as *mut i32;
    for i in 0..NR_CPUS as i32 { if cpu_online(i) { *prev=i; prev=core::ptr::null_mut(); /* cpu_data(i).next supplied by kernel */ } }
    if !prev.is_null() {*prev=first;} (*local_ops).cache_all();
    if !cpu_present(1) { free_reserved_page(virt_to_page(core::ptr::null())); }
    if !cpu_present(2) { free_reserved_page(virt_to_page(core::ptr::null())); }
    if !cpu_present(3) { free_reserved_page(virt_to_page(core::ptr::null())); }
    smp_processors_ready=1;
}

#[repr(C)]
struct leon_ipi_work { single: i32, msk: i32, resched: i32 }
static mut leon_ipi_work_per_cpu: [leon_ipi_work; NR_CPUS] = [leon_ipi_work { single: 0, msk: 0, resched: 0 }; NR_CPUS];

unsafe fn leon_ipi_init() {
    let mut len = 0; let rootnp = of_find_node_by_path(b"/ambapp0\0".as_ptr() as _);
    if !rootnp.is_null() { let pp = of_find_property(rootnp, b"ipi_num\0".as_ptr() as _, &mut len); if !pp.is_null() && *( (*pp).value as *const i32) != 0 { leon_ipi_irq = *((*pp).value) as *const i32; } }
    printk(b"leon: SMP IPIs at IRQ %d\n\0".as_ptr() as _, leon_ipi_irq);
    let mut flags=0; local_irq_save(&mut flags); let trap_table=&mut sparc_ttable[(SP_TRAP_IRQ1 + leon_ipi_irq - 1) as usize]; trap_table.inst_three += smpleon_ipi - real_irq_entry; (*local_ops).cache_all(); local_irq_restore(flags);
    for work in leon_ipi_work_per_cpu.iter_mut() { *work = leon_ipi_work { single:0, msk:0, resched:0 }; }
}
unsafe fn leon_send_ipi(cpu:i32, level:i32) { LEON3_BYPASS_STORE_PA(&mut (*leon3_irqctrl_regs).force[cpu as usize], leon_get_irqmask(level)); }
unsafe fn leon_ipi_single(cpu:i32) { leon_ipi_work_per_cpu[cpu as usize].single=1; leon_send_ipi(cpu,leon_ipi_irq); }
unsafe fn leon_ipi_mask_one(cpu:i32) { leon_ipi_work_per_cpu[cpu as usize].msk=1; leon_send_ipi(cpu,leon_ipi_irq); }
unsafe fn leon_ipi_resched(cpu:i32) { leon_ipi_work_per_cpu[cpu as usize].resched=1; leon_send_ipi(cpu,leon_ipi_irq); }

#[no_mangle] pub unsafe extern "C" fn leonsmp_ipi_interrupt() { let w=&mut leon_ipi_work_per_cpu[smp_processor_id() as usize]; if w.single!=0 {w.single=0;smp_call_function_single_interrupt();} if w.msk!=0 {w.msk=0;smp_call_function_interrupt();} if w.resched!=0 {w.resched=0;smp_resched_interrupt();} }

#[repr(C)] struct smp_funcall { func: *mut core::ffi::c_void, arg1: core::ffi::c_ulong, arg2: core::ffi::c_ulong, arg3: core::ffi::c_ulong, arg4: core::ffi::c_ulong, arg5: core::ffi::c_ulong, processors_in:[core::ffi::c_ulong;NR_CPUS], processors_out:[core::ffi::c_ulong;NR_CPUS] }
static mut ccall_info:smp_funcall=smp_funcall{func:core::ptr::null_mut(),arg1:0,arg2:0,arg3:0,arg4:0,arg5:0,processors_in:[0;NR_CPUS],processors_out:[0;NR_CPUS]};
unsafe fn leon_cross_call(func:*mut core::ffi::c_void, mut mask:cpumask_t,arg1:core::ffi::c_ulong,arg2:core::ffi::c_ulong,arg3:core::ffi::c_ulong,arg4:core::ffi::c_ulong) { if smp_processors_ready!=0 { let me=smp_processor_id(); spin_lock_irqsave(core::ptr::null_mut(),core::ptr::null_mut()); ccall_info.func=func; ccall_info.arg1=arg1;ccall_info.arg2=arg2;ccall_info.arg3=arg3;ccall_info.arg4=arg4;ccall_info.arg5=0; cpumask_clear_cpu(me,&mut mask); for i in 0..NR_CPUS as i32 { if cpumask_test_cpu(i,&mask) { ccall_info.processors_in[i as usize]=0;ccall_info.processors_out[i as usize]=0;leon_send_ipi(i,LEON3_IRQ_CROSS_CALL); } } for i in 0..NR_CPUS as i32 { if cpumask_test_cpu(i,&mask) { while ccall_info.processors_in[i as usize]==0 {barrier();} while ccall_info.processors_out[i as usize]==0 {barrier();} } } spin_unlock_irqrestore(core::ptr::null_mut(),0); } }
#[no_mangle] pub unsafe extern "C" fn leon_cross_call_irq() { let i=smp_processor_id() as usize; ccall_info.processors_in[i]=1; let f:extern "C" fn(core::ffi::c_ulong,core::ffi::c_ulong,core::ffi::c_ulong,core::ffi::c_ulong,core::ffi::c_ulong)=core::mem::transmute(ccall_info.func); f(ccall_info.arg1,ccall_info.arg2,ccall_info.arg3,ccall_info.arg4,ccall_info.arg5); ccall_info.processors_out[i]=1; }
#[no_mangle] pub unsafe extern "C" fn leon_init_smp() { t_nmi[1]=t_nmi[1]+(linux_trap_ipi15_leon-linux_trap_ipi15_sun4m); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
