# SPDX-License-Identifier: GPL-2.0
# Inactive source registration. All sched_ext projections must ultimately share
# canonical native type identities; this file does not qualify that composition.
ifeq ($(CONFIG_RUST_SCHED_EXT_CORE),y)
$(error SOURCE ONLY HOLD: Rust sched_ext core bindings and BTF are unqualified)
endif
targets += bindings/sched_ext_core_generated.rs
always-$(CONFIG_RUST_SCHED_EXT_CORE) += bindings/sched_ext_core_generated.rs
$(obj)/bindings/sched_ext_core_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/ext/sched_ext_core_bindgen_parameters)
$(obj)/bindings/sched_ext_core_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_ext_core_generated.rs: $(srctree)/kernel/sched/ext/sched_ext_core_bindings.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_slice_cursor_bindings.h \
    $(srctree)/kernel/sched/ext/internal.h $(srctree)/kernel/sched/ext/cid.h \
    $(srctree)/kernel/sched/ext/arena.h $(srctree)/kernel/sched/ext/idle.h \
    $(srctree)/kernel/sched/ext/sub.h $(srctree)/kernel/sched/ext/inlines.h \
    $(srctree)/kernel/sched/ext/sched_ext_core_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_EXT_CORE),y)
$(obj)/bindings.o: $(obj)/bindings/sched_ext_core_generated.rs
endif
