// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level translation of process.c. */

use core::ffi::{c_char, c_int, c_void};

// C headers and build-time configuration are supplied by the surrounding UML tree.
extern "C" {
    fn getpgrp() -> c_int;
    fn ptrace(request: c_int, pid: c_int, addr: *mut c_void, data: *mut c_void) -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn errno_location() -> *mut c_int;
    fn printk(fmt: *const c_char, ...);
    fn fatal_sigsegv() -> !;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn mmap(addr: *mut c_void, len: usize, prot: c_int, flags: c_int, fd: c_int, off: isize) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: usize) -> c_int;
    fn socketpair(domain: c_int, ty: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn clone(f: unsafe extern "C" fn(*mut c_void) -> c_int, stack: *mut c_void, flags: c_int, arg: *mut c_void) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, n: usize) -> isize;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn execveat(fd: c_int, path: *const c_char, argv: *const *const c_char, envp: *const *const c_char, flags: c_int) -> c_int;
    fn malloc(n: usize) -> *mut c_void;
    fn free(p: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn unlink(path: *const c_char) -> c_int;
    fn fchmod(fd: c_int, mode: u32) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn memfd_create(name: *const c_char, flags: u32) -> c_int;
    fn setjmp(env: *mut JmpBuf) -> c_int;
    fn longjmp(env: *mut JmpBuf, val: c_int) -> !;
}

type c_long = i64;
type JmpBuf = [JmpSlot; 1];
#[repr(C)]
pub struct JmpSlot { pub JB_IP: usize, pub JB_SP: usize }

#[repr(C)] pub struct mm_id { pub stack: usize, pub pid: c_int, pub syscall_fd_num: u32, pub syscall_fd_map: *mut c_int, pub sock: c_int, pub syscall_data_len: c_int }
#[repr(C)] pub struct stub_data { pub signal: c_int, pub futex: c_int, pub mctx_offset: usize, pub si_offset: usize, pub err: c_int, pub restart_wait: c_int, pub syscall_data_len: c_int, pub sigstack: [u8; 4096] }
#[repr(C)] pub struct faultinfo { pub _data: [u8; 128] }
#[repr(C)] pub struct uml_pt_regs { pub gp: *mut c_void, pub fp: *mut c_void, pub faultinfo: faultinfo, pub is_user: c_int }
#[repr(C)] pub struct iovec { pub iov_base: *mut c_void, pub iov_len: usize }
#[repr(C)] pub struct cmsghdr { pub cmsg_len: usize, pub cmsg_level: c_int, pub cmsg_type: c_int }
#[repr(C)] pub struct msghdr { pub msg_name: *mut c_void, pub msg_namelen: u32, pub msg_iov: *mut iovec, pub msg_iovlen: usize, pub msg_control: *mut c_void, pub msg_controllen: usize, pub msg_flags: c_int }
#[repr(C)] pub struct siginfo_t { pub si_syscall: c_int, pub _pad: [u8; 128] }
#[repr(C)] pub struct mcontext_t { pub _data: [u8; 512] }
#[repr(C)] struct stub_init_data { seccomp: c_int, stub_start: usize, signal_handler: usize, signal_restorer: usize, stub_code_fd: c_int, stub_code_offset: usize, stub_data_fd: c_int, stub_data_offset: usize }

extern "C" {
    fn current_mm_id() -> *mut mm_id; fn handle_syscall(r: *mut uml_pt_regs); fn singlestepping() -> c_int;
    fn set_stub_state(r: *mut uml_pt_regs, d: *mut stub_data, s: c_int) -> c_int; fn get_stub_state(r: *mut uml_pt_regs, d: *mut stub_data, x: *mut c_void) -> c_int;
    fn syscall_stub_dump_error(m: *mut mm_id); fn syscall_stub_flush(m: *mut mm_id) -> c_int; fn report_enomem(); fn current_mm_sync(); fn enter_turnstile(m: *mut mm_id); fn exit_turnstile(m: *mut mm_id);
    fn interrupt_end(); fn time_travel_print_bc_msg(); fn segv(f: faultinfo, a: c_int, b: c_int, c: *mut c_void, d: *mut c_void); fn relay_signal(s: c_int, i: *mut siginfo_t, r: *mut uml_pt_regs, x: *mut c_void);
    fn block_signals_trace(); fn unblock_signals_trace(); fn set_handler(s: c_int); fn uml_finishsetup(); fn initial_jmpbuf_lock(); fn initial_jmpbuf_unlock(); fn os_kill_ptraced_process(pid: c_int, t: c_int);
    fn phys_mapping(p: usize, off: *mut u64) -> c_int; fn uml_to_phys(p: *const c_char) -> usize; fn stub_signal_interrupt(); fn stub_signal_restorer(); fn stub_segv_handler();
    static mut using_seccomp: c_int; static mut kmalloc_ok: c_int; static mut time_travel_mode: c_int; static mut tt_extra_sched_jiffies: u64; static mut tempdir: *mut c_char; static mut sig_info: [Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut uml_pt_regs, *mut c_void)>; 64];
    static mut __syscall_stub_start: c_char; static mut stub_exe_start: c_char; static mut stub_exe_end: c_char;
}

const SIGALRM: c_int = 14; const SIGWINCH: c_int = 28; const SIGTRAP: c_int = 5; const SIGSEGV: c_int = 11; const SIGSTOP: c_int = 19; const SIGSYS: c_int = 31;
const SIGCHLD: c_int = 17; const SIGIO: c_int = 29; const SIGILL: c_int = 4; const SIGBUS: c_int = 7; const SIGFPE: c_int = 8;
const STUB_SIG_MASK: c_int = (1 << SIGALRM) | (1 << SIGWINCH); const STUB_DONE_MASK: c_int = 1 << SIGTRAP;

#[no_mangle] pub unsafe extern "C" fn is_skas_winch(pid: c_int, _fd: c_int, _data: *mut c_void) -> c_int { (pid == getpgrp()) as c_int }
unsafe fn ptrace_reg_name(_idx: c_int) -> *const c_char { b"\0".as_ptr() as *const c_char }
unsafe fn ptrace_dump_regs(pid: c_int) -> c_int { let mut regs = [0usize; 64]; if ptrace(12,pid,core::ptr::null_mut(),regs.as_mut_ptr() as *mut c_void)<0 { return -(*errno_location()); } printk(b"Stub registers -\n\0".as_ptr() as _,); for i in 0..regs.len() { printk(b"\t%s\t(%2d): %lx\n\0".as_ptr() as _,ptrace_reg_name(i as c_int),i as c_int,regs[i]); } 0 }

#[no_mangle] pub unsafe extern "C" fn wait_stub_done(pid: c_int) { let mut n; let mut status=0; loop { n=waitpid(pid,&mut status,0x40000000|2); if n<0 || status&0x7f != 0x7f { break } let sig=(status>>8)&0xff; if (1<<sig)&STUB_SIG_MASK==0 { break } if ptrace(7,pid,core::ptr::null_mut(),core::ptr::null_mut())!=0 { fatal_sigsegv() } } if ((1<<((status>>8)&0xff))&STUB_DONE_MASK)!=0{return} let err=ptrace_dump_regs(pid); if err!=0 { printk(b"Failed to get registers from stub\n\0".as_ptr() as _) } printk(b"failed to wait for SIGTRAP\n\0".as_ptr() as _); fatal_sigsegv() }

#[no_mangle] pub unsafe extern "C" fn wait_stub_done_seccomp(mm: *mut mm_id, mut running: c_int, wait_sigsys: c_int) { let data=(*mm).stack as *mut stub_data; loop { if running==0 { (*data).signal=0; (*data).futex=1; } loop { if (*mm).pid<0 { break } let ret=syscall(202,data as *mut c_void,0x81,1,core::ptr::null_mut(),core::ptr::null_mut(),0); if ret<0 && *errno_location()!=4 && *errno_location()!=11 { break } if (*data).futex!=1 { break } } if (*mm).pid<0 { break } running=0; if !(wait_sigsys!=0 && (*data).signal==SIGALRM) { break } } if (*data).mctx_offset > (*data).sigstack.len()-128 || (wait_sigsys!=0 && (*data).signal!=SIGSYS) { if (*mm).pid>=0 && current_mm_id()==mm { fatal_sigsegv() } } }

extern "C" { fn current_stub_stack() -> usize; }
unsafe fn get_skas_faultinfo(pid:c_int, fi:*mut faultinfo){ ptrace(7,pid,core::ptr::null_mut(),SIGSEGV as *mut c_void); wait_stub_done(pid); memcpy(fi,current_stub_stack() as *const c_void,core::mem::size_of::<faultinfo>()); }
unsafe fn handle_trap(r:*mut uml_pt_regs){ handle_syscall(r) }

static mut stub_exe_fd:c_int=0;
#[repr(C)] struct tramp_data { stub_data:*mut stub_data, sockpair:[c_int;2] }
unsafe extern "C" fn userspace_tramp(p:*mut c_void)->c_int { let t=p as *mut tramp_data; let mut init=stub_init_data{seccomp:using_seccomp,stub_start:0,signal_handler:0,signal_restorer:0,stub_code_fd:0,stub_code_offset:0,stub_data_fd:0,stub_data_offset:0}; init.stub_data_fd=phys_mapping((*t).stub_data as usize,core::ptr::null_mut()); if dup2((*t).sockpair[0],0)<0{exit(3)} close((*t).sockpair[0]); if write((*t).sockpair[1],&init as *const _ as _,core::mem::size_of_val(&init)) != core::mem::size_of_val(&init) {exit(4)} close((*t).sockpair[1]); syscall(322,stub_exe_fd,b"\0".as_ptr(),core::ptr::null_mut(),core::ptr::null_mut(),0x1000); exit(5) }

#[no_mangle] pub unsafe extern "C" fn start_userspace(mm:*mut mm_id)->c_int {
    let mut td=tramp_data{stub_data:(*mm).stack as *mut stub_data,sockpair:[0;2]};
    let stack=mmap(core::ptr::null_mut(),0x1000,7,0x22,-1,0);
    if stack as isize == -1 { return -*errno_location(); }
    if socketpair(1,1,0,td.sockpair.as_mut_ptr())!=0 { return -*errno_location(); }
    if using_seccomp!=0 { (*td.stub_data).futex=1; }
    (*mm).pid=clone(userspace_tramp,stack as *mut u8.add(0x1000) as *mut c_void,0x500011, &mut td as *mut _ as *mut c_void);
    if (*mm).pid<0 { close(td.sockpair[0]);close(td.sockpair[1]);(*mm).pid=-1;return -*errno_location(); }
    munmap(stack,0x1000); close(td.sockpair[0]); if using_seccomp!=0 {(*mm).sock=td.sockpair[1]} else {close(td.sockpair[1])}; 0
}

static mut unscheduled_userspace_iterations:c_int=0;
#[no_mangle] pub unsafe extern "C" fn userspace(regs:*mut uml_pt_regs){
    loop { let mm=current_mm_id(); enter_turnstile(mm); current_mm_sync();
        if using_seccomp!=0 { let d=(*mm).stack as *mut stub_data; let e=set_stub_state(regs,d,singlestepping()); if e!=0{fatal_sigsegv()}; (*d).syscall_data_len=(*mm).syscall_data_len; wait_stub_done_seccomp(mm,0,0); (*mm).syscall_data_len=0; (*mm).syscall_fd_num=0; if get_stub_state(regs,d,core::ptr::null_mut())!=0{fatal_sigsegv()} }
        else { if syscall_stub_flush(mm)!=0{fatal_sigsegv()} }
        exit_turnstile(mm); (*regs).is_user=1; interrupt_end();
    }
}

static mut initial_jmpbuf:JmpBuf=[JmpSlot{JB_IP:0,JB_SP:0}]; static mut cb_proc:Option<unsafe extern "C" fn(*mut c_void)>=None; static mut cb_arg:*mut c_void=core::ptr::null_mut(); static mut cb_back:*mut JmpBuf=core::ptr::null_mut(); static mut noreboot:bool=false;
#[no_mangle] pub unsafe extern "C" fn init_stub_exe_fd()->c_int { stub_exe_fd=memfd_create(b"uml-userspace\0".as_ptr() as _,0x13); if stub_exe_fd<0 { return -*errno_location(); } let mut p=&stub_exe_start as *const c_char; let end=&stub_exe_end as *const c_char; while p<end { let n=write(stub_exe_fd,p as _,(end as usize-p as usize)); if n<0 { if *errno_location()==4 {continue} return -*errno_location(); } p=p.add(n as usize); } 0 }
#[no_mangle] pub unsafe extern "C" fn new_thread(stack:*mut c_void,buf:*mut JmpBuf,handler:unsafe extern "C" fn()){(*buf)[0].JB_IP=handler as usize;(*buf)[0].JB_SP=stack as usize+0x10000-core::mem::size_of::<usize>()}
#[no_mangle] pub unsafe extern "C" fn switch_threads(me:*mut JmpBuf,you:*mut JmpBuf){if setjmp(me)==0{longjmp(you,1)}}
#[no_mangle] pub unsafe extern "C" fn start_idle_thread(stack:*mut c_void,switch_buf:*mut JmpBuf)->c_int{let n=setjmp(&mut initial_jmpbuf);match n{0=>{(*switch_buf)[0].JB_IP=uml_finishsetup as usize;(*switch_buf)[0].JB_SP=stack as usize+0x10000-8},1=>{if let Some(f)=cb_proc{f(cb_arg)};longjmp(cb_back,1)},2=>{kmalloc_ok=0;return 0},3=>{kmalloc_ok=0;return 1},_=>fatal_sigsegv()} longjmp(switch_buf,1)}
#[no_mangle] pub unsafe extern "C" fn initial_thread_cb_skas(proc:unsafe extern "C" fn(*mut c_void),arg:*mut c_void){let mut here:JmpBuf=[JmpSlot{JB_IP:0,JB_SP:0}];cb_proc=Some(proc);cb_arg=arg;cb_back=&mut here;if setjmp(&mut here)==0{longjmp(&mut initial_jmpbuf,1)}cb_proc=None;cb_arg=core::ptr::null_mut();cb_back=core::ptr::null_mut()}
#[no_mangle] pub unsafe extern "C" fn halt_skas(){longjmp(&mut initial_jmpbuf,2)}
#[no_mangle] pub unsafe extern "C" fn reboot_skas(){longjmp(&mut initial_jmpbuf,if noreboot{2}else{3})}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
