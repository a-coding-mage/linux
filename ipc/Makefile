# SPDX-License-Identifier: GPL-2.0
#
# Makefile for the linux ipc.
#

obj-$(CONFIG_SYSVIPC_COMPAT) += compat.o
obj-$(CONFIG_SYSVIPC) += util.o msgutil.o msg.o sem.o shm.o syscall.o
obj-$(CONFIG_SYSVIPC_SYSCTL) += ipc_sysctl.o
obj-$(CONFIG_POSIX_MQUEUE) += mqueue.o msgutil.o
obj-$(CONFIG_IPC_NS) += namespace.o
obj-$(CONFIG_POSIX_MQUEUE_SYSCTL) += mq_sysctl.o


ifeq ($(CONFIG_RUST_SYSVIPC_MSG),y)
# Keep msg.o's original archive position, selecting the Rust implementation.
obj-y += msg_helpers.o
RUSTFLAGS_msg.o += -Zfunction-sections=n
$(obj)/msg.o: $(src)/msg.rs $(objtree)/rust/bindings/ipc_msg_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

# Explicit targets have no implicit stem; keep the canonical owner identity.
$(addprefix $(obj)/,msg.o msg.s msg.ll msg.rsi .rust-listing/msg.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))

# Inspection uses the selected source and its canonical per-object policy.
$(addprefix $(obj)/,msg.s msg.ll msg.rsi): private target-stem := msg
$(addprefix $(obj)/,msg.s msg.ll msg.rsi): private part-of-builtin = $(if $(filter $(obj)/msg.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,msg.s msg.ll msg.rsi): private part-of-module = $(if $(filter $(obj)/msg.o,$(real-obj-m)),y)
$(obj)/msg.s: $(src)/msg.rs $(objtree)/rust/bindings/ipc_msg_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/msg.ll: $(src)/msg.rs $(objtree)/rust/bindings/ipc_msg_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/msg.rsi: $(src)/msg.rs $(objtree)/rust/bindings/ipc_msg_generated.rs FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Debug information belongs only to this unlinked listing object.
$(obj)/.rust-listing/msg.o: private target-stem := msg
$(obj)/.rust-listing/msg.o: private part-of-builtin = $(if $(filter $(obj)/msg.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/msg.o: private part-of-module = $(if $(filter $(obj)/msg.o,$(real-obj-m)),y)
$(obj)/.rust-listing/msg.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/msg.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/msg.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/msg.o: $(src)/msg.rs $(objtree)/rust/bindings/ipc_msg_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_msg_rust_lst_elf = LD      $@
      cmd_msg_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/msg.o: $(obj)/.rust-listing/msg.o FORCE
	$(call if_changed,msg_rust_lst_elf)
# Keep makelst's real System.map behavior, including creation and deletion.
quiet_cmd_msg_rust_lst = MKLST   $@
      cmd_msg_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/msg.lst: $(obj)/.rust-listing-elf/msg.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,msg_rust_lst)
targets += msg.s msg.ll msg.rsi msg.lst \
	.rust-listing/msg.o .rust-listing-elf/msg.o
endif

# Private inspection sidecars remain cleanable after deselection.
clean-files += .rust-listing/ .rust-listing-elf/
