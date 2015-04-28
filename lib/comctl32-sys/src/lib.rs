// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to comctl32.
extern crate winapi;
use winapi::*;
use winapi::commctrl::*;
extern "system" {
	pub fn InitCommonControls();
	pub fn InitCommonControlsEx(lpInitCtrls: *const INITCOMMONCONTROLSEX) -> BOOL;
}
