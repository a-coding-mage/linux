// SPDX-License-Identifier: GPL-2.0
/*
 * dwarf-regs-table.h : Mapping of DWARF debug register numbers into
 * register names.
 *
 * Copyright (C) 2020-2023 Loongson Technology Corporation Limited
 */

use core::ffi::CStr;

// C source condition: #ifdef DEFINE_DWARF_REGSTR_TABLE
static loongarch_regstr_tbl: [&CStr; 32] = [
    c"%r0", c"%r1", c"%r2", c"%r3", c"%r4", c"%r5", c"%r6", c"%r7",
    c"%r8", c"%r9", c"%r10", c"%r11", c"%r12", c"%r13", c"%r14", c"%r15",
    c"%r16", c"%r17", c"%r18", c"%r19", c"%r20", c"%r21", c"%r22", c"%r23",
    c"%r24", c"%r25", c"%r26", c"%r27", c"%r28", c"%r29", c"%r30", c"%r31",
];
// C source condition end: #endif

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
