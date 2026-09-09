/*
 * Interface for smsc fdc48m81x Super IO chip
 *
 * Author: MontaVista Software, Inc. source@mvista.com
 *
 * 2001-2003 (c) MontaVista Software, Inc. This file is licensed under
 * the terms of the GNU General Public License version 2. This program is
 * licensed "as is" without any warranty of any kind, whether express
 * or implied.
 *
 * Copyright 2004 (c) MontaVista Software, Inc.
 */

/* Common Registers */
const SMSC_FDC37M81X_CONFIG_INDEX: u8 = 0x00;
const SMSC_FDC37M81X_CONFIG_DATA: u8 = 0x01;
const SMSC_FDC37M81X_CONF: u8 = 0x02;
const SMSC_FDC37M81X_INDEX: u8 = 0x03;
const SMSC_FDC37M81X_DNUM: u8 = 0x07;
const SMSC_FDC37M81X_DID: u8 = 0x20;
const SMSC_FDC37M81X_DREV: u8 = 0x21;
const SMSC_FDC37M81X_PCNT: u8 = 0x22;
const SMSC_FDC37M81X_PMGT: u8 = 0x23;
const SMSC_FDC37M81X_OSC: u8 = 0x24;
const SMSC_FDC37M81X_CONFPA0: u8 = 0x26;
const SMSC_FDC37M81X_CONFPA1: u8 = 0x27;
const SMSC_FDC37M81X_TEST4: u8 = 0x2B;
const SMSC_FDC37M81X_TEST5: u8 = 0x2C;
const SMSC_FDC37M81X_TEST1: u8 = 0x2D;
const SMSC_FDC37M81X_TEST2: u8 = 0x2E;
const SMSC_FDC37M81X_TEST3: u8 = 0x2F;

/* Logical device numbers */
const SMSC_FDC37M81X_FDD: u8 = 0x00;
const SMSC_FDC37M81X_SERIAL1: u8 = 0x04;
const SMSC_FDC37M81X_SERIAL2: u8 = 0x05;
const SMSC_FDC37M81X_KBD: u8 = 0x07;

/* Logical device Config Registers */
const SMSC_FDC37M81X_ACTIVE: u8 = 0x30;
const SMSC_FDC37M81X_BASEADDR0: u8 = 0x60;
const SMSC_FDC37M81X_BASEADDR1: u8 = 0x61;
const SMSC_FDC37M81X_INT: u8 = 0x70;
const SMSC_FDC37M81X_INT2: u8 = 0x72;
const SMSC_FDC37M81X_MODE: u8 = 0xF0;

/* Chip Config Values */
const SMSC_FDC37M81X_CONFIG_ENTER: u8 = 0x55;
const SMSC_FDC37M81X_CONFIG_EXIT: u8 = 0xaa;
const SMSC_FDC37M81X_CHIP_ID: u8 = 0x4d;

static mut g_smsc_fdc37m81x_base: usize = 0;

extern "C" {
    fn outb(value: u8, port: usize);
    fn inb(port: usize) -> u8;
}

#[inline]
unsafe fn smsc_fdc37m81x_rd(index: u8) -> u8 {
    outb(index, g_smsc_fdc37m81x_base + SMSC_FDC37M81X_CONFIG_INDEX as usize);
    inb(g_smsc_fdc37m81x_base + SMSC_FDC37M81X_CONFIG_DATA as usize)
}

#[inline]
unsafe fn smsc_dc37m81x_wr(index: u8, data: u8) {
    outb(index, g_smsc_fdc37m81x_base + SMSC_FDC37M81X_CONFIG_INDEX as usize);
    outb(data, g_smsc_fdc37m81x_base + SMSC_FDC37M81X_CONFIG_DATA as usize);
}

pub unsafe fn smsc_fdc37m81x_config_beg() {
    if g_smsc_fdc37m81x_base != 0 {
        outb(SMSC_FDC37M81X_CONFIG_ENTER,
             g_smsc_fdc37m81x_base + SMSC_FDC37M81X_CONFIG_INDEX as usize);
    }
}

pub unsafe fn smsc_fdc37m81x_config_end() {
    if g_smsc_fdc37m81x_base != 0 {
        outb(SMSC_FDC37M81X_CONFIG_EXIT,
             g_smsc_fdc37m81x_base + SMSC_FDC37M81X_CONFIG_INDEX as usize);
    }
}

pub unsafe fn smsc_fdc37m81x_config_get(reg: u8) -> u8 {
    let mut val: u8 = 0;
    if g_smsc_fdc37m81x_base != 0 {
        val = smsc_fdc37m81x_rd(reg);
    }
    val
}

pub unsafe fn smsc_fdc37m81x_config_set(reg: u8, val: u8) {
    if g_smsc_fdc37m81x_base != 0 {
        smsc_dc37m81x_wr(reg, val);
    }
}

pub unsafe fn smsc_fdc37m81x_init(port: usize) -> usize {
    let field = core::mem::size_of::<usize>() * 2;
    let chip_id: u8;

    if g_smsc_fdc37m81x_base != 0 {
        pr_warn("{}: stepping on old base=0x{:0width$x}\n", "smsc_fdc37m81x_init", g_smsc_fdc37m81x_base, width = field);
    }

    g_smsc_fdc37m81x_base = port;
    smsc_fdc37m81x_config_beg();

    chip_id = smsc_fdc37m81x_rd(SMSC_FDC37M81X_DID);
    if chip_id == SMSC_FDC37M81X_CHIP_ID {
        smsc_fdc37m81x_config_end();
    } else {
        pr_warn("{}: unknown chip id 0x{:02x}\n", "smsc_fdc37m81x_init", chip_id);
        g_smsc_fdc37m81x_base = 0;
    }
    g_smsc_fdc37m81x_base
}

#[cfg(debug_assertions)]
unsafe fn smsc_fdc37m81x_config_dump_one(key: *const core::ffi::c_char, dev: u8, reg: u8) {
    pr_info("{}: dev=0x{:02x} reg=0x{:02x} val=0x{:02x}\n", key, dev, reg,
            smsc_fdc37m81x_rd(reg));
}

#[cfg(debug_assertions)]
pub unsafe fn smsc_fdc37m81x_config_dump() {
    let orig = smsc_fdc37m81x_rd(SMSC_FDC37M81X_DNUM);
    let fname = "smsc_fdc37m81x_config_dump";
    smsc_fdc37m81x_config_beg();
    pr_info("{}: common\n", fname);
    smsc_fdc37m81x_config_dump_one(core::ptr::null(), SMSC_FDC37M81X_NONE, SMSC_FDC37M81X_DNUM);
    smsc_fdc37m81x_config_dump_one(core::ptr::null(), SMSC_FDC37M81X_NONE, SMSC_FDC37M81X_DID);
    smsc_fdc37m81x_config_dump_one(core::ptr::null(), SMSC_FDC37M81X_NONE, SMSC_FDC37M81X_DREV);
    smsc_fdc37m81x_config_dump_one(core::ptr::null(), SMSC_FDC37M81X_NONE, SMSC_FDC37M81X_PCNT);
    smsc_fdc37m81x_config_dump_one(core::ptr::null(), SMSC_FDC37M81X_NONE, SMSC_FDC37M81X_PMGT);
    pr_info("{}: keyboard\n", fname);
    smsc_dc37m81x_wr(SMSC_FDC37M81X_DNUM, SMSC_FDC37M81X_KBD);
    smsc_fdc37m81x_config_dump_one(core::ptr::null(), SMSC_FDC37M81X_KBD, SMSC_FDC37M81X_ACTIVE);
    smsc_fdc37m81x_config_dump_one(core::ptr::null(), SMSC_FDC37M81X_KBD, SMSC_FDC37M81X_INT);
    smsc_fdc37m81x_config_dump_one(core::ptr::null(), SMSC_FDC37M81X_KBD, SMSC_FDC37M81X_INT2);
    smsc_fdc37m81x_config_dump_one(core::ptr::null(), SMSC_FDC37M81X_KBD, SMSC_FDC37M81X_LDCR_F0);
    smsc_dc37m81x_wr(SMSC_FDC37M81X_DNUM, orig);
    smsc_fdc37m81x_config_end();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
