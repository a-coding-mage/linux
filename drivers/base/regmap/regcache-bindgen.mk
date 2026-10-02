# SPDX-License-Identifier: GPL-2.0
# Included by rust/Makefile; use its canonical architecture/compiler flags.
regcache-bindgen := $(srctree)/drivers/base/regmap

targets += bindings/regcache_generated.rs
always-$(CONFIG_RUST_REGCACHE) += bindings/regcache_generated.rs

$(obj)/bindings/regcache_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(regcache-bindgen)/regcache-bindgen-parameters)
$(obj)/bindings/regcache_generated.rs: $(src)/bindings/regcache.h \
    $(regcache-bindgen)/regcache-bindgen-parameters FORCE
	$(call if_changed_dep,bindgen)
