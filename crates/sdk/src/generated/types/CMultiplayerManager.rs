// Generated types. Do not edit by hand.
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

#[repr(C)]
pub struct PlayerData {
    pub _opaque: [u8; 232],
}
pub const PlayerData__SIZE: usize = 232;
pub const PlayerData__NAME: &str = "CMultiplayerManager::PlayerData";
pub const PlayerData__battletag__OFFSET: usize = 0;
pub const PlayerData__clanName__OFFSET: usize = 40;
pub const PlayerData__avatarId__OFFSET: usize = 80;
pub const PlayerData__gatewayId__OFFSET: usize = 120;
pub const PlayerData__playerRegion__OFFSET: usize = 128;
pub const PlayerData__encounterTime__OFFSET: usize = 168;
pub const PlayerData__playerId__OFFSET: usize = 172;
pub const PlayerData__playerSelectedRace__OFFSET: usize = 176;
pub const PlayerData__mmr__OFFSET: usize = 216;
pub const PlayerData__xp__OFFSET: usize = 224;

#[repr(C)]
pub struct PlayerHistory {
    pub _opaque: [u8; 64],
}
pub const PlayerHistory__SIZE: usize = 64;
pub const PlayerHistory__NAME: &str = "CMultiplayerManager::PlayerHistory";
pub const PlayerHistory__m_recentPlayersMaxSize__OFFSET: usize = 24;
pub const PlayerHistory__m_recentPlayers__OFFSET: usize = 32;
pub const PlayerHistory__m_recentGameId__OFFSET: usize = 56;

