// SPDX-License-Identifier: GPL-2.0
/*
 * ip22-nvram.c: NVRAM and serial EEPROM handling.
 *
 * Copyright (C) 2003 Ladislav Michl (ladis@linux-mips.org)
 */

// External declarations supplied by the kernel and SGI IP22 support.
extern "C" {
    fn __raw_readl(addr: *const u32) -> u32;
    fn __raw_writel(value: u32, addr: *mut u32);
    fn ip22_is_fullhouse() -> bool;
    static mut hpc3c0: *mut Hpc3c0;
}

#[repr(C)]
pub struct Hpc3c0 {
    pub eeprom: u32,
    pub bbram: [u8; 8192],
}

/* Control opcode for serial eeprom */
const EEPROM_READ: u32 = 0xc000;
const EEPROM_WEN: u32 = 0x9800;
const EEPROM_WRITE: u32 = 0xa000;
const EEPROM_WRALL: u32 = 0x8800;
const EEPROM_WDS: u32 = 0x8000;
const EEPROM_PRREAD: u32 = 0xc000;
const EEPROM_PREN: u32 = 0x9800;
const EEPROM_PRCLEAR: u32 = 0xffff;
const EEPROM_PRWRITE: u32 = 0xa000;
const EEPROM_PRDS: u32 = 0x8000;

const EEPROM_EPROT: u32 = 0x01;
const EEPROM_CSEL: u32 = 0x02;
const EEPROM_ECLK: u32 = 0x04;
const EEPROM_DATO: u32 = 0x08;
const EEPROM_DATI: u32 = 0x10;

/* We need to use these functions early... */
#[inline(always)]
unsafe fn delay() {
    for _x in 0..100000 {
        core::hint::spin_loop();
    }
}

#[inline(always)]
unsafe fn eeprom_cs_on(ptr: *mut u32) {
    __raw_writel(__raw_readl(ptr) & !EEPROM_DATO, ptr);
    __raw_writel(__raw_readl(ptr) & !EEPROM_ECLK, ptr);
    __raw_writel(__raw_readl(ptr) & !EEPROM_EPROT, ptr);
    delay();
    __raw_writel(__raw_readl(ptr) | EEPROM_CSEL, ptr);
    __raw_writel(__raw_readl(ptr) | EEPROM_ECLK, ptr);
}

#[inline(always)]
unsafe fn eeprom_cs_off(ptr: *mut u32) {
    __raw_writel(__raw_readl(ptr) & !EEPROM_ECLK, ptr);
    __raw_writel(__raw_readl(ptr) & !EEPROM_CSEL, ptr);
    __raw_writel(__raw_readl(ptr) | EEPROM_EPROT, ptr);
    __raw_writel(__raw_readl(ptr) | EEPROM_ECLK, ptr);
}

const BITS_IN_COMMAND: u32 = 11;

/*
 * clock in the nvram command and the register number. For the
 * national semiconductor nv ram chip the op code is 3 bits and
 * the address is 6/8 bits.
 */
#[inline]
unsafe fn eeprom_cmd(ctrl: *mut u32, cmd: u32, reg: u32) {
    let mut ser_cmd: u16 = (cmd | (reg << (16 - BITS_IN_COMMAND))) as u16;
    for _i in 0..BITS_IN_COMMAND {
        if ser_cmd & (1 << 15) != 0 {
            __raw_writel(__raw_readl(ctrl) | EEPROM_DATO, ctrl);
        } else {
            __raw_writel(__raw_readl(ctrl) & !EEPROM_DATO, ctrl);
        }
        __raw_writel(__raw_readl(ctrl) & !EEPROM_ECLK, ctrl);
        delay();
        __raw_writel(__raw_readl(ctrl) | EEPROM_ECLK, ctrl);
        delay();
        ser_cmd <<= 1;
    }
    /* see data sheet timing diagram */
    __raw_writel(__raw_readl(ctrl) & !EEPROM_DATO, ctrl);
}

#[no_mangle]
pub unsafe extern "C" fn ip22_eeprom_read(ctrl: *mut u32, reg: i32) -> u16 {
    let mut res: u16 = 0;

    __raw_writel(__raw_readl(ctrl) & !EEPROM_EPROT, ctrl);
    eeprom_cs_on(ctrl);
    eeprom_cmd(ctrl, EEPROM_READ, reg as u32);

    /* clock the data ouf of serial mem */
    for _i in 0..16 {
        __raw_writel(__raw_readl(ctrl) & !EEPROM_ECLK, ctrl);
        delay();
        __raw_writel(__raw_readl(ctrl) | EEPROM_ECLK, ctrl);
        delay();
        res <<= 1;
        if __raw_readl(ctrl) & EEPROM_DATI != 0 {
            res |= 1;
        }
    }

    eeprom_cs_off(ctrl);
    res
}

#[no_mangle]
pub unsafe extern "C" fn ip22_nvram_read(mut reg: i32) -> u16 {
    if ip22_is_fullhouse() {
        /* IP22 (Indigo2 aka FullHouse) stores env variables into
         * 93CS56 Microwire Bus EEPROM 2048 Bit (128x16) */
        ip22_eeprom_read(core::ptr::addr_of_mut!((*hpc3c0).eeprom), reg)
    } else {
        let hpc = &mut *hpc3c0;
        /* IP24 (Indy aka Guiness) uses DS1386 8K version */
        reg <<= 1;
        let tmp = hpc.bbram[reg as usize] as u16;
        (tmp << 8) | hpc.bbram[(reg + 1) as usize] as u16
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
