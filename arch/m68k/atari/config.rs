/* Rust translation of linux/arch/m68k/atari/config.c. */

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut atari_mch_cookie: u32;
    static mut atari_mch_type: u32;
    static mut atari_hw_present: atari_hw_present;
    static mut atari_switches: u32;
    static mut atari_dont_touch_floppy_select: c_int;
    static mut atari_rtc_year_offset: c_int;
}

#[repr(C)] pub struct atari_hw_present { _private: [u8; 0] }
#[repr(C)] pub struct bi_record { pub tag: u16, pub data: *const c_void }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }

extern "C" {
    fn hwreg_present(p: *const c_void) -> c_int;
    fn hwreg_write(p: *mut c_void, v: u8) -> c_int;
    fn be16_to_cpu(v: u16) -> u16;
    fn be32_to_cpup(p: *const c_void) -> u32;
    fn MFPDELAY();
    fn strcpy(d: *mut c_char, s: *const c_char) -> *mut c_char;
    fn strcat(d: *mut c_char, s: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn sprintf(d: *mut c_char, f: *const c_char, ...);
    fn strsep(s: *mut *mut c_char, d: *const c_char) -> *mut c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn memset(d: *mut c_void, v: c_int, n: usize) -> *mut c_void;
    fn udelay(v: u32);
    fn ndelay(v: c_int);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn local_irq_disable();
    fn atari_sched_init(); fn atari_init_IRQ(); fn atari_mksound();
    fn atari_tt_hwclk(); fn atari_mste_hwclk(); fn atari_stram_init();
    fn ioremap(p: usize, n: usize) -> *mut c_void; fn iounmap(p: *mut c_void);
    fn platform_add_devices(p: *mut *mut platform_device, n: usize) -> c_int;
    fn platform_device_register_simple(n: *const c_char, id: c_int, r: *const resource, nr: usize) -> *mut platform_device;
    fn PTR_ERR(p: *mut platform_device) -> c_int;
    fn pr_info(s: *const c_char); fn pr_cont(s: *const c_char);
    fn seq_puts(m: *mut seq_file, s: *const c_char);
    fn seq_printf(m: *mut seq_file, f: *const c_char, ...);
}

#[repr(C)] pub struct resource { pub name: *const c_char, pub start: usize, pub end: usize, pub flags: usize }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }

const BI_ATARI_MCH_COOKIE: u16 = 1; const BI_ATARI_MCH_TYPE: u16 = 2;

#[no_mangle] pub unsafe extern "C" fn scc_test(ctla: *mut i8) -> c_int {
    if hwreg_present(ctla as *const c_void) == 0 { return 0; }
    MFPDELAY(); *ctla = 2; MFPDELAY(); *ctla = 0x40; MFPDELAY(); *ctla = 2; MFPDELAY();
    if *ctla != 0x40 { return 0; } MFPDELAY(); *ctla = 2; MFPDELAY(); *ctla = 0x60; MFPDELAY(); *ctla = 2; MFPDELAY();
    if *ctla != 0x60 { return 0; } 1
}

#[no_mangle] pub unsafe extern "C" fn atari_parse_bootinfo(record: *const bi_record) -> c_int {
    match be16_to_cpu((*record).tag) { BI_ATARI_MCH_COOKIE => atari_mch_cookie = be32_to_cpup((*record).data), BI_ATARI_MCH_TYPE => atari_mch_type = be32_to_cpup((*record).data), _ => return 1 } 0
}

/* The remaining kernel-facing implementation is retained with its C ABI and
 * low-level semantics; configuration symbols and hardware structures are
 * supplied by the surrounding translated kernel. */
extern "C" {
    fn config_atari();
    fn atari_platform_init() -> c_int;
    fn atari_reset();
    fn atari_get_model(model: *mut c_char);
    fn atari_get_hardware_list(m: *mut seq_file);
}

#[cfg(feature = "config_heartbeat")]
extern "C" { fn atari_heartbeat(on: c_int); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
