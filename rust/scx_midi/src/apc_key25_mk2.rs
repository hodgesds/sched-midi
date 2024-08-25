// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

// Copied from https://cdn.inmusicbrands.com/akai/attachments/APC%20Key%2025%20mk2%20-%20Communication%20Protocol%20-%20v1.1.pdf

// use crate::{RGBLEDButton, LEDButton};

pub const PAD_MATRIX: [[u8; 8];5] = [
    [0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27],
    [0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F],
    [0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17],
    [0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F],
    [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
];

pub const LED_VEL_OFF: u8 = 0x00;
pub const LED_VEL_ON: u8 = 0x01;
pub const LED_VEL_BLINK: u8 = 0x02;

pub const LED_VEL_10_PCT_BRIGHT: u8 = 0x90;
pub const LED_VEL_25_PCT_BRIGHT: u8 = 0x91;
pub const LED_VEL_50_PCT_BRIGHT: u8 = 0x92;
pub const LED_VEL_65_PCT_BRIGHT: u8 = 0x93;
pub const LED_VEL_75_PCT_BRIGHT: u8 = 0x94;
pub const LED_VEL_90_PCT_BRIGHT: u8 = 0x95;
pub const LED_VEL_100_PCT_BRIGHT: u8 = 0x96;
pub const LED_VEL_PULSE_1_16: u8 = 0x97;
pub const LED_VEL_PULSE_1_8: u8 = 0x98;
pub const LED_VEL_PULSE_1_4: u8 = 0x99;
pub const LED_VEL_PULSE_1_2: u8 = 0x9A;
pub const LED_VEL_BLINK_1_24: u8 = 0x9B;
pub const LED_VEL_BLINK_1_16: u8 = 0x9C;
pub const LED_VEL_BLINK_1_8: u8 = 0x9D;
pub const LED_VEL_BLINK_1_4: u8 = 0x9E;
pub const LED_VEL_BLINK_1_2: u8 = 0x9F;


// MIDI, Channel, Byte 1 Value, Port, Function
// 0 90 0 On 10% brightness
// 1 91 0 On 25% brightness
// 2 92 0 On 50% brightness
// 3 93 0 On 65% brightness
// 4 94 0 On 75% brightness
// 5 95 0 On 90% brightness
// 6 96 0 On 100% brightness
// 7 97 0 Pulsing 1/16
// 8 98 0 Pulsing 1/8
// 9 99 0 Pulsing 1/4
// 10 9A 0 Pulsing 1/2
// 11 9B 0 Blinking 1/24
// 12 9C 0 Blinking 1/16
// 13 9D 0 Blinking 1/8
// 14 9E 0 Blinking 1/4
// 15 9F 0 Blinking 1/2


// UI Buttons
// Button Name Note # Channel LED Port #
// Track Button 1 0x40 0 Red 1
// Track Button 2 0x41 0 Red 1
// Track Button 3 0x42 0 Red 1
// Track Button 4 0x43 0 Red 1
// Track Button 5 0x44 0 Red 1
// Track Button 6 0x45 0 Red 1
// Track Button 7 0x46 0 Red 1
// Track Button 8 0x47 0 Red 1
// Scene Launch 1 0x52 0 Green 1
// Scene Launch 2 0x53 0 Green 1
// Scene Launch 3 0x54 0 Green 1
// Scene Launch 4 0x55 0 Green 1
// Scene Launch 5 0x56 0 Green 1
// Stop All Clips 0x51 0 None 1
// Play 0x5b 0 Green 1
// Record 0x5d 0 Red 1

pub const UI_BUT_TRACK_1: u8 = 0x40;
pub const UI_BUT_TRACK_2: u8 = 0x41;
pub const UI_BUT_TRACK_3: u8 = 0x42;
pub const UI_BUT_TRACK_4: u8 = 0x43;
pub const UI_BUT_TRACK_5: u8 = 0x44;
pub const UI_BUT_TRACK_6: u8 = 0x45;
pub const UI_BUT_TRACK_7: u8 = 0x46;
pub const UI_BUT_TRACK_8: u8 = 0x47;
pub const UI_BUT_STOP_CLIPS: u8 = 0x51;
pub const UI_BUT_SCENE_LAUNCH_1: u8 = 0x52;
pub const UI_BUT_SCENE_LAUNCH_2: u8 = 0x53;
pub const UI_BUT_SCENE_LAUNCH_3: u8 = 0x54;
pub const UI_BUT_SCENE_LAUNCH_4: u8 = 0x55;
pub const UI_BUT_SCENE_LAUNCH_5: u8 = 0x56;
pub const UI_BUT_PLAY: u8 = 0x5b;
pub const UI_BUT_RECORD: u8 = 0x5d;

// Byte number Value Description
// 1 0x90 MIDI CH Note-On
// 2 <Button Value> 0x40-0x5B
// *See Pad/Button Values above
// 3 <Velocity> Used to determine LED behavior as follows:
// LED Off = 0x00
// LED On = 0x01, 0x03-0x7F
// LED Blink = 0x02 


// Button Name Note # Channel LED Port # Notes
// Track Button 1 0x40 0 Red 1
// Track Button 2 0x41 0 Red 1
// Track Button 3 0x42 0 Red 1
// Track Button 4 0x43 0 Red 1
// Track Button 5 0x44 0 Red 1
// Track Button 6 0x45 0 Red 1
// Track Button 7 0x46 0 Red 1
// Track Button 8 0x47 0 Red 1
// Scene Launch 1 0x52 0 Green 1
// Scene Launch 2 0x53 0 Green 1
// Scene Launch 3 0x54 0 Green 1
// Scene Launch 4 0x55 0 Green 1
// Scene Launch 5 0x56 0 Green 1
// Stop All Clips 0x51 0 None 1
// Play 0x5b 0 None 1
// Record 0x5d 0 None 1
// Shift 0x62 0 None 1
// Oct Down - - - - Transposes keybed by an octave. Does
// not send Note On/Off.
// Oct Up - - - - Transposes keybed by an octave. Does
// not send Note On/Off
// Clip Launch
// Button 0-39
// 0x00 –
// 0x27 See notes RGB 1
// Numbered bottom left to upper right.
// MIDI Channel is used to determine LED
// illumination Status on CH 00-0F.
// See section on RGB LED Behavior.
// Sustain 0x40 0 None 0 Midi CC# message
// Keybed 0x00 –
// 0x7f 0 None 0 Full range accessible using Oct Down /
// Oct Up buttons

// Control Name CC# Channel Port Notes
// Knob 1 0x30 0 1 Relative
// Knob 2 0x31 0 1 Relative
// Knob 3 0x32 0 1 Relative
// Knob 4 0x33 0 1 Relative
// Knob 5 0x34 0 1 Relative
// Knob 6 0x35 0 1 Relative
// Knob 7 0x36 0 1 Relative
// Knob 8 0x37 0 1 Relative 

pub const KNOB_1: u8 = 0x30;
pub const KNOB_2: u8 = 0x31;
pub const KNOB_3: u8 = 0x32;
pub const KNOB_4: u8 = 0x33;
pub const KNOB_5: u8 = 0x34;
pub const KNOB_6: u8 = 0x35;
pub const KNOB_7: u8 = 0x36;
pub const KNOB_8: u8 = 0x37;


// #000000 0 #142B00 19 #004152 38
// #1E1E1E 1 #4CFF4C 20 #001019 39
// #7F7F7F 2 #00FF00 21 #4C88FF 40
// #FFFFFF 3 #005900 22 #0055FF 41
// #FF4C4C 4 #001900 23 #001D59 42
// #FF0000 5 #4CFF5E 24 #000819 43
// #590000 6 #00FF19 25 #4C4CFF 44
// #190000 7 #00590D 26 #0000FF 45
// #FFBD6C 8 #001902 27 #000059 46
// #FF5400 9 #4CFF88 28 #000019 47
// #591D00 10 #00FF55 29 #874CFF 48
// #271B00 11 #00591D 30 #5400FF 49
// #FFFF4C 12 #001F12 31 #190064 50
// #FFFF00 13 #4CFFB7 32 #0F0030 51
// #595900 14 #00FF99 33 #FF4CFF 52
// #191900 15 #005935 34 #FF00FF 53
// #88FF4C 16 #001912 35 #590059 54
// #54FF00 17 #4CC3FF 36 #190019 55
// #1D5900 18 #00A9FF 37 #FF4C87 56 
