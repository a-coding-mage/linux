// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// Linux/UML headers supplying the external types, constants, macros, and functions below.

pub const DEFAULT_COMMAND_LINE_ROOT: &str = "root=98:0";
pub const DEFAULT_COMMAND_LINE_CONSOLE: &str = "console=tty0";

static mut COMMAND_LINE: [u8; COMMAND_LINE_SIZE] = [0; COMMAND_LINE_SIZE];

unsafe fn add_arg(arg: *mut c_char) {
    if strlen(COMMAND_LINE.as_ptr() as *const c_char) + strlen(arg) + 1 > COMMAND_LINE_SIZE {
        os_warn(b"add_arg: Too many command line arguments!\n\0".as_ptr() as *const c_char);
        exit(1);
    }
    if strlen(COMMAND_LINE.as_ptr() as *const c_char) > 0 {
        strcat(COMMAND_LINE.as_mut_ptr() as *mut c_char, b" \0".as_ptr() as *const c_char);
    }
    strcat(COMMAND_LINE.as_mut_ptr() as *mut c_char, arg);
}

pub static mut BOOT_CPU_DATA: cpuinfo_um = cpuinfo_um {
    loops_per_jiffy: 0,
    cache_alignment: L1_CACHE_BYTES,
    x86_capability: [0; NCAPINTS],
};

static mut HOST_INFO: [c_char; (__NEW_UTS_LEN + 1) * 5] = [0; (__NEW_UTS_LEN + 1) * 5];

unsafe fn show_cpuinfo(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut i: c_int = 0;
    // CONFIG_SMP conditional retained from the source.
    #[cfg(CONFIG_SMP)] {
        i = (v as usize - 1) as c_int;
        if !cpu_online(i) { return 0; }
    }
    seq_printf(m, b"processor\t: %d\n\0".as_ptr() as *const c_char, i);
    seq_printf(m, b"vendor_id\t: User Mode Linux\n\0".as_ptr() as *const c_char);
    seq_printf(m, b"model name\t: UML\n\0".as_ptr() as *const c_char);
    seq_printf(m, b"mode\t\t: skas\n\0".as_ptr() as *const c_char);
    seq_printf(m, b"host\t\t: %s\n\0".as_ptr() as *const c_char, HOST_INFO.as_ptr());
    seq_printf(m, b"fpu\t\t: %s\n\0".as_ptr() as *const c_char, str_yes_no(cpu_has(&BOOT_CPU_DATA, X86_FEATURE_FPU)));
    seq_printf(m, b"flags\t\t:\0".as_ptr() as *const c_char);
    for i in 0..(32 * NCAPINTS) {
        if cpu_has(&BOOT_CPU_DATA, i) && !x86_cap_flags[i].is_null() {
            seq_printf(m, b" %s\0".as_ptr() as *const c_char, x86_cap_flags[i]);
        }
    }
    seq_printf(m, b"\n\0".as_ptr() as *const c_char);
    seq_printf(m, b"cache_alignment\t: %d\n\0".as_ptr() as *const c_char, BOOT_CPU_DATA.cache_alignment);
    seq_printf(m, b"bogomips\t: %lu.%02lu\n\0".as_ptr() as *const c_char,
        LOOPS_PER_JIFFY / (500000 / HZ), (LOOPS_PER_JIFFY / (5000 / HZ)) % 100);
    0
}

unsafe fn c_start(_m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    if *pos < nr_cpu_ids { (*pos + 1) as usize as *mut c_void } else { core::ptr::null_mut() }
}
unsafe fn c_next(m: *mut seq_file, _v: *mut c_void, pos: *mut loff_t) -> *mut c_void { *pos += 1; c_start(m, pos) }
unsafe fn c_stop(_m: *mut seq_file, _v: *mut c_void) {}

pub static CPUINFO_OP: seq_operations = seq_operations { start: Some(c_start), next: Some(c_next), stop: Some(c_stop), show: Some(show_cpuinfo) };

pub static mut UML_PHYSMEM: c_ulong = 0;
pub static mut UML_RESERVED: c_ulong = 0;
pub static mut START_VM: c_ulong = 0;
pub static mut END_VM: c_ulong = 0;
static mut HAVE_ROOT: c_int = 0;
static mut HAVE_CONSOLE: c_int = 0;
pub static mut PHYSMEM_SIZE: u64 = 64 * 1024 * 1024;
static USAGE_STRING: &[u8] = b"User Mode Linux v%s\n\tavailable at http://user-mode-linux.sourceforge.net/\n\n\0";

unsafe fn uml_version_setup(_line: *mut c_char, _add: *mut c_int) -> c_int { printf(b"%s\n\0".as_ptr() as *const c_char, init_utsname().release.as_ptr()); exit(0); 0 }
unsafe fn uml_root_setup(_line: *mut c_char, _add: *mut c_int) -> c_int { HAVE_ROOT = 1; 0 }
unsafe fn uml_console_setup(_line: *mut c_char, _add: *mut c_int) -> c_int { HAVE_CONSOLE = 1; 0 }
unsafe fn Usage(_line: *mut c_char, _add: *mut c_int) -> c_int {
    printf(USAGE_STRING.as_ptr() as *const c_char, init_utsname().release.as_ptr());
    let mut p = &__uml_help_start as *const *const c_char;
    while p < &__uml_help_end as *const *const c_char { printf(b"%s\0".as_ptr() as *const c_char, *p); p = p.add(1); }
    exit(0); 0
}
unsafe fn uml_checksetup(line: *mut c_char, add: *mut c_int) {
    let mut p = &__uml_setup_start as *const uml_param;
    while p < &__uml_setup_end as *const uml_param {
        let n = strlen((*p).str_);
        if strncmp(line, (*p).str_, n) == 0 && ((*p).setup_func)(line.add(n), add) != 0 { return; }
        p = p.add(1);
    }
}
unsafe fn uml_postsetup() { let mut p = &__uml_postsetup_start as *const initcall_t; while p < &__uml_postsetup_end as *const initcall_t { (*p)(); p = p.add(1); } }

unsafe fn panic_exit(_self: *mut notifier_block, _unused1: c_ulong, _unused2: *mut c_void) -> c_int {
    kmsg_dump(KMSG_DUMP_PANIC); bust_spinlocks(1); bust_spinlocks(0); uml_exitcode = 1; os_dump_core(); NOTIFY_DONE
}
static mut PANIC_EXIT_NOTIFIER: notifier_block = notifier_block { notifier_call: Some(panic_exit), priority: INT_MAX - 1 };
pub unsafe fn uml_finishsetup() { atomic_notifier_chain_register(&mut panic_notifier_list, &mut PANIC_EXIT_NOTIFIER); uml_postsetup(); new_thread_handler(); }

pub static mut STUB_START: c_ulong = 0;
pub static mut TASK_SIZE: c_ulong = 0;
pub static mut BRK_START: c_ulong = 0;
pub const MIN_VMALLOC: c_ulong = 32 * 1024 * 1024;

unsafe fn parse_host_cpu_flags(line: *mut c_char) { for i in 0..(32 * NCAPINTS) { if !x86_cap_flags[i].is_null() && !strstr(line, x86_cap_flags[i]).is_null() { set_cpu_cap(&mut BOOT_CPU_DATA, i); } } }
unsafe fn parse_cache_line(line: *mut c_char) { let mut p = strstr(line, b":\0".as_ptr() as *const c_char); if !p.is_null() { p = p.add(1); while *p != 0 && isspace(*p as c_int) != 0 { p = p.add(1); } let mut res = 0; if kstrtoul(p, 10, &mut res) == 0 && is_power_of_2(res) { BOOT_CPU_DATA.cache_alignment = res; } else { BOOT_CPU_DATA.cache_alignment = L1_CACHE_BYTES; } } }
unsafe fn get_top_address(envp: *mut *mut c_char) -> c_ulong { let mut top = (&envp as *const _ as c_ulong); let mut i = 0; while !(*envp.add(i)).is_null() { if (*envp.add(i) as c_ulong) > top { top = *envp.add(i) as c_ulong; } i += 1; } PAGE_ALIGN(top + 1) }

// Remaining architecture entry points retain the C implementation's externally supplied ABI and operations.
pub unsafe fn linux_main(argc: c_int, argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int {
    let mut add; for i in 1..argc { if i == 1 && *(*argv.add(i as usize)) == b' ' as c_char { continue; } add = 1; uml_checksetup(*argv.add(i as usize), &mut add); if add != 0 { add_arg(*argv.add(i as usize)); } }
    if HAVE_ROOT == 0 { add_arg(DEFAULT_COMMAND_LINE_ROOT.as_ptr() as *mut c_char); } if HAVE_CONSOLE == 0 { add_arg(DEFAULT_COMMAND_LINE_CONSOLE.as_ptr() as *mut c_char); }
    let host_task_size = get_top_address(envp); STUB_START = host_task_size - STUB_SIZE; TASK_SIZE = STUB_START; if TASK_SIZE > PTRS_PER_PGD as c_ulong * PGDIR_SIZE { TASK_SIZE = PTRS_PER_PGD as c_ulong * PGDIR_SIZE; } TASK_SIZE &= PGDIR_MASK; os_early_checks(); get_host_cpu_features(parse_host_cpu_flags, parse_cache_line); BRK_START = sbrk(0) as c_ulong;
    let diff = PAGE_ALIGN(BRK_START) - PAGE_ALIGN(&_end as *const _ as c_ulong); if diff > 1024 * 1024 { os_info(b"Adding %ld bytes to physical memory to account for exec-shield gap\n\0".as_ptr() as *const c_char, diff); PHYSMEM_SIZE += diff as u64; }
    UML_PHYSMEM = &__binary_start as *const _ as c_ulong & PAGE_MASK; UML_RESERVED = ROUND_4M(BRK_START) + (1 << 22); setup_machinename(init_utsname().machine.as_mut_ptr()); PHYSMEM_SIZE = PAGE_ALIGN(PHYSMEM_SIZE as c_ulong) as u64; high_physmem = UML_PHYSMEM + PHYSMEM_SIZE as c_ulong; START_VM = VMALLOC_START; END_VM = START_VM + PHYSMEM_SIZE as c_ulong; arch_task_struct_size = size_of::<task_struct>() + host_fp_size; os_flush_stdout(); start_uml()
}

pub unsafe fn read_initrd() -> c_int { 0 }
pub unsafe fn setup_arch(cmdline_p: *mut *mut c_char) { stack_protections(init_task.stack as c_ulong); setup_physmem(UML_PHYSMEM, UML_RESERVED, PHYSMEM_SIZE); uml_dtb_init(); read_initrd(); strscpy(boot_command_line.as_mut_ptr(), COMMAND_LINE.as_ptr(), COMMAND_LINE_SIZE); *cmdline_p = COMMAND_LINE.as_mut_ptr() as *mut c_char; setup_hostinfo(HOST_INFO.as_mut_ptr(), HOST_INFO.len()); prefill_possible_map(); }
pub unsafe fn arch_cpu_finalize_init() { arch_check_bugs(); os_check_bugs(); }
pub unsafe fn text_poke(addr: *mut c_void, opcode: *const c_void, len: usize) -> *mut c_void { WARN_ON(1); memcpy(addr, opcode, len) }
pub unsafe fn text_poke_copy(addr: *mut c_void, opcode: *const c_void, len: usize) -> *mut c_void { text_poke(addr, opcode, len) }
pub unsafe fn apply_seal_endbr(_start: *mut s32, _end: *mut s32) {}
pub unsafe fn apply_retpolines(_start: *mut s32, _end: *mut s32) {}
pub unsafe fn apply_returns(_start: *mut s32, _end: *mut s32) {}
pub unsafe fn apply_fineibt(_start_retpoline: *mut s32, _end_retpoline: *mut s32, _start_cfi: *mut s32, _end_cfi: *mut s32) {}
pub unsafe fn apply_alternatives(_start: *mut alt_instr, _end: *mut alt_instr) {}
pub unsafe fn smp_text_poke_sync_each_cpu() {}
pub unsafe fn uml_pm_wake() { pm_system_wakeup(); }

// CONFIG_PM_SLEEP conditional retained from the source.
#[cfg(CONFIG_PM_SLEEP)]
unsafe fn um_suspend_valid(state: suspend_state_t) -> bool { state == PM_SUSPEND_MEM }
#[cfg(CONFIG_PM_SLEEP)]
unsafe fn um_suspend_prepare() -> c_int { um_irqs_suspend(); 0 }
#[cfg(CONFIG_PM_SLEEP)]
unsafe fn um_suspend_enter(state: suspend_state_t) -> c_int { if WARN_ON(state != PM_SUSPEND_MEM) { return -EINVAL; } um_idle_sleep(); 0 }
#[cfg(CONFIG_PM_SLEEP)]
unsafe fn um_suspend_finish() { um_irqs_resume(); }
#[cfg(CONFIG_PM_SLEEP)]
static UM_SUSPEND_OPS: platform_suspend_ops = platform_suspend_ops { valid: Some(um_suspend_valid), prepare: Some(um_suspend_prepare), enter: Some(um_suspend_enter), finish: Some(um_suspend_finish) };
#[cfg(CONFIG_PM_SLEEP)]
unsafe fn init_pm_wake_signal() -> c_int { if time_travel_mode != TT_MODE_EXTERNAL { register_pm_wake_signal(); } suspend_set_ops(&UM_SUSPEND_OPS); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
