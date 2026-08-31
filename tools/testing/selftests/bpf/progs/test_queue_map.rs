// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Politecnico di Torino

const MAP_TYPE: u32 = BPF_MAP_TYPE_QUEUE;

// C source included "test_queue_stack_map.h" here; the shared test body is a
// future dependency and is intentionally not expanded in this isolated pass.
