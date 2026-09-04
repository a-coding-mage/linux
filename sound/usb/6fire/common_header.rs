// SPDX-License-Identifier: GPL-2.0-or-later
//
// Linux driver for TerraTec DMX 6Fire USB
//
// Author:	Torsten Schenk <torsten.schenk@zoho.com>
// Created:	Jan 01, 2011
// Copyright:	(C) Torsten Schenk

// Requires: linux/slab.h, linux/usb.h, sound/core.h (external dependencies)

pub const PREFIX: &str = "6fire: ";

// Forward declarations of structs defined in other modules
pub struct SfireChip;
pub struct MidiRuntime;
pub struct PcmRuntime;
pub struct ControlRuntime;
pub struct CommRuntime;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
