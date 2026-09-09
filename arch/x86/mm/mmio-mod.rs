// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) IBM Corporation, 2005
//              Jeff Muizelaar, 2006, 2007
//              Pekka Paalanen, 2008 <pq@iki.fi>
//
// Derived from the read-mod example from relay-examples by Tom Zanussi.

// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
struct TrapReason {
    addr: c_ulong,
    ip: c_ulong,
    type_: reason_type,
    active_traces: c_int,
}

#[repr(C)]
struct RemapTrace {
    list: list_head,
    probe: kmmio_probe,
    phys: resource_size_t,
    id: c_ulong,
}

// Accessed per-cpu.
static mut PF_REASON: TrapReason = TrapReason {
    addr: 0,
    ip: 0,
    type_: 0 as reason_type,
    active_traces: 0,
};
static mut CPU_TRACE: mmiotrace_rw = unsafe { core::mem::zeroed() };

static mut MMIOTRACE_MUTEX: mutex = unsafe { core::mem::zeroed() };
static mut TRACE_LOCK: spinlock = unsafe { core::mem::zeroed() };
static mut MMIOTRACE_ENABLED: atomic_t = atomic_t { counter: 0 };
static mut TRACE_LIST: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

// Module parameters.
static mut FILTER_OFFSET: c_ulong = 0;
static mut NOMMIOTRACE: bool = false;
static mut TRACE_PC: bool = false;

unsafe fn is_enabled() -> bool {
    atomic_read(&MMIOTRACE_ENABLED) != 0
}

unsafe fn print_pte(address: c_ulong) {
    let mut level: c_uint = 0;
    let pte = lookup_address(address, &mut level);
    if pte.is_null() {
        pr_err!("Error in print_pte: no pte for page 0x{:08x}\n", address);
        return;
    }
    if level == PG_LEVEL_2M {
        pr_emerg!("4MB pages are not currently supported: 0x{:08x}\n", address);
        BUG!();
    }
    pr_info!("pte for 0x{:x}: 0x{:x} 0x{:x}\n", address,
        pte_val(*pte) as c_ulonglong,
        (pte_val(*pte) as c_ulonglong) & _PAGE_PRESENT as c_ulonglong);
}

unsafe fn die_kmmio_nesting_error(regs: *mut pt_regs, addr: c_ulong) -> ! {
    let my_reason = &PF_REASON;
    pr_emerg!("unexpected fault for address: 0x{:08x}, last fault for address: 0x{:08x}\n", addr, my_reason.addr);
    print_pte(addr);
    pr_emerg!("faulting IP is at {:p}\n", (*regs).ip as *const c_void);
    pr_emerg!("last faulting IP was at {:p}\n", my_reason.ip as *const c_void);
    #[cfg(target_arch = "x86")]
    {
        pr_emerg!("eax: {:08x}   ebx: {:08x}   ecx: {:08x}   edx: {:08x}\n", (*regs).ax, (*regs).bx, (*regs).cx, (*regs).dx);
        pr_emerg!("esi: {:08x}   edi: {:08x}   ebp: {:08x}   esp: {:08x}\n", (*regs).si, (*regs).di, (*regs).bp, (*regs).sp);
    }
    #[cfg(target_arch = "x86_64")]
    {
        pr_emerg!("rax: {:016x}   rcx: {:016x}   rdx: {:016x}\n", (*regs).ax, (*regs).cx, (*regs).dx);
        pr_emerg!("rsi: {:016x}   rdi: {:016x}   rbp: {:016x}   rsp: {:016x}\n", (*regs).si, (*regs).di, (*regs).bp, (*regs).sp);
    }
    BUG!();
}

unsafe extern "C" fn pre(p: *mut kmmio_probe, regs: *mut pt_regs, addr: c_ulong) {
    let trace = (*p).private as *mut RemapTrace;
    if PF_REASON.active_traces != 0 { die_kmmio_nesting_error(regs, addr); }
    PF_REASON.active_traces += 1;
    let instptr = instruction_pointer(regs);
    let type_ = get_ins_type(instptr);
    PF_REASON.type_ = type_; PF_REASON.addr = addr; PF_REASON.ip = instptr;
    CPU_TRACE.phys = addr.wrapping_sub((*trace).probe.addr).wrapping_add((*trace).phys);
    CPU_TRACE.map_id = (*trace).id;
    CPU_TRACE.pc = if TRACE_PC { instptr } else { 0 };
    match type_ {
        REG_READ => { CPU_TRACE.opcode = MMIO_READ; CPU_TRACE.width = get_ins_mem_width(instptr); }
        REG_WRITE => { CPU_TRACE.opcode = MMIO_WRITE; CPU_TRACE.width = get_ins_mem_width(instptr); CPU_TRACE.value = get_ins_reg_val(instptr, regs); }
        IMM_WRITE => { CPU_TRACE.opcode = MMIO_WRITE; CPU_TRACE.width = get_ins_mem_width(instptr); CPU_TRACE.value = get_ins_imm_val(instptr); }
        _ => { let ip = instptr as *const u8; CPU_TRACE.opcode = MMIO_UNKNOWN_OP; CPU_TRACE.width = 0; CPU_TRACE.value = ((*ip as u32) << 16) | ((*ip.add(1) as u32) << 8) | (*ip.add(2) as u32); }
    }
}

unsafe extern "C" fn post(_p: *mut kmmio_probe, _condition: c_ulong, regs: *mut pt_regs) {
    PF_REASON.active_traces -= 1;
    if PF_REASON.active_traces != 0 { pr_emerg!("unexpected post handler"); BUG!(); }
    if PF_REASON.type_ == REG_READ { CPU_TRACE.value = get_ins_reg_val(PF_REASON.ip, regs); }
    mmio_trace_rw(&mut CPU_TRACE);
}

unsafe fn ioremap_trace_core(offset: resource_size_t, size: c_ulong, addr: *mut c_void) {
    static mut NEXT_ID: atomic_t = atomic_t { counter: 0 };
    let trace = kmalloc_obj::<RemapTrace>();
    let mut map = mmiotrace_map { phys: offset, virt: addr as c_ulong, len: size, opcode: MMIO_PROBE, map_id: 0 };
    if trace.is_null() { pr_err!("kmalloc failed in ioremap\n"); return; }
    (*trace).probe = kmmio_probe { addr: addr as c_ulong, len: size, pre_handler: Some(pre), post_handler: Some(post), private: trace as *mut c_void };
    (*trace).phys = offset; (*trace).id = atomic_inc_return(&mut NEXT_ID) as c_ulong; map.map_id = (*trace).id;
    spin_lock_irq(&mut TRACE_LOCK);
    if !is_enabled() { kfree(trace); spin_unlock_irq(&mut TRACE_LOCK); return; }
    mmio_trace_mapping(&mut map); list_add_tail(&mut (*trace).list, &mut TRACE_LIST);
    if !NOMMIOTRACE { register_kmmio_probe(&mut (*trace).probe); }
    spin_unlock_irq(&mut TRACE_LOCK);
}

#[no_mangle]
pub unsafe extern "C" fn mmiotrace_ioremap(offset: resource_size_t, size: c_ulong, addr: *mut c_void) {
    if !is_enabled() { return; }
    pr_debug!("ioremap_*(0x{:x}, 0x{:x}) = {:p}\n", offset, size, addr);
    if FILTER_OFFSET != 0 && offset != FILTER_OFFSET as resource_size_t { return; }
    ioremap_trace_core(offset, size, addr);
}

unsafe fn iounmap_trace_core(addr: *mut c_void) {
    let mut map = mmiotrace_map { phys: 0, virt: addr as c_ulong, len: 0, opcode: MMIO_UNPROBE, map_id: 0 };
    let mut found: *mut RemapTrace = core::ptr::null_mut();
    pr_debug!("Unmapping {:p}.\n", addr);
    spin_lock_irq(&mut TRACE_LOCK);
    if is_enabled() {
        let mut trace = trace_list_first(&mut TRACE_LIST);
        while !trace.is_null() {
            if addr as c_ulong == (*trace).probe.addr {
                if !NOMMIOTRACE { unregister_kmmio_probe(&mut (*trace).probe); }
                list_del(&mut (*trace).list); found = trace; break;
            }
            trace = trace_list_next(trace);
        }
        map.map_id = if !found.is_null() { (*found).id } else { c_ulong::MAX };
        mmio_trace_mapping(&mut map);
    }
    spin_unlock_irq(&mut TRACE_LOCK);
    if !found.is_null() { synchronize_rcu(); kfree(found); }
}

#[no_mangle]
pub unsafe extern "C" fn mmiotrace_iounmap(addr: *mut c_void) { might_sleep(); if is_enabled() { iounmap_trace_core(addr); } }

#[no_mangle]
pub unsafe extern "C" fn mmiotrace_printk(fmt: *const c_char, mut args: ...) -> c_int {
    let mut ret = 0; let mut flags = 0;
    spin_lock_irqsave(&mut TRACE_LOCK, &mut flags);
    if is_enabled() { ret = mmio_trace_printk(fmt, &mut args); }
    spin_unlock_irqrestore(&mut TRACE_LOCK, flags); ret
}

unsafe fn clear_trace_list() {
    let mut trace = trace_list_first(&mut TRACE_LIST);
    while !trace.is_null() { pr_notice!("purging non-iounmapped trace @0x{:08x}, size 0x{:x}.\n", (*trace).probe.addr, (*trace).probe.len); if !NOMMIOTRACE { unregister_kmmio_probe(&mut (*trace).probe); } trace = trace_list_next(trace); }
    synchronize_rcu();
    let mut trace = trace_list_first(&mut TRACE_LIST);
    while !trace.is_null() { let next = trace_list_next(trace); list_del(&mut (*trace).list); kfree(trace); trace = next; }
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
static mut DOWNED_CPUS: cpumask_var_t = core::ptr::null_mut();

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn enter_uniprocessor() {
    let mut cpu: c_int;
    let mut err: c_int;
    if !cpumask_available(DOWNED_CPUS) && !alloc_cpumask_var(&mut DOWNED_CPUS, GFP_KERNEL) {
        pr_notice!("Failed to allocate mask\n");
    } else {
        cpus_read_lock(); cpumask_copy(DOWNED_CPUS, cpu_online_mask());
        cpumask_clear_cpu(cpumask_first(cpu_online_mask()), DOWNED_CPUS);
        if num_online_cpus() > 1 { pr_notice!("Disabling non-boot CPUs...\n"); }
        cpus_read_unlock();
        for_each_cpu!(cpu, DOWNED_CPUS) {
            err = remove_cpu(cpu);
            if err == 0 { pr_info!("CPU{} is down.\n", cpu); }
            else { pr_err!("Error taking CPU{} down: {}\n", cpu, err); }
        }
    }
    if num_online_cpus() > 1 { pr_warn!("multiple CPUs still online, may miss events.\n"); }
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn leave_uniprocessor() {
    if !cpumask_available(DOWNED_CPUS) || cpumask_empty(DOWNED_CPUS) { return; }
    pr_notice!("Re-enabling CPUs...\n");
    let mut cpu: c_int;
    for_each_cpu!(cpu, DOWNED_CPUS) {
        let err = add_cpu(cpu);
        if err == 0 { pr_info!("enabled CPU{}.\n", cpu); }
        else { pr_err!("cannot re-enable CPU{}: {}\n", cpu, err); }
    }
}

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
unsafe fn enter_uniprocessor() { if num_online_cpus() > 1 { pr_warn!("multiple CPUs are online, may miss events. Suggest booting with maxcpus=1 kernel argument.\n"); } }
#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
unsafe fn leave_uniprocessor() {}

#[no_mangle]
pub unsafe extern "C" fn enable_mmiotrace() {
    mutex_lock(&mut MMIOTRACE_MUTEX); if is_enabled() { mutex_unlock(&mut MMIOTRACE_MUTEX); return; }
    if NOMMIOTRACE { pr_info!("MMIO tracing disabled.\n"); } kmmio_init(); enter_uniprocessor(); spin_lock_irq(&mut TRACE_LOCK); atomic_inc(&mut MMIOTRACE_ENABLED); spin_unlock_irq(&mut TRACE_LOCK); pr_info!("enabled.\n"); mutex_unlock(&mut MMIOTRACE_MUTEX);
}

#[no_mangle]
pub unsafe extern "C" fn disable_mmiotrace() {
    mutex_lock(&mut MMIOTRACE_MUTEX); if !is_enabled() { mutex_unlock(&mut MMIOTRACE_MUTEX); return; }
    spin_lock_irq(&mut TRACE_LOCK); atomic_dec(&mut MMIOTRACE_ENABLED); BUG_ON!(is_enabled()); spin_unlock_irq(&mut TRACE_LOCK); clear_trace_list(); leave_uniprocessor(); kmmio_cleanup(); pr_info!("disabled.\n"); mutex_unlock(&mut MMIOTRACE_MUTEX);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
