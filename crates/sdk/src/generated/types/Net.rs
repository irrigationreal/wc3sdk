// Generated types. Do not edit by hand.
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

#[repr(C)]
pub struct PlayerResult {
    pub _opaque: [u8; 1424],
}
pub const PlayerResult__SIZE: usize = 1424;
pub const PlayerResult__NAME: &str = "Net::PlayerResult";
pub const PlayerResult__m_accountToken__OFFSET: usize = 0;
pub const PlayerResult__m_playerData__OFFSET: usize = 4;
pub const PlayerResult__m_clanTag__OFFSET: usize = 80;
pub const PlayerResult__m_clanRank__OFFSET: usize = 84;
pub const PlayerResult__m_clanName__OFFSET: usize = 85;
pub const PlayerResult__m_gamesPlayed__OFFSET: usize = 104;
pub const PlayerResult__m_gamesPlayedLadder__OFFSET: usize = 108;
pub const PlayerResult__m_rating__OFFSET: usize = 112;
pub const PlayerResult__m_officialPlayerData__OFFSET: usize = 114;
pub const PlayerResult__m_addr__OFFSET: usize = 136;
pub const PlayerResult__m_cdKeyProduct__OFFSET: usize = 152;
pub const PlayerResult__m_cdKeySequence__OFFSET: usize = 156;
pub const PlayerResult__m_extraPlayerData__OFFSET: usize = 160;
pub const PlayerResult__m_resultCode__OFFSET: usize = 164;
pub const PlayerResult__m_resultFlags__OFFSET: usize = 168;
pub const PlayerResult__m_loadDurationSec__OFFSET: usize = 172;
pub const PlayerResult__m_playDurationSec__OFFSET: usize = 176;
pub const PlayerResult__m_score__OFFSET: usize = 180;
pub const PlayerResult__m_customGameResults__OFFSET: usize = 184;

pub mod NetClient {
    #[repr(C)]
    pub struct Game {
        pub _opaque: [u8; 984],
    }
    pub const Game__SIZE: usize = 984;
    pub const Game__NAME: &str = "Net::NetClient::Game";
    pub const Game__m_provider__OFFSET: usize = 32;
    pub const Game__m_gameData__OFFSET: usize = 40;
    pub const Game__m_playerList__OFFSET: usize = 416;
    pub const Game__m_startPlayerList__OFFSET: usize = 440;
    pub const Game__m_numPlayers__OFFSET: usize = 464;
    pub const Game__m_state__OFFSET: usize = 468;
    pub const Game__m_refCount__OFFSET: usize = 472;
    pub const Game__m_perfStats__OFFSET: usize = 476;
    pub const Game__m_gameSecret__OFFSET: usize = 484;
    pub const Game__m_context__OFFSET: usize = 488;
    pub const Game__m_router__OFFSET: usize = 496;
    pub const Game__m_trustedRouter__OFFSET: usize = 504;
    pub const Game__m_routerAddr__OFFSET: usize = 512;
    pub const Game__m_supressLocalPlayerLeave__OFFSET: usize = 528;
    pub const Game__m_spawnInfo__OFFSET: usize = 536;
    pub const Game__m_checkingRouter__OFFSET: usize = 544;
    pub const Game__m_routerDisconnectTime__OFFSET: usize = 548;
    pub const Game__m_abruptHandoffTime__OFFSET: usize = 552;
    pub const Game__m_destroyGameTime__OFFSET: usize = 556;
    pub const Game__m_pingTime__OFFSET: usize = 560;
    pub const Game__m_clientTickTime__OFFSET: usize = 564;
    pub const Game__m_turnsCritsect__OFFSET: usize = 568;
    pub const Game__m_turnsId__OFFSET: usize = 576;
    pub const Game__m_turnsHandoffId__OFFSET: usize = 580;
    pub const Game__m_turnsSyncId__OFFSET: usize = 584;
    pub const Game__m_turnsLatency__OFFSET: usize = 588;
    pub const Game__m_turnTime__OFFSET: usize = 592;
    pub const Game__m_turnUnresponsive__OFFSET: usize = 596;
    pub const Game__m_syncEventRecycler__OFFSET: usize = 600;
    pub const Game__m_syncEventId__OFFSET: usize = 648;
    pub const Game__m_incomingSyncEventId__OFFSET: usize = 652;
    pub const Game__m_incomingSyncEventList__OFFSET: usize = 656;
    pub const Game__m_outgoingTurnRecycler__OFFSET: usize = 688;
    pub const Game__m_outgoingTurnCommandTimer__OFFSET: usize = 728;
    pub const Game__m_outgoingTurnList__OFFSET: usize = 736;
    pub const Game__m_outgoingTurnUnsent__OFFSET: usize = 760;
    pub const Game__m_turnsSyncRecycler__OFFSET: usize = 768;
    pub const Game__m_turnsSyncList__OFFSET: usize = 808;
    pub const Game__m_suspended__OFFSET: usize = 840;
    pub const Game__m_turnsSuspendCount__OFFSET: usize = 844;
    pub const Game__m_host__OFFSET: usize = 848;
    pub const Game__m_isAppointedHost__OFFSET: usize = 849;
    pub const Game__m_port__OFFSET: usize = 850;
    pub const Game__m_shutdownEvent__OFFSET: usize = 856;
    pub const Game__m_trustedDisconnectEvent__OFFSET: usize = 864;
    pub const Game__m_handoffReadyList__OFFSET: usize = 872;
    pub const Game__m_handoffPlayerId__OFFSET: usize = 896;
    pub const Game__m_handoffFailedPlayerId__OFFSET: usize = 897;
    pub const Game__m_handoffFailedPlayers__OFFSET: usize = 904;
    pub const Game__m_handoffReady__OFFSET: usize = 928;
    pub const Game__m_syscmdCreateGame__OFFSET: usize = 936;
    pub const Game__m_syscmdJoinGame__OFFSET: usize = 936;
    pub const Game__m_fileId__OFFSET: usize = 944;
    pub const Game__m_fileInfoList__OFFSET: usize = 952;
    pub const Game__m_reconnecting__OFFSET: usize = 976;

    pub mod HandoffReady {
        #[repr(C)]
        pub struct Player {
            pub _opaque: [u8; 56],
        }
        pub const Player__SIZE: usize = 56;
        pub const Player__NAME: &str = "Net::NetClient::HandoffReady::Player";
        pub const Player__m_playerId__OFFSET: usize = 0;
        pub const Player__m_addr1__OFFSET: usize = 8;
        pub const Player__m_addr2__OFFSET: usize = 24;
        pub const Player__m_syncEventId__OFFSET: usize = 40;
        pub const Player__m_syncEventsQueued__OFFSET: usize = 44;
        pub const Player__m_ready__OFFSET: usize = 48;

    }

}

pub mod NetProvider {
    #[repr(C)]
    pub struct GameAd {
        pub _opaque: [u8; 552],
    }
    pub const GameAd__SIZE: usize = 552;
    pub const GameAd__NAME: &str = "Net::NetProvider::GameAd";
    pub const GameAd__m_id__OFFSET: usize = 16;
    pub const GameAd__m_time__OFFSET: usize = 20;
    pub const GameAd__m_flags__OFFSET: usize = 24;
    pub const GameAd__m_routerAdAddr__OFFSET: usize = 32;
    pub const GameAd__m_routerAddr__OFFSET: usize = 48;
    pub const GameAd__m_routerGameId__OFFSET: usize = 64;
    pub const GameAd__m_routerGameSecret__OFFSET: usize = 68;
    pub const GameAd__m_numPlayers__OFFSET: usize = 72;
    pub const GameAd__m_maxPlayers__OFFSET: usize = 76;
    pub const GameAd__m_creationTime__OFFSET: usize = 80;
    pub const GameAd__m_gameData__OFFSET: usize = 84;
    pub const GameAd__m_hostBattleTag__OFFSET: usize = 464;
    pub const GameAd__m_region__OFFSET: usize = 504;
    pub const GameAd__m_ping__OFFSET: usize = 544;
    pub const GameAd__m_pingNeedsUpdate__OFFSET: usize = 548;
    pub const GameAd__isFilteredOut__OFFSET: usize = 549;

}

pub mod NetProviderBNET {
    #[repr(C)]
    pub struct GameData {
        pub _opaque: [u8; 56],
    }
    pub const GameData__SIZE: usize = 56;
    pub const GameData__NAME: &str = "Net::NetProviderBNET::GameData";
    pub const GameData__m_gameId__OFFSET: usize = 0;
    pub const GameData__m_gatewayId__OFFSET: usize = 8;
    pub const GameData__m_publicKey__OFFSET: usize = 16;

}

pub mod TurnsSuspendList {
    #[repr(C)]
    pub struct Player {
        pub _opaque: [u8; 12],
    }
    pub const Player__SIZE: usize = 12;
    pub const Player__NAME: &str = "Net::TurnsSuspendList::Player";
    pub const Player__m_playerId__OFFSET: usize = 0;
    pub const Player__m_voteCount__OFFSET: usize = 1;
    pub const Player__m_dropTime__OFFSET: usize = 4;
    pub const Player__m_dropped__OFFSET: usize = 8;

}

