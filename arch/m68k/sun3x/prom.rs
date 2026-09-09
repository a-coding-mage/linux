// SPDX-License-Identifier: GPL-2.0
/* Prom access routines for the sun3x */

// Dependencies supplied by the surrounding kernel sources are intentionally
// referenced here rather than reimplemented.

extern "C" {
    static mut idprom: *mut idprom;
    static mut vectors: *mut e_vector;
}

pub static mut sun3x_putchar: Option<unsafe extern "C" fn(i32)> = None;
pub static mut sun3x_getchar: Option<unsafe extern "C" fn() -> i32> = None;
pub static mut sun3x_mayget: Option<unsafe extern "C" fn() -> i32> = None;
pub static mut sun3x_mayput: Option<unsafe extern "C" fn(i32) -> i32> = None;
pub static mut sun3x_prom_reboot: Option<unsafe extern "C" fn()> = None;
pub static mut sun3x_prom_abort: e_vector = 0;
pub static mut romvec: *mut linux_romvec = core::ptr::null_mut();

/* prom vector table */
pub static mut sun3x_prom_vbr: *mut e_vector = core::ptr::null_mut();

/* Handle returning to the prom */
unsafe fn sun3x_halt() {
    let mut flags: c_ulong = 0;

    /* Disable interrupts while we mess with things */
    local_irq_save(&mut flags);

    /* Restore prom vbr */
    core::arch::asm!("movec {0},%vbr", in(reg) sun3x_prom_vbr);

    /* Restore prom NMI clock */
    // sun3x_disable_intreg(5);
    sun3_enable_irq(7);

    /* Let 'er rip */
    core::arch::asm!("trap #14");

    /* Restore everything */
    sun3_disable_irq(7);
    sun3_enable_irq(5);

    core::arch::asm!("movec {0},%vbr", in(reg) vectors);
    local_irq_restore(flags);
}

pub unsafe extern "C" fn sun3x_reboot() {
    /* This never returns, don't bother saving things */
    local_irq_disable();

    /* Restore prom vbr */
    core::arch::asm!("movec {0},%vbr", in(reg) sun3x_prom_vbr);

    /* Restore prom NMI clock */
    sun3_disable_irq(5);
    sun3_enable_irq(7);

    /* Let 'er rip */
    ((*romvec).pv_reboot)("vmlinux".as_ptr() as *const i8);
}

unsafe fn sun3x_prom_write(_co: *mut console, mut s: *const i8, mut count: c_uint) {
    while count != 0 {
        if *s == b'\n' as i8 {
            if let Some(putchar) = sun3x_putchar {
                putchar(b'\r' as i32);
            }
        }
        if let Some(putchar) = sun3x_putchar {
            putchar(*s as i32);
        }
        s = s.add(1);
        count -= 1;
    }
}

/* debug console - write-only */

static mut sun3x_debug: console = console {
    name: b"debug\0".as_ptr() as *const i8,
    write: Some(sun3x_prom_write),
    flags: CON_PRINTBUFFER,
    index: -1,
};

pub unsafe extern "C" fn sun3x_prom_init() {
    /* Read the vector table */

    sun3x_putchar = Some(core::mem::transmute(*(SUN3X_P_PUTCHAR as *const usize)));
    sun3x_getchar = Some(core::mem::transmute(*(SUN3X_P_GETCHAR as *const usize)));
    sun3x_mayget = Some(core::mem::transmute(*(SUN3X_P_MAYGET as *const usize)));
    sun3x_mayput = Some(core::mem::transmute(*(SUN3X_P_MAYPUT as *const usize)));
    sun3x_prom_reboot = Some(core::mem::transmute(*(SUN3X_P_REBOOT as *const usize)));
    sun3x_prom_abort = *(SUN3X_P_ABORT as *const e_vector);
    romvec = SUN3X_PROM_BASE as *mut linux_romvec;

    idprom_init();

    if !(((*idprom).id_machtype & SM_ARCH_MASK) == SM_SUN3X) {
        pr_warn(b"Machine reports strange type %02x\n\0".as_ptr() as *const i8,
                (*idprom).id_machtype);
        pr_warn(b"Pretending it's a 3/80, but very afraid...\n\0".as_ptr() as *const i8);
        (*idprom).id_machtype = SM_SUN3X | SM_3_80;
    }

    /* point trap #14 at abort.
     * XXX this is futile since we restore the vbr first - oops
     */
    *vectors.add(VEC_TRAP14 as usize) = sun3x_prom_abort;
}

unsafe extern "C" fn sun3x_debug_setup(arg: *mut i8) -> i32 {
    /* If debug=prom was specified, start the debug console */
    if MACH_IS_SUN3X && strcmp(arg, b"prom\0".as_ptr() as *const i8) == 0 {
        register_console(&mut sun3x_debug);
    }
    0
}

early_param!(b"debug\0".as_ptr() as *const i8, sun3x_debug_setup);

/* some prom functions to export */
pub unsafe extern "C" fn prom_getintdefault(_node: i32, _property: *mut i8, deflt: i32) -> i32 {
    deflt
}

pub unsafe extern "C" fn prom_getbool(_node: i32, _prop: *mut i8) -> i32 {
    1
}

pub unsafe extern "C" fn prom_printf(_fmt: *mut i8, ...) {}

pub unsafe extern "C" fn prom_halt() {
    sun3x_halt();
}

/* Get the idprom and stuff it into buffer 'idbuf'.  Returns the
 * format type.  'num_bytes' is the number of bytes that your idbuf
 * has space for.  Returns 0xff on error.
 */
pub unsafe extern "C" fn prom_get_idprom(idbuf: *mut i8, num_bytes: i32) -> u8 {
    let mut i = 0;

    /* make a copy of the idprom structure */
    while i < num_bytes {
        *idbuf.add(i as usize) = *(SUN3X_IDPROM as *const i8).add(i as usize);
        i += 1;
    }

    *idbuf as u8
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
