// SPDX-License-Identifier: GPL-2.0
/*
 * General MIPS MT support routines, usable in AP/SP and SMVP.
 * Copyright (C) 2005 Mips Technologies, Inc
 */

// C dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_char;

extern "C" {
    fn get_option(input: *mut *mut c_char, integer: *mut i32) -> i32;
    fn read_c0_config7() -> u32;
    fn write_c0_config7(value: u32);
    fn ehb();
    fn printk(format: *const c_char, ...);
    fn read_c0_errctl() -> usize;
    fn write_c0_errctl(value: usize);
    fn cache_op(operation: i32, address: usize);
    fn read_c0_dtaglo() -> usize;
    fn write_c0_dtaglo(value: usize);
    fn class_register(class: *const Class) -> i32;
}

#[repr(C)]
pub struct Class {
    pub name: *const c_char,
}

pub static mut vpelimit: i32 = 0;

unsafe extern "C" fn maxvpes(mut str_: *mut c_char) -> i32 {
    get_option(&mut str_, &mut vpelimit);
    1
}

// __setup("maxvpes=", maxvpes);

pub static mut tclimit: i32 = 0;

unsafe extern "C" fn maxtcs(mut str_: *mut c_char) -> i32 {
    get_option(&mut str_, &mut tclimit);
    1
}

// __setup("maxtcs=", maxtcs);

static mut mt_opt_rpsctl: i32 = -1;
static mut mt_opt_nblsu: i32 = -1;
static mut mt_opt_forceconfig7: i32 = 0;
static mut mt_opt_config7: i32 = -1;

unsafe extern "C" fn rpsctl_set(mut str_: *mut c_char) -> i32 {
    get_option(&mut str_, &mut mt_opt_rpsctl);
    1
}
// __setup("rpsctl=", rpsctl_set);

unsafe extern "C" fn nblsu_set(mut str_: *mut c_char) -> i32 {
    get_option(&mut str_, &mut mt_opt_nblsu);
    1
}
// __setup("nblsu=", nblsu_set);

unsafe extern "C" fn config7_set(mut str_: *mut c_char) -> i32 {
    get_option(&mut str_, &mut mt_opt_config7);
    mt_opt_forceconfig7 = 1;
    1
}
// __setup("config7=", config7_set);

static mut itc_base: u32 = 0;

unsafe extern "C" fn set_itc_base(mut str_: *mut c_char) -> i32 {
    get_option(&mut str_, &mut itc_base as *mut u32 as *mut i32);
    1
}

// __setup("itcbase=", set_itc_base);

pub unsafe extern "C" fn mips_mt_set_cpuoptions() {
    let oconfig7 = read_c0_config7();
    let mut nconfig7 = oconfig7;

    if mt_opt_rpsctl >= 0 {
        if mt_opt_rpsctl != 0 {
            nconfig7 |= 1u32 << 2;
        } else {
            nconfig7 &= !(1u32 << 2);
        }
    }
    if mt_opt_nblsu >= 0 {
        if mt_opt_nblsu != 0 {
            nconfig7 |= 1u32 << 5;
        } else {
            nconfig7 &= !(1u32 << 5);
        }
    }
    if mt_opt_forceconfig7 != 0 {
        nconfig7 = mt_opt_config7 as u32;
    }
    if oconfig7 != nconfig7 {
        core::arch::asm!("sync");
        write_c0_config7(nconfig7);
        ehb();
        printk(b"Config7: 0x%08x\n\0".as_ptr() as *const c_char, read_c0_config7());
    }

    if itc_base != 0 {
        /*
         * Configure ITC mapping.  This code is very
         * specific to the 34K core family, which uses
         * a special mode bit ("ITC") in the ErrCtl
         * register to enable access to ITC control
         * registers via cache "tag" operations.
         */
        let ectlval = read_c0_errctl();
        let mut itcblkgrn: usize;

        write_c0_errctl(ectlval | (0x1usize << 26));
        ehb();
        const INDEX_0: usize = 0x80000000;
        const INDEX_8: usize = 0x80000008;
        // Read "cache tag" for Dcache pseudo-index 8
        cache_op(0, INDEX_8); // Index_Load_Tag_D
        ehb();
        itcblkgrn = read_c0_dtaglo();
        itcblkgrn &= 0xfffe0000;
        // Set for 128 byte pitch of ITC cells
        itcblkgrn |= 0x00000c00;
        // Stage in Tag register
        write_c0_dtaglo(itcblkgrn);
        ehb();
        // Write out to ITU with CACHE op
        cache_op(1, INDEX_8); // Index_Store_Tag_D
        // Now set base address, and turn ITC on with 0x1 bit
        write_c0_dtaglo(((itc_base as usize) & 0xfffffc00) | 0x1);
        ehb();
        // Write out to ITU with CACHE op
        cache_op(1, INDEX_0); // Index_Store_Tag_D
        write_c0_errctl(ectlval);
        ehb();
    }
}

pub static mt_class: Class = Class {
    name: b"mt\0".as_ptr() as *const c_char,
};

unsafe extern "C" fn mips_mt_init() -> i32 {
    class_register(&mt_class)
}

// subsys_initcall(mips_mt_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
