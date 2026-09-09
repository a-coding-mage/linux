/* SPDX-License-Identifier: GPL-2.0 */

// Rust translation of trace/events/spi.h.
// The Linux tracepoint DSL declarations below are retained as documentation
// of the externally visible trace events; their implementation is supplied by
// the tracepoint subsystem.

// TRACE_SYSTEM: spi
// Dependencies: linux/ktime.h, linux/tracepoint.h, and the SPI kernel types.

// DECLARE_EVENT_CLASS(spi_controller,
//     TP_PROTO(struct spi_controller *controller),
//     TP_ARGS(controller),
//     TP_STRUCT__entry(__field(int, bus_num)),
//     TP_fast_assign(__entry->bus_num = controller->bus_num;),
//     TP_printk("spi%d", (int)__entry->bus_num));

// DEFINE_EVENT(spi_controller, spi_controller_idle,
//     TP_PROTO(struct spi_controller *controller), TP_ARGS(controller));
// DEFINE_EVENT(spi_controller, spi_controller_busy,
//     TP_PROTO(struct spi_controller *controller), TP_ARGS(controller));

// TRACE_EVENT(spi_setup,
//     TP_PROTO(struct spi_device *spi, int status),
//     TP_ARGS(spi, status),
//     TP_STRUCT__entry(
//         __field(int, bus_num)
//         __field(int, chip_select)
//         __field(unsigned long, mode)
//         __field(unsigned int, bits_per_word)
//         __field(unsigned int, max_speed_hz)
//         __field(int, status)),
//     TP_fast_assign(
//         __entry->bus_num = spi->controller->bus_num;
//         __entry->chip_select = spi_get_chipselect(spi, 0);
//         __entry->mode = spi->mode;
//         __entry->bits_per_word = spi->bits_per_word;
//         __entry->max_speed_hz = spi->max_speed_hz;
//         __entry->status = status;));

// TRACE_EVENT(spi_set_cs,
//     TP_PROTO(struct spi_device *spi, bool enable), TP_ARGS(spi, enable),
//     TP_STRUCT__entry(
//         __field(int, bus_num) __field(int, chip_select)
//         __field(unsigned long, mode) __field(bool, enable)),
//     TP_fast_assign(
//         __entry->bus_num = spi->controller->bus_num;
//         __entry->chip_select = spi_get_chipselect(spi, 0);
//         __entry->mode = spi->mode; __entry->enable = enable;));

// DECLARE_EVENT_CLASS(spi_message,
//     TP_PROTO(struct spi_message *msg), TP_ARGS(msg),
//     TP_STRUCT__entry(__field(int, bus_num) __field(int, chip_select)
//                      __field(struct spi_message *, msg)),
//     TP_fast_assign(
//         __entry->bus_num = msg->spi->controller->bus_num;
//         __entry->chip_select = spi_get_chipselect(msg->spi, 0);
//         __entry->msg = msg;));
// DEFINE_EVENT(spi_message, spi_message_submit,
//     TP_PROTO(struct spi_message *msg), TP_ARGS(msg));
// DEFINE_EVENT(spi_message, spi_message_start,
//     TP_PROTO(struct spi_message *msg), TP_ARGS(msg));

// TRACE_EVENT(spi_message_done) has fields bus_num, chip_select, msg, frame,
// and actual, assigned from msg->spi->controller, msg, frame_length, and
// actual_length respectively.

/*
 * Consider a buffer valid if non-NULL and if it doesn't match the dummy buffer
 * that only exist to work with controllers that have SPI_CONTROLLER_MUST_TX or
 * SPI_CONTROLLER_MUST_RX.
 */

/// Equivalent of `spi_valid_txbuf(msg, xfer)`.
#[macro_export]
macro_rules! spi_valid_txbuf {
    ($msg:expr, $xfer:expr) => {
        !$xfer.tx_buf.is_null() && $xfer.tx_buf != $msg.spi.controller.dummy_tx
    };
}

/// Equivalent of `spi_valid_rxbuf(msg, xfer)`.
#[macro_export]
macro_rules! spi_valid_rxbuf {
    ($msg:expr, $xfer:expr) => {
        !$xfer.rx_buf.is_null() && $xfer.rx_buf != $msg.spi.controller.dummy_rx
    };
}

// DECLARE_EVENT_CLASS(spi_transfer) captures bus_num, chip_select, xfer, len,
// and up to 64 bytes each from valid tx_buf and rx_buf, using memcpy.
// DEFINE_EVENT(spi_transfer, spi_transfer_start,
//     TP_PROTO(struct spi_message *msg, struct spi_transfer *xfer),
//     TP_ARGS(msg, xfer));
// DEFINE_EVENT(spi_transfer, spi_transfer_stop,
//     TP_PROTO(struct spi_message *msg, struct spi_transfer *xfer),
//     TP_ARGS(msg, xfer));

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
