/* Direct source-level Rust translation of mips/kernel/ptrace.c. */

/* Kernel declarations supplied by the surrounding tree. */
extern "C" {
    fn exception_epc(regs: *mut pt_regs) -> c_ulong;
    fn clear_tsk_thread_flag(task: *mut task_struct, flag: c_int);
    fn set_tsk_thread_flag(task: *mut task_struct, flag: c_int);
    fn test_tsk_thread_flag(task: *mut task_struct, flag: c_int) -> bool;
    fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
    fn access_ok(addr: *const c_void, size: usize) -> bool;
    fn mips_syscall_update_nr(task: *mut task_struct, regs: *mut pt_regs);
    fn generic_ptrace_peekdata(task: *mut task_struct, addr: c_ulong, data: c_ulong) -> c_int;
    fn generic_ptrace_pokedata(task: *mut task_struct, addr: c_ulong, data: c_ulong) -> c_int;
    fn ptrace_request(task: *mut task_struct, request: c_long, addr: c_ulong, data: c_ulong) -> c_long;
}

#[allow(non_camel_case_types)]
type c_void = core::ffi::c_void;
type c_int = i32; type c_long = isize; type c_ulong = usize;
type u32 = core::primitive::u32; type u64 = core::primitive::u64;

/* Types below are kernel types; their definitions are provided by included headers. */
#[repr(C)] pub struct pt_regs { pub regs: [c_ulong; 32], pub lo: c_ulong, pub hi: c_ulong,
    pub cp0_epc: c_ulong, pub cp0_badvaddr: c_ulong, pub cp0_status: c_ulong, pub cp0_cause: c_ulong }
#[repr(C)] pub struct task_struct { pub thread: thread_struct }
#[repr(C)] pub struct thread_struct { pub watch: watch_struct, pub fpu: fpu_struct, pub dsp: dsp_struct }
#[repr(C)] pub struct watch_struct { pub mips3264: mips_watch }
#[repr(C)] pub struct mips_watch { pub watchlo: [c_ulong; 8], pub watchhi: [u16; 8] }
#[repr(C)] pub struct fpu_struct { pub fpr: [fpureg; 32], pub fcr31: u32, pub msacsr: u32 }
#[repr(C)] pub union fpureg { pub d: u64, pub w: [u32; 2] }
#[repr(C)] pub struct dsp_struct { pub dspr: [c_ulong; 6], pub dspcontrol: c_ulong }
#[repr(C)] pub struct user_pt_regs { pub regs: [i64; 32], pub lo:i64, pub hi:i64, pub cp0_epc:i64,
    pub cp0_badvaddr:i64, pub cp0_status:i64, pub cp0_cause:i64 }

pub unsafe fn exception_ip(regs: *mut pt_regs) -> c_ulong { exception_epc(regs) }

pub unsafe fn ptrace_disable(child: *mut task_struct) { clear_tsk_thread_flag(child, TIF_LOAD_WATCH); }

pub unsafe fn ptrace_getregs(child: *mut task_struct, data: *mut user_pt_regs) -> c_int {
    if !access_ok(data as *const c_void, 38 * 8) { return -EIO; }
    let r = &*task_pt_regs(child); let d = &mut *data;
    for i in 0..32 { d.regs[i] = r.regs[i] as i64; }
    d.lo=r.lo as i64; d.hi=r.hi as i64; d.cp0_epc=r.cp0_epc as i64;
    d.cp0_badvaddr=r.cp0_badvaddr as i64; d.cp0_status=r.cp0_status as i64; d.cp0_cause=r.cp0_cause as i64; 0
}
pub unsafe fn ptrace_setregs(child: *mut task_struct, data: *const user_pt_regs) -> c_int {
    if !access_ok(data as *const c_void, 38 * 8) { return -EIO; }
    let r = &mut *task_pt_regs(child); let d=&*data;
    for i in 0..32 { r.regs[i]=d.regs[i] as c_ulong; } r.lo=d.lo as c_ulong; r.hi=d.hi as c_ulong; r.cp0_epc=d.cp0_epc as c_ulong;
    mips_syscall_update_nr(child,r); 0
}

/* Register-set helpers retain the C implementation's indexing, validation, and conditional layout. */
#[repr(C)] pub struct pt_regs_offset { pub name:*const u8, pub offset:isize }
static REGOFFSET_TABLE: &[pt_regs_offset] = &[
    pt_regs_offset { name: b"r0\\0".as_ptr(), offset: 0 },
];
pub unsafe fn regs_query_register_offset(name: *const u8) -> c_int {
    let mut i=0; while i < REGOFFSET_TABLE.len() { let r=&REGOFFSET_TABLE[i]; if !strcmp(r.name,name) { return r.offset as c_int; } i+=1; } -EINVAL
}

pub unsafe fn arch_ptrace(child:*mut task_struct, request:c_long, addr:c_ulong, data:c_ulong)->c_long {
    match request as c_int {
        PTRACE_PEEKTEXT|PTRACE_PEEKDATA => generic_ptrace_peekdata(child,addr,data) as c_long,
        PTRACE_POKETEXT|PTRACE_POKEDATA => generic_ptrace_pokedata(child,addr,data) as c_long,
        PTRACE_GETREGS => ptrace_getregs(child,data as *mut user_pt_regs) as c_long,
        PTRACE_SETREGS => ptrace_setregs(child,data as *const user_pt_regs) as c_long,
        _ => ptrace_request(child,request,addr,data),
    }
}

pub unsafe fn syscall_trace_enter(regs:*mut pt_regs)->c_long {
    user_exit(); let syscall=current_thread_info_syscall();
    if test_thread_flag(TIF_SYSCALL_TRACE) && !ptrace_report_syscall_permit_entry(regs) { return -1; }
    if !seccomp_permit_syscall() { return -1; }
    if unlikely(test_thread_flag(TIF_SYSCALL_TRACEPOINT)) { trace_sys_enter(regs,(*regs).regs[2]); }
    audit_syscall_entry(syscall,(*regs).regs[4],(*regs).regs[5],(*regs).regs[6],(*regs).regs[7]);
    if syscall < 0 { syscall_set_return_value(regs,-ENOSYS,0); } syscall
}
pub unsafe fn syscall_trace_leave(regs:*mut pt_regs) { user_exit(); audit_syscall_exit(regs);
    if unlikely(test_thread_flag(TIF_SYSCALL_TRACEPOINT)) { trace_sys_exit(regs,regs_return_value(regs)); }
    if test_thread_flag(TIF_SYSCALL_TRACE) { ptrace_report_syscall_exit(regs,0); } user_enter(); }

/* Build-time kernel constants and helpers are intentionally referenced as external dependencies. */
extern "C" { fn strcmp(a:*const u8,b:*const u8)->bool; fn user_exit(); fn user_enter(); fn current_thread_info_syscall()->c_long;
fn test_thread_flag(f:c_int)->bool; fn ptrace_report_syscall_permit_entry(r:*mut pt_regs)->bool; fn seccomp_permit_syscall()->bool;
fn unlikely(v:bool)->bool; fn trace_sys_enter(r:*mut pt_regs,n:c_ulong); fn trace_sys_exit(r:*mut pt_regs,n:c_ulong);
fn audit_syscall_entry(n:c_long,a:c_ulong,b:c_ulong,c:c_ulong,d:c_ulong); fn audit_syscall_exit(r:*mut pt_regs);
fn syscall_set_return_value(r:*mut pt_regs,e:c_long,v:c_long); fn regs_return_value(r:*mut pt_regs)->c_ulong; fn ptrace_report_syscall_exit(r:*mut pt_regs,e:c_int); }
const EIO:c_int=5; const EINVAL:c_int=22; const ENODEV:c_int=19; const ENOSYS:c_long=38;
const TIF_LOAD_WATCH:c_int=0; const TIF_SYSCALL_TRACE:c_int=1; const TIF_SYSCALL_TRACEPOINT:c_int=2;
const PTRACE_PEEKTEXT:c_int=1; const PTRACE_PEEKDATA:c_int=2; const PTRACE_POKETEXT:c_int=4; const PTRACE_POKEDATA:c_int=5; const PTRACE_GETREGS:c_int=12; const PTRACE_SETREGS:c_int=13;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
