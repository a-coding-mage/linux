/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_SHMEM_BINDINGS_H
#define RUST_SHMEM_BINDINGS_H
#include "shmem_native_types.h"
static_assert(sizeof(__kernel_size_t) == sizeof(void *));
static_assert(sizeof(__kernel_ssize_t) == sizeof(void *));
static_assert(sizeof(__kernel_ptrdiff_t) == sizeof(void *));
static_assert((__kernel_size_t)-1 > 0);
static_assert((__kernel_ssize_t)-1 < 0);
static_assert((__kernel_ptrdiff_t)-1 < 0);
#include "shmem_native_constants.h"
#include "shmem_native_primitives.h"
#if defined(CONFIG_CC_HAS_SANE_FUNCTION_ALIGNMENT) || (CONFIG_FUNCTION_ALIGNMENT == 0)
static const bool RUST_SHMEM_CFG_INIT_COLD = true;
#endif
#endif
