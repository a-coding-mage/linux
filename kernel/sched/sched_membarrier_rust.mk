# SPDX-License-Identifier: GPL-2.0-or-later
# This source-only proposal selects no objects and no generated-binding target.
ifeq ($(CONFIG_RUST_SCHED_MEMBARRIER),y)
$(error Rust scheduler membarrier source is not admitted: native ABI, ownership, protection and behavioral verification remain open)
endif

# Future, separately authorized admission must preserve build_utility.c's native
# compiler/instrumentation policy and original C/header/test oracles. Omit only
# its membarrier.c owner include when selecting the Rust owner plus primitives.
# Select explicit .rs ownership rather than an implicit same-basename .c rule.
# Put configured generated bindings in the existing bindings crate as
# sched_membarrier_native, using sched_membarrier_bindings.h and the parameter
# proposal. No duplicate syscall, initcall, mutex or per-CPU storage definitions.
# Keep CFI/KCFI, architecture protections, KCOV/KCSAN policy and instrumented
# scheduler barriers; do not introduce warning or protection waivers.
# Objects, assembly, IR, expanded Rust and listing outputs need the same policy.
# Removing a hard error is not admission or evidence of runtime correctness.
