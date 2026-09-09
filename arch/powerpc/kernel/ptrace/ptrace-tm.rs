// SPDX-License-Identifier: GPL-2.0-or-later

// Translated from ptrace-tm.c. Kernel types, constants, macros, and helpers
// are supplied by the surrounding PowerPC kernel bindings.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub unsafe fn flush_tmregs_to_thread(tsk: *mut task_struct) {
    if !cpu_has_feature(CPU_FTR_TM) || tsk != current {
        return;
    }
    if MSR_TM_SUSPENDED(mfmsr()) {
        tm_reclaim_current(TM_CAUSE_SIGNAL);
    } else {
        tm_enable();
        tm_save_sprs(&mut (*tsk).thread);
    }
}

unsafe fn get_user_ckpt_msr(task: *mut task_struct) -> unsigned_long {
    (*task).thread.ckpt_regs.msr | (*task).thread.fpexc_mode
}

unsafe fn set_user_ckpt_msr(task: *mut task_struct, msr: unsigned_long) -> c_int {
    (*task).thread.ckpt_regs.msr &= !MSR_DEBUGCHANGE;
    (*task).thread.ckpt_regs.msr |= msr & MSR_DEBUGCHANGE;
    0
}

unsafe fn set_user_ckpt_trap(task: *mut task_struct, trap: unsigned_long) -> c_int {
    set_trap(&mut (*task).thread.ckpt_regs, trap);
    0
}

pub unsafe fn tm_cgpr_active(target: *mut task_struct, regset: *const user_regset) -> c_int {
    if !cpu_has_feature(CPU_FTR_TM) { return -ENODEV; }
    if !MSR_TM_ACTIVE((*target).thread.regs.msr) { return 0; }
    (*regset).n
}

pub unsafe fn tm_cgpr_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> c_int {
    let mut to_msr = membuf_at(&to, offset_of!(pt_regs, msr));
    #[cfg(target_pointer_width = "64")]
    let mut to_softe = membuf_at(&to, offset_of!(pt_regs, softe));
    if !cpu_has_feature(CPU_FTR_TM) { return -ENODEV; }
    if !MSR_TM_ACTIVE((*target).thread.regs.msr) { return -ENODATA; }
    flush_tmregs_to_thread(target); flush_fp_to_thread(target); flush_altivec_to_thread(target);
    membuf_write(&mut to, &(*target).thread.ckpt_regs as *const _, size_of::<user_pt_regs>());
    membuf_store(&mut to_msr, get_user_ckpt_msr(target));
    #[cfg(target_pointer_width = "64")]
    membuf_store(&mut to_softe, 0x1ul);
    membuf_zero(&mut to, ELF_NGREG * size_of::<unsigned_long>() - size_of::<user_pt_regs>())
}

pub unsafe fn tm_cgpr_set(target: *mut task_struct, _regset: *const user_regset, mut pos: *mut c_uint, mut count: *mut c_uint, mut kbuf: *const c_void, mut ubuf: *const c_void) -> c_int {
    let mut reg: unsigned_long = 0; let mut ret: c_int;
    if !cpu_has_feature(CPU_FTR_TM) { return -ENODEV; }
    if !MSR_TM_ACTIVE((*target).thread.regs.msr) { return -ENODATA; }
    flush_tmregs_to_thread(target); flush_fp_to_thread(target); flush_altivec_to_thread(target);
    ret = user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf, &mut (*target).thread.ckpt_regs as *mut _, 0, PT_MSR * size_of::<unsigned_long>());
    if ret == 0 && *count > 0 { ret = user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut reg,PT_MSR*size_of::<unsigned_long>(),(PT_MSR+1)*size_of::<unsigned_long>()); if ret == 0 { ret=set_user_ckpt_msr(target,reg); } }
    if ret == 0 { ret=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut (*target).thread.ckpt_regs.orig_gpr3 as *mut _,PT_ORIG_R3*size_of::<unsigned_long>(),(PT_MAX_PUT_REG+1)*size_of::<unsigned_long>()); }
    if PT_MAX_PUT_REG + 1 < PT_TRAP && ret == 0 { user_regset_copyin_ignore(&mut pos,&mut count,&mut kbuf,&mut ubuf,(PT_MAX_PUT_REG+1)*size_of::<unsigned_long>(),PT_TRAP*size_of::<unsigned_long>()); }
    if ret == 0 && *count > 0 { ret=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut reg,PT_TRAP*size_of::<unsigned_long>(),(PT_TRAP+1)*size_of::<unsigned_long>()); if ret == 0 { ret=set_user_ckpt_trap(target,reg); } }
    if ret == 0 { user_regset_copyin_ignore(&mut pos,&mut count,&mut kbuf,&mut ubuf,(PT_TRAP+1)*size_of::<unsigned_long>(),-1); }
    ret
}

pub unsafe fn tm_cfpr_active(target:*mut task_struct, regset:*const user_regset)->c_int { if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;} if !MSR_TM_ACTIVE((*target).thread.regs.msr){return 0;} (*regset).n }
pub unsafe fn tm_cfpr_get(target:*mut task_struct,_:*const user_regset,mut to:membuf)->c_int { let mut buf=[0u64;33]; if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;} if !MSR_TM_ACTIVE((*target).thread.regs.msr){return -ENODATA;} flush_tmregs_to_thread(target);flush_fp_to_thread(target);flush_altivec_to_thread(target); for i in 0..32 {buf[i]=(*target).thread.TS_CKFPR(i);} buf[32]=(*target).thread.ckfp_state.fpscr; membuf_write(&mut to,buf.as_ptr() as *const c_void,size_of_val(&buf)) }
pub unsafe fn tm_cfpr_set(target:*mut task_struct,_:*const user_regset,mut pos:*mut c_uint,mut count:*mut c_uint,mut kbuf:*const c_void,mut ubuf:*const c_void)->c_int { let mut buf=[0u64;33]; if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;} if !MSR_TM_ACTIVE((*target).thread.regs.msr){return -ENODATA;} flush_tmregs_to_thread(target);flush_fp_to_thread(target);flush_altivec_to_thread(target); for i in 0..32{buf[i]=(*target).thread.TS_CKFPR(i);}buf[32]=(*target).thread.ckfp_state.fpscr; let r=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,buf.as_mut_ptr() as *mut c_void,0,-1);if r!=0{return r;}for i in 0..32{(*target).thread.TS_CKFPR(i)=buf[i];}(*target).thread.ckfp_state.fpscr=buf[32];0 }

pub unsafe fn tm_cvmx_active(target:*mut task_struct,regset:*const user_regset)->c_int{if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;}if !MSR_TM_ACTIVE((*target).thread.regs.msr){return 0;}(*regset).n}
pub unsafe fn tm_cvmx_get(target:*mut task_struct,_:*const user_regset,mut to:membuf)->c_int{if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;}if !MSR_TM_ACTIVE((*target).thread.regs.msr){return -ENODATA;}flush_tmregs_to_thread(target);flush_fp_to_thread(target);flush_altivec_to_thread(target);membuf_write(&mut to,&(*target).thread.ckvr_state as *const _ as *const c_void,33*size_of::<vector128>());let mut vrsave=0u32;vrsave=(*target).thread.ckvrsave as u32;membuf_write(&mut to,&vrsave as *const _ as *const c_void,size_of::<u32>())}
pub unsafe fn tm_cvmx_set(target:*mut task_struct,_:*const user_regset,mut pos:*mut c_uint,mut count:*mut c_uint,mut kbuf:*const c_void,mut ubuf:*const c_void)->c_int{if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;}if !MSR_TM_ACTIVE((*target).thread.regs.msr){return -ENODATA;}flush_tmregs_to_thread(target);flush_fp_to_thread(target);flush_altivec_to_thread(target);let mut r=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut (*target).thread.ckvr_state as *mut _,0,33*size_of::<vector128>());if r==0&&*count>0{let mut v=(*target).thread.ckvrsave as u32;r=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut v,33*size_of::<vector128>(),-1);if r==0{(*target).thread.ckvrsave=v as _;}}r}

pub unsafe fn tm_cvsx_active(target:*mut task_struct,regset:*const user_regset)->c_int{if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;}if !MSR_TM_ACTIVE((*target).thread.regs.msr){return 0;}flush_vsx_to_thread(target);if (*target).thread.used_vsr!=0{(*regset).n}else{0}}
pub unsafe fn tm_cvsx_get(target:*mut task_struct,_:*const user_regset,mut to:membuf)->c_int{let mut b=[0u64;32];if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;}if !MSR_TM_ACTIVE((*target).thread.regs.msr){return -ENODATA;}flush_tmregs_to_thread(target);flush_fp_to_thread(target);flush_altivec_to_thread(target);flush_vsx_to_thread(target);for i in 0..32{b[i]=(*target).thread.ckfp_state.fpr[i][TS_VSRLOWOFFSET];}membuf_write(&mut to,b.as_ptr() as *const c_void,32*size_of::<f64>())}
pub unsafe fn tm_cvsx_set(target:*mut task_struct,_:*const user_regset,mut pos:*mut c_uint,mut count:*mut c_uint,mut kbuf:*const c_void,mut ubuf:*const c_void)->c_int{let mut b=[0u64;32];if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;}if !MSR_TM_ACTIVE((*target).thread.regs.msr){return -ENODATA;}flush_tmregs_to_thread(target);flush_fp_to_thread(target);flush_altivec_to_thread(target);flush_vsx_to_thread(target);for i in 0..32{b[i]=(*target).thread.ckfp_state.fpr[i][TS_VSRLOWOFFSET];}let r=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,b.as_mut_ptr() as *mut c_void,0,32*size_of::<f64>());if r==0{for i in 0..32{(*target).thread.ckfp_state.fpr[i][TS_VSRLOWOFFSET]=b[i];}}r}

pub unsafe fn tm_spr_active(_:*mut task_struct,regset:*const user_regset)->c_int{if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;}(*regset).n}
pub unsafe fn tm_spr_get(target:*mut task_struct,_:*const user_regset,mut to:membuf)->c_int{if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;}flush_tmregs_to_thread(target);flush_fp_to_thread(target);flush_altivec_to_thread(target);membuf_write(&mut to,&(*target).thread.tm_tfhar as *const _ as *const c_void,size_of::<u64>());membuf_write(&mut to,&(*target).thread.tm_texasr as *const _ as *const c_void,size_of::<u64>());membuf_write(&mut to,&(*target).thread.tm_tfiar as *const _ as *const c_void,size_of::<u64>())}
pub unsafe fn tm_spr_set(target:*mut task_struct,_:*const user_regset,mut pos:*mut c_uint,mut count:*mut c_uint,mut kbuf:*const c_void,mut ubuf:*const c_void)->c_int{if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;}flush_tmregs_to_thread(target);flush_fp_to_thread(target);flush_altivec_to_thread(target);let mut r=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut (*target).thread.tm_tfhar as *mut _,0,size_of::<u64>());if r==0{r=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut (*target).thread.tm_texasr as *mut _,size_of::<u64>(),2*size_of::<u64>());}if r==0{r=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut (*target).thread.tm_tfiar as *mut _,2*size_of::<u64>(),3*size_of::<u64>());}r}

pub unsafe fn tm_tar_active(t:*mut task_struct,r:*const user_regset)->c_int{if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;}if MSR_TM_ACTIVE((*t).thread.regs.msr){(*r).n}else{0}}
pub unsafe fn tm_tar_get(t:*mut task_struct,_:*const user_regset,mut to:membuf)->c_int{if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;}if !MSR_TM_ACTIVE((*t).thread.regs.msr){return -ENODATA;}membuf_write(&mut to,&(*t).thread.tm_tar as *const _ as *const c_void,size_of::<u64>())}
pub unsafe fn tm_tar_set(t:*mut task_struct,_:*const user_regset,mut p:*mut c_uint,mut n:*mut c_uint,mut k:*const c_void,mut u:*const c_void)->c_int{if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;}if !MSR_TM_ACTIVE((*t).thread.regs.msr){return -ENODATA;}user_regset_copyin(&mut p,&mut n,&mut k,&mut u,&mut (*t).thread.tm_tar as *mut _,0,size_of::<u64>())}

macro_rules! tm_simple_reg { ($a:ident,$g:ident,$s:ident,$f:ident) => { pub unsafe fn $a(t:*mut task_struct,r:*const user_regset)->c_int{if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;}if MSR_TM_ACTIVE((*t).thread.regs.msr){(*r).n}else{0}} pub unsafe fn $g(t:*mut task_struct,_:*const user_regset,mut to:membuf)->c_int{if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;}if !MSR_TM_ACTIVE((*t).thread.regs.msr){return -ENODATA;}membuf_write(&mut to,&(*t).thread.$f as *const _ as *const c_void,size_of::<u64>())} pub unsafe fn $s(t:*mut task_struct,_:*const user_regset,mut p:*mut c_uint,mut n:*mut c_uint,mut k:*const c_void,mut u:*const c_void)->c_int{if !cpu_has_feature(CPU_FTR_TM){return -ENODEV;}if !MSR_TM_ACTIVE((*t).thread.regs.msr){return -ENODATA;}user_regset_copyin(&mut p,&mut n,&mut k,&mut u,&mut (*t).thread.$f as *mut _,0,size_of::<u64>())} }; }
tm_simple_reg!(tm_ppr_active,tm_ppr_get,tm_ppr_set,tm_ppr);
tm_simple_reg!(tm_dscr_active,tm_dscr_get,tm_dscr_set,tm_dscr);

pub unsafe fn tm_cgpr32_get(t:*mut task_struct,r:*const user_regset,mut to:membuf)->c_int{gpr32_get_common(t,r,to,&mut (*t).thread.ckpt_regs.gpr[0]);membuf_zero(&mut to,ELF_NGREG*size_of::<u32>())}
pub unsafe fn tm_cgpr32_set(t:*mut task_struct,r:*const user_regset,p:*mut c_uint,n:*mut c_uint,k:*const c_void,u:*const c_void)->c_int{gpr32_set_common(t,r,p,n,k,u,&mut (*t).thread.ckpt_regs.gpr[0])}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
