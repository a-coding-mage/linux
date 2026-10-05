# SPDX-License-Identifier: GPL-2.0
# Source-only gate: no object or generated prerequisite is selected here.
ifeq ($(CONFIG_RUST_SCHED_STATS),y)
$(error SOURCE ONLY HOLD: Rust scheduler statistics ABI, policy and behavior are unqualified)
endif

# FUTURE ADMISSION PLAN, inactive and requiring separate authorization:
# - Preserve build_utility.c's CONFIG_SCHEDSTATS inclusion scope. At its stats.c
#   include, select exactly one owner, with explicit stats.rs and native leaf
#   object rules; do not rely on same-basename implicit rule selection.
# - Never include/call original stats.c bodies as a selected-owner fallback.
# - The native leaf unit owns only field/header primitives, the seq_operations
#   table and the original subsys_initcall adapter. Rust owns all algorithms.
# - Gate objects, bindings, every generated prerequisite and alternate compiler
#   output path by this owner. Retain the selected-owner error during the hold.
# - Inherit build_utility.c policy: scheduler KCOV/KCSAN policy, KCSAN barrier
#   instrumentation, configured DISABLE_BRANCH_PROFILING, native CFI/KCFI and
#   architecture/compiler protections. Add no warning or instrumentation waiver.
# - Inputs include stats.rs, sched_stats_bindings.h, native headers/configuration,
#   sched_stats_bindgen_parameters and native policy/owner manifests.
