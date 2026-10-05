# SPDX-License-Identifier: GPL-2.0
#
# Makefile for the linux memory manager.
#

KASAN_SANITIZE_slab_common.o := n
KASAN_SANITIZE_slub.o := n
KASAN_SANITIZE_kmemleak.o := n
KCSAN_SANITIZE_kmemleak.o := n

# These produce frequent data race reports: most of them are due to races on
# the same word but accesses to different bits of that word. Re-enable KCSAN
# for these when we have more consensus on what to do about them.
KCSAN_SANITIZE_slab_common.o := n
KCSAN_SANITIZE_slub.o := n
KCSAN_SANITIZE_page_alloc.o := n
# But enable explicit instrumentation for memory barriers.
KCSAN_INSTRUMENT_BARRIERS := y

# These files are disabled because they produce non-interesting and/or
# flaky coverage that is not a function of syscall inputs. E.g. slab is out of
# free pages, or a task is migrated between nodes.
KCOV_INSTRUMENT_slab_common.o := n
KCOV_INSTRUMENT_slub.o := n
KCOV_INSTRUMENT_page_alloc.o := n
KCOV_INSTRUMENT_debug-pagealloc.o := n
KCOV_INSTRUMENT_kmemleak.o := n
KCOV_INSTRUMENT_memcontrol.o := n
KCOV_INSTRUMENT_memcontrol-v1.o := n
KCOV_INSTRUMENT_mmzone.o := n
KCOV_INSTRUMENT_vmstat.o := n
KCOV_INSTRUMENT_failslab.o := n

CFLAGS_init-mm.o += -Wno-override-init

mmu-y			:= nommu.o
mmu-$(CONFIG_MMU)	:= highmem.o memory.o mincore.o \
			   mlock.o mmap.o mmu_gather.o mprotect.o mremap.o \
			   msync.o page_vma_mapped.o pagewalk.o \
			   pgtable-generic.o rmap.o vmalloc.o vma.o vma_exec.o

include $(srctree)/mm/vmalloc-rust.mk


ifdef CONFIG_CROSS_MEMORY_ATTACH
mmu-$(CONFIG_MMU)	+= process_vm_access.o
endif

ifdef CONFIG_64BIT
mmu-$(CONFIG_MMU)	+= mseal.o
endif

obj-y			:= filemap.o mempool.o oom_kill.o fadvise.o \
			   maccess.o page-writeback.o folio-compat.o \
			   readahead.o folio.o truncate.o vmscan.o shrinker.o \
			   shmem.o util.o mmzone.o vmstat.o backing-dev.o \
			   mm_init.o percpu.o slab_common.o \
			   compaction.o show_mem.o \
			   interval_tree.o list_lru.o workingset.o \
			   debug.o gup.o mmap_lock.o vma_init.o $(mmu-y)

# Give 'page_alloc' its own module-parameter namespace
page-alloc-y := page_alloc.o
include $(srctree)/mm/page_alloc-rust.mk
page-alloc-$(CONFIG_SHUFFLE_PAGE_ALLOCATOR) += shuffle.o

# Give 'memory_hotplug' its own module-parameter namespace
memory-hotplug-$(CONFIG_MEMORY_HOTPLUG) += memory_hotplug.o

obj-y += page-alloc.o
obj-y += page_frag_cache.o
obj-y += init-mm.o
obj-y += memblock.o
include $(srctree)/mm/memblock-rust.mk
obj-y += $(memory-hotplug-y)
obj-y += slub.o
include $(srctree)/mm/slub-rust.mk

ifdef CONFIG_MMU
	obj-$(CONFIG_ADVISE_SYSCALLS)	+= madvise.o
endif

obj-$(CONFIG_SWAP)	+= page_io.o swap_state.o swapfile.o
obj-$(CONFIG_ZSWAP)	+= zswap.o
obj-$(CONFIG_HAS_DMA)	+= dmapool.o
obj-$(CONFIG_HUGETLBFS)	+= hugetlb.o hugetlb_sysfs.o hugetlb_sysctl.o
ifdef CONFIG_CMA
obj-$(CONFIG_HUGETLBFS)	+= hugetlb_cma.o
endif
obj-$(CONFIG_HUGETLB_PAGE_OPTIMIZE_VMEMMAP)	+= hugetlb_vmemmap.o
obj-$(CONFIG_NUMA) 	+= mempolicy.o
obj-$(CONFIG_SPARSEMEM)	+= sparse.o
obj-$(CONFIG_SPARSEMEM_VMEMMAP) += sparse-vmemmap.o
obj-$(CONFIG_MMU_NOTIFIER) += mmu_notifier.o
obj-$(CONFIG_KSM) += ksm.o
obj-$(CONFIG_PAGE_POISONING) += page_poison.o
obj-$(CONFIG_KASAN)	+= kasan/
obj-$(CONFIG_KFENCE) += kfence/
obj-$(CONFIG_KMSAN)	+= kmsan/
obj-$(CONFIG_FAILSLAB) += failslab.o
obj-$(CONFIG_FAIL_PAGE_ALLOC) += fail_page_alloc.o
obj-$(CONFIG_MEMTEST)		+= memtest.o
obj-$(CONFIG_MIGRATION) += migrate.o
obj-$(CONFIG_NUMA) += memory-tiers.o
obj-$(CONFIG_DEVICE_MIGRATION) += migrate_device.o
obj-$(CONFIG_TRANSPARENT_HUGEPAGE) += huge_memory.o khugepaged.o
obj-$(CONFIG_PAGE_COUNTER) += page_counter.o
obj-$(CONFIG_LIVEUPDATE_MEMFD) += memfd_luo.o
obj-$(CONFIG_MEMCG_V1) += memcontrol-v1.o
obj-$(CONFIG_MEMCG) += memcontrol.o vmpressure.o
ifdef CONFIG_BPF_SYSCALL
obj-$(CONFIG_MEMCG) += bpf_memcontrol.o
endif
obj-$(CONFIG_CGROUP_HUGETLB) += hugetlb_cgroup.o
obj-$(CONFIG_GUP_TEST) += gup_test.o
obj-$(CONFIG_DMAPOOL_TEST) += dmapool_test.o
obj-$(CONFIG_MEMORY_FAILURE) += memory-failure.o
obj-$(CONFIG_HWPOISON_INJECT) += hwpoison-inject.o
obj-$(CONFIG_DEBUG_KMEMLEAK) += kmemleak.o
obj-$(CONFIG_DEBUG_RODATA_TEST) += rodata_test.o
obj-$(CONFIG_DEBUG_VM_PGTABLE) += debug_vm_pgtable.o
obj-$(CONFIG_PAGE_OWNER) += page_owner.o
obj-$(CONFIG_MEMORY_ISOLATION) += page_isolation.o
obj-$(CONFIG_ZSMALLOC)	+= zsmalloc.o
obj-$(CONFIG_GENERIC_EARLY_IOREMAP) += early_ioremap.o
obj-$(CONFIG_CMA)	+= cma.o
obj-$(CONFIG_NUMA) += numa.o
obj-$(CONFIG_NUMA_MEMBLKS) += numa_memblks.o
obj-$(CONFIG_NUMA_EMU) += numa_emulation.o
obj-$(CONFIG_GENERIC_ARCH_NUMA) += arch_numa.o
obj-$(CONFIG_BALLOON) += balloon.o
obj-$(CONFIG_PAGE_EXTENSION) += page_ext.o
obj-$(CONFIG_PAGE_TABLE_CHECK) += page_table_check.o
obj-$(CONFIG_CMA_DEBUGFS) += cma_debug.o
obj-$(CONFIG_SECRETMEM) += secretmem.o
obj-$(CONFIG_CMA_SYSFS) += cma_sysfs.o
obj-$(CONFIG_USERFAULTFD) += userfaultfd.o
obj-$(CONFIG_IDLE_PAGE_TRACKING) += page_idle.o
obj-$(CONFIG_DEBUG_PAGEALLOC) += debug_page_alloc.o
obj-$(CONFIG_DEBUG_PAGE_REF) += debug_page_ref.o
obj-$(CONFIG_DAMON) += damon/
obj-$(CONFIG_HARDENED_USERCOPY) += usercopy.o
obj-$(CONFIG_PERCPU_STATS) += percpu-stats.o
obj-$(CONFIG_ZONE_DEVICE) += memremap.o
obj-$(CONFIG_HMM_MIRROR) += hmm.o
obj-$(CONFIG_MEMFD_CREATE) += memfd.o
obj-$(CONFIG_MAPPING_DIRTY_HELPERS) += mapping_dirty_helpers.o
obj-$(CONFIG_PTDUMP) += ptdump.o
obj-$(CONFIG_PAGE_REPORTING) += page_reporting.o
obj-$(CONFIG_GENERIC_IOREMAP) += ioremap.o
obj-$(CONFIG_SHRINKER_DEBUG) += shrinker_debug.o
obj-$(CONFIG_EXECMEM) += execmem.o
obj-$(CONFIG_TMPFS_QUOTA) += shmem_quota.o
obj-$(CONFIG_LAZY_MMU_MODE_KUNIT_TEST) += tests/lazy_mmu_mode_kunit.o
obj-$(CONFIG_MEM_ALLOC_PROFILING) += alloc_tag.o

ifeq ($(CONFIG_RUST_WORKINGSET),y)
# Preserve workingset.o's original archive position and the C fallback.
# The declaration bridge owns the initcall record, so keep it at this position
# as well instead of moving workingset initialization to the end of mm/.
obj-y := $(patsubst workingset.o,workingset.o workingset_helpers.o,$(obj-y))
RUSTFLAGS_workingset.o += -Zfunction-sections=n
$(obj)/workingset.o: $(src)/workingset.rs $(objtree)/rust/bindings/workingset_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

# Explicit targets have no implicit stem; keep the canonical owner identity.
$(addprefix $(obj)/,workingset.o workingset.s workingset.ll workingset.rsi .rust-listing/workingset.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))

# Inspection uses the selected source and its canonical per-object policy.
$(addprefix $(obj)/,workingset.s workingset.ll workingset.rsi): private target-stem := workingset
$(addprefix $(obj)/,workingset.s workingset.ll workingset.rsi): private part-of-builtin = $(if $(filter $(obj)/workingset.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,workingset.s workingset.ll workingset.rsi): private part-of-module = $(if $(filter $(obj)/workingset.o,$(real-obj-m)),y)
$(obj)/workingset.s: $(src)/workingset.rs $(objtree)/rust/bindings/workingset_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/workingset.ll: $(src)/workingset.rs $(objtree)/rust/bindings/workingset_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/workingset.rsi: $(src)/workingset.rs $(objtree)/rust/bindings/workingset_generated.rs FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Debug information belongs only to this unlinked listing object.
$(obj)/.rust-listing/workingset.o: private target-stem := workingset
$(obj)/.rust-listing/workingset.o: private part-of-builtin = $(if $(filter $(obj)/workingset.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/workingset.o: private part-of-module = $(if $(filter $(obj)/workingset.o,$(real-obj-m)),y)
$(obj)/.rust-listing/workingset.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/workingset.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/workingset.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/workingset.o: $(src)/workingset.rs $(objtree)/rust/bindings/workingset_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_workingset_rust_lst_elf = LD      $@
      cmd_workingset_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/workingset.o: $(obj)/.rust-listing/workingset.o FORCE
	$(call if_changed,workingset_rust_lst_elf)
# Keep makelst's real System.map behavior, including creation and deletion.
quiet_cmd_workingset_rust_lst = MKLST   $@
      cmd_workingset_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/workingset.lst: $(obj)/.rust-listing-elf/workingset.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,workingset_rust_lst)
targets += workingset.s workingset.ll workingset.rsi workingset.lst \
	.rust-listing/workingset.o .rust-listing-elf/workingset.o
endif

# Private inspection sidecars remain cleanable after deselection.
clean-files += .rust-listing/ .rust-listing-elf/
# Append to mm/Makefile; leave the existing obj-y += init-mm.o archive slot.
ifeq ($(CONFIG_RUST_INIT_MM),y)
RUSTFLAGS_init-mm.o += -Zfunction-sections=n

# Explicit rules select the Rust stem even while init-mm.c remains adjacent.
$(addprefix $(obj)/,init-mm.o init-mm.s init-mm.ll init-mm.rsi .rust-listing/init-mm.o): private target-stem := init-mm
$(addprefix $(obj)/,init-mm.o init-mm.s init-mm.ll init-mm.rsi .rust-listing/init-mm.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,init-mm.s init-mm.ll init-mm.rsi .rust-listing/init-mm.o): private part-of-builtin = \
    $(if $(filter $(obj)/init-mm.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,init-mm.s init-mm.ll init-mm.rsi .rust-listing/init-mm.o): private part-of-module :=

$(obj)/init-mm.o: $(src)/init-mm.rs $(src)/init_mm_initializers.rs \
    $(objtree)/rust/bindings/init_mm_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/init-mm.s: $(src)/init-mm.rs $(src)/init_mm_initializers.rs \
    $(objtree)/rust/bindings/init_mm_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/init-mm.ll: $(src)/init-mm.rs $(src)/init_mm_initializers.rs \
    $(objtree)/rust/bindings/init_mm_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
# Propagate the compiler status before formatting the source listing.
quiet_cmd_init_mm_rust_rsi = $(RUSTC_OR_CLIPPY_QUIET) $(quiet_modtag) $@
      cmd_init_mm_rust_rsi = \
	$(rust_common_cmd) -Zunpretty=expanded $< >$@ || exit $$?; \
	command -v $(RUSTFMT) >/dev/null || exit $$?; \
	$(RUSTFMT) --config-path $(srctree)/.rustfmt.toml $@ || exit $$?
$(obj)/init-mm.rsi: $(src)/init-mm.rs $(src)/init_mm_initializers.rs \
    $(objtree)/rust/bindings/init_mm_generated.rs FORCE
	+$(call if_changed_dep,init_mm_rust_rsi)

# The generic .lst rule would compile adjacent C into canonical init-mm.o.
# Build a private Rust debug object and materialize ELF without touching it.
$(obj)/.rust-listing/init-mm.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/init-mm.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/init-mm.o: $(src)/init-mm.rs $(src)/init_mm_initializers.rs \
    $(objtree)/rust/bindings/init_mm_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_init_mm_rust_lst_elf = LD      $@
      cmd_init_mm_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/init-mm.o: $(obj)/.rust-listing/init-mm.o FORCE
	$(call if_changed,init_mm_rust_lst_elf)
# Retain makelst's System.map behavior, including creation and deletion.
quiet_cmd_init_mm_rust_lst = MKLST   $@
      cmd_init_mm_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/init-mm.lst: $(obj)/.rust-listing-elf/init-mm.o $(srctree)/scripts/makelst \
    $(wildcard System.map) FORCE
	$(call if_changed,init_mm_rust_lst)

targets += init-mm.s init-mm.ll init-mm.rsi init-mm.lst \
    .rust-listing/init-mm.o .rust-listing-elf/init-mm.o
endif
# mm/Makefile already lists .rust-listing/ and .rust-listing-elf/ in clean-files
# outside the selector, preserving sidecar cleanup after deselection.

include $(srctree)/mm/mmap_native_rust.mk

include $(srctree)/mm/memory_native_rust.mk
include $(srctree)/mm/userfaultfd_native_rust.mk
include $(srctree)/mm/mremap_native_rust.mk
include $(srctree)/mm/mprotect_native_rust.mk

include $(srctree)/mm/rmap_native_rust.mk

include $(srctree)/mm/vmscan_native_rust.mk

include $(srctree)/mm/slab_common-rust.mk

include $(srctree)/mm/compaction_native_rust.mk

include $(srctree)/mm/migrate_native_rust.mk

include $(srctree)/mm/vmstat_native_rust.mk

include $(srctree)/mm/filemap_native_rust.mk

include $(srctree)/mm/shmem_native_rust.mk
