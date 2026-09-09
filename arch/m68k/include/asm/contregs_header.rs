/* SPDX-License-Identifier: GPL-2.0 */
/*
 * contregs.h: Addresses of registers in the ASI_CONTROL alternate address
 *             space. These are for the mmu's context register, etc.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

/* 3=sun3
   4=sun4 (as in sun4 sysmaint student book)
   c=sun4c (according to davem) */

pub const AC_IDPROM: u32 = 0x00000000;    /* 34  ID PROM, R/O, byte, 32 bytes      */
pub const AC_PAGEMAP: u32 = 0x10000000;   /* 3   Pagemap R/W, long                 */
pub const AC_SEGMAP: u32 = 0x20000000;    /* 3   Segment map, byte                 */
pub const AC_CONTEXT: u32 = 0x30000000;   /* 34c current mmu-context               */
pub const AC_SENABLE: u32 = 0x40000000;   /* 34c system dvma/cache/reset enable reg*/
pub const AC_UDVMA_ENB: u32 = 0x50000000; /* 34  Not used on Sun boards, byte      */
pub const AC_BUS_ERROR: u32 = 0x60000000; /* 34  Not cleared on read, byte.        */
pub const AC_SYNC_ERR: u32 = 0x60000000;  /*  c fault type                         */
pub const AC_SYNC_VA: u32 = 0x60000004;   /*  c fault virtual address              */
pub const AC_ASYNC_ERR: u32 = 0x60000008; /*  c asynchronous fault type            */
pub const AC_ASYNC_VA: u32 = 0x6000000c;  /*  c async fault virtual address        */
pub const AC_LEDS: u32 = 0x70000000;      /* 34  Zero turns on LEDs, byte          */
pub const AC_CACHETAGS: u32 = 0x80000000; /* 34c direct access to the VAC tags     */
pub const AC_CACHEDDATA: u32 = 0x90000000; /* 3 c direct access to the VAC data     */
pub const AC_UDVMA_MAP: u32 = 0xD0000000; /* 4  Not used on Sun boards, byte       */
pub const AC_VME_VECTOR: u32 = 0xE0000000; /* 4  For non-Autovector VME, byte       */
pub const AC_BOOT_SCC: u32 = 0xF0000000;  /* 34  bypass to access Zilog 8530. byte.*/

/* s=Swift, h=Ross_HyperSPARC, v=TI_Viking, t=Tsunami, r=Ross_Cypress        */
pub const AC_M_PCR: u32 = 0x0000;      /* shv Processor Control Reg             */
pub const AC_M_CTPR: u32 = 0x0100;     /* shv Context Table Pointer Reg         */
pub const AC_M_CXR: u32 = 0x0200;      /* shv Context Register                  */
pub const AC_M_SFSR: u32 = 0x0300;     /* shv Synchronous Fault Status Reg      */
pub const AC_M_SFAR: u32 = 0x0400;     /* shv Synchronous Fault Address Reg     */
pub const AC_M_AFSR: u32 = 0x0500;     /*  hv Asynchronous Fault Status Reg     */
pub const AC_M_AFAR: u32 = 0x0600;     /*  hv Asynchronous Fault Address Reg    */
pub const AC_M_RESET: u32 = 0x0700;    /*  hv Reset Reg                         */
pub const AC_M_RPR: u32 = 0x1000;      /*  hv Root Pointer Reg                  */
pub const AC_M_TSUTRCR: u32 = 0x1000;  /* s   TLB Replacement Ctrl Reg          */
pub const AC_M_IAPTP: u32 = 0x1100;    /*  hv Instruction Access PTP            */
pub const AC_M_DAPTP: u32 = 0x1200;    /*  hv Data Access PTP                   */
pub const AC_M_ITR: u32 = 0x1300;      /*  hv Index Tag Register                */
pub const AC_M_TRCR: u32 = 0x1400;     /*  hv TLB Replacement Control Reg       */
pub const AC_M_SFSRX: u32 = 0x1300;    /* s   Synch Fault Status Reg prim       */
pub const AC_M_SFARX: u32 = 0x1400;    /* s   Synch Fault Address Reg prim      */
pub const AC_M_RPR1: u32 = 0x1500;     /*  h  Root Pointer Reg (entry 2)        */
pub const AC_M_IAPTP1: u32 = 0x1600;   /*  h  Instruction Access PTP (entry 2)  */
pub const AC_M_DAPTP1: u32 = 0x1700;   /*  h  Data Access PTP (entry 2)          */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
