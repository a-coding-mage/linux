/*
 * Setup the right wbflush routine for the different DECstations.
 *
 * Created with information from:
 *	DECstation 3100 Desktop Workstation Functional Specification
 *	DECstation 5000/200 KN02 System Module Functional Specification
 *	mipsel-linux-objdump --disassemble vmunix | grep "wbflush" :-)
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1998 Harald Koerfgen
 * Copyright (C) 2002 Maciej W. Rozycki
 */

// Dependencies supplied by the surrounding kernel build.
unsafe extern "C" {
    static mut mips_machtype: i32;
    fn __fast_iob();
}

static mut __wbflush: Option<unsafe extern "C" fn()> = None;

unsafe extern "C" fn wbflush_setup() {
    match mips_machtype {
        MACH_DS23100 | MACH_DS5000_200 => {
            __wbflush = Some(wbflush_kn01);
        }
        MACH_DS5100 => {
            __wbflush = Some(wbflush_kn210);
        }
        MACH_DS5000_1XX | MACH_DS5000_XX | MACH_DS5000_2X0 | MACH_DS5900 => {
            __wbflush = Some(wbflush_mips);
        }
        _ => {
            __wbflush = Some(wbflush_mips);
        }
    }
}

/*
 * For the DS3100 and DS5000/200 the R2020/R3220 writeback buffer functions
 * as part of Coprocessor 0.
 */
unsafe extern "C" fn wbflush_kn01() {
    core::arch::asm!(
        ".set\tpush\n\t",
        ".set\tnoreorder\n\t",
        "1:\tbc0f\t1b\n\t",
        "nop\n\t",
        ".set\tpop",
    );
}

/*
 * For the DS5100 the writeback buffer seems to be a part of Coprocessor 3.
 * But CP3 has to enabled first.
 */
unsafe extern "C" fn wbflush_kn210() {
    core::arch::asm!(
        ".set\tpush\n\t",
        ".set\tnoreorder\n\t",
        "mfc0\t$2,$12\n\t",
        "lui\t$3,0x8000\n\t",
        "or\t$3,$2,$3\n\t",
        "mtc0\t$3,$12\n\t",
        "nop\n",
        "1:\tbc3f\t1b\n\t",
        "nop\n\t",
        "mtc0\t$2,$12\n\t",
        "nop\n\t",
        ".set\tpop",
        lateout("$2") _,
        lateout("$3") _,
    );
}

/*
 * I/O ASIC systems use a standard writeback buffer that gets flushed
 * upon an uncached read.
 */
unsafe extern "C" fn wbflush_mips() {
    __fast_iob();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
