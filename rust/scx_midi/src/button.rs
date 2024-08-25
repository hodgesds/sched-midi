// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

pub trait Button {
    fn name(&self) -> &str;
    fn value(&self) -> u8;
}

pub trait LEDButton {
    type Button;

    fn toggle(&mut self) -> bool;
}

pub trait RGBLEDButton {
    type Button;

    fn set(&mut self, color: u8, velocity: u8) -> bool;
}

