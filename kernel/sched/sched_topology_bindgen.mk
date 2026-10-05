# SPDX-License-Identifier: GPL-2.0
# Source registration only; no configured native generation is admitted.
ifeq ($(CONFIG_RUST_SCHED_TOPOLOGY),y)
$(error SOURCE ONLY HOLD: Rust scheduler topology bindings are unqualified)
endif
targets += bindings/sched_topology_generated.rs
always-$(CONFIG_RUST_SCHED_TOPOLOGY) += bindings/sched_topology_generated.rs
$(obj)/bindings/sched_topology_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/sched_topology_bindgen_parameters)
$(obj)/bindings/sched_topology_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_topology_generated.rs: $(srctree)/kernel/sched/sched_topology_bindings.h \
    $(srctree)/kernel/sched/sched_topology_energy_bindings.h \
    $(srctree)/kernel/sched/sched_topology_root_bindings.h \
    $(srctree)/kernel/sched/sched_topology_cache_bindings.h \
    $(srctree)/kernel/sched/sched_topology_init_bindings.h \
    $(srctree)/kernel/sched/sched_topology_numa_bindings.h \
    $(srctree)/kernel/sched/sched_topology_groups_bindings.h \
    $(srctree)/kernel/sched/sched_topology_build_bindings.h \
    $(srctree)/kernel/sched/sched_topology_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
ifeq ($(CONFIG_RUST_SCHED_TOPOLOGY),y)
$(obj)/bindings.o: $(obj)/bindings/sched_topology_generated.rs
endif
