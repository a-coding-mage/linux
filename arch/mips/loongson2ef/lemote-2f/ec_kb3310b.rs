// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Basic KB3310B Embedded Controller support for the YeeLoong 2F netbook
 *
 *  Copyright (C) 2008 Lemote Inc.
 *  Author: liujl <liujl@lemote.com>, 2008-04-20
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn spin_lock_irqsave(lock: *mut core::ffi::c_void, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut core::ffi::c_void, flags: usize);
    fn outb(value: u8, port: u16);
    fn inb(port: u16) -> u8;
    fn udelay(usecs: u32);
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

static EC_QUERY_TIMEOUT_MSG: &[u8] = b"%s: deadable error : timeout...\n\0";
static EC_QUERY_STATUS_MSG: &[u8] = b"(%x/%d)ec issued command %d status : 0x%x\n\0";
static EC_EVENT_TIMEOUT_MSG: &[u8] = b"%s: get event number timeout.\n\0";

// Constants and command symbols are supplied by ec_kb3310b.h.
extern "C" {
    static EC_IO_PORT_HIGH: u16;
    static EC_IO_PORT_LOW: u16;
    static EC_IO_PORT_DATA: u16;
    static EC_CMD_PORT: u16;
    static EC_STS_PORT: u16;
    static EC_DAT_PORT: u16;
    static EC_REG_DELAY: u32;
    static EC_CMD_TIMEOUT: i32;
    static CMD_GET_EVENT_NUM: u8;
}

static mut index_access_lock: core::ffi::c_void = core::mem::MaybeUninit::uninit().assume_init();
static mut port_access_lock: core::ffi::c_void = core::mem::MaybeUninit::uninit().assume_init();

#[no_mangle]
pub unsafe extern "C" fn ec_read(addr: u16) -> u8 {
    let mut value: u8;
    let mut flags: usize = 0;

    spin_lock_irqsave(&raw mut index_access_lock, &mut flags);
    outb((addr & 0xff00) >> 8, EC_IO_PORT_HIGH);
    outb(addr & 0x00ff, EC_IO_PORT_LOW);
    value = inb(EC_IO_PORT_DATA);
    spin_unlock_irqrestore(&raw mut index_access_lock, flags);

    value
}

#[no_mangle]
pub unsafe extern "C" fn ec_write(addr: u16, val: u8) {
    let mut flags: usize = 0;

    spin_lock_irqsave(&raw mut index_access_lock, &mut flags);
    outb((addr & 0xff00) >> 8, EC_IO_PORT_HIGH);
    outb(addr & 0x00ff, EC_IO_PORT_LOW);
    outb(val, EC_IO_PORT_DATA);
    /*  flush the write action */
    let _ = inb(EC_IO_PORT_DATA);
    spin_unlock_irqrestore(&raw mut index_access_lock, flags);
}

/*
 * This function is used for EC command writes and corresponding status queries.
 */
#[no_mangle]
pub unsafe extern "C" fn ec_query_seq(cmd: u8) -> i32 {
    let mut timeout: i32;
    let mut status: u8;
    let mut flags: usize = 0;
    let mut ret: i32 = 0;

    spin_lock_irqsave(&raw mut port_access_lock, &mut flags);

    /* make chip goto reset mode */
    udelay(EC_REG_DELAY);
    outb(cmd, EC_CMD_PORT);
    udelay(EC_REG_DELAY);

    /* check if the command is received by ec */
    timeout = EC_CMD_TIMEOUT;
    status = inb(EC_STS_PORT);
    while { timeout -= 1; timeout >= 0 } && (status & (1 << 1)) != 0 {
        status = inb(EC_STS_PORT);
        udelay(EC_REG_DELAY);
    }

    spin_unlock_irqrestore(&raw mut port_access_lock, flags);

    if timeout <= 0 {
        printk(EC_QUERY_TIMEOUT_MSG.as_ptr() as *const core::ffi::c_char,
            b"ec_query_seq\0".as_ptr(),);
        ret = -22; // -EINVAL
    } else {
        printk(EC_QUERY_STATUS_MSG.as_ptr() as *const core::ffi::c_char,
            timeout, EC_CMD_TIMEOUT - timeout, cmd, status);
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn ec_query_event_num() -> i32 {
    ec_query_seq(CMD_GET_EVENT_NUM)
}

/*
 * Get event number from EC
 *
 * NOTE: This routine must follow the query_event_num function in the
 * interrupt.
 */
#[no_mangle]
pub unsafe extern "C" fn ec_get_event_num() -> i32 {
    let mut timeout: i32 = 100;
    let value: u8;
    let mut status: u8;

    udelay(EC_REG_DELAY);
    status = inb(EC_STS_PORT);
    udelay(EC_REG_DELAY);
    while { timeout -= 1; timeout >= 0 } && (status & (1 << 0)) == 0 {
        status = inb(EC_STS_PORT);
        udelay(EC_REG_DELAY);
    }
    if timeout <= 0 {
        pr_info(EC_EVENT_TIMEOUT_MSG.as_ptr() as *const core::ffi::c_char,
            b"ec_get_event_num\0".as_ptr());
        return -22; // -EINVAL
    }
    value = inb(EC_DAT_PORT);
    udelay(EC_REG_DELAY);

    value as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
