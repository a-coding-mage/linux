/* SPDX-License-Identifier: GPL-2.0 */

// Declarations are provided when CONFIG_PROVIDE_OHCI1394_DMA_INIT is enabled.
unsafe extern "C" {
    // C annotation: __initdata
    pub static mut init_ohci1394_dma_early: i32;

    // C annotation: __init
    pub fn init_ohci1394_dma_on_all_controllers();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
