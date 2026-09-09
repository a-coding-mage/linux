/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/* eBPF instruction mini library */

pub struct bpf_insn;

/* ALU ops on registers, bpf_add|sub|...: dst_reg += src_reg */
macro_rules! BPF_ALU64_REG { ($OP:expr, $DST:expr, $SRC:expr) => { bpf_insn { code: BPF_ALU64 | BPF_OP($OP) | BPF_X, dst_reg: $DST, src_reg: $SRC, off: 0, imm: 0 } }; }
macro_rules! BPF_ALU32_REG { ($OP:expr, $DST:expr, $SRC:expr) => { bpf_insn { code: BPF_ALU | BPF_OP($OP) | BPF_X, dst_reg: $DST, src_reg: $SRC, off: 0, imm: 0 } }; }

/* ALU ops on immediates, bpf_add|sub|...: dst_reg += imm32 */
macro_rules! BPF_ALU64_IMM { ($OP:expr, $DST:expr, $IMM:expr) => { bpf_insn { code: BPF_ALU64 | BPF_OP($OP) | BPF_K, dst_reg: $DST, src_reg: 0, off: 0, imm: $IMM } }; }
macro_rules! BPF_ALU32_IMM { ($OP:expr, $DST:expr, $IMM:expr) => { bpf_insn { code: BPF_ALU | BPF_OP($OP) | BPF_K, dst_reg: $DST, src_reg: 0, off: 0, imm: $IMM } }; }

/* Short form of mov, dst_reg = src_reg */
macro_rules! BPF_MOV64_REG { ($DST:expr, $SRC:expr) => { bpf_insn { code: BPF_ALU64 | BPF_MOV | BPF_X, dst_reg: $DST, src_reg: $SRC, off: 0, imm: 0 } }; }
macro_rules! BPF_MOV32_REG { ($DST:expr, $SRC:expr) => { bpf_insn { code: BPF_ALU | BPF_MOV | BPF_X, dst_reg: $DST, src_reg: $SRC, off: 0, imm: 0 } }; }

/* Short form of mov, dst_reg = imm32 */
macro_rules! BPF_MOV64_IMM { ($DST:expr, $IMM:expr) => { bpf_insn { code: BPF_ALU64 | BPF_MOV | BPF_K, dst_reg: $DST, src_reg: 0, off: 0, imm: $IMM } }; }
macro_rules! BPF_MOV32_IMM { ($DST:expr, $IMM:expr) => { bpf_insn { code: BPF_ALU | BPF_MOV | BPF_K, dst_reg: $DST, src_reg: 0, off: 0, imm: $IMM } }; }

/* BPF_LD_IMM64 macro encodes single 'load 64-bit immediate' insn */
macro_rules! BPF_LD_IMM64 { ($DST:expr, $IMM:expr) => { BPF_LD_IMM64_RAW!($DST, 0, $IMM) }; }
macro_rules! BPF_LD_IMM64_RAW { ($DST:expr, $SRC:expr, $IMM:expr) => { (
    bpf_insn { code: BPF_LD | BPF_DW | BPF_IMM, dst_reg: $DST, src_reg: $SRC, off: 0, imm: ($IMM as u32) },
    bpf_insn { code: 0, dst_reg: 0, src_reg: 0, off: 0, imm: (($IMM as u64) >> 32) },
) }; }

/* The build-time default applies when BPF_PSEUDO_MAP_FD is not supplied. */
#[allow(non_upper_case_globals)]
const BPF_PSEUDO_MAP_FD: u32 = 1;

/* pseudo BPF_LD_IMM64 insn used to refer to process-local map_fd */
macro_rules! BPF_LD_MAP_FD { ($DST:expr, $MAP_FD:expr) => { BPF_LD_IMM64_RAW!($DST, BPF_PSEUDO_MAP_FD, $MAP_FD) }; }

/* Direct packet access, R0 = *(uint *) (skb->data + imm32) */
macro_rules! BPF_LD_ABS { ($SIZE:expr, $IMM:expr) => { bpf_insn { code: BPF_LD | BPF_SIZE($SIZE) | BPF_ABS, dst_reg: 0, src_reg: 0, off: 0, imm: $IMM } }; }

/* Memory load, dst_reg = *(uint *) (src_reg + off16) */
macro_rules! BPF_LDX_MEM { ($SIZE:expr, $DST:expr, $SRC:expr, $OFF:expr) => { bpf_insn { code: BPF_LDX | BPF_SIZE($SIZE) | BPF_MEM, dst_reg: $DST, src_reg: $SRC, off: $OFF, imm: 0 } }; }

/* Memory store, *(uint *) (dst_reg + off16) = src_reg */
macro_rules! BPF_STX_MEM { ($SIZE:expr, $DST:expr, $SRC:expr, $OFF:expr) => { bpf_insn { code: BPF_STX | BPF_SIZE($SIZE) | BPF_MEM, dst_reg: $DST, src_reg: $SRC, off: $OFF, imm: 0 } }; }

/* Atomic operations: see the source header for the complete operation list. */
macro_rules! BPF_ATOMIC_OP { ($SIZE:expr, $OP:expr, $DST:expr, $SRC:expr, $OFF:expr) => { bpf_insn { code: BPF_STX | BPF_SIZE($SIZE) | BPF_ATOMIC, dst_reg: $DST, src_reg: $SRC, off: $OFF, imm: $OP } }; }

/* Legacy alias */
macro_rules! BPF_STX_XADD { ($SIZE:expr, $DST:expr, $SRC:expr, $OFF:expr) => { BPF_ATOMIC_OP!($SIZE, BPF_ADD, $DST, $SRC, $OFF) }; }

/* Memory store, *(uint *) (dst_reg + off16) = imm32 */
macro_rules! BPF_ST_MEM { ($SIZE:expr, $DST:expr, $OFF:expr, $IMM:expr) => { bpf_insn { code: BPF_ST | BPF_SIZE($SIZE) | BPF_MEM, dst_reg: $DST, src_reg: 0, off: $OFF, imm: $IMM } }; }

/* Conditional jumps against registers, if (dst_reg 'op' src_reg) goto pc + off16 */
macro_rules! BPF_JMP_REG { ($OP:expr, $DST:expr, $SRC:expr, $OFF:expr) => { bpf_insn { code: BPF_JMP | BPF_OP($OP) | BPF_X, dst_reg: $DST, src_reg: $SRC, off: $OFF, imm: 0 } }; }
/* Like BPF_JMP_REG, but with 32-bit wide operands for comparison. */
macro_rules! BPF_JMP32_REG { ($OP:expr, $DST:expr, $SRC:expr, $OFF:expr) => { bpf_insn { code: BPF_JMP32 | BPF_OP($OP) | BPF_X, dst_reg: $DST, src_reg: $SRC, off: $OFF, imm: 0 } }; }
/* Conditional jumps against immediates, if (dst_reg 'op' imm32) goto pc + off16 */
macro_rules! BPF_JMP_IMM { ($OP:expr, $DST:expr, $IMM:expr, $OFF:expr) => { bpf_insn { code: BPF_JMP | BPF_OP($OP) | BPF_K, dst_reg: $DST, src_reg: 0, off: $OFF, imm: $IMM } }; }
/* Like BPF_JMP_IMM, but with 32-bit wide operands for comparison. */
macro_rules! BPF_JMP32_IMM { ($OP:expr, $DST:expr, $IMM:expr, $OFF:expr) => { bpf_insn { code: BPF_JMP32 | BPF_OP($OP) | BPF_K, dst_reg: $DST, src_reg: 0, off: $OFF, imm: $IMM } }; }

/* Raw code statement block */
macro_rules! BPF_RAW_INSN { ($CODE:expr, $DST:expr, $SRC:expr, $OFF:expr, $IMM:expr) => { bpf_insn { code: $CODE, dst_reg: $DST, src_reg: $SRC, off: $OFF, imm: $IMM } }; }

/* Program exit */
macro_rules! BPF_EXIT_INSN { () => { bpf_insn { code: BPF_JMP | BPF_EXIT, dst_reg: 0, src_reg: 0, off: 0, imm: 0 } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
