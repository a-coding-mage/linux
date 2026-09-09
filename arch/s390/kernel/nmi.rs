// SPDX-License-Identifier: GPL-2.0
/* Machine check handler. Direct translation of nmi.c. */

#[repr(C)]
struct McckStruct {
    kill_task: bool,
    channel_report: bool,
    warning: bool,
    stp_queue: bool,
    mcck_code: usize,
}

static mut CPU_MCCK: McckStruct = McckStruct { kill_task: false, channel_report: false, warning: false, stp_queue: false, mcck_code: 0 };
static mut BOOT_MCESA: mcesa = mcesa { _opaque: 0 };

const MAX_IPD_COUNT: i32 = 29;
const MAX_IPD_TIME: u64 = 5 * 60 * USEC_PER_SEC;
const ED_STP_ISLAND: u32 = 6;
const ED_STP_SYNC: u32 = 7;

#[inline]
unsafe fn nmi_needs_mcesa() -> i32 { (cpu_has_vx() || cpu_has_gs()) as i32 }

unsafe fn nmi_alloc_mcesa_early(mcesad: *mut u64) {
    if nmi_needs_mcesa() == 0 { return; }
    *mcesad = __pa(&raw const BOOT_MCESA);
    if cpu_has_gs() { *mcesad |= ilog2(MCESA_MAX_SIZE); }
}

unsafe fn nmi_alloc_mcesa(mcesad: *mut u64) -> i32 {
    *mcesad = 0;
    if nmi_needs_mcesa() == 0 { return 0; }
    let size = if cpu_has_gs() { MCESA_MAX_SIZE } else { MCESA_MIN_SIZE };
    let origin = kmalloc(size, GFP_KERNEL);
    if origin.is_null() { return -ENOMEM; }
    // The pointer is stored with mcesa_bits ORed in.
    kmemleak_not_leak(origin);
    *mcesad = __pa(origin);
    if cpu_has_gs() { *mcesad |= ilog2(MCESA_MAX_SIZE); }
    0
}

unsafe fn nmi_free_mcesa(mcesad: *mut u64) {
    if nmi_needs_mcesa() == 0 { return; }
    kfree(__va(*mcesad & MCESA_ORIGIN_MASK));
}

#[inline(always)]
unsafe fn nmi_puts(mut dest: *mut u8, mut src: *const u8) -> *mut u8 {
    while *src != 0 { *dest = *src; dest = dest.add(1); src = src.add(1); }
    *dest = 0; dest
}

#[inline(always)]
unsafe fn u64_to_hex(mut dest: *mut u8, val: u64) -> *mut u8 {
    for i in 1..=16 {
        let num = ((val >> (64 - 4 * i)) & 0xf) as u8;
        *dest = if num >= 10 { b'A' + num - 10 } else { b'0' + num };
        dest = dest.add(1);
    }
    *dest = 0; dest
}

unsafe fn nmi_print_info() {
    let lc = get_lowcore();
    let mut message = [0u8; 100];
    let mut ptr;
    ptr = nmi_puts(message.as_mut_ptr(), b"Unrecoverable machine check, code: \0".as_ptr());
    ptr = u64_to_hex(ptr, (*lc).mcck_interruption_code);
    ptr = nmi_puts(ptr, b"\n\0".as_ptr()); sclp_emergency_printk(message.as_ptr());
    ptr = nmi_puts(message.as_mut_ptr(), init_utsname().release); ptr = nmi_puts(ptr, b"\n\0".as_ptr()); sclp_emergency_printk(message.as_ptr());
    ptr = nmi_puts(message.as_mut_ptr(), arch_hw_string); ptr = nmi_puts(ptr, b"\n\0".as_ptr()); sclp_emergency_printk(message.as_ptr());
    ptr = nmi_puts(message.as_mut_ptr(), b"PSW: \0".as_ptr());
    ptr = u64_to_hex(ptr, (*lc).mcck_old_psw.mask); ptr = nmi_puts(ptr, b" \0".as_ptr());
    ptr = u64_to_hex(ptr, (*lc).mcck_old_psw.addr); ptr = nmi_puts(ptr, b" PFX: \0".as_ptr());
    ptr = u64_to_hex(ptr, get_lowcore() as u64); ptr = nmi_puts(ptr, b"\n\0".as_ptr()); sclp_emergency_printk(message.as_ptr());
    ptr = nmi_puts(message.as_mut_ptr(), b"LBA: \0".as_ptr()); ptr = u64_to_hex(ptr, (*lc).last_break_save_area);
    ptr = nmi_puts(ptr, b" EDC: \0".as_ptr()); ptr = u64_to_hex(ptr, (*lc).external_damage_code as u64);
    ptr = nmi_puts(ptr, b" FSA: \0".as_ptr()); ptr = u64_to_hex(ptr, (*lc).failing_storage_address); ptr = nmi_puts(ptr, b"\n\0".as_ptr()); sclp_emergency_printk(message.as_ptr());
    ptr = nmi_puts(message.as_mut_ptr(), b"CRS:\n\0".as_ptr()); sclp_emergency_printk(message.as_ptr()); ptr = message.as_mut_ptr();
    for i in 0..16 { ptr = u64_to_hex(ptr, (*lc).cregs_save_area[i].val); ptr = nmi_puts(ptr, b" \0".as_ptr()); if (i + 1) % 4 == 0 { ptr = nmi_puts(ptr, b"\n\0".as_ptr()); sclp_emergency_printk(message.as_ptr()); ptr = message.as_mut_ptr(); } }
    ptr = nmi_puts(message.as_mut_ptr(), b"GPRS:\n\0".as_ptr()); sclp_emergency_printk(message.as_ptr()); ptr = message.as_mut_ptr();
    for i in 0..16 { ptr = u64_to_hex(ptr, (*lc).gpregs_save_area[i]); ptr = nmi_puts(ptr, b" \0".as_ptr()); if (i + 1) % 4 == 0 { ptr = nmi_puts(ptr, b"\n\0".as_ptr()); sclp_emergency_printk(message.as_ptr()); ptr = message.as_mut_ptr(); } }
    ptr = nmi_puts(message.as_mut_ptr(), b"System stopped\n\0".as_ptr()); sclp_emergency_printk(message.as_ptr());
}

unsafe fn s390_handle_damage() -> ! {
    let lc = get_lowcore(); let mut cr0: ctlreg0; let mut cr0_new: ctlreg0; let psw_save;
    smp_emergency_stop(); diag_amode31_ops.diag308_reset();
    local_ctl_store(0, &mut cr0.reg); cr0_new = cr0; cr0_new.lap = 0; local_ctl_load(0, &cr0_new.reg);
    psw_save = (*lc).mcck_new_psw; psw_bits(&mut (*lc).mcck_new_psw).io = 0; psw_bits(&mut (*lc).mcck_new_psw).ext = 0; psw_bits(&mut (*lc).mcck_new_psw).wait = 1;
    nmi_print_info(); (*lc).mcck_new_psw = psw_save; local_ctl_load(0, &cr0.reg); disabled_wait()
}

unsafe fn nmi_registers_valid(mci: mci) -> bool {
    set_tod_programmable_field(raw_smp_processor_id()); set_clock_comparator((*get_lowcore()).clock_comparator);
    if !mci.gr || !mci.fp || !mci.fc { return false; }
    if !mci.vr && !test_cpu_flag(CIF_MCCK_GUEST) { return false; }
    if !mci.ar { return false; }
    let cr2 = (*get_lowcore()).cregs_save_area[2];
    if cr2.gse && !mci.gs && !test_cpu_flag(CIF_MCCK_GUEST) { return false; }
    mci.ms && mci.pm && mci.ia
}

unsafe fn s390_backup_mcck_info(regs: *mut pt_regs) {
    let sie_block = phys_to_virt((*regs).gprs[14]);
    if sie_block.is_null() { s390_handle_damage(); }
    let sie_page = container_of(sie_block, sie_page, sie_block);
    (*sie_page).mcck_info.mcic = (*get_lowcore()).mcck_interruption_code & !MCCK_CODE_NO_GUEST;
    (*sie_page).mcck_info.ext_damage_code = (*get_lowcore()).external_damage_code;
    (*sie_page).mcck_info.failing_storage_address = (*get_lowcore()).failing_storage_address;
}

unsafe fn s390_do_machine_check(regs: *mut pt_regs) {
    let mut percpu_needs_fixup; let mut ipd_count: i32 = 0; let mut last_ipd: u64 = 0;
    let lc = get_lowcore(); let mcck = &mut CPU_MCCK; let mut mcck_pending = false;
    percpu_entry(regs); let irq_state = irqentry_nmi_enter(regs);
    if user_mode(regs) { update_timer_mcck(); } inc_irq_stat(NMI_NMI);
    let mci = mci { val: (*lc).mcck_interruption_code };
    if mci.pd && !test_cpu_flag(CIF_MCCK_GUEST) { if mci.b {
        let z_mcic = (1u64<<63) | (1u64<<59) | (1u64<<29); let o_mcic = (1u64<<43)|(1u64<<42)|(1u64<<41)|(1u64<<40)|(1u64<<36)|(1u64<<35)|(1u64<<34)|(1u64<<32)|(1u64<<30)|(1u64<<21)|(1u64<<20)|(1u64<<17)|(1u64<<16);
        if (mci.val & z_mcic) != 0 || (mci.val & o_mcic) != o_mcic { s390_handle_damage(); }
        spin_lock(&IPD_LOCK); let tmp = get_tod_clock(); if ((tmp - last_ipd) >> 12) < MAX_IPD_TIME { ipd_count += 1; } else { ipd_count = 1; } last_ipd = tmp; if ipd_count == MAX_IPD_COUNT { s390_handle_damage(); } spin_unlock(&IPD_LOCK);
    } else { s390_handle_damage(); } }
    if !nmi_registers_valid(mci) { if !user_mode(regs) { s390_handle_damage(); } mcck.kill_task = true; mcck.mcck_code = mci.val as usize; mcck_pending = true; }
    if test_cpu_flag(CIF_MCCK_GUEST) { s390_backup_mcck_info(regs); }
    if mci.cd { s390_handle_damage(); }
    if mci.ed && mci.ec { if (*lc).external_damage_code & (1u32<<ED_STP_SYNC) != 0 { mcck.stp_queue |= stp_sync_check(); } if (*lc).external_damage_code & (1u32<<ED_STP_ISLAND) != 0 { mcck.stp_queue |= stp_island_check(); } mcck_pending = true; }
    if !test_cpu_flag(CIF_MCCK_GUEST) { if mci.se || mci.ke || (mci.ds && mci.fa) { s390_handle_damage(); } }
    if mci.cp { mcck.channel_report = true; mcck_pending = true; } if mci.w { mcck.warning = true; mcck_pending = true; }
    let mcck_dam_code = mci.val & MCIC_SUBCLASS_MASK; if test_cpu_flag(CIF_MCCK_GUEST) && (mcck_dam_code & MCCK_CODE_NO_GUEST) != mcck_dam_code { (*(regs as *mut stack_frame)).sie_return = SIE64_RETURN_MCCK; }
    clear_cpu_flag(CIF_MCCK_GUEST); if mcck_pending { schedule_mcck_handler(); }
    percpu_needs_fixup = percpu_code_check(regs); irqentry_nmi_exit(regs, irq_state); percpu_exit(regs, percpu_needs_fixup);
}

unsafe fn s390_handle_mcck() {
    let mut mflags: usize = 0;
    local_mcck_save(&mut mflags);
    let mcck = CPU_MCCK;
    CPU_MCCK = McckStruct { kill_task: false, channel_report: false, warning: false, stp_queue: false, mcck_code: 0 };
    local_mcck_restore(mflags);
    if mcck.channel_report { crw_handle_channel_report(); }
    if mcck.warning {
        static mut MCHCHK_WNG_POSTED: i32 = 0;
        local_ctl_clear_bit(14, CR14_WARNING_SUBMASK_BIT);
        if xchg(&mut MCHCHK_WNG_POSTED, 1) == 0 { kill_cad_pid(SIGPWR, 1); }
    }
    if mcck.stp_queue { stp_queue_work(); }
    if mcck.kill_task {
        printk(KERN_EMERG, "mcck: Terminating task because of machine malfunction (code 0x%016lx).\n", mcck.mcck_code);
        printk(KERN_EMERG, "mcck: task: %s, pid: %d.\n", current.comm, current.pid);
        if is_global_init(current) { panic!("mcck: Attempting to kill init!\n"); }
        do_send_sig_info(SIGKILL, SEND_SIG_PRIV, current, PIDTYPE_PID);
    }
}

unsafe fn machine_check_init() -> i32 {
    system_ctl_set_bit(14, CR14_EXTERNAL_DAMAGE_SUBMASK_BIT);
    system_ctl_set_bit(14, CR14_RECOVERY_SUBMASK_BIT);
    system_ctl_set_bit(14, CR14_WARNING_SUBMASK_BIT);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
