// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2015 Thomas Meyer (thomas@m3y3r.de)
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// Kernel and UML declarations supplied by the corresponding dependencies.

const STUB_DATA_PAGES: usize = /* STUB_DATA_PAGES */ 0;
const UM_KERN_PAGE_SIZE: usize = /* UM_KERN_PAGE_SIZE */ 0;
const STUB_START: usize = /* STUB_START */ 0;
const FUTEX_IN_KERN: i32 = /* FUTEX_IN_KERN */ 0;
const SIGCHLD_IRQ: i32 = /* SIGCHLD_IRQ */ 0;

static mut MM_LIST_LOCK: spinlock_t = unsafe { core::mem::zeroed() };
static mut MM_LIST: list_head = unsafe { core::mem::zeroed() };

pub unsafe fn __get_turnstile(mm_id: *mut mm_id) -> *mut mutex {
    let ctx = container_of!(mm_id, mm_context, id);
    &mut (*ctx).turnstile
}

pub unsafe fn enter_turnstile(mm_id: *mut mm_id) {
    mutex_lock(__get_turnstile(mm_id));
}

pub unsafe fn exit_turnstile(mm_id: *mut mm_id) {
    mutex_unlock(__get_turnstile(mm_id));
}

pub unsafe fn init_new_context(task: *mut task_struct, mm: *mut mm_struct) -> i32 {
    let new_id = &mut (*mm).context.id;
    let mut stack: usize = 0;
    let mut ret: i32 = -ENOMEM;

    mutex_init(&mut (*mm).context.turnstile);
    spin_lock_init(&mut (*mm).context.sync_tlb_lock);

    stack = __get_free_pages(GFP_KERNEL | __GFP_ZERO, ilog2(STUB_DATA_PAGES));
    if stack == 0 {
        return ret;
    }

    new_id.stack = stack;
    new_id.syscall_data_len = 0;
    new_id.syscall_fd_num = 0;

    // scoped_guard(spinlock_irqsave, &mm_list_lock)
    spin_lock_irqsave(&mut MM_LIST_LOCK);
    // Insert into list, used for lookups when the child dies
    list_add(&mut (*mm).context.list, &mut MM_LIST);
    spin_unlock_irqrestore(&mut MM_LIST_LOCK);

    ret = start_userspace(new_id);
    if ret < 0 {
        free_pages(new_id.stack, ilog2(STUB_DATA_PAGES));
        return ret;
    }

    // Ensure the new MM is clean and nothing unwanted is mapped.
    unmap(new_id, 0, STUB_START);

    0
}

pub unsafe fn destroy_context(mm: *mut mm_struct) {
    let mmu = &mut (*mm).context;

    /*
     * If init_new_context wasn't called, this will be
     * zero, resulting in a kill(0), which will result in the
     * whole UML suddenly dying. Also, cover negative and
     * 1 cases, since they shouldn't happen either.
     *
     * Negative cases happen if the child died unexpectedly.
     */
    if mmu.id.pid >= 0 && mmu.id.pid < 2 {
        printk!(KERN_ERR "corrupt mm_context - pid = %d\n", mmu.id.pid);
        return;
    }

    spin_lock_irqsave(&mut MM_LIST_LOCK);
    list_del(&mut (*mm).context.list);
    spin_unlock_irqrestore(&mut MM_LIST_LOCK);

    if mmu.id.pid > 0 {
        os_kill_ptraced_process(mmu.id.pid, 1);
        mmu.id.pid = -1;
    }

    if using_seccomp && mmu.id.sock != 0 {
        os_close_file(mmu.id.sock);
    }

    free_pages(mmu.id.stack, ilog2(STUB_DATA_PAGES));
}

unsafe fn mm_sigchld_irq(irq: i32, dev: *mut core::ffi::c_void) -> irqreturn_t {
    let mut mm_context: *mut mm_context;
    let mut pid: pid_t;

    spin_lock(&mut MM_LIST_LOCK);

    while {
        pid = os_reap_child();
        pid > 0
    } {
        /* A child died; check if we have an MM with the PID. */
        list_for_each_entry!(mm_context, &mut MM_LIST, list, {
            if (*mm_context).id.pid == pid {
                printk!("Unexpectedly lost MM child! Affected tasks will segfault.");

                // Marks the MM as dead.
                (*mm_context).id.pid = -1;

                let stub_data = (*mm_context).id.stack as *mut stub_data;
                (*stub_data).futex = FUTEX_IN_KERN;
                // CONFIG_SMP conditional: wake the futex on SMP builds.
                #[cfg(CONFIG_SMP)]
                os_futex_wake(&mut (*stub_data).futex);

                break;
            }
        });
    }

    spin_unlock(&mut MM_LIST_LOCK);
    IRQ_HANDLED
}

unsafe fn init_child_tracking() -> i32 {
    let mut err: i32;

    spin_lock_init(&mut MM_LIST_LOCK);
    INIT_LIST_HEAD!(&mut MM_LIST);

    err = request_irq(SIGCHLD_IRQ, Some(mm_sigchld_irq), 0, "SIGCHLD", core::ptr::null_mut());
    if err < 0 {
        panic!("Failed to register SIGCHLD IRQ: {}", err);
    }

    0
}

// early_initcall(init_child_tracking)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
