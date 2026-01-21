// Generated types. Do not edit by hand.
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

#[repr(C)]
pub struct SelectionSegment {
    pub _opaque: [u8; 32],
}
pub const SelectionSegment__SIZE: usize = 32;
pub const SelectionSegment__NAME: &str = "Scintilla::SelectionSegment";
pub const SelectionSegment__start__OFFSET: usize = 0;
pub const SelectionSegment__end__OFFSET: usize = 16;

#[repr(C)]
pub struct SelectionPosition {
    pub _opaque: [u8; 16],
}
pub const SelectionPosition__SIZE: usize = 16;
pub const SelectionPosition__NAME: &str = "Scintilla::SelectionPosition";
pub const SelectionPosition__position__OFFSET: usize = 0;
pub const SelectionPosition__virtualSpace__OFFSET: usize = 8;

#[repr(C)]
pub struct Selection {
    pub _opaque: [u8; 96],
}
pub const Selection__SIZE: usize = 96;
pub const Selection__NAME: &str = "Scintilla::Selection";
pub const Selection__ranges__OFFSET: usize = 0;
pub const Selection__rangesSaved__OFFSET: usize = 24;
pub const Selection__rangeRectangular__OFFSET: usize = 48;
pub const Selection__mainRange__OFFSET: usize = 80;
pub const Selection__moveExtends__OFFSET: usize = 88;
pub const Selection__tentativeMain__OFFSET: usize = 89;
pub const Selection__selType__OFFSET: usize = 92;

#[repr(C)]
pub struct Timer {
    pub _opaque: [u8; 16],
}
pub const Timer__SIZE: usize = 16;
pub const Timer__NAME: &str = "Scintilla::Timer";
pub const Timer__ticking__OFFSET: usize = 0;
pub const Timer__ticksToWait__OFFSET: usize = 4;
pub const Timer__tickerID__OFFSET: usize = 8;

#[repr(C)]
pub struct SelectionText {
    pub _opaque: [u8; 48],
}
pub const SelectionText__SIZE: usize = 48;
pub const SelectionText__NAME: &str = "Scintilla::SelectionText";
pub const SelectionText__s__OFFSET: usize = 0;
pub const SelectionText__rectangular__OFFSET: usize = 32;
pub const SelectionText__lineCopy__OFFSET: usize = 33;
pub const SelectionText__codePage__OFFSET: usize = 36;
pub const SelectionText__characterSet__OFFSET: usize = 40;

#[repr(C)]
pub struct SelectionRange {
    pub _opaque: [u8; 32],
}
pub const SelectionRange__SIZE: usize = 32;
pub const SelectionRange__NAME: &str = "Scintilla::SelectionRange";
pub const SelectionRange__caret__OFFSET: usize = 0;
pub const SelectionRange__anchor__OFFSET: usize = 16;

