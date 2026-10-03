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
$(obj)/rw.s: $(src)/rw.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/rw.ll: $(src)/rw.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
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
