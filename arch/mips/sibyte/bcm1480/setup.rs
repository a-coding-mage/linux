// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2000,2001,2002,2003,2004 Broadcom Corporation
 */

// Linux and architecture headers supplying the symbols used below are
// intentionally omitted; they are provided by the surrounding translation.

pub static mut sb1_pass: u32 = 0;
pub static mut soc_pass: u32 = 0;
pub static mut soc_type: u32 = 0;
pub static mut periph_rev: u32 = 0;
pub static mut zbbus_mhz: u32 = 0;

static mut part_type: u32 = 0;

static mut soc_str: *const u8 = core::ptr::null();
static mut pass_str: *const u8 = core::ptr::null();

unsafe fn setup_bcm1x80_bcm1x55() -> i32 {
    match soc_pass {
        K_SYS_REVISION_BCM1480_S0 => {
            periph_rev = 1;
            pass_str = b"S0 (pass1)\0".as_ptr();
        }
        K_SYS_REVISION_BCM1480_A1 => {
            periph_rev = 1;
            pass_str = b"A1 (pass1)\0".as_ptr();
        }
        K_SYS_REVISION_BCM1480_A2 => {
            periph_rev = 1;
            pass_str = b"A2 (pass1)\0".as_ptr();
        }
        K_SYS_REVISION_BCM1480_A3 => {
            periph_rev = 1;
            pass_str = b"A3 (pass1)\0".as_ptr();
        }
        K_SYS_REVISION_BCM1480_B0 => {
            periph_rev = 1;
            pass_str = b"B0 (pass2)\0".as_ptr();
        }
        _ => {
            printk(b"Unknown %s rev %x\n\0".as_ptr(), soc_str, soc_pass);
            periph_rev = 1;
            pass_str = b"Unknown Revision\0".as_ptr();
        }
    }

    0
}

/* Setup code likely to be common to all SiByte platforms */

unsafe fn sys_rev_decode() -> i32 {
    let mut ret: i32 = 0;

    match soc_type {
        K_SYS_SOC_TYPE_BCM1x80 => {
            if part_type == K_SYS_PART_BCM1480 {
                soc_str = b"BCM1480\0".as_ptr();
            } else if part_type == K_SYS_PART_BCM1280 {
                soc_str = b"BCM1280\0".as_ptr();
            } else {
                soc_str = b"BCM1x80\0".as_ptr();
            }
            ret = setup_bcm1x80_bcm1x55();
        }
        K_SYS_SOC_TYPE_BCM1x55 => {
            if part_type == K_SYS_PART_BCM1455 {
                soc_str = b"BCM1455\0".as_ptr();
            } else if part_type == K_SYS_PART_BCM1255 {
                soc_str = b"BCM1255\0".as_ptr();
            } else {
                soc_str = b"BCM1x55\0".as_ptr();
            }
            ret = setup_bcm1x80_bcm1x55();
        }
        _ => {
            printk(b"Unknown part type %x\n\0".as_ptr(), part_type);
            ret = 1;
        }
    }

    ret
}

pub unsafe fn bcm1480_setup() {
    let sys_rev: u64;
    let plldiv: i32;

    sb1_pass = read_c0_prid() & PRID_REV_MASK;
    sys_rev = __raw_readq(IOADDR(A_SCD_SYSTEM_REVISION));
    soc_type = SYS_SOC_TYPE(sys_rev);
    part_type = G_SYS_PART(sys_rev);
    soc_pass = G_SYS_REVISION(sys_rev);

    if sys_rev_decode() != 0 {
        printk(b"Restart after failure to identify SiByte chip\n\0".as_ptr());
        machine_restart(core::ptr::null());
    }

    plldiv = G_BCM1480_SYS_PLL_DIV(__raw_readq(IOADDR(A_SCD_SYSTEM_CFG))) as i32;
    zbbus_mhz = (((plldiv >> 1) * 50) + ((plldiv & 1) * 25)) as u32;

    printk(
        b"Broadcom SiByte %s %s @ %d MHz (SB-1A rev %d)\n\0".as_ptr(),
        soc_str,
        pass_str,
        zbbus_mhz * 2,
        sb1_pass,
    );
    printk(b"Board type: %s\n\0".as_ptr(), get_system_type());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
