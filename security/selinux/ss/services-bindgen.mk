# SPDX-License-Identifier: GPL-2.0-only
# Proposed inclusion by rust/Makefile; canonical configured bindgen command.
targets += bindings/selinux_services_generated.rs
always-$(CONFIG_RUST_SELINUX_SERVICES) += bindings/selinux_services_generated.rs
$(obj)/bindings/selinux_services_generated.rs: private bindgen_target_flags = \
    --no-doc-comments --no-prepend-enum-name --no-default '.*' \
    --allowlist-type 'policydb|context|sidtab.*|selinux_.*|security_class_mapping|av_decision|avtab_.*|extended_perms.*|constraint_.*|role_.*|type_datum|user_datum|class_datum|perm_datum|cond_bool_datum|filename_trans_.*|ebitmap.*|policy_file|audit_.*|lsm_prop|netlbl_lsm_.*|qstr|super_block|gfp_t' \
    --allowlist-function 'lupos_services_.*|sidtab_.*|symtab_search|hashtab_map|mls_.*|policydb_.*|avtab_search_.*|cond_.*|evaluate_cond_nodes|string_to_.*|ebitmap_.*|audit_.*|avc_ss_reset|selnl_notify_policyload|selinux_status_update_policyload|selinux_complete_init|synchronize_rcu|kfree|vfree|vmalloc|vmalloc_user|strlen|strcmp|strncmp|strchr|sprintf|memcpy|memset|match_wildcard|netlbl_.*' \
    --allowlist-var 'LUPOS_SERVICES_.*|SEC.*|SEL_VEC_MAX|SYM_.*|CEXPR_.*|AVTAB_.*|AVC_.*|AVD_.*|XPERMS_.*|DEFAULT_.*|OBJECT_R_VAL|OCON_.*|POLICYDB_CAP_.*|POLICYDB_BOUNDS_MAXDEPTH|AUDIT_.*|Audit_.*|AF_INET.*|IB_DEVICE_NAME_MAX|NETLBL_.*|EINVAL|ENOMEM|EPERM|EACCES|ENOENT|ESTALE|EFAULT|EOPNOTSUPP|EIDRM' \
    --blocklist-type '__kernel_size_t|__kernel_ssize_t|__kernel_ptrdiff_t' \
    --wrap-unsafe-ops
$(obj)/bindings/selinux_services_generated.rs: private bindgen_target_cflags = \
    -I$(srctree)/security/selinux -I$(srctree)/security/selinux/include \
    -I$(objtree)/security/selinux -I$(srctree)/security/selinux/ss \
    $(if $(CONFIG_SECURITY_SELINUX_DEBUG),-DDEBUG)
quiet_cmd_selinux_services_bindgen = BINDGEN $@
      cmd_selinux_services_bindgen = ($(cmd_bindgen)) || exit $$?
$(obj)/bindings/selinux_services_generated.rs: $(srctree)/security/selinux/ss/services-rust.h \
    $(srctree)/security/selinux/ss/services-policy-glue.h \
    $(srctree)/security/selinux/ss/services-query-glue.h \
    $(srctree)/security/selinux/ss/services-bindgen.mk $(objtree)/security/selinux/flask.h FORCE
	$(call if_changed_dep,selinux_services_bindgen)

# rust/ is prepared before the SELinux composite is built. Reuse the original
# directory's genheaders recipe instead of guessing/copying generated values.
ifeq ($(CONFIG_RUST_SELINUX_SERVICES),y)
$(objtree)/security/selinux/flask.h: FORCE
	+$(Q)$(MAKE) $(build)=security/selinux security/selinux/flask.h
endif
