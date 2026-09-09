// SPDX-License-Identifier: GPL-2.0-only
/*
 * Arch related setup for Hexagon
 *
 * Copyright (c) 2010-2013, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation.

pub static mut cmd_line: [core::ffi::c_char; COMMAND_LINE_SIZE] = [0; COMMAND_LINE_SIZE];
static mut default_command_line: [core::ffi::c_char; COMMAND_LINE_SIZE] = CONFIG_CMDLINE;

pub static mut on_simulator: core::ffi::c_int = 0;

extern "C" {
    static mut loops_per_jiffy: core::ffi::c_ulong;
    static mut thread_freq_mhz: core::ffi::c_ulong;
    static mut pcycle_freq_mhz: core::ffi::c_ulong;
    static mut sleep_clk_freq: core::ffi::c_ulong;
    static mut external_cmdline_buffer: core::ffi::c_char;
    static mut boot_command_line: [core::ffi::c_char; COMMAND_LINE_SIZE];
    static nr_cpu_ids: core::ffi::c_ulong;

    fn __vmsetvec(vector: core::ffi::c_ulong);
    fn printk(format: *const core::ffi::c_char, ...);
    fn strscpy(
        dest: *mut core::ffi::c_char,
        src: *const core::ffi::c_char,
        count: usize,
    ) -> isize;
    fn parse_early_param();
    fn setup_arch_memory();
    fn smp_start_cpus();
    fn cpu_online(cpu: core::ffi::c_int) -> bool;
    fn seq_printf(m: *mut seq_file, format: *const core::ffi::c_char, ...);
}

// Build-time configuration constants and symbols supplied by the kernel.
const COMMAND_LINE_SIZE: usize = 2048;
const CONFIG_CMDLINE: [core::ffi::c_char; COMMAND_LINE_SIZE] = [0; COMMAND_LINE_SIZE];
const HZ: core::ffi::c_ulong = 100;
const PHYS_OFFSET: core::ffi::c_ulong = 0;
const _K_VM_event_vector: core::ffi::c_ulong = 0;
const KERN_INFO: *const core::ffi::c_char = b"<6>\0".as_ptr() as *const _;

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_operations {
    pub start: Option<unsafe extern "C" fn(*mut seq_file, *mut i64) -> *mut core::ffi::c_void>,
    pub next: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void, *mut i64) -> *mut core::ffi::c_void>,
    pub stop: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void)>,
    pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> core::ffi::c_int>,
}

#[no_mangle]
pub unsafe extern "C" fn calibrate_delay() {
    loops_per_jiffy = thread_freq_mhz.wrapping_mul(1_000_000) / HZ;
}

/*
 * setup_arch -  high level architectural setup routine
 * @cmdline_p: pointer to pointer to command-line arguments
 */
#[no_mangle]
pub unsafe extern "C" fn setup_arch(cmdline_p: *mut *mut core::ffi::c_char) {
    let p: *mut core::ffi::c_char = &mut external_cmdline_buffer;

    /*
     * These will eventually be pulled in via either some hypervisor
     * or devicetree description.  Hardwiring for now.
     */
    pcycle_freq_mhz = 600;
    thread_freq_mhz = 100;
    sleep_clk_freq = 32000;

    /* Set up event bindings to handle exceptions and interrupts. */
    __vmsetvec(_K_VM_event_vector);

    printk(b"<6>PHYS_OFFSET=0x%08lx\n\0".as_ptr() as *const _, PHYS_OFFSET);

    if *(((_end as usize + 8) as *const i32)) == 0x1f1f1f1f {
        on_simulator = 1;
    } else {
        on_simulator = 0;
    }

    if *p != 0 {
        strscpy(boot_command_line.as_mut_ptr(), p, COMMAND_LINE_SIZE);
    } else {
        strscpy(boot_command_line.as_mut_ptr(), default_command_line.as_ptr(), COMMAND_LINE_SIZE);
    }

    strscpy(cmd_line.as_mut_ptr(), boot_command_line.as_ptr(), COMMAND_LINE_SIZE);
    *cmdline_p = cmd_line.as_mut_ptr();

    parse_early_param();
    setup_arch_memory();

    #[cfg(CONFIG_SMP)]
    smp_start_cpus();
}

static unsafe extern "C" fn c_start(_m: *mut seq_file, pos: *mut i64) -> *mut core::ffi::c_void {
    if *pos < nr_cpu_ids as i64 {
        (*pos as usize + 1) as *mut core::ffi::c_void
    } else {
        core::ptr::null_mut()
    }
}

static unsafe extern "C" fn c_next(m: *mut seq_file, _v: *mut core::ffi::c_void, pos: *mut i64) -> *mut core::ffi::c_void {
    *pos += 1;
    c_start(m, pos)
}

static unsafe extern "C" fn c_stop(_m: *mut seq_file, _v: *mut core::ffi::c_void) {}

static unsafe extern "C" fn show_cpuinfo(m: *mut seq_file, v: *mut core::ffi::c_void) -> core::ffi::c_int {
    let cpu = v as usize as i64 - 1;
    #[cfg(CONFIG_SMP)]
    if !cpu_online(cpu as core::ffi::c_int) {
        return 0;
    }

    seq_printf(m, b"processor\t: %d\n\0".as_ptr() as *const _, cpu as core::ffi::c_int);
    seq_printf(m, b"model name\t: Hexagon Virtual Machine\n\0".as_ptr() as *const _);
    seq_printf(m, b"BogoMips\t: %lu.%02lu\n\0".as_ptr() as *const _,
        (loops_per_jiffy * HZ) / 500000,
        ((loops_per_jiffy * HZ) / 5000) % 100);
    seq_printf(m, b"\n\0".as_ptr() as *const _);
    0
}

#[no_mangle]
pub static cpuinfo_op: seq_operations = seq_operations {
    start: Some(c_start),
    next: Some(c_next),
    stop: Some(c_stop),
    show: Some(show_cpuinfo),
};

extern "C" {
    static _end: u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
