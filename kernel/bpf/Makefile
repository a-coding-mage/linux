# SPDX-License-Identifier: GPL-2.0
obj-y := core.o
ifneq ($(CONFIG_BPF_JIT_ALWAYS_ON),y)
# ___bpf_prog_run() needs GCSE disabled on x86; see 3193c0836f203 for details
cflags-nogcse-$(CONFIG_X86)$(CONFIG_CC_IS_GCC) := -fno-gcse
endif
CFLAGS_core.o += -Wno-override-init $(cflags-nogcse-yy)

obj-$(CONFIG_BPF_SYSCALL) += syscall.o verifier.o inode.o helpers.o tnum.o cnum.o log.o token.o liveness.o const_fold.o diagnostics.o
ifeq ($(CONFIG_RUST_BPF_TOKEN),y)
# Preserve token.o and the retained C provider; select exactly one provider.
$(obj)/token.o: $(src)/token.rs $(objtree)/rust/bindings/bpf_token_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/token.s: $(src)/token.rs $(objtree)/rust/bindings/bpf_token_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/token.ll: $(src)/token.rs $(objtree)/rust/bindings/bpf_token_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
obj-y += token-rust.o
endif
obj-$(CONFIG_BPF_SYSCALL) += bpf_iter.o map_iter.o task_iter.o prog_iter.o link_iter.o
obj-$(CONFIG_BPF_SYSCALL) += hashtab.o arraymap.o percpu_freelist.o bpf_lru_list.o lpm_trie.o map_in_map.o bloom_filter.o

ifeq ($(CONFIG_RUST_BPF_LPM_TRIE),y)
# Preserve the lpm_trie.o archive slot and original C source. Select exactly
# one implementation; the companion object owns macro glue/registration only.
$(obj)/lpm_trie.o: $(src)/lpm_trie.rs $(objtree)/rust/bindings/lpm_trie_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/lpm_trie.s: $(src)/lpm_trie.rs $(objtree)/rust/bindings/lpm_trie_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/lpm_trie.ll: $(src)/lpm_trie.rs $(objtree)/rust/bindings/lpm_trie_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
obj-y += lpm_trie-rust.o
CFLAGS_REMOVE_lpm_trie-rust.o = $(CC_FLAGS_FTRACE)
endif
obj-$(CONFIG_BPF_SYSCALL) += local_storage.o queue_stack_maps.o ringbuf.o bpf_insn_array.o
obj-$(CONFIG_BPF_SYSCALL) += bpf_local_storage.o bpf_task_storage.o
obj-$(CONFIG_BPF_SYSCALL) += fixups.o cfg.o states.o backtrack.o check_btf.o
obj-${CONFIG_BPF_LSM}	  += bpf_inode_storage.o
obj-$(CONFIG_BPF_SYSCALL) += disasm.o mprog.o
obj-$(CONFIG_BPF_JIT) += trampoline.o
obj-$(CONFIG_BPF_SYSCALL) += btf.o memalloc.o rqspinlock.o stream.o
ifeq ($(CONFIG_MMU)$(CONFIG_64BIT),yy)
obj-$(CONFIG_BPF_SYSCALL) += arena.o range_tree.o
endif
obj-$(CONFIG_BPF_JIT) += dispatcher.o
ifeq ($(CONFIG_NET),y)
obj-$(CONFIG_BPF_SYSCALL) += devmap.o
obj-$(CONFIG_BPF_SYSCALL) += cpumap.o
obj-$(CONFIG_BPF_SYSCALL) += offload.o
obj-$(CONFIG_BPF_SYSCALL) += net_namespace.o
obj-$(CONFIG_BPF_SYSCALL) += tcx.o
endif
ifeq ($(CONFIG_PERF_EVENTS),y)
obj-$(CONFIG_BPF_SYSCALL) += stackmap.o
endif
ifeq ($(CONFIG_CGROUPS),y)
obj-$(CONFIG_BPF_SYSCALL) += cgroup_iter.o bpf_cgrp_storage.o
endif
obj-$(CONFIG_CGROUP_BPF) += cgroup.o
ifeq ($(CONFIG_INET),y)
obj-$(CONFIG_BPF_SYSCALL) += reuseport_array.o
endif
ifeq ($(CONFIG_SYSFS),y)
obj-$(CONFIG_DEBUG_INFO_BTF) += sysfs_btf.o
endif
ifeq ($(CONFIG_BPF_JIT),y)
obj-$(CONFIG_BPF_SYSCALL) += bpf_struct_ops.o
obj-$(CONFIG_BPF_SYSCALL) += cpumask.o
# bpf_lsm_proto.o must precede bpf_lsm.o. The current pahole logic
# deduplicates function prototypes within
# btf_encoder__add_saved_func() by keeping the first instance seen. We
# need the function prototype(s) in bpf_lsm_proto.o to take precedence
# over those within bpf_lsm.o. Having bpf_lsm_proto.o precede
# bpf_lsm.o ensures its DWARF CU is processed early, forcing the
# generated BTF to contain the overrides.
#
# Notably, this is a temporary workaround whilst the deduplication
# semantics within pahole are revisited accordingly.
obj-${CONFIG_BPF_LSM} += bpf_lsm_proto.o bpf_lsm.o
endif
ifneq ($(CONFIG_CRYPTO),)
obj-$(CONFIG_BPF_SYSCALL) += crypto.o
endif
obj-$(CONFIG_BPF_PRELOAD) += preload/

obj-$(CONFIG_BPF_SYSCALL) += relo_core.o
obj-$(CONFIG_BPF_SYSCALL) += btf_iter.o
obj-$(CONFIG_BPF_SYSCALL) += btf_relocate.o
obj-$(CONFIG_BPF_SYSCALL) += kmem_cache_iter.o
ifeq ($(CONFIG_DMA_SHARED_BUFFER),y)
obj-$(CONFIG_BPF_SYSCALL) += dmabuf_iter.o
endif

CFLAGS_REMOVE_percpu_freelist.o = $(CC_FLAGS_FTRACE)
CFLAGS_REMOVE_bpf_lru_list.o = $(CC_FLAGS_FTRACE)
CFLAGS_REMOVE_queue_stack_maps.o = $(CC_FLAGS_FTRACE)
CFLAGS_REMOVE_lpm_trie.o = $(CC_FLAGS_FTRACE)
CFLAGS_REMOVE_ringbuf.o = $(CC_FLAGS_FTRACE)
CFLAGS_REMOVE_rqspinlock.o = $(CC_FLAGS_FTRACE)
