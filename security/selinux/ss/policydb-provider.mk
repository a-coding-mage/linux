# SPDX-License-Identifier: GPL-2.0-only
# Additive inclusion at END of security/selinux/Makefile.
ifeq ($(CONFIG_RUST_SELINUX_POLICYDB),y)
# Keep original ss/policydb.o composite position and override only its producer.
selinux-y += ss/policydb-glue.o
selinux-policydb-rust-deps := $(src)/ss/policydb-core.rs \
    $(src)/ss/policydb-symbols.rs $(src)/ss/policydb-wire.rs \
    $(objtree)/rust/bindings/selinux_policydb_generated.rs
# Explicit targets have no implicit-rule stem; retain the original composite
# identity by matching their full relative stem against selinux-y's ss/ paths.
$(addprefix $(obj)/ss/,policydb.o policydb.s policydb.ll policydb.rsi): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
quiet_cmd_selinux_policydb_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_selinux_policydb_rsi = \
    $(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
    command -v $(RUSTFMT) >/dev/null || exit $$?; \
    $(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/ss/policydb.o: $(src)/ss/policydb.rs $(selinux-policydb-rust-deps) \
    $(wildcard $(objtree)/include/config/RUST_SELINUX_POLICYDB) FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/ss/policydb.s: $(src)/ss/policydb.rs $(selinux-policydb-rust-deps) FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/ss/policydb.ll: $(src)/ss/policydb.rs $(selinux-policydb-rust-deps) FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/ss/policydb.rsi: $(src)/ss/policydb.rs $(selinux-policydb-rust-deps) FORCE
	+$(call if_changed_dep,selinux_policydb_rsi)
$(obj)/ss/policydb-glue.o: $(obj)/flask.h
endif
