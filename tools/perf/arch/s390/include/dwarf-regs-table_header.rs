// SPDX-License-Identifier: GPL-2.0

/*
 * For reference, see DWARF register mapping:
 * http://refspecs.linuxfoundation.org/ELF/zSeries/lzsabi0_s390/x1542.html
 */
pub static s390_dwarf_regs: [&str; 66] = [
    "%r0", "%r1", "%r2", "%r3", "%r4", "%r5", "%r6", "%r7",
    "%r8", "%r9", "%r10", "%r11", "%r12", "%r13", "%r14", "%r15",
    "%f0",
    "%f2",
    "%f4",
    "%f6",
    "%f1",
    "%f3",
    "%f5",
    "%f7",
    "%f8",
    "%f10",
    "%f12",
    "%f14",
    "%f9",
    "%f11",
    "%f13",
    "%f15",
    "%c0",
    "%c1",
    "%c2",
    "%c3",
    "%c4",
    "%c5",
    "%c6",
    "%c7",
    "%c8",
    "%c9",
    "%c10",
    "%c11",
    "%c12",
    "%c13",
    "%c14",
    "%c15",
    "%a0",
    "%a1",
    "%a2",
    "%a3",
    "%a4",
    "%a5",
    "%a6",
    "%a7",
    "%a8",
    "%a9",
    "%a10",
    "%a11",
    "%a12",
    "%a13",
    "%a14",
    "%a15",
    "%pswm",
    "%pswa",
];

// If DEFINE_DWARF_REGSTR_TABLE is defined by the includer:
// #define s390_regstr_tbl s390_dwarf_regs

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
