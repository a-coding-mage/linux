// SPDX-License-Identifier: GPL-2.0
/*
 * Test sigreturn to an unaligned address, ie. low 2 bits set.
 * Nothing bad should happen.
 * This was able to trigger warnings with CONFIG_PPC_RFI_SRR_DEBUG=y.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

const SIGUSR1: c_int = 10;
const SA_SIGINFO: c_int = 4;

#[repr(C)]
pub struct sigset_t {
	__val: [u64; 16],
}

#[repr(C)]
pub struct siginfo_t {
	_private: [u8; 0],
}

#[repr(C)]
pub struct mcontext_t {
	_private: [u8; 0],
}

#[repr(C)]
pub struct ucontext_t {
	uc_flags: u64,
	uc_link: *mut ucontext_t,
	uc_stack: stack_t,
	uc_sigmask: sigset_t,
	uc_mcontext: mcontext_t,
}

#[repr(C)]
pub struct stack_t {
	ss_sp: *mut c_void,
	ss_flags: c_int,
	ss_size: size_t,
}

#[repr(C)]
pub union sigaction_handler {
	sa_handler: sighandler_t,
	sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
}

#[repr(C)]
pub struct sigaction {
	sa_handler: sigaction_handler,
	sa_mask: sigset_t,
	sa_flags: c_int,
	sa_restorer: Option<unsafe extern "C" fn()>,
}

extern "C" {
	fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
	fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
	fn raise(sig: c_int) -> c_int;
	fn test_harness(
		test_function: Option<unsafe extern "C" fn() -> c_int>,
		name: *const c_char,
	) -> c_int;
}

extern "C" {
	static mut UCONTEXT_NIA: usize;
}

unsafe extern "C" fn sigusr1_handler(
	_signo: c_int,
	_info: *mut siginfo_t,
	ptr: *mut c_void,
) {
	let uc: *mut ucontext_t = ptr.cast();

	/* UCONTEXT_NIA(uc) |= 3; */
	let nia = UCONTEXT_NIA as *mut u64;
	*nia |= 3;
}

unsafe extern "C" fn test_sigreturn_unaligned() -> c_int {
	let mut action: sigaction = mem::zeroed();

	memset(
		&mut action as *mut sigaction as *mut c_void,
		0,
		mem::size_of::<sigaction>(),
	);
	action.sa_handler.sa_sigaction = Some(sigusr1_handler);
	action.sa_flags = SA_SIGINFO;

	if sigaction(SIGUSR1, &action, ptr::null_mut()) == -1 {
		return 1;
	}

	raise(SIGUSR1);

	0
}

fn main() {
	unsafe {
		std::process::exit(test_harness(
			Some(test_sigreturn_unaligned),
			c"sigreturn_unaligned".as_ptr(),
		));
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
