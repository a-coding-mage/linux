# SPDX-License-Identifier: GPL-2.0-or-later
# Included by rust/Makefile for the configured target bindgen command.
targets += bindings/ipv6_sockglue_generated.rs
always-$(CONFIG_RUST_IPV6_SOCKGLUE) += bindings/ipv6_sockglue_generated.rs
$(obj)/bindings/ipv6_sockglue_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/net/ipv6/ipv6_sockglue_bindgen_parameters)
$(obj)/bindings/ipv6_sockglue_generated.rs: $(srctree)/net/ipv6/ipv6_sockglue_bindings.h \
    $(srctree)/net/ipv6/ipv6_sockglue_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
