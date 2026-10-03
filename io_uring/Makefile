# SPDX-License-Identifier: GPL-2.0
#
# Makefile for io_uring

ifdef CONFIG_GCOV_PROFILE_URING
GCOV_PROFILE := y
endif

obj-$(CONFIG_IO_URING)		+= io_uring.o opdef.o kbuf.o rsrc.o notif.o \
					tctx.o filetable.o rw.o poll.o \
					tw.o wait.o eventfd.o uring_cmd.o \
					openclose.o sqpoll.o xattr.o nop.o \
					fs.o splice.o sync.o msg_ring.o \
					advise.o openclose.o statx.o timeout.o \
					cancel.o waitid.o register.o \
					truncate.o memmap.o alloc_cache.o \
					query.o loop.o

obj-$(CONFIG_IO_URING_ZCRX)	+= zcrx.o
obj-$(CONFIG_IO_WQ)		+= io-wq.o
obj-$(CONFIG_FUTEX)		+= futex.o
obj-$(CONFIG_EPOLL)		+= epoll.o
obj-$(CONFIG_NET_RX_BUSY_POLL)	+= napi.o
obj-$(CONFIG_NET) += net.o cmd_net.o
obj-$(CONFIG_PROC_FS) += fdinfo.o
obj-$(CONFIG_IO_URING_MOCK_FILE) += mock_file.o
obj-$(CONFIG_IO_URING_BPF) += bpf_filter.o
obj-$(CONFIG_IO_URING_BPF_OPS) += bpf-ops.o

# Preserve rw.o's external ABI while selecting exactly one implementation.
ifeq ($(CONFIG_RUST_IO_URING_RW),y)
obj-$(CONFIG_IO_URING) += rw_helpers.o
$(obj)/rw.o: $(src)/rw.rs $(objtree)/rust/bindings/io_uring_rw_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

# Explicit targets have no implicit stem; keep the canonical owner identity.
$(addprefix $(obj)/,rw.o rw.s rw.ll rw.rsi .rust-listing/rw.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))

# Inspection uses the selected source and its canonical per-object policy.
$(addprefix $(obj)/,rw.s rw.ll rw.rsi): private target-stem := rw
$(addprefix $(obj)/,rw.s rw.ll rw.rsi): private part-of-builtin = $(if $(filter $(obj)/rw.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,rw.s rw.ll rw.rsi): private part-of-module = $(if $(filter $(obj)/rw.o,$(real-obj-m)),y)
$(obj)/rw.s: $(src)/rw.rs $(objtree)/rust/bindings/io_uring_rw_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/rw.ll: $(src)/rw.rs $(objtree)/rust/bindings/io_uring_rw_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/rw.rsi: $(src)/rw.rs $(objtree)/rust/bindings/io_uring_rw_generated.rs FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Debug information belongs only to this unlinked listing object.
$(obj)/.rust-listing/rw.o: private target-stem := rw
$(obj)/.rust-listing/rw.o: private part-of-builtin = $(if $(filter $(obj)/rw.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/rw.o: private part-of-module = $(if $(filter $(obj)/rw.o,$(real-obj-m)),y)
$(obj)/.rust-listing/rw.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/rw.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/rw.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/rw.o: $(src)/rw.rs $(objtree)/rust/bindings/io_uring_rw_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_rw_rust_lst_elf = LD      $@
      cmd_rw_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/rw.o: $(obj)/.rust-listing/rw.o FORCE
	$(call if_changed,rw_rust_lst_elf)
# Keep makelst's real System.map behavior, including creation and deletion.
quiet_cmd_rw_rust_lst = MKLST   $@
      cmd_rw_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/rw.lst: $(obj)/.rust-listing-elf/rw.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,rw_rust_lst)
targets += rw.s rw.ll rw.rsi rw.lst \
	.rust-listing/rw.o .rust-listing-elf/rw.o
endif

# The explicit recipe overrides only zcrx.o; never link both implementations.
ifeq ($(CONFIG_RUST_IO_URING_ZCRX),y)
obj-$(CONFIG_IO_URING_ZCRX) += zcrx_helpers.o
$(obj)/zcrx.o: $(src)/zcrx.rs $(objtree)/rust/bindings/io_uring_zcrx_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/zcrx.s: $(src)/zcrx.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/zcrx.ll: $(src)/zcrx.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
endif

# Private inspection sidecars remain cleanable after deselection.
clean-files += .rust-listing/ .rust-listing-elf/
