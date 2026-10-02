# SPDX-License-Identifier: GPL-2.0-only
# Included by rust/Makefile; retain the configured canonical bindgen command.
targets += bindings/ip_tunnel_generated.rs
always-$(CONFIG_RUST_NET_IP_TUNNEL) += bindings/ip_tunnel_generated.rs
$(obj)/bindings/ip_tunnel_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/net/ipv4/ip_tunnel_bindgen_parameters)
$(obj)/bindings/ip_tunnel_generated.rs: $(srctree)/net/ipv4/ip_tunnel_bindings.h \
    $(srctree)/net/ipv4/ip_tunnel_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
