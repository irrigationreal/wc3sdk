// Generated types. Do not edit by hand.
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub mod Telemetry {
    pub mod W3 {
        pub mod Client {
            #[repr(C)]
            pub struct Game {
                pub _opaque: [u8; 128],
            }
            pub const Game__SIZE: usize = 128;
            pub const Game__NAME: &str = "Blizzard::Telemetry::W3::Client::Game";
            pub const Game__NONE__OFFSET: usize = 0;
            pub const Game__MELEE__OFFSET: usize = 0;
            pub const Game__FFA__OFFSET: usize = 0;
            pub const Game__ONE_ON_ONE__OFFSET: usize = 0;
            pub const Game__TWO_TEAM_PLAY__OFFSET: usize = 0;
            pub const Game__THREE_TEAM_PLAY__OFFSET: usize = 0;
            pub const Game__FOUR_TEAM_PLAY__OFFSET: usize = 0;
            pub const Game__Type_MIN__OFFSET: usize = 0;
            pub const Game__Type_MAX__OFFSET: usize = 0;
            pub const Game__Type_ARRAYSIZE__OFFSET: usize = 0;
            pub const Game__OFFLINE__OFFSET: usize = 0;
            pub const Game__LAN__OFFSET: usize = 0;
            pub const Game__BNET__OFFSET: usize = 0;
            pub const Game__NetworkType_MIN__OFFSET: usize = 0;
            pub const Game__NetworkType_MAX__OFFSET: usize = 0;
            pub const Game__NetworkType_ARRAYSIZE__OFFSET: usize = 0;
            pub const Game__kMapFileNameFieldNumber__OFFSET: usize = 0;
            pub const Game__kTypeFieldNumber__OFFSET: usize = 0;
            pub const Game__kIsCampaignFieldNumber__OFFSET: usize = 0;
            pub const Game__kIsReplayFieldNumber__OFFSET: usize = 0;
            pub const Game__kIsTournamentFieldNumber__OFFSET: usize = 0;
            pub const Game__kNetworkTypeFieldNumber__OFFSET: usize = 0;
            pub const Game__kNumPlayersFieldNumber__OFFSET: usize = 0;
            pub const Game__kIsBlizzardMapFieldNumber__OFFSET: usize = 0;
            pub const Game__kLoadingTimeSecFieldNumber__OFFSET: usize = 0;
            pub const Game__kMatchGuidFieldNumber__OFFSET: usize = 0;
            pub const Game__kToonNameFieldNumber__OFFSET: usize = 0;
            pub const Game__kGatewayIdFieldNumber__OFFSET: usize = 0;
            pub const Game__kDataVariationFieldNumber__OFFSET: usize = 0;
            pub const Game__kIsHdAllowedFieldNumber__OFFSET: usize = 0;
            pub const Game__kHasRequiredHdHardwareFieldNumber__OFFSET: usize = 0;
            pub const Game__kUserPrefersHdFieldNumber__OFFSET: usize = 0;
            pub const Game__kMapAllowsSdFieldNumber__OFFSET: usize = 0;
            pub const Game__kMapAllowsHdFieldNumber__OFFSET: usize = 0;
            pub const Game___unknown_fields___OFFSET: usize = 8;
            pub const Game___has_bits___OFFSET: usize = 48;
            pub const Game___cached_size___OFFSET: usize = 52;
            pub const Game__map_file_name___OFFSET: usize = 56;
            pub const Game__type___OFFSET: usize = 64;
            pub const Game__is_campaign___OFFSET: usize = 68;
            pub const Game__is_replay___OFFSET: usize = 69;
            pub const Game__is_tournament___OFFSET: usize = 70;
            pub const Game__is_blizzard_map___OFFSET: usize = 71;
            pub const Game__network_type___OFFSET: usize = 72;
            pub const Game__num_players___OFFSET: usize = 76;
            pub const Game__matchguid___OFFSET: usize = 80;
            pub const Game__toonname___OFFSET: usize = 88;
            pub const Game__gatewayid___OFFSET: usize = 96;
            pub const Game__data_variation___OFFSET: usize = 104;
            pub const Game__loadingtimesec___OFFSET: usize = 112;
            pub const Game__is_hd_allowed___OFFSET: usize = 116;
            pub const Game__has_required_hd_hardware___OFFSET: usize = 117;
            pub const Game__user_prefers_hd___OFFSET: usize = 118;
            pub const Game__map_allows_sd___OFFSET: usize = 119;
            pub const Game__map_allows_hd___OFFSET: usize = 120;
            pub const Game__default_instance___OFFSET: usize = 0;

            #[repr(C)]
            pub struct RegionCompliancePrompt {
                pub _opaque: [u8; 72],
            }
            pub const RegionCompliancePrompt__SIZE: usize = 72;
            pub const RegionCompliancePrompt__NAME: &str = "Blizzard::Telemetry::W3::Client::RegionCompliancePrompt";
            pub const RegionCompliancePrompt__kRegionComplianceAcceptedFieldNumber__OFFSET: usize = 0;
            pub const RegionCompliancePrompt__kToonNameFieldNumber__OFFSET: usize = 0;
            pub const RegionCompliancePrompt__kGatewayIdFieldNumber__OFFSET: usize = 0;
            pub const RegionCompliancePrompt___unknown_fields___OFFSET: usize = 8;
            pub const RegionCompliancePrompt___has_bits___OFFSET: usize = 48;
            pub const RegionCompliancePrompt___cached_size___OFFSET: usize = 52;
            pub const RegionCompliancePrompt__toon_name___OFFSET: usize = 56;
            pub const RegionCompliancePrompt__region_compliance_accepted___OFFSET: usize = 64;
            pub const RegionCompliancePrompt__gateway_id___OFFSET: usize = 68;
            pub const RegionCompliancePrompt__default_instance___OFFSET: usize = 0;

            #[repr(C)]
            pub struct GamePlayTime {
                pub _opaque: [u8; 112],
            }
            pub const GamePlayTime__SIZE: usize = 112;
            pub const GamePlayTime__NAME: &str = "Blizzard::Telemetry::W3::Client::GamePlayTime";
            pub const GamePlayTime__NONE__OFFSET: usize = 0;
            pub const GamePlayTime__MELEE__OFFSET: usize = 0;
            pub const GamePlayTime__FFA__OFFSET: usize = 0;
            pub const GamePlayTime__ONE_ON_ONE__OFFSET: usize = 0;
            pub const GamePlayTime__TWO_TEAM_PLAY__OFFSET: usize = 0;
            pub const GamePlayTime__THREE_TEAM_PLAY__OFFSET: usize = 0;
            pub const GamePlayTime__FOUR_TEAM_PLAY__OFFSET: usize = 0;
            pub const GamePlayTime__Type_MIN__OFFSET: usize = 0;
            pub const GamePlayTime__Type_MAX__OFFSET: usize = 0;
            pub const GamePlayTime__Type_ARRAYSIZE__OFFSET: usize = 0;
            pub const GamePlayTime__OFFLINE__OFFSET: usize = 0;
            pub const GamePlayTime__LAN__OFFSET: usize = 0;
            pub const GamePlayTime__BNET__OFFSET: usize = 0;
            pub const GamePlayTime__NetworkType_MIN__OFFSET: usize = 0;
            pub const GamePlayTime__NetworkType_MAX__OFFSET: usize = 0;
            pub const GamePlayTime__NetworkType_ARRAYSIZE__OFFSET: usize = 0;
            pub const GamePlayTime__kMapFileNameFieldNumber__OFFSET: usize = 0;
            pub const GamePlayTime__kTypeFieldNumber__OFFSET: usize = 0;
            pub const GamePlayTime__kIsCampaignFieldNumber__OFFSET: usize = 0;
            pub const GamePlayTime__kIsReplayFieldNumber__OFFSET: usize = 0;
            pub const GamePlayTime__kIsTournamentFieldNumber__OFFSET: usize = 0;
            pub const GamePlayTime__kNetworkTypeFieldNumber__OFFSET: usize = 0;
            pub const GamePlayTime__kNumPlayersFieldNumber__OFFSET: usize = 0;
            pub const GamePlayTime__kIsBlizzardMapFieldNumber__OFFSET: usize = 0;
            pub const GamePlayTime__kPlayTimeSecFieldNumber__OFFSET: usize = 0;
            pub const GamePlayTime__kMatchGuidFieldNumber__OFFSET: usize = 0;
            pub const GamePlayTime__kToonNameFieldNumber__OFFSET: usize = 0;
            pub const GamePlayTime__kGatewayIdFieldNumber__OFFSET: usize = 0;
            pub const GamePlayTime___unknown_fields___OFFSET: usize = 8;
            pub const GamePlayTime___has_bits___OFFSET: usize = 48;
            pub const GamePlayTime___cached_size___OFFSET: usize = 52;
            pub const GamePlayTime__map_file_name___OFFSET: usize = 56;
            pub const GamePlayTime__type___OFFSET: usize = 64;
            pub const GamePlayTime__is_campaign___OFFSET: usize = 68;
            pub const GamePlayTime__is_replay___OFFSET: usize = 69;
            pub const GamePlayTime__is_tournament___OFFSET: usize = 70;
            pub const GamePlayTime__is_blizzard_map___OFFSET: usize = 71;
            pub const GamePlayTime__network_type___OFFSET: usize = 72;
            pub const GamePlayTime__num_players___OFFSET: usize = 76;
            pub const GamePlayTime__matchguid___OFFSET: usize = 80;
            pub const GamePlayTime__toonname___OFFSET: usize = 88;
            pub const GamePlayTime__gatewayid___OFFSET: usize = 96;
            pub const GamePlayTime__playtimesec___OFFSET: usize = 104;
            pub const GamePlayTime__default_instance___OFFSET: usize = 0;

        }

    }

}

