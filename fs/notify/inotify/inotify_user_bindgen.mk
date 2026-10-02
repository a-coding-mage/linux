# SPDX-License-Identifier: GPL-2.0-or-later
# Included by rust/Makefile for the canonical configured target bindgen command.
targets += bindings/inotify_user_generated.rs
always-$(CONFIG_RUST_INOTIFY_USER) += bindings/inotify_user_generated.rs
$(obj)/bindings/inotify_user_generated.rs: private bindgen_target_flags = \
    $(shell grep -Ev '^\#|^$$' $(srctree)/fs/notify/inotify/inotify_user_bindgen_parameters)
$(obj)/bindings/inotify_user_generated.rs: $(srctree)/fs/notify/inotify/inotify_user_bindings.h \
    $(srctree)/fs/notify/inotify/inotify_user_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
