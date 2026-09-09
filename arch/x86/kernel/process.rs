// SPDX-License-Identifier: GPL-2.0
// Translated from process.c. Kernel headers and configuration provide the referenced items.

#[repr(C)]
pub struct TssStruct { pub x86_tss: X86Tss }
#[repr(C)] pub struct X86Tss { pub sp0: usize, pub io_bitmap_base: u16 }
#[repr(C)] pub struct SsbState { pub shared_state: *mut SsbState, pub lock: RawSpinlock, pub disable_state: u32, pub local_state: usize }
pub type RawSpinlock = [u8; 0];

pub const LSTATE_SSB: usize = 0;
pub const BOOT_OPTION_IDLE_OVERRIDE: usize = IDLE_NO_OVERRIDE;

#[no_mangle] pub static mut cpu_tss_rw: TssStruct = TssStruct { x86_tss: X86Tss { sp0: (1usize << (BITS_PER_LONG - 1)) + 1, io_bitmap_base: IO_BITMAP_OFFSET_INVALID } };
#[no_mangle] pub static mut __tss_limit_invalid: bool = false;
#[no_mangle] pub static mut cache_state_incoherent: bool = false;

pub unsafe fn arch_dup_task_struct(dst: *mut task_struct, src: *const task_struct) -> i32 {
    memcpy_and_pad(dst as *mut _, arch_task_struct_size, src as *const _, core::mem::size_of::<task_struct>(), 0);
    0
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn arch_release_task_struct(tsk: *mut task_struct) { if fpu_state_size_dynamic() && ((*tsk).flags & (PF_KTHREAD | PF_USER_WORKER)) == 0 { fpstate_free(x86_task_fpu(tsk)); } }

pub unsafe fn exit_thread(tsk: *mut task_struct) { if test_thread_flag(TIF_IO_BITMAP) { io_bitmap_exit(tsk); } free_vm86(&mut (*tsk).thread); shstk_free(tsk); fpu__drop(tsk); }

unsafe fn set_new_tls(p: *mut task_struct, tls: usize) -> i32 {
    let utls = tls as *mut user_desc;
    if in_ia32_syscall() { do_set_thread_area(p, -1, utls, 0) } else { do_set_thread_area_64(p, ARCH_SET_FS, tls) }
}

pub unsafe fn ret_from_fork(prev: *mut task_struct, regs: *mut pt_regs, f: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>, arg: *mut core::ffi::c_void) {
    schedule_tail(prev); if let Some(fun) = f { fun(arg); (*regs).ax = 0; } syscall_exit_to_user_mode(regs);
}

pub unsafe fn copy_thread(p: *mut task_struct, args: *const kernel_clone_args) -> i32 {
    let clone_flags = (*args).flags; let sp = (*args).stack; let tls = (*args).tls;
    let childregs = task_pt_regs(p); let fork_frame = container_of(childregs, fork_frame, regs); let frame = &mut (*fork_frame).frame;
    (*frame).bp = encode_frame_pointer(childregs); (*frame).ret_addr = ret_from_fork_asm as usize;
    (*p).thread.sp = fork_frame as usize; (*p).thread.io_bitmap = core::ptr::null_mut(); clear_tsk_thread_flag(p, TIF_IO_BITMAP); (*p).thread.iopl_warn = 0; memset((*p).thread.ptrace_bps.as_mut_ptr() as *mut _, 0, core::mem::size_of_val(&(*p).thread.ptrace_bps));
    #[cfg(target_arch="x86_64")] { current_save_fsgs(); (*p).thread.fsindex=current.thread.fsindex; (*p).thread.fsbase=current.thread.fsbase; (*p).thread.gsindex=current.thread.gsindex; (*p).thread.gsbase=current.thread.gsbase; savesegment!(es, (*p).thread.es); savesegment!(ds, (*p).thread.ds); if !(*p).mm.is_null() && (clone_flags & (CLONE_VM|CLONE_VFORK)) == CLONE_VM { set_bit(MM_CONTEXT_LOCK_LAM, &mut (*(*p).mm).context.flags); } }
    let new_ssp = shstk_alloc_thread_stack(p, clone_flags, (*args).stack_size); if IS_ERR_VALUE(new_ssp) { return PTR_ERR(new_ssp as *mut _); } fpu_clone(p, clone_flags, (*args).fn_, new_ssp);
    if ((*p).flags & PF_KTHREAD) != 0 { (*p).thread.pkru=pkru_get_init_value(); memset(childregs as *mut _,0,core::mem::size_of::<pt_regs>()); kthread_frame_init(frame, (*args).fn_, (*args).fn_arg); return 0; }
    (*p).thread.pkru=read_pkru(); (*frame).bx=0; *childregs=*current_pt_regs(); (*childregs).ax=0; if sp != 0 { (*childregs).sp=sp; }
    if (*args).fn_.is_some() { (*childregs).sp=0; (*childregs).ip=0; kthread_frame_init(frame, (*args).fn_, (*args).fn_arg); return 0; }
    let mut ret=0; if (clone_flags & CLONE_SETTLS)!=0 { ret=set_new_tls(p,tls); } if ret==0 && test_tsk_thread_flag(current, TIF_IO_BITMAP) { io_bitmap_share(p); } ret
}

unsafe fn pkru_flush_thread() { pkru_write_default(); }
pub unsafe fn flush_thread() { let tsk=current; flush_ptrace_hw_breakpoint(tsk); memset((*tsk).thread.tls_array.as_mut_ptr() as *mut _,0,core::mem::size_of_val(&(*tsk).thread.tls_array)); fpu_flush_thread(); pkru_flush_thread(); }
pub unsafe fn disable_TSC() { preempt_disable(); if !test_and_set_thread_flag(TIF_NOTSC) { cr4_set_bits(X86_CR4_TSD); } preempt_enable(); }
unsafe fn enable_TSC() { preempt_disable(); if test_and_clear_thread_flag(TIF_NOTSC) { cr4_clear_bits(X86_CR4_TSD); } preempt_enable(); }
pub unsafe fn get_tsc_mode(adr: usize) -> i32 { let val=if test_thread_flag(TIF_NOTSC){PR_TSC_SIGSEGV}else{PR_TSC_ENABLE}; put_user(val,adr as *mut u32) }
pub unsafe fn set_tsc_mode(val:u32)->i32 { if val==PR_TSC_SIGSEGV{disable_TSC()}else if val==PR_TSC_ENABLE{enable_TSC()}else{return -EINVAL};0 }

pub unsafe fn default_idle() { raw_safe_halt(); raw_local_irq_disable(); }
unsafe fn x86_idle_set()->bool { static_call_query(x86_idle) }
pub unsafe fn arch_cpu_idle_enter(){tsc_verify_tsc_adjust(false);local_touch_nmi();}
pub unsafe fn arch_cpu_idle_dead()->!{play_dead()}
pub unsafe fn arch_cpu_idle(){static_call!(x86_idle)();}
pub unsafe fn stop_this_cpu(_dummy:*mut core::ffi::c_void)->! { let cpu=smp_processor_id(); local_irq_disable(); set_cpu_online(cpu,false); disable_local_APIC(); mcheck_cpu_clear(this_cpu_ptr(&cpu_info)); if this_cpu_read(cache_state_incoherent){wbinvd();} cpumask_clear_cpu(cpu,&mut cpus_stop_mask); loop{native_halt();} }

unsafe fn prefer_mwait_c1_over_halt()->bool { let c=&boot_cpu_data; if boot_option_idle_override!=IDLE_NO_OVERRIDE || !cpu_has(c,X86_FEATURE_MWAIT) || boot_cpu_has_bug(X86_BUG_MONITOR)||boot_cpu_has_bug(X86_BUG_AMD_APIC_C1E){return false;} let (eax,ebx,ecx,edx)=cpuid(CPUID_LEAF_MWAIT); let _=(eax,ebx); if (ecx&CPUID5_ECX_EXTENSIONS_SUPPORTED)==0 {true}else{(edx&MWAIT_C1_SUBSTATE_MASK)!=0} }
pub unsafe fn select_idle_routine(){ if boot_option_idle_override==IDLE_POLL{return;} if x86_idle_set(){return;} if prefer_mwait_c1_over_halt(){static_call_update(x86_idle,mwait_idle);}else if cpu_feature_enabled(X86_FEATURE_TDX_GUEST){static_call_update(x86_idle,tdx_halt);}else{static_call_update(x86_idle,default_idle);} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
