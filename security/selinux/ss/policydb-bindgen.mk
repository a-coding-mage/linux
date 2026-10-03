# SPDX-License-Identifier: GPL-2.0-only
# Additive inclusion from rust/Makefile. Uses configured canonical bindgen.
targets += bindings/selinux_policydb_generated.rs
always-$(CONFIG_RUST_SELINUX_POLICYDB) += bindings/selinux_policydb_generated.rs
$(obj)/bindings/selinux_policydb_generated.rs: private bindgen_target_flags = \
    --no-doc-comments --no-prepend-enum-name --no-default '.*' \
    --allowlist-type 'policydb|policy_data|policy_file|context|sidtab.*|avtab.*|hashtab.*|symtab|.*_datum|constraint_.*|type_set|role_.*|filename_trans_.*|range_trans|mls_.*|ebitmap.*|ocontext|genfs|gfp_t' \
    --allowlist-function 'lupos_policydb_.*|sidtab_.*|symtab_.*|hashtab_init|hashtab_destroy|hashtab_map|mls_.*|avtab_.*|cond_.*|ebitmap_.*|kfree|kvfree|strlen|strcmp|strncmp|memcpy|memset|full_name_hash|security_get_initial_sid_context' \
    --allowlist-var 'LUPOS_POLICYDB_.*|SEC.*|SEL_VEC_MAX|SYM_.*|CEXPR_.*|DEFAULT_.*|OBJECT_R.*|OCON_.*|POLICYDB_.*|TYPEDATUM_.*|REJECT_UNKNOWN|ALLOW_UNKNOWN|EINVAL|ENOMEM|ENOENT|EEXIST|IB_DEVICE_NAME_MAX' \
    --blocklist-type '__kernel_size_t|__kernel_ssize_t|__kernel_ptrdiff_t' \
    --wrap-unsafe-ops
$(obj)/bindings/selinux_policydb_generated.rs: private bindgen_target_cflags = \
    -I$(srctree)/security/selinux -I$(srctree)/security/selinux/include \
    -I$(objtree)/security/selinux -I$(srctree)/security/selinux/ss \
    $(if $(CONFIG_SECURITY_SELINUX_DEBUG),-DDEBUG)
quiet_cmd_selinux_policydb_bindgen = BINDGEN $@
      cmd_selinux_policydb_bindgen = ($(cmd_bindgen)) || exit $$?
$(obj)/bindings/selinux_policydb_generated.rs: $(srctree)/security/selinux/ss/policydb-rust.h \
    $(srctree)/security/selinux/ss/policydb-bindgen.mk $(objtree)/security/selinux/flask.h FORCE
	$(call if_changed_dep,selinux_policydb_bindgen)

# services-bindgen.mk already owns this generated target if both are enabled.
ifeq ($(CONFIG_RUST_SELINUX_POLICYDB),y)
ifneq ($(CONFIG_RUST_SELINUX_SERVICES),y)
$(objtree)/security/selinux/flask.h: FORCE
	+$(Q)$(MAKE) $(build)=security/selinux security/selinux/flask.h
endif
endif
