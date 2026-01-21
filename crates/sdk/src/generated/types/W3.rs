// Generated types. Do not edit by hand.
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub mod Net {
    pub mod Client {
        #[repr(C)]
        pub struct PlayerSlot {
            pub _opaque: [u8; 80],
        }
        pub const PlayerSlot__SIZE: usize = 80;
        pub const PlayerSlot__NAME: &str = "W3::Net::Client::PlayerSlot";
        pub const PlayerSlot__kIsComputerControlledFieldNumber__OFFSET: usize = 0;
        pub const PlayerSlot__kRaceFieldNumber__OFFSET: usize = 0;
        pub const PlayerSlot__kTeamFieldNumber__OFFSET: usize = 0;
        pub const PlayerSlot__kColorFieldNumber__OFFSET: usize = 0;
        pub const PlayerSlot__kHandicapFieldNumber__OFFSET: usize = 0;
        pub const PlayerSlot__kAiDifficultyFieldNumber__OFFSET: usize = 0;
        pub const PlayerSlot__kAllowedToChangeInfoFieldNumber__OFFSET: usize = 0;
        pub const PlayerSlot___unknown_fields___OFFSET: usize = 8;
        pub const PlayerSlot___has_bits___OFFSET: usize = 48;
        pub const PlayerSlot___cached_size___OFFSET: usize = 52;
        pub const PlayerSlot__race___OFFSET: usize = 56;
        pub const PlayerSlot__team___OFFSET: usize = 60;
        pub const PlayerSlot__color___OFFSET: usize = 64;
        pub const PlayerSlot__is_computer_controlled___OFFSET: usize = 68;
        pub const PlayerSlot__allowed_to_change_info___OFFSET: usize = 69;
        pub const PlayerSlot__handicap___OFFSET: usize = 72;
        pub const PlayerSlot__ai_difficulty___OFFSET: usize = 76;
        pub const PlayerSlot__default_instance___OFFSET: usize = 0;

        #[repr(C)]
        pub struct PlayerInfo {
            pub _opaque: [u8; 112],
        }
        pub const PlayerInfo__SIZE: usize = 112;
        pub const PlayerInfo__NAME: &str = "W3::Net::Client::PlayerInfo";
        pub const PlayerInfo__kPlayerIdFieldNumber__OFFSET: usize = 0;
        pub const PlayerInfo__kPlayerNameFieldNumber__OFFSET: usize = 0;
        pub const PlayerInfo__kPlayerClanFieldNumber__OFFSET: usize = 0;
        pub const PlayerInfo__kPortraitIdFieldNumber__OFFSET: usize = 0;
        pub const PlayerInfo__kGatewayIdFieldNumber__OFFSET: usize = 0;
        pub const PlayerInfo__kQueuedRaceFieldNumber__OFFSET: usize = 0;
        pub const PlayerInfo__kMmrFieldNumber__OFFSET: usize = 0;
        pub const PlayerInfo__kXpFieldNumber__OFFSET: usize = 0;
        pub const PlayerInfo___unknown_fields___OFFSET: usize = 8;
        pub const PlayerInfo___has_bits___OFFSET: usize = 48;
        pub const PlayerInfo___cached_size___OFFSET: usize = 52;
        pub const PlayerInfo__player_name___OFFSET: usize = 56;
        pub const PlayerInfo__player_clan___OFFSET: usize = 64;
        pub const PlayerInfo__player_id___OFFSET: usize = 72;
        pub const PlayerInfo__gateway_id___OFFSET: usize = 76;
        pub const PlayerInfo__portrait_id___OFFSET: usize = 80;
        pub const PlayerInfo__queued_race___OFFSET: usize = 88;
        pub const PlayerInfo__mmr___OFFSET: usize = 96;
        pub const PlayerInfo__xp___OFFSET: usize = 104;
        pub const PlayerInfo__default_instance___OFFSET: usize = 0;

        #[repr(C)]
        pub struct PlayerProfileInfo {
            pub _opaque: [u8; 80],
        }
        pub const PlayerProfileInfo__SIZE: usize = 80;
        pub const PlayerProfileInfo__NAME: &str = "W3::Net::Client::PlayerProfileInfo";
        pub const PlayerProfileInfo__kPlayerInfoFieldNumber__OFFSET: usize = 0;
        pub const PlayerProfileInfo___unknown_fields___OFFSET: usize = 8;
        pub const PlayerProfileInfo___has_bits___OFFSET: usize = 48;
        pub const PlayerProfileInfo___cached_size___OFFSET: usize = 52;
        pub const PlayerProfileInfo__player_info___OFFSET: usize = 56;
        pub const PlayerProfileInfo__default_instance___OFFSET: usize = 0;

    }

}

