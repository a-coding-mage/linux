// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of x86/kernel/ptrace.c.  Kernel dependencies are supplied externally. */

#[repr(C)]
pub enum X86Regset32 { General, Fp, Xfp, Xstate, Tls, Ioperm }
#[repr(C)]
pub enum X86Regset64 { General, Fp, Ioperm, Xstate, Ssp }

#[repr(C)]
pub struct PtRegsOffset { pub name: *const core::ffi::c_char, pub offset: i32 }

extern "C" {
    static regoffset_table: [PtRegsOffset; 0];
    fn strcmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> i32;
    fn task_pt_regs(task: *mut TaskStruct) -> *mut PtRegs;
    fn test_tsk_thread_flag(task: *mut TaskStruct, flag: i32) -> bool;
    fn clear_tsk_thread_flag(task: *mut TaskStruct, flag: i32);
    fn user_disable_single_step(task: *mut TaskStruct);
    fn force_sig_fault(sig: i32, code: i32, addr: *mut core::ffi::c_void);
    fn user_mode(regs: *mut PtRegs) -> bool;
}

#[repr(C)] pub struct TaskStruct { pub thread: ThreadStruct, pub thread_info: ThreadInfo }
#[repr(C)] pub struct ThreadInfo { pub status: u32 }
#[repr(C)] pub struct ThreadStruct {
    pub gs: u16, pub fsindex: u16, pub gsindex: u16, pub ds: u16, pub es: u16,
    pub fsbase: u64, pub gsbase: u64, pub virtual_dr6: usize, pub ptrace_dr7: usize,
    pub ptrace_bps: [*mut PerfEvent; 4], pub io_bitmap: *mut IoBitmap,
    pub trap_nr: i32, pub error_code: i32,
}
#[repr(C)] pub struct PtRegs { pub r15: usize, pub r14:usize, pub r13:usize, pub r12:usize,
    pub r11:usize, pub r10:usize, pub r9:usize, pub r8:usize, pub bx:usize, pub cx:usize,
    pub dx:usize, pub si:usize, pub di:usize, pub bp:usize, pub ax:usize, pub orig_ax:usize,
    pub ip:usize, pub cs:usize, pub flags:usize, pub sp:usize, pub ss:usize }
#[repr(C)] pub struct PerfEvent { pub attr: PerfEventAttr, pub hw: Hw }
#[repr(C)] pub struct Hw { pub info: HwInfo }
#[repr(C)] pub struct HwInfo { pub address: usize }
#[repr(C)] pub struct PerfEventAttr { pub bp_addr:usize, pub bp_len:i32, pub bp_type:i32, pub disabled:bool }
#[repr(C)] pub struct IoBitmap { pub max:usize, pub bitmap:*mut u8 }
#[repr(C)] pub struct UserRegset { pub size:usize }
#[repr(C)] pub struct Membuf { pub left:usize }

#[inline] unsafe fn invalid_selector(value:u16)->bool { value != 0 && (value & 3) != 3 }

pub unsafe fn regs_query_register_offset(name:*const core::ffi::c_char)->i32 {
    let mut r = regoffset_table.as_ptr();
    while !(*r).name.is_null() { if strcmp((*r).name,name)==0 { return (*r).offset; } r=r.add(1); }
    -22
}
pub unsafe fn regs_query_register_name(offset:u32)->*const core::ffi::c_char {
    let mut r=regoffset_table.as_ptr(); while !(*r).name.is_null() { if (*r).offset as u32==offset{return (*r).name;} r=r.add(1); } core::ptr::null()
}

unsafe fn pt_regs_access(regs:*mut PtRegs, offset:usize)->*mut usize { (regs as *mut usize).add(offset / core::mem::size_of::<usize>()) }
unsafe fn get_segment_reg(task:*mut TaskStruct, offset:usize)->u16 { *pt_regs_access(task_pt_regs(task),offset) as u16 }
unsafe fn set_segment_reg(task:*mut TaskStruct, offset:usize, value:u16)->i32 {
    if invalid_selector(value) { return -5; } *pt_regs_access(task_pt_regs(task),offset)=value as usize; 0
}
unsafe fn get_flags(task:*mut TaskStruct)->usize { (*task_pt_regs(task)).flags }
unsafe fn set_flags(task:*mut TaskStruct, value:usize)->i32 { (*task_pt_regs(task)).flags=value; 0 }
unsafe fn getreg(task:*mut TaskStruct, offset:usize)->usize { match offset { _ => *pt_regs_access(task_pt_regs(task),offset) } }
unsafe fn putreg(task:*mut TaskStruct, offset:usize, value:usize)->i32 { match offset { _ => {*pt_regs_access(task_pt_regs(task),offset)=value;0} } }

unsafe fn ptrace_triggered(_bp:*mut PerfEvent,_data:*mut core::ffi::c_void,_regs:*mut PtRegs) {}
unsafe fn ptrace_get_dr7(bp:*mut *mut PerfEvent)->usize { let mut v=0; for i in 0..4 { let b=*bp.add(i); if !b.is_null() && !(*b).attr.disabled { v |= 1usize << (i*2); } } v }
unsafe fn ptrace_set_debugreg(tsk:*mut TaskStruct,n:i32,val:usize)->i32 { if n<4 { (*tsk).thread.ptrace_bps[n as usize]=core::ptr::null_mut(); 0 } else if n==6 { (*tsk).thread.virtual_dr6=val;0 } else if n==7 { (*tsk).thread.ptrace_dr7=val;0 } else {-5} }

pub unsafe fn ptrace_disable(child:*mut TaskStruct) { user_disable_single_step(child); }

pub unsafe fn arch_ptrace(child:*mut TaskStruct, request:i64, addr:usize, data:usize)->i64 {
    match request { 1 => { if addr < core::mem::size_of::<PtRegs>() { return putreg(child,addr,data) as i64; } -5 },
        2 => { if addr < core::mem::size_of::<PtRegs>() { return getreg(child,addr) as i64; } -5 },
        _ => ptrace_request(child,request,addr,data) }
}
extern "C" { fn ptrace_request(*mut TaskStruct,i64,usize,usize)->i64; }

pub static mut xstate_fx_sw_bytes:[u64; 12]=[0;12];
pub unsafe fn update_regset_xstate_info(size:usize,xstate_mask:u64) { let _=size; xstate_fx_sw_bytes[0]=xstate_mask; }
pub unsafe fn task_user_regset_view(_task:*mut TaskStruct)->*const UserRegset { core::ptr::null() }
pub unsafe fn send_sigtrap(regs:*mut PtRegs,error_code:i32,si_code:i32) { let _=si_code; force_sig_fault(5,si_code,if user_mode(regs){(*regs).ip as *mut _}else{core::ptr::null_mut()}); }
pub unsafe fn user_single_step_report(regs:*mut PtRegs) { send_sigtrap(regs,0,1); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
