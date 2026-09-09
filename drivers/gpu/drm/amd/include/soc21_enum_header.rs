/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

#endif

/*******************************************************
 * Chip Enums
 *******************************************************/

/*
 * DSM_DATA_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DSM_DATA_SEL {
DSM_DATA_SEL_DISABLE                     = 0x00000000,
DSM_DATA_SEL_0                           = 0x00000001,
DSM_DATA_SEL_1                           = 0x00000002,
DSM_DATA_SEL_BOTH                        = 0x00000003,
}


/*
 * DSM_ENABLE_ERROR_INJECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DSM_ENABLE_ERROR_INJECT {
DSM_ENABLE_ERROR_INJECT_FED_IN           = 0x00000000,
DSM_ENABLE_ERROR_INJECT_SINGLE           = 0x00000001,
DSM_ENABLE_ERROR_INJECT_UNCORRECTABLE    = 0x00000002,
DSM_ENABLE_ERROR_INJECT_UNCORRECTABLE_LIMITED = 0x00000003,
}


/*
 * DSM_SELECT_INJECT_DELAY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DSM_SELECT_INJECT_DELAY {
DSM_SELECT_INJECT_DELAY_NO_DELAY         = 0x00000000,
DSM_SELECT_INJECT_DELAY_DELAY_ERROR      = 0x00000001,
}


/*
 * DSM_SINGLE_WRITE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DSM_SINGLE_WRITE {
DSM_SINGLE_WRITE_DIS                     = 0x00000000,
DSM_SINGLE_WRITE_EN                      = 0x00000001,
}


/*
 * ENUM_NUM_SIMD_PER_CU enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_NUM_SIMD_PER_CU {
NUM_SIMD_PER_CU                          = 0x00000002,
}


/*
 * GATCL1RequestType enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GATCL1RequestType {
GATCL1_TYPE_NORMAL                       = 0x00000000,
GATCL1_TYPE_SHOOTDOWN                    = 0x00000001,
GATCL1_TYPE_BYPASS                       = 0x00000002,
}


/*
 * GL0V_CACHE_POLICIES enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GL0V_CACHE_POLICIES {
GL0V_CACHE_POLICY_MISS_LRU               = 0x00000000,
GL0V_CACHE_POLICY_MISS_EVICT             = 0x00000001,
GL0V_CACHE_POLICY_HIT_LRU                = 0x00000002,
GL0V_CACHE_POLICY_HIT_EVICT              = 0x00000003,
}


/*
 * GL1_CACHE_POLICIES enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GL1_CACHE_POLICIES {
GL1_CACHE_POLICY_MISS_LRU                = 0x00000000,
GL1_CACHE_POLICY_MISS_EVICT              = 0x00000001,
GL1_CACHE_POLICY_HIT_LRU                 = 0x00000002,
GL1_CACHE_POLICY_HIT_EVICT               = 0x00000003,
}


/*
 * GL1_CACHE_STORE_POLICIES enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GL1_CACHE_STORE_POLICIES {
GL1_CACHE_STORE_POLICY_BYPASS            = 0x00000000,
}


/*
 * GL2_CACHE_POLICIES enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GL2_CACHE_POLICIES {
GL2_CACHE_POLICY_LRU                     = 0x00000000,
GL2_CACHE_POLICY_STREAM                  = 0x00000001,
GL2_CACHE_POLICY_NOA                     = 0x00000002,
GL2_CACHE_POLICY_BYPASS                  = 0x00000003,
}


/*
 * Hdp_SurfaceEndian enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Hdp_SurfaceEndian {
HDP_ENDIAN_NONE                          = 0x00000000,
HDP_ENDIAN_8IN16                         = 0x00000001,
HDP_ENDIAN_8IN32                         = 0x00000002,
HDP_ENDIAN_8IN64                         = 0x00000003,
}


/*
 * MTYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MTYPE {
MTYPE_C_RW_US                            = 0x00000000,
MTYPE_RESERVED_1                         = 0x00000001,
MTYPE_C_RO_S                             = 0x00000002,
MTYPE_UC                                 = 0x00000003,
MTYPE_C_RW_S                             = 0x00000004,
MTYPE_RESERVED_5                         = 0x00000005,
MTYPE_C_RO_US                            = 0x00000006,
MTYPE_RESERVED_7                         = 0x00000007,
}


/*
 * PERFMON_COUNTER_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFMON_COUNTER_MODE {
PERFMON_COUNTER_MODE_ACCUM               = 0x00000000,
PERFMON_COUNTER_MODE_ACTIVE_CYCLES       = 0x00000001,
PERFMON_COUNTER_MODE_MAX                 = 0x00000002,
PERFMON_COUNTER_MODE_DIRTY               = 0x00000003,
PERFMON_COUNTER_MODE_SAMPLE              = 0x00000004,
PERFMON_COUNTER_MODE_CYCLES_SINCE_FIRST_EVENT = 0x00000005,
PERFMON_COUNTER_MODE_CYCLES_SINCE_LAST_EVENT = 0x00000006,
PERFMON_COUNTER_MODE_CYCLES_GE_HI        = 0x00000007,
PERFMON_COUNTER_MODE_CYCLES_EQ_HI        = 0x00000008,
PERFMON_COUNTER_MODE_INACTIVE_CYCLES     = 0x00000009,
PERFMON_COUNTER_MODE_RESERVED            = 0x0000000f,
}


/*
 * PERFMON_SPM_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFMON_SPM_MODE {
PERFMON_SPM_MODE_OFF                     = 0x00000000,
PERFMON_SPM_MODE_16BIT_CLAMP             = 0x00000001,
PERFMON_SPM_MODE_16BIT_NO_CLAMP          = 0x00000002,
PERFMON_SPM_MODE_32BIT_CLAMP             = 0x00000003,
PERFMON_SPM_MODE_32BIT_NO_CLAMP          = 0x00000004,
PERFMON_SPM_MODE_RESERVED_5              = 0x00000005,
PERFMON_SPM_MODE_RESERVED_6              = 0x00000006,
PERFMON_SPM_MODE_RESERVED_7              = 0x00000007,
PERFMON_SPM_MODE_TEST_MODE_0             = 0x00000008,
PERFMON_SPM_MODE_TEST_MODE_1             = 0x00000009,
PERFMON_SPM_MODE_TEST_MODE_2             = 0x0000000a,
}


/*
 * RMI_CID enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RMI_CID {
RMI_CID_CC                               = 0x00000000,
RMI_CID_FC                               = 0x00000001,
RMI_CID_CM                               = 0x00000002,
RMI_CID_DC                               = 0x00000003,
RMI_CID_Z                                = 0x00000004,
RMI_CID_S                                = 0x00000005,
RMI_CID_TILE                             = 0x00000006,
RMI_CID_ZPCPSD                           = 0x00000007,
}


/*
 * ReadPolicy enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ReadPolicy {
CACHE_LRU_RD                             = 0x00000000,
CACHE_STREAM_RD                          = 0x00000001,
CACHE_NOA                                = 0x00000002,
RESERVED_RDPOLICY                        = 0x00000003,
}


/*
 * SDMA_PERFMON_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SDMA_PERFMON_SEL {
SDMA_PERFMON_SEL_CYCLE                   = 0x00000000,
SDMA_PERFMON_SEL_IDLE                    = 0x00000001,
SDMA_PERFMON_SEL_REG_IDLE                = 0x00000002,
SDMA_PERFMON_SEL_RB_EMPTY                = 0x00000003,
SDMA_PERFMON_SEL_RB_FULL                 = 0x00000004,
SDMA_PERFMON_SEL_RB_WPTR_WRAP            = 0x00000005,
SDMA_PERFMON_SEL_RB_RPTR_WRAP            = 0x00000006,
SDMA_PERFMON_SEL_RB_WPTR_POLL_READ       = 0x00000007,
SDMA_PERFMON_SEL_RB_RPTR_WB              = 0x00000008,
SDMA_PERFMON_SEL_RB_CMD_IDLE             = 0x00000009,
SDMA_PERFMON_SEL_RB_CMD_FULL             = 0x0000000a,
SDMA_PERFMON_SEL_IB_CMD_IDLE             = 0x0000000b,
SDMA_PERFMON_SEL_IB_CMD_FULL             = 0x0000000c,
SDMA_PERFMON_SEL_EX_IDLE                 = 0x0000000d,
SDMA_PERFMON_SEL_SRBM_REG_SEND           = 0x0000000e,
SDMA_PERFMON_SEL_EX_IDLE_POLL_TIMER_EXPIRE = 0x0000000f,
SDMA_PERFMON_SEL_WR_BA_RTR               = 0x00000010,
SDMA_PERFMON_SEL_MC_WR_IDLE              = 0x00000011,
SDMA_PERFMON_SEL_MC_WR_COUNT             = 0x00000012,
SDMA_PERFMON_SEL_RD_BA_RTR               = 0x00000013,
SDMA_PERFMON_SEL_MC_RD_IDLE              = 0x00000014,
SDMA_PERFMON_SEL_MC_RD_COUNT             = 0x00000015,
SDMA_PERFMON_SEL_MC_RD_RET_STALL         = 0x00000016,
SDMA_PERFMON_SEL_MC_RD_NO_POLL_IDLE      = 0x00000017,
SDMA_PERFMON_SEL_SEM_IDLE                = 0x0000001a,
SDMA_PERFMON_SEL_SEM_REQ_STALL           = 0x0000001b,
SDMA_PERFMON_SEL_SEM_REQ_COUNT           = 0x0000001c,
SDMA_PERFMON_SEL_SEM_RESP_INCOMPLETE     = 0x0000001d,
SDMA_PERFMON_SEL_SEM_RESP_FAIL           = 0x0000001e,
SDMA_PERFMON_SEL_SEM_RESP_PASS           = 0x0000001f,
SDMA_PERFMON_SEL_INT_IDLE                = 0x00000020,
SDMA_PERFMON_SEL_INT_REQ_STALL           = 0x00000021,
SDMA_PERFMON_SEL_INT_REQ_COUNT           = 0x00000022,
SDMA_PERFMON_SEL_INT_RESP_ACCEPTED       = 0x00000023,
SDMA_PERFMON_SEL_INT_RESP_RETRY          = 0x00000024,
SDMA_PERFMON_SEL_NUM_PACKET              = 0x00000025,
SDMA_PERFMON_SEL_CE_WREQ_IDLE            = 0x00000027,
SDMA_PERFMON_SEL_CE_WR_IDLE              = 0x00000028,
SDMA_PERFMON_SEL_CE_SPLIT_IDLE           = 0x00000029,
SDMA_PERFMON_SEL_CE_RREQ_IDLE            = 0x0000002a,
SDMA_PERFMON_SEL_CE_OUT_IDLE             = 0x0000002b,
SDMA_PERFMON_SEL_CE_IN_IDLE              = 0x0000002c,
SDMA_PERFMON_SEL_CE_DST_IDLE             = 0x0000002d,
SDMA_PERFMON_SEL_CE_AFIFO_FULL           = 0x00000030,
SDMA_PERFMON_SEL_CE_INFO_FULL            = 0x00000033,
SDMA_PERFMON_SEL_CE_INFO1_FULL           = 0x00000034,
SDMA_PERFMON_SEL_CE_RD_STALL             = 0x00000035,
SDMA_PERFMON_SEL_CE_WR_STALL             = 0x00000036,
SDMA_PERFMON_SEL_GFX_SELECT              = 0x00000037,
SDMA_PERFMON_SEL_RLC0_SELECT             = 0x00000038,
SDMA_PERFMON_SEL_RLC1_SELECT             = 0x00000039,
SDMA_PERFMON_SEL_PAGE_SELECT             = 0x0000003a,
SDMA_PERFMON_SEL_CTX_CHANGE              = 0x0000003b,
SDMA_PERFMON_SEL_CTX_CHANGE_EXPIRED      = 0x0000003c,
SDMA_PERFMON_SEL_CTX_CHANGE_EXCEPTION    = 0x0000003d,
SDMA_PERFMON_SEL_DOORBELL                = 0x0000003e,
SDMA_PERFMON_SEL_F32_L1_WR_VLD           = 0x0000003f,
SDMA_PERFMON_SEL_CE_L1_WR_VLD            = 0x00000040,
SDMA_PERFMON_SEL_CPF_SDMA_INVREQ         = 0x00000041,
SDMA_PERFMON_SEL_SDMA_CPF_INVACK         = 0x00000042,
SDMA_PERFMON_SEL_UTCL2_SDMA_INVREQ       = 0x00000043,
SDMA_PERFMON_SEL_SDMA_UTCL2_INVACK       = 0x00000044,
SDMA_PERFMON_SEL_UTCL2_SDMA_INVREQ_ALL   = 0x00000045,
SDMA_PERFMON_SEL_SDMA_UTCL2_INVACK_ALL   = 0x00000046,
SDMA_PERFMON_SEL_UTCL2_RET_XNACK         = 0x00000047,
SDMA_PERFMON_SEL_UTCL2_RET_ACK           = 0x00000048,
SDMA_PERFMON_SEL_UTCL2_FREE              = 0x00000049,
SDMA_PERFMON_SEL_SDMA_UTCL2_SEND         = 0x0000004a,
SDMA_PERFMON_SEL_DMA_L1_WR_SEND          = 0x0000004b,
SDMA_PERFMON_SEL_DMA_L1_RD_SEND          = 0x0000004c,
SDMA_PERFMON_SEL_DMA_MC_WR_SEND          = 0x0000004d,
SDMA_PERFMON_SEL_DMA_MC_RD_SEND          = 0x0000004e,
SDMA_PERFMON_SEL_GPUVM_INV_HIGH          = 0x0000004f,
SDMA_PERFMON_SEL_GPUVM_INV_LOW           = 0x00000050,
SDMA_PERFMON_SEL_L1_WRL2_IDLE            = 0x00000051,
SDMA_PERFMON_SEL_L1_RDL2_IDLE            = 0x00000052,
SDMA_PERFMON_SEL_L1_WRMC_IDLE            = 0x00000053,
SDMA_PERFMON_SEL_L1_RDMC_IDLE            = 0x00000054,
SDMA_PERFMON_SEL_L1_WR_INV_IDLE          = 0x00000055,
SDMA_PERFMON_SEL_L1_RD_INV_IDLE          = 0x00000056,
SDMA_PERFMON_SEL_META_L2_REQ_SEND        = 0x00000057,
SDMA_PERFMON_SEL_L2_META_RET_VLD         = 0x00000058,
SDMA_PERFMON_SEL_SDMA_UTCL2_RD_SEND      = 0x00000059,
SDMA_PERFMON_SEL_UTCL2_SDMA_RD_RTN       = 0x0000005a,
SDMA_PERFMON_SEL_SDMA_UTCL2_WR_SEND      = 0x0000005b,
SDMA_PERFMON_SEL_UTCL2_SDMA_WR_RTN       = 0x0000005c,
SDMA_PERFMON_SEL_META_REQ_SEND           = 0x0000005d,
SDMA_PERFMON_SEL_META_RTN_VLD            = 0x0000005e,
SDMA_PERFMON_SEL_TLBI_SEND               = 0x0000005f,
SDMA_PERFMON_SEL_TLBI_RTN                = 0x00000060,
SDMA_PERFMON_SEL_GCR_SEND                = 0x00000061,
SDMA_PERFMON_SEL_GCR_RTN                 = 0x00000062,
SDMA_PERFMON_SEL_UTCL1_TAG_DELAY_COUNTER = 0x00000063,
SDMA_PERFMON_SEL_MMHUB_TAG_DELAY_COUNTER = 0x00000064,
}


/*
 * SDMA_PERF_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SDMA_PERF_SEL {
SDMA_PERF_SEL_CYCLE                      = 0x00000000,
SDMA_PERF_SEL_IDLE                       = 0x00000001,
SDMA_PERF_SEL_REG_IDLE                   = 0x00000002,
SDMA_PERF_SEL_RB_EMPTY                   = 0x00000003,
SDMA_PERF_SEL_RB_FULL                    = 0x00000004,
SDMA_PERF_SEL_RB_WPTR_WRAP               = 0x00000005,
SDMA_PERF_SEL_RB_RPTR_WRAP               = 0x00000006,
SDMA_PERF_SEL_RB_WPTR_POLL_READ          = 0x00000007,
SDMA_PERF_SEL_RB_RPTR_WB                 = 0x00000008,
SDMA_PERF_SEL_RB_CMD_IDLE                = 0x00000009,
SDMA_PERF_SEL_RB_CMD_FULL                = 0x0000000a,
SDMA_PERF_SEL_IB_CMD_IDLE                = 0x0000000b,
SDMA_PERF_SEL_IB_CMD_FULL                = 0x0000000c,
SDMA_PERF_SEL_EX_IDLE                    = 0x0000000d,
SDMA_PERF_SEL_SRBM_REG_SEND              = 0x0000000e,
SDMA_PERF_SEL_EX_IDLE_POLL_TIMER_EXPIRE  = 0x0000000f,
SDMA_PERF_SEL_MC_WR_IDLE                 = 0x00000010,
SDMA_PERF_SEL_MC_WR_COUNT                = 0x00000011,
SDMA_PERF_SEL_MC_RD_IDLE                 = 0x00000012,
SDMA_PERF_SEL_MC_RD_COUNT                = 0x00000013,
SDMA_PERF_SEL_MC_RD_RET_STALL            = 0x00000014,
SDMA_PERF_SEL_MC_RD_NO_POLL_IDLE         = 0x00000015,
SDMA_PERF_SEL_SEM_IDLE                   = 0x00000018,
SDMA_PERF_SEL_SEM_REQ_STALL              = 0x00000019,
SDMA_PERF_SEL_SEM_REQ_COUNT              = 0x0000001a,
SDMA_PERF_SEL_SEM_RESP_INCOMPLETE        = 0x0000001b,
SDMA_PERF_SEL_SEM_RESP_FAIL              = 0x0000001c,
SDMA_PERF_SEL_SEM_RESP_PASS              = 0x0000001d,
SDMA_PERF_SEL_INT_IDLE                   = 0x0000001e,
SDMA_PERF_SEL_INT_REQ_STALL              = 0x0000001f,
SDMA_PERF_SEL_INT_REQ_COUNT              = 0x00000020,
SDMA_PERF_SEL_INT_RESP_ACCEPTED          = 0x00000021,
SDMA_PERF_SEL_INT_RESP_RETRY             = 0x00000022,
SDMA_PERF_SEL_NUM_PACKET                 = 0x00000023,
SDMA_PERF_SEL_CE_WREQ_IDLE               = 0x00000025,
SDMA_PERF_SEL_CE_WR_IDLE                 = 0x00000026,
SDMA_PERF_SEL_CE_SPLIT_IDLE              = 0x00000027,
SDMA_PERF_SEL_CE_RREQ_IDLE               = 0x00000028,
SDMA_PERF_SEL_CE_OUT_IDLE                = 0x00000029,
SDMA_PERF_SEL_CE_IN_IDLE                 = 0x0000002a,
SDMA_PERF_SEL_CE_DST_IDLE                = 0x0000002b,
SDMA_PERF_SEL_CE_AFIFO_FULL              = 0x0000002e,
SDMA_PERF_SEL_CE_INFO_FULL               = 0x00000031,
SDMA_PERF_SEL_CE_INFO1_FULL              = 0x00000032,
SDMA_PERF_SEL_CE_RD_STALL                = 0x00000033,
SDMA_PERF_SEL_CE_WR_STALL                = 0x00000034,
SDMA_PERF_SEL_GFX_SELECT                 = 0x00000035,
SDMA_PERF_SEL_RLC0_SELECT                = 0x00000036,
SDMA_PERF_SEL_RLC1_SELECT                = 0x00000037,
SDMA_PERF_SEL_PAGE_SELECT                = 0x00000038,
SDMA_PERF_SEL_CTX_CHANGE                 = 0x00000039,
SDMA_PERF_SEL_CTX_CHANGE_EXPIRED         = 0x0000003a,
SDMA_PERF_SEL_CTX_CHANGE_EXCEPTION       = 0x0000003b,
SDMA_PERF_SEL_DOORBELL                   = 0x0000003c,
SDMA_PERF_SEL_RD_BA_RTR                  = 0x0000003d,
SDMA_PERF_SEL_WR_BA_RTR                  = 0x0000003e,
SDMA_PERF_SEL_F32_L1_WR_VLD              = 0x0000003f,
SDMA_PERF_SEL_CE_L1_WR_VLD               = 0x00000040,
SDMA_PERF_SEL_CPF_SDMA_INVREQ            = 0x00000041,
SDMA_PERF_SEL_SDMA_CPF_INVACK            = 0x00000042,
SDMA_PERF_SEL_UTCL2_SDMA_INVREQ          = 0x00000043,
SDMA_PERF_SEL_SDMA_UTCL2_INVACK          = 0x00000044,
SDMA_PERF_SEL_UTCL2_SDMA_INVREQ_ALL      = 0x00000045,
SDMA_PERF_SEL_SDMA_UTCL2_INVACK_ALL      = 0x00000046,
SDMA_PERF_SEL_UTCL2_RET_XNACK            = 0x00000047,
SDMA_PERF_SEL_UTCL2_RET_ACK              = 0x00000048,
SDMA_PERF_SEL_UTCL2_FREE                 = 0x00000049,
SDMA_PERF_SEL_SDMA_UTCL2_SEND            = 0x0000004a,
SDMA_PERF_SEL_DMA_L1_WR_SEND             = 0x0000004b,
SDMA_PERF_SEL_DMA_L1_RD_SEND             = 0x0000004c,
SDMA_PERF_SEL_DMA_MC_WR_SEND             = 0x0000004d,
SDMA_PERF_SEL_DMA_MC_RD_SEND             = 0x0000004e,
SDMA_PERF_SEL_GPUVM_INV_HIGH             = 0x0000004f,
SDMA_PERF_SEL_GPUVM_INV_LOW              = 0x00000050,
SDMA_PERF_SEL_L1_WRL2_IDLE               = 0x00000051,
SDMA_PERF_SEL_L1_RDL2_IDLE               = 0x00000052,
SDMA_PERF_SEL_L1_WRMC_IDLE               = 0x00000053,
SDMA_PERF_SEL_L1_RDMC_IDLE               = 0x00000054,
SDMA_PERF_SEL_L1_WR_INV_IDLE             = 0x00000055,
SDMA_PERF_SEL_L1_RD_INV_IDLE             = 0x00000056,
SDMA_PERF_SEL_META_L2_REQ_SEND           = 0x00000057,
SDMA_PERF_SEL_L2_META_RET_VLD            = 0x00000058,
SDMA_PERF_SEL_SDMA_UTCL2_RD_SEND         = 0x00000059,
SDMA_PERF_SEL_UTCL2_SDMA_RD_RTN          = 0x0000005a,
SDMA_PERF_SEL_SDMA_UTCL2_WR_SEND         = 0x0000005b,
SDMA_PERF_SEL_UTCL2_SDMA_WR_RTN          = 0x0000005c,
SDMA_PERF_SEL_META_REQ_SEND              = 0x0000005d,
SDMA_PERF_SEL_META_RTN_VLD               = 0x0000005e,
SDMA_PERF_SEL_TLBI_SEND                  = 0x0000005f,
SDMA_PERF_SEL_TLBI_RTN                   = 0x00000060,
SDMA_PERF_SEL_GCR_SEND                   = 0x00000061,
SDMA_PERF_SEL_GCR_RTN                    = 0x00000062,
SDMA_PERF_SEL_CGCG_FENCE                 = 0x00000063,
SDMA_PERF_SEL_CE_CH_WR_REQ               = 0x00000064,
SDMA_PERF_SEL_CE_CH_WR_RET               = 0x00000065,
SDMA_PERF_SEL_F32_CH_WR_REQ              = 0x00000066,
SDMA_PERF_SEL_F32_CH_WR_RET              = 0x00000067,
SDMA_PERF_SEL_CE_OR_F32_CH_RD_REQ        = 0x00000068,
SDMA_PERF_SEL_CE_OR_F32_CH_RD_RET        = 0x00000069,
SDMA_PERF_SEL_RB_CH_RD_REQ               = 0x0000006a,
SDMA_PERF_SEL_RB_CH_RD_RET               = 0x0000006b,
SDMA_PERF_SEL_IB_CH_RD_REQ               = 0x0000006c,
SDMA_PERF_SEL_IB_CH_RD_RET               = 0x0000006d,
SDMA_PERF_SEL_WPTR_CH_RD_REQ             = 0x0000006e,
SDMA_PERF_SEL_WPTR_CH_RD_RET             = 0x0000006f,
SDMA_PERF_SEL_UTCL1_UTCL2_REQ            = 0x00000070,
SDMA_PERF_SEL_UTCL1_UTCL2_RET            = 0x00000071,
SDMA_PERF_SEL_CMD_OP_MATCH               = 0x00000072,
SDMA_PERF_SEL_CMD_OP_START               = 0x00000073,
SDMA_PERF_SEL_CMD_OP_END                 = 0x00000074,
SDMA_PERF_SEL_CE_BUSY                    = 0x00000075,
SDMA_PERF_SEL_CE_BUSY_START              = 0x00000076,
SDMA_PERF_SEL_CE_BUSY_END                = 0x00000077,
SDMA_PERF_SEL_F32_PERFCNT_TRIGGER        = 0x00000078,
SDMA_PERF_SEL_F32_PERFCNT_TRIGGER_START  = 0x00000079,
SDMA_PERF_SEL_F32_PERFCNT_TRIGGER_END    = 0x0000007a,
SDMA_PERF_SEL_CE_CH_WRREQ_SEND           = 0x0000007b,
SDMA_PERF_SEL_CH_CE_WRRET_VALID          = 0x0000007c,
SDMA_PERF_SEL_CE_CH_RDREQ_SEND           = 0x0000007d,
SDMA_PERF_SEL_CH_CE_RDRET_VALID          = 0x0000007e,
}


/*
 * TCC_CACHE_POLICIES enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TCC_CACHE_POLICIES {
TCC_CACHE_POLICY_LRU                     = 0x00000000,
TCC_CACHE_POLICY_STREAM                  = 0x00000001,
}


/*
 * TCC_MTYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TCC_MTYPE {
MTYPE_NC                                 = 0x00000000,
MTYPE_WC                                 = 0x00000001,
MTYPE_CC                                 = 0x00000002,
}


/*
 * UTCL0FaultType enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UTCL0FaultType {
UTCL0_XNACK_SUCCESS                      = 0x00000000,
UTCL0_XNACK_RETRY                        = 0x00000001,
UTCL0_XNACK_PRT                          = 0x00000002,
UTCL0_XNACK_NO_RETRY                     = 0x00000003,
}


/*
 * UTCL0RequestType enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UTCL0RequestType {
UTCL0_TYPE_NORMAL                        = 0x00000000,
UTCL0_TYPE_SHOOTDOWN                     = 0x00000001,
UTCL0_TYPE_BYPASS                        = 0x00000002,
}


/*
 * UTCL1FaultType enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UTCL1FaultType {
UTCL1_XNACK_SUCCESS                      = 0x00000000,
UTCL1_XNACK_RETRY                        = 0x00000001,
UTCL1_XNACK_PRT                          = 0x00000002,
UTCL1_XNACK_NO_RETRY                     = 0x00000003,
}


/*
 * UTCL1RequestType enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UTCL1RequestType {
UTCL1_TYPE_NORMAL                        = 0x00000000,
UTCL1_TYPE_SHOOTDOWN                     = 0x00000001,
UTCL1_TYPE_BYPASS                        = 0x00000002,
}


/*
 * VMEMCMD_RETURN_ORDER enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VMEMCMD_RETURN_ORDER {
VMEMCMD_RETURN_OUT_OF_ORDER              = 0x00000000,
VMEMCMD_RETURN_IN_ORDER                  = 0x00000001,
VMEMCMD_RETURN_IN_ORDER_READ             = 0x00000002,
}


/*
 * WritePolicy enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WritePolicy {
CACHE_LRU_WR                             = 0x00000000,
CACHE_STREAM                             = 0x00000001,
CACHE_NOA_WR                             = 0x00000002,
CACHE_BYPASS                             = 0x00000003,
}


/*******************************************************
 * CNVC_CFG Enums
 *******************************************************/

/*
 * CNVC_BYPASS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CNVC_BYPASS {
CNVC_BYPASS_DISABLE                      = 0x00000000,
CNVC_BYPASS_EN                           = 0x00000001,
}


/*
 * CNVC_COEF_FORMAT_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CNVC_COEF_FORMAT_ENUM {
CNVC_FIX_S2_13                           = 0x00000000,
CNVC_FIX_S3_12                           = 0x00000001,
}


/*
 * CNVC_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CNVC_ENABLE {
CNVC_DIS                                 = 0x00000000,
CNVC_EN                                  = 0x00000001,
}


/*
 * CNVC_PENDING enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CNVC_PENDING {
CNVC_NOT_PENDING                         = 0x00000000,
CNVC_YES_PENDING                         = 0x00000001,
}


/*
 * COLOR_KEYER_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum COLOR_KEYER_MODE {
FORCE_00                                 = 0x00000000,
FORCE_FF                                 = 0x00000001,
RANGE_00                                 = 0x00000002,
RANGE_FF                                 = 0x00000003,
}


/*
 * DENORM_TRUNCATE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DENORM_TRUNCATE {
CNVC_ROUND                               = 0x00000000,
CNVC_TRUNCATE                            = 0x00000001,
}


/*
 * FORMAT_CROSSBAR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FORMAT_CROSSBAR {
FORMAT_CROSSBAR_R                        = 0x00000000,
FORMAT_CROSSBAR_G                        = 0x00000001,
FORMAT_CROSSBAR_B                        = 0x00000002,
}


/*
 * PIX_EXPAND_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PIX_EXPAND_MODE {
PIX_DYNAMIC_EXPANSION                    = 0x00000000,
PIX_ZERO_EXPANSION                       = 0x00000001,
}


/*
 * PRE_CSC_MODE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PRE_CSC_MODE_ENUM {
PRE_CSC_BYPASS                           = 0x00000000,
PRE_CSC_SET_A                            = 0x00000001,
PRE_CSC_SET_B                            = 0x00000002,
}


/*
 * PRE_DEGAM_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PRE_DEGAM_MODE {
PRE_DEGAM_BYPASS                         = 0x00000000,
PRE_DEGAM_ENABLE                         = 0x00000001,
}


/*
 * PRE_DEGAM_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PRE_DEGAM_SELECT {
PRE_DEGAM_SRGB                           = 0x00000000,
PRE_DEGAM_GAMMA_22                       = 0x00000001,
PRE_DEGAM_GAMMA_24                       = 0x00000002,
PRE_DEGAM_GAMMA_26                       = 0x00000003,
PRE_DEGAM_BT2020                         = 0x00000004,
PRE_DEGAM_BT2100PQ                       = 0x00000005,
PRE_DEGAM_BT2100HLG                      = 0x00000006,
}


/*
 * SURFACE_PIXEL_FORMAT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURFACE_PIXEL_FORMAT {
ARGB1555                                 = 0x00000001,
RGBA5551                                 = 0x00000002,
RGB565                                   = 0x00000003,
BGR565                                   = 0x00000004,
ARGB4444                                 = 0x00000005,
RGBA4444                                 = 0x00000006,
ARGB8888                                 = 0x00000008,
RGBA8888                                 = 0x00000009,
ARGB2101010                              = 0x0000000a,
RGBA1010102                              = 0x0000000b,
AYCrCb8888                               = 0x0000000c,
YCrCbA8888                               = 0x0000000d,
ACrYCb8888                               = 0x0000000e,
CrYCbA8888                               = 0x0000000f,
ARGB16161616_10MSB                       = 0x00000010,
RGBA16161616_10MSB                       = 0x00000011,
ARGB16161616_10LSB                       = 0x00000012,
RGBA16161616_10LSB                       = 0x00000013,
ARGB16161616_12MSB                       = 0x00000014,
RGBA16161616_12MSB                       = 0x00000015,
ARGB16161616_12LSB                       = 0x00000016,
RGBA16161616_12LSB                       = 0x00000017,
ARGB16161616_FLOAT                       = 0x00000018,
RGBA16161616_FLOAT                       = 0x00000019,
ARGB16161616_UNORM                       = 0x0000001a,
RGBA16161616_UNORM                       = 0x0000001b,
ARGB16161616_SNORM                       = 0x0000001c,
RGBA16161616_SNORM                       = 0x0000001d,
AYCrCb16161616_10MSB                     = 0x00000020,
AYCrCb16161616_10LSB                     = 0x00000021,
YCrCbA16161616_10MSB                     = 0x00000022,
YCrCbA16161616_10LSB                     = 0x00000023,
ACrYCb16161616_10MSB                     = 0x00000024,
ACrYCb16161616_10LSB                     = 0x00000025,
CrYCbA16161616_10MSB                     = 0x00000026,
CrYCbA16161616_10LSB                     = 0x00000027,
AYCrCb16161616_12MSB                     = 0x00000028,
AYCrCb16161616_12LSB                     = 0x00000029,
YCrCbA16161616_12MSB                     = 0x0000002a,
YCrCbA16161616_12LSB                     = 0x0000002b,
ACrYCb16161616_12MSB                     = 0x0000002c,
ACrYCb16161616_12LSB                     = 0x0000002d,
CrYCbA16161616_12MSB                     = 0x0000002e,
CrYCbA16161616_12LSB                     = 0x0000002f,
Y8_CrCb88_420_PLANAR                     = 0x00000040,
Y8_CbCr88_420_PLANAR                     = 0x00000041,
Y10_CrCb1010_420_PLANAR                  = 0x00000042,
Y10_CbCr1010_420_PLANAR                  = 0x00000043,
Y12_CrCb1212_420_PLANAR                  = 0x00000044,
Y12_CbCr1212_420_PLANAR                  = 0x00000045,
YCrYCb8888_422_PACKED                    = 0x00000048,
YCbYCr8888_422_PACKED                    = 0x00000049,
CrYCbY8888_422_PACKED                    = 0x0000004a,
CbYCrY8888_422_PACKED                    = 0x0000004b,
YCrYCb10101010_422_PACKED                = 0x0000004c,
YCbYCr10101010_422_PACKED                = 0x0000004d,
CrYCbY10101010_422_PACKED                = 0x0000004e,
CbYCrY10101010_422_PACKED                = 0x0000004f,
YCrYCb12121212_422_PACKED                = 0x00000050,
YCbYCr12121212_422_PACKED                = 0x00000051,
CrYCbY12121212_422_PACKED                = 0x00000052,
CbYCrY12121212_422_PACKED                = 0x00000053,
RGB111110_FIX                            = 0x00000070,
BGR101111_FIX                            = 0x00000071,
ACrYCb2101010                            = 0x00000072,
CrYCbA1010102                            = 0x00000073,
RGBE                                     = 0x00000074,
RGB111110_FLOAT                          = 0x00000076,
BGR101111_FLOAT                          = 0x00000077,
MONO_8                                   = 0x00000078,
MONO_10MSB                               = 0x00000079,
MONO_10LSB                               = 0x0000007a,
MONO_12MSB                               = 0x0000007b,
MONO_12LSB                               = 0x0000007c,
MONO_16                                  = 0x0000007d,
}


/*
 * XNORM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum XNORM {
XNORM_A                                  = 0x00000000,
XNORM_B                                  = 0x00000001,
}


/*******************************************************
 * CNVC_CUR Enums
 *******************************************************/

/*
 * CUR_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CUR_ENABLE {
CUR_DIS                                  = 0x00000000,
CUR_EN                                   = 0x00000001,
}


/*
 * CUR_EXPAND_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CUR_EXPAND_MODE {
CUR_DYNAMIC_EXPANSION                    = 0x00000000,
CUR_ZERO_EXPANSION                       = 0x00000001,
}


/*
 * CUR_INV_CLAMP enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CUR_INV_CLAMP {
CUR_CLAMP_DIS                            = 0x00000000,
CUR_CLAMP_EN                             = 0x00000001,
}


/*
 * CUR_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CUR_MODE {
MONO_2BIT                                = 0x00000000,
COLOR_24BIT_1BIT_AND                     = 0x00000001,
COLOR_24BIT_8BIT_ALPHA_PREMULT           = 0x00000002,
COLOR_24BIT_8BIT_ALPHA_UNPREMULT         = 0x00000003,
COLOR_64BIT_FP_PREMULT                   = 0x00000004,
COLOR_64BIT_FP_UNPREMULT                 = 0x00000005,
}


/*
 * CUR_PENDING enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CUR_PENDING {
CUR_NOT_PENDING                          = 0x00000000,
CUR_YES_PENDING                          = 0x00000001,
}


/*
 * CUR_ROM_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CUR_ROM_EN {
CUR_FP_NO_ROM                            = 0x00000000,
CUR_FP_USE_ROM                           = 0x00000001,
}


/*******************************************************
 * DSCL Enums
 *******************************************************/

/*
 * COEF_RAM_SELECT_RD enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum COEF_RAM_SELECT_RD {
COEF_RAM_SELECT_BACK                     = 0x00000000,
COEF_RAM_SELECT_CURRENT                  = 0x00000001,
}


/*
 * DSCL_MODE_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DSCL_MODE_SEL {
DSCL_MODE_SCALING_444_BYPASS             = 0x00000000,
DSCL_MODE_SCALING_444_RGB_ENABLE         = 0x00000001,
DSCL_MODE_SCALING_444_YCBCR_ENABLE       = 0x00000002,
DSCL_MODE_SCALING_YCBCR_ENABLE           = 0x00000003,
DSCL_MODE_LUMA_SCALING_BYPASS            = 0x00000004,
DSCL_MODE_CHROMA_SCALING_BYPASS          = 0x00000005,
DSCL_MODE_DSCL_BYPASS                    = 0x00000006,
}


/*
 * LB_ALPHA_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LB_ALPHA_EN {
LB_ALPHA_DISABLE                         = 0x00000000,
LB_ALPHA_ENABLE                          = 0x00000001,
}


/*
 * LB_INTERLEAVE_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LB_INTERLEAVE_EN {
LB_INTERLEAVE_DISABLE                    = 0x00000000,
LB_INTERLEAVE_ENABLE                     = 0x00000001,
}


/*
 * LB_MEMORY_CONFIG enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LB_MEMORY_CONFIG {
LB_MEMORY_CONFIG_0                       = 0x00000000,
LB_MEMORY_CONFIG_1                       = 0x00000001,
LB_MEMORY_CONFIG_2                       = 0x00000002,
LB_MEMORY_CONFIG_3                       = 0x00000003,
}


/*
 * OBUF_BYPASS_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OBUF_BYPASS_SEL {
OBUF_BYPASS_DIS                          = 0x00000000,
OBUF_BYPASS_EN                           = 0x00000001,
}


/*
 * OBUF_IS_HALF_RECOUT_WIDTH_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OBUF_IS_HALF_RECOUT_WIDTH_SEL {
OBUF_FULL_RECOUT                         = 0x00000000,
OBUF_HALF_RECOUT                         = 0x00000001,
}


/*
 * OBUF_USE_FULL_BUFFER_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OBUF_USE_FULL_BUFFER_SEL {
OBUF_RECOUT                              = 0x00000000,
OBUF_FULL                                = 0x00000001,
}


/*
 * SCL_2TAP_HARDCODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SCL_2TAP_HARDCODE {
SCL_COEF_2TAP_HARDCODE_OFF               = 0x00000000,
SCL_COEF_2TAP_HARDCODE_ON                = 0x00000001,
}


/*
 * SCL_ALPHA_COEF enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SCL_ALPHA_COEF {
SCL_ALPHA_COEF_FIRST                     = 0x00000000,
SCL_ALPHA_COEF_SECOND                    = 0x00000001,
}


/*
 * SCL_AUTOCAL_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SCL_AUTOCAL_MODE {
AUTOCAL_MODE_OFF                         = 0x00000000,
AUTOCAL_MODE_AUTOSCALE                   = 0x00000001,
AUTOCAL_MODE_AUTOCENTER                  = 0x00000002,
AUTOCAL_MODE_AUTOREPLICATE               = 0x00000003,
}


/*
 * SCL_BOUNDARY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SCL_BOUNDARY {
SCL_BOUNDARY_EDGE                        = 0x00000000,
SCL_BOUNDARY_BLACK                       = 0x00000001,
}


/*
 * SCL_CHROMA_COEF enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SCL_CHROMA_COEF {
SCL_CHROMA_COEF_FIRST                    = 0x00000000,
SCL_CHROMA_COEF_SECOND                   = 0x00000001,
}


/*
 * SCL_COEF_FILTER_TYPE_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SCL_COEF_FILTER_TYPE_SEL {
SCL_COEF_LUMA_VERT_FILTER                = 0x00000000,
SCL_COEF_LUMA_HORZ_FILTER                = 0x00000001,
SCL_COEF_CHROMA_VERT_FILTER              = 0x00000002,
SCL_COEF_CHROMA_HORZ_FILTER              = 0x00000003,
}


/*
 * SCL_COEF_RAM_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SCL_COEF_RAM_SEL {
SCL_COEF_RAM_SEL_0                       = 0x00000000,
SCL_COEF_RAM_SEL_1                       = 0x00000001,
}


/*
 * SCL_SHARP_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SCL_SHARP_EN {
SCL_SHARP_DISABLE                        = 0x00000000,
SCL_SHARP_ENABLE                         = 0x00000001,
}


/*******************************************************
 * CM Enums
 *******************************************************/

/*
 * CMC_3DLUT_30BIT_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CMC_3DLUT_30BIT_ENUM {
CMC_3DLUT_36BIT                          = 0x00000000,
CMC_3DLUT_30BIT                          = 0x00000001,
}


/*
 * CMC_3DLUT_RAM_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CMC_3DLUT_RAM_SEL {
CMC_RAM0_ACCESS                          = 0x00000000,
CMC_RAM1_ACCESS                          = 0x00000001,
CMC_RAM2_ACCESS                          = 0x00000002,
CMC_RAM3_ACCESS                          = 0x00000003,
}


/*
 * CMC_3DLUT_SIZE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CMC_3DLUT_SIZE_ENUM {
CMC_3DLUT_17CUBE                         = 0x00000000,
CMC_3DLUT_9CUBE                          = 0x00000001,
}


/*
 * CMC_LUT_2_CONFIG_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CMC_LUT_2_CONFIG_ENUM {
CMC_LUT_2CFG_NO_MEMORY                   = 0x00000000,
CMC_LUT_2CFG_MEMORY_A                    = 0x00000001,
CMC_LUT_2CFG_MEMORY_B                    = 0x00000002,
}


/*
 * CMC_LUT_2_MODE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CMC_LUT_2_MODE_ENUM {
CMC_LUT_2_MODE_BYPASS                    = 0x00000000,
CMC_LUT_2_MODE_RAMA_LUT                  = 0x00000001,
CMC_LUT_2_MODE_RAMB_LUT                  = 0x00000002,
}


/*
 * CMC_LUT_NUM_SEG enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CMC_LUT_NUM_SEG {
CMC_SEGMENTS_1                           = 0x00000000,
CMC_SEGMENTS_2                           = 0x00000001,
CMC_SEGMENTS_4                           = 0x00000002,
CMC_SEGMENTS_8                           = 0x00000003,
CMC_SEGMENTS_16                          = 0x00000004,
CMC_SEGMENTS_32                          = 0x00000005,
CMC_SEGMENTS_64                          = 0x00000006,
CMC_SEGMENTS_128                         = 0x00000007,
}


/*
 * CMC_LUT_RAM_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CMC_LUT_RAM_SEL {
CMC_RAMA_ACCESS                          = 0x00000000,
CMC_RAMB_ACCESS                          = 0x00000001,
}


/*
 * CM_BYPASS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_BYPASS {
NON_BYPASS                               = 0x00000000,
BYPASS_EN                                = 0x00000001,
}


/*
 * CM_COEF_FORMAT_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_COEF_FORMAT_ENUM {
FIX_S2_13                                = 0x00000000,
FIX_S3_12                                = 0x00000001,
}


/*
 * CM_DATA_SIGNED enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_DATA_SIGNED {
UNSIGNED                                 = 0x00000000,
SIGNED                                   = 0x00000001,
}


/*
 * CM_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_EN {
CM_DISABLE                               = 0x00000000,
CM_ENABLE                                = 0x00000001,
}


/*
 * CM_GAMMA_LUT_MODE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_GAMMA_LUT_MODE_ENUM {
BYPASS                                   = 0x00000000,
RESERVED_1                               = 0x00000001,
RAM_LUT                                  = 0x00000002,
RESERVED_3                               = 0x00000003,
}


/*
 * CM_GAMMA_LUT_PWL_DISABLE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_GAMMA_LUT_PWL_DISABLE_ENUM {
ENABLE_PWL                               = 0x00000000,
DISABLE_PWL                              = 0x00000001,
}


/*
 * CM_GAMMA_LUT_SEL_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_GAMMA_LUT_SEL_ENUM {
RAMA                                     = 0x00000000,
RAMB                                     = 0x00000001,
}


/*
 * CM_GAMUT_REMAP_MODE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_GAMUT_REMAP_MODE_ENUM {
BYPASS_GAMUT                             = 0x00000000,
GAMUT_COEF                               = 0x00000001,
GAMUT_COEF_B                             = 0x00000002,
}


/*
 * CM_LUT_2_CONFIG_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_LUT_2_CONFIG_ENUM {
LUT_2CFG_NO_MEMORY                       = 0x00000000,
LUT_2CFG_MEMORY_A                        = 0x00000001,
LUT_2CFG_MEMORY_B                        = 0x00000002,
}


/*
 * CM_LUT_2_MODE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_LUT_2_MODE_ENUM {
LUT_2_MODE_BYPASS                        = 0x00000000,
LUT_2_MODE_RAMA_LUT                      = 0x00000001,
LUT_2_MODE_RAMB_LUT                      = 0x00000002,
}


/*
 * CM_LUT_4_CONFIG_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_LUT_4_CONFIG_ENUM {
LUT_4CFG_NO_MEMORY                       = 0x00000000,
LUT_4CFG_ROM_A                           = 0x00000001,
LUT_4CFG_ROM_B                           = 0x00000002,
LUT_4CFG_MEMORY_A                        = 0x00000003,
LUT_4CFG_MEMORY_B                        = 0x00000004,
}


/*
 * CM_LUT_4_MODE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_LUT_4_MODE_ENUM {
LUT_4_MODE_BYPASS                        = 0x00000000,
LUT_4_MODE_ROMA_LUT                      = 0x00000001,
LUT_4_MODE_ROMB_LUT                      = 0x00000002,
LUT_4_MODE_RAMA_LUT                      = 0x00000003,
LUT_4_MODE_RAMB_LUT                      = 0x00000004,
}


/*
 * CM_LUT_CONFIG_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_LUT_CONFIG_MODE {
DIFFERENT_RGB                            = 0x00000000,
ALL_USE_R                                = 0x00000001,
}


/*
 * CM_LUT_NUM_SEG enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_LUT_NUM_SEG {
SEGMENTS_1                               = 0x00000000,
SEGMENTS_2                               = 0x00000001,
SEGMENTS_4                               = 0x00000002,
SEGMENTS_8                               = 0x00000003,
SEGMENTS_16                              = 0x00000004,
SEGMENTS_32                              = 0x00000005,
SEGMENTS_64                              = 0x00000006,
SEGMENTS_128                             = 0x00000007,
}


/*
 * CM_LUT_RAM_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_LUT_RAM_SEL {
RAMA_ACCESS                              = 0x00000000,
RAMB_ACCESS                              = 0x00000001,
}


/*
 * CM_LUT_READ_COLOR_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_LUT_READ_COLOR_SEL {
BLUE_LUT                                 = 0x00000000,
GREEN_LUT                                = 0x00000001,
RED_LUT                                  = 0x00000002,
}


/*
 * CM_LUT_READ_DBG enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_LUT_READ_DBG {
DISABLE_DEBUG                            = 0x00000000,
ENABLE_DEBUG                             = 0x00000001,
}


/*
 * CM_PENDING enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_PENDING {
CM_NOT_PENDING                           = 0x00000000,
CM_YES_PENDING                           = 0x00000001,
}


/*
 * CM_POST_CSC_MODE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_POST_CSC_MODE_ENUM {
BYPASS_POST_CSC                          = 0x00000000,
COEF_POST_CSC                            = 0x00000001,
COEF_POST_CSC_B                          = 0x00000002,
}


/*
 * CM_WRITE_BASE_ONLY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CM_WRITE_BASE_ONLY {
WRITE_BOTH                               = 0x00000000,
WRITE_BASE_ONLY                          = 0x00000001,
}


/*******************************************************
 * DPP_TOP Enums
 *******************************************************/

/*
 * CRC_CUR_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CRC_CUR_SEL {
CRC_CUR_0                                = 0x00000000,
CRC_CUR_1                                = 0x00000001,
}


/*
 * CRC_INTERLACE_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CRC_INTERLACE_SEL {
CRC_INTERLACE_0                          = 0x00000000,
CRC_INTERLACE_1                          = 0x00000001,
CRC_INTERLACE_2                          = 0x00000002,
CRC_INTERLACE_3                          = 0x00000003,
}


/*
 * CRC_IN_CUR_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CRC_IN_CUR_SEL {
CRC_IN_CUR_0                             = 0x00000000,
CRC_IN_CUR_1                             = 0x00000001,
CRC_IN_CUR_2                             = 0x00000002,
CRC_IN_CUR_3                             = 0x00000003,
}


/*
 * CRC_IN_PIX_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CRC_IN_PIX_SEL {
CRC_IN_PIX_0                             = 0x00000000,
CRC_IN_PIX_1                             = 0x00000001,
CRC_IN_PIX_2                             = 0x00000002,
CRC_IN_PIX_3                             = 0x00000003,
CRC_IN_PIX_4                             = 0x00000004,
CRC_IN_PIX_5                             = 0x00000005,
CRC_IN_PIX_6                             = 0x00000006,
CRC_IN_PIX_7                             = 0x00000007,
}


/*
 * CRC_SRC_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CRC_SRC_SEL {
CRC_SRC_0                                = 0x00000000,
CRC_SRC_1                                = 0x00000001,
CRC_SRC_2                                = 0x00000002,
CRC_SRC_3                                = 0x00000003,
}


/*
 * CRC_STEREO_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CRC_STEREO_SEL {
CRC_STEREO_0                             = 0x00000000,
CRC_STEREO_1                             = 0x00000001,
CRC_STEREO_2                             = 0x00000002,
CRC_STEREO_3                             = 0x00000003,
}


/*
 * TEST_CLK_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TEST_CLK_SEL {
TEST_CLK_SEL_0                           = 0x00000000,
TEST_CLK_SEL_1                           = 0x00000001,
TEST_CLK_SEL_2                           = 0x00000002,
TEST_CLK_SEL_3                           = 0x00000003,
TEST_CLK_SEL_4                           = 0x00000004,
TEST_CLK_SEL_5                           = 0x00000005,
TEST_CLK_SEL_6                           = 0x00000006,
TEST_CLK_SEL_7                           = 0x00000007,
}


/*******************************************************
 * DC_PERFMON Enums
 *******************************************************/

/*
 * PERFCOUNTER_ACTIVE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_ACTIVE {
PERFCOUNTER_IS_IDLE                      = 0x00000000,
PERFCOUNTER_IS_ACTIVE                    = 0x00000001,
}


/*
 * PERFCOUNTER_CNT0_STATE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_CNT0_STATE {
PERFCOUNTER_CNT0_STATE_RESET             = 0x00000000,
PERFCOUNTER_CNT0_STATE_START             = 0x00000001,
PERFCOUNTER_CNT0_STATE_FREEZE            = 0x00000002,
PERFCOUNTER_CNT0_STATE_HW                = 0x00000003,
}


/*
 * PERFCOUNTER_CNT1_STATE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_CNT1_STATE {
PERFCOUNTER_CNT1_STATE_RESET             = 0x00000000,
PERFCOUNTER_CNT1_STATE_START             = 0x00000001,
PERFCOUNTER_CNT1_STATE_FREEZE            = 0x00000002,
PERFCOUNTER_CNT1_STATE_HW                = 0x00000003,
}


/*
 * PERFCOUNTER_CNT2_STATE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_CNT2_STATE {
PERFCOUNTER_CNT2_STATE_RESET             = 0x00000000,
PERFCOUNTER_CNT2_STATE_START             = 0x00000001,
PERFCOUNTER_CNT2_STATE_FREEZE            = 0x00000002,
PERFCOUNTER_CNT2_STATE_HW                = 0x00000003,
}


/*
 * PERFCOUNTER_CNT3_STATE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_CNT3_STATE {
PERFCOUNTER_CNT3_STATE_RESET             = 0x00000000,
PERFCOUNTER_CNT3_STATE_START             = 0x00000001,
PERFCOUNTER_CNT3_STATE_FREEZE            = 0x00000002,
PERFCOUNTER_CNT3_STATE_HW                = 0x00000003,
}


/*
 * PERFCOUNTER_CNT4_STATE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_CNT4_STATE {
PERFCOUNTER_CNT4_STATE_RESET             = 0x00000000,
PERFCOUNTER_CNT4_STATE_START             = 0x00000001,
PERFCOUNTER_CNT4_STATE_FREEZE            = 0x00000002,
PERFCOUNTER_CNT4_STATE_HW                = 0x00000003,
}


/*
 * PERFCOUNTER_CNT5_STATE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_CNT5_STATE {
PERFCOUNTER_CNT5_STATE_RESET             = 0x00000000,
PERFCOUNTER_CNT5_STATE_START             = 0x00000001,
PERFCOUNTER_CNT5_STATE_FREEZE            = 0x00000002,
PERFCOUNTER_CNT5_STATE_HW                = 0x00000003,
}


/*
 * PERFCOUNTER_CNT6_STATE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_CNT6_STATE {
PERFCOUNTER_CNT6_STATE_RESET             = 0x00000000,
PERFCOUNTER_CNT6_STATE_START             = 0x00000001,
PERFCOUNTER_CNT6_STATE_FREEZE            = 0x00000002,
PERFCOUNTER_CNT6_STATE_HW                = 0x00000003,
}


/*
 * PERFCOUNTER_CNT7_STATE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_CNT7_STATE {
PERFCOUNTER_CNT7_STATE_RESET             = 0x00000000,
PERFCOUNTER_CNT7_STATE_START             = 0x00000001,
PERFCOUNTER_CNT7_STATE_FREEZE            = 0x00000002,
PERFCOUNTER_CNT7_STATE_HW                = 0x00000003,
}


/*
 * PERFCOUNTER_CNTL_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_CNTL_SEL {
PERFCOUNTER_CNTL_SEL_0                   = 0x00000000,
PERFCOUNTER_CNTL_SEL_1                   = 0x00000001,
PERFCOUNTER_CNTL_SEL_2                   = 0x00000002,
PERFCOUNTER_CNTL_SEL_3                   = 0x00000003,
PERFCOUNTER_CNTL_SEL_4                   = 0x00000004,
PERFCOUNTER_CNTL_SEL_5                   = 0x00000005,
PERFCOUNTER_CNTL_SEL_6                   = 0x00000006,
PERFCOUNTER_CNTL_SEL_7                   = 0x00000007,
}


/*
 * PERFCOUNTER_CNTOFF_START_DIS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_CNTOFF_START_DIS {
PERFCOUNTER_CNTOFF_START_ENABLE          = 0x00000000,
PERFCOUNTER_CNTOFF_START_DISABLE         = 0x00000001,
}


/*
 * PERFCOUNTER_COUNTED_VALUE_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_COUNTED_VALUE_TYPE {
PERFCOUNTER_COUNTED_VALUE_TYPE_ACC       = 0x00000000,
PERFCOUNTER_COUNTED_VALUE_TYPE_MAX       = 0x00000001,
PERFCOUNTER_COUNTED_VALUE_TYPE_MIN       = 0x00000002,
}


/*
 * PERFCOUNTER_CVALUE_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_CVALUE_SEL {
PERFCOUNTER_CVALUE_SEL_47_0              = 0x00000000,
PERFCOUNTER_CVALUE_SEL_15_0              = 0x00000001,
PERFCOUNTER_CVALUE_SEL_31_16             = 0x00000002,
PERFCOUNTER_CVALUE_SEL_47_32             = 0x00000003,
PERFCOUNTER_CVALUE_SEL_11_0              = 0x00000004,
PERFCOUNTER_CVALUE_SEL_23_12             = 0x00000005,
PERFCOUNTER_CVALUE_SEL_35_24             = 0x00000006,
PERFCOUNTER_CVALUE_SEL_47_36             = 0x00000007,
}


/*
 * PERFCOUNTER_HW_CNTL_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_HW_CNTL_SEL {
PERFCOUNTER_HW_CNTL_SEL_RUNEN            = 0x00000000,
PERFCOUNTER_HW_CNTL_SEL_CNTOFF           = 0x00000001,
}


/*
 * PERFCOUNTER_HW_STOP1_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_HW_STOP1_SEL {
PERFCOUNTER_HW_STOP1_0                   = 0x00000000,
PERFCOUNTER_HW_STOP1_1                   = 0x00000001,
}


/*
 * PERFCOUNTER_HW_STOP2_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_HW_STOP2_SEL {
PERFCOUNTER_HW_STOP2_0                   = 0x00000000,
PERFCOUNTER_HW_STOP2_1                   = 0x00000001,
}


/*
 * PERFCOUNTER_INC_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_INC_MODE {
PERFCOUNTER_INC_MODE_MULTI_BIT           = 0x00000000,
PERFCOUNTER_INC_MODE_BOTH_EDGE           = 0x00000001,
PERFCOUNTER_INC_MODE_LSB                 = 0x00000002,
PERFCOUNTER_INC_MODE_POS_EDGE            = 0x00000003,
PERFCOUNTER_INC_MODE_NEG_EDGE            = 0x00000004,
}


/*
 * PERFCOUNTER_INT_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_INT_EN {
PERFCOUNTER_INT_DISABLE                  = 0x00000000,
PERFCOUNTER_INT_ENABLE                   = 0x00000001,
}


/*
 * PERFCOUNTER_INT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_INT_TYPE {
PERFCOUNTER_INT_TYPE_LEVEL               = 0x00000000,
PERFCOUNTER_INT_TYPE_PULSE               = 0x00000001,
}


/*
 * PERFCOUNTER_OFF_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_OFF_MASK {
PERFCOUNTER_OFF_MASK_DISABLE             = 0x00000000,
PERFCOUNTER_OFF_MASK_ENABLE              = 0x00000001,
}


/*
 * PERFCOUNTER_RESTART_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_RESTART_EN {
PERFCOUNTER_RESTART_DISABLE              = 0x00000000,
PERFCOUNTER_RESTART_ENABLE               = 0x00000001,
}


/*
 * PERFCOUNTER_RUNEN_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_RUNEN_MODE {
PERFCOUNTER_RUNEN_MODE_LEVEL             = 0x00000000,
PERFCOUNTER_RUNEN_MODE_EDGE              = 0x00000001,
}


/*
 * PERFCOUNTER_STATE_SEL0 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_STATE_SEL0 {
PERFCOUNTER_STATE_SEL0_GLOBAL            = 0x00000000,
PERFCOUNTER_STATE_SEL0_LOCAL             = 0x00000001,
}


/*
 * PERFCOUNTER_STATE_SEL1 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_STATE_SEL1 {
PERFCOUNTER_STATE_SEL1_GLOBAL            = 0x00000000,
PERFCOUNTER_STATE_SEL1_LOCAL             = 0x00000001,
}


/*
 * PERFCOUNTER_STATE_SEL2 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_STATE_SEL2 {
PERFCOUNTER_STATE_SEL2_GLOBAL            = 0x00000000,
PERFCOUNTER_STATE_SEL2_LOCAL             = 0x00000001,
}


/*
 * PERFCOUNTER_STATE_SEL3 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_STATE_SEL3 {
PERFCOUNTER_STATE_SEL3_GLOBAL            = 0x00000000,
PERFCOUNTER_STATE_SEL3_LOCAL             = 0x00000001,
}


/*
 * PERFCOUNTER_STATE_SEL4 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_STATE_SEL4 {
PERFCOUNTER_STATE_SEL4_GLOBAL            = 0x00000000,
PERFCOUNTER_STATE_SEL4_LOCAL             = 0x00000001,
}


/*
 * PERFCOUNTER_STATE_SEL5 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_STATE_SEL5 {
PERFCOUNTER_STATE_SEL5_GLOBAL            = 0x00000000,
PERFCOUNTER_STATE_SEL5_LOCAL             = 0x00000001,
}


/*
 * PERFCOUNTER_STATE_SEL6 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_STATE_SEL6 {
PERFCOUNTER_STATE_SEL6_GLOBAL            = 0x00000000,
PERFCOUNTER_STATE_SEL6_LOCAL             = 0x00000001,
}


/*
 * PERFCOUNTER_STATE_SEL7 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFCOUNTER_STATE_SEL7 {
PERFCOUNTER_STATE_SEL7_GLOBAL            = 0x00000000,
PERFCOUNTER_STATE_SEL7_LOCAL             = 0x00000001,
}


/*
 * PERFMON_CNTOFF_AND_OR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFMON_CNTOFF_AND_OR {
PERFMON_CNTOFF_OR                        = 0x00000000,
PERFMON_CNTOFF_AND                       = 0x00000001,
}


/*
 * PERFMON_CNTOFF_INT_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFMON_CNTOFF_INT_EN {
PERFMON_CNTOFF_INT_DISABLE               = 0x00000000,
PERFMON_CNTOFF_INT_ENABLE                = 0x00000001,
}


/*
 * PERFMON_CNTOFF_INT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFMON_CNTOFF_INT_TYPE {
PERFMON_CNTOFF_INT_TYPE_LEVEL            = 0x00000000,
PERFMON_CNTOFF_INT_TYPE_PULSE            = 0x00000001,
}


/*
 * PERFMON_STATE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PERFMON_STATE {
PERFMON_STATE_RESET                      = 0x00000000,
PERFMON_STATE_START                      = 0x00000001,
PERFMON_STATE_FREEZE                     = 0x00000002,
PERFMON_STATE_HW                         = 0x00000003,
}


/*******************************************************
 * HUBP Enums
 *******************************************************/

/*
 * BIGK_FRAGMENT_SIZE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BIGK_FRAGMENT_SIZE {
VM_PG_SIZE_4KB                           = 0x00000000,
VM_PG_SIZE_8KB                           = 0x00000001,
VM_PG_SIZE_16KB                          = 0x00000002,
VM_PG_SIZE_32KB                          = 0x00000003,
VM_PG_SIZE_64KB                          = 0x00000004,
VM_PG_SIZE_128KB                         = 0x00000005,
VM_PG_SIZE_256KB                         = 0x00000006,
VM_PG_SIZE_512KB                         = 0x00000007,
VM_PG_SIZE_1024KB                        = 0x00000008,
VM_PG_SIZE_2048KB                        = 0x00000009,
}


/*
 * CHUNK_SIZE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CHUNK_SIZE {
CHUNK_SIZE_1KB                           = 0x00000000,
CHUNK_SIZE_2KB                           = 0x00000001,
CHUNK_SIZE_4KB                           = 0x00000002,
CHUNK_SIZE_8KB                           = 0x00000003,
CHUNK_SIZE_16KB                          = 0x00000004,
CHUNK_SIZE_32KB                          = 0x00000005,
CHUNK_SIZE_64KB                          = 0x00000006,
}


/*
 * COMPAT_LEVEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum COMPAT_LEVEL {
ADDR_GEN_ZERO                            = 0x00000000,
ADDR_GEN_ONE                             = 0x00000001,
ADDR_GEN_TWO                             = 0x00000002,
ADDR_RESERVED                            = 0x00000003,
}


/*
 * DPTE_GROUP_SIZE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPTE_GROUP_SIZE {
DPTE_GROUP_SIZE_64B                      = 0x00000000,
DPTE_GROUP_SIZE_128B                     = 0x00000001,
DPTE_GROUP_SIZE_256B                     = 0x00000002,
DPTE_GROUP_SIZE_512B                     = 0x00000003,
DPTE_GROUP_SIZE_1024B                    = 0x00000004,
DPTE_GROUP_SIZE_2048B                    = 0x00000005,
}


/*
 * FORCE_ONE_ROW_FOR_FRAME enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FORCE_ONE_ROW_FOR_FRAME {
FORCE_ONE_ROW_FOR_FRAME_0                = 0x00000000,
FORCE_ONE_ROW_FOR_FRAME_1                = 0x00000001,
}


/*
 * HUBP_BLANK_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HUBP_BLANK_EN {
HUBP_BLANK_SW_DEASSERT                   = 0x00000000,
HUBP_BLANK_SW_ASSERT                     = 0x00000001,
}


/*
 * HUBP_IN_BLANK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HUBP_IN_BLANK {
HUBP_IN_ACTIVE                           = 0x00000000,
HUBP_IN_VBLANK                           = 0x00000001,
}


/*
 * HUBP_MEASURE_WIN_MODE_DCFCLK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HUBP_MEASURE_WIN_MODE_DCFCLK {
HUBP_MEASURE_WIN_MODE_DCFCLK_0           = 0x00000000,
HUBP_MEASURE_WIN_MODE_DCFCLK_1           = 0x00000001,
HUBP_MEASURE_WIN_MODE_DCFCLK_2           = 0x00000002,
HUBP_MEASURE_WIN_MODE_DCFCLK_3           = 0x00000003,
}


/*
 * HUBP_NO_OUTSTANDING_REQ enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HUBP_NO_OUTSTANDING_REQ {
OUTSTANDING_REQ                          = 0x00000000,
NO_OUTSTANDING_REQ                       = 0x00000001,
}


/*
 * HUBP_SOFT_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HUBP_SOFT_RESET {
HUBP_SOFT_RESET_ON                       = 0x00000000,
HUBP_SOFT_RESET_OFF                      = 0x00000001,
}


/*
 * HUBP_TTU_DISABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HUBP_TTU_DISABLE {
HUBP_TTU_ENABLED                         = 0x00000000,
HUBP_TTU_DISABLED                        = 0x00000001,
}


/*
 * HUBP_VREADY_AT_OR_AFTER_VSYNC enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HUBP_VREADY_AT_OR_AFTER_VSYNC {
VREADY_BEFORE_VSYNC                      = 0x00000000,
VREADY_AT_OR_AFTER_VSYNC                 = 0x00000001,
}


/*
 * HUBP_VTG_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HUBP_VTG_SEL {
VTG_SEL_0                                = 0x00000000,
VTG_SEL_1                                = 0x00000001,
VTG_SEL_2                                = 0x00000002,
VTG_SEL_3                                = 0x00000003,
VTG_SEL_4                                = 0x00000004,
VTG_SEL_5                                = 0x00000005,
}


/*
 * H_MIRROR_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum H_MIRROR_EN {
HW_MIRRORING_DISABLE                     = 0x00000000,
HW_MIRRORING_ENABLE                      = 0x00000001,
}


/*
 * LEGACY_PIPE_INTERLEAVE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LEGACY_PIPE_INTERLEAVE {
LEGACY_PIPE_INTERLEAVE_256B              = 0x00000000,
LEGACY_PIPE_INTERLEAVE_512B              = 0x00000001,
}


/*
 * META_CHUNK_SIZE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum META_CHUNK_SIZE {
META_CHUNK_SIZE_1KB                      = 0x00000000,
META_CHUNK_SIZE_2KB                      = 0x00000001,
META_CHUNK_SIZE_4KB                      = 0x00000002,
META_CHUNK_SIZE_8KB                      = 0x00000003,
}


/*
 * META_LINEAR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum META_LINEAR {
META_SURF_TILED                          = 0x00000000,
META_SURF_LINEAR                         = 0x00000001,
}


/*
 * MIN_CHUNK_SIZE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MIN_CHUNK_SIZE {
NO_MIN_CHUNK_SIZE                        = 0x00000000,
MIN_CHUNK_SIZE_256B                      = 0x00000001,
MIN_CHUNK_SIZE_512B                      = 0x00000002,
MIN_CHUNK_SIZE_1024B                     = 0x00000003,
}


/*
 * MIN_META_CHUNK_SIZE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MIN_META_CHUNK_SIZE {
NO_MIN_META_CHUNK_SIZE                   = 0x00000000,
MIN_META_CHUNK_SIZE_64B                  = 0x00000001,
MIN_META_CHUNK_SIZE_128B                 = 0x00000002,
MIN_META_CHUNK_SIZE_256B                 = 0x00000003,
}


/*
 * PIPE_ALIGNED enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PIPE_ALIGNED {
PIPE_UNALIGNED_SURF                      = 0x00000000,
PIPE_ALIGNED_SURF                        = 0x00000001,
}


/*
 * PTE_BUFFER_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PTE_BUFFER_MODE {
PTE_BUFFER_MODE_0                        = 0x00000000,
PTE_BUFFER_MODE_1                        = 0x00000001,
}


/*
 * PTE_ROW_HEIGHT_LINEAR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PTE_ROW_HEIGHT_LINEAR {
PTE_ROW_HEIGHT_LINEAR_8L                 = 0x00000000,
PTE_ROW_HEIGHT_LINEAR_16L                = 0x00000001,
PTE_ROW_HEIGHT_LINEAR_32L                = 0x00000002,
PTE_ROW_HEIGHT_LINEAR_64L                = 0x00000003,
PTE_ROW_HEIGHT_LINEAR_128L               = 0x00000004,
PTE_ROW_HEIGHT_LINEAR_256L               = 0x00000005,
PTE_ROW_HEIGHT_LINEAR_512L               = 0x00000006,
PTE_ROW_HEIGHT_LINEAR_1024L              = 0x00000007,
}


/*
 * ROTATION_ANGLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ROTATION_ANGLE {
ROTATE_0_DEGREES                         = 0x00000000,
ROTATE_90_DEGREES                        = 0x00000001,
ROTATE_180_DEGREES                       = 0x00000002,
ROTATE_270_DEGREES                       = 0x00000003,
}


/*
 * SWATH_HEIGHT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SWATH_HEIGHT {
SWATH_HEIGHT_1L                          = 0x00000000,
SWATH_HEIGHT_2L                          = 0x00000001,
SWATH_HEIGHT_4L                          = 0x00000002,
SWATH_HEIGHT_8L                          = 0x00000003,
SWATH_HEIGHT_16L                         = 0x00000004,
}


/*
 * USE_MALL_FOR_CURSOR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum USE_MALL_FOR_CURSOR {
USE_MALL_FOR_CURSOR_0                    = 0x00000000,
USE_MALL_FOR_CURSOR_1                    = 0x00000001,
}


/*
 * USE_MALL_FOR_PSTATE_CHANGE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum USE_MALL_FOR_PSTATE_CHANGE {
USE_MALL_FOR_PSTATE_CHANGE_0             = 0x00000000,
USE_MALL_FOR_PSTATE_CHANGE_1             = 0x00000001,
}


/*
 * USE_MALL_FOR_STATIC_SCREEN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum USE_MALL_FOR_STATIC_SCREEN {
USE_MALL_FOR_STATIC_SCREEN_0             = 0x00000000,
USE_MALL_FOR_STATIC_SCREEN_1             = 0x00000001,
}


/*
 * VMPG_SIZE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VMPG_SIZE {
VMPG_SIZE_4KB                            = 0x00000000,
VMPG_SIZE_64KB                           = 0x00000001,
}


/*
 * VM_GROUP_SIZE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VM_GROUP_SIZE {
VM_GROUP_SIZE_64B                        = 0x00000000,
VM_GROUP_SIZE_128B                       = 0x00000001,
VM_GROUP_SIZE_256B                       = 0x00000002,
VM_GROUP_SIZE_512B                       = 0x00000003,
VM_GROUP_SIZE_1024B                      = 0x00000004,
VM_GROUP_SIZE_2048B                      = 0x00000005,
}


/*******************************************************
 * HUBPREQ Enums
 *******************************************************/

/*
 * DFQ_MIN_FREE_ENTRIES enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DFQ_MIN_FREE_ENTRIES {
DFQ_MIN_FREE_ENTRIES_0                   = 0x00000000,
DFQ_MIN_FREE_ENTRIES_1                   = 0x00000001,
DFQ_MIN_FREE_ENTRIES_2                   = 0x00000002,
DFQ_MIN_FREE_ENTRIES_3                   = 0x00000003,
DFQ_MIN_FREE_ENTRIES_4                   = 0x00000004,
DFQ_MIN_FREE_ENTRIES_5                   = 0x00000005,
DFQ_MIN_FREE_ENTRIES_6                   = 0x00000006,
DFQ_MIN_FREE_ENTRIES_7                   = 0x00000007,
}


/*
 * DFQ_NUM_ENTRIES enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DFQ_NUM_ENTRIES {
DFQ_NUM_ENTRIES_0                        = 0x00000000,
DFQ_NUM_ENTRIES_1                        = 0x00000001,
DFQ_NUM_ENTRIES_2                        = 0x00000002,
DFQ_NUM_ENTRIES_3                        = 0x00000003,
DFQ_NUM_ENTRIES_4                        = 0x00000004,
DFQ_NUM_ENTRIES_5                        = 0x00000005,
DFQ_NUM_ENTRIES_6                        = 0x00000006,
DFQ_NUM_ENTRIES_7                        = 0x00000007,
DFQ_NUM_ENTRIES_8                        = 0x00000008,
}


/*
 * DFQ_SIZE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DFQ_SIZE {
DFQ_SIZE_0                               = 0x00000000,
DFQ_SIZE_1                               = 0x00000001,
DFQ_SIZE_2                               = 0x00000002,
DFQ_SIZE_3                               = 0x00000003,
DFQ_SIZE_4                               = 0x00000004,
DFQ_SIZE_5                               = 0x00000005,
DFQ_SIZE_6                               = 0x00000006,
DFQ_SIZE_7                               = 0x00000007,
}


/*
 * DMDATA_VM_DONE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DMDATA_VM_DONE {
DMDATA_VM_IS_NOT_DONE                    = 0x00000000,
DMDATA_VM_IS_DONE                        = 0x00000001,
}


/*
 * EXPANSION_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EXPANSION_MODE {
EXPANSION_MODE_ZERO                      = 0x00000000,
EXPANSION_MODE_CONSERVATIVE              = 0x00000001,
EXPANSION_MODE_OPTIMAL                   = 0x00000002,
}


/*
 * FLIP_RATE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FLIP_RATE {
FLIP_RATE_0                              = 0x00000000,
FLIP_RATE_1                              = 0x00000001,
FLIP_RATE_2                              = 0x00000002,
FLIP_RATE_3                              = 0x00000003,
FLIP_RATE_4                              = 0x00000004,
FLIP_RATE_5                              = 0x00000005,
FLIP_RATE_6                              = 0x00000006,
FLIP_RATE_7                              = 0x00000007,
}


/*
 * INT_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum INT_MASK {
INT_DISABLED                             = 0x00000000,
INT_ENABLED                              = 0x00000001,
}


/*
 * PIPE_IN_FLUSH_URGENT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PIPE_IN_FLUSH_URGENT {
PIPE_IN_FLUSH_URGENT_ENABLE              = 0x00000000,
PIPE_IN_FLUSH_URGENT_DISABLE             = 0x00000001,
}


/*
 * PRQ_MRQ_FLUSH_URGENT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PRQ_MRQ_FLUSH_URGENT {
PRQ_MRQ_FLUSH_URGENT_ENABLE              = 0x00000000,
PRQ_MRQ_FLUSH_URGENT_DISABLE             = 0x00000001,
}


/*
 * ROW_TTU_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ROW_TTU_MODE {
END_OF_ROW_MODE                          = 0x00000000,
WATERMARK_MODE                           = 0x00000001,
}


/*
 * SURFACE_DCC enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURFACE_DCC {
SURFACE_IS_NOT_DCC                       = 0x00000000,
SURFACE_IS_DCC                           = 0x00000001,
}


/*
 * SURFACE_DCC_IND_128B enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURFACE_DCC_IND_128B {
SURFACE_DCC_IS_NOT_IND_128B              = 0x00000000,
SURFACE_DCC_IS_IND_128B                  = 0x00000001,
}


/*
 * SURFACE_DCC_IND_64B enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURFACE_DCC_IND_64B {
SURFACE_DCC_IS_NOT_IND_64B               = 0x00000000,
SURFACE_DCC_IS_IND_64B                   = 0x00000001,
}


/*
 * SURFACE_DCC_IND_BLK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURFACE_DCC_IND_BLK {
SURFACE_DCC_BLOCK_IS_UNCONSTRAINED       = 0x00000000,
SURFACE_DCC_BLOCK_IS_IND_64B             = 0x00000001,
SURFACE_DCC_BLOCK_IS_IND_128B            = 0x00000002,
SURFACE_DCC_BLOCK_IS_IND_64B_NO_128BCL   = 0x00000003,
}


/*
 * SURFACE_FLIP_AWAY_INT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURFACE_FLIP_AWAY_INT_TYPE {
SURFACE_FLIP_AWAY_INT_LEVEL              = 0x00000000,
SURFACE_FLIP_AWAY_INT_PULSE              = 0x00000001,
}


/*
 * SURFACE_FLIP_EXEC_DEBUG_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURFACE_FLIP_EXEC_DEBUG_MODE {
SURFACE_FLIP_EXEC_NORMAL_MODE            = 0x00000000,
SURFACE_FLIP_EXEC_DEBUG_MODE_ENABLE      = 0x00000001,
}


/*
 * SURFACE_FLIP_INT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURFACE_FLIP_INT_TYPE {
SURFACE_FLIP_INT_LEVEL                   = 0x00000000,
SURFACE_FLIP_INT_PULSE                   = 0x00000001,
}


/*
 * SURFACE_FLIP_IN_STEREOSYNC enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURFACE_FLIP_IN_STEREOSYNC {
SURFACE_FLIP_NOT_IN_STEREOSYNC_MODE      = 0x00000000,
SURFACE_FLIP_IN_STEREOSYNC_MODE          = 0x00000001,
}


/*
 * SURFACE_FLIP_MODE_FOR_STEREOSYNC enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURFACE_FLIP_MODE_FOR_STEREOSYNC {
FLIP_ANY_FRAME                           = 0x00000000,
FLIP_LEFT_EYE                            = 0x00000001,
FLIP_RIGHT_EYE                           = 0x00000002,
SURFACE_FLIP_MODE_FOR_STEREOSYNC_RESERVED = 0x00000003,
}


/*
 * SURFACE_FLIP_STEREO_SELECT_DISABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURFACE_FLIP_STEREO_SELECT_DISABLE {
SURFACE_FLIP_STEREO_SELECT_ENABLED       = 0x00000000,
SURFACE_FLIP_STEREO_SELECT_DISABLED      = 0x00000001,
}


/*
 * SURFACE_FLIP_STEREO_SELECT_POLARITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURFACE_FLIP_STEREO_SELECT_POLARITY {
SURFACE_FLIP_STEREO_SELECT_POLARITY_NOT_INVERT = 0x00000000,
SURFACE_FLIP_STEREO_SELECT_POLARITY_INVERT = 0x00000001,
}


/*
 * SURFACE_FLIP_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURFACE_FLIP_TYPE {
SURFACE_V_FLIP                           = 0x00000000,
SURFACE_I_FLIP                           = 0x00000001,
}


/*
 * SURFACE_FLIP_VUPDATE_SKIP_NUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURFACE_FLIP_VUPDATE_SKIP_NUM {
SURFACE_FLIP_VUPDATE_SKIP_NUM_0          = 0x00000000,
SURFACE_FLIP_VUPDATE_SKIP_NUM_1          = 0x00000001,
SURFACE_FLIP_VUPDATE_SKIP_NUM_2          = 0x00000002,
SURFACE_FLIP_VUPDATE_SKIP_NUM_3          = 0x00000003,
SURFACE_FLIP_VUPDATE_SKIP_NUM_4          = 0x00000004,
SURFACE_FLIP_VUPDATE_SKIP_NUM_5          = 0x00000005,
SURFACE_FLIP_VUPDATE_SKIP_NUM_6          = 0x00000006,
SURFACE_FLIP_VUPDATE_SKIP_NUM_7          = 0x00000007,
SURFACE_FLIP_VUPDATE_SKIP_NUM_8          = 0x00000008,
SURFACE_FLIP_VUPDATE_SKIP_NUM_9          = 0x00000009,
SURFACE_FLIP_VUPDATE_SKIP_NUM_10         = 0x0000000a,
SURFACE_FLIP_VUPDATE_SKIP_NUM_11         = 0x0000000b,
SURFACE_FLIP_VUPDATE_SKIP_NUM_12         = 0x0000000c,
SURFACE_FLIP_VUPDATE_SKIP_NUM_13         = 0x0000000d,
SURFACE_FLIP_VUPDATE_SKIP_NUM_14         = 0x0000000e,
SURFACE_FLIP_VUPDATE_SKIP_NUM_15         = 0x0000000f,
}


/*
 * SURFACE_INUSE_RAED_NO_LATCH enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURFACE_INUSE_RAED_NO_LATCH {
SURFACE_INUSE_IS_LATCHED                 = 0x00000000,
SURFACE_INUSE_IS_NOT_LATCHED             = 0x00000001,
}


/*
 * SURFACE_TMZ enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURFACE_TMZ {
SURFACE_IS_NOT_TMZ                       = 0x00000000,
SURFACE_IS_TMZ                           = 0x00000001,
}


/*
 * SURFACE_UPDATE_LOCK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURFACE_UPDATE_LOCK {
SURFACE_UPDATE_IS_UNLOCKED               = 0x00000000,
SURFACE_UPDATE_IS_LOCKED                 = 0x00000001,
}


/*******************************************************
 * HUBPRET Enums
 *******************************************************/

/*
 * CROSSBAR_FOR_ALPHA enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CROSSBAR_FOR_ALPHA {
ALPHA_DATA_ONTO_ALPHA_PORT               = 0x00000000,
Y_G_DATA_ONTO_ALPHA_PORT                 = 0x00000001,
CB_B_DATA_ONTO_ALPHA_PORT                = 0x00000002,
CR_R_DATA_ONTO_ALPHA_PORT                = 0x00000003,
}


/*
 * CROSSBAR_FOR_CB_B enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CROSSBAR_FOR_CB_B {
ALPHA_DATA_ONTO_CB_B_PORT                = 0x00000000,
Y_G_DATA_ONTO_CB_B_PORT                  = 0x00000001,
CB_B_DATA_ONTO_CB_B_PORT                 = 0x00000002,
CR_R_DATA_ONTO_CB_B_PORT                 = 0x00000003,
}


/*
 * CROSSBAR_FOR_CR_R enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CROSSBAR_FOR_CR_R {
ALPHA_DATA_ONTO_CR_R_PORT                = 0x00000000,
Y_G_DATA_ONTO_CR_R_PORT                  = 0x00000001,
CB_B_DATA_ONTO_CR_R_PORT                 = 0x00000002,
CR_R_DATA_ONTO_CR_R_PORT                 = 0x00000003,
}


/*
 * CROSSBAR_FOR_Y_G enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CROSSBAR_FOR_Y_G {
ALPHA_DATA_ONTO_Y_G_PORT                 = 0x00000000,
Y_G_DATA_ONTO_Y_G_PORT                   = 0x00000001,
CB_B_DATA_ONTO_Y_G_PORT                  = 0x00000002,
CR_R_DATA_ONTO_Y_G_PORT                  = 0x00000003,
}


/*
 * DETILE_BUFFER_PACKER_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DETILE_BUFFER_PACKER_ENABLE {
DETILE_BUFFER_PACKER_IS_DISABLE          = 0x00000000,
DETILE_BUFFER_PACKER_IS_ENABLE           = 0x00000001,
}


/*
 * MEM_PWR_DIS_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MEM_PWR_DIS_MODE {
MEM_POWER_DIS_MODE_ENABLE                = 0x00000000,
MEM_POWER_DIS_MODE_DISABLE               = 0x00000001,
}


/*
 * MEM_PWR_FORCE_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MEM_PWR_FORCE_MODE {
MEM_POWER_FORCE_MODE_OFF                 = 0x00000000,
MEM_POWER_FORCE_MODE_LIGHT_SLEEP         = 0x00000001,
MEM_POWER_FORCE_MODE_DEEP_SLEEP          = 0x00000002,
MEM_POWER_FORCE_MODE_SHUT_DOWN           = 0x00000003,
}


/*
 * MEM_PWR_STATUS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MEM_PWR_STATUS {
MEM_POWER_STATUS_ON                      = 0x00000000,
MEM_POWER_STATUS_LIGHT_SLEEP             = 0x00000001,
MEM_POWER_STATUS_DEEP_SLEEP              = 0x00000002,
MEM_POWER_STATUS_SHUT_DOWN               = 0x00000003,
}


/*
 * PIPE_INT_MASK_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PIPE_INT_MASK_MODE {
PIPE_INT_MASK_MODE_DISABLE               = 0x00000000,
PIPE_INT_MASK_MODE_ENABLE                = 0x00000001,
}


/*
 * PIPE_INT_TYPE_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PIPE_INT_TYPE_MODE {
PIPE_INT_TYPE_MODE_DISABLE               = 0x00000000,
PIPE_INT_TYPE_MODE_ENABLE                = 0x00000001,
}


/*
 * PIXCDC_MEM_PWR_LIGHT_SLEEP_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PIXCDC_MEM_PWR_LIGHT_SLEEP_MODE {
PIXCDC_MEM_POWER_LIGHT_SLEEP_MODE_OFF    = 0x00000000,
PIXCDC_MEM_POWER_LIGHT_SLEEP_MODE_1      = 0x00000001,
}


/*******************************************************
 * CURSOR Enums
 *******************************************************/

/*
 * CROB_MEM_PWR_LIGHT_SLEEP_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CROB_MEM_PWR_LIGHT_SLEEP_MODE {
CROB_MEM_POWER_LIGHT_SLEEP_MODE_OFF      = 0x00000000,
CROB_MEM_POWER_LIGHT_SLEEP_MODE_1        = 0x00000001,
CROB_MEM_POWER_LIGHT_SLEEP_MODE_2        = 0x00000002,
}


/*
 * CURSOR_2X_MAGNIFY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CURSOR_2X_MAGNIFY {
CURSOR_2X_MAGNIFY_IS_DISABLE             = 0x00000000,
CURSOR_2X_MAGNIFY_IS_ENABLE              = 0x00000001,
}


/*
 * CURSOR_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CURSOR_ENABLE {
CURSOR_IS_DISABLE                        = 0x00000000,
CURSOR_IS_ENABLE                         = 0x00000001,
}


/*
 * CURSOR_LINES_PER_CHUNK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CURSOR_LINES_PER_CHUNK {
CURSOR_LINE_PER_CHUNK_1                  = 0x00000000,
CURSOR_LINE_PER_CHUNK_2                  = 0x00000001,
CURSOR_LINE_PER_CHUNK_4                  = 0x00000002,
CURSOR_LINE_PER_CHUNK_8                  = 0x00000003,
CURSOR_LINE_PER_CHUNK_16                 = 0x00000004,
}


/*
 * CURSOR_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CURSOR_MODE {
CURSOR_MONO_2BIT                         = 0x00000000,
CURSOR_COLOR_24BIT_1BIT_AND              = 0x00000001,
CURSOR_COLOR_24BIT_8BIT_ALPHA_PREMULT    = 0x00000002,
CURSOR_COLOR_24BIT_8BIT_ALPHA_UNPREMULT  = 0x00000003,
CURSOR_COLOR_64BIT_FP_PREMULT            = 0x00000004,
CURSOR_COLOR_64BIT_FP_UNPREMULT          = 0x00000005,
}


/*
 * CURSOR_PERFMON_LATENCY_MEASURE_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CURSOR_PERFMON_LATENCY_MEASURE_EN {
CURSOR_PERFMON_LATENCY_MEASURE_IS_DISABLED = 0x00000000,
CURSOR_PERFMON_LATENCY_MEASURE_IS_ENABLED = 0x00000001,
}


/*
 * CURSOR_PERFMON_LATENCY_MEASURE_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CURSOR_PERFMON_LATENCY_MEASURE_SEL {
CURSOR_PERFMON_LATENCY_MEASURE_MC_LATENCY = 0x00000000,
CURSOR_PERFMON_LATENCY_MEASURE_CROB_LATENCY = 0x00000001,
}


/*
 * CURSOR_PITCH enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CURSOR_PITCH {
CURSOR_PITCH_64_PIXELS                   = 0x00000000,
CURSOR_PITCH_128_PIXELS                  = 0x00000001,
CURSOR_PITCH_256_PIXELS                  = 0x00000002,
}


/*
 * CURSOR_REQ_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CURSOR_REQ_MODE {
CURSOR_REQUEST_NORMALLY                  = 0x00000000,
CURSOR_REQUEST_EARLY                     = 0x00000001,
}


/*
 * CURSOR_SNOOP enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CURSOR_SNOOP {
CURSOR_IS_NOT_SNOOP                      = 0x00000000,
CURSOR_IS_SNOOP                          = 0x00000001,
}


/*
 * CURSOR_STEREO_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CURSOR_STEREO_EN {
CURSOR_STEREO_IS_DISABLED                = 0x00000000,
CURSOR_STEREO_IS_ENABLED                 = 0x00000001,
}


/*
 * CURSOR_SURFACE_TMZ enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CURSOR_SURFACE_TMZ {
CURSOR_SURFACE_IS_NOT_TMZ                = 0x00000000,
CURSOR_SURFACE_IS_TMZ                    = 0x00000001,
}


/*
 * CURSOR_SYSTEM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CURSOR_SYSTEM {
CURSOR_IN_SYSTEM_PHYSICAL_ADDRESS        = 0x00000000,
CURSOR_IN_GUEST_PHYSICAL_ADDRESS         = 0x00000001,
}


/*
 * CURSOR_XY_POSITION_ROTATION_AND_MIRRORING_BYPASS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CURSOR_XY_POSITION_ROTATION_AND_MIRRORING_BYPASS {
CURSOR_XY_POSITION_ROTATION_AND_MIRRORING_BYPASS_0 = 0x00000000,
CURSOR_XY_POSITION_ROTATION_AND_MIRRORING_BYPASS_1 = 0x00000001,
}


/*
 * DMDATA_DONE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DMDATA_DONE {
DMDATA_NOT_SENT_TO_DIG                   = 0x00000000,
DMDATA_SENT_TO_DIG                       = 0x00000001,
}


/*
 * DMDATA_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DMDATA_MODE {
DMDATA_SOFTWARE_UPDATE_MODE              = 0x00000000,
DMDATA_HARDWARE_UPDATE_MODE              = 0x00000001,
}


/*
 * DMDATA_QOS_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DMDATA_QOS_MODE {
DMDATA_QOS_LEVEL_FROM_TTU                = 0x00000000,
DMDATA_QOS_LEVEL_FROM_SOFTWARE           = 0x00000001,
}


/*
 * DMDATA_REPEAT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DMDATA_REPEAT {
DMDATA_USE_FOR_CURRENT_FRAME_ONLY        = 0x00000000,
DMDATA_USE_FOR_CURRENT_AND_FUTURE_FRAMES = 0x00000001,
}


/*
 * DMDATA_UNDERFLOW enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DMDATA_UNDERFLOW {
DMDATA_NOT_UNDERFLOW                     = 0x00000000,
DMDATA_UNDERFLOWED                       = 0x00000001,
}


/*
 * DMDATA_UNDERFLOW_CLEAR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DMDATA_UNDERFLOW_CLEAR {
DMDATA_DONT_CLEAR                        = 0x00000000,
DMDATA_CLEAR_UNDERFLOW_STATUS            = 0x00000001,
}


/*
 * DMDATA_UPDATED enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DMDATA_UPDATED {
DMDATA_NOT_UPDATED                       = 0x00000000,
DMDATA_WAS_UPDATED                       = 0x00000001,
}


/*******************************************************
 * HUBBUB_SDPIF Enums
 *******************************************************/

/*
 * RESPONSE_STATUS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RESPONSE_STATUS {
OKAY                                     = 0x00000000,
EXOKAY                                   = 0x00000001,
SLVERR                                   = 0x00000002,
DECERR                                   = 0x00000003,
EARLY                                    = 0x00000004,
OKAY_NODATA                              = 0x00000005,
PROTVIOL                                 = 0x00000006,
TRANSERR                                 = 0x00000007,
CMPTO                                    = 0x00000008,
CRS                                      = 0x0000000c,
}


/*******************************************************
 * HUBBUB_RET_PATH Enums
 *******************************************************/

/*
 * DCHUBBUB_DET_MEM_PWR_LIGHT_SLEEP_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCHUBBUB_DET_MEM_PWR_LIGHT_SLEEP_MODE {
DCHUBBUB_DET_MEM_POWER_LIGHT_SLEEP_MODE_OFF = 0x00000000,
DCHUBBUB_DET_MEM_POWER_LIGHT_SLEEP_MODE_1 = 0x00000001,
DCHUBBUB_DET_MEM_POWER_LIGHT_SLEEP_MODE_2 = 0x00000002,
}


/*
 * DCHUBBUB_MEM_PWR_DIS_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCHUBBUB_MEM_PWR_DIS_MODE {
DCHUBBUB_MEM_POWER_DIS_MODE_ENABLE       = 0x00000000,
DCHUBBUB_MEM_POWER_DIS_MODE_DISABLE      = 0x00000001,
}


/*
 * DCHUBBUB_MEM_PWR_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCHUBBUB_MEM_PWR_MODE {
DCHUBBUB_MEM_POWER_MODE_OFF              = 0x00000000,
DCHUBBUB_MEM_POWER_MODE_LIGHT_SLEEP      = 0x00000001,
DCHUBBUB_MEM_POWER_MODE_DEEP_SLEEP       = 0x00000002,
DCHUBBUB_MEM_POWER_MODE_SHUT_DOWN        = 0x00000003,
}


/*******************************************************
 * MPC_CFG Enums
 *******************************************************/

/*
 * MPC_CFG_ADR_CFG_CUR_VUPDATE_LOCK_SET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_CFG_ADR_CFG_CUR_VUPDATE_LOCK_SET {
MPC_CFG_ADR_CFG_CUR_VUPDATE_LOCK_SET_FALSE = 0x00000000,
MPC_CFG_ADR_CFG_CUR_VUPDATE_LOCK_SET_TRUE = 0x00000001,
}


/*
 * MPC_CFG_ADR_CFG_VUPDATE_LOCK_SET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_CFG_ADR_CFG_VUPDATE_LOCK_SET {
MPC_CFG_ADR_CFG_VUPDATE_LOCK_SET_FALSE   = 0x00000000,
MPC_CFG_ADR_CFG_VUPDATE_LOCK_SET_TRUE    = 0x00000001,
}


/*
 * MPC_CFG_ADR_VUPDATE_LOCK_SET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_CFG_ADR_VUPDATE_LOCK_SET {
MPC_CFG_ADR_VUPDATE_LOCK_SET_FALSE       = 0x00000000,
MPC_CFG_ADR_VUPDATE_LOCK_SET_TRUE        = 0x00000001,
}


/*
 * MPC_CFG_CFG_VUPDATE_LOCK_SET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_CFG_CFG_VUPDATE_LOCK_SET {
MPC_CFG_CFG_VUPDATE_LOCK_SET_FALSE       = 0x00000000,
MPC_CFG_CFG_VUPDATE_LOCK_SET_TRUE        = 0x00000001,
}


/*
 * MPC_CFG_CUR_VUPDATE_LOCK_SET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_CFG_CUR_VUPDATE_LOCK_SET {
MPC_CFG_CUR_VUPDATE_LOCK_SET_FALSE       = 0x00000000,
MPC_CFG_CUR_VUPDATE_LOCK_SET_TRUE        = 0x00000001,
}


/*
 * MPC_CFG_MPC_TEST_CLK_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_CFG_MPC_TEST_CLK_SEL {
MPC_CFG_MPC_TEST_CLK_SEL_0               = 0x00000000,
MPC_CFG_MPC_TEST_CLK_SEL_1               = 0x00000001,
MPC_CFG_MPC_TEST_CLK_SEL_2               = 0x00000002,
MPC_CFG_MPC_TEST_CLK_SEL_3               = 0x00000003,
}


/*
 * MPC_CFG_TEST_DEBUG_INDEX_MPC_CFG_TEST_DEBUG_WRITE_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_CFG_TEST_DEBUG_INDEX_MPC_CFG_TEST_DEBUG_WRITE_EN {
MPC_CFG_TEST_DEBUG_INDEX_MPC_CFG_TEST_DEBUG_WRITE_EN_FALSE = 0x00000000,
MPC_CFG_TEST_DEBUG_INDEX_MPC_CFG_TEST_DEBUG_WRITE_EN_TRUE = 0x00000001,
}


/*
 * MPC_CRC_CALC_INTERLACE_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_CRC_CALC_INTERLACE_MODE {
MPC_CRC_INTERLACE_MODE_TOP               = 0x00000000,
MPC_CRC_INTERLACE_MODE_BOTTOM            = 0x00000001,
MPC_CRC_INTERLACE_MODE_BOTH_RESET_BOTTOM = 0x00000002,
MPC_CRC_INTERLACE_MODE_BOTH_RESET_EACH   = 0x00000003,
}


/*
 * MPC_CRC_CALC_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_CRC_CALC_MODE {
MPC_CRC_ONE_SHOT_MODE                    = 0x00000000,
MPC_CRC_CONTINUOUS_MODE                  = 0x00000001,
}


/*
 * MPC_CRC_CALC_STEREO_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_CRC_CALC_STEREO_MODE {
MPC_CRC_STEREO_MODE_LEFT                 = 0x00000000,
MPC_CRC_STEREO_MODE_RIGHT                = 0x00000001,
MPC_CRC_STEREO_MODE_BOTH_RESET_RIGHT     = 0x00000002,
MPC_CRC_STEREO_MODE_BOTH_RESET_EACH      = 0x00000003,
}


/*
 * MPC_CRC_SOURCE_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_CRC_SOURCE_SELECT {
MPC_CRC_SOURCE_SEL_DPP                   = 0x00000000,
MPC_CRC_SOURCE_SEL_OPP                   = 0x00000001,
MPC_CRC_SOURCE_SEL_DWB                   = 0x00000002,
MPC_CRC_SOURCE_SEL_OTHER                 = 0x00000003,
}


/*
 * MPC_DEBUG_BUS1_DATA_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_DEBUG_BUS1_DATA_SELECT {
MPC_DEBUG_BUS1_DATA_SELECT_MPC_CFG       = 0x00000000,
MPC_DEBUG_BUS1_DATA_SELECT_MPC_CONT      = 0x00000001,
MPC_DEBUG_BUS1_DATA_SELECT_MPC_RSV1      = 0x00000002,
MPC_DEBUG_BUS1_DATA_SELECT_MPC_RSV       = 0x00000003,
}


/*
 * MPC_DEBUG_BUS2_DATA_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_DEBUG_BUS2_DATA_SELECT {
MPC_DEBUG_BUS2_DATA_SELECT_MPCC          = 0x00000000,
MPC_DEBUG_BUS2_DATA_SELECT_MPCC_CONT     = 0x00000001,
MPC_DEBUG_BUS2_DATA_SELECT_MPCC_MCM      = 0x00000002,
MPC_DEBUG_BUS2_DATA_SELECT_RES           = 0x00000003,
}


/*
 * MPC_DEBUG_BUS_DIRECT_OUT_DATA_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_DEBUG_BUS_DIRECT_OUT_DATA_SELECT {
MPC_DEBUG_BUS_DIRECT_OUT_DATA_SELECT_MPC_DEBUG_ID = 0x00000000,
MPC_DEBUG_BUS_DIRECT_OUT_DATA_SELECT_MPCC_DEBUG_ID = 0x00000001,
MPC_DEBUG_BUS_DIRECT_OUT_DATA_SELECT_MPCC_OGAM_DEBUG_ID = 0x00000002,
MPC_DEBUG_BUS_DIRECT_OUT_DATA_SELECT_MPC_OCSC_DEBUG_ID = 0x00000003,
MPC_DEBUG_BUS_DIRECT_OUT_DATA_SELECT_SFR_DEBUG_DATA = 0x00000004,
MPC_DEBUG_BUS_DIRECT_OUT_DATA_SELECT_SFT_DEBUG_DATA = 0x00000005,
MPC_DEBUG_BUS_DIRECT_OUT_DATA_SELECT_RSV1 = 0x00000006,
MPC_DEBUG_BUS_DIRECT_OUT_DATA_SELECT_MPCC_MCM_DEBUG_ID = 0x00000007,
}


/*
 * MPC_DEBUG_BUS_MPCC_BYTE_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_DEBUG_BUS_MPCC_BYTE_SELECT {
MPC_DEBUG_BUS_MPCC_BYTE0                 = 0x00000000,
MPC_DEBUG_BUS_MPCC_BYTE1                 = 0x00000001,
MPC_DEBUG_BUS_MPCC_BYTE2                 = 0x00000002,
MPC_DEBUG_BUS_MPCC_BYTE3                 = 0x00000003,
}


/*******************************************************
 * MPC_OCSC Enums
 *******************************************************/

/*
 * MPC_OCSC_COEF_FORMAT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_OCSC_COEF_FORMAT {
MPC_OCSC_COEF_FORMAT_S2_13               = 0x00000000,
MPC_OCSC_COEF_FORMAT_S3_12               = 0x00000001,
}


/*
 * MPC_OCSC_TEST_DEBUG_INDEX_MPC_OCSC_TEST_DEBUG_WRITE_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_OCSC_TEST_DEBUG_INDEX_MPC_OCSC_TEST_DEBUG_WRITE_EN {
MPC_OCSC_TEST_DEBUG_INDEX_MPC_OCSC_TEST_DEBUG_WRITE_EN_FALSE = 0x00000000,
MPC_OCSC_TEST_DEBUG_INDEX_MPC_OCSC_TEST_DEBUG_WRITE_EN_TRUE = 0x00000001,
}


/*
 * MPC_OUT_CSC_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_OUT_CSC_MODE {
MPC_OUT_CSC_MODE_0                       = 0x00000000,
MPC_OUT_CSC_MODE_1                       = 0x00000001,
MPC_OUT_CSC_MODE_2                       = 0x00000002,
MPC_OUT_CSC_MODE_RSV                     = 0x00000003,
}


/*
 * MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_MODE {
MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_BYPASS = 0x00000000,
MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_6BITS = 0x00000001,
MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_8BITS = 0x00000002,
MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_9BITS = 0x00000003,
MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_10BITS = 0x00000004,
MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_11BITS = 0x00000005,
MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_12BITS = 0x00000006,
MPC_OUT_DENORM_CONTROL_MPC_OUT_DENORM_PASSTHROUGH = 0x00000007,
}


/*
 * MPC_OUT_RATE_CONTROL_DISABLE_SET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPC_OUT_RATE_CONTROL_DISABLE_SET {
MPC_OUT_RATE_CONTROL_SET_ENABLE          = 0x00000000,
MPC_OUT_RATE_CONTROL_SET_DISABLE         = 0x00000001,
}


/*******************************************************
 * MPCC Enums
 *******************************************************/

/*
 * MPCC_BG_COLOR_BPC enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_BG_COLOR_BPC {
MPCC_BG_COLOR_BPC_8bit                   = 0x00000000,
MPCC_BG_COLOR_BPC_9bit                   = 0x00000001,
MPCC_BG_COLOR_BPC_10bit                  = 0x00000002,
MPCC_BG_COLOR_BPC_11bit                  = 0x00000003,
MPCC_BG_COLOR_BPC_12bit                  = 0x00000004,
}


/*
 * MPCC_CONTROL_MPCC_ACTIVE_OVERLAP_ONLY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_CONTROL_MPCC_ACTIVE_OVERLAP_ONLY {
MPCC_CONTROL_MPCC_ACTIVE_OVERLAP_ONLY_FALSE = 0x00000000,
MPCC_CONTROL_MPCC_ACTIVE_OVERLAP_ONLY_TRUE = 0x00000001,
}


/*
 * MPCC_CONTROL_MPCC_ALPHA_BLND_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_CONTROL_MPCC_ALPHA_BLND_MODE {
MPCC_CONTROL_MPCC_ALPHA_BLND_MODE_PER_PIXEL_ALPHA = 0x00000000,
MPCC_CONTROL_MPCC_ALPHA_BLND_MODE_PER_PIXEL_ALPHA_COMBINED_GLOBAL_GAIN = 0x00000001,
MPCC_CONTROL_MPCC_ALPHA_BLND_MODE_GLOBAL_ALPHA = 0x00000002,
MPCC_CONTROL_MPCC_ALPHA_BLND_MODE_UNUSED = 0x00000003,
}


/*
 * MPCC_CONTROL_MPCC_ALPHA_MULTIPLIED_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_CONTROL_MPCC_ALPHA_MULTIPLIED_MODE {
MPCC_CONTROL_MPCC_ALPHA_MULTIPLIED_MODE_FALSE = 0x00000000,
MPCC_CONTROL_MPCC_ALPHA_MULTIPLIED_MODE_TRUE = 0x00000001,
}


/*
 * MPCC_CONTROL_MPCC_BOT_GAIN_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_CONTROL_MPCC_BOT_GAIN_MODE {
MPCC_CONTROL_MPCC_BOT_GAIN_MODE_0        = 0x00000000,
MPCC_CONTROL_MPCC_BOT_GAIN_MODE_1        = 0x00000001,
}


/*
 * MPCC_CONTROL_MPCC_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_CONTROL_MPCC_MODE {
MPCC_CONTROL_MPCC_MODE_BYPASS            = 0x00000000,
MPCC_CONTROL_MPCC_MODE_TOP_LAYER_PASSTHROUGH = 0x00000001,
MPCC_CONTROL_MPCC_MODE_TOP_LAYER_ONLY    = 0x00000002,
MPCC_CONTROL_MPCC_MODE_TOP_BOT_BLENDING  = 0x00000003,
}


/*
 * MPCC_SM_CONTROL_MPCC_SM_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_SM_CONTROL_MPCC_SM_EN {
MPCC_SM_CONTROL_MPCC_SM_EN_FALSE         = 0x00000000,
MPCC_SM_CONTROL_MPCC_SM_EN_TRUE          = 0x00000001,
}


/*
 * MPCC_SM_CONTROL_MPCC_SM_FIELD_ALT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_SM_CONTROL_MPCC_SM_FIELD_ALT {
MPCC_SM_CONTROL_MPCC_SM_FIELD_ALT_FALSE  = 0x00000000,
MPCC_SM_CONTROL_MPCC_SM_FIELD_ALT_TRUE   = 0x00000001,
}


/*
 * MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_FRAME_POL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_FRAME_POL {
MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_FRAME_POL_NO_FORCE = 0x00000000,
MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_FRAME_POL_RESERVED = 0x00000001,
MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_FRAME_POL_FORCE_LOW = 0x00000002,
MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_FRAME_POL_FORCE_HIGH = 0x00000003,
}


/*
 * MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_TOP_POL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_TOP_POL {
MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_TOP_POL_NO_FORCE = 0x00000000,
MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_TOP_POL_RESERVED = 0x00000001,
MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_TOP_POL_FORCE_LOW = 0x00000002,
MPCC_SM_CONTROL_MPCC_SM_FORCE_NEXT_TOP_POL_FORCE_HIGH = 0x00000003,
}


/*
 * MPCC_SM_CONTROL_MPCC_SM_FRAME_ALT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_SM_CONTROL_MPCC_SM_FRAME_ALT {
MPCC_SM_CONTROL_MPCC_SM_FRAME_ALT_FALSE  = 0x00000000,
MPCC_SM_CONTROL_MPCC_SM_FRAME_ALT_TRUE   = 0x00000001,
}


/*
 * MPCC_SM_CONTROL_MPCC_SM_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_SM_CONTROL_MPCC_SM_MODE {
MPCC_SM_CONTROL_MPCC_SM_MODE_SINGLE_PLANE = 0x00000000,
MPCC_SM_CONTROL_MPCC_SM_MODE_ROW_SUBSAMPLING = 0x00000002,
MPCC_SM_CONTROL_MPCC_SM_MODE_COLUMN_SUBSAMPLING = 0x00000004,
MPCC_SM_CONTROL_MPCC_SM_MODE_CHECKERBOARD_SUBSAMPLING = 0x00000006,
}


/*
 * MPCC_TEST_DEBUG_INDEX_MPCC_TEST_DEBUG_WRITE_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_TEST_DEBUG_INDEX_MPCC_TEST_DEBUG_WRITE_EN {
MPCC_TEST_DEBUG_INDEX_MPCC_TEST_DEBUG_WRITE_EN_FALSE = 0x00000000,
MPCC_TEST_DEBUG_INDEX_MPCC_TEST_DEBUG_WRITE_EN_TRUE = 0x00000001,
}


/*******************************************************
 * MPCC_OGAM Enums
 *******************************************************/

/*
 * MPCC_GAMUT_REMAP_COEF_FORMAT_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_GAMUT_REMAP_COEF_FORMAT_ENUM {
MPCC_GAMUT_REMAP_COEF_FORMAT_S2_13       = 0x00000000,
MPCC_GAMUT_REMAP_COEF_FORMAT_S3_12       = 0x00000001,
}


/*
 * MPCC_GAMUT_REMAP_MODE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_GAMUT_REMAP_MODE_ENUM {
MPCC_GAMUT_REMAP_MODE_0                  = 0x00000000,
MPCC_GAMUT_REMAP_MODE_1                  = 0x00000001,
MPCC_GAMUT_REMAP_MODE_2                  = 0x00000002,
MPCC_GAMUT_REMAP_MODE_RSV                = 0x00000003,
}


/*
 * MPCC_OGAM_LUT_2_CONFIG_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_OGAM_LUT_2_CONFIG_ENUM {
MPCC_OGAM_LUT_2CFG_NO_MEMORY             = 0x00000000,
MPCC_OGAM_LUT_2CFG_MEMORY_A              = 0x00000001,
MPCC_OGAM_LUT_2CFG_MEMORY_B              = 0x00000002,
}


/*
 * MPCC_OGAM_LUT_CONFIG_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_OGAM_LUT_CONFIG_MODE {
MPCC_OGAM_DIFFERENT_RGB                  = 0x00000000,
MPCC_OGAM_ALL_USE_R                      = 0x00000001,
}


/*
 * MPCC_OGAM_LUT_PWL_DISABLE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_OGAM_LUT_PWL_DISABLE_ENUM {
MPCC_OGAM_ENABLE_PWL                     = 0x00000000,
MPCC_OGAM_DISABLE_PWL                    = 0x00000001,
}


/*
 * MPCC_OGAM_LUT_RAM_CONTROL_MPCC_OGAM_LUT_RAM_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_OGAM_LUT_RAM_CONTROL_MPCC_OGAM_LUT_RAM_SEL {
MPCC_OGAM_LUT_RAM_CONTROL_MPCC_OGAM_LUT_RAM_SEL_RAMA = 0x00000000,
MPCC_OGAM_LUT_RAM_CONTROL_MPCC_OGAM_LUT_RAM_SEL_RAMB = 0x00000001,
}


/*
 * MPCC_OGAM_LUT_RAM_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_OGAM_LUT_RAM_SEL {
MPCC_OGAM_RAMA_ACCESS                    = 0x00000000,
MPCC_OGAM_RAMB_ACCESS                    = 0x00000001,
}


/*
 * MPCC_OGAM_LUT_READ_COLOR_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_OGAM_LUT_READ_COLOR_SEL {
MPCC_OGAM_BLUE_LUT                       = 0x00000000,
MPCC_OGAM_GREEN_LUT                      = 0x00000001,
MPCC_OGAM_RED_LUT                        = 0x00000002,
}


/*
 * MPCC_OGAM_LUT_READ_DBG enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_OGAM_LUT_READ_DBG {
MPCC_OGAM_DISABLE_DEBUG                  = 0x00000000,
MPCC_OGAM_ENABLE_DEBUG                   = 0x00000001,
}


/*
 * MPCC_OGAM_LUT_SEL_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_OGAM_LUT_SEL_ENUM {
MPCC_OGAM_RAMA                           = 0x00000000,
MPCC_OGAM_RAMB                           = 0x00000001,
}


/*
 * MPCC_OGAM_MODE_MPCC_OGAM_MODE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_OGAM_MODE_MPCC_OGAM_MODE_ENUM {
MPCC_OGAM_MODE_0                         = 0x00000000,
MPCC_OGAM_MODE_RSV1                      = 0x00000001,
MPCC_OGAM_MODE_2                         = 0x00000002,
MPCC_OGAM_MODE_RSV                       = 0x00000003,
}


/*
 * MPCC_OGAM_NUM_SEG enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_OGAM_NUM_SEG {
MPCC_OGAM_SEGMENTS_1                     = 0x00000000,
MPCC_OGAM_SEGMENTS_2                     = 0x00000001,
MPCC_OGAM_SEGMENTS_4                     = 0x00000002,
MPCC_OGAM_SEGMENTS_8                     = 0x00000003,
MPCC_OGAM_SEGMENTS_16                    = 0x00000004,
MPCC_OGAM_SEGMENTS_32                    = 0x00000005,
MPCC_OGAM_SEGMENTS_64                    = 0x00000006,
MPCC_OGAM_SEGMENTS_128                   = 0x00000007,
}


/*
 * MPCC_OGAM_TEST_DEBUG_INDEX_MPCC_OGAM_TEST_DEBUG_WRITE_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_OGAM_TEST_DEBUG_INDEX_MPCC_OGAM_TEST_DEBUG_WRITE_EN {
MPCC_OGAM_TEST_DEBUG_INDEX_MPCC_OGAM_TEST_DEBUG_WRITE_EN_FALSE = 0x00000000,
MPCC_OGAM_TEST_DEBUG_INDEX_MPCC_OGAM_TEST_DEBUG_WRITE_EN_TRUE = 0x00000001,
}


/*******************************************************
 * MPCC_MCM Enums
 *******************************************************/

/*
 * MPCC_MCM_3DLUT_30BIT_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_MCM_3DLUT_30BIT_ENUM {
MPCC_MCM_3DLUT_36BIT                     = 0x00000000,
MPCC_MCM_3DLUT_30BIT                     = 0x00000001,
}


/*
 * MPCC_MCM_3DLUT_RAM_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_MCM_3DLUT_RAM_SEL {
MPCC_MCM_RAM0_ACCESS                     = 0x00000000,
MPCC_MCM_RAM1_ACCESS                     = 0x00000001,
MPCC_MCM_RAM2_ACCESS                     = 0x00000002,
MPCC_MCM_RAM3_ACCESS                     = 0x00000003,
}


/*
 * MPCC_MCM_3DLUT_SIZE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_MCM_3DLUT_SIZE_ENUM {
MPCC_MCM_3DLUT_17CUBE                    = 0x00000000,
MPCC_MCM_3DLUT_9CUBE                     = 0x00000001,
}


/*
 * MPCC_MCM_GAMMA_LUT_MODE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_MCM_GAMMA_LUT_MODE_ENUM {
MPCC_MCM_GAMMA_LUT_BYPASS                = 0x00000000,
MPCC_MCM_GAMMA_LUT_RESERVED_1            = 0x00000001,
MPCC_MCM_GAMMA_LUT_RAM_LUT               = 0x00000002,
MPCC_MCM_GAMMA_LUT_RESERVED_3            = 0x00000003,
}


/*
 * MPCC_MCM_GAMMA_LUT_PWL_DISABLE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_MCM_GAMMA_LUT_PWL_DISABLE_ENUM {
MPCC_MCM_GAMMA_LUT_ENABLE_PWL            = 0x00000000,
MPCC_MCM_GAMMA_LUT_DISABLE_PWL           = 0x00000001,
}


/*
 * MPCC_MCM_GAMMA_LUT_SEL_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_MCM_GAMMA_LUT_SEL_ENUM {
MPCC_MCM_GAMMA_LUT_RAMA                  = 0x00000000,
MPCC_MCM_GAMMA_LUT_RAMB                  = 0x00000001,
}


/*
 * MPCC_MCM_LUT_2_MODE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_MCM_LUT_2_MODE_ENUM {
MPCC_MCM_LUT_2_MODE_BYPASS               = 0x00000000,
MPCC_MCM_LUT_2_MODE_RAMA_LUT             = 0x00000001,
MPCC_MCM_LUT_2_MODE_RAMB_LUT             = 0x00000002,
}


/*
 * MPCC_MCM_LUT_CONFIG_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_MCM_LUT_CONFIG_MODE {
MPCC_MCM_LUT_DIFFERENT_RGB               = 0x00000000,
MPCC_MCM_LUT_ALL_USE_R                   = 0x00000001,
}


/*
 * MPCC_MCM_LUT_NUM_SEG enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_MCM_LUT_NUM_SEG {
MPCC_MCM_LUT_SEGMENTS_1                  = 0x00000000,
MPCC_MCM_LUT_SEGMENTS_2                  = 0x00000001,
MPCC_MCM_LUT_SEGMENTS_4                  = 0x00000002,
MPCC_MCM_LUT_SEGMENTS_8                  = 0x00000003,
MPCC_MCM_LUT_SEGMENTS_16                 = 0x00000004,
MPCC_MCM_LUT_SEGMENTS_32                 = 0x00000005,
MPCC_MCM_LUT_SEGMENTS_64                 = 0x00000006,
MPCC_MCM_LUT_SEGMENTS_128                = 0x00000007,
}


/*
 * MPCC_MCM_LUT_RAM_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_MCM_LUT_RAM_SEL {
MPCC_MCM_LUT_RAMA_ACCESS                 = 0x00000000,
MPCC_MCM_LUT_RAMB_ACCESS                 = 0x00000001,
}


/*
 * MPCC_MCM_LUT_READ_COLOR_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_MCM_LUT_READ_COLOR_SEL {
MPCC_MCM_LUT_BLUE_LUT                    = 0x00000000,
MPCC_MCM_LUT_GREEN_LUT                   = 0x00000001,
MPCC_MCM_LUT_RED_LUT                     = 0x00000002,
}


/*
 * MPCC_MCM_LUT_READ_DBG enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_MCM_LUT_READ_DBG {
MPCC_MCM_LUT_DISABLE_DEBUG               = 0x00000000,
MPCC_MCM_LUT_ENABLE_DEBUG                = 0x00000001,
}


/*
 * MPCC_MCM_MEM_PWR_FORCE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_MCM_MEM_PWR_FORCE_ENUM {
MPCC_MCM_MEM_PWR_FORCE_DIS               = 0x00000000,
MPCC_MCM_MEM_PWR_FORCE_LS                = 0x00000001,
MPCC_MCM_MEM_PWR_FORCE_DS                = 0x00000002,
MPCC_MCM_MEM_PWR_FORCE_SD                = 0x00000003,
}


/*
 * MPCC_MCM_MEM_PWR_STATE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MPCC_MCM_MEM_PWR_STATE_ENUM {
MPCC_MCM_MEM_PWR_STATE_ON                = 0x00000000,
MPCC_MCM_MEM_PWR_STATE_LS                = 0x00000001,
MPCC_MCM_MEM_PWR_STATE_DS                = 0x00000002,
MPCC_MCM_MEM_PWR_STATE_SD                = 0x00000003,
}


/*******************************************************
 * ABM Enums
 *******************************************************/

/*******************************************************
 * DPG Enums
 *******************************************************/

/*
 * ENUM_DPG_BIT_DEPTH enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DPG_BIT_DEPTH {
ENUM_DPG_BIT_DEPTH_6BPC                  = 0x00000000,
ENUM_DPG_BIT_DEPTH_8BPC                  = 0x00000001,
ENUM_DPG_BIT_DEPTH_10BPC                 = 0x00000002,
ENUM_DPG_BIT_DEPTH_12BPC                 = 0x00000003,
}


/*
 * ENUM_DPG_DYNAMIC_RANGE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DPG_DYNAMIC_RANGE {
ENUM_DPG_DYNAMIC_RANGE_VESA              = 0x00000000,
ENUM_DPG_DYNAMIC_RANGE_CEA               = 0x00000001,
}


/*
 * ENUM_DPG_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DPG_EN {
ENUM_DPG_DISABLE                         = 0x00000000,
ENUM_DPG_ENABLE                          = 0x00000001,
}


/*
 * ENUM_DPG_FIELD_POLARITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DPG_FIELD_POLARITY {
ENUM_DPG_FIELD_POLARITY_TOP_EVEN_BOTTOM_ODD = 0x00000000,
ENUM_DPG_FIELD_POLARITY_TOP_ODD_BOTTOM_EVEN = 0x00000001,
}


/*
 * ENUM_DPG_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DPG_MODE {
ENUM_DPG_MODE_RGB_COLOUR_BLOCK           = 0x00000000,
ENUM_DPG_MODE_YCBCR_601_COLOUR_BLOCK     = 0x00000001,
ENUM_DPG_MODE_YCBCR_709_COLOUR_BLOCK     = 0x00000002,
ENUM_DPG_MODE_VERTICAL_BAR               = 0x00000003,
ENUM_DPG_MODE_HORIZONTAL_BAR             = 0x00000004,
ENUM_DPG_MODE_RGB_SINGLE_RAMP            = 0x00000005,
ENUM_DPG_MODE_RGB_DUAL_RAMP              = 0x00000006,
ENUM_DPG_MODE_RGB_XR_BIAS                = 0x00000007,
}


/*******************************************************
 * FMT Enums
 *******************************************************/

/*
 * FMTMEM_PWR_DIS_CTRL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMTMEM_PWR_DIS_CTRL {
FMTMEM_ENABLE_MEM_PWR_CTRL               = 0x00000000,
FMTMEM_DISABLE_MEM_PWR_CTRL              = 0x00000001,
}


/*
 * FMTMEM_PWR_FORCE_CTRL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMTMEM_PWR_FORCE_CTRL {
FMTMEM_NO_FORCE_REQUEST                  = 0x00000000,
FMTMEM_FORCE_LIGHT_SLEEP_REQUEST         = 0x00000001,
FMTMEM_FORCE_DEEP_SLEEP_REQUEST          = 0x00000002,
FMTMEM_FORCE_SHUT_DOWN_REQUEST           = 0x00000003,
}


/*
 * FMT_BIT_DEPTH_CONTROL_25FRC_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_BIT_DEPTH_CONTROL_25FRC_SEL {
FMT_BIT_DEPTH_CONTROL_25FRC_SEL_Ei       = 0x00000000,
FMT_BIT_DEPTH_CONTROL_25FRC_SEL_Fi       = 0x00000001,
FMT_BIT_DEPTH_CONTROL_25FRC_SEL_Gi       = 0x00000002,
FMT_BIT_DEPTH_CONTROL_25FRC_SEL_RESERVED = 0x00000003,
}


/*
 * FMT_BIT_DEPTH_CONTROL_50FRC_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_BIT_DEPTH_CONTROL_50FRC_SEL {
FMT_BIT_DEPTH_CONTROL_50FRC_SEL_A        = 0x00000000,
FMT_BIT_DEPTH_CONTROL_50FRC_SEL_B        = 0x00000001,
FMT_BIT_DEPTH_CONTROL_50FRC_SEL_C        = 0x00000002,
FMT_BIT_DEPTH_CONTROL_50FRC_SEL_D        = 0x00000003,
}


/*
 * FMT_BIT_DEPTH_CONTROL_75FRC_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_BIT_DEPTH_CONTROL_75FRC_SEL {
FMT_BIT_DEPTH_CONTROL_75FRC_SEL_E        = 0x00000000,
FMT_BIT_DEPTH_CONTROL_75FRC_SEL_F        = 0x00000001,
FMT_BIT_DEPTH_CONTROL_75FRC_SEL_G        = 0x00000002,
FMT_BIT_DEPTH_CONTROL_75FRC_SEL_RESERVED = 0x00000003,
}


/*
 * FMT_BIT_DEPTH_CONTROL_SPATIAL_DITHER_DEPTH enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_BIT_DEPTH_CONTROL_SPATIAL_DITHER_DEPTH {
FMT_BIT_DEPTH_CONTROL_SPATIAL_DITHER_DEPTH_18BPP = 0x00000000,
FMT_BIT_DEPTH_CONTROL_SPATIAL_DITHER_DEPTH_24BPP = 0x00000001,
FMT_BIT_DEPTH_CONTROL_SPATIAL_DITHER_DEPTH_30BPP = 0x00000002,
}


/*
 * FMT_BIT_DEPTH_CONTROL_TEMPORAL_DITHER_DEPTH enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_BIT_DEPTH_CONTROL_TEMPORAL_DITHER_DEPTH {
FMT_BIT_DEPTH_CONTROL_TEMPORAL_DITHER_DEPTH_18BPP = 0x00000000,
FMT_BIT_DEPTH_CONTROL_TEMPORAL_DITHER_DEPTH_24BPP = 0x00000001,
FMT_BIT_DEPTH_CONTROL_TEMPORAL_DITHER_DEPTH_30BPP = 0x00000002,
}


/*
 * FMT_BIT_DEPTH_CONTROL_TEMPORAL_LEVEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_BIT_DEPTH_CONTROL_TEMPORAL_LEVEL {
FMT_BIT_DEPTH_CONTROL_TEMPORAL_LEVEL_GREY_LEVEL2 = 0x00000000,
FMT_BIT_DEPTH_CONTROL_TEMPORAL_LEVEL_GREY_LEVEL4 = 0x00000001,
}


/*
 * FMT_BIT_DEPTH_CONTROL_TRUNCATE_DEPTH enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_BIT_DEPTH_CONTROL_TRUNCATE_DEPTH {
FMT_BIT_DEPTH_CONTROL_TRUNCATE_DEPTH_18BPP = 0x00000000,
FMT_BIT_DEPTH_CONTROL_TRUNCATE_DEPTH_24BPP = 0x00000001,
FMT_BIT_DEPTH_CONTROL_TRUNCATE_DEPTH_30BPP = 0x00000002,
}


/*
 * FMT_BIT_DEPTH_CONTROL_TRUNCATE_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_BIT_DEPTH_CONTROL_TRUNCATE_MODE {
FMT_BIT_DEPTH_CONTROL_TRUNCATE_MODE_TRUNCATION = 0x00000000,
FMT_BIT_DEPTH_CONTROL_TRUNCATE_MODE_ROUNDING = 0x00000001,
}


/*
 * FMT_CLAMP_CNTL_COLOR_FORMAT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_CLAMP_CNTL_COLOR_FORMAT {
FMT_CLAMP_CNTL_COLOR_FORMAT_6BPC         = 0x00000000,
FMT_CLAMP_CNTL_COLOR_FORMAT_8BPC         = 0x00000001,
FMT_CLAMP_CNTL_COLOR_FORMAT_10BPC        = 0x00000002,
FMT_CLAMP_CNTL_COLOR_FORMAT_12BPC        = 0x00000003,
FMT_CLAMP_CNTL_COLOR_FORMAT_RESERVED1    = 0x00000004,
FMT_CLAMP_CNTL_COLOR_FORMAT_RESERVED2    = 0x00000005,
FMT_CLAMP_CNTL_COLOR_FORMAT_RESERVED3    = 0x00000006,
FMT_CLAMP_CNTL_COLOR_FORMAT_PROGRAMMABLE = 0x00000007,
}


/*
 * FMT_CONTROL_CBCR_BIT_REDUCTION_BYPASS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_CONTROL_CBCR_BIT_REDUCTION_BYPASS {
FMT_CONTROL_CBCR_BIT_REDUCTION_BYPASS_DISABLE = 0x00000000,
FMT_CONTROL_CBCR_BIT_REDUCTION_BYPASS_ENABLE = 0x00000001,
}


/*
 * FMT_CONTROL_PIXEL_ENCODING enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_CONTROL_PIXEL_ENCODING {
FMT_CONTROL_PIXEL_ENCODING_RGB444_OR_YCBCR444 = 0x00000000,
FMT_CONTROL_PIXEL_ENCODING_YCBCR422      = 0x00000001,
FMT_CONTROL_PIXEL_ENCODING_YCBCR420      = 0x00000002,
FMT_CONTROL_PIXEL_ENCODING_RESERVED      = 0x00000003,
}


/*
 * FMT_CONTROL_SUBSAMPLING_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_CONTROL_SUBSAMPLING_MODE {
FMT_CONTROL_SUBSAMPLING_MODE_DROP        = 0x00000000,
FMT_CONTROL_SUBSAMPLING_MODE_AVERAGE     = 0x00000001,
FMT_CONTROL_SUBSAMPLING_MOME_3_TAP       = 0x00000002,
FMT_CONTROL_SUBSAMPLING_MOME_RESERVED    = 0x00000003,
}


/*
 * FMT_CONTROL_SUBSAMPLING_ORDER enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_CONTROL_SUBSAMPLING_ORDER {
FMT_CONTROL_SUBSAMPLING_ORDER_CB_BEFORE_CR = 0x00000000,
FMT_CONTROL_SUBSAMPLING_ORDER_CR_BEFORE_CB = 0x00000001,
}


/*
 * FMT_DEBUG_CNTL_COLOR_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_DEBUG_CNTL_COLOR_SELECT {
FMT_DEBUG_CNTL_COLOR_SELECT_BLUE         = 0x00000000,
FMT_DEBUG_CNTL_COLOR_SELECT_GREEN        = 0x00000001,
FMT_DEBUG_CNTL_COLOR_SELECT_RED1         = 0x00000002,
FMT_DEBUG_CNTL_COLOR_SELECT_RED2         = 0x00000003,
}


/*
 * FMT_DYNAMIC_EXP_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_DYNAMIC_EXP_MODE {
FMT_DYNAMIC_EXP_MODE_10to12              = 0x00000000,
FMT_DYNAMIC_EXP_MODE_8to12               = 0x00000001,
}


/*
 * FMT_FRAME_RANDOM_ENABLE_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_FRAME_RANDOM_ENABLE_CONTROL {
FMT_FRAME_RANDOM_ENABLE_RESET_EACH_FRAME = 0x00000000,
FMT_FRAME_RANDOM_ENABLE_RESET_ONCE       = 0x00000001,
}


/*
 * FMT_POWER_STATE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_POWER_STATE_ENUM {
FMT_POWER_STATE_ENUM_ON                  = 0x00000000,
FMT_POWER_STATE_ENUM_LS                  = 0x00000001,
FMT_POWER_STATE_ENUM_DS                  = 0x00000002,
FMT_POWER_STATE_ENUM_SD                  = 0x00000003,
}


/*
 * FMT_RGB_RANDOM_ENABLE_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_RGB_RANDOM_ENABLE_CONTROL {
FMT_RGB_RANDOM_ENABLE_CONTROL_DISABLE    = 0x00000000,
FMT_RGB_RANDOM_ENABLE_CONTROL_ENABLE     = 0x00000001,
}


/*
 * FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP_CONTROL {
FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP_NO_SWAP = 0x00000000,
FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP_1 = 0x00000001,
FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP_2 = 0x00000002,
FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP_RESERVED = 0x00000003,
}


/*
 * FMT_SPATIAL_DITHER_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_SPATIAL_DITHER_MODE {
FMT_SPATIAL_DITHER_MODE_0                = 0x00000000,
FMT_SPATIAL_DITHER_MODE_1                = 0x00000001,
FMT_SPATIAL_DITHER_MODE_2                = 0x00000002,
FMT_SPATIAL_DITHER_MODE_3                = 0x00000003,
}


/*
 * FMT_STEREOSYNC_OVERRIDE_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_STEREOSYNC_OVERRIDE_CONTROL {
FMT_STEREOSYNC_OVERRIDE_CONTROL_0        = 0x00000000,
FMT_STEREOSYNC_OVERRIDE_CONTROL_1        = 0x00000001,
}


/*
 * FMT_TEMPORAL_DITHER_PATTERN_CONTROL_RGB1_BGR0 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FMT_TEMPORAL_DITHER_PATTERN_CONTROL_RGB1_BGR0 {
FMT_TEMPORAL_DITHER_PATTERN_CONTROL_RGB1_BGR0_BGR = 0x00000000,
FMT_TEMPORAL_DITHER_PATTERN_CONTROL_RGB1_BGR0_RGB = 0x00000001,
}


/*******************************************************
 * OPPBUF Enums
 *******************************************************/

/*
 * OPPBUF_DISPLAY_SEGMENTATION enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPPBUF_DISPLAY_SEGMENTATION {
OPPBUF_DISPLAY_SEGMENTATION_1_SEGMENT    = 0x00000000,
OPPBUF_DISPLAY_SEGMENTATION_2_SEGMENT    = 0x00000001,
OPPBUF_DISPLAY_SEGMENTATION_4_SEGMENT    = 0x00000002,
OPPBUF_DISPLAY_SEGMENTATION_4_SEGMENT_SPLIT_LEFT = 0x00000003,
OPPBUF_DISPLAY_SEGMENTATION_4_SEGMENT_SPLIT_RIGHT = 0x00000004,
}


/*******************************************************
 * OPP_PIPE Enums
 *******************************************************/

/*
 * OPP_PIPE_CLOCK_ENABLE_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_PIPE_CLOCK_ENABLE_CONTROL {
OPP_PIPE_CLOCK_DISABLE                   = 0x00000000,
OPP_PIPE_CLOCK_ENABLE                    = 0x00000001,
}


/*
 * OPP_PIPE_DIGTIAL_BYPASS_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_PIPE_DIGTIAL_BYPASS_CONTROL {
OPP_PIPE_DIGTIAL_BYPASS_DISABLE          = 0x00000000,
OPP_PIPE_DIGTIAL_BYPASS_ENABLE           = 0x00000001,
}


/*******************************************************
 * OPP_PIPE_CRC Enums
 *******************************************************/

/*
 * OPP_PIPE_CRC_CONT_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_PIPE_CRC_CONT_EN {
OPP_PIPE_CRC_MODE_ONE_SHOT               = 0x00000000,
OPP_PIPE_CRC_MODE_CONTINUOUS             = 0x00000001,
}


/*
 * OPP_PIPE_CRC_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_PIPE_CRC_EN {
OPP_PIPE_CRC_DISABLE                     = 0x00000000,
OPP_PIPE_CRC_ENABLE                      = 0x00000001,
}


/*
 * OPP_PIPE_CRC_INTERLACE_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_PIPE_CRC_INTERLACE_EN {
OPP_PIPE_CRC_INTERLACE_EN_INTERPRET_AS_PROGRESSIVE = 0x00000000,
OPP_PIPE_CRC_INTERLACE_EN_INTERPRET_AS_INTERLACED = 0x00000001,
}


/*
 * OPP_PIPE_CRC_INTERLACE_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_PIPE_CRC_INTERLACE_MODE {
OPP_PIPE_CRC_INTERLACE_MODE_TOP          = 0x00000000,
OPP_PIPE_CRC_INTERLACE_MODE_BOTTOM       = 0x00000001,
OPP_PIPE_CRC_INTERLACE_MODE_BOTH_RESET_AFTER_BOTTOM_FIELD = 0x00000002,
OPP_PIPE_CRC_INTERLACE_MODE_BOTH_RESET_AFTER_EACH_FIELD = 0x00000003,
}


/*
 * OPP_PIPE_CRC_ONE_SHOT_PENDING enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_PIPE_CRC_ONE_SHOT_PENDING {
OPP_PIPE_CRC_ONE_SHOT_PENDING_NOT_PENDING = 0x00000000,
OPP_PIPE_CRC_ONE_SHOT_PENDING_PENDING    = 0x00000001,
}


/*
 * OPP_PIPE_CRC_PIXEL_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_PIPE_CRC_PIXEL_SELECT {
OPP_PIPE_CRC_PIXEL_SELECT_ALL_PIXELS     = 0x00000000,
OPP_PIPE_CRC_PIXEL_SELECT_RESERVED       = 0x00000001,
OPP_PIPE_CRC_PIXEL_SELECT_EVEN_PIXELS    = 0x00000002,
OPP_PIPE_CRC_PIXEL_SELECT_ODD_PIXELS     = 0x00000003,
}


/*
 * OPP_PIPE_CRC_SOURCE_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_PIPE_CRC_SOURCE_SELECT {
OPP_PIPE_CRC_SOURCE_SELECT_FMT           = 0x00000000,
OPP_PIPE_CRC_SOURCE_SELECT_SFT           = 0x00000001,
}


/*
 * OPP_PIPE_CRC_STEREO_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_PIPE_CRC_STEREO_EN {
OPP_PIPE_CRC_STEREO_EN_INTERPRET_AS_NON_STEREO = 0x00000000,
OPP_PIPE_CRC_STEREO_EN_INTERPRET_AS_STEREO = 0x00000001,
}


/*
 * OPP_PIPE_CRC_STEREO_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_PIPE_CRC_STEREO_MODE {
OPP_PIPE_CRC_STEREO_MODE_LEFT            = 0x00000000,
OPP_PIPE_CRC_STEREO_MODE_RIGHT           = 0x00000001,
OPP_PIPE_CRC_STEREO_MODE_BOTH_RESET_AFTER_RIGHT_EYE = 0x00000002,
OPP_PIPE_CRC_STEREO_MODE_BOTH_RESET_AFTER_EACH_EYE = 0x00000003,
}


/*******************************************************
 * OPP_TOP Enums
 *******************************************************/

/*
 * OPP_ABM_DEBUG_BUS_SELECT_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_ABM_DEBUG_BUS_SELECT_CONTROL {
DEBUG_BUS_SELECT_ABM0                    = 0x00000000,
DEBUG_BUS_SELECT_ABM1                    = 0x00000001,
DEBUG_BUS_SELECT_ABM2                    = 0x00000002,
DEBUG_BUS_SELECT_ABM3                    = 0x00000003,
DEBUG_BUS_SELECT_ABM_RESERVED0           = 0x00000004,
DEBUG_BUS_SELECT_ABM_RESERVED1           = 0x00000005,
}


/*
 * OPP_DPG_DEBUG_BUS_SELECT_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_DPG_DEBUG_BUS_SELECT_CONTROL {
DEBUG_BUS_SELECT_DPG0                    = 0x00000000,
DEBUG_BUS_SELECT_DPG1                    = 0x00000001,
DEBUG_BUS_SELECT_DPG2                    = 0x00000002,
DEBUG_BUS_SELECT_DPG3                    = 0x00000003,
DEBUG_BUS_SELECT_DPG_RESERVED0           = 0x00000004,
DEBUG_BUS_SELECT_DPG_RESERVED1           = 0x00000005,
}


/*
 * OPP_FMT_DEBUG_BUS_SELECT_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_FMT_DEBUG_BUS_SELECT_CONTROL {
DEBUG_BUS_SELECT_FMT0                    = 0x00000000,
DEBUG_BUS_SELECT_FMT1                    = 0x00000001,
DEBUG_BUS_SELECT_FMT2                    = 0x00000002,
DEBUG_BUS_SELECT_FMT3                    = 0x00000003,
DEBUG_BUS_SELECT_FMT_RESERVED0           = 0x00000004,
DEBUG_BUS_SELECT_FMT_RESERVED1           = 0x00000005,
}


/*
 * OPP_OPPBUF_DEBUG_BUS_SELECT_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_OPPBUF_DEBUG_BUS_SELECT_CONTROL {
DEBUG_BUS_SELECT_OPPBUF0                 = 0x00000000,
DEBUG_BUS_SELECT_OPPBUF1                 = 0x00000001,
DEBUG_BUS_SELECT_OPPBUF2                 = 0x00000002,
DEBUG_BUS_SELECT_OPPBUF3                 = 0x00000003,
DEBUG_BUS_SELECT_OPPBUF_RESERVED0        = 0x00000004,
DEBUG_BUS_SELECT_OPPBUF_RESERVED1        = 0x00000005,
}


/*
 * OPP_OPP_PIPE_DEBUG_BUS_SELECT_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_OPP_PIPE_DEBUG_BUS_SELECT_CONTROL {
DEBUG_BUS_SELECT_OPP_PIPE0               = 0x00000000,
DEBUG_BUS_SELECT_OPP_PIPE1               = 0x00000001,
DEBUG_BUS_SELECT_OPP_PIPE2               = 0x00000002,
DEBUG_BUS_SELECT_OPP_PIPE3               = 0x00000003,
DEBUG_BUS_SELECT_OPP_PIPE_RESERVED0      = 0x00000004,
DEBUG_BUS_SELECT_OPP_PIPE_RESERVED1      = 0x00000005,
}


/*
 * OPP_TEST_CLK_SEL_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_TEST_CLK_SEL_CONTROL {
OPP_TEST_CLK_SEL_DISPCLK_P               = 0x00000000,
OPP_TEST_CLK_SEL_DISPCLK_R               = 0x00000001,
OPP_TEST_CLK_SEL_DISPCLK_ABM0            = 0x00000002,
OPP_TEST_CLK_SEL_DISPCLK_ABM1            = 0x00000003,
OPP_TEST_CLK_SEL_DISPCLK_ABM2            = 0x00000004,
OPP_TEST_CLK_SEL_DISPCLK_ABM3            = 0x00000005,
OPP_TEST_CLK_SEL_RESERVED0               = 0x00000006,
OPP_TEST_CLK_SEL_RESERVED1               = 0x00000007,
OPP_TEST_CLK_SEL_DISPCLK_OPP0            = 0x00000008,
OPP_TEST_CLK_SEL_DISPCLK_OPP1            = 0x00000009,
OPP_TEST_CLK_SEL_DISPCLK_OPP2            = 0x0000000a,
OPP_TEST_CLK_SEL_DISPCLK_OPP3            = 0x0000000b,
OPP_TEST_CLK_SEL_RESERVED2               = 0x0000000c,
OPP_TEST_CLK_SEL_RESERVED3               = 0x0000000d,
}


/*
 * OPP_TOP_CLOCK_ENABLE_STATUS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_TOP_CLOCK_ENABLE_STATUS {
OPP_TOP_CLOCK_DISABLED_STATUS            = 0x00000000,
OPP_TOP_CLOCK_ENABLED_STATUS             = 0x00000001,
}


/*
 * OPP_TOP_CLOCK_GATING_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPP_TOP_CLOCK_GATING_CONTROL {
OPP_TOP_CLOCK_GATING_ENABLED             = 0x00000000,
OPP_TOP_CLOCK_GATING_DISABLED            = 0x00000001,
}


/*******************************************************
 * DSCRM Enums
 *******************************************************/

/*
 * ENUM_DSCRM_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DSCRM_EN {
ENUM_DSCRM_DISABLE                       = 0x00000000,
ENUM_DSCRM_ENABLE                        = 0x00000001,
}


/*******************************************************
 * OTG Enums
 *******************************************************/

/*
 * MASTER_UPDATE_LOCK_MASTER_UPDATE_LOCK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MASTER_UPDATE_LOCK_MASTER_UPDATE_LOCK {
MASTER_UPDATE_LOCK_MASTER_UPDATE_LOCK_FALSE = 0x00000000,
MASTER_UPDATE_LOCK_MASTER_UPDATE_LOCK_TRUE = 0x00000001,
}


/*
 * MASTER_UPDATE_LOCK_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MASTER_UPDATE_LOCK_SEL {
MASTER_UPDATE_LOCK_SEL_0                 = 0x00000000,
MASTER_UPDATE_LOCK_SEL_1                 = 0x00000001,
MASTER_UPDATE_LOCK_SEL_2                 = 0x00000002,
MASTER_UPDATE_LOCK_SEL_3                 = 0x00000003,
MASTER_UPDATE_LOCK_SEL_RESERVED4         = 0x00000004,
MASTER_UPDATE_LOCK_SEL_RESERVED5         = 0x00000005,
}


/*
 * MASTER_UPDATE_MODE_MASTER_UPDATE_INTERLACED_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MASTER_UPDATE_MODE_MASTER_UPDATE_INTERLACED_MODE {
MASTER_UPDATE_MODE_MASTER_UPDATE_INTERLACED_MODE_BOTH = 0x00000000,
MASTER_UPDATE_MODE_MASTER_UPDATE_INTERLACED_MODE_TOP = 0x00000001,
MASTER_UPDATE_MODE_MASTER_UPDATE_INTERLACED_MODE_BOTTOM = 0x00000002,
MASTER_UPDATE_MODE_MASTER_UPDATE_INTERLACED_MODE_RESERVED = 0x00000003,
}


/*
 * OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_EN {
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_EN_FALSE = 0x00000000,
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_EN_TRUE = 0x00000001,
}


/*
 * OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_EN_DB enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_EN_DB {
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_EN_DB_FALSE = 0x00000000,
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_EN_DB_TRUE = 0x00000001,
}


/*
 * OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_STEREO_SEL_OVR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_STEREO_SEL_OVR {
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_STEREO_SEL_OVR_FALSE = 0x00000000,
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_STEREO_SEL_OVR_TRUE = 0x00000001,
}


/*
 * OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_V_UPDATE_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_V_UPDATE_MODE {
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_V_UPDATE_MODE_BLOCK_BOTH = 0x00000000,
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_V_UPDATE_MODE_BLOCK_INTERLACE = 0x00000001,
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_V_UPDATE_MODE_BLOCK_PROGRASSIVE = 0x00000002,
OTG_3D_STRUCTURE_CONTROL_OTG_3D_STRUCTURE_V_UPDATE_MODE_RESERVED = 0x00000003,
}


/*
 * OTG_CONTROL_OTG_DISABLE_POINT_CNTL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_CONTROL_OTG_DISABLE_POINT_CNTL {
OTG_CONTROL_OTG_DISABLE_POINT_CNTL_DISABLE = 0x00000000,
OTG_CONTROL_OTG_DISABLE_POINT_CNTL_DISABLE_CURRENT = 0x00000001,
OTG_CONTROL_OTG_DISABLE_POINT_CNTL_DISABLE_VUPDATE = 0x00000002,
OTG_CONTROL_OTG_DISABLE_POINT_CNTL_DISABLE_FIRST = 0x00000003,
}


/*
 * OTG_CONTROL_OTG_FIELD_NUMBER_CNTL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_CONTROL_OTG_FIELD_NUMBER_CNTL {
OTG_CONTROL_OTG_FIELD_NUMBER_CNTL_NORMAL = 0x00000000,
OTG_CONTROL_OTG_FIELD_NUMBER_CNTL_DP     = 0x00000001,
}


/*
 * OTG_CONTROL_OTG_FIELD_NUMBER_POLARITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_CONTROL_OTG_FIELD_NUMBER_POLARITY {
OTG_CONTROL_OTG_FIELD_NUMBER_POLARITY_FALSE = 0x00000000,
OTG_CONTROL_OTG_FIELD_NUMBER_POLARITY_TRUE = 0x00000001,
}


/*
 * OTG_CONTROL_OTG_MASTER_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_CONTROL_OTG_MASTER_EN {
OTG_CONTROL_OTG_MASTER_EN_FALSE          = 0x00000000,
OTG_CONTROL_OTG_MASTER_EN_TRUE           = 0x00000001,
}


/*
 * OTG_CONTROL_OTG_OUT_MUX enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_CONTROL_OTG_OUT_MUX {
OTG_CONTROL_OTG_OUT_MUX_0                = 0x00000000,
OTG_CONTROL_OTG_OUT_MUX_1                = 0x00000001,
OTG_CONTROL_OTG_OUT_MUX_2                = 0x00000002,
}


/*
 * OTG_CONTROL_OTG_START_POINT_CNTL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_CONTROL_OTG_START_POINT_CNTL {
OTG_CONTROL_OTG_START_POINT_CNTL_NORMAL  = 0x00000000,
OTG_CONTROL_OTG_START_POINT_CNTL_DP      = 0x00000001,
}


/*
 * OTG_COUNT_CONTROL_OTG_HORZ_COUNT_BY2_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_COUNT_CONTROL_OTG_HORZ_COUNT_BY2_EN {
OTG_COUNT_CONTROL_OTG_HORZ_COUNT_BY2_EN_FALSE = 0x00000000,
OTG_COUNT_CONTROL_OTG_HORZ_COUNT_BY2_EN_TRUE = 0x00000001,
}


/*
 * OTG_CRC_CNTL_OTG_CRC1_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_CRC_CNTL_OTG_CRC1_EN {
OTG_CRC_CNTL_OTG_CRC1_EN_FALSE           = 0x00000000,
OTG_CRC_CNTL_OTG_CRC1_EN_TRUE            = 0x00000001,
}


/*
 * OTG_CRC_CNTL_OTG_CRC_CONT_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_CRC_CNTL_OTG_CRC_CONT_EN {
OTG_CRC_CNTL_OTG_CRC_CONT_EN_FALSE       = 0x00000000,
OTG_CRC_CNTL_OTG_CRC_CONT_EN_TRUE        = 0x00000001,
}


/*
 * OTG_CRC_CNTL_OTG_CRC_CONT_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_CRC_CNTL_OTG_CRC_CONT_MODE {
OTG_CRC_CNTL_OTG_CRC_CONT_MODE_RESET     = 0x00000000,
OTG_CRC_CNTL_OTG_CRC_CONT_MODE_NORESET   = 0x00000001,
}


/*
 * OTG_CRC_CNTL_OTG_CRC_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_CRC_CNTL_OTG_CRC_EN {
OTG_CRC_CNTL_OTG_CRC_EN_FALSE            = 0x00000000,
OTG_CRC_CNTL_OTG_CRC_EN_TRUE             = 0x00000001,
}


/*
 * OTG_CRC_CNTL_OTG_CRC_INTERLACE_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_CRC_CNTL_OTG_CRC_INTERLACE_MODE {
OTG_CRC_CNTL_OTG_CRC_INTERLACE_MODE_TOP  = 0x00000000,
OTG_CRC_CNTL_OTG_CRC_INTERLACE_MODE_BOTTOM = 0x00000001,
OTG_CRC_CNTL_OTG_CRC_INTERLACE_MODE_BOTH_BOTTOM = 0x00000002,
OTG_CRC_CNTL_OTG_CRC_INTERLACE_MODE_BOTH_FIELD = 0x00000003,
}


/*
 * OTG_CRC_CNTL_OTG_CRC_STEREO_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_CRC_CNTL_OTG_CRC_STEREO_MODE {
OTG_CRC_CNTL_OTG_CRC_STEREO_MODE_LEFT    = 0x00000000,
OTG_CRC_CNTL_OTG_CRC_STEREO_MODE_RIGHT   = 0x00000001,
OTG_CRC_CNTL_OTG_CRC_STEREO_MODE_BOTH_EYES = 0x00000002,
OTG_CRC_CNTL_OTG_CRC_STEREO_MODE_BOTH_FIELDS = 0x00000003,
}


/*
 * OTG_CRC_CNTL_OTG_CRC_USE_NEW_AND_REPEATED_PIXELS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_CRC_CNTL_OTG_CRC_USE_NEW_AND_REPEATED_PIXELS {
OTG_CRC_CNTL_OTG_CRC_USE_NEW_AND_REPEATED_PIXELS_FALSE = 0x00000000,
OTG_CRC_CNTL_OTG_CRC_USE_NEW_AND_REPEATED_PIXELS_TRUE = 0x00000001,
}


/*
 * OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT {
OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT_UAB     = 0x00000000,
OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT_UA_B    = 0x00000001,
OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT_U_AB    = 0x00000002,
OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT_U_A_B   = 0x00000003,
OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT_IAB     = 0x00000004,
OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT_IA_B    = 0x00000005,
OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT_I_AB    = 0x00000006,
OTG_CRC_CNTL_OTG_OTG_CRC0_SELECT_I_A_B   = 0x00000007,
}


/*
 * OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT {
OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT_UAB     = 0x00000000,
OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT_UA_B    = 0x00000001,
OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT_U_AB    = 0x00000002,
OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT_U_A_B   = 0x00000003,
OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT_IAB     = 0x00000004,
OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT_IA_B    = 0x00000005,
OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT_I_AB    = 0x00000006,
OTG_CRC_CNTL_OTG_OTG_CRC1_SELECT_I_A_B   = 0x00000007,
}


/*
 * OTG_DIG_UPDATE_VCOUNT_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_DIG_UPDATE_VCOUNT_MODE {
OTG_DIG_UPDATE_VCOUNT_0                  = 0x00000000,
OTG_DIG_UPDATE_VCOUNT_1                  = 0x00000001,
}


/*
 * OTG_DOUBLE_BUFFER_CONTROL_OTG_DRR_TIMING_DBUF_UPDATE_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_DOUBLE_BUFFER_CONTROL_OTG_DRR_TIMING_DBUF_UPDATE_MODE {
OTG_DOUBLE_BUFFER_CONTROL_OTG_DRR_TIMING_DBUF_UPDATE_MODE_0 = 0x00000000,
OTG_DOUBLE_BUFFER_CONTROL_OTG_DRR_TIMING_DBUF_UPDATE_MODE_1 = 0x00000001,
OTG_DOUBLE_BUFFER_CONTROL_OTG_DRR_TIMING_DBUF_UPDATE_MODE_2 = 0x00000002,
OTG_DOUBLE_BUFFER_CONTROL_OTG_DRR_TIMING_DBUF_UPDATE_MODE_3 = 0x00000003,
}


/*
 * OTG_DOUBLE_BUFFER_CONTROL_OTG_UPDATE_INSTANTLY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_DOUBLE_BUFFER_CONTROL_OTG_UPDATE_INSTANTLY {
OTG_DOUBLE_BUFFER_CONTROL_OTG_UPDATE_INSTANTLY_FALSE = 0x00000000,
OTG_DOUBLE_BUFFER_CONTROL_OTG_UPDATE_INSTANTLY_TRUE = 0x00000001,
}


/*
 * OTG_DRR_CONTROL_OTG_DRR_AVERAGE_FRAME enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_DRR_CONTROL_OTG_DRR_AVERAGE_FRAME {
OTG_DRR_CONTROL_OTG_DRR_AVERAGE_FRAME_1FRAME = 0x00000000,
OTG_DRR_CONTROL_OTG_DRR_AVERAGE_FRAME_2FRAME = 0x00000001,
OTG_DRR_CONTROL_OTG_DRR_AVERAGE_FRAME_4FRAME = 0x00000002,
OTG_DRR_CONTROL_OTG_DRR_AVERAGE_FRAME_8FRAME = 0x00000003,
}


/*
 * OTG_DTMTEST_CNTL_OTG_DTMTEST_OTG_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_DTMTEST_CNTL_OTG_DTMTEST_OTG_EN {
OTG_DTMTEST_CNTL_OTG_DTMTEST_OTG_EN_FALSE = 0x00000000,
OTG_DTMTEST_CNTL_OTG_DTMTEST_OTG_EN_TRUE = 0x00000001,
}


/*
 * OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_GRANULARITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_GRANULARITY {
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_GRANULARITY_FALSE = 0x00000000,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_GRANULARITY_TRUE = 0x00000001,
}


/*
 * OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_POLARITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_POLARITY {
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_POLARITY_FALSE = 0x00000000,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_POLARITY_TRUE = 0x00000001,
}


/*
 * OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT {
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_LOGIC0 = 0x00000000,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_LOGIC1 = 0x00000001,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_GENERICA = 0x00000002,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_GENERICB = 0x00000003,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_GENERICC = 0x00000004,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_GENERICD = 0x00000005,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_GENERICE = 0x00000006,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_GENERICF = 0x00000007,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_HPD1 = 0x00000008,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_HPD2 = 0x00000009,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_DDC1DATA = 0x0000000a,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_DDC1CLK = 0x0000000b,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_DDC2DATA = 0x0000000c,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_DDC2CLK = 0x0000000d,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_MANUAL_FLOW_CONTROL = 0x0000000e,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_RESERVED = 0x0000000f,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_GENLK_CLK = 0x00000010,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_GENLK_VSYNC = 0x00000011,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_SWAPLOCKA = 0x00000012,
OTG_FLOW_CONTROL_OTG_FLOW_CONTROL_SOURCE_SELECT_SWAPLOCKB = 0x00000013,
}


/*
 * OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_CHECK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_CHECK {
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_CHECK_FALSE = 0x00000000,
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_CHECK_TRUE = 0x00000001,
}


/*
 * OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_CLEAR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_CLEAR {
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_CLEAR_FALSE = 0x00000000,
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_CLEAR_TRUE = 0x00000001,
}


/*
 * OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_MODE {
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_MODE_DISABLE = 0x00000000,
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_MODE_HCOUNT = 0x00000001,
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_MODE_HCOUNT_VCOUNT = 0x00000002,
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_MODE_RESERVED = 0x00000003,
}


/*
 * OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_TRIG_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_TRIG_SEL {
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_TRIG_SEL_FALSE = 0x00000000,
OTG_FORCE_COUNT_NOW_CNTL_OTG_FORCE_COUNT_NOW_TRIG_SEL_TRUE = 0x00000001,
}


/*
 * OTG_GLOBAL_CONTROL2_MANUAL_FLOW_CONTROL_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_GLOBAL_CONTROL2_MANUAL_FLOW_CONTROL_SEL {
OTG_GLOBAL_CONTROL2_MANUAL_FLOW_CONTROL_SEL_OTG0 = 0x00000000,
OTG_GLOBAL_CONTROL2_MANUAL_FLOW_CONTROL_SEL_OTG1 = 0x00000001,
OTG_GLOBAL_CONTROL2_MANUAL_FLOW_CONTROL_SEL_OTG2 = 0x00000002,
OTG_GLOBAL_CONTROL2_MANUAL_FLOW_CONTROL_SEL_OTG3 = 0x00000003,
OTG_GLOBAL_CONTROL2_MANUAL_FLOW_CONTROL_SEL_RESERVED4 = 0x00000004,
OTG_GLOBAL_CONTROL2_MANUAL_FLOW_CONTROL_SEL_RESERVED5 = 0x00000005,
}


/*
 * OTG_GLOBAL_CONTROL3_DIG_UPDATE_EYE_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_GLOBAL_CONTROL3_DIG_UPDATE_EYE_SEL {
DIG_UPDATE_EYE_SEL_BOTH                  = 0x00000000,
DIG_UPDATE_EYE_SEL_LEFT                  = 0x00000001,
DIG_UPDATE_EYE_SEL_RIGHT                 = 0x00000002,
}


/*
 * OTG_GLOBAL_CONTROL3_DIG_UPDATE_FIELD_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_GLOBAL_CONTROL3_DIG_UPDATE_FIELD_SEL {
DIG_UPDATE_FIELD_SEL_BOTH                = 0x00000000,
DIG_UPDATE_FIELD_SEL_TOP                 = 0x00000001,
DIG_UPDATE_FIELD_SEL_BOTTOM              = 0x00000002,
DIG_UPDATE_FIELD_SEL_RESERVED            = 0x00000003,
}


/*
 * OTG_GLOBAL_CONTROL3_MASTER_UPDATE_LOCK_DB_FIELD enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_GLOBAL_CONTROL3_MASTER_UPDATE_LOCK_DB_FIELD {
MASTER_UPDATE_LOCK_DB_FIELD_BOTH         = 0x00000000,
MASTER_UPDATE_LOCK_DB_FIELD_TOP          = 0x00000001,
MASTER_UPDATE_LOCK_DB_FIELD_BOTTOM       = 0x00000002,
MASTER_UPDATE_LOCK_DB_FIELD_RESERVED     = 0x00000003,
}


/*
 * OTG_GLOBAL_CONTROL3_MASTER_UPDATE_LOCK_DB_STEREO_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_GLOBAL_CONTROL3_MASTER_UPDATE_LOCK_DB_STEREO_SEL {
MASTER_UPDATE_LOCK_DB_STEREO_SEL_BOTH    = 0x00000000,
MASTER_UPDATE_LOCK_DB_STEREO_SEL_LEFT    = 0x00000001,
MASTER_UPDATE_LOCK_DB_STEREO_SEL_RIGHT   = 0x00000002,
MASTER_UPDATE_LOCK_DB_STEREO_SEL_RESERVED = 0x00000003,
}


/*
 * OTG_GLOBAL_UPDATE_LOCK_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_GLOBAL_UPDATE_LOCK_EN {
OTG_GLOBAL_UPDATE_LOCK_DISABLE           = 0x00000000,
OTG_GLOBAL_UPDATE_LOCK_ENABLE            = 0x00000001,
}


/*
 * OTG_GSL_MASTER_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_GSL_MASTER_MODE {
OTG_GSL_MASTER_MODE_0                    = 0x00000000,
OTG_GSL_MASTER_MODE_1                    = 0x00000001,
OTG_GSL_MASTER_MODE_2                    = 0x00000002,
OTG_GSL_MASTER_MODE_3                    = 0x00000003,
}


/*
 * OTG_HORZ_REPETITION_COUNT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_HORZ_REPETITION_COUNT {
OTG_HORZ_REPETITION_COUNT_0              = 0x00000000,
OTG_HORZ_REPETITION_COUNT_1              = 0x00000001,
OTG_HORZ_REPETITION_COUNT_2              = 0x00000002,
OTG_HORZ_REPETITION_COUNT_3              = 0x00000003,
OTG_HORZ_REPETITION_COUNT_4              = 0x00000004,
OTG_HORZ_REPETITION_COUNT_5              = 0x00000005,
OTG_HORZ_REPETITION_COUNT_6              = 0x00000006,
OTG_HORZ_REPETITION_COUNT_7              = 0x00000007,
OTG_HORZ_REPETITION_COUNT_8              = 0x00000008,
OTG_HORZ_REPETITION_COUNT_9              = 0x00000009,
OTG_HORZ_REPETITION_COUNT_10             = 0x0000000a,
OTG_HORZ_REPETITION_COUNT_11             = 0x0000000b,
OTG_HORZ_REPETITION_COUNT_12             = 0x0000000c,
OTG_HORZ_REPETITION_COUNT_13             = 0x0000000d,
OTG_HORZ_REPETITION_COUNT_14             = 0x0000000e,
OTG_HORZ_REPETITION_COUNT_15             = 0x0000000f,
}


/*
 * OTG_H_SYNC_A_POL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_H_SYNC_A_POL {
OTG_H_SYNC_A_POL_HIGH                    = 0x00000000,
OTG_H_SYNC_A_POL_LOW                     = 0x00000001,
}


/*
 * OTG_H_TIMING_DIV_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_H_TIMING_DIV_MODE {
OTG_H_TIMING_DIV_MODE_NO_DIV             = 0x00000000,
OTG_H_TIMING_DIV_MODE_DIV_BY2            = 0x00000001,
OTG_H_TIMING_DIV_MODE_RESERVED           = 0x00000002,
OTG_H_TIMING_DIV_MODE_DIV_BY4            = 0x00000003,
}


/*
 * OTG_H_TIMING_DIV_MODE_MANUAL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_H_TIMING_DIV_MODE_MANUAL {
OTG_H_TIMING_DIV_MODE_AUTO               = 0x00000000,
OTG_H_TIMING_DIV_MODE_NOAUTO             = 0x00000001,
}


/*
 * OTG_INTERLACE_CONTROL_OTG_INTERLACE_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_INTERLACE_CONTROL_OTG_INTERLACE_ENABLE {
OTG_INTERLACE_CONTROL_OTG_INTERLACE_ENABLE_FALSE = 0x00000000,
OTG_INTERLACE_CONTROL_OTG_INTERLACE_ENABLE_TRUE = 0x00000001,
}


/*
 * OTG_INTERLACE_CONTROL_OTG_INTERLACE_FORCE_NEXT_FIELD enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_INTERLACE_CONTROL_OTG_INTERLACE_FORCE_NEXT_FIELD {
OTG_INTERLACE_CONTROL_OTG_INTERLACE_FORCE_NEXT_FIELD_NOT = 0x00000000,
OTG_INTERLACE_CONTROL_OTG_INTERLACE_FORCE_NEXT_FIELD_BOTTOM = 0x00000001,
OTG_INTERLACE_CONTROL_OTG_INTERLACE_FORCE_NEXT_FIELD_TOP = 0x00000002,
OTG_INTERLACE_CONTROL_OTG_INTERLACE_FORCE_NEXT_FIELD_NOT2 = 0x00000003,
}


/*
 * OTG_INTERRUPT_CONTROL_OTG_FORCE_COUNT_NOW_INT_MSK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_INTERRUPT_CONTROL_OTG_FORCE_COUNT_NOW_INT_MSK {
OTG_INTERRUPT_CONTROL_OTG_FORCE_COUNT_NOW_INT_MSK_FALSE = 0x00000000,
OTG_INTERRUPT_CONTROL_OTG_FORCE_COUNT_NOW_INT_MSK_TRUE = 0x00000001,
}


/*
 * OTG_INTERRUPT_CONTROL_OTG_FORCE_COUNT_NOW_INT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_INTERRUPT_CONTROL_OTG_FORCE_COUNT_NOW_INT_TYPE {
OTG_INTERRUPT_CONTROL_OTG_FORCE_COUNT_NOW_INT_TYPE_FALSE = 0x00000000,
OTG_INTERRUPT_CONTROL_OTG_FORCE_COUNT_NOW_INT_TYPE_TRUE = 0x00000001,
}


/*
 * OTG_INTERRUPT_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_INT_MSK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_INTERRUPT_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_INT_MSK {
OTG_INTERRUPT_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_INT_MSK_FALSE = 0x00000000,
OTG_INTERRUPT_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_INT_MSK_TRUE = 0x00000001,
}


/*
 * OTG_INTERRUPT_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_INT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_INTERRUPT_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_INT_TYPE {
OTG_INTERRUPT_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_INT_TYPE_FALSE = 0x00000000,
OTG_INTERRUPT_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_INT_TYPE_TRUE = 0x00000001,
}


/*
 * OTG_INTERRUPT_CONTROL_OTG_GSL_VSYNC_GAP_INT_MSK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_INTERRUPT_CONTROL_OTG_GSL_VSYNC_GAP_INT_MSK {
OTG_INTERRUPT_CONTROL_OTG_GSL_VSYNC_GAP_INT_MSK_FALSE = 0x00000000,
OTG_INTERRUPT_CONTROL_OTG_GSL_VSYNC_GAP_INT_MSK_TRUE = 0x00000001,
}


/*
 * OTG_INTERRUPT_CONTROL_OTG_GSL_VSYNC_GAP_INT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_INTERRUPT_CONTROL_OTG_GSL_VSYNC_GAP_INT_TYPE {
OTG_INTERRUPT_CONTROL_OTG_GSL_VSYNC_GAP_INT_TYPE_FALSE = 0x00000000,
OTG_INTERRUPT_CONTROL_OTG_GSL_VSYNC_GAP_INT_TYPE_TRUE = 0x00000001,
}


/*
 * OTG_INTERRUPT_CONTROL_OTG_SNAPSHOT_INT_MSK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_INTERRUPT_CONTROL_OTG_SNAPSHOT_INT_MSK {
OTG_INTERRUPT_CONTROL_OTG_SNAPSHOT_INT_MSK_FALSE = 0x00000000,
OTG_INTERRUPT_CONTROL_OTG_SNAPSHOT_INT_MSK_TRUE = 0x00000001,
}


/*
 * OTG_INTERRUPT_CONTROL_OTG_SNAPSHOT_INT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_INTERRUPT_CONTROL_OTG_SNAPSHOT_INT_TYPE {
OTG_INTERRUPT_CONTROL_OTG_SNAPSHOT_INT_TYPE_FALSE = 0x00000000,
OTG_INTERRUPT_CONTROL_OTG_SNAPSHOT_INT_TYPE_TRUE = 0x00000001,
}


/*
 * OTG_INTERRUPT_CONTROL_OTG_TRIGA_INT_MSK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_INTERRUPT_CONTROL_OTG_TRIGA_INT_MSK {
OTG_INTERRUPT_CONTROL_OTG_TRIGA_INT_MSK_FALSE = 0x00000000,
OTG_INTERRUPT_CONTROL_OTG_TRIGA_INT_MSK_TRUE = 0x00000001,
}


/*
 * OTG_INTERRUPT_CONTROL_OTG_TRIGA_INT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_INTERRUPT_CONTROL_OTG_TRIGA_INT_TYPE {
OTG_INTERRUPT_CONTROL_OTG_TRIGA_INT_TYPE_FALSE = 0x00000000,
OTG_INTERRUPT_CONTROL_OTG_TRIGA_INT_TYPE_TRUE = 0x00000001,
}


/*
 * OTG_INTERRUPT_CONTROL_OTG_TRIGB_INT_MSK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_INTERRUPT_CONTROL_OTG_TRIGB_INT_MSK {
OTG_INTERRUPT_CONTROL_OTG_TRIGB_INT_MSK_FALSE = 0x00000000,
OTG_INTERRUPT_CONTROL_OTG_TRIGB_INT_MSK_TRUE = 0x00000001,
}


/*
 * OTG_INTERRUPT_CONTROL_OTG_TRIGB_INT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_INTERRUPT_CONTROL_OTG_TRIGB_INT_TYPE {
OTG_INTERRUPT_CONTROL_OTG_TRIGB_INT_TYPE_FALSE = 0x00000000,
OTG_INTERRUPT_CONTROL_OTG_TRIGB_INT_TYPE_TRUE = 0x00000001,
}


/*
 * OTG_INTERRUPT_CONTROL_OTG_VSYNC_NOM_INT_MSK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_INTERRUPT_CONTROL_OTG_VSYNC_NOM_INT_MSK {
OTG_INTERRUPT_CONTROL_OTG_VSYNC_NOM_INT_MSK_FALSE = 0x00000000,
OTG_INTERRUPT_CONTROL_OTG_VSYNC_NOM_INT_MSK_TRUE = 0x00000001,
}


/*
 * OTG_INTERRUPT_CONTROL_OTG_VSYNC_NOM_INT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_INTERRUPT_CONTROL_OTG_VSYNC_NOM_INT_TYPE {
OTG_INTERRUPT_CONTROL_OTG_VSYNC_NOM_INT_TYPE_FALSE = 0x00000000,
OTG_INTERRUPT_CONTROL_OTG_VSYNC_NOM_INT_TYPE_TRUE = 0x00000001,
}


/*
 * OTG_MANUAL_FORCE_VSYNC_NEXT_LINE_OTG_MANUAL_FORCE_VSYNC_NEXT_LINE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_MANUAL_FORCE_VSYNC_NEXT_LINE_OTG_MANUAL_FORCE_VSYNC_NEXT_LINE {
OTG_MANUAL_FORCE_VSYNC_NEXT_LINE_OTG_MANUAL_FORCE_VSYNC_NEXT_LINE_FALSE = 0x00000000,
OTG_MANUAL_FORCE_VSYNC_NEXT_LINE_OTG_MANUAL_FORCE_VSYNC_NEXT_LINE_TRUE = 0x00000001,
}


/*
 * OTG_MASTER_UPDATE_LOCK_DB_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_MASTER_UPDATE_LOCK_DB_EN {
OTG_MASTER_UPDATE_LOCK_DISABLE           = 0x00000000,
OTG_MASTER_UPDATE_LOCK_ENABLE            = 0x00000001,
}


/*
 * OTG_MASTER_UPDATE_LOCK_GSL_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_MASTER_UPDATE_LOCK_GSL_EN {
OTG_MASTER_UPDATE_LOCK_GSL_EN_FALSE      = 0x00000000,
OTG_MASTER_UPDATE_LOCK_GSL_EN_TRUE       = 0x00000001,
}


/*
 * OTG_MASTER_UPDATE_LOCK_VCOUNT_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_MASTER_UPDATE_LOCK_VCOUNT_MODE {
OTG_MASTER_UPDATE_LOCK_VCOUNT_0          = 0x00000000,
OTG_MASTER_UPDATE_LOCK_VCOUNT_1          = 0x00000001,
}


/*
 * OTG_SNAPSHOT_CONTROL_OTG_AUTO_SNAPSHOT_TRIG_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_SNAPSHOT_CONTROL_OTG_AUTO_SNAPSHOT_TRIG_SEL {
OTG_SNAPSHOT_CONTROL_OTG_AUTO_SNAPSHOT_TRIG_SEL_DISABLE = 0x00000000,
OTG_SNAPSHOT_CONTROL_OTG_AUTO_SNAPSHOT_TRIG_SEL_TRIGGERA = 0x00000001,
OTG_SNAPSHOT_CONTROL_OTG_AUTO_SNAPSHOT_TRIG_SEL_TRIGGERB = 0x00000002,
OTG_SNAPSHOT_CONTROL_OTG_AUTO_SNAPSHOT_TRIG_SEL_RESERVED = 0x00000003,
}


/*
 * OTG_SNAPSHOT_STATUS_OTG_SNAPSHOT_CLEAR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_SNAPSHOT_STATUS_OTG_SNAPSHOT_CLEAR {
OTG_SNAPSHOT_STATUS_OTG_SNAPSHOT_CLEAR_FALSE = 0x00000000,
OTG_SNAPSHOT_STATUS_OTG_SNAPSHOT_CLEAR_TRUE = 0x00000001,
}


/*
 * OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_CLEAR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_CLEAR {
OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_CLEAR_FALSE = 0x00000000,
OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_CLEAR_TRUE = 0x00000001,
}


/*
 * OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_ENABLE {
OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_ENABLE_FALSE = 0x00000000,
OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_ENABLE_TRUE = 0x00000001,
}


/*
 * OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_TYPE {
OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_TYPE_FALSE = 0x00000000,
OTG_STATIC_SCREEN_CONTROL_OTG_CPU_SS_INT_TYPE_TRUE = 0x00000001,
}


/*
 * OTG_STATIC_SCREEN_CONTROL_OTG_STATIC_SCREEN_OVERRIDE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_STATIC_SCREEN_CONTROL_OTG_STATIC_SCREEN_OVERRIDE {
OTG_STATIC_SCREEN_CONTROL_OTG_STATIC_SCREEN_OVERRIDE_FALSE = 0x00000000,
OTG_STATIC_SCREEN_CONTROL_OTG_STATIC_SCREEN_OVERRIDE_TRUE = 0x00000001,
}


/*
 * OTG_STATIC_SCREEN_CONTROL_OTG_STATIC_SCREEN_OVERRIDE_VALUE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_STATIC_SCREEN_CONTROL_OTG_STATIC_SCREEN_OVERRIDE_VALUE {
OTG_STATIC_SCREEN_CONTROL_OTG_STATIC_SCREEN_OVERRIDE_VALUE_OFF = 0x00000000,
OTG_STATIC_SCREEN_CONTROL_OTG_STATIC_SCREEN_OVERRIDE_VALUE_ON = 0x00000001,
}


/*
 * OTG_STEREO_CONTROL_OTG_FIELD_NUM_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_STEREO_CONTROL_OTG_FIELD_NUM_SEL {
OTG_STEREO_CONTROL_OTG_FIELD_NUM_SEL_FALSE = 0x00000000,
OTG_STEREO_CONTROL_OTG_FIELD_NUM_SEL_TRUE = 0x00000001,
}


/*
 * OTG_STEREO_CONTROL_OTG_STEREO_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_STEREO_CONTROL_OTG_STEREO_EN {
OTG_STEREO_CONTROL_OTG_STEREO_EN_FALSE   = 0x00000000,
OTG_STEREO_CONTROL_OTG_STEREO_EN_TRUE    = 0x00000001,
}


/*
 * OTG_STEREO_CONTROL_OTG_STEREO_EYE_FLAG_POLARITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_STEREO_CONTROL_OTG_STEREO_EYE_FLAG_POLARITY {
OTG_STEREO_CONTROL_OTG_STEREO_EYE_FLAG_POLARITY_FALSE = 0x00000000,
OTG_STEREO_CONTROL_OTG_STEREO_EYE_FLAG_POLARITY_TRUE = 0x00000001,
}


/*
 * OTG_STEREO_CONTROL_OTG_STEREO_SYNC_OUTPUT_POLARITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_STEREO_CONTROL_OTG_STEREO_SYNC_OUTPUT_POLARITY {
OTG_STEREO_CONTROL_OTG_STEREO_SYNC_OUTPUT_POLARITY_FALSE = 0x00000000,
OTG_STEREO_CONTROL_OTG_STEREO_SYNC_OUTPUT_POLARITY_TRUE = 0x00000001,
}


/*
 * OTG_STEREO_FORCE_NEXT_EYE_OTG_STEREO_FORCE_NEXT_EYE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_STEREO_FORCE_NEXT_EYE_OTG_STEREO_FORCE_NEXT_EYE {
OTG_STEREO_FORCE_NEXT_EYE_OTG_STEREO_FORCE_NEXT_EYE_NO = 0x00000000,
OTG_STEREO_FORCE_NEXT_EYE_OTG_STEREO_FORCE_NEXT_EYE_RIGHT = 0x00000001,
OTG_STEREO_FORCE_NEXT_EYE_OTG_STEREO_FORCE_NEXT_EYE_LEFT = 0x00000002,
OTG_STEREO_FORCE_NEXT_EYE_OTG_STEREO_FORCE_NEXT_EYE_RESERVED = 0x00000003,
}


/*
 * OTG_TRIGA_CNTL_OTG_TRIGA_CLEAR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_TRIGA_CNTL_OTG_TRIGA_CLEAR {
OTG_TRIGA_CNTL_OTG_TRIGA_CLEAR_FALSE     = 0x00000000,
OTG_TRIGA_CNTL_OTG_TRIGA_CLEAR_TRUE      = 0x00000001,
}


/*
 * OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT {
OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT_LOGIC0 = 0x00000000,
OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT_INTERLACE = 0x00000001,
OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT_GENERICA = 0x00000002,
OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT_GENERICB = 0x00000003,
OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT_HSYNCA = 0x00000004,
OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT_LOGIC1 = 0x00000005,
OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT_GENERICC = 0x00000006,
OTG_TRIGA_CNTL_OTG_TRIGA_POLARITY_SELECT_GENERICD = 0x00000007,
}


/*
 * OTG_TRIGA_CNTL_OTG_TRIGA_RESYNC_BYPASS_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_TRIGA_CNTL_OTG_TRIGA_RESYNC_BYPASS_EN {
OTG_TRIGA_CNTL_OTG_TRIGA_RESYNC_BYPASS_EN_FALSE = 0x00000000,
OTG_TRIGA_CNTL_OTG_TRIGA_RESYNC_BYPASS_EN_TRUE = 0x00000001,
}


/*
 * OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_PIPE_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_PIPE_SELECT {
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_PIPE_SELECT_OTG0 = 0x00000000,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_PIPE_SELECT_OTG1 = 0x00000001,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_PIPE_SELECT_OTG2 = 0x00000002,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_PIPE_SELECT_OTG3 = 0x00000003,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_PIPE_SELECT_RESERVED4 = 0x00000004,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_PIPE_SELECT_RESERVED5 = 0x00000005,
}


/*
 * OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT {
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_LOGIC0 = 0x00000000,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GENERICA_PIN = 0x00000001,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GENERICB_PIN = 0x00000002,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GENERICC_PIN = 0x00000003,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GENERICD_PIN = 0x00000004,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GENERICE_PIN = 0x00000005,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GENERICF_PIN = 0x00000006,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_SWAPLOCKA_PIN = 0x00000007,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_SWAPLOCKB_PIN = 0x00000008,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GENLK_CLK_PIN = 0x00000009,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GENLK_VSYNC_PIN = 0x0000000a,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_HPD1 = 0x0000000b,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_HPD2 = 0x0000000c,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_BLON_Y_PIN = 0x0000000d,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_RESERVED14 = 0x0000000e,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_UPDATE_LOCK = 0x0000000f,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_GSL_ALLOW_FLIP = 0x00000010,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_UPDATE_PENDING = 0x00000011,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_OTG_SOF = 0x00000012,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_HSYNC = 0x00000013,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_VSYNC = 0x00000014,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_OTG_TRIG_MANUAL_CONTROL = 0x00000015,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_MANUAL_FLOW_CONTROL = 0x00000016,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_LOGIC1 = 0x00000017,
OTG_TRIGA_CNTL_OTG_TRIGA_SOURCE_SELECT_FLIP_PENDING = 0x00000018,
}


/*
 * OTG_TRIGA_FALLING_EDGE_DETECT_CNTL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_TRIGA_FALLING_EDGE_DETECT_CNTL {
OTG_TRIGA_FALLING_EDGE_DETECT_CNTL_0     = 0x00000000,
OTG_TRIGA_FALLING_EDGE_DETECT_CNTL_1     = 0x00000001,
OTG_TRIGA_FALLING_EDGE_DETECT_CNTL_2     = 0x00000002,
OTG_TRIGA_FALLING_EDGE_DETECT_CNTL_3     = 0x00000003,
}


/*
 * OTG_TRIGA_FREQUENCY_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_TRIGA_FREQUENCY_SELECT {
OTG_TRIGA_FREQUENCY_SELECT_0             = 0x00000000,
OTG_TRIGA_FREQUENCY_SELECT_1             = 0x00000001,
OTG_TRIGA_FREQUENCY_SELECT_2             = 0x00000002,
OTG_TRIGA_FREQUENCY_SELECT_3             = 0x00000003,
}


/*
 * OTG_TRIGA_RISING_EDGE_DETECT_CNTL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_TRIGA_RISING_EDGE_DETECT_CNTL {
OTG_TRIGA_RISING_EDGE_DETECT_CNTL_0      = 0x00000000,
OTG_TRIGA_RISING_EDGE_DETECT_CNTL_1      = 0x00000001,
OTG_TRIGA_RISING_EDGE_DETECT_CNTL_2      = 0x00000002,
OTG_TRIGA_RISING_EDGE_DETECT_CNTL_3      = 0x00000003,
}


/*
 * OTG_TRIGB_CNTL_OTG_TRIGB_CLEAR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_TRIGB_CNTL_OTG_TRIGB_CLEAR {
OTG_TRIGB_CNTL_OTG_TRIGB_CLEAR_FALSE     = 0x00000000,
OTG_TRIGB_CNTL_OTG_TRIGB_CLEAR_TRUE      = 0x00000001,
}


/*
 * OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT {
OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT_LOGIC0 = 0x00000000,
OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT_INTERLACE = 0x00000001,
OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT_GENERICA = 0x00000002,
OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT_GENERICB = 0x00000003,
OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT_HSYNCA = 0x00000004,
OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT_LOGIC1 = 0x00000005,
OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT_GENERICC = 0x00000006,
OTG_TRIGB_CNTL_OTG_TRIGB_POLARITY_SELECT_GENERICD = 0x00000007,
}


/*
 * OTG_TRIGB_CNTL_OTG_TRIGB_RESYNC_BYPASS_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_TRIGB_CNTL_OTG_TRIGB_RESYNC_BYPASS_EN {
OTG_TRIGB_CNTL_OTG_TRIGB_RESYNC_BYPASS_EN_FALSE = 0x00000000,
OTG_TRIGB_CNTL_OTG_TRIGB_RESYNC_BYPASS_EN_TRUE = 0x00000001,
}


/*
 * OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_PIPE_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_PIPE_SELECT {
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_PIPE_SELECT_OTG0 = 0x00000000,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_PIPE_SELECT_OTG1 = 0x00000001,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_PIPE_SELECT_OTG2 = 0x00000002,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_PIPE_SELECT_OTG3 = 0x00000003,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_PIPE_SELECT_RESERVED4 = 0x00000004,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_PIPE_SELECT_RESERVED5 = 0x00000005,
}


/*
 * OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT {
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_LOGIC0 = 0x00000000,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GENERICA_PIN = 0x00000001,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GENERICB_PIN = 0x00000002,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GENERICC_PIN = 0x00000003,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GENERICD_PIN = 0x00000004,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GENERICE_PIN = 0x00000005,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GENERICF_PIN = 0x00000006,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_SWAPLOCKA_PIN = 0x00000007,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_SWAPLOCKB_PIN = 0x00000008,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GENLK_CLK_PIN = 0x00000009,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GENLK_VSYNC_PIN = 0x0000000a,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_HPD1 = 0x0000000b,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_HPD2 = 0x0000000c,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_BLON_Y_PIN = 0x0000000d,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_RESERVED14 = 0x0000000e,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_UPDATE_LOCK = 0x0000000f,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_GSL_ALLOW_FLIP = 0x00000010,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_UPDATE_PENDING = 0x00000011,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_OTG_SOF = 0x00000012,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_HSYNC = 0x00000013,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_VSYNC = 0x00000014,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_OTG_TRIG_MANUAL_CONTROL = 0x00000015,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_MANUAL_FLOW_CONTROL = 0x00000016,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_LOGIC1 = 0x00000017,
OTG_TRIGB_CNTL_OTG_TRIGB_SOURCE_SELECT_FLIP_PENDING = 0x00000018,
}


/*
 * OTG_TRIGB_FALLING_EDGE_DETECT_CNTL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_TRIGB_FALLING_EDGE_DETECT_CNTL {
OTG_TRIGB_FALLING_EDGE_DETECT_CNTL_0     = 0x00000000,
OTG_TRIGB_FALLING_EDGE_DETECT_CNTL_1     = 0x00000001,
OTG_TRIGB_FALLING_EDGE_DETECT_CNTL_2     = 0x00000002,
OTG_TRIGB_FALLING_EDGE_DETECT_CNTL_3     = 0x00000003,
}


/*
 * OTG_TRIGB_FREQUENCY_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_TRIGB_FREQUENCY_SELECT {
OTG_TRIGB_FREQUENCY_SELECT_0             = 0x00000000,
OTG_TRIGB_FREQUENCY_SELECT_1             = 0x00000001,
OTG_TRIGB_FREQUENCY_SELECT_2             = 0x00000002,
OTG_TRIGB_FREQUENCY_SELECT_3             = 0x00000003,
}


/*
 * OTG_TRIGB_RISING_EDGE_DETECT_CNTL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_TRIGB_RISING_EDGE_DETECT_CNTL {
OTG_TRIGB_RISING_EDGE_DETECT_CNTL_0      = 0x00000000,
OTG_TRIGB_RISING_EDGE_DETECT_CNTL_1      = 0x00000001,
OTG_TRIGB_RISING_EDGE_DETECT_CNTL_2      = 0x00000002,
OTG_TRIGB_RISING_EDGE_DETECT_CNTL_3      = 0x00000003,
}


/*
 * OTG_UPDATE_LOCK_OTG_UPDATE_LOCK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_UPDATE_LOCK_OTG_UPDATE_LOCK {
OTG_UPDATE_LOCK_OTG_UPDATE_LOCK_FALSE    = 0x00000000,
OTG_UPDATE_LOCK_OTG_UPDATE_LOCK_TRUE     = 0x00000001,
}


/*
 * OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_CLEAR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_CLEAR {
OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_CLEAR_FALSE = 0x00000000,
OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_CLEAR_TRUE = 0x00000001,
}


/*
 * OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_INT_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_INT_ENABLE {
OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_INT_ENABLE_FALSE = 0x00000000,
OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_INT_ENABLE_TRUE = 0x00000001,
}


/*
 * OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_INT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_INT_TYPE {
OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_INT_TYPE_FALSE = 0x00000000,
OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_INT_TYPE_TRUE = 0x00000001,
}


/*
 * OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_OUTPUT_POLARITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_OUTPUT_POLARITY {
OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_OUTPUT_POLARITY_FALSE = 0x00000000,
OTG_VERTICAL_INTERRUPT0_CONTROL_OTG_VERTICAL_INTERRUPT0_OUTPUT_POLARITY_TRUE = 0x00000001,
}


/*
 * OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_CLEAR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_CLEAR {
OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_CLEAR_CLEAR_FALSE = 0x00000000,
OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_CLEAR_TRUE = 0x00000001,
}


/*
 * OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_INT_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_INT_ENABLE {
OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_INT_ENABLE_FALSE = 0x00000000,
OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_INT_ENABLE_TRUE = 0x00000001,
}


/*
 * OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_INT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_INT_TYPE {
OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_INT_TYPE_FALSE = 0x00000000,
OTG_VERTICAL_INTERRUPT1_CONTROL_OTG_VERTICAL_INTERRUPT1_INT_TYPE_TRUE = 0x00000001,
}


/*
 * OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_CLEAR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_CLEAR {
OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_CLEAR_CLEAR_FALSE = 0x00000000,
OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_CLEAR_TRUE = 0x00000001,
}


/*
 * OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_INT_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_INT_ENABLE {
OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_INT_ENABLE_FALSE = 0x00000000,
OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_INT_ENABLE_TRUE = 0x00000001,
}


/*
 * OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_INT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_INT_TYPE {
OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_INT_TYPE_FALSE = 0x00000000,
OTG_VERTICAL_INTERRUPT2_CONTROL_OTG_VERTICAL_INTERRUPT2_INT_TYPE_TRUE = 0x00000001,
}


/*
 * OTG_VERT_SYNC_CONTROL_OTG_AUTO_FORCE_VSYNC_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_VERT_SYNC_CONTROL_OTG_AUTO_FORCE_VSYNC_MODE {
OTG_VERT_SYNC_CONTROL_OTG_AUTO_FORCE_VSYNC_MODE_DISABLE = 0x00000000,
OTG_VERT_SYNC_CONTROL_OTG_AUTO_FORCE_VSYNC_MODE_TRIGGERA = 0x00000001,
OTG_VERT_SYNC_CONTROL_OTG_AUTO_FORCE_VSYNC_MODE_TRIGGERB = 0x00000002,
OTG_VERT_SYNC_CONTROL_OTG_AUTO_FORCE_VSYNC_MODE_RESERVED = 0x00000003,
}


/*
 * OTG_VERT_SYNC_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_CLEAR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_VERT_SYNC_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_CLEAR {
OTG_VERT_SYNC_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_CLEAR_FALSE = 0x00000000,
OTG_VERT_SYNC_CONTROL_OTG_FORCE_VSYNC_NEXT_LINE_CLEAR_TRUE = 0x00000001,
}


/*
 * OTG_VSYNC_NOM_INT_STATUS_OTG_VSYNC_NOM_INT_CLEAR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_VSYNC_NOM_INT_STATUS_OTG_VSYNC_NOM_INT_CLEAR {
OTG_VSYNC_NOM_INT_STATUS_OTG_VSYNC_NOM_INT_CLEAR_FALSE = 0x00000000,
OTG_VSYNC_NOM_INT_STATUS_OTG_VSYNC_NOM_INT_CLEAR_TRUE = 0x00000001,
}


/*
 * OTG_VUPDATE_BLOCK_DISABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_VUPDATE_BLOCK_DISABLE {
OTG_VUPDATE_BLOCK_DISABLE_OFF            = 0x00000000,
OTG_VUPDATE_BLOCK_DISABLE_ON             = 0x00000001,
}


/*
 * OTG_V_SYNC_A_POL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_V_SYNC_A_POL {
OTG_V_SYNC_A_POL_HIGH                    = 0x00000000,
OTG_V_SYNC_A_POL_LOW                     = 0x00000001,
}


/*
 * OTG_V_SYNC_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_V_SYNC_MODE {
OTG_V_SYNC_MODE_HSYNC                    = 0x00000000,
OTG_V_SYNC_MODE_HBLANK                   = 0x00000001,
}


/*
 * OTG_V_TOTAL_CONTROL_OTG_DRR_EVENT_ACTIVE_PERIOD enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_V_TOTAL_CONTROL_OTG_DRR_EVENT_ACTIVE_PERIOD {
OTG_V_TOTAL_CONTROL_OTG_DRR_EVENT_ACTIVE_PERIOD_0 = 0x00000000,
OTG_V_TOTAL_CONTROL_OTG_DRR_EVENT_ACTIVE_PERIOD_1 = 0x00000001,
}


/*
 * OTG_V_TOTAL_CONTROL_OTG_FORCE_LOCK_ON_EVENT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_V_TOTAL_CONTROL_OTG_FORCE_LOCK_ON_EVENT {
OTG_V_TOTAL_CONTROL_OTG_FORCE_LOCK_ON_EVENT_DISABLE = 0x00000000,
OTG_V_TOTAL_CONTROL_OTG_FORCE_LOCK_ON_EVENT_ENABLE = 0x00000001,
}


/*
 * OTG_V_TOTAL_CONTROL_OTG_FORCE_LOCK_TO_MASTER_VSYNC enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_V_TOTAL_CONTROL_OTG_FORCE_LOCK_TO_MASTER_VSYNC {
OTG_V_TOTAL_CONTROL_OTG_FORCE_LOCK_TO_MASTER_VSYNC_DISABLE = 0x00000000,
OTG_V_TOTAL_CONTROL_OTG_FORCE_LOCK_TO_MASTER_VSYNC_ENABLE = 0x00000001,
}


/*
 * OTG_V_TOTAL_CONTROL_OTG_V_TOTAL_MAX_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_V_TOTAL_CONTROL_OTG_V_TOTAL_MAX_SEL {
OTG_V_TOTAL_CONTROL_OTG_V_TOTAL_MAX_SEL_FALSE = 0x00000000,
OTG_V_TOTAL_CONTROL_OTG_V_TOTAL_MAX_SEL_TRUE = 0x00000001,
}


/*
 * OTG_V_TOTAL_CONTROL_OTG_V_TOTAL_MIN_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_V_TOTAL_CONTROL_OTG_V_TOTAL_MIN_SEL {
OTG_V_TOTAL_CONTROL_OTG_V_TOTAL_MIN_SEL_FALSE = 0x00000000,
OTG_V_TOTAL_CONTROL_OTG_V_TOTAL_MIN_SEL_TRUE = 0x00000001,
}


/*
 * OTG_V_TOTAL_INT_STATUS_OTG_SET_V_TOTAL_MIN_EVENT_OCCURRED_ACK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_V_TOTAL_INT_STATUS_OTG_SET_V_TOTAL_MIN_EVENT_OCCURRED_ACK {
OTG_V_TOTAL_INT_STATUS_OTG_SET_V_TOTAL_MIN_EVENT_OCCURRED_ACK_FALSE = 0x00000000,
OTG_V_TOTAL_INT_STATUS_OTG_SET_V_TOTAL_MIN_EVENT_OCCURRED_ACK_TRUE = 0x00000001,
}


/*******************************************************
 * OPTC_MISC Enums
 *******************************************************/

/*
 * OPTC_GSL_SOURCE_SELECT_GSL_TIMING_SYNC_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OPTC_GSL_SOURCE_SELECT_GSL_TIMING_SYNC_SEL {
OPTC_GSL_SOURCE_SELECT_GSL_TIMING_SYNC_SEL_OTG0 = 0x00000000,
OPTC_GSL_SOURCE_SELECT_GSL_TIMING_SYNC_SEL_OTG1 = 0x00000001,
OPTC_GSL_SOURCE_SELECT_GSL_TIMING_SYNC_SEL_OTG2 = 0x00000002,
OPTC_GSL_SOURCE_SELECT_GSL_TIMING_SYNC_SEL_OTG3 = 0x00000003,
OPTC_GSL_SOURCE_SELECT_GSL_TIMING_SYNC_SEL_RESERVED4 = 0x00000004,
OPTC_GSL_SOURCE_SELECT_GSL_TIMING_SYNC_SEL_RESERVED5 = 0x00000005,
}


/*******************************************************
 * DMCUB Enums
 *******************************************************/

/*
 * DC_DMCUB_INT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DC_DMCUB_INT_TYPE {
INT_LEVEL                                = 0x00000000,
INT_PULSE                                = 0x00000001,
}


/*
 * DC_DMCUB_TIMER_WINDOW enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DC_DMCUB_TIMER_WINDOW {
BITS_31_0                                = 0x00000000,
BITS_32_1                                = 0x00000001,
BITS_33_2                                = 0x00000002,
BITS_34_3                                = 0x00000003,
BITS_35_4                                = 0x00000004,
BITS_36_5                                = 0x00000005,
BITS_37_6                                = 0x00000006,
BITS_38_7                                = 0x00000007,
}


/*******************************************************
 * RBBMIF Enums
 *******************************************************/

/*
 * INVALID_REG_ACCESS_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum INVALID_REG_ACCESS_TYPE {
REG_UNALLOCATED_ADDR_WRITE               = 0x00000000,
REG_UNALLOCATED_ADDR_READ                = 0x00000001,
REG_VIRTUAL_WRITE                        = 0x00000002,
REG_VIRTUAL_READ                         = 0x00000003,
REG_SECURE_VIOLATE_WRITE                 = 0x00000004,
REG_SECURE_VIOLATE_READ                  = 0x00000005,
}


/*******************************************************
 * IHC Enums
 *******************************************************/

/*
 * DMU_DC_GPU_TIMER_READ_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DMU_DC_GPU_TIMER_READ_SELECT {
DMU_GPU_TIMER_READ_SELECT_LOWER_D1_V_UPDATE_0 = 0x00000000,
DMU_GPU_TIMER_READ_SELECT_UPPER_D1_V_UPDATE_1 = 0x00000001,
DMU_GPU_TIMER_READ_SELECT_LOWER_D2_V_UPDATE_2 = 0x00000002,
DMU_GPU_TIMER_READ_SELECT_UPPER_D2_V_UPDATE_3 = 0x00000003,
DMU_GPU_TIMER_READ_SELECT_LOWER_D3_V_UPDATE_4 = 0x00000004,
DMU_GPU_TIMER_READ_SELECT_UPPER_D3_V_UPDATE_5 = 0x00000005,
DMU_GPU_TIMER_READ_SELECT_LOWER_D4_V_UPDATE_6 = 0x00000006,
DMU_GPU_TIMER_READ_SELECT_UPPER_D4_V_UPDATE_7 = 0x00000007,
RESERVED_8                               = 0x00000008,
RESERVED_9                               = 0x00000009,
RESERVED_10                              = 0x0000000a,
RESERVED_11                              = 0x0000000b,
DMU_GPU_TIMER_READ_SELECT_LOWER_D1_V_STARTUP_12 = 0x0000000c,
DMU_GPU_TIMER_READ_SELECT_UPPER_D1_V_STARTUP_13 = 0x0000000d,
DMU_GPU_TIMER_READ_SELECT_LOWER_D2_V_STARTUP_14 = 0x0000000e,
DMU_GPU_TIMER_READ_SELECT_UPPER_D2_V_STARTUP_15 = 0x0000000f,
DMU_GPU_TIMER_READ_SELECT_LOWER_D3_V_STARTUP_16 = 0x00000010,
DMU_GPU_TIMER_READ_SELECT_UPPER_D3_V_STARTUP_17 = 0x00000011,
DMU_GPU_TIMER_READ_SELECT_LOWER_D4_V_STARTUP_18 = 0x00000012,
DMU_GPU_TIMER_READ_SELECT_UPPER_D4_V_STARTUP_19 = 0x00000013,
RESERVED_20                              = 0x00000014,
RESERVED_21                              = 0x00000015,
RESERVED_22                              = 0x00000016,
RESERVED_23                              = 0x00000017,
DMU_GPU_TIMER_READ_SELECT_LOWER_D1_VSYNC_NOM_24 = 0x00000018,
DMU_GPU_TIMER_READ_SELECT_UPPER_D1_VSYNC_NOM_25 = 0x00000019,
DMU_GPU_TIMER_READ_SELECT_LOWER_D2_VSYNC_NOM_26 = 0x0000001a,
DMU_GPU_TIMER_READ_SELECT_UPPER_D2_VSYNC_NOM_27 = 0x0000001b,
DMU_GPU_TIMER_READ_SELECT_LOWER_D3_VSYNC_NOM_28 = 0x0000001c,
DMU_GPU_TIMER_READ_SELECT_UPPER_D3_VSYNC_NOM_29 = 0x0000001d,
DMU_GPU_TIMER_READ_SELECT_LOWER_D4_VSYNC_NOM_30 = 0x0000001e,
DMU_GPU_TIMER_READ_SELECT_UPPER_D4_VSYNC_NOM_31 = 0x0000001f,
RESERVED_32                              = 0x00000020,
RESERVED_33                              = 0x00000021,
RESERVED_34                              = 0x00000022,
RESERVED_35                              = 0x00000023,
DMU_GPU_TIMER_READ_SELECT_LOWER_D1_VREADY_36 = 0x00000024,
DMU_GPU_TIMER_READ_SELECT_UPPER_D1_VREADY_37 = 0x00000025,
DMU_GPU_TIMER_READ_SELECT_LOWER_D2_VREADY_38 = 0x00000026,
DMU_GPU_TIMER_READ_SELECT_UPPER_D2_VREADY_39 = 0x00000027,
DMU_GPU_TIMER_READ_SELECT_LOWER_D3_VREADY_40 = 0x00000028,
DMU_GPU_TIMER_READ_SELECT_UPPER_D3_VREADY_41 = 0x00000029,
DMU_GPU_TIMER_READ_SELECT_LOWER_D4_VREADY_42 = 0x0000002a,
DMU_GPU_TIMER_READ_SELECT_UPPER_D4_VREADY_43 = 0x0000002b,
RESERVED_44                              = 0x0000002c,
RESERVED_45                              = 0x0000002d,
RESERVED_46                              = 0x0000002e,
RESERVED_47                              = 0x0000002f,
DMU_GPU_TIMER_READ_SELECT_LOWER_D1_FLIP_48 = 0x00000030,
DMU_GPU_TIMER_READ_SELECT_UPPER_D1_FLIP_49 = 0x00000031,
DMU_GPU_TIMER_READ_SELECT_LOWER_D2_FLIP_50 = 0x00000032,
DMU_GPU_TIMER_READ_SELECT_UPPER_D2_FLIP_51 = 0x00000033,
DMU_GPU_TIMER_READ_SELECT_LOWER_D3_FLIP_52 = 0x00000034,
DMU_GPU_TIMER_READ_SELECT_UPPER_D3_FLIP_53 = 0x00000035,
DMU_GPU_TIMER_READ_SELECT_LOWER_D4_FLIP_54 = 0x00000036,
DMU_GPU_TIMER_READ_SELECT_UPPER_D4_FLIP_55 = 0x00000037,
RESERVED_56                              = 0x00000038,
RESERVED_57                              = 0x00000039,
RESERVED_58                              = 0x0000003a,
RESERVED_59                              = 0x0000003b,
RESERVED_60                              = 0x0000003c,
RESERVED_61                              = 0x0000003d,
RESERVED_62                              = 0x0000003e,
RESERVED_63                              = 0x0000003f,
DMU_GPU_TIMER_READ_SELECT_LOWER_D1_V_UPDATE_NO_LOCK_64 = 0x00000040,
DMU_GPU_TIMER_READ_SELECT_UPPER_D1_V_UPDATE_NO_LOCK_65 = 0x00000041,
DMU_GPU_TIMER_READ_SELECT_LOWER_D2_V_UPDATE_NO_LOCK_66 = 0x00000042,
DMU_GPU_TIMER_READ_SELECT_UPPER_D2_V_UPDATE_NO_LOCK_67 = 0x00000043,
DMU_GPU_TIMER_READ_SELECT_LOWER_D3_V_UPDATE_NO_LOCK_68 = 0x00000044,
DMU_GPU_TIMER_READ_SELECT_UPPER_D3_V_UPDATE_NO_LOCK_69 = 0x00000045,
DMU_GPU_TIMER_READ_SELECT_LOWER_D4_V_UPDATE_NO_LOCK_70 = 0x00000046,
DMU_GPU_TIMER_READ_SELECT_UPPER_D4_V_UPDATE_NO_LOCK_71 = 0x00000047,
RESERVED_72                              = 0x00000048,
RESERVED_73                              = 0x00000049,
RESERVED_74                              = 0x0000004a,
RESERVED_75                              = 0x0000004b,
DMU_GPU_TIMER_READ_SELECT_LOWER_D1_FLIP_AWAY_76 = 0x0000004c,
DMU_GPU_TIMER_READ_SELECT_UPPER_D1_FLIP_AWAY_77 = 0x0000004d,
DMU_GPU_TIMER_READ_SELECT_LOWER_D2_FLIP_AWAY_78 = 0x0000004e,
DMU_GPU_TIMER_READ_SELECT_UPPER_D2_FLIP_AWAY_79 = 0x0000004f,
DMU_GPU_TIMER_READ_SELECT_LOWER_D3_FLIP_AWAY_80 = 0x00000050,
DMU_GPU_TIMER_READ_SELECT_UPPER_D3_FLIP_AWAY_81 = 0x00000051,
DMU_GPU_TIMER_READ_SELECT_LOWER_D4_FLIP_AWAY_82 = 0x00000052,
DMU_GPU_TIMER_READ_SELECT_UPPER_D4_FLIP_AWAY_83 = 0x00000053,
RESERVED_84                              = 0x00000054,
RESERVED_85                              = 0x00000055,
RESERVED_86                              = 0x00000056,
RESERVED_87                              = 0x00000057,
RESERVED_88                              = 0x00000058,
RESERVED_89                              = 0x00000059,
RESERVED_90                              = 0x0000005a,
RESERVED_91                              = 0x0000005b,
}


/*
 * DMU_DC_GPU_TIMER_START_POSITION enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DMU_DC_GPU_TIMER_START_POSITION {
DMU_GPU_TIMER_START_0_END_27             = 0x00000000,
DMU_GPU_TIMER_START_1_END_28             = 0x00000001,
DMU_GPU_TIMER_START_2_END_29             = 0x00000002,
DMU_GPU_TIMER_START_3_END_30             = 0x00000003,
DMU_GPU_TIMER_START_4_END_31             = 0x00000004,
DMU_GPU_TIMER_START_6_END_33             = 0x00000005,
DMU_GPU_TIMER_START_8_END_35             = 0x00000006,
DMU_GPU_TIMER_START_10_END_37            = 0x00000007,
}


/*
 * IHC_INTERRUPT_DEST enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IHC_INTERRUPT_DEST {
INTERRUPT_SENT_TO_IH                     = 0x00000000,
INTERRUPT_SENT_TO_DMCUB                  = 0x00000001,
}


/*
 * IHC_INTERRUPT_LINE_STATUS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IHC_INTERRUPT_LINE_STATUS {
INTERRUPT_LINE_NOT_ASSERTED              = 0x00000000,
INTERRUPT_LINE_ASSERTED                  = 0x00000001,
}


/*******************************************************
 * DMU_MISC Enums
 *******************************************************/

/*
 * DC_SMU_INTERRUPT_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DC_SMU_INTERRUPT_ENABLE {
DISABLE_THE_INTERRUPT                    = 0x00000000,
ENABLE_THE_INTERRUPT                     = 0x00000001,
}


/*
 * DMU_CLOCK_ON enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DMU_CLOCK_ON {
DMU_CLOCK_STATUS_ON                      = 0x00000000,
DMU_CLOCK_STATUS_OFF                     = 0x00000001,
}


/*
 * SMU_INTR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SMU_INTR {
SMU_MSG_INTR_NOOP                        = 0x00000000,
SET_SMU_MSG_INTR                         = 0x00000001,
}


/*******************************************************
 * DCCG Enums
 *******************************************************/

/*
 * ALLOW_SR_ON_TRANS_REQ enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ALLOW_SR_ON_TRANS_REQ {
ALLOW_SR_ON_TRANS_REQ_ENABLE             = 0x00000000,
ALLOW_SR_ON_TRANS_REQ_DISABLE            = 0x00000001,
}


/*
 * AMCLOCK_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AMCLOCK_ENABLE {
ENABLE_AMCLK0                            = 0x00000000,
ENABLE_AMCLK1                            = 0x00000001,
}


/*
 * CLEAR_SMU_INTR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CLEAR_SMU_INTR {
SMU_INTR_STATUS_NOOP                     = 0x00000000,
SMU_INTR_STATUS_CLEAR                    = 0x00000001,
}


/*
 * CLOCK_BRANCH_SOFT_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CLOCK_BRANCH_SOFT_RESET {
CLOCK_BRANCH_SOFT_RESET_NOOP             = 0x00000000,
CLOCK_BRANCH_SOFT_RESET_FORCE            = 0x00000001,
}


/*
 * DCCG_AUDIO_DTO0_SOURCE_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCCG_AUDIO_DTO0_SOURCE_SEL {
DCCG_AUDIO_DTO0_SOURCE_SEL_OTG0          = 0x00000000,
DCCG_AUDIO_DTO0_SOURCE_SEL_OTG1          = 0x00000001,
DCCG_AUDIO_DTO0_SOURCE_SEL_OTG2          = 0x00000002,
DCCG_AUDIO_DTO0_SOURCE_SEL_OTG3          = 0x00000003,
DCCG_AUDIO_DTO0_SOURCE_SEL_RESERVED      = 0x00000004,
}


/*
 * DCCG_AUDIO_DTO2_SOURCE_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCCG_AUDIO_DTO2_SOURCE_SEL {
DCCG_AUDIO_DTO2_SOURCE_SEL_AMCLK0        = 0x00000000,
DCCG_AUDIO_DTO2_SOURCE_SEL_AMCLK0_DIV2   = 0x00000001,
}


/*
 * DCCG_AUDIO_DTO_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCCG_AUDIO_DTO_SEL {
DCCG_AUDIO_DTO_SEL_AUDIO_DTO0            = 0x00000000,
DCCG_AUDIO_DTO_SEL_AUDIO_DTO1            = 0x00000001,
DCCG_AUDIO_DTO_SEL_NO_AUDIO_DTO          = 0x00000002,
DCCG_AUDIO_DTO_SEL_AUDIO_DTO_DTBCLK      = 0x00000003,
}


/*
 * DCCG_AUDIO_DTO_USE_512FBR_DTO enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCCG_AUDIO_DTO_USE_512FBR_DTO {
DCCG_AUDIO_DTO_USE_128FBR_FOR_DP         = 0x00000000,
DCCG_AUDIO_DTO_USE_512FBR_FOR_DP         = 0x00000001,
}


/*
 * DCCG_DBG_BLOCK_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCCG_DBG_BLOCK_SEL {
DCCG_DBG_BLOCK_SEL_DCCG                  = 0x00000000,
DCCG_DBG_BLOCK_SEL_PMON                  = 0x00000001,
DCCG_DBG_BLOCK_SEL_PMON2                 = 0x00000002,
}


/*
 * DCCG_DBG_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCCG_DBG_EN {
DCCG_DBG_EN_DISABLE                      = 0x00000000,
DCCG_DBG_EN_ENABLE                       = 0x00000001,
}


/*
 * DCCG_DEEP_COLOR_CNTL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCCG_DEEP_COLOR_CNTL {
DCCG_DEEP_COLOR_DTO_DISABLE              = 0x00000000,
DCCG_DEEP_COLOR_DTO_5_4_RATIO            = 0x00000001,
DCCG_DEEP_COLOR_DTO_3_2_RATIO            = 0x00000002,
DCCG_DEEP_COLOR_DTO_2_1_RATIO            = 0x00000003,
}


/*
 * DCCG_FIFO_ERRDET_OVR_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCCG_FIFO_ERRDET_OVR_EN {
DCCG_FIFO_ERRDET_OVR_DISABLE             = 0x00000000,
DCCG_FIFO_ERRDET_OVR_ENABLE              = 0x00000001,
}


/*
 * DCCG_FIFO_ERRDET_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCCG_FIFO_ERRDET_RESET {
DCCG_FIFO_ERRDET_RESET_NOOP              = 0x00000000,
DCCG_FIFO_ERRDET_RESET_FORCE             = 0x00000001,
}


/*
 * DCCG_FIFO_ERRDET_STATE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCCG_FIFO_ERRDET_STATE {
DCCG_FIFO_ERRDET_STATE_CALIBRATION       = 0x00000000,
DCCG_FIFO_ERRDET_STATE_DETECTION         = 0x00000001,
}


/*
 * DCCG_PERF_MODE_HSYNC enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCCG_PERF_MODE_HSYNC {
DCCG_PERF_MODE_HSYNC_NOOP                = 0x00000000,
DCCG_PERF_MODE_HSYNC_START               = 0x00000001,
}


/*
 * DCCG_PERF_MODE_VSYNC enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCCG_PERF_MODE_VSYNC {
DCCG_PERF_MODE_VSYNC_NOOP                = 0x00000000,
DCCG_PERF_MODE_VSYNC_START               = 0x00000001,
}


/*
 * DCCG_PERF_OTG_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCCG_PERF_OTG_SELECT {
DCCG_PERF_SEL_OTG0                       = 0x00000000,
DCCG_PERF_SEL_OTG1                       = 0x00000001,
DCCG_PERF_SEL_OTG2                       = 0x00000002,
DCCG_PERF_SEL_OTG3                       = 0x00000003,
DCCG_PERF_SEL_RESERVED                   = 0x00000004,
}


/*
 * DCCG_PERF_RUN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCCG_PERF_RUN {
DCCG_PERF_RUN_NOOP                       = 0x00000000,
DCCG_PERF_RUN_START                      = 0x00000001,
}


/*
 * DC_MEM_GLOBAL_PWR_REQ_DIS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DC_MEM_GLOBAL_PWR_REQ_DIS {
DC_MEM_GLOBAL_PWR_REQ_ENABLE             = 0x00000000,
DC_MEM_GLOBAL_PWR_REQ_DISABLE            = 0x00000001,
}


/*
 * DIO_FIFO_ERROR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIO_FIFO_ERROR {
DIO_FIFO_ERROR_00                        = 0x00000000,
DIO_FIFO_ERROR_01                        = 0x00000001,
DIO_FIFO_ERROR_10                        = 0x00000002,
DIO_FIFO_ERROR_11                        = 0x00000003,
}


/*
 * DISABLE_CLOCK_GATING enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DISABLE_CLOCK_GATING {
CLOCK_GATING_ENABLED                     = 0x00000000,
CLOCK_GATING_DISABLED                    = 0x00000001,
}


/*
 * DISABLE_CLOCK_GATING_IN_DCO enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DISABLE_CLOCK_GATING_IN_DCO {
CLOCK_GATING_ENABLED_IN_DCO              = 0x00000000,
CLOCK_GATING_DISABLED_IN_DCO             = 0x00000001,
}


/*
 * DISPCLK_CHG_FWD_CORR_DISABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DISPCLK_CHG_FWD_CORR_DISABLE {
DISPCLK_CHG_FWD_CORR_ENABLE_AT_BEGINNING = 0x00000000,
DISPCLK_CHG_FWD_CORR_DISABLE_AT_BEGINNING = 0x00000001,
}


/*
 * DISPCLK_FREQ_RAMP_DONE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DISPCLK_FREQ_RAMP_DONE {
DISPCLK_FREQ_RAMP_IN_PROGRESS            = 0x00000000,
DISPCLK_FREQ_RAMP_COMPLETED              = 0x00000001,
}


/*
 * DPREFCLK_SRC_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPREFCLK_SRC_SEL {
DPREFCLK_SRC_SEL_CK                      = 0x00000000,
DPREFCLK_SRC_SEL_P0PLL                   = 0x00000001,
DPREFCLK_SRC_SEL_P1PLL                   = 0x00000002,
DPREFCLK_SRC_SEL_P2PLL                   = 0x00000003,
}


/*
 * DP_DTO_DS_DISABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_DTO_DS_DISABLE {
DP_DTO_DESPREAD_DISABLE                  = 0x00000000,
DP_DTO_DESPREAD_ENABLE                   = 0x00000001,
}


/*
 * DS_HW_CAL_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DS_HW_CAL_ENABLE {
DS_HW_CAL_DIS                            = 0x00000000,
DS_HW_CAL_EN                             = 0x00000001,
}


/*
 * DS_JITTER_COUNT_SRC_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DS_JITTER_COUNT_SRC_SEL {
DS_JITTER_COUNT_SRC_SEL0                 = 0x00000000,
DS_JITTER_COUNT_SRC_SEL1                 = 0x00000001,
}


/*
 * DS_REF_SRC enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DS_REF_SRC {
DS_REF_IS_XTALIN                         = 0x00000000,
DS_REF_IS_EXT_GENLOCK                    = 0x00000001,
DS_REF_IS_PCIE                           = 0x00000002,
}


/*
 * DVOACLKC_IN_PHASE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DVOACLKC_IN_PHASE {
DVOACLKC_IN_OPPOSITE_PHASE_WITH_PCLK_DVO = 0x00000000,
DVOACLKC_IN_PHASE_WITH_PCLK_DVO          = 0x00000001,
}


/*
 * DVOACLKC_MVP_IN_PHASE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DVOACLKC_MVP_IN_PHASE {
DVOACLKC_MVP_IN_OPPOSITE_PHASE_WITH_PCLK_DVO = 0x00000000,
DVOACLKC_MVP_IN_PHASE_WITH_PCLK_DVO      = 0x00000001,
}


/*
 * DVOACLKC_MVP_SKEW_PHASE_OVERRIDE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DVOACLKC_MVP_SKEW_PHASE_OVERRIDE {
DVOACLKC_MVP_SKEW_PHASE_OVERRIDE_DISABLE = 0x00000000,
DVOACLKC_MVP_SKEW_PHASE_OVERRIDE_ENABLE  = 0x00000001,
}


/*
 * DVOACLKD_IN_PHASE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DVOACLKD_IN_PHASE {
DVOACLKD_IN_OPPOSITE_PHASE_WITH_PCLK_DVO = 0x00000000,
DVOACLKD_IN_PHASE_WITH_PCLK_DVO          = 0x00000001,
}


/*
 * DVOACLK_COARSE_SKEW_CNTL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DVOACLK_COARSE_SKEW_CNTL {
DVOACLK_COARSE_SKEW_CNTL_NO_ADJUSTMENT   = 0x00000000,
DVOACLK_COARSE_SKEW_CNTL_DELAY_1_STEP    = 0x00000001,
DVOACLK_COARSE_SKEW_CNTL_DELAY_2_STEPS   = 0x00000002,
DVOACLK_COARSE_SKEW_CNTL_DELAY_3_STEPS   = 0x00000003,
DVOACLK_COARSE_SKEW_CNTL_DELAY_4_STEPS   = 0x00000004,
DVOACLK_COARSE_SKEW_CNTL_DELAY_5_STEPS   = 0x00000005,
DVOACLK_COARSE_SKEW_CNTL_DELAY_6_STEPS   = 0x00000006,
DVOACLK_COARSE_SKEW_CNTL_DELAY_7_STEPS   = 0x00000007,
DVOACLK_COARSE_SKEW_CNTL_DELAY_8_STEPS   = 0x00000008,
DVOACLK_COARSE_SKEW_CNTL_DELAY_9_STEPS   = 0x00000009,
DVOACLK_COARSE_SKEW_CNTL_DELAY_10_STEPS  = 0x0000000a,
DVOACLK_COARSE_SKEW_CNTL_DELAY_11_STEPS  = 0x0000000b,
DVOACLK_COARSE_SKEW_CNTL_DELAY_12_STEPS  = 0x0000000c,
DVOACLK_COARSE_SKEW_CNTL_DELAY_13_STEPS  = 0x0000000d,
DVOACLK_COARSE_SKEW_CNTL_DELAY_14_STEPS  = 0x0000000e,
DVOACLK_COARSE_SKEW_CNTL_DELAY_15_STEPS  = 0x0000000f,
DVOACLK_COARSE_SKEW_CNTL_EARLY_1_STEP    = 0x00000010,
DVOACLK_COARSE_SKEW_CNTL_EARLY_2_STEPS   = 0x00000011,
DVOACLK_COARSE_SKEW_CNTL_EARLY_3_STEPS   = 0x00000012,
DVOACLK_COARSE_SKEW_CNTL_EARLY_4_STEPS   = 0x00000013,
DVOACLK_COARSE_SKEW_CNTL_EARLY_5_STEPS   = 0x00000014,
DVOACLK_COARSE_SKEW_CNTL_EARLY_6_STEPS   = 0x00000015,
DVOACLK_COARSE_SKEW_CNTL_EARLY_7_STEPS   = 0x00000016,
DVOACLK_COARSE_SKEW_CNTL_EARLY_8_STEPS   = 0x00000017,
DVOACLK_COARSE_SKEW_CNTL_EARLY_9_STEPS   = 0x00000018,
DVOACLK_COARSE_SKEW_CNTL_EARLY_10_STEPS  = 0x00000019,
DVOACLK_COARSE_SKEW_CNTL_EARLY_11_STEPS  = 0x0000001a,
DVOACLK_COARSE_SKEW_CNTL_EARLY_12_STEPS  = 0x0000001b,
DVOACLK_COARSE_SKEW_CNTL_EARLY_13_STEPS  = 0x0000001c,
DVOACLK_COARSE_SKEW_CNTL_EARLY_14_STEPS  = 0x0000001d,
DVOACLK_COARSE_SKEW_CNTL_EARLY_15_STEPS  = 0x0000001e,
}


/*
 * DVOACLK_FINE_SKEW_CNTL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DVOACLK_FINE_SKEW_CNTL {
DVOACLK_FINE_SKEW_CNTL_NO_ADJUSTMENT     = 0x00000000,
DVOACLK_FINE_SKEW_CNTL_DELAY_1_STEP      = 0x00000001,
DVOACLK_FINE_SKEW_CNTL_DELAY_2_STEPS     = 0x00000002,
DVOACLK_FINE_SKEW_CNTL_DELAY_3_STEPS     = 0x00000003,
DVOACLK_FINE_SKEW_CNTL_EARLY_1_STEP      = 0x00000004,
DVOACLK_FINE_SKEW_CNTL_EARLY_2_STEPS     = 0x00000005,
DVOACLK_FINE_SKEW_CNTL_EARLY_3_STEPS     = 0x00000006,
DVOACLK_FINE_SKEW_CNTL_EARLY_4_STEPS     = 0x00000007,
}


/*
 * DVO_ENABLE_RST enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DVO_ENABLE_RST {
DVO_ENABLE_RST_DISABLE                   = 0x00000000,
DVO_ENABLE_RST_ENABLE                    = 0x00000001,
}


/*
 * ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENABLE {
DISABLE_THE_FEATURE                      = 0x00000000,
ENABLE_THE_FEATURE                       = 0x00000001,
}


/*
 * ENABLE_CLOCK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENABLE_CLOCK {
ENABLE_THE_REFCLK                        = 0x00000000,
ENABLE_THE_FUNC_CLOCK                    = 0x00000001,
}


/*
 * FORCE_DISABLE_CLOCK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FORCE_DISABLE_CLOCK {
NOT_FORCE_THE_CLOCK_DISABLED             = 0x00000000,
FORCE_THE_CLOCK_DISABLED                 = 0x00000001,
}


/*
 * HDMICHARCLK_SRC_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMICHARCLK_SRC_SEL {
HDMICHARCLK_SRC_SEL_UNIPHYA              = 0x00000000,
HDMICHARCLK_SRC_SEL_UNIPHYB              = 0x00000001,
HDMICHARCLK_SRC_SEL_UNIPHYC              = 0x00000002,
HDMICHARCLK_SRC_SEL_UNIPHYD              = 0x00000003,
HDMICHARCLK_SRC_SEL_UNIPHYE              = 0x00000004,
HDMICHARCLK_SRC_SEL_SRC_RESERVED         = 0x00000005,
}


/*
 * HDMISTREAMCLK_DTO_FORCE_DIS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMISTREAMCLK_DTO_FORCE_DIS {
DTO_FORCE_NO_BYPASS                      = 0x00000000,
DTO_FORCE_BYPASS                         = 0x00000001,
}


/*
 * HDMISTREAMCLK_SRC_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMISTREAMCLK_SRC_SEL {
SEL_REFCLK0                              = 0x00000000,
SEL_DTBCLK0                              = 0x00000001,
SEL_DTBCLK1                              = 0x00000002,
}


/*
 * JITTER_REMOVE_DISABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JITTER_REMOVE_DISABLE {
ENABLE_JITTER_REMOVAL                    = 0x00000000,
DISABLE_JITTER_REMOVAL                   = 0x00000001,
}


/*
 * MICROSECOND_TIME_BASE_CLOCK_SOURCE_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MICROSECOND_TIME_BASE_CLOCK_SOURCE_SEL {
MICROSECOND_TIME_BASE_CLOCK_IS_XTALIN    = 0x00000000,
MICROSECOND_TIME_BASE_CLOCK_IS_DCCGREFCLK = 0x00000001,
}


/*
 * MILLISECOND_TIME_BASE_CLOCK_SOURCE_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MILLISECOND_TIME_BASE_CLOCK_SOURCE_SEL {
MILLISECOND_TIME_BASE_CLOCK_IS_XTALIN    = 0x00000000,
MILLISECOND_TIME_BASE_CLOCK_IS_DCCGREFCLK = 0x00000001,
}


/*
 * OTG_ADD_PIXEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_ADD_PIXEL {
OTG_ADD_PIXEL_NOOP                       = 0x00000000,
OTG_ADD_PIXEL_FORCE                      = 0x00000001,
}


/*
 * OTG_DROP_PIXEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OTG_DROP_PIXEL {
OTG_DROP_PIXEL_NOOP                      = 0x00000000,
OTG_DROP_PIXEL_FORCE                     = 0x00000001,
}


/*
 * PHYSYMCLK_FORCE_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PHYSYMCLK_FORCE_EN {
PHYSYMCLK_FORCE_EN_DISABLE               = 0x00000000,
PHYSYMCLK_FORCE_EN_ENABLE                = 0x00000001,
}


/*
 * PHYSYMCLK_FORCE_SRC_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PHYSYMCLK_FORCE_SRC_SEL {
PHYSYMCLK_FORCE_SRC_SYMCLK               = 0x00000000,
PHYSYMCLK_FORCE_SRC_PHYD18CLK            = 0x00000001,
PHYSYMCLK_FORCE_SRC_PHYD32CLK            = 0x00000002,
}


/*
 * PIPE_PHYPLL_PIXEL_RATE_SOURCE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PIPE_PHYPLL_PIXEL_RATE_SOURCE {
PIPE_PHYPLL_PIXEL_RATE_SOURCE_UNIPHYA    = 0x00000000,
PIPE_PHYPLL_PIXEL_RATE_SOURCE_UNIPHYB    = 0x00000001,
PIPE_PHYPLL_PIXEL_RATE_SOURCE_UNIPHYC    = 0x00000002,
PIPE_PHYPLL_PIXEL_RATE_SOURCE_UNIPHYD    = 0x00000003,
PIPE_PHYPLL_PIXEL_RATE_SOURCE_RESERVED   = 0x00000004,
}


/*
 * PIPE_PIXEL_RATE_PLL_SOURCE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PIPE_PIXEL_RATE_PLL_SOURCE {
PIPE_PIXEL_RATE_PLL_SOURCE_PHYPLL        = 0x00000000,
PIPE_PIXEL_RATE_PLL_SOURCE_DISPPLL       = 0x00000001,
}


/*
 * PIPE_PIXEL_RATE_SOURCE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PIPE_PIXEL_RATE_SOURCE {
PIPE_PIXEL_RATE_SOURCE_P0PLL             = 0x00000000,
PIPE_PIXEL_RATE_SOURCE_P1PLL             = 0x00000001,
PIPE_PIXEL_RATE_SOURCE_P2PLL             = 0x00000002,
}


/*
 * PLL_CFG_IF_SOFT_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PLL_CFG_IF_SOFT_RESET {
PLL_CFG_IF_SOFT_RESET_NOOP               = 0x00000000,
PLL_CFG_IF_SOFT_RESET_FORCE              = 0x00000001,
}


/*
 * SYMCLK_FE_FORCE_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SYMCLK_FE_FORCE_EN {
SYMCLK_FE_FORCE_EN_DISABLE               = 0x00000000,
SYMCLK_FE_FORCE_EN_ENABLE                = 0x00000001,
}


/*
 * SYMCLK_FE_FORCE_SRC enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SYMCLK_FE_FORCE_SRC {
SYMCLK_FE_FORCE_SRC_UNIPHYA              = 0x00000000,
SYMCLK_FE_FORCE_SRC_UNIPHYB              = 0x00000001,
SYMCLK_FE_FORCE_SRC_UNIPHYC              = 0x00000002,
SYMCLK_FE_FORCE_SRC_UNIPHYD              = 0x00000003,
SYMCLK_FE_FORCE_SRC_RESERVED             = 0x00000004,
}


/*
 * TEST_CLK_DIV_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TEST_CLK_DIV_SEL {
NO_DIV                                   = 0x00000000,
DIV_2                                    = 0x00000001,
DIV_4                                    = 0x00000002,
DIV_8                                    = 0x00000003,
}


/*
 * VSYNC_CNT_LATCH_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VSYNC_CNT_LATCH_MASK {
VSYNC_CNT_LATCH_MASK_0                   = 0x00000000,
VSYNC_CNT_LATCH_MASK_1                   = 0x00000001,
}


/*
 * VSYNC_CNT_RESET_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VSYNC_CNT_RESET_SEL {
VSYNC_CNT_RESET_SEL_0                    = 0x00000000,
VSYNC_CNT_RESET_SEL_1                    = 0x00000001,
}


/*
 * XTAL_REF_CLOCK_SOURCE_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum XTAL_REF_CLOCK_SOURCE_SEL {
XTAL_REF_CLOCK_SOURCE_SEL_XTALIN         = 0x00000000,
XTAL_REF_CLOCK_SOURCE_SEL_DCCGREFCLK     = 0x00000001,
}


/*
 * XTAL_REF_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum XTAL_REF_SEL {
XTAL_REF_SEL_1X                          = 0x00000000,
XTAL_REF_SEL_2X                          = 0x00000001,
}


/*******************************************************
 * HPD Enums
 *******************************************************/

/*
 * HPD_INT_CONTROL_ACK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HPD_INT_CONTROL_ACK {
HPD_INT_CONTROL_ACK_0                    = 0x00000000,
HPD_INT_CONTROL_ACK_1                    = 0x00000001,
}


/*
 * HPD_INT_CONTROL_POLARITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HPD_INT_CONTROL_POLARITY {
HPD_INT_CONTROL_GEN_INT_ON_DISCON        = 0x00000000,
HPD_INT_CONTROL_GEN_INT_ON_CON           = 0x00000001,
}


/*
 * HPD_INT_CONTROL_RX_INT_ACK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HPD_INT_CONTROL_RX_INT_ACK {
HPD_INT_CONTROL_RX_INT_ACK_0             = 0x00000000,
HPD_INT_CONTROL_RX_INT_ACK_1             = 0x00000001,
}


/*******************************************************
 * DP Enums
 *******************************************************/

/*
 * DPHY_8B10B_CUR_DISP enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_8B10B_CUR_DISP {
DPHY_8B10B_CUR_DISP_ZERO                 = 0x00000000,
DPHY_8B10B_CUR_DISP_ONE                  = 0x00000001,
}


/*
 * DPHY_8B10B_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_8B10B_RESET {
DPHY_8B10B_NOT_RESET                     = 0x00000000,
DPHY_8B10B_RESETET                       = 0x00000001,
}


/*
 * DPHY_ALT_SCRAMBLER_RESET_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_ALT_SCRAMBLER_RESET_EN {
DPHY_ALT_SCRAMBLER_REGULAR_RESET_VALUE   = 0x00000000,
DPHY_ALT_SCRAMBLER_INTERNAL_RESET_SOLUTION = 0x00000001,
}


/*
 * DPHY_ALT_SCRAMBLER_RESET_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_ALT_SCRAMBLER_RESET_SEL {
DPHY_ALT_SCRAMBLER_RESET_SEL_EDP_RESET_VALUE = 0x00000000,
DPHY_ALT_SCRAMBLER_RESET_SEL_CUSTOM_RESET_VALUE = 0x00000001,
}


/*
 * DPHY_ATEST_SEL_LANE0 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_ATEST_SEL_LANE0 {
DPHY_ATEST_LANE0_PRBS_PATTERN            = 0x00000000,
DPHY_ATEST_LANE0_REG_PATTERN             = 0x00000001,
}


/*
 * DPHY_ATEST_SEL_LANE1 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_ATEST_SEL_LANE1 {
DPHY_ATEST_LANE1_PRBS_PATTERN            = 0x00000000,
DPHY_ATEST_LANE1_REG_PATTERN             = 0x00000001,
}


/*
 * DPHY_ATEST_SEL_LANE2 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_ATEST_SEL_LANE2 {
DPHY_ATEST_LANE2_PRBS_PATTERN            = 0x00000000,
DPHY_ATEST_LANE2_REG_PATTERN             = 0x00000001,
}


/*
 * DPHY_ATEST_SEL_LANE3 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_ATEST_SEL_LANE3 {
DPHY_ATEST_LANE3_PRBS_PATTERN            = 0x00000000,
DPHY_ATEST_LANE3_REG_PATTERN             = 0x00000001,
}


/*
 * DPHY_BYPASS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_BYPASS {
DPHY_8B10B_OUTPUT                        = 0x00000000,
DPHY_DBG_OUTPUT                          = 0x00000001,
}


/*
 * DPHY_CRC_CONT_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_CRC_CONT_EN {
DPHY_CRC_ONE_SHOT                        = 0x00000000,
DPHY_CRC_CONTINUOUS                      = 0x00000001,
}


/*
 * DPHY_CRC_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_CRC_EN {
DPHY_CRC_DISABLED                        = 0x00000000,
DPHY_CRC_ENABLED                         = 0x00000001,
}


/*
 * DPHY_CRC_FIELD enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_CRC_FIELD {
DPHY_CRC_START_FROM_TOP_FIELD            = 0x00000000,
DPHY_CRC_START_FROM_BOTTOM_FIELD         = 0x00000001,
}


/*
 * DPHY_CRC_MST_PHASE_ERROR_ACK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_CRC_MST_PHASE_ERROR_ACK {
DPHY_CRC_MST_PHASE_ERROR_NO_ACK          = 0x00000000,
DPHY_CRC_MST_PHASE_ERROR_ACKED           = 0x00000001,
}


/*
 * DPHY_CRC_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_CRC_SEL {
DPHY_CRC_LANE0_SELECTED                  = 0x00000000,
DPHY_CRC_LANE1_SELECTED                  = 0x00000001,
DPHY_CRC_LANE2_SELECTED                  = 0x00000002,
DPHY_CRC_LANE3_SELECTED                  = 0x00000003,
}


/*
 * DPHY_FEC_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_FEC_ENABLE {
DPHY_FEC_DISABLED                        = 0x00000000,
DPHY_FEC_ENABLED                         = 0x00000001,
}


/*
 * DPHY_FEC_READY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_FEC_READY {
DPHY_FEC_READY_EN                        = 0x00000000,
DPHY_FEC_READY_DIS                       = 0x00000001,
}


/*
 * DPHY_LOAD_BS_COUNT_START enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_LOAD_BS_COUNT_START {
DPHY_LOAD_BS_COUNT_STARTED               = 0x00000000,
DPHY_LOAD_BS_COUNT_NOT_STARTED           = 0x00000001,
}


/*
 * DPHY_PRBS_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_PRBS_EN {
DPHY_PRBS_DISABLE                        = 0x00000000,
DPHY_PRBS_ENABLE                         = 0x00000001,
}


/*
 * DPHY_PRBS_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_PRBS_SEL {
DPHY_PRBS7_SELECTED                      = 0x00000000,
DPHY_PRBS23_SELECTED                     = 0x00000001,
DPHY_PRBS11_SELECTED                     = 0x00000002,
}


/*
 * DPHY_RX_FAST_TRAINING_CAPABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_RX_FAST_TRAINING_CAPABLE {
DPHY_FAST_TRAINING_NOT_CAPABLE_0         = 0x00000000,
DPHY_FAST_TRAINING_CAPABLE               = 0x00000001,
}


/*
 * DPHY_SCRAMBLER_ADVANCE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_SCRAMBLER_ADVANCE {
DPHY_DPHY_SCRAMBLER_ADVANCE_ON_DATA_SYMBOL_ONLY = 0x00000000,
DPHY_SCRAMBLER_ADVANCE_ON_BOTH_DATA_AND_CTRL = 0x00000001,
}


/*
 * DPHY_SCRAMBLER_DIS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_SCRAMBLER_DIS {
DPHY_SCR_ENABLED                         = 0x00000000,
DPHY_SCR_DISABLED                        = 0x00000001,
}


/*
 * DPHY_SCRAMBLER_KCODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_SCRAMBLER_KCODE {
DPHY_SCRAMBLER_KCODE_DISABLED            = 0x00000000,
DPHY_SCRAMBLER_KCODE_ENABLED             = 0x00000001,
}


/*
 * DPHY_SCRAMBLER_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_SCRAMBLER_SEL {
DPHY_SCRAMBLER_SEL_LANE_DATA             = 0x00000000,
DPHY_SCRAMBLER_SEL_DBG_DATA              = 0x00000001,
}


/*
 * DPHY_SKEW_BYPASS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_SKEW_BYPASS {
DPHY_WITH_SKEW                           = 0x00000000,
DPHY_NO_SKEW                             = 0x00000001,
}


/*
 * DPHY_STREAM_RESET_DURING_FAST_TRAINING_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_STREAM_RESET_DURING_FAST_TRAINING_ENUM {
DPHY_STREAM_RESET_DURING_FAST_TRAINING_RESET = 0x00000000,
DPHY_STREAM_RESET_DURING_FAST_TRAINING_NOT_RESET = 0x00000001,
}


/*
 * DPHY_SW_FAST_TRAINING_START enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_SW_FAST_TRAINING_START {
DPHY_SW_FAST_TRAINING_NOT_STARTED        = 0x00000000,
DPHY_SW_FAST_TRAINING_STARTED            = 0x00000001,
}


/*
 * DPHY_TRAINING_PATTERN_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DPHY_TRAINING_PATTERN_SEL {
DPHY_TRAINING_PATTERN_1                  = 0x00000000,
DPHY_TRAINING_PATTERN_2                  = 0x00000001,
DPHY_TRAINING_PATTERN_3                  = 0x00000002,
DPHY_TRAINING_PATTERN_4                  = 0x00000003,
}


/*
 * DP_COMPONENT_DEPTH enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_COMPONENT_DEPTH {
DP_COMPONENT_DEPTH_6BPC                  = 0x00000000,
DP_COMPONENT_DEPTH_8BPC                  = 0x00000001,
DP_COMPONENT_DEPTH_10BPC                 = 0x00000002,
DP_COMPONENT_DEPTH_12BPC                 = 0x00000003,
DP_COMPONENT_DEPTH_16BPC                 = 0x00000004,
}


/*
 * DP_CP_ENCRYPTION_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_CP_ENCRYPTION_TYPE {
DP_CP_ENCRYPTION_TYPE_0                  = 0x00000000,
DP_CP_ENCRYPTION_TYPE_1                  = 0x00000001,
}


/*
 * DP_DPHY_8B10B_EXT_DISP enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_DPHY_8B10B_EXT_DISP {
DP_DPHY_8B10B_EXT_DISP_ZERO              = 0x00000000,
DP_DPHY_8B10B_EXT_DISP_ONE               = 0x00000001,
}


/*
 * DP_DPHY_FAST_TRAINING_COMPLETE_ACK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_DPHY_FAST_TRAINING_COMPLETE_ACK {
DP_DPHY_FAST_TRAINING_COMPLETE_NOT_ACKED = 0x00000000,
DP_DPHY_FAST_TRAINING_COMPLETE_ACKED     = 0x00000001,
}


/*
 * DP_DPHY_FAST_TRAINING_COMPLETE_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_DPHY_FAST_TRAINING_COMPLETE_MASK {
DP_DPHY_FAST_TRAINING_COMPLETE_MASKED    = 0x00000000,
DP_DPHY_FAST_TRAINING_COMPLETE_NOT_MASKED = 0x00000001,
}


/*
 * DP_DPHY_FAST_TRAINING_VBLANK_EDGE_DETECT_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_DPHY_FAST_TRAINING_VBLANK_EDGE_DETECT_EN {
DP_DPHY_FAST_TRAINING_VBLANK_EDGE_DETECT_DISABLED = 0x00000000,
DP_DPHY_FAST_TRAINING_VBLANK_EDGE_DETECT_ENABLED = 0x00000001,
}


/*
 * DP_DPHY_HBR2_PATTERN_CONTROL_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_DPHY_HBR2_PATTERN_CONTROL_MODE {
DP_DPHY_HBR2_PASS_THROUGH                = 0x00000000,
DP_DPHY_HBR2_PATTERN_1                   = 0x00000001,
DP_DPHY_HBR2_PATTERN_2_NEG               = 0x00000002,
DP_DPHY_HBR2_PATTERN_3                   = 0x00000003,
DP_DPHY_HBR2_PATTERN_2_POS               = 0x00000006,
}


/*
 * DP_DSC_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_DSC_MODE {
DP_DSC_DISABLE                           = 0x00000000,
DP_DSC_444_SIMPLE_422                    = 0x00000001,
DP_DSC_NATIVE_422_420                    = 0x00000002,
}


/*
 * DP_EMBEDDED_PANEL_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_EMBEDDED_PANEL_MODE {
DP_EXTERNAL_PANEL                        = 0x00000000,
DP_EMBEDDED_PANEL                        = 0x00000001,
}


/*
 * DP_LINK_TRAINING_COMPLETE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_LINK_TRAINING_COMPLETE {
DP_LINK_TRAINING_NOT_COMPLETE            = 0x00000000,
DP_LINK_TRAINING_ALREADY_COMPLETE        = 0x00000001,
}


/*
 * DP_LINK_TRAINING_SWITCH_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_LINK_TRAINING_SWITCH_MODE {
DP_LINK_TRAINING_SWITCH_TO_IDLE          = 0x00000000,
DP_LINK_TRAINING_SWITCH_TO_VIDEO         = 0x00000001,
}


/*
 * DP_ML_PHY_SEQ_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_ML_PHY_SEQ_MODE {
DP_ML_PHY_SEQ_LINE_NUM                   = 0x00000000,
DP_ML_PHY_SEQ_IMMEDIATE                  = 0x00000001,
}


/*
 * DP_MSA_V_TIMING_OVERRIDE_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_MSA_V_TIMING_OVERRIDE_EN {
MSA_V_TIMING_OVERRIDE_DISABLED           = 0x00000000,
MSA_V_TIMING_OVERRIDE_ENABLED            = 0x00000001,
}


/*
 * DP_MSE_BLANK_CODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_MSE_BLANK_CODE {
DP_MSE_BLANK_CODE_SF_FILLED              = 0x00000000,
DP_MSE_BLANK_CODE_ZERO_FILLED            = 0x00000001,
}


/*
 * DP_MSE_LINK_LINE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_MSE_LINK_LINE {
DP_MSE_LINK_LINE_32_MTP_LONG             = 0x00000000,
DP_MSE_LINK_LINE_64_MTP_LONG             = 0x00000001,
DP_MSE_LINK_LINE_128_MTP_LONG            = 0x00000002,
DP_MSE_LINK_LINE_256_MTP_LONG            = 0x00000003,
}


/*
 * DP_MSE_SAT_ENCRYPT0 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_MSE_SAT_ENCRYPT0 {
DP_MSE_SAT_ENCRYPT0_DISABLED             = 0x00000000,
DP_MSE_SAT_ENCRYPT0_ENABLED              = 0x00000001,
}


/*
 * DP_MSE_SAT_ENCRYPT1 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_MSE_SAT_ENCRYPT1 {
DP_MSE_SAT_ENCRYPT1_DISABLED             = 0x00000000,
DP_MSE_SAT_ENCRYPT1_ENABLED              = 0x00000001,
}


/*
 * DP_MSE_SAT_ENCRYPT2 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_MSE_SAT_ENCRYPT2 {
DP_MSE_SAT_ENCRYPT2_DISABLED             = 0x00000000,
DP_MSE_SAT_ENCRYPT2_ENABLED              = 0x00000001,
}


/*
 * DP_MSE_SAT_ENCRYPT3 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_MSE_SAT_ENCRYPT3 {
DP_MSE_SAT_ENCRYPT3_DISABLED             = 0x00000000,
DP_MSE_SAT_ENCRYPT3_ENABLED              = 0x00000001,
}


/*
 * DP_MSE_SAT_ENCRYPT4 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_MSE_SAT_ENCRYPT4 {
DP_MSE_SAT_ENCRYPT4_DISABLED             = 0x00000000,
DP_MSE_SAT_ENCRYPT4_ENABLED              = 0x00000001,
}


/*
 * DP_MSE_SAT_ENCRYPT5 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_MSE_SAT_ENCRYPT5 {
DP_MSE_SAT_ENCRYPT5_DISABLED             = 0x00000000,
DP_MSE_SAT_ENCRYPT5_ENABLED              = 0x00000001,
}


/*
 * DP_MSE_SAT_UPDATE_ACT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_MSE_SAT_UPDATE_ACT {
DP_MSE_SAT_UPDATE_NO_ACTION              = 0x00000000,
DP_MSE_SAT_UPDATE_WITH_TRIGGER           = 0x00000001,
DP_MSE_SAT_UPDATE_WITHOUT_TRIGGER        = 0x00000002,
}


/*
 * DP_MSE_TIMESTAMP_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_MSE_TIMESTAMP_MODE {
DP_MSE_TIMESTAMP_CALC_BASED_ON_LINK_RATE = 0x00000000,
DP_MSE_TIMESTAMP_CALC_BASED_ON_VC_RATE   = 0x00000001,
}


/*
 * DP_MSE_ZERO_ENCODER enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_MSE_ZERO_ENCODER {
DP_MSE_NOT_ZERO_FE_ENCODER               = 0x00000000,
DP_MSE_ZERO_FE_ENCODER                   = 0x00000001,
}


/*
 * DP_MSO_NUM_OF_SST_LINKS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_MSO_NUM_OF_SST_LINKS {
DP_MSO_ONE_SSTLINK                       = 0x00000000,
DP_MSO_TWO_SSTLINK                       = 0x00000001,
DP_MSO_FOUR_SSTLINK                      = 0x00000002,
}


/*
 * DP_PIXEL_ENCODING enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_PIXEL_ENCODING {
DP_PIXEL_ENCODING_RGB444                 = 0x00000000,
DP_PIXEL_ENCODING_YCBCR422               = 0x00000001,
DP_PIXEL_ENCODING_YCBCR444               = 0x00000002,
DP_PIXEL_ENCODING_RGB_WIDE_GAMUT         = 0x00000003,
DP_PIXEL_ENCODING_Y_ONLY                 = 0x00000004,
DP_PIXEL_ENCODING_YCBCR420               = 0x00000005,
}


/*
 * DP_PIXEL_PER_CYCLE_PROCESSING_NUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_PIXEL_PER_CYCLE_PROCESSING_NUM {
DP_ONE_PIXEL_PER_CYCLE                   = 0x00000000,
DP_TWO_PIXEL_PER_CYCLE                   = 0x00000001,
}


/*
 * DP_SEC_ASP_CHANNEL_COUNT_OVERRIDE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_SEC_ASP_CHANNEL_COUNT_OVERRIDE {
DP_SEC_ASP_CHANNEL_COUNT_FROM_AZ         = 0x00000000,
DP_SEC_ASP_CHANNEL_COUNT_OVERRIDE_ENABLED = 0x00000001,
}


/*
 * DP_SEC_ASP_PRIORITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_SEC_ASP_PRIORITY {
DP_SEC_ASP_LOW_PRIORITY                  = 0x00000000,
DP_SEC_ASP_HIGH_PRIORITY                 = 0x00000001,
}


/*
 * DP_SEC_AUDIO_MUTE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_SEC_AUDIO_MUTE {
DP_SEC_AUDIO_MUTE_HW_CTRL                = 0x00000000,
DP_SEC_AUDIO_MUTE_SW_CTRL                = 0x00000001,
}


/*
 * DP_SEC_COLLISION_ACK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_SEC_COLLISION_ACK {
DP_SEC_COLLISION_ACK_NO_EFFECT           = 0x00000000,
DP_SEC_COLLISION_ACK_CLR_FLAG            = 0x00000001,
}


/*
 * DP_SEC_GSP0_PRIORITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_SEC_GSP0_PRIORITY {
SEC_GSP0_PRIORITY_LOW                    = 0x00000000,
SEC_GSP0_PRIORITY_HIGH                   = 0x00000001,
}


/*
 * DP_SEC_GSP_SEND enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_SEC_GSP_SEND {
NOT_SENT                                 = 0x00000000,
FORCE_SENT                               = 0x00000001,
}


/*
 * DP_SEC_GSP_SEND_ANY_LINE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_SEC_GSP_SEND_ANY_LINE {
SEND_AT_LINK_NUMBER                      = 0x00000000,
SEND_AT_EARLIEST_TIME                    = 0x00000001,
}


/*
 * DP_SEC_GSP_SEND_PPS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_SEC_GSP_SEND_PPS {
SEND_NORMAL_PACKET                       = 0x00000000,
SEND_PPS_PACKET                          = 0x00000001,
}


/*
 * DP_SEC_LINE_REFERENCE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_SEC_LINE_REFERENCE {
REFER_TO_DP_SOF                          = 0x00000000,
REFER_TO_OTG_SOF                         = 0x00000001,
}


/*
 * DP_SEC_TIMESTAMP_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_SEC_TIMESTAMP_MODE {
DP_SEC_TIMESTAMP_PROGRAMMABLE_MODE       = 0x00000000,
DP_SEC_TIMESTAMP_AUTO_CALC_MODE          = 0x00000001,
}


/*
 * DP_STEER_OVERFLOW_ACK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_STEER_OVERFLOW_ACK {
DP_STEER_OVERFLOW_ACK_NO_EFFECT          = 0x00000000,
DP_STEER_OVERFLOW_ACK_CLR_INTERRUPT      = 0x00000001,
}


/*
 * DP_STEER_OVERFLOW_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_STEER_OVERFLOW_MASK {
DP_STEER_OVERFLOW_MASKED                 = 0x00000000,
DP_STEER_OVERFLOW_UNMASK                 = 0x00000001,
}


/*
 * DP_SYNC_POLARITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_SYNC_POLARITY {
DP_SYNC_POLARITY_ACTIVE_HIGH             = 0x00000000,
DP_SYNC_POLARITY_ACTIVE_LOW              = 0x00000001,
}


/*
 * DP_TU_OVERFLOW_ACK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_TU_OVERFLOW_ACK {
DP_TU_OVERFLOW_ACK_NO_EFFECT             = 0x00000000,
DP_TU_OVERFLOW_ACK_CLR_INTERRUPT         = 0x00000001,
}


/*
 * DP_UDI_LANES enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_UDI_LANES {
DP_UDI_1_LANE                            = 0x00000000,
DP_UDI_2_LANES                           = 0x00000001,
DP_UDI_LANES_RESERVED                    = 0x00000002,
DP_UDI_4_LANES                           = 0x00000003,
}


/*
 * DP_VID_ENHANCED_FRAME_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_VID_ENHANCED_FRAME_MODE {
VID_NORMAL_FRAME_MODE                    = 0x00000000,
VID_ENHANCED_MODE                        = 0x00000001,
}


/*
 * DP_VID_M_N_DOUBLE_BUFFER_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_VID_M_N_DOUBLE_BUFFER_MODE {
DP_VID_M_N_DOUBLE_BUFFER_AFTER_VID_M_UPDATE = 0x00000000,
DP_VID_M_N_DOUBLE_BUFFER_AT_FRAME_START  = 0x00000001,
}


/*
 * DP_VID_M_N_GEN_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_VID_M_N_GEN_EN {
DP_VID_M_N_PROGRAMMED_VIA_REG            = 0x00000000,
DP_VID_M_N_CALC_AUTO                     = 0x00000001,
}


/*
 * DP_VID_N_MUL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_VID_N_MUL {
DP_VID_M_1X_INPUT_PIXEL_RATE             = 0x00000000,
DP_VID_M_2X_INPUT_PIXEL_RATE             = 0x00000001,
DP_VID_M_4X_INPUT_PIXEL_RATE             = 0x00000002,
DP_VID_M_8X_INPUT_PIXEL_RATE             = 0x00000003,
}


/*
 * DP_VID_STREAM_DISABLE_ACK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_VID_STREAM_DISABLE_ACK {
ID_STREAM_DISABLE_NO_ACK                 = 0x00000000,
ID_STREAM_DISABLE_ACKED                  = 0x00000001,
}


/*
 * DP_VID_STREAM_DISABLE_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_VID_STREAM_DISABLE_MASK {
VID_STREAM_DISABLE_MASKED                = 0x00000000,
VID_STREAM_DISABLE_UNMASK                = 0x00000001,
}


/*
 * DP_VID_STREAM_DIS_DEFER enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_VID_STREAM_DIS_DEFER {
DP_VID_STREAM_DIS_NO_DEFER               = 0x00000000,
DP_VID_STREAM_DIS_DEFER_TO_HBLANK        = 0x00000001,
DP_VID_STREAM_DIS_DEFER_TO_VBLANK        = 0x00000002,
}


/*
 * DP_VID_VBID_FIELD_POL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_VID_VBID_FIELD_POL {
DP_VID_VBID_FIELD_POL_NORMAL             = 0x00000000,
DP_VID_VBID_FIELD_POL_INV                = 0x00000001,
}


/*
 * FEC_ACTIVE_STATUS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FEC_ACTIVE_STATUS {
DPHY_FEC_NOT_ACTIVE                      = 0x00000000,
DPHY_FEC_ACTIVE                          = 0x00000001,
}


/*******************************************************
 * DIG Enums
 *******************************************************/

/*
 * DIG_BE_CNTL_HPD_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_BE_CNTL_HPD_SELECT {
DIG_BE_CNTL_HPD1                         = 0x00000000,
DIG_BE_CNTL_HPD2                         = 0x00000001,
DIG_BE_CNTL_HPD3                         = 0x00000002,
DIG_BE_CNTL_HPD4                         = 0x00000003,
DIG_BE_CNTL_HPD5                         = 0x00000004,
DIG_BE_CNTL_NO_HPD                       = 0x00000005,
}


/*
 * DIG_BE_CNTL_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_BE_CNTL_MODE {
DIG_BE_DP_SST_MODE                       = 0x00000000,
DIG_BE_RESERVED1                         = 0x00000001,
DIG_BE_TMDS_DVI_MODE                     = 0x00000002,
DIG_BE_TMDS_HDMI_MODE                    = 0x00000003,
DIG_BE_RESERVED4                         = 0x00000004,
DIG_BE_DP_MST_MODE                       = 0x00000005,
DIG_BE_RESERVED2                         = 0x00000006,
DIG_BE_RESERVED3                         = 0x00000007,
}


/*
 * DIG_DIGITAL_BYPASS_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_DIGITAL_BYPASS_ENABLE {
DIG_DIGITAL_BYPASS_OFF                   = 0x00000000,
DIG_DIGITAL_BYPASS_ON                    = 0x00000001,
}


/*
 * DIG_DIGITAL_BYPASS_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_DIGITAL_BYPASS_SEL {
DIG_DIGITAL_BYPASS_SEL_BYPASS            = 0x00000000,
DIG_DIGITAL_BYPASS_SEL_36BPP             = 0x00000001,
DIG_DIGITAL_BYPASS_SEL_48BPP_LSB         = 0x00000002,
DIG_DIGITAL_BYPASS_SEL_48BPP_MSB         = 0x00000003,
DIG_DIGITAL_BYPASS_SEL_10BPP_LSB         = 0x00000004,
DIG_DIGITAL_BYPASS_SEL_12BPC_LSB         = 0x00000005,
DIG_DIGITAL_BYPASS_SEL_ALPHA             = 0x00000006,
}


/*
 * DIG_FE_CNTL_SOURCE_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_FE_CNTL_SOURCE_SELECT {
DIG_FE_SOURCE_FROM_OTG0                  = 0x00000000,
DIG_FE_SOURCE_FROM_OTG1                  = 0x00000001,
DIG_FE_SOURCE_FROM_OTG2                  = 0x00000002,
DIG_FE_SOURCE_FROM_OTG3                  = 0x00000003,
DIG_FE_SOURCE_RESERVED                   = 0x00000004,
}


/*
 * DIG_FE_CNTL_STEREOSYNC_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_FE_CNTL_STEREOSYNC_SELECT {
DIG_FE_STEREOSYNC_FROM_OTG0              = 0x00000000,
DIG_FE_STEREOSYNC_FROM_OTG1              = 0x00000001,
DIG_FE_STEREOSYNC_FROM_OTG2              = 0x00000002,
DIG_FE_STEREOSYNC_FROM_OTG3              = 0x00000003,
DIG_FE_STEREOSYNC_RESERVED               = 0x00000004,
}


/*
 * DIG_FIFO_CTRL_FORCE_RECOMP_MINMAX enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_FIFO_CTRL_FORCE_RECOMP_MINMAX {
DIG_FIFO_NOT_FORCE_RECOMP_MINMAX         = 0x00000000,
DIG_FIFO_FORCE_RECOMP_MINMAX             = 0x00000001,
}


/*
 * DIG_FIFO_CTRL_USE_OVERWRITE_LEVEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_FIFO_CTRL_USE_OVERWRITE_LEVEL {
DIG_FIFO_USE_OVERWRITE_LEVEL             = 0x00000000,
DIG_FIFO_USE_CAL_AVERAGE_LEVEL           = 0x00000001,
}


/*
 * DIG_FIFO_FORCE_RECAL_AVERAGE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_FIFO_FORCE_RECAL_AVERAGE {
DIG_FIFO_NOT_FORCE_RECAL_AVERAGE         = 0x00000000,
DIG_FIFO_FORCE_RECAL_AVERAGE_LEVEL       = 0x00000001,
}


/*
 * DIG_FIFO_OUTPUT_PROCESSING_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_FIFO_OUTPUT_PROCESSING_MODE {
DIG_FIFO_1_PIX_PER_CYCLE                 = 0x00000000,
DIG_FIFO_2_PIX_PER_CYCLE                 = 0x00000001,
}


/*
 * DIG_FIFO_OVERFLOW_UNDERFLOW_ERROR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_FIFO_OVERFLOW_UNDERFLOW_ERROR {
DIG_FIFO_NO_ERROR_OCCURRED               = 0x00000000,
DIG_FIFO_UNDERFLOW_OCCURRED              = 0x00000001,
DIG_FIFO_OVERFLOW_OCCURRED               = 0x00000002,
}


/*
 * DIG_FIFO_READ_CLOCK_SRC enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_FIFO_READ_CLOCK_SRC {
DIG_FIFO_READ_CLOCK_SRC_FROM_DCCG        = 0x00000000,
DIG_FIFO_READ_CLOCK_SRC_FROM_DISPLAY_PIPE = 0x00000001,
}


/*
 * DIG_INPUT_PIXEL_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_INPUT_PIXEL_SEL {
DIG_ALL_PIXEL                            = 0x00000000,
DIG_EVEN_PIXEL_ONLY                      = 0x00000001,
DIG_ODD_PIXEL_ONLY                       = 0x00000002,
}


/*
 * DIG_OUTPUT_CRC_CNTL_LINK_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_OUTPUT_CRC_CNTL_LINK_SEL {
DIG_OUTPUT_CRC_ON_LINK0                  = 0x00000000,
DIG_OUTPUT_CRC_ON_LINK1                  = 0x00000001,
}


/*
 * DIG_OUTPUT_CRC_DATA_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_OUTPUT_CRC_DATA_SEL {
DIG_OUTPUT_CRC_FOR_FULLFRAME             = 0x00000000,
DIG_OUTPUT_CRC_FOR_ACTIVEONLY            = 0x00000001,
DIG_OUTPUT_CRC_FOR_VBI                   = 0x00000002,
DIG_OUTPUT_CRC_FOR_AUDIO                 = 0x00000003,
}


/*
 * DIG_RANDOM_PATTERN_SEED_RAN_PAT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_RANDOM_PATTERN_SEED_RAN_PAT {
DIG_RANDOM_PATTERN_SEED_RAN_PAT_ALL_PIXELS = 0x00000000,
DIG_RANDOM_PATTERN_SEED_RAN_PAT_DE_HIGH  = 0x00000001,
}


/*
 * DIG_SL_PIXEL_GROUPING enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_SL_PIXEL_GROUPING {
DIG_SINGLETON_PIXELS                     = 0x00000000,
DIG_PAIR_PIXELS                          = 0x00000001,
}


/*
 * DIG_TEST_PATTERN_EXTERNAL_RESET_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_TEST_PATTERN_EXTERNAL_RESET_EN {
DIG_TEST_PATTERN_EXTERNAL_RESET_ENABLE   = 0x00000000,
DIG_TEST_PATTERN_EXTERNAL_RESET_BY_EXT_SIG = 0x00000001,
}


/*
 * DIG_TEST_PATTERN_HALF_CLOCK_PATTERN_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_TEST_PATTERN_HALF_CLOCK_PATTERN_SEL {
DIG_10BIT_TEST_PATTERN                   = 0x00000000,
DIG_ALTERNATING_TEST_PATTERN             = 0x00000001,
}


/*
 * DIG_TEST_PATTERN_RANDOM_PATTERN_OUT_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_TEST_PATTERN_RANDOM_PATTERN_OUT_EN {
DIG_TEST_PATTERN_NORMAL                  = 0x00000000,
DIG_TEST_PATTERN_RANDOM                  = 0x00000001,
}


/*
 * DIG_TEST_PATTERN_RANDOM_PATTERN_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_TEST_PATTERN_RANDOM_PATTERN_RESET {
DIG_RANDOM_PATTERN_ENABLED               = 0x00000000,
DIG_RANDOM_PATTERN_RESETED               = 0x00000001,
}


/*
 * DIG_TEST_PATTERN_TEST_PATTERN_OUT_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIG_TEST_PATTERN_TEST_PATTERN_OUT_EN {
DIG_IN_NORMAL_OPERATION                  = 0x00000000,
DIG_IN_DEBUG_MODE                        = 0x00000001,
}


/*
 * DOLBY_VISION_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOLBY_VISION_ENABLE {
DOLBY_VISION_DISABLED                    = 0x00000000,
DOLBY_VISION_ENABLED                     = 0x00000001,
}


/*
 * HDMI_ACP_SEND enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_ACP_SEND {
HDMI_ACP_NOT_SEND                        = 0x00000000,
HDMI_ACP_PKT_SEND                        = 0x00000001,
}


/*
 * HDMI_ACR_AUDIO_PRIORITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_ACR_AUDIO_PRIORITY {
HDMI_ACR_PKT_HIGH_PRIORITY_THAN_AUDIO_SAMPLE = 0x00000000,
HDMI_AUDIO_SAMPLE_HIGH_PRIORITY_THAN_ACR_PKT = 0x00000001,
}


/*
 * HDMI_ACR_CONT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_ACR_CONT {
HDMI_ACR_CONT_DISABLE                    = 0x00000000,
HDMI_ACR_CONT_ENABLE                     = 0x00000001,
}


/*
 * HDMI_ACR_N_MULTIPLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_ACR_N_MULTIPLE {
HDMI_ACR_0_MULTIPLE_RESERVED             = 0x00000000,
HDMI_ACR_1_MULTIPLE                      = 0x00000001,
HDMI_ACR_2_MULTIPLE                      = 0x00000002,
HDMI_ACR_3_MULTIPLE_RESERVED             = 0x00000003,
HDMI_ACR_4_MULTIPLE                      = 0x00000004,
HDMI_ACR_5_MULTIPLE_RESERVED             = 0x00000005,
HDMI_ACR_6_MULTIPLE_RESERVED             = 0x00000006,
HDMI_ACR_7_MULTIPLE_RESERVED             = 0x00000007,
}


/*
 * HDMI_ACR_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_ACR_SELECT {
HDMI_ACR_SELECT_HW                       = 0x00000000,
HDMI_ACR_SELECT_32K                      = 0x00000001,
HDMI_ACR_SELECT_44K                      = 0x00000002,
HDMI_ACR_SELECT_48K                      = 0x00000003,
}


/*
 * HDMI_ACR_SEND enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_ACR_SEND {
HDMI_ACR_NOT_SEND                        = 0x00000000,
HDMI_ACR_PKT_SEND                        = 0x00000001,
}


/*
 * HDMI_ACR_SOURCE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_ACR_SOURCE {
HDMI_ACR_SOURCE_HW                       = 0x00000000,
HDMI_ACR_SOURCE_SW                       = 0x00000001,
}


/*
 * HDMI_AUDIO_DELAY_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_AUDIO_DELAY_EN {
HDMI_AUDIO_DELAY_DISABLE                 = 0x00000000,
HDMI_AUDIO_DELAY_58CLK                   = 0x00000001,
HDMI_AUDIO_DELAY_56CLK                   = 0x00000002,
HDMI_AUDIO_DELAY_RESERVED                = 0x00000003,
}


/*
 * HDMI_AUDIO_INFO_CONT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_AUDIO_INFO_CONT {
HDMI_AUDIO_INFO_CONT_DISABLE             = 0x00000000,
HDMI_AUDIO_INFO_CONT_ENABLE              = 0x00000001,
}


/*
 * HDMI_AUDIO_INFO_SEND enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_AUDIO_INFO_SEND {
HDMI_AUDIO_INFO_NOT_SEND                 = 0x00000000,
HDMI_AUDIO_INFO_PKT_SEND                 = 0x00000001,
}


/*
 * HDMI_CLOCK_CHANNEL_RATE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_CLOCK_CHANNEL_RATE {
HDMI_CLOCK_CHANNEL_FREQ_EQUAL_TO_CHAR_RATE = 0x00000000,
HDMI_CLOCK_CHANNEL_FREQ_QUARTER_TO_CHAR_RATE = 0x00000001,
}


/*
 * HDMI_DATA_SCRAMBLE_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_DATA_SCRAMBLE_EN {
HDMI_DATA_SCRAMBLE_DISABLE               = 0x00000000,
HDMI_DATA_SCRAMBLE_ENABLE                = 0x00000001,
}


/*
 * HDMI_DEEP_COLOR_DEPTH enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_DEEP_COLOR_DEPTH {
HDMI_DEEP_COLOR_DEPTH_24BPP              = 0x00000000,
HDMI_DEEP_COLOR_DEPTH_30BPP              = 0x00000001,
HDMI_DEEP_COLOR_DEPTH_36BPP              = 0x00000002,
HDMI_DEEP_COLOR_DEPTH_48BPP              = 0x00000003,
}


/*
 * HDMI_DEFAULT_PAHSE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_DEFAULT_PAHSE {
HDMI_DEFAULT_PHASE_IS_0                  = 0x00000000,
HDMI_DEFAULT_PHASE_IS_1                  = 0x00000001,
}


/*
 * HDMI_ERROR_ACK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_ERROR_ACK {
HDMI_ERROR_ACK_INT                       = 0x00000000,
HDMI_ERROR_NOT_ACK                       = 0x00000001,
}


/*
 * HDMI_ERROR_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_ERROR_MASK {
HDMI_ERROR_MASK_INT                      = 0x00000000,
HDMI_ERROR_NOT_MASK                      = 0x00000001,
}


/*
 * HDMI_GC_AVMUTE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_GC_AVMUTE {
HDMI_GC_AVMUTE_SET                       = 0x00000000,
HDMI_GC_AVMUTE_UNSET                     = 0x00000001,
}


/*
 * HDMI_GC_AVMUTE_CONT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_GC_AVMUTE_CONT {
HDMI_GC_AVMUTE_CONT_DISABLE              = 0x00000000,
HDMI_GC_AVMUTE_CONT_ENABLE               = 0x00000001,
}


/*
 * HDMI_GC_CONT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_GC_CONT {
HDMI_GC_CONT_DISABLE                     = 0x00000000,
HDMI_GC_CONT_ENABLE                      = 0x00000001,
}


/*
 * HDMI_GC_SEND enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_GC_SEND {
HDMI_GC_NOT_SEND                         = 0x00000000,
HDMI_GC_PKT_SEND                         = 0x00000001,
}


/*
 * HDMI_GENERIC_CONT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_GENERIC_CONT {
HDMI_GENERIC_CONT_DISABLE                = 0x00000000,
HDMI_GENERIC_CONT_ENABLE                 = 0x00000001,
}


/*
 * HDMI_GENERIC_SEND enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_GENERIC_SEND {
HDMI_GENERIC_NOT_SEND                    = 0x00000000,
HDMI_GENERIC_PKT_SEND                    = 0x00000001,
}


/*
 * HDMI_ISRC_CONT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_ISRC_CONT {
HDMI_ISRC_CONT_DISABLE                   = 0x00000000,
HDMI_ISRC_CONT_ENABLE                    = 0x00000001,
}


/*
 * HDMI_ISRC_SEND enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_ISRC_SEND {
HDMI_ISRC_NOT_SEND                       = 0x00000000,
HDMI_ISRC_PKT_SEND                       = 0x00000001,
}


/*
 * HDMI_KEEPOUT_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_KEEPOUT_MODE {
HDMI_KEEPOUT_0_650PIX_AFTER_VSYNC        = 0x00000000,
HDMI_KEEPOUT_509_650PIX_AFTER_VSYNC      = 0x00000001,
}


/*
 * HDMI_METADATA_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_METADATA_ENABLE {
HDMI_METADATA_NOT_SEND                   = 0x00000000,
HDMI_METADATA_PKT_SEND                   = 0x00000001,
}


/*
 * HDMI_MPEG_INFO_CONT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_MPEG_INFO_CONT {
HDMI_MPEG_INFO_CONT_DISABLE              = 0x00000000,
HDMI_MPEG_INFO_CONT_ENABLE               = 0x00000001,
}


/*
 * HDMI_MPEG_INFO_SEND enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_MPEG_INFO_SEND {
HDMI_MPEG_INFO_NOT_SEND                  = 0x00000000,
HDMI_MPEG_INFO_PKT_SEND                  = 0x00000001,
}


/*
 * HDMI_NO_EXTRA_NULL_PACKET_FILLED enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_NO_EXTRA_NULL_PACKET_FILLED {
HDMI_EXTRA_NULL_PACKET_FILLED_ENABLE     = 0x00000000,
HDMI_EXTRA_NULL_PACKET_FILLED_DISABLE    = 0x00000001,
}


/*
 * HDMI_NULL_SEND enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_NULL_SEND {
HDMI_NULL_NOT_SEND                       = 0x00000000,
HDMI_NULL_PKT_SEND                       = 0x00000001,
}


/*
 * HDMI_PACKET_GEN_VERSION enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_PACKET_GEN_VERSION {
HDMI_PACKET_GEN_VERSION_OLD              = 0x00000000,
HDMI_PACKET_GEN_VERSION_NEW              = 0x00000001,
}


/*
 * HDMI_PACKET_LINE_REFERENCE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_PACKET_LINE_REFERENCE {
HDMI_PKT_LINE_REF_VSYNC                  = 0x00000000,
HDMI_PKT_LINE_REF_OTGSOF                 = 0x00000001,
}


/*
 * HDMI_PACKING_PHASE_OVERRIDE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_PACKING_PHASE_OVERRIDE {
HDMI_PACKING_PHASE_SET_BY_HW             = 0x00000000,
HDMI_PACKING_PHASE_SET_BY_SW             = 0x00000001,
}


/*
 * LVTMA_RANDOM_PATTERN_SEED_RAN_PAT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LVTMA_RANDOM_PATTERN_SEED_RAN_PAT {
LVTMA_RANDOM_PATTERN_SEED_ALL_PIXELS     = 0x00000000,
LVTMA_RANDOM_PATTERN_SEED_ONLY_DE_HIGH   = 0x00000001,
}


/*
 * TMDS_COLOR_FORMAT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_COLOR_FORMAT {
TMDS_COLOR_FORMAT__24BPP__TWIN30BPP_MSB__DUAL48BPP = 0x00000000,
TMDS_COLOR_FORMAT_TWIN30BPP_LSB          = 0x00000001,
TMDS_COLOR_FORMAT_DUAL30BPP              = 0x00000002,
TMDS_COLOR_FORMAT_RESERVED               = 0x00000003,
}


/*
 * TMDS_CTL0_DATA_INVERT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_CTL0_DATA_INVERT {
TMDS_CTL0_DATA_NORMAL                    = 0x00000000,
TMDS_CTL0_DATA_INVERT_EN                 = 0x00000001,
}


/*
 * TMDS_CTL0_DATA_MODULATION enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_CTL0_DATA_MODULATION {
TMDS_CTL0_DATA_MODULATION_DISABLE        = 0x00000000,
TMDS_CTL0_DATA_MODULATION_BIT0           = 0x00000001,
TMDS_CTL0_DATA_MODULATION_BIT1           = 0x00000002,
TMDS_CTL0_DATA_MODULATION_BIT2           = 0x00000003,
}


/*
 * TMDS_CTL0_DATA_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_CTL0_DATA_SEL {
TMDS_CTL0_DATA_SEL0_RESERVED             = 0x00000000,
TMDS_CTL0_DATA_SEL1_DISPLAY_ENABLE       = 0x00000001,
TMDS_CTL0_DATA_SEL2_VSYNC                = 0x00000002,
TMDS_CTL0_DATA_SEL3_RESERVED             = 0x00000003,
TMDS_CTL0_DATA_SEL4_HSYNC                = 0x00000004,
TMDS_CTL0_DATA_SEL5_SEL7_RESERVED        = 0x00000005,
TMDS_CTL0_DATA_SEL8_RANDOM_DATA          = 0x00000006,
TMDS_CTL0_DATA_SEL9_SEL15_RANDOM_DATA    = 0x00000007,
}


/*
 * TMDS_CTL0_PATTERN_OUT_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_CTL0_PATTERN_OUT_EN {
TMDS_CTL0_PATTERN_OUT_DISABLE            = 0x00000000,
TMDS_CTL0_PATTERN_OUT_ENABLE             = 0x00000001,
}


/*
 * TMDS_CTL1_DATA_INVERT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_CTL1_DATA_INVERT {
TMDS_CTL1_DATA_NORMAL                    = 0x00000000,
TMDS_CTL1_DATA_INVERT_EN                 = 0x00000001,
}


/*
 * TMDS_CTL1_DATA_MODULATION enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_CTL1_DATA_MODULATION {
TMDS_CTL1_DATA_MODULATION_DISABLE        = 0x00000000,
TMDS_CTL1_DATA_MODULATION_BIT0           = 0x00000001,
TMDS_CTL1_DATA_MODULATION_BIT1           = 0x00000002,
TMDS_CTL1_DATA_MODULATION_BIT2           = 0x00000003,
}


/*
 * TMDS_CTL1_DATA_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_CTL1_DATA_SEL {
TMDS_CTL1_DATA_SEL0_RESERVED             = 0x00000000,
TMDS_CTL1_DATA_SEL1_DISPLAY_ENABLE       = 0x00000001,
TMDS_CTL1_DATA_SEL2_VSYNC                = 0x00000002,
TMDS_CTL1_DATA_SEL3_RESERVED             = 0x00000003,
TMDS_CTL1_DATA_SEL4_HSYNC                = 0x00000004,
TMDS_CTL1_DATA_SEL5_SEL7_RESERVED        = 0x00000005,
TMDS_CTL1_DATA_SEL8_BLANK_TIME           = 0x00000006,
TMDS_CTL1_DATA_SEL9_SEL15_RESERVED       = 0x00000007,
}


/*
 * TMDS_CTL1_PATTERN_OUT_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_CTL1_PATTERN_OUT_EN {
TMDS_CTL1_PATTERN_OUT_DISABLE            = 0x00000000,
TMDS_CTL1_PATTERN_OUT_ENABLE             = 0x00000001,
}


/*
 * TMDS_CTL2_DATA_INVERT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_CTL2_DATA_INVERT {
TMDS_CTL2_DATA_NORMAL                    = 0x00000000,
TMDS_CTL2_DATA_INVERT_EN                 = 0x00000001,
}


/*
 * TMDS_CTL2_DATA_MODULATION enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_CTL2_DATA_MODULATION {
TMDS_CTL2_DATA_MODULATION_DISABLE        = 0x00000000,
TMDS_CTL2_DATA_MODULATION_BIT0           = 0x00000001,
TMDS_CTL2_DATA_MODULATION_BIT1           = 0x00000002,
TMDS_CTL2_DATA_MODULATION_BIT2           = 0x00000003,
}


/*
 * TMDS_CTL2_DATA_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_CTL2_DATA_SEL {
TMDS_CTL2_DATA_SEL0_RESERVED             = 0x00000000,
TMDS_CTL2_DATA_SEL1_DISPLAY_ENABLE       = 0x00000001,
TMDS_CTL2_DATA_SEL2_VSYNC                = 0x00000002,
TMDS_CTL2_DATA_SEL3_RESERVED             = 0x00000003,
TMDS_CTL2_DATA_SEL4_HSYNC                = 0x00000004,
TMDS_CTL2_DATA_SEL5_SEL7_RESERVED        = 0x00000005,
TMDS_CTL2_DATA_SEL8_BLANK_TIME           = 0x00000006,
TMDS_CTL2_DATA_SEL9_SEL15_RESERVED       = 0x00000007,
}


/*
 * TMDS_CTL2_PATTERN_OUT_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_CTL2_PATTERN_OUT_EN {
TMDS_CTL2_PATTERN_OUT_DISABLE            = 0x00000000,
TMDS_CTL2_PATTERN_OUT_ENABLE             = 0x00000001,
}


/*
 * TMDS_CTL3_DATA_INVERT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_CTL3_DATA_INVERT {
TMDS_CTL3_DATA_NORMAL                    = 0x00000000,
TMDS_CTL3_DATA_INVERT_EN                 = 0x00000001,
}


/*
 * TMDS_CTL3_DATA_MODULATION enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_CTL3_DATA_MODULATION {
TMDS_CTL3_DATA_MODULATION_DISABLE        = 0x00000000,
TMDS_CTL3_DATA_MODULATION_BIT0           = 0x00000001,
TMDS_CTL3_DATA_MODULATION_BIT1           = 0x00000002,
TMDS_CTL3_DATA_MODULATION_BIT2           = 0x00000003,
}


/*
 * TMDS_CTL3_DATA_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_CTL3_DATA_SEL {
TMDS_CTL3_DATA_SEL0_RESERVED             = 0x00000000,
TMDS_CTL3_DATA_SEL1_DISPLAY_ENABLE       = 0x00000001,
TMDS_CTL3_DATA_SEL2_VSYNC                = 0x00000002,
TMDS_CTL3_DATA_SEL3_RESERVED             = 0x00000003,
TMDS_CTL3_DATA_SEL4_HSYNC                = 0x00000004,
TMDS_CTL3_DATA_SEL5_SEL7_RESERVED        = 0x00000005,
TMDS_CTL3_DATA_SEL8_BLANK_TIME           = 0x00000006,
TMDS_CTL3_DATA_SEL9_SEL15_RESERVED       = 0x00000007,
}


/*
 * TMDS_CTL3_PATTERN_OUT_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_CTL3_PATTERN_OUT_EN {
TMDS_CTL3_PATTERN_OUT_DISABLE            = 0x00000000,
TMDS_CTL3_PATTERN_OUT_ENABLE             = 0x00000001,
}


/*
 * TMDS_DATA_SYNCHRONIZATION_DSINTSEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_DATA_SYNCHRONIZATION_DSINTSEL {
TMDS_DATA_SYNCHRONIZATION_DSINTSEL_PCLK_TMDS = 0x00000000,
TMDS_DATA_SYNCHRONIZATION_DSINTSEL_TMDS_PLL = 0x00000001,
}


/*
 * TMDS_PIXEL_ENCODING enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_PIXEL_ENCODING {
TMDS_PIXEL_ENCODING_444_OR_420           = 0x00000000,
TMDS_PIXEL_ENCODING_422                  = 0x00000001,
}


/*
 * TMDS_REG_TEST_OUTPUTA_CNTLA enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_REG_TEST_OUTPUTA_CNTLA {
TMDS_REG_TEST_OUTPUTA_CNTLA_OTDATA0      = 0x00000000,
TMDS_REG_TEST_OUTPUTA_CNTLA_OTDATA1      = 0x00000001,
TMDS_REG_TEST_OUTPUTA_CNTLA_OTDATA2      = 0x00000002,
TMDS_REG_TEST_OUTPUTA_CNTLA_NA           = 0x00000003,
}


/*
 * TMDS_REG_TEST_OUTPUTB_CNTLB enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_REG_TEST_OUTPUTB_CNTLB {
TMDS_REG_TEST_OUTPUTB_CNTLB_OTDATB0      = 0x00000000,
TMDS_REG_TEST_OUTPUTB_CNTLB_OTDATB1      = 0x00000001,
TMDS_REG_TEST_OUTPUTB_CNTLB_OTDATB2      = 0x00000002,
TMDS_REG_TEST_OUTPUTB_CNTLB_NA           = 0x00000003,
}


/*
 * TMDS_STEREOSYNC_CTL_SEL_REG enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_STEREOSYNC_CTL_SEL_REG {
TMDS_STEREOSYNC_CTL0                     = 0x00000000,
TMDS_STEREOSYNC_CTL1                     = 0x00000001,
TMDS_STEREOSYNC_CTL2                     = 0x00000002,
TMDS_STEREOSYNC_CTL3                     = 0x00000003,
}


/*
 * TMDS_SYNC_PHASE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_SYNC_PHASE {
TMDS_NOT_SYNC_PHASE_ON_FRAME_START       = 0x00000000,
TMDS_SYNC_PHASE_ON_FRAME_START           = 0x00000001,
}


/*
 * TMDS_TRANSMITTER_CONTROL_BYPASS_PLLA enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_TRANSMITTER_CONTROL_BYPASS_PLLA {
TMDS_TRANSMITTER_BYPASS_PLLA_COHERENT    = 0x00000000,
TMDS_TRANSMITTER_BYPASS_PLLA_INCOHERENT  = 0x00000001,
}


/*
 * TMDS_TRANSMITTER_CONTROL_BYPASS_PLLB enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_TRANSMITTER_CONTROL_BYPASS_PLLB {
TMDS_TRANSMITTER_BYPASS_PLLB_COHERENT    = 0x00000000,
TMDS_TRANSMITTER_BYPASS_PLLB_INCOHERENT  = 0x00000001,
}


/*
 * TMDS_TRANSMITTER_CONTROL_IDSCKSELA enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_TRANSMITTER_CONTROL_IDSCKSELA {
TMDS_TRANSMITTER_IDSCKSELA_USE_IPIXCLK   = 0x00000000,
TMDS_TRANSMITTER_IDSCKSELA_USE_IDCLK     = 0x00000001,
}


/*
 * TMDS_TRANSMITTER_CONTROL_IDSCKSELB enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_TRANSMITTER_CONTROL_IDSCKSELB {
TMDS_TRANSMITTER_IDSCKSELB_USE_IPIXCLK   = 0x00000000,
TMDS_TRANSMITTER_IDSCKSELB_USE_IDCLK     = 0x00000001,
}


/*
 * TMDS_TRANSMITTER_CONTROL_PLLSEL_OVERWRITE_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_TRANSMITTER_CONTROL_PLLSEL_OVERWRITE_EN {
TMDS_TRANSMITTER_PLLSEL_BY_HW            = 0x00000000,
TMDS_TRANSMITTER_PLLSEL_OVERWRITE_BY_SW  = 0x00000001,
}


/*
 * TMDS_TRANSMITTER_CONTROL_PLL_ENABLE_HPD_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_TRANSMITTER_CONTROL_PLL_ENABLE_HPD_MASK {
TMDS_TRANSMITTER_HPD_NOT_OVERRIDE_PLL_ENABLE = 0x00000000,
TMDS_TRANSMITTER_HPD_OVERRIDE_PLL_ENABLE_ON_DISCON = 0x00000001,
TMDS_TRANSMITTER_HPD_OVERRIDE_PLL_ENABLE_ON_CON = 0x00000002,
TMDS_TRANSMITTER_HPD_OVERRIDE_PLL_ENABLE = 0x00000003,
}


/*
 * TMDS_TRANSMITTER_CONTROL_PLL_PWRUP_SEQ_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_TRANSMITTER_CONTROL_PLL_PWRUP_SEQ_EN {
TMDS_TRANSMITTER_PLL_PWRUP_SEQ_DISABLE   = 0x00000000,
TMDS_TRANSMITTER_PLL_PWRUP_SEQ_ENABLE    = 0x00000001,
}


/*
 * TMDS_TRANSMITTER_CONTROL_PLL_RESET_HPD_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_TRANSMITTER_CONTROL_PLL_RESET_HPD_MASK {
TMDS_TRANSMITTER_PLL_NOT_RST_ON_HPD      = 0x00000000,
TMDS_TRANSMITTER_PLL_RST_ON_HPD          = 0x00000001,
}


/*
 * TMDS_TRANSMITTER_CONTROL_TDCLK_FROM_PADS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_TRANSMITTER_CONTROL_TDCLK_FROM_PADS {
TMDS_TRANSMITTER_TDCLK_FROM_TMDS_TDCLK   = 0x00000000,
TMDS_TRANSMITTER_TDCLK_FROM_PADS         = 0x00000001,
}


/*
 * TMDS_TRANSMITTER_CONTROL_TMCLK_FROM_PADS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_TRANSMITTER_CONTROL_TMCLK_FROM_PADS {
TMDS_TRANSMITTER_TMCLK_FROM_TMDS_TMCLK   = 0x00000000,
TMDS_TRANSMITTER_TMCLK_FROM_PADS         = 0x00000001,
}


/*
 * TMDS_TRANSMITTER_ENABLE_HPD_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_TRANSMITTER_ENABLE_HPD_MASK {
TMDS_TRANSMITTER_HPD_MASK_NOT_OVERRIDE   = 0x00000000,
TMDS_TRANSMITTER_HPD_MASK_OVERRIDE       = 0x00000001,
}


/*
 * TMDS_TRANSMITTER_ENABLE_LNKCEN_HPD_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_TRANSMITTER_ENABLE_LNKCEN_HPD_MASK {
TMDS_TRANSMITTER_LNKCEN_HPD_MASK_NOT_OVERRIDE = 0x00000000,
TMDS_TRANSMITTER_LNKCEN_HPD_MASK_OVERRIDE = 0x00000001,
}


/*
 * TMDS_TRANSMITTER_ENABLE_LNKDEN_HPD_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_TRANSMITTER_ENABLE_LNKDEN_HPD_MASK {
TMDS_TRANSMITTER_LNKDEN_HPD_MASK_NOT_OVERRIDE = 0x00000000,
TMDS_TRANSMITTER_LNKDEN_HPD_MASK_OVERRIDE = 0x00000001,
}


/*******************************************************
 * DP_AUX Enums
 *******************************************************/

/*
 * DP_AUX_ARB_CONTROL_ARB_PRIORITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_ARB_CONTROL_ARB_PRIORITY {
DP_AUX_ARB_CONTROL_ARB_PRIORITY__GTC_LS_SW = 0x00000000,
DP_AUX_ARB_CONTROL_ARB_PRIORITY__LS_GTC_SW = 0x00000001,
DP_AUX_ARB_CONTROL_ARB_PRIORITY__SW_LS_GTC = 0x00000002,
DP_AUX_ARB_CONTROL_ARB_PRIORITY__SW_GTC_LS = 0x00000003,
}


/*
 * DP_AUX_ARB_CONTROL_DONE_USING_AUX_REG enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_ARB_CONTROL_DONE_USING_AUX_REG {
DP_AUX_ARB_CONTROL__DONE_NOT_USING_AUX_REG = 0x00000000,
DP_AUX_ARB_CONTROL__DONE_USING_AUX_REG   = 0x00000001,
}


/*
 * DP_AUX_ARB_CONTROL_USE_AUX_REG_REQ enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_ARB_CONTROL_USE_AUX_REG_REQ {
DP_AUX_ARB_CONTROL__NOT_USE_AUX_REG_REQ  = 0x00000000,
DP_AUX_ARB_CONTROL__USE_AUX_REG_REQ      = 0x00000001,
}


/*
 * DP_AUX_ARB_STATUS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_ARB_STATUS {
DP_AUX_IDLE                              = 0x00000000,
DP_AUX_IN_USE_LS                         = 0x00000001,
DP_AUX_IN_USE_GTC                        = 0x00000002,
DP_AUX_IN_USE_SW                         = 0x00000003,
DP_AUX_IN_USE_PHYWAKE                    = 0x00000004,
}


/*
 * DP_AUX_CONTROL_HPD_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_CONTROL_HPD_SEL {
DP_AUX_CONTROL_HPD1_SELECTED             = 0x00000000,
DP_AUX_CONTROL_HPD2_SELECTED             = 0x00000001,
DP_AUX_CONTROL_HPD3_SELECTED             = 0x00000002,
DP_AUX_CONTROL_HPD4_SELECTED             = 0x00000003,
DP_AUX_CONTROL_HPD5_SELECTED             = 0x00000004,
DP_AUX_CONTROL_NO_HPD_SELECTED           = 0x00000005,
}


/*
 * DP_AUX_CONTROL_TEST_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_CONTROL_TEST_MODE {
DP_AUX_CONTROL_TEST_MODE_DISABLE         = 0x00000000,
DP_AUX_CONTROL_TEST_MODE_ENABLE          = 0x00000001,
}


/*
 * DP_AUX_DEFINITE_ERR_REACHED_ACK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_DEFINITE_ERR_REACHED_ACK {
ALPHA_DP_AUX_DEFINITE_ERR_REACHED_NOT_ACK = 0x00000000,
ALPHA_DP_AUX_DEFINITE_ERR_REACHED_ACK    = 0x00000001,
}


/*
 * DP_AUX_DPHY_RX_CONTROL_ALLOW_BELOW_THRESHOLD_PHASE_DETECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_DPHY_RX_CONTROL_ALLOW_BELOW_THRESHOLD_PHASE_DETECT {
DP_AUX_DPHY_RX_CONTROL__NOT_ALLOW_BELOW_THRESHOLD_PHASE_DETECT = 0x00000000,
DP_AUX_DPHY_RX_CONTROL__ALLOW_BELOW_THRESHOLD_PHASE_DETECT = 0x00000001,
}


/*
 * DP_AUX_DPHY_RX_CONTROL_ALLOW_BELOW_THRESHOLD_START enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_DPHY_RX_CONTROL_ALLOW_BELOW_THRESHOLD_START {
DP_AUX_DPHY_RX_CONTROL__NOT_ALLOW_BELOW_THRESHOLD_START = 0x00000000,
DP_AUX_DPHY_RX_CONTROL__ALLOW_BELOW_THRESHOLD_START = 0x00000001,
}


/*
 * DP_AUX_DPHY_RX_CONTROL_ALLOW_BELOW_THRESHOLD_STOP enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_DPHY_RX_CONTROL_ALLOW_BELOW_THRESHOLD_STOP {
DP_AUX_DPHY_RX_CONTROL__NOT_ALLOW_BELOW_THRESHOLD_STOP = 0x00000000,
DP_AUX_DPHY_RX_CONTROL__ALLOW_BELOW_THRESHOLD_STOP = 0x00000001,
}


/*
 * DP_AUX_DPHY_RX_CONTROL_HALF_SYM_DETECT_LEN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_DPHY_RX_CONTROL_HALF_SYM_DETECT_LEN {
DP_AUX_DPHY_RX_CONTROL_HALF_SYM_DETECT_LEN__6_EDGES = 0x00000000,
DP_AUX_DPHY_RX_CONTROL_HALF_SYM_DETECT_LEN__10_EDGES = 0x00000001,
DP_AUX_DPHY_RX_CONTROL_HALF_SYM_DETECT_LEN__18_EDGES = 0x00000002,
DP_AUX_DPHY_RX_CONTROL_HALF_SYM_DETECT_LEN__RESERVED = 0x00000003,
}


/*
 * DP_AUX_DPHY_RX_CONTROL_PHASE_DETECT_LEN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_DPHY_RX_CONTROL_PHASE_DETECT_LEN {
DP_AUX_DPHY_RX_CONTROL_PHASE_DETECT_LEN__2_HALF_SYMBOLS = 0x00000000,
DP_AUX_DPHY_RX_CONTROL_PHASE_DETECT_LEN__4_HALF_SYMBOLS = 0x00000001,
DP_AUX_DPHY_RX_CONTROL_PHASE_DETECT_LEN__6_HALF_SYMBOLS = 0x00000002,
DP_AUX_DPHY_RX_CONTROL_PHASE_DETECT_LEN__8_HALF_SYMBOLS = 0x00000003,
}


/*
 * DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW {
DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW__1TO2_PERIOD = 0x00000000,
DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW__1TO4_PERIOD = 0x00000001,
DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW__1TO8_PERIOD = 0x00000002,
DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW__1TO16_PERIOD = 0x00000003,
DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW__1TO32_PERIOD = 0x00000004,
DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW__1TO64_PERIOD = 0x00000005,
DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW__1TO128_PERIOD = 0x00000006,
DP_AUX_DPHY_RX_CONTROL_RECEIVE_WINDOW__1TO256_PERIOD = 0x00000007,
}


/*
 * DP_AUX_DPHY_RX_CONTROL_START_WINDOW enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_DPHY_RX_CONTROL_START_WINDOW {
DP_AUX_DPHY_RX_CONTROL_START_WINDOW__1TO2_PERIOD = 0x00000000,
DP_AUX_DPHY_RX_CONTROL_START_WINDOW__1TO4_PERIOD = 0x00000001,
DP_AUX_DPHY_RX_CONTROL_START_WINDOW__1TO8_PERIOD = 0x00000002,
DP_AUX_DPHY_RX_CONTROL_START_WINDOW__1TO16_PERIOD = 0x00000003,
DP_AUX_DPHY_RX_CONTROL_START_WINDOW__1TO32_PERIOD = 0x00000004,
DP_AUX_DPHY_RX_CONTROL_START_WINDOW__1TO64_PERIOD = 0x00000005,
DP_AUX_DPHY_RX_CONTROL_START_WINDOW__1TO128_PERIOD = 0x00000006,
DP_AUX_DPHY_RX_CONTROL_START_WINDOW__1TO256_PERIOD = 0x00000007,
}


/*
 * DP_AUX_DPHY_RX_DETECTION_THRESHOLD enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_DPHY_RX_DETECTION_THRESHOLD {
DP_AUX_DPHY_RX_DETECTION_THRESHOLD__1to2 = 0x00000000,
DP_AUX_DPHY_RX_DETECTION_THRESHOLD__3to4 = 0x00000001,
DP_AUX_DPHY_RX_DETECTION_THRESHOLD__7to8 = 0x00000002,
DP_AUX_DPHY_RX_DETECTION_THRESHOLD__15to16 = 0x00000003,
DP_AUX_DPHY_RX_DETECTION_THRESHOLD__31to32 = 0x00000004,
DP_AUX_DPHY_RX_DETECTION_THRESHOLD__63to64 = 0x00000005,
DP_AUX_DPHY_RX_DETECTION_THRESHOLD__127to128 = 0x00000006,
DP_AUX_DPHY_RX_DETECTION_THRESHOLD__255to256 = 0x00000007,
}


/*
 * DP_AUX_DPHY_TX_CONTROL_MODE_DET_CHECK_DELAY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_DPHY_TX_CONTROL_MODE_DET_CHECK_DELAY {
DP_AUX_DPHY_TX_CONTROL_MODE_DET_CHECK_DELAY__0 = 0x00000000,
DP_AUX_DPHY_TX_CONTROL_MODE_DET_CHECK_DELAY__16US = 0x00000001,
DP_AUX_DPHY_TX_CONTROL_MODE_DET_CHECK_DELAY__32US = 0x00000002,
DP_AUX_DPHY_TX_CONTROL_MODE_DET_CHECK_DELAY__64US = 0x00000003,
DP_AUX_DPHY_TX_CONTROL_MODE_DET_CHECK_DELAY__128US = 0x00000004,
DP_AUX_DPHY_TX_CONTROL_MODE_DET_CHECK_DELAY__256US = 0x00000005,
}


/*
 * DP_AUX_DPHY_TX_REF_CONTROL_TX_RATE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_DPHY_TX_REF_CONTROL_TX_RATE {
DP_AUX_DPHY_TX_REF_CONTROL_TX_RATE__1MHZ = 0x00000000,
DP_AUX_DPHY_TX_REF_CONTROL_TX_RATE__2MHZ = 0x00000001,
DP_AUX_DPHY_TX_REF_CONTROL_TX_RATE__4MHZ = 0x00000002,
DP_AUX_DPHY_TX_REF_CONTROL_TX_RATE__8MHZ = 0x00000003,
}


/*
 * DP_AUX_DPHY_TX_REF_CONTROL_TX_REF_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_DPHY_TX_REF_CONTROL_TX_REF_SEL {
DP_AUX_DPHY_TX_REF_CONTROL_TX_REF_SEL__DIVIDED_SYM_CLK = 0x00000000,
DP_AUX_DPHY_TX_REF_CONTROL_TX_REF_SEL__FROM_DCCG_MICROSECOND_REF = 0x00000001,
}


/*
 * DP_AUX_ERR_OCCURRED_ACK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_ERR_OCCURRED_ACK {
DP_AUX_ERR_OCCURRED__NOT_ACK             = 0x00000000,
DP_AUX_ERR_OCCURRED__ACK                 = 0x00000001,
}


/*
 * DP_AUX_GTC_SYNC_CONTROL_GTC_SYNC_BLOCK_REQ enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_GTC_SYNC_CONTROL_GTC_SYNC_BLOCK_REQ {
DP_AUX_GTC_SYNC_CONTROL_GTC_SYNC_ALLOW_REQ_FROM_OTHER_AUX = 0x00000000,
DP_AUX_GTC_SYNC_CONTROL_GTC_SYNC_BLOCK_REQ_FROM_OTHER_AUX = 0x00000001,
}


/*
 * DP_AUX_GTC_SYNC_CONTROL_INTERVAL_RESET_WINDOW enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_GTC_SYNC_CONTROL_INTERVAL_RESET_WINDOW {
DP_AUX_GTC_SYNC_CONTROL_INTERVAL_RESET_WINDOW__300US = 0x00000000,
DP_AUX_GTC_SYNC_CONTROL_INTERVAL_RESET_WINDOW__400US = 0x00000001,
DP_AUX_GTC_SYNC_CONTROL_INTERVAL_RESET_WINDOW__500US = 0x00000002,
DP_AUX_GTC_SYNC_CONTROL_INTERVAL_RESET_WINDOW__600US = 0x00000003,
}


/*
 * DP_AUX_GTC_SYNC_CONTROL_OFFSET_CALC_MAX_ATTEMPT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_GTC_SYNC_CONTROL_OFFSET_CALC_MAX_ATTEMPT {
DP_AUX_GTC_SYNC_CONTROL_OFFSET_CALC_MAX_ATTEMPT__4_ATTAMPS = 0x00000000,
DP_AUX_GTC_SYNC_CONTROL_OFFSET_CALC_MAX_ATTEMPT__8_ATTAMPS = 0x00000001,
DP_AUX_GTC_SYNC_CONTROL_OFFSET_CALC_MAX_ATTEMPT__16_ATTAMPS = 0x00000002,
DP_AUX_GTC_SYNC_CONTROL_OFFSET_CALC_MAX_ATTEMPT__RESERVED = 0x00000003,
}


/*
 * DP_AUX_GTC_SYNC_ERROR_CONTROL_LOCK_ACQ_TIMEOUT_LEN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_GTC_SYNC_ERROR_CONTROL_LOCK_ACQ_TIMEOUT_LEN {
DP_AUX_GTC_SYNC_ERROR_CONTROL_LOCK_ACQ_TIMEOUT_LEN__0 = 0x00000000,
DP_AUX_GTC_SYNC_ERROR_CONTROL_LOCK_ACQ_TIMEOUT_LEN__64 = 0x00000001,
DP_AUX_GTC_SYNC_ERROR_CONTROL_LOCK_ACQ_TIMEOUT_LEN__128 = 0x00000002,
DP_AUX_GTC_SYNC_ERROR_CONTROL_LOCK_ACQ_TIMEOUT_LEN__256 = 0x00000003,
}


/*
 * DP_AUX_INT_ACK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_INT_ACK {
DP_AUX_INT__NOT_ACK                      = 0x00000000,
DP_AUX_INT__ACK                          = 0x00000001,
}


/*
 * DP_AUX_LS_UPDATE_ACK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_LS_UPDATE_ACK {
DP_AUX_INT_LS_UPDATE_NOT_ACK             = 0x00000000,
DP_AUX_INT_LS_UPDATE_ACK                 = 0x00000001,
}


/*
 * DP_AUX_PHY_WAKE_PRIORITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_PHY_WAKE_PRIORITY {
DP_AUX_PHY_WAKE_HIGH_PRIORITY            = 0x00000000,
DP_AUX_PHY_WAKE_LOW_PRIORITY             = 0x00000001,
}


/*
 * DP_AUX_POTENTIAL_ERR_REACHED_ACK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_POTENTIAL_ERR_REACHED_ACK {
DP_AUX_POTENTIAL_ERR_REACHED__NOT_ACK    = 0x00000000,
DP_AUX_POTENTIAL_ERR_REACHED__ACK        = 0x00000001,
}


/*
 * DP_AUX_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_RESET {
DP_AUX_RESET_DEASSERTED                  = 0x00000000,
DP_AUX_RESET_ASSERTED                    = 0x00000001,
}


/*
 * DP_AUX_RESET_DONE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_RESET_DONE {
DP_AUX_RESET_SEQUENCE_NOT_DONE           = 0x00000000,
DP_AUX_RESET_SEQUENCE_DONE               = 0x00000001,
}


/*
 * DP_AUX_RX_TIMEOUT_LEN_MUL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_RX_TIMEOUT_LEN_MUL {
DP_AUX_RX_TIMEOUT_LEN_NO_MUL             = 0x00000000,
DP_AUX_RX_TIMEOUT_LEN_MUL_2              = 0x00000001,
DP_AUX_RX_TIMEOUT_LEN_MUL_4              = 0x00000002,
DP_AUX_RX_TIMEOUT_LEN_MUL_8              = 0x00000003,
}


/*
 * DP_AUX_SW_CONTROL_LS_READ_TRIG enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_SW_CONTROL_LS_READ_TRIG {
DP_AUX_SW_CONTROL_LS_READ__NOT_TRIG      = 0x00000000,
DP_AUX_SW_CONTROL_LS_READ__TRIG          = 0x00000001,
}


/*
 * DP_AUX_SW_CONTROL_SW_GO enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_SW_CONTROL_SW_GO {
DP_AUX_SW_CONTROL_SW__NOT_GO             = 0x00000000,
DP_AUX_SW_CONTROL_SW__GO                 = 0x00000001,
}


/*
 * DP_AUX_TX_PRECHARGE_LEN_MUL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_AUX_TX_PRECHARGE_LEN_MUL {
DP_AUX_TX_PRECHARGE_LEN_NO_MUL           = 0x00000000,
DP_AUX_TX_PRECHARGE_LEN_MUL_2            = 0x00000001,
DP_AUX_TX_PRECHARGE_LEN_MUL_4            = 0x00000002,
DP_AUX_TX_PRECHARGE_LEN_MUL_8            = 0x00000003,
}


/*******************************************************
 * DOUT_I2C Enums
 *******************************************************/

/*
 * DOUT_I2C_ACK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_ACK {
DOUT_I2C_NO_ACK                          = 0x00000000,
DOUT_I2C_ACK_TO_CLEAN                    = 0x00000001,
}


/*
 * DOUT_I2C_ARBITRATION_ABORT_XFER enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_ARBITRATION_ABORT_XFER {
DOUT_I2C_ARBITRATION_NOT_ABORT_CURRENT_TRANSFER = 0x00000000,
DOUT_I2C_ARBITRATION_ABORT_CURRENT_TRANSFER = 0x00000001,
}


/*
 * DOUT_I2C_ARBITRATION_DONE_USING_I2C_REG enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_ARBITRATION_DONE_USING_I2C_REG {
DOUT_I2C_ARBITRATION_DONE__NOT_USING_I2C_REG = 0x00000000,
DOUT_I2C_ARBITRATION_DONE__USING_I2C_REG = 0x00000001,
}


/*
 * DOUT_I2C_ARBITRATION_NO_QUEUED_SW_GO enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_ARBITRATION_NO_QUEUED_SW_GO {
DOUT_I2C_ARBITRATION_SW_QUEUE_ENABLED    = 0x00000000,
DOUT_I2C_ARBITRATION_SW_QUEUE_DISABLED   = 0x00000001,
}


/*
 * DOUT_I2C_ARBITRATION_SW_PRIORITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_ARBITRATION_SW_PRIORITY {
DOUT_I2C_ARBITRATION_SW_PRIORITY_NORMAL  = 0x00000000,
DOUT_I2C_ARBITRATION_SW_PRIORITY_HIGH    = 0x00000001,
DOUT_I2C_ARBITRATION_SW_PRIORITY_0_RESERVED = 0x00000002,
DOUT_I2C_ARBITRATION_SW_PRIORITY_1_RESERVED = 0x00000003,
}


/*
 * DOUT_I2C_ARBITRATION_USE_I2C_REG_REQ enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_ARBITRATION_USE_I2C_REG_REQ {
DOUT_I2C_ARBITRATION__NOT_USE_I2C_REG_REQ = 0x00000000,
DOUT_I2C_ARBITRATION__USE_I2C_REG_REQ    = 0x00000001,
}


/*
 * DOUT_I2C_CONTROL_DBG_REF_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_CONTROL_DBG_REF_SEL {
DOUT_I2C_CONTROL_NORMAL_DEBUG            = 0x00000000,
DOUT_I2C_CONTROL_FAST_REFERENCE_DEBUG    = 0x00000001,
}


/*
 * DOUT_I2C_CONTROL_DDC_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_CONTROL_DDC_SELECT {
DOUT_I2C_CONTROL_SELECT_DDC1             = 0x00000000,
DOUT_I2C_CONTROL_SELECT_DDC2             = 0x00000001,
DOUT_I2C_CONTROL_SELECT_DDC3             = 0x00000002,
DOUT_I2C_CONTROL_SELECT_DDC4             = 0x00000003,
DOUT_I2C_CONTROL_SELECT_DDC5             = 0x00000004,
DOUT_I2C_CONTROL_SELECT_DDCVGA           = 0x00000005,
}


/*
 * DOUT_I2C_CONTROL_GO enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_CONTROL_GO {
DOUT_I2C_CONTROL_STOP_TRANSFER           = 0x00000000,
DOUT_I2C_CONTROL_START_TRANSFER          = 0x00000001,
}


/*
 * DOUT_I2C_CONTROL_SEND_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_CONTROL_SEND_RESET {
DOUT_I2C_CONTROL__NOT_SEND_RESET         = 0x00000000,
DOUT_I2C_CONTROL__SEND_RESET             = 0x00000001,
}


/*
 * DOUT_I2C_CONTROL_SEND_RESET_LENGTH enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_CONTROL_SEND_RESET_LENGTH {
DOUT_I2C_CONTROL__SEND_RESET_LENGTH_9    = 0x00000000,
DOUT_I2C_CONTROL__SEND_RESET_LENGTH_10   = 0x00000001,
}


/*
 * DOUT_I2C_CONTROL_SOFT_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_CONTROL_SOFT_RESET {
DOUT_I2C_CONTROL_NOT_RESET_I2C_CONTROLLER = 0x00000000,
DOUT_I2C_CONTROL_RESET_I2C_CONTROLLER    = 0x00000001,
}


/*
 * DOUT_I2C_CONTROL_SW_STATUS_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_CONTROL_SW_STATUS_RESET {
DOUT_I2C_CONTROL_NOT_RESET_SW_STATUS     = 0x00000000,
DOUT_I2C_CONTROL_RESET_SW_STATUS         = 0x00000001,
}


/*
 * DOUT_I2C_CONTROL_TRANSACTION_COUNT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_CONTROL_TRANSACTION_COUNT {
DOUT_I2C_CONTROL_TRANS0                  = 0x00000000,
DOUT_I2C_CONTROL_TRANS0_TRANS1           = 0x00000001,
DOUT_I2C_CONTROL_TRANS0_TRANS1_TRANS2    = 0x00000002,
DOUT_I2C_CONTROL_TRANS0_TRANS1_TRANS2_TRANS3 = 0x00000003,
}


/*
 * DOUT_I2C_DATA_INDEX_WRITE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_DATA_INDEX_WRITE {
DOUT_I2C_DATA__NOT_INDEX_WRITE           = 0x00000000,
DOUT_I2C_DATA__INDEX_WRITE               = 0x00000001,
}


/*
 * DOUT_I2C_DDC_SETUP_CLK_DRIVE_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_DDC_SETUP_CLK_DRIVE_EN {
DOUT_I2C_DDC_SETUP_CLK_DRIVE_BY_EXTERNAL_RESISTOR = 0x00000000,
DOUT_I2C_DDC_SETUP_I2C_PAD_DRIVE_SCL     = 0x00000001,
}


/*
 * DOUT_I2C_DDC_SETUP_DATA_DRIVE_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_DDC_SETUP_DATA_DRIVE_EN {
DOUT_I2C_DDC_SETUP_DATA_DRIVE_BY_EXTERNAL_RESISTOR = 0x00000000,
DOUT_I2C_DDC_SETUP_I2C_PAD_DRIVE_SDA     = 0x00000001,
}


/*
 * DOUT_I2C_DDC_SETUP_DATA_DRIVE_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_DDC_SETUP_DATA_DRIVE_SEL {
DOUT_I2C_DDC_SETUP_DATA_DRIVE_FOR_10MCLKS = 0x00000000,
DOUT_I2C_DDC_SETUP_DATA_DRIVE_FOR_20MCLKS = 0x00000001,
}


/*
 * DOUT_I2C_DDC_SETUP_EDID_DETECT_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_DDC_SETUP_EDID_DETECT_MODE {
DOUT_I2C_DDC_SETUP_EDID_DETECT_CONNECT   = 0x00000000,
DOUT_I2C_DDC_SETUP_EDID_DETECT_DISCONNECT = 0x00000001,
}


/*
 * DOUT_I2C_DDC_SPEED_THRESHOLD enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_DDC_SPEED_THRESHOLD {
DOUT_I2C_DDC_SPEED_THRESHOLD_BIG_THAN_ZERO = 0x00000000,
DOUT_I2C_DDC_SPEED_THRESHOLD_QUATER_OF_TOTAL_SAMPLE = 0x00000001,
DOUT_I2C_DDC_SPEED_THRESHOLD_HALF_OF_TOTAL_SAMPLE = 0x00000002,
DOUT_I2C_DDC_SPEED_THRESHOLD_THREE_QUATERS_OF_TOTAL_SAMPLE = 0x00000003,
}


/*
 * DOUT_I2C_EDID_DETECT_CTRL_SEND_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_EDID_DETECT_CTRL_SEND_RESET {
DOUT_I2C_EDID_NOT_SEND_RESET_BEFORE_EDID_READ_TRACTION = 0x00000000,
DOUT_I2C_EDID_SEND_RESET_BEFORE_EDID_READ_TRACTION = 0x00000001,
}


/*
 * DOUT_I2C_READ_REQUEST_INTERRUPT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_READ_REQUEST_INTERRUPT_TYPE {
DOUT_I2C_READ_REQUEST_INTERRUPT_TYPE__LEVEL = 0x00000000,
DOUT_I2C_READ_REQUEST_INTERRUPT_TYPE__PULSE = 0x00000001,
}


/*
 * DOUT_I2C_TRANSACTION_STOP_ON_NACK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DOUT_I2C_TRANSACTION_STOP_ON_NACK {
DOUT_I2C_TRANSACTION_STOP_CURRENT_TRANS  = 0x00000000,
DOUT_I2C_TRANSACTION_STOP_ALL_TRANS      = 0x00000001,
}


/*******************************************************
 * DIO_MISC Enums
 *******************************************************/

/*
 * CLOCK_GATING_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CLOCK_GATING_EN {
CLOCK_GATING_ENABLE                      = 0x00000000,
CLOCK_GATING_DISABLE                     = 0x00000001,
}


/*
 * DAC_MUX_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DAC_MUX_SELECT {
DAC_MUX_SELECT_DACA                      = 0x00000000,
DAC_MUX_SELECT_DACB                      = 0x00000001,
}


/*
 * DIOMEM_PWR_DIS_CTRL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIOMEM_PWR_DIS_CTRL {
DIOMEM_ENABLE_MEM_PWR_CTRL               = 0x00000000,
DIOMEM_DISABLE_MEM_PWR_CTRL              = 0x00000001,
}


/*
 * DIOMEM_PWR_FORCE_CTRL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIOMEM_PWR_FORCE_CTRL {
DIOMEM_NO_FORCE_REQUEST                  = 0x00000000,
DIOMEM_FORCE_LIGHT_SLEEP_REQUEST         = 0x00000001,
DIOMEM_FORCE_DEEP_SLEEP_REQUEST          = 0x00000002,
DIOMEM_FORCE_SHUT_DOWN_REQUEST           = 0x00000003,
}


/*
 * DIOMEM_PWR_FORCE_CTRL2 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIOMEM_PWR_FORCE_CTRL2 {
DIOMEM_NO_FORCE_REQ                      = 0x00000000,
DIOMEM_FORCE_LIGHT_SLEEP_REQ             = 0x00000001,
}


/*
 * DIOMEM_PWR_SEL_CTRL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIOMEM_PWR_SEL_CTRL {
DIOMEM_DYNAMIC_SHUT_DOWN_ENABLE          = 0x00000000,
DIOMEM_DYNAMIC_DEEP_SLEEP_ENABLE         = 0x00000001,
DIOMEM_DYNAMIC_LIGHT_SLEEP_ENABLE        = 0x00000002,
}


/*
 * DIOMEM_PWR_SEL_CTRL2 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIOMEM_PWR_SEL_CTRL2 {
DIOMEM_DYNAMIC_DEEP_SLEEP_EN             = 0x00000000,
DIOMEM_DYNAMIC_LIGHT_SLEEP_EN            = 0x00000001,
}


/*
 * DIO_DBG_BLOCK_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIO_DBG_BLOCK_SEL {
DIO_DBG_BLOCK_SEL_DIO                    = 0x00000000,
DIO_DBG_BLOCK_SEL_DIGFE_A                = 0x0000000b,
DIO_DBG_BLOCK_SEL_DIGFE_B                = 0x0000000c,
DIO_DBG_BLOCK_SEL_DIGFE_C                = 0x0000000d,
DIO_DBG_BLOCK_SEL_DIGFE_D                = 0x0000000e,
DIO_DBG_BLOCK_SEL_DIGFE_E                = 0x0000000f,
DIO_DBG_BLOCK_SEL_DIGA                   = 0x00000012,
DIO_DBG_BLOCK_SEL_DIGB                   = 0x00000013,
DIO_DBG_BLOCK_SEL_DIGC                   = 0x00000014,
DIO_DBG_BLOCK_SEL_DIGD                   = 0x00000015,
DIO_DBG_BLOCK_SEL_DIGE                   = 0x00000016,
DIO_DBG_BLOCK_SEL_DPFE_A                 = 0x00000019,
DIO_DBG_BLOCK_SEL_DPFE_B                 = 0x0000001a,
DIO_DBG_BLOCK_SEL_DPFE_C                 = 0x0000001b,
DIO_DBG_BLOCK_SEL_DPFE_D                 = 0x0000001c,
DIO_DBG_BLOCK_SEL_DPFE_E                 = 0x0000001d,
DIO_DBG_BLOCK_SEL_DPA                    = 0x00000020,
DIO_DBG_BLOCK_SEL_DPB                    = 0x00000021,
DIO_DBG_BLOCK_SEL_DPC                    = 0x00000022,
DIO_DBG_BLOCK_SEL_DPD                    = 0x00000023,
DIO_DBG_BLOCK_SEL_DPE                    = 0x00000024,
DIO_DBG_BLOCK_SEL_AUX0                   = 0x00000027,
DIO_DBG_BLOCK_SEL_AUX1                   = 0x00000028,
DIO_DBG_BLOCK_SEL_AUX2                   = 0x00000029,
DIO_DBG_BLOCK_SEL_AUX3                   = 0x0000002a,
DIO_DBG_BLOCK_SEL_AUX4                   = 0x0000002b,
DIO_DBG_BLOCK_SEL_PERFMON_DIO            = 0x0000002d,
DIO_DBG_BLOCK_SEL_RESERVED               = 0x0000002e,
}


/*
 * DIO_HDMI_RXSTATUS_TIMER_CONTROL_DIO_HDMI_RXSTATUS_TIMER_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DIO_HDMI_RXSTATUS_TIMER_CONTROL_DIO_HDMI_RXSTATUS_TIMER_TYPE {
DIO_HDMI_RXSTATUS_TIMER_TYPE_LEVEL       = 0x00000000,
DIO_HDMI_RXSTATUS_TIMER_TYPE_PULSE       = 0x00000001,
}


/*
 * DX_PROTECTION_DX_PIPE_ENC_REQUIRED_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DX_PROTECTION_DX_PIPE_ENC_REQUIRED_TYPE {
DX_PROTECTION_DX_PIPE_ENC_REQUIRED_TYPE_0 = 0x00000000,
DX_PROTECTION_DX_PIPE_ENC_REQUIRED_TYPE_1 = 0x00000001,
}


/*
 * ENUM_DIO_DCN_ACTIVE_STATUS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DIO_DCN_ACTIVE_STATUS {
ENUM_DCN_NOT_ACTIVE                      = 0x00000000,
ENUM_DCN_ACTIVE                          = 0x00000001,
}


/*
 * GENERIC_STEREOSYNC_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GENERIC_STEREOSYNC_SEL {
GENERIC_STEREOSYNC_SEL_D1                = 0x00000000,
GENERIC_STEREOSYNC_SEL_D2                = 0x00000001,
GENERIC_STEREOSYNC_SEL_D3                = 0x00000002,
GENERIC_STEREOSYNC_SEL_D4                = 0x00000003,
GENERIC_STEREOSYNC_SEL_RESERVED          = 0x00000004,
}


/*
 * PM_ASSERT_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PM_ASSERT_RESET {
PM_ASSERT_RESET_0                        = 0x00000000,
PM_ASSERT_RESET_1                        = 0x00000001,
}


/*
 * SOFT_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SOFT_RESET {
SOFT_RESET_0                             = 0x00000000,
SOFT_RESET_1                             = 0x00000001,
}


/*
 * TMDS_MUX_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TMDS_MUX_SELECT {
TMDS_MUX_SELECT_B                        = 0x00000000,
TMDS_MUX_SELECT_G                        = 0x00000001,
TMDS_MUX_SELECT_R                        = 0x00000002,
TMDS_MUX_SELECT_RESERVED                 = 0x00000003,
}


/*******************************************************
 * DME Enums
 *******************************************************/

/*
 * DME_MEM_POWER_STATE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DME_MEM_POWER_STATE_ENUM {
DME_MEM_POWER_STATE_ENUM_ON              = 0x00000000,
DME_MEM_POWER_STATE_ENUM_LS              = 0x00000001,
DME_MEM_POWER_STATE_ENUM_DS              = 0x00000002,
DME_MEM_POWER_STATE_ENUM_SD              = 0x00000003,
}


/*
 * DME_MEM_PWR_DIS_CTRL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DME_MEM_PWR_DIS_CTRL {
DME_MEM_ENABLE_MEM_PWR_CTRL              = 0x00000000,
DME_MEM_DISABLE_MEM_PWR_CTRL             = 0x00000001,
}


/*
 * DME_MEM_PWR_FORCE_CTRL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DME_MEM_PWR_FORCE_CTRL {
DME_MEM_NO_FORCE_REQUEST                 = 0x00000000,
DME_MEM_FORCE_LIGHT_SLEEP_REQUEST        = 0x00000001,
DME_MEM_FORCE_DEEP_SLEEP_REQUEST         = 0x00000002,
DME_MEM_FORCE_SHUT_DOWN_REQUEST          = 0x00000003,
}


/*
 * METADATA_HUBP_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum METADATA_HUBP_SEL {
METADATA_HUBP_SEL_0                      = 0x00000000,
METADATA_HUBP_SEL_1                      = 0x00000001,
METADATA_HUBP_SEL_2                      = 0x00000002,
METADATA_HUBP_SEL_3                      = 0x00000003,
METADATA_HUBP_SEL_RESERVED               = 0x00000004,
}


/*
 * METADATA_STREAM_TYPE_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum METADATA_STREAM_TYPE_SEL {
METADATA_STREAM_DP                       = 0x00000000,
METADATA_STREAM_DVE                      = 0x00000001,
}


/*******************************************************
 * VPG Enums
 *******************************************************/

/*
 * VPG_MEM_PWR_DIS_CTRL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VPG_MEM_PWR_DIS_CTRL {
VPG_MEM_ENABLE_MEM_PWR_CTRL              = 0x00000000,
VPG_MEM_DISABLE_MEM_PWR_CTRL             = 0x00000001,
}


/*
 * VPG_MEM_PWR_FORCE_CTRL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VPG_MEM_PWR_FORCE_CTRL {
VPG_MEM_NO_FORCE_REQ                     = 0x00000000,
VPG_MEM_FORCE_LIGHT_SLEEP_REQ            = 0x00000001,
}


/*******************************************************
 * AFMT Enums
 *******************************************************/

/*
 * AFMT_ACP_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AFMT_ACP_TYPE {
ACP_TYPE_GENERIC_AUDIO                   = 0x00000000,
ACP_TYPE_ICE60958_AUDIO                  = 0x00000001,
ACP_TYPE_DVD_AUDIO                       = 0x00000002,
ACP_TYPE_SUPER_AUDIO_CD                  = 0x00000003,
}


/*
 * AFMT_AUDIO_CRC_CONTROL_CH_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AFMT_AUDIO_CRC_CONTROL_CH_SEL {
AFMT_AUDIO_CRC_CH0_SIG                   = 0x00000000,
AFMT_AUDIO_CRC_CH1_SIG                   = 0x00000001,
AFMT_AUDIO_CRC_CH2_SIG                   = 0x00000002,
AFMT_AUDIO_CRC_CH3_SIG                   = 0x00000003,
AFMT_AUDIO_CRC_CH4_SIG                   = 0x00000004,
AFMT_AUDIO_CRC_CH5_SIG                   = 0x00000005,
AFMT_AUDIO_CRC_CH6_SIG                   = 0x00000006,
AFMT_AUDIO_CRC_CH7_SIG                   = 0x00000007,
AFMT_AUDIO_CRC_RESERVED_8                = 0x00000008,
AFMT_AUDIO_CRC_RESERVED_9                = 0x00000009,
AFMT_AUDIO_CRC_RESERVED_10               = 0x0000000a,
AFMT_AUDIO_CRC_RESERVED_11               = 0x0000000b,
AFMT_AUDIO_CRC_RESERVED_12               = 0x0000000c,
AFMT_AUDIO_CRC_RESERVED_13               = 0x0000000d,
AFMT_AUDIO_CRC_RESERVED_14               = 0x0000000e,
AFMT_AUDIO_CRC_AUDIO_SAMPLE_COUNT        = 0x0000000f,
}


/*
 * AFMT_AUDIO_CRC_CONTROL_CONT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AFMT_AUDIO_CRC_CONTROL_CONT {
AFMT_AUDIO_CRC_ONESHOT                   = 0x00000000,
AFMT_AUDIO_CRC_AUTO_RESTART              = 0x00000001,
}


/*
 * AFMT_AUDIO_CRC_CONTROL_SOURCE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AFMT_AUDIO_CRC_CONTROL_SOURCE {
AFMT_AUDIO_CRC_SOURCE_FROM_FIFO_INPUT    = 0x00000000,
AFMT_AUDIO_CRC_SOURCE_FROM_FIFO_OUTPUT   = 0x00000001,
}


/*
 * AFMT_AUDIO_PACKET_CONTROL2_AUDIO_LAYOUT_OVRD enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AFMT_AUDIO_PACKET_CONTROL2_AUDIO_LAYOUT_OVRD {
AFMT_AUDIO_LAYOUT_DETERMINED_BY_AZ_AUDIO_CHANNEL_STATUS = 0x00000000,
AFMT_AUDIO_LAYOUT_OVRD_BY_REGISTER       = 0x00000001,
}


/*
 * AFMT_AUDIO_PACKET_CONTROL_AUDIO_SAMPLE_SEND enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AFMT_AUDIO_PACKET_CONTROL_AUDIO_SAMPLE_SEND {
AFMT_AUDIO_PACKET_SENT_DISABLED          = 0x00000000,
AFMT_AUDIO_PACKET_SENT_ENABLED           = 0x00000001,
}


/*
 * AFMT_AUDIO_PACKET_CONTROL_RESET_FIFO_WHEN_AUDIO_DIS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AFMT_AUDIO_PACKET_CONTROL_RESET_FIFO_WHEN_AUDIO_DIS {
AFMT_NOT_RESET_AUDIO_FIFO_WHEN_AUDIO_DISABLED_RESERVED = 0x00000000,
AFMT_RESET_AUDIO_FIFO_WHEN_AUDIO_DISABLED = 0x00000001,
}


/*
 * AFMT_AUDIO_SRC_CONTROL_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AFMT_AUDIO_SRC_CONTROL_SELECT {
AFMT_AUDIO_SRC_FROM_AZ_STREAM0           = 0x00000000,
AFMT_AUDIO_SRC_FROM_AZ_STREAM1           = 0x00000001,
AFMT_AUDIO_SRC_FROM_AZ_STREAM2           = 0x00000002,
AFMT_AUDIO_SRC_FROM_AZ_STREAM3           = 0x00000003,
AFMT_AUDIO_SRC_FROM_AZ_STREAM4           = 0x00000004,
AFMT_AUDIO_SRC_FROM_AZ_STREAM5           = 0x00000005,
}


/*
 * AFMT_HDMI_AUDIO_SEND_MAX_PACKETS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AFMT_HDMI_AUDIO_SEND_MAX_PACKETS {
HDMI_NOT_SEND_MAX_AUDIO_PACKETS          = 0x00000000,
HDMI_SEND_MAX_AUDIO_PACKETS              = 0x00000001,
}


/*
 * AFMT_INFOFRAME_CONTROL0_AUDIO_INFO_SOURCE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AFMT_INFOFRAME_CONTROL0_AUDIO_INFO_SOURCE {
AFMT_INFOFRAME_SOURCE_FROM_AZALIA_BLOCK  = 0x00000000,
AFMT_INFOFRAME_SOURCE_FROM_AFMT_REGISTERS = 0x00000001,
}


/*
 * AFMT_INTERRUPT_STATUS_CHG_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AFMT_INTERRUPT_STATUS_CHG_MASK {
AFMT_INTERRUPT_DISABLE                   = 0x00000000,
AFMT_INTERRUPT_ENABLE                    = 0x00000001,
}


/*
 * AFMT_MEM_PWR_DIS_CTRL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AFMT_MEM_PWR_DIS_CTRL {
AFMT_MEM_ENABLE_MEM_PWR_CTRL             = 0x00000000,
AFMT_MEM_DISABLE_MEM_PWR_CTRL            = 0x00000001,
}


/*
 * AFMT_MEM_PWR_FORCE_CTRL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AFMT_MEM_PWR_FORCE_CTRL {
AFMT_MEM_NO_FORCE_REQUEST                = 0x00000000,
AFMT_MEM_FORCE_LIGHT_SLEEP_REQUEST       = 0x00000001,
AFMT_MEM_FORCE_DEEP_SLEEP_REQUEST        = 0x00000002,
AFMT_MEM_FORCE_SHUT_DOWN_REQUEST         = 0x00000003,
}


/*
 * AFMT_RAMP_CONTROL0_SIGN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AFMT_RAMP_CONTROL0_SIGN {
AFMT_RAMP_SIGNED                         = 0x00000000,
AFMT_RAMP_UNSIGNED                       = 0x00000001,
}


/*
 * AFMT_VBI_PACKET_CONTROL_ACP_SOURCE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AFMT_VBI_PACKET_CONTROL_ACP_SOURCE {
AFMT_ACP_SOURCE_FROM_AZALIA              = 0x00000000,
AFMT_ACP_SOURCE_FROM_AFMT_REGISTERS      = 0x00000001,
}


/*
 * AUDIO_LAYOUT_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AUDIO_LAYOUT_SELECT {
AUDIO_LAYOUT_0                           = 0x00000000,
AUDIO_LAYOUT_1                           = 0x00000001,
}


/*******************************************************
 * HPO_TOP Enums
 *******************************************************/

/*
 * HPO_TOP_CLOCK_GATING_DISABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HPO_TOP_CLOCK_GATING_DISABLE {
HPO_TOP_CLOCK_GATING_EN                  = 0x00000000,
HPO_TOP_CLOCK_GATING_DIS                 = 0x00000001,
}


/*
 * HPO_TOP_TEST_CLK_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HPO_TOP_TEST_CLK_SEL {
HPO_TOP_PERMANENT_DISPCLK                = 0x00000000,
HPO_TOP_REGISTER_GATED_DISPCLK           = 0x00000001,
HPO_TOP_PERMANENT_SOCCLK                 = 0x00000002,
HPO_TOP_TEST_CLOCK_RESERVED              = 0x00000003,
HPO_TOP_PERMANENT_HDMISTREAMCLK0         = 0x00000004,
HPO_TOP_FEATURE_GATED_HDMISTREAMCLK0     = 0x00000005,
HPO_TOP_REGISTER_GATED_HDMISTREAMCLK0    = 0x00000006,
HPO_TOP_FEATURE_GATED_DISPCLK_IN_HDMISTREAMENC0 = 0x00000007,
HPO_TOP_FEATURE_GATED_SOCCLK_IN_HDMISTREAMENC0 = 0x00000008,
HPO_TOP_PERMANENT_HDMICHARCLK0           = 0x00000009,
HPO_TOP_FEATURE_GATED_HDMICHARCLK0       = 0x0000000a,
HPO_TOP_REGISTER_GATED_HDMICHARCLK0      = 0x0000000b,
}


/*******************************************************
 * DP_STREAM_MAPPER Enums
 *******************************************************/

/*
 * DP_STREAM_MAPPER_DP_STREAM_LINK_TARGET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_STREAM_MAPPER_DP_STREAM_LINK_TARGET {
DP_STREAM_MAPPER_LINK0                   = 0x00000000,
DP_STREAM_MAPPER_LINK1                   = 0x00000001,
DP_STREAM_MAPPER_RESERVED                = 0x00000002,
}


/*******************************************************
 * HDMI_STREAM_ENC Enums
 *******************************************************/

/*
 * HDMI_STREAM_ENC_DB_DISABLE_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_STREAM_ENC_DB_DISABLE_CONTROL {
HDMI_STREAM_ENC_DB_ENABLE                = 0x00000000,
HDMI_STREAM_ENC_DB_DISABLE               = 0x00000001,
}


/*
 * HDMI_STREAM_ENC_DSC_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_STREAM_ENC_DSC_MODE {
STREAM_DSC_DISABLE                       = 0x00000000,
STREAM_DSC_444_RGB                       = 0x00000001,
STREAM_DSC_NATIVE_422_420                = 0x00000002,
}


/*
 * HDMI_STREAM_ENC_ENABLE_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_STREAM_ENC_ENABLE_CONTROL {
HDMI_STREAM_ENC_DISABLE                  = 0x00000000,
HDMI_STREAM_ENC_ENABLE                   = 0x00000001,
}


/*
 * HDMI_STREAM_ENC_ODM_COMBINE_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_STREAM_ENC_ODM_COMBINE_MODE {
STREAM_ODM_COMBINE_1_SEGMENT             = 0x00000000,
STREAM_ODM_COMBINE_2_SEGMENT             = 0x00000001,
STREAM_ODM_COMBINE_RESERVED              = 0x00000002,
STREAM_ODM_COMBINE_4_SEGMENT             = 0x00000003,
}


/*
 * HDMI_STREAM_ENC_OVERFLOW_UNDERFLOW_ERROR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_STREAM_ENC_OVERFLOW_UNDERFLOW_ERROR {
HDMI_STREAM_ENC_NO_ERROR_OCCURRED        = 0x00000000,
HDMI_STREAM_ENC_UNDERFLOW_OCCURRED       = 0x00000001,
HDMI_STREAM_ENC_OVERFLOW_OCCURRED        = 0x00000002,
}


/*
 * HDMI_STREAM_ENC_OVERWRITE_LEVEL_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_STREAM_ENC_OVERWRITE_LEVEL_SELECT {
HDMI_STREAM_ENC_HARDWARE                 = 0x00000000,
HDMI_STREAM_ENC_PROGRAMMABLE             = 0x00000001,
}


/*
 * HDMI_STREAM_ENC_PIXEL_ENCODING enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_STREAM_ENC_PIXEL_ENCODING {
STREAM_PIXEL_ENCODING_444_RGB            = 0x00000000,
STREAM_PIXEL_ENCODING_422                = 0x00000001,
STREAM_PIXEL_ENCODING_420                = 0x00000002,
}


/*
 * HDMI_STREAM_ENC_READ_CLOCK_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_STREAM_ENC_READ_CLOCK_CONTROL {
HDMI_STREAM_ENC_DCCG                     = 0x00000000,
HDMI_STREAM_ENC_DISPLAY_PIPE             = 0x00000001,
}


/*
 * HDMI_STREAM_ENC_RESET_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_STREAM_ENC_RESET_CONTROL {
HDMI_STREAM_ENC_NOT_RESET                = 0x00000000,
HDMI_STREAM_ENC_RESET                    = 0x00000001,
}


/*
 * HDMI_STREAM_ENC_STREAM_ACTIVE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_STREAM_ENC_STREAM_ACTIVE {
HDMI_STREAM_ENC_VIDEO_STREAM_NOT_ACTIVE  = 0x00000000,
HDMI_STREAM_ENC_VIDEO_STREAM_ACTIVE      = 0x00000001,
}


/*******************************************************
 * HDMI_TB_ENC Enums
 *******************************************************/

/*
 * BORROWBUFFER_MEM_POWER_STATE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BORROWBUFFER_MEM_POWER_STATE_ENUM {
BORROWBUFFER_MEM_POWER_STATE_ENUM_ON     = 0x00000000,
BORROWBUFFER_MEM_POWER_STATE_ENUM_LS     = 0x00000001,
BORROWBUFFER_MEM_POWER_STATE_ENUM_DS     = 0x00000002,
BORROWBUFFER_MEM_POWER_STATE_ENUM_SD     = 0x00000003,
}


/*
 * HDMI_BORROW_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_BORROW_MODE {
TB_BORROW_MODE_NONE                      = 0x00000000,
TB_BORROW_MODE_ACTIVE                    = 0x00000001,
TB_BORROW_MODE_BLANK                     = 0x00000002,
TB_BORROW_MODE_RESERVED                  = 0x00000003,
}


/*
 * HDMI_TB_ENC_ACP_SEND enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_ACP_SEND {
TB_ACP_NOT_SEND                          = 0x00000000,
TB_ACP_PKT_SEND                          = 0x00000001,
}


/*
 * HDMI_TB_ENC_ACR_AUDIO_PRIORITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_ACR_AUDIO_PRIORITY {
TB_ACR_PKT_HIGH_PRIORITY_THAN_AUDIO_SAMPLE = 0x00000000,
TB_AUDIO_SAMPLE_HIGH_PRIORITY_THAN_ACR_PKT = 0x00000001,
}


/*
 * HDMI_TB_ENC_ACR_CONT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_ACR_CONT {
TB_ACR_CONT_DISABLE                      = 0x00000000,
TB_ACR_CONT_ENABLE                       = 0x00000001,
}


/*
 * HDMI_TB_ENC_ACR_N_MULTIPLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_ACR_N_MULTIPLE {
TB_ACR_0_MULTIPLE_RESERVED               = 0x00000000,
TB_ACR_1_MULTIPLE                        = 0x00000001,
TB_ACR_2_MULTIPLE                        = 0x00000002,
TB_ACR_3_MULTIPLE_RESERVED               = 0x00000003,
TB_ACR_4_MULTIPLE                        = 0x00000004,
TB_ACR_5_MULTIPLE_RESERVED               = 0x00000005,
TB_ACR_6_MULTIPLE_RESERVED               = 0x00000006,
TB_ACR_7_MULTIPLE_RESERVED               = 0x00000007,
}


/*
 * HDMI_TB_ENC_ACR_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_ACR_SELECT {
TB_ACR_SELECT_HW                         = 0x00000000,
TB_ACR_SELECT_32K                        = 0x00000001,
TB_ACR_SELECT_44K                        = 0x00000002,
TB_ACR_SELECT_48K                        = 0x00000003,
}


/*
 * HDMI_TB_ENC_ACR_SEND enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_ACR_SEND {
TB_ACR_NOT_SEND                          = 0x00000000,
TB_ACR_PKT_SEND                          = 0x00000001,
}


/*
 * HDMI_TB_ENC_ACR_SOURCE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_ACR_SOURCE {
TB_ACR_SOURCE_HW                         = 0x00000000,
TB_ACR_SOURCE_SW                         = 0x00000001,
}


/*
 * HDMI_TB_ENC_AUDIO_INFO_CONT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_AUDIO_INFO_CONT {
TB_AUDIO_INFO_CONT_DISABLE               = 0x00000000,
TB_AUDIO_INFO_CONT_ENABLE                = 0x00000001,
}


/*
 * HDMI_TB_ENC_AUDIO_INFO_SEND enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_AUDIO_INFO_SEND {
TB_AUDIO_INFO_NOT_SEND                   = 0x00000000,
TB_AUDIO_INFO_PKT_SEND                   = 0x00000001,
}


/*
 * HDMI_TB_ENC_CRC_SRC_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_CRC_SRC_SEL {
TB_CRC_TB_ENC_INPUT                      = 0x00000000,
TB_CRC_DSC_PACKER                        = 0x00000001,
TB_CRC_DEEP_COLOR_PACKER                 = 0x00000002,
TB_CRC_ENCRYPTOR_INPUT                   = 0x00000003,
}


/*
 * HDMI_TB_ENC_CRC_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_CRC_TYPE {
TB_CRC_ALL_TRIBYTES                      = 0x00000000,
TB_CRC_ACTIVE_TRIBYTES                   = 0x00000001,
TB_CRC_DATAISLAND_TRIBYTES               = 0x00000002,
TB_CRC_ACTIVE_AND_DATAISLAND_TRIBYTES    = 0x00000003,
}


/*
 * HDMI_TB_ENC_DEEP_COLOR_DEPTH enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_DEEP_COLOR_DEPTH {
TB_DEEP_COLOR_DEPTH_24BPP                = 0x00000000,
TB_DEEP_COLOR_DEPTH_30BPP                = 0x00000001,
TB_DEEP_COLOR_DEPTH_36BPP                = 0x00000002,
TB_DEEP_COLOR_DEPTH_RESERVED             = 0x00000003,
}


/*
 * HDMI_TB_ENC_DEFAULT_PAHSE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_DEFAULT_PAHSE {
TB_DEFAULT_PHASE_IS_0                    = 0x00000000,
TB_DEFAULT_PHASE_IS_1                    = 0x00000001,
}


/*
 * HDMI_TB_ENC_DSC_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_DSC_MODE {
TB_DSC_DISABLE                           = 0x00000000,
TB_DSC_444_RGB                           = 0x00000001,
TB_DSC_NATIVE_422_420                    = 0x00000002,
}


/*
 * HDMI_TB_ENC_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_ENABLE {
TB_DISABLE                               = 0x00000000,
TB_ENABLE                                = 0x00000001,
}


/*
 * HDMI_TB_ENC_GC_AVMUTE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_GC_AVMUTE {
TB_GC_AVMUTE_SET                         = 0x00000000,
TB_GC_AVMUTE_UNSET                       = 0x00000001,
}


/*
 * HDMI_TB_ENC_GC_AVMUTE_CONT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_GC_AVMUTE_CONT {
TB_GC_AVMUTE_CONT_DISABLE                = 0x00000000,
TB_GC_AVMUTE_CONT_ENABLE                 = 0x00000001,
}


/*
 * HDMI_TB_ENC_GC_CONT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_GC_CONT {
TB_GC_CONT_DISABLE                       = 0x00000000,
TB_GC_CONT_ENABLE                        = 0x00000001,
}


/*
 * HDMI_TB_ENC_GC_SEND enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_GC_SEND {
TB_GC_NOT_SEND                           = 0x00000000,
TB_GC_PKT_SEND                           = 0x00000001,
}


/*
 * HDMI_TB_ENC_GENERIC_CONT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_GENERIC_CONT {
TB_GENERIC_CONT_DISABLE                  = 0x00000000,
TB_GENERIC_CONT_ENABLE                   = 0x00000001,
}


/*
 * HDMI_TB_ENC_GENERIC_LOCK_EN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_GENERIC_LOCK_EN {
HDMI_TB_ENC_GENERIC_LOCK_DISABLE         = 0x00000000,
HDMI_TB_ENC_GENERIC_LOCK_ENABLE          = 0x00000001,
}


/*
 * HDMI_TB_ENC_GENERIC_SEND enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_GENERIC_SEND {
TB_GENERIC_NOT_SEND                      = 0x00000000,
TB_GENERIC_PKT_SEND                      = 0x00000001,
}


/*
 * HDMI_TB_ENC_ISRC_CONT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_ISRC_CONT {
TB_ISRC_CONT_DISABLE                     = 0x00000000,
TB_ISRC_CONT_ENABLE                      = 0x00000001,
}


/*
 * HDMI_TB_ENC_ISRC_SEND enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_ISRC_SEND {
TB_ISRC_NOT_SEND                         = 0x00000000,
TB_ISRC_PKT_SEND                         = 0x00000001,
}


/*
 * HDMI_TB_ENC_METADATA_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_METADATA_ENABLE {
TB_METADATA_NOT_SEND                     = 0x00000000,
TB_METADATA_PKT_SEND                     = 0x00000001,
}


/*
 * HDMI_TB_ENC_PACKET_LINE_REFERENCE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_PACKET_LINE_REFERENCE {
TB_PKT_LINE_REF_END_OF_ACTIVE            = 0x00000000,
TB_PKT_LINE_REF_OTGSOF                   = 0x00000001,
}


/*
 * HDMI_TB_ENC_PIXEL_ENCODING enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_PIXEL_ENCODING {
TB_PIXEL_ENCODING_444_RGB                = 0x00000000,
TB_PIXEL_ENCODING_422                    = 0x00000001,
TB_PIXEL_ENCODING_420                    = 0x00000002,
}


/*
 * HDMI_TB_ENC_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_RESET {
TB_NOT_RESET                             = 0x00000000,
TB_RESET                                 = 0x00000001,
}


/*
 * HDMI_TB_ENC_SYNC_PHASE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDMI_TB_ENC_SYNC_PHASE {
TB_NOT_SYNC_PHASE_ON_FRAME_START         = 0x00000000,
TB_SYNC_PHASE_ON_FRAME_START             = 0x00000001,
}


/*
 * INPUT_FIFO_ERROR_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum INPUT_FIFO_ERROR_TYPE {
TB_NO_ERROR_OCCURRED                     = 0x00000000,
TB_OVERFLOW_OCCURRED                     = 0x00000001,
}


/*******************************************************
 * DP_STREAM_ENC Enums
 *******************************************************/

/*
 * DP_STREAM_ENC_OVERFLOW_UNDERFLOW_ERROR enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_STREAM_ENC_OVERFLOW_UNDERFLOW_ERROR {
DP_STREAM_ENC_NO_ERROR_OCCURRED          = 0x00000000,
DP_STREAM_ENC_UNDERFLOW_OCCURRED         = 0x00000001,
DP_STREAM_ENC_OVERFLOW_OCCURRED          = 0x00000002,
}


/*
 * DP_STREAM_ENC_OVERWRITE_LEVEL_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_STREAM_ENC_OVERWRITE_LEVEL_SELECT {
DP_STREAM_ENC_HARDWARE                   = 0x00000000,
DP_STREAM_ENC_PROGRAMMABLE               = 0x00000001,
}


/*
 * DP_STREAM_ENC_READ_CLOCK_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_STREAM_ENC_READ_CLOCK_CONTROL {
DP_STREAM_ENC_DCCG                       = 0x00000000,
DP_STREAM_ENC_DISPLAY_PIPE               = 0x00000001,
}


/*
 * DP_STREAM_ENC_RESET_CONTROL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_STREAM_ENC_RESET_CONTROL {
DP_STREAM_ENC_NOT_RESET                  = 0x00000000,
DP_STREAM_ENC_RESET                      = 0x00000001,
}


/*
 * DP_STREAM_ENC_STREAM_ACTIVE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DP_STREAM_ENC_STREAM_ACTIVE {
DP_STREAM_ENC_VIDEO_STREAM_NOT_ACTIVE    = 0x00000000,
DP_STREAM_ENC_VIDEO_STREAM_ACTIVE        = 0x00000001,
}


/*******************************************************
 * DP_SYM32_ENC Enums
 *******************************************************/

/*
 * ENUM_DP_SYM32_ENC_AUDIO_MUTE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_AUDIO_MUTE {
DP_SYM32_ENC_SDP_AUDIO_MUTE_NOT_FORCED   = 0x00000000,
DP_SYM32_ENC_SDP_AUDIO_MUTE_FORCED       = 0x00000001,
}


/*
 * ENUM_DP_SYM32_ENC_CONTINUOUS_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_CONTINUOUS_MODE {
DP_SYM32_ENC_ONE_SHOT_MODE               = 0x00000000,
DP_SYM32_ENC_CONTINUOUS_MODE             = 0x00000001,
}


/*
 * ENUM_DP_SYM32_ENC_CRC_VALID enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_CRC_VALID {
DP_SYM32_ENC_CRC_NOT_VALID               = 0x00000000,
DP_SYM32_ENC_CRC_VALID                   = 0x00000001,
}


/*
 * ENUM_DP_SYM32_ENC_DP_COMPONENT_DEPTH enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_DP_COMPONENT_DEPTH {
DP_SYM32_ENC_COMPONENT_DEPTH_6BPC        = 0x00000000,
DP_SYM32_ENC_COMPONENT_DEPTH_8BPC        = 0x00000001,
DP_SYM32_ENC_COMPONENT_DEPTH_10BPC       = 0x00000002,
DP_SYM32_ENC_COMPONENT_DEPTH_12BPC       = 0x00000003,
}


/*
 * ENUM_DP_SYM32_ENC_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_ENABLE {
DP_SYM32_ENC_DISABLE                     = 0x00000000,
DP_SYM32_ENC_ENABLE                      = 0x00000001,
}


/*
 * ENUM_DP_SYM32_ENC_GSP_DEADLINE_MISSED enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_GSP_DEADLINE_MISSED {
DP_SYM32_ENC_GSP_DEADLINE_NOT_MISSED     = 0x00000000,
DP_SYM32_ENC_GSP_DEADLINE_MISSED         = 0x00000001,
}


/*
 * ENUM_DP_SYM32_ENC_GSP_ONE_SHOT_TRIGGER_POSITION enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_GSP_ONE_SHOT_TRIGGER_POSITION {
DP_SYM32_ENC_GSP_SEND_AT_LINE_NUMBER     = 0x00000000,
DP_SYM32_ENC_GSP_SEND_AT_EARLIEST_TIME   = 0x00000001,
}


/*
 * ENUM_DP_SYM32_ENC_GSP_PAYLOAD_SIZE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_GSP_PAYLOAD_SIZE {
DP_SYM32_ENC_GSP_PAYLOAD_SIZE_32         = 0x00000000,
DP_SYM32_ENC_GSP_PAYLOAD_SIZE_RESERVED0  = 0x00000001,
DP_SYM32_ENC_GSP_PAYLOAD_SIZE_RESERVED1  = 0x00000002,
DP_SYM32_ENC_GSP_PAYLOAD_SIZE_128        = 0x00000003,
}


/*
 * ENUM_DP_SYM32_ENC_GSP_TRIGGER_PENDING enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_GSP_TRIGGER_PENDING {
DP_SYM32_ENC_GSP_TRIGGER_NOT_PENDING     = 0x00000000,
DP_SYM32_ENC_GSP_TRIGGER_PENDING         = 0x00000001,
}


/*
 * ENUM_DP_SYM32_ENC_MEM_PWR_FORCE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_MEM_PWR_FORCE_ENUM {
DP_SYM32_ENC_MEM_PWR_NO_FORCE_REQUEST    = 0x00000000,
DP_SYM32_ENC_MEM_PWR_FORCE_LIGHT_SLEEP_REQUEST = 0x00000001,
DP_SYM32_ENC_MEM_PWR_FORCE_DEEP_SLEEP_REQUEST = 0x00000002,
DP_SYM32_ENC_MEM_PWR_FORCE_SHUT_DOWN_REQUEST = 0x00000003,
}


/*
 * ENUM_DP_SYM32_ENC_OVERFLOW_STATUS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_OVERFLOW_STATUS {
DP_SYM32_ENC_NO_OVERFLOW_OCCURRED        = 0x00000000,
DP_SYM32_ENC_OVERFLOW_OCCURRED           = 0x00000001,
}


/*
 * ENUM_DP_SYM32_ENC_PENDING enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_PENDING {
DP_SYM32_ENC_NOT_PENDING                 = 0x00000000,
DP_SYM32_ENC_PENDING                     = 0x00000001,
}


/*
 * ENUM_DP_SYM32_ENC_PIXEL_ENCODING enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_PIXEL_ENCODING {
DP_SYM32_ENC_PIXEL_ENCODING_RGB_YCBCR444 = 0x00000000,
DP_SYM32_ENC_PIXEL_ENCODING_YCBCR422     = 0x00000001,
DP_SYM32_ENC_PIXEL_ENCODING_YCBCR420     = 0x00000002,
DP_SYM32_ENC_PIXEL_ENCODING_Y_ONLY       = 0x00000003,
}


/*
 * ENUM_DP_SYM32_ENC_PIXEL_ENCODING_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_PIXEL_ENCODING_TYPE {
DP_SYM32_ENC_UNCOMPRESSED_FORMAT         = 0x00000000,
DP_SYM32_ENC_COMPRESSED_FORMAT           = 0x00000001,
}


/*
 * ENUM_DP_SYM32_ENC_POWER_STATE_ENUM enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_POWER_STATE_ENUM {
DP_SYM32_ENC_POWER_STATE_ENUM_ON         = 0x00000000,
DP_SYM32_ENC_POWER_STATE_ENUM_LS         = 0x00000001,
DP_SYM32_ENC_POWER_STATE_ENUM_DS         = 0x00000002,
DP_SYM32_ENC_POWER_STATE_ENUM_SD         = 0x00000003,
}


/*
 * ENUM_DP_SYM32_ENC_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_RESET {
DP_SYM32_ENC_NOT_RESET                   = 0x00000000,
DP_SYM32_ENC_RESET                       = 0x00000001,
}


/*
 * ENUM_DP_SYM32_ENC_SDP_PRIORITY enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_SDP_PRIORITY {
DP_SYM32_ENC_SDP_LOW_PRIORITY            = 0x00000000,
DP_SYM32_ENC_SDP_HIGH_PRIORITY           = 0x00000001,
}


/*
 * ENUM_DP_SYM32_ENC_SOF_REFERENCE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_SOF_REFERENCE {
DP_SYM32_ENC_DP_SOF                      = 0x00000000,
DP_SYM32_ENC_OTG_SOF                     = 0x00000001,
}


/*
 * ENUM_DP_SYM32_ENC_VID_STREAM_DEFER enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_SYM32_ENC_VID_STREAM_DEFER {
DP_SYM32_ENC_VID_STREAM_NO_DEFER         = 0x00000000,
DP_SYM32_ENC_VID_STREAM_DEFER_TO_HBLANK  = 0x00000001,
DP_SYM32_ENC_VID_STREAM_DEFER_TO_VBLANK  = 0x00000002,
}


/*******************************************************
 * DP_DPHY_SYM32 Enums
 *******************************************************/

/*
 * ENUM_DP_DPHY_SYM32_CRC_END_EVENT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_CRC_END_EVENT {
DP_DPHY_SYM32_CRC_END_LLCP               = 0x00000000,
DP_DPHY_SYM32_CRC_END_PS_ONLY            = 0x00000001,
DP_DPHY_SYM32_CRC_END_PS_LT_SR           = 0x00000002,
DP_DPHY_SYM32_CRC_END_PS_ANY             = 0x00000003,
}


/*
 * ENUM_DP_DPHY_SYM32_CRC_START_EVENT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_CRC_START_EVENT {
DP_DPHY_SYM32_CRC_START_LLCP             = 0x00000000,
DP_DPHY_SYM32_CRC_START_PS_ONLY          = 0x00000001,
DP_DPHY_SYM32_CRC_START_PS_LT_SR         = 0x00000002,
DP_DPHY_SYM32_CRC_START_PS_POST_LT_SR    = 0x00000003,
DP_DPHY_SYM32_CRC_START_TP_START         = 0x00000004,
}


/*
 * ENUM_DP_DPHY_SYM32_CRC_TAP_SOURCE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_CRC_TAP_SOURCE {
DP_DPHY_SYM32_CRC_TAP_SOURCE_SCHEDULER   = 0x00000000,
DP_DPHY_SYM32_CRC_TAP_SOURCE_SYMBOL_HANDLER = 0x00000001,
DP_DPHY_SYM32_CRC_TAP_SOURCE_TP_GEN_MUX  = 0x00000002,
}


/*
 * ENUM_DP_DPHY_SYM32_CRC_USE_NUM_SYMBOLS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_CRC_USE_NUM_SYMBOLS {
DP_DPHY_SYM32_CRC_USE_END_EVENT          = 0x00000000,
DP_DPHY_SYM32_CRC_USE_NUM_SYMBOLS        = 0x00000001,
}


/*
 * ENUM_DP_DPHY_SYM32_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_ENABLE {
DP_DPHY_SYM32_DISABLE                    = 0x00000000,
DP_DPHY_SYM32_ENABLE                     = 0x00000001,
}


/*
 * ENUM_DP_DPHY_SYM32_ENCRYPT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_ENCRYPT_TYPE {
DP_DPHY_SYM32_ENCRYPT_TYPE0              = 0x00000000,
DP_DPHY_SYM32_ENCRYPT_TYPE1              = 0x00000001,
}


/*
 * ENUM_DP_DPHY_SYM32_MODE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_MODE {
DP_DPHY_SYM32_LT_TPS1                    = 0x00000000,
DP_DPHY_SYM32_LT_TPS2                    = 0x00000001,
DP_DPHY_SYM32_ACTIVE                     = 0x00000002,
DP_DPHY_SYM32_TEST                       = 0x00000003,
}


/*
 * ENUM_DP_DPHY_SYM32_NUM_LANES enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_NUM_LANES {
DP_DPHY_SYM32_1LANE                      = 0x00000000,
DP_DPHY_SYM32_2LANE                      = 0x00000001,
DP_DPHY_SYM32_RESERVED                   = 0x00000002,
DP_DPHY_SYM32_4LANE                      = 0x00000003,
}


/*
 * ENUM_DP_DPHY_SYM32_RATE_UPDATE_PENDING enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_RATE_UPDATE_PENDING {
DP_DPHY_SYM32_NO_RATE_UPDATE_PENDING     = 0x00000000,
DP_DPHY_SYM32_RATE_UPDATE_PENDING        = 0x00000001,
}


/*
 * ENUM_DP_DPHY_SYM32_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_RESET {
DP_DPHY_SYM32_NOT_RESET                  = 0x00000000,
DP_DPHY_SYM32_RESET                      = 0x00000001,
}


/*
 * ENUM_DP_DPHY_SYM32_RESET_STATUS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_RESET_STATUS {
DP_DPHY_SYM32_RESET_STATUS_DEASSERTED    = 0x00000000,
DP_DPHY_SYM32_RESET_STATUS_ASSERTED      = 0x00000001,
}


/*
 * ENUM_DP_DPHY_SYM32_SAT_UPDATE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_SAT_UPDATE {
DP_DPHY_SYM32_SAT_NO_UPDATE              = 0x00000000,
DP_DPHY_SYM32_SAT_TRIGGER_UPDATE         = 0x00000001,
DP_DPHY_SYM32_SAT_NOTRIGGER_UPDATE       = 0x00000002,
}


/*
 * ENUM_DP_DPHY_SYM32_SAT_UPDATE_PENDING enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_SAT_UPDATE_PENDING {
DP_DPHY_SYM32_SAT_NO_UPDATE_PENDING      = 0x00000000,
DP_DPHY_SYM32_SAT_TRIGGER_UPDATE_PENDING = 0x00000001,
DP_DPHY_SYM32_SAT_NOTRIGGER_UPDATE_PENDING = 0x00000002,
}


/*
 * ENUM_DP_DPHY_SYM32_STATUS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_STATUS {
DP_DPHY_SYM32_STATUS_IDLE                = 0x00000000,
DP_DPHY_SYM32_STATUS_ENABLED             = 0x00000001,
}


/*
 * ENUM_DP_DPHY_SYM32_STREAM_OVR_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_STREAM_OVR_ENABLE {
DP_DPHY_SYM32_STREAM_OVR_NONE            = 0x00000000,
DP_DPHY_SYM32_STREAM_OVR_REPLACE         = 0x00000001,
DP_DPHY_SYM32_STREAM_OVR_ALWAYS          = 0x00000002,
}


/*
 * ENUM_DP_DPHY_SYM32_STREAM_OVR_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_STREAM_OVR_TYPE {
DP_DPHY_SYM32_STREAM_OVR_TYPE_DATA       = 0x00000000,
DP_DPHY_SYM32_STREAM_OVR_TYPE_CONTROL    = 0x00000001,
}


/*
 * ENUM_DP_DPHY_SYM32_TP_PRBS_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_TP_PRBS_SEL {
DP_DPHY_SYM32_TP_PRBS_SEL_PRBS7          = 0x00000000,
DP_DPHY_SYM32_TP_PRBS_SEL_PRBS9          = 0x00000001,
DP_DPHY_SYM32_TP_PRBS_SEL_PRBS11         = 0x00000002,
DP_DPHY_SYM32_TP_PRBS_SEL_PRBS15         = 0x00000003,
DP_DPHY_SYM32_TP_PRBS_SEL_PRBS23         = 0x00000004,
DP_DPHY_SYM32_TP_PRBS_SEL_PRBS31         = 0x00000005,
}


/*
 * ENUM_DP_DPHY_SYM32_TP_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ENUM_DP_DPHY_SYM32_TP_SELECT {
DP_DPHY_SYM32_TP_SELECT_TPS1             = 0x00000000,
DP_DPHY_SYM32_TP_SELECT_TPS2             = 0x00000001,
DP_DPHY_SYM32_TP_SELECT_PRBS             = 0x00000002,
DP_DPHY_SYM32_TP_SELECT_CUSTOM           = 0x00000003,
DP_DPHY_SYM32_TP_SELECT_SQUARE           = 0x00000004,
}


/*******************************************************
 * APG Enums
 *******************************************************/

/*
 * APG_AUDIO_CRC_CONTROL_CH_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum APG_AUDIO_CRC_CONTROL_CH_SEL {
APG_AUDIO_CRC_CH0_SIG                    = 0x00000000,
APG_AUDIO_CRC_CH1_SIG                    = 0x00000001,
APG_AUDIO_CRC_CH2_SIG                    = 0x00000002,
APG_AUDIO_CRC_CH3_SIG                    = 0x00000003,
APG_AUDIO_CRC_CH4_SIG                    = 0x00000004,
APG_AUDIO_CRC_CH5_SIG                    = 0x00000005,
APG_AUDIO_CRC_CH6_SIG                    = 0x00000006,
APG_AUDIO_CRC_CH7_SIG                    = 0x00000007,
APG_AUDIO_CRC_RESERVED_8                 = 0x00000008,
APG_AUDIO_CRC_RESERVED_9                 = 0x00000009,
APG_AUDIO_CRC_RESERVED_10                = 0x0000000a,
APG_AUDIO_CRC_RESERVED_11                = 0x0000000b,
APG_AUDIO_CRC_RESERVED_12                = 0x0000000c,
APG_AUDIO_CRC_RESERVED_13                = 0x0000000d,
APG_AUDIO_CRC_RESERVED_14                = 0x0000000e,
APG_AUDIO_CRC_RESERVED_15                = 0x0000000f,
}


/*
 * APG_AUDIO_CRC_CONTROL_CONT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum APG_AUDIO_CRC_CONTROL_CONT {
APG_AUDIO_CRC_ONESHOT                    = 0x00000000,
APG_AUDIO_CRC_CONTINUOUS                 = 0x00000001,
}


/*
 * APG_DBG_ACP_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum APG_DBG_ACP_TYPE {
APG_ACP_TYPE_GENERIC_AUDIO               = 0x00000000,
APG_ACP_TYPE_ICE60958_AUDIO              = 0x00000001,
APG_ACP_TYPE_DVD_AUDIO                   = 0x00000002,
APG_ACP_TYPE_SUPER_AUDIO_CD              = 0x00000003,
}


/*
 * APG_DBG_AUDIO_DTO_BASE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum APG_DBG_AUDIO_DTO_BASE {
BASE_RATE_48KHZ                          = 0x00000000,
BASE_RATE_44P1KHZ                        = 0x00000001,
}


/*
 * APG_DBG_AUDIO_DTO_DIV enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum APG_DBG_AUDIO_DTO_DIV {
DIVISOR_BY1                              = 0x00000000,
DIVISOR_BY2_RESERVED                     = 0x00000001,
DIVISOR_BY3                              = 0x00000002,
DIVISOR_BY4_RESERVED                     = 0x00000003,
DIVISOR_BY5_RESERVED                     = 0x00000004,
DIVISOR_BY6_RESERVED                     = 0x00000005,
DIVISOR_BY7_RESERVED                     = 0x00000006,
DIVISOR_BY8_RESERVED                     = 0x00000007,
}


/*
 * APG_DBG_AUDIO_DTO_MULTI enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum APG_DBG_AUDIO_DTO_MULTI {
MULTIPLE_BY1                             = 0x00000000,
MULTIPLE_BY2                             = 0x00000001,
MULTIPLE_BY3_RESERVED                    = 0x00000002,
MULTIPLE_BY4                             = 0x00000003,
MULTIPLE_RESERVED                        = 0x00000004,
}


/*
 * APG_DBG_MUX_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum APG_DBG_MUX_SEL {
APG_FUNCTIONAL_MODE                      = 0x00000000,
APG_DEBUG_AUDIO_MODE                     = 0x00000001,
}


/*
 * APG_DP_ASP_CHANNEL_COUNT_OVERRIDE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum APG_DP_ASP_CHANNEL_COUNT_OVERRIDE {
APG_DP_ASP_CHANNEL_COUNT_FROM_AZ         = 0x00000000,
APG_DP_ASP_CHANNEL_COUNT_OVERRIDE_ENABLED = 0x00000001,
}


/*
 * APG_MEM_POWER_STATE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum APG_MEM_POWER_STATE {
APG_MEM_POWER_STATE_ON                   = 0x00000000,
APG_MEM_POWER_STATE_LS                   = 0x00000001,
APG_MEM_POWER_STATE_DS                   = 0x00000002,
APG_MEM_POWER_STATE_SD                   = 0x00000003,
}


/*
 * APG_MEM_PWR_DIS_CTRL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum APG_MEM_PWR_DIS_CTRL {
APG_MEM_ENABLE_MEM_PWR_CTRL              = 0x00000000,
APG_MEM_DISABLE_MEM_PWR_CTRL             = 0x00000001,
}


/*
 * APG_MEM_PWR_FORCE_CTRL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum APG_MEM_PWR_FORCE_CTRL {
APG_MEM_NO_FORCE_REQUEST                 = 0x00000000,
APG_MEM_FORCE_LIGHT_SLEEP_REQUEST        = 0x00000001,
APG_MEM_FORCE_DEEP_SLEEP_REQUEST         = 0x00000002,
APG_MEM_FORCE_SHUT_DOWN_REQUEST          = 0x00000003,
}


/*
 * APG_PACKET_CONTROL_ACP_SOURCE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum APG_PACKET_CONTROL_ACP_SOURCE {
APG_ACP_SOURCE_NO_OVERRIDE               = 0x00000000,
APG_ACP_OVERRIDE                         = 0x00000001,
}


/*
 * APG_PACKET_CONTROL_AUDIO_INFO_SOURCE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum APG_PACKET_CONTROL_AUDIO_INFO_SOURCE {
APG_INFOFRAME_SOURCE_NO_OVERRIDE         = 0x00000000,
APG_INFOFRAME_SOURCE_FROM_APG_REGISTERS  = 0x00000001,
}


/*
 * APG_RAMP_CONTROL_SIGN enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum APG_RAMP_CONTROL_SIGN {
APG_RAMP_SIGNED                          = 0x00000000,
APG_RAMP_UNSIGNED                        = 0x00000001,
}


/*******************************************************
 * DCIO Enums
 *******************************************************/

/*
 * DCIO_BL_PWM_GRP1_FRAME_START_DISP_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_BL_PWM_GRP1_FRAME_START_DISP_SEL {
DCIO_BL_PWM_GRP1_FRAME_START_DISP_SEL_CONTROLLER1 = 0x00000000,
DCIO_BL_PWM_GRP1_FRAME_START_DISP_SEL_CONTROLLER2 = 0x00000001,
DCIO_BL_PWM_GRP1_FRAME_START_DISP_SEL_CONTROLLER3 = 0x00000002,
DCIO_BL_PWM_GRP1_FRAME_START_DISP_SEL_CONTROLLER4 = 0x00000003,
DCIO_BL_PWM_GRP1_FRAME_START_DISP_SEL_CONTROLLER5 = 0x00000004,
DCIO_BL_PWM_GRP1_FRAME_START_DISP_SEL_CONTROLLER6 = 0x00000005,
}


/*
 * DCIO_CLOCK_CNTL_DCIO_TEST_CLK_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_CLOCK_CNTL_DCIO_TEST_CLK_SEL {
DCIO_TEST_CLK_SEL_DISPCLK                = 0x00000000,
DCIO_TEST_CLK_SEL_GATED_DISPCLK          = 0x00000001,
DCIO_TEST_CLK_SEL_SOCCLK                 = 0x00000002,
}


/*
 * DCIO_CLOCK_CNTL_DISPCLK_R_DCIO_GATE_DIS enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_CLOCK_CNTL_DISPCLK_R_DCIO_GATE_DIS {
DCIO_DISPCLK_R_DCIO_GATE_DISABLE         = 0x00000000,
DCIO_DISPCLK_R_DCIO_GATE_ENABLE          = 0x00000001,
}


/*
 * DCIO_DBG_ASYNC_4BIT_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DBG_ASYNC_4BIT_SEL {
DCIO_DBG_ASYNC_4BIT_SEL_3TO0             = 0x00000000,
DCIO_DBG_ASYNC_4BIT_SEL_7TO4             = 0x00000001,
DCIO_DBG_ASYNC_4BIT_SEL_11TO8            = 0x00000002,
DCIO_DBG_ASYNC_4BIT_SEL_15TO12           = 0x00000003,
DCIO_DBG_ASYNC_4BIT_SEL_19TO16           = 0x00000004,
DCIO_DBG_ASYNC_4BIT_SEL_23TO20           = 0x00000005,
DCIO_DBG_ASYNC_4BIT_SEL_27TO24           = 0x00000006,
DCIO_DBG_ASYNC_4BIT_SEL_31TO28           = 0x00000007,
}


/*
 * DCIO_DBG_ASYNC_BLOCK_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DBG_ASYNC_BLOCK_SEL {
DCIO_DBG_ASYNC_BLOCK_SEL_OVERRIDE        = 0x00000000,
DCIO_DBG_ASYNC_BLOCK_SEL_DCCG            = 0x00000001,
DCIO_DBG_ASYNC_BLOCK_SEL_DCIO            = 0x00000002,
DCIO_DBG_ASYNC_BLOCK_SEL_DIO             = 0x00000003,
}


/*
 * DCIO_DCRXPHY_SOFT_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DCRXPHY_SOFT_RESET {
DCIO_DCRXPHY_SOFT_RESET_DEASSERT         = 0x00000000,
DCIO_DCRXPHY_SOFT_RESET_ASSERT           = 0x00000001,
}


/*
 * DCIO_DC_GENERICA_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DC_GENERICA_SEL {
DCIO_GENERICA_SEL_STEREOSYNC             = 0x00000001,
DCIO_GENERICA_SEL_GENERICA_DCCG          = 0x0000000a,
DCIO_GENERICA_SEL_SYNCEN                 = 0x0000000b,
}


/*
 * DCIO_DC_GENERICB_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DC_GENERICB_SEL {
DCIO_GENERICB_SEL_STEREOSYNC             = 0x00000001,
DCIO_GENERICB_SEL_GENERICB_DCCG          = 0x0000000a,
DCIO_GENERICB_SEL_SYNCEN                 = 0x0000000b,
}


/*
 * DCIO_DC_GENERIC_UNIPHY_FBDIV_CLK_DIV2_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DC_GENERIC_UNIPHY_FBDIV_CLK_DIV2_SEL {
DCIO_UNIPHYA_TEST_FBDIV_CLK_DIV2         = 0x00000000,
DCIO_UNIPHYB_TEST_FBDIV_CLK_DIV2         = 0x00000001,
DCIO_UNIPHYC_TEST_FBDIV_CLK_DIV2         = 0x00000002,
DCIO_UNIPHYD_TEST_FBDIV_CLK_DIV2         = 0x00000003,
DCIO_UNIPHYE_TEST_FBDIV_CLK_DIV2         = 0x00000004,
DCIO_UNIPHYF_TEST_FBDIV_CLK_DIV2         = 0x00000005,
DCIO_UNIPHYG_TEST_FBDIV_CLK_DIV2         = 0x00000006,
}


/*
 * DCIO_DC_GENERIC_UNIPHY_FBDIV_CLK_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DC_GENERIC_UNIPHY_FBDIV_CLK_SEL {
DCIO_UNIPHYA_FBDIV_CLK                   = 0x00000000,
DCIO_UNIPHYB_FBDIV_CLK                   = 0x00000001,
DCIO_UNIPHYC_FBDIV_CLK                   = 0x00000002,
DCIO_UNIPHYD_FBDIV_CLK                   = 0x00000003,
DCIO_UNIPHYE_FBDIV_CLK                   = 0x00000004,
DCIO_UNIPHYF_FBDIV_CLK                   = 0x00000005,
DCIO_UNIPHYG_FBDIV_CLK                   = 0x00000006,
}


/*
 * DCIO_DC_GENERIC_UNIPHY_FBDIV_SSC_CLK_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DC_GENERIC_UNIPHY_FBDIV_SSC_CLK_SEL {
DCIO_UNIPHYA_FBDIV_SSC_CLK               = 0x00000000,
DCIO_UNIPHYB_FBDIV_SSC_CLK               = 0x00000001,
DCIO_UNIPHYC_FBDIV_SSC_CLK               = 0x00000002,
DCIO_UNIPHYD_FBDIV_SSC_CLK               = 0x00000003,
DCIO_UNIPHYE_FBDIV_SSC_CLK               = 0x00000004,
DCIO_UNIPHYF_FBDIV_SSC_CLK               = 0x00000005,
DCIO_UNIPHYG_FBDIV_SSC_CLK               = 0x00000006,
}


/*
 * DCIO_DC_GENERIC_UNIPHY_REFDIV_CLK_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DC_GENERIC_UNIPHY_REFDIV_CLK_SEL {
DCIO_UNIPHYA_TEST_REFDIV_CLK             = 0x00000000,
DCIO_UNIPHYB_TEST_REFDIV_CLK             = 0x00000001,
DCIO_UNIPHYC_TEST_REFDIV_CLK             = 0x00000002,
DCIO_UNIPHYD_TEST_REFDIV_CLK             = 0x00000003,
DCIO_UNIPHYE_TEST_REFDIV_CLK             = 0x00000004,
DCIO_UNIPHYF_TEST_REFDIV_CLK             = 0x00000005,
DCIO_UNIPHYG_TEST_REFDIV_CLK             = 0x00000006,
}


/*
 * DCIO_DC_GPIO_DEBUG_DPRX_LOOPBACK_ENABLE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DC_GPIO_DEBUG_DPRX_LOOPBACK_ENABLE {
DCIO_DPRX_LOOPBACK_ENABLE_NORMAL         = 0x00000000,
DCIO_DPRX_LOOPBACK_ENABLE_LOOP           = 0x00000001,
}


/*
 * DCIO_DC_GPU_TIMER_READ_SELECT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DC_GPU_TIMER_READ_SELECT {
DCIO_GPU_TIMER_READ_SELECT_LOWER_D1_V_UPDATE = 0x00000000,
DCIO_GPU_TIMER_READ_SELECT_UPPER_D1_V_UPDATE = 0x00000001,
DCIO_GPU_TIMER_READ_SELECT_LOWER_D1_P_FLIP = 0x00000002,
DCIO_GPU_TIMER_READ_SELECT_UPPER_D1_P_FLIP = 0x00000003,
DCIO_GPU_TIMER_READ_SELECT_LOWER_D1_VSYNC_NOM = 0x00000004,
DCIO_GPU_TIMER_READ_SELECT_UPPER_D1_VSYNC_NOM = 0x00000005,
}


/*
 * DCIO_DC_GPU_TIMER_START_POSITION enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DC_GPU_TIMER_START_POSITION {
DCIO_GPU_TIMER_START_0_END_27            = 0x00000000,
DCIO_GPU_TIMER_START_1_END_28            = 0x00000001,
DCIO_GPU_TIMER_START_2_END_29            = 0x00000002,
DCIO_GPU_TIMER_START_3_END_30            = 0x00000003,
DCIO_GPU_TIMER_START_4_END_31            = 0x00000004,
DCIO_GPU_TIMER_START_6_END_33            = 0x00000005,
DCIO_GPU_TIMER_START_8_END_35            = 0x00000006,
DCIO_GPU_TIMER_START_10_END_37           = 0x00000007,
}


/*
 * DCIO_DC_REF_CLK_CNTL_GENLK_CLK_OUTPUT_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DC_REF_CLK_CNTL_GENLK_CLK_OUTPUT_SEL {
DCIO_GENLK_CLK_OUTPUT_SEL_DISABLE        = 0x00000000,
DCIO_GENLK_CLK_OUTPUT_SEL_PPLL1          = 0x00000001,
DCIO_GENLK_CLK_OUTPUT_SEL_PPLL2          = 0x00000002,
DCIO_GENLK_CLK_OUTPUT_SEL_RESERVED_VALUE3 = 0x00000003,
}


/*
 * DCIO_DC_REF_CLK_CNTL_HSYNCA_OUTPUT_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DC_REF_CLK_CNTL_HSYNCA_OUTPUT_SEL {
DCIO_HSYNCA_OUTPUT_SEL_DISABLE           = 0x00000000,
DCIO_HSYNCA_OUTPUT_SEL_PPLL1             = 0x00000001,
DCIO_HSYNCA_OUTPUT_SEL_PPLL2             = 0x00000002,
DCIO_HSYNCA_OUTPUT_SEL_RESERVED          = 0x00000003,
}


/*
 * DCIO_DIO_EXT_VSYNC_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DIO_EXT_VSYNC_MASK {
DCIO_EXT_VSYNC_MASK_NONE                 = 0x00000000,
DCIO_EXT_VSYNC_MASK_PIPE0                = 0x00000001,
DCIO_EXT_VSYNC_MASK_PIPE1                = 0x00000002,
DCIO_EXT_VSYNC_MASK_PIPE2                = 0x00000003,
DCIO_EXT_VSYNC_MASK_PIPE3                = 0x00000004,
DCIO_EXT_VSYNC_MASK_PIPE4                = 0x00000005,
DCIO_EXT_VSYNC_MASK_PIPE5                = 0x00000006,
DCIO_EXT_VSYNC_MASK_NONE_DUPLICATE       = 0x00000007,
}


/*
 * DCIO_DIO_OTG_EXT_VSYNC_MUX enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DIO_OTG_EXT_VSYNC_MUX {
DCIO_EXT_VSYNC_MUX_SWAPLOCKB             = 0x00000000,
DCIO_EXT_VSYNC_MUX_OTG0                  = 0x00000001,
DCIO_EXT_VSYNC_MUX_OTG1                  = 0x00000002,
DCIO_EXT_VSYNC_MUX_OTG2                  = 0x00000003,
DCIO_EXT_VSYNC_MUX_OTG3                  = 0x00000004,
DCIO_EXT_VSYNC_MUX_OTG4                  = 0x00000005,
DCIO_EXT_VSYNC_MUX_OTG5                  = 0x00000006,
DCIO_EXT_VSYNC_MUX_GENERICB              = 0x00000007,
}


/*
 * DCIO_DPCS_INTERRUPT_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DPCS_INTERRUPT_MASK {
DCIO_DPCS_INTERRUPT_DISABLE              = 0x00000000,
DCIO_DPCS_INTERRUPT_ENABLE               = 0x00000001,
}


/*
 * DCIO_DPCS_INTERRUPT_TYPE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DPCS_INTERRUPT_TYPE {
DCIO_DPCS_INTERRUPT_TYPE_LEVEL_BASED     = 0x00000000,
DCIO_DPCS_INTERRUPT_TYPE_PULSE_BASED     = 0x00000001,
}


/*
 * DCIO_DSYNC_SOFT_RESET enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_DSYNC_SOFT_RESET {
DCIO_DSYNC_SOFT_RESET_DEASSERT           = 0x00000000,
DCIO_DSYNC_SOFT_RESET_ASSERT             = 0x00000001,
}


/*
 * DCIO_GENLK_CLK_GSL_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_GENLK_CLK_GSL_MASK {
DCIO_GENLK_CLK_GSL_MASK_NO               = 0x00000000,
DCIO_GENLK_CLK_GSL_MASK_TIMING           = 0x00000001,
DCIO_GENLK_CLK_GSL_MASK_STEREO           = 0x00000002,
}


/*
 * DCIO_GENLK_VSYNC_GSL_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_GENLK_VSYNC_GSL_MASK {
DCIO_GENLK_VSYNC_GSL_MASK_NO             = 0x00000000,
DCIO_GENLK_VSYNC_GSL_MASK_TIMING         = 0x00000001,
DCIO_GENLK_VSYNC_GSL_MASK_STEREO         = 0x00000002,
}


/*
 * DCIO_GSL_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_GSL_SEL {
DCIO_GSL_SEL_GROUP_0                     = 0x00000000,
DCIO_GSL_SEL_GROUP_1                     = 0x00000001,
DCIO_GSL_SEL_GROUP_2                     = 0x00000002,
}


/*
 * DCIO_PHY_HPO_ENC_SRC_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_PHY_HPO_ENC_SRC_SEL {
HPO_SRC0                                 = 0x00000000,
HPO_SRC_RESERVED                         = 0x00000001,
}


/*
 * DCIO_SWAPLOCK_A_GSL_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_SWAPLOCK_A_GSL_MASK {
DCIO_SWAPLOCK_A_GSL_MASK_NO              = 0x00000000,
DCIO_SWAPLOCK_A_GSL_MASK_TIMING          = 0x00000001,
DCIO_SWAPLOCK_A_GSL_MASK_STEREO          = 0x00000002,
}


/*
 * DCIO_SWAPLOCK_B_GSL_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_SWAPLOCK_B_GSL_MASK {
DCIO_SWAPLOCK_B_GSL_MASK_NO              = 0x00000000,
DCIO_SWAPLOCK_B_GSL_MASK_TIMING          = 0x00000001,
DCIO_SWAPLOCK_B_GSL_MASK_STEREO          = 0x00000002,
}


/*
 * DCIO_UNIPHY_CHANNEL_XBAR_SOURCE enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_UNIPHY_CHANNEL_XBAR_SOURCE {
DCIO_UNIPHY_CHANNEL_XBAR_SOURCE_CH0      = 0x00000000,
DCIO_UNIPHY_CHANNEL_XBAR_SOURCE_CH1      = 0x00000001,
DCIO_UNIPHY_CHANNEL_XBAR_SOURCE_CH2      = 0x00000002,
DCIO_UNIPHY_CHANNEL_XBAR_SOURCE_CH3      = 0x00000003,
}


/*
 * DCIO_UNIPHY_IMPCAL_SEL enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_UNIPHY_IMPCAL_SEL {
DCIO_UNIPHY_IMPCAL_SEL_TEMPERATURE       = 0x00000000,
DCIO_UNIPHY_IMPCAL_SEL_BINARY            = 0x00000001,
}


/*
 * DCIO_UNIPHY_LINK_CNTL_CHANNEL_INVERT enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_UNIPHY_LINK_CNTL_CHANNEL_INVERT {
DCIO_UNIPHY_CHANNEL_NO_INVERSION         = 0x00000000,
DCIO_UNIPHY_CHANNEL_INVERTED             = 0x00000001,
}


/*
 * DCIO_UNIPHY_LINK_CNTL_ENABLE_HPD_MASK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIO_UNIPHY_LINK_CNTL_ENABLE_HPD_MASK {
DCIO_UNIPHY_LINK_ENABLE_HPD_MASK_DISALLOW = 0x00000000,
DCIO_UNIPHY_LINK_ENABLE_HPD_MASK_ALLOW   = 0x00000001,
DCIO_UNIPHY_LINK_ENABLE_HPD_MASK_ALLOW_DEBOUNCED = 0x00000002,
DCIO_UNIPHY_LINK_ENABLE_HPD_MASK_ALLOW_TOGGLE_FILTERED = 0x00000003,
}


/*******************************************************
 * DCIO_CHIP Enums
 *******************************************************/

/*
 * DCIOCHIP_AUX_ALL_PWR_OK enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIOCHIP_AUX_ALL_PWR_OK {
DCIOCHIP_AUX_ALL_PWR_OK_0                = 0x00000000,
DCIOCHIP_AUX_ALL_PWR_OK_1                = 0x00000001,
}


/*
 * DCIOCHIP_AUX_CSEL0P9 enum
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DCIOCHIP_AUX_CSEL0P9 {
DCIOCHIP_AUX_CSEL_DEC1P0                 = 0x00000000,
DCIOCHIP_AUX_CSEL_DEC0P9                 = 0x00000001,
}


/*
 * DCIOCHIP_AUX_CSEL1P1 enum
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
