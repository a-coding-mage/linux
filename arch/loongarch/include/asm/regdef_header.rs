/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// LoongArch register-name substitutions from the original assembly macros.
pub const zero: &str = "$r0"; // wired zero
pub const ra: &str = "$r1"; // return address
pub const tp: &str = "$r2";
pub const sp: &str = "$r3"; // stack pointer
pub const a0: &str = "$r4"; // argument registers, a0/a1 reused as v0/v1 for return value
pub const a1: &str = "$r5";
pub const a2: &str = "$r6";
pub const a3: &str = "$r7";
pub const a4: &str = "$r8";
pub const a5: &str = "$r9";
pub const a6: &str = "$r10";
pub const a7: &str = "$r11";
pub const t0: &str = "$r12"; // caller saved
pub const t1: &str = "$r13";
pub const t2: &str = "$r14";
pub const t3: &str = "$r15";
pub const t4: &str = "$r16";
pub const t5: &str = "$r17";
pub const t6: &str = "$r18";
pub const t7: &str = "$r19";
pub const t8: &str = "$r20";
pub const u0: &str = "$r21";
pub const fp: &str = "$r22"; // frame pointer
pub const s0: &str = "$r23"; // callee saved
pub const s1: &str = "$r24";
pub const s2: &str = "$r25";
pub const s3: &str = "$r26";
pub const s4: &str = "$r27";
pub const s5: &str = "$r28";
pub const s6: &str = "$r29";
pub const s7: &str = "$r30";
pub const s8: &str = "$r31";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
