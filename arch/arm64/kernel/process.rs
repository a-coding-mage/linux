// SPDX-License-Identifier: GPL-2.0-only
/* Direct translation of the arm64 kernel process implementation. */

#[cfg(all(feature = "CONFIG_STACKPROTECTOR", not(feature = "CONFIG_STACKPROTECTOR_PER_TASK")))]
#[no_mangle]
pub static mut __stack_chk_guard: c_ulong = 0;

// Optional machine specific power-off hook.
#[no_mangle]
pub static mut pm_power_off: Option<unsafe extern "C" fn()> = None;

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
pub unsafe extern "C" fn arch_cpu_idle_dead() -> ! { cpu_die(); }

pub unsafe extern "C" fn machine_shutdown() { smp_shutdown_nonboot_cpus(reboot_cpu); }

pub unsafe extern "C" fn machine_halt() -> ! {
    local_irq_disable(); smp_send_stop(); loop {}
}

pub unsafe extern "C" fn machine_power_off() -> ! {
    local_irq_disable(); smp_send_stop(); do_kernel_power_off(); loop {}
}

pub unsafe extern "C" fn machine_restart(cmd: *mut c_char) -> ! {
    local_irq_disable(); smp_send_stop();
    if efi_enabled(EFI_RUNTIME_SERVICES) { efi_reboot(reboot_mode, core::ptr::null_mut()); }
    do_kernel_restart(cmd);
    printk(c"Reboot failed -- System halted\n".as_ptr());
    loop {}
}

// bstr(NONE, "--"), bstr(JC, "jc"), bstr(C, "-c"), bstr(J, "j-")
static BTYPES: [&[u8]; 4] = [b"--", b"jc", b"-c", b"j-"];

unsafe fn print_pstate(regs: *mut pt_regs) {
    let pstate = (*regs).pstate;
    if compat_user_mode(regs) {
        printk(c"pstate: %08llx (%c%c%c%c %c %s %s %c%c%c %cDIT %cSSBS)\n".as_ptr(), pstate,
            if pstate & PSR_AA32_N_BIT != 0 {'N'} else {'n'}, if pstate & PSR_AA32_Z_BIT != 0 {'Z'} else {'z'},
            if pstate & PSR_AA32_C_BIT != 0 {'C'} else {'c'}, if pstate & PSR_AA32_V_BIT != 0 {'V'} else {'v'},
            if pstate & PSR_AA32_Q_BIT != 0 {'Q'} else {'q'}, if pstate & PSR_AA32_T_BIT != 0 {c"T32".as_ptr()} else {c"A32".as_ptr()},
            if pstate & PSR_AA32_E_BIT != 0 {c"BE".as_ptr()} else {c"LE".as_ptr()}, if pstate & PSR_AA32_A_BIT != 0 {'A'} else {'a'},
            if pstate & PSR_AA32_I_BIT != 0 {'I'} else {'i'}, if pstate & PSR_AA32_F_BIT != 0 {'F'} else {'f'},
            if pstate & PSR_AA32_DIT_BIT != 0 {'+'} else {'-'}, if pstate & PSR_AA32_SSBS_BIT != 0 {'+'} else {'-' });
    } else {
        let btype_str = BTYPES[((pstate & PSR_BTYPE_MASK) >> PSR_BTYPE_SHIFT) as usize].as_ptr();
        printk(c"pstate: %08llx (%c%c%c%c %c%c%c%c %cPAN %cUAO %cTCO %cDIT %cSSBS BTYPE=%s)\n".as_ptr(), pstate,
            if pstate & PSR_N_BIT != 0 {'N'} else {'n'}, if pstate & PSR_Z_BIT != 0 {'Z'} else {'z'},
            if pstate & PSR_C_BIT != 0 {'C'} else {'c'}, if pstate & PSR_V_BIT != 0 {'V'} else {'v'},
            if pstate & PSR_D_BIT != 0 {'D'} else {'d'}, if pstate & PSR_A_BIT != 0 {'A'} else {'a'},
            if pstate & PSR_I_BIT != 0 {'I'} else {'i'}, if pstate & PSR_F_BIT != 0 {'F'} else {'f'},
            if pstate & PSR_PAN_BIT != 0 {'+'} else {'-'}, if pstate & PSR_UAO_BIT != 0 {'+'} else {'-'},
            if pstate & PSR_TCO_BIT != 0 {'+'} else {'-'}, if pstate & PSR_DIT_BIT != 0 {'+'} else {'-'},
            if pstate & PSR_SSBS_BIT != 0 {'+'} else {'-'}, btype_str);
    }
}

pub unsafe extern "C" fn __show_regs(regs: *mut pt_regs) {
    let (lr, sp, top_reg) = if compat_user_mode(regs) { ((*regs).compat_lr, (*regs).compat_sp, 12) } else { ((*regs).regs[30], (*regs).sp, 29) };
    show_regs_print_info(KERN_DEFAULT); print_pstate(regs);
    if !user_mode(regs) { printk(c"pc : %pS\n".as_ptr(), (*regs).pc as *mut c_void); printk(c"lr : %pS\n".as_ptr(), ptrauth_strip_kernel_insn_pac(lr) as *mut c_void); }
    else { printk(c"pc : %016llx\n".as_ptr(), (*regs).pc); printk(c"lr : %016llx\n".as_ptr(), lr); }
    printk(c"sp : %016llx\n".as_ptr(), sp);
    if system_uses_irq_prio_masking() { printk(c"pmr: %08x\n".as_ptr(), (*regs).pmr); }
    let mut i = top_reg; while i >= 0 { printk(c"x%-2d: %016llx".as_ptr(), i, (*regs).regs[i as usize]); i -= 1; while i >= 0 && i % 3 != 0 { pr_cont(c" x%-2d: %016llx".as_ptr(), i, (*regs).regs[i as usize]); i -= 1; } pr_cont(c"\n".as_ptr()); }
}
pub unsafe extern "C" fn show_regs(regs: *mut pt_regs) { __show_regs(regs); dump_backtrace(regs, core::ptr::null_mut(), KERN_DEFAULT); }

unsafe fn tls_thread_flush() { write_sysreg(0, tpidr_el0); if system_supports_tpidr2() { write_sysreg_s(0, SYS_TPIDR2_EL0); } if is_compat_task() { (*current).thread.uw.tp_value = 0; barrier(); write_sysreg(0, tpidrro_el0); } }
unsafe fn flush_tagged_addr_state() { if IS_ENABLED_CONFIG_ARM64_TAGGED_ADDR_ABI { clear_thread_flag(TIF_TAGGED_ADDR); } }
unsafe fn flush_poe() { if system_supports_poe() { write_sysreg_s(POR_EL0_INIT, SYS_POR_EL0); } }

#[cfg(feature = "CONFIG_ARM64_GCS")]
unsafe fn flush_gcs() { if system_supports_gcs() { (*current).thread.gcspr_el0=0; (*current).thread.gcs_base=0; (*current).thread.gcs_size=0; (*current).thread.gcs_el0_mode=0; (*current).thread.gcs_el0_locked=0; write_sysreg_s(GCSCRE0_EL1_nTR,SYS_GCSCRE0_EL1); write_sysreg_s(0,SYS_GCSPR_EL0); } }
#[cfg(not(feature = "CONFIG_ARM64_GCS"))] unsafe fn flush_gcs() {}

pub unsafe extern "C" fn flush_thread() { fpsimd_flush_thread(); tls_thread_flush(); flush_ptrace_hw_breakpoint(current); flush_tagged_addr_state(); flush_poe(); flush_gcs(); }
pub unsafe extern "C" fn arch_release_task_struct(tsk: *mut task_struct) { fpsimd_release_task(tsk); }

pub unsafe extern "C" fn arch_dup_task_struct(dst: *mut task_struct, src: *mut task_struct) -> c_int {
    fpsimd_save_and_flush_current_state(); fpsimd_sync_from_effective_state(src); *dst = *src;
    (*dst).thread.fp_type=FP_STATE_FPSIMD; (*dst).thread.sve_state=core::ptr::null_mut(); clear_tsk_thread_flag(dst,TIF_SVE); task_smstop_sm(dst);
    (*dst).thread.sme_state=core::ptr::null_mut(); clear_tsk_thread_flag(dst,TIF_SME); (*dst).thread.svcr &= !SVCR_ZA_MASK; clear_tsk_thread_flag(dst,TIF_MTE_ASYNC_FAULT); 0
}

unsafe fn copy_thread_za(dst: *mut task_struct, src: *mut task_struct) -> c_int {
    if !thread_za_enabled(&(*src).thread) { return 0; }
    (*dst).thread.sve_state=kzalloc(sve_state_size(src),GFP_KERNEL); if (*dst).thread.sve_state.is_null() { return -ENOMEM; }
    (*dst).thread.sme_state=kmemdup((*src).thread.sme_state,sme_state_size(src),GFP_KERNEL); if (*dst).thread.sme_state.is_null() { kfree((*dst).thread.sve_state); (*dst).thread.sve_state=core::ptr::null_mut(); return -ENOMEM; }
    set_tsk_thread_flag(dst,TIF_SME); (*dst).thread.svcr |= SVCR_ZA_MASK; 0
}

extern "C" { fn ret_from_fork(); }

pub unsafe extern "C" fn copy_thread(p: *mut task_struct, args: *const kernel_clone_args) -> c_int {
    let clone_flags=(*args).flags; let stack_start=(*args).stack; let tls=(*args).tls; let childregs=task_pt_regs(p); let mut ret;
    memset(&mut (*p).thread.cpu_context as *mut _,0,core::mem::size_of::<cpu_context>()); fpsimd_flush_task_state(p); ptrauth_thread_init_kernel(p);
    if likely((*args).fn_.is_null()) { *childregs=*current_pt_regs(); (*childregs).regs[0]=0; *task_user_tls(p)=read_sysreg(tpidr_el0); if system_supports_poe(){(*p).thread.por_el0=read_sysreg_s(SYS_POR_EL0);} if stack_start!=0 { if is_compat_thread(task_thread_info(p)){(*childregs).compat_sp=stack_start;}else{(*childregs).sp=stack_start;} } if system_supports_sme(){ if clone_flags&CLONE_VM==0 {(*p).thread.tpidr2_el0=read_sysreg_s(SYS_TPIDR2_EL0); ret=copy_thread_za(p,current); if ret!=0{return ret;}}else{(*p).thread.tpidr2_el0=0; WARN_ON_ONCE((*p).thread.svcr&SVCR_ZA_MASK!=0);} } if clone_flags&CLONE_SETTLS!=0 {(*p).thread.uw.tp_value=tls;} ret=copy_thread_gcs(p,args); if ret!=0{return ret;} }
    else { memset(childregs,0,core::mem::size_of::<pt_regs>()); (*childregs).pstate=PSR_MODE_EL1h|PSR_IL_BIT; (*childregs).stackframe.type_=FRAME_META_TYPE_FINAL; (*p).thread.cpu_context.x19=(*args).fn_ as usize as c_ulong; (*p).thread.cpu_context.x20=(*args).fn_arg as usize as c_ulong; if system_supports_poe(){(*p).thread.por_el0=POR_EL0_INIT;} }
    (*p).thread.cpu_context.pc=ret_from_fork as usize as c_ulong; (*p).thread.cpu_context.sp=childregs as usize as c_ulong; (*p).thread.cpu_context.fp=&mut (*childregs).stackframe as *mut _ as usize as c_ulong; ptrace_hw_copy_thread(p); 0
}

pub unsafe extern "C" fn tls_preserve_current_state(){*task_user_tls(current)=read_sysreg(tpidr_el0);if system_supports_tpidr2()&&!is_compat_task(){(*current).thread.tpidr2_el0=read_sysreg_s(SYS_TPIDR2_EL0);}}
unsafe fn tls_thread_switch(next:*mut task_struct){tls_preserve_current_state();if is_compat_thread(task_thread_info(next)){write_sysreg((*next).thread.uw.tp_value,tpidrro_el0);}else{write_sysreg(0,tpidrro_el0);}write_sysreg(*task_user_tls(next),tpidr_el0);if system_supports_tpidr2(){write_sysreg_s((*next).thread.tpidr2_el0,SYS_TPIDR2_EL0);}}

unsafe fn ssbs_thread_switch(next:*mut task_struct){if (*next).flags&PF_KTHREAD!=0{return;}if alternative_has_cap_unlikely(ARM64_SSBS){return;}spectre_v4_enable_task_mitigation(next);}
pub static mut __entry_task: *mut task_struct = core::ptr::null_mut();
unsafe fn entry_task_switch(next:*mut task_struct){__entry_task=next;}

#[cfg(feature="CONFIG_ARM64_GCS")] pub unsafe extern "C" fn gcs_preserve_current_state(){(*current).thread.gcspr_el0=read_sysreg_s(SYS_GCSPR_EL0);}
#[cfg(feature="CONFIG_ARM64_GCS")] unsafe fn gcs_thread_switch(next:*mut task_struct){if !system_supports_gcs(){return;}gcs_preserve_current_state();write_sysreg_s((*next).thread.gcspr_el0,SYS_GCSPR_EL0);if (*current).thread.gcs_el0_mode!=(*next).thread.gcs_el0_mode{gcs_set_el0_mode(next);}if task_gcs_el0_enabled(current)||task_gcs_el0_enabled(next){gcsb_dsync();}}
#[cfg(not(feature="CONFIG_ARM64_GCS"))] unsafe fn gcs_thread_switch(_: *mut task_struct){}

unsafe fn update_cntkctl_el1(next:*mut task_struct){let ti=task_thread_info(next);if test_ti_thread_flag(ti,TIF_TSC_SIGSEGV)||has_erratum_handler(read_cntvct_el0)||(IS_ENABLED_CONFIG_ARM64_ERRATUM_1418040&&this_cpu_has_cap(ARM64_WORKAROUND_1418040)&&is_compat_thread(ti)){sysreg_clear_set(cntkctl_el1,ARCH_TIMER_USR_VCT_ACCESS_EN,0);}else{sysreg_clear_set(cntkctl_el1,0,ARCH_TIMER_USR_VCT_ACCESS_EN);}}
unsafe fn cntkctl_thread_switch(prev:*mut task_struct,next:*mut task_struct){let a=read_ti_thread_flags(task_thread_info(prev))&(_TIF_32BIT|_TIF_TSC_SIGSEGV);let b=read_ti_thread_flags(task_thread_info(next))&(_TIF_32BIT|_TIF_TSC_SIGSEGV);if a!=b{update_cntkctl_el1(next);}}
unsafe fn do_set_tsc_mode(val:c_uint)->c_int{let t=if val==PR_TSC_SIGSEGV{true}else if val==PR_TSC_ENABLE{false}else{return -EINVAL;};preempt_disable();update_thread_flag(TIF_TSC_SIGSEGV,t);update_cntkctl_el1(current);preempt_enable();0}
unsafe fn permission_overlay_switch(next:*mut task_struct){if !system_supports_poe(){return;}(*current).thread.por_el0=read_sysreg_s(SYS_POR_EL0);if (*current).thread.por_el0!=(*next).thread.por_el0{write_sysreg_s((*next).thread.por_el0,SYS_POR_EL0);}}
pub unsafe extern "C" fn update_sctlr_el1(sctlr:u64){sysreg_clear_set(sctlr_el1,SCTLR_USER_MASK&!SCTLR_ELx_ENIA,sctlr);isb();}

unsafe fn debug_switch_state(){if system_uses_irq_prio_masking(){let daif_actual=read_sysreg(daif);let pmr_actual=read_sysreg_s(SYS_ICC_PMR_EL1);WARN_ONCE(daif_actual!=0||pmr_actual!=GIC_PRIO_IRQOFF,c"Unexpected DAIF + PMR\n".as_ptr(),daif_actual,pmr_actual);}else{let daif_actual=read_sysreg(daif);WARN_ONCE(daif_actual!=DAIF_PROCCTX_NOIRQ,c"Unexpected DAIF value\n".as_ptr(),daif_actual);}}
pub unsafe extern "C" fn __switch_to(prev:*mut task_struct,next:*mut task_struct)->*mut task_struct{debug_switch_state();fpsimd_thread_switch(next);tls_thread_switch(next);hw_breakpoint_thread_switch(next);contextidr_thread_switch(next);entry_task_switch(next);ssbs_thread_switch(next);cntkctl_thread_switch(prev,next);ptrauth_thread_switch_user(next);permission_overlay_switch(next);gcs_thread_switch(next);dsb(ish);mte_thread_switch(next);if (*prev).thread.sctlr_user!=(*next).thread.sctlr_user{update_sctlr_el1((*next).thread.sctlr_user);}mpam_thread_switch(next);cpu_switch_to(prev,next)}

#[repr(C)] struct wchan_info{pc:c_ulong,count:c_int}
unsafe extern "C" fn get_wchan_cb(arg:*mut c_void,pc:c_ulong)->bool{let w=&mut *(arg as *mut wchan_info);if !in_sched_functions(pc){w.pc=pc;return false;}w.count+=1;w.count<17}
pub unsafe extern "C" fn __get_wchan(p:*mut task_struct)->c_ulong{let mut w=wchan_info{pc:0,count:0};if !try_get_task_stack(p){return 0;}arch_stack_walk(get_wchan_cb,&mut w,p,core::ptr::null_mut());put_task_stack(p);w.pc}
pub unsafe extern "C" fn arch_align_stack(mut sp:c_ulong)->c_ulong{if ((*current).personality&ADDR_NO_RANDOMIZE)==0&&randomize_va_space!=0{sp-=get_random_u32_below(PAGE_SIZE) as c_ulong;}sp&!0xf}

#[cfg(feature="CONFIG_COMPAT")] pub unsafe extern "C" fn compat_elf_check_arch(hdr:*const elf32_hdr)->bool{if !system_supports_32bit_el0()||(*hdr).e_machine!=EM_ARM||(*hdr).e_flags&EF_ARM_EABI_MASK==0{return false;}!static_branch_unlikely(&arm64_mismatched_32bit_el0)||!dl_task_check_affinity(current,system_32bit_el0_cpumask())}
pub unsafe extern "C" fn arch_setup_new_exec(){let mut mmflags=0;if is_compat_task(){mmflags=MMCF_AARCH32;if static_branch_unlikely(&arm64_mismatched_32bit_el0){force_compatible_cpus_allowed_ptr(current);}}else if static_branch_unlikely(&arm64_mismatched_32bit_el0){relax_compatible_cpus_allowed_ptr(current);}(*current).mm.context.flags=mmflags;ptrauth_thread_init_user();mte_thread_init_user();do_set_tsc_mode(PR_TSC_ENABLE);if task_spec_ssb_noexec(current){arch_prctl_spec_ctrl_set(current,PR_SPEC_STORE_BYPASS,PR_SPEC_ENABLE);}}

#[cfg(feature="CONFIG_ARM64_TAGGED_ADDR_ABI")]
static mut tagged_addr_disabled: c_uint = 0;
#[cfg(feature="CONFIG_ARM64_TAGGED_ADDR_ABI")]
pub unsafe extern "C" fn set_tagged_addr_ctrl(task:*mut task_struct,arg:c_ulong)->c_long{let mut valid_mask=PR_TAGGED_ADDR_ENABLE;let ti=task_thread_info(task);if is_compat_thread(ti){return -EINVAL;}if system_supports_mte(){valid_mask|=PR_MTE_TCF_SYNC|PR_MTE_TCF_ASYNC|PR_MTE_TAG_MASK;if cpus_have_cap(ARM64_MTE_STORE_ONLY){valid_mask|=PR_MTE_STORE_ONLY;}}if arg&!valid_mask!=0{return -EINVAL;}if arg&PR_TAGGED_ADDR_ENABLE!=0&&tagged_addr_disabled!=0{return -EINVAL;}if set_mte_ctrl(task,arg)!=0{return -EINVAL;}update_ti_thread_flag(ti,TIF_TAGGED_ADDR,arg&PR_TAGGED_ADDR_ENABLE!=0);0}
#[cfg(feature="CONFIG_ARM64_TAGGED_ADDR_ABI")]
pub unsafe extern "C" fn get_tagged_addr_ctrl(task:*mut task_struct)->c_long{let ti=task_thread_info(task);if is_compat_thread(ti){return -EINVAL;}let mut ret=0;if test_ti_thread_flag(ti,TIF_TAGGED_ADDR){ret=PR_TAGGED_ADDR_ENABLE;}ret|get_mte_ctrl(task)}

// Global sysctl: abi.tagged_addr_disabled, mode 0644, proc_dointvec_minmax,
// bounded by SYSCTL_ZERO and SYSCTL_ONE.
#[cfg(feature="CONFIG_ARM64_TAGGED_ADDR_ABI")]
unsafe fn tagged_addr_init()->c_int{if !register_sysctl(c"abi".as_ptr(),core::ptr::null_mut()){return -EINVAL;}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
