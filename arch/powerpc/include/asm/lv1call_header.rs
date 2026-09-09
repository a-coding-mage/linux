/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of the PS3 hypervisor lv1 call interface. */

/* The C header's preprocessor-generated argument declarations are represented
 * by this declarative macro.  The raw entry point is kept separate so callers
 * may instrument the public wrapper, as in the original header. */
#[macro_export]
macro_rules! LV1_CALL {
    ($name:ident, $raw:ident, [$($in:ident),*], [$($out:ident),*], $number:expr) => {
        extern "C" {
            #[link_name = concat!("_lv1_", stringify!($name))]
            pub fn $raw($($in: u64,)* $($out: *mut u64,)*) -> i64;
        }
        #[inline]
        pub unsafe fn $name($($in: u64,)* $($out: *mut u64,)*) -> i32 {
            $raw($($in,)* $($out,)*) as i32
        }
    };
}

/* lv1 call table (input and output argument counts are retained explicitly). */
pub const LV1_CALL_TABLE: &[(&str, usize, usize, u32)] = &[
    ("allocate_memory",4,2,0), ("write_htab_entry",4,0,1),
    ("construct_virtual_address_space",3,2,2), ("invalidate_htab_entries",5,0,3),
    ("get_virtual_address_space_id_of_ppe",0,1,4),
    ("query_logical_partition_address_region_info",1,5,6),
    ("select_virtual_address_space",1,0,7), ("pause",1,0,9),
    ("destruct_virtual_address_space",1,0,10), ("configure_irq_state_bitmap",3,0,11),
    ("connect_irq_plug_ext",5,0,12), ("release_memory",1,0,13),
    ("put_iopte",5,0,15), ("disconnect_irq_plug_ext",3,0,17),
    ("construct_event_receive_port",0,1,18), ("destruct_event_receive_port",1,0,19),
    ("send_event_locally",1,0,24), ("end_of_interrupt",1,0,27),
    ("connect_irq_plug",2,0,28), ("disconnect_irq_plug",1,0,29),
    ("end_of_interrupt_ext",3,0,30), ("did_update_interrupt_mask",2,0,31),
    ("shutdown_logical_partition",1,0,44), ("destruct_logical_spe",1,0,54),
    ("construct_logical_spe",7,6,57), ("set_spe_interrupt_mask",3,0,61),
    ("set_spe_transition_notifier",3,0,64), ("disable_logical_spe",2,0,65),
    ("clear_spe_interrupt_status",4,0,66), ("get_spe_interrupt_status",2,1,67),
    ("get_logical_ppe_id",0,1,69), ("set_interrupt_mask",5,0,73),
    ("get_logical_partition_id",0,1,74), ("configure_execution_time_variable",1,0,77),
    ("get_spe_irq_outlet",2,1,78),
    ("set_spe_privilege_state_area_1_register",3,0,79),
    ("create_repository_node",6,0,90), ("read_repository_node",5,2,91),
    ("write_repository_node",6,0,92), ("delete_repository_node",4,0,93),
    ("read_htab_entries",2,5,95), ("set_dabr",2,0,96),
    ("get_total_execution_time",2,1,103), ("allocate_io_segment",3,1,116),
    ("release_io_segment",2,0,117), ("construct_io_irq_outlet",1,1,120),
    ("destruct_io_irq_outlet",1,0,121), ("map_htab",1,1,122),
    ("unmap_htab",1,0,123), ("get_version_info",0,2,127),
    ("insert_htab_entry",6,3,158), ("read_virtual_uart",3,1,162),
    ("write_virtual_uart",3,1,163), ("set_virtual_uart_param",3,0,164),
    ("get_virtual_uart_param",2,1,165), ("configure_virtual_uart_irq",1,1,166),
    ("open_device",3,0,170), ("close_device",2,0,171),
    ("map_device_mmio_region",5,1,172), ("unmap_device_mmio_region",3,0,173),
    ("allocate_device_dma_region",5,1,174), ("free_device_dma_region",3,0,175),
    ("map_device_dma_region",6,0,176), ("unmap_device_dma_region",4,0,177),
    ("net_add_multicast_address",4,0,185), ("net_remove_multicast_address",4,0,186),
    ("net_start_tx_dma",4,0,187), ("net_stop_tx_dma",2,0,188),
    ("net_start_rx_dma",4,0,189), ("net_stop_rx_dma",2,0,190),
    ("net_set_interrupt_status_indicator",4,0,191), ("net_set_interrupt_mask",4,0,193),
    ("net_control",6,2,194), ("connect_interrupt_event_receive_port",4,0,197),
    ("disconnect_interrupt_event_receive_port",4,0,198),
    ("get_spe_all_interrupt_statuses",1,1,199), ("deconfigure_virtual_uart_irq",0,0,202),
    ("enable_logical_spe",2,0,207), ("gpu_open",1,0,210), ("gpu_close",0,0,211),
    ("gpu_device_map",1,2,212), ("gpu_device_unmap",1,0,213),
    ("gpu_memory_allocate",5,2,214), ("gpu_memory_free",1,0,216),
    ("gpu_context_allocate",2,5,217), ("gpu_context_free",1,0,218),
    ("gpu_context_iomap",5,0,221), ("gpu_context_attribute",6,0,225),
    ("gpu_context_intr",1,1,227), ("gpu_attribute",3,0,228),
    ("get_rtc",0,2,232), ("set_ppe_periodic_tracer_frequency",1,0,240),
    ("start_ppe_periodic_tracer",5,0,241), ("stop_ppe_periodic_tracer",1,1,242),
    ("storage_read",6,1,245), ("storage_write",6,1,246),
    ("storage_send_device_command",6,1,248), ("storage_get_async_status",1,2,249),
    ("storage_check_async_status",2,1,254), ("panic",1,0,255),
    ("construct_lpm",6,3,140), ("destruct_lpm",1,0,141), ("start_lpm",1,0,142),
    ("stop_lpm",1,1,143), ("copy_lpm_trace_buffer",3,1,144),
    ("add_lpm_event_bookmark",5,0,145), ("delete_lpm_event_bookmark",3,0,146),
    ("set_lpm_interrupt_mask",3,1,147), ("get_lpm_interrupt_status",1,1,148),
    ("set_lpm_general_control",5,2,149), ("set_lpm_interval",3,1,150),
    ("set_lpm_trigger_control",3,1,151), ("set_lpm_counter_control",4,1,152),
    ("set_lpm_group_control",3,1,153), ("set_lpm_debug_bus_control",3,1,154),
    ("set_lpm_counter",5,2,155), ("set_lpm_signal",7,0,156),
    ("set_lpm_spr_trigger",2,0,157),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
