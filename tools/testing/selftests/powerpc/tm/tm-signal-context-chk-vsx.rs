// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2016, Cyril Bur, IBM Corp.
 *
 * Test the kernel's signal frame code.
 *
 * The kernel sets up two sets of ucontexts if the signal was to be
 * delivered while the thread was in a transaction (referred too as
 * first and second contexts).
 * Expected behaviour is that the checkpointed state is in the user
 * context passed to the signal handler (first context). The speculated
 * state can be accessed with the uc_link pointer (second context).
 *
 * The rationale for this is that if TM unaware code (which linked
 * against TM libs) installs a signal handler it will not know of the
 * speculative nature of the 'live' registers and may infer the wrong
 * thing.
 */

/* C dependencies: stdlib.h, stdio.h, string.h, signal.h, unistd.h, altivec.h,
 * utils.h, tm.h.
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::size_of;
use core::ptr;

const MAX_ATTEMPT: c_int = 500000;

const NV_VSX_REGS: usize = 12; /* Number of VSX registers to check. */
const VSX20: usize = 20; /* First VSX register to check in vsr20-vsr31 subset */
const FPR20: usize = 20; /* FPR20 overlaps VSX20 most significant doubleword */

const SIGUSR1: c_int = 10;
const SA_SIGINFO: c_int = 4;

type pid_t = c_int;
type sig_atomic_t = c_int;

#[repr(C)]
pub struct siginfo_t {
	_private: [u8; 0],
}

#[repr(C)]
pub struct sigset_t {
	_private: [u64; 16],
}

#[repr(C)]
pub struct sigaction {
	pub sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
	pub sa_mask: sigset_t,
	pub sa_flags: c_int,
	pub sa_restorer: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct mcontext_t {
	/* Fields supplied by the PowerPC ucontext ABI. */
	pub fp_regs: *mut f64,
	pub v_regs: *mut elf_vrregset_t,
}

#[repr(C)]
pub struct ucontext_t {
	pub uc_link: *mut ucontext_t,
	pub uc_mcontext: mcontext_t,
}

#[repr(C)]
pub struct elf_vrregset_t {
	_private: [u8; 0],
}

#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct vector_int(pub [c_int; 4]);

unsafe extern "C" {
	fn tm_signal_self_context_load(
		pid: pid_t,
		gprs: *mut c_long,
		fps: *mut f64,
		vms: *mut vector_int,
		vss: *mut vector_int,
	) -> c_long;

	fn have_htm() -> c_int;
	fn htm_is_synthetic() -> c_int;
	fn test_harness(
		test_function: unsafe extern "C" fn() -> c_int,
		name: *const c_char,
	) -> c_int;

	fn getpid() -> pid_t;
	fn sigemptyset(set: *mut sigset_t) -> c_int;
	fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
	fn perror(s: *const c_char);
	fn exit(status: c_int) -> !;
	fn printf(format: *const c_char, ...) -> c_int;
	fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
	fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
}

unsafe fn skip_if(cond: bool) {
	if cond {
		exit(0);
	}
}

unsafe fn fail_if(cond: bool) {
	if cond {
		exit(1);
	}
}

static mut FAIL: sig_atomic_t = 0;
static mut BROKEN: sig_atomic_t = 0;

/* Test only 12 vsx registers from vsr20 to vsr31 */
static mut VSXS: [vector_int; NV_VSX_REGS * 2] = [
	/* First context will be set with these values, i.e. non-speculative */
	/* VSX20     ,  VSX21      , ... */
	vector_int([1, 2, 3, 4]), vector_int([5, 6, 7, 8]), vector_int([9, 10, 11, 12]),
	vector_int([13, 14, 15, 16]), vector_int([17, 18, 19, 20]), vector_int([21, 22, 23, 24]),
	vector_int([25, 26, 27, 28]), vector_int([29, 30, 31, 32]), vector_int([33, 34, 35, 36]),
	vector_int([37, 38, 39, 40]), vector_int([41, 42, 43, 44]), vector_int([45, 46, 47, 48]),
	/* Second context will be set with these values, i.e. speculative */
	/* VSX20         ,  VSX21          , ... */
	vector_int([-1, -2, -3, -4]), vector_int([-5, -6, -7, -8]), vector_int([-9, -10, -11, -12]),
	vector_int([-13, -14, -15, -16]), vector_int([-17, -18, -19, -20]), vector_int([-21, -22, -23, -24]),
	vector_int([-25, -26, -27, -28]), vector_int([-29, -30, -31, -32]), vector_int([-33, -34, -35, -36]),
	vector_int([-37, -38, -39, -40]), vector_int([-41, -42, -43, -44]), vector_int([-45, -46, -47, -48]),
];

unsafe extern "C" fn signal_usr1(signum: c_int, info: *mut siginfo_t, uc: *mut c_void) {
	let mut i: c_int;
	let mut j: c_int;
	let mut vsx: [u8; size_of::<vector_int>()] = [0; size_of::<vector_int>()];
	let mut vsx_tm: [u8; size_of::<vector_int>()] = [0; size_of::<vector_int>()];
	let ucp: *mut ucontext_t = uc as *mut ucontext_t;
	let tm_ucp: *mut ucontext_t = (*ucp).uc_link;

	let _ = signum;
	let _ = info;

	/*
	 * FP registers and VMX registers overlap the VSX registers.
	 *
	 * FP registers (f0-31) overlap the most significant 64 bits of VSX
	 * registers vsr0-31, whilst VMX registers vr0-31, being 128-bit like
	 * the VSX registers, overlap fully the other half of VSX registers,
	 * i.e. vr0-31 overlaps fully vsr32-63.
	 *
	 * Due to compatibility and historical reasons (VMX/Altivec support
	 * appeared first on the architecture), VMX registers vr0-31 (so VSX
	 * half vsr32-63 too) are stored right after the v_regs pointer, in an
	 * area allocated for 'vmx_reverse' array (please see
	 * arch/powerpc/include/uapi/asm/sigcontext.h for details about the
	 * mcontext_t structure on Power).
	 *
	 * The other VSX half (vsr0-31) is hence stored below vr0-31/vsr32-63
	 * registers, but only the least significant 64 bits of vsr0-31. The
	 * most significant 64 bits of vsr0-31 (f0-31), as it overlaps the FP
	 * registers, is kept in fp_regs.
	 *
	 * v_regs is a 16 byte aligned pointer at the start of vmx_reserve
	 * (vmx_reserve may or may not be 16 aligned) where the v_regs structure
	 * exists, so v_regs points to where vr0-31 / vsr32-63 registers are
	 * fully stored. Since v_regs type is elf_vrregset_t, v_regs + 1
	 * skips all the slots used to store vr0-31 / vsr32-64 and points to
	 * part of one VSX half, i.e. v_regs + 1 points to the least significant
	 * 64 bits of vsr0-31. The other part of this half (the most significant
	 * part of vsr0-31) is stored in fp_regs.
	 *
	 */
	/* Get pointer to least significant doubleword of vsr0-31 */
	let vsx_ptr: *mut c_long = (*ucp).uc_mcontext.v_regs.add(1) as *mut c_long;
	let tm_vsx_ptr: *mut c_long = (*tm_ucp).uc_mcontext.v_regs.add(1) as *mut c_long;

	/* Check first context. Print all mismatches. */
	i = 0;
	while i < NV_VSX_REGS as c_int {
		/*
		 * Copy VSX most significant doubleword from fp_regs and
		 * copy VSX least significant one from 64-bit slots below
		 * saved VMX registers.
		 */
		memcpy(
			vsx.as_mut_ptr() as *mut c_void,
			(*ucp).uc_mcontext.fp_regs.add(FPR20 + i as usize) as *const c_void,
			8,
		);
		memcpy(
			vsx.as_mut_ptr().add(8) as *mut c_void,
			vsx_ptr.add(VSX20 + i as usize) as *const c_void,
			8,
		);

		FAIL = memcmp(
			vsx.as_ptr() as *const c_void,
			&VSXS[i as usize] as *const vector_int as *const c_void,
			size_of::<vector_int>(),
		);

		if FAIL != 0 {
			BROKEN = 1;
			printf(b"VSX%d (1st context) == 0x\0".as_ptr() as *const c_char, VSX20 as c_int + i);
			j = 0;
			while j < 16 {
				printf(b"%02x\0".as_ptr() as *const c_char, vsx[j as usize] as c_int);
				j += 1;
			}
			printf(b" instead of 0x\0".as_ptr() as *const c_char);
			j = 0;
			while j < 4 {
				printf(b"%08x\0".as_ptr() as *const c_char, VSXS[i as usize].0[j as usize]);
				j += 1;
			}
			printf(b" (expected)\n\0".as_ptr() as *const c_char);
		}
		i += 1;
	}

	/* Check second context. Print all mismatches. */
	i = 0;
	while i < NV_VSX_REGS as c_int {
		/*
		 * Copy VSX most significant doubleword from fp_regs and
		 * copy VSX least significant one from 64-bit slots below
		 * saved VMX registers.
		 */
		memcpy(
			vsx_tm.as_mut_ptr() as *mut c_void,
			(*tm_ucp).uc_mcontext.fp_regs.add(FPR20 + i as usize) as *const c_void,
			8,
		);
		memcpy(
			vsx_tm.as_mut_ptr().add(8) as *mut c_void,
			tm_vsx_ptr.add(VSX20 + i as usize) as *const c_void,
			8,
		);

		FAIL = memcmp(
			vsx_tm.as_ptr() as *const c_void,
			&VSXS[NV_VSX_REGS + i as usize] as *const vector_int as *const c_void,
			size_of::<vector_int>(),
		);

		if FAIL != 0 {
			BROKEN = 1;
			printf(b"VSX%d (2nd context) == 0x\0".as_ptr() as *const c_char, VSX20 as c_int + i);
			j = 0;
			while j < 16 {
				printf(b"%02x\0".as_ptr() as *const c_char, vsx_tm[j as usize] as c_int);
				j += 1;
			}
			printf(b" instead of 0x\0".as_ptr() as *const c_char);
			j = 0;
			while j < 4 {
				printf(
					b"%08x\0".as_ptr() as *const c_char,
					VSXS[NV_VSX_REGS + i as usize].0[j as usize],
				);
				j += 1;
			}
			printf(b"(expected)\n\0".as_ptr() as *const c_char);
		}
		i += 1;
	}
}

unsafe extern "C" fn tm_signal_context_chk() -> c_int {
	let mut act: sigaction = core::mem::zeroed();
	let mut i: c_int;
	let mut rc: c_long;
	let pid: pid_t = getpid();

	skip_if(have_htm() == 0);
	skip_if(htm_is_synthetic() != 0);

	act.sa_sigaction = Some(signal_usr1);
	sigemptyset(&mut act.sa_mask);
	act.sa_flags = SA_SIGINFO;
	if sigaction(SIGUSR1, &act, ptr::null_mut()) < 0 {
		perror(b"sigaction sigusr1\0".as_ptr() as *const c_char);
		exit(1);
	}

	i = 0;
	while i < MAX_ATTEMPT && BROKEN == 0 {
		/*
		 * tm_signal_self_context_load will set both first and second
		 * contexts accordingly to the values passed through non-NULL
		 * array pointers to it, in that case 'vsxs', and invoke the
		 * signal handler installed for SIGUSR1.
		 */
		rc = tm_signal_self_context_load(
			pid,
			ptr::null_mut(),
			ptr::null_mut(),
			ptr::null_mut(),
			VSXS.as_mut_ptr(),
		);
		fail_if(rc != pid as c_long);
		i += 1;
	}

	BROKEN
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
	test_harness(
		tm_signal_context_chk,
		b"tm_signal_context_chk_vsx\0".as_ptr() as *const c_char,
	)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
