# SPDX-License-Identifier: GPL-2.0
obj-y := core.o
ifneq ($(CONFIG_BPF_JIT_ALWAYS_ON),y)
# ___bpf_prog_run() needs GCSE disabled on x86; see 3193c0836f203 for details
cflags-nogcse-$(CONFIG_X86)$(CONFIG_CC_IS_GCC) := -fno-gcse
endif
CFLAGS_core.o += -Wno-override-init $(cflags-nogcse-yy)

# Keep private listing products cleanable after either selector is disabled.
clean-files += .rust-listing/ .rust-listing-elf/

ifneq ($(filter y,$(CONFIG_RUST_BPF_TOKEN) $(CONFIG_RUST_BPF_LPM_TRIE)),)
# Inline helpers/LTO can leave bitcode. Materialize only the private listing
# object as ELF, leaving the canonical object and objtool policy unchanged.
quiet_cmd_bpf_rust_lst_elf = LD      $@
      cmd_bpf_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<

# Track map creation/deletion as well as changes to an existing map. Preserve
# the original map argument and let makelst own its missing-map behavior.
quiet_cmd_bpf_rust_lst = MKLST   $@
      cmd_bpf_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
endif

obj-$(CONFIG_BPF_SYSCALL) += syscall.o verifier.o inode.o helpers.o tnum.o cnum.o log.o token.o liveness.o const_fold.o diagnostics.o
ifeq ($(CONFIG_RUST_BPF_TOKEN),y)
# Preserve token.o and the retained C provider; select exactly one provider.
# These bool providers are standalone built-ins. With no composite identity,
# Kbuild's basetarget keeps the same modname for normal and private targets.
$(addprefix $(obj)/,token.o token.s token.ll token.rsi \
	.rust-listing/token.o): private modname-multi :=
$(obj)/token.o: $(src)/token.rs $(objtree)/rust/bindings/bpf_token_generated.rs \
	$(wildcard $(objtree)/include/config/RUST_BPF_TOKEN) FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/token.s: $(src)/token.rs $(objtree)/rust/bindings/bpf_token_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/token.ll: $(src)/token.rs $(objtree)/rust/bindings/bpf_token_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/token.rsi: $(src)/token.rs $(objtree)/rust/bindings/bpf_token_generated.rs FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Give listings debug information without modifying the canonical .o or .cmd.
# Restore the canonical stem, per-object policy and built-in membership.
$(obj)/.rust-listing/token.o: private target-stem := token
$(obj)/.rust-listing/token.o: private part-of-builtin = $(if $(filter $(obj)/token.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/token.o: private part-of-module :=
$(obj)/.rust-listing/token.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/token.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/token.o: $(src)/token.rs $(objtree)/rust/bindings/bpf_token_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/.rust-listing-elf/token.o: $(obj)/.rust-listing/token.o FORCE
	$(call if_changed,bpf_rust_lst_elf)
$(obj)/token.lst: $(obj)/.rust-listing-elf/token.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,bpf_rust_lst)
targets += token.s token.ll token.rsi token.lst \
	.rust-listing/token.o .rust-listing-elf/token.o
obj-y += token-rust.o
endif
obj-$(CONFIG_BPF_SYSCALL) += bpf_iter.o map_iter.o task_iter.o prog_iter.o link_iter.o
obj-$(CONFIG_BPF_SYSCALL) += hashtab.o arraymap.o percpu_freelist.o bpf_lru_list.o lpm_trie.o map_in_map.o bloom_filter.o

ifeq ($(CONFIG_RUST_BPF_LPM_TRIE),y)
# Preserve the lpm_trie.o archive slot and original C source. Select exactly
# one implementation; the companion object owns macro glue/registration only.
# These bool providers are standalone built-ins. With no composite identity,
# Kbuild's basetarget keeps the same modname for normal and private targets.
$(addprefix $(obj)/,lpm_trie.o lpm_trie.s lpm_trie.ll lpm_trie.rsi \
	.rust-listing/lpm_trie.o): private modname-multi :=
$(obj)/lpm_trie.o: $(src)/lpm_trie.rs $(objtree)/rust/bindings/lpm_trie_generated.rs \
	$(wildcard $(objtree)/include/config/RUST_BPF_LPM_TRIE) FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/lpm_trie.s: $(src)/lpm_trie.rs $(objtree)/rust/bindings/lpm_trie_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/lpm_trie.ll: $(src)/lpm_trie.rs $(objtree)/rust/bindings/lpm_trie_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/lpm_trie.rsi: $(src)/lpm_trie.rs $(objtree)/rust/bindings/lpm_trie_generated.rs FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Give listings debug information without modifying the canonical .o or .cmd.
# Restore the canonical stem, per-object policy and built-in membership.
$(obj)/.rust-listing/lpm_trie.o: private target-stem := lpm_trie
$(obj)/.rust-listing/lpm_trie.o: private part-of-builtin = $(if $(filter $(obj)/lpm_trie.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/lpm_trie.o: private part-of-module :=
$(obj)/.rust-listing/lpm_trie.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/lpm_trie.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/lpm_trie.o: $(src)/lpm_trie.rs $(objtree)/rust/bindings/lpm_trie_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/.rust-listing-elf/lpm_trie.o: $(obj)/.rust-listing/lpm_trie.o FORCE
	$(call if_changed,bpf_rust_lst_elf)
$(obj)/lpm_trie.lst: $(obj)/.rust-listing-elf/lpm_trie.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,bpf_rust_lst)
targets += lpm_trie.s lpm_trie.ll lpm_trie.rsi lpm_trie.lst \
	.rust-listing/lpm_trie.o .rust-listing-elf/lpm_trie.o
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
