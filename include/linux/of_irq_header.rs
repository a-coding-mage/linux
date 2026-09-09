/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux Rust bindings.

pub type of_irq_init_cb_t = unsafe extern "C" fn(*mut device_node, *mut device_node) -> i32;

#[repr(C)]
pub struct of_imap_parser {
    pub node: *mut device_node,
    pub imap: *const __be32,
    pub imap_end: *const __be32,
    pub parent_offset: u32,
}

#[repr(C)]
pub struct of_imap_item {
    pub parent_args: of_phandle_args,
    pub child_imap_count: u32,
    pub child_imap: [u32; 16], /* Arbitrary size.
                                 * Should be #address-cells + #interrupt-cells but
                                 * avoid using allocation and so, expect that 16
                                 * should be enough
                                 */
}

/*
 * If the iterator is exited prematurely (break, goto, return) of_node_put() has
 * to be called on item.parent_args.np
 */
#[macro_export]
macro_rules! for_each_of_imap_item {
    ($parser:expr, $item:expr) => {
        while unsafe { of_imap_parser_one($parser, $item) }.is_null() == false {}
    };
}

/* Workarounds only applied to 32bit powermac machines */
pub const OF_IMAP_OLDWORLD_MAC: u32 = 0x00000001;
pub const OF_IMAP_NO_PHANDLE: u32 = 0x00000002;

#[cfg(all(CONFIG_PPC32, CONFIG_PPC_PMAC))]
extern "C" {
    pub static mut of_irq_workarounds: c_uint;
    pub static mut of_irq_dflt_pic: *mut device_node;
    pub fn of_irq_parse_oldworld(device: *const device_node, index: i32,
                                  out_irq: *mut of_phandle_args) -> i32;
}

#[cfg(not(all(CONFIG_PPC32, CONFIG_PPC_PMAC)))]
pub const of_irq_workarounds: u32 = 0;
#[cfg(not(all(CONFIG_PPC32, CONFIG_PPC_PMAC)))]
pub const of_irq_dflt_pic: *mut device_node = core::ptr::null_mut();
#[cfg(not(all(CONFIG_PPC32, CONFIG_PPC_PMAC)))]
pub unsafe fn of_irq_parse_oldworld(_device: *const device_node, _index: i32,
                                    _out_irq: *mut of_phandle_args) -> i32 { -EINVAL }

extern "C" {
    pub fn of_irq_parse_raw(addr: *const __be32, out_irq: *mut of_phandle_args) -> i32;
    pub fn irq_create_of_mapping(irq_data: *mut of_phandle_args) -> c_uint;
    pub fn of_irq_to_resource(dev: *mut device_node, index: i32, r: *mut resource) -> i32;
}

#[cfg(CONFIG_OF_IRQ)]
extern "C" {
    pub fn of_irq_init(matches: *const of_device_id);
    pub fn of_irq_parse_one(device: *mut device_node, index: i32, out_irq: *mut of_phandle_args) -> i32;
    pub fn of_irq_count(dev: *mut device_node) -> i32;
    pub fn of_irq_get(dev: *mut device_node, index: i32) -> i32;
    pub fn of_irq_get_affinity(dev: *mut device_node, index: i32) -> *const cpumask;
    pub fn of_irq_get_byname(dev: *mut device_node, name: *const c_char) -> i32;
    pub fn of_irq_to_resource_table(dev: *mut device_node, res: *mut resource, nr_irqs: i32) -> i32;
    pub fn of_irq_find_parent(child: *mut device_node) -> *mut device_node;
    pub fn of_imap_parser_init(parser: *mut of_imap_parser, node: *mut device_node, item: *mut of_imap_item) -> i32;
    pub fn of_imap_parser_one(parser: *mut of_imap_parser, item: *mut of_imap_item) -> *mut of_imap_item;
    pub fn of_msi_get_domain(dev: *mut device, np: *const device_node, token: irq_domain_bus_token) -> *mut irq_domain;
    pub fn of_msi_map_get_device_domain(dev: *mut device, id: u32, bus_token: u32) -> *mut irq_domain;
    pub fn of_msi_configure(dev: *mut device, np: *const device_node);
    pub fn of_msi_xlate(dev: *mut device, msi_np: *mut *mut device_node, id_in: u32) -> u32;
}

#[cfg(not(CONFIG_OF_IRQ))]
pub unsafe fn of_irq_init(_matches: *const of_device_id) {}
#[cfg(not(CONFIG_OF_IRQ))]
pub unsafe fn of_irq_parse_one(_device: *mut device_node, _index: i32, _out_irq: *mut of_phandle_args) -> i32 { -EINVAL }
#[cfg(not(CONFIG_OF_IRQ))]
pub unsafe fn of_irq_count(_dev: *mut device_node) -> i32 { 0 }
#[cfg(not(CONFIG_OF_IRQ))]
pub unsafe fn of_irq_get(_dev: *mut device_node, _index: i32) -> i32 { 0 }
#[cfg(not(CONFIG_OF_IRQ))]
pub unsafe fn of_irq_get_byname(_dev: *mut device_node, _name: *const c_char) -> i32 { 0 }
#[cfg(not(CONFIG_OF_IRQ))]
pub unsafe fn of_irq_get_affinity(_dev: *mut device_node, _index: i32) -> *const cpumask { core::ptr::null() }
#[cfg(not(CONFIG_OF_IRQ))]
pub unsafe fn of_irq_to_resource_table(_dev: *mut device_node, _res: *mut resource, _nr_irqs: i32) -> i32 { 0 }
#[cfg(not(CONFIG_OF_IRQ))]
pub unsafe fn of_irq_find_parent(_child: *mut device_node) -> *mut device_node { core::ptr::null_mut() }
#[cfg(not(CONFIG_OF_IRQ))]
pub unsafe fn of_imap_parser_init(_parser: *mut of_imap_parser, _node: *mut device_node, _item: *mut of_imap_item) -> i32 { -ENOSYS }
#[cfg(not(CONFIG_OF_IRQ))]
pub unsafe fn of_imap_parser_one(_parser: *mut of_imap_parser, _item: *mut of_imap_item) -> *mut of_imap_item { core::ptr::null_mut() }
#[cfg(not(CONFIG_OF_IRQ))]
pub unsafe fn of_msi_get_domain(_dev: *mut device, _np: *mut device_node, _token: irq_domain_bus_token) -> *mut irq_domain { core::ptr::null_mut() }
#[cfg(not(CONFIG_OF_IRQ))]
pub unsafe fn of_msi_map_get_device_domain(_dev: *mut device, _id: u32, _bus_token: u32) -> *mut irq_domain { core::ptr::null_mut() }
#[cfg(not(CONFIG_OF_IRQ))]
pub unsafe fn of_msi_configure(_dev: *mut device, _np: *mut device_node) {}
#[cfg(not(CONFIG_OF_IRQ))]
pub unsafe fn of_msi_xlate(_dev: *mut device, _msi_np: *mut *mut device_node, id_in: u32) -> u32 { id_in }

#[cfg(any(CONFIG_OF_IRQ, CONFIG_SPARC))]
extern "C" { pub fn irq_of_parse_and_map(node: *mut device_node, index: i32) -> c_uint; }
#[cfg(not(any(CONFIG_OF_IRQ, CONFIG_SPARC)))]
pub unsafe fn irq_of_parse_and_map(_dev: *mut device_node, _index: i32) -> c_uint { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
