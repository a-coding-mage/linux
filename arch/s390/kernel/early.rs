// SPDX-License-Identifier: GPL-2.0
/*
 *    Copyright IBM Corp. 2007, 2009
 *    Author(s): Hongjie Yang <hongjie@us.ibm.com>,
 */

// C header dependencies are supplied by the surrounding kernel translation.

use core::ffi::c_char;

// The decompressor consumes these parameters before the kernel starts; retain
// the early-parameter handlers as declaration-level Rust functions.
unsafe extern "C" fn ignore_decompressor_param_mem(_s: *mut c_char) -> i32 { 0 }
unsafe extern "C" fn ignore_decompressor_param_vmalloc(_s: *mut c_char) -> i32 { 0 }
unsafe extern "C" fn ignore_decompressor_param_dfltcc(_s: *mut c_char) -> i32 { 0 }
unsafe extern "C" fn ignore_decompressor_param_facilities(_s: *mut c_char) -> i32 { 0 }
unsafe extern "C" fn ignore_decompressor_param_nokaslr(_s: *mut c_char) -> i32 { 0 }
unsafe extern "C" fn ignore_decompressor_param_cmma(_s: *mut c_char) -> i32 { 0 }
unsafe extern "C" fn ignore_decompressor_param_relocate_lowcore(_s: *mut c_char) -> i32 { 0 }
unsafe extern "C" fn ignore_decompressor_param_bootdebug(_s: *mut c_char) -> i32 { 0 }
unsafe extern "C" fn ignore_decompressor_param_debug_alternative(_s: *mut c_char) -> i32 { 0 }

// `early_param` registrations for the handlers above are performed by the
// surrounding kernel initialization machinery.

unsafe fn kasan_early_init() {
    // CONFIG_KASAN
    #[cfg(feature = "CONFIG_KASAN")]
    {
        init_task.kasan_depth = 0;
        kasan_init_generic();
    }
}

/* Initialize storage key for kernel pages. */
#[inline(never)]
unsafe fn init_kernel_storage_key() {
    if PAGE_DEFAULT_KEY != 0 {
        let end_pfn: c_ulong = PFN_UP(__pa(_end));
        let mut init_pfn: c_ulong = 0;
        while init_pfn < end_pfn {
            page_set_storage_key(
                init_pfn.wrapping_shl(PAGE_SHIFT),
                PAGE_DEFAULT_KEY,
                0,
            );
            init_pfn = init_pfn.wrapping_add(1);
        }
    }
}

static mut sysinfo_page: [u8; PAGE_SIZE] = [0; PAGE_SIZE];

/* Remove leading, trailing and double whitespace. */
unsafe fn strim_all(str_: *mut c_char) {
    let mut s: *mut c_char;
    s = strim(str_);
    if s != str_ {
        memmove(str_, s, strlen(s));
    }
    while *str_ != 0 {
        if !isspace(*str_) {
            str_ = str_.add(1);
            continue;
        }
        if isspace(*str_.add(1)) {
            s = skip_spaces(str_);
            memmove(str_, s, strlen(s).wrapping_add(1));
        }
        str_ = str_.add(1);
    }
}

pub static mut arch_hw_string: [c_char; 128] = [0; 128];

#[inline(never)]
unsafe fn setup_arch_string() {
    let mach = &mut *(sysinfo_page.as_mut_ptr() as *mut sysinfo_1_1_1);
    let vm = &mut *(sysinfo_page.as_mut_ptr() as *mut sysinfo_3_2_2);
    let mut mstr: [c_char; 80] = [0; 80];
    let mut hvstr: [c_char; 17] = [0; 17];

    if stsi(mach, 1, 1, 1) != 0 { return; }
    EBCASC(mach.manufacturer.as_mut_ptr(), core::mem::size_of_val(&mach.manufacturer));
    EBCASC(mach.type_.as_mut_ptr(), core::mem::size_of_val(&mach.type_));
    EBCASC(mach.model.as_mut_ptr(), core::mem::size_of_val(&mach.model));
    EBCASC(mach.model_capacity.as_mut_ptr(), core::mem::size_of_val(&mach.model_capacity));
    scnprintf(mstr.as_mut_ptr(), mstr.len(), c"%-16.16s %-4.4s %-16.16s %-16.16s", mach.manufacturer.as_ptr(), mach.type_.as_ptr(), mach.model.as_ptr(), mach.model_capacity.as_ptr());
    strim_all(mstr.as_mut_ptr());
    if stsi(vm, 3, 2, 2) == 0 && vm.count != 0 {
        EBCASC(vm.vm[0].cpi.as_mut_ptr(), core::mem::size_of_val(&vm.vm[0].cpi));
        scnprintf(hvstr.as_mut_ptr(), hvstr.len(), c"%-16.16s", vm.vm[0].cpi.as_ptr());
        strim_all(hvstr.as_mut_ptr());
    } else {
        scnprintf(hvstr.as_mut_ptr(), hvstr.len(), c"%s", if machine_is_lpar() { c"LPAR" } else if machine_is_vm() { c"z/VM" } else if machine_is_kvm() { c"KVM" } else { c"unknown" });
    }
    scnprintf(arch_hw_string.as_mut_ptr(), arch_hw_string.len(), c"HW: %s (%s)", mstr.as_ptr(), hvstr.as_ptr());
    dump_stack_set_arch_desc(c"%s (%s)", mstr.as_ptr(), hvstr.as_ptr());
}

unsafe fn setup_topology() {
    if !cpu_has_topology() { return; }
    let mut max_mnest = 6;
    while max_mnest > 1 {
        if stsi(&mut sysinfo_page as *mut _ as *mut _, 15, 1, max_mnest) == 0 { break; }
        max_mnest -= 1;
    }
    topology_max_mnest = max_mnest;
}

pub unsafe fn __do_early_pgm_check(regs: *mut pt_regs) {
    let lc = get_lowcore();
    (*regs).int_code = (*lc).pgm_int_code;
    (*regs).int_parm_long = (*lc).trans_exc_code;
    (*regs).last_break = (*lc).pgm_last_break;
    let ip = __rewind_psw((*regs).psw, (*regs).int_code >> 16);
    if ((*regs).int_code & PGM_INT_CODE_MASK) == 0x40 {
        if report_bug(ip, regs) == BUG_TRAP_TYPE_WARN { return; }
    }
    if fixup_exception(regs) { return; }
    register_early_console();
    early_printk(c"PANIC: early exception %04x PSW: %016lx %016lx\n", (*regs).int_code & 0xffff, (*regs).psw.mask, (*regs).psw.addr);
    show_regs(regs);
    disabled_wait();
}

#[inline(never)]
unsafe fn setup_lowcore_early() {
    let lc = get_lowcore();
    let mut psw: psw_t = core::mem::zeroed();
    psw.addr = early_pgm_check_handler as usize as c_ulong;
    psw.mask = PSW_KERNEL_BITS;
    (*lc).program_new_psw = psw;
    (*lc).preempt_count = INIT_PREEMPT_COUNT;
    (*lc).return_lpswe = gen_lpswe(__LC_RETURN_PSW);
    (*lc).return_mcck_lpswe = gen_lpswe(__LC_RETURN_MCCK_PSW);
}

unsafe fn save_vector_registers() {
    // CONFIG_CRASH_DUMP
    #[cfg(feature = "CONFIG_CRASH_DUMP")]
    if cpu_has_vx() { save_vx_regs(boot_cpu_vector_save_area); }
}

unsafe fn setup_low_address_protection() { system_ctl_set_bit(0, CR0_LOW_ADDRESS_PROTECTION_BIT); }

unsafe fn setup_access_registers() {
    let acrs: [u32; NUM_ACRS] = [0; NUM_ACRS];
    restore_access_regs(acrs.as_ptr());
}

pub static mut early_command_line: [c_char; COMMAND_LINE_SIZE] = [0; COMMAND_LINE_SIZE];

unsafe fn setup_boot_command_line() { strscpy(boot_command_line, early_command_line.as_ptr(), COMMAND_LINE_SIZE); }

unsafe fn sort_amode31_extable() { sort_extable(__start_amode31_ex_table, __stop_amode31_ex_table); }

pub unsafe fn startup_init() {
    kasan_early_init();
    time_early_init();
    init_kernel_storage_key();
    lockdep_off();
    sort_amode31_extable();
    setup_lowcore_early();
    setup_arch_string();
    setup_boot_command_line();
    save_vector_registers();
    setup_topology();
    sclp_early_detect();
    setup_low_address_protection();
    setup_access_registers();
    lockdep_on();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
