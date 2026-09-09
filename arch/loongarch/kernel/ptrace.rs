// SPDX-License-Identifier: GPL-2.0
// Direct low-level translation of loongarch/kernel/ptrace.c.
// Kernel and architecture declarations referenced below are supplied externally.

unsafe fn init_fp_ctx(target: *mut task_struct) {
    if tsk_used_math(target) { return; }
    memset(&mut (*target).thread.fpu.fpr as *mut _, !0, core::mem::size_of_val(&(*target).thread.fpu.fpr));
    set_stopped_child_used_math(target);
}

pub unsafe fn ptrace_disable(child: *mut task_struct) {
    clear_tsk_thread_flag(child, TIF_LOAD_WATCH);
    clear_tsk_thread_flag(child, TIF_SINGLESTEP);
}

unsafe fn gpr_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 {
    let regs = task_pt_regs(target);
    let mut r = membuf_write(&mut to, (*regs).regs.as_mut_ptr(), core::mem::size_of::<u64>() * GPR_NUM);
    r = membuf_write(&mut to, &mut (*regs).orig_a0 as *mut _, core::mem::size_of::<u64>());
    r = membuf_write(&mut to, &mut (*regs).csr_era as *mut _, core::mem::size_of::<u64>());
    r = membuf_write(&mut to, &mut (*regs).csr_badvaddr as *mut _, core::mem::size_of::<u64>());
    r
}

unsafe fn gpr_set(target: *mut task_struct, _regset: *const user_regset, mut pos: u32, mut count: u32, mut kbuf: *const core::ffi::c_void, mut ubuf: *const core::ffi::c_void) -> i32 {
    let regs = task_pt_regs(target);
    let a0_start = core::mem::size_of::<u64>() * GPR_NUM;
    let era_start = a0_start + core::mem::size_of::<u64>();
    let badvaddr_start = era_start + core::mem::size_of::<u64>();
    let mut err = user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,(*regs).regs.as_mut_ptr() as *mut _,0,a0_start);
    err |= user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut (*regs).orig_a0 as *mut _,a0_start,a0_start+8);
    err |= user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut (*regs).csr_era as *mut _,era_start,era_start+8);
    err |= user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut (*regs).csr_badvaddr as *mut _,badvaddr_start,badvaddr_start+8);
    err
}

unsafe fn gfpr_get(target: *mut task_struct, to: *mut membuf) -> i32 { membuf_write(to, (*target).thread.fpu.fpr.as_mut_ptr() as *mut _, core::mem::size_of::<elf_fpreg_t>() * NUM_FPU_REGS) }
unsafe fn gfpr_get_simd(target: *mut task_struct, to: *mut membuf) -> i32 {
    let mut r = 0; for i in 0..NUM_FPU_REGS { let v = get_fpr64(&(*target).thread.fpu.fpr[i],0); r = membuf_write(to,&v as *const _ as *mut _,core::mem::size_of::<elf_fpreg_t>()); } r
}
unsafe fn fpr_get(target: *mut task_struct, _r: *const user_regset, mut to: membuf) -> i32 {
    save_fpu_regs(target); let mut r;
    if core::mem::size_of_val(&(*target).thread.fpu.fpr[0]) == core::mem::size_of::<elf_fpreg_t>() { r=gfpr_get(target,&mut to); } else { r=gfpr_get_simd(target,&mut to); }
    r=membuf_write(&mut to,&mut (*target).thread.fpu.fcc as *mut _,core::mem::size_of_val(&(*target).thread.fpu.fcc));
    r=membuf_write(&mut to,&mut (*target).thread.fpu.fcsr as *mut _,core::mem::size_of_val(&(*target).thread.fpu.fcsr)); r
}
unsafe fn gfpr_set(target:*mut task_struct,pos:*mut u32,count:*mut u32,kbuf:*mut *const core::ffi::c_void,ubuf:*mut *const core::ffi::c_void)->i32 { user_regset_copyin(pos,count,kbuf,ubuf,(*target).thread.fpu.fpr.as_mut_ptr() as *mut _,0,NUM_FPU_REGS*core::mem::size_of::<elf_fpreg_t>()) }
unsafe fn gfpr_set_simd(target:*mut task_struct,pos:*mut u32,count:*mut u32,kbuf:*mut *const core::ffi::c_void,ubuf:*mut *const core::ffi::c_void)->i32 { let mut v=0u64; for i in 0..NUM_FPU_REGS { if *count==0 { break; } let e=user_regset_copyin(pos,count,kbuf,ubuf,&mut v as *mut _ as *mut _,i*8,(i+1)*8); if e!=0{return e;} set_fpr64(&mut (*target).thread.fpu.fpr[i],0,v); } 0 }
unsafe fn fpr_set(target:*mut task_struct,_r:*const user_regset,mut pos:u32,mut count:u32,mut kbuf:*const core::ffi::c_void,mut ubuf:*const core::ffi::c_void)->i32 { if count%core::mem::size_of::<elf_fpreg_t>() as u32!=0{return -EINVAL;} init_fp_ctx(target); let mut e=if core::mem::size_of_val(&(*target).thread.fpu.fpr[0])==core::mem::size_of::<elf_fpreg_t>(){gfpr_set(target,&mut pos,&mut count,&mut kbuf,&mut ubuf)}else{gfpr_set_simd(target,&mut pos,&mut count,&mut kbuf,&mut ubuf)}; if e!=0{return e;} let s=NUM_FPU_REGS*core::mem::size_of::<elf_fpreg_t>(); e|=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut (*target).thread.fpu.fcc as *mut _,s,s+8); e|=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut (*target).thread.fpu.fcsr as *mut _,s+8,s+12); e }

unsafe fn cfg_get(_target:*mut task_struct,_r:*const user_regset,mut to:membuf)->i32 { let mut i=0; let mut r=0; while to.left>0 { let mut v=read_cpucfg(i); i+=1; r=membuf_write(&mut to,&mut v,4); } r }
unsafe fn cfg_set(_target:*mut task_struct,_r:*const user_regset,_p:u32,_c:u32,_k:*const core::ffi::c_void,_u:*const core::ffi::c_void)->i32 { 0 }

#[cfg(CONFIG_CPU_HAS_LSX)]
unsafe fn simd_get(target:*mut task_struct,regset:*const user_regset,mut to:membuf)->i32 { save_fpu_regs(target); membuf_write(&mut to,(*target).thread.fpu.fpr.as_mut_ptr() as *mut _, NUM_FPU_REGS*(*regset).size as usize); 0 }
#[cfg(CONFIG_CPU_HAS_LSX)]
unsafe fn simd_set(target:*mut task_struct,regset:*const user_regset,mut pos:u32,mut count:u32,mut k:*const core::ffi::c_void,mut u:*const core::ffi::c_void)->i32 { init_fp_ctx(target); user_regset_copyin(&mut pos,&mut count,&mut k,&mut u,(*target).thread.fpu.fpr.as_mut_ptr() as *mut _,0,NUM_FPU_REGS*(*regset).size as usize) }

#[repr(C)] pub struct pt_regs_offset { pub name:*const core::ffi::c_char, pub offset:i32 }
static REG_NAMES:[&[u8];36]=[b"r0",b"r1",b"r2",b"r3",b"r4",b"r5",b"r6",b"r7",b"r8",b"r9",b"r10",b"r11",b"r12",b"r13",b"r14",b"r15",b"r16",b"r17",b"r18",b"r19",b"r20",b"r21",b"r22",b"r23",b"r24",b"r25",b"r26",b"r27",b"r28",b"r29",b"r30",b"r31",b"orig_a0",b"csr_era",b"csr_badvaddr",b"csr_crmd"];
pub unsafe fn regs_query_register_offset(name:*const core::ffi::c_char)->i32 { for (i,n) in REG_NAMES.iter().enumerate(){ if strcmp(name,n.as_ptr() as *const _)==0{return (i*8) as i32;} } -EINVAL }

unsafe fn read_user(target:*mut task_struct,addr:usize,data:*mut usize)->i32 { let r=task_pt_regs(target); let v=match addr { 0..=31=>(*r).regs[addr], ARG0=>(*r).orig_a0 as usize, PC=>(*r).csr_era as usize, BADVADDR=>(*r).csr_badvaddr as usize, _=>return -EIO }; put_user(v,data) }
unsafe fn write_user(target:*mut task_struct,addr:usize,data:usize)->i32 { let r=task_pt_regs(target); match addr {0..=31=>(*r).regs[addr]=data as u64,ARG0=>(*r).orig_a0=data as u64,PC=>(*r).csr_era=data as u64,BADVADDR=>(*r).csr_badvaddr=data as u64,_=>return -EIO};0 }
pub unsafe fn arch_ptrace(child:*mut task_struct,request: i64,addr:usize,data:usize)->i64 { let p=data as *mut usize; match request as i32 { PTRACE_PEEKUSR=>read_user(child,addr,p) as i64,PTRACE_POKEUSR=>write_user(child,addr,data) as i64,_=>ptrace_request(child,request,addr,data) } }

#[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
unsafe fn ptrace_triggered(bp:*mut perf_event,_data:*mut perf_sample_data,_regs:*mut pt_regs){let mut a=(*bp).attr;a.disabled=true;modify_user_hw_breakpoint(bp,&a);}
#[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
pub unsafe fn user_enable_single_step(task:*mut task_struct){let ti=task_thread_info(task);set_ti_thread_flag(ti,TIF_SINGLESTEP);(*task).thread.single_step=(*task_pt_regs(task)).csr_era;}
#[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
pub unsafe fn user_disable_single_step(task:*mut task_struct){clear_tsk_thread_flag(task,TIF_SINGLESTEP);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
