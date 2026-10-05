/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_VMSTAT_NATIVE_BINDINGS_H
#define RUST_VMSTAT_NATIVE_BINDINGS_H
#include "vmstat_native_includes.h"
#define RV_UINT(n) static const unsigned int RUST_VMSTAT_##n = (n)
#define RV_ULONG(n) static const unsigned long RUST_VMSTAT_##n = (n)
RV_UINT(NR_PAGE_ORDERS);
RV_UINT(MAX_PAGE_ORDER);
RV_UINT(PAGE_SHIFT);
RV_ULONG(PAGE_SIZE);
RV_ULONG(HPAGE_PMD_NR);
RV_UINT(HZ);
RV_UINT(MAX_NUMNODES);
RV_UINT(WQ_MEM_RECLAIM);
RV_UINT(WQ_PERCPU);
static const int RUST_VMSTAT_ENOMEM = ENOMEM;
static const gfp_t RUST_VMSTAT_GFP_KERNEL = GFP_KERNEL;
#if defined(CONFIG_NUMA) && defined(CONFIG_PROC_FS)
enum {
 RUST_VMSTAT_SYSCTL_ZERO_INDEX = (const int *)SYSCTL_ZERO - sysctl_vals,
 RUST_VMSTAT_SYSCTL_ONE_INDEX = (const int *)SYSCTL_ONE - sysctl_vals,
};
#endif
#if defined(CONFIG_CC_HAS_SANE_FUNCTION_ALIGNMENT) || CONFIG_FUNCTION_ALIGNMENT == 0
static const bool RUST_VMSTAT_CFG_INIT_COLD = true;
#endif
#define RUST_VMSTAT_NATIVE_READ_MOSTLY __stringify(__read_mostly)
#if IS_ENABLED(CONFIG_ZSMALLOC)
static const bool RUST_VMSTAT_CFG_ENABLED_ZSMALLOC = true;
#endif
#if IS_ENABLED(CONFIG_SHADOW_CALL_STACK)
static const bool RUST_VMSTAT_CFG_ENABLED_SHADOW_CALL_STACK = true;
#endif
#if THREAD_SIZE > 1024
static const bool RUST_VMSTAT_CFG_THREAD_GT_1024 = true;
#endif
#if THREAD_SIZE > 2048
static const bool RUST_VMSTAT_CFG_THREAD_GT_2048 = true;
#endif
#if THREAD_SIZE > 4096
static const bool RUST_VMSTAT_CFG_THREAD_GT_4096 = true;
#endif
#if THREAD_SIZE > 8192
static const bool RUST_VMSTAT_CFG_THREAD_GT_8192 = true;
#endif
#if THREAD_SIZE > 16384
static const bool RUST_VMSTAT_CFG_THREAD_GT_16384 = true;
#endif
#if THREAD_SIZE > 32768
static const bool RUST_VMSTAT_CFG_THREAD_GT_32768 = true;
#endif
#if THREAD_SIZE > 65536
static const bool RUST_VMSTAT_CFG_THREAD_GT_65536 = true;
#endif
#undef RV_UINT
#undef RV_ULONG
#define RVM(ret, name, args, body) ret name args;
#include "vmstat_native_primitives.h"
#undef RVM
#endif
