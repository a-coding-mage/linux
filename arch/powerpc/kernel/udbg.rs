// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * polling mode stateless debugging stuff, originally for NS16550 Serial Ports
 *
 * c 2001 PPC 64 Team, IBM Corp
 */

use core::ffi::{c_char, c_int, c_uint, c_ushort, c_void};

extern "C" {
    static mut console_loglevel: c_int;
    static mut early_console: *mut console;
    static mut boot_command_line: *const c_char;

    fn udbg_init_debug_lpar();
    fn udbg_init_debug_lpar_hvsi();
    fn udbg_init_pmac_realmode();
    fn udbg_init_rtas_panel();
    fn udbg_init_pas_realmode();
    fn udbg_init_btext();
    fn udbg_init_44x_as1();
    fn udbg_init_cpm();
    fn udbg_init_usbgecko();
    fn udbg_init_memcons();
    fn udbg_init_ehv_bc();
    fn udbg_init_ps3gelic();
    fn udbg_init_debug_opal_raw();
    fn udbg_init_debug_opal_hvsi();
    fn udbg_init_debug_16550();
    fn register_early_udbg_console();
    fn register_console(con: *mut console);
    fn printk(fmt: *const c_char, ...);
    fn strstr(s: *const c_char, needle: *const c_char) -> *mut c_char;
    fn vsnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, args: *mut c_void) -> c_int;
}

#[repr(C)]
pub struct console {
    pub name: *const c_char,
    pub write: Option<unsafe extern "C" fn(*mut console, *const c_char, c_uint)>,
    pub flags: c_uint,
    pub index: c_int,
}

pub static mut udbg_putc: Option<unsafe extern "C" fn(c_char)> = None;
pub static mut udbg_flush: Option<unsafe extern "C" fn()> = None;
pub static mut udbg_getc: Option<unsafe extern "C" fn() -> c_int> = None;
pub static mut udbg_getc_poll: Option<unsafe extern "C" fn() -> c_int> = None;

/*
 * Early debugging facilities. You can enable _one_ of these via .config,
 * if you do so your kernel _will not boot_ on anything else. Be careful.
 */
pub unsafe extern "C" fn udbg_early_init() {
    // Build-time CONFIG_PPC_EARLY_DEBUG_* selection is preserved here.
    #[cfg(CONFIG_PPC_EARLY_DEBUG_LPAR)]
    udbg_init_debug_lpar();
    #[cfg(CONFIG_PPC_EARLY_DEBUG_LPAR_HVSI)]
    udbg_init_debug_lpar_hvsi();
    #[cfg(CONFIG_PPC_EARLY_DEBUG_G5)]
    udbg_init_pmac_realmode();
    #[cfg(CONFIG_PPC_EARLY_DEBUG_RTAS_PANEL)]
    udbg_init_rtas_panel();
    #[cfg(CONFIG_PPC_EARLY_DEBUG_PAS_REALMODE)]
    udbg_init_pas_realmode();
    #[cfg(CONFIG_PPC_EARLY_DEBUG_BOOTX)]
    udbg_init_btext();
    #[cfg(CONFIG_PPC_EARLY_DEBUG_44x)]
    udbg_init_44x_as1();
    #[cfg(CONFIG_PPC_EARLY_DEBUG_CPM)]
    udbg_init_cpm();
    #[cfg(CONFIG_PPC_EARLY_DEBUG_USBGECKO)]
    udbg_init_usbgecko();
    #[cfg(CONFIG_PPC_EARLY_DEBUG_MEMCONS)]
    udbg_init_memcons();
    #[cfg(CONFIG_PPC_EARLY_DEBUG_EHV_BC)]
    udbg_init_ehv_bc();
    #[cfg(CONFIG_PPC_EARLY_DEBUG_PS3GELIC)]
    udbg_init_ps3gelic();
    #[cfg(CONFIG_PPC_EARLY_DEBUG_OPAL_RAW)]
    udbg_init_debug_opal_raw();
    #[cfg(CONFIG_PPC_EARLY_DEBUG_OPAL_HVSI)]
    udbg_init_debug_opal_hvsi();
    #[cfg(CONFIG_PPC_EARLY_DEBUG_16550)]
    udbg_init_debug_16550();

    #[cfg(CONFIG_PPC_EARLY_DEBUG)]
    {
        console_loglevel = CONSOLE_LOGLEVEL_DEBUG;
        register_early_udbg_console();
    }
}

/* udbg library, used by xmon et al */
pub unsafe extern "C" fn udbg_puts(mut s: *const c_char) {
    if let Some(putc) = udbg_putc {
        if !s.is_null() && *s != 0 {
            while *s != 0 {
                putc(*s);
                s = s.add(1);
            }
        }
        if let Some(flush) = udbg_flush { flush(); }
    }
}

pub unsafe extern "C" fn udbg_write(mut s: *const c_char, mut n: c_int) -> c_int {
    let remain_start = n;
    if udbg_putc.is_none() { return 0; }
    if !s.is_null() && *s != 0 {
        while *s != 0 && { let old = n; n -= 1; old > 0 } {
            udbg_putc.unwrap()(*s);
            s = s.add(1);
        }
    }
    if let Some(flush) = udbg_flush { flush(); }
    remain_start - n
}

pub unsafe extern "C" fn udbg_printf(fmt: *const c_char, mut args: ...) {
    if udbg_putc.is_some() {
        let mut buf = [0i8; 256];
        vsnprintf(buf.as_mut_ptr(), 256, fmt, (&mut args as *mut _) as *mut c_void);
        udbg_puts(buf.as_ptr());
    }
}

pub unsafe extern "C" fn udbg_progress(s: *mut c_char, _hex: c_ushort) {
    udbg_puts(s);
    udbg_puts(b"\n\0".as_ptr() as *const c_char);
}

/* Early boot console based on udbg */
unsafe extern "C" fn udbg_console_write(_con: *mut console, s: *const c_char, n: c_uint) {
    udbg_write(s, n as c_int);
}

const CON_PRINTBUFFER: c_uint = 1 << 0;
const CON_ENABLED: c_uint = 1 << 1;
const CON_BOOT: c_uint = 1 << 2;
const CON_ANYTIME: c_uint = 1 << 3;
const CONSOLE_LOGLEVEL_DEBUG: c_int = 10;

static mut udbg_console: console = console {
    name: b"udbg\0".as_ptr() as *const c_char,
    write: Some(udbg_console_write),
    flags: CON_PRINTBUFFER | CON_ENABLED | CON_BOOT | CON_ANYTIME,
    index: 0,
};

/*
 * Called by setup_system after ppc_md->probe and ppc_md->early_init.
 * Call it again after setting udbg_putc in ppc_md->setup_arch.
 */
pub unsafe extern "C" fn register_early_udbg_console() {
    if !early_console.is_null() || udbg_putc.is_none() { return; }
    if !strstr(boot_command_line, b"udbg-immortal\0".as_ptr() as *const c_char).is_null() {
        printk(b"early console immortal !\n\0".as_ptr() as *const c_char);
        udbg_console.flags &= !CON_BOOT;
    }
    early_console = &mut udbg_console;
    register_console(&mut udbg_console);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
