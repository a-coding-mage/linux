/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*----------------------------------------
  PERFORMANCE INSTRUMENTATION
  Guillaume Thouvenin           08/10/98
  David S. Miller               10/06/98
  ---------------------------------------*/

/* sys_perfctr() interface.  First arg is operation code
 * from enumeration below.  The meaning of further arguments
 * are determined by the operation code.
 *
 * NOTE: This system call is no longer provided, use the perf_events
 *       infrastructure.
 *
 * Pointers which are passed by the user are pointers to 64-bit
 * integers.
 *
 * Once enabled, performance counter state is retained until the
 * process either exits or performs an exec.  That is, performance
 * counters remain enabled for fork/clone children.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum perfctr_opcode {
    /* Enable UltraSparc performance counters, ARG0 is pointer
     * to 64-bit accumulator for D0 counter in PIC, ARG1 is pointer
     * to 64-bit accumulator for D1 counter.  ARG2 is a pointer to
     * the initial PCR register value to use.
     */
    PERFCTR_ON,

    /* Disable UltraSparc performance counters.  The PCR is written
     * with zero and the user counter accumulator pointers and
     * working PCR register value are forgotten.
     */
    PERFCTR_OFF,

    /* Add current D0 and D1 PIC values into user pointers given
     * in PERFCTR_ON operation.  The PIC is cleared before returning.
     */
    PERFCTR_READ,

    /* Clear the PIC register. */
    PERFCTR_CLRPIC,

    /* Begin using a new PCR value, the pointer to which is passed
     * in ARG0.  The PIC is also cleared after the new PCR value is
     * written.
     */
    PERFCTR_SETPCR,

    /* Store in pointer given in ARG0 the current PCR register value
     * being used.
     */
    PERFCTR_GETPCR,
}

pub const PRIV: u32 = 0x00000001;
pub const SYS: u32 = 0x00000002;
pub const USR: u32 = 0x00000004;

/* Pic.S0 Selection Bit Field Encoding, Ultra-I/II */
pub const CYCLE_CNT: u32 = 0x00000000;
pub const INSTR_CNT: u32 = 0x00000010;
pub const DISPATCH0_IC_MISS: u32 = 0x00000020;
pub const DISPATCH0_STOREBUF: u32 = 0x00000030;
pub const IC_REF: u32 = 0x00000080;
pub const DC_RD: u32 = 0x00000090;
pub const DC_WR: u32 = 0x000000A0;
pub const LOAD_USE: u32 = 0x000000B0;
pub const EC_REF: u32 = 0x000000C0;
pub const EC_WRITE_HIT_RDO: u32 = 0x000000D0;
pub const EC_SNOOP_INV: u32 = 0x000000E0;
pub const EC_RD_HIT: u32 = 0x000000F0;

/* Pic.S0 Selection Bit Field Encoding, Ultra-III */
pub const US3_CYCLE_CNT: u32 = 0x00000000;
pub const US3_INSTR_CNT: u32 = 0x00000010;
pub const US3_DISPATCH0_IC_MISS: u32 = 0x00000020;
pub const US3_DISPATCH0_BR_TGT: u32 = 0x00000030;
pub const US3_DISPATCH0_2ND_BR: u32 = 0x00000040;
pub const US3_RSTALL_STOREQ: u32 = 0x00000050;
pub const US3_RSTALL_IU_USE: u32 = 0x00000060;
pub const US3_IC_REF: u32 = 0x00000080;
pub const US3_DC_RD: u32 = 0x00000090;
pub const US3_DC_WR: u32 = 0x000000a0;
pub const US3_EC_REF: u32 = 0x000000c0;
pub const US3_EC_WR_HIT_RTO: u32 = 0x000000d0;
pub const US3_EC_SNOOP_INV: u32 = 0x000000e0;
pub const US3_EC_RD_MISS: u32 = 0x000000f0;
pub const US3_PC_PORT0_RD: u32 = 0x00000100;
pub const US3_SI_SNOOP: u32 = 0x00000110;
pub const US3_SI_CIQ_FLOW: u32 = 0x00000120;
pub const US3_SI_OWNED: u32 = 0x00000130;
pub const US3_SW_COUNT_0: u32 = 0x00000140;
pub const US3_IU_BR_MISS_TAKEN: u32 = 0x00000150;
pub const US3_IU_BR_COUNT_TAKEN: u32 = 0x00000160;
pub const US3_DISP_RS_MISPRED: u32 = 0x00000170;
pub const US3_FA_PIPE_COMPL: u32 = 0x00000180;
pub const US3_MC_READS_0: u32 = 0x00000200;
pub const US3_MC_READS_1: u32 = 0x00000210;
pub const US3_MC_READS_2: u32 = 0x00000220;
pub const US3_MC_READS_3: u32 = 0x00000230;
pub const US3_MC_STALLS_0: u32 = 0x00000240;
pub const US3_MC_STALLS_2: u32 = 0x00000250;

/* Pic.S1 Selection Bit Field Encoding, Ultra-I/II */
pub const CYCLE_CNT_D1: u32 = 0x00000000;
pub const INSTR_CNT_D1: u32 = 0x00000800;
pub const DISPATCH0_IC_MISPRED: u32 = 0x00001000;
pub const DISPATCH0_FP_USE: u32 = 0x00001800;
pub const IC_HIT: u32 = 0x00004000;
pub const DC_RD_HIT: u32 = 0x00004800;
pub const DC_WR_HIT: u32 = 0x00005000;
pub const LOAD_USE_RAW: u32 = 0x00005800;
pub const EC_HIT: u32 = 0x00006000;
pub const EC_WB: u32 = 0x00006800;
pub const EC_SNOOP_CB: u32 = 0x00007000;
pub const EC_IT_HIT: u32 = 0x00007800;

/* Pic.S1 Selection Bit Field Encoding, Ultra-III */
pub const US3_CYCLE_CNT_D1: u32 = 0x00000000;
pub const US3_INSTR_CNT_D1: u32 = 0x00000800;
pub const US3_DISPATCH0_MISPRED: u32 = 0x00001000;
pub const US3_IC_MISS_CANCELLED: u32 = 0x00001800;
pub const US3_RE_ENDIAN_MISS: u32 = 0x00002000;
pub const US3_RE_FPU_BYPASS: u32 = 0x00002800;
pub const US3_RE_DC_MISS: u32 = 0x00003000;
pub const US3_RE_EC_MISS: u32 = 0x00003800;
pub const US3_IC_MISS: u32 = 0x00004000;
pub const US3_DC_RD_MISS: u32 = 0x00004800;
pub const US3_DC_WR_MISS: u32 = 0x00005000;
pub const US3_RSTALL_FP_USE: u32 = 0x00005800;
pub const US3_EC_MISSES: u32 = 0x00006000;
pub const US3_EC_WB: u32 = 0x00006800;
pub const US3_EC_SNOOP_CB: u32 = 0x00007000;
pub const US3_EC_IC_MISS: u32 = 0x00007800;
pub const US3_RE_PC_MISS: u32 = 0x00008000;
pub const US3_ITLB_MISS: u32 = 0x00008800;
pub const US3_DTLB_MISS: u32 = 0x00009000;
pub const US3_WC_MISS: u32 = 0x00009800;
pub const US3_WC_SNOOP_CB: u32 = 0x0000a000;
pub const US3_WC_SCRUBBED: u32 = 0x0000a800;
pub const US3_WC_WB_WO_READ: u32 = 0x0000b000;
pub const US3_PC_SOFT_HIT: u32 = 0x0000c000;
pub const US3_PC_SNOOP_INV: u32 = 0x0000c800;
pub const US3_PC_HARD_HIT: u32 = 0x0000d000;
pub const US3_PC_PORT1_RD: u32 = 0x0000d800;
pub const US3_SW_COUNT_1: u32 = 0x0000e000;
pub const US3_IU_STAT_BR_MIS_UNTAKEN: u32 = 0x0000e800;
pub const US3_IU_STAT_BR_COUNT_UNTAKEN: u32 = 0x0000f000;
pub const US3_PC_MS_MISSES: u32 = 0x0000f800;
pub const US3_MC_WRITES_0: u32 = 0x00010800;
pub const US3_MC_WRITES_1: u32 = 0x00011000;
pub const US3_MC_WRITES_2: u32 = 0x00011800;
pub const US3_MC_WRITES_3: u32 = 0x00012000;
pub const US3_MC_STALLS_1: u32 = 0x00012800;
pub const US3_MC_STALLS_3: u32 = 0x00013000;
pub const US3_RE_RAW_MISS: u32 = 0x00013800;
pub const US3_FM_PIPE_COMPLETION: u32 = 0x00014000;

#[repr(C)]
pub struct vcounter_struct {
    pub vcnt0: u64,
    pub vcnt1: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
