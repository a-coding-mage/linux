/* Translation of trace/events/tegra_apb_dma.h. */

/* The following types and tracepoint primitives are supplied by the Linux
 * tracepoint and dmaengine dependencies. */

#[repr(C)]
pub struct TegraDmaTxStatusEntry {
    pub chan: *const core::ffi::c_char,
    pub cookie: dma_cookie_t,
    pub residue: u32,
}

#[repr(C)]
pub struct TegraDmaCompleteCbEntry {
    pub chan: *const core::ffi::c_char,
    pub count: core::ffi::c_int,
    pub ptr: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct TegraDmaIsrEntry {
    pub chan: *const core::ffi::c_char,
    pub irq: core::ffi::c_int,
}

/*
 * TRACE_EVENT(tegra_dma_tx_status,
 *     TP_PROTO(struct dma_chan *dc, dma_cookie_t cookie,
 *              struct dma_tx_state *state),
 *     TP_ARGS(dc, cookie, state),
 *     TP_STRUCT__entry(
 *         __string(chan, dev_name(&dc->dev->device))
 *         __field(dma_cookie_t, cookie)
 *         __field(__u32, residue)
 *     ),
 *     TP_fast_assign(
 *         __assign_str(chan);
 *         __entry->cookie = cookie;
 *         __entry->residue = state ? state->residue : (u32)-1;
 *     ),
 *     TP_printk("channel %s: dma cookie %d, residue %u",
 *               __get_str(chan), __entry->cookie, __entry->residue)
 * );
 */

/*
 * TRACE_EVENT(tegra_dma_complete_cb,
 *     TP_PROTO(struct dma_chan *dc, int count, void *ptr),
 *     TP_ARGS(dc, count, ptr),
 *     TP_STRUCT__entry(
 *         __string(chan, dev_name(&dc->dev->device))
 *         __field(int, count)
 *         __field(void *, ptr)
 *     ),
 *     TP_fast_assign(
 *         __assign_str(chan);
 *         __entry->count = count;
 *         __entry->ptr = ptr;
 *     ),
 *     TP_printk("channel %s: done %d, ptr %p",
 *               __get_str(chan), __entry->count, __entry->ptr)
 * );
 */

/*
 * TRACE_EVENT(tegra_dma_isr,
 *     TP_PROTO(struct dma_chan *dc, int irq),
 *     TP_ARGS(dc, irq),
 *     TP_STRUCT__entry(
 *         __string(chan, dev_name(&dc->dev->device))
 *         __field(int, irq)
 *     ),
 *     TP_fast_assign(
 *         __assign_str(chan);
 *         __entry->irq = irq;
 *     ),
 *     TP_printk("%s: irq %d\\n", __get_str(chan), __entry->irq)
 * );
 */

/* The C header includes trace/define_trace.h outside its include guard. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
