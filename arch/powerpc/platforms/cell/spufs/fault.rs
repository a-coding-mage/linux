// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Low-level SPU handling
 *
 * (C) Copyright IBM Deutschland Entwicklung GmbH 2005
 *
 * Author: Arnd Bergmann <arndb@de.ibm.com>
 */
// Dependencies supplied by the surrounding kernel translation.

/**
 * Handle an SPE event, depending on context SPU_CREATE_EVENTS_ENABLED flag.
 *
 * If the context was created with events, we just set the return event.
 * Otherwise, send an appropriate signal to the process.
 */
unsafe fn spufs_handle_event(ctx: *mut spu_context, ea: c_ulong, event_type: c_int) {
	if (*ctx).flags & SPU_CREATE_EVENTS_ENABLED != 0 {
		(*ctx).event_return |= event_type;
		wake_up_all(&mut (*ctx).stop_wq);
		return;
	}

	match event_type {
		SPE_EVENT_INVALID_DMA => {
			force_sig_fault(SIGBUS, BUS_OBJERR, core::ptr::null_mut());
		}
		SPE_EVENT_SPE_DATA_STORAGE => {
			((*ctx).ops).restart_dma(ctx);
			force_sig_fault(SIGSEGV, SEGV_ACCERR, ea as *mut c_void);
		}
		SPE_EVENT_DMA_ALIGNMENT => {
			/* DAR isn't set for an alignment fault :( */
			force_sig_fault(SIGBUS, BUS_ADRALN, core::ptr::null_mut());
		}
		SPE_EVENT_SPE_ERROR => {
			force_sig_fault(
				SIGILL,
				ILL_ILLOPC,
				((((*ctx).ops).npc_read(ctx) as c_ulong).wrapping_sub(4)) as *mut c_void,
			);
		}
		_ => {}
	}
}

pub unsafe fn spufs_handle_class0(ctx: *mut spu_context) -> c_int {
	let stat: c_ulong = (*ctx).csa.class_0_pending & CLASS0_INTR_MASK;

	if likely(stat == 0) {
		return 0;
	}

	if stat & CLASS0_DMA_ALIGNMENT_INTR != 0 {
		spufs_handle_event(ctx, (*ctx).csa.class_0_dar, SPE_EVENT_DMA_ALIGNMENT);
	}

	if stat & CLASS0_INVALID_DMA_COMMAND_INTR != 0 {
		spufs_handle_event(ctx, (*ctx).csa.class_0_dar, SPE_EVENT_INVALID_DMA);
	}

	if stat & CLASS0_SPU_ERROR_INTR != 0 {
		spufs_handle_event(ctx, (*ctx).csa.class_0_dar, SPE_EVENT_SPE_ERROR);
	}

	(*ctx).csa.class_0_pending = 0;

	-EIO
}

/*
 * bottom half handler for page faults, we can't do this from
 * interrupt context, since we might need to sleep.
 * we also need to give up the mutex so we can get scheduled
 * out while waiting for the backing store.
 *
 * TODO: try calling hash_page from the interrupt handler first
 *       in order to speed up the easy case.
 */
pub unsafe fn spufs_handle_class1(ctx: *mut spu_context) -> c_int {
	let ea: u64;
	let dsisr: u64;
	let access: u64;
	let mut flags: c_ulong;
	let mut flt: vm_fault_t = 0;
	let mut ret: c_int;

	/*
	 * dar and dsisr get passed from the registers
	 * to the spu_context, to this function, but not
	 * back to the spu if it gets scheduled again.
	 *
	 * if we don't handle the fault for a saved context
	 * in time, we can still expect to get the same fault
	 * the immediately after the context restore.
	 */
	ea = (*ctx).csa.class_1_dar;
	dsisr = (*ctx).csa.class_1_dsisr;

	if dsisr & (MFC_DSISR_PTE_NOT_FOUND | MFC_DSISR_ACCESS_DENIED) == 0 {
		return 0;
	}

	spuctx_switch_state(ctx, SPU_UTIL_IOWAIT);

	pr_debug!("ctx {:p}: ea {:016x}, dsisr {:016x} state {}\n", ctx, ea, dsisr, (*ctx).state);

	(*ctx).stats.hash_flt += 1;
	if (*ctx).state == SPU_STATE_RUNNABLE {
		(*(*ctx).spu).stats.hash_flt += 1;
	}

	/* we must not hold the lock when entering copro_handle_mm_fault */
	spu_release(ctx);

	access = _PAGE_PRESENT | _PAGE_READ;
	let access = access | if dsisr & MFC_DSISR_ACCESS_PUT != 0 { _PAGE_WRITE } else { 0 };
	local_irq_save(&mut flags);
	ret = hash_page(ea, access, 0x300, dsisr);
	local_irq_restore(flags);

	/* hashing failed, so try the actual fault handler */
	if ret != 0 {
		ret = copro_handle_mm_fault((*current).mm, ea, dsisr, &mut flt);
	}

	/*
	 * This is nasty: we need the state_mutex for all the bookkeeping even
	 * if the syscall was interrupted by a signal. ewww.
	 */
	mutex_lock(&mut (*ctx).state_mutex);

	/*
	 * Clear dsisr under ctxt lock after handling the fault, so that
	 * time slicing will not preempt the context while the page fault
	 * handler is running. Context switch code removes mappings.
	 */
	(*ctx).csa.class_1_dar = 0;
	(*ctx).csa.class_1_dsisr = 0;

	/*
	 * If we handled the fault successfully and are in runnable
	 * state, restart the DMA.
	 * In case of unhandled error report the problem to user space.
	 */
	if ret == 0 {
		if flt & VM_FAULT_MAJOR != 0 {
			(*ctx).stats.maj_flt += 1;
		} else {
			(*ctx).stats.min_flt += 1;
		}
		if (*ctx).state == SPU_STATE_RUNNABLE {
			if flt & VM_FAULT_MAJOR != 0 {
				(*(*ctx).spu).stats.maj_flt += 1;
			} else {
				(*(*ctx).spu).stats.min_flt += 1;
			}
		}

		if !(*ctx).spu.is_null() {
			((*ctx).ops).restart_dma(ctx);
		}
	} else {
		spufs_handle_event(ctx, ea, SPE_EVENT_SPE_DATA_STORAGE);
	}

	spuctx_switch_state(ctx, SPU_UTIL_SYSTEM);
	ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
