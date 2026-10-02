# SPDX-License-Identifier: GPL-2.0

ifeq ($(CONFIG_DYNAMIC_DEBUG), y)
CFLAGS_swap.o                   := -DDEBUG
CFLAGS_snapshot.o               := -DDEBUG
CFLAGS_energy_model.o           := -DDEBUG
endif

KASAN_SANITIZE_snapshot.o	:= n

obj-y				+= qos.o
obj-$(CONFIG_PM)		+= main.o
obj-$(CONFIG_VT_CONSOLE_SLEEP)	+= console.o
obj-$(CONFIG_FREEZER)		+= process.o

# Keep private inspection outputs cleanable when the selector is disabled.
clean-files += .rust-listing/ .rust-listing-elf/
clean-files += process-state-generated.rs process-state-generated.rs.i \
	process-state-generated.rs.tmp

ifeq ($(CONFIG_RUST_POWER_PROCESS),y)
# Retain the original built-in process.o slot and unmodified process.c.
# Explicit rules select Rust for both code and inspection products.
$(addprefix $(obj)/,process.o process.s process.ll process.rsi \
	.rust-listing/process.o): private modname-multi :=
$(obj)/process.o: $(src)/process.rs $(objtree)/rust/bindings/power_process_generated.rs \
	$(obj)/process-state-generated.rs \
	$(wildcard $(objtree)/include/config/RUST_POWER_PROCESS) FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/process.s: $(src)/process.rs $(objtree)/rust/bindings/power_process_generated.rs \
	$(obj)/process-state-generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/process.ll: $(src)/process.rs $(objtree)/rust/bindings/power_process_generated.rs \
	$(obj)/process-state-generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/process.rsi: $(src)/process.rs $(objtree)/rust/bindings/power_process_generated.rs \
	$(obj)/process-state-generated.rs FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# As in the current BPF providers, listings use a private debug object and,
# if needed, a private ELF materialization rather than altering process.o.
$(obj)/.rust-listing/process.o: private target-stem := process
$(obj)/.rust-listing/process.o: private part-of-builtin = $(if $(filter $(obj)/process.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/process.o: private part-of-module :=
$(obj)/.rust-listing/process.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/process.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/process.o: $(src)/process.rs $(objtree)/rust/bindings/power_process_generated.rs \
	$(obj)/process-state-generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_power_process_lst_elf = LD      $@
      cmd_power_process_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/process.o: $(obj)/.rust-listing/process.o FORCE
	$(call if_changed,power_process_lst_elf)
quiet_cmd_power_process_lst = MKLST   $@
      cmd_power_process_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/process.lst: $(obj)/.rust-listing-elf/process.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,power_process_lst)
targets += process.s process.ll process.rsi process.lst \
	.rust-listing/process.o .rust-listing-elf/process.o

# Expand __read_mostly with this directory's actual C adapter flags, including
# its built-in identity and instrumentation policy. No architecture list or
# assumed section/alignment is used. Unknown annotations fail closed.
# Kbuild appends fixdep to this command under set -e: keep these as sequential
# commands so any failed stage stops before a saved command can be replaced.
quiet_cmd_power_process_state = GEN     $@
      cmd_power_process_state = \
	$(CC) $(c_flags) -E -P -x c $< -o $@.i; \
	$(AWK) -f $(src)/process-state.awk $@.i > $@.tmp; \
	mv -f $@.tmp $@; rm -f $@.i
$(obj)/process-state-generated.rs: private target-stem := process-rust
$(obj)/process-state-generated.rs: private basetarget := process-rust
$(obj)/process-state-generated.rs: private modname-multi :=
$(obj)/process-state-generated.rs: private part-of-builtin := y
$(obj)/process-state-generated.rs: private part-of-module :=
$(obj)/process-state-generated.rs: private is-kernel-object := y
$(obj)/process-state-generated.rs: $(src)/process-state.h $(src)/process-rust.h \
	$(src)/process-state.awk $(src)/Makefile FORCE
	$(call if_changed_dep,power_process_state)
targets += process-state-generated.rs
obj-y += process-rust.o
endif

obj-$(CONFIG_SUSPEND)		+= suspend.o
obj-$(CONFIG_PM_TEST_SUSPEND)	+= suspend_test.o
obj-$(CONFIG_HIBERNATION)	+= hibernate.o snapshot.o swap.o
obj-$(CONFIG_HIBERNATION_SNAPSHOT_DEV) += user.o
obj-$(CONFIG_PM_AUTOSLEEP)	+= autosleep.o
obj-$(CONFIG_PM_WAKELOCKS)	+= wakelock.o

obj-$(CONFIG_MAGIC_SYSRQ)	+= poweroff.o

obj-$(CONFIG_ENERGY_MODEL)	+= em.o
em-y				:= energy_model.o
em-$(CONFIG_NET)		+= em_netlink_autogen.o em_netlink.o
