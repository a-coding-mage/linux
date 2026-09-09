// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (c) 1998  Grant R. Guenther <grant@torque.net>
 *
 * fit2.c is a low-level protocol driver for the older version
 * of the Fidelity International Technology parallel port adapter.
 * This adapter is used in their TransDisk 2000 and older TransDisk
 * 3000 portable hard-drives.  As far as I can tell, this device
 * supports 4-bit mode _only_.
 *
 * Newer models of the FIT products use an enhanced protocol.
 * The "fit3" protocol module should support current drives.
 */

// C headers and symbols supplied by the surrounding kernel translation.

#[inline]
fn j44(a: i32, b: i32) -> i32 {
    ((a >> 4) & 0x0f) | (b & 0xf0)
}

/*
 * cont = 0 - access the IDE register file
 * cont = 1 - access the IDE command set
 *
 * NB: The FIT adapter does not appear to use the control registers.
 * So, we map ALT_STATUS to STATUS and NO-OP writes to the device
 * control register - this means that IDE reset will not work on these
 * devices.
 */

unsafe fn fit2_write_regr(pi: *mut crate::pi_adapter, cont: i32, regr: i32, val: i32) {
    if cont == 1 {
        return;
    }
    crate::w2(pi, 0xc);
    crate::w0(pi, regr);
    crate::w2(pi, 4);
    crate::w0(pi, val);
    crate::w2(pi, 5);
    crate::w0(pi, 0);
    crate::w2(pi, 4);
}

unsafe fn fit2_read_regr(pi: *mut crate::pi_adapter, cont: i32, regr: i32) -> i32 {
    let r: i32;
    let a: i32;
    let b: i32;

    if cont != 0 {
        if regr != 6 {
            return 0xff;
        }
        r = 7;
    } else {
        r = regr + 0x10;
    }

    crate::w2(pi, 0xc);
    crate::w0(pi, r);
    crate::w2(pi, 4);
    crate::w2(pi, 5);
    crate::w0(pi, 0);
    a = crate::r1(pi);
    crate::w0(pi, 1);
    b = crate::r1(pi);
    crate::w2(pi, 4);

    j44(a, b)
}

unsafe fn fit2_read_block(pi: *mut crate::pi_adapter, buf: *mut i8, count: i32) {
    let mut k: i32;
    let a: i32;
    let b: i32;
    let c: i32;
    let d: i32;

    crate::w2(pi, 0xc);
    crate::w0(pi, 0x10);

    k = 0;
    while k < count / 4 {
        crate::w2(pi, 4);
        crate::w2(pi, 5);
        crate::w0(pi, 0);
        a = crate::r1(pi);
        crate::w0(pi, 1);
        b = crate::r1(pi);
        crate::w0(pi, 3);
        c = crate::r1(pi);
        crate::w0(pi, 2);
        d = crate::r1(pi);
        *buf.add((4 * k + 0) as usize) = j44(a, b) as i8;
        *buf.add((4 * k + 1) as usize) = j44(d, c) as i8;

        crate::w2(pi, 4);
        crate::w2(pi, 5);
        a = crate::r1(pi);
        crate::w0(pi, 3);
        b = crate::r1(pi);
        crate::w0(pi, 1);
        c = crate::r1(pi);
        crate::w0(pi, 0);
        d = crate::r1(pi);
        *buf.add((4 * k + 2) as usize) = j44(d, c) as i8;
        *buf.add((4 * k + 3) as usize) = j44(a, b) as i8;
        k += 1;
    }

    crate::w2(pi, 4);
}

unsafe fn fit2_write_block(pi: *mut crate::pi_adapter, buf: *mut i8, count: i32) {
    let mut k: i32;

    crate::w2(pi, 0xc);
    crate::w0(pi, 0);
    k = 0;
    while k < count / 2 {
        crate::w2(pi, 4);
        crate::w0(pi, *buf.add((2 * k) as usize) as i32);
        crate::w2(pi, 5);
        crate::w0(pi, *buf.add((2 * k + 1) as usize) as i32);
        k += 1;
    }
    crate::w2(pi, 4);
}

unsafe fn fit2_connect(pi: *mut crate::pi_adapter) {
    (*pi).saved_r0 = crate::r0(pi);
    (*pi).saved_r2 = crate::r2(pi);
    crate::w2(pi, 0xcc);
}

unsafe fn fit2_disconnect(pi: *mut crate::pi_adapter) {
    crate::w0(pi, (*pi).saved_r0);
    crate::w2(pi, (*pi).saved_r2);
}

unsafe fn fit2_log_adapter(pi: *mut crate::pi_adapter) {
    crate::dev_info(
        &(*pi).dev,
        "FIT 2000 adapter at 0x%x, delay %d\n",
        (*pi).port,
        (*pi).delay,
    );
}

static mut fit2: crate::pi_protocol = crate::pi_protocol {
    owner: crate::THIS_MODULE,
    name: "fit2",
    max_mode: 1,
    epp_first: 2,
    default_delay: 1,
    max_units: 1,
    write_regr: Some(fit2_write_regr),
    read_regr: Some(fit2_read_regr),
    write_block: Some(fit2_write_block),
    read_block: Some(fit2_read_block),
    connect: Some(fit2_connect),
    disconnect: Some(fit2_disconnect),
    log_adapter: Some(fit2_log_adapter),
};

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Grant R. Guenther <grant@torque.net>");
// MODULE_DESCRIPTION("Fidelity International Technology parallel port IDE adapter (older models) protocol driver");
// module_pata_parport_driver(fit2);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
