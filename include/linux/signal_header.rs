/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by other translated kernel headers are intentionally
// referenced but not redefined here.

extern "C" {
    pub static mut print_fatal_signals: ::core::ffi::c_int;
    pub fn copy_siginfo_to_user(to: *mut siginfo_t, from: *const kernel_siginfo_t) -> ::core::ffi::c_int;
    pub fn copy_siginfo_from_user(to: *mut kernel_siginfo_t, from: *const siginfo_t) -> ::core::ffi::c_int;
    pub fn siginfo_layout(sig: ::core::ffi::c_uint, si_code: ::core::ffi::c_int) -> siginfo_layout;
    pub fn flush_sigqueue(queue: *mut sigpending);
    pub fn next_signal(pending: *mut sigpending, mask: *mut sigset_t) -> ::core::ffi::c_int;
    pub fn do_send_sig_info(sig: ::core::ffi::c_int, info: *mut kernel_siginfo, p: *mut task_struct, ty: pid_type) -> ::core::ffi::c_int;
    pub fn group_send_sig_info(sig: ::core::ffi::c_int, info: *mut kernel_siginfo, p: *mut task_struct, ty: pid_type) -> ::core::ffi::c_int;
    pub fn send_signal_locked(sig: ::core::ffi::c_int, info: *mut kernel_siginfo, p: *mut task_struct, ty: pid_type) -> ::core::ffi::c_int;
    pub fn sigprocmask(a: ::core::ffi::c_int, b: *mut sigset_t, c: *mut sigset_t) -> ::core::ffi::c_int;
    pub fn set_current_blocked(mask: *mut sigset_t);
    pub fn __set_current_blocked(mask: *const sigset_t);
    pub static mut show_unhandled_signals: bool;
    pub fn get_signal(ksig: *mut ksignal) -> bool;
    pub fn signal_setup_done(failed: ::core::ffi::c_int, ksig: *mut ksignal, stepping: ::core::ffi::c_int);
    pub fn exit_signals(tsk: *mut task_struct);
    pub fn kernel_sigaction(sig: ::core::ffi::c_int, handler: __sighandler_t);
    pub static mut sighand_cachep: *mut kmem_cache;
    pub fn unhandled_signal(tsk: *mut task_struct, sig: ::core::ffi::c_int) -> bool;
    pub fn signals_init();
    pub fn restore_altstack(uss: *const stack_t) -> ::core::ffi::c_int;
    pub fn __save_altstack(uss: *mut stack_t, sp: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn render_sigset_t(file: *mut seq_file, name: *const ::core::ffi::c_char, set: *mut sigset_t);
}

#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct siginfo_t { _private: [u8; 0] }
#[repr(C)] pub struct kernel_siginfo_t { _private: [u8; 0] }
#[repr(C)] pub struct kernel_siginfo { _private: [u8; 0] }
#[repr(C)] pub struct sigpending { pub signal: sigset_t, pub list: list_head }
#[repr(C)] pub struct sigset_t { pub sig: [::core::ffi::c_ulong; 4] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct ksignal { _private: [u8; 0] }
#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct stack_t { pub ss_sp: *mut ::core::ffi::c_void, pub ss_flags: ::core::ffi::c_int, pub ss_size: usize }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
pub type __sighandler_t = usize;
pub type pid_type = ::core::ffi::c_int;

#[repr(C)] #[derive(Copy, Clone)] pub enum siginfo_layout { SIL_KILL, SIL_TIMER, SIL_POLL, SIL_FAULT, SIL_FAULT_TRAPNO, SIL_FAULT_MCEERR, SIL_FAULT_BNDERR, SIL_FAULT_PKUERR, SIL_FAULT_PERF_EVENT, SIL_CHLD, SIL_RT, SIL_SYS }

pub const SI_EXPANSION_SIZE: usize = core::mem::size_of::<siginfo_t>() - core::mem::size_of::<kernel_siginfo_t>();

#[inline] pub unsafe fn copy_siginfo(to: *mut kernel_siginfo_t, from: *const kernel_siginfo_t) { core::ptr::copy_nonoverlapping(from, to, 1); }
#[inline] pub unsafe fn clear_siginfo(info: *mut kernel_siginfo_t) { core::ptr::write_bytes(info, 0, 1); }
#[inline] pub unsafe fn copy_siginfo_to_external(to: *mut siginfo_t, from: *const kernel_siginfo_t) { core::ptr::copy_nonoverlapping(from, to as *mut kernel_siginfo_t, 1); core::ptr::write_bytes((to as *mut u8).add(core::mem::size_of::<kernel_siginfo>()), 0, SI_EXPANSION_SIZE); }

#[inline] pub unsafe fn sigaddset(set: *mut sigset_t, sig_: ::core::ffi::c_int) { let sig = (sig_ - 1) as usize; if _NSIG_WORDS == 1 { (*set).sig[0] |= 1usize << sig; } else { (*set).sig[sig / _NSIG_BPW] |= 1usize << (sig % _NSIG_BPW); } }
#[inline] pub unsafe fn sigdelset(set: *mut sigset_t, sig_: ::core::ffi::c_int) { let sig = (sig_ - 1) as usize; if _NSIG_WORDS == 1 { (*set).sig[0] &= !(1usize << sig); } else { (*set).sig[sig / _NSIG_BPW] &= !(1usize << (sig % _NSIG_BPW)); } }
#[inline] pub unsafe fn sigismember(set: *mut sigset_t, sig_: ::core::ffi::c_int) -> ::core::ffi::c_int { let sig = (sig_ - 1) as usize; if _NSIG_WORDS == 1 { 1 & ((*set).sig[0] >> sig) as i32 } else { 1 & ((*set).sig[sig / _NSIG_BPW] >> (sig % _NSIG_BPW)) as i32 } }
#[inline] pub unsafe fn sigisemptyset(set: *mut sigset_t) -> ::core::ffi::c_int { match _NSIG_WORDS { 4 => (((*set).sig[3] | (*set).sig[2] | (*set).sig[1] | (*set).sig[0]) == 0) as i32, 2 => (((*set).sig[1] | (*set).sig[0]) == 0) as i32, 1 => ((*set).sig[0] == 0) as i32, _ => 0 } }
#[inline] pub unsafe fn sigequalsets(a: *const sigset_t, b: *const sigset_t) -> ::core::ffi::c_int { match _NSIG_WORDS { 4 => ((*a).sig[3] == (*b).sig[3] && (*a).sig[2] == (*b).sig[2] && (*a).sig[1] == (*b).sig[1] && (*a).sig[0] == (*b).sig[0]) as i32, 2 => ((*a).sig[1] == (*b).sig[1] && (*a).sig[0] == (*b).sig[0]) as i32, 1 => ((*a).sig[0] == (*b).sig[0]) as i32, _ => 0 } }

pub const fn sigmask(sig: usize) -> usize { 1usize << (sig - 1) }
#[inline] pub unsafe fn sigorsets(r: *mut sigset_t, a: *const sigset_t, b: *const sigset_t) { sigset_binop(r,a,b,|x,y| x|y); }
#[inline] pub unsafe fn sigandsets(r: *mut sigset_t, a: *const sigset_t, b: *const sigset_t) { sigset_binop(r,a,b,|x,y| x&y); }
#[inline] pub unsafe fn sigandnsets(r: *mut sigset_t, a: *const sigset_t, b: *const sigset_t) { sigset_binop(r,a,b,|x,y| x&!y); }
#[inline] unsafe fn sigset_binop<F: Fn(usize,usize)->usize>(r:*mut sigset_t,a:*const sigset_t,b:*const sigset_t,op:F) { for i in 0.._NSIG_WORDS { (*r).sig[i] = op((*a).sig[i], (*b).sig[i]); } }
#[inline] pub unsafe fn signotset(set: *mut sigset_t) { for i in 0.._NSIG_WORDS { (*set).sig[i] = !(*set).sig[i]; } }
#[inline] pub unsafe fn sigemptyset(set:*mut sigset_t) { for x in &mut (*set).sig { *x=0; } }
#[inline] pub unsafe fn sigfillset(set:*mut sigset_t) { for x in &mut (*set).sig { *x=!0; } }
#[inline] pub unsafe fn sigaddsetmask(set:*mut sigset_t, mask:usize){(*set).sig[0]|=mask;} #[inline] pub unsafe fn sigdelsetmask(set:*mut sigset_t,mask:usize){(*set).sig[0]&=!mask;} #[inline] pub unsafe fn sigtestsetmask(set:*mut sigset_t,mask:usize)->::core::ffi::c_int{(((*set).sig[0]&mask)!=0)as i32;}
#[inline] pub unsafe fn siginitset(set:*mut sigset_t,mask:usize){(*set).sig[0]=mask;for x in &mut (*set).sig[1..]{*x=0;}} #[inline] pub unsafe fn siginitsetinv(set:*mut sigset_t,mask:usize){(*set).sig[0]=!mask;for x in &mut (*set).sig[1..]{*x=!0;}}
#[inline] pub unsafe fn init_sigpending(sig:*mut sigpending){sigemptyset(&mut (*sig).signal);(*sig).list.next=&mut (*sig).list;(*sig).list.prev=&mut (*sig).list;}
#[inline] pub fn valid_signal(sig:usize)->::core::ffi::c_int{(sig<=_NSIG)as i32}
pub const SIG_KTHREAD: __sighandler_t = 2; pub const SIG_KTHREAD_KERNEL: __sighandler_t = 3;
#[inline] pub unsafe fn allow_signal(sig:i32){kernel_sigaction(sig,SIG_KTHREAD);} #[inline] pub unsafe fn allow_kernel_signal(sig:i32){kernel_sigaction(sig,SIG_KTHREAD_KERNEL);} #[inline] pub unsafe fn disallow_signal(sig:i32){kernel_sigaction(sig,SIG_IGN);}
pub const fn rt_sigmask(sig:usize)->usize{sigmask(sig)} pub const fn siginmask(sig:usize,mask:usize)->bool{sig>0&&sig<SIGRTMIN&&(rt_sigmask(sig)&mask)!=0}
pub const SIGEMT_MASK:usize=0; pub const SIG_KERNEL_ONLY_MASK:usize=rt_sigmask(SIGKILL)|rt_sigmask(SIGSTOP); pub const SIG_KERNEL_STOP_MASK:usize=rt_sigmask(SIGSTOP)|rt_sigmask(SIGTSTP)|rt_sigmask(SIGTTIN)|rt_sigmask(SIGTTOU); pub const SIG_KERNEL_IGNORE_MASK:usize=rt_sigmask(SIGCONT)|rt_sigmask(SIGCHLD)|rt_sigmask(SIGWINCH)|rt_sigmask(SIGURG);
pub const SIG_KERNEL_COREDUMP_MASK:usize=rt_sigmask(3)|rt_sigmask(4)|rt_sigmask(5)|rt_sigmask(6)|rt_sigmask(8)|rt_sigmask(11)|rt_sigmask(7)|rt_sigmask(31)|rt_sigmask(24)|rt_sigmask(25)|SIGEMT_MASK;
pub const SIG_SPECIFIC_SICODES_MASK:usize=rt_sigmask(4)|rt_sigmask(8)|rt_sigmask(11)|rt_sigmask(7)|rt_sigmask(5)|rt_sigmask(17)|rt_sigmask(29)|rt_sigmask(31)|SIGEMT_MASK;
#[inline] pub fn sig_kernel_only(sig:usize)->bool{siginmask(sig,SIG_KERNEL_ONLY_MASK)} #[inline] pub fn sig_kernel_coredump(sig:usize)->bool{siginmask(sig,0)} #[inline] pub fn sig_kernel_ignore(sig:usize)->bool{siginmask(sig,SIG_KERNEL_IGNORE_MASK)} #[inline] pub fn sig_kernel_stop(sig:usize)->bool{siginmask(sig,SIG_KERNEL_STOP_MASK)} #[inline] pub fn sig_specific_sicodes(sig:usize)->bool{siginmask(sig,0)}
#[cfg(feature="CONFIG_DYNAMIC_SIGFRAME")] pub unsafe fn sigaltstack_size_valid(size:usize)->bool { extern "C" { fn sigaltstack_size_valid(size:usize)->bool; } sigaltstack_size_valid(size) }
#[cfg(not(feature="CONFIG_DYNAMIC_SIGFRAME"))] pub const fn sigaltstack_size_valid(_size:usize)->bool { true }
pub const SIG_IGN:__sighandler_t=1; pub const SIG_DFL:__sighandler_t=0; pub const SIGKILL:usize=9; pub const SIGSTOP:usize=19; pub const SIGTSTP:usize=20; pub const SIGTTIN:usize=21; pub const SIGTTOU:usize=22; pub const SIGCONT:usize=18; pub const SIGCHLD:usize=17; pub const SIGWINCH:usize=28; pub const SIGURG:usize=23; pub const SIGRTMIN:usize=32; pub const _NSIG:usize=64; pub const _NSIG_WORDS:usize=4; pub const _NSIG_BPW:usize=64;
#[inline] pub unsafe fn arch_untagged_si_addr(addr:*mut ::core::ffi::c_void,_sig:usize,_si_code:usize)->*mut ::core::ffi::c_void{addr}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
