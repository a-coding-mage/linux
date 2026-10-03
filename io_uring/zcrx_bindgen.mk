# SPDX-License-Identifier: GPL-2.0
# Include separately from rust/Makefile. Use the canonical configured rule.
targets += bindings/io_uring_zcrx_generated.rs
always-$(CONFIG_RUST_IO_URING_ZCRX) += bindings/io_uring_zcrx_generated.rs
$(obj)/bindings/io_uring_zcrx_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/io_uring/zcrx_bindgen_parameters)
# Preserve failure before if_changed_dep's fixdep/command-recording tail.
quiet_cmd_io_uring_zcrx_bindgen = BINDGEN $@
      cmd_io_uring_zcrx_bindgen = ($(cmd_bindgen)) || exit $$?
$(obj)/bindings/io_uring_zcrx_generated.rs: \
    $(srctree)/io_uring/zcrx_bindings.h \
    $(srctree)/io_uring/zcrx_bindgen_parameters \
    $(srctree)/io_uring/zcrx_bindgen.mk FORCE
	$(call if_changed_dep,io_uring_zcrx_bindgen)
