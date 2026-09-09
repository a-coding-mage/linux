// SPDX-License-Identifier: GPL-2.0
/*
 *    Copyright IBM Corp. 2004, 2011
 *    Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>,
 *             Holger Smolinski <Holger.Smolinski@de.ibm.com>,
 *             Thomas Spatzier <tspat@de.ibm.com>,
 *
 * This file contains interrupt related functions.
 */

#[repr(C)]
struct irq_class {
    irq: ::core::ffi::c_int,
    name: *mut ::core::ffi::c_char,
    desc: *mut ::core::ffi::c_char,
}

// The following declarations are supplied by the surrounding kernel translation.
extern "C" {
    static mut irq_stat: irq_stat;
    static irqclass_main_desc: [irq_class; NR_IRQS_BASE as usize];
    static irqclass_sub_desc: [irq_class; NR_ARCH_IRQS as usize];
    static mut ext_int_hash: [hlist_head; 32];
    static mut ext_int_hash_lock: spinlock_t;
    static mut irq_subclass_lock: spinlock_t;
    static mut irq_subclass_refcount: [u8; 64];
}

#[repr(C)]
struct irq_class_placeholder;

// Main interrupt classes shown by /proc/stat and /proc/interrupts.
// The irqclass_* initializers are retained as declarations because their
// architecture constants and C string representation are supplied externally.

unsafe fn show_msi_interrupt(p: *mut seq_file, irq: ::core::ffi::c_int) {
    rcu_read_lock();
    let desc = irq_to_desc(irq);
    if desc.is_null() { rcu_read_unlock(); return; }
    let mut flags: ::core::ffi::c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*desc).lock, &mut flags);
    seq_printf(p, c"%3d: ".as_ptr(), irq);
    let mut cpu = 0;
    for_each_online_cpu!(cpu) { seq_printf(p, c"%10u ".as_ptr(), irq_desc_kstat_cpu(desc, cpu)); }
    if !(*desc).irq_data.chip.is_null() { seq_printf(p, c" %8s".as_ptr(), (*(*desc).irq_data.chip).name); }
    if !(*desc).action.is_null() { seq_printf(p, c"  %s".as_ptr(), (*(*desc).action).name); }
    seq_putc(p, b'\n' as _);
    raw_spin_unlock_irqrestore(&mut (*desc).lock, flags);
    rcu_read_unlock();
}

pub unsafe fn show_interrupts(p: *mut seq_file, v: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut index = *(v as *mut i64) as ::core::ffi::c_int;
    let mut cpu = 0;
    cpus_read_lock();
    if index == 0 {
        seq_puts(p, c"           ".as_ptr());
        for_each_online_cpu!(cpu) { seq_printf(p, c"CPU%-8d".as_ptr(), cpu); }
        seq_putc(p, b'\n' as _);
    }
    if index < NR_IRQS_BASE {
        seq_printf(p, c"%s: ".as_ptr(), irqclass_main_desc[index as usize].name);
        let irq = irqclass_main_desc[index as usize].irq;
        for_each_online_cpu!(cpu) { seq_printf(p, c"%10u ".as_ptr(), kstat_irqs_cpu(irq, cpu)); }
        seq_putc(p, b'\n' as _);
    } else if index < irq_get_nr_irqs() {
        show_msi_interrupt(p, index);
    } else {
        index = 0;
        while index < NR_ARCH_IRQS {
            seq_printf(p, c"%s: ".as_ptr(), irqclass_sub_desc[index as usize].name);
            let irq = irqclass_sub_desc[index as usize].irq;
            for_each_online_cpu!(cpu) { seq_printf(p, c"%10u ".as_ptr(), per_cpu_irq_stat(cpu).irqs[irq as usize]); }
            if !irqclass_sub_desc[index as usize].desc.is_null() { seq_printf(p, c"  %s".as_ptr(), irqclass_sub_desc[index as usize].desc); }
            seq_putc(p, b'\n' as _);
            index += 1;
        }
    }
    cpus_read_unlock();
    0
}

#[repr(C)]
struct ext_int_info {
    handler: ext_int_handler_t,
    entry: hlist_node,
    rcu: rcu_head,
    code: u16,
}

unsafe fn do_IRQ(regs: *mut pt_regs, irq: ::core::ffi::c_int) {
    let lc = get_lowcore();
    if tod_after_eq((*lc).int_clock.tod, (*lc).clock_comparator) {
        clock_comparator_work();
    }
    generic_handle_irq(irq);
}

unsafe fn on_async_stack() -> ::core::ffi::c_int {
    let frame = current_frame_address();
    ((((*get_lowcore()).async_stack ^ frame) & !(THREAD_SIZE - 1)) == 0) as ::core::ffi::c_int
}

unsafe fn do_irq_async(regs: *mut pt_regs, irq: ::core::ffi::c_int) {
    if on_async_stack() != 0 {
        do_IRQ(regs, irq);
    } else {
        call_on_stack(2, (*get_lowcore()).async_stack, do_IRQ, regs, irq);
    }
}

unsafe fn irq_pending(_regs: *mut pt_regs) -> ::core::ffi::c_int {
    let mut cc: ::core::ffi::c_int = 0;
    // C condition-code assembly (tpi 0) is provided by the target architecture.
    asm!("tpi 0", out("cc") cc, options(nostack));
    CC_TRANSFORM(cc)
}

pub unsafe fn do_io_irq(regs: *mut pt_regs) {
    let mut from_idle: bool;
    let mut percpu_needs_fixup: bool;
    let old_regs: *mut pt_regs;
    let state: irqentry_state_t;

    percpu_entry(regs);
    state = irqentry_enter(regs);
    old_regs = set_irq_regs(regs);
    from_idle = test_and_clear_cpu_flag(CIF_ENABLED_WAIT);
    if from_idle { update_timer_idle(); }
    irq_enter_rcu();
    if user_mode(regs) {
        update_timer_sys();
        if cpu_has_bear() { (*current).thread.last_break = (*regs).last_break; }
    }
    if from_idle { account_idle_time_irq(); }
    loop {
        (*regs).tpi_info = (*get_lowcore()).tpi_info;
        if (*get_lowcore()).tpi_info.adapter_IO {
            do_irq_async(regs, THIN_INTERRUPT);
        } else {
            do_irq_async(regs, IO_INTERRUPT);
        }
        if !(machine_is_lpar() && irq_pending(regs) != 0) { break; }
    }
    percpu_needs_fixup = percpu_code_check(regs);
    irq_exit_rcu();
    set_irq_regs(old_regs);
    irqentry_exit(regs, state);
    if from_idle { (*regs).psw.mask &= !(PSW_MASK_EXT | PSW_MASK_IO | PSW_MASK_WAIT); }
    percpu_exit(regs, percpu_needs_fixup);
}

pub unsafe fn do_ext_irq(regs: *mut pt_regs) {
    let from_idle: bool;
    let percpu_needs_fixup: bool;
    let old_regs: *mut pt_regs;
    let state: irqentry_state_t;
    percpu_entry(regs);
    state = irqentry_enter(regs);
    old_regs = set_irq_regs(regs);
    from_idle = test_and_clear_cpu_flag(CIF_ENABLED_WAIT);
    if from_idle { update_timer_idle(); }
    irq_enter_rcu();
    if user_mode(regs) {
        update_timer_sys();
        if cpu_has_bear() { (*current).thread.last_break = (*regs).last_break; }
    }
    (*regs).int_code = (*get_lowcore()).ext_int_code_addr;
    (*regs).int_parm = (*get_lowcore()).ext_params;
    (*regs).int_parm_long = (*get_lowcore()).ext_params2;
    if from_idle { account_idle_time_irq(); }
    do_irq_async(regs, EXT_INTERRUPT);
    percpu_needs_fixup = percpu_code_check(regs);
    irq_exit_rcu();
    set_irq_regs(old_regs);
    irqentry_exit(regs, state);
    if from_idle { (*regs).psw.mask &= !(PSW_MASK_EXT | PSW_MASK_IO | PSW_MASK_WAIT); }
    percpu_exit(regs, percpu_needs_fixup);
}

pub unsafe fn arch_dynirq_lower_bound(from: u32) -> u32 {
    if from < NR_IRQS_BASE { NR_IRQS_BASE } else { from }
}

unsafe fn ext_hash(code: u16) -> ::core::ffi::c_int {
    ((code as u32 + ((code as u32) >> 9)) & 31) as ::core::ffi::c_int
}

pub unsafe fn register_external_irq(code: u16, handler: ext_int_handler_t) -> ::core::ffi::c_int {
    let p = kmalloc_obj::<ext_int_info>(GFP_ATOMIC);
    if p.is_null() { return -ENOMEM; }
    (*p).code = code;
    (*p).handler = handler;
    let index = ext_hash(code) as usize;
    spin_lock_irqsave(&mut ext_int_hash_lock);
    hlist_add_head_rcu(&mut (*p).entry, &mut ext_int_hash[index]);
    spin_unlock_irqrestore(&mut ext_int_hash_lock);
    0
}

pub unsafe fn unregister_external_irq(code: u16, handler: ext_int_handler_t) -> ::core::ffi::c_int {
    let index = ext_hash(code) as usize;
    spin_lock_irqsave(&mut ext_int_hash_lock);
    let mut p: *mut ext_int_info = core::ptr::null_mut();
    hlist_for_each_entry_rcu(p, &mut ext_int_hash[index], entry) {
        if (*p).code == code && (*p).handler == handler {
            hlist_del_rcu(&mut (*p).entry);
            kfree_rcu(p, rcu);
        }
    }
    spin_unlock_irqrestore(&mut ext_int_hash_lock);
    0
}

unsafe fn do_ext_interrupt(_irq: ::core::ffi::c_int, _dummy: *mut ::core::ffi::c_void) -> irqreturn_t {
    let regs = get_irq_regs();
    let mut ext_code: ext_code = core::mem::zeroed();
    (*ext_code).int_code = (*regs).int_code;
    let index = ext_hash((*ext_code).code) as usize;
    rcu_read_lock();
    let mut p: *mut ext_int_info = core::ptr::null_mut();
    hlist_for_each_entry_rcu(p, &mut ext_int_hash[index], entry) {
        if (*p).code != (*ext_code).code { continue; }
        ((*p).handler)((*ext_code), (*regs).int_parm, (*regs).int_parm_long);
    }
    rcu_read_unlock();
    IRQ_HANDLED
}

unsafe fn init_ext_interrupts() {
    for idx in 0..32 { INIT_HLIST_HEAD(&mut ext_int_hash[idx]); }
    irq_set_chip_and_handler(EXT_INTERRUPT, &dummy_irq_chip, handle_percpu_irq);
    if request_irq(EXT_INTERRUPT, do_ext_interrupt, 0, c"EXT".as_ptr(), core::ptr::null_mut()) != 0 {
        panic!("Failed to register EXT interrupt\n");
    }
}

pub unsafe fn init_IRQ() {
    init_cio_interrupts();
    init_airq_interrupts();
    init_ext_interrupts();
}

pub unsafe fn irq_subclass_register(subclass: irq_subclass) {
    spin_lock(&mut irq_subclass_lock);
    let i = subclass as usize;
    if irq_subclass_refcount[i] == 0 { system_ctl_set_bit(0, subclass); }
    irq_subclass_refcount[i] = irq_subclass_refcount[i].wrapping_add(1);
    spin_unlock(&mut irq_subclass_lock);
}

pub unsafe fn irq_subclass_unregister(subclass: irq_subclass) {
    spin_lock(&mut irq_subclass_lock);
    let i = subclass as usize;
    irq_subclass_refcount[i] = irq_subclass_refcount[i].wrapping_sub(1);
    if irq_subclass_refcount[i] == 0 { system_ctl_clear_bit(0, subclass); }
    spin_unlock(&mut irq_subclass_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
