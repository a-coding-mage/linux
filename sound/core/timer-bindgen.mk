# SPDX-License-Identifier: GPL-2.0
# Canonical target bindgen command supplied by rust/Makefile.
targets += bindings/snd_timer_generated.rs
always-$(CONFIG_RUST_SND_TIMER) += bindings/snd_timer_generated.rs
$(obj)/bindings/snd_timer_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/sound/core/timer-bindgen-parameters)
$(obj)/bindings/snd_timer_generated.rs: $(srctree)/sound/core/timer-bindings.h \
    $(srctree)/sound/core/timer-bindgen-parameters FORCE
	$(call if_changed_dep,bindgen)
