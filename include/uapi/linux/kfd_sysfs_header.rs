/* SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR MIT */
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
 */

/* Capability bits in node properties */
pub const HSA_CAP_HOT_PLUGGABLE: u32 = 0x00000001;
pub const HSA_CAP_ATS_PRESENT: u32 = 0x00000002;
pub const HSA_CAP_SHARED_WITH_GRAPHICS: u32 = 0x00000004;
pub const HSA_CAP_QUEUE_SIZE_POW2: u32 = 0x00000008;
pub const HSA_CAP_QUEUE_SIZE_32BIT: u32 = 0x00000010;
pub const HSA_CAP_QUEUE_IDLE_EVENT: u32 = 0x00000020;
pub const HSA_CAP_VA_LIMIT: u32 = 0x00000040;
pub const HSA_CAP_WATCH_POINTS_SUPPORTED: u32 = 0x00000080;
pub const HSA_CAP_WATCH_POINTS_TOTALBITS_MASK: u32 = 0x00000f00;
pub const HSA_CAP_WATCH_POINTS_TOTALBITS_SHIFT: u32 = 8;
pub const HSA_CAP_DOORBELL_TYPE_TOTALBITS_MASK: u32 = 0x00003000;
pub const HSA_CAP_DOORBELL_TYPE_TOTALBITS_SHIFT: u32 = 12;
pub const HSA_CAP_DOORBELL_TYPE_PRE_1_0: u32 = 0x0;
pub const HSA_CAP_DOORBELL_TYPE_1_0: u32 = 0x1;
pub const HSA_CAP_DOORBELL_TYPE_2_0: u32 = 0x2;
pub const HSA_CAP_AQL_QUEUE_DOUBLE_MAP: u32 = 0x00004000;
pub const HSA_CAP_TRAP_DEBUG_SUPPORT: u32 = 0x00008000;
pub const HSA_CAP_TRAP_DEBUG_WAVE_LAUNCH_TRAP_OVERRIDE_SUPPORTED: u32 = 0x00010000;
pub const HSA_CAP_TRAP_DEBUG_WAVE_LAUNCH_MODE_SUPPORTED: u32 = 0x00020000;
pub const HSA_CAP_TRAP_DEBUG_PRECISE_MEMORY_OPERATIONS_SUPPORTED: u32 = 0x00040000;
/* Old buggy user mode depends on this being 0 */
pub const HSA_CAP_RESERVED_WAS_SRAM_EDCSUPPORTED: u32 = 0x00080000;
pub const HSA_CAP_MEM_EDCSUPPORTED: u32 = 0x00100000;
pub const HSA_CAP_RASEVENTNOTIFY: u32 = 0x00200000;
pub const HSA_CAP_ASIC_REVISION_MASK: u32 = 0x03c00000;
pub const HSA_CAP_ASIC_REVISION_SHIFT: u32 = 22;
pub const HSA_CAP_SRAM_EDCSUPPORTED: u32 = 0x04000000;
pub const HSA_CAP_SVMAPI_SUPPORTED: u32 = 0x08000000;
pub const HSA_CAP_FLAGS_COHERENTHOSTACCESS: u32 = 0x10000000;
pub const HSA_CAP_TRAP_DEBUG_FIRMWARE_SUPPORTED: u32 = 0x20000000;
pub const HSA_CAP_TRAP_DEBUG_PRECISE_ALU_OPERATIONS_SUPPORTED: u32 = 0x40000000;
pub const HSA_CAP_PER_QUEUE_RESET_SUPPORTED: u32 = 0x80000000;
pub const HSA_CAP_RESERVED: u32 = 0x000f8000;

pub const HSA_CAP2_PER_SDMA_QUEUE_RESET_SUPPORTED: u32 = 0x00000001;
pub const HSA_CAP2_TRAP_DEBUG_LDS_OUT_OF_ADDR_RANGE_SUPPORTED: u32 = 0x00000002;
pub const HSA_CAP2_RESERVED: u32 = 0xfffffffc;

/* debug_prop bits in node properties */
pub const HSA_DBG_WATCH_ADDR_MASK_LO_BIT_MASK: u64 = 0x0000000f;
pub const HSA_DBG_WATCH_ADDR_MASK_LO_BIT_SHIFT: u64 = 0;
pub const HSA_DBG_WATCH_ADDR_MASK_HI_BIT_MASK: u64 = 0x000003f0;
pub const HSA_DBG_WATCH_ADDR_MASK_HI_BIT_SHIFT: u64 = 4;
pub const HSA_DBG_DISPATCH_INFO_ALWAYS_VALID: u64 = 0x00000400;
pub const HSA_DBG_WATCHPOINTS_EXCLUSIVE: u64 = 0x00000800;
pub const HSA_DBG_RESERVED: u64 = 0xfffffffffffff000;

/* Heap types in memory properties */
pub const HSA_MEM_HEAP_TYPE_SYSTEM: u32 = 0;
pub const HSA_MEM_HEAP_TYPE_FB_PUBLIC: u32 = 1;
pub const HSA_MEM_HEAP_TYPE_FB_PRIVATE: u32 = 2;
pub const HSA_MEM_HEAP_TYPE_GPU_GDS: u32 = 3;
pub const HSA_MEM_HEAP_TYPE_GPU_LDS: u32 = 4;
pub const HSA_MEM_HEAP_TYPE_GPU_SCRATCH: u32 = 5;

/* Flag bits in memory properties */
pub const HSA_MEM_FLAGS_HOT_PLUGGABLE: u32 = 0x00000001;
pub const HSA_MEM_FLAGS_NON_VOLATILE: u32 = 0x00000002;
pub const HSA_MEM_FLAGS_RESERVED: u32 = 0xfffffffc;

/* Cache types in cache properties */
pub const HSA_CACHE_TYPE_DATA: u32 = 0x00000001;
pub const HSA_CACHE_TYPE_INSTRUCTION: u32 = 0x00000002;
pub const HSA_CACHE_TYPE_CPU: u32 = 0x00000004;
pub const HSA_CACHE_TYPE_HSACU: u32 = 0x00000008;
pub const HSA_CACHE_TYPE_RESERVED: u32 = 0xfffffff0;

/* Link types in IO link properties (matches CRAT link types) */
pub const HSA_IOLINK_TYPE_UNDEFINED: u32 = 0;
pub const HSA_IOLINK_TYPE_HYPERTRANSPORT: u32 = 1;
pub const HSA_IOLINK_TYPE_PCIEXPRESS: u32 = 2;
pub const HSA_IOLINK_TYPE_AMBA: u32 = 3;
pub const HSA_IOLINK_TYPE_MIPI: u32 = 4;
pub const HSA_IOLINK_TYPE_QPI_1_1: u32 = 5;
pub const HSA_IOLINK_TYPE_RESERVED1: u32 = 6;
pub const HSA_IOLINK_TYPE_RESERVED2: u32 = 7;
pub const HSA_IOLINK_TYPE_RAPID_IO: u32 = 8;
pub const HSA_IOLINK_TYPE_INFINIBAND: u32 = 9;
pub const HSA_IOLINK_TYPE_RESERVED3: u32 = 10;
pub const HSA_IOLINK_TYPE_XGMI: u32 = 11;
pub const HSA_IOLINK_TYPE_XGOP: u32 = 12;
pub const HSA_IOLINK_TYPE_GZ: u32 = 13;
pub const HSA_IOLINK_TYPE_ETHERNET_RDMA: u32 = 14;
pub const HSA_IOLINK_TYPE_RDMA_OTHER: u32 = 15;
pub const HSA_IOLINK_TYPE_OTHER: u32 = 16;

/* Flag bits in IO link properties (matches CRAT flags, excluding the
 * bi-directional flag, which is not offially part of the CRAT spec, and
 * only used internally in KFD)
 */
pub const HSA_IOLINK_FLAGS_ENABLED: u32 = 1 << 0;
pub const HSA_IOLINK_FLAGS_NON_COHERENT: u32 = 1 << 1;
pub const HSA_IOLINK_FLAGS_NO_ATOMICS_32_BIT: u32 = 1 << 2;
pub const HSA_IOLINK_FLAGS_NO_ATOMICS_64_BIT: u32 = 1 << 3;
pub const HSA_IOLINK_FLAGS_NO_PEER_TO_PEER_DMA: u32 = 1 << 4;
pub const HSA_IOLINK_FLAGS_RESERVED: u32 = 0xffffffe0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
