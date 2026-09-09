/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by the surrounding kernel translation unit.

pub unsafe fn unwind_get_return_address(state: *mut unwind_state) -> ::core::ffi::c_ulong {
	if unwind_done(state) {
		return 0;
	}
	if __kernel_text_address((*state).ip) {
		(*state).ip
	} else {
		0
	}
}

unsafe fn outside_of_stack(state: *mut unwind_state, sp: ::core::ffi::c_ulong) -> bool {
	(sp <= (*state).sp)
		|| (sp > (*state).stack_info.end - ::core::mem::size_of::<stack_frame>())
}

unsafe fn update_stack_info(state: *mut unwind_state, sp: ::core::ffi::c_ulong) -> bool {
	let info: *mut stack_info = &mut (*state).stack_info;
	let mask: *mut ::core::ffi::c_ulong = &mut (*state).stack_mask;

	/* New stack pointer leaves the current stack */
	if get_stack_info(sp, (*state).task, info, mask) != 0
		|| !on_stack(
			info,
			sp,
			::core::mem::size_of::<stack_frame>(),
		) {
		/* 'sp' does not point to a valid stack */
		return false;
	}
	true
}

#[inline]
unsafe fn is_final_pt_regs(
	state: *mut unwind_state,
	regs: *mut pt_regs,
) -> bool {
	/* user mode or kernel thread pt_regs at the bottom of task stack */
	if task_pt_regs((*state).task) == regs {
		return true;
	}

	/* user mode pt_regs at the bottom of irq stack */
	(*state).stack_info.type_ == STACK_TYPE_IRQ
		&& (*state).stack_info.end - ::core::mem::size_of::<pt_regs>()
			== regs as ::core::ffi::c_ulong
		&& (READ_ONCE_NOCHECK((*regs).psw.mask) & PSW_MASK_PSTATE) != 0
}

/* Avoid KMSAN false positives from touching uninitialized frames. */
pub unsafe fn unwind_next_frame(state: *mut unwind_state) -> bool {
	let info: *mut stack_info = &mut (*state).stack_info;
	let mut sf: *mut stack_frame;
	let mut regs: *mut pt_regs;
	let mut sp: ::core::ffi::c_ulong;
	let mut ip: ::core::ffi::c_ulong;
	let reliable: bool;

	regs = (*state).regs;
	if unlikely(!regs.is_null()) {
		sp = (*state).sp;
		sf = sp as *mut stack_frame;
		ip = READ_ONCE_NOCHECK((*sf).gprs[8]);
		reliable = false;
		regs = ::core::ptr::null_mut();
		/* skip bogus %r14 or if is the same as regs->psw.addr */
		if !__kernel_text_address(ip)
			|| (*state).ip == unwind_recover_ret_addr(state, ip)
		{
			(*state).regs = ::core::ptr::null_mut();
			return unwind_next_frame(state);
		}
	} else {
		sf = (*state).sp as *mut stack_frame;
		sp = READ_ONCE_NOCHECK((*sf).back_chain);
		if likely(sp != 0) {
			/* Non-zero back-chain points to the previous frame */
			if unlikely(outside_of_stack(state, sp)) {
				if !update_stack_info(state, sp) {
					goto out_err;
				}
			}
			sf = sp as *mut stack_frame;
			ip = READ_ONCE_NOCHECK((*sf).gprs[8]);
			reliable = true;
		} else {
			/* No back-chain, look for a pt_regs structure */
			sp = (*state).sp + STACK_FRAME_OVERHEAD;
			if !on_stack(info, sp, ::core::mem::size_of::<pt_regs>()) {
				goto out_err;
			}
			regs = sp as *mut pt_regs;
			if is_final_pt_regs(state, regs) {
				goto out_stop;
			}
			ip = READ_ONCE_NOCHECK((*regs).psw.addr);
			sp = READ_ONCE_NOCHECK((*regs).gprs[15]);
			if unlikely(outside_of_stack(state, sp)) {
				if !update_stack_info(state, sp) {
					goto out_err;
				}
			}
			reliable = true;
		}
	}

	/* Sanity check: ABI requires SP to be aligned 8 bytes. */
	if (sp & 0x7) != 0 {
		goto out_err;
	}

	/* Update unwind state */
	(*state).sp = sp;
	(*state).regs = regs;
	(*state).reliable = reliable;
	(*state).ip = unwind_recover_ret_addr(state, ip);
	return true;

out_err:
	(*state).error = true;
out_stop:
	(*state).stack_info.type_ = STACK_TYPE_UNKNOWN;
	false
}

/* Avoid KMSAN false positives from touching uninitialized frames. */
pub unsafe fn __unwind_start(
	state: *mut unwind_state,
	task: *mut task_struct,
	regs: *mut pt_regs,
	first_frame: ::core::ffi::c_ulong,
) {
	let info: *mut stack_info = &mut (*state).stack_info;
	let mut sf: *mut stack_frame;
	let mut ip: ::core::ffi::c_ulong;
	let mut sp: ::core::ffi::c_ulong;

	::core::ptr::write_bytes(state as *mut u8, 0, ::core::mem::size_of::<unwind_state>());
	(*state).task = task;
	(*state).regs = regs;

	/* Don't even attempt to start from user mode regs: */
	if !regs.is_null() && user_mode(regs) {
		(*info).type_ = STACK_TYPE_UNKNOWN;
		return;
	}

	/* Get the instruction pointer from pt_regs or the stack frame */
	if !regs.is_null() {
		ip = (*regs).psw.addr;
		sp = (*regs).gprs[15];
	} else if task == current {
		sp = current_frame_address();
	} else {
		sp = (*task).thread.ksp;
	}

	/* Get current stack pointer and initialize stack info */
	if !update_stack_info(state, sp) {
		/* Something is wrong with the stack pointer */
		(*info).type_ = STACK_TYPE_UNKNOWN;
		(*state).error = true;
		return;
	}

	if regs.is_null() {
		/* Stack frame is within valid stack */
		sf = sp as *mut stack_frame;
		ip = READ_ONCE_NOCHECK((*sf).gprs[8]);
	}

	/* Update unwind state */
	(*state).sp = sp;
	(*state).reliable = true;
	(*state).ip = unwind_recover_ret_addr(state, ip);

	if first_frame == 0 {
		return;
	}
	/* Skip through the call chain to the specified starting frame */
	while !unwind_done(state) {
		if on_stack(
			&mut (*state).stack_info,
			first_frame,
			::core::mem::size_of::<stack_frame>(),
		) {
			if (*state).sp >= first_frame {
				break;
			}
		}
		unwind_next_frame(state);
	}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
