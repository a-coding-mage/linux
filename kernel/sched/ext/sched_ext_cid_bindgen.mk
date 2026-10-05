# SPDX-License-Identifier: GPL-2.0-only
# Source registration only; no configured native generation is admitted.
ifeq ($(CONFIG_RUST_SCHED_EXT_CID),y)
$(error SOURCE ONLY HOLD: Rust sched_ext CID bindings and BTF are unqualified)
endif
targets += bindings/sched_ext_cid_generated.rs
always-$(CONFIG_RUST_SCHED_EXT_CID) += bindings/sched_ext_cid_generated.rs
$(obj)/bindings/sched_ext_cid_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/ext/sched_ext_cid_bindgen_parameters)
$(obj)/bindings/sched_ext_cid_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_ext_cid_generated.rs: $(srctree)/kernel/sched/ext/sched_ext_cid_bindings.h \
    $(srctree)/kernel/sched/ext/internal.h $(srctree)/kernel/sched/ext/cid.h \
    $(srctree)/kernel/sched/ext/sched_ext_cid_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_EXT_CID),y)
$(obj)/bindings.o: $(obj)/bindings/sched_ext_cid_generated.rs
endif
