# SPDX-License-Identifier: GPL-2.0-only
# Source-only explicit proposal for rust/Makefile inclusion after qualification.
# No bindgen/configuration has been run during this source phase.
targets += bindings/sched_core_generated.rs
always-$(CONFIG_RUST_SCHED_CORE) += bindings/sched_core_generated.rs
$(obj)/bindings/sched_core_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/sched_core_bindgen_parameters)
$(obj)/bindings/sched_core_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_core_generated.rs: $(srctree)/kernel/sched/sched_core_bindings.h \
    $(srctree)/kernel/sched/sched_core_constants.h \
    $(srctree)/kernel/sched/sched_core_native_leaves.def \
    $(srctree)/kernel/sched/sched_core_private.h \
    $(srctree)/kernel/sched/sched_core_callbacks.h \
    $(srctree)/kernel/sched/sched_core_header_leaves.def \
    $(srctree)/kernel/sched/sched_core_state_leaves.def \
    $(srctree)/kernel/sched/sched_core_dynamic_leaves.def \
    $(srctree)/kernel/sched/sched_core_diagnostics.def \
    $(srctree)/kernel/sched/sched_core_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
# OPEN: emit LUPOS_CORE_HAS_TIF_POLLING_NRFLAG from configured preprocessing, never
# infer it from a Rust architecture name or a handwritten numeric constant.

# The bindings crate owns generated code; inactive CORE builds add no dependency.
ifeq ($(CONFIG_RUST_SCHED_CORE),y)
$(obj)/bindings.o: $(obj)/bindings/sched_core_generated.rs
endif
