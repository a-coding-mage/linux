// SPDX-License-Identifier: GPL-2.0
/*
 * init.c: PROM library initialisation code.
 *
 * Copyright (C) 1998 Harald Koerfgen
 * Copyright (C) 2002, 2004, 2026  Maciej W. Rozycki
 */

// Linux and architecture headers provide the declarations used below.

#[cfg(CONFIG_64BIT)]
pub static mut o32_stk: [core::ffi::c_ulong; O32_STK_SIZE] = [0; O32_STK_SIZE];

pub static mut __rex_bootinit: Option<unsafe extern "C" fn() -> core::ffi::c_int> = None;
pub static mut __rex_bootread: Option<unsafe extern "C" fn() -> core::ffi::c_int> = None;
pub static mut __rex_getbitmap: Option<unsafe extern "C" fn(*mut memmap) -> core::ffi::c_int> = None;
pub static mut __rex_slot_address:
    Option<unsafe extern "C" fn(core::ffi::c_int) -> *mut core::ffi::c_ulong> = None;
pub static mut __rex_gettcinfo: Option<unsafe extern "C" fn() -> *mut core::ffi::c_void> = None;
pub static mut __rex_getsysid: Option<unsafe extern "C" fn() -> core::ffi::c_int> = None;
pub static mut __rex_clear_cache: Option<unsafe extern "C" fn()> = None;

pub static mut __prom_getchar: Option<unsafe extern "C" fn() -> core::ffi::c_int> = None;
pub static mut __prom_getenv:
    Option<unsafe extern "C" fn(*mut core::ffi::c_char) -> *mut core::ffi::c_char> = None;
pub static mut __prom_printf: Option<unsafe extern "C" fn(*mut core::ffi::c_char, ...)> = None;

pub static mut __pmax_open:
    Option<unsafe extern "C" fn(*mut core::ffi::c_char, core::ffi::c_int) -> core::ffi::c_int> = None;
pub static mut __pmax_lseek:
    Option<unsafe extern "C" fn(core::ffi::c_int, core::ffi::c_long, core::ffi::c_int) -> core::ffi::c_int> = None;
pub static mut __pmax_read:
    Option<unsafe extern "C" fn(core::ffi::c_int, *mut core::ffi::c_void, core::ffi::c_int) -> core::ffi::c_int> = None;
pub static mut __pmax_close: Option<unsafe extern "C" fn(core::ffi::c_int) -> core::ffi::c_int> = None;

/*
 * Detect which PROM the DECSTATION has, and set the callback vectors
 * appropriately.
 */
unsafe fn which_prom(magic: i32, prom_vec: *mut i32) {
    /*
     * No sign of the REX PROM's magic number means we assume a non-REX
     * machine (i.e. we're on a DS2100/3100, DS5100 or DS5000/2xx)
     */
    if prom_is_rex(magic) {
        /* Set up prom abstraction structure with REX entry points. */
        __rex_bootinit = core::mem::transmute((*prom_vec.add(REX_PROM_BOOTINIT)) as isize);
        __rex_bootread = core::mem::transmute((*prom_vec.add(REX_PROM_BOOTREAD)) as isize);
        __rex_getbitmap = core::mem::transmute((*prom_vec.add(REX_PROM_GETBITMAP)) as isize);
        __prom_getchar = core::mem::transmute((*prom_vec.add(REX_PROM_GETCHAR)) as isize);
        __prom_getenv = core::mem::transmute((*prom_vec.add(REX_PROM_GETENV)) as isize);
        __rex_getsysid = core::mem::transmute((*prom_vec.add(REX_PROM_GETSYSID)) as isize);
        __rex_gettcinfo = core::mem::transmute((*prom_vec.add(REX_PROM_GETTCINFO)) as isize);
        __prom_printf = core::mem::transmute((*prom_vec.add(REX_PROM_PRINTF)) as isize);
        __rex_slot_address = core::mem::transmute((*prom_vec.add(REX_PROM_SLOTADDR)) as isize);
        __rex_clear_cache = core::mem::transmute((*prom_vec.add(REX_PROM_CLEARCACHE)) as isize);
    } else {
        /* Set up prom abstraction structure with non-REX entry points. */
        __prom_getchar = Some(core::mem::transmute(PMAX_PROM_GETCHAR as usize));
        __prom_getenv = Some(core::mem::transmute(PMAX_PROM_GETENV as usize));
        __prom_printf = Some(core::mem::transmute(PMAX_PROM_PRINTF as usize));
        __pmax_open = Some(core::mem::transmute(PMAX_PROM_OPEN as usize));
        __pmax_lseek = Some(core::mem::transmute(PMAX_PROM_LSEEK as usize));
        __pmax_read = Some(core::mem::transmute(PMAX_PROM_READ as usize));
        __pmax_close = Some(core::mem::transmute(PMAX_PROM_CLOSE as usize));
    }
}

pub unsafe fn prom_init() {
    extern "C" {
        fn dec_machine_halt();
    }
    static CPU_MSG: &[u8] = b"Sorry, this kernel is compiled for a wrong CPU type!\n\0";
    let argc: i32 = fw_arg0;
    let argv: *mut i32 = fw_arg1 as *mut i32;
    let magic: u32 = fw_arg2;
    let prom_vec: *mut i32 = fw_arg3 as *mut i32;

    which_prom(magic as i32, prom_vec);
    if prom_is_rex(magic as i32) {
        rex_clear_cache();
    }
    register_prom_console();

    #[cfg(CONFIG_CPU_R3000)]
    if current_cpu_type() == CPU_R4000SC || current_cpu_type() == CPU_R4400SC {
        static R4K_MSG: &[u8] = b"Please recompile with \"CONFIG_CPU_R4X00 = y\".\n\0";
        printk(CPU_MSG.as_ptr() as *mut core::ffi::c_char);
        printk(R4K_MSG.as_ptr() as *mut core::ffi::c_char);
        dec_machine_halt();
    }

    #[cfg(CONFIG_CPU_R4X00)]
    if current_cpu_type() == CPU_R3000 || current_cpu_type() == CPU_R3000A {
        static R3K_MSG: &[u8] = b"Please recompile with \"CONFIG_CPU_R3000 = y\".\n\0";
        printk(CPU_MSG.as_ptr() as *mut core::ffi::c_char);
        printk(R3K_MSG.as_ptr() as *mut core::ffi::c_char);
        dec_machine_halt();
    }

    prom_meminit(magic as i32);
    prom_identify_arch(magic as i32);
    prom_init_cmdline(argc, argv, magic as i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
