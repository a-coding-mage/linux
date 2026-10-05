# SPDX-License-Identifier: GPL-2.0-only
# Source proposal only. Do not remove this gate to "fix" a build. It records
# unfinished native/compiler-policy/architecture and source-review work.
ifeq ($(CONFIG_RUST_SCHED_CORE),y)
$(error Rust scheduler core source is not admitted: native ABI/policy and independent body review remain open)
endif

# FUTURE ADMISSION PLAN (inactive): keep the original core.o ordering; supply
# an explicit core.rs -> core.o rule to override make's existing core.c rule.
# The enabled owner must be exactly Rust core.o plus sched_core_native.o.
# Do not link core.c, include its bodies, or rename-and-forward C algorithms.
#
# Preserve these original Makefile properties for both owner/native leaves:
# CONTEXT_ANALYSIS_core.o := y
# KCOV_INSTRUMENT := n
# KCSAN_SANITIZE := n
# KCSAN_INSTRUMENT_BARRIERS := y
# CFLAGS_core.o := $(PROFILING) -fno-omit-frame-pointer
#   when CONFIG_SCHED_OMIT_FRAME_POINTER != y
#
# Native compiler policy must be extended and independently reviewed before
# including scripts/Makefile.rust-native-policy for this owner. Existing
# scheduler KCOV/KCSAN exemptions are inherited, not broadened. The barrier
# instrumentation setting must remain effective at Rust memory-order sites.
# Do not add warning suppressions, disable stack protection, strip CFI/KCFI,
# lower optimization, or force unsupported configs to reach a successful link.
#
# Required explicit future dependencies (not an executable rule here):
# core.rs core_foundation.rs core_foundation_*.rs core_tail.rs core_tail_*.rs
# $(objtree)/rust/bindings/sched_core_generated.rs
# sched_core_bindings.h sched_core_constants.h sched_core_native_leaves.def
# sched_core_private.h sched_core_callbacks.h sched_core_header_leaves.def
# sched_core_state_leaves.def sched_core_dynamic_leaves.def sched_core_diagnostics.def
# sched_core_state.inc sched_core_dynamic.inc sched_core_registration.inc
# sched_core_metadata.inc core_native_arch.rs
# native policy source, owners manifest, scripts and configured headers
#
# .o/.s/.ll/.rsi/.lst and listing paths must select the same Rust owner and
# protection policy. Merely adding an .o recipe would leave C listing routes.
