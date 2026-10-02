# SPDX-License-Identifier: GPL-2.0
# Included by rust/Makefile to reuse the canonical target bindgen command.
targets += bindings/ipc_msg_generated.rs
always-$(CONFIG_RUST_SYSVIPC_MSG) += bindings/ipc_msg_generated.rs
$(obj)/bindings/ipc_msg_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/ipc/msg_bindgen_parameters)
$(obj)/bindings/ipc_msg_generated.rs: $(srctree)/ipc/msg_bindings.h \
    $(srctree)/ipc/msg_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
