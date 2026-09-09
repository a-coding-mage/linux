// SPDX-License-Identifier: GPL-2.0
/*
 * Performance events callchain code, extracted from core.c:
 *
 *  Copyright (C) 2008 Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
 *  Copyright (C) 2008-2011 Red Hat, Inc., Ingo Molnar
 *  Copyright (C) 2008-2011 Red Hat, Inc., Peter Zijlstra
 *  Copyright  ©  2009 Paul Mackerras, IBM Corp. <paulus@au1.ibm.com>
 */

#[repr(C)]
pub struct callchain_cpus_entries {
    pub rcu_head: rcu_head,
    pub cpu_entries: [*mut perf_callchain_entry; 0],
}

pub static mut sysctl_perf_event_max_stack: i32 = PERF_MAX_STACK_DEPTH;
pub static mut sysctl_perf_event_max_contexts_per_stack: i32 = PERF_MAX_CONTEXTS_PER_STACK;
static six_hundred_forty_kb: i32 = 640 * 1024;

#[inline]
unsafe fn perf_callchain_entry__sizeof() -> usize {
    core::mem::size_of::<perf_callchain_entry>()
        + core::mem::size_of::<u64>()
            * (sysctl_perf_event_max_stack as usize
                + sysctl_perf_event_max_contexts_per_stack as usize)
}

static mut callchain_recursion: [u8; PERF_NR_CONTEXTS as usize] = [0; PERF_NR_CONTEXTS as usize];
static mut nr_callchain_events: atomic_t = atomic_t { counter: 0 };
static mut callchain_mutex: mutex = mutex {};
static mut callchain_cpus_entries: *mut callchain_cpus_entries = core::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn perf_callchain_kernel(
    _entry: *mut perf_callchain_entry_ctx,
    _regs: *mut pt_regs,
) {
}

#[no_mangle]
pub unsafe extern "C" fn perf_callchain_user(
    _entry: *mut perf_callchain_entry_ctx,
    _regs: *mut pt_regs,
) {
}

unsafe fn release_callchain_buffers_rcu(head: *mut rcu_head) {
    let entries: *mut callchain_cpus_entries = container_of!(head, callchain_cpus_entries, rcu_head);
    let mut cpu: i32;

    for_each_possible_cpu!(cpu) {
        kfree((*entries).cpu_entries[cpu as usize] as *mut core::ffi::c_void);
    }

    kfree(entries as *mut core::ffi::c_void);
}

unsafe fn release_callchain_buffers() {
    let entries = callchain_cpus_entries;
    RCU_INIT_POINTER!(callchain_cpus_entries, core::ptr::null_mut());
    call_rcu!(&mut (*entries).rcu_head, release_callchain_buffers_rcu);
}

unsafe fn alloc_callchain_buffers() -> i32 {
    let mut cpu: i32;
    let mut size: usize;
    let entries: *mut callchain_cpus_entries;

    /* We can't use the percpu allocation API for data that can be accessed
     * from NMI. Use a temporary manual per cpu allocation until that gets
     * sorted out. */
    size = core::mem::offset_of!(callchain_cpus_entries, cpu_entries)
        + core::mem::size_of::<*mut perf_callchain_entry>() * nr_cpu_ids as usize;

    entries = kzalloc(size, GFP_KERNEL) as *mut callchain_cpus_entries;
    if entries.is_null() {
        return -ENOMEM;
    }

    size = perf_callchain_entry__sizeof() * PERF_NR_CONTEXTS as usize;

    for_each_possible_cpu!(cpu) {
        (*entries).cpu_entries[cpu as usize] =
            kmalloc_node(size, GFP_KERNEL, cpu_to_node(cpu));
        if (*entries).cpu_entries[cpu as usize].is_null() {
            for_each_possible_cpu!(cpu) {
                kfree((*entries).cpu_entries[cpu as usize] as *mut core::ffi::c_void);
            }
            kfree(entries as *mut core::ffi::c_void);
            return -ENOMEM;
        }
    }

    rcu_assign_pointer!(callchain_cpus_entries, entries);
    0
}

#[no_mangle]
pub unsafe extern "C" fn get_callchain_buffers(event_max_stack: i32) -> i32 {
    let mut err: i32 = 0;
    let count: i32;

    mutex_lock!(&mut callchain_mutex);

    count = atomic_inc_return!(&mut nr_callchain_events);
    if WARN_ON_ONCE!(count < 1) {
        err = -EINVAL;
        goto_exit!();
    }

    if event_max_stack > sysctl_perf_event_max_stack {
        err = -EOVERFLOW;
        goto_exit!();
    }

    if count == 1 {
        err = alloc_callchain_buffers();
    }

    if err != 0 {
        atomic_dec!(&mut nr_callchain_events);
    }
    mutex_unlock!(&mut callchain_mutex);
    err
}

#[no_mangle]
pub unsafe extern "C" fn put_callchain_buffers() {
    if atomic_dec_and_mutex_lock!(&mut nr_callchain_events, &mut callchain_mutex) {
        release_callchain_buffers();
        mutex_unlock!(&mut callchain_mutex);
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_callchain_entry(rctx: *mut i32) -> *mut perf_callchain_entry {
    let cpu: i32;
    let entries: *mut callchain_cpus_entries;

    *rctx = get_recursion_context!(this_cpu_ptr!(callchain_recursion.as_mut_ptr()));
    if *rctx == -1 {
        return core::ptr::null_mut();
    }

    entries = rcu_dereference!(callchain_cpus_entries);
    if entries.is_null() {
        put_recursion_context!(this_cpu_ptr!(callchain_recursion.as_mut_ptr()), *rctx);
        return core::ptr::null_mut();
    }

    cpu = smp_processor_id!();
    ((*entries).cpu_entries[cpu as usize] as *mut u8)
        .add(*rctx as usize * perf_callchain_entry__sizeof()) as *mut perf_callchain_entry
}

#[no_mangle]
pub unsafe extern "C" fn put_callchain_entry(rctx: i32) {
    put_recursion_context!(this_cpu_ptr!(callchain_recursion.as_mut_ptr()), rctx);
}

unsafe fn fixup_uretprobe_trampoline_entries(
    entry: *mut perf_callchain_entry,
    start_entry_idx: i32,
) {
    /* CONFIG_UPROBES conditional from the C implementation. */
    #[cfg(CONFIG_UPROBES)]
    {
        let utask = (*current).utask;
        if utask.is_null() || (*utask).return_instances.is_null() {
            return;
        }
        let mut cur_ip = (*entry).ip.as_mut_ptr().add(start_entry_idx as usize);
        let last_ip = (*entry).ip.as_mut_ptr().add((*entry).nr as usize - 1);
        let mut ri = (*utask).return_instances;
        let tramp_addr = uprobe_get_trampoline_vaddr();
        while !ri.is_null() && cur_ip <= last_ip {
            if *cur_ip == tramp_addr {
                *cur_ip = (*ri).orig_ret_vaddr;
                ri = (*ri).next;
            }
            cur_ip = cur_ip.add(1);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_perf_callchain(
    mut regs: *mut pt_regs,
    kernel: bool,
    user: bool,
    max_stack: u32,
    crosstask: bool,
    add_mark: bool,
    defer_cookie: u64,
) -> *mut perf_callchain_entry {
    let entry: *mut perf_callchain_entry;
    let mut ctx: perf_callchain_entry_ctx;
    let mut rctx: i32 = 0;
    let mut start_entry_idx: i32;

    if crosstask && user && !kernel {
        return core::ptr::null_mut();
    }
    entry = get_callchain_entry(&mut rctx);
    if entry.is_null() {
        return core::ptr::null_mut();
    }

    ctx.entry = entry;
    ctx.max_stack = max_stack;
    ctx.nr = (*entry).nr = 0;
    ctx.contexts = 0;
    ctx.contexts_maxed = false;

    if kernel && !user_mode(regs) {
        if add_mark { perf_callchain_store_context!(&mut ctx, PERF_CONTEXT_KERNEL); }
        perf_callchain_kernel(&mut ctx, regs);
    }
    if user && !crosstask {
        if !user_mode(regs) {
            if !is_user_task!(current) { put_callchain_entry(rctx); return entry; }
            regs = task_pt_regs!(current);
        }
        if defer_cookie != 0 {
            perf_callchain_store_context!(&mut ctx, PERF_CONTEXT_USER_DEFERRED);
            perf_callchain_store_context!(&mut ctx, defer_cookie);
            put_callchain_entry(rctx);
            return entry;
        }
        if add_mark { perf_callchain_store_context!(&mut ctx, PERF_CONTEXT_USER); }
        start_entry_idx = (*entry).nr;
        perf_callchain_user(&mut ctx, regs);
        fixup_uretprobe_trampoline_entries(entry, start_entry_idx);
    }
    put_callchain_entry(rctx);
    entry
}

static unsafe fn perf_event_max_stack_handler(
    table: *const ctl_table, write: i32, buffer: *mut core::ffi::c_void,
    lenp: *mut usize, ppos: *mut loff_t,
) -> i32 {
    let value = (*table).data as *mut i32;
    let mut new_value = *value;
    let mut new_table = *table;
    new_table.data = &mut new_value as *mut i32 as *mut core::ffi::c_void;
    let mut ret = proc_dointvec_minmax(&mut new_table, write, buffer, lenp, ppos);
    if ret != 0 || write == 0 { return ret; }
    mutex_lock!(&mut callchain_mutex);
    if atomic_read!(&mut nr_callchain_events) != 0 { ret = -EBUSY; } else { *value = new_value; }
    mutex_unlock!(&mut callchain_mutex);
    ret
}

static callchain_sysctl_table: [ctl_table; 3] = [
    ctl_table { procname: "perf_event_max_stack", data: unsafe { &sysctl_perf_event_max_stack as *const _ as *mut _ }, maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(perf_event_max_stack_handler), extra1: SYSCTL_ZERO, extra2: &six_hundred_forty_kb as *const _ as *mut _ },
    ctl_table { procname: "perf_event_max_contexts_per_stack", data: unsafe { &sysctl_perf_event_max_contexts_per_stack as *const _ as *mut _ }, maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(perf_event_max_stack_handler), extra1: SYSCTL_ZERO, extra2: SYSCTL_ONE_THOUSAND },
    ctl_table { ..unsafe { core::mem::zeroed() } },
];

unsafe fn init_callchain_sysctls() -> i32 {
    register_sysctl_init!("kernel", callchain_sysctl_table.as_ptr());
    0
}

core_initcall!(init_callchain_sysctls);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
