// SPDX-License-Identifier: GPL-2.0
/*
 *  C interface for trapping into the standard LinuxSH BIOS.
 *
 *  Copyright (C) 2000 Greg Banks, Mitch Davis
 *  Copyright (C) 1999, 2000  Niibe Yutaka
 *  Copyright (C) 2002  M. R. Brown
 *  Copyright (C) 2004 - 2010  Paul Mundt
 */

const BIOS_CALL_CONSOLE_WRITE: libc::c_long = 0;
const BIOS_CALL_ETH_NODE_ADDR: libc::c_long = 10;
const BIOS_CALL_SHUTDOWN: libc::c_long = 11;
const BIOS_CALL_GDB_DETACH: libc::c_long = 0xff;

#[no_mangle]
pub static mut gdb_vbr_vector: *mut libc::c_void = core::ptr::null_mut();

#[inline]
unsafe fn sh_bios_call(
    func: libc::c_long,
    arg0: libc::c_long,
    arg1: libc::c_long,
    arg2: libc::c_long,
    arg3: libc::c_long,
) -> libc::c_long {
    if gdb_vbr_vector.is_null() {
        return -libc::ENOSYS as libc::c_long;
    }

    // The C implementation performs `trapa #0x3f` with arguments in SH
    // registers r0, r4, r5, r6, and r7. Preserve that target-specific trap.
    let mut r0 = func;
    core::arch::asm!(
        "trapa #0x3f",
        inout("r0") r0,
        in("r4") arg0,
        in("r5") arg1,
        in("r6") arg2,
        in("r7") arg3,
        options(nostack)
    );
    r0
}

#[no_mangle]
pub unsafe extern "C" fn sh_bios_console_write(buf: *const libc::c_char, len: libc::c_uint) {
    sh_bios_call(
        BIOS_CALL_CONSOLE_WRITE,
        buf as libc::c_long,
        len as libc::c_long,
        0,
        0,
    );
}

#[no_mangle]
pub unsafe extern "C" fn sh_bios_gdb_detach() {
    sh_bios_call(BIOS_CALL_GDB_DETACH, 0, 0, 0, 0);
}

#[no_mangle]
pub unsafe extern "C" fn sh_bios_get_node_addr(node_addr: *mut libc::c_uchar) {
    sh_bios_call(BIOS_CALL_ETH_NODE_ADDR, 0, node_addr as libc::c_long, 0, 0);
}

#[no_mangle]
pub unsafe extern "C" fn sh_bios_shutdown(how: libc::c_uint) {
    sh_bios_call(BIOS_CALL_SHUTDOWN, how as libc::c_long, 0, 0, 0);
}

/*
 * Read the old value of the VBR register to initialise the vector
 * through which debug and BIOS traps are delegated by the Linux trap
 * handler.
 */
#[no_mangle]
pub unsafe extern "C" fn sh_bios_vbr_init() {
    if !gdb_vbr_vector.is_null() {
        return;
    }

    let vbr: libc::c_ulong;
    core::arch::asm!("stc vbr, {0}", out(reg) vbr);

    if vbr != 0 {
        gdb_vbr_vector = (vbr.wrapping_add(0x100)) as *mut libc::c_void;
        // printk(KERN_NOTICE, "Setting GDB trap vector to %p\n", gdb_vbr_vector);
    } else {
        // printk(KERN_NOTICE, "SH-BIOS not detected\n");
    }
}

/**
 * sh_bios_vbr_reload - Re-load the system VBR from the BIOS vector.
 *
 * This can be used by save/restore code to reinitialize the system VBR
 * from the fixed BIOS VBR. A no-op if no BIOS VBR is known.
 */
#[no_mangle]
pub unsafe extern "C" fn sh_bios_vbr_reload() {
    if !gdb_vbr_vector.is_null() {
        let vbr = (gdb_vbr_vector as libc::c_ulong).wrapping_sub(0x100);
        core::arch::asm!("ldc {0}, vbr", in(reg) vbr, options(nostack));
    }
}

// CONFIG_EARLY_PRINTK-dependent console implementation is retained below
// as a source-level translation; kernel console types and constants are
// supplied by the surrounding LinuxSH Rust bindings.
#[cfg(CONFIG_EARLY_PRINTK)]
mod early_printk {
    use super::*;

    unsafe fn sh_console_write(_co: *mut console, s: *const libc::c_char, count: libc::c_uint) {
        sh_bios_console_write(s, count);
    }

    unsafe fn sh_console_setup(_co: *mut console, _options: *mut libc::c_char) -> libc::c_int {
        let mut cflag = CREAD | HUPCL | CLOCAL;
        cflag |= B115200 | CS8;
        (*_co).cflag = cflag;
        0
    }

    static mut bios_console: console = console {
        name: b"bios\0".as_ptr() as *const libc::c_char,
        write: Some(sh_console_write),
        setup: Some(sh_console_setup),
        flags: CON_PRINTBUFFER,
        index: -1,
        ..console::ZERO
    };

    unsafe fn setup_early_printk(buf: *mut libc::c_char) -> libc::c_int {
        let mut keep_early = 0;
        if buf.is_null() {
            return 0;
        }
        if !strstr(buf, b"keep\0".as_ptr() as *const libc::c_char).is_null() {
            keep_early = 1;
        }
        if strncmp(buf, b"bios\0".as_ptr() as *const libc::c_char, 4) == 0 {
            early_console = &mut bios_console;
        }
        if !early_console.is_null() {
            if keep_early != 0 {
                (*early_console).flags &= !CON_BOOT;
            } else {
                (*early_console).flags |= CON_BOOT;
            }
            register_console(early_console);
        }
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
