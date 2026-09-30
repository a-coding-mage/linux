# SPDX-License-Identifier: GPL-2.0
#
# Makefile for the linux kernel.
#

ccflags-y := -fno-function-sections -fno-data-sections

obj-y                          := main.o version.o mounts.o

ifeq ($(CONFIG_RUST_INIT_MAIN),y)
# Preserve the original main.o archive position and build identity. An explicit
# rule is required because the original main.c remains beside main.rs.
RUST_ALLOWED_FEATURES_main.o := linkage,no_sanitize
RUSTFLAGS_main.o += --extern init_main_bindings -Zfunction-sections=n

$(obj)/main.o: $(src)/main.rs $(objtree)/rust/libinit_main_bindings.rmeta FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/main.s: $(src)/main.rs $(objtree)/rust/libinit_main_bindings.rmeta FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/main.ll: $(src)/main.rs $(objtree)/rust/libinit_main_bindings.rmeta FORCE
	+$(call if_changed_dep,rustc_ll_rs)

# Canonical compiler/shared-infrastructure boundaries; init sequencing and
# trace registration/callbacks are owned by main.rs.
obj-y += main_alloc.o main_tracepoints.o
endif

ifneq ($(CONFIG_BLK_DEV_INITRD),y)
obj-y                          += noinitramfs.o
ifeq ($(CONFIG_RUST_INIT_NOINITRAMFS),y)
# Keep the original archive slot and rootfs initcall registration identity.
RUST_ALLOWED_FEATURES_noinitramfs.o := linkage
RUSTFLAGS_noinitramfs.o += -Zfunction-sections=n
$(obj)/noinitramfs.o: $(src)/noinitramfs.rs $(objtree)/rust/bindings/init_noinitramfs_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/noinitramfs.s: $(src)/noinitramfs.rs $(objtree)/rust/bindings/init_noinitramfs_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/noinitramfs.ll: $(src)/noinitramfs.rs $(objtree)/rust/bindings/init_noinitramfs_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
endif
else
obj-$(CONFIG_BLK_DEV_INITRD)   += initramfs.o
endif
obj-$(CONFIG_GENERIC_CALIBRATE_DELAY) += calibrate.o
obj-$(CONFIG_INITRAMFS_TEST)   += initramfs_test.o

obj-y                          += init_task.o

mounts-y			:= do_mounts.o
mounts-$(CONFIG_BLK_DEV_RAM)	+= do_mounts_rd.o
mounts-$(CONFIG_BLK_DEV_INITRD)	+= do_mounts_initrd.o

ifeq ($(CONFIG_RUST_INIT_MOUNTS_INITRD),y)
# Keep the original component name and position within mounts.o. The explicit
# rules select Rust while preserving the original C source as the default.
RUSTFLAGS_do_mounts_initrd.o += -Zfunction-sections=n
$(obj)/do_mounts_initrd.o: $(src)/do_mounts_initrd.rs $(objtree)/rust/bindings/init_mounts_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/do_mounts_initrd.s: $(src)/do_mounts_initrd.rs $(objtree)/rust/bindings/init_mounts_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/do_mounts_initrd.ll: $(src)/do_mounts_initrd.rs $(objtree)/rust/bindings/init_mounts_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
endif

#
# UTS_VERSION
#

smp-flag-$(CONFIG_SMP)			:= SMP
preempt-flag-$(CONFIG_PREEMPT_BUILD)	:= PREEMPT
preempt-flag-$(CONFIG_PREEMPT_DYNAMIC)	:= PREEMPT_DYNAMIC
preempt-flag-$(CONFIG_PREEMPT_RT)	:= PREEMPT_RT

build-version = $(or $(KBUILD_BUILD_VERSION), $(build-version-auto))
build-timestamp = $(or $(KBUILD_BUILD_TIMESTAMP), $(build-timestamp-auto))

# Maximum length of UTS_VERSION is 64 chars
filechk_uts_version = \
	utsver=$$(echo '$(pound)'"$(build-version)" $(smp-flag-y) $(preempt-flag-y) "$(build-timestamp)" | cut -b -64); \
	echo '$(pound)'define UTS_VERSION \""$${utsver}"\"

#
# Build version.c with temporary UTS_VERSION
#

$(obj)/utsversion-tmp.h: FORCE
	$(call filechk,uts_version)

clean-files += utsversion-tmp.h

ifneq ($(CONFIG_RUST_INIT_VERSION),y)
$(obj)/version.o: $(obj)/utsversion-tmp.h
endif
CFLAGS_version.o := -include $(obj)/utsversion-tmp.h

#
# Build version-timestamp.c with final UTS_VERSION
#

include/generated/utsversion.h: build-version-auto = $(shell $(srctree)/scripts/build-version)
include/generated/utsversion.h: build-timestamp-auto = $(shell LC_ALL=C date)
include/generated/utsversion.h: FORCE
	$(call filechk,uts_version)

ifneq ($(CONFIG_RUST_INIT_VERSION),y)
$(obj)/version-timestamp.o: include/generated/utsversion.h
endif
CFLAGS_version-timestamp.o := -include include/generated/utsversion.h

ifeq ($(CONFIG_RUST_INIT_VERSION),y)
# The final timestamp must stay outside normal preparation and archive builds.
# Generate each owner's bindings only after its own UTS_VERSION header exists;
# the rust/ descent reuses the canonical compiler flags and bindgen dependency
# tracking without adding either stage to the prepare targets.
$(objtree)/rust/bindings/init_version_generated.rs: $(obj)/utsversion-tmp.h FORCE
	+$(Q)$(MAKE) $(build)=rust rust/bindings/init_version_generated.rs
$(objtree)/rust/bindings/init_version_timestamp_generated.rs: include/generated/utsversion.h FORCE
	+$(Q)$(MAKE) $(build)=rust rust/bindings/init_version_timestamp_generated.rs

RUST_ALLOWED_FEATURES_version.o := linkage
RUSTFLAGS_version.o += -Zfunction-sections=n
RUSTFLAGS_version-timestamp.o += -Zfunction-sections=n

$(obj)/version.o: $(src)/version.rs $(objtree)/rust/bindings/init_version_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/version.s: $(src)/version.rs $(objtree)/rust/bindings/init_version_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/version.ll: $(src)/version.rs $(objtree)/rust/bindings/init_version_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)

$(obj)/version-timestamp.o: $(src)/version-timestamp.rs $(objtree)/rust/bindings/init_version_timestamp_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/version-timestamp.s: $(src)/version-timestamp.rs $(objtree)/rust/bindings/init_version_timestamp_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/version-timestamp.ll: $(src)/version-timestamp.rs $(objtree)/rust/bindings/init_version_timestamp_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
endif
