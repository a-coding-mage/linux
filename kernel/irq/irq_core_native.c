// SPDX-License-Identifier: GPL-2.0
/* Compiler/architecture primitives and native initializer/registration macros.
 * No irqdesc.c, handle.c or chip.c body is included, renamed or forwarded.
 * Descriptor/dispatch/chip decisions, loops and lifetimes are Rust-owned.
 */
#include "irq_core_bindings.h"

#define IRQ_PRIMITIVE(ret, name, args, body) ret lupos_irq_##name args body
#include "irq_core_primitives.inc"
#undef IRQ_PRIMITIVE

extern int __init lupos_irq_duration_setup(char *arg);
__setup("irqhandler.duration_warn_us=", lupos_irq_duration_setup);
#ifdef CONFIG_SMP
extern int __init lupos_irq_affinity_setup(char *arg);
__setup("irqaffinity=", lupos_irq_affinity_setup);
#endif

#ifdef CONFIG_SPARSE_IRQ
extern void lupos_irq_kobj_release(struct kobject *kobj);
#ifdef CONFIG_SYSFS
#define LUPOS_IRQ_ATTR(name) \
    extern ssize_t lupos_irq_##name##_show(struct kobject *, struct kobj_attribute *, char *); \
    static struct kobj_attribute name##_attr = __ATTR(name, 0444, lupos_irq_##name##_show, NULL)
LUPOS_IRQ_ATTR(per_cpu_count);
LUPOS_IRQ_ATTR(chip_name);
LUPOS_IRQ_ATTR(hwirq);
LUPOS_IRQ_ATTR(type);
LUPOS_IRQ_ATTR(wakeup);
LUPOS_IRQ_ATTR(name);
LUPOS_IRQ_ATTR(actions);
static struct attribute *irq_attrs[] = {
    &per_cpu_count_attr.attr,
    &chip_name_attr.attr,
    &hwirq_attr.attr,
    &type_attr.attr,
    &wakeup_attr.attr,
    &name_attr.attr,
    &actions_attr.attr,
    NULL,
};
ATTRIBUTE_GROUPS(irq);
const struct kobj_type lupos_irq_kobj_type = {
    .release = lupos_irq_kobj_release,
    .sysfs_ops = &kobj_sysfs_ops,
    .default_groups = irq_groups,
};
extern int __init lupos_irq_sysfs_init(void);
postcore_initcall(lupos_irq_sysfs_init);
#else
const struct kobj_type lupos_irq_kobj_type = { .release = lupos_irq_kobj_release };
#endif
#endif

/* Preserve original export class and configuration guards. */
EXPORT_SYMBOL_GPL(irq_get_nr_irqs);
#ifdef CONFIG_SPARSE_IRQ
#ifdef CONFIG_KVM_BOOK3S_64_HV_MODULE
EXPORT_SYMBOL_GPL(irq_to_desc);
#endif
#else
EXPORT_SYMBOL(irq_to_desc);
#endif
EXPORT_SYMBOL_GPL(generic_handle_irq);
EXPORT_SYMBOL_GPL(generic_handle_irq_safe);
#ifdef CONFIG_IRQ_DOMAIN
EXPORT_SYMBOL_GPL(generic_handle_domain_irq);
EXPORT_SYMBOL_GPL(generic_handle_domain_irq_safe);
EXPORT_SYMBOL_GPL(generic_handle_demux_domain_irq);
#endif
EXPORT_SYMBOL_GPL(irq_free_descs);
EXPORT_SYMBOL_GPL(__irq_alloc_descs);
#ifdef CONFIG_LOCKDEP
EXPORT_SYMBOL_GPL(__irq_set_lockdep_class);
#endif
EXPORT_SYMBOL_GPL(handle_bad_irq);
EXPORT_SYMBOL_GPL(no_action);
EXPORT_SYMBOL(irq_set_chip);
EXPORT_SYMBOL(irq_set_irq_type);
EXPORT_SYMBOL(irq_set_handler_data);
EXPORT_SYMBOL(irq_set_chip_data);
EXPORT_SYMBOL_GPL(irq_get_irq_data);
EXPORT_SYMBOL_GPL(handle_nested_irq);
EXPORT_SYMBOL_GPL(handle_simple_irq);
EXPORT_SYMBOL_GPL(handle_untracked_irq);
EXPORT_SYMBOL_GPL(handle_level_irq);
EXPORT_SYMBOL_GPL(handle_fasteoi_irq);
EXPORT_SYMBOL_GPL(handle_fasteoi_nmi);
EXPORT_SYMBOL(handle_edge_irq);
EXPORT_SYMBOL_GPL(__irq_set_handler);
EXPORT_SYMBOL_GPL(irq_set_chained_handler_and_data);
EXPORT_SYMBOL_GPL(irq_set_chip_and_handler_name);
EXPORT_SYMBOL_GPL(irq_modify_status);
#ifdef CONFIG_IRQ_DOMAIN_HIERARCHY
#ifdef CONFIG_IRQ_FASTEOI_HIERARCHY_HANDLERS
EXPORT_SYMBOL_GPL(handle_fasteoi_ack_irq);
EXPORT_SYMBOL_GPL(handle_fasteoi_mask_irq);
#endif
#ifdef CONFIG_SMP
EXPORT_SYMBOL_GPL(irq_chip_pre_redirect_parent);
#endif
EXPORT_SYMBOL_GPL(irq_chip_set_parent_state);
EXPORT_SYMBOL_GPL(irq_chip_get_parent_state);
EXPORT_SYMBOL_GPL(irq_chip_shutdown_parent);
EXPORT_SYMBOL_GPL(irq_chip_startup_parent);
EXPORT_SYMBOL_GPL(irq_chip_enable_parent);
EXPORT_SYMBOL_GPL(irq_chip_disable_parent);
EXPORT_SYMBOL_GPL(irq_chip_ack_parent);
EXPORT_SYMBOL_GPL(irq_chip_mask_parent);
EXPORT_SYMBOL_GPL(irq_chip_mask_ack_parent);
EXPORT_SYMBOL_GPL(irq_chip_unmask_parent);
EXPORT_SYMBOL_GPL(irq_chip_eoi_parent);
EXPORT_SYMBOL_GPL(irq_chip_set_affinity_parent);
EXPORT_SYMBOL_GPL(irq_chip_set_type_parent);
EXPORT_SYMBOL_GPL(irq_chip_retrigger_hierarchy);
EXPORT_SYMBOL_GPL(irq_chip_set_vcpu_affinity_parent);
EXPORT_SYMBOL_GPL(irq_chip_set_wake_parent);
EXPORT_SYMBOL_GPL(irq_chip_request_resources_parent);
EXPORT_SYMBOL_GPL(irq_chip_release_resources_parent);
#endif
#ifdef CONFIG_SMP
EXPORT_SYMBOL_GPL(irq_chip_redirect_set_affinity);
#endif
