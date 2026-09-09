/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 */

// Conditional compilation intent: !defined(_DRIVER_BUILD)
// Conditional compilation intent: !defined(GL_ZERO)
pub const GL__ZERO: u32 = BLEND_ZERO;
pub const GL__ONE: u32 = BLEND_ONE;
pub const GL__SRC_COLOR: u32 = BLEND_SRC_COLOR;
pub const GL__ONE_MINUS_SRC_COLOR: u32 = BLEND_ONE_MINUS_SRC_COLOR;
pub const GL__DST_COLOR: u32 = BLEND_DST_COLOR;
pub const GL__ONE_MINUS_DST_COLOR: u32 = BLEND_ONE_MINUS_DST_COLOR;
pub const GL__SRC_ALPHA: u32 = BLEND_SRC_ALPHA;
pub const GL__ONE_MINUS_SRC_ALPHA: u32 = BLEND_ONE_MINUS_SRC_ALPHA;
pub const GL__DST_ALPHA: u32 = BLEND_DST_ALPHA;
pub const GL__ONE_MINUS_DST_ALPHA: u32 = BLEND_ONE_MINUS_DST_ALPHA;
pub const GL__SRC_ALPHA_SATURATE: u32 = BLEND_SRC_ALPHA_SATURATE;
pub const GL__CONSTANT_COLOR: u32 = BLEND_CONSTANT_COLOR;
pub const GL__ONE_MINUS_CONSTANT_COLOR: u32 = BLEND_ONE_MINUS_CONSTANT_COLOR;
pub const GL__CONSTANT_ALPHA: u32 = BLEND_CONSTANT_ALPHA;
pub const GL__ONE_MINUS_CONSTANT_ALPHA: u32 = BLEND_ONE_MINUS_CONSTANT_ALPHA;
// End conditional compilation
// End conditional compilation!defined(_DRIVER_BUILD)


/*
 * CP_PERFMON_ENABLE_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CP_PERFMON_ENABLE_MODE {
CP_PERFMON_ENABLE_MODE_ALWAYS_COUNT      = 0x00000000u32,
CP_PERFMON_ENABLE_MODE_RESERVED_1        = 0x00000001u32,
CP_PERFMON_ENABLE_MODE_COUNT_CONTEXT_TRUE = 0x00000002u32,
CP_PERFMON_ENABLE_MODE_COUNT_CONTEXT_FALSE = 0x00000003u32,
}

/*
 * CP_PERFMON_STATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CP_PERFMON_STATE {
CP_PERFMON_STATE_DISABLE_AND_RESET       = 0x00000000u32,
CP_PERFMON_STATE_START_COUNTING          = 0x00000001u32,
CP_PERFMON_STATE_STOP_COUNTING           = 0x00000002u32,
CP_PERFMON_STATE_RESERVED_3              = 0x00000003u32,
CP_PERFMON_STATE_DISABLE_AND_RESET_PHANTOM = 0x00000004u32,
CP_PERFMON_STATE_COUNT_AND_DUMP_PHANTOM  = 0x00000005u32,
}

/*
 * ENUM_NUM_SIMD_PER_CU enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_NUM_SIMD_PER_CU {
NUM_SIMD_PER_CU                          = 0x00000002u32,
}

/*
 * GATCL1RequestType enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GATCL1RequestType {
GATCL1_TYPE_NORMAL                       = 0x00000000u32,
GATCL1_TYPE_SHOOTDOWN                    = 0x00000001u32,
GATCL1_TYPE_BYPASS                       = 0x00000002u32,
}

/*
 * GL0V_CACHE_POLICIES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GL0V_CACHE_POLICIES {
GL0V_CACHE_POLICY_MISS_LRU               = 0x00000000u32,
GL0V_CACHE_POLICY_MISS_EVICT             = 0x00000001u32,
GL0V_CACHE_POLICY_HIT_LRU                = 0x00000002u32,
GL0V_CACHE_POLICY_HIT_EVICT              = 0x00000003u32,
GL0V_CACHE_POLICY_MISS_INVAL             = 0x00000004u32,
}

/*
 * GL1_CACHE_POLICIES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GL1_CACHE_POLICIES {
GL1_CACHE_POLICY_MISS_LRU                = 0x00000000u32,
GL1_CACHE_POLICY_MISS_EVICT              = 0x00000001u32,
GL1_CACHE_POLICY_HIT_LRU                 = 0x00000002u32,
GL1_CACHE_POLICY_HIT_EVICT               = 0x00000003u32,
}

/*
 * GL1_CACHE_STORE_POLICIES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GL1_CACHE_STORE_POLICIES {
GL1_CACHE_STORE_POLICY_BYPASS            = 0x00000000u32,
}

/*
 * GL2_CACHE_POLICIES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GL2_CACHE_POLICIES {
GL2_CACHE_POLICY_LRU                     = 0x00000000u32,
GL2_CACHE_POLICY_STREAM                  = 0x00000001u32,
GL2_CACHE_POLICY_NOA                     = 0x00000002u32,
GL2_CACHE_POLICY_BYPASS                  = 0x00000003u32,
}

/*
 * GL2_NACKS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GL2_NACKS {
GL2_NACK_NO_FAULT                        = 0x00000000u32,
GL2_NACK_PAGE_FAULT                      = 0x00000001u32,
GL2_NACK_PROTECTION_FAULT                = 0x00000002u32,
GL2_NACK_DATA_ERROR                      = 0x00000003u32,
}

/*
 * GL2_OP enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GL2_OP {
GL2_OP_READ                              = 0x00000000u32,
GL2_OP_ATOMIC_FCMPSWAP_RTN_32            = 0x00000001u32,
GL2_OP_ATOMIC_FMIN_RTN_32                = 0x00000002u32,
GL2_OP_ATOMIC_FMAX_RTN_32                = 0x00000003u32,
GL2_OP_ATOMIC_PK_ADD_FP16_RTN            = 0x00000004u32,
GL2_OP_ATOMIC_FADD_RTN_32                = 0x00000005u32,
GL2_OP_ATOMIC_PK_ADD_BF16_RTN            = 0x00000006u32,
GL2_OP_ATOMIC_SWAP_RTN_32                = 0x00000007u32,
GL2_OP_ATOMIC_CMPSWAP_RTN_32             = 0x00000008u32,
GL2_OP_ATOMIC_FCMPSWAP_FLUSH_DENORM_RTN_32 = 0x00000009u32,
GL2_OP_ATOMIC_FMIN_FLUSH_DENORM_RTN_32   = 0x0000000au32,
GL2_OP_ATOMIC_FMAX_FLUSH_DENORM_RTN_32   = 0x0000000bu32,
GL2_OP_PROBE_FILTER                      = 0x0000000cu32,
GL2_OP_ATOMIC_FADD_FLUSH_DENORM_RTN_32   = 0x0000000du32,
GL2_OP_RESERVED_FOP_FLUSH_DENORM_RTN_32_2 = 0x0000000eu32,
GL2_OP_ATOMIC_ADD_RTN_32                 = 0x0000000fu32,
GL2_OP_ATOMIC_SUB_RTN_32                 = 0x00000010u32,
GL2_OP_ATOMIC_SMIN_RTN_32                = 0x00000011u32,
GL2_OP_ATOMIC_UMIN_RTN_32                = 0x00000012u32,
GL2_OP_ATOMIC_SMAX_RTN_32                = 0x00000013u32,
GL2_OP_ATOMIC_UMAX_RTN_32                = 0x00000014u32,
GL2_OP_ATOMIC_AND_RTN_32                 = 0x00000015u32,
GL2_OP_ATOMIC_OR_RTN_32                  = 0x00000016u32,
GL2_OP_ATOMIC_XOR_RTN_32                 = 0x00000017u32,
GL2_OP_ATOMIC_INC_RTN_32                 = 0x00000018u32,
GL2_OP_ATOMIC_DEC_RTN_32                 = 0x00000019u32,
GL2_OP_ATOMIC_CLAMP_SUB_RTN_32           = 0x0000001au32,
GL2_OP_ATOMIC_COND_SUB_RTN_32            = 0x0000001bu32,
GL2_OP_UTC_PROBE                         = 0x0000001du32,
GL2_OP_LOAD_RESERVE                      = 0x0000001eu32,
GL2_OP_WRITE                             = 0x00000020u32,
GL2_OP_ATOMIC_FCMPSWAP_RTN_64            = 0x00000021u32,
GL2_OP_ATOMIC_FMIN_RTN_64                = 0x00000022u32,
GL2_OP_ATOMIC_FMAX_RTN_64                = 0x00000023u32,
GL2_OP_ATOMIC_SWAP_RTN_64                = 0x00000027u32,
GL2_OP_ATOMIC_CMPSWAP_RTN_64             = 0x00000028u32,
GL2_OP_ATOMIC_FCMPSWAP_FLUSH_DENORM_RTN_64 = 0x00000029u32,
GL2_OP_ATOMIC_FMIN_FLUSH_DENORM_RTN_64   = 0x0000002au32,
GL2_OP_ATOMIC_FMAX_FLUSH_DENORM_RTN_64   = 0x0000002bu32,
GL2_OP_ATOMIC_ADD_RTN_64                 = 0x0000002fu32,
GL2_OP_ATOMIC_SUB_RTN_64                 = 0x00000030u32,
GL2_OP_ATOMIC_SMIN_RTN_64                = 0x00000031u32,
GL2_OP_ATOMIC_UMIN_RTN_64                = 0x00000032u32,
GL2_OP_ATOMIC_SMAX_RTN_64                = 0x00000033u32,
GL2_OP_ATOMIC_UMAX_RTN_64                = 0x00000034u32,
GL2_OP_ATOMIC_AND_RTN_64                 = 0x00000035u32,
GL2_OP_ATOMIC_OR_RTN_64                  = 0x00000036u32,
GL2_OP_ATOMIC_XOR_RTN_64                 = 0x00000037u32,
GL2_OP_ATOMIC_INC_RTN_64                 = 0x00000038u32,
GL2_OP_ATOMIC_DEC_RTN_64                 = 0x00000039u32,
GL2_OP_WRITE_ZERO_SIZE                   = 0x0000003bu32,
GL2_OP_GL2_INV                           = 0x0000003du32,
GL2_OP_ATOMIC_STORE_COND_RTN             = 0x0000003eu32,
GL2_OP_GL1_INV                           = 0x00000040u32,
GL2_OP_ATOMIC_FCMPSWAP_32                = 0x00000041u32,
GL2_OP_ATOMIC_FMIN_32                    = 0x00000042u32,
GL2_OP_ATOMIC_FMAX_32                    = 0x00000043u32,
GL2_OP_ATOMIC_PK_ADD_FP16                = 0x00000044u32,
GL2_OP_ATOMIC_FADD_32                    = 0x00000045u32,
GL2_OP_ATOMIC_PK_ADD_BF16                = 0x00000046u32,
GL2_OP_ATOMIC_SWAP_32                    = 0x00000047u32,
GL2_OP_ATOMIC_CMPSWAP_32                 = 0x00000048u32,
GL2_OP_ATOMIC_FCMPSWAP_FLUSH_DENORM_32   = 0x00000049u32,
GL2_OP_ATOMIC_FMIN_FLUSH_DENORM_32       = 0x0000004au32,
GL2_OP_ATOMIC_FMAX_FLUSH_DENORM_32       = 0x0000004bu32,
GL2_OP_ATOMIC_UMIN_8                     = 0x0000004cu32,
GL2_OP_ATOMIC_FADD_FLUSH_DENORM_32       = 0x0000004du32,
GL2_OP_ATOMIC_ADD_32                     = 0x0000004fu32,
GL2_OP_ATOMIC_SUB_32                     = 0x00000050u32,
GL2_OP_ATOMIC_SMIN_32                    = 0x00000051u32,
GL2_OP_ATOMIC_UMIN_32                    = 0x00000052u32,
GL2_OP_ATOMIC_SMAX_32                    = 0x00000053u32,
GL2_OP_ATOMIC_UMAX_32                    = 0x00000054u32,
GL2_OP_ATOMIC_AND_32                     = 0x00000055u32,
GL2_OP_ATOMIC_OR_32                      = 0x00000056u32,
GL2_OP_ATOMIC_XOR_32                     = 0x00000057u32,
GL2_OP_ATOMIC_INC_32                     = 0x00000058u32,
GL2_OP_ATOMIC_DEC_32                     = 0x00000059u32,
GL2_OP_NOP_RTN0                          = 0x0000005bu32,
GL2_OP_GL2_WB                            = 0x0000005du32,
GL2_OP_FORCE_EXISTING_DATA_TO_DECOMPRESS = 0x0000005eu32,
GL2_OP_ATOMIC_FCMPSWAP_64                = 0x00000061u32,
GL2_OP_ATOMIC_FMIN_64                    = 0x00000062u32,
GL2_OP_ATOMIC_FMAX_64                    = 0x00000063u32,
GL2_OP_ATOMIC_SWAP_64                    = 0x00000067u32,
GL2_OP_ATOMIC_CMPSWAP_64                 = 0x00000068u32,
GL2_OP_ATOMIC_FCMPSWAP_FLUSH_DENORM_64   = 0x00000069u32,
GL2_OP_ATOMIC_FMIN_FLUSH_DENORM_64       = 0x0000006au32,
GL2_OP_ATOMIC_FMAX_FLUSH_DENORM_64       = 0x0000006bu32,
GL2_OP_ATOMIC_ADD_64                     = 0x0000006fu32,
GL2_OP_ATOMIC_SUB_64                     = 0x00000070u32,
GL2_OP_ATOMIC_SMIN_64                    = 0x00000071u32,
GL2_OP_ATOMIC_UMIN_64                    = 0x00000072u32,
GL2_OP_ATOMIC_SMAX_64                    = 0x00000073u32,
GL2_OP_ATOMIC_UMAX_64                    = 0x00000074u32,
GL2_OP_ATOMIC_AND_64                     = 0x00000075u32,
GL2_OP_ATOMIC_OR_64                      = 0x00000076u32,
GL2_OP_ATOMIC_XOR_64                     = 0x00000077u32,
GL2_OP_ATOMIC_INC_64                     = 0x00000078u32,
GL2_OP_ATOMIC_DEC_64                     = 0x00000079u32,
GL2_OP_ATOMIC_UMAX_8                     = 0x0000007au32,
GL2_OP_NOP_ACK                           = 0x0000007bu32,
GL2_OP_GL2_WBINV                         = 0x0000007du32,
GL2_OP_READ_COMPRESSION_KEY              = 0x0000007eu32,
}

/*
 * GL2_OP_MASKS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GL2_OP_MASKS {
GL2_OP_MASK_FLUSH_DENROM                 = 0x00000008u32,
GL2_OP_MASK_64                           = 0x00000020u32,
GL2_OP_MASK_NO_RTN                       = 0x00000040u32,
}

/*
 * Hdp_SurfaceEndian enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum Hdp_SurfaceEndian {
HDP_ENDIAN_NONE                          = 0x00000000u32,
HDP_ENDIAN_8IN16                         = 0x00000001u32,
HDP_ENDIAN_8IN32                         = 0x00000002u32,
HDP_ENDIAN_8IN64                         = 0x00000003u32,
}

/*
 * MTYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTYPE {
MTYPE_C_RW_US                            = 0x00000000u32,
MTYPE_RESERVED_1                         = 0x00000001u32,
MTYPE_C_RO_S                             = 0x00000002u32,
MTYPE_UC                                 = 0x00000003u32,
MTYPE_C_RW_S                             = 0x00000004u32,
MTYPE_RESERVED_5                         = 0x00000005u32,
MTYPE_C_RO_US                            = 0x00000006u32,
MTYPE_RESERVED_7                         = 0x00000007u32,
}

/*
 * PERFMON_COUNTER_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFMON_COUNTER_MODE {
PERFMON_COUNTER_MODE_ACCUM               = 0x00000000u32,
PERFMON_COUNTER_MODE_ACTIVE_CYCLES       = 0x00000001u32,
PERFMON_COUNTER_MODE_MAX                 = 0x00000002u32,
PERFMON_COUNTER_MODE_DIRTY               = 0x00000003u32,
PERFMON_COUNTER_MODE_SAMPLE              = 0x00000004u32,
PERFMON_COUNTER_MODE_CYCLES_SINCE_FIRST_EVENT = 0x00000005u32,
PERFMON_COUNTER_MODE_CYCLES_SINCE_LAST_EVENT = 0x00000006u32,
PERFMON_COUNTER_MODE_CYCLES_GE_HI        = 0x00000007u32,
PERFMON_COUNTER_MODE_CYCLES_EQ_HI        = 0x00000008u32,
PERFMON_COUNTER_MODE_INACTIVE_CYCLES     = 0x00000009u32,
PERFMON_COUNTER_MODE_RESERVED            = 0x0000000fu32,
}

/*
 * PERFMON_SPM_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFMON_SPM_MODE {
PERFMON_SPM_MODE_OFF                     = 0x00000000u32,
PERFMON_SPM_MODE_16BIT_CLAMP             = 0x00000001u32,
PERFMON_SPM_MODE_16BIT_NO_CLAMP          = 0x00000002u32,
PERFMON_SPM_MODE_32BIT_CLAMP             = 0x00000003u32,
PERFMON_SPM_MODE_32BIT_NO_CLAMP          = 0x00000004u32,
PERFMON_SPM_MODE_RESERVED_5              = 0x00000005u32,
PERFMON_SPM_MODE_RESERVED_6              = 0x00000006u32,
PERFMON_SPM_MODE_RESERVED_7              = 0x00000007u32,
PERFMON_SPM_MODE_TEST_MODE_0             = 0x00000008u32,
PERFMON_SPM_MODE_TEST_MODE_1             = 0x00000009u32,
PERFMON_SPM_MODE_TEST_MODE_2             = 0x0000000au32,
}

/*
 * READ_COMPRESSION_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum READ_COMPRESSION_MODE {
COMPRESSION_MODE_BYPASS_COMPRESSION      = 0x00000000u32,
COMPRESSION_MODE_READ_RAW_COMPRESSED_DATA = 0x00000001u32,
COMPRESSION_MODE_READ_DECOMPRESSED       = 0x00000002u32,
}

/*
 * ReadPolicy enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ReadPolicy {
CACHE_LRU_RD                             = 0x00000000u32,
CACHE_STREAM_RD                          = 0x00000001u32,
CACHE_NOA                                = 0x00000002u32,
RESERVED_RDPOLICY                        = 0x00000003u32,
}

/*
 * SCOPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SCOPE {
SCOPE_CU                                 = 0x00000000u32,
SCOPE_SE                                 = 0x00000001u32,
SCOPE_DEV                                = 0x00000002u32,
SCOPE_SYS                                = 0x00000003u32,
}

/*
 * SDMA_PERFMON_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SDMA_PERFMON_SEL {
SDMA_PERFMON_SEL_CYCLE                   = 0x00000000u32,
SDMA_PERFMON_SEL_IDLE                    = 0x00000001u32,
SDMA_PERFMON_SEL_REG_IDLE                = 0x00000002u32,
SDMA_PERFMON_SEL_RB_EMPTY                = 0x00000003u32,
SDMA_PERFMON_SEL_RB_FULL                 = 0x00000004u32,
SDMA_PERFMON_SEL_RB_WPTR_WRAP            = 0x00000005u32,
SDMA_PERFMON_SEL_RB_RPTR_WRAP            = 0x00000006u32,
SDMA_PERFMON_SEL_RB_WPTR_POLL_READ       = 0x00000007u32,
SDMA_PERFMON_SEL_RB_RPTR_WB              = 0x00000008u32,
SDMA_PERFMON_SEL_RB_CMD_IDLE             = 0x00000009u32,
SDMA_PERFMON_SEL_RB_CMD_FULL             = 0x0000000au32,
SDMA_PERFMON_SEL_IB_CMD_IDLE             = 0x0000000bu32,
SDMA_PERFMON_SEL_IB_CMD_FULL             = 0x0000000cu32,
SDMA_PERFMON_SEL_EX_IDLE                 = 0x0000000du32,
SDMA_PERFMON_SEL_SRBM_REG_SEND           = 0x0000000eu32,
SDMA_PERFMON_SEL_EX_IDLE_POLL_TIMER_EXPIRE = 0x0000000fu32,
SDMA_PERFMON_SEL_WR_BA_RTR               = 0x00000010u32,
SDMA_PERFMON_SEL_MC_WR_IDLE              = 0x00000011u32,
SDMA_PERFMON_SEL_MC_WR_COUNT             = 0x00000012u32,
SDMA_PERFMON_SEL_RD_BA_RTR               = 0x00000013u32,
SDMA_PERFMON_SEL_MC_RD_IDLE              = 0x00000014u32,
SDMA_PERFMON_SEL_MC_RD_COUNT             = 0x00000015u32,
SDMA_PERFMON_SEL_MC_RD_RET_STALL         = 0x00000016u32,
SDMA_PERFMON_SEL_MC_RD_NO_POLL_IDLE      = 0x00000017u32,
SDMA_PERFMON_SEL_SEM_IDLE                = 0x0000001au32,
SDMA_PERFMON_SEL_SEM_REQ_STALL           = 0x0000001bu32,
SDMA_PERFMON_SEL_SEM_REQ_COUNT           = 0x0000001cu32,
SDMA_PERFMON_SEL_SEM_RESP_INCOMPLETE     = 0x0000001du32,
SDMA_PERFMON_SEL_SEM_RESP_FAIL           = 0x0000001eu32,
SDMA_PERFMON_SEL_SEM_RESP_PASS           = 0x0000001fu32,
SDMA_PERFMON_SEL_INT_IDLE                = 0x00000020u32,
SDMA_PERFMON_SEL_INT_REQ_STALL           = 0x00000021u32,
SDMA_PERFMON_SEL_INT_REQ_COUNT           = 0x00000022u32,
SDMA_PERFMON_SEL_INT_RESP_ACCEPTED       = 0x00000023u32,
SDMA_PERFMON_SEL_INT_RESP_RETRY          = 0x00000024u32,
SDMA_PERFMON_SEL_NUM_PACKET              = 0x00000025u32,
SDMA_PERFMON_SEL_CE_WREQ_IDLE            = 0x00000027u32,
SDMA_PERFMON_SEL_CE_WR_IDLE              = 0x00000028u32,
SDMA_PERFMON_SEL_CE_SPLIT_IDLE           = 0x00000029u32,
SDMA_PERFMON_SEL_CE_RREQ_IDLE            = 0x0000002au32,
SDMA_PERFMON_SEL_CE_OUT_IDLE             = 0x0000002bu32,
SDMA_PERFMON_SEL_CE_IN_IDLE              = 0x0000002cu32,
SDMA_PERFMON_SEL_CE_DST_IDLE             = 0x0000002du32,
SDMA_PERFMON_SEL_CE_AFIFO_FULL           = 0x00000030u32,
SDMA_PERFMON_SEL_DUMMY_0                 = 0x00000031u32,
SDMA_PERFMON_SEL_DUMMY_1                 = 0x00000032u32,
SDMA_PERFMON_SEL_CE_INFO_FULL            = 0x00000033u32,
SDMA_PERFMON_SEL_CE_INFO1_FULL           = 0x00000034u32,
SDMA_PERFMON_SEL_CE_RD_STALL             = 0x00000035u32,
SDMA_PERFMON_SEL_CE_WR_STALL             = 0x00000036u32,
SDMA_PERFMON_SEL_QUEUE0_SELECT           = 0x00000037u32,
SDMA_PERFMON_SEL_QUEUE1_SELECT           = 0x00000038u32,
SDMA_PERFMON_SEL_QUEUE2_SELECT           = 0x00000039u32,
SDMA_PERFMON_SEL_QUEUE3_SELECT           = 0x0000003au32,
SDMA_PERFMON_SEL_CTX_CHANGE              = 0x0000003bu32,
SDMA_PERFMON_SEL_CTX_CHANGE_EXPIRED      = 0x0000003cu32,
SDMA_PERFMON_SEL_CTX_CHANGE_EXCEPTION    = 0x0000003du32,
SDMA_PERFMON_SEL_DOORBELL                = 0x0000003eu32,
SDMA_PERFMON_SEL_MCU_L1_WR_VLD           = 0x0000003fu32,
SDMA_PERFMON_SEL_CE_L1_WR_VLD            = 0x00000040u32,
SDMA_PERFMON_SEL_CPF_SDMA_INVREQ         = 0x00000041u32,
SDMA_PERFMON_SEL_SDMA_CPF_INVACK         = 0x00000042u32,
SDMA_PERFMON_SEL_UTCL2_SDMA_INVREQ       = 0x00000043u32,
SDMA_PERFMON_SEL_SDMA_UTCL2_INVACK       = 0x00000044u32,
SDMA_PERFMON_SEL_UTCL2_SDMA_INVREQ_ALL   = 0x00000045u32,
SDMA_PERFMON_SEL_SDMA_UTCL2_INVACK_ALL   = 0x00000046u32,
SDMA_PERFMON_SEL_UTCL2_RET_XNACK         = 0x00000047u32,
SDMA_PERFMON_SEL_UTCL2_RET_ACK           = 0x00000048u32,
SDMA_PERFMON_SEL_UTCL2_FREE              = 0x00000049u32,
SDMA_PERFMON_SEL_SDMA_UTCL2_SEND         = 0x0000004au32,
SDMA_PERFMON_SEL_DMA_L1_WR_SEND          = 0x0000004bu32,
SDMA_PERFMON_SEL_DMA_L1_RD_SEND          = 0x0000004cu32,
SDMA_PERFMON_SEL_DMA_MC_WR_SEND          = 0x0000004du32,
SDMA_PERFMON_SEL_DMA_MC_RD_SEND          = 0x0000004eu32,
SDMA_PERFMON_SEL_GPUVM_INV_HIGH          = 0x0000004fu32,
SDMA_PERFMON_SEL_GPUVM_INV_LOW           = 0x00000050u32,
SDMA_PERFMON_SEL_L1_WRL2_IDLE            = 0x00000051u32,
SDMA_PERFMON_SEL_L1_RDL2_IDLE            = 0x00000052u32,
SDMA_PERFMON_SEL_L1_WRMC_IDLE            = 0x00000053u32,
SDMA_PERFMON_SEL_L1_RDMC_IDLE            = 0x00000054u32,
SDMA_PERFMON_SEL_L1_WR_INV_IDLE          = 0x00000055u32,
SDMA_PERFMON_SEL_L1_RD_INV_IDLE          = 0x00000056u32,
SDMA_PERFMON_SEL_META_L2_REQ_SEND        = 0x00000057u32,
SDMA_PERFMON_SEL_L2_META_RET_VLD         = 0x00000058u32,
SDMA_PERFMON_SEL_SDMA_UTCL2_RD_SEND      = 0x00000059u32,
SDMA_PERFMON_SEL_UTCL2_SDMA_RD_RTN       = 0x0000005au32,
SDMA_PERFMON_SEL_SDMA_UTCL2_WR_SEND      = 0x0000005bu32,
SDMA_PERFMON_SEL_UTCL2_SDMA_WR_RTN       = 0x0000005cu32,
SDMA_PERFMON_SEL_META_REQ_SEND           = 0x0000005du32,
SDMA_PERFMON_SEL_META_RTN_VLD            = 0x0000005eu32,
SDMA_PERFMON_SEL_TLBI_SEND               = 0x0000005fu32,
SDMA_PERFMON_SEL_TLBI_RTN                = 0x00000060u32,
SDMA_PERFMON_SEL_GCR_SEND                = 0x00000061u32,
SDMA_PERFMON_SEL_GCR_RTN                 = 0x00000062u32,
SDMA_PERFMON_SEL_UTCL1_TAG_DELAY_COUNTER = 0x00000063u32,
SDMA_PERFMON_SEL_MMHUB_TAG_DELAY_COUNTER = 0x00000064u32,
}

/*
 * SDMA_PERF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SDMA_PERF_SEL {
SDMA_PERF_SEL_CYCLE                      = 0x00000000u32,
SDMA_PERF_SEL_IDLE                       = 0x00000001u32,
SDMA_PERF_SEL_REG_IDLE                   = 0x00000002u32,
SDMA_PERF_SEL_RB_EMPTY                   = 0x00000003u32,
SDMA_PERF_SEL_RB_FULL                    = 0x00000004u32,
SDMA_PERF_SEL_RB_WPTR_WRAP               = 0x00000005u32,
SDMA_PERF_SEL_RB_RPTR_WRAP               = 0x00000006u32,
SDMA_PERF_SEL_RB_WPTR_POLL_READ          = 0x00000007u32,
SDMA_PERF_SEL_RB_RPTR_WB                 = 0x00000008u32,
SDMA_PERF_SEL_RB_CMD_IDLE                = 0x00000009u32,
SDMA_PERF_SEL_RB_CMD_FULL                = 0x0000000au32,
SDMA_PERF_SEL_IB_CMD_IDLE                = 0x0000000bu32,
SDMA_PERF_SEL_IB_CMD_FULL                = 0x0000000cu32,
SDMA_PERF_SEL_EX_IDLE                    = 0x0000000du32,
SDMA_PERF_SEL_SRBM_REG_SEND              = 0x0000000eu32,
SDMA_PERF_SEL_EX_IDLE_POLL_TIMER_EXPIRE  = 0x0000000fu32,
SDMA_PERF_SEL_MC_WR_IDLE                 = 0x00000010u32,
SDMA_PERF_SEL_MC_WR_COUNT                = 0x00000011u32,
SDMA_PERF_SEL_MC_RD_IDLE                 = 0x00000012u32,
SDMA_PERF_SEL_MC_RD_COUNT                = 0x00000013u32,
SDMA_PERF_SEL_MC_RD_RET_STALL            = 0x00000014u32,
SDMA_PERF_SEL_MC_RD_NO_POLL_IDLE         = 0x00000015u32,
SDMA_PERF_SEL_SEM_IDLE                   = 0x00000018u32,
SDMA_PERF_SEL_SEM_REQ_STALL              = 0x00000019u32,
SDMA_PERF_SEL_SEM_REQ_COUNT              = 0x0000001au32,
SDMA_PERF_SEL_SEM_RESP_INCOMPLETE        = 0x0000001bu32,
SDMA_PERF_SEL_SEM_RESP_FAIL              = 0x0000001cu32,
SDMA_PERF_SEL_SEM_RESP_PASS              = 0x0000001du32,
SDMA_PERF_SEL_INT_IDLE                   = 0x0000001eu32,
SDMA_PERF_SEL_INT_REQ_STALL              = 0x0000001fu32,
SDMA_PERF_SEL_INT_REQ_COUNT              = 0x00000020u32,
SDMA_PERF_SEL_INT_RESP_ACCEPTED          = 0x00000021u32,
SDMA_PERF_SEL_INT_RESP_RETRY             = 0x00000022u32,
SDMA_PERF_SEL_NUM_PACKET                 = 0x00000023u32,
SDMA_PERF_SEL_CE_WREQ_IDLE               = 0x00000025u32,
SDMA_PERF_SEL_CE_WR_IDLE                 = 0x00000026u32,
SDMA_PERF_SEL_CE_SPLIT_IDLE              = 0x00000027u32,
SDMA_PERF_SEL_CE_RREQ_IDLE               = 0x00000028u32,
SDMA_PERF_SEL_CE_OUT_IDLE                = 0x00000029u32,
SDMA_PERF_SEL_CE_IN_IDLE                 = 0x0000002au32,
SDMA_PERF_SEL_CE_DST_IDLE                = 0x0000002bu32,
SDMA_PERF_SEL_CE_AFIFO_FULL              = 0x0000002eu32,
SDMA_PERF_SEL_DUMMY_0                    = 0x0000002fu32,
SDMA_PERF_SEL_DUMMY_1                    = 0x00000030u32,
SDMA_PERF_SEL_CE_INFO_FULL               = 0x00000031u32,
SDMA_PERF_SEL_CE_INFO1_FULL              = 0x00000032u32,
SDMA_PERF_SEL_CE_RD_STALL                = 0x00000033u32,
SDMA_PERF_SEL_CE_WR_STALL                = 0x00000034u32,
SDMA_PERF_SEL_QUEUE0_SELECT              = 0x00000035u32,
SDMA_PERF_SEL_QUEUE1_SELECT              = 0x00000036u32,
SDMA_PERF_SEL_QUEUE2_SELECT              = 0x00000037u32,
SDMA_PERF_SEL_QUEUE3_SELECT              = 0x00000038u32,
SDMA_PERF_SEL_CTX_CHANGE                 = 0x00000039u32,
SDMA_PERF_SEL_CTX_CHANGE_EXPIRED         = 0x0000003au32,
SDMA_PERF_SEL_CTX_CHANGE_EXCEPTION       = 0x0000003bu32,
SDMA_PERF_SEL_DOORBELL                   = 0x0000003cu32,
SDMA_PERF_SEL_RD_BA_RTR                  = 0x0000003du32,
SDMA_PERF_SEL_WR_BA_RTR                  = 0x0000003eu32,
SDMA_PERF_SEL_MCU_L1_WR_VLD              = 0x0000003fu32,
SDMA_PERF_SEL_CE_L1_WR_VLD               = 0x00000040u32,
SDMA_PERF_SEL_CPF_SDMA_INVREQ            = 0x00000041u32,
SDMA_PERF_SEL_SDMA_CPF_INVACK            = 0x00000042u32,
SDMA_PERF_SEL_UTCL2_SDMA_INVREQ          = 0x00000043u32,
SDMA_PERF_SEL_SDMA_UTCL2_INVACK          = 0x00000044u32,
SDMA_PERF_SEL_UTCL2_SDMA_INVREQ_ALL      = 0x00000045u32,
SDMA_PERF_SEL_SDMA_UTCL2_INVACK_ALL      = 0x00000046u32,
SDMA_PERF_SEL_UTCL2_RET_XNACK            = 0x00000047u32,
SDMA_PERF_SEL_UTCL2_RET_ACK              = 0x00000048u32,
SDMA_PERF_SEL_UTCL2_FREE                 = 0x00000049u32,
SDMA_PERF_SEL_SDMA_UTCL2_SEND            = 0x0000004au32,
SDMA_PERF_SEL_DMA_L1_WR_SEND             = 0x0000004bu32,
SDMA_PERF_SEL_DMA_L1_RD_SEND             = 0x0000004cu32,
SDMA_PERF_SEL_DMA_MC_WR_SEND             = 0x0000004du32,
SDMA_PERF_SEL_DMA_MC_RD_SEND             = 0x0000004eu32,
SDMA_PERF_SEL_GPUVM_INV_HIGH             = 0x0000004fu32,
SDMA_PERF_SEL_GPUVM_INV_LOW              = 0x00000050u32,
SDMA_PERF_SEL_L1_WRL2_IDLE               = 0x00000051u32,
SDMA_PERF_SEL_L1_RDL2_IDLE               = 0x00000052u32,
SDMA_PERF_SEL_L1_WRMC_IDLE               = 0x00000053u32,
SDMA_PERF_SEL_L1_RDMC_IDLE               = 0x00000054u32,
SDMA_PERF_SEL_L1_WR_INV_IDLE             = 0x00000055u32,
SDMA_PERF_SEL_L1_RD_INV_IDLE             = 0x00000056u32,
SDMA_PERF_SEL_META_L2_REQ_SEND           = 0x00000057u32,
SDMA_PERF_SEL_L2_META_RET_VLD            = 0x00000058u32,
SDMA_PERF_SEL_SDMA_UTCL2_RD_SEND         = 0x00000059u32,
SDMA_PERF_SEL_UTCL2_SDMA_RD_RTN          = 0x0000005au32,
SDMA_PERF_SEL_SDMA_UTCL2_WR_SEND         = 0x0000005bu32,
SDMA_PERF_SEL_UTCL2_SDMA_WR_RTN          = 0x0000005cu32,
SDMA_PERF_SEL_META_REQ_SEND              = 0x0000005du32,
SDMA_PERF_SEL_META_RTN_VLD               = 0x0000005eu32,
SDMA_PERF_SEL_TLBI_SEND                  = 0x0000005fu32,
SDMA_PERF_SEL_TLBI_RTN                   = 0x00000060u32,
SDMA_PERF_SEL_GCR_SEND                   = 0x00000061u32,
SDMA_PERF_SEL_GCR_RTN                    = 0x00000062u32,
SDMA_PERF_SEL_CGCG_FENCE                 = 0x00000063u32,
SDMA_PERF_SEL_CE_CH_WR_REQ               = 0x00000064u32,
SDMA_PERF_SEL_CE_CH_WR_RET               = 0x00000065u32,
SDMA_PERF_SEL_MCU_CH_WR_REQ              = 0x00000066u32,
SDMA_PERF_SEL_MCU_CH_WR_RET              = 0x00000067u32,
SDMA_PERF_SEL_CE_OR_MCU_CH_RD_REQ        = 0x00000068u32,
SDMA_PERF_SEL_CE_OR_MCU_CH_RD_RET        = 0x00000069u32,
SDMA_PERF_SEL_RB_CH_RD_REQ               = 0x0000006au32,
SDMA_PERF_SEL_RB_CH_RD_RET               = 0x0000006bu32,
SDMA_PERF_SEL_IB_CH_RD_REQ               = 0x0000006cu32,
SDMA_PERF_SEL_IB_CH_RD_RET               = 0x0000006du32,
SDMA_PERF_SEL_WPTR_CH_RD_REQ             = 0x0000006eu32,
SDMA_PERF_SEL_WPTR_CH_RD_RET             = 0x0000006fu32,
SDMA_PERF_SEL_UTCL1_UTCL2_REQ            = 0x00000070u32,
SDMA_PERF_SEL_UTCL1_UTCL2_RET            = 0x00000071u32,
SDMA_PERF_SEL_CMD_OP_MATCH               = 0x00000072u32,
SDMA_PERF_SEL_CMD_OP_START               = 0x00000073u32,
SDMA_PERF_SEL_CMD_OP_END                 = 0x00000074u32,
SDMA_PERF_SEL_CE_BUSY                    = 0x00000075u32,
SDMA_PERF_SEL_CE_BUSY_START              = 0x00000076u32,
SDMA_PERF_SEL_CE_BUSY_END                = 0x00000077u32,
SDMA_PERF_SEL_MCU_PERFCNT_TRIGGER        = 0x00000078u32,
SDMA_PERF_SEL_MCU_PERFCNT_TRIGGER_START  = 0x00000079u32,
SDMA_PERF_SEL_MCU_PERFCNT_TRIGGER_END    = 0x0000007au32,
SDMA_PERF_SEL_CE_CH_WRREQ_SEND           = 0x0000007bu32,
SDMA_PERF_SEL_CH_CE_WRRET_VALID          = 0x0000007cu32,
SDMA_PERF_SEL_CE_CH_RDREQ_SEND           = 0x0000007du32,
SDMA_PERF_SEL_CH_CE_RDRET_VALID          = 0x0000007eu32,
SDMA_PERF_SEL_QUEUE4_SELECT              = 0x0000007fu32,
SDMA_PERF_SEL_QUEUE5_SELECT              = 0x00000080u32,
SDMA_PERF_SEL_QUEUE6_SELECT              = 0x00000081u32,
SDMA_PERF_SEL_QUEUE7_SELECT              = 0x00000082u32,
}

/*
 * SPM_PERFMON_STATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SPM_PERFMON_STATE {
STRM_PERFMON_STATE_DISABLE_AND_RESET     = 0x00000000u32,
STRM_PERFMON_STATE_START_COUNTING        = 0x00000001u32,
STRM_PERFMON_STATE_STOP_COUNTING         = 0x00000002u32,
STRM_PERFMON_STATE_RESERVED_3            = 0x00000003u32,
STRM_PERFMON_STATE_DISABLE_AND_RESET_PHANTOM = 0x00000004u32,
STRM_PERFMON_STATE_COUNT_AND_DUMP_PHANTOM = 0x00000005u32,
}

/*
 * TCC_MTYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TCC_MTYPE {
MTYPE_NC                                 = 0x00000000u32,
MTYPE_WC                                 = 0x00000001u32,
MTYPE_CC                                 = 0x00000002u32,
}

/*
 * UTCL0FaultType enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum UTCL0FaultType {
UTCL0_XNACK_SUCCESS                      = 0x00000000u32,
UTCL0_XNACK_RETRY                        = 0x00000001u32,
UTCL0_XNACK_PRT                          = 0x00000002u32,
UTCL0_XNACK_NO_RETRY                     = 0x00000003u32,
}

/*
 * UTCL0RequestType enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum UTCL0RequestType {
UTCL0_TYPE_NORMAL                        = 0x00000000u32,
UTCL0_TYPE_SHOOTDOWN                     = 0x00000001u32,
UTCL0_TYPE_BYPASS                        = 0x00000002u32,
}

/*
 * UTCL1FaultType enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum UTCL1FaultType {
UTCL1_XNACK_SUCCESS                      = 0x00000000u32,
UTCL1_XNACK_RETRY                        = 0x00000001u32,
UTCL1_XNACK_PRT                          = 0x00000002u32,
UTCL1_XNACK_NO_RETRY                     = 0x00000003u32,
}

/*
 * UTCL1RequestType enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum UTCL1RequestType {
UTCL1_TYPE_NORMAL                        = 0x00000000u32,
UTCL1_TYPE_SHOOTDOWN                     = 0x00000001u32,
UTCL1_TYPE_BYPASS                        = 0x00000002u32,
}

/*
 * WRITE_COMPRESSION_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum WRITE_COMPRESSION_MODE {
COMPRESSION_MODE_BYPASS_METADATA_CACHE   = 0x00000000u32,
COMPRESSION_MODE_COMPRESSION_ENABLED     = 0x00000001u32,
COMPRESSION_MODE_WRITE_COMPRESSION_DISABLED = 0x00000002u32,
}

/*
 * WritePolicy enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum WritePolicy {
CACHE_LRU_WR                             = 0x00000000u32,
CACHE_STREAM                             = 0x00000001u32,
CACHE_NOA_WR                             = 0x00000002u32,
CACHE_BYPASS                             = 0x00000003u32,
}

/*
 * COLOR_KEYER_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum COLOR_KEYER_ENABLE {
COLOR_KEY_EN                             = 0x00000000u32,
COLOR_KEY_DIS                            = 0x00000001u32,
}

/*
 * COLOR_KEYER_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum COLOR_KEYER_MODE {
FORCE_00                                 = 0x00000000u32,
FORCE_FF                                 = 0x00000001u32,
RANGE_00                                 = 0x00000002u32,
RANGE_FF                                 = 0x00000003u32,
}

/*
 * DENORM_TRUNCATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DENORM_TRUNCATE {
CNVC_ROUND                               = 0x00000000u32,
CNVC_TRUNCATE                            = 0x00000001u32,
}

/*
 * FORMAT_CROSSBAR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FORMAT_CROSSBAR {
FORMAT_CROSSBAR_R                        = 0x00000000u32,
FORMAT_CROSSBAR_G                        = 0x00000001u32,
FORMAT_CROSSBAR_B                        = 0x00000002u32,
}

/*
 * LUMA_KEYER_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum LUMA_KEYER_ENABLE {
LUMA_KEY_EN                              = 0x00000000u32,
LUMA_KEY_DIS                             = 0x00000001u32,
}

/*
 * PIX_EXPAND_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PIX_EXPAND_MODE {
PIX_DYNAMIC_EXPANSION                    = 0x00000000u32,
PIX_ZERO_EXPANSION                       = 0x00000001u32,
}

/*
 * PRE_CSC_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PRE_CSC_MODE_ENUM {
PRE_CSC_BYPASS                           = 0x00000000u32,
PRE_CSC_SET_A                            = 0x00000001u32,
PRE_CSC_SET_B                            = 0x00000002u32,
}

/*
 * PRE_DEGAM_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PRE_DEGAM_MODE {
PRE_DEGAM_BYPASS                         = 0x00000000u32,
PRE_DEGAM_ENABLE                         = 0x00000001u32,
}

/*
 * PRE_DEGAM_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PRE_DEGAM_SELECT {
PRE_DEGAM_SRGB                           = 0x00000000u32,
PRE_DEGAM_GAMMA_22                       = 0x00000001u32,
PRE_DEGAM_GAMMA_24                       = 0x00000002u32,
PRE_DEGAM_GAMMA_26                       = 0x00000003u32,
PRE_DEGAM_BT2020                         = 0x00000004u32,
PRE_DEGAM_BT2100PQ                       = 0x00000005u32,
PRE_DEGAM_BT2100HLG                      = 0x00000006u32,
}

/*
 * SURFACE_PIXEL_FORMAT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SURFACE_PIXEL_FORMAT {
ARGB1555                                 = 0x00000001u32,
RGBA5551                                 = 0x00000002u32,
RGB565                                   = 0x00000003u32,
BGR565                                   = 0x00000004u32,
ARGB4444                                 = 0x00000005u32,
RGBA4444                                 = 0x00000006u32,
ARGB8888                                 = 0x00000008u32,
RGBA8888                                 = 0x00000009u32,
ARGB2101010                              = 0x0000000au32,
RGBA1010102                              = 0x0000000bu32,
AYCrCb8888                               = 0x0000000cu32,
YCrCbA8888                               = 0x0000000du32,
ACrYCb8888                               = 0x0000000eu32,
CrYCbA8888                               = 0x0000000fu32,
ARGB16161616_10MSB                       = 0x00000010u32,
RGBA16161616_10MSB                       = 0x00000011u32,
ARGB16161616_10LSB                       = 0x00000012u32,
RGBA16161616_10LSB                       = 0x00000013u32,
ARGB16161616_12MSB                       = 0x00000014u32,
RGBA16161616_12MSB                       = 0x00000015u32,
ARGB16161616_12LSB                       = 0x00000016u32,
RGBA16161616_12LSB                       = 0x00000017u32,
ARGB16161616_FLOAT                       = 0x00000018u32,
RGBA16161616_FLOAT                       = 0x00000019u32,
ARGB16161616_UNORM                       = 0x0000001au32,
RGBA16161616_UNORM                       = 0x0000001bu32,
ARGB16161616_SNORM                       = 0x0000001cu32,
RGBA16161616_SNORM                       = 0x0000001du32,
AYCrCb16161616_10MSB                     = 0x00000020u32,
AYCrCb16161616_10LSB                     = 0x00000021u32,
YCrCbA16161616_10MSB                     = 0x00000022u32,
YCrCbA16161616_10LSB                     = 0x00000023u32,
ACrYCb16161616_10MSB                     = 0x00000024u32,
ACrYCb16161616_10LSB                     = 0x00000025u32,
CrYCbA16161616_10MSB                     = 0x00000026u32,
CrYCbA16161616_10LSB                     = 0x00000027u32,
AYCrCb16161616_12MSB                     = 0x00000028u32,
AYCrCb16161616_12LSB                     = 0x00000029u32,
YCrCbA16161616_12MSB                     = 0x0000002au32,
YCrCbA16161616_12LSB                     = 0x0000002bu32,
ACrYCb16161616_12MSB                     = 0x0000002cu32,
ACrYCb16161616_12LSB                     = 0x0000002du32,
CrYCbA16161616_12MSB                     = 0x0000002eu32,
CrYCbA16161616_12LSB                     = 0x0000002fu32,
Y8_CrCb88_420_PLANAR                     = 0x00000040u32,
Y8_CbCr88_420_PLANAR                     = 0x00000041u32,
Y10_CrCb1010_420_PLANAR                  = 0x00000042u32,
Y10_CbCr1010_420_PLANAR                  = 0x00000043u32,
Y12_CrCb1212_420_PLANAR                  = 0x00000044u32,
Y12_CbCr1212_420_PLANAR                  = 0x00000045u32,
YCrYCb8888_422_PACKED                    = 0x00000048u32,
YCbYCr8888_422_PACKED                    = 0x00000049u32,
CrYCbY8888_422_PACKED                    = 0x0000004au32,
CbYCrY8888_422_PACKED                    = 0x0000004bu32,
YCrYCb10101010_422_PACKED                = 0x0000004cu32,
YCbYCr10101010_422_PACKED                = 0x0000004du32,
CrYCbY10101010_422_PACKED                = 0x0000004eu32,
CbYCrY10101010_422_PACKED                = 0x0000004fu32,
YCrYCb12121212_422_PACKED                = 0x00000050u32,
YCbYCr12121212_422_PACKED                = 0x00000051u32,
CrYCbY12121212_422_PACKED                = 0x00000052u32,
CbYCrY12121212_422_PACKED                = 0x00000053u32,
RGB111110_FIX                            = 0x00000070u32,
BGR101111_FIX                            = 0x00000071u32,
ACrYCb2101010                            = 0x00000072u32,
CrYCbA1010102                            = 0x00000073u32,
RGBE                                     = 0x00000074u32,
RGB111110_FLOAT                          = 0x00000076u32,
BGR101111_FLOAT                          = 0x00000077u32,
MONO_8                                   = 0x00000078u32,
MONO_10MSB                               = 0x00000079u32,
MONO_10LSB                               = 0x0000007au32,
MONO_12MSB                               = 0x0000007bu32,
MONO_12LSB                               = 0x0000007cu32,
MONO_16                                  = 0x0000007du32,
}

/*
 * XNORM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum XNORM {
XNORM_A                                  = 0x00000000u32,
XNORM_B                                  = 0x00000001u32,
}

/*
 * CUR_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CUR_ENABLE {
CUR_DIS                                  = 0x00000000u32,
CUR_EN                                   = 0x00000001u32,
}

/*
 * CUR_EXPAND_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CUR_EXPAND_MODE {
CUR_DYNAMIC_EXPANSION                    = 0x00000000u32,
CUR_ZERO_EXPANSION                       = 0x00000001u32,
}

/*
 * CUR_INV_CLAMP enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CUR_INV_CLAMP {
CUR_CLAMP_DIS                            = 0x00000000u32,
CUR_CLAMP_EN                             = 0x00000001u32,
}

/*
 * CUR_MATRIX_COEF_FORMAT_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CUR_MATRIX_COEF_FORMAT_ENUM {
CUR_MATRIX_FIX_S2_13                     = 0x00000000u32,
CUR_MATRIX_FIX_S3_12                     = 0x00000001u32,
}

/*
 * CUR_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CUR_MODE {
MONO_2BIT                                = 0x00000000u32,
COLOR_24BIT_1BIT_AND                     = 0x00000001u32,
COLOR_24BIT_8BIT_ALPHA_PREMULT           = 0x00000002u32,
COLOR_24BIT_8BIT_ALPHA_UNPREMULT         = 0x00000003u32,
COLOR_64BIT_FP_PREMULT                   = 0x00000004u32,
COLOR_64BIT_FP_UNPREMULT                 = 0x00000005u32,
}

/*
 * CUR_PENDING enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CUR_PENDING {
CUR_NOT_PENDING                          = 0x00000000u32,
CUR_YES_PENDING                          = 0x00000001u32,
}

/*
 * CUR_ROM_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CUR_ROM_EN {
CUR_FP_NO_ROM                            = 0x00000000u32,
CUR_FP_USE_ROM                           = 0x00000001u32,
}

/*
 * COEF_RAM_SELECT_RD enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum COEF_RAM_SELECT_RD {
COEF_RAM_SELECT_BACK                     = 0x00000000u32,
COEF_RAM_SELECT_CURRENT                  = 0x00000001u32,
}

/*
 * DSCL_MODE_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DSCL_MODE_SEL {
DSCL_MODE_SCALING_444_BYPASS             = 0x00000000u32,
DSCL_MODE_SCALING_444_RGB_ENABLE         = 0x00000001u32,
DSCL_MODE_SCALING_444_YCBCR_ENABLE       = 0x00000002u32,
DSCL_MODE_SCALING_YCBCR_ENABLE           = 0x00000003u32,
DSCL_MODE_LUMA_SCALING_BYPASS            = 0x00000004u32,
DSCL_MODE_CHROMA_SCALING_BYPASS          = 0x00000005u32,
DSCL_MODE_DSCL_BYPASS                    = 0x00000006u32,
}

/*
 * ISHARP_FMT_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ISHARP_FMT_MODE_ENUM {
ISHARP_FMT_MODE_0                        = 0x00000000u32,
ISHARP_FMT_MODE_1                        = 0x00000001u32,
}

/*
 * ISHARP_LBA_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ISHARP_LBA_MODE_ENUM {
ISHARP_LBA_MODE_0                        = 0x00000000u32,
ISHARP_LBA_MODE_1                        = 0x00000001u32,
}

/*
 * ISHARP_NOISEDET_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ISHARP_NOISEDET_MODE_ENUM {
ISHARP_NOISEDET_MODE_0                   = 0x00000000u32,
ISHARP_NOISEDET_MODE_1                   = 0x00000001u32,
ISHARP_NOISEDET_MODE_2                   = 0x00000002u32,
ISHARP_NOISEDET_MODE_3                   = 0x00000003u32,
}

/*
 * LB_ALPHA_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum LB_ALPHA_EN {
LB_ALPHA_DISABLE                         = 0x00000000u32,
LB_ALPHA_ENABLE                          = 0x00000001u32,
}

/*
 * LB_INTERLEAVE_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum LB_INTERLEAVE_EN {
LB_INTERLEAVE_DISABLE                    = 0x00000000u32,
LB_INTERLEAVE_ENABLE                     = 0x00000001u32,
}

/*
 * LB_MEMORY_CONFIG enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum LB_MEMORY_CONFIG {
LB_MEMORY_CONFIG_0                       = 0x00000000u32,
LB_MEMORY_CONFIG_1                       = 0x00000001u32,
LB_MEMORY_CONFIG_2                       = 0x00000002u32,
LB_MEMORY_CONFIG_3                       = 0x00000003u32,
}

/*
 * MATRIX_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MATRIX_MODE_ENUM {
MATRIX_MODE_0                            = 0x00000000u32,
MATRIX_MODE_1                            = 0x00000001u32,
}

/*
 * OBUF_BYPASS_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OBUF_BYPASS_SEL {
OBUF_BYPASS_DIS                          = 0x00000000u32,
OBUF_BYPASS_EN                           = 0x00000001u32,
}

/*
 * OBUF_IS_HALF_RECOUT_WIDTH_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OBUF_IS_HALF_RECOUT_WIDTH_SEL {
OBUF_FULL_RECOUT                         = 0x00000000u32,
OBUF_HALF_RECOUT                         = 0x00000001u32,
}

/*
 * OBUF_USE_FULL_BUFFER_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OBUF_USE_FULL_BUFFER_SEL {
OBUF_RECOUT                              = 0x00000000u32,
OBUF_FULL                                = 0x00000001u32,
}

/*
 * SCL_2TAP_HARDCODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SCL_2TAP_HARDCODE {
SCL_COEF_2TAP_HARDCODE_OFF               = 0x00000000u32,
SCL_COEF_2TAP_HARDCODE_ON                = 0x00000001u32,
}

/*
 * SCL_ALPHA_COEF enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SCL_ALPHA_COEF {
SCL_ALPHA_COEF_FIRST                     = 0x00000000u32,
SCL_ALPHA_COEF_SECOND                    = 0x00000001u32,
}

/*
 * SCL_AUTOCAL_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SCL_AUTOCAL_MODE {
AUTOCAL_MODE_OFF                         = 0x00000000u32,
AUTOCAL_MODE_AUTOSCALE                   = 0x00000001u32,
AUTOCAL_MODE_AUTOCENTER                  = 0x00000002u32,
AUTOCAL_MODE_AUTOREPLICATE               = 0x00000003u32,
}

/*
 * SCL_BOUNDARY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SCL_BOUNDARY {
SCL_BOUNDARY_EDGE                        = 0x00000000u32,
SCL_BOUNDARY_BLACK                       = 0x00000001u32,
}

/*
 * SCL_CHROMA_COEF enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SCL_CHROMA_COEF {
SCL_CHROMA_COEF_FIRST                    = 0x00000000u32,
SCL_CHROMA_COEF_SECOND                   = 0x00000001u32,
}

/*
 * SCL_COEF_FILTER_TYPE_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SCL_COEF_FILTER_TYPE_SEL {
SCL_COEF_LUMA_VERT_FILTER                = 0x00000000u32,
SCL_COEF_LUMA_HORZ_FILTER                = 0x00000001u32,
SCL_COEF_CHROMA_VERT_FILTER              = 0x00000002u32,
SCL_COEF_CHROMA_HORZ_FILTER              = 0x00000003u32,
SCL_COEF_SC_VERT_FILTER                  = 0x00000004u32,
SCL_COEF_SC_HORZ_FILTER                  = 0x00000005u32,
}

/*
 * SCL_COEF_RAM_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SCL_COEF_RAM_SEL {
SCL_COEF_RAM_SEL_0                       = 0x00000000u32,
SCL_COEF_RAM_SEL_1                       = 0x00000001u32,
}

/*
 * SCL_SHARP_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SCL_SHARP_EN {
SCL_SHARP_DISABLE                        = 0x00000000u32,
SCL_SHARP_ENABLE                         = 0x00000001u32,
}

/*******************************************************
 * CM Enums
 *******************************************************/

/*
 * CMC_3DLUT_30BIT_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CMC_3DLUT_30BIT_ENUM {
CMC_3DLUT_36BIT                          = 0x00000000u32,
CMC_3DLUT_30BIT                          = 0x00000001u32,
}

/*
 * CMC_3DLUT_RAM_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CMC_3DLUT_RAM_SEL {
CMC_RAM0_ACCESS                          = 0x00000000u32,
CMC_RAM1_ACCESS                          = 0x00000001u32,
CMC_RAM2_ACCESS                          = 0x00000002u32,
CMC_RAM3_ACCESS                          = 0x00000003u32,
}

/*
 * CMC_3DLUT_SIZE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CMC_3DLUT_SIZE_ENUM {
CMC_3DLUT_17CUBE                         = 0x00000000u32,
CMC_3DLUT_9CUBE                          = 0x00000001u32,
}

/*
 * CMC_LUT_2_CONFIG_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CMC_LUT_2_CONFIG_ENUM {
CMC_LUT_2CFG_NO_MEMORY                   = 0x00000000u32,
CMC_LUT_2CFG_MEMORY_A                    = 0x00000001u32,
CMC_LUT_2CFG_MEMORY_B                    = 0x00000002u32,
}

/*
 * CMC_LUT_2_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CMC_LUT_2_MODE_ENUM {
CMC_LUT_2_MODE_BYPASS                    = 0x00000000u32,
CMC_LUT_2_MODE_RAMA_LUT                  = 0x00000001u32,
CMC_LUT_2_MODE_RAMB_LUT                  = 0x00000002u32,
}

/*
 * CMC_LUT_NUM_SEG enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CMC_LUT_NUM_SEG {
CMC_SEGMENTS_1                           = 0x00000000u32,
CMC_SEGMENTS_2                           = 0x00000001u32,
CMC_SEGMENTS_4                           = 0x00000002u32,
CMC_SEGMENTS_8                           = 0x00000003u32,
CMC_SEGMENTS_16                          = 0x00000004u32,
CMC_SEGMENTS_32                          = 0x00000005u32,
CMC_SEGMENTS_64                          = 0x00000006u32,
CMC_SEGMENTS_128                         = 0x00000007u32,
}

/*
 * CMC_LUT_RAM_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CMC_LUT_RAM_SEL {
CMC_RAMA_ACCESS                          = 0x00000000u32,
CMC_RAMB_ACCESS                          = 0x00000001u32,
}

/*
 * CM_BYPASS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_BYPASS {
NON_BYPASS                               = 0x00000000u32,
BYPASS_EN                                = 0x00000001u32,
}

/*
 * CM_COEF_FORMAT_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_COEF_FORMAT_ENUM {
FIX_S2_13                                = 0x00000000u32,
FIX_S3_12                                = 0x00000001u32,
}

/*
 * CM_DATA_SIGNED enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_DATA_SIGNED {
UNSIGNED                                 = 0x00000000u32,
SIGNED                                   = 0x00000001u32,
}

/*
 * CM_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_EN {
CM_DISABLE                               = 0x00000000u32,
CM_ENABLE                                = 0x00000001u32,
}

/*
 * CM_GAMMA_LUT_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_GAMMA_LUT_MODE_ENUM {
BYPASS                                   = 0x00000000u32,
RESERVED_1                               = 0x00000001u32,
RAM_LUT                                  = 0x00000002u32,
RESERVED_3                               = 0x00000003u32,
}

/*
 * CM_GAMMA_LUT_PWL_DISABLE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_GAMMA_LUT_PWL_DISABLE_ENUM {
ENABLE_PWL                               = 0x00000000u32,
DISABLE_PWL                              = 0x00000001u32,
}

/*
 * CM_GAMMA_LUT_SEL_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_GAMMA_LUT_SEL_ENUM {
RAMA                                     = 0x00000000u32,
RAMB                                     = 0x00000001u32,
}

/*
 * CM_LUT_2_CONFIG_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_LUT_2_CONFIG_ENUM {
LUT_2CFG_NO_MEMORY                       = 0x00000000u32,
LUT_2CFG_MEMORY_A                        = 0x00000001u32,
LUT_2CFG_MEMORY_B                        = 0x00000002u32,
}

/*
 * CM_LUT_2_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_LUT_2_MODE_ENUM {
LUT_2_MODE_BYPASS                        = 0x00000000u32,
LUT_2_MODE_RAMA_LUT                      = 0x00000001u32,
LUT_2_MODE_RAMB_LUT                      = 0x00000002u32,
}

/*
 * CM_LUT_4_CONFIG_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_LUT_4_CONFIG_ENUM {
LUT_4CFG_NO_MEMORY                       = 0x00000000u32,
LUT_4CFG_ROM_A                           = 0x00000001u32,
LUT_4CFG_ROM_B                           = 0x00000002u32,
LUT_4CFG_MEMORY_A                        = 0x00000003u32,
LUT_4CFG_MEMORY_B                        = 0x00000004u32,
}

/*
 * CM_LUT_4_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_LUT_4_MODE_ENUM {
LUT_4_MODE_BYPASS                        = 0x00000000u32,
LUT_4_MODE_ROMA_LUT                      = 0x00000001u32,
LUT_4_MODE_ROMB_LUT                      = 0x00000002u32,
LUT_4_MODE_RAMA_LUT                      = 0x00000003u32,
LUT_4_MODE_RAMB_LUT                      = 0x00000004u32,
}

/*
 * CM_LUT_CONFIG_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_LUT_CONFIG_MODE {
DIFFERENT_RGB                            = 0x00000000u32,
ALL_USE_R                                = 0x00000001u32,
}

/*
 * CM_LUT_NUM_SEG enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_LUT_NUM_SEG {
SEGMENTS_1                               = 0x00000000u32,
SEGMENTS_2                               = 0x00000001u32,
SEGMENTS_4                               = 0x00000002u32,
SEGMENTS_8                               = 0x00000003u32,
SEGMENTS_16                              = 0x00000004u32,
SEGMENTS_32                              = 0x00000005u32,
SEGMENTS_64                              = 0x00000006u32,
SEGMENTS_128                             = 0x00000007u32,
}

/*
 * CM_LUT_RAM_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_LUT_RAM_SEL {
RAMA_ACCESS                              = 0x00000000u32,
RAMB_ACCESS                              = 0x00000001u32,
}

/*
 * CM_LUT_READ_COLOR_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_LUT_READ_COLOR_SEL {
BLUE_LUT                                 = 0x00000000u32,
GREEN_LUT                                = 0x00000001u32,
RED_LUT                                  = 0x00000002u32,
}

/*
 * CM_LUT_READ_DBG enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_LUT_READ_DBG {
DISABLE_DEBUG                            = 0x00000000u32,
ENABLE_DEBUG                             = 0x00000001u32,
}

/*
 * CM_PENDING enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_PENDING {
CM_NOT_PENDING                           = 0x00000000u32,
CM_YES_PENDING                           = 0x00000001u32,
}

/*
 * CM_POST_CSC_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_POST_CSC_MODE_ENUM {
BYPASS_POST_CSC                          = 0x00000000u32,
COEF_POST_CSC                            = 0x00000001u32,
COEF_POST_CSC_B                          = 0x00000002u32,
}

/*
 * CM_WRITE_BASE_ONLY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CM_WRITE_BASE_ONLY {
WRITE_BOTH                               = 0x00000000u32,
WRITE_BASE_ONLY                          = 0x00000001u32,
}

/*******************************************************
 * DPP_TOP Enums
 *******************************************************/

/*
 * CRC_CUR_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CRC_CUR_SEL {
CRC_CUR_0                                = 0x00000000u32,
CRC_CUR_1                                = 0x00000001u32,
}

/*
 * CRC_INTERLACE_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CRC_INTERLACE_SEL {
CRC_INTERLACE_0                          = 0x00000000u32,
CRC_INTERLACE_1                          = 0x00000001u32,
CRC_INTERLACE_2                          = 0x00000002u32,
CRC_INTERLACE_3                          = 0x00000003u32,
}

/*
 * CRC_IN_PIX_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CRC_IN_PIX_SEL {
CRC_IN_PIX_0                             = 0x00000000u32,
CRC_IN_PIX_1                             = 0x00000001u32,
CRC_IN_PIX_2                             = 0x00000002u32,
CRC_IN_PIX_3                             = 0x00000003u32,
CRC_IN_PIX_4                             = 0x00000004u32,
CRC_IN_PIX_5                             = 0x00000005u32,
CRC_IN_PIX_6                             = 0x00000006u32,
CRC_IN_PIX_7                             = 0x00000007u32,
}

/*
 * CRC_SRC_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CRC_SRC_SEL {
CRC_SRC_0                                = 0x00000000u32,
CRC_SRC_1                                = 0x00000001u32,
CRC_SRC_2                                = 0x00000002u32,
CRC_SRC_3                                = 0x00000003u32,
}

/*
 * CRC_STEREO_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CRC_STEREO_SEL {
CRC_STEREO_0                             = 0x00000000u32,
CRC_STEREO_1                             = 0x00000001u32,
CRC_STEREO_2                             = 0x00000002u32,
CRC_STEREO_3                             = 0x00000003u32,
}

/*
 * TEST_CLK_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TEST_CLK_SEL {
TEST_CLK_SEL_0                           = 0x00000000u32,
TEST_CLK_SEL_1                           = 0x00000001u32,
TEST_CLK_SEL_2                           = 0x00000002u32,
TEST_CLK_SEL_3                           = 0x00000003u32,
TEST_CLK_SEL_4                           = 0x00000004u32,
TEST_CLK_SEL_5                           = 0x00000005u32,
TEST_CLK_SEL_6                           = 0x00000006u32,
TEST_CLK_SEL_7                           = 0x00000007u32,
}

/*******************************************************
 * DC_PERFMON Enums
 *******************************************************/

/*
 * PERFCOUNTER_ACTIVE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_ACTIVE {
PERFCOUNTER_IS_IDLE                      = 0x00000000u32,
PERFCOUNTER_IS_ACTIVE                    = 0x00000001u32,
}

/*
 * PERFCOUNTER_CNT0_STATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_CNT0_STATE {
PERFCOUNTER_CNT0_STATE_RESET             = 0x00000000u32,
PERFCOUNTER_CNT0_STATE_START             = 0x00000001u32,
PERFCOUNTER_CNT0_STATE_FREEZE            = 0x00000002u32,
PERFCOUNTER_CNT0_STATE_HW                = 0x00000003u32,
}

/*
 * PERFCOUNTER_CNT1_STATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_CNT1_STATE {
PERFCOUNTER_CNT1_STATE_RESET             = 0x00000000u32,
PERFCOUNTER_CNT1_STATE_START             = 0x00000001u32,
PERFCOUNTER_CNT1_STATE_FREEZE            = 0x00000002u32,
PERFCOUNTER_CNT1_STATE_HW                = 0x00000003u32,
}

/*
 * PERFCOUNTER_CNT2_STATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_CNT2_STATE {
PERFCOUNTER_CNT2_STATE_RESET             = 0x00000000u32,
PERFCOUNTER_CNT2_STATE_START             = 0x00000001u32,
PERFCOUNTER_CNT2_STATE_FREEZE            = 0x00000002u32,
PERFCOUNTER_CNT2_STATE_HW                = 0x00000003u32,
}

/*
 * PERFCOUNTER_CNT3_STATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_CNT3_STATE {
PERFCOUNTER_CNT3_STATE_RESET             = 0x00000000u32,
PERFCOUNTER_CNT3_STATE_START             = 0x00000001u32,
PERFCOUNTER_CNT3_STATE_FREEZE            = 0x00000002u32,
PERFCOUNTER_CNT3_STATE_HW                = 0x00000003u32,
}

/*
 * PERFCOUNTER_CNT4_STATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_CNT4_STATE {
PERFCOUNTER_CNT4_STATE_RESET             = 0x00000000u32,
PERFCOUNTER_CNT4_STATE_START             = 0x00000001u32,
PERFCOUNTER_CNT4_STATE_FREEZE            = 0x00000002u32,
PERFCOUNTER_CNT4_STATE_HW                = 0x00000003u32,
}

/*
 * PERFCOUNTER_CNT5_STATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_CNT5_STATE {
PERFCOUNTER_CNT5_STATE_RESET             = 0x00000000u32,
PERFCOUNTER_CNT5_STATE_START             = 0x00000001u32,
PERFCOUNTER_CNT5_STATE_FREEZE            = 0x00000002u32,
PERFCOUNTER_CNT5_STATE_HW                = 0x00000003u32,
}

/*
 * PERFCOUNTER_CNT6_STATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_CNT6_STATE {
PERFCOUNTER_CNT6_STATE_RESET             = 0x00000000u32,
PERFCOUNTER_CNT6_STATE_START             = 0x00000001u32,
PERFCOUNTER_CNT6_STATE_FREEZE            = 0x00000002u32,
PERFCOUNTER_CNT6_STATE_HW                = 0x00000003u32,
}

/*
 * PERFCOUNTER_CNT7_STATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_CNT7_STATE {
PERFCOUNTER_CNT7_STATE_RESET             = 0x00000000u32,
PERFCOUNTER_CNT7_STATE_START             = 0x00000001u32,
PERFCOUNTER_CNT7_STATE_FREEZE            = 0x00000002u32,
PERFCOUNTER_CNT7_STATE_HW                = 0x00000003u32,
}

/*
 * PERFCOUNTER_CNTL_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_CNTL_SEL {
PERFCOUNTER_CNTL_SEL_0                   = 0x00000000u32,
PERFCOUNTER_CNTL_SEL_1                   = 0x00000001u32,
PERFCOUNTER_CNTL_SEL_2                   = 0x00000002u32,
PERFCOUNTER_CNTL_SEL_3                   = 0x00000003u32,
PERFCOUNTER_CNTL_SEL_4                   = 0x00000004u32,
PERFCOUNTER_CNTL_SEL_5                   = 0x00000005u32,
PERFCOUNTER_CNTL_SEL_6                   = 0x00000006u32,
PERFCOUNTER_CNTL_SEL_7                   = 0x00000007u32,
}

/*
 * PERFCOUNTER_CNTOFF_START_DIS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_CNTOFF_START_DIS {
PERFCOUNTER_CNTOFF_START_ENABLE          = 0x00000000u32,
PERFCOUNTER_CNTOFF_START_DISABLE         = 0x00000001u32,
}

/*
 * PERFCOUNTER_COUNTED_VALUE_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_COUNTED_VALUE_TYPE {
PERFCOUNTER_COUNTED_VALUE_TYPE_ACC       = 0x00000000u32,
PERFCOUNTER_COUNTED_VALUE_TYPE_MAX       = 0x00000001u32,
PERFCOUNTER_COUNTED_VALUE_TYPE_MIN       = 0x00000002u32,
}

/*
 * PERFCOUNTER_CVALUE_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_CVALUE_SEL {
PERFCOUNTER_CVALUE_SEL_47_0              = 0x00000000u32,
PERFCOUNTER_CVALUE_SEL_15_0              = 0x00000001u32,
PERFCOUNTER_CVALUE_SEL_31_16             = 0x00000002u32,
PERFCOUNTER_CVALUE_SEL_47_32             = 0x00000003u32,
PERFCOUNTER_CVALUE_SEL_11_0              = 0x00000004u32,
PERFCOUNTER_CVALUE_SEL_23_12             = 0x00000005u32,
PERFCOUNTER_CVALUE_SEL_35_24             = 0x00000006u32,
PERFCOUNTER_CVALUE_SEL_47_36             = 0x00000007u32,
}

/*
 * PERFCOUNTER_HW_CNTL_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_HW_CNTL_SEL {
PERFCOUNTER_HW_CNTL_SEL_RUNEN            = 0x00000000u32,
PERFCOUNTER_HW_CNTL_SEL_CNTOFF           = 0x00000001u32,
}

/*
 * PERFCOUNTER_HW_STOP1_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_HW_STOP1_SEL {
PERFCOUNTER_HW_STOP1_0                   = 0x00000000u32,
PERFCOUNTER_HW_STOP1_1                   = 0x00000001u32,
}

/*
 * PERFCOUNTER_HW_STOP2_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_HW_STOP2_SEL {
PERFCOUNTER_HW_STOP2_0                   = 0x00000000u32,
PERFCOUNTER_HW_STOP2_1                   = 0x00000001u32,
}

/*
 * PERFCOUNTER_INC_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_INC_MODE {
PERFCOUNTER_INC_MODE_MULTI_BIT           = 0x00000000u32,
PERFCOUNTER_INC_MODE_BOTH_EDGE           = 0x00000001u32,
PERFCOUNTER_INC_MODE_LSB                 = 0x00000002u32,
PERFCOUNTER_INC_MODE_POS_EDGE            = 0x00000003u32,
PERFCOUNTER_INC_MODE_NEG_EDGE            = 0x00000004u32,
}

/*
 * PERFCOUNTER_INT_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_INT_EN {
PERFCOUNTER_INT_DISABLE                  = 0x00000000u32,
PERFCOUNTER_INT_ENABLE                   = 0x00000001u32,
}

/*
 * PERFCOUNTER_INT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_INT_TYPE {
PERFCOUNTER_INT_TYPE_LEVEL               = 0x00000000u32,
PERFCOUNTER_INT_TYPE_PULSE               = 0x00000001u32,
}

/*
 * PERFCOUNTER_OFF_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_OFF_MASK {
PERFCOUNTER_OFF_MASK_DISABLE             = 0x00000000u32,
PERFCOUNTER_OFF_MASK_ENABLE              = 0x00000001u32,
}

/*
 * PERFCOUNTER_RESTART_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_RESTART_EN {
PERFCOUNTER_RESTART_DISABLE              = 0x00000000u32,
PERFCOUNTER_RESTART_ENABLE               = 0x00000001u32,
}

/*
 * PERFCOUNTER_RUNEN_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_RUNEN_MODE {
PERFCOUNTER_RUNEN_MODE_LEVEL             = 0x00000000u32,
PERFCOUNTER_RUNEN_MODE_EDGE              = 0x00000001u32,
}

/*
 * PERFCOUNTER_STATE_SEL0 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_STATE_SEL0 {
PERFCOUNTER_STATE_SEL0_GLOBAL            = 0x00000000u32,
PERFCOUNTER_STATE_SEL0_LOCAL             = 0x00000001u32,
}

/*
 * PERFCOUNTER_STATE_SEL1 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_STATE_SEL1 {
PERFCOUNTER_STATE_SEL1_GLOBAL            = 0x00000000u32,
PERFCOUNTER_STATE_SEL1_LOCAL             = 0x00000001u32,
}

/*
 * PERFCOUNTER_STATE_SEL2 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_STATE_SEL2 {
PERFCOUNTER_STATE_SEL2_GLOBAL            = 0x00000000u32,
PERFCOUNTER_STATE_SEL2_LOCAL             = 0x00000001u32,
}

/*
 * PERFCOUNTER_STATE_SEL3 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_STATE_SEL3 {
PERFCOUNTER_STATE_SEL3_GLOBAL            = 0x00000000u32,
PERFCOUNTER_STATE_SEL3_LOCAL             = 0x00000001u32,
}

/*
 * PERFCOUNTER_STATE_SEL4 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_STATE_SEL4 {
PERFCOUNTER_STATE_SEL4_GLOBAL            = 0x00000000u32,
PERFCOUNTER_STATE_SEL4_LOCAL             = 0x00000001u32,
}

/*
 * PERFCOUNTER_STATE_SEL5 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_STATE_SEL5 {
PERFCOUNTER_STATE_SEL5_GLOBAL            = 0x00000000u32,
PERFCOUNTER_STATE_SEL5_LOCAL             = 0x00000001u32,
}

/*
 * PERFCOUNTER_STATE_SEL6 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_STATE_SEL6 {
PERFCOUNTER_STATE_SEL6_GLOBAL            = 0x00000000u32,
PERFCOUNTER_STATE_SEL6_LOCAL             = 0x00000001u32,
}

/*
 * PERFCOUNTER_STATE_SEL7 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFCOUNTER_STATE_SEL7 {
PERFCOUNTER_STATE_SEL7_GLOBAL            = 0x00000000u32,
PERFCOUNTER_STATE_SEL7_LOCAL             = 0x00000001u32,
}

/*
 * PERFMON_CNTOFF_AND_OR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFMON_CNTOFF_AND_OR {
PERFMON_CNTOFF_OR                        = 0x00000000u32,
PERFMON_CNTOFF_AND                       = 0x00000001u32,
}

/*
 * PERFMON_CNTOFF_INT_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFMON_CNTOFF_INT_EN {
PERFMON_CNTOFF_INT_DISABLE               = 0x00000000u32,
PERFMON_CNTOFF_INT_ENABLE                = 0x00000001u32,
}

/*
 * PERFMON_CNTOFF_INT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFMON_CNTOFF_INT_TYPE {
PERFMON_CNTOFF_INT_TYPE_LEVEL            = 0x00000000u32,
PERFMON_CNTOFF_INT_TYPE_PULSE            = 0x00000001u32,
}

/*
 * PERFMON_STATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PERFMON_STATE {
PERFMON_STATE_RESET                      = 0x00000000u32,
PERFMON_STATE_START                      = 0x00000001u32,
PERFMON_STATE_FREEZE                     = 0x00000002u32,
PERFMON_STATE_HW                         = 0x00000003u32,
}

/*******************************************************
 * HUBP Enums
 *******************************************************/

/*
 * BIGK_FRAGMENT_SIZE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum BIGK_FRAGMENT_SIZE {
VM_PG_SIZE_4KB                           = 0x00000000u32,
VM_PG_SIZE_8KB                           = 0x00000001u32,
VM_PG_SIZE_16KB                          = 0x00000002u32,
VM_PG_SIZE_32KB                          = 0x00000003u32,
VM_PG_SIZE_64KB                          = 0x00000004u32,
VM_PG_SIZE_128KB                         = 0x00000005u32,
VM_PG_SIZE_256KB                         = 0x00000006u32,
VM_PG_SIZE_512KB                         = 0x00000007u32,
VM_PG_SIZE_1MB                           = 0x00000008u32,
VM_PG_SIZE_2MB                           = 0x00000009u32,
VM_PG_SIZE_4MB                           = 0x0000000au32,
VM_PG_SIZE_8MB                           = 0x0000000bu32,
VM_PG_SIZE_16MB                          = 0x0000000cu32,
VM_PG_SIZE_32MB                          = 0x0000000du32,
VM_PG_SIZE_64MB                          = 0x0000000eu32,
VM_PG_SIZE_128MB                         = 0x0000000fu32,
}

/*
 * CHUNK_SIZE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CHUNK_SIZE {
CHUNK_SIZE_1KB                           = 0x00000000u32,
CHUNK_SIZE_2KB                           = 0x00000001u32,
CHUNK_SIZE_4KB                           = 0x00000002u32,
CHUNK_SIZE_8KB                           = 0x00000003u32,
CHUNK_SIZE_16KB                          = 0x00000004u32,
CHUNK_SIZE_32KB                          = 0x00000005u32,
CHUNK_SIZE_64KB                          = 0x00000006u32,
}

/*
 * DPTE_GROUP_SIZE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPTE_GROUP_SIZE {
DPTE_GROUP_SIZE_64B                      = 0x00000000u32,
DPTE_GROUP_SIZE_128B                     = 0x00000001u32,
DPTE_GROUP_SIZE_256B                     = 0x00000002u32,
DPTE_GROUP_SIZE_512B                     = 0x00000003u32,
DPTE_GROUP_SIZE_1024B                    = 0x00000004u32,
DPTE_GROUP_SIZE_2048B                    = 0x00000005u32,
}

/*
 * FORCE_ONE_ROW_FOR_FRAME enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FORCE_ONE_ROW_FOR_FRAME {
FORCE_ONE_ROW_FOR_FRAME_0                = 0x00000000u32,
FORCE_ONE_ROW_FOR_FRAME_1                = 0x00000001u32,
}

/*
 * HUBP_BLANK_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HUBP_BLANK_EN {
HUBP_BLANK_SW_DEASSERT                   = 0x00000000u32,
HUBP_BLANK_SW_ASSERT                     = 0x00000001u32,
}

/*
 * HUBP_IN_BLANK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HUBP_IN_BLANK {
HUBP_IN_ACTIVE                           = 0x00000000u32,
HUBP_IN_VBLANK                           = 0x00000001u32,
}

/*
 * HUBP_MEASURE_WIN_MODE_DCFCLK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HUBP_MEASURE_WIN_MODE_DCFCLK {
HUBP_MEASURE_WIN_MODE_DCFCLK_0           = 0x00000000u32,
HUBP_MEASURE_WIN_MODE_DCFCLK_1           = 0x00000001u32,
HUBP_MEASURE_WIN_MODE_DCFCLK_2           = 0x00000002u32,
HUBP_MEASURE_WIN_MODE_DCFCLK_3           = 0x00000003u32,
}

/*
 * HUBP_NO_OUTSTANDING_REQ enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HUBP_NO_OUTSTANDING_REQ {
OUTSTANDING_REQ                          = 0x00000000u32,
NO_OUTSTANDING_REQ                       = 0x00000001u32,
}

/*
 * HUBP_SOFT_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HUBP_SOFT_RESET {
HUBP_SOFT_RESET_ON                       = 0x00000000u32,
HUBP_SOFT_RESET_OFF                      = 0x00000001u32,
}

/*
 * HUBP_TTU_DISABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HUBP_TTU_DISABLE {
HUBP_TTU_ENABLED                         = 0x00000000u32,
HUBP_TTU_DISABLED                        = 0x00000001u32,
}

/*
 * HUBP_VREADY_AT_OR_AFTER_VSYNC enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HUBP_VREADY_AT_OR_AFTER_VSYNC {
VREADY_BEFORE_VSYNC                      = 0x00000000u32,
VREADY_AT_OR_AFTER_VSYNC                 = 0x00000001u32,
}

/*
 * HUBP_VTG_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HUBP_VTG_SEL {
VTG_SEL_0                                = 0x00000000u32,
VTG_SEL_1                                = 0x00000001u32,
VTG_SEL_2                                = 0x00000002u32,
VTG_SEL_3                                = 0x00000003u32,
VTG_SEL_4                                = 0x00000004u32,
VTG_SEL_5                                = 0x00000005u32,
}

/*
 * H_MIRROR_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum H_MIRROR_EN {
HW_MIRRORING_DISABLE                     = 0x00000000u32,
HW_MIRRORING_ENABLE                      = 0x00000001u32,
}

/*
 * LEGACY_PIPE_INTERLEAVE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum LEGACY_PIPE_INTERLEAVE {
LEGACY_PIPE_INTERLEAVE_256B              = 0x00000000u32,
LEGACY_PIPE_INTERLEAVE_512B              = 0x00000001u32,
}

/*
 * META_CHUNK_SIZE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum META_CHUNK_SIZE {
META_CHUNK_SIZE_1KB                      = 0x00000000u32,
META_CHUNK_SIZE_2KB                      = 0x00000001u32,
META_CHUNK_SIZE_4KB                      = 0x00000002u32,
META_CHUNK_SIZE_8KB                      = 0x00000003u32,
}

/*
 * META_LINEAR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum META_LINEAR {
META_SURF_TILED                          = 0x00000000u32,
META_SURF_LINEAR                         = 0x00000001u32,
}

/*
 * MIN_CHUNK_SIZE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MIN_CHUNK_SIZE {
NO_MIN_CHUNK_SIZE                        = 0x00000000u32,
MIN_CHUNK_SIZE_256B                      = 0x00000001u32,
MIN_CHUNK_SIZE_512B                      = 0x00000002u32,
MIN_CHUNK_SIZE_1024B                     = 0x00000003u32,
}

/*
 * MIN_META_CHUNK_SIZE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MIN_META_CHUNK_SIZE {
NO_MIN_META_CHUNK_SIZE                   = 0x00000000u32,
MIN_META_CHUNK_SIZE_64B                  = 0x00000001u32,
MIN_META_CHUNK_SIZE_128B                 = 0x00000002u32,
MIN_META_CHUNK_SIZE_256B                 = 0x00000003u32,
}

/*
 * PIPE_ALIGNED enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PIPE_ALIGNED {
PIPE_UNALIGNED_SURF                      = 0x00000000u32,
PIPE_ALIGNED_SURF                        = 0x00000001u32,
}

/*
 * PTE_BUFFER_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PTE_BUFFER_MODE {
PTE_BUFFER_MODE_0                        = 0x00000000u32,
PTE_BUFFER_MODE_1                        = 0x00000001u32,
}

/*
 * PTE_ROW_HEIGHT_LINEAR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PTE_ROW_HEIGHT_LINEAR {
PTE_ROW_HEIGHT_LINEAR_8L                 = 0x00000000u32,
PTE_ROW_HEIGHT_LINEAR_16L                = 0x00000001u32,
PTE_ROW_HEIGHT_LINEAR_32L                = 0x00000002u32,
PTE_ROW_HEIGHT_LINEAR_64L                = 0x00000003u32,
PTE_ROW_HEIGHT_LINEAR_128L               = 0x00000004u32,
PTE_ROW_HEIGHT_LINEAR_256L               = 0x00000005u32,
PTE_ROW_HEIGHT_LINEAR_512L               = 0x00000006u32,
PTE_ROW_HEIGHT_LINEAR_1024L              = 0x00000007u32,
}

/*
 * ROTATION_ANGLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ROTATION_ANGLE {
ROTATE_0_DEGREES                         = 0x00000000u32,
ROTATE_90_DEGREES                        = 0x00000001u32,
ROTATE_180_DEGREES                       = 0x00000002u32,
ROTATE_270_DEGREES                       = 0x00000003u32,
}

/*
 * SWATH_HEIGHT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SWATH_HEIGHT {
SWATH_HEIGHT_1L                          = 0x00000000u32,
SWATH_HEIGHT_2L                          = 0x00000001u32,
SWATH_HEIGHT_4L                          = 0x00000002u32,
SWATH_HEIGHT_8L                          = 0x00000003u32,
SWATH_HEIGHT_16L                         = 0x00000004u32,
}

/*
 * VMPG_SIZE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VMPG_SIZE {
VMPG_SIZE_4KB                            = 0x00000000u32,
VMPG_SIZE_64KB                           = 0x00000001u32,
}

/*
 * VM_GROUP_SIZE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VM_GROUP_SIZE {
VM_GROUP_SIZE_64B                        = 0x00000000u32,
VM_GROUP_SIZE_128B                       = 0x00000001u32,
VM_GROUP_SIZE_256B                       = 0x00000002u32,
VM_GROUP_SIZE_512B                       = 0x00000003u32,
VM_GROUP_SIZE_1024B                      = 0x00000004u32,
VM_GROUP_SIZE_2048B                      = 0x00000005u32,
}

/*******************************************************
 * HUBPREQ Enums
 *******************************************************/

/*
 * DFQ_MIN_FREE_ENTRIES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DFQ_MIN_FREE_ENTRIES {
DFQ_MIN_FREE_ENTRIES_0                   = 0x00000000u32,
DFQ_MIN_FREE_ENTRIES_1                   = 0x00000001u32,
DFQ_MIN_FREE_ENTRIES_2                   = 0x00000002u32,
DFQ_MIN_FREE_ENTRIES_3                   = 0x00000003u32,
DFQ_MIN_FREE_ENTRIES_4                   = 0x00000004u32,
DFQ_MIN_FREE_ENTRIES_5                   = 0x00000005u32,
DFQ_MIN_FREE_ENTRIES_6                   = 0x00000006u32,
DFQ_MIN_FREE_ENTRIES_7                   = 0x00000007u32,
}

/*
 * DFQ_NUM_ENTRIES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DFQ_NUM_ENTRIES {
DFQ_NUM_ENTRIES_0                        = 0x00000000u32,
DFQ_NUM_ENTRIES_1                        = 0x00000001u32,
DFQ_NUM_ENTRIES_2                        = 0x00000002u32,
DFQ_NUM_ENTRIES_3                        = 0x00000003u32,
DFQ_NUM_ENTRIES_4                        = 0x00000004u32,
DFQ_NUM_ENTRIES_5                        = 0x00000005u32,
DFQ_NUM_ENTRIES_6                        = 0x00000006u32,
DFQ_NUM_ENTRIES_7                        = 0x00000007u32,
DFQ_NUM_ENTRIES_8                        = 0x00000008u32,
}

/*
 * DFQ_SIZE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DFQ_SIZE {
DFQ_SIZE_0                               = 0x00000000u32,
DFQ_SIZE_1                               = 0x00000001u32,
DFQ_SIZE_2                               = 0x00000002u32,
DFQ_SIZE_3                               = 0x00000003u32,
DFQ_SIZE_4                               = 0x00000004u32,
DFQ_SIZE_5                               = 0x00000005u32,
DFQ_SIZE_6                               = 0x00000006u32,
DFQ_SIZE_7                               = 0x00000007u32,
}

/*
 * DMDATA_VM_DONE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DMDATA_VM_DONE {
DMDATA_VM_IS_NOT_DONE                    = 0x00000000u32,
DMDATA_VM_IS_DONE                        = 0x00000001u32,
}

/*
 * EXPANSION_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum EXPANSION_MODE {
EXPANSION_MODE_ZERO                      = 0x00000000u32,
EXPANSION_MODE_CONSERVATIVE              = 0x00000001u32,
EXPANSION_MODE_OPTIMAL                   = 0x00000002u32,
}

/*
 * FLIP_RATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FLIP_RATE {
FLIP_RATE_0                              = 0x00000000u32,
FLIP_RATE_1                              = 0x00000001u32,
FLIP_RATE_2                              = 0x00000002u32,
FLIP_RATE_3                              = 0x00000003u32,
FLIP_RATE_4                              = 0x00000004u32,
FLIP_RATE_5                              = 0x00000005u32,
FLIP_RATE_6                              = 0x00000006u32,
FLIP_RATE_7                              = 0x00000007u32,
}

/*
 * INT_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum INT_MASK {
INT_DISABLED                             = 0x00000000u32,
INT_ENABLED                              = 0x00000001u32,
}

/*
 * PIPE_IN_FLUSH_URGENT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PIPE_IN_FLUSH_URGENT {
PIPE_IN_FLUSH_URGENT_ENABLE              = 0x00000000u32,
PIPE_IN_FLUSH_URGENT_DISABLE             = 0x00000001u32,
}

/*
 * PRQ_MRQ_FLUSH_URGENT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PRQ_MRQ_FLUSH_URGENT {
PRQ_MRQ_FLUSH_URGENT_ENABLE              = 0x00000000u32,
PRQ_MRQ_FLUSH_URGENT_DISABLE             = 0x00000001u32,
}

/*
 * ROW_TTU_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ROW_TTU_MODE {
END_OF_ROW_MODE                          = 0x00000000u32,
WATERMARK_MODE                           = 0x00000001u32,
}

/*
 * SURFACE_DCC enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SURFACE_DCC {
SURFACE_IS_NOT_DCC                       = 0x00000000u32,
SURFACE_IS_DCC                           = 0x00000001u32,
}

/*
 * SURFACE_DCC_IND_128B enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SURFACE_DCC_IND_128B {
SURFACE_DCC_IS_NOT_IND_128B              = 0x00000000u32,
SURFACE_DCC_IS_IND_128B                  = 0x00000001u32,
}

/*
 * SURFACE_DCC_IND_64B enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SURFACE_DCC_IND_64B {
SURFACE_DCC_IS_NOT_IND_64B               = 0x00000000u32,
SURFACE_DCC_IS_IND_64B                   = 0x00000001u32,
}

/*
 * SURFACE_DCC_IND_BLK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SURFACE_DCC_IND_BLK {
SURFACE_DCC_BLOCK_IS_UNCONSTRAINED       = 0x00000000u32,
SURFACE_DCC_BLOCK_IS_IND_64B             = 0x00000001u32,
SURFACE_DCC_BLOCK_IS_IND_128B            = 0x00000002u32,
SURFACE_DCC_BLOCK_IS_IND_64B_NO_128BCL   = 0x00000003u32,
}

/*
 * SURFACE_FLIP_AWAY_INT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SURFACE_FLIP_AWAY_INT_TYPE {
SURFACE_FLIP_AWAY_INT_LEVEL              = 0x00000000u32,
SURFACE_FLIP_AWAY_INT_PULSE              = 0x00000001u32,
}

/*
 * SURFACE_FLIP_EXEC_DEBUG_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SURFACE_FLIP_EXEC_DEBUG_MODE {
SURFACE_FLIP_EXEC_NORMAL_MODE            = 0x00000000u32,
SURFACE_FLIP_EXEC_DEBUG_MODE_ENABLE      = 0x00000001u32,
}

/*
 * SURFACE_FLIP_INT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SURFACE_FLIP_INT_TYPE {
SURFACE_FLIP_INT_LEVEL                   = 0x00000000u32,
SURFACE_FLIP_INT_PULSE                   = 0x00000001u32,
}

/*
 * SURFACE_FLIP_IN_STEREOSYNC enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SURFACE_FLIP_IN_STEREOSYNC {
SURFACE_FLIP_NOT_IN_STEREOSYNC_MODE      = 0x00000000u32,
SURFACE_FLIP_IN_STEREOSYNC_MODE          = 0x00000001u32,
}

/*
 * SURFACE_FLIP_MODE_FOR_STEREOSYNC enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SURFACE_FLIP_MODE_FOR_STEREOSYNC {
FLIP_ANY_FRAME                           = 0x00000000u32,
FLIP_LEFT_EYE                            = 0x00000001u32,
FLIP_RIGHT_EYE                           = 0x00000002u32,
SURFACE_FLIP_MODE_FOR_STEREOSYNC_RESERVED = 0x00000003u32,
}

/*
 * SURFACE_FLIP_STEREO_SELECT_DISABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SURFACE_FLIP_STEREO_SELECT_DISABLE {
SURFACE_FLIP_STEREO_SELECT_ENABLED       = 0x00000000u32,
SURFACE_FLIP_STEREO_SELECT_DISABLED      = 0x00000001u32,
}

/*
 * SURFACE_FLIP_STEREO_SELECT_POLARITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SURFACE_FLIP_STEREO_SELECT_POLARITY {
SURFACE_FLIP_STEREO_SELECT_POLARITY_NOT_INVERT = 0x00000000u32,
SURFACE_FLIP_STEREO_SELECT_POLARITY_INVERT = 0x00000001u32,
}

/*
 * SURFACE_FLIP_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SURFACE_FLIP_TYPE {
SURFACE_V_FLIP                           = 0x00000000u32,
SURFACE_I_FLIP                           = 0x00000001u32,
}

/*
 * SURFACE_FLIP_VUPDATE_SKIP_NUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SURFACE_FLIP_VUPDATE_SKIP_NUM {
SURFACE_FLIP_VUPDATE_SKIP_NUM_0          = 0x00000000u32,
SURFACE_FLIP_VUPDATE_SKIP_NUM_1          = 0x00000001u32,
SURFACE_FLIP_VUPDATE_SKIP_NUM_2          = 0x00000002u32,
SURFACE_FLIP_VUPDATE_SKIP_NUM_3          = 0x00000003u32,
SURFACE_FLIP_VUPDATE_SKIP_NUM_4          = 0x00000004u32,
SURFACE_FLIP_VUPDATE_SKIP_NUM_5          = 0x00000005u32,
SURFACE_FLIP_VUPDATE_SKIP_NUM_6          = 0x00000006u32,
SURFACE_FLIP_VUPDATE_SKIP_NUM_7          = 0x00000007u32,
SURFACE_FLIP_VUPDATE_SKIP_NUM_8          = 0x00000008u32,
SURFACE_FLIP_VUPDATE_SKIP_NUM_9          = 0x00000009u32,
SURFACE_FLIP_VUPDATE_SKIP_NUM_10         = 0x0000000au32,
SURFACE_FLIP_VUPDATE_SKIP_NUM_11         = 0x0000000bu32,
SURFACE_FLIP_VUPDATE_SKIP_NUM_12         = 0x0000000cu32,
SURFACE_FLIP_VUPDATE_SKIP_NUM_13         = 0x0000000du32,
SURFACE_FLIP_VUPDATE_SKIP_NUM_14         = 0x0000000eu32,
SURFACE_FLIP_VUPDATE_SKIP_NUM_15         = 0x0000000fu32,
}

/*
 * SURFACE_INUSE_RAED_NO_LATCH enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SURFACE_INUSE_RAED_NO_LATCH {
SURFACE_INUSE_IS_LATCHED                 = 0x00000000u32,
SURFACE_INUSE_IS_NOT_LATCHED             = 0x00000001u32,
}

/*
 * SURFACE_TMZ enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SURFACE_TMZ {
SURFACE_IS_NOT_TMZ                       = 0x00000000u32,
SURFACE_IS_TMZ                           = 0x00000001u32,
}

/*
 * SURFACE_UPDATE_LOCK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SURFACE_UPDATE_LOCK {
SURFACE_UPDATE_IS_UNLOCKED               = 0x00000000u32,
SURFACE_UPDATE_IS_LOCKED                 = 0x00000001u32,
}

/*******************************************************
 * HUBPRET Enums
 *******************************************************/

/*
 * CROSSBAR_FOR_ALPHA enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CROSSBAR_FOR_ALPHA {
ALPHA_DATA_ONTO_ALPHA_PORT               = 0x00000000u32,
Y_G_DATA_ONTO_ALPHA_PORT                 = 0x00000001u32,
CB_B_DATA_ONTO_ALPHA_PORT                = 0x00000002u32,
CR_R_DATA_ONTO_ALPHA_PORT                = 0x00000003u32,
}

/*
 * CROSSBAR_FOR_CB_B enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CROSSBAR_FOR_CB_B {
ALPHA_DATA_ONTO_CB_B_PORT                = 0x00000000u32,
Y_G_DATA_ONTO_CB_B_PORT                  = 0x00000001u32,
CB_B_DATA_ONTO_CB_B_PORT                 = 0x00000002u32,
CR_R_DATA_ONTO_CB_B_PORT                 = 0x00000003u32,
}

/*
 * CROSSBAR_FOR_CR_R enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CROSSBAR_FOR_CR_R {
ALPHA_DATA_ONTO_CR_R_PORT                = 0x00000000u32,
Y_G_DATA_ONTO_CR_R_PORT                  = 0x00000001u32,
CB_B_DATA_ONTO_CR_R_PORT                 = 0x00000002u32,
CR_R_DATA_ONTO_CR_R_PORT                 = 0x00000003u32,
}

/*
 * CROSSBAR_FOR_Y_G enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CROSSBAR_FOR_Y_G {
ALPHA_DATA_ONTO_Y_G_PORT                 = 0x00000000u32,
Y_G_DATA_ONTO_Y_G_PORT                   = 0x00000001u32,
CB_B_DATA_ONTO_Y_G_PORT                  = 0x00000002u32,
CR_R_DATA_ONTO_Y_G_PORT                  = 0x00000003u32,
}

/*
 * DETILE_BUFFER_PACKER_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DETILE_BUFFER_PACKER_ENABLE {
DETILE_BUFFER_PACKER_IS_DISABLE          = 0x00000000u32,
DETILE_BUFFER_PACKER_IS_ENABLE           = 0x00000001u32,
}

/*
 * MEM_PWR_DIS_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MEM_PWR_DIS_MODE {
MEM_POWER_DIS_MODE_ENABLE                = 0x00000000u32,
MEM_POWER_DIS_MODE_DISABLE               = 0x00000001u32,
}

/*
 * MEM_PWR_FORCE_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MEM_PWR_FORCE_MODE {
MEM_POWER_FORCE_MODE_OFF                 = 0x00000000u32,
MEM_POWER_FORCE_MODE_LIGHT_SLEEP         = 0x00000001u32,
MEM_POWER_FORCE_MODE_DEEP_SLEEP          = 0x00000002u32,
MEM_POWER_FORCE_MODE_SHUT_DOWN           = 0x00000003u32,
}

/*
 * MEM_PWR_STATUS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MEM_PWR_STATUS {
MEM_POWER_STATUS_ON                      = 0x00000000u32,
MEM_POWER_STATUS_LIGHT_SLEEP             = 0x00000001u32,
MEM_POWER_STATUS_DEEP_SLEEP              = 0x00000002u32,
MEM_POWER_STATUS_SHUT_DOWN               = 0x00000003u32,
}

/*
 * PIPE_INT_MASK_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PIPE_INT_MASK_MODE {
PIPE_INT_MASK_MODE_DISABLE               = 0x00000000u32,
PIPE_INT_MASK_MODE_ENABLE                = 0x00000001u32,
}

/*
 * PIPE_INT_TYPE_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PIPE_INT_TYPE_MODE {
PIPE_INT_TYPE_MODE_DISABLE               = 0x00000000u32,
PIPE_INT_TYPE_MODE_ENABLE                = 0x00000001u32,
}

/*
 * PIXCDC_MEM_PWR_LIGHT_SLEEP_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PIXCDC_MEM_PWR_LIGHT_SLEEP_MODE {
PIXCDC_MEM_POWER_LIGHT_SLEEP_MODE_OFF    = 0x00000000u32,
PIXCDC_MEM_POWER_LIGHT_SLEEP_MODE_1      = 0x00000001u32,
}

/*******************************************************
 * CURSOR Enums
 *******************************************************/

/*
 * CROB_MEM_PWR_LIGHT_SLEEP_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CROB_MEM_PWR_LIGHT_SLEEP_MODE {
CROB_MEM_POWER_LIGHT_SLEEP_MODE_OFF      = 0x00000000u32,
CROB_MEM_POWER_LIGHT_SLEEP_MODE_1        = 0x00000001u32,
CROB_MEM_POWER_LIGHT_SLEEP_MODE_2        = 0x00000002u32,
}

/*
 * CURSOR_2X_MAGNIFY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CURSOR_2X_MAGNIFY {
CURSOR_2X_MAGNIFY_IS_DISABLE             = 0x00000000u32,
CURSOR_2X_MAGNIFY_IS_ENABLE              = 0x00000001u32,
}

/*
 * CURSOR_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CURSOR_ENABLE {
CURSOR_IS_DISABLE                        = 0x00000000u32,
CURSOR_IS_ENABLE                         = 0x00000001u32,
}

/*
 * CURSOR_LINES_PER_CHUNK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CURSOR_LINES_PER_CHUNK {
CURSOR_LINE_PER_CHUNK_1                  = 0x00000000u32,
CURSOR_LINE_PER_CHUNK_2                  = 0x00000001u32,
CURSOR_LINE_PER_CHUNK_4                  = 0x00000002u32,
CURSOR_LINE_PER_CHUNK_8                  = 0x00000003u32,
CURSOR_LINE_PER_CHUNK_16                 = 0x00000004u32,
}

/*
 * CURSOR_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CURSOR_MODE {
CURSOR_MONO_2BIT                         = 0x00000000u32,
CURSOR_COLOR_24BIT_1BIT_AND              = 0x00000001u32,
CURSOR_COLOR_24BIT_8BIT_ALPHA_PREMULT    = 0x00000002u32,
CURSOR_COLOR_24BIT_8BIT_ALPHA_UNPREMULT  = 0x00000003u32,
CURSOR_COLOR_64BIT_FP_PREMULT            = 0x00000004u32,
CURSOR_COLOR_64BIT_FP_UNPREMULT          = 0x00000005u32,
}

/*
 * CURSOR_PERFMON_LATENCY_MEASURE_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CURSOR_PERFMON_LATENCY_MEASURE_EN {
CURSOR_PERFMON_LATENCY_MEASURE_IS_DISABLED = 0x00000000u32,
CURSOR_PERFMON_LATENCY_MEASURE_IS_ENABLED = 0x00000001u32,
}

/*
 * CURSOR_PERFMON_LATENCY_MEASURE_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CURSOR_PERFMON_LATENCY_MEASURE_SEL {
CURSOR_PERFMON_LATENCY_MEASURE_MC_LATENCY = 0x00000000u32,
CURSOR_PERFMON_LATENCY_MEASURE_CROB_LATENCY = 0x00000001u32,
}

/*
 * CURSOR_PITCH enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CURSOR_PITCH {
CURSOR_PITCH_64_PIXELS                   = 0x00000000u32,
CURSOR_PITCH_128_PIXELS                  = 0x00000001u32,
CURSOR_PITCH_256_PIXELS                  = 0x00000002u32,
}

/*
 * CURSOR_REQ_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CURSOR_REQ_MODE {
CURSOR_REQUEST_NORMALLY                  = 0x00000000u32,
CURSOR_REQUEST_EARLY                     = 0x00000001u32,
}

/*
 * CURSOR_SNOOP enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CURSOR_SNOOP {
CURSOR_IS_NOT_SNOOP                      = 0x00000000u32,
CURSOR_IS_SNOOP                          = 0x00000001u32,
}

/*
 * CURSOR_STEREO_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CURSOR_STEREO_EN {
CURSOR_STEREO_IS_DISABLED                = 0x00000000u32,
CURSOR_STEREO_IS_ENABLED                 = 0x00000001u32,
}

/*
 * CURSOR_SURFACE_TMZ enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CURSOR_SURFACE_TMZ {
CURSOR_SURFACE_IS_NOT_TMZ                = 0x00000000u32,
CURSOR_SURFACE_IS_TMZ                    = 0x00000001u32,
}

/*
 * CURSOR_SYSTEM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CURSOR_SYSTEM {
CURSOR_IN_SYSTEM_PHYSICAL_ADDRESS        = 0x00000000u32,
CURSOR_IN_GUEST_PHYSICAL_ADDRESS         = 0x00000001u32,
}

/*
 * CURSOR_XY_POSITION_ROTATION_AND_MIRRORING_BYPASS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CURSOR_XY_POSITION_ROTATION_AND_MIRRORING_BYPASS {
CURSOR_XY_POSITION_ROTATION_AND_MIRRORING_BYPASS_0 = 0x00000000u32,
CURSOR_XY_POSITION_ROTATION_AND_MIRRORING_BYPASS_1 = 0x00000001u32,
}

/*
 * DMDATA_DONE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DMDATA_DONE {
DMDATA_NOT_SENT_TO_DIG                   = 0x00000000u32,
DMDATA_SENT_TO_DIG                       = 0x00000001u32,
}

/*
 * DMDATA_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DMDATA_MODE {
DMDATA_SOFTWARE_UPDATE_MODE              = 0x00000000u32,
DMDATA_HARDWARE_UPDATE_MODE              = 0x00000001u32,
}

/*
 * DMDATA_QOS_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DMDATA_QOS_MODE {
DMDATA_QOS_LEVEL_FROM_TTU                = 0x00000000u32,
DMDATA_QOS_LEVEL_FROM_SOFTWARE           = 0x00000001u32,
}

/*
 * DMDATA_REPEAT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DMDATA_REPEAT {
DMDATA_USE_FOR_CURRENT_FRAME_ONLY        = 0x00000000u32,
DMDATA_USE_FOR_CURRENT_AND_FUTURE_FRAMES = 0x00000001u32,
}

/*
 * DMDATA_UNDERFLOW enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DMDATA_UNDERFLOW {
DMDATA_NOT_UNDERFLOW                     = 0x00000000u32,
DMDATA_UNDERFLOWED                       = 0x00000001u32,
}

/*
 * DMDATA_UNDERFLOW_CLEAR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DMDATA_UNDERFLOW_CLEAR {
DMDATA_DONT_CLEAR                        = 0x00000000u32,
DMDATA_CLEAR_UNDERFLOW_STATUS            = 0x00000001u32,
}

/*
 * DMDATA_UPDATED enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DMDATA_UPDATED {
DMDATA_NOT_UPDATED                       = 0x00000000u32,
DMDATA_WAS_UPDATED                       = 0x00000001u32,
}

/*
 * HUBP_3DLUT_ADDRESSING_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HUBP_3DLUT_ADDRESSING_MODE {
HUBP_3DLUT_SW_LINEAR                     = 0x00000000u32,
HUBP_3DLUT_SIMPLE_LINEAR                 = 0x00000001u32,
}

/*******************************************************
 * HUBBUB_SDPIF Enums
 *******************************************************/

/*
 * RESPONSE_STATUS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RESPONSE_STATUS {
OKAY                                     = 0x00000000u32,
EXOKAY                                   = 0x00000001u32,
SLVERR                                   = 0x00000002u32,
DECERR                                   = 0x00000003u32,
EARLY                                    = 0x00000004u32,
OKAY_NODATA                              = 0x00000005u32,
PROTVIOL                                 = 0x00000006u32,
TRANSERR                                 = 0x00000007u32,
CMPTO                                    = 0x00000008u32,
CRS                                      = 0x0000000cu32,
}

/*******************************************************
 * HUBBUB_RET_PATH Enums
 *******************************************************/

/*
 * DCHUBBUB_DET_MEM_PWR_LIGHT_SLEEP_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCHUBBUB_DET_MEM_PWR_LIGHT_SLEEP_MODE {
DCHUBBUB_DET_MEM_POWER_LIGHT_SLEEP_MODE_OFF = 0x00000000u32,
DCHUBBUB_DET_MEM_POWER_LIGHT_SLEEP_MODE_1 = 0x00000001u32,
DCHUBBUB_DET_MEM_POWER_LIGHT_SLEEP_MODE_2 = 0x00000002u32,
}

/*
 * DCHUBBUB_MEM_PWR_DIS_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCHUBBUB_MEM_PWR_DIS_MODE {
DCHUBBUB_MEM_POWER_DIS_MODE_ENABLE       = 0x00000000u32,
DCHUBBUB_MEM_POWER_DIS_MODE_DISABLE      = 0x00000001u32,
}

/*
 * DCHUBBUB_MEM_PWR_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCHUBBUB_MEM_PWR_MODE {
DCHUBBUB_MEM_POWER_MODE_OFF              = 0x00000000u32,
DCHUBBUB_MEM_POWER_MODE_LIGHT_SLEEP      = 0x00000001u32,
DCHUBBUB_MEM_POWER_MODE_DEEP_SLEEP       = 0x00000002u32,
DCHUBBUB_MEM_POWER_MODE_SHUT_DOWN        = 0x00000003u32,
}

/*******************************************************
 * MPC_CFG Enums
 *******************************************************/

/*
 * MPC_CFG_3DLUT_FL_FORMAT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_CFG_3DLUT_FL_FORMAT {
MPC_CFG_3DLUT_FL_FORMAT_0                = 0x00000000u32,
MPC_CFG_3DLUT_FL_FORMAT_1                = 0x00000001u32,
MPC_CFG_3DLUT_FL_FORMAT_2                = 0x00000002u32,
}

/*
 * MPC_CFG_3DLUT_FL_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_CFG_3DLUT_FL_MODE {
MPC_CFG_3DLUT_FL_MODE_0                  = 0x00000000u32,
MPC_CFG_3DLUT_FL_MODE_1                  = 0x00000001u32,
MPC_CFG_3DLUT_FL_MODE_2                  = 0x00000002u32,
MPC_CFG_3DLUT_FL_MODE_3                  = 0x00000003u32,
}

/*
 * MPC_CFG_ADR_CFG_CUR_VUPDATE_LOCK_SET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_CFG_ADR_CFG_CUR_VUPDATE_LOCK_SET {
MPC_CFG_ADR_CFG_CUR_VUPDATE_LOCK_SET_FALSE = 0x00000000u32,
MPC_CFG_ADR_CFG_CUR_VUPDATE_LOCK_SET_TRUE = 0x00000001u32,
}

/*
 * MPC_CFG_ADR_CFG_VUPDATE_LOCK_SET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_CFG_ADR_CFG_VUPDATE_LOCK_SET {
MPC_CFG_ADR_CFG_VUPDATE_LOCK_SET_FALSE   = 0x00000000u32,
MPC_CFG_ADR_CFG_VUPDATE_LOCK_SET_TRUE    = 0x00000001u32,
}

/*
 * MPC_CFG_ADR_VUPDATE_LOCK_SET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_CFG_ADR_VUPDATE_LOCK_SET {
MPC_CFG_ADR_VUPDATE_LOCK_SET_FALSE       = 0x00000000u32,
MPC_CFG_ADR_VUPDATE_LOCK_SET_TRUE        = 0x00000001u32,
}

/*
 * MPC_CFG_CFG_VUPDATE_LOCK_SET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_CFG_CFG_VUPDATE_LOCK_SET {
MPC_CFG_CFG_VUPDATE_LOCK_SET_FALSE       = 0x00000000u32,
MPC_CFG_CFG_VUPDATE_LOCK_SET_TRUE        = 0x00000001u32,
}

/*
 * MPC_CFG_CUR_VUPDATE_LOCK_SET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_CFG_CUR_VUPDATE_LOCK_SET {
MPC_CFG_CUR_VUPDATE_LOCK_SET_FALSE       = 0x00000000u32,
MPC_CFG_CUR_VUPDATE_LOCK_SET_TRUE        = 0x00000001u32,
}

/*
 * MPC_CFG_MPC_TEST_CLK_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_CFG_MPC_TEST_CLK_SEL {
MPC_CFG_MPC_TEST_CLK_SEL_0               = 0x00000000u32,
MPC_CFG_MPC_TEST_CLK_SEL_1               = 0x00000001u32,
MPC_CFG_MPC_TEST_CLK_SEL_2               = 0x00000002u32,
MPC_CFG_MPC_TEST_CLK_SEL_3               = 0x00000003u32,
}

/*
 * MPC_CFG_TEST_DEBUG_INDEX_MPC_CFG_TEST_DEBUG_WRITE_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_CFG_TEST_DEBUG_INDEX_MPC_CFG_TEST_DEBUG_WRITE_EN {
MPC_CFG_TEST_DEBUG_INDEX_MPC_CFG_TEST_DEBUG_WRITE_EN_FALSE = 0x00000000u32,
MPC_CFG_TEST_DEBUG_INDEX_MPC_CFG_TEST_DEBUG_WRITE_EN_TRUE = 0x00000001u32,
}

/*
 * MPC_CRC_CALC_INTERLACE_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_CRC_CALC_INTERLACE_MODE {
MPC_CRC_INTERLACE_MODE_TOP               = 0x00000000u32,
MPC_CRC_INTERLACE_MODE_BOTTOM            = 0x00000001u32,
MPC_CRC_INTERLACE_MODE_BOTH_RESET_BOTTOM = 0x00000002u32,
MPC_CRC_INTERLACE_MODE_BOTH_RESET_EACH   = 0x00000003u32,
}

/*
 * MPC_CRC_CALC_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_CRC_CALC_MODE {
MPC_CRC_ONE_SHOT_MODE                    = 0x00000000u32,
MPC_CRC_CONTINUOUS_MODE                  = 0x00000001u32,
}

/*
 * MPC_CRC_CALC_STEREO_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_CRC_CALC_STEREO_MODE {
MPC_CRC_STEREO_MODE_LEFT                 = 0x00000000u32,
MPC_CRC_STEREO_MODE_RIGHT                = 0x00000001u32,
MPC_CRC_STEREO_MODE_BOTH_RESET_RIGHT     = 0x00000002u32,
MPC_CRC_STEREO_MODE_BOTH_RESET_EACH      = 0x00000003u32,
}

/*
 * MPC_CRC_SOURCE_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_CRC_SOURCE_SELECT {
MPC_CRC_SOURCE_SEL_DPP                   = 0x00000000u32,
MPC_CRC_SOURCE_SEL_OPP                   = 0x00000001u32,
MPC_CRC_SOURCE_SEL_DWB                   = 0x00000002u32,
MPC_CRC_SOURCE_SEL_OTHER                 = 0x00000003u32,
}

/*******************************************************
 * MPC_OCSC Enums
 *******************************************************/

/*
 * MPC_OCSC_COEF_FORMAT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_OCSC_COEF_FORMAT {
MPC_OCSC_COEF_FORMAT_S2_13               = 0x00000000u32,
MPC_OCSC_COEF_FORMAT_S3_12               = 0x00000001u32,
}

/*
 * MPC_OCSC_TEST_DEBUG_INDEX_MPC_OCSC_TEST_DEBUG_WRITE_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_OCSC_TEST_DEBUG_INDEX_MPC_OCSC_TEST_DEBUG_WRITE_EN {
MPC_OCSC_TEST_DEBUG_INDEX_MPC_OCSC_TEST_DEBUG_WRITE_EN_FALSE = 0x00000000u32,
MPC_OCSC_TEST_DEBUG_INDEX_MPC_OCSC_TEST_DEBUG_WRITE_EN_TRUE = 0x00000001u32,
}

/*
 * MPC_OUT_CSC_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_OUT_CSC_MODE {
MPC_OUT_CSC_MODE_0                       = 0x00000000u32,
MPC_OUT_CSC_MODE_1                       = 0x00000001u32,
MPC_OUT_CSC_MODE_2                       = 0x00000002u32,
MPC_OUT_CSC_MODE_RSV                     = 0x00000003u32,
}

/*
 * MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_MODE {
MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_BYPASS = 0x00000000u32,
MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_6BITS = 0x00000001u32,
MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_8BITS = 0x00000002u32,
MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_9BITS = 0x00000003u32,
MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_10BITS = 0x00000004u32,
MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_11BITS = 0x00000005u32,
MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_12BITS = 0x00000006u32,
MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_PASSTHROUGH = 0x00000007u32,
}

/*
 * MPC_OUT_RATE_CONTROL_DISABLE_SET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPC_OUT_RATE_CONTROL_DISABLE_SET {
MPC_OUT_RATE_CONTROL_SET_ENABLE          = 0x00000000u32,
MPC_OUT_RATE_CONTROL_SET_DISABLE         = 0x00000001u32,
}

/*******************************************************
 * MPCC Enums
 *******************************************************/

/*
 * MPCC_BG_COLOR_BPC enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_BG_COLOR_BPC {
MPCC_BG_COLOR_BPC_8bit                   = 0x00000000u32,
MPCC_BG_COLOR_BPC_9bit                   = 0x00000001u32,
MPCC_BG_COLOR_BPC_10bit                  = 0x00000002u32,
MPCC_BG_COLOR_BPC_11bit                  = 0x00000003u32,
MPCC_BG_COLOR_BPC_12bit                  = 0x00000004u32,
}

/*
 * MPCC_CONTROL_MPCC_ACTIVE_OVERLAP_ONLY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_CONTROL_MPCC_ACTIVE_OVERLAP_ONLY {
MPCC_CONTROL_MPCC_ACTIVE_OVERLAP_ONLY_FALSE = 0x00000000u32,
MPCC_CONTROL_MPCC_ACTIVE_OVERLAP_ONLY_TRUE = 0x00000001u32,
}

/*
 * MPCC_CONTROL_MPCC_ALPHA_BLND_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_CONTROL_MPCC_ALPHA_BLND_MODE {
MPCC_CONTROL_MPCC_ALPHA_BLND_MODE_PER_PIXEL_ALPHA = 0x00000000u32,
MPCC_CONTROL_MPCC_ALPHA_BLND_MODE_PER_PIXEL_ALPHA_COMBINED_GLOBAL_GAIN = 0x00000001u32,
MPCC_CONTROL_MPCC_ALPHA_BLND_MODE_GLOBAL_ALPHA = 0x00000002u32,
MPCC_CONTROL_MPCC_ALPHA_BLND_MODE_UNUSED = 0x00000003u32,
}

/*
 * MPCC_CONTROL_MPCC_ALPHA_MULTIPLIED_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_CONTROL_MPCC_ALPHA_MULTIPLIED_MODE {
MPCC_CONTROL_MPCC_ALPHA_MULTIPLIED_MODE_FALSE = 0x00000000u32,
MPCC_CONTROL_MPCC_ALPHA_MULTIPLIED_MODE_TRUE = 0x00000001u32,
}

/*
 * MPCC_CONTROL_MPCC_BOT_GAIN_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_CONTROL_MPCC_BOT_GAIN_MODE {
MPCC_CONTROL_MPCC_BOT_GAIN_MODE_0        = 0x00000000u32,
MPCC_CONTROL_MPCC_BOT_GAIN_MODE_1        = 0x00000001u32,
}

/*
 * MPCC_CONTROL_MPCC_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_CONTROL_MPCC_MODE {
MPCC_CONTROL_MPCC_MODE_BYPASS            = 0x00000000u32,
MPCC_CONTROL_MPCC_MODE_TOP_LAYER_PASSTHROUGH = 0x00000001u32,
MPCC_CONTROL_MPCC_MODE_TOP_LAYER_ONLY    = 0x00000002u32,
MPCC_CONTROL_MPCC_MODE_TOP_BOT_BLENDING  = 0x00000003u32,
}

/*
 * MPCC_SM_CONTROL_MPCC_SM_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_SM_CONTROL_MPCC_SM_EN {
MPCC_SM_CONTROL_MPCC_SM_EN_FALSE         = 0x00000000u32,
MPCC_SM_CONTROL_MPCC_SM_EN_TRUE          = 0x00000001u32,
}

/*
 * MPCC_SM_CONTROL_MPCC_SM_FIELD_ALT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_SM_CONTROL_MPCC_SM_FIELD_ALT {
MPCC_SM_CONTROL_MPCC_SM_FIELD_ALT_FALSE  = 0x00000000u32,
MPCC_SM_CONTROL_MPCC_SM_FIELD_ALT_TRUE   = 0x00000001u32,
}

/*
 * MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_FRAME_POL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_FRAME_POL {
MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_FRAME_POL_NO_FORCE = 0x00000000u32,
MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_FRAME_POL_RESERVED = 0x00000001u32,
MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_FRAME_POL_FORCE_LOW = 0x00000002u32,
MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_FRAME_POL_FORCE_HIGH = 0x00000003u32,
}

/*
 * MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_TOP_POL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_TOP_POL {
MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_TOP_POL_NO_FORCE = 0x00000000u32,
MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_TOP_POL_RESERVED = 0x00000001u32,
MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_TOP_POL_FORCE_LOW = 0x00000002u32,
MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_TOP_POL_FORCE_HIGH = 0x00000003u32,
}

/*
 * MPCC_SM_CONTROL_MPCC_SM_FRAME_ALT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_SM_CONTROL_MPCC_SM_FRAME_ALT {
MPCC_SM_CONTROL_MPCC_SM_FRAME_ALT_FALSE  = 0x00000000u32,
MPCC_SM_CONTROL_MPCC_SM_FRAME_ALT_TRUE   = 0x00000001u32,
}

/*
 * MPCC_SM_CONTROL_MPCC_SM_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_SM_CONTROL_MPCC_SM_MODE {
MPCC_SM_CONTROL_MPCC_SM_MODE_SINGLE_PLANE = 0x00000000u32,
MPCC_SM_CONTROL_MPCC_SM_MODE_ROW_SUBSAMPLING = 0x00000002u32,
MPCC_SM_CONTROL_MPCC_SM_MODE_COLUMN_SUBSAMPLING = 0x00000004u32,
MPCC_SM_CONTROL_MPCC_SM_MODE_CHECKERBOARD_SUBSAMPLING = 0x00000006u32,
}

/*******************************************************
 * MPCC_OGAM Enums
 *******************************************************/

/*
 * MPCC_GAMUT_REMAP_COEF_FORMAT_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_GAMUT_REMAP_COEF_FORMAT_ENUM {
MPCC_GAMUT_REMAP_COEF_FORMAT_S2_13       = 0x00000000u32,
MPCC_GAMUT_REMAP_COEF_FORMAT_S3_12       = 0x00000001u32,
}

/*
 * MPCC_GAMUT_REMAP_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_GAMUT_REMAP_MODE_ENUM {
MPCC_GAMUT_REMAP_MODE_0                  = 0x00000000u32,
MPCC_GAMUT_REMAP_MODE_1                  = 0x00000001u32,
MPCC_GAMUT_REMAP_MODE_2                  = 0x00000002u32,
MPCC_GAMUT_REMAP_MODE_RSV                = 0x00000003u32,
}

/*
 * MPCC_OGAM_LUT_2_CONFIG_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_OGAM_LUT_2_CONFIG_ENUM {
MPCC_OGAM_LUT_2CFG_NO_MEMORY             = 0x00000000u32,
MPCC_OGAM_LUT_2CFG_MEMORY_A              = 0x00000001u32,
MPCC_OGAM_LUT_2CFG_MEMORY_B              = 0x00000002u32,
}

/*
 * MPCC_OGAM_LUT_CONFIG_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_OGAM_LUT_CONFIG_MODE {
MPCC_OGAM_DIFFERENT_RGB                  = 0x00000000u32,
MPCC_OGAM_ALL_USE_R                      = 0x00000001u32,
}

/*
 * MPCC_OGAM_LUT_PWL_DISABLE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_OGAM_LUT_PWL_DISABLE_ENUM {
MPCC_OGAM_ENABLE_PWL                     = 0x00000000u32,
MPCC_OGAM_DISABLE_PWL                    = 0x00000001u32,
}

/*
 * MPCC_OGAM_LUT_RAM_CONTROL_MPCC_OGAM_LUT_RAM_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_OGAM_LUT_RAM_CONTROL_MPCC_OGAM_LUT_RAM_SEL {
MPCC_OGAM_LUT_RAM_CONTROL_MPCC_OGAM_LUT_RAM_SEL_RAMA = 0x00000000u32,
MPCC_OGAM_LUT_RAM_CONTROL_MPCC_OGAM_LUT_RAM_SEL_RAMB = 0x00000001u32,
}

/*
 * MPCC_OGAM_LUT_RAM_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_OGAM_LUT_RAM_SEL {
MPCC_OGAM_RAMA_ACCESS                    = 0x00000000u32,
MPCC_OGAM_RAMB_ACCESS                    = 0x00000001u32,
}

/*
 * MPCC_OGAM_LUT_READ_COLOR_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_OGAM_LUT_READ_COLOR_SEL {
MPCC_OGAM_BLUE_LUT                       = 0x00000000u32,
MPCC_OGAM_GREEN_LUT                      = 0x00000001u32,
MPCC_OGAM_RED_LUT                        = 0x00000002u32,
}

/*
 * MPCC_OGAM_LUT_READ_DBG enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_OGAM_LUT_READ_DBG {
MPCC_OGAM_DISABLE_DEBUG                  = 0x00000000u32,
MPCC_OGAM_ENABLE_DEBUG                   = 0x00000001u32,
}

/*
 * MPCC_OGAM_LUT_SEL_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_OGAM_LUT_SEL_ENUM {
MPCC_OGAM_RAMA                           = 0x00000000u32,
MPCC_OGAM_RAMB                           = 0x00000001u32,
}

/*
 * MPCC_OGAM_MODE_MPCC_OGAM_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_OGAM_MODE_MPCC_OGAM_MODE_ENUM {
MPCC_OGAM_MODE_0                         = 0x00000000u32,
MPCC_OGAM_MODE_RSV1                      = 0x00000001u32,
MPCC_OGAM_MODE_2                         = 0x00000002u32,
MPCC_OGAM_MODE_RSV                       = 0x00000003u32,
}

/*
 * MPCC_OGAM_NUM_SEG enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_OGAM_NUM_SEG {
MPCC_OGAM_SEGMENTS_1                     = 0x00000000u32,
MPCC_OGAM_SEGMENTS_2                     = 0x00000001u32,
MPCC_OGAM_SEGMENTS_4                     = 0x00000002u32,
MPCC_OGAM_SEGMENTS_8                     = 0x00000003u32,
MPCC_OGAM_SEGMENTS_16                    = 0x00000004u32,
MPCC_OGAM_SEGMENTS_32                    = 0x00000005u32,
MPCC_OGAM_SEGMENTS_64                    = 0x00000006u32,
MPCC_OGAM_SEGMENTS_128                   = 0x00000007u32,
}

/*
 * MPCC_OGAM_TEST_DEBUG_INDEX_MPCC_OGAM_TEST_DEBUG_WRITE_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_OGAM_TEST_DEBUG_INDEX_MPCC_OGAM_TEST_DEBUG_WRITE_EN {
MPCC_OGAM_TEST_DEBUG_INDEX_MPCC_OGAM_TEST_DEBUG_WRITE_EN_FALSE = 0x00000000u32,
MPCC_OGAM_TEST_DEBUG_INDEX_MPCC_OGAM_TEST_DEBUG_WRITE_EN_TRUE = 0x00000001u32,
}

/*******************************************************
 * MPCC_MCM Enums
 *******************************************************/

/*
 * MPCC_MCM_3DLUT_30BIT_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_MCM_3DLUT_30BIT_ENUM {
MPCC_MCM_3DLUT_36BIT                     = 0x00000000u32,
MPCC_MCM_3DLUT_30BIT                     = 0x00000001u32,
}

/*
 * MPCC_MCM_3DLUT_RAM_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_MCM_3DLUT_RAM_SEL {
MPCC_MCM_RAM0_ACCESS                     = 0x00000000u32,
MPCC_MCM_RAM1_ACCESS                     = 0x00000001u32,
MPCC_MCM_RAM2_ACCESS                     = 0x00000002u32,
MPCC_MCM_RAM3_ACCESS                     = 0x00000003u32,
}

/*
 * MPCC_MCM_3DLUT_SIZE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_MCM_3DLUT_SIZE_ENUM {
MPCC_MCM_3DLUT_17CUBE                    = 0x00000000u32,
MPCC_MCM_3DLUT_9CUBE                     = 0x00000001u32,
}

/*
 * MPCC_MCM_GAMMA_LUT_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_MCM_GAMMA_LUT_MODE_ENUM {
MPCC_MCM_GAMMA_LUT_BYPASS                = 0x00000000u32,
MPCC_MCM_GAMMA_LUT_RESERVED_1            = 0x00000001u32,
MPCC_MCM_GAMMA_LUT_RAM_LUT               = 0x00000002u32,
MPCC_MCM_GAMMA_LUT_RESERVED_3            = 0x00000003u32,
}

/*
 * MPCC_MCM_GAMMA_LUT_PWL_DISABLE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_MCM_GAMMA_LUT_PWL_DISABLE_ENUM {
MPCC_MCM_GAMMA_LUT_ENABLE_PWL            = 0x00000000u32,
MPCC_MCM_GAMMA_LUT_DISABLE_PWL           = 0x00000001u32,
}

/*
 * MPCC_MCM_GAMMA_LUT_SEL_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_MCM_GAMMA_LUT_SEL_ENUM {
MPCC_MCM_GAMMA_LUT_RAMA                  = 0x00000000u32,
MPCC_MCM_GAMMA_LUT_RAMB                  = 0x00000001u32,
}

/*
 * MPCC_MCM_GAMUT_REMAP_COEF_FORMAT_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_MCM_GAMUT_REMAP_COEF_FORMAT_ENUM {
MPCC_MCM_GAMUT_REMAP_COEF_FORMAT_S2_13   = 0x00000000u32,
MPCC_MCM_GAMUT_REMAP_COEF_FORMAT_S3_12   = 0x00000001u32,
}

/*
 * MPCC_MCM_GAMUT_REMAP_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_MCM_GAMUT_REMAP_MODE_ENUM {
MPCC_MCM_GAMUT_REMAP_MODE_0              = 0x00000000u32,
MPCC_MCM_GAMUT_REMAP_MODE_1              = 0x00000001u32,
MPCC_MCM_GAMUT_REMAP_MODE_2              = 0x00000002u32,
MPCC_MCM_GAMUT_REMAP_MODE_RSV            = 0x00000003u32,
}

/*
 * MPCC_MCM_LUT_2_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_MCM_LUT_2_MODE_ENUM {
MPCC_MCM_LUT_2_MODE_BYPASS               = 0x00000000u32,
MPCC_MCM_LUT_2_MODE_RAMA_LUT             = 0x00000001u32,
MPCC_MCM_LUT_2_MODE_RAMB_LUT             = 0x00000002u32,
}

/*
 * MPCC_MCM_LUT_CONFIG_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_MCM_LUT_CONFIG_MODE {
MPCC_MCM_LUT_DIFFERENT_RGB               = 0x00000000u32,
MPCC_MCM_LUT_ALL_USE_R                   = 0x00000001u32,
}

/*
 * MPCC_MCM_LUT_NUM_SEG enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_MCM_LUT_NUM_SEG {
MPCC_MCM_LUT_SEGMENTS_1                  = 0x00000000u32,
MPCC_MCM_LUT_SEGMENTS_2                  = 0x00000001u32,
MPCC_MCM_LUT_SEGMENTS_4                  = 0x00000002u32,
MPCC_MCM_LUT_SEGMENTS_8                  = 0x00000003u32,
MPCC_MCM_LUT_SEGMENTS_16                 = 0x00000004u32,
MPCC_MCM_LUT_SEGMENTS_32                 = 0x00000005u32,
MPCC_MCM_LUT_SEGMENTS_64                 = 0x00000006u32,
MPCC_MCM_LUT_SEGMENTS_128                = 0x00000007u32,
}

/*
 * MPCC_MCM_LUT_RAM_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_MCM_LUT_RAM_SEL {
MPCC_MCM_LUT_RAMA_ACCESS                 = 0x00000000u32,
MPCC_MCM_LUT_RAMB_ACCESS                 = 0x00000001u32,
}

/*
 * MPCC_MCM_LUT_READ_COLOR_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_MCM_LUT_READ_COLOR_SEL {
MPCC_MCM_LUT_BLUE_LUT                    = 0x00000000u32,
MPCC_MCM_LUT_GREEN_LUT                   = 0x00000001u32,
MPCC_MCM_LUT_RED_LUT                     = 0x00000002u32,
}

/*
 * MPCC_MCM_LUT_READ_DBG enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_MCM_LUT_READ_DBG {
MPCC_MCM_LUT_DISABLE_DEBUG               = 0x00000000u32,
MPCC_MCM_LUT_ENABLE_DEBUG                = 0x00000001u32,
}

/*
 * MPCC_MCM_MEM_PWR_FORCE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_MCM_MEM_PWR_FORCE_ENUM {
MPCC_MCM_MEM_PWR_FORCE_DIS               = 0x00000000u32,
MPCC_MCM_MEM_PWR_FORCE_LS                = 0x00000001u32,
MPCC_MCM_MEM_PWR_FORCE_DS                = 0x00000002u32,
MPCC_MCM_MEM_PWR_FORCE_SD                = 0x00000003u32,
}

/*
 * MPCC_MCM_MEM_PWR_STATE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MPCC_MCM_MEM_PWR_STATE_ENUM {
MPCC_MCM_MEM_PWR_STATE_ON                = 0x00000000u32,
MPCC_MCM_MEM_PWR_STATE_LS                = 0x00000001u32,
MPCC_MCM_MEM_PWR_STATE_DS                = 0x00000002u32,
MPCC_MCM_MEM_PWR_STATE_SD                = 0x00000003u32,
}

/*******************************************************
 * DPG Enums
 *******************************************************/

/*
 * ENUM_DPG_BIT_DEPTH enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DPG_BIT_DEPTH {
ENUM_DPG_BIT_DEPTH_6BPC                  = 0x00000000u32,
ENUM_DPG_BIT_DEPTH_8BPC                  = 0x00000001u32,
ENUM_DPG_BIT_DEPTH_10BPC                 = 0x00000002u32,
ENUM_DPG_BIT_DEPTH_12BPC                 = 0x00000003u32,
}

/*
 * ENUM_DPG_DYNAMIC_RANGE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DPG_DYNAMIC_RANGE {
ENUM_DPG_DYNAMIC_RANGE_VESA              = 0x00000000u32,
ENUM_DPG_DYNAMIC_RANGE_CEA               = 0x00000001u32,
}

/*
 * ENUM_DPG_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DPG_EN {
ENUM_DPG_DISABLE                         = 0x00000000u32,
ENUM_DPG_ENABLE                          = 0x00000001u32,
}

/*
 * ENUM_DPG_FIELD_POLARITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DPG_FIELD_POLARITY {
ENUM_DPG_FIELD_POLARITY_TOP_EVEN_BOTTOM_ODD = 0x00000000u32,
ENUM_DPG_FIELD_POLARITY_TOP_ODD_BOTTOM_EVEN = 0x00000001u32,
}

/*
 * ENUM_DPG_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DPG_MODE {
ENUM_DPG_MODE_RGB_COLOUR_BLOCK           = 0x00000000u32,
ENUM_DPG_MODE_YCBCR_601_COLOUR_BLOCK     = 0x00000001u32,
ENUM_DPG_MODE_YCBCR_709_COLOUR_BLOCK     = 0x00000002u32,
ENUM_DPG_MODE_VERTICAL_BAR               = 0x00000003u32,
ENUM_DPG_MODE_HORIZONTAL_BAR             = 0x00000004u32,
ENUM_DPG_MODE_RGB_SINGLE_RAMP            = 0x00000005u32,
ENUM_DPG_MODE_RGB_DUAL_RAMP              = 0x00000006u32,
ENUM_DPG_MODE_RGB_XR_BIAS                = 0x00000007u32,
}

/*******************************************************
 * FMT Enums
 *******************************************************/

/*
 * FMTMEM_PWR_DIS_CTRL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMTMEM_PWR_DIS_CTRL {
FMTMEM_ENABLE_MEM_PWR_CTRL               = 0x00000000u32,
FMTMEM_DISABLE_MEM_PWR_CTRL              = 0x00000001u32,
}

/*
 * FMTMEM_PWR_FORCE_CTRL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMTMEM_PWR_FORCE_CTRL {
FMTMEM_NO_FORCE_REQUEST                  = 0x00000000u32,
FMTMEM_FORCE_LIGHT_SLEEP_REQUEST         = 0x00000001u32,
FMTMEM_FORCE_DEEP_SLEEP_REQUEST          = 0x00000002u32,
FMTMEM_FORCE_SHUT_DOWN_REQUEST           = 0x00000003u32,
}

/*
 * FMT_BIT_DEPTH_CONTROL_25FRC_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_BIT_DEPTH_CONTROL_25FRC_SEL {
FMT_BIT_DEPTH_CONTROL_25FRC_SEL_Ei       = 0x00000000u32,
FMT_BIT_DEPTH_CONTROL_25FRC_SEL_Fi       = 0x00000001u32,
FMT_BIT_DEPTH_CONTROL_25FRC_SEL_Gi       = 0x00000002u32,
FMT_BIT_DEPTH_CONTROL_25FRC_SEL_RESERVED = 0x00000003u32,
}

/*
 * FMT_BIT_DEPTH_CONTROL_50FRC_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_BIT_DEPTH_CONTROL_50FRC_SEL {
FMT_BIT_DEPTH_CONTROL_50FRC_SEL_A        = 0x00000000u32,
FMT_BIT_DEPTH_CONTROL_50FRC_SEL_B        = 0x00000001u32,
FMT_BIT_DEPTH_CONTROL_50FRC_SEL_C        = 0x00000002u32,
FMT_BIT_DEPTH_CONTROL_50FRC_SEL_D        = 0x00000003u32,
}

/*
 * FMT_BIT_DEPTH_CONTROL_75FRC_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_BIT_DEPTH_CONTROL_75FRC_SEL {
FMT_BIT_DEPTH_CONTROL_75FRC_SEL_E        = 0x00000000u32,
FMT_BIT_DEPTH_CONTROL_75FRC_SEL_F        = 0x00000001u32,
FMT_BIT_DEPTH_CONTROL_75FRC_SEL_G        = 0x00000002u32,
FMT_BIT_DEPTH_CONTROL_75FRC_SEL_RESERVED = 0x00000003u32,
}

/*
 * FMT_BIT_DEPTH_CONTROL_SPATIAL_DITHER_DEPTH enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_BIT_DEPTH_CONTROL_SPATIAL_DITHER_DEPTH {
FMT_BIT_DEPTH_CONTROL_SPATIAL_DITHER_DEPTH_18BPP = 0x00000000u32,
FMT_BIT_DEPTH_CONTROL_SPATIAL_DITHER_DEPTH_24BPP = 0x00000001u32,
FMT_BIT_DEPTH_CONTROL_SPATIAL_DITHER_DEPTH_30BPP = 0x00000002u32,
}

/*
 * FMT_BIT_DEPTH_CONTROL_TEMPORAL_DITHER_DEPTH enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_BIT_DEPTH_CONTROL_TEMPORAL_DITHER_DEPTH {
FMT_BIT_DEPTH_CONTROL_TEMPORAL_DITHER_DEPTH_18BPP = 0x00000000u32,
FMT_BIT_DEPTH_CONTROL_TEMPORAL_DITHER_DEPTH_24BPP = 0x00000001u32,
FMT_BIT_DEPTH_CONTROL_TEMPORAL_DITHER_DEPTH_30BPP = 0x00000002u32,
}

/*
 * FMT_BIT_DEPTH_CONTROL_TEMPORAL_LEVEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_BIT_DEPTH_CONTROL_TEMPORAL_LEVEL {
FMT_BIT_DEPTH_CONTROL_TEMPORAL_LEVEL_GREY_LEVEL2 = 0x00000000u32,
FMT_BIT_DEPTH_CONTROL_TEMPORAL_LEVEL_GREY_LEVEL4 = 0x00000001u32,
}

/*
 * FMT_BIT_DEPTH_CONTROL_TRUNCATE_DEPTH enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_BIT_DEPTH_CONTROL_TRUNCATE_DEPTH {
FMT_BIT_DEPTH_CONTROL_TRUNCATE_DEPTH_18BPP = 0x00000000u32,
FMT_BIT_DEPTH_CONTROL_TRUNCATE_DEPTH_24BPP = 0x00000001u32,
FMT_BIT_DEPTH_CONTROL_TRUNCATE_DEPTH_30BPP = 0x00000002u32,
}

/*
 * FMT_BIT_DEPTH_CONTROL_TRUNCATE_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_BIT_DEPTH_CONTROL_TRUNCATE_MODE {
FMT_BIT_DEPTH_CONTROL_TRUNCATE_MODE_TRUNCATION = 0x00000000u32,
FMT_BIT_DEPTH_CONTROL_TRUNCATE_MODE_ROUNDING = 0x00000001u32,
}

/*
 * FMT_CLAMP_CNTL_COLOR_FORMAT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_CLAMP_CNTL_COLOR_FORMAT {
FMT_CLAMP_CNTL_COLOR_FORMAT_6BPC         = 0x00000000u32,
FMT_CLAMP_CNTL_COLOR_FORMAT_8BPC         = 0x00000001u32,
FMT_CLAMP_CNTL_COLOR_FORMAT_10BPC        = 0x00000002u32,
FMT_CLAMP_CNTL_COLOR_FORMAT_12BPC        = 0x00000003u32,
FMT_CLAMP_CNTL_COLOR_FORMAT_RESERVED1    = 0x00000004u32,
FMT_CLAMP_CNTL_COLOR_FORMAT_RESERVED2    = 0x00000005u32,
FMT_CLAMP_CNTL_COLOR_FORMAT_RESERVED3    = 0x00000006u32,
FMT_CLAMP_CNTL_COLOR_FORMAT_PROGRAMMABLE = 0x00000007u32,
}

/*
 * FMT_CONTROL_CBCR_BIT_REDUCTION_BYPASS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_CONTROL_CBCR_BIT_REDUCTION_BYPASS {
FMT_CONTROL_CBCR_BIT_REDUCTION_BYPASS_DISABLE = 0x00000000u32,
FMT_CONTROL_CBCR_BIT_REDUCTION_BYPASS_ENABLE = 0x00000001u32,
}

/*
 * FMT_CONTROL_PIXEL_ENCODING enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_CONTROL_PIXEL_ENCODING {
FMT_CONTROL_PIXEL_ENCODING_RGB444_OR_YCBCR444 = 0x00000000u32,
FMT_CONTROL_PIXEL_ENCODING_YCBCR422      = 0x00000001u32,
FMT_CONTROL_PIXEL_ENCODING_YCBCR420      = 0x00000002u32,
FMT_CONTROL_PIXEL_ENCODING_RESERVED      = 0x00000003u32,
}

/*
 * FMT_CONTROL_SUBSAMPLING_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_CONTROL_SUBSAMPLING_MODE {
FMT_CONTROL_SUBSAMPLING_MODE_DROP        = 0x00000000u32,
FMT_CONTROL_SUBSAMPLING_MODE_AVERAGE     = 0x00000001u32,
FMT_CONTROL_SUBSAMPLING_MOME_3_TAP       = 0x00000002u32,
FMT_CONTROL_SUBSAMPLING_MOME_RESERVED    = 0x00000003u32,
}

/*
 * FMT_CONTROL_SUBSAMPLING_ORDER enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_CONTROL_SUBSAMPLING_ORDER {
FMT_CONTROL_SUBSAMPLING_ORDER_CB_BEFORE_CR = 0x00000000u32,
FMT_CONTROL_SUBSAMPLING_ORDER_CR_BEFORE_CB = 0x00000001u32,
}

/*
 * FMT_DEBUG_CNTL_COLOR_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_DEBUG_CNTL_COLOR_SELECT {
FMT_DEBUG_CNTL_COLOR_SELECT_BLUE         = 0x00000000u32,
FMT_DEBUG_CNTL_COLOR_SELECT_GREEN        = 0x00000001u32,
FMT_DEBUG_CNTL_COLOR_SELECT_RED1         = 0x00000002u32,
FMT_DEBUG_CNTL_COLOR_SELECT_RED2         = 0x00000003u32,
}

/*
 * FMT_DYNAMIC_EXP_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_DYNAMIC_EXP_MODE {
FMT_DYNAMIC_EXP_MODE_10to12              = 0x00000000u32,
FMT_DYNAMIC_EXP_MODE_8to12               = 0x00000001u32,
}

/*
 * FMT_FRAME_RANDOM_ENABLE_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_FRAME_RANDOM_ENABLE_CONTROL {
FMT_FRAME_RANDOM_ENABLE_RESET_EACH_FRAME = 0x00000000u32,
FMT_FRAME_RANDOM_ENABLE_RESET_ONCE       = 0x00000001u32,
}

/*
 * FMT_POWER_STATE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_POWER_STATE_ENUM {
FMT_POWER_STATE_ENUM_ON                  = 0x00000000u32,
FMT_POWER_STATE_ENUM_LS                  = 0x00000001u32,
FMT_POWER_STATE_ENUM_DS                  = 0x00000002u32,
FMT_POWER_STATE_ENUM_SD                  = 0x00000003u32,
}

/*
 * FMT_RGB_RANDOM_ENABLE_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_RGB_RANDOM_ENABLE_CONTROL {
FMT_RGB_RANDOM_ENABLE_CONTROL_DISABLE    = 0x00000000u32,
FMT_RGB_RANDOM_ENABLE_CONTROL_ENABLE     = 0x00000001u32,
}

/*
 * FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP_CONTROL {
FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP_NO_SWAP = 0x00000000u32,
FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP_1 = 0x00000001u32,
FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP_2 = 0x00000002u32,
FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP_RESERVED = 0x00000003u32,
}

/*
 * FMT_SPATIAL_DITHER_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_SPATIAL_DITHER_MODE {
FMT_SPATIAL_DITHER_MODE_0                = 0x00000000u32,
FMT_SPATIAL_DITHER_MODE_1                = 0x00000001u32,
FMT_SPATIAL_DITHER_MODE_2                = 0x00000002u32,
FMT_SPATIAL_DITHER_MODE_3                = 0x00000003u32,
}

/*
 * FMT_STEREOSYNC_OVERRIDE_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_STEREOSYNC_OVERRIDE_CONTROL {
FMT_STEREOSYNC_OVERRIDE_CONTROL_0        = 0x00000000u32,
FMT_STEREOSYNC_OVERRIDE_CONTROL_1        = 0x00000001u32,
}

/*
 * FMT_TEMPORAL_DITHER_PATTERN_CONTROL_RGB1_BGR0 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FMT_TEMPORAL_DITHER_PATTERN_CONTROL_RGB1_BGR0 {
FMT_TEMPORAL_DITHER_PATTERN_CONTROL_RGB1_BGR0_BGR = 0x00000000u32,
FMT_TEMPORAL_DITHER_PATTERN_CONTROL_RGB1_BGR0_RGB = 0x00000001u32,
}

/*******************************************************
 * OPPBUF Enums
 *******************************************************/

/*
 * OPPBUF_DISPLAY_SEGMENTATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OPPBUF_DISPLAY_SEGMENTATION {
OPPBUF_DISPLAY_SEGMENTATION_1_SEGMENT    = 0x00000000u32,
OPPBUF_DISPLAY_SEGMENTATION_2_SEGMENT    = 0x00000001u32,
OPPBUF_DISPLAY_SEGMENTATION_4_SEGMENT    = 0x00000002u32,
OPPBUF_DISPLAY_SEGMENTATION_4_SEGMENT_SPLIT_LEFT = 0x00000003u32,
OPPBUF_DISPLAY_SEGMENTATION_4_SEGMENT_SPLIT_RIGHT = 0x00000004u32,
}

/*******************************************************
 * OPP_PIPE Enums
 *******************************************************/

/*
 * OPP_PIPE_CLOCK_ENABLE_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OPP_PIPE_CLOCK_ENABLE_CONTROL {
OPP_PIPE_CLOCK_DISABLE                   = 0x00000000u32,
OPP_PIPE_CLOCK_ENABLE                    = 0x00000001u32,
}

/*
 * OPP_PIPE_DIGTIAL_BYPASS_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OPP_PIPE_DIGTIAL_BYPASS_CONTROL {
OPP_PIPE_DIGTIAL_BYPASS_DISABLE          = 0x00000000u32,
OPP_PIPE_DIGTIAL_BYPASS_ENABLE           = 0x00000001u32,
}

/*******************************************************
 * OPP_PIPE_CRC Enums
 *******************************************************/

/*
 * OPP_PIPE_CRC_CONT_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OPP_PIPE_CRC_CONT_EN {
OPP_PIPE_CRC_MODE_ONE_SHOT               = 0x00000000u32,
OPP_PIPE_CRC_MODE_CONTINUOUS             = 0x00000001u32,
}

/*
 * OPP_PIPE_CRC_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OPP_PIPE_CRC_EN {
OPP_PIPE_CRC_DISABLE                     = 0x00000000u32,
OPP_PIPE_CRC_ENABLE                      = 0x00000001u32,
}

/*
 * OPP_PIPE_CRC_INTERLACE_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OPP_PIPE_CRC_INTERLACE_EN {
OPP_PIPE_CRC_INTERLACE_EN_INTERPRET_AS_PROGRESSIVE = 0x00000000u32,
OPP_PIPE_CRC_INTERLACE_EN_INTERPRET_AS_INTERLACED = 0x00000001u32,
}

/*
 * OPP_PIPE_CRC_INTERLACE_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OPP_PIPE_CRC_INTERLACE_MODE {
OPP_PIPE_CRC_INTERLACE_MODE_TOP          = 0x00000000u32,
OPP_PIPE_CRC_INTERLACE_MODE_BOTTOM       = 0x00000001u32,
OPP_PIPE_CRC_INTERLACE_MODE_BOTH_RESET_AFTER_BOTTOM_FIELD = 0x00000002u32,
OPP_PIPE_CRC_INTERLACE_MODE_BOTH_RESET_AFTER_EACH_FIELD = 0x00000003u32,
}

/*
 * OPP_PIPE_CRC_ONE_SHOT_PENDING enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OPP_PIPE_CRC_ONE_SHOT_PENDING {
OPP_PIPE_CRC_ONE_SHOT_PENDING_NOT_PENDING = 0x00000000u32,
OPP_PIPE_CRC_ONE_SHOT_PENDING_PENDING    = 0x00000001u32,
}

/*
 * OPP_PIPE_CRC_PIXEL_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OPP_PIPE_CRC_PIXEL_SELECT {
OPP_PIPE_CRC_PIXEL_SELECT_ALL_PIXELS     = 0x00000000u32,
OPP_PIPE_CRC_PIXEL_SELECT_RESERVED       = 0x00000001u32,
OPP_PIPE_CRC_PIXEL_SELECT_EVEN_PIXELS    = 0x00000002u32,
OPP_PIPE_CRC_PIXEL_SELECT_ODD_PIXELS     = 0x00000003u32,
}

/*
 * OPP_PIPE_CRC_SOURCE_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OPP_PIPE_CRC_SOURCE_SELECT {
OPP_PIPE_CRC_SOURCE_SELECT_FMT           = 0x00000000u32,
OPP_PIPE_CRC_SOURCE_SELECT_SFT           = 0x00000001u32,
}

/*
 * OPP_PIPE_CRC_STEREO_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OPP_PIPE_CRC_STEREO_EN {
OPP_PIPE_CRC_STEREO_EN_INTERPRET_AS_NON_STEREO = 0x00000000u32,
OPP_PIPE_CRC_STEREO_EN_INTERPRET_AS_STEREO = 0x00000001u32,
}

/*
 * OPP_PIPE_CRC_STEREO_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OPP_PIPE_CRC_STEREO_MODE {
OPP_PIPE_CRC_STEREO_MODE_LEFT            = 0x00000000u32,
OPP_PIPE_CRC_STEREO_MODE_RIGHT           = 0x00000001u32,
OPP_PIPE_CRC_STEREO_MODE_BOTH_RESET_AFTER_RIGHT_EYE = 0x00000002u32,
OPP_PIPE_CRC_STEREO_MODE_BOTH_RESET_AFTER_EACH_EYE = 0x00000003u32,
}

/*******************************************************
 * OPP_TOP Enums
 *******************************************************/

/*
 * OPP_TEST_CLK_SEL_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OPP_TEST_CLK_SEL_CONTROL {
OPP_TEST_CLK_SEL_DISPCLK_P               = 0x00000000u32,
OPP_TEST_CLK_SEL_DISPCLK_R               = 0x00000001u32,
OPP_TEST_CLK_SEL_DISPCLK_ABM0            = 0x00000002u32,
OPP_TEST_CLK_SEL_DISPCLK_ABM1            = 0x00000003u32,
OPP_TEST_CLK_SEL_DISPCLK_ABM2            = 0x00000004u32,
OPP_TEST_CLK_SEL_DISPCLK_ABM3            = 0x00000005u32,
OPP_TEST_CLK_SEL_RESERVED0               = 0x00000006u32,
OPP_TEST_CLK_SEL_RESERVED1               = 0x00000007u32,
OPP_TEST_CLK_SEL_DISPCLK_OPP0            = 0x00000008u32,
OPP_TEST_CLK_SEL_DISPCLK_OPP1            = 0x00000009u32,
OPP_TEST_CLK_SEL_DISPCLK_OPP2            = 0x0000000au32,
OPP_TEST_CLK_SEL_DISPCLK_OPP3            = 0x0000000bu32,
OPP_TEST_CLK_SEL_RESERVED2               = 0x0000000cu32,
OPP_TEST_CLK_SEL_RESERVED3               = 0x0000000du32,
}

/*
 * OPP_TOP_CLOCK_ENABLE_STATUS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OPP_TOP_CLOCK_ENABLE_STATUS {
OPP_TOP_CLOCK_DISABLED_STATUS            = 0x00000000u32,
OPP_TOP_CLOCK_ENABLED_STATUS             = 0x00000001u32,
}

/*
 * OPP_TOP_CLOCK_GATING_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OPP_TOP_CLOCK_GATING_CONTROL {
OPP_TOP_CLOCK_GATING_ENABLED             = 0x00000000u32,
OPP_TOP_CLOCK_GATING_DISABLED            = 0x00000001u32,
}

/*******************************************************
 * OTG Enums
 *******************************************************/

/*
 * MASTER_UPDATE_LOCK_MASTER_UPDATE_LOCK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MASTER_UPDATE_LOCK_MASTER_UPDATE_LOCK {
MASTER_UPDATE_LOCK_MASTER_UPDATE_LOCK_FALSE = 0x00000000u32,
MASTER_UPDATE_LOCK_MASTER_UPDATE_LOCK_TRUE = 0x00000001u32,
}

/*
 * MASTER_UPDATE_LOCK_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MASTER_UPDATE_LOCK_SEL {
MASTER_UPDATE_LOCK_SEL_0                 = 0x00000000u32,
MASTER_UPDATE_LOCK_SEL_1                 = 0x00000001u32,
MASTER_UPDATE_LOCK_SEL_2                 = 0x00000002u32,
MASTER_UPDATE_LOCK_SEL_3                 = 0x00000003u32,
MASTER_UPDATE_LOCK_SEL_RESERVED4         = 0x00000004u32,
MASTER_UPDATE_LOCK_SEL_RESERVED5         = 0x00000005u32,
}

/*
 * MASTER_UPDATE_MODE_MASTER_UPDATE_INTERLACED_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MASTER_UPDATE_MODE_MASTER_UPDATE_INTERLACED_MODE {
MASTER_UPDATE_MODE_MASTER_UPDATE_INTERLACED_MODE_BOTH = 0x00000000u32,
MASTER_UPDATE_MODE_MASTER_UPDATE_INTERLACED_MODE_TOP = 0x00000001u32,
MASTER_UPDATE_MODE_MASTER_UPDATE_INTERLACED_MODE_BOTTOM = 0x00000002u32,
MASTER_UPDATE_MODE_MASTER_UPDATE_INTERLACED_MODE_RESERVED = 0x00000003u32,
}

/*
 * OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_EN {
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_EN_FALSE = 0x00000000u32,
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_EN_TRUE = 0x00000001u32,
}

/*
 * OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_EN_DB enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_EN_DB {
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_EN_DB_FALSE = 0x00000000u32,
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_EN_DB_TRUE = 0x00000001u32,
}

/*
 * OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_STEREO_SEL_OVR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_STEREO_SEL_OVR {
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_STEREO_SEL_OVR_FALSE = 0x00000000u32,
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_STEREO_SEL_OVR_TRUE = 0x00000001u32,
}

/*
 * OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_V_UPDATE_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_V_UPDATE_MODE {
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_V_UPDATE_MODE_BLOCK_BOTH = 0x00000000u32,
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_V_UPDATE_MODE_BLOCK_INTERLACE = 0x00000001u32,
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_V_UPDATE_MODE_BLOCK_PROGRASSIVE = 0x00000002u32,
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_V_UPDATE_MODE_RESERVED = 0x00000003u32,
}

/*
 * OTG_CONTROL_OTG_DISABLE_POINT_CNTL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_CONTROL_OTG_DISABLE_POINT_CNTL {
OTG_CONTROL_OTG_DISABLE_POINT_CNTL_DISABLE = 0x00000000u32,
OTG_CONTROL_OTG_DISABLE_POINT_CNTL_DISABLE_CURRENT = 0x00000001u32,
OTG_CONTROL_OTG_DISABLE_POINT_CNTL_DISABLE_VUPDATE = 0x00000002u32,
OTG_CONTROL_OTG_DISABLE_POINT_CNTL_DISABLE_FIRST = 0x00000003u32,
}

/*
 * OTG_CONTROL_OTG_FIELD_NUMBER_CNTL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_CONTROL_OTG_FIELD_NUMBER_CNTL {
OTG_CONTROL_OTG_FIELD_NUMBER_CNTL_NORMAL = 0x00000000u32,
OTG_CONTROL_OTG_FIELD_NUMBER_CNTL_DP     = 0x00000001u32,
}

/*
 * OTG_CONTROL_OTG_FIELD_NUMBER_POLARITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_CONTROL_OTG_FIELD_NUMBER_POLARITY {
OTG_CONTROL_OTG_FIELD_NUMBER_POLARITY_FALSE = 0x00000000u32,
OTG_CONTROL_OTG_FIELD_NUMBER_POLARITY_TRUE = 0x00000001u32,
}

/*
 * OTG_CONTROL_OTG_MASTER_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_CONTROL_OTG_MASTER_EN {
OTG_CONTROL_OTG_MASTER_EN_FALSE          = 0x00000000u32,
OTG_CONTROL_OTG_MASTER_EN_TRUE           = 0x00000001u32,
}

/*
 * OTG_CONTROL_OTG_OUT_MUX enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_CONTROL_OTG_OUT_MUX {
OTG_CONTROL_OTG_OUT_MUX_0                = 0x00000000u32,
OTG_CONTROL_OTG_OUT_MUX_1                = 0x00000001u32,
OTG_CONTROL_OTG_OUT_MUX_2                = 0x00000002u32,
}

/*
 * OTG_CONTROL_OTG_START_POINT_CNTL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_CONTROL_OTG_START_POINT_CNTL {
OTG_CONTROL_OTG_START_POINT_CNTL_NORMAL  = 0x00000000u32,
OTG_CONTROL_OTG_START_POINT_CNTL_DP      = 0x00000001u32,
}

/*
 * OTG_COUNT_CONTROL_OTG_HORZ_COUNT_BY2_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_COUNT_CONTROL_OTG_HORZ_COUNT_BY2_EN {
OTG_COUNT_CONTROL_OTG_HORZ_COUNT_BY2_EN_FALSE = 0x00000000u32,
OTG_COUNT_CONTROL_OTG_HORZ_COUNT_BY2_EN_TRUE = 0x00000001u32,
}

/*
 * OTG_CRC_CNTL_OTG_CRC1_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_CRC_CNTL_OTG_CRC1_EN {
OTG_CRC_CNTL_OTG_CRC1_EN_FALSE           = 0x00000000u32,
OTG_CRC_CNTL_OTG_CRC1_EN_TRUE            = 0x00000001u32,
}

/*
 * OTG_CRC_CNTL_OTG_CRC_CONT_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_CRC_CNTL_OTG_CRC_CONT_EN {
OTG_CRC_CNTL_OTG_CRC_CONT_EN_FALSE       = 0x00000000u32,
OTG_CRC_CNTL_OTG_CRC_CONT_EN_TRUE        = 0x00000001u32,
}

/*
 * OTG_CRC_CNTL_OTG_CRC_CONT_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_CRC_CNTL_OTG_CRC_CONT_MODE {
OTG_CRC_CNTL_OTG_CRC_CONT_MODE_RESET     = 0x00000000u32,
OTG_CRC_CNTL_OTG_CRC_CONT_MODE_NORESET   = 0x00000001u32,
}

/*
 * OTG_CRC_CNTL_OTG_CRC_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_CRC_CNTL_OTG_CRC_EN {
OTG_CRC_CNTL_OTG_CRC_EN_FALSE            = 0x00000000u32,
OTG_CRC_CNTL_OTG_CRC_EN_TRUE             = 0x00000001u32,
}

/*
 * OTG_CRC_CNTL_OTG_CRC_INTERLACE_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_CRC_CNTL_OTG_CRC_INTERLACE_MODE {
OTG_CRC_CNTL_OTG_CRC_INTERLACE_MODE_TOP  = 0x00000000u32,
OTG_CRC_CNTL_OTG_CRC_INTERLACE_MODE_BOTTOM = 0x00000001u32,
OTG_CRC_CNTL_OTG_CRC_INTERLACE_MODE_BOTH_BOTTOM = 0x00000002u32,
OTG_CRC_CNTL_OTG_CRC_INTERLACE_MODE_BOTH_FIELD = 0x00000003u32,
}

/*
 * OTG_CRC_CNTL_OTG_CRC_STEREO_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_CRC_CNTL_OTG_CRC_STEREO_MODE {
OTG_CRC_CNTL_OTG_CRC_STEREO_MODE_LEFT    = 0x00000000u32,
OTG_CRC_CNTL_OTG_CRC_STEREO_MODE_RIGHT   = 0x00000001u32,
OTG_CRC_CNTL_OTG_CRC_STEREO_MODE_BOTH_EYES = 0x00000002u32,
OTG_CRC_CNTL_OTG_CRC_STEREO_MODE_BOTH_FIELDS = 0x00000003u32,
}

/*
 * OTG_CRC_CNTL_OTG_CRC_USE_NEW_AND_REPEATED_PIXELS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_CRC_CNTL_OTG_CRC_USE_NEW_AND_REPEATED_PIXELS {
OTG_CRC_CNTL_OTG_CRC_USE_NEW_AND_REPEATED_PIXELS_FALSE = 0x00000000u32,
OTG_CRC_CNTL_OTG_CRC_USE_NEW_AND_REPEATED_PIXELS_TRUE = 0x00000001u32,
}

/*
 * OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT {
OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT_UAB     = 0x00000000u32,
OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT_UA_B    = 0x00000001u32,
OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT_U_AB    = 0x00000002u32,
OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT_U_A_B   = 0x00000003u32,
OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT_IAB     = 0x00000004u32,
OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT_IA_B    = 0x00000005u32,
OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT_I_AB    = 0x00000006u32,
OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT_I_A_B   = 0x00000007u32,
}

/*
 * OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT {
OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT_UAB     = 0x00000000u32,
OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT_UA_B    = 0x00000001u32,
OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT_U_AB    = 0x00000002u32,
OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT_U_A_B   = 0x00000003u32,
OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT_IAB     = 0x00000004u32,
OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT_IA_B    = 0x00000005u32,
OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT_I_AB    = 0x00000006u32,
OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT_I_A_B   = 0x00000007u32,
}

/*
 * OTG_DIG_UPDATE_VCOUNT_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_DIG_UPDATE_VCOUNT_MODE {
OTG_DIG_UPDATE_VCOUNT_0                  = 0x00000000u32,
OTG_DIG_UPDATE_VCOUNT_1                  = 0x00000001u32,
}

/*
 * OTG_DLPC_CONTROL_OTG_RESYNC_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_DLPC_CONTROL_OTG_RESYNC_MODE {
OTG_DLPC_CONTROL_OTG_RESYNC_MODE_0       = 0x00000000u32,
OTG_DLPC_CONTROL_OTG_RESYNC_MODE_1       = 0x00000001u32,
}

/*
 * OTG_DOUBLE_BUFFER_CONTROL_OTG_DRR_TIMING_DBUF_UPDATE_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_DOUBLE_BUFFER_CONTROL_OTG_DRR_TIMING_DBUF_UPDATE_MODE {
OTG_DOUBLE_BUFFER_CONTROL_OTG_DRR_TIMING_DBUF_UPDATE_MODE_0 = 0x00000000u32,
OTG_DOUBLE_BUFFER_CONTROL_OTG_DRR_TIMING_DBUF_UPDATE_MODE_1 = 0x00000001u32,
OTG_DOUBLE_BUFFER_CONTROL_OTG_DRR_TIMING_DBUF_UPDATE_MODE_2 = 0x00000002u32,
OTG_DOUBLE_BUFFER_CONTROL_OTG_DRR_TIMING_DBUF_UPDATE_MODE_3 = 0x00000003u32,
}

/*
 * OTG_DOUBLE_BUFFER_CONTROL_OTG_UPDATE_INSTANTLY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_DOUBLE_BUFFER_CONTROL_OTG_UPDATE_INSTANTLY {
OTG_DOUBLE_BUFFER_CONTROL_OTG_UPDATE_INSTANTLY_FALSE = 0x00000000u32,
OTG_DOUBLE_BUFFER_CONTROL_OTG_UPDATE_INSTANTLY_TRUE = 0x00000001u32,
}

/*
 * OTG_DRR_CONTROL_OTG_DRR_AVERAGE_FRAME enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_DRR_CONTROL_OTG_DRR_AVERAGE_FRAME {
OTG_DRR_CONTROL_OTG_DRR_AVERAGE_FRAME_1FRAME = 0x00000000u32,
OTG_DRR_CONTROL_OTG_DRR_AVERAGE_FRAME_2FRAME = 0x00000001u32,
OTG_DRR_CONTROL_OTG_DRR_AVERAGE_FRAME_4FRAME = 0x00000002u32,
OTG_DRR_CONTROL_OTG_DRR_AVERAGE_FRAME_8FRAME = 0x00000003u32,
}

/*
 * OTG_DTMTEST_CNTL_OTG_DTMTEST_OTG_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_DTMTEST_CNTL_OTG_DTMTEST_OTG_EN {
OTG_DTMTEST_CNTL_OTG_DTMTEST_OTG_EN_FALSE = 0x00000000u32,
OTG_DTMTEST_CNTL_OTG_DTMTEST_OTG_EN_TRUE = 0x00000001u32,
}

/*
 * OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_GRANULARITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_GRANULARITY {
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_GRANULARITY_FALSE = 0x00000000u32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_GRANULARITY_TRUE = 0x00000001u32,
}

/*
 * OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_POLARITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_POLARITY {
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_POLARITY_FALSE = 0x00000000u32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_POLARITY_TRUE = 0x00000001u32,
}

/*
 * OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT {
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_LOGIC0 = 0x00000000u32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_LOGIC1 = 0x00000001u32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_GENERICA = 0x00000002u32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_GENERICB = 0x00000003u32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_GENERICC = 0x00000004u32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_GENERICD = 0x00000005u32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_GENERICE = 0x00000006u32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_GENERICF = 0x00000007u32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_HPD1 = 0x00000008u32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_HPD2 = 0x00000009u32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_DDC1DATA = 0x0000000au32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_DDC1CLK = 0x0000000bu32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_DDC2DATA = 0x0000000cu32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_DDC2CLK = 0x0000000du32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_MANUAL_FLOW_CONTROL = 0x0000000eu32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_RESERVED = 0x0000000fu32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_GENLK_CLK = 0x00000010u32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_GENLK_VSYNC = 0x00000011u32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_SWAPLOCKA = 0x00000012u32,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_SWAPLOCKB = 0x00000013u32,
}

/*
 * OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_CHECK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_CHECK {
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_CHECK_FALSE = 0x00000000u32,
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_CHECK_TRUE = 0x00000001u32,
}

/*
 * OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_CLEAR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_CLEAR {
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_CLEAR_FALSE = 0x00000000u32,
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_CLEAR_TRUE = 0x00000001u32,
}

/*
 * OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_MODE {
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_MODE_DISABLE = 0x00000000u32,
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_MODE_HCOUNT = 0x00000001u32,
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_MODE_HCOUNT_VCOUNT = 0x00000002u32,
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_MODE_RESERVED = 0x00000003u32,
}

/*
 * OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_TRIG_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_TRIG_SEL {
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_TRIG_SEL_FALSE = 0x00000000u32,
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_TRIG_SEL_TRUE = 0x00000001u32,
}

/*
 * OTG_GLOBAL_CONTROL2_MANUAL_FLOW_CONTROL_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_GLOBAL_CONTROL2_MANUAL_FLOW_CONTROL_SEL {
OTG_GLOBAL_CONTROL2_MANUAL_FLOW_CONTROL_SEL_OTG0 = 0x00000000u32,
OTG_GLOBAL_CONTROL2_MANUAL_FLOW_CONTROL_SEL_OTG1 = 0x00000001u32,
OTG_GLOBAL_CONTROL2_MANUAL_FLOW_CONTROL_SEL_OTG2 = 0x00000002u32,
OTG_GLOBAL_CONTROL2_MANUAL_FLOW_CONTROL_SEL_OTG3 = 0x00000003u32,
OTG_GLOBAL_CONTROL2_MANUAL_FLOW_CONTROL_SEL_RESERVED4 = 0x00000004u32,
OTG_GLOBAL_CONTROL2_MANUAL_FLOW_CONTROL_SEL_RESERVED5 = 0x00000005u32,
}

/*
 * OTG_GLOBAL_CONTROL3_DIG_UPDATE_EYE_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_GLOBAL_CONTROL3_DIG_UPDATE_EYE_SEL {
DIG_UPDATE_EYE_SEL_BOTH                  = 0x00000000u32,
DIG_UPDATE_EYE_SEL_LEFT                  = 0x00000001u32,
DIG_UPDATE_EYE_SEL_RIGHT                 = 0x00000002u32,
}

/*
 * OTG_GLOBAL_CONTROL3_DIG_UPDATE_FIELD_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_GLOBAL_CONTROL3_DIG_UPDATE_FIELD_SEL {
DIG_UPDATE_FIELD_SEL_BOTH                = 0x00000000u32,
DIG_UPDATE_FIELD_SEL_TOP                 = 0x00000001u32,
DIG_UPDATE_FIELD_SEL_BOTTOM              = 0x00000002u32,
DIG_UPDATE_FIELD_SEL_RESERVED            = 0x00000003u32,
}

/*
 * OTG_GLOBAL_CONTROL3_MASTER_UPDATE_LOCK_DB_FIELD enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_GLOBAL_CONTROL3_MASTER_UPDATE_LOCK_DB_FIELD {
MASTER_UPDATE_LOCK_DB_FIELD_BOTH         = 0x00000000u32,
MASTER_UPDATE_LOCK_DB_FIELD_TOP          = 0x00000001u32,
MASTER_UPDATE_LOCK_DB_FIELD_BOTTOM       = 0x00000002u32,
MASTER_UPDATE_LOCK_DB_FIELD_RESERVED     = 0x00000003u32,
}

/*
 * OTG_GLOBAL_CONTROL3_MASTER_UPDATE_LOCK_DB_STEREO_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_GLOBAL_CONTROL3_MASTER_UPDATE_LOCK_DB_STEREO_SEL {
MASTER_UPDATE_LOCK_DB_STEREO_SEL_BOTH    = 0x00000000u32,
MASTER_UPDATE_LOCK_DB_STEREO_SEL_LEFT    = 0x00000001u32,
MASTER_UPDATE_LOCK_DB_STEREO_SEL_RIGHT   = 0x00000002u32,
MASTER_UPDATE_LOCK_DB_STEREO_SEL_RESERVED = 0x00000003u32,
}

/*
 * OTG_GLOBAL_UPDATE_LOCK_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_GLOBAL_UPDATE_LOCK_EN {
OTG_GLOBAL_UPDATE_LOCK_DISABLE           = 0x00000000u32,
OTG_GLOBAL_UPDATE_LOCK_ENABLE            = 0x00000001u32,
}

/*
 * OTG_GSL_MASTER_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_GSL_MASTER_MODE {
OTG_GSL_MASTER_MODE_0                    = 0x00000000u32,
OTG_GSL_MASTER_MODE_1                    = 0x00000001u32,
OTG_GSL_MASTER_MODE_2                    = 0x00000002u32,
OTG_GSL_MASTER_MODE_3                    = 0x00000003u32,
}

/*
 * OTG_HORZ_REPETITION_COUNT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_HORZ_REPETITION_COUNT {
OTG_HORZ_REPETITION_COUNT_0              = 0x00000000u32,
OTG_HORZ_REPETITION_COUNT_1              = 0x00000001u32,
OTG_HORZ_REPETITION_COUNT_2              = 0x00000002u32,
OTG_HORZ_REPETITION_COUNT_3              = 0x00000003u32,
OTG_HORZ_REPETITION_COUNT_4              = 0x00000004u32,
OTG_HORZ_REPETITION_COUNT_5              = 0x00000005u32,
OTG_HORZ_REPETITION_COUNT_6              = 0x00000006u32,
OTG_HORZ_REPETITION_COUNT_7              = 0x00000007u32,
OTG_HORZ_REPETITION_COUNT_8              = 0x00000008u32,
OTG_HORZ_REPETITION_COUNT_9              = 0x00000009u32,
OTG_HORZ_REPETITION_COUNT_10             = 0x0000000au32,
OTG_HORZ_REPETITION_COUNT_11             = 0x0000000bu32,
OTG_HORZ_REPETITION_COUNT_12             = 0x0000000cu32,
OTG_HORZ_REPETITION_COUNT_13             = 0x0000000du32,
OTG_HORZ_REPETITION_COUNT_14             = 0x0000000eu32,
OTG_HORZ_REPETITION_COUNT_15             = 0x0000000fu32,
}

/*
 * OTG_H_SYNC_A_POL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_H_SYNC_A_POL {
OTG_H_SYNC_A_POL_HIGH                    = 0x00000000u32,
OTG_H_SYNC_A_POL_LOW                     = 0x00000001u32,
}

/*
 * OTG_H_TIMING_DIV_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_H_TIMING_DIV_MODE {
OTG_H_TIMING_DIV_MODE_NO_DIV             = 0x00000000u32,
OTG_H_TIMING_DIV_MODE_DIV_BY2            = 0x00000001u32,
OTG_H_TIMING_DIV_MODE_RESERVED           = 0x00000002u32,
OTG_H_TIMING_DIV_MODE_DIV_BY4            = 0x00000003u32,
}

/*
 * OTG_H_TIMING_DIV_MODE_MANUAL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_H_TIMING_DIV_MODE_MANUAL {
OTG_H_TIMING_DIV_MODE_AUTO               = 0x00000000u32,
OTG_H_TIMING_DIV_MODE_NOAUTO             = 0x00000001u32,
}

/*
 * OTG_INTERLACE_CONTROL_OTG_INTERLACE_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_INTERLACE_CONTROL_OTG_INTERLACE_ENABLE {
OTG_INTERLACE_CONTROL_OTG_INTERLACE_ENABLE_FALSE = 0x00000000u32,
OTG_INTERLACE_CONTROL_OTG_INTERLACE_ENABLE_TRUE = 0x00000001u32,
}

/*
 * OTG_INTERLACE_CONTROL_OTG_INTERLACE_FORCE_NEXT_FIELD enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_INTERLACE_CONTROL_OTG_INTERLACE_FORCE_NEXT_FIELD {
OTG_INTERLACE_CONTROL_OTG_INTERLACE_FORCE_NEXT_FIELD_NOT = 0x00000000u32,
OTG_INTERLACE_CONTROL_OTG_INTERLACE_FORCE_NEXT_FIELD_BOTTOM = 0x00000001u32,
OTG_INTERLACE_CONTROL_OTG_INTERLACE_FORCE_NEXT_FIELD_TOP = 0x00000002u32,
OTG_INTERLACE_CONTROL_OTG_INTERLACE_FORCE_NEXT_FIELD_NOT2 = 0x00000003u32,
}

/*
 * OTG_INTERRUPT_CONTROL_OTG_FORCE_COUNT_NOW_INT_MSK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_INTERRUPT_CONTROL_OTG_FORCE_COUNT_NOW_INT_MSK {
OTG_INTERRUPT_CONTROL_OTG_FORCE_COUNT_NOW_INT_MSK_FALSE = 0x00000000u32,
OTG_INTERRUPT_CONTROL_OTG_FORCE_COUNT_NOW_INT_MSK_TRUE = 0x00000001u32,
}

/*
 * OTG_INTERRUPT_CONTROL_OTG_FORCE_COUNT_NOW_INT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_INTERRUPT_CONTROL_OTG_FORCE_COUNT_NOW_INT_TYPE {
OTG_INTERRUPT_CONTROL_OTG_FORCE_COUNT_NOW_INT_TYPE_FALSE = 0x00000000u32,
OTG_INTERRUPT_CONTROL_OTG_FORCE_COUNT_NOW_INT_TYPE_TRUE = 0x00000001u32,
}

/*
 * OTG_INTERRUPT_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_INT_MSK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_INTERRUPT_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_INT_MSK {
OTG_INTERRUPT_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_INT_MSK_FALSE = 0x00000000u32,
OTG_INTERRUPT_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_INT_MSK_TRUE = 0x00000001u32,
}

/*
 * OTG_INTERRUPT_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_INT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_INTERRUPT_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_INT_TYPE {
OTG_INTERRUPT_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_INT_TYPE_FALSE = 0x00000000u32,
OTG_INTERRUPT_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_INT_TYPE_TRUE = 0x00000001u32,
}

/*
 * OTG_INTERRUPT_CONTROL_OTG_GSL_VSYNC_GAP_INT_MSK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_INTERRUPT_CONTROL_OTG_GSL_VSYNC_GAP_INT_MSK {
OTG_INTERRUPT_CONTROL_OTG_GSL_VSYNC_GAP_INT_MSK_FALSE = 0x00000000u32,
OTG_INTERRUPT_CONTROL_OTG_GSL_VSYNC_GAP_INT_MSK_TRUE = 0x00000001u32,
}

/*
 * OTG_INTERRUPT_CONTROL_OTG_GSL_VSYNC_GAP_INT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_INTERRUPT_CONTROL_OTG_GSL_VSYNC_GAP_INT_TYPE {
OTG_INTERRUPT_CONTROL_OTG_GSL_VSYNC_GAP_INT_TYPE_FALSE = 0x00000000u32,
OTG_INTERRUPT_CONTROL_OTG_GSL_VSYNC_GAP_INT_TYPE_TRUE = 0x00000001u32,
}

/*
 * OTG_INTERRUPT_CONTROL_OTG_SNAPSHOT_INT_MSK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_INTERRUPT_CONTROL_OTG_SNAPSHOT_INT_MSK {
OTG_INTERRUPT_CONTROL_OTG_SNAPSHOT_INT_MSK_FALSE = 0x00000000u32,
OTG_INTERRUPT_CONTROL_OTG_SNAPSHOT_INT_MSK_TRUE = 0x00000001u32,
}

/*
 * OTG_INTERRUPT_CONTROL_OTG_SNAPSHOT_INT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_INTERRUPT_CONTROL_OTG_SNAPSHOT_INT_TYPE {
OTG_INTERRUPT_CONTROL_OTG_SNAPSHOT_INT_TYPE_FALSE = 0x00000000u32,
OTG_INTERRUPT_CONTROL_OTG_SNAPSHOT_INT_TYPE_TRUE = 0x00000001u32,
}

/*
 * OTG_INTERRUPT_CONTROL_OTG_TRIGA_INT_MSK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_INTERRUPT_CONTROL_OTG_TRIGA_INT_MSK {
OTG_INTERRUPT_CONTROL_OTG_TRIGA_INT_MSK_FALSE = 0x00000000u32,
OTG_INTERRUPT_CONTROL_OTG_TRIGA_INT_MSK_TRUE = 0x00000001u32,
}

/*
 * OTG_INTERRUPT_CONTROL_OTG_TRIGA_INT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_INTERRUPT_CONTROL_OTG_TRIGA_INT_TYPE {
OTG_INTERRUPT_CONTROL_OTG_TRIGA_INT_TYPE_FALSE = 0x00000000u32,
OTG_INTERRUPT_CONTROL_OTG_TRIGA_INT_TYPE_TRUE = 0x00000001u32,
}

/*
 * OTG_INTERRUPT_CONTROL_OTG_TRIGB_INT_MSK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_INTERRUPT_CONTROL_OTG_TRIGB_INT_MSK {
OTG_INTERRUPT_CONTROL_OTG_TRIGB_INT_MSK_FALSE = 0x00000000u32,
OTG_INTERRUPT_CONTROL_OTG_TRIGB_INT_MSK_TRUE = 0x00000001u32,
}

/*
 * OTG_INTERRUPT_CONTROL_OTG_TRIGB_INT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_INTERRUPT_CONTROL_OTG_TRIGB_INT_TYPE {
OTG_INTERRUPT_CONTROL_OTG_TRIGB_INT_TYPE_FALSE = 0x00000000u32,
OTG_INTERRUPT_CONTROL_OTG_TRIGB_INT_TYPE_TRUE = 0x00000001u32,
}

/*
 * OTG_INTERRUPT_CONTROL_OTG_VSYNC_NOM_INT_MSK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_INTERRUPT_CONTROL_OTG_VSYNC_NOM_INT_MSK {
OTG_INTERRUPT_CONTROL_OTG_VSYNC_NOM_INT_MSK_FALSE = 0x00000000u32,
OTG_INTERRUPT_CONTROL_OTG_VSYNC_NOM_INT_MSK_TRUE = 0x00000001u32,
}

/*
 * OTG_INTERRUPT_CONTROL_OTG_VSYNC_NOM_INT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_INTERRUPT_CONTROL_OTG_VSYNC_NOM_INT_TYPE {
OTG_INTERRUPT_CONTROL_OTG_VSYNC_NOM_INT_TYPE_FALSE = 0x00000000u32,
OTG_INTERRUPT_CONTROL_OTG_VSYNC_NOM_INT_TYPE_TRUE = 0x00000001u32,
}

/*
 * OTG_MANUAL_FORCE_VSYNC_NEXT_LINE_OTG_MANUAL_FORCE_VSYNC_NEXT_LINE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_MANUAL_FORCE_VSYNC_NEXT_LINE_OTG_MANUAL_FORCE_VSYNC_NEXT_LINE {
OTG_MANUAL_FORCE_VSYNC_NEXT_LINE_OTG_MANUAL_FORCE_VSYNC_NEXT_LINE_FALSE = 0x00000000u32,
OTG_MANUAL_FORCE_VSYNC_NEXT_LINE_OTG_MANUAL_FORCE_VSYNC_NEXT_LINE_TRUE = 0x00000001u32,
}

/*
 * OTG_MASTER_UPDATE_LOCK_DB_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_MASTER_UPDATE_LOCK_DB_EN {
OTG_MASTER_UPDATE_LOCK_DISABLE           = 0x00000000u32,
OTG_MASTER_UPDATE_LOCK_ENABLE            = 0x00000001u32,
}

/*
 * OTG_MASTER_UPDATE_LOCK_GSL_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_MASTER_UPDATE_LOCK_GSL_EN {
OTG_MASTER_UPDATE_LOCK_GSL_EN_FALSE      = 0x00000000u32,
OTG_MASTER_UPDATE_LOCK_GSL_EN_TRUE       = 0x00000001u32,
}

/*
 * OTG_MASTER_UPDATE_LOCK_VCOUNT_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_MASTER_UPDATE_LOCK_VCOUNT_MODE {
OTG_MASTER_UPDATE_LOCK_VCOUNT_0          = 0x00000000u32,
OTG_MASTER_UPDATE_LOCK_VCOUNT_1          = 0x00000001u32,
}

/*
 * OTG_SNAPSHOT_CONTROL_OTG_AUTO_SNAPSHOT_TRIG_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_SNAPSHOT_CONTROL_OTG_AUTO_SNAPSHOT_TRIG_SEL {
OTG_SNAPSHOT_CONTROL_OTG_AUTO_SNAPSHOT_TRIG_SEL_DISABLE = 0x00000000u32,
OTG_SNAPSHOT_CONTROL_OTG_AUTO_SNAPSHOT_TRIG_SEL_TRIGGERA = 0x00000001u32,
OTG_SNAPSHOT_CONTROL_OTG_AUTO_SNAPSHOT_TRIG_SEL_TRIGGERB = 0x00000002u32,
OTG_SNAPSHOT_CONTROL_OTG_AUTO_SNAPSHOT_TRIG_SEL_RESERVED = 0x00000003u32,
}

/*
 * OTG_SNAPSHOT_STATUS_OTG_SNAPSHOT_CLEAR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_SNAPSHOT_STATUS_OTG_SNAPSHOT_CLEAR {
OTG_SNAPSHOT_STATUS_OTG_SNAPSHOT_CLEAR_FALSE = 0x00000000u32,
OTG_SNAPSHOT_STATUS_OTG_SNAPSHOT_CLEAR_TRUE = 0x00000001u32,
}

/*
 * OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_CLEAR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_CLEAR {
OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_CLEAR_FALSE = 0x00000000u32,
OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_CLEAR_TRUE = 0x00000001u32,
}

/*
 * OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_ENABLE {
OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_ENABLE_FALSE = 0x00000000u32,
OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_ENABLE_TRUE = 0x00000001u32,
}

/*
 * OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_TYPE {
OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_TYPE_FALSE = 0x00000000u32,
OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_TYPE_TRUE = 0x00000001u32,
}

/*
 * OTG_STATIC_SCREEN_CONTROL_OTG_STATIC_SCREEN_OVERRIDE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_STATIC_SCREEN_CONTROL_OTG_STATIC_SCREEN_OVERRIDE {
OTG_STATIC_SCREEN_CONTROL_OTG_STATIC_SCREEN_OVERRIDE_FALSE = 0x00000000u32,
OTG_STATIC_SCREEN_CONTROL_OTG_STATIC_SCREEN_OVERRIDE_TRUE = 0x00000001u32,
}

/*
 * OTG_STATIC_SCREEN_CONTROL_OTG_STATIC_SCREEN_OVERRIDE_VALUE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_STATIC_SCREEN_CONTROL_OTG_STATIC_SCREEN_OVERRIDE_VALUE {
OTG_STATIC_SCREEN_CONTROL_OTG_STATIC_SCREEN_OVERRIDE_VALUE_OFF = 0x00000000u32,
OTG_STATIC_SCREEN_CONTROL_OTG_STATIC_SCREEN_OVERRIDE_VALUE_ON = 0x00000001u32,
}

/*
 * OTG_STEREO_CONTROL_OTG_FIELD_NUM_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_STEREO_CONTROL_OTG_FIELD_NUM_SEL {
OTG_STEREO_CONTROL_OTG_FIELD_NUM_SEL_FALSE = 0x00000000u32,
OTG_STEREO_CONTROL_OTG_FIELD_NUM_SEL_TRUE = 0x00000001u32,
}

/*
 * OTG_STEREO_CONTROL_OTG_STEREO_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_STEREO_CONTROL_OTG_STEREO_EN {
OTG_STEREO_CONTROL_OTG_STEREO_EN_FALSE   = 0x00000000u32,
OTG_STEREO_CONTROL_OTG_STEREO_EN_TRUE    = 0x00000001u32,
}

/*
 * OTG_STEREO_CONTROL_OTG_STEREO_EYE_FLAG_POLARITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_STEREO_CONTROL_OTG_STEREO_EYE_FLAG_POLARITY {
OTG_STEREO_CONTROL_OTG_STEREO_EYE_FLAG_POLARITY_FALSE = 0x00000000u32,
OTG_STEREO_CONTROL_OTG_STEREO_EYE_FLAG_POLARITY_TRUE = 0x00000001u32,
}

/*
 * OTG_STEREO_CONTROL_OTG_STEREO_SYNC_OUTPUT_POLARITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_STEREO_CONTROL_OTG_STEREO_SYNC_OUTPUT_POLARITY {
OTG_STEREO_CONTROL_OTG_STEREO_SYNC_OUTPUT_POLARITY_FALSE = 0x00000000u32,
OTG_STEREO_CONTROL_OTG_STEREO_SYNC_OUTPUT_POLARITY_TRUE = 0x00000001u32,
}

/*
 * OTG_STEREO_FORCE_NEXT_EYE_OTG_STEREO_FORCE_NEXT_EYE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_STEREO_FORCE_NEXT_EYE_OTG_STEREO_FORCE_NEXT_EYE {
OTG_STEREO_FORCE_NEXT_EYE_OTG_STEREO_FORCE_NEXT_EYE_NO = 0x00000000u32,
OTG_STEREO_FORCE_NEXT_EYE_OTG_STEREO_FORCE_NEXT_EYE_RIGHT = 0x00000001u32,
OTG_STEREO_FORCE_NEXT_EYE_OTG_STEREO_FORCE_NEXT_EYE_LEFT = 0x00000002u32,
OTG_STEREO_FORCE_NEXT_EYE_OTG_STEREO_FORCE_NEXT_EYE_RESERVED = 0x00000003u32,
}

/*
 * OTG_TRIGA_CNTL_OTG_TRIGA_CLEAR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_TRIGA_CNTL_OTG_TRIGA_CLEAR {
OTG_TRIGA_CNTL_OTG_TRIGA_CLEAR_FALSE     = 0x00000000u32,
OTG_TRIGA_CNTL_OTG_TRIGA_CLEAR_TRUE      = 0x00000001u32,
}

/*
 * OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT {
OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT_LOGIC0 = 0x00000000u32,
OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT_INTERLACE = 0x00000001u32,
OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT_GENERICA = 0x00000002u32,
OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT_GENERICB = 0x00000003u32,
OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT_HSYNCA = 0x00000004u32,
OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT_LOGIC1 = 0x00000005u32,
OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT_GENERICC = 0x00000006u32,
OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT_GENERICD = 0x00000007u32,
}

/*
 * OTG_TRIGA_CNTL_OTG_TRIGA_RESYNC_BYPASS_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_TRIGA_CNTL_OTG_TRIGA_RESYNC_BYPASS_EN {
OTG_TRIGA_CNTL_OTG_TRIGA_RESYNC_BYPASS_EN_FALSE = 0x00000000u32,
OTG_TRIGA_CNTL_OTG_TRIGA_RESYNC_BYPASS_EN_TRUE = 0x00000001u32,
}

/*
 * OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_PIPE_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_PIPE_SELECT {
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_PIPE_SELECT_OTG0 = 0x00000000u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_PIPE_SELECT_OTG1 = 0x00000001u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_PIPE_SELECT_OTG2 = 0x00000002u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_PIPE_SELECT_OTG3 = 0x00000003u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_PIPE_SELECT_RESERVED4 = 0x00000004u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_PIPE_SELECT_RESERVED5 = 0x00000005u32,
}

/*
 * OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT {
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_LOGIC0 = 0x00000000u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GENERICA_PIN = 0x00000001u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GENERICB_PIN = 0x00000002u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GENERICC_PIN = 0x00000003u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GENERICD_PIN = 0x00000004u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GENERICE_PIN = 0x00000005u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GENERICF_PIN = 0x00000006u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_SWAPLOCKA_PIN = 0x00000007u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_SWAPLOCKB_PIN = 0x00000008u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GENLK_CLK_PIN = 0x00000009u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GENLK_VSYNC_PIN = 0x0000000au32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_HPD1 = 0x0000000bu32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_HPD2 = 0x0000000cu32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_BLON_Y_PIN = 0x0000000du32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_RESERVED14 = 0x0000000eu32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_UPDATE_LOCK = 0x0000000fu32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GSL_ALLOW_FLIP = 0x00000010u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_UPDATE_PENDING = 0x00000011u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_OTG_SOF = 0x00000012u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_HSYNC = 0x00000013u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_VSYNC = 0x00000014u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_OTG_TRIG_MANUAL_CONTROL = 0x00000015u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_MANUAL_FLOW_CONTROL = 0x00000016u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_LOGIC1 = 0x00000017u32,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_FLIP_PENDING = 0x00000018u32,
}

/*
 * OTG_TRIGA_FALLING_EDGE_DETECT_CNTL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_TRIGA_FALLING_EDGE_DETECT_CNTL {
OTG_TRIGA_FALLING_EDGE_DETECT_CNTL_0     = 0x00000000u32,
OTG_TRIGA_FALLING_EDGE_DETECT_CNTL_1     = 0x00000001u32,
OTG_TRIGA_FALLING_EDGE_DETECT_CNTL_2     = 0x00000002u32,
OTG_TRIGA_FALLING_EDGE_DETECT_CNTL_3     = 0x00000003u32,
}

/*
 * OTG_TRIGA_FREQUENCY_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_TRIGA_FREQUENCY_SELECT {
OTG_TRIGA_FREQUENCY_SELECT_0             = 0x00000000u32,
OTG_TRIGA_FREQUENCY_SELECT_1             = 0x00000001u32,
OTG_TRIGA_FREQUENCY_SELECT_2             = 0x00000002u32,
OTG_TRIGA_FREQUENCY_SELECT_3             = 0x00000003u32,
}

/*
 * OTG_TRIGA_RISING_EDGE_DETECT_CNTL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_TRIGA_RISING_EDGE_DETECT_CNTL {
OTG_TRIGA_RISING_EDGE_DETECT_CNTL_0      = 0x00000000u32,
OTG_TRIGA_RISING_EDGE_DETECT_CNTL_1      = 0x00000001u32,
OTG_TRIGA_RISING_EDGE_DETECT_CNTL_2      = 0x00000002u32,
OTG_TRIGA_RISING_EDGE_DETECT_CNTL_3      = 0x00000003u32,
}

/*
 * OTG_TRIGB_CNTL_OTG_TRIGB_CLEAR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_TRIGB_CNTL_OTG_TRIGB_CLEAR {
OTG_TRIGB_CNTL_OTG_TRIGB_CLEAR_FALSE     = 0x00000000u32,
OTG_TRIGB_CNTL_OTG_TRIGB_CLEAR_TRUE      = 0x00000001u32,
}

/*
 * OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT {
OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT_LOGIC0 = 0x00000000u32,
OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT_INTERLACE = 0x00000001u32,
OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT_GENERICA = 0x00000002u32,
OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT_GENERICB = 0x00000003u32,
OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT_HSYNCA = 0x00000004u32,
OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT_LOGIC1 = 0x00000005u32,
OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT_GENERICC = 0x00000006u32,
OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT_GENERICD = 0x00000007u32,
}

/*
 * OTG_TRIGB_CNTL_OTG_TRIGB_RESYNC_BYPASS_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_TRIGB_CNTL_OTG_TRIGB_RESYNC_BYPASS_EN {
OTG_TRIGB_CNTL_OTG_TRIGB_RESYNC_BYPASS_EN_FALSE = 0x00000000u32,
OTG_TRIGB_CNTL_OTG_TRIGB_RESYNC_BYPASS_EN_TRUE = 0x00000001u32,
}

/*
 * OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_PIPE_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_PIPE_SELECT {
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_PIPE_SELECT_OTG0 = 0x00000000u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_PIPE_SELECT_OTG1 = 0x00000001u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_PIPE_SELECT_OTG2 = 0x00000002u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_PIPE_SELECT_OTG3 = 0x00000003u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_PIPE_SELECT_RESERVED4 = 0x00000004u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_PIPE_SELECT_RESERVED5 = 0x00000005u32,
}

/*
 * OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT {
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_LOGIC0 = 0x00000000u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GENERICA_PIN = 0x00000001u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GENERICB_PIN = 0x00000002u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GENERICC_PIN = 0x00000003u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GENERICD_PIN = 0x00000004u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GENERICE_PIN = 0x00000005u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GENERICF_PIN = 0x00000006u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_SWAPLOCKA_PIN = 0x00000007u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_SWAPLOCKB_PIN = 0x00000008u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GENLK_CLK_PIN = 0x00000009u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GENLK_VSYNC_PIN = 0x0000000au32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_HPD1 = 0x0000000bu32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_HPD2 = 0x0000000cu32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_BLON_Y_PIN = 0x0000000du32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_RESERVED14 = 0x0000000eu32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_UPDATE_LOCK = 0x0000000fu32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GSL_ALLOW_FLIP = 0x00000010u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_UPDATE_PENDING = 0x00000011u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_OTG_SOF = 0x00000012u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_HSYNC = 0x00000013u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_VSYNC = 0x00000014u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_OTG_TRIG_MANUAL_CONTROL = 0x00000015u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_MANUAL_FLOW_CONTROL = 0x00000016u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_LOGIC1 = 0x00000017u32,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_FLIP_PENDING = 0x00000018u32,
}

/*
 * OTG_TRIGB_FALLING_EDGE_DETECT_CNTL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_TRIGB_FALLING_EDGE_DETECT_CNTL {
OTG_TRIGB_FALLING_EDGE_DETECT_CNTL_0     = 0x00000000u32,
OTG_TRIGB_FALLING_EDGE_DETECT_CNTL_1     = 0x00000001u32,
OTG_TRIGB_FALLING_EDGE_DETECT_CNTL_2     = 0x00000002u32,
OTG_TRIGB_FALLING_EDGE_DETECT_CNTL_3     = 0x00000003u32,
}

/*
 * OTG_TRIGB_FREQUENCY_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_TRIGB_FREQUENCY_SELECT {
OTG_TRIGB_FREQUENCY_SELECT_0             = 0x00000000u32,
OTG_TRIGB_FREQUENCY_SELECT_1             = 0x00000001u32,
OTG_TRIGB_FREQUENCY_SELECT_2             = 0x00000002u32,
OTG_TRIGB_FREQUENCY_SELECT_3             = 0x00000003u32,
}

/*
 * OTG_TRIGB_RISING_EDGE_DETECT_CNTL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_TRIGB_RISING_EDGE_DETECT_CNTL {
OTG_TRIGB_RISING_EDGE_DETECT_CNTL_0      = 0x00000000u32,
OTG_TRIGB_RISING_EDGE_DETECT_CNTL_1      = 0x00000001u32,
OTG_TRIGB_RISING_EDGE_DETECT_CNTL_2      = 0x00000002u32,
OTG_TRIGB_RISING_EDGE_DETECT_CNTL_3      = 0x00000003u32,
}

/*
 * OTG_UPDATE_LOCK_OTG_UPDATE_LOCK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_UPDATE_LOCK_OTG_UPDATE_LOCK {
OTG_UPDATE_LOCK_OTG_UPDATE_LOCK_FALSE    = 0x00000000u32,
OTG_UPDATE_LOCK_OTG_UPDATE_LOCK_TRUE     = 0x00000001u32,
}

/*
 * OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_CLEAR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_CLEAR {
OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_CLEAR_FALSE = 0x00000000u32,
OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_CLEAR_TRUE = 0x00000001u32,
}

/*
 * OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_INT_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_INT_ENABLE {
OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_INT_ENABLE_FALSE = 0x00000000u32,
OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_INT_ENABLE_TRUE = 0x00000001u32,
}

/*
 * OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_INT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_INT_TYPE {
OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_INT_TYPE_FALSE = 0x00000000u32,
OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_INT_TYPE_TRUE = 0x00000001u32,
}

/*
 * OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_OUTPUT_POLARITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_OUTPUT_POLARITY {
OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_OUTPUT_POLARITY_FALSE = 0x00000000u32,
OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_OUTPUT_POLARITY_TRUE = 0x00000001u32,
}

/*
 * OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_CLEAR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_CLEAR {
OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_CLEAR_CLEAR_FALSE = 0x00000000u32,
OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_CLEAR_TRUE = 0x00000001u32,
}

/*
 * OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_INT_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_INT_ENABLE {
OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_INT_ENABLE_FALSE = 0x00000000u32,
OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_INT_ENABLE_TRUE = 0x00000001u32,
}

/*
 * OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_INT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_INT_TYPE {
OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_INT_TYPE_FALSE = 0x00000000u32,
OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_INT_TYPE_TRUE = 0x00000001u32,
}

/*
 * OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_CLEAR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_CLEAR {
OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_CLEAR_CLEAR_FALSE = 0x00000000u32,
OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_CLEAR_TRUE = 0x00000001u32,
}

/*
 * OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_INT_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_INT_ENABLE {
OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_INT_ENABLE_FALSE = 0x00000000u32,
OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_INT_ENABLE_TRUE = 0x00000001u32,
}

/*
 * OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_INT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_INT_TYPE {
OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_INT_TYPE_FALSE = 0x00000000u32,
OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_INT_TYPE_TRUE = 0x00000001u32,
}

/*
 * OTG_VERT_SYNC_CONTROL_OTG_AUTO_FORCE_VSYNC_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_VERT_SYNC_CONTROL_OTG_AUTO_FORCE_VSYNC_MODE {
OTG_VERT_SYNC_CONTROL_OTG_AUTO_FORCE_VSYNC_MODE_DISABLE = 0x00000000u32,
OTG_VERT_SYNC_CONTROL_OTG_AUTO_FORCE_VSYNC_MODE_TRIGGERA = 0x00000001u32,
OTG_VERT_SYNC_CONTROL_OTG_AUTO_FORCE_VSYNC_MODE_TRIGGERB = 0x00000002u32,
OTG_VERT_SYNC_CONTROL_OTG_AUTO_FORCE_VSYNC_MODE_RESERVED = 0x00000003u32,
}

/*
 * OTG_VERT_SYNC_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_CLEAR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_VERT_SYNC_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_CLEAR {
OTG_VERT_SYNC_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_CLEAR_FALSE = 0x00000000u32,
OTG_VERT_SYNC_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_CLEAR_TRUE = 0x00000001u32,
}

/*
 * OTG_VSYNC_NOM_INT_STATUS_OTG_VSYNC_NOM_INT_CLEAR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_VSYNC_NOM_INT_STATUS_OTG_VSYNC_NOM_INT_CLEAR {
OTG_VSYNC_NOM_INT_STATUS_OTG_VSYNC_NOM_INT_CLEAR_FALSE = 0x00000000u32,
OTG_VSYNC_NOM_INT_STATUS_OTG_VSYNC_NOM_INT_CLEAR_TRUE = 0x00000001u32,
}

/*
 * OTG_VUPDATE_BLOCK_DISABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_VUPDATE_BLOCK_DISABLE {
OTG_VUPDATE_BLOCK_DISABLE_OFF            = 0x00000000u32,
OTG_VUPDATE_BLOCK_DISABLE_ON             = 0x00000001u32,
}

/*
 * OTG_V_SYNC_A_POL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_V_SYNC_A_POL {
OTG_V_SYNC_A_POL_HIGH                    = 0x00000000u32,
OTG_V_SYNC_A_POL_LOW                     = 0x00000001u32,
}

/*
 * OTG_V_SYNC_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_V_SYNC_MODE {
OTG_V_SYNC_MODE_HSYNC                    = 0x00000000u32,
OTG_V_SYNC_MODE_HBLANK                   = 0x00000001u32,
}

/*
 * OTG_V_TOTAL_CONTROL_OTG_DRR_EVENT_ACTIVE_PERIOD enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_V_TOTAL_CONTROL_OTG_DRR_EVENT_ACTIVE_PERIOD {
OTG_V_TOTAL_CONTROL_OTG_DRR_EVENT_ACTIVE_PERIOD_0 = 0x00000000u32,
OTG_V_TOTAL_CONTROL_OTG_DRR_EVENT_ACTIVE_PERIOD_1 = 0x00000001u32,
}

/*
 * OTG_V_TOTAL_CONTROL_OTG_FORCE_LOCK_ON_EVENT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_V_TOTAL_CONTROL_OTG_FORCE_LOCK_ON_EVENT {
OTG_V_TOTAL_CONTROL_OTG_FORCE_LOCK_ON_EVENT_DISABLE = 0x00000000u32,
OTG_V_TOTAL_CONTROL_OTG_FORCE_LOCK_ON_EVENT_ENABLE = 0x00000001u32,
}

/*
 * OTG_V_TOTAL_CONTROL_OTG_FORCE_LOCK_TO_MASTER_VSYNC enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_V_TOTAL_CONTROL_OTG_FORCE_LOCK_TO_MASTER_VSYNC {
OTG_V_TOTAL_CONTROL_OTG_FORCE_LOCK_TO_MASTER_VSYNC_DISABLE = 0x00000000u32,
OTG_V_TOTAL_CONTROL_OTG_FORCE_LOCK_TO_MASTER_VSYNC_ENABLE = 0x00000001u32,
}

/*
 * OTG_V_TOTAL_CONTROL_OTG_V_TOTAL_MAX_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_V_TOTAL_CONTROL_OTG_V_TOTAL_MAX_SEL {
OTG_V_TOTAL_CONTROL_OTG_V_TOTAL_MAX_SEL_FALSE = 0x00000000u32,
OTG_V_TOTAL_CONTROL_OTG_V_TOTAL_MAX_SEL_TRUE = 0x00000001u32,
}

/*
 * OTG_V_TOTAL_CONTROL_OTG_V_TOTAL_MIN_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_V_TOTAL_CONTROL_OTG_V_TOTAL_MIN_SEL {
OTG_V_TOTAL_CONTROL_OTG_V_TOTAL_MIN_SEL_FALSE = 0x00000000u32,
OTG_V_TOTAL_CONTROL_OTG_V_TOTAL_MIN_SEL_TRUE = 0x00000001u32,
}

/*
 * OTG_V_TOTAL_INT_STATUS_OTG_SET_V_TOTAL_MIN_EVENT_OCCURRED_ACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_V_TOTAL_INT_STATUS_OTG_SET_V_TOTAL_MIN_EVENT_OCCURRED_ACK {
OTG_V_TOTAL_INT_STATUS_OTG_SET_V_TOTAL_MIN_EVENT_OCCURRED_ACK_FALSE = 0x00000000u32,
OTG_V_TOTAL_INT_STATUS_OTG_SET_V_TOTAL_MIN_EVENT_OCCURRED_ACK_TRUE = 0x00000001u32,
}

/*******************************************************
 * OPTC_MISC Enums
 *******************************************************/

/*
 * OPTC_GSL_SOURCE_SELECT_GSL_TIMING_SYNC_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OPTC_GSL_SOURCE_SELECT_GSL_TIMING_SYNC_SEL {
OPTC_GSL_SOURCE_SELECT_GSL_TIMING_SYNC_SEL_OTG0 = 0x00000000u32,
OPTC_GSL_SOURCE_SELECT_GSL_TIMING_SYNC_SEL_OTG1 = 0x00000001u32,
OPTC_GSL_SOURCE_SELECT_GSL_TIMING_SYNC_SEL_OTG2 = 0x00000002u32,
OPTC_GSL_SOURCE_SELECT_GSL_TIMING_SYNC_SEL_OTG3 = 0x00000003u32,
OPTC_GSL_SOURCE_SELECT_GSL_TIMING_SYNC_SEL_RESERVED4 = 0x00000004u32,
OPTC_GSL_SOURCE_SELECT_GSL_TIMING_SYNC_SEL_RESERVED5 = 0x00000005u32,
}

/*******************************************************
 * DMCUB Enums
 *******************************************************/

/*
 * DC_DMCUB_INT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DC_DMCUB_INT_TYPE {
INT_LEVEL                                = 0x00000000u32,
INT_PULSE                                = 0x00000001u32,
}

/*
 * DC_DMCUB_TIMER_WINDOW enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DC_DMCUB_TIMER_WINDOW {
BITS_31_0                                = 0x00000000u32,
BITS_32_1                                = 0x00000001u32,
BITS_33_2                                = 0x00000002u32,
BITS_34_3                                = 0x00000003u32,
BITS_35_4                                = 0x00000004u32,
BITS_36_5                                = 0x00000005u32,
BITS_37_6                                = 0x00000006u32,
BITS_38_7                                = 0x00000007u32,
}

/*******************************************************
 * RBBMIF Enums
 *******************************************************/

/*
 * INVALID_REG_ACCESS_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum INVALID_REG_ACCESS_TYPE {
REG_UNALLOCATED_ADDR_WRITE               = 0x00000000u32,
REG_UNALLOCATED_ADDR_READ                = 0x00000001u32,
REG_VIRTUAL_WRITE                        = 0x00000002u32,
REG_VIRTUAL_READ                         = 0x00000003u32,
REG_SECURE_VIOLATE_WRITE                 = 0x00000004u32,
REG_SECURE_VIOLATE_READ                  = 0x00000005u32,
}

/*******************************************************
 * IHC Enums
 *******************************************************/

/*
 * DMU_DC_GPU_TIMER_READ_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DMU_DC_GPU_TIMER_READ_SELECT {
DMU_GPU_TIMER_READ_SELECT_LOWER_D1_V_UPDATE_0 = 0x00000000u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D1_V_UPDATE_1 = 0x00000001u32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D2_V_UPDATE_2 = 0x00000002u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D2_V_UPDATE_3 = 0x00000003u32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D3_V_UPDATE_4 = 0x00000004u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D3_V_UPDATE_5 = 0x00000005u32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D4_V_UPDATE_6 = 0x00000006u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D4_V_UPDATE_7 = 0x00000007u32,
RESERVED_8                               = 0x00000008u32,
RESERVED_9                               = 0x00000009u32,
RESERVED_10                              = 0x0000000au32,
RESERVED_11                              = 0x0000000bu32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D1_V_STARTUP_12 = 0x0000000cu32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D1_V_STARTUP_13 = 0x0000000du32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D2_V_STARTUP_14 = 0x0000000eu32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D2_V_STARTUP_15 = 0x0000000fu32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D3_V_STARTUP_16 = 0x00000010u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D3_V_STARTUP_17 = 0x00000011u32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D4_V_STARTUP_18 = 0x00000012u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D4_V_STARTUP_19 = 0x00000013u32,
RESERVED_20                              = 0x00000014u32,
RESERVED_21                              = 0x00000015u32,
RESERVED_22                              = 0x00000016u32,
RESERVED_23                              = 0x00000017u32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D1_VSYNC_NOM_24 = 0x00000018u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D1_VSYNC_NOM_25 = 0x00000019u32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D2_VSYNC_NOM_26 = 0x0000001au32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D2_VSYNC_NOM_27 = 0x0000001bu32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D3_VSYNC_NOM_28 = 0x0000001cu32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D3_VSYNC_NOM_29 = 0x0000001du32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D4_VSYNC_NOM_30 = 0x0000001eu32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D4_VSYNC_NOM_31 = 0x0000001fu32,
RESERVED_32                              = 0x00000020u32,
RESERVED_33                              = 0x00000021u32,
RESERVED_34                              = 0x00000022u32,
RESERVED_35                              = 0x00000023u32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D1_VREADY_36 = 0x00000024u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D1_VREADY_37 = 0x00000025u32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D2_VREADY_38 = 0x00000026u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D2_VREADY_39 = 0x00000027u32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D3_VREADY_40 = 0x00000028u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D3_VREADY_41 = 0x00000029u32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D4_VREADY_42 = 0x0000002au32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D4_VREADY_43 = 0x0000002bu32,
RESERVED_44                              = 0x0000002cu32,
RESERVED_45                              = 0x0000002du32,
RESERVED_46                              = 0x0000002eu32,
RESERVED_47                              = 0x0000002fu32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D1_FLIP_48 = 0x00000030u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D1_FLIP_49 = 0x00000031u32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D2_FLIP_50 = 0x00000032u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D2_FLIP_51 = 0x00000033u32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D3_FLIP_52 = 0x00000034u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D3_FLIP_53 = 0x00000035u32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D4_FLIP_54 = 0x00000036u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D4_FLIP_55 = 0x00000037u32,
RESERVED_56                              = 0x00000038u32,
RESERVED_57                              = 0x00000039u32,
RESERVED_58                              = 0x0000003au32,
RESERVED_59                              = 0x0000003bu32,
RESERVED_60                              = 0x0000003cu32,
RESERVED_61                              = 0x0000003du32,
RESERVED_62                              = 0x0000003eu32,
RESERVED_63                              = 0x0000003fu32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D1_V_UPDATE_NO_LOCK_64 = 0x00000040u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D1_V_UPDATE_NO_LOCK_65 = 0x00000041u32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D2_V_UPDATE_NO_LOCK_66 = 0x00000042u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D2_V_UPDATE_NO_LOCK_67 = 0x00000043u32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D3_V_UPDATE_NO_LOCK_68 = 0x00000044u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D3_V_UPDATE_NO_LOCK_69 = 0x00000045u32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D4_V_UPDATE_NO_LOCK_70 = 0x00000046u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D4_V_UPDATE_NO_LOCK_71 = 0x00000047u32,
RESERVED_72                              = 0x00000048u32,
RESERVED_73                              = 0x00000049u32,
RESERVED_74                              = 0x0000004au32,
RESERVED_75                              = 0x0000004bu32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D1_FLIP_AWAY_76 = 0x0000004cu32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D1_FLIP_AWAY_77 = 0x0000004du32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D2_FLIP_AWAY_78 = 0x0000004eu32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D2_FLIP_AWAY_79 = 0x0000004fu32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D3_FLIP_AWAY_80 = 0x00000050u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D3_FLIP_AWAY_81 = 0x00000051u32,
DMU_GPU_TIMER_READ_SELECT_LOWER_D4_FLIP_AWAY_82 = 0x00000052u32,
DMU_GPU_TIMER_READ_SELECT_UPPER_D4_FLIP_AWAY_83 = 0x00000053u32,
RESERVED_84                              = 0x00000054u32,
RESERVED_85                              = 0x00000055u32,
RESERVED_86                              = 0x00000056u32,
RESERVED_87                              = 0x00000057u32,
RESERVED_88                              = 0x00000058u32,
RESERVED_89                              = 0x00000059u32,
RESERVED_90                              = 0x0000005au32,
RESERVED_91                              = 0x0000005bu32,
}

/*
 * DMU_DC_GPU_TIMER_START_POSITION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DMU_DC_GPU_TIMER_START_POSITION {
DMU_GPU_TIMER_START_0_END_27             = 0x00000000u32,
DMU_GPU_TIMER_START_1_END_28             = 0x00000001u32,
DMU_GPU_TIMER_START_2_END_29             = 0x00000002u32,
DMU_GPU_TIMER_START_3_END_30             = 0x00000003u32,
DMU_GPU_TIMER_START_4_END_31             = 0x00000004u32,
DMU_GPU_TIMER_START_6_END_33             = 0x00000005u32,
DMU_GPU_TIMER_START_8_END_35             = 0x00000006u32,
DMU_GPU_TIMER_START_10_END_37            = 0x00000007u32,
}

/*
 * IHC_INTERRUPT_DEST enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum IHC_INTERRUPT_DEST {
INTERRUPT_SENT_TO_IH                     = 0x00000000u32,
INTERRUPT_SENT_TO_DMCUB                  = 0x00000001u32,
}

/*
 * IHC_INTERRUPT_LINE_STATUS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum IHC_INTERRUPT_LINE_STATUS {
INTERRUPT_LINE_NOT_ASSERTED              = 0x00000000u32,
INTERRUPT_LINE_ASSERTED                  = 0x00000001u32,
}

/*******************************************************
 * DMU_MISC Enums
 *******************************************************/

/*
 * DC_SMU_INTERRUPT_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DC_SMU_INTERRUPT_ENABLE {
DISABLE_THE_INTERRUPT                    = 0x00000000u32,
ENABLE_THE_INTERRUPT                     = 0x00000001u32,
}

/*
 * DMU_CLOCK_ON enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DMU_CLOCK_ON {
DMU_CLOCK_STATUS_ON                      = 0x00000000u32,
DMU_CLOCK_STATUS_OFF                     = 0x00000001u32,
}

/*
 * SMU_INTR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SMU_INTR {
SMU_MSG_INTR_NOOP                        = 0x00000000u32,
SET_SMU_MSG_INTR                         = 0x00000001u32,
}

/*******************************************************
 * DCCG Enums
 *******************************************************/

/*
 * ALLOW_SR_ON_TRANS_REQ enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ALLOW_SR_ON_TRANS_REQ {
ALLOW_SR_ON_TRANS_REQ_ENABLE             = 0x00000000u32,
ALLOW_SR_ON_TRANS_REQ_DISABLE            = 0x00000001u32,
}

/*
 * AMCLOCK_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AMCLOCK_ENABLE {
ENABLE_AMCLK0                            = 0x00000000u32,
ENABLE_AMCLK1                            = 0x00000001u32,
}

/*
 * CLEAR_SMU_INTR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CLEAR_SMU_INTR {
SMU_INTR_STATUS_NOOP                     = 0x00000000u32,
SMU_INTR_STATUS_CLEAR                    = 0x00000001u32,
}

/*
 * CLOCK_BRANCH_SOFT_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CLOCK_BRANCH_SOFT_RESET {
CLOCK_BRANCH_SOFT_RESET_NOOP             = 0x00000000u32,
CLOCK_BRANCH_SOFT_RESET_FORCE            = 0x00000001u32,
}

/*
 * DCCG_AUDIO_DTO0_SOURCE_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCCG_AUDIO_DTO0_SOURCE_SEL {
DCCG_AUDIO_DTO0_SOURCE_SEL_OTG0          = 0x00000000u32,
DCCG_AUDIO_DTO0_SOURCE_SEL_OTG1          = 0x00000001u32,
DCCG_AUDIO_DTO0_SOURCE_SEL_OTG2          = 0x00000002u32,
DCCG_AUDIO_DTO0_SOURCE_SEL_OTG3          = 0x00000003u32,
DCCG_AUDIO_DTO0_SOURCE_SEL_RESERVED      = 0x00000004u32,
}

/*
 * DCCG_AUDIO_DTO2_SOURCE_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCCG_AUDIO_DTO2_SOURCE_SEL {
DCCG_AUDIO_DTO2_SOURCE_SEL_AMCLK0        = 0x00000000u32,
DCCG_AUDIO_DTO2_SOURCE_SEL_AMCLK0_DIV2   = 0x00000001u32,
}

/*
 * DCCG_AUDIO_DTO_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCCG_AUDIO_DTO_SEL {
DCCG_AUDIO_DTO_SEL_AUDIO_DTO0            = 0x00000000u32,
DCCG_AUDIO_DTO_SEL_AUDIO_DTO1            = 0x00000001u32,
DCCG_AUDIO_DTO_SEL_NO_AUDIO_DTO          = 0x00000002u32,
}

/*
 * DCCG_AUDIO_DTO_USE_512FBR_DTO enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCCG_AUDIO_DTO_USE_512FBR_DTO {
DCCG_AUDIO_DTO_USE_128FBR_FOR_DP         = 0x00000000u32,
DCCG_AUDIO_DTO_USE_512FBR_FOR_DP         = 0x00000001u32,
}

/*
 * DCCG_DBG_BLOCK_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCCG_DBG_BLOCK_SEL {
DCCG_DBG_BLOCK_SEL_DCCG                  = 0x00000000u32,
DCCG_DBG_BLOCK_SEL_PMON                  = 0x00000001u32,
DCCG_DBG_BLOCK_SEL_PMON2                 = 0x00000002u32,
}

/*
 * DCCG_DBG_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCCG_DBG_EN {
DCCG_DBG_EN_DISABLE                      = 0x00000000u32,
DCCG_DBG_EN_ENABLE                       = 0x00000001u32,
}

/*
 * DCCG_DEEP_COLOR_CNTL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCCG_DEEP_COLOR_CNTL {
DCCG_DEEP_COLOR_DTO_DISABLE              = 0x00000000u32,
DCCG_DEEP_COLOR_DTO_5_4_RATIO            = 0x00000001u32,
DCCG_DEEP_COLOR_DTO_3_2_RATIO            = 0x00000002u32,
DCCG_DEEP_COLOR_DTO_2_1_RATIO            = 0x00000003u32,
}

/*
 * DCCG_FIFO_ERRDET_OVR_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCCG_FIFO_ERRDET_OVR_EN {
DCCG_FIFO_ERRDET_OVR_DISABLE             = 0x00000000u32,
DCCG_FIFO_ERRDET_OVR_ENABLE              = 0x00000001u32,
}

/*
 * DCCG_FIFO_ERRDET_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCCG_FIFO_ERRDET_RESET {
DCCG_FIFO_ERRDET_RESET_NOOP              = 0x00000000u32,
DCCG_FIFO_ERRDET_RESET_FORCE             = 0x00000001u32,
}

/*
 * DCCG_FIFO_ERRDET_STATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCCG_FIFO_ERRDET_STATE {
DCCG_FIFO_ERRDET_STATE_CALIBRATION       = 0x00000000u32,
DCCG_FIFO_ERRDET_STATE_DETECTION         = 0x00000001u32,
}

/*
 * DCCG_PERF_MODE_HSYNC enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCCG_PERF_MODE_HSYNC {
DCCG_PERF_MODE_HSYNC_NOOP                = 0x00000000u32,
DCCG_PERF_MODE_HSYNC_START               = 0x00000001u32,
}

/*
 * DCCG_PERF_MODE_VSYNC enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCCG_PERF_MODE_VSYNC {
DCCG_PERF_MODE_VSYNC_NOOP                = 0x00000000u32,
DCCG_PERF_MODE_VSYNC_START               = 0x00000001u32,
}

/*
 * DCCG_PERF_OTG_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCCG_PERF_OTG_SELECT {
DCCG_PERF_SEL_OTG0                       = 0x00000000u32,
DCCG_PERF_SEL_OTG1                       = 0x00000001u32,
DCCG_PERF_SEL_OTG2                       = 0x00000002u32,
DCCG_PERF_SEL_OTG3                       = 0x00000003u32,
DCCG_PERF_SEL_RESERVED                   = 0x00000004u32,
}

/*
 * DCCG_PERF_RUN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCCG_PERF_RUN {
DCCG_PERF_RUN_NOOP                       = 0x00000000u32,
DCCG_PERF_RUN_START                      = 0x00000001u32,
}

/*
 * DC_MEM_GLOBAL_PWR_REQ_DIS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DC_MEM_GLOBAL_PWR_REQ_DIS {
DC_MEM_GLOBAL_PWR_REQ_ENABLE             = 0x00000000u32,
DC_MEM_GLOBAL_PWR_REQ_DISABLE            = 0x00000001u32,
}

/*
 * DIO_FIFO_ERROR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIO_FIFO_ERROR {
DIO_FIFO_ERROR_00                        = 0x00000000u32,
DIO_FIFO_ERROR_01                        = 0x00000001u32,
DIO_FIFO_ERROR_10                        = 0x00000002u32,
DIO_FIFO_ERROR_11                        = 0x00000003u32,
}

/*
 * DISABLE_CLOCK_GATING enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DISABLE_CLOCK_GATING {
CLOCK_GATING_ENABLED                     = 0x00000000u32,
CLOCK_GATING_DISABLED                    = 0x00000001u32,
}

/*
 * DISABLE_CLOCK_GATING_IN_DCO enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DISABLE_CLOCK_GATING_IN_DCO {
CLOCK_GATING_ENABLED_IN_DCO              = 0x00000000u32,
CLOCK_GATING_DISABLED_IN_DCO             = 0x00000001u32,
}

/*
 * DISPCLK_CHG_FWD_CORR_DISABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DISPCLK_CHG_FWD_CORR_DISABLE {
DISPCLK_CHG_FWD_CORR_ENABLE_AT_BEGINNING = 0x00000000u32,
DISPCLK_CHG_FWD_CORR_DISABLE_AT_BEGINNING = 0x00000001u32,
}

/*
 * DISPCLK_FREQ_RAMP_DONE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DISPCLK_FREQ_RAMP_DONE {
DISPCLK_FREQ_RAMP_IN_PROGRESS            = 0x00000000u32,
DISPCLK_FREQ_RAMP_COMPLETED              = 0x00000001u32,
}

/*
 * DPREFCLK_SRC_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPREFCLK_SRC_SEL {
DPREFCLK_SRC_SEL_CK                      = 0x00000000u32,
DPREFCLK_SRC_SEL_P0PLL                   = 0x00000001u32,
DPREFCLK_SRC_SEL_P1PLL                   = 0x00000002u32,
DPREFCLK_SRC_SEL_P2PLL                   = 0x00000003u32,
}

/*
 * DP_DTO_DS_DISABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_DTO_DS_DISABLE {
DP_DTO_DESPREAD_DISABLE                  = 0x00000000u32,
DP_DTO_DESPREAD_ENABLE                   = 0x00000001u32,
}

/*
 * DS_HW_CAL_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DS_HW_CAL_ENABLE {
DS_HW_CAL_DIS                            = 0x00000000u32,
DS_HW_CAL_EN                             = 0x00000001u32,
}

/*
 * DS_REF_SRC enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DS_REF_SRC {
DS_REF_IS_XTALIN                         = 0x00000000u32,
DS_REF_IS_EXT_GENLOCK                    = 0x00000001u32,
DS_REF_IS_PCIE                           = 0x00000002u32,
}

/*
 * DVO_ENABLE_RST enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DVO_ENABLE_RST {
DVO_ENABLE_RST_DISABLE                   = 0x00000000u32,
DVO_ENABLE_RST_ENABLE                    = 0x00000001u32,
}

/*
 * ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENABLE {
DISABLE_THE_FEATURE                      = 0x00000000u32,
ENABLE_THE_FEATURE                       = 0x00000001u32,
}

/*
 * ENABLE_CLOCK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENABLE_CLOCK {
ENABLE_THE_REFCLK                        = 0x00000000u32,
ENABLE_THE_FUNC_CLOCK                    = 0x00000001u32,
}

/*
 * FORCE_DISABLE_CLOCK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FORCE_DISABLE_CLOCK {
NOT_FORCE_THE_CLOCK_DISABLED             = 0x00000000u32,
FORCE_THE_CLOCK_DISABLED                 = 0x00000001u32,
}

/*
 * HDMICHARCLK_SRC_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMICHARCLK_SRC_SEL {
HDMICHARCLK_SRC_SEL_UNIPHYA              = 0x00000000u32,
HDMICHARCLK_SRC_SEL_UNIPHYB              = 0x00000001u32,
HDMICHARCLK_SRC_SEL_UNIPHYC              = 0x00000002u32,
HDMICHARCLK_SRC_SEL_UNIPHYD              = 0x00000003u32,
HDMICHARCLK_SRC_SEL_SRC_RESERVED         = 0x00000004u32,
}

/*
 * HDMISTREAMCLK_SRC_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMISTREAMCLK_SRC_SEL {
SEL_DTBCLK_P0                            = 0x00000000u32,
SEL_DTBCLK_P1                            = 0x00000001u32,
SEL_DTBCLK_P2                            = 0x00000002u32,
SEL_DTBCLK_P3                            = 0x00000003u32,
}

/*
 * JITTER_REMOVE_DISABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum JITTER_REMOVE_DISABLE {
ENABLE_JITTER_REMOVAL                    = 0x00000000u32,
DISABLE_JITTER_REMOVAL                   = 0x00000001u32,
}

/*
 * MICROSECOND_TIME_BASE_CLOCK_SOURCE_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MICROSECOND_TIME_BASE_CLOCK_SOURCE_SEL {
MICROSECOND_TIME_BASE_CLOCK_IS_XTALIN    = 0x00000000u32,
MICROSECOND_TIME_BASE_CLOCK_IS_DCCGREFCLK = 0x00000001u32,
}

/*
 * MILLISECOND_TIME_BASE_CLOCK_SOURCE_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MILLISECOND_TIME_BASE_CLOCK_SOURCE_SEL {
MILLISECOND_TIME_BASE_CLOCK_IS_XTALIN    = 0x00000000u32,
MILLISECOND_TIME_BASE_CLOCK_IS_DCCGREFCLK = 0x00000001u32,
}

/*
 * OTG_ADD_PIXEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_ADD_PIXEL {
OTG_ADD_PIXEL_NOOP                       = 0x00000000u32,
OTG_ADD_PIXEL_FORCE                      = 0x00000001u32,
}

/*
 * OTG_DROP_PIXEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OTG_DROP_PIXEL {
OTG_DROP_PIXEL_NOOP                      = 0x00000000u32,
OTG_DROP_PIXEL_FORCE                     = 0x00000001u32,
}

/*
 * PHYSYMCLK_FORCE_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PHYSYMCLK_FORCE_EN {
PHYSYMCLK_FORCE_EN_DISABLE               = 0x00000000u32,
PHYSYMCLK_FORCE_EN_ENABLE                = 0x00000001u32,
}

/*
 * PHYSYMCLK_FORCE_SRC_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PHYSYMCLK_FORCE_SRC_SEL {
PHYSYMCLK_FORCE_SRC_SYMCLK               = 0x00000000u32,
PHYSYMCLK_FORCE_SRC_PHYD18CLK            = 0x00000001u32,
PHYSYMCLK_FORCE_SRC_PHYD32CLK            = 0x00000002u32,
}

/*
 * PIPE_PHYPLL_PIXEL_RATE_SOURCE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PIPE_PHYPLL_PIXEL_RATE_SOURCE {
PIPE_PHYPLL_PIXEL_RATE_SOURCE_UNIPHYA    = 0x00000000u32,
PIPE_PHYPLL_PIXEL_RATE_SOURCE_UNIPHYB    = 0x00000001u32,
PIPE_PHYPLL_PIXEL_RATE_SOURCE_UNIPHYC    = 0x00000002u32,
PIPE_PHYPLL_PIXEL_RATE_SOURCE_UNIPHYD    = 0x00000003u32,
PIPE_PHYPLL_PIXEL_RATE_SOURCE_RESERVED   = 0x00000004u32,
}

/*
 * PIPE_PIXEL_RATE_PLL_SOURCE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PIPE_PIXEL_RATE_PLL_SOURCE {
PIPE_PIXEL_RATE_PLL_SOURCE_PHYPLL        = 0x00000000u32,
PIPE_PIXEL_RATE_PLL_SOURCE_DISPPLL       = 0x00000001u32,
}

/*
 * PIPE_PIXEL_RATE_SOURCE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PIPE_PIXEL_RATE_SOURCE {
PIPE_PIXEL_RATE_SOURCE_P0PLL             = 0x00000000u32,
PIPE_PIXEL_RATE_SOURCE_P1PLL             = 0x00000001u32,
PIPE_PIXEL_RATE_SOURCE_P2PLL             = 0x00000002u32,
}

/*
 * PLL_CFG_IF_SOFT_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PLL_CFG_IF_SOFT_RESET {
PLL_CFG_IF_SOFT_RESET_NOOP               = 0x00000000u32,
PLL_CFG_IF_SOFT_RESET_FORCE              = 0x00000001u32,
}

/*
 * SYMCLK_FE_SRC enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SYMCLK_FE_SRC {
SYMCLK_FE_SRC_UNIPHYA                    = 0x00000000u32,
SYMCLK_FE_SRC_UNIPHYB                    = 0x00000001u32,
SYMCLK_FE_SRC_UNIPHYC                    = 0x00000002u32,
SYMCLK_FE_SRC_UNIPHYD                    = 0x00000003u32,
SYMCLK_FE_SRC_RESERVED                   = 0x00000004u32,
}

/*
 * TEST_CLK_DIV_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TEST_CLK_DIV_SEL {
NO_DIV                                   = 0x00000000u32,
DIV_2                                    = 0x00000001u32,
DIV_4                                    = 0x00000002u32,
DIV_8                                    = 0x00000003u32,
}

/*
 * VSYNC_CNT_LATCH_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VSYNC_CNT_LATCH_MASK {
VSYNC_CNT_LATCH_MASK_0                   = 0x00000000u32,
VSYNC_CNT_LATCH_MASK_1                   = 0x00000001u32,
}

/*
 * VSYNC_CNT_RESET_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VSYNC_CNT_RESET_SEL {
VSYNC_CNT_RESET_SEL_0                    = 0x00000000u32,
VSYNC_CNT_RESET_SEL_1                    = 0x00000001u32,
}

/*
 * XTAL_REF_CLOCK_SOURCE_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum XTAL_REF_CLOCK_SOURCE_SEL {
XTAL_REF_CLOCK_SOURCE_SEL_XTALIN         = 0x00000000u32,
XTAL_REF_CLOCK_SOURCE_SEL_DCCGREFCLK     = 0x00000001u32,
}

/*
 * XTAL_REF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum XTAL_REF_SEL {
XTAL_REF_SEL_1X                          = 0x00000000u32,
XTAL_REF_SEL_2X                          = 0x00000001u32,
}

/*******************************************************
 * DP Enums
 *******************************************************/

/*
 * DPHY_8B10B_CUR_DISP enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_8B10B_CUR_DISP {
DPHY_8B10B_CUR_DISP_ZERO                 = 0x00000000u32,
DPHY_8B10B_CUR_DISP_ONE                  = 0x00000001u32,
}

/*
 * DPHY_8B10B_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_8B10B_RESET {
DPHY_8B10B_NOT_RESET                     = 0x00000000u32,
DPHY_8B10B_RESETET                       = 0x00000001u32,
}

/*
 * DPHY_ATEST_SEL_LANE0 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_ATEST_SEL_LANE0 {
DPHY_ATEST_LANE0_PRBS_PATTERN            = 0x00000000u32,
DPHY_ATEST_LANE0_REG_PATTERN             = 0x00000001u32,
}

/*
 * DPHY_ATEST_SEL_LANE1 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_ATEST_SEL_LANE1 {
DPHY_ATEST_LANE1_PRBS_PATTERN            = 0x00000000u32,
DPHY_ATEST_LANE1_REG_PATTERN             = 0x00000001u32,
}

/*
 * DPHY_ATEST_SEL_LANE2 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_ATEST_SEL_LANE2 {
DPHY_ATEST_LANE2_PRBS_PATTERN            = 0x00000000u32,
DPHY_ATEST_LANE2_REG_PATTERN             = 0x00000001u32,
}

/*
 * DPHY_ATEST_SEL_LANE3 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_ATEST_SEL_LANE3 {
DPHY_ATEST_LANE3_PRBS_PATTERN            = 0x00000000u32,
DPHY_ATEST_LANE3_REG_PATTERN             = 0x00000001u32,
}

/*
 * DPHY_BYPASS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_BYPASS {
DPHY_8B10B_OUTPUT                        = 0x00000000u32,
DPHY_DBG_OUTPUT                          = 0x00000001u32,
}

/*
 * DPHY_CRC_CONT_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_CRC_CONT_EN {
DPHY_CRC_ONE_SHOT                        = 0x00000000u32,
DPHY_CRC_CONTINUOUS                      = 0x00000001u32,
}

/*
 * DPHY_CRC_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_CRC_EN {
DPHY_CRC_DISABLED                        = 0x00000000u32,
DPHY_CRC_ENABLED                         = 0x00000001u32,
}

/*
 * DPHY_CRC_FIELD enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_CRC_FIELD {
DPHY_CRC_START_FROM_TOP_FIELD            = 0x00000000u32,
DPHY_CRC_START_FROM_BOTTOM_FIELD         = 0x00000001u32,
}

/*
 * DPHY_CRC_MST_PHASE_ERROR_ACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_CRC_MST_PHASE_ERROR_ACK {
DPHY_CRC_MST_PHASE_ERROR_NO_ACK          = 0x00000000u32,
DPHY_CRC_MST_PHASE_ERROR_ACKED           = 0x00000001u32,
}

/*
 * DPHY_CRC_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_CRC_SEL {
DPHY_CRC_LANE0_SELECTED                  = 0x00000000u32,
DPHY_CRC_LANE1_SELECTED                  = 0x00000001u32,
DPHY_CRC_LANE2_SELECTED                  = 0x00000002u32,
DPHY_CRC_LANE3_SELECTED                  = 0x00000003u32,
}

/*
 * DPHY_FEC_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_FEC_ENABLE {
DPHY_FEC_DISABLED                        = 0x00000000u32,
DPHY_FEC_ENABLED                         = 0x00000001u32,
}

/*
 * DPHY_FEC_READY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_FEC_READY {
DPHY_FEC_READY_EN                        = 0x00000000u32,
DPHY_FEC_READY_DIS                       = 0x00000001u32,
}

/*
 * DPHY_LOAD_BS_COUNT_START enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_LOAD_BS_COUNT_START {
DPHY_LOAD_BS_COUNT_STARTED               = 0x00000000u32,
DPHY_LOAD_BS_COUNT_NOT_STARTED           = 0x00000001u32,
}

/*
 * DPHY_PRBS_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_PRBS_EN {
DPHY_PRBS_DISABLE                        = 0x00000000u32,
DPHY_PRBS_ENABLE                         = 0x00000001u32,
}

/*
 * DPHY_PRBS_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_PRBS_SEL {
DPHY_PRBS7_SELECTED                      = 0x00000000u32,
DPHY_PRBS23_SELECTED                     = 0x00000001u32,
DPHY_PRBS11_SELECTED                     = 0x00000002u32,
}

/*
 * DPHY_RX_FAST_TRAINING_CAPABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_RX_FAST_TRAINING_CAPABLE {
DPHY_FAST_TRAINING_NOT_CAPABLE_0         = 0x00000000u32,
DPHY_FAST_TRAINING_CAPABLE               = 0x00000001u32,
}

/*
 * DPHY_SKEW_BYPASS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_SKEW_BYPASS {
DPHY_WITH_SKEW                           = 0x00000000u32,
DPHY_NO_SKEW                             = 0x00000001u32,
}

/*
 * DPHY_STREAM_RESET_DURING_FAST_TRAINING_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_STREAM_RESET_DURING_FAST_TRAINING_ENUM {
DPHY_STREAM_RESET_DURING_FAST_TRAINING_RESET = 0x00000000u32,
DPHY_STREAM_RESET_DURING_FAST_TRAINING_NOT_RESET = 0x00000001u32,
}

/*
 * DPHY_SW_FAST_TRAINING_START enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_SW_FAST_TRAINING_START {
DPHY_SW_FAST_TRAINING_NOT_STARTED        = 0x00000000u32,
DPHY_SW_FAST_TRAINING_STARTED            = 0x00000001u32,
}

/*
 * DPHY_TRAINING_PATTERN_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DPHY_TRAINING_PATTERN_SEL {
DPHY_TRAINING_PATTERN_1                  = 0x00000000u32,
DPHY_TRAINING_PATTERN_2                  = 0x00000001u32,
DPHY_TRAINING_PATTERN_3                  = 0x00000002u32,
DPHY_TRAINING_PATTERN_4                  = 0x00000003u32,
}

/*
 * DP_COMPONENT_DEPTH enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_COMPONENT_DEPTH {
DP_COMPONENT_DEPTH_6BPC                  = 0x00000000u32,
DP_COMPONENT_DEPTH_8BPC                  = 0x00000001u32,
DP_COMPONENT_DEPTH_10BPC                 = 0x00000002u32,
DP_COMPONENT_DEPTH_12BPC                 = 0x00000003u32,
DP_COMPONENT_DEPTH_16BPC                 = 0x00000004u32,
}

/*
 * DP_COMPRESSED_PIXEL_FORMAT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_COMPRESSED_PIXEL_FORMAT {
DP_DSC_444_S422                          = 0x00000000u32,
DP_DSC_N422_N420                         = 0x00000001u32,
}

/*
 * DP_DPHY_8B10B_EXT_DISP enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_DPHY_8B10B_EXT_DISP {
DP_DPHY_8B10B_EXT_DISP_ZERO              = 0x00000000u32,
DP_DPHY_8B10B_EXT_DISP_ONE               = 0x00000001u32,
}

/*
 * DP_DPHY_FAST_TRAINING_COMPLETE_ACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_DPHY_FAST_TRAINING_COMPLETE_ACK {
DP_DPHY_FAST_TRAINING_COMPLETE_NOT_ACKED = 0x00000000u32,
DP_DPHY_FAST_TRAINING_COMPLETE_ACKED     = 0x00000001u32,
}

/*
 * DP_DPHY_FAST_TRAINING_COMPLETE_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_DPHY_FAST_TRAINING_COMPLETE_MASK {
DP_DPHY_FAST_TRAINING_COMPLETE_MASKED    = 0x00000000u32,
DP_DPHY_FAST_TRAINING_COMPLETE_NOT_MASKED = 0x00000001u32,
}

/*
 * DP_DPHY_FAST_TRAINING_VBLANK_EDGE_DETECT_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_DPHY_FAST_TRAINING_VBLANK_EDGE_DETECT_EN {
DP_DPHY_FAST_TRAINING_VBLANK_EDGE_DETECT_DISABLED = 0x00000000u32,
DP_DPHY_FAST_TRAINING_VBLANK_EDGE_DETECT_ENABLED = 0x00000001u32,
}

/*
 * DP_DPHY_HBR2_PATTERN_CONTROL_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_DPHY_HBR2_PATTERN_CONTROL_MODE {
DP_DPHY_HBR2_PASS_THROUGH                = 0x00000000u32,
DP_DPHY_HBR2_PATTERN_1                   = 0x00000001u32,
DP_DPHY_HBR2_PATTERN_2_NEG               = 0x00000002u32,
DP_DPHY_HBR2_PATTERN_3                   = 0x00000003u32,
DP_DPHY_HBR2_PATTERN_2_POS               = 0x00000006u32,
}

/*
 * DP_LINK_TRAINING_COMPLETE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_LINK_TRAINING_COMPLETE {
DP_LINK_TRAINING_NOT_COMPLETE            = 0x00000000u32,
DP_LINK_TRAINING_ALREADY_COMPLETE        = 0x00000001u32,
}

/*
 * DP_LINK_TRAINING_SWITCH_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_LINK_TRAINING_SWITCH_MODE {
DP_LINK_TRAINING_SWITCH_TO_IDLE          = 0x00000000u32,
DP_LINK_TRAINING_SWITCH_TO_VIDEO         = 0x00000001u32,
}

/*
 * DP_ML_PHY_SEQ_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_ML_PHY_SEQ_MODE {
DP_ML_PHY_SEQ_LINE_NUM                   = 0x00000000u32,
DP_ML_PHY_SEQ_IMMEDIATE                  = 0x00000001u32,
}

/*
 * DP_MSA_V_TIMING_OVERRIDE_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_MSA_V_TIMING_OVERRIDE_EN {
MSA_V_TIMING_OVERRIDE_DISABLED           = 0x00000000u32,
MSA_V_TIMING_OVERRIDE_ENABLED            = 0x00000001u32,
}

/*
 * DP_MSE_BLANK_CODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_MSE_BLANK_CODE {
DP_MSE_BLANK_CODE_SF_FILLED              = 0x00000000u32,
DP_MSE_BLANK_CODE_ZERO_FILLED            = 0x00000001u32,
}

/*
 * DP_MSE_LINK_LINE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_MSE_LINK_LINE {
DP_MSE_LINK_LINE_32_MTP_LONG             = 0x00000000u32,
DP_MSE_LINK_LINE_64_MTP_LONG             = 0x00000001u32,
DP_MSE_LINK_LINE_128_MTP_LONG            = 0x00000002u32,
DP_MSE_LINK_LINE_256_MTP_LONG            = 0x00000003u32,
}

/*
 * DP_MSE_TIMESTAMP_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_MSE_TIMESTAMP_MODE {
DP_MSE_TIMESTAMP_CALC_BASED_ON_LINK_RATE = 0x00000000u32,
DP_MSE_TIMESTAMP_CALC_BASED_ON_VC_RATE   = 0x00000001u32,
}

/*
 * DP_MSE_ZERO_ENCODER enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_MSE_ZERO_ENCODER {
DP_MSE_NOT_ZERO_FE_ENCODER               = 0x00000000u32,
DP_MSE_ZERO_FE_ENCODER                   = 0x00000001u32,
}

/*
 * DP_MSO_NUM_OF_SST_LINKS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_MSO_NUM_OF_SST_LINKS {
DP_MSO_ONE_SSTLINK                       = 0x00000000u32,
DP_MSO_TWO_SSTLINK                       = 0x00000001u32,
DP_MSO_FOUR_SSTLINK                      = 0x00000002u32,
}

/*
 * DP_PIXEL_ENCODING enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_PIXEL_ENCODING {
DP_PIXEL_ENCODING_RGB_YCBCR444           = 0x00000000u32,
DP_PIXEL_ENCODING_YCBCR422               = 0x00000001u32,
DP_PIXEL_ENCODING_YCBCR420               = 0x00000002u32,
DP_PIXEL_ENCODING_Y_ONLY                 = 0x00000003u32,
}

/*
 * DP_PIXEL_ENCODING_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_PIXEL_ENCODING_TYPE {
DP_PIXEL_ENCODING_UNCOMPRESSED           = 0x00000000u32,
DP_PIXEL_ENCODING_COMPRESSED             = 0x00000001u32,
}

/*
 * DP_SEC_ASP_CHANNEL_COUNT_OVERRIDE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_SEC_ASP_CHANNEL_COUNT_OVERRIDE {
DP_SEC_ASP_CHANNEL_COUNT_FROM_AZ         = 0x00000000u32,
DP_SEC_ASP_CHANNEL_COUNT_OVERRIDE_ENABLED = 0x00000001u32,
}

/*
 * DP_SEC_ASP_PRIORITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_SEC_ASP_PRIORITY {
DP_SEC_ASP_LOW_PRIORITY                  = 0x00000000u32,
DP_SEC_ASP_HIGH_PRIORITY                 = 0x00000001u32,
}

/*
 * DP_SEC_AUDIO_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_SEC_AUDIO_MUTE {
DP_SEC_AUDIO_MUTE_HW_CTRL                = 0x00000000u32,
DP_SEC_AUDIO_MUTE_SW_CTRL                = 0x00000001u32,
}

/*
 * DP_SEC_COLLISION_ACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_SEC_COLLISION_ACK {
DP_SEC_COLLISION_ACK_NO_EFFECT           = 0x00000000u32,
DP_SEC_COLLISION_ACK_CLR_FLAG            = 0x00000001u32,
}

/*
 * DP_SEC_GSP0_PRIORITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_SEC_GSP0_PRIORITY {
SEC_GSP0_PRIORITY_LOW                    = 0x00000000u32,
SEC_GSP0_PRIORITY_HIGH                   = 0x00000001u32,
}

/*
 * DP_SEC_GSP_SEND enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_SEC_GSP_SEND {
NOT_SENT                                 = 0x00000000u32,
FORCE_SENT                               = 0x00000001u32,
}

/*
 * DP_SEC_GSP_SEND_ANY_LINE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_SEC_GSP_SEND_ANY_LINE {
SEND_AT_LINK_NUMBER                      = 0x00000000u32,
SEND_AT_EARLIEST_TIME                    = 0x00000001u32,
}

/*
 * DP_SEC_GSP_SEND_PPS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_SEC_GSP_SEND_PPS {
SEND_NORMAL_PACKET                       = 0x00000000u32,
SEND_PPS_PACKET                          = 0x00000001u32,
}

/*
 * DP_SEC_LINE_REFERENCE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_SEC_LINE_REFERENCE {
REFER_TO_DP_SOF                          = 0x00000000u32,
REFER_TO_OTG_SOF                         = 0x00000001u32,
}

/*
 * DP_SEC_TIMESTAMP_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_SEC_TIMESTAMP_MODE {
DP_SEC_TIMESTAMP_PROGRAMMABLE_MODE       = 0x00000000u32,
DP_SEC_TIMESTAMP_AUTO_CALC_MODE          = 0x00000001u32,
}

/*
 * DP_STEER_OUTPUT_PIXEL_PER_CYCLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_STEER_OUTPUT_PIXEL_PER_CYCLE {
DP_STEER_1_PIX_PER_CYCLE                 = 0x00000000u32,
DP_STEER_2_PIX_PER_CYCLE                 = 0x00000001u32,
DP_STEER_4_PIX_PER_CYCLE                 = 0x00000002u32,
DP_STEER_8_PIX_PER_CYCLE                 = 0x00000003u32,
}

/*
 * DP_STEER_OVERFLOW_ACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_STEER_OVERFLOW_ACK {
DP_STEER_OVERFLOW_ACK_NO_EFFECT          = 0x00000000u32,
DP_STEER_OVERFLOW_ACK_CLR_INTERRUPT      = 0x00000001u32,
}

/*
 * DP_STEER_OVERFLOW_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_STEER_OVERFLOW_MASK {
DP_STEER_OVERFLOW_MASKED                 = 0x00000000u32,
DP_STEER_OVERFLOW_UNMASK                 = 0x00000001u32,
}

/*
 * DP_SYNC_POLARITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_SYNC_POLARITY {
DP_SYNC_POLARITY_ACTIVE_HIGH             = 0x00000000u32,
DP_SYNC_POLARITY_ACTIVE_LOW              = 0x00000001u32,
}

/*
 * DP_TU_OVERFLOW_ACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_TU_OVERFLOW_ACK {
DP_TU_OVERFLOW_ACK_NO_EFFECT             = 0x00000000u32,
DP_TU_OVERFLOW_ACK_CLR_INTERRUPT         = 0x00000001u32,
}

/*
 * DP_UDI_LANES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_UDI_LANES {
DP_UDI_1_LANE                            = 0x00000000u32,
DP_UDI_2_LANES                           = 0x00000001u32,
DP_UDI_LANES_RESERVED                    = 0x00000002u32,
DP_UDI_4_LANES                           = 0x00000003u32,
}

/*
 * DP_VID_ENHANCED_FRAME_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_VID_ENHANCED_FRAME_MODE {
VID_NORMAL_FRAME_MODE                    = 0x00000000u32,
VID_ENHANCED_MODE                        = 0x00000001u32,
}

/*
 * DP_VID_M_N_DOUBLE_BUFFER_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_VID_M_N_DOUBLE_BUFFER_MODE {
DP_VID_M_N_DOUBLE_BUFFER_AFTER_VID_M_UPDATE = 0x00000000u32,
DP_VID_M_N_DOUBLE_BUFFER_AT_FRAME_START  = 0x00000001u32,
}

/*
 * DP_VID_M_N_GEN_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_VID_M_N_GEN_EN {
DP_VID_M_N_PROGRAMMED_VIA_REG            = 0x00000000u32,
DP_VID_M_N_CALC_AUTO                     = 0x00000001u32,
}

/*
 * DP_VID_N_INTERVAL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_VID_N_INTERVAL {
DP_VID_1X_Nvid                           = 0x00000000u32,
DP_VID_2X_Nvid                           = 0x00000001u32,
DP_VID_4X_Nvid                           = 0x00000002u32,
DP_VID_8X_Nvid                           = 0x00000003u32,
}

/*
 * DP_VID_STREAM_DISABLE_ACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_VID_STREAM_DISABLE_ACK {
ID_STREAM_DISABLE_NO_ACK                 = 0x00000000u32,
ID_STREAM_DISABLE_ACKED                  = 0x00000001u32,
}

/*
 * DP_VID_STREAM_DISABLE_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_VID_STREAM_DISABLE_MASK {
VID_STREAM_DISABLE_MASKED                = 0x00000000u32,
VID_STREAM_DISABLE_UNMASK                = 0x00000001u32,
}

/*
 * DP_VID_STREAM_DIS_DEFER enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_VID_STREAM_DIS_DEFER {
DP_VID_STREAM_DIS_NO_DEFER               = 0x00000000u32,
DP_VID_STREAM_DIS_DEFER_TO_HBLANK        = 0x00000001u32,
DP_VID_STREAM_DIS_DEFER_TO_VBLANK        = 0x00000002u32,
}

/*
 * DP_VID_VBID_FIELD_POL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_VID_VBID_FIELD_POL {
DP_VID_VBID_FIELD_POL_NORMAL             = 0x00000000u32,
DP_VID_VBID_FIELD_POL_INV                = 0x00000001u32,
}

/*
 * FEC_ACTIVE_STATUS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FEC_ACTIVE_STATUS {
DPHY_FEC_NOT_ACTIVE                      = 0x00000000u32,
DPHY_FEC_ACTIVE                          = 0x00000001u32,
}

/*******************************************************
 * DIG Enums
 *******************************************************/

/*
 * DIG_BE_CNTL_HPD_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_BE_CNTL_HPD_SELECT {
DIG_BE_CNTL_HPD1                         = 0x00000000u32,
DIG_BE_CNTL_HPD2                         = 0x00000001u32,
DIG_BE_CNTL_HPD3                         = 0x00000002u32,
DIG_BE_CNTL_HPD4                         = 0x00000003u32,
DIG_BE_CNTL_NO_HPD                       = 0x00000004u32,
}

/*
 * DIG_BE_CNTL_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_BE_CNTL_MODE {
DIG_BE_DP_SST_MODE                       = 0x00000000u32,
DIG_BE_RESERVED1                         = 0x00000001u32,
DIG_BE_TMDS_DVI_MODE                     = 0x00000002u32,
DIG_BE_TMDS_HDMI_MODE                    = 0x00000003u32,
DIG_BE_RESERVED4                         = 0x00000004u32,
DIG_BE_DP_MST_MODE                       = 0x00000005u32,
DIG_BE_RESERVED2                         = 0x00000006u32,
DIG_BE_RESERVED3                         = 0x00000007u32,
}

/*
 * DIG_DIGITAL_BYPASS_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_DIGITAL_BYPASS_ENABLE {
DIG_DIGITAL_BYPASS_OFF                   = 0x00000000u32,
DIG_DIGITAL_BYPASS_ON                    = 0x00000001u32,
}

/*
 * DIG_DIGITAL_BYPASS_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_DIGITAL_BYPASS_SEL {
DIG_DIGITAL_BYPASS_SEL_BYPASS            = 0x00000000u32,
DIG_DIGITAL_BYPASS_SEL_36BPP             = 0x00000001u32,
DIG_DIGITAL_BYPASS_SEL_48BPP_LSB         = 0x00000002u32,
DIG_DIGITAL_BYPASS_SEL_48BPP_MSB         = 0x00000003u32,
DIG_DIGITAL_BYPASS_SEL_10BPP_LSB         = 0x00000004u32,
DIG_DIGITAL_BYPASS_SEL_12BPC_LSB         = 0x00000005u32,
DIG_DIGITAL_BYPASS_SEL_ALPHA             = 0x00000006u32,
}

/*
 * DIG_FE_CNTL_SOURCE_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_FE_CNTL_SOURCE_SELECT {
DIG_FE_SOURCE_FROM_OTG0                  = 0x00000000u32,
DIG_FE_SOURCE_FROM_OTG1                  = 0x00000001u32,
DIG_FE_SOURCE_FROM_OTG2                  = 0x00000002u32,
DIG_FE_SOURCE_FROM_OTG3                  = 0x00000003u32,
DIG_FE_SOURCE_RESERVED                   = 0x00000004u32,
}

/*
 * DIG_FE_CNTL_STEREOSYNC_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_FE_CNTL_STEREOSYNC_SELECT {
DIG_FE_STEREOSYNC_FROM_OTG0              = 0x00000000u32,
DIG_FE_STEREOSYNC_FROM_OTG1              = 0x00000001u32,
DIG_FE_STEREOSYNC_FROM_OTG2              = 0x00000002u32,
DIG_FE_STEREOSYNC_FROM_OTG3              = 0x00000003u32,
DIG_FE_STEREOSYNC_RESERVED               = 0x00000004u32,
}

/*
 * DIG_FIFO_CTRL_FORCE_RECOMP_MINMAX enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_FIFO_CTRL_FORCE_RECOMP_MINMAX {
DIG_FIFO_NOT_FORCE_RECOMP_MINMAX         = 0x00000000u32,
DIG_FIFO_FORCE_RECOMP_MINMAX             = 0x00000001u32,
}

/*
 * DIG_FIFO_CTRL_USE_OVERWRITE_LEVEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_FIFO_CTRL_USE_OVERWRITE_LEVEL {
DIG_FIFO_USE_OVERWRITE_LEVEL             = 0x00000000u32,
DIG_FIFO_USE_CAL_AVERAGE_LEVEL           = 0x00000001u32,
}

/*
 * DIG_FIFO_FORCE_RECAL_AVERAGE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_FIFO_FORCE_RECAL_AVERAGE {
DIG_FIFO_NOT_FORCE_RECAL_AVERAGE         = 0x00000000u32,
DIG_FIFO_FORCE_RECAL_AVERAGE_LEVEL       = 0x00000001u32,
}

/*
 * DIG_FIFO_OUTPUT_PIXEL_PER_CYCLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_FIFO_OUTPUT_PIXEL_PER_CYCLE {
DIG_FIFO_1_PIX_PER_CYCLE                 = 0x00000000u32,
DIG_FIFO_2_PIX_PER_CYCLE                 = 0x00000001u32,
DIG_FIFO_4_PIX_PER_CYCLE                 = 0x00000002u32,
DIG_FIFO_8_PIX_PER_CYCLE                 = 0x00000003u32,
}

/*
 * DIG_FIFO_OVERFLOW_UNDERFLOW_ERROR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_FIFO_OVERFLOW_UNDERFLOW_ERROR {
DIG_FIFO_NO_ERROR_OCCURRED               = 0x00000000u32,
DIG_FIFO_UNDERFLOW_OCCURRED              = 0x00000001u32,
DIG_FIFO_OVERFLOW_OCCURRED               = 0x00000002u32,
}

/*
 * DIG_FIFO_READ_CLOCK_SRC enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_FIFO_READ_CLOCK_SRC {
DIG_FIFO_READ_CLOCK_SRC_FROM_DCCG        = 0x00000000u32,
DIG_FIFO_READ_CLOCK_SRC_FROM_DISPLAY_PIPE = 0x00000001u32,
}

/*
 * DIG_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_MODE {
DP_SST_MODE                              = 0x00000000u32,
RESERVED1                                = 0x00000001u32,
TMDS_DVI_MODE                            = 0x00000002u32,
TMDS_HDMI_MODE                           = 0x00000003u32,
RESERVED4                                = 0x00000004u32,
DP_MST_MODE                              = 0x00000005u32,
RESERVED2                                = 0x00000006u32,
RESERVED3                                = 0x00000007u32,
}

/*
 * DIG_OUTPUT_CRC_CNTL_LINK_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_OUTPUT_CRC_CNTL_LINK_SEL {
DIG_OUTPUT_CRC_ON_LINK0                  = 0x00000000u32,
DIG_OUTPUT_CRC_ON_LINK1                  = 0x00000001u32,
}

/*
 * DIG_OUTPUT_CRC_DATA_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_OUTPUT_CRC_DATA_SEL {
DIG_OUTPUT_CRC_FOR_FULLFRAME             = 0x00000000u32,
DIG_OUTPUT_CRC_FOR_ACTIVEONLY            = 0x00000001u32,
DIG_OUTPUT_CRC_FOR_VBI                   = 0x00000002u32,
DIG_OUTPUT_CRC_FOR_AUDIO                 = 0x00000003u32,
}

/*
 * DIG_RANDOM_PATTERN_SEED_RAN_PAT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_RANDOM_PATTERN_SEED_RAN_PAT {
DIG_RANDOM_PATTERN_SEED_RAN_PAT_ALL_PIXELS = 0x00000000u32,
DIG_RANDOM_PATTERN_SEED_RAN_PAT_DE_HIGH  = 0x00000001u32,
}

/*
 * DIG_TEST_PATTERN_EXTERNAL_RESET_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_TEST_PATTERN_EXTERNAL_RESET_EN {
DIG_TEST_PATTERN_EXTERNAL_RESET_ENABLE   = 0x00000000u32,
DIG_TEST_PATTERN_EXTERNAL_RESET_BY_EXT_SIG = 0x00000001u32,
}

/*
 * DIG_TEST_PATTERN_HALF_CLOCK_PATTERN_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_TEST_PATTERN_HALF_CLOCK_PATTERN_SEL {
DIG_10BIT_TEST_PATTERN                   = 0x00000000u32,
DIG_ALTERNATING_TEST_PATTERN             = 0x00000001u32,
}

/*
 * DIG_TEST_PATTERN_RANDOM_PATTERN_OUT_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_TEST_PATTERN_RANDOM_PATTERN_OUT_EN {
DIG_TEST_PATTERN_NORMAL                  = 0x00000000u32,
DIG_TEST_PATTERN_RANDOM                  = 0x00000001u32,
}

/*
 * DIG_TEST_PATTERN_RANDOM_PATTERN_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_TEST_PATTERN_RANDOM_PATTERN_RESET {
DIG_RANDOM_PATTERN_ENABLED               = 0x00000000u32,
DIG_RANDOM_PATTERN_RESETED               = 0x00000001u32,
}

/*
 * DIG_TEST_PATTERN_TEST_PATTERN_OUT_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_TEST_PATTERN_TEST_PATTERN_OUT_EN {
DIG_IN_NORMAL_OPERATION                  = 0x00000000u32,
DIG_IN_DEBUG_MODE                        = 0x00000001u32,
}

/*
 * HDMI_ACP_SEND enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_ACP_SEND {
HDMI_ACP_NOT_SEND                        = 0x00000000u32,
HDMI_ACP_PKT_SEND                        = 0x00000001u32,
}

/*
 * HDMI_ACR_AUDIO_PRIORITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_ACR_AUDIO_PRIORITY {
HDMI_ACR_PKT_HIGH_PRIORITY_THAN_AUDIO_SAMPLE = 0x00000000u32,
HDMI_AUDIO_SAMPLE_HIGH_PRIORITY_THAN_ACR_PKT = 0x00000001u32,
}

/*
 * HDMI_ACR_CONT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_ACR_CONT {
HDMI_ACR_CONT_DISABLE                    = 0x00000000u32,
HDMI_ACR_CONT_ENABLE                     = 0x00000001u32,
}

/*
 * HDMI_ACR_N_MULTIPLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_ACR_N_MULTIPLE {
HDMI_ACR_0_MULTIPLE_RESERVED             = 0x00000000u32,
HDMI_ACR_1_MULTIPLE                      = 0x00000001u32,
HDMI_ACR_2_MULTIPLE                      = 0x00000002u32,
HDMI_ACR_3_MULTIPLE_RESERVED             = 0x00000003u32,
HDMI_ACR_4_MULTIPLE                      = 0x00000004u32,
HDMI_ACR_5_MULTIPLE_RESERVED             = 0x00000005u32,
HDMI_ACR_6_MULTIPLE_RESERVED             = 0x00000006u32,
HDMI_ACR_7_MULTIPLE_RESERVED             = 0x00000007u32,
}

/*
 * HDMI_ACR_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_ACR_SELECT {
HDMI_ACR_SELECT_HW                       = 0x00000000u32,
HDMI_ACR_SELECT_32K                      = 0x00000001u32,
HDMI_ACR_SELECT_44K                      = 0x00000002u32,
HDMI_ACR_SELECT_48K                      = 0x00000003u32,
}

/*
 * HDMI_ACR_SEND enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_ACR_SEND {
HDMI_ACR_NOT_SEND                        = 0x00000000u32,
HDMI_ACR_PKT_SEND                        = 0x00000001u32,
}

/*
 * HDMI_ACR_SOURCE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_ACR_SOURCE {
HDMI_ACR_SOURCE_HW                       = 0x00000000u32,
HDMI_ACR_SOURCE_SW                       = 0x00000001u32,
}

/*
 * HDMI_AUDIO_DELAY_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_AUDIO_DELAY_EN {
HDMI_AUDIO_DELAY_DISABLE                 = 0x00000000u32,
HDMI_AUDIO_DELAY_58CLK                   = 0x00000001u32,
HDMI_AUDIO_DELAY_56CLK                   = 0x00000002u32,
HDMI_AUDIO_DELAY_RESERVED                = 0x00000003u32,
}

/*
 * HDMI_AUDIO_INFO_CONT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_AUDIO_INFO_CONT {
HDMI_AUDIO_INFO_CONT_DISABLE             = 0x00000000u32,
HDMI_AUDIO_INFO_CONT_ENABLE              = 0x00000001u32,
}

/*
 * HDMI_AUDIO_INFO_SEND enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_AUDIO_INFO_SEND {
HDMI_AUDIO_INFO_NOT_SEND                 = 0x00000000u32,
HDMI_AUDIO_INFO_PKT_SEND                 = 0x00000001u32,
}

/*
 * HDMI_CLOCK_CHANNEL_RATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_CLOCK_CHANNEL_RATE {
HDMI_CLOCK_CHANNEL_FREQ_EQUAL_TO_CHAR_RATE = 0x00000000u32,
HDMI_CLOCK_CHANNEL_FREQ_QUARTER_TO_CHAR_RATE = 0x00000001u32,
}

/*
 * HDMI_DATA_SCRAMBLE_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_DATA_SCRAMBLE_EN {
HDMI_DATA_SCRAMBLE_DISABLE               = 0x00000000u32,
HDMI_DATA_SCRAMBLE_ENABLE                = 0x00000001u32,
}

/*
 * HDMI_DEEP_COLOR_DEPTH enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_DEEP_COLOR_DEPTH {
HDMI_DEEP_COLOR_DEPTH_24BPP              = 0x00000000u32,
HDMI_DEEP_COLOR_DEPTH_30BPP              = 0x00000001u32,
HDMI_DEEP_COLOR_DEPTH_36BPP              = 0x00000002u32,
HDMI_DEEP_COLOR_DEPTH_48BPP              = 0x00000003u32,
}

/*
 * HDMI_DEFAULT_PAHSE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_DEFAULT_PAHSE {
HDMI_DEFAULT_PHASE_IS_0                  = 0x00000000u32,
HDMI_DEFAULT_PHASE_IS_1                  = 0x00000001u32,
}

/*
 * HDMI_ERROR_ACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_ERROR_ACK {
HDMI_ERROR_ACK_INT                       = 0x00000000u32,
HDMI_ERROR_NOT_ACK                       = 0x00000001u32,
}

/*
 * HDMI_ERROR_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_ERROR_MASK {
HDMI_ERROR_MASK_INT                      = 0x00000000u32,
HDMI_ERROR_NOT_MASK                      = 0x00000001u32,
}

/*
 * HDMI_GC_AVMUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_GC_AVMUTE {
HDMI_GC_AVMUTE_SET                       = 0x00000000u32,
HDMI_GC_AVMUTE_UNSET                     = 0x00000001u32,
}

/*
 * HDMI_GC_AVMUTE_CONT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_GC_AVMUTE_CONT {
HDMI_GC_AVMUTE_CONT_DISABLE              = 0x00000000u32,
HDMI_GC_AVMUTE_CONT_ENABLE               = 0x00000001u32,
}

/*
 * HDMI_GC_CONT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_GC_CONT {
HDMI_GC_CONT_DISABLE                     = 0x00000000u32,
HDMI_GC_CONT_ENABLE                      = 0x00000001u32,
}

/*
 * HDMI_GC_SEND enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_GC_SEND {
HDMI_GC_NOT_SEND                         = 0x00000000u32,
HDMI_GC_PKT_SEND                         = 0x00000001u32,
}

/*
 * HDMI_GENERIC_CONT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_GENERIC_CONT {
HDMI_GENERIC_CONT_DISABLE                = 0x00000000u32,
HDMI_GENERIC_CONT_ENABLE                 = 0x00000001u32,
}

/*
 * HDMI_GENERIC_SEND enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_GENERIC_SEND {
HDMI_GENERIC_NOT_SEND                    = 0x00000000u32,
HDMI_GENERIC_PKT_SEND                    = 0x00000001u32,
}

/*
 * HDMI_ISRC_CONT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_ISRC_CONT {
HDMI_ISRC_CONT_DISABLE                   = 0x00000000u32,
HDMI_ISRC_CONT_ENABLE                    = 0x00000001u32,
}

/*
 * HDMI_ISRC_SEND enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_ISRC_SEND {
HDMI_ISRC_NOT_SEND                       = 0x00000000u32,
HDMI_ISRC_PKT_SEND                       = 0x00000001u32,
}

/*
 * HDMI_KEEPOUT_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_KEEPOUT_MODE {
HDMI_KEEPOUT_0_650PIX_AFTER_VSYNC        = 0x00000000u32,
HDMI_KEEPOUT_509_650PIX_AFTER_VSYNC      = 0x00000001u32,
}

/*
 * HDMI_METADATA_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_METADATA_ENABLE {
HDMI_METADATA_NOT_SEND                   = 0x00000000u32,
HDMI_METADATA_PKT_SEND                   = 0x00000001u32,
}

/*
 * HDMI_MPEG_INFO_CONT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_MPEG_INFO_CONT {
HDMI_MPEG_INFO_CONT_DISABLE              = 0x00000000u32,
HDMI_MPEG_INFO_CONT_ENABLE               = 0x00000001u32,
}

/*
 * HDMI_MPEG_INFO_SEND enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_MPEG_INFO_SEND {
HDMI_MPEG_INFO_NOT_SEND                  = 0x00000000u32,
HDMI_MPEG_INFO_PKT_SEND                  = 0x00000001u32,
}

/*
 * HDMI_NO_EXTRA_NULL_PACKET_FILLED enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_NO_EXTRA_NULL_PACKET_FILLED {
HDMI_EXTRA_NULL_PACKET_FILLED_ENABLE     = 0x00000000u32,
HDMI_EXTRA_NULL_PACKET_FILLED_DISABLE    = 0x00000001u32,
}

/*
 * HDMI_NULL_SEND enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_NULL_SEND {
HDMI_NULL_NOT_SEND                       = 0x00000000u32,
HDMI_NULL_PKT_SEND                       = 0x00000001u32,
}

/*
 * HDMI_PACKET_GEN_VERSION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_PACKET_GEN_VERSION {
HDMI_PACKET_GEN_VERSION_OLD              = 0x00000000u32,
HDMI_PACKET_GEN_VERSION_NEW              = 0x00000001u32,
}

/*
 * HDMI_PACKET_LINE_REFERENCE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_PACKET_LINE_REFERENCE {
HDMI_PKT_LINE_REF_VSYNC                  = 0x00000000u32,
HDMI_PKT_LINE_REF_OTGSOF                 = 0x00000001u32,
}

/*
 * HDMI_PACKING_PHASE_OVERRIDE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HDMI_PACKING_PHASE_OVERRIDE {
HDMI_PACKING_PHASE_SET_BY_HW             = 0x00000000u32,
HDMI_PACKING_PHASE_SET_BY_SW             = 0x00000001u32,
}

/*
 * LVTMA_RANDOM_PATTERN_SEED_RAN_PAT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum LVTMA_RANDOM_PATTERN_SEED_RAN_PAT {
LVTMA_RANDOM_PATTERN_SEED_ALL_PIXELS     = 0x00000000u32,
LVTMA_RANDOM_PATTERN_SEED_ONLY_DE_HIGH   = 0x00000001u32,
}

/*
 * TMDS_COLOR_FORMAT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_COLOR_FORMAT {
TMDS_COLOR_FORMAT__24BPP__TWIN30BPP_MSB__DUAL48BPP = 0x00000000u32,
TMDS_COLOR_FORMAT_TWIN30BPP_LSB          = 0x00000001u32,
TMDS_COLOR_FORMAT_DUAL30BPP              = 0x00000002u32,
TMDS_COLOR_FORMAT_RESERVED               = 0x00000003u32,
}

/*
 * TMDS_CTL0_DATA_INVERT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_CTL0_DATA_INVERT {
TMDS_CTL0_DATA_NORMAL                    = 0x00000000u32,
TMDS_CTL0_DATA_INVERT_EN                 = 0x00000001u32,
}

/*
 * TMDS_CTL0_DATA_MODULATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_CTL0_DATA_MODULATION {
TMDS_CTL0_DATA_MODULATION_DISABLE        = 0x00000000u32,
TMDS_CTL0_DATA_MODULATION_BIT0           = 0x00000001u32,
TMDS_CTL0_DATA_MODULATION_BIT1           = 0x00000002u32,
TMDS_CTL0_DATA_MODULATION_BIT2           = 0x00000003u32,
}

/*
 * TMDS_CTL0_DATA_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_CTL0_DATA_SEL {
TMDS_CTL0_DATA_SEL0_RESERVED             = 0x00000000u32,
TMDS_CTL0_DATA_SEL1_DISPLAY_ENABLE       = 0x00000001u32,
TMDS_CTL0_DATA_SEL2_VSYNC                = 0x00000002u32,
TMDS_CTL0_DATA_SEL3_RESERVED             = 0x00000003u32,
TMDS_CTL0_DATA_SEL4_HSYNC                = 0x00000004u32,
TMDS_CTL0_DATA_SEL5_SEL7_RESERVED        = 0x00000005u32,
TMDS_CTL0_DATA_SEL8_RANDOM_DATA          = 0x00000006u32,
TMDS_CTL0_DATA_SEL9_SEL15_RANDOM_DATA    = 0x00000007u32,
}

/*
 * TMDS_CTL0_PATTERN_OUT_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_CTL0_PATTERN_OUT_EN {
TMDS_CTL0_PATTERN_OUT_DISABLE            = 0x00000000u32,
TMDS_CTL0_PATTERN_OUT_ENABLE             = 0x00000001u32,
}

/*
 * TMDS_CTL1_DATA_INVERT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_CTL1_DATA_INVERT {
TMDS_CTL1_DATA_NORMAL                    = 0x00000000u32,
TMDS_CTL1_DATA_INVERT_EN                 = 0x00000001u32,
}

/*
 * TMDS_CTL1_DATA_MODULATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_CTL1_DATA_MODULATION {
TMDS_CTL1_DATA_MODULATION_DISABLE        = 0x00000000u32,
TMDS_CTL1_DATA_MODULATION_BIT0           = 0x00000001u32,
TMDS_CTL1_DATA_MODULATION_BIT1           = 0x00000002u32,
TMDS_CTL1_DATA_MODULATION_BIT2           = 0x00000003u32,
}

/*
 * TMDS_CTL1_DATA_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_CTL1_DATA_SEL {
TMDS_CTL1_DATA_SEL0_RESERVED             = 0x00000000u32,
TMDS_CTL1_DATA_SEL1_DISPLAY_ENABLE       = 0x00000001u32,
TMDS_CTL1_DATA_SEL2_VSYNC                = 0x00000002u32,
TMDS_CTL1_DATA_SEL3_RESERVED             = 0x00000003u32,
TMDS_CTL1_DATA_SEL4_HSYNC                = 0x00000004u32,
TMDS_CTL1_DATA_SEL5_SEL7_RESERVED        = 0x00000005u32,
TMDS_CTL1_DATA_SEL8_BLANK_TIME           = 0x00000006u32,
TMDS_CTL1_DATA_SEL9_SEL15_RESERVED       = 0x00000007u32,
}

/*
 * TMDS_CTL1_PATTERN_OUT_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_CTL1_PATTERN_OUT_EN {
TMDS_CTL1_PATTERN_OUT_DISABLE            = 0x00000000u32,
TMDS_CTL1_PATTERN_OUT_ENABLE             = 0x00000001u32,
}

/*
 * TMDS_CTL2_DATA_INVERT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_CTL2_DATA_INVERT {
TMDS_CTL2_DATA_NORMAL                    = 0x00000000u32,
TMDS_CTL2_DATA_INVERT_EN                 = 0x00000001u32,
}

/*
 * TMDS_CTL2_DATA_MODULATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_CTL2_DATA_MODULATION {
TMDS_CTL2_DATA_MODULATION_DISABLE        = 0x00000000u32,
TMDS_CTL2_DATA_MODULATION_BIT0           = 0x00000001u32,
TMDS_CTL2_DATA_MODULATION_BIT1           = 0x00000002u32,
TMDS_CTL2_DATA_MODULATION_BIT2           = 0x00000003u32,
}

/*
 * TMDS_CTL2_DATA_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_CTL2_DATA_SEL {
TMDS_CTL2_DATA_SEL0_RESERVED             = 0x00000000u32,
TMDS_CTL2_DATA_SEL1_DISPLAY_ENABLE       = 0x00000001u32,
TMDS_CTL2_DATA_SEL2_VSYNC                = 0x00000002u32,
TMDS_CTL2_DATA_SEL3_RESERVED             = 0x00000003u32,
TMDS_CTL2_DATA_SEL4_HSYNC                = 0x00000004u32,
TMDS_CTL2_DATA_SEL5_SEL7_RESERVED        = 0x00000005u32,
TMDS_CTL2_DATA_SEL8_BLANK_TIME           = 0x00000006u32,
TMDS_CTL2_DATA_SEL9_SEL15_RESERVED       = 0x00000007u32,
}

/*
 * TMDS_CTL2_PATTERN_OUT_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_CTL2_PATTERN_OUT_EN {
TMDS_CTL2_PATTERN_OUT_DISABLE            = 0x00000000u32,
TMDS_CTL2_PATTERN_OUT_ENABLE             = 0x00000001u32,
}

/*
 * TMDS_CTL3_DATA_INVERT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_CTL3_DATA_INVERT {
TMDS_CTL3_DATA_NORMAL                    = 0x00000000u32,
TMDS_CTL3_DATA_INVERT_EN                 = 0x00000001u32,
}

/*
 * TMDS_CTL3_DATA_MODULATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_CTL3_DATA_MODULATION {
TMDS_CTL3_DATA_MODULATION_DISABLE        = 0x00000000u32,
TMDS_CTL3_DATA_MODULATION_BIT0           = 0x00000001u32,
TMDS_CTL3_DATA_MODULATION_BIT1           = 0x00000002u32,
TMDS_CTL3_DATA_MODULATION_BIT2           = 0x00000003u32,
}

/*
 * TMDS_CTL3_DATA_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_CTL3_DATA_SEL {
TMDS_CTL3_DATA_SEL0_RESERVED             = 0x00000000u32,
TMDS_CTL3_DATA_SEL1_DISPLAY_ENABLE       = 0x00000001u32,
TMDS_CTL3_DATA_SEL2_VSYNC                = 0x00000002u32,
TMDS_CTL3_DATA_SEL3_RESERVED             = 0x00000003u32,
TMDS_CTL3_DATA_SEL4_HSYNC                = 0x00000004u32,
TMDS_CTL3_DATA_SEL5_SEL7_RESERVED        = 0x00000005u32,
TMDS_CTL3_DATA_SEL8_BLANK_TIME           = 0x00000006u32,
TMDS_CTL3_DATA_SEL9_SEL15_RESERVED       = 0x00000007u32,
}

/*
 * TMDS_CTL3_PATTERN_OUT_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_CTL3_PATTERN_OUT_EN {
TMDS_CTL3_PATTERN_OUT_DISABLE            = 0x00000000u32,
TMDS_CTL3_PATTERN_OUT_ENABLE             = 0x00000001u32,
}

/*
 * TMDS_DATA_SYNCHRONIZATION_DSINTSEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_DATA_SYNCHRONIZATION_DSINTSEL {
TMDS_DATA_SYNCHRONIZATION_DSINTSEL_PCLK_TMDS = 0x00000000u32,
TMDS_DATA_SYNCHRONIZATION_DSINTSEL_TMDS_PLL = 0x00000001u32,
}

/*
 * TMDS_PIXEL_ENCODING enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_PIXEL_ENCODING {
TMDS_PIXEL_ENCODING_444_OR_420           = 0x00000000u32,
TMDS_PIXEL_ENCODING_422                  = 0x00000001u32,
}

/*
 * TMDS_REG_TEST_OUTPUTA_CNTLA enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_REG_TEST_OUTPUTA_CNTLA {
TMDS_REG_TEST_OUTPUTA_CNTLA_OTDATA0      = 0x00000000u32,
TMDS_REG_TEST_OUTPUTA_CNTLA_OTDATA1      = 0x00000001u32,
TMDS_REG_TEST_OUTPUTA_CNTLA_OTDATA2      = 0x00000002u32,
TMDS_REG_TEST_OUTPUTA_CNTLA_NA           = 0x00000003u32,
}

/*
 * TMDS_REG_TEST_OUTPUTB_CNTLB enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_REG_TEST_OUTPUTB_CNTLB {
TMDS_REG_TEST_OUTPUTB_CNTLB_OTDATB0      = 0x00000000u32,
TMDS_REG_TEST_OUTPUTB_CNTLB_OTDATB1      = 0x00000001u32,
TMDS_REG_TEST_OUTPUTB_CNTLB_OTDATB2      = 0x00000002u32,
TMDS_REG_TEST_OUTPUTB_CNTLB_NA           = 0x00000003u32,
}

/*
 * TMDS_STEREOSYNC_CTL_SEL_REG enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_STEREOSYNC_CTL_SEL_REG {
TMDS_STEREOSYNC_CTL0                     = 0x00000000u32,
TMDS_STEREOSYNC_CTL1                     = 0x00000001u32,
TMDS_STEREOSYNC_CTL2                     = 0x00000002u32,
TMDS_STEREOSYNC_CTL3                     = 0x00000003u32,
}

/*
 * TMDS_SYNC_PHASE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_SYNC_PHASE {
TMDS_NOT_SYNC_PHASE_ON_FRAME_START       = 0x00000000u32,
TMDS_SYNC_PHASE_ON_FRAME_START           = 0x00000001u32,
}

/*
 * TMDS_TRANSMITTER_CONTROL_BYPASS_PLLA enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_TRANSMITTER_CONTROL_BYPASS_PLLA {
TMDS_TRANSMITTER_BYPASS_PLLA_COHERENT    = 0x00000000u32,
TMDS_TRANSMITTER_BYPASS_PLLA_INCOHERENT  = 0x00000001u32,
}

/*
 * TMDS_TRANSMITTER_CONTROL_BYPASS_PLLB enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_TRANSMITTER_CONTROL_BYPASS_PLLB {
TMDS_TRANSMITTER_BYPASS_PLLB_COHERENT    = 0x00000000u32,
TMDS_TRANSMITTER_BYPASS_PLLB_INCOHERENT  = 0x00000001u32,
}

/*
 * TMDS_TRANSMITTER_CONTROL_IDSCKSELA enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_TRANSMITTER_CONTROL_IDSCKSELA {
TMDS_TRANSMITTER_IDSCKSELA_USE_IPIXCLK   = 0x00000000u32,
TMDS_TRANSMITTER_IDSCKSELA_USE_IDCLK     = 0x00000001u32,
}

/*
 * TMDS_TRANSMITTER_CONTROL_IDSCKSELB enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_TRANSMITTER_CONTROL_IDSCKSELB {
TMDS_TRANSMITTER_IDSCKSELB_USE_IPIXCLK   = 0x00000000u32,
TMDS_TRANSMITTER_IDSCKSELB_USE_IDCLK     = 0x00000001u32,
}

/*
 * TMDS_TRANSMITTER_CONTROL_PLLSEL_OVERWRITE_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_TRANSMITTER_CONTROL_PLLSEL_OVERWRITE_EN {
TMDS_TRANSMITTER_PLLSEL_BY_HW            = 0x00000000u32,
TMDS_TRANSMITTER_PLLSEL_OVERWRITE_BY_SW  = 0x00000001u32,
}

/*
 * TMDS_TRANSMITTER_CONTROL_PLL_ENABLE_HPD_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_TRANSMITTER_CONTROL_PLL_ENABLE_HPD_MASK {
TMDS_TRANSMITTER_HPD_NOT_OVERRIDE_PLL_ENABLE = 0x00000000u32,
TMDS_TRANSMITTER_HPD_OVERRIDE_PLL_ENABLE_ON_DISCON = 0x00000001u32,
TMDS_TRANSMITTER_HPD_OVERRIDE_PLL_ENABLE_ON_CON = 0x00000002u32,
TMDS_TRANSMITTER_HPD_OVERRIDE_PLL_ENABLE = 0x00000003u32,
}

/*
 * TMDS_TRANSMITTER_CONTROL_PLL_PWRUP_SEQ_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_TRANSMITTER_CONTROL_PLL_PWRUP_SEQ_EN {
TMDS_TRANSMITTER_PLL_PWRUP_SEQ_DISABLE   = 0x00000000u32,
TMDS_TRANSMITTER_PLL_PWRUP_SEQ_ENABLE    = 0x00000001u32,
}

/*
 * TMDS_TRANSMITTER_CONTROL_PLL_RESET_HPD_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_TRANSMITTER_CONTROL_PLL_RESET_HPD_MASK {
TMDS_TRANSMITTER_PLL_NOT_RST_ON_HPD      = 0x00000000u32,
TMDS_TRANSMITTER_PLL_RST_ON_HPD          = 0x00000001u32,
}

/*
 * TMDS_TRANSMITTER_CONTROL_TDCLK_FROM_PADS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_TRANSMITTER_CONTROL_TDCLK_FROM_PADS {
TMDS_TRANSMITTER_TDCLK_FROM_TMDS_TDCLK   = 0x00000000u32,
TMDS_TRANSMITTER_TDCLK_FROM_PADS         = 0x00000001u32,
}

/*
 * TMDS_TRANSMITTER_CONTROL_TMCLK_FROM_PADS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_TRANSMITTER_CONTROL_TMCLK_FROM_PADS {
TMDS_TRANSMITTER_TMCLK_FROM_TMDS_TMCLK   = 0x00000000u32,
TMDS_TRANSMITTER_TMCLK_FROM_PADS         = 0x00000001u32,
}

/*
 * TMDS_TRANSMITTER_ENABLE_HPD_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_TRANSMITTER_ENABLE_HPD_MASK {
TMDS_TRANSMITTER_HPD_MASK_NOT_OVERRIDE   = 0x00000000u32,
TMDS_TRANSMITTER_HPD_MASK_OVERRIDE       = 0x00000001u32,
}

/*
 * TMDS_TRANSMITTER_ENABLE_LNKCEN_HPD_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_TRANSMITTER_ENABLE_LNKCEN_HPD_MASK {
TMDS_TRANSMITTER_LNKCEN_HPD_MASK_NOT_OVERRIDE = 0x00000000u32,
TMDS_TRANSMITTER_LNKCEN_HPD_MASK_OVERRIDE = 0x00000001u32,
}

/*
 * TMDS_TRANSMITTER_ENABLE_LNKDEN_HPD_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_TRANSMITTER_ENABLE_LNKDEN_HPD_MASK {
TMDS_TRANSMITTER_LNKDEN_HPD_MASK_NOT_OVERRIDE = 0x00000000u32,
TMDS_TRANSMITTER_LNKDEN_HPD_MASK_OVERRIDE = 0x00000001u32,
}

/*******************************************************
 * DOUT_I2C Enums
 *******************************************************/

/*
 * DOUT_I2C_ACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_ACK {
DOUT_I2C_NO_ACK                          = 0x00000000u32,
DOUT_I2C_ACK_TO_CLEAN                    = 0x00000001u32,
}

/*
 * DOUT_I2C_ARBITRATION_ABORT_XFER enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_ARBITRATION_ABORT_XFER {
DOUT_I2C_ARBITRATION_NOT_ABORT_CURRENT_TRANSFER = 0x00000000u32,
DOUT_I2C_ARBITRATION_ABORT_CURRENT_TRANSFER = 0x00000001u32,
}

/*
 * DOUT_I2C_ARBITRATION_DONE_USING_I2C_REG enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_ARBITRATION_DONE_USING_I2C_REG {
DOUT_I2C_ARBITRATION_DONE__NOT_USING_I2C_REG = 0x00000000u32,
DOUT_I2C_ARBITRATION_DONE__USING_I2C_REG = 0x00000001u32,
}

/*
 * DOUT_I2C_ARBITRATION_NO_QUEUED_SW_GO enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_ARBITRATION_NO_QUEUED_SW_GO {
DOUT_I2C_ARBITRATION_SW_QUEUE_ENABLED    = 0x00000000u32,
DOUT_I2C_ARBITRATION_SW_QUEUE_DISABLED   = 0x00000001u32,
}

/*
 * DOUT_I2C_ARBITRATION_SW_PRIORITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_ARBITRATION_SW_PRIORITY {
DOUT_I2C_ARBITRATION_SW_PRIORITY_NORMAL  = 0x00000000u32,
DOUT_I2C_ARBITRATION_SW_PRIORITY_HIGH    = 0x00000001u32,
DOUT_I2C_ARBITRATION_SW_PRIORITY_0_RESERVED = 0x00000002u32,
DOUT_I2C_ARBITRATION_SW_PRIORITY_1_RESERVED = 0x00000003u32,
}

/*
 * DOUT_I2C_ARBITRATION_USE_I2C_REG_REQ enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_ARBITRATION_USE_I2C_REG_REQ {
DOUT_I2C_ARBITRATION__NOT_USE_I2C_REG_REQ = 0x00000000u32,
DOUT_I2C_ARBITRATION__USE_I2C_REG_REQ    = 0x00000001u32,
}

/*
 * DOUT_I2C_CONTROL_DBG_REF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_CONTROL_DBG_REF_SEL {
DOUT_I2C_CONTROL_NORMAL_DEBUG            = 0x00000000u32,
DOUT_I2C_CONTROL_FAST_REFERENCE_DEBUG    = 0x00000001u32,
}

/*
 * DOUT_I2C_CONTROL_DDC_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_CONTROL_DDC_SELECT {
DOUT_I2C_CONTROL_SELECT_DDC1             = 0x00000000u32,
DOUT_I2C_CONTROL_SELECT_DDC2             = 0x00000001u32,
DOUT_I2C_CONTROL_SELECT_DDC3             = 0x00000002u32,
DOUT_I2C_CONTROL_SELECT_DDC4             = 0x00000003u32,
DOUT_I2C_CONTROL_SELECT_DDCVGA           = 0x00000004u32,
}

/*
 * DOUT_I2C_CONTROL_GO enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_CONTROL_GO {
DOUT_I2C_CONTROL_STOP_TRANSFER           = 0x00000000u32,
DOUT_I2C_CONTROL_START_TRANSFER          = 0x00000001u32,
}

/*
 * DOUT_I2C_CONTROL_SEND_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_CONTROL_SEND_RESET {
DOUT_I2C_CONTROL__NOT_SEND_RESET         = 0x00000000u32,
DOUT_I2C_CONTROL__SEND_RESET             = 0x00000001u32,
}

/*
 * DOUT_I2C_CONTROL_SEND_RESET_LENGTH enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_CONTROL_SEND_RESET_LENGTH {
DOUT_I2C_CONTROL__SEND_RESET_LENGTH_9    = 0x00000000u32,
DOUT_I2C_CONTROL__SEND_RESET_LENGTH_10   = 0x00000001u32,
}

/*
 * DOUT_I2C_CONTROL_SOFT_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_CONTROL_SOFT_RESET {
DOUT_I2C_CONTROL_NOT_RESET_I2C_CONTROLLER = 0x00000000u32,
DOUT_I2C_CONTROL_RESET_I2C_CONTROLLER    = 0x00000001u32,
}

/*
 * DOUT_I2C_CONTROL_SW_STATUS_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_CONTROL_SW_STATUS_RESET {
DOUT_I2C_CONTROL_NOT_RESET_SW_STATUS     = 0x00000000u32,
DOUT_I2C_CONTROL_RESET_SW_STATUS         = 0x00000001u32,
}

/*
 * DOUT_I2C_CONTROL_TRANSACTION_COUNT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_CONTROL_TRANSACTION_COUNT {
DOUT_I2C_CONTROL_TRANS0                  = 0x00000000u32,
DOUT_I2C_CONTROL_TRANS0_TRANS1           = 0x00000001u32,
DOUT_I2C_CONTROL_TRANS0_TRANS1_TRANS2    = 0x00000002u32,
DOUT_I2C_CONTROL_TRANS0_TRANS1_TRANS2_TRANS3 = 0x00000003u32,
}

/*
 * DOUT_I2C_DATA_INDEX_WRITE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_DATA_INDEX_WRITE {
DOUT_I2C_DATA__NOT_INDEX_WRITE           = 0x00000000u32,
DOUT_I2C_DATA__INDEX_WRITE               = 0x00000001u32,
}

/*
 * DOUT_I2C_DDC_SETUP_CLK_DRIVE_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_DDC_SETUP_CLK_DRIVE_EN {
DOUT_I2C_DDC_SETUP_CLK_DRIVE_BY_EXTERNAL_RESISTOR = 0x00000000u32,
DOUT_I2C_DDC_SETUP_I2C_PAD_DRIVE_SCL     = 0x00000001u32,
}

/*
 * DOUT_I2C_DDC_SETUP_DATA_DRIVE_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_DDC_SETUP_DATA_DRIVE_EN {
DOUT_I2C_DDC_SETUP_DATA_DRIVE_BY_EXTERNAL_RESISTOR = 0x00000000u32,
DOUT_I2C_DDC_SETUP_I2C_PAD_DRIVE_SDA     = 0x00000001u32,
}

/*
 * DOUT_I2C_DDC_SETUP_DATA_DRIVE_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_DDC_SETUP_DATA_DRIVE_SEL {
DOUT_I2C_DDC_SETUP_DATA_DRIVE_FOR_10MCLKS = 0x00000000u32,
DOUT_I2C_DDC_SETUP_DATA_DRIVE_FOR_20MCLKS = 0x00000001u32,
}

/*
 * DOUT_I2C_DDC_SETUP_EDID_DETECT_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_DDC_SETUP_EDID_DETECT_MODE {
DOUT_I2C_DDC_SETUP_EDID_DETECT_CONNECT   = 0x00000000u32,
DOUT_I2C_DDC_SETUP_EDID_DETECT_DISCONNECT = 0x00000001u32,
}

/*
 * DOUT_I2C_DDC_SPEED_THRESHOLD enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_DDC_SPEED_THRESHOLD {
DOUT_I2C_DDC_SPEED_THRESHOLD_BIG_THAN_ZERO = 0x00000000u32,
DOUT_I2C_DDC_SPEED_THRESHOLD_QUATER_OF_TOTAL_SAMPLE = 0x00000001u32,
DOUT_I2C_DDC_SPEED_THRESHOLD_HALF_OF_TOTAL_SAMPLE = 0x00000002u32,
DOUT_I2C_DDC_SPEED_THRESHOLD_THREE_QUATERS_OF_TOTAL_SAMPLE = 0x00000003u32,
}

/*
 * DOUT_I2C_EDID_DETECT_CTRL_SEND_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_EDID_DETECT_CTRL_SEND_RESET {
DOUT_I2C_EDID_NOT_SEND_RESET_BEFORE_EDID_READ_TRACTION = 0x00000000u32,
DOUT_I2C_EDID_SEND_RESET_BEFORE_EDID_READ_TRACTION = 0x00000001u32,
}

/*
 * DOUT_I2C_READ_REQUEST_INTERRUPT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_READ_REQUEST_INTERRUPT_TYPE {
DOUT_I2C_READ_REQUEST_INTERRUPT_TYPE__LEVEL = 0x00000000u32,
DOUT_I2C_READ_REQUEST_INTERRUPT_TYPE__PULSE = 0x00000001u32,
}

/*
 * DOUT_I2C_TRANSACTION_STOP_ON_NACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DOUT_I2C_TRANSACTION_STOP_ON_NACK {
DOUT_I2C_TRANSACTION_STOP_CURRENT_TRANS  = 0x00000000u32,
DOUT_I2C_TRANSACTION_STOP_ALL_TRANS      = 0x00000001u32,
}

/*******************************************************
 * DIO_MISC Enums
 *******************************************************/

/*
 * CLOCK_GATING_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CLOCK_GATING_EN {
CLOCK_GATING_ENABLE                      = 0x00000000u32,
CLOCK_GATING_DISABLE                     = 0x00000001u32,
}

/*
 * DAC_MUX_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DAC_MUX_SELECT {
DAC_MUX_SELECT_DACA                      = 0x00000000u32,
DAC_MUX_SELECT_DACB                      = 0x00000001u32,
}

/*
 * DIOMEM_PWR_DIS_CTRL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIOMEM_PWR_DIS_CTRL {
DIOMEM_ENABLE_MEM_PWR_CTRL               = 0x00000000u32,
DIOMEM_DISABLE_MEM_PWR_CTRL              = 0x00000001u32,
}

/*
 * DIOMEM_PWR_FORCE_CTRL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIOMEM_PWR_FORCE_CTRL {
DIOMEM_NO_FORCE_REQUEST                  = 0x00000000u32,
DIOMEM_FORCE_LIGHT_SLEEP_REQUEST         = 0x00000001u32,
DIOMEM_FORCE_DEEP_SLEEP_REQUEST          = 0x00000002u32,
DIOMEM_FORCE_SHUT_DOWN_REQUEST           = 0x00000003u32,
}

/*
 * DIOMEM_PWR_FORCE_CTRL2 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIOMEM_PWR_FORCE_CTRL2 {
DIOMEM_NO_FORCE_REQ                      = 0x00000000u32,
DIOMEM_FORCE_LIGHT_SLEEP_REQ             = 0x00000001u32,
}

/*
 * DIOMEM_PWR_SEL_CTRL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIOMEM_PWR_SEL_CTRL {
DIOMEM_DYNAMIC_SHUT_DOWN_ENABLE          = 0x00000000u32,
DIOMEM_DYNAMIC_DEEP_SLEEP_ENABLE         = 0x00000001u32,
DIOMEM_DYNAMIC_LIGHT_SLEEP_ENABLE        = 0x00000002u32,
}

/*
 * DIOMEM_PWR_SEL_CTRL2 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIOMEM_PWR_SEL_CTRL2 {
DIOMEM_DYNAMIC_DEEP_SLEEP_EN             = 0x00000000u32,
DIOMEM_DYNAMIC_LIGHT_SLEEP_EN            = 0x00000001u32,
}

/*
 * DIO_CLOCK_GATING_DISABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIO_CLOCK_GATING_DISABLE {
DIO_CLOCK_GATING_EN                      = 0x00000000u32,
DIO_CLOCK_GATING_DIS                     = 0x00000001u32,
}

/*
 * DIO_DBG_BLOCK_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIO_DBG_BLOCK_SEL {
DIO_DBG_BLOCK_SEL_DIO                    = 0x00000000u32,
DIO_DBG_BLOCK_SEL_DIGFE_A                = 0x0000000bu32,
DIO_DBG_BLOCK_SEL_DIGFE_B                = 0x0000000cu32,
DIO_DBG_BLOCK_SEL_DIGFE_C                = 0x0000000du32,
DIO_DBG_BLOCK_SEL_DIGFE_D                = 0x0000000eu32,
DIO_DBG_BLOCK_SEL_DIGA                   = 0x00000012u32,
DIO_DBG_BLOCK_SEL_DIGB                   = 0x00000013u32,
DIO_DBG_BLOCK_SEL_DIGC                   = 0x00000014u32,
DIO_DBG_BLOCK_SEL_DIGD                   = 0x00000015u32,
DIO_DBG_BLOCK_SEL_DPFE_A                 = 0x00000019u32,
DIO_DBG_BLOCK_SEL_DPFE_B                 = 0x0000001au32,
DIO_DBG_BLOCK_SEL_DPFE_C                 = 0x0000001bu32,
DIO_DBG_BLOCK_SEL_DPFE_D                 = 0x0000001cu32,
DIO_DBG_BLOCK_SEL_DPA                    = 0x00000020u32,
DIO_DBG_BLOCK_SEL_DPB                    = 0x00000021u32,
DIO_DBG_BLOCK_SEL_DPC                    = 0x00000022u32,
DIO_DBG_BLOCK_SEL_DPD                    = 0x00000023u32,
DIO_DBG_BLOCK_SEL_AUX0                   = 0x00000027u32,
DIO_DBG_BLOCK_SEL_AUX1                   = 0x00000028u32,
DIO_DBG_BLOCK_SEL_AUX2                   = 0x00000029u32,
DIO_DBG_BLOCK_SEL_AUX3                   = 0x0000002au32,
DIO_DBG_BLOCK_SEL_PERFMON_DIO            = 0x0000002du32,
DIO_DBG_BLOCK_SEL_RESERVED               = 0x0000002eu32,
}

/*
 * DIO_HDMI_RXSTATUS_TIMER_CONTROL_DIO_HDMI_RXSTATUS_TIMER_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIO_HDMI_RXSTATUS_TIMER_CONTROL_DIO_HDMI_RXSTATUS_TIMER_TYPE {
DIO_HDMI_RXSTATUS_TIMER_TYPE_LEVEL       = 0x00000000u32,
DIO_HDMI_RXSTATUS_TIMER_TYPE_PULSE       = 0x00000001u32,
}

/*
 * ENUM_DIO_DCN_ACTIVE_STATUS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DIO_DCN_ACTIVE_STATUS {
ENUM_DCN_NOT_ACTIVE                      = 0x00000000u32,
ENUM_DCN_ACTIVE                          = 0x00000001u32,
}

/*
 * GENERIC_STEREOSYNC_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GENERIC_STEREOSYNC_SEL {
GENERIC_STEREOSYNC_SEL_D1                = 0x00000000u32,
GENERIC_STEREOSYNC_SEL_D2                = 0x00000001u32,
GENERIC_STEREOSYNC_SEL_D3                = 0x00000002u32,
GENERIC_STEREOSYNC_SEL_D4                = 0x00000003u32,
GENERIC_STEREOSYNC_SEL_RESERVED          = 0x00000004u32,
}

/*
 * PM_ASSERT_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PM_ASSERT_RESET {
PM_ASSERT_RESET_0                        = 0x00000000u32,
PM_ASSERT_RESET_1                        = 0x00000001u32,
}

/*
 * SOFT_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SOFT_RESET {
SOFT_RESET_0                             = 0x00000000u32,
SOFT_RESET_1                             = 0x00000001u32,
}

/*
 * TMDS_MUX_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TMDS_MUX_SELECT {
TMDS_MUX_SELECT_B                        = 0x00000000u32,
TMDS_MUX_SELECT_G                        = 0x00000001u32,
TMDS_MUX_SELECT_R                        = 0x00000002u32,
TMDS_MUX_SELECT_RESERVED                 = 0x00000003u32,
}

/*******************************************************
 * DIG_STREAM_MAPPER Enums
 *******************************************************/

/*
 * DIG_STREAM_MAPPER_DIG_STREAM_LINK_TARGET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DIG_STREAM_MAPPER_DIG_STREAM_LINK_TARGET {
DIG_STREAM_MAPPER_LINK0                  = 0x00000000u32,
DIG_STREAM_MAPPER_LINK1                  = 0x00000001u32,
DIG_STREAM_MAPPER_LINK2                  = 0x00000002u32,
DIG_STREAM_MAPPER_LINK3                  = 0x00000003u32,
DIG_STREAM_MAPPER_LINK6                  = 0x00000004u32,
}

/*******************************************************
 * DME Enums
 *******************************************************/

/*
 * DME_MEM_POWER_STATE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DME_MEM_POWER_STATE_ENUM {
DME_MEM_POWER_STATE_ENUM_ON              = 0x00000000u32,
DME_MEM_POWER_STATE_ENUM_LS              = 0x00000001u32,
DME_MEM_POWER_STATE_ENUM_DS              = 0x00000002u32,
DME_MEM_POWER_STATE_ENUM_SD              = 0x00000003u32,
}

/*
 * DME_MEM_PWR_DIS_CTRL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DME_MEM_PWR_DIS_CTRL {
DME_MEM_ENABLE_MEM_PWR_CTRL              = 0x00000000u32,
DME_MEM_DISABLE_MEM_PWR_CTRL             = 0x00000001u32,
}

/*
 * DME_MEM_PWR_FORCE_CTRL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DME_MEM_PWR_FORCE_CTRL {
DME_MEM_NO_FORCE_REQUEST                 = 0x00000000u32,
DME_MEM_FORCE_LIGHT_SLEEP_REQUEST        = 0x00000001u32,
DME_MEM_FORCE_DEEP_SLEEP_REQUEST         = 0x00000002u32,
DME_MEM_FORCE_SHUT_DOWN_REQUEST          = 0x00000003u32,
}

/*
 * METADATA_HUBP_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum METADATA_HUBP_SEL {
METADATA_HUBP_SEL_0                      = 0x00000000u32,
METADATA_HUBP_SEL_1                      = 0x00000001u32,
METADATA_HUBP_SEL_2                      = 0x00000002u32,
METADATA_HUBP_SEL_3                      = 0x00000003u32,
METADATA_HUBP_SEL_RESERVED               = 0x00000004u32,
}

/*
 * METADATA_STREAM_TYPE_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum METADATA_STREAM_TYPE_SEL {
METADATA_STREAM_DP                       = 0x00000000u32,
METADATA_STREAM_DVE                      = 0x00000001u32,
}

/*******************************************************
 * VPG Enums
 *******************************************************/

/*
 * VPG_MEM_PWR_DIS_CTRL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VPG_MEM_PWR_DIS_CTRL {
VPG_MEM_ENABLE_MEM_PWR_CTRL              = 0x00000000u32,
VPG_MEM_DISABLE_MEM_PWR_CTRL             = 0x00000001u32,
}

/*
 * VPG_MEM_PWR_FORCE_CTRL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VPG_MEM_PWR_FORCE_CTRL {
VPG_MEM_NO_FORCE_REQ                     = 0x00000000u32,
VPG_MEM_FORCE_LIGHT_SLEEP_REQ            = 0x00000001u32,
}

/*******************************************************
 * AFMT Enums
 *******************************************************/

/*
 * AFMT_ACP_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AFMT_ACP_TYPE {
ACP_TYPE_GENERIC_AUDIO                   = 0x00000000u32,
ACP_TYPE_ICE60958_AUDIO                  = 0x00000001u32,
ACP_TYPE_DVD_AUDIO                       = 0x00000002u32,
ACP_TYPE_SUPER_AUDIO_CD                  = 0x00000003u32,
}

/*
 * AFMT_AUDIO_CRC_CONTROL_CH_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AFMT_AUDIO_CRC_CONTROL_CH_SEL {
AFMT_AUDIO_CRC_CH0_SIG                   = 0x00000000u32,
AFMT_AUDIO_CRC_CH1_SIG                   = 0x00000001u32,
AFMT_AUDIO_CRC_CH2_SIG                   = 0x00000002u32,
AFMT_AUDIO_CRC_CH3_SIG                   = 0x00000003u32,
AFMT_AUDIO_CRC_CH4_SIG                   = 0x00000004u32,
AFMT_AUDIO_CRC_CH5_SIG                   = 0x00000005u32,
AFMT_AUDIO_CRC_CH6_SIG                   = 0x00000006u32,
AFMT_AUDIO_CRC_CH7_SIG                   = 0x00000007u32,
AFMT_AUDIO_CRC_RESERVED_8                = 0x00000008u32,
AFMT_AUDIO_CRC_RESERVED_9                = 0x00000009u32,
AFMT_AUDIO_CRC_RESERVED_10               = 0x0000000au32,
AFMT_AUDIO_CRC_RESERVED_11               = 0x0000000bu32,
AFMT_AUDIO_CRC_RESERVED_12               = 0x0000000cu32,
AFMT_AUDIO_CRC_RESERVED_13               = 0x0000000du32,
AFMT_AUDIO_CRC_RESERVED_14               = 0x0000000eu32,
AFMT_AUDIO_CRC_AUDIO_SAMPLE_COUNT        = 0x0000000fu32,
}

/*
 * AFMT_AUDIO_CRC_CONTROL_CONT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AFMT_AUDIO_CRC_CONTROL_CONT {
AFMT_AUDIO_CRC_ONESHOT                   = 0x00000000u32,
AFMT_AUDIO_CRC_AUTO_RESTART              = 0x00000001u32,
}

/*
 * AFMT_AUDIO_CRC_CONTROL_SOURCE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AFMT_AUDIO_CRC_CONTROL_SOURCE {
AFMT_AUDIO_CRC_SOURCE_FROM_FIFO_INPUT    = 0x00000000u32,
AFMT_AUDIO_CRC_SOURCE_FROM_FIFO_OUTPUT   = 0x00000001u32,
}

/*
 * AFMT_AUDIO_PACKET_CONTROL2_AUDIO_LAYOUT_OVRD enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AFMT_AUDIO_PACKET_CONTROL2_AUDIO_LAYOUT_OVRD {
AFMT_AUDIO_LAYOUT_DETERMINED_BY_AZ_AUDIO_CHANNEL_STATUS = 0x00000000u32,
AFMT_AUDIO_LAYOUT_OVRD_BY_REGISTER       = 0x00000001u32,
}

/*
 * AFMT_AUDIO_PACKET_CONTROL_AUDIO_SAMPLE_SEND enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AFMT_AUDIO_PACKET_CONTROL_AUDIO_SAMPLE_SEND {
AFMT_AUDIO_PACKET_SENT_DISABLED          = 0x00000000u32,
AFMT_AUDIO_PACKET_SENT_ENABLED           = 0x00000001u32,
}

/*
 * AFMT_AUDIO_PACKET_CONTROL_RESET_FIFO_WHEN_AUDIO_DIS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AFMT_AUDIO_PACKET_CONTROL_RESET_FIFO_WHEN_AUDIO_DIS {
AFMT_NOT_RESET_AUDIO_FIFO_WHEN_AUDIO_DISABLED_RESERVED = 0x00000000u32,
AFMT_RESET_AUDIO_FIFO_WHEN_AUDIO_DISABLED = 0x00000001u32,
}

/*
 * AFMT_AUDIO_SRC_CONTROL_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AFMT_AUDIO_SRC_CONTROL_SELECT {
AFMT_AUDIO_SRC_FROM_AZ_STREAM0           = 0x00000000u32,
AFMT_AUDIO_SRC_FROM_AZ_STREAM1           = 0x00000001u32,
AFMT_AUDIO_SRC_FROM_AZ_STREAM2           = 0x00000002u32,
AFMT_AUDIO_SRC_FROM_AZ_STREAM3           = 0x00000003u32,
AFMT_AUDIO_SRC_FROM_AZ_STREAM4           = 0x00000004u32,
AFMT_AUDIO_SRC_FROM_AZ_STREAM5           = 0x00000005u32,
}

/*
 * AFMT_HDMI_AUDIO_SEND_MAX_PACKETS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AFMT_HDMI_AUDIO_SEND_MAX_PACKETS {
HDMI_NOT_SEND_MAX_AUDIO_PACKETS          = 0x00000000u32,
HDMI_SEND_MAX_AUDIO_PACKETS              = 0x00000001u32,
}

/*
 * AFMT_INFOFRAME_CONTROL0_AUDIO_INFO_SOURCE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AFMT_INFOFRAME_CONTROL0_AUDIO_INFO_SOURCE {
AFMT_INFOFRAME_SOURCE_FROM_AZALIA_BLOCK  = 0x00000000u32,
AFMT_INFOFRAME_SOURCE_FROM_AFMT_REGISTERS = 0x00000001u32,
}

/*
 * AFMT_INTERRUPT_STATUS_CHG_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AFMT_INTERRUPT_STATUS_CHG_MASK {
AFMT_INTERRUPT_DISABLE                   = 0x00000000u32,
AFMT_INTERRUPT_ENABLE                    = 0x00000001u32,
}

/*
 * AFMT_MEM_PWR_DIS_CTRL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AFMT_MEM_PWR_DIS_CTRL {
AFMT_MEM_ENABLE_MEM_PWR_CTRL             = 0x00000000u32,
AFMT_MEM_DISABLE_MEM_PWR_CTRL            = 0x00000001u32,
}

/*
 * AFMT_MEM_PWR_FORCE_CTRL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AFMT_MEM_PWR_FORCE_CTRL {
AFMT_MEM_NO_FORCE_REQUEST                = 0x00000000u32,
AFMT_MEM_FORCE_LIGHT_SLEEP_REQUEST       = 0x00000001u32,
AFMT_MEM_FORCE_DEEP_SLEEP_REQUEST        = 0x00000002u32,
AFMT_MEM_FORCE_SHUT_DOWN_REQUEST         = 0x00000003u32,
}

/*
 * AFMT_RAMP_CONTROL0_SIGN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AFMT_RAMP_CONTROL0_SIGN {
AFMT_RAMP_SIGNED                         = 0x00000000u32,
AFMT_RAMP_UNSIGNED                       = 0x00000001u32,
}

/*
 * AFMT_VBI_PACKET_CONTROL_ACP_SOURCE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AFMT_VBI_PACKET_CONTROL_ACP_SOURCE {
AFMT_ACP_SOURCE_FROM_AZALIA              = 0x00000000u32,
AFMT_ACP_SOURCE_FROM_AFMT_REGISTERS      = 0x00000001u32,
}

/*
 * AUDIO_LAYOUT_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AUDIO_LAYOUT_SELECT {
AUDIO_LAYOUT_0                           = 0x00000000u32,
AUDIO_LAYOUT_1                           = 0x00000001u32,
}

/*******************************************************
 * DCOH_TOP Enums
 *******************************************************/

/*
 * DCOH_TEST_CLOCK_MUX_SELECT_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCOH_TEST_CLOCK_MUX_SELECT_ENUM {
DCOH_TEST_CLOCK_MUX_SELECT_DISPCLK_P     = 0x00000000u32,
DCOH_TEST_CLOCK_MUX_SELECT_DISPCLK_R     = 0x00000001u32,
DCOH_TEST_CLOCK_MUX_SELECT_DISPCLK_G_AUX1 = 0x00000002u32,
DCOH_TEST_CLOCK_MUX_SELECT_DISPCLK_G_AUX2 = 0x00000003u32,
DCOH_TEST_CLOCK_MUX_SELECT_DISPCLK_G_AUX3 = 0x00000004u32,
DCOH_TEST_CLOCK_MUX_SELECT_DISPCLK_G_AUX4 = 0x00000005u32,
DCOH_TEST_CLOCK_MUX_SELECT_DISPCLK_G_AUX5 = 0x00000006u32,
DCOH_TEST_CLOCK_MUX_SELECT_DISPCLK_G_AUX6 = 0x00000007u32,
DCOH_TEST_CLOCK_MUX_SELECT_REFCLK_P      = 0x00000008u32,
DCOH_TEST_CLOCK_MUX_SELECT_REFCLK_R      = 0x00000009u32,
DCOH_TEST_CLOCK_MUX_SELECT_REFCLK_G_AUX1 = 0x0000000au32,
DCOH_TEST_CLOCK_MUX_SELECT_REFCLK_G_AUX2 = 0x0000000bu32,
DCOH_TEST_CLOCK_MUX_SELECT_REFCLK_G_AUX3 = 0x0000000cu32,
DCOH_TEST_CLOCK_MUX_SELECT_REFCLK_G_AUX4 = 0x0000000du32,
DCOH_TEST_CLOCK_MUX_SELECT_REFCLK_G_AUX5 = 0x0000000eu32,
DCOH_TEST_CLOCK_MUX_SELECT_REFCLK_G_AUX6 = 0x0000000fu32,
DCOH_TEST_CLOCK_MUX_SELECT_DPIASYMCLK0   = 0x00000010u32,
DCOH_TEST_CLOCK_MUX_SELECT_DPIASYMCLK1   = 0x00000011u32,
DCOH_TEST_CLOCK_MUX_SELECT_DPIASYMCLK2   = 0x00000012u32,
DCOH_TEST_CLOCK_MUX_SELECT_DPIASYMCLK3   = 0x00000013u32,
DCOH_TEST_CLOCK_MUX_SELECT_DPIASYMCLK4   = 0x00000014u32,
DCOH_TEST_CLOCK_MUX_SELECT_DPIASYMCLK5   = 0x00000015u32,
DCOH_TEST_CLOCK_MUX_SELECT_DPIASYMCLK6   = 0x00000016u32,
DCOH_TEST_CLOCK_MUX_SELECT_DPIASYMCLK7   = 0x00000017u32,
DCOH_TEST_CLOCK_MUX_SELECT_PHYASYMCLK    = 0x00000018u32,
DCOH_TEST_CLOCK_MUX_SELECT_PHYBSYMCLK    = 0x00000019u32,
DCOH_TEST_CLOCK_MUX_SELECT_PHYCSYMCLK    = 0x0000001au32,
DCOH_TEST_CLOCK_MUX_SELECT_PHYDSYMCLK    = 0x0000001bu32,
DCOH_TEST_CLOCK_MUX_SELECT_PHYESYMCLK    = 0x0000001cu32,
DCOH_TEST_CLOCK_MUX_SELECT_PHYFSYMCLK    = 0x0000001du32,
DCOH_TEST_CLOCK_MUX_SELECT_PHYGSYMCLK    = 0x0000001eu32,
}

/*
 * DCOH_TOP_CLOCK_GATING_DISABLE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCOH_TOP_CLOCK_GATING_DISABLE_ENUM {
DCOH_TOP_CLOCK_GATING_DISABLE_ENUM_ENABLED = 0x00000000u32,
DCOH_TOP_CLOCK_GATING_DISABLE_ENUM_DISABLED = 0x00000001u32,
}

/*
 * DCOH_TOP_ENABLE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCOH_TOP_ENABLE_ENUM {
DCOH_TOP_ENABLE_ENUM_DISABLED            = 0x00000000u32,
DCOH_TOP_ENABLE_ENUM_ENABLED             = 0x00000001u32,
}

/*******************************************************
 * PHY_MUX Enums
 *******************************************************/

/*
 * PHY_MUX_ENABLE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PHY_MUX_ENABLE_ENUM {
PHY_MUX_ENABLE_ENUM_DISABLED             = 0x00000000u32,
PHY_MUX_ENABLE_ENUM_ENABLED              = 0x00000001u32,
}

/*******************************************************
 * DP_AUX Enums
 *******************************************************/

/*
 * DP_AUX_ARB_CONTROL_ARB_PRIORITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_ARB_CONTROL_ARB_PRIORITY {
DP_AUX_ARB_CONTROL_ARB_PRIORITY__GTC_LS_SW = 0x00000000u32,
DP_AUX_ARB_CONTROL_ARB_PRIORITY__LS_GTC_SW = 0x00000001u32,
DP_AUX_ARB_CONTROL_ARB_PRIORITY__SW_LS_GTC = 0x00000002u32,
DP_AUX_ARB_CONTROL_ARB_PRIORITY__SW_GTC_LS = 0x00000003u32,
}

/*
 * DP_AUX_ARB_CONTROL_DONE_USING_AUX_REG enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_ARB_CONTROL_DONE_USING_AUX_REG {
DP_AUX_ARB_CONTROL__DONE_NOT_USING_AUX_REG = 0x00000000u32,
DP_AUX_ARB_CONTROL__DONE_USING_AUX_REG   = 0x00000001u32,
}

/*
 * DP_AUX_ARB_CONTROL_USE_AUX_REG_REQ enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_ARB_CONTROL_USE_AUX_REG_REQ {
DP_AUX_ARB_CONTROL__NOT_USE_AUX_REG_REQ  = 0x00000000u32,
DP_AUX_ARB_CONTROL__USE_AUX_REG_REQ      = 0x00000001u32,
}

/*
 * DP_AUX_ARB_STATUS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_ARB_STATUS {
DP_AUX_IDLE                              = 0x00000000u32,
DP_AUX_IN_USE_LS                         = 0x00000001u32,
DP_AUX_IN_USE_GTC                        = 0x00000002u32,
DP_AUX_IN_USE_SW                         = 0x00000003u32,
DP_AUX_IN_USE_PHYWAKE                    = 0x00000004u32,
}

/*
 * DP_AUX_CONTROL_HPD_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_CONTROL_HPD_SEL {
DP_AUX_CONTROL_HPD1_SELECTED             = 0x00000000u32,
DP_AUX_CONTROL_HPD2_SELECTED             = 0x00000001u32,
DP_AUX_CONTROL_HPD3_SELECTED             = 0x00000002u32,
DP_AUX_CONTROL_HPD4_SELECTED             = 0x00000003u32,
DP_AUX_CONTROL_NO_HPD_SELECTED           = 0x00000004u32,
}

/*
 * DP_AUX_CONTROL_TEST_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_CONTROL_TEST_MODE {
DP_AUX_CONTROL_TEST_MODE_DISABLE         = 0x00000000u32,
DP_AUX_CONTROL_TEST_MODE_ENABLE          = 0x00000001u32,
}

/*
 * DP_AUX_DEFINITE_ERR_REACHED_ACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_DEFINITE_ERR_REACHED_ACK {
ALPHA_DP_AUX_DEFINITE_ERR_REACHED_NOT_ACK = 0x00000000u32,
ALPHA_DP_AUX_DEFINITE_ERR_REACHED_ACK    = 0x00000001u32,
}

/*
 * DP_AUX_DPHY_RX_CONTROL_ALLOW_BELOW_THRESHOLD_PHASE_DETECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_DPHY_RX_CONTROL_ALLOW_BELOW_THRESHOLD_PHASE_DETECT {
DP_AUX_DPHY_RX_CONTROL__NOT_ALLOW_BELOW_THRESHOLD_PHASE_DETECT = 0x00000000u32,
DP_AUX_DPHY_RX_CONTROL__ALLOW_BELOW_THRESHOLD_PHASE_DETECT = 0x00000001u32,
}

/*
 * DP_AUX_DPHY_RX_CONTROL_ALLOW_BELOW_THRESHOLD_START enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_DPHY_RX_CONTROL_ALLOW_BELOW_THRESHOLD_START {
DP_AUX_DPHY_RX_CONTROL__NOT_ALLOW_BELOW_THRESHOLD_START = 0x00000000u32,
DP_AUX_DPHY_RX_CONTROL__ALLOW_BELOW_THRESHOLD_START = 0x00000001u32,
}

/*
 * DP_AUX_DPHY_RX_CONTROL_ALLOW_BELOW_THRESHOLD_STOP enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_DPHY_RX_CONTROL_ALLOW_BELOW_THRESHOLD_STOP {
DP_AUX_DPHY_RX_CONTROL__NOT_ALLOW_BELOW_THRESHOLD_STOP = 0x00000000u32,
DP_AUX_DPHY_RX_CONTROL__ALLOW_BELOW_THRESHOLD_STOP = 0x00000001u32,
}

/*
 * DP_AUX_DPHY_RX_CONTROL_HALF_SYM_DETECT_LEN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_DPHY_RX_CONTROL_HALF_SYM_DETECT_LEN {
DP_AUX_DPHY_RX_CONTROL_HALF_SYM_DETECT_LEN__6_EDGES = 0x00000000u32,
DP_AUX_DPHY_RX_CONTROL_HALF_SYM_DETECT_LEN__10_EDGES = 0x00000001u32,
DP_AUX_DPHY_RX_CONTROL_HALF_SYM_DETECT_LEN__18_EDGES = 0x00000002u32,
DP_AUX_DPHY_RX_CONTROL_HALF_SYM_DETECT_LEN__RESERVED = 0x00000003u32,
}

/*
 * DP_AUX_DPHY_RX_CONTROL_PHASE_DETECT_LEN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_DPHY_RX_CONTROL_PHASE_DETECT_LEN {
DP_AUX_DPHY_RX_CONTROL_PHASE_DETECT_LEN__2_HALF_SYMBOLS = 0x00000000u32,
DP_AUX_DPHY_RX_CONTROL_PHASE_DETECT_LEN__4_HALF_SYMBOLS = 0x00000001u32,
DP_AUX_DPHY_RX_CONTROL_PHASE_DETECT_LEN__6_HALF_SYMBOLS = 0x00000002u32,
DP_AUX_DPHY_RX_CONTROL_PHASE_DETECT_LEN__8_HALF_SYMBOLS = 0x00000003u32,
}

/*
 * DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW {
DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW__1TO2_PERIOD = 0x00000000u32,
DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW__1TO4_PERIOD = 0x00000001u32,
DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW__1TO8_PERIOD = 0x00000002u32,
DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW__1TO16_PERIOD = 0x00000003u32,
DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW__1TO32_PERIOD = 0x00000004u32,
DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW__1TO64_PERIOD = 0x00000005u32,
DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW__1TO128_PERIOD = 0x00000006u32,
DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW__1TO256_PERIOD = 0x00000007u32,
}

/*
 * DP_AUX_DPHY_RX_CONTROL_START_WINDOW enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_DPHY_RX_CONTROL_START_WINDOW {
DP_AUX_DPHY_RX_CONTROL_START_WINDOW__1TO2_PERIOD = 0x00000000u32,
DP_AUX_DPHY_RX_CONTROL_START_WINDOW__1TO4_PERIOD = 0x00000001u32,
DP_AUX_DPHY_RX_CONTROL_START_WINDOW__1TO8_PERIOD = 0x00000002u32,
DP_AUX_DPHY_RX_CONTROL_START_WINDOW__1TO16_PERIOD = 0x00000003u32,
DP_AUX_DPHY_RX_CONTROL_START_WINDOW__1TO32_PERIOD = 0x00000004u32,
DP_AUX_DPHY_RX_CONTROL_START_WINDOW__1TO64_PERIOD = 0x00000005u32,
DP_AUX_DPHY_RX_CONTROL_START_WINDOW__1TO128_PERIOD = 0x00000006u32,
DP_AUX_DPHY_RX_CONTROL_START_WINDOW__1TO256_PERIOD = 0x00000007u32,
}

/*
 * DP_AUX_DPHY_RX_DETECTION_THRESHOLD enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_DPHY_RX_DETECTION_THRESHOLD {
DP_AUX_DPHY_RX_DETECTION_THRESHOLD__1to2 = 0x00000000u32,
DP_AUX_DPHY_RX_DETECTION_THRESHOLD__3to4 = 0x00000001u32,
DP_AUX_DPHY_RX_DETECTION_THRESHOLD__7to8 = 0x00000002u32,
DP_AUX_DPHY_RX_DETECTION_THRESHOLD__15to16 = 0x00000003u32,
DP_AUX_DPHY_RX_DETECTION_THRESHOLD__31to32 = 0x00000004u32,
DP_AUX_DPHY_RX_DETECTION_THRESHOLD__63to64 = 0x00000005u32,
DP_AUX_DPHY_RX_DETECTION_THRESHOLD__127to128 = 0x00000006u32,
DP_AUX_DPHY_RX_DETECTION_THRESHOLD__255to256 = 0x00000007u32,
}

/*
 * DP_AUX_DPHY_TX_CONTROL_MODE_DET_CHECK_DELAY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_DPHY_TX_CONTROL_MODE_DET_CHECK_DELAY {
DP_AUX_DPHY_TX_CONTROL_MODE_DET_CHECK_DELAY__0 = 0x00000000u32,
DP_AUX_DPHY_TX_CONTROL_MODE_DET_CHECK_DELAY__16US = 0x00000001u32,
DP_AUX_DPHY_TX_CONTROL_MODE_DET_CHECK_DELAY__32US = 0x00000002u32,
DP_AUX_DPHY_TX_CONTROL_MODE_DET_CHECK_DELAY__64US = 0x00000003u32,
DP_AUX_DPHY_TX_CONTROL_MODE_DET_CHECK_DELAY__128US = 0x00000004u32,
DP_AUX_DPHY_TX_CONTROL_MODE_DET_CHECK_DELAY__256US = 0x00000005u32,
}

/*
 * DP_AUX_DPHY_TX_REF_CONTROL_TX_RATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_DPHY_TX_REF_CONTROL_TX_RATE {
DP_AUX_DPHY_TX_REF_CONTROL_TX_RATE__1MHZ = 0x00000000u32,
DP_AUX_DPHY_TX_REF_CONTROL_TX_RATE__2MHZ = 0x00000001u32,
DP_AUX_DPHY_TX_REF_CONTROL_TX_RATE__4MHZ = 0x00000002u32,
DP_AUX_DPHY_TX_REF_CONTROL_TX_RATE__8MHZ = 0x00000003u32,
}

/*
 * DP_AUX_DPHY_TX_REF_CONTROL_TX_REF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_DPHY_TX_REF_CONTROL_TX_REF_SEL {
DP_AUX_DPHY_TX_REF_CONTROL_TX_REF_SEL__DIVIDED_SYM_CLK = 0x00000000u32,
DP_AUX_DPHY_TX_REF_CONTROL_TX_REF_SEL__FROM_DCCG_MICROSECOND_REF = 0x00000001u32,
}

/*
 * DP_AUX_ERR_OCCURRED_ACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_ERR_OCCURRED_ACK {
DP_AUX_ERR_OCCURRED__NOT_ACK             = 0x00000000u32,
DP_AUX_ERR_OCCURRED__ACK                 = 0x00000001u32,
}

/*
 * DP_AUX_GTC_SYNC_CONTROL_GTC_SYNC_BLOCK_REQ enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_GTC_SYNC_CONTROL_GTC_SYNC_BLOCK_REQ {
DP_AUX_GTC_SYNC_CONTROL_GTC_SYNC_ALLOW_REQ_FROM_OTHER_AUX = 0x00000000u32,
DP_AUX_GTC_SYNC_CONTROL_GTC_SYNC_BLOCK_REQ_FROM_OTHER_AUX = 0x00000001u32,
}

/*
 * DP_AUX_GTC_SYNC_CONTROL_INTERVAL_RESET_WINDOW enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_GTC_SYNC_CONTROL_INTERVAL_RESET_WINDOW {
DP_AUX_GTC_SYNC_CONTROL_INTERVAL_RESET_WINDOW__300US = 0x00000000u32,
DP_AUX_GTC_SYNC_CONTROL_INTERVAL_RESET_WINDOW__400US = 0x00000001u32,
DP_AUX_GTC_SYNC_CONTROL_INTERVAL_RESET_WINDOW__500US = 0x00000002u32,
DP_AUX_GTC_SYNC_CONTROL_INTERVAL_RESET_WINDOW__600US = 0x00000003u32,
}

/*
 * DP_AUX_GTC_SYNC_CONTROL_OFFSET_CALC_MAX_ATTEMPT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_GTC_SYNC_CONTROL_OFFSET_CALC_MAX_ATTEMPT {
DP_AUX_GTC_SYNC_CONTROL_OFFSET_CALC_MAX_ATTEMPT__4_ATTAMPS = 0x00000000u32,
DP_AUX_GTC_SYNC_CONTROL_OFFSET_CALC_MAX_ATTEMPT__8_ATTAMPS = 0x00000001u32,
DP_AUX_GTC_SYNC_CONTROL_OFFSET_CALC_MAX_ATTEMPT__16_ATTAMPS = 0x00000002u32,
DP_AUX_GTC_SYNC_CONTROL_OFFSET_CALC_MAX_ATTEMPT__RESERVED = 0x00000003u32,
}

/*
 * DP_AUX_GTC_SYNC_ERROR_CONTROL_LOCK_ACQ_TIMEOUT_LEN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_GTC_SYNC_ERROR_CONTROL_LOCK_ACQ_TIMEOUT_LEN {
DP_AUX_GTC_SYNC_ERROR_CONTROL_LOCK_ACQ_TIMEOUT_LEN__0 = 0x00000000u32,
DP_AUX_GTC_SYNC_ERROR_CONTROL_LOCK_ACQ_TIMEOUT_LEN__64 = 0x00000001u32,
DP_AUX_GTC_SYNC_ERROR_CONTROL_LOCK_ACQ_TIMEOUT_LEN__128 = 0x00000002u32,
DP_AUX_GTC_SYNC_ERROR_CONTROL_LOCK_ACQ_TIMEOUT_LEN__256 = 0x00000003u32,
}

/*
 * DP_AUX_INT_ACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_INT_ACK {
DP_AUX_INT__NOT_ACK                      = 0x00000000u32,
DP_AUX_INT__ACK                          = 0x00000001u32,
}

/*
 * DP_AUX_LS_UPDATE_ACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_LS_UPDATE_ACK {
DP_AUX_INT_LS_UPDATE_NOT_ACK             = 0x00000000u32,
DP_AUX_INT_LS_UPDATE_ACK                 = 0x00000001u32,
}

/*
 * DP_AUX_PHY_WAKE_PRIORITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_PHY_WAKE_PRIORITY {
DP_AUX_PHY_WAKE_HIGH_PRIORITY            = 0x00000000u32,
DP_AUX_PHY_WAKE_LOW_PRIORITY             = 0x00000001u32,
}

/*
 * DP_AUX_POTENTIAL_ERR_REACHED_ACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_POTENTIAL_ERR_REACHED_ACK {
DP_AUX_POTENTIAL_ERR_REACHED__NOT_ACK    = 0x00000000u32,
DP_AUX_POTENTIAL_ERR_REACHED__ACK        = 0x00000001u32,
}

/*
 * DP_AUX_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_RESET {
DP_AUX_RESET_DEASSERTED                  = 0x00000000u32,
DP_AUX_RESET_ASSERTED                    = 0x00000001u32,
}

/*
 * DP_AUX_RESET_DONE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_RESET_DONE {
DP_AUX_RESET_SEQUENCE_NOT_DONE           = 0x00000000u32,
DP_AUX_RESET_SEQUENCE_DONE               = 0x00000001u32,
}

/*
 * DP_AUX_RX_TIMEOUT_LEN_MUL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_RX_TIMEOUT_LEN_MUL {
DP_AUX_RX_TIMEOUT_LEN_NO_MUL             = 0x00000000u32,
DP_AUX_RX_TIMEOUT_LEN_MUL_2              = 0x00000001u32,
DP_AUX_RX_TIMEOUT_LEN_MUL_4              = 0x00000002u32,
DP_AUX_RX_TIMEOUT_LEN_MUL_8              = 0x00000003u32,
}

/*
 * DP_AUX_SW_CONTROL_LS_READ_TRIG enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_SW_CONTROL_LS_READ_TRIG {
DP_AUX_SW_CONTROL_LS_READ__NOT_TRIG      = 0x00000000u32,
DP_AUX_SW_CONTROL_LS_READ__TRIG          = 0x00000001u32,
}

/*
 * DP_AUX_SW_CONTROL_SW_GO enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_SW_CONTROL_SW_GO {
DP_AUX_SW_CONTROL_SW__NOT_GO             = 0x00000000u32,
DP_AUX_SW_CONTROL_SW__GO                 = 0x00000001u32,
}

/*
 * DP_AUX_TX_PRECHARGE_LEN_MUL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_AUX_TX_PRECHARGE_LEN_MUL {
DP_AUX_TX_PRECHARGE_LEN_NO_MUL           = 0x00000000u32,
DP_AUX_TX_PRECHARGE_LEN_MUL_2            = 0x00000001u32,
DP_AUX_TX_PRECHARGE_LEN_MUL_4            = 0x00000002u32,
DP_AUX_TX_PRECHARGE_LEN_MUL_8            = 0x00000003u32,
}

/*******************************************************
 * HPD Enums
 *******************************************************/

/*
 * HPD_INT_CONTROL_ACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HPD_INT_CONTROL_ACK {
HPD_INT_CONTROL_ACK_0                    = 0x00000000u32,
HPD_INT_CONTROL_ACK_1                    = 0x00000001u32,
}

/*
 * HPD_INT_CONTROL_POLARITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HPD_INT_CONTROL_POLARITY {
HPD_INT_CONTROL_GEN_INT_ON_DISCON        = 0x00000000u32,
HPD_INT_CONTROL_GEN_INT_ON_CON           = 0x00000001u32,
}

/*
 * HPD_INT_CONTROL_RX_INT_ACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HPD_INT_CONTROL_RX_INT_ACK {
HPD_INT_CONTROL_RX_INT_ACK_0             = 0x00000000u32,
HPD_INT_CONTROL_RX_INT_ACK_1             = 0x00000001u32,
}

/*******************************************************
 * HPO_TOP Enums
 *******************************************************/

/*
 * HPO_TOP_CLOCK_GATING_DISABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HPO_TOP_CLOCK_GATING_DISABLE {
HPO_TOP_CLOCK_GATING_EN                  = 0x00000000u32,
HPO_TOP_CLOCK_GATING_DIS                 = 0x00000001u32,
}

/*
 * HPO_TOP_TEST_CLK_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum HPO_TOP_TEST_CLK_SEL {
HPO_TOP_PERMANENT_DISPCLK                = 0x00000000u32,
HPO_TOP_REGISTER_GATED_DISPCLK           = 0x00000001u32,
HPO_TOP_PERMANENT_SOCCLK                 = 0x00000002u32,
HPO_TOP_TEST_CLOCK_RESERVED              = 0x00000003u32,
HPO_TOP_PERMANENT_HDMISTREAMCLK0         = 0x00000004u32,
HPO_TOP_FEATURE_GATED_HDMISTREAMCLK0     = 0x00000005u32,
HPO_TOP_REGISTER_GATED_HDMISTREAMCLK0    = 0x00000006u32,
HPO_TOP_FEATURE_GATED_DISPCLK_IN_HDMISTREAMENC0 = 0x00000007u32,
HPO_TOP_FEATURE_GATED_SOCCLK_IN_HDMISTREAMENC0 = 0x00000008u32,
HPO_TOP_PERMANENT_HDMICHARCLK0           = 0x00000009u32,
HPO_TOP_FEATURE_GATED_HDMICHARCLK0       = 0x0000000au32,
HPO_TOP_REGISTER_GATED_HDMICHARCLK0      = 0x0000000bu32,
}

/*******************************************************
 * DP_STREAM_MAPPER Enums
 *******************************************************/

/*
 * DP_STREAM_MAPPER_DP_STREAM_LINK_TARGET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_STREAM_MAPPER_DP_STREAM_LINK_TARGET {
DP_STREAM_MAPPER_LINK0                   = 0x00000000u32,
DP_STREAM_MAPPER_LINK1                   = 0x00000001u32,
DP_STREAM_MAPPER_LINK2                   = 0x00000002u32,
DP_STREAM_MAPPER_LINK3                   = 0x00000003u32,
DP_STREAM_MAPPER_RESERVED                = 0x00000004u32,
}

/*******************************************************
 * DP_STREAM_ENC Enums
 *******************************************************/

/*
 * DP_STREAM_ENC_OVERFLOW_UNDERFLOW_ERROR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_STREAM_ENC_OVERFLOW_UNDERFLOW_ERROR {
DP_STREAM_ENC_NO_ERROR_OCCURRED          = 0x00000000u32,
DP_STREAM_ENC_UNDERFLOW_OCCURRED         = 0x00000001u32,
DP_STREAM_ENC_OVERFLOW_OCCURRED          = 0x00000002u32,
}

/*
 * DP_STREAM_ENC_OVERWRITE_LEVEL_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_STREAM_ENC_OVERWRITE_LEVEL_SELECT {
DP_STREAM_ENC_HARDWARE                   = 0x00000000u32,
DP_STREAM_ENC_PROGRAMMABLE               = 0x00000001u32,
}

/*
 * DP_STREAM_ENC_READ_CLOCK_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_STREAM_ENC_READ_CLOCK_CONTROL {
DP_STREAM_ENC_DCCG                       = 0x00000000u32,
DP_STREAM_ENC_DISPLAY_PIPE               = 0x00000001u32,
}

/*
 * DP_STREAM_ENC_RESET_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_STREAM_ENC_RESET_CONTROL {
DP_STREAM_ENC_NOT_RESET                  = 0x00000000u32,
DP_STREAM_ENC_RESET                      = 0x00000001u32,
}

/*
 * DP_STREAM_ENC_STREAM_ACTIVE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DP_STREAM_ENC_STREAM_ACTIVE {
DP_STREAM_ENC_VIDEO_STREAM_NOT_ACTIVE    = 0x00000000u32,
DP_STREAM_ENC_VIDEO_STREAM_ACTIVE        = 0x00000001u32,
}

/*******************************************************
 * DP_SYM32_ENC Enums
 *******************************************************/

/*
 * ENUM_DP_SYM32_ENC_AUDIO_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_AUDIO_MUTE {
DP_SYM32_ENC_SDP_AUDIO_MUTE_NOT_FORCED   = 0x00000000u32,
DP_SYM32_ENC_SDP_AUDIO_MUTE_FORCED       = 0x00000001u32,
}

/*
 * ENUM_DP_SYM32_ENC_CONTINUOUS_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_CONTINUOUS_MODE {
DP_SYM32_ENC_ONE_SHOT_MODE               = 0x00000000u32,
DP_SYM32_ENC_CONTINUOUS_MODE             = 0x00000001u32,
}

/*
 * ENUM_DP_SYM32_ENC_CRC_VALID enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_CRC_VALID {
DP_SYM32_ENC_CRC_NOT_VALID               = 0x00000000u32,
DP_SYM32_ENC_CRC_VALID                   = 0x00000001u32,
}

/*
 * ENUM_DP_SYM32_ENC_DP_COMPONENT_DEPTH enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_DP_COMPONENT_DEPTH {
DP_SYM32_ENC_COMPONENT_DEPTH_6BPC        = 0x00000000u32,
DP_SYM32_ENC_COMPONENT_DEPTH_8BPC        = 0x00000001u32,
DP_SYM32_ENC_COMPONENT_DEPTH_10BPC       = 0x00000002u32,
DP_SYM32_ENC_COMPONENT_DEPTH_12BPC       = 0x00000003u32,
}

/*
 * ENUM_DP_SYM32_ENC_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_ENABLE {
DP_SYM32_ENC_DISABLE                     = 0x00000000u32,
DP_SYM32_ENC_ENABLE                      = 0x00000001u32,
}

/*
 * ENUM_DP_SYM32_ENC_GSP_DEADLINE_MISSED enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_GSP_DEADLINE_MISSED {
DP_SYM32_ENC_GSP_DEADLINE_NOT_MISSED     = 0x00000000u32,
DP_SYM32_ENC_GSP_DEADLINE_MISSED         = 0x00000001u32,
}

/*
 * ENUM_DP_SYM32_ENC_GSP_ONE_SHOT_TRIGGER_POSITION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_GSP_ONE_SHOT_TRIGGER_POSITION {
DP_SYM32_ENC_GSP_SEND_AT_LINE_NUMBER     = 0x00000000u32,
DP_SYM32_ENC_GSP_SEND_AT_EARLIEST_TIME   = 0x00000001u32,
}

/*
 * ENUM_DP_SYM32_ENC_GSP_PAYLOAD_SIZE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_GSP_PAYLOAD_SIZE {
DP_SYM32_ENC_GSP_PAYLOAD_SIZE_32         = 0x00000000u32,
DP_SYM32_ENC_GSP_PAYLOAD_SIZE_RESERVED0  = 0x00000001u32,
DP_SYM32_ENC_GSP_PAYLOAD_SIZE_RESERVED1  = 0x00000002u32,
DP_SYM32_ENC_GSP_PAYLOAD_SIZE_128        = 0x00000003u32,
}

/*
 * ENUM_DP_SYM32_ENC_GSP_TRIGGER_PENDING enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_GSP_TRIGGER_PENDING {
DP_SYM32_ENC_GSP_TRIGGER_NOT_PENDING     = 0x00000000u32,
DP_SYM32_ENC_GSP_TRIGGER_PENDING         = 0x00000001u32,
}

/*
 * ENUM_DP_SYM32_ENC_MEM_PWR_FORCE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_MEM_PWR_FORCE_ENUM {
DP_SYM32_ENC_MEM_PWR_NO_FORCE_REQUEST    = 0x00000000u32,
DP_SYM32_ENC_MEM_PWR_FORCE_LIGHT_SLEEP_REQUEST = 0x00000001u32,
DP_SYM32_ENC_MEM_PWR_FORCE_DEEP_SLEEP_REQUEST = 0x00000002u32,
DP_SYM32_ENC_MEM_PWR_FORCE_SHUT_DOWN_REQUEST = 0x00000003u32,
}

/*
 * ENUM_DP_SYM32_ENC_OVERFLOW_STATUS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_OVERFLOW_STATUS {
DP_SYM32_ENC_NO_OVERFLOW_OCCURRED        = 0x00000000u32,
DP_SYM32_ENC_OVERFLOW_OCCURRED           = 0x00000001u32,
}

/*
 * ENUM_DP_SYM32_ENC_PENDING enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_PENDING {
DP_SYM32_ENC_NOT_PENDING                 = 0x00000000u32,
DP_SYM32_ENC_PENDING                     = 0x00000001u32,
}

/*
 * ENUM_DP_SYM32_ENC_PIXEL_ENCODING enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_PIXEL_ENCODING {
DP_SYM32_ENC_PIXEL_ENCODING_RGB_YCBCR444 = 0x00000000u32,
DP_SYM32_ENC_PIXEL_ENCODING_YCBCR422     = 0x00000001u32,
DP_SYM32_ENC_PIXEL_ENCODING_YCBCR420     = 0x00000002u32,
DP_SYM32_ENC_PIXEL_ENCODING_Y_ONLY       = 0x00000003u32,
}

/*
 * ENUM_DP_SYM32_ENC_PIXEL_ENCODING_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_PIXEL_ENCODING_TYPE {
DP_SYM32_ENC_UNCOMPRESSED_FORMAT         = 0x00000000u32,
DP_SYM32_ENC_COMPRESSED_FORMAT           = 0x00000001u32,
}

/*
 * ENUM_DP_SYM32_ENC_POWER_STATE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_POWER_STATE_ENUM {
DP_SYM32_ENC_POWER_STATE_ENUM_ON         = 0x00000000u32,
DP_SYM32_ENC_POWER_STATE_ENUM_LS         = 0x00000001u32,
DP_SYM32_ENC_POWER_STATE_ENUM_DS         = 0x00000002u32,
DP_SYM32_ENC_POWER_STATE_ENUM_SD         = 0x00000003u32,
}

/*
 * ENUM_DP_SYM32_ENC_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_RESET {
DP_SYM32_ENC_NOT_RESET                   = 0x00000000u32,
DP_SYM32_ENC_RESET                       = 0x00000001u32,
}

/*
 * ENUM_DP_SYM32_ENC_SDP_PRIORITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_SDP_PRIORITY {
DP_SYM32_ENC_SDP_LOW_PRIORITY            = 0x00000000u32,
DP_SYM32_ENC_SDP_HIGH_PRIORITY           = 0x00000001u32,
}

/*
 * ENUM_DP_SYM32_ENC_SOF_REFERENCE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_SOF_REFERENCE {
DP_SYM32_ENC_DP_SOF                      = 0x00000000u32,
DP_SYM32_ENC_OTG_SOF                     = 0x00000001u32,
}

/*
 * ENUM_DP_SYM32_ENC_VID_STREAM_DEFER enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_SYM32_ENC_VID_STREAM_DEFER {
DP_SYM32_ENC_VID_STREAM_NO_DEFER         = 0x00000000u32,
DP_SYM32_ENC_VID_STREAM_DEFER_TO_HBLANK  = 0x00000001u32,
DP_SYM32_ENC_VID_STREAM_DEFER_TO_VBLANK  = 0x00000002u32,
}

/*******************************************************
 * DP_DPHY_SYM32 Enums
 *******************************************************/

/*
 * ENUM_DP_DPHY_SYM32_CRC_END_EVENT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_CRC_END_EVENT {
DP_DPHY_SYM32_CRC_END_LLCP               = 0x00000000u32,
DP_DPHY_SYM32_CRC_END_PS_ONLY            = 0x00000001u32,
DP_DPHY_SYM32_CRC_END_PS_LT_SR           = 0x00000002u32,
DP_DPHY_SYM32_CRC_END_PS_ANY             = 0x00000003u32,
}

/*
 * ENUM_DP_DPHY_SYM32_CRC_START_EVENT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_CRC_START_EVENT {
DP_DPHY_SYM32_CRC_START_LLCP             = 0x00000000u32,
DP_DPHY_SYM32_CRC_START_PS_ONLY          = 0x00000001u32,
DP_DPHY_SYM32_CRC_START_PS_LT_SR         = 0x00000002u32,
DP_DPHY_SYM32_CRC_START_PS_POST_LT_SR    = 0x00000003u32,
DP_DPHY_SYM32_CRC_START_TP_START         = 0x00000004u32,
}

/*
 * ENUM_DP_DPHY_SYM32_CRC_TAP_SOURCE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_CRC_TAP_SOURCE {
DP_DPHY_SYM32_CRC_TAP_SOURCE_SCHEDULER   = 0x00000000u32,
DP_DPHY_SYM32_CRC_TAP_SOURCE_SYMBOL_HANDLER = 0x00000001u32,
DP_DPHY_SYM32_CRC_TAP_SOURCE_TP_GEN_MUX  = 0x00000002u32,
}

/*
 * ENUM_DP_DPHY_SYM32_CRC_USE_NUM_SYMBOLS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_CRC_USE_NUM_SYMBOLS {
DP_DPHY_SYM32_CRC_USE_END_EVENT          = 0x00000000u32,
DP_DPHY_SYM32_CRC_USE_NUM_SYMBOLS        = 0x00000001u32,
}

/*
 * ENUM_DP_DPHY_SYM32_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_ENABLE {
DP_DPHY_SYM32_DISABLE                    = 0x00000000u32,
DP_DPHY_SYM32_ENABLE                     = 0x00000001u32,
}

/*
 * ENUM_DP_DPHY_SYM32_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_MODE {
DP_DPHY_SYM32_LT_TPS1                    = 0x00000000u32,
DP_DPHY_SYM32_LT_TPS2                    = 0x00000001u32,
DP_DPHY_SYM32_ACTIVE                     = 0x00000002u32,
DP_DPHY_SYM32_TEST                       = 0x00000003u32,
}

/*
 * ENUM_DP_DPHY_SYM32_NUM_LANES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_NUM_LANES {
DP_DPHY_SYM32_1LANE                      = 0x00000000u32,
DP_DPHY_SYM32_2LANE                      = 0x00000001u32,
DP_DPHY_SYM32_RESERVED                   = 0x00000002u32,
DP_DPHY_SYM32_4LANE                      = 0x00000003u32,
}

/*
 * ENUM_DP_DPHY_SYM32_OUTPUT_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_OUTPUT_MODE {
DP_DPHY_SYM32_OUTPUT_PHY                 = 0x00000000u32,
DP_DPHY_SYM32_OUTPUT_DPIA                = 0x00000001u32,
}

/*
 * ENUM_DP_DPHY_SYM32_RATE_UPDATE_PENDING enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_RATE_UPDATE_PENDING {
DP_DPHY_SYM32_NO_RATE_UPDATE_PENDING     = 0x00000000u32,
DP_DPHY_SYM32_RATE_UPDATE_PENDING        = 0x00000001u32,
}

/*
 * ENUM_DP_DPHY_SYM32_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_RESET {
DP_DPHY_SYM32_NOT_RESET                  = 0x00000000u32,
DP_DPHY_SYM32_RESET                      = 0x00000001u32,
}

/*
 * ENUM_DP_DPHY_SYM32_RESET_STATUS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_RESET_STATUS {
DP_DPHY_SYM32_RESET_STATUS_DEASSERTED    = 0x00000000u32,
DP_DPHY_SYM32_RESET_STATUS_ASSERTED      = 0x00000001u32,
}

/*
 * ENUM_DP_DPHY_SYM32_SAT_UPDATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_SAT_UPDATE {
DP_DPHY_SYM32_SAT_NO_UPDATE              = 0x00000000u32,
DP_DPHY_SYM32_SAT_TRIGGER_UPDATE         = 0x00000001u32,
DP_DPHY_SYM32_SAT_NOTRIGGER_UPDATE       = 0x00000002u32,
}

/*
 * ENUM_DP_DPHY_SYM32_SAT_UPDATE_PENDING enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_SAT_UPDATE_PENDING {
DP_DPHY_SYM32_SAT_NO_UPDATE_PENDING      = 0x00000000u32,
DP_DPHY_SYM32_SAT_TRIGGER_UPDATE_PENDING = 0x00000001u32,
DP_DPHY_SYM32_SAT_NOTRIGGER_UPDATE_PENDING = 0x00000002u32,
}

/*
 * ENUM_DP_DPHY_SYM32_SCHEDULER_STATUS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_SCHEDULER_STATUS {
DP_DPHY_SYM32_SCHEDULER_OFF              = 0x00000000u32,
DP_DPHY_SYM32_SCHEDULER_ASLEEP           = 0x00000001u32,
DP_DPHY_SYM32_SCHEDULER_AWAKE            = 0x00000002u32,
}

/*
 * ENUM_DP_DPHY_SYM32_STATUS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_STATUS {
DP_DPHY_SYM32_STATUS_IDLE                = 0x00000000u32,
DP_DPHY_SYM32_STATUS_ENABLED             = 0x00000001u32,
}

/*
 * ENUM_DP_DPHY_SYM32_STREAM_OVR_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_STREAM_OVR_ENABLE {
DP_DPHY_SYM32_STREAM_OVR_NONE            = 0x00000000u32,
DP_DPHY_SYM32_STREAM_OVR_REPLACE         = 0x00000001u32,
DP_DPHY_SYM32_STREAM_OVR_ALWAYS          = 0x00000002u32,
}

/*
 * ENUM_DP_DPHY_SYM32_STREAM_OVR_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_STREAM_OVR_TYPE {
DP_DPHY_SYM32_STREAM_OVR_TYPE_DATA       = 0x00000000u32,
DP_DPHY_SYM32_STREAM_OVR_TYPE_CONTROL    = 0x00000001u32,
}

/*
 * ENUM_DP_DPHY_SYM32_TP_PRBS_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_TP_PRBS_SEL {
DP_DPHY_SYM32_TP_PRBS_SEL_PRBS7          = 0x00000000u32,
DP_DPHY_SYM32_TP_PRBS_SEL_PRBS9          = 0x00000001u32,
DP_DPHY_SYM32_TP_PRBS_SEL_PRBS11         = 0x00000002u32,
DP_DPHY_SYM32_TP_PRBS_SEL_PRBS15         = 0x00000003u32,
DP_DPHY_SYM32_TP_PRBS_SEL_PRBS23         = 0x00000004u32,
DP_DPHY_SYM32_TP_PRBS_SEL_PRBS31         = 0x00000005u32,
}

/*
 * ENUM_DP_DPHY_SYM32_TP_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENUM_DP_DPHY_SYM32_TP_SELECT {
DP_DPHY_SYM32_TP_SELECT_TPS1             = 0x00000000u32,
DP_DPHY_SYM32_TP_SELECT_TPS2             = 0x00000001u32,
DP_DPHY_SYM32_TP_SELECT_PRBS             = 0x00000002u32,
DP_DPHY_SYM32_TP_SELECT_CUSTOM           = 0x00000003u32,
DP_DPHY_SYM32_TP_SELECT_SQUARE           = 0x00000004u32,
}

/*******************************************************
 * APG Enums
 *******************************************************/

/*
 * APG_AUDIO_CRC_CONTROL_CH_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum APG_AUDIO_CRC_CONTROL_CH_SEL {
APG_AUDIO_CRC_CH0_SIG                    = 0x00000000u32,
APG_AUDIO_CRC_CH1_SIG                    = 0x00000001u32,
APG_AUDIO_CRC_CH2_SIG                    = 0x00000002u32,
APG_AUDIO_CRC_CH3_SIG                    = 0x00000003u32,
APG_AUDIO_CRC_CH4_SIG                    = 0x00000004u32,
APG_AUDIO_CRC_CH5_SIG                    = 0x00000005u32,
APG_AUDIO_CRC_CH6_SIG                    = 0x00000006u32,
APG_AUDIO_CRC_CH7_SIG                    = 0x00000007u32,
APG_AUDIO_CRC_RESERVED_8                 = 0x00000008u32,
APG_AUDIO_CRC_RESERVED_9                 = 0x00000009u32,
APG_AUDIO_CRC_RESERVED_10                = 0x0000000au32,
APG_AUDIO_CRC_RESERVED_11                = 0x0000000bu32,
APG_AUDIO_CRC_RESERVED_12                = 0x0000000cu32,
APG_AUDIO_CRC_RESERVED_13                = 0x0000000du32,
APG_AUDIO_CRC_RESERVED_14                = 0x0000000eu32,
APG_AUDIO_CRC_RESERVED_15                = 0x0000000fu32,
}

/*
 * APG_AUDIO_CRC_CONTROL_CONT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum APG_AUDIO_CRC_CONTROL_CONT {
APG_AUDIO_CRC_ONESHOT                    = 0x00000000u32,
APG_AUDIO_CRC_CONTINUOUS                 = 0x00000001u32,
}

/*
 * APG_DBG_ACP_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum APG_DBG_ACP_TYPE {
APG_ACP_TYPE_GENERIC_AUDIO               = 0x00000000u32,
APG_ACP_TYPE_ICE60958_AUDIO              = 0x00000001u32,
APG_ACP_TYPE_DVD_AUDIO                   = 0x00000002u32,
APG_ACP_TYPE_SUPER_AUDIO_CD              = 0x00000003u32,
}

/*
 * APG_DBG_AUDIO_DTO_BASE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum APG_DBG_AUDIO_DTO_BASE {
BASE_RATE_48KHZ                          = 0x00000000u32,
BASE_RATE_44P1KHZ                        = 0x00000001u32,
}

/*
 * APG_DBG_AUDIO_DTO_DIV enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum APG_DBG_AUDIO_DTO_DIV {
DIVISOR_BY1                              = 0x00000000u32,
DIVISOR_BY2_RESERVED                     = 0x00000001u32,
DIVISOR_BY3                              = 0x00000002u32,
DIVISOR_BY4_RESERVED                     = 0x00000003u32,
DIVISOR_BY5_RESERVED                     = 0x00000004u32,
DIVISOR_BY6_RESERVED                     = 0x00000005u32,
DIVISOR_BY7_RESERVED                     = 0x00000006u32,
DIVISOR_BY8_RESERVED                     = 0x00000007u32,
}

/*
 * APG_DBG_AUDIO_DTO_MULTI enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum APG_DBG_AUDIO_DTO_MULTI {
MULTIPLE_BY1                             = 0x00000000u32,
MULTIPLE_BY2                             = 0x00000001u32,
MULTIPLE_BY3_RESERVED                    = 0x00000002u32,
MULTIPLE_BY4                             = 0x00000003u32,
MULTIPLE_RESERVED                        = 0x00000004u32,
}

/*
 * APG_DBG_MUX_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum APG_DBG_MUX_SEL {
APG_FUNCTIONAL_MODE                      = 0x00000000u32,
APG_DEBUG_AUDIO_MODE                     = 0x00000001u32,
}

/*
 * APG_DP_ASP_CHANNEL_COUNT_OVERRIDE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum APG_DP_ASP_CHANNEL_COUNT_OVERRIDE {
APG_DP_ASP_CHANNEL_COUNT_FROM_AZ         = 0x00000000u32,
APG_DP_ASP_CHANNEL_COUNT_OVERRIDE_ENABLED = 0x00000001u32,
}

/*
 * APG_MEM_POWER_STATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum APG_MEM_POWER_STATE {
APG_MEM_POWER_STATE_ON                   = 0x00000000u32,
APG_MEM_POWER_STATE_LS                   = 0x00000001u32,
APG_MEM_POWER_STATE_DS                   = 0x00000002u32,
APG_MEM_POWER_STATE_SD                   = 0x00000003u32,
}

/*
 * APG_MEM_PWR_DIS_CTRL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum APG_MEM_PWR_DIS_CTRL {
APG_MEM_ENABLE_MEM_PWR_CTRL              = 0x00000000u32,
APG_MEM_DISABLE_MEM_PWR_CTRL             = 0x00000001u32,
}

/*
 * APG_MEM_PWR_FORCE_CTRL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum APG_MEM_PWR_FORCE_CTRL {
APG_MEM_NO_FORCE_REQUEST                 = 0x00000000u32,
APG_MEM_FORCE_LIGHT_SLEEP_REQUEST        = 0x00000001u32,
APG_MEM_FORCE_DEEP_SLEEP_REQUEST         = 0x00000002u32,
APG_MEM_FORCE_SHUT_DOWN_REQUEST          = 0x00000003u32,
}

/*
 * APG_PACKET_CONTROL_ACP_SOURCE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum APG_PACKET_CONTROL_ACP_SOURCE {
APG_ACP_SOURCE_NO_OVERRIDE               = 0x00000000u32,
APG_ACP_OVERRIDE                         = 0x00000001u32,
}

/*
 * APG_PACKET_CONTROL_AUDIO_INFO_SOURCE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum APG_PACKET_CONTROL_AUDIO_INFO_SOURCE {
APG_INFOFRAME_SOURCE_NO_OVERRIDE         = 0x00000000u32,
APG_INFOFRAME_SOURCE_FROM_APG_REGISTERS  = 0x00000001u32,
}

/*
 * APG_RAMP_CONTROL_SIGN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum APG_RAMP_CONTROL_SIGN {
APG_RAMP_SIGNED                          = 0x00000000u32,
APG_RAMP_UNSIGNED                        = 0x00000001u32,
}

/*******************************************************
 * DCIO Enums
 *******************************************************/

/*
 * DCIO_BL_PWM_GRP1_FRAME_START_DISP_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_BL_PWM_GRP1_FRAME_START_DISP_SEL {
DCIO_BL_PWM_GRP1_FRAME_START_DISP_SEL_CONTROLLER1 = 0x00000000u32,
DCIO_BL_PWM_GRP1_FRAME_START_DISP_SEL_CONTROLLER2 = 0x00000001u32,
DCIO_BL_PWM_GRP1_FRAME_START_DISP_SEL_CONTROLLER3 = 0x00000002u32,
DCIO_BL_PWM_GRP1_FRAME_START_DISP_SEL_CONTROLLER4 = 0x00000003u32,
DCIO_BL_PWM_GRP1_FRAME_START_DISP_SEL_CONTROLLER5 = 0x00000004u32,
DCIO_BL_PWM_GRP1_FRAME_START_DISP_SEL_CONTROLLER6 = 0x00000005u32,
}

/*
 * DCIO_CLOCK_CNTL_DCIO_TEST_CLK_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_CLOCK_CNTL_DCIO_TEST_CLK_SEL {
DCIO_TEST_CLK_SEL_DISPCLK                = 0x00000000u32,
DCIO_TEST_CLK_SEL_GATED_DISPCLK          = 0x00000001u32,
DCIO_TEST_CLK_SEL_SOCCLK                 = 0x00000002u32,
}

/*
 * DCIO_CLOCK_CNTL_DISPCLK_R_DCIO_GATE_DIS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_CLOCK_CNTL_DISPCLK_R_DCIO_GATE_DIS {
DCIO_DISPCLK_R_DCIO_GATE_DISABLE         = 0x00000000u32,
DCIO_DISPCLK_R_DCIO_GATE_ENABLE          = 0x00000001u32,
}

/*
 * DCIO_DBG_ASYNC_4BIT_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_DBG_ASYNC_4BIT_SEL {
DCIO_DBG_ASYNC_4BIT_SEL_3TO0             = 0x00000000u32,
DCIO_DBG_ASYNC_4BIT_SEL_7TO4             = 0x00000001u32,
DCIO_DBG_ASYNC_4BIT_SEL_11TO8            = 0x00000002u32,
DCIO_DBG_ASYNC_4BIT_SEL_15TO12           = 0x00000003u32,
DCIO_DBG_ASYNC_4BIT_SEL_19TO16           = 0x00000004u32,
DCIO_DBG_ASYNC_4BIT_SEL_23TO20           = 0x00000005u32,
DCIO_DBG_ASYNC_4BIT_SEL_27TO24           = 0x00000006u32,
DCIO_DBG_ASYNC_4BIT_SEL_31TO28           = 0x00000007u32,
}

/*
 * DCIO_DBG_ASYNC_BLOCK_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_DBG_ASYNC_BLOCK_SEL {
DCIO_DBG_ASYNC_BLOCK_SEL_OVERRIDE        = 0x00000000u32,
DCIO_DBG_ASYNC_BLOCK_SEL_DCCG            = 0x00000001u32,
DCIO_DBG_ASYNC_BLOCK_SEL_DCIO            = 0x00000002u32,
DCIO_DBG_ASYNC_BLOCK_SEL_DIO             = 0x00000003u32,
}

/*
 * DCIO_DCRXPHY_SOFT_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_DCRXPHY_SOFT_RESET {
DCIO_DCRXPHY_SOFT_RESET_DEASSERT         = 0x00000000u32,
DCIO_DCRXPHY_SOFT_RESET_ASSERT           = 0x00000001u32,
}

/*
 * DCIO_DC_GENERICA_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_DC_GENERICA_SEL {
DCIO_GENERICA_SEL_STEREOSYNC             = 0x00000001u32,
DCIO_GENERICA_SEL_GENERICA_DCCG          = 0x0000000au32,
DCIO_GENERICA_SEL_SYNCEN                 = 0x0000000bu32,
}

/*
 * DCIO_DC_GENERICB_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_DC_GENERICB_SEL {
DCIO_GENERICB_SEL_STEREOSYNC             = 0x00000001u32,
DCIO_GENERICB_SEL_GENERICB_DCCG          = 0x0000000au32,
DCIO_GENERICB_SEL_SYNCEN                 = 0x0000000bu32,
}

/*
 * DCIO_DC_GENERIC_UNIPHY_FBDIV_CLK_DIV2_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_DC_GENERIC_UNIPHY_FBDIV_CLK_DIV2_SEL {
DCIO_UNIPHYA_TEST_FBDIV_CLK_DIV2         = 0x00000000u32,
DCIO_UNIPHYB_TEST_FBDIV_CLK_DIV2         = 0x00000001u32,
DCIO_UNIPHYC_TEST_FBDIV_CLK_DIV2         = 0x00000002u32,
DCIO_UNIPHYD_TEST_FBDIV_CLK_DIV2         = 0x00000003u32,
DCIO_UNIPHYE_TEST_FBDIV_CLK_DIV2         = 0x00000004u32,
DCIO_UNIPHYF_TEST_FBDIV_CLK_DIV2         = 0x00000005u32,
DCIO_UNIPHYG_TEST_FBDIV_CLK_DIV2         = 0x00000006u32,
}

/*
 * DCIO_DC_GENERIC_UNIPHY_FBDIV_CLK_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_DC_GENERIC_UNIPHY_FBDIV_CLK_SEL {
DCIO_UNIPHYA_FBDIV_CLK                   = 0x00000000u32,
DCIO_UNIPHYB_FBDIV_CLK                   = 0x00000001u32,
DCIO_UNIPHYC_FBDIV_CLK                   = 0x00000002u32,
DCIO_UNIPHYD_FBDIV_CLK                   = 0x00000003u32,
DCIO_UNIPHYE_FBDIV_CLK                   = 0x00000004u32,
DCIO_UNIPHYF_FBDIV_CLK                   = 0x00000005u32,
DCIO_UNIPHYG_FBDIV_CLK                   = 0x00000006u32,
}

/*
 * DCIO_DC_GENERIC_UNIPHY_FBDIV_SSC_CLK_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_DC_GENERIC_UNIPHY_FBDIV_SSC_CLK_SEL {
DCIO_UNIPHYA_FBDIV_SSC_CLK               = 0x00000000u32,
DCIO_UNIPHYB_FBDIV_SSC_CLK               = 0x00000001u32,
DCIO_UNIPHYC_FBDIV_SSC_CLK               = 0x00000002u32,
DCIO_UNIPHYD_FBDIV_SSC_CLK               = 0x00000003u32,
DCIO_UNIPHYE_FBDIV_SSC_CLK               = 0x00000004u32,
DCIO_UNIPHYF_FBDIV_SSC_CLK               = 0x00000005u32,
DCIO_UNIPHYG_FBDIV_SSC_CLK               = 0x00000006u32,
}

/*
 * DCIO_DC_GENERIC_UNIPHY_REFDIV_CLK_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_DC_GENERIC_UNIPHY_REFDIV_CLK_SEL {
DCIO_UNIPHYA_TEST_REFDIV_CLK             = 0x00000000u32,
DCIO_UNIPHYB_TEST_REFDIV_CLK             = 0x00000001u32,
DCIO_UNIPHYC_TEST_REFDIV_CLK             = 0x00000002u32,
DCIO_UNIPHYD_TEST_REFDIV_CLK             = 0x00000003u32,
DCIO_UNIPHYE_TEST_REFDIV_CLK             = 0x00000004u32,
DCIO_UNIPHYF_TEST_REFDIV_CLK             = 0x00000005u32,
DCIO_UNIPHYG_TEST_REFDIV_CLK             = 0x00000006u32,
}

/*
 * DCIO_DC_GPIO_DEBUG_DPRX_LOOPBACK_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_DC_GPIO_DEBUG_DPRX_LOOPBACK_ENABLE {
DCIO_DPRX_LOOPBACK_ENABLE_NORMAL         = 0x00000000u32,
DCIO_DPRX_LOOPBACK_ENABLE_LOOP           = 0x00000001u32,
}

/*
 * DCIO_DC_GPU_TIMER_READ_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_DC_GPU_TIMER_READ_SELECT {
DCIO_GPU_TIMER_READ_SELECT_LOWER_D1_V_UPDATE = 0x00000000u32,
DCIO_GPU_TIMER_READ_SELECT_UPPER_D1_V_UPDATE = 0x00000001u32,
DCIO_GPU_TIMER_READ_SELECT_LOWER_D1_P_FLIP = 0x00000002u32,
DCIO_GPU_TIMER_READ_SELECT_UPPER_D1_P_FLIP = 0x00000003u32,
DCIO_GPU_TIMER_READ_SELECT_LOWER_D1_VSYNC_NOM = 0x00000004u32,
DCIO_GPU_TIMER_READ_SELECT_UPPER_D1_VSYNC_NOM = 0x00000005u32,
}

/*
 * DCIO_DC_GPU_TIMER_START_POSITION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_DC_GPU_TIMER_START_POSITION {
DCIO_GPU_TIMER_START_0_END_27            = 0x00000000u32,
DCIO_GPU_TIMER_START_1_END_28            = 0x00000001u32,
DCIO_GPU_TIMER_START_2_END_29            = 0x00000002u32,
DCIO_GPU_TIMER_START_3_END_30            = 0x00000003u32,
DCIO_GPU_TIMER_START_4_END_31            = 0x00000004u32,
DCIO_GPU_TIMER_START_6_END_33            = 0x00000005u32,
DCIO_GPU_TIMER_START_8_END_35            = 0x00000006u32,
DCIO_GPU_TIMER_START_10_END_37           = 0x00000007u32,
}

/*
 * DCIO_DC_REF_CLK_CNTL_GENLK_CLK_OUTPUT_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_DC_REF_CLK_CNTL_GENLK_CLK_OUTPUT_SEL {
DCIO_GENLK_CLK_OUTPUT_SEL_DISABLE        = 0x00000000u32,
DCIO_GENLK_CLK_OUTPUT_SEL_PPLL1          = 0x00000001u32,
DCIO_GENLK_CLK_OUTPUT_SEL_PPLL2          = 0x00000002u32,
DCIO_GENLK_CLK_OUTPUT_SEL_RESERVED_VALUE3 = 0x00000003u32,
}

/*
 * DCIO_DIO_EXT_VSYNC_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_DIO_EXT_VSYNC_MASK {
DCIO_EXT_VSYNC_MASK_NONE                 = 0x00000000u32,
DCIO_EXT_VSYNC_MASK_PIPE0                = 0x00000001u32,
DCIO_EXT_VSYNC_MASK_PIPE1                = 0x00000002u32,
DCIO_EXT_VSYNC_MASK_PIPE2                = 0x00000003u32,
DCIO_EXT_VSYNC_MASK_PIPE3                = 0x00000004u32,
DCIO_EXT_VSYNC_MASK_PIPE4                = 0x00000005u32,
DCIO_EXT_VSYNC_MASK_PIPE5                = 0x00000006u32,
DCIO_EXT_VSYNC_MASK_NONE_DUPLICATE       = 0x00000007u32,
}

/*
 * DCIO_DIO_OTG_EXT_VSYNC_MUX enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_DIO_OTG_EXT_VSYNC_MUX {
DCIO_EXT_VSYNC_MUX_SWAPLOCKB             = 0x00000000u32,
DCIO_EXT_VSYNC_MUX_OTG0                  = 0x00000001u32,
DCIO_EXT_VSYNC_MUX_OTG1                  = 0x00000002u32,
DCIO_EXT_VSYNC_MUX_OTG2                  = 0x00000003u32,
DCIO_EXT_VSYNC_MUX_OTG3                  = 0x00000004u32,
DCIO_EXT_VSYNC_MUX_OTG4                  = 0x00000005u32,
DCIO_EXT_VSYNC_MUX_OTG5                  = 0x00000006u32,
DCIO_EXT_VSYNC_MUX_GENERICB              = 0x00000007u32,
}

/*
 * DCIO_DPCS_INTERRUPT_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_DPCS_INTERRUPT_MASK {
DCIO_DPCS_INTERRUPT_DISABLE              = 0x00000000u32,
DCIO_DPCS_INTERRUPT_ENABLE               = 0x00000001u32,
}

/*
 * DCIO_DPCS_INTERRUPT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_DPCS_INTERRUPT_TYPE {
DCIO_DPCS_INTERRUPT_TYPE_LEVEL_BASED     = 0x00000000u32,
DCIO_DPCS_INTERRUPT_TYPE_PULSE_BASED     = 0x00000001u32,
}

/*
 * DCIO_GENLK_CLK_GSL_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_GENLK_CLK_GSL_MASK {
DCIO_GENLK_CLK_GSL_MASK_NO               = 0x00000000u32,
DCIO_GENLK_CLK_GSL_MASK_TIMING           = 0x00000001u32,
DCIO_GENLK_CLK_GSL_MASK_STEREO           = 0x00000002u32,
}

/*
 * DCIO_GENLK_VSYNC_GSL_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_GENLK_VSYNC_GSL_MASK {
DCIO_GENLK_VSYNC_GSL_MASK_NO             = 0x00000000u32,
DCIO_GENLK_VSYNC_GSL_MASK_TIMING         = 0x00000001u32,
DCIO_GENLK_VSYNC_GSL_MASK_STEREO         = 0x00000002u32,
}

/*
 * DCIO_GSL_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_GSL_SEL {
DCIO_GSL_SEL_GROUP_0                     = 0x00000000u32,
DCIO_GSL_SEL_GROUP_1                     = 0x00000001u32,
DCIO_GSL_SEL_GROUP_2                     = 0x00000002u32,
}

/*
 * DCIO_PHY_HPO_ENC_SRC_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_PHY_HPO_ENC_SRC_SEL {
HPO_SRC0                                 = 0x00000000u32,
HPO_SRC_RESERVED                         = 0x00000001u32,
}

/*
 * DCIO_SWAPLOCK_A_GSL_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_SWAPLOCK_A_GSL_MASK {
DCIO_SWAPLOCK_A_GSL_MASK_NO              = 0x00000000u32,
DCIO_SWAPLOCK_A_GSL_MASK_TIMING          = 0x00000001u32,
DCIO_SWAPLOCK_A_GSL_MASK_STEREO          = 0x00000002u32,
}

/*
 * DCIO_SWAPLOCK_B_GSL_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_SWAPLOCK_B_GSL_MASK {
DCIO_SWAPLOCK_B_GSL_MASK_NO              = 0x00000000u32,
DCIO_SWAPLOCK_B_GSL_MASK_TIMING          = 0x00000001u32,
DCIO_SWAPLOCK_B_GSL_MASK_STEREO          = 0x00000002u32,
}

/*
 * DCIO_UNIPHY_CHANNEL_XBAR_SOURCE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_UNIPHY_CHANNEL_XBAR_SOURCE {
DCIO_UNIPHY_CHANNEL_XBAR_SOURCE_CH0      = 0x00000000u32,
DCIO_UNIPHY_CHANNEL_XBAR_SOURCE_CH1      = 0x00000001u32,
DCIO_UNIPHY_CHANNEL_XBAR_SOURCE_CH2      = 0x00000002u32,
DCIO_UNIPHY_CHANNEL_XBAR_SOURCE_CH3      = 0x00000003u32,
}

/*
 * DCIO_UNIPHY_IMPCAL_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_UNIPHY_IMPCAL_SEL {
DCIO_UNIPHY_IMPCAL_SEL_TEMPERATURE       = 0x00000000u32,
DCIO_UNIPHY_IMPCAL_SEL_BINARY            = 0x00000001u32,
}

/*
 * DCIO_UNIPHY_LINK_CNTL_CHANNEL_INVERT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_UNIPHY_LINK_CNTL_CHANNEL_INVERT {
DCIO_UNIPHY_CHANNEL_NO_INVERSION         = 0x00000000u32,
DCIO_UNIPHY_CHANNEL_INVERTED             = 0x00000001u32,
}

/*
 * DCIO_UNIPHY_LINK_CNTL_ENABLE_HPD_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIO_UNIPHY_LINK_CNTL_ENABLE_HPD_MASK {
DCIO_UNIPHY_LINK_ENABLE_HPD_MASK_DISALLOW = 0x00000000u32,
DCIO_UNIPHY_LINK_ENABLE_HPD_MASK_ALLOW   = 0x00000001u32,
DCIO_UNIPHY_LINK_ENABLE_HPD_MASK_ALLOW_DEBOUNCED = 0x00000002u32,
DCIO_UNIPHY_LINK_ENABLE_HPD_MASK_ALLOW_TOGGLE_FILTERED = 0x00000003u32,
}

/*******************************************************
 * DCIO_CHIP Enums
 *******************************************************/

/*
 * DCIOCHIP_AUX_ALL_PWR_OK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_AUX_ALL_PWR_OK {
DCIOCHIP_AUX_ALL_PWR_OK_0                = 0x00000000u32,
DCIOCHIP_AUX_ALL_PWR_OK_1                = 0x00000001u32,
}

/*
 * DCIOCHIP_AUX_CSEL0P9 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_AUX_CSEL0P9 {
DCIOCHIP_AUX_CSEL_DEC1P0                 = 0x00000000u32,
DCIOCHIP_AUX_CSEL_DEC0P9                 = 0x00000001u32,
}

/*
 * DCIOCHIP_AUX_CSEL1P1 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_AUX_CSEL1P1 {
DCIOCHIP_AUX_CSEL_INC1P0                 = 0x00000000u32,
DCIOCHIP_AUX_CSEL_INC1P1                 = 0x00000001u32,
}

/*
 * DCIOCHIP_AUX_FALLSLEWSEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_AUX_FALLSLEWSEL {
DCIOCHIP_AUX_FALLSLEWSEL_LOW             = 0x00000000u32,
DCIOCHIP_AUX_FALLSLEWSEL_HIGH0           = 0x00000001u32,
DCIOCHIP_AUX_FALLSLEWSEL_HIGH1           = 0x00000002u32,
DCIOCHIP_AUX_FALLSLEWSEL_ULTRAHIGH       = 0x00000003u32,
}

/*
 * DCIOCHIP_AUX_HYS_TUNE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_AUX_HYS_TUNE {
DCIOCHIP_AUX_HYS_TUNE_0                  = 0x00000000u32,
DCIOCHIP_AUX_HYS_TUNE_1                  = 0x00000001u32,
DCIOCHIP_AUX_HYS_TUNE_2                  = 0x00000002u32,
DCIOCHIP_AUX_HYS_TUNE_3                  = 0x00000003u32,
}

/*
 * DCIOCHIP_AUX_RECEIVER_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_AUX_RECEIVER_SEL {
DCIOCHIP_AUX_RECEIVER_SEL_0              = 0x00000000u32,
DCIOCHIP_AUX_RECEIVER_SEL_1              = 0x00000001u32,
DCIOCHIP_AUX_RECEIVER_SEL_2              = 0x00000002u32,
DCIOCHIP_AUX_RECEIVER_SEL_3              = 0x00000003u32,
}

/*
 * DCIOCHIP_AUX_RSEL0P9 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_AUX_RSEL0P9 {
DCIOCHIP_AUX_RSEL_DEC1P0                 = 0x00000000u32,
DCIOCHIP_AUX_RSEL_DEC0P9                 = 0x00000001u32,
}

/*
 * DCIOCHIP_AUX_RSEL1P1 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_AUX_RSEL1P1 {
DCIOCHIP_AUX_RSEL_INC1P0                 = 0x00000000u32,
DCIOCHIP_AUX_RSEL_INC1P1                 = 0x00000001u32,
}

/*
 * DCIOCHIP_AUX_SPIKESEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_AUX_SPIKESEL {
DCIOCHIP_AUX_SPIKESEL_50NS               = 0x00000000u32,
DCIOCHIP_AUX_SPIKESEL_10NS               = 0x00000001u32,
}

/*
 * DCIOCHIP_AUX_VOD_TUNE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_AUX_VOD_TUNE {
DCIOCHIP_AUX_VOD_TUNE_0                  = 0x00000000u32,
DCIOCHIP_AUX_VOD_TUNE_1                  = 0x00000001u32,
DCIOCHIP_AUX_VOD_TUNE_2                  = 0x00000002u32,
DCIOCHIP_AUX_VOD_TUNE_3                  = 0x00000003u32,
}

/*
 * DCIOCHIP_GPIO_MASK_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_GPIO_MASK_EN {
DCIOCHIP_GPIO_MASK_EN_HARDWARE           = 0x00000000u32,
DCIOCHIP_GPIO_MASK_EN_SOFTWARE           = 0x00000001u32,
}

/*
 * DCIOCHIP_HPD_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_HPD_SEL {
DCIOCHIP_HPD_SEL_ASYNC                   = 0x00000000u32,
DCIOCHIP_HPD_SEL_CLOCKED                 = 0x00000001u32,
}

/*
 * DCIOCHIP_I2C_COMPSEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_I2C_COMPSEL {
DCIOCHIP_I2C_REC_SCHMIT                  = 0x00000000u32,
DCIOCHIP_I2C_REC_COMPARATOR              = 0x00000001u32,
}

/*
 * DCIOCHIP_I2C_FALLSLEWSEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_I2C_FALLSLEWSEL {
DCIOCHIP_I2C_FALLSLEWSEL_00              = 0x00000000u32,
DCIOCHIP_I2C_FALLSLEWSEL_01              = 0x00000001u32,
DCIOCHIP_I2C_FALLSLEWSEL_10              = 0x00000002u32,
DCIOCHIP_I2C_FALLSLEWSEL_11              = 0x00000003u32,
}

/*
 * DCIOCHIP_I2C_RECEIVER_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_I2C_RECEIVER_SEL {
DCIOCHIP_I2C_RECEIVER_SEL_0              = 0x00000000u32,
DCIOCHIP_I2C_RECEIVER_SEL_1              = 0x00000001u32,
DCIOCHIP_I2C_RECEIVER_SEL_2              = 0x00000002u32,
DCIOCHIP_I2C_RECEIVER_SEL_3              = 0x00000003u32,
}

/*
 * DCIOCHIP_I2C_VPH_1V2_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_I2C_VPH_1V2_EN {
DCIOCHIP_I2C_VPH_1V2_EN_0                = 0x00000000u32,
DCIOCHIP_I2C_VPH_1V2_EN_1                = 0x00000001u32,
}

/*
 * DCIOCHIP_INVERT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_INVERT {
DCIOCHIP_POL_NON_INVERT                  = 0x00000000u32,
DCIOCHIP_POL_INVERT                      = 0x00000001u32,
}

/*
 * DCIOCHIP_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_MASK {
DCIOCHIP_MASK_DISABLE                    = 0x00000000u32,
DCIOCHIP_MASK_ENABLE                     = 0x00000001u32,
}

/*
 * DCIOCHIP_PAD_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_PAD_MODE {
DCIOCHIP_PAD_MODE_DDC                    = 0x00000000u32,
DCIOCHIP_PAD_MODE_DP                     = 0x00000001u32,
}

/*
 * DCIOCHIP_PD_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_PD_EN {
DCIOCHIP_PD_EN_NOTALLOW                  = 0x00000000u32,
DCIOCHIP_PD_EN_ALLOW                     = 0x00000001u32,
}

/*
 * DCIOCHIP_REF_27_SRC_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DCIOCHIP_REF_27_SRC_SEL {
DCIOCHIP_REF_27_SRC_SEL_XTAL_DIVIDER     = 0x00000000u32,
DCIOCHIP_REF_27_SRC_SEL_DISP_CLKIN2_DIVIDER = 0x00000001u32,
DCIOCHIP_REF_27_SRC_SEL_XTAL_BYPASS      = 0x00000002u32,
DCIOCHIP_REF_27_SRC_SEL_DISP_CLKIN2_BYPASS = 0x00000003u32,
}

/*******************************************************
 * PWRSEQ Enums
 *******************************************************/

/*
 * PWRSEQ_BL_PWM_CNTL2_BL_PWM_OVERRIDE_BL_OUT_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PWRSEQ_BL_PWM_CNTL2_BL_PWM_OVERRIDE_BL_OUT_ENABLE {
PWRSEQ_BL_PWM_OVERRIDE_BL_OUT_DISABLE    = 0x00000000u32,
PWRSEQ_BL_PWM_OVERRIDE_BL_OUT_ENABLE     = 0x00000001u32,
}

/*
 * PWRSEQ_BL_PWM_CNTL2_BL_PWM_OVERRIDE_PANEL_PWRSEQ_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PWRSEQ_BL_PWM_CNTL2_BL_PWM_OVERRIDE_PANEL_PWRSEQ_EN {
PWRSEQ_BL_PWM_OVERRIDE_PANEL_PWRSEQ_EN_NORMAL = 0x00000000u32,
PWRSEQ_BL_PWM_OVERRIDE_PANEL_PWRSEQ_EN_PWM = 0x00000001u32,
}

/*
 * PWRSEQ_BL_PWM_CNTL2_DBG_BL_PWM_INPUT_REFCLK_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PWRSEQ_BL_PWM_CNTL2_DBG_BL_PWM_INPUT_REFCLK_SELECT {
PWRSEQ_DBG_BL_PWM_INPUT_REFCLK_SELECT_NORMAL = 0x00000000u32,
PWRSEQ_DBG_BL_PWM_INPUT_REFCLK_SELECT_DEBUG1 = 0x00000001u32,
PWRSEQ_DBG_BL_PWM_INPUT_REFCLK_SELECT_DEBUG2 = 0x00000002u32,
PWRSEQ_DBG_BL_PWM_INPUT_REFCLK_SELECT_DEBUG3 = 0x00000003u32,
}

/*
 * PWRSEQ_BL_PWM_CNTL_BL_PWM_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PWRSEQ_BL_PWM_CNTL_BL_PWM_EN {
PWRSEQ_BL_PWM_DISABLE                    = 0x00000000u32,
PWRSEQ_BL_PWM_ENABLE                     = 0x00000001u32,
}

/*
 * PWRSEQ_BL_PWM_CNTL_BL_PWM_FRACTIONAL_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PWRSEQ_BL_PWM_CNTL_BL_PWM_FRACTIONAL_EN {
PWRSEQ_BL_PWM_FRACTIONAL_DISABLE         = 0x00000000u32,
PWRSEQ_BL_PWM_FRACTIONAL_ENABLE          = 0x00000001u32,
}

/*
 * PWRSEQ_BL_PWM_GRP1_IGNORE_MASTER_LOCK_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PWRSEQ_BL_PWM_GRP1_IGNORE_MASTER_LOCK_EN {
PWRSEQ_BL_PWM_GRP1_IGNORE_MASTER_LOCK_ENABLE = 0x00000000u32,
PWRSEQ_BL_PWM_GRP1_IGNORE_MASTER_LOCK_DISABLE = 0x00000001u32,
}

/*
 * PWRSEQ_BL_PWM_GRP1_READBACK_DB_REG_VALUE_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PWRSEQ_BL_PWM_GRP1_READBACK_DB_REG_VALUE_EN {
PWRSEQ_BL_PWM_GRP1_READBACK_DB_REG_VALUE_EN_BL_PWM = 0x00000000u32,
PWRSEQ_BL_PWM_GRP1_READBACK_DB_REG_VALUE_EN_BL1_PWM = 0x00000001u32,
}

/*
 * PWRSEQ_BL_PWM_GRP1_REG_LOCK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PWRSEQ_BL_PWM_GRP1_REG_LOCK {
PWRSEQ_BL_PWM_GRP1_REG_LOCK_DISABLE      = 0x00000000u32,
PWRSEQ_BL_PWM_GRP1_REG_LOCK_ENABLE       = 0x00000001u32,
}

/*
 * PWRSEQ_BL_PWM_GRP1_UPDATE_AT_FRAME_START enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PWRSEQ_BL_PWM_GRP1_UPDATE_AT_FRAME_START {
PWRSEQ_BL_PWM_GRP1_UPDATE_AT_FRAME_START_DISABLE = 0x00000000u32,
PWRSEQ_BL_PWM_GRP1_UPDATE_AT_FRAME_START_ENABLE = 0x00000001u32,
}

/*
 * PWRSEQ_GPIO_MASK_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PWRSEQ_GPIO_MASK_EN {
PWRSEQ_GPIO_MASK_EN_HARDWARE             = 0x00000000u32,
PWRSEQ_GPIO_MASK_EN_SOFTWARE             = 0x00000001u32,
}

/*
 * PWRSEQ_PANEL_PWRSEQ_CNTL_PANEL_BLON enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PWRSEQ_PANEL_PWRSEQ_CNTL_PANEL_BLON {
PWRSEQ_PANEL_BLON_OFF                    = 0x00000000u32,
PWRSEQ_PANEL_BLON_ON                     = 0x00000001u32,
}

/*
 * PWRSEQ_PANEL_PWRSEQ_CNTL_PANEL_BLON_POL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PWRSEQ_PANEL_PWRSEQ_CNTL_PANEL_BLON_POL {
PWRSEQ_PANEL_BLON_POL_NON_INVERT         = 0x00000000u32,
PWRSEQ_PANEL_BLON_POL_INVERT             = 0x00000001u32,
}

/*
 * PWRSEQ_PANEL_PWRSEQ_CNTL_PANEL_DIGON enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PWRSEQ_PANEL_PWRSEQ_CNTL_PANEL_DIGON {
PWRSEQ_PANEL_DIGON_OFF                   = 0x00000000u32,
PWRSEQ_PANEL_DIGON_ON                    = 0x00000001u32,
}

/*
 * PWRSEQ_PANEL_PWRSEQ_CNTL_PANEL_DIGON_POL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PWRSEQ_PANEL_PWRSEQ_CNTL_PANEL_DIGON_POL {
PWRSEQ_PANEL_DIGON_POL_NON_INVERT        = 0x00000000u32,
PWRSEQ_PANEL_DIGON_POL_INVERT            = 0x00000001u32,
}

/*
 * PWRSEQ_PANEL_PWRSEQ_CNTL_PANEL_SYNCEN_POL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PWRSEQ_PANEL_PWRSEQ_CNTL_PANEL_SYNCEN_POL {
PWRSEQ_PANEL_SYNCEN_POL_NON_INVERT       = 0x00000000u32,
PWRSEQ_PANEL_SYNCEN_POL_INVERT           = 0x00000001u32,
}

/*
 * PWRSEQ_PANEL_PWRSEQ_CNTL_TARGET_STATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PWRSEQ_PANEL_PWRSEQ_CNTL_TARGET_STATE {
PWRSEQ_PANEL_PWRSEQ_TARGET_STATE_LCD_OFF = 0x00000000u32,
PWRSEQ_PANEL_PWRSEQ_TARGET_STATE_LCD_ON  = 0x00000001u32,
}

/*
 * PWRSEQ_PANEL_PWRSEQ_DELAY2_PANEL_VARY_BL_OVERRIDE_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PWRSEQ_PANEL_PWRSEQ_DELAY2_PANEL_VARY_BL_OVERRIDE_EN {
PWRSEQ_PANEL_VARY_BL_OVERRIDE_EN_BLON    = 0x00000000u32,
PWRSEQ_PANEL_VARY_BL_OVERRIDE_EN_SEPARATE = 0x00000001u32,
}

/*******************************************************
 * AZCONTROLLER Enums
 *******************************************************/

/*
 * AZ_CORB_SIZE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZ_CORB_SIZE {
AZ_CORB_SIZE_2ENTRIES_RESERVED           = 0x00000000u32,
AZ_CORB_SIZE_16ENTRIES_RESERVED          = 0x00000001u32,
AZ_CORB_SIZE_256ENTRIES                  = 0x00000002u32,
AZ_CORB_SIZE_RESERVED                    = 0x00000003u32,
}

/*
 * AZ_GLOBAL_CAPABILITIES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZ_GLOBAL_CAPABILITIES {
AZ_GLOBAL_CAPABILITIES_SIXTY_FOUR_BIT_ADDRESS_NOT_SUPPORTED = 0x00000000u32,
AZ_GLOBAL_CAPABILITIES_SIXTY_FOUR_BIT_ADDRESS_SUPPORTED = 0x00000001u32,
}

/*
 * AZ_RIRB_SIZE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZ_RIRB_SIZE {
AZ_RIRB_SIZE_2ENTRIES_RESERVED           = 0x00000000u32,
AZ_RIRB_SIZE_16ENTRIES_RESERVED          = 0x00000001u32,
AZ_RIRB_SIZE_256ENTRIES                  = 0x00000002u32,
AZ_RIRB_SIZE_UNDEFINED                   = 0x00000003u32,
}

/*
 * AZ_RIRB_WRITE_POINTER_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZ_RIRB_WRITE_POINTER_RESET {
AZ_RIRB_WRITE_POINTER_NOT_RESET          = 0x00000000u32,
AZ_RIRB_WRITE_POINTER_DO_RESET           = 0x00000001u32,
}

/*
 * AZ_STATE_CHANGE_STATUS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZ_STATE_CHANGE_STATUS {
AZ_STATE_CHANGE_STATUS_CODEC_NOT_PRESENT = 0x00000000u32,
AZ_STATE_CHANGE_STATUS_CODEC_PRESENT     = 0x00000001u32,
}

/*
 * CORB_READ_POINTER_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CORB_READ_POINTER_RESET {
CORB_READ_POINTER_RESET_CORB_DMA_IS_NOT_RESET = 0x00000000u32,
CORB_READ_POINTER_RESET_CORB_DMA_IS_RESET = 0x00000001u32,
}

/*
 * DMA_POSITION_LOWER_BASE_ADDRESS_BUFFER_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DMA_POSITION_LOWER_BASE_ADDRESS_BUFFER_ENABLE {
DMA_POSITION_LOWER_BASE_ADDRESS_BUFFER_ENABLE_DMA_DISABLE = 0x00000000u32,
DMA_POSITION_LOWER_BASE_ADDRESS_BUFFER_ENABLE_DMA_ENABLE = 0x00000001u32,
}

/*
 * GENERIC_AZ_CONTROLLER_REGISTER_ENABLE_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GENERIC_AZ_CONTROLLER_REGISTER_ENABLE_CONTROL {
GENERIC_AZ_CONTROLLER_REGISTER_DISABLE   = 0x00000000u32,
GENERIC_AZ_CONTROLLER_REGISTER_ENABLE    = 0x00000001u32,
}

/*
 * GENERIC_AZ_CONTROLLER_REGISTER_ENABLE_CONTROL_RESERVED enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GENERIC_AZ_CONTROLLER_REGISTER_ENABLE_CONTROL_RESERVED {
GENERIC_AZ_CONTROLLER_REGISTER_DISABLE_RESERVED = 0x00000000u32,
GENERIC_AZ_CONTROLLER_REGISTER_ENABLE_RESERVED = 0x00000001u32,
}

/*
 * GENERIC_AZ_CONTROLLER_REGISTER_STATUS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GENERIC_AZ_CONTROLLER_REGISTER_STATUS {
GENERIC_AZ_CONTROLLER_REGISTER_STATUS_NOT_SET = 0x00000000u32,
GENERIC_AZ_CONTROLLER_REGISTER_STATUS_SET = 0x00000001u32,
}

/*
 * GENERIC_AZ_CONTROLLER_REGISTER_STATUS_RESERVED enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GENERIC_AZ_CONTROLLER_REGISTER_STATUS_RESERVED {
GENERIC_AZ_CONTROLLER_REGISTER_STATUS_NOT_SET_RESERVED = 0x00000000u32,
GENERIC_AZ_CONTROLLER_REGISTER_STATUS_SET_RESERVED = 0x00000001u32,
}

/*
 * GLOBAL_CONTROL_ACCEPT_UNSOLICITED_RESPONSE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLOBAL_CONTROL_ACCEPT_UNSOLICITED_RESPONSE {
ACCEPT_UNSOLICITED_RESPONSE_NOT_ENABLE   = 0x00000000u32,
ACCEPT_UNSOLICITED_RESPONSE_ENABLE       = 0x00000001u32,
}

/*
 * GLOBAL_CONTROL_CONTROLLER_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLOBAL_CONTROL_CONTROLLER_RESET {
CONTROLLER_RESET_AZ_CONTROLLER_IN_RESET  = 0x00000000u32,
CONTROLLER_RESET_AZ_CONTROLLER_NOT_IN_RESET = 0x00000001u32,
}

/*
 * GLOBAL_CONTROL_FLUSH_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLOBAL_CONTROL_FLUSH_CONTROL {
FLUSH_CONTROL_FLUSH_NOT_STARTED          = 0x00000000u32,
FLUSH_CONTROL_FLUSH_STARTED              = 0x00000001u32,
}

/*
 * GLOBAL_STATUS_FLUSH_STATUS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLOBAL_STATUS_FLUSH_STATUS {
GLOBAL_STATUS_FLUSH_STATUS_FLUSH_NOT_ENDED = 0x00000000u32,
GLOBAL_STATUS_FLUSH_STATUS_FLUSH_ENDED   = 0x00000001u32,
}

/*
 * IMMEDIATE_COMMAND_STATUS_IMMEDIATE_COMMAND_BUSY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum IMMEDIATE_COMMAND_STATUS_IMMEDIATE_COMMAND_BUSY {
IMMEDIATE_COMMAND_STATUS_IMMEDIATE_COMMAND_NOT_BUSY = 0x00000000u32,
IMMEDIATE_COMMAND_STATUS_IMMEDIATE_COMMAND_IS_BUSY = 0x00000001u32,
}

/*
 * IMMEDIATE_COMMAND_STATUS_IMMEDIATE_RESULT_VALID enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum IMMEDIATE_COMMAND_STATUS_IMMEDIATE_RESULT_VALID {
IMMEDIATE_COMMAND_STATUS_IMMEDIATE_RESULT_VALID_NO_IMMEDIATE_RESPONSE_VALID = 0x00000000u32,
IMMEDIATE_COMMAND_STATUS_IMMEDIATE_RESULT_VALID_IMMEDIATE_RESPONSE_VALID = 0x00000001u32,
}

/*
 * RIRB_CONTROL_RESPONSE_INTERRUPT_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RIRB_CONTROL_RESPONSE_INTERRUPT_CONTROL {
RIRB_CONTROL_RESPONSE_INTERRUPT_CONTROL_INTERRUPT_DISABLED = 0x00000000u32,
RIRB_CONTROL_RESPONSE_INTERRUPT_CONTROL_INTERRUPT_ENABLED = 0x00000001u32,
}

/*
 * RIRB_CONTROL_RESPONSE_OVERRUN_INTERRUPT_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RIRB_CONTROL_RESPONSE_OVERRUN_INTERRUPT_CONTROL {
RIRB_CONTROL_RESPONSE_OVERRUN_INTERRUPT_CONTROL_INTERRUPT_DISABLED = 0x00000000u32,
RIRB_CONTROL_RESPONSE_OVERRUN_INTERRUPT_CONTROL_INTERRUPT_ENABLED = 0x00000001u32,
}

/*
 * STREAM_0_SYNCHRONIZATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum STREAM_0_SYNCHRONIZATION {
STREAM_0_SYNCHRONIZATION_STEAM_NOT_STOPPED = 0x00000000u32,
STREAM_0_SYNCHRONIZATION_STEAM_STOPPED   = 0x00000001u32,
}

/*
 * STREAM_10_SYNCHRONIZATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum STREAM_10_SYNCHRONIZATION {
STREAM_10_SYNCHRONIZATION_STEAM_NOT_STOPPED_RESERVED = 0x00000000u32,
STREAM_10_SYNCHRONIZATION_STEAM_STOPPED_RESERVED = 0x00000001u32,
}

/*
 * STREAM_11_SYNCHRONIZATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum STREAM_11_SYNCHRONIZATION {
STREAM_11_SYNCHRONIZATION_STEAM_NOT_STOPPED_RESERVED = 0x00000000u32,
STREAM_11_SYNCHRONIZATION_STEAM_STOPPED_RESERVED = 0x00000001u32,
}

/*
 * STREAM_12_SYNCHRONIZATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum STREAM_12_SYNCHRONIZATION {
STREAM_12_SYNCHRONIZATION_STEAM_NOT_STOPPED_RESERVED = 0x00000000u32,
STREAM_12_SYNCHRONIZATION_STEAM_STOPPED_RESERVED = 0x00000001u32,
}

/*
 * STREAM_13_SYNCHRONIZATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum STREAM_13_SYNCHRONIZATION {
STREAM_13_SYNCHRONIZATION_STEAM_NOT_STOPPED_RESERVED = 0x00000000u32,
STREAM_13_SYNCHRONIZATION_STEAM_STOPPED_RESERVED = 0x00000001u32,
}

/*
 * STREAM_14_SYNCHRONIZATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum STREAM_14_SYNCHRONIZATION {
STREAM_14_SYNCHRONIZATION_STEAM_NOT_STOPPED_RESERVED = 0x00000000u32,
STREAM_14_SYNCHRONIZATION_STEAM_STOPPED_RESERVED = 0x00000001u32,
}

/*
 * STREAM_15_SYNCHRONIZATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum STREAM_15_SYNCHRONIZATION {
STREAM_15_SYNCHRONIZATION_STEAM_NOT_STOPPED_RESERVED = 0x00000000u32,
STREAM_15_SYNCHRONIZATION_STEAM_STOPPED_RESERVED = 0x00000001u32,
}

/*
 * STREAM_1_SYNCHRONIZATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum STREAM_1_SYNCHRONIZATION {
STREAM_1_SYNCHRONIZATION_STEAM_NOT_STOPPED = 0x00000000u32,
STREAM_1_SYNCHRONIZATION_STEAM_STOPPED   = 0x00000001u32,
}

/*
 * STREAM_2_SYNCHRONIZATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum STREAM_2_SYNCHRONIZATION {
STREAM_2_SYNCHRONIZATION_STEAM_NOT_STOPPED = 0x00000000u32,
STREAM_2_SYNCHRONIZATION_STEAM_STOPPED   = 0x00000001u32,
}

/*
 * STREAM_3_SYNCHRONIZATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum STREAM_3_SYNCHRONIZATION {
STREAM_3_SYNCHRONIZATION_STEAM_NOT_STOPPED = 0x00000000u32,
STREAM_3_SYNCHRONIZATION_STEAM_STOPPED   = 0x00000001u32,
}

/*
 * STREAM_4_SYNCHRONIZATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum STREAM_4_SYNCHRONIZATION {
STREAM_4_SYNCHRONIZATION_STEAM_NOT_STOPPED_RESERVED = 0x00000000u32,
STREAM_4_SYNCHRONIZATION_STEAM_STOPPED_RESERVED = 0x00000001u32,
}

/*
 * STREAM_5_SYNCHRONIZATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum STREAM_5_SYNCHRONIZATION {
STREAM_5_SYNCHRONIZATION_STEAM_NOT_STOPPED_RESERVED = 0x00000000u32,
STREAM_5_SYNCHRONIZATION_STEAM_STOPPED_RESERVED = 0x00000001u32,
}

/*
 * STREAM_6_SYNCHRONIZATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum STREAM_6_SYNCHRONIZATION {
STREAM_6_SYNCHRONIZATION_STEAM_NOT_STOPPED_RESERVED = 0x00000000u32,
STREAM_6_SYNCHRONIZATION_STEAM_STOPPED_RESERVED = 0x00000001u32,
}

/*
 * STREAM_7_SYNCHRONIZATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum STREAM_7_SYNCHRONIZATION {
STREAM_7_SYNCHRONIZATION_STEAM_NOT_STOPPED_RESERVED = 0x00000000u32,
STREAM_7_SYNCHRONIZATION_STEAM_STOPPED_RESERVED = 0x00000001u32,
}

/*
 * STREAM_8_SYNCHRONIZATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum STREAM_8_SYNCHRONIZATION {
STREAM_8_SYNCHRONIZATION_STEAM_NOT_STOPPED_RESERVED = 0x00000000u32,
STREAM_8_SYNCHRONIZATION_STEAM_STOPPED_RESERVED = 0x00000001u32,
}

/*
 * STREAM_9_SYNCHRONIZATION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum STREAM_9_SYNCHRONIZATION {
STREAM_9_SYNCHRONIZATION_STEAM_NOT_STOPPED_RESERVED = 0x00000000u32,
STREAM_9_SYNCHRONIZATION_STEAM_STOPPED_RESERVED = 0x00000001u32,
}

/*******************************************************
 * AZENDPOINT Enums
 *******************************************************/

/*
 * AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_BITS_PER_SAMPLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_BITS_PER_SAMPLE {
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_BITS_PER_SAMPLE_8_RESERVED = 0x00000000u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_BITS_PER_SAMPLE_16 = 0x00000001u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_BITS_PER_SAMPLE_20 = 0x00000002u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_BITS_PER_SAMPLE_24 = 0x00000003u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_BITS_PER_SAMPLE_32_RESERVED = 0x00000004u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_BITS_PER_SAMPLE_RESERVED = 0x00000005u32,
}

/*
 * AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS {
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_1 = 0x00000000u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_2 = 0x00000001u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_3 = 0x00000002u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_4 = 0x00000003u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_5 = 0x00000004u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_6 = 0x00000005u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_7 = 0x00000006u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_8 = 0x00000007u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_RESERVED = 0x00000008u32,
}

/*
 * AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR {
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR_BY1 = 0x00000000u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR_BY2_RESERVED = 0x00000001u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR_BY3 = 0x00000002u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR_BY4_RESERVED = 0x00000003u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR_BY5_RESERVED = 0x00000004u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR_BY6_RESERVED = 0x00000005u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR_BY7_RESERVED = 0x00000006u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR_BY8_RESERVED = 0x00000007u32,
}

/*
 * AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_MULTIPLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_MULTIPLE {
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_MULTIPLE_BY1 = 0x00000000u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_MULTIPLE_BY2 = 0x00000001u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_MULTIPLE_BY3_RESERVED = 0x00000002u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_MULTIPLE_BY4 = 0x00000003u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_MULTIPLE_RESERVED = 0x00000004u32,
}

/*
 * AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_RATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_RATE {
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_RATE_48KHZ = 0x00000000u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_RATE_44P1KHZ = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_STREAM_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_STREAM_TYPE {
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_STREAM_TYPE_PCM = 0x00000000u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_CONVERTER_FORMAT_STREAM_TYPE_NOT_PCM = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_3_KEEPALIVE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_3_KEEPALIVE {
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_3_KEEPALIVE_SILENT_STREAM_NOT_ENABLE = 0x00000000u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_3_KEEPALIVE_SILENT_STREAM_ENABLE = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_COPY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_COPY {
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_COPY_BIT_C_IS_SET = 0x00000000u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_COPY_BIT_C_NOT_SET = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_DIGEN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_DIGEN {
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_DIGEN_DIGITAL_TRANSMISSION_DISABLED = 0x00000000u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_DIGEN_DIGITAL_TRANSMISSION_ENABLED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_L enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_L {
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_L_BIT7_NOT_SET = 0x00000000u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_L_BIT7_IS_SET = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_NON_AUDIO enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_NON_AUDIO {
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_NON_AUDIO_BIT_B_NOT_SET = 0x00000000u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_NON_AUDIO_BIT_B_IS_SET = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_PRE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_PRE {
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_PRE_LSB_OF_D_NOT_SET = 0x00000000u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_PRE_LSB_OF_D_IS_SET = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_PRO enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_PRO {
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_PRO_BIT_A_NOT_SET = 0x00000000u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_PRO_BIT_A_IS_SET = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_V enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_V {
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_V_BIT28_IS_ZERO = 0x00000000u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_V_BIT28_IS_ONE = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_VCFG enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_VCFG {
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_VALIDITY_CFG_NOT_ON = 0x00000000u32,
AZALIA_F2_CODEC_CONVERTER_CONTROL_DIGITAL_CONVERTER_VALIDITY_CFG_ON = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE {
AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE_0 = 0x00000000u32,
AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE_1 = 0x00000001u32,
AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE_2 = 0x00000002u32,
AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE_3 = 0x00000003u32,
AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE_4 = 0x00000004u32,
AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE_5 = 0x00000005u32,
AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE_6 = 0x00000006u32,
AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE_7 = 0x00000007u32,
AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE_8 = 0x00000008u32,
AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE_9 = 0x00000009u32,
AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE_10 = 0x0000000au32,
AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE_11 = 0x0000000bu32,
AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE_12 = 0x0000000cu32,
AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE_13 = 0x0000000du32,
AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE_14 = 0x0000000eu32,
AZALIA_F2_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR_FORMAT_CODE_15 = 0x0000000fu32,
}

/*
 * AZALIA_F2_CODEC_PIN_CONTROL_DOWN_MIX_INFO_DOWN_MIX_INHIBIT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_PIN_CONTROL_DOWN_MIX_INFO_DOWN_MIX_INHIBIT {
AZALIA_F2_CODEC_PIN_CONTROL_DOWN_MIX_NO_INFO_OR_PERMITTED = 0x00000000u32,
AZALIA_F2_CODEC_PIN_CONTROL_DOWN_MIX_FORBIDDEN = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL01_ENABLE_MULTICHANNEL01_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL01_ENABLE_MULTICHANNEL01_MUTE {
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL01_ENABLE_MULTICHANNEL01_NOT_MUTED = 0x00000000u32,
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL01_ENABLE_MULTICHANNEL01_MUTED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL1_ENABLE_MULTICHANNEL1_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL1_ENABLE_MULTICHANNEL1_MUTE {
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL1_ENABLE_MULTICHANNEL1_NOT_MUTED = 0x00000000u32,
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL1_ENABLE_MULTICHANNEL1_MUTED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL23_ENABLE_MULTICHANNEL23_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL23_ENABLE_MULTICHANNEL23_MUTE {
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL23_ENABLE_MULTICHANNEL23_NOT_MUTED = 0x00000000u32,
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL23_ENABLE_MULTICHANNEL23_MUTED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL3_ENABLE_MULTICHANNEL3_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL3_ENABLE_MULTICHANNEL3_MUTE {
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL3_ENABLE_MULTICHANNEL3_NOT_MUTED = 0x00000000u32,
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL3_ENABLE_MULTICHANNEL3_MUTED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL45_ENABLE_MULTICHANNEL45_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL45_ENABLE_MULTICHANNEL45_MUTE {
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL45_ENABLE_MULTICHANNEL45_NOT_MUTED = 0x00000000u32,
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL45_ENABLE_MULTICHANNEL45_MUTED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL5_ENABLE_MULTICHANNEL5_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL5_ENABLE_MULTICHANNEL5_MUTE {
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL5_ENABLE_MULTICHANNEL5_NOT_MUTED = 0x00000000u32,
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL5_ENABLE_MULTICHANNEL5_MUTED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL67_ENABLE_MULTICHANNEL67_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL67_ENABLE_MULTICHANNEL67_MUTE {
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL67_ENABLE_MULTICHANNEL67_NOT_MUTED = 0x00000000u32,
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL67_ENABLE_MULTICHANNEL67_MUTED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL7_ENABLE_MULTICHANNEL7_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL7_ENABLE_MULTICHANNEL7_MUTE {
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL7_ENABLE_MULTICHANNEL7_NOT_MUTED = 0x00000000u32,
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL7_ENABLE_MULTICHANNEL7_MUTED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL_MODE_MULTICHANNEL_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL_MODE_MULTICHANNEL_MODE {
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL_MODE_MULTICHANNEL_PAIR_MODE = 0x00000000u32,
AZALIA_F2_CODEC_PIN_CONTROL_MULTICHANNEL_MODE_MULTICHANNEL_SINGLE_MODE = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_PIN_CONTROL_UNSOLICITED_RESPONSE_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_PIN_CONTROL_UNSOLICITED_RESPONSE_ENABLE {
AZALIA_F2_CODEC_PIN_CONTROL_UNSOLICITED_RESPONSE_DISABLED = 0x00000000u32,
AZALIA_F2_CODEC_PIN_CONTROL_UNSOLICITED_RESPONSE_ENABLED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_PIN_CONTROL_WIDGET_CONTROL_OUT_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_PIN_CONTROL_WIDGET_CONTROL_OUT_ENABLE {
AZALIA_F2_CODEC_PIN_CONTROL_WIDGET_CONTROL_OUT_ENABLE_PIN_SHUT_OFF = 0x00000000u32,
AZALIA_F2_CODEC_PIN_CONTROL_WIDGET_CONTROL_OUT_ENABLE_PIN_DRIVEN = 0x00000001u32,
}

/*******************************************************
 * AZF0CONTROLLER Enums
 *******************************************************/

/*
 * AZALIA_SOFT_RESET_REFCLK_SOFT_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_SOFT_RESET_REFCLK_SOFT_RESET {
AZALIA_SOFT_RESET_REFCLK_SOFT_RESET_NOT_RESET = 0x00000000u32,
AZALIA_SOFT_RESET_REFCLK_SOFT_RESET_RESET_REFCLK_LOGIC = 0x00000001u32,
}

/*
 * MEM_PWR_DIS_CTRL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MEM_PWR_DIS_CTRL {
ENABLE_MEM_PWR_CTRL                      = 0x00000000u32,
DISABLE_MEM_PWR_CTRL                     = 0x00000001u32,
}

/*
 * MEM_PWR_FORCE_CTRL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MEM_PWR_FORCE_CTRL {
NO_FORCE_REQUEST                         = 0x00000000u32,
FORCE_LIGHT_SLEEP_REQUEST                = 0x00000001u32,
FORCE_DEEP_SLEEP_REQUEST                 = 0x00000002u32,
FORCE_SHUT_DOWN_REQUEST                  = 0x00000003u32,
}

/*
 * MEM_PWR_FORCE_CTRL2 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MEM_PWR_FORCE_CTRL2 {
NO_FORCE_REQ                             = 0x00000000u32,
FORCE_LIGHT_SLEEP_REQ                    = 0x00000001u32,
}

/*
 * MEM_PWR_SEL_CTRL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MEM_PWR_SEL_CTRL {
DYNAMIC_SHUT_DOWN_ENABLE                 = 0x00000000u32,
DYNAMIC_DEEP_SLEEP_ENABLE                = 0x00000001u32,
DYNAMIC_LIGHT_SLEEP_ENABLE               = 0x00000002u32,
}

/*
 * MEM_PWR_SEL_CTRL2 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MEM_PWR_SEL_CTRL2 {
DYNAMIC_DEEP_SLEEP_EN                    = 0x00000000u32,
DYNAMIC_LIGHT_SLEEP_EN                   = 0x00000001u32,
}

/*******************************************************
 * AZF0ROOT Enums
 *******************************************************/

/*
 * CC_RCU_DC_AUDIO_INPUT_PORT_CONNECTIVITY_INPUT_PORT_CONNECTIVITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CC_RCU_DC_AUDIO_INPUT_PORT_CONNECTIVITY_INPUT_PORT_CONNECTIVITY {
CC_RCU_DC_AUDIO_INPUT_PORT_CONNECTIVITY_INPUT_PORT_CONNECTIVITY_ALL = 0x00000000u32,
CC_RCU_DC_AUDIO_INPUT_PORT_CONNECTIVITY_INPUT_PORT_CONNECTIVITY_6 = 0x00000001u32,
CC_RCU_DC_AUDIO_INPUT_PORT_CONNECTIVITY_INPUT_PORT_CONNECTIVITY_5 = 0x00000002u32,
CC_RCU_DC_AUDIO_INPUT_PORT_CONNECTIVITY_INPUT_PORT_CONNECTIVITY_4 = 0x00000003u32,
CC_RCU_DC_AUDIO_INPUT_PORT_CONNECTIVITY_INPUT_PORT_CONNECTIVITY_3 = 0x00000004u32,
CC_RCU_DC_AUDIO_INPUT_PORT_CONNECTIVITY_INPUT_PORT_CONNECTIVITY_2 = 0x00000005u32,
CC_RCU_DC_AUDIO_INPUT_PORT_CONNECTIVITY_INPUT_PORT_CONNECTIVITY_1 = 0x00000006u32,
CC_RCU_DC_AUDIO_INPUT_PORT_CONNECTIVITY_INPUT_PORT_CONNECTIVITY_0 = 0x00000007u32,
}

/*
 * CC_RCU_DC_AUDIO_PORT_CONNECTIVITY_PORT_CONNECTIVITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CC_RCU_DC_AUDIO_PORT_CONNECTIVITY_PORT_CONNECTIVITY {
CC_RCU_DC_AUDIO_PORT_CONNECTIVITY_PORT_CONNECTIVITY_ALL = 0x00000000u32,
CC_RCU_DC_AUDIO_PORT_CONNECTIVITY_PORT_CONNECTIVITY_6 = 0x00000001u32,
CC_RCU_DC_AUDIO_PORT_CONNECTIVITY_PORT_CONNECTIVITY_5 = 0x00000002u32,
CC_RCU_DC_AUDIO_PORT_CONNECTIVITY_PORT_CONNECTIVITY_4 = 0x00000003u32,
CC_RCU_DC_AUDIO_PORT_CONNECTIVITY_PORT_CONNECTIVITY_3 = 0x00000004u32,
CC_RCU_DC_AUDIO_PORT_CONNECTIVITY_PORT_CONNECTIVITY_2 = 0x00000005u32,
CC_RCU_DC_AUDIO_PORT_CONNECTIVITY_PORT_CONNECTIVITY_1 = 0x00000006u32,
CC_RCU_DC_AUDIO_PORT_CONNECTIVITY_PORT_CONNECTIVITY_0 = 0x00000007u32,
}

/*******************************************************
 * AZINPUTENDPOINT Enums
 *******************************************************/

/*
 * AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_BITS_PER_SAMPLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_BITS_PER_SAMPLE {
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_BITS_PER_SAMPLE_8_RESERVED = 0x00000000u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_BITS_PER_SAMPLE_16 = 0x00000001u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_BITS_PER_SAMPLE_20 = 0x00000002u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_BITS_PER_SAMPLE_24 = 0x00000003u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_BITS_PER_SAMPLE_32_RESERVED = 0x00000004u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_BITS_PER_SAMPLE_RESERVED = 0x00000005u32,
}

/*
 * AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS {
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_1 = 0x00000000u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_2 = 0x00000001u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_3 = 0x00000002u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_4 = 0x00000003u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_5 = 0x00000004u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_6 = 0x00000005u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_7 = 0x00000006u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_8 = 0x00000007u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_NUMBER_OF_CHANNELS_RESERVED = 0x00000008u32,
}

/*
 * AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR {
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR_BY1 = 0x00000000u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR_BY2_RESERVED = 0x00000001u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR_BY3 = 0x00000002u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR_BY4_RESERVED = 0x00000003u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR_BY5_RESERVED = 0x00000004u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR_BY6_RESERVED = 0x00000005u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR_BY7_RESERVED = 0x00000006u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_DIVISOR_BY8_RESERVED = 0x00000007u32,
}

/*
 * AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_MULTIPLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_MULTIPLE {
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_MULTIPLE_BY1 = 0x00000000u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_MULTIPLE_BY2 = 0x00000001u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_MULTIPLE_BY3_RESERVED = 0x00000002u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_MULTIPLE_BY4 = 0x00000003u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_MULTIPLE_RESERVED = 0x00000004u32,
}

/*
 * AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_RATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_RATE {
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_RATE_48KHZ = 0x00000000u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_SAMPLE_BASE_RATE_44P1KHZ = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_STREAM_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_STREAM_TYPE {
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_STREAM_TYPE_PCM = 0x00000000u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_CONVERTER_FORMAT_STREAM_TYPE_NOT_PCM = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_DIGITAL_CONVERTER_DIGEN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_DIGITAL_CONVERTER_DIGEN {
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_DIGITAL_CONVERTER_DIGEN_DIGITAL_TRANSMISSION_DISABLED = 0x00000000u32,
AZALIA_F2_CODEC_INPUT_CONVERTER_CONTROL_DIGITAL_CONVERTER_DIGEN_DIGITAL_TRANSMISSION_ENABLED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL0_ENABLE_MULTICHANNEL0_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL0_ENABLE_MULTICHANNEL0_MUTE {
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL0_ENABLE_MULTICHANNEL0_NOT_MUTED = 0x00000000u32,
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL0_ENABLE_MULTICHANNEL0_MUTED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL1_ENABLE_MULTICHANNEL1_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL1_ENABLE_MULTICHANNEL1_MUTE {
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL1_ENABLE_MULTICHANNEL1_NOT_MUTED = 0x00000000u32,
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL1_ENABLE_MULTICHANNEL1_MUTED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL2_ENABLE_MULTICHANNEL2_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL2_ENABLE_MULTICHANNEL2_MUTE {
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL2_ENABLE_MULTICHANNEL2_NOT_MUTED = 0x00000000u32,
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL2_ENABLE_MULTICHANNEL2_MUTED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL3_ENABLE_MULTICHANNEL3_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL3_ENABLE_MULTICHANNEL3_MUTE {
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL3_ENABLE_MULTICHANNEL3_NOT_MUTED = 0x00000000u32,
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL3_ENABLE_MULTICHANNEL3_MUTED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL4_ENABLE_MULTICHANNEL4_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL4_ENABLE_MULTICHANNEL4_MUTE {
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL4_ENABLE_MULTICHANNEL4_NOT_MUTED = 0x00000000u32,
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL4_ENABLE_MULTICHANNEL4_MUTED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL5_ENABLE_MULTICHANNEL5_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL5_ENABLE_MULTICHANNEL5_MUTE {
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL5_ENABLE_MULTICHANNEL5_NOT_MUTED = 0x00000000u32,
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL5_ENABLE_MULTICHANNEL5_MUTED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL6_ENABLE_MULTICHANNEL6_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL6_ENABLE_MULTICHANNEL6_MUTE {
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL6_ENABLE_MULTICHANNEL6_NOT_MUTED = 0x00000000u32,
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL6_ENABLE_MULTICHANNEL6_MUTED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL7_ENABLE_MULTICHANNEL7_MUTE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL7_ENABLE_MULTICHANNEL7_MUTE {
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL7_ENABLE_MULTICHANNEL7_NOT_MUTED = 0x00000000u32,
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_MULTICHANNEL7_ENABLE_MULTICHANNEL7_MUTED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_INPUT_PIN_CONTROL_UNSOLICITED_RESPONSE_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_INPUT_PIN_CONTROL_UNSOLICITED_RESPONSE_ENABLE {
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_UNSOLICITED_RESPONSE_DISABLED = 0x00000000u32,
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_UNSOLICITED_RESPONSE_ENABLED = 0x00000001u32,
}

/*
 * AZALIA_F2_CODEC_INPUT_PIN_CONTROL_WIDGET_CONTROL_IN_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_INPUT_PIN_CONTROL_WIDGET_CONTROL_IN_ENABLE {
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_WIDGET_CONTROL_IN_ENABLE_PIN_SHUT_OFF = 0x00000000u32,
AZALIA_F2_CODEC_INPUT_PIN_CONTROL_WIDGET_CONTROL_IN_ENABLE_PIN_DRIVEN = 0x00000001u32,
}

/*******************************************************
 * AZROOT Enums
 *******************************************************/

/*
 * AZALIA_F2_CODEC_FUNCTION_CONTROL_RESET_CODEC_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F2_CODEC_FUNCTION_CONTROL_RESET_CODEC_RESET {
AZALIA_F2_CODEC_FUNCTION_CONTROL_RESET_CODEC_NOT_RESET = 0x00000000u32,
AZALIA_F2_CODEC_FUNCTION_CONTROL_RESET_CODEC_DO_RESET = 0x00000001u32,
}

/*******************************************************
 * AZF0STREAM Enums
 *******************************************************/

/*
 * AZ_LATENCY_COUNTER_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZ_LATENCY_COUNTER_CONTROL {
AZ_LATENCY_COUNTER_NO_RESET              = 0x00000000u32,
AZ_LATENCY_COUNTER_RESET_DONE            = 0x00000001u32,
}

/*******************************************************
 * AZSTREAM Enums
 *******************************************************/

/*
 * OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_BUFFER_COMPLETION_INTERRUPT_STATUS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_BUFFER_COMPLETION_INTERRUPT_STATUS {
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_BUFFER_COMPLETION_INTERRUPT_STATUS_NOT_SET = 0x00000000u32,
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_BUFFER_COMPLETION_INTERRUPT_STATUS_SET = 0x00000001u32,
}

/*
 * OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_DESCRIPTOR_ERROR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_DESCRIPTOR_ERROR {
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_DESCRIPTOR_ERROR_STATUS_NOT_SET = 0x00000000u32,
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_DESCRIPTOR_ERROR_STATUS_SET = 0x00000001u32,
}

/*
 * OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_DESCRIPTOR_ERROR_INTERRUPT_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_DESCRIPTOR_ERROR_INTERRUPT_ENABLE {
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_DESCRIPTOR_ERROR_INTERRUPT_DISABLED = 0x00000000u32,
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_DESCRIPTOR_ERROR_INTERRUPT_ENABLED = 0x00000001u32,
}

/*
 * OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_FIFO_ERROR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_FIFO_ERROR {
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_FIFO_ERROR_STATUS_NOT_SET = 0x00000000u32,
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_FIFO_ERROR_STATUS_SET = 0x00000001u32,
}

/*
 * OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_FIFO_ERROR_INTERRUPT_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_FIFO_ERROR_INTERRUPT_ENABLE {
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_FIFO_ERROR_INTERRUPT_DISABLED = 0x00000000u32,
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_FIFO_ERROR_INTERRUPT_ENABLED = 0x00000001u32,
}

/*
 * OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_INTERRUPT_ON_COMPLETION_ENABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_INTERRUPT_ON_COMPLETION_ENABLE {
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_INTERRUPT_ON_COMPLETION_ENABLE_INTERRUPT_DISABLED = 0x00000000u32,
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_INTERRUPT_ON_COMPLETION_ENABLE_INTERRUPT_ENABLED = 0x00000001u32,
}

/*
 * OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_STREAM_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_STREAM_RESET {
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_STREAM_NOT_RESET = 0x00000000u32,
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_STREAM_IS_RESET = 0x00000001u32,
}

/*
 * OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_STREAM_RUN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_STREAM_RUN {
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_STREAM_NOT_RUN = 0x00000000u32,
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_STREAM_DO_RUN = 0x00000001u32,
}

/*
 * OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_TRAFFIC_PRIORITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_TRAFFIC_PRIORITY {
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_NO_TRAFFIC_PRIORITY = 0x00000000u32,
OUTPUT_STREAM_DESCRIPTOR_CONTROL_AND_STATUS_YES_TRAFFIC_PRIORITY = 0x00000001u32,
}

/*
 * OUTPUT_STREAM_DESCRIPTOR_FORMAT_BITS_PER_SAMPLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OUTPUT_STREAM_DESCRIPTOR_FORMAT_BITS_PER_SAMPLE {
OUTPUT_STREAM_DESCRIPTOR_FORMAT_BITS_PER_SAMPLE_8_RESERVED = 0x00000000u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_BITS_PER_SAMPLE_16 = 0x00000001u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_BITS_PER_SAMPLE_20 = 0x00000002u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_BITS_PER_SAMPLE_24 = 0x00000003u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_BITS_PER_SAMPLE_32_RESERVED = 0x00000004u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_BITS_PER_SAMPLE_RESERVED = 0x00000005u32,
}

/*
 * OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS {
OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS_1 = 0x00000000u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS_2 = 0x00000001u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS_3 = 0x00000002u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS_4 = 0x00000003u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS_5 = 0x00000004u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS_6 = 0x00000005u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS_7 = 0x00000006u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS_8 = 0x00000007u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS_9_RESERVED = 0x00000008u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS_10_RESERVED = 0x00000009u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS_11_RESERVED = 0x0000000au32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS_12_RESERVED = 0x0000000bu32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS_13_RESERVED = 0x0000000cu32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS_14_RESERVED = 0x0000000du32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS_15_RESERVED = 0x0000000eu32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_NUMBER_OF_CHANNELS_16_RESERVED = 0x0000000fu32,
}

/*
 * OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_DIVISOR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_DIVISOR {
OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_DIVISOR_BY1 = 0x00000000u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_DIVISOR_BY2_RESERVED = 0x00000001u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_DIVISOR_BY3 = 0x00000002u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_DIVISOR_BY4_RESERVED = 0x00000003u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_DIVISOR_BY5_RESERVED = 0x00000004u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_DIVISOR_BY6_RESERVED = 0x00000005u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_DIVISOR_BY7_RESERVED = 0x00000006u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_DIVISOR_BY8_RESERVED = 0x00000007u32,
}

/*
 * OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_MULTIPLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_MULTIPLE {
OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_MULTIPLE_BY1 = 0x00000000u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_MULTIPLE_BY2 = 0x00000001u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_MULTIPLE_BY3_RESERVED = 0x00000002u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_MULTIPLE_BY4 = 0x00000003u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_MULTIPLE_RESERVED = 0x00000004u32,
}

/*
 * OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_RATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_RATE {
OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_RATE_48KHZ = 0x00000000u32,
OUTPUT_STREAM_DESCRIPTOR_FORMAT_SAMPLE_BASE_RATE_44P1KHZ = 0x00000001u32,
}

/*******************************************************
 * AZF0ENDPOINT Enums
 *******************************************************/

/*
 * AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_AMPLIFIER_PARAMETER_OVERRIDE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_AMPLIFIER_PARAMETER_OVERRIDE {
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_AMPLIFIER_PARAMETER = 0x00000000u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_AMPLIFIER_PARAMETER_OVERRIDE = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_AUDIO_CHANNEL_CAPABILITIES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_AUDIO_CHANNEL_CAPABILITIES {
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_AUDIO_CHANNEL_CAPABILITIES_MONOPHONIC = 0x00000000u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_AUDIO_CHANNEL_CAPABILITIES_STEREO = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_CONNECTION_LIST enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_CONNECTION_LIST {
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_CONNECTION_LIST = 0x00000000u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_CONNECTION_LIST = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_DIGITAL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_DIGITAL {
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_IS_ANALOG = 0x00000000u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_IS_DIGITAL = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_FORMAT_OVERRIDE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_FORMAT_OVERRIDE {
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_FORMAT_OVERRIDE = 0x00000000u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_SUPPORT_FORMAT_OVERRIDE = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_INPUT_AMPLIFIER_PRESENT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_INPUT_AMPLIFIER_PRESENT {
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_INPUT_AMPLIFIER = 0x00000000u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_INPUT_AMPLIFIER = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_LR_SWAP enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_LR_SWAP {
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_LR_SWAP_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_LR_SWAP_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_OUTPUT_AMPLIFIER_PRESENT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_OUTPUT_AMPLIFIER_PRESENT {
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_OUTPUT_AMPLIFIER = 0x00000000u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_OUTPUT_AMPLIFIER = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_POWER_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_POWER_CONTROL {
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_POWER_CONTROL_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_POWER_CONTROL_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_PROCESSING_WIDGET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_PROCESSING_WIDGET {
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_PROCESSING_WIDGET_NO_PROCESSING_CAPABILITIES = 0x00000000u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_PROCESSING_WIDGET_HAVE_PROCESSING_CAPABILITIES = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_STRIPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_STRIPE {
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_SUPPORT_STRIPING = 0x00000000u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_SUPPORT_STRIPING = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE {
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_OUTPUT_CONVERTER_RESERVED = 0x00000000u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_INPUT_CONVERTER_RESERVED = 0x00000001u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_MIXER_RESERVED = 0x00000002u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_SELECTOR_RESERVED = 0x00000003u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_PIN_RESERVED = 0x00000004u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_POWER_WIDGET_RESERVED = 0x00000005u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_VOLUME_KNOB_RESERVED = 0x00000006u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_BEEP_GENERATOR_RESERVED = 0x00000007u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_RESERVED_RESERVED = 0x00000008u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_VENDOR_DEFINED_RESERVED = 0x00000009u32,
}

/*
 * AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_UNSOLICITED_RESPONSE_CAPABILITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_UNSOLICITED_RESPONSE_CAPABILITY {
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_UNSOLICITED_RESPONSE_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_UNSOLICITED_RESPONSE_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_CONTROL_MULTICHANNEL_MODE_MULTICHANNEL_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_CONTROL_MULTICHANNEL_MODE_MULTICHANNEL_MODE {
AZALIA_F0_CODEC_PIN_CONTROL_MULTICHANNEL_MODE_MULTICHANNEL_PAIR_MODE = 0x00000000u32,
AZALIA_F0_CODEC_PIN_CONTROL_MULTICHANNEL_MODE_MULTICHANNEL_SINGLE_MODE = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_CONTROL_RESPONSE_HBR_HBR_CAPABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_CONTROL_RESPONSE_HBR_HBR_CAPABLE {
AZALIA_F0_CODEC_PIN_CONTROL_RESPONSE_HBR_NO_HBR_CAPABLILITY = 0x00000000u32,
AZALIA_F0_CODEC_PIN_CONTROL_RESPONSE_HBR_HAVE_HBR_CAPABLILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_AMPLIFIER_PARAMETER_OVERRIDE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_AMPLIFIER_PARAMETER_OVERRIDE {
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_AMPLIFIER_PARAMETER = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_AMPLIFIER_PARAMETER_OVERRIDE = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_CONNECTION_LIST enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_CONNECTION_LIST {
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_CONNECTION_LIST = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_CONNECTION_LIST = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_DIGITAL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_DIGITAL {
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_IS_ANALOG = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_IS_DIGITAL = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_INPUT_AMPLIFIER_PRESENT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_INPUT_AMPLIFIER_PRESENT {
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_INPUT_AMPLIFIER_PRESENT = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_INPUT_AMPLIFIER = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_LR_SWAP enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_LR_SWAP {
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_LR_SWAP_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_LR_SWAP_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_OUTPUT_AMPLIFIER_PRESENT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_OUTPUT_AMPLIFIER_PRESENT {
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_OUTPUT_AMPLIFIER = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_OUTPUT_AMPLIFIER = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_POWER_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_POWER_CONTROL {
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_POWER_CONTROL_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_POWER_CONTROL_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_PROCESSING_WIDGET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_PROCESSING_WIDGET {
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_PROCESSING_WIDGET_NO_PROCESSING_CAPABILITIES = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_PROCESSING_WIDGET_HAVE_PROCESSING_CAPABILITIES = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_STRIPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_STRIPE {
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_SUPPORT_STRIPING = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_SUPPORT_STRIPING = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE {
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_OUTPUT_CONVERTER_RESERVED = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_INPUT_CONVERTER_RESERVED = 0x00000001u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_MIXER_RESERVED = 0x00000002u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_SELECTOR_RESERVED = 0x00000003u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_PIN_RESERVED = 0x00000004u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_POWER_WIDGET_RESERVED = 0x00000005u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_VOLUME_KNOB_RESERVED = 0x00000006u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_BEEP_GENERATOR_RESERVED = 0x00000007u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_RESERVED_RESERVED = 0x00000008u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_VENDOR_DEFINED_RESERVED = 0x00000009u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_UNSOLICITED_RESPONSE_CAPABILITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_UNSOLICITED_RESPONSE_CAPABILITY {
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_UNSOLICITED_RESPONSE_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_UNSOLICITED_RESPONSE_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_BALANCED_I_O_PINS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_BALANCED_I_O_PINS {
AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_I_O_PINS_ARE_NOT_BALANCED = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_I_O_PINS_ARE_BALANCED = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_EAPD_CAPABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_EAPD_CAPABLE {
AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_NO_EAPD_PIN = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_HAVE_EAPD_PIN = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_HEADPHONE_DRIVE_CAPABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_HEADPHONE_DRIVE_CAPABLE {
AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_NO_HEADPHONE_DRIVE_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_HAVE_HEADPHONE_DRIVE_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_IMPEDANCE_SENSE_CAPABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_IMPEDANCE_SENSE_CAPABLE {
AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_NO_IMPEDANCE_SENSE_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_HAVE_IMPEDANCE_SENSE_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_INPUT_CAPABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_INPUT_CAPABLE {
AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_NO_INPUT_PIN = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_HAVE_INPUT_PIN = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_JACK_DETECTION_CAPABILITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_JACK_DETECTION_CAPABILITY {
AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_NO_JACK_DETECTION_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_HAVE_JACK_DETECTION_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_OUTPUT_CAPABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_OUTPUT_CAPABLE {
AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_NO_OUTPUT_PIN = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_HAVE_OUTPUT_PIN = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_TRIGGER_REQUIRED enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_TRIGGER_REQUIRED {
AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_NO_TRIGGER_REQUIRED_FOR_IMPEDANCE_MEASUREMENT = 0x00000000u32,
AZALIA_F0_CODEC_PIN_PARAMETER_CAPABILITIES_TRIGGER_REQUIRED_FOR_IMPEDANCE_MEASUREMENT = 0x00000001u32,
}

/*******************************************************
 * AZF0INPUTENDPOINT Enums
 *******************************************************/

/*
 * AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_AMPLIFIER_PARAMETER_OVERRIDE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_AMPLIFIER_PARAMETER_OVERRIDE {
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_AMPLIFIER_PARAMETER = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_AMPLIFIER_PARAMETER = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_AUDIO_CHANNEL_CAPABILITIES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_AUDIO_CHANNEL_CAPABILITIES {
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_AUDIO_CHANNEL_CAPABILITIES_MONOPHONIC = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_AUDIO_CHANNEL_CAPABILITIES_STEREO = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_CONNECTION_LIST enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_CONNECTION_LIST {
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_CONNECTION_LIST = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_CONNECTION_LIST = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_DIGITAL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_DIGITAL {
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_CODEC_CONVERTER0_IS_ANALOG = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_CODEC_CONVERTER0_IS_DIGITAL = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_FORMAT_OVERRIDE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_FORMAT_OVERRIDE {
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_FORMAT_OVERRIDE = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_FORMAT_OVERRIDE = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_INPUT_AMPLIFIER_PRESENT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_INPUT_AMPLIFIER_PRESENT {
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_INPUT_AMPLIFIER = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_INPUT_AMPLIFIER = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_LR_SWAP enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_LR_SWAP {
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_LR_SWAP_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_LR_SWAP_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_OUTPUT_AMPLIFIER_PRESENT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_OUTPUT_AMPLIFIER_PRESENT {
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_OUTPUT_AMPLIFIER = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_OUTPUT_AMPLIFIER = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_POWER_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_POWER_CONTROL {
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_POWER_CONTROL_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_POWER_CONTROL_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_PROCESSING_WIDGET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_PROCESSING_WIDGET {
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_PROCESSING_WIDGET_CODEC_CONVERTER0_HAVE_NO_PROCESSING_CAPABILITIES = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_PROCESSING_WIDGET_CODEC_CONVERTER0_HAVE_PROCESSING_CAPABILITIES = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_STRIPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_STRIPE {
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NOT_SUPPORT_STRIPING = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_SUPPORT_STRIPING = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE {
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_OUTPUT_CONVERTER_RESERVED = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_INPUT_CONVERTER_RESERVED = 0x00000001u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_MIXER_RESERVED = 0x00000002u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_SELECTOR_RESERVED = 0x00000003u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_PIN_RESERVED = 0x00000004u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_POWER_WIDGET_RESERVED = 0x00000005u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_VOLUME_KNOB_RESERVED = 0x00000006u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_BEEP_GENERATOR_RESERVED = 0x00000007u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_RESERVED = 0x00000008u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_VENDOR_DEFINED_RESERVED = 0x00000009u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_UNSOLICITED_RESPONSE_CAPABILITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_UNSOLICITED_RESPONSE_CAPABILITY {
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_UNSOLICITED_RESPONSE_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_CONVERTER_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_UNSOLICITED_RESPONSE_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_CONTROL_RESPONSE_HBR_HBR_CAPABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_CONTROL_RESPONSE_HBR_HBR_CAPABLE {
AZALIA_F0_CODEC_INPUT_PIN_CONTROL_RESPONSE_HBR_NO_HBR_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_CONTROL_RESPONSE_HBR_HAVE_HBR_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_AMPLIFIER_PARAMETER_OVERRIDE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_AMPLIFIER_PARAMETER_OVERRIDE {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_AMPLIFIER_PARAMETER = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_AMPLIFIER_PARAMETER_OVERRIDE = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_CONNECTION_LIST enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_CONNECTION_LIST {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_CONNECTION_LIST = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_CONNECTION_LIST = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_DIGITAL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_DIGITAL {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_IS_ANALOG = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_IS_DIGITAL = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_INPUT_AMPLIFIER_PRESENT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_INPUT_AMPLIFIER_PRESENT {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_INPUT_AMPLIFIER = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_INPUT_AMPLIFIER = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_LR_SWAP enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_LR_SWAP {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_LR_SWAP = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_LR_SWAP = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_OUTPUT_AMPLIFIER_PRESENT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_OUTPUT_AMPLIFIER_PRESENT {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_OUTPUT_AMPLIFIER = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_OUTPUT_AMPLIFIER = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_POWER_CONTROL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_POWER_CONTROL {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_POWER_CONTROL_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_POWER_CONTROL_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_PROCESSING_WIDGET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_PROCESSING_WIDGET {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_PROCESSING_WIDGET_NO_PROCESING_CAPABILITIES = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_PROCESSING_WIDGET_HAVE_PROCESING_CAPABILITIES = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_STRIPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_STRIPE {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_SUPPORT_STRIPING = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_SUPPORT_STRIPING = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_OUTPUT_CONVERTER_RESERVED = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_INPUT_CONVERTER_RESERVED = 0x00000001u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_MIXER_RESERVED = 0x00000002u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_SELECTOR_RESERVED = 0x00000003u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_PIN_RESERVED = 0x00000004u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_POWER_WIDGET_RESERVED = 0x00000005u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_VOLUME_KNOB_RESERVED = 0x00000006u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_BEEP_GENERATOR_RESERVED = 0x00000007u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_RESERVED = 0x00000008u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_TYPE_VENDOR_DEFINED_RESERVED = 0x00000009u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_UNSOLICITED_RESPONSE_CAPABILITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_UNSOLICITED_RESPONSE_CAPABILITY {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_NO_UNSOLICITED_RESPONSE_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_AUDIO_WIDGET_CAPABILITIES_HAVE_UNSOLICITED_RESPONSE_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_BALANCED_I_O_PINS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_BALANCED_I_O_PINS {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_I_O_PINS_NOT_BALANCED = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_I_O_PINS_ARE_BALANCED = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_DP enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_DP {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_DP_NOT_ENABLED = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_DP_ENABLED = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_EAPD_CAPABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_EAPD_CAPABLE {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_EAPD_CAPABLE_NO_EAPD_PIN = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_EAPD_CAPABLE_HAVE_EAPD_PIN = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_HDMI enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_HDMI {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_HDMI_NOT_ENABLED = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_HDMI_ENABLED = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_HEADPHONE_DRIVE_CAPABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_HEADPHONE_DRIVE_CAPABLE {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_NO_HEADPHONE_DRIVE_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_HAVE_HEADPHONE_DRIVE_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_IMPEDANCE_SENSE_CAPABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_IMPEDANCE_SENSE_CAPABLE {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_NO_IMPEDANCE_SENSE_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_HAVE_IMPEDANCE_SENSE_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_INPUT_CAPABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_INPUT_CAPABLE {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_NO_INPUT_PIN = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_HAVE_INPUT_PIN = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_JACK_DETECTION_CAPABILITY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_JACK_DETECTION_CAPABILITY {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_NO_JACK_PRESENCE_DETECTION_CAPABILITY = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_HAVE_JACK_PRESENCE_DETECTION_CAPABILITY = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_OUTPUT_CAPABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_OUTPUT_CAPABLE {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_NO_OUTPUT_PIN = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_HAVE_OUTPUT_PIN = 0x00000001u32,
}

/*
 * AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_TRIGGER_REQUIRED enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_TRIGGER_REQUIRED {
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_NO_TRIGGER_REQUIRED_FOR_IMPEDANCE_MEASUREMENT = 0x00000000u32,
AZALIA_F0_CODEC_INPUT_PIN_PARAMETER_CAPABILITIES_TRIGGER_REQUIRED_FOR_IMPEDANCE_MEASUREMENT = 0x00000001u32,
}

/*******************************************************
 * DSCC Enums
 *******************************************************/

/*
 * DSCC_BITS_PER_COMPONENT_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DSCC_BITS_PER_COMPONENT_ENUM {
DSCC_BITS_PER_COMPONENT_ENUM_BITS_PER_COMPONENT_8_BIT = 0x00000008u32,
DSCC_BITS_PER_COMPONENT_ENUM_BITS_PER_COMPONENT_10_BIT = 0x0000000au32,
DSCC_BITS_PER_COMPONENT_ENUM_BITS_PER_COMPONENT_12_BIT = 0x0000000cu32,
}

/*
 * DSCC_DSC_VERSION_MAJOR_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DSCC_DSC_VERSION_MAJOR_ENUM {
DSCC_DSC_VERSION_MAJOR_ENUM_DSC_1_X_MAJOR_VERSION = 0x00000001u32,
}

/*
 * DSCC_DSC_VERSION_MINOR_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DSCC_DSC_VERSION_MINOR_ENUM {
DSCC_DSC_VERSION_MINOR_ENUM_DSC_X_1_MINOR_VERSION = 0x00000001u32,
DSCC_DSC_VERSION_MINOR_ENUM_DSC_X_2_MINOR_VERSION = 0x00000002u32,
}

/*
 * DSCC_ENABLE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DSCC_ENABLE_ENUM {
DSCC_ENABLE_ENUM_DISABLED                = 0x00000000u32,
DSCC_ENABLE_ENUM_ENABLED                 = 0x00000001u32,
}

/*
 * DSCC_ICH_RESET_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DSCC_ICH_RESET_ENUM {
DSCC_ICH_RESET_ENUM_SLICE0_ICH_RESET     = 0x00000001u32,
DSCC_ICH_RESET_ENUM_SLICE1_ICH_RESET     = 0x00000002u32,
DSCC_ICH_RESET_ENUM_SLICE2_ICH_RESET     = 0x00000004u32,
DSCC_ICH_RESET_ENUM_SLICE3_ICH_RESET     = 0x00000008u32,
}

/*
 * DSCC_LINEBUF_DEPTH_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DSCC_LINEBUF_DEPTH_ENUM {
DSCC_LINEBUF_DEPTH_ENUM_LINEBUF_DEPTH_8_BIT = 0x00000008u32,
DSCC_LINEBUF_DEPTH_ENUM_LINEBUF_DEPTH_9_BIT = 0x00000009u32,
DSCC_LINEBUF_DEPTH_ENUM_LINEBUF_DEPTH_10_BIT = 0x0000000au32,
DSCC_LINEBUF_DEPTH_ENUM_LINEBUF_DEPTH_11_BIT = 0x0000000bu32,
DSCC_LINEBUF_DEPTH_ENUM_LINEBUF_DEPTH_12_BIT = 0x0000000cu32,
DSCC_LINEBUF_DEPTH_ENUM_LINEBUF_DEPTH_13_BIT = 0x0000000du32,
}

/*
 * DSCC_MEM_PWR_DIS_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DSCC_MEM_PWR_DIS_ENUM {
DSCC_MEM_PWR_DIS_ENUM_REQUEST_EN         = 0x00000000u32,
DSCC_MEM_PWR_DIS_ENUM_REQUEST_DIS        = 0x00000001u32,
}

/*
 * DSCC_MEM_PWR_FORCE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DSCC_MEM_PWR_FORCE_ENUM {
DSCC_MEM_PWR_FORCE_ENUM_NO_FORCE_REQUEST = 0x00000000u32,
DSCC_MEM_PWR_FORCE_ENUM_FORCE_LIGHT_SLEEP_REQUEST = 0x00000001u32,
DSCC_MEM_PWR_FORCE_ENUM_FORCE_DEEP_SLEEP_REQUEST = 0x00000002u32,
DSCC_MEM_PWR_FORCE_ENUM_FORCE_SHUT_DOWN_REQUEST = 0x00000003u32,
}

/*
 * POWER_STATE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum POWER_STATE_ENUM {
POWER_STATE_ENUM_ON                      = 0x00000000u32,
POWER_STATE_ENUM_LS                      = 0x00000001u32,
POWER_STATE_ENUM_DS                      = 0x00000002u32,
POWER_STATE_ENUM_SD                      = 0x00000003u32,
}

/*******************************************************
 * DSCCIF Enums
 *******************************************************/

/*
 * DSCCIF_BITS_PER_COMPONENT_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DSCCIF_BITS_PER_COMPONENT_ENUM {
DSCCIF_BITS_PER_COMPONENT_ENUM_BITS_PER_COMPONENT_8_BIT = 0x00000008u32,
DSCCIF_BITS_PER_COMPONENT_ENUM_BITS_PER_COMPONENT_10_BIT = 0x0000000au32,
DSCCIF_BITS_PER_COMPONENT_ENUM_BITS_PER_COMPONENT_12_BIT = 0x0000000cu32,
}

/*
 * DSCCIF_ENABLE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DSCCIF_ENABLE_ENUM {
DSCCIF_ENABLE_ENUM_DISABLED              = 0x00000000u32,
DSCCIF_ENABLE_ENUM_ENABLED               = 0x00000001u32,
}

/*
 * DSCCIF_INPUT_PIXEL_FORMAT_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DSCCIF_INPUT_PIXEL_FORMAT_ENUM {
DSCCIF_INPUT_PIXEL_FORMAT_ENUM_RGB       = 0x00000000u32,
DSCCIF_INPUT_PIXEL_FORMAT_ENUM_YCBCR_444 = 0x00000001u32,
DSCCIF_INPUT_PIXEL_FORMAT_ENUM_SIMPLE_YCBCR_422 = 0x00000002u32,
DSCCIF_INPUT_PIXEL_FORMAT_ENUM_NATIVE_YCBCR_422 = 0x00000003u32,
DSCCIF_INPUT_PIXEL_FORMAT_ENUM_NATIVE_YCBCR_420 = 0x00000004u32,
}

/*******************************************************
 * DSC_TOP Enums
 *******************************************************/

/*
 * CLOCK_GATING_DISABLE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CLOCK_GATING_DISABLE_ENUM {
CLOCK_GATING_DISABLE_ENUM_ENABLED        = 0x00000000u32,
CLOCK_GATING_DISABLE_ENUM_DISABLED       = 0x00000001u32,
}

/*
 * ENABLE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ENABLE_ENUM {
ENABLE_ENUM_DISABLED                     = 0x00000000u32,
ENABLE_ENUM_ENABLED                      = 0x00000001u32,
}

/*
 * TEST_CLOCK_MUX_SELECT_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TEST_CLOCK_MUX_SELECT_ENUM {
TEST_CLOCK_MUX_SELECT_DISPCLK_P          = 0x00000000u32,
TEST_CLOCK_MUX_SELECT_DISPCLK_G          = 0x00000001u32,
TEST_CLOCK_MUX_SELECT_DISPCLK_R          = 0x00000002u32,
TEST_CLOCK_MUX_SELECT_DSCCLK_P           = 0x00000003u32,
TEST_CLOCK_MUX_SELECT_DSCCLK_G           = 0x00000004u32,
TEST_CLOCK_MUX_SELECT_DSCCLK_R           = 0x00000005u32,
TEST_CLOCK_MUX_SELECT_DSCCLK_D           = 0x00000006u32,
}

/*******************************************************
 * DWB_TOP Enums
 *******************************************************/

/*
 * DWB_CRC_CONT_EN_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_CRC_CONT_EN_ENUM {
DWB_CRC_CONT_EN_ONE_SHOT                 = 0x00000000u32,
DWB_CRC_CONT_EN_CONT                     = 0x00000001u32,
}

/*
 * DWB_CRC_SRC_SEL_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_CRC_SRC_SEL_ENUM {
DWB_CRC_SRC_SEL_DWB_IN                   = 0x00000000u32,
DWB_CRC_SRC_SEL_OGAM_OUT                 = 0x00000001u32,
DWB_CRC_SRC_SEL_DWB_OUT                  = 0x00000002u32,
}

/*
 * DWB_DATA_OVERFLOW_INT_TYPE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_DATA_OVERFLOW_INT_TYPE_ENUM {
DWB_DATA_OVERFLOW_INT_TYPE_0             = 0x00000000u32,
DWB_DATA_OVERFLOW_INT_TYPE_1             = 0x00000001u32,
}

/*
 * DWB_DATA_OVERFLOW_TYPE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_DATA_OVERFLOW_TYPE_ENUM {
DWB_DATA_OVERFLOW_TYPE_NO_OVERFLOW       = 0x00000000u32,
DWB_DATA_OVERFLOW_TYPE_BUFFER            = 0x00000001u32,
DWB_DATA_OVERFLOW_TYPE_VUPDATE           = 0x00000002u32,
DWB_DATA_OVERFLOW_TYPE_VREADY            = 0x00000003u32,
}

/*
 * DWB_DEBUG_SEL_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_DEBUG_SEL_ENUM {
DWB_DEBUG_SEL_FC                         = 0x00000000u32,
DWB_DEBUG_SEL_RESERVED                   = 0x00000001u32,
DWB_DEBUG_SEL_DWBCP                      = 0x00000002u32,
DWB_DEBUG_SEL_PERFMON                    = 0x00000003u32,
}

/*
 * DWB_MEM_PWR_FORCE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_MEM_PWR_FORCE_ENUM {
DWB_MEM_PWR_FORCE_DIS                    = 0x00000000u32,
DWB_MEM_PWR_FORCE_LS                     = 0x00000001u32,
DWB_MEM_PWR_FORCE_DS                     = 0x00000002u32,
DWB_MEM_PWR_FORCE_SD                     = 0x00000003u32,
}

/*
 * DWB_MEM_PWR_STATE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_MEM_PWR_STATE_ENUM {
DWB_MEM_PWR_STATE_ON                     = 0x00000000u32,
DWB_MEM_PWR_STATE_LS                     = 0x00000001u32,
DWB_MEM_PWR_STATE_DS                     = 0x00000002u32,
DWB_MEM_PWR_STATE_SD                     = 0x00000003u32,
}

/*
 * DWB_TEST_CLK_SEL_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_TEST_CLK_SEL_ENUM {
DWB_TEST_CLK_SEL_R                       = 0x00000000u32,
DWB_TEST_CLK_SEL_G                       = 0x00000001u32,
DWB_TEST_CLK_SEL_P                       = 0x00000002u32,
}

/*
 * FC_EYE_SELECTION_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FC_EYE_SELECTION_ENUM {
FC_EYE_SELECTION_STEREO_DIS              = 0x00000000u32,
FC_EYE_SELECTION_LEFT_EYE                = 0x00000001u32,
FC_EYE_SELECTION_RIGHT_EYE               = 0x00000002u32,
}

/*
 * FC_FRAME_CAPTURE_RATE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FC_FRAME_CAPTURE_RATE_ENUM {
FC_FRAME_CAPTURE_RATE_FULL               = 0x00000000u32,
FC_FRAME_CAPTURE_RATE_HALF               = 0x00000001u32,
FC_FRAME_CAPTURE_RATE_THIRD              = 0x00000002u32,
FC_FRAME_CAPTURE_RATE_QUARTER            = 0x00000003u32,
}

/*
 * FC_STEREO_EYE_POLARITY_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum FC_STEREO_EYE_POLARITY_ENUM {
FC_STEREO_EYE_POLARITY_LEFT              = 0x00000000u32,
FC_STEREO_EYE_POLARITY_RIGHT             = 0x00000001u32,
}

/*******************************************************
 * DWBCP Enums
 *******************************************************/

/*
 * DWB_GAMUT_REMAP_COEF_FORMAT_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_GAMUT_REMAP_COEF_FORMAT_ENUM {
DWB_GAMUT_REMAP_COEF_FORMAT_S2_13        = 0x00000000u32,
DWB_GAMUT_REMAP_COEF_FORMAT_S3_12        = 0x00000001u32,
}

/*
 * DWB_GAMUT_REMAP_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_GAMUT_REMAP_MODE_ENUM {
DWB_GAMUT_REMAP_MODE_BYPASS              = 0x00000000u32,
DWB_GAMUT_REMAP_MODE_COEF_A              = 0x00000001u32,
DWB_GAMUT_REMAP_MODE_COEF_B              = 0x00000002u32,
DWB_GAMUT_REMAP_MODE_RESERVED            = 0x00000003u32,
}

/*
 * DWB_LUT_NUM_SEG enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_LUT_NUM_SEG {
DWB_SEGMENTS_1                           = 0x00000000u32,
DWB_SEGMENTS_2                           = 0x00000001u32,
DWB_SEGMENTS_4                           = 0x00000002u32,
DWB_SEGMENTS_8                           = 0x00000003u32,
DWB_SEGMENTS_16                          = 0x00000004u32,
DWB_SEGMENTS_32                          = 0x00000005u32,
DWB_SEGMENTS_64                          = 0x00000006u32,
DWB_SEGMENTS_128                         = 0x00000007u32,
}

/*
 * DWB_OGAM_LUT_CONFIG_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_OGAM_LUT_CONFIG_MODE_ENUM {
DWB_OGAM_LUT_CONFIG_MODE_DIFF            = 0x00000000u32,
DWB_OGAM_LUT_CONFIG_MODE_SAME            = 0x00000001u32,
}

/*
 * DWB_OGAM_LUT_HOST_SEL_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_OGAM_LUT_HOST_SEL_ENUM {
DWB_OGAM_LUT_HOST_SEL_RAMA               = 0x00000000u32,
DWB_OGAM_LUT_HOST_SEL_RAMB               = 0x00000001u32,
}

/*
 * DWB_OGAM_LUT_READ_COLOR_SEL_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_OGAM_LUT_READ_COLOR_SEL_ENUM {
DWB_OGAM_LUT_READ_COLOR_SEL_B            = 0x00000000u32,
DWB_OGAM_LUT_READ_COLOR_SEL_G            = 0x00000001u32,
DWB_OGAM_LUT_READ_COLOR_SEL_R            = 0x00000002u32,
DWB_OGAM_LUT_READ_COLOR_SEL_RESERVED     = 0x00000003u32,
}

/*
 * DWB_OGAM_LUT_READ_DBG_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_OGAM_LUT_READ_DBG_ENUM {
DWB_OGAM_LUT_READ_DBG_DISABLE            = 0x00000000u32,
DWB_OGAM_LUT_READ_DBG_ENABLE             = 0x00000001u32,
}

/*
 * DWB_OGAM_MODE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_OGAM_MODE_ENUM {
DWB_OGAM_MODE_BYPASS                     = 0x00000000u32,
DWB_OGAM_MODE_RESERVED                   = 0x00000001u32,
DWB_OGAM_MODE_RAM_LUT_ENABLED            = 0x00000002u32,
}

/*
 * DWB_OGAM_PWL_DISABLE_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_OGAM_PWL_DISABLE_ENUM {
DWB_OGAM_PWL_DISABLE_FALSE               = 0x00000000u32,
DWB_OGAM_PWL_DISABLE_TRUE                = 0x00000001u32,
}

/*
 * DWB_OGAM_SELECT_ENUM enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DWB_OGAM_SELECT_ENUM {
DWB_OGAM_SELECT_A                        = 0x00000000u32,
DWB_OGAM_SELECT_B                        = 0x00000001u32,
}

/*******************************************************
 * RDPCSTX Enums
 *******************************************************/

/*
 * RDPCSTX_CLOCK_CNTL_RDPCS_EXT_REFCLK_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_CLOCK_CNTL_RDPCS_EXT_REFCLK_EN {
RDPCS_EXT_REFCLK_DISABLE                 = 0x00000000u32,
RDPCS_EXT_REFCLK_ENABLE                  = 0x00000001u32,
}

/*
 * RDPCSTX_CLOCK_CNTL_RDPCS_OCLACLK_CLOCK_ON enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_CLOCK_CNTL_RDPCS_OCLACLK_CLOCK_ON {
RDPCS_OCLACLK_CLOCK_OFF                  = 0x00000000u32,
RDPCS_OCLACLK_CLOCK_ON                   = 0x00000001u32,
}

/*
 * RDPCSTX_CLOCK_CNTL_RDPCS_OCLACLK_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_CLOCK_CNTL_RDPCS_OCLACLK_EN {
RDPCS_OCLACLK_DISABLE                    = 0x00000000u32,
RDPCS_OCLACLK_ENABLE                     = 0x00000001u32,
}

/*
 * RDPCSTX_CLOCK_CNTL_RDPCS_OCLACLK_GATE_DIS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_CLOCK_CNTL_RDPCS_OCLACLK_GATE_DIS {
RDPCS_OCLACLK_GATE_ENABLE                = 0x00000000u32,
RDPCS_OCLACLK_GATE_DISABLE               = 0x00000001u32,
}

/*
 * RDPCSTX_CLOCK_CNTL_RDPCS_SRAMCLK_CLOCK_ON enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_CLOCK_CNTL_RDPCS_SRAMCLK_CLOCK_ON {
RDPCS_SYMCLK_SRAMCLK_CLOCK_OFF           = 0x00000000u32,
RDPCS_SYMCLK_SRAMCLK_CLOCK_ON            = 0x00000001u32,
}

/*
 * RDPCSTX_CLOCK_CNTL_RDPCS_SRAMCLK_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_CLOCK_CNTL_RDPCS_SRAMCLK_EN {
RDPCS_SRAMCLK_DISABLE                    = 0x00000000u32,
RDPCS_SRAMCLK_ENABLE                     = 0x00000001u32,
}

/*
 * RDPCSTX_CLOCK_CNTL_RDPCS_SRAMCLK_GATE_DIS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_CLOCK_CNTL_RDPCS_SRAMCLK_GATE_DIS {
RDPCS_SRAMCLK_GATE_ENABLE                = 0x00000000u32,
RDPCS_SRAMCLK_GATE_DISABLE               = 0x00000001u32,
}

/*
 * RDPCSTX_CLOCK_CNTL_RDPCS_SRAMCLK_PASS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_CLOCK_CNTL_RDPCS_SRAMCLK_PASS {
RDPCS_SRAMCLK_NOT_PASS                   = 0x00000000u32,
RDPCS_SRAMCLK_PASS                       = 0x00000001u32,
}

/*
 * RDPCSTX_CLOCK_CNTL_RDPCS_TX_CLK_CLOCK_ON enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_CLOCK_CNTL_RDPCS_TX_CLK_CLOCK_ON {
RDPCS_TX_CLK_CLOCK_OFF                   = 0x00000000u32,
RDPCS_TX_CLK_CLOCK_ON                    = 0x00000001u32,
}

/*
 * RDPCSTX_CLOCK_CNTL_RDPCS_TX_CLK_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_CLOCK_CNTL_RDPCS_TX_CLK_EN {
RDPCS_TX_CLK_DISABLE                     = 0x00000000u32,
RDPCS_TX_CLK_ENABLE                      = 0x00000001u32,
}

/*
 * RDPCSTX_CLOCK_CNTL_RDPCS_TX_CLK_GATE_DIS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_CLOCK_CNTL_RDPCS_TX_CLK_GATE_DIS {
RDPCS_TX_CLK_GATE_ENABLE                 = 0x00000000u32,
RDPCS_TX_CLK_GATE_DISABLE                = 0x00000001u32,
}

/*
 * RDPCSTX_CLOCK_CNTL_TX_CLK_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_CLOCK_CNTL_TX_CLK_EN {
RDPCS_EXT_REFCLK_EN_DISABLE              = 0x00000000u32,
RDPCS_EXT_REFCLK_EN_ENABLE               = 0x00000001u32,
}

/*
 * RDPCSTX_CNTL_RDPCS_CBUS_SOFT_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_CNTL_RDPCS_CBUS_SOFT_RESET {
RDPCS_CBUS_SOFT_RESET_DISABLE            = 0x00000000u32,
RDPCS_CBUS_SOFT_RESET_ENABLE             = 0x00000001u32,
}

/*
 * RDPCSTX_CNTL_RDPCS_SRAM_SOFT_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_CNTL_RDPCS_SRAM_SOFT_RESET {
RDPCS_SRAM_SRAM_RESET_DISABLE            = 0x00000000u32,
RDPCS_SRAM_SRAM_RESET_ENABLE             = 0x00000001u32,
}

/*
 * RDPCSTX_CNTL_RDPCS_TX_FIFO_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_CNTL_RDPCS_TX_FIFO_EN {
RDPCS_TX_FIFO_DISABLE                    = 0x00000000u32,
RDPCS_TX_FIFO_ENABLE                     = 0x00000001u32,
}

/*
 * RDPCSTX_CNTL_RDPCS_TX_FIFO_LANE_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_CNTL_RDPCS_TX_FIFO_LANE_EN {
RDPCS_TX_FIFO_LANE_DISABLE               = 0x00000000u32,
RDPCS_TX_FIFO_LANE_ENABLE                = 0x00000001u32,
}

/*
 * RDPCSTX_CNTL_RDPCS_TX_SOFT_RESET enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_CNTL_RDPCS_TX_SOFT_RESET {
RDPCS_TX_SOFT_RESET_DISABLE              = 0x00000000u32,
RDPCS_TX_SOFT_RESET_ENABLE               = 0x00000001u32,
}

/*
 * RDPCSTX_FIFO_EMPTY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_FIFO_EMPTY {
RDPCSTX_FIFO_NOT_EMPTY                   = 0x00000000u32,
RDPCSTX_FIFO_IS_EMPTY                    = 0x00000001u32,
}

/*
 * RDPCSTX_FIFO_FULL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_FIFO_FULL {
RDPCSTX_FIFO_NOT_FULL                    = 0x00000000u32,
RDPCSTX_FIFO_IS_FULL                     = 0x00000001u32,
}

/*
 * RDPCSTX_INTERRUPT_CONTROL_RDPCS_DPALT_4LANE_TOGGLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_INTERRUPT_CONTROL_RDPCS_DPALT_4LANE_TOGGLE {
RDPCS_DPALT_4LANE_TOGGLE_2LANE           = 0x00000000u32,
RDPCS_DPALT_4LANE_TOGGLE_4LANE           = 0x00000001u32,
}

/*
 * RDPCSTX_INTERRUPT_CONTROL_RDPCS_DPALT_4LANE_TOGGLE_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_INTERRUPT_CONTROL_RDPCS_DPALT_4LANE_TOGGLE_MASK {
RDPCS_DPALT_4LANE_TOGGLE_MASK_DISABLE    = 0x00000000u32,
RDPCS_DPALT_4LANE_TOGGLE_MASK_ENABLE     = 0x00000001u32,
}

/*
 * RDPCSTX_INTERRUPT_CONTROL_RDPCS_DPALT_DISABLE_TOGGLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_INTERRUPT_CONTROL_RDPCS_DPALT_DISABLE_TOGGLE {
RDPCS_DPALT_DISABLE_TOGGLE_ENABLE        = 0x00000000u32,
RDPCS_DPALT_DISABLE_TOGGLE_DISABLE       = 0x00000001u32,
}

/*
 * RDPCSTX_INTERRUPT_CONTROL_RDPCS_DPALT_DISABLE_TOGGLE_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_INTERRUPT_CONTROL_RDPCS_DPALT_DISABLE_TOGGLE_MASK {
RDPCS_DPALT_DISABLE_TOGGLE_MASK_DISABLE  = 0x00000000u32,
RDPCS_DPALT_DISABLE_TOGGLE_MASK_ENABLE   = 0x00000001u32,
}

/*
 * RDPCSTX_INTERRUPT_CONTROL_RDPCS_REG_FIFO_ERROR_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_INTERRUPT_CONTROL_RDPCS_REG_FIFO_ERROR_MASK {
RDPCS_REG_FIFO_ERROR_MASK_DISABLE        = 0x00000000u32,
RDPCS_REG_FIFO_ERROR_MASK_ENABLE         = 0x00000001u32,
}

/*
 * RDPCSTX_INTERRUPT_CONTROL_RDPCS_TX_FIFO_ERROR_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_INTERRUPT_CONTROL_RDPCS_TX_FIFO_ERROR_MASK {
RDPCS_TX_FIFO_ERROR_MASK_DISABLE         = 0x00000000u32,
RDPCS_TX_FIFO_ERROR_MASK_ENABLE          = 0x00000001u32,
}

/*
 * RDPCSTX_PHY_CNTL0_RDPCS_PHY_CR_MUX_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_PHY_CNTL0_RDPCS_PHY_CR_MUX_SEL {
RDPCS_PHY_CR_MUX_SEL_FOR_USB             = 0x00000000u32,
RDPCS_PHY_CR_MUX_SEL_FOR_DC              = 0x00000001u32,
}

/*
 * RDPCSTX_PHY_CNTL0_RDPCS_PHY_CR_PARA_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_PHY_CNTL0_RDPCS_PHY_CR_PARA_SEL {
RDPCS_PHY_CR_PARA_SEL_JTAG               = 0x00000000u32,
RDPCS_PHY_CR_PARA_SEL_CR                 = 0x00000001u32,
}

/*
 * RDPCSTX_PHY_CNTL0_RDPCS_PHY_REF_RANGE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_PHY_CNTL0_RDPCS_PHY_REF_RANGE {
RDPCS_PHY_REF_RANGE_0                    = 0x00000000u32,
RDPCS_PHY_REF_RANGE_1                    = 0x00000001u32,
RDPCS_PHY_REF_RANGE_2                    = 0x00000002u32,
RDPCS_PHY_REF_RANGE_3                    = 0x00000003u32,
RDPCS_PHY_REF_RANGE_4                    = 0x00000004u32,
RDPCS_PHY_REF_RANGE_5                    = 0x00000005u32,
RDPCS_PHY_REF_RANGE_6                    = 0x00000006u32,
RDPCS_PHY_REF_RANGE_7                    = 0x00000007u32,
}

/*
 * RDPCSTX_PHY_CNTL0_RDPCS_SRAM_EXT_LD_DONE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_PHY_CNTL0_RDPCS_SRAM_EXT_LD_DONE {
RDPCS_SRAM_EXT_LD_NOT_DONE               = 0x00000000u32,
RDPCS_SRAM_EXT_LD_DONE                   = 0x00000001u32,
}

/*
 * RDPCSTX_PHY_CNTL0_RDPCS_SRAM_INIT_DONE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_PHY_CNTL0_RDPCS_SRAM_INIT_DONE {
RDPCS_SRAM_INIT_NOT_DONE                 = 0x00000000u32,
RDPCS_SRAM_INIT_DONE                     = 0x00000001u32,
}

/*
 * RDPCSTX_PHY_CNTL11_RDPCS_PHY_DP_REF_CLK_MPLLB_DIV enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_PHY_CNTL11_RDPCS_PHY_DP_REF_CLK_MPLLB_DIV {
RDPCS_PHY_DP_REF_CLK_MPLLB_DIV1          = 0x00000000u32,
RDPCS_PHY_DP_REF_CLK_MPLLB_DIV2          = 0x00000001u32,
RDPCS_PHY_DP_REF_CLK_MPLLB_DIV3          = 0x00000002u32,
RDPCS_PHY_DP_REF_CLK_MPLLB_DIV8          = 0x00000003u32,
RDPCS_PHY_DP_REF_CLK_MPLLB_DIV16         = 0x00000004u32,
}

/*
 * RDPCSTX_PHY_CNTL11_RDPCS_PHY_HDMI_MPLLB_HDMI_PIXEL_CLK_DIV enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_PHY_CNTL11_RDPCS_PHY_HDMI_MPLLB_HDMI_PIXEL_CLK_DIV {
RDPCS_PHY_HDMI_MPLLB_HDMI_PIXEL_CLK_DIV_0 = 0x00000000u32,
RDPCS_PHY_HDMI_MPLLB_HDMI_PIXEL_CLK_DIV_1 = 0x00000001u32,
RDPCS_PHY_HDMI_MPLLB_HDMI_PIXEL_CLK_DIV_2 = 0x00000002u32,
RDPCS_PHY_HDMI_MPLLB_HDMI_PIXEL_CLK_DIV_3 = 0x00000003u32,
}

/*
 * RDPCSTX_PHY_CNTL12_RDPCS_PHY_DP_MPLLB_TX_CLK_DIV enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_PHY_CNTL12_RDPCS_PHY_DP_MPLLB_TX_CLK_DIV {
RDPCS_PHY_DP_MPLLB_TX_CLK_DIV            = 0x00000000u32,
RDPCS_PHY_DP_MPLLB_TX_CLK_DIV2           = 0x00000001u32,
RDPCS_PHY_DP_MPLLB_TX_CLK_DIV4           = 0x00000002u32,
RDPCS_PHY_DP_MPLLB_TX_CLK_DIV8           = 0x00000003u32,
RDPCS_PHY_DP_MPLLB_TX_CLK_DIV3           = 0x00000004u32,
RDPCS_PHY_DP_MPLLB_TX_CLK_DIV5           = 0x00000005u32,
RDPCS_PHY_DP_MPLLB_TX_CLK_DIV6           = 0x00000006u32,
RDPCS_PHY_DP_MPLLB_TX_CLK_DIV10          = 0x00000007u32,
}

/*
 * RDPCSTX_PHY_CNTL4_RDPCS_PHY_DP_TX_TERM_CTRL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_PHY_CNTL4_RDPCS_PHY_DP_TX_TERM_CTRL {
RDPCS_PHY_DP_TX_TERM_CTRL_54             = 0x00000000u32,
RDPCS_PHY_DP_TX_TERM_CTRL_52             = 0x00000001u32,
RDPCS_PHY_DP_TX_TERM_CTRL_50             = 0x00000002u32,
RDPCS_PHY_DP_TX_TERM_CTRL_48             = 0x00000003u32,
RDPCS_PHY_DP_TX_TERM_CTRL_46             = 0x00000004u32,
RDPCS_PHY_DP_TX_TERM_CTRL_44             = 0x00000005u32,
RDPCS_PHY_DP_TX_TERM_CTRL_42             = 0x00000006u32,
RDPCS_PHY_DP_TX_TERM_CTRL_40             = 0x00000007u32,
}

/*
 * RDPCSTX_PHY_CNTL_RDPCS_PHY_DP_TX_DETRX_RESULT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_PHY_CNTL_RDPCS_PHY_DP_TX_DETRX_RESULT {
RDPCS_PHY_DP_TX_DETRX_RESULT_NO_DETECT   = 0x00000000u32,
RDPCS_PHY_DP_TX_DETRX_RESULT_DETECT      = 0x00000001u32,
}

/*
 * RDPCSTX_PHY_CNTL_RDPCS_PHY_DP_TX_RATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_PHY_CNTL_RDPCS_PHY_DP_TX_RATE {
RDPCS_PHY_DP_TX_RATE                     = 0x00000000u32,
RDPCS_PHY_DP_TX_RATE_DIV2                = 0x00000001u32,
RDPCS_PHY_DP_TX_RATE_DIV4                = 0x00000002u32,
}

/*
 * RDPCSTX_PHY_CNTL_RDPCS_PHY_DP_TX_WIDTH enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_PHY_CNTL_RDPCS_PHY_DP_TX_WIDTH {
RDPCS_PHY_DP_TX_WIDTH_8                  = 0x00000000u32,
RDPCS_PHY_DP_TX_WIDTH_10                 = 0x00000001u32,
RDPCS_PHY_DP_TX_WIDTH_16                 = 0x00000002u32,
RDPCS_PHY_DP_TX_WIDTH_20                 = 0x00000003u32,
}

/*
 * RDPCSTX_PHY_CNTL_RRDPCS_PHY_DP_TX_PSTATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_PHY_CNTL_RRDPCS_PHY_DP_TX_PSTATE {
RRDPCS_PHY_DP_TX_PSTATE_POWER_UP         = 0x00000000u32,
RRDPCS_PHY_DP_TX_PSTATE_HOLD             = 0x00000001u32,
RRDPCS_PHY_DP_TX_PSTATE_HOLD_OFF         = 0x00000002u32,
RRDPCS_PHY_DP_TX_PSTATE_POWER_DOWN       = 0x00000003u32,
}

/*
 * RDPCSTX_PHY_REF_ALT_CLK_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_PHY_REF_ALT_CLK_EN {
RDPCS_PHY_REF_ALT_CLK_DISABLE            = 0x00000000u32,
RDPCS_PHY_REF_ALT_CLK_ENABLE             = 0x00000001u32,
}

/*
 * RDPCSTX_TX_FIFO_DISABLED_MASK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCSTX_TX_FIFO_DISABLED_MASK {
RDPCSTX_TX_FIFO_DISABLED_MASK_DISABLE    = 0x00000000u32,
RDPCSTX_TX_FIFO_DISABLED_MASK_ENABLE     = 0x00000001u32,
}

/*
 * RDPCS_DBG_OCLA_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCS_DBG_OCLA_SEL {
RDPCS_DBG_OCLA_SEL_MON_OUT_7_0           = 0x00000000u32,
RDPCS_DBG_OCLA_SEL_MON_OUT_15_8          = 0x00000001u32,
RDPCS_DBG_OCLA_SEL_MON_OUT_23_16         = 0x00000002u32,
RDPCS_DBG_OCLA_SEL_MON_OUT_31_24         = 0x00000003u32,
RDPCS_DBG_OCLA_SEL_MON_OUT_39_32         = 0x00000004u32,
RDPCS_DBG_OCLA_SEL_MON_OUT_47_40         = 0x00000005u32,
RDPCS_DBG_OCLA_SEL_MON_OUT_55_48         = 0x00000006u32,
RDPCS_DBG_OCLA_SEL_MON_OUT_63_56         = 0x00000007u32,
}

/*
 * RDPCS_TEST_CLK_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCS_TEST_CLK_SEL {
RDPCS_TEST_CLK_SEL_NONE                  = 0x00000000u32,
RDPCS_TEST_CLK_SEL_CFGCLK                = 0x00000001u32,
RDPCS_TEST_CLK_SEL_SYMCLK_DIV2_LDPCS     = 0x00000002u32,
RDPCS_TEST_CLK_SEL_SYMCLK_DIV2_RDPCS     = 0x00000003u32,
RDPCS_TEST_CLK_SEL_SYMCLK_DIV2_LDPCS_DIV4 = 0x00000004u32,
RDPCS_TEST_CLK_SEL_SYMCLK_DIV2_RDPCS_DIV4 = 0x00000005u32,
RDPCS_TEST_CLK_SEL_SRAMCLK               = 0x00000006u32,
RDPCS_TEST_CLK_SEL_EXT_CR_CLK            = 0x00000007u32,
RDPCS_TEST_CLK_SEL_DP_TX0_WORD_CLK       = 0x00000008u32,
RDPCS_TEST_CLK_SEL_DP_TX1_WORD_CLK       = 0x00000009u32,
RDPCS_TEST_CLK_SEL_DP_TX2_WORD_CLK       = 0x0000000au32,
RDPCS_TEST_CLK_SEL_DP_TX3_WORD_CLK       = 0x0000000bu32,
RDPCS_TEST_CLK_SEL_DP_MPLLB_DIV_CLK      = 0x0000000cu32,
RDPCS_TEST_CLK_SEL_HDMI_MPLLB_HDMI_PIXEL_CLK = 0x0000000du32,
RDPCS_TEST_CLK_SEL_PHY_REF_DIG_CLK       = 0x0000000eu32,
RDPCS_TEST_CLK_SEL_REF_DIG_FR_clk        = 0x0000000fu32,
RDPCS_TEST_CLK_SEL_dtb_out0              = 0x00000010u32,
RDPCS_TEST_CLK_SEL_dtb_out1              = 0x00000011u32,
}

/*
 * RDPCS_TX_CNTL_TX_LANE_PACK_FROM_MSB enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCS_TX_CNTL_TX_LANE_PACK_FROM_MSB {
RDPCS_LANE_PACK_FROM_MSB_DISABLE         = 0x00000000u32,
RDPCS_LANE_PACK_FROM_MSB_ENABLE          = 0x00000001u32,
}

/*
 * RDPCS_TX_SRAM_CNTL_RDPCS_MEM_PWR_FORCE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCS_TX_SRAM_CNTL_RDPCS_MEM_PWR_FORCE {
RDPCS_MEM_PWR_NO_FORCE                   = 0x00000000u32,
RDPCS_MEM_PWR_LIGHT_SLEEP                = 0x00000001u32,
RDPCS_MEM_PWR_DEEP_SLEEP                 = 0x00000002u32,
RDPCS_MEM_PWR_SHUT_DOWN                  = 0x00000003u32,
}

/*
 * RDPCS_TX_SRAM_CNTL_RDPCS_MEM_PWR_PWR_STATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RDPCS_TX_SRAM_CNTL_RDPCS_MEM_PWR_PWR_STATE {
RDPCS_MEM_PWR_PWR_STATE_ON               = 0x00000000u32,
RDPCS_MEM_PWR_PWR_STATE_LIGHT_SLEEP      = 0x00000001u32,
RDPCS_MEM_PWR_PWR_STATE_DEEP_SLEEP       = 0x00000002u32,
RDPCS_MEM_PWR_PWR_STATE_SHUT_DOWN        = 0x00000003u32,
}

/*
 * RPDCSTX_CNTL_TX_LANE_BIT_ORDER_REVERSE_BEFORE_PACK enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RPDCSTX_CNTL_TX_LANE_BIT_ORDER_REVERSE_BEFORE_PACK {
RDPCS_LANE_BIT_ORDER_REVERSE_DISABLE     = 0x00000000u32,
RDPCS_LANE_BIT_ORDER_REVERSE_ENABLE      = 0x00000001u32,
}

/*******************************************************
 * RLC Enums
 *******************************************************/

/*
 * RLC_DOORBELL_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RLC_DOORBELL_MODE {
RLC_DOORBELL_MODE_DISABLE                = 0x00000000u32,
RLC_DOORBELL_MODE_ENABLE                 = 0x00000001u32,
RLC_DOORBELL_MODE_ENABLE_PF              = 0x00000002u32,
RLC_DOORBELL_MODE_ENABLE_PF_VF           = 0x00000003u32,
}

/*
 * RLC_PERFCOUNTER_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RLC_PERFCOUNTER_SEL {
RLC_PERF_SEL_POWER_FEATURE_0             = 0x00000000u32,
RLC_PERF_SEL_POWER_FEATURE_1             = 0x00000001u32,
RLC_PERF_SEL_CP_INTERRUPT                = 0x00000002u32,
RLC_PERF_SEL_GRBM_INTERRUPT              = 0x00000003u32,
RLC_PERF_SEL_SPM_INTERRUPT               = 0x00000004u32,
RLC_PERF_SEL_IH_INTERRUPT                = 0x00000005u32,
RLC_PERF_SEL_SERDES_COMMAND_WRITE        = 0x00000006u32,
}

/*
 * RLC_PERFMON_STATE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RLC_PERFMON_STATE {
RLC_PERFMON_STATE_RESET                  = 0x00000000u32,
RLC_PERFMON_STATE_ENABLE                 = 0x00000001u32,
RLC_PERFMON_STATE_DISABLE                = 0x00000002u32,
RLC_PERFMON_STATE_RESERVED_3             = 0x00000003u32,
RLC_PERFMON_STATE_RESERVED_4             = 0x00000004u32,
RLC_PERFMON_STATE_RESERVED_5             = 0x00000005u32,
RLC_PERFMON_STATE_RESERVED_6             = 0x00000006u32,
RLC_PERFMON_STATE_ROLLOVER               = 0x00000007u32,
}

/*
 * RSPM_CMD enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RSPM_CMD {
RSPM_CMD_INVALID                         = 0x00000000u32,
RSPM_CMD_IDLE                            = 0x00000001u32,
RSPM_CMD_CALIBRATE                       = 0x00000002u32,
RSPM_CMD_SPM_RESET                       = 0x00000003u32,
RSPM_CMD_SPM_START                       = 0x00000004u32,
RSPM_CMD_SPM_STOP                        = 0x00000005u32,
RSPM_CMD_PERF_RESET                      = 0x00000006u32,
RSPM_CMD_PERF_SAMPLE                     = 0x00000007u32,
RSPM_CMD_PROF_START                      = 0x00000008u32,
RSPM_CMD_PROF_STOP                       = 0x00000009u32,
RSPM_CMD_FORCE_SAMPLE                    = 0x0000000au32,
}

/*******************************************************
 * COMP Enums
 *******************************************************/

/*
 * CSCNTL_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CSCNTL_TYPE {
CSCNTL_TYPE_TG                           = 0x00000000u32,
CSCNTL_TYPE_STATE                        = 0x00000001u32,
CSCNTL_TYPE_EVENT                        = 0x00000002u32,
CSCNTL_TYPE_PRIVATE                      = 0x00000003u32,
}

/*
 * CSDATA_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CSDATA_TYPE {
CSDATA_TYPE_TG                           = 0x00000000u32,
CSDATA_TYPE_STATE                        = 0x00000001u32,
CSDATA_TYPE_EVENT                        = 0x00000002u32,
CSDATA_TYPE_PRIVATE                      = 0x00000003u32,
}

/*
 * CSDATA_TYPE_WIDTH value
 */

#define CSDATA_TYPE_WIDTH              0x00000002

/*
 * CSDATA_ADDR_WIDTH value
 */

#define CSDATA_ADDR_WIDTH              0x00000007

/*
 * CSDATA_DATA_WIDTH value
 */

#define CSDATA_DATA_WIDTH              0x00000020

/*
 * CSCNTL_TYPE_WIDTH value
 */

#define CSCNTL_TYPE_WIDTH              0x00000002

/*
 * CSCNTL_ADDR_WIDTH value
 */

#define CSCNTL_ADDR_WIDTH              0x00000007

/*
 * CSCNTL_DATA_WIDTH value
 */

#define CSCNTL_DATA_WIDTH              0x00000020

/*******************************************************
 * GE Enums
 *******************************************************/

/*
 * GE1_PERFCOUNT_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GE1_PERFCOUNT_SELECT {
ge1_assembler_busy                       = 0x00000000u32,
ge1_assembler_stalled                    = 0x00000001u32,
ge1_dma_busy                             = 0x00000002u32,
ge1_dma_lat_bin_0                        = 0x00000003u32,
ge1_dma_lat_bin_1                        = 0x00000004u32,
ge1_dma_lat_bin_2                        = 0x00000005u32,
ge1_dma_lat_bin_3                        = 0x00000006u32,
ge1_dma_lat_bin_4                        = 0x00000007u32,
ge1_dma_lat_bin_5                        = 0x00000008u32,
ge1_dma_lat_bin_6                        = 0x00000009u32,
ge1_dma_lat_bin_7                        = 0x0000000au32,
ge1_dma_return_cl0                       = 0x0000000bu32,
ge1_dma_return_cl1                       = 0x0000000cu32,
ge1_dma_utcl1_consecutive_retry_event    = 0x0000000du32,
ge1_dma_utcl1_request_event              = 0x0000000eu32,
ge1_dma_utcl1_retry_event                = 0x0000000fu32,
ge1_dma_utcl1_stall_event                = 0x00000010u32,
ge1_dma_utcl1_stall_utcl2_event          = 0x00000011u32,
ge1_dma_utcl1_translation_hit_event      = 0x00000012u32,
ge1_dma_utcl1_translation_miss_event     = 0x00000013u32,
ge1_assembler_dma_starved                = 0x00000014u32,
ge1_rbiu_di_fifo_stalled_p0              = 0x00000015u32,
ge1_rbiu_di_fifo_starved_p0              = 0x00000016u32,
ge1_rbiu_dr_fifo_stalled_p0              = 0x00000017u32,
ge1_rbiu_dr_fifo_starved_p0              = 0x00000018u32,
ge1_sclk_reg_vld                         = 0x00000019u32,
ge1_stat_busy                            = 0x0000001au32,
ge1_stat_no_dma_busy                     = 0x0000001bu32,
ge1_pipe0_to_pipe1                       = 0x0000001cu32,
ge1_pipe1_to_pipe0                       = 0x0000001du32,
ge1_dma_return_size_cl0                  = 0x0000001eu32,
ge1_dma_return_size_cl1                  = 0x0000001fu32,
ge1_small_draws_one_instance             = 0x00000020u32,
ge1_sclk_input_vld                       = 0x00000021u32,
ge1_prim_group_limit_hit                 = 0x00000022u32,
ge1_unopt_multi_instance_draws           = 0x00000023u32,
ge1_rbiu_di_fifo_stalled_p1              = 0x00000024u32,
ge1_rbiu_di_fifo_starved_p1              = 0x00000025u32,
ge1_rbiu_dr_fifo_stalled_p1              = 0x00000026u32,
ge1_rbiu_dr_fifo_starved_p1              = 0x00000027u32,
}

/*
 * GE2_DIST_PERFCOUNT_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GE2_DIST_PERFCOUNT_SELECT {
ge_dist_hs_done                          = 0x00000000u32,
ge_dist_hs_done_latency_se0              = 0x00000001u32,
ge_dist_hs_done_latency_se1              = 0x00000002u32,
ge_dist_hs_done_latency_se2              = 0x00000003u32,
ge_dist_hs_done_latency_se3              = 0x00000004u32,
ge_dist_hs_done_latency_se4              = 0x00000005u32,
ge_dist_hs_done_latency_se5              = 0x00000006u32,
ge_dist_hs_done_latency_se6              = 0x00000007u32,
ge_dist_hs_done_latency_se7              = 0x00000008u32,
ge_dist_inside_tf_bin_0                  = 0x00000009u32,
ge_dist_inside_tf_bin_1                  = 0x0000000au32,
ge_dist_inside_tf_bin_2                  = 0x0000000bu32,
ge_dist_inside_tf_bin_3                  = 0x0000000cu32,
ge_dist_inside_tf_bin_4                  = 0x0000000du32,
ge_dist_inside_tf_bin_5                  = 0x0000000eu32,
ge_dist_inside_tf_bin_6                  = 0x0000000fu32,
ge_dist_inside_tf_bin_7                  = 0x00000010u32,
ge_dist_inside_tf_bin_8                  = 0x00000011u32,
ge_dist_null_patch                       = 0x00000012u32,
ge_dist_sclk_core_vld                    = 0x00000013u32,
ge_dist_sclk_wd_te11_vld                 = 0x00000014u32,
ge_dist_tfreq_lat_bin_0                  = 0x00000015u32,
ge_dist_tfreq_lat_bin_1                  = 0x00000016u32,
ge_dist_tfreq_lat_bin_2                  = 0x00000017u32,
ge_dist_tfreq_lat_bin_3                  = 0x00000018u32,
ge_dist_tfreq_lat_bin_4                  = 0x00000019u32,
ge_dist_tfreq_lat_bin_5                  = 0x0000001au32,
ge_dist_tfreq_lat_bin_6                  = 0x0000001bu32,
ge_dist_tfreq_lat_bin_7                  = 0x0000001cu32,
ge_dist_tfreq_utcl1_consecutive_retry_event = 0x0000001du32,
ge_dist_tfreq_utcl1_request_event        = 0x0000001eu32,
ge_dist_tfreq_utcl1_retry_event          = 0x0000001fu32,
ge_dist_tfreq_utcl1_stall_event          = 0x00000020u32,
ge_dist_tfreq_utcl1_stall_utcl2_event    = 0x00000021u32,
ge_dist_tfreq_utcl1_translation_hit_event = 0x00000022u32,
ge_dist_tfreq_utcl1_translation_miss_event = 0x00000023u32,
ge_dist_pc_feorder_fifo_full             = 0x00000024u32,
ge_dist_pc_ge_manager_busy               = 0x00000025u32,
ge_dist_sclk_input_vld                   = 0x00000026u32,
ge_dist_wd_te11_busy                     = 0x00000027u32,
ge_dist_te11_starved                     = 0x00000028u32,
ge_dist_switch_mode_stall                = 0x00000029u32,
ge_all_tf_eq                             = 0x0000002au32,
ge_all_tf2                               = 0x0000002bu32,
ge_all_tf3                               = 0x0000002cu32,
ge_all_tf4                               = 0x0000002du32,
ge_all_tf5                               = 0x0000002eu32,
ge_all_tf6                               = 0x0000002fu32,
ge_se0_te11_starved_on_hs_done           = 0x00000030u32,
ge_se1_te11_starved_on_hs_done           = 0x00000031u32,
ge_se2_te11_starved_on_hs_done           = 0x00000032u32,
ge_se3_te11_starved_on_hs_done           = 0x00000033u32,
ge_se4_te11_starved_on_hs_done           = 0x00000034u32,
ge_se5_te11_starved_on_hs_done           = 0x00000035u32,
ge_se6_te11_starved_on_hs_done           = 0x00000036u32,
ge_se7_te11_starved_on_hs_done           = 0x00000037u32,
ge_dist_op_fifo_full_starve              = 0x00000038u32,
ge_dist_hs_done_se0                      = 0x00000039u32,
ge_dist_hs_done_se1                      = 0x0000003au32,
ge_dist_hs_done_se2                      = 0x0000003bu32,
ge_dist_hs_done_se3                      = 0x0000003cu32,
ge_dist_hs_done_se4                      = 0x0000003du32,
ge_dist_hs_done_se5                      = 0x0000003eu32,
ge_dist_hs_done_se6                      = 0x0000003fu32,
ge_dist_hs_done_se7                      = 0x00000040u32,
ge_dist_hs_done_latency                  = 0x00000041u32,
ge_dist_distributer_busy                 = 0x00000042u32,
ge_tf_ret_data_stalling_hs_done          = 0x00000043u32,
ge_num_of_no_dist_patches                = 0x00000044u32,
ge_num_of_donut_dist_patches             = 0x00000045u32,
ge_num_of_patch_dist_patches             = 0x00000046u32,
ge_num_of_se_switches_due_to_patch_accum = 0x00000047u32,
ge_num_of_se_switches_due_to_donut       = 0x00000048u32,
ge_num_of_se_switches_due_to_trap        = 0x00000049u32,
ge_num_of_hs_dealloc_events              = 0x0000004au32,
ge_agm_gcr_req                           = 0x0000004bu32,
ge_agm_gcr_tag_stall                     = 0x0000004cu32,
ge_agm_gcr_crd_stall                     = 0x0000004du32,
ge_agm_gcr_stall                         = 0x0000004eu32,
ge_agm_gcr_latency                       = 0x0000004fu32,
ge_distclk_vld                           = 0x00000050u32,
ge_dist_indx_fifos_full_and_empty        = 0x00000051u32,
ge_hs_done_all_tf0_se0                   = 0x00000052u32,
ge_hs_done_all_tf0_se1                   = 0x00000053u32,
ge_hs_done_all_tf0_se2                   = 0x00000054u32,
ge_hs_done_all_tf0_se3                   = 0x00000055u32,
ge_hs_done_all_tf0_se4                   = 0x00000056u32,
ge_hs_done_all_tf0_se5                   = 0x00000057u32,
ge_hs_done_all_tf0_se6                   = 0x00000058u32,
ge_hs_done_all_tf0_se7                   = 0x00000059u32,
ge_hs_done_all_tf1_se0                   = 0x0000005au32,
ge_hs_done_all_tf1_se1                   = 0x0000005bu32,
ge_hs_done_all_tf1_se2                   = 0x0000005cu32,
ge_hs_done_all_tf1_se3                   = 0x0000005du32,
ge_hs_done_all_tf1_se4                   = 0x0000005eu32,
ge_hs_done_all_tf1_se5                   = 0x0000005fu32,
ge_hs_done_all_tf1_se6                   = 0x00000060u32,
ge_hs_done_all_tf1_se7                   = 0x00000061u32,
ge_agm_gcr_req_outstanding               = 0x00000062u32,
ge_agm_gcr_req_amount                    = 0x00000063u32,
ge_agm_gcr_combine                       = 0x00000064u32,
}

/*
 * GE2_SE_PERFCOUNT_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GE2_SE_PERFCOUNT_SELECT {
ge_se_ds_prims                           = 0x00000000u32,
ge_se_es_thread_groups                   = 0x00000001u32,
ge_se_esvert_stalled_gsprim              = 0x00000002u32,
ge_se_hs_tfm_stall                       = 0x00000003u32,
ge_se_hs_tgs_active_high_water_mark      = 0x00000004u32,
ge_se_hs_thread_groups                   = 0x00000005u32,
ge_se_reused_es_indices                  = 0x00000006u32,
ge_se_sclk_ngg_vld                       = 0x00000007u32,
ge_se_sclk_te11_vld                      = 0x00000008u32,
ge_se_spi_esvert_eov                     = 0x00000009u32,
ge_se_spi_esvert_stalled                 = 0x0000000au32,
ge_se_spi_esvert_starved_busy            = 0x0000000bu32,
ge_se_spi_esvert_valid                   = 0x0000000cu32,
ge_se_spi_gsprim_cont                    = 0x0000000du32,
ge_se_spi_gsprim_eov                     = 0x0000000eu32,
ge_se_spi_gsprim_stalled                 = 0x0000000fu32,
ge_se_spi_gsprim_starved_busy            = 0x00000010u32,
ge_se_spi_gsprim_valid                   = 0x00000011u32,
ge_se_spi_gssubgrp_is_event              = 0x00000012u32,
ge_se_spi_gssubgrp_send                  = 0x00000013u32,
ge_se_spi_hsvert_eov                     = 0x00000014u32,
ge_se_spi_hsvert_stalled                 = 0x00000015u32,
ge_se_spi_hsvert_starved_busy            = 0x00000016u32,
ge_se_spi_hsvert_valid                   = 0x00000017u32,
ge_se_spi_hsgrp_is_event                 = 0x00000018u32,
ge_se_spi_hsgrp_send                     = 0x00000019u32,
ge_se_spi_lsvert_eov                     = 0x0000001au32,
ge_se_spi_lsvert_stalled                 = 0x0000001bu32,
ge_se_spi_lsvert_starved_busy            = 0x0000001cu32,
ge_se_spi_lsvert_valid                   = 0x0000001du32,
ge_se_spi_hsvert_fifo_full_stall         = 0x0000001eu32,
ge_se_spi_tgrp_fifo_stall                = 0x0000001fu32,
ge_spi_hsgrp_spi_stall                   = 0x00000020u32,
ge_se_spi_gssubgrp_event_window_active   = 0x00000021u32,
ge_se_hs_input_stall                     = 0x00000022u32,
ge_se_sending_vert_or_prim               = 0x00000023u32,
ge_se_sclk_input_vld                     = 0x00000024u32,
ge_spi_lswave_fifo_full_stall            = 0x00000025u32,
ge_spi_hswave_fifo_full_stall            = 0x00000026u32,
ge_hs_tif_stall                          = 0x00000027u32,
ge_csb_spi_bp                            = 0x00000028u32,
ge_ngg_starving_for_wave_id              = 0x00000029u32,
ge_pa0_csb_eop                           = 0x0000002au32,
ge_ngg_starved_idle                      = 0x0000002bu32,
ge_gsprim_send                           = 0x0000002cu32,
ge_esvert_send                           = 0x0000002du32,
ge_ngg_starved_after_work                = 0x0000002eu32,
ge_ngg_subgrp_fifo_stall                 = 0x0000002fu32,
ge_ngg_ord_id_req_stall                  = 0x00000030u32,
ge_ngg_indx_bus_stall                    = 0x00000031u32,
ge_hs_stall_tfmm_fifo_full               = 0x00000032u32,
ge_gs_issue_rtr_stalled                  = 0x00000033u32,
ge_gsprim_stalled_esvert                 = 0x00000034u32,
ge_gsthread_stalled                      = 0x00000035u32,
ge_ngg_attr_grp_alloc                    = 0x00000036u32,
ge_ngg_attr_discard_alloc                = 0x00000037u32,
ge_ngg_pc_space_not_avail                = 0x00000038u32,
ge_ngg_agm_req_stall                     = 0x00000039u32,
ge_ngg_spi_esvert_partial_eov            = 0x0000003au32,
ge_ngg_spi_gsprim_partial_eov            = 0x0000003bu32,
ge_spi_gsgrp_valid                       = 0x0000003cu32,
ge_ngg_attr_grp_latency                  = 0x0000003du32,
ge_ngg_reuse_prim_limit_hit              = 0x0000003eu32,
ge_ngg_reuse_vert_limit_hit              = 0x0000003fu32,
ge_te11_con_stall                        = 0x00000040u32,
ge_te11_compactor_starved                = 0x00000041u32,
ge_ngg_stall_tess_off_tess_on            = 0x00000042u32,
ge_ngg_stall_tess_on_tess_off            = 0x00000043u32,
ge_merged_lses_vert_stalled              = 0x00000044u32,
ge_merged_hsgs_vert_stalled              = 0x00000045u32,
ge_merged_hsgs_grp_stalled               = 0x00000046u32,
ge_merge_lses_fifo_blocked               = 0x00000047u32,
ge_merge_hsgs_fifo_blocked               = 0x00000048u32,
ge_merge_lses_vert_switch                = 0x00000049u32,
ge_merge_hsgs_vert_switch                = 0x0000004au32,
ge_merge_hsgs_grp_switch                 = 0x0000004bu32,
ge_merge_gsgrp_rdy_pending_verts         = 0x0000004cu32,
ge_merge_hsgrp_rdy_pending_verts         = 0x0000004du32,
ge_se_ds_cache_hits                      = 0x0000004eu32,
ge_se_api_vs_verts                       = 0x0000004fu32,
ge_se_api_ds_verts                       = 0x00000050u32,
ge_se_combined_busy                      = 0x00000051u32,
ge_spi_lsvert_send                       = 0x00000052u32,
ge_spi_hsvert_send                       = 0x00000053u32,
ge_ngg_attr_grp_wasted                   = 0x00000054u32,
ge_spi_gssubgrp_stalled                  = 0x00000055u32,
ge_ngg_attr_null_dealloc                 = 0x00000056u32,
ge_ngg_busy_base                         = 0x00000057u32,
}

/*
 * VGT_DETECT_ONE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_DETECT_ONE {
ENABLE_TF1_OPT                           = 0x00000000u32,
DISABLE_TF1_OPT                          = 0x00000001u32,
}

/*
 * VGT_DETECT_ZERO enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_DETECT_ZERO {
ENABLE_TF0_OPT                           = 0x00000000u32,
DISABLE_TF0_OPT                          = 0x00000001u32,
}

/*
 * VGT_DIST_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_DIST_MODE {
NO_DIST                                  = 0x00000000u32,
PATCHES                                  = 0x00000001u32,
DONUTS                                   = 0x00000002u32,
TRAPEZOIDS                               = 0x00000003u32,
}

/*
 * VGT_DI_INDEX_SIZE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_DI_INDEX_SIZE {
DI_INDEX_SIZE_16_BIT                     = 0x00000000u32,
DI_INDEX_SIZE_32_BIT                     = 0x00000001u32,
DI_INDEX_SIZE_8_BIT                      = 0x00000002u32,
}

/*
 * VGT_DI_PRIM_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_DI_PRIM_TYPE {
DI_PT_NONE                               = 0x00000000u32,
DI_PT_POINTLIST                          = 0x00000001u32,
DI_PT_LINELIST                           = 0x00000002u32,
DI_PT_LINESTRIP                          = 0x00000003u32,
DI_PT_TRILIST                            = 0x00000004u32,
DI_PT_TRIFAN                             = 0x00000005u32,
DI_PT_TRISTRIP                           = 0x00000006u32,
DI_PT_2D_RECTANGLE                       = 0x00000007u32,
DI_PT_UNUSED_1                           = 0x00000008u32,
DI_PT_PATCH                              = 0x00000009u32,
DI_PT_LINELIST_ADJ                       = 0x0000000au32,
DI_PT_LINESTRIP_ADJ                      = 0x0000000bu32,
DI_PT_TRILIST_ADJ                        = 0x0000000cu32,
DI_PT_TRISTRIP_ADJ                       = 0x0000000du32,
DI_PT_UNUSED_3                           = 0x0000000eu32,
DI_PT_UNUSED_4                           = 0x0000000fu32,
DI_PT_UNUSED_5                           = 0x00000010u32,
DI_PT_RECTLIST                           = 0x00000011u32,
DI_PT_LINELOOP                           = 0x00000012u32,
DI_PT_QUADLIST                           = 0x00000013u32,
DI_PT_QUADSTRIP                          = 0x00000014u32,
DI_PT_POLYGON                            = 0x00000015u32,
}

/*
 * VGT_DI_SOURCE_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_DI_SOURCE_SELECT {
DI_SRC_SEL_DMA                           = 0x00000000u32,
DI_SRC_SEL_IMMEDIATE                     = 0x00000001u32,
DI_SRC_SEL_AUTO_INDEX                    = 0x00000002u32,
DI_SRC_SEL_RESERVED                      = 0x00000003u32,
}

/*
 * VGT_DMA_BUF_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_DMA_BUF_TYPE {
VGT_DMA_BUF_MEM                          = 0x00000000u32,
VGT_DMA_BUF_RING                         = 0x00000001u32,
VGT_DMA_BUF_SETUP                        = 0x00000002u32,
VGT_DMA_PTR_UPDATE                       = 0x00000003u32,
}

/*
 * VGT_DMA_SWAP_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_DMA_SWAP_MODE {
VGT_DMA_SWAP_NONE                        = 0x00000000u32,
VGT_DMA_SWAP_16_BIT                      = 0x00000001u32,
VGT_DMA_SWAP_32_BIT                      = 0x00000002u32,
VGT_DMA_SWAP_WORD                        = 0x00000003u32,
}

/*
 * VGT_EVENT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_EVENT_TYPE {
Reserved_0x00                            = 0x00000000u32,
SAMPLE_STREAMOUTSTATS1                   = 0x00000001u32,
SAMPLE_STREAMOUTSTATS2                   = 0x00000002u32,
SAMPLE_STREAMOUTSTATS3                   = 0x00000003u32,
CACHE_FLUSH_TS                           = 0x00000004u32,
CONTEXT_DONE                             = 0x00000005u32,
CACHE_FLUSH                              = 0x00000006u32,
CS_PARTIAL_FLUSH                         = 0x00000007u32,
VGT_STREAMOUT_SYNC                       = 0x00000008u32,
EVENT_STATE_CHANGE                       = 0x00000009u32,
VGT_STREAMOUT_RESET                      = 0x0000000au32,
END_OF_PIPE_INCR_DE                      = 0x0000000bu32,
END_OF_PIPE_IB_END                       = 0x0000000cu32,
RST_PIX_CNT                              = 0x0000000du32,
BREAK_BATCH                              = 0x0000000eu32,
VS_PARTIAL_FLUSH                         = 0x0000000fu32,
PS_PARTIAL_FLUSH                         = 0x00000010u32,
FLUSH_HS_OUTPUT                          = 0x00000011u32,
FLUSH_DFSM                               = 0x00000012u32,
RESET_TO_LOWEST_VGT                      = 0x00000013u32,
CACHE_FLUSH_AND_INV_TS_EVENT             = 0x00000014u32,
WAIT_SYNC                                = 0x00000015u32,
CACHE_FLUSH_AND_INV_EVENT                = 0x00000016u32,
PERFCOUNTER_START                        = 0x00000017u32,
PERFCOUNTER_STOP                         = 0x00000018u32,
PIPELINESTAT_START                       = 0x00000019u32,
PIPELINESTAT_STOP                        = 0x0000001au32,
PERFCOUNTER_SAMPLE                       = 0x0000001bu32,
FLUSH_ES_OUTPUT                          = 0x0000001cu32,
BIN_CONF_OVERRIDE_CHECK                  = 0x0000001du32,
SAMPLE_PIPELINESTAT                      = 0x0000001eu32,
SO_VGTSTREAMOUT_FLUSH                    = 0x0000001fu32,
SAMPLE_STREAMOUTSTATS                    = 0x00000020u32,
RESET_VTX_CNT                            = 0x00000021u32,
BLOCK_CONTEXT_DONE                       = 0x00000022u32,
CS_CONTEXT_DONE                          = 0x00000023u32,
VGT_FLUSH                                = 0x00000024u32,
TGID_ROLLOVER                            = 0x00000025u32,
SQ_NON_EVENT                             = 0x00000026u32,
SC_SEND_DB_VPZ                           = 0x00000027u32,
BOTTOM_OF_PIPE_TS                        = 0x00000028u32,
FLUSH_SX_TS                              = 0x00000029u32,
DB_CACHE_FLUSH_AND_INV                   = 0x0000002au32,
FLUSH_AND_INV_DB_DATA_TS                 = 0x0000002bu32,
FLUSH_AND_INV_DB_META                    = 0x0000002cu32,
FLUSH_AND_INV_CB_DATA_TS                 = 0x0000002du32,
FLUSH_AND_INV_CB_META                    = 0x0000002eu32,
CS_DONE                                  = 0x0000002fu32,
PS_DONE                                  = 0x00000030u32,
FLUSH_AND_INV_CB_PIXEL_DATA              = 0x00000031u32,
SX_CB_RAT_ACK_REQUEST                    = 0x00000032u32,
THREAD_TRACE_START                       = 0x00000033u32,
THREAD_TRACE_STOP                        = 0x00000034u32,
THREAD_TRACE_MARKER                      = 0x00000035u32,
THREAD_TRACE_DRAW                        = 0x00000036u32,
THREAD_TRACE_FINISH                      = 0x00000037u32,
PIXEL_PIPE_STAT_CONTROL                  = 0x00000038u32,
PIXEL_PIPE_STAT_DUMP                     = 0x00000039u32,
PIXEL_PIPE_STAT_RESET                    = 0x0000003au32,
CONTEXT_SUSPEND                          = 0x0000003bu32,
OFFCHIP_HS_DEALLOC                       = 0x0000003cu32,
ENABLE_NGG_PIPELINE                      = 0x0000003du32,
ENABLE_PIPELINE_NOT_USED                 = 0x0000003eu32,
DRAW_DONE                                = 0x0000003fu32,
}

/*
 * VGT_GROUP_CONV_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_GROUP_CONV_SEL {
VGT_GRP_INDEX_16                         = 0x00000000u32,
VGT_GRP_INDEX_32                         = 0x00000001u32,
VGT_GRP_UINT_16                          = 0x00000002u32,
VGT_GRP_UINT_32                          = 0x00000003u32,
VGT_GRP_SINT_16                          = 0x00000004u32,
VGT_GRP_SINT_32                          = 0x00000005u32,
VGT_GRP_FLOAT_32                         = 0x00000006u32,
VGT_GRP_AUTO_PRIM                        = 0x00000007u32,
VGT_GRP_FIX_1_23_TO_FLOAT                = 0x00000008u32,
}

/*
 * VGT_GS_MODE_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_GS_MODE_TYPE {
GS_OFF                                   = 0x00000000u32,
GS_SCENARIO_A                            = 0x00000001u32,
GS_SCENARIO_B                            = 0x00000002u32,
GS_SCENARIO_G                            = 0x00000003u32,
GS_SCENARIO_C                            = 0x00000004u32,
SPRITE_EN                                = 0x00000005u32,
}

/*
 * VGT_GS_OUTPRIM_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_GS_OUTPRIM_TYPE {
POINTLIST                                = 0x00000000u32,
LINESTRIP                                = 0x00000001u32,
TRISTRIP                                 = 0x00000002u32,
RECT_2D                                  = 0x00000003u32,
RECTLIST                                 = 0x00000004u32,
}

/*
 * VGT_INDEX_TYPE_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_INDEX_TYPE_MODE {
VGT_INDEX_16                             = 0x00000000u32,
VGT_INDEX_32                             = 0x00000001u32,
VGT_INDEX_8                              = 0x00000002u32,
}

/*
 * VGT_OUTPATH_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_OUTPATH_SELECT {
VGT_OUTPATH_VTX_REUSE                    = 0x00000000u32,
VGT_OUTPATH_GS_BLOCK                     = 0x00000001u32,
VGT_OUTPATH_HS_BLOCK                     = 0x00000002u32,
VGT_OUTPATH_PRIM_GEN                     = 0x00000003u32,
VGT_OUTPATH_TE_PRIM_GEN                  = 0x00000004u32,
VGT_OUTPATH_TE_GS_BLOCK                  = 0x00000005u32,
VGT_OUTPATH_TE_OUTPUT                    = 0x00000006u32,
}

/*
 * VGT_OUT_PRIM_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_OUT_PRIM_TYPE {
VGT_OUT_POINT                            = 0x00000000u32,
VGT_OUT_LINE                             = 0x00000001u32,
VGT_OUT_TRI                              = 0x00000002u32,
VGT_OUT_2D_RECT                          = 0x00000003u32,
VGT_OUT_RECT_V0                          = 0x00000004u32,
VGT_OUT_DUMMY_1                          = 0x00000005u32,
VGT_OUT_DUMMY_2                          = 0x00000006u32,
VGT_OUT_DUMMY_3                          = 0x00000007u32,
VGT_OUT_PATCH                            = 0x00000008u32,
VGT_OUT_LINE_ADJ                         = 0x00000009u32,
VGT_OUT_TRI_ADJ                          = 0x0000000au32,
}

/*
 * VGT_RDREQ_POLICY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_RDREQ_POLICY {
VGT_POLICY_LRU                           = 0x00000000u32,
VGT_POLICY_STREAM                        = 0x00000001u32,
VGT_POLICY_BYPASS                        = 0x00000002u32,
}

/*
 * VGT_SPEC_DATA_READ enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_SPEC_DATA_READ {
VGT_SPEC_DATA_READ_AUTO                  = 0x00000000u32,
VGT_SPEC_DATA_READ_FORCE_ON              = 0x00000001u32,
VGT_SPEC_DATA_READ_FORCE_OFF             = 0x00000002u32,
}

/*
 * VGT_STAGES_GS_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_STAGES_GS_EN {
GS_STAGE_OFF                             = 0x00000000u32,
GS_STAGE_ON                              = 0x00000001u32,
}

/*
 * VGT_STAGES_HS_EN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_STAGES_HS_EN {
HS_STAGE_OFF                             = 0x00000000u32,
HS_STAGE_ON                              = 0x00000001u32,
}

/*
 * VGT_TEMPORAL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_TEMPORAL {
VGT_TEMPORAL_NORMAL                      = 0x00000000u32,
VGT_TEMPORAL_HIGH_PRIORITY               = 0x00000001u32,
VGT_TEMPORAL_STREAM                      = 0x00000002u32,
VGT_TEMPORAL_DISCARD                     = 0x00000003u32,
}

/*
 * VGT_TESS_PARTITION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_TESS_PARTITION {
PART_INTEGER                             = 0x00000000u32,
PART_POW2                                = 0x00000001u32,
PART_FRAC_ODD                            = 0x00000002u32,
PART_FRAC_EVEN                           = 0x00000003u32,
}

/*
 * VGT_TESS_TOPOLOGY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_TESS_TOPOLOGY {
OUTPUT_POINT                             = 0x00000000u32,
OUTPUT_LINE                              = 0x00000001u32,
OUTPUT_TRIANGLE_CW                       = 0x00000002u32,
OUTPUT_TRIANGLE_CCW                      = 0x00000003u32,
}

/*
 * VGT_TESS_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VGT_TESS_TYPE {
TESS_ISOLINE                             = 0x00000000u32,
TESS_TRIANGLE                            = 0x00000001u32,
TESS_QUAD                                = 0x00000002u32,
}

/*
 * WD_IA_DRAW_REG_XFER enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum WD_IA_DRAW_REG_XFER {
WD_IA_DRAW_REG_XFER_VGT_INSTANCE_BASE_ID = 0x00000000u32,
WD_IA_DRAW_REG_XFER_VGT_MULTI_PRIM_IB_RESET_EN = 0x00000001u32,
WD_IA_DRAW_REG_XFER_VGT_GS_OUT_PRIM_TYPE = 0x00000002u32,
WD_IA_DRAW_REG_XFER_GE_CNTL              = 0x00000003u32,
WD_IA_DRAW_REG_XFER_VGT_PRIMITIVE_TYPE   = 0x00000004u32,
WD_IA_DRAW_REG_XFER_GFX_PIPE_CONTROL     = 0x00000005u32,
WD_IA_DRAW_REG_XFER_GE_USER_VGPR_EN      = 0x00000006u32,
WD_IA_DRAW_REG_XFER_FL_MS_WG_DIM         = 0x00000007u32,
WD_IA_DRAW_REG_XFER_FL_MS_WG_DIM_1       = 0x00000008u32,
WD_IA_DRAW_REG_XFER_FL_MS_EXP_ALLOC      = 0x00000009u32,
WD_IA_DRAW_REG_XFER_FL_MS_TG_SIZE        = 0x0000000au32,
WD_IA_DRAW_REG_XFER_VGT_DRAW_PAYLOAD_CNTL = 0x0000000bu32,
WD_IA_DRAW_REG_XFER_GE_STEREO_CNTL       = 0x0000000cu32,
WD_IA_DRAW_REG_XFER_VGT_PRIMITIVEID_RESET = 0x0000000du32,
WD_IA_DRAW_REG_XFER_VGT_PRIMITIVEID_EN   = 0x0000000eu32,
WD_IA_DRAW_REG_XFER_GE_USER_VGPR1        = 0x0000000fu32,
WD_IA_DRAW_REG_XFER_GE_USER_VGPR2        = 0x00000010u32,
WD_IA_DRAW_REG_XFER_GE_USER_VGPR3        = 0x00000011u32,
WD_IA_DRAW_REG_XFER_GE_VRS_RATE          = 0x00000012u32,
WD_IA_DRAW_REG_XFER_GE_PC_ALLOC          = 0x00000013u32,
WD_IA_DRAW_REG_XFER_SPI_SHADER_GS_OUT_CONFIG_PS = 0x00000014u32,
WD_IA_DRAW_REG_XFER_GE_GS_THROTTLE       = 0x00000015u32,
}

/*
 * WD_IA_DRAW_SOURCE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum WD_IA_DRAW_SOURCE {
WD_IA_DRAW_SOURCE_DMA                    = 0x00000000u32,
WD_IA_DRAW_SOURCE_IMMD                   = 0x00000001u32,
WD_IA_DRAW_SOURCE_AUTO                   = 0x00000002u32,
WD_IA_DRAW_SOURCE_OPAQ                   = 0x00000003u32,
}

/*
 * WD_IA_DRAW_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum WD_IA_DRAW_TYPE {
WD_IA_DRAW_TYPE_DI_MM0                   = 0x00000000u32,
WD_IA_DRAW_TYPE_INDX_OFF                 = 0x00000001u32,
WD_IA_DRAW_TYPE_EVENT_INIT               = 0x00000002u32,
WD_IA_DRAW_TYPE_EVENT_ADDR               = 0x00000003u32,
WD_IA_DRAW_TYPE_REG_XFER                 = 0x00000004u32,
WD_IA_DRAW_TYPE_MIN_INDX                 = 0x00000005u32,
WD_IA_DRAW_TYPE_MAX_INDX                 = 0x00000006u32,
WD_IA_DRAW_TYPE_IMM_DATA                 = 0x00000007u32,
}

/*
 * GS_THREADID_SIZE value
 */

#define GSTHREADID_SIZE                0x00000002

/*******************************************************
 * CH Enums
 *******************************************************/

/*
 * CHA_PERF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CHA_PERF_SEL {
CHA_PERF_SEL_BUSY                        = 0x00000000u32,
CHA_PERF_SEL_STALL_CHC0                  = 0x00000001u32,
CHA_PERF_SEL_STALL_CHC1                  = 0x00000002u32,
CHA_PERF_SEL_STALL_CHC2                  = 0x00000003u32,
CHA_PERF_SEL_STALL_CHC3                  = 0x00000004u32,
CHA_PERF_SEL_REQUEST_CHC0                = 0x00000005u32,
CHA_PERF_SEL_REQUEST_CHC1                = 0x00000006u32,
CHA_PERF_SEL_REQUEST_CHC2                = 0x00000007u32,
CHA_PERF_SEL_REQUEST_CHC3                = 0x00000008u32,
CHA_PERF_SEL_MEM_32B_WDS_CHC0            = 0x00000009u32,
CHA_PERF_SEL_MEM_32B_WDS_CHC1            = 0x0000000au32,
CHA_PERF_SEL_MEM_32B_WDS_CHC2            = 0x0000000bu32,
CHA_PERF_SEL_MEM_32B_WDS_CHC3            = 0x0000000cu32,
CHA_PERF_SEL_IO_32B_WDS_CHC0             = 0x0000000du32,
CHA_PERF_SEL_IO_32B_WDS_CHC1             = 0x0000000eu32,
CHA_PERF_SEL_IO_32B_WDS_CHC2             = 0x0000000fu32,
CHA_PERF_SEL_IO_32B_WDS_CHC3             = 0x00000010u32,
CHA_PERF_SEL_MEM_BURST_COUNT_CHC0        = 0x00000011u32,
CHA_PERF_SEL_MEM_BURST_COUNT_CHC1        = 0x00000012u32,
CHA_PERF_SEL_MEM_BURST_COUNT_CHC2        = 0x00000013u32,
CHA_PERF_SEL_MEM_BURST_COUNT_CHC3        = 0x00000014u32,
CHA_PERF_SEL_IO_BURST_COUNT_CHC0         = 0x00000015u32,
CHA_PERF_SEL_IO_BURST_COUNT_CHC1         = 0x00000016u32,
CHA_PERF_SEL_IO_BURST_COUNT_CHC2         = 0x00000017u32,
CHA_PERF_SEL_IO_BURST_COUNT_CHC3         = 0x00000018u32,
CHA_PERF_SEL_ARB_REQUESTS                = 0x00000019u32,
CHA_PERF_SEL_REQ_INFLIGHT_LEVEL          = 0x0000001au32,
CHA_PERF_SEL_STALL_RET_CONFLICT_CHC0     = 0x0000001bu32,
CHA_PERF_SEL_STALL_RET_CONFLICT_CHC1     = 0x0000001cu32,
CHA_PERF_SEL_STALL_RET_CONFLICT_CHC2     = 0x0000001du32,
CHA_PERF_SEL_STALL_RET_CONFLICT_CHC3     = 0x0000001eu32,
CHA_PERF_SEL_CYCLE                       = 0x0000001fu32,
}

/*
 * CHC_PERF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CHC_PERF_SEL {
CHC_PERF_SEL_CYCLE                       = 0x00000000u32,
CHC_PERF_SEL_BUSY                        = 0x00000001u32,
CHC_PERF_SEL_STARVE                      = 0x00000002u32,
CHC_PERF_SEL_ARB_RET_LEVEL               = 0x00000003u32,
CHC_PERF_SEL_GL2_REQ_READ_LATENCY        = 0x00000004u32,
CHC_PERF_SEL_GL2_REQ_WRITE_LATENCY       = 0x00000005u32,
CHC_PERF_SEL_REQ                         = 0x00000006u32,
CHC_PERF_SEL_REQ_ATOMIC_WITH_RET         = 0x00000007u32,
CHC_PERF_SEL_REQ_ATOMIC_WITHOUT_RET      = 0x00000008u32,
CHC_PERF_SEL_REQ_NOP_ACK                 = 0x00000009u32,
CHC_PERF_SEL_REQ_NOP_RTN0                = 0x0000000au32,
CHC_PERF_SEL_REQ_READ                    = 0x0000000bu32,
CHC_PERF_SEL_REQ_READ_128B               = 0x0000000cu32,
CHC_PERF_SEL_REQ_READ_32B                = 0x0000000du32,
CHC_PERF_SEL_REQ_READ_64B                = 0x0000000eu32,
CHC_PERF_SEL_REQ_WRITE                   = 0x0000000fu32,
CHC_PERF_SEL_REQ_WRITE_32B               = 0x00000010u32,
CHC_PERF_SEL_REQ_WRITE_64B               = 0x00000011u32,
CHC_PERF_SEL_STALL_GL2_GL1               = 0x00000012u32,
CHC_PERF_SEL_STALL_BUFFER_FULL           = 0x00000013u32,
CHC_PERF_SEL_REQ_CLIENT0                 = 0x00000014u32,
CHC_PERF_SEL_REQ_CLIENT1                 = 0x00000015u32,
CHC_PERF_SEL_REQ_CLIENT2                 = 0x00000016u32,
CHC_PERF_SEL_REQ_CLIENT3                 = 0x00000017u32,
CHC_PERF_SEL_REQ_CLIENT4                 = 0x00000018u32,
CHC_PERF_SEL_REQ_CLIENT5                 = 0x00000019u32,
CHC_PERF_SEL_REQ_CLIENT6                 = 0x0000001au32,
CHC_PERF_SEL_REQ_CLIENT7                 = 0x0000001bu32,
CHC_PERF_SEL_REQ_CLIENT8                 = 0x0000001cu32,
CHC_PERF_SEL_REQ_CLIENT9                 = 0x0000001du32,
CHC_PERF_SEL_REQ_CLIENT10                = 0x0000001eu32,
CHC_PERF_SEL_REQ_CLIENT11                = 0x0000001fu32,
CHC_PERF_SEL_REQ_CLIENT12                = 0x00000020u32,
CHC_PERF_SEL_REQ_CLIENT13                = 0x00000021u32,
CHC_PERF_SEL_REQ_CLIENT14                = 0x00000022u32,
CHC_PERF_SEL_REQ_CLIENT15                = 0x00000023u32,
CHC_PERF_SEL_REQ_CLIENT16                = 0x00000024u32,
CHC_PERF_SEL_REQ_CLIENT17                = 0x00000025u32,
CHC_PERF_SEL_REQ_CLIENT18                = 0x00000026u32,
CHC_PERF_SEL_REQ_CLIENT19                = 0x00000027u32,
CHC_PERF_SEL_REQ_CLIENT20                = 0x00000028u32,
CHC_PERF_SEL_REQ_CLIENT21                = 0x00000029u32,
CHC_PERF_SEL_REQ_CLIENT22                = 0x0000002au32,
CHC_PERF_SEL_REQ_CLIENT23                = 0x0000002bu32,
}

/*******************************************************
 * GRBM Enums
 *******************************************************/

/*
 * GRBM_PERF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GRBM_PERF_SEL {
GRBM_PERF_SEL_COUNT                      = 0x00000000u32,
GRBM_PERF_SEL_USER_DEFINED               = 0x00000001u32,
GRBM_PERF_SEL_GUI_ACTIVE                 = 0x00000002u32,
GRBM_PERF_SEL_CP_BUSY                    = 0x00000003u32,
GRBM_PERF_SEL_CP_COHER_BUSY              = 0x00000004u32,
GRBM_PERF_SEL_CP_DMA_BUSY                = 0x00000005u32,
GRBM_PERF_SEL_CB_BUSY                    = 0x00000006u32,
GRBM_PERF_SEL_DB_BUSY                    = 0x00000007u32,
GRBM_PERF_SEL_PA_BUSY                    = 0x00000008u32,
GRBM_PERF_SEL_SC_BUSY                    = 0x00000009u32,
GRBM_PERF_SEL_SPI_BUSY                   = 0x0000000bu32,
GRBM_PERF_SEL_SX_BUSY                    = 0x0000000cu32,
GRBM_PERF_SEL_TA_BUSY                    = 0x0000000du32,
GRBM_PERF_SEL_CB_CLEAN                   = 0x0000000eu32,
GRBM_PERF_SEL_DB_CLEAN                   = 0x0000000fu32,
GRBM_PERF_SEL_BCI_BUSY                   = 0x0000001au32,
GRBM_PERF_SEL_RLC_BUSY                   = 0x0000001bu32,
GRBM_PERF_SEL_TCP_BUSY                   = 0x0000001cu32,
GRBM_PERF_SEL_CPG_BUSY                   = 0x0000001du32,
GRBM_PERF_SEL_CPC_BUSY                   = 0x0000001eu32,
GRBM_PERF_SEL_CPF_BUSY                   = 0x0000001fu32,
GRBM_PERF_SEL_GE_BUSY                    = 0x00000020u32,
GRBM_PERF_SEL_GE_NO_DMA_BUSY             = 0x00000021u32,
GRBM_PERF_SEL_UTCL2_BUSY                 = 0x00000022u32,
GRBM_PERF_SEL_EA_BUSY                    = 0x00000023u32,
GRBM_PERF_SEL_UTCL1_BUSY                 = 0x00000027u32,
GRBM_PERF_SEL_GL2CC_BUSY                 = 0x00000028u32,
GRBM_PERF_SEL_SDMA_BUSY                  = 0x00000029u32,
GRBM_PERF_SEL_CH_BUSY                    = 0x0000002au32,
GRBM_PERF_SEL_PMM_BUSY                   = 0x0000002cu32,
GRBM_PERF_SEL_GUS_BUSY                   = 0x0000002du32,
GRBM_PERF_SEL_GL1CC_BUSY                 = 0x0000002eu32,
GRBM_PERF_SEL_ANY_ACTIVE_F_BUSY          = 0x0000002fu32,
GRBM_PERF_SEL_GL1XCC_BUSY                = 0x00000030u32,
GRBM_PERF_SEL_PC_BUSY                    = 0x00000031u32,
}

/*******************************************************
 * CP Enums
 *******************************************************/

/*
 * CPC_LATENCY_STATS_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CPC_LATENCY_STATS_SEL {
CPC_LATENCY_STATS_SEL_XACK_MAX           = 0x00000000u32,
CPC_LATENCY_STATS_SEL_XACK_MIN           = 0x00000001u32,
CPC_LATENCY_STATS_SEL_XACK_LAST          = 0x00000002u32,
CPC_LATENCY_STATS_SEL_XNACK_MAX          = 0x00000003u32,
CPC_LATENCY_STATS_SEL_XNACK_MIN          = 0x00000004u32,
CPC_LATENCY_STATS_SEL_XNACK_LAST         = 0x00000005u32,
CPC_LATENCY_STATS_SEL_INVAL_MAX          = 0x00000006u32,
CPC_LATENCY_STATS_SEL_INVAL_MIN          = 0x00000007u32,
CPC_LATENCY_STATS_SEL_INVAL_LAST         = 0x00000008u32,
}

/*
 * CPC_PERFCOUNT_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CPC_PERFCOUNT_SEL {
CPC_PERF_SEL_ALWAYS_COUNT                = 0x00000000u32,
CPC_PERF_SEL_RCIU_STALL_WAIT_ON_FREE     = 0x00000001u32,
CPC_PERF_SEL_RCIU_STALL_PRIV_VIOLATION   = 0x00000002u32,
CPC_PERF_SEL_TCIU_STALL_WAIT_ON_FREE     = 0x00000005u32,
CPC_PERF_SEL_ME1_STALL_WAIT_ON_RCIU_READY = 0x00000006u32,
CPC_PERF_SEL_ME1_STALL_WAIT_ON_RCIU_READY_PERF = 0x00000007u32,
CPC_PERF_SEL_ME1_STALL_WAIT_ON_RCIU_READ = 0x00000008u32,
CPC_PERF_SEL_ME1_STALL_WAIT_ON_MEM_READ  = 0x00000009u32,
CPC_PERF_SEL_ME1_STALL_WAIT_ON_MEM_WRITE = 0x0000000au32,
CPC_PERF_SEL_ME1_STALL_ON_DATA_FROM_ROQ  = 0x0000000bu32,
CPC_PERF_SEL_ME1_STALL_ON_DATA_FROM_ROQ_PERF = 0x0000000cu32,
CPC_PERF_SEL_ME1_BUSY_FOR_PACKET_DECODE  = 0x0000000du32,
CPC_PERF_SEL_ME2_STALL_WAIT_ON_RCIU_READY = 0x0000000eu32,
CPC_PERF_SEL_ME2_STALL_WAIT_ON_RCIU_READY_PERF = 0x0000000fu32,
CPC_PERF_SEL_ME2_STALL_WAIT_ON_RCIU_READ = 0x00000010u32,
CPC_PERF_SEL_ME2_STALL_WAIT_ON_MEM_READ  = 0x00000011u32,
CPC_PERF_SEL_ME2_STALL_WAIT_ON_MEM_WRITE = 0x00000012u32,
CPC_PERF_SEL_ME2_STALL_ON_DATA_FROM_ROQ  = 0x00000013u32,
CPC_PERF_SEL_ME2_STALL_ON_DATA_FROM_ROQ_PERF = 0x00000014u32,
CPC_PERF_SEL_ME2_BUSY_FOR_PACKET_DECODE  = 0x00000015u32,
CPC_PERF_SEL_UTCL2IU_STALL_WAIT_ON_FREE  = 0x00000016u32,
CPC_PERF_SEL_UTCL2IU_STALL_WAIT_ON_TAGS  = 0x00000017u32,
CPC_PERF_SEL_UTCL1_STALL_ON_TRANSLATION  = 0x00000018u32,
CPC_PERF_SEL_CPC_STAT_BUSY               = 0x00000019u32,
CPC_PERF_SEL_CPC_STAT_IDLE               = 0x0000001au32,
CPC_PERF_SEL_CPC_STAT_STALL              = 0x0000001bu32,
CPC_PERF_SEL_CPC_TCIU_BUSY               = 0x0000001cu32,
CPC_PERF_SEL_CPC_TCIU_IDLE               = 0x0000001du32,
CPC_PERF_SEL_CPC_UTCL2IU_BUSY            = 0x0000001eu32,
CPC_PERF_SEL_CPC_UTCL2IU_IDLE            = 0x0000001fu32,
CPC_PERF_SEL_CPC_UTCL2IU_STALL           = 0x00000020u32,
CPC_PERF_SEL_ME1_DC0_SPI_BUSY            = 0x00000021u32,
CPC_PERF_SEL_ME2_DC1_SPI_BUSY            = 0x00000022u32,
CPC_PERF_SEL_CPC_GCRIU_BUSY              = 0x00000023u32,
CPC_PERF_SEL_CPC_GCRIU_IDLE              = 0x00000024u32,
CPC_PERF_SEL_CPC_GCRIU_STALL             = 0x00000025u32,
CPC_PERF_SEL_GCRIU_STALL_WAIT_ON_FREE    = 0x00000026u32,
CPC_PERF_SEL_ME1_STALL_WAIT_ON_TCIU_READ = 0x00000027u32,
CPC_PERF_SEL_ME2_STALL_WAIT_ON_TCIU_READ = 0x00000028u32,
CPC_PERF_SEL_CPC_UTCL2IU_XACK            = 0x00000029u32,
CPC_PERF_SEL_CPC_UTCL2IU_XNACK           = 0x0000002au32,
CPC_PERF_SEL_MEC_INSTR_CACHE_HIT         = 0x0000002bu32,
CPC_PERF_SEL_MEC_INSTR_CACHE_MISS        = 0x0000002cu32,
CPC_PERF_SEL_MES_THREAD0                 = 0x0000002du32,
CPC_PERF_SEL_MES_THREAD1                 = 0x0000002eu32,
CPC_PERF_SEL_TCIU_STALL_WAIT_ON_TAGS     = 0x0000002fu32,
CPC_PERF_SEL_TCIU_WRITE_REQUEST_SENT     = 0x00000030u32,
CPC_PERF_SEL_TCIU_READ_REQUEST_SENT      = 0x00000031u32,
CPC_PERF_SEL_GUS_WRITE_REQUEST_SENT      = 0x00000032u32,
CPC_PERF_SEL_GUS_READ_REQUEST_SENT       = 0x00000033u32,
CPC_PERF_SEL_MEC_THREAD0                 = 0x00000034u32,
CPC_PERF_SEL_MEC_THREAD1                 = 0x00000035u32,
CPC_PERF_SEL_MEC_THREAD2                 = 0x00000036u32,
CPC_PERF_SEL_MEC_THREAD3                 = 0x00000037u32,
}

/*
 * CPF_LATENCY_STATS_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CPF_LATENCY_STATS_SEL {
CPF_LATENCY_STATS_SEL_XACK_MAX           = 0x00000000u32,
CPF_LATENCY_STATS_SEL_XACK_MIN           = 0x00000001u32,
CPF_LATENCY_STATS_SEL_XACK_LAST          = 0x00000002u32,
CPF_LATENCY_STATS_SEL_XNACK_MAX          = 0x00000003u32,
CPF_LATENCY_STATS_SEL_XNACK_MIN          = 0x00000004u32,
CPF_LATENCY_STATS_SEL_XNACK_LAST         = 0x00000005u32,
CPF_LATENCY_STATS_SEL_READ_MAX           = 0x00000006u32,
CPF_LATENCY_STATS_SEL_READ_MIN           = 0x00000007u32,
CPF_LATENCY_STATS_SEL_READ_LAST          = 0x00000008u32,
CPF_LATENCY_STATS_SEL_INVAL_MAX          = 0x00000009u32,
CPF_LATENCY_STATS_SEL_INVAL_MIN          = 0x0000000au32,
CPF_LATENCY_STATS_SEL_INVAL_LAST         = 0x0000000bu32,
}

/*
 * CPF_PERFCOUNTWINDOW_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CPF_PERFCOUNTWINDOW_SEL {
CPF_PERFWINDOW_SEL_CSF                   = 0x00000000u32,
CPF_PERFWINDOW_SEL_HQD1                  = 0x00000001u32,
CPF_PERFWINDOW_SEL_HQD2                  = 0x00000002u32,
CPF_PERFWINDOW_SEL_RDMA                  = 0x00000003u32,
CPF_PERFWINDOW_SEL_RWPP                  = 0x00000004u32,
}

/*
 * CPF_PERFCOUNT_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CPF_PERFCOUNT_SEL {
CPF_PERF_SEL_ALWAYS_COUNT                = 0x00000000u32,
CPF_PERF_SEL_TCIU_STALLED_WAITING_ON_FREE = 0x00000002u32,
CPF_PERF_SEL_TCIU_STALLED_WAITING_ON_TAGS = 0x00000003u32,
CPF_PERF_SEL_CSF_BUSY_FOR_FETCHING_RING  = 0x00000004u32,
CPF_PERF_SEL_CSF_BUSY_FOR_FETCHING_IB1   = 0x00000005u32,
CPF_PERF_SEL_CSF_BUSY_FOR_FETCHING_IB2   = 0x00000006u32,
CPF_PERF_SEL_CSF_BUSY_FOR_FETCHING_STATE = 0x00000007u32,
CPF_PERF_SEL_CSF_STATE_FIFO_NOT_RTR      = 0x0000000au32,
CPF_PERF_SEL_CSF_FETCHING_CMD_BUFFERS    = 0x0000000bu32,
CPF_PERF_SEL_GRBM_DWORDS_SENT            = 0x0000000cu32,
CPF_PERF_SEL_DYNAMIC_CLOCK_VALID         = 0x0000000du32,
CPF_PERF_SEL_REGISTER_CLOCK_VALID        = 0x0000000eu32,
CPF_PERF_SEL_GUS_WRITE_REQUEST_SENT      = 0x0000000fu32,
CPF_PERF_SEL_GUS_READ_REQUEST_SENT       = 0x00000010u32,
CPF_PERF_SEL_UTCL2IU_STALL_WAIT_ON_FREE  = 0x00000011u32,
CPF_PERF_SEL_UTCL2IU_STALL_WAIT_ON_TAGS  = 0x00000012u32,
CPF_PERF_SEL_GFX_UTCL1_STALL_ON_TRANSLATION = 0x00000013u32,
CPF_PERF_SEL_CMP_UTCL1_STALL_ON_TRANSLATION = 0x00000014u32,
CPF_PERF_SEL_RCIU_STALL_WAIT_ON_FREE     = 0x00000015u32,
CPF_PERF_SEL_TCIU_WRITE_REQUEST_SENT     = 0x00000016u32,
CPF_PERF_SEL_TCIU_READ_REQUEST_SENT      = 0x00000017u32,
CPF_PERF_SEL_CPF_STAT_BUSY               = 0x00000018u32,
CPF_PERF_SEL_CPF_STAT_IDLE               = 0x00000019u32,
CPF_PERF_SEL_CPF_STAT_STALL              = 0x0000001au32,
CPF_PERF_SEL_CPF_TCIU_BUSY               = 0x0000001bu32,
CPF_PERF_SEL_CPF_TCIU_IDLE               = 0x0000001cu32,
CPF_PERF_SEL_CPF_TCIU_STALL              = 0x0000001du32,
CPF_PERF_SEL_CPF_UTCL2IU_BUSY            = 0x0000001eu32,
CPF_PERF_SEL_CPF_UTCL2IU_IDLE            = 0x0000001fu32,
CPF_PERF_SEL_CPF_UTCL2IU_STALL           = 0x00000020u32,
CPF_PERF_SEL_CPF_GCRIU_BUSY              = 0x00000021u32,
CPF_PERF_SEL_CPF_GCRIU_IDLE              = 0x00000022u32,
CPF_PERF_SEL_CPF_GCRIU_STALL             = 0x00000023u32,
CPF_PERF_SEL_GCRIU_STALL_WAIT_ON_FREE    = 0x00000024u32,
CPF_PERF_SEL_CSF_BUSY_FOR_FETCHING_DB    = 0x00000025u32,
CPF_PERF_SEL_CPF_UTCL2IU_XACK            = 0x00000026u32,
CPF_PERF_SEL_CPF_UTCL2IU_XNACK           = 0x00000027u32,
CPF_PERF_SEL_CP_SDMA_MNGR_DMA_REQ        = 0x00000028u32,
CPF_PERF_SEL_CP_SDMA_MNGR_DMA_DONE       = 0x00000029u32,
CPF_PERF_SEL_CP_SDMA_MNGR_LATENCY        = 0x0000002au32,
CPF_PERF_SEL_CP_SDMA_MNGR_SDMABUSY       = 0x0000002bu32,
}

/*
 * CPF_SCRATCH_REG_ATOMIC_OP enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CPF_SCRATCH_REG_ATOMIC_OP {
CPF_SCRATCH_REG_ATOMIC_ADD               = 0x00000000u32,
CPF_SCRATCH_REG_ATOMIC_SUB               = 0x00000001u32,
CPF_SCRATCH_REG_ATOMIC_OR                = 0x00000002u32,
CPF_SCRATCH_REG_ATOMIC_AND               = 0x00000003u32,
CPF_SCRATCH_REG_ATOMIC_NOT               = 0x00000004u32,
CPF_SCRATCH_REG_ATOMIC_MIN               = 0x00000005u32,
CPF_SCRATCH_REG_ATOMIC_MAX               = 0x00000006u32,
CPF_SCRATCH_REG_ATOMIC_CMPSWAP           = 0x00000007u32,
}

/*
 * CPG_LATENCY_STATS_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CPG_LATENCY_STATS_SEL {
CPG_LATENCY_STATS_SEL_XACK_MAX           = 0x00000000u32,
CPG_LATENCY_STATS_SEL_XACK_MIN           = 0x00000001u32,
CPG_LATENCY_STATS_SEL_XACK_LAST          = 0x00000002u32,
CPG_LATENCY_STATS_SEL_XNACK_MAX          = 0x00000003u32,
CPG_LATENCY_STATS_SEL_XNACK_MIN          = 0x00000004u32,
CPG_LATENCY_STATS_SEL_XNACK_LAST         = 0x00000005u32,
CPG_LATENCY_STATS_SEL_WRITE_MAX          = 0x00000006u32,
CPG_LATENCY_STATS_SEL_WRITE_MIN          = 0x00000007u32,
CPG_LATENCY_STATS_SEL_WRITE_LAST         = 0x00000008u32,
CPG_LATENCY_STATS_SEL_READ_MAX           = 0x00000009u32,
CPG_LATENCY_STATS_SEL_READ_MIN           = 0x0000000au32,
CPG_LATENCY_STATS_SEL_READ_LAST          = 0x0000000bu32,
CPG_LATENCY_STATS_SEL_ATOMIC_MAX         = 0x0000000cu32,
CPG_LATENCY_STATS_SEL_ATOMIC_MIN         = 0x0000000du32,
CPG_LATENCY_STATS_SEL_ATOMIC_LAST        = 0x0000000eu32,
CPG_LATENCY_STATS_SEL_INVAL_MAX          = 0x0000000fu32,
CPG_LATENCY_STATS_SEL_INVAL_MIN          = 0x00000010u32,
CPG_LATENCY_STATS_SEL_INVAL_LAST         = 0x00000011u32,
}

/*
 * CPG_PERFCOUNTWINDOW_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CPG_PERFCOUNTWINDOW_SEL {
CPG_PERFWINDOW_SEL_PFP                   = 0x00000000u32,
CPG_PERFWINDOW_SEL_ME                    = 0x00000001u32,
CPG_PERFWINDOW_SEL_CE                    = 0x00000002u32,
CPG_PERFWINDOW_SEL_MES                   = 0x00000003u32,
CPG_PERFWINDOW_SEL_MEC1                  = 0x00000004u32,
CPG_PERFWINDOW_SEL_MEC2                  = 0x00000005u32,
CPG_PERFWINDOW_SEL_DFY                   = 0x00000006u32,
CPG_PERFWINDOW_SEL_DMA                   = 0x00000007u32,
CPG_PERFWINDOW_SEL_SHADOW                = 0x00000008u32,
CPG_PERFWINDOW_SEL_RB                    = 0x00000009u32,
CPG_PERFWINDOW_SEL_CEDMA                 = 0x0000000au32,
CPG_PERFWINDOW_SEL_PRT_HDR_RPTR          = 0x0000000bu32,
CPG_PERFWINDOW_SEL_PRT_SMP_RPTR          = 0x0000000cu32,
CPG_PERFWINDOW_SEL_PQ1                   = 0x0000000du32,
CPG_PERFWINDOW_SEL_PQ2                   = 0x0000000eu32,
CPG_PERFWINDOW_SEL_PQ3                   = 0x0000000fu32,
CPG_PERFWINDOW_SEL_MEMWR                 = 0x00000010u32,
CPG_PERFWINDOW_SEL_MEMRD                 = 0x00000011u32,
CPG_PERFWINDOW_SEL_VGT0                  = 0x00000012u32,
CPG_PERFWINDOW_SEL_VGT1                  = 0x00000013u32,
CPG_PERFWINDOW_SEL_APPEND                = 0x00000014u32,
CPG_PERFWINDOW_SEL_QURD                  = 0x00000015u32,
CPG_PERFWINDOW_SEL_DDID                  = 0x00000016u32,
CPG_PERFWINDOW_SEL_SR                    = 0x00000017u32,
CPG_PERFWINDOW_SEL_QU_EOP                = 0x00000018u32,
CPG_PERFWINDOW_SEL_QU_STRM               = 0x00000019u32,
CPG_PERFWINDOW_SEL_QU_PIPE               = 0x0000001au32,
CPG_PERFWINDOW_SEL_RESERVED1             = 0x0000001bu32,
CPG_PERFWINDOW_SEL_CPC_IC                = 0x0000001cu32,
CPG_PERFWINDOW_SEL_RESERVED2             = 0x0000001du32,
CPG_PERFWINDOW_SEL_CPG_IC                = 0x0000001eu32,
}

/*
 * CPG_PERFCOUNT_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CPG_PERFCOUNT_SEL {
CPG_PERF_SEL_ALWAYS_COUNT                = 0x00000000u32,
CPG_PERF_SEL_RBIU_FIFO_FULL              = 0x00000001u32,
CPG_PERF_SEL_CP_GRBM_DWORDS_SENT         = 0x00000004u32,
CPG_PERF_SEL_ME_PARSER_BUSY              = 0x00000005u32,
CPG_PERF_SEL_COUNT_TYPE0_PACKETS         = 0x00000006u32,
CPG_PERF_SEL_COUNT_TYPE3_PACKETS         = 0x00000007u32,
CPG_PERF_SEL_CP_GRBM_OUT_OF_CREDITS      = 0x00000009u32,
CPG_PERF_SEL_CP_PFP_GRBM_OUT_OF_CREDITS  = 0x0000000au32,
CPG_PERF_SEL_CP_GDS_GRBM_OUT_OF_CREDITS  = 0x0000000bu32,
CPG_PERF_SEL_RCIU_STALLED_ON_ME_READ     = 0x0000000cu32,
CPG_PERF_SEL_RCIU_STALLED_ON_DMA_READ    = 0x0000000du32,
CPG_PERF_SEL_SSU_STALLED_ON_ACTIVE_CNTX  = 0x0000000eu32,
CPG_PERF_SEL_SSU_STALLED_ON_CLEAN_SIGNALS = 0x0000000fu32,
CPG_PERF_SEL_QU_STALLED_ON_EOP_DONE_PULSE = 0x00000010u32,
CPG_PERF_SEL_QU_STALLED_ON_EOP_DONE_WR_CONFIRM = 0x00000011u32,
CPG_PERF_SEL_PFP_STALLED_ON_CSF_READY    = 0x00000012u32,
CPG_PERF_SEL_PFP_STALLED_ON_MEQ_READY    = 0x00000013u32,
CPG_PERF_SEL_PFP_STALLED_ON_RCIU_READY   = 0x00000014u32,
CPG_PERF_SEL_PFP_STALLED_FOR_DATA_FROM_ROQ = 0x00000015u32,
CPG_PERF_SEL_ME_STALLED_FOR_DATA_FROM_PFP = 0x00000016u32,
CPG_PERF_SEL_ME_STALLED_FOR_DATA_FROM_STQ = 0x00000017u32,
CPG_PERF_SEL_ME_STALLED_ON_NO_AVAIL_GFX_CNTX = 0x00000018u32,
CPG_PERF_SEL_ME_STALLED_WRITING_TO_RCIU  = 0x00000019u32,
CPG_PERF_SEL_ME_STALLED_WRITING_CONSTANTS = 0x0000001au32,
CPG_PERF_SEL_ME_STALLED_ON_PARTIAL_FLUSH = 0x0000001bu32,
CPG_PERF_SEL_ME_WAIT_ON_CE_COUNTER       = 0x0000001cu32,
CPG_PERF_SEL_ME_WAIT_ON_AVAIL_BUFFER     = 0x0000001du32,
CPG_PERF_SEL_LOAD_STALLED_ON_SET_COHERENCY = 0x0000001fu32,
CPG_PERF_SEL_DYNAMIC_CLK_VALID           = 0x00000020u32,
CPG_PERF_SEL_REGISTER_CLK_VALID          = 0x00000021u32,
CPG_PERF_SEL_GUS_WRITE_REQUEST_SENT      = 0x00000022u32,
CPG_PERF_SEL_GUS_READ_REQUEST_SENT       = 0x00000023u32,
CPG_PERF_SEL_CE_STALL_RAM_DUMP           = 0x00000024u32,
CPG_PERF_SEL_CE_STALL_RAM_WRITE          = 0x00000025u32,
CPG_PERF_SEL_CE_STALL_ON_INC_FIFO        = 0x00000026u32,
CPG_PERF_SEL_CE_STALL_ON_WR_RAM_FIFO     = 0x00000027u32,
CPG_PERF_SEL_CE_STALL_ON_DATA_FROM_ROQ   = 0x00000029u32,
CPG_PERF_SEL_CE_STALL_ON_CE_BUFFER_FLAG  = 0x0000002au32,
CPG_PERF_SEL_CE_STALL_ON_DE_COUNTER      = 0x0000002bu32,
CPG_PERF_SEL_TCIU_STALL_WAIT_ON_FREE     = 0x0000002cu32,
CPG_PERF_SEL_TCIU_STALL_WAIT_ON_TAGS     = 0x0000002du32,
CPG_PERF_SEL_UTCL2IU_STALL_WAIT_ON_FREE  = 0x0000002eu32,
CPG_PERF_SEL_UTCL2IU_STALL_WAIT_ON_TAGS  = 0x0000002fu32,
CPG_PERF_SEL_UTCL1_STALL_ON_TRANSLATION  = 0x00000030u32,
CPG_PERF_SEL_TCIU_WRITE_REQUEST_SENT     = 0x00000031u32,
CPG_PERF_SEL_TCIU_READ_REQUEST_SENT      = 0x00000032u32,
CPG_PERF_SEL_CPG_STAT_BUSY               = 0x00000033u32,
CPG_PERF_SEL_CPG_STAT_IDLE               = 0x00000034u32,
CPG_PERF_SEL_CPG_STAT_STALL              = 0x00000035u32,
CPG_PERF_SEL_CPG_TCIU_BUSY               = 0x00000036u32,
CPG_PERF_SEL_CPG_TCIU_IDLE               = 0x00000037u32,
CPG_PERF_SEL_CPG_TCIU_STALL              = 0x00000038u32,
CPG_PERF_SEL_CPG_UTCL2IU_BUSY            = 0x00000039u32,
CPG_PERF_SEL_CPG_UTCL2IU_IDLE            = 0x0000003au32,
CPG_PERF_SEL_CPG_UTCL2IU_STALL           = 0x0000003bu32,
CPG_PERF_SEL_CPG_GCRIU_BUSY              = 0x0000003cu32,
CPG_PERF_SEL_CPG_GCRIU_IDLE              = 0x0000003du32,
CPG_PERF_SEL_CPG_GCRIU_STALL             = 0x0000003eu32,
CPG_PERF_SEL_GCRIU_STALL_WAIT_ON_FREE    = 0x0000003fu32,
CPG_PERF_SEL_ALL_GFX_PIPES_BUSY          = 0x00000040u32,
CPG_PERF_SEL_CPG_UTCL2IU_XACK            = 0x00000041u32,
CPG_PERF_SEL_CPG_UTCL2IU_XNACK           = 0x00000042u32,
CPG_PERF_SEL_PFP_STALLED_ON_MEQ_DDID_READY = 0x00000043u32,
CPG_PERF_SEL_PFP_INSTR_CACHE_HIT         = 0x00000044u32,
CPG_PERF_SEL_PFP_INSTR_CACHE_MISS        = 0x00000045u32,
CPG_PERF_SEL_CE_INSTR_CACHE_HIT          = 0x00000046u32,
CPG_PERF_SEL_CE_INSTR_CACHE_MISS         = 0x00000047u32,
CPG_PERF_SEL_ME_INSTR_CACHE_HIT          = 0x00000048u32,
CPG_PERF_SEL_ME_INSTR_CACHE_MISS         = 0x00000049u32,
CPG_PERF_SEL_PFP_PACKET_FILTER_HIT_IB1   = 0x0000004au32,
CPG_PERF_SEL_PFP_PACKET_FILTER_MISS_IB1  = 0x0000004bu32,
CPG_PERF_SEL_PFP_PACKET_FILTER_HIT_IB2   = 0x0000004cu32,
CPG_PERF_SEL_PFP_PACKET_FILTER_MISS_IB2  = 0x0000004du32,
CPG_PERF_SEL_DMA_BUSY                    = 0x0000004eu32,
CPG_PERF_SEL_DMA_STARVED                 = 0x0000004fu32,
CPG_PERF_SEL_DMA_STALLED                 = 0x00000050u32,
CPG_PERF_SEL_DMA_FETCHER_STALLED_ON_ROQ_FULL = 0x00000051u32,
CPG_PERF_SEL_PFP_PWS_STALLED0            = 0x00000052u32,
CPG_PERF_SEL_ME_PWS_STALLED0             = 0x00000053u32,
CPG_PERF_SEL_PFP_VGTDMA_INDR_STRUCT_BYPASS0 = 0x00000054u32,
CPG_PERF_SEL_PFP_VGTDMA_INDR_STRUCT_NOT_BYPASS0 = 0x00000055u32,
CPG_PERF_SEL_PFP_VGTDMA_DB_ROQ_DATA_STALL0 = 0x00000056u32,
CPG_PERF_SEL_PFP_PWS_STALLED1            = 0x00000057u32,
CPG_PERF_SEL_ME_PWS_STALLED1             = 0x00000058u32,
CPG_PERF_SEL_PFP_VGTDMA_INDR_STRUCT_BYPASS1 = 0x00000059u32,
CPG_PERF_SEL_PFP_VGTDMA_INDR_STRUCT_NOT_BYPASS1 = 0x0000005au32,
CPG_PERF_SEL_PFP_VGTDMA_DB_ROQ_DATA_STALL1 = 0x0000005bu32,
}

/*
 * CP_ALPHA_TAG_RAM_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CP_ALPHA_TAG_RAM_SEL {
CPG_TAG_RAM                              = 0x00000000u32,
CPC_TAG_RAM                              = 0x00000001u32,
CPF_TAG_RAM                              = 0x00000002u32,
RSV_TAG_RAM                              = 0x00000003u32,
}

/*
 * CP_DDID_CNTL_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CP_DDID_CNTL_MODE {
STALL                                    = 0x00000000u32,
OVERRUN                                  = 0x00000001u32,
}

/*
 * CP_DDID_CNTL_SIZE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CP_DDID_CNTL_SIZE {
SIZE_8K                                  = 0x00000000u32,
SIZE_16K                                 = 0x00000001u32,
}

/*
 * CP_DDID_CNTL_VMID_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CP_DDID_CNTL_VMID_SEL {
DDID_VMID_PIPE                           = 0x00000000u32,
DDID_VMID_CNTL                           = 0x00000001u32,
}

/*
 * CP_ME_ID enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CP_ME_ID {
ME_ID0                                   = 0x00000000u32,
ME_ID1                                   = 0x00000001u32,
ME_ID2                                   = 0x00000002u32,
ME_ID3                                   = 0x00000003u32,
}

/*
 * CP_PIPE_ID enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CP_PIPE_ID {
PIPE_ID0                                 = 0x00000000u32,
PIPE_ID1                                 = 0x00000001u32,
PIPE_ID2                                 = 0x00000002u32,
PIPE_ID3                                 = 0x00000003u32,
}

/*
 * CP_RING_ID enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CP_RING_ID {
RINGID0                                  = 0x00000000u32,
RINGID1                                  = 0x00000001u32,
RINGID2                                  = 0x00000002u32,
RINGID3                                  = 0x00000003u32,
}

/*
 * IQ_RETRY_TYPE value
 */

#define IQ_QUEUE_SLEEP                 0x00000000
#define IQ_OFFLOAD_RETRY               0x00000001
#define IQ_SCH_WAVE_MSG                0x00000002
#define IQ_DEQUEUE_RETRY               0x00000004

/*
 * IQ_INTR_TYPE value
 */

#define IQ_INTR_TYPE_PQ                0x00000000
#define IQ_INTR_TYPE_IB                0x00000001
#define IQ_INTR_TYPE_MQD               0x00000002

/*
 * VMID_SIZE value
 */

#define VMID_SZ                        0x00000004

/*
 * CONFIG_SPACE value
 */

#define CONFIG_SPACE_START             0x00002000
#define CONFIG_SPACE_END               0x00009fff

/*
 * CONFIG_SPACE1 valu
 */

#define CONFIG_SPACE1_START            0x00002000
#define CONFIG_SPACE1_END              0x00002bff

/*
 * CONFIG_SPACE2 value
 */

#define CONFIG_SPACE2_START            0x00003000
#define CONFIG_SPACE2_END              0x00009fff

/*
 * UCONFIG_SPACE value
 */

#define UCONFIG_SPACE_START            0x0000c000
#define UCONFIG_SPACE_END              0x0000ffff

/*
 * PERSISTENT_SPACE value
 */

#define PERSISTENT_SPACE_START         0x00002c00
#define PERSISTENT_SPACE_END           0x00002fff

/*
 * CONTEXT_SPACE value
 */

#define CONTEXT_SPACE_START            0x0000a000
#define CONTEXT_SPACE_END              0x0000a3ff

/*******************************************************
 * GCR Enums
 *******************************************************/

/*
 * GCRPerfSel enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GCRPerfSel {
GCR_PERF_SEL_NONE                        = 0x00000000u32,
GCR_PERF_SEL_SDMA0_ALL_REQ               = 0x00000001u32,
GCR_PERF_SEL_SDMA0_GL2_RANGE_REQ         = 0x00000002u32,
GCR_PERF_SEL_SDMA0_GL2_RANGE_LT16K_REQ   = 0x00000003u32,
GCR_PERF_SEL_SDMA0_GL2_RANGE_16K_REQ     = 0x00000004u32,
GCR_PERF_SEL_SDMA0_GL2_RANGE_GT16K_REQ   = 0x00000005u32,
GCR_PERF_SEL_SDMA0_GL2_ALL_REQ           = 0x00000006u32,
GCR_PERF_SEL_SDMA0_GL1_RANGE_REQ         = 0x00000007u32,
GCR_PERF_SEL_SDMA0_GL1_RANGE_LT16K_REQ   = 0x00000008u32,
GCR_PERF_SEL_SDMA0_GL1_RANGE_16K_REQ     = 0x00000009u32,
GCR_PERF_SEL_SDMA0_GL1_RANGE_GT16K_REQ   = 0x0000000au32,
GCR_PERF_SEL_SDMA0_GL1_ALL_REQ           = 0x0000000bu32,
GCR_PERF_SEL_SDMA0_METADATA_REQ          = 0x0000000cu32,
GCR_PERF_SEL_SDMA0_SQC_DATA_REQ          = 0x0000000du32,
GCR_PERF_SEL_SDMA0_SQC_INST_REQ          = 0x0000000eu32,
GCR_PERF_SEL_SDMA0_TCP_REQ               = 0x0000000fu32,
GCR_PERF_SEL_SDMA0_GL1_TLB_SHOOTDOWN_REQ = 0x00000010u32,
GCR_PERF_SEL_SDMA1_ALL_REQ               = 0x00000011u32,
GCR_PERF_SEL_SDMA1_GL2_RANGE_REQ         = 0x00000012u32,
GCR_PERF_SEL_SDMA1_GL2_RANGE_LT16K_REQ   = 0x00000013u32,
GCR_PERF_SEL_SDMA1_GL2_RANGE_16K_REQ     = 0x00000014u32,
GCR_PERF_SEL_SDMA1_GL2_RANGE_GT16K_REQ   = 0x00000015u32,
GCR_PERF_SEL_SDMA1_GL2_ALL_REQ           = 0x00000016u32,
GCR_PERF_SEL_SDMA1_GL1_RANGE_REQ         = 0x00000017u32,
GCR_PERF_SEL_SDMA1_GL1_RANGE_LT16K_REQ   = 0x00000018u32,
GCR_PERF_SEL_SDMA1_GL1_RANGE_16K_REQ     = 0x00000019u32,
GCR_PERF_SEL_SDMA1_GL1_RANGE_GT16K_REQ   = 0x0000001au32,
GCR_PERF_SEL_SDMA1_GL1_ALL_REQ           = 0x0000001bu32,
GCR_PERF_SEL_SDMA1_METADATA_REQ          = 0x0000001cu32,
GCR_PERF_SEL_SDMA1_SQC_DATA_REQ          = 0x0000001du32,
GCR_PERF_SEL_SDMA1_SQC_INST_REQ          = 0x0000001eu32,
GCR_PERF_SEL_SDMA1_TCP_REQ               = 0x0000001fu32,
GCR_PERF_SEL_SDMA1_GL1_TLB_SHOOTDOWN_REQ = 0x00000020u32,
GCR_PERF_SEL_CPC_ALL_REQ                 = 0x00000021u32,
GCR_PERF_SEL_CPC_GL2_RANGE_REQ           = 0x00000022u32,
GCR_PERF_SEL_CPC_GL2_RANGE_LT16K_REQ     = 0x00000023u32,
GCR_PERF_SEL_CPC_GL2_RANGE_16K_REQ       = 0x00000024u32,
GCR_PERF_SEL_CPC_GL2_RANGE_GT16K_REQ     = 0x00000025u32,
GCR_PERF_SEL_CPC_GL2_ALL_REQ             = 0x00000026u32,
GCR_PERF_SEL_CPC_GL1_RANGE_REQ           = 0x00000027u32,
GCR_PERF_SEL_CPC_GL1_RANGE_LT16K_REQ     = 0x00000028u32,
GCR_PERF_SEL_CPC_GL1_RANGE_16K_REQ       = 0x00000029u32,
GCR_PERF_SEL_CPC_GL1_RANGE_GT16K_REQ     = 0x0000002au32,
GCR_PERF_SEL_CPC_GL1_ALL_REQ             = 0x0000002bu32,
GCR_PERF_SEL_CPC_METADATA_REQ            = 0x0000002cu32,
GCR_PERF_SEL_CPC_SQC_DATA_REQ            = 0x0000002du32,
GCR_PERF_SEL_CPC_SQC_INST_REQ            = 0x0000002eu32,
GCR_PERF_SEL_CPC_TCP_REQ                 = 0x0000002fu32,
GCR_PERF_SEL_CPC_GL1_TLB_SHOOTDOWN_REQ   = 0x00000030u32,
GCR_PERF_SEL_CPG_ALL_REQ                 = 0x00000031u32,
GCR_PERF_SEL_CPG_GL2_RANGE_REQ           = 0x00000032u32,
GCR_PERF_SEL_CPG_GL2_RANGE_LT16K_REQ     = 0x00000033u32,
GCR_PERF_SEL_CPG_GL2_RANGE_16K_REQ       = 0x00000034u32,
GCR_PERF_SEL_CPG_GL2_RANGE_GT16K_REQ     = 0x00000035u32,
GCR_PERF_SEL_CPG_GL2_ALL_REQ             = 0x00000036u32,
GCR_PERF_SEL_CPG_GL1_RANGE_REQ           = 0x00000037u32,
GCR_PERF_SEL_CPG_GL1_RANGE_LT16K_REQ     = 0x00000038u32,
GCR_PERF_SEL_CPG_GL1_RANGE_16K_REQ       = 0x00000039u32,
GCR_PERF_SEL_CPG_GL1_RANGE_GT16K_REQ     = 0x0000003au32,
GCR_PERF_SEL_CPG_GL1_ALL_REQ             = 0x0000003bu32,
GCR_PERF_SEL_CPG_METADATA_REQ            = 0x0000003cu32,
GCR_PERF_SEL_CPG_SQC_DATA_REQ            = 0x0000003du32,
GCR_PERF_SEL_CPG_SQC_INST_REQ            = 0x0000003eu32,
GCR_PERF_SEL_CPG_TCP_REQ                 = 0x0000003fu32,
GCR_PERF_SEL_CPG_GL1_TLB_SHOOTDOWN_REQ   = 0x00000040u32,
GCR_PERF_SEL_CPF_ALL_REQ                 = 0x00000041u32,
GCR_PERF_SEL_CPF_GL2_RANGE_REQ           = 0x00000042u32,
GCR_PERF_SEL_CPF_GL2_RANGE_LT16K_REQ     = 0x00000043u32,
GCR_PERF_SEL_CPF_GL2_RANGE_16K_REQ       = 0x00000044u32,
GCR_PERF_SEL_CPF_GL2_RANGE_GT16K_REQ     = 0x00000045u32,
GCR_PERF_SEL_CPF_GL2_ALL_REQ             = 0x00000046u32,
GCR_PERF_SEL_CPF_GL1_RANGE_REQ           = 0x00000047u32,
GCR_PERF_SEL_CPF_GL1_RANGE_LT16K_REQ     = 0x00000048u32,
GCR_PERF_SEL_CPF_GL1_RANGE_16K_REQ       = 0x00000049u32,
GCR_PERF_SEL_CPF_GL1_RANGE_GT16K_REQ     = 0x0000004au32,
GCR_PERF_SEL_CPF_GL1_ALL_REQ             = 0x0000004bu32,
GCR_PERF_SEL_CPF_METADATA_REQ            = 0x0000004cu32,
GCR_PERF_SEL_CPF_SQC_DATA_REQ            = 0x0000004du32,
GCR_PERF_SEL_CPF_SQC_INST_REQ            = 0x0000004eu32,
GCR_PERF_SEL_CPF_TCP_REQ                 = 0x0000004fu32,
GCR_PERF_SEL_CPF_GL1_TLB_SHOOTDOWN_REQ   = 0x00000050u32,
GCR_PERF_SEL_VIRT_REQ                    = 0x00000051u32,
GCR_PERF_SEL_PHY_REQ                     = 0x00000052u32,
GCR_PERF_SEL_TLB_SHOOTDOWN_HEAVY_REQ     = 0x00000053u32,
GCR_PERF_SEL_TLB_SHOOTDOWN_LIGHT_REQ     = 0x00000054u32,
GCR_PERF_SEL_ALL_REQ                     = 0x00000055u32,
GCR_PERF_SEL_CLK_FOR_PHY_OUTSTANDING_REQ = 0x00000056u32,
GCR_PERF_SEL_CLK_FOR_VIRT_OUTSTANDING_REQ = 0x00000057u32,
GCR_PERF_SEL_CLK_FOR_ALL_OUTSTANDING_REQ = 0x00000058u32,
GCR_PERF_SEL_UTCL2_REQ                   = 0x00000059u32,
GCR_PERF_SEL_UTCL2_RET                   = 0x0000005au32,
GCR_PERF_SEL_UTCL2_OUT_OF_CREDIT_EVENT   = 0x0000005bu32,
GCR_PERF_SEL_UTCL2_INFLIGHT_REQ          = 0x0000005cu32,
GCR_PERF_SEL_UTCL2_FILTERED_RET          = 0x0000005du32,
GCR_PERF_SEL_PMM_ABIT_NUM_FLUSH          = 0x0000005eu32,
GCR_PERF_SEL_PMM_ABIT_FLUSH_ONGOING      = 0x0000005fu32,
GCR_PERF_SEL_PMM_NUM_INTERRUPT           = 0x00000060u32,
GCR_PERF_SEL_PMM_STALL_PMM_IH_CREDITS    = 0x00000061u32,
GCR_PERF_SEL_PMM_INTERRUPT_READY_TO_SEND = 0x00000062u32,
GCR_PERF_SEL_PMM_ABIT_TIMER_FLUSH        = 0x00000063u32,
GCR_PERF_SEL_PMM_ABIT_FORCE_FLUSH        = 0x00000064u32,
GCR_PERF_SEL_PMM_ABIT_FLUSH_INTERRUPT    = 0x00000065u32,
GCR_PERF_SEL_PMM_ALOG_INTERRUPT          = 0x00000066u32,
GCR_PERF_SEL_PMM_MAM_FLUSH_REQ           = 0x00000067u32,
GCR_PERF_SEL_PMM_MAM_FLUSH_RESP          = 0x00000068u32,
GCR_PERF_SEL_PMM_RLC_CGCG_REQ            = 0x00000069u32,
GCR_PERF_SEL_PMM_RLC_CGCG_RESP           = 0x0000006au32,
GCR_PERF_SEL_RLC_ALL_REQ                 = 0x0000006bu32,
GCR_PERF_SEL_RLC_GL2_RANGE_REQ           = 0x0000006cu32,
GCR_PERF_SEL_RLC_GL2_RANGE_LT16K_REQ     = 0x0000006du32,
GCR_PERF_SEL_RLC_GL2_RANGE_16K_REQ       = 0x0000006eu32,
GCR_PERF_SEL_RLC_GL2_RANGE_GT16K_REQ     = 0x0000006fu32,
GCR_PERF_SEL_RLC_GL2_ALL_REQ             = 0x00000070u32,
GCR_PERF_SEL_RLC_GL1_RANGE_REQ           = 0x00000071u32,
GCR_PERF_SEL_RLC_GL1_RANGE_LT16K_REQ     = 0x00000072u32,
GCR_PERF_SEL_RLC_GL1_RANGE_16K_REQ       = 0x00000073u32,
GCR_PERF_SEL_RLC_GL1_RANGE_GT16K_REQ     = 0x00000074u32,
GCR_PERF_SEL_RLC_GL1_ALL_REQ             = 0x00000075u32,
GCR_PERF_SEL_RLC_METADATA_REQ            = 0x00000076u32,
GCR_PERF_SEL_RLC_SQC_DATA_REQ            = 0x00000077u32,
GCR_PERF_SEL_RLC_SQC_INST_REQ            = 0x00000078u32,
GCR_PERF_SEL_RLC_TCP_REQ                 = 0x00000079u32,
GCR_PERF_SEL_RLC_GL1_TLB_SHOOTDOWN_REQ   = 0x0000007au32,
GCR_PERF_SEL_PM_ALL_REQ                  = 0x0000007bu32,
GCR_PERF_SEL_PM_GL2_RANGE_REQ            = 0x0000007cu32,
GCR_PERF_SEL_PM_GL2_RANGE_LT16K_REQ      = 0x0000007du32,
GCR_PERF_SEL_PM_GL2_RANGE_16K_REQ        = 0x0000007eu32,
GCR_PERF_SEL_PM_GL2_RANGE_GT16K_REQ      = 0x0000007fu32,
GCR_PERF_SEL_PM_GL2_ALL_REQ              = 0x00000080u32,
GCR_PERF_SEL_PM_GL1_RANGE_REQ            = 0x00000081u32,
GCR_PERF_SEL_PM_GL1_RANGE_LT16K_REQ      = 0x00000082u32,
GCR_PERF_SEL_PM_GL1_RANGE_16K_REQ        = 0x00000083u32,
GCR_PERF_SEL_PM_GL1_RANGE_GT16K_REQ      = 0x00000084u32,
GCR_PERF_SEL_PM_GL1_ALL_REQ              = 0x00000085u32,
GCR_PERF_SEL_PM_METADATA_REQ             = 0x00000086u32,
GCR_PERF_SEL_PM_SQC_DATA_REQ             = 0x00000087u32,
GCR_PERF_SEL_PM_SQC_INST_REQ             = 0x00000088u32,
GCR_PERF_SEL_PM_TCP_REQ                  = 0x00000089u32,
GCR_PERF_SEL_PM_GL1_TLB_SHOOTDOWN_REQ    = 0x0000008au32,
GCR_PERF_SEL_PIO_ALL_REQ                 = 0x0000008bu32,
GCR_PERF_SEL_PIO_GL2_RANGE_REQ           = 0x0000008cu32,
GCR_PERF_SEL_PIO_GL2_RANGE_LT16K_REQ     = 0x0000008du32,
GCR_PERF_SEL_PIO_GL2_RANGE_16K_REQ       = 0x0000008eu32,
GCR_PERF_SEL_PIO_GL2_RANGE_GT16K_REQ     = 0x0000008fu32,
GCR_PERF_SEL_PIO_GL2_ALL_REQ             = 0x00000090u32,
GCR_PERF_SEL_PIO_GL1_RANGE_REQ           = 0x00000091u32,
GCR_PERF_SEL_PIO_GL1_RANGE_LT16K_REQ     = 0x00000092u32,
GCR_PERF_SEL_PIO_GL1_RANGE_16K_REQ       = 0x00000093u32,
GCR_PERF_SEL_PIO_GL1_RANGE_GT16K_REQ     = 0x00000094u32,
GCR_PERF_SEL_PIO_GL1_ALL_REQ             = 0x00000095u32,
GCR_PERF_SEL_PIO_METADATA_REQ            = 0x00000096u32,
GCR_PERF_SEL_PIO_SQC_DATA_REQ            = 0x00000097u32,
GCR_PERF_SEL_PIO_SQC_INST_REQ            = 0x00000098u32,
GCR_PERF_SEL_PIO_TCP_REQ                 = 0x00000099u32,
GCR_PERF_SEL_PIO_GL1_TLB_SHOOTDOWN_REQ   = 0x0000009au32,
}

/*******************************************************
 * GC_EA_CPWD Enums
 *******************************************************/

/*
 * GC_EA_CPWD_PERFCOUNT_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GC_EA_CPWD_PERFCOUNT_SEL {
GC_EA_CPWD_PERF_SEL_ALWAYS_COUNT         = 0x00000000u32,
GC_EA_CPWD_PERF_SEL_RDRAM_NUM_BANKS_VLD  = 0x00000001u32,
GC_EA_CPWD_PERF_SEL_RDRAM_REQ_PER_CLIGRP = 0x00000002u32,
GC_EA_CPWD_PERF_SEL_RDRAM_CHAINED_REQ_PER_CLIGRP = 0x00000003u32,
GC_EA_CPWD_PERF_SEL_RDRAM_LATENCY_START0 = 0x00000004u32,
GC_EA_CPWD_PERF_SEL_RDRAM_LATENCY_END0   = 0x00000005u32,
GC_EA_CPWD_PERF_SEL_RDRAM_LATENCY_START1 = 0x00000006u32,
GC_EA_CPWD_PERF_SEL_RDRAM_LATENCY_END1   = 0x00000007u32,
GC_EA_CPWD_PERF_SEL_WDRAM_NUM_BANKS_VLD  = 0x00000008u32,
GC_EA_CPWD_PERF_SEL_WDRAM_REQ_PER_CLIGRP = 0x00000009u32,
GC_EA_CPWD_PERF_SEL_WDRAM_CHAINED_REQ_PER_CLIGRP = 0x0000000au32,
GC_EA_CPWD_PERF_SEL_WDRAM_LATENCY_START0 = 0x0000000bu32,
GC_EA_CPWD_PERF_SEL_WDRAM_LATENCY_END0   = 0x0000000cu32,
GC_EA_CPWD_PERF_SEL_WDRAM_LATENCY_START1 = 0x0000000du32,
GC_EA_CPWD_PERF_SEL_WDRAM_LATENCY_END1   = 0x0000000eu32,
GC_EA_CPWD_PERF_SEL_RGMI_NUM_BANKS_VLD   = 0x0000000fu32,
GC_EA_CPWD_PERF_SEL_RGMI_REQ_PER_CLIGRP  = 0x00000010u32,
GC_EA_CPWD_PERF_SEL_RGMI_CHAINED_REQ_PER_CLIGR = 0x00000011u32,
GC_EA_CPWD_PERF_SEL_RGMI_LATENCY_START0  = 0x00000012u32,
GC_EA_CPWD_PERF_SEL_RGMI_LATENCY_END0    = 0x00000013u32,
GC_EA_CPWD_PERF_SEL_RGMI_LATENCY_START1  = 0x00000014u32,
GC_EA_CPWD_PERF_SEL_RGMI_LATENCY_END1    = 0x00000015u32,
GC_EA_CPWD_PERF_SEL_WGMI_NUM_BANKS_VLD   = 0x00000016u32,
GC_EA_CPWD_PERF_SEL_WGMI_REQ_PER_CLIGRP  = 0x00000017u32,
GC_EA_CPWD_PERF_SEL_WGMI_CHAINED_REQ_PER_CLIGRP = 0x00000018u32,
GC_EA_CPWD_PERF_SEL_WGMI_LATENCY_START0  = 0x00000019u32,
GC_EA_CPWD_PERF_SEL_WGMI_LATENCY_END0    = 0x0000001au32,
GC_EA_CPWD_PERF_SEL_WGMI_LATENCY_START1  = 0x0000001bu32,
GC_EA_CPWD_PERF_SEL_WGMI_LATENCY_END1    = 0x0000001cu32,
GC_EA_CPWD_PERF_SEL_RIO_REQ_PER_CLIGRP   = 0x0000001du32,
GC_EA_CPWD_PERF_SEL_RIO_SIZE_REQ         = 0x0000001eu32,
GC_EA_CPWD_PERF_SEL_RIO_GRP0_SIZE_REQ    = 0x0000001fu32,
GC_EA_CPWD_PERF_SEL_RIO_GRP1_SIZE_REQ    = 0x00000020u32,
GC_EA_CPWD_PERF_SEL_RIO_GRP2_SIZE_REQ    = 0x00000021u32,
GC_EA_CPWD_PERF_SEL_RIO_GRP3_SIZE_REQ    = 0x00000022u32,
GC_EA_CPWD_PERF_SEL_RIO_LATENCY_START0   = 0x00000023u32,
GC_EA_CPWD_PERF_SEL_RIO_LATENCY_END0     = 0x00000024u32,
GC_EA_CPWD_PERF_SEL_RIO_LATENCY_START1   = 0x00000025u32,
GC_EA_CPWD_PERF_SEL_RIO_LATENCY_END1     = 0x00000026u32,
GC_EA_CPWD_PERF_SEL_WIO_REQ_PER_CLIGRP   = 0x00000027u32,
GC_EA_CPWD_PERF_SEL_WIO_CHAINED_REQ_PER_CLIGRP = 0x00000028u32,
GC_EA_CPWD_PERF_SEL_WIO_SIZE_REQ         = 0x00000029u32,
GC_EA_CPWD_PERF_SEL_WIO_GRP0_SIZE_REQ    = 0x0000002au32,
GC_EA_CPWD_PERF_SEL_WIO_GRP1_SIZE_REQ    = 0x0000002bu32,
GC_EA_CPWD_PERF_SEL_WIO_GRP2_SIZE_REQ    = 0x0000002cu32,
GC_EA_CPWD_PERF_SEL_WIO_GRP3_SIZE_REQ    = 0x0000002du32,
GC_EA_CPWD_PERF_SEL_WIO_LATENCY_START0   = 0x0000002eu32,
GC_EA_CPWD_PERF_SEL_WIO_LATENCY_END0     = 0x0000002fu32,
GC_EA_CPWD_PERF_SEL_WIO_LATENCY_START1   = 0x00000030u32,
GC_EA_CPWD_PERF_SEL_WIO_LATENCY_END1     = 0x00000031u32,
GC_EA_CPWD_PERF_SEL_SARB_REQ_PER_VC      = 0x00000032u32,
GC_EA_CPWD_PERF_SEL_SARB_DRAM_REQ_PER_VC = 0x00000033u32,
GC_EA_CPWD_PERF_SEL_SARB_GMI_REQ_PER_VC  = 0x00000034u32,
GC_EA_CPWD_PERF_SEL_SARB_IO_REQ_PER_VC   = 0x00000035u32,
GC_EA_CPWD_PERF_SEL_SARB_SIZE_REQ        = 0x00000036u32,
GC_EA_CPWD_PERF_SEL_SARB_DRAM_SIZE_REQ   = 0x00000037u32,
GC_EA_CPWD_PERF_SEL_SARB_GMI_SIZE_REQ    = 0x00000038u32,
GC_EA_CPWD_PERF_SEL_SARB_IO_SIZE_REQ     = 0x00000039u32,
GC_EA_CPWD_PERF_SEL_SARB_LATENCY_START0  = 0x0000003au32,
GC_EA_CPWD_PERF_SEL_SARB_LATENCY_END0    = 0x0000003bu32,
GC_EA_CPWD_PERF_SEL_SARB_LATENCY_START1  = 0x0000003cu32,
GC_EA_CPWD_PERF_SEL_SARB_LATENCY_END1    = 0x0000003du32,
GC_EA_CPWD_PERF_SEL_SARB_BUSY            = 0x0000003eu32,
GC_EA_CPWD_PERF_SEL_SARB_STALLED         = 0x0000003fu32,
GC_EA_CPWD_PERF_SEL_SARB_STARVING        = 0x00000040u32,
GC_EA_CPWD_PERF_SEL_SARB_IDLE            = 0x00000041u32,
GC_EA_CPWD_PERF_SEL_RRET_VLD             = 0x00000042u32,
GC_EA_CPWD_PERF_SEL_WRET_VLD             = 0x00000043u32,
GC_EA_CPWD_PERF_SEL_PRB_REQ              = 0x00000044u32,
GC_EA_CPWD_PERF_SEL_MAM_ARAM_FA_EVICT    = 0x00000045u32,
GC_EA_CPWD_PERF_SEL_MAM_ARAM_REQ_VLD     = 0x00000046u32,
GC_EA_CPWD_PERF_SEL_MAM_DBIT_FA_HIT      = 0x00000047u32,
GC_EA_CPWD_PERF_SEL_MAM_NUM_DQRY         = 0x00000048u32,
GC_EA_CPWD_PERF_SEL_MAM_AFLUSH_INTERRUPT = 0x00000049u32,
GC_EA_CPWD_PERF_SEL_MAM_AFLUSH_INTERRUPT_STALLED = 0x0000004au32,
GC_EA_CPWD_PERF_SEL_MAM_AFLUSH_COMPLETED = 0x0000004bu32,
GC_EA_CPWD_PERF_SEL_MAM_AFLUSH_ONGOING   = 0x0000004cu32,
GC_EA_CPWD_PERF_SEL_RDRAM_SIZE_REQ       = 0x0000004du32,
GC_EA_CPWD_PERF_SEL_WDRAM_SIZE_REQ       = 0x0000004eu32,
GC_EA_CPWD_PERF_SEL_RGMI_SIZE_REQ        = 0x0000004fu32,
GC_EA_CPWD_PERF_SEL_WGMI_SIZE_REQ        = 0x00000050u32,
GC_EA_CPWD_PERF_SEL_SARB_DRAM_RW_TURN_AROUND = 0x00000051u32,
GC_EA_CPWD_PERF_SEL_SARB_GMI_RW_TURN_AROUND = 0x00000052u32,
GC_EA_CPWD_PERF_SEL_RDRAM_CHAINED_REQ_PER_BURSTS_LENGTH = 0x00000053u32,
GC_EA_CPWD_PERF_SEL_WDRAM_CHAINED_REQ_PER_BURSTS_LENGTH = 0x00000054u32,
GC_EA_CPWD_PERF_SEL_RGMI_CHAINED_REQ_PER_BURSTS_LENGTH = 0x00000055u32,
GC_EA_CPWD_PERF_SEL_WGMI_CHAINED_REQ_PER_BURSTS_LENGTH = 0x00000056u32,
GC_EA_CPWD_PERF_SEL_MAM_DBIT_FA_EVICT    = 0x00000057u32,
GC_EA_CPWD_PERF_SEL_MAM_DBIT_REQ_VLD     = 0x00000058u32,
GC_EA_CPWD_PERF_SEL_SARB_COHERENT_SIZE_REQ = 0x00000059u32,
GC_EA_CPWD_PERF_SEL_MAM_ARAM_FA_HIT_EVICT = 0x0000005au32,
GC_EA_CPWD_PERF_SEL_MAM_ARAM_FA_LRU_EVICT = 0x0000005bu32,
GC_EA_CPWD_PERF_SEL_MAM_FLUSH_REQ        = 0x0000005cu32,
GC_EA_CPWD_PERF_SEL_MAM_FLUSH_RESP       = 0x0000005du32,
GC_EA_CPWD_PERF_SEL_MAM_DBIT_FA_HIT_EVICT = 0x0000005eu32,
GC_EA_CPWD_PERF_SEL_MAM_DBIT_FA_LRU_EVICT = 0x0000005fu32,
GC_EA_CPWD_PERF_SEL_MAM_DQRY_ONGOING     = 0x00000060u32,
GC_EA_CPWD_PERF_SEL_MAM_ARAM_FA_HIT      = 0x00000061u32,
}

/*******************************************************
 * GC_VML2PERFS Enums
 *******************************************************/

/*
 * GCVML2_SPM_PERF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GCVML2_SPM_PERF_SEL {
GCVML2_SPM_PERF_SEL_EVENT_0              = 0x00000000u32,
GCVML2_SPM_PERF_SEL_EVENT_1              = 0x00000001u32,
GCVML2_SPM_PERF_SEL_EVENT_2              = 0x00000002u32,
GCVML2_SPM_PERF_SEL_EVENT_3              = 0x00000003u32,
GCVML2_SPM_PERF_SEL_EVENT_4              = 0x00000004u32,
GCVML2_SPM_PERF_SEL_EVENT_5              = 0x00000005u32,
GCVML2_SPM_PERF_SEL_EVENT_6              = 0x00000006u32,
GCVML2_SPM_PERF_SEL_EVENT_7              = 0x00000007u32,
GCVML2_SPM_PERF_SEL_EVENT_8              = 0x00000008u32,
GCVML2_SPM_PERF_SEL_EVENT_9              = 0x00000009u32,
GCVML2_SPM_PERF_SEL_EVENT_10             = 0x0000000au32,
GCVML2_SPM_PERF_SEL_EVENT_11             = 0x0000000bu32,
GCVML2_SPM_PERF_SEL_EVENT_12             = 0x0000000cu32,
GCVML2_SPM_PERF_SEL_EVENT_13             = 0x0000000du32,
GCVML2_SPM_PERF_SEL_EVENT_14             = 0x0000000eu32,
GCVML2_SPM_PERF_SEL_EVENT_15             = 0x0000000fu32,
GCVML2_SPM_PERF_SEL_EVENT_16             = 0x00000010u32,
GCVML2_SPM_PERF_SEL_EVENT_17             = 0x00000011u32,
GCVML2_SPM_PERF_SEL_EVENT_18             = 0x00000012u32,
GCVML2_SPM_PERF_SEL_EVENT_19             = 0x00000013u32,
GCVML2_SPM_PERF_SEL_EVENT_20             = 0x00000014u32,
GCVML2_SPM_PERF_SEL_EVENT_21             = 0x00000015u32,
GCVML2_SPM_PERF_SEL_EVENT_22             = 0x00000016u32,
GCVML2_SPM_PERF_SEL_EVENT_23             = 0x00000017u32,
GCVML2_SPM_PERF_SEL_EVENT_24             = 0x00000018u32,
GCVML2_SPM_PERF_SEL_EVENT_25             = 0x00000019u32,
GCVML2_SPM_PERF_SEL_EVENT_26             = 0x0000001au32,
GCVML2_SPM_PERF_SEL_EVENT_27             = 0x0000001bu32,
GCVML2_SPM_PERF_SEL_EVENT_28             = 0x0000001cu32,
GCVML2_SPM_PERF_SEL_EVENT_29             = 0x0000001du32,
GCVML2_SPM_PERF_SEL_EVENT_30             = 0x0000001eu32,
GCVML2_SPM_PERF_SEL_EVENT_31             = 0x0000001fu32,
GCVML2_SPM_PERF_SEL_EVENT_32             = 0x00000020u32,
GCVML2_SPM_PERF_SEL_EVENT_33             = 0x00000021u32,
GCVML2_SPM_PERF_SEL_EVENT_34             = 0x00000022u32,
GCVML2_SPM_PERF_SEL_EVENT_35             = 0x00000023u32,
GCVML2_SPM_PERF_SEL_EVENT_36             = 0x00000024u32,
GCVML2_SPM_PERF_SEL_EVENT_37             = 0x00000025u32,
GCVML2_SPM_PERF_SEL_EVENT_38             = 0x00000026u32,
GCVML2_SPM_PERF_SEL_EVENT_39             = 0x00000027u32,
GCVML2_SPM_PERF_SEL_EVENT_40             = 0x00000028u32,
GCVML2_SPM_PERF_SEL_EVENT_41             = 0x00000029u32,
GCVML2_SPM_PERF_SEL_EVENT_42             = 0x0000002au32,
GCVML2_SPM_PERF_SEL_EVENT_43             = 0x0000002bu32,
GCVML2_SPM_PERF_SEL_EVENT_44             = 0x0000002cu32,
GCVML2_SPM_PERF_SEL_EVENT_45             = 0x0000002du32,
GCVML2_SPM_PERF_SEL_EVENT_46             = 0x0000002eu32,
GCVML2_SPM_PERF_SEL_EVENT_47             = 0x0000002fu32,
GCVML2_SPM_PERF_SEL_EVENT_48             = 0x00000030u32,
GCVML2_SPM_PERF_SEL_EVENT_49             = 0x00000031u32,
GCVML2_SPM_PERF_SEL_EVENT_50             = 0x00000032u32,
GCVML2_SPM_PERF_SEL_EVENT_51             = 0x00000033u32,
GCVML2_SPM_PERF_SEL_EVENT_52             = 0x00000034u32,
GCVML2_SPM_PERF_SEL_EVENT_53             = 0x00000035u32,
GCVML2_SPM_PERF_SEL_EVENT_54             = 0x00000036u32,
GCVML2_SPM_PERF_SEL_EVENT_55             = 0x00000037u32,
GCVML2_SPM_PERF_SEL_EVENT_56             = 0x00000038u32,
GCVML2_SPM_PERF_SEL_EVENT_57             = 0x00000039u32,
GCVML2_SPM_PERF_SEL_EVENT_58             = 0x0000003au32,
GCVML2_SPM_PERF_SEL_EVENT_59             = 0x0000003bu32,
GCVML2_SPM_PERF_SEL_EVENT_60             = 0x0000003cu32,
GCVML2_SPM_PERF_SEL_EVENT_61             = 0x0000003du32,
GCVML2_SPM_PERF_SEL_EVENT_62             = 0x0000003eu32,
GCVML2_SPM_PERF_SEL_EVENT_63             = 0x0000003fu32,
GCVML2_SPM_PERF_SEL_EVENT_64             = 0x00000040u32,
GCVML2_SPM_PERF_SEL_EVENT_65             = 0x00000041u32,
GCVML2_SPM_PERF_SEL_EVENT_66             = 0x00000042u32,
GCVML2_SPM_PERF_SEL_EVENT_67             = 0x00000043u32,
GCVML2_SPM_PERF_SEL_EVENT_68             = 0x00000044u32,
GCVML2_SPM_PERF_SEL_EVENT_69             = 0x00000045u32,
GCVML2_SPM_PERF_SEL_EVENT_70             = 0x00000046u32,
GCVML2_SPM_PERF_SEL_EVENT_71             = 0x00000047u32,
GCVML2_SPM_PERF_SEL_EVENT_72             = 0x00000048u32,
GCVML2_SPM_PERF_SEL_EVENT_73             = 0x00000049u32,
GCVML2_SPM_PERF_SEL_EVENT_74             = 0x0000004au32,
GCVML2_SPM_PERF_SEL_EVENT_75             = 0x0000004bu32,
GCVML2_SPM_PERF_SEL_EVENT_76             = 0x0000004cu32,
GCVML2_SPM_PERF_SEL_EVENT_77             = 0x0000004du32,
GCVML2_SPM_PERF_SEL_EVENT_78             = 0x0000004eu32,
GCVML2_SPM_PERF_SEL_EVENT_79             = 0x0000004fu32,
GCVML2_SPM_PERF_SEL_EVENT_80             = 0x00000050u32,
GCVML2_SPM_PERF_SEL_EVENT_81             = 0x00000051u32,
GCVML2_SPM_PERF_SEL_EVENT_82             = 0x00000052u32,
GCVML2_SPM_PERF_SEL_EVENT_83             = 0x00000053u32,
GCVML2_SPM_PERF_SEL_EVENT_84             = 0x00000054u32,
GCVML2_SPM_PERF_SEL_EVENT_85             = 0x00000055u32,
GCVML2_SPM_PERF_SEL_EVENT_86             = 0x00000056u32,
GCVML2_SPM_PERF_SEL_EVENT_87             = 0x00000057u32,
GCVML2_SPM_PERF_SEL_EVENT_88             = 0x00000058u32,
GCVML2_SPM_PERF_SEL_EVENT_89             = 0x00000059u32,
GCVML2_SPM_PERF_SEL_EVENT_90             = 0x0000005au32,
}

/*******************************************************
 * GC_VML2PL Enums
 *******************************************************/

/*
 * GCUTCL2_PERF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GCUTCL2_PERF_SEL {
GCUTCL2_PERF_SEL_EVENT_0                 = 0x00000000u32,
GCUTCL2_PERF_SEL_EVENT_1                 = 0x00000001u32,
GCUTCL2_PERF_SEL_EVENT_2                 = 0x00000002u32,
GCUTCL2_PERF_SEL_EVENT_3                 = 0x00000003u32,
GCUTCL2_PERF_SEL_EVENT_4                 = 0x00000004u32,
GCUTCL2_PERF_SEL_EVENT_5                 = 0x00000005u32,
GCUTCL2_PERF_SEL_EVENT_6                 = 0x00000006u32,
GCUTCL2_PERF_SEL_EVENT_7                 = 0x00000007u32,
GCUTCL2_PERF_SEL_EVENT_8                 = 0x00000008u32,
GCUTCL2_PERF_SEL_EVENT_9                 = 0x00000009u32,
GCUTCL2_PERF_SEL_EVENT_10                = 0x0000000au32,
GCUTCL2_PERF_SEL_EVENT_11                = 0x0000000bu32,
GCUTCL2_PERF_SEL_EVENT_12                = 0x0000000cu32,
GCUTCL2_PERF_SEL_EVENT_13                = 0x0000000du32,
GCUTCL2_PERF_SEL_EVENT_14                = 0x0000000eu32,
GCUTCL2_PERF_SEL_EVENT_15                = 0x0000000fu32,
GCUTCL2_PERF_SEL_EVENT_16                = 0x00000010u32,
GCUTCL2_PERF_SEL_EVENT_17                = 0x00000011u32,
GCUTCL2_PERF_SEL_EVENT_18                = 0x00000012u32,
GCUTCL2_PERF_SEL_EVENT_19                = 0x00000013u32,
GCUTCL2_PERF_SEL_EVENT_20                = 0x00000014u32,
GCUTCL2_PERF_SEL_EVENT_21                = 0x00000015u32,
GCUTCL2_PERF_SEL_EVENT_22                = 0x00000016u32,
GCUTCL2_PERF_SEL_EVENT_23                = 0x00000017u32,
GCUTCL2_PERF_SEL_EVENT_24                = 0x00000018u32,
GCUTCL2_PERF_SEL_EVENT_25                = 0x00000019u32,
GCUTCL2_PERF_SEL_EVENT_26                = 0x0000001au32,
GCUTCL2_PERF_SEL_EVENT_27                = 0x0000001bu32,
GCUTCL2_PERF_SEL_EVENT_28                = 0x0000001cu32,
GCUTCL2_PERF_SEL_EVENT_29                = 0x0000001du32,
GCUTCL2_PERF_SEL_EVENT_30                = 0x0000001eu32,
GCUTCL2_PERF_SEL_EVENT_31                = 0x0000001fu32,
GCUTCL2_PERF_SEL_EVENT_32                = 0x00000020u32,
GCUTCL2_PERF_SEL_EVENT_33                = 0x00000021u32,
GCUTCL2_PERF_SEL_EVENT_34                = 0x00000022u32,
GCUTCL2_PERF_SEL_EVENT_35                = 0x00000023u32,
GCUTCL2_PERF_SEL_EVENT_36                = 0x00000024u32,
}

/*
 * GCVML2_PERF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GCVML2_PERF_SEL {
GCVML2_PERF_SEL_EVENT_0                  = 0x00000000u32,
GCVML2_PERF_SEL_EVENT_1                  = 0x00000001u32,
GCVML2_PERF_SEL_EVENT_2                  = 0x00000002u32,
GCVML2_PERF_SEL_EVENT_3                  = 0x00000003u32,
GCVML2_PERF_SEL_EVENT_4                  = 0x00000004u32,
GCVML2_PERF_SEL_EVENT_5                  = 0x00000005u32,
GCVML2_PERF_SEL_EVENT_6                  = 0x00000006u32,
GCVML2_PERF_SEL_EVENT_7                  = 0x00000007u32,
GCVML2_PERF_SEL_EVENT_8                  = 0x00000008u32,
GCVML2_PERF_SEL_EVENT_9                  = 0x00000009u32,
GCVML2_PERF_SEL_EVENT_10                 = 0x0000000au32,
GCVML2_PERF_SEL_EVENT_11                 = 0x0000000bu32,
GCVML2_PERF_SEL_EVENT_12                 = 0x0000000cu32,
GCVML2_PERF_SEL_EVENT_13                 = 0x0000000du32,
GCVML2_PERF_SEL_EVENT_14                 = 0x0000000eu32,
GCVML2_PERF_SEL_EVENT_15                 = 0x0000000fu32,
GCVML2_PERF_SEL_EVENT_16                 = 0x00000010u32,
GCVML2_PERF_SEL_EVENT_17                 = 0x00000011u32,
GCVML2_PERF_SEL_EVENT_18                 = 0x00000012u32,
GCVML2_PERF_SEL_EVENT_19                 = 0x00000013u32,
GCVML2_PERF_SEL_EVENT_20                 = 0x00000014u32,
GCVML2_PERF_SEL_EVENT_21                 = 0x00000015u32,
GCVML2_PERF_SEL_EVENT_22                 = 0x00000016u32,
GCVML2_PERF_SEL_EVENT_23                 = 0x00000017u32,
GCVML2_PERF_SEL_EVENT_24                 = 0x00000018u32,
GCVML2_PERF_SEL_EVENT_25                 = 0x00000019u32,
GCVML2_PERF_SEL_EVENT_26                 = 0x0000001au32,
GCVML2_PERF_SEL_EVENT_27                 = 0x0000001bu32,
GCVML2_PERF_SEL_EVENT_28                 = 0x0000001cu32,
GCVML2_PERF_SEL_EVENT_29                 = 0x0000001du32,
GCVML2_PERF_SEL_EVENT_30                 = 0x0000001eu32,
GCVML2_PERF_SEL_EVENT_31                 = 0x0000001fu32,
GCVML2_PERF_SEL_EVENT_32                 = 0x00000020u32,
GCVML2_PERF_SEL_EVENT_33                 = 0x00000021u32,
GCVML2_PERF_SEL_EVENT_34                 = 0x00000022u32,
GCVML2_PERF_SEL_EVENT_35                 = 0x00000023u32,
GCVML2_PERF_SEL_EVENT_36                 = 0x00000024u32,
GCVML2_PERF_SEL_EVENT_37                 = 0x00000025u32,
GCVML2_PERF_SEL_EVENT_38                 = 0x00000026u32,
GCVML2_PERF_SEL_EVENT_39                 = 0x00000027u32,
GCVML2_PERF_SEL_EVENT_40                 = 0x00000028u32,
GCVML2_PERF_SEL_EVENT_41                 = 0x00000029u32,
GCVML2_PERF_SEL_EVENT_42                 = 0x0000002au32,
GCVML2_PERF_SEL_EVENT_43                 = 0x0000002bu32,
GCVML2_PERF_SEL_EVENT_44                 = 0x0000002cu32,
GCVML2_PERF_SEL_EVENT_45                 = 0x0000002du32,
GCVML2_PERF_SEL_EVENT_46                 = 0x0000002eu32,
GCVML2_PERF_SEL_EVENT_47                 = 0x0000002fu32,
GCVML2_PERF_SEL_EVENT_48                 = 0x00000030u32,
GCVML2_PERF_SEL_EVENT_49                 = 0x00000031u32,
GCVML2_PERF_SEL_EVENT_50                 = 0x00000032u32,
GCVML2_PERF_SEL_EVENT_51                 = 0x00000033u32,
GCVML2_PERF_SEL_EVENT_52                 = 0x00000034u32,
GCVML2_PERF_SEL_EVENT_53                 = 0x00000035u32,
GCVML2_PERF_SEL_EVENT_54                 = 0x00000036u32,
GCVML2_PERF_SEL_EVENT_55                 = 0x00000037u32,
GCVML2_PERF_SEL_EVENT_56                 = 0x00000038u32,
GCVML2_PERF_SEL_EVENT_57                 = 0x00000039u32,
GCVML2_PERF_SEL_EVENT_58                 = 0x0000003au32,
GCVML2_PERF_SEL_EVENT_59                 = 0x0000003bu32,
GCVML2_PERF_SEL_EVENT_60                 = 0x0000003cu32,
GCVML2_PERF_SEL_EVENT_61                 = 0x0000003du32,
GCVML2_PERF_SEL_EVENT_62                 = 0x0000003eu32,
GCVML2_PERF_SEL_EVENT_63                 = 0x0000003fu32,
GCVML2_PERF_SEL_EVENT_64                 = 0x00000040u32,
GCVML2_PERF_SEL_EVENT_65                 = 0x00000041u32,
GCVML2_PERF_SEL_EVENT_66                 = 0x00000042u32,
GCVML2_PERF_SEL_EVENT_67                 = 0x00000043u32,
GCVML2_PERF_SEL_EVENT_68                 = 0x00000044u32,
GCVML2_PERF_SEL_EVENT_69                 = 0x00000045u32,
GCVML2_PERF_SEL_EVENT_70                 = 0x00000046u32,
GCVML2_PERF_SEL_EVENT_71                 = 0x00000047u32,
GCVML2_PERF_SEL_EVENT_72                 = 0x00000048u32,
GCVML2_PERF_SEL_EVENT_73                 = 0x00000049u32,
GCVML2_PERF_SEL_EVENT_74                 = 0x0000004au32,
GCVML2_PERF_SEL_EVENT_75                 = 0x0000004bu32,
GCVML2_PERF_SEL_EVENT_76                 = 0x0000004cu32,
GCVML2_PERF_SEL_EVENT_77                 = 0x0000004du32,
GCVML2_PERF_SEL_EVENT_78                 = 0x0000004eu32,
GCVML2_PERF_SEL_EVENT_79                 = 0x0000004fu32,
GCVML2_PERF_SEL_EVENT_80                 = 0x00000050u32,
GCVML2_PERF_SEL_EVENT_81                 = 0x00000051u32,
GCVML2_PERF_SEL_EVENT_82                 = 0x00000052u32,
GCVML2_PERF_SEL_EVENT_83                 = 0x00000053u32,
GCVML2_PERF_SEL_EVENT_84                 = 0x00000054u32,
GCVML2_PERF_SEL_EVENT_85                 = 0x00000055u32,
GCVML2_PERF_SEL_EVENT_86                 = 0x00000056u32,
GCVML2_PERF_SEL_EVENT_87                 = 0x00000057u32,
GCVML2_PERF_SEL_EVENT_88                 = 0x00000058u32,
GCVML2_PERF_SEL_EVENT_89                 = 0x00000059u32,
GCVML2_PERF_SEL_EVENT_90                 = 0x0000005au32,
}

/*******************************************************
 * CB Enums
 *******************************************************/

/*
 * BlendOp enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum BlendOp {
BLEND_ZERO                               = 0x00000000u32,
BLEND_ONE                                = 0x00000001u32,
BLEND_SRC_COLOR                          = 0x00000002u32,
BLEND_ONE_MINUS_SRC_COLOR                = 0x00000003u32,
BLEND_SRC_ALPHA                          = 0x00000004u32,
BLEND_ONE_MINUS_SRC_ALPHA                = 0x00000005u32,
BLEND_DST_ALPHA                          = 0x00000006u32,
BLEND_ONE_MINUS_DST_ALPHA                = 0x00000007u32,
BLEND_DST_COLOR                          = 0x00000008u32,
BLEND_ONE_MINUS_DST_COLOR                = 0x00000009u32,
BLEND_SRC_ALPHA_SATURATE                 = 0x0000000au32,
BLEND_CONSTANT_COLOR                     = 0x0000000bu32,
BLEND_ONE_MINUS_CONSTANT_COLOR           = 0x0000000cu32,
BLEND_SRC1_COLOR                         = 0x0000000du32,
BLEND_INV_SRC1_COLOR                     = 0x0000000eu32,
BLEND_SRC1_ALPHA                         = 0x0000000fu32,
BLEND_INV_SRC1_ALPHA                     = 0x00000010u32,
BLEND_CONSTANT_ALPHA                     = 0x00000011u32,
BLEND_ONE_MINUS_CONSTANT_ALPHA           = 0x00000012u32,
}

/*
 * BlendOpt enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum BlendOpt {
FORCE_OPT_AUTO                           = 0x00000000u32,
FORCE_OPT_DISABLE                        = 0x00000001u32,
FORCE_OPT_ENABLE_IF_SRC_A_0              = 0x00000002u32,
FORCE_OPT_ENABLE_IF_SRC_RGB_0            = 0x00000003u32,
FORCE_OPT_ENABLE_IF_SRC_ARGB_0           = 0x00000004u32,
FORCE_OPT_ENABLE_IF_SRC_A_1              = 0x00000005u32,
FORCE_OPT_ENABLE_IF_SRC_RGB_1            = 0x00000006u32,
FORCE_OPT_ENABLE_IF_SRC_ARGB_1           = 0x00000007u32,
}

/*
 * CBMode enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CBMode {
CB_DISABLE                               = 0x00000000u32,
CB_NORMAL                                = 0x00000001u32,
CB_ELIMINATE_FAST_CLEAR                  = 0x00000002u32,
CB_DCC_DECOMPRESS                        = 0x00000003u32,
CB_RESERVED                              = 0x00000004u32,
}

/*
 * CBPerfClearFilterSel enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CBPerfClearFilterSel {
CB_PERF_CLEAR_FILTER_SEL_NONCLEAR        = 0x00000000u32,
CB_PERF_CLEAR_FILTER_SEL_CLEAR           = 0x00000001u32,
}

/*
 * CBPerfOpFilterSel enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CBPerfOpFilterSel {
CB_PERF_OP_FILTER_SEL_WRITE_ONLY         = 0x00000000u32,
CB_PERF_OP_FILTER_SEL_NEEDS_DESTINATION  = 0x00000001u32,
CB_PERF_OP_FILTER_SEL_RESOLVE            = 0x00000002u32,
CB_PERF_OP_FILTER_SEL_DECOMPRESS         = 0x00000003u32,
CB_PERF_OP_FILTER_SEL_FMASK_DECOMPRESS   = 0x00000004u32,
CB_PERF_OP_FILTER_SEL_ELIMINATE_FAST_CLEAR = 0x00000005u32,
}

/*
 * CBPerfSel enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CBPerfSel {
CB_PERF_SEL_BUSY                         = 0x00000001u32,
CB_PERF_SEL_DRAWN_BUSY                   = 0x00000002u32,
CB_PERF_SEL_DRAWN_PIXEL                  = 0x00000003u32,
CB_PERF_SEL_DRAWN_QUAD                   = 0x00000004u32,
CB_PERF_SEL_DRAWN_QUAD_FRAGMENT          = 0x00000005u32,
CB_PERF_SEL_DB_CB_EXPORT_VALID_READY     = 0x0000000fu32,
CB_PERF_SEL_DB_CB_EXPORT_VALID_READYB    = 0x00000010u32,
CB_PERF_SEL_DB_CB_EXPORT_VALIDB_READY    = 0x00000011u32,
CB_PERF_SEL_DB_CB_EXPORT_VALIDB_READYB   = 0x00000012u32,
CB_PERF_SEL_CC_CRW_GLX_REQ_READ_REQUEST  = 0x00000015u32,
CB_PERF_SEL_CC_CRW_GLX_REQ_READ_REQUEST_IN_FLIGHT = 0x00000016u32,
CB_PERF_SEL_CC_CRW_GLX_REQ_WRITE_REQUEST = 0x00000017u32,
CB_PERF_SEL_CC_CRW_GLX_SRC_WRITE_CYCLES  = 0x00000018u32,
CB_PERF_SEL_CC_FDCC_COMPRESS_FRAG_TIDS_IN = 0x00000019u32,
CB_PERF_SEL_CC_FDCC_DECOMPRESS_FRAG_TIDS_OUT = 0x0000001au32,
CB_PERF_SEL_EVENT                        = 0x00000032u32,
CB_PERF_SEL_EVENT_CACHE_FLUSH_TS         = 0x00000033u32,
CB_PERF_SEL_EVENT_CONTEXT_DONE           = 0x00000034u32,
CB_PERF_SEL_EVENT_CACHE_FLUSH            = 0x00000035u32,
CB_PERF_SEL_EVENT_CACHE_FLUSH_AND_INV_TS_EVENT = 0x00000036u32,
CB_PERF_SEL_EVENT_CACHE_FLUSH_AND_INV_EVENT = 0x00000037u32,
CB_PERF_SEL_EVENT_FLUSH_AND_INV_CB_DATA_TS = 0x00000038u32,
CB_PERF_SEL_EVENT_FLUSH_AND_INV_CB_META  = 0x00000039u32,
CB_PERF_SEL_EVENT_BOTTOM_OF_PIPE_TS      = 0x0000003au32,
CB_PERF_SEL_STATIC_CLOCK_EN              = 0x0000003cu32,
CB_PERF_SEL_PERFMON_CLOCK_EN             = 0x0000003du32,
CB_PERF_SEL_BLEND_CLOCK_EN               = 0x0000003eu32,
CB_PERF_SEL_COLOR_STORE_CLOCK_EN         = 0x0000003fu32,
CB_PERF_SEL_BACKEND_READ_CLOCK_EN        = 0x00000040u32,
CB_PERF_SEL_GRBM_CLOCK_EN                = 0x00000041u32,
CB_PERF_SEL_MEMARB_CLOCK_EN              = 0x00000042u32,
CB_PERF_SEL_BACKEND_EVICT_PIPE_CLOCK_EN  = 0x00000043u32,
CB_PERF_SEL_BACKEND_FRAGOP_CLOCK_EN      = 0x00000044u32,
CB_PERF_SEL_BACKEND_SRC_FIFO_CLOCK_EN    = 0x00000045u32,
CB_PERF_SEL_BACKEND_CACHE_CTL_CLOCK_EN   = 0x00000046u32,
CB_PERF_SEL_FRONTEND_INPUT_CLOCK_EN      = 0x00000047u32,
CB_PERF_SEL_FRONTEND_ADDR_CLOCK_EN       = 0x00000048u32,
CB_PERF_SEL_FRONTEND_FDCC_CLOCK_EN       = 0x00000049u32,
CB_PERF_SEL_FRONTEND_SAMPLE_MASK_TRACKER_CLOCK_EN = 0x0000004au32,
CB_PERF_SEL_EVENTS_CLK_EN                = 0x0000004bu32,
CB_PERF_SEL_CC_TAG_HIT                   = 0x00000050u32,
CB_PERF_SEL_CC_CACHE_TAG_MISS            = 0x00000051u32,
CB_PERF_SEL_CC_CACHE_SECTOR_MISS         = 0x00000052u32,
CB_PERF_SEL_CC_CACHE_SECTOR_HIT          = 0x00000053u32,
CB_PERF_SEL_CC_CACHE_READ_OUTPUT_STALL   = 0x00000058u32,
CB_PERF_SEL_CC_CACHE_WRITE_OUTPUT_STALL  = 0x00000059u32,
CB_PERF_SEL_CC_CACHE_ACK_OUTPUT_STALL    = 0x0000005au32,
CB_PERF_SEL_CC_CACHE_STALL               = 0x0000005bu32,
CB_PERF_SEL_CC_CACHE_FLUSH               = 0x0000005cu32,
CB_PERF_SEL_CC_CACHE_SECTORS_FLUSHED     = 0x0000005du32,
CB_PERF_SEL_CC_CACHE_WA_TO_RMW_CONVERSION = 0x0000005eu32,
CB_PERF_SEL_CC_CACHE_QBLOCKS_FLUSHED     = 0x0000005fu32,
CB_PERF_SEL_CC_CACHE_DIRTY_QBLOCKS_FLUSHED = 0x00000060u32,
CB_PERF_SEL_CC_CACHE_READS_SAVED_DUE_TO_DCC = 0x00000061u32,
CB_PERF_SEL_CCC_IN_EVICT_HAZARD_STALL    = 0x00000062u32,
CB_PERF_SEL_CCC_COLOR_RESOURCE_PANIC     = 0x00000063u32,
CB_PERF_SEL_CCC_FMASK_RESOURCE_PANIC     = 0x00000064u32,
CB_PERF_SEL_CCC_FREE_WAYS_PANIC          = 0x00000065u32,
CB_PERF_SEL_CCC_SKID_FIFO_FULL           = 0x00000066u32,
CB_PERF_SEL_CCC_SKID_FIFO_STALL          = 0x00000067u32,
CB_PERF_SEL_CCC_COLOR_RESOURCE_STALL     = 0x00000068u32,
CB_PERF_SEL_CCC_FMASK_RESOURCE_STALL     = 0x00000069u32,
CB_PERF_SEL_CCC_FREE_WAYS_STALL          = 0x0000006au32,
CB_PERF_SEL_BE_SRCFIFO_FULL              = 0x0000006eu32,
CB_PERF_SEL_BE_RDLATFIFO_FULL            = 0x0000006fu32,
CB_PERF_SEL_RDLAT_FIFO_QUAD_RESIDENCY_STALL = 0x00000070u32,
CB_PERF_SEL_CC_QUADFRAG_VALID_READY      = 0x00000071u32,
CB_PERF_SEL_CC_QUADFRAG_VALID_READYB     = 0x00000072u32,
CB_PERF_SEL_CC_QUADFRAG_VALIDB_READY     = 0x00000073u32,
CB_PERF_SEL_CC_QUADFRAG_VALIDB_READYB    = 0x00000074u32,
CB_PERF_SEL_CC_BB_BLEND_PIXEL_VALID_READY = 0x00000076u32,
CB_PERF_SEL_CC_BB_BLEND_PIXEL_VALID_READYB = 0x00000077u32,
CB_PERF_SEL_CC_BB_BLEND_PIXEL_VALIDB_READY = 0x00000078u32,
CB_PERF_SEL_CC_BB_BLEND_PIXEL_VALIDB_READYB = 0x00000079u32,
CB_PERF_SEL_RBP_EXPORT_8PIX_LIT_BOTH     = 0x00000096u32,
CB_PERF_SEL_RBP_EXPORT_8PIX_LIT_LEFT     = 0x00000097u32,
CB_PERF_SEL_RBP_EXPORT_8PIX_LIT_RIGHT    = 0x00000098u32,
CB_PERF_SEL_BLEND_QUAD_DST_READ_COULD_HAVE_BEEN_OPTIMIZED = 0x000000b4u32,
CB_PERF_SEL_BLEND_QUAD_BLENDING_COULD_HAVE_BEEN_BYPASSED = 0x000000b5u32,
CB_PERF_SEL_BLEND_QUAD_COULD_HAVE_BEEN_DISCARDED = 0x000000b6u32,
CB_PERF_SEL_BLEND_OPT_PIXELS_RESULT_EQ_DEST = 0x000000b7u32,
CB_PERF_SEL_BLEND_STALL_AT_OUTPUT        = 0x000000b8u32,
CB_PERF_SEL_BLEND_STALL_ON_CACHE_ACCESS  = 0x000000b9u32,
CB_PERF_SEL_BLEND_COLLISION_DUE_TO_CACHE_WRITE = 0x000000bau32,
CB_PERF_SEL_BLEND_RAW_HAZARD_STALL       = 0x000000bbu32,
CB_PERF_SEL_BE_CS_FILLRATE_1X2           = 0x000000beu32,
CB_PERF_SEL_BE_CS_FILLRATE_2X1           = 0x000000bfu32,
CB_PERF_SEL_BE_CS_FILLRATE_2X2           = 0x000000c0u32,
CB_PERF_SEL_FORMAT_IS_32_R               = 0x000000fau32,
CB_PERF_SEL_FORMAT_IS_32_AR              = 0x000000fbu32,
CB_PERF_SEL_FORMAT_IS_32_GR              = 0x000000fcu32,
CB_PERF_SEL_FORMAT_IS_32_ABGR            = 0x000000fdu32,
CB_PERF_SEL_FORMAT_IS_FP16_ABGR          = 0x000000feu32,
CB_PERF_SEL_FORMAT_IS_SIGNED16_ABGR      = 0x000000ffu32,
CB_PERF_SEL_FORMAT_IS_UNSIGNED16_ABGR    = 0x00000100u32,
CB_PERF_SEL_FORMAT_IS_32BPP_8PIX         = 0x00000101u32,
CB_PERF_SEL_FORMAT_IS_16_16_UNSIGNED_8PIX = 0x00000102u32,
CB_PERF_SEL_FORMAT_IS_16_16_SIGNED_8PIX  = 0x00000103u32,
CB_PERF_SEL_FORMAT_IS_16_16_FLOAT_8PIX   = 0x00000104u32,
CB_PERF_SEL_EXPORT_ADDED_1_FRAGMENT      = 0x00000105u32,
CB_PERF_SEL_EXPORT_ADDED_2_FRAGMENTS     = 0x00000106u32,
CB_PERF_SEL_EXPORT_ADDED_3_FRAGMENTS     = 0x00000107u32,
CB_PERF_SEL_EXPORT_ADDED_4_FRAGMENTS     = 0x00000108u32,
CB_PERF_SEL_EXPORT_ADDED_5_FRAGMENTS     = 0x00000109u32,
CB_PERF_SEL_EXPORT_ADDED_6_FRAGMENTS     = 0x0000010au32,
CB_PERF_SEL_EXPORT_ADDED_7_FRAGMENTS     = 0x0000010bu32,
CB_PERF_SEL_EXPORT_BLEND_OPT_DONT_READ_DST = 0x0000010cu32,
CB_PERF_SEL_EXPORT_BLEND_OPT_BLEND_BYPASS = 0x0000010du32,
CB_PERF_SEL_EXPORT_BLEND_OPT_DISCARD_PIXELS = 0x0000010eu32,
CB_PERF_SEL_EXPORT_HAS_1_FRAGMENT_BEFORE_UPDATE = 0x0000010fu32,
CB_PERF_SEL_EXPORT_HAS_1_FRAGMENT_AFTER_UPDATE = 0x00000110u32,
CB_PERF_SEL_EXPORT_HAS_2_FRAGMENTS_BEFORE_UPDATE = 0x00000111u32,
CB_PERF_SEL_EXPORT_HAS_2_FRAGMENTS_AFTER_UPDATE = 0x00000112u32,
CB_PERF_SEL_EXPORT_HAS_3_FRAGMENTS_BEFORE_UPDATE = 0x00000113u32,
CB_PERF_SEL_EXPORT_HAS_3_FRAGMENTS_AFTER_UPDATE = 0x00000114u32,
CB_PERF_SEL_EXPORT_HAS_4_FRAGMENTS_BEFORE_UPDATE = 0x00000115u32,
CB_PERF_SEL_EXPORT_HAS_4_FRAGMENTS_AFTER_UPDATE = 0x00000116u32,
CB_PERF_SEL_EXPORT_HAS_5_FRAGMENTS_BEFORE_UPDATE = 0x00000117u32,
CB_PERF_SEL_EXPORT_HAS_5_FRAGMENTS_AFTER_UPDATE = 0x00000118u32,
CB_PERF_SEL_EXPORT_HAS_6_FRAGMENTS_BEFORE_UPDATE = 0x00000119u32,
CB_PERF_SEL_EXPORT_HAS_6_FRAGMENTS_AFTER_UPDATE = 0x0000011au32,
CB_PERF_SEL_EXPORT_HAS_7_FRAGMENTS_BEFORE_UPDATE = 0x0000011bu32,
CB_PERF_SEL_EXPORT_HAS_7_FRAGMENTS_AFTER_UPDATE = 0x0000011cu32,
CB_PERF_SEL_EXPORT_HAS_8_FRAGMENTS_BEFORE_UPDATE = 0x0000011du32,
CB_PERF_SEL_EXPORT_HAS_8_FRAGMENTS_AFTER_UPDATE = 0x0000011eu32,
CB_PERF_SEL_EXPORT_READS_FRAGMENT_0      = 0x0000011fu32,
CB_PERF_SEL_EXPORT_READS_FRAGMENT_1      = 0x00000120u32,
CB_PERF_SEL_EXPORT_READS_FRAGMENT_2      = 0x00000121u32,
CB_PERF_SEL_EXPORT_READS_FRAGMENT_3      = 0x00000122u32,
CB_PERF_SEL_EXPORT_READS_FRAGMENT_4      = 0x00000123u32,
CB_PERF_SEL_EXPORT_READS_FRAGMENT_5      = 0x00000124u32,
CB_PERF_SEL_EXPORT_READS_FRAGMENT_6      = 0x00000125u32,
CB_PERF_SEL_EXPORT_READS_FRAGMENT_7      = 0x00000126u32,
CB_PERF_SEL_EXPORT_REMOVED_1_FRAGMENT    = 0x00000127u32,
CB_PERF_SEL_EXPORT_REMOVED_2_FRAGMENTS   = 0x00000128u32,
CB_PERF_SEL_EXPORT_REMOVED_3_FRAGMENTS   = 0x00000129u32,
CB_PERF_SEL_EXPORT_REMOVED_4_FRAGMENTS   = 0x0000012au32,
CB_PERF_SEL_EXPORT_REMOVED_5_FRAGMENTS   = 0x0000012bu32,
CB_PERF_SEL_EXPORT_REMOVED_6_FRAGMENTS   = 0x0000012cu32,
CB_PERF_SEL_EXPORT_REMOVED_7_FRAGMENTS   = 0x0000012du32,
CB_PERF_SEL_EXPORT_WRITES_FRAGMENT_0     = 0x0000012eu32,
CB_PERF_SEL_EXPORT_WRITES_FRAGMENT_1     = 0x0000012fu32,
CB_PERF_SEL_EXPORT_WRITES_FRAGMENT_2     = 0x00000130u32,
CB_PERF_SEL_EXPORT_WRITES_FRAGMENT_3     = 0x00000131u32,
CB_PERF_SEL_EXPORT_WRITES_FRAGMENT_4     = 0x00000132u32,
CB_PERF_SEL_EXPORT_WRITES_FRAGMENT_5     = 0x00000133u32,
CB_PERF_SEL_EXPORT_WRITES_FRAGMENT_6     = 0x00000134u32,
CB_PERF_SEL_EXPORT_WRITES_FRAGMENT_7     = 0x00000135u32,
CB_PERF_SEL_EXPORT_KILLED_BY_COLOR_INVALID = 0x00000136u32,
CB_PERF_SEL_EXPORT_KILLED_BY_DISCARD_PIXEL = 0x00000137u32,
CB_PERF_SEL_EXPORT_KILLED_BY_NULL_SAMPLE_MASK = 0x00000138u32,
CB_PERF_SEL_EXPORT_KILLED_BY_NULL_TARGET_SHADER_MASK = 0x00000139u32,
}

/*
 * CombFunc enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CombFunc {
COMB_DST_PLUS_SRC                        = 0x00000000u32,
COMB_SRC_MINUS_DST                       = 0x00000001u32,
COMB_MIN_DST_SRC                         = 0x00000002u32,
COMB_MAX_DST_SRC                         = 0x00000003u32,
COMB_DST_MINUS_SRC                       = 0x00000004u32,
}

/*
 * MemArbMode enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MemArbMode {
MEM_ARB_MODE_FIXED                       = 0x00000000u32,
MEM_ARB_MODE_AGE                         = 0x00000001u32,
MEM_ARB_MODE_WEIGHT                      = 0x00000002u32,
MEM_ARB_MODE_BOTH                        = 0x00000003u32,
}

/*******************************************************
 * PH Enums
 *******************************************************/

/*
 * PH_PERFCNT_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PH_PERFCNT_SEL {
PH_PERF_SEL_SC0_SRPS_WINDOW_VALID        = 0x00000000u32,
PH_PERF_SEL_SC0_ARB_XFC_ALL_EVENT_OR_PRIM_CYCLES = 0x00000001u32,
PH_PERF_SEL_SC0_ARB_XFC_ONLY_PRIM_CYCLES = 0x00000002u32,
PH_PERF_SEL_SC0_ARB_XFC_ONLY_ONE_INC_PER_PRIM = 0x00000003u32,
PH_PERF_SEL_SC0_ARB_STALLED_FROM_BELOW   = 0x00000004u32,
PH_PERF_SEL_SC0_ARB_STARVED_FROM_ABOVE   = 0x00000005u32,
PH_PERF_SEL_SC0_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_NOT_EMPTY = 0x00000006u32,
PH_PERF_SEL_SC0_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_FULL = 0x00000007u32,
PH_PERF_SEL_SC0_ARB_BUSY                 = 0x00000008u32,
PH_PERF_SEL_SC0_ARB_PA_BUSY_SOP          = 0x00000009u32,
PH_PERF_SEL_SC0_ARB_EOP_POP_SYNC_POP     = 0x0000000au32,
PH_PERF_SEL_SC0_ARB_EVENT_SYNC_POP       = 0x0000000bu32,
PH_PERF_SEL_SC0_PS_ENG_MULTICYCLE_BUBBLE = 0x0000000cu32,
PH_PERF_SEL_SC0_EOP_SYNC_WINDOW          = 0x0000000du32,
PH_PERF_SEL_SC0_BUSY_PROCESSING_MULTICYCLE_PRIM = 0x0000000eu32,
PH_PERF_SEL_SC0_BUSY_CNT_NOT_ZERO        = 0x0000000fu32,
PH_PERF_SEL_SC0_SEND                     = 0x00000010u32,
PH_PERF_SEL_SC0_CREDIT_AT_ZERO_WITH_PENDING_SEND = 0x00000011u32,
PH_PERF_SEL_SC0_CREDIT_AT_MAX            = 0x00000012u32,
PH_PERF_SEL_SC0_CREDIT_AT_MAX_NO_PENDING_SEND = 0x00000013u32,
PH_PERF_SEL_SC0_GFX_PIPE0_TO_1_TRANSITION = 0x00000014u32,
PH_PERF_SEL_SC0_GFX_PIPE1_TO_0_TRANSITION = 0x00000015u32,
PH_PERF_SEL_SC0_GFX_PIPE_PRIM_PROVOKED_TRANSITION = 0x00000016u32,
PH_PERF_SEL_SC0_GFX_PIPE_EVENT_PROVOKED_TRANSITION = 0x00000017u32,
PH_PERF_SEL_SC0_PA0_DATA_FIFO_RD         = 0x00000018u32,
PH_PERF_SEL_SC0_PA0_DATA_FIFO_WE         = 0x00000019u32,
PH_PERF_SEL_SC0_PA0_FIFO_EMPTY           = 0x0000001au32,
PH_PERF_SEL_SC0_PA0_FIFO_FULL            = 0x0000001bu32,
PH_PERF_SEL_SC0_PA0_NULL_WE              = 0x0000001cu32,
PH_PERF_SEL_SC0_PA0_EVENT_WE             = 0x0000001du32,
PH_PERF_SEL_SC0_PA0_FPOV_WE              = 0x0000001eu32,
PH_PERF_SEL_SC0_PA0_FPOP_WE              = 0x0000001fu32,
PH_PERF_SEL_SC0_PA0_EOP_WE               = 0x00000020u32,
PH_PERF_SEL_SC0_PA0_DATA_FIFO_EOP_RD     = 0x00000021u32,
PH_PERF_SEL_SC0_PA0_EOPG_WE              = 0x00000022u32,
PH_PERF_SEL_SC0_PA0_DEALLOC_WE           = 0x00000023u32,
PH_PERF_SEL_SC0_PA1_DATA_FIFO_RD         = 0x00000024u32,
PH_PERF_SEL_SC0_PA1_DATA_FIFO_WE         = 0x00000025u32,
PH_PERF_SEL_SC0_PA1_FIFO_EMPTY           = 0x00000026u32,
PH_PERF_SEL_SC0_PA1_FIFO_FULL            = 0x00000027u32,
PH_PERF_SEL_SC0_PA1_NULL_WE              = 0x00000028u32,
PH_PERF_SEL_SC0_PA1_EVENT_WE             = 0x00000029u32,
PH_PERF_SEL_SC0_PA1_FPOV_WE              = 0x0000002au32,
PH_PERF_SEL_SC0_PA1_FPOP_WE              = 0x0000002bu32,
PH_PERF_SEL_SC0_PA1_EOP_WE               = 0x0000002cu32,
PH_PERF_SEL_SC0_PA1_DATA_FIFO_EOP_RD     = 0x0000002du32,
PH_PERF_SEL_SC0_PA1_EOPG_WE              = 0x0000002eu32,
PH_PERF_SEL_SC0_PA1_DEALLOC_WE           = 0x0000002fu32,
PH_PERF_SEL_SC0_PA2_DATA_FIFO_RD         = 0x00000030u32,
PH_PERF_SEL_SC0_PA2_DATA_FIFO_WE         = 0x00000031u32,
PH_PERF_SEL_SC0_PA2_FIFO_EMPTY           = 0x00000032u32,
PH_PERF_SEL_SC0_PA2_FIFO_FULL            = 0x00000033u32,
PH_PERF_SEL_SC0_PA2_NULL_WE              = 0x00000034u32,
PH_PERF_SEL_SC0_PA2_EVENT_WE             = 0x00000035u32,
PH_PERF_SEL_SC0_PA2_FPOV_WE              = 0x00000036u32,
PH_PERF_SEL_SC0_PA2_FPOP_WE              = 0x00000037u32,
PH_PERF_SEL_SC0_PA2_EOP_WE               = 0x00000038u32,
PH_PERF_SEL_SC0_PA2_DATA_FIFO_EOP_RD     = 0x00000039u32,
PH_PERF_SEL_SC0_PA2_EOPG_WE              = 0x0000003au32,
PH_PERF_SEL_SC0_PA2_DEALLOC_WE           = 0x0000003bu32,
PH_PERF_SEL_SC0_PA3_DATA_FIFO_RD         = 0x0000003cu32,
PH_PERF_SEL_SC0_PA3_DATA_FIFO_WE         = 0x0000003du32,
PH_PERF_SEL_SC0_PA3_FIFO_EMPTY           = 0x0000003eu32,
PH_PERF_SEL_SC0_PA3_FIFO_FULL            = 0x0000003fu32,
PH_PERF_SEL_SC0_PA3_NULL_WE              = 0x00000040u32,
PH_PERF_SEL_SC0_PA3_EVENT_WE             = 0x00000041u32,
PH_PERF_SEL_SC0_PA3_FPOV_WE              = 0x00000042u32,
PH_PERF_SEL_SC0_PA3_FPOP_WE              = 0x00000043u32,
PH_PERF_SEL_SC0_PA3_EOP_WE               = 0x00000044u32,
PH_PERF_SEL_SC0_PA3_DATA_FIFO_EOP_RD     = 0x00000045u32,
PH_PERF_SEL_SC0_PA3_EOPG_WE              = 0x00000046u32,
PH_PERF_SEL_SC0_PA3_DEALLOC_WE           = 0x00000047u32,
PH_PERF_SEL_SC0_PA4_DATA_FIFO_RD         = 0x00000048u32,
PH_PERF_SEL_SC0_PA4_DATA_FIFO_WE         = 0x00000049u32,
PH_PERF_SEL_SC0_PA4_FIFO_EMPTY           = 0x0000004au32,
PH_PERF_SEL_SC0_PA4_FIFO_FULL            = 0x0000004bu32,
PH_PERF_SEL_SC0_PA4_NULL_WE              = 0x0000004cu32,
PH_PERF_SEL_SC0_PA4_EVENT_WE             = 0x0000004du32,
PH_PERF_SEL_SC0_PA4_FPOV_WE              = 0x0000004eu32,
PH_PERF_SEL_SC0_PA4_FPOP_WE              = 0x0000004fu32,
PH_PERF_SEL_SC0_PA4_EOP_WE               = 0x00000050u32,
PH_PERF_SEL_SC0_PA4_DATA_FIFO_EOP_RD     = 0x00000051u32,
PH_PERF_SEL_SC0_PA4_EOPG_WE              = 0x00000052u32,
PH_PERF_SEL_SC0_PA4_DEALLOC_WE           = 0x00000053u32,
PH_PERF_SEL_SC0_PA5_DATA_FIFO_RD         = 0x00000054u32,
PH_PERF_SEL_SC0_PA5_DATA_FIFO_WE         = 0x00000055u32,
PH_PERF_SEL_SC0_PA5_FIFO_EMPTY           = 0x00000056u32,
PH_PERF_SEL_SC0_PA5_FIFO_FULL            = 0x00000057u32,
PH_PERF_SEL_SC0_PA5_NULL_WE              = 0x00000058u32,
PH_PERF_SEL_SC0_PA5_EVENT_WE             = 0x00000059u32,
PH_PERF_SEL_SC0_PA5_FPOV_WE              = 0x0000005au32,
PH_PERF_SEL_SC0_PA5_FPOP_WE              = 0x0000005bu32,
PH_PERF_SEL_SC0_PA5_EOP_WE               = 0x0000005cu32,
PH_PERF_SEL_SC0_PA5_DATA_FIFO_EOP_RD     = 0x0000005du32,
PH_PERF_SEL_SC0_PA5_EOPG_WE              = 0x0000005eu32,
PH_PERF_SEL_SC0_PA5_DEALLOC_WE           = 0x0000005fu32,
PH_PERF_SEL_SC0_PA6_DATA_FIFO_RD         = 0x00000060u32,
PH_PERF_SEL_SC0_PA6_DATA_FIFO_WE         = 0x00000061u32,
PH_PERF_SEL_SC0_PA6_FIFO_EMPTY           = 0x00000062u32,
PH_PERF_SEL_SC0_PA6_FIFO_FULL            = 0x00000063u32,
PH_PERF_SEL_SC0_PA6_NULL_WE              = 0x00000064u32,
PH_PERF_SEL_SC0_PA6_EVENT_WE             = 0x00000065u32,
PH_PERF_SEL_SC0_PA6_FPOV_WE              = 0x00000066u32,
PH_PERF_SEL_SC0_PA6_FPOP_WE              = 0x00000067u32,
PH_PERF_SEL_SC0_PA6_EOP_WE               = 0x00000068u32,
PH_PERF_SEL_SC0_PA6_DATA_FIFO_EOP_RD     = 0x00000069u32,
PH_PERF_SEL_SC0_PA6_EOPG_WE              = 0x0000006au32,
PH_PERF_SEL_SC0_PA6_DEALLOC_WE           = 0x0000006bu32,
PH_PERF_SEL_SC0_PA7_DATA_FIFO_RD         = 0x0000006cu32,
PH_PERF_SEL_SC0_PA7_DATA_FIFO_WE         = 0x0000006du32,
PH_PERF_SEL_SC0_PA7_FIFO_EMPTY           = 0x0000006eu32,
PH_PERF_SEL_SC0_PA7_FIFO_FULL            = 0x0000006fu32,
PH_PERF_SEL_SC0_PA7_NULL_WE              = 0x00000070u32,
PH_PERF_SEL_SC0_PA7_EVENT_WE             = 0x00000071u32,
PH_PERF_SEL_SC0_PA7_FPOV_WE              = 0x00000072u32,
PH_PERF_SEL_SC0_PA7_FPOP_WE              = 0x00000073u32,
PH_PERF_SEL_SC0_PA7_EOP_WE               = 0x00000074u32,
PH_PERF_SEL_SC0_PA7_DATA_FIFO_EOP_RD     = 0x00000075u32,
PH_PERF_SEL_SC0_PA7_EOPG_WE              = 0x00000076u32,
PH_PERF_SEL_SC0_PA7_DEALLOC_WE           = 0x00000077u32,
PH_PERF_SEL_SC1_SRPS_WINDOW_VALID        = 0x00000078u32,
PH_PERF_SEL_SC1_ARB_XFC_ALL_EVENT_OR_PRIM_CYCLES = 0x00000079u32,
PH_PERF_SEL_SC1_ARB_XFC_ONLY_PRIM_CYCLES = 0x0000007au32,
PH_PERF_SEL_SC1_ARB_XFC_ONLY_ONE_INC_PER_PRIM = 0x0000007bu32,
PH_PERF_SEL_SC1_ARB_STALLED_FROM_BELOW   = 0x0000007cu32,
PH_PERF_SEL_SC1_ARB_STARVED_FROM_ABOVE   = 0x0000007du32,
PH_PERF_SEL_SC1_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_NOT_EMPTY = 0x0000007eu32,
PH_PERF_SEL_SC1_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_FULL = 0x0000007fu32,
PH_PERF_SEL_SC1_ARB_BUSY                 = 0x00000080u32,
PH_PERF_SEL_SC1_ARB_PA_BUSY_SOP          = 0x00000081u32,
PH_PERF_SEL_SC1_ARB_EOP_POP_SYNC_POP     = 0x00000082u32,
PH_PERF_SEL_SC1_ARB_EVENT_SYNC_POP       = 0x00000083u32,
PH_PERF_SEL_SC1_PS_ENG_MULTICYCLE_BUBBLE = 0x00000084u32,
PH_PERF_SEL_SC1_EOP_SYNC_WINDOW          = 0x00000085u32,
PH_PERF_SEL_SC1_BUSY_PROCESSING_MULTICYCLE_PRIM = 0x00000086u32,
PH_PERF_SEL_SC1_BUSY_CNT_NOT_ZERO        = 0x00000087u32,
PH_PERF_SEL_SC1_SEND                     = 0x00000088u32,
PH_PERF_SEL_SC1_CREDIT_AT_ZERO_WITH_PENDING_SEND = 0x00000089u32,
PH_PERF_SEL_SC1_CREDIT_AT_MAX            = 0x0000008au32,
PH_PERF_SEL_SC1_CREDIT_AT_MAX_NO_PENDING_SEND = 0x0000008bu32,
PH_PERF_SEL_SC1_GFX_PIPE0_TO_1_TRANSITION = 0x0000008cu32,
PH_PERF_SEL_SC1_GFX_PIPE1_TO_0_TRANSITION = 0x0000008du32,
PH_PERF_SEL_SC1_GFX_PIPE_EOP_PRIM_PROVOKED_TRANSITION = 0x0000008eu32,
PH_PERF_SEL_SC1_GFX_PIPE_EVENT_PROVOKED_TRANSITION = 0x0000008fu32,
PH_PERF_SEL_SC1_PA0_DATA_FIFO_RD         = 0x00000090u32,
PH_PERF_SEL_SC1_PA0_DATA_FIFO_WE         = 0x00000091u32,
PH_PERF_SEL_SC1_PA0_FIFO_EMPTY           = 0x00000092u32,
PH_PERF_SEL_SC1_PA0_FIFO_FULL            = 0x00000093u32,
PH_PERF_SEL_SC1_PA0_NULL_WE              = 0x00000094u32,
PH_PERF_SEL_SC1_PA0_EVENT_WE             = 0x00000095u32,
PH_PERF_SEL_SC1_PA0_FPOV_WE              = 0x00000096u32,
PH_PERF_SEL_SC1_PA0_FPOP_WE              = 0x00000097u32,
PH_PERF_SEL_SC1_PA0_EOP_WE               = 0x00000098u32,
PH_PERF_SEL_SC1_PA0_DATA_FIFO_EOP_RD     = 0x00000099u32,
PH_PERF_SEL_SC1_PA0_EOPG_WE              = 0x0000009au32,
PH_PERF_SEL_SC1_PA0_DEALLOC_WE           = 0x0000009bu32,
PH_PERF_SEL_SC1_PA1_DATA_FIFO_RD         = 0x0000009cu32,
PH_PERF_SEL_SC1_PA1_DATA_FIFO_WE         = 0x0000009du32,
PH_PERF_SEL_SC1_PA1_FIFO_EMPTY           = 0x0000009eu32,
PH_PERF_SEL_SC1_PA1_FIFO_FULL            = 0x0000009fu32,
PH_PERF_SEL_SC1_PA1_NULL_WE              = 0x000000a0u32,
PH_PERF_SEL_SC1_PA1_EVENT_WE             = 0x000000a1u32,
PH_PERF_SEL_SC1_PA1_FPOV_WE              = 0x000000a2u32,
PH_PERF_SEL_SC1_PA1_FPOP_WE              = 0x000000a3u32,
PH_PERF_SEL_SC1_PA1_EOP_WE               = 0x000000a4u32,
PH_PERF_SEL_SC1_PA1_DATA_FIFO_EOP_RD     = 0x000000a5u32,
PH_PERF_SEL_SC1_PA1_EOPG_WE              = 0x000000a6u32,
PH_PERF_SEL_SC1_PA1_DEALLOC_WE           = 0x000000a7u32,
PH_PERF_SEL_SC1_PA2_DATA_FIFO_RD         = 0x000000a8u32,
PH_PERF_SEL_SC1_PA2_DATA_FIFO_WE         = 0x000000a9u32,
PH_PERF_SEL_SC1_PA2_FIFO_EMPTY           = 0x000000aau32,
PH_PERF_SEL_SC1_PA2_FIFO_FULL            = 0x000000abu32,
PH_PERF_SEL_SC1_PA2_NULL_WE              = 0x000000acu32,
PH_PERF_SEL_SC1_PA2_EVENT_WE             = 0x000000adu32,
PH_PERF_SEL_SC1_PA2_FPOV_WE              = 0x000000aeu32,
PH_PERF_SEL_SC1_PA2_FPOP_WE              = 0x000000afu32,
PH_PERF_SEL_SC1_PA2_EOP_WE               = 0x000000b0u32,
PH_PERF_SEL_SC1_PA2_DATA_FIFO_EOP_RD     = 0x000000b1u32,
PH_PERF_SEL_SC1_PA2_EOPG_WE              = 0x000000b2u32,
PH_PERF_SEL_SC1_PA2_DEALLOC_WE           = 0x000000b3u32,
PH_PERF_SEL_SC1_PA3_DATA_FIFO_RD         = 0x000000b4u32,
PH_PERF_SEL_SC1_PA3_DATA_FIFO_WE         = 0x000000b5u32,
PH_PERF_SEL_SC1_PA3_FIFO_EMPTY           = 0x000000b6u32,
PH_PERF_SEL_SC1_PA3_FIFO_FULL            = 0x000000b7u32,
PH_PERF_SEL_SC1_PA3_NULL_WE              = 0x000000b8u32,
PH_PERF_SEL_SC1_PA3_EVENT_WE             = 0x000000b9u32,
PH_PERF_SEL_SC1_PA3_FPOV_WE              = 0x000000bau32,
PH_PERF_SEL_SC1_PA3_FPOP_WE              = 0x000000bbu32,
PH_PERF_SEL_SC1_PA3_EOP_WE               = 0x000000bcu32,
PH_PERF_SEL_SC1_PA3_DATA_FIFO_EOP_RD     = 0x000000bdu32,
PH_PERF_SEL_SC1_PA3_EOPG_WE              = 0x000000beu32,
PH_PERF_SEL_SC1_PA3_DEALLOC_WE           = 0x000000bfu32,
PH_PERF_SEL_SC1_PA4_DATA_FIFO_RD         = 0x000000c0u32,
PH_PERF_SEL_SC1_PA4_DATA_FIFO_WE         = 0x000000c1u32,
PH_PERF_SEL_SC1_PA4_FIFO_EMPTY           = 0x000000c2u32,
PH_PERF_SEL_SC1_PA4_FIFO_FULL            = 0x000000c3u32,
PH_PERF_SEL_SC1_PA4_NULL_WE              = 0x000000c4u32,
PH_PERF_SEL_SC1_PA4_EVENT_WE             = 0x000000c5u32,
PH_PERF_SEL_SC1_PA4_FPOV_WE              = 0x000000c6u32,
PH_PERF_SEL_SC1_PA4_FPOP_WE              = 0x000000c7u32,
PH_PERF_SEL_SC1_PA4_EOP_WE               = 0x000000c8u32,
PH_PERF_SEL_SC1_PA4_DATA_FIFO_EOP_RD     = 0x000000c9u32,
PH_PERF_SEL_SC1_PA4_EOPG_WE              = 0x000000cau32,
PH_PERF_SEL_SC1_PA4_DEALLOC_WE           = 0x000000cbu32,
PH_PERF_SEL_SC1_PA5_DATA_FIFO_RD         = 0x000000ccu32,
PH_PERF_SEL_SC1_PA5_DATA_FIFO_WE         = 0x000000cdu32,
PH_PERF_SEL_SC1_PA5_FIFO_EMPTY           = 0x000000ceu32,
PH_PERF_SEL_SC1_PA5_FIFO_FULL            = 0x000000cfu32,
PH_PERF_SEL_SC1_PA5_NULL_WE              = 0x000000d0u32,
PH_PERF_SEL_SC1_PA5_EVENT_WE             = 0x000000d1u32,
PH_PERF_SEL_SC1_PA5_FPOV_WE              = 0x000000d2u32,
PH_PERF_SEL_SC1_PA5_FPOP_WE              = 0x000000d3u32,
PH_PERF_SEL_SC1_PA5_EOP_WE               = 0x000000d4u32,
PH_PERF_SEL_SC1_PA5_DATA_FIFO_EOP_RD     = 0x000000d5u32,
PH_PERF_SEL_SC1_PA5_EOPG_WE              = 0x000000d6u32,
PH_PERF_SEL_SC1_PA5_DEALLOC_WE           = 0x000000d7u32,
PH_PERF_SEL_SC1_PA6_DATA_FIFO_RD         = 0x000000d8u32,
PH_PERF_SEL_SC1_PA6_DATA_FIFO_WE         = 0x000000d9u32,
PH_PERF_SEL_SC1_PA6_FIFO_EMPTY           = 0x000000dau32,
PH_PERF_SEL_SC1_PA6_FIFO_FULL            = 0x000000dbu32,
PH_PERF_SEL_SC1_PA6_NULL_WE              = 0x000000dcu32,
PH_PERF_SEL_SC1_PA6_EVENT_WE             = 0x000000ddu32,
PH_PERF_SEL_SC1_PA6_FPOV_WE              = 0x000000deu32,
PH_PERF_SEL_SC1_PA6_FPOP_WE              = 0x000000dfu32,
PH_PERF_SEL_SC1_PA6_EOP_WE               = 0x000000e0u32,
PH_PERF_SEL_SC1_PA6_DATA_FIFO_EOP_RD     = 0x000000e1u32,
PH_PERF_SEL_SC1_PA6_EOPG_WE              = 0x000000e2u32,
PH_PERF_SEL_SC1_PA6_DEALLOC_WE           = 0x000000e3u32,
PH_PERF_SEL_SC1_PA7_DATA_FIFO_RD         = 0x000000e4u32,
PH_PERF_SEL_SC1_PA7_DATA_FIFO_WE         = 0x000000e5u32,
PH_PERF_SEL_SC1_PA7_FIFO_EMPTY           = 0x000000e6u32,
PH_PERF_SEL_SC1_PA7_FIFO_FULL            = 0x000000e7u32,
PH_PERF_SEL_SC1_PA7_NULL_WE              = 0x000000e8u32,
PH_PERF_SEL_SC1_PA7_EVENT_WE             = 0x000000e9u32,
PH_PERF_SEL_SC1_PA7_FPOV_WE              = 0x000000eau32,
PH_PERF_SEL_SC1_PA7_FPOP_WE              = 0x000000ebu32,
PH_PERF_SEL_SC1_PA7_EOP_WE               = 0x000000ecu32,
PH_PERF_SEL_SC1_PA7_DATA_FIFO_EOP_RD     = 0x000000edu32,
PH_PERF_SEL_SC1_PA7_EOPG_WE              = 0x000000eeu32,
PH_PERF_SEL_SC1_PA7_DEALLOC_WE           = 0x000000efu32,
PH_PERF_SEL_SC2_SRPS_WINDOW_VALID        = 0x000000f0u32,
PH_PERF_SEL_SC2_ARB_XFC_ALL_EVENT_OR_PRIM_CYCLES = 0x000000f1u32,
PH_PERF_SEL_SC2_ARB_XFC_ONLY_PRIM_CYCLES = 0x000000f2u32,
PH_PERF_SEL_SC2_ARB_XFC_ONLY_ONE_INC_PER_PRIM = 0x000000f3u32,
PH_PERF_SEL_SC2_ARB_STALLED_FROM_BELOW   = 0x000000f4u32,
PH_PERF_SEL_SC2_ARB_STARVED_FROM_ABOVE   = 0x000000f5u32,
PH_PERF_SEL_SC2_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_NOT_EMPTY = 0x000000f6u32,
PH_PERF_SEL_SC2_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_FULL = 0x000000f7u32,
PH_PERF_SEL_SC2_ARB_BUSY                 = 0x000000f8u32,
PH_PERF_SEL_SC2_ARB_PA_BUSY_SOP          = 0x000000f9u32,
PH_PERF_SEL_SC2_ARB_EOP_POP_SYNC_POP     = 0x000000fau32,
PH_PERF_SEL_SC2_ARB_EVENT_SYNC_POP       = 0x000000fbu32,
PH_PERF_SEL_SC2_PS_ENG_MULTICYCLE_BUBBLE = 0x000000fcu32,
PH_PERF_SEL_SC2_EOP_SYNC_WINDOW          = 0x000000fdu32,
PH_PERF_SEL_SC2_BUSY_PROCESSING_MULTICYCLE_PRIM = 0x000000feu32,
PH_PERF_SEL_SC2_BUSY_CNT_NOT_ZERO        = 0x000000ffu32,
PH_PERF_SEL_SC2_SEND                     = 0x00000100u32,
PH_PERF_SEL_SC2_CREDIT_AT_ZERO_WITH_PENDING_SEND = 0x00000101u32,
PH_PERF_SEL_SC2_CREDIT_AT_MAX            = 0x00000102u32,
PH_PERF_SEL_SC2_CREDIT_AT_MAX_NO_PENDING_SEND = 0x00000103u32,
PH_PERF_SEL_SC2_GFX_PIPE0_TO_1_TRANSITION = 0x00000104u32,
PH_PERF_SEL_SC2_GFX_PIPE1_TO_0_TRANSITION = 0x00000105u32,
PH_PERF_SEL_SC2_GFX_PIPE_EOP_PRIM_PROVOKED_TRANSITION = 0x00000106u32,
PH_PERF_SEL_SC2_GFX_PIPE_EVENT_PROVOKED_TRANSITION = 0x00000107u32,
PH_PERF_SEL_SC2_PA0_DATA_FIFO_RD         = 0x00000108u32,
PH_PERF_SEL_SC2_PA0_DATA_FIFO_WE         = 0x00000109u32,
PH_PERF_SEL_SC2_PA0_FIFO_EMPTY           = 0x0000010au32,
PH_PERF_SEL_SC2_PA0_FIFO_FULL            = 0x0000010bu32,
PH_PERF_SEL_SC2_PA0_NULL_WE              = 0x0000010cu32,
PH_PERF_SEL_SC2_PA0_EVENT_WE             = 0x0000010du32,
PH_PERF_SEL_SC2_PA0_FPOV_WE              = 0x0000010eu32,
PH_PERF_SEL_SC2_PA0_FPOP_WE              = 0x0000010fu32,
PH_PERF_SEL_SC2_PA0_EOP_WE               = 0x00000110u32,
PH_PERF_SEL_SC2_PA0_DATA_FIFO_EOP_RD     = 0x00000111u32,
PH_PERF_SEL_SC2_PA0_EOPG_WE              = 0x00000112u32,
PH_PERF_SEL_SC2_PA0_DEALLOC_WE           = 0x00000113u32,
PH_PERF_SEL_SC2_PA1_DATA_FIFO_RD         = 0x00000114u32,
PH_PERF_SEL_SC2_PA1_DATA_FIFO_WE         = 0x00000115u32,
PH_PERF_SEL_SC2_PA1_FIFO_EMPTY           = 0x00000116u32,
PH_PERF_SEL_SC2_PA1_FIFO_FULL            = 0x00000117u32,
PH_PERF_SEL_SC2_PA1_NULL_WE              = 0x00000118u32,
PH_PERF_SEL_SC2_PA1_EVENT_WE             = 0x00000119u32,
PH_PERF_SEL_SC2_PA1_FPOV_WE              = 0x0000011au32,
PH_PERF_SEL_SC2_PA1_FPOP_WE              = 0x0000011bu32,
PH_PERF_SEL_SC2_PA1_EOP_WE               = 0x0000011cu32,
PH_PERF_SEL_SC2_PA1_DATA_FIFO_EOP_RD     = 0x0000011du32,
PH_PERF_SEL_SC2_PA1_EOPG_WE              = 0x0000011eu32,
PH_PERF_SEL_SC2_PA1_DEALLOC_WE           = 0x0000011fu32,
PH_PERF_SEL_SC2_PA2_DATA_FIFO_RD         = 0x00000120u32,
PH_PERF_SEL_SC2_PA2_DATA_FIFO_WE         = 0x00000121u32,
PH_PERF_SEL_SC2_PA2_FIFO_EMPTY           = 0x00000122u32,
PH_PERF_SEL_SC2_PA2_FIFO_FULL            = 0x00000123u32,
PH_PERF_SEL_SC2_PA2_NULL_WE              = 0x00000124u32,
PH_PERF_SEL_SC2_PA2_EVENT_WE             = 0x00000125u32,
PH_PERF_SEL_SC2_PA2_FPOV_WE              = 0x00000126u32,
PH_PERF_SEL_SC2_PA2_FPOP_WE              = 0x00000127u32,
PH_PERF_SEL_SC2_PA2_EOP_WE               = 0x00000128u32,
PH_PERF_SEL_SC2_PA2_DATA_FIFO_EOP_RD     = 0x00000129u32,
PH_PERF_SEL_SC2_PA2_EOPG_WE              = 0x0000012au32,
PH_PERF_SEL_SC2_PA2_DEALLOC_WE           = 0x0000012bu32,
PH_PERF_SEL_SC2_PA3_DATA_FIFO_RD         = 0x0000012cu32,
PH_PERF_SEL_SC2_PA3_DATA_FIFO_WE         = 0x0000012du32,
PH_PERF_SEL_SC2_PA3_FIFO_EMPTY           = 0x0000012eu32,
PH_PERF_SEL_SC2_PA3_FIFO_FULL            = 0x0000012fu32,
PH_PERF_SEL_SC2_PA3_NULL_WE              = 0x00000130u32,
PH_PERF_SEL_SC2_PA3_EVENT_WE             = 0x00000131u32,
PH_PERF_SEL_SC2_PA3_FPOV_WE              = 0x00000132u32,
PH_PERF_SEL_SC2_PA3_FPOP_WE              = 0x00000133u32,
PH_PERF_SEL_SC2_PA3_EOP_WE               = 0x00000134u32,
PH_PERF_SEL_SC2_PA3_DATA_FIFO_EOP_RD     = 0x00000135u32,
PH_PERF_SEL_SC2_PA3_EOPG_WE              = 0x00000136u32,
PH_PERF_SEL_SC2_PA3_DEALLOC_WE           = 0x00000137u32,
PH_PERF_SEL_SC2_PA4_DATA_FIFO_RD         = 0x00000138u32,
PH_PERF_SEL_SC2_PA4_DATA_FIFO_WE         = 0x00000139u32,
PH_PERF_SEL_SC2_PA4_FIFO_EMPTY           = 0x0000013au32,
PH_PERF_SEL_SC2_PA4_FIFO_FULL            = 0x0000013bu32,
PH_PERF_SEL_SC2_PA4_NULL_WE              = 0x0000013cu32,
PH_PERF_SEL_SC2_PA4_EVENT_WE             = 0x0000013du32,
PH_PERF_SEL_SC2_PA4_FPOV_WE              = 0x0000013eu32,
PH_PERF_SEL_SC2_PA4_FPOP_WE              = 0x0000013fu32,
PH_PERF_SEL_SC2_PA4_EOP_WE               = 0x00000140u32,
PH_PERF_SEL_SC2_PA4_DATA_FIFO_EOP_RD     = 0x00000141u32,
PH_PERF_SEL_SC2_PA4_EOPG_WE              = 0x00000142u32,
PH_PERF_SEL_SC2_PA4_DEALLOC_WE           = 0x00000143u32,
PH_PERF_SEL_SC2_PA5_DATA_FIFO_RD         = 0x00000144u32,
PH_PERF_SEL_SC2_PA5_DATA_FIFO_WE         = 0x00000145u32,
PH_PERF_SEL_SC2_PA5_FIFO_EMPTY           = 0x00000146u32,
PH_PERF_SEL_SC2_PA5_FIFO_FULL            = 0x00000147u32,
PH_PERF_SEL_SC2_PA5_NULL_WE              = 0x00000148u32,
PH_PERF_SEL_SC2_PA5_EVENT_WE             = 0x00000149u32,
PH_PERF_SEL_SC2_PA5_FPOV_WE              = 0x0000014au32,
PH_PERF_SEL_SC2_PA5_FPOP_WE              = 0x0000014bu32,
PH_PERF_SEL_SC2_PA5_EOP_WE               = 0x0000014cu32,
PH_PERF_SEL_SC2_PA5_DATA_FIFO_EOP_RD     = 0x0000014du32,
PH_PERF_SEL_SC2_PA5_EOPG_WE              = 0x0000014eu32,
PH_PERF_SEL_SC2_PA5_DEALLOC_WE           = 0x0000014fu32,
PH_PERF_SEL_SC2_PA6_DATA_FIFO_RD         = 0x00000150u32,
PH_PERF_SEL_SC2_PA6_DATA_FIFO_WE         = 0x00000151u32,
PH_PERF_SEL_SC2_PA6_FIFO_EMPTY           = 0x00000152u32,
PH_PERF_SEL_SC2_PA6_FIFO_FULL            = 0x00000153u32,
PH_PERF_SEL_SC2_PA6_NULL_WE              = 0x00000154u32,
PH_PERF_SEL_SC2_PA6_EVENT_WE             = 0x00000155u32,
PH_PERF_SEL_SC2_PA6_FPOV_WE              = 0x00000156u32,
PH_PERF_SEL_SC2_PA6_FPOP_WE              = 0x00000157u32,
PH_PERF_SEL_SC2_PA6_EOP_WE               = 0x00000158u32,
PH_PERF_SEL_SC2_PA6_DATA_FIFO_EOP_RD     = 0x00000159u32,
PH_PERF_SEL_SC2_PA6_EOPG_WE              = 0x0000015au32,
PH_PERF_SEL_SC2_PA6_DEALLOC_WE           = 0x0000015bu32,
PH_PERF_SEL_SC2_PA7_DATA_FIFO_RD         = 0x0000015cu32,
PH_PERF_SEL_SC2_PA7_DATA_FIFO_WE         = 0x0000015du32,
PH_PERF_SEL_SC2_PA7_FIFO_EMPTY           = 0x0000015eu32,
PH_PERF_SEL_SC2_PA7_FIFO_FULL            = 0x0000015fu32,
PH_PERF_SEL_SC2_PA7_NULL_WE              = 0x00000160u32,
PH_PERF_SEL_SC2_PA7_EVENT_WE             = 0x00000161u32,
PH_PERF_SEL_SC2_PA7_FPOV_WE              = 0x00000162u32,
PH_PERF_SEL_SC2_PA7_FPOP_WE              = 0x00000163u32,
PH_PERF_SEL_SC2_PA7_EOP_WE               = 0x00000164u32,
PH_PERF_SEL_SC2_PA7_DATA_FIFO_EOP_RD     = 0x00000165u32,
PH_PERF_SEL_SC2_PA7_EOPG_WE              = 0x00000166u32,
PH_PERF_SEL_SC2_PA7_DEALLOC_WE           = 0x00000167u32,
PH_PERF_SEL_SC3_SRPS_WINDOW_VALID        = 0x00000168u32,
PH_PERF_SEL_SC3_ARB_XFC_ALL_EVENT_OR_PRIM_CYCLES = 0x00000169u32,
PH_PERF_SEL_SC3_ARB_XFC_ONLY_PRIM_CYCLES = 0x0000016au32,
PH_PERF_SEL_SC3_ARB_XFC_ONLY_ONE_INC_PER_PRIM = 0x0000016bu32,
PH_PERF_SEL_SC3_ARB_STALLED_FROM_BELOW   = 0x0000016cu32,
PH_PERF_SEL_SC3_ARB_STARVED_FROM_ABOVE   = 0x0000016du32,
PH_PERF_SEL_SC3_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_NOT_EMPTY = 0x0000016eu32,
PH_PERF_SEL_SC3_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_FULL = 0x0000016fu32,
PH_PERF_SEL_SC3_ARB_BUSY                 = 0x00000170u32,
PH_PERF_SEL_SC3_ARB_PA_BUSY_SOP          = 0x00000171u32,
PH_PERF_SEL_SC3_ARB_EOP_POP_SYNC_POP     = 0x00000172u32,
PH_PERF_SEL_SC3_ARB_EVENT_SYNC_POP       = 0x00000173u32,
PH_PERF_SEL_SC3_PS_ENG_MULTICYCLE_BUBBLE = 0x00000174u32,
PH_PERF_SEL_SC3_EOP_SYNC_WINDOW          = 0x00000175u32,
PH_PERF_SEL_SC3_BUSY_PROCESSING_MULTICYCLE_PRIM = 0x00000176u32,
PH_PERF_SEL_SC3_BUSY_CNT_NOT_ZERO        = 0x00000177u32,
PH_PERF_SEL_SC3_SEND                     = 0x00000178u32,
PH_PERF_SEL_SC3_CREDIT_AT_ZERO_WITH_PENDING_SEND = 0x00000179u32,
PH_PERF_SEL_SC3_CREDIT_AT_MAX            = 0x0000017au32,
PH_PERF_SEL_SC3_CREDIT_AT_MAX_NO_PENDING_SEND = 0x0000017bu32,
PH_PERF_SEL_SC3_GFX_PIPE0_TO_1_TRANSITION = 0x0000017cu32,
PH_PERF_SEL_SC3_GFX_PIPE1_TO_0_TRANSITION = 0x0000017du32,
PH_PERF_SEL_SC3_GFX_PIPE_EOP_PRIM_PROVOKED_TRANSITION = 0x0000017eu32,
PH_PERF_SEL_SC3_GFX_PIPE_EVENT_PROVOKED_TRANSITION = 0x0000017fu32,
PH_PERF_SEL_SC3_PA0_DATA_FIFO_RD         = 0x00000180u32,
PH_PERF_SEL_SC3_PA0_DATA_FIFO_WE         = 0x00000181u32,
PH_PERF_SEL_SC3_PA0_FIFO_EMPTY           = 0x00000182u32,
PH_PERF_SEL_SC3_PA0_FIFO_FULL            = 0x00000183u32,
PH_PERF_SEL_SC3_PA0_NULL_WE              = 0x00000184u32,
PH_PERF_SEL_SC3_PA0_EVENT_WE             = 0x00000185u32,
PH_PERF_SEL_SC3_PA0_FPOV_WE              = 0x00000186u32,
PH_PERF_SEL_SC3_PA0_FPOP_WE              = 0x00000187u32,
PH_PERF_SEL_SC3_PA0_EOP_WE               = 0x00000188u32,
PH_PERF_SEL_SC3_PA0_DATA_FIFO_EOP_RD     = 0x00000189u32,
PH_PERF_SEL_SC3_PA0_EOPG_WE              = 0x0000018au32,
PH_PERF_SEL_SC3_PA0_DEALLOC_WE           = 0x0000018bu32,
PH_PERF_SEL_SC3_PA1_DATA_FIFO_RD         = 0x0000018cu32,
PH_PERF_SEL_SC3_PA1_DATA_FIFO_WE         = 0x0000018du32,
PH_PERF_SEL_SC3_PA1_FIFO_EMPTY           = 0x0000018eu32,
PH_PERF_SEL_SC3_PA1_FIFO_FULL            = 0x0000018fu32,
PH_PERF_SEL_SC3_PA1_NULL_WE              = 0x00000190u32,
PH_PERF_SEL_SC3_PA1_EVENT_WE             = 0x00000191u32,
PH_PERF_SEL_SC3_PA1_FPOV_WE              = 0x00000192u32,
PH_PERF_SEL_SC3_PA1_FPOP_WE              = 0x00000193u32,
PH_PERF_SEL_SC3_PA1_EOP_WE               = 0x00000194u32,
PH_PERF_SEL_SC3_PA1_DATA_FIFO_EOP_RD     = 0x00000195u32,
PH_PERF_SEL_SC3_PA1_EOPG_WE              = 0x00000196u32,
PH_PERF_SEL_SC3_PA1_DEALLOC_WE           = 0x00000197u32,
PH_PERF_SEL_SC3_PA2_DATA_FIFO_RD         = 0x00000198u32,
PH_PERF_SEL_SC3_PA2_DATA_FIFO_WE         = 0x00000199u32,
PH_PERF_SEL_SC3_PA2_FIFO_EMPTY           = 0x0000019au32,
PH_PERF_SEL_SC3_PA2_FIFO_FULL            = 0x0000019bu32,
PH_PERF_SEL_SC3_PA2_NULL_WE              = 0x0000019cu32,
PH_PERF_SEL_SC3_PA2_EVENT_WE             = 0x0000019du32,
PH_PERF_SEL_SC3_PA2_FPOV_WE              = 0x0000019eu32,
PH_PERF_SEL_SC3_PA2_FPOP_WE              = 0x0000019fu32,
PH_PERF_SEL_SC3_PA2_EOP_WE               = 0x000001a0u32,
PH_PERF_SEL_SC3_PA2_DATA_FIFO_EOP_RD     = 0x000001a1u32,
PH_PERF_SEL_SC3_PA2_EOPG_WE              = 0x000001a2u32,
PH_PERF_SEL_SC3_PA2_DEALLOC_WE           = 0x000001a3u32,
PH_PERF_SEL_SC3_PA3_DATA_FIFO_RD         = 0x000001a4u32,
PH_PERF_SEL_SC3_PA3_DATA_FIFO_WE         = 0x000001a5u32,
PH_PERF_SEL_SC3_PA3_FIFO_EMPTY           = 0x000001a6u32,
PH_PERF_SEL_SC3_PA3_FIFO_FULL            = 0x000001a7u32,
PH_PERF_SEL_SC3_PA3_NULL_WE              = 0x000001a8u32,
PH_PERF_SEL_SC3_PA3_EVENT_WE             = 0x000001a9u32,
PH_PERF_SEL_SC3_PA3_FPOV_WE              = 0x000001aau32,
PH_PERF_SEL_SC3_PA3_FPOP_WE              = 0x000001abu32,
PH_PERF_SEL_SC3_PA3_EOP_WE               = 0x000001acu32,
PH_PERF_SEL_SC3_PA3_DATA_FIFO_EOP_RD     = 0x000001adu32,
PH_PERF_SEL_SC3_PA3_EOPG_WE              = 0x000001aeu32,
PH_PERF_SEL_SC3_PA3_DEALLOC_WE           = 0x000001afu32,
PH_PERF_SEL_SC3_PA4_DATA_FIFO_RD         = 0x000001b0u32,
PH_PERF_SEL_SC3_PA4_DATA_FIFO_WE         = 0x000001b1u32,
PH_PERF_SEL_SC3_PA4_FIFO_EMPTY           = 0x000001b2u32,
PH_PERF_SEL_SC3_PA4_FIFO_FULL            = 0x000001b3u32,
PH_PERF_SEL_SC3_PA4_NULL_WE              = 0x000001b4u32,
PH_PERF_SEL_SC3_PA4_EVENT_WE             = 0x000001b5u32,
PH_PERF_SEL_SC3_PA4_FPOV_WE              = 0x000001b6u32,
PH_PERF_SEL_SC3_PA4_FPOP_WE              = 0x000001b7u32,
PH_PERF_SEL_SC3_PA4_EOP_WE               = 0x000001b8u32,
PH_PERF_SEL_SC3_PA4_DATA_FIFO_EOP_RD     = 0x000001b9u32,
PH_PERF_SEL_SC3_PA4_EOPG_WE              = 0x000001bau32,
PH_PERF_SEL_SC3_PA4_DEALLOC_WE           = 0x000001bbu32,
PH_PERF_SEL_SC3_PA5_DATA_FIFO_RD         = 0x000001bcu32,
PH_PERF_SEL_SC3_PA5_DATA_FIFO_WE         = 0x000001bdu32,
PH_PERF_SEL_SC3_PA5_FIFO_EMPTY           = 0x000001beu32,
PH_PERF_SEL_SC3_PA5_FIFO_FULL            = 0x000001bfu32,
PH_PERF_SEL_SC3_PA5_NULL_WE              = 0x000001c0u32,
PH_PERF_SEL_SC3_PA5_EVENT_WE             = 0x000001c1u32,
PH_PERF_SEL_SC3_PA5_FPOV_WE              = 0x000001c2u32,
PH_PERF_SEL_SC3_PA5_FPOP_WE              = 0x000001c3u32,
PH_PERF_SEL_SC3_PA5_EOP_WE               = 0x000001c4u32,
PH_PERF_SEL_SC3_PA5_DATA_FIFO_EOP_RD     = 0x000001c5u32,
PH_PERF_SEL_SC3_PA5_EOPG_WE              = 0x000001c6u32,
PH_PERF_SEL_SC3_PA5_DEALLOC_WE           = 0x000001c7u32,
PH_PERF_SEL_SC3_PA6_DATA_FIFO_RD         = 0x000001c8u32,
PH_PERF_SEL_SC3_PA6_DATA_FIFO_WE         = 0x000001c9u32,
PH_PERF_SEL_SC3_PA6_FIFO_EMPTY           = 0x000001cau32,
PH_PERF_SEL_SC3_PA6_FIFO_FULL            = 0x000001cbu32,
PH_PERF_SEL_SC3_PA6_NULL_WE              = 0x000001ccu32,
PH_PERF_SEL_SC3_PA6_EVENT_WE             = 0x000001cdu32,
PH_PERF_SEL_SC3_PA6_FPOV_WE              = 0x000001ceu32,
PH_PERF_SEL_SC3_PA6_FPOP_WE              = 0x000001cfu32,
PH_PERF_SEL_SC3_PA6_EOP_WE               = 0x000001d0u32,
PH_PERF_SEL_SC3_PA6_DATA_FIFO_EOP_RD     = 0x000001d1u32,
PH_PERF_SEL_SC3_PA6_EOPG_WE              = 0x000001d2u32,
PH_PERF_SEL_SC3_PA6_DEALLOC_WE           = 0x000001d3u32,
PH_PERF_SEL_SC3_PA7_DATA_FIFO_RD         = 0x000001d4u32,
PH_PERF_SEL_SC3_PA7_DATA_FIFO_WE         = 0x000001d5u32,
PH_PERF_SEL_SC3_PA7_FIFO_EMPTY           = 0x000001d6u32,
PH_PERF_SEL_SC3_PA7_FIFO_FULL            = 0x000001d7u32,
PH_PERF_SEL_SC3_PA7_NULL_WE              = 0x000001d8u32,
PH_PERF_SEL_SC3_PA7_EVENT_WE             = 0x000001d9u32,
PH_PERF_SEL_SC3_PA7_FPOV_WE              = 0x000001dau32,
PH_PERF_SEL_SC3_PA7_FPOP_WE              = 0x000001dbu32,
PH_PERF_SEL_SC3_PA7_EOP_WE               = 0x000001dcu32,
PH_PERF_SEL_SC3_PA7_DATA_FIFO_EOP_RD     = 0x000001ddu32,
PH_PERF_SEL_SC3_PA7_EOPG_WE              = 0x000001deu32,
PH_PERF_SEL_SC3_PA7_DEALLOC_WE           = 0x000001dfu32,
PH_PERF_SEL_SC4_SRPS_WINDOW_VALID        = 0x000001e0u32,
PH_PERF_SEL_SC4_ARB_XFC_ALL_EVENT_OR_PRIM_CYCLES = 0x000001e1u32,
PH_PERF_SEL_SC4_ARB_XFC_ONLY_PRIM_CYCLES = 0x000001e2u32,
PH_PERF_SEL_SC4_ARB_XFC_ONLY_ONE_INC_PER_PRIM = 0x000001e3u32,
PH_PERF_SEL_SC4_ARB_STALLED_FROM_BELOW   = 0x000001e4u32,
PH_PERF_SEL_SC4_ARB_STARVED_FROM_ABOVE   = 0x000001e5u32,
PH_PERF_SEL_SC4_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_NOT_EMPTY = 0x000001e6u32,
PH_PERF_SEL_SC4_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_FULL = 0x000001e7u32,
PH_PERF_SEL_SC4_ARB_BUSY                 = 0x000001e8u32,
PH_PERF_SEL_SC4_ARB_PA_BUSY_SOP          = 0x000001e9u32,
PH_PERF_SEL_SC4_ARB_EOP_POP_SYNC_POP     = 0x000001eau32,
PH_PERF_SEL_SC4_ARB_EVENT_SYNC_POP       = 0x000001ebu32,
PH_PERF_SEL_SC4_PS_ENG_MULTICYCLE_BUBBLE = 0x000001ecu32,
PH_PERF_SEL_SC4_EOP_SYNC_WINDOW          = 0x000001edu32,
PH_PERF_SEL_SC4_BUSY_PROCESSING_MULTICYCLE_PRIM = 0x000001eeu32,
PH_PERF_SEL_SC4_BUSY_CNT_NOT_ZERO        = 0x000001efu32,
PH_PERF_SEL_SC4_SEND                     = 0x000001f0u32,
PH_PERF_SEL_SC4_CREDIT_AT_ZERO_WITH_PENDING_SEND = 0x000001f1u32,
PH_PERF_SEL_SC4_CREDIT_AT_MAX            = 0x000001f2u32,
PH_PERF_SEL_SC4_CREDIT_AT_MAX_NO_PENDING_SEND = 0x000001f3u32,
PH_PERF_SEL_SC4_GFX_PIPE0_TO_1_TRANSITION = 0x000001f4u32,
PH_PERF_SEL_SC4_GFX_PIPE1_TO_0_TRANSITION = 0x000001f5u32,
PH_PERF_SEL_SC4_GFX_PIPE_EOP_PRIM_PROVOKED_TRANSITION = 0x000001f6u32,
PH_PERF_SEL_SC4_GFX_PIPE_EVENT_PROVOKED_TRANSITION = 0x000001f7u32,
PH_PERF_SEL_SC4_PA0_DATA_FIFO_RD         = 0x000001f8u32,
PH_PERF_SEL_SC4_PA0_DATA_FIFO_WE         = 0x000001f9u32,
PH_PERF_SEL_SC4_PA0_FIFO_EMPTY           = 0x000001fau32,
PH_PERF_SEL_SC4_PA0_FIFO_FULL            = 0x000001fbu32,
PH_PERF_SEL_SC4_PA0_NULL_WE              = 0x000001fcu32,
PH_PERF_SEL_SC4_PA0_EVENT_WE             = 0x000001fdu32,
PH_PERF_SEL_SC4_PA0_FPOV_WE              = 0x000001feu32,
PH_PERF_SEL_SC4_PA0_FPOP_WE              = 0x000001ffu32,
PH_PERF_SEL_SC4_PA0_EOP_WE               = 0x00000200u32,
PH_PERF_SEL_SC4_PA0_DATA_FIFO_EOP_RD     = 0x00000201u32,
PH_PERF_SEL_SC4_PA0_EOPG_WE              = 0x00000202u32,
PH_PERF_SEL_SC4_PA0_DEALLOC_WE           = 0x00000203u32,
PH_PERF_SEL_SC4_PA1_DATA_FIFO_RD         = 0x00000204u32,
PH_PERF_SEL_SC4_PA1_DATA_FIFO_WE         = 0x00000205u32,
PH_PERF_SEL_SC4_PA1_FIFO_EMPTY           = 0x00000206u32,
PH_PERF_SEL_SC4_PA1_FIFO_FULL            = 0x00000207u32,
PH_PERF_SEL_SC4_PA1_NULL_WE              = 0x00000208u32,
PH_PERF_SEL_SC4_PA1_EVENT_WE             = 0x00000209u32,
PH_PERF_SEL_SC4_PA1_FPOV_WE              = 0x0000020au32,
PH_PERF_SEL_SC4_PA1_FPOP_WE              = 0x0000020bu32,
PH_PERF_SEL_SC4_PA1_EOP_WE               = 0x0000020cu32,
PH_PERF_SEL_SC4_PA1_DATA_FIFO_EOP_RD     = 0x0000020du32,
PH_PERF_SEL_SC4_PA1_EOPG_WE              = 0x0000020eu32,
PH_PERF_SEL_SC4_PA1_DEALLOC_WE           = 0x0000020fu32,
PH_PERF_SEL_SC4_PA2_DATA_FIFO_RD         = 0x00000210u32,
PH_PERF_SEL_SC4_PA2_DATA_FIFO_WE         = 0x00000211u32,
PH_PERF_SEL_SC4_PA2_FIFO_EMPTY           = 0x00000212u32,
PH_PERF_SEL_SC4_PA2_FIFO_FULL            = 0x00000213u32,
PH_PERF_SEL_SC4_PA2_NULL_WE              = 0x00000214u32,
PH_PERF_SEL_SC4_PA2_EVENT_WE             = 0x00000215u32,
PH_PERF_SEL_SC4_PA2_FPOV_WE              = 0x00000216u32,
PH_PERF_SEL_SC4_PA2_FPOP_WE              = 0x00000217u32,
PH_PERF_SEL_SC4_PA2_EOP_WE               = 0x00000218u32,
PH_PERF_SEL_SC4_PA2_DATA_FIFO_EOP_RD     = 0x00000219u32,
PH_PERF_SEL_SC4_PA2_EOPG_WE              = 0x0000021au32,
PH_PERF_SEL_SC4_PA2_DEALLOC_WE           = 0x0000021bu32,
PH_PERF_SEL_SC4_PA3_DATA_FIFO_RD         = 0x0000021cu32,
PH_PERF_SEL_SC4_PA3_DATA_FIFO_WE         = 0x0000021du32,
PH_PERF_SEL_SC4_PA3_FIFO_EMPTY           = 0x0000021eu32,
PH_PERF_SEL_SC4_PA3_FIFO_FULL            = 0x0000021fu32,
PH_PERF_SEL_SC4_PA3_NULL_WE              = 0x00000220u32,
PH_PERF_SEL_SC4_PA3_EVENT_WE             = 0x00000221u32,
PH_PERF_SEL_SC4_PA3_FPOV_WE              = 0x00000222u32,
PH_PERF_SEL_SC4_PA3_FPOP_WE              = 0x00000223u32,
PH_PERF_SEL_SC4_PA3_EOP_WE               = 0x00000224u32,
PH_PERF_SEL_SC4_PA3_DATA_FIFO_EOP_RD     = 0x00000225u32,
PH_PERF_SEL_SC4_PA3_EOPG_WE              = 0x00000226u32,
PH_PERF_SEL_SC4_PA3_DEALLOC_WE           = 0x00000227u32,
PH_PERF_SEL_SC4_PA4_DATA_FIFO_RD         = 0x00000228u32,
PH_PERF_SEL_SC4_PA4_DATA_FIFO_WE         = 0x00000229u32,
PH_PERF_SEL_SC4_PA4_FIFO_EMPTY           = 0x0000022au32,
PH_PERF_SEL_SC4_PA4_FIFO_FULL            = 0x0000022bu32,
PH_PERF_SEL_SC4_PA4_NULL_WE              = 0x0000022cu32,
PH_PERF_SEL_SC4_PA4_EVENT_WE             = 0x0000022du32,
PH_PERF_SEL_SC4_PA4_FPOV_WE              = 0x0000022eu32,
PH_PERF_SEL_SC4_PA4_FPOP_WE              = 0x0000022fu32,
PH_PERF_SEL_SC4_PA4_EOP_WE               = 0x00000230u32,
PH_PERF_SEL_SC4_PA4_DATA_FIFO_EOP_RD     = 0x00000231u32,
PH_PERF_SEL_SC4_PA4_EOPG_WE              = 0x00000232u32,
PH_PERF_SEL_SC4_PA4_DEALLOC_WE           = 0x00000233u32,
PH_PERF_SEL_SC4_PA5_DATA_FIFO_RD         = 0x00000234u32,
PH_PERF_SEL_SC4_PA5_DATA_FIFO_WE         = 0x00000235u32,
PH_PERF_SEL_SC4_PA5_FIFO_EMPTY           = 0x00000236u32,
PH_PERF_SEL_SC4_PA5_FIFO_FULL            = 0x00000237u32,
PH_PERF_SEL_SC4_PA5_NULL_WE              = 0x00000238u32,
PH_PERF_SEL_SC4_PA5_EVENT_WE             = 0x00000239u32,
PH_PERF_SEL_SC4_PA5_FPOV_WE              = 0x0000023au32,
PH_PERF_SEL_SC4_PA5_FPOP_WE              = 0x0000023bu32,
PH_PERF_SEL_SC4_PA5_EOP_WE               = 0x0000023cu32,
PH_PERF_SEL_SC4_PA5_DATA_FIFO_EOP_RD     = 0x0000023du32,
PH_PERF_SEL_SC4_PA5_EOPG_WE              = 0x0000023eu32,
PH_PERF_SEL_SC4_PA5_DEALLOC_WE           = 0x0000023fu32,
PH_PERF_SEL_SC4_PA6_DATA_FIFO_RD         = 0x00000240u32,
PH_PERF_SEL_SC4_PA6_DATA_FIFO_WE         = 0x00000241u32,
PH_PERF_SEL_SC4_PA6_FIFO_EMPTY           = 0x00000242u32,
PH_PERF_SEL_SC4_PA6_FIFO_FULL            = 0x00000243u32,
PH_PERF_SEL_SC4_PA6_NULL_WE              = 0x00000244u32,
PH_PERF_SEL_SC4_PA6_EVENT_WE             = 0x00000245u32,
PH_PERF_SEL_SC4_PA6_FPOV_WE              = 0x00000246u32,
PH_PERF_SEL_SC4_PA6_FPOP_WE              = 0x00000247u32,
PH_PERF_SEL_SC4_PA6_EOP_WE               = 0x00000248u32,
PH_PERF_SEL_SC4_PA6_DATA_FIFO_EOP_RD     = 0x00000249u32,
PH_PERF_SEL_SC4_PA6_EOPG_WE              = 0x0000024au32,
PH_PERF_SEL_SC4_PA6_DEALLOC_WE           = 0x0000024bu32,
PH_PERF_SEL_SC4_PA7_DATA_FIFO_RD         = 0x0000024cu32,
PH_PERF_SEL_SC4_PA7_DATA_FIFO_WE         = 0x0000024du32,
PH_PERF_SEL_SC4_PA7_FIFO_EMPTY           = 0x0000024eu32,
PH_PERF_SEL_SC4_PA7_FIFO_FULL            = 0x0000024fu32,
PH_PERF_SEL_SC4_PA7_NULL_WE              = 0x00000250u32,
PH_PERF_SEL_SC4_PA7_EVENT_WE             = 0x00000251u32,
PH_PERF_SEL_SC4_PA7_FPOV_WE              = 0x00000252u32,
PH_PERF_SEL_SC4_PA7_FPOP_WE              = 0x00000253u32,
PH_PERF_SEL_SC4_PA7_EOP_WE               = 0x00000254u32,
PH_PERF_SEL_SC4_PA7_DATA_FIFO_EOP_RD     = 0x00000255u32,
PH_PERF_SEL_SC4_PA7_EOPG_WE              = 0x00000256u32,
PH_PERF_SEL_SC4_PA7_DEALLOC_WE           = 0x00000257u32,
PH_PERF_SEL_SC5_SRPS_WINDOW_VALID        = 0x00000258u32,
PH_PERF_SEL_SC5_ARB_XFC_ALL_EVENT_OR_PRIM_CYCLES = 0x00000259u32,
PH_PERF_SEL_SC5_ARB_XFC_ONLY_PRIM_CYCLES = 0x0000025au32,
PH_PERF_SEL_SC5_ARB_XFC_ONLY_ONE_INC_PER_PRIM = 0x0000025bu32,
PH_PERF_SEL_SC5_ARB_STALLED_FROM_BELOW   = 0x0000025cu32,
PH_PERF_SEL_SC5_ARB_STARVED_FROM_ABOVE   = 0x0000025du32,
PH_PERF_SEL_SC5_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_NOT_EMPTY = 0x0000025eu32,
PH_PERF_SEL_SC5_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_FULL = 0x0000025fu32,
PH_PERF_SEL_SC5_ARB_BUSY                 = 0x00000260u32,
PH_PERF_SEL_SC5_ARB_PA_BUSY_SOP          = 0x00000261u32,
PH_PERF_SEL_SC5_ARB_EOP_POP_SYNC_POP     = 0x00000262u32,
PH_PERF_SEL_SC5_ARB_EVENT_SYNC_POP       = 0x00000263u32,
PH_PERF_SEL_SC5_PS_ENG_MULTICYCLE_BUBBLE = 0x00000264u32,
PH_PERF_SEL_SC5_EOP_SYNC_WINDOW          = 0x00000265u32,
PH_PERF_SEL_SC5_BUSY_PROCESSING_MULTICYCLE_PRIM = 0x00000266u32,
PH_PERF_SEL_SC5_BUSY_CNT_NOT_ZERO        = 0x00000267u32,
PH_PERF_SEL_SC5_SEND                     = 0x00000268u32,
PH_PERF_SEL_SC5_CREDIT_AT_ZERO_WITH_PENDING_SEND = 0x00000269u32,
PH_PERF_SEL_SC5_CREDIT_AT_MAX            = 0x0000026au32,
PH_PERF_SEL_SC5_CREDIT_AT_MAX_NO_PENDING_SEND = 0x0000026bu32,
PH_PERF_SEL_SC5_GFX_PIPE0_TO_1_TRANSITION = 0x0000026cu32,
PH_PERF_SEL_SC5_GFX_PIPE1_TO_0_TRANSITION = 0x0000026du32,
PH_PERF_SEL_SC5_GFX_PIPE_EOP_PRIM_PROVOKED_TRANSITION = 0x0000026eu32,
PH_PERF_SEL_SC5_GFX_PIPE_EVENT_PROVOKED_TRANSITION = 0x0000026fu32,
PH_PERF_SEL_SC5_PA0_DATA_FIFO_RD         = 0x00000270u32,
PH_PERF_SEL_SC5_PA0_DATA_FIFO_WE         = 0x00000271u32,
PH_PERF_SEL_SC5_PA0_FIFO_EMPTY           = 0x00000272u32,
PH_PERF_SEL_SC5_PA0_FIFO_FULL            = 0x00000273u32,
PH_PERF_SEL_SC5_PA0_NULL_WE              = 0x00000274u32,
PH_PERF_SEL_SC5_PA0_EVENT_WE             = 0x00000275u32,
PH_PERF_SEL_SC5_PA0_FPOV_WE              = 0x00000276u32,
PH_PERF_SEL_SC5_PA0_FPOP_WE              = 0x00000277u32,
PH_PERF_SEL_SC5_PA0_EOP_WE               = 0x00000278u32,
PH_PERF_SEL_SC5_PA0_DATA_FIFO_EOP_RD     = 0x00000279u32,
PH_PERF_SEL_SC5_PA0_EOPG_WE              = 0x0000027au32,
PH_PERF_SEL_SC5_PA0_DEALLOC_WE           = 0x0000027bu32,
PH_PERF_SEL_SC5_PA1_DATA_FIFO_RD         = 0x0000027cu32,
PH_PERF_SEL_SC5_PA1_DATA_FIFO_WE         = 0x0000027du32,
PH_PERF_SEL_SC5_PA1_FIFO_EMPTY           = 0x0000027eu32,
PH_PERF_SEL_SC5_PA1_FIFO_FULL            = 0x0000027fu32,
PH_PERF_SEL_SC5_PA1_NULL_WE              = 0x00000280u32,
PH_PERF_SEL_SC5_PA1_EVENT_WE             = 0x00000281u32,
PH_PERF_SEL_SC5_PA1_FPOV_WE              = 0x00000282u32,
PH_PERF_SEL_SC5_PA1_FPOP_WE              = 0x00000283u32,
PH_PERF_SEL_SC5_PA1_EOP_WE               = 0x00000284u32,
PH_PERF_SEL_SC5_PA1_DATA_FIFO_EOP_RD     = 0x00000285u32,
PH_PERF_SEL_SC5_PA1_EOPG_WE              = 0x00000286u32,
PH_PERF_SEL_SC5_PA1_DEALLOC_WE           = 0x00000287u32,
PH_PERF_SEL_SC5_PA2_DATA_FIFO_RD         = 0x00000288u32,
PH_PERF_SEL_SC5_PA2_DATA_FIFO_WE         = 0x00000289u32,
PH_PERF_SEL_SC5_PA2_FIFO_EMPTY           = 0x0000028au32,
PH_PERF_SEL_SC5_PA2_FIFO_FULL            = 0x0000028bu32,
PH_PERF_SEL_SC5_PA2_NULL_WE              = 0x0000028cu32,
PH_PERF_SEL_SC5_PA2_EVENT_WE             = 0x0000028du32,
PH_PERF_SEL_SC5_PA2_FPOV_WE              = 0x0000028eu32,
PH_PERF_SEL_SC5_PA2_FPOP_WE              = 0x0000028fu32,
PH_PERF_SEL_SC5_PA2_EOP_WE               = 0x00000290u32,
PH_PERF_SEL_SC5_PA2_DATA_FIFO_EOP_RD     = 0x00000291u32,
PH_PERF_SEL_SC5_PA2_EOPG_WE              = 0x00000292u32,
PH_PERF_SEL_SC5_PA2_DEALLOC_WE           = 0x00000293u32,
PH_PERF_SEL_SC5_PA3_DATA_FIFO_RD         = 0x00000294u32,
PH_PERF_SEL_SC5_PA3_DATA_FIFO_WE         = 0x00000295u32,
PH_PERF_SEL_SC5_PA3_FIFO_EMPTY           = 0x00000296u32,
PH_PERF_SEL_SC5_PA3_FIFO_FULL            = 0x00000297u32,
PH_PERF_SEL_SC5_PA3_NULL_WE              = 0x00000298u32,
PH_PERF_SEL_SC5_PA3_EVENT_WE             = 0x00000299u32,
PH_PERF_SEL_SC5_PA3_FPOV_WE              = 0x0000029au32,
PH_PERF_SEL_SC5_PA3_FPOP_WE              = 0x0000029bu32,
PH_PERF_SEL_SC5_PA3_EOP_WE               = 0x0000029cu32,
PH_PERF_SEL_SC5_PA3_DATA_FIFO_EOP_RD     = 0x0000029du32,
PH_PERF_SEL_SC5_PA3_EOPG_WE              = 0x0000029eu32,
PH_PERF_SEL_SC5_PA3_DEALLOC_WE           = 0x0000029fu32,
PH_PERF_SEL_SC5_PA4_DATA_FIFO_RD         = 0x000002a0u32,
PH_PERF_SEL_SC5_PA4_DATA_FIFO_WE         = 0x000002a1u32,
PH_PERF_SEL_SC5_PA4_FIFO_EMPTY           = 0x000002a2u32,
PH_PERF_SEL_SC5_PA4_FIFO_FULL            = 0x000002a3u32,
PH_PERF_SEL_SC5_PA4_NULL_WE              = 0x000002a4u32,
PH_PERF_SEL_SC5_PA4_EVENT_WE             = 0x000002a5u32,
PH_PERF_SEL_SC5_PA4_FPOV_WE              = 0x000002a6u32,
PH_PERF_SEL_SC5_PA4_FPOP_WE              = 0x000002a7u32,
PH_PERF_SEL_SC5_PA4_EOP_WE               = 0x000002a8u32,
PH_PERF_SEL_SC5_PA4_DATA_FIFO_EOP_RD     = 0x000002a9u32,
PH_PERF_SEL_SC5_PA4_EOPG_WE              = 0x000002aau32,
PH_PERF_SEL_SC5_PA4_DEALLOC_WE           = 0x000002abu32,
PH_PERF_SEL_SC5_PA5_DATA_FIFO_RD         = 0x000002acu32,
PH_PERF_SEL_SC5_PA5_DATA_FIFO_WE         = 0x000002adu32,
PH_PERF_SEL_SC5_PA5_FIFO_EMPTY           = 0x000002aeu32,
PH_PERF_SEL_SC5_PA5_FIFO_FULL            = 0x000002afu32,
PH_PERF_SEL_SC5_PA5_NULL_WE              = 0x000002b0u32,
PH_PERF_SEL_SC5_PA5_EVENT_WE             = 0x000002b1u32,
PH_PERF_SEL_SC5_PA5_FPOV_WE              = 0x000002b2u32,
PH_PERF_SEL_SC5_PA5_FPOP_WE              = 0x000002b3u32,
PH_PERF_SEL_SC5_PA5_EOP_WE               = 0x000002b4u32,
PH_PERF_SEL_SC5_PA5_DATA_FIFO_EOP_RD     = 0x000002b5u32,
PH_PERF_SEL_SC5_PA5_EOPG_WE              = 0x000002b6u32,
PH_PERF_SEL_SC5_PA5_DEALLOC_WE           = 0x000002b7u32,
PH_PERF_SEL_SC5_PA6_DATA_FIFO_RD         = 0x000002b8u32,
PH_PERF_SEL_SC5_PA6_DATA_FIFO_WE         = 0x000002b9u32,
PH_PERF_SEL_SC5_PA6_FIFO_EMPTY           = 0x000002bau32,
PH_PERF_SEL_SC5_PA6_FIFO_FULL            = 0x000002bbu32,
PH_PERF_SEL_SC5_PA6_NULL_WE              = 0x000002bcu32,
PH_PERF_SEL_SC5_PA6_EVENT_WE             = 0x000002bdu32,
PH_PERF_SEL_SC5_PA6_FPOV_WE              = 0x000002beu32,
PH_PERF_SEL_SC5_PA6_FPOP_WE              = 0x000002bfu32,
PH_PERF_SEL_SC5_PA6_EOP_WE               = 0x000002c0u32,
PH_PERF_SEL_SC5_PA6_DATA_FIFO_EOP_RD     = 0x000002c1u32,
PH_PERF_SEL_SC5_PA6_EOPG_WE              = 0x000002c2u32,
PH_PERF_SEL_SC5_PA6_DEALLOC_WE           = 0x000002c3u32,
PH_PERF_SEL_SC5_PA7_DATA_FIFO_RD         = 0x000002c4u32,
PH_PERF_SEL_SC5_PA7_DATA_FIFO_WE         = 0x000002c5u32,
PH_PERF_SEL_SC5_PA7_FIFO_EMPTY           = 0x000002c6u32,
PH_PERF_SEL_SC5_PA7_FIFO_FULL            = 0x000002c7u32,
PH_PERF_SEL_SC5_PA7_NULL_WE              = 0x000002c8u32,
PH_PERF_SEL_SC5_PA7_EVENT_WE             = 0x000002c9u32,
PH_PERF_SEL_SC5_PA7_FPOV_WE              = 0x000002cau32,
PH_PERF_SEL_SC5_PA7_FPOP_WE              = 0x000002cbu32,
PH_PERF_SEL_SC5_PA7_EOP_WE               = 0x000002ccu32,
PH_PERF_SEL_SC5_PA7_DATA_FIFO_EOP_RD     = 0x000002cdu32,
PH_PERF_SEL_SC5_PA7_EOPG_WE              = 0x000002ceu32,
PH_PERF_SEL_SC5_PA7_DEALLOC_WE           = 0x000002cfu32,
PH_PERF_SEL_SC6_SRPS_WINDOW_VALID        = 0x000002d0u32,
PH_PERF_SEL_SC6_ARB_XFC_ALL_EVENT_OR_PRIM_CYCLES = 0x000002d1u32,
PH_PERF_SEL_SC6_ARB_XFC_ONLY_PRIM_CYCLES = 0x000002d2u32,
PH_PERF_SEL_SC6_ARB_XFC_ONLY_ONE_INC_PER_PRIM = 0x000002d3u32,
PH_PERF_SEL_SC6_ARB_STALLED_FROM_BELOW   = 0x000002d4u32,
PH_PERF_SEL_SC6_ARB_STARVED_FROM_ABOVE   = 0x000002d5u32,
PH_PERF_SEL_SC6_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_NOT_EMPTY = 0x000002d6u32,
PH_PERF_SEL_SC6_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_FULL = 0x000002d7u32,
PH_PERF_SEL_SC6_ARB_BUSY                 = 0x000002d8u32,
PH_PERF_SEL_SC6_ARB_PA_BUSY_SOP          = 0x000002d9u32,
PH_PERF_SEL_SC6_ARB_EOP_POP_SYNC_POP     = 0x000002dau32,
PH_PERF_SEL_SC6_ARB_EVENT_SYNC_POP       = 0x000002dbu32,
PH_PERF_SEL_SC6_PS_ENG_MULTICYCLE_BUBBLE = 0x000002dcu32,
PH_PERF_SEL_SC6_EOP_SYNC_WINDOW          = 0x000002ddu32,
PH_PERF_SEL_SC6_BUSY_PROCESSING_MULTICYCLE_PRIM = 0x000002deu32,
PH_PERF_SEL_SC6_BUSY_CNT_NOT_ZERO        = 0x000002dfu32,
PH_PERF_SEL_SC6_SEND                     = 0x000002e0u32,
PH_PERF_SEL_SC6_CREDIT_AT_ZERO_WITH_PENDING_SEND = 0x000002e1u32,
PH_PERF_SEL_SC6_CREDIT_AT_MAX            = 0x000002e2u32,
PH_PERF_SEL_SC6_CREDIT_AT_MAX_NO_PENDING_SEND = 0x000002e3u32,
PH_PERF_SEL_SC6_GFX_PIPE0_TO_1_TRANSITION = 0x000002e4u32,
PH_PERF_SEL_SC6_GFX_PIPE1_TO_0_TRANSITION = 0x000002e5u32,
PH_PERF_SEL_SC6_GFX_PIPE_EOP_PRIM_PROVOKED_TRANSITION = 0x000002e6u32,
PH_PERF_SEL_SC6_GFX_PIPE_EVENT_PROVOKED_TRANSITION = 0x000002e7u32,
PH_PERF_SEL_SC6_PA0_DATA_FIFO_RD         = 0x000002e8u32,
PH_PERF_SEL_SC6_PA0_DATA_FIFO_WE         = 0x000002e9u32,
PH_PERF_SEL_SC6_PA0_FIFO_EMPTY           = 0x000002eau32,
PH_PERF_SEL_SC6_PA0_FIFO_FULL            = 0x000002ebu32,
PH_PERF_SEL_SC6_PA0_NULL_WE              = 0x000002ecu32,
PH_PERF_SEL_SC6_PA0_EVENT_WE             = 0x000002edu32,
PH_PERF_SEL_SC6_PA0_FPOV_WE              = 0x000002eeu32,
PH_PERF_SEL_SC6_PA0_FPOP_WE              = 0x000002efu32,
PH_PERF_SEL_SC6_PA0_EOP_WE               = 0x000002f0u32,
PH_PERF_SEL_SC6_PA0_DATA_FIFO_EOP_RD     = 0x000002f1u32,
PH_PERF_SEL_SC6_PA0_EOPG_WE              = 0x000002f2u32,
PH_PERF_SEL_SC6_PA0_DEALLOC_WE           = 0x000002f3u32,
PH_PERF_SEL_SC6_PA1_DATA_FIFO_RD         = 0x000002f4u32,
PH_PERF_SEL_SC6_PA1_DATA_FIFO_WE         = 0x000002f5u32,
PH_PERF_SEL_SC6_PA1_FIFO_EMPTY           = 0x000002f6u32,
PH_PERF_SEL_SC6_PA1_FIFO_FULL            = 0x000002f7u32,
PH_PERF_SEL_SC6_PA1_NULL_WE              = 0x000002f8u32,
PH_PERF_SEL_SC6_PA1_EVENT_WE             = 0x000002f9u32,
PH_PERF_SEL_SC6_PA1_FPOV_WE              = 0x000002fau32,
PH_PERF_SEL_SC6_PA1_FPOP_WE              = 0x000002fbu32,
PH_PERF_SEL_SC6_PA1_EOP_WE               = 0x000002fcu32,
PH_PERF_SEL_SC6_PA1_DATA_FIFO_EOP_RD     = 0x000002fdu32,
PH_PERF_SEL_SC6_PA1_EOPG_WE              = 0x000002feu32,
PH_PERF_SEL_SC6_PA1_DEALLOC_WE           = 0x000002ffu32,
PH_PERF_SEL_SC6_PA2_DATA_FIFO_RD         = 0x00000300u32,
PH_PERF_SEL_SC6_PA2_DATA_FIFO_WE         = 0x00000301u32,
PH_PERF_SEL_SC6_PA2_FIFO_EMPTY           = 0x00000302u32,
PH_PERF_SEL_SC6_PA2_FIFO_FULL            = 0x00000303u32,
PH_PERF_SEL_SC6_PA2_NULL_WE              = 0x00000304u32,
PH_PERF_SEL_SC6_PA2_EVENT_WE             = 0x00000305u32,
PH_PERF_SEL_SC6_PA2_FPOV_WE              = 0x00000306u32,
PH_PERF_SEL_SC6_PA2_FPOP_WE              = 0x00000307u32,
PH_PERF_SEL_SC6_PA2_EOP_WE               = 0x00000308u32,
PH_PERF_SEL_SC6_PA2_DATA_FIFO_EOP_RD     = 0x00000309u32,
PH_PERF_SEL_SC6_PA2_EOPG_WE              = 0x0000030au32,
PH_PERF_SEL_SC6_PA2_DEALLOC_WE           = 0x0000030bu32,
PH_PERF_SEL_SC6_PA3_DATA_FIFO_RD         = 0x0000030cu32,
PH_PERF_SEL_SC6_PA3_DATA_FIFO_WE         = 0x0000030du32,
PH_PERF_SEL_SC6_PA3_FIFO_EMPTY           = 0x0000030eu32,
PH_PERF_SEL_SC6_PA3_FIFO_FULL            = 0x0000030fu32,
PH_PERF_SEL_SC6_PA3_NULL_WE              = 0x00000310u32,
PH_PERF_SEL_SC6_PA3_EVENT_WE             = 0x00000311u32,
PH_PERF_SEL_SC6_PA3_FPOV_WE              = 0x00000312u32,
PH_PERF_SEL_SC6_PA3_FPOP_WE              = 0x00000313u32,
PH_PERF_SEL_SC6_PA3_EOP_WE               = 0x00000314u32,
PH_PERF_SEL_SC6_PA3_DATA_FIFO_EOP_RD     = 0x00000315u32,
PH_PERF_SEL_SC6_PA3_EOPG_WE              = 0x00000316u32,
PH_PERF_SEL_SC6_PA3_DEALLOC_WE           = 0x00000317u32,
PH_PERF_SEL_SC6_PA4_DATA_FIFO_RD         = 0x00000318u32,
PH_PERF_SEL_SC6_PA4_DATA_FIFO_WE         = 0x00000319u32,
PH_PERF_SEL_SC6_PA4_FIFO_EMPTY           = 0x0000031au32,
PH_PERF_SEL_SC6_PA4_FIFO_FULL            = 0x0000031bu32,
PH_PERF_SEL_SC6_PA4_NULL_WE              = 0x0000031cu32,
PH_PERF_SEL_SC6_PA4_EVENT_WE             = 0x0000031du32,
PH_PERF_SEL_SC6_PA4_FPOV_WE              = 0x0000031eu32,
PH_PERF_SEL_SC6_PA4_FPOP_WE              = 0x0000031fu32,
PH_PERF_SEL_SC6_PA4_EOP_WE               = 0x00000320u32,
PH_PERF_SEL_SC6_PA4_DATA_FIFO_EOP_RD     = 0x00000321u32,
PH_PERF_SEL_SC6_PA4_EOPG_WE              = 0x00000322u32,
PH_PERF_SEL_SC6_PA4_DEALLOC_WE           = 0x00000323u32,
PH_PERF_SEL_SC6_PA5_DATA_FIFO_RD         = 0x00000324u32,
PH_PERF_SEL_SC6_PA5_DATA_FIFO_WE         = 0x00000325u32,
PH_PERF_SEL_SC6_PA5_FIFO_EMPTY           = 0x00000326u32,
PH_PERF_SEL_SC6_PA5_FIFO_FULL            = 0x00000327u32,
PH_PERF_SEL_SC6_PA5_NULL_WE              = 0x00000328u32,
PH_PERF_SEL_SC6_PA5_EVENT_WE             = 0x00000329u32,
PH_PERF_SEL_SC6_PA5_FPOV_WE              = 0x0000032au32,
PH_PERF_SEL_SC6_PA5_FPOP_WE              = 0x0000032bu32,
PH_PERF_SEL_SC6_PA5_EOP_WE               = 0x0000032cu32,
PH_PERF_SEL_SC6_PA5_DATA_FIFO_EOP_RD     = 0x0000032du32,
PH_PERF_SEL_SC6_PA5_EOPG_WE              = 0x0000032eu32,
PH_PERF_SEL_SC6_PA5_DEALLOC_WE           = 0x0000032fu32,
PH_PERF_SEL_SC6_PA6_DATA_FIFO_RD         = 0x00000330u32,
PH_PERF_SEL_SC6_PA6_DATA_FIFO_WE         = 0x00000331u32,
PH_PERF_SEL_SC6_PA6_FIFO_EMPTY           = 0x00000332u32,
PH_PERF_SEL_SC6_PA6_FIFO_FULL            = 0x00000333u32,
PH_PERF_SEL_SC6_PA6_NULL_WE              = 0x00000334u32,
PH_PERF_SEL_SC6_PA6_EVENT_WE             = 0x00000335u32,
PH_PERF_SEL_SC6_PA6_FPOV_WE              = 0x00000336u32,
PH_PERF_SEL_SC6_PA6_FPOP_WE              = 0x00000337u32,
PH_PERF_SEL_SC6_PA6_EOP_WE               = 0x00000338u32,
PH_PERF_SEL_SC6_PA6_DATA_FIFO_EOP_RD     = 0x00000339u32,
PH_PERF_SEL_SC6_PA6_EOPG_WE              = 0x0000033au32,
PH_PERF_SEL_SC6_PA6_DEALLOC_WE           = 0x0000033bu32,
PH_PERF_SEL_SC6_PA7_DATA_FIFO_RD         = 0x0000033cu32,
PH_PERF_SEL_SC6_PA7_DATA_FIFO_WE         = 0x0000033du32,
PH_PERF_SEL_SC6_PA7_FIFO_EMPTY           = 0x0000033eu32,
PH_PERF_SEL_SC6_PA7_FIFO_FULL            = 0x0000033fu32,
PH_PERF_SEL_SC6_PA7_NULL_WE              = 0x00000340u32,
PH_PERF_SEL_SC6_PA7_EVENT_WE             = 0x00000341u32,
PH_PERF_SEL_SC6_PA7_FPOV_WE              = 0x00000342u32,
PH_PERF_SEL_SC6_PA7_FPOP_WE              = 0x00000343u32,
PH_PERF_SEL_SC6_PA7_EOP_WE               = 0x00000344u32,
PH_PERF_SEL_SC6_PA7_DATA_FIFO_EOP_RD     = 0x00000345u32,
PH_PERF_SEL_SC6_PA7_EOPG_WE              = 0x00000346u32,
PH_PERF_SEL_SC6_PA7_DEALLOC_WE           = 0x00000347u32,
PH_PERF_SEL_SC7_SRPS_WINDOW_VALID        = 0x00000348u32,
PH_PERF_SEL_SC7_ARB_XFC_ALL_EVENT_OR_PRIM_CYCLES = 0x00000349u32,
PH_PERF_SEL_SC7_ARB_XFC_ONLY_PRIM_CYCLES = 0x0000034au32,
PH_PERF_SEL_SC7_ARB_XFC_ONLY_ONE_INC_PER_PRIM = 0x0000034bu32,
PH_PERF_SEL_SC7_ARB_STALLED_FROM_BELOW   = 0x0000034cu32,
PH_PERF_SEL_SC7_ARB_STARVED_FROM_ABOVE   = 0x0000034du32,
PH_PERF_SEL_SC7_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_NOT_EMPTY = 0x0000034eu32,
PH_PERF_SEL_SC7_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_FULL = 0x0000034fu32,
PH_PERF_SEL_SC7_ARB_BUSY                 = 0x00000350u32,
PH_PERF_SEL_SC7_ARB_PA_BUSY_SOP          = 0x00000351u32,
PH_PERF_SEL_SC7_ARB_EOP_POP_SYNC_POP     = 0x00000352u32,
PH_PERF_SEL_SC7_ARB_EVENT_SYNC_POP       = 0x00000353u32,
PH_PERF_SEL_SC7_PS_ENG_MULTICYCLE_BUBBLE = 0x00000354u32,
PH_PERF_SEL_SC7_EOP_SYNC_WINDOW          = 0x00000355u32,
PH_PERF_SEL_SC7_BUSY_PROCESSING_MULTICYCLE_PRIM = 0x00000356u32,
PH_PERF_SEL_SC7_BUSY_CNT_NOT_ZERO        = 0x00000357u32,
PH_PERF_SEL_SC7_SEND                     = 0x00000358u32,
PH_PERF_SEL_SC7_CREDIT_AT_ZERO_WITH_PENDING_SEND = 0x00000359u32,
PH_PERF_SEL_SC7_CREDIT_AT_MAX            = 0x0000035au32,
PH_PERF_SEL_SC7_CREDIT_AT_MAX_NO_PENDING_SEND = 0x0000035bu32,
PH_PERF_SEL_SC7_GFX_PIPE0_TO_1_TRANSITION = 0x0000035cu32,
PH_PERF_SEL_SC7_GFX_PIPE1_TO_0_TRANSITION = 0x0000035du32,
PH_PERF_SEL_SC7_GFX_PIPE_EOP_PRIM_PROVOKED_TRANSITION = 0x0000035eu32,
PH_PERF_SEL_SC7_GFX_PIPE_EVENT_PROVOKED_TRANSITION = 0x0000035fu32,
PH_PERF_SEL_SC7_PA0_DATA_FIFO_RD         = 0x00000360u32,
PH_PERF_SEL_SC7_PA0_DATA_FIFO_WE         = 0x00000361u32,
PH_PERF_SEL_SC7_PA0_FIFO_EMPTY           = 0x00000362u32,
PH_PERF_SEL_SC7_PA0_FIFO_FULL            = 0x00000363u32,
PH_PERF_SEL_SC7_PA0_NULL_WE              = 0x00000364u32,
PH_PERF_SEL_SC7_PA0_EVENT_WE             = 0x00000365u32,
PH_PERF_SEL_SC7_PA0_FPOV_WE              = 0x00000366u32,
PH_PERF_SEL_SC7_PA0_FPOP_WE              = 0x00000367u32,
PH_PERF_SEL_SC7_PA0_EOP_WE               = 0x00000368u32,
PH_PERF_SEL_SC7_PA0_DATA_FIFO_EOP_RD     = 0x00000369u32,
PH_PERF_SEL_SC7_PA0_EOPG_WE              = 0x0000036au32,
PH_PERF_SEL_SC7_PA0_DEALLOC_WE           = 0x0000036bu32,
PH_PERF_SEL_SC7_PA1_DATA_FIFO_RD         = 0x0000036cu32,
PH_PERF_SEL_SC7_PA1_DATA_FIFO_WE         = 0x0000036du32,
PH_PERF_SEL_SC7_PA1_FIFO_EMPTY           = 0x0000036eu32,
PH_PERF_SEL_SC7_PA1_FIFO_FULL            = 0x0000036fu32,
PH_PERF_SEL_SC7_PA1_NULL_WE              = 0x00000370u32,
PH_PERF_SEL_SC7_PA1_EVENT_WE             = 0x00000371u32,
PH_PERF_SEL_SC7_PA1_FPOV_WE              = 0x00000372u32,
PH_PERF_SEL_SC7_PA1_FPOP_WE              = 0x00000373u32,
PH_PERF_SEL_SC7_PA1_EOP_WE               = 0x00000374u32,
PH_PERF_SEL_SC7_PA1_DATA_FIFO_EOP_RD     = 0x00000375u32,
PH_PERF_SEL_SC7_PA1_EOPG_WE              = 0x00000376u32,
PH_PERF_SEL_SC7_PA1_DEALLOC_WE           = 0x00000377u32,
PH_PERF_SEL_SC7_PA2_DATA_FIFO_RD         = 0x00000378u32,
PH_PERF_SEL_SC7_PA2_DATA_FIFO_WE         = 0x00000379u32,
PH_PERF_SEL_SC7_PA2_FIFO_EMPTY           = 0x0000037au32,
PH_PERF_SEL_SC7_PA2_FIFO_FULL            = 0x0000037bu32,
PH_PERF_SEL_SC7_PA2_NULL_WE              = 0x0000037cu32,
PH_PERF_SEL_SC7_PA2_EVENT_WE             = 0x0000037du32,
PH_PERF_SEL_SC7_PA2_FPOV_WE              = 0x0000037eu32,
PH_PERF_SEL_SC7_PA2_FPOP_WE              = 0x0000037fu32,
PH_PERF_SEL_SC7_PA2_EOP_WE               = 0x00000380u32,
PH_PERF_SEL_SC7_PA2_DATA_FIFO_EOP_RD     = 0x00000381u32,
PH_PERF_SEL_SC7_PA2_EOPG_WE              = 0x00000382u32,
PH_PERF_SEL_SC7_PA2_DEALLOC_WE           = 0x00000383u32,
PH_PERF_SEL_SC7_PA3_DATA_FIFO_RD         = 0x00000384u32,
PH_PERF_SEL_SC7_PA3_DATA_FIFO_WE         = 0x00000385u32,
PH_PERF_SEL_SC7_PA3_FIFO_EMPTY           = 0x00000386u32,
PH_PERF_SEL_SC7_PA3_FIFO_FULL            = 0x00000387u32,
PH_PERF_SEL_SC7_PA3_NULL_WE              = 0x00000388u32,
PH_PERF_SEL_SC7_PA3_EVENT_WE             = 0x00000389u32,
PH_PERF_SEL_SC7_PA3_FPOV_WE              = 0x0000038au32,
PH_PERF_SEL_SC7_PA3_FPOP_WE              = 0x0000038bu32,
PH_PERF_SEL_SC7_PA3_EOP_WE               = 0x0000038cu32,
PH_PERF_SEL_SC7_PA3_DATA_FIFO_EOP_RD     = 0x0000038du32,
PH_PERF_SEL_SC7_PA3_EOPG_WE              = 0x0000038eu32,
PH_PERF_SEL_SC7_PA3_DEALLOC_WE           = 0x0000038fu32,
PH_PERF_SEL_SC7_PA4_DATA_FIFO_RD         = 0x00000390u32,
PH_PERF_SEL_SC7_PA4_DATA_FIFO_WE         = 0x00000391u32,
PH_PERF_SEL_SC7_PA4_FIFO_EMPTY           = 0x00000392u32,
PH_PERF_SEL_SC7_PA4_FIFO_FULL            = 0x00000393u32,
PH_PERF_SEL_SC7_PA4_NULL_WE              = 0x00000394u32,
PH_PERF_SEL_SC7_PA4_EVENT_WE             = 0x00000395u32,
PH_PERF_SEL_SC7_PA4_FPOV_WE              = 0x00000396u32,
PH_PERF_SEL_SC7_PA4_FPOP_WE              = 0x00000397u32,
PH_PERF_SEL_SC7_PA4_EOP_WE               = 0x00000398u32,
PH_PERF_SEL_SC7_PA4_DATA_FIFO_EOP_RD     = 0x00000399u32,
PH_PERF_SEL_SC7_PA4_EOPG_WE              = 0x0000039au32,
PH_PERF_SEL_SC7_PA4_DEALLOC_WE           = 0x0000039bu32,
PH_PERF_SEL_SC7_PA5_DATA_FIFO_RD         = 0x0000039cu32,
PH_PERF_SEL_SC7_PA5_DATA_FIFO_WE         = 0x0000039du32,
PH_PERF_SEL_SC7_PA5_FIFO_EMPTY           = 0x0000039eu32,
PH_PERF_SEL_SC7_PA5_FIFO_FULL            = 0x0000039fu32,
PH_PERF_SEL_SC7_PA5_NULL_WE              = 0x000003a0u32,
PH_PERF_SEL_SC7_PA5_EVENT_WE             = 0x000003a1u32,
PH_PERF_SEL_SC7_PA5_FPOV_WE              = 0x000003a2u32,
PH_PERF_SEL_SC7_PA5_FPOP_WE              = 0x000003a3u32,
PH_PERF_SEL_SC7_PA5_EOP_WE               = 0x000003a4u32,
PH_PERF_SEL_SC7_PA5_DATA_FIFO_EOP_RD     = 0x000003a5u32,
PH_PERF_SEL_SC7_PA5_EOPG_WE              = 0x000003a6u32,
PH_PERF_SEL_SC7_PA5_DEALLOC_WE           = 0x000003a7u32,
PH_PERF_SEL_SC7_PA6_DATA_FIFO_RD         = 0x000003a8u32,
PH_PERF_SEL_SC7_PA6_DATA_FIFO_WE         = 0x000003a9u32,
PH_PERF_SEL_SC7_PA6_FIFO_EMPTY           = 0x000003aau32,
PH_PERF_SEL_SC7_PA6_FIFO_FULL            = 0x000003abu32,
PH_PERF_SEL_SC7_PA6_NULL_WE              = 0x000003acu32,
PH_PERF_SEL_SC7_PA6_EVENT_WE             = 0x000003adu32,
PH_PERF_SEL_SC7_PA6_FPOV_WE              = 0x000003aeu32,
PH_PERF_SEL_SC7_PA6_FPOP_WE              = 0x000003afu32,
PH_PERF_SEL_SC7_PA6_EOP_WE               = 0x000003b0u32,
PH_PERF_SEL_SC7_PA6_DATA_FIFO_EOP_RD     = 0x000003b1u32,
PH_PERF_SEL_SC7_PA6_EOPG_WE              = 0x000003b2u32,
PH_PERF_SEL_SC7_PA6_DEALLOC_WE           = 0x000003b3u32,
PH_PERF_SEL_SC7_PA7_DATA_FIFO_RD         = 0x000003b4u32,
PH_PERF_SEL_SC7_PA7_DATA_FIFO_WE         = 0x000003b5u32,
PH_PERF_SEL_SC7_PA7_FIFO_EMPTY           = 0x000003b6u32,
PH_PERF_SEL_SC7_PA7_FIFO_FULL            = 0x000003b7u32,
PH_PERF_SEL_SC7_PA7_NULL_WE              = 0x000003b8u32,
PH_PERF_SEL_SC7_PA7_EVENT_WE             = 0x000003b9u32,
PH_PERF_SEL_SC7_PA7_FPOV_WE              = 0x000003bau32,
PH_PERF_SEL_SC7_PA7_FPOP_WE              = 0x000003bbu32,
PH_PERF_SEL_SC7_PA7_EOP_WE               = 0x000003bcu32,
PH_PERF_SEL_SC7_PA7_DATA_FIFO_EOP_RD     = 0x000003bdu32,
PH_PERF_SEL_SC7_PA7_EOPG_WE              = 0x000003beu32,
PH_PERF_SEL_SC7_PA7_DEALLOC_WE           = 0x000003bfu32,
PH_PERF_SEL_1_SC_ARB_STALLED_FROM_BELOW  = 0x000003c0u32,
PH_PERF_SEL_2_SC_ARB_STALLED_FROM_BELOW  = 0x000003c1u32,
PH_PERF_SEL_3_SC_ARB_STALLED_FROM_BELOW  = 0x000003c2u32,
PH_PERF_SEL_4_SC_ARB_STALLED_FROM_BELOW  = 0x000003c3u32,
PH_PERF_SEL_5_SC_ARB_STALLED_FROM_BELOW  = 0x000003c4u32,
PH_PERF_SEL_6_SC_ARB_STALLED_FROM_BELOW  = 0x000003c5u32,
PH_PERF_SEL_7_SC_ARB_STALLED_FROM_BELOW  = 0x000003c6u32,
PH_PERF_SEL_8_SC_ARB_STALLED_FROM_BELOW  = 0x000003c7u32,
PH_PERF_SEL_1_SC_ARB_STARVED_FROM_ABOVE  = 0x000003c8u32,
PH_PERF_SEL_2_SC_ARB_STARVED_FROM_ABOVE  = 0x000003c9u32,
PH_PERF_SEL_3_SC_ARB_STARVED_FROM_ABOVE  = 0x000003cau32,
PH_PERF_SEL_4_SC_ARB_STARVED_FROM_ABOVE  = 0x000003cbu32,
PH_PERF_SEL_5_SC_ARB_STARVED_FROM_ABOVE  = 0x000003ccu32,
PH_PERF_SEL_6_SC_ARB_STARVED_FROM_ABOVE  = 0x000003cdu32,
PH_PERF_SEL_7_SC_ARB_STARVED_FROM_ABOVE  = 0x000003ceu32,
PH_PERF_SEL_8_SC_ARB_STARVED_FROM_ABOVE  = 0x000003cfu32,
PH_PERF_SEL_1_SC_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_NOT_EMPTY = 0x000003d0u32,
PH_PERF_SEL_2_SC_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_NOT_EMPTY = 0x000003d1u32,
PH_PERF_SEL_3_SC_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_NOT_EMPTY = 0x000003d2u32,
PH_PERF_SEL_4_SC_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_NOT_EMPTY = 0x000003d3u32,
PH_PERF_SEL_5_SC_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_NOT_EMPTY = 0x000003d4u32,
PH_PERF_SEL_6_SC_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_NOT_EMPTY = 0x000003d5u32,
PH_PERF_SEL_7_SC_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_NOT_EMPTY = 0x000003d6u32,
PH_PERF_SEL_8_SC_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_NOT_EMPTY = 0x000003d7u32,
PH_PERF_SEL_1_SC_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_FULL = 0x000003d8u32,
PH_PERF_SEL_2_SC_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_FULL = 0x000003d9u32,
PH_PERF_SEL_3_SC_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_FULL = 0x000003dau32,
PH_PERF_SEL_4_SC_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_FULL = 0x000003dbu32,
PH_PERF_SEL_5_SC_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_FULL = 0x000003dcu32,
PH_PERF_SEL_6_SC_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_FULL = 0x000003ddu32,
PH_PERF_SEL_7_SC_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_FULL = 0x000003deu32,
PH_PERF_SEL_8_SC_ARB_STARVED_FROM_ABOVE_WITH_UNSELECTED_FIFO_FULL = 0x000003dfu32,
PH_PERF_SC0_FIFO_STATUS_0                = 0x000003e0u32,
PH_PERF_SC0_FIFO_STATUS_1                = 0x000003e1u32,
PH_PERF_SC0_FIFO_STATUS_2                = 0x000003e2u32,
PH_PERF_SC0_FIFO_STATUS_3                = 0x000003e3u32,
PH_PERF_SC1_FIFO_STATUS_0                = 0x000003e4u32,
PH_PERF_SC1_FIFO_STATUS_1                = 0x000003e5u32,
PH_PERF_SC1_FIFO_STATUS_2                = 0x000003e6u32,
PH_PERF_SC1_FIFO_STATUS_3                = 0x000003e7u32,
PH_PERF_SC2_FIFO_STATUS_0                = 0x000003e8u32,
PH_PERF_SC2_FIFO_STATUS_1                = 0x000003e9u32,
PH_PERF_SC2_FIFO_STATUS_2                = 0x000003eau32,
PH_PERF_SC2_FIFO_STATUS_3                = 0x000003ebu32,
PH_PERF_SC3_FIFO_STATUS_0                = 0x000003ecu32,
PH_PERF_SC3_FIFO_STATUS_1                = 0x000003edu32,
PH_PERF_SC3_FIFO_STATUS_2                = 0x000003eeu32,
PH_PERF_SC3_FIFO_STATUS_3                = 0x000003efu32,
PH_PERF_SC4_FIFO_STATUS_0                = 0x000003f0u32,
PH_PERF_SC4_FIFO_STATUS_1                = 0x000003f1u32,
PH_PERF_SC4_FIFO_STATUS_2                = 0x000003f2u32,
PH_PERF_SC4_FIFO_STATUS_3                = 0x000003f3u32,
PH_PERF_SC5_FIFO_STATUS_0                = 0x000003f4u32,
PH_PERF_SC5_FIFO_STATUS_1                = 0x000003f5u32,
PH_PERF_SC5_FIFO_STATUS_2                = 0x000003f6u32,
PH_PERF_SC5_FIFO_STATUS_3                = 0x000003f7u32,
PH_PERF_SC6_FIFO_STATUS_0                = 0x000003f8u32,
PH_PERF_SC6_FIFO_STATUS_1                = 0x000003f9u32,
PH_PERF_SC6_FIFO_STATUS_2                = 0x000003fau32,
PH_PERF_SC6_FIFO_STATUS_3                = 0x000003fbu32,
PH_PERF_SC7_FIFO_STATUS_0                = 0x000003fcu32,
PH_PERF_SC7_FIFO_STATUS_1                = 0x000003fdu32,
PH_PERF_SC7_FIFO_STATUS_2                = 0x000003feu32,
PH_PERF_SC7_FIFO_STATUS_3                = 0x000003ffu32,
}

/*
 * PhSPIstatusMode enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PhSPIstatusMode {
PH_SPI_MODE_LARGEST_PA_PH_FIFO_COUNT     = 0x00000000u32,
PH_SPI_MODE_ARBITER_SELECTED_PA_PH_FIFO_COUNT = 0x00000001u32,
PH_SPI_MODE_DISABLED                     = 0x00000002u32,
}

/*******************************************************
 * SC Enums
 *******************************************************/

/*
 * BinEventCntl enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum BinEventCntl {
BINNER_BREAK_BATCH                       = 0x00000000u32,
BINNER_PIPELINE                          = 0x00000001u32,
BINNER_DROP                              = 0x00000002u32,
BINNER_PIPELINE_BREAK                    = 0x00000003u32,
}

/*
 * BinMapMode enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum BinMapMode {
BIN_MAP_MODE_NONE                        = 0x00000000u32,
BIN_MAP_MODE_RTA_INDEX                   = 0x00000001u32,
BIN_MAP_MODE_POPS                        = 0x00000002u32,
}

/*
 * BinSizeExtend enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum BinSizeExtend {
BIN_SIZE_32_PIXELS                       = 0x00000000u32,
BIN_SIZE_64_PIXELS                       = 0x00000001u32,
BIN_SIZE_128_PIXELS                      = 0x00000002u32,
BIN_SIZE_256_PIXELS                      = 0x00000003u32,
BIN_SIZE_512_PIXELS                      = 0x00000004u32,
}

/*
 * BinningMode enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum BinningMode {
BINNING_ALLOWED                          = 0x00000000u32,
FORCE_BINNING_ON                         = 0x00000001u32,
BINNING_ONE_PRIM_PER_BATCH               = 0x00000002u32,
BINNING_DISABLED                         = 0x00000003u32,
}

/*
 * PkrMap enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PkrMap {
RASTER_CONFIG_PKR_MAP_0                  = 0x00000000u32,
RASTER_CONFIG_PKR_MAP_1                  = 0x00000001u32,
RASTER_CONFIG_PKR_MAP_2                  = 0x00000002u32,
RASTER_CONFIG_PKR_MAP_3                  = 0x00000003u32,
}

/*
 * PkrXsel enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PkrXsel {
RASTER_CONFIG_PKR_XSEL_0                 = 0x00000000u32,
RASTER_CONFIG_PKR_XSEL_1                 = 0x00000001u32,
RASTER_CONFIG_PKR_XSEL_2                 = 0x00000002u32,
RASTER_CONFIG_PKR_XSEL_3                 = 0x00000003u32,
}

/*
 * PkrXsel2 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PkrXsel2 {
RASTER_CONFIG_PKR_XSEL2_0                = 0x00000000u32,
RASTER_CONFIG_PKR_XSEL2_1                = 0x00000001u32,
RASTER_CONFIG_PKR_XSEL2_2                = 0x00000002u32,
RASTER_CONFIG_PKR_XSEL2_3                = 0x00000003u32,
}

/*
 * PkrYsel enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PkrYsel {
RASTER_CONFIG_PKR_YSEL_0                 = 0x00000000u32,
RASTER_CONFIG_PKR_YSEL_1                 = 0x00000001u32,
RASTER_CONFIG_PKR_YSEL_2                 = 0x00000002u32,
RASTER_CONFIG_PKR_YSEL_3                 = 0x00000003u32,
}

/*
 * RbMap enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RbMap {
RASTER_CONFIG_RB_MAP_0                   = 0x00000000u32,
RASTER_CONFIG_RB_MAP_1                   = 0x00000001u32,
RASTER_CONFIG_RB_MAP_2                   = 0x00000002u32,
RASTER_CONFIG_RB_MAP_3                   = 0x00000003u32,
}

/*
 * RbXsel enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RbXsel {
RASTER_CONFIG_RB_XSEL_0                  = 0x00000000u32,
RASTER_CONFIG_RB_XSEL_1                  = 0x00000001u32,
}

/*
 * RbXsel2 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RbXsel2 {
RASTER_CONFIG_RB_XSEL2_0                 = 0x00000000u32,
RASTER_CONFIG_RB_XSEL2_1                 = 0x00000001u32,
RASTER_CONFIG_RB_XSEL2_2                 = 0x00000002u32,
RASTER_CONFIG_RB_XSEL2_3                 = 0x00000003u32,
}

/*
 * RbYsel enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RbYsel {
RASTER_CONFIG_RB_YSEL_0                  = 0x00000000u32,
RASTER_CONFIG_RB_YSEL_1                  = 0x00000001u32,
}

/*
 * SC_PERFCNT_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SC_PERFCNT_SEL {
SC_SRPS_WINDOW_VALID                     = 0x00000000u32,
SC_PSSW_WINDOW_VALID                     = 0x00000001u32,
SC_TPQZ_WINDOW_VALID                     = 0x00000002u32,
SC_QZQP_WINDOW_VALID                     = 0x00000003u32,
SC_TRPK_WINDOW_VALID                     = 0x00000004u32,
SC_SRPS_WINDOW_VALID_BUSY                = 0x00000005u32,
SC_PSSW_WINDOW_VALID_BUSY                = 0x00000006u32,
SC_TPQZ_WINDOW_VALID_BUSY                = 0x00000007u32,
SC_QZQP_WINDOW_VALID_BUSY                = 0x00000008u32,
SC_TRPK_WINDOW_VALID_BUSY                = 0x00000009u32,
SC_STARVED_BY_PA                         = 0x0000000au32,
SC_STALLED_BY_PRIMFIFO                   = 0x0000000bu32,
SC_STALLED_BY_DB_TILE                    = 0x0000000cu32,
SC_STARVED_BY_DB_TILE                    = 0x0000000du32,
SC_STALLED_BY_TILEORDERFIFO              = 0x0000000eu32,
SC_STALLED_BY_TILEFIFO                   = 0x0000000fu32,
SC_STALLED_BY_DB_QUAD                    = 0x00000010u32,
SC_STARVED_BY_DB_QUAD                    = 0x00000011u32,
SC_STALLED_BY_QUADFIFO                   = 0x00000012u32,
SC_STALLED_BY_BCI                        = 0x00000013u32,
SC_STALLED_BY_SPI                        = 0x00000014u32,
SC_SCISSOR_DISCARD                       = 0x00000015u32,
SC_BB_DISCARD                            = 0x00000016u32,
SC_SUPERTILE_COUNT                       = 0x00000017u32,
SC_SUPERTILE_PER_PRIM_H0                 = 0x00000018u32,
SC_SUPERTILE_PER_PRIM_H1                 = 0x00000019u32,
SC_SUPERTILE_PER_PRIM_H2                 = 0x0000001au32,
SC_SUPERTILE_PER_PRIM_H3                 = 0x0000001bu32,
SC_SUPERTILE_PER_PRIM_H4                 = 0x0000001cu32,
SC_SUPERTILE_PER_PRIM_H5                 = 0x0000001du32,
SC_SUPERTILE_PER_PRIM_H6                 = 0x0000001eu32,
SC_SUPERTILE_PER_PRIM_H7                 = 0x0000001fu32,
SC_SUPERTILE_PER_PRIM_H8                 = 0x00000020u32,
SC_SUPERTILE_PER_PRIM_H9                 = 0x00000021u32,
SC_SUPERTILE_PER_PRIM_H10                = 0x00000022u32,
SC_SUPERTILE_PER_PRIM_H11                = 0x00000023u32,
SC_SUPERTILE_PER_PRIM_H12                = 0x00000024u32,
SC_SUPERTILE_PER_PRIM_H13                = 0x00000025u32,
SC_SUPERTILE_PER_PRIM_H14                = 0x00000026u32,
SC_SUPERTILE_PER_PRIM_H15                = 0x00000027u32,
SC_SUPERTILE_PER_PRIM_H16                = 0x00000028u32,
SC_TILE_PER_PRIM_H0                      = 0x00000029u32,
SC_TILE_PER_PRIM_H1                      = 0x0000002au32,
SC_TILE_PER_PRIM_H2                      = 0x0000002bu32,
SC_TILE_PER_PRIM_H3                      = 0x0000002cu32,
SC_TILE_PER_PRIM_H4                      = 0x0000002du32,
SC_TILE_PER_PRIM_H5                      = 0x0000002eu32,
SC_TILE_PER_PRIM_H6                      = 0x0000002fu32,
SC_TILE_PER_PRIM_H7                      = 0x00000030u32,
SC_TILE_PER_PRIM_H8                      = 0x00000031u32,
SC_TILE_PER_PRIM_H9                      = 0x00000032u32,
SC_TILE_PER_PRIM_H10                     = 0x00000033u32,
SC_TILE_PER_PRIM_H11                     = 0x00000034u32,
SC_TILE_PER_PRIM_H12                     = 0x00000035u32,
SC_TILE_PER_PRIM_H13                     = 0x00000036u32,
SC_TILE_PER_PRIM_H14                     = 0x00000037u32,
SC_TILE_PER_PRIM_H15                     = 0x00000038u32,
SC_TILE_PER_PRIM_H16                     = 0x00000039u32,
SC_TILE_PER_SUPERTILE_H0                 = 0x0000003au32,
SC_TILE_PER_SUPERTILE_H1                 = 0x0000003bu32,
SC_TILE_PER_SUPERTILE_H2                 = 0x0000003cu32,
SC_TILE_PER_SUPERTILE_H3                 = 0x0000003du32,
SC_TILE_PER_SUPERTILE_H4                 = 0x0000003eu32,
SC_TILE_PER_SUPERTILE_H5                 = 0x0000003fu32,
SC_TILE_PER_SUPERTILE_H6                 = 0x00000040u32,
SC_TILE_PER_SUPERTILE_H7                 = 0x00000041u32,
SC_TILE_PER_SUPERTILE_H8                 = 0x00000042u32,
SC_TILE_PER_SUPERTILE_H9                 = 0x00000043u32,
SC_TILE_PER_SUPERTILE_H10                = 0x00000044u32,
SC_TILE_PER_SUPERTILE_H11                = 0x00000045u32,
SC_TILE_PER_SUPERTILE_H12                = 0x00000046u32,
SC_TILE_PER_SUPERTILE_H13                = 0x00000047u32,
SC_TILE_PER_SUPERTILE_H14                = 0x00000048u32,
SC_TILE_PER_SUPERTILE_H15                = 0x00000049u32,
SC_TILE_PER_SUPERTILE_H16                = 0x0000004au32,
SC_TILE_PICKED_H1                        = 0x0000004bu32,
SC_PERF_SEL_RESERVED_76                  = 0x0000004cu32,
SC_PERF_SEL_RESERVED_77                  = 0x0000004du32,
SC_PERF_SEL_RESERVED_78                  = 0x0000004eu32,
SC_QZ0_TILE_COUNT                        = 0x0000004fu32,
SC_PERF_SEL_RESERVED_80                  = 0x00000050u32,
SC_PERF_SEL_RESERVED_81                  = 0x00000051u32,
SC_PERF_SEL_RESERVED_82                  = 0x00000052u32,
SC_QZ0_TILE_COVERED_COUNT                = 0x00000053u32,
SC_PERF_SEL_RESERVED_84                  = 0x00000054u32,
SC_PERF_SEL_RESERVED_85                  = 0x00000055u32,
SC_PERF_SEL_RESERVED_86                  = 0x00000056u32,
SC_QZ0_TILE_NOT_COVERED_COUNT            = 0x00000057u32,
SC_PERF_SEL_RESERVED_88                  = 0x00000058u32,
SC_PERF_SEL_RESERVED_89                  = 0x00000059u32,
SC_PERF_SEL_RESERVED_90                  = 0x0000005au32,
SC_QZ0_QUAD_PER_TILE_H0                  = 0x0000005bu32,
SC_QZ0_QUAD_PER_TILE_H1                  = 0x0000005cu32,
SC_QZ0_QUAD_PER_TILE_H2                  = 0x0000005du32,
SC_QZ0_QUAD_PER_TILE_H3                  = 0x0000005eu32,
SC_QZ0_QUAD_PER_TILE_H4                  = 0x0000005fu32,
SC_QZ0_QUAD_PER_TILE_H5                  = 0x00000060u32,
SC_QZ0_QUAD_PER_TILE_H6                  = 0x00000061u32,
SC_QZ0_QUAD_PER_TILE_H7                  = 0x00000062u32,
SC_QZ0_QUAD_PER_TILE_H8                  = 0x00000063u32,
SC_QZ0_QUAD_PER_TILE_H9                  = 0x00000064u32,
SC_QZ0_QUAD_PER_TILE_H10                 = 0x00000065u32,
SC_QZ0_QUAD_PER_TILE_H11                 = 0x00000066u32,
SC_QZ0_QUAD_PER_TILE_H12                 = 0x00000067u32,
SC_QZ0_QUAD_PER_TILE_H13                 = 0x00000068u32,
SC_QZ0_QUAD_PER_TILE_H14                 = 0x00000069u32,
SC_QZ0_QUAD_PER_TILE_H15                 = 0x0000006au32,
SC_QZ0_QUAD_PER_TILE_H16                 = 0x0000006bu32,
SC_PERF_SEL_RESERVED_108                 = 0x0000006cu32,
SC_PERF_SEL_RESERVED_109                 = 0x0000006du32,
SC_PERF_SEL_RESERVED_110                 = 0x0000006eu32,
SC_PERF_SEL_RESERVED_111                 = 0x0000006fu32,
SC_PERF_SEL_RESERVED_112                 = 0x00000070u32,
SC_PERF_SEL_RESERVED_113                 = 0x00000071u32,
SC_PERF_SEL_RESERVED_114                 = 0x00000072u32,
SC_PERF_SEL_RESERVED_115                 = 0x00000073u32,
SC_PERF_SEL_RESERVED_116                 = 0x00000074u32,
SC_PERF_SEL_RESERVED_117                 = 0x00000075u32,
SC_PERF_SEL_RESERVED_118                 = 0x00000076u32,
SC_PERF_SEL_RESERVED_119                 = 0x00000077u32,
SC_PERF_SEL_RESERVED_120                 = 0x00000078u32,
SC_PERF_SEL_RESERVED_121                 = 0x00000079u32,
SC_PERF_SEL_RESERVED_122                 = 0x0000007au32,
SC_PERF_SEL_RESERVED_123                 = 0x0000007bu32,
SC_PERF_SEL_RESERVED_124                 = 0x0000007cu32,
SC_PERF_SEL_RESERVED_125                 = 0x0000007du32,
SC_PERF_SEL_RESERVED_126                 = 0x0000007eu32,
SC_PERF_SEL_RESERVED_127                 = 0x0000007fu32,
SC_PERF_SEL_RESERVED_128                 = 0x00000080u32,
SC_PERF_SEL_RESERVED_129                 = 0x00000081u32,
SC_PERF_SEL_RESERVED_130                 = 0x00000082u32,
SC_PERF_SEL_RESERVED_131                 = 0x00000083u32,
SC_PERF_SEL_RESERVED_132                 = 0x00000084u32,
SC_PERF_SEL_RESERVED_133                 = 0x00000085u32,
SC_PERF_SEL_RESERVED_134                 = 0x00000086u32,
SC_PERF_SEL_RESERVED_135                 = 0x00000087u32,
SC_PERF_SEL_RESERVED_136                 = 0x00000088u32,
SC_PERF_SEL_RESERVED_137                 = 0x00000089u32,
SC_PERF_SEL_RESERVED_138                 = 0x0000008au32,
SC_PERF_SEL_RESERVED_139                 = 0x0000008bu32,
SC_PERF_SEL_RESERVED_140                 = 0x0000008cu32,
SC_PERF_SEL_RESERVED_141                 = 0x0000008du32,
SC_PERF_SEL_RESERVED_142                 = 0x0000008eu32,
SC_PERF_SEL_RESERVED_143                 = 0x0000008fu32,
SC_PERF_SEL_RESERVED_144                 = 0x00000090u32,
SC_PERF_SEL_RESERVED_145                 = 0x00000091u32,
SC_PERF_SEL_RESERVED_146                 = 0x00000092u32,
SC_PERF_SEL_RESERVED_147                 = 0x00000093u32,
SC_PERF_SEL_RESERVED_148                 = 0x00000094u32,
SC_PERF_SEL_RESERVED_149                 = 0x00000095u32,
SC_PERF_SEL_RESERVED_150                 = 0x00000096u32,
SC_PERF_SEL_RESERVED_151                 = 0x00000097u32,
SC_PERF_SEL_RESERVED_152                 = 0x00000098u32,
SC_PERF_SEL_RESERVED_153                 = 0x00000099u32,
SC_PERF_SEL_RESERVED_154                 = 0x0000009au32,
SC_PERF_SEL_RESERVED_155                 = 0x0000009bu32,
SC_PERF_SEL_RESERVED_156                 = 0x0000009cu32,
SC_PERF_SEL_RESERVED_157                 = 0x0000009du32,
SC_PERF_SEL_RESERVED_158                 = 0x0000009eu32,
SC_QZ0_QUAD_COUNT                        = 0x0000009fu32,
SC_PERF_SEL_RESERVED_160                 = 0x000000a0u32,
SC_PERF_SEL_RESERVED_161                 = 0x000000a1u32,
SC_PERF_SEL_RESERVED_162                 = 0x000000a2u32,
SC_P0_HIZ_TILE_COUNT                     = 0x000000a3u32,
SC_PERF_SEL_RESERVED_164                 = 0x000000a4u32,
SC_PERF_SEL_RESERVED_165                 = 0x000000a5u32,
SC_PERF_SEL_RESERVED_166                 = 0x000000a6u32,
SC_P0_HIZ_QUAD_PER_TILE_H0               = 0x000000a7u32,
SC_P0_HIZ_QUAD_PER_TILE_H1               = 0x000000a8u32,
SC_P0_HIZ_QUAD_PER_TILE_H2               = 0x000000a9u32,
SC_P0_HIZ_QUAD_PER_TILE_H3               = 0x000000aau32,
SC_P0_HIZ_QUAD_PER_TILE_H4               = 0x000000abu32,
SC_P0_HIZ_QUAD_PER_TILE_H5               = 0x000000acu32,
SC_P0_HIZ_QUAD_PER_TILE_H6               = 0x000000adu32,
SC_P0_HIZ_QUAD_PER_TILE_H7               = 0x000000aeu32,
SC_P0_HIZ_QUAD_PER_TILE_H8               = 0x000000afu32,
SC_P0_HIZ_QUAD_PER_TILE_H9               = 0x000000b0u32,
SC_P0_HIZ_QUAD_PER_TILE_H10              = 0x000000b1u32,
SC_P0_HIZ_QUAD_PER_TILE_H11              = 0x000000b2u32,
SC_P0_HIZ_QUAD_PER_TILE_H12              = 0x000000b3u32,
SC_P0_HIZ_QUAD_PER_TILE_H13              = 0x000000b4u32,
SC_P0_HIZ_QUAD_PER_TILE_H14              = 0x000000b5u32,
SC_P0_HIZ_QUAD_PER_TILE_H15              = 0x000000b6u32,
SC_P0_HIZ_QUAD_PER_TILE_H16              = 0x000000b7u32,
SC_PERF_SEL_RESERVED_184                 = 0x000000b8u32,
SC_PERF_SEL_RESERVED_185                 = 0x000000b9u32,
SC_PERF_SEL_RESERVED_186                 = 0x000000bau32,
SC_PERF_SEL_RESERVED_187                 = 0x000000bbu32,
SC_PERF_SEL_RESERVED_188                 = 0x000000bcu32,
SC_PERF_SEL_RESERVED_189                 = 0x000000bdu32,
SC_PERF_SEL_RESERVED_190                 = 0x000000beu32,
SC_PERF_SEL_RESERVED_191                 = 0x000000bfu32,
SC_PERF_SEL_RESERVED_192                 = 0x000000c0u32,
SC_PERF_SEL_RESERVED_193                 = 0x000000c1u32,
SC_PERF_SEL_RESERVED_194                 = 0x000000c2u32,
SC_PERF_SEL_RESERVED_195                 = 0x000000c3u32,
SC_PERF_SEL_RESERVED_196                 = 0x000000c4u32,
SC_PERF_SEL_RESERVED_197                 = 0x000000c5u32,
SC_PERF_SEL_RESERVED_198                 = 0x000000c6u32,
SC_PERF_SEL_RESERVED_199                 = 0x000000c7u32,
SC_PERF_SEL_RESERVED_200                 = 0x000000c8u32,
SC_PERF_SEL_RESERVED_201                 = 0x000000c9u32,
SC_PERF_SEL_RESERVED_202                 = 0x000000cau32,
SC_PERF_SEL_RESERVED_203                 = 0x000000cbu32,
SC_PERF_SEL_RESERVED_204                 = 0x000000ccu32,
SC_PERF_SEL_RESERVED_205                 = 0x000000cdu32,
SC_PERF_SEL_RESERVED_206                 = 0x000000ceu32,
SC_PERF_SEL_RESERVED_207                 = 0x000000cfu32,
SC_PERF_SEL_RESERVED_208                 = 0x000000d0u32,
SC_PERF_SEL_RESERVED_209                 = 0x000000d1u32,
SC_PERF_SEL_RESERVED_210                 = 0x000000d2u32,
SC_PERF_SEL_RESERVED_211                 = 0x000000d3u32,
SC_PERF_SEL_RESERVED_212                 = 0x000000d4u32,
SC_PERF_SEL_RESERVED_213                 = 0x000000d5u32,
SC_PERF_SEL_RESERVED_214                 = 0x000000d6u32,
SC_PERF_SEL_RESERVED_215                 = 0x000000d7u32,
SC_PERF_SEL_RESERVED_216                 = 0x000000d8u32,
SC_PERF_SEL_RESERVED_217                 = 0x000000d9u32,
SC_PERF_SEL_RESERVED_218                 = 0x000000dau32,
SC_PERF_SEL_RESERVED_219                 = 0x000000dbu32,
SC_PERF_SEL_RESERVED_220                 = 0x000000dcu32,
SC_PERF_SEL_RESERVED_221                 = 0x000000ddu32,
SC_PERF_SEL_RESERVED_222                 = 0x000000deu32,
SC_PERF_SEL_RESERVED_223                 = 0x000000dfu32,
SC_PERF_SEL_RESERVED_224                 = 0x000000e0u32,
SC_PERF_SEL_RESERVED_225                 = 0x000000e1u32,
SC_PERF_SEL_RESERVED_226                 = 0x000000e2u32,
SC_PERF_SEL_RESERVED_227                 = 0x000000e3u32,
SC_PERF_SEL_RESERVED_228                 = 0x000000e4u32,
SC_PERF_SEL_RESERVED_229                 = 0x000000e5u32,
SC_PERF_SEL_RESERVED_230                 = 0x000000e6u32,
SC_PERF_SEL_RESERVED_231                 = 0x000000e7u32,
SC_PERF_SEL_RESERVED_232                 = 0x000000e8u32,
SC_PERF_SEL_RESERVED_233                 = 0x000000e9u32,
SC_PERF_SEL_RESERVED_234                 = 0x000000eau32,
SC_P0_HIZ_QUAD_COUNT                     = 0x000000ebu32,
SC_PERF_SEL_RESERVED_236                 = 0x000000ecu32,
SC_PERF_SEL_RESERVED_237                 = 0x000000edu32,
SC_PERF_SEL_RESERVED_238                 = 0x000000eeu32,
SC_P0_DETAIL_QUAD_COUNT                  = 0x000000efu32,
SC_PERF_SEL_RESERVED_240                 = 0x000000f0u32,
SC_PERF_SEL_RESERVED_241                 = 0x000000f1u32,
SC_PERF_SEL_RESERVED_242                 = 0x000000f2u32,
SC_P0_DETAIL_QUAD_WITH_1_PIX             = 0x000000f3u32,
SC_P0_DETAIL_QUAD_WITH_2_PIX             = 0x000000f4u32,
SC_P0_DETAIL_QUAD_WITH_3_PIX             = 0x000000f5u32,
SC_P0_DETAIL_QUAD_WITH_4_PIX             = 0x000000f6u32,
SC_PERF_SEL_RESERVED_247                 = 0x000000f7u32,
SC_PERF_SEL_RESERVED_248                 = 0x000000f8u32,
SC_PERF_SEL_RESERVED_249                 = 0x000000f9u32,
SC_PERF_SEL_RESERVED_250                 = 0x000000fau32,
SC_PERF_SEL_RESERVED_251                 = 0x000000fbu32,
SC_PERF_SEL_RESERVED_252                 = 0x000000fcu32,
SC_PERF_SEL_RESERVED_253                 = 0x000000fdu32,
SC_PERF_SEL_RESERVED_254                 = 0x000000feu32,
SC_PERF_SEL_RESERVED_255                 = 0x000000ffu32,
SC_PERF_SEL_RESERVED_256                 = 0x00000100u32,
SC_PERF_SEL_RESERVED_257                 = 0x00000101u32,
SC_PERF_SEL_RESERVED_258                 = 0x00000102u32,
SC_EARLYZ_QUAD_COUNT                     = 0x00000103u32,
SC_EARLYZ_QUAD_WITH_1_PIX                = 0x00000104u32,
SC_EARLYZ_QUAD_WITH_2_PIX                = 0x00000105u32,
SC_EARLYZ_QUAD_WITH_3_PIX                = 0x00000106u32,
SC_EARLYZ_QUAD_WITH_4_PIX                = 0x00000107u32,
SC_PKR_QUAD_PER_ROW_H1                   = 0x00000108u32,
SC_PKR_QUAD_PER_ROW_H2                   = 0x00000109u32,
SC_PKR_4X2_QUAD_SPLIT                    = 0x0000010au32,
SC_PKR_4X2_FILL_QUAD                     = 0x0000010bu32,
SC_PKR_END_OF_VECTOR                     = 0x0000010cu32,
SC_PKR_CONTROL_XFER                      = 0x0000010du32,
SC_PKR_DBHANG_FORCE_EOV                  = 0x0000010eu32,
SC_REG_SCLK_BUSY                         = 0x0000010fu32,
SC_GRP0_DYN_SCLK_BUSY                    = 0x00000110u32,
SC_GRP1_DYN_SCLK_BUSY                    = 0x00000111u32,
SC_GRP2_DYN_SCLK_BUSY                    = 0x00000112u32,
SC_GRP3_DYN_SCLK_BUSY                    = 0x00000113u32,
SC_GRP4_DYN_SCLK_BUSY                    = 0x00000114u32,
SC_PA0_SC_DATA_FIFO_RD                   = 0x00000115u32,
SC_PA0_SC_DATA_FIFO_WE                   = 0x00000116u32,
SC_PERF_SEL_RESERVED_279                 = 0x00000117u32,
SC_PERF_SEL_RESERVED_280                 = 0x00000118u32,
SC_PS_ARB_XFC_ALL_EVENT_OR_PRIM_CYCLES   = 0x00000119u32,
SC_PS_ARB_XFC_ONLY_PRIM_CYCLES           = 0x0000011au32,
SC_PS_ARB_XFC_ONLY_ONE_INC_PER_PRIM      = 0x0000011bu32,
SC_PS_ARB_STALLED_FROM_BELOW             = 0x0000011cu32,
SC_PS_ARB_STARVED_FROM_ABOVE             = 0x0000011du32,
SC_PS_ARB_SC_BUSY                        = 0x0000011eu32,
SC_PS_ARB_PA_SC_BUSY                     = 0x0000011fu32,
SC_PERF_SEL_RESERVED_288                 = 0x00000120u32,
SC_PERF_SEL_RESERVED_289                 = 0x00000121u32,
SC_PERF_SEL_RESERVED_290                 = 0x00000122u32,
SC_PERF_SEL_RESERVED_291                 = 0x00000123u32,
SC_PA_SC_DEALLOC_2_0_WE                  = 0x00000124u32,
SC_PERF_SEL_RESERVED_293                 = 0x00000125u32,
SC_PERF_SEL_RESERVED_294                 = 0x00000126u32,
SC_PERF_SEL_RESERVED_295                 = 0x00000127u32,
SC_PERF_SEL_RESERVED_296                 = 0x00000128u32,
SC_PERF_SEL_RESERVED_297                 = 0x00000129u32,
SC_PERF_SEL_RESERVED_298                 = 0x0000012au32,
SC_PERF_SEL_RESERVED_299                 = 0x0000012bu32,
SC_PA0_SC_EOP_WE                         = 0x0000012cu32,
SC_PERF_SEL_RESERVED_301                 = 0x0000012du32,
SC_PA0_SC_EVENT_WE                       = 0x0000012eu32,
SC_PERF_SEL_RESERVED_303                 = 0x0000012fu32,
SC_PERF_SEL_RESERVED_304                 = 0x00000130u32,
SC_PERF_SEL_RESERVED_305                 = 0x00000131u32,
SC_PERF_SEL_RESERVED_306                 = 0x00000132u32,
SC_PERF_SEL_RESERVED_307                 = 0x00000133u32,
SC_PERF_SEL_RESERVED_308                 = 0x00000134u32,
SC_PERF_SEL_RESERVED_309                 = 0x00000135u32,
SC_PERF_SEL_RESERVED_310                 = 0x00000136u32,
SC_PERF_SEL_RESERVED_311                 = 0x00000137u32,
SC_PERF_SEL_RESERVED_312                 = 0x00000138u32,
SC_PERF_SEL_RESERVED_313                 = 0x00000139u32,
SC_PERF_SEL_RESERVED_314                 = 0x0000013au32,
SC_PERF_SEL_RESERVED_315                 = 0x0000013bu32,
SC_PERF_SEL_RESERVED_316                 = 0x0000013cu32,
SC_PERF_SEL_RESERVED_317                 = 0x0000013du32,
SC_PA_SC_FPOV_WE                         = 0x0000013eu32,
SC_PERF_SEL_RESERVED_319                 = 0x0000013fu32,
SC_PERF_SEL_RESERVED_320                 = 0x00000140u32,
SC_PERF_SEL_RESERVED_321                 = 0x00000141u32,
SC_PERF_SEL_RESERVED_322                 = 0x00000142u32,
SC_PERF_SEL_RESERVED_323                 = 0x00000143u32,
SC_PERF_SEL_RESERVED_324                 = 0x00000144u32,
SC_PERF_SEL_RESERVED_325                 = 0x00000145u32,
SC_SPI_DEALLOC_4_0                       = 0x00000146u32,
SC_SPI_DEALLOC_7_5                       = 0x00000147u32,
SC_PERF_SEL_RESERVED_328                 = 0x00000148u32,
SC_PERF_SEL_RESERVED_329                 = 0x00000149u32,
SC_PERF_SEL_RESERVED_330                 = 0x0000014au32,
SC_PERF_SEL_RESERVED_331                 = 0x0000014bu32,
SC_PERF_SEL_RESERVED_332                 = 0x0000014cu32,
SC_PERF_SEL_RESERVED_333                 = 0x0000014du32,
SC_PERF_SEL_RESERVED_334                 = 0x0000014eu32,
SC_PERF_SEL_RESERVED_335                 = 0x0000014fu32,
SC_PERF_SEL_RESERVED_336                 = 0x00000150u32,
SC_PERF_SEL_RESERVED_337                 = 0x00000151u32,
SC_SPI_FPOV_4_0                          = 0x00000152u32,
SC_SPI_FPOV_7_5                          = 0x00000153u32,
SC_PERF_SEL_RESERVED_340                 = 0x00000154u32,
SC_PERF_SEL_RESERVED_341                 = 0x00000155u32,
SC_SPI_EVENT                             = 0x00000156u32,
SC_PS_TS_EVENT_FIFO_PUSH                 = 0x00000157u32,
SC_PS_TS_EVENT_FIFO_POP                  = 0x00000158u32,
SC_PS_CTX_DONE_FIFO_PUSH                 = 0x00000159u32,
SC_PS_CTX_DONE_FIFO_POP                  = 0x0000015au32,
SC_PERF_SEL_RESERVED_347                 = 0x0000015bu32,
SC_PERF_SEL_RESERVED_348                 = 0x0000015cu32,
SC_PA0_SC_NULL_WE                        = 0x0000015du32,
SC_PA0_SC_NULL_DEALLOC_WE                = 0x0000015eu32,
SC_PERF_SEL_RESERVED_351                 = 0x0000015fu32,
SC_PA0_SC_DATA_FIFO_EOP_RD               = 0x00000160u32,
SC_PA0_SC_DEALLOC_2_0_RD                 = 0x00000161u32,
SC_PERF_SEL_RESERVED_354                 = 0x00000162u32,
SC_PERF_SEL_RESERVED_355                 = 0x00000163u32,
SC_PERF_SEL_RESERVED_356                 = 0x00000164u32,
SC_PERF_SEL_RESERVED_357                 = 0x00000165u32,
SC_PERF_SEL_RESERVED_358                 = 0x00000166u32,
SC_PERF_SEL_RESERVED_359                 = 0x00000167u32,
SC_PERF_SEL_RESERVED_360                 = 0x00000168u32,
SC_PERF_SEL_RESERVED_361                 = 0x00000169u32,
SC_PERF_SEL_RESERVED_362                 = 0x0000016au32,
SC_PERF_SEL_RESERVED_363                 = 0x0000016bu32,
SC_PERF_SEL_RESERVED_364                 = 0x0000016cu32,
SC_PERF_SEL_RESERVED_365                 = 0x0000016du32,
SC_PERF_SEL_RESERVED_366                 = 0x0000016eu32,
SC_PERF_SEL_RESERVED_367                 = 0x0000016fu32,
SC_PERF_SEL_RESERVED_368                 = 0x00000170u32,
SC_PERF_SEL_RESERVED_369                 = 0x00000171u32,
SC_PERF_SEL_RESERVED_370                 = 0x00000172u32,
SC_PERF_SEL_RESERVED_371                 = 0x00000173u32,
SC_PERF_SEL_RESERVED_372                 = 0x00000174u32,
SC_PS_PA0_SC_FIFO_EMPTY                  = 0x00000175u32,
SC_PS_PA0_SC_FIFO_FULL                   = 0x00000176u32,
SC_PERF_SEL_RESERVED_375                 = 0x00000177u32,
SC_PERF_SEL_RESERVED_376                 = 0x00000178u32,
SC_PERF_SEL_RESERVED_377                 = 0x00000179u32,
SC_PERF_SEL_RESERVED_378                 = 0x0000017au32,
SC_PERF_SEL_RESERVED_379                 = 0x0000017bu32,
SC_PERF_SEL_RESERVED_380                 = 0x0000017cu32,
SC_PERF_SEL_RESERVED_381                 = 0x0000017du32,
SC_PERF_SEL_RESERVED_382                 = 0x0000017eu32,
SC_PERF_SEL_RESERVED_383                 = 0x0000017fu32,
SC_PERF_SEL_RESERVED_384                 = 0x00000180u32,
SC_PERF_SEL_RESERVED_385                 = 0x00000181u32,
SC_BUSY_CNT_NOT_ZERO                     = 0x00000182u32,
SC_BM_BUSY                               = 0x00000183u32,
SC_BACKEND_BUSY                          = 0x00000184u32,
SC_SCF_SCB_INTERFACE_BUSY                = 0x00000185u32,
SC_SCB_BUSY                              = 0x00000186u32,
SC_STARVED_BY_PA_WITH_UNSELECTED_PA_NOT_EMPTY = 0x00000187u32,
SC_STARVED_BY_PA_WITH_UNSELECTED_PA_FULL = 0x00000188u32,
SC_PBB_BIN_HIST_NUM_PRIMS                = 0x00000189u32,
SC_PBB_BATCH_HIST_NUM_PRIMS              = 0x0000018au32,
SC_PBB_BIN_HIST_NUM_CONTEXTS             = 0x0000018bu32,
SC_PBB_BATCH_HIST_NUM_CONTEXTS           = 0x0000018cu32,
SC_PBB_BIN_HIST_NUM_PERSISTENT_STATES    = 0x0000018du32,
SC_PBB_BATCH_HIST_NUM_PERSISTENT_STATES  = 0x0000018eu32,
SC_PBB_BATCH_HIST_NUM_PS_WAVE_BREAKS     = 0x0000018fu32,
SC_PBB_BATCH_HIST_NUM_TRIV_REJECTED_PRIMS = 0x00000190u32,
SC_PBB_BATCH_HIST_NUM_ROWS_PER_PRIM      = 0x00000191u32,
SC_PBB_BATCH_HIST_NUM_COLUMNS_PER_ROW    = 0x00000192u32,
SC_PBB_BUSY                              = 0x00000193u32,
SC_PBB_BUSY_AND_NO_SENDS                 = 0x00000194u32,
SC_PBB_STALLS_PA_DUE_TO_NO_TILES         = 0x00000195u32,
SC_PBB_NUM_BINS                          = 0x00000196u32,
SC_PBB_END_OF_BIN                        = 0x00000197u32,
SC_PBB_END_OF_BATCH                      = 0x00000198u32,
SC_PBB_PRIMBIN_PROCESSED                 = 0x00000199u32,
SC_PBB_PRIM_ADDED_TO_BATCH               = 0x0000019au32,
SC_PBB_NONBINNED_PRIM                    = 0x0000019bu32,
SC_PBB_TOTAL_REAL_PRIMS_OUT_OF_PBB       = 0x0000019cu32,
SC_PBB_TOTAL_NULL_PRIMS_OUT_OF_PBB       = 0x0000019du32,
SC_PBB_IDLE_CLK_DUE_TO_ROW_TO_COLUMN_TRANSITION = 0x0000019eu32,
SC_PBB_IDLE_CLK_DUE_TO_FALSE_POSITIVE_ON_ROW = 0x0000019fu32,
SC_PBB_IDLE_CLK_DUE_TO_FALSE_POSITIVE_ON_COLUMN = 0x000001a0u32,
SC_PBB_BATCH_BREAK_DUE_TO_PERSISTENT_STATE = 0x000001a1u32,
SC_PBB_BATCH_BREAK_DUE_TO_CONTEXT_STATE  = 0x000001a2u32,
SC_PBB_BATCH_BREAK_DUE_TO_PRIM           = 0x000001a3u32,
SC_PBB_BATCH_BREAK_DUE_TO_PC_STORAGE     = 0x000001a4u32,
SC_PBB_BATCH_BREAK_DUE_TO_EVENT          = 0x000001a5u32,
SC_PBB_BATCH_BREAK_DUE_TO_FPOV_LIMIT     = 0x000001a6u32,
SC_PERF_SEL_RESERVED_423                 = 0x000001a7u32,
SC_PERF_SEL_RESERVED_424                 = 0x000001a8u32,
SC_PERF_SEL_RESERVED_425                 = 0x000001a9u32,
SC_PERF_SEL_RESERVED_426                 = 0x000001aau32,
SC_PERF_SEL_RESERVED_427                 = 0x000001abu32,
SC_PERF_SEL_RESERVED_428                 = 0x000001acu32,
SC_PERF_SEL_RESERVED_429                 = 0x000001adu32,
SC_PERF_SEL_RESERVED_430                 = 0x000001aeu32,
SC_PERF_SEL_RESERVED_431                 = 0x000001afu32,
SC_PERF_SEL_RESERVED_432                 = 0x000001b0u32,
SC_PERF_SEL_RESERVED_433                 = 0x000001b1u32,
SC_PERF_SEL_RESERVED_434                 = 0x000001b2u32,
SC_PERF_SEL_RESERVED_435                 = 0x000001b3u32,
SC_PERF_SEL_RESERVED_436                 = 0x000001b4u32,
SC_GRP5_DYN_SCLK_BUSY                    = 0x000001b5u32,
SC_GRP6_DYN_SCLK_BUSY                    = 0x000001b6u32,
SC_GRP7_DYN_SCLK_BUSY                    = 0x000001b7u32,
SC_GRP8_DYN_SCLK_BUSY                    = 0x000001b8u32,
SC_GRP9_DYN_SCLK_BUSY                    = 0x000001b9u32,
SC_PS_TO_BE_SCLK_GATE_STALL              = 0x000001bau32,
SC_PA_TO_PBB_SCLK_GATE_STALL_STALL       = 0x000001bbu32,
SC_PK_BUSY                               = 0x000001bcu32,
SC_PK_MAX_DEALLOC_FORCE_EOV              = 0x000001bdu32,
SC_PK_DEALLOC_WAVE_BREAK                 = 0x000001beu32,
SC_SPI_SEND                              = 0x000001bfu32,
SC_SPI_CREDIT_AT_ZERO_WITH_PENDING_SEND  = 0x000001c0u32,
SC_SPI_CREDIT_AT_MAX                     = 0x000001c1u32,
SC_SPI_CREDIT_AT_MAX_NO_PENDING_SEND     = 0x000001c2u32,
SC_BCI_SEND                              = 0x000001c3u32,
SC_BCI_CREDIT_AT_ZERO_WITH_PENDING_SEND  = 0x000001c4u32,
SC_BCI_CREDIT_AT_MAX                     = 0x000001c5u32,
SC_BCI_CREDIT_AT_MAX_NO_PENDING_SEND     = 0x000001c6u32,
SC_SPIBC_FULL_FREEZE                     = 0x000001c7u32,
SC_PW_BM_PASS_EMPTY_PRIM                 = 0x000001c8u32,
SC_SUPERTILE_COUNT_EXCLUDE_PASS_EMPTY_PRIM = 0x000001c9u32,
SC_SUPERTILE_PER_PRIM_EXCLUDE_PASS_EMPTY_PRIM_H0 = 0x000001cau32,
SC_SUPERTILE_PER_PRIM_EXCLUDE_PASS_EMPTY_PRIM_H1 = 0x000001cbu32,
SC_SUPERTILE_PER_PRIM_EXCLUDE_PASS_EMPTY_PRIM_H2 = 0x000001ccu32,
SC_SUPERTILE_PER_PRIM_EXCLUDE_PASS_EMPTY_PRIM_H3 = 0x000001cdu32,
SC_SUPERTILE_PER_PRIM_EXCLUDE_PASS_EMPTY_PRIM_H4 = 0x000001ceu32,
SC_SUPERTILE_PER_PRIM_EXCLUDE_PASS_EMPTY_PRIM_H5 = 0x000001cfu32,
SC_SUPERTILE_PER_PRIM_EXCLUDE_PASS_EMPTY_PRIM_H6 = 0x000001d0u32,
SC_SUPERTILE_PER_PRIM_EXCLUDE_PASS_EMPTY_PRIM_H7 = 0x000001d1u32,
SC_SUPERTILE_PER_PRIM_EXCLUDE_PASS_EMPTY_PRIM_H8 = 0x000001d2u32,
SC_SUPERTILE_PER_PRIM_EXCLUDE_PASS_EMPTY_PRIM_H9 = 0x000001d3u32,
SC_SUPERTILE_PER_PRIM_EXCLUDE_PASS_EMPTY_PRIM_H10 = 0x000001d4u32,
SC_SUPERTILE_PER_PRIM_EXCLUDE_PASS_EMPTY_PRIM_H11 = 0x000001d5u32,
SC_SUPERTILE_PER_PRIM_EXCLUDE_PASS_EMPTY_PRIM_H12 = 0x000001d6u32,
SC_SUPERTILE_PER_PRIM_EXCLUDE_PASS_EMPTY_PRIM_H13 = 0x000001d7u32,
SC_SUPERTILE_PER_PRIM_EXCLUDE_PASS_EMPTY_PRIM_H14 = 0x000001d8u32,
SC_SUPERTILE_PER_PRIM_EXCLUDE_PASS_EMPTY_PRIM_H15 = 0x000001d9u32,
SC_SUPERTILE_PER_PRIM_EXCLUDE_PASS_EMPTY_PRIM_H16 = 0x000001dau32,
SC_DB0_TILE_INTERFACE_BUSY               = 0x000001dbu32,
SC_DB0_TILE_INTERFACE_SEND               = 0x000001dcu32,
SC_DB0_TILE_INTERFACE_SEND_EVENT         = 0x000001ddu32,
SC_PERF_SEL_RESERVED_478                 = 0x000001deu32,
SC_PERF_SEL_RESERVED_479                 = 0x000001dfu32,
SC_DB0_TILE_INTERFACE_CREDIT_AT_ZERO_WITH_PENDING_SEND = 0x000001e0u32,
SC_DB0_TILE_INTERFACE_CREDIT_AT_MAX      = 0x000001e1u32,
SC_DB0_TILE_INTERFACE_CREDIT_AT_MAX_WITH_NO_PENDING_SEND = 0x000001e2u32,
SC_PERF_SEL_RESERVED_483                 = 0x000001e3u32,
SC_PERF_SEL_RESERVED_484                 = 0x000001e4u32,
SC_PERF_SEL_RESERVED_485                 = 0x000001e5u32,
SC_PERF_SEL_RESERVED_486                 = 0x000001e6u32,
SC_PERF_SEL_RESERVED_487                 = 0x000001e7u32,
SC_PERF_SEL_RESERVED_488                 = 0x000001e8u32,
SC_PERF_SEL_RESERVED_489                 = 0x000001e9u32,
SC_PERF_SEL_RESERVED_490                 = 0x000001eau32,
SC_BACKEND_PRIM_FIFO_FULL                = 0x000001ebu32,
SC_PBB_BATCH_BREAK_DUE_TO_TIMEOUT_COUNTER = 0x000001ecu32,
SC_PBB_BATCH_BREAK_DUE_TO_NONBINNED_BATCH = 0x000001edu32,
SC_PBB_BATCH_BREAK_DUE_TO_DEBUG_DATA_PER_DRAW_DISPATCH = 0x000001eeu32,
SC_PBB_BATCH_BREAK_DUE_TO_OVERRIDE_REGISTER_PERSISTENT = 0x000001efu32,
SC_PBB_BATCH_BREAK_DUE_TO_OVERRIDE_REGISTER_CONTEXT = 0x000001f0u32,
SC_PBB_BATCH_BREAK_DUE_TO_OVERRIDE_REGISTER_FPOV = 0x000001f1u32,
SC_PBB_BATCH_BREAK_DUE_TO_NEW_SC_MODE    = 0x000001f2u32,
SC_PBB_BATCH_BREAK_DUE_TO_BINNING_MODE_CHANGE = 0x000001f3u32,
SC_PBB_BATCH_BREAK_DUE_TO_PIPELINE_EVENT_COUNT = 0x000001f4u32,
SC_PBB_BATCH_BREAK_DUE_TO_PIPE_RESET     = 0x000001f5u32,
SC_PBB_BATCH_BREAK_DUE_TO_GFX_PIPE_CHANGE = 0x000001f6u32,
SC_STALLED_BY_DB0_TILEFIFO               = 0x000001f7u32,
SC_DB0_QUAD_INTF_SEND                    = 0x000001f8u32,
SC_DB0_QUAD_INTF_BUSY                    = 0x000001f9u32,
SC_DB0_QUAD_INTF_STALLED_BY_DB           = 0x000001fau32,
SC_DB0_QUAD_INTF_CREDIT_AT_MAX           = 0x000001fbu32,
SC_DB0_QUAD_INTF_IDLE                    = 0x000001fcu32,
SC_PERF_SEL_RESERVED_509                 = 0x000001fdu32,
SC_PERF_SEL_RESERVED_510                 = 0x000001feu32,
SC_PERF_SEL_RESERVED_511                 = 0x000001ffu32,
SC_PERF_SEL_RESERVED_512                 = 0x00000200u32,
SC_PERF_SEL_RESERVED_513                 = 0x00000201u32,
SC_PERF_SEL_RESERVED_514                 = 0x00000202u32,
SC_PKR_WAVE_BREAK_OUTSIDE_REGION         = 0x00000203u32,
SC_PKR_WAVE_BREAK_FULL_TILE              = 0x00000204u32,
SC_RESERVED_60                           = 0x00000205u32,
SC_PBB_EMPTY_INPUT_CYCLE_WHEN_BATCH_OPEN = 0x00000206u32,
SC_PBB_BATCH_BREAK_DUE_TO_NULL_PRIM_BREAK_BATCH_LIMIT = 0x00000207u32,
SC_DB0_WE_STALLED_BY_RSLT_FIFO_FULL      = 0x00000208u32,
SC_DB0_WE_TILE_MASK_RETURN_FIFO_FULL_WITH_WE_RSLT_FIFO_STALL = 0x00000209u32,
SC_DB0_TILE_MASK_FIFO_FULL               = 0x0000020au32,
SC_PERF_SEL_RESERVED_523                 = 0x0000020bu32,
SC_PERF_SEL_RESERVED_524                 = 0x0000020cu32,
SC_PERF_SEL_RESERVED_525                 = 0x0000020du32,
SC_PS_PM_PBB_TO_PSE_FIFO_WE_STALL_BY_PFF_PW_FULL = 0x0000020eu32,
SC_PS_PM_PBB_TO_PSE_FIFO_WE_STALL_BY_ZFF_PW_FULL = 0x0000020fu32,
SC_PS_PM_PBB_TO_PSE_FIFO_WE_STALL_BY_PBB_TO_PSE_FIFO_FULL = 0x00000210u32,
SC_PS_PM_PFF_PW_FULL                     = 0x00000211u32,
SC_PS_PM_ZFF_PW_FULL                     = 0x00000212u32,
SC_PS_PM_PBB_TO_PSE_FIFO_FULL            = 0x00000213u32,
SC_PERF_SEL_RESERVED_532                 = 0x00000214u32,
SC_PERF_SEL_RESERVED_533                 = 0x00000215u32,
SC_PERF_SEL_RESERVED_534                 = 0x00000216u32,
SC_PK_PM_4X2_SPLIT_WAVE_BRK_1H           = 0x00000217u32,
SC_PK_PM_PKR_FILL_4X2_WAVE_BRK_1H        = 0x00000218u32,
SC_PK_PM_SPLIT_OR_FILL_4X2_WAVE_BRK_1H   = 0x00000219u32,
SC_PK_PM_END_OF_VECTOR_WAVE_BRK_1H       = 0x0000021au32,
SC_PERF_SEL_RESERVED_539                 = 0x0000021bu32,
SC_PK_PM_CTL_ONLY_CMD_WAVE_BRK_1H        = 0x0000021cu32,
SC_PK_PM_AVOID_DEALLOC_ADD_WAVE_BRK_1H   = 0x0000021du32,
SC_PK_PM_FD_CONFLICT_WAVE_BRK_1H         = 0x0000021eu32,
SC_PK_PM_FORCE_PARTIAL_FOR_DEALLOC_WAVE_BRK_1H = 0x0000021fu32,
SC_PK_PM_AE_CONFLICT_WAVE_BRK_1H         = 0x00000220u32,
SC_PK_PM_EOP_OR_LAD_WAVE_BRK_1H          = 0x00000221u32,
SC_PK_PM_FULL_TILE_WAVE_BRK_1H           = 0x00000222u32,
SC_PK_PM_OREO_CONFLICT_QUAD_FORCE_EOV_WAVE_BRK_1H = 0x00000223u32,
SC_PK_PM_MAX_DEALLOC_FORCE_EOV_WAVE_BRK_1H = 0x00000224u32,
SC_PK_PM_WAVE_BREAK_OUTSIDE_REGION_WAVE_BRK_1H = 0x00000225u32,
SC_PK_PM_MAX_CLK_CNT_FORCE_EOV_WAVE_BRK_1H = 0x00000226u32,
SC_PK_PM_MAX_REZ_CNT_FORCE_EOV_WAVE_BRK_1H = 0x00000227u32,
SC_PK_PM_VRS_RATE_X_00_Y_00_QUAD         = 0x00000228u32,
SC_PK_PM_VRS_RATE_X_00_Y_01_QUAD         = 0x00000229u32,
SC_PK_PM_VRS_RATE_X_00_Y_10_QUAD         = 0x0000022au32,
SC_PK_PM_VRS_RATE_X_00_Y_11_QUAD         = 0x0000022bu32,
SC_PK_PM_VRS_RATE_X_01_Y_00_QUAD         = 0x0000022cu32,
SC_PK_PM_VRS_RATE_X_01_Y_01_QUAD         = 0x0000022du32,
SC_PK_PM_VRS_RATE_X_01_Y_10_QUAD         = 0x0000022eu32,
SC_PK_PM_VRS_RATE_X_01_Y_11_QUAD         = 0x0000022fu32,
SC_PK_PM_VRS_RATE_X_10_Y_00_QUAD         = 0x00000230u32,
SC_PK_PM_VRS_RATE_X_10_Y_01_QUAD         = 0x00000231u32,
SC_PK_PM_VRS_RATE_X_10_Y_10_QUAD         = 0x00000232u32,
SC_PK_PM_VRS_RATE_X_10_Y_11_QUAD         = 0x00000233u32,
SC_PK_PM_VRS_RATE_X_11_Y_00_QUAD         = 0x00000234u32,
SC_PK_PM_VRS_RATE_X_11_Y_01_QUAD         = 0x00000235u32,
SC_PK_PM_VRS_RATE_X_11_Y_10_QUAD         = 0x00000236u32,
SC_PK_PM_VRS_RATE_X_11_Y_11_QUAD         = 0x00000237u32,
SC_PERF_SEL_RESERVED_568                 = 0x00000238u32,
SC_PBB_RESERVED                          = 0x00000239u32,
SC_BM_BE0_STALLED                        = 0x0000023au32,
SC_BM_BE1_STALLED                        = 0x0000023bu32,
SC_BM_BE2_STALLED                        = 0x0000023cu32,
SC_BM_BE3_STALLED                        = 0x0000023du32,
SC_BM_MULTI_ACCUM_1_BE_STALLED           = 0x0000023eu32,
SC_BM_MULTI_ACCUM_2_BE_STALLED           = 0x0000023fu32,
SC_BM_MULTI_ACCUM_3_BE_STALLED           = 0x00000240u32,
SC_BM_MULTI_ACCUM_4_BE_STALLED           = 0x00000241u32,
SC_PBB_READ_PH0                          = 0x00000242u32,
SC_PBB_READ_DEALLOC_4_0                  = 0x00000243u32,
SC_PBB_READ_DEALLOC_7_5                  = 0x00000244u32,
SC_PBB_READ_FPOG_4_0                     = 0x00000245u32,
SC_PBB_READ_FPOG_7_5                     = 0x00000246u32,
SC_VRC_SECTOR_HIT                        = 0x00000247u32,
SC_VRC_TAG_MISS                          = 0x00000248u32,
SC_VRC_SECTOR_MISS                       = 0x00000249u32,
SC_VRC_LRU_EVICT_STALL                   = 0x0000024au32,
SC_VRC_LRU_EVICT_SCHEDULED_EVICT_STALL   = 0x0000024bu32,
SC_VRC_LRU_EVICT_PENDING_EVICT_STALL     = 0x0000024cu32,
SC_VRC_REEVICTION_STALL                  = 0x0000024du32,
SC_VRC_EVICT_NONZERO_INFLIGHT_STALL      = 0x0000024eu32,
SC_VRC_REPLACE_SCHEDULED_EVICT_STALL     = 0x0000024fu32,
SC_VRC_REPLACE_PENDING_EVICT_STALL       = 0x00000250u32,
SC_VRC_REPLACE_FLUSH_IN_PROGRESS_STALL   = 0x00000251u32,
SC_VRC_INFLIGHT_COUNTER_MAXIMUM_STALL    = 0x00000252u32,
SC_VRC_READ_OUTPUT_STALL                 = 0x00000253u32,
SC_VRC_WRITE_OUTPUT_STALL                = 0x00000254u32,
SC_VRC_ACK_OUTPUT_STALL                  = 0x00000255u32,
SC_VRC_FLUSH_EVICT_STALL                 = 0x00000256u32,
SC_VRC_FLUSH_REFLUSH_STALL               = 0x00000257u32,
SC_VRC_FLUSH_FIP_HIT_STALL               = 0x00000258u32,
SC_VRC_FLUSH_WRREQ_DRAIN_STALL           = 0x00000259u32,
SC_VRC_FLUSH_DONE_STALL                  = 0x0000025au32,
SC_VRC_FLUSH_STALL                       = 0x0000025bu32,
SC_VRC_STALL                             = 0x0000025cu32,
SC_VRC_FLUSH                             = 0x0000025du32,
SC_VRC_SECTORS_FLUSHED                   = 0x0000025eu32,
SC_VRC_DIRTY_SECTORS_FLUSHED             = 0x0000025fu32,
SC_VRC_TAGS_FLUSHED                      = 0x00000260u32,
SC_VRC_HPF_REQ                           = 0x00000261u32,
SC_VRC_HPF_EVENT                         = 0x00000262u32,
SC_VRC_HPF_STALLED                       = 0x00000263u32,
SC_VRC_PROBE_ACK_TILES                   = 0x00000264u32,
SC_VRC_GL1X_RD_REQ                       = 0x00000265u32,
SC_VRC_GL1X_WR_REQ                       = 0x00000266u32,
SC_VRC_GL1X_SRC_XFR                      = 0x00000267u32,
SC_VRC_GL1X_RD_RET                       = 0x00000268u32,
SC_VRC_GL1X_WR_ACK                       = 0x00000269u32,
SC_VRC_GL1X_RD_XNACK                     = 0x0000026au32,
SC_VRC_GL1X_WR_XNACK                     = 0x0000026bu32,
SC_VRC_GL1X_REQ_STALLED                  = 0x0000026cu32,
SC_VRC_GL1X_SRC_STALLED                  = 0x0000026du32,
SC_VRC_RATEMEM_WE_CNT                    = 0x0000026eu32,
SC_VRC_RATEMEM_RE_CNT                    = 0x0000026fu32,
SC_VRC_HINTMEM_WE_CNT                    = 0x00000270u32,
SC_VRC_HINTMEM_RE_CNT                    = 0x00000271u32,
SC_VRC_BUSY                              = 0x00000272u32,
SC_GL1X_BUSY                             = 0x00000273u32,
SC_BE_VRS_RD_REQ                         = 0x00000274u32,
SC_BE_VRS_RD_REQ_STALLED                 = 0x00000275u32,
SC_BE_VRS_RD_REQ_HIT                     = 0x00000276u32,
SC_BE_VRS_RD_RET                         = 0x00000277u32,
SC_BE_VRS_RD_RET_STALLED                 = 0x00000278u32,
SC_BE_VRS_FB_RET                         = 0x00000279u32,
SC_BE_VRS_FB_RET_STALLED                 = 0x0000027au32,
SC_BE_VRS_FB_RET_HIT                     = 0x0000027bu32,
SC_VRS_BE_BUSY                           = 0x0000027cu32,
SC_PWS_CS_EVENTS_PWS_ENABLE              = 0x0000027du32,
SC_PWS_PS_EVENTS_PWS_ENABLE              = 0x0000027eu32,
SC_PWS_TS_EVENTS_PWS_ENABLE              = 0x0000027fu32,
SC_PWS_STALLED                           = 0x00000280u32,
SC_PWS_P0_CS_SYNC_COMPLETE               = 0x00000281u32,
SC_PWS_P0_PS_SYNC_COMPLETE               = 0x00000282u32,
SC_PWS_P0_TS_SYNC_COMPLETE               = 0x00000283u32,
SC_PWS_P1_CS_SYNC_COMPLETE               = 0x00000284u32,
SC_PWS_P1_PS_SYNC_COMPLETE               = 0x00000285u32,
SC_PWS_P1_TS_SYNC_COMPLETE               = 0x00000286u32,
SC_PKR_PC_NO_CREDITS                     = 0x00000287u32,
SC_PKR_PC_STALLED                        = 0x00000288u32,
SC_PKR_PC_SEND                           = 0x00000289u32,
SC_PKR_PC_SEND_PRIM_VALID_1              = 0x0000028au32,
SC_PKR_PC_SEND_PRIM_VALID_0              = 0x0000028bu32,
SC_PKR_PC_SEND_TRUE_PRIM                 = 0x0000028cu32,
SC_PKR_PC_SEND_EOV                       = 0x0000028du32,
SC_PKR_PC_SEND_EVENT                     = 0x0000028eu32,
SC_PKR_DB_WAVE_STALL                     = 0x0000028fu32,
SC_PKR_PSINVOC_SEDC_FIFO_FULL            = 0x00000290u32,
SC_PKR_OREO_STALLED_BY_NO_VALID_WAIVE_ID = 0x00000291u32,
SC_PKR_SPI_QUAD_COUNT                    = 0x00000292u32,
SC_PKR_DB_OREO_WAVE_QUAD_COUNT           = 0x00000293u32,
SC_PKR_BCI_QUAD_NEW_PRIM                 = 0x00000294u32,
SC_SPI_WAVE_STALLED_BY_SPI               = 0x00000295u32,
}

/*
 * ScMap enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ScMap {
RASTER_CONFIG_SC_MAP_0                   = 0x00000000u32,
RASTER_CONFIG_SC_MAP_1                   = 0x00000001u32,
RASTER_CONFIG_SC_MAP_2                   = 0x00000002u32,
RASTER_CONFIG_SC_MAP_3                   = 0x00000003u32,
}

/*
 * ScUncertaintyRegionMode enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ScUncertaintyRegionMode {
SC_HALF_LSB                              = 0x00000000u32,
SC_LSB_ONE_SIDED                         = 0x00000001u32,
SC_LSB_TWO_SIDED                         = 0x00000002u32,
}

/*
 * ScUncertaintyRegionMult enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ScUncertaintyRegionMult {
SC_UR_1X                                 = 0x00000000u32,
SC_UR_2X                                 = 0x00000001u32,
SC_UR_4X                                 = 0x00000002u32,
SC_UR_8X                                 = 0x00000003u32,
}

/*
 * ScXsel enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ScXsel {
RASTER_CONFIG_SC_XSEL_8_WIDE_TILE        = 0x00000000u32,
RASTER_CONFIG_SC_XSEL_16_WIDE_TILE       = 0x00000001u32,
RASTER_CONFIG_SC_XSEL_32_WIDE_TILE       = 0x00000002u32,
RASTER_CONFIG_SC_XSEL_64_WIDE_TILE       = 0x00000003u32,
}

/*
 * ScYsel enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ScYsel {
RASTER_CONFIG_SC_YSEL_8_WIDE_TILE        = 0x00000000u32,
RASTER_CONFIG_SC_YSEL_16_WIDE_TILE       = 0x00000001u32,
RASTER_CONFIG_SC_YSEL_32_WIDE_TILE       = 0x00000002u32,
RASTER_CONFIG_SC_YSEL_64_WIDE_TILE       = 0x00000003u32,
}

/*
 * SeMap enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SeMap {
RASTER_CONFIG_SE_MAP_0                   = 0x00000000u32,
RASTER_CONFIG_SE_MAP_1                   = 0x00000001u32,
RASTER_CONFIG_SE_MAP_2                   = 0x00000002u32,
RASTER_CONFIG_SE_MAP_3                   = 0x00000003u32,
}

/*
 * SePairMap enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SePairMap {
RASTER_CONFIG_SE_PAIR_MAP_0              = 0x00000000u32,
RASTER_CONFIG_SE_PAIR_MAP_1              = 0x00000001u32,
RASTER_CONFIG_SE_PAIR_MAP_2              = 0x00000002u32,
RASTER_CONFIG_SE_PAIR_MAP_3              = 0x00000003u32,
}

/*
 * SePairXsel enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SePairXsel {
RASTER_CONFIG_SE_PAIR_XSEL_8_WIDE_TILE   = 0x00000000u32,
RASTER_CONFIG_SE_PAIR_XSEL_16_WIDE_TILE  = 0x00000001u32,
RASTER_CONFIG_SE_PAIR_XSEL_32_WIDE_TILE  = 0x00000002u32,
RASTER_CONFIG_SE_PAIR_XSEL_64_WIDE_TILE  = 0x00000003u32,
}

/*
 * SePairYsel enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SePairYsel {
RASTER_CONFIG_SE_PAIR_YSEL_8_WIDE_TILE   = 0x00000000u32,
RASTER_CONFIG_SE_PAIR_YSEL_16_WIDE_TILE  = 0x00000001u32,
RASTER_CONFIG_SE_PAIR_YSEL_32_WIDE_TILE  = 0x00000002u32,
RASTER_CONFIG_SE_PAIR_YSEL_64_WIDE_TILE  = 0x00000003u32,
}

/*
 * SeXsel enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SeXsel {
RASTER_CONFIG_SE_XSEL_8_WIDE_TILE        = 0x00000000u32,
RASTER_CONFIG_SE_XSEL_16_WIDE_TILE       = 0x00000001u32,
RASTER_CONFIG_SE_XSEL_32_WIDE_TILE       = 0x00000002u32,
RASTER_CONFIG_SE_XSEL_64_WIDE_TILE       = 0x00000003u32,
}

/*
 * SeYsel enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SeYsel {
RASTER_CONFIG_SE_YSEL_8_WIDE_TILE        = 0x00000000u32,
RASTER_CONFIG_SE_YSEL_16_WIDE_TILE       = 0x00000001u32,
RASTER_CONFIG_SE_YSEL_32_WIDE_TILE       = 0x00000002u32,
RASTER_CONFIG_SE_YSEL_64_WIDE_TILE       = 0x00000003u32,
}

/*
 * VRSCombinerModeSC enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VRSCombinerModeSC {
SC_VRS_COMB_MODE_PASSTHRU                = 0x00000000u32,
SC_VRS_COMB_MODE_OVERRIDE                = 0x00000001u32,
SC_VRS_COMB_MODE_MIN                     = 0x00000002u32,
SC_VRS_COMB_MODE_MAX                     = 0x00000003u32,
SC_VRS_COMB_MODE_SATURATE                = 0x00000004u32,
}

/*
 * VRSrate enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VRSrate {
VRS_SHADING_RATE_1X1                     = 0x00000000u32,
VRS_SHADING_RATE_1X2                     = 0x00000001u32,
VRS_SHADING_RATE_UNDEFINED0              = 0x00000002u32,
VRS_SHADING_RATE_UNDEFINED1              = 0x00000003u32,
VRS_SHADING_RATE_2X1                     = 0x00000004u32,
VRS_SHADING_RATE_2X2                     = 0x00000005u32,
VRS_SHADING_RATE_2X4                     = 0x00000006u32,
VRS_SHADING_RATE_UNDEFINED2              = 0x00000007u32,
VRS_SHADING_RATE_UNDEFINED3              = 0x00000008u32,
VRS_SHADING_RATE_4X2                     = 0x00000009u32,
VRS_SHADING_RATE_4X4                     = 0x0000000au32,
VRS_SHADING_RATE_UNDEFINED4              = 0x0000000bu32,
VRS_SHADING_RATE_16X_SSAA                = 0x0000000cu32,
VRS_SHADING_RATE_8X_SSAA                 = 0x0000000du32,
VRS_SHADING_RATE_4X_SSAA                 = 0x0000000eu32,
VRS_SHADING_RATE_2X_SSAA                 = 0x0000000fu32,
}

/*******************************************************
 * TC Enums
 *******************************************************/

/*
 * TC_EA_CID enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TC_EA_CID {
TC_EA_CID_RT                             = 0x00000000u32,
TC_EA_CID_FMASK                          = 0x00000001u32,
TC_EA_CID_DCC                            = 0x00000002u32,
TC_EA_CID_TCPMETA                        = 0x00000003u32,
TC_EA_CID_Z                              = 0x00000004u32,
TC_EA_CID_STENCIL                        = 0x00000005u32,
TC_EA_CID_HTILE                          = 0x00000006u32,
TC_EA_CID_MISC                           = 0x00000007u32,
TC_EA_CID_TCP                            = 0x00000008u32,
TC_EA_CID_SQC                            = 0x00000009u32,
TC_EA_CID_CPF                            = 0x0000000au32,
TC_EA_CID_CPG                            = 0x0000000bu32,
TC_EA_CID_IA                             = 0x0000000cu32,
TC_EA_CID_WD                             = 0x0000000du32,
TC_EA_CID_PA                             = 0x0000000eu32,
TC_EA_CID_UTCL2_TPI                      = 0x0000000fu32,
}

/*
 * TC_NACKS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TC_NACKS {
TC_NACK_NO_FAULT                         = 0x00000000u32,
TC_NACK_PAGE_FAULT                       = 0x00000001u32,
TC_NACK_PROTECTION_FAULT                 = 0x00000002u32,
TC_NACK_DATA_ERROR                       = 0x00000003u32,
}

/*
 * TC_OP enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TC_OP {
TC_OP_READ                               = 0x00000000u32,
TC_OP_ATOMIC_FCMPSWAP_RTN_32             = 0x00000001u32,
TC_OP_ATOMIC_FMIN_RTN_32                 = 0x00000002u32,
TC_OP_ATOMIC_FMAX_RTN_32                 = 0x00000003u32,
TC_OP_RESERVED_FOP_RTN_32_0              = 0x00000004u32,
TC_OP_RESERVED_FADD_RTN_32               = 0x00000005u32,
TC_OP_RESERVED_FOP_RTN_32_2              = 0x00000006u32,
TC_OP_ATOMIC_SWAP_RTN_32                 = 0x00000007u32,
TC_OP_ATOMIC_CMPSWAP_RTN_32              = 0x00000008u32,
TC_OP_ATOMIC_FCMPSWAP_FLUSH_DENORM_RTN_32 = 0x00000009u32,
TC_OP_ATOMIC_FMIN_FLUSH_DENORM_RTN_32    = 0x0000000au32,
TC_OP_ATOMIC_FMAX_FLUSH_DENORM_RTN_32    = 0x0000000bu32,
TC_OP_PROBE_FILTER                       = 0x0000000cu32,
TC_OP_ATOMIC_FADD_FLUSH_DENORM_RTN_32    = 0x0000000du32,
TC_OP_RESERVED_FOP_FLUSH_DENORM_RTN_32_2 = 0x0000000eu32,
TC_OP_ATOMIC_ADD_RTN_32                  = 0x0000000fu32,
TC_OP_ATOMIC_SUB_RTN_32                  = 0x00000010u32,
TC_OP_ATOMIC_SMIN_RTN_32                 = 0x00000011u32,
TC_OP_ATOMIC_UMIN_RTN_32                 = 0x00000012u32,
TC_OP_ATOMIC_SMAX_RTN_32                 = 0x00000013u32,
TC_OP_ATOMIC_UMAX_RTN_32                 = 0x00000014u32,
TC_OP_ATOMIC_AND_RTN_32                  = 0x00000015u32,
TC_OP_ATOMIC_OR_RTN_32                   = 0x00000016u32,
TC_OP_ATOMIC_XOR_RTN_32                  = 0x00000017u32,
TC_OP_ATOMIC_INC_RTN_32                  = 0x00000018u32,
TC_OP_ATOMIC_DEC_RTN_32                  = 0x00000019u32,
TC_OP_WBINVL1_VOL                        = 0x0000001au32,
TC_OP_WBINVL1_SD                         = 0x0000001bu32,
TC_OP_RESERVED_NON_FLOAT_RTN_32_0        = 0x0000001cu32,
TC_OP_RESERVED_NON_FLOAT_RTN_32_1        = 0x0000001du32,
TC_OP_RESERVED_NON_FLOAT_RTN_32_2        = 0x0000001eu32,
TC_OP_RESERVED_NON_FLOAT_RTN_32_3        = 0x0000001fu32,
TC_OP_WRITE                              = 0x00000020u32,
TC_OP_ATOMIC_FCMPSWAP_RTN_64             = 0x00000021u32,
TC_OP_ATOMIC_FMIN_RTN_64                 = 0x00000022u32,
TC_OP_ATOMIC_FMAX_RTN_64                 = 0x00000023u32,
TC_OP_RESERVED_FOP_RTN_64_0              = 0x00000024u32,
TC_OP_RESERVED_FOP_RTN_64_1              = 0x00000025u32,
TC_OP_RESERVED_FOP_RTN_64_2              = 0x00000026u32,
TC_OP_ATOMIC_SWAP_RTN_64                 = 0x00000027u32,
TC_OP_ATOMIC_CMPSWAP_RTN_64              = 0x00000028u32,
TC_OP_ATOMIC_FCMPSWAP_FLUSH_DENORM_RTN_64 = 0x00000029u32,
TC_OP_ATOMIC_FMIN_FLUSH_DENORM_RTN_64    = 0x0000002au32,
TC_OP_ATOMIC_FMAX_FLUSH_DENORM_RTN_64    = 0x0000002bu32,
TC_OP_WBINVL2_SD                         = 0x0000002cu32,
TC_OP_RESERVED_FOP_FLUSH_DENORM_RTN_64_0 = 0x0000002du32,
TC_OP_RESERVED_FOP_FLUSH_DENORM_RTN_64_1 = 0x0000002eu32,
TC_OP_ATOMIC_ADD_RTN_64                  = 0x0000002fu32,
TC_OP_ATOMIC_SUB_RTN_64                  = 0x00000030u32,
TC_OP_ATOMIC_SMIN_RTN_64                 = 0x00000031u32,
TC_OP_ATOMIC_UMIN_RTN_64                 = 0x00000032u32,
TC_OP_ATOMIC_SMAX_RTN_64                 = 0x00000033u32,
TC_OP_ATOMIC_UMAX_RTN_64                 = 0x00000034u32,
TC_OP_ATOMIC_AND_RTN_64                  = 0x00000035u32,
TC_OP_ATOMIC_OR_RTN_64                   = 0x00000036u32,
TC_OP_ATOMIC_XOR_RTN_64                  = 0x00000037u32,
TC_OP_ATOMIC_INC_RTN_64                  = 0x00000038u32,
TC_OP_ATOMIC_DEC_RTN_64                  = 0x00000039u32,
TC_OP_WBL2_NC                            = 0x0000003au32,
TC_OP_WBL2_WC                            = 0x0000003bu32,
TC_OP_RESERVED_NON_FLOAT_RTN_64_1        = 0x0000003cu32,
TC_OP_RESERVED_NON_FLOAT_RTN_64_2        = 0x0000003du32,
TC_OP_RESERVED_NON_FLOAT_RTN_64_3        = 0x0000003eu32,
TC_OP_RESERVED_NON_FLOAT_RTN_64_4        = 0x0000003fu32,
TC_OP_WBINVL1                            = 0x00000040u32,
TC_OP_ATOMIC_FCMPSWAP_32                 = 0x00000041u32,
TC_OP_ATOMIC_FMIN_32                     = 0x00000042u32,
TC_OP_ATOMIC_FMAX_32                     = 0x00000043u32,
TC_OP_RESERVED_FOP_32_0                  = 0x00000044u32,
TC_OP_RESERVED_FADD_32                   = 0x00000045u32,
TC_OP_RESERVED_FOP_32_2                  = 0x00000046u32,
TC_OP_ATOMIC_SWAP_32                     = 0x00000047u32,
TC_OP_ATOMIC_CMPSWAP_32                  = 0x00000048u32,
TC_OP_ATOMIC_FCMPSWAP_FLUSH_DENORM_32    = 0x00000049u32,
TC_OP_ATOMIC_FMIN_FLUSH_DENORM_32        = 0x0000004au32,
TC_OP_ATOMIC_FMAX_FLUSH_DENORM_32        = 0x0000004bu32,
TC_OP_INV_METADATA                       = 0x0000004cu32,
TC_OP_ATOMIC_FADD_FLUSH_DENORM_32        = 0x0000004du32,
TC_OP_RESERVED_FOP_FLUSH_DENORM_32_2     = 0x0000004eu32,
TC_OP_ATOMIC_ADD_32                      = 0x0000004fu32,
TC_OP_ATOMIC_SUB_32                      = 0x00000050u32,
TC_OP_ATOMIC_SMIN_32                     = 0x00000051u32,
TC_OP_ATOMIC_UMIN_32                     = 0x00000052u32,
TC_OP_ATOMIC_SMAX_32                     = 0x00000053u32,
TC_OP_ATOMIC_UMAX_32                     = 0x00000054u32,
TC_OP_ATOMIC_AND_32                      = 0x00000055u32,
TC_OP_ATOMIC_OR_32                       = 0x00000056u32,
TC_OP_ATOMIC_XOR_32                      = 0x00000057u32,
TC_OP_ATOMIC_INC_32                      = 0x00000058u32,
TC_OP_ATOMIC_DEC_32                      = 0x00000059u32,
TC_OP_INVL2_NC                           = 0x0000005au32,
TC_OP_NOP_RTN0                           = 0x0000005bu32,
TC_OP_RESERVED_NON_FLOAT_32_1            = 0x0000005cu32,
TC_OP_RESERVED_NON_FLOAT_32_2            = 0x0000005du32,
TC_OP_RESERVED_NON_FLOAT_32_3            = 0x0000005eu32,
TC_OP_RESERVED_NON_FLOAT_32_4            = 0x0000005fu32,
TC_OP_WBINVL2                            = 0x00000060u32,
TC_OP_ATOMIC_FCMPSWAP_64                 = 0x00000061u32,
TC_OP_ATOMIC_FMIN_64                     = 0x00000062u32,
TC_OP_ATOMIC_FMAX_64                     = 0x00000063u32,
TC_OP_RESERVED_FOP_64_0                  = 0x00000064u32,
TC_OP_RESERVED_FOP_64_1                  = 0x00000065u32,
TC_OP_RESERVED_FOP_64_2                  = 0x00000066u32,
TC_OP_ATOMIC_SWAP_64                     = 0x00000067u32,
TC_OP_ATOMIC_CMPSWAP_64                  = 0x00000068u32,
TC_OP_ATOMIC_FCMPSWAP_FLUSH_DENORM_64    = 0x00000069u32,
TC_OP_ATOMIC_FMIN_FLUSH_DENORM_64        = 0x0000006au32,
TC_OP_ATOMIC_FMAX_FLUSH_DENORM_64        = 0x0000006bu32,
TC_OP_RESERVED_FOP_FLUSH_DENORM_64_0     = 0x0000006cu32,
TC_OP_RESERVED_FOP_FLUSH_DENORM_64_1     = 0x0000006du32,
TC_OP_RESERVED_FOP_FLUSH_DENORM_64_2     = 0x0000006eu32,
TC_OP_ATOMIC_ADD_64                      = 0x0000006fu32,
TC_OP_ATOMIC_SUB_64                      = 0x00000070u32,
TC_OP_ATOMIC_SMIN_64                     = 0x00000071u32,
TC_OP_ATOMIC_UMIN_64                     = 0x00000072u32,
TC_OP_ATOMIC_SMAX_64                     = 0x00000073u32,
TC_OP_ATOMIC_UMAX_64                     = 0x00000074u32,
TC_OP_ATOMIC_AND_64                      = 0x00000075u32,
TC_OP_ATOMIC_OR_64                       = 0x00000076u32,
TC_OP_ATOMIC_XOR_64                      = 0x00000077u32,
TC_OP_ATOMIC_INC_64                      = 0x00000078u32,
TC_OP_ATOMIC_DEC_64                      = 0x00000079u32,
TC_OP_WBINVL2_NC                         = 0x0000007au32,
TC_OP_NOP_ACK                            = 0x0000007bu32,
TC_OP_RESERVED_NON_FLOAT_64_1            = 0x0000007cu32,
TC_OP_RESERVED_NON_FLOAT_64_2            = 0x0000007du32,
TC_OP_RESERVED_NON_FLOAT_64_3            = 0x0000007eu32,
TC_OP_RESERVED_NON_FLOAT_64_4            = 0x0000007fu32,
}

/*
 * TC_OP_MASKS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TC_OP_MASKS {
TC_OP_MASK_FLUSH_DENROM                  = 0x00000008u32,
TC_OP_MASK_64                            = 0x00000020u32,
TC_OP_MASK_NO_RTN                        = 0x00000040u32,
}

/*******************************************************
 * SPI Enums
 *******************************************************/

/*
 * CLKGATE_BASE_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CLKGATE_BASE_MODE {
MULT_8                                   = 0x00000000u32,
MULT_16                                  = 0x00000001u32,
}

/*
 * CLKGATE_SM_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CLKGATE_SM_MODE {
ON_SEQ                                   = 0x00000000u32,
OFF_SEQ                                  = 0x00000001u32,
PROG_SEQ                                 = 0x00000002u32,
READ_SEQ                                 = 0x00000003u32,
SM_MODE_RESERVED                         = 0x00000004u32,
}

/*
 * CovToShaderSel enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CovToShaderSel {
INPUT_COVERAGE                           = 0x00000000u32,
INPUT_INNER_COVERAGE                     = 0x00000001u32,
INPUT_DEPTH_COVERAGE                     = 0x00000002u32,
RAW                                      = 0x00000003u32,
}

/*
 * PC_PERFCNT_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PC_PERFCNT_SEL {
PC_PERF_SC_PC_PTR_SEND0                  = 0x00000000u32,
PC_PERF_SC_PC_PTR_VALID0                 = 0x00000001u32,
PC_PERF_SC_FPOSG0                        = 0x00000002u32,
PC_PERF_SC_FPOSG_WAIT0                   = 0x00000003u32,
PC_PERF_SC_WAIT_SYNC0                    = 0x00000004u32,
PC_PERF_SC_PQ_FREEZE0                    = 0x00000005u32,
PC_PERF_PKR0_FPOSG_EQ1                   = 0x00000006u32,
PC_PERF_PKR0_FPOSG_GT1                   = 0x00000007u32,
PC_PERF_PKR0_FPOSG_GT16                  = 0x00000008u32,
PC_PERF_PKR0_FPOSG_GT64                  = 0x00000009u32,
PC_PERF_PKR0_FPOSG_GT128                 = 0x0000000au32,
PC_PERF_PKR0_FPOSG_OUT_OF_WAVE           = 0x0000000bu32,
PC_PERF_PKR0_NUM_PROBES                  = 0x0000000cu32,
PC_PERF_PKR0_PRIMS_PER_PROBE_EQ1         = 0x0000000du32,
PC_PERF_PKR0_PRIMS_PER_PROBE_GT1         = 0x0000000eu32,
PC_PERF_PKR0_PRIMS_PER_PROBE_GT2         = 0x0000000fu32,
PC_PERF_PKR0_PRIMS_PER_PROBE_GT4         = 0x00000010u32,
PC_PERF_PKR0_PRIMS_PER_PROBE_GT8         = 0x00000011u32,
PC_PERF_PKR0_NUM_WAVES                   = 0x00000012u32,
PC_PERF_PKR0_PRIMS_PER_WAVE_EQ1          = 0x00000013u32,
PC_PERF_PKR0_PRIMS_PER_WAVE_GT1          = 0x00000014u32,
PC_PERF_PKR0_PRIMS_PER_WAVE_GT2          = 0x00000015u32,
PC_PERF_PKR0_PRIMS_PER_WAVE_GT4          = 0x00000016u32,
PC_PERF_PKR0_PRIMS_PER_WAVE_GT8          = 0x00000017u32,
PC_PERF_PKR0_PROBES_PER_WAVE_EQ1         = 0x00000018u32,
PC_PERF_PKR0_PROBES_PER_WAVE_GT1         = 0x00000019u32,
PC_PERF_PKR0_PROBES_PER_WAVE_GT2         = 0x0000001au32,
PC_PERF_PKR0_PROBES_PER_WAVE_GT4         = 0x0000001bu32,
PC_PERF_PKR0_PROBES_PER_WAVE_GT8         = 0x0000001cu32,
PC_PERF_PKR0_PRIMS_REUSE                 = 0x0000001du32,
PC_PERF_SC_PC_PTR_SEND1                  = 0x0000001eu32,
PC_PERF_SC_PC_PTR_VALID1                 = 0x0000001fu32,
PC_PERF_SC_FPOSG1                        = 0x00000020u32,
PC_PERF_SC_FPOSG_WAIT1                   = 0x00000021u32,
PC_PERF_SC_WAIT_SYNC1                    = 0x00000022u32,
PC_PERF_SC_PQ_FREEZE1                    = 0x00000023u32,
PC_PERF_PKR1_FPOSG_EQ1                   = 0x00000024u32,
PC_PERF_PKR1_FPOSG_GT1                   = 0x00000025u32,
PC_PERF_PKR1_FPOSG_GT16                  = 0x00000026u32,
PC_PERF_PKR1_FPOSG_GT64                  = 0x00000027u32,
PC_PERF_PKR1_FPOSG_GT128                 = 0x00000028u32,
PC_PERF_PKR1_FPOSG_OUT_OF_WAVE           = 0x00000029u32,
PC_PERF_PKR1_NUM_PROBES                  = 0x0000002au32,
PC_PERF_PKR1_PRIMS_PER_PROBE_EQ1         = 0x0000002bu32,
PC_PERF_PKR1_PRIMS_PER_PROBE_GT1         = 0x0000002cu32,
PC_PERF_PKR1_PRIMS_PER_PROBE_GT2         = 0x0000002du32,
PC_PERF_PKR1_PRIMS_PER_PROBE_GT4         = 0x0000002eu32,
PC_PERF_PKR1_PRIMS_PER_PROBE_GT8         = 0x0000002fu32,
PC_PERF_PKR1_NUM_WAVES                   = 0x00000030u32,
PC_PERF_PKR1_PRIMS_PER_WAVE_EQ1          = 0x00000031u32,
PC_PERF_PKR1_PRIMS_PER_WAVE_GT1          = 0x00000032u32,
PC_PERF_PKR1_PRIMS_PER_WAVE_GT2          = 0x00000033u32,
PC_PERF_PKR1_PRIMS_PER_WAVE_GT4          = 0x00000034u32,
PC_PERF_PKR1_PRIMS_PER_WAVE_GT8          = 0x00000035u32,
PC_PERF_PKR1_PROBES_PER_WAVE_EQ1         = 0x00000036u32,
PC_PERF_PKR1_PROBES_PER_WAVE_GT1         = 0x00000037u32,
PC_PERF_PKR1_PROBES_PER_WAVE_GT2         = 0x00000038u32,
PC_PERF_PKR1_PROBES_PER_WAVE_GT4         = 0x00000039u32,
PC_PERF_PKR1_PROBES_PER_WAVE_GT8         = 0x0000003au32,
PC_PERF_PKR1_PRIMS_REUSE                 = 0x0000003bu32,
PC_PERF_SC_PC_PTR_SEND2                  = 0x0000003cu32,
PC_PERF_SC_PC_PTR_VALID2                 = 0x0000003du32,
PC_PERF_SC_FPOSG2                        = 0x0000003eu32,
PC_PERF_SC_FPOSG_WAIT2                   = 0x0000003fu32,
PC_PERF_SC_WAIT_SYNC2                    = 0x00000040u32,
PC_PERF_SC_PQ_FREEZE2                    = 0x00000041u32,
PC_PERF_PKR2_FPOSG_EQ1                   = 0x00000042u32,
PC_PERF_PKR2_FPOSG_GT1                   = 0x00000043u32,
PC_PERF_PKR2_FPOSG_GT16                  = 0x00000044u32,
PC_PERF_PKR2_FPOSG_GT64                  = 0x00000045u32,
PC_PERF_PKR2_FPOSG_GT128                 = 0x00000046u32,
PC_PERF_PKR2_FPOSG_OUT_OF_WAVE           = 0x00000047u32,
PC_PERF_PKR2_NUM_PROBES                  = 0x00000048u32,
PC_PERF_PKR2_PRIMS_PER_PROBE_EQ1         = 0x00000049u32,
PC_PERF_PKR2_PRIMS_PER_PROBE_GT1         = 0x0000004au32,
PC_PERF_PKR2_PRIMS_PER_PROBE_GT2         = 0x0000004bu32,
PC_PERF_PKR2_PRIMS_PER_PROBE_GT4         = 0x0000004cu32,
PC_PERF_PKR2_PRIMS_PER_PROBE_GT8         = 0x0000004du32,
PC_PERF_PKR2_NUM_WAVES                   = 0x0000004eu32,
PC_PERF_PKR2_PRIMS_PER_WAVE_EQ1          = 0x0000004fu32,
PC_PERF_PKR2_PRIMS_PER_WAVE_GT1          = 0x00000050u32,
PC_PERF_PKR2_PRIMS_PER_WAVE_GT2          = 0x00000051u32,
PC_PERF_PKR2_PRIMS_PER_WAVE_GT4          = 0x00000052u32,
PC_PERF_PKR2_PRIMS_PER_WAVE_GT8          = 0x00000053u32,
PC_PERF_PKR2_PROBES_PER_WAVE_EQ1         = 0x00000054u32,
PC_PERF_PKR2_PROBES_PER_WAVE_GT1         = 0x00000055u32,
PC_PERF_PKR2_PROBES_PER_WAVE_GT2         = 0x00000056u32,
PC_PERF_PKR2_PROBES_PER_WAVE_GT4         = 0x00000057u32,
PC_PERF_PKR2_PROBES_PER_WAVE_GT8         = 0x00000058u32,
PC_PERF_PKR2_PRIMS_REUSE                 = 0x00000059u32,
PC_PERF_SC_PC_PTR_SEND3                  = 0x0000005au32,
PC_PERF_SC_PC_PTR_VALID3                 = 0x0000005bu32,
PC_PERF_SC_FPOSG3                        = 0x0000005cu32,
PC_PERF_SC_FPOSG_WAIT3                   = 0x0000005du32,
PC_PERF_SC_WAIT_SYNC3                    = 0x0000005eu32,
PC_PERF_SC_PQ_FREEZE3                    = 0x0000005fu32,
PC_PERF_PKR3_FPOSG_EQ1                   = 0x00000060u32,
PC_PERF_PKR3_FPOSG_GT1                   = 0x00000061u32,
PC_PERF_PKR3_FPOSG_GT16                  = 0x00000062u32,
PC_PERF_PKR3_FPOSG_GT64                  = 0x00000063u32,
PC_PERF_PKR3_FPOSG_GT128                 = 0x00000064u32,
PC_PERF_PKR3_FPOSG_OUT_OF_WAVE           = 0x00000065u32,
PC_PERF_PKR3_NUM_PROBES                  = 0x00000066u32,
PC_PERF_PKR3_PRIMS_PER_PROBE_EQ1         = 0x00000067u32,
PC_PERF_PKR3_PRIMS_PER_PROBE_GT1         = 0x00000068u32,
PC_PERF_PKR3_PRIMS_PER_PROBE_GT2         = 0x00000069u32,
PC_PERF_PKR3_PRIMS_PER_PROBE_GT4         = 0x0000006au32,
PC_PERF_PKR3_PRIMS_PER_PROBE_GT8         = 0x0000006bu32,
PC_PERF_PKR3_NUM_WAVES                   = 0x0000006cu32,
PC_PERF_PKR3_PRIMS_PER_WAVE_EQ1          = 0x0000006du32,
PC_PERF_PKR3_PRIMS_PER_WAVE_GT1          = 0x0000006eu32,
PC_PERF_PKR3_PRIMS_PER_WAVE_GT2          = 0x0000006fu32,
PC_PERF_PKR3_PRIMS_PER_WAVE_GT4          = 0x00000070u32,
PC_PERF_PKR3_PRIMS_PER_WAVE_GT8          = 0x00000071u32,
PC_PERF_PKR3_PROBES_PER_WAVE_EQ1         = 0x00000072u32,
PC_PERF_PKR3_PROBES_PER_WAVE_GT1         = 0x00000073u32,
PC_PERF_PKR3_PROBES_PER_WAVE_GT2         = 0x00000074u32,
PC_PERF_PKR3_PROBES_PER_WAVE_GT4         = 0x00000075u32,
PC_PERF_PKR3_PROBES_PER_WAVE_GT8         = 0x00000076u32,
PC_PERF_PKR3_PRIMS_REUSE                 = 0x00000077u32,
PC_PERF_SC_MW_FREEZE                     = 0x00000078u32,
PC_PERF_SC_NUM_PROBES                    = 0x00000079u32,
PC_PERF_SC_NUM_WAVES                     = 0x0000007au32,
PC_PERF_SC_NUM_SPLIT_WAVES               = 0x0000007bu32,
PC_PERF_GE_GSDONE                        = 0x0000007cu32,
PC_PERF_PKR0_GSDONE_WHILE_IDLE           = 0x0000007du32,
PC_PERF_PKR1_GSDONE_WHILE_IDLE           = 0x0000007eu32,
PC_PERF_PKR2_GSDONE_WHILE_IDLE           = 0x0000007fu32,
PC_PERF_PKR3_GSDONE_WHILE_IDLE           = 0x00000080u32,
PC_PERF_PC_SPI_PROBE_FREEZE              = 0x00000081u32,
PC_PERF_PC_SPI_PROBE_OUT_OF_CREDIT       = 0x00000082u32,
PC_PERF_MW_RTN_ADDR_FREEZE               = 0x00000083u32,
PC_PERF_MW_PROBE_CNT_FREEZE              = 0x00000084u32,
PC_PERF_MW_GL1H_REQ_FREEZE               = 0x00000085u32,
PC_PERF_MW_GL1H_NUM_REQS                 = 0x00000086u32,
PC_PERF_MW_DLINE_ALLOC                   = 0x00000087u32,
PC_PERF_MW_DLINE_DEALLOC                 = 0x00000088u32,
PC_PERF_MW_TAGLINE_ALLOC                 = 0x00000089u32,
PC_PERF_MW_TAGLINE_DEALLOC               = 0x0000008au32,
PC_PERF_MW_PHY_DLINE_FULL_STALL          = 0x0000008bu32,
PC_PERF_MW_CACHE_CNTL_FULL_STALL         = 0x0000008cu32,
PC_PERF_MW_STAMP_LIMIT_STALL             = 0x0000008du32,
PC_PERF_MW_CACHE_MISS                    = 0x0000008eu32,
PC_PERF_MW_CACHE_HIT                     = 0x0000008fu32,
PC_PERF_MW_CACHE_REUSE                   = 0x00000090u32,
PC_PERF_MW_DEALLOC_HIT                   = 0x00000091u32,
PC_PERF_PC_MEM_BANK_CONF0                = 0x00000092u32,
PC_PERF_PC_MEM_BANK_CONF1                = 0x00000093u32,
PC_PERF_PC_LDS_VERTEX_REUSE0             = 0x00000094u32,
PC_PERF_PC_LDS_CNTL_VALID0               = 0x00000095u32,
PC_PERF_PC_LDS_VERTEX_REUSE1             = 0x00000096u32,
PC_PERF_PC_LDS_CNTL_VALID1               = 0x00000097u32,
PC_PERF_GRBM_BUSY                        = 0x00000098u32,
PC_PERF_GL1_RTN_CNT_GTE1                 = 0x00000099u32,
PC_PERF_GL1_RTN_CNT_GT512                = 0x0000009au32,
PC_PERF_GL1_RTN_CNT_GT768                = 0x0000009bu32,
PC_PERF_LWC0_PROBE_ORDER_STALL           = 0x0000009cu32,
PC_PERF_LWC0_PC_MEM_READ_STALL           = 0x0000009du32,
PC_PERF_LWC0_PKR2_SA_BDRY_CROSSING       = 0x0000009eu32,
PC_PERF_LWC0_PKR3_SA_BDRY_CROSSING       = 0x0000009fu32,
PC_PERF_LWC1_PROBE_ORDER_STALL           = 0x000000a0u32,
PC_PERF_LWC1_PC_MEM_READ_STALL           = 0x000000a1u32,
PC_PERF_LWC1_PKR0_SA_BDRY_CROSSING       = 0x000000a2u32,
PC_PERF_LWC1_PKR1_SA_BDRY_CROSSING       = 0x000000a3u32,
PC_PERF_NUM_PSWAVE                       = 0x000000a4u32,
}

/*
 * SPI_FOG_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SPI_FOG_MODE {
SPI_FOG_NONE                             = 0x00000000u32,
SPI_FOG_EXP                              = 0x00000001u32,
SPI_FOG_EXP2                             = 0x00000002u32,
SPI_FOG_LINEAR                           = 0x00000003u32,
}

/*
 * SPI_LB_WAVES_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SPI_LB_WAVES_SELECT {
HS_GS                                    = 0x00000000u32,
PS                                       = 0x00000001u32,
CS_NA                                    = 0x00000002u32,
SPI_LB_WAVES_RSVD                        = 0x00000003u32,
}

/*
 * SPI_PERFCNT_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SPI_PERFCNT_SEL {
SPI_PERF_GS_WINDOW_VALID                 = 0x00000001u32,
SPI_PERF_GS_BUSY                         = 0x00000002u32,
SPI_PERF_GS_CRAWLER_STALL                = 0x00000003u32,
SPI_PERF_GS_EVENT_WAVE                   = 0x00000004u32,
SPI_PERF_GS_WAVE                         = 0x00000005u32,
SPI_PERF_GS_PERS_UPD_FULL0               = 0x00000006u32,
SPI_PERF_GS_PERS_UPD_FULL1               = 0x00000007u32,
SPI_PERF_GS_FIRST_SUBGRP                 = 0x00000008u32,
SPI_PERF_GS_HS_DEALLOC                   = 0x00000009u32,
SPI_PERF_GS_NGG_SE_LATE_ALLOC_LIMIT      = 0x0000000au32,
SPI_PERF_GS_POS0_STALL                   = 0x0000000bu32,
SPI_PERF_GS_POS1_STALL                   = 0x0000000cu32,
SPI_PERF_GS_INDX0_STALL                  = 0x0000000du32,
SPI_PERF_GS_INDX1_STALL                  = 0x0000000eu32,
SPI_PERF_GS_PWS_STALL                    = 0x0000000fu32,
SPI_PERF_GS_GRP_LIFETIME                 = 0x00000010u32,
SPI_PERF_GS_WAVE_IN_FLIGHT               = 0x00000011u32,
SPI_PERF_GS_GRP_LIFETIME_SAMPLE          = 0x00000012u32,
SPI_PERF_HS_WINDOW_VALID                 = 0x00000015u32,
SPI_PERF_HS_BUSY                         = 0x00000016u32,
SPI_PERF_HS_CRAWLER_STALL                = 0x00000017u32,
SPI_PERF_HS_FIRST_WAVE                   = 0x00000018u32,
SPI_PERF_HS_EVENT_WAVE                   = 0x0000001au32,
SPI_PERF_HS_WAVE                         = 0x0000001bu32,
SPI_PERF_HS_PERS_UPD_FULL0               = 0x0000001cu32,
SPI_PERF_HS_PERS_UPD_FULL1               = 0x0000001du32,
SPI_PERF_HS_PWS_STALL                    = 0x0000001eu32,
SPI_PERF_HS_WAVE_IN_FLIGHT               = 0x0000001fu32,
SPI_PERF_CSGN_WINDOW_VALID               = 0x00000025u32,
SPI_PERF_CSGN_BUSY                       = 0x00000026u32,
SPI_PERF_CSGN_NUM_THREADGROUPS           = 0x00000027u32,
SPI_PERF_CSGN_CRAWLER_STALL              = 0x00000028u32,
SPI_PERF_CSGN_EVENT_WAVE                 = 0x00000029u32,
SPI_PERF_CSGN_WAVE                       = 0x0000002au32,
SPI_PERF_CSGN_PWS_STALL                  = 0x0000002bu32,
SPI_PERF_CSGN_WAVE_IN_FLIGHT             = 0x0000002cu32,
SPI_PERF_CSN_WINDOW_VALID                = 0x0000002du32,
SPI_PERF_CSN_BUSY                        = 0x0000002eu32,
SPI_PERF_CSN_NUM_THREADGROUPS            = 0x0000002fu32,
SPI_PERF_CSN_CRAWLER_STALL               = 0x00000030u32,
SPI_PERF_CSN_EVENT_WAVE                  = 0x00000031u32,
SPI_PERF_CSN_WAVE                        = 0x00000032u32,
SPI_PERF_CSN_WAVE_IN_FLIGHT              = 0x00000033u32,
SPI_PERF_PS0_WINDOW_VALID                = 0x00000035u32,
SPI_PERF_PS1_WINDOW_VALID                = 0x00000036u32,
SPI_PERF_PS2_WINDOW_VALID                = 0x00000037u32,
SPI_PERF_PS3_WINDOW_VALID                = 0x00000038u32,
SPI_PERF_PS0_BUSY                        = 0x00000039u32,
SPI_PERF_PS1_BUSY                        = 0x0000003au32,
SPI_PERF_PS2_BUSY                        = 0x0000003bu32,
SPI_PERF_PS3_BUSY                        = 0x0000003cu32,
SPI_PERF_PS0_ACTIVE                      = 0x0000003du32,
SPI_PERF_PS1_ACTIVE                      = 0x0000003eu32,
SPI_PERF_PS2_ACTIVE                      = 0x0000003fu32,
SPI_PERF_PS3_ACTIVE                      = 0x00000040u32,
SPI_PERF_PS0_DEALLOC                     = 0x00000041u32,
SPI_PERF_PS1_DEALLOC                     = 0x00000042u32,
SPI_PERF_PS2_DEALLOC                     = 0x00000043u32,
SPI_PERF_PS3_DEALLOC                     = 0x00000044u32,
SPI_PERF_PS0_EVENT_WAVE                  = 0x00000045u32,
SPI_PERF_PS1_EVENT_WAVE                  = 0x00000046u32,
SPI_PERF_PS2_EVENT_WAVE                  = 0x00000047u32,
SPI_PERF_PS3_EVENT_WAVE                  = 0x00000048u32,
SPI_PERF_PS0_WAVE                        = 0x00000049u32,
SPI_PERF_PS1_WAVE                        = 0x0000004au32,
SPI_PERF_PS2_WAVE                        = 0x0000004bu32,
SPI_PERF_PS3_WAVE                        = 0x0000004cu32,
SPI_PERF_PS0_OPT_WAVE                    = 0x0000004du32,
SPI_PERF_PS1_OPT_WAVE                    = 0x0000004eu32,
SPI_PERF_PS2_OPT_WAVE                    = 0x0000004fu32,
SPI_PERF_PS3_OPT_WAVE                    = 0x00000050u32,
SPI_PERF_PS0_PRIM_BIN0                   = 0x00000051u32,
SPI_PERF_PS1_PRIM_BIN0                   = 0x00000052u32,
SPI_PERF_PS2_PRIM_BIN0                   = 0x00000053u32,
SPI_PERF_PS3_PRIM_BIN0                   = 0x00000054u32,
SPI_PERF_PS0_PRIM_BIN1                   = 0x00000055u32,
SPI_PERF_PS1_PRIM_BIN1                   = 0x00000056u32,
SPI_PERF_PS2_PRIM_BIN1                   = 0x00000057u32,
SPI_PERF_PS3_PRIM_BIN1                   = 0x00000058u32,
SPI_PERF_PS0_CRAWLER_STALL               = 0x00000059u32,
SPI_PERF_PS1_CRAWLER_STALL               = 0x0000005au32,
SPI_PERF_PS2_CRAWLER_STALL               = 0x0000005bu32,
SPI_PERF_PS3_CRAWLER_STALL               = 0x0000005cu32,
SPI_PERF_PS_PERS_UPD_FULL0               = 0x0000005du32,
SPI_PERF_PS_PERS_UPD_FULL1               = 0x0000005eu32,
SPI_PERF_PS0_2_WAVE_GROUPS               = 0x0000005fu32,
SPI_PERF_PS1_2_WAVE_GROUPS               = 0x00000060u32,
SPI_PERF_PS2_2_WAVE_GROUPS               = 0x00000061u32,
SPI_PERF_PS3_2_WAVE_GROUPS               = 0x00000062u32,
SPI_PERF_PS0_WAVE_GROUP_CLOCK_DELAY      = 0x00000063u32,
SPI_PERF_PS1_WAVE_GROUP_CLOCK_DELAY      = 0x00000064u32,
SPI_PERF_PS2_WAVE_GROUP_CLOCK_DELAY      = 0x00000065u32,
SPI_PERF_PS3_WAVE_GROUP_CLOCK_DELAY      = 0x00000066u32,
SPI_PERF_PS0_WAVE_GROUP_TIMEOUTS         = 0x00000067u32,
SPI_PERF_PS1_WAVE_GROUP_TIMEOUTS         = 0x00000068u32,
SPI_PERF_PS2_WAVE_GROUP_TIMEOUTS         = 0x00000069u32,
SPI_PERF_PS3_WAVE_GROUP_TIMEOUTS         = 0x0000006au32,
SPI_PERF_PS_PWS_STALL                    = 0x0000006bu32,
SPI_PERF_PS0_LDS_DONE_FULL               = 0x0000006cu32,
SPI_PERF_PS1_LDS_DONE_FULL               = 0x0000006du32,
SPI_PERF_PS2_LDS_DONE_FULL               = 0x0000006eu32,
SPI_PERF_PS3_LDS_DONE_FULL               = 0x0000006fu32,
SPI_PERF_PS0_DEALLOC_FULL                = 0x00000070u32,
SPI_PERF_PS1_DEALLOC_FULL                = 0x00000071u32,
SPI_PERF_PS2_DEALLOC_FULL                = 0x00000072u32,
SPI_PERF_PS3_DEALLOC_FULL                = 0x00000073u32,
SPI_PERF_PS0_WAVE_IN_FLIGHT              = 0x00000074u32,
SPI_PERF_PS1_WAVE_IN_FLIGHT              = 0x00000075u32,
SPI_PERF_PS2_WAVE_IN_FLIGHT              = 0x00000076u32,
SPI_PERF_PS3_WAVE_IN_FLIGHT              = 0x00000077u32,
SPI_PERF_RA_GS_LDS_OCCUPANCY             = 0x00000085u32,
SPI_PERF_RA_GS_VGPR_OCCUPANCY            = 0x00000086u32,
SPI_PERF_RA_PS_LDS_OCCUPANCY             = 0x00000087u32,
SPI_PERF_RA_PS_VGPR_OCCUPANCY            = 0x00000088u32,
SPI_PERF_RA_SPI_THROTTLE                 = 0x00000089u32,
SPI_PERF_RA_PH_THROTTLE                  = 0x0000008au32,
SPI_PERF_RA_PC_PROBE_STALL_PS            = 0x0000008bu32,
SPI_PERF_RA_PC_PSWAVE_STALL_PS           = 0x0000008cu32,
SPI_PERF_RA_PIPE_REQ_BIN2                = 0x0000008du32,
SPI_PERF_RA_TASK_REQ_BIN3                = 0x0000008eu32,
SPI_PERF_RA_WR_CTL_FULL                  = 0x0000008fu32,
SPI_PERF_RA_REQ_NO_ALLOC                 = 0x00000090u32,
SPI_PERF_RA_REQ_NO_ALLOC_PS              = 0x00000091u32,
SPI_PERF_RA_REQ_NO_ALLOC_GS              = 0x00000092u32,
SPI_PERF_RA_REQ_NO_ALLOC_HS              = 0x00000093u32,
SPI_PERF_RA_REQ_NO_ALLOC_CSG             = 0x00000094u32,
SPI_PERF_RA_REQ_NO_ALLOC_CSN             = 0x00000095u32,
SPI_PERF_RA_RES_STALL_PS                 = 0x00000096u32,
SPI_PERF_RA_RES_STALL_GS                 = 0x00000097u32,
SPI_PERF_RA_RES_STALL_HS                 = 0x00000098u32,
SPI_PERF_RA_RES_STALL_CSG                = 0x00000099u32,
SPI_PERF_RA_RES_STALL_CSN                = 0x0000009au32,
SPI_PERF_RA_TMP_STALL_PS                 = 0x0000009bu32,
SPI_PERF_RA_TMP_STALL_GS                 = 0x0000009cu32,
SPI_PERF_RA_TMP_STALL_HS                 = 0x0000009du32,
SPI_PERF_RA_TMP_STALL_CSG                = 0x0000009eu32,
SPI_PERF_RA_TMP_STALL_CSN                = 0x0000009fu32,
SPI_PERF_RA_WAVE_SIMD_FULL_PS            = 0x000000a0u32,
SPI_PERF_RA_WAVE_SIMD_FULL_GS            = 0x000000a1u32,
SPI_PERF_RA_WAVE_SIMD_FULL_HS            = 0x000000a2u32,
SPI_PERF_RA_WAVE_SIMD_FULL_CSG           = 0x000000a3u32,
SPI_PERF_RA_WAVE_SIMD_FULL_CSN           = 0x000000a4u32,
SPI_PERF_RA_VGPR_SIMD_FULL_PS            = 0x000000a5u32,
SPI_PERF_RA_VGPR_SIMD_FULL_GS            = 0x000000a6u32,
SPI_PERF_RA_VGPR_SIMD_FULL_HS            = 0x000000a7u32,
SPI_PERF_RA_VGPR_SIMD_FULL_CSG           = 0x000000a8u32,
SPI_PERF_RA_VGPR_SIMD_FULL_CSN           = 0x000000a9u32,
SPI_PERF_RA_LDS_CU_FULL_PS               = 0x000000aau32,
SPI_PERF_RA_LDS_CU_FULL_HS               = 0x000000abu32,
SPI_PERF_RA_LDS_CU_FULL_GS               = 0x000000acu32,
SPI_PERF_RA_LDS_CU_FULL_CSG              = 0x000000adu32,
SPI_PERF_RA_LDS_CU_FULL_CSN              = 0x000000aeu32,
SPI_PERF_RA_BAR_CU_FULL_PS               = 0x000000afu32,
SPI_PERF_RA_BAR_CU_FULL_GS               = 0x000000b0u32,
SPI_PERF_RA_BAR_CU_FULL_HS               = 0x000000b1u32,
SPI_PERF_RA_BAR_CU_FULL_CSG              = 0x000000b2u32,
SPI_PERF_RA_BAR_CU_FULL_CSN              = 0x000000b3u32,
SPI_PERF_RA_BULKY_CU_FULL_CSG            = 0x000000b4u32,
SPI_PERF_RA_BULKY_CU_FULL_CSN            = 0x000000b5u32,
SPI_PERF_RA_TGLIM_CU_FULL_CSG            = 0x000000b6u32,
SPI_PERF_RA_TGLIM_CU_FULL_CSN            = 0x000000b7u32,
SPI_PERF_RA_WVLIM_STALL_PS               = 0x000000b8u32,
SPI_PERF_RA_WVLIM_STALL_GS               = 0x000000b9u32,
SPI_PERF_RA_WVLIM_STALL_HS               = 0x000000bau32,
SPI_PERF_RA_WVLIM_STALL_CSG              = 0x000000bbu32,
SPI_PERF_RA_WVLIM_STALL_CSN              = 0x000000bcu32,
SPI_PERF_RA_GS_LOCK                      = 0x000000bdu32,
SPI_PERF_RA_HS_LOCK                      = 0x000000beu32,
SPI_PERF_RA_CSG_LOCK                     = 0x000000bfu32,
SPI_PERF_RA_CSN_LOCK                     = 0x000000c0u32,
SPI_PERF_RA_RSV_UPD                      = 0x000000c1u32,
SPI_PERF_RA_PRE_ALLOC_STALL              = 0x000000c2u32,
SPI_PERF_RA_GFX_UNDER_TUNNEL             = 0x000000c3u32,
SPI_PERF_RA_CSC_UNDER_TUNNEL             = 0x000000c4u32,
SPI_PERF_RA_WVALLOC_STALL                = 0x000000c5u32,
SPI_PERF_RA_ACCUM0_SIMD_FULL_PS          = 0x000000c6u32,
SPI_PERF_RA_ACCUM1_SIMD_FULL_PS          = 0x000000c7u32,
SPI_PERF_RA_ACCUM2_SIMD_FULL_PS          = 0x000000c8u32,
SPI_PERF_RA_ACCUM3_SIMD_FULL_PS          = 0x000000c9u32,
SPI_PERF_RA_ACCUM0_SIMD_FULL_GS          = 0x000000cau32,
SPI_PERF_RA_ACCUM1_SIMD_FULL_GS          = 0x000000cbu32,
SPI_PERF_RA_ACCUM2_SIMD_FULL_GS          = 0x000000ccu32,
SPI_PERF_RA_ACCUM3_SIMD_FULL_GS          = 0x000000cdu32,
SPI_PERF_RA_ACCUM0_SIMD_FULL_HS          = 0x000000ceu32,
SPI_PERF_RA_ACCUM1_SIMD_FULL_HS          = 0x000000cfu32,
SPI_PERF_RA_ACCUM2_SIMD_FULL_HS          = 0x000000d0u32,
SPI_PERF_RA_ACCUM3_SIMD_FULL_HS          = 0x000000d1u32,
SPI_PERF_RA_ACCUM0_SIMD_FULL_CSG         = 0x000000d2u32,
SPI_PERF_RA_ACCUM1_SIMD_FULL_CSG         = 0x000000d3u32,
SPI_PERF_RA_ACCUM2_SIMD_FULL_CSG         = 0x000000d4u32,
SPI_PERF_RA_ACCUM3_SIMD_FULL_CSG         = 0x000000d5u32,
SPI_PERF_RA_ACCUM0_SIMD_FULL_CSN         = 0x000000d6u32,
SPI_PERF_RA_ACCUM1_SIMD_FULL_CSN         = 0x000000d7u32,
SPI_PERF_RA_ACCUM2_SIMD_FULL_CSN         = 0x000000d8u32,
SPI_PERF_RA_ACCUM3_SIMD_FULL_CSN         = 0x000000d9u32,
SPI_PERF_EXP_ARB_COL_CNT                 = 0x000000dau32,
SPI_PERF_EXP_ARB_POS_CNT                 = 0x000000dbu32,
SPI_PERF_EXP_ARB_GDS_CNT                 = 0x000000dcu32,
SPI_PERF_EXP_ARB_IDX_CNT                 = 0x000000ddu32,
SPI_PERF_EXP_WITH_CONFLICT               = 0x000000deu32,
SPI_PERF_EXP_WITH_CONFLICT_CLEAR         = 0x000000dfu32,
SPI_PERF_GS_EXP_DONE                     = 0x000000e0u32,
SPI_PERF_PS_EXP_DONE                     = 0x000000e1u32,
SPI_PERF_PS_EXP_ARB_CONFLICT             = 0x000000e2u32,
SPI_PERF_GS_SCBD_IDX_CLEANUP             = 0x000000e3u32,
SPI_PERF_GS_SCBD_POS_CLEANUP             = 0x000000e4u32,
SPI_PERF_PS_EXP_ALLOC                    = 0x000000e5u32,
SPI_PERF_PS0_WAVEID_STARVED              = 0x000000e6u32,
SPI_PERF_PS1_WAVEID_STARVED              = 0x000000e7u32,
SPI_PERF_PS2_WAVEID_STARVED              = 0x000000e8u32,
SPI_PERF_PS3_WAVEID_STARVED              = 0x000000e9u32,
SPI_PERF_PS0_EXP_ALLOC_WITH_CONFLICT     = 0x000000eau32,
SPI_PERF_PS1_EXP_ALLOC_WITH_CONFLICT     = 0x000000ebu32,
SPI_PERF_PS2_EXP_ALLOC_WITH_CONFLICT     = 0x000000ecu32,
SPI_PERF_PS3_EXP_ALLOC_WITH_CONFLICT     = 0x000000edu32,
SPI_PERF_NUM_PS_COL_SA0SQ0_EXPORTS       = 0x000000eeu32,
SPI_PERF_NUM_PS_COL_SA0SQ1_EXPORTS       = 0x000000efu32,
SPI_PERF_NUM_PS_COL_SA1SQ0_EXPORTS       = 0x000000f0u32,
SPI_PERF_NUM_PS_COL_SA1SQ1_EXPORTS       = 0x000000f1u32,
SPI_PERF_NUM_POS_SA0SQ0_EXPORTS          = 0x000000f2u32,
SPI_PERF_NUM_POS_SA0SQ1_EXPORTS          = 0x000000f3u32,
SPI_PERF_NUM_POS_SA1SQ0_EXPORTS          = 0x000000f4u32,
SPI_PERF_NUM_POS_SA1SQ1_EXPORTS          = 0x000000f5u32,
SPI_PERF_NUM_GDS_SA0SQ0_EXPORTS          = 0x000000f6u32,
SPI_PERF_NUM_GDS_SA0SQ1_EXPORTS          = 0x000000f7u32,
SPI_PERF_NUM_GDS_SA1SQ0_EXPORTS          = 0x000000f8u32,
SPI_PERF_NUM_GDS_SA1SQ1_EXPORTS          = 0x000000f9u32,
SPI_PERF_NUM_EXPGRANT_EXPORTS            = 0x000000fau32,
SPI_PERF_GS_ALLOC_IDX                    = 0x000000fbu32,
SPI_PERF_GS_ALLOC_POS                    = 0x000000fcu32,
SPI_PERF_PIX_ALLOC_PEND_CNT              = 0x000000fdu32,
SPI_PERF_EXPORT_SCB0_STALL               = 0x000000feu32,
SPI_PERF_EXPORT_SCB1_STALL               = 0x000000ffu32,
SPI_PERF_EXPORT_SCB2_STALL               = 0x00000100u32,
SPI_PERF_EXPORT_SCB3_STALL               = 0x00000101u32,
SPI_PERF_EXPORT_DB0_STALL                = 0x00000102u32,
SPI_PERF_EXPORT_DB1_STALL                = 0x00000103u32,
SPI_PERF_EXPORT_DB2_STALL                = 0x00000104u32,
SPI_PERF_EXPORT_DB3_STALL                = 0x00000105u32,
SPI_PERF_EXPORT_DB4_STALL                = 0x00000106u32,
SPI_PERF_EXPORT_DB5_STALL                = 0x00000107u32,
SPI_PERF_EXPORT_DB6_STALL                = 0x00000108u32,
SPI_PERF_EXPORT_DB7_STALL                = 0x00000109u32,
SPI_PERF_GS_NGG_SE_SEND_GS_ALLOC         = 0x0000010au32,
SPI_PERF_GS_NGG_STALL_MSG_VAL            = 0x0000010bu32,
SPI_PERF_SWC_PS_WR                       = 0x0000010cu32,
SPI_PERF_SWC_GS_WR                       = 0x0000010du32,
SPI_PERF_SWC_HS_WR                       = 0x0000010eu32,
SPI_PERF_SWC_CSGN_WR                     = 0x0000010fu32,
SPI_PERF_SWC_CSN_WR                      = 0x00000110u32,
SPI_PERF_VWC_PS_WR                       = 0x00000111u32,
SPI_PERF_VWC_ES_WR                       = 0x00000112u32,
SPI_PERF_VWC_GS_WR                       = 0x00000113u32,
SPI_PERF_VWC_LS_WR                       = 0x00000114u32,
SPI_PERF_VWC_HS_WR                       = 0x00000115u32,
SPI_PERF_VWC_CSGN_WR                     = 0x00000116u32,
SPI_PERF_VWC_CSN_WR                      = 0x00000117u32,
SPI_PERF_EXP_THROT_UPSTEP                = 0x00000118u32,
SPI_PERF_EXP_THROT_DOWNSTEP              = 0x00000119u32,
SPI_PERF_EXP_THROT_CAUSALITY_DETECTED    = 0x0000011au32,
SPI_PERF_BUSY                            = 0x0000011bu32,
SPI_PERF_ALL_PS_WAVE                     = 0x0000011cu32,
SPI_PERF_ALL_PS_WAVE_IN_FLIGHT           = 0x0000011du32,
SPI_PERF_ALL_WAVE                        = 0x0000011eu32,
SPI_PERF_ALL_WAVE_IN_FLIGHT              = 0x0000011fu32,
SPI_PERF_RA_REQ_ALLOC                    = 0x00000120u32,
SPI_PERF_VGPR_INIT                       = 0x00000121u32,
SPI_PERF_SGPR_INIT                       = 0x00000122u32,
SPI_PERF_VGPR_ALLOC_LEVEL                = 0x00000123u32,
SPI_PERF_LDS_ALLOC_LEVEL                 = 0x00000124u32,
SPI_PERF_GFX_TEMP_ALLOC_LEVEL            = 0x00000125u32,
SPI_PERF_CSG_TEMP_ALLOC_LEVEL            = 0x00000126u32,
SPI_PERF_CSN_TEMP_ALLOC_LEVEL            = 0x00000127u32,
SPI_PERF_ALL_WAVE_RESTORED               = 0x00000128u32,
SPI_PERF_ALL_WAVE_SAVED                  = 0x00000129u32,
SPI_PERF_ALL_WAVE_W32                    = 0x0000012au32,
SPI_PERF_ALL_WAVE_W64                    = 0x0000012bu32,
SPI_PERF_ALL_WAVE_ITEMS                  = 0x0000012cu32,
SPI_PERF_ALL_WAVE_ITEMS_W32              = 0x0000012du32,
SPI_PERF_ALL_WAVE_ITEMS_W64              = 0x0000012eu32,
SPI_PERF_RA_REQ_ALLOC_WGP_TAKEOVER_STALL = 0x0000012fu32,
SPI_PERF_RA_REQ_ALLOC_WGP_TAKEOVER_LEVEL = 0x00000130u32,
SPI_PERF_RA_REQ_ALLOC_DYN_VGPR_STALL     = 0x00000131u32,
SPI_PERF_RA_REQ_ALLOC_DYN_VGPR_CU_LEVEL  = 0x00000132u32,
}

/*
 * SPI_PNT_SPRITE_OVERRIDE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SPI_PNT_SPRITE_OVERRIDE {
SPI_PNT_SPRITE_SEL_0                     = 0x00000000u32,
SPI_PNT_SPRITE_SEL_1                     = 0x00000001u32,
SPI_PNT_SPRITE_SEL_S                     = 0x00000002u32,
SPI_PNT_SPRITE_SEL_T                     = 0x00000003u32,
SPI_PNT_SPRITE_SEL_NONE                  = 0x00000004u32,
}

/*
 * SPI_PS_LDS_GROUP_SIZE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SPI_PS_LDS_GROUP_SIZE {
SPI_PS_LDS_GROUP_1                       = 0x00000000u32,
SPI_PS_LDS_GROUP_2                       = 0x00000001u32,
SPI_PS_LDS_GROUP_4                       = 0x00000002u32,
}

/*
 * SPI_SAMPLE_CNTL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SPI_SAMPLE_CNTL {
CENTROIDS_ONLY                           = 0x00000000u32,
CENTERS_ONLY                             = 0x00000001u32,
CENTROIDS_AND_CENTERS                    = 0x00000002u32,
UNDEF                                    = 0x00000003u32,
}

/*
 * SPI_SHADER_EX_FORMAT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SPI_SHADER_EX_FORMAT {
SPI_SHADER_ZERO                          = 0x00000000u32,
SPI_SHADER_32_R                          = 0x00000001u32,
SPI_SHADER_32_GR                         = 0x00000002u32,
SPI_SHADER_32_AR                         = 0x00000003u32,
SPI_SHADER_FP16_ABGR                     = 0x00000004u32,
SPI_SHADER_UNORM16_ABGR                  = 0x00000005u32,
SPI_SHADER_SNORM16_ABGR                  = 0x00000006u32,
SPI_SHADER_UINT16_ABGR                   = 0x00000007u32,
SPI_SHADER_SINT16_ABGR                   = 0x00000008u32,
SPI_SHADER_32_ABGR                       = 0x00000009u32,
}

/*
 * SPI_SHADER_FORMAT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SPI_SHADER_FORMAT {
SPI_SHADER_NONE                          = 0x00000000u32,
SPI_SHADER_1COMP                         = 0x00000001u32,
SPI_SHADER_2COMP                         = 0x00000002u32,
SPI_SHADER_4COMPRESS                     = 0x00000003u32,
SPI_SHADER_4COMP                         = 0x00000004u32,
}

/*******************************************************
 * SQ Enums
 *******************************************************/

/*
 * SH_MEM_ADDRESS_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SH_MEM_ADDRESS_MODE {
SH_MEM_ADDRESS_MODE_64                   = 0x00000000u32,
SH_MEM_ADDRESS_MODE_32                   = 0x00000001u32,
}

/*
 * SH_MEM_ALIGNMENT_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SH_MEM_ALIGNMENT_MODE {
SH_MEM_ALIGNMENT_MODE_DWORD              = 0x00000000u32,
SH_MEM_ALIGNMENT_MODE_DWORD_STRICT       = 0x00000001u32,
SH_MEM_ALIGNMENT_MODE_STRICT             = 0x00000002u32,
SH_MEM_ALIGNMENT_MODE_UNALIGNED          = 0x00000003u32,
}

/*
 * SQG_PERF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQG_PERF_SEL {
SQG_PERF_SEL_NONE                        = 0x00000000u32,
SQG_PERF_SEL_MSG_BUS_BUSY                = 0x00000001u32,
SQG_PERF_SEL_EXP_REQ0_BUS_BUSY           = 0x00000002u32,
SQG_PERF_SEL_EXP_REQ1_BUS_BUSY           = 0x00000003u32,
SQG_PERF_SEL_EXP_BUS0_BUSY               = 0x00000004u32,
SQG_PERF_SEL_EXP_BUS1_BUSY               = 0x00000005u32,
SQG_PERF_SEL_TTRACE_WRITE_DATA           = 0x00000006u32,
SQG_PERF_SEL_TTRACE_STALL                = 0x00000007u32,
SQG_PERF_SEL_TTRACE_LOST_PACKETS         = 0x00000008u32,
SQG_PERF_SEL_WAVES_INITIAL_PREFETCH      = 0x00000009u32,
SQG_PERF_SEL_EVENTS                      = 0x0000000au32,
SQG_PERF_SEL_WAVES_RESTORED              = 0x0000000bu32,
SQG_PERF_SEL_WAVES_SAVED                 = 0x0000000cu32,
SQG_PERF_SEL_ACCUM_PREV                  = 0x0000000du32,
SQG_PERF_SEL_CYCLES                      = 0x0000000eu32,
SQG_PERF_SEL_BUSY_CYCLES                 = 0x0000000fu32,
SQG_PERF_SEL_WAVE_CYCLES                 = 0x00000010u32,
SQG_PERF_SEL_MSG                         = 0x00000011u32,
SQG_PERF_SEL_MSG_INTERRUPT               = 0x00000012u32,
SQG_PERF_SEL_WAVES                       = 0x00000013u32,
SQG_PERF_SEL_WAVES_32                    = 0x00000014u32,
SQG_PERF_SEL_WAVES_64                    = 0x00000015u32,
SQG_PERF_SEL_LEVEL_WAVES                 = 0x00000016u32,
SQG_PERF_SEL_ITEMS                       = 0x00000017u32,
SQG_PERF_SEL_WAVE32_ITEMS                = 0x00000018u32,
SQG_PERF_SEL_WAVE64_ITEMS                = 0x00000019u32,
SQG_PERF_SEL_PS_QUADS                    = 0x0000001au32,
SQG_PERF_SEL_WAVES_EQ_64                 = 0x0000001bu32,
SQG_PERF_SEL_WAVES_EQ_32                 = 0x0000001cu32,
SQG_PERF_SEL_WAVES_LT_64                 = 0x0000001du32,
SQG_PERF_SEL_WAVES_LT_48                 = 0x0000001eu32,
SQG_PERF_SEL_WAVES_LT_32                 = 0x0000001fu32,
SQG_PERF_SEL_WAVES_LT_16                 = 0x00000020u32,
SQG_PERF_SEL_REFCLKS                     = 0x00000021u32,
SQG_PERF_SEL_WAVES_WGP_TAKEOVER          = 0x00000022u32,
SQG_PERF_SEL_WAVES_DYN_VGPR              = 0x00000023u32,
SQG_PERF_SEL_ITEMS_PS                    = 0x00000024u32,
SQG_PERF_SEL_ITEMS_GS                    = 0x00000025u32,
SQG_PERF_SEL_ITEMS_HS                    = 0x00000026u32,
SQG_PERF_SEL_ITEMS_CS                    = 0x00000027u32,
SQG_PERF_SEL_WAVES_VEC32                 = 0x00000028u32,
SQG_PERF_SEL_WAVES_PS_VEC32              = 0x00000029u32,
SQG_PERF_SEL_WAVES_GS_VEC32              = 0x0000002au32,
SQG_PERF_SEL_WAVES_HS_VEC32              = 0x0000002bu32,
SQG_PERF_SEL_WAVES_CS_VEC32              = 0x0000002cu32,
SQG_PERF_SEL_LEVEL_WGP_ACTIVE            = 0x0000002du32,
SQG_PERF_SEL_DUMMY_LAST                  = 0x0000002eu32,
}

/*
 * SQ_CAC_POWER_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_CAC_POWER_SEL {
SQ_CAC_POWER_VALU                        = 0x00000000u32,
SQ_CAC_POWER_VALU0                       = 0x00000001u32,
SQ_CAC_POWER_VALU1                       = 0x00000002u32,
SQ_CAC_POWER_VALU2                       = 0x00000003u32,
SQ_CAC_POWER_GPR_RD                      = 0x00000004u32,
SQ_CAC_POWER_GPR_WR                      = 0x00000005u32,
SQ_CAC_POWER_LDS_BUSY                    = 0x00000006u32,
SQ_CAC_POWER_ALU_BUSY                    = 0x00000007u32,
SQ_CAC_POWER_TEX_BUSY                    = 0x00000008u32,
}

/*
 * SQ_EDC_INFO_SOURCE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_EDC_INFO_SOURCE {
SQ_EDC_INFO_SOURCE_INVALID               = 0x00000000u32,
SQ_EDC_INFO_SOURCE_INST                  = 0x00000001u32,
SQ_EDC_INFO_SOURCE_SGPR                  = 0x00000002u32,
SQ_EDC_INFO_SOURCE_VGPR                  = 0x00000003u32,
SQ_EDC_INFO_SOURCE_LDS                   = 0x00000004u32,
SQ_EDC_INFO_SOURCE_GDS                   = 0x00000005u32,
SQ_EDC_INFO_SOURCE_TA                    = 0x00000006u32,
}

/*
 * SQ_IBUF_ST enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_IBUF_ST {
SQ_IBUF_IB_IDLE                          = 0x00000000u32,
SQ_IBUF_IB_INI_WAIT_GNT                  = 0x00000001u32,
SQ_IBUF_IB_INI_WAIT_DRET                 = 0x00000002u32,
SQ_IBUF_IB_LE_4DW                        = 0x00000003u32,
SQ_IBUF_IB_WAIT_DRET                     = 0x00000004u32,
SQ_IBUF_IB_EMPTY_WAIT_DRET               = 0x00000005u32,
SQ_IBUF_IB_DRET                          = 0x00000006u32,
SQ_IBUF_IB_EMPTY_WAIT_GNT                = 0x00000007u32,
}

/*
 * SQ_IMG_FILTER_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_IMG_FILTER_TYPE {
SQ_IMG_FILTER_MODE_BLEND                 = 0x00000000u32,
SQ_IMG_FILTER_MODE_MIN                   = 0x00000001u32,
SQ_IMG_FILTER_MODE_MAX                   = 0x00000002u32,
}

/*
 * SQ_IND_CMD_CMD enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_IND_CMD_CMD {
SQ_IND_CMD_CMD_NULL                      = 0x00000000u32,
SQ_IND_CMD_CMD_SETHALT                   = 0x00000001u32,
SQ_IND_CMD_CMD_SAVECTX                   = 0x00000002u32,
SQ_IND_CMD_CMD_KILL                      = 0x00000003u32,
SQ_IND_CMD_CMD_TRAP_AFTER_INST           = 0x00000004u32,
SQ_IND_CMD_CMD_TRAP                      = 0x00000005u32,
SQ_IND_CMD_CMD_SET_SYS_PRIO              = 0x00000006u32,
SQ_IND_CMD_CMD_SETFATALHALT              = 0x00000007u32,
SQ_IND_CMD_CMD_SINGLE_STEP               = 0x00000008u32,
}

/*
 * SQ_IND_CMD_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_IND_CMD_MODE {
SQ_IND_CMD_MODE_SINGLE                   = 0x00000000u32,
SQ_IND_CMD_MODE_BROADCAST                = 0x00000001u32,
SQ_IND_CMD_MODE_BROADCAST_QUEUE          = 0x00000002u32,
SQ_IND_CMD_MODE_BROADCAST_PIPE           = 0x00000003u32,
SQ_IND_CMD_MODE_BROADCAST_ME             = 0x00000004u32,
}

/*
 * SQ_INST_STR_ST enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_INST_STR_ST {
SQ_INST_STR_IB_WAVE_NORML                = 0x00000000u32,
SQ_INST_STR_IB_WAVE2ID_NORMAL_INST_AV    = 0x00000001u32,
SQ_INST_STR_IB_WAVE_INTERNAL_INST_AV     = 0x00000002u32,
SQ_INST_STR_IB_WAVE_INST_SKIP_AV         = 0x00000003u32,
SQ_INST_STR_IB_WAVE_NOP_SLEEP_WAIT       = 0x00000004u32,
SQ_INST_STR_IB_WAVE_PC_FROM_SGPR_MSG_WAIT = 0x00000005u32,
}

/*
 * SQ_INST_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_INST_TYPE {
SQ_INST_TYPE_VALU                        = 0x00000000u32,
SQ_INST_TYPE_SCALAR                      = 0x00000001u32,
SQ_INST_TYPE_TEX                         = 0x00000002u32,
SQ_INST_TYPE_LDS                         = 0x00000003u32,
SQ_INST_TYPE_LDS_DIRECT                  = 0x00000004u32,
SQ_INST_TYPE_EXP                         = 0x00000005u32,
SQ_INST_TYPE_MSG                         = 0x00000006u32,
SQ_INST_TYPE_BARRIER                     = 0x00000007u32,
SQ_INST_TYPE_BRANCH_NOT_TAKEN            = 0x00000008u32,
SQ_INST_TYPE_BRANCH_TAKEN                = 0x00000009u32,
SQ_INST_TYPE_JUMP                        = 0x0000000au32,
SQ_INST_TYPE_OTHER                       = 0x0000000bu32,
SQ_INST_TYPE_NONE                        = 0x0000000cu32,
SQ_INST_TYPE_DUAL_VALU                   = 0x0000000du32,
SQ_INST_TYPE_FLAT                        = 0x0000000eu32,
SQ_INST_TYPE_VALU_MATRIX                 = 0x0000000fu32,
}

/*
 * SQ_LLC_CTL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_LLC_CTL {
SQ_LLC_0                                 = 0x00000000u32,
SQ_LLC_1                                 = 0x00000001u32,
SQ_LLC_RSVD_2                            = 0x00000002u32,
SQ_LLC_BYPASS                            = 0x00000003u32,
}

/*
 * SQ_NO_INST_ISSUE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_NO_INST_ISSUE {
SQ_NO_INST_ISSUE_NO_INSTS                = 0x00000000u32,
SQ_NO_INST_ISSUE_ALU_DEP                 = 0x00000001u32,
SQ_NO_INST_ISSUE_S_WAITCNT               = 0x00000002u32,
SQ_NO_INST_ISSUE_NO_ARB_WIN              = 0x00000003u32,
SQ_NO_INST_ISSUE_SLEEP_WAIT              = 0x00000004u32,
SQ_NO_INST_ISSUE_BARRIER_WAIT            = 0x00000005u32,
SQ_NO_INST_ISSUE_OTHER                   = 0x00000006u32,
SQ_NO_INST_ISSUE_INTERNAL                = 0x00000007u32,
}

/*
 * SQ_OOB_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_OOB_SELECT {
SQ_OOB_INDEX_AND_OFFSET                  = 0x00000000u32,
SQ_OOB_INDEX_ONLY                        = 0x00000001u32,
SQ_OOB_NUM_RECORDS_0                     = 0x00000002u32,
SQ_OOB_COMPLETE                          = 0x00000003u32,
}

/*
 * SQ_PERF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_PERF_SEL {
SQ_PERF_SEL_NONE                         = 0x00000000u32,
SQ_PERF_SEL_ACCUM_PREV                   = 0x00000001u32,
SQ_PERF_SEL_CYCLES                       = 0x00000002u32,
SQ_PERF_SEL_BUSY_CYCLES                  = 0x00000003u32,
SQ_PERF_SEL_WAVES                        = 0x00000004u32,
SQ_PERF_SEL_WAVES_32                     = 0x00000005u32,
SQ_PERF_SEL_WAVES_64                     = 0x00000006u32,
SQ_PERF_SEL_LEVEL_WAVES                  = 0x00000007u32,
SQ_PERF_SEL_ITEMS                        = 0x00000008u32,
SQ_PERF_SEL_WAVE32_ITEMS                 = 0x00000009u32,
SQ_PERF_SEL_WAVE64_ITEMS                 = 0x0000000au32,
SQ_PERF_SEL_PS_QUADS                     = 0x0000000bu32,
SQ_PERF_SEL_EVENTS                       = 0x0000000cu32,
SQ_PERF_SEL_WAVES_EQ_32                  = 0x0000000du32,
SQ_PERF_SEL_WAVES_EQ_64                  = 0x0000000eu32,
SQ_PERF_SEL_WAVES_LT_64                  = 0x0000000fu32,
SQ_PERF_SEL_WAVES_LT_48                  = 0x00000010u32,
SQ_PERF_SEL_WAVES_LT_32                  = 0x00000011u32,
SQ_PERF_SEL_WAVES_LT_16                  = 0x00000012u32,
SQ_PERF_SEL_WAVES_RESTORED               = 0x00000013u32,
SQ_PERF_SEL_WAVES_SAVED                  = 0x00000014u32,
SQ_PERF_SEL_MSG                          = 0x00000015u32,
SQ_PERF_SEL_MSG_INTERRUPT                = 0x00000016u32,
SQ_PERF_SEL_WAVES_INITIAL_PREFETCH       = 0x00000017u32,
SQ_PERF_SEL_WAVE_CYCLES                  = 0x00000018u32,
SQ_PERF_SEL_WAVE_READY                   = 0x00000019u32,
SQ_PERF_SEL_WAIT_INST_ANY                = 0x0000001au32,
SQ_PERF_SEL_WAIT_ANY                     = 0x0000001bu32,
SQ_PERF_SEL_WAIT_CNT_ANY                 = 0x0000001cu32,
SQ_PERF_SEL_WAIT_CNT_LOAD                = 0x0000001du32,
SQ_PERF_SEL_WAIT_CNT_STORE               = 0x0000001eu32,
SQ_PERF_SEL_WAIT_TTRACE                  = 0x0000001fu32,
SQ_PERF_SEL_WAIT_IFETCH                  = 0x00000020u32,
SQ_PERF_SEL_WAIT_BARRIER                 = 0x00000021u32,
SQ_PERF_SEL_WAIT_EXP_ALLOC               = 0x00000022u32,
SQ_PERF_SEL_WAIT_SLEEP                   = 0x00000023u32,
SQ_PERF_SEL_WAIT_DELAY_ALU               = 0x00000024u32,
SQ_PERF_SEL_WAIT_DEPCTR                  = 0x00000025u32,
SQ_PERF_SEL_WAIT_OTHER                   = 0x00000026u32,
SQ_PERF_SEL_INSTS_ALL                    = 0x00000027u32,
SQ_PERF_SEL_INSTS_BRANCH                 = 0x00000028u32,
SQ_PERF_SEL_INSTS_CBRANCH_NOT_TAKEN      = 0x00000029u32,
SQ_PERF_SEL_INSTS_CBRANCH_TAKEN          = 0x0000002au32,
SQ_PERF_SEL_INSTS_EXP                    = 0x0000002bu32,
SQ_PERF_SEL_INSTS_FLAT                   = 0x0000002cu32,
SQ_PERF_SEL_INSTS_LDS                    = 0x0000002du32,
SQ_PERF_SEL_INSTS_SALU                   = 0x0000002eu32,
SQ_PERF_SEL_INSTS_SMEM                   = 0x0000002fu32,
SQ_PERF_SEL_INSTS_SMEM_NORM              = 0x00000030u32,
SQ_PERF_SEL_INSTS_SENDMSG                = 0x00000031u32,
SQ_PERF_SEL_INSTS_VALU                   = 0x00000032u32,
SQ_PERF_SEL_INSTS_VALU_TRANS32           = 0x00000033u32,
SQ_PERF_SEL_INSTS_VALU_NO_COEXEC         = 0x00000034u32,
SQ_PERF_SEL_INSTS_TEX                    = 0x00000035u32,
SQ_PERF_SEL_INSTS_TEX_LOAD               = 0x00000036u32,
SQ_PERF_SEL_INSTS_TEX_STORE              = 0x00000037u32,
SQ_PERF_SEL_INSTS_DELAY_ALU              = 0x00000038u32,
SQ_PERF_SEL_INSTS_INTERNAL               = 0x00000039u32,
SQ_PERF_SEL_INSTS_VEC32                  = 0x0000003au32,
SQ_PERF_SEL_INSTS_VEC32_FLAT             = 0x0000003bu32,
SQ_PERF_SEL_INSTS_VEC32_LDS              = 0x0000003cu32,
SQ_PERF_SEL_INSTS_VEC32_VALU             = 0x0000003du32,
SQ_PERF_SEL_VEC32_INSTS_EXP              = 0x0000003eu32,
SQ_PERF_SEL_INSTS_VEC32_VALU_TRANS32     = 0x0000003fu32,
SQ_PERF_SEL_INSTS_VEC32_VALU_NO_COEXEC   = 0x00000040u32,
SQ_PERF_SEL_INSTS_VEC32_TEX              = 0x00000041u32,
SQ_PERF_SEL_INSTS_VEC32_TEX_LOAD         = 0x00000042u32,
SQ_PERF_SEL_INSTS_VEC32_TEX_STORE        = 0x00000043u32,
SQ_PERF_SEL_ITEM_CYCLES_VALU             = 0x00000044u32,
SQ_PERF_SEL_VALU_READWRITELANE_CYCLES    = 0x00000045u32,
SQ_PERF_SEL_WAVE32_INSTS                 = 0x00000046u32,
SQ_PERF_SEL_WAVE64_INSTS                 = 0x00000047u32,
SQ_PERF_SEL_INSTS_VALU_EXEC_SKIPPED      = 0x00000048u32,
SQ_PERF_SEL_WAVE64_HALF_SKIP             = 0x00000049u32,
SQ_PERF_SEL_INST_LEVEL_EXP               = 0x0000004au32,
SQ_PERF_SEL_INST_LEVEL_LDS               = 0x0000004bu32,
SQ_PERF_SEL_INST_LEVEL_SMEM              = 0x0000004cu32,
SQ_PERF_SEL_INST_LEVEL_TEX_LOAD          = 0x0000004du32,
SQ_PERF_SEL_INST_LEVEL_TEX_STORE         = 0x0000004eu32,
SQ_PERF_SEL_IFETCH_REQS                  = 0x0000004fu32,
SQ_PERF_SEL_IFETCH_LEVEL                 = 0x00000050u32,
SQ_PERF_SEL_LDS_DIRECT_CMD_FIFO_FULL_STALL = 0x00000051u32,
SQ_PERF_SEL_VALU_SGATHER_STALL           = 0x00000052u32,
SQ_PERF_SEL_VALU_FWD_BUFFER_FULL_STALL   = 0x00000053u32,
SQ_PERF_SEL_VALU_SGPR_RD_FIFO_FULL_STALL = 0x00000054u32,
SQ_PERF_SEL_VALU_SGATHER_FULL_STALL      = 0x00000055u32,
SQ_PERF_SEL_SALU_SGATHER_STALL           = 0x00000056u32,
SQ_PERF_SEL_SALU_SGPR_RD_FIFO_FULL_STALL = 0x00000057u32,
SQ_PERF_SEL_SALU_GATHER_FULL_STALL       = 0x00000058u32,
SQ_PERF_SEL_INST_ISSUE_SMEM_STALL        = 0x00000059u32,
SQ_PERF_SEL_INST_ISSUE_ALL_STALL         = 0x0000005au32,
SQ_PERF_SEL_INST_ISSUE_VALU_STALL        = 0x0000005bu32,
SQ_PERF_SEL_INST_ISSUE_SALU_STALL        = 0x0000005cu32,
SQ_PERF_SEL_INST_ISSUE_TEX_STALL         = 0x0000005du32,
SQ_PERF_SEL_INST_ISSUE_LDS_STALL         = 0x0000005eu32,
SQ_PERF_SEL_INST_ISSUE_EXP_STALL         = 0x00000060u32,
SQ_PERF_SEL_INST_WAITCNT_STALL           = 0x00000061u32,
SQ_PERF_SEL_INST_BARRIER_STALL           = 0x00000062u32,
SQ_PERF_SEL_INST_CYCLES_VALU             = 0x00000063u32,
SQ_PERF_SEL_INST_CYCLES_VALU_TRANS32     = 0x00000064u32,
SQ_PERF_SEL_INST_CYCLES_VALU_NO_COEXEC   = 0x00000065u32,
SQ_PERF_SEL_INST_CYCLES_VMEM             = 0x00000066u32,
SQ_PERF_SEL_INST_CYCLES_VMEM_LOAD        = 0x00000067u32,
SQ_PERF_SEL_INST_CYCLES_VMEM_STORE       = 0x00000068u32,
SQ_PERF_SEL_INST_CYCLES_LDS              = 0x00000069u32,
SQ_PERF_SEL_INST_CYCLES_TEX              = 0x0000006au32,
SQ_PERF_SEL_INST_CYCLES_FLAT             = 0x0000006bu32,
SQ_PERF_SEL_INST_CYCLES_EXP              = 0x0000006cu32,
SQ_PERF_SEL_VALU_STARVE                  = 0x0000006du32,
SQ_PERF_SEL_VMEM_ARB_FIFO_FULL           = 0x0000006eu32,
SQ_PERF_SEL_MSG_FIFO_FULL_STALL          = 0x0000006fu32,
SQ_PERF_SEL_EXP_REQ_FIFO_FULL            = 0x00000070u32,
SQ_PERF_SEL_VMEM_BUS_ACTIVE              = 0x00000071u32,
SQ_PERF_SEL_VMEM_BUS_STALL               = 0x00000072u32,
SQ_PERF_SEL_VMEM_BUS_STALL_TA_ADDR_FIFO_FULL = 0x00000073u32,
SQ_PERF_SEL_VMEM_BUS_STALL_TA_CMD_FIFO_FULL = 0x00000074u32,
SQ_PERF_SEL_VMEM_BUS_STALL_LDS_ADDR_FIFO_FULL = 0x00000075u32,
SQ_PERF_SEL_VMEM_BUS_STALL_LDS_CMD_FIFO_FULL = 0x00000076u32,
SQ_PERF_SEL_VMEM_STARVE_TA_ADDR_EMPTY    = 0x00000077u32,
SQ_PERF_SEL_VMEM_STARVE_LDS_ADDR_EMPTY   = 0x00000078u32,
SQ_PERF_SEL_SALU_PIPE_STALL              = 0x00000079u32,
SQ_PERF_SEL_SMEM_DCACHE_RETURN_CYCLES    = 0x0000007au32,
SQ_PERF_SEL_MSG_BUS_BUSY                 = 0x0000007bu32,
SQ_PERF_SEL_EXP_REQ_BUS_STALL            = 0x0000007cu32,
SQ_PERF_SEL_EXP_REQ0_BUS_BUSY            = 0x0000007du32,
SQ_PERF_SEL_EXP_REQ1_BUS_BUSY            = 0x0000007eu32,
SQ_PERF_SEL_EXP_BUS0_BUSY                = 0x0000007fu32,
SQ_PERF_SEL_EXP_BUS1_BUSY                = 0x00000080u32,
SQ_PERF_SEL_INST_CACHE_REQ_STALL         = 0x00000081u32,
SQ_PERF_SEL_USER0                        = 0x00000082u32,
SQ_PERF_SEL_USER1                        = 0x00000083u32,
SQ_PERF_SEL_USER2                        = 0x00000084u32,
SQ_PERF_SEL_USER3                        = 0x00000085u32,
SQ_PERF_SEL_USER4                        = 0x00000086u32,
SQ_PERF_SEL_USER5                        = 0x00000087u32,
SQ_PERF_SEL_USER6                        = 0x00000088u32,
SQ_PERF_SEL_USER7                        = 0x00000089u32,
SQ_PERF_SEL_USER8                        = 0x0000008au32,
SQ_PERF_SEL_USER9                        = 0x0000008bu32,
SQ_PERF_SEL_USER10                       = 0x0000008cu32,
SQ_PERF_SEL_USER11                       = 0x0000008du32,
SQ_PERF_SEL_USER12                       = 0x0000008eu32,
SQ_PERF_SEL_USER13                       = 0x0000008fu32,
SQ_PERF_SEL_USER14                       = 0x00000090u32,
SQ_PERF_SEL_USER15                       = 0x00000091u32,
SQ_PERF_SEL_USER_LEVEL0                  = 0x00000092u32,
SQ_PERF_SEL_USER_LEVEL1                  = 0x00000093u32,
SQ_PERF_SEL_USER_LEVEL2                  = 0x00000094u32,
SQ_PERF_SEL_USER_LEVEL3                  = 0x00000095u32,
SQ_PERF_SEL_USER_LEVEL4                  = 0x00000096u32,
SQ_PERF_SEL_USER_LEVEL5                  = 0x00000097u32,
SQ_PERF_SEL_USER_LEVEL6                  = 0x00000098u32,
SQ_PERF_SEL_USER_LEVEL7                  = 0x00000099u32,
SQ_PERF_SEL_USER_LEVEL8                  = 0x0000009au32,
SQ_PERF_SEL_USER_LEVEL9                  = 0x0000009bu32,
SQ_PERF_SEL_USER_LEVEL10                 = 0x0000009cu32,
SQ_PERF_SEL_USER_LEVEL11                 = 0x0000009du32,
SQ_PERF_SEL_USER_LEVEL12                 = 0x0000009eu32,
SQ_PERF_SEL_USER_LEVEL13                 = 0x0000009fu32,
SQ_PERF_SEL_USER_LEVEL14                 = 0x000000a0u32,
SQ_PERF_SEL_USER_LEVEL15                 = 0x000000a1u32,
SQ_PERF_SEL_VALU_RETURN_SDST             = 0x000000a2u32,
SQ_PERF_SEL_VMEM_VGPR_READ_STALLED_BY_EXPORT = 0x000000a3u32,
SQ_PERF_SEL_INSTS_VALU_TRANS             = 0x000000a4u32,
SQ_PERF_SEL_INSTS_LDS_DIRECT_LOAD        = 0x000000a5u32,
SQ_PERF_SEL_INSTS_LDS_PARAM_LOAD         = 0x000000a6u32,
SQ_PERF_SEL_INSTS_VEC32_LDS_PARAM_LOAD   = 0x000000a7u32,
SQ_PERF_SEL_INSTS_VALU_ONE_CYCLE_WAVE64  = 0x000000a8u32,
SQ_PERF_SEL_INSTS_VALU_VINTERP           = 0x000000a9u32,
SQ_PERF_SEL_INSTS_VEC32_VALU_VINTERP     = 0x000000aau32,
SQ_PERF_SEL_OVERFLOW_PREV                = 0x000000abu32,
SQ_PERF_SEL_INSTS_DUAL_VALU_WAVE32       = 0x000000acu32,
SQ_PERF_SEL_INSTS_VALU_1_PASS            = 0x000000adu32,
SQ_PERF_SEL_INSTS_VALU_2_PASS            = 0x000000aeu32,
SQ_PERF_SEL_INSTS_VALU_4_PASS            = 0x000000afu32,
SQ_PERF_SEL_INSTS_VALU_DP                = 0x000000b0u32,
SQ_PERF_SEL_SP_CONST_CYCLES              = 0x000000b1u32,
SQ_PERF_SEL_SP_CONST_STALL_CYCLES        = 0x000000b2u32,
SQ_PERF_SEL_ITEMS_VALU                   = 0x000000b3u32,
SQ_PERF_SEL_ITEMS_MAX_VALU               = 0x000000b4u32,
SQ_PERF_SEL_ITEM_CYCLES_VMEM             = 0x000000b5u32,
SQ_PERF_SEL_INSTS_DELAY_ALU_COISSUE      = 0x000000b6u32,
SQ_PERF_SEL_INSTS_FLAT_LOAD              = 0x000000b7u32,
SQ_PERF_SEL_INSTS_FLAT_STORE             = 0x000000b8u32,
SQ_PERF_SEL_INSTS_VALU_ONE_CYCLE_WAVE64_16BIT = 0x000000b9u32,
SQ_PERF_SEL_INSTS_VALU_ONE_CYCLE_WAVE64_32BIT = 0x000000bau32,
SQ_PERF_SEL_INSTS_NON_VALU_EXEC_SKIPPED  = 0x000000bbu32,
SQ_PERF_SEL_INSTS_BARRIER_LOCK           = 0x000000bcu32,
SQ_PERF_SEL_INSTS_WAKEUP                 = 0x000000bdu32,
SQ_PERF_SEL_IS_CACHE_REQ                 = 0x000000beu32,
SQ_PERF_SEL_INSTS_SALU_PS                = 0x000000bfu32,
SQ_PERF_SEL_INSTS_SALU_GS                = 0x000000c0u32,
SQ_PERF_SEL_INSTS_SALU_HS                = 0x000000c1u32,
SQ_PERF_SEL_INSTS_SALU_CS                = 0x000000c2u32,
SQ_PERF_SEL_INSTS_SMEM_PS                = 0x000000c3u32,
SQ_PERF_SEL_INSTS_SMEM_GS                = 0x000000c4u32,
SQ_PERF_SEL_INSTS_SMEM_HS                = 0x000000c5u32,
SQ_PERF_SEL_INSTS_SMEM_CS                = 0x000000c6u32,
SQ_PERF_SEL_INSTS_VEC32_TEX_PS           = 0x000000c7u32,
SQ_PERF_SEL_INSTS_VEC32_TEX_GS           = 0x000000c8u32,
SQ_PERF_SEL_INSTS_VEC32_TEX_HS           = 0x000000c9u32,
SQ_PERF_SEL_INSTS_VEC32_TEX_CS           = 0x000000cau32,
SQ_PERF_SEL_INSTS_VEC32_VALU_PS          = 0x000000cbu32,
SQ_PERF_SEL_INSTS_VEC32_VALU_GS          = 0x000000ccu32,
SQ_PERF_SEL_INSTS_VEC32_VALU_HS          = 0x000000cdu32,
SQ_PERF_SEL_INSTS_VEC32_VALU_CS          = 0x000000ceu32,
SQ_PERF_SEL_WAIT_CNT_SAMPLE              = 0x000000cfu32,
SQ_PERF_SEL_WAIT_CNT_KM                  = 0x000000d1u32,
SQ_PERF_SEL_WAIT_CNT_DS                  = 0x000000d2u32,
SQ_PERF_SEL_WAIT_CNT_EXP                 = 0x000000d3u32,
SQ_PERF_SEL_INSTS_SALU_FLOAT             = 0x000000d4u32,
SQ_PERF_SEL_INSTS_VGPR_ALLOC             = 0x000000d5u32,
SQ_PERF_SEL_INSTS_VGPR_ALLOC_FAIL        = 0x000000d6u32,
SQ_PERF_SEL_INSTS_LOCK                   = 0x000000d7u32,
SQ_PERF_SEL_INSTS_VALU_COISSUE           = 0x000000d8u32,
SQ_PERF_SEL_INSTS_VEC32_LEVEL_LDS_LOAD   = 0x000000d9u32,
SQ_PERF_SEL_INSTS_VEC32_LEVEL_LDS_STORE  = 0x000000dau32,
SQ_PERF_SEL_IS_CACHE_MISS                = 0x000000dbu32,
SQ_PERF_SEL_IS_CACHE_DUP_MISS            = 0x000000dcu32,
SQ_PERF_SEL_INST_CYCLES_VMEM_ATOMIC      = 0x000000ddu32,
SQ_PERF_SEL_INSTS_TEX_BLOCK_LOAD         = 0x000000deu32,
SQ_PERF_SEL_INSTS_TEX_SAMPLE             = 0x000000e0u32,
SQ_PERF_SEL_INSTS_TEX_ATOMIC_RTN         = 0x000000e1u32,
SQ_PERF_SEL_INSTS_TEX_BLOCK_STORE        = 0x000000e2u32,
SQ_PERF_SEL_INSTS_TEX_ATOMIC_NORTN       = 0x000000e3u32,
SQ_PERF_SEL_INSTS_GLOBAL_SCRATCH         = 0x000000e4u32,
SQ_PERF_SEL_INSTS_WMMA_LOAD              = 0x000000e5u32,
SQ_PERF_SEL_INSTS_FLAT_ATOMIC            = 0x000000e6u32,
SQ_PERF_SEL_INSTS_EXP_MRT                = 0x000000e7u32,
SQ_PERF_SEL_INSTS_EXP_Z                  = 0x000000e8u32,
SQ_PERF_SEL_INSTS_VEC32_VALU_WMMA        = 0x000000e9u32,
SQ_PERF_SEL_INSTS_VEC32_LDS_LOAD         = 0x000000eau32,
SQ_PERF_SEL_INSTS_VEC32_LDS_ATOMIC_RTN   = 0x000000ebu32,
SQ_PERF_SEL_INSTS_VEC32_LDS_STORE        = 0x000000ecu32,
SQ_PERF_SEL_INSTS_VEC32_LDS_ATOMIC_NORTN = 0x000000edu32,
SQ_PERF_SEL_INSTS_VEC32_LDS_OTHER        = 0x000000efu32,
SQ_PERF_SEL_INSTS_VEC32_TEX_SAMPLE       = 0x000000f1u32,
SQ_PERF_SEL_INSTS_VEC32_TEX_ATOMIC       = 0x000000f2u32,
SQ_PERF_SEL_INSTS_VEC32_FLAT_LOAD        = 0x000000f3u32,
SQ_PERF_SEL_INSTS_VEC32_FLAT_STORE       = 0x000000f4u32,
SQ_PERF_SEL_INSTS_VEC32_FLAT_ATOMIC      = 0x000000f5u32,
SQ_PERF_SEL_INSTS_VEC32_GLOBAL_SCRATCH   = 0x000000f6u32,
SQ_PERF_SEL_INSTS_VEC32_GLOBAL_SCRATCH_LOAD = 0x000000f7u32,
SQ_PERF_SEL_INSTS_VEC32_GLOBAL_SCRATCH_STORE = 0x000000f8u32,
SQ_PERF_SEL_INSTS_VEC32_GLOBAL_SCRATCH_ATOMIC = 0x000000f9u32,
SQ_PERF_SEL_INSTS_VEC32_LEVEL_LDS        = 0x000000fau32,
SQ_PERF_SEL_DUMMY_END                    = 0x000000fbu32,
SQ_PERF_SEL_DUMMY_LAST                   = 0x0000011fu32,
SQC_PERF_SEL_LDS_BANK_CONFLICT           = 0x00000120u32,
SQC_PERF_SEL_LDS_ADDR_CONFLICT           = 0x00000121u32,
SQC_PERF_SEL_LDS_UNALIGNED_STALL         = 0x00000122u32,
SQC_PERF_SEL_LDS_MEM_VIOLATIONS          = 0x00000123u32,
SQC_PERF_SEL_LDS_ATOMIC_RETURN           = 0x00000124u32,
SQC_PERF_SEL_LDS_IDX_ACTIVE              = 0x00000125u32,
SQC_PERF_SEL_LDS_ADDR_STALL              = 0x00000126u32,
SQC_PERF_SEL_LDS_ADDR_ACTIVE             = 0x00000127u32,
SQC_PERF_SEL_LDS_PC_LDS_WRITE_STALL_TD   = 0x00000128u32,
SQC_PERF_SEL_LDS_SPI_VGPR_WRITE_STALL_TD = 0x00000129u32,
SQC_PERF_SEL_LDS_LDS_VGPR_WRITE_STALL    = 0x0000012au32,
SQC_PERF_SEL_LDS_FP_ADD_CYCLES           = 0x0000012bu32,
SQC_PERF_SEL_ICACHE_BUSY_CYCLES          = 0x0000012cu32,
SQC_PERF_SEL_ICACHE_REQ                  = 0x0000012du32,
SQC_PERF_SEL_ICACHE_HITS                 = 0x0000012eu32,
SQC_PERF_SEL_ICACHE_MISSES               = 0x0000012fu32,
SQC_PERF_SEL_ICACHE_MISSES_DUPLICATE     = 0x00000130u32,
SQC_PERF_SEL_ICACHE_INVAL_INST           = 0x00000131u32,
SQC_PERF_SEL_ICACHE_INVAL_ASYNC          = 0x00000132u32,
SQC_PERF_SEL_ICACHE_INFLIGHT_LEVEL       = 0x00000133u32,
SQC_PERF_SEL_DCACHE_INFLIGHT_LEVEL       = 0x00000134u32,
SQC_PERF_SEL_TC_INFLIGHT_LEVEL           = 0x00000135u32,
SQC_PERF_SEL_ICACHE_TC_INFLIGHT_LEVEL    = 0x00000136u32,
SQC_PERF_SEL_DCACHE_TC_INFLIGHT_LEVEL    = 0x00000137u32,
SQC_PERF_SEL_ICACHE_INPUT_VALID_READYB   = 0x00000138u32,
SQC_PERF_SEL_DCACHE_INPUT_VALID_READYB   = 0x00000139u32,
SQC_PERF_SEL_TC_REQ                      = 0x0000013au32,
SQC_PERF_SEL_TC_INST_REQ                 = 0x0000013bu32,
SQC_PERF_SEL_TC_DATA_READ_REQ            = 0x0000013cu32,
SQC_PERF_SEL_TC_STALL                    = 0x0000013du32,
SQC_PERF_SEL_TC_STARVE                   = 0x0000013eu32,
SQC_PERF_SEL_ICACHE_INPUT_STALL_ARB_NO_GRANT = 0x0000013fu32,
SQC_PERF_SEL_ICACHE_INPUT_STALL_BANK_READYB = 0x00000140u32,
SQC_PERF_SEL_ICACHE_CACHE_STALLED        = 0x00000141u32,
SQC_PERF_SEL_ICACHE_CACHE_STALL_INFLIGHT_MAX = 0x00000142u32,
SQC_PERF_SEL_ICACHE_STALL_OUTXBAR_ARB_NO_GRANT = 0x00000143u32,
SQC_PERF_SEL_DCACHE_BUSY_CYCLES          = 0x00000144u32,
SQC_PERF_SEL_DCACHE_REQ                  = 0x00000145u32,
SQC_PERF_SEL_DCACHE_HITS                 = 0x00000146u32,
SQC_PERF_SEL_DCACHE_MISSES               = 0x00000147u32,
SQC_PERF_SEL_DCACHE_MISSES_DUPLICATE     = 0x00000148u32,
SQC_PERF_SEL_DCACHE_INVAL_INST           = 0x00000149u32,
SQC_PERF_SEL_DCACHE_INVAL_ASYNC          = 0x0000014au32,
SQC_PERF_SEL_DCACHE_HIT_LRU_READ         = 0x0000014bu32,
SQC_PERF_SEL_DCACHE_INPUT_STALL_ARB_NO_GRANT = 0x0000014cu32,
SQC_PERF_SEL_DCACHE_INPUT_STALL_BANK_READYB = 0x0000014du32,
SQC_PERF_SEL_DCACHE_CACHE_STALLED        = 0x0000014eu32,
SQC_PERF_SEL_DCACHE_CACHE_STALL_INFLIGHT_MAX = 0x0000014fu32,
SQC_PERF_SEL_DCACHE_CACHE_STALL_OUTPUT   = 0x00000150u32,
SQC_PERF_SEL_DCACHE_STALL_OUTXBAR_ARB_NO_GRANT = 0x00000151u32,
SQC_PERF_SEL_DCACHE_REQ_READ_1           = 0x00000152u32,
SQC_PERF_SEL_DCACHE_REQ_READ_2           = 0x00000153u32,
SQC_PERF_SEL_DCACHE_REQ_READ_4           = 0x00000154u32,
SQC_PERF_SEL_DCACHE_REQ_READ_8           = 0x00000155u32,
SQC_PERF_SEL_DCACHE_REQ_READ_16          = 0x00000156u32,
SQC_PERF_SEL_DCACHE_REQ_ATC_PROBE        = 0x00000157u32,
SQC_PERF_SEL_SQ_DCACHE_REQS              = 0x00000158u32,
SQC_PERF_SEL_DCACHE_FLAT_REQ             = 0x00000159u32,
SQC_PERF_SEL_TD_VGPR_BUSY                = 0x0000015au32,
SQC_PERF_SEL_LDS_VGPR_BUSY               = 0x0000015bu32,
SQC_PERF_SEL_LDS_TD_VGPR_CONF_STALL      = 0x0000015cu32,
SQC_PERF_SEL_ICACHE_GCR                  = 0x0000015du32,
SQC_PERF_SEL_ICACHE_GCR_HITS             = 0x0000015eu32,
SQC_PERF_SEL_DCACHE_GCR                  = 0x0000015fu32,
SQC_PERF_SEL_DCACHE_GCR_HITS             = 0x00000160u32,
SQC_PERF_SEL_ICACHE_GCR_INVALIDATE       = 0x00000161u32,
SQC_PERF_SEL_DCACHE_GCR_INVALIDATE       = 0x00000162u32,
SQC_PERF_SEL_DCACHE_SPI_RETURN_STALL     = 0x00000163u32,
SQC_PERF_SEL_ICACHE_PREFETCH_REQ_CACHELINES = 0x00000164u32,
SQC_PERF_SEL_DCACHE_PREFETCH_REQ_CACHELINES = 0x00000165u32,
SQC_PERF_SEL_ICACHE_PREFETCH_MISSES      = 0x00000166u32,
SQC_PERF_SEL_DCACHE_PREFETCH_MISSES      = 0x00000167u32,
SQC_PERF_SEL_LDS_BANKCONF_LOAD_CNT       = 0x00000168u32,
SQC_PERF_SEL_LDS_BANKCONF_STORE_CNT      = 0x00000169u32,
SQC_PERF_SEL_LDS_BANKCONF_ATOMIC_CNT     = 0x0000016au32,
SQC_PERF_SEL_LDS_ACTIVE_LOAD_CNT         = 0x0000016bu32,
SQC_PERF_SEL_LDS_ACTIVE_STORE_CNT        = 0x0000016cu32,
SQC_PERF_SEL_LDS_ACTIVE_ATOMIC_CNT       = 0x0000016du32,
SQC_PERF_SEL_LDS_STORE_DWORDS            = 0x0000016eu32,
SQC_PERF_SEL_LDS_LOAD_DWORDS             = 0x0000016fu32,
SQC_PERF_SEL_LDS_ATOMIC_DWORDS           = 0x00000170u32,
SQC_PERF_SEL_LDS_LDS_EXECUTION_STALL     = 0x00000171u32,
SQC_PERF_SEL_DUMMY_LAST                  = 0x00000172u32,
SP_PERF_SEL_DST_BUF_ALLOC_STALL          = 0x000001c0u32,
SP_PERF_SEL_DST_BUF_WB_CONF_W_TD_LDS     = 0x000001c1u32,
SP_PERF_SEL_DST_BUF_WB_CONF_W_SPI        = 0x000001c2u32,
SP_PERF_SEL_DST_BUF_EVEN_DIRTY           = 0x000001c3u32,
SP_PERF_SEL_DST_BUF_ODD_DIRTY            = 0x000001c4u32,
SP_PERF_SEL_SRC_CACHE_HIT_B0             = 0x000001c5u32,
SP_PERF_SEL_SRC_CACHE_HIT_B1             = 0x000001c6u32,
SP_PERF_SEL_SRC_CACHE_HIT_B2             = 0x000001c7u32,
SP_PERF_SEL_SRC_CACHE_HIT_B3             = 0x000001c8u32,
SP_PERF_SEL_SRC_CACHE_PROBE_B0           = 0x000001c9u32,
SP_PERF_SEL_SRC_CACHE_PROBE_B1           = 0x000001cau32,
SP_PERF_SEL_SRC_CACHE_PROBE_B2           = 0x000001cbu32,
SP_PERF_SEL_SRC_CACHE_PROBE_B3           = 0x000001ccu32,
SP_PERF_SEL_SRC_CACHE_VGPR_RD_B0         = 0x000001cdu32,
SP_PERF_SEL_SRC_CACHE_VGPR_RD_B1         = 0x000001ceu32,
SP_PERF_SEL_SRC_CACHE_VGPR_RD_B2         = 0x000001cfu32,
SP_PERF_SEL_SRC_CACHE_VGPR_RD_B3         = 0x000001d0u32,
SP_PERF_SEL_SRC_CACHE_RECYCLE_HIT_B0     = 0x000001d1u32,
SP_PERF_SEL_SRC_CACHE_RECYCLE_HIT_B1     = 0x000001d2u32,
SP_PERF_SEL_SRC_CACHE_RECYCLE_HIT_B2     = 0x000001d3u32,
SP_PERF_SEL_SRC_CACHE_RECYCLE_HIT_B3     = 0x000001d4u32,
SP_PERF_SEL_SRC_CACHE_PROBE_SUCCESS_B0   = 0x000001d5u32,
SP_PERF_SEL_SRC_CACHE_PROBE_SUCCESS_B1   = 0x000001d6u32,
SP_PERF_SEL_SRC_CACHE_PROBE_SUCCESS_B2   = 0x000001d7u32,
SP_PERF_SEL_SRC_CACHE_PROBE_SUCCESS_B3   = 0x000001d8u32,
SP_PERF_SEL_VALU_PENDING_QUEUE_STALL     = 0x000001d9u32,
SP_PERF_SEL_VALU_OPERAND                 = 0x000001dau32,
SP_PERF_SEL_VALU_VGPR_OPERAND            = 0x000001dbu32,
SP_PERF_SEL_VALU_OPERAND_FROM_DST_BUF    = 0x000001dcu32,
SP_PERF_SEL_VALU_EXEC_MASK_CHANGE        = 0x000001ddu32,
SP_PERF_SEL_VALU_COEXEC_WITH_TRANS       = 0x000001deu32,
SP_PERF_SEL_VALU_SGPR_FWD_BUF_FULL       = 0x000001dfu32,
SP_PERF_SEL_VALU_STALL                   = 0x000001e0u32,
SP_PERF_SEL_VALU_STALL_VGPR_NOT_READY    = 0x000001e1u32,
SP_PERF_SEL_VALU_STALL_SGPR_NOT_READY    = 0x000001e2u32,
SP_PERF_SEL_VALU_STALL_VDST_FWD          = 0x000001e3u32,
SP_PERF_SEL_VALU_STALL_SDST_FWD          = 0x000001e4u32,
SP_PERF_SEL_VALU_STALL_DST_STALL         = 0x000001e5u32,
SP_PERF_SEL_VALU_FAST_OP_STALL_VGPR_NOT_READY = 0x000001e6u32,
SP_PERF_SEL_VGPR_VMEM_RD                 = 0x000001e7u32,
SP_PERF_SEL_VGPR_EXP_RD                  = 0x000001e8u32,
SP_PERF_SEL_VGPR_SPI_WR                  = 0x000001e9u32,
SP_PERF_SEL_VGPR_TDLDS_DATA_WR           = 0x000001eau32,
SP_PERF_SEL_VGPR_WR                      = 0x000001ebu32,
SP_PERF_SEL_VGPR_RD                      = 0x000001ecu32,
SP_PERF_SEL_VGPR_WR_KILL                 = 0x000001edu32,
SP_PERF_SEL_VALU_VGPR_RD_CONFLICT_EXP    = 0x000001eeu32,
SP_PERF_SEL_VALU_VGPR_RD_CONFLICT_LDS    = 0x000001efu32,
SP_PERF_SEL_VALU_VGPR_RD_CONFLICT_TEX    = 0x000001f0u32,
SP_PERF_SEL_DUMMY_LAST                   = 0x000001f1u32,
SQ_PERF_SEL_NONE2                        = 0x000001ffu32,
}

/*
 * SQ_ROUND_MODE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_ROUND_MODE {
SQ_ROUND_NEAREST_EVEN                    = 0x00000000u32,
SQ_ROUND_PLUS_INFINITY                   = 0x00000001u32,
SQ_ROUND_MINUS_INFINITY                  = 0x00000002u32,
SQ_ROUND_TO_ZERO                         = 0x00000003u32,
}

/*
 * SQ_RSRC_BUF_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_RSRC_BUF_TYPE {
SQ_RSRC_BUF                              = 0x00000000u32,
SQ_RSRC_BUF_RSVD_1                       = 0x00000001u32,
SQ_RSRC_BUF_RSVD_2                       = 0x00000002u32,
SQ_RSRC_BUF_RSVD_3                       = 0x00000003u32,
}

/*
 * SQ_RSRC_FLAT_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_RSRC_FLAT_TYPE {
SQ_RSRC_FLAT_RSVD_0                      = 0x00000000u32,
SQ_RSRC_FLAT                             = 0x00000001u32,
SQ_RSRC_FLAT_RSVD_2                      = 0x00000002u32,
SQ_RSRC_FLAT_RSVD_3                      = 0x00000003u32,
}

/*
 * SQ_RSRC_IMG_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_RSRC_IMG_TYPE {
SQ_RSRC_IMG_RSVD_0                       = 0x00000000u32,
SQ_RSRC_IMG_RSVD_1                       = 0x00000001u32,
SQ_RSRC_IMG_RSVD_2                       = 0x00000002u32,
SQ_RSRC_IMG_RSVD_3                       = 0x00000003u32,
SQ_RSRC_IMG_RSVD_4                       = 0x00000004u32,
SQ_RSRC_IMG_RSVD_5                       = 0x00000005u32,
SQ_RSRC_IMG_RSVD_6                       = 0x00000006u32,
SQ_RSRC_IMG_RSVD_7                       = 0x00000007u32,
SQ_RSRC_IMG_1D                           = 0x00000008u32,
SQ_RSRC_IMG_2D                           = 0x00000009u32,
SQ_RSRC_IMG_3D                           = 0x0000000au32,
SQ_RSRC_IMG_CUBE                         = 0x0000000bu32,
SQ_RSRC_IMG_1D_ARRAY                     = 0x0000000cu32,
SQ_RSRC_IMG_2D_ARRAY                     = 0x0000000du32,
SQ_RSRC_IMG_2D_MSAA                      = 0x0000000eu32,
SQ_RSRC_IMG_2D_MSAA_ARRAY                = 0x0000000fu32,
}

/*
 * SQ_SEL_XYZW01 enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_SEL_XYZW01 {
SQ_SEL_0                                 = 0x00000000u32,
SQ_SEL_1                                 = 0x00000001u32,
SQ_SEL_N_BC_1                            = 0x00000002u32,
SQ_SEL_RESERVED_1                        = 0x00000003u32,
SQ_SEL_X                                 = 0x00000004u32,
SQ_SEL_Y                                 = 0x00000005u32,
SQ_SEL_Z                                 = 0x00000006u32,
SQ_SEL_W                                 = 0x00000007u32,
}

/*
 * SQ_TEX_ANISO_RATIO enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_TEX_ANISO_RATIO {
SQ_TEX_ANISO_RATIO_1                     = 0x00000000u32,
SQ_TEX_ANISO_RATIO_2                     = 0x00000001u32,
SQ_TEX_ANISO_RATIO_4                     = 0x00000002u32,
SQ_TEX_ANISO_RATIO_8                     = 0x00000003u32,
SQ_TEX_ANISO_RATIO_16                    = 0x00000004u32,
}

/*
 * SQ_TEX_BORDER_COLOR enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_TEX_BORDER_COLOR {
SQ_TEX_BORDER_COLOR_TRANS_BLACK          = 0x00000000u32,
SQ_TEX_BORDER_COLOR_OPAQUE_BLACK         = 0x00000001u32,
SQ_TEX_BORDER_COLOR_OPAQUE_WHITE         = 0x00000002u32,
SQ_TEX_BORDER_COLOR_REGISTER             = 0x00000003u32,
}

/*
 * SQ_TEX_CLAMP enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_TEX_CLAMP {
SQ_TEX_WRAP                              = 0x00000000u32,
SQ_TEX_MIRROR                            = 0x00000001u32,
SQ_TEX_CLAMP_LAST_TEXEL                  = 0x00000002u32,
SQ_TEX_MIRROR_ONCE_LAST_TEXEL            = 0x00000003u32,
SQ_TEX_CLAMP_HALF_BORDER                 = 0x00000004u32,
SQ_TEX_MIRROR_ONCE_HALF_BORDER           = 0x00000005u32,
SQ_TEX_CLAMP_BORDER                      = 0x00000006u32,
SQ_TEX_MIRROR_ONCE_BORDER                = 0x00000007u32,
}

/*
 * SQ_TEX_DEPTH_COMPARE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_TEX_DEPTH_COMPARE {
SQ_TEX_DEPTH_COMPARE_NEVER               = 0x00000000u32,
SQ_TEX_DEPTH_COMPARE_LESS                = 0x00000001u32,
SQ_TEX_DEPTH_COMPARE_EQUAL               = 0x00000002u32,
SQ_TEX_DEPTH_COMPARE_LESSEQUAL           = 0x00000003u32,
SQ_TEX_DEPTH_COMPARE_GREATER             = 0x00000004u32,
SQ_TEX_DEPTH_COMPARE_NOTEQUAL            = 0x00000005u32,
SQ_TEX_DEPTH_COMPARE_GREATEREQUAL        = 0x00000006u32,
SQ_TEX_DEPTH_COMPARE_ALWAYS              = 0x00000007u32,
}

/*
 * SQ_TEX_MIP_FILTER enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_TEX_MIP_FILTER {
SQ_TEX_MIP_FILTER_NONE                   = 0x00000000u32,
SQ_TEX_MIP_FILTER_POINT                  = 0x00000001u32,
SQ_TEX_MIP_FILTER_LINEAR                 = 0x00000002u32,
SQ_TEX_MIP_FILTER_POINT_ANISO_ADJ        = 0x00000003u32,
}

/*
 * SQ_TEX_XY_FILTER enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_TEX_XY_FILTER {
SQ_TEX_XY_FILTER_POINT                   = 0x00000000u32,
SQ_TEX_XY_FILTER_BILINEAR                = 0x00000001u32,
SQ_TEX_XY_FILTER_ANISO_POINT             = 0x00000002u32,
SQ_TEX_XY_FILTER_ANISO_BILINEAR          = 0x00000003u32,
}

/*
 * SQ_TEX_Z_FILTER enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_TEX_Z_FILTER {
SQ_TEX_Z_FILTER_NONE                     = 0x00000000u32,
SQ_TEX_Z_FILTER_POINT                    = 0x00000001u32,
SQ_TEX_Z_FILTER_LINEAR                   = 0x00000002u32,
}

/*
 * SQ_WATCH_MODES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_WATCH_MODES {
SQ_WATCH_MODE_READ                       = 0x00000000u32,
SQ_WATCH_MODE_NONREAD                    = 0x00000001u32,
SQ_WATCH_MODE_ATOMIC                     = 0x00000002u32,
SQ_WATCH_MODE_ALL                        = 0x00000003u32,
}

/*
 * SQ_WAVE_FWD_PROG_INTERVAL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_WAVE_FWD_PROG_INTERVAL {
SQ_WAVE_FWD_PROG_INTERVAL_NEVER          = 0x00000000u32,
SQ_WAVE_FWD_PROG_INTERVAL_256            = 0x00000001u32,
SQ_WAVE_FWD_PROG_INTERVAL_1024           = 0x00000002u32,
SQ_WAVE_FWD_PROG_INTERVAL_4096           = 0x00000003u32,
}

/*
 * SQ_WAVE_SCHED_MODES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_WAVE_SCHED_MODES {
SQ_WAVE_SCHED_MODE_NORMAL                = 0x00000000u32,
SQ_WAVE_SCHED_MODE_EXPERT                = 0x00000001u32,
SQ_WAVE_SCHED_MODE_DISABLE_VA_VDST_VM_VSRC = 0x00000002u32,
}

/*
 * SQ_WAVE_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SQ_WAVE_TYPE {
SQ_WAVE_TYPE_PS                          = 0x00000000u32,
SQ_WAVE_TYPE_RSVD0                       = 0x00000001u32,
SQ_WAVE_TYPE_GS                          = 0x00000002u32,
SQ_WAVE_TYPE_RSVD1                       = 0x00000003u32,
SQ_WAVE_TYPE_HS                          = 0x00000004u32,
SQ_WAVE_TYPE_RSVD2                       = 0x00000005u32,
SQ_WAVE_TYPE_CS                          = 0x00000006u32,
SQ_WAVE_TYPE_PS1                         = 0x00000007u32,
SQ_WAVE_TYPE_PS2                         = 0x00000008u32,
SQ_WAVE_TYPE_PS3                         = 0x00000009u32,
}

/*
 * SQ_WAVE_TYPE value
 */

#define SQ_WAVE_TYPE_PS0               0x00000000

/*
 * SQ_SEG value
 */

#define SQ_FLAT                        0x00000000
#define SQ_SCRATCH                     0x00000001
#define SQ_GLOBAL                      0x00000002

/*
 * SQIND_PARTITIONS value
 */

#define SQIND_GLOBAL_REGS_OFFSET       0x00000000
#define SQIND_GLOBAL_REGS_SIZE         0x00000008
#define SQIND_LOCAL_REGS_OFFSET        0x00000008
#define SQIND_LOCAL_REGS_SIZE          0x00000008
#define SQIND_WAVE_HW_REGS_OFFSET      0x00000100
#define SQIND_WAVE_HW_REGS_SIZE        0x00000040
#define SQIND_WAVE_HOST_REGS_OFFSET    0x00000140
#define SQIND_WAVE_HOST_REGS_SIZE      0x000000c0
#define SQIND_WAVE_SGPRS_OFFSET        0x00000200
#define SQIND_WAVE_SGPRS_SIZE          0x00000200
#define SQIND_WAVE_VGPRS_OFFSET        0x00000400
#define SQIND_WAVE_VGPRS_SIZE          0x00000400

/*
 * SQ_GFXDEC value
 */

#define SQ_GFXDEC_BEGIN                0x0000a000
#define SQ_GFXDEC_END                  0x0000c000
#define SQ_GFXDEC_STATE_ID_SHIFT       0x0000000a

/*
 * SQDEC value
 */

#define SQDEC_BEGIN                    0x00002300
#define SQDEC_END                      0x000023ff

/*
 * PFVF_SQDEC value
 */

#define PFVF_SQDEC_BEGIN               0x0000a9e0
#define PFVF_SQDEC_END                 0x0000a9ff

/*
 * SQPERFSDEC value
 */

#define SQPERFSDEC_BEGIN               0x0000d9c0
#define SQPERFSDEC_END                 0x0000da40

/*
 * SQPERFDDEC value
 */

#define SQPERFDDEC_BEGIN               0x0000d1c0
#define SQPERFDDEC_END                 0x0000d240

/*
 * SQGFXUDEC value
 */

#define SQGFXUDEC_BEGIN                0x0000c330
#define SQGFXUDEC_END                  0x0000c380

/*
 * SQPWRDEC value
 */

#define SQPWRDEC_BEGIN                 0x0000f08c
#define SQPWRDEC_END                   0x0000f094

/*
 * SQ_DISPATCHER value
 */

#define SQ_DISPATCHER_GFX_MIN          0x00000010
#define SQ_DISPATCHER_GFX_CNT_PER_RING 0x00000008

/*
 * SQ_MAX value
 */

#define SQ_MAX_PGM_SGPRS               0x00000068
#define SQ_MAX_PGM_VGPRS               0x00000100

/*
 * SQ_EXCP_BITS value
 */

#define SQ_EX_EXCP_VALU_BASE           0x00000000
#define SQ_EX_EXCP_VALU_SIZE           0x00000007
#define SQ_EX_EXCP_ALU_INVALID         0x00000000
#define SQ_EX_EXCP_ALU_INPUT_DENORM    0x00000001
#define SQ_EX_EXCP_ALU_FLOAT_DIV0      0x00000002
#define SQ_EX_EXCP_ALU_OVERFLOW        0x00000003
#define SQ_EX_EXCP_ALU_UNDERFLOW       0x00000004
#define SQ_EX_EXCP_ALU_INEXACT         0x00000005
#define SQ_EX_EXCP_ALU_INT_DIV0        0x00000006
#define SQ_EX_EXCP_ADDR_WATCH          0x00000007

/*
 * HW_INSERTED_INST_ID value
 */

#define INST_ID_PRIV_START             0x80000000
#define INST_ID_ECC_INTERRUPT_MSG      0xfffffff0
#define INST_ID_TTRACE_NEW_PC_MSG      0xfffffff1
#define INST_ID_HW_TRAP                0xfffffff2
#define INST_ID_KILL_SEQ               0xfffffff3
#define INST_ID_SPI_WREXEC             0xfffffff4
#define INST_ID_HW_TRAP_GET_TBA        0xfffffff5
#define INST_ID_HOST_REG_TRAP_MSG      0xfffffffe

/*
 * SIMM16_WAITCNT_PARTITIONS value
 */

#define SIMM16_WAITCNT_EXP_CNT_START   0x00000000
#define SIMM16_WAITCNT_EXP_CNT_SIZE    0x00000003
#define SIMM16_WAITCNT_LGKM_CNT_START  0x00000004
#define SIMM16_WAITCNT_LGKM_CNT_SIZE   0x00000006
#define SIMM16_WAITCNT_VM_CNT_START    0x0000000a
#define SIMM16_WAITCNT_VM_CNT_SIZE     0x00000006
#define SIMM16_WAITCNT_DEPCTR_SA_SDST_START 0x00000000
#define SIMM16_WAITCNT_DEPCTR_SA_SDST_SIZE 0x00000001
#define SIMM16_WAITCNT_DEPCTR_VA_VCC_START 0x00000001
#define SIMM16_WAITCNT_DEPCTR_VA_VCC_SIZE 0x00000001
#define SIMM16_WAITCNT_DEPCTR_VM_VSRC_START 0x00000002
#define SIMM16_WAITCNT_DEPCTR_VM_VSRC_SIZE 0x00000003
#define SIMM16_WAITCNT_DEPCTR_HOLD_CNT_START 0x00000007
#define SIMM16_WAITCNT_DEPCTR_HOLD_CNT_SIZE 0x00000001
#define SIMM16_WAITCNT_DEPCTR_VA_SSRC_START 0x00000008
#define SIMM16_WAITCNT_DEPCTR_VA_SSRC_SIZE 0x00000001
#define SIMM16_WAITCNT_DEPCTR_VA_SDST_START 0x00000009
#define SIMM16_WAITCNT_DEPCTR_VA_SDST_SIZE 0x00000003
#define SIMM16_WAITCNT_DEPCTR_VA_VDST_START 0x0000000c
#define SIMM16_WAITCNT_DEPCTR_VA_VDST_SIZE 0x00000004

/*
 * SIMM16_WAIT_EVENT_PARTITIONS value
 */

#define SIMM16_WAIT_EVENT_EXP_RDY_START 0x00000000
#define SIMM16_WAIT_EVENT_EXP_RDY_SIZE 0x00000001

/*
 * SQ_WAVE_IB_DEP_COUNTER_SIZES value
 */

#define SQ_WAVE_IB_DEP_SA_SDST_SIZE    0x00000004
#define SQ_WAVE_IB_DEP_SA_EXEC_SIZE    0x00000002
#define SQ_WAVE_IB_DEP_SA_M0_SIZE      0x00000001
#define SQ_WAVE_IB_DEP_VM_VSRC_SIZE    0x00000004
#define SQ_WAVE_IB_DEP_HOLD_CNT_SIZE   0x00000001
#define SQ_WAVE_IB_DEP_VA_SSRC_SIZE    0x00000003
#define SQ_WAVE_IB_DEP_VA_SDST_SIZE    0x00000004
#define SQ_WAVE_IB_DEP_VA_VCC_SIZE     0x00000003
#define SQ_WAVE_IB_DEP_VA_EXEC_SIZE    0x00000002
#define SQ_WAVE_IB_DEP_VA_VDST_SIZE    0x00000005
#define SQ_WAVE_IB_DEP_LDS_DIR_SIZE    0x00000003

/*
 * SQ_ARB_STATE value
 */

#define SQ_ARB_STATE_ISSUED_BRMSG      0x00000000
#define SQ_ARB_STATE_ISSUED_EXPORT     0x00000001
#define SQ_ARB_STATE_ISSUED_LDS_DIRECT 0x00000002
#define SQ_ARB_STATE_ISSUED_LDS        0x00000003
#define SQ_ARB_STATE_ISSUED_TEX        0x00000004
#define SQ_ARB_STATE_ISSUED_SCALAR     0x00000005
#define SQ_ARB_STATE_ISSUED_VALU       0x00000006
#define SQ_ARB_STATE_STALLED_BRMSG     0x00000008
#define SQ_ARB_STATE_STALLED_EXPORT    0x00000009
#define SQ_ARB_STATE_STALLED_LDS_DIRECT 0x0000000a
#define SQ_ARB_STATE_STALLED_LDS       0x0000000b
#define SQ_ARB_STATE_STALLED_TEX       0x0000000c
#define SQ_ARB_STATE_STALLED_SCALAR    0x0000000d
#define SQ_ARB_STATE_STALLED_VALU      0x0000000e

/*******************************************************
 * GL1 Enums
 *******************************************************/

/*
 * GL1A_PERF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GL1A_PERF_SEL {
GL1A_PERF_SEL_BUSY                       = 0x00000000u32,
GL1A_PERF_SEL_STALL_GL1C0                = 0x00000001u32,
GL1A_PERF_SEL_STALL_GL1C1                = 0x00000002u32,
GL1A_PERF_SEL_STALL_GL1C2                = 0x00000003u32,
GL1A_PERF_SEL_STALL_GL1C3                = 0x00000004u32,
GL1A_PERF_SEL_REQUEST_GL1C0              = 0x00000005u32,
GL1A_PERF_SEL_REQUEST_GL1C1              = 0x00000006u32,
GL1A_PERF_SEL_REQUEST_GL1C2              = 0x00000007u32,
GL1A_PERF_SEL_REQUEST_GL1C3              = 0x00000008u32,
GL1A_PERF_SEL_WDS_32B_GL1C0              = 0x00000009u32,
GL1A_PERF_SEL_WDS_32B_GL1C1              = 0x0000000au32,
GL1A_PERF_SEL_WDS_32B_GL1C2              = 0x0000000bu32,
GL1A_PERF_SEL_WDS_32B_GL1C3              = 0x0000000cu32,
GL1A_PERF_SEL_BURST_COUNT_GL1C0          = 0x0000000du32,
GL1A_PERF_SEL_BURST_COUNT_GL1C1          = 0x0000000eu32,
GL1A_PERF_SEL_BURST_COUNT_GL1C2          = 0x0000000fu32,
GL1A_PERF_SEL_BURST_COUNT_GL1C3          = 0x00000010u32,
GL1A_PERF_SEL_ARB_REQUESTS               = 0x00000011u32,
GL1A_PERF_SEL_REQ_INFLIGHT_LEVEL         = 0x00000012u32,
GL1A_PERF_SEL_STALL_RET_CONFLICT_GL1C0   = 0x00000013u32,
GL1A_PERF_SEL_STALL_RET_CONFLICT_GL1C1   = 0x00000014u32,
GL1A_PERF_SEL_STALL_RET_CONFLICT_GL1C2   = 0x00000015u32,
GL1A_PERF_SEL_STALL_RET_CONFLICT_GL1C3   = 0x00000016u32,
GL1A_PERF_SEL_CYCLE                      = 0x00000017u32,
}

/*
 * GL1C_PERF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GL1C_PERF_SEL {
GL1C_PERF_SEL_CYCLE                      = 0x00000000u32,
GL1C_PERF_SEL_BUSY                       = 0x00000001u32,
GL1C_PERF_SEL_STARVE                     = 0x00000002u32,
GL1C_PERF_SEL_ARB_RET_LEVEL              = 0x00000003u32,
GL1C_PERF_SEL_GL2_REQ_READ_LATENCY       = 0x00000004u32,
GL1C_PERF_SEL_GL2_REQ_WRITE_LATENCY      = 0x00000005u32,
GL1C_PERF_SEL_REQ                        = 0x00000006u32,
GL1C_PERF_SEL_REQ_ATOMIC_WITH_RET        = 0x00000007u32,
GL1C_PERF_SEL_REQ_ATOMIC_WITHOUT_RET     = 0x00000008u32,
GL1C_PERF_SEL_REQ_NOP_ACK                = 0x00000009u32,
GL1C_PERF_SEL_REQ_NOP_RTN0               = 0x0000000au32,
GL1C_PERF_SEL_REQ_READ                   = 0x0000000bu32,
GL1C_PERF_SEL_REQ_READ_128B              = 0x0000000cu32,
GL1C_PERF_SEL_REQ_READ_32B               = 0x0000000du32,
GL1C_PERF_SEL_REQ_READ_64B               = 0x0000000eu32,
GL1C_PERF_SEL_REQ_WRITE                  = 0x0000000fu32,
GL1C_PERF_SEL_REQ_WRITE_32B              = 0x00000010u32,
GL1C_PERF_SEL_REQ_WRITE_64B              = 0x00000011u32,
GL1C_PERF_SEL_STALL_GL2_GL1              = 0x00000012u32,
GL1C_PERF_SEL_STALL_BUFFER_FULL          = 0x00000013u32,
GL1C_PERF_SEL_STALL_VM                   = 0x00000014u32,
GL1C_PERF_SEL_REQ_CLIENT0                = 0x00000015u32,
GL1C_PERF_SEL_REQ_CLIENT1                = 0x00000016u32,
GL1C_PERF_SEL_REQ_CLIENT2                = 0x00000017u32,
GL1C_PERF_SEL_REQ_CLIENT3                = 0x00000018u32,
GL1C_PERF_SEL_REQ_CLIENT4                = 0x00000019u32,
GL1C_PERF_SEL_REQ_CLIENT5                = 0x0000001au32,
GL1C_PERF_SEL_REQ_CLIENT6                = 0x0000001bu32,
GL1C_PERF_SEL_REQ_CLIENT7                = 0x0000001cu32,
GL1C_PERF_SEL_REQ_CLIENT8                = 0x0000001du32,
GL1C_PERF_SEL_REQ_CLIENT9                = 0x0000001eu32,
GL1C_PERF_SEL_REQ_CLIENT10               = 0x0000001fu32,
GL1C_PERF_SEL_REQ_CLIENT11               = 0x00000020u32,
GL1C_PERF_SEL_REQ_CLIENT12               = 0x00000021u32,
GL1C_PERF_SEL_REQ_CLIENT13               = 0x00000022u32,
GL1C_PERF_SEL_REQ_CLIENT14               = 0x00000023u32,
GL1C_PERF_SEL_REQ_CLIENT15               = 0x00000024u32,
GL1C_PERF_SEL_REQ_CLIENT16               = 0x00000025u32,
GL1C_PERF_SEL_REQ_CLIENT17               = 0x00000026u32,
GL1C_PERF_SEL_REQ_CLIENT18               = 0x00000027u32,
GL1C_PERF_SEL_REQ_CLIENT19               = 0x00000028u32,
GL1C_PERF_SEL_REQ_CLIENT20               = 0x00000029u32,
GL1C_PERF_SEL_REQ_CLIENT21               = 0x0000002au32,
GL1C_PERF_SEL_REQ_CLIENT22               = 0x0000002bu32,
GL1C_PERF_SEL_REQ_CLIENT23               = 0x0000002cu32,
GL1C_PERF_SEL_REQ_CLIENT24               = 0x0000002du32,
GL1C_PERF_SEL_REQ_CLIENT25               = 0x0000002eu32,
GL1C_PERF_SEL_REQ_CLIENT26               = 0x0000002fu32,
GL1C_PERF_SEL_REQ_CLIENT27               = 0x00000030u32,
GL1C_PERF_SEL_UTCL0_REQUEST              = 0x00000031u32,
GL1C_PERF_SEL_UTCL0_TRANSLATION_HIT      = 0x00000032u32,
GL1C_PERF_SEL_UTCL0_TRANSLATION_MISS     = 0x00000033u32,
GL1C_PERF_SEL_UTCL0_PERMISSION_MISS      = 0x00000034u32,
GL1C_PERF_SEL_UTCL0_MISS_UNDER_MISS      = 0x00000035u32,
GL1C_PERF_SEL_UTCL0_LFIFO_FULL           = 0x00000036u32,
GL1C_PERF_SEL_UTCL0_STALL_INFLIGHT_MAX   = 0x00000037u32,
GL1C_PERF_SEL_UTCL0_STALL_LFIFO_NOT_RES  = 0x00000038u32,
GL1C_PERF_SEL_UTCL0_STALL_LRU_INFLIGHT   = 0x00000039u32,
GL1C_PERF_SEL_UTCL0_STALL_MISSFIFO_FULL  = 0x0000003au32,
GL1C_PERF_SEL_UTCL0_STALL_MULTI_MISS     = 0x0000003bu32,
GL1C_PERF_SEL_UTCL0_STALL_UTCL1_REQ_OUT_OF_CREDITS = 0x0000003cu32,
GL1C_PERF_SEL_UTCL0_UTCL1_PERM_FAULT     = 0x0000003du32,
GL1C_PERF_SEL_CLIENT_UTCL0_INFLIGHT      = 0x0000003eu32,
GL1C_PERF_SEL_UTCL0_UTCL1_INFLIGHT       = 0x0000003fu32,
GL1C_PERF_SEL_UTCL0_INTERNAL_RETRY_REQ   = 0x00000040u32,
GL1C_PERF_SEL_UTCL0_UTCL1_XNACK_RETRY_FAULT = 0x00000041u32,
GL1C_PERF_SEL_UTCL0_UTCL1_XNACK_PRT_FAULT = 0x00000042u32,
GL1C_PERF_SEL_UTCL0_UTCL1_XNACK_NO_RETRY_FAULT = 0x00000043u32,
GL1C_PERF_SEL_UTCL0_GPA3_REQUEST         = 0x00000044u32,
}

/*
 * GL1XA_PERF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GL1XA_PERF_SEL {
GL1XA_PERF_SEL_BUSY                      = 0x00000000u32,
GL1XA_PERF_SEL_STALL_GL1XC0              = 0x00000001u32,
GL1XA_PERF_SEL_STALL_GL1XC1              = 0x00000002u32,
GL1XA_PERF_SEL_STALL_GL1XC2              = 0x00000003u32,
GL1XA_PERF_SEL_STALL_GL1XC3              = 0x00000004u32,
GL1XA_PERF_SEL_REQUEST_GL1XC0            = 0x00000005u32,
GL1XA_PERF_SEL_REQUEST_GL1XC1            = 0x00000006u32,
GL1XA_PERF_SEL_REQUEST_GL1XC2            = 0x00000007u32,
GL1XA_PERF_SEL_REQUEST_GL1XC3            = 0x00000008u32,
GL1XA_PERF_SEL_WDS_32B_GL1XC0            = 0x00000009u32,
GL1XA_PERF_SEL_WDS_32B_GL1XC1            = 0x0000000au32,
GL1XA_PERF_SEL_WDS_32B_GL1XC2            = 0x0000000bu32,
GL1XA_PERF_SEL_WDS_32B_GL1XC3            = 0x0000000cu32,
GL1XA_PERF_SEL_BURST_COUNT_GL1XC0        = 0x0000000du32,
GL1XA_PERF_SEL_BURST_COUNT_GL1XC1        = 0x0000000eu32,
GL1XA_PERF_SEL_BURST_COUNT_GL1XC2        = 0x0000000fu32,
GL1XA_PERF_SEL_BURST_COUNT_GL1XC3        = 0x00000010u32,
GL1XA_PERF_SEL_ARB_REQUESTS              = 0x00000011u32,
GL1XA_PERF_SEL_REQ_INFLIGHT_LEVEL        = 0x00000012u32,
GL1XA_PERF_SEL_STALL_RET_CONFLICT_GL1XC0 = 0x00000013u32,
GL1XA_PERF_SEL_STALL_RET_CONFLICT_GL1XC1 = 0x00000014u32,
GL1XA_PERF_SEL_STALL_RET_CONFLICT_GL1XC2 = 0x00000015u32,
GL1XA_PERF_SEL_STALL_RET_CONFLICT_GL1XC3 = 0x00000016u32,
GL1XA_PERF_SEL_CYCLE                     = 0x00000017u32,
}

/*
 * GL1XC_PERF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GL1XC_PERF_SEL {
GL1XC_PERF_SEL_CYCLE                     = 0x00000000u32,
GL1XC_PERF_SEL_BUSY                      = 0x00000001u32,
GL1XC_PERF_SEL_STARVE                    = 0x00000002u32,
GL1XC_PERF_SEL_ARB_RET_LEVEL             = 0x00000003u32,
GL1XC_PERF_SEL_GL2_REQ_READ_LATENCY      = 0x00000004u32,
GL1XC_PERF_SEL_GL2_REQ_WRITE_LATENCY     = 0x00000005u32,
GL1XC_PERF_SEL_REQ                       = 0x00000006u32,
GL1XC_PERF_SEL_REQ_ATOMIC_WITH_RET       = 0x00000007u32,
GL1XC_PERF_SEL_REQ_ATOMIC_WITHOUT_RET    = 0x00000008u32,
GL1XC_PERF_SEL_REQ_NOP_ACK               = 0x00000009u32,
GL1XC_PERF_SEL_REQ_NOP_RTN0              = 0x0000000au32,
GL1XC_PERF_SEL_REQ_READ                  = 0x0000000bu32,
GL1XC_PERF_SEL_REQ_READ_128B             = 0x0000000cu32,
GL1XC_PERF_SEL_REQ_READ_32B              = 0x0000000du32,
GL1XC_PERF_SEL_REQ_READ_64B              = 0x0000000eu32,
GL1XC_PERF_SEL_REQ_WRITE                 = 0x0000000fu32,
GL1XC_PERF_SEL_REQ_WRITE_32B             = 0x00000010u32,
GL1XC_PERF_SEL_REQ_WRITE_64B             = 0x00000011u32,
GL1XC_PERF_SEL_STALL_GL2_GL1             = 0x00000012u32,
GL1XC_PERF_SEL_STALL_BUFFER_FULL         = 0x00000013u32,
GL1XC_PERF_SEL_STALL_VM                  = 0x00000014u32,
GL1XC_PERF_SEL_REQ_CLIENT0               = 0x00000015u32,
GL1XC_PERF_SEL_REQ_CLIENT1               = 0x00000016u32,
GL1XC_PERF_SEL_REQ_CLIENT2               = 0x00000017u32,
GL1XC_PERF_SEL_REQ_CLIENT3               = 0x00000018u32,
GL1XC_PERF_SEL_REQ_CLIENT4               = 0x00000019u32,
GL1XC_PERF_SEL_REQ_CLIENT5               = 0x0000001au32,
GL1XC_PERF_SEL_REQ_CLIENT6               = 0x0000001bu32,
GL1XC_PERF_SEL_REQ_CLIENT7               = 0x0000001cu32,
GL1XC_PERF_SEL_REQ_CLIENT8               = 0x0000001du32,
GL1XC_PERF_SEL_REQ_CLIENT9               = 0x0000001eu32,
GL1XC_PERF_SEL_REQ_CLIENT10              = 0x0000001fu32,
GL1XC_PERF_SEL_REQ_CLIENT11              = 0x00000020u32,
GL1XC_PERF_SEL_REQ_CLIENT12              = 0x00000021u32,
GL1XC_PERF_SEL_REQ_CLIENT13              = 0x00000022u32,
GL1XC_PERF_SEL_REQ_CLIENT14              = 0x00000023u32,
GL1XC_PERF_SEL_REQ_CLIENT15              = 0x00000024u32,
GL1XC_PERF_SEL_REQ_CLIENT16              = 0x00000025u32,
GL1XC_PERF_SEL_REQ_CLIENT17              = 0x00000026u32,
GL1XC_PERF_SEL_REQ_CLIENT18              = 0x00000027u32,
GL1XC_PERF_SEL_REQ_CLIENT19              = 0x00000028u32,
GL1XC_PERF_SEL_REQ_CLIENT20              = 0x00000029u32,
GL1XC_PERF_SEL_REQ_CLIENT21              = 0x0000002au32,
GL1XC_PERF_SEL_REQ_CLIENT22              = 0x0000002bu32,
GL1XC_PERF_SEL_REQ_CLIENT23              = 0x0000002cu32,
GL1XC_PERF_SEL_REQ_CLIENT24              = 0x0000002du32,
GL1XC_PERF_SEL_REQ_CLIENT25              = 0x0000002eu32,
GL1XC_PERF_SEL_REQ_CLIENT26              = 0x0000002fu32,
GL1XC_PERF_SEL_REQ_CLIENT27              = 0x00000030u32,
GL1XC_PERF_SEL_UTCL0_REQUEST             = 0x00000031u32,
GL1XC_PERF_SEL_UTCL0_TRANSLATION_HIT     = 0x00000032u32,
GL1XC_PERF_SEL_UTCL0_TRANSLATION_MISS    = 0x00000033u32,
GL1XC_PERF_SEL_UTCL0_PERMISSION_MISS     = 0x00000034u32,
GL1XC_PERF_SEL_UTCL0_MISS_UNDER_MISS     = 0x00000035u32,
GL1XC_PERF_SEL_UTCL0_LFIFO_FULL          = 0x00000036u32,
GL1XC_PERF_SEL_UTCL0_STALL_INFLIGHT_MAX  = 0x00000037u32,
GL1XC_PERF_SEL_UTCL0_STALL_LFIFO_NOT_RES = 0x00000038u32,
GL1XC_PERF_SEL_UTCL0_STALL_LRU_INFLIGHT  = 0x00000039u32,
GL1XC_PERF_SEL_UTCL0_STALL_MISSFIFO_FULL = 0x0000003au32,
GL1XC_PERF_SEL_UTCL0_STALL_MULTI_MISS    = 0x0000003bu32,
GL1XC_PERF_SEL_UTCL0_STALL_UTCL1_REQ_OUT_OF_CREDITS = 0x0000003cu32,
GL1XC_PERF_SEL_UTCL0_UTCL1_PERM_FAULT    = 0x0000003du32,
GL1XC_PERF_SEL_CLIENT_UTCL0_INFLIGHT     = 0x0000003eu32,
GL1XC_PERF_SEL_UTCL0_UTCL1_INFLIGHT      = 0x0000003fu32,
GL1XC_PERF_SEL_UTCL0_INTERNAL_RETRY_REQ  = 0x00000040u32,
GL1XC_PERF_SEL_UTCL0_UTCL1_XNACK_RETRY_FAULT = 0x00000041u32,
GL1XC_PERF_SEL_UTCL0_UTCL1_XNACK_PRT_FAULT = 0x00000042u32,
GL1XC_PERF_SEL_UTCL0_UTCL1_XNACK_NO_RETRY_FAULT = 0x00000043u32,
GL1XC_PERF_SEL_UTCL0_GPA3_REQUEST        = 0x00000044u32,
}

/*******************************************************
 * GRBMH Enums
 *******************************************************/

/*
 * GRBMH_PERF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GRBMH_PERF_SEL {
GRBMH_PERF_SEL_COUNT                     = 0x00000000u32,
GRBMH_PERF_SEL_USER_DEFINED              = 0x00000001u32,
GRBMH_PERF_SEL_CB_BUSY                   = 0x00000002u32,
GRBMH_PERF_SEL_CB_CLEAN                  = 0x00000003u32,
GRBMH_PERF_SEL_DB_BUSY                   = 0x00000004u32,
GRBMH_PERF_SEL_DB_CLEAN                  = 0x00000005u32,
GRBMH_PERF_SEL_SC_BUSY                   = 0x00000006u32,
GRBMH_PERF_SEL_SC_CLEAN                  = 0x00000007u32,
GRBMH_PERF_SEL_SPI_BUSY                  = 0x00000009u32,
GRBMH_PERF_SEL_SX_BUSY                   = 0x0000000au32,
GRBMH_PERF_SEL_TA_BUSY                   = 0x0000000bu32,
GRBMH_PERF_SEL_EA_BUSY                   = 0x0000000cu32,
GRBMH_PERF_SEL_EA_LINK_BUSY              = 0x0000000du32,
GRBMH_PERF_SEL_PA_BUSY                   = 0x0000000eu32,
GRBMH_PERF_SEL_BCI_BUSY                  = 0x0000000fu32,
GRBMH_PERF_SEL_GL2A_BUSY                 = 0x00000010u32,
GRBMH_PERF_SEL_GL2C_BUSY                 = 0x00000011u32,
GRBMH_PERF_SEL_UTCL1_BUSY                = 0x00000012u32,
GRBMH_PERF_SEL_TCP_BUSY                  = 0x00000013u32,
GRBMH_PERF_SEL_GL1A_BUSY                 = 0x00000014u32,
GRBMH_PERF_SEL_GL1CC_BUSY                = 0x00000015u32,
GRBMH_PERF_SEL_GL1XCC_BUSY               = 0x00000016u32,
GRBMH_PERF_SEL_PC_BUSY                   = 0x00000017u32,
GRBMH_PERF_SEL_GE_BUSY                   = 0x00000018u32,
GRBMH_PERF_SEL_RLC_BUSY                  = 0x00000019u32,
}

/*******************************************************
 * TA Enums
 *******************************************************/

/*
 * TA_PERFCOUNT_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TA_PERFCOUNT_SEL {
TA_PERF_SEL_NULL                         = 0x00000000u32,
TA_PERF_SEL_image_sampler_has_offset_instructions = 0x00000001u32,
TA_PERF_SEL_image_sampler_has_bias_instructions = 0x00000002u32,
TA_PERF_SEL_image_sampler_has_reference_instructions = 0x00000003u32,
TA_PERF_SEL_image_sampler_has_ds_instructions = 0x00000004u32,
TA_PERF_SEL_image_sampler_has_dt_instructions = 0x00000005u32,
TA_PERF_SEL_image_sampler_has_dr_instructions = 0x00000006u32,
TA_PERF_SEL_gradient_busy                = 0x00000007u32,
TA_PERF_SEL_gradient_fifo_busy           = 0x00000008u32,
TA_PERF_SEL_lod_busy                     = 0x00000009u32,
TA_PERF_SEL_lod_fifo_busy                = 0x0000000au32,
TA_PERF_SEL_addresser_busy               = 0x0000000bu32,
TA_PERF_SEL_addresser_fifo_busy          = 0x0000000cu32,
TA_PERF_SEL_aligner_busy                 = 0x0000000du32,
TA_PERF_SEL_write_path_busy              = 0x0000000eu32,
TA_PERF_SEL_ta_busy                      = 0x0000000fu32,
TA_PERF_SEL_image_sampler_1_input_vgpr_instructions = 0x00000010u32,
TA_PERF_SEL_image_sampler_2_input_vgpr_instructions = 0x00000011u32,
TA_PERF_SEL_image_sampler_3_input_vgpr_instructions = 0x00000012u32,
TA_PERF_SEL_image_sampler_4_input_vgpr_instructions = 0x00000013u32,
TA_PERF_SEL_image_sampler_5_input_vgpr_instructions = 0x00000014u32,
TA_PERF_SEL_image_sampler_6_input_vgpr_instructions = 0x00000015u32,
TA_PERF_SEL_image_sampler_7_input_vgpr_instructions = 0x00000016u32,
TA_PERF_SEL_image_sampler_8_input_vgpr_instructions = 0x00000017u32,
TA_PERF_SEL_image_sampler_9_input_vgpr_instructions = 0x00000018u32,
TA_PERF_SEL_image_sampler_10_input_vgpr_instructions = 0x00000019u32,
TA_PERF_SEL_image_sampler_11_input_vgpr_instructions = 0x0000001au32,
TA_PERF_SEL_image_sampler_12_input_vgpr_instructions = 0x0000001bu32,
TA_PERF_SEL_image_sampler_has_t_instructions = 0x0000001cu32,
TA_PERF_SEL_image_sampler_has_r_instructions = 0x0000001du32,
TA_PERF_SEL_image_sampler_has_q_instructions = 0x0000001eu32,
TA_PERF_SEL_total_wavefronts             = 0x00000020u32,
TA_PERF_SEL_gradient_cycles              = 0x00000021u32,
TA_PERF_SEL_walker_cycles                = 0x00000022u32,
TA_PERF_SEL_aligner_cycles               = 0x00000023u32,
TA_PERF_SEL_image_wavefronts             = 0x00000024u32,
TA_PERF_SEL_image_read_wavefronts        = 0x00000025u32,
TA_PERF_SEL_image_store_wavefronts       = 0x00000026u32,
TA_PERF_SEL_image_atomic_wavefronts      = 0x00000027u32,
TA_PERF_SEL_image_sampler_total_cycles   = 0x00000028u32,
TA_PERF_SEL_image_nosampler_total_cycles = 0x00000029u32,
TA_PERF_SEL_flat_total_cycles            = 0x0000002au32,
TA_PERF_SEL_buffer_wavefronts            = 0x0000002cu32,
TA_PERF_SEL_buffer_load_wavefronts       = 0x0000002du32,
TA_PERF_SEL_buffer_store_wavefronts      = 0x0000002eu32,
TA_PERF_SEL_buffer_atomic_wavefronts     = 0x0000002fu32,
TA_PERF_SEL_buffer_total_cycles          = 0x00000031u32,
TA_PERF_SEL_buffer_1_address_input_vgpr_instructions = 0x00000032u32,
TA_PERF_SEL_buffer_2_address_input_vgpr_instructions = 0x00000033u32,
TA_PERF_SEL_buffer_has_index_instructions = 0x00000034u32,
TA_PERF_SEL_buffer_has_offset_instructions = 0x00000035u32,
TA_PERF_SEL_addr_stalled_by_tc_cycles    = 0x00000036u32,
TA_PERF_SEL_addr_stalled_by_td_cycles    = 0x00000037u32,
TA_PERF_SEL_image_sampler_wavefronts     = 0x00000038u32,
TA_PERF_SEL_addresser_stalled_by_aligner_only_cycles = 0x00000039u32,
TA_PERF_SEL_addresser_stalled_cycles     = 0x0000003au32,
TA_PERF_SEL_aniso_stalled_by_addresser_only_cycles = 0x0000003bu32,
TA_PERF_SEL_aniso_stalled_cycles         = 0x0000003cu32,
TA_PERF_SEL_deriv_stalled_by_aniso_only_cycles = 0x0000003du32,
TA_PERF_SEL_deriv_stalled_cycles         = 0x0000003eu32,
TA_PERF_SEL_aniso_gt1_cycle_quads        = 0x0000003fu32,
TA_PERF_SEL_color_1_cycle_quads          = 0x00000040u32,
TA_PERF_SEL_color_2_cycle_quads          = 0x00000041u32,
TA_PERF_SEL_color_3_cycle_quads          = 0x00000042u32,
TA_PERF_SEL_mip_1_cycle_quads            = 0x00000044u32,
TA_PERF_SEL_mip_2_cycle_quads            = 0x00000045u32,
TA_PERF_SEL_vol_1_cycle_quads            = 0x00000046u32,
TA_PERF_SEL_vol_2_cycle_quads            = 0x00000047u32,
TA_PERF_SEL_sampler_op_quads             = 0x00000048u32,
TA_PERF_SEL_mipmap_lod_0_samples         = 0x00000049u32,
TA_PERF_SEL_mipmap_lod_1_samples         = 0x0000004au32,
TA_PERF_SEL_mipmap_lod_2_samples         = 0x0000004bu32,
TA_PERF_SEL_mipmap_lod_3_samples         = 0x0000004cu32,
TA_PERF_SEL_mipmap_lod_4_samples         = 0x0000004du32,
TA_PERF_SEL_mipmap_lod_5_samples         = 0x0000004eu32,
TA_PERF_SEL_mipmap_lod_6_samples         = 0x0000004fu32,
TA_PERF_SEL_mipmap_lod_7_samples         = 0x00000050u32,
TA_PERF_SEL_mipmap_lod_8_samples         = 0x00000051u32,
TA_PERF_SEL_mipmap_lod_9_samples         = 0x00000052u32,
TA_PERF_SEL_mipmap_lod_10_samples        = 0x00000053u32,
TA_PERF_SEL_mipmap_lod_11_samples        = 0x00000054u32,
TA_PERF_SEL_mipmap_lod_12_samples        = 0x00000055u32,
TA_PERF_SEL_mipmap_lod_13_samples        = 0x00000056u32,
TA_PERF_SEL_mipmap_lod_14_samples        = 0x00000057u32,
TA_PERF_SEL_mipmap_invalid_samples       = 0x00000058u32,
TA_PERF_SEL_aniso_1_cycle_quads          = 0x00000059u32,
TA_PERF_SEL_aniso_2_cycle_quads          = 0x0000005au32,
TA_PERF_SEL_aniso_4_cycle_quads          = 0x0000005bu32,
TA_PERF_SEL_aniso_6_cycle_quads          = 0x0000005cu32,
TA_PERF_SEL_aniso_8_cycle_quads          = 0x0000005du32,
TA_PERF_SEL_aniso_10_cycle_quads         = 0x0000005eu32,
TA_PERF_SEL_aniso_12_cycle_quads         = 0x0000005fu32,
TA_PERF_SEL_aniso_14_cycle_quads         = 0x00000060u32,
TA_PERF_SEL_aniso_16_cycle_quads         = 0x00000061u32,
TA_PERF_SEL_store_write_data_input_cycles = 0x00000062u32,
TA_PERF_SEL_store_write_data_output_cycles = 0x00000063u32,
TA_PERF_SEL_flat_wavefronts              = 0x00000064u32,
TA_PERF_SEL_flat_load_wavefronts         = 0x00000065u32,
TA_PERF_SEL_flat_store_wavefronts        = 0x00000066u32,
TA_PERF_SEL_flat_atomic_wavefronts       = 0x00000067u32,
TA_PERF_SEL_flat_1_address_input_vgpr_instructions = 0x00000068u32,
TA_PERF_SEL_register_clk_valid_cycles    = 0x00000069u32,
TA_PERF_SEL_non_harvestable_clk_enabled_cycles = 0x0000006au32,
TA_PERF_SEL_harvestable_clk_enabled_cycles = 0x0000006bu32,
TA_PERF_SEL_flat_2_address_input_vgpr_instructions = 0x0000006cu32,
TA_PERF_SEL_boundary_non_harvestable_clk_enabled_cycles = 0x0000006du32,
TA_PERF_SEL_boundary_harvestable_clk_enabled_cycles = 0x0000006eu32,
TA_PERF_SEL_mipmap_lod_15_samples        = 0x00000070u32,
TA_PERF_SEL_mipmap_lod_16_samples        = 0x00000071u32,
TA_PERF_SEL_store_2_write_data_vgpr_instructions = 0x00000072u32,
TA_PERF_SEL_store_3_write_data_vgpr_instructions = 0x00000073u32,
TA_PERF_SEL_store_4_write_data_vgpr_instructions = 0x00000074u32,
TA_PERF_SEL_store_has_x_instructions     = 0x00000075u32,
TA_PERF_SEL_store_has_y_instructions     = 0x00000076u32,
TA_PERF_SEL_store_has_z_instructions     = 0x00000077u32,
TA_PERF_SEL_store_has_w_instructions     = 0x00000078u32,
TA_PERF_SEL_image_nosampler_has_t_instructions = 0x00000079u32,
TA_PERF_SEL_image_nosampler_has_r_instructions = 0x0000007au32,
TA_PERF_SEL_image_nosampler_has_q_instructions = 0x0000007bu32,
TA_PERF_SEL_image_nosampler_1_address_input_vgpr_instructions = 0x0000007cu32,
TA_PERF_SEL_image_nosampler_2_address_input_vgpr_instructions = 0x0000007du32,
TA_PERF_SEL_image_nosampler_3_address_input_vgpr_instructions = 0x0000007eu32,
TA_PERF_SEL_image_nosampler_4_address_input_vgpr_instructions = 0x0000007fu32,
TA_PERF_SEL_in_busy                      = 0x00000080u32,
TA_PERF_SEL_in_fifos_busy                = 0x00000081u32,
TA_PERF_SEL_in_cfifo_busy                = 0x00000082u32,
TA_PERF_SEL_in_qfifo_busy                = 0x00000083u32,
TA_PERF_SEL_in_wfifo_busy                = 0x00000084u32,
TA_PERF_SEL_in_rfifo_busy                = 0x00000085u32,
TA_PERF_SEL_bf_busy                      = 0x00000086u32,
TA_PERF_SEL_ns_busy                      = 0x00000087u32,
TA_PERF_SEL_smp_busy_ns_idle             = 0x00000088u32,
TA_PERF_SEL_smp_idle_ns_busy             = 0x00000089u32,
TA_PERF_SEL_vmemcmd_cycles               = 0x00000090u32,
TA_PERF_SEL_vmemreq_cycles               = 0x00000091u32,
TA_PERF_SEL_in_waiting_on_req_cycles     = 0x00000092u32,
TA_PERF_SEL_in_addr_cycles               = 0x00000096u32,
TA_PERF_SEL_in_data_cycles               = 0x00000097u32,
TA_PERF_SEL_point_sampled_quads          = 0x000000a0u32,
TA_PERF_SEL_atomic_2_write_data_vgpr_instructions = 0x000000a2u32,
TA_PERF_SEL_atomic_4_write_data_vgpr_instructions = 0x000000a3u32,
TA_PERF_SEL_atomic_write_data_input_cycles = 0x000000a4u32,
TA_PERF_SEL_atomic_write_data_output_cycles = 0x000000a5u32,
TA_PERF_SEL_num_unlit_nodes_ta_opt       = 0x000000adu32,
TA_PERF_SEL_num_nodes_invalidated_due_to_bad_input = 0x000000aeu32,
TA_PERF_SEL_num_nodes_invalidated_due_to_oob = 0x000000afu32,
TA_PERF_SEL_image_sampler_1_op_burst     = 0x000000c0u32,
TA_PERF_SEL_image_sampler_2to3_op_burst  = 0x000000c1u32,
TA_PERF_SEL_image_sampler_4to7_op_burst  = 0x000000c2u32,
TA_PERF_SEL_image_sampler_ge8_op_burst   = 0x000000c3u32,
TA_PERF_SEL_image_linked_1_op_burst      = 0x000000c4u32,
TA_PERF_SEL_image_linked_2to3_op_burst   = 0x000000c5u32,
TA_PERF_SEL_image_linked_4to7_op_burst   = 0x000000c6u32,
TA_PERF_SEL_image_linked_ge8_op_burst    = 0x000000c7u32,
TA_PERF_SEL_image_nosampler_1_op_burst   = 0x000000ccu32,
TA_PERF_SEL_image_nosampler_2to3_op_burst = 0x000000cdu32,
TA_PERF_SEL_image_nosampler_4to31_op_burst = 0x000000ceu32,
TA_PERF_SEL_image_nosampler_ge32_op_burst = 0x000000cfu32,
TA_PERF_SEL_buffer_flat_1_op_burst       = 0x000000d0u32,
TA_PERF_SEL_buffer_flat_2to3_op_burst    = 0x000000d1u32,
TA_PERF_SEL_buffer_flat_4to31_op_burst   = 0x000000d2u32,
TA_PERF_SEL_buffer_flat_ge32_op_burst    = 0x000000d3u32,
TA_PERF_SEL_write_1_op_burst             = 0x000000d4u32,
TA_PERF_SEL_write_2to3_op_burst          = 0x000000d5u32,
TA_PERF_SEL_write_4to31_op_burst         = 0x000000d6u32,
TA_PERF_SEL_write_ge32_op_burst          = 0x000000d7u32,
TA_PERF_SEL_ibubble_1_cycle_burst        = 0x000000d8u32,
TA_PERF_SEL_ibubble_2to3_cycle_burst     = 0x000000d9u32,
TA_PERF_SEL_ibubble_4to15_cycle_burst    = 0x000000dau32,
TA_PERF_SEL_ibubble_16to31_cycle_burst   = 0x000000dbu32,
TA_PERF_SEL_ibubble_32to63_cycle_burst   = 0x000000dcu32,
TA_PERF_SEL_ibubble_ge64_cycle_burst     = 0x000000ddu32,
TA_PERF_SEL_sampler_clk_valid_cycles     = 0x000000e0u32,
TA_PERF_SEL_nonsampler_clk_valid_cycles  = 0x000000e1u32,
TA_PERF_SEL_buffer_flat_clk_valid_cycles = 0x000000e2u32,
TA_PERF_SEL_write_data_clk_valid_cycles  = 0x000000e3u32,
TA_PERF_SEL_gradient_clk_valid_cycles    = 0x000000e4u32,
TA_PERF_SEL_lod_aniso_clk_valid_cycles   = 0x000000e5u32,
TA_PERF_SEL_sampler_addressing_clk_valid_cycles = 0x000000e6u32,
TA_PERF_SEL_sync_sampler_sstate_fifo_clk_valid_cycles = 0x000000e7u32,
TA_PERF_SEL_sync_sampler_cstate_fifo_clk_valid_cycles = 0x000000e8u32,
TA_PERF_SEL_sync_nonsampler_fifo_clk_valid_cycles = 0x000000e9u32,
TA_PERF_SEL_aligner_clk_valid_cycles     = 0x000000eau32,
TA_PERF_SEL_tcreq_clk_valid_cycles       = 0x000000ebu32,
}

/*
 * TEX_BC_SWIZZLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TEX_BC_SWIZZLE {
TEX_BC_Swizzle_XYZW                      = 0x00000000u32,
TEX_BC_Swizzle_XWYZ                      = 0x00000001u32,
TEX_BC_Swizzle_WZYX                      = 0x00000002u32,
TEX_BC_Swizzle_WXYZ                      = 0x00000003u32,
TEX_BC_Swizzle_ZYXW                      = 0x00000004u32,
TEX_BC_Swizzle_YXWZ                      = 0x00000005u32,
}

/*
 * TEX_BORDER_COLOR_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TEX_BORDER_COLOR_TYPE {
TEX_BorderColor_TransparentBlack         = 0x00000000u32,
TEX_BorderColor_OpaqueBlack              = 0x00000001u32,
TEX_BorderColor_OpaqueWhite              = 0x00000002u32,
TEX_BorderColor_Register                 = 0x00000003u32,
}

/*
 * TEX_CHROMA_KEY enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TEX_CHROMA_KEY {
TEX_ChromaKey_Disabled                   = 0x00000000u32,
TEX_ChromaKey_Kill                       = 0x00000001u32,
TEX_ChromaKey_Blend                      = 0x00000002u32,
TEX_ChromaKey_RESERVED_3                 = 0x00000003u32,
}

/*
 * TEX_CLAMP enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TEX_CLAMP {
TEX_Clamp_Repeat                         = 0x00000000u32,
TEX_Clamp_Mirror                         = 0x00000001u32,
TEX_Clamp_ClampToLast                    = 0x00000002u32,
TEX_Clamp_MirrorOnceToLast               = 0x00000003u32,
TEX_Clamp_ClampHalfToBorder              = 0x00000004u32,
TEX_Clamp_MirrorOnceHalfToBorder         = 0x00000005u32,
TEX_Clamp_ClampToBorder                  = 0x00000006u32,
TEX_Clamp_MirrorOnceToBorder             = 0x00000007u32,
}

/*
 * TEX_COORD_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TEX_COORD_TYPE {
TEX_CoordType_Unnormalized               = 0x00000000u32,
TEX_CoordType_Normalized                 = 0x00000001u32,
}

/*
 * TEX_DEPTH_COMPARE_FUNCTION enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TEX_DEPTH_COMPARE_FUNCTION {
TEX_DepthCompareFunction_Never           = 0x00000000u32,
TEX_DepthCompareFunction_Less            = 0x00000001u32,
TEX_DepthCompareFunction_Equal           = 0x00000002u32,
TEX_DepthCompareFunction_LessEqual       = 0x00000003u32,
TEX_DepthCompareFunction_Greater         = 0x00000004u32,
TEX_DepthCompareFunction_NotEqual        = 0x00000005u32,
TEX_DepthCompareFunction_GreaterEqual    = 0x00000006u32,
TEX_DepthCompareFunction_Always          = 0x00000007u32,
}

/*
 * TEX_FORMAT_COMP enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TEX_FORMAT_COMP {
TEX_FormatComp_Unsigned                  = 0x00000000u32,
TEX_FormatComp_Signed                    = 0x00000001u32,
TEX_FormatComp_UnsignedBiased            = 0x00000002u32,
TEX_FormatComp_RESERVED_3                = 0x00000003u32,
}

/*
 * TEX_MAX_ANISO_RATIO enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TEX_MAX_ANISO_RATIO {
TEX_MaxAnisoRatio_1to1                   = 0x00000000u32,
TEX_MaxAnisoRatio_2to1                   = 0x00000001u32,
TEX_MaxAnisoRatio_4to1                   = 0x00000002u32,
TEX_MaxAnisoRatio_8to1                   = 0x00000003u32,
TEX_MaxAnisoRatio_16to1                  = 0x00000004u32,
TEX_MaxAnisoRatio_RESERVED_5             = 0x00000005u32,
TEX_MaxAnisoRatio_RESERVED_6             = 0x00000006u32,
TEX_MaxAnisoRatio_RESERVED_7             = 0x00000007u32,
}

/*
 * TEX_MIP_FILTER enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TEX_MIP_FILTER {
TEX_MipFilter_None                       = 0x00000000u32,
TEX_MipFilter_Point                      = 0x00000001u32,
TEX_MipFilter_Linear                     = 0x00000002u32,
TEX_MipFilter_Point_Aniso_Adj            = 0x00000003u32,
}

/*
 * TEX_REQUEST_SIZE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TEX_REQUEST_SIZE {
TEX_RequestSize_32B                      = 0x00000000u32,
TEX_RequestSize_64B                      = 0x00000001u32,
TEX_RequestSize_128B                     = 0x00000002u32,
TEX_RequestSize_2X64B                    = 0x00000003u32,
}

/*
 * TEX_SAMPLER_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TEX_SAMPLER_TYPE {
TEX_SamplerType_Invalid                  = 0x00000000u32,
TEX_SamplerType_Valid                    = 0x00000001u32,
}

/*
 * TEX_XY_FILTER enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TEX_XY_FILTER {
TEX_XYFilter_Point                       = 0x00000000u32,
TEX_XYFilter_Linear                      = 0x00000001u32,
TEX_XYFilter_AnisoPoint                  = 0x00000002u32,
TEX_XYFilter_AnisoLinear                 = 0x00000003u32,
}

/*
 * TEX_Z_FILTER enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TEX_Z_FILTER {
TEX_ZFilter_None                         = 0x00000000u32,
TEX_ZFilter_Point                        = 0x00000001u32,
TEX_ZFilter_Linear                       = 0x00000002u32,
TEX_ZFilter_RESERVED_3                   = 0x00000003u32,
}

/*
 * TVX_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TVX_TYPE {
TVX_Type_InvalidTextureResource          = 0x00000000u32,
TVX_Type_InvalidVertexBuffer             = 0x00000001u32,
TVX_Type_ValidTextureResource            = 0x00000002u32,
TVX_Type_ValidVertexBuffer               = 0x00000003u32,
}

/*
 * TA_TC_ADDR_MODES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TA_TC_ADDR_MODES {
TA_TC_ADDR_MODE_DEFAULT                  = 0x00000000u32,
TA_TC_ADDR_MODE_COMP0                    = 0x00000001u32,
TA_TC_ADDR_MODE_COMP1                    = 0x00000002u32,
TA_TC_ADDR_MODE_COMP2                    = 0x00000003u32,
TA_TC_ADDR_MODE_COMP3                    = 0x00000004u32,
TA_TC_ADDR_MODE_UNALIGNED                = 0x00000005u32,
TA_TC_ADDR_MODE_BORDER_COLOR             = 0x00000006u32,
}

/*
 * TA_TC_REQ_MODES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TA_TC_REQ_MODES {
TA_TC_REQ_MODE_BORDER                    = 0x00000000u32,
TA_TC_REQ_MODE_TEX2                      = 0x00000001u32,
TA_TC_REQ_MODE_TEX1                      = 0x00000002u32,
TA_TC_REQ_MODE_TEX0                      = 0x00000003u32,
TA_TC_REQ_MODE_NORMAL                    = 0x00000004u32,
TA_TC_REQ_MODE_DWORD                     = 0x00000005u32,
TA_TC_REQ_MODE_BYTE                      = 0x00000006u32,
TA_TC_REQ_MODE_BYTE_NV                   = 0x00000007u32,
}

/*
 * TCP_CACHE_POLICIES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TCP_CACHE_POLICIES {
TCP_CACHE_POLICY_MISS_LRU                = 0x00000000u32,
TCP_CACHE_POLICY_MISS_EVICT              = 0x00000001u32,
TCP_CACHE_POLICY_HIT_LRU                 = 0x00000002u32,
TCP_CACHE_POLICY_HIT_EVICT               = 0x00000003u32,
}

/*
 * TCP_CACHE_STORE_POLICIES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TCP_CACHE_STORE_POLICIES {
TCP_CACHE_STORE_POLICY_WT_LRU            = 0x00000000u32,
TCP_CACHE_STORE_POLICY_WT_EVICT          = 0x00000001u32,
}

/*
 * TCP_COMPRESSION_BYPASS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TCP_COMPRESSION_BYPASS {
TCP_COMPRESSION_BYPASS_DIS               = 0x00000000u32,
TCP_COMPRESSION_BYPASS_EN                = 0x00000001u32,
}

/*
 * TCP_COMPRESSION_OVERRIDE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TCP_COMPRESSION_OVERRIDE {
TCP_COMPRESSION_OVERRIDE_DIS             = 0x00000000u32,
TCP_COMPRESSION_OVERRIDE_EN              = 0x00000001u32,
}

/*
 * TCP_OPCODE_TYPE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TCP_OPCODE_TYPE {
TCP_OPCODE_READ                          = 0x00000000u32,
TCP_OPCODE_WRITE                         = 0x00000001u32,
TCP_OPCODE_ATOMIC                        = 0x00000002u32,
TCP_OPCODE_INV                           = 0x00000003u32,
TCP_OPCODE_ATOMIC_CMPSWAP                = 0x00000004u32,
TCP_OPCODE_SAMPLER                       = 0x00000005u32,
TCP_OPCODE_LOAD                          = 0x00000006u32,
TCP_OPCODE_GATHERH                       = 0x00000007u32,
}

/*
 * TCP_PERFCOUNT_SELECT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TCP_PERFCOUNT_SELECT {
TCP_PERF_SEL_GATE_EN1                    = 0x00000000u32,
TCP_PERF_SEL_GATE_EN2                    = 0x00000001u32,
TCP_PERF_SEL_TA_REQ                      = 0x00000002u32,
TCP_PERF_SEL_TA_REQ_STATE_READ           = 0x00000003u32,
TCP_PERF_SEL_TA_REQ_READ                 = 0x00000004u32,
TCP_PERF_SEL_TA_REQ_WRITE                = 0x00000005u32,
TCP_PERF_SEL_TA_REQ_ATOMIC_WITH_RET      = 0x00000006u32,
TCP_PERF_SEL_TA_REQ_ATOMIC_WITHOUT_RET   = 0x00000007u32,
TCP_PERF_SEL_TA_REQ_GL0_INV              = 0x00000008u32,
TCP_PERF_SEL_REQ                         = 0x00000009u32,
TCP_PERF_SEL_REQ_READ                    = 0x0000000au32,
TCP_PERF_SEL_REQ_READ_HIT_LRU            = 0x0000000cu32,
TCP_PERF_SEL_REQ_READ_MISS_EVICT         = 0x0000000du32,
TCP_PERF_SEL_REQ_WRITE                   = 0x0000000eu32,
TCP_PERF_SEL_REQ_WRITE_MISS_EVICT        = 0x0000000fu32,
TCP_PERF_SEL_REQ_NON_READ                = 0x00000010u32,
TCP_PERF_SEL_REQ_MISS                    = 0x00000011u32,
TCP_PERF_SEL_REQ_TAGBANK0_SET0           = 0x00000012u32,
TCP_PERF_SEL_REQ_TAGBANK0_SET1           = 0x00000013u32,
TCP_PERF_SEL_REQ_TAGBANK1_SET0           = 0x00000014u32,
TCP_PERF_SEL_REQ_TAGBANK1_SET1           = 0x00000015u32,
TCP_PERF_SEL_REQ_TAGBANK2_SET0           = 0x00000016u32,
TCP_PERF_SEL_REQ_TAGBANK2_SET1           = 0x00000017u32,
TCP_PERF_SEL_REQ_TAGBANK3_SET0           = 0x00000018u32,
TCP_PERF_SEL_REQ_TAGBANK3_SET1           = 0x00000019u32,
TCP_PERF_SEL_REQ_MISS_TAGBANK0           = 0x0000001au32,
TCP_PERF_SEL_REQ_MISS_TAGBANK1           = 0x0000001bu32,
TCP_PERF_SEL_REQ_MISS_TAGBANK2           = 0x0000001cu32,
TCP_PERF_SEL_REQ_MISS_TAGBANK3           = 0x0000001du32,
TCP_PERF_SEL_GL1_REQ_READ                = 0x0000001eu32,
TCP_PERF_SEL_GL1_REQ_READ_128B           = 0x0000001fu32,
TCP_PERF_SEL_GL1_REQ_READ_64B            = 0x00000020u32,
TCP_PERF_SEL_GL1_REQ_WRITE               = 0x00000021u32,
TCP_PERF_SEL_GL1_REQ_ATOMIC_WITH_RET     = 0x00000022u32,
TCP_PERF_SEL_GL1_REQ_ATOMIC_WITHOUT_RET  = 0x00000023u32,
TCP_PERF_SEL_GL1_READ_LATENCY            = 0x00000024u32,
TCP_PERF_SEL_GL1_WRITE_LATENCY           = 0x00000025u32,
TCP_PERF_SEL_TCP_LATENCY                 = 0x00000026u32,
TCP_PERF_SEL_TCP_TA_REQ_STALL            = 0x00000027u32,
TCP_PERF_SEL_TA_TCP_REQ_STARVE           = 0x00000028u32,
TCP_PERF_SEL_DATA_FIFO_STALL             = 0x00000029u32,
TCP_PERF_SEL_LOD_STALL                   = 0x0000002au32,
TCP_PERF_SEL_POWER_STALL                 = 0x0000002bu32,
TCP_PERF_SEL_ALLOC_STALL                 = 0x0000002cu32,
TCP_PERF_SEL_READ_TAGCONFLICT_STALL      = 0x0000002eu32,
TCP_PERF_SEL_WRITE_TAGCONFLICT_STALL     = 0x0000002fu32,
TCP_PERF_SEL_ATOMIC_TAGCONFLICT_STALL    = 0x00000030u32,
TCP_PERF_SEL_LFIFO_STALL                 = 0x00000031u32,
TCP_PERF_SEL_MEM_REQ_FIFO_STALL          = 0x00000032u32,
TCP_PERF_SEL_GL1_TCP_BACK_PRESSURE       = 0x00000033u32,
TCP_PERF_SEL_GL1_TCP_RDRET_STALL         = 0x00000034u32,
TCP_PERF_SEL_GL1_GRANT_READ_STALL        = 0x00000035u32,
TCP_PERF_SEL_GL1_PENDING_STALL           = 0x00000036u32,
TCP_PERF_SEL_TD_DATA_CYCLE_STALL         = 0x00000037u32,
TCP_PERF_SEL_COMP_TEX_LOAD_STALL         = 0x00000038u32,
TCP_PERF_SEL_READ_DATACONFLICT_STALL     = 0x00000039u32,
TCP_PERF_SEL_WRITE_DATACONFLICT_STALL    = 0x0000003au32,
TCP_PERF_SEL_TD_TCP_STALL                = 0x0000003bu32,
TCP_PERF_SEL_TA_REQ_BUFFERNOP            = 0x0000003cu32,
TCP_PERF_SEL_WRITECOMBINE_ENDCLAUSE      = 0x0000003du32,
TCP_PERF_SEL_TAGFAKE_EOW                 = 0x0000003eu32,
TCP_PERF_SEL_REQ_TAG_MATCH_AND_NOT_VALID = 0x0000003fu32,
TCP_PERF_SEL_BURST_BIN_WRITECOMBINE_0    = 0x00000040u32,
TCP_PERF_SEL_BURST_BIN_WRITECOMBINE_1to2 = 0x00000041u32,
TCP_PERF_SEL_BURST_BIN_WRITECOMBINE_3to4 = 0x00000042u32,
TCP_PERF_SEL_BURST_BIN_WRITECOMBINE_5to8 = 0x00000043u32,
TCP_PERF_SEL_BURST_BIN_WRITECOMBINE_9to16 = 0x00000044u32,
TCP_PERF_SEL_BURST_BIN_READHIT_0         = 0x00000046u32,
TCP_PERF_SEL_BURST_BIN_READHIT_1         = 0x00000047u32,
TCP_PERF_SEL_BURST_BIN_READHIT_2to4      = 0x00000048u32,
TCP_PERF_SEL_BURST_BIN_READHIT_5to8      = 0x00000049u32,
TCP_PERF_SEL_BURST_BIN_READHIT_9to16     = 0x0000004au32,
TCP_PERF_SEL_BURST_BIN_READHIT_gt16      = 0x0000004bu32,
TCP_PERF_SEL_TA_TC_REQ_EN_SUM            = 0x0000004cu32,
TCP_PERF_SEL_GL1_REQ_LU                  = 0x0000004du32,
TCP_PERF_SEL_REQ_TAG_MATCH_AND_LU_INVALIDATE = 0x0000004eu32,
}

/*
 * TCP_WATCH_MODES enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TCP_WATCH_MODES {
TCP_WATCH_MODE_READ                      = 0x00000000u32,
TCP_WATCH_MODE_NONREAD                   = 0x00000001u32,
TCP_WATCH_MODE_ATOMIC                    = 0x00000002u32,
TCP_WATCH_MODE_ALL                       = 0x00000003u32,
}

/*
 * TCP_WRITE_COMPRESSION_DISABLE enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TCP_WRITE_COMPRESSION_DISABLE {
TCP_WRITE_COMPRESSION_DISABLE_DIS        = 0x00000000u32,
TCP_WRITE_COMPRESSION_DISABLE_EN         = 0x00000001u32,
}

/*******************************************************
 * TD Enums
 *******************************************************/

/*
 * TD_PERFCOUNT_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TD_PERFCOUNT_SEL {
TD_PERF_SEL_none                         = 0x00000000u32,
TD_PERF_SEL_td_busy                      = 0x00000001u32,
TD_PERF_SEL_input_busy                   = 0x00000002u32,
TD_PERF_SEL_sampler_lerp_busy            = 0x00000003u32,
TD_PERF_SEL_sampler_out_busy             = 0x00000004u32,
TD_PERF_SEL_nofilter_busy                = 0x00000005u32,
TD_PERF_SEL_sampler_core_sclk_en         = 0x00000007u32,
TD_PERF_SEL_sampler_preformatter_sclk_en = 0x00000008u32,
TD_PERF_SEL_sampler_bilerp_sclk_en       = 0x00000009u32,
TD_PERF_SEL_sampler_bypass_sclk_en       = 0x0000000au32,
TD_PERF_SEL_sampler_minmax_sclk_en       = 0x0000000bu32,
TD_PERF_SEL_sampler_accum_sclk_en        = 0x0000000cu32,
TD_PERF_SEL_sampler_format_flt_sclk_en   = 0x0000000du32,
TD_PERF_SEL_sampler_format_fxdpt_sclk_en = 0x0000000eu32,
TD_PERF_SEL_sampler_out_sclk_en          = 0x0000000fu32,
TD_PERF_SEL_nofilter_sclk_en             = 0x00000010u32,
TD_PERF_SEL_nofilter_d32_sclk_en         = 0x00000011u32,
TD_PERF_SEL_nofilter_d16_sclk_en         = 0x00000012u32,
TD_PERF_SEL_sampler_sclk_on_nofilter_sclk_off = 0x0000001au32,
TD_PERF_SEL_nofilter_sclk_on_sampler_sclk_off = 0x0000001bu32,
TD_PERF_SEL_all_pipes_sclk_on_at_same_time = 0x0000001cu32,
TD_PERF_SEL_core_state_ram_max_cnt       = 0x00000020u32,
TD_PERF_SEL_core_state_rams_read         = 0x00000021u32,
TD_PERF_SEL_weight_data_rams_read        = 0x00000022u32,
TD_PERF_SEL_reference_data_rams_read     = 0x00000023u32,
TD_PERF_SEL_tc_td_ram_fifo_full          = 0x00000024u32,
TD_PERF_SEL_tc_td_ram_fifo_max_cnt       = 0x00000025u32,
TD_PERF_SEL_tc_td_data_fifo_full         = 0x00000026u32,
TD_PERF_SEL_input_state_fifo_full        = 0x00000027u32,
TD_PERF_SEL_ta_data_stall                = 0x00000028u32,
TD_PERF_SEL_tc_data_stall                = 0x00000029u32,
TD_PERF_SEL_tc_ram_stall                 = 0x0000002au32,
TD_PERF_SEL_lds_stall                    = 0x0000002bu32,
TD_PERF_SEL_sampler_pkr_full             = 0x0000002cu32,
TD_PERF_SEL_sampler_pkr_full_due_to_arb  = 0x0000002du32,
TD_PERF_SEL_nofilter_pkr_full            = 0x0000002eu32,
TD_PERF_SEL_nofilter_pkr_full_due_to_arb = 0x0000002fu32,
TD_PERF_SEL_gather4_instr                = 0x00000032u32,
TD_PERF_SEL_gather4h_instr               = 0x00000033u32,
TD_PERF_SEL_getlod_instr                 = 0x00000034u32,
TD_PERF_SEL_sample_instr                 = 0x00000036u32,
TD_PERF_SEL_sample_c_instr               = 0x00000037u32,
TD_PERF_SEL_load_instr                   = 0x00000038u32,
TD_PERF_SEL_ps_load_instr                = 0x00000039u32,
TD_PERF_SEL_write_ack_instr              = 0x0000003au32,
TD_PERF_SEL_d16_en_instr                 = 0x0000003bu32,
TD_PERF_SEL_bypassLerp_instr             = 0x0000003cu32,
TD_PERF_SEL_min_max_filter_instr         = 0x0000003du32,
TD_PERF_SEL_one_comp_return_instr        = 0x0000003eu32,
TD_PERF_SEL_two_comp_return_instr        = 0x0000003fu32,
TD_PERF_SEL_three_comp_return_instr      = 0x00000040u32,
TD_PERF_SEL_four_comp_return_instr       = 0x00000041u32,
TD_PERF_SEL_user_defined_border          = 0x00000042u32,
TD_PERF_SEL_white_border                 = 0x00000043u32,
TD_PERF_SEL_opaque_black_border          = 0x00000044u32,
TD_PERF_SEL_lod_warn_from_ta             = 0x00000045u32,
TD_PERF_SEL_instruction_dest_is_lds      = 0x00000046u32,
TD_PERF_SEL_td_cycling_of_nofilter_instr_2cycles = 0x00000047u32,
TD_PERF_SEL_td_cycling_of_nofilter_instr_4cycles = 0x00000048u32,
TD_PERF_SEL_tc_cycling_of_nofilter_instr_2cycles = 0x00000049u32,
TD_PERF_SEL_tc_cycling_of_nofilter_instr_4cycles = 0x0000004au32,
TD_PERF_SEL_out_of_order_instr           = 0x0000004bu32,
TD_PERF_SEL_total_num_instr              = 0x0000004cu32,
TD_PERF_SEL_total_num_instr_with_perf_wdw = 0x0000004du32,
TD_PERF_SEL_total_num_sampler_instr      = 0x0000004eu32,
TD_PERF_SEL_total_num_sampler_instr_with_perf_wdw = 0x0000004fu32,
TD_PERF_SEL_total_num_nofilter_instr     = 0x00000050u32,
TD_PERF_SEL_total_num_nofilter_instr_with_perf_wdw = 0x00000051u32,
TD_PERF_SEL_mixmode_instr                = 0x00000054u32,
TD_PERF_SEL_mixmode_resource             = 0x00000055u32,
TD_PERF_SEL_status_packet                = 0x00000056u32,
TD_PERF_SEL_done_scoreboard_max_stored_cnt = 0x00000059u32,
TD_PERF_SEL_done_scoreboard_max_waiting_cnt = 0x0000005au32,
TD_PERF_SEL_done_scoreboard_not_empty    = 0x0000005bu32,
TD_PERF_SEL_done_scoreboard_is_full      = 0x0000005cu32,
TD_PERF_SEL_done_scoreboard_bp_due_to_ooo = 0x0000005du32,
TD_PERF_SEL_done_scoreboard_bp_due_to_lds = 0x0000005eu32,
TD_PERF_SEL_nofilter_formatters_turned_on = 0x0000005fu32,
TD_PERF_SEL_nofilter_insert_extra_comps  = 0x00000060u32,
TD_PERF_SEL_nofilter_popcount_dmask_gt_num_comp_of_fmt = 0x00000061u32,
TD_PERF_SEL_nofilter_popcount_dmask_lt_num_comp_of_fmt = 0x00000062u32,
TD_PERF_SEL_msaa_load_instr              = 0x00000063u32,
TD_PERF_SEL_blend_prt_with_prt_default_0 = 0x00000064u32,
TD_PERF_SEL_resmap_instr                 = 0x00000066u32,
TD_PERF_SEL_prt_ack_instr                = 0x00000067u32,
TD_PERF_SEL_resmap_with_volume_filtering = 0x00000068u32,
TD_PERF_SEL_resmap_with_aniso_filtering  = 0x00000069u32,
TD_PERF_SEL_resmap_with_no_more_filtering = 0x0000006au32,
TD_PERF_SEL_resmap_with_cubemap_corner   = 0x0000006bu32,
TD_PERF_SEL_burst_bin_preempting_nofilter_1 = 0x00000083u32,
TD_PERF_SEL_burst_bin_preempting_nofilter_2to4 = 0x00000084u32,
TD_PERF_SEL_burst_bin_preempting_nofilter_5to7 = 0x00000085u32,
TD_PERF_SEL_burst_bin_preempting_nofilter_8to16 = 0x00000086u32,
TD_PERF_SEL_burst_bin_preempting_nofilter_gt16 = 0x00000087u32,
TD_PERF_SEL_burst_bin_sampler_1          = 0x00000088u32,
TD_PERF_SEL_burst_bin_sampler_2to8       = 0x00000089u32,
TD_PERF_SEL_burst_bin_sampler_9to16      = 0x0000008au32,
TD_PERF_SEL_burst_bin_sampler_gt16       = 0x0000008bu32,
TD_PERF_SEL_burst_bin_gather_1           = 0x0000008cu32,
TD_PERF_SEL_burst_bin_gather_2to8        = 0x0000008du32,
TD_PERF_SEL_burst_bin_gather_9to16       = 0x0000008eu32,
TD_PERF_SEL_burst_bin_gather_gt16        = 0x0000008fu32,
TD_PERF_SEL_burst_bin_nofilter_1         = 0x00000090u32,
TD_PERF_SEL_burst_bin_nofilter_2to4      = 0x00000091u32,
TD_PERF_SEL_burst_bin_nofilter_5to7      = 0x00000092u32,
TD_PERF_SEL_burst_bin_nofilter_8to16     = 0x00000093u32,
TD_PERF_SEL_burst_bin_nofilter_gt16      = 0x00000094u32,
TD_PERF_SEL_bubble_bin_ta_waiting_for_tc_data_0 = 0x000000aau32,
TD_PERF_SEL_bubble_bin_ta_waiting_for_tc_data_1 = 0x000000abu32,
TD_PERF_SEL_bubble_bin_ta_waiting_for_tc_data_2to31 = 0x000000acu32,
TD_PERF_SEL_bubble_bin_ta_waiting_for_tc_data_32to127 = 0x000000adu32,
TD_PERF_SEL_bubble_bin_ta_waiting_for_tc_data_128to511 = 0x000000aeu32,
TD_PERF_SEL_bubble_bin_ta_waiting_for_tc_data_gt511 = 0x000000afu32,
TD_PERF_SEL_bubble_bin_lds_stall_1to3    = 0x000000b0u32,
TD_PERF_SEL_bubble_bin_lds_stall_4to7    = 0x000000b1u32,
TD_PERF_SEL_bubble_bin_lds_stall_8to15   = 0x000000b2u32,
TD_PERF_SEL_bubble_bin_lds_stall_gt15    = 0x000000b3u32,
TD_PERF_SEL_preempting_nofilter_max_cnt  = 0x000000b4u32,
TD_PERF_SEL_sampler_lerp0_active         = 0x000000b5u32,
TD_PERF_SEL_sampler_lerp1_active         = 0x000000b6u32,
TD_PERF_SEL_sampler_lerp2_active         = 0x000000b7u32,
TD_PERF_SEL_sampler_lerp3_active         = 0x000000b8u32,
TD_PERF_SEL_sampler_lerp4_active         = 0x000000b9u32,
TD_PERF_SEL_sampler_lerp5_active         = 0x000000bau32,
TD_PERF_SEL_sampler_lerp6_active         = 0x000000bbu32,
TD_PERF_SEL_sampler_lerp7_active         = 0x000000bcu32,
TD_PERF_SEL_nofilter_total_num_comps_to_lds = 0x000000bdu32,
TD_PERF_SEL_nofilter_byte_cycling_4cycles = 0x000000beu32,
TD_PERF_SEL_nofilter_byte_cycling_8cycles = 0x000000bfu32,
TD_PERF_SEL_nofilter_byte_cycling_16cycles = 0x000000c0u32,
TD_PERF_SEL_nofilter_dword_cycling_2cycles = 0x000000c1u32,
TD_PERF_SEL_nofilter_dword_cycling_4cycles = 0x000000c2u32,
TD_PERF_SEL_input_bp_due_to_done_scoreboard_full = 0x000000c3u32,
TD_PERF_SEL_store_preempts_a_load        = 0x000000c8u32,
TD_PERF_SEL_sample_2x_instr              = 0x000000c9u32,
TD_PERF_SEL_gather4_2x_instr             = 0x000000cau32,
TD_PERF_SEL_gather4h_2x_instr            = 0x000000cbu32,
TD_PERF_SEL_getlod_2x_instr              = 0x000000ccu32,
TD_PERF_SEL_resmap_2x_instr              = 0x000000cdu32,
TD_PERF_SEL_2x_sampler_op_with_1_unlit_quad = 0x000000ceu32,
TD_PERF_SEL_2x_sampler_op_with_both_quads_unlit = 0x000000cfu32,
TD_PERF_SEL_tri_proc_node_override_slot0 = 0x000000d0u32,
TD_PERF_SEL_tri_run_intersect_ahs_slot0  = 0x000000d1u32,
TD_PERF_SEL_tri_run_ahs_slot0            = 0x000000d2u32,
TD_PERF_SEL_tri_proc_node_override_slot1 = 0x000000e7u32,
TD_PERF_SEL_tri_run_intersect_ahs_slot1  = 0x000000e8u32,
TD_PERF_SEL_tri_run_ahs_slot1            = 0x000000e9u32,
TD_PERF_SEL_instance_mask_culled         = 0x000000f1u32,
TD_PERF_SEL_box_opaque_culled            = 0x000000f2u32,
TD_PERF_SEL_box_non_opaque_culled        = 0x000000f3u32,
TD_PERF_SEL_box_with_triangle_children_only_culled = 0x000000f4u32,
TD_PERF_SEL_box_with_procedural_children_only_culled = 0x000000f5u32,
TD_PERF_SEL_triangle_opaque_culled       = 0x000000f6u32,
TD_PERF_SEL_triangle_non_opaque_culled   = 0x000000f7u32,
TD_PERF_SEL_triangle_front_facing_culled = 0x000000f8u32,
TD_PERF_SEL_triangle_back_facing_culled  = 0x000000f9u32,
}

/*
 * GL2A_PERF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GL2A_PERF_SEL {
GL2A_PERF_SEL_NONE                       = 0x00000000u32,
GL2A_PERF_SEL_CYCLE                      = 0x00000001u32,
GL2A_PERF_SEL_BUSY                       = 0x00000002u32,
GL2A_PERF_SEL_REQ_GL2C0                  = 0x00000003u32,
GL2A_PERF_SEL_REQ_GL2C1                  = 0x00000004u32,
GL2A_PERF_SEL_REQ_GL2C2                  = 0x00000005u32,
GL2A_PERF_SEL_REQ_GL2C3                  = 0x00000006u32,
GL2A_PERF_SEL_REQ_GL2C4                  = 0x00000007u32,
GL2A_PERF_SEL_REQ_GL2C5                  = 0x00000008u32,
GL2A_PERF_SEL_REQ_GL2C6                  = 0x00000009u32,
GL2A_PERF_SEL_REQ_GL2C7                  = 0x0000000au32,
GL2A_PERF_SEL_REQ_BURST_GL2C0            = 0x00000013u32,
GL2A_PERF_SEL_REQ_BURST_GL2C1            = 0x00000014u32,
GL2A_PERF_SEL_REQ_BURST_GL2C2            = 0x00000015u32,
GL2A_PERF_SEL_REQ_BURST_GL2C3            = 0x00000016u32,
GL2A_PERF_SEL_REQ_BURST_GL2C4            = 0x00000017u32,
GL2A_PERF_SEL_REQ_BURST_GL2C5            = 0x00000018u32,
GL2A_PERF_SEL_REQ_BURST_GL2C6            = 0x00000019u32,
GL2A_PERF_SEL_REQ_BURST_GL2C7            = 0x0000001au32,
GL2A_PERF_SEL_REQ_STALL_GL2C0            = 0x0000001bu32,
GL2A_PERF_SEL_REQ_STALL_GL2C1            = 0x0000001cu32,
GL2A_PERF_SEL_REQ_STALL_GL2C2            = 0x0000001du32,
GL2A_PERF_SEL_REQ_STALL_GL2C3            = 0x0000001eu32,
GL2A_PERF_SEL_REQ_STALL_GL2C4            = 0x0000001fu32,
GL2A_PERF_SEL_REQ_STALL_GL2C5            = 0x00000020u32,
GL2A_PERF_SEL_REQ_STALL_GL2C6            = 0x00000021u32,
GL2A_PERF_SEL_REQ_STALL_GL2C7            = 0x00000022u32,
GL2A_PERF_SEL_RTN_STALL_GL2C0            = 0x00000023u32,
GL2A_PERF_SEL_RTN_STALL_GL2C1            = 0x00000024u32,
GL2A_PERF_SEL_RTN_STALL_GL2C2            = 0x00000025u32,
GL2A_PERF_SEL_RTN_STALL_GL2C3            = 0x00000026u32,
GL2A_PERF_SEL_RTN_STALL_GL2C4            = 0x00000027u32,
GL2A_PERF_SEL_RTN_STALL_GL2C5            = 0x00000028u32,
GL2A_PERF_SEL_RTN_STALL_GL2C6            = 0x00000029u32,
GL2A_PERF_SEL_RTN_STALL_GL2C7            = 0x0000002au32,
GL2A_PERF_SEL_RTN_CLIENT0                = 0x0000002bu32,
GL2A_PERF_SEL_RTN_CLIENT1                = 0x0000002cu32,
GL2A_PERF_SEL_RTN_CLIENT2                = 0x0000002du32,
GL2A_PERF_SEL_RTN_CLIENT3                = 0x0000002eu32,
GL2A_PERF_SEL_RTN_CLIENT4                = 0x0000002fu32,
GL2A_PERF_SEL_RTN_CLIENT5                = 0x00000030u32,
GL2A_PERF_SEL_RTN_CLIENT6                = 0x00000031u32,
GL2A_PERF_SEL_RTN_CLIENT7                = 0x00000032u32,
GL2A_PERF_SEL_RTN_CLIENT8                = 0x00000033u32,
GL2A_PERF_SEL_RTN_CLIENT9                = 0x00000034u32,
GL2A_PERF_SEL_RTN_CLIENT10               = 0x00000035u32,
GL2A_PERF_SEL_RTN_CLIENT11               = 0x00000036u32,
GL2A_PERF_SEL_RTN_CLIENT12               = 0x00000037u32,
GL2A_PERF_SEL_RTN_CLIENT13               = 0x00000038u32,
GL2A_PERF_SEL_RTN_CLIENT14               = 0x00000039u32,
GL2A_PERF_SEL_RTN_CLIENT15               = 0x0000003au32,
GL2A_PERF_SEL_RTN_ARB_COLLISION_CLIENT0  = 0x0000003bu32,
GL2A_PERF_SEL_RTN_ARB_COLLISION_CLIENT1  = 0x0000003cu32,
GL2A_PERF_SEL_RTN_ARB_COLLISION_CLIENT2  = 0x0000003du32,
GL2A_PERF_SEL_RTN_ARB_COLLISION_CLIENT3  = 0x0000003eu32,
GL2A_PERF_SEL_RTN_ARB_COLLISION_CLIENT4  = 0x0000003fu32,
GL2A_PERF_SEL_RTN_ARB_COLLISION_CLIENT5  = 0x00000040u32,
GL2A_PERF_SEL_RTN_ARB_COLLISION_CLIENT6  = 0x00000041u32,
GL2A_PERF_SEL_RTN_ARB_COLLISION_CLIENT7  = 0x00000042u32,
GL2A_PERF_SEL_RTN_ARB_COLLISION_CLIENT8  = 0x00000043u32,
GL2A_PERF_SEL_RTN_ARB_COLLISION_CLIENT9  = 0x00000044u32,
GL2A_PERF_SEL_RTN_ARB_COLLISION_CLIENT10 = 0x00000045u32,
GL2A_PERF_SEL_RTN_ARB_COLLISION_CLIENT11 = 0x00000046u32,
GL2A_PERF_SEL_RTN_ARB_COLLISION_CLIENT12 = 0x00000047u32,
GL2A_PERF_SEL_RTN_ARB_COLLISION_CLIENT13 = 0x00000048u32,
GL2A_PERF_SEL_RTN_ARB_COLLISION_CLIENT14 = 0x00000049u32,
GL2A_PERF_SEL_RTN_ARB_COLLISION_CLIENT15 = 0x0000004au32,
GL2A_PERF_SEL_REQ_BURST_CLIENT0          = 0x0000004bu32,
GL2A_PERF_SEL_REQ_BURST_CLIENT1          = 0x0000004cu32,
GL2A_PERF_SEL_REQ_BURST_CLIENT2          = 0x0000004du32,
GL2A_PERF_SEL_REQ_BURST_CLIENT3          = 0x0000004eu32,
GL2A_PERF_SEL_REQ_BURST_CLIENT4          = 0x0000004fu32,
GL2A_PERF_SEL_REQ_BURST_CLIENT5          = 0x00000050u32,
GL2A_PERF_SEL_REQ_BURST_CLIENT6          = 0x00000051u32,
GL2A_PERF_SEL_REQ_BURST_CLIENT7          = 0x00000052u32,
GL2A_PERF_SEL_REQ_BURST_CLIENT8          = 0x00000053u32,
GL2A_PERF_SEL_REQ_BURST_CLIENT9          = 0x00000054u32,
GL2A_PERF_SEL_REQ_BURST_CLIENT10         = 0x00000055u32,
GL2A_PERF_SEL_REQ_BURST_CLIENT11         = 0x00000056u32,
GL2A_PERF_SEL_REQ_BURST_CLIENT12         = 0x00000057u32,
GL2A_PERF_SEL_REQ_BURST_CLIENT13         = 0x00000058u32,
GL2A_PERF_SEL_REQ_BURST_CLIENT14         = 0x00000059u32,
GL2A_PERF_SEL_REQ_BURST_CLIENT15         = 0x0000005au32,
GL2A_PERF_SEL_RTN_CREDIT_STALL_CLIENT0   = 0x0000005bu32,
GL2A_PERF_SEL_RTN_CREDIT_STALL_CLIENT1   = 0x0000005cu32,
GL2A_PERF_SEL_RTN_CREDIT_STALL_CLIENT2   = 0x0000005du32,
GL2A_PERF_SEL_RTN_CREDIT_STALL_CLIENT3   = 0x0000005eu32,
GL2A_PERF_SEL_RTN_CREDIT_STALL_CLIENT4   = 0x0000005fu32,
GL2A_PERF_SEL_RTN_CREDIT_STALL_CLIENT5   = 0x00000060u32,
GL2A_PERF_SEL_RTN_CREDIT_STALL_CLIENT6   = 0x00000061u32,
GL2A_PERF_SEL_RTN_CREDIT_STALL_CLIENT7   = 0x00000062u32,
GL2A_PERF_SEL_RTN_CREDIT_STALL_CLIENT8   = 0x00000063u32,
GL2A_PERF_SEL_RTN_CREDIT_STALL_CLIENT9   = 0x00000064u32,
GL2A_PERF_SEL_RTN_CREDIT_STALL_CLIENT10  = 0x00000065u32,
GL2A_PERF_SEL_RTN_CREDIT_STALL_CLIENT11  = 0x00000067u32,
GL2A_PERF_SEL_RTN_CREDIT_STALL_CLIENT12  = 0x00000068u32,
GL2A_PERF_SEL_RTN_CREDIT_STALL_CLIENT13  = 0x00000069u32,
GL2A_PERF_SEL_RTN_CREDIT_STALL_CLIENT14  = 0x0000006au32,
GL2A_PERF_SEL_RTN_CREDIT_STALL_CLIENT15  = 0x0000006bu32,
}

/*
 * GL2C_PERF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GL2C_PERF_SEL {
GL2C_PERF_SEL_NONE                       = 0x00000000u32,
GL2C_PERF_SEL_CYCLE                      = 0x00000001u32,
GL2C_PERF_SEL_BUSY                       = 0x00000002u32,
GL2C_PERF_SEL_REQ                        = 0x00000003u32,
GL2C_PERF_SEL_VOL_REQ                    = 0x00000004u32,
GL2C_PERF_SEL_HIGH_PRIORITY_REQ          = 0x00000005u32,
GL2C_PERF_SEL_READ                       = 0x00000006u32,
GL2C_PERF_SEL_WRITE                      = 0x00000007u32,
GL2C_PERF_SEL_ATOMIC                     = 0x00000008u32,
GL2C_PERF_SEL_NOP_ACK                    = 0x00000009u32,
GL2C_PERF_SEL_NOP_RTN0                   = 0x0000000au32,
GL2C_PERF_SEL_COMPRESSED_READ_REQ        = 0x0000000bu32,
GL2C_PERF_SEL_METADATA_READ_REQ          = 0x0000000cu32,
GL2C_PERF_SEL_CLIENT0_REQ                = 0x0000000du32,
GL2C_PERF_SEL_CLIENT1_REQ                = 0x0000000eu32,
GL2C_PERF_SEL_CLIENT2_REQ                = 0x0000000fu32,
GL2C_PERF_SEL_CLIENT3_REQ                = 0x00000010u32,
GL2C_PERF_SEL_CLIENT4_REQ                = 0x00000011u32,
GL2C_PERF_SEL_CLIENT5_REQ                = 0x00000012u32,
GL2C_PERF_SEL_CLIENT6_REQ                = 0x00000013u32,
GL2C_PERF_SEL_CLIENT7_REQ                = 0x00000014u32,
GL2C_PERF_SEL_CLIENT8_REQ                = 0x00000015u32,
GL2C_PERF_SEL_CLIENT9_REQ                = 0x00000016u32,
GL2C_PERF_SEL_CLIENT10_REQ               = 0x00000017u32,
GL2C_PERF_SEL_CLIENT11_REQ               = 0x00000018u32,
GL2C_PERF_SEL_CLIENT12_REQ               = 0x00000019u32,
GL2C_PERF_SEL_CLIENT13_REQ               = 0x0000001au32,
GL2C_PERF_SEL_CLIENT14_REQ               = 0x0000001bu32,
GL2C_PERF_SEL_CLIENT15_REQ               = 0x0000001cu32,
GL2C_PERF_SEL_C_RW_S_REQ                 = 0x0000001du32,
GL2C_PERF_SEL_C_RW_US_REQ                = 0x0000001eu32,
GL2C_PERF_SEL_C_RO_S_REQ                 = 0x0000001fu32,
GL2C_PERF_SEL_C_RO_US_REQ                = 0x00000020u32,
GL2C_PERF_SEL_UC_REQ                     = 0x00000021u32,
GL2C_PERF_SEL_LRU_REQ                    = 0x00000022u32,
GL2C_PERF_SEL_STREAM_REQ                 = 0x00000023u32,
GL2C_PERF_SEL_BYPASS_REQ                 = 0x00000024u32,
GL2C_PERF_SEL_NOA_REQ                    = 0x00000025u32,
GL2C_PERF_SEL_SHARED_REQ                 = 0x00000026u32,
GL2C_PERF_SEL_HIT                        = 0x00000027u32,
GL2C_PERF_SEL_MISS                       = 0x00000028u32,
GL2C_PERF_SEL_FULL_HIT                   = 0x00000029u32,
GL2C_PERF_SEL_PARTIAL_32B_HIT            = 0x0000002au32,
GL2C_PERF_SEL_PARTIAL_64B_HIT            = 0x0000002bu32,
GL2C_PERF_SEL_PARTIAL_96B_HIT            = 0x0000002cu32,
GL2C_PERF_SEL_DEWRITE_ALLOCATE_HIT       = 0x0000002du32,
GL2C_PERF_SEL_FULLY_WRITTEN_HIT          = 0x0000002eu32,
GL2C_PERF_SEL_UNCACHED_WRITE             = 0x0000002fu32,
GL2C_PERF_SEL_WRITEBACK                  = 0x00000030u32,
GL2C_PERF_SEL_NORMAL_WRITEBACK           = 0x00000031u32,
GL2C_PERF_SEL_EVICT                      = 0x00000032u32,
GL2C_PERF_SEL_NORMAL_EVICT               = 0x00000033u32,
GL2C_PERF_SEL_REQ_TO_MISS_QUEUE          = 0x00000034u32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT0   = 0x00000035u32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT1   = 0x00000036u32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT2   = 0x00000037u32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT3   = 0x00000038u32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT4   = 0x00000039u32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT5   = 0x0000003au32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT6   = 0x0000003bu32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT7   = 0x0000003cu32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT8   = 0x0000003du32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT9   = 0x0000003eu32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT10  = 0x0000003fu32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT11  = 0x00000040u32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT12  = 0x00000041u32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT13  = 0x00000042u32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT14  = 0x00000043u32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT15  = 0x00000044u32,
GL2C_PERF_SEL_READ_32_REQ                = 0x00000045u32,
GL2C_PERF_SEL_READ_64_REQ                = 0x00000046u32,
GL2C_PERF_SEL_READ_128_REQ               = 0x00000047u32,
GL2C_PERF_SEL_WRITE_32_REQ               = 0x00000048u32,
GL2C_PERF_SEL_WRITE_64_REQ               = 0x00000049u32,
GL2C_PERF_SEL_COMPRESSED_READ_0_REQ      = 0x0000004au32,
GL2C_PERF_SEL_COMPRESSED_READ_32_REQ     = 0x0000004bu32,
GL2C_PERF_SEL_COMPRESSED_READ_64_REQ     = 0x0000004cu32,
GL2C_PERF_SEL_COMPRESSED_READ_96_REQ     = 0x0000004du32,
GL2C_PERF_SEL_COMPRESSED_READ_128_REQ    = 0x0000004eu32,
GL2C_PERF_SEL_MC_WRREQ                   = 0x0000004fu32,
GL2C_PERF_SEL_EA_WRREQ_SNOOP             = 0x00000050u32,
GL2C_PERF_SEL_EA_WRREQ_64B               = 0x00000051u32,
GL2C_PERF_SEL_EA_WR_UNCACHED_32B         = 0x00000052u32,
GL2C_PERF_SEL_MC_WRREQ_STALL             = 0x00000053u32,
GL2C_PERF_SEL_EA_WRREQ_IO_CREDIT_STALL   = 0x00000054u32,
GL2C_PERF_SEL_EA_WRREQ_GMI_CREDIT_STALL  = 0x00000055u32,
GL2C_PERF_SEL_EA_WRREQ_DRAM_CREDIT_STALL = 0x00000056u32,
GL2C_PERF_SEL_TOO_MANY_EA_WRREQS_STALL   = 0x00000057u32,
GL2C_PERF_SEL_MC_WRREQ_LEVEL             = 0x00000058u32,
GL2C_PERF_SEL_EA_ATOMIC                  = 0x00000059u32,
GL2C_PERF_SEL_EA_ATOMIC_LEVEL            = 0x0000005au32,
GL2C_PERF_SEL_MC_RDREQ                   = 0x0000005bu32,
GL2C_PERF_SEL_EA_RDREQ_SNOOP             = 0x0000005cu32,
GL2C_PERF_SEL_EA_RDREQ_SPLIT             = 0x0000005du32,
GL2C_PERF_SEL_EA_RDREQ_32B               = 0x0000005eu32,
GL2C_PERF_SEL_EA_RDREQ_64B               = 0x0000005fu32,
GL2C_PERF_SEL_EA_RDREQ_96B               = 0x00000060u32,
GL2C_PERF_SEL_EA_RDREQ_128B              = 0x00000061u32,
GL2C_PERF_SEL_EA_RD_UNCACHED_32B         = 0x00000062u32,
GL2C_PERF_SEL_EA_RD_COMPRESSED_32B       = 0x00000063u32,
GL2C_PERF_SEL_EA_RDREQ_IO_CREDIT_STALL   = 0x00000064u32,
GL2C_PERF_SEL_EA_RDREQ_GMI_CREDIT_STALL  = 0x00000065u32,
GL2C_PERF_SEL_EA_RDREQ_DRAM_CREDIT_STALL = 0x00000066u32,
GL2C_PERF_SEL_MC_RDREQ_LEVEL             = 0x00000067u32,
GL2C_PERF_SEL_EA_RDREQ_DRAM              = 0x00000068u32,
GL2C_PERF_SEL_EA_WRREQ_DRAM              = 0x00000069u32,
GL2C_PERF_SEL_EA_RDREQ_DRAM_32B          = 0x0000006au32,
GL2C_PERF_SEL_EA_WRREQ_DRAM_32B          = 0x0000006bu32,
GL2C_PERF_SEL_ONION_READ                 = 0x0000006cu32,
GL2C_PERF_SEL_ONION_WRITE                = 0x0000006du32,
GL2C_PERF_SEL_IO_READ                    = 0x0000006eu32,
GL2C_PERF_SEL_IO_WRITE                   = 0x0000006fu32,
GL2C_PERF_SEL_GARLIC_READ                = 0x00000070u32,
GL2C_PERF_SEL_GARLIC_WRITE               = 0x00000071u32,
GL2C_PERF_SEL_EA_OUTSTANDING             = 0x00000072u32,
GL2C_PERF_SEL_LATENCY_FIFO_FULL          = 0x00000073u32,
GL2C_PERF_SEL_SRC_FIFO_FULL              = 0x00000074u32,
GL2C_PERF_SEL_TAG_STALL                  = 0x00000075u32,
GL2C_PERF_SEL_TAG_WRITEBACK_FIFO_FULL_STALL = 0x00000076u32,
GL2C_PERF_SEL_TAG_MISS_NOTHING_REPLACEABLE_STALL = 0x00000077u32,
GL2C_PERF_SEL_TAG_UNCACHED_WRITE_ATOMIC_FIFO_FULL_STALL = 0x00000078u32,
GL2C_PERF_SEL_TAG_NO_UNCACHED_WRITE_ATOMIC_ENTRIES_STALL = 0x00000079u32,
GL2C_PERF_SEL_TAG_READ_DST_STALL         = 0x0000007au32,
GL2C_PERF_SEL_READ_RETURN_TIMEOUT        = 0x0000007bu32,
GL2C_PERF_SEL_WRITEBACK_READ_TIMEOUT     = 0x0000007cu32,
GL2C_PERF_SEL_READ_RETURN_FULL_BUBBLE    = 0x0000007du32,
GL2C_PERF_SEL_BUBBLE                     = 0x0000007eu32,
GL2C_PERF_SEL_IB_REQ                     = 0x0000007fu32,
GL2C_PERF_SEL_IB_STALL                   = 0x00000080u32,
GL2C_PERF_SEL_IB_TAG_STALL               = 0x00000081u32,
GL2C_PERF_SEL_RETURN_ACK                 = 0x00000082u32,
GL2C_PERF_SEL_RETURN_DATA                = 0x00000083u32,
GL2C_PERF_SEL_EA_RDRET_NACK              = 0x00000084u32,
GL2C_PERF_SEL_EA_WRRET_NACK              = 0x00000085u32,
GL2C_PERF_SEL_GL2A_LEVEL                 = 0x00000086u32,
GL2C_PERF_SEL_ALL_TC_OP_WB_OR_INV_START  = 0x00000087u32,
GL2C_PERF_SEL_ALL_TC_OP_WB_OR_INV_VOL_START = 0x00000088u32,
GL2C_PERF_SEL_GCR_INV                    = 0x00000089u32,
GL2C_PERF_SEL_GCR_WB                     = 0x0000008au32,
GL2C_PERF_SEL_GCR_DISCARD                = 0x0000008bu32,
GL2C_PERF_SEL_GCR_RANGE                  = 0x0000008cu32,
GL2C_PERF_SEL_GCR_ALL                    = 0x0000008du32,
GL2C_PERF_SEL_GCR_VOL                    = 0x0000008eu32,
GL2C_PERF_SEL_GCR_UNSHARED               = 0x0000008fu32,
GL2C_PERF_SEL_GCR_GL2_INV_ALL            = 0x00000090u32,
GL2C_PERF_SEL_GCR_GL2_WB_ALL             = 0x00000091u32,
GL2C_PERF_SEL_GCR_GL2_INV_RANGE          = 0x00000092u32,
GL2C_PERF_SEL_GCR_GL2_WB_RANGE           = 0x00000093u32,
GL2C_PERF_SEL_GCR_GL2_WB_INV_RANGE       = 0x00000094u32,
GL2C_PERF_SEL_ALL_GCR_INV_EVICT          = 0x00000095u32,
GL2C_PERF_SEL_ALL_GCR_INV_VOL_EVICT      = 0x00000096u32,
GL2C_PERF_SEL_ALL_GCR_WB_OR_INV_CYCLE    = 0x00000097u32,
GL2C_PERF_SEL_ALL_GCR_WB_OR_INV_VOL_CYCLE = 0x00000098u32,
GL2C_PERF_SEL_ALL_GCR_WB_WRITEBACK       = 0x00000099u32,
GL2C_PERF_SEL_GCR_INVL2_VOL_CYCLE        = 0x0000009au32,
GL2C_PERF_SEL_GCR_INVL2_VOL_EVICT        = 0x0000009bu32,
GL2C_PERF_SEL_GCR_INVL2_VOL_START        = 0x0000009cu32,
GL2C_PERF_SEL_GCR_WBL2_VOL_CYCLE         = 0x0000009du32,
GL2C_PERF_SEL_GCR_WBL2_VOL_START         = 0x0000009eu32,
GL2C_PERF_SEL_GCR_WBINVL2_CYCLE          = 0x0000009fu32,
GL2C_PERF_SEL_GCR_WBINVL2_EVICT          = 0x000000a0u32,
GL2C_PERF_SEL_GCR_WBINVL2_START          = 0x000000a1u32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT16  = 0x000000a2u32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT17  = 0x000000a3u32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT18  = 0x000000a4u32,
GL2C_PERF_SEL_HIT_PASS_MISS_IN_CLIENT19  = 0x000000a5u32,
}

/*
 * SX_BLEND_OPT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SX_BLEND_OPT {
BLEND_OPT_PRESERVE_NONE_IGNORE_ALL       = 0x00000000u32,
BLEND_OPT_PRESERVE_ALL_IGNORE_NONE       = 0x00000001u32,
BLEND_OPT_PRESERVE_C1_IGNORE_C0          = 0x00000002u32,
BLEND_OPT_PRESERVE_C0_IGNORE_C1          = 0x00000003u32,
BLEND_OPT_PRESERVE_A1_IGNORE_A0          = 0x00000004u32,
BLEND_OPT_PRESERVE_A0_IGNORE_A1          = 0x00000005u32,
BLEND_OPT_PRESERVE_NONE_IGNORE_A0        = 0x00000006u32,
BLEND_OPT_PRESERVE_NONE_IGNORE_NONE      = 0x00000007u32,
}

/*
 * SX_DOWNCONVERT_FORMAT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SX_DOWNCONVERT_FORMAT {
SX_RT_EXPORT_NO_CONVERSION               = 0x00000000u32,
SX_RT_EXPORT_32_R                        = 0x00000001u32,
SX_RT_EXPORT_32_A                        = 0x00000002u32,
SX_RT_EXPORT_10_11_11                    = 0x00000003u32,
SX_RT_EXPORT_2_10_10_10                  = 0x00000004u32,
SX_RT_EXPORT_8_8_8_8                     = 0x00000005u32,
SX_RT_EXPORT_5_6_5                       = 0x00000006u32,
SX_RT_EXPORT_1_5_5_5                     = 0x00000007u32,
SX_RT_EXPORT_4_4_4_4                     = 0x00000008u32,
SX_RT_EXPORT_16_16_GR                    = 0x00000009u32,
SX_RT_EXPORT_16_16_AR                    = 0x0000000au32,
SX_RT_EXPORT_9_9_9_E5                    = 0x0000000bu32,
SX_RT_EXPORT_2_10_10_10_7E3              = 0x0000000cu32,
SX_RT_EXPORT_2_10_10_10_6E4              = 0x0000000du32,
}

/*
 * SX_OPT_COMB_FCN enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SX_OPT_COMB_FCN {
OPT_COMB_NONE                            = 0x00000000u32,
OPT_COMB_ADD                             = 0x00000001u32,
OPT_COMB_SUBTRACT                        = 0x00000002u32,
OPT_COMB_MIN                             = 0x00000003u32,
OPT_COMB_MAX                             = 0x00000004u32,
OPT_COMB_REVSUBTRACT                     = 0x00000005u32,
OPT_COMB_BLEND_DISABLED                  = 0x00000006u32,
OPT_COMB_SAFE_ADD                        = 0x00000007u32,
}

/*
 * SX_PERFCOUNTER_VALS enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SX_PERFCOUNTER_VALS {
SX_PERF_SEL_PA_IDLE_CYCLES               = 0x00000000u32,
SX_PERF_SEL_PA_REQ                       = 0x00000001u32,
SX_PERF_SEL_PA_POS                       = 0x00000002u32,
SX_PERF_SEL_CLOCK                        = 0x00000003u32,
SX_PERF_SEL_GATE_EN1                     = 0x00000004u32,
SX_PERF_SEL_GATE_EN2                     = 0x00000005u32,
SX_PERF_SEL_GATE_EN3                     = 0x00000006u32,
SX_PERF_SEL_GATE_EN4                     = 0x00000007u32,
SX_PERF_SEL_SH_POS_STARVE                = 0x00000008u32,
SX_PERF_SEL_SH_COLOR_STARVE              = 0x00000009u32,
SX_PERF_SEL_SH_POS_STALL                 = 0x0000000au32,
SX_PERF_SEL_SH_COLOR_STALL               = 0x0000000bu32,
SX_PERF_SEL_DB0_PIXELS                   = 0x0000000cu32,
SX_PERF_SEL_DB0_HALF_QUADS               = 0x0000000du32,
SX_PERF_SEL_DB0_PIXEL_STALL              = 0x0000000eu32,
SX_PERF_SEL_DB0_PIXEL_IDLE               = 0x0000000fu32,
SX_PERF_SEL_DB0_PRED_PIXELS              = 0x00000010u32,
SX_PERF_SEL_DB1_PIXELS                   = 0x00000011u32,
SX_PERF_SEL_DB1_HALF_QUADS               = 0x00000012u32,
SX_PERF_SEL_DB1_PIXEL_STALL              = 0x00000013u32,
SX_PERF_SEL_DB1_PIXEL_IDLE               = 0x00000014u32,
SX_PERF_SEL_DB1_PRED_PIXELS              = 0x00000015u32,
SX_PERF_SEL_DB2_PIXELS                   = 0x00000016u32,
SX_PERF_SEL_DB2_HALF_QUADS               = 0x00000017u32,
SX_PERF_SEL_DB2_PIXEL_STALL              = 0x00000018u32,
SX_PERF_SEL_DB2_PIXEL_IDLE               = 0x00000019u32,
SX_PERF_SEL_DB2_PRED_PIXELS              = 0x0000001au32,
SX_PERF_SEL_DB3_PIXELS                   = 0x0000001bu32,
SX_PERF_SEL_DB3_HALF_QUADS               = 0x0000001cu32,
SX_PERF_SEL_DB3_PIXEL_STALL              = 0x0000001du32,
SX_PERF_SEL_DB3_PIXEL_IDLE               = 0x0000001eu32,
SX_PERF_SEL_DB3_PRED_PIXELS              = 0x0000001fu32,
SX_PERF_SEL_COL_BUSY                     = 0x00000020u32,
SX_PERF_SEL_POS_BUSY                     = 0x00000021u32,
SX_PERF_SEL_DB0_MRT_BLEND_BYPASS         = 0x00000022u32,
SX_PERF_SEL_DB0_MRT_DONT_RD_DEST         = 0x00000023u32,
SX_PERF_SEL_DB0_MRT_DISCARD_SRC          = 0x00000024u32,
SX_PERF_SEL_DB0_MRT_SINGLE_QUADS         = 0x00000025u32,
SX_PERF_SEL_DB0_MRT_DOUBLE_QUADS         = 0x00000026u32,
SX_PERF_SEL_DB1_MRT_BLEND_BYPASS         = 0x00000027u32,
SX_PERF_SEL_DB1_MRT_DONT_RD_DEST         = 0x00000028u32,
SX_PERF_SEL_DB1_MRT_DISCARD_SRC          = 0x00000029u32,
SX_PERF_SEL_DB1_MRT_SINGLE_QUADS         = 0x0000002au32,
SX_PERF_SEL_DB1_MRT_DOUBLE_QUADS         = 0x0000002bu32,
SX_PERF_SEL_DB2_MRT_BLEND_BYPASS         = 0x0000002cu32,
SX_PERF_SEL_DB2_MRT_DONT_RD_DEST         = 0x0000002du32,
SX_PERF_SEL_DB2_MRT_DISCARD_SRC          = 0x0000002eu32,
SX_PERF_SEL_DB2_MRT_SINGLE_QUADS         = 0x0000002fu32,
SX_PERF_SEL_DB2_MRT_DOUBLE_QUADS         = 0x00000030u32,
SX_PERF_SEL_DB3_MRT_BLEND_BYPASS         = 0x00000031u32,
SX_PERF_SEL_DB3_MRT_DONT_RD_DEST         = 0x00000032u32,
SX_PERF_SEL_DB3_MRT_DISCARD_SRC          = 0x00000033u32,
SX_PERF_SEL_DB3_MRT_SINGLE_QUADS         = 0x00000034u32,
SX_PERF_SEL_DB3_MRT_DOUBLE_QUADS         = 0x00000035u32,
SX_PERF_SEL_PA_REQ_LATENCY               = 0x00000036u32,
SX_PERF_SEL_POS_SCBD_STALL               = 0x00000037u32,
SX_PERF_SEL_CLOCK_DROP_STALL             = 0x00000038u32,
SX_PERF_SEL_GATE_EN5                     = 0x00000039u32,
SX_PERF_SEL_GATE_EN6                     = 0x0000003au32,
SX_PERF_SEL_DB0_SIZE                     = 0x0000003bu32,
SX_PERF_SEL_DB1_SIZE                     = 0x0000003cu32,
SX_PERF_SEL_DB2_SIZE                     = 0x0000003du32,
SX_PERF_SEL_DB3_SIZE                     = 0x0000003eu32,
SX_PERF_SEL_IDX_STALL_CYCLES             = 0x0000003fu32,
SX_PERF_SEL_IDX_IDLE_CYCLES              = 0x00000040u32,
SX_PERF_SEL_IDX_REQ                      = 0x00000041u32,
SX_PERF_SEL_IDX_RET                      = 0x00000042u32,
SX_PERF_SEL_IDX_REQ_LATENCY              = 0x00000043u32,
SX_PERF_SEL_IDX_SCBD_STALL               = 0x00000044u32,
SX_PERF_SEL_GATE_EN7                     = 0x00000045u32,
SX_PERF_SEL_GATE_EN8                     = 0x00000046u32,
SX_PERF_SEL_SH_IDX_STARVE                = 0x00000047u32,
SX_PERF_SEL_IDX_BUSY                     = 0x00000048u32,
SX_PERF_SEL_PA_POS_BANK_CONF             = 0x00000049u32,
SX_PERF_SEL_DB0_END_OF_WAVE              = 0x0000004au32,
SX_PERF_SEL_DB0_4X2_DISCARD              = 0x0000004bu32,
SX_PERF_SEL_DB1_END_OF_WAVE              = 0x0000004cu32,
SX_PERF_SEL_DB1_4X2_DISCARD              = 0x0000004du32,
SX_PERF_SEL_DB2_END_OF_WAVE              = 0x0000004eu32,
SX_PERF_SEL_DB2_4X2_DISCARD              = 0x0000004fu32,
SX_PERF_SEL_DB3_END_OF_WAVE              = 0x00000050u32,
SX_PERF_SEL_DB3_4X2_DISCARD              = 0x00000051u32,
}

/*
 * CompareFrag enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum CompareFrag {
FRAG_NEVER                               = 0x00000000u32,
FRAG_LESS                                = 0x00000001u32,
FRAG_EQUAL                               = 0x00000002u32,
FRAG_LEQUAL                              = 0x00000003u32,
FRAG_GREATER                             = 0x00000004u32,
FRAG_NOTEQUAL                            = 0x00000005u32,
FRAG_GEQUAL                              = 0x00000006u32,
FRAG_ALWAYS                              = 0x00000007u32,
}

/*
 * ConservativeZExport enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ConservativeZExport {
EXPORT_ANY_Z                             = 0x00000000u32,
EXPORT_LESS_THAN_Z                       = 0x00000001u32,
EXPORT_GREATER_THAN_Z                    = 0x00000002u32,
EXPORT_RESERVED                          = 0x00000003u32,
}

/*
 * DbMemArbWatermarks enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DbMemArbWatermarks {
TRANSFERRED_64_BYTES                     = 0x00000000u32,
TRANSFERRED_128_BYTES                    = 0x00000001u32,
TRANSFERRED_256_BYTES                    = 0x00000002u32,
TRANSFERRED_512_BYTES                    = 0x00000003u32,
TRANSFERRED_1024_BYTES                   = 0x00000004u32,
TRANSFERRED_2048_BYTES                   = 0x00000005u32,
TRANSFERRED_4096_BYTES                   = 0x00000006u32,
TRANSFERRED_8192_BYTES                   = 0x00000007u32,
}

/*
 * DbPRTFaultBehavior enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DbPRTFaultBehavior {
FAULT_ZERO                               = 0x00000000u32,
FAULT_ONE                                = 0x00000001u32,
FAULT_FAIL                               = 0x00000002u32,
FAULT_PASS                               = 0x00000003u32,
}

/*
 * DbPSLControl enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DbPSLControl {
PSLC_AUTO                                = 0x00000000u32,
PSLC_ON_HANG_ONLY                        = 0x00000001u32,
PSLC_ASAP                                = 0x00000002u32,
PSLC_COUNTDOWN                           = 0x00000003u32,
}

/*
 * ForceControl enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ForceControl {
FORCE_OFF                                = 0x00000000u32,
FORCE_ENABLE                             = 0x00000001u32,
FORCE_DISABLE                            = 0x00000002u32,
FORCE_RESERVED                           = 0x00000003u32,
}

/*
 * GLCompressionMode enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GLCompressionMode {
DB_DEFAULT                               = 0x00000000u32,
DB_BYPASS                                = 0x00000001u32,
DB_COMP_WR_DISABLE                       = 0x00000002u32,
DB_BYPASS_WR_DISABLE                     = 0x00000003u32,
}

/*
 * OreoMode enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum OreoMode {
OMODE_BLEND                              = 0x00000000u32,
OMODE_O_THEN_B                           = 0x00000001u32,
OMODE_P_THEN_O_THEN_B                    = 0x00000002u32,
OMODE_RESERVED_3                         = 0x00000003u32,
}

/*
 * PerfCounter_Vals enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PerfCounter_Vals {
DB_PERF_SEL_SC_DB_tile_sends             = 0x00000000u32,
DB_PERF_SEL_SC_DB_tile_busy              = 0x00000001u32,
DB_PERF_SEL_SC_DB_tile_stalls            = 0x00000002u32,
DB_PERF_SEL_SC_DB_tile_events            = 0x00000003u32,
DB_PERF_SEL_SC_DB_tile_tiles             = 0x00000004u32,
DB_PERF_SEL_SC_DB_tile_covered           = 0x00000005u32,
DB_PERF_SEL_hiz_tc_read_starved          = 0x00000006u32,
DB_PERF_SEL_hiz_tc_write_stall           = 0x00000007u32,
DB_PERF_SEL_hiz_tile_culled              = 0x00000008u32,
DB_PERF_SEL_his_tile_culled              = 0x00000009u32,
DB_PERF_SEL_DB_SC_tile_sends             = 0x0000000au32,
DB_PERF_SEL_DB_SC_tile_busy              = 0x0000000bu32,
DB_PERF_SEL_DB_SC_tile_stalls            = 0x0000000cu32,
DB_PERF_SEL_DB_SC_tile_df_stalls         = 0x0000000du32,
DB_PERF_SEL_DB_SC_tile_tiles             = 0x0000000eu32,
DB_PERF_SEL_DB_SC_tile_culled            = 0x0000000fu32,
DB_PERF_SEL_DB_SC_tile_hier_kill         = 0x00000010u32,
DB_PERF_SEL_DB_SC_tile_fast_ops          = 0x00000011u32,
DB_PERF_SEL_DB_SC_tile_no_ops            = 0x00000012u32,
DB_PERF_SEL_DB_SC_tile_tile_rate         = 0x00000013u32,
DB_PERF_SEL_DB_SC_tile_ssaa_kill         = 0x00000014u32,
DB_PERF_SEL_DB_SC_tile_fast_z_ops        = 0x00000015u32,
DB_PERF_SEL_DB_SC_tile_fast_stencil_ops  = 0x00000016u32,
DB_PERF_SEL_SC_DB_quad_sends             = 0x00000017u32,
DB_PERF_SEL_SC_DB_quad_busy              = 0x00000018u32,
DB_PERF_SEL_SC_DB_quad_squads            = 0x00000019u32,
DB_PERF_SEL_SC_DB_quad_tiles             = 0x0000001au32,
DB_PERF_SEL_SC_DB_quad_pixels            = 0x0000001bu32,
DB_PERF_SEL_SC_DB_quad_killed_tiles      = 0x0000001cu32,
DB_PERF_SEL_DB_SC_quad_sends             = 0x0000001du32,
DB_PERF_SEL_DB_SC_quad_busy              = 0x0000001eu32,
DB_PERF_SEL_DB_SC_quad_stalls            = 0x0000001fu32,
DB_PERF_SEL_DB_SC_quad_tiles             = 0x00000020u32,
DB_PERF_SEL_DB_SC_quad_lit_quad          = 0x00000021u32,
DB_PERF_SEL_DB_CB_export_events          = 0x00000022u32,
DB_PERF_SEL_SX_DB_quad_sends             = 0x00000025u32,
DB_PERF_SEL_SX_DB_quad_busy              = 0x00000026u32,
DB_PERF_SEL_SX_DB_quad_stalls            = 0x00000027u32,
DB_PERF_SEL_SX_DB_quad_quads             = 0x00000028u32,
DB_PERF_SEL_SX_DB_quad_pixels            = 0x00000029u32,
DB_PERF_SEL_SX_DB_quad_exports           = 0x0000002au32,
DB_PERF_SEL_SH_quads_outstanding_sum     = 0x0000002bu32,
DB_PERF_SEL_DB_CB_export_sends           = 0x0000002cu32,
DB_PERF_SEL_DB_CB_export_busy            = 0x0000002du32,
DB_PERF_SEL_DB_CB_export_stalls          = 0x0000002eu32,
DB_PERF_SEL_DB_CB_export_quads           = 0x0000002fu32,
DB_PERF_SEL_tile_rd_sends                = 0x00000030u32,
DB_PERF_SEL_mi_tile_rd_outstanding_sum   = 0x00000031u32,
DB_PERF_SEL_quad_rd_sends                = 0x00000032u32,
DB_PERF_SEL_quad_rd_busy                 = 0x00000033u32,
DB_PERF_SEL_quad_rd_mi_stall             = 0x00000034u32,
DB_PERF_SEL_quad_rd_rw_collision         = 0x00000035u32,
DB_PERF_SEL_quad_rd_tag_stall            = 0x00000036u32,
DB_PERF_SEL_quad_rd_32byte_reqs          = 0x00000037u32,
DB_PERF_SEL_quad_rd_panic                = 0x00000038u32,
DB_PERF_SEL_mi_quad_rd_outstanding_sum   = 0x00000039u32,
DB_PERF_SEL_quad_rdret_sends             = 0x0000003au32,
DB_PERF_SEL_quad_rdret_busy              = 0x0000003bu32,
DB_PERF_SEL_tile_wr_sends                = 0x0000003cu32,
DB_PERF_SEL_tile_wr_acks                 = 0x0000003du32,
DB_PERF_SEL_mi_tile_wr_outstanding_sum   = 0x0000003eu32,
DB_PERF_SEL_quad_wr_sends                = 0x0000003fu32,
DB_PERF_SEL_quad_wr_busy                 = 0x00000040u32,
DB_PERF_SEL_quad_wr_mi_stall             = 0x00000041u32,
DB_PERF_SEL_quad_wr_coherency_stall      = 0x00000042u32,
DB_PERF_SEL_quad_wr_acks                 = 0x00000043u32,
DB_PERF_SEL_mi_quad_wr_outstanding_sum   = 0x00000044u32,
DB_PERF_SEL_Tile_Cache_misses            = 0x00000045u32,
DB_PERF_SEL_Tile_Cache_hits              = 0x00000046u32,
DB_PERF_SEL_Tile_Cache_flushes           = 0x00000047u32,
DB_PERF_SEL_Tile_Cache_surface_stall     = 0x00000048u32,
DB_PERF_SEL_Tile_Cache_starves           = 0x00000049u32,
DB_PERF_SEL_Tile_Cache_mem_return_starve = 0x0000004au32,
DB_PERF_SEL_tcp_dispatcher_reads         = 0x0000004bu32,
DB_PERF_SEL_tcp_prefetcher_reads         = 0x0000004cu32,
DB_PERF_SEL_tcp_preloader_reads          = 0x0000004du32,
DB_PERF_SEL_tcp_dispatcher_flushes       = 0x0000004eu32,
DB_PERF_SEL_tcp_prefetcher_flushes       = 0x0000004fu32,
DB_PERF_SEL_tcp_preloader_flushes        = 0x00000050u32,
DB_PERF_SEL_Depth_Tile_Cache_sends       = 0x00000051u32,
DB_PERF_SEL_Depth_Tile_Cache_busy        = 0x00000052u32,
DB_PERF_SEL_Depth_Tile_Cache_starves     = 0x00000053u32,
DB_PERF_SEL_Depth_Tile_Cache_dtile_locked = 0x00000054u32,
DB_PERF_SEL_Depth_Tile_Cache_alloc_stall = 0x00000055u32,
DB_PERF_SEL_Depth_Tile_Cache_misses      = 0x00000056u32,
DB_PERF_SEL_Depth_Tile_Cache_hits        = 0x00000057u32,
DB_PERF_SEL_Depth_Tile_Cache_flushes     = 0x00000058u32,
DB_PERF_SEL_Depth_Tile_Cache_noop_tile   = 0x00000059u32,
DB_PERF_SEL_Depth_Tile_Cache_detailed_noop = 0x0000005au32,
DB_PERF_SEL_Depth_Tile_Cache_event       = 0x0000005bu32,
DB_PERF_SEL_Depth_Tile_Cache_tile_frees  = 0x0000005cu32,
DB_PERF_SEL_Depth_Tile_Cache_data_frees  = 0x0000005du32,
DB_PERF_SEL_Depth_Tile_Cache_mem_return_starve = 0x0000005eu32,
DB_PERF_SEL_Stencil_Cache_misses         = 0x0000005fu32,
DB_PERF_SEL_Stencil_Cache_hits           = 0x00000060u32,
DB_PERF_SEL_Stencil_Cache_flushes        = 0x00000061u32,
DB_PERF_SEL_Stencil_Cache_starves        = 0x00000062u32,
DB_PERF_SEL_Stencil_Cache_frees          = 0x00000063u32,
DB_PERF_SEL_Z_Cache_separate_Z_misses    = 0x00000064u32,
DB_PERF_SEL_Z_Cache_separate_Z_hits      = 0x00000065u32,
DB_PERF_SEL_Z_Cache_separate_Z_flushes   = 0x00000066u32,
DB_PERF_SEL_Z_Cache_separate_Z_starves   = 0x00000067u32,
DB_PERF_SEL_Z_Cache_pmask_misses         = 0x00000068u32,
DB_PERF_SEL_Z_Cache_pmask_hits           = 0x00000069u32,
DB_PERF_SEL_Z_Cache_pmask_flushes        = 0x0000006au32,
DB_PERF_SEL_Z_Cache_pmask_starves        = 0x0000006bu32,
DB_PERF_SEL_Z_Cache_frees                = 0x0000006cu32,
DB_PERF_SEL_Plane_Cache_misses           = 0x0000006du32,
DB_PERF_SEL_Plane_Cache_hits             = 0x0000006eu32,
DB_PERF_SEL_Plane_Cache_flushes          = 0x0000006fu32,
DB_PERF_SEL_Plane_Cache_starves          = 0x00000070u32,
DB_PERF_SEL_Plane_Cache_frees            = 0x00000071u32,
DB_PERF_SEL_flush_expanded_stencil       = 0x00000072u32,
DB_PERF_SEL_flush_compressed_stencil     = 0x00000073u32,
DB_PERF_SEL_flush_single_stencil         = 0x00000074u32,
DB_PERF_SEL_planes_flushed               = 0x00000075u32,
DB_PERF_SEL_flush_1plane                 = 0x00000076u32,
DB_PERF_SEL_flush_2plane                 = 0x00000077u32,
DB_PERF_SEL_flush_3plane                 = 0x00000078u32,
DB_PERF_SEL_flush_4plane                 = 0x00000079u32,
DB_PERF_SEL_flush_5plane                 = 0x0000007au32,
DB_PERF_SEL_flush_6plane                 = 0x0000007bu32,
DB_PERF_SEL_flush_7plane                 = 0x0000007cu32,
DB_PERF_SEL_flush_8plane                 = 0x0000007du32,
DB_PERF_SEL_flush_9plane                 = 0x0000007eu32,
DB_PERF_SEL_flush_10plane                = 0x0000007fu32,
DB_PERF_SEL_flush_11plane                = 0x00000080u32,
DB_PERF_SEL_flush_12plane                = 0x00000081u32,
DB_PERF_SEL_flush_13plane                = 0x00000082u32,
DB_PERF_SEL_flush_14plane                = 0x00000083u32,
DB_PERF_SEL_flush_15plane                = 0x00000084u32,
DB_PERF_SEL_flush_16plane                = 0x00000085u32,
DB_PERF_SEL_flush_expanded_z             = 0x00000086u32,
DB_PERF_SEL_earlyZ_waiting_for_postZ_done = 0x00000087u32,
DB_PERF_SEL_reZ_waiting_for_postZ_done   = 0x00000088u32,
DB_PERF_SEL_dk_tile_sends                = 0x00000089u32,
DB_PERF_SEL_dk_tile_busy                 = 0x0000008au32,
DB_PERF_SEL_dk_tile_quad_starves         = 0x0000008bu32,
DB_PERF_SEL_dk_tile_stalls               = 0x0000008cu32,
DB_PERF_SEL_dk_squad_sends               = 0x0000008du32,
DB_PERF_SEL_dk_squad_busy                = 0x0000008eu32,
DB_PERF_SEL_dk_squad_stalls              = 0x0000008fu32,
DB_PERF_SEL_Op_Pipe_Busy                 = 0x00000090u32,
DB_PERF_SEL_Op_Pipe_MC_Read_stall        = 0x00000091u32,
DB_PERF_SEL_qc_busy                      = 0x00000092u32,
DB_PERF_SEL_qc_xfc                       = 0x00000093u32,
DB_PERF_SEL_qc_conflicts                 = 0x00000094u32,
DB_PERF_SEL_qc_full_stall                = 0x00000095u32,
DB_PERF_SEL_qc_in_preZ_tile_stalls_postZ = 0x00000096u32,
DB_PERF_SEL_qc_in_postZ_tile_stalls_preZ = 0x00000097u32,
DB_PERF_SEL_tsc_insert_summarize_stall   = 0x00000098u32,
DB_PERF_SEL_tl_busy                      = 0x00000099u32,
DB_PERF_SEL_tl_dtc_read_starved          = 0x0000009au32,
DB_PERF_SEL_tl_z_fetch_stall             = 0x0000009bu32,
DB_PERF_SEL_tl_stencil_stall             = 0x0000009cu32,
DB_PERF_SEL_tl_z_decompress_stall        = 0x0000009du32,
DB_PERF_SEL_tl_stencil_locked_stall      = 0x0000009eu32,
DB_PERF_SEL_tl_events                    = 0x0000009fu32,
DB_PERF_SEL_tl_summarize_squads          = 0x000000a0u32,
DB_PERF_SEL_tl_flush_expand_squads       = 0x000000a1u32,
DB_PERF_SEL_tl_expand_squads             = 0x000000a2u32,
DB_PERF_SEL_tl_preZ_squads               = 0x000000a3u32,
DB_PERF_SEL_tl_postZ_squads              = 0x000000a4u32,
DB_PERF_SEL_tl_preZ_noop_squads          = 0x000000a5u32,
DB_PERF_SEL_tl_postZ_noop_squads         = 0x000000a6u32,
DB_PERF_SEL_tl_tile_ops                  = 0x000000a7u32,
DB_PERF_SEL_tl_in_xfc                    = 0x000000a8u32,
DB_PERF_SEL_tl_in_single_stencil_expand_stall = 0x000000a9u32,
DB_PERF_SEL_tl_in_fast_z_stall           = 0x000000aau32,
DB_PERF_SEL_tl_out_xfc                   = 0x000000abu32,
DB_PERF_SEL_tl_out_squads                = 0x000000acu32,
DB_PERF_SEL_zf_plane_multicycle          = 0x000000adu32,
DB_PERF_SEL_PostZ_Samples_passing_Z      = 0x000000aeu32,
DB_PERF_SEL_PostZ_Samples_failing_Z      = 0x000000afu32,
DB_PERF_SEL_PostZ_Samples_failing_S      = 0x000000b0u32,
DB_PERF_SEL_PreZ_Samples_passing_Z       = 0x000000b1u32,
DB_PERF_SEL_PreZ_Samples_failing_Z       = 0x000000b2u32,
DB_PERF_SEL_PreZ_Samples_failing_S       = 0x000000b3u32,
DB_PERF_SEL_ts_tc_update_stall           = 0x000000b4u32,
DB_PERF_SEL_sc_kick_start                = 0x000000b5u32,
DB_PERF_SEL_sc_kick_end                  = 0x000000b6u32,
DB_PERF_SEL_clock_reg_active             = 0x000000b7u32,
DB_PERF_SEL_clock_main_active            = 0x000000b8u32,
DB_PERF_SEL_clock_mem_export_active      = 0x000000b9u32,
DB_PERF_SEL_esr_ps_out_busy              = 0x000000bau32,
DB_PERF_SEL_esr_ps_lqf_busy              = 0x000000bbu32,
DB_PERF_SEL_esr_ps_lqf_stall             = 0x000000bcu32,
DB_PERF_SEL_etr_out_send                 = 0x000000bdu32,
DB_PERF_SEL_etr_out_busy                 = 0x000000beu32,
DB_PERF_SEL_etr_out_ltile_probe_fifo_full_stall = 0x000000bfu32,
DB_PERF_SEL_etr_out_esr_stall            = 0x000000c1u32,
DB_PERF_SEL_esr_ps_vic_busy              = 0x000000c2u32,
DB_PERF_SEL_esr_ps_vic_stall             = 0x000000c3u32,
DB_PERF_SEL_esr_eot_fwd_busy             = 0x000000c4u32,
DB_PERF_SEL_esr_eot_fwd_holding_squad    = 0x000000c5u32,
DB_PERF_SEL_esr_eot_fwd_forward          = 0x000000c6u32,
DB_PERF_SEL_esr_sqq_zi_busy              = 0x000000c7u32,
DB_PERF_SEL_esr_sqq_zi_stall             = 0x000000c8u32,
DB_PERF_SEL_postzl_sq_pt_busy            = 0x000000c9u32,
DB_PERF_SEL_postzl_sq_pt_stall           = 0x000000cau32,
DB_PERF_SEL_postzl_se_busy               = 0x000000cbu32,
DB_PERF_SEL_postzl_se_stall              = 0x000000ccu32,
DB_PERF_SEL_postzl_partial_launch        = 0x000000cdu32,
DB_PERF_SEL_postzl_full_launch           = 0x000000ceu32,
DB_PERF_SEL_postzl_partial_waiting       = 0x000000cfu32,
DB_PERF_SEL_postzl_tile_mem_stall        = 0x000000d0u32,
DB_PERF_SEL_postzl_tile_init_stall       = 0x000000d1u32,
DB_PERF_SEL_prezl_tile_mem_stall         = 0x000000d2u32,
DB_PERF_SEL_prezl_tile_init_stall        = 0x000000d3u32,
DB_PERF_SEL_dtt_sm_clash_stall           = 0x000000d4u32,
DB_PERF_SEL_dtt_sm_slot_stall            = 0x000000d5u32,
DB_PERF_SEL_dtt_sm_miss_stall            = 0x000000d6u32,
DB_PERF_SEL_mi_rdreq_busy                = 0x000000d7u32,
DB_PERF_SEL_mi_rdreq_stall               = 0x000000d8u32,
DB_PERF_SEL_mi_wrreq_busy                = 0x000000d9u32,
DB_PERF_SEL_mi_wrreq_stall               = 0x000000dau32,
DB_PERF_SEL_recomp_tile_to_1zplane_no_fastop = 0x000000dbu32,
DB_PERF_SEL_dkg_tile_rate_tile           = 0x000000dcu32,
DB_PERF_SEL_prezl_src_in_sends           = 0x000000ddu32,
DB_PERF_SEL_prezl_src_in_stall           = 0x000000deu32,
DB_PERF_SEL_prezl_src_in_squads          = 0x000000dfu32,
DB_PERF_SEL_prezl_src_in_squads_unrolled = 0x000000e0u32,
DB_PERF_SEL_prezl_src_in_tile_rate       = 0x000000e1u32,
DB_PERF_SEL_prezl_src_in_tile_rate_unrolled = 0x000000e2u32,
DB_PERF_SEL_prezl_src_out_stall          = 0x000000e3u32,
DB_PERF_SEL_postzl_src_in_sends          = 0x000000e4u32,
DB_PERF_SEL_postzl_src_in_stall          = 0x000000e5u32,
DB_PERF_SEL_postzl_src_in_squads         = 0x000000e6u32,
DB_PERF_SEL_postzl_src_in_squads_unrolled = 0x000000e7u32,
DB_PERF_SEL_postzl_src_in_tile_rate      = 0x000000e8u32,
DB_PERF_SEL_postzl_src_in_tile_rate_unrolled = 0x000000e9u32,
DB_PERF_SEL_postzl_src_out_stall         = 0x000000eau32,
DB_PERF_SEL_esr_ps_src_in_sends          = 0x000000ebu32,
DB_PERF_SEL_esr_ps_src_in_stall          = 0x000000ecu32,
DB_PERF_SEL_esr_ps_src_in_squads         = 0x000000edu32,
DB_PERF_SEL_esr_ps_src_in_squads_unrolled = 0x000000eeu32,
DB_PERF_SEL_esr_ps_src_in_tile_rate      = 0x000000efu32,
DB_PERF_SEL_esr_ps_src_in_tile_rate_unrolled = 0x000000f0u32,
DB_PERF_SEL_esr_ps_src_in_tile_rate_unrolled_to_pixel_rate = 0x000000f1u32,
DB_PERF_SEL_esr_ps_src_out_stall         = 0x000000f2u32,
DB_PERF_SEL_depth_bounds_tile_culled     = 0x000000f3u32,
DB_PERF_SEL_PreZ_Samples_failing_DB      = 0x000000f4u32,
DB_PERF_SEL_PostZ_Samples_failing_DB     = 0x000000f5u32,
DB_PERF_SEL_flush_compressed             = 0x000000f6u32,
DB_PERF_SEL_flush_plane_le4              = 0x000000f7u32,
DB_PERF_SEL_tiles_z_fully_summarized     = 0x000000f8u32,
DB_PERF_SEL_tiles_stencil_fully_summarized = 0x000000f9u32,
DB_PERF_SEL_tiles_z_clear_on_expclear    = 0x000000fau32,
DB_PERF_SEL_tiles_s_clear_on_expclear    = 0x000000fbu32,
DB_PERF_SEL_tiles_decomp_on_expclear     = 0x000000fcu32,
DB_PERF_SEL_tiles_compressed_to_decompressed = 0x000000fdu32,
DB_PERF_SEL_Op_Pipe_Prez_Busy            = 0x000000feu32,
DB_PERF_SEL_Op_Pipe_Postz_Busy           = 0x000000ffu32,
DB_PERF_SEL_di_dt_stall                  = 0x00000100u32,
DB_PERF_SEL_DB_SC_s_tile_rate            = 0x00000102u32,
DB_PERF_SEL_DB_SC_c_tile_rate            = 0x00000103u32,
DB_PERF_SEL_DB_SC_z_tile_rate            = 0x00000104u32,
DB_PERF_SEL_DB_CB_export_export_quads    = 0x00000105u32,
DB_PERF_SEL_DB_CB_export_double_format   = 0x00000106u32,
DB_PERF_SEL_DB_CB_export_fast_format     = 0x00000107u32,
DB_PERF_SEL_DB_CB_export_slow_format     = 0x00000108u32,
DB_PERF_SEL_CB_DB_rdreq_sends            = 0x00000109u32,
DB_PERF_SEL_CB_DB_rdreq_prt_sends        = 0x0000010au32,
DB_PERF_SEL_CB_DB_wrreq_sends            = 0x0000010bu32,
DB_PERF_SEL_CB_DB_wrreq_prt_sends        = 0x0000010cu32,
DB_PERF_SEL_DB_CB_rdret_ack              = 0x0000010du32,
DB_PERF_SEL_DB_CB_rdret_nack             = 0x0000010eu32,
DB_PERF_SEL_DB_CB_wrret_ack              = 0x0000010fu32,
DB_PERF_SEL_DB_CB_wrret_nack             = 0x00000110u32,
DB_PERF_SEL_MI_tile_req_wrack_counter_stall = 0x00000111u32,
DB_PERF_SEL_MI_quad_req_wrack_counter_stall = 0x00000112u32,
DB_PERF_SEL_MI_zpc_req_wrack_counter_stall = 0x00000113u32,
DB_PERF_SEL_MI_psd_req_wrack_counter_stall = 0x00000114u32,
DB_PERF_SEL_unmapped_z_tile_culled       = 0x00000115u32,
DB_PERF_SEL_DB_CB_export_is_event_FLUSH_AND_INV_DB_DATA_TS = 0x00000116u32,
DB_PERF_SEL_DB_CB_export_is_event_FLUSH_AND_INV_CB_PIXEL_DATA = 0x00000117u32,
DB_PERF_SEL_DB_CB_export_is_event_BOTTOM_OF_PIPE_TS = 0x00000118u32,
DB_PERF_SEL_DB_CB_export_waiting_for_perfcounter_stop_event = 0x00000119u32,
DB_PERF_SEL_DB_CB_export_fmt_32bpp_8pix  = 0x0000011au32,
DB_PERF_SEL_DB_CB_export_fmt_16_16_unsigned_8pix = 0x0000011bu32,
DB_PERF_SEL_DB_CB_export_fmt_16_16_signed_8pix = 0x0000011cu32,
DB_PERF_SEL_DB_CB_export_fmt_16_16_float_8pix = 0x0000011du32,
DB_PERF_SEL_DB_CB_export_num_pixels_need_blending = 0x0000011eu32,
DB_PERF_SEL_DB_CB_context_dones          = 0x0000011fu32,
DB_PERF_SEL_DB_CB_eop_dones              = 0x00000120u32,
DB_PERF_SEL_SX_DB_quad_all_pixels_killed = 0x00000121u32,
DB_PERF_SEL_SX_DB_quad_all_pixels_enabled = 0x00000122u32,
DB_PERF_SEL_SX_DB_quad_need_blending_and_dst_read = 0x00000123u32,
DB_PERF_SEL_SC_DB_tile_backface          = 0x00000124u32,
DB_PERF_SEL_SC_DB_quad_quads             = 0x00000125u32,
DB_PERF_SEL_DB_SC_quad_quads_with_1_pixel = 0x00000126u32,
DB_PERF_SEL_DB_SC_quad_quads_with_2_pixels = 0x00000127u32,
DB_PERF_SEL_DB_SC_quad_quads_with_3_pixels = 0x00000128u32,
DB_PERF_SEL_DB_SC_quad_quads_with_4_pixels = 0x00000129u32,
DB_PERF_SEL_DB_SC_quad_double_quad       = 0x0000012au32,
DB_PERF_SEL_SX_DB_quad_export_quads      = 0x0000012bu32,
DB_PERF_SEL_SX_DB_quad_double_format     = 0x0000012cu32,
DB_PERF_SEL_SX_DB_quad_fast_format       = 0x0000012du32,
DB_PERF_SEL_SX_DB_quad_slow_format       = 0x0000012eu32,
DB_PERF_SEL_quad_rd_sends_unc            = 0x0000012fu32,
DB_PERF_SEL_quad_rd_mi_stall_unc         = 0x00000130u32,
DB_PERF_SEL_SC_DB_tile_tiles_pipe0       = 0x00000131u32,
DB_PERF_SEL_SC_DB_tile_tiles_pipe1       = 0x00000132u32,
DB_PERF_SEL_SC_DB_quad_quads_pipe0       = 0x00000133u32,
DB_PERF_SEL_SC_DB_quad_quads_pipe1       = 0x00000134u32,
DB_PERF_SEL_PERF_fg_lob_fwdr_timeout_hits = 0x00000135u32,
DB_PERF_SEL_noz_waiting_for_postz_done   = 0x00000136u32,
DB_PERF_SEL_DB_CB_export_quads_vrs_rate_1x1 = 0x00000137u32,
DB_PERF_SEL_DB_CB_export_quads_vrs_rate_2x1 = 0x00000138u32,
DB_PERF_SEL_DB_CB_export_quads_vrs_rate_1x2 = 0x00000139u32,
DB_PERF_SEL_DB_CB_export_quads_vrs_rate_2x2 = 0x0000013au32,
DB_PERF_SEL_RMI_rd_tile_32byte_req       = 0x0000013bu32,
DB_PERF_SEL_RMI_rd_z_32byte_req          = 0x0000013cu32,
DB_PERF_SEL_RMI_rd_s_32byte_req          = 0x0000013du32,
DB_PERF_SEL_RMI_wr_tile_32byte_req       = 0x0000013eu32,
DB_PERF_SEL_RMI_wr_z_32byte_req          = 0x0000013fu32,
DB_PERF_SEL_RMI_wr_s_32byte_req          = 0x00000140u32,
DB_PERF_SEL_RMI_wr_psdzpc_32byte_req     = 0x00000141u32,
DB_PERF_SEL_RMI_rd_tile_32byte_ret       = 0x00000142u32,
DB_PERF_SEL_RMI_rd_z_32byte_ret          = 0x00000143u32,
DB_PERF_SEL_RMI_rd_s_32byte_ret          = 0x00000144u32,
DB_PERF_SEL_RMI_wr_tile_32byte_ack       = 0x00000145u32,
DB_PERF_SEL_RMI_wr_z_32byte_ack          = 0x00000146u32,
DB_PERF_SEL_RMI_wr_s_32byte_ack          = 0x00000147u32,
DB_PERF_SEL_RMI_wr_psdzpc_32byte_ack     = 0x00000148u32,
DB_PERF_SEL_esr_vic_sqq_busy             = 0x00000149u32,
DB_PERF_SEL_esr_vic_sqq_stall            = 0x0000014au32,
DB_PERF_SEL_esr_psi_vic_tile_rate        = 0x0000014bu32,
DB_PERF_SEL_esr_vic_footprint_match_2x2  = 0x0000014cu32,
DB_PERF_SEL_esr_vic_footprint_match_2x1  = 0x0000014du32,
DB_PERF_SEL_esr_vic_footprint_match_1x2  = 0x0000014eu32,
DB_PERF_SEL_DB_SC_quad_num_null_2x2_coarse_pixels = 0x0000014fu32,
DB_PERF_SEL_DB_SC_quad_num_null_2x1_coarse_pixels = 0x00000150u32,
DB_PERF_SEL_DB_SC_quad_num_null_1x2_coarse_pixels = 0x00000151u32,
DB_PERF_SEL_hi_z_s_checker_force_coarse_vrs_1x1 = 0x00000152u32,
DB_PERF_SEL_hi_z_s_checker_force_ssaa_vrs_1x1 = 0x00000153u32,
DB_PERF_SEL_esr_ps_woc_1squadIn_2squadOut = 0x00000154u32,
DB_PERF_SEL_esr_ps_woc_2squadIn_1squadOut = 0x00000155u32,
DB_PERF_SEL_prez_ps_invoked_pixel_cnt    = 0x00000156u32,
DB_PERF_SEL_postz_ps_invoked_pixel_cnt   = 0x00000157u32,
DB_PERF_SEL_ts_events_pws_enable         = 0x00000158u32,
DB_PERF_SEL_ps_events_pws_enable         = 0x00000159u32,
DB_PERF_SEL_cs_events_pws_enable         = 0x0000015au32,
DB_PERF_SEL_DB_SC_quad_noz_tiles         = 0x0000015bu32,
DB_PERF_SEL_DB_SC_quad_lit_noz_quad      = 0x0000015cu32,
DB_PERF_SEL_DB_SC_quad_conflicts         = 0x0000015du32,
DB_PERF_SEL_SC_DB_quad_vrs_1x1           = 0x0000015eu32,
DB_PERF_SEL_SC_DB_quad_vrs_1x2           = 0x0000015fu32,
DB_PERF_SEL_SC_DB_quad_vrs_2x1           = 0x00000160u32,
DB_PERF_SEL_SC_DB_quad_vrs_2x2           = 0x00000161u32,
DB_PERF_SEL_SC_DB_quad_vrs_2x_ssaa       = 0x00000162u32,
DB_PERF_SEL_SC_DB_quad_vrs_4x_ssaa       = 0x00000163u32,
DB_PERF_SEL_SC_DB_quad_vrs_8x_ssaa       = 0x00000164u32,
DB_PERF_SEL_SC_DB_wave_sends             = 0x00000165u32,
DB_PERF_SEL_SC_DB_wave_busy              = 0x00000166u32,
DB_PERF_SEL_SC_DB_wave_quads             = 0x00000167u32,
DB_PERF_SEL_SC_DB_wave_id_wrapped        = 0x00000168u32,
DB_PERF_SEL_DB_SC_wave_sends             = 0x00000169u32,
DB_PERF_SEL_DB_SC_wave_busy              = 0x0000016au32,
DB_PERF_SEL_DB_SC_wave_stalls            = 0x0000016bu32,
DB_PERF_SEL_DB_SC_wave_conflict          = 0x0000016cu32,
DB_PERF_SEL_DB_SC_wave_hard_conflict     = 0x0000016du32,
DB_PERF_SEL_DB_SC_wave_id_wrapped        = 0x0000016eu32,
DB_PERF_SEL_SX_DB_quad_waves             = 0x0000016fu32,
DB_PERF_SEL_pws_stall                    = 0x00000170u32,
DB_PERF_SEL_pws_liveness_stall_dtt_tag   = 0x00000171u32,
DB_PERF_SEL_pws_liveness_stall_tcp_cache_mgr = 0x00000172u32,
DB_PERF_SEL_OREO_TT_load                 = 0x00000173u32,
DB_PERF_SEL_OREO_TT_read                 = 0x00000174u32,
DB_PERF_SEL_OREO_TT_stalls               = 0x00000175u32,
DB_PERF_SEL_OREO_ST_load                 = 0x00000176u32,
DB_PERF_SEL_OREO_ST_read                 = 0x00000177u32,
DB_PERF_SEL_OREO_ST_stalls               = 0x00000178u32,
DB_PERF_SEL_OREO_WT_load                 = 0x00000179u32,
DB_PERF_SEL_OREO_WT_read                 = 0x0000017au32,
DB_PERF_SEL_OREO_SB_misses               = 0x0000017bu32,
DB_PERF_SEL_OREO_SB_hits                 = 0x0000017cu32,
DB_PERF_SEL_OREO_SB_evicts               = 0x0000017du32,
DB_PERF_SEL_OREO_SB_stalls               = 0x0000017eu32,
DB_PERF_SEL_OREO_Events_load             = 0x0000017fu32,
DB_PERF_SEL_OREO_Events_transition       = 0x00000180u32,
DB_PERF_SEL_OREO_Events_non_transition   = 0x00000181u32,
DB_PERF_SEL_OREO_Events_delayed          = 0x00000182u32,
DB_PERF_SEL_OREO_Events_stalls           = 0x00000183u32,
}

/*
 * PixelPipeCounterId enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PixelPipeCounterId {
PIXEL_PIPE_OCCLUSION_COUNT_0             = 0x00000000u32,
PIXEL_PIPE_OCCLUSION_COUNT_1             = 0x00000001u32,
PIXEL_PIPE_OCCLUSION_COUNT_2             = 0x00000002u32,
PIXEL_PIPE_OCCLUSION_COUNT_3             = 0x00000003u32,
PIXEL_PIPE_SCREEN_MIN_EXTENTS_0          = 0x00000004u32,
PIXEL_PIPE_SCREEN_MAX_EXTENTS_0          = 0x00000005u32,
PIXEL_PIPE_SCREEN_MIN_EXTENTS_1          = 0x00000006u32,
PIXEL_PIPE_SCREEN_MAX_EXTENTS_1          = 0x00000007u32,
}

/*
 * PixelPipeStride enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum PixelPipeStride {
PIXEL_PIPE_STRIDE_32_BITS                = 0x00000000u32,
PIXEL_PIPE_STRIDE_64_BITS                = 0x00000001u32,
PIXEL_PIPE_STRIDE_128_BITS               = 0x00000002u32,
PIXEL_PIPE_STRIDE_256_BITS               = 0x00000003u32,
}

/*
 * RingCounterControl enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RingCounterControl {
COUNTER_RING_SPLIT                       = 0x00000000u32,
COUNTER_RING_0                           = 0x00000001u32,
COUNTER_RING_1                           = 0x00000002u32,
}

/*
 * StencilOp enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum StencilOp {
STENCIL_KEEP                             = 0x00000000u32,
STENCIL_ZERO                             = 0x00000001u32,
STENCIL_ONES                             = 0x00000002u32,
STENCIL_REPLACE_TEST                     = 0x00000003u32,
STENCIL_REPLACE_OP                       = 0x00000004u32,
STENCIL_ADD_CLAMP                        = 0x00000005u32,
STENCIL_SUB_CLAMP                        = 0x00000006u32,
STENCIL_INVERT                           = 0x00000007u32,
STENCIL_ADD_WRAP                         = 0x00000008u32,
STENCIL_SUB_WRAP                         = 0x00000009u32,
STENCIL_AND                              = 0x0000000au32,
STENCIL_OR                               = 0x0000000bu32,
STENCIL_XOR                              = 0x0000000cu32,
STENCIL_NAND                             = 0x0000000du32,
STENCIL_NOR                              = 0x0000000eu32,
STENCIL_XNOR                             = 0x0000000fu32,
}

/*
 * ZLimitSumm enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ZLimitSumm {
FORCE_SUMM_OFF                           = 0x00000000u32,
FORCE_SUMM_MINZ                          = 0x00000001u32,
FORCE_SUMM_MAXZ                          = 0x00000002u32,
FORCE_SUMM_BOTH                          = 0x00000003u32,
}

/*
 * ZModeForce enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ZModeForce {
NO_FORCE                                 = 0x00000000u32,
FORCE_EARLY_Z                            = 0x00000001u32,
FORCE_LATE_Z                             = 0x00000002u32,
FORCE_RE_Z                               = 0x00000003u32,
}

/*
 * ZOrder enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ZOrder {
LATE_Z                                   = 0x00000000u32,
EARLY_Z_THEN_LATE_Z                      = 0x00000001u32,
RE_Z                                     = 0x00000002u32,
EARLY_Z_THEN_RE_Z                        = 0x00000003u32,
}

/*
 * ZSamplePosition enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ZSamplePosition {
Z_SAMPLE_CENTER                          = 0x00000000u32,
Z_SAMPLE_CENTROID                        = 0x00000001u32,
}

/*
 * SU_PERFCNT_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SU_PERFCNT_SEL {
PERF_PAPC_PASX_REQ                       = 0x00000000u32,
PERF_PAPC_PASX_VTX_KILL_DISCARD          = 0x00000006u32,
PERF_PAPC_PASX_VTX_NAN_DISCARD           = 0x00000007u32,
PERF_CLPR_INPUT_PRIM                     = 0x00000008u32,
PERF_CLPR_INPUT_NULL_PRIM                = 0x00000009u32,
PERF_CLPR_INPUT_EVENT                    = 0x0000000au32,
PERF_CLPR_INPUT_FIRST_OF_SUBGROUP        = 0x0000000bu32,
PERF_CLPR_INPUT_END_OF_PACKET            = 0x0000000cu32,
PERF_CLPR_INPUT_EXTENDED_EVENT           = 0x0000000du32,
PERF_PAPC_CLPR_CULL_PRIM                 = 0x0000000eu32,
PERF_PAPC_CLPR_VVUCP_CULL_PRIM           = 0x0000000fu32,
PERF_PAPC_CLPR_VV_CULL_PRIM              = 0x00000010u32,
PERF_PAPC_CLPR_UCP_CULL_PRIM             = 0x00000011u32,
PERF_PAPC_CLPR_VTX_KILL_CULL_PRIM        = 0x00000012u32,
PERF_PAPC_CLPR_VTX_NAN_CULL_PRIM         = 0x00000013u32,
PERF_PAPC_CLPR_CULL_TO_NULL_PRIM         = 0x00000014u32,
PERF_PAPC_CLPR_VVUCP_CLIP_PRIM           = 0x00000015u32,
PERF_PAPC_CLPR_VV_CLIP_PRIM              = 0x00000016u32,
PERF_PAPC_CLPR_UCP_CLIP_PRIM             = 0x00000017u32,
PERF_PAPC_CLPR_POINT_CLIP_CANDIDATE      = 0x00000018u32,
PERF_PAPC_CLPR_CLIP_PLANE_CNT_1          = 0x00000019u32,
PERF_PAPC_CLPR_CLIP_PLANE_CNT_2          = 0x0000001au32,
PERF_PAPC_CLPR_CLIP_PLANE_CNT_3          = 0x0000001bu32,
PERF_PAPC_CLPR_CLIP_PLANE_CNT_4          = 0x0000001cu32,
PERF_PAPC_CLPR_CLIP_PLANE_CNT_5_8        = 0x0000001du32,
PERF_CLPR_CLIP_PLANE_CNT_9_PLUS          = 0x0000001eu32,
PERF_PAPC_CLPR_CLIP_PLANE_NEAR           = 0x0000001fu32,
PERF_PAPC_CLPR_CLIP_PLANE_FAR            = 0x00000020u32,
PERF_PAPC_CLPR_CLIP_PLANE_LEFT           = 0x00000021u32,
PERF_PAPC_CLPR_CLIP_PLANE_RIGHT          = 0x00000022u32,
PERF_PAPC_CLPR_CLIP_PLANE_TOP            = 0x00000023u32,
PERF_PAPC_CLPR_CLIP_PLANE_BOTTOM         = 0x00000024u32,
PERF_PAPC_CLPR_RASTER_KILL_CULL_PRIM     = 0x00000026u32,
PERF_PAPC_CLSM_NULL_PRIM                 = 0x00000027u32,
PERF_PAPC_CLSM_TOTALLY_VISIBLE_PRIM      = 0x00000028u32,
PERF_PAPC_CLSM_CULL_TO_NULL_PRIM         = 0x00000029u32,
PERF_PAPC_CLSM_OUT_PRIM_CNT_1            = 0x0000002au32,
PERF_PAPC_CLSM_OUT_PRIM_CNT_2            = 0x0000002bu32,
PERF_PAPC_CLSM_OUT_PRIM_CNT_3            = 0x0000002cu32,
PERF_PAPC_CLSM_OUT_PRIM_CNT_4            = 0x0000002du32,
PERF_PAPC_CLSM_OUT_PRIM_CNT_5_8          = 0x0000002eu32,
PERF_PAPC_CLSM_OUT_PRIM_CNT_9_PLUS       = 0x0000002fu32,
PERF_PAPC_CLIPGA_VTE_KILL_PRIM           = 0x00000030u32,
PERF_PAPC_SU_INPUT_PRIM                  = 0x00000031u32,
PERF_PAPC_SU_INPUT_CLIP_PRIM             = 0x00000032u32,
PERF_PAPC_SU_INPUT_NULL_PRIM             = 0x00000033u32,
PERF_PAPC_SU_INPUT_PRIM_DUAL             = 0x00000034u32,
PERF_PAPC_SU_INPUT_CLIP_PRIM_DUAL        = 0x00000035u32,
PERF_PAPC_SU_ZERO_AREA_CULL_PRIM         = 0x00000036u32,
PERF_PAPC_SU_BACK_FACE_CULL_PRIM         = 0x00000037u32,
PERF_PAPC_SU_FRONT_FACE_CULL_PRIM        = 0x00000038u32,
PERF_PAPC_SU_POLYMODE_FACE_CULL          = 0x00000039u32,
PERF_PAPC_SU_POLYMODE_BACK_CULL          = 0x0000003au32,
PERF_PAPC_SU_POLYMODE_FRONT_CULL         = 0x0000003bu32,
PERF_PAPC_SU_POLYMODE_INVALID_FILL       = 0x0000003cu32,
PERF_PAPC_SU_OUTPUT_PRIM                 = 0x0000003du32,
PERF_PAPC_SU_OUTPUT_CLIP_PRIM            = 0x0000003eu32,
PERF_PAPC_SU_OUTPUT_NULL_PRIM            = 0x0000003fu32,
PERF_PAPC_SU_OUTPUT_EVENT_FLAG           = 0x00000040u32,
PERF_PAPC_SU_OUTPUT_FIRST_PRIM_SLOT      = 0x00000041u32,
PERF_PAPC_SU_OUTPUT_END_OF_PACKET        = 0x00000042u32,
PERF_PAPC_SU_OUTPUT_POLYMODE_FACE        = 0x00000043u32,
PERF_PAPC_SU_OUTPUT_POLYMODE_BACK        = 0x00000044u32,
PERF_PAPC_SU_OUTPUT_POLYMODE_FRONT       = 0x00000045u32,
PERF_PAPC_SU_OUT_CLIP_POLYMODE_FACE      = 0x00000046u32,
PERF_PAPC_SU_OUT_CLIP_POLYMODE_BACK      = 0x00000047u32,
PERF_PAPC_SU_OUT_CLIP_POLYMODE_FRONT     = 0x00000048u32,
PERF_PAPC_SU_OUTPUT_PRIM_DUAL            = 0x00000049u32,
PERF_PAPC_SU_OUTPUT_CLIP_PRIM_DUAL       = 0x0000004au32,
PERF_PAPC_SU_OUTPUT_POLYMODE_DUAL        = 0x0000004bu32,
PERF_PAPC_SU_OUTPUT_CLIP_POLYMODE_DUAL   = 0x0000004cu32,
PERF_PAPC_PASX_REQ_IDLE                  = 0x0000004du32,
PERF_PAPC_PASX_REQ_BUSY                  = 0x0000004eu32,
PERF_PAPC_PASX_REQ_STALLED               = 0x0000004fu32,
PERF_PAPC_PASX_REC_IDLE                  = 0x00000050u32,
PERF_PAPC_PASX_REC_BUSY                  = 0x00000051u32,
PERF_PAPC_PASX_REC_STARVED_SX            = 0x00000052u32,
PERF_PAPC_PASX_REC_STALLED               = 0x00000053u32,
PERF_PAPC_PASX_REC_STALLED_POS_MEM       = 0x00000054u32,
PERF_PAPC_PASX_REC_STALLED_CCGSM_IN      = 0x00000055u32,
PERF_PAPC_CCGSM_IDLE                     = 0x00000056u32,
PERF_PAPC_CCGSM_BUSY                     = 0x00000057u32,
PERF_PAPC_CCGSM_STALLED                  = 0x00000058u32,
PERF_PAPC_CLPRIM_IDLE                    = 0x00000059u32,
PERF_PAPC_CLPRIM_BUSY                    = 0x0000005au32,
PERF_PAPC_CLPRIM_STALLED                 = 0x0000005bu32,
PERF_PAPC_CLPRIM_STARVED_CCGSM           = 0x0000005cu32,
PERF_PAPC_CLIPSM_IDLE                    = 0x0000005du32,
PERF_PAPC_CLIPSM_BUSY                    = 0x0000005eu32,
PERF_PAPC_CLIPSM_WAIT_CLIP_VERT_ENGH     = 0x0000005fu32,
PERF_PAPC_CLIPSM_WAIT_HIGH_PRI_SEQ       = 0x00000060u32,
PERF_PAPC_CLIPSM_WAIT_CLIPGA             = 0x00000061u32,
PERF_PAPC_CLIPSM_WAIT_AVAIL_VTE_CLIP     = 0x00000062u32,
PERF_PAPC_CLIPSM_WAIT_CLIP_OUTSM         = 0x00000063u32,
PERF_PAPC_CLIPGA_IDLE                    = 0x00000064u32,
PERF_PAPC_CLIPGA_BUSY                    = 0x00000065u32,
PERF_PAPC_CLIPGA_STARVED_VTE_CLIP        = 0x00000066u32,
PERF_PAPC_CLIPGA_STALLED                 = 0x00000067u32,
PERF_PAPC_CLIP_IDLE                      = 0x00000068u32,
PERF_PAPC_CLIP_BUSY                      = 0x00000069u32,
PERF_PAPC_SU_IDLE                        = 0x0000006au32,
PERF_PAPC_SU_BUSY                        = 0x0000006bu32,
PERF_PAPC_SU_STARVED_CLIP                = 0x0000006cu32,
PERF_PAPC_SU_STALLED_SC                  = 0x0000006du32,
PERF_PAPC_CL_DYN_SCLK_VLD                = 0x0000006eu32,
PERF_PAPC_SU_DYN_SCLK_VLD                = 0x0000006fu32,
PERF_PAPC_PA_REG_SCLK_VLD                = 0x00000070u32,
PERF_PAPC_SU_SE0_PRIM_FILTER_CULL        = 0x00000078u32,
PERF_PAPC_SU_SE1_PRIM_FILTER_CULL        = 0x00000079u32,
PERF_PAPC_SU_SE0_OUTPUT_PRIM             = 0x0000007bu32,
PERF_PAPC_SU_SE1_OUTPUT_PRIM             = 0x0000007cu32,
PERF_PAPC_SU_ALL_OUTPUT_PRIM             = 0x0000007du32,
PERF_PAPC_SU_SE0_OUTPUT_NULL_PRIM        = 0x0000007eu32,
PERF_PAPC_SU_SE1_OUTPUT_NULL_PRIM        = 0x0000007fu32,
PERF_PAPC_SU_ALL_OUTPUT_NULL_PRIM        = 0x00000080u32,
PERF_PAPC_SU_SE0_STALLED_SC              = 0x00000083u32,
PERF_PAPC_SU_SE1_STALLED_SC              = 0x00000084u32,
PERF_PAPC_SU_ALL_STALLED_SC              = 0x00000085u32,
PERF_PAPC_CLSM_CLIPPING_PRIM             = 0x00000086u32,
PERF_PAPC_SU_CULLED_PRIM                 = 0x00000087u32,
PERF_PAPC_SU_OUTPUT_EOPG                 = 0x00000088u32,
PERF_PAPC_SU_SE2_PRIM_FILTER_CULL        = 0x00000089u32,
PERF_PAPC_SU_SE3_PRIM_FILTER_CULL        = 0x0000008au32,
PERF_PAPC_SU_SE2_OUTPUT_PRIM             = 0x0000008bu32,
PERF_PAPC_SU_SE3_OUTPUT_PRIM             = 0x0000008cu32,
PERF_PAPC_SU_SE2_OUTPUT_NULL_PRIM        = 0x0000008du32,
PERF_PAPC_SU_SE3_OUTPUT_NULL_PRIM        = 0x0000008eu32,
PERF_PAPC_SU_SE2_STALLED_SC              = 0x00000097u32,
PERF_PAPC_SU_SE3_STALLED_SC              = 0x00000098u32,
PERF_SU_SMALL_PRIM_FILTER_CULL_CNT       = 0x00000099u32,
PERF_SMALL_PRIM_CULL_PRIM_1X1            = 0x0000009au32,
PERF_SMALL_PRIM_CULL_PRIM_2X1            = 0x0000009bu32,
PERF_SMALL_PRIM_CULL_PRIM_1X2            = 0x0000009cu32,
PERF_SMALL_PRIM_CULL_PRIM_2X2            = 0x0000009du32,
PERF_SMALL_PRIM_CULL_PRIM_3X1            = 0x0000009eu32,
PERF_SMALL_PRIM_CULL_PRIM_1X3            = 0x0000009fu32,
PERF_SMALL_PRIM_CULL_PRIM_3X2            = 0x000000a0u32,
PERF_SMALL_PRIM_CULL_PRIM_2X3            = 0x000000a1u32,
PERF_SMALL_PRIM_CULL_PRIM_NX1            = 0x000000a2u32,
PERF_SMALL_PRIM_CULL_PRIM_1XN            = 0x000000a3u32,
PERF_SMALL_PRIM_CULL_PRIM_NX2            = 0x000000a4u32,
PERF_SMALL_PRIM_CULL_PRIM_2XN            = 0x000000a5u32,
PERF_SC0_QUALIFIED_SEND_BUSY_EVENT       = 0x000000a9u32,
PERF_SC0_QUALIFIED_SEND_NOT_BUSY_EVENT   = 0x000000aau32,
PERF_SC1_QUALIFIED_SEND_BUSY_EVENT       = 0x000000abu32,
PERF_SC1_QUALIFIED_SEND_NOT_BUSY_EVENT   = 0x000000acu32,
PERF_SC2_QUALIFIED_SEND_BUSY_EVENT       = 0x000000adu32,
PERF_SC2_QUALIFIED_SEND_NOT_BUSY_EVENT   = 0x000000aeu32,
PERF_SC3_QUALIFIED_SEND_BUSY_EVENT       = 0x000000afu32,
PERF_SC3_QUALIFIED_SEND_NOT_BUSY_EVENT   = 0x000000b0u32,
PERF_PA_VERTEX_FIFO_FULL                 = 0x000000b1u32,
PERF_PA_PRIMIC_TO_CLPRIM_FIFO_FULL       = 0x000000b2u32,
PERF_PA_FETCH_TO_PRIMIC_P_FIFO_FULL      = 0x000000b3u32,
PERF_ENGG_CSB_MACHINE_IS_STARVED         = 0x000000b7u32,
PERF_ENGG_CSB_MACHINE_STALLED_BY_CSB_MEMORY = 0x000000b8u32,
PERF_ENGG_CSB_MACHINE_STALLED_BY_SPI     = 0x000000b9u32,
PERF_ENGG_CSB_GE_INPUT_FIFO_FULL         = 0x000000bau32,
PERF_ENGG_CSB_PAYLOAD_INPUT_FIFO_FULL    = 0x000000bcu32,
PERF_ENGG_CSB_GE_INPUT_FIFO_POP_BIT      = 0x000000bdu32,
PERF_ENGG_CSB_PRIM_COUNT_EQ0             = 0x000000beu32,
PERF_ENGG_CSB_NULL_SUBGROUP              = 0x000000bfu32,
PERF_ENGG_CSB_GE_SENDING_SUBGROUP        = 0x000000c0u32,
PERF_ENGG_CSB_GE_MEMORY_FULL             = 0x000000c1u32,
PERF_ENGG_CSB_GE_MEMORY_EMPTY            = 0x000000c2u32,
PERF_ENGG_CSB_SPI_MEMORY_FULL            = 0x000000c3u32,
PERF_ENGG_CSB_SPI_MEMORY_EMPTY           = 0x000000c4u32,
PERF_ENGG_INDEX_REQ_NULL_REQUEST         = 0x000000e0u32,
PERF_ENGG_INDEX_RET_0_NEW_VERTS_THIS_PRIM = 0x000000e1u32,
PERF_ENGG_INDEX_RET_1_NEW_VERTS_THIS_PRIM = 0x000000e2u32,
PERF_ENGG_INDEX_RET_2_NEW_VERTS_THIS_PRIM = 0x000000e3u32,
PERF_ENGG_INDEX_RET_3_NEW_VERTS_THIS_PRIM = 0x000000e4u32,
PERF_ENGG_INDEX_REQ_STARVED              = 0x000000e5u32,
PERF_ENGG_INDEX_REQ_IDLE_AND_STALLED_BY_REQ2RTN_FIFO_FULL = 0x000000e6u32,
PERF_ENGG_INDEX_REQ_BUSY_AND_STALLED_BY_REQ2RTN_FIFO_FULL = 0x000000e7u32,
PERF_ENGG_INDEX_REQ_STALLED_BY_SX_CREDITS = 0x000000e8u32,
PERF_ENGG_INDEX_RET_REQ2RTN_FIFO_FULL    = 0x000000e9u32,
PERF_ENGG_INDEX_RET_REQ2RTN_FIFO_EMPTY   = 0x000000eau32,
PERF_ENGG_INDEX_RET_SX_RECEIVE_FIFO_FULL = 0x000000ebu32,
PERF_ENGG_INDEX_RET_SXRX_STARVED_BY_CSB  = 0x000000ecu32,
PERF_ENGG_INDEX_RET_SXRX_STARVED_BY_PRIMS = 0x000000edu32,
PERF_ENGG_INDEX_RET_SXRX_STALLED_BY_PRIM_INDICES_CSB_FIFO = 0x000000eeu32,
PERF_ENGG_INDEX_RET_SXRX_STALLED_BY_PRIM_INDICES_FIFO = 0x000000efu32,
PERF_ENGG_INDEX_RET_SXRX_READING_EVENT   = 0x000000f0u32,
PERF_ENGG_INDEX_RET_SXRX_READING_NULL_SUBGROUP = 0x000000f1u32,
PERF_ENGG_INDEX_RET_SXRX_READING_SUBGROUP_PRIMCOUNT_EQ0 = 0x000000f2u32,
PERF_ENGG_INDEX_RET_SXRX_READING_QDWORD_0_VALID_PRIMS_NOPL = 0x000000f3u32,
PERF_ENGG_INDEX_RET_SXRX_READING_QDWORD_1_VALID_PRIMS_NOPL = 0x000000f4u32,
PERF_ENGG_INDEX_RET_SXRX_READING_QDWORD_2_VALID_PRIMS_NOPL = 0x000000f5u32,
PERF_ENGG_INDEX_RET_SXRX_READING_QDWORD_3_VALID_PRIMS_NOPL = 0x000000f6u32,
PERF_ENGG_INDEX_RET_SXRX_READING_QDWORD_4_VALID_PRIMS_NOPL = 0x000000f7u32,
PERF_ENGG_INDEX_RET_SXRX_READING_QDWORD_0_VALID_PRIMS_PL = 0x000000f8u32,
PERF_ENGG_INDEX_RET_SXRX_READING_QDWORD_1_VALID_PRIMS_PL = 0x000000f9u32,
PERF_ENGG_INDEX_RET_SXRX_READING_QDWORD_2_VALID_PRIMS_PL = 0x000000fau32,
PERF_ENGG_INDEX_RET_SXRX_READING_QDWORD_3_VALID_PRIMS_PL = 0x000000fbu32,
PERF_ENGG_INDEX_RET_SXRX_READING_QDWORD_4_VALID_PRIMS_PL = 0x000000fcu32,
PERF_ENGG_INDEX_PRIM_IF_STALLED_BY_FULL_FETCH_TO_PRIMIC_P_FIFO = 0x00000102u32,
PERF_ENGG_INDEX_PRIM_IF_STALLED_BY_FULL_FETCH_TO_PRIMIC_S_FIFO = 0x00000103u32,
PERF_ENGG_INDEX_PRIM_IF_STARVED_BY_NO_CSB = 0x00000104u32,
PERF_ENGG_INDEX_PRIM_IF_STARVED_BY_NO_PRIM = 0x00000105u32,
PERF_ENGG_INDEX_PRIM_IF_FETCH_TO_PRIMIC_P_FIFO_WRITE = 0x00000106u32,
PERF_ENGG_INDEX_PRIM_IF_FETCH_TO_PRIMIC_P_FIFO_NO_WRITE = 0x00000107u32,
PERF_ENGG_POS_REQ_STARVED                = 0x00000108u32,
PERF_ENGG_INDEX_RET_SXRX_NULL_DROPPER_STALLED_BY_FULL_PRIM_FIFO = 0x00000109u32,
PERF_ENGG_BUSY                           = 0x0000010au32,
PERF_CLIPSM_CULL_PRIMS_CNT               = 0x0000010bu32,
PERF_PH_SEND_1_SC                        = 0x0000010cu32,
PERF_PH_SEND_2_SC                        = 0x0000010du32,
PERF_PH_SEND_3_SC                        = 0x0000010eu32,
PERF_PH_SEND_4_SC                        = 0x0000010fu32,
PERF_OUTPUT_PRIM_1_SC                    = 0x00000110u32,
PERF_OUTPUT_PRIM_2_SC                    = 0x00000111u32,
PERF_OUTPUT_PRIM_3_SC                    = 0x00000112u32,
PERF_OUTPUT_PRIM_4_SC                    = 0x00000113u32,
PERF_PASX_POS_VECTOR                     = 0x00000114u32,
PERF_PASX_MISC_VECTOR                    = 0x00000115u32,
PERF_PASX_CCDIST0_VECTOR                 = 0x00000116u32,
PERF_PASX_CCDIST1_VECTOR                 = 0x00000117u32,
PERF_PASX_STEREO_POS_VECTOR              = 0x00000118u32,
PERF_CLPR_INPUT_SEND                     = 0x00000119u32,
PERF_SU_INPUT_SEND                       = 0x0000011au32,
PERF_SU_OUTPUT_SEND                      = 0x0000011bu32,
PERF_PAPC_SU_SE4_PRIM_FILTER_CULL        = 0x0000011cu32,
PERF_PAPC_SU_SE5_PRIM_FILTER_CULL        = 0x0000011du32,
PERF_PAPC_SU_SE4_OUTPUT_PRIM             = 0x0000011eu32,
PERF_PAPC_SU_SE5_OUTPUT_PRIM             = 0x0000011fu32,
PERF_PAPC_SU_SE4_OUTPUT_NULL_PRIM        = 0x00000120u32,
PERF_PAPC_SU_SE5_OUTPUT_NULL_PRIM        = 0x00000121u32,
PERF_PAPC_SU_SE4_STALLED_SC              = 0x00000122u32,
PERF_PAPC_SU_SE5_STALLED_SC              = 0x00000123u32,
PERF_ENGG_INDEX_RET0_NEW_VERTS           = 0x00000124u32,
PERF_ENGG_INDEX_RET1_NEW_VERTS           = 0x00000125u32,
PERF_ENGG_INDEX_RET2_NEW_VERTS           = 0x00000126u32,
PERF_ENGG_INDEX_RET3_NEW_VERTS           = 0x00000127u32,
PERF_ENGG_INDEX_RET4_NEW_VERTS           = 0x00000128u32,
PERF_ENGG_INDEX_RET5_NEW_VERTS           = 0x00000129u32,
PERF_ENGG_INDEX_RET6_NEW_VERTS           = 0x0000012au32,
PERF_ENGG_INDEX_RET7_NEW_VERTS           = 0x0000012bu32,
PERF_ENGG_INDEX_RET8_NEW_VERTS           = 0x0000012cu32,
PERF_ENGG_INDEX_RET9_NEW_VERTS           = 0x0000012du32,
PERF_ENGG_INDEX_RET10_NEW_VERTS          = 0x0000012eu32,
PERF_ENGG_INDEX_RET11_NEW_VERTS          = 0x0000012fu32,
PERF_ENGG_INDEX_RET12_NEW_VERTS          = 0x00000130u32,
PERF_PH_SEND_5_SC                        = 0x00000131u32,
PERF_PH_SEND_6_SC                        = 0x00000132u32,
PERF_OUTPUT_PRIM_5_SC                    = 0x00000133u32,
PERF_OUTPUT_PRIM_6_SC                    = 0x00000134u32,
PERF_CLPR_BACK_PRIM                      = 0x00000135u32,
PERF_PA_BUSY                             = 0x00000136u32,
}

/*
 * RMIPerfSel enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum RMIPerfSel {
RMI_PERF_SEL_NONE                        = 0x00000000u32,
RMI_PERF_SEL_BUSY                        = 0x00000001u32,
RMI_PERF_SEL_REG_CLK_VLD                 = 0x00000002u32,
RMI_PERF_SEL_DYN_CLK_CMN_VLD             = 0x00000003u32,
RMI_PERF_SEL_DYN_CLK_RB_VLD              = 0x00000004u32,
RMI_PERF_SEL_DYN_CLK_PERF_VLD            = 0x00000005u32,
RMI_PERF_SEL_PERF_WINDOW                 = 0x00000006u32,
RMI_PERF_SEL_EVENT_SEND                  = 0x00000007u32,
RMI_PERF_SEL_RB_RMI_WRREQ_ALL_CID        = 0x00000008u32,
RMI_PERF_SEL_RB_RMI_WRREQ_TO_WRRET_BUSY  = 0x00000009u32,
RMI_PERF_SEL_RB_RMI_WRREQ_CID0           = 0x0000000au32,
RMI_PERF_SEL_RB_RMI_WRREQ_CID1           = 0x0000000bu32,
RMI_PERF_SEL_RB_RMI_WRREQ_CID2           = 0x0000000cu32,
RMI_PERF_SEL_RB_RMI_WRREQ_CID3           = 0x0000000du32,
RMI_PERF_SEL_RB_RMI_WRREQ_CID4           = 0x0000000eu32,
RMI_PERF_SEL_RB_RMI_WRREQ_CID5           = 0x0000000fu32,
RMI_PERF_SEL_RB_RMI_WRREQ_CID6           = 0x00000010u32,
RMI_PERF_SEL_RB_RMI_WRREQ_CID7           = 0x00000011u32,
RMI_PERF_SEL_RB_RMI_32BWRREQ_INFLIGHT_ALL_ORONE_CID = 0x00000012u32,
RMI_PERF_SEL_RB_RMI_WRREQ_BURST_LENGTH_ALL_ORONE_CID = 0x00000013u32,
RMI_PERF_SEL_RB_RMI_WRREQ_BURST_ALL_ORONE_CID = 0x00000014u32,
RMI_PERF_SEL_RB_RMI_WRREQ_RESIDENCY      = 0x00000015u32,
RMI_PERF_SEL_RMI_RB_WRRET_VALID_ALL_CID  = 0x00000016u32,
RMI_PERF_SEL_RMI_RB_WRRET_VALID_CID0     = 0x00000017u32,
RMI_PERF_SEL_RMI_RB_WRRET_VALID_CID1     = 0x00000018u32,
RMI_PERF_SEL_RMI_RB_WRRET_VALID_CID2     = 0x00000019u32,
RMI_PERF_SEL_RMI_RB_WRRET_VALID_CID3     = 0x0000001au32,
RMI_PERF_SEL_RMI_RB_WRRET_VALID_CID4     = 0x0000001bu32,
RMI_PERF_SEL_RMI_RB_WRRET_VALID_CID5     = 0x0000001cu32,
RMI_PERF_SEL_RMI_RB_WRRET_VALID_CID6     = 0x0000001du32,
RMI_PERF_SEL_RMI_RB_WRRET_VALID_CID7     = 0x0000001eu32,
RMI_PERF_SEL_RMI_RB_WRRET_VALID_NACK0    = 0x0000001fu32,
RMI_PERF_SEL_RMI_RB_WRRET_VALID_NACK1    = 0x00000020u32,
RMI_PERF_SEL_RMI_RB_WRRET_VALID_NACK2    = 0x00000021u32,
RMI_PERF_SEL_RMI_RB_WRRET_VALID_NACK3    = 0x00000022u32,
RMI_PERF_SEL_RB_RMI_32BRDREQ_ALL_CID     = 0x00000023u32,
RMI_PERF_SEL_RB_RMI_RDREQ_ALL_CID        = 0x00000024u32,
RMI_PERF_SEL_RB_RMI_RDREQ_TO_RDRET_BUSY  = 0x00000025u32,
RMI_PERF_SEL_RB_RMI_32BRDREQ_CID0        = 0x00000026u32,
RMI_PERF_SEL_RB_RMI_32BRDREQ_CID1        = 0x00000027u32,
RMI_PERF_SEL_RB_RMI_32BRDREQ_CID2        = 0x00000028u32,
RMI_PERF_SEL_RB_RMI_32BRDREQ_CID3        = 0x00000029u32,
RMI_PERF_SEL_RB_RMI_32BRDREQ_CID4        = 0x0000002au32,
RMI_PERF_SEL_RB_RMI_32BRDREQ_CID5        = 0x0000002bu32,
RMI_PERF_SEL_RB_RMI_32BRDREQ_CID6        = 0x0000002cu32,
RMI_PERF_SEL_RB_RMI_32BRDREQ_CID7        = 0x0000002du32,
RMI_PERF_SEL_RB_RMI_RDREQ_CID0           = 0x0000002eu32,
RMI_PERF_SEL_RB_RMI_RDREQ_CID1           = 0x0000002fu32,
RMI_PERF_SEL_RB_RMI_RDREQ_CID2           = 0x00000030u32,
RMI_PERF_SEL_RB_RMI_RDREQ_CID3           = 0x00000031u32,
RMI_PERF_SEL_RB_RMI_RDREQ_CID4           = 0x00000032u32,
RMI_PERF_SEL_RB_RMI_RDREQ_CID5           = 0x00000033u32,
RMI_PERF_SEL_RB_RMI_RDREQ_CID6           = 0x00000034u32,
RMI_PERF_SEL_RB_RMI_RDREQ_CID7           = 0x00000035u32,
RMI_PERF_SEL_RB_RMI_32BRDREQ_INFLIGHT_ALL_ORONE_CID = 0x00000036u32,
RMI_PERF_SEL_RB_RMI_RDREQ_BURST_LENGTH_ALL_ORONE_CID = 0x00000037u32,
RMI_PERF_SEL_RB_RMI_RDREQ_BURST_ALL_ORONE_CID = 0x00000038u32,
RMI_PERF_SEL_RB_RMI_RDREQ_RESIDENCY      = 0x00000039u32,
RMI_PERF_SEL_RMI_RB_32BRDRET_VALID_ALL_CID = 0x0000003au32,
RMI_PERF_SEL_RMI_RB_32BRDRET_VALID_CID0  = 0x0000003bu32,
RMI_PERF_SEL_RMI_RB_32BRDRET_VALID_CID1  = 0x0000003cu32,
RMI_PERF_SEL_RMI_RB_32BRDRET_VALID_CID2  = 0x0000003du32,
RMI_PERF_SEL_RMI_RB_32BRDRET_VALID_CID3  = 0x0000003eu32,
RMI_PERF_SEL_RMI_RB_32BRDRET_VALID_CID4  = 0x0000003fu32,
RMI_PERF_SEL_RMI_RB_32BRDRET_VALID_CID5  = 0x00000040u32,
RMI_PERF_SEL_RMI_RB_32BRDRET_VALID_CID6  = 0x00000041u32,
RMI_PERF_SEL_RMI_RB_32BRDRET_VALID_CID7  = 0x00000042u32,
RMI_PERF_SEL_RMI_RB_32BRDRET_VALID_NACK0 = 0x00000043u32,
RMI_PERF_SEL_RMI_RB_32BRDRET_VALID_NACK1 = 0x00000044u32,
RMI_PERF_SEL_RMI_RB_32BRDRET_VALID_NACK2 = 0x00000045u32,
RMI_PERF_SEL_RMI_RB_32BRDRET_VALID_NACK3 = 0x00000046u32,
RMI_PERF_SEL_RB_RMI_WR_FIFO_MAX          = 0x00000047u32,
RMI_PERF_SEL_RB_RMI_WR_FIFO_EMPTY        = 0x00000048u32,
RMI_PERF_SEL_RB_RMI_WR_IDLE              = 0x00000049u32,
RMI_PERF_SEL_RB_RMI_WR_STARVE            = 0x0000004au32,
RMI_PERF_SEL_RB_RMI_WR_STALL             = 0x0000004bu32,
RMI_PERF_SEL_RB_RMI_WR_BUSY              = 0x0000004cu32,
RMI_PERF_SEL_RB_RMI_WR_INTF_BUSY         = 0x0000004du32,
RMI_PERF_SEL_RB_RMI_RD_FIFO_MAX          = 0x0000004eu32,
RMI_PERF_SEL_RB_RMI_RD_FIFO_EMPTY        = 0x0000004fu32,
RMI_PERF_SEL_RB_RMI_RD_IDLE              = 0x00000050u32,
RMI_PERF_SEL_RB_RMI_RD_STARVE            = 0x00000051u32,
RMI_PERF_SEL_RB_RMI_RD_STALL             = 0x00000052u32,
RMI_PERF_SEL_RB_RMI_RD_BUSY              = 0x00000053u32,
RMI_PERF_SEL_RB_RMI_RD_INTF_BUSY         = 0x00000054u32,
RMI_PERF_SEL_RMI_TC_64BWRREQ_ALL_ORONE_CID = 0x00000055u32,
RMI_PERF_SEL_RMI_TC_64BRDREQ_ALL_ORONE_CID = 0x00000056u32,
RMI_PERF_SEL_RMI_TC_WRREQ_ALL_CID        = 0x00000057u32,
RMI_PERF_SEL_RMI_TC_REQ_BUSY             = 0x00000058u32,
RMI_PERF_SEL_RMI_TC_WRREQ_CID0           = 0x00000059u32,
RMI_PERF_SEL_RMI_TC_WRREQ_CID1           = 0x0000005au32,
RMI_PERF_SEL_RMI_TC_WRREQ_CID2           = 0x0000005bu32,
RMI_PERF_SEL_RMI_TC_WRREQ_CID3           = 0x0000005cu32,
RMI_PERF_SEL_RMI_TC_WRREQ_CID4           = 0x0000005du32,
RMI_PERF_SEL_RMI_TC_WRREQ_CID5           = 0x0000005eu32,
RMI_PERF_SEL_RMI_TC_WRREQ_CID6           = 0x0000005fu32,
RMI_PERF_SEL_RMI_TC_WRREQ_CID7           = 0x00000060u32,
RMI_PERF_SEL_RMI_TC_WRREQ_INFLIGHT_ALL_CID = 0x00000061u32,
RMI_PERF_SEL_TC_RMI_WRRET_VALID_ALL_CID  = 0x00000062u32,
RMI_PERF_SEL_RMI_TC_RDREQ_ALL_CID        = 0x00000063u32,
RMI_PERF_SEL_RMI_TC_RDREQ_CID0           = 0x00000064u32,
RMI_PERF_SEL_RMI_TC_RDREQ_CID1           = 0x00000065u32,
RMI_PERF_SEL_RMI_TC_RDREQ_CID2           = 0x00000066u32,
RMI_PERF_SEL_RMI_TC_RDREQ_CID3           = 0x00000067u32,
RMI_PERF_SEL_RMI_TC_RDREQ_CID4           = 0x00000068u32,
RMI_PERF_SEL_RMI_TC_RDREQ_CID5           = 0x00000069u32,
RMI_PERF_SEL_RMI_TC_RDREQ_CID6           = 0x0000006au32,
RMI_PERF_SEL_RMI_TC_RDREQ_CID7           = 0x0000006bu32,
RMI_PERF_SEL_RMI_TC_STALL_RDREQ          = 0x0000006cu32,
RMI_PERF_SEL_RMI_TC_STALL_WRREQ          = 0x0000006du32,
RMI_PERF_SEL_RMI_TC_STALL_ALLREQ         = 0x0000006eu32,
RMI_PERF_SEL_RMI_TC_CREDIT_FULL_NO_PENDING_SEND = 0x0000006fu32,
RMI_PERF_SEL_RMI_TC_CREDIT_ZERO_PENDING_SEND = 0x00000070u32,
RMI_PERF_SEL_RMI_TC_RDREQ_INFLIGHT_ALL_CID = 0x00000071u32,
RMI_PERF_SEL_TC_RMI_RDRET_VALID_ALL_CID  = 0x00000072u32,
RMI_PERF_SEL_TCIW_INFLIGHT_COUNT         = 0x00000073u32,
RMI_PERF_SEL_TCIW_REQ                    = 0x00000074u32,
RMI_PERF_SEL_TCIW_BUSY                   = 0x00000075u32,
RMI_PERF_SEL_DEMUX_TCIW_FORMATTER_RTS_RTR = 0x00000076u32,
RMI_PERF_SEL_DEMUX_TCIW_FORMATTER_RTSB_RTR = 0x00000077u32,
RMI_PERF_SEL_DEMUX_TCIW_FORMATTER_RTS_RTRB = 0x00000078u32,
RMI_PERF_SEL_DEMUX_TCIW_FORMATTER_RTSB_RTRB = 0x00000079u32,
RMI_PERF_SEL_REORDER_FIFO_REQ            = 0x0000007au32,
RMI_PERF_SEL_REORDER_FIFO_BUSY           = 0x0000007bu32,
RMI_PERF_SEL_RMI_RB_EARLY_WRACK_ALL_CID  = 0x0000007cu32,
RMI_PERF_SEL_RMI_RB_EARLY_WRACK_CID0     = 0x0000007du32,
RMI_PERF_SEL_RMI_RB_EARLY_WRACK_CID1     = 0x0000007eu32,
RMI_PERF_SEL_RMI_RB_EARLY_WRACK_CID2     = 0x0000007fu32,
RMI_PERF_SEL_RMI_RB_EARLY_WRACK_CID3     = 0x00000080u32,
RMI_PERF_SEL_RMI_RB_EARLY_WRACK_CID4     = 0x00000081u32,
RMI_PERF_SEL_RMI_RB_EARLY_WRACK_CID5     = 0x00000082u32,
RMI_PERF_SEL_RMI_RB_EARLY_WRACK_CID6     = 0x00000083u32,
RMI_PERF_SEL_RMI_RB_EARLY_WRACK_CID7     = 0x00000084u32,
RMI_PERF_SEL_CONSUMER_PROBEGEN_READ_RTS_RTR = 0x00000085u32,
RMI_PERF_SEL_CONSUMER_PROBEGEN_WRITE_RTS_RTR = 0x00000086u32,
RMI_PERF_SEL_CONSUMER_PROBEGEN_IN0_RTS_RTR = 0x00000087u32,
RMI_PERF_SEL_CONSUMER_PROBEGEN_IN1_RTS_RTR = 0x00000088u32,
RMI_PERF_SEL_CONSUMER_PROBEGEN_CB_RTS_RTR = 0x00000089u32,
RMI_PERF_SEL_CONSUMER_PROBEGEN_DB_RTS_RTR = 0x0000008au32,
}

/*
 * UTCL1PerfSel enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum UTCL1PerfSel {
UTCL1_PERF_SEL_NONE                      = 0x00000000u32,
UTCL1_PERF_SEL_REQS                      = 0x00000001u32,
UTCL1_PERF_SEL_HITS                      = 0x00000002u32,
UTCL1_PERF_SEL_MISSES                    = 0x00000003u32,
UTCL1_PERF_SEL_MH_RECENT_BUF_HIT         = 0x00000004u32,
UTCL1_PERF_SEL_MH_DUPLICATE_DETECT       = 0x00000005u32,
UTCL1_PERF_SEL_UTCL2_REQS                = 0x00000006u32,
UTCL1_PERF_SEL_UTCL2_RET_XNACK_RETRY     = 0x00000007u32,
UTCL1_PERF_SEL_UTCL2_RET_FAULT           = 0x00000008u32,
UTCL1_PERF_SEL_STALL_UTCL2_CREDITS       = 0x00000009u32,
UTCL1_PERF_SEL_STALL_MH_FULL             = 0x0000000au32,
UTCL1_PERF_SEL_UTCL2_REQS_OUTSTANDING_ACCUM = 0x0000000bu32,
UTCL1_PERF_SEL_UTCL2_RET_CNT             = 0x0000000cu32,
UTCL1_PERF_SEL_RTNS                      = 0x0000000du32,
UTCL1_PERF_SEL_XLAT_REQ_BUSY             = 0x0000000eu32,
UTCL1_PERF_SEL_RANGE_INVREQS             = 0x0000000fu32,
UTCL1_PERF_SEL_INV_ALL_VMID_INVREQS      = 0x00000010u32,
UTCL1_PERF_SEL_BYPASS_REQS               = 0x00000011u32,
UTCL1_PERF_SEL_HIT_INV_FILTER_REQS       = 0x00000012u32,
UTCL1_PERF_SEL_UTCL2_RET_PERM_FAULT      = 0x00000013u32,
UTCL1_PERF_SEL_UTCL2_RET_PRT_FAULT       = 0x00000014u32,
UTCL1_PERF_SEL_CP_INVREQS                = 0x00000015u32,
UTCL1_PERF_SEL_UTCL2_UTCL1_INVREQS       = 0x00000016u32,
UTCL1_PERF_SEL_NUM_UTCL2_RTN_SIZE_4K_64K = 0x00000017u32,
UTCL1_PERF_SEL_NUM_UTCL2_RTN_SIZE_64K_256K = 0x00000018u32,
UTCL1_PERF_SEL_NUM_UTCL2_RTN_SIZE_256K_512K = 0x00000019u32,
UTCL1_PERF_SEL_NUM_UTCL2_RTN_SIZE_512K_1M = 0x0000001au32,
UTCL1_PERF_SEL_NUM_UTCL2_RTN_SIZE_1M_2M  = 0x0000001bu32,
UTCL1_PERF_SEL_NUM_UTCL2_RTN_SIZE_2M_4M  = 0x0000001cu32,
UTCL1_PERF_SEL_NUM_UTCL2_RTN_SIZE_4M_8M  = 0x0000001du32,
UTCL1_PERF_SEL_NUM_UTCL2_RTN_SIZE_8M_16M = 0x0000001eu32,
UTCL1_PERF_SEL_NUM_UTCL2_RTN_SIZE_16M_32M = 0x0000001fu32,
UTCL1_PERF_SEL_NUM_UTCL2_RTN_SIZE_32M_INF = 0x00000020u32,
UTCL1_PERF_SEL_UTCL2_REQ_SQUASHED_NUM    = 0x00000021u32,
UTCL1_PERF_SEL_REQ_NUM_CACHE_CORE_0      = 0x00000022u32,
UTCL1_PERF_SEL_REQ_NUM_CACHE_CORE_1      = 0x00000023u32,
UTCL1_PERF_SEL_REQ_NUM_CACHE_CORE_2      = 0x00000024u32,
UTCL1_PERF_SEL_REQ_NUM_CACHE_CORE_3      = 0x00000025u32,
UTCL1_PERF_SEL_STALL_CYCLES_CACHE_CORE_0 = 0x00000026u32,
UTCL1_PERF_SEL_STALL_CYCLES_CACHE_CORE_1 = 0x00000027u32,
UTCL1_PERF_SEL_STALL_CYCLES_CACHE_CORE_2 = 0x00000028u32,
UTCL1_PERF_SEL_STALL_CYCLES_CACHE_CORE_3 = 0x00000029u32,
UTCL1_PERF_SEL_UTCL1_UTCL2_INVACKS       = 0x0000002au32,
UTCL1_PERF_SEL_UTCL0_UTCL1_INVACKS       = 0x0000002bu32,
UTCL1_PERF_SEL_HITS_PG_SIZE_1            = 0x0000002cu32,
UTCL1_PERF_SEL_HITS_PG_SIZE_2            = 0x0000002du32,
UTCL1_PERF_SEL_HITS_PG_SIZE_3            = 0x0000002eu32,
UTCL1_PERF_SEL_HITS_PG_SIZE_4            = 0x0000002fu32,
UTCL1_PERF_SEL_REQ_TO_MISS_HNDLR_0       = 0x00000030u32,
UTCL1_PERF_SEL_REQ_TO_MISS_HNDLR_1       = 0x00000031u32,
UTCL1_PERF_SEL_REQ_TO_MISS_HNDLR_2       = 0x00000032u32,
UTCL1_PERF_SEL_REQ_TO_MISS_HNDLR_3       = 0x00000033u32,
UTCL1_PERF_SEL_AVG_INV_LATENCY           = 0x00000034u32,
UTCL1_PERF_SEL_NUM_OF_CYCLES_RQ_EXISTS_TO_CC0 = 0x00000035u32,
UTCL1_PERF_SEL_NUM_OF_CYCLES_RQ_EXISTS_TO_CC1 = 0x00000036u32,
UTCL1_PERF_SEL_NUM_OF_CYCLES_RQ_EXISTS_TO_CC2 = 0x00000037u32,
UTCL1_PERF_SEL_NUM_OF_CYCLES_RQ_EXISTS_TO_CC3 = 0x00000038u32,
UTCL1_PERF_SEL_NUM_OF_CYCLES_W_COLLISION_CC0 = 0x00000039u32,
UTCL1_PERF_SEL_NUM_OF_CYCLES_W_COLLISION_CC1 = 0x0000003au32,
UTCL1_PERF_SEL_NUM_OF_CYCLES_W_COLLISION_CC2 = 0x0000003bu32,
UTCL1_PERF_SEL_NUM_OF_CYCLES_W_COLLISION_CC3 = 0x0000003cu32,
UTCL1_PERF_SEL_EVICTIONS_NUM_CC0         = 0x0000003du32,
UTCL1_PERF_SEL_EVICTIONS_NUM_CC1         = 0x0000003eu32,
UTCL1_PERF_SEL_EVICTIONS_NUM_CC2         = 0x0000003fu32,
UTCL1_PERF_SEL_EVICTIONS_NUM_CC3         = 0x00000040u32,
UTCL1_PERF_SEL_ALOG_INTERRUPT            = 0x00000041u32,
UTCL1_PERF_SEL_ALOG_INTERRUPT_DROPPED    = 0x00000042u32,
UTCL1_PERF_SEL_ALOG_CACHE_REQ            = 0x00000043u32,
UTCL1_PERF_SEL_ALOG_CACHE_HIT            = 0x00000044u32,
UTCL1_PERF_SEL_ALOG_STALL_PMM_CREDITS    = 0x00000045u32,
}

/*
 * GC_EA_SE_PERFCOUNT_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GC_EA_SE_PERFCOUNT_SEL {
GC_EA_SE_PERF_SEL_ALWAYS_COUNT           = 0x00000000u32,
GC_EA_SE_PERF_SEL_RDRAM_NUM_BANKS_VLD    = 0x00000001u32,
GC_EA_SE_PERF_SEL_RDRAM_REQ_PER_CLIGRP   = 0x00000002u32,
GC_EA_SE_PERF_SEL_RDRAM_CHAINED_REQ_PER_CLIGRP = 0x00000003u32,
GC_EA_SE_PERF_SEL_RDRAM_LATENCY_START0   = 0x00000004u32,
GC_EA_SE_PERF_SEL_RDRAM_LATENCY_END0     = 0x00000005u32,
GC_EA_SE_PERF_SEL_RDRAM_LATENCY_START1   = 0x00000006u32,
GC_EA_SE_PERF_SEL_RDRAM_LATENCY_END1     = 0x00000007u32,
GC_EA_SE_PERF_SEL_WDRAM_NUM_BANKS_VLD    = 0x00000008u32,
GC_EA_SE_PERF_SEL_WDRAM_REQ_PER_CLIGRP   = 0x00000009u32,
GC_EA_SE_PERF_SEL_WDRAM_CHAINED_REQ_PER_CLIGRP = 0x0000000au32,
GC_EA_SE_PERF_SEL_WDRAM_LATENCY_START0   = 0x0000000bu32,
GC_EA_SE_PERF_SEL_WDRAM_LATENCY_END0     = 0x0000000cu32,
GC_EA_SE_PERF_SEL_WDRAM_LATENCY_START1   = 0x0000000du32,
GC_EA_SE_PERF_SEL_WDRAM_LATENCY_END1     = 0x0000000eu32,
GC_EA_SE_PERF_SEL_RGMI_NUM_BANKS_VLD     = 0x0000000fu32,
GC_EA_SE_PERF_SEL_RGMI_REQ_PER_CLIGRP    = 0x00000010u32,
GC_EA_SE_PERF_SEL_RGMI_CHAINED_REQ_PER_CLIGR = 0x00000011u32,
GC_EA_SE_PERF_SEL_RGMI_LATENCY_START0    = 0x00000012u32,
GC_EA_SE_PERF_SEL_RGMI_LATENCY_END0      = 0x00000013u32,
GC_EA_SE_PERF_SEL_RGMI_LATENCY_START1    = 0x00000014u32,
GC_EA_SE_PERF_SEL_RGMI_LATENCY_END1      = 0x00000015u32,
GC_EA_SE_PERF_SEL_WGMI_NUM_BANKS_VLD     = 0x00000016u32,
GC_EA_SE_PERF_SEL_WGMI_REQ_PER_CLIGRP    = 0x00000017u32,
GC_EA_SE_PERF_SEL_WGMI_CHAINED_REQ_PER_CLIGRP = 0x00000018u32,
GC_EA_SE_PERF_SEL_WGMI_LATENCY_START0    = 0x00000019u32,
GC_EA_SE_PERF_SEL_WGMI_LATENCY_END0      = 0x0000001au32,
GC_EA_SE_PERF_SEL_WGMI_LATENCY_START1    = 0x0000001bu32,
GC_EA_SE_PERF_SEL_WGMI_LATENCY_END1      = 0x0000001cu32,
GC_EA_SE_PERF_SEL_RIO_REQ_PER_CLIGRP     = 0x0000001du32,
GC_EA_SE_PERF_SEL_RIO_SIZE_REQ           = 0x0000001eu32,
GC_EA_SE_PERF_SEL_RIO_GRP0_SIZE_REQ      = 0x0000001fu32,
GC_EA_SE_PERF_SEL_RIO_GRP1_SIZE_REQ      = 0x00000020u32,
GC_EA_SE_PERF_SEL_RIO_GRP2_SIZE_REQ      = 0x00000021u32,
GC_EA_SE_PERF_SEL_RIO_GRP3_SIZE_REQ      = 0x00000022u32,
GC_EA_SE_PERF_SEL_RIO_LATENCY_START0     = 0x00000023u32,
GC_EA_SE_PERF_SEL_RIO_LATENCY_END0       = 0x00000024u32,
GC_EA_SE_PERF_SEL_RIO_LATENCY_START1     = 0x00000025u32,
GC_EA_SE_PERF_SEL_RIO_LATENCY_END1       = 0x00000026u32,
GC_EA_SE_PERF_SEL_WIO_REQ_PER_CLIGRP     = 0x00000027u32,
GC_EA_SE_PERF_SEL_WIO_CHAINED_REQ_PER_CLIGRP = 0x00000028u32,
GC_EA_SE_PERF_SEL_WIO_SIZE_REQ           = 0x00000029u32,
GC_EA_SE_PERF_SEL_WIO_GRP0_SIZE_REQ      = 0x0000002au32,
GC_EA_SE_PERF_SEL_WIO_GRP1_SIZE_REQ      = 0x0000002bu32,
GC_EA_SE_PERF_SEL_WIO_GRP2_SIZE_REQ      = 0x0000002cu32,
GC_EA_SE_PERF_SEL_WIO_GRP3_SIZE_REQ      = 0x0000002du32,
GC_EA_SE_PERF_SEL_WIO_LATENCY_START0     = 0x0000002eu32,
GC_EA_SE_PERF_SEL_WIO_LATENCY_END0       = 0x0000002fu32,
GC_EA_SE_PERF_SEL_WIO_LATENCY_START1     = 0x00000030u32,
GC_EA_SE_PERF_SEL_WIO_LATENCY_END1       = 0x00000031u32,
GC_EA_SE_PERF_SEL_SARB_REQ_PER_VC        = 0x00000032u32,
GC_EA_SE_PERF_SEL_SARB_DRAM_REQ_PER_VC   = 0x00000033u32,
GC_EA_SE_PERF_SEL_SARB_GMI_REQ_PER_VC    = 0x00000034u32,
GC_EA_SE_PERF_SEL_SARB_IO_REQ_PER_VC     = 0x00000035u32,
GC_EA_SE_PERF_SEL_SARB_SIZE_REQ          = 0x00000036u32,
GC_EA_SE_PERF_SEL_SARB_DRAM_SIZE_REQ     = 0x00000037u32,
GC_EA_SE_PERF_SEL_SARB_GMI_SIZE_REQ      = 0x00000038u32,
GC_EA_SE_PERF_SEL_SARB_IO_SIZE_REQ       = 0x00000039u32,
GC_EA_SE_PERF_SEL_SARB_LATENCY_START0    = 0x0000003au32,
GC_EA_SE_PERF_SEL_SARB_LATENCY_END0      = 0x0000003bu32,
GC_EA_SE_PERF_SEL_SARB_LATENCY_START1    = 0x0000003cu32,
GC_EA_SE_PERF_SEL_SARB_LATENCY_END1      = 0x0000003du32,
GC_EA_SE_PERF_SEL_SARB_BUSY              = 0x0000003eu32,
GC_EA_SE_PERF_SEL_SARB_STALLED           = 0x0000003fu32,
GC_EA_SE_PERF_SEL_SARB_STARVING          = 0x00000040u32,
GC_EA_SE_PERF_SEL_SARB_IDLE              = 0x00000041u32,
GC_EA_SE_PERF_SEL_RRET_VLD               = 0x00000042u32,
GC_EA_SE_PERF_SEL_WRET_VLD               = 0x00000043u32,
GC_EA_SE_PERF_SEL_PRB_REQ                = 0x00000044u32,
GC_EA_SE_PERF_SEL_MAM_ARAM_FA_EVICT      = 0x00000045u32,
GC_EA_SE_PERF_SEL_MAM_ARAM_REQ_VLD       = 0x00000046u32,
GC_EA_SE_PERF_SEL_MAM_DBIT_FA_HIT        = 0x00000047u32,
GC_EA_SE_PERF_SEL_MAM_NUM_DQRY           = 0x00000048u32,
GC_EA_SE_PERF_SEL_MAM_AFLUSH_INTERRUPT   = 0x00000049u32,
GC_EA_SE_PERF_SEL_MAM_AFLUSH_INTERRUPT_STALLED = 0x0000004au32,
GC_EA_SE_PERF_SEL_MAM_AFLUSH_COMPLETED   = 0x0000004bu32,
GC_EA_SE_PERF_SEL_MAM_AFLUSH_ONGOING     = 0x0000004cu32,
GC_EA_SE_PERF_SEL_RDRAM_SIZE_REQ         = 0x0000004du32,
GC_EA_SE_PERF_SEL_WDRAM_SIZE_REQ         = 0x0000004eu32,
GC_EA_SE_PERF_SEL_RGMI_SIZE_REQ          = 0x0000004fu32,
GC_EA_SE_PERF_SEL_WGMI_SIZE_REQ          = 0x00000050u32,
GC_EA_SE_PERF_SEL_SARB_DRAM_RW_TURN_AROUND = 0x00000051u32,
GC_EA_SE_PERF_SEL_SARB_GMI_RW_TURN_AROUND = 0x00000052u32,
GC_EA_SE_PERF_SEL_RDRAM_CHAINED_REQ_PER_BURSTS_LENGTH = 0x00000053u32,
GC_EA_SE_PERF_SEL_WDRAM_CHAINED_REQ_PER_BURSTS_LENGTH = 0x00000054u32,
GC_EA_SE_PERF_SEL_RGMI_CHAINED_REQ_PER_BURSTS_LENGTH = 0x00000055u32,
GC_EA_SE_PERF_SEL_WGMI_CHAINED_REQ_PER_BURSTS_LENGTH = 0x00000056u32,
GC_EA_SE_PERF_SEL_MAM_DBIT_FA_EVICT      = 0x00000057u32,
GC_EA_SE_PERF_SEL_MAM_DBIT_REQ_VLD       = 0x00000058u32,
GC_EA_SE_PERF_SEL_SARB_COHERENT_SIZE_REQ = 0x00000059u32,
GC_EA_SE_PERF_SEL_MAM_ARAM_FA_HIT_EVICT  = 0x0000005au32,
GC_EA_SE_PERF_SEL_MAM_ARAM_FA_LRU_EVICT  = 0x0000005bu32,
GC_EA_SE_PERF_SEL_MAM_FLUSH_REQ          = 0x0000005cu32,
GC_EA_SE_PERF_SEL_MAM_FLUSH_RESP         = 0x0000005du32,
GC_EA_SE_PERF_SEL_MAM_DBIT_FA_HIT_EVICT  = 0x0000005eu32,
GC_EA_SE_PERF_SEL_MAM_DBIT_FA_LRU_EVICT  = 0x0000005fu32,
GC_EA_SE_PERF_SEL_MAM_DQRY_ONGOING       = 0x00000060u32,
GC_EA_SE_PERF_SEL_MAM_ARAM_FA_HIT        = 0x00000061u32,
}

/*
 * LSDMA_PERF_SEL enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum LSDMA_PERF_SEL {
LSDMA_PERF_SEL_CYCLE                     = 0x00000000u32,
LSDMA_PERF_SEL_IDLE                      = 0x00000001u32,
LSDMA_PERF_SEL_REG_IDLE                  = 0x00000002u32,
LSDMA_PERF_SEL_RB_EMPTY                  = 0x00000003u32,
LSDMA_PERF_SEL_RB_FULL                   = 0x00000004u32,
LSDMA_PERF_SEL_RB_WPTR_WRAP              = 0x00000005u32,
LSDMA_PERF_SEL_RB_RPTR_WRAP              = 0x00000006u32,
LSDMA_PERF_SEL_RB_WPTR_POLL_READ         = 0x00000007u32,
LSDMA_PERF_SEL_RB_RPTR_WB                = 0x00000008u32,
LSDMA_PERF_SEL_RB_CMD_IDLE               = 0x00000009u32,
LSDMA_PERF_SEL_RB_CMD_FULL               = 0x0000000au32,
LSDMA_PERF_SEL_IB_CMD_IDLE               = 0x0000000bu32,
LSDMA_PERF_SEL_IB_CMD_FULL               = 0x0000000cu32,
LSDMA_PERF_SEL_EX_IDLE                   = 0x0000000du32,
LSDMA_PERF_SEL_SRBM_REG_SEND             = 0x0000000eu32,
LSDMA_PERF_SEL_EX_IDLE_POLL_TIMER_EXPIRE = 0x0000000fu32,
LSDMA_PERF_SEL_MC_WR_IDLE                = 0x00000010u32,
LSDMA_PERF_SEL_MC_WR_COUNT               = 0x00000011u32,
LSDMA_PERF_SEL_MC_RD_IDLE                = 0x00000012u32,
LSDMA_PERF_SEL_MC_RD_COUNT               = 0x00000013u32,
LSDMA_PERF_SEL_MC_RD_RET_STALL           = 0x00000014u32,
LSDMA_PERF_SEL_MC_RD_NO_POLL_IDLE        = 0x00000015u32,
LSDMA_PERF_SEL_SEM_IDLE                  = 0x00000018u32,
LSDMA_PERF_SEL_SEM_REQ_STALL             = 0x00000019u32,
LSDMA_PERF_SEL_SEM_REQ_COUNT             = 0x0000001au32,
LSDMA_PERF_SEL_SEM_RESP_INCOMPLETE       = 0x0000001bu32,
LSDMA_PERF_SEL_SEM_RESP_FAIL             = 0x0000001cu32,
LSDMA_PERF_SEL_SEM_RESP_PASS             = 0x0000001du32,
LSDMA_PERF_SEL_INT_IDLE                  = 0x0000001eu32,
LSDMA_PERF_SEL_INT_REQ_STALL             = 0x0000001fu32,
LSDMA_PERF_SEL_INT_REQ_COUNT             = 0x00000020u32,
LSDMA_PERF_SEL_INT_RESP_ACCEPTED         = 0x00000021u32,
LSDMA_PERF_SEL_INT_RESP_RETRY            = 0x00000022u32,
LSDMA_PERF_SEL_NUM_PACKET                = 0x00000023u32,
LSDMA_PERF_SEL_CE_WREQ_IDLE              = 0x00000025u32,
LSDMA_PERF_SEL_CE_WR_IDLE                = 0x00000026u32,
LSDMA_PERF_SEL_CE_SPLIT_IDLE             = 0x00000027u32,
LSDMA_PERF_SEL_CE_RREQ_IDLE              = 0x00000028u32,
LSDMA_PERF_SEL_CE_OUT_IDLE               = 0x00000029u32,
LSDMA_PERF_SEL_CE_IN_IDLE                = 0x0000002au32,
LSDMA_PERF_SEL_CE_DST_IDLE               = 0x0000002bu32,
LSDMA_PERF_SEL_CE_AFIFO_FULL             = 0x0000002eu32,
LSDMA_PERF_SEL_DUMMY_0                   = 0x0000002fu32,
LSDMA_PERF_SEL_DUMMY_1                   = 0x00000030u32,
LSDMA_PERF_SEL_CE_INFO_FULL              = 0x00000031u32,
LSDMA_PERF_SEL_CE_INFO1_FULL             = 0x00000032u32,
LSDMA_PERF_SEL_CE_RD_STALL               = 0x00000033u32,
LSDMA_PERF_SEL_CE_WR_STALL               = 0x00000034u32,
LSDMA_PERF_SEL_GFX_SELECT                = 0x00000035u32,
LSDMA_PERF_SEL_RLC0_SELECT               = 0x00000036u32,
LSDMA_PERF_SEL_RLC1_SELECT               = 0x00000037u32,
LSDMA_PERF_SEL_PAGE_SELECT               = 0x00000038u32,
LSDMA_PERF_SEL_CTX_CHANGE                = 0x00000039u32,
LSDMA_PERF_SEL_CTX_CHANGE_EXPIRED        = 0x0000003au32,
LSDMA_PERF_SEL_CTX_CHANGE_EXCEPTION      = 0x0000003bu32,
LSDMA_PERF_SEL_DOORBELL                  = 0x0000003cu32,
LSDMA_PERF_SEL_RD_BA_RTR                 = 0x0000003du32,
LSDMA_PERF_SEL_WR_BA_RTR                 = 0x0000003eu32,
LSDMA_PERF_SEL_F32_L1_WR_VLD             = 0x0000003fu32,
LSDMA_PERF_SEL_CE_L1_WR_VLD              = 0x00000040u32,
LSDMA_PERF_SEL_CE_L1_STALL               = 0x00000041u32,
LSDMA_PERF_SEL_SDMA_INVACK_NFLUSH        = 0x00000042u32,
LSDMA_PERF_SEL_SDMA_INVACK_FLUSH         = 0x00000043u32,
LSDMA_PERF_SEL_ATCL2_INVREQ_NFLUSH       = 0x00000044u32,
LSDMA_PERF_SEL_ATCL2_INVREQ_FLUSH        = 0x00000045u32,
LSDMA_PERF_SEL_ATCL2_RET_XNACK           = 0x00000046u32,
LSDMA_PERF_SEL_ATCL2_RET_ACK             = 0x00000047u32,
LSDMA_PERF_SEL_ATCL2_FREE                = 0x00000048u32,
LSDMA_PERF_SEL_SDMA_ATCL2_SEND           = 0x00000049u32,
LSDMA_PERF_SEL_DMA_L1_WR_SEND            = 0x0000004au32,
LSDMA_PERF_SEL_DMA_L1_RD_SEND            = 0x0000004bu32,
LSDMA_PERF_SEL_DMA_MC_WR_SEND            = 0x0000004cu32,
LSDMA_PERF_SEL_DMA_MC_RD_SEND            = 0x0000004du32,
LSDMA_PERF_SEL_L1_WR_FIFO_IDLE           = 0x0000004eu32,
LSDMA_PERF_SEL_L1_RD_FIFO_IDLE           = 0x0000004fu32,
LSDMA_PERF_SEL_L1_WRL2_IDLE              = 0x00000050u32,
LSDMA_PERF_SEL_L1_RDL2_IDLE              = 0x00000051u32,
LSDMA_PERF_SEL_L1_WRMC_IDLE              = 0x00000052u32,
LSDMA_PERF_SEL_L1_RDMC_IDLE              = 0x00000053u32,
LSDMA_PERF_SEL_L1_WR_INV_IDLE            = 0x00000054u32,
LSDMA_PERF_SEL_L1_RD_INV_IDLE            = 0x00000055u32,
LSDMA_PERF_SEL_L1_WR_INV_EN              = 0x00000056u32,
LSDMA_PERF_SEL_L1_RD_INV_EN              = 0x00000057u32,
LSDMA_PERF_SEL_L1_WR_WAIT_INVADR         = 0x00000058u32,
LSDMA_PERF_SEL_L1_RD_WAIT_INVADR         = 0x00000059u32,
LSDMA_PERF_SEL_IS_INVREQ_ADDR_WR         = 0x0000005au32,
LSDMA_PERF_SEL_IS_INVREQ_ADDR_RD         = 0x0000005bu32,
LSDMA_PERF_SEL_L1_WR_XNACK_TIMEOUT       = 0x0000005cu32,
LSDMA_PERF_SEL_L1_RD_XNACK_TIMEOUT       = 0x0000005du32,
LSDMA_PERF_SEL_L1_INV_MIDDLE             = 0x0000005eu32,
LSDMA_PERF_SEL_CE_OR_F32_MMHUB_WR_REQ    = 0x0000005fu32,
LSDMA_PERF_SEL_CE_OR_F32_MMHUB_WR_RET    = 0x00000060u32,
LSDMA_PERF_SEL_ATOMIC_MMHUB_WR_REQ       = 0x00000061u32,
LSDMA_PERF_SEL_ATOMIC_MMHUB_WR_RET       = 0x00000062u32,
LSDMA_PERF_SEL_CE_OR_F32_MMHUB_RD_REQ    = 0x00000063u32,
LSDMA_PERF_SEL_CE_OR_F32_MMHUB_RD_RET    = 0x00000064u32,
LSDMA_PERF_SEL_RB_MMHUB_RD_REQ           = 0x00000065u32,
LSDMA_PERF_SEL_RB_MMHUB_RD_RET           = 0x00000066u32,
LSDMA_PERF_SEL_IB_MMHUB_RD_REQ           = 0x00000067u32,
LSDMA_PERF_SEL_IB_MMHUB_RD_RET           = 0x00000068u32,
LSDMA_PERF_SEL_WPTR_MMHUB_RD_REQ         = 0x00000069u32,
LSDMA_PERF_SEL_WPTR_MMHUB_RD_RET         = 0x0000006au32,
LSDMA_PERF_SEL_UTCL1_UTCL2_REQ           = 0x0000006bu32,
LSDMA_PERF_SEL_UTCL1_UTCL2_RET           = 0x0000006cu32,
LSDMA_PERF_SEL_CMD_OP_MATCH              = 0x0000006du32,
LSDMA_PERF_SEL_CMD_OP_START              = 0x0000006eu32,
LSDMA_PERF_SEL_CMD_OP_END                = 0x0000006fu32,
LSDMA_PERF_SEL_CE_BUSY                   = 0x00000070u32,
LSDMA_PERF_SEL_CE_BUSY_START             = 0x00000071u32,
LSDMA_PERF_SEL_CE_BUSY_END               = 0x00000072u32,
LSDMA_PERF_SEL_F32_PERFCNT_TRIGGER       = 0x00000073u32,
LSDMA_PERF_SEL_F32_PERFCNT_TRIGGER_START = 0x00000074u32,
LSDMA_PERF_SEL_F32_PERFCNT_TRIGGER_END   = 0x00000075u32,
LSDMA_PERF_SEL_CE_MMHUB_WRREQ_SEND       = 0x00000076u32,
LSDMA_PERF_SEL_MMHUB_CE_WRRET_VALID      = 0x00000077u32,
LSDMA_PERF_SEL_CE_MMHUB_RDREQ_SEND       = 0x00000078u32,
LSDMA_PERF_SEL_MMHUB_CE_RDRET_VALID      = 0x00000079u32,
LSDMA_PERF_SEL_DRAM_ECC                  = 0x0000007au32,
LSDMA_PERF_SEL_NACK_GEN_ERR              = 0x0000007bu32,
}

/*
 * ROM_SIGNATURE value
 */

#define ROM_SIGNATURE                  0x0000aa55

/*
 * EFC_SURFACE_PIXEL_FORMAT enum
 */

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum EFC_SURFACE_PIXEL_FORMAT {
EFC_ARGB1555                             = 0x00000001u32,
EFC_RGBA5551                             = 0x00000002u32,
EFC_RGB565                               = 0x00000003u32,
EFC_BGR565                               = 0x00000004u32,
EFC_ARGB4444                             = 0x00000005u32,
EFC_RGBA4444                             = 0x00000006u32,
EFC_ARGB8888                             = 0x00000008u32,
EFC_RGBA8888                             = 0x00000009u32,
EFC_ARGB2101010                          = 0x0000000au32,
EFC_RGBA1010102                          = 0x0000000bu32,
EFC_AYCrCb8888                           = 0x0000000cu32,
EFC_YCrCbA8888                           = 0x0000000du32,
EFC_ACrYCb8888                           = 0x0000000eu32,
EFC_CrYCbA8888                           = 0x0000000fu32,
EFC_ARGB16161616_10MSB                   = 0x00000010u32,
EFC_RGBA16161616_10MSB                   = 0x00000011u32,
EFC_ARGB16161616_10LSB                   = 0x00000012u32,
EFC_RGBA16161616_10LSB                   = 0x00000013u32,
EFC_ARGB16161616_12MSB                   = 0x00000014u32,
EFC_RGBA16161616_12MSB                   = 0x00000015u32,
EFC_ARGB16161616_12LSB                   = 0x00000016u32,
EFC_RGBA16161616_12LSB                   = 0x00000017u32,
EFC_ARGB16161616_FLOAT                   = 0x00000018u32,
EFC_RGBA16161616_FLOAT                   = 0x00000019u32,
EFC_ARGB16161616_UNORM                   = 0x0000001au32,
EFC_RGBA16161616_UNORM                   = 0x0000001bu32,
EFC_ARGB16161616_SNORM                   = 0x0000001cu32,
EFC_RGBA16161616_SNORM                   = 0x0000001du32,
EFC_AYCrCb16161616_10MSB                 = 0x00000020u32,
EFC_AYCrCb16161616_10LSB                 = 0x00000021u32,
EFC_YCrCbA16161616_10MSB                 = 0x00000022u32,
EFC_YCrCbA16161616_10LSB                 = 0x00000023u32,
EFC_ACrYCb16161616_10MSB                 = 0x00000024u32,
EFC_ACrYCb16161616_10LSB                 = 0x00000025u32,
EFC_CrYCbA16161616_10MSB                 = 0x00000026u32,
EFC_CrYCbA16161616_10LSB                 = 0x00000027u32,
EFC_AYCrCb16161616_12MSB                 = 0x00000028u32,
EFC_AYCrCb16161616_12LSB                 = 0x00000029u32,
EFC_YCrCbA16161616_12MSB                 = 0x0000002au32,
EFC_YCrCbA16161616_12LSB                 = 0x0000002bu32,
EFC_ACrYCb16161616_12MSB                 = 0x0000002cu32,
EFC_ACrYCb16161616_12LSB                 = 0x0000002du32,
EFC_CrYCbA16161616_12MSB                 = 0x0000002eu32,
EFC_CrYCbA16161616_12LSB                 = 0x0000002fu32,
EFC_Y8_CrCb88_420_PLANAR                 = 0x00000040u32,
EFC_Y8_CbCr88_420_PLANAR                 = 0x00000041u32,
EFC_Y10_CrCb1010_420_PLANAR              = 0x00000042u32,
EFC_Y10_CbCr1010_420_PLANAR              = 0x00000043u32,
EFC_Y12_CrCb1212_420_PLANAR              = 0x00000044u32,
EFC_Y12_CbCr1212_420_PLANAR              = 0x00000045u32,
EFC_YCrYCb8888_422_PACKED                = 0x00000048u32,
EFC_YCbYCr8888_422_PACKED                = 0x00000049u32,
EFC_CrYCbY8888_422_PACKED                = 0x0000004au32,
EFC_CbYCrY8888_422_PACKED                = 0x0000004bu32,
EFC_YCrYCb10101010_422_PACKED            = 0x0000004cu32,
EFC_YCbYCr10101010_422_PACKED            = 0x0000004du32,
EFC_CrYCbY10101010_422_PACKED            = 0x0000004eu32,
EFC_CbYCrY10101010_422_PACKED            = 0x0000004fu32,
EFC_YCrYCb12121212_422_PACKED            = 0x00000050u32,
EFC_YCbYCr12121212_422_PACKED            = 0x00000051u32,
EFC_CrYCbY12121212_422_PACKED            = 0x00000052u32,
EFC_CbYCrY12121212_422_PACKED            = 0x00000053u32,
EFC_RGB111110_FIX                        = 0x00000070u32,
EFC_BGR101111_FIX                        = 0x00000071u32,
EFC_ACrYCb2101010                        = 0x00000072u32,
EFC_CrYCbA1010102                        = 0x00000073u32,
EFC_RGB111110_FLOAT                      = 0x00000076u32,
EFC_BGR101111_FLOAT                      = 0x00000077u32,
EFC_MONO_8                               = 0x00000078u32,
EFC_MONO_10MSB                           = 0x00000079u32,
EFC_MONO_10LSB                           = 0x0000007au32,
EFC_MONO_12MSB                           = 0x0000007bu32,
EFC_MONO_12LSB                           = 0x0000007cu32,
EFC_MONO_16                              = 0x0000007du32,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
