// SPDX-License-Identifier: GPL-2.0
/*
 * Kernel Debugger Architecture Dependent Console I/O handler
 *
 * Copyright (c) 1999-2006 Silicon Graphics, Inc.  All Rights Reserved.
 * Copyright (c) 2009 Wind River Systems, Inc.  All Rights Reserved.
 */

// Dependency declarations supplied by the surrounding kernel build.

const KBD_STATUS_REG: u16 = 0x64;
const KBD_DATA_REG: u16 = 0x60;
const KBD_STAT_OBF: u8 = 0x01;
const KBD_STAT_MOUSE_OBF: u8 = 0x20;

#[inline]
const fn ctrl(c: i32) -> i32 { c - 64 }

static mut KBD_EXISTS: i32 = 0;
static mut KBD_LAST_RET: i32 = 0;

extern "C" {
    fn inb(port: u16) -> u8;
    fn cpu_relax();
    fn kdb_printf(fmt: *const u8, ...);
    static plain_map: [u16; 128];
    static key_maps: [*const u16; 5];
}

// KDB_FLAG, NO_I8042, NO_VT_CONSOLE, KTYP, KT_LETTER, KT_LATIN,
// KT_SPEC, K_ENTER, and isprint are provided by the kernel headers.

pub unsafe fn kdb_get_kbd_char() -> i32 {
    let mut scancode: i32;
    let mut scanstatus: u8;
    static mut shift_lock: i32 = 0;
    static mut shift_key: i32 = 0;
    static mut ctrl_key: i32 = 0;
    let mut keychar: u16;

    if KDB_FLAG!(NO_I8042) || KDB_FLAG!(NO_VT_CONSOLE)
        || (inb(KBD_STATUS_REG) == 0xff && inb(KBD_DATA_REG) == 0xff)
    {
        KBD_EXISTS = 0;
        return -1;
    }
    KBD_EXISTS = 1;

    if (inb(KBD_STATUS_REG) & KBD_STAT_OBF) == 0 { return -1; }

    scancode = inb(KBD_DATA_REG) as i32;
    scanstatus = inb(KBD_STATUS_REG);

    if (scanstatus & KBD_STAT_MOUSE_OBF) != 0 { return -1; }

    if ((scancode & 0x7f) == 0x2a) || ((scancode & 0x7f) == 0x36) {
        if (scancode & 0x80) == 0 { shift_key = 1; } else { shift_key = 0; }
        return -1;
    }

    if (scancode & 0x7f) == 0x1d {
        if (scancode & 0x80) == 0 { ctrl_key = 1; } else { ctrl_key = 0; }
        return -1;
    }

    if (scancode & 0x80) != 0 {
        if scancode == 0x9c { KBD_LAST_RET = 0; }
        return -1;
    }
    scancode &= 0x7f;

    if scancode == 0x3a {
        shift_lock ^= 1;
        // #ifdef KDB_BLINK_LED: kdb_toggleled(0x4)
        return -1;
    }
    if scancode == 0x0e { return 8; }

    match scancode {
        0x0f => return ctrl('I' as i32),
        0x53 => return ctrl('D' as i32),
        0x47 => return ctrl('A' as i32),
        0x4f => return ctrl('E' as i32),
        0x4b => return ctrl('B' as i32),
        0x48 => return ctrl('P' as i32),
        0x50 => return ctrl('N' as i32),
        0x4d => return ctrl('F' as i32),
        _ => {}
    }

    if scancode == 0x73 { scancode = 0x59; }
    else if scancode == 0x7d { scancode = 0x7c; }

    if shift_lock == 0 && shift_key == 0 && ctrl_key == 0 {
        keychar = plain_map[scancode as usize];
    } else if (shift_lock != 0 || shift_key != 0) && !key_maps[1].is_null() {
        keychar = *key_maps[1].add(scancode as usize);
    } else if ctrl_key != 0 && !key_maps[4].is_null() {
        keychar = *key_maps[4].add(scancode as usize);
    } else {
        keychar = 0x0020;
        kdb_printf(b"Unknown state/scancode (%d)\n\0".as_ptr(), scancode);
    }
    keychar &= 0x0fff;
    if keychar == b'\t' as u16 { keychar = b' ' as u16; }

    match KTYP!(keychar) {
        KT_LETTER | KT_LATIN => match keychar as i32 {
            x if x == ctrl('A' as i32) || x == ctrl('B' as i32)
                || x == ctrl('D' as i32) || x == ctrl('E' as i32)
                || x == ctrl('F' as i32) || x == ctrl('I' as i32)
                || x == ctrl('N' as i32) || x == ctrl('P' as i32) => return x,
            _ => {}
        },
        KT_SPEC if keychar == K_ENTER => {},
        _ => return -1,
    }

    if scancode == 0x1c { KBD_LAST_RET = 1; return 13; }
    (keychar & 0xff) as i32
}

pub unsafe fn kdb_kbd_cleanup_state() {
    let mut scancode: i32;
    let mut scanstatus: u8;
    if KBD_LAST_RET == 0 { return; }
    KBD_LAST_RET = 0;

    loop {
        while (inb(KBD_STATUS_REG) & KBD_STAT_OBF) == 0 { cpu_relax(); }
        scancode = inb(KBD_DATA_REG) as i32;
        scanstatus = inb(KBD_STATUS_REG);
        if (scanstatus & KBD_STAT_MOUSE_OBF) != 0 { continue; }
        if scancode != 0x9c { continue; }
        return;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
