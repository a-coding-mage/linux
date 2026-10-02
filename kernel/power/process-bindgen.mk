# SPDX-License-Identifier: GPL-2.0
# Included by rust/Makefile: use its canonical configured bindgen command.
targets += bindings/power_process_generated.rs
always-$(CONFIG_RUST_POWER_PROCESS) += bindings/power_process_generated.rs
$(obj)/bindings/power_process_generated.rs: private bindgen_target_flags = \
    --no-doc-comments --no-prepend-enum-name --no-default '.*' \
    --allowlist-type 'task_struct|signal_struct|list_head|ktime_t|umh_disable_depth' \
    --allowlist-function 'lupos_process_.*|freeze_workqueues_begin|freeze_workqueues_busy|show_freezable_workqueues|freeze_task|frozen|sched_show_task|__usermodehelper_disable|__usermodehelper_set_disable_depth|oom_killer_disable|oom_killer_enable|thaw_workqueues|__thaw_task|schedule' \
    --allowlist-var 'LUPOS_PROCESS_.*|init_task|freeze_timeout_msecs|pm_freezing|pm_nosig_freezing' \
    --blocklist-type '__kernel_size_t|__kernel_ssize_t|__kernel_ptrdiff_t' \
    --wrap-unsafe-ops
$(obj)/bindings/power_process_generated.rs: $(srctree)/kernel/power/process-rust.h \
    $(srctree)/kernel/power/process-bindgen.mk FORCE
	$(call if_changed_dep,bindgen)
