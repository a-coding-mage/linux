// SPDX-License-Identifier: GPL-2.0
/*
 * Alpha specific irq code.
 */

// C header dependencies are supplied by the surrounding kernel translation.

#[cfg(CONFIG_ALPHA_BROKEN_IRQ_MASK)]
#[no_mangle]
pub static mut __min_ipl: ::core::ffi::c_int = 0;

unsafe fn dummy_perf(vector: ::core::ffi::c_ulong, regs: *mut pt_regs) {
    let _ = vector;
    let _ = regs;
    irq_err_count += 1;
    printk(KERN_CRIT, "Performance counter interrupt!\n");
}

#[no_mangle]
pub static mut perf_irq: unsafe fn(::core::ffi::c_ulong, *mut pt_regs) = dummy_perf;

#[no_mangle]
pub unsafe extern "C" fn do_entInt(
    type_: ::core::ffi::c_ulong,
    vector: ::core::ffi::c_ulong,
    la_ptr: ::core::ffi::c_ulong,
    regs: *mut pt_regs,
) {
    let old_regs: *mut pt_regs;

    raw_local_irq_disable();
    if lockdep_hardirqs_enabled() {
        trace_hardirqs_off();
    }

    old_regs = set_irq_regs(regs);

    match type_ {
        0 => {
            #[cfg(CONFIG_SMP)]
            {
                irq_enter();
                handle_ipi(regs);
                irq_exit();
            }
            #[cfg(not(CONFIG_SMP))]
            {
                irq_err_count += 1;
                pr_crit("Interprocessor interrupt? You must be kidding!\n");
            }
        }
        1 => {
            handle_irq(RTC_IRQ);
        }
        2 => {
            irq_enter();
            alpha_mv.machine_check(vector, la_ptr);
            irq_exit();
        }
        3 => {
            irq_enter();
            alpha_mv.device_interrupt(vector);
            irq_exit();
        }
        4 => {
            irq_enter();
            perf_irq(la_ptr, regs);
            irq_exit();
        }
        _ => {
            pr_crit("Hardware intr %lu %lx? Huh?\n", type_, vector);
            pr_crit("PC = %016lx PS=%04lx\n", (*regs).pc, (*regs).ps);
        }
    }

    set_irq_regs(old_regs);
}

#[no_mangle]
pub unsafe extern "C" fn lockdep_on_restore(ps: ::core::ffi::c_ulong, ip: ::core::ffi::c_ulong) {
    #[cfg(CONFIG_PROVE_LOCKING)]
    {
        if (ps & 7) == 7 {
            return;
        }
        if !irqs_disabled() {
            return;
        }
        if lockdep_hardirqs_enabled() {
            return;
        }
        lockdep_hardirqs_on_prepare();
        lockdep_hardirqs_on(ip);
    }
}

#[no_mangle]
pub unsafe extern "C" fn common_init_isa_dma() {
    outb(0, DMA1_RESET_REG);
    outb(0, DMA2_RESET_REG);
    outb(0, DMA1_CLR_MASK_REG);
    outb(0, DMA2_CLR_MASK_REG);
}

#[no_mangle]
pub unsafe extern "C" fn init_IRQ() {
    wrent(entInt, 0);
    alpha_mv.init_irq();
}

pub const MCHK_K_TPERR: ::core::ffi::c_int = 0x0080;
pub const MCHK_K_TCPERR: ::core::ffi::c_int = 0x0082;
pub const MCHK_K_HERR: ::core::ffi::c_int = 0x0084;
pub const MCHK_K_ECC_C: ::core::ffi::c_int = 0x0086;
pub const MCHK_K_ECC_NC: ::core::ffi::c_int = 0x0088;
pub const MCHK_K_OS_BUGCHECK: ::core::ffi::c_int = 0x008A;
pub const MCHK_K_PAL_BUGCHECK: ::core::ffi::c_int = 0x0090;

#[cfg(not(CONFIG_SMP))]
pub static mut __mcheck_info: mcheck_info = mcheck_info::zeroed();

#[no_mangle]
pub unsafe extern "C" fn process_mcheck_info(
    vector: ::core::ffi::c_ulong,
    la_ptr: ::core::ffi::c_ulong,
    machine: *const ::core::ffi::c_char,
    expected: ::core::ffi::c_int,
) {
    let mchk_header = la_ptr as *mut el_common;
    let reason: *const ::core::ffi::c_char;

    #[cfg(CONFIG_VERBOSE_MCHECK)]
    if alpha_verbose_mcheck > 1 {
        printk(KERN_CRIT, "%s machine check %s\n", machine,
               if expected != 0 { "expected." } else { "NOT expected!!!" });
    }

    if expected != 0 {
        let cpu = smp_processor_id();
        *mcheck_expected(cpu) = 0;
        *mcheck_taken(cpu) = 1;
        return;
    }

    printk(KERN_CRIT, "%s machine check: vector=0x%lx pc=0x%lx code=0x%x\n",
           machine, vector, (*get_irq_regs()).pc, (*mchk_header).code);

    reason = match (*mchk_header).code {
        0x80 => "tag parity error\0",
        0x82 => "tag control parity error\0",
        0x84 => "generic hard error\0",
        0x86 => "correctable ECC error\0",
        0x88 => "uncorrectable ECC error\0",
        0x8A => "OS-specific PAL bugcheck\0",
        0x90 => "callsys in kernel mode\0",
        0x96 => "i-cache read retryable error\0",
        0x98 => "processor detected hard error\0",
        0x202 => "system detected hard error\0",
        0x203 => "system detected uncorrectable ECC error\0",
        0x204 => "SIO SERR occurred on PCI bus\0",
        0x205 => "parity error detected by core logic\0",
        0x206 => "SIO IOCHK occurred on ISA bus\0",
        0x207 => "non-existent memory error\0",
        0x208 => "MCHK_K_DCSR\0",
        0x209 => "PCI SERR detected\0",
        0x20b => "PCI data parity error detected\0",
        0x20d => "PCI address parity error detected\0",
        0x20f => "PCI master abort error\0",
        0x211 => "PCI target abort error\0",
        0x213 => "scatter/gather PTE invalid error\0",
        0x215 => "flash ROM write error\0",
        0x217 => "IOA timeout detected\0",
        0x219 => "IOCHK#, EISA add-in board parity or other catastrophic error\0",
        0x21b => "EISA fail-safe timer timeout\0",
        0x21d => "EISA bus time-out\0",
        0x21f => "EISA software generated NMI\0",
        0x221 => "unexpected ev5 IRQ[3] interrupt\0",
        _ => "unknown\0",
    }.as_ptr() as *const ::core::ffi::c_char;

    printk(KERN_CRIT, "machine check type: %s%s\n", reason,
           if (*mchk_header).retry != 0 { " (retryable)" } else { "" });
    dik_show_regs(get_irq_regs(), core::ptr::null_mut());

    #[cfg(CONFIG_VERBOSE_MCHECK)]
    if alpha_verbose_mcheck > 1 {
        let ptr = la_ptr as *const ::core::ffi::c_ulong;
        let mut i = 0;
        while i < (*mchk_header).size / core::mem::size_of::<::core::ffi::c_long>() {
            printk(KERN_CRIT, "   +%8lx %016lx %016lx\n", i * core::mem::size_of::<::core::ffi::c_long>(), *ptr.add(i), *ptr.add(i + 1));
            i += 2;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn init_rtc_irq(mut handler: irq_handler_t) {
    irq_set_chip_and_handler_name(RTC_IRQ, &dummy_irq_chip, handle_percpu_irq, "RTC");
    if handler.is_none() {
        handler = Some(rtc_timer_interrupt);
    }
    if request_irq(RTC_IRQ, handler, 0, "timer", core::ptr::null_mut()) != 0 {
        pr_err("Failed to register timer interrupt\n");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
