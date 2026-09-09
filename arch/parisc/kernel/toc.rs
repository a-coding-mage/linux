// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the Linux kernel and PA-RISC headers are intentionally
// referenced as external Rust items.

#[repr(align(16))]
struct TocLockAlign(u32);

static mut toc_lock: TocLockAlign = TocLockAlign(1);
// DEFINE_PER_CPU_PAGE_ALIGNED(char [16384], toc_stack) __visible;
static mut toc_stack: [u8; 16384] = [0; 16384];

extern "C" {
    static mut boot_cpu_data: BootCpuData;
    static mut PAGE0: *mut PageZero;
    static toc_handler: *const core::ffi::c_void;
    static toc_handler_size: u32;
    static mut toc_handler_csum: u32;
    static mut kgdb_active: AtomicInt;

    fn pdc_pim_toc20(data: *mut PdcTocPim20) -> i32;
    fn pdc_pim_toc11(data: *mut PdcTocPim11) -> i32;
    fn panic(message: *const core::ffi::c_char) -> !;
    fn nmi_enter();
    fn atomic_read(value: *const AtomicInt) -> i32;
    fn kgdb_nmicallback(cpu: i32, regs: *mut PtRegs);
    fn kgdb_handle_exception(signo: i32, code: i32, err: i32, regs: *mut PtRegs);
    fn __ldcw(value: *mut u32) -> u32;
    fn show_regs(regs: *mut PtRegs);
    fn raw_smp_processor_id() -> i32;
    fn mdelay(milliseconds: u32);
    fn machine_restart(command: *const core::ffi::c_char) -> !;
    fn dereference_function_descriptor(function: *const core::ffi::c_void) -> *const core::ffi::c_void;
    fn __pa(address: usize) -> usize;
    fn pr_info(message: *const core::ffi::c_char);
}

unsafe fn toc20_to_pt_regs(regs: *mut PtRegs, toc: *mut PdcTocPim20) {
    (*regs).gr[0] = (*toc).cr[22] as usize;

    for i in 1..32 {
        (*regs).gr[i] = (*toc).gr[i] as usize;
    }

    for i in 0..8 {
        (*regs).sr[i] = (*toc).sr[i] as usize;
    }

    (*regs).iasq[0] = (*toc).cr[17] as usize;
    (*regs).iasq[1] = (*toc).iasq_back as usize;
    (*regs).iaoq[0] = (*toc).cr[18] as usize;
    (*regs).iaoq[1] = (*toc).iaoq_back as usize;

    (*regs).sar = (*toc).cr[11] as usize;
    (*regs).iir = (*toc).cr[19] as usize;
    (*regs).isr = (*toc).cr[20] as usize;
    (*regs).ior = (*toc).cr[21] as usize;
}

unsafe fn toc11_to_pt_regs(regs: *mut PtRegs, toc: *mut PdcTocPim11) {
    (*regs).gr[0] = (*toc).cr[22];

    for i in 1..32 {
        (*regs).gr[i] = (*toc).gr[i];
    }

    for i in 0..8 {
        (*regs).sr[i] = (*toc).sr[i];
    }

    (*regs).iasq[0] = (*toc).cr[17];
    (*regs).iasq[1] = (*toc).iasq_back;
    (*regs).iaoq[0] = (*toc).cr[18];
    (*regs).iaoq[1] = (*toc).iaoq_back;

    (*regs).sar = (*toc).cr[11];
    (*regs).iir = (*toc).cr[19];
    (*regs).isr = (*toc).cr[20];
    (*regs).ior = (*toc).cr[21];
}

pub unsafe fn toc_intr(regs: *mut PtRegs) -> ! {
    // verify we wrote regs to the correct stack
    // BUG_ON(regs != (struct pt_regs *)&per_cpu(toc_stack, raw_smp_processor_id()));

    if boot_cpu_data.cpu_type >= pcxu {
        let mut pim_data20: PdcTocPim20 = core::mem::zeroed();
        if pdc_pim_toc20(&mut pim_data20) != 0 {
            panic(b"Failed to get PIM data\0".as_ptr() as *const core::ffi::c_char);
        }
        toc20_to_pt_regs(regs, &mut pim_data20);
    } else {
        let mut pim_data11: PdcTocPim11 = core::mem::zeroed();
        if pdc_pim_toc11(&mut pim_data11) != 0 {
            panic(b"Failed to get PIM data\0".as_ptr() as *const core::ffi::c_char);
        }
        toc11_to_pt_regs(regs, &mut pim_data11);
    }

    // CONFIG_KGDB conditional code is preserved here for the kernel build configuration.
    #[cfg(CONFIG_KGDB)]
    {
        nmi_enter();
        if atomic_read(&kgdb_active) != -1 {
            kgdb_nmicallback(raw_smp_processor_id(), regs);
        }
        kgdb_handle_exception(9, SIGTRAP, 0, regs);
    }

    // serialize output, otherwise all CPUs write backtrace at once
    while __ldcw(&mut toc_lock.0) == 0 {
        // wait
    }
    show_regs(regs);
    toc_lock.0 = 1; // release lock for next CPU

    if raw_smp_processor_id() != 0 {
        loop {
            // all but monarch CPU will wait endless.
        }
    }

    // give other CPUs time to show their backtrace
    mdelay(2000);

    machine_restart(b"TOC\0".as_ptr() as *const core::ffi::c_char);

    // should never reach this
    panic(b"TOC\0".as_ptr() as *const core::ffi::c_char);
}

unsafe fn setup_toc() -> i32 {
    let mut csum: u32 = 0;
    let toc_code = dereference_function_descriptor(toc_handler) as usize;

    (*PAGE0).vec_toc = (__pa(toc_code) & 0xffff_ffff) as u32;
    // CONFIG_64BIT conditional field assignment.
    #[cfg(CONFIG_64BIT)]
    {
        (*PAGE0).vec_toc_hi = (__pa(toc_code) >> 32) as u32;
    }
    (*PAGE0).vec_toclen = toc_handler_size;

    for i in 0..(toc_handler_size / 4) {
        csum = csum.wrapping_add(*((toc_code as *const u32).add(i as usize)));
    }
    toc_handler_csum = csum.wrapping_neg();
    pr_info(b"TOC handler registered\n\0".as_ptr() as *const core::ffi::c_char);
    0
}

// early_initcall(setup_toc);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
