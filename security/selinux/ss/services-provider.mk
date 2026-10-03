# SPDX-License-Identifier: GPL-2.0-only
# Proposed inclusion at the end of security/selinux/Makefile. Source-only.
# Retain ss/services.o in the original selinux-y slot; never link services.c
# alongside the Rust implementation when this explicit selector is enabled.
ifeq ($(CONFIG_RUST_SELINUX_SERVICES),y)
selinux-y += ss/services-glue.o ss/services-policy-glue.o ss/services-query-glue.o
selinux-services-rust-deps := $(src)/ss/services-access.rs $(src)/ss/services-context.rs \
    $(src)/ss/services-policy.rs $(src)/ss/services-query.rs \
    $(objtree)/rust/bindings/selinux_services_generated.rs
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
# Composite members are appended after the original Makefile's generated-header
# prerequisite expansion, so give all three adapters the prerequisite explicitly.
$(addprefix $(obj)/ss/,services-glue.o services-policy-glue.o services-query-glue.o): $(obj)/flask.h
endif
