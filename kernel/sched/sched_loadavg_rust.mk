# SPDX-License-Identifier: GPL-2.0
# Source-only gate. This file intentionally selects no object or binding target.
ifeq ($(CONFIG_RUST_SCHED_LOADAVG),y)
$(error Rust scheduler loadavg source is not admitted: native ABI, policy and behavioral verification remain open)
endif

# FUTURE ADMISSION PLAN (inactive; requires a separately authorized phase):
# - The native owner is loadavg.c included by build_utility.c, not build_policy.c.
# - Under this owner only, omit that include and select exactly loadavg.rs plus
#   sched_loadavg_primitives.c. Do not rename/include/forward loadavg.c bodies.
# - Use an explicit loadavg.rs object rule; an implicit rule could choose the
#   identically named loadavg.c. Object, assembly, IR, Rust-expanded and listing
#   outputs must use the same owner and protection policy.
# - Keep sched_loadavg_generated.rs in the existing bindings crate, with the
#   sched_loadavg_native module and all prerequisites gated by this owner.
# - Inputs include loadavg.rs, sched_loadavg_bindings.h,
#   sched_loadavg_bindgen_parameters, configured native headers, and the
#   reviewed native policy implementation and owners manifest.
# - Native compiler/instrumentation policy origin is build_utility.c. Preserve
#   scheduler KCOV/KCSAN policy and KCSAN_INSTRUMENT_BARRIERS, native CFI/KCFI,
#   architecture protections and DISABLE_BRANCH_PROFILING when configured.
# - Do not add warning allowances, substitute generated layouts/constants,
#   weaken native protection, or remove these hard errors to obtain a build.
