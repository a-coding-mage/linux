# SPDX-License-Identifier: GPL-2.0-or-later
# Included by rust/Makefile; use the configured, canonical bindgen command.
targets += bindings/ah6_generated.rs
always-$(CONFIG_RUST_INET6_AH) += bindings/ah6_generated.rs
$(obj)/bindings/ah6_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/net/ipv6/ah6_bindgen_parameters)
$(obj)/bindings/ah6_generated.rs: $(srctree)/net/ipv6/ah6_bindings.h \
    $(srctree)/net/ipv6/ah6_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
