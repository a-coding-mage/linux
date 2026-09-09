// SPDX-License-Identifier: GPL-2.0
/* Kernel support for ptrace() and syscall tracing interfaces. */

// Linux headers and architecture-provided symbols are external dependencies.

const USER_PSW_BITS: usize = PSW_N | PSW_B | PSW_V | PSW_CB;

#[repr(C)]
pub enum parisc_regset { REGSET_GENERAL, REGSET_FP }

pub unsafe fn ptrace_disable(task: *mut task_struct) {
    clear_tsk_thread_flag(task, TIF_SINGLESTEP);
    clear_tsk_thread_flag(task, TIF_BLOCKSTEP);
    (*pa_psw(task)).r = 0; (*pa_psw(task)).t = 0;
    (*pa_psw(task)).h = 0; (*pa_psw(task)).l = 0;
}

pub unsafe fn user_disable_single_step(task: *mut task_struct) { ptrace_disable(task); }

pub unsafe fn user_enable_single_step(task: *mut task_struct) {
    clear_tsk_thread_flag(task, TIF_BLOCKSTEP);
    set_tsk_thread_flag(task, TIF_SINGLESTEP);
    if (*pa_psw(task)).n != 0 {
        (*task_regs(task)).iaoq[0] = (*task_regs(task)).iaoq[1];
        (*task_regs(task)).iasq[0] = (*task_regs(task)).iasq[1];
        (*task_regs(task)).iaoq[1] = (*task_regs(task)).iaoq[0] + 4;
        (*pa_psw(task)).n = 0; (*pa_psw(task)).x = 0;
        (*pa_psw(task)).y = 0; (*pa_psw(task)).z = 0; (*pa_psw(task)).b = 0;
        ptrace_disable(task);
        force_sig_fault_to_task(SIGTRAP, TRAP_TRACE,
            ((*task_regs(task)).iaoq[0] & !3) as *mut core::ffi::c_void, task);
        return;
    }
    (*pa_psw(task)).r = 1; (*pa_psw(task)).t = 0;
    (*pa_psw(task)).h = 0; (*pa_psw(task)).l = 0;
}

pub unsafe fn user_enable_block_step(task: *mut task_struct) {
    clear_tsk_thread_flag(task, TIF_SINGLESTEP);
    set_tsk_thread_flag(task, TIF_BLOCKSTEP);
    (*pa_psw(task)).r = 0; (*pa_psw(task)).t = 1;
    (*pa_psw(task)).h = 0; (*pa_psw(task)).l = 0;
}

pub unsafe fn arch_ptrace(child: *mut task_struct, request: c_long,
    addr: c_ulong, mut data: c_ulong) -> c_long {
    let datap = data as *mut c_ulong;
    let mut tmp: c_ulong;
    let mut ret: c_long = -EIO as c_long;
    let mut user_regs_struct_size = core::mem::size_of::<user_regs_struct>();
    #[cfg(CONFIG_64BIT)] if is_compat_task() { user_regs_struct_size /= 2; }
    match request {
        PTRACE_PEEKUSR => {
            if (addr & (core::mem::size_of::<c_ulong>() - 1)) != 0 || addr >= core::mem::size_of::<pt_regs>() { return ret; }
            tmp = *((task_regs(child) as *mut u8).add(addr as usize) as *mut c_ulong);
            ret = put_user(tmp, datap);
        }
        PTRACE_POKEUSR => {
            if addr == PT_PSW {
                data &= USER_PSW_BITS as c_ulong;
                (*task_regs(child)).gr[0] &= !(USER_PSW_BITS as c_ulong);
                (*task_regs(child)).gr[0] |= data; return 0;
            }
            if (addr & (core::mem::size_of::<c_ulong>() - 1)) != 0 || addr >= core::mem::size_of::<pt_regs>() { return ret; }
            if addr == PT_IAOQ0 || addr == PT_IAOQ1 { data |= PRIV_USER; }
            if (addr >= PT_GR1 && addr <= PT_GR31) || addr == PT_IAOQ0 || addr == PT_IAOQ1 ||
               (addr >= PT_FR0 && addr <= PT_FR31 + 4) || addr == PT_SAR {
                *((task_regs(child) as *mut u8).add(addr as usize) as *mut c_ulong) = data; ret = 0;
            }
        }
        PTRACE_GETREGS => return copy_regset_to_user(child, task_user_regset_view(current), REGSET_GENERAL as _, 0, user_regs_struct_size, datap),
        PTRACE_SETREGS => return copy_regset_from_user(child, task_user_regset_view(current), REGSET_GENERAL as _, 0, user_regs_struct_size, datap),
        PTRACE_GETFPREGS => return copy_regset_to_user(child, task_user_regset_view(current), REGSET_FP as _, 0, core::mem::size_of::<user_fp_struct>(), datap),
        PTRACE_SETFPREGS => return copy_regset_from_user(child, task_user_regset_view(current), REGSET_FP as _, 0, core::mem::size_of::<user_fp_struct>(), datap),
        _ => ret = ptrace_request(child, request, addr, data),
    }
    ret
}

#[cfg(CONFIG_COMPAT)]
unsafe fn translate_usr_offset(offset: compat_ulong_t) -> compat_ulong_t {
    if offset < 32*4 { offset*2 + 4 }
    else if offset < 32*4 + 32*8 { offset - 32*4 + PT_FR0 }
    else if offset < core::mem::size_of::<pt_regs>() as u64/2 + 32*4 { (offset - 32*4 - 32*8)*2 + PT_SR0 + 4 }
    else { core::mem::size_of::<pt_regs>() as compat_ulong_t }
}

#[cfg(CONFIG_COMPAT)]
pub unsafe fn compat_arch_ptrace(child: *mut task_struct, request: compat_long_t, mut addr: compat_ulong_t, data: compat_ulong_t) -> c_long {
    let mut ret = -EIO as c_long;
    match request {
        PTRACE_PEEKUSR => { if addr & (core::mem::size_of::<compat_uint_t>() as u64-1) != 0 { return ret; } addr=translate_usr_offset(addr); if addr >= core::mem::size_of::<pt_regs>() as u64 { return ret; } let tmp=*((task_regs(child) as *mut u8).add(addr as usize) as *mut compat_uint_t); ret=put_user(tmp, data as *mut compat_uint_t); }
        PTRACE_POKEUSR => { if addr==PT_PSW { ret=arch_ptrace(child,request,addr,data); } else { if addr & (core::mem::size_of::<compat_uint_t>() as u64-1)!=0{return ret;} addr=translate_usr_offset(addr); if addr>=core::mem::size_of::<pt_regs>() as u64{return ret;} let mut d=data; if addr==PT_IAOQ0+4 || addr==PT_IAOQ1+4 {d|=PRIV_USER;} if addr>=PT_FR0&&addr<=PT_FR31+4 { *((task_regs(child) as *mut u8).add(addr as usize) as *mut u32)=d as u32;ret=0;} else if (addr>=PT_GR1+4&&addr<=PT_GR31+4)||addr==PT_IAOQ0+4||addr==PT_IAOQ1+4||addr==PT_SAR+4 { *((task_regs(child) as *mut u8).add((addr-4) as usize) as *mut u32)=0; *((task_regs(child) as *mut u8).add(addr as usize) as *mut u32)=d as u32;ret=0; } } }
        PTRACE_GETREGS|PTRACE_SETREGS|PTRACE_GETFPREGS|PTRACE_SETFPREGS => arch_ptrace(child,request,addr,data),
        _ => compat_ptrace_request(child,request,addr,data),
    }
}

pub unsafe fn do_syscall_trace_enter(regs: *mut pt_regs) -> c_long {
    if test_thread_flag(TIF_SYSCALL_TRACE) { let permit=ptrace_report_syscall_permit_entry(regs); (*regs).gr[28]=(-ENOSYS) as _; if !permit { (*regs).gr[20]=!0; return -1; } }
    if !seccomp_permit_syscall() { return -1; }
    #[cfg(CONFIG_HAVE_SYSCALL_TRACEPOINTS)] if unlikely(test_thread_flag(TIF_SYSCALL_TRACEPOINT)) { trace_sys_enter(regs,(*regs).gr[20]); }
    #[cfg(CONFIG_64BIT)] if !is_compat_task() { audit_syscall_entry((*regs).gr[20],(*regs).gr[26],(*regs).gr[25],(*regs).gr[24],(*regs).gr[23]); } else
    { audit_syscall_entry((*regs).gr[20]&0xffffffff,(*regs).gr[26]&0xffffffff,(*regs).gr[25]&0xffffffff,(*regs).gr[24]&0xffffffff,(*regs).gr[23]&0xffffffff); }
    (*regs).gr[20] as u32 as i32 as c_long
}

pub unsafe fn do_syscall_trace_exit(regs: *mut pt_regs) { let stepping=test_thread_flag(TIF_SINGLESTEP)||test_thread_flag(TIF_BLOCKSTEP); audit_syscall_exit(regs); #[cfg(CONFIG_HAVE_SYSCALL_TRACEPOINTS)] if unlikely(test_thread_flag(TIF_SYSCALL_TRACEPOINT)){trace_sys_exit(regs,(*regs).gr[20]);} if stepping||test_thread_flag(TIF_SYSCALL_TRACE){ptrace_report_syscall_exit(regs,stepping);} }

#[repr(C)] pub struct pt_regs_offset { pub name: *const c_char, pub offset: c_int }

pub unsafe fn regs_query_register_offset(name:*const c_char)->c_int { let mut p=regoffset_table.as_ptr(); while !(*p).name.is_null(){if strcmp((*p).name,name)==0{return (*p).offset;}p=p.add(1);} -EINVAL }
pub unsafe fn regs_query_register_name(offset:c_uint)->*const c_char { let mut p=regoffset_table.as_ptr(); while !(*p).name.is_null(){if (*p).offset==offset as c_int{return (*p).name;}p=p.add(1);} core::ptr::null() }
pub unsafe fn regs_within_kernel_stack(regs:*mut pt_regs,addr:c_ulong)->bool { (addr & !(THREAD_SIZE-1)) == (kernel_stack_pointer(regs)&!(THREAD_SIZE-1)) }
pub unsafe fn regs_get_kernel_stack_nth(regs:*mut pt_regs,n:c_uint)->c_ulong { let addr=(kernel_stack_pointer(regs) as *mut c_ulong).sub(n as usize); if !regs_within_kernel_stack(regs,addr as c_ulong){0}else{*addr} }

pub unsafe fn fpr_get(target:*mut task_struct,_regset:*const user_regset,mut to:membuf)->c_int { let regs=task_regs(target); membuf_write(&mut to,(*regs).fr.as_mut_ptr() as *const _,ELF_NFPREG*core::mem::size_of::<u64>()); }
pub unsafe fn fpr_set(target:*mut task_struct,_regset:*const user_regset,mut pos:c_uint,mut count:c_uint,kbuf:*const core::ffi::c_void,ubuf:*const core::ffi::c_void)->c_int {
    let regs=task_regs(target); let mut k=kbuf as *const u64; let mut u=ubuf as *const u64; let mut reg:u64;
    pos/=core::mem::size_of::<u64>() as u32; count/=core::mem::size_of::<u64>() as u32;
    if !kbuf.is_null(){while count>0&&pos<ELF_NFPREG{(*regs).fr[pos as usize]=*k;k=k.add(1);pos+=1;count-=1;}}
    else{while count>0&&pos<ELF_NFPREG{if __get_user(&mut reg,u)!=0{return -EFAULT;}u=u.add(1);(*regs).fr[pos as usize]=reg;pos+=1;count-=1;}}
    user_regset_copyin_ignore(&mut pos,&mut count,&mut (k as *const _ as *mut _),&mut (u as *const _ as *mut _),ELF_NFPREG*8,!0);0
}

pub unsafe fn gpr_get(target:*mut task_struct,_regset:*const user_regset,mut to:membuf)->c_int { let regs=task_regs(target); for pos in 0..ELF_NGREG{membuf_store(&mut to,get_reg(regs,pos as c_int));}0 }
pub unsafe fn gpr_set(target:*mut task_struct,_regset:*const user_regset,mut pos:c_uint,mut count:c_uint,kbuf:*const core::ffi::c_void,ubuf:*const core::ffi::c_void)->c_int { let regs=task_regs(target); let mut k=kbuf as *const c_ulong; let mut u=ubuf as *const c_ulong; let mut reg:c_ulong; pos/=core::mem::size_of::<c_ulong>() as u32;count/=core::mem::size_of::<c_ulong>() as u32;if !kbuf.is_null(){while count>0&&pos<ELF_NGREG{set_reg(regs,pos as c_int,*k);k=k.add(1);pos+=1;count-=1;}}else{while count>0&&pos<ELF_NGREG{if __get_user(&mut reg,u)!=0{return -EFAULT;}u=u.add(1);set_reg(regs,pos as c_int,reg);pos+=1;count-=1;}}user_regset_copyin_ignore(&mut pos,&mut count,&mut (k as *const _ as *mut _),&mut (u as *const _ as *mut _),ELF_NGREG*core::mem::size_of::<c_ulong>() as u32,!0);0 }

// Native and compat regset tables, and their view objects, are declarations
// of the same externally consumed kernel ABI as in the C implementation.
extern "C" {
    static native_regsets: [user_regset; 2];
    static user_parisc_native_view: user_regset_view;
    #[cfg(CONFIG_64BIT)] static compat_regsets: [user_regset; 2];
    #[cfg(CONFIG_64BIT)] static user_parisc_compat_view: user_regset_view;
}

pub unsafe fn task_user_regset_view(task:*mut task_struct)->*const user_regset_view {
    #[cfg(CONFIG_64BIT)] if is_compat_task(){return &user_parisc_compat_view;}
    &user_parisc_native_view
}

unsafe fn get_reg(regs:*mut pt_regs,num:c_int)->c_ulong { match num { 0..=31=>(*regs).gr[num as usize], 32..=39=>(*regs).sr[(num-32) as usize], 40=>(*regs).iasq[0],41=>(*regs).iasq[1],42=>(*regs).iaoq[0],43=>(*regs).iaoq[1],44=>(*regs).sar,45=>(*regs).iir,46=>(*regs).isr,47=>(*regs).ior,48=>(*regs).ipsw,49=>(*regs).cr27,_=>0 } }
unsafe fn set_reg(regs:*mut pt_regs,num:c_int,val:c_ulong) { match num { 0=>{(*regs).gr[0]=( (*regs).gr[0]&!(USER_PSW_BITS as c_ulong))|(val&(USER_PSW_BITS as c_ulong));},1..=31=>(*regs).gr[num as usize]=val,42|43=>(*regs).iaoq[(num-42) as usize]=val|PRIV_USER,44=>(*regs).sar=val,_=>{} } }

extern "C" {
    static regoffset_table: [pt_regs_offset; 59];
    fn strcmp(a:*const c_char,b:*const c_char)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
