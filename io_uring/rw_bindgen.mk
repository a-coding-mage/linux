# SPDX-License-Identifier: GPL-2.0
# Included by rust/Makefile to use the canonical target-configured bindgen rule.
targets += bindings/io_uring_rw_generated.rs
always-$(CONFIG_RUST_IO_URING_RW) += bindings/io_uring_rw_generated.rs
$(obj)/bindings/io_uring_rw_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/io_uring/rw_bindgen_parameters)
$(obj)/bindings/io_uring_rw_generated.rs: $(srctree)/io_uring/rw_bindings.h \
    $(srctree)/io_uring/rw_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
