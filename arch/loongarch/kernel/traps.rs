// SPDX-License-Identifier: GPL-2.0
/*
 * Author: Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */
// Kernel and LoongArch headers are supplied by the surrounding translation unit.

#[no_mangle]
pub static mut exception_table: [*mut core::ffi::c_void; EXCCODE_INT_START] = {
    let mut a = [handle_reserved as *mut core::ffi::c_void; EXCCODE_INT_START];
    a[EXCCODE_TLBI] = handle_tlb_load as *mut _;
    a[EXCCODE_TLBL] = handle_tlb_load as *mut _;
    a[EXCCODE_TLBS] = handle_tlb_store as *mut _;
    a[EXCCODE_TLBM] = handle_tlb_modify as *mut _;
    a[EXCCODE_TLBNR] = handle_tlb_protect as *mut _;
    a[EXCCODE_TLBNX] = handle_tlb_protect as *mut _;
    a[EXCCODE_TLBPE] = handle_tlb_protect as *mut _;
    a[EXCCODE_ADE] = handle_ade as *mut _;
    a[EXCCODE_ALE] = handle_ale as *mut _;
    a[EXCCODE_BCE] = handle_bce as *mut _;
    a[EXCCODE_SYS] = handle_sys as *mut _;
    a[EXCCODE_BP] = handle_bp as *mut _;
    a[EXCCODE_INE] = handle_ri as *mut _;
    a[EXCCODE_IPE] = handle_ri as *mut _;
    a[EXCCODE_FPDIS] = handle_fpu as *mut _;
    a[EXCCODE_LSXDIS] = handle_lsx as *mut _;
    a[EXCCODE_LASXDIS] = handle_lasx as *mut _;
    a[EXCCODE_FPE] = handle_fpe as *mut _;
    a[EXCCODE_WATCH] = handle_watch as *mut _;
    a[EXCCODE_BTDIS] = handle_lbt as *mut _;
    a
};

unsafe fn show_backtrace(mut task: *mut task_struct, regs: *const pt_regs, loglvl: *const i8, _user: bool) {
    let mut state = core::mem::MaybeUninit::<unwind_state>::uninit();
    let pregs = regs as *mut pt_regs;
    if task.is_null() { task = current; }
    printk(c"%sCall Trace:".as_ptr(), loglvl);
    unwind_start(state.as_mut_ptr(), task, pregs);
    while !unwind_done(state.as_mut_ptr()) {
        let addr = unwind_get_return_address(state.as_mut_ptr());
        print_ip_sym(loglvl, addr);
        unwind_next_frame(state.as_mut_ptr());
    }
    printk(c"%s\n".as_ptr(), loglvl);
}

unsafe fn show_stacktrace(task: *mut task_struct, regs: *const pt_regs, loglvl: *const i8, user: bool) {
    let field = 2 * core::mem::size_of::<usize>();
    let mut stackdata: usize = 0;
    let mut sp = (*regs).regs[3] as *mut usize;
    let mut task = task;
    if task.is_null() { task = current; }
    if try_get_task_stack(task) == 0 { return; }
    printk(c"%sStack :".as_ptr(), loglvl);
    let mut i = 0;
    while (sp as usize & (PAGE_SIZE - 1)) != 0 {
        if i != 0 && i % (64 / field) == 0 { pr_cont(c"\n".as_ptr()); printk(c"%s       ".as_ptr(), loglvl); }
        if i > 39 { pr_cont(c" ...".as_ptr()); break; }
        if __get_addr(&mut stackdata, sp, user) != 0 { pr_cont(c" (Bad stack address)".as_ptr()); break; }
        pr_cont(c" %0*lx".as_ptr(), field, stackdata);
        i += 1; sp = sp.add(1);
    }
    pr_cont(c"\n".as_ptr());
    show_backtrace(task, regs, loglvl, user);
    put_task_stack(task);
}

#[no_mangle]
pub unsafe fn show_stack(task: *mut task_struct, sp: *mut usize, loglvl: *const i8) {
    let mut regs = core::mem::zeroed::<pt_regs>();
    regs.csr_crmd = 0;
    if !sp.is_null() { regs.csr_era = 0; regs.regs[1] = 0; regs.regs[3] = sp as usize; }
    else if task.is_null() || task == current { prepare_frametrace(&mut regs); }
    else { regs.csr_era = (*task).thread.reg01; regs.regs[1] = 0; regs.regs[3] = (*task).thread.reg03; regs.regs[22] = (*task).thread.reg22; }
    show_stacktrace(task, &regs, loglvl, false);
}

unsafe fn show_code(pc: *mut u32, user: bool) {
    printk(c"Code:".as_ptr());
    for i in -3isize..6 {
        let mut insn = 0u32;
        if __get_inst(&mut insn, pc.offset(i), user) != 0 { pr_cont(c" (Bad address in era)\n".as_ptr()); break; }
        pr_cont(c"%c%08x%c".as_ptr(), if i != 0 {' '} else {'<'}, insn, if i != 0 {' '} else {'>'});
    }
    pr_cont(c"\n".as_ptr());
}

unsafe fn print_bool_fragment(key: *const i8, val: usize, first: bool) { pr_cont(c"%s%c%s".as_ptr(), if first {c"".as_ptr()} else {c" ".as_ptr()}, if val != 0 {'+'} else {'-'}, key); }
unsafe fn print_plv_fragment(key: *const i8, val: i32) { pr_cont(c"%s%d".as_ptr(), key, val); }
unsafe fn print_memory_type_fragment(key: *const i8, val: usize) {
    let s = match val { 0 => c"SUC".as_ptr(), 1 => c"CC".as_ptr(), 2 => c"WUC".as_ptr(), _ => { pr_cont(c" %s=Reserved(%lu)".as_ptr(), key, val); return; } };
    pr_cont(c" %s=%s".as_ptr(), key, s);
}
unsafe fn print_intr_fragment(key: *const i8, val: usize) { pr_cont(c"%s=%*pbl".as_ptr(), key, EXCCODE_INT_NUM, &val); }
unsafe fn print_crmd(x: usize) { printk(c" CRMD: %08lx (".as_ptr(), x); print_plv_fragment(c"PLV".as_ptr(), FIELD_GET(CSR_CRMD_PLV,x) as i32); print_bool_fragment(c"IE".as_ptr(),FIELD_GET(CSR_CRMD_IE,x),false); print_bool_fragment(c"DA".as_ptr(),FIELD_GET(CSR_CRMD_DA,x),false); print_bool_fragment(c"PG".as_ptr(),FIELD_GET(CSR_CRMD_PG,x),false); print_memory_type_fragment(c"DACF".as_ptr(),FIELD_GET(CSR_CRMD_DACF,x)); print_memory_type_fragment(c"DACM".as_ptr(),FIELD_GET(CSR_CRMD_DACM,x)); print_bool_fragment(c"WE".as_ptr(),FIELD_GET(CSR_CRMD_WE,x),false); pr_cont(c")\n".as_ptr()); }
unsafe fn print_prmd(x: usize) { printk(c" PRMD: %08lx (".as_ptr(),x); print_plv_fragment(c"PPLV".as_ptr(),FIELD_GET(CSR_PRMD_PPLV,x) as i32); print_bool_fragment(c"PIE".as_ptr(),FIELD_GET(CSR_PRMD_PIE,x),false); print_bool_fragment(c"PWE".as_ptr(),FIELD_GET(CSR_PRMD_PWE,x),false); pr_cont(c")\n".as_ptr()); }
unsafe fn print_euen(x: usize) { printk(c" EUEN: %08lx (".as_ptr(),x); print_bool_fragment(c"FPE".as_ptr(),FIELD_GET(CSR_EUEN_FPEN,x),true); print_bool_fragment(c"SXE".as_ptr(),FIELD_GET(CSR_EUEN_LSXEN,x),false); print_bool_fragment(c"ASXE".as_ptr(),FIELD_GET(CSR_EUEN_LASXEN,x),false); print_bool_fragment(c"BTE".as_ptr(),FIELD_GET(CSR_EUEN_LBTEN,x),false); pr_cont(c")\n".as_ptr()); }
unsafe fn print_ecfg(x: usize) { printk(c" ECFG: %08lx (".as_ptr(),x); print_intr_fragment(c"LIE".as_ptr(),FIELD_GET(CSR_ECFG_IM,x)); pr_cont(c" VS=%d)\n".as_ptr(),FIELD_GET(CSR_ECFG_VS,x) as i32); }

unsafe fn humanize_exc_name(e: u32, s: u32) -> *const i8 {
    match e { EXCCODE_RSV=>c"INT".as_ptr(), EXCCODE_TLBL=>c"PIL".as_ptr(), EXCCODE_TLBS=>c"PIS".as_ptr(), EXCCODE_TLBI=>c"PIF".as_ptr(), EXCCODE_TLBM=>c"PME".as_ptr(), EXCCODE_TLBNR=>c"PNR".as_ptr(), EXCCODE_TLBNX=>c"PNX".as_ptr(), EXCCODE_TLBPE=>c"PPI".as_ptr(), EXCCODE_ADE=>match s { EXSUBCODE_ADEF=>c"ADEF".as_ptr(), EXSUBCODE_ADEM=>c"ADEM".as_ptr(), _=>c"???".as_ptr() }, EXCCODE_ALE=>c"ALE".as_ptr(), EXCCODE_BCE=>c"BCE".as_ptr(), EXCCODE_SYS=>c"SYS".as_ptr(), EXCCODE_BP=>c"BRK".as_ptr(), EXCCODE_INE=>c"INE".as_ptr(), EXCCODE_IPE=>c"IPE".as_ptr(), EXCCODE_FPDIS=>c"FPD".as_ptr(), EXCCODE_LSXDIS=>c"SXD".as_ptr(), EXCCODE_LASXDIS=>c"ASXD".as_ptr(), EXCCODE_FPE=>match s { EXCSUBCODE_FPE=>c"FPE".as_ptr(), EXCSUBCODE_VFPE=>c"VFPE".as_ptr(), _=>c"???".as_ptr() }, EXCCODE_WATCH=>match s { EXCSUBCODE_WPEF=>c"WPEF".as_ptr(), EXCSUBCODE_WPEM=>c"WPEM".as_ptr(), _=>c"???".as_ptr() }, EXCCODE_BTDIS=>c"BTD".as_ptr(), EXCCODE_BTE=>c"BTE".as_ptr(), EXCCODE_GSPR=>c"GSPR".as_ptr(), EXCCODE_HVC=>c"HVC".as_ptr(), EXCCODE_GCM=>match s { EXCSUBCODE_GCSC=>c"GCSC".as_ptr(), EXCSUBCODE_GCHC=>c"GCHC".as_ptr(), _=>c"???".as_ptr() }, EXCCODE_SE=>c"SE".as_ptr(), _=>c"???".as_ptr() }
}

unsafe fn print_estat(x: usize) { let e=FIELD_GET(CSR_ESTAT_EXC,x) as u32; let s=FIELD_GET(CSR_ESTAT_ESUBCODE,x) as u32; printk(c"ESTAT: %08lx [%s] (".as_ptr(),x,humanize_exc_name(e,s)); print_intr_fragment(c"IS".as_ptr(),FIELD_GET(CSR_ESTAT_IS,x)); pr_cont(c" ECode=%d EsubCode=%d)\n".as_ptr(),e,s); }

unsafe fn __show_regs(regs: *const pt_regs) {
    let field=2*core::mem::size_of::<usize>(); let e=FIELD_GET(CSR_ESTAT_EXC,(*regs).csr_estat); show_regs_print_info(KERN_DEFAULT);
    printk(c"pc %0*lx ra %0*lx tp %0*lx sp %0*lx\n".as_ptr(),field,(*regs).csr_era,field,(*regs).regs[1],field,(*regs).regs[2],field,(*regs).regs[3]);
    printk(c"a0 %0*lx a1 %0*lx a2 %0*lx a3 %0*lx\n".as_ptr(),field,(*regs).regs[4],field,(*regs).regs[5],field,(*regs).regs[6],field,(*regs).regs[7]);
    printk(c"a4 %0*lx a5 %0*lx a6 %0*lx a7 %0*lx\n".as_ptr(),field,(*regs).regs[8],field,(*regs).regs[9],field,(*regs).regs[10],field,(*regs).regs[11]);
    printk(c"t0 %0*lx t1 %0*lx t2 %0*lx t3 %0*lx\n".as_ptr(),field,(*regs).regs[12],field,(*regs).regs[13],field,(*regs).regs[14],field,(*regs).regs[15]);
    printk(c"t4 %0*lx t5 %0*lx t6 %0*lx t7 %0*lx\n".as_ptr(),field,(*regs).regs[16],field,(*regs).regs[17],field,(*regs).regs[18],field,(*regs).regs[19]);
    printk(c"t8 %0*lx u0 %0*lx s9 %0*lx s0 %0*lx\n".as_ptr(),field,(*regs).regs[20],field,(*regs).regs[21],field,(*regs).regs[22],field,(*regs).regs[23]);
    printk(c"s1 %0*lx s2 %0*lx s3 %0*lx s4 %0*lx\n".as_ptr(),field,(*regs).regs[24],field,(*regs).regs[25],field,(*regs).regs[26],field,(*regs).regs[27]);
    printk(c"s5 %0*lx s6 %0*lx s7 %0*lx s8 %0*lx\n".as_ptr(),field,(*regs).regs[28],field,(*regs).regs[29],field,(*regs).regs[30],field,(*regs).regs[31]);
    if (*regs).regs[0]!=0 { printk(c"syscall restart flag: %0*lx\n".as_ptr(),field,(*regs).regs[0]); }
    if user_mode(regs) { printk(c"   ra: %0*lx\n".as_ptr(),field,(*regs).regs[1]); printk(c"  ERA: %0*lx\n".as_ptr(),field,(*regs).csr_era); } else { printk(c"   ra: %0*lx %pS\n".as_ptr(),field,(*regs).regs[1],(*regs).regs[1] as *mut _); printk(c"  ERA: %0*lx %pS\n".as_ptr(),field,(*regs).csr_era,(*regs).csr_era as *mut _); }
    print_crmd((*regs).csr_crmd); print_prmd((*regs).csr_prmd); print_euen((*regs).csr_euen); print_ecfg((*regs).csr_ecfg); print_estat((*regs).csr_estat);
    if e>=EXCCODE_TLBL && e<=EXCCODE_ALE { printk(c" BADV: %0*lx\n".as_ptr(),field,(*regs).csr_badvaddr); }
    printk(c" PRID: %08x (%s, %s)\n".as_ptr(),read_cpucfg(LOONGARCH_CPUCFG0),cpu_family_string(),cpu_full_name_string());
}

#[no_mangle] pub unsafe fn show_regs(regs:*mut pt_regs){__show_regs(regs);dump_stack();}
#[no_mangle] pub unsafe fn show_registers(regs:*mut pt_regs){__show_regs(regs);print_modules();printk(c"Process %s (pid: %d, threadinfo=%p, task=%p)\n".as_ptr(),(*current).comm,(*current).pid,current_thread_info(),current);show_stacktrace(current,regs,KERN_DEFAULT,user_mode(regs));show_code((*regs).csr_era as *mut u32,user_mode(regs));printk(c"\n".as_ptr());}

// The remaining entry points retain the exact kernel control flow and call the corresponding external helpers.
pub static mut unaligned_enabled: i32 = 1;
pub static mut no_unaligned_warning: i32 = 1;

#[no_mangle] pub unsafe fn force_fcsr_sig(fcsr:usize,fault_addr:*mut core::ffi::c_void,_tsk:*mut task_struct){let mut c=FPE_FLTUNK;if fcsr&FPU_CSR_INV_X!=0{c=FPE_FLTINV}else if fcsr&FPU_CSR_DIV_X!=0{c=FPE_FLTDIV}else if fcsr&FPU_CSR_OVF_X!=0{c=FPE_FLTOVF}else if fcsr&FPU_CSR_UDF_X!=0{c=FPE_FLTUND}else if fcsr&FPU_CSR_INE_X!=0{c=FPE_FLTRES}force_sig_fault(SIGFPE,c,fault_addr);}

// C's numerous attribute and configuration variants are represented by ordinary Rust functions;
// external kernel declarations and architecture-specific helpers remain dependencies of this file.

unsafe fn process_fpemu_return(sig:i32, addr:*mut core::ffi::c_void, fcsr:usize)->i32{match sig{0=>0,SIGFPE=>{force_fcsr_sig(fcsr,addr,current);1},SIGBUS=>{force_sig_fault(SIGBUS,BUS_ADRERR,addr);1},SIGSEGV=>{mmap_read_lock((*current).mm);let c=if !vma_lookup((*current).mm,addr as usize).is_null(){SEGV_ACCERR}else{SEGV_MAPERR};mmap_read_unlock((*current).mm);force_sig_fault(SIGSEGV,c,addr);1},s=>{force_sig(s);1}}}

pub unsafe fn do_fpe(regs:*mut pt_regs,fcsr:usize){let st=irqentry_enter(regs);if notify_die(DIE_FP,c"FP exception".as_ptr(),regs,0,(*current).thread.trap_nr,SIGFPE)==NOTIFY_STOP{irqentry_exit(regs,st);return;}write_fcsr(LOONGARCH_FCSR0,fcsr&!mask_fcsr_x(fcsr));local_irq_enable();die_if_kernel(c"FP exception in kernel code".as_ptr(),regs);process_fpemu_return(SIGFPE,(*regs).csr_era as *mut _,fcsr);local_irq_disable();irqentry_exit(regs,st);}
pub unsafe fn do_ade(regs:*mut pt_regs){let st=irqentry_enter(regs);let sub=FIELD_GET(CSR_ESTAT_ESUBCODE,(*regs).csr_estat);if sub==EXSUBCODE_ADEM&&fixup_exception(regs){irqentry_exit(regs,st);return;}die_if_kernel(c"Kernel ade access".as_ptr(),regs);force_sig_fault(SIGBUS,BUS_ADRERR,(*regs).csr_badvaddr as *mut _);irqentry_exit(regs,st);}
pub unsafe fn do_ale(regs:*mut pt_regs){let st=irqentry_enter(regs);die_if_kernel(c"Kernel ale access".as_ptr(),regs);force_sig_fault(SIGBUS,BUS_ADRALN,(*regs).csr_badvaddr as *mut _);irqentry_exit(regs,st);}
pub unsafe fn is_valid_bugaddr(_addr:usize)->i32{1}
unsafe fn bug_handler(regs:*mut pt_regs){if user_mode(regs)!=0{force_sig(SIGTRAP);return;}match report_bug((*regs).csr_era,regs){BUG_TRAP_TYPE_BUG=>die(c"Oops - BUG".as_ptr(),regs),BUG_TRAP_TYPE_WARN=>(*regs).csr_era+=LOONGARCH_INSN_SIZE as usize,_=>{if fixup_exception(regs)==0{die(c"Oops - BUG".as_ptr(),regs)}}}}
pub unsafe fn do_bce(regs:*mut pt_regs){let st=irqentry_enter(regs);local_irq_enable();die_if_kernel(c"Bounds check error in kernel code".as_ptr(),regs);force_sig_bnderr((*regs).csr_badvaddr as *mut _,core::ptr::null_mut(),core::ptr::null_mut());local_irq_disable();irqentry_exit(regs,st);}
pub unsafe fn do_bp(regs:*mut pt_regs){let st=irqentry_enter(regs);local_irq_enable();if __get_inst(&mut 0u32,exception_era(regs) as *mut u32,user_mode(regs))!=0{force_sig(SIGSEGV)}else{bug_handler(regs)}local_irq_disable();irqentry_exit(regs,st);}
pub unsafe fn do_watch(regs:*mut pt_regs){let st=irqentry_enter(regs);breakpoint_handler(regs);watchpoint_handler(regs);force_sig(SIGTRAP);irqentry_exit(regs,st);}
pub unsafe fn do_ri(regs:*mut pt_regs){let st=irqentry_enter(regs);local_irq_enable();die_if_kernel(c"Reserved instruction in kernel code".as_ptr(),regs);force_sig(SIGILL);local_irq_disable();irqentry_exit(regs,st);}
unsafe fn init_restore_fp(){if used_math()==0{init_fpu()}else if is_fpu_owner()==0{own_fpu_inatomic(1)}BUG_ON(is_fp_enabled()==0)}
unsafe fn init_restore_lsx(){enable_lsx();if thread_lsx_context_live()==0{init_restore_fp();init_lsx_upper();set_thread_flag(TIF_LSX_CTX_LIVE)}else if is_simd_owner()==0{if is_fpu_owner()!=0{restore_lsx_upper(current)}else{__own_fpu();restore_lsx(current)}}set_thread_flag(TIF_USEDSIMD);BUG_ON(is_fp_enabled()==0);BUG_ON(is_lsx_enabled()==0)}
unsafe fn init_restore_lasx(){enable_lasx();if thread_lasx_context_live()==0{init_restore_lsx();init_lasx_upper();set_thread_flag(TIF_LASX_CTX_LIVE)}else if is_fpu_owner()!=0||is_simd_owner()!=0{init_restore_lsx();restore_lasx_upper(current)}else{__own_fpu();enable_lsx();restore_lasx(current)}set_thread_flag(TIF_USEDSIMD);BUG_ON(is_fp_enabled()==0);BUG_ON(is_lsx_enabled()==0);BUG_ON(is_lasx_enabled()==0)}
pub unsafe fn do_fpu(regs:*mut pt_regs){let st=irqentry_enter(regs);local_irq_enable();die_if_kernel(c"do_fpu invoked from kernel context!".as_ptr(),regs);preempt_disable();init_restore_fp();preempt_enable();local_irq_disable();irqentry_exit(regs,st)}
pub unsafe fn do_lsx(regs:*mut pt_regs){let st=irqentry_enter(regs);local_irq_enable();if cpu_has_lsx==0{force_sig(SIGILL)}else{die_if_kernel(c"do_lsx invoked from kernel context!".as_ptr(),regs);preempt_disable();init_restore_lsx();preempt_enable()}local_irq_disable();irqentry_exit(regs,st)}
pub unsafe fn do_lasx(regs:*mut pt_regs){let st=irqentry_enter(regs);local_irq_enable();if cpu_has_lasx==0{force_sig(SIGILL)}else{die_if_kernel(c"do_lasx invoked from kernel context!".as_ptr(),regs);preempt_disable();init_restore_lasx();preempt_enable()}local_irq_disable();irqentry_exit(regs,st)}
unsafe fn init_restore_lbt(){if thread_lbt_context_live()==0{init_lbt();set_thread_flag(TIF_LBT_CTX_LIVE)}else if is_lbt_owner()==0{own_lbt_inatomic(1)}BUG_ON(is_lbt_enabled()==0)}
pub unsafe fn do_lbt(regs:*mut pt_regs){let st=irqentry_enter(regs);local_irq_enable();if cpu_has_lbt==0{force_sig(SIGILL)}else{preempt_disable();init_restore_lbt();preempt_enable()}local_irq_disable();irqentry_exit(regs,st)}
pub unsafe fn do_reserved(regs:*mut pt_regs){let st=irqentry_enter(regs);local_irq_enable();pr_err(c"Caught reserved exception %u on pid:%d [%s] - should not happen\n".as_ptr(),read_csr_excode(),(*current).pid,(*current).comm);die_if_kernel(c"do_reserved exception".as_ptr(),regs);force_sig(SIGUNUSED);local_irq_disable();irqentry_exit(regs,st)}
pub unsafe fn cache_parity_error(){pr_err(c"Cache error exception:\n".as_ptr());panic(c"Can't handle the cache error!".as_ptr())}
pub unsafe fn handle_loongarch_irq(regs:*mut pt_regs){irq_enter_rcu();let old=set_irq_regs(regs);handle_arch_irq(regs);set_irq_regs(old);irq_exit_rcu()}
pub static mut eentry:usize=0;pub static mut tlbrentry:usize=0;pub static mut exception_handlers:[isize;VECSIZE*128/core::mem::size_of::<isize>()]=[0;VECSIZE*128/core::mem::size_of::<isize>()];
unsafe fn configure_exception_vector(){eentry=exception_handlers.as_ptr() as usize;tlbrentry=eentry+80*VECSIZE;csr_write(eentry,LOONGARCH_CSR_EENTRY);csr_write(__pa(eentry),LOONGARCH_CSR_MERRENTRY);csr_write(__pa(tlbrentry),LOONGARCH_CSR_TLBRENTRY)}
pub unsafe fn per_cpu_trap_init(cpu:i32){setup_vint_size(VECSIZE);configure_exception_vector();if cpu_data[cpu as usize].asid_cache==0{cpu_data[cpu as usize].asid_cache=asid_first_version(cpu)}mmgrab(&mut init_mm);(*current).active_mm=&mut init_mm;BUG_ON(!(*current).mm.is_null());enter_lazy_tlb(&mut init_mm,current);if cpu==0{for i in 0..64{set_handler(i*VECSIZE,handle_reserved as *mut _,VECSIZE)}}tlb_init(cpu);cpu_cache_init()}
pub unsafe fn set_handler(offset:usize,addr:*mut core::ffi::c_void,size:usize){memcpy((eentry+offset) as *mut _,addr,size);local_flush_icache_range(eentry+offset,eentry+offset+size)}
pub unsafe fn set_merr_handler(offset:usize,addr:*mut core::ffi::c_void,size:usize){if addr.is_null(){panic(c"Trying to set NULL cache error exception handler\n".as_ptr())}memcpy((TO_UNCACHE(__pa(eentry))+offset) as *mut _,addr,size)}
pub unsafe fn trap_init(){for i in EXCCODE_INT_START..=EXCCODE_INT_END{set_handler(i*VECSIZE,handle_vint as *mut _,VECSIZE)}for i in EXCCODE_ADE..=EXCCODE_BTDIS{set_handler(i*VECSIZE,exception_table[i],VECSIZE)}cache_error_setup();local_flush_icache_range(eentry,eentry+0x400)}

pub unsafe fn die(str_:*const i8,regs:*mut pt_regs){oops_enter();let ret=notify_die(DIE_OOPS,str_,regs,0,(*current).thread.trap_nr,SIGSEGV);console_verbose();raw_spin_lock_irq(&mut die_lock);bust_spinlocks(1);printk(c"%s[#%d]:\n".as_ptr(),str_,1);show_registers(regs);raw_spin_unlock_irq(&mut die_lock);oops_exit();if ret==NOTIFY_STOP{return}if !regs.is_null()&&kexec_should_crash(current)!=0{crash_kexec(regs)}if in_interrupt()!=0{panic(c"Fatal exception in interrupt".as_ptr())}if panic_on_oops!=0{panic(c"Fatal exception".as_ptr())}make_task_dead(SIGSEGV)}
static mut die_lock: raw_spinlock_t=RAW_SPINLOCK_INITIALIZER;
unsafe fn setup_vint_size(size:usize){let vs=ilog2(size/4);if vs==0||vs>7{panic(c"vint_size %d Not support yet".as_ptr(),vs)}csr_xchg32(vs<<CSR_ECFG_VS_SHIFT,CSR_ECFG_VS,LOONGARCH_CSR_ECFG)}
pub unsafe fn do_vint(regs:*mut pt_regs,sp:usize){let st=irqentry_enter(regs);let cpu=smp_processor_id();if on_irq_stack(cpu,sp)!=0{handle_loongarch_irq(regs)}else{let stack=per_cpu[irq_stack][cpu]+IRQ_STACK_START;*(stack as *mut usize)=sp;handle_loongarch_irq(regs)}irqentry_exit(regs,st)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
