// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * spu_restore.c
 *
 * (C) Copyright IBM Corp. 2005
 *
 * SPU-side context restore sequence outlined in
 * Synergistic Processor Element Book IV
 *
 * Author: Mark Nutter <mnutter@us.ibm.com>
 */

pub type u32 = std::ffi::c_uint;
pub type u64 = std::ffi::c_ulonglong;

pub const LS_SIZE: u32 = 0x40000;
pub const BR_INSTR: u32 = 0x327fff80;
pub const NOP_INSTR: u32 = 0x40200000;
pub const HEQ_INSTR: u32 = 0x7b000000;
pub const STOP_INSTR: u32 = 0x00000000;
pub const ILLEGAL_INSTR: u32 = 0x00800000;
pub const RESTORE_COMPLETE: u32 = 0x00003ffc;

#[repr(C)]
pub union addr64 {
    pub ui: [u32; 2],
    pub ull: u64,
}

#[repr(C)]
pub union spill_quad {
    pub slot: [u32; 4],
    pub v: [u32; 4],
}

extern "C" {
    static mut regs_spill: [spill_quad; 65536];
    static mut dma_list: [u32; 65536];
    fn spu_writech(channel: u32, value: u32);
    fn spu_readch(channel: u32) -> u32;
    fn spu_mtfpscr(value: [u32; 4]);
    fn spu_sync();
    fn set_event_mask();
    fn set_tag_mask();
    fn build_dma_list(ea: addr64);
    fn enqueue_putllc(ea: addr64);
    fn set_tag_update();
    fn read_tag_status();
    fn read_llar_status();
    fn exit_fini();
}

extern "C" {
    static MFC_LSA: u32;
    static MFC_EAH: u32;
    static MFC_EAL: u32;
    static MFC_Size: u32;
    static MFC_TagID: u32;
    static MFC_Cmd: u32;
    static SPU_WrDec: u32;
    static SPU_WrOutMbox: u32;
    static SPU_WrOutIntrMbox: u32;
    static SPU_WrSRR0: u32;
    static SPU_WrEventMask: u32;
    static MFC_WrTagMask: u32;
    static SPU_RdSigNotify1: u32;
    static SPU_RdSigNotify2: u32;
    static SPU_DECR_STATUS_RUNNING: u32;
    static SPU_STOPPED_STATUS_P_I: u32;
    static SPU_STOPPED_STATUS_P_H: u32;
    static SPU_STOPPED_STATUS_S_P: u32;
    static SPU_STOPPED_STATUS_S_I: u32;
    static SPU_STOPPED_STATUS_I: u32;
    static SPU_STOPPED_STATUS_S: u32;
    static SPU_STOPPED_STATUS_H: u32;
    static SPU_STOPPED_STATUS_P: u32;
    static SPU_STOPPED_STATUS_R: u32;
}

extern "C" {
    fn LSCSA_QW_OFFSET(field: u32) -> u32;
    static decr_status: u32;
    static decr: u32;
    static ppu_mb: u32;
    static ppuint_mb: u32;
    static fpcr: u32;
    static srr0: u32;
    static event_mask: u32;
    static tag_mask: u32;
    static stopped_status: u32;
}

unsafe fn fetch_regs_from_mem(lscsa_ea: addr64) {
    let ls = (&raw mut regs_spill as *mut spill_quad) as u32;
    let size = std::mem::size_of_val(&regs_spill) as u32;
    spu_writech(MFC_LSA, ls);
    spu_writech(MFC_EAH, lscsa_ea.ui[0]);
    spu_writech(MFC_EAL, lscsa_ea.ui[1]);
    spu_writech(MFC_Size, size);
    spu_writech(MFC_TagID, 0);
    spu_writech(MFC_Cmd, 0x40);
}

unsafe fn restore_upper_240kb(lscsa_ea: addr64) {
    let list = (&raw mut dma_list as *mut u32) as u32;
    spu_writech(MFC_LSA, 16384);
    spu_writech(MFC_EAH, lscsa_ea.ui[0]);
    spu_writech(MFC_EAL, list);
    spu_writech(MFC_Size, std::mem::size_of_val(&dma_list) as u32);
    spu_writech(MFC_TagID, 0);
    spu_writech(MFC_Cmd, 0x44);
}

unsafe fn restore_decr() {
    let mut offset = LSCSA_QW_OFFSET(decr_status);
    let decr_running = regs_spill[offset as usize].slot[0] & SPU_DECR_STATUS_RUNNING;
    if decr_running != 0 {
        offset = LSCSA_QW_OFFSET(decr);
        spu_writech(SPU_WrDec, regs_spill[offset as usize].slot[0]);
    }
}

unsafe fn write_ppu_mb() { spu_writech(SPU_WrOutMbox, regs_spill[LSCSA_QW_OFFSET(ppu_mb) as usize].slot[0]); }
unsafe fn write_ppuint_mb() { spu_writech(SPU_WrOutIntrMbox, regs_spill[LSCSA_QW_OFFSET(ppuint_mb) as usize].slot[0]); }
unsafe fn restore_fpcr() { spu_mtfpscr(regs_spill[LSCSA_QW_OFFSET(fpcr) as usize].v); }
unsafe fn restore_srr0() { spu_writech(SPU_WrSRR0, regs_spill[LSCSA_QW_OFFSET(srr0) as usize].slot[0]); }
unsafe fn restore_event_mask() { spu_writech(SPU_WrEventMask, regs_spill[LSCSA_QW_OFFSET(event_mask) as usize].slot[0]); }
unsafe fn restore_tag_mask() { spu_writech(MFC_WrTagMask, regs_spill[LSCSA_QW_OFFSET(tag_mask) as usize].slot[0]); }

unsafe fn restore_complete() {
    let exit_instrs = exit_fini as *mut u32;
    let offset = LSCSA_QW_OFFSET(stopped_status) as usize;
    let stopped_status_value = regs_spill[offset].slot[0];
    let stopped_code = regs_spill[offset].slot[1];
    match stopped_status_value {
        x if x == SPU_STOPPED_STATUS_P_I => { *exit_instrs.add(0)=RESTORE_COMPLETE; *exit_instrs.add(1)=ILLEGAL_INSTR; *exit_instrs.add(2)=STOP_INSTR|stopped_code; }
        x if x == SPU_STOPPED_STATUS_P_H => { *exit_instrs.add(0)=RESTORE_COMPLETE; *exit_instrs.add(1)=HEQ_INSTR; *exit_instrs.add(2)=STOP_INSTR|stopped_code; }
        x if x == SPU_STOPPED_STATUS_S_P => { *exit_instrs.add(0)=RESTORE_COMPLETE; *exit_instrs.add(1)=STOP_INSTR|stopped_code; *exit_instrs.add(2)=NOP_INSTR; *exit_instrs.add(3)=BR_INSTR; }
        x if x == SPU_STOPPED_STATUS_S_I || x == SPU_STOPPED_STATUS_I => { *exit_instrs.add(0)=RESTORE_COMPLETE; *exit_instrs.add(1)=ILLEGAL_INSTR; *exit_instrs.add(2)=NOP_INSTR; *exit_instrs.add(3)=BR_INSTR; }
        x if x == SPU_STOPPED_STATUS_S || x == SPU_STOPPED_STATUS_H || x == SPU_STOPPED_STATUS_R => { *exit_instrs.add(0)=RESTORE_COMPLETE; *exit_instrs.add(1)=if x == SPU_STOPPED_STATUS_H { HEQ_INSTR } else { NOP_INSTR }; *exit_instrs.add(2)=NOP_INSTR; *exit_instrs.add(3)=BR_INSTR; }
        x if x == SPU_STOPPED_STATUS_P => { *exit_instrs.add(0)=RESTORE_COMPLETE; *exit_instrs.add(1)=STOP_INSTR|stopped_code; }
        _ => {}
    }
    spu_sync();
}

pub unsafe fn main() -> i32 {
    let mut lscsa_ea = addr64 { ui: [0; 2] };
    lscsa_ea.ui[0] = spu_readch(SPU_RdSigNotify1);
    lscsa_ea.ui[1] = spu_readch(SPU_RdSigNotify2);
    fetch_regs_from_mem(lscsa_ea); set_event_mask(); set_tag_mask(); build_dma_list(lscsa_ea);
    restore_upper_240kb(lscsa_ea); enqueue_putllc(lscsa_ea); set_tag_update(); read_tag_status();
    restore_decr(); read_llar_status(); write_ppu_mb(); write_ppuint_mb(); restore_fpcr();
    restore_srr0(); restore_event_mask(); restore_tag_mask(); restore_complete();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
