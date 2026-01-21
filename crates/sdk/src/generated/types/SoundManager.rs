// Generated types. Do not edit by hand.
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub mod Events {
    #[repr(C)]
    pub struct UnitDialogueCallbackData {
        pub _opaque: [u8; 136],
    }
    pub const UnitDialogueCallbackData__SIZE: usize = 136;
    pub const UnitDialogueCallbackData__NAME: &str = "SoundManager::Events::UnitDialogueCallbackData";
    pub const UnitDialogueCallbackData__unit__OFFSET: usize = 0;
    pub const UnitDialogueCallbackData__soundAssetEntry__OFFSET: usize = 8;
    pub const UnitDialogueCallbackData__durationInSeconds__OFFSET: usize = 24;
    pub const UnitDialogueCallbackData__animationSetScope__OFFSET: usize = 28;
    pub const UnitDialogueCallbackData__animIndex__OFFSET: usize = 32;
    pub const UnitDialogueCallbackData__speakerName__OFFSET: usize = 40;
    pub const UnitDialogueCallbackData__dialogueText__OFFSET: usize = 80;
    pub const UnitDialogueCallbackData__dialogueAudioInstance__OFFSET: usize = 120;

}

