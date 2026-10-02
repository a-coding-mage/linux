# SPDX-License-Identifier: GPL-2.0-only
# Reuse rust/Makefile's canonical target, Kconfig and C bindgen flags.
targets += bindings/lpm_trie_generated.rs
always-$(CONFIG_RUST_BPF_LPM_TRIE) += bindings/lpm_trie_generated.rs
$(obj)/bindings/lpm_trie_generated.rs: private bindgen_target_flags = \
    --allowlist-type 'lpm_trie|lpm_trie_node|bpf_lpm_trie_key_u8|bpf_attr|btf|btf_type' \
    --allowlist-function 'lupos_lpm_.*|bpf_map_area_alloc|bpf_map_area_free|bpf_map_init_from_attr|bpf_mem_alloc_init|bpf_mem_alloc_destroy|bpf_mem_cache_alloc|bpf_mem_cache_free|bpf_mem_cache_free_rcu|bpf_mem_cache_raw_free|kfree' \
    --allowlist-var 'LUPOS_LPM_.*|BPF_(EXIST|NOEXIST|F_NO_PREALLOC)|BTF_KIND_STRUCT|EINVAL|ENOMEM|ENOENT|ENOSPC|EEXIST' \
    --blocklist-type '__kernel_size_t|__kernel_ssize_t|__kernel_ptrdiff_t' \
    --wrap-unsafe-ops
$(obj)/bindings/lpm_trie_generated.rs: $(srctree)/kernel/bpf/lpm_trie-rust.h \
    $(srctree)/kernel/bpf/lpm_trie-bindgen.mk FORCE
	$(call if_changed_dep,bindgen)
