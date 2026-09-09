// SPDX-License-Identifier: GPL-2.0-only
/* Stack tracing support; translated from the AArch64 kernel implementation. */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum KunwindSource { Unknown, Frame, Caller, Task, RegsPc }

#[repr(C)]
#[derive(Copy, Clone)]
pub union UnwindFlags { pub all: usize, pub bits: UnwindFlagBits }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UnwindFlagBits { pub fgraph: usize, pub kretprobe: usize }

#[repr(C)]
pub struct KunwindState {
    pub common: UnwindState,
    pub task: *mut TaskStruct,
    pub graph_idx: i32,
    pub kr_cur: *mut LlistNode,
    pub source: KunwindSource,
    pub flags: UnwindFlags,
    pub regs: *mut PtRegs,
}

#[repr(C)] pub struct UnwindState { pub stacks: *mut StackInfo, pub nr_stacks: usize, pub fp: usize, pub pc: usize }
#[repr(C)] pub struct TaskStruct;
#[repr(C)] pub struct LlistNode;
#[repr(C)] pub struct StackInfo;
#[repr(C)] pub struct PtRegs { pub regs: [usize; 31], pub sp: usize, pub pc: usize }
#[repr(C)] pub struct FrameRecord { pub fp: usize, pub lr: usize }
#[repr(C)] pub struct FrameRecordMeta { pub typ: u32 }
#[repr(C)] pub struct FrameTail { pub fp: *mut FrameTail, pub lr: usize }
#[repr(C)] pub struct CompatFrameTail { pub fp: u32, pub sp: u32, pub lr: u32 }

pub type KunwindConsumeFn = unsafe extern "C" fn(*const KunwindState, *mut core::ffi::c_void) -> bool;
pub type StackTraceConsumeFn = unsafe extern "C" fn(*mut core::ffi::c_void, usize) -> bool;

extern "C" {
    static mut current: *mut TaskStruct;
    fn unwind_init_common(s: *mut UnwindState);
    fn thread_saved_fp(t: *mut TaskStruct) -> usize;
    fn thread_saved_pc(t: *mut TaskStruct) -> usize;
    fn unwind_find_stack(s: *mut UnwindState, addr: usize, size: usize) -> *mut StackInfo;
    fn unwind_consume_stack(s: *mut UnwindState, i: *mut StackInfo, addr: usize, size: usize);
    fn ptrauth_strip_kernel_insn_pac(pc: usize) -> usize;
    fn ptrauth_strip_user_insn_pac(pc: usize) -> usize;
    fn preemptible() -> bool; fn in_nmi() -> bool; fn current_in_efi() -> bool;
    fn stackinfo_get_task(t: *mut TaskStruct) -> StackInfo;
    fn stackinfo_get_unknown() -> StackInfo;
    fn stackinfo_get_irq() -> StackInfo; fn stackinfo_get_overflow() -> StackInfo;
    fn stackinfo_get_sdei_normal() -> StackInfo; fn stackinfo_get_sdei_critical() -> StackInfo;
    fn stackinfo_get_efi() -> StackInfo;
    fn user_mode(r: *mut PtRegs) -> bool; fn compat_user_mode(r: *const PtRegs) -> bool;
    fn try_get_task_stack(t: *mut TaskStruct) -> bool; fn put_task_stack(t: *mut TaskStruct);
    fn access_ok(p: *const core::ffi::c_void, n: usize) -> bool;
    fn pagefault_disable(); fn pagefault_enable();
    fn __copy_from_user_inatomic(d: *mut core::ffi::c_void, s: *const core::ffi::c_void, n: usize) -> usize;
    fn printk(fmt: *const u8, ...); fn pr_debug(fmt: *const u8, ...);
}

unsafe fn kunwind_init(s: *mut KunwindState, task: *mut TaskStruct) { unwind_init_common(&mut (*s).common); (*s).task=task; (*s).source=KunwindSource::Unknown; (*s).flags=UnwindFlags{all:0}; (*s).regs=core::ptr::null_mut(); }
unsafe fn kunwind_init_from_regs(s: *mut KunwindState, r: *mut PtRegs) { kunwind_init(s,current); (*s).regs=r; (*s).common.fp=(*r).regs[29]; (*s).common.pc=(*r).pc; (*s).source=KunwindSource::RegsPc; }
unsafe fn kunwind_init_from_caller(s: *mut KunwindState) { kunwind_init(s,current); (*s).common.fp=0; (*s).common.pc=0; (*s).source=KunwindSource::Caller; }
unsafe fn kunwind_init_from_task(s: *mut KunwindState,t:*mut TaskStruct) { kunwind_init(s,t); (*s).common.fp=thread_saved_fp(t); (*s).common.pc=thread_saved_pc(t); (*s).source=KunwindSource::Task; }

unsafe fn kunwind_next(s: *mut KunwindState) -> i32 { (*s).flags=UnwindFlags{all:0}; let fp=(*s).common.fp; if fp&7!=0{return -22}; let r=unwind_find_stack(&mut (*s).common,fp,16); if r.is_null(){return -22} let rec=fp as *const FrameRecord; let nf=(*rec).fp; let np=(*rec).lr; if nf==0&&np==0{return -2}; unwind_consume_stack(&mut (*s).common,r,fp,16); (*s).common.fp=nf; (*s).common.pc=ptrauth_strip_kernel_insn_pac(np); (*s).source=KunwindSource::Frame; 0 }
unsafe fn do_kunwind(s:*mut KunwindState, f:KunwindConsumeFn, c:*mut core::ffi::c_void)->i32 { loop { if !f(s,c){return -22} let e=kunwind_next(s); if e==-2{return 0} if e<0{return e} } }

#[repr(C)] struct ConsumeData { consume_entry: StackTraceConsumeFn, cookie:*mut core::ffi::c_void }
unsafe extern "C" fn arch_consume(s:*const KunwindState,c:*mut core::ffi::c_void)->bool { let d=&*(c as *const ConsumeData); (d.consume_entry)(d.cookie,(*s).common.pc) }
#[no_mangle] pub unsafe extern "C" fn arch_stack_walk(f:StackTraceConsumeFn,c:*mut core::ffi::c_void,t:*mut TaskStruct,r:*mut PtRegs){let d=ConsumeData{consume_entry:f,cookie:c};let mut s=core::mem::zeroed::<KunwindState>();s.common.stacks=core::ptr::null_mut();if r.is_null(){if t==current{kunwind_init_from_caller(&mut s)}else{kunwind_init_from_task(&mut s,t)}}else{kunwind_init_from_regs(&mut s,r)}let _=do_kunwind(&mut s,arch_consume,&d as *const _ as *mut _);}

#[no_mangle] pub unsafe extern "C" fn dump_backtrace(r:*mut PtRegs,mut t:*mut TaskStruct,log:*const u8){if !r.is_null()&&user_mode(r){return}if t.is_null(){t=current}if !try_get_task_stack(t){return}let mut s=core::mem::zeroed::<KunwindState>();kunwind_init_from_task(&mut s,t);let d=ConsumeData{consume_entry:arch_consume as StackTraceConsumeFn,cookie:log as *mut _};let _=do_kunwind(&mut s,arch_consume,&d as *const _ as *mut _);put_task_stack(t);}
#[no_mangle] pub unsafe extern "C" fn show_stack(t:*mut TaskStruct,_sp:*mut usize,log:*const u8){dump_backtrace(core::ptr::null_mut(),t,log);core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst)}

unsafe fn unwind_user_frame(t:*mut FrameTail,c:*mut core::ffi::c_void,f:StackTraceConsumeFn)->*mut FrameTail { let mut b=core::mem::zeroed::<FrameTail>();if !access_ok(t as *const _,core::mem::size_of::<FrameTail>()){return core::ptr::null_mut()}pagefault_disable();let e=__copy_from_user_inatomic(&mut b as *mut _ as *mut _,t as *const _,core::mem::size_of::<FrameTail>());pagefault_enable();if e!=0||!(f)(c,ptrauth_strip_user_insn_pac(b.lr)){return core::ptr::null_mut()}if (t as usize)>=(b.fp as usize){return core::ptr::null_mut()}b.fp}

#[no_mangle] pub unsafe extern "C" fn arch_stack_walk_user(f:StackTraceConsumeFn,c:*mut core::ffi::c_void,r:*const PtRegs){if !(f)(c,(*r).pc){return}if !compat_user_mode(r){let mut t=(*r).regs[29] as *mut FrameTail;while !t.is_null()&&(t as usize)&7==0{t=unwind_user_frame(t,c,f)}}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
