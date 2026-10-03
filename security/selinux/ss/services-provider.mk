# SPDX-License-Identifier: GPL-2.0-only
# Proposed inclusion at the end of security/selinux/Makefile. Source-only.
# Retain ss/services.o in the original selinux-y slot; never link services.c
# alongside the Rust implementation when this explicit selector is enabled.
ifeq ($(CONFIG_RUST_SELINUX_SERVICES),y)
selinux-y += ss/services-glue.o ss/services-policy-glue.o ss/services-query-glue.o
selinux-services-rust-deps := $(src)/ss/services-access.rs $(src)/ss/services-context.rs \
    $(src)/ss/services-policy.rs $(src)/ss/services-query.rs \
    $(objtree)/rust/bindings/selinux_services_generated.rs
# Explicit targets have no implicit-rule stem; retain the original composite
# identity by matching their full relative stem against selinux-y's ss/ paths.
$(addprefix $(obj)/ss/,services.o services.s services.ll services.rsi): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
# if_changed_dep appends fixdep; preserve expansion/formatter failures.
quiet_cmd_selinux_services_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_selinux_services_rsi = \
    $(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
    command -v $(RUSTFMT) >/dev/null || exit $$?; \
    $(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/ss/services.o: $(src)/ss/services.rs $(selinux-services-rust-deps) \
    $(wildcard $(objtree)/include/config/RUST_SELINUX_SERVICES) FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/ss/services.s: $(src)/ss/services.rs $(selinux-services-rust-deps) FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/ss/services.ll: $(src)/ss/services.rs $(selinux-services-rust-deps) FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/ss/services.rsi: $(src)/ss/services.rs $(selinux-services-rust-deps) FORCE
	+$(call if_changed_dep,selinux_services_rsi)

# Preserve the canonical member's flags/identity in an unlinked debug object.
$(obj)/.rust-listing/ss/services.o: private target-stem := ss/services
$(obj)/.rust-listing/ss/services.o: private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(obj)/.rust-listing/ss/services.o: private part-of-builtin = $(if $(filter $(obj)/ss/services.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/ss/services.o: private part-of-module :=
$(obj)/.rust-listing/ss/services.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/ss/services.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/ss/services.o: $(src)/ss/services.rs $(selinux-services-rust-deps) \
    $(obj)/flask.h $(wildcard $(objtree)/include/config/RUST_SELINUX_SERVICES) FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/.rust-listing-elf/ss/services.o: $(obj)/.rust-listing/ss/services.o FORCE
	$(call if_changed,selinux_rust_lst_elf)
$(obj)/ss/services.lst: $(obj)/.rust-listing-elf/ss/services.o $(srctree)/scripts/makelst \
    $(wildcard System.map) FORCE
	$(call if_changed,selinux_rust_lst)
targets += ss/services.s ss/services.ll ss/services.rsi ss/services.lst \
    .rust-listing/ss/services.o .rust-listing-elf/ss/services.o

# Composite members are appended after the original Makefile's generated-header
# prerequisite expansion, so give all three adapters the prerequisite explicitly.
$(addprefix $(obj)/ss/,services-glue.o services-policy-glue.o services-query-glue.o): $(obj)/flask.h
endif
