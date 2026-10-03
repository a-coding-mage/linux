# SPDX-License-Identifier: GPL-2.0
#
# Makefile for the linux kernel.
#

ccflags-y := -fno-function-sections -fno-data-sections

obj-y                          := main.o version.o mounts.o

# Keep selected Rust source/assembly listings with their native boot owners.
# The generic .lst rule instead compiles the adjacent C source into $*.o.
clean-files += .rust-listing/ .rust-listing-elf/
ifneq ($(filter y,$(CONFIG_RUST_INIT_MAIN) $(CONFIG_RUST_INIT_NOINITRAMFS) \
	$(CONFIG_RUST_INIT_MOUNTS_INITRD) $(CONFIG_RUST_INIT_VERSION) \
	$(CONFIG_RUST_INIT_CALIBRATE) $(CONFIG_RUST_INIT_ROOT_MOUNTS) \
	$(CONFIG_RUST_INIT_TASK) $(CONFIG_RUST_INIT_INITRAMFS) $(CONFIG_RUST_INIT_RAMDISK)),)
quiet_cmd_init_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_init_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
quiet_cmd_init_rust_lst_elf = LD      $@
      cmd_init_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
quiet_cmd_init_rust_lst = MKLST   $@
      cmd_init_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@

# $1 is the original object stem; $2 is its existing native binding dependency.
# Explicit rules have no relative implicit-rule stem. Match target-stem against
# the original composite members so do_mounts_initrd keeps the mounts identity.
define init-rust-inspection
$(addprefix $(obj)/,$1.o $1.s $1.ll $1.rsi .rust-listing/$1.o): private modname-multi = \
	$$(sort $$(foreach m,$$(multi-obj-ym),\
		$$(if $$(filter $$(target-stem).o,$$(call suffix-search,$$m,.o,-objs -y -m)),$$(m:.o=))))
$(obj)/$1.rsi: $(src)/$1.rs $2 FORCE
	+$$(call if_changed_dep,init_rust_rsi)
$(obj)/.rust-listing/$1.o: private target-stem := $1
$(obj)/.rust-listing/$1.o: private part-of-builtin = $$(if $$(filter $(obj)/$1.o,$$(real-obj-y) $$(lib-y)),y)
$(obj)/.rust-listing/$1.o: private part-of-module :=
$(obj)/.rust-listing/$1.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/$1.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/$1.o: $(src)/$1.rs $2 FORCE
	+$$(call if_changed_rule,rustc_o_rs)
$(obj)/.rust-listing-elf/$1.o: $(obj)/.rust-listing/$1.o FORCE
	$$(call if_changed,init_rust_lst_elf)
$(obj)/$1.lst: $(obj)/.rust-listing-elf/$1.o $(srctree)/scripts/makelst \
	$$(wildcard System.map) FORCE
	$$(call if_changed,init_rust_lst)
targets += $1.s $1.ll $1.rsi $1.lst .rust-listing/$1.o .rust-listing-elf/$1.o
endef
endif

ifeq ($(CONFIG_RUST_INIT_MAIN),y)
# Preserve the original main.o archive position and build identity. An explicit
# rule is required because the original main.c remains beside main.rs.
RUST_ALLOWED_FEATURES_main.o := linkage,no_sanitize
RUSTFLAGS_main.o += --extern init_main_bindings -Zfunction-sections=n
$(eval $(call init-rust-inspection,main,$(objtree)/rust/libinit_main_bindings.rmeta))

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
$(eval $(call init-rust-inspection,noinitramfs,$(objtree)/rust/bindings/init_noinitramfs_generated.rs))
$(obj)/noinitramfs.o: $(src)/noinitramfs.rs $(objtree)/rust/bindings/init_noinitramfs_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/noinitramfs.s: $(src)/noinitramfs.rs $(objtree)/rust/bindings/init_noinitramfs_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/noinitramfs.ll: $(src)/noinitramfs.rs $(objtree)/rust/bindings/init_noinitramfs_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
endif
else
obj-$(CONFIG_BLK_DEV_INITRD)   += initramfs.o

ifeq ($(CONFIG_RUST_INIT_INITRAMFS),y)
RUST_ALLOWED_FEATURES_initramfs.o := linkage
RUSTFLAGS_initramfs.o += -Zfunction-sections=n
$(eval $(call init-rust-inspection,initramfs,$(objtree)/rust/bindings/init_initramfs_generated.rs))
$(obj)/initramfs.o: $(src)/initramfs.rs $(objtree)/rust/bindings/init_initramfs_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/initramfs.s: $(src)/initramfs.rs $(objtree)/rust/bindings/init_initramfs_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/initramfs.ll: $(src)/initramfs.rs $(objtree)/rust/bindings/init_initramfs_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
obj-y += initramfs_abi.o
endif
endif
obj-$(CONFIG_GENERIC_CALIBRATE_DELAY) += calibrate.o

ifeq ($(CONFIG_RUST_INIT_CALIBRATE),y)
RUST_ALLOWED_FEATURES_calibrate.o := linkage
RUSTFLAGS_calibrate.o += -Zfunction-sections=n
$(eval $(call init-rust-inspection,calibrate,$(objtree)/rust/bindings/init_calibrate_generated.rs))
$(obj)/calibrate.o: $(src)/calibrate.rs $(objtree)/rust/bindings/init_calibrate_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/calibrate.s: $(src)/calibrate.rs $(objtree)/rust/bindings/init_calibrate_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/calibrate.ll: $(src)/calibrate.rs $(objtree)/rust/bindings/init_calibrate_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
obj-y += calibrate_percpu.o
endif
obj-$(CONFIG_INITRAMFS_TEST)   += initramfs_test.o

obj-y                          += init_task.o

ifeq ($(CONFIG_RUST_INIT_TASK),y)
RUSTFLAGS_init_task.o += -Zfunction-sections=n
$(eval $(call init-rust-inspection,init_task,$(objtree)/rust/bindings/init_task_generated.rs))
$(obj)/init_task.o: $(src)/init_task.rs $(src)/init_task_initializers.rs \
    $(objtree)/rust/bindings/init_task_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/init_task.s: $(src)/init_task.rs $(src)/init_task_initializers.rs \
    $(objtree)/rust/bindings/init_task_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/init_task.ll: $(src)/init_task.rs $(src)/init_task_initializers.rs \
    $(objtree)/rust/bindings/init_task_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
obj-$(CONFIG_MODVERSIONS) += init_task_exports.o
endif

mounts-y			:= do_mounts.o

ifeq ($(CONFIG_RUST_INIT_ROOT_MOUNTS),y)
mounts-y += root_mounts_alloc.o
RUSTFLAGS_do_mounts.o += -Zfunction-sections=n
$(eval $(call init-rust-inspection,do_mounts,$(objtree)/rust/bindings/init_root_mounts_generated.rs))
$(obj)/do_mounts.o: $(src)/do_mounts.rs $(objtree)/rust/bindings/init_root_mounts_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/do_mounts.s: $(src)/do_mounts.rs $(objtree)/rust/bindings/init_root_mounts_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/do_mounts.ll: $(src)/do_mounts.rs $(objtree)/rust/bindings/init_root_mounts_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
endif
mounts-$(CONFIG_BLK_DEV_RAM)	+= do_mounts_rd.o
mounts-$(CONFIG_BLK_DEV_INITRD)	+= do_mounts_initrd.o

ifeq ($(CONFIG_RUST_INIT_RAMDISK),y)
mounts-y += ramdisk_loader_abi.o
RUSTFLAGS_do_mounts_rd.o += -Zfunction-sections=n
$(eval $(call init-rust-inspection,do_mounts_rd,$(objtree)/rust/bindings/init_ramdisk_generated.rs))
$(obj)/do_mounts_rd.o: $(src)/do_mounts_rd.rs $(objtree)/rust/bindings/init_ramdisk_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/do_mounts_rd.s: $(src)/do_mounts_rd.rs $(objtree)/rust/bindings/init_ramdisk_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/do_mounts_rd.ll: $(src)/do_mounts_rd.rs $(objtree)/rust/bindings/init_ramdisk_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
endif

ifeq ($(CONFIG_RUST_INIT_MOUNTS_INITRD),y)
# Keep the original component name and position within mounts.o. The explicit
# rules select Rust while preserving the original C source as the default.
RUSTFLAGS_do_mounts_initrd.o += -Zfunction-sections=n
$(eval $(call init-rust-inspection,do_mounts_initrd,$(objtree)/rust/bindings/init_mounts_generated.rs))
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
$(eval $(call init-rust-inspection,version,$(objtree)/rust/bindings/init_version_generated.rs))
$(eval $(call init-rust-inspection,version-timestamp,$(objtree)/rust/bindings/init_version_timestamp_generated.rs))

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
