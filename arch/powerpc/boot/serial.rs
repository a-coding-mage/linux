// SPDX-License-Identifier: GPL-2.0
/*
 * Generic serial console support
 *
 * Author: Mark A. Greer <mgreer@mvista.com>
 *
 * Code in serial_edit_cmdline() copied from <file:arch/ppc/boot/simple/misc.c>
 * and was written by Matt Porter <mporter@kernel.crashing.org>.
 *
 * 2001,2006 (c) MontaVista Software, Inc.
 */

// Dependencies supplied by the surrounding translation unit/build.

unsafe fn serial_open() -> i32 {
    let scdp = console_ops.data as *mut serial_console_data;
    ((*scdp).open)()
}

unsafe fn serial_write(mut buf: *const i8, _len: i32) {
    let scdp = console_ops.data as *mut serial_console_data;

    while *buf != 0 {
        ((*scdp).putc)(*buf);
        buf = buf.add(1);
    }
}

unsafe fn serial_edit_cmdline(mut buf: *mut i8, len: i32, timeout: u32) {
    let mut timer: i32 = 0;
    let mut count: i32;
    let mut ch: i8;
    let mut cp: *mut i8;
    let scdp = console_ops.data as *mut serial_console_data;

    count = strlen(buf) as i32;
    cp = buf.add(count as usize);
    count += 1;

    loop {
        if ((*scdp).tstc)() != 0 {
            loop {
                ch = ((*scdp).getc)();
                if ch == b'\n' as i8 || ch == b'\r' as i8 {
                    break;
                }

                /* Test for backspace/delete */
                if ch == b'\b' as i8 || ch == 0o177i8 {
                    if cp != buf {
                        cp = cp.sub(1);
                        count -= 1;
                        printf(b"\b \b\0".as_ptr() as *const i8);
                    }
                /* Test for ^x/^u (and wipe the line) */
                } else if ch == 0o30i8 || ch == 0o25i8 {
                    while cp != buf {
                        cp = cp.sub(1);
                        count -= 1;
                        printf(b"\b \b\0".as_ptr() as *const i8);
                    }
                } else if count < len {
                    *cp = ch;
                    cp = cp.add(1);
                    count += 1;
                    ((*scdp).putc)(ch);
                }
            }
            break; /* Exit 'timer' loop */
        }
        udelay(1000); /* 1 msec */
        timer += 1;
        if !(timer <= timeout as i32) {
            break;
        }
    }
    *cp = 0;
}

unsafe fn serial_close() {
    let scdp = console_ops.data as *mut serial_console_data;

    if ((*scdp).close).is_some() {
        ((*scdp).close.unwrap())();
    }
}

unsafe fn serial_get_stdout_devp() -> *mut core::ffi::c_void {
    let mut devp: *mut core::ffi::c_void;
    let mut devtype = [0i8; MAX_PROP_LEN];
    let mut path = [0i8; MAX_PATH_LEN];

    devp = finddevice(b"/chosen\0".as_ptr() as *const i8);
    if devp.is_null() {
        return core::ptr::null_mut();
    }

    if getprop(devp, b"linux,stdout-path\0".as_ptr() as *const i8,
              path.as_mut_ptr(), MAX_PATH_LEN) > 0
        || getprop(devp, b"stdout-path\0".as_ptr() as *const i8,
                   path.as_mut_ptr(), MAX_PATH_LEN) > 0
    {
        devp = finddevice(path.as_ptr());
        if devp.is_null() {
            return core::ptr::null_mut();
        }

        if getprop(devp, b"device_type\0".as_ptr() as *const i8,
                   devtype.as_mut_ptr(), core::mem::size_of_val(&devtype) as i32) > 0
            && strcmp(devtype.as_ptr(), b"serial\0".as_ptr() as *const i8) == 0
        {
            return devp;
        }
    }
    core::ptr::null_mut()
}

static mut serial_cd: serial_console_data = unsafe { core::mem::zeroed() };

/* Node's "compatible" property determines which serial driver to use */
unsafe fn serial_console_init() -> i32 {
    let devp: *mut core::ffi::c_void;
    let mut rc: i32 = -1;

    devp = serial_get_stdout_devp();
    if devp.is_null() {
        return -1;
    }

    if dt_is_compatible(devp, b"ns16550\0".as_ptr() as *const i8) != 0
        || dt_is_compatible(devp, b"pnpPNP,501\0".as_ptr() as *const i8) != 0
    {
        rc = ns16550_console_init(devp, &raw mut serial_cd);
    }
    // #ifdef CONFIG_CPM
    // else if dt_is_compatible(devp, "fsl,cpm1-scc-uart") || ...
    //     rc = cpm_console_init(devp, &serial_cd);
    // #endif
    // #ifdef CONFIG_PPC_MPC52xx
    // else if dt_is_compatible(devp, "fsl,mpc5200-psc-uart")
    //     rc = mpc5200_psc_console_init(devp, &serial_cd);
    // #endif
    // #ifdef CONFIG_PPC_POWERNV
    // else if dt_is_compatible(devp, "ibm,opal-console-raw")
    //     rc = opal_console_init(devp, &serial_cd);
    // #endif

    /* Add other serial console driver calls here */

    if rc == 0 {
        console_ops.open = Some(serial_open);
        console_ops.write = Some(serial_write);
        console_ops.close = Some(serial_close);
        console_ops.data = &raw mut serial_cd as *mut _;

        if ((*(&raw mut serial_cd)).getc).is_some() {
            console_ops.edit_cmdline = Some(serial_edit_cmdline);
        }

        return 0;
    }
    -1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
