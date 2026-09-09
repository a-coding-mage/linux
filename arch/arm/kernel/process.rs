// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/kernel/process.c
 * Copyright (C) 1996-2000 Russell King - Converted to ARM.
 * Original Copyright (C) 1995 Linus Torvalds
 */

// Linux and ARM headers are dependencies supplied by the surrounding tree.

#[cfg(any(feature = "CONFIG_CURRENT_POINTER_IN_TPIDRURO", feature = "CONFIG_SMP"))]
extern "C" {
    pub static mut __entry_task: *mut task_struct;
}

#[cfg(all(feature = "CONFIG_STACKPROTECTOR", not(feature = "CONFIG_STACKPROTECTOR_PER_TASK")))]
#[no_mangle] pub static mut __stack_chk_guard: c_ulong = 0;

#[cfg(not(feature = "CONFIG_CURRENT_POINTER_IN_TPIDRURO"))]
#[no_mangle] pub static mut __current: *mut task_struct = core::ptr::null_mut();

static PROCESSOR_MODES: [&[u8]; 32] = [
    b"USER_26", b"FIQ_26", b"IRQ_26", b"SVC_26", b"UK4_26", b"UK5_26", b"UK6_26", b"UK7_26",
    b"UK8_26", b"UK9_26", b"UK10_26", b"UK11_26", b"UK12_26", b"UK13_26", b"UK14_26", b"UK15_26",
    b"USER_32", b"FIQ_32", b"IRQ_32", b"SVC_32", b"UK4_32", b"UK5_32", b"MON_32", b"ABT_32",
    b"UK8_32", b"UK9_32", b"HYP_32", b"UND_32", b"UK12_32", b"UK13_32", b"UK14_32", b"SYS_32",
];
static ISA_MODES: [&[u8]; 4] = [b"ARM", b"Thumb", b"Jazelle", b"ThumbEE"];

extern "C" {
    static mut arm_pm_idle: Option<unsafe extern "C" fn()>;
}

#[no_mangle]
pub unsafe extern "C" fn arch_cpu_idle() { if let Some(f) = arm_pm_idle { f(); } else { cpu_do_idle(); } }
#[no_mangle] pub unsafe extern "C" fn arch_cpu_idle_prepare() { local_fiq_enable(); }
#[no_mangle] pub unsafe extern "C" fn arch_cpu_idle_enter() {
    ledtrig_cpu(CPU_LED_IDLE_START);
    #[cfg(feature = "CONFIG_PL310_ERRATA_769419")] wmb();
}
#[no_mangle] pub unsafe extern "C" fn arch_cpu_idle_exit() { ledtrig_cpu(CPU_LED_IDLE_END); }

#[no_mangle]
pub unsafe extern "C" fn __show_regs_alloc_free(regs: *mut pt_regs) {
    for i in 0..13 { pr_alert(b"Register r%d information:\0".as_ptr(), i); mem_dump_obj((*regs).uregs[i] as *mut core::ffi::c_void); }
}

#[no_mangle]
pub unsafe extern "C" fn __show_regs(regs: *mut pt_regs) {
    let mut flags: c_ulong;
    let mut buf = [0u8; 64];
    #[cfg(not(feature = "CONFIG_CPU_V7M"))]
    let domain: c_uint = {
        #[cfg(feature = "CONFIG_CPU_SW_DOMAIN_PAN")]
        { if user_mode(regs) { DACR_UACCESS_ENABLE } else { (*to_svc_pt_regs(regs)).dacr } }
        #[cfg(not(feature = "CONFIG_CPU_SW_DOMAIN_PAN"))]
        { get_domain() }
    };
    show_regs_print_info(KERN_DEFAULT);
    printk(b"PC is at %pS\0".as_ptr(), instruction_pointer(regs) as *mut _);
    printk(b"LR is at %pS\0".as_ptr(), (*regs).ARM_lr as *mut _);
    printk(b"pc : [<%08lx>]    lr : [<%08lx>]    psr: %08lx\0".as_ptr(), (*regs).ARM_pc, (*regs).ARM_lr, (*regs).ARM_cpsr);
    printk(b"sp : %08lx  ip : %08lx  fp : %08lx\0".as_ptr(), (*regs).ARM_sp, (*regs).ARM_ip, (*regs).ARM_fp);
    printk(b"r10: %08lx  r9 : %08lx  r8 : %08lx\0".as_ptr(), (*regs).ARM_r10, (*regs).ARM_r9, (*regs).ARM_r8);
    printk(b"r7 : %08lx  r6 : %08lx  r5 : %08lx  r4 : %08lx\0".as_ptr(), (*regs).ARM_r7, (*regs).ARM_r6, (*regs).ARM_r5, (*regs).ARM_r4);
    printk(b"r3 : %08lx  r2 : %08lx  r1 : %08lx  r0 : %08lx\0".as_ptr(), (*regs).ARM_r3, (*regs).ARM_r2, (*regs).ARM_r1, (*regs).ARM_r0);
    flags = (*regs).ARM_cpsr; buf[0] = if flags & PSR_N_BIT != 0 { b'N' } else { b'n' }; buf[1] = if flags & PSR_Z_BIT != 0 { b'Z' } else { b'z' }; buf[2] = if flags & PSR_C_BIT != 0 { b'C' } else { b'c' }; buf[3] = if flags & PSR_V_BIT != 0 { b'V' } else { b'v' }; buf[4] = 0;
    #[cfg(not(feature = "CONFIG_CPU_V7M"))]
    { let segment = if (domain & domain_mask(DOMAIN_USER)) == domain_val(DOMAIN_USER, DOMAIN_NOACCESS) { b"none\0" } else { b"user\0" }; printk(b"Flags: %s  IRQs o%s  FIQs o%s  Mode %s  ISA %s  Segment %s\0".as_ptr(), buf.as_ptr(), if interrupts_enabled(regs) { b"n\0" } else { b"ff\0" }, if fast_interrupts_enabled(regs) { b"n\0" } else { b"ff\0" }, PROCESSOR_MODES[processor_mode(regs)] .as_ptr(), ISA_MODES[isa_mode(regs)].as_ptr(), segment.as_ptr()); }
    #[cfg(feature = "CONFIG_CPU_V7M")] printk(b"xPSR: %08lx\0".as_ptr(), (*regs).ARM_cpsr);
}

#[no_mangle] pub unsafe extern "C" fn show_regs(regs: *mut pt_regs) { __show_regs(regs); dump_backtrace(regs, core::ptr::null_mut(), KERN_DEFAULT); }

extern "C" { static mut thread_notify_head: atomic_notifier_head; }

#[no_mangle] pub unsafe extern "C" fn exit_thread(tsk: *mut task_struct) { thread_notify(THREAD_NOTIFY_EXIT, task_thread_info(tsk)); }
#[no_mangle] pub unsafe extern "C" fn flush_thread() { let thread = current_thread_info(); let tsk = current; flush_ptrace_hw_breakpoint(tsk); memset(&mut (*tsk).thread.debug as *mut _ as *mut _, 0, core::mem::size_of::<debug_info>()); memset(&mut (*thread).fpstate as *mut _ as *mut _, 0, core::mem::size_of::<fp_state>()); flush_tls(); thread_notify(THREAD_NOTIFY_FLUSH, thread); }

extern "C" { fn ret_from_fork(); }

#[no_mangle]
pub unsafe extern "C" fn copy_thread(p: *mut task_struct, args: *const kernel_clone_args) -> c_int {
    let clone_flags = (*args).flags; let stack_start = (*args).stack; let tls = (*args).tls; let thread = task_thread_info(p); let childregs = task_pt_regs(p);
    memset(&mut (*thread).cpu_context as *mut _ as *mut _, 0, core::mem::size_of::<cpu_context_save>());
    #[cfg(feature = "CONFIG_CPU_USE_DOMAINS")] { (*thread).cpu_domain = get_domain(); }
    if (*args).fn_.is_none() { *childregs = *current_pt_regs(); (*childregs).ARM_r0 = 0; if stack_start != 0 { (*childregs).ARM_sp = stack_start; } } else { memset(childregs as *mut _, 0, core::mem::size_of::<pt_regs>()); (*thread).cpu_context.r4 = (*args).fn_arg as c_ulong; (*thread).cpu_context.r5 = (*args).fn_ as c_ulong; (*childregs).ARM_cpsr = SVC_MODE; }
    (*thread).cpu_context.pc = ret_from_fork as usize as c_ulong; (*thread).cpu_context.sp = childregs as c_ulong; clear_ptrace_hw_breakpoint(p); if clone_flags & CLONE_SETTLS != 0 { (*thread).tp_value[0] = tls; } (*thread).tp_value[1] = get_tpuser(); thread_notify(THREAD_NOTIFY_COPY, thread); 0
}

#[no_mangle]
pub unsafe extern "C" fn __get_wchan(p: *mut task_struct) -> c_ulong { let mut frame = stackframe { fp: thread_saved_fp(p), sp: thread_saved_sp(p), lr: 0, pc: thread_saved_pc(p) }; let stack_page = task_stack_page(p) as c_ulong; let mut count = 0; loop { if frame.sp < stack_page || frame.sp >= stack_page + THREAD_SIZE || unwind_frame(&mut frame) < 0 { return 0; } if !in_sched_functions(frame.pc) { return frame.pc; } count += 1; if count > 16 { return 0; } } }

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_KUSER_HELPERS"))]
static mut gate_vma: vm_area_struct = unsafe { core::mem::zeroed() };
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_KUSER_HELPERS"))]
#[no_mangle] pub unsafe extern "C" fn get_gate_vma(_: *mut mm_struct) -> *mut vm_area_struct { &mut gate_vma }
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_KUSER_HELPERS"))]
#[no_mangle] pub unsafe extern "C" fn in_gate_area(_: *mut mm_struct, addr: c_ulong) -> c_int { (addr >= gate_vma.vm_start && addr < gate_vma.vm_end) as c_int }
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_KUSER_HELPERS"))]
#[no_mangle] pub unsafe extern "C" fn in_gate_area_no_mm(addr: c_ulong) -> c_int { in_gate_area(core::ptr::null_mut(), addr) }
#[cfg(feature = "CONFIG_MMU")]
#[no_mangle] pub unsafe extern "C" fn arch_vma_name(vma: *mut vm_area_struct) -> *const u8 { #[cfg(feature = "CONFIG_KUSER_HELPERS")] if vma == &mut gate_vma { return b"[vectors]\0".as_ptr(); } core::ptr::null() }

// The remaining MMU sigpage/vDSO mapping declarations preserve the source-level
// interface; their Linux types and helpers are supplied by the surrounding tree.
#[cfg(feature = "CONFIG_MMU")]
extern "C" { static mut signal_page: *mut page; fn get_signal_page() -> *mut page; }

#[cfg(feature = "CONFIG_MMU")]
unsafe extern "C" fn sigpage_addr(mm: *const mm_struct, npages: c_uint) -> c_ulong {
    let first = PAGE_ALIGN((*mm).start_stack); let last = TASK_SIZE - (npages as c_ulong * PAGE_SIZE);
    if first > last { return 0; } if first == last { return first; }
    let slots = ((last - first) >> PAGE_SHIFT) + 1; first + (get_random_u32_below(slots as u32) as c_ulong << PAGE_SHIFT)
}

#[cfg(feature = "CONFIG_MMU")]
unsafe extern "C" fn sigpage_mremap(_: *const vm_special_mapping, new_vma: *mut vm_area_struct) -> c_int { (*current).mm.context.sigpage = (*new_vma).vm_start; 0 }

#[cfg(feature = "CONFIG_MMU")]
#[no_mangle]
pub unsafe extern "C" fn arch_setup_additional_pages(_: *mut linux_binprm, _: c_int) -> c_int {
    let mm = (*current).mm as *mut mm_struct;
    if signal_page.is_null() { signal_page = get_signal_page(); }
    if signal_page.is_null() { return -ENOMEM; }
    let npages = 1 + vdso_total_pages; if mmap_write_lock_killable(mm) != 0 { return -EINTR; }
    let hint = sigpage_addr(mm, npages); let addr = get_unmapped_area(core::ptr::null_mut(), hint, npages as c_ulong << PAGE_SHIFT, 0, 0);
    if IS_ERR_VALUE(addr) { mmap_write_unlock(mm); return addr as c_int; }
    let mapping = vm_special_mapping { name: b"[sigpage]\0".as_ptr(), pages: &mut signal_page, mremap: Some(sigpage_mremap) };
    let vma = _install_special_mapping(mm, addr, PAGE_SIZE, VM_READ | VM_EXEC | VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC, &mapping);
    if IS_ERR(vma) { let ret = PTR_ERR(vma); mmap_write_unlock(mm); return ret; }
    (*mm).context.sigpage = addr; arm_install_vdso(mm, addr + PAGE_SIZE); mmap_write_unlock(mm); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
