# SPDX-License-Identifier: GPL-2.0+
targets += bindings/rseq_generated.rs
always-$(CONFIG_RUST_RSEQ) += bindings/rseq_generated.rs
$(obj)/bindings/rseq_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/kernel/rseq_bindgen_parameters)
$(obj)/bindings/rseq_generated.rs: $(srctree)/kernel/rseq_bindings.h \
    $(srctree)/kernel/rseq_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
