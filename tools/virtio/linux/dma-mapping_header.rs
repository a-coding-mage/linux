/* SPDX-License-Identifier: GPL-2.0 */

// C header guard removed.
// If CONFIG_HAS_DMA is enabled, the original header emits:
// "Virtio userspace code does not support CONFIG_HAS_DMA".

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum dma_data_direction {
    DMA_BIDIRECTIONAL = 0,
    DMA_TO_DEVICE = 1,
    DMA_FROM_DEVICE = 2,
    DMA_NONE = 3,
}

macro_rules! dma_alloc_coherent {
    ($d:expr, $s:expr, $hp:expr, $f:expr) => {{
        let __dma_alloc_coherent_p = kmalloc(($s), ($f));
        *($hp) = __dma_alloc_coherent_p as unsigned_long;
        __dma_alloc_coherent_p
    }};
}

macro_rules! dma_free_coherent {
    ($d:expr, $s:expr, $p:expr, $h:expr) => {
        kfree($p)
    };
}

macro_rules! dma_map_page {
    ($d:expr, $p:expr, $o:expr, $s:expr, $dir:expr) => {
        page_to_phys($p).wrapping_add($o)
    };
}

macro_rules! dma_map_page_attrs {
    ($d:expr, $p:expr, $o:expr, $s:expr, $dir:expr, $a:expr) => {
        page_to_phys($p).wrapping_add($o)
    };
}

macro_rules! dma_map_single {
    ($d:expr, $p:expr, $s:expr, $dir:expr) => {
        virt_to_phys($p)
    };
}

macro_rules! dma_map_single_attrs {
    ($d:expr, $p:expr, $s:expr, $dir:expr, $a:expr) => {
        virt_to_phys($p)
    };
}

macro_rules! dma_mapping_error {
    ($($arg:expr),* $(,)?) => {
        0
    };
}

macro_rules! dma_unmap_single {
    ($d:expr, $a:expr, $s:expr, $r:expr) => {{
        let _ = &$d;
        let _ = &$a;
        let _ = &$s;
        let _ = &$r;
    }};
}

macro_rules! dma_unmap_page {
    ($d:expr, $a:expr, $s:expr, $r:expr) => {{
        let _ = &$d;
        let _ = &$a;
        let _ = &$s;
        let _ = &$r;
    }};
}

macro_rules! dma_unmap_page_attrs {
    ($d:expr, $a:expr, $s:expr, $r:expr, $t:expr) => {{
        let _ = &$d;
        let _ = &$a;
        let _ = &$s;
        let _ = &$r;
        let _ = &$t;
    }};
}

macro_rules! sg_dma_address {
    ($sg:expr) => {
        0
    };
}

macro_rules! sg_dma_len {
    ($sg:expr) => {
        0
    };
}

macro_rules! dma_need_sync {
    ($v:expr, $a:expr) => {
        0
    };
}

macro_rules! dma_unmap_single_attrs {
    ($d:expr, $a:expr, $s:expr, $r:expr, $t:expr) => {{
        let _ = &$d;
        let _ = &$a;
        let _ = &$s;
        let _ = &$r;
        let _ = &$t;
    }};
}

macro_rules! dma_sync_single_range_for_cpu {
    ($d:expr, $a:expr, $o:expr, $s:expr, $r:expr) => {{
        let _ = &$d;
        let _ = &$a;
        let _ = &$o;
        let _ = &$s;
        let _ = &$r;
    }};
}

macro_rules! dma_sync_single_range_for_device {
    ($d:expr, $a:expr, $o:expr, $s:expr, $r:expr) => {{
        let _ = &$d;
        let _ = &$a;
        let _ = &$o;
        let _ = &$s;
        let _ = &$r;
    }};
}

macro_rules! dma_max_mapping_size {
    ($($arg:expr),* $(,)?) => {
        SIZE_MAX
    };
}

/*
 * A dma_addr_t can hold any valid DMA or bus address for the platform.  It can
 * be given to a device to use as a DMA source or target.  It is specific to a
 * given device and there may be a translation between the CPU physical address
 * space and the bus address space.
 *
 * DMA_MAPPING_ERROR is the magic error code if a mapping failed.  It should not
 * be used directly in drivers, but checked for using dma_mapping_error()
 * instead.
 */
pub const DMA_MAPPING_ERROR: dma_addr_t = !(0 as dma_addr_t);

pub const DMA_ATTR_CPU_CACHE_CLEAN: unsigned_long = 1 as unsigned_long << 11;
pub const DMA_ATTR_DEBUGGING_IGNORE_CACHELINES: u32 = 0;
