// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * spu_save.c
 *
 * (C) Copyright IBM Corp. 2005
 *
 * SPU-side context save sequence outlined in
 * Synergistic Processor Element Book IV
 *
 * Author: Mark Nutter <mnutter@us.ibm.com>
 */

// LS_SIZE is supplied by the build when configured; the source default is
// 0x40000 (256K, in bytes).
pub const LS_SIZE: usize = 0x40000;

pub type u32 = ::core::ffi::c_uint;
pub type u64 = ::core::ffi::c_ulonglong;

// Types, channels, globals, and helper routines below are supplied by the
// SPU runtime and CSA support headers.

#[inline]
unsafe fn save_event_mask() {
    let offset: u32;

    /* Save, Step 2:
     *    Read the SPU_RdEventMsk channel and save to the LSCSA.
     */
    offset = LSCSA_QW_OFFSET(event_mask);
    regs_spill[offset as usize].slot[0] = spu_readch(SPU_RdEventMask);
}

#[inline]
unsafe fn save_tag_mask() {
    let offset: u32;

    /* Save, Step 3:
     *    Read the SPU_RdTagMsk channel and save to the LSCSA.
     */
    offset = LSCSA_QW_OFFSET(tag_mask);
    regs_spill[offset as usize].slot[0] = spu_readch(MFC_RdTagMask);
}

#[inline]
unsafe fn save_upper_240kb(lscsa_ea: addr64) {
    let ls: u32 = 16384;
    let list: u32 = (&raw const dma_list as *const _ as usize) as u32;
    let size: u32 = core::mem::size_of_val(&dma_list) as u32;
    let tag_id: u32 = 0;
    let cmd: u32 = 0x24; // PUTL

    /* Save, Step 7:
     *    Enqueue the PUTL command (tag 0) to the MFC SPU command
     *    queue to transfer the remaining 240 kb of LS to CSA.
     */
    spu_writech(MFC_LSA, ls);
    spu_writech(MFC_EAH, lscsa_ea.ui[0]);
    spu_writech(MFC_EAL, list);
    spu_writech(MFC_Size, size);
    spu_writech(MFC_TagID, tag_id);
    spu_writech(MFC_Cmd, cmd);
}

#[inline]
unsafe fn save_fpcr() {
    // vector unsigned int fpcr;
    let offset: u32;

    /* Save, Step 9:
     *    Issue the floating-point status and control register
     *    read instruction, and save to the LSCSA.
     */
    offset = LSCSA_QW_OFFSET(fpcr);
    regs_spill[offset as usize].v = spu_mffpscr();
}

#[inline]
unsafe fn save_decr() {
    let offset: u32;
    /* Save, Step 10:
     *    Read and save the SPU_RdDec channel data to
     *    the LSCSA.
     */
    offset = LSCSA_QW_OFFSET(decr);
    regs_spill[offset as usize].slot[0] = spu_readch(SPU_RdDec);
}

#[inline]
unsafe fn save_srr0() {
    let offset: u32;
    /* Save, Step 11:
     *    Read and save the SPU_WSRR0 channel data to
     *    the LSCSA.
     */
    offset = LSCSA_QW_OFFSET(srr0);
    regs_spill[offset as usize].slot[0] = spu_readch(SPU_RdSRR0);
}

#[inline]
unsafe fn spill_regs_to_mem(lscsa_ea: addr64) {
    let ls: u32 = (&raw const regs_spill as *const _ as usize) as u32;
    let size: u32 = core::mem::size_of_val(&regs_spill) as u32;
    let tag_id: u32 = 0;
    let cmd: u32 = 0x20; // PUT

    /* Save, Step 13:
     *    Enqueue a PUT command (tag 0) to send the LSCSA
     *    to the CSA.
     */
    spu_writech(MFC_LSA, ls);
    spu_writech(MFC_EAH, lscsa_ea.ui[0]);
    spu_writech(MFC_EAL, lscsa_ea.ui[1]);
    spu_writech(MFC_Size, size);
    spu_writech(MFC_TagID, tag_id);
    spu_writech(MFC_Cmd, cmd);
}

#[inline]
unsafe fn enqueue_sync(_lscsa_ea: addr64) {
    let tag_id: u32 = 0;
    let cmd: u32 = 0xCC;
    /* Save, Step 14:
     *    Enqueue an MFC_SYNC command (tag 0).
     */
    spu_writech(MFC_TagID, tag_id);
    spu_writech(MFC_Cmd, cmd);
}

#[inline]
unsafe fn save_complete() {
    /* Save, Step 18:
     *    Issue a stop-and-signal instruction indicating
     *    "save complete".  Note: This function will not
     *    return!!
     */
    spu_stop(SPU_SAVE_COMPLETE);
}

/**
 * main - entry point for SPU-side context save.
 *
 * This code deviates from the documented sequence as follows:
 *
 *      1. The EA for LSCSA is passed from PPE in the
 *         signal notification channels.
 *      2. All 128 registers are saved by crt0.o.
 */
pub unsafe fn main() -> i32 {
    let mut lscsa_ea: addr64 = core::mem::zeroed();

    lscsa_ea.ui[0] = spu_readch(SPU_RdSigNotify1);
    lscsa_ea.ui[1] = spu_readch(SPU_RdSigNotify2);

    /* Step 1: done by exit(). */
    save_event_mask(); // Step 2.
    save_tag_mask(); // Step 3.
    set_event_mask(); // Step 4.
    set_tag_mask(); // Step 5.
    build_dma_list(lscsa_ea); // Step 6.
    save_upper_240kb(lscsa_ea); // Step 7.
    /* Step 8: done by exit(). */
    save_fpcr(); // Step 9.
    save_decr(); // Step 10.
    save_srr0(); // Step 11.
    enqueue_putllc(lscsa_ea); // Step 12.
    spill_regs_to_mem(lscsa_ea); // Step 13.
    enqueue_sync(lscsa_ea); // Step 14.
    set_tag_update(); // Step 15.
    read_tag_status(); // Step 16.
    read_llar_status(); // Step 17.
    save_complete(); // Step 18.

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
