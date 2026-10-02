# SPDX-License-Identifier: GPL-2.0-or-later
# Included by rust/Makefile; use the configured canonical bindgen command.
targets += bindings/ip6_gre_generated.rs
always-$(CONFIG_RUST_IPV6_GRE) += bindings/ip6_gre_generated.rs
$(obj)/bindings/ip6_gre_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/net/ipv6/ip6_gre_bindgen_parameters)
$(obj)/bindings/ip6_gre_generated.rs: $(srctree)/net/ipv6/ip6_gre_bindings.h \
    $(srctree)/net/ipv6/ip6_gre_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
