# SPDX-License-Identifier: GPL-2.0-only
# One inactive declaration target for all five owners. The five owner fragments
# retain their exact SOURCE ONLY HOLD errors; this adds no admission or objects.
targets += bindings/sched_ext_native_generated.rs
ifneq ($(filter y,$(CONFIG_RUST_SCHED_EXT_CORE) $(CONFIG_RUST_SCHED_EXT_CID) \
    $(CONFIG_RUST_SCHED_EXT_ARENA) $(CONFIG_RUST_SCHED_EXT_IDLE) \
    $(CONFIG_RUST_SCHED_EXT_SUB)),)
always-y += bindings/sched_ext_native_generated.rs
$(obj)/bindings.o: $(obj)/bindings/sched_ext_native_generated.rs
endif

$(obj)/bindings/sched_ext_native_generated.rs: private bindgen_target_flags = \
    $(shell sed '/^[[:space:]]*\#/d; /^[[:space:]]*$$/d' $(srctree)/kernel/sched/ext/sched_ext_bindgen_parameters)
$(obj)/bindings/sched_ext_native_generated.rs: private bindgen_target_cflags = -UMODULE
$(obj)/bindings/sched_ext_native_generated.rs: $(srctree)/kernel/sched/ext/sched_ext_bindings.h \
    $(sort $(sched-ext-native-headers)) $(srctree)/kernel/sched/ext/types.h \
    $(srctree)/kernel/sched/ext/sched_ext_bindgen_parameters FORCE
	$(call if_changed_dep,bindgen)
