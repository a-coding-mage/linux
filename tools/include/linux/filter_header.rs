/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Linux Socket Filter Data Structures
 *
 * C header dependencies: <linux/bpf.h>, <uapi/linux/filter.h>
 */

/* ArgX, context and stack frame pointer register positions. Note,
 * Arg1, Arg2, Arg3, etc are used as argument mappings of function
 * calls in BPF_CALL instruction.
 */
pub const BPF_REG_ARG1: u32 = BPF_REG_1 as u32;
pub const BPF_REG_ARG2: u32 = BPF_REG_2 as u32;
pub const BPF_REG_ARG3: u32 = BPF_REG_3 as u32;
pub const BPF_REG_ARG4: u32 = BPF_REG_4 as u32;
pub const BPF_REG_ARG5: u32 = BPF_REG_5 as u32;
pub const BPF_REG_CTX: u32 = BPF_REG_6 as u32;
pub const BPF_REG_FP: u32 = BPF_REG_10 as u32;

/* Additional register mappings for converted user programs. */
pub const BPF_REG_A: u32 = BPF_REG_0 as u32;
pub const BPF_REG_X: u32 = BPF_REG_7 as u32;
pub const BPF_REG_TMP: u32 = BPF_REG_8 as u32;

/* BPF program can access up to 512 bytes of stack space. */
pub const MAX_BPF_STACK: u32 = 512;

/* Helper macros for filter block array initializers. */

/* ALU ops on registers, bpf_add|sub|...: dst_reg += src_reg */

pub const fn BPF_ALU64_REG(OP: u32, DST: u32, SRC: u32) -> bpf_insn {
    bpf_insn {
        code: (BPF_ALU64 | BPF_OP(OP) | BPF_X) as _,
        dst_reg: DST as _,
        src_reg: SRC as _,
        off: 0,
        imm: 0,
    }
}

pub const fn BPF_ALU32_REG(OP: u32, DST: u32, SRC: u32) -> bpf_insn {
    bpf_insn {
        code: (BPF_ALU | BPF_OP(OP) | BPF_X) as _,
        dst_reg: DST as _,
        src_reg: SRC as _,
        off: 0,
        imm: 0,
    }
}

/* ALU ops on immediates, bpf_add|sub|...: dst_reg += imm32 */

pub const fn BPF_ALU64_IMM(OP: u32, DST: u32, IMM: i32) -> bpf_insn {
    bpf_insn {
        code: (BPF_ALU64 | BPF_OP(OP) | BPF_K) as _,
        dst_reg: DST as _,
        src_reg: 0,
        off: 0,
        imm: IMM,
    }
}

pub const fn BPF_ALU32_IMM(OP: u32, DST: u32, IMM: i32) -> bpf_insn {
    bpf_insn {
        code: (BPF_ALU | BPF_OP(OP) | BPF_K) as _,
        dst_reg: DST as _,
        src_reg: 0,
        off: 0,
        imm: IMM,
    }
}

/* Endianess conversion, cpu_to_{l,b}e(), {l,b}e_to_cpu() */

pub const fn BPF_ENDIAN(TYPE: u32, DST: u32, LEN: i32) -> bpf_insn {
    bpf_insn {
        code: (BPF_ALU | BPF_END | BPF_SRC(TYPE)) as _,
        dst_reg: DST as _,
        src_reg: 0,
        off: 0,
        imm: LEN,
    }
}

/* Short form of mov, dst_reg = src_reg */

pub const fn BPF_MOV64_REG(DST: u32, SRC: u32) -> bpf_insn {
    bpf_insn {
        code: (BPF_ALU64 | BPF_MOV | BPF_X) as _,
        dst_reg: DST as _,
        src_reg: SRC as _,
        off: 0,
        imm: 0,
    }
}

pub const fn BPF_MOV32_REG(DST: u32, SRC: u32) -> bpf_insn {
    bpf_insn {
        code: (BPF_ALU | BPF_MOV | BPF_X) as _,
        dst_reg: DST as _,
        src_reg: SRC as _,
        off: 0,
        imm: 0,
    }
}

/* Short form of mov, dst_reg = imm32 */

pub const fn BPF_MOV64_IMM(DST: u32, IMM: i32) -> bpf_insn {
    bpf_insn {
        code: (BPF_ALU64 | BPF_MOV | BPF_K) as _,
        dst_reg: DST as _,
        src_reg: 0,
        off: 0,
        imm: IMM,
    }
}

pub const fn BPF_MOV32_IMM(DST: u32, IMM: i32) -> bpf_insn {
    bpf_insn {
        code: (BPF_ALU | BPF_MOV | BPF_K) as _,
        dst_reg: DST as _,
        src_reg: 0,
        off: 0,
        imm: IMM,
    }
}

/* Short form of movsx, dst_reg = (s8,s16,s32)src_reg */

pub const fn BPF_MOVSX64_REG(DST: u32, SRC: u32, OFF: i16) -> bpf_insn {
    bpf_insn {
        code: (BPF_ALU64 | BPF_MOV | BPF_X) as _,
        dst_reg: DST as _,
        src_reg: SRC as _,
        off: OFF,
        imm: 0,
    }
}

pub const fn BPF_MOVSX32_REG(DST: u32, SRC: u32, OFF: i16) -> bpf_insn {
    bpf_insn {
        code: (BPF_ALU | BPF_MOV | BPF_X) as _,
        dst_reg: DST as _,
        src_reg: SRC as _,
        off: OFF,
        imm: 0,
    }
}

/* Short form of mov based on type,  BPF_X: dst_reg = src_reg, BPF_K: dst_reg = imm32 */

pub const fn BPF_MOV64_RAW(TYPE: u32, DST: u32, SRC: u32, IMM: i32) -> bpf_insn {
    bpf_insn {
        code: (BPF_ALU64 | BPF_MOV | BPF_SRC(TYPE)) as _,
        dst_reg: DST as _,
        src_reg: SRC as _,
        off: 0,
        imm: IMM,
    }
}

pub const fn BPF_MOV32_RAW(TYPE: u32, DST: u32, SRC: u32, IMM: i32) -> bpf_insn {
    bpf_insn {
        code: (BPF_ALU | BPF_MOV | BPF_SRC(TYPE)) as _,
        dst_reg: DST as _,
        src_reg: SRC as _,
        off: 0,
        imm: IMM,
    }
}

/* Direct packet access, R0 = *(uint *) (skb->data + imm32) */

pub const fn BPF_LD_ABS(SIZE: u32, IMM: i32) -> bpf_insn {
    bpf_insn {
        code: (BPF_LD | BPF_SIZE(SIZE) | BPF_ABS) as _,
        dst_reg: 0,
        src_reg: 0,
        off: 0,
        imm: IMM,
    }
}

/* Indirect packet access, R0 = *(uint *) (skb->data + src_reg + imm32) */

pub const fn BPF_LD_IND(SIZE: u32, SRC: u32, IMM: i32) -> bpf_insn {
    bpf_insn {
        code: (BPF_LD | BPF_SIZE(SIZE) | BPF_IND) as _,
        dst_reg: 0,
        src_reg: SRC as _,
        off: 0,
        imm: IMM,
    }
}

/* Memory load, dst_reg = *(uint *) (src_reg + off16) */

pub const fn BPF_LDX_MEM(SIZE: u32, DST: u32, SRC: u32, OFF: i16) -> bpf_insn {
    bpf_insn {
        code: (BPF_LDX | BPF_SIZE(SIZE) | BPF_MEM) as _,
        dst_reg: DST as _,
        src_reg: SRC as _,
        off: OFF,
        imm: 0,
    }
}

/* Memory store, *(uint *) (dst_reg + off16) = src_reg */

pub const fn BPF_STX_MEM(SIZE: u32, DST: u32, SRC: u32, OFF: i16) -> bpf_insn {
    bpf_insn {
        code: (BPF_STX | BPF_SIZE(SIZE) | BPF_MEM) as _,
        dst_reg: DST as _,
        src_reg: SRC as _,
        off: OFF,
        imm: 0,
    }
}

/*
 * Atomic operations:
 *
 *   BPF_ADD                  *(uint *) (dst_reg + off16) += src_reg
 *   BPF_AND                  *(uint *) (dst_reg + off16) &= src_reg
 *   BPF_OR                   *(uint *) (dst_reg + off16) |= src_reg
 *   BPF_XOR                  *(uint *) (dst_reg + off16) ^= src_reg
 *   BPF_ADD | BPF_FETCH      src_reg = atomic_fetch_add(dst_reg + off16, src_reg);
 *   BPF_AND | BPF_FETCH      src_reg = atomic_fetch_and(dst_reg + off16, src_reg);
 *   BPF_OR | BPF_FETCH       src_reg = atomic_fetch_or(dst_reg + off16, src_reg);
 *   BPF_XOR | BPF_FETCH      src_reg = atomic_fetch_xor(dst_reg + off16, src_reg);
 *   BPF_XCHG                 src_reg = atomic_xchg(dst_reg + off16, src_reg)
 *   BPF_CMPXCHG              r0 = atomic_cmpxchg(dst_reg + off16, r0, src_reg)
 */

pub const fn BPF_ATOMIC_OP(SIZE: u32, OP: i32, DST: u32, SRC: u32, OFF: i16) -> bpf_insn {
    bpf_insn {
        code: (BPF_STX | BPF_SIZE(SIZE) | BPF_ATOMIC) as _,
        dst_reg: DST as _,
        src_reg: SRC as _,
        off: OFF,
        imm: OP,
    }
}

/* Legacy alias */
pub const fn BPF_STX_XADD(SIZE: u32, DST: u32, SRC: u32, OFF: i16) -> bpf_insn {
    BPF_ATOMIC_OP(SIZE, BPF_ADD as i32, DST, SRC, OFF)
}

/* Memory store, *(uint *) (dst_reg + off16) = imm32 */

pub const fn BPF_ST_MEM(SIZE: u32, DST: u32, OFF: i16, IMM: i32) -> bpf_insn {
    bpf_insn {
        code: (BPF_ST | BPF_SIZE(SIZE) | BPF_MEM) as _,
        dst_reg: DST as _,
        src_reg: 0,
        off: OFF,
        imm: IMM,
    }
}

/* Conditional jumps against registers, if (dst_reg 'op' src_reg) goto pc + off16 */

pub const fn BPF_JMP_REG(OP: u32, DST: u32, SRC: u32, OFF: i16) -> bpf_insn {
    bpf_insn {
        code: (BPF_JMP | BPF_OP(OP) | BPF_X) as _,
        dst_reg: DST as _,
        src_reg: SRC as _,
        off: OFF,
        imm: 0,
    }
}

/* Like BPF_JMP_REG, but with 32-bit wide operands for comparison. */

pub const fn BPF_JMP32_REG(OP: u32, DST: u32, SRC: u32, OFF: i16) -> bpf_insn {
    bpf_insn {
        code: (BPF_JMP32 | BPF_OP(OP) | BPF_X) as _,
        dst_reg: DST as _,
        src_reg: SRC as _,
        off: OFF,
        imm: 0,
    }
}

/* Conditional jumps against immediates, if (dst_reg 'op' imm32) goto pc + off16 */

pub const fn BPF_JMP_IMM(OP: u32, DST: u32, IMM: i32, OFF: i16) -> bpf_insn {
    bpf_insn {
        code: (BPF_JMP | BPF_OP(OP) | BPF_K) as _,
        dst_reg: DST as _,
        src_reg: 0,
        off: OFF,
        imm: IMM,
    }
}

/* Like BPF_JMP_IMM, but with 32-bit wide operands for comparison. */

pub const fn BPF_JMP32_IMM(OP: u32, DST: u32, IMM: i32, OFF: i16) -> bpf_insn {
    bpf_insn {
        code: (BPF_JMP32 | BPF_OP(OP) | BPF_K) as _,
        dst_reg: DST as _,
        src_reg: 0,
        off: OFF,
        imm: IMM,
    }
}

/* Unconditional jumps, goto pc + off16 */

pub const fn BPF_JMP_A(OFF: i16) -> bpf_insn {
    bpf_insn {
        code: (BPF_JMP | BPF_JA) as _,
        dst_reg: 0,
        src_reg: 0,
        off: OFF,
        imm: 0,
    }
}

/* Unconditional jumps, gotol pc + imm32 */

pub const fn BPF_JMP32_A(IMM: i32) -> bpf_insn {
    bpf_insn {
        code: (BPF_JMP32 | BPF_JA) as _,
        dst_reg: 0,
        src_reg: 0,
        off: 0,
        imm: IMM,
    }
}

/* Function call */

pub const fn BPF_EMIT_CALL(FUNC: i32) -> bpf_insn {
    bpf_insn {
        code: (BPF_JMP | BPF_CALL) as _,
        dst_reg: 0,
        src_reg: 0,
        off: 0,
        imm: FUNC - BPF_FUNC_unspec as i32,
    }
}

/* Raw code statement block */

pub const fn BPF_RAW_INSN(CODE: u32, DST: u32, SRC: u32, OFF: i16, IMM: i32) -> bpf_insn {
    bpf_insn {
        code: CODE as _,
        dst_reg: DST as _,
        src_reg: SRC as _,
        off: OFF,
        imm: IMM,
    }
}

/* BPF_LD_IMM64 macro encodes single 'load 64-bit immediate' insn */

pub const fn BPF_LD_IMM64(DST: u32, IMM: u64) -> [bpf_insn; 2] {
    BPF_LD_IMM64_RAW(DST, 0, IMM)
}

pub const fn BPF_LD_IMM64_RAW(DST: u32, SRC: u32, IMM: u64) -> [bpf_insn; 2] {
    [
        bpf_insn {
            code: (BPF_LD | BPF_DW | BPF_IMM) as _,
            dst_reg: DST as _,
            src_reg: SRC as _,
            off: 0,
            imm: IMM as u32 as i32,
        },
        bpf_insn {
            code: 0, /* zero is reserved opcode */
            dst_reg: 0,
            src_reg: 0,
            off: 0,
            imm: (IMM >> 32) as i32,
        },
    ]
}

pub const fn BPF_LD_IMM64_RAW_FULL(
    DST: u32,
    SRC: u32,
    OFF1: i16,
    OFF2: i16,
    IMM1: i32,
    IMM2: i32,
) -> [bpf_insn; 2] {
    [
        bpf_insn {
            code: (BPF_LD | BPF_DW | BPF_IMM) as _,
            dst_reg: DST as _,
            src_reg: SRC as _,
            off: OFF1,
            imm: IMM1,
        },
        bpf_insn {
            code: 0, /* zero is reserved opcode */
            dst_reg: 0,
            src_reg: 0,
            off: OFF2,
            imm: IMM2,
        },
    ]
}

/* pseudo BPF_LD_IMM64 insn used to refer to process-local map_fd */

pub const fn BPF_LD_MAP_FD(DST: u32, MAP_FD: i32) -> [bpf_insn; 2] {
    BPF_LD_IMM64_RAW_FULL(DST, BPF_PSEUDO_MAP_FD as u32, 0, 0, MAP_FD, 0)
}

pub const fn BPF_LD_MAP_VALUE(DST: u32, MAP_FD: i32, VALUE_OFF: i32) -> [bpf_insn; 2] {
    BPF_LD_IMM64_RAW_FULL(
        DST,
        BPF_PSEUDO_MAP_VALUE as u32,
        0,
        0,
        MAP_FD,
        VALUE_OFF,
    )
}

/* Relative call */

pub const fn BPF_CALL_REL(TGT: i32) -> bpf_insn {
    bpf_insn {
        code: (BPF_JMP | BPF_CALL) as _,
        dst_reg: 0,
        src_reg: BPF_PSEUDO_CALL as _,
        off: 0,
        imm: TGT,
    }
}

/* Program exit */

pub const fn BPF_EXIT_INSN() -> bpf_insn {
    bpf_insn {
        code: (BPF_JMP | BPF_EXIT) as _,
        dst_reg: 0,
        src_reg: 0,
        off: 0,
        imm: 0,
    }
}
