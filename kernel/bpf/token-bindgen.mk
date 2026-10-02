# SPDX-License-Identifier: GPL-2.0
# Use rust/Makefile's canonical target, configured C and bindgen flags.
targets += bindings/bpf_token_generated.rs
always-$(CONFIG_RUST_BPF_TOKEN) += bindings/bpf_token_generated.rs
$(obj)/bindings/bpf_token_generated.rs: private bindgen_target_flags = \
    --allowlist-type 'bpf_token|bpf_token_info|bpf_mount_opts|bpf_attr|bpf_cmd|bpf_map_type|bpf_prog_type|bpf_attach_type|fd|fd_prepare' \
    --allowlist-function 'lupos_token_.*|fdget|bpf_get_inode|clear_nlink|seq_printf|kfree' \
    --allowlist-var 'LUPOS_TOKEN_.*|lupos_bpf_token_iops|bpf_token_fops|bpf_super_ops|init_user_ns|EBADF|EINVAL|EPERM|EOPNOTSUPP|ENOENT|ENOMEM|EFAULT' \
    --blocklist-type '__kernel_size_t|__kernel_ssize_t|__kernel_ptrdiff_t' \
    --wrap-unsafe-ops
$(obj)/bindings/bpf_token_generated.rs: $(srctree)/kernel/bpf/token-rust.h \
    $(srctree)/kernel/bpf/token-bindgen.mk FORCE
	$(call if_changed_dep,bindgen)
