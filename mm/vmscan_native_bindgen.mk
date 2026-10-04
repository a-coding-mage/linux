# SPDX-License-Identifier: GPL-2.0-only
# Proposed include from rust/Makefile; generated from configured native headers.
targets += bindings/vmscan_native_generated.rs
always-$(CONFIG_RUST_VMSCAN) += bindings/vmscan_native_generated.rs
$(obj)/bindings/vmscan_native_generated.rs: private bindgen_target_flags = \
    $(shell grep -hEv '^\#|^$$' $(srctree)/mm/vmscan_native_bindgen_parameters)
$(obj)/bindings/vmscan_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/vmscan_native_generated.rs: $(addprefix $(srctree)/mm/,\
    vmscan_native_bindings.h vmscan_native_types.h vmscan_native_constants.h \
    vmscan_native_helpers.h vmscan_mglru_native.h vmscan_native_bindgen_parameters) FORCE
	$(call if_changed_dep,bindgen)
