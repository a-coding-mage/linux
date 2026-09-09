// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2010 Werner Fink, Jiri Slaby
 */

// Translated from consoles.c. Kernel-provided declarations and constants are
// supplied by the surrounding Rust translation unit.

#[repr(C)]
struct ConsoleFlag {
    flag: i16,
    name: i8,
}

static CON_FLAGS: [ConsoleFlag; 7] = [
    ConsoleFlag { flag: CON_ENABLED as i16, name: b'E' as i8 },
    ConsoleFlag { flag: CON_CONSDEV as i16, name: b'C' as i8 },
    ConsoleFlag { flag: CON_BOOT as i16, name: b'B' as i8 },
    ConsoleFlag { flag: CON_NBCON as i16, name: b'N' as i8 },
    ConsoleFlag { flag: CON_PRINTBUFFER as i16, name: b'p' as i8 },
    ConsoleFlag { flag: CON_BRL as i16, name: b'b' as i8 },
    ConsoleFlag { flag: CON_ANYTIME as i16, name: b'a' as i8 },
];

unsafe fn show_console_dev(m: *mut SeqFile, v: *mut core::ffi::c_void) -> i32 {
    let mut flags = [0i8; 8];
    let con = v as *mut Console;
    let mut a: usize;
    let mut dev: DevT = 0;

    if !(*con).device.is_none() {
        let mut driver: *const TtyDriver;
        let mut index: i32 = 0;

        /* Take console_lock to serialize device() with other console operations. */
        console_lock();
        driver = ((*con).device.unwrap())(con, &mut index);
        console_unlock();

        if !driver.is_null() {
            dev = mkdev((*driver).major as u32, (*driver).minor_start as u32);
            dev = dev.wrapping_add(index as DevT);
        }
    }

    a = 0;
    while a < CON_FLAGS.len() {
        flags[a] = if ((*con).flags & CON_FLAGS[a].flag as u32) != 0 {
            CON_FLAGS[a].name
        } else {
            b' ' as i8
        };
        a += 1;
    }
    flags[a] = 0;

    seq_setwidth(m, 21 - 1);
    seq_printf(m, c"%s%d", (*con).name, (*con).index);
    seq_pad(m, b' ' as i32);
    seq_printf(
        m,
        c"%c%c%c (%s)",
        if (*con).read.is_some() { b'R' } else { b'-' },
        if ((*con).flags & CON_NBCON as u32) != 0 || (*con).write.is_some() { b'W' } else { b'-' },
        if (*con).unblank.is_some() { b'U' } else { b'-' },
        flags.as_mut_ptr(),
    );
    if dev != 0 {
        seq_printf(m, c" %4d:%d", major(dev), minor(dev));
    }

    seq_putc(m, b'\n' as i32);
    0
}

unsafe fn c_start(_m: *mut SeqFile, pos: *mut LoFF) -> *mut core::ffi::c_void {
    let mut con: *mut Console;
    let mut off: LoFF = 0;

    console_list_lock();
    con = core::ptr::null_mut();
    for_each_console!(item in con {
        if off == *pos {
            break;
        }
        off += 1;
    });
    con as *mut core::ffi::c_void
}

unsafe fn c_next(_m: *mut SeqFile, v: *mut core::ffi::c_void, pos: *mut LoFF) -> *mut core::ffi::c_void {
    let con = v as *mut Console;
    *pos += 1;
    hlist_entry_safe((*con).node.next, Console, node)
}

unsafe fn c_stop(_m: *mut SeqFile, _v: *mut core::ffi::c_void) {
    console_list_unlock();
}

static CONSOLES_OP: SeqOperations = SeqOperations {
    start: Some(c_start),
    next: Some(c_next),
    stop: Some(c_stop),
    show: Some(show_console_dev),
};

unsafe fn proc_consoles_init() -> i32 {
    proc_create_seq(c"consoles", 0, core::ptr::null_mut(), &CONSOLES_OP);
    0
}

fs_initcall!(proc_consoles_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
