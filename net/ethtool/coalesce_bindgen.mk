# SPDX-License-Identifier: GPL-2.0-only
# NEW RECONSTRUCTION: proposed CONFIG_RUST_ETHTOOL_COALESCE integration.
targets += bindings/ethtool_coalesce_generated.rs
always-$(CONFIG_RUST_ETHTOOL_COALESCE) += bindings/ethtool_coalesce_generated.rs
$(obj)/bindings/ethtool_coalesce_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/net/ethtool/coalesce_bindgen_parameters)
$(obj)/bindings/ethtool_coalesce_generated.rs: $(srctree)/net/ethtool/coalesce_bindings.h \
    $(srctree)/net/ethtool/coalesce_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
