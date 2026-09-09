/*
 * linux/membarrier.h
 *
 * membarrier system call API
 *
 * Copyright (c) 2010, 2015 Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// C enum membarrier_cmd translated as integer constants.
pub type MembarrierCmd = i32;

pub const MEMBARRIER_CMD_QUERY: MembarrierCmd = 0;
pub const MEMBARRIER_CMD_GLOBAL: MembarrierCmd = 1 << 0;
pub const MEMBARRIER_CMD_GLOBAL_EXPEDITED: MembarrierCmd = 1 << 1;
pub const MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED: MembarrierCmd = 1 << 2;
pub const MEMBARRIER_CMD_PRIVATE_EXPEDITED: MembarrierCmd = 1 << 3;
pub const MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED: MembarrierCmd = 1 << 4;
pub const MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE: MembarrierCmd = 1 << 5;
pub const MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE: MembarrierCmd = 1 << 6;
pub const MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ: MembarrierCmd = 1 << 7;
pub const MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ: MembarrierCmd = 1 << 8;
pub const MEMBARRIER_CMD_GET_REGISTRATIONS: MembarrierCmd = 1 << 9;

/* Alias for header backward compatibility. */
pub const MEMBARRIER_CMD_SHARED: MembarrierCmd = MEMBARRIER_CMD_GLOBAL;

pub type MembarrierCmdFlag = i32;

pub const MEMBARRIER_CMD_FLAG_CPU: MembarrierCmdFlag = 1 << 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
