/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ecc.h: Definitions and defines for the external cache/memory
 *        controller on the sun4m.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

/* These registers are accessed through the SRMMU passthrough ASI 0x20 */
pub const ECC_ENABLE: u32 = 0x00000000; // ECC enable register
pub const ECC_FSTATUS: u32 = 0x00000008; // ECC fault status register
pub const ECC_FADDR: u32 = 0x00000010; // ECC fault address register
pub const ECC_DIGNOSTIC: u32 = 0x00000018; // ECC diagnostics register
pub const ECC_MBAENAB: u32 = 0x00000020; // MBus arbiter enable register
pub const ECC_DMESG: u32 = 0x00001000; // Diagnostic message passing area

/* ECC MBus Arbiter Enable register:
 *
 * ----------------------------------------
 * |              |SBUS|MOD3|MOD2|MOD1|RSV|
 * ----------------------------------------
 *  31           5   4   3    2    1    0
 *
 * SBUS: Enable MBus Arbiter on the SBus 0=off 1=on
 * MOD3: Enable MBus Arbiter on MBus module 3  0=off 1=on
 * MOD2: Enable MBus Arbiter on MBus module 2  0=off 1=on
 * MOD1: Enable MBus Arbiter on MBus module 1  0=off 1=on
 */
pub const ECC_MBAE_SBUS: u32 = 0x00000010;
pub const ECC_MBAE_MOD3: u32 = 0x00000008;
pub const ECC_MBAE_MOD2: u32 = 0x00000004;
pub const ECC_MBAE_MOD1: u32 = 0x00000002;

/* ECC Fault Control Register layout:
 *
 * -----------------------------
 * |    RESV   | ECHECK | EINT |
 * -----------------------------
 *  31        2     1       0
 *
 * ECHECK:  Enable ECC checking.  0=off 1=on
 * EINT:  Enable Interrupts for correctable errors. 0=off 1=on
 */
pub const ECC_FCR_CHECK: u32 = 0x00000002;
pub const ECC_FCR_INTENAB: u32 = 0x00000001;

/* ECC Fault Address Register Zero layout:
 *
 * -----------------------------------------------------
 * | MID | S | RSV |  VA   | BM |AT| C| SZ |TYP| PADDR |
 * -----------------------------------------------------
 *  31-28  27 26-22  21-14   13  12 11 10-8 7-4   3-0
 *
 * MID: ModuleID of the faulting processor. ie. who did it?
 * S: Supervisor/Privileged access? 0=no 1=yes
 * VA: Bits 19-12 of the virtual faulting address, these are the
 *     superset bits in the virtual cache and can be used for
 *     a flush operation if necessary.
 * BM: Boot mode? 0=no 1=yes  This is just like the SRMMU boot
 *     mode bit.
 * AT: Did this fault happen during an atomic instruction? 0=no
 *     1=yes.  This means either an 'ldstub' or 'swap' instruction
 *     was in progress (but not finished) when this fault happened.
 *     This indicated whether the bus was locked when the fault
 *     occurred.
 * C: Did the pte for this access indicate that it was cacheable?
 *    0=no 1=yes
 * SZ: The size of the transaction.
 * TYP: The transaction type.
 * PADDR: Bits 35-32 of the physical address for the fault.
 */
pub const ECC_FADDR0_MIDMASK: u32 = 0xf0000000;
pub const ECC_FADDR0_S: u32 = 0x08000000;
pub const ECC_FADDR0_VADDR: u32 = 0x003fc000;
pub const ECC_FADDR0_BMODE: u32 = 0x00002000;
pub const ECC_FADDR0_ATOMIC: u32 = 0x00001000;
pub const ECC_FADDR0_CACHE: u32 = 0x00000800;
pub const ECC_FADDR0_SIZE: u32 = 0x00000700;
pub const ECC_FADDR0_TYPE: u32 = 0x000000f0;
pub const ECC_FADDR0_PADDR: u32 = 0x0000000f;

/* ECC Fault Address Register One layout:
 *
 * -------------------------------------
 * |          Physical Address 31-0    |
 * -------------------------------------
 *  31                               0
 *
 * You get the upper 4 bits of the physical address from the
 * PADDR field in ECC Fault Address Zero register.
 */

/* ECC Fault Status Register layout:
 *
 * ----------------------------------------------
 * | RESV|C2E|MULT|SYNDROME|DWORD|UNC|TIMEO|BS|C|
 * ----------------------------------------------
 *  31-18  17  16    15-8    7-4   3    2    1 0
 *
 * C2E: A C2 graphics error occurred. 0=no 1=yes (SS10 only)
 * MULT: Multiple errors occurred ;-O 0=no 1=prom_panic(yes)
 * SYNDROME: Controller is mentally unstable.
 * DWORD:
 * UNC: Uncorrectable error.  0=no 1=yes
 * TIMEO: Timeout occurred. 0=no 1=yes
 * BS: C2 graphics bad slot access. 0=no 1=yes (SS10 only)
 * C: Correctable error? 0=no 1=yes
 */
pub const ECC_FSR_C2ERR: u32 = 0x00020000;
pub const ECC_FSR_MULT: u32 = 0x00010000;
pub const ECC_FSR_SYND: u32 = 0x0000ff00;
pub const ECC_FSR_DWORD: u32 = 0x000000f0;
pub const ECC_FSR_UNC: u32 = 0x00000008;
pub const ECC_FSR_TIMEO: u32 = 0x00000004;
pub const ECC_FSR_BADSLOT: u32 = 0x00000002;
pub const ECC_FSR_C: u32 = 0x00000001;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
