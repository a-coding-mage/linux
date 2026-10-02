# SPDX-License-Identifier: GPL-2.0-only
obj-$(CONFIG_INOTIFY_USER)	+= inotify_fsnotify.o inotify_user.o

ifeq ($(CONFIG_RUST_INOTIFY_USER),y)
obj-y += inotify_user_helpers.o
RUSTFLAGS_inotify_user.o += -Zfunction-sections=n
$(obj)/inotify_user.o: $(src)/inotify_user.rs $(objtree)/rust/bindings/inotify_user_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif
