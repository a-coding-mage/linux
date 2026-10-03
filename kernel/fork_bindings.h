/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_FORK_BINDINGS_H
#define RUST_FORK_BINDINGS_H
#include "fork_includes.h"
#include <trace/events/task.h>
#include "fork_storage_types.h"
#include "fork_header_primitives.h"
#include "fork_task_primitives.h"
#include "fork_mm_primitives.h"
#ifdef CONFIG_SCHED_MM_CID
/* Bindgen-only typed view of the native BIT() expression; no runtime storage. */
static const unsigned int RUST_FORK_MM_CID_UNSET = MM_CID_UNSET;
#endif
#endif
