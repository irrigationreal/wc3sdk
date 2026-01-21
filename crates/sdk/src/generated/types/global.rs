// Generated types. Do not edit by hand.
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

#[repr(C)]
pub struct UnitProperties {
    pub _opaque: [u8; 168],
}
pub const UnitProperties__SIZE: usize = 168;
pub const UnitProperties__NAME: &str = "UnitProperties";
pub const UnitProperties__owner__OFFSET: usize = 0;
pub const UnitProperties__flags__OFFSET: usize = 4;
pub const UnitProperties__life__OFFSET: usize = 8;
pub const UnitProperties__mana__OFFSET: usize = 12;
pub const UnitProperties__level__OFFSET: usize = 16;
pub const UnitProperties__heroStr__OFFSET: usize = 20;
pub const UnitProperties__heroAgi__OFFSET: usize = 24;
pub const UnitProperties__heroInt__OFFSET: usize = 28;
pub const UnitProperties__itemTableID__OFFSET: usize = 32;
pub const UnitProperties__itemDist__OFFSET: usize = 40;
pub const UnitProperties__resAmount__OFFSET: usize = 64;
pub const UnitProperties__acquireRad__OFFSET: usize = 68;
pub const UnitProperties__inventory__OFFSET: usize = 72;
pub const UnitProperties__abilities__OFFSET: usize = 96;
pub const UnitProperties__randomInfo__OFFSET: usize = 120;
pub const UnitProperties__color__OFFSET: usize = 160;
pub const UnitProperties__regionTag__OFFSET: usize = 164;

#[repr(C)]
pub struct JassEventInfo {
    pub _opaque: [u8; 8],
}
pub const JassEventInfo__SIZE: usize = 8;
pub const JassEventInfo__NAME: &str = "JassEventInfo";
pub const JassEventInfo__type___OFFSET: usize = 0;
pub const JassEventInfo__version__OFFSET: usize = 4;

#[repr(C)]
pub struct CWidget {
    pub _opaque: [u8; 408],
}
pub const CWidget__SIZE: usize = 408;
pub const CWidget__NAME: &str = "CWidget";
pub const CWidget__s_CWidget_pool__OFFSET: usize = 0;
pub const CWidget__s_updateHintMaps__OFFSET: usize = 0;
pub const CWidget__m_everSeen__OFFSET: usize = 104;
pub const CWidget__m_isFogged__OFFSET: usize = 108;
pub const CWidget__m_widget_id__OFFSET: usize = 112;
pub const CWidget__m_widget_name__OFFSET: usize = 116;
pub const CWidget__m_skin_id__OFFSET: usize = 376;
pub const CWidget__m_pPathingRegion__OFFSET: usize = 384;
pub const CWidget__m_occluder__OFFSET: usize = 392;
pub const CWidget__m_wasOccluding__OFFSET: usize = 396;
pub const CWidget__m_occluderHeight__OFFSET: usize = 400;

#[repr(C)]
pub struct CUnit {
    pub _opaque: [u8; 1864],
}
pub const CUnit__SIZE: usize = 1864;
pub const CUnit__NAME: &str = "CUnit";
pub const CUnit__s_CUnit_pool__OFFSET: usize = 0;
pub const CUnit__m_unit_paused_count__OFFSET: usize = 440;
pub const CUnit__m_PlayingDialogSound__OFFSET: usize = 444;
pub const CUnit__m_owner__OFFSET: usize = 448;
pub const CUnit__m_flags0__OFFSET: usize = 452;
pub const CUnit__m_flags1__OFFSET: usize = 456;
pub const CUnit__m_foodAndTechTreeFlags__OFFSET: usize = 460;
pub const CUnit__m_minimapFlashTimer__OFFSET: usize = 464;
pub const CUnit__m_damage_dealt__OFFSET: usize = 496;
pub const CUnit__m_self_damage__OFFSET: usize = 500;
pub const CUnit__m_healing_done__OFFSET: usize = 504;
pub const CUnit__m_time_stopped__OFFSET: usize = 508;
pub const CUnit__m_total_kills__OFFSET: usize = 512;
pub const CUnit__m_self_kills__OFFSET: usize = 516;
pub const CUnit__m_hero_kills__OFFSET: usize = 520;
pub const CUnit__m_building_kills__OFFSET: usize = 524;
pub const CUnit__m_override_ability_id_stats_credit__OFFSET: usize = 528;
pub const CUnit__m_override_item_id_stats_credit__OFFSET: usize = 532;
pub const CUnit__m_unit_stats_override__OFFSET: usize = 536;
pub const CUnit__m_caller__OFFSET: usize = 544;
pub const CUnit__m_lastAttackNotificationTime__OFFSET: usize = 576;
pub const CUnit__m_lastAttackNotificationSoundTime__OFFSET: usize = 580;
pub const CUnit__m_life__OFFSET: usize = 584;
pub const CUnit__m_death__OFFSET: usize = 608;
pub const CUnit__m_total_life_lost__OFFSET: usize = 616;
pub const CUnit__m_stored_time_when_last_alive__OFFSET: usize = 620;
pub const CUnit__m_time_alive_ms__OFFSET: usize = 624;
pub const CUnit__m_number_of_deaths__OFFSET: usize = 628;
pub const CUnit__m_life_regen_rate__OFFSET: usize = 632;
pub const CUnit__m_life_regen__OFFSET: usize = 648;
pub const CUnit__m_life_regen_type__OFFSET: usize = 656;
pub const CUnit__m_mana__OFFSET: usize = 664;
pub const CUnit__m_ai_mana__OFFSET: usize = 688;
pub const CUnit__m_mana_regen_rate__OFFSET: usize = 704;
pub const CUnit__m_mana_regen__OFFSET: usize = 720;
pub const CUnit__m_total_mana_spent__OFFSET: usize = 728;
pub const CUnit__m_defense__OFFSET: usize = 736;
pub const CUnit__m_battle_type__OFFSET: usize = 752;
pub const CUnit__m_invulnerable_count__OFFSET: usize = 756;
pub const CUnit__m_auto_attack_ignore_count__OFFSET: usize = 760;
pub const CUnit__m_fogFlash__OFFSET: usize = 764;
pub const CUnit__m_sight__OFFSET: usize = 776;
pub const CUnit__m_sightRangeCached__OFFSET: usize = 800;
pub const CUnit__m_sightMod__OFFSET: usize = 808;
pub const CUnit__m_invisibility_count__OFFSET: usize = 816;
pub const CUnit__m_invisibility_percent__OFFSET: usize = 824;
pub const CUnit__m_invis_modify__OFFSET: usize = 848;
pub const CUnit__m_invis_listen__OFFSET: usize = 856;
pub const CUnit__m_detectedData__OFFSET: usize = 864;
pub const CUnit__m_sharedVisionData__OFFSET: usize = 876;
pub const CUnit__m_sharedVisionDataCached__OFFSET: usize = 888;
pub const CUnit__m_sharedVisionTransitiveDataCached__OFFSET: usize = 892;
pub const CUnit__m_frost_count__OFFSET: usize = 896;
pub const CUnit__m_stone_count__OFFSET: usize = 900;
pub const CUnit__m_ethereal_count__OFFSET: usize = 904;
pub const CUnit__m_magic_immunity_count__OFFSET: usize = 908;
pub const CUnit__m_dispel_immunity_count__OFFSET: usize = 912;
pub const CUnit__m_fog_radius__OFFSET: usize = 920;
pub const CUnit__m_pos__OFFSET: usize = 936;
pub const CUnit__m_exp_level__OFFSET: usize = 960;
pub const CUnit__MaxNameLength__OFFSET: usize = 0;
pub const CUnit__m_unit_proper_name__OFFSET: usize = 964;
pub const CUnit__m_pick_order__OFFSET: usize = 1224;
pub const CUnit__m_task_stack__OFFSET: usize = 1228;
pub const CUnit__m_task_timer__OFFSET: usize = 1240;
pub const CUnit__m_action__OFFSET: usize = 1272;
pub const CUnit__m_uninterruptable_count__OFFSET: usize = 1276;
pub const CUnit__m_order_head__OFFSET: usize = 1280;
pub const CUnit__m_order_tail__OFFSET: usize = 1292;
pub const CUnit__order_count__OFFSET: usize = 1304;
pub const CUnit__m_gold_bounty_dice__OFFSET: usize = 1308;
pub const CUnit__m_gold_bounty_sides__OFFSET: usize = 1312;
pub const CUnit__m_gold_bounty_plus__OFFSET: usize = 1316;
pub const CUnit__m_wood_bounty_dice__OFFSET: usize = 1320;
pub const CUnit__m_wood_bounty_sides__OFFSET: usize = 1324;
pub const CUnit__m_wood_bounty_plus__OFFSET: usize = 1328;
pub const CUnit__m_cast_point__OFFSET: usize = 1336;
pub const CUnit__m_cast_back_swing__OFFSET: usize = 1352;
pub const CUnit__m_abilities__OFFSET: usize = 1368;
pub const CUnit__m_disabled_count__OFFSET: usize = 1380;
pub const CUnit__m_disabled_hidden_magic_count__OFFSET: usize = 1384;
pub const CUnit__m_silence_count__OFFSET: usize = 1388;
pub const CUnit__anticheat_dummy_array_4__OFFSET: usize = 1392;
pub const CUnit__m_disabled_magic_count__OFFSET: usize = 1404;
pub const CUnit__m_disabled_hidden_count__OFFSET: usize = 1408;
pub const CUnit__m_abil_paused_count__OFFSET: usize = 1412;
pub const CUnit__m_disabled_hidden_physical_count__OFFSET: usize = 1416;
pub const CUnit__m_disabled_physical_count__OFFSET: usize = 1420;
pub const CUnit__anticheat_dummy_array_5__OFFSET: usize = 1424;
pub const CUnit__m_buff_paused_count__OFFSET: usize = 1432;
pub const CUnit__m_abil_inv__OFFSET: usize = 1440;
pub const CUnit__m_abil_hero__OFFSET: usize = 1448;
pub const CUnit__m_abil_move__OFFSET: usize = 1456;
pub const CUnit__anticheat_dummy_array_6__OFFSET: usize = 1464;
pub const CUnit__m_abil_attack__OFFSET: usize = 1472;
pub const CUnit__m_abil_build__OFFSET: usize = 1480;
pub const CUnit__m_base_priority__OFFSET: usize = 1488;
pub const CUnit__m_death_time__OFFSET: usize = 1504;
pub const CUnit__s_minUnitSpeed__OFFSET: usize = 0;
pub const CUnit__s_maxUnitSpeed__OFFSET: usize = 0;
pub const CUnit__s_minBldgSpeed__OFFSET: usize = 0;
pub const CUnit__s_maxBldgSpeed__OFFSET: usize = 0;
pub const CUnit__s_buildingKillsGiveExp__OFFSET: usize = 0;
pub const CUnit__s_globalExperience__OFFSET: usize = 0;
pub const CUnit__s_maxLevelHeroesDrainExp__OFFSET: usize = 0;
pub const CUnit__m_display_height__OFFSET: usize = 1520;
pub const CUnit__m_display_height_modifier__OFFSET: usize = 1544;
pub const CUnit__anticheat_dummy_array_7__OFFSET: usize = 1552;
pub const CUnit__m_last_cluster_birth_tag__OFFSET: usize = 1560;
pub const CUnit__m_last_cluster_presence_tag__OFFSET: usize = 1564;
pub const CUnit__m_move_type__OFFSET: usize = 1568;
pub const CUnit__m_ground_count__OFFSET: usize = 1572;
pub const CUnit__m_launch_offset__OFFSET: usize = 1576;
pub const CUnit__m_fly_height__OFFSET: usize = 1592;
pub const CUnit__m_impact_swim__OFFSET: usize = 1616;
pub const CUnit__anticheat_dummy_array_8__OFFSET: usize = 1620;
pub const CUnit__m_launch_swim_z__OFFSET: usize = 1624;
pub const CUnit__m_impact_offset__OFFSET: usize = 1628;
pub const CUnit__m_category__OFFSET: usize = 1632;
pub const CUnit__m_target_type_flags__OFFSET: usize = 1636;
pub const CUnit__m_cargo_size__OFFSET: usize = 1640;
pub const CUnit__m_killedBy__OFFSET: usize = 1644;
pub const CUnit__m_shadow__OFFSET: usize = 1656;
pub const CUnit__m_shadowWater__OFFSET: usize = 1660;
pub const CUnit__anticheat_dummy_array_9__OFFSET: usize = 1664;
pub const CUnit__m_shadowAboveWater__OFFSET: usize = 1671;
pub const CUnit__m_shadowShow__OFFSET: usize = 1672;
pub const CUnit__m_showOccMark__OFFSET: usize = 1673;
pub const CUnit__m_allySelectionImage__OFFSET: usize = 1676;
pub const CUnit__m_occluded__OFFSET: usize = 1680;
pub const CUnit__anticheat_dummy_array_10__OFFSET: usize = 1681;
pub const CUnit__m_occlusionMark__OFFSET: usize = 1684;
pub const CUnit__m_jassUserData__OFFSET: usize = 1688;
pub const CUnit__m_foliageExclusionId__OFFSET: usize = 1692;
pub const CUnit__m_uiFlags__OFFSET: usize = 1696;
pub const CUnit__m_cachedPos__OFFSET: usize = 1700;
pub const CUnit__m_cachedFacing__OFFSET: usize = 1712;
pub const CUnit__m_cachedOrientation__OFFSET: usize = 1716;
pub const CUnit__m_lookAtTarget__OFFSET: usize = 1752;
pub const CUnit__m_lookAtOffset__OFFSET: usize = 1760;
pub const CUnit__m_overheadOffset__OFFSET: usize = 1772;
pub const CUnit__m_occupUI__OFFSET: usize = 1776;
pub const CUnit__m_unitStatusIconUI__OFFSET: usize = 1784;
pub const CUnit__m_unitStatusTextUI__OFFSET: usize = 1792;
pub const CUnit__m_uberSplatId__OFFSET: usize = 1800;
pub const CUnit__m_vertexColor__OFFSET: usize = 1804;
pub const CUnit__m_anim_size__OFFSET: usize = 1808;
pub const CUnit__m_anims__OFFSET: usize = 1812;
pub const CUnit__m_forcedAnimScale__OFFSET: usize = 1840;
pub const CUnit__m_walk_anim_speed__OFFSET: usize = 1844;
pub const CUnit__m_run_anim_speed__OFFSET: usize = 1848;
pub const CUnit__m_unit_ai__OFFSET: usize = 1852;

#[repr(C)]
pub struct CMapSetupWar3 {
    pub _opaque: [u8; 168],
}
pub const CMapSetupWar3__SIZE: usize = 168;
pub const CMapSetupWar3__NAME: &str = "CMapSetupWar3";
pub const CMapSetupWar3__m_name__OFFSET: usize = 8;
pub const CMapSetupWar3__m_description__OFFSET: usize = 32;
pub const CMapSetupWar3__m_checkSum__OFFSET: usize = 56;
pub const CMapSetupWar3__m_valid__OFFSET: usize = 60;
pub const CMapSetupWar3__m_mapFile__OFFSET: usize = 64;
pub const CMapSetupWar3__m_gameTypesSupported__OFFSET: usize = 88;
pub const CMapSetupWar3__m_gameTypeSelected__OFFSET: usize = 92;
pub const CMapSetupWar3__m_mapFlags__OFFSET: usize = 96;
pub const CMapSetupWar3__m_mapSizeX__OFFSET: usize = 100;
pub const CMapSetupWar3__m_mapSizeY__OFFSET: usize = 104;
pub const CMapSetupWar3__m_teams__OFFSET: usize = 108;
pub const CMapSetupWar3__m_players__OFFSET: usize = 109;
pub const CMapSetupWar3__m_difficultyLevel__OFFSET: usize = 110;
pub const CMapSetupWar3__m_gameSpeed__OFFSET: usize = 111;
pub const CMapSetupWar3__m_placement__OFFSET: usize = 112;
pub const CMapSetupWar3__m_creatureDensity__OFFSET: usize = 113;
pub const CMapSetupWar3__m_resourceDensity__OFFSET: usize = 114;
pub const CMapSetupWar3__m_introText__OFFSET: usize = 120;
pub const CMapSetupWar3__m_introModel__OFFSET: usize = 144;

#[repr(C)]
pub struct CPlayerWar3 {
    pub _opaque: [u8; 1960],
}
pub const CPlayerWar3__SIZE: usize = 1960;
pub const CPlayerWar3__NAME: &str = "CPlayerWar3";
pub const CPlayerWar3__s_CPlayerWar3_pool__OFFSET: usize = 0;
pub const CPlayerWar3__anticheat_dummy_array_11__OFFSET: usize = 88;
pub const CPlayerWar3__m_playerId__OFFSET: usize = 103;
pub const CPlayerWar3__m_playerName__OFFSET: usize = 104;
pub const CPlayerWar3__m_toonName__OFFSET: usize = 128;
pub const CPlayerWar3__m_protobufPlayerData__OFFSET: usize = 152;
pub const CPlayerWar3__m_slotNetPlayerId__OFFSET: usize = 336;
pub const CPlayerWar3__m_playerNamePrefix__OFFSET: usize = 344;
pub const CPlayerWar3__m_pAlliances__OFFSET: usize = 368;
pub const CPlayerWar3__m_pSelection__OFFSET: usize = 376;
pub const CPlayerWar3__m_playerStates__OFFSET: usize = 384;
pub const CPlayerWar3__m_lumberDivertTax__OFFSET: usize = 1016;
pub const CPlayerWar3__m_goldDivertTax__OFFSET: usize = 1116;
pub const CPlayerWar3__m_highestFood__OFFSET: usize = 1216;
pub const CPlayerWar3__m_highestFoodUsed__OFFSET: usize = 1220;
pub const CPlayerWar3__anticheat_dummy_array_12__OFFSET: usize = 1224;
pub const CPlayerWar3__m_heroKills__OFFSET: usize = 1240;
pub const CPlayerWar3__m_itemsObtained__OFFSET: usize = 1244;
pub const CPlayerWar3__m_mercsHired__OFFSET: usize = 1248;
pub const CPlayerWar3__m_teamColorIndex__OFFSET: usize = 1252;
pub const CPlayerWar3__m_controller__OFFSET: usize = 1256;
pub const CPlayerWar3__m_startLocIndex__OFFSET: usize = 1260;
pub const CPlayerWar3__m_difficulty__OFFSET: usize = 1264;
pub const CPlayerWar3__m_slotState__OFFSET: usize = 1268;
pub const CPlayerWar3__m_teamIndex__OFFSET: usize = 1272;
pub const CPlayerWar3__m_numActions__OFFSET: usize = 1276;
pub const CPlayerWar3__m_racePref__OFFSET: usize = 1280;
pub const CPlayerWar3__m_race__OFFSET: usize = 1284;
pub const CPlayerWar3__m_realTimeAPM__OFFSET: usize = 1288;
pub const CPlayerWar3__m_capChangedListener__OFFSET: usize = 1384;
pub const CPlayerWar3__m_usedChangedListener__OFFSET: usize = 1392;
pub const CPlayerWar3__m_foodUsedUpkeepListener__OFFSET: usize = 1400;
pub const CPlayerWar3__m_lumberChangedListener__OFFSET: usize = 1408;
pub const CPlayerWar3__m_goldChangedListener__OFFSET: usize = 1416;
pub const CPlayerWar3__m_goldUpkeepChanged__OFFSET: usize = 1424;
pub const CPlayerWar3__m_lumberUpkeepChanged__OFFSET: usize = 1432;
pub const CPlayerWar3__m_handicap__OFFSET: usize = 1440;
pub const CPlayerWar3__m_referee__OFFSET: usize = 1512;
pub const CPlayerWar3__m_deadHeroAge__OFFSET: usize = 1520;
pub const CPlayerWar3__m_deadHeroes__OFFSET: usize = 1544;
pub const CPlayerWar3__m_revivableFlags__OFFSET: usize = 1568;
pub const CPlayerWar3__anticheat_dummy_array_13__OFFSET: usize = 1572;
pub const CPlayerWar3__m_pTechTree__OFFSET: usize = 1584;
pub const CPlayerWar3__m_fogMask__OFFSET: usize = 1592;
pub const CPlayerWar3__m_ai__OFFSET: usize = 1596;
pub const CPlayerWar3__m_bestHeroes__OFFSET: usize = 1608;
pub const CPlayerWar3__m_leaderboard__OFFSET: usize = 1632;
pub const CPlayerWar3__m_leaveGameTimer__OFFSET: usize = 1648;
pub const CPlayerWar3__anticheat_dummy_array_14__OFFSET: usize = 1680;
pub const CPlayerWar3__m_doScoreScreen__OFFSET: usize = 1708;
pub const CPlayerWar3__m_allowContinue__OFFSET: usize = 1709;
pub const CPlayerWar3__m_crippled__OFFSET: usize = 1712;
pub const CPlayerWar3__m_gameOverMessage__OFFSET: usize = 1720;
pub const CPlayerWar3__m_showInScoreScreen__OFFSET: usize = 1744;
pub const CPlayerWar3__m_cachedScore__OFFSET: usize = 1748;
pub const CPlayerWar3__m_cachedScoreIsCached__OFFSET: usize = 1752;
pub const CPlayerWar3__m_heroCount__OFFSET: usize = 1756;
pub const CPlayerWar3__m_bnetPlayerId__OFFSET: usize = 1760;
pub const CPlayerWar3__m_heroPickOrder__OFFSET: usize = 1768;
pub const CPlayerWar3__m_unitKills__OFFSET: usize = 1792;
pub const CPlayerWar3__m_unitsCreated__OFFSET: usize = 1824;
pub const CPlayerWar3__m_abilityInfo__OFFSET: usize = 1856;
pub const CPlayerWar3__m_upkeepTime__OFFSET: usize = 1888;
pub const CPlayerWar3__m_lastUpkeepTime__OFFSET: usize = 1920;
pub const CPlayerWar3__m_playerItemUseMap__OFFSET: usize = 1928;

#[repr(C)]
pub struct CUnit__1 {
    pub _opaque: [u8; 1864],
}
pub const CUnit__1__SIZE: usize = 1864;
pub const CUnit__1__NAME: &str = "CUnit";
pub const CUnit__1__s_CUnit_pool__OFFSET: usize = 0;
pub const CUnit__1__m_unit_paused_count__OFFSET: usize = 440;
pub const CUnit__1__m_PlayingDialogSound__OFFSET: usize = 444;
pub const CUnit__1__m_owner__OFFSET: usize = 448;
pub const CUnit__1__m_flags0__OFFSET: usize = 452;
pub const CUnit__1__m_flags1__OFFSET: usize = 456;
pub const CUnit__1__m_foodAndTechTreeFlags__OFFSET: usize = 460;
pub const CUnit__1__m_minimapFlashTimer__OFFSET: usize = 464;
pub const CUnit__1__m_damage_dealt__OFFSET: usize = 496;
pub const CUnit__1__m_self_damage__OFFSET: usize = 500;
pub const CUnit__1__m_healing_done__OFFSET: usize = 504;
pub const CUnit__1__m_time_stopped__OFFSET: usize = 508;
pub const CUnit__1__m_total_kills__OFFSET: usize = 512;
pub const CUnit__1__m_self_kills__OFFSET: usize = 516;
pub const CUnit__1__m_hero_kills__OFFSET: usize = 520;
pub const CUnit__1__m_building_kills__OFFSET: usize = 524;
pub const CUnit__1__m_override_ability_id_stats_credit__OFFSET: usize = 528;
pub const CUnit__1__m_override_item_id_stats_credit__OFFSET: usize = 532;
pub const CUnit__1__m_unit_stats_override__OFFSET: usize = 536;
pub const CUnit__1__m_caller__OFFSET: usize = 544;
pub const CUnit__1__m_lastAttackNotificationTime__OFFSET: usize = 576;
pub const CUnit__1__m_lastAttackNotificationSoundTime__OFFSET: usize = 580;
pub const CUnit__1__m_life__OFFSET: usize = 584;
pub const CUnit__1__m_death__OFFSET: usize = 608;
pub const CUnit__1__m_total_life_lost__OFFSET: usize = 616;
pub const CUnit__1__m_stored_time_when_last_alive__OFFSET: usize = 620;
pub const CUnit__1__m_time_alive_ms__OFFSET: usize = 624;
pub const CUnit__1__m_number_of_deaths__OFFSET: usize = 628;
pub const CUnit__1__m_life_regen_rate__OFFSET: usize = 632;
pub const CUnit__1__m_life_regen__OFFSET: usize = 648;
pub const CUnit__1__m_life_regen_type__OFFSET: usize = 656;
pub const CUnit__1__m_mana__OFFSET: usize = 664;
pub const CUnit__1__m_ai_mana__OFFSET: usize = 688;
pub const CUnit__1__m_mana_regen_rate__OFFSET: usize = 704;
pub const CUnit__1__m_mana_regen__OFFSET: usize = 720;
pub const CUnit__1__m_total_mana_spent__OFFSET: usize = 728;
pub const CUnit__1__m_defense__OFFSET: usize = 736;
pub const CUnit__1__m_battle_type__OFFSET: usize = 752;
pub const CUnit__1__m_invulnerable_count__OFFSET: usize = 756;
pub const CUnit__1__m_auto_attack_ignore_count__OFFSET: usize = 760;
pub const CUnit__1__m_fogFlash__OFFSET: usize = 764;
pub const CUnit__1__m_sight__OFFSET: usize = 776;
pub const CUnit__1__m_sightRangeCached__OFFSET: usize = 800;
pub const CUnit__1__m_sightMod__OFFSET: usize = 808;
pub const CUnit__1__m_invisibility_count__OFFSET: usize = 816;
pub const CUnit__1__m_invisibility_percent__OFFSET: usize = 824;
pub const CUnit__1__m_invis_modify__OFFSET: usize = 848;
pub const CUnit__1__m_invis_listen__OFFSET: usize = 856;
pub const CUnit__1__m_detectedData__OFFSET: usize = 864;
pub const CUnit__1__m_sharedVisionData__OFFSET: usize = 876;
pub const CUnit__1__m_sharedVisionDataCached__OFFSET: usize = 888;
pub const CUnit__1__m_sharedVisionTransitiveDataCached__OFFSET: usize = 892;
pub const CUnit__1__m_frost_count__OFFSET: usize = 896;
pub const CUnit__1__m_stone_count__OFFSET: usize = 900;
pub const CUnit__1__m_ethereal_count__OFFSET: usize = 904;
pub const CUnit__1__m_magic_immunity_count__OFFSET: usize = 908;
pub const CUnit__1__m_dispel_immunity_count__OFFSET: usize = 912;
pub const CUnit__1__m_fog_radius__OFFSET: usize = 920;
pub const CUnit__1__m_pos__OFFSET: usize = 936;
pub const CUnit__1__m_exp_level__OFFSET: usize = 960;
pub const CUnit__1__MaxNameLength__OFFSET: usize = 0;
pub const CUnit__1__m_unit_proper_name__OFFSET: usize = 964;
pub const CUnit__1__m_pick_order__OFFSET: usize = 1224;
pub const CUnit__1__m_task_stack__OFFSET: usize = 1228;
pub const CUnit__1__m_task_timer__OFFSET: usize = 1240;
pub const CUnit__1__m_action__OFFSET: usize = 1272;
pub const CUnit__1__m_uninterruptable_count__OFFSET: usize = 1276;
pub const CUnit__1__m_order_head__OFFSET: usize = 1280;
pub const CUnit__1__m_order_tail__OFFSET: usize = 1292;
pub const CUnit__1__order_count__OFFSET: usize = 1304;
pub const CUnit__1__m_gold_bounty_dice__OFFSET: usize = 1308;
pub const CUnit__1__m_gold_bounty_sides__OFFSET: usize = 1312;
pub const CUnit__1__m_gold_bounty_plus__OFFSET: usize = 1316;
pub const CUnit__1__m_wood_bounty_dice__OFFSET: usize = 1320;
pub const CUnit__1__m_wood_bounty_sides__OFFSET: usize = 1324;
pub const CUnit__1__m_wood_bounty_plus__OFFSET: usize = 1328;
pub const CUnit__1__m_cast_point__OFFSET: usize = 1336;
pub const CUnit__1__m_cast_back_swing__OFFSET: usize = 1352;
pub const CUnit__1__m_abilities__OFFSET: usize = 1368;
pub const CUnit__1__m_disabled_count__OFFSET: usize = 1380;
pub const CUnit__1__m_disabled_hidden_magic_count__OFFSET: usize = 1384;
pub const CUnit__1__m_silence_count__OFFSET: usize = 1388;
pub const CUnit__1__anticheat_dummy_array_18__OFFSET: usize = 1392;
pub const CUnit__1__m_disabled_magic_count__OFFSET: usize = 1404;
pub const CUnit__1__m_disabled_hidden_count__OFFSET: usize = 1408;
pub const CUnit__1__m_abil_paused_count__OFFSET: usize = 1412;
pub const CUnit__1__m_disabled_hidden_physical_count__OFFSET: usize = 1416;
pub const CUnit__1__m_disabled_physical_count__OFFSET: usize = 1420;
pub const CUnit__1__anticheat_dummy_array_19__OFFSET: usize = 1424;
pub const CUnit__1__m_buff_paused_count__OFFSET: usize = 1432;
pub const CUnit__1__m_abil_inv__OFFSET: usize = 1440;
pub const CUnit__1__m_abil_hero__OFFSET: usize = 1448;
pub const CUnit__1__m_abil_move__OFFSET: usize = 1456;
pub const CUnit__1__anticheat_dummy_array_20__OFFSET: usize = 1464;
pub const CUnit__1__m_abil_attack__OFFSET: usize = 1472;
pub const CUnit__1__m_abil_build__OFFSET: usize = 1480;
pub const CUnit__1__m_base_priority__OFFSET: usize = 1488;
pub const CUnit__1__m_death_time__OFFSET: usize = 1504;
pub const CUnit__1__s_minUnitSpeed__OFFSET: usize = 0;
pub const CUnit__1__s_maxUnitSpeed__OFFSET: usize = 0;
pub const CUnit__1__s_minBldgSpeed__OFFSET: usize = 0;
pub const CUnit__1__s_maxBldgSpeed__OFFSET: usize = 0;
pub const CUnit__1__s_buildingKillsGiveExp__OFFSET: usize = 0;
pub const CUnit__1__s_globalExperience__OFFSET: usize = 0;
pub const CUnit__1__s_maxLevelHeroesDrainExp__OFFSET: usize = 0;
pub const CUnit__1__m_display_height__OFFSET: usize = 1520;
pub const CUnit__1__m_display_height_modifier__OFFSET: usize = 1544;
pub const CUnit__1__anticheat_dummy_array_21__OFFSET: usize = 1552;
pub const CUnit__1__m_last_cluster_birth_tag__OFFSET: usize = 1560;
pub const CUnit__1__m_last_cluster_presence_tag__OFFSET: usize = 1564;
pub const CUnit__1__m_move_type__OFFSET: usize = 1568;
pub const CUnit__1__m_ground_count__OFFSET: usize = 1572;
pub const CUnit__1__m_launch_offset__OFFSET: usize = 1576;
pub const CUnit__1__m_fly_height__OFFSET: usize = 1592;
pub const CUnit__1__m_impact_swim__OFFSET: usize = 1616;
pub const CUnit__1__anticheat_dummy_array_22__OFFSET: usize = 1620;
pub const CUnit__1__m_launch_swim_z__OFFSET: usize = 1624;
pub const CUnit__1__m_impact_offset__OFFSET: usize = 1628;
pub const CUnit__1__m_category__OFFSET: usize = 1632;
pub const CUnit__1__m_target_type_flags__OFFSET: usize = 1636;
pub const CUnit__1__m_cargo_size__OFFSET: usize = 1640;
pub const CUnit__1__m_killedBy__OFFSET: usize = 1644;
pub const CUnit__1__m_shadow__OFFSET: usize = 1656;
pub const CUnit__1__m_shadowWater__OFFSET: usize = 1660;
pub const CUnit__1__anticheat_dummy_array_23__OFFSET: usize = 1664;
pub const CUnit__1__m_shadowAboveWater__OFFSET: usize = 1671;
pub const CUnit__1__m_shadowShow__OFFSET: usize = 1672;
pub const CUnit__1__m_showOccMark__OFFSET: usize = 1673;
pub const CUnit__1__m_allySelectionImage__OFFSET: usize = 1676;
pub const CUnit__1__m_occluded__OFFSET: usize = 1680;
pub const CUnit__1__anticheat_dummy_array_24__OFFSET: usize = 1681;
pub const CUnit__1__m_occlusionMark__OFFSET: usize = 1684;
pub const CUnit__1__m_jassUserData__OFFSET: usize = 1688;
pub const CUnit__1__m_foliageExclusionId__OFFSET: usize = 1692;
pub const CUnit__1__m_uiFlags__OFFSET: usize = 1696;
pub const CUnit__1__m_cachedPos__OFFSET: usize = 1700;
pub const CUnit__1__m_cachedFacing__OFFSET: usize = 1712;
pub const CUnit__1__m_cachedOrientation__OFFSET: usize = 1716;
pub const CUnit__1__m_lookAtTarget__OFFSET: usize = 1752;
pub const CUnit__1__m_lookAtOffset__OFFSET: usize = 1760;
pub const CUnit__1__m_overheadOffset__OFFSET: usize = 1772;
pub const CUnit__1__m_occupUI__OFFSET: usize = 1776;
pub const CUnit__1__m_unitStatusIconUI__OFFSET: usize = 1784;
pub const CUnit__1__m_unitStatusTextUI__OFFSET: usize = 1792;
pub const CUnit__1__m_uberSplatId__OFFSET: usize = 1800;
pub const CUnit__1__m_vertexColor__OFFSET: usize = 1804;
pub const CUnit__1__m_anim_size__OFFSET: usize = 1808;
pub const CUnit__1__m_anims__OFFSET: usize = 1812;
pub const CUnit__1__m_forcedAnimScale__OFFSET: usize = 1840;
pub const CUnit__1__m_walk_anim_speed__OFFSET: usize = 1844;
pub const CUnit__1__m_run_anim_speed__OFFSET: usize = 1848;
pub const CUnit__1__m_unit_ai__OFFSET: usize = 1852;

#[repr(C)]
pub struct GameCacheIntegerReg {
    pub _opaque: [u8; 56],
}
pub const GameCacheIntegerReg__SIZE: usize = 56;
pub const GameCacheIntegerReg__NAME: &str = "GameCacheIntegerReg";
pub const GameCacheIntegerReg__value__OFFSET: usize = 48;

#[repr(C)]
pub struct GameCacheRealReg {
    pub _opaque: [u8; 56],
}
pub const GameCacheRealReg__SIZE: usize = 56;
pub const GameCacheRealReg__NAME: &str = "GameCacheRealReg";
pub const GameCacheRealReg__value__OFFSET: usize = 48;

#[repr(C)]
pub struct GameCacheBooleanReg {
    pub _opaque: [u8; 56],
}
pub const GameCacheBooleanReg__SIZE: usize = 56;
pub const GameCacheBooleanReg__NAME: &str = "GameCacheBooleanReg";
pub const GameCacheBooleanReg__value__OFFSET: usize = 48;

#[repr(C)]
pub struct GameCacheStringReg {
    pub _opaque: [u8; 72],
}
pub const GameCacheStringReg__SIZE: usize = 72;
pub const GameCacheStringReg__NAME: &str = "GameCacheStringReg";
pub const GameCacheStringReg__value__OFFSET: usize = 48;

#[repr(C)]
pub struct GameCacheUnitReg {
    pub _opaque: [u8; 184],
}
pub const GameCacheUnitReg__SIZE: usize = 184;
pub const GameCacheUnitReg__NAME: &str = "GameCacheUnitReg";
pub const GameCacheUnitReg__unitType__OFFSET: usize = 48;
pub const GameCacheUnitReg__items__OFFSET: usize = 56;
pub const GameCacheUnitReg__heroData__OFFSET: usize = 80;

#[repr(C)]
pub struct CTriggerStrings {
    pub _opaque: [u8; 24],
}
pub const CTriggerStrings__SIZE: usize = 24;
pub const CTriggerStrings__NAME: &str = "CTriggerStrings";
pub const CTriggerStrings__TriggerStringInvalidID__OFFSET: usize = 0;
pub const CTriggerStrings__mStrings__OFFSET: usize = 0;

#[repr(C)]
pub struct CGameCache {
    pub _opaque: [u8; 112],
}
pub const CGameCache__SIZE: usize = 112;
pub const CGameCache__NAME: &str = "CGameCache";
pub const CGameCache__s_CGameCache_pool__OFFSET: usize = 0;
pub const CGameCache__m_campaignKey__OFFSET: usize = 88;

#[repr(C)]
pub struct CGameCacheManager {
    pub _opaque: [u8; 216],
}
pub const CGameCacheManager__SIZE: usize = 216;
pub const CGameCacheManager__NAME: &str = "CGameCacheManager";
pub const CGameCacheManager__s_CGameCacheManager_pool__OFFSET: usize = 0;
pub const CGameCacheManager__m_campaignFilePath__OFFSET: usize = 88;
pub const CGameCacheManager__m_campaignKey__OFFSET: usize = 112;
pub const CGameCacheManager__m_campaigns__OFFSET: usize = 136;
pub const CGameCacheManager__m_campaignCount__OFFSET: usize = 208;

#[repr(C)]
pub struct GameHashTableIntegerReg {
    pub _opaque: [u8; 56],
}
pub const GameHashTableIntegerReg__SIZE: usize = 56;
pub const GameHashTableIntegerReg__NAME: &str = "GameHashTableIntegerReg";
pub const GameHashTableIntegerReg__value__OFFSET: usize = 48;

#[repr(C)]
pub struct GameHashTableRealReg {
    pub _opaque: [u8; 56],
}
pub const GameHashTableRealReg__SIZE: usize = 56;
pub const GameHashTableRealReg__NAME: &str = "GameHashTableRealReg";
pub const GameHashTableRealReg__value__OFFSET: usize = 48;

#[repr(C)]
pub struct GameHashTableBooleanReg {
    pub _opaque: [u8; 56],
}
pub const GameHashTableBooleanReg__SIZE: usize = 56;
pub const GameHashTableBooleanReg__NAME: &str = "GameHashTableBooleanReg";
pub const GameHashTableBooleanReg__value__OFFSET: usize = 48;

#[repr(C)]
pub struct GameHashTableStringReg {
    pub _opaque: [u8; 72],
}
pub const GameHashTableStringReg__SIZE: usize = 72;
pub const GameHashTableStringReg__NAME: &str = "GameHashTableStringReg";
pub const GameHashTableStringReg__value__OFFSET: usize = 48;

#[repr(C)]
pub struct GameHashTableHandleReg {
    pub _opaque: [u8; 56],
}
pub const GameHashTableHandleReg__SIZE: usize = 56;
pub const GameHashTableHandleReg__NAME: &str = "GameHashTableHandleReg";
pub const GameHashTableHandleReg__handle__OFFSET: usize = 48;
pub const GameHashTableHandleReg__handleType__OFFSET: usize = 52;

#[repr(C)]
pub struct CGameHashTable {
    pub _opaque: [u8; 96],
}
pub const CGameHashTable__SIZE: usize = 96;
pub const CGameHashTable__NAME: &str = "CGameHashTable";
pub const CGameHashTable__s_CGameHashTable_pool__OFFSET: usize = 0;
pub const CGameHashTable__m_parentKey__OFFSET: usize = 88;

#[repr(C)]
pub struct CGameHashTableManager {
    pub _opaque: [u8; 88],
}
pub const CGameHashTableManager__SIZE: usize = 88;
pub const CGameHashTableManager__NAME: &str = "CGameHashTableManager";
pub const CGameHashTableManager__m_parents__OFFSET: usize = 8;
pub const CGameHashTableManager__m_parentCount__OFFSET: usize = 80;
pub const CGameHashTableManager__m_nextKey__OFFSET: usize = 84;

#[repr(C)]
pub struct CPlayerWar3__1 {
    pub _opaque: [u8; 1960],
}
pub const CPlayerWar3__1__SIZE: usize = 1960;
pub const CPlayerWar3__1__NAME: &str = "CPlayerWar3";
pub const CPlayerWar3__1__s_CPlayerWar3_pool__OFFSET: usize = 0;
pub const CPlayerWar3__1__anticheat_dummy_array_4__OFFSET: usize = 88;
pub const CPlayerWar3__1__m_playerId__OFFSET: usize = 103;
pub const CPlayerWar3__1__m_playerName__OFFSET: usize = 104;
pub const CPlayerWar3__1__m_toonName__OFFSET: usize = 128;
pub const CPlayerWar3__1__m_protobufPlayerData__OFFSET: usize = 152;
pub const CPlayerWar3__1__m_slotNetPlayerId__OFFSET: usize = 336;
pub const CPlayerWar3__1__m_playerNamePrefix__OFFSET: usize = 344;
pub const CPlayerWar3__1__m_pAlliances__OFFSET: usize = 368;
pub const CPlayerWar3__1__m_pSelection__OFFSET: usize = 376;
pub const CPlayerWar3__1__m_playerStates__OFFSET: usize = 384;
pub const CPlayerWar3__1__m_lumberDivertTax__OFFSET: usize = 1016;
pub const CPlayerWar3__1__m_goldDivertTax__OFFSET: usize = 1116;
pub const CPlayerWar3__1__m_highestFood__OFFSET: usize = 1216;
pub const CPlayerWar3__1__m_highestFoodUsed__OFFSET: usize = 1220;
pub const CPlayerWar3__1__anticheat_dummy_array_5__OFFSET: usize = 1224;
pub const CPlayerWar3__1__m_heroKills__OFFSET: usize = 1240;
pub const CPlayerWar3__1__m_itemsObtained__OFFSET: usize = 1244;
pub const CPlayerWar3__1__m_mercsHired__OFFSET: usize = 1248;
pub const CPlayerWar3__1__m_teamColorIndex__OFFSET: usize = 1252;
pub const CPlayerWar3__1__m_controller__OFFSET: usize = 1256;
pub const CPlayerWar3__1__m_startLocIndex__OFFSET: usize = 1260;
pub const CPlayerWar3__1__m_difficulty__OFFSET: usize = 1264;
pub const CPlayerWar3__1__m_slotState__OFFSET: usize = 1268;
pub const CPlayerWar3__1__m_teamIndex__OFFSET: usize = 1272;
pub const CPlayerWar3__1__m_numActions__OFFSET: usize = 1276;
pub const CPlayerWar3__1__m_racePref__OFFSET: usize = 1280;
pub const CPlayerWar3__1__m_race__OFFSET: usize = 1284;
pub const CPlayerWar3__1__m_realTimeAPM__OFFSET: usize = 1288;
pub const CPlayerWar3__1__m_capChangedListener__OFFSET: usize = 1384;
pub const CPlayerWar3__1__m_usedChangedListener__OFFSET: usize = 1392;
pub const CPlayerWar3__1__m_foodUsedUpkeepListener__OFFSET: usize = 1400;
pub const CPlayerWar3__1__m_lumberChangedListener__OFFSET: usize = 1408;
pub const CPlayerWar3__1__m_goldChangedListener__OFFSET: usize = 1416;
pub const CPlayerWar3__1__m_goldUpkeepChanged__OFFSET: usize = 1424;
pub const CPlayerWar3__1__m_lumberUpkeepChanged__OFFSET: usize = 1432;
pub const CPlayerWar3__1__m_handicap__OFFSET: usize = 1440;
pub const CPlayerWar3__1__m_referee__OFFSET: usize = 1512;
pub const CPlayerWar3__1__m_deadHeroAge__OFFSET: usize = 1520;
pub const CPlayerWar3__1__m_deadHeroes__OFFSET: usize = 1544;
pub const CPlayerWar3__1__m_revivableFlags__OFFSET: usize = 1568;
pub const CPlayerWar3__1__anticheat_dummy_array_6__OFFSET: usize = 1572;
pub const CPlayerWar3__1__m_pTechTree__OFFSET: usize = 1584;
pub const CPlayerWar3__1__m_fogMask__OFFSET: usize = 1592;
pub const CPlayerWar3__1__m_ai__OFFSET: usize = 1596;
pub const CPlayerWar3__1__m_bestHeroes__OFFSET: usize = 1608;
pub const CPlayerWar3__1__m_leaderboard__OFFSET: usize = 1632;
pub const CPlayerWar3__1__m_leaveGameTimer__OFFSET: usize = 1648;
pub const CPlayerWar3__1__anticheat_dummy_array_7__OFFSET: usize = 1680;
pub const CPlayerWar3__1__m_doScoreScreen__OFFSET: usize = 1708;
pub const CPlayerWar3__1__m_allowContinue__OFFSET: usize = 1709;
pub const CPlayerWar3__1__m_crippled__OFFSET: usize = 1712;
pub const CPlayerWar3__1__m_gameOverMessage__OFFSET: usize = 1720;
pub const CPlayerWar3__1__m_showInScoreScreen__OFFSET: usize = 1744;
pub const CPlayerWar3__1__m_cachedScore__OFFSET: usize = 1748;
pub const CPlayerWar3__1__m_cachedScoreIsCached__OFFSET: usize = 1752;
pub const CPlayerWar3__1__m_heroCount__OFFSET: usize = 1756;
pub const CPlayerWar3__1__m_bnetPlayerId__OFFSET: usize = 1760;
pub const CPlayerWar3__1__m_heroPickOrder__OFFSET: usize = 1768;
pub const CPlayerWar3__1__m_unitKills__OFFSET: usize = 1792;
pub const CPlayerWar3__1__m_unitsCreated__OFFSET: usize = 1824;
pub const CPlayerWar3__1__m_abilityInfo__OFFSET: usize = 1856;
pub const CPlayerWar3__1__m_upkeepTime__OFFSET: usize = 1888;
pub const CPlayerWar3__1__m_lastUpkeepTime__OFFSET: usize = 1920;
pub const CPlayerWar3__1__m_playerItemUseMap__OFFSET: usize = 1928;

#[repr(C)]
pub struct UnitUIDef {
    pub _opaque: [u8; 656],
}
pub const UnitUIDef__SIZE: usize = 656;
pub const UnitUIDef__NAME: &str = "UnitUIDef";
pub const UnitUIDef__idStr__OFFSET: usize = 72;
pub const UnitUIDef__m_revive__OFFSET: usize = 77;
pub const UnitUIDef__m_hideHeroBar__OFFSET: usize = 78;
pub const UnitUIDef__m_hideHeroMinimap__OFFSET: usize = 79;
pub const UnitUIDef__m_hideOnMinimap__OFFSET: usize = 80;
pub const UnitUIDef__m_hideHeroDeathMsg__OFFSET: usize = 81;
pub const UnitUIDef__m_useNBMinimapIcon__OFFSET: usize = 82;
pub const UnitUIDef__m_isStructure__OFFSET: usize = 83;
pub const UnitUIDef__m_preventPlacementMask__OFFSET: usize = 84;
pub const UnitUIDef__m_requirePlacementMask__OFFSET: usize = 85;
pub const UnitUIDef__m_scoreScreenIcon__OFFSET: usize = 88;
pub const UnitUIDef__m_name__OFFSET: usize = 128;
pub const UnitUIDef__m_buttonPos__OFFSET: usize = 152;
pub const UnitUIDef__m_launchOffset__OFFSET: usize = 160;
pub const UnitUIDef__m_pathingFootprint__OFFSET: usize = 168;
pub const UnitUIDef__m_pathingFootprintAlt__OFFSET: usize = 184;
pub const UnitUIDef__m_rangedMissileSpeed__OFFSET: usize = 200;
pub const UnitUIDef__m_rangedMissileHoming__OFFSET: usize = 224;
pub const UnitUIDef__m_dependencyOr__OFFSET: usize = 248;
pub const UnitUIDef__m_hotkey__OFFSET: usize = 272;
pub const UnitUIDef__m_xpFactor__OFFSET: usize = 296;
pub const UnitUIDef__m_upgradesIds__OFFSET: usize = 320;
pub const UnitUIDef__m_buildsIds__OFFSET: usize = 348;
pub const UnitUIDef__m_trainsIds__OFFSET: usize = 376;
pub const UnitUIDef__m_researchesIds__OFFSET: usize = 404;
pub const UnitUIDef__m_sellsUnitIds__OFFSET: usize = 432;
pub const UnitUIDef__m_sellsItemIds__OFFSET: usize = 460;
pub const UnitUIDef__m_makesItemIds__OFFSET: usize = 488;
pub const UnitUIDef__m_requirementIds__OFFSET: usize = 516;
pub const UnitUIDef__m_requirementAmounts__OFFSET: usize = 544;
pub const UnitUIDef__m_requirementHadIds__OFFSET: usize = 572;
pub const UnitUIDef__m_requirementHadAmounts__OFFSET: usize = 600;
pub const UnitUIDef__m_reviveAtIds__OFFSET: usize = 628;

#[repr(C)]
pub struct PlayerItemInfo {
    pub _opaque: [u8; 32],
}
pub const PlayerItemInfo__SIZE: usize = 32;
pub const PlayerItemInfo__NAME: &str = "PlayerItemInfo";
pub const PlayerItemInfo__collected__OFFSET: usize = 0;
pub const PlayerItemInfo__purchased__OFFSET: usize = 4;
pub const PlayerItemInfo__sold__OFFSET: usize = 8;
pub const PlayerItemInfo__used__OFFSET: usize = 12;
pub const PlayerItemInfo__destroyed__OFFSET: usize = 16;
pub const PlayerItemInfo__damage_dealt__OFFSET: usize = 20;
pub const PlayerItemInfo__healing_done__OFFSET: usize = 24;
pub const PlayerItemInfo__item_level__OFFSET: usize = 28;

#[repr(C)]
pub struct PlayerAbibiltyInfo {
    pub _opaque: [u8; 8],
}
pub const PlayerAbibiltyInfo__SIZE: usize = 8;
pub const PlayerAbibiltyInfo__NAME: &str = "PlayerAbibiltyInfo";
pub const PlayerAbibiltyInfo__healing_done__OFFSET: usize = 0;
pub const PlayerAbibiltyInfo__damage_dealt__OFFSET: usize = 4;

#[repr(C)]
pub struct CGameWar3 {
    pub _opaque: [u8; 12872],
}
pub const CGameWar3__SIZE: usize = 12872;
pub const CGameWar3__NAME: &str = "CGameWar3";
pub const CGameWar3__s_campaignArchiveFile__OFFSET: usize = 0;
pub const CGameWar3__anticheat_dummy_array_15__OFFSET: usize = 9640;
pub const CGameWar3__s_pGameWar3__OFFSET: usize = 0;
pub const CGameWar3__anticheat_dummy_array_16__OFFSET: usize = 9695;
pub const CGameWar3__m_mapScriptInstance__OFFSET: usize = 9784;
pub const CGameWar3__m_mapScriptInstanceId__OFFSET: usize = 9792;
pub const CGameWar3__m_mapScript__OFFSET: usize = 9800;
pub const CGameWar3__m_mapScriptId__OFFSET: usize = 9808;
pub const CGameWar3__m_executions__OFFSET: usize = 9816;
pub const CGameWar3__anticheat_dummy_array_17__OFFSET: usize = 9840;
pub const CGameWar3__m_gameIdMap__OFFSET: usize = 9864;
pub const CGameWar3__m_gameState__OFFSET: usize = 9872;
pub const CGameWar3__m_buildId__OFFSET: usize = 9880;
pub const CGameWar3__m_localPlayerId__OFFSET: usize = 9884;
pub const CGameWar3__m_replayLocalPlayerId__OFFSET: usize = 9886;
pub const CGameWar3__m_playerIdBitsToGhost__OFFSET: usize = 9888;
pub const CGameWar3__m_playerIdBitsToGhostSet__OFFSET: usize = 9892;
pub const CGameWar3__m_pMapSetup__OFFSET: usize = 9896;
pub const CGameWar3__m_pFog__OFFSET: usize = 9904;
pub const CGameWar3__m_gameName__OFFSET: usize = 9912;
pub const CGameWar3__anticheat_dummy_array_18__OFFSET: usize = 9936;
pub const CGameWar3__m_maxPlayerCount__OFFSET: usize = 9952;
pub const CGameWar3__m_playerCount__OFFSET: usize = 9956;
pub const CGameWar3__m_gameMaxPlayerCount__OFFSET: usize = 9960;
pub const CGameWar3__m_maxTeamCount__OFFSET: usize = 9964;
pub const CGameWar3__m_teams__OFFSET: usize = 9968;
pub const CGameWar3__m_players__OFFSET: usize = 9992;
pub const CGameWar3__m_startLocations__OFFSET: usize = 10224;
pub const CGameWar3__m_slotToPlayerId__OFFSET: usize = 11960;
pub const CGameWar3__m_levelToTransitionTo__OFFSET: usize = 12064;
pub const CGameWar3__m_modelCine__OFFSET: usize = 12088;
pub const CGameWar3__m_cine__OFFSET: usize = 12112;
pub const CGameWar3__m_gameOverMessage__OFFSET: usize = 12376;
pub const CGameWar3__m_switchingLevels__OFFSET: usize = 12400;
pub const CGameWar3__m_doScoreScreen__OFFSET: usize = 12401;
pub const CGameWar3__m_isReload__OFFSET: usize = 12402;
pub const CGameWar3__anticheat_dummy_array_19__OFFSET: usize = 12403;
pub const CGameWar3__m_wasScriptLoad__OFFSET: usize = 12410;
pub const CGameWar3__m_wasLoadGame__OFFSET: usize = 12411;
pub const CGameWar3__m_isCampaignMission__OFFSET: usize = 12412;
pub const CGameWar3__m_campaignIndex__OFFSET: usize = 12416;
pub const CGameWar3__m_gameSetup__OFFSET: usize = 12420;
pub const CGameWar3__m_usingDarkMask__OFFSET: usize = 12678;
pub const CGameWar3__m_unseenQuests__OFFSET: usize = 12679;
pub const CGameWar3__m_showAllUnitsCmdBar__OFFSET: usize = 12680;
pub const CGameWar3__m_isReplay__OFFSET: usize = 12681;
pub const CGameWar3__m_replayFile__OFFSET: usize = 12688;
pub const CGameWar3__m_isTournament__OFFSET: usize = 12712;
pub const CGameWar3__m_copySaveCount__OFFSET: usize = 12716;
pub const CGameWar3__m_finishedInit__OFFSET: usize = 12720;
pub const CGameWar3__m_didSaveGameFromScript__OFFSET: usize = 12721;
pub const CGameWar3__m_playingGame__OFFSET: usize = 12722;
pub const CGameWar3__m_FacialAnimationSetScopeID__OFFSET: usize = 12724;
pub const CGameWar3__m_pTriggerStrings__OFFSET: usize = 12728;
pub const CGameWar3__anticheat_dummy_array_20__OFFSET: usize = 12736;
pub const CGameWar3__m_messageLogContents__OFFSET: usize = 12776;
pub const CGameWar3__m_pGameHashTableManager__OFFSET: usize = 12800;
pub const CGameWar3__m_gameCacheManager__OFFSET: usize = 12808;
pub const CGameWar3__m_gameOverCallback__OFFSET: usize = 12816;
pub const CGameWar3__m_maxCheckpointSaves__OFFSET: usize = 12824;
pub const CGameWar3__m_pSaveThread__OFFSET: usize = 12832;
pub const CGameWar3__m_pCheckpointPopup__OFFSET: usize = 12840;
pub const CGameWar3__m_gamePlayStartTime__OFFSET: usize = 12848;
pub const CGameWar3__m_latency__OFFSET: usize = 12856;

#[repr(C)]
pub struct CGameWar3__1 {
    pub _opaque: [u8; 12872],
}
pub const CGameWar3__1__SIZE: usize = 12872;
pub const CGameWar3__1__NAME: &str = "CGameWar3";
pub const CGameWar3__1__s_campaignArchiveFile__OFFSET: usize = 0;
pub const CGameWar3__1__anticheat_dummy_array_8__OFFSET: usize = 9640;
pub const CGameWar3__1__s_pGameWar3__OFFSET: usize = 0;
pub const CGameWar3__1__anticheat_dummy_array_9__OFFSET: usize = 9695;
pub const CGameWar3__1__m_mapScriptInstance__OFFSET: usize = 9784;
pub const CGameWar3__1__m_mapScriptInstanceId__OFFSET: usize = 9792;
pub const CGameWar3__1__m_mapScript__OFFSET: usize = 9800;
pub const CGameWar3__1__m_mapScriptId__OFFSET: usize = 9808;
pub const CGameWar3__1__m_executions__OFFSET: usize = 9816;
pub const CGameWar3__1__anticheat_dummy_array_10__OFFSET: usize = 9840;
pub const CGameWar3__1__m_gameIdMap__OFFSET: usize = 9864;
pub const CGameWar3__1__m_gameState__OFFSET: usize = 9872;
pub const CGameWar3__1__m_buildId__OFFSET: usize = 9880;
pub const CGameWar3__1__m_localPlayerId__OFFSET: usize = 9884;
pub const CGameWar3__1__m_replayLocalPlayerId__OFFSET: usize = 9886;
pub const CGameWar3__1__m_playerIdBitsToGhost__OFFSET: usize = 9888;
pub const CGameWar3__1__m_playerIdBitsToGhostSet__OFFSET: usize = 9892;
pub const CGameWar3__1__m_pMapSetup__OFFSET: usize = 9896;
pub const CGameWar3__1__m_pFog__OFFSET: usize = 9904;
pub const CGameWar3__1__m_gameName__OFFSET: usize = 9912;
pub const CGameWar3__1__anticheat_dummy_array_11__OFFSET: usize = 9936;
pub const CGameWar3__1__m_maxPlayerCount__OFFSET: usize = 9952;
pub const CGameWar3__1__m_playerCount__OFFSET: usize = 9956;
pub const CGameWar3__1__m_gameMaxPlayerCount__OFFSET: usize = 9960;
pub const CGameWar3__1__m_maxTeamCount__OFFSET: usize = 9964;
pub const CGameWar3__1__m_teams__OFFSET: usize = 9968;
pub const CGameWar3__1__m_players__OFFSET: usize = 9992;
pub const CGameWar3__1__m_startLocations__OFFSET: usize = 10224;
pub const CGameWar3__1__m_slotToPlayerId__OFFSET: usize = 11960;
pub const CGameWar3__1__m_levelToTransitionTo__OFFSET: usize = 12064;
pub const CGameWar3__1__m_modelCine__OFFSET: usize = 12088;
pub const CGameWar3__1__m_cine__OFFSET: usize = 12112;
pub const CGameWar3__1__m_gameOverMessage__OFFSET: usize = 12376;
pub const CGameWar3__1__m_switchingLevels__OFFSET: usize = 12400;
pub const CGameWar3__1__m_doScoreScreen__OFFSET: usize = 12401;
pub const CGameWar3__1__m_isReload__OFFSET: usize = 12402;
pub const CGameWar3__1__anticheat_dummy_array_12__OFFSET: usize = 12403;
pub const CGameWar3__1__m_wasScriptLoad__OFFSET: usize = 12410;
pub const CGameWar3__1__m_wasLoadGame__OFFSET: usize = 12411;
pub const CGameWar3__1__m_isCampaignMission__OFFSET: usize = 12412;
pub const CGameWar3__1__m_campaignIndex__OFFSET: usize = 12416;
pub const CGameWar3__1__m_gameSetup__OFFSET: usize = 12420;
pub const CGameWar3__1__m_usingDarkMask__OFFSET: usize = 12678;
pub const CGameWar3__1__m_unseenQuests__OFFSET: usize = 12679;
pub const CGameWar3__1__m_showAllUnitsCmdBar__OFFSET: usize = 12680;
pub const CGameWar3__1__m_isReplay__OFFSET: usize = 12681;
pub const CGameWar3__1__m_replayFile__OFFSET: usize = 12688;
pub const CGameWar3__1__m_isTournament__OFFSET: usize = 12712;
pub const CGameWar3__1__m_copySaveCount__OFFSET: usize = 12716;
pub const CGameWar3__1__m_finishedInit__OFFSET: usize = 12720;
pub const CGameWar3__1__m_didSaveGameFromScript__OFFSET: usize = 12721;
pub const CGameWar3__1__m_playingGame__OFFSET: usize = 12722;
pub const CGameWar3__1__m_FacialAnimationSetScopeID__OFFSET: usize = 12724;
pub const CGameWar3__1__m_pTriggerStrings__OFFSET: usize = 12728;
pub const CGameWar3__1__anticheat_dummy_array_13__OFFSET: usize = 12736;
pub const CGameWar3__1__m_messageLogContents__OFFSET: usize = 12776;
pub const CGameWar3__1__m_pGameHashTableManager__OFFSET: usize = 12800;
pub const CGameWar3__1__m_gameCacheManager__OFFSET: usize = 12808;
pub const CGameWar3__1__m_gameOverCallback__OFFSET: usize = 12816;
pub const CGameWar3__1__m_maxCheckpointSaves__OFFSET: usize = 12824;
pub const CGameWar3__1__m_pSaveThread__OFFSET: usize = 12832;
pub const CGameWar3__1__m_pCheckpointPopup__OFFSET: usize = 12840;
pub const CGameWar3__1__m_gamePlayStartTime__OFFSET: usize = 12848;
pub const CGameWar3__1__m_latency__OFFSET: usize = 12856;

#[repr(C)]
pub struct MapPropsHeader {
    pub _opaque: [u8; 124],
}
pub const MapPropsHeader__SIZE: usize = 124;
pub const MapPropsHeader__NAME: &str = "MapPropsHeader";
pub const MapPropsHeader__identifier__OFFSET: usize = 0;
pub const MapPropsHeader__name__OFFSET: usize = 4;
pub const MapPropsHeader__flags__OFFSET: usize = 116;
pub const MapPropsHeader__numPlayerSlots__OFFSET: usize = 120;

#[repr(C)]
pub struct CUnitRefList {
    pub _opaque: [u8; 104],
}
pub const CUnitRefList__SIZE: usize = 104;
pub const CUnitRefList__NAME: &str = "CUnitRefList";
pub const CUnitRefList__s_CUnitRefList_pool__OFFSET: usize = 0;
pub const CUnitRefList__m_next__OFFSET: usize = 88;

#[repr(C)]
pub struct CUnit__2 {
    pub _opaque: [u8; 1864],
}
pub const CUnit__2__SIZE: usize = 1864;
pub const CUnit__2__NAME: &str = "CUnit";
pub const CUnit__2__s_CUnit_pool__OFFSET: usize = 0;
pub const CUnit__2__m_unit_paused_count__OFFSET: usize = 440;
pub const CUnit__2__m_PlayingDialogSound__OFFSET: usize = 444;
pub const CUnit__2__m_owner__OFFSET: usize = 448;
pub const CUnit__2__m_flags0__OFFSET: usize = 452;
pub const CUnit__2__m_flags1__OFFSET: usize = 456;
pub const CUnit__2__m_foodAndTechTreeFlags__OFFSET: usize = 460;
pub const CUnit__2__m_minimapFlashTimer__OFFSET: usize = 464;
pub const CUnit__2__m_damage_dealt__OFFSET: usize = 496;
pub const CUnit__2__m_self_damage__OFFSET: usize = 500;
pub const CUnit__2__m_healing_done__OFFSET: usize = 504;
pub const CUnit__2__m_time_stopped__OFFSET: usize = 508;
pub const CUnit__2__m_total_kills__OFFSET: usize = 512;
pub const CUnit__2__m_self_kills__OFFSET: usize = 516;
pub const CUnit__2__m_hero_kills__OFFSET: usize = 520;
pub const CUnit__2__m_building_kills__OFFSET: usize = 524;
pub const CUnit__2__m_override_ability_id_stats_credit__OFFSET: usize = 528;
pub const CUnit__2__m_override_item_id_stats_credit__OFFSET: usize = 532;
pub const CUnit__2__m_unit_stats_override__OFFSET: usize = 536;
pub const CUnit__2__m_caller__OFFSET: usize = 544;
pub const CUnit__2__m_lastAttackNotificationTime__OFFSET: usize = 576;
pub const CUnit__2__m_lastAttackNotificationSoundTime__OFFSET: usize = 580;
pub const CUnit__2__m_life__OFFSET: usize = 584;
pub const CUnit__2__m_death__OFFSET: usize = 608;
pub const CUnit__2__m_total_life_lost__OFFSET: usize = 616;
pub const CUnit__2__m_stored_time_when_last_alive__OFFSET: usize = 620;
pub const CUnit__2__m_time_alive_ms__OFFSET: usize = 624;
pub const CUnit__2__m_number_of_deaths__OFFSET: usize = 628;
pub const CUnit__2__m_life_regen_rate__OFFSET: usize = 632;
pub const CUnit__2__m_life_regen__OFFSET: usize = 648;
pub const CUnit__2__m_life_regen_type__OFFSET: usize = 656;
pub const CUnit__2__m_mana__OFFSET: usize = 664;
pub const CUnit__2__m_ai_mana__OFFSET: usize = 688;
pub const CUnit__2__m_mana_regen_rate__OFFSET: usize = 704;
pub const CUnit__2__m_mana_regen__OFFSET: usize = 720;
pub const CUnit__2__m_total_mana_spent__OFFSET: usize = 728;
pub const CUnit__2__m_defense__OFFSET: usize = 736;
pub const CUnit__2__m_battle_type__OFFSET: usize = 752;
pub const CUnit__2__m_invulnerable_count__OFFSET: usize = 756;
pub const CUnit__2__m_auto_attack_ignore_count__OFFSET: usize = 760;
pub const CUnit__2__m_fogFlash__OFFSET: usize = 764;
pub const CUnit__2__m_sight__OFFSET: usize = 776;
pub const CUnit__2__m_sightRangeCached__OFFSET: usize = 800;
pub const CUnit__2__m_sightMod__OFFSET: usize = 808;
pub const CUnit__2__m_invisibility_count__OFFSET: usize = 816;
pub const CUnit__2__m_invisibility_percent__OFFSET: usize = 824;
pub const CUnit__2__m_invis_modify__OFFSET: usize = 848;
pub const CUnit__2__m_invis_listen__OFFSET: usize = 856;
pub const CUnit__2__m_detectedData__OFFSET: usize = 864;
pub const CUnit__2__m_sharedVisionData__OFFSET: usize = 876;
pub const CUnit__2__m_sharedVisionDataCached__OFFSET: usize = 888;
pub const CUnit__2__m_sharedVisionTransitiveDataCached__OFFSET: usize = 892;
pub const CUnit__2__m_frost_count__OFFSET: usize = 896;
pub const CUnit__2__m_stone_count__OFFSET: usize = 900;
pub const CUnit__2__m_ethereal_count__OFFSET: usize = 904;
pub const CUnit__2__m_magic_immunity_count__OFFSET: usize = 908;
pub const CUnit__2__m_dispel_immunity_count__OFFSET: usize = 912;
pub const CUnit__2__m_fog_radius__OFFSET: usize = 920;
pub const CUnit__2__m_pos__OFFSET: usize = 936;
pub const CUnit__2__m_exp_level__OFFSET: usize = 960;
pub const CUnit__2__MaxNameLength__OFFSET: usize = 0;
pub const CUnit__2__m_unit_proper_name__OFFSET: usize = 964;
pub const CUnit__2__m_pick_order__OFFSET: usize = 1224;
pub const CUnit__2__m_task_stack__OFFSET: usize = 1228;
pub const CUnit__2__m_task_timer__OFFSET: usize = 1240;
pub const CUnit__2__m_action__OFFSET: usize = 1272;
pub const CUnit__2__m_uninterruptable_count__OFFSET: usize = 1276;
pub const CUnit__2__m_order_head__OFFSET: usize = 1280;
pub const CUnit__2__m_order_tail__OFFSET: usize = 1292;
pub const CUnit__2__order_count__OFFSET: usize = 1304;
pub const CUnit__2__m_gold_bounty_dice__OFFSET: usize = 1308;
pub const CUnit__2__m_gold_bounty_sides__OFFSET: usize = 1312;
pub const CUnit__2__m_gold_bounty_plus__OFFSET: usize = 1316;
pub const CUnit__2__m_wood_bounty_dice__OFFSET: usize = 1320;
pub const CUnit__2__m_wood_bounty_sides__OFFSET: usize = 1324;
pub const CUnit__2__m_wood_bounty_plus__OFFSET: usize = 1328;
pub const CUnit__2__m_cast_point__OFFSET: usize = 1336;
pub const CUnit__2__m_cast_back_swing__OFFSET: usize = 1352;
pub const CUnit__2__m_abilities__OFFSET: usize = 1368;
pub const CUnit__2__m_disabled_count__OFFSET: usize = 1380;
pub const CUnit__2__m_disabled_hidden_magic_count__OFFSET: usize = 1384;
pub const CUnit__2__m_silence_count__OFFSET: usize = 1388;
pub const CUnit__2__anticheat_dummy_array_14__OFFSET: usize = 1392;
pub const CUnit__2__m_disabled_magic_count__OFFSET: usize = 1404;
pub const CUnit__2__m_disabled_hidden_count__OFFSET: usize = 1408;
pub const CUnit__2__m_abil_paused_count__OFFSET: usize = 1412;
pub const CUnit__2__m_disabled_hidden_physical_count__OFFSET: usize = 1416;
pub const CUnit__2__m_disabled_physical_count__OFFSET: usize = 1420;
pub const CUnit__2__anticheat_dummy_array_15__OFFSET: usize = 1424;
pub const CUnit__2__m_buff_paused_count__OFFSET: usize = 1432;
pub const CUnit__2__m_abil_inv__OFFSET: usize = 1440;
pub const CUnit__2__m_abil_hero__OFFSET: usize = 1448;
pub const CUnit__2__m_abil_move__OFFSET: usize = 1456;
pub const CUnit__2__anticheat_dummy_array_16__OFFSET: usize = 1464;
pub const CUnit__2__m_abil_attack__OFFSET: usize = 1472;
pub const CUnit__2__m_abil_build__OFFSET: usize = 1480;
pub const CUnit__2__m_base_priority__OFFSET: usize = 1488;
pub const CUnit__2__m_death_time__OFFSET: usize = 1504;
pub const CUnit__2__s_minUnitSpeed__OFFSET: usize = 0;
pub const CUnit__2__s_maxUnitSpeed__OFFSET: usize = 0;
pub const CUnit__2__s_minBldgSpeed__OFFSET: usize = 0;
pub const CUnit__2__s_maxBldgSpeed__OFFSET: usize = 0;
pub const CUnit__2__s_buildingKillsGiveExp__OFFSET: usize = 0;
pub const CUnit__2__s_globalExperience__OFFSET: usize = 0;
pub const CUnit__2__s_maxLevelHeroesDrainExp__OFFSET: usize = 0;
pub const CUnit__2__m_display_height__OFFSET: usize = 1520;
pub const CUnit__2__m_display_height_modifier__OFFSET: usize = 1544;
pub const CUnit__2__anticheat_dummy_array_17__OFFSET: usize = 1552;
pub const CUnit__2__m_last_cluster_birth_tag__OFFSET: usize = 1560;
pub const CUnit__2__m_last_cluster_presence_tag__OFFSET: usize = 1564;
pub const CUnit__2__m_move_type__OFFSET: usize = 1568;
pub const CUnit__2__m_ground_count__OFFSET: usize = 1572;
pub const CUnit__2__m_launch_offset__OFFSET: usize = 1576;
pub const CUnit__2__m_fly_height__OFFSET: usize = 1592;
pub const CUnit__2__m_impact_swim__OFFSET: usize = 1616;
pub const CUnit__2__anticheat_dummy_array_18__OFFSET: usize = 1620;
pub const CUnit__2__m_launch_swim_z__OFFSET: usize = 1624;
pub const CUnit__2__m_impact_offset__OFFSET: usize = 1628;
pub const CUnit__2__m_category__OFFSET: usize = 1632;
pub const CUnit__2__m_target_type_flags__OFFSET: usize = 1636;
pub const CUnit__2__m_cargo_size__OFFSET: usize = 1640;
pub const CUnit__2__m_killedBy__OFFSET: usize = 1644;
pub const CUnit__2__m_shadow__OFFSET: usize = 1656;
pub const CUnit__2__m_shadowWater__OFFSET: usize = 1660;
pub const CUnit__2__anticheat_dummy_array_19__OFFSET: usize = 1664;
pub const CUnit__2__m_shadowAboveWater__OFFSET: usize = 1671;
pub const CUnit__2__m_shadowShow__OFFSET: usize = 1672;
pub const CUnit__2__m_showOccMark__OFFSET: usize = 1673;
pub const CUnit__2__m_allySelectionImage__OFFSET: usize = 1676;
pub const CUnit__2__m_occluded__OFFSET: usize = 1680;
pub const CUnit__2__anticheat_dummy_array_20__OFFSET: usize = 1681;
pub const CUnit__2__m_occlusionMark__OFFSET: usize = 1684;
pub const CUnit__2__m_jassUserData__OFFSET: usize = 1688;
pub const CUnit__2__m_foliageExclusionId__OFFSET: usize = 1692;
pub const CUnit__2__m_uiFlags__OFFSET: usize = 1696;
pub const CUnit__2__m_cachedPos__OFFSET: usize = 1700;
pub const CUnit__2__m_cachedFacing__OFFSET: usize = 1712;
pub const CUnit__2__m_cachedOrientation__OFFSET: usize = 1716;
pub const CUnit__2__m_lookAtTarget__OFFSET: usize = 1752;
pub const CUnit__2__m_lookAtOffset__OFFSET: usize = 1760;
pub const CUnit__2__m_overheadOffset__OFFSET: usize = 1772;
pub const CUnit__2__m_occupUI__OFFSET: usize = 1776;
pub const CUnit__2__m_unitStatusIconUI__OFFSET: usize = 1784;
pub const CUnit__2__m_unitStatusTextUI__OFFSET: usize = 1792;
pub const CUnit__2__m_uberSplatId__OFFSET: usize = 1800;
pub const CUnit__2__m_vertexColor__OFFSET: usize = 1804;
pub const CUnit__2__m_anim_size__OFFSET: usize = 1808;
pub const CUnit__2__m_anims__OFFSET: usize = 1812;
pub const CUnit__2__m_forcedAnimScale__OFFSET: usize = 1840;
pub const CUnit__2__m_walk_anim_speed__OFFSET: usize = 1844;
pub const CUnit__2__m_run_anim_speed__OFFSET: usize = 1848;
pub const CUnit__2__m_unit_ai__OFFSET: usize = 1852;

#[repr(C)]
pub struct CPlayerUpgrade {
    pub _opaque: [u8; 160],
}
pub const CPlayerUpgrade__SIZE: usize = 160;
pub const CPlayerUpgrade__NAME: &str = "CPlayerUpgrade";
pub const CPlayerUpgrade__s_CPlayerUpgrade_pool__OFFSET: usize = 0;
pub const CPlayerUpgrade__m_playerId__OFFSET: usize = 88;
pub const CPlayerUpgrade__m_upgradeId__OFFSET: usize = 92;
pub const CPlayerUpgrade__m_level__OFFSET: usize = 96;
pub const CPlayerUpgrade__m_unused__OFFSET: usize = 100;
pub const CPlayerUpgrade__m_race__OFFSET: usize = 104;
pub const CPlayerUpgrade__m_upgradeEffects__OFFSET: usize = 108;

#[repr(C)]
pub struct CUnitSkin {
    pub _opaque: [u8; 1264],
}
pub const CUnitSkin__SIZE: usize = 1264;
pub const CUnitSkin__NAME: &str = "CUnitSkin";
pub const CUnitSkin__m_name__OFFSET: usize = 104;
pub const CUnitSkin__m_description__OFFSET: usize = 128;
pub const CUnitSkin__m_modelPath__OFFSET: usize = 168;
pub const CUnitSkin__m_portraitModelPath__OFFSET: usize = 208;
pub const CUnitSkin__m_oldPortraitModelPath__OFFSET: usize = 248;
pub const CUnitSkin__m_missileModelPath__OFFSET: usize = 288;
pub const CUnitSkin__m_uberSplat__OFFSET: usize = 312;
pub const CUnitSkin__m_shadow__OFFSET: usize = 352;
pub const CUnitSkin__m_structureShadow__OFFSET: usize = 392;
pub const CUnitSkin__m_sound__OFFSET: usize = 432;
pub const CUnitSkin__m_scaleFactor__OFFSET: usize = 472;
pub const CUnitSkin__m_legacyScaleFactor__OFFSET: usize = 476;
pub const CUnitSkin__m_zOffset__OFFSET: usize = 480;
pub const CUnitSkin__m_rangedMissileArc__OFFSET: usize = 488;
pub const CUnitSkin__m_shadowOffset__OFFSET: usize = 512;
pub const CUnitSkin__m_shadowSize__OFFSET: usize = 520;
pub const CUnitSkin__m_shadowOnWater__OFFSET: usize = 528;
pub const CUnitSkin__m_selCircOnWater__OFFSET: usize = 529;
pub const CUnitSkin__m_maxPitch__OFFSET: usize = 532;
pub const CUnitSkin__m_maxRoll__OFFSET: usize = 536;
pub const CUnitSkin__m_elevPoints__OFFSET: usize = 540;
pub const CUnitSkin__m_elevRadius__OFFSET: usize = 544;
pub const CUnitSkin__m_armorType__OFFSET: usize = 548;
pub const CUnitSkin__m_modelColor__OFFSET: usize = 552;
pub const CUnitSkin__m_modelScale__OFFSET: usize = 556;
pub const CUnitSkin__m_legacyModelScale__OFFSET: usize = 560;
pub const CUnitSkin__m_walkAnimSpeed__OFFSET: usize = 564;
pub const CUnitSkin__m_runAnimSpeed__OFFSET: usize = 568;
pub const CUnitSkin__m_scaleBullet__OFFSET: usize = 572;
pub const CUnitSkin__m_animBlendTime__OFFSET: usize = 576;
pub const CUnitSkin__m_abilSkin__OFFSET: usize = 584;
pub const CUnitSkin__m_heroAbilSkin__OFFSET: usize = 608;
pub const CUnitSkin__m_awakenTip__OFFSET: usize = 632;
pub const CUnitSkin__m_reviveTip__OFFSET: usize = 656;
pub const CUnitSkin__m_properNames__OFFSET: usize = 680;
pub const CUnitSkin__m_animProps__OFFSET: usize = 704;
pub const CUnitSkin__m_attachmentAnimProps__OFFSET: usize = 728;
pub const CUnitSkin__m_attachmentLinkProps__OFFSET: usize = 752;
pub const CUnitSkin__m_boneProps__OFFSET: usize = 776;
pub const CUnitSkin__m_scoreScreenIcon__OFFSET: usize = 800;
pub const CUnitSkin__m_movementSoundLabel__OFFSET: usize = 840;
pub const CUnitSkin__m_buildingSoundLabel__OFFSET: usize = 880;
pub const CUnitSkin__m_fadeInRate__OFFSET: usize = 920;
pub const CUnitSkin__m_fadeOutRate__OFFSET: usize = 924;
pub const CUnitSkin__m_randomSoundLabel__OFFSET: usize = 928;
pub const CUnitSkin__m_casterUpgradeArt__OFFSET: usize = 968;
pub const CUnitSkin__m_casterUpgradeName__OFFSET: usize = 992;
pub const CUnitSkin__m_casterUpgradeTip__OFFSET: usize = 1016;
pub const CUnitSkin__m_specialArt__OFFSET: usize = 1040;
pub const CUnitSkin__m_buttonArt__OFFSET: usize = 1064;
pub const CUnitSkin__m_tooltip__OFFSET: usize = 1088;
pub const CUnitSkin__m_uberTooltip__OFFSET: usize = 1112;
pub const CUnitSkin__m_targetArt__OFFSET: usize = 1136;
pub const CUnitSkin__m_specialAttachProps__OFFSET: usize = 1160;
pub const CUnitSkin__m_projectileVisOffsetX__OFFSET: usize = 1188;
pub const CUnitSkin__m_projectileVisOffsetY__OFFSET: usize = 1192;
pub const CUnitSkin__m_launchOffsetZ__OFFSET: usize = 1196;
pub const CUnitSkin__m_launchSwimZ__OFFSET: usize = 1200;
pub const CUnitSkin__m_impactOffset__OFFSET: usize = 1204;
pub const CUnitSkin__m_impactSwimOffset__OFFSET: usize = 1208;
pub const CUnitSkin__m_showUI__OFFSET: usize = 1216;
pub const CUnitSkin__m_weapType__OFFSET: usize = 1240;

#[repr(C)]
pub struct UIHotkeys {
    pub _opaque: [u8; 158],
}
pub const UIHotkeys__SIZE: usize = 158;
pub const UIHotkeys__NAME: &str = "UIHotkeys";
pub const UIHotkeys__COMMANDBAR_ROW_NUM__OFFSET: usize = 0;
pub const UIHotkeys__COMMANDBAR_COLUMN_NUM__OFFSET: usize = 0;
pub const UIHotkeys__INVENTORYBAR_SLOT_NUM__OFFSET: usize = 0;
pub const UIHotkeys__CUSTOM_HOTKEY_SLOT_NUM__OFFSET: usize = 0;
pub const UIHotkeys__commandBarHotkeys__OFFSET: usize = 0;
pub const UIHotkeys__inventoryBarHotkeys__OFFSET: usize = 60;
pub const UIHotkeys__customHotkeys__OFFSET: usize = 90;
pub const UIHotkeys__customMouseHotkeys__OFFSET: usize = 146;

#[repr(C)]
pub struct CCameraWar3 {
    pub _opaque: [u8; 2216],
}
pub const CCameraWar3__SIZE: usize = 2216;
pub const CCameraWar3__NAME: &str = "CCameraWar3";
pub const CCameraWar3__s_CCameraWar3_pool__OFFSET: usize = 0;
pub const CCameraWar3__m_cameraNdx__OFFSET: usize = 56;
pub const CCameraWar3__m_defaultCameraDifferencePercentage__OFFSET: usize = 60;
pub const CCameraWar3__m_defaultCameraDistanceDifference__OFFSET: usize = 64;
pub const CCameraWar3__m_defaultCameraNdx__OFFSET: usize = 68;
pub const CCameraWar3__m_defaultPrefCameraDist__OFFSET: usize = 72;
pub const CCameraWar3__m_maxCameraDist__OFFSET: usize = 76;
pub const CCameraWar3__m_rotationNdx__OFFSET: usize = 80;
pub const CCameraWar3__m_rollNdx__OFFSET: usize = 84;
pub const CCameraWar3__m_localPitchNdx__OFFSET: usize = 88;
pub const CCameraWar3__m_localYawNdx__OFFSET: usize = 92;
pub const CCameraWar3__m_localRollNdx__OFFSET: usize = 96;
pub const CCameraWar3__m_wheelPos__OFFSET: usize = 100;
pub const CCameraWar3__m_lock__OFFSET: usize = 104;
pub const CCameraWar3__m_disableMouseScroll__OFFSET: usize = 105;
pub const CCameraWar3__m_enableMouseWheel__OFFSET: usize = 106;
pub const CCameraWar3__m_currentCamera__OFFSET: usize = 112;
pub const CCameraWar3__m_currentMode__OFFSET: usize = 120;
pub const CCameraWar3__m_isCinematicAudio__OFFSET: usize = 124;
pub const CCameraWar3__m_listener__OFFSET: usize = 128;
pub const CCameraWar3__m_camera__OFFSET: usize = 136;
pub const CCameraWar3__m_targetController__OFFSET: usize = 144;
pub const CCameraWar3__m_targetControllerInheritOrientation__OFFSET: usize = 152;
pub const CCameraWar3__m_targetControllerOrientationOnly__OFFSET: usize = 153;
pub const CCameraWar3__m_targetControllerOffset__OFFSET: usize = 156;
pub const CCameraWar3__m_cinematicCameraSprite__OFFSET: usize = 168;
pub const CCameraWar3__m_cinematicOrientation__OFFSET: usize = 176;
pub const CCameraWar3__m_timer__OFFSET: usize = 216;
pub const CCameraWar3__m_lastQT__OFFSET: usize = 248;
pub const CCameraWar3__m_lastQTTime__OFFSET: usize = 252;
pub const CCameraWar3__m_QTHead__OFFSET: usize = 256;
pub const CCameraWar3__m_activeQTCount__OFFSET: usize = 260;
pub const CCameraWar3__m_quickTargets__OFFSET: usize = 264;
pub const CCameraWar3__m_lastTH__OFFSET: usize = 288;
pub const CCameraWar3__m_lastTHTime__OFFSET: usize = 292;
pub const CCameraWar3__m_townhalls__OFFSET: usize = 296;
pub const CCameraWar3__m_listenerDistance__OFFSET: usize = 320;
pub const CCameraWar3__m_listenerDistanceMod__OFFSET: usize = 344;
pub const CCameraWar3__m_listenerDistanceSmoothMod__OFFSET: usize = 360;
pub const CCameraWar3__m_listenerAngleCorrection__OFFSET: usize = 376;
pub const CCameraWar3__m_listenerAngleCorrectionMod__OFFSET: usize = 400;
pub const CCameraWar3__m_listenerAngleCorrectionSmoothMod__OFFSET: usize = 416;
pub const CCameraWar3__m_listenerAOA__OFFSET: usize = 432;
pub const CCameraWar3__m_listenerAOAMod__OFFSET: usize = 456;
pub const CCameraWar3__m_listenerAOASmoothMod__OFFSET: usize = 472;
pub const CCameraWar3__m_distance__OFFSET: usize = 488;
pub const CCameraWar3__m_distanceMod__OFFSET: usize = 512;
pub const CCameraWar3__m_distanceFactor__OFFSET: usize = 528;
pub const CCameraWar3__m_distanceSmoothMod__OFFSET: usize = 536;
pub const CCameraWar3__m_farZ__OFFSET: usize = 552;
pub const CCameraWar3__m_farZMod__OFFSET: usize = 576;
pub const CCameraWar3__m_farZSmoothMod__OFFSET: usize = 592;
pub const CCameraWar3__m_nearZ__OFFSET: usize = 608;
pub const CCameraWar3__m_nearZMod__OFFSET: usize = 632;
pub const CCameraWar3__m_nearZSmoothMod__OFFSET: usize = 648;
pub const CCameraWar3__m_fov__OFFSET: usize = 664;
pub const CCameraWar3__m_fovMod__OFFSET: usize = 688;
pub const CCameraWar3__m_fovSmoothMod__OFFSET: usize = 704;
pub const CCameraWar3__m_rotation__OFFSET: usize = 720;
pub const CCameraWar3__m_rotationMod__OFFSET: usize = 744;
pub const CCameraWar3__m_rotationSmoothMod__OFFSET: usize = 760;
pub const CCameraWar3__m_aoa__OFFSET: usize = 776;
pub const CCameraWar3__m_aoaMod__OFFSET: usize = 800;
pub const CCameraWar3__m_aoaSmoothMod__OFFSET: usize = 816;
pub const CCameraWar3__m_roll__OFFSET: usize = 832;
pub const CCameraWar3__m_rollMod__OFFSET: usize = 856;
pub const CCameraWar3__m_rollSmoothMod__OFFSET: usize = 872;
pub const CCameraWar3__m_localPitch__OFFSET: usize = 888;
pub const CCameraWar3__m_localPitchMod__OFFSET: usize = 912;
pub const CCameraWar3__m_localPitchSmoothMod__OFFSET: usize = 928;
pub const CCameraWar3__m_localYaw__OFFSET: usize = 944;
pub const CCameraWar3__m_localYawMod__OFFSET: usize = 968;
pub const CCameraWar3__m_localYawSmoothMod__OFFSET: usize = 984;
pub const CCameraWar3__m_localRoll__OFFSET: usize = 1000;
pub const CCameraWar3__m_localRollMod__OFFSET: usize = 1024;
pub const CCameraWar3__m_localRollSmoothMod__OFFSET: usize = 1040;
pub const CCameraWar3__m_target__OFFSET: usize = 1056;
pub const CCameraWar3__m_targetZOffset__OFFSET: usize = 1080;
pub const CCameraWar3__m_targetZOffsetMod__OFFSET: usize = 1104;
pub const CCameraWar3__m_targetZOffsetSmoothMod__OFFSET: usize = 1112;
pub const CCameraWar3__m_panning__OFFSET: usize = 1120;
pub const CCameraWar3__m_panInterpZ__OFFSET: usize = 1121;
pub const CCameraWar3__m_panZ__OFFSET: usize = 1128;
pub const CCameraWar3__m_panZMod__OFFSET: usize = 1152;
pub const CCameraWar3__m_panZSmoothMod__OFFSET: usize = 1160;
pub const CCameraWar3__m_doTargetNoise__OFFSET: usize = 1168;
pub const CCameraWar3__m_doTargetNoiseVertOnly__OFFSET: usize = 1169;
pub const CCameraWar3__m_targetNoiseDir__OFFSET: usize = 1172;
pub const CCameraWar3__m_targetNoiseMag__OFFSET: usize = 1320;
pub const CCameraWar3__m_doSourceNoise__OFFSET: usize = 1468;
pub const CCameraWar3__m_doSourceNoiseVertOnly__OFFSET: usize = 1469;
pub const CCameraWar3__m_sourceNoiseDir__OFFSET: usize = 1472;
pub const CCameraWar3__m_sourceNoiseMag__OFFSET: usize = 1620;
pub const CCameraWar3__m_horMove__OFFSET: usize = 1768;
pub const CCameraWar3__m_verMove__OFFSET: usize = 1784;
pub const CCameraWar3__m_panMove__OFFSET: usize = 1800;
pub const CCameraWar3__m_panSmoothMove__OFFSET: usize = 1808;
pub const CCameraWar3__m_position__OFFSET: usize = 1816;
pub const CCameraWar3__m_lastListenerPos__OFFSET: usize = 1840;
pub const CCameraWar3__m_centerOnUnitKey__OFFSET: usize = 1852;
pub const CCameraWar3__m_resetZoomKey__OFFSET: usize = 1856;
pub const CCameraWar3__m_rotModeCenter__OFFSET: usize = 1860;
pub const CCameraWar3__m_rotModeTargOffset__OFFSET: usize = 1872;
pub const CCameraWar3__m_rotModeEyeOffset__OFFSET: usize = 1884;
pub const CCameraWar3__m_rotModeAngle__OFFSET: usize = 1896;
pub const CCameraWar3__m_rotModeInitialRot__OFFSET: usize = 1920;
pub const CCameraWar3__m_rotModeRate__OFFSET: usize = 1928;
pub const CCameraWar3__m_startPos__OFFSET: usize = 1936;
pub const CCameraWar3__m_boundRect__OFFSET: usize = 1944;
pub const CCameraWar3__m_boundPoints__OFFSET: usize = 1960;
pub const CCameraWar3__m_cameraGuardBand__OFFSET: usize = 1992;
pub const CCameraWar3__m_mouseSpeedFactor__OFFSET: usize = 2008;
pub const CCameraWar3__m_keyboardSpeedFactor__OFFSET: usize = 2012;
pub const CCameraWar3__m_worldToCamBasis__OFFSET: usize = 2016;
pub const CCameraWar3__m_worldToCam__OFFSET: usize = 2052;
pub const CCameraWar3__m_camToWorld__OFFSET: usize = 2088;
pub const CCameraWar3__m_filter__OFFSET: usize = 2124;
pub const CCameraWar3__m_heights__OFFSET: usize = 2160;
pub const CCameraWar3__m_elapsed__OFFSET: usize = 2172;
pub const CCameraWar3__m_focalDistance__OFFSET: usize = 2176;
pub const CCameraWar3__m_dofScale__OFFSET: usize = 2180;
pub const CCameraWar3__m_dofEnabled__OFFSET: usize = 2184;
pub const CCameraWar3__s_listenerNeedsUpdate__OFFSET: usize = 0;
pub const CCameraWar3__savedTarget__OFFSET: usize = 2188;
pub const CCameraWar3__savedDistance__OFFSET: usize = 2200;
pub const CCameraWar3__savedAOA__OFFSET: usize = 2204;
pub const CCameraWar3__savedRotation__OFFSET: usize = 2208;
pub const CCameraWar3__m_smoothingCoef__OFFSET: usize = 2212;

#[repr(C)]
pub struct GameAttributeV1Impl {
    pub _opaque: [u8; 88],
}
pub const GameAttributeV1Impl__SIZE: usize = 88;
pub const GameAttributeV1Impl__NAME: &str = "GameAttributeV1Impl";
pub const GameAttributeV1Impl__m_key__OFFSET: usize = 8;
pub const GameAttributeV1Impl__m_value__OFFSET: usize = 48;

#[repr(C)]
pub struct CUnitUIManager {
    pub _opaque: [u8; 72],
}
pub const CUnitUIManager__SIZE: usize = 72;
pub const CUnitUIManager__NAME: &str = "CUnitUIManager";
pub const CUnitUIManager__s_customKeysEnabled__OFFSET: usize = 0;
pub const CUnitUIManager__s_unitUIManager__OFFSET: usize = 0;
pub const CUnitUIManager__m_stringUtil__OFFSET: usize = 0;
pub const CUnitUIManager__m_unitUIRegistry__OFFSET: usize = 8;
pub const CUnitUIManager__m_abilityUIRegistry__OFFSET: usize = 40;

#[repr(C)]
pub struct CGameUI {
    pub _opaque: [u8; 2384],
}
pub const CGameUI__SIZE: usize = 2384;
pub const CGameUI__NAME: &str = "CGameUI";
pub const CGameUI__m_transitionState__OFFSET: usize = 704;
pub const CGameUI__m_cachedFonts__OFFSET: usize = 712;
pub const CGameUI__NUMBER_OF_PLAYER_ROWS__OFFSET: usize = 0;
pub const CGameUI__FIRST_ROW_Y_OFFSET__OFFSET: usize = 0;
pub const CGameUI__ROW_SPACING__OFFSET: usize = 0;
pub const CGameUI__m_userControl__OFFSET: usize = 784;
pub const CGameUI__m_userUI__OFFSET: usize = 785;
pub const CGameUI__m_currentInputMode__OFFSET: usize = 792;
pub const CGameUI__m_inputModeStack__OFFSET: usize = 800;
pub const CGameUI__m_sharedTooltip__OFFSET: usize = 824;
pub const CGameUI__m_uberTooltip__OFFSET: usize = 832;
pub const CGameUI__m_showUberTips__OFFSET: usize = 840;
pub const CGameUI__m_autosaveReplay__OFFSET: usize = 841;
pub const CGameUI__m_healthBarsMode__OFFSET: usize = 844;
pub const CGameUI__m_forceShowHealthBars__OFFSET: usize = 848;
pub const CGameUI__m_minimapSignals__OFFSET: usize = 849;
pub const CGameUI__m_useColoredHealthBars__OFFSET: usize = 850;
pub const CGameUI__m_useHeroFrames__OFFSET: usize = 851;
pub const CGameUI__m_showHeroLevelFrame__OFFSET: usize = 852;
pub const CGameUI__m_showNumericCooldown__OFFSET: usize = 853;
pub const CGameUI__m_showDenyIcon__OFFSET: usize = 854;
pub const CGameUI__m_showGoldmineUnitCounter__OFFSET: usize = 855;
pub const CGameUI__m_enableObserverUiV2__OFFSET: usize = 856;
pub const CGameUI__m_asyncPlayerData__OFFSET: usize = 860;
pub const CGameUI__m_targetMode__OFFSET: usize = 960;
pub const CGameUI__m_selectMode__OFFSET: usize = 968;
pub const CGameUI__m_dragSelectMode__OFFSET: usize = 976;
pub const CGameUI__m_dragScrollMode__OFFSET: usize = 984;
pub const CGameUI__m_buildMode__OFFSET: usize = 992;
pub const CGameUI__m_signalMode__OFFSET: usize = 1000;
pub const CGameUI__m_escapeMenu__OFFSET: usize = 1008;
pub const CGameUI__m_gameOverMenu__OFFSET: usize = 1016;
pub const CGameUI__m_suspendIndicator__OFFSET: usize = 1024;
pub const CGameUI__m_unresponsiveIndicator__OFFSET: usize = 1032;
pub const CGameUI__m_allianceMode__OFFSET: usize = 1040;
pub const CGameUI__m_chatMode__OFFSET: usize = 1048;
pub const CGameUI__m_questMode__OFFSET: usize = 1056;
pub const CGameUI__anticheat_dummy_array_14__OFFSET: usize = 1064;
pub const CGameUI__m_scriptDialogMode__OFFSET: usize = 1152;
pub const CGameUI__m_unresponsiveDelayTimerId__OFFSET: usize = 1160;
pub const CGameUI__m_selectionManager__OFFSET: usize = 1168;
pub const CGameUI__m_dragScrollManager__OFFSET: usize = 1176;
pub const CGameUI__m_camera__OFFSET: usize = 1184;
pub const CGameUI__m_paused__OFFSET: usize = 1192;
pub const CGameUI__m_uiPaused__OFFSET: usize = 1193;
pub const CGameUI__m_scriptPaused__OFFSET: usize = 1194;
pub const CGameUI__m_disableUpkeepMsg__OFFSET: usize = 1195;
pub const CGameUI__m_upkeepLvlSave__OFFSET: usize = 1196;
pub const CGameUI__m_refresh__OFFSET: usize = 1200;
pub const CGameUI__m_refreshWaypoints__OFFSET: usize = 1201;
pub const CGameUI__m_deferCleanSelUI__OFFSET: usize = 1202;
pub const CGameUI__m_refreshInfoBar__OFFSET: usize = 1203;
pub const CGameUI__m_refreshCommandBar__OFFSET: usize = 1204;
pub const CGameUI__m_refreshHeroBar__OFFSET: usize = 1205;
pub const CGameUI__m_popSubMenu__OFFSET: usize = 1206;
pub const CGameUI__m_activeValidOrder__OFFSET: usize = 1208;
pub const CGameUI__m_cineForceSubtitles__OFFSET: usize = 1212;
pub const CGameUI__m_cinematicPanelsHidden__OFFSET: usize = 1213;
pub const CGameUI__m_dragSelectEnabled__OFFSET: usize = 1214;
pub const CGameUI__m_dragSelectUIEnabled__OFFSET: usize = 1215;
pub const CGameUI__m_preSelectEnabled__OFFSET: usize = 1216;
pub const CGameUI__m_preSelectUIEnabled__OFFSET: usize = 1217;
pub const CGameUI__m_selectEnabled__OFFSET: usize = 1218;
pub const CGameUI__m_selectUIEnabled__OFFSET: usize = 1219;
pub const CGameUI__m_observerModeEnabled__OFFSET: usize = 1220;
pub const CGameUI__m_observerModeV2Active__OFFSET: usize = 1221;
pub const CGameUI__m_wideScreenAutoPosition__OFFSET: usize = 1222;
pub const CGameUI__m_fadeTimer__OFFSET: usize = 1224;
pub const CGameUI__m_cinematicFadeTimer__OFFSET: usize = 1328;
pub const CGameUI__anticheat_dummy_array_15__OFFSET: usize = 1432;
pub const CGameUI__m_frameParentTimer__OFFSET: usize = 1440;
pub const CGameUI__m_simpleFadeTimer__OFFSET: usize = 1544;
pub const CGameUI__m_attackNotify__OFFSET: usize = 1552;
pub const CGameUI__m_warpNotify__OFFSET: usize = 1576;
pub const CGameUI__m_timerDialogs__OFFSET: usize = 1608;
pub const CGameUI__m_leaderboard__OFFSET: usize = 1632;
pub const CGameUI__anticheat_dummy_array_16__OFFSET: usize = 1640;
pub const CGameUI__m_multiboard__OFFSET: usize = 1648;
pub const CGameUI__m_suspendMultiboardDisplay__OFFSET: usize = 1656;
pub const CGameUI__anticheat_dummy_array_17__OFFSET: usize = 1657;
pub const CGameUI__m_scriptDialog__OFFSET: usize = 1728;
pub const CGameUI__m_replaySelection__OFFSET: usize = 1736;
pub const CGameUI__m_unitStatusIconUIFactory__OFFSET: usize = 1744;
pub const CGameUI__m_unitStatusTexts__OFFSET: usize = 1752;
pub const CGameUI__m_worldFrame__OFFSET: usize = 1776;
pub const CGameUI__m_infoBar__OFFSET: usize = 1784;
pub const CGameUI__m_commandBar__OFFSET: usize = 1792;
pub const CGameUI__m_miniMap__OFFSET: usize = 1800;
pub const CGameUI__m_resourceBar__OFFSET: usize = 1808;
pub const CGameUI__m_upperButtonBar__OFFSET: usize = 1816;
pub const CGameUI__m_observerButtonBar__OFFSET: usize = 1824;
pub const CGameUI__m_playerStatisticsPanel__OFFSET: usize = 1832;
pub const CGameUI__m_heroPanel__OFFSET: usize = 1840;
pub const CGameUI__m_productionPanel__OFFSET: usize = 1848;
pub const CGameUI__m_minimapButtonBar__OFFSET: usize = 1856;
pub const CGameUI__m_trackingStub__OFFSET: usize = 1864;
pub const CGameUI__m_heroBar__OFFSET: usize = 1872;
pub const CGameUI__m_peonBar__OFFSET: usize = 1880;
pub const CGameUI__m_messageBox__OFFSET: usize = 1888;
pub const CGameUI__m_unitMessageBox__OFFSET: usize = 1896;
pub const CGameUI__m_chatMessageBox__OFFSET: usize = 1904;
pub const CGameUI__m_topMessageBox__OFFSET: usize = 1912;
pub const CGameUI__m_currentPortraitButton__OFFSET: usize = 1920;
pub const CGameUI__m_timeOfDayIndicator__OFFSET: usize = 1928;
pub const CGameUI__m_chatEditBar__OFFSET: usize = 1936;
pub const CGameUI__m_cinematicPanel__OFFSET: usize = 1944;
pub const CGameUI__m_lastBuffBar__OFFSET: usize = 1952;
pub const CGameUI__m_observerBar__OFFSET: usize = 1960;
pub const CGameUI__m_miniMapSignalButton__OFFSET: usize = 1968;
pub const CGameUI__m_miniMapTerrainButton__OFFSET: usize = 1976;
pub const CGameUI__m_miniMapAllyButton__OFFSET: usize = 1984;
pub const CGameUI__m_miniMapCreepButton__OFFSET: usize = 1992;
pub const CGameUI__m_formationButton__OFFSET: usize = 2000;
pub const CGameUI__m_consoleFrame__OFFSET: usize = 2008;
pub const CGameUI__m_simpleConsoleFrame__OFFSET: usize = 2016;
pub const CGameUI__m_simpleConsoleBottomBarFrame__OFFSET: usize = 2024;
pub const CGameUI__m_simpleConsoleBottomBarFrameOverlay__OFFSET: usize = 2032;
pub const CGameUI__m_frameUIParent__OFFSET: usize = 2040;
pub const CGameUI__top__OFFSET: usize = 2048;
pub const CGameUI__topInGame__OFFSET: usize = 2056;
pub const CGameUI__m_simpleUIParent__OFFSET: usize = 2064;
pub const CGameUI__m_observerUIParent__OFFSET: usize = 2072;
pub const CGameUI__m_observerInfoPanelBackground__OFFSET: usize = 2080;
pub const CGameUI__m_observerInfoPanelBackpack__OFFSET: usize = 2088;
pub const CGameUI__m_consoleBackdropFrame__OFFSET: usize = 2096;
pub const CGameUI__m_minimapTextureBackground__OFFSET: usize = 2104;
pub const CGameUI__m_infoPanelTexture__OFFSET: usize = 2112;
pub const CGameUI__m_quickSaveKey__OFFSET: usize = 2120;
pub const CGameUI__m_quickLoadKey__OFFSET: usize = 2124;
pub const CGameUI__m_quickHelpKey__OFFSET: usize = 2128;
pub const CGameUI__m_quickOptionsKey__OFFSET: usize = 2132;
pub const CGameUI__m_quickQuitKey__OFFSET: usize = 2136;
pub const CGameUI__m_minimapSignalKey__OFFSET: usize = 2140;
pub const CGameUI__m_minimapTerrainKey__OFFSET: usize = 2144;
pub const CGameUI__m_minimapColorsKey__OFFSET: usize = 2148;
pub const CGameUI__m_minimapCreepKey__OFFSET: usize = 2152;
pub const CGameUI__m_formationToggleKey__OFFSET: usize = 2156;
pub const CGameUI__m_observerToggleAllPanelsKey__OFFSET: usize = 2160;
pub const CGameUI__m_observerProductionModeKey__OFFSET: usize = 2164;
pub const CGameUI__m_observerUnitsModeKey__OFFSET: usize = 2168;
pub const CGameUI__m_mouseDownHandler__OFFSET: usize = 2176;
pub const CGameUI__m_mouseWheelHandler__OFFSET: usize = 2184;
pub const CGameUI__m_mouseUpHandler__OFFSET: usize = 2192;
pub const CGameUI__m_mouseMoveHandler__OFFSET: usize = 2200;
pub const CGameUI__m_observerToggleablePanels__OFFSET: usize = 2208;
pub const CGameUI__m_consoleBackdropHeight__OFFSET: usize = 2240;
pub const CGameUI__m_auroraChatHandler__OFFSET: usize = 2248;
pub const CGameUI__m_lobbyEventsHandler__OFFSET: usize = 2328;
pub const CGameUI__s_indexedCustomHotkeyMappingMaxCount__OFFSET: usize = 0;
pub const CGameUI__s_customHotkeyEnabled__OFFSET: usize = 0;
pub const CGameUI__s_customHotkeyMappingTable__OFFSET: usize = 0;
pub const CGameUI__s_customHotkeyMappingTableNonHero__OFFSET: usize = 0;
pub const CGameUI__m_observerUnitToFollow__OFFSET: usize = 2376;

#[repr(C)]
pub struct WorldShadowParams {
    pub _opaque: [u8; 40],
}
pub const WorldShadowParams__SIZE: usize = 40;
pub const WorldShadowParams__NAME: &str = "WorldShadowParams";
pub const WorldShadowParams__numCinematicCascades__OFFSET: usize = 0;
pub const WorldShadowParams__numGameCascades__OFFSET: usize = 4;
pub const WorldShadowParams__numPortraitCascades__OFFSET: usize = 8;
pub const WorldShadowParams__debug__OFFSET: usize = 12;
pub const WorldShadowParams__show__OFFSET: usize = 13;
pub const WorldShadowParams__cinematicDistances__OFFSET: usize = 16;
pub const WorldShadowParams__gameDistances__OFFSET: usize = 24;
pub const WorldShadowParams__portraitDistances__OFFSET: usize = 32;

#[repr(C)]
pub struct OrderTypeReg {
    pub _opaque: [u8; 56],
}
pub const OrderTypeReg__SIZE: usize = 56;
pub const OrderTypeReg__NAME: &str = "OrderTypeReg";
pub const OrderTypeReg__orderType__OFFSET: usize = 48;

#[repr(C)]
pub struct GameAccountService {
    pub _opaque: [u8; 32],
}
pub const GameAccountService__SIZE: usize = 32;
pub const GameAccountService__NAME: &str = "GameAccountService";
pub const GameAccountService__m_account__OFFSET: usize = 8;
pub const GameAccountService__m_loggedInToodId__OFFSET: usize = 16;

#[repr(C)]
pub struct GameVersionService {
    pub _opaque: [u8; 16],
}
pub const GameVersionService__SIZE: usize = 16;
pub const GameVersionService__NAME: &str = "GameVersionService";
pub const GameVersionService__m_version__OFFSET: usize = 8;

#[repr(C)]
pub struct GameConnectionService {
    pub _opaque: [u8; 16],
}
pub const GameConnectionService__SIZE: usize = 16;
pub const GameConnectionService__NAME: &str = "GameConnectionService";
pub const GameConnectionService__m_gameConnection__OFFSET: usize = 8;

#[repr(C)]
pub struct CUnitDB {
    pub _opaque: [u8; 648],
}
pub const CUnitDB__SIZE: usize = 648;
pub const CUnitDB__NAME: &str = "CUnitDB";

#[repr(C)]
pub struct UINT128 {
    pub _opaque: [u8; 16],
}
pub const UINT128__SIZE: usize = 16;
pub const UINT128__NAME: &str = "UINT128";
pub const UINT128__low__OFFSET: usize = 0;
pub const UINT128__high__OFFSET: usize = 8;

#[repr(C)]
pub struct CGameUI__1 {
    pub _opaque: [u8; 2384],
}
pub const CGameUI__1__SIZE: usize = 2384;
pub const CGameUI__1__NAME: &str = "CGameUI";
pub const CGameUI__1__m_transitionState__OFFSET: usize = 704;
pub const CGameUI__1__m_cachedFonts__OFFSET: usize = 712;
pub const CGameUI__1__NUMBER_OF_PLAYER_ROWS__OFFSET: usize = 0;
pub const CGameUI__1__FIRST_ROW_Y_OFFSET__OFFSET: usize = 0;
pub const CGameUI__1__ROW_SPACING__OFFSET: usize = 0;
pub const CGameUI__1__m_userControl__OFFSET: usize = 784;
pub const CGameUI__1__m_userUI__OFFSET: usize = 785;
pub const CGameUI__1__m_currentInputMode__OFFSET: usize = 792;
pub const CGameUI__1__m_inputModeStack__OFFSET: usize = 800;
pub const CGameUI__1__m_sharedTooltip__OFFSET: usize = 824;
pub const CGameUI__1__m_uberTooltip__OFFSET: usize = 832;
pub const CGameUI__1__m_showUberTips__OFFSET: usize = 840;
pub const CGameUI__1__m_autosaveReplay__OFFSET: usize = 841;
pub const CGameUI__1__m_healthBarsMode__OFFSET: usize = 844;
pub const CGameUI__1__m_forceShowHealthBars__OFFSET: usize = 848;
pub const CGameUI__1__m_minimapSignals__OFFSET: usize = 849;
pub const CGameUI__1__m_useColoredHealthBars__OFFSET: usize = 850;
pub const CGameUI__1__m_useHeroFrames__OFFSET: usize = 851;
pub const CGameUI__1__m_showHeroLevelFrame__OFFSET: usize = 852;
pub const CGameUI__1__m_showNumericCooldown__OFFSET: usize = 853;
pub const CGameUI__1__m_showDenyIcon__OFFSET: usize = 854;
pub const CGameUI__1__m_showGoldmineUnitCounter__OFFSET: usize = 855;
pub const CGameUI__1__m_enableObserverUiV2__OFFSET: usize = 856;
pub const CGameUI__1__m_asyncPlayerData__OFFSET: usize = 860;
pub const CGameUI__1__m_targetMode__OFFSET: usize = 960;
pub const CGameUI__1__m_selectMode__OFFSET: usize = 968;
pub const CGameUI__1__m_dragSelectMode__OFFSET: usize = 976;
pub const CGameUI__1__m_dragScrollMode__OFFSET: usize = 984;
pub const CGameUI__1__m_buildMode__OFFSET: usize = 992;
pub const CGameUI__1__m_signalMode__OFFSET: usize = 1000;
pub const CGameUI__1__m_escapeMenu__OFFSET: usize = 1008;
pub const CGameUI__1__m_gameOverMenu__OFFSET: usize = 1016;
pub const CGameUI__1__m_suspendIndicator__OFFSET: usize = 1024;
pub const CGameUI__1__m_unresponsiveIndicator__OFFSET: usize = 1032;
pub const CGameUI__1__m_allianceMode__OFFSET: usize = 1040;
pub const CGameUI__1__m_chatMode__OFFSET: usize = 1048;
pub const CGameUI__1__m_questMode__OFFSET: usize = 1056;
pub const CGameUI__1__anticheat_dummy_array_11__OFFSET: usize = 1064;
pub const CGameUI__1__m_scriptDialogMode__OFFSET: usize = 1152;
pub const CGameUI__1__m_unresponsiveDelayTimerId__OFFSET: usize = 1160;
pub const CGameUI__1__m_selectionManager__OFFSET: usize = 1168;
pub const CGameUI__1__m_dragScrollManager__OFFSET: usize = 1176;
pub const CGameUI__1__m_camera__OFFSET: usize = 1184;
pub const CGameUI__1__m_paused__OFFSET: usize = 1192;
pub const CGameUI__1__m_uiPaused__OFFSET: usize = 1193;
pub const CGameUI__1__m_scriptPaused__OFFSET: usize = 1194;
pub const CGameUI__1__m_disableUpkeepMsg__OFFSET: usize = 1195;
pub const CGameUI__1__m_upkeepLvlSave__OFFSET: usize = 1196;
pub const CGameUI__1__m_refresh__OFFSET: usize = 1200;
pub const CGameUI__1__m_refreshWaypoints__OFFSET: usize = 1201;
pub const CGameUI__1__m_deferCleanSelUI__OFFSET: usize = 1202;
pub const CGameUI__1__m_refreshInfoBar__OFFSET: usize = 1203;
pub const CGameUI__1__m_refreshCommandBar__OFFSET: usize = 1204;
pub const CGameUI__1__m_refreshHeroBar__OFFSET: usize = 1205;
pub const CGameUI__1__m_popSubMenu__OFFSET: usize = 1206;
pub const CGameUI__1__m_activeValidOrder__OFFSET: usize = 1208;
pub const CGameUI__1__m_cineForceSubtitles__OFFSET: usize = 1212;
pub const CGameUI__1__m_cinematicPanelsHidden__OFFSET: usize = 1213;
pub const CGameUI__1__m_dragSelectEnabled__OFFSET: usize = 1214;
pub const CGameUI__1__m_dragSelectUIEnabled__OFFSET: usize = 1215;
pub const CGameUI__1__m_preSelectEnabled__OFFSET: usize = 1216;
pub const CGameUI__1__m_preSelectUIEnabled__OFFSET: usize = 1217;
pub const CGameUI__1__m_selectEnabled__OFFSET: usize = 1218;
pub const CGameUI__1__m_selectUIEnabled__OFFSET: usize = 1219;
pub const CGameUI__1__m_observerModeEnabled__OFFSET: usize = 1220;
pub const CGameUI__1__m_observerModeV2Active__OFFSET: usize = 1221;
pub const CGameUI__1__m_wideScreenAutoPosition__OFFSET: usize = 1222;
pub const CGameUI__1__m_fadeTimer__OFFSET: usize = 1224;
pub const CGameUI__1__m_cinematicFadeTimer__OFFSET: usize = 1328;
pub const CGameUI__1__anticheat_dummy_array_12__OFFSET: usize = 1432;
pub const CGameUI__1__m_frameParentTimer__OFFSET: usize = 1440;
pub const CGameUI__1__m_simpleFadeTimer__OFFSET: usize = 1544;
pub const CGameUI__1__m_attackNotify__OFFSET: usize = 1552;
pub const CGameUI__1__m_warpNotify__OFFSET: usize = 1576;
pub const CGameUI__1__m_timerDialogs__OFFSET: usize = 1608;
pub const CGameUI__1__m_leaderboard__OFFSET: usize = 1632;
pub const CGameUI__1__anticheat_dummy_array_13__OFFSET: usize = 1640;
pub const CGameUI__1__m_multiboard__OFFSET: usize = 1648;
pub const CGameUI__1__m_suspendMultiboardDisplay__OFFSET: usize = 1656;
pub const CGameUI__1__anticheat_dummy_array_14__OFFSET: usize = 1657;
pub const CGameUI__1__m_scriptDialog__OFFSET: usize = 1728;
pub const CGameUI__1__m_replaySelection__OFFSET: usize = 1736;
pub const CGameUI__1__m_unitStatusIconUIFactory__OFFSET: usize = 1744;
pub const CGameUI__1__m_unitStatusTexts__OFFSET: usize = 1752;
pub const CGameUI__1__m_worldFrame__OFFSET: usize = 1776;
pub const CGameUI__1__m_infoBar__OFFSET: usize = 1784;
pub const CGameUI__1__m_commandBar__OFFSET: usize = 1792;
pub const CGameUI__1__m_miniMap__OFFSET: usize = 1800;
pub const CGameUI__1__m_resourceBar__OFFSET: usize = 1808;
pub const CGameUI__1__m_upperButtonBar__OFFSET: usize = 1816;
pub const CGameUI__1__m_observerButtonBar__OFFSET: usize = 1824;
pub const CGameUI__1__m_playerStatisticsPanel__OFFSET: usize = 1832;
pub const CGameUI__1__m_heroPanel__OFFSET: usize = 1840;
pub const CGameUI__1__m_productionPanel__OFFSET: usize = 1848;
pub const CGameUI__1__m_minimapButtonBar__OFFSET: usize = 1856;
pub const CGameUI__1__m_trackingStub__OFFSET: usize = 1864;
pub const CGameUI__1__m_heroBar__OFFSET: usize = 1872;
pub const CGameUI__1__m_peonBar__OFFSET: usize = 1880;
pub const CGameUI__1__m_messageBox__OFFSET: usize = 1888;
pub const CGameUI__1__m_unitMessageBox__OFFSET: usize = 1896;
pub const CGameUI__1__m_chatMessageBox__OFFSET: usize = 1904;
pub const CGameUI__1__m_topMessageBox__OFFSET: usize = 1912;
pub const CGameUI__1__m_currentPortraitButton__OFFSET: usize = 1920;
pub const CGameUI__1__m_timeOfDayIndicator__OFFSET: usize = 1928;
pub const CGameUI__1__m_chatEditBar__OFFSET: usize = 1936;
pub const CGameUI__1__m_cinematicPanel__OFFSET: usize = 1944;
pub const CGameUI__1__m_lastBuffBar__OFFSET: usize = 1952;
pub const CGameUI__1__m_observerBar__OFFSET: usize = 1960;
pub const CGameUI__1__m_miniMapSignalButton__OFFSET: usize = 1968;
pub const CGameUI__1__m_miniMapTerrainButton__OFFSET: usize = 1976;
pub const CGameUI__1__m_miniMapAllyButton__OFFSET: usize = 1984;
pub const CGameUI__1__m_miniMapCreepButton__OFFSET: usize = 1992;
pub const CGameUI__1__m_formationButton__OFFSET: usize = 2000;
pub const CGameUI__1__m_consoleFrame__OFFSET: usize = 2008;
pub const CGameUI__1__m_simpleConsoleFrame__OFFSET: usize = 2016;
pub const CGameUI__1__m_simpleConsoleBottomBarFrame__OFFSET: usize = 2024;
pub const CGameUI__1__m_simpleConsoleBottomBarFrameOverlay__OFFSET: usize = 2032;
pub const CGameUI__1__m_frameUIParent__OFFSET: usize = 2040;
pub const CGameUI__1__top__OFFSET: usize = 2048;
pub const CGameUI__1__topInGame__OFFSET: usize = 2056;
pub const CGameUI__1__m_simpleUIParent__OFFSET: usize = 2064;
pub const CGameUI__1__m_observerUIParent__OFFSET: usize = 2072;
pub const CGameUI__1__m_observerInfoPanelBackground__OFFSET: usize = 2080;
pub const CGameUI__1__m_observerInfoPanelBackpack__OFFSET: usize = 2088;
pub const CGameUI__1__m_consoleBackdropFrame__OFFSET: usize = 2096;
pub const CGameUI__1__m_minimapTextureBackground__OFFSET: usize = 2104;
pub const CGameUI__1__m_infoPanelTexture__OFFSET: usize = 2112;
pub const CGameUI__1__m_quickSaveKey__OFFSET: usize = 2120;
pub const CGameUI__1__m_quickLoadKey__OFFSET: usize = 2124;
pub const CGameUI__1__m_quickHelpKey__OFFSET: usize = 2128;
pub const CGameUI__1__m_quickOptionsKey__OFFSET: usize = 2132;
pub const CGameUI__1__m_quickQuitKey__OFFSET: usize = 2136;
pub const CGameUI__1__m_minimapSignalKey__OFFSET: usize = 2140;
pub const CGameUI__1__m_minimapTerrainKey__OFFSET: usize = 2144;
pub const CGameUI__1__m_minimapColorsKey__OFFSET: usize = 2148;
pub const CGameUI__1__m_minimapCreepKey__OFFSET: usize = 2152;
pub const CGameUI__1__m_formationToggleKey__OFFSET: usize = 2156;
pub const CGameUI__1__m_observerToggleAllPanelsKey__OFFSET: usize = 2160;
pub const CGameUI__1__m_observerProductionModeKey__OFFSET: usize = 2164;
pub const CGameUI__1__m_observerUnitsModeKey__OFFSET: usize = 2168;
pub const CGameUI__1__m_mouseDownHandler__OFFSET: usize = 2176;
pub const CGameUI__1__m_mouseWheelHandler__OFFSET: usize = 2184;
pub const CGameUI__1__m_mouseUpHandler__OFFSET: usize = 2192;
pub const CGameUI__1__m_mouseMoveHandler__OFFSET: usize = 2200;
pub const CGameUI__1__m_observerToggleablePanels__OFFSET: usize = 2208;
pub const CGameUI__1__m_consoleBackdropHeight__OFFSET: usize = 2240;
pub const CGameUI__1__m_auroraChatHandler__OFFSET: usize = 2248;
pub const CGameUI__1__m_lobbyEventsHandler__OFFSET: usize = 2328;
pub const CGameUI__1__s_indexedCustomHotkeyMappingMaxCount__OFFSET: usize = 0;
pub const CGameUI__1__s_customHotkeyEnabled__OFFSET: usize = 0;
pub const CGameUI__1__s_customHotkeyMappingTable__OFFSET: usize = 0;
pub const CGameUI__1__s_customHotkeyMappingTableNonHero__OFFSET: usize = 0;
pub const CGameUI__1__m_observerUnitToFollow__OFFSET: usize = 2376;

#[repr(C)]
pub struct CMapLoader {
    pub _opaque: [u8; 376],
}
pub const CMapLoader__SIZE: usize = 376;
pub const CMapLoader__NAME: &str = "CMapLoader";
pub const CMapLoader__m_host__OFFSET: usize = 0;
pub const CMapLoader__m_controlSet__OFFSET: usize = 8;
pub const CMapLoader__m_speed__OFFSET: usize = 16;
pub const CMapLoader__m_connectEventId__OFFSET: usize = 20;
pub const CMapLoader__m_gameCreateEventId__OFFSET: usize = 24;
pub const CMapLoader__m_gameCloseEventId__OFFSET: usize = 28;
pub const CMapLoader__m_replayFailedEventId__OFFSET: usize = 32;
pub const CMapLoader__m_mapGameType__OFFSET: usize = 36;
pub const CMapLoader__m_mapFlags__OFFSET: usize = 40;
pub const CMapLoader__m_mapFile__OFFSET: usize = 44;
pub const CMapLoader__m_playerName__OFFSET: usize = 304;
pub const CMapLoader__m_fullSyncChecking__OFFSET: usize = 368;
pub const CMapLoader__m_rerecordSyncValues__OFFSET: usize = 369;

#[repr(C)]
pub struct CTimerTextFrame {
    pub _opaque: [u8; 1024],
}
pub const CTimerTextFrame__SIZE: usize = 1024;
pub const CTimerTextFrame__NAME: &str = "CTimerTextFrame";
pub const CTimerTextFrame__m_countDown__OFFSET: usize = 952;
pub const CTimerTextFrame__m_style__OFFSET: usize = 956;
pub const CTimerTextFrame__m_elapsed__OFFSET: usize = 960;
pub const CTimerTextFrame__m_duration__OFFSET: usize = 964;
pub const CTimerTextFrame__m_timer__OFFSET: usize = 968;
pub const CTimerTextFrame__m_lastHour__OFFSET: usize = 1008;
pub const CTimerTextFrame__m_lastMinute__OFFSET: usize = 1012;
pub const CTimerTextFrame__m_lastSecond__OFFSET: usize = 1016;

#[repr(C)]
pub struct MapScreenInfo {
    pub _opaque: [u8; 6652],
}
pub const MapScreenInfo__SIZE: usize = 6652;
pub const MapScreenInfo__NAME: &str = "MapScreenInfo";
pub const MapScreenInfo__imageID__OFFSET: usize = 0;
pub const MapScreenInfo__imageFile__OFFSET: usize = 4;
pub const MapScreenInfo__title__OFFSET: usize = 264;
pub const MapScreenInfo__subtitle__OFFSET: usize = 361;
pub const MapScreenInfo__text__OFFSET: usize = 506;

#[repr(C)]
pub struct MapProperties {
    pub _opaque: [u8; 14476],
}
pub const MapProperties__SIZE: usize = 14476;
pub const MapProperties__NAME: &str = "MapProperties";
pub const MapProperties__mapVersion__OFFSET: usize = 0;
pub const MapProperties__editorVersion__OFFSET: usize = 4;
pub const MapProperties__scriptLanguage__OFFSET: usize = 8;
pub const MapProperties__assetMode__OFFSET: usize = 12;
pub const MapProperties__name__OFFSET: usize = 16;
pub const MapProperties__suggestedPlayers__OFFSET: usize = 125;
pub const MapProperties__author__OFFSET: usize = 174;
pub const MapProperties__description__OFFSET: usize = 271;
pub const MapProperties__size__OFFSET: usize = 1040;
pub const MapProperties__upMargins__OFFSET: usize = 1048;
pub const MapProperties__cameraBounds__OFFSET: usize = 1064;
pub const MapProperties__flags__OFFSET: usize = 1096;
pub const MapProperties__tileSetCode__OFFSET: usize = 1100;
pub const MapProperties__loadingScreen__OFFSET: usize = 1104;
pub const MapProperties__prologueScreen__OFFSET: usize = 7756;
pub const MapProperties__fogParams__OFFSET: usize = 14408;
pub const MapProperties__weatherEffect__OFFSET: usize = 14428;
pub const MapProperties__soundEnv__OFFSET: usize = 14432;
pub const MapProperties__lightCode__OFFSET: usize = 14464;
pub const MapProperties__waterColor__OFFSET: usize = 14465;
pub const MapProperties__mapDataVersion__OFFSET: usize = 14472;

#[repr(C)]
pub struct MapDownloadData {
    pub _opaque: [u8; 192],
}
pub const MapDownloadData__SIZE: usize = 192;
pub const MapDownloadData__NAME: &str = "MapDownloadData";
pub const MapDownloadData__m_gameAdId__OFFSET: usize = 0;
pub const MapDownloadData__m_name__OFFSET: usize = 8;
pub const MapDownloadData__m_password__OFFSET: usize = 48;
pub const MapDownloadData__m_mapName__OFFSET: usize = 88;
pub const MapDownloadData__m_downloadStarted__OFFSET: usize = 128;
pub const MapDownloadData__m_downloadOnly__OFFSET: usize = 129;
pub const MapDownloadData__m_cancelled__OFFSET: usize = 130;
pub const MapDownloadData__m_data__OFFSET: usize = 136;
pub const MapDownloadData__m_urlResolveRequestId__OFFSET: usize = 176;
pub const MapDownloadData__m_urlDownloadRequestId__OFFSET: usize = 184;

#[repr(C)]
pub struct CPlayerReportingCallbacks {
    pub _opaque: [u8; 8],
}
pub const CPlayerReportingCallbacks__SIZE: usize = 8;
pub const CPlayerReportingCallbacks__NAME: &str = "CPlayerReportingCallbacks";

#[repr(C)]
pub struct UIDivision {
    pub _opaque: [u8; 24],
}
pub const UIDivision__SIZE: usize = 24;
pub const UIDivision__NAME: &str = "UIDivision";
pub const UIDivision__divisionId__OFFSET: usize = 0;
pub const UIDivision__lowerBoundMMR__OFFSET: usize = 8;
pub const UIDivision__upperBoundMMR__OFFSET: usize = 16;

#[repr(C)]
pub struct CTriggerRegion {
    pub _opaque: [u8; 64],
}
pub const CTriggerRegion__SIZE: usize = 64;
pub const CTriggerRegion__NAME: &str = "CTriggerRegion";
pub const CTriggerRegion__s_CTriggerRegion_pool__OFFSET: usize = 0;
pub const CTriggerRegion__m_pRegion__OFFSET: usize = 56;

#[repr(C)]
pub struct CTriggerRegionEnterEvent {
    pub _opaque: [u8; 32],
}
pub const CTriggerRegionEnterEvent__SIZE: usize = 32;
pub const CTriggerRegionEnterEvent__NAME: &str = "CTriggerRegionEnterEvent";
pub const CTriggerRegionEnterEvent__agent__OFFSET: usize = 16;
pub const CTriggerRegionEnterEvent__region__OFFSET: usize = 24;

#[repr(C)]
pub struct CTriggerRegionExitEvent {
    pub _opaque: [u8; 32],
}
pub const CTriggerRegionExitEvent__SIZE: usize = 32;
pub const CTriggerRegionExitEvent__NAME: &str = "CTriggerRegionExitEvent";

#[repr(C)]
pub struct WorldShadowParams__1 {
    pub _opaque: [u8; 40],
}
pub const WorldShadowParams__1__SIZE: usize = 40;
pub const WorldShadowParams__1__NAME: &str = "WorldShadowParams";
pub const WorldShadowParams__1__numCinematicCascades__OFFSET: usize = 0;
pub const WorldShadowParams__1__numGameCascades__OFFSET: usize = 4;
pub const WorldShadowParams__1__numPortraitCascades__OFFSET: usize = 8;
pub const WorldShadowParams__1__debug__OFFSET: usize = 12;
pub const WorldShadowParams__1__show__OFFSET: usize = 13;
pub const WorldShadowParams__1__cinematicDistances__OFFSET: usize = 16;
pub const WorldShadowParams__1__gameDistances__OFFSET: usize = 24;
pub const WorldShadowParams__1__portraitDistances__OFFSET: usize = 32;

#[repr(C)]
pub struct CMapCallbacks {
    pub _opaque: [u8; 260],
}
pub const CMapCallbacks__SIZE: usize = 260;
pub const CMapCallbacks__NAME: &str = "CMapCallbacks";
pub const CMapCallbacks__m_lastUsedMapPath__OFFSET: usize = 0;
pub const CMapCallbacks__m_rootDir__OFFSET: usize = 0;

#[repr(C)]
pub struct COrder {
    pub _opaque: [u8; 120],
}
pub const COrder__SIZE: usize = 120;
pub const COrder__NAME: &str = "COrder";
pub const COrder__s_COrder_pool__OFFSET: usize = 0;
pub const COrder__m_id__OFFSET: usize = 88;
pub const COrder__m_playerId__OFFSET: usize = 92;
pub const COrder__m_next__OFFSET: usize = 96;
pub const COrder__m_agent__OFFSET: usize = 108;

#[repr(C)]
pub struct COrderPoint {
    pub _opaque: [u8; 160],
}
pub const COrderPoint__SIZE: usize = 160;
pub const COrderPoint__NAME: &str = "COrderPoint";
pub const COrderPoint__s_COrderPoint_pool__OFFSET: usize = 0;
pub const COrderPoint__m_x__OFFSET: usize = 120;
pub const COrderPoint__m_y__OFFSET: usize = 136;
pub const COrderPoint__m_pPointMoveRequest__OFFSET: usize = 152;

#[repr(C)]
pub struct COrderTarget {
    pub _opaque: [u8; 232],
}
pub const COrderTarget__SIZE: usize = 232;
pub const COrderTarget__NAME: &str = "COrderTarget";
pub const COrderTarget__s_COrderTarget_pool__OFFSET: usize = 0;
pub const COrderTarget__m_target__OFFSET: usize = 160;
pub const COrderTarget__m_pTargetMoveRequest__OFFSET: usize = 176;
pub const COrderTarget__m_ghostImageId__OFFSET: usize = 184;
pub const COrderTarget__m_ghostTargetFlags__OFFSET: usize = 188;
pub const COrderTarget__m_ghostCategory__OFFSET: usize = 192;
pub const COrderTarget__m_ghostImageOwner__OFFSET: usize = 196;
pub const COrderTarget__m_ghostX__OFFSET: usize = 200;
pub const COrderTarget__m_ghostY__OFFSET: usize = 216;

#[repr(C)]
pub struct COrderPoint2 {
    pub _opaque: [u8; 192],
}
pub const COrderPoint2__SIZE: usize = 192;
pub const COrderPoint2__NAME: &str = "COrderPoint2";
pub const COrderPoint2__s_COrderPoint2_pool__OFFSET: usize = 0;
pub const COrderPoint2__m_x2__OFFSET: usize = 160;
pub const COrderPoint2__m_y2__OFFSET: usize = 176;

#[repr(C)]
pub struct COrderTarget2 {
    pub _opaque: [u8; 248],
}
pub const COrderTarget2__SIZE: usize = 248;
pub const COrderTarget2__NAME: &str = "COrderTarget2";
pub const COrderTarget2__s_COrderTarget2_pool__OFFSET: usize = 0;
pub const COrderTarget2__m_target2__OFFSET: usize = 232;

#[repr(C)]
pub struct COrderTrainCancel {
    pub _opaque: [u8; 128],
}
pub const COrderTrainCancel__SIZE: usize = 128;
pub const COrderTrainCancel__NAME: &str = "COrderTrainCancel";
pub const COrderTrainCancel__s_COrderTrainCancel_pool__OFFSET: usize = 0;
pub const COrderTrainCancel__m_slot__OFFSET: usize = 120;
pub const COrderTrainCancel__m_trainableId__OFFSET: usize = 124;

#[repr(C)]
pub struct COrderReviveCancel {
    pub _opaque: [u8; 136],
}
pub const COrderReviveCancel__SIZE: usize = 136;
pub const COrderReviveCancel__NAME: &str = "COrderReviveCancel";
pub const COrderReviveCancel__s_COrderReviveCancel_pool__OFFSET: usize = 0;
pub const COrderReviveCancel__m_hero__OFFSET: usize = 120;

#[repr(C)]
pub struct CWorldFrameWar3 {
    pub _opaque: [u8; 3128],
}
pub const CWorldFrameWar3__SIZE: usize = 3128;
pub const CWorldFrameWar3__NAME: &str = "CWorldFrameWar3";
pub const CWorldFrameWar3__m_trackUpdating__OFFSET: usize = 656;
pub const CWorldFrameWar3__m_sprites__OFFSET: usize = 664;
pub const CWorldFrameWar3__m_targetedSprites__OFFSET: usize = 672;
pub const CWorldFrameWar3__m_walkableSprites__OFFSET: usize = 680;
pub const CWorldFrameWar3__m_pGame__OFFSET: usize = 688;
pub const CWorldFrameWar3__m_pDoHpBarsEnemies__OFFSET: usize = 696;
pub const CWorldFrameWar3__m_pDoHpBarsAllies__OFFSET: usize = 697;
pub const CWorldFrameWar3__m_pLocalPlayer__OFFSET: usize = 704;
pub const CWorldFrameWar3__m_pFog__OFFSET: usize = 712;
pub const CWorldFrameWar3__m_cachedCameraPos__OFFSET: usize = 720;
pub const CWorldFrameWar3__m_localPlayerId__OFFSET: usize = 732;
pub const CWorldFrameWar3__m_pCameraWar3__OFFSET: usize = 736;
pub const CWorldFrameWar3__m_pMinimap__OFFSET: usize = 744;
pub const CWorldFrameWar3__m_spriteButtons__OFFSET: usize = 752;
pub const CWorldFrameWar3__m_terrainButtons__OFFSET: usize = 756;
pub const CWorldFrameWar3__m_cursorMode__OFFSET: usize = 760;
pub const CWorldFrameWar3__anticheat_dummy_array_22__OFFSET: usize = 764;
pub const CWorldFrameWar3__m_cursorModeStack__OFFSET: usize = 808;
pub const CWorldFrameWar3__m_cursorModeLocked__OFFSET: usize = 832;
pub const CWorldFrameWar3__m_forceCursorUpdate__OFFSET: usize = 833;
pub const CWorldFrameWar3__m_cursorArt__OFFSET: usize = 840;
pub const CWorldFrameWar3__m_aspectScaledProjection__OFFSET: usize = 864;
pub const CWorldFrameWar3__m_fovScale__OFFSET: usize = 880;
pub const CWorldFrameWar3__m_frustumBounds__OFFSET: usize = 884;
pub const CWorldFrameWar3__m_frustumBoundsInt__OFFSET: usize = 900;
pub const CWorldFrameWar3__m_localPlayerIdBit__OFFSET: usize = 916;
pub const CWorldFrameWar3__m_playerIdBitsToGhostMasks__OFFSET: usize = 920;
pub const CWorldFrameWar3__m_playerIdBitsToInvisMasks__OFFSET: usize = 1020;
pub const CWorldFrameWar3__m_localPlayerIsObserver__OFFSET: usize = 1120;
pub const CWorldFrameWar3__m_isReplayFogDisabled__OFFSET: usize = 1121;
pub const CWorldFrameWar3__m_lastUpdateElapsedSec__OFFSET: usize = 1124;
pub const CWorldFrameWar3__m_lastAsyncElapsedSec__OFFSET: usize = 1128;
pub const CWorldFrameWar3__m_lastTrackSprite__OFFSET: usize = 1136;
pub const CWorldFrameWar3__m_lastTrackAgent__OFFSET: usize = 1144;
pub const CWorldFrameWar3__m_renderPlacement__OFFSET: usize = 1152;
pub const CWorldFrameWar3__m_buildingPlacementRender__OFFSET: usize = 1153;
pub const CWorldFrameWar3__m_pBuildFrame__OFFSET: usize = 1160;
pub const CWorldFrameWar3__m_cineFilter__OFFSET: usize = 1168;
pub const CWorldFrameWar3__m_targetingSprite__OFFSET: usize = 1696;
pub const CWorldFrameWar3__m_renderTargetSprite__OFFSET: usize = 1704;
pub const CWorldFrameWar3__m_targetingImage__OFFSET: usize = 1708;
pub const CWorldFrameWar3__m_targetingRadius__OFFSET: usize = 1712;
pub const CWorldFrameWar3__m_targetingSqMag__OFFSET: usize = 1716;
pub const CWorldFrameWar3__m_targetImageColor__OFFSET: usize = 1720;
pub const CWorldFrameWar3__m_currentWorldPos__OFFSET: usize = 1724;
pub const CWorldFrameWar3__m_suspended__OFFSET: usize = 1736;
pub const CWorldFrameWar3__m_noHitTests__OFFSET: usize = 1737;
pub const CWorldFrameWar3__m_doOcclusion__OFFSET: usize = 1738;
pub const CWorldFrameWar3__m_mouseMoveProcessed__OFFSET: usize = 1739;
pub const CWorldFrameWar3__m_doFogOfWar__OFFSET: usize = 1740;
pub const CWorldFrameWar3__anticheat_dummy_array_23__OFFSET: usize = 1741;
pub const CWorldFrameWar3__m_useDarkMask__OFFSET: usize = 1744;
pub const CWorldFrameWar3__m_renderUnits__OFFSET: usize = 1745;
pub const CWorldFrameWar3__m_terrainFog__OFFSET: usize = 1752;
pub const CWorldFrameWar3__m_terrainLight__OFFSET: usize = 1760;
pub const CWorldFrameWar3__m_unitLight__OFFSET: usize = 1768;
pub const CWorldFrameWar3__m_suspendedAmbientLight__OFFSET: usize = 1776;
pub const CWorldFrameWar3__m_targetLight__OFFSET: usize = 1784;
pub const CWorldFrameWar3__m_skyModelPath__OFFSET: usize = 1792;
pub const CWorldFrameWar3__m_sky__OFFSET: usize = 1816;
pub const CWorldFrameWar3__m_showSky__OFFSET: usize = 1824;
pub const CWorldFrameWar3__m_renderSky__OFFSET: usize = 1825;
pub const CWorldFrameWar3__m_indicators__OFFSET: usize = 1832;
pub const CWorldFrameWar3__m_targetIndicators__OFFSET: usize = 1856;
pub const CWorldFrameWar3__m_targetIndicatorIndex__OFFSET: usize = 1880;
pub const CWorldFrameWar3__m_enumElapsed__OFFSET: usize = 1884;
pub const CWorldFrameWar3__m_asyncElapsed__OFFSET: usize = 1888;
pub const CWorldFrameWar3__m_doEnumUpdate__OFFSET: usize = 1892;
pub const CWorldFrameWar3__m_doFogEnum__OFFSET: usize = 1893;
pub const CWorldFrameWar3__m_trackingElapsed__OFFSET: usize = 1896;
pub const CWorldFrameWar3__m_terrainMinZ__OFFSET: usize = 1900;
pub const CWorldFrameWar3__m_scaledAnimTime__OFFSET: usize = 1904;
pub const CWorldFrameWar3__m_hoursPerDay__OFFSET: usize = 1908;
pub const CWorldFrameWar3__m_selCircColorFriend__OFFSET: usize = 1912;
pub const CWorldFrameWar3__anticheat_dummy_array_24__OFFSET: usize = 1916;
pub const CWorldFrameWar3__m_selCircColorNeutral__OFFSET: usize = 1921;
pub const CWorldFrameWar3__m_selCircColorEnemy__OFFSET: usize = 1925;
pub const CWorldFrameWar3__m_enableTargetIndicator__OFFSET: usize = 1929;
pub const CWorldFrameWar3__m_rallyFlags__OFFSET: usize = 1932;
pub const CWorldFrameWar3__m_rallyDst__OFFSET: usize = 1936;
pub const CWorldFrameWar3__m_rallyDstReuse__OFFSET: usize = 2708;
pub const CWorldFrameWar3__m_rallySrc__OFFSET: usize = 2776;
pub const CWorldFrameWar3__m_indicatorTerrain__OFFSET: usize = 2784;
pub const CWorldFrameWar3__m_waypointIndicators__OFFSET: usize = 2792;
pub const CWorldFrameWar3__m_waypointIndicatorIndex__OFFSET: usize = 2816;
pub const CWorldFrameWar3__m_unitUpdateQueue__OFFSET: usize = 2824;
pub const CWorldFrameWar3__m_destUpdateQueue__OFFSET: usize = 2848;
pub const CWorldFrameWar3__m_itemUpdateQueue__OFFSET: usize = 2872;
pub const CWorldFrameWar3__m_captUpdateQueue__OFFSET: usize = 2896;
pub const CWorldFrameWar3__m_imagUpdateQueue__OFFSET: usize = 2920;
pub const CWorldFrameWar3__m_ghstUpdateQueue__OFFSET: usize = 2944;
pub const CWorldFrameWar3__anticheat_dummy_array_25__OFFSET: usize = 2968;
pub const CWorldFrameWar3__m_enumeratedUnits__OFFSET: usize = 2976;
pub const CWorldFrameWar3__m_enumeratedItems__OFFSET: usize = 3000;
pub const CWorldFrameWar3__m_visibleUnits__OFFSET: usize = 3024;
pub const CWorldFrameWar3__m_visibleSelectables__OFFSET: usize = 3048;
pub const CWorldFrameWar3__m_fogUpdateList__OFFSET: usize = 3072;
pub const CWorldFrameWar3__m_ghostImages__OFFSET: usize = 3096;
pub const CWorldFrameWar3__m_lastRenderLight__OFFSET: usize = 3120;
pub const CWorldFrameWar3__m_lastRenderFog__OFFSET: usize = 3124;
pub const CWorldFrameWar3__enableDSQDenialMessage__OFFSET: usize = 0;

#[repr(C)]
pub struct CGameUI__2 {
    pub _opaque: [u8; 2384],
}
pub const CGameUI__2__SIZE: usize = 2384;
pub const CGameUI__2__NAME: &str = "CGameUI";
pub const CGameUI__2__m_transitionState__OFFSET: usize = 704;
pub const CGameUI__2__m_cachedFonts__OFFSET: usize = 712;
pub const CGameUI__2__NUMBER_OF_PLAYER_ROWS__OFFSET: usize = 0;
pub const CGameUI__2__FIRST_ROW_Y_OFFSET__OFFSET: usize = 0;
pub const CGameUI__2__ROW_SPACING__OFFSET: usize = 0;
pub const CGameUI__2__m_userControl__OFFSET: usize = 784;
pub const CGameUI__2__m_userUI__OFFSET: usize = 785;
pub const CGameUI__2__m_currentInputMode__OFFSET: usize = 792;
pub const CGameUI__2__m_inputModeStack__OFFSET: usize = 800;
pub const CGameUI__2__m_sharedTooltip__OFFSET: usize = 824;
pub const CGameUI__2__m_uberTooltip__OFFSET: usize = 832;
pub const CGameUI__2__m_showUberTips__OFFSET: usize = 840;
pub const CGameUI__2__m_autosaveReplay__OFFSET: usize = 841;
pub const CGameUI__2__m_healthBarsMode__OFFSET: usize = 844;
pub const CGameUI__2__m_forceShowHealthBars__OFFSET: usize = 848;
pub const CGameUI__2__m_minimapSignals__OFFSET: usize = 849;
pub const CGameUI__2__m_useColoredHealthBars__OFFSET: usize = 850;
pub const CGameUI__2__m_useHeroFrames__OFFSET: usize = 851;
pub const CGameUI__2__m_showHeroLevelFrame__OFFSET: usize = 852;
pub const CGameUI__2__m_showNumericCooldown__OFFSET: usize = 853;
pub const CGameUI__2__m_showDenyIcon__OFFSET: usize = 854;
pub const CGameUI__2__m_showGoldmineUnitCounter__OFFSET: usize = 855;
pub const CGameUI__2__m_enableObserverUiV2__OFFSET: usize = 856;
pub const CGameUI__2__m_asyncPlayerData__OFFSET: usize = 860;
pub const CGameUI__2__m_targetMode__OFFSET: usize = 960;
pub const CGameUI__2__m_selectMode__OFFSET: usize = 968;
pub const CGameUI__2__m_dragSelectMode__OFFSET: usize = 976;
pub const CGameUI__2__m_dragScrollMode__OFFSET: usize = 984;
pub const CGameUI__2__m_buildMode__OFFSET: usize = 992;
pub const CGameUI__2__m_signalMode__OFFSET: usize = 1000;
pub const CGameUI__2__m_escapeMenu__OFFSET: usize = 1008;
pub const CGameUI__2__m_gameOverMenu__OFFSET: usize = 1016;
pub const CGameUI__2__m_suspendIndicator__OFFSET: usize = 1024;
pub const CGameUI__2__m_unresponsiveIndicator__OFFSET: usize = 1032;
pub const CGameUI__2__m_allianceMode__OFFSET: usize = 1040;
pub const CGameUI__2__m_chatMode__OFFSET: usize = 1048;
pub const CGameUI__2__m_questMode__OFFSET: usize = 1056;
pub const CGameUI__2__anticheat_dummy_array_26__OFFSET: usize = 1064;
pub const CGameUI__2__m_scriptDialogMode__OFFSET: usize = 1152;
pub const CGameUI__2__m_unresponsiveDelayTimerId__OFFSET: usize = 1160;
pub const CGameUI__2__m_selectionManager__OFFSET: usize = 1168;
pub const CGameUI__2__m_dragScrollManager__OFFSET: usize = 1176;
pub const CGameUI__2__m_camera__OFFSET: usize = 1184;
pub const CGameUI__2__m_paused__OFFSET: usize = 1192;
pub const CGameUI__2__m_uiPaused__OFFSET: usize = 1193;
pub const CGameUI__2__m_scriptPaused__OFFSET: usize = 1194;
pub const CGameUI__2__m_disableUpkeepMsg__OFFSET: usize = 1195;
pub const CGameUI__2__m_upkeepLvlSave__OFFSET: usize = 1196;
pub const CGameUI__2__m_refresh__OFFSET: usize = 1200;
pub const CGameUI__2__m_refreshWaypoints__OFFSET: usize = 1201;
pub const CGameUI__2__m_deferCleanSelUI__OFFSET: usize = 1202;
pub const CGameUI__2__m_refreshInfoBar__OFFSET: usize = 1203;
pub const CGameUI__2__m_refreshCommandBar__OFFSET: usize = 1204;
pub const CGameUI__2__m_refreshHeroBar__OFFSET: usize = 1205;
pub const CGameUI__2__m_popSubMenu__OFFSET: usize = 1206;
pub const CGameUI__2__m_activeValidOrder__OFFSET: usize = 1208;
pub const CGameUI__2__m_cineForceSubtitles__OFFSET: usize = 1212;
pub const CGameUI__2__m_cinematicPanelsHidden__OFFSET: usize = 1213;
pub const CGameUI__2__m_dragSelectEnabled__OFFSET: usize = 1214;
pub const CGameUI__2__m_dragSelectUIEnabled__OFFSET: usize = 1215;
pub const CGameUI__2__m_preSelectEnabled__OFFSET: usize = 1216;
pub const CGameUI__2__m_preSelectUIEnabled__OFFSET: usize = 1217;
pub const CGameUI__2__m_selectEnabled__OFFSET: usize = 1218;
pub const CGameUI__2__m_selectUIEnabled__OFFSET: usize = 1219;
pub const CGameUI__2__m_observerModeEnabled__OFFSET: usize = 1220;
pub const CGameUI__2__m_observerModeV2Active__OFFSET: usize = 1221;
pub const CGameUI__2__m_wideScreenAutoPosition__OFFSET: usize = 1222;
pub const CGameUI__2__m_fadeTimer__OFFSET: usize = 1224;
pub const CGameUI__2__m_cinematicFadeTimer__OFFSET: usize = 1328;
pub const CGameUI__2__anticheat_dummy_array_27__OFFSET: usize = 1432;
pub const CGameUI__2__m_frameParentTimer__OFFSET: usize = 1440;
pub const CGameUI__2__m_simpleFadeTimer__OFFSET: usize = 1544;
pub const CGameUI__2__m_attackNotify__OFFSET: usize = 1552;
pub const CGameUI__2__m_warpNotify__OFFSET: usize = 1576;
pub const CGameUI__2__m_timerDialogs__OFFSET: usize = 1608;
pub const CGameUI__2__m_leaderboard__OFFSET: usize = 1632;
pub const CGameUI__2__anticheat_dummy_array_28__OFFSET: usize = 1640;
pub const CGameUI__2__m_multiboard__OFFSET: usize = 1648;
pub const CGameUI__2__m_suspendMultiboardDisplay__OFFSET: usize = 1656;
pub const CGameUI__2__anticheat_dummy_array_29__OFFSET: usize = 1657;
pub const CGameUI__2__m_scriptDialog__OFFSET: usize = 1728;
pub const CGameUI__2__m_replaySelection__OFFSET: usize = 1736;
pub const CGameUI__2__m_unitStatusIconUIFactory__OFFSET: usize = 1744;
pub const CGameUI__2__m_unitStatusTexts__OFFSET: usize = 1752;
pub const CGameUI__2__m_worldFrame__OFFSET: usize = 1776;
pub const CGameUI__2__m_infoBar__OFFSET: usize = 1784;
pub const CGameUI__2__m_commandBar__OFFSET: usize = 1792;
pub const CGameUI__2__m_miniMap__OFFSET: usize = 1800;
pub const CGameUI__2__m_resourceBar__OFFSET: usize = 1808;
pub const CGameUI__2__m_upperButtonBar__OFFSET: usize = 1816;
pub const CGameUI__2__m_observerButtonBar__OFFSET: usize = 1824;
pub const CGameUI__2__m_playerStatisticsPanel__OFFSET: usize = 1832;
pub const CGameUI__2__m_heroPanel__OFFSET: usize = 1840;
pub const CGameUI__2__m_productionPanel__OFFSET: usize = 1848;
pub const CGameUI__2__m_minimapButtonBar__OFFSET: usize = 1856;
pub const CGameUI__2__m_trackingStub__OFFSET: usize = 1864;
pub const CGameUI__2__m_heroBar__OFFSET: usize = 1872;
pub const CGameUI__2__m_peonBar__OFFSET: usize = 1880;
pub const CGameUI__2__m_messageBox__OFFSET: usize = 1888;
pub const CGameUI__2__m_unitMessageBox__OFFSET: usize = 1896;
pub const CGameUI__2__m_chatMessageBox__OFFSET: usize = 1904;
pub const CGameUI__2__m_topMessageBox__OFFSET: usize = 1912;
pub const CGameUI__2__m_currentPortraitButton__OFFSET: usize = 1920;
pub const CGameUI__2__m_timeOfDayIndicator__OFFSET: usize = 1928;
pub const CGameUI__2__m_chatEditBar__OFFSET: usize = 1936;
pub const CGameUI__2__m_cinematicPanel__OFFSET: usize = 1944;
pub const CGameUI__2__m_lastBuffBar__OFFSET: usize = 1952;
pub const CGameUI__2__m_observerBar__OFFSET: usize = 1960;
pub const CGameUI__2__m_miniMapSignalButton__OFFSET: usize = 1968;
pub const CGameUI__2__m_miniMapTerrainButton__OFFSET: usize = 1976;
pub const CGameUI__2__m_miniMapAllyButton__OFFSET: usize = 1984;
pub const CGameUI__2__m_miniMapCreepButton__OFFSET: usize = 1992;
pub const CGameUI__2__m_formationButton__OFFSET: usize = 2000;
pub const CGameUI__2__m_consoleFrame__OFFSET: usize = 2008;
pub const CGameUI__2__m_simpleConsoleFrame__OFFSET: usize = 2016;
pub const CGameUI__2__m_simpleConsoleBottomBarFrame__OFFSET: usize = 2024;
pub const CGameUI__2__m_simpleConsoleBottomBarFrameOverlay__OFFSET: usize = 2032;
pub const CGameUI__2__m_frameUIParent__OFFSET: usize = 2040;
pub const CGameUI__2__top__OFFSET: usize = 2048;
pub const CGameUI__2__topInGame__OFFSET: usize = 2056;
pub const CGameUI__2__m_simpleUIParent__OFFSET: usize = 2064;
pub const CGameUI__2__m_observerUIParent__OFFSET: usize = 2072;
pub const CGameUI__2__m_observerInfoPanelBackground__OFFSET: usize = 2080;
pub const CGameUI__2__m_observerInfoPanelBackpack__OFFSET: usize = 2088;
pub const CGameUI__2__m_consoleBackdropFrame__OFFSET: usize = 2096;
pub const CGameUI__2__m_minimapTextureBackground__OFFSET: usize = 2104;
pub const CGameUI__2__m_infoPanelTexture__OFFSET: usize = 2112;
pub const CGameUI__2__m_quickSaveKey__OFFSET: usize = 2120;
pub const CGameUI__2__m_quickLoadKey__OFFSET: usize = 2124;
pub const CGameUI__2__m_quickHelpKey__OFFSET: usize = 2128;
pub const CGameUI__2__m_quickOptionsKey__OFFSET: usize = 2132;
pub const CGameUI__2__m_quickQuitKey__OFFSET: usize = 2136;
pub const CGameUI__2__m_minimapSignalKey__OFFSET: usize = 2140;
pub const CGameUI__2__m_minimapTerrainKey__OFFSET: usize = 2144;
pub const CGameUI__2__m_minimapColorsKey__OFFSET: usize = 2148;
pub const CGameUI__2__m_minimapCreepKey__OFFSET: usize = 2152;
pub const CGameUI__2__m_formationToggleKey__OFFSET: usize = 2156;
pub const CGameUI__2__m_observerToggleAllPanelsKey__OFFSET: usize = 2160;
pub const CGameUI__2__m_observerProductionModeKey__OFFSET: usize = 2164;
pub const CGameUI__2__m_observerUnitsModeKey__OFFSET: usize = 2168;
pub const CGameUI__2__m_mouseDownHandler__OFFSET: usize = 2176;
pub const CGameUI__2__m_mouseWheelHandler__OFFSET: usize = 2184;
pub const CGameUI__2__m_mouseUpHandler__OFFSET: usize = 2192;
pub const CGameUI__2__m_mouseMoveHandler__OFFSET: usize = 2200;
pub const CGameUI__2__m_observerToggleablePanels__OFFSET: usize = 2208;
pub const CGameUI__2__m_consoleBackdropHeight__OFFSET: usize = 2240;
pub const CGameUI__2__m_auroraChatHandler__OFFSET: usize = 2248;
pub const CGameUI__2__m_lobbyEventsHandler__OFFSET: usize = 2328;
pub const CGameUI__2__s_indexedCustomHotkeyMappingMaxCount__OFFSET: usize = 0;
pub const CGameUI__2__s_customHotkeyEnabled__OFFSET: usize = 0;
pub const CGameUI__2__s_customHotkeyMappingTable__OFFSET: usize = 0;
pub const CGameUI__2__s_customHotkeyMappingTableNonHero__OFFSET: usize = 0;
pub const CGameUI__2__m_observerUnitToFollow__OFFSET: usize = 2376;

#[repr(C)]
pub struct CPlayerSkinInventory {
    pub _opaque: [u8; 288],
}
pub const CPlayerSkinInventory__SIZE: usize = 288;
pub const CPlayerSkinInventory__NAME: &str = "CPlayerSkinInventory";
pub const CPlayerSkinInventory__s_playerSkinInventory__OFFSET: usize = 0;
pub const CPlayerSkinInventory__m_skins__OFFSET: usize = 0;
pub const CPlayerSkinInventory__m_localPlayerSkin__OFFSET: usize = 24;
pub const CPlayerSkinInventory__m_allSkins__OFFSET: usize = 224;
pub const CPlayerSkinInventory__m_allSkinSections__OFFSET: usize = 256;

#[repr(C)]
pub struct CPlayerSkin {
    pub _opaque: [u8; 200],
}
pub const CPlayerSkin__SIZE: usize = 200;
pub const CPlayerSkin__NAME: &str = "CPlayerSkin";
pub const CPlayerSkin__m_selectedSkin__OFFSET: usize = 0;
pub const CPlayerSkin__m_selectedSkinSection__OFFSET: usize = 32;
pub const CPlayerSkin__m_skinLicenses__OFFSET: usize = 64;
pub const CPlayerSkin__m_skinSectionLicenses__OFFSET: usize = 96;
pub const CPlayerSkin__m_licenses__OFFSET: usize = 128;
pub const CPlayerSkin__m_availableSkin__OFFSET: usize = 136;
pub const CPlayerSkin__m_availableSkinSection__OFFSET: usize = 168;

#[repr(C)]
pub struct CPlayerSkinGame {
    pub _opaque: [u8; 32],
}
pub const CPlayerSkinGame__SIZE: usize = 32;
pub const CPlayerSkinGame__NAME: &str = "CPlayerSkinGame";
pub const CPlayerSkinGame__m_selectedSkins__OFFSET: usize = 0;

#[repr(C)]
pub struct CCamera {
    pub _opaque: [u8; 904],
}
pub const CCamera__SIZE: usize = 904;
pub const CCamera__NAME: &str = "CCamera";
pub const CCamera__m_position__OFFSET: usize = 64;
pub const CCamera__m_target__OFFSET: usize = 136;
pub const CCamera__m_distance__OFFSET: usize = 208;
pub const CCamera__m_zFar__OFFSET: usize = 272;
pub const CCamera__m_zNear__OFFSET: usize = 336;
pub const CCamera__m_aoa__OFFSET: usize = 400;
pub const CCamera__m_fov__OFFSET: usize = 472;
pub const CCamera__m_roll__OFFSET: usize = 544;
pub const CCamera__m_localPitch__OFFSET: usize = 616;
pub const CCamera__m_localYaw__OFFSET: usize = 688;
pub const CCamera__m_localRoll__OFFSET: usize = 760;
pub const CCamera__m_rotation__OFFSET: usize = 832;

#[repr(C)]
pub struct CameraInfo {
    pub _opaque: [u8; 104],
}
pub const CameraInfo__SIZE: usize = 104;
pub const CameraInfo__NAME: &str = "CameraInfo";
pub const CameraInfo__cameras__OFFSET: usize = 96;

#[repr(C)]
pub struct CGameUI__3 {
    pub _opaque: [u8; 2384],
}
pub const CGameUI__3__SIZE: usize = 2384;
pub const CGameUI__3__NAME: &str = "CGameUI";
pub const CGameUI__3__m_transitionState__OFFSET: usize = 704;
pub const CGameUI__3__m_cachedFonts__OFFSET: usize = 712;
pub const CGameUI__3__NUMBER_OF_PLAYER_ROWS__OFFSET: usize = 0;
pub const CGameUI__3__FIRST_ROW_Y_OFFSET__OFFSET: usize = 0;
pub const CGameUI__3__ROW_SPACING__OFFSET: usize = 0;
pub const CGameUI__3__m_userControl__OFFSET: usize = 784;
pub const CGameUI__3__m_userUI__OFFSET: usize = 785;
pub const CGameUI__3__m_currentInputMode__OFFSET: usize = 792;
pub const CGameUI__3__m_inputModeStack__OFFSET: usize = 800;
pub const CGameUI__3__m_sharedTooltip__OFFSET: usize = 824;
pub const CGameUI__3__m_uberTooltip__OFFSET: usize = 832;
pub const CGameUI__3__m_showUberTips__OFFSET: usize = 840;
pub const CGameUI__3__m_autosaveReplay__OFFSET: usize = 841;
pub const CGameUI__3__m_healthBarsMode__OFFSET: usize = 844;
pub const CGameUI__3__m_forceShowHealthBars__OFFSET: usize = 848;
pub const CGameUI__3__m_minimapSignals__OFFSET: usize = 849;
pub const CGameUI__3__m_useColoredHealthBars__OFFSET: usize = 850;
pub const CGameUI__3__m_useHeroFrames__OFFSET: usize = 851;
pub const CGameUI__3__m_showHeroLevelFrame__OFFSET: usize = 852;
pub const CGameUI__3__m_showNumericCooldown__OFFSET: usize = 853;
pub const CGameUI__3__m_showDenyIcon__OFFSET: usize = 854;
pub const CGameUI__3__m_showGoldmineUnitCounter__OFFSET: usize = 855;
pub const CGameUI__3__m_enableObserverUiV2__OFFSET: usize = 856;
pub const CGameUI__3__m_asyncPlayerData__OFFSET: usize = 860;
pub const CGameUI__3__m_targetMode__OFFSET: usize = 960;
pub const CGameUI__3__m_selectMode__OFFSET: usize = 968;
pub const CGameUI__3__m_dragSelectMode__OFFSET: usize = 976;
pub const CGameUI__3__m_dragScrollMode__OFFSET: usize = 984;
pub const CGameUI__3__m_buildMode__OFFSET: usize = 992;
pub const CGameUI__3__m_signalMode__OFFSET: usize = 1000;
pub const CGameUI__3__m_escapeMenu__OFFSET: usize = 1008;
pub const CGameUI__3__m_gameOverMenu__OFFSET: usize = 1016;
pub const CGameUI__3__m_suspendIndicator__OFFSET: usize = 1024;
pub const CGameUI__3__m_unresponsiveIndicator__OFFSET: usize = 1032;
pub const CGameUI__3__m_allianceMode__OFFSET: usize = 1040;
pub const CGameUI__3__m_chatMode__OFFSET: usize = 1048;
pub const CGameUI__3__m_questMode__OFFSET: usize = 1056;
pub const CGameUI__3__anticheat_dummy_array_4__OFFSET: usize = 1064;
pub const CGameUI__3__m_scriptDialogMode__OFFSET: usize = 1152;
pub const CGameUI__3__m_unresponsiveDelayTimerId__OFFSET: usize = 1160;
pub const CGameUI__3__m_selectionManager__OFFSET: usize = 1168;
pub const CGameUI__3__m_dragScrollManager__OFFSET: usize = 1176;
pub const CGameUI__3__m_camera__OFFSET: usize = 1184;
pub const CGameUI__3__m_paused__OFFSET: usize = 1192;
pub const CGameUI__3__m_uiPaused__OFFSET: usize = 1193;
pub const CGameUI__3__m_scriptPaused__OFFSET: usize = 1194;
pub const CGameUI__3__m_disableUpkeepMsg__OFFSET: usize = 1195;
pub const CGameUI__3__m_upkeepLvlSave__OFFSET: usize = 1196;
pub const CGameUI__3__m_refresh__OFFSET: usize = 1200;
pub const CGameUI__3__m_refreshWaypoints__OFFSET: usize = 1201;
pub const CGameUI__3__m_deferCleanSelUI__OFFSET: usize = 1202;
pub const CGameUI__3__m_refreshInfoBar__OFFSET: usize = 1203;
pub const CGameUI__3__m_refreshCommandBar__OFFSET: usize = 1204;
pub const CGameUI__3__m_refreshHeroBar__OFFSET: usize = 1205;
pub const CGameUI__3__m_popSubMenu__OFFSET: usize = 1206;
pub const CGameUI__3__m_activeValidOrder__OFFSET: usize = 1208;
pub const CGameUI__3__m_cineForceSubtitles__OFFSET: usize = 1212;
pub const CGameUI__3__m_cinematicPanelsHidden__OFFSET: usize = 1213;
pub const CGameUI__3__m_dragSelectEnabled__OFFSET: usize = 1214;
pub const CGameUI__3__m_dragSelectUIEnabled__OFFSET: usize = 1215;
pub const CGameUI__3__m_preSelectEnabled__OFFSET: usize = 1216;
pub const CGameUI__3__m_preSelectUIEnabled__OFFSET: usize = 1217;
pub const CGameUI__3__m_selectEnabled__OFFSET: usize = 1218;
pub const CGameUI__3__m_selectUIEnabled__OFFSET: usize = 1219;
pub const CGameUI__3__m_observerModeEnabled__OFFSET: usize = 1220;
pub const CGameUI__3__m_observerModeV2Active__OFFSET: usize = 1221;
pub const CGameUI__3__m_wideScreenAutoPosition__OFFSET: usize = 1222;
pub const CGameUI__3__m_fadeTimer__OFFSET: usize = 1224;
pub const CGameUI__3__m_cinematicFadeTimer__OFFSET: usize = 1328;
pub const CGameUI__3__anticheat_dummy_array_5__OFFSET: usize = 1432;
pub const CGameUI__3__m_frameParentTimer__OFFSET: usize = 1440;
pub const CGameUI__3__m_simpleFadeTimer__OFFSET: usize = 1544;
pub const CGameUI__3__m_attackNotify__OFFSET: usize = 1552;
pub const CGameUI__3__m_warpNotify__OFFSET: usize = 1576;
pub const CGameUI__3__m_timerDialogs__OFFSET: usize = 1608;
pub const CGameUI__3__m_leaderboard__OFFSET: usize = 1632;
pub const CGameUI__3__anticheat_dummy_array_6__OFFSET: usize = 1640;
pub const CGameUI__3__m_multiboard__OFFSET: usize = 1648;
pub const CGameUI__3__m_suspendMultiboardDisplay__OFFSET: usize = 1656;
pub const CGameUI__3__anticheat_dummy_array_7__OFFSET: usize = 1657;
pub const CGameUI__3__m_scriptDialog__OFFSET: usize = 1728;
pub const CGameUI__3__m_replaySelection__OFFSET: usize = 1736;
pub const CGameUI__3__m_unitStatusIconUIFactory__OFFSET: usize = 1744;
pub const CGameUI__3__m_unitStatusTexts__OFFSET: usize = 1752;
pub const CGameUI__3__m_worldFrame__OFFSET: usize = 1776;
pub const CGameUI__3__m_infoBar__OFFSET: usize = 1784;
pub const CGameUI__3__m_commandBar__OFFSET: usize = 1792;
pub const CGameUI__3__m_miniMap__OFFSET: usize = 1800;
pub const CGameUI__3__m_resourceBar__OFFSET: usize = 1808;
pub const CGameUI__3__m_upperButtonBar__OFFSET: usize = 1816;
pub const CGameUI__3__m_observerButtonBar__OFFSET: usize = 1824;
pub const CGameUI__3__m_playerStatisticsPanel__OFFSET: usize = 1832;
pub const CGameUI__3__m_heroPanel__OFFSET: usize = 1840;
pub const CGameUI__3__m_productionPanel__OFFSET: usize = 1848;
pub const CGameUI__3__m_minimapButtonBar__OFFSET: usize = 1856;
pub const CGameUI__3__m_trackingStub__OFFSET: usize = 1864;
pub const CGameUI__3__m_heroBar__OFFSET: usize = 1872;
pub const CGameUI__3__m_peonBar__OFFSET: usize = 1880;
pub const CGameUI__3__m_messageBox__OFFSET: usize = 1888;
pub const CGameUI__3__m_unitMessageBox__OFFSET: usize = 1896;
pub const CGameUI__3__m_chatMessageBox__OFFSET: usize = 1904;
pub const CGameUI__3__m_topMessageBox__OFFSET: usize = 1912;
pub const CGameUI__3__m_currentPortraitButton__OFFSET: usize = 1920;
pub const CGameUI__3__m_timeOfDayIndicator__OFFSET: usize = 1928;
pub const CGameUI__3__m_chatEditBar__OFFSET: usize = 1936;
pub const CGameUI__3__m_cinematicPanel__OFFSET: usize = 1944;
pub const CGameUI__3__m_lastBuffBar__OFFSET: usize = 1952;
pub const CGameUI__3__m_observerBar__OFFSET: usize = 1960;
pub const CGameUI__3__m_miniMapSignalButton__OFFSET: usize = 1968;
pub const CGameUI__3__m_miniMapTerrainButton__OFFSET: usize = 1976;
pub const CGameUI__3__m_miniMapAllyButton__OFFSET: usize = 1984;
pub const CGameUI__3__m_miniMapCreepButton__OFFSET: usize = 1992;
pub const CGameUI__3__m_formationButton__OFFSET: usize = 2000;
pub const CGameUI__3__m_consoleFrame__OFFSET: usize = 2008;
pub const CGameUI__3__m_simpleConsoleFrame__OFFSET: usize = 2016;
pub const CGameUI__3__m_simpleConsoleBottomBarFrame__OFFSET: usize = 2024;
pub const CGameUI__3__m_simpleConsoleBottomBarFrameOverlay__OFFSET: usize = 2032;
pub const CGameUI__3__m_frameUIParent__OFFSET: usize = 2040;
pub const CGameUI__3__top__OFFSET: usize = 2048;
pub const CGameUI__3__topInGame__OFFSET: usize = 2056;
pub const CGameUI__3__m_simpleUIParent__OFFSET: usize = 2064;
pub const CGameUI__3__m_observerUIParent__OFFSET: usize = 2072;
pub const CGameUI__3__m_observerInfoPanelBackground__OFFSET: usize = 2080;
pub const CGameUI__3__m_observerInfoPanelBackpack__OFFSET: usize = 2088;
pub const CGameUI__3__m_consoleBackdropFrame__OFFSET: usize = 2096;
pub const CGameUI__3__m_minimapTextureBackground__OFFSET: usize = 2104;
pub const CGameUI__3__m_infoPanelTexture__OFFSET: usize = 2112;
pub const CGameUI__3__m_quickSaveKey__OFFSET: usize = 2120;
pub const CGameUI__3__m_quickLoadKey__OFFSET: usize = 2124;
pub const CGameUI__3__m_quickHelpKey__OFFSET: usize = 2128;
pub const CGameUI__3__m_quickOptionsKey__OFFSET: usize = 2132;
pub const CGameUI__3__m_quickQuitKey__OFFSET: usize = 2136;
pub const CGameUI__3__m_minimapSignalKey__OFFSET: usize = 2140;
pub const CGameUI__3__m_minimapTerrainKey__OFFSET: usize = 2144;
pub const CGameUI__3__m_minimapColorsKey__OFFSET: usize = 2148;
pub const CGameUI__3__m_minimapCreepKey__OFFSET: usize = 2152;
pub const CGameUI__3__m_formationToggleKey__OFFSET: usize = 2156;
pub const CGameUI__3__m_observerToggleAllPanelsKey__OFFSET: usize = 2160;
pub const CGameUI__3__m_observerProductionModeKey__OFFSET: usize = 2164;
pub const CGameUI__3__m_observerUnitsModeKey__OFFSET: usize = 2168;
pub const CGameUI__3__m_mouseDownHandler__OFFSET: usize = 2176;
pub const CGameUI__3__m_mouseWheelHandler__OFFSET: usize = 2184;
pub const CGameUI__3__m_mouseUpHandler__OFFSET: usize = 2192;
pub const CGameUI__3__m_mouseMoveHandler__OFFSET: usize = 2200;
pub const CGameUI__3__m_observerToggleablePanels__OFFSET: usize = 2208;
pub const CGameUI__3__m_consoleBackdropHeight__OFFSET: usize = 2240;
pub const CGameUI__3__m_auroraChatHandler__OFFSET: usize = 2248;
pub const CGameUI__3__m_lobbyEventsHandler__OFFSET: usize = 2328;
pub const CGameUI__3__s_indexedCustomHotkeyMappingMaxCount__OFFSET: usize = 0;
pub const CGameUI__3__s_customHotkeyEnabled__OFFSET: usize = 0;
pub const CGameUI__3__s_customHotkeyMappingTable__OFFSET: usize = 0;
pub const CGameUI__3__s_customHotkeyMappingTableNonHero__OFFSET: usize = 0;
pub const CGameUI__3__m_observerUnitToFollow__OFFSET: usize = 2376;

#[repr(C)]
pub struct CSelectionWar3 {
    pub _opaque: [u8; 1056],
}
pub const CSelectionWar3__SIZE: usize = 1056;
pub const CSelectionWar3__NAME: &str = "CSelectionWar3";
pub const CSelectionWar3__s_selectionLog__OFFSET: usize = 0;
pub const CSelectionWar3__s_indentLevel__OFFSET: usize = 0;
pub const CSelectionWar3__m_controlGroups__OFFSET: usize = 40;
pub const CSelectionWar3__m_controlGroupsLocal__OFFSET: usize = 440;
pub const CSelectionWar3__m_leaderUnit__OFFSET: usize = 840;
pub const CSelectionWar3__m_selectable__OFFSET: usize = 848;
pub const CSelectionWar3__m_playerId__OFFSET: usize = 856;
pub const CSelectionWar3__m_syncSubgroupList__OFFSET: usize = 864;
pub const CSelectionWar3__m_syncSubgroupCount__OFFSET: usize = 888;
pub const CSelectionWar3__m_syncSubgroupSelected__OFFSET: usize = 896;
pub const CSelectionWar3__m_mirrorSelection__OFFSET: usize = 904;
pub const CSelectionWar3__m_mirrorSelectedSubgroupLeader__OFFSET: usize = 944;
pub const CSelectionWar3__m_mirrorSelectedSubgroupUnitId__OFFSET: usize = 952;
pub const CSelectionWar3__m_localLeaderUnit__OFFSET: usize = 960;
pub const CSelectionWar3__m_localSelectable__OFFSET: usize = 968;
pub const CSelectionWar3__m_localSelection__OFFSET: usize = 976;
pub const CSelectionWar3__m_localSubgroupList__OFFSET: usize = 1016;
pub const CSelectionWar3__m_localSubgroupCount__OFFSET: usize = 1040;
pub const CSelectionWar3__m_localSubgroupSelected__OFFSET: usize = 1048;

#[repr(C)]
pub struct CUnit__3 {
    pub _opaque: [u8; 1864],
}
pub const CUnit__3__SIZE: usize = 1864;
pub const CUnit__3__NAME: &str = "CUnit";
pub const CUnit__3__s_CUnit_pool__OFFSET: usize = 0;
pub const CUnit__3__m_unit_paused_count__OFFSET: usize = 440;
pub const CUnit__3__m_PlayingDialogSound__OFFSET: usize = 444;
pub const CUnit__3__m_owner__OFFSET: usize = 448;
pub const CUnit__3__m_flags0__OFFSET: usize = 452;
pub const CUnit__3__m_flags1__OFFSET: usize = 456;
pub const CUnit__3__m_foodAndTechTreeFlags__OFFSET: usize = 460;
pub const CUnit__3__m_minimapFlashTimer__OFFSET: usize = 464;
pub const CUnit__3__m_damage_dealt__OFFSET: usize = 496;
pub const CUnit__3__m_self_damage__OFFSET: usize = 500;
pub const CUnit__3__m_healing_done__OFFSET: usize = 504;
pub const CUnit__3__m_time_stopped__OFFSET: usize = 508;
pub const CUnit__3__m_total_kills__OFFSET: usize = 512;
pub const CUnit__3__m_self_kills__OFFSET: usize = 516;
pub const CUnit__3__m_hero_kills__OFFSET: usize = 520;
pub const CUnit__3__m_building_kills__OFFSET: usize = 524;
pub const CUnit__3__m_override_ability_id_stats_credit__OFFSET: usize = 528;
pub const CUnit__3__m_override_item_id_stats_credit__OFFSET: usize = 532;
pub const CUnit__3__m_unit_stats_override__OFFSET: usize = 536;
pub const CUnit__3__m_caller__OFFSET: usize = 544;
pub const CUnit__3__m_lastAttackNotificationTime__OFFSET: usize = 576;
pub const CUnit__3__m_lastAttackNotificationSoundTime__OFFSET: usize = 580;
pub const CUnit__3__m_life__OFFSET: usize = 584;
pub const CUnit__3__m_death__OFFSET: usize = 608;
pub const CUnit__3__m_total_life_lost__OFFSET: usize = 616;
pub const CUnit__3__m_stored_time_when_last_alive__OFFSET: usize = 620;
pub const CUnit__3__m_time_alive_ms__OFFSET: usize = 624;
pub const CUnit__3__m_number_of_deaths__OFFSET: usize = 628;
pub const CUnit__3__m_life_regen_rate__OFFSET: usize = 632;
pub const CUnit__3__m_life_regen__OFFSET: usize = 648;
pub const CUnit__3__m_life_regen_type__OFFSET: usize = 656;
pub const CUnit__3__m_mana__OFFSET: usize = 664;
pub const CUnit__3__m_ai_mana__OFFSET: usize = 688;
pub const CUnit__3__m_mana_regen_rate__OFFSET: usize = 704;
pub const CUnit__3__m_mana_regen__OFFSET: usize = 720;
pub const CUnit__3__m_total_mana_spent__OFFSET: usize = 728;
pub const CUnit__3__m_defense__OFFSET: usize = 736;
pub const CUnit__3__m_battle_type__OFFSET: usize = 752;
pub const CUnit__3__m_invulnerable_count__OFFSET: usize = 756;
pub const CUnit__3__m_auto_attack_ignore_count__OFFSET: usize = 760;
pub const CUnit__3__m_fogFlash__OFFSET: usize = 764;
pub const CUnit__3__m_sight__OFFSET: usize = 776;
pub const CUnit__3__m_sightRangeCached__OFFSET: usize = 800;
pub const CUnit__3__m_sightMod__OFFSET: usize = 808;
pub const CUnit__3__m_invisibility_count__OFFSET: usize = 816;
pub const CUnit__3__m_invisibility_percent__OFFSET: usize = 824;
pub const CUnit__3__m_invis_modify__OFFSET: usize = 848;
pub const CUnit__3__m_invis_listen__OFFSET: usize = 856;
pub const CUnit__3__m_detectedData__OFFSET: usize = 864;
pub const CUnit__3__m_sharedVisionData__OFFSET: usize = 876;
pub const CUnit__3__m_sharedVisionDataCached__OFFSET: usize = 888;
pub const CUnit__3__m_sharedVisionTransitiveDataCached__OFFSET: usize = 892;
pub const CUnit__3__m_frost_count__OFFSET: usize = 896;
pub const CUnit__3__m_stone_count__OFFSET: usize = 900;
pub const CUnit__3__m_ethereal_count__OFFSET: usize = 904;
pub const CUnit__3__m_magic_immunity_count__OFFSET: usize = 908;
pub const CUnit__3__m_dispel_immunity_count__OFFSET: usize = 912;
pub const CUnit__3__m_fog_radius__OFFSET: usize = 920;
pub const CUnit__3__m_pos__OFFSET: usize = 936;
pub const CUnit__3__m_exp_level__OFFSET: usize = 960;
pub const CUnit__3__MaxNameLength__OFFSET: usize = 0;
pub const CUnit__3__m_unit_proper_name__OFFSET: usize = 964;
pub const CUnit__3__m_pick_order__OFFSET: usize = 1224;
pub const CUnit__3__m_task_stack__OFFSET: usize = 1228;
pub const CUnit__3__m_task_timer__OFFSET: usize = 1240;
pub const CUnit__3__m_action__OFFSET: usize = 1272;
pub const CUnit__3__m_uninterruptable_count__OFFSET: usize = 1276;
pub const CUnit__3__m_order_head__OFFSET: usize = 1280;
pub const CUnit__3__m_order_tail__OFFSET: usize = 1292;
pub const CUnit__3__order_count__OFFSET: usize = 1304;
pub const CUnit__3__m_gold_bounty_dice__OFFSET: usize = 1308;
pub const CUnit__3__m_gold_bounty_sides__OFFSET: usize = 1312;
pub const CUnit__3__m_gold_bounty_plus__OFFSET: usize = 1316;
pub const CUnit__3__m_wood_bounty_dice__OFFSET: usize = 1320;
pub const CUnit__3__m_wood_bounty_sides__OFFSET: usize = 1324;
pub const CUnit__3__m_wood_bounty_plus__OFFSET: usize = 1328;
pub const CUnit__3__m_cast_point__OFFSET: usize = 1336;
pub const CUnit__3__m_cast_back_swing__OFFSET: usize = 1352;
pub const CUnit__3__m_abilities__OFFSET: usize = 1368;
pub const CUnit__3__m_disabled_count__OFFSET: usize = 1380;
pub const CUnit__3__m_disabled_hidden_magic_count__OFFSET: usize = 1384;
pub const CUnit__3__m_silence_count__OFFSET: usize = 1388;
pub const CUnit__3__anticheat_dummy_array_12__OFFSET: usize = 1392;
pub const CUnit__3__m_disabled_magic_count__OFFSET: usize = 1404;
pub const CUnit__3__m_disabled_hidden_count__OFFSET: usize = 1408;
pub const CUnit__3__m_abil_paused_count__OFFSET: usize = 1412;
pub const CUnit__3__m_disabled_hidden_physical_count__OFFSET: usize = 1416;
pub const CUnit__3__m_disabled_physical_count__OFFSET: usize = 1420;
pub const CUnit__3__anticheat_dummy_array_13__OFFSET: usize = 1424;
pub const CUnit__3__m_buff_paused_count__OFFSET: usize = 1432;
pub const CUnit__3__m_abil_inv__OFFSET: usize = 1440;
pub const CUnit__3__m_abil_hero__OFFSET: usize = 1448;
pub const CUnit__3__m_abil_move__OFFSET: usize = 1456;
pub const CUnit__3__anticheat_dummy_array_14__OFFSET: usize = 1464;
pub const CUnit__3__m_abil_attack__OFFSET: usize = 1472;
pub const CUnit__3__m_abil_build__OFFSET: usize = 1480;
pub const CUnit__3__m_base_priority__OFFSET: usize = 1488;
pub const CUnit__3__m_death_time__OFFSET: usize = 1504;
pub const CUnit__3__s_minUnitSpeed__OFFSET: usize = 0;
pub const CUnit__3__s_maxUnitSpeed__OFFSET: usize = 0;
pub const CUnit__3__s_minBldgSpeed__OFFSET: usize = 0;
pub const CUnit__3__s_maxBldgSpeed__OFFSET: usize = 0;
pub const CUnit__3__s_buildingKillsGiveExp__OFFSET: usize = 0;
pub const CUnit__3__s_globalExperience__OFFSET: usize = 0;
pub const CUnit__3__s_maxLevelHeroesDrainExp__OFFSET: usize = 0;
pub const CUnit__3__m_display_height__OFFSET: usize = 1520;
pub const CUnit__3__m_display_height_modifier__OFFSET: usize = 1544;
pub const CUnit__3__anticheat_dummy_array_15__OFFSET: usize = 1552;
pub const CUnit__3__m_last_cluster_birth_tag__OFFSET: usize = 1560;
pub const CUnit__3__m_last_cluster_presence_tag__OFFSET: usize = 1564;
pub const CUnit__3__m_move_type__OFFSET: usize = 1568;
pub const CUnit__3__m_ground_count__OFFSET: usize = 1572;
pub const CUnit__3__m_launch_offset__OFFSET: usize = 1576;
pub const CUnit__3__m_fly_height__OFFSET: usize = 1592;
pub const CUnit__3__m_impact_swim__OFFSET: usize = 1616;
pub const CUnit__3__anticheat_dummy_array_16__OFFSET: usize = 1620;
pub const CUnit__3__m_launch_swim_z__OFFSET: usize = 1624;
pub const CUnit__3__m_impact_offset__OFFSET: usize = 1628;
pub const CUnit__3__m_category__OFFSET: usize = 1632;
pub const CUnit__3__m_target_type_flags__OFFSET: usize = 1636;
pub const CUnit__3__m_cargo_size__OFFSET: usize = 1640;
pub const CUnit__3__m_killedBy__OFFSET: usize = 1644;
pub const CUnit__3__m_shadow__OFFSET: usize = 1656;
pub const CUnit__3__m_shadowWater__OFFSET: usize = 1660;
pub const CUnit__3__anticheat_dummy_array_17__OFFSET: usize = 1664;
pub const CUnit__3__m_shadowAboveWater__OFFSET: usize = 1671;
pub const CUnit__3__m_shadowShow__OFFSET: usize = 1672;
pub const CUnit__3__m_showOccMark__OFFSET: usize = 1673;
pub const CUnit__3__m_allySelectionImage__OFFSET: usize = 1676;
pub const CUnit__3__m_occluded__OFFSET: usize = 1680;
pub const CUnit__3__anticheat_dummy_array_18__OFFSET: usize = 1681;
pub const CUnit__3__m_occlusionMark__OFFSET: usize = 1684;
pub const CUnit__3__m_jassUserData__OFFSET: usize = 1688;
pub const CUnit__3__m_foliageExclusionId__OFFSET: usize = 1692;
pub const CUnit__3__m_uiFlags__OFFSET: usize = 1696;
pub const CUnit__3__m_cachedPos__OFFSET: usize = 1700;
pub const CUnit__3__m_cachedFacing__OFFSET: usize = 1712;
pub const CUnit__3__m_cachedOrientation__OFFSET: usize = 1716;
pub const CUnit__3__m_lookAtTarget__OFFSET: usize = 1752;
pub const CUnit__3__m_lookAtOffset__OFFSET: usize = 1760;
pub const CUnit__3__m_overheadOffset__OFFSET: usize = 1772;
pub const CUnit__3__m_occupUI__OFFSET: usize = 1776;
pub const CUnit__3__m_unitStatusIconUI__OFFSET: usize = 1784;
pub const CUnit__3__m_unitStatusTextUI__OFFSET: usize = 1792;
pub const CUnit__3__m_uberSplatId__OFFSET: usize = 1800;
pub const CUnit__3__m_vertexColor__OFFSET: usize = 1804;
pub const CUnit__3__m_anim_size__OFFSET: usize = 1808;
pub const CUnit__3__m_anims__OFFSET: usize = 1812;
pub const CUnit__3__m_forcedAnimScale__OFFSET: usize = 1840;
pub const CUnit__3__m_walk_anim_speed__OFFSET: usize = 1844;
pub const CUnit__3__m_run_anim_speed__OFFSET: usize = 1848;
pub const CUnit__3__m_unit_ai__OFFSET: usize = 1852;

#[repr(C)]
pub struct PlayerOverallScoreData {
    pub _opaque: [u8; 24],
}
pub const PlayerOverallScoreData__SIZE: usize = 24;
pub const PlayerOverallScoreData__NAME: &str = "PlayerOverallScoreData";
pub const PlayerOverallScoreData__score__OFFSET: usize = 0;

#[repr(C)]
pub struct PlayerUnitScoreData {
    pub _opaque: [u8; 24],
}
pub const PlayerUnitScoreData__SIZE: usize = 24;
pub const PlayerUnitScoreData__NAME: &str = "PlayerUnitScoreData";
pub const PlayerUnitScoreData__score__OFFSET: usize = 0;

#[repr(C)]
pub struct PlayerHeroScoreData {
    pub _opaque: [u8; 24],
}
pub const PlayerHeroScoreData__SIZE: usize = 24;
pub const PlayerHeroScoreData__NAME: &str = "PlayerHeroScoreData";
pub const PlayerHeroScoreData__score__OFFSET: usize = 0;

#[repr(C)]
pub struct PlayerResourceScoreData {
    pub _opaque: [u8; 24],
}
pub const PlayerResourceScoreData__SIZE: usize = 24;
pub const PlayerResourceScoreData__NAME: &str = "PlayerResourceScoreData";
pub const PlayerResourceScoreData__score__OFFSET: usize = 0;

#[repr(C)]
pub struct PlayerScoreData {
    pub _opaque: [u8; 176],
}
pub const PlayerScoreData__SIZE: usize = 176;
pub const PlayerScoreData__NAME: &str = "PlayerScoreData";
pub const PlayerScoreData__rawOrderPos__OFFSET: usize = 0;
pub const PlayerScoreData__orderPos__OFFSET: usize = 4;
pub const PlayerScoreData__race__OFFSET: usize = 8;
pub const PlayerScoreData__gamePlayerId__OFFSET: usize = 12;
pub const PlayerScoreData__color__OFFSET: usize = 16;
pub const PlayerScoreData__team__OFFSET: usize = 20;
pub const PlayerScoreData__hero__OFFSET: usize = 24;
pub const PlayerScoreData__heroLevel__OFFSET: usize = 28;
pub const PlayerScoreData__otherHero__OFFSET: usize = 32;
pub const PlayerScoreData__otherHeroLevel__OFFSET: usize = 36;
pub const PlayerScoreData__lastHero__OFFSET: usize = 40;
pub const PlayerScoreData__lastHeroLevel__OFFSET: usize = 44;
pub const PlayerScoreData__won__OFFSET: usize = 48;
pub const PlayerScoreData__name__OFFSET: usize = 56;
pub const PlayerScoreData__overallScore__OFFSET: usize = 80;
pub const PlayerScoreData__unitScore__OFFSET: usize = 104;
pub const PlayerScoreData__heroScore__OFFSET: usize = 128;
pub const PlayerScoreData__resourceScore__OFFSET: usize = 152;

#[repr(C)]
pub struct CGameListCustomData {
    pub _opaque: [u8; 400],
}
pub const CGameListCustomData__SIZE: usize = 400;
pub const CGameListCustomData__NAME: &str = "CGameListCustomData";
pub const CGameListCustomData__m_gameAdId__OFFSET: usize = 16;
pub const CGameListCustomData__m_gameData__OFFSET: usize = 20;
pub const CGameListCustomData__m_numPlayers__OFFSET: usize = 396;

#[repr(C)]
pub struct CGameChatroom {
    pub _opaque: [u8; 984],
}
pub const CGameChatroom__SIZE: usize = 984;
pub const CGameChatroom__NAME: &str = "CGameChatroom";
pub const CGameChatroom__m_session__OFFSET: usize = 672;
pub const CGameChatroom__m_controlSet__OFFSET: usize = 680;
pub const CGameChatroom__m_fadeTimer__OFFSET: usize = 704;
pub const CGameChatroom__m_startTimeElapsed__OFFSET: usize = 808;
pub const CGameChatroom__m_startTimer__OFFSET: usize = 816;
pub const CGameChatroom__m_gameTimer__OFFSET: usize = 856;
pub const CGameChatroom__m_teamSetup__OFFSET: usize = 896;
pub const CGameChatroom__m_chatTextArea__OFFSET: usize = 904;
pub const CGameChatroom__m_chatEditBox__OFFSET: usize = 912;
pub const CGameChatroom__m_gameNameLabel__OFFSET: usize = 920;
pub const CGameChatroom__m_gameNameValue__OFFSET: usize = 928;
pub const CGameChatroom__m_mapInfoPane__OFFSET: usize = 936;
pub const CGameChatroom__m_startGameBackdrop__OFFSET: usize = 944;
pub const CGameChatroom__m_startGameButton__OFFSET: usize = 952;
pub const CGameChatroom__m_cancelButton__OFFSET: usize = 960;
pub const CGameChatroom__m_advancedOptionsDisplay__OFFSET: usize = 968;
pub const CGameChatroom__m_netGameId__OFFSET: usize = 976;

#[repr(C)]
pub struct CPlayerWar3__2 {
    pub _opaque: [u8; 1960],
}
pub const CPlayerWar3__2__SIZE: usize = 1960;
pub const CPlayerWar3__2__NAME: &str = "CPlayerWar3";
pub const CPlayerWar3__2__s_CPlayerWar3_pool__OFFSET: usize = 0;
pub const CPlayerWar3__2__anticheat_dummy_array_20__OFFSET: usize = 88;
pub const CPlayerWar3__2__m_playerId__OFFSET: usize = 103;
pub const CPlayerWar3__2__m_playerName__OFFSET: usize = 104;
pub const CPlayerWar3__2__m_toonName__OFFSET: usize = 128;
pub const CPlayerWar3__2__m_protobufPlayerData__OFFSET: usize = 152;
pub const CPlayerWar3__2__m_slotNetPlayerId__OFFSET: usize = 336;
pub const CPlayerWar3__2__m_playerNamePrefix__OFFSET: usize = 344;
pub const CPlayerWar3__2__m_pAlliances__OFFSET: usize = 368;
pub const CPlayerWar3__2__m_pSelection__OFFSET: usize = 376;
pub const CPlayerWar3__2__m_playerStates__OFFSET: usize = 384;
pub const CPlayerWar3__2__m_lumberDivertTax__OFFSET: usize = 1016;
pub const CPlayerWar3__2__m_goldDivertTax__OFFSET: usize = 1116;
pub const CPlayerWar3__2__m_highestFood__OFFSET: usize = 1216;
pub const CPlayerWar3__2__m_highestFoodUsed__OFFSET: usize = 1220;
pub const CPlayerWar3__2__anticheat_dummy_array_21__OFFSET: usize = 1224;
pub const CPlayerWar3__2__m_heroKills__OFFSET: usize = 1240;
pub const CPlayerWar3__2__m_itemsObtained__OFFSET: usize = 1244;
pub const CPlayerWar3__2__m_mercsHired__OFFSET: usize = 1248;
pub const CPlayerWar3__2__m_teamColorIndex__OFFSET: usize = 1252;
pub const CPlayerWar3__2__m_controller__OFFSET: usize = 1256;
pub const CPlayerWar3__2__m_startLocIndex__OFFSET: usize = 1260;
pub const CPlayerWar3__2__m_difficulty__OFFSET: usize = 1264;
pub const CPlayerWar3__2__m_slotState__OFFSET: usize = 1268;
pub const CPlayerWar3__2__m_teamIndex__OFFSET: usize = 1272;
pub const CPlayerWar3__2__m_numActions__OFFSET: usize = 1276;
pub const CPlayerWar3__2__m_racePref__OFFSET: usize = 1280;
pub const CPlayerWar3__2__m_race__OFFSET: usize = 1284;
pub const CPlayerWar3__2__m_realTimeAPM__OFFSET: usize = 1288;
pub const CPlayerWar3__2__m_capChangedListener__OFFSET: usize = 1384;
pub const CPlayerWar3__2__m_usedChangedListener__OFFSET: usize = 1392;
pub const CPlayerWar3__2__m_foodUsedUpkeepListener__OFFSET: usize = 1400;
pub const CPlayerWar3__2__m_lumberChangedListener__OFFSET: usize = 1408;
pub const CPlayerWar3__2__m_goldChangedListener__OFFSET: usize = 1416;
pub const CPlayerWar3__2__m_goldUpkeepChanged__OFFSET: usize = 1424;
pub const CPlayerWar3__2__m_lumberUpkeepChanged__OFFSET: usize = 1432;
pub const CPlayerWar3__2__m_handicap__OFFSET: usize = 1440;
pub const CPlayerWar3__2__m_referee__OFFSET: usize = 1512;
pub const CPlayerWar3__2__m_deadHeroAge__OFFSET: usize = 1520;
pub const CPlayerWar3__2__m_deadHeroes__OFFSET: usize = 1544;
pub const CPlayerWar3__2__m_revivableFlags__OFFSET: usize = 1568;
pub const CPlayerWar3__2__anticheat_dummy_array_22__OFFSET: usize = 1572;
pub const CPlayerWar3__2__m_pTechTree__OFFSET: usize = 1584;
pub const CPlayerWar3__2__m_fogMask__OFFSET: usize = 1592;
pub const CPlayerWar3__2__m_ai__OFFSET: usize = 1596;
pub const CPlayerWar3__2__m_bestHeroes__OFFSET: usize = 1608;
pub const CPlayerWar3__2__m_leaderboard__OFFSET: usize = 1632;
pub const CPlayerWar3__2__m_leaveGameTimer__OFFSET: usize = 1648;
pub const CPlayerWar3__2__anticheat_dummy_array_23__OFFSET: usize = 1680;
pub const CPlayerWar3__2__m_doScoreScreen__OFFSET: usize = 1708;
pub const CPlayerWar3__2__m_allowContinue__OFFSET: usize = 1709;
pub const CPlayerWar3__2__m_crippled__OFFSET: usize = 1712;
pub const CPlayerWar3__2__m_gameOverMessage__OFFSET: usize = 1720;
pub const CPlayerWar3__2__m_showInScoreScreen__OFFSET: usize = 1744;
pub const CPlayerWar3__2__m_cachedScore__OFFSET: usize = 1748;
pub const CPlayerWar3__2__m_cachedScoreIsCached__OFFSET: usize = 1752;
pub const CPlayerWar3__2__m_heroCount__OFFSET: usize = 1756;
pub const CPlayerWar3__2__m_bnetPlayerId__OFFSET: usize = 1760;
pub const CPlayerWar3__2__m_heroPickOrder__OFFSET: usize = 1768;
pub const CPlayerWar3__2__m_unitKills__OFFSET: usize = 1792;
pub const CPlayerWar3__2__m_unitsCreated__OFFSET: usize = 1824;
pub const CPlayerWar3__2__m_abilityInfo__OFFSET: usize = 1856;
pub const CPlayerWar3__2__m_upkeepTime__OFFSET: usize = 1888;
pub const CPlayerWar3__2__m_lastUpkeepTime__OFFSET: usize = 1920;
pub const CPlayerWar3__2__m_playerItemUseMap__OFFSET: usize = 1928;

#[repr(C)]
pub struct CGameWar3__2 {
    pub _opaque: [u8; 12872],
}
pub const CGameWar3__2__SIZE: usize = 12872;
pub const CGameWar3__2__NAME: &str = "CGameWar3";
pub const CGameWar3__2__s_campaignArchiveFile__OFFSET: usize = 0;
pub const CGameWar3__2__anticheat_dummy_array_24__OFFSET: usize = 9640;
pub const CGameWar3__2__s_pGameWar3__OFFSET: usize = 0;
pub const CGameWar3__2__anticheat_dummy_array_25__OFFSET: usize = 9695;
pub const CGameWar3__2__m_mapScriptInstance__OFFSET: usize = 9784;
pub const CGameWar3__2__m_mapScriptInstanceId__OFFSET: usize = 9792;
pub const CGameWar3__2__m_mapScript__OFFSET: usize = 9800;
pub const CGameWar3__2__m_mapScriptId__OFFSET: usize = 9808;
pub const CGameWar3__2__m_executions__OFFSET: usize = 9816;
pub const CGameWar3__2__anticheat_dummy_array_26__OFFSET: usize = 9840;
pub const CGameWar3__2__m_gameIdMap__OFFSET: usize = 9864;
pub const CGameWar3__2__m_gameState__OFFSET: usize = 9872;
pub const CGameWar3__2__m_buildId__OFFSET: usize = 9880;
pub const CGameWar3__2__m_localPlayerId__OFFSET: usize = 9884;
pub const CGameWar3__2__m_replayLocalPlayerId__OFFSET: usize = 9886;
pub const CGameWar3__2__m_playerIdBitsToGhost__OFFSET: usize = 9888;
pub const CGameWar3__2__m_playerIdBitsToGhostSet__OFFSET: usize = 9892;
pub const CGameWar3__2__m_pMapSetup__OFFSET: usize = 9896;
pub const CGameWar3__2__m_pFog__OFFSET: usize = 9904;
pub const CGameWar3__2__m_gameName__OFFSET: usize = 9912;
pub const CGameWar3__2__anticheat_dummy_array_27__OFFSET: usize = 9936;
pub const CGameWar3__2__m_maxPlayerCount__OFFSET: usize = 9952;
pub const CGameWar3__2__m_playerCount__OFFSET: usize = 9956;
pub const CGameWar3__2__m_gameMaxPlayerCount__OFFSET: usize = 9960;
pub const CGameWar3__2__m_maxTeamCount__OFFSET: usize = 9964;
pub const CGameWar3__2__m_teams__OFFSET: usize = 9968;
pub const CGameWar3__2__m_players__OFFSET: usize = 9992;
pub const CGameWar3__2__m_startLocations__OFFSET: usize = 10224;
pub const CGameWar3__2__m_slotToPlayerId__OFFSET: usize = 11960;
pub const CGameWar3__2__m_levelToTransitionTo__OFFSET: usize = 12064;
pub const CGameWar3__2__m_modelCine__OFFSET: usize = 12088;
pub const CGameWar3__2__m_cine__OFFSET: usize = 12112;
pub const CGameWar3__2__m_gameOverMessage__OFFSET: usize = 12376;
pub const CGameWar3__2__m_switchingLevels__OFFSET: usize = 12400;
pub const CGameWar3__2__m_doScoreScreen__OFFSET: usize = 12401;
pub const CGameWar3__2__m_isReload__OFFSET: usize = 12402;
pub const CGameWar3__2__anticheat_dummy_array_28__OFFSET: usize = 12403;
pub const CGameWar3__2__m_wasScriptLoad__OFFSET: usize = 12410;
pub const CGameWar3__2__m_wasLoadGame__OFFSET: usize = 12411;
pub const CGameWar3__2__m_isCampaignMission__OFFSET: usize = 12412;
pub const CGameWar3__2__m_campaignIndex__OFFSET: usize = 12416;
pub const CGameWar3__2__m_gameSetup__OFFSET: usize = 12420;
pub const CGameWar3__2__m_usingDarkMask__OFFSET: usize = 12678;
pub const CGameWar3__2__m_unseenQuests__OFFSET: usize = 12679;
pub const CGameWar3__2__m_showAllUnitsCmdBar__OFFSET: usize = 12680;
pub const CGameWar3__2__m_isReplay__OFFSET: usize = 12681;
pub const CGameWar3__2__m_replayFile__OFFSET: usize = 12688;
pub const CGameWar3__2__m_isTournament__OFFSET: usize = 12712;
pub const CGameWar3__2__m_copySaveCount__OFFSET: usize = 12716;
pub const CGameWar3__2__m_finishedInit__OFFSET: usize = 12720;
pub const CGameWar3__2__m_didSaveGameFromScript__OFFSET: usize = 12721;
pub const CGameWar3__2__m_playingGame__OFFSET: usize = 12722;
pub const CGameWar3__2__m_FacialAnimationSetScopeID__OFFSET: usize = 12724;
pub const CGameWar3__2__m_pTriggerStrings__OFFSET: usize = 12728;
pub const CGameWar3__2__anticheat_dummy_array_29__OFFSET: usize = 12736;
pub const CGameWar3__2__m_messageLogContents__OFFSET: usize = 12776;
pub const CGameWar3__2__m_pGameHashTableManager__OFFSET: usize = 12800;
pub const CGameWar3__2__m_gameCacheManager__OFFSET: usize = 12808;
pub const CGameWar3__2__m_gameOverCallback__OFFSET: usize = 12816;
pub const CGameWar3__2__m_maxCheckpointSaves__OFFSET: usize = 12824;
pub const CGameWar3__2__m_pSaveThread__OFFSET: usize = 12832;
pub const CGameWar3__2__m_pCheckpointPopup__OFFSET: usize = 12840;
pub const CGameWar3__2__m_gamePlayStartTime__OFFSET: usize = 12848;
pub const CGameWar3__2__m_latency__OFFSET: usize = 12856;

#[repr(C)]
pub struct CGameState {
    pub _opaque: [u8; 1416],
}
pub const CGameState__SIZE: usize = 1416;
pub const CGameState__NAME: &str = "CGameState";
pub const CGameState__s_CGameState_pool__OFFSET: usize = 0;
pub const CGameState__m_gameStatesFloat__OFFSET: usize = 88;
pub const CGameState__m_gameStatesInt__OFFSET: usize = 120;
pub const CGameState__m_timeScale__OFFSET: usize = 176;
pub const CGameState__m_todHaltFlags__OFFSET: usize = 200;
pub const CGameState__m_todMod__OFFSET: usize = 208;
pub const CGameState__m_todLis__OFFSET: usize = 216;
pub const CGameState__m_dawnLis__OFFSET: usize = 224;
pub const CGameState__m_duskLis__OFFSET: usize = 232;
pub const CGameState__m_isDayTime__OFFSET: usize = 240;
pub const CGameState__m_agentStacks__OFFSET: usize = 264;
pub const CGameState__m_unusedHandles__OFFSET: usize = 632;
pub const CGameState__m_handle2AgentMap__OFFSET: usize = 656;
pub const CGameState__m_agent2HandleMap__OFFSET: usize = 680;
pub const CGameState__m_func2AgentMap__OFFSET: usize = 752;
pub const CGameState__m_filter2AgentMap__OFFSET: usize = 824;
pub const CGameState__m_frame2AgentMap__OFFSET: usize = 896;
pub const CGameState__m_variableEvents__OFFSET: usize = 928;
pub const CGameState__m_varChangedEvent__OFFSET: usize = 1000;
pub const CGameState__m_heroNamesUsed__OFFSET: usize = 1024;
pub const CGameState__m_fogTimer__OFFSET: usize = 1096;
pub const CGameState__m_pathingTimer__OFFSET: usize = 1128;
pub const CGameState__m_levelChangeTimer__OFFSET: usize = 1160;
pub const CGameState__m_scriptGCTimer__OFFSET: usize = 1192;
pub const CGameState__m_questManager__OFFSET: usize = 1224;
pub const CGameState__m_textTagManager__OFFSET: usize = 1240;

#[repr(C)]
pub struct CPlayerEventReg {
    pub _opaque: [u8; 136],
}
pub const CPlayerEventReg__SIZE: usize = 136;
pub const CPlayerEventReg__NAME: &str = "CPlayerEventReg";
pub const CPlayerEventReg__s_CPlayerEventReg_pool__OFFSET: usize = 0;
pub const CPlayerEventReg__m_eventId__OFFSET: usize = 104;
pub const CPlayerEventReg__m_player__OFFSET: usize = 108;
pub const CPlayerEventReg__m_state__OFFSET: usize = 120;
pub const CPlayerEventReg__m_unitFilter__OFFSET: usize = 128;

#[repr(C)]
pub struct CPlayerWar3FrameEventReg {
    pub _opaque: [u8; 112],
}
pub const CPlayerWar3FrameEventReg__SIZE: usize = 112;
pub const CPlayerWar3FrameEventReg__NAME: &str = "CPlayerWar3FrameEventReg";
pub const CPlayerWar3FrameEventReg__s_CPlayerWar3FrameEventReg_pool__OFFSET: usize = 0;
pub const CPlayerWar3FrameEventReg__m_frame__OFFSET: usize = 104;

#[repr(C)]
pub struct CPlayerWar3FrameEventData {
    pub _opaque: [u8; 104],
}
pub const CPlayerWar3FrameEventData__SIZE: usize = 104;
pub const CPlayerWar3FrameEventData__NAME: &str = "CPlayerWar3FrameEventData";
pub const CPlayerWar3FrameEventData__s_CPlayerWar3FrameEventData_pool__OFFSET: usize = 0;
pub const CPlayerWar3FrameEventData__m_frame__OFFSET: usize = 56;
pub const CPlayerWar3FrameEventData__m_eventId__OFFSET: usize = 68;
pub const CPlayerWar3FrameEventData__m_val__OFFSET: usize = 72;
pub const CPlayerWar3FrameEventData__m_text__OFFSET: usize = 80;

#[repr(C)]
pub struct CGameVarChangedEvent {
    pub _opaque: [u8; 24],
}
pub const CGameVarChangedEvent__SIZE: usize = 24;
pub const CGameVarChangedEvent__NAME: &str = "CGameVarChangedEvent";
pub const CGameVarChangedEvent__m_newValue__OFFSET: usize = 16;

#[repr(C)]
pub struct CUnitListNode {
    pub _opaque: [u8; 24],
}
pub const CUnitListNode__SIZE: usize = 24;
pub const CUnitListNode__NAME: &str = "CUnitListNode";
pub const CUnitListNode__unit__OFFSET: usize = 16;

#[repr(C)]
pub struct CUnitSet {
    pub _opaque: [u8; 40],
}
pub const CUnitSet__SIZE: usize = 40;
pub const CUnitSet__NAME: &str = "CUnitSet";
pub const CUnitSet__m_units__OFFSET: usize = 8;
pub const CUnitSet__m_count__OFFSET: usize = 32;
pub const CUnitSet__m_enumerating__OFFSET: usize = 36;

#[repr(C)]
pub struct GameVarEventReg {
    pub _opaque: [u8; 56],
}
pub const GameVarEventReg__SIZE: usize = 56;
pub const GameVarEventReg__NAME: &str = "GameVarEventReg";
pub const GameVarEventReg__varEvent__OFFSET: usize = 48;

#[repr(C)]
pub struct UnitDatabaseFile {
    pub _opaque: [u8; 16],
}
pub const UnitDatabaseFile__SIZE: usize = 16;
pub const UnitDatabaseFile__NAME: &str = "UnitDatabaseFile";
pub const UnitDatabaseFile__db__OFFSET: usize = 0;
pub const UnitDatabaseFile__fileID__OFFSET: usize = 8;

#[repr(C)]
pub struct UnitDatabaseEntry {
    pub _opaque: [u8; 16],
}
pub const UnitDatabaseEntry__SIZE: usize = 16;
pub const UnitDatabaseEntry__NAME: &str = "UnitDatabaseEntry";
pub const UnitDatabaseEntry__dbEntry__OFFSET: usize = 0;
pub const UnitDatabaseEntry__unitID__OFFSET: usize = 8;
pub const UnitDatabaseEntry__fileID__OFFSET: usize = 12;

#[repr(C)]
pub struct UnitDatabaseField {
    pub _opaque: [u8; 16],
}
pub const UnitDatabaseField__SIZE: usize = 16;
pub const UnitDatabaseField__NAME: &str = "UnitDatabaseField";
pub const UnitDatabaseField__dbField__OFFSET: usize = 0;
pub const UnitDatabaseField__fieldID__OFFSET: usize = 8;
pub const UnitDatabaseField__fileID__OFFSET: usize = 12;

#[repr(C)]
pub struct CUnitDatabase {
    pub _opaque: [u8; 12776],
}
pub const CUnitDatabase__SIZE: usize = 12776;
pub const CUnitDatabase__NAME: &str = "CUnitDatabase";
pub const CUnitDatabase__mFiles__OFFSET: usize = 9656;
pub const CUnitDatabase__m_unitID__OFFSET: usize = 9736;
pub const CUnitDatabase__m_race__OFFSET: usize = 9752;
pub const CUnitDatabase__m_valid__OFFSET: usize = 9768;
pub const CUnitDatabase__m_deathType__OFFSET: usize = 9784;
pub const CUnitDatabase__m_prio__OFFSET: usize = 9800;
pub const CUnitDatabase__m_turnRate__OFFSET: usize = 9816;
pub const CUnitDatabase__m_propWin__OFFSET: usize = 9832;
pub const CUnitDatabase__m_orientInterp__OFFSET: usize = 9848;
pub const CUnitDatabase__m_formation__OFFSET: usize = 9864;
pub const CUnitDatabase__m_death__OFFSET: usize = 9880;
pub const CUnitDatabase__m_threat__OFFSET: usize = 9896;
pub const CUnitDatabase__m_canSleep__OFFSET: usize = 9912;
pub const CUnitDatabase__m_cargoSize__OFFSET: usize = 9928;
pub const CUnitDatabase__m_targType__OFFSET: usize = 9944;
pub const CUnitDatabase__m_movetp__OFFSET: usize = 9960;
pub const CUnitDatabase__m_moveHeight__OFFSET: usize = 9976;
pub const CUnitDatabase__m_moveFloor__OFFSET: usize = 9992;
pub const CUnitDatabase__m_pathTex__OFFSET: usize = 10008;
pub const CUnitDatabase__m_fatLOS__OFFSET: usize = 10024;
pub const CUnitDatabase__m_points__OFFSET: usize = 10040;
pub const CUnitDatabase__m_sort__OFFSET: usize = 10056;
pub const CUnitDatabase__m_buffType__OFFSET: usize = 10072;
pub const CUnitDatabase__m_buffRadius__OFFSET: usize = 10088;
pub const CUnitDatabase__m_nameCount__OFFSET: usize = 10104;
pub const CUnitDatabase__m_version__OFFSET: usize = 10120;
pub const CUnitDatabase__m_canFlee__OFFSET: usize = 10136;
pub const CUnitDatabase__m_requireWaterRadius__OFFSET: usize = 10152;
pub const CUnitDatabase__m_isBuildOn__OFFSET: usize = 10168;
pub const CUnitDatabase__m_canBuildOn__OFFSET: usize = 10184;
pub const CUnitDatabase__m_unitBalanceID__OFFSET: usize = 10200;
pub const CUnitDatabase__m_level__OFFSET: usize = 10216;
pub const CUnitDatabase__m_type__OFFSET: usize = 10232;
pub const CUnitDatabase__m_goldcost__OFFSET: usize = 10248;
pub const CUnitDatabase__m_lumbercost__OFFSET: usize = 10264;
pub const CUnitDatabase__m_goldRep__OFFSET: usize = 10280;
pub const CUnitDatabase__m_lumberRep__OFFSET: usize = 10296;
pub const CUnitDatabase__m_fmade__OFFSET: usize = 10312;
pub const CUnitDatabase__m_fused__OFFSET: usize = 10328;
pub const CUnitDatabase__m_stockMax__OFFSET: usize = 10344;
pub const CUnitDatabase__m_stockRegen__OFFSET: usize = 10360;
pub const CUnitDatabase__m_stockStart__OFFSET: usize = 10376;
pub const CUnitDatabase__m_stockInitial__OFFSET: usize = 10392;
pub const CUnitDatabase__m_bountydice__OFFSET: usize = 10408;
pub const CUnitDatabase__m_bountysides__OFFSET: usize = 10424;
pub const CUnitDatabase__m_bountyplus__OFFSET: usize = 10440;
pub const CUnitDatabase__m_lumberbountydice__OFFSET: usize = 10456;
pub const CUnitDatabase__m_lumberbountysides__OFFSET: usize = 10472;
pub const CUnitDatabase__m_lumberbountyplus__OFFSET: usize = 10488;
pub const CUnitDatabase__m_HP__OFFSET: usize = 10504;
pub const CUnitDatabase__m_regenHP__OFFSET: usize = 10520;
pub const CUnitDatabase__m_regenType__OFFSET: usize = 10536;
pub const CUnitDatabase__m_manaN__OFFSET: usize = 10552;
pub const CUnitDatabase__m_mana0__OFFSET: usize = 10568;
pub const CUnitDatabase__m_regenMana__OFFSET: usize = 10584;
pub const CUnitDatabase__m_spd__OFFSET: usize = 10600;
pub const CUnitDatabase__m_minSpd__OFFSET: usize = 10616;
pub const CUnitDatabase__m_maxSpd__OFFSET: usize = 10632;
pub const CUnitDatabase__m_def__OFFSET: usize = 10648;
pub const CUnitDatabase__m_defUp__OFFSET: usize = 10664;
pub const CUnitDatabase__m_defType__OFFSET: usize = 10680;
pub const CUnitDatabase__m_bldtm__OFFSET: usize = 10696;
pub const CUnitDatabase__m_reptm__OFFSET: usize = 10712;
pub const CUnitDatabase__m_sight__OFFSET: usize = 10728;
pub const CUnitDatabase__m_nsight__OFFSET: usize = 10744;
pub const CUnitDatabase__m_STR__OFFSET: usize = 10760;
pub const CUnitDatabase__m_AGI__OFFSET: usize = 10776;
pub const CUnitDatabase__m_INT__OFFSET: usize = 10792;
pub const CUnitDatabase__m_STRplus__OFFSET: usize = 10808;
pub const CUnitDatabase__m_AGIplus__OFFSET: usize = 10824;
pub const CUnitDatabase__m_INTplus__OFFSET: usize = 10840;
pub const CUnitDatabase__m_Primary__OFFSET: usize = 10856;
pub const CUnitDatabase__m_upgrades__OFFSET: usize = 10872;
pub const CUnitDatabase__m_tilesets__OFFSET: usize = 10888;
pub const CUnitDatabase__m_nbrandom__OFFSET: usize = 10904;
pub const CUnitDatabase__m_repulse__OFFSET: usize = 10920;
pub const CUnitDatabase__m_repulseParam__OFFSET: usize = 10936;
pub const CUnitDatabase__m_repulseGroup__OFFSET: usize = 10952;
pub const CUnitDatabase__m_repulsePrio__OFFSET: usize = 10968;
pub const CUnitDatabase__m_preventPlace__OFFSET: usize = 10984;
pub const CUnitDatabase__m_requirePlace__OFFSET: usize = 11000;
pub const CUnitDatabase__m_isbldg__OFFSET: usize = 11016;
pub const CUnitDatabase__m_collision__OFFSET: usize = 11032;
pub const CUnitDatabase__m_unitUIID__OFFSET: usize = 11048;
pub const CUnitDatabase__m_scale__OFFSET: usize = 11064;
pub const CUnitDatabase__m_legacyScale__OFFSET: usize = 11080;
pub const CUnitDatabase__m_scaleBull__OFFSET: usize = 11096;
pub const CUnitDatabase__m_blend__OFFSET: usize = 11112;
pub const CUnitDatabase__m_selZ__OFFSET: usize = 11128;
pub const CUnitDatabase__m_maxPitch__OFFSET: usize = 11144;
pub const CUnitDatabase__m_maxRoll__OFFSET: usize = 11160;
pub const CUnitDatabase__m_elevPts__OFFSET: usize = 11176;
pub const CUnitDatabase__m_elevRad__OFFSET: usize = 11192;
pub const CUnitDatabase__m_fogRad__OFFSET: usize = 11208;
pub const CUnitDatabase__m_walk__OFFSET: usize = 11224;
pub const CUnitDatabase__m_run__OFFSET: usize = 11240;
pub const CUnitDatabase__m_armor__OFFSET: usize = 11256;
pub const CUnitDatabase__m_file__OFFSET: usize = 11272;
pub const CUnitDatabase__m_fileVerFlags__OFFSET: usize = 11288;
pub const CUnitDatabase__m_unitSound__OFFSET: usize = 11304;
pub const CUnitDatabase__m_name__OFFSET: usize = 11320;
pub const CUnitDatabase__m_modelScale__OFFSET: usize = 11336;
pub const CUnitDatabase__m_legacyModelScale__OFFSET: usize = 11352;
pub const CUnitDatabase__m_red__OFFSET: usize = 11368;
pub const CUnitDatabase__m_green__OFFSET: usize = 11384;
pub const CUnitDatabase__m_blue__OFFSET: usize = 11400;
pub const CUnitDatabase__m_uberSplat__OFFSET: usize = 11416;
pub const CUnitDatabase__m_unitShadow__OFFSET: usize = 11432;
pub const CUnitDatabase__m_buildingShadow__OFFSET: usize = 11448;
pub const CUnitDatabase__m_shadowW__OFFSET: usize = 11464;
pub const CUnitDatabase__m_shadowH__OFFSET: usize = 11480;
pub const CUnitDatabase__m_shadowX__OFFSET: usize = 11496;
pub const CUnitDatabase__m_shadowY__OFFSET: usize = 11512;
pub const CUnitDatabase__m_shadowOnWater__OFFSET: usize = 11528;
pub const CUnitDatabase__m_selCircOnWater__OFFSET: usize = 11544;
pub const CUnitDatabase__m_occH__OFFSET: usize = 11560;
pub const CUnitDatabase__m_special__OFFSET: usize = 11576;
pub const CUnitDatabase__m_hostilePal__OFFSET: usize = 11592;
pub const CUnitDatabase__m_nbmmIcon__OFFSET: usize = 11608;
pub const CUnitDatabase__m_hideHeroBar__OFFSET: usize = 11624;
pub const CUnitDatabase__m_hideHeroMinimap__OFFSET: usize = 11640;
pub const CUnitDatabase__m_hideHeroDeathMsg__OFFSET: usize = 11656;
pub const CUnitDatabase__m_hideOnMinimap__OFFSET: usize = 11672;
pub const CUnitDatabase__m_unitWeapID__OFFSET: usize = 11688;
pub const CUnitDatabase__m_weapsOn__OFFSET: usize = 11704;
pub const CUnitDatabase__m_acquire__OFFSET: usize = 11720;
pub const CUnitDatabase__m_minRange__OFFSET: usize = 11736;
pub const CUnitDatabase__m_castpt__OFFSET: usize = 11752;
pub const CUnitDatabase__m_castbsw__OFFSET: usize = 11768;
pub const CUnitDatabase__m_launchX__OFFSET: usize = 11784;
pub const CUnitDatabase__m_launchY__OFFSET: usize = 11800;
pub const CUnitDatabase__m_launchZ__OFFSET: usize = 11816;
pub const CUnitDatabase__m_launchSwimZ__OFFSET: usize = 11832;
pub const CUnitDatabase__m_impactZ__OFFSET: usize = 11848;
pub const CUnitDatabase__m_impactSwimZ__OFFSET: usize = 11864;
pub const CUnitDatabase__m_weapType__OFFSET: usize = 11880;
pub const CUnitDatabase__m_weapTp__OFFSET: usize = 11912;
pub const CUnitDatabase__m_targs__OFFSET: usize = 11944;
pub const CUnitDatabase__m_splashTargs__OFFSET: usize = 11976;
pub const CUnitDatabase__m_atkType__OFFSET: usize = 12008;
pub const CUnitDatabase__m_rangeN__OFFSET: usize = 12040;
pub const CUnitDatabase__m_cool__OFFSET: usize = 12072;
pub const CUnitDatabase__m_dmgpt__OFFSET: usize = 12104;
pub const CUnitDatabase__m_backSw__OFFSET: usize = 12136;
pub const CUnitDatabase__m_RngBuff__OFFSET: usize = 12168;
pub const CUnitDatabase__m_dice__OFFSET: usize = 12200;
pub const CUnitDatabase__m_sides__OFFSET: usize = 12232;
pub const CUnitDatabase__m_dmgplus__OFFSET: usize = 12264;
pub const CUnitDatabase__m_dmgUp__OFFSET: usize = 12296;
pub const CUnitDatabase__m_Farea__OFFSET: usize = 12328;
pub const CUnitDatabase__m_Harea__OFFSET: usize = 12360;
pub const CUnitDatabase__m_Qarea__OFFSET: usize = 12392;
pub const CUnitDatabase__m_Hfact__OFFSET: usize = 12424;
pub const CUnitDatabase__m_Qfact__OFFSET: usize = 12456;
pub const CUnitDatabase__m_targCount__OFFSET: usize = 12488;
pub const CUnitDatabase__m_damageLoss__OFFSET: usize = 12520;
pub const CUnitDatabase__m_spillDist__OFFSET: usize = 12552;
pub const CUnitDatabase__m_spillRadius__OFFSET: usize = 12584;
pub const CUnitDatabase__m_mindmg__OFFSET: usize = 12616;
pub const CUnitDatabase__m_maxdmg__OFFSET: usize = 12648;
pub const CUnitDatabase__m_showUI__OFFSET: usize = 12680;
pub const CUnitDatabase__m_unitAbilID__OFFSET: usize = 12712;
pub const CUnitDatabase__m_auto__OFFSET: usize = 12728;
pub const CUnitDatabase__m_abilList__OFFSET: usize = 12744;
pub const CUnitDatabase__m_heroAbilList__OFFSET: usize = 12760;

#[repr(C)]
pub struct CUnitCustomData {
    pub _opaque: [u8; 9664],
}
pub const CUnitCustomData__SIZE: usize = 9664;
pub const CUnitCustomData__NAME: &str = "CUnitCustomData";

#[repr(C)]
pub struct CPlayerWar3__3 {
    pub _opaque: [u8; 1960],
}
pub const CPlayerWar3__3__SIZE: usize = 1960;
pub const CPlayerWar3__3__NAME: &str = "CPlayerWar3";
pub const CPlayerWar3__3__s_CPlayerWar3_pool__OFFSET: usize = 0;
pub const CPlayerWar3__3__anticheat_dummy_array_12__OFFSET: usize = 88;
pub const CPlayerWar3__3__m_playerId__OFFSET: usize = 103;
pub const CPlayerWar3__3__m_playerName__OFFSET: usize = 104;
pub const CPlayerWar3__3__m_toonName__OFFSET: usize = 128;
pub const CPlayerWar3__3__m_protobufPlayerData__OFFSET: usize = 152;
pub const CPlayerWar3__3__m_slotNetPlayerId__OFFSET: usize = 336;
pub const CPlayerWar3__3__m_playerNamePrefix__OFFSET: usize = 344;
pub const CPlayerWar3__3__m_pAlliances__OFFSET: usize = 368;
pub const CPlayerWar3__3__m_pSelection__OFFSET: usize = 376;
pub const CPlayerWar3__3__m_playerStates__OFFSET: usize = 384;
pub const CPlayerWar3__3__m_lumberDivertTax__OFFSET: usize = 1016;
pub const CPlayerWar3__3__m_goldDivertTax__OFFSET: usize = 1116;
pub const CPlayerWar3__3__m_highestFood__OFFSET: usize = 1216;
pub const CPlayerWar3__3__m_highestFoodUsed__OFFSET: usize = 1220;
pub const CPlayerWar3__3__anticheat_dummy_array_13__OFFSET: usize = 1224;
pub const CPlayerWar3__3__m_heroKills__OFFSET: usize = 1240;
pub const CPlayerWar3__3__m_itemsObtained__OFFSET: usize = 1244;
pub const CPlayerWar3__3__m_mercsHired__OFFSET: usize = 1248;
pub const CPlayerWar3__3__m_teamColorIndex__OFFSET: usize = 1252;
pub const CPlayerWar3__3__m_controller__OFFSET: usize = 1256;
pub const CPlayerWar3__3__m_startLocIndex__OFFSET: usize = 1260;
pub const CPlayerWar3__3__m_difficulty__OFFSET: usize = 1264;
pub const CPlayerWar3__3__m_slotState__OFFSET: usize = 1268;
pub const CPlayerWar3__3__m_teamIndex__OFFSET: usize = 1272;
pub const CPlayerWar3__3__m_numActions__OFFSET: usize = 1276;
pub const CPlayerWar3__3__m_racePref__OFFSET: usize = 1280;
pub const CPlayerWar3__3__m_race__OFFSET: usize = 1284;
pub const CPlayerWar3__3__m_realTimeAPM__OFFSET: usize = 1288;
pub const CPlayerWar3__3__m_capChangedListener__OFFSET: usize = 1384;
pub const CPlayerWar3__3__m_usedChangedListener__OFFSET: usize = 1392;
pub const CPlayerWar3__3__m_foodUsedUpkeepListener__OFFSET: usize = 1400;
pub const CPlayerWar3__3__m_lumberChangedListener__OFFSET: usize = 1408;
pub const CPlayerWar3__3__m_goldChangedListener__OFFSET: usize = 1416;
pub const CPlayerWar3__3__m_goldUpkeepChanged__OFFSET: usize = 1424;
pub const CPlayerWar3__3__m_lumberUpkeepChanged__OFFSET: usize = 1432;
pub const CPlayerWar3__3__m_handicap__OFFSET: usize = 1440;
pub const CPlayerWar3__3__m_referee__OFFSET: usize = 1512;
pub const CPlayerWar3__3__m_deadHeroAge__OFFSET: usize = 1520;
pub const CPlayerWar3__3__m_deadHeroes__OFFSET: usize = 1544;
pub const CPlayerWar3__3__m_revivableFlags__OFFSET: usize = 1568;
pub const CPlayerWar3__3__anticheat_dummy_array_14__OFFSET: usize = 1572;
pub const CPlayerWar3__3__m_pTechTree__OFFSET: usize = 1584;
pub const CPlayerWar3__3__m_fogMask__OFFSET: usize = 1592;
pub const CPlayerWar3__3__m_ai__OFFSET: usize = 1596;
pub const CPlayerWar3__3__m_bestHeroes__OFFSET: usize = 1608;
pub const CPlayerWar3__3__m_leaderboard__OFFSET: usize = 1632;
pub const CPlayerWar3__3__m_leaveGameTimer__OFFSET: usize = 1648;
pub const CPlayerWar3__3__anticheat_dummy_array_15__OFFSET: usize = 1680;
pub const CPlayerWar3__3__m_doScoreScreen__OFFSET: usize = 1708;
pub const CPlayerWar3__3__m_allowContinue__OFFSET: usize = 1709;
pub const CPlayerWar3__3__m_crippled__OFFSET: usize = 1712;
pub const CPlayerWar3__3__m_gameOverMessage__OFFSET: usize = 1720;
pub const CPlayerWar3__3__m_showInScoreScreen__OFFSET: usize = 1744;
pub const CPlayerWar3__3__m_cachedScore__OFFSET: usize = 1748;
pub const CPlayerWar3__3__m_cachedScoreIsCached__OFFSET: usize = 1752;
pub const CPlayerWar3__3__m_heroCount__OFFSET: usize = 1756;
pub const CPlayerWar3__3__m_bnetPlayerId__OFFSET: usize = 1760;
pub const CPlayerWar3__3__m_heroPickOrder__OFFSET: usize = 1768;
pub const CPlayerWar3__3__m_unitKills__OFFSET: usize = 1792;
pub const CPlayerWar3__3__m_unitsCreated__OFFSET: usize = 1824;
pub const CPlayerWar3__3__m_abilityInfo__OFFSET: usize = 1856;
pub const CPlayerWar3__3__m_upkeepTime__OFFSET: usize = 1888;
pub const CPlayerWar3__3__m_lastUpkeepTime__OFFSET: usize = 1920;
pub const CPlayerWar3__3__m_playerItemUseMap__OFFSET: usize = 1928;

#[repr(C)]
pub struct CGameWar3__3 {
    pub _opaque: [u8; 12872],
}
pub const CGameWar3__3__SIZE: usize = 12872;
pub const CGameWar3__3__NAME: &str = "CGameWar3";
pub const CGameWar3__3__s_campaignArchiveFile__OFFSET: usize = 0;
pub const CGameWar3__3__anticheat_dummy_array_16__OFFSET: usize = 9640;
pub const CGameWar3__3__s_pGameWar3__OFFSET: usize = 0;
pub const CGameWar3__3__anticheat_dummy_array_17__OFFSET: usize = 9695;
pub const CGameWar3__3__m_mapScriptInstance__OFFSET: usize = 9784;
pub const CGameWar3__3__m_mapScriptInstanceId__OFFSET: usize = 9792;
pub const CGameWar3__3__m_mapScript__OFFSET: usize = 9800;
pub const CGameWar3__3__m_mapScriptId__OFFSET: usize = 9808;
pub const CGameWar3__3__m_executions__OFFSET: usize = 9816;
pub const CGameWar3__3__anticheat_dummy_array_18__OFFSET: usize = 9840;
pub const CGameWar3__3__m_gameIdMap__OFFSET: usize = 9864;
pub const CGameWar3__3__m_gameState__OFFSET: usize = 9872;
pub const CGameWar3__3__m_buildId__OFFSET: usize = 9880;
pub const CGameWar3__3__m_localPlayerId__OFFSET: usize = 9884;
pub const CGameWar3__3__m_replayLocalPlayerId__OFFSET: usize = 9886;
pub const CGameWar3__3__m_playerIdBitsToGhost__OFFSET: usize = 9888;
pub const CGameWar3__3__m_playerIdBitsToGhostSet__OFFSET: usize = 9892;
pub const CGameWar3__3__m_pMapSetup__OFFSET: usize = 9896;
pub const CGameWar3__3__m_pFog__OFFSET: usize = 9904;
pub const CGameWar3__3__m_gameName__OFFSET: usize = 9912;
pub const CGameWar3__3__anticheat_dummy_array_19__OFFSET: usize = 9936;
pub const CGameWar3__3__m_maxPlayerCount__OFFSET: usize = 9952;
pub const CGameWar3__3__m_playerCount__OFFSET: usize = 9956;
pub const CGameWar3__3__m_gameMaxPlayerCount__OFFSET: usize = 9960;
pub const CGameWar3__3__m_maxTeamCount__OFFSET: usize = 9964;
pub const CGameWar3__3__m_teams__OFFSET: usize = 9968;
pub const CGameWar3__3__m_players__OFFSET: usize = 9992;
pub const CGameWar3__3__m_startLocations__OFFSET: usize = 10224;
pub const CGameWar3__3__m_slotToPlayerId__OFFSET: usize = 11960;
pub const CGameWar3__3__m_levelToTransitionTo__OFFSET: usize = 12064;
pub const CGameWar3__3__m_modelCine__OFFSET: usize = 12088;
pub const CGameWar3__3__m_cine__OFFSET: usize = 12112;
pub const CGameWar3__3__m_gameOverMessage__OFFSET: usize = 12376;
pub const CGameWar3__3__m_switchingLevels__OFFSET: usize = 12400;
pub const CGameWar3__3__m_doScoreScreen__OFFSET: usize = 12401;
pub const CGameWar3__3__m_isReload__OFFSET: usize = 12402;
pub const CGameWar3__3__anticheat_dummy_array_20__OFFSET: usize = 12403;
pub const CGameWar3__3__m_wasScriptLoad__OFFSET: usize = 12410;
pub const CGameWar3__3__m_wasLoadGame__OFFSET: usize = 12411;
pub const CGameWar3__3__m_isCampaignMission__OFFSET: usize = 12412;
pub const CGameWar3__3__m_campaignIndex__OFFSET: usize = 12416;
pub const CGameWar3__3__m_gameSetup__OFFSET: usize = 12420;
pub const CGameWar3__3__m_usingDarkMask__OFFSET: usize = 12678;
pub const CGameWar3__3__m_unseenQuests__OFFSET: usize = 12679;
pub const CGameWar3__3__m_showAllUnitsCmdBar__OFFSET: usize = 12680;
pub const CGameWar3__3__m_isReplay__OFFSET: usize = 12681;
pub const CGameWar3__3__m_replayFile__OFFSET: usize = 12688;
pub const CGameWar3__3__m_isTournament__OFFSET: usize = 12712;
pub const CGameWar3__3__m_copySaveCount__OFFSET: usize = 12716;
pub const CGameWar3__3__m_finishedInit__OFFSET: usize = 12720;
pub const CGameWar3__3__m_didSaveGameFromScript__OFFSET: usize = 12721;
pub const CGameWar3__3__m_playingGame__OFFSET: usize = 12722;
pub const CGameWar3__3__m_FacialAnimationSetScopeID__OFFSET: usize = 12724;
pub const CGameWar3__3__m_pTriggerStrings__OFFSET: usize = 12728;
pub const CGameWar3__3__anticheat_dummy_array_21__OFFSET: usize = 12736;
pub const CGameWar3__3__m_messageLogContents__OFFSET: usize = 12776;
pub const CGameWar3__3__m_pGameHashTableManager__OFFSET: usize = 12800;
pub const CGameWar3__3__m_gameCacheManager__OFFSET: usize = 12808;
pub const CGameWar3__3__m_gameOverCallback__OFFSET: usize = 12816;
pub const CGameWar3__3__m_maxCheckpointSaves__OFFSET: usize = 12824;
pub const CGameWar3__3__m_pSaveThread__OFFSET: usize = 12832;
pub const CGameWar3__3__m_pCheckpointPopup__OFFSET: usize = 12840;
pub const CGameWar3__3__m_gamePlayStartTime__OFFSET: usize = 12848;
pub const CGameWar3__3__m_latency__OFFSET: usize = 12856;

#[repr(C)]
pub struct CUnitMetaDB {
    pub _opaque: [u8; 696],
}
pub const CUnitMetaDB__SIZE: usize = 696;
pub const CUnitMetaDB__NAME: &str = "CUnitMetaDB";

#[repr(C)]
pub struct CUnitOrderEventData {
    pub _opaque: [u8; 104],
}
pub const CUnitOrderEventData__SIZE: usize = 104;
pub const CUnitOrderEventData__NAME: &str = "CUnitOrderEventData";
pub const CUnitOrderEventData__s_CUnitOrderEventData_pool__OFFSET: usize = 0;
pub const CUnitOrderEventData__m_order__OFFSET: usize = 88;

#[repr(C)]
pub struct CUnitSpellEventDataBase {
    pub _opaque: [u8; 88],
}
pub const CUnitSpellEventDataBase__SIZE: usize = 88;
pub const CUnitSpellEventDataBase__NAME: &str = "CUnitSpellEventDataBase";
pub const CUnitSpellEventDataBase__s_CUnitSpellEventDataBase_pool__OFFSET: usize = 0;
pub const CUnitSpellEventDataBase__m_abil__OFFSET: usize = 72;

#[repr(C)]
pub struct CUnitEventDataBase {
    pub _opaque: [u8; 72],
}
pub const CUnitEventDataBase__SIZE: usize = 72;
pub const CUnitEventDataBase__NAME: &str = "CUnitEventDataBase";
pub const CUnitEventDataBase__s_CUnitEventDataBase_pool__OFFSET: usize = 0;
pub const CUnitEventDataBase__m_unit__OFFSET: usize = 56;

#[repr(C)]
pub struct CUnitStateLimitEventData {
    pub _opaque: [u8; 80],
}
pub const CUnitStateLimitEventData__SIZE: usize = 80;
pub const CUnitStateLimitEventData__NAME: &str = "CUnitStateLimitEventData";
pub const CUnitStateLimitEventData__s_CUnitStateLimitEventData_pool__OFFSET: usize = 0;
pub const CUnitStateLimitEventData__whichState__OFFSET: usize = 72;

#[repr(C)]
pub struct CUnitDamagedEventData {
    pub _opaque: [u8; 120],
}
pub const CUnitDamagedEventData__SIZE: usize = 120;
pub const CUnitDamagedEventData__NAME: &str = "CUnitDamagedEventData";
pub const CUnitDamagedEventData__s_CUnitDamagedEventData_pool__OFFSET: usize = 0;
pub const CUnitDamagedEventData__m_damage__OFFSET: usize = 72;
pub const CUnitDamagedEventData__m_source__OFFSET: usize = 88;
pub const CUnitDamagedEventData__m_attackType__OFFSET: usize = 100;
pub const CUnitDamagedEventData__m_damageType__OFFSET: usize = 104;
pub const CUnitDamagedEventData__m_weaponType__OFFSET: usize = 108;
pub const CUnitDamagedEventData__m_attack__OFFSET: usize = 112;

#[repr(C)]
pub struct CUnitConstructEventDataBase {
    pub _opaque: [u8; 72],
}
pub const CUnitConstructEventDataBase__SIZE: usize = 72;
pub const CUnitConstructEventDataBase__NAME: &str = "CUnitConstructEventDataBase";
pub const CUnitConstructEventDataBase__s_CUnitConstructEventDataBase_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitUpgradeEventDataBase {
    pub _opaque: [u8; 80],
}
pub const CUnitUpgradeEventDataBase__SIZE: usize = 80;
pub const CUnitUpgradeEventDataBase__NAME: &str = "CUnitUpgradeEventDataBase";
pub const CUnitUpgradeEventDataBase__s_CUnitUpgradeEventDataBase_pool__OFFSET: usize = 0;
pub const CUnitUpgradeEventDataBase__m_upgradeTo__OFFSET: usize = 72;

#[repr(C)]
pub struct CUnitResearchEventDataBase {
    pub _opaque: [u8; 80],
}
pub const CUnitResearchEventDataBase__SIZE: usize = 80;
pub const CUnitResearchEventDataBase__NAME: &str = "CUnitResearchEventDataBase";
pub const CUnitResearchEventDataBase__s_CUnitResearchEventDataBase_pool__OFFSET: usize = 0;
pub const CUnitResearchEventDataBase__m_researching__OFFSET: usize = 72;

#[repr(C)]
pub struct CUnitChangeOwnerEventData {
    pub _opaque: [u8; 80],
}
pub const CUnitChangeOwnerEventData__SIZE: usize = 80;
pub const CUnitChangeOwnerEventData__NAME: &str = "CUnitChangeOwnerEventData";
pub const CUnitChangeOwnerEventData__s_CUnitChangeOwnerEventData_pool__OFFSET: usize = 0;
pub const CUnitChangeOwnerEventData__m_oldOwner__OFFSET: usize = 72;

#[repr(C)]
pub struct CUnitDetectedEventData {
    pub _opaque: [u8; 88],
}
pub const CUnitDetectedEventData__SIZE: usize = 88;
pub const CUnitDetectedEventData__NAME: &str = "CUnitDetectedEventData";
pub const CUnitDetectedEventData__s_CUnitDetectedEventData_pool__OFFSET: usize = 0;
pub const CUnitDetectedEventData__m_detectingPlayer__OFFSET: usize = 72;

#[repr(C)]
pub struct CUnitHiddenEventData {
    pub _opaque: [u8; 88],
}
pub const CUnitHiddenEventData__SIZE: usize = 88;
pub const CUnitHiddenEventData__NAME: &str = "CUnitHiddenEventData";
pub const CUnitHiddenEventData__s_CUnitHiddenEventData_pool__OFFSET: usize = 0;
pub const CUnitHiddenEventData__m_playerLosingDetection__OFFSET: usize = 72;

#[repr(C)]
pub struct CUnitEventOtherUnitDataBase {
    pub _opaque: [u8; 88],
}
pub const CUnitEventOtherUnitDataBase__SIZE: usize = 88;
pub const CUnitEventOtherUnitDataBase__NAME: &str = "CUnitEventOtherUnitDataBase";
pub const CUnitEventOtherUnitDataBase__s_CUnitEventOtherUnitDataBase_pool__OFFSET: usize = 0;
pub const CUnitEventOtherUnitDataBase__m_otherUnit__OFFSET: usize = 72;

#[repr(C)]
pub struct CUnitRescueEventData {
    pub _opaque: [u8; 88],
}
pub const CUnitRescueEventData__SIZE: usize = 88;
pub const CUnitRescueEventData__NAME: &str = "CUnitRescueEventData";
pub const CUnitRescueEventData__s_CUnitRescueEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitAcquireEventData {
    pub _opaque: [u8; 88],
}
pub const CUnitAcquireEventData__SIZE: usize = 88;
pub const CUnitAcquireEventData__NAME: &str = "CUnitAcquireEventData";
pub const CUnitAcquireEventData__s_CUnitAcquireEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitAttackedEventData {
    pub _opaque: [u8; 88],
}
pub const CUnitAttackedEventData__SIZE: usize = 88;
pub const CUnitAttackedEventData__NAME: &str = "CUnitAttackedEventData";
pub const CUnitAttackedEventData__s_CUnitAttackedEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitDeathEventData {
    pub _opaque: [u8; 88],
}
pub const CUnitDeathEventData__SIZE: usize = 88;
pub const CUnitDeathEventData__NAME: &str = "CUnitDeathEventData";
pub const CUnitDeathEventData__s_CUnitDeathEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitSummonedEventData {
    pub _opaque: [u8; 88],
}
pub const CUnitSummonedEventData__SIZE: usize = 88;
pub const CUnitSummonedEventData__NAME: &str = "CUnitSummonedEventData";
pub const CUnitSummonedEventData__s_CUnitSummonedEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitTrainEventDataBase {
    pub _opaque: [u8; 88],
}
pub const CUnitTrainEventDataBase__SIZE: usize = 88;
pub const CUnitTrainEventDataBase__NAME: &str = "CUnitTrainEventDataBase";
pub const CUnitTrainEventDataBase__s_CUnitTrainEventDataBase_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitTrainStartEventData {
    pub _opaque: [u8; 96],
}
pub const CUnitTrainStartEventData__SIZE: usize = 96;
pub const CUnitTrainStartEventData__NAME: &str = "CUnitTrainStartEventData";
pub const CUnitTrainStartEventData__s_CUnitTrainStartEventData_pool__OFFSET: usize = 0;
pub const CUnitTrainStartEventData__m_unitId__OFFSET: usize = 88;

#[repr(C)]
pub struct CUnitTrainFinishEventData {
    pub _opaque: [u8; 88],
}
pub const CUnitTrainFinishEventData__SIZE: usize = 88;
pub const CUnitTrainFinishEventData__NAME: &str = "CUnitTrainFinishEventData";
pub const CUnitTrainFinishEventData__s_CUnitTrainFinishEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitSellEventData {
    pub _opaque: [u8; 104],
}
pub const CUnitSellEventData__SIZE: usize = 104;
pub const CUnitSellEventData__NAME: &str = "CUnitSellEventData";
pub const CUnitSellEventData__s_CUnitSellEventData_pool__OFFSET: usize = 0;
pub const CUnitSellEventData__m_buyingUnit__OFFSET: usize = 88;

#[repr(C)]
pub struct CUnitSellItemEventData {
    pub _opaque: [u8; 104],
}
pub const CUnitSellItemEventData__SIZE: usize = 104;
pub const CUnitSellItemEventData__NAME: &str = "CUnitSellItemEventData";
pub const CUnitSellItemEventData__s_CUnitSellItemEventData_pool__OFFSET: usize = 0;
pub const CUnitSellItemEventData__m_soldItem__OFFSET: usize = 88;

#[repr(C)]
pub struct CUnitEventHeroSkillData {
    pub _opaque: [u8; 80],
}
pub const CUnitEventHeroSkillData__SIZE: usize = 80;
pub const CUnitEventHeroSkillData__NAME: &str = "CUnitEventHeroSkillData";
pub const CUnitEventHeroSkillData__s_CUnitEventHeroSkillData_pool__OFFSET: usize = 0;
pub const CUnitEventHeroSkillData__skillLearned__OFFSET: usize = 72;
pub const CUnitEventHeroSkillData__skillLevel__OFFSET: usize = 76;

#[repr(C)]
pub struct CUnitItemEventData {
    pub _opaque: [u8; 88],
}
pub const CUnitItemEventData__SIZE: usize = 88;
pub const CUnitItemEventData__NAME: &str = "CUnitItemEventData";
pub const CUnitItemEventData__s_CUnitItemEventData_pool__OFFSET: usize = 0;
pub const CUnitItemEventData__m_item__OFFSET: usize = 72;

#[repr(C)]
pub struct CUnitItemItemEventData {
    pub _opaque: [u8; 104],
}
pub const CUnitItemItemEventData__SIZE: usize = 104;
pub const CUnitItemItemEventData__NAME: &str = "CUnitItemItemEventData";
pub const CUnitItemItemEventData__s_CUnitItemItemEventData_pool__OFFSET: usize = 0;
pub const CUnitItemItemEventData__m_otherItem__OFFSET: usize = 88;

#[repr(C)]
pub struct CUnitItemPickupEventData {
    pub _opaque: [u8; 112],
}
pub const CUnitItemPickupEventData__SIZE: usize = 112;
pub const CUnitItemPickupEventData__NAME: &str = "CUnitItemPickupEventData";
pub const CUnitItemPickupEventData__s_CUnitItemPickupEventData_pool__OFFSET: usize = 0;
pub const CUnitItemPickupEventData__m_wasAbsorbed__OFFSET: usize = 104;

#[repr(C)]
pub struct CUnitItemStackEventData {
    pub _opaque: [u8; 112],
}
pub const CUnitItemStackEventData__SIZE: usize = 112;
pub const CUnitItemStackEventData__NAME: &str = "CUnitItemStackEventData";
pub const CUnitItemStackEventData__s_CUnitItemStackEventData_pool__OFFSET: usize = 0;
pub const CUnitItemStackEventData__m_previousCharges__OFFSET: usize = 104;

#[repr(C)]
pub struct CUnitLoadEventData {
    pub _opaque: [u8; 88],
}
pub const CUnitLoadEventData__SIZE: usize = 88;
pub const CUnitLoadEventData__NAME: &str = "CUnitLoadEventData";
pub const CUnitLoadEventData__s_CUnitLoadEventData_pool__OFFSET: usize = 0;
pub const CUnitLoadEventData__m_transport__OFFSET: usize = 72;

#[repr(C)]
pub struct CUnitAI {
    pub _opaque: [u8; 152],
}
pub const CUnitAI__SIZE: usize = 152;
pub const CUnitAI__NAME: &str = "CUnitAI";
pub const CUnitAI__s_CUnitAI_pool__OFFSET: usize = 0;
pub const CUnitAI__m_captain__OFFSET: usize = 88;
pub const CUnitAI__anticheat_dummy_array_12__OFFSET: usize = 100;
pub const CUnitAI__m_mode__OFFSET: usize = 108;
pub const CUnitAI__m_town__OFFSET: usize = 112;
pub const CUnitAI__m_flags__OFFSET: usize = 124;
pub const CUnitAI__m_post__OFFSET: usize = 128;
pub const CUnitAI__m_data__OFFSET: usize = 140;
pub const CUnitAI__m_thinker__OFFSET: usize = 144;

#[repr(C)]
pub struct CPlayerWar3__4 {
    pub _opaque: [u8; 1960],
}
pub const CPlayerWar3__4__SIZE: usize = 1960;
pub const CPlayerWar3__4__NAME: &str = "CPlayerWar3";
pub const CPlayerWar3__4__s_CPlayerWar3_pool__OFFSET: usize = 0;
pub const CPlayerWar3__4__anticheat_dummy_array_13__OFFSET: usize = 88;
pub const CPlayerWar3__4__m_playerId__OFFSET: usize = 103;
pub const CPlayerWar3__4__m_playerName__OFFSET: usize = 104;
pub const CPlayerWar3__4__m_toonName__OFFSET: usize = 128;
pub const CPlayerWar3__4__m_protobufPlayerData__OFFSET: usize = 152;
pub const CPlayerWar3__4__m_slotNetPlayerId__OFFSET: usize = 336;
pub const CPlayerWar3__4__m_playerNamePrefix__OFFSET: usize = 344;
pub const CPlayerWar3__4__m_pAlliances__OFFSET: usize = 368;
pub const CPlayerWar3__4__m_pSelection__OFFSET: usize = 376;
pub const CPlayerWar3__4__m_playerStates__OFFSET: usize = 384;
pub const CPlayerWar3__4__m_lumberDivertTax__OFFSET: usize = 1016;
pub const CPlayerWar3__4__m_goldDivertTax__OFFSET: usize = 1116;
pub const CPlayerWar3__4__m_highestFood__OFFSET: usize = 1216;
pub const CPlayerWar3__4__m_highestFoodUsed__OFFSET: usize = 1220;
pub const CPlayerWar3__4__anticheat_dummy_array_14__OFFSET: usize = 1224;
pub const CPlayerWar3__4__m_heroKills__OFFSET: usize = 1240;
pub const CPlayerWar3__4__m_itemsObtained__OFFSET: usize = 1244;
pub const CPlayerWar3__4__m_mercsHired__OFFSET: usize = 1248;
pub const CPlayerWar3__4__m_teamColorIndex__OFFSET: usize = 1252;
pub const CPlayerWar3__4__m_controller__OFFSET: usize = 1256;
pub const CPlayerWar3__4__m_startLocIndex__OFFSET: usize = 1260;
pub const CPlayerWar3__4__m_difficulty__OFFSET: usize = 1264;
pub const CPlayerWar3__4__m_slotState__OFFSET: usize = 1268;
pub const CPlayerWar3__4__m_teamIndex__OFFSET: usize = 1272;
pub const CPlayerWar3__4__m_numActions__OFFSET: usize = 1276;
pub const CPlayerWar3__4__m_racePref__OFFSET: usize = 1280;
pub const CPlayerWar3__4__m_race__OFFSET: usize = 1284;
pub const CPlayerWar3__4__m_realTimeAPM__OFFSET: usize = 1288;
pub const CPlayerWar3__4__m_capChangedListener__OFFSET: usize = 1384;
pub const CPlayerWar3__4__m_usedChangedListener__OFFSET: usize = 1392;
pub const CPlayerWar3__4__m_foodUsedUpkeepListener__OFFSET: usize = 1400;
pub const CPlayerWar3__4__m_lumberChangedListener__OFFSET: usize = 1408;
pub const CPlayerWar3__4__m_goldChangedListener__OFFSET: usize = 1416;
pub const CPlayerWar3__4__m_goldUpkeepChanged__OFFSET: usize = 1424;
pub const CPlayerWar3__4__m_lumberUpkeepChanged__OFFSET: usize = 1432;
pub const CPlayerWar3__4__m_handicap__OFFSET: usize = 1440;
pub const CPlayerWar3__4__m_referee__OFFSET: usize = 1512;
pub const CPlayerWar3__4__m_deadHeroAge__OFFSET: usize = 1520;
pub const CPlayerWar3__4__m_deadHeroes__OFFSET: usize = 1544;
pub const CPlayerWar3__4__m_revivableFlags__OFFSET: usize = 1568;
pub const CPlayerWar3__4__anticheat_dummy_array_15__OFFSET: usize = 1572;
pub const CPlayerWar3__4__m_pTechTree__OFFSET: usize = 1584;
pub const CPlayerWar3__4__m_fogMask__OFFSET: usize = 1592;
pub const CPlayerWar3__4__m_ai__OFFSET: usize = 1596;
pub const CPlayerWar3__4__m_bestHeroes__OFFSET: usize = 1608;
pub const CPlayerWar3__4__m_leaderboard__OFFSET: usize = 1632;
pub const CPlayerWar3__4__m_leaveGameTimer__OFFSET: usize = 1648;
pub const CPlayerWar3__4__anticheat_dummy_array_16__OFFSET: usize = 1680;
pub const CPlayerWar3__4__m_doScoreScreen__OFFSET: usize = 1708;
pub const CPlayerWar3__4__m_allowContinue__OFFSET: usize = 1709;
pub const CPlayerWar3__4__m_crippled__OFFSET: usize = 1712;
pub const CPlayerWar3__4__m_gameOverMessage__OFFSET: usize = 1720;
pub const CPlayerWar3__4__m_showInScoreScreen__OFFSET: usize = 1744;
pub const CPlayerWar3__4__m_cachedScore__OFFSET: usize = 1748;
pub const CPlayerWar3__4__m_cachedScoreIsCached__OFFSET: usize = 1752;
pub const CPlayerWar3__4__m_heroCount__OFFSET: usize = 1756;
pub const CPlayerWar3__4__m_bnetPlayerId__OFFSET: usize = 1760;
pub const CPlayerWar3__4__m_heroPickOrder__OFFSET: usize = 1768;
pub const CPlayerWar3__4__m_unitKills__OFFSET: usize = 1792;
pub const CPlayerWar3__4__m_unitsCreated__OFFSET: usize = 1824;
pub const CPlayerWar3__4__m_abilityInfo__OFFSET: usize = 1856;
pub const CPlayerWar3__4__m_upkeepTime__OFFSET: usize = 1888;
pub const CPlayerWar3__4__m_lastUpkeepTime__OFFSET: usize = 1920;
pub const CPlayerWar3__4__m_playerItemUseMap__OFFSET: usize = 1928;

#[repr(C)]
pub struct CGameWar3__4 {
    pub _opaque: [u8; 12872],
}
pub const CGameWar3__4__SIZE: usize = 12872;
pub const CGameWar3__4__NAME: &str = "CGameWar3";
pub const CGameWar3__4__s_campaignArchiveFile__OFFSET: usize = 0;
pub const CGameWar3__4__anticheat_dummy_array_17__OFFSET: usize = 9640;
pub const CGameWar3__4__s_pGameWar3__OFFSET: usize = 0;
pub const CGameWar3__4__anticheat_dummy_array_18__OFFSET: usize = 9695;
pub const CGameWar3__4__m_mapScriptInstance__OFFSET: usize = 9784;
pub const CGameWar3__4__m_mapScriptInstanceId__OFFSET: usize = 9792;
pub const CGameWar3__4__m_mapScript__OFFSET: usize = 9800;
pub const CGameWar3__4__m_mapScriptId__OFFSET: usize = 9808;
pub const CGameWar3__4__m_executions__OFFSET: usize = 9816;
pub const CGameWar3__4__anticheat_dummy_array_19__OFFSET: usize = 9840;
pub const CGameWar3__4__m_gameIdMap__OFFSET: usize = 9864;
pub const CGameWar3__4__m_gameState__OFFSET: usize = 9872;
pub const CGameWar3__4__m_buildId__OFFSET: usize = 9880;
pub const CGameWar3__4__m_localPlayerId__OFFSET: usize = 9884;
pub const CGameWar3__4__m_replayLocalPlayerId__OFFSET: usize = 9886;
pub const CGameWar3__4__m_playerIdBitsToGhost__OFFSET: usize = 9888;
pub const CGameWar3__4__m_playerIdBitsToGhostSet__OFFSET: usize = 9892;
pub const CGameWar3__4__m_pMapSetup__OFFSET: usize = 9896;
pub const CGameWar3__4__m_pFog__OFFSET: usize = 9904;
pub const CGameWar3__4__m_gameName__OFFSET: usize = 9912;
pub const CGameWar3__4__anticheat_dummy_array_20__OFFSET: usize = 9936;
pub const CGameWar3__4__m_maxPlayerCount__OFFSET: usize = 9952;
pub const CGameWar3__4__m_playerCount__OFFSET: usize = 9956;
pub const CGameWar3__4__m_gameMaxPlayerCount__OFFSET: usize = 9960;
pub const CGameWar3__4__m_maxTeamCount__OFFSET: usize = 9964;
pub const CGameWar3__4__m_teams__OFFSET: usize = 9968;
pub const CGameWar3__4__m_players__OFFSET: usize = 9992;
pub const CGameWar3__4__m_startLocations__OFFSET: usize = 10224;
pub const CGameWar3__4__m_slotToPlayerId__OFFSET: usize = 11960;
pub const CGameWar3__4__m_levelToTransitionTo__OFFSET: usize = 12064;
pub const CGameWar3__4__m_modelCine__OFFSET: usize = 12088;
pub const CGameWar3__4__m_cine__OFFSET: usize = 12112;
pub const CGameWar3__4__m_gameOverMessage__OFFSET: usize = 12376;
pub const CGameWar3__4__m_switchingLevels__OFFSET: usize = 12400;
pub const CGameWar3__4__m_doScoreScreen__OFFSET: usize = 12401;
pub const CGameWar3__4__m_isReload__OFFSET: usize = 12402;
pub const CGameWar3__4__anticheat_dummy_array_21__OFFSET: usize = 12403;
pub const CGameWar3__4__m_wasScriptLoad__OFFSET: usize = 12410;
pub const CGameWar3__4__m_wasLoadGame__OFFSET: usize = 12411;
pub const CGameWar3__4__m_isCampaignMission__OFFSET: usize = 12412;
pub const CGameWar3__4__m_campaignIndex__OFFSET: usize = 12416;
pub const CGameWar3__4__m_gameSetup__OFFSET: usize = 12420;
pub const CGameWar3__4__m_usingDarkMask__OFFSET: usize = 12678;
pub const CGameWar3__4__m_unseenQuests__OFFSET: usize = 12679;
pub const CGameWar3__4__m_showAllUnitsCmdBar__OFFSET: usize = 12680;
pub const CGameWar3__4__m_isReplay__OFFSET: usize = 12681;
pub const CGameWar3__4__m_replayFile__OFFSET: usize = 12688;
pub const CGameWar3__4__m_isTournament__OFFSET: usize = 12712;
pub const CGameWar3__4__m_copySaveCount__OFFSET: usize = 12716;
pub const CGameWar3__4__m_finishedInit__OFFSET: usize = 12720;
pub const CGameWar3__4__m_didSaveGameFromScript__OFFSET: usize = 12721;
pub const CGameWar3__4__m_playingGame__OFFSET: usize = 12722;
pub const CGameWar3__4__m_FacialAnimationSetScopeID__OFFSET: usize = 12724;
pub const CGameWar3__4__m_pTriggerStrings__OFFSET: usize = 12728;
pub const CGameWar3__4__anticheat_dummy_array_22__OFFSET: usize = 12736;
pub const CGameWar3__4__m_messageLogContents__OFFSET: usize = 12776;
pub const CGameWar3__4__m_pGameHashTableManager__OFFSET: usize = 12800;
pub const CGameWar3__4__m_gameCacheManager__OFFSET: usize = 12808;
pub const CGameWar3__4__m_gameOverCallback__OFFSET: usize = 12816;
pub const CGameWar3__4__m_maxCheckpointSaves__OFFSET: usize = 12824;
pub const CGameWar3__4__m_pSaveThread__OFFSET: usize = 12832;
pub const CGameWar3__4__m_pCheckpointPopup__OFFSET: usize = 12840;
pub const CGameWar3__4__m_gamePlayStartTime__OFFSET: usize = 12848;
pub const CGameWar3__4__m_latency__OFFSET: usize = 12856;

#[repr(C)]
pub struct CGameUI__4 {
    pub _opaque: [u8; 2384],
}
pub const CGameUI__4__SIZE: usize = 2384;
pub const CGameUI__4__NAME: &str = "CGameUI";
pub const CGameUI__4__m_transitionState__OFFSET: usize = 704;
pub const CGameUI__4__m_cachedFonts__OFFSET: usize = 712;
pub const CGameUI__4__NUMBER_OF_PLAYER_ROWS__OFFSET: usize = 0;
pub const CGameUI__4__FIRST_ROW_Y_OFFSET__OFFSET: usize = 0;
pub const CGameUI__4__ROW_SPACING__OFFSET: usize = 0;
pub const CGameUI__4__m_userControl__OFFSET: usize = 784;
pub const CGameUI__4__m_userUI__OFFSET: usize = 785;
pub const CGameUI__4__m_currentInputMode__OFFSET: usize = 792;
pub const CGameUI__4__m_inputModeStack__OFFSET: usize = 800;
pub const CGameUI__4__m_sharedTooltip__OFFSET: usize = 824;
pub const CGameUI__4__m_uberTooltip__OFFSET: usize = 832;
pub const CGameUI__4__m_showUberTips__OFFSET: usize = 840;
pub const CGameUI__4__m_autosaveReplay__OFFSET: usize = 841;
pub const CGameUI__4__m_healthBarsMode__OFFSET: usize = 844;
pub const CGameUI__4__m_forceShowHealthBars__OFFSET: usize = 848;
pub const CGameUI__4__m_minimapSignals__OFFSET: usize = 849;
pub const CGameUI__4__m_useColoredHealthBars__OFFSET: usize = 850;
pub const CGameUI__4__m_useHeroFrames__OFFSET: usize = 851;
pub const CGameUI__4__m_showHeroLevelFrame__OFFSET: usize = 852;
pub const CGameUI__4__m_showNumericCooldown__OFFSET: usize = 853;
pub const CGameUI__4__m_showDenyIcon__OFFSET: usize = 854;
pub const CGameUI__4__m_showGoldmineUnitCounter__OFFSET: usize = 855;
pub const CGameUI__4__m_enableObserverUiV2__OFFSET: usize = 856;
pub const CGameUI__4__m_asyncPlayerData__OFFSET: usize = 860;
pub const CGameUI__4__m_targetMode__OFFSET: usize = 960;
pub const CGameUI__4__m_selectMode__OFFSET: usize = 968;
pub const CGameUI__4__m_dragSelectMode__OFFSET: usize = 976;
pub const CGameUI__4__m_dragScrollMode__OFFSET: usize = 984;
pub const CGameUI__4__m_buildMode__OFFSET: usize = 992;
pub const CGameUI__4__m_signalMode__OFFSET: usize = 1000;
pub const CGameUI__4__m_escapeMenu__OFFSET: usize = 1008;
pub const CGameUI__4__m_gameOverMenu__OFFSET: usize = 1016;
pub const CGameUI__4__m_suspendIndicator__OFFSET: usize = 1024;
pub const CGameUI__4__m_unresponsiveIndicator__OFFSET: usize = 1032;
pub const CGameUI__4__m_allianceMode__OFFSET: usize = 1040;
pub const CGameUI__4__m_chatMode__OFFSET: usize = 1048;
pub const CGameUI__4__m_questMode__OFFSET: usize = 1056;
pub const CGameUI__4__anticheat_dummy_array_12__OFFSET: usize = 1064;
pub const CGameUI__4__m_scriptDialogMode__OFFSET: usize = 1152;
pub const CGameUI__4__m_unresponsiveDelayTimerId__OFFSET: usize = 1160;
pub const CGameUI__4__m_selectionManager__OFFSET: usize = 1168;
pub const CGameUI__4__m_dragScrollManager__OFFSET: usize = 1176;
pub const CGameUI__4__m_camera__OFFSET: usize = 1184;
pub const CGameUI__4__m_paused__OFFSET: usize = 1192;
pub const CGameUI__4__m_uiPaused__OFFSET: usize = 1193;
pub const CGameUI__4__m_scriptPaused__OFFSET: usize = 1194;
pub const CGameUI__4__m_disableUpkeepMsg__OFFSET: usize = 1195;
pub const CGameUI__4__m_upkeepLvlSave__OFFSET: usize = 1196;
pub const CGameUI__4__m_refresh__OFFSET: usize = 1200;
pub const CGameUI__4__m_refreshWaypoints__OFFSET: usize = 1201;
pub const CGameUI__4__m_deferCleanSelUI__OFFSET: usize = 1202;
pub const CGameUI__4__m_refreshInfoBar__OFFSET: usize = 1203;
pub const CGameUI__4__m_refreshCommandBar__OFFSET: usize = 1204;
pub const CGameUI__4__m_refreshHeroBar__OFFSET: usize = 1205;
pub const CGameUI__4__m_popSubMenu__OFFSET: usize = 1206;
pub const CGameUI__4__m_activeValidOrder__OFFSET: usize = 1208;
pub const CGameUI__4__m_cineForceSubtitles__OFFSET: usize = 1212;
pub const CGameUI__4__m_cinematicPanelsHidden__OFFSET: usize = 1213;
pub const CGameUI__4__m_dragSelectEnabled__OFFSET: usize = 1214;
pub const CGameUI__4__m_dragSelectUIEnabled__OFFSET: usize = 1215;
pub const CGameUI__4__m_preSelectEnabled__OFFSET: usize = 1216;
pub const CGameUI__4__m_preSelectUIEnabled__OFFSET: usize = 1217;
pub const CGameUI__4__m_selectEnabled__OFFSET: usize = 1218;
pub const CGameUI__4__m_selectUIEnabled__OFFSET: usize = 1219;
pub const CGameUI__4__m_observerModeEnabled__OFFSET: usize = 1220;
pub const CGameUI__4__m_observerModeV2Active__OFFSET: usize = 1221;
pub const CGameUI__4__m_wideScreenAutoPosition__OFFSET: usize = 1222;
pub const CGameUI__4__m_fadeTimer__OFFSET: usize = 1224;
pub const CGameUI__4__m_cinematicFadeTimer__OFFSET: usize = 1328;
pub const CGameUI__4__anticheat_dummy_array_13__OFFSET: usize = 1432;
pub const CGameUI__4__m_frameParentTimer__OFFSET: usize = 1440;
pub const CGameUI__4__m_simpleFadeTimer__OFFSET: usize = 1544;
pub const CGameUI__4__m_attackNotify__OFFSET: usize = 1552;
pub const CGameUI__4__m_warpNotify__OFFSET: usize = 1576;
pub const CGameUI__4__m_timerDialogs__OFFSET: usize = 1608;
pub const CGameUI__4__m_leaderboard__OFFSET: usize = 1632;
pub const CGameUI__4__anticheat_dummy_array_14__OFFSET: usize = 1640;
pub const CGameUI__4__m_multiboard__OFFSET: usize = 1648;
pub const CGameUI__4__m_suspendMultiboardDisplay__OFFSET: usize = 1656;
pub const CGameUI__4__anticheat_dummy_array_15__OFFSET: usize = 1657;
pub const CGameUI__4__m_scriptDialog__OFFSET: usize = 1728;
pub const CGameUI__4__m_replaySelection__OFFSET: usize = 1736;
pub const CGameUI__4__m_unitStatusIconUIFactory__OFFSET: usize = 1744;
pub const CGameUI__4__m_unitStatusTexts__OFFSET: usize = 1752;
pub const CGameUI__4__m_worldFrame__OFFSET: usize = 1776;
pub const CGameUI__4__m_infoBar__OFFSET: usize = 1784;
pub const CGameUI__4__m_commandBar__OFFSET: usize = 1792;
pub const CGameUI__4__m_miniMap__OFFSET: usize = 1800;
pub const CGameUI__4__m_resourceBar__OFFSET: usize = 1808;
pub const CGameUI__4__m_upperButtonBar__OFFSET: usize = 1816;
pub const CGameUI__4__m_observerButtonBar__OFFSET: usize = 1824;
pub const CGameUI__4__m_playerStatisticsPanel__OFFSET: usize = 1832;
pub const CGameUI__4__m_heroPanel__OFFSET: usize = 1840;
pub const CGameUI__4__m_productionPanel__OFFSET: usize = 1848;
pub const CGameUI__4__m_minimapButtonBar__OFFSET: usize = 1856;
pub const CGameUI__4__m_trackingStub__OFFSET: usize = 1864;
pub const CGameUI__4__m_heroBar__OFFSET: usize = 1872;
pub const CGameUI__4__m_peonBar__OFFSET: usize = 1880;
pub const CGameUI__4__m_messageBox__OFFSET: usize = 1888;
pub const CGameUI__4__m_unitMessageBox__OFFSET: usize = 1896;
pub const CGameUI__4__m_chatMessageBox__OFFSET: usize = 1904;
pub const CGameUI__4__m_topMessageBox__OFFSET: usize = 1912;
pub const CGameUI__4__m_currentPortraitButton__OFFSET: usize = 1920;
pub const CGameUI__4__m_timeOfDayIndicator__OFFSET: usize = 1928;
pub const CGameUI__4__m_chatEditBar__OFFSET: usize = 1936;
pub const CGameUI__4__m_cinematicPanel__OFFSET: usize = 1944;
pub const CGameUI__4__m_lastBuffBar__OFFSET: usize = 1952;
pub const CGameUI__4__m_observerBar__OFFSET: usize = 1960;
pub const CGameUI__4__m_miniMapSignalButton__OFFSET: usize = 1968;
pub const CGameUI__4__m_miniMapTerrainButton__OFFSET: usize = 1976;
pub const CGameUI__4__m_miniMapAllyButton__OFFSET: usize = 1984;
pub const CGameUI__4__m_miniMapCreepButton__OFFSET: usize = 1992;
pub const CGameUI__4__m_formationButton__OFFSET: usize = 2000;
pub const CGameUI__4__m_consoleFrame__OFFSET: usize = 2008;
pub const CGameUI__4__m_simpleConsoleFrame__OFFSET: usize = 2016;
pub const CGameUI__4__m_simpleConsoleBottomBarFrame__OFFSET: usize = 2024;
pub const CGameUI__4__m_simpleConsoleBottomBarFrameOverlay__OFFSET: usize = 2032;
pub const CGameUI__4__m_frameUIParent__OFFSET: usize = 2040;
pub const CGameUI__4__top__OFFSET: usize = 2048;
pub const CGameUI__4__topInGame__OFFSET: usize = 2056;
pub const CGameUI__4__m_simpleUIParent__OFFSET: usize = 2064;
pub const CGameUI__4__m_observerUIParent__OFFSET: usize = 2072;
pub const CGameUI__4__m_observerInfoPanelBackground__OFFSET: usize = 2080;
pub const CGameUI__4__m_observerInfoPanelBackpack__OFFSET: usize = 2088;
pub const CGameUI__4__m_consoleBackdropFrame__OFFSET: usize = 2096;
pub const CGameUI__4__m_minimapTextureBackground__OFFSET: usize = 2104;
pub const CGameUI__4__m_infoPanelTexture__OFFSET: usize = 2112;
pub const CGameUI__4__m_quickSaveKey__OFFSET: usize = 2120;
pub const CGameUI__4__m_quickLoadKey__OFFSET: usize = 2124;
pub const CGameUI__4__m_quickHelpKey__OFFSET: usize = 2128;
pub const CGameUI__4__m_quickOptionsKey__OFFSET: usize = 2132;
pub const CGameUI__4__m_quickQuitKey__OFFSET: usize = 2136;
pub const CGameUI__4__m_minimapSignalKey__OFFSET: usize = 2140;
pub const CGameUI__4__m_minimapTerrainKey__OFFSET: usize = 2144;
pub const CGameUI__4__m_minimapColorsKey__OFFSET: usize = 2148;
pub const CGameUI__4__m_minimapCreepKey__OFFSET: usize = 2152;
pub const CGameUI__4__m_formationToggleKey__OFFSET: usize = 2156;
pub const CGameUI__4__m_observerToggleAllPanelsKey__OFFSET: usize = 2160;
pub const CGameUI__4__m_observerProductionModeKey__OFFSET: usize = 2164;
pub const CGameUI__4__m_observerUnitsModeKey__OFFSET: usize = 2168;
pub const CGameUI__4__m_mouseDownHandler__OFFSET: usize = 2176;
pub const CGameUI__4__m_mouseWheelHandler__OFFSET: usize = 2184;
pub const CGameUI__4__m_mouseUpHandler__OFFSET: usize = 2192;
pub const CGameUI__4__m_mouseMoveHandler__OFFSET: usize = 2200;
pub const CGameUI__4__m_observerToggleablePanels__OFFSET: usize = 2208;
pub const CGameUI__4__m_consoleBackdropHeight__OFFSET: usize = 2240;
pub const CGameUI__4__m_auroraChatHandler__OFFSET: usize = 2248;
pub const CGameUI__4__m_lobbyEventsHandler__OFFSET: usize = 2328;
pub const CGameUI__4__s_indexedCustomHotkeyMappingMaxCount__OFFSET: usize = 0;
pub const CGameUI__4__s_customHotkeyEnabled__OFFSET: usize = 0;
pub const CGameUI__4__s_customHotkeyMappingTable__OFFSET: usize = 0;
pub const CGameUI__4__s_customHotkeyMappingTableNonHero__OFFSET: usize = 0;
pub const CGameUI__4__m_observerUnitToFollow__OFFSET: usize = 2376;

#[repr(C)]
pub struct CWorldFrameWar3__1 {
    pub _opaque: [u8; 3128],
}
pub const CWorldFrameWar3__1__SIZE: usize = 3128;
pub const CWorldFrameWar3__1__NAME: &str = "CWorldFrameWar3";
pub const CWorldFrameWar3__1__m_trackUpdating__OFFSET: usize = 656;
pub const CWorldFrameWar3__1__m_sprites__OFFSET: usize = 664;
pub const CWorldFrameWar3__1__m_targetedSprites__OFFSET: usize = 672;
pub const CWorldFrameWar3__1__m_walkableSprites__OFFSET: usize = 680;
pub const CWorldFrameWar3__1__m_pGame__OFFSET: usize = 688;
pub const CWorldFrameWar3__1__m_pDoHpBarsEnemies__OFFSET: usize = 696;
pub const CWorldFrameWar3__1__m_pDoHpBarsAllies__OFFSET: usize = 697;
pub const CWorldFrameWar3__1__m_pLocalPlayer__OFFSET: usize = 704;
pub const CWorldFrameWar3__1__m_pFog__OFFSET: usize = 712;
pub const CWorldFrameWar3__1__m_cachedCameraPos__OFFSET: usize = 720;
pub const CWorldFrameWar3__1__m_localPlayerId__OFFSET: usize = 732;
pub const CWorldFrameWar3__1__m_pCameraWar3__OFFSET: usize = 736;
pub const CWorldFrameWar3__1__m_pMinimap__OFFSET: usize = 744;
pub const CWorldFrameWar3__1__m_spriteButtons__OFFSET: usize = 752;
pub const CWorldFrameWar3__1__m_terrainButtons__OFFSET: usize = 756;
pub const CWorldFrameWar3__1__m_cursorMode__OFFSET: usize = 760;
pub const CWorldFrameWar3__1__anticheat_dummy_array_16__OFFSET: usize = 764;
pub const CWorldFrameWar3__1__m_cursorModeStack__OFFSET: usize = 808;
pub const CWorldFrameWar3__1__m_cursorModeLocked__OFFSET: usize = 832;
pub const CWorldFrameWar3__1__m_forceCursorUpdate__OFFSET: usize = 833;
pub const CWorldFrameWar3__1__m_cursorArt__OFFSET: usize = 840;
pub const CWorldFrameWar3__1__m_aspectScaledProjection__OFFSET: usize = 864;
pub const CWorldFrameWar3__1__m_fovScale__OFFSET: usize = 880;
pub const CWorldFrameWar3__1__m_frustumBounds__OFFSET: usize = 884;
pub const CWorldFrameWar3__1__m_frustumBoundsInt__OFFSET: usize = 900;
pub const CWorldFrameWar3__1__m_localPlayerIdBit__OFFSET: usize = 916;
pub const CWorldFrameWar3__1__m_playerIdBitsToGhostMasks__OFFSET: usize = 920;
pub const CWorldFrameWar3__1__m_playerIdBitsToInvisMasks__OFFSET: usize = 1020;
pub const CWorldFrameWar3__1__m_localPlayerIsObserver__OFFSET: usize = 1120;
pub const CWorldFrameWar3__1__m_isReplayFogDisabled__OFFSET: usize = 1121;
pub const CWorldFrameWar3__1__m_lastUpdateElapsedSec__OFFSET: usize = 1124;
pub const CWorldFrameWar3__1__m_lastAsyncElapsedSec__OFFSET: usize = 1128;
pub const CWorldFrameWar3__1__m_lastTrackSprite__OFFSET: usize = 1136;
pub const CWorldFrameWar3__1__m_lastTrackAgent__OFFSET: usize = 1144;
pub const CWorldFrameWar3__1__m_renderPlacement__OFFSET: usize = 1152;
pub const CWorldFrameWar3__1__m_buildingPlacementRender__OFFSET: usize = 1153;
pub const CWorldFrameWar3__1__m_pBuildFrame__OFFSET: usize = 1160;
pub const CWorldFrameWar3__1__m_cineFilter__OFFSET: usize = 1168;
pub const CWorldFrameWar3__1__m_targetingSprite__OFFSET: usize = 1696;
pub const CWorldFrameWar3__1__m_renderTargetSprite__OFFSET: usize = 1704;
pub const CWorldFrameWar3__1__m_targetingImage__OFFSET: usize = 1708;
pub const CWorldFrameWar3__1__m_targetingRadius__OFFSET: usize = 1712;
pub const CWorldFrameWar3__1__m_targetingSqMag__OFFSET: usize = 1716;
pub const CWorldFrameWar3__1__m_targetImageColor__OFFSET: usize = 1720;
pub const CWorldFrameWar3__1__m_currentWorldPos__OFFSET: usize = 1724;
pub const CWorldFrameWar3__1__m_suspended__OFFSET: usize = 1736;
pub const CWorldFrameWar3__1__m_noHitTests__OFFSET: usize = 1737;
pub const CWorldFrameWar3__1__m_doOcclusion__OFFSET: usize = 1738;
pub const CWorldFrameWar3__1__m_mouseMoveProcessed__OFFSET: usize = 1739;
pub const CWorldFrameWar3__1__m_doFogOfWar__OFFSET: usize = 1740;
pub const CWorldFrameWar3__1__anticheat_dummy_array_17__OFFSET: usize = 1741;
pub const CWorldFrameWar3__1__m_useDarkMask__OFFSET: usize = 1744;
pub const CWorldFrameWar3__1__m_renderUnits__OFFSET: usize = 1745;
pub const CWorldFrameWar3__1__m_terrainFog__OFFSET: usize = 1752;
pub const CWorldFrameWar3__1__m_terrainLight__OFFSET: usize = 1760;
pub const CWorldFrameWar3__1__m_unitLight__OFFSET: usize = 1768;
pub const CWorldFrameWar3__1__m_suspendedAmbientLight__OFFSET: usize = 1776;
pub const CWorldFrameWar3__1__m_targetLight__OFFSET: usize = 1784;
pub const CWorldFrameWar3__1__m_skyModelPath__OFFSET: usize = 1792;
pub const CWorldFrameWar3__1__m_sky__OFFSET: usize = 1816;
pub const CWorldFrameWar3__1__m_showSky__OFFSET: usize = 1824;
pub const CWorldFrameWar3__1__m_renderSky__OFFSET: usize = 1825;
pub const CWorldFrameWar3__1__m_indicators__OFFSET: usize = 1832;
pub const CWorldFrameWar3__1__m_targetIndicators__OFFSET: usize = 1856;
pub const CWorldFrameWar3__1__m_targetIndicatorIndex__OFFSET: usize = 1880;
pub const CWorldFrameWar3__1__m_enumElapsed__OFFSET: usize = 1884;
pub const CWorldFrameWar3__1__m_asyncElapsed__OFFSET: usize = 1888;
pub const CWorldFrameWar3__1__m_doEnumUpdate__OFFSET: usize = 1892;
pub const CWorldFrameWar3__1__m_doFogEnum__OFFSET: usize = 1893;
pub const CWorldFrameWar3__1__m_trackingElapsed__OFFSET: usize = 1896;
pub const CWorldFrameWar3__1__m_terrainMinZ__OFFSET: usize = 1900;
pub const CWorldFrameWar3__1__m_scaledAnimTime__OFFSET: usize = 1904;
pub const CWorldFrameWar3__1__m_hoursPerDay__OFFSET: usize = 1908;
pub const CWorldFrameWar3__1__m_selCircColorFriend__OFFSET: usize = 1912;
pub const CWorldFrameWar3__1__anticheat_dummy_array_18__OFFSET: usize = 1916;
pub const CWorldFrameWar3__1__m_selCircColorNeutral__OFFSET: usize = 1921;
pub const CWorldFrameWar3__1__m_selCircColorEnemy__OFFSET: usize = 1925;
pub const CWorldFrameWar3__1__m_enableTargetIndicator__OFFSET: usize = 1929;
pub const CWorldFrameWar3__1__m_rallyFlags__OFFSET: usize = 1932;
pub const CWorldFrameWar3__1__m_rallyDst__OFFSET: usize = 1936;
pub const CWorldFrameWar3__1__m_rallyDstReuse__OFFSET: usize = 2708;
pub const CWorldFrameWar3__1__m_rallySrc__OFFSET: usize = 2776;
pub const CWorldFrameWar3__1__m_indicatorTerrain__OFFSET: usize = 2784;
pub const CWorldFrameWar3__1__m_waypointIndicators__OFFSET: usize = 2792;
pub const CWorldFrameWar3__1__m_waypointIndicatorIndex__OFFSET: usize = 2816;
pub const CWorldFrameWar3__1__m_unitUpdateQueue__OFFSET: usize = 2824;
pub const CWorldFrameWar3__1__m_destUpdateQueue__OFFSET: usize = 2848;
pub const CWorldFrameWar3__1__m_itemUpdateQueue__OFFSET: usize = 2872;
pub const CWorldFrameWar3__1__m_captUpdateQueue__OFFSET: usize = 2896;
pub const CWorldFrameWar3__1__m_imagUpdateQueue__OFFSET: usize = 2920;
pub const CWorldFrameWar3__1__m_ghstUpdateQueue__OFFSET: usize = 2944;
pub const CWorldFrameWar3__1__anticheat_dummy_array_19__OFFSET: usize = 2968;
pub const CWorldFrameWar3__1__m_enumeratedUnits__OFFSET: usize = 2976;
pub const CWorldFrameWar3__1__m_enumeratedItems__OFFSET: usize = 3000;
pub const CWorldFrameWar3__1__m_visibleUnits__OFFSET: usize = 3024;
pub const CWorldFrameWar3__1__m_visibleSelectables__OFFSET: usize = 3048;
pub const CWorldFrameWar3__1__m_fogUpdateList__OFFSET: usize = 3072;
pub const CWorldFrameWar3__1__m_ghostImages__OFFSET: usize = 3096;
pub const CWorldFrameWar3__1__m_lastRenderLight__OFFSET: usize = 3120;
pub const CWorldFrameWar3__1__m_lastRenderFog__OFFSET: usize = 3124;
pub const CWorldFrameWar3__1__enableDSQDenialMessage__OFFSET: usize = 0;

#[repr(C)]
pub struct CGameUI__5 {
    pub _opaque: [u8; 2384],
}
pub const CGameUI__5__SIZE: usize = 2384;
pub const CGameUI__5__NAME: &str = "CGameUI";
pub const CGameUI__5__m_transitionState__OFFSET: usize = 704;
pub const CGameUI__5__m_cachedFonts__OFFSET: usize = 712;
pub const CGameUI__5__NUMBER_OF_PLAYER_ROWS__OFFSET: usize = 0;
pub const CGameUI__5__FIRST_ROW_Y_OFFSET__OFFSET: usize = 0;
pub const CGameUI__5__ROW_SPACING__OFFSET: usize = 0;
pub const CGameUI__5__m_userControl__OFFSET: usize = 784;
pub const CGameUI__5__m_userUI__OFFSET: usize = 785;
pub const CGameUI__5__m_currentInputMode__OFFSET: usize = 792;
pub const CGameUI__5__m_inputModeStack__OFFSET: usize = 800;
pub const CGameUI__5__m_sharedTooltip__OFFSET: usize = 824;
pub const CGameUI__5__m_uberTooltip__OFFSET: usize = 832;
pub const CGameUI__5__m_showUberTips__OFFSET: usize = 840;
pub const CGameUI__5__m_autosaveReplay__OFFSET: usize = 841;
pub const CGameUI__5__m_healthBarsMode__OFFSET: usize = 844;
pub const CGameUI__5__m_forceShowHealthBars__OFFSET: usize = 848;
pub const CGameUI__5__m_minimapSignals__OFFSET: usize = 849;
pub const CGameUI__5__m_useColoredHealthBars__OFFSET: usize = 850;
pub const CGameUI__5__m_useHeroFrames__OFFSET: usize = 851;
pub const CGameUI__5__m_showHeroLevelFrame__OFFSET: usize = 852;
pub const CGameUI__5__m_showNumericCooldown__OFFSET: usize = 853;
pub const CGameUI__5__m_showDenyIcon__OFFSET: usize = 854;
pub const CGameUI__5__m_showGoldmineUnitCounter__OFFSET: usize = 855;
pub const CGameUI__5__m_enableObserverUiV2__OFFSET: usize = 856;
pub const CGameUI__5__m_asyncPlayerData__OFFSET: usize = 860;
pub const CGameUI__5__m_targetMode__OFFSET: usize = 960;
pub const CGameUI__5__m_selectMode__OFFSET: usize = 968;
pub const CGameUI__5__m_dragSelectMode__OFFSET: usize = 976;
pub const CGameUI__5__m_dragScrollMode__OFFSET: usize = 984;
pub const CGameUI__5__m_buildMode__OFFSET: usize = 992;
pub const CGameUI__5__m_signalMode__OFFSET: usize = 1000;
pub const CGameUI__5__m_escapeMenu__OFFSET: usize = 1008;
pub const CGameUI__5__m_gameOverMenu__OFFSET: usize = 1016;
pub const CGameUI__5__m_suspendIndicator__OFFSET: usize = 1024;
pub const CGameUI__5__m_unresponsiveIndicator__OFFSET: usize = 1032;
pub const CGameUI__5__m_allianceMode__OFFSET: usize = 1040;
pub const CGameUI__5__m_chatMode__OFFSET: usize = 1048;
pub const CGameUI__5__m_questMode__OFFSET: usize = 1056;
pub const CGameUI__5__anticheat_dummy_array_23__OFFSET: usize = 1064;
pub const CGameUI__5__m_scriptDialogMode__OFFSET: usize = 1152;
pub const CGameUI__5__m_unresponsiveDelayTimerId__OFFSET: usize = 1160;
pub const CGameUI__5__m_selectionManager__OFFSET: usize = 1168;
pub const CGameUI__5__m_dragScrollManager__OFFSET: usize = 1176;
pub const CGameUI__5__m_camera__OFFSET: usize = 1184;
pub const CGameUI__5__m_paused__OFFSET: usize = 1192;
pub const CGameUI__5__m_uiPaused__OFFSET: usize = 1193;
pub const CGameUI__5__m_scriptPaused__OFFSET: usize = 1194;
pub const CGameUI__5__m_disableUpkeepMsg__OFFSET: usize = 1195;
pub const CGameUI__5__m_upkeepLvlSave__OFFSET: usize = 1196;
pub const CGameUI__5__m_refresh__OFFSET: usize = 1200;
pub const CGameUI__5__m_refreshWaypoints__OFFSET: usize = 1201;
pub const CGameUI__5__m_deferCleanSelUI__OFFSET: usize = 1202;
pub const CGameUI__5__m_refreshInfoBar__OFFSET: usize = 1203;
pub const CGameUI__5__m_refreshCommandBar__OFFSET: usize = 1204;
pub const CGameUI__5__m_refreshHeroBar__OFFSET: usize = 1205;
pub const CGameUI__5__m_popSubMenu__OFFSET: usize = 1206;
pub const CGameUI__5__m_activeValidOrder__OFFSET: usize = 1208;
pub const CGameUI__5__m_cineForceSubtitles__OFFSET: usize = 1212;
pub const CGameUI__5__m_cinematicPanelsHidden__OFFSET: usize = 1213;
pub const CGameUI__5__m_dragSelectEnabled__OFFSET: usize = 1214;
pub const CGameUI__5__m_dragSelectUIEnabled__OFFSET: usize = 1215;
pub const CGameUI__5__m_preSelectEnabled__OFFSET: usize = 1216;
pub const CGameUI__5__m_preSelectUIEnabled__OFFSET: usize = 1217;
pub const CGameUI__5__m_selectEnabled__OFFSET: usize = 1218;
pub const CGameUI__5__m_selectUIEnabled__OFFSET: usize = 1219;
pub const CGameUI__5__m_observerModeEnabled__OFFSET: usize = 1220;
pub const CGameUI__5__m_observerModeV2Active__OFFSET: usize = 1221;
pub const CGameUI__5__m_wideScreenAutoPosition__OFFSET: usize = 1222;
pub const CGameUI__5__m_fadeTimer__OFFSET: usize = 1224;
pub const CGameUI__5__m_cinematicFadeTimer__OFFSET: usize = 1328;
pub const CGameUI__5__anticheat_dummy_array_24__OFFSET: usize = 1432;
pub const CGameUI__5__m_frameParentTimer__OFFSET: usize = 1440;
pub const CGameUI__5__m_simpleFadeTimer__OFFSET: usize = 1544;
pub const CGameUI__5__m_attackNotify__OFFSET: usize = 1552;
pub const CGameUI__5__m_warpNotify__OFFSET: usize = 1576;
pub const CGameUI__5__m_timerDialogs__OFFSET: usize = 1608;
pub const CGameUI__5__m_leaderboard__OFFSET: usize = 1632;
pub const CGameUI__5__anticheat_dummy_array_25__OFFSET: usize = 1640;
pub const CGameUI__5__m_multiboard__OFFSET: usize = 1648;
pub const CGameUI__5__m_suspendMultiboardDisplay__OFFSET: usize = 1656;
pub const CGameUI__5__anticheat_dummy_array_26__OFFSET: usize = 1657;
pub const CGameUI__5__m_scriptDialog__OFFSET: usize = 1728;
pub const CGameUI__5__m_replaySelection__OFFSET: usize = 1736;
pub const CGameUI__5__m_unitStatusIconUIFactory__OFFSET: usize = 1744;
pub const CGameUI__5__m_unitStatusTexts__OFFSET: usize = 1752;
pub const CGameUI__5__m_worldFrame__OFFSET: usize = 1776;
pub const CGameUI__5__m_infoBar__OFFSET: usize = 1784;
pub const CGameUI__5__m_commandBar__OFFSET: usize = 1792;
pub const CGameUI__5__m_miniMap__OFFSET: usize = 1800;
pub const CGameUI__5__m_resourceBar__OFFSET: usize = 1808;
pub const CGameUI__5__m_upperButtonBar__OFFSET: usize = 1816;
pub const CGameUI__5__m_observerButtonBar__OFFSET: usize = 1824;
pub const CGameUI__5__m_playerStatisticsPanel__OFFSET: usize = 1832;
pub const CGameUI__5__m_heroPanel__OFFSET: usize = 1840;
pub const CGameUI__5__m_productionPanel__OFFSET: usize = 1848;
pub const CGameUI__5__m_minimapButtonBar__OFFSET: usize = 1856;
pub const CGameUI__5__m_trackingStub__OFFSET: usize = 1864;
pub const CGameUI__5__m_heroBar__OFFSET: usize = 1872;
pub const CGameUI__5__m_peonBar__OFFSET: usize = 1880;
pub const CGameUI__5__m_messageBox__OFFSET: usize = 1888;
pub const CGameUI__5__m_unitMessageBox__OFFSET: usize = 1896;
pub const CGameUI__5__m_chatMessageBox__OFFSET: usize = 1904;
pub const CGameUI__5__m_topMessageBox__OFFSET: usize = 1912;
pub const CGameUI__5__m_currentPortraitButton__OFFSET: usize = 1920;
pub const CGameUI__5__m_timeOfDayIndicator__OFFSET: usize = 1928;
pub const CGameUI__5__m_chatEditBar__OFFSET: usize = 1936;
pub const CGameUI__5__m_cinematicPanel__OFFSET: usize = 1944;
pub const CGameUI__5__m_lastBuffBar__OFFSET: usize = 1952;
pub const CGameUI__5__m_observerBar__OFFSET: usize = 1960;
pub const CGameUI__5__m_miniMapSignalButton__OFFSET: usize = 1968;
pub const CGameUI__5__m_miniMapTerrainButton__OFFSET: usize = 1976;
pub const CGameUI__5__m_miniMapAllyButton__OFFSET: usize = 1984;
pub const CGameUI__5__m_miniMapCreepButton__OFFSET: usize = 1992;
pub const CGameUI__5__m_formationButton__OFFSET: usize = 2000;
pub const CGameUI__5__m_consoleFrame__OFFSET: usize = 2008;
pub const CGameUI__5__m_simpleConsoleFrame__OFFSET: usize = 2016;
pub const CGameUI__5__m_simpleConsoleBottomBarFrame__OFFSET: usize = 2024;
pub const CGameUI__5__m_simpleConsoleBottomBarFrameOverlay__OFFSET: usize = 2032;
pub const CGameUI__5__m_frameUIParent__OFFSET: usize = 2040;
pub const CGameUI__5__top__OFFSET: usize = 2048;
pub const CGameUI__5__topInGame__OFFSET: usize = 2056;
pub const CGameUI__5__m_simpleUIParent__OFFSET: usize = 2064;
pub const CGameUI__5__m_observerUIParent__OFFSET: usize = 2072;
pub const CGameUI__5__m_observerInfoPanelBackground__OFFSET: usize = 2080;
pub const CGameUI__5__m_observerInfoPanelBackpack__OFFSET: usize = 2088;
pub const CGameUI__5__m_consoleBackdropFrame__OFFSET: usize = 2096;
pub const CGameUI__5__m_minimapTextureBackground__OFFSET: usize = 2104;
pub const CGameUI__5__m_infoPanelTexture__OFFSET: usize = 2112;
pub const CGameUI__5__m_quickSaveKey__OFFSET: usize = 2120;
pub const CGameUI__5__m_quickLoadKey__OFFSET: usize = 2124;
pub const CGameUI__5__m_quickHelpKey__OFFSET: usize = 2128;
pub const CGameUI__5__m_quickOptionsKey__OFFSET: usize = 2132;
pub const CGameUI__5__m_quickQuitKey__OFFSET: usize = 2136;
pub const CGameUI__5__m_minimapSignalKey__OFFSET: usize = 2140;
pub const CGameUI__5__m_minimapTerrainKey__OFFSET: usize = 2144;
pub const CGameUI__5__m_minimapColorsKey__OFFSET: usize = 2148;
pub const CGameUI__5__m_minimapCreepKey__OFFSET: usize = 2152;
pub const CGameUI__5__m_formationToggleKey__OFFSET: usize = 2156;
pub const CGameUI__5__m_observerToggleAllPanelsKey__OFFSET: usize = 2160;
pub const CGameUI__5__m_observerProductionModeKey__OFFSET: usize = 2164;
pub const CGameUI__5__m_observerUnitsModeKey__OFFSET: usize = 2168;
pub const CGameUI__5__m_mouseDownHandler__OFFSET: usize = 2176;
pub const CGameUI__5__m_mouseWheelHandler__OFFSET: usize = 2184;
pub const CGameUI__5__m_mouseUpHandler__OFFSET: usize = 2192;
pub const CGameUI__5__m_mouseMoveHandler__OFFSET: usize = 2200;
pub const CGameUI__5__m_observerToggleablePanels__OFFSET: usize = 2208;
pub const CGameUI__5__m_consoleBackdropHeight__OFFSET: usize = 2240;
pub const CGameUI__5__m_auroraChatHandler__OFFSET: usize = 2248;
pub const CGameUI__5__m_lobbyEventsHandler__OFFSET: usize = 2328;
pub const CGameUI__5__s_indexedCustomHotkeyMappingMaxCount__OFFSET: usize = 0;
pub const CGameUI__5__s_customHotkeyEnabled__OFFSET: usize = 0;
pub const CGameUI__5__s_customHotkeyMappingTable__OFFSET: usize = 0;
pub const CGameUI__5__s_customHotkeyMappingTableNonHero__OFFSET: usize = 0;
pub const CGameUI__5__m_observerUnitToFollow__OFFSET: usize = 2376;

#[repr(C)]
pub struct CMapLocation {
    pub _opaque: [u8; 96],
}
pub const CMapLocation__SIZE: usize = 96;
pub const CMapLocation__NAME: &str = "CMapLocation";
pub const CMapLocation__s_CMapLocation_pool__OFFSET: usize = 0;
pub const CMapLocation__m_x__OFFSET: usize = 88;
pub const CMapLocation__m_y__OFFSET: usize = 92;

#[repr(C)]
pub struct CUnitAI__1 {
    pub _opaque: [u8; 152],
}
pub const CUnitAI__1__SIZE: usize = 152;
pub const CUnitAI__1__NAME: &str = "CUnitAI";
pub const CUnitAI__1__s_CUnitAI_pool__OFFSET: usize = 0;
pub const CUnitAI__1__m_captain__OFFSET: usize = 88;
pub const CUnitAI__1__anticheat_dummy_array_22__OFFSET: usize = 100;
pub const CUnitAI__1__m_mode__OFFSET: usize = 108;
pub const CUnitAI__1__m_town__OFFSET: usize = 112;
pub const CUnitAI__1__m_flags__OFFSET: usize = 124;
pub const CUnitAI__1__m_post__OFFSET: usize = 128;
pub const CUnitAI__1__m_data__OFFSET: usize = 140;
pub const CUnitAI__1__m_thinker__OFFSET: usize = 144;

#[repr(C)]
pub struct CUnitAI__2 {
    pub _opaque: [u8; 152],
}
pub const CUnitAI__2__SIZE: usize = 152;
pub const CUnitAI__2__NAME: &str = "CUnitAI";
pub const CUnitAI__2__s_CUnitAI_pool__OFFSET: usize = 0;
pub const CUnitAI__2__m_captain__OFFSET: usize = 88;
pub const CUnitAI__2__anticheat_dummy_array_21__OFFSET: usize = 100;
pub const CUnitAI__2__m_mode__OFFSET: usize = 108;
pub const CUnitAI__2__m_town__OFFSET: usize = 112;
pub const CUnitAI__2__m_flags__OFFSET: usize = 124;
pub const CUnitAI__2__m_post__OFFSET: usize = 128;
pub const CUnitAI__2__m_data__OFFSET: usize = 140;
pub const CUnitAI__2__m_thinker__OFFSET: usize = 144;

#[repr(C)]
pub struct CUnitDecayEventData {
    pub _opaque: [u8; 72],
}
pub const CUnitDecayEventData__SIZE: usize = 72;
pub const CUnitDecayEventData__NAME: &str = "CUnitDecayEventData";
pub const CUnitDecayEventData__s_CUnitDecayEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitSelectedEventData {
    pub _opaque: [u8; 72],
}
pub const CUnitSelectedEventData__SIZE: usize = 72;
pub const CUnitSelectedEventData__NAME: &str = "CUnitSelectedEventData";
pub const CUnitSelectedEventData__s_CUnitSelectedEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitDeselectedEventData {
    pub _opaque: [u8; 72],
}
pub const CUnitDeselectedEventData__SIZE: usize = 72;
pub const CUnitDeselectedEventData__NAME: &str = "CUnitDeselectedEventData";
pub const CUnitDeselectedEventData__s_CUnitDeselectedEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitConstructCancelEventData {
    pub _opaque: [u8; 72],
}
pub const CUnitConstructCancelEventData__SIZE: usize = 72;
pub const CUnitConstructCancelEventData__NAME: &str = "CUnitConstructCancelEventData";
pub const CUnitConstructCancelEventData__s_CUnitConstructCancelEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitConstructFinishEventData {
    pub _opaque: [u8; 72],
}
pub const CUnitConstructFinishEventData__SIZE: usize = 72;
pub const CUnitConstructFinishEventData__NAME: &str = "CUnitConstructFinishEventData";
pub const CUnitConstructFinishEventData__s_CUnitConstructFinishEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitUpgradeStartEventData {
    pub _opaque: [u8; 80],
}
pub const CUnitUpgradeStartEventData__SIZE: usize = 80;
pub const CUnitUpgradeStartEventData__NAME: &str = "CUnitUpgradeStartEventData";
pub const CUnitUpgradeStartEventData__s_CUnitUpgradeStartEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitUpgradeCancelEventData {
    pub _opaque: [u8; 80],
}
pub const CUnitUpgradeCancelEventData__SIZE: usize = 80;
pub const CUnitUpgradeCancelEventData__NAME: &str = "CUnitUpgradeCancelEventData";
pub const CUnitUpgradeCancelEventData__s_CUnitUpgradeCancelEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitUpgradeFinishEventData {
    pub _opaque: [u8; 80],
}
pub const CUnitUpgradeFinishEventData__SIZE: usize = 80;
pub const CUnitUpgradeFinishEventData__NAME: &str = "CUnitUpgradeFinishEventData";
pub const CUnitUpgradeFinishEventData__s_CUnitUpgradeFinishEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitResearchStartEventData {
    pub _opaque: [u8; 80],
}
pub const CUnitResearchStartEventData__SIZE: usize = 80;
pub const CUnitResearchStartEventData__NAME: &str = "CUnitResearchStartEventData";
pub const CUnitResearchStartEventData__s_CUnitResearchStartEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitResearchCancelEventData {
    pub _opaque: [u8; 80],
}
pub const CUnitResearchCancelEventData__SIZE: usize = 80;
pub const CUnitResearchCancelEventData__NAME: &str = "CUnitResearchCancelEventData";
pub const CUnitResearchCancelEventData__s_CUnitResearchCancelEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitResearchFinishEventData {
    pub _opaque: [u8; 80],
}
pub const CUnitResearchFinishEventData__SIZE: usize = 80;
pub const CUnitResearchFinishEventData__NAME: &str = "CUnitResearchFinishEventData";
pub const CUnitResearchFinishEventData__s_CUnitResearchFinishEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitInRangeEventData {
    pub _opaque: [u8; 88],
}
pub const CUnitInRangeEventData__SIZE: usize = 88;
pub const CUnitInRangeEventData__NAME: &str = "CUnitInRangeEventData";
pub const CUnitInRangeEventData__s_CUnitInRangeEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitTrainCancelEventData {
    pub _opaque: [u8; 96],
}
pub const CUnitTrainCancelEventData__SIZE: usize = 96;
pub const CUnitTrainCancelEventData__NAME: &str = "CUnitTrainCancelEventData";
pub const CUnitTrainCancelEventData__s_CUnitTrainCancelEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitOrderPointEventData {
    pub _opaque: [u8; 104],
}
pub const CUnitOrderPointEventData__SIZE: usize = 104;
pub const CUnitOrderPointEventData__NAME: &str = "CUnitOrderPointEventData";
pub const CUnitOrderPointEventData__s_CUnitOrderPointEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitOrderTargetEventData {
    pub _opaque: [u8; 104],
}
pub const CUnitOrderTargetEventData__SIZE: usize = 104;
pub const CUnitOrderTargetEventData__NAME: &str = "CUnitOrderTargetEventData";
pub const CUnitOrderTargetEventData__s_CUnitOrderTargetEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerKeyEventReg {
    pub _opaque: [u8; 152],
}
pub const CPlayerKeyEventReg__SIZE: usize = 152;
pub const CPlayerKeyEventReg__NAME: &str = "CPlayerKeyEventReg";
pub const CPlayerKeyEventReg__s_CPlayerKeyEventReg_pool__OFFSET: usize = 0;
pub const CPlayerKeyEventReg__m_key__OFFSET: usize = 136;
pub const CPlayerKeyEventReg__m_metaKey__OFFSET: usize = 140;
pub const CPlayerKeyEventReg__m_keyDown__OFFSET: usize = 144;

#[repr(C)]
pub struct CPlayerChatEventReg {
    pub _opaque: [u8; 168],
}
pub const CPlayerChatEventReg__SIZE: usize = 168;
pub const CPlayerChatEventReg__NAME: &str = "CPlayerChatEventReg";
pub const CPlayerChatEventReg__s_CPlayerChatEventReg_pool__OFFSET: usize = 0;
pub const CPlayerChatEventReg__m_requireExactMatch__OFFSET: usize = 136;
pub const CPlayerChatEventReg__m_messageToDetect__OFFSET: usize = 144;

#[repr(C)]
pub struct CUnit__4 {
    pub _opaque: [u8; 1864],
}
pub const CUnit__4__SIZE: usize = 1864;
pub const CUnit__4__NAME: &str = "CUnit";
pub const CUnit__4__s_CUnit_pool__OFFSET: usize = 0;
pub const CUnit__4__m_unit_paused_count__OFFSET: usize = 440;
pub const CUnit__4__m_PlayingDialogSound__OFFSET: usize = 444;
pub const CUnit__4__m_owner__OFFSET: usize = 448;
pub const CUnit__4__m_flags0__OFFSET: usize = 452;
pub const CUnit__4__m_flags1__OFFSET: usize = 456;
pub const CUnit__4__m_foodAndTechTreeFlags__OFFSET: usize = 460;
pub const CUnit__4__m_minimapFlashTimer__OFFSET: usize = 464;
pub const CUnit__4__m_damage_dealt__OFFSET: usize = 496;
pub const CUnit__4__m_self_damage__OFFSET: usize = 500;
pub const CUnit__4__m_healing_done__OFFSET: usize = 504;
pub const CUnit__4__m_time_stopped__OFFSET: usize = 508;
pub const CUnit__4__m_total_kills__OFFSET: usize = 512;
pub const CUnit__4__m_self_kills__OFFSET: usize = 516;
pub const CUnit__4__m_hero_kills__OFFSET: usize = 520;
pub const CUnit__4__m_building_kills__OFFSET: usize = 524;
pub const CUnit__4__m_override_ability_id_stats_credit__OFFSET: usize = 528;
pub const CUnit__4__m_override_item_id_stats_credit__OFFSET: usize = 532;
pub const CUnit__4__m_unit_stats_override__OFFSET: usize = 536;
pub const CUnit__4__m_caller__OFFSET: usize = 544;
pub const CUnit__4__m_lastAttackNotificationTime__OFFSET: usize = 576;
pub const CUnit__4__m_lastAttackNotificationSoundTime__OFFSET: usize = 580;
pub const CUnit__4__m_life__OFFSET: usize = 584;
pub const CUnit__4__m_death__OFFSET: usize = 608;
pub const CUnit__4__m_total_life_lost__OFFSET: usize = 616;
pub const CUnit__4__m_stored_time_when_last_alive__OFFSET: usize = 620;
pub const CUnit__4__m_time_alive_ms__OFFSET: usize = 624;
pub const CUnit__4__m_number_of_deaths__OFFSET: usize = 628;
pub const CUnit__4__m_life_regen_rate__OFFSET: usize = 632;
pub const CUnit__4__m_life_regen__OFFSET: usize = 648;
pub const CUnit__4__m_life_regen_type__OFFSET: usize = 656;
pub const CUnit__4__m_mana__OFFSET: usize = 664;
pub const CUnit__4__m_ai_mana__OFFSET: usize = 688;
pub const CUnit__4__m_mana_regen_rate__OFFSET: usize = 704;
pub const CUnit__4__m_mana_regen__OFFSET: usize = 720;
pub const CUnit__4__m_total_mana_spent__OFFSET: usize = 728;
pub const CUnit__4__m_defense__OFFSET: usize = 736;
pub const CUnit__4__m_battle_type__OFFSET: usize = 752;
pub const CUnit__4__m_invulnerable_count__OFFSET: usize = 756;
pub const CUnit__4__m_auto_attack_ignore_count__OFFSET: usize = 760;
pub const CUnit__4__m_fogFlash__OFFSET: usize = 764;
pub const CUnit__4__m_sight__OFFSET: usize = 776;
pub const CUnit__4__m_sightRangeCached__OFFSET: usize = 800;
pub const CUnit__4__m_sightMod__OFFSET: usize = 808;
pub const CUnit__4__m_invisibility_count__OFFSET: usize = 816;
pub const CUnit__4__m_invisibility_percent__OFFSET: usize = 824;
pub const CUnit__4__m_invis_modify__OFFSET: usize = 848;
pub const CUnit__4__m_invis_listen__OFFSET: usize = 856;
pub const CUnit__4__m_detectedData__OFFSET: usize = 864;
pub const CUnit__4__m_sharedVisionData__OFFSET: usize = 876;
pub const CUnit__4__m_sharedVisionDataCached__OFFSET: usize = 888;
pub const CUnit__4__m_sharedVisionTransitiveDataCached__OFFSET: usize = 892;
pub const CUnit__4__m_frost_count__OFFSET: usize = 896;
pub const CUnit__4__m_stone_count__OFFSET: usize = 900;
pub const CUnit__4__m_ethereal_count__OFFSET: usize = 904;
pub const CUnit__4__m_magic_immunity_count__OFFSET: usize = 908;
pub const CUnit__4__m_dispel_immunity_count__OFFSET: usize = 912;
pub const CUnit__4__m_fog_radius__OFFSET: usize = 920;
pub const CUnit__4__m_pos__OFFSET: usize = 936;
pub const CUnit__4__m_exp_level__OFFSET: usize = 960;
pub const CUnit__4__MaxNameLength__OFFSET: usize = 0;
pub const CUnit__4__m_unit_proper_name__OFFSET: usize = 964;
pub const CUnit__4__m_pick_order__OFFSET: usize = 1224;
pub const CUnit__4__m_task_stack__OFFSET: usize = 1228;
pub const CUnit__4__m_task_timer__OFFSET: usize = 1240;
pub const CUnit__4__m_action__OFFSET: usize = 1272;
pub const CUnit__4__m_uninterruptable_count__OFFSET: usize = 1276;
pub const CUnit__4__m_order_head__OFFSET: usize = 1280;
pub const CUnit__4__m_order_tail__OFFSET: usize = 1292;
pub const CUnit__4__order_count__OFFSET: usize = 1304;
pub const CUnit__4__m_gold_bounty_dice__OFFSET: usize = 1308;
pub const CUnit__4__m_gold_bounty_sides__OFFSET: usize = 1312;
pub const CUnit__4__m_gold_bounty_plus__OFFSET: usize = 1316;
pub const CUnit__4__m_wood_bounty_dice__OFFSET: usize = 1320;
pub const CUnit__4__m_wood_bounty_sides__OFFSET: usize = 1324;
pub const CUnit__4__m_wood_bounty_plus__OFFSET: usize = 1328;
pub const CUnit__4__m_cast_point__OFFSET: usize = 1336;
pub const CUnit__4__m_cast_back_swing__OFFSET: usize = 1352;
pub const CUnit__4__m_abilities__OFFSET: usize = 1368;
pub const CUnit__4__m_disabled_count__OFFSET: usize = 1380;
pub const CUnit__4__m_disabled_hidden_magic_count__OFFSET: usize = 1384;
pub const CUnit__4__m_silence_count__OFFSET: usize = 1388;
pub const CUnit__4__anticheat_dummy_array_8__OFFSET: usize = 1392;
pub const CUnit__4__m_disabled_magic_count__OFFSET: usize = 1404;
pub const CUnit__4__m_disabled_hidden_count__OFFSET: usize = 1408;
pub const CUnit__4__m_abil_paused_count__OFFSET: usize = 1412;
pub const CUnit__4__m_disabled_hidden_physical_count__OFFSET: usize = 1416;
pub const CUnit__4__m_disabled_physical_count__OFFSET: usize = 1420;
pub const CUnit__4__anticheat_dummy_array_9__OFFSET: usize = 1424;
pub const CUnit__4__m_buff_paused_count__OFFSET: usize = 1432;
pub const CUnit__4__m_abil_inv__OFFSET: usize = 1440;
pub const CUnit__4__m_abil_hero__OFFSET: usize = 1448;
pub const CUnit__4__m_abil_move__OFFSET: usize = 1456;
pub const CUnit__4__anticheat_dummy_array_10__OFFSET: usize = 1464;
pub const CUnit__4__m_abil_attack__OFFSET: usize = 1472;
pub const CUnit__4__m_abil_build__OFFSET: usize = 1480;
pub const CUnit__4__m_base_priority__OFFSET: usize = 1488;
pub const CUnit__4__m_death_time__OFFSET: usize = 1504;
pub const CUnit__4__s_minUnitSpeed__OFFSET: usize = 0;
pub const CUnit__4__s_maxUnitSpeed__OFFSET: usize = 0;
pub const CUnit__4__s_minBldgSpeed__OFFSET: usize = 0;
pub const CUnit__4__s_maxBldgSpeed__OFFSET: usize = 0;
pub const CUnit__4__s_buildingKillsGiveExp__OFFSET: usize = 0;
pub const CUnit__4__s_globalExperience__OFFSET: usize = 0;
pub const CUnit__4__s_maxLevelHeroesDrainExp__OFFSET: usize = 0;
pub const CUnit__4__m_display_height__OFFSET: usize = 1520;
pub const CUnit__4__m_display_height_modifier__OFFSET: usize = 1544;
pub const CUnit__4__anticheat_dummy_array_11__OFFSET: usize = 1552;
pub const CUnit__4__m_last_cluster_birth_tag__OFFSET: usize = 1560;
pub const CUnit__4__m_last_cluster_presence_tag__OFFSET: usize = 1564;
pub const CUnit__4__m_move_type__OFFSET: usize = 1568;
pub const CUnit__4__m_ground_count__OFFSET: usize = 1572;
pub const CUnit__4__m_launch_offset__OFFSET: usize = 1576;
pub const CUnit__4__m_fly_height__OFFSET: usize = 1592;
pub const CUnit__4__m_impact_swim__OFFSET: usize = 1616;
pub const CUnit__4__anticheat_dummy_array_12__OFFSET: usize = 1620;
pub const CUnit__4__m_launch_swim_z__OFFSET: usize = 1624;
pub const CUnit__4__m_impact_offset__OFFSET: usize = 1628;
pub const CUnit__4__m_category__OFFSET: usize = 1632;
pub const CUnit__4__m_target_type_flags__OFFSET: usize = 1636;
pub const CUnit__4__m_cargo_size__OFFSET: usize = 1640;
pub const CUnit__4__m_killedBy__OFFSET: usize = 1644;
pub const CUnit__4__m_shadow__OFFSET: usize = 1656;
pub const CUnit__4__m_shadowWater__OFFSET: usize = 1660;
pub const CUnit__4__anticheat_dummy_array_13__OFFSET: usize = 1664;
pub const CUnit__4__m_shadowAboveWater__OFFSET: usize = 1671;
pub const CUnit__4__m_shadowShow__OFFSET: usize = 1672;
pub const CUnit__4__m_showOccMark__OFFSET: usize = 1673;
pub const CUnit__4__m_allySelectionImage__OFFSET: usize = 1676;
pub const CUnit__4__m_occluded__OFFSET: usize = 1680;
pub const CUnit__4__anticheat_dummy_array_14__OFFSET: usize = 1681;
pub const CUnit__4__m_occlusionMark__OFFSET: usize = 1684;
pub const CUnit__4__m_jassUserData__OFFSET: usize = 1688;
pub const CUnit__4__m_foliageExclusionId__OFFSET: usize = 1692;
pub const CUnit__4__m_uiFlags__OFFSET: usize = 1696;
pub const CUnit__4__m_cachedPos__OFFSET: usize = 1700;
pub const CUnit__4__m_cachedFacing__OFFSET: usize = 1712;
pub const CUnit__4__m_cachedOrientation__OFFSET: usize = 1716;
pub const CUnit__4__m_lookAtTarget__OFFSET: usize = 1752;
pub const CUnit__4__m_lookAtOffset__OFFSET: usize = 1760;
pub const CUnit__4__m_overheadOffset__OFFSET: usize = 1772;
pub const CUnit__4__m_occupUI__OFFSET: usize = 1776;
pub const CUnit__4__m_unitStatusIconUI__OFFSET: usize = 1784;
pub const CUnit__4__m_unitStatusTextUI__OFFSET: usize = 1792;
pub const CUnit__4__m_uberSplatId__OFFSET: usize = 1800;
pub const CUnit__4__m_vertexColor__OFFSET: usize = 1804;
pub const CUnit__4__m_anim_size__OFFSET: usize = 1808;
pub const CUnit__4__m_anims__OFFSET: usize = 1812;
pub const CUnit__4__m_forcedAnimScale__OFFSET: usize = 1840;
pub const CUnit__4__m_walk_anim_speed__OFFSET: usize = 1844;
pub const CUnit__4__m_run_anim_speed__OFFSET: usize = 1848;
pub const CUnit__4__m_unit_ai__OFFSET: usize = 1852;

#[repr(C)]
pub struct CGameUI__6 {
    pub _opaque: [u8; 2384],
}
pub const CGameUI__6__SIZE: usize = 2384;
pub const CGameUI__6__NAME: &str = "CGameUI";
pub const CGameUI__6__m_transitionState__OFFSET: usize = 704;
pub const CGameUI__6__m_cachedFonts__OFFSET: usize = 712;
pub const CGameUI__6__NUMBER_OF_PLAYER_ROWS__OFFSET: usize = 0;
pub const CGameUI__6__FIRST_ROW_Y_OFFSET__OFFSET: usize = 0;
pub const CGameUI__6__ROW_SPACING__OFFSET: usize = 0;
pub const CGameUI__6__m_userControl__OFFSET: usize = 784;
pub const CGameUI__6__m_userUI__OFFSET: usize = 785;
pub const CGameUI__6__m_currentInputMode__OFFSET: usize = 792;
pub const CGameUI__6__m_inputModeStack__OFFSET: usize = 800;
pub const CGameUI__6__m_sharedTooltip__OFFSET: usize = 824;
pub const CGameUI__6__m_uberTooltip__OFFSET: usize = 832;
pub const CGameUI__6__m_showUberTips__OFFSET: usize = 840;
pub const CGameUI__6__m_autosaveReplay__OFFSET: usize = 841;
pub const CGameUI__6__m_healthBarsMode__OFFSET: usize = 844;
pub const CGameUI__6__m_forceShowHealthBars__OFFSET: usize = 848;
pub const CGameUI__6__m_minimapSignals__OFFSET: usize = 849;
pub const CGameUI__6__m_useColoredHealthBars__OFFSET: usize = 850;
pub const CGameUI__6__m_useHeroFrames__OFFSET: usize = 851;
pub const CGameUI__6__m_showHeroLevelFrame__OFFSET: usize = 852;
pub const CGameUI__6__m_showNumericCooldown__OFFSET: usize = 853;
pub const CGameUI__6__m_showDenyIcon__OFFSET: usize = 854;
pub const CGameUI__6__m_showGoldmineUnitCounter__OFFSET: usize = 855;
pub const CGameUI__6__m_enableObserverUiV2__OFFSET: usize = 856;
pub const CGameUI__6__m_asyncPlayerData__OFFSET: usize = 860;
pub const CGameUI__6__m_targetMode__OFFSET: usize = 960;
pub const CGameUI__6__m_selectMode__OFFSET: usize = 968;
pub const CGameUI__6__m_dragSelectMode__OFFSET: usize = 976;
pub const CGameUI__6__m_dragScrollMode__OFFSET: usize = 984;
pub const CGameUI__6__m_buildMode__OFFSET: usize = 992;
pub const CGameUI__6__m_signalMode__OFFSET: usize = 1000;
pub const CGameUI__6__m_escapeMenu__OFFSET: usize = 1008;
pub const CGameUI__6__m_gameOverMenu__OFFSET: usize = 1016;
pub const CGameUI__6__m_suspendIndicator__OFFSET: usize = 1024;
pub const CGameUI__6__m_unresponsiveIndicator__OFFSET: usize = 1032;
pub const CGameUI__6__m_allianceMode__OFFSET: usize = 1040;
pub const CGameUI__6__m_chatMode__OFFSET: usize = 1048;
pub const CGameUI__6__m_questMode__OFFSET: usize = 1056;
pub const CGameUI__6__anticheat_dummy_array_15__OFFSET: usize = 1064;
pub const CGameUI__6__m_scriptDialogMode__OFFSET: usize = 1152;
pub const CGameUI__6__m_unresponsiveDelayTimerId__OFFSET: usize = 1160;
pub const CGameUI__6__m_selectionManager__OFFSET: usize = 1168;
pub const CGameUI__6__m_dragScrollManager__OFFSET: usize = 1176;
pub const CGameUI__6__m_camera__OFFSET: usize = 1184;
pub const CGameUI__6__m_paused__OFFSET: usize = 1192;
pub const CGameUI__6__m_uiPaused__OFFSET: usize = 1193;
pub const CGameUI__6__m_scriptPaused__OFFSET: usize = 1194;
pub const CGameUI__6__m_disableUpkeepMsg__OFFSET: usize = 1195;
pub const CGameUI__6__m_upkeepLvlSave__OFFSET: usize = 1196;
pub const CGameUI__6__m_refresh__OFFSET: usize = 1200;
pub const CGameUI__6__m_refreshWaypoints__OFFSET: usize = 1201;
pub const CGameUI__6__m_deferCleanSelUI__OFFSET: usize = 1202;
pub const CGameUI__6__m_refreshInfoBar__OFFSET: usize = 1203;
pub const CGameUI__6__m_refreshCommandBar__OFFSET: usize = 1204;
pub const CGameUI__6__m_refreshHeroBar__OFFSET: usize = 1205;
pub const CGameUI__6__m_popSubMenu__OFFSET: usize = 1206;
pub const CGameUI__6__m_activeValidOrder__OFFSET: usize = 1208;
pub const CGameUI__6__m_cineForceSubtitles__OFFSET: usize = 1212;
pub const CGameUI__6__m_cinematicPanelsHidden__OFFSET: usize = 1213;
pub const CGameUI__6__m_dragSelectEnabled__OFFSET: usize = 1214;
pub const CGameUI__6__m_dragSelectUIEnabled__OFFSET: usize = 1215;
pub const CGameUI__6__m_preSelectEnabled__OFFSET: usize = 1216;
pub const CGameUI__6__m_preSelectUIEnabled__OFFSET: usize = 1217;
pub const CGameUI__6__m_selectEnabled__OFFSET: usize = 1218;
pub const CGameUI__6__m_selectUIEnabled__OFFSET: usize = 1219;
pub const CGameUI__6__m_observerModeEnabled__OFFSET: usize = 1220;
pub const CGameUI__6__m_observerModeV2Active__OFFSET: usize = 1221;
pub const CGameUI__6__m_wideScreenAutoPosition__OFFSET: usize = 1222;
pub const CGameUI__6__m_fadeTimer__OFFSET: usize = 1224;
pub const CGameUI__6__m_cinematicFadeTimer__OFFSET: usize = 1328;
pub const CGameUI__6__anticheat_dummy_array_16__OFFSET: usize = 1432;
pub const CGameUI__6__m_frameParentTimer__OFFSET: usize = 1440;
pub const CGameUI__6__m_simpleFadeTimer__OFFSET: usize = 1544;
pub const CGameUI__6__m_attackNotify__OFFSET: usize = 1552;
pub const CGameUI__6__m_warpNotify__OFFSET: usize = 1576;
pub const CGameUI__6__m_timerDialogs__OFFSET: usize = 1608;
pub const CGameUI__6__m_leaderboard__OFFSET: usize = 1632;
pub const CGameUI__6__anticheat_dummy_array_17__OFFSET: usize = 1640;
pub const CGameUI__6__m_multiboard__OFFSET: usize = 1648;
pub const CGameUI__6__m_suspendMultiboardDisplay__OFFSET: usize = 1656;
pub const CGameUI__6__anticheat_dummy_array_18__OFFSET: usize = 1657;
pub const CGameUI__6__m_scriptDialog__OFFSET: usize = 1728;
pub const CGameUI__6__m_replaySelection__OFFSET: usize = 1736;
pub const CGameUI__6__m_unitStatusIconUIFactory__OFFSET: usize = 1744;
pub const CGameUI__6__m_unitStatusTexts__OFFSET: usize = 1752;
pub const CGameUI__6__m_worldFrame__OFFSET: usize = 1776;
pub const CGameUI__6__m_infoBar__OFFSET: usize = 1784;
pub const CGameUI__6__m_commandBar__OFFSET: usize = 1792;
pub const CGameUI__6__m_miniMap__OFFSET: usize = 1800;
pub const CGameUI__6__m_resourceBar__OFFSET: usize = 1808;
pub const CGameUI__6__m_upperButtonBar__OFFSET: usize = 1816;
pub const CGameUI__6__m_observerButtonBar__OFFSET: usize = 1824;
pub const CGameUI__6__m_playerStatisticsPanel__OFFSET: usize = 1832;
pub const CGameUI__6__m_heroPanel__OFFSET: usize = 1840;
pub const CGameUI__6__m_productionPanel__OFFSET: usize = 1848;
pub const CGameUI__6__m_minimapButtonBar__OFFSET: usize = 1856;
pub const CGameUI__6__m_trackingStub__OFFSET: usize = 1864;
pub const CGameUI__6__m_heroBar__OFFSET: usize = 1872;
pub const CGameUI__6__m_peonBar__OFFSET: usize = 1880;
pub const CGameUI__6__m_messageBox__OFFSET: usize = 1888;
pub const CGameUI__6__m_unitMessageBox__OFFSET: usize = 1896;
pub const CGameUI__6__m_chatMessageBox__OFFSET: usize = 1904;
pub const CGameUI__6__m_topMessageBox__OFFSET: usize = 1912;
pub const CGameUI__6__m_currentPortraitButton__OFFSET: usize = 1920;
pub const CGameUI__6__m_timeOfDayIndicator__OFFSET: usize = 1928;
pub const CGameUI__6__m_chatEditBar__OFFSET: usize = 1936;
pub const CGameUI__6__m_cinematicPanel__OFFSET: usize = 1944;
pub const CGameUI__6__m_lastBuffBar__OFFSET: usize = 1952;
pub const CGameUI__6__m_observerBar__OFFSET: usize = 1960;
pub const CGameUI__6__m_miniMapSignalButton__OFFSET: usize = 1968;
pub const CGameUI__6__m_miniMapTerrainButton__OFFSET: usize = 1976;
pub const CGameUI__6__m_miniMapAllyButton__OFFSET: usize = 1984;
pub const CGameUI__6__m_miniMapCreepButton__OFFSET: usize = 1992;
pub const CGameUI__6__m_formationButton__OFFSET: usize = 2000;
pub const CGameUI__6__m_consoleFrame__OFFSET: usize = 2008;
pub const CGameUI__6__m_simpleConsoleFrame__OFFSET: usize = 2016;
pub const CGameUI__6__m_simpleConsoleBottomBarFrame__OFFSET: usize = 2024;
pub const CGameUI__6__m_simpleConsoleBottomBarFrameOverlay__OFFSET: usize = 2032;
pub const CGameUI__6__m_frameUIParent__OFFSET: usize = 2040;
pub const CGameUI__6__top__OFFSET: usize = 2048;
pub const CGameUI__6__topInGame__OFFSET: usize = 2056;
pub const CGameUI__6__m_simpleUIParent__OFFSET: usize = 2064;
pub const CGameUI__6__m_observerUIParent__OFFSET: usize = 2072;
pub const CGameUI__6__m_observerInfoPanelBackground__OFFSET: usize = 2080;
pub const CGameUI__6__m_observerInfoPanelBackpack__OFFSET: usize = 2088;
pub const CGameUI__6__m_consoleBackdropFrame__OFFSET: usize = 2096;
pub const CGameUI__6__m_minimapTextureBackground__OFFSET: usize = 2104;
pub const CGameUI__6__m_infoPanelTexture__OFFSET: usize = 2112;
pub const CGameUI__6__m_quickSaveKey__OFFSET: usize = 2120;
pub const CGameUI__6__m_quickLoadKey__OFFSET: usize = 2124;
pub const CGameUI__6__m_quickHelpKey__OFFSET: usize = 2128;
pub const CGameUI__6__m_quickOptionsKey__OFFSET: usize = 2132;
pub const CGameUI__6__m_quickQuitKey__OFFSET: usize = 2136;
pub const CGameUI__6__m_minimapSignalKey__OFFSET: usize = 2140;
pub const CGameUI__6__m_minimapTerrainKey__OFFSET: usize = 2144;
pub const CGameUI__6__m_minimapColorsKey__OFFSET: usize = 2148;
pub const CGameUI__6__m_minimapCreepKey__OFFSET: usize = 2152;
pub const CGameUI__6__m_formationToggleKey__OFFSET: usize = 2156;
pub const CGameUI__6__m_observerToggleAllPanelsKey__OFFSET: usize = 2160;
pub const CGameUI__6__m_observerProductionModeKey__OFFSET: usize = 2164;
pub const CGameUI__6__m_observerUnitsModeKey__OFFSET: usize = 2168;
pub const CGameUI__6__m_mouseDownHandler__OFFSET: usize = 2176;
pub const CGameUI__6__m_mouseWheelHandler__OFFSET: usize = 2184;
pub const CGameUI__6__m_mouseUpHandler__OFFSET: usize = 2192;
pub const CGameUI__6__m_mouseMoveHandler__OFFSET: usize = 2200;
pub const CGameUI__6__m_observerToggleablePanels__OFFSET: usize = 2208;
pub const CGameUI__6__m_consoleBackdropHeight__OFFSET: usize = 2240;
pub const CGameUI__6__m_auroraChatHandler__OFFSET: usize = 2248;
pub const CGameUI__6__m_lobbyEventsHandler__OFFSET: usize = 2328;
pub const CGameUI__6__s_indexedCustomHotkeyMappingMaxCount__OFFSET: usize = 0;
pub const CGameUI__6__s_customHotkeyEnabled__OFFSET: usize = 0;
pub const CGameUI__6__s_customHotkeyMappingTable__OFFSET: usize = 0;
pub const CGameUI__6__s_customHotkeyMappingTableNonHero__OFFSET: usize = 0;
pub const CGameUI__6__m_observerUnitToFollow__OFFSET: usize = 2376;

#[repr(C)]
pub struct CGameIdMaps {
    pub _opaque: [u8; 432],
}
pub const CGameIdMaps__SIZE: usize = 432;
pub const CGameIdMaps__NAME: &str = "CGameIdMaps";
pub const CGameIdMaps__m_unitIdMap__OFFSET: usize = 8;
pub const CGameIdMaps__m_itemIdMap__OFFSET: usize = 40;
pub const CGameIdMaps__m_abilityIdMap__OFFSET: usize = 72;
pub const CGameIdMaps__m_orderIdMap__OFFSET: usize = 104;
pub const CGameIdMaps__m_unitIdStringMap__OFFSET: usize = 136;
pub const CGameIdMaps__m_itemIdStringMap__OFFSET: usize = 208;
pub const CGameIdMaps__m_abilityIdStringMap__OFFSET: usize = 280;
pub const CGameIdMaps__m_orderIdStringMap__OFFSET: usize = 352;
pub const CGameIdMaps__m_affix__OFFSET: usize = 424;

#[repr(C)]
pub struct CTriggerExecution {
    pub _opaque: [u8; 320],
}
pub const CTriggerExecution__SIZE: usize = 320;
pub const CTriggerExecution__NAME: &str = "CTriggerExecution";
pub const CTriggerExecution__s_CTriggerExecution_pool__OFFSET: usize = 0;
pub const CTriggerExecution__m_context__OFFSET: usize = 88;
pub const CTriggerExecution__m_waitTimer__OFFSET: usize = 112;
pub const CTriggerExecution__m_instance_id__OFFSET: usize = 152;
pub const CTriggerExecution__m_waitingActionThread__OFFSET: usize = 160;
pub const CTriggerExecution__m_waitingAction__OFFSET: usize = 168;
pub const CTriggerExecution__m_actions__OFFSET: usize = 176;
pub const CTriggerExecution__m_remaining__OFFSET: usize = 200;
pub const CTriggerExecution__m_waitOnSleeps__OFFSET: usize = 204;
pub const CTriggerExecution__m_sleepWaiters__OFFSET: usize = 205;
pub const CTriggerExecution__m_destroyActionsOnFinish__OFFSET: usize = 206;
pub const CTriggerExecution__m_suspended__OFFSET: usize = 208;
pub const CTriggerExecution__m_destroyOnReturn__OFFSET: usize = 212;
pub const CTriggerExecution__m_waitingOn__OFFSET: usize = 216;
pub const CTriggerExecution__m_sync__OFFSET: usize = 240;
pub const CTriggerExecution__m_sleepId__OFFSET: usize = 244;
pub const CTriggerExecution__m_waitOnSound__OFFSET: usize = 248;
pub const CTriggerExecution__m_waitOnSoundOffset__OFFSET: usize = 256;
pub const CTriggerExecution__m_saveGameFileName__OFFSET: usize = 264;
pub const CTriggerExecution__m_saveMessage__OFFSET: usize = 288;
pub const CTriggerExecution__m_fromMenu__OFFSET: usize = 312;

#[repr(C)]
pub struct CWorldFrameWar3__2 {
    pub _opaque: [u8; 3128],
}
pub const CWorldFrameWar3__2__SIZE: usize = 3128;
pub const CWorldFrameWar3__2__NAME: &str = "CWorldFrameWar3";
pub const CWorldFrameWar3__2__m_trackUpdating__OFFSET: usize = 656;
pub const CWorldFrameWar3__2__m_sprites__OFFSET: usize = 664;
pub const CWorldFrameWar3__2__m_targetedSprites__OFFSET: usize = 672;
pub const CWorldFrameWar3__2__m_walkableSprites__OFFSET: usize = 680;
pub const CWorldFrameWar3__2__m_pGame__OFFSET: usize = 688;
pub const CWorldFrameWar3__2__m_pDoHpBarsEnemies__OFFSET: usize = 696;
pub const CWorldFrameWar3__2__m_pDoHpBarsAllies__OFFSET: usize = 697;
pub const CWorldFrameWar3__2__m_pLocalPlayer__OFFSET: usize = 704;
pub const CWorldFrameWar3__2__m_pFog__OFFSET: usize = 712;
pub const CWorldFrameWar3__2__m_cachedCameraPos__OFFSET: usize = 720;
pub const CWorldFrameWar3__2__m_localPlayerId__OFFSET: usize = 732;
pub const CWorldFrameWar3__2__m_pCameraWar3__OFFSET: usize = 736;
pub const CWorldFrameWar3__2__m_pMinimap__OFFSET: usize = 744;
pub const CWorldFrameWar3__2__m_spriteButtons__OFFSET: usize = 752;
pub const CWorldFrameWar3__2__m_terrainButtons__OFFSET: usize = 756;
pub const CWorldFrameWar3__2__m_cursorMode__OFFSET: usize = 760;
pub const CWorldFrameWar3__2__anticheat_dummy_array_20__OFFSET: usize = 764;
pub const CWorldFrameWar3__2__m_cursorModeStack__OFFSET: usize = 808;
pub const CWorldFrameWar3__2__m_cursorModeLocked__OFFSET: usize = 832;
pub const CWorldFrameWar3__2__m_forceCursorUpdate__OFFSET: usize = 833;
pub const CWorldFrameWar3__2__m_cursorArt__OFFSET: usize = 840;
pub const CWorldFrameWar3__2__m_aspectScaledProjection__OFFSET: usize = 864;
pub const CWorldFrameWar3__2__m_fovScale__OFFSET: usize = 880;
pub const CWorldFrameWar3__2__m_frustumBounds__OFFSET: usize = 884;
pub const CWorldFrameWar3__2__m_frustumBoundsInt__OFFSET: usize = 900;
pub const CWorldFrameWar3__2__m_localPlayerIdBit__OFFSET: usize = 916;
pub const CWorldFrameWar3__2__m_playerIdBitsToGhostMasks__OFFSET: usize = 920;
pub const CWorldFrameWar3__2__m_playerIdBitsToInvisMasks__OFFSET: usize = 1020;
pub const CWorldFrameWar3__2__m_localPlayerIsObserver__OFFSET: usize = 1120;
pub const CWorldFrameWar3__2__m_isReplayFogDisabled__OFFSET: usize = 1121;
pub const CWorldFrameWar3__2__m_lastUpdateElapsedSec__OFFSET: usize = 1124;
pub const CWorldFrameWar3__2__m_lastAsyncElapsedSec__OFFSET: usize = 1128;
pub const CWorldFrameWar3__2__m_lastTrackSprite__OFFSET: usize = 1136;
pub const CWorldFrameWar3__2__m_lastTrackAgent__OFFSET: usize = 1144;
pub const CWorldFrameWar3__2__m_renderPlacement__OFFSET: usize = 1152;
pub const CWorldFrameWar3__2__m_buildingPlacementRender__OFFSET: usize = 1153;
pub const CWorldFrameWar3__2__m_pBuildFrame__OFFSET: usize = 1160;
pub const CWorldFrameWar3__2__m_cineFilter__OFFSET: usize = 1168;
pub const CWorldFrameWar3__2__m_targetingSprite__OFFSET: usize = 1696;
pub const CWorldFrameWar3__2__m_renderTargetSprite__OFFSET: usize = 1704;
pub const CWorldFrameWar3__2__m_targetingImage__OFFSET: usize = 1708;
pub const CWorldFrameWar3__2__m_targetingRadius__OFFSET: usize = 1712;
pub const CWorldFrameWar3__2__m_targetingSqMag__OFFSET: usize = 1716;
pub const CWorldFrameWar3__2__m_targetImageColor__OFFSET: usize = 1720;
pub const CWorldFrameWar3__2__m_currentWorldPos__OFFSET: usize = 1724;
pub const CWorldFrameWar3__2__m_suspended__OFFSET: usize = 1736;
pub const CWorldFrameWar3__2__m_noHitTests__OFFSET: usize = 1737;
pub const CWorldFrameWar3__2__m_doOcclusion__OFFSET: usize = 1738;
pub const CWorldFrameWar3__2__m_mouseMoveProcessed__OFFSET: usize = 1739;
pub const CWorldFrameWar3__2__m_doFogOfWar__OFFSET: usize = 1740;
pub const CWorldFrameWar3__2__anticheat_dummy_array_21__OFFSET: usize = 1741;
pub const CWorldFrameWar3__2__m_useDarkMask__OFFSET: usize = 1744;
pub const CWorldFrameWar3__2__m_renderUnits__OFFSET: usize = 1745;
pub const CWorldFrameWar3__2__m_terrainFog__OFFSET: usize = 1752;
pub const CWorldFrameWar3__2__m_terrainLight__OFFSET: usize = 1760;
pub const CWorldFrameWar3__2__m_unitLight__OFFSET: usize = 1768;
pub const CWorldFrameWar3__2__m_suspendedAmbientLight__OFFSET: usize = 1776;
pub const CWorldFrameWar3__2__m_targetLight__OFFSET: usize = 1784;
pub const CWorldFrameWar3__2__m_skyModelPath__OFFSET: usize = 1792;
pub const CWorldFrameWar3__2__m_sky__OFFSET: usize = 1816;
pub const CWorldFrameWar3__2__m_showSky__OFFSET: usize = 1824;
pub const CWorldFrameWar3__2__m_renderSky__OFFSET: usize = 1825;
pub const CWorldFrameWar3__2__m_indicators__OFFSET: usize = 1832;
pub const CWorldFrameWar3__2__m_targetIndicators__OFFSET: usize = 1856;
pub const CWorldFrameWar3__2__m_targetIndicatorIndex__OFFSET: usize = 1880;
pub const CWorldFrameWar3__2__m_enumElapsed__OFFSET: usize = 1884;
pub const CWorldFrameWar3__2__m_asyncElapsed__OFFSET: usize = 1888;
pub const CWorldFrameWar3__2__m_doEnumUpdate__OFFSET: usize = 1892;
pub const CWorldFrameWar3__2__m_doFogEnum__OFFSET: usize = 1893;
pub const CWorldFrameWar3__2__m_trackingElapsed__OFFSET: usize = 1896;
pub const CWorldFrameWar3__2__m_terrainMinZ__OFFSET: usize = 1900;
pub const CWorldFrameWar3__2__m_scaledAnimTime__OFFSET: usize = 1904;
pub const CWorldFrameWar3__2__m_hoursPerDay__OFFSET: usize = 1908;
pub const CWorldFrameWar3__2__m_selCircColorFriend__OFFSET: usize = 1912;
pub const CWorldFrameWar3__2__anticheat_dummy_array_22__OFFSET: usize = 1916;
pub const CWorldFrameWar3__2__m_selCircColorNeutral__OFFSET: usize = 1921;
pub const CWorldFrameWar3__2__m_selCircColorEnemy__OFFSET: usize = 1925;
pub const CWorldFrameWar3__2__m_enableTargetIndicator__OFFSET: usize = 1929;
pub const CWorldFrameWar3__2__m_rallyFlags__OFFSET: usize = 1932;
pub const CWorldFrameWar3__2__m_rallyDst__OFFSET: usize = 1936;
pub const CWorldFrameWar3__2__m_rallyDstReuse__OFFSET: usize = 2708;
pub const CWorldFrameWar3__2__m_rallySrc__OFFSET: usize = 2776;
pub const CWorldFrameWar3__2__m_indicatorTerrain__OFFSET: usize = 2784;
pub const CWorldFrameWar3__2__m_waypointIndicators__OFFSET: usize = 2792;
pub const CWorldFrameWar3__2__m_waypointIndicatorIndex__OFFSET: usize = 2816;
pub const CWorldFrameWar3__2__m_unitUpdateQueue__OFFSET: usize = 2824;
pub const CWorldFrameWar3__2__m_destUpdateQueue__OFFSET: usize = 2848;
pub const CWorldFrameWar3__2__m_itemUpdateQueue__OFFSET: usize = 2872;
pub const CWorldFrameWar3__2__m_captUpdateQueue__OFFSET: usize = 2896;
pub const CWorldFrameWar3__2__m_imagUpdateQueue__OFFSET: usize = 2920;
pub const CWorldFrameWar3__2__m_ghstUpdateQueue__OFFSET: usize = 2944;
pub const CWorldFrameWar3__2__anticheat_dummy_array_23__OFFSET: usize = 2968;
pub const CWorldFrameWar3__2__m_enumeratedUnits__OFFSET: usize = 2976;
pub const CWorldFrameWar3__2__m_enumeratedItems__OFFSET: usize = 3000;
pub const CWorldFrameWar3__2__m_visibleUnits__OFFSET: usize = 3024;
pub const CWorldFrameWar3__2__m_visibleSelectables__OFFSET: usize = 3048;
pub const CWorldFrameWar3__2__m_fogUpdateList__OFFSET: usize = 3072;
pub const CWorldFrameWar3__2__m_ghostImages__OFFSET: usize = 3096;
pub const CWorldFrameWar3__2__m_lastRenderLight__OFFSET: usize = 3120;
pub const CWorldFrameWar3__2__m_lastRenderFog__OFFSET: usize = 3124;
pub const CWorldFrameWar3__2__enableDSQDenialMessage__OFFSET: usize = 0;

#[repr(C)]
pub struct CTimerWar3 {
    pub _opaque: [u8; 168],
}
pub const CTimerWar3__SIZE: usize = 168;
pub const CTimerWar3__NAME: &str = "CTimerWar3";
pub const CTimerWar3__s_CTimerWar3_pool__OFFSET: usize = 0;
pub const CTimerWar3__m_timer__OFFSET: usize = 88;
pub const CTimerWar3__m_handler__OFFSET: usize = 128;
pub const CTimerWar3__m_timeout__OFFSET: usize = 136;
pub const CTimerWar3__m_remaining__OFFSET: usize = 152;

#[repr(C)]
pub struct CTriggerAction {
    pub _opaque: [u8; 96],
}
pub const CTriggerAction__SIZE: usize = 96;
pub const CTriggerAction__NAME: &str = "CTriggerAction";
pub const CTriggerAction__s_CTriggerAction_pool__OFFSET: usize = 0;
pub const CTriggerAction__m_actionFunc__OFFSET: usize = 88;

#[repr(C)]
pub struct TriggerActionNode {
    pub _opaque: [u8; 24],
}
pub const TriggerActionNode__SIZE: usize = 24;
pub const TriggerActionNode__NAME: &str = "TriggerActionNode";
pub const TriggerActionNode__action__OFFSET: usize = 16;

#[repr(C)]
pub struct CTriggerCondition {
    pub _opaque: [u8; 96],
}
pub const CTriggerCondition__SIZE: usize = 96;
pub const CTriggerCondition__NAME: &str = "CTriggerCondition";
pub const CTriggerCondition__s_CTriggerCondition_pool__OFFSET: usize = 0;
pub const CTriggerCondition__m_condition__OFFSET: usize = 88;

#[repr(C)]
pub struct TriggerConditionNode {
    pub _opaque: [u8; 24],
}
pub const TriggerConditionNode__SIZE: usize = 24;
pub const TriggerConditionNode__NAME: &str = "TriggerConditionNode";
pub const TriggerConditionNode__condition__OFFSET: usize = 16;

#[repr(C)]
pub struct CTriggerWar3 {
    pub _opaque: [u8; 232],
}
pub const CTriggerWar3__SIZE: usize = 232;
pub const CTriggerWar3__NAME: &str = "CTriggerWar3";
pub const CTriggerWar3__s_CTriggerWar3_pool__OFFSET: usize = 0;
pub const CTriggerWar3__m_events__OFFSET: usize = 88;
pub const CTriggerWar3__m_conditions__OFFSET: usize = 112;
pub const CTriggerWar3__m_actions__OFFSET: usize = 136;
pub const CTriggerWar3__m_activateStyle__OFFSET: usize = 160;
pub const CTriggerWar3__m_triggerCount__OFFSET: usize = 184;
pub const CTriggerWar3__m_executionCount__OFFSET: usize = 188;
pub const CTriggerWar3__m_executionsInFlight__OFFSET: usize = 192;
pub const CTriggerWar3__m_enabled__OFFSET: usize = 200;
pub const CTriggerWar3__m_lastEvalResult__OFFSET: usize = 224;
pub const CTriggerWar3__m_waitOnSleeps__OFFSET: usize = 228;

#[repr(C)]
pub struct CPlayerChatEvent {
    pub _opaque: [u8; 48],
}
pub const CPlayerChatEvent__SIZE: usize = 48;
pub const CPlayerChatEvent__NAME: &str = "CPlayerChatEvent";
pub const CPlayerChatEvent__m_player__OFFSET: usize = 16;
pub const CPlayerChatEvent__m_chatMessage__OFFSET: usize = 24;

#[repr(C)]
pub struct CPlayerStateChanged {
    pub _opaque: [u8; 24],
}
pub const CPlayerStateChanged__SIZE: usize = 24;
pub const CPlayerStateChanged__NAME: &str = "CPlayerStateChanged";
pub const CPlayerStateChanged__m_stateVal__OFFSET: usize = 16;

#[repr(C)]
pub struct CPlayerEventDataBase {
    pub _opaque: [u8; 80],
}
pub const CPlayerEventDataBase__SIZE: usize = 80;
pub const CPlayerEventDataBase__NAME: &str = "CPlayerEventDataBase";
pub const CPlayerEventDataBase__s_CPlayerEventDataBase_pool__OFFSET: usize = 0;
pub const CPlayerEventDataBase__m_player__OFFSET: usize = 56;
pub const CPlayerEventDataBase__m_mouseX__OFFSET: usize = 68;
pub const CPlayerEventDataBase__m_mouseY__OFFSET: usize = 72;
pub const CPlayerEventDataBase__m_mouseButton__OFFSET: usize = 76;

#[repr(C)]
pub struct CPlayerStateLimitEventData {
    pub _opaque: [u8; 88],
}
pub const CPlayerStateLimitEventData__SIZE: usize = 88;
pub const CPlayerStateLimitEventData__NAME: &str = "CPlayerStateLimitEventData";
pub const CPlayerStateLimitEventData__s_CPlayerStateLimitEventData_pool__OFFSET: usize = 0;
pub const CPlayerStateLimitEventData__whichState__OFFSET: usize = 80;

#[repr(C)]
pub struct CPlayerChatMatchEventData {
    pub _opaque: [u8; 120],
}
pub const CPlayerChatMatchEventData__SIZE: usize = 120;
pub const CPlayerChatMatchEventData__NAME: &str = "CPlayerChatMatchEventData";
pub const CPlayerChatMatchEventData__s_CPlayerChatMatchEventData_pool__OFFSET: usize = 0;
pub const CPlayerChatMatchEventData__m_chatMessage__OFFSET: usize = 80;
pub const CPlayerChatMatchEventData__m_issuer__OFFSET: usize = 104;

#[repr(C)]
pub struct CPlayerUnitEventDataBase {
    pub _opaque: [u8; 104],
}
pub const CPlayerUnitEventDataBase__SIZE: usize = 104;
pub const CPlayerUnitEventDataBase__NAME: &str = "CPlayerUnitEventDataBase";
pub const CPlayerUnitEventDataBase__s_CPlayerUnitEventDataBase_pool__OFFSET: usize = 0;
pub const CPlayerUnitEventDataBase__m_unit__OFFSET: usize = 80;
pub const CPlayerUnitEventDataBase__m_otherUnit__OFFSET: usize = 92;

#[repr(C)]
pub struct CPlayerSyncEventData {
    pub _opaque: [u8; 136],
}
pub const CPlayerSyncEventData__SIZE: usize = 136;
pub const CPlayerSyncEventData__NAME: &str = "CPlayerSyncEventData";
pub const CPlayerSyncEventData__s_CPlayerSyncEventData_pool__OFFSET: usize = 0;
pub const CPlayerSyncEventData__m_prefix__OFFSET: usize = 80;
pub const CPlayerSyncEventData__m_data__OFFSET: usize = 104;
pub const CPlayerSyncEventData__m_fromServer__OFFSET: usize = 128;

#[repr(C)]
pub struct CPlayerKeyEvent {
    pub _opaque: [u8; 40],
}
pub const CPlayerKeyEvent__SIZE: usize = 40;
pub const CPlayerKeyEvent__NAME: &str = "CPlayerKeyEvent";
pub const CPlayerKeyEvent__m_player__OFFSET: usize = 16;
pub const CPlayerKeyEvent__m_key__OFFSET: usize = 24;
pub const CPlayerKeyEvent__m_metaKey__OFFSET: usize = 28;
pub const CPlayerKeyEvent__m_keyDown__OFFSET: usize = 32;

#[repr(C)]
pub struct CPlayerKeyEventData {
    pub _opaque: [u8; 104],
}
pub const CPlayerKeyEventData__SIZE: usize = 104;
pub const CPlayerKeyEventData__NAME: &str = "CPlayerKeyEventData";
pub const CPlayerKeyEventData__s_CPlayerKeyEventData_pool__OFFSET: usize = 0;
pub const CPlayerKeyEventData__m_key__OFFSET: usize = 80;
pub const CPlayerKeyEventData__m_metaKey__OFFSET: usize = 84;
pub const CPlayerKeyEventData__m_keyDown__OFFSET: usize = 88;
pub const CPlayerKeyEventData__m_issuer__OFFSET: usize = 92;

#[repr(C)]
pub struct CPlayerUnitAttackedEventData {
    pub _opaque: [u8; 104],
}
pub const CPlayerUnitAttackedEventData__SIZE: usize = 104;
pub const CPlayerUnitAttackedEventData__NAME: &str = "CPlayerUnitAttackedEventData";
pub const CPlayerUnitAttackedEventData__s_CPlayerUnitAttackedEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitDamagedEventData {
    pub _opaque: [u8; 136],
}
pub const CPlayerUnitDamagedEventData__SIZE: usize = 136;
pub const CPlayerUnitDamagedEventData__NAME: &str = "CPlayerUnitDamagedEventData";
pub const CPlayerUnitDamagedEventData__s_CPlayerUnitDamagedEventData_pool__OFFSET: usize = 0;
pub const CPlayerUnitDamagedEventData__m_damage__OFFSET: usize = 104;
pub const CPlayerUnitDamagedEventData__m_attackType__OFFSET: usize = 120;
pub const CPlayerUnitDamagedEventData__m_damageType__OFFSET: usize = 124;
pub const CPlayerUnitDamagedEventData__m_weaponType__OFFSET: usize = 128;
pub const CPlayerUnitDamagedEventData__m_attack__OFFSET: usize = 132;

#[repr(C)]
pub struct CPlayerUnitRescuedEventData {
    pub _opaque: [u8; 104],
}
pub const CPlayerUnitRescuedEventData__SIZE: usize = 104;
pub const CPlayerUnitRescuedEventData__NAME: &str = "CPlayerUnitRescuedEventData";
pub const CPlayerUnitRescuedEventData__s_CPlayerUnitRescuedEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitDeathEventData {
    pub _opaque: [u8; 104],
}
pub const CPlayerUnitDeathEventData__SIZE: usize = 104;
pub const CPlayerUnitDeathEventData__NAME: &str = "CPlayerUnitDeathEventData";
pub const CPlayerUnitDeathEventData__s_CPlayerUnitDeathEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitDecayEventData {
    pub _opaque: [u8; 104],
}
pub const CPlayerUnitDecayEventData__SIZE: usize = 104;
pub const CPlayerUnitDecayEventData__NAME: &str = "CPlayerUnitDecayEventData";
pub const CPlayerUnitDecayEventData__s_CPlayerUnitDecayEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitSelectedEventData {
    pub _opaque: [u8; 104],
}
pub const CPlayerUnitSelectedEventData__SIZE: usize = 104;
pub const CPlayerUnitSelectedEventData__NAME: &str = "CPlayerUnitSelectedEventData";
pub const CPlayerUnitSelectedEventData__s_CPlayerUnitSelectedEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitDeselectedEventData {
    pub _opaque: [u8; 104],
}
pub const CPlayerUnitDeselectedEventData__SIZE: usize = 104;
pub const CPlayerUnitDeselectedEventData__NAME: &str = "CPlayerUnitDeselectedEventData";
pub const CPlayerUnitDeselectedEventData__s_CPlayerUnitDeselectedEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitDetectedEventData {
    pub _opaque: [u8; 104],
}
pub const CPlayerUnitDetectedEventData__SIZE: usize = 104;
pub const CPlayerUnitDetectedEventData__NAME: &str = "CPlayerUnitDetectedEventData";
pub const CPlayerUnitDetectedEventData__s_CPlayerUnitDetectedEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitHiddenEventData {
    pub _opaque: [u8; 104],
}
pub const CPlayerUnitHiddenEventData__SIZE: usize = 104;
pub const CPlayerUnitHiddenEventData__NAME: &str = "CPlayerUnitHiddenEventData";
pub const CPlayerUnitHiddenEventData__s_CPlayerUnitHiddenEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitSummonedEventData {
    pub _opaque: [u8; 104],
}
pub const CPlayerUnitSummonedEventData__SIZE: usize = 104;
pub const CPlayerUnitSummonedEventData__NAME: &str = "CPlayerUnitSummonedEventData";
pub const CPlayerUnitSummonedEventData__s_CPlayerUnitSummonedEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitLoadedEventData {
    pub _opaque: [u8; 104],
}
pub const CPlayerUnitLoadedEventData__SIZE: usize = 104;
pub const CPlayerUnitLoadedEventData__NAME: &str = "CPlayerUnitLoadedEventData";
pub const CPlayerUnitLoadedEventData__s_CPlayerUnitLoadedEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitConstructStartEventData {
    pub _opaque: [u8; 104],
}
pub const CPlayerUnitConstructStartEventData__SIZE: usize = 104;
pub const CPlayerUnitConstructStartEventData__NAME: &str = "CPlayerUnitConstructStartEventData";
pub const CPlayerUnitConstructStartEventData__s_CPlayerUnitConstructStartEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitConstructFinishEventData {
    pub _opaque: [u8; 104],
}
pub const CPlayerUnitConstructFinishEventData__SIZE: usize = 104;
pub const CPlayerUnitConstructFinishEventData__NAME: &str = "CPlayerUnitConstructFinishEventData";
pub const CPlayerUnitConstructFinishEventData__s_CPlayerUnitConstructFinishEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitConstructCancelEventData {
    pub _opaque: [u8; 104],
}
pub const CPlayerUnitConstructCancelEventData__SIZE: usize = 104;
pub const CPlayerUnitConstructCancelEventData__NAME: &str = "CPlayerUnitConstructCancelEventData";
pub const CPlayerUnitConstructCancelEventData__s_CPlayerUnitConstructCancelEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitUpgradeEventDataBase {
    pub _opaque: [u8; 112],
}
pub const CPlayerUnitUpgradeEventDataBase__SIZE: usize = 112;
pub const CPlayerUnitUpgradeEventDataBase__NAME: &str = "CPlayerUnitUpgradeEventDataBase";
pub const CPlayerUnitUpgradeEventDataBase__s_CPlayerUnitUpgradeEventDataBase_pool__OFFSET: usize = 0;
pub const CPlayerUnitUpgradeEventDataBase__m_upgradeTo__OFFSET: usize = 104;

#[repr(C)]
pub struct CPlayerUnitUpgradeFinishEventData {
    pub _opaque: [u8; 112],
}
pub const CPlayerUnitUpgradeFinishEventData__SIZE: usize = 112;
pub const CPlayerUnitUpgradeFinishEventData__NAME: &str = "CPlayerUnitUpgradeFinishEventData";
pub const CPlayerUnitUpgradeFinishEventData__s_CPlayerUnitUpgradeFinishEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitUpgradeCancelEventData {
    pub _opaque: [u8; 112],
}
pub const CPlayerUnitUpgradeCancelEventData__SIZE: usize = 112;
pub const CPlayerUnitUpgradeCancelEventData__NAME: &str = "CPlayerUnitUpgradeCancelEventData";
pub const CPlayerUnitUpgradeCancelEventData__s_CPlayerUnitUpgradeCancelEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitResearchEventDataBase {
    pub _opaque: [u8; 112],
}
pub const CPlayerUnitResearchEventDataBase__SIZE: usize = 112;
pub const CPlayerUnitResearchEventDataBase__NAME: &str = "CPlayerUnitResearchEventDataBase";
pub const CPlayerUnitResearchEventDataBase__s_CPlayerUnitResearchEventDataBase_pool__OFFSET: usize = 0;
pub const CPlayerUnitResearchEventDataBase__m_researching__OFFSET: usize = 104;

#[repr(C)]
pub struct CPlayerHeroSkillEventData {
    pub _opaque: [u8; 112],
}
pub const CPlayerHeroSkillEventData__SIZE: usize = 112;
pub const CPlayerHeroSkillEventData__NAME: &str = "CPlayerHeroSkillEventData";
pub const CPlayerHeroSkillEventData__s_CPlayerHeroSkillEventData_pool__OFFSET: usize = 0;
pub const CPlayerHeroSkillEventData__m_skill__OFFSET: usize = 104;
pub const CPlayerHeroSkillEventData__m_level__OFFSET: usize = 108;

#[repr(C)]
pub struct CPlayerUnitTrainEventDataBase {
    pub _opaque: [u8; 104],
}
pub const CPlayerUnitTrainEventDataBase__SIZE: usize = 104;
pub const CPlayerUnitTrainEventDataBase__NAME: &str = "CPlayerUnitTrainEventDataBase";
pub const CPlayerUnitTrainEventDataBase__s_CPlayerUnitTrainEventDataBase_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitChangeOwnerEventData {
    pub _opaque: [u8; 112],
}
pub const CPlayerUnitChangeOwnerEventData__SIZE: usize = 112;
pub const CPlayerUnitChangeOwnerEventData__NAME: &str = "CPlayerUnitChangeOwnerEventData";
pub const CPlayerUnitChangeOwnerEventData__s_CPlayerUnitChangeOwnerEventData_pool__OFFSET: usize = 0;
pub const CPlayerUnitChangeOwnerEventData__m_oldOwner__OFFSET: usize = 104;

#[repr(C)]
pub struct CPlayerUnitTrainStartEventData {
    pub _opaque: [u8; 112],
}
pub const CPlayerUnitTrainStartEventData__SIZE: usize = 112;
pub const CPlayerUnitTrainStartEventData__NAME: &str = "CPlayerUnitTrainStartEventData";
pub const CPlayerUnitTrainStartEventData__s_CPlayerUnitTrainStartEventData_pool__OFFSET: usize = 0;
pub const CPlayerUnitTrainStartEventData__m_unitId__OFFSET: usize = 104;

#[repr(C)]
pub struct CPlayerUnitTrainCancelEventData {
    pub _opaque: [u8; 112],
}
pub const CPlayerUnitTrainCancelEventData__SIZE: usize = 112;
pub const CPlayerUnitTrainCancelEventData__NAME: &str = "CPlayerUnitTrainCancelEventData";
pub const CPlayerUnitTrainCancelEventData__s_CPlayerUnitTrainCancelEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitTrainFinishEventData {
    pub _opaque: [u8; 112],
}
pub const CPlayerUnitTrainFinishEventData__SIZE: usize = 112;
pub const CPlayerUnitTrainFinishEventData__NAME: &str = "CPlayerUnitTrainFinishEventData";
pub const CPlayerUnitTrainFinishEventData__s_CPlayerUnitTrainFinishEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitSellEventData {
    pub _opaque: [u8; 120],
}
pub const CPlayerUnitSellEventData__SIZE: usize = 120;
pub const CPlayerUnitSellEventData__NAME: &str = "CPlayerUnitSellEventData";
pub const CPlayerUnitSellEventData__s_CPlayerUnitSellEventData_pool__OFFSET: usize = 0;
pub const CPlayerUnitSellEventData__m_buyingUnit__OFFSET: usize = 104;

#[repr(C)]
pub struct CPlayerUnitSellItemEventData {
    pub _opaque: [u8; 120],
}
pub const CPlayerUnitSellItemEventData__SIZE: usize = 120;
pub const CPlayerUnitSellItemEventData__NAME: &str = "CPlayerUnitSellItemEventData";
pub const CPlayerUnitSellItemEventData__s_CPlayerUnitSellItemEventData_pool__OFFSET: usize = 0;
pub const CPlayerUnitSellItemEventData__m_soldItem__OFFSET: usize = 104;

#[repr(C)]
pub struct CPlayerUnitOrderEventData {
    pub _opaque: [u8; 120],
}
pub const CPlayerUnitOrderEventData__SIZE: usize = 120;
pub const CPlayerUnitOrderEventData__NAME: &str = "CPlayerUnitOrderEventData";
pub const CPlayerUnitOrderEventData__s_CPlayerUnitOrderEventData_pool__OFFSET: usize = 0;
pub const CPlayerUnitOrderEventData__m_unitOrderEventData__OFFSET: usize = 104;

#[repr(C)]
pub struct CPlayerUnitSpellEventDataBase {
    pub _opaque: [u8; 120],
}
pub const CPlayerUnitSpellEventDataBase__SIZE: usize = 120;
pub const CPlayerUnitSpellEventDataBase__NAME: &str = "CPlayerUnitSpellEventDataBase";
pub const CPlayerUnitSpellEventDataBase__s_CPlayerUnitSpellEventDataBase_pool__OFFSET: usize = 0;
pub const CPlayerUnitSpellEventDataBase__m_unitEventData__OFFSET: usize = 104;

#[repr(C)]
pub struct CPlayerUnitItemEventData {
    pub _opaque: [u8; 120],
}
pub const CPlayerUnitItemEventData__SIZE: usize = 120;
pub const CPlayerUnitItemEventData__NAME: &str = "CPlayerUnitItemEventData";
pub const CPlayerUnitItemEventData__s_CPlayerUnitItemEventData_pool__OFFSET: usize = 0;
pub const CPlayerUnitItemEventData__m_item__OFFSET: usize = 104;

#[repr(C)]
pub struct CPlayerUnitItemItemEventData {
    pub _opaque: [u8; 136],
}
pub const CPlayerUnitItemItemEventData__SIZE: usize = 136;
pub const CPlayerUnitItemItemEventData__NAME: &str = "CPlayerUnitItemItemEventData";
pub const CPlayerUnitItemItemEventData__s_CPlayerUnitItemItemEventData_pool__OFFSET: usize = 0;
pub const CPlayerUnitItemItemEventData__m_otherItem__OFFSET: usize = 120;

#[repr(C)]
pub struct CPlayerUnitItemPickupEventData {
    pub _opaque: [u8; 144],
}
pub const CPlayerUnitItemPickupEventData__SIZE: usize = 144;
pub const CPlayerUnitItemPickupEventData__NAME: &str = "CPlayerUnitItemPickupEventData";
pub const CPlayerUnitItemPickupEventData__s_CPlayerUnitItemPickupEventData_pool__OFFSET: usize = 0;
pub const CPlayerUnitItemPickupEventData__m_wasAbsorbed__OFFSET: usize = 136;

#[repr(C)]
pub struct CPlayerUnitItemStackEventData {
    pub _opaque: [u8; 144],
}
pub const CPlayerUnitItemStackEventData__SIZE: usize = 144;
pub const CPlayerUnitItemStackEventData__NAME: &str = "CPlayerUnitItemStackEventData";
pub const CPlayerUnitItemStackEventData__s_CPlayerUnitItemStackEventData_pool__OFFSET: usize = 0;
pub const CPlayerUnitItemStackEventData__m_previousCharges__OFFSET: usize = 136;

#[repr(C)]
pub struct CUnitStatusIconUI {
    pub _opaque: [u8; 592],
}
pub const CUnitStatusIconUI__SIZE: usize = 592;
pub const CUnitStatusIconUI__NAME: &str = "CUnitStatusIconUI";
pub const CUnitStatusIconUI__m_unit__OFFSET: usize = 504;
pub const CUnitStatusIconUI__m_icon__OFFSET: usize = 512;
pub const CUnitStatusIconUI__m_shouldShow__OFFSET: usize = 520;
pub const CUnitStatusIconUI__m_updateTimer__OFFSET: usize = 528;
pub const CUnitStatusIconUI__m_fadeTimer__OFFSET: usize = 568;
pub const CUnitStatusIconUI__m_targetHpWidth__OFFSET: usize = 576;
pub const CUnitStatusIconUI__m_targetManaWidth__OFFSET: usize = 580;
pub const CUnitStatusIconUI__m_lastCameraDistance__OFFSET: usize = 584;

#[repr(C)]
pub struct CCameraSetup {
    pub _opaque: [u8; 696],
}
pub const CCameraSetup__SIZE: usize = 696;
pub const CCameraSetup__NAME: &str = "CCameraSetup";
pub const CCameraSetup__s_CCameraSetup_pool__OFFSET: usize = 0;
pub const CCameraSetup__m_distance__OFFSET: usize = 88;
pub const CCameraSetup__m_farZ__OFFSET: usize = 112;
pub const CCameraSetup__m_nearZ__OFFSET: usize = 136;
pub const CCameraSetup__m_fov__OFFSET: usize = 160;
pub const CCameraSetup__m_rotation__OFFSET: usize = 184;
pub const CCameraSetup__m_aoa__OFFSET: usize = 208;
pub const CCameraSetup__m_roll__OFFSET: usize = 232;
pub const CCameraSetup__m_localPitch__OFFSET: usize = 256;
pub const CCameraSetup__m_localYaw__OFFSET: usize = 280;
pub const CCameraSetup__m_localRoll__OFFSET: usize = 304;
pub const CCameraSetup__m_targetZOffset__OFFSET: usize = 328;
pub const CCameraSetup__m_destPos__OFFSET: usize = 352;
pub const CCameraSetup__m_distanceDuration__OFFSET: usize = 368;
pub const CCameraSetup__m_farZDuration__OFFSET: usize = 392;
pub const CCameraSetup__m_nearZDuration__OFFSET: usize = 416;
pub const CCameraSetup__m_fovDuration__OFFSET: usize = 440;
pub const CCameraSetup__m_rotationDuration__OFFSET: usize = 464;
pub const CCameraSetup__m_aoaDuration__OFFSET: usize = 488;
pub const CCameraSetup__m_rollDuration__OFFSET: usize = 512;
pub const CCameraSetup__m_localPitchDuration__OFFSET: usize = 536;
pub const CCameraSetup__m_localYawDuration__OFFSET: usize = 560;
pub const CCameraSetup__m_localRollDuration__OFFSET: usize = 584;
pub const CCameraSetup__m_targetZOffsetDuration__OFFSET: usize = 608;
pub const CCameraSetup__m_destPosDuration__OFFSET: usize = 632;
pub const CCameraSetup__m_label__OFFSET: usize = 656;

#[repr(C)]
pub struct CGameEventReg {
    pub _opaque: [u8; 104],
}
pub const CGameEventReg__SIZE: usize = 104;
pub const CGameEventReg__NAME: &str = "CGameEventReg";
pub const CGameEventReg__s_CGameEventReg_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitEventReg {
    pub _opaque: [u8; 128],
}
pub const CUnitEventReg__SIZE: usize = 128;
pub const CUnitEventReg__NAME: &str = "CUnitEventReg";
pub const CUnitEventReg__s_CUnitEventReg_pool__OFFSET: usize = 0;
pub const CUnitEventReg__m_eventId__OFFSET: usize = 104;
pub const CUnitEventReg__m_unit__OFFSET: usize = 108;
pub const CUnitEventReg__m_unitFilter__OFFSET: usize = 120;

#[repr(C)]
pub struct CUnitStateEventReg {
    pub _opaque: [u8; 176],
}
pub const CUnitStateEventReg__SIZE: usize = 176;
pub const CUnitStateEventReg__NAME: &str = "CUnitStateEventReg";
pub const CUnitStateEventReg__s_CUnitStateEventReg_pool__OFFSET: usize = 0;
pub const CUnitStateEventReg__m_limitOp__OFFSET: usize = 128;
pub const CUnitStateEventReg__m_limitValue__OFFSET: usize = 136;
pub const CUnitStateEventReg__m_listener__OFFSET: usize = 152;
pub const CUnitStateEventReg__m_state__OFFSET: usize = 168;

#[repr(C)]
pub struct CPlayerStateEventReg {
    pub _opaque: [u8; 168],
}
pub const CPlayerStateEventReg__SIZE: usize = 168;
pub const CPlayerStateEventReg__NAME: &str = "CPlayerStateEventReg";
pub const CPlayerStateEventReg__s_CPlayerStateEventReg_pool__OFFSET: usize = 0;
pub const CPlayerStateEventReg__m_limitOp__OFFSET: usize = 136;
pub const CPlayerStateEventReg__m_limitValue__OFFSET: usize = 144;
pub const CPlayerStateEventReg__m_listener__OFFSET: usize = 160;

#[repr(C)]
pub struct CTimerEventReg {
    pub _opaque: [u8; 144],
}
pub const CTimerEventReg__SIZE: usize = 144;
pub const CTimerEventReg__NAME: &str = "CTimerEventReg";
pub const CTimerEventReg__s_CTimerEventReg_pool__OFFSET: usize = 0;
pub const CTimerEventReg__m_timer__OFFSET: usize = 104;

#[repr(C)]
pub struct CTimerExpireEventReg {
    pub _opaque: [u8; 112],
}
pub const CTimerExpireEventReg__SIZE: usize = 112;
pub const CTimerExpireEventReg__NAME: &str = "CTimerExpireEventReg";
pub const CTimerExpireEventReg__s_CTimerExpireEventReg_pool__OFFSET: usize = 0;
pub const CTimerExpireEventReg__m_timer__OFFSET: usize = 104;

#[repr(C)]
pub struct RegionEvent {
    pub _opaque: [u8; 32],
}
pub const RegionEvent__SIZE: usize = 32;
pub const RegionEvent__NAME: &str = "RegionEvent";
pub const RegionEvent__who__OFFSET: usize = 16;
pub const RegionEvent__enter__OFFSET: usize = 24;

#[repr(C)]
pub struct CRegionEventReg {
    pub _opaque: [u8; 176],
}
pub const CRegionEventReg__SIZE: usize = 176;
pub const CRegionEventReg__NAME: &str = "CRegionEventReg";
pub const CRegionEventReg__s_CRegionEventReg_pool__OFFSET: usize = 0;
pub const CRegionEventReg__m_triggerRegion__OFFSET: usize = 104;
pub const CRegionEventReg__m_filter__OFFSET: usize = 112;
pub const CRegionEventReg__m_timer__OFFSET: usize = 120;
pub const CRegionEventReg__m_events__OFFSET: usize = 152;

#[repr(C)]
pub struct UnitChoice {
    pub _opaque: [u8; 8],
}
pub const UnitChoice__SIZE: usize = 8;
pub const UnitChoice__NAME: &str = "UnitChoice";
pub const UnitChoice__unitId__OFFSET: usize = 0;
pub const UnitChoice__weight__OFFSET: usize = 4;

#[repr(C)]
pub struct CTimerDialogWar3 {
    pub _opaque: [u8; 120],
}
pub const CTimerDialogWar3__SIZE: usize = 120;
pub const CTimerDialogWar3__NAME: &str = "CTimerDialogWar3";
pub const CTimerDialogWar3__s_CTimerDialogWar3_pool__OFFSET: usize = 0;
pub const CTimerDialogWar3__s_timerCount__OFFSET: usize = 0;
pub const CTimerDialogWar3__m_pDialog__OFFSET: usize = 88;
pub const CTimerDialogWar3__m_timer__OFFSET: usize = 96;
pub const CTimerDialogWar3__m_visible__OFFSET: usize = 108;
pub const CTimerDialogWar3__m_data__OFFSET: usize = 112;

#[repr(C)]
pub struct TimerDialogData {
    pub _opaque: [u8; 40],
}
pub const TimerDialogData__SIZE: usize = 40;
pub const TimerDialogData__NAME: &str = "TimerDialogData";
pub const TimerDialogData__titleColor__OFFSET: usize = 0;
pub const TimerDialogData__valueColor__OFFSET: usize = 4;
pub const TimerDialogData__titleText__OFFSET: usize = 8;
pub const TimerDialogData__speedFactor__OFFSET: usize = 32;

#[repr(C)]
pub struct CTimerDialog {
    pub _opaque: [u8; 744],
}
pub const CTimerDialog__SIZE: usize = 744;
pub const CTimerDialog__NAME: &str = "CTimerDialog";
pub const CTimerDialog__m_timerBackdrop__OFFSET: usize = 656;
pub const CTimerDialog__m_timerTitle__OFFSET: usize = 664;
pub const CTimerDialog__m_timerValue__OFFSET: usize = 672;
pub const CTimerDialog__m_refreshTimer__OFFSET: usize = 680;
pub const CTimerDialog__m_speedFactor__OFFSET: usize = 720;
pub const CTimerDialog__m_timer__OFFSET: usize = 728;
pub const CTimerDialog__m_useRealTime__OFFSET: usize = 736;
pub const CTimerDialog__m_timeRemaining__OFFSET: usize = 740;

#[repr(C)]
pub struct PlayerHeroInfo {
    pub _opaque: [u8; 52],
}
pub const PlayerHeroInfo__SIZE: usize = 52;
pub const PlayerHeroInfo__NAME: &str = "PlayerHeroInfo";
pub const PlayerHeroInfo__level__OFFSET: usize = 0;
pub const PlayerHeroInfo__number_of_deaths__OFFSET: usize = 4;
pub const PlayerHeroInfo__experience__OFFSET: usize = 8;
pub const PlayerHeroInfo__time_alive_ms__OFFSET: usize = 12;
pub const PlayerHeroInfo__xp_per_minute__OFFSET: usize = 16;
pub const PlayerHeroInfo__play_time__OFFSET: usize = 20;
pub const PlayerHeroInfo__total_kills__OFFSET: usize = 24;
pub const PlayerHeroInfo__hero_kills__OFFSET: usize = 28;
pub const PlayerHeroInfo__building_kills__OFFSET: usize = 32;
pub const PlayerHeroInfo__damage_dealt__OFFSET: usize = 36;
pub const PlayerHeroInfo__healing_done__OFFSET: usize = 40;
pub const PlayerHeroInfo__damage_received__OFFSET: usize = 44;
pub const PlayerHeroInfo__pick_order__OFFSET: usize = 48;

#[repr(C)]
pub struct CPlayerSyncEventReg {
    pub _opaque: [u8; 152],
}
pub const CPlayerSyncEventReg__SIZE: usize = 152;
pub const CPlayerSyncEventReg__NAME: &str = "CPlayerSyncEventReg";
pub const CPlayerSyncEventReg__s_CPlayerSyncEventReg_pool__OFFSET: usize = 0;
pub const CPlayerSyncEventReg__m_eventId__OFFSET: usize = 104;
pub const CPlayerSyncEventReg__m_player__OFFSET: usize = 108;
pub const CPlayerSyncEventReg__m_fromServer__OFFSET: usize = 120;
pub const CPlayerSyncEventReg__m_prefix__OFFSET: usize = 128;

#[repr(C)]
pub struct CPlayerDefeatEventData {
    pub _opaque: [u8; 80],
}
pub const CPlayerDefeatEventData__SIZE: usize = 80;
pub const CPlayerDefeatEventData__NAME: &str = "CPlayerDefeatEventData";
pub const CPlayerDefeatEventData__s_CPlayerDefeatEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerVictoryEventData {
    pub _opaque: [u8; 80],
}
pub const CPlayerVictoryEventData__SIZE: usize = 80;
pub const CPlayerVictoryEventData__NAME: &str = "CPlayerVictoryEventData";
pub const CPlayerVictoryEventData__s_CPlayerVictoryEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerLeaveEventData {
    pub _opaque: [u8; 80],
}
pub const CPlayerLeaveEventData__SIZE: usize = 80;
pub const CPlayerLeaveEventData__NAME: &str = "CPlayerLeaveEventData";
pub const CPlayerLeaveEventData__s_CPlayerLeaveEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerEndCinematicData {
    pub _opaque: [u8; 80],
}
pub const CPlayerEndCinematicData__SIZE: usize = 80;
pub const CPlayerEndCinematicData__NAME: &str = "CPlayerEndCinematicData";
pub const CPlayerEndCinematicData__s_CPlayerEndCinematicData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitUpgradeStartEventData {
    pub _opaque: [u8; 112],
}
pub const CPlayerUnitUpgradeStartEventData__SIZE: usize = 112;
pub const CPlayerUnitUpgradeStartEventData__NAME: &str = "CPlayerUnitUpgradeStartEventData";
pub const CPlayerUnitUpgradeStartEventData__s_CPlayerUnitUpgradeStartEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitResearchStartEventData {
    pub _opaque: [u8; 112],
}
pub const CPlayerUnitResearchStartEventData__SIZE: usize = 112;
pub const CPlayerUnitResearchStartEventData__NAME: &str = "CPlayerUnitResearchStartEventData";
pub const CPlayerUnitResearchStartEventData__s_CPlayerUnitResearchStartEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitResearchFinishEventData {
    pub _opaque: [u8; 112],
}
pub const CPlayerUnitResearchFinishEventData__SIZE: usize = 112;
pub const CPlayerUnitResearchFinishEventData__NAME: &str = "CPlayerUnitResearchFinishEventData";
pub const CPlayerUnitResearchFinishEventData__s_CPlayerUnitResearchFinishEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitResearchCancelEventData {
    pub _opaque: [u8; 112],
}
pub const CPlayerUnitResearchCancelEventData__SIZE: usize = 112;
pub const CPlayerUnitResearchCancelEventData__NAME: &str = "CPlayerUnitResearchCancelEventData";
pub const CPlayerUnitResearchCancelEventData__s_CPlayerUnitResearchCancelEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CGameUI__7 {
    pub _opaque: [u8; 2384],
}
pub const CGameUI__7__SIZE: usize = 2384;
pub const CGameUI__7__NAME: &str = "CGameUI";
pub const CGameUI__7__m_transitionState__OFFSET: usize = 704;
pub const CGameUI__7__m_cachedFonts__OFFSET: usize = 712;
pub const CGameUI__7__NUMBER_OF_PLAYER_ROWS__OFFSET: usize = 0;
pub const CGameUI__7__FIRST_ROW_Y_OFFSET__OFFSET: usize = 0;
pub const CGameUI__7__ROW_SPACING__OFFSET: usize = 0;
pub const CGameUI__7__m_userControl__OFFSET: usize = 784;
pub const CGameUI__7__m_userUI__OFFSET: usize = 785;
pub const CGameUI__7__m_currentInputMode__OFFSET: usize = 792;
pub const CGameUI__7__m_inputModeStack__OFFSET: usize = 800;
pub const CGameUI__7__m_sharedTooltip__OFFSET: usize = 824;
pub const CGameUI__7__m_uberTooltip__OFFSET: usize = 832;
pub const CGameUI__7__m_showUberTips__OFFSET: usize = 840;
pub const CGameUI__7__m_autosaveReplay__OFFSET: usize = 841;
pub const CGameUI__7__m_healthBarsMode__OFFSET: usize = 844;
pub const CGameUI__7__m_forceShowHealthBars__OFFSET: usize = 848;
pub const CGameUI__7__m_minimapSignals__OFFSET: usize = 849;
pub const CGameUI__7__m_useColoredHealthBars__OFFSET: usize = 850;
pub const CGameUI__7__m_useHeroFrames__OFFSET: usize = 851;
pub const CGameUI__7__m_showHeroLevelFrame__OFFSET: usize = 852;
pub const CGameUI__7__m_showNumericCooldown__OFFSET: usize = 853;
pub const CGameUI__7__m_showDenyIcon__OFFSET: usize = 854;
pub const CGameUI__7__m_showGoldmineUnitCounter__OFFSET: usize = 855;
pub const CGameUI__7__m_enableObserverUiV2__OFFSET: usize = 856;
pub const CGameUI__7__m_asyncPlayerData__OFFSET: usize = 860;
pub const CGameUI__7__m_targetMode__OFFSET: usize = 960;
pub const CGameUI__7__m_selectMode__OFFSET: usize = 968;
pub const CGameUI__7__m_dragSelectMode__OFFSET: usize = 976;
pub const CGameUI__7__m_dragScrollMode__OFFSET: usize = 984;
pub const CGameUI__7__m_buildMode__OFFSET: usize = 992;
pub const CGameUI__7__m_signalMode__OFFSET: usize = 1000;
pub const CGameUI__7__m_escapeMenu__OFFSET: usize = 1008;
pub const CGameUI__7__m_gameOverMenu__OFFSET: usize = 1016;
pub const CGameUI__7__m_suspendIndicator__OFFSET: usize = 1024;
pub const CGameUI__7__m_unresponsiveIndicator__OFFSET: usize = 1032;
pub const CGameUI__7__m_allianceMode__OFFSET: usize = 1040;
pub const CGameUI__7__m_chatMode__OFFSET: usize = 1048;
pub const CGameUI__7__m_questMode__OFFSET: usize = 1056;
pub const CGameUI__7__anticheat_dummy_array_26__OFFSET: usize = 1064;
pub const CGameUI__7__m_scriptDialogMode__OFFSET: usize = 1152;
pub const CGameUI__7__m_unresponsiveDelayTimerId__OFFSET: usize = 1160;
pub const CGameUI__7__m_selectionManager__OFFSET: usize = 1168;
pub const CGameUI__7__m_dragScrollManager__OFFSET: usize = 1176;
pub const CGameUI__7__m_camera__OFFSET: usize = 1184;
pub const CGameUI__7__m_paused__OFFSET: usize = 1192;
pub const CGameUI__7__m_uiPaused__OFFSET: usize = 1193;
pub const CGameUI__7__m_scriptPaused__OFFSET: usize = 1194;
pub const CGameUI__7__m_disableUpkeepMsg__OFFSET: usize = 1195;
pub const CGameUI__7__m_upkeepLvlSave__OFFSET: usize = 1196;
pub const CGameUI__7__m_refresh__OFFSET: usize = 1200;
pub const CGameUI__7__m_refreshWaypoints__OFFSET: usize = 1201;
pub const CGameUI__7__m_deferCleanSelUI__OFFSET: usize = 1202;
pub const CGameUI__7__m_refreshInfoBar__OFFSET: usize = 1203;
pub const CGameUI__7__m_refreshCommandBar__OFFSET: usize = 1204;
pub const CGameUI__7__m_refreshHeroBar__OFFSET: usize = 1205;
pub const CGameUI__7__m_popSubMenu__OFFSET: usize = 1206;
pub const CGameUI__7__m_activeValidOrder__OFFSET: usize = 1208;
pub const CGameUI__7__m_cineForceSubtitles__OFFSET: usize = 1212;
pub const CGameUI__7__m_cinematicPanelsHidden__OFFSET: usize = 1213;
pub const CGameUI__7__m_dragSelectEnabled__OFFSET: usize = 1214;
pub const CGameUI__7__m_dragSelectUIEnabled__OFFSET: usize = 1215;
pub const CGameUI__7__m_preSelectEnabled__OFFSET: usize = 1216;
pub const CGameUI__7__m_preSelectUIEnabled__OFFSET: usize = 1217;
pub const CGameUI__7__m_selectEnabled__OFFSET: usize = 1218;
pub const CGameUI__7__m_selectUIEnabled__OFFSET: usize = 1219;
pub const CGameUI__7__m_observerModeEnabled__OFFSET: usize = 1220;
pub const CGameUI__7__m_observerModeV2Active__OFFSET: usize = 1221;
pub const CGameUI__7__m_wideScreenAutoPosition__OFFSET: usize = 1222;
pub const CGameUI__7__m_fadeTimer__OFFSET: usize = 1224;
pub const CGameUI__7__m_cinematicFadeTimer__OFFSET: usize = 1328;
pub const CGameUI__7__anticheat_dummy_array_27__OFFSET: usize = 1432;
pub const CGameUI__7__m_frameParentTimer__OFFSET: usize = 1440;
pub const CGameUI__7__m_simpleFadeTimer__OFFSET: usize = 1544;
pub const CGameUI__7__m_attackNotify__OFFSET: usize = 1552;
pub const CGameUI__7__m_warpNotify__OFFSET: usize = 1576;
pub const CGameUI__7__m_timerDialogs__OFFSET: usize = 1608;
pub const CGameUI__7__m_leaderboard__OFFSET: usize = 1632;
pub const CGameUI__7__anticheat_dummy_array_28__OFFSET: usize = 1640;
pub const CGameUI__7__m_multiboard__OFFSET: usize = 1648;
pub const CGameUI__7__m_suspendMultiboardDisplay__OFFSET: usize = 1656;
pub const CGameUI__7__anticheat_dummy_array_29__OFFSET: usize = 1657;
pub const CGameUI__7__m_scriptDialog__OFFSET: usize = 1728;
pub const CGameUI__7__m_replaySelection__OFFSET: usize = 1736;
pub const CGameUI__7__m_unitStatusIconUIFactory__OFFSET: usize = 1744;
pub const CGameUI__7__m_unitStatusTexts__OFFSET: usize = 1752;
pub const CGameUI__7__m_worldFrame__OFFSET: usize = 1776;
pub const CGameUI__7__m_infoBar__OFFSET: usize = 1784;
pub const CGameUI__7__m_commandBar__OFFSET: usize = 1792;
pub const CGameUI__7__m_miniMap__OFFSET: usize = 1800;
pub const CGameUI__7__m_resourceBar__OFFSET: usize = 1808;
pub const CGameUI__7__m_upperButtonBar__OFFSET: usize = 1816;
pub const CGameUI__7__m_observerButtonBar__OFFSET: usize = 1824;
pub const CGameUI__7__m_playerStatisticsPanel__OFFSET: usize = 1832;
pub const CGameUI__7__m_heroPanel__OFFSET: usize = 1840;
pub const CGameUI__7__m_productionPanel__OFFSET: usize = 1848;
pub const CGameUI__7__m_minimapButtonBar__OFFSET: usize = 1856;
pub const CGameUI__7__m_trackingStub__OFFSET: usize = 1864;
pub const CGameUI__7__m_heroBar__OFFSET: usize = 1872;
pub const CGameUI__7__m_peonBar__OFFSET: usize = 1880;
pub const CGameUI__7__m_messageBox__OFFSET: usize = 1888;
pub const CGameUI__7__m_unitMessageBox__OFFSET: usize = 1896;
pub const CGameUI__7__m_chatMessageBox__OFFSET: usize = 1904;
pub const CGameUI__7__m_topMessageBox__OFFSET: usize = 1912;
pub const CGameUI__7__m_currentPortraitButton__OFFSET: usize = 1920;
pub const CGameUI__7__m_timeOfDayIndicator__OFFSET: usize = 1928;
pub const CGameUI__7__m_chatEditBar__OFFSET: usize = 1936;
pub const CGameUI__7__m_cinematicPanel__OFFSET: usize = 1944;
pub const CGameUI__7__m_lastBuffBar__OFFSET: usize = 1952;
pub const CGameUI__7__m_observerBar__OFFSET: usize = 1960;
pub const CGameUI__7__m_miniMapSignalButton__OFFSET: usize = 1968;
pub const CGameUI__7__m_miniMapTerrainButton__OFFSET: usize = 1976;
pub const CGameUI__7__m_miniMapAllyButton__OFFSET: usize = 1984;
pub const CGameUI__7__m_miniMapCreepButton__OFFSET: usize = 1992;
pub const CGameUI__7__m_formationButton__OFFSET: usize = 2000;
pub const CGameUI__7__m_consoleFrame__OFFSET: usize = 2008;
pub const CGameUI__7__m_simpleConsoleFrame__OFFSET: usize = 2016;
pub const CGameUI__7__m_simpleConsoleBottomBarFrame__OFFSET: usize = 2024;
pub const CGameUI__7__m_simpleConsoleBottomBarFrameOverlay__OFFSET: usize = 2032;
pub const CGameUI__7__m_frameUIParent__OFFSET: usize = 2040;
pub const CGameUI__7__top__OFFSET: usize = 2048;
pub const CGameUI__7__topInGame__OFFSET: usize = 2056;
pub const CGameUI__7__m_simpleUIParent__OFFSET: usize = 2064;
pub const CGameUI__7__m_observerUIParent__OFFSET: usize = 2072;
pub const CGameUI__7__m_observerInfoPanelBackground__OFFSET: usize = 2080;
pub const CGameUI__7__m_observerInfoPanelBackpack__OFFSET: usize = 2088;
pub const CGameUI__7__m_consoleBackdropFrame__OFFSET: usize = 2096;
pub const CGameUI__7__m_minimapTextureBackground__OFFSET: usize = 2104;
pub const CGameUI__7__m_infoPanelTexture__OFFSET: usize = 2112;
pub const CGameUI__7__m_quickSaveKey__OFFSET: usize = 2120;
pub const CGameUI__7__m_quickLoadKey__OFFSET: usize = 2124;
pub const CGameUI__7__m_quickHelpKey__OFFSET: usize = 2128;
pub const CGameUI__7__m_quickOptionsKey__OFFSET: usize = 2132;
pub const CGameUI__7__m_quickQuitKey__OFFSET: usize = 2136;
pub const CGameUI__7__m_minimapSignalKey__OFFSET: usize = 2140;
pub const CGameUI__7__m_minimapTerrainKey__OFFSET: usize = 2144;
pub const CGameUI__7__m_minimapColorsKey__OFFSET: usize = 2148;
pub const CGameUI__7__m_minimapCreepKey__OFFSET: usize = 2152;
pub const CGameUI__7__m_formationToggleKey__OFFSET: usize = 2156;
pub const CGameUI__7__m_observerToggleAllPanelsKey__OFFSET: usize = 2160;
pub const CGameUI__7__m_observerProductionModeKey__OFFSET: usize = 2164;
pub const CGameUI__7__m_observerUnitsModeKey__OFFSET: usize = 2168;
pub const CGameUI__7__m_mouseDownHandler__OFFSET: usize = 2176;
pub const CGameUI__7__m_mouseWheelHandler__OFFSET: usize = 2184;
pub const CGameUI__7__m_mouseUpHandler__OFFSET: usize = 2192;
pub const CGameUI__7__m_mouseMoveHandler__OFFSET: usize = 2200;
pub const CGameUI__7__m_observerToggleablePanels__OFFSET: usize = 2208;
pub const CGameUI__7__m_consoleBackdropHeight__OFFSET: usize = 2240;
pub const CGameUI__7__m_auroraChatHandler__OFFSET: usize = 2248;
pub const CGameUI__7__m_lobbyEventsHandler__OFFSET: usize = 2328;
pub const CGameUI__7__s_indexedCustomHotkeyMappingMaxCount__OFFSET: usize = 0;
pub const CGameUI__7__s_customHotkeyEnabled__OFFSET: usize = 0;
pub const CGameUI__7__s_customHotkeyMappingTable__OFFSET: usize = 0;
pub const CGameUI__7__s_customHotkeyMappingTableNonHero__OFFSET: usize = 0;
pub const CGameUI__7__m_observerUnitToFollow__OFFSET: usize = 2376;

#[repr(C)]
pub struct CSelectionManager {
    pub _opaque: [u8; 584],
}
pub const CSelectionManager__SIZE: usize = 584;
pub const CSelectionManager__NAME: &str = "CSelectionManager";
pub const CSelectionManager__m_worldFrame__OFFSET: usize = 24;
pub const CSelectionManager__m_doubleClickTimeout__OFFSET: usize = 32;
pub const CSelectionManager__m_doubleClickUnit__OFFSET: usize = 40;
pub const CSelectionManager__m_doubleClickStartTime__OFFSET: usize = 48;
pub const CSelectionManager__m_doubleClick__OFFSET: usize = 52;
pub const CSelectionManager__m_do3dSelect__OFFSET: usize = 53;
pub const CSelectionManager__m_dragSelectedUnits__OFFSET: usize = 56;
pub const CSelectionManager__m_dragAlreadySelectedUnits__OFFSET: usize = 80;
pub const CSelectionManager__m_dragSelectedSelectables__OFFSET: usize = 104;
pub const CSelectionManager__m_startScreen__OFFSET: usize = 128;
pub const CSelectionManager__m_vertCount__OFFSET: usize = 136;
pub const CSelectionManager__m_verts__OFFSET: usize = 144;
pub const CSelectionManager__m_indices__OFFSET: usize = 168;
pub const CSelectionManager__m_material__OFFSET: usize = 192;
pub const CSelectionManager__m_start__OFFSET: usize = 464;
pub const CSelectionManager__m_selecting__OFFSET: usize = 476;
pub const CSelectionManager__m_update__OFFSET: usize = 477;
pub const CSelectionManager__m_dragRect__OFFSET: usize = 480;
pub const CSelectionManager__m_selectorRect__OFFSET: usize = 496;
pub const CSelectionManager__m_pointsPerSide__OFFSET: usize = 544;
pub const CSelectionManager__m_zOffset__OFFSET: usize = 548;
pub const CSelectionManager__m_samplePoints__OFFSET: usize = 552;
pub const CSelectionManager__m_samplePointCount__OFFSET: usize = 576;
pub const CSelectionManager__m_selCircColorFriend__OFFSET: usize = 580;

#[repr(C)]
pub struct CPlayerUnitPointOrderEventData {
    pub _opaque: [u8; 120],
}
pub const CPlayerUnitPointOrderEventData__SIZE: usize = 120;
pub const CPlayerUnitPointOrderEventData__NAME: &str = "CPlayerUnitPointOrderEventData";
pub const CPlayerUnitPointOrderEventData__s_CPlayerUnitPointOrderEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CWorldFrameWar3__3 {
    pub _opaque: [u8; 3128],
}
pub const CWorldFrameWar3__3__SIZE: usize = 3128;
pub const CWorldFrameWar3__3__NAME: &str = "CWorldFrameWar3";
pub const CWorldFrameWar3__3__m_trackUpdating__OFFSET: usize = 656;
pub const CWorldFrameWar3__3__m_sprites__OFFSET: usize = 664;
pub const CWorldFrameWar3__3__m_targetedSprites__OFFSET: usize = 672;
pub const CWorldFrameWar3__3__m_walkableSprites__OFFSET: usize = 680;
pub const CWorldFrameWar3__3__m_pGame__OFFSET: usize = 688;
pub const CWorldFrameWar3__3__m_pDoHpBarsEnemies__OFFSET: usize = 696;
pub const CWorldFrameWar3__3__m_pDoHpBarsAllies__OFFSET: usize = 697;
pub const CWorldFrameWar3__3__m_pLocalPlayer__OFFSET: usize = 704;
pub const CWorldFrameWar3__3__m_pFog__OFFSET: usize = 712;
pub const CWorldFrameWar3__3__m_cachedCameraPos__OFFSET: usize = 720;
pub const CWorldFrameWar3__3__m_localPlayerId__OFFSET: usize = 732;
pub const CWorldFrameWar3__3__m_pCameraWar3__OFFSET: usize = 736;
pub const CWorldFrameWar3__3__m_pMinimap__OFFSET: usize = 744;
pub const CWorldFrameWar3__3__m_spriteButtons__OFFSET: usize = 752;
pub const CWorldFrameWar3__3__m_terrainButtons__OFFSET: usize = 756;
pub const CWorldFrameWar3__3__m_cursorMode__OFFSET: usize = 760;
pub const CWorldFrameWar3__3__anticheat_dummy_array_30__OFFSET: usize = 764;
pub const CWorldFrameWar3__3__m_cursorModeStack__OFFSET: usize = 808;
pub const CWorldFrameWar3__3__m_cursorModeLocked__OFFSET: usize = 832;
pub const CWorldFrameWar3__3__m_forceCursorUpdate__OFFSET: usize = 833;
pub const CWorldFrameWar3__3__m_cursorArt__OFFSET: usize = 840;
pub const CWorldFrameWar3__3__m_aspectScaledProjection__OFFSET: usize = 864;
pub const CWorldFrameWar3__3__m_fovScale__OFFSET: usize = 880;
pub const CWorldFrameWar3__3__m_frustumBounds__OFFSET: usize = 884;
pub const CWorldFrameWar3__3__m_frustumBoundsInt__OFFSET: usize = 900;
pub const CWorldFrameWar3__3__m_localPlayerIdBit__OFFSET: usize = 916;
pub const CWorldFrameWar3__3__m_playerIdBitsToGhostMasks__OFFSET: usize = 920;
pub const CWorldFrameWar3__3__m_playerIdBitsToInvisMasks__OFFSET: usize = 1020;
pub const CWorldFrameWar3__3__m_localPlayerIsObserver__OFFSET: usize = 1120;
pub const CWorldFrameWar3__3__m_isReplayFogDisabled__OFFSET: usize = 1121;
pub const CWorldFrameWar3__3__m_lastUpdateElapsedSec__OFFSET: usize = 1124;
pub const CWorldFrameWar3__3__m_lastAsyncElapsedSec__OFFSET: usize = 1128;
pub const CWorldFrameWar3__3__m_lastTrackSprite__OFFSET: usize = 1136;
pub const CWorldFrameWar3__3__m_lastTrackAgent__OFFSET: usize = 1144;
pub const CWorldFrameWar3__3__m_renderPlacement__OFFSET: usize = 1152;
pub const CWorldFrameWar3__3__m_buildingPlacementRender__OFFSET: usize = 1153;
pub const CWorldFrameWar3__3__m_pBuildFrame__OFFSET: usize = 1160;
pub const CWorldFrameWar3__3__m_cineFilter__OFFSET: usize = 1168;
pub const CWorldFrameWar3__3__m_targetingSprite__OFFSET: usize = 1696;
pub const CWorldFrameWar3__3__m_renderTargetSprite__OFFSET: usize = 1704;
pub const CWorldFrameWar3__3__m_targetingImage__OFFSET: usize = 1708;
pub const CWorldFrameWar3__3__m_targetingRadius__OFFSET: usize = 1712;
pub const CWorldFrameWar3__3__m_targetingSqMag__OFFSET: usize = 1716;
pub const CWorldFrameWar3__3__m_targetImageColor__OFFSET: usize = 1720;
pub const CWorldFrameWar3__3__m_currentWorldPos__OFFSET: usize = 1724;
pub const CWorldFrameWar3__3__m_suspended__OFFSET: usize = 1736;
pub const CWorldFrameWar3__3__m_noHitTests__OFFSET: usize = 1737;
pub const CWorldFrameWar3__3__m_doOcclusion__OFFSET: usize = 1738;
pub const CWorldFrameWar3__3__m_mouseMoveProcessed__OFFSET: usize = 1739;
pub const CWorldFrameWar3__3__m_doFogOfWar__OFFSET: usize = 1740;
pub const CWorldFrameWar3__3__anticheat_dummy_array_31__OFFSET: usize = 1741;
pub const CWorldFrameWar3__3__m_useDarkMask__OFFSET: usize = 1744;
pub const CWorldFrameWar3__3__m_renderUnits__OFFSET: usize = 1745;
pub const CWorldFrameWar3__3__m_terrainFog__OFFSET: usize = 1752;
pub const CWorldFrameWar3__3__m_terrainLight__OFFSET: usize = 1760;
pub const CWorldFrameWar3__3__m_unitLight__OFFSET: usize = 1768;
pub const CWorldFrameWar3__3__m_suspendedAmbientLight__OFFSET: usize = 1776;
pub const CWorldFrameWar3__3__m_targetLight__OFFSET: usize = 1784;
pub const CWorldFrameWar3__3__m_skyModelPath__OFFSET: usize = 1792;
pub const CWorldFrameWar3__3__m_sky__OFFSET: usize = 1816;
pub const CWorldFrameWar3__3__m_showSky__OFFSET: usize = 1824;
pub const CWorldFrameWar3__3__m_renderSky__OFFSET: usize = 1825;
pub const CWorldFrameWar3__3__m_indicators__OFFSET: usize = 1832;
pub const CWorldFrameWar3__3__m_targetIndicators__OFFSET: usize = 1856;
pub const CWorldFrameWar3__3__m_targetIndicatorIndex__OFFSET: usize = 1880;
pub const CWorldFrameWar3__3__m_enumElapsed__OFFSET: usize = 1884;
pub const CWorldFrameWar3__3__m_asyncElapsed__OFFSET: usize = 1888;
pub const CWorldFrameWar3__3__m_doEnumUpdate__OFFSET: usize = 1892;
pub const CWorldFrameWar3__3__m_doFogEnum__OFFSET: usize = 1893;
pub const CWorldFrameWar3__3__m_trackingElapsed__OFFSET: usize = 1896;
pub const CWorldFrameWar3__3__m_terrainMinZ__OFFSET: usize = 1900;
pub const CWorldFrameWar3__3__m_scaledAnimTime__OFFSET: usize = 1904;
pub const CWorldFrameWar3__3__m_hoursPerDay__OFFSET: usize = 1908;
pub const CWorldFrameWar3__3__m_selCircColorFriend__OFFSET: usize = 1912;
pub const CWorldFrameWar3__3__anticheat_dummy_array_32__OFFSET: usize = 1916;
pub const CWorldFrameWar3__3__m_selCircColorNeutral__OFFSET: usize = 1921;
pub const CWorldFrameWar3__3__m_selCircColorEnemy__OFFSET: usize = 1925;
pub const CWorldFrameWar3__3__m_enableTargetIndicator__OFFSET: usize = 1929;
pub const CWorldFrameWar3__3__m_rallyFlags__OFFSET: usize = 1932;
pub const CWorldFrameWar3__3__m_rallyDst__OFFSET: usize = 1936;
pub const CWorldFrameWar3__3__m_rallyDstReuse__OFFSET: usize = 2708;
pub const CWorldFrameWar3__3__m_rallySrc__OFFSET: usize = 2776;
pub const CWorldFrameWar3__3__m_indicatorTerrain__OFFSET: usize = 2784;
pub const CWorldFrameWar3__3__m_waypointIndicators__OFFSET: usize = 2792;
pub const CWorldFrameWar3__3__m_waypointIndicatorIndex__OFFSET: usize = 2816;
pub const CWorldFrameWar3__3__m_unitUpdateQueue__OFFSET: usize = 2824;
pub const CWorldFrameWar3__3__m_destUpdateQueue__OFFSET: usize = 2848;
pub const CWorldFrameWar3__3__m_itemUpdateQueue__OFFSET: usize = 2872;
pub const CWorldFrameWar3__3__m_captUpdateQueue__OFFSET: usize = 2896;
pub const CWorldFrameWar3__3__m_imagUpdateQueue__OFFSET: usize = 2920;
pub const CWorldFrameWar3__3__m_ghstUpdateQueue__OFFSET: usize = 2944;
pub const CWorldFrameWar3__3__anticheat_dummy_array_33__OFFSET: usize = 2968;
pub const CWorldFrameWar3__3__m_enumeratedUnits__OFFSET: usize = 2976;
pub const CWorldFrameWar3__3__m_enumeratedItems__OFFSET: usize = 3000;
pub const CWorldFrameWar3__3__m_visibleUnits__OFFSET: usize = 3024;
pub const CWorldFrameWar3__3__m_visibleSelectables__OFFSET: usize = 3048;
pub const CWorldFrameWar3__3__m_fogUpdateList__OFFSET: usize = 3072;
pub const CWorldFrameWar3__3__m_ghostImages__OFFSET: usize = 3096;
pub const CWorldFrameWar3__3__m_lastRenderLight__OFFSET: usize = 3120;
pub const CWorldFrameWar3__3__m_lastRenderFog__OFFSET: usize = 3124;
pub const CWorldFrameWar3__3__enableDSQDenialMessage__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerUnitTargetOrderEventData {
    pub _opaque: [u8; 120],
}
pub const CPlayerUnitTargetOrderEventData__SIZE: usize = 120;
pub const CPlayerUnitTargetOrderEventData__NAME: &str = "CPlayerUnitTargetOrderEventData";
pub const CPlayerUnitTargetOrderEventData__s_CPlayerUnitTargetOrderEventData_pool__OFFSET: usize = 0;

#[repr(C)]
pub struct CGameUI__8 {
    pub _opaque: [u8; 2384],
}
pub const CGameUI__8__SIZE: usize = 2384;
pub const CGameUI__8__NAME: &str = "CGameUI";
pub const CGameUI__8__m_transitionState__OFFSET: usize = 704;
pub const CGameUI__8__m_cachedFonts__OFFSET: usize = 712;
pub const CGameUI__8__NUMBER_OF_PLAYER_ROWS__OFFSET: usize = 0;
pub const CGameUI__8__FIRST_ROW_Y_OFFSET__OFFSET: usize = 0;
pub const CGameUI__8__ROW_SPACING__OFFSET: usize = 0;
pub const CGameUI__8__m_userControl__OFFSET: usize = 784;
pub const CGameUI__8__m_userUI__OFFSET: usize = 785;
pub const CGameUI__8__m_currentInputMode__OFFSET: usize = 792;
pub const CGameUI__8__m_inputModeStack__OFFSET: usize = 800;
pub const CGameUI__8__m_sharedTooltip__OFFSET: usize = 824;
pub const CGameUI__8__m_uberTooltip__OFFSET: usize = 832;
pub const CGameUI__8__m_showUberTips__OFFSET: usize = 840;
pub const CGameUI__8__m_autosaveReplay__OFFSET: usize = 841;
pub const CGameUI__8__m_healthBarsMode__OFFSET: usize = 844;
pub const CGameUI__8__m_forceShowHealthBars__OFFSET: usize = 848;
pub const CGameUI__8__m_minimapSignals__OFFSET: usize = 849;
pub const CGameUI__8__m_useColoredHealthBars__OFFSET: usize = 850;
pub const CGameUI__8__m_useHeroFrames__OFFSET: usize = 851;
pub const CGameUI__8__m_showHeroLevelFrame__OFFSET: usize = 852;
pub const CGameUI__8__m_showNumericCooldown__OFFSET: usize = 853;
pub const CGameUI__8__m_showDenyIcon__OFFSET: usize = 854;
pub const CGameUI__8__m_showGoldmineUnitCounter__OFFSET: usize = 855;
pub const CGameUI__8__m_enableObserverUiV2__OFFSET: usize = 856;
pub const CGameUI__8__m_asyncPlayerData__OFFSET: usize = 860;
pub const CGameUI__8__m_targetMode__OFFSET: usize = 960;
pub const CGameUI__8__m_selectMode__OFFSET: usize = 968;
pub const CGameUI__8__m_dragSelectMode__OFFSET: usize = 976;
pub const CGameUI__8__m_dragScrollMode__OFFSET: usize = 984;
pub const CGameUI__8__m_buildMode__OFFSET: usize = 992;
pub const CGameUI__8__m_signalMode__OFFSET: usize = 1000;
pub const CGameUI__8__m_escapeMenu__OFFSET: usize = 1008;
pub const CGameUI__8__m_gameOverMenu__OFFSET: usize = 1016;
pub const CGameUI__8__m_suspendIndicator__OFFSET: usize = 1024;
pub const CGameUI__8__m_unresponsiveIndicator__OFFSET: usize = 1032;
pub const CGameUI__8__m_allianceMode__OFFSET: usize = 1040;
pub const CGameUI__8__m_chatMode__OFFSET: usize = 1048;
pub const CGameUI__8__m_questMode__OFFSET: usize = 1056;
pub const CGameUI__8__anticheat_dummy_array_21__OFFSET: usize = 1064;
pub const CGameUI__8__m_scriptDialogMode__OFFSET: usize = 1152;
pub const CGameUI__8__m_unresponsiveDelayTimerId__OFFSET: usize = 1160;
pub const CGameUI__8__m_selectionManager__OFFSET: usize = 1168;
pub const CGameUI__8__m_dragScrollManager__OFFSET: usize = 1176;
pub const CGameUI__8__m_camera__OFFSET: usize = 1184;
pub const CGameUI__8__m_paused__OFFSET: usize = 1192;
pub const CGameUI__8__m_uiPaused__OFFSET: usize = 1193;
pub const CGameUI__8__m_scriptPaused__OFFSET: usize = 1194;
pub const CGameUI__8__m_disableUpkeepMsg__OFFSET: usize = 1195;
pub const CGameUI__8__m_upkeepLvlSave__OFFSET: usize = 1196;
pub const CGameUI__8__m_refresh__OFFSET: usize = 1200;
pub const CGameUI__8__m_refreshWaypoints__OFFSET: usize = 1201;
pub const CGameUI__8__m_deferCleanSelUI__OFFSET: usize = 1202;
pub const CGameUI__8__m_refreshInfoBar__OFFSET: usize = 1203;
pub const CGameUI__8__m_refreshCommandBar__OFFSET: usize = 1204;
pub const CGameUI__8__m_refreshHeroBar__OFFSET: usize = 1205;
pub const CGameUI__8__m_popSubMenu__OFFSET: usize = 1206;
pub const CGameUI__8__m_activeValidOrder__OFFSET: usize = 1208;
pub const CGameUI__8__m_cineForceSubtitles__OFFSET: usize = 1212;
pub const CGameUI__8__m_cinematicPanelsHidden__OFFSET: usize = 1213;
pub const CGameUI__8__m_dragSelectEnabled__OFFSET: usize = 1214;
pub const CGameUI__8__m_dragSelectUIEnabled__OFFSET: usize = 1215;
pub const CGameUI__8__m_preSelectEnabled__OFFSET: usize = 1216;
pub const CGameUI__8__m_preSelectUIEnabled__OFFSET: usize = 1217;
pub const CGameUI__8__m_selectEnabled__OFFSET: usize = 1218;
pub const CGameUI__8__m_selectUIEnabled__OFFSET: usize = 1219;
pub const CGameUI__8__m_observerModeEnabled__OFFSET: usize = 1220;
pub const CGameUI__8__m_observerModeV2Active__OFFSET: usize = 1221;
pub const CGameUI__8__m_wideScreenAutoPosition__OFFSET: usize = 1222;
pub const CGameUI__8__m_fadeTimer__OFFSET: usize = 1224;
pub const CGameUI__8__m_cinematicFadeTimer__OFFSET: usize = 1328;
pub const CGameUI__8__anticheat_dummy_array_22__OFFSET: usize = 1432;
pub const CGameUI__8__m_frameParentTimer__OFFSET: usize = 1440;
pub const CGameUI__8__m_simpleFadeTimer__OFFSET: usize = 1544;
pub const CGameUI__8__m_attackNotify__OFFSET: usize = 1552;
pub const CGameUI__8__m_warpNotify__OFFSET: usize = 1576;
pub const CGameUI__8__m_timerDialogs__OFFSET: usize = 1608;
pub const CGameUI__8__m_leaderboard__OFFSET: usize = 1632;
pub const CGameUI__8__anticheat_dummy_array_23__OFFSET: usize = 1640;
pub const CGameUI__8__m_multiboard__OFFSET: usize = 1648;
pub const CGameUI__8__m_suspendMultiboardDisplay__OFFSET: usize = 1656;
pub const CGameUI__8__anticheat_dummy_array_24__OFFSET: usize = 1657;
pub const CGameUI__8__m_scriptDialog__OFFSET: usize = 1728;
pub const CGameUI__8__m_replaySelection__OFFSET: usize = 1736;
pub const CGameUI__8__m_unitStatusIconUIFactory__OFFSET: usize = 1744;
pub const CGameUI__8__m_unitStatusTexts__OFFSET: usize = 1752;
pub const CGameUI__8__m_worldFrame__OFFSET: usize = 1776;
pub const CGameUI__8__m_infoBar__OFFSET: usize = 1784;
pub const CGameUI__8__m_commandBar__OFFSET: usize = 1792;
pub const CGameUI__8__m_miniMap__OFFSET: usize = 1800;
pub const CGameUI__8__m_resourceBar__OFFSET: usize = 1808;
pub const CGameUI__8__m_upperButtonBar__OFFSET: usize = 1816;
pub const CGameUI__8__m_observerButtonBar__OFFSET: usize = 1824;
pub const CGameUI__8__m_playerStatisticsPanel__OFFSET: usize = 1832;
pub const CGameUI__8__m_heroPanel__OFFSET: usize = 1840;
pub const CGameUI__8__m_productionPanel__OFFSET: usize = 1848;
pub const CGameUI__8__m_minimapButtonBar__OFFSET: usize = 1856;
pub const CGameUI__8__m_trackingStub__OFFSET: usize = 1864;
pub const CGameUI__8__m_heroBar__OFFSET: usize = 1872;
pub const CGameUI__8__m_peonBar__OFFSET: usize = 1880;
pub const CGameUI__8__m_messageBox__OFFSET: usize = 1888;
pub const CGameUI__8__m_unitMessageBox__OFFSET: usize = 1896;
pub const CGameUI__8__m_chatMessageBox__OFFSET: usize = 1904;
pub const CGameUI__8__m_topMessageBox__OFFSET: usize = 1912;
pub const CGameUI__8__m_currentPortraitButton__OFFSET: usize = 1920;
pub const CGameUI__8__m_timeOfDayIndicator__OFFSET: usize = 1928;
pub const CGameUI__8__m_chatEditBar__OFFSET: usize = 1936;
pub const CGameUI__8__m_cinematicPanel__OFFSET: usize = 1944;
pub const CGameUI__8__m_lastBuffBar__OFFSET: usize = 1952;
pub const CGameUI__8__m_observerBar__OFFSET: usize = 1960;
pub const CGameUI__8__m_miniMapSignalButton__OFFSET: usize = 1968;
pub const CGameUI__8__m_miniMapTerrainButton__OFFSET: usize = 1976;
pub const CGameUI__8__m_miniMapAllyButton__OFFSET: usize = 1984;
pub const CGameUI__8__m_miniMapCreepButton__OFFSET: usize = 1992;
pub const CGameUI__8__m_formationButton__OFFSET: usize = 2000;
pub const CGameUI__8__m_consoleFrame__OFFSET: usize = 2008;
pub const CGameUI__8__m_simpleConsoleFrame__OFFSET: usize = 2016;
pub const CGameUI__8__m_simpleConsoleBottomBarFrame__OFFSET: usize = 2024;
pub const CGameUI__8__m_simpleConsoleBottomBarFrameOverlay__OFFSET: usize = 2032;
pub const CGameUI__8__m_frameUIParent__OFFSET: usize = 2040;
pub const CGameUI__8__top__OFFSET: usize = 2048;
pub const CGameUI__8__topInGame__OFFSET: usize = 2056;
pub const CGameUI__8__m_simpleUIParent__OFFSET: usize = 2064;
pub const CGameUI__8__m_observerUIParent__OFFSET: usize = 2072;
pub const CGameUI__8__m_observerInfoPanelBackground__OFFSET: usize = 2080;
pub const CGameUI__8__m_observerInfoPanelBackpack__OFFSET: usize = 2088;
pub const CGameUI__8__m_consoleBackdropFrame__OFFSET: usize = 2096;
pub const CGameUI__8__m_minimapTextureBackground__OFFSET: usize = 2104;
pub const CGameUI__8__m_infoPanelTexture__OFFSET: usize = 2112;
pub const CGameUI__8__m_quickSaveKey__OFFSET: usize = 2120;
pub const CGameUI__8__m_quickLoadKey__OFFSET: usize = 2124;
pub const CGameUI__8__m_quickHelpKey__OFFSET: usize = 2128;
pub const CGameUI__8__m_quickOptionsKey__OFFSET: usize = 2132;
pub const CGameUI__8__m_quickQuitKey__OFFSET: usize = 2136;
pub const CGameUI__8__m_minimapSignalKey__OFFSET: usize = 2140;
pub const CGameUI__8__m_minimapTerrainKey__OFFSET: usize = 2144;
pub const CGameUI__8__m_minimapColorsKey__OFFSET: usize = 2148;
pub const CGameUI__8__m_minimapCreepKey__OFFSET: usize = 2152;
pub const CGameUI__8__m_formationToggleKey__OFFSET: usize = 2156;
pub const CGameUI__8__m_observerToggleAllPanelsKey__OFFSET: usize = 2160;
pub const CGameUI__8__m_observerProductionModeKey__OFFSET: usize = 2164;
pub const CGameUI__8__m_observerUnitsModeKey__OFFSET: usize = 2168;
pub const CGameUI__8__m_mouseDownHandler__OFFSET: usize = 2176;
pub const CGameUI__8__m_mouseWheelHandler__OFFSET: usize = 2184;
pub const CGameUI__8__m_mouseUpHandler__OFFSET: usize = 2192;
pub const CGameUI__8__m_mouseMoveHandler__OFFSET: usize = 2200;
pub const CGameUI__8__m_observerToggleablePanels__OFFSET: usize = 2208;
pub const CGameUI__8__m_consoleBackdropHeight__OFFSET: usize = 2240;
pub const CGameUI__8__m_auroraChatHandler__OFFSET: usize = 2248;
pub const CGameUI__8__m_lobbyEventsHandler__OFFSET: usize = 2328;
pub const CGameUI__8__s_indexedCustomHotkeyMappingMaxCount__OFFSET: usize = 0;
pub const CGameUI__8__s_customHotkeyEnabled__OFFSET: usize = 0;
pub const CGameUI__8__s_customHotkeyMappingTable__OFFSET: usize = 0;
pub const CGameUI__8__s_customHotkeyMappingTableNonHero__OFFSET: usize = 0;
pub const CGameUI__8__m_observerUnitToFollow__OFFSET: usize = 2376;

#[repr(C)]
pub struct CWorldFrameWar3__4 {
    pub _opaque: [u8; 3128],
}
pub const CWorldFrameWar3__4__SIZE: usize = 3128;
pub const CWorldFrameWar3__4__NAME: &str = "CWorldFrameWar3";
pub const CWorldFrameWar3__4__m_trackUpdating__OFFSET: usize = 656;
pub const CWorldFrameWar3__4__m_sprites__OFFSET: usize = 664;
pub const CWorldFrameWar3__4__m_targetedSprites__OFFSET: usize = 672;
pub const CWorldFrameWar3__4__m_walkableSprites__OFFSET: usize = 680;
pub const CWorldFrameWar3__4__m_pGame__OFFSET: usize = 688;
pub const CWorldFrameWar3__4__m_pDoHpBarsEnemies__OFFSET: usize = 696;
pub const CWorldFrameWar3__4__m_pDoHpBarsAllies__OFFSET: usize = 697;
pub const CWorldFrameWar3__4__m_pLocalPlayer__OFFSET: usize = 704;
pub const CWorldFrameWar3__4__m_pFog__OFFSET: usize = 712;
pub const CWorldFrameWar3__4__m_cachedCameraPos__OFFSET: usize = 720;
pub const CWorldFrameWar3__4__m_localPlayerId__OFFSET: usize = 732;
pub const CWorldFrameWar3__4__m_pCameraWar3__OFFSET: usize = 736;
pub const CWorldFrameWar3__4__m_pMinimap__OFFSET: usize = 744;
pub const CWorldFrameWar3__4__m_spriteButtons__OFFSET: usize = 752;
pub const CWorldFrameWar3__4__m_terrainButtons__OFFSET: usize = 756;
pub const CWorldFrameWar3__4__m_cursorMode__OFFSET: usize = 760;
pub const CWorldFrameWar3__4__anticheat_dummy_array_26__OFFSET: usize = 764;
pub const CWorldFrameWar3__4__m_cursorModeStack__OFFSET: usize = 808;
pub const CWorldFrameWar3__4__m_cursorModeLocked__OFFSET: usize = 832;
pub const CWorldFrameWar3__4__m_forceCursorUpdate__OFFSET: usize = 833;
pub const CWorldFrameWar3__4__m_cursorArt__OFFSET: usize = 840;
pub const CWorldFrameWar3__4__m_aspectScaledProjection__OFFSET: usize = 864;
pub const CWorldFrameWar3__4__m_fovScale__OFFSET: usize = 880;
pub const CWorldFrameWar3__4__m_frustumBounds__OFFSET: usize = 884;
pub const CWorldFrameWar3__4__m_frustumBoundsInt__OFFSET: usize = 900;
pub const CWorldFrameWar3__4__m_localPlayerIdBit__OFFSET: usize = 916;
pub const CWorldFrameWar3__4__m_playerIdBitsToGhostMasks__OFFSET: usize = 920;
pub const CWorldFrameWar3__4__m_playerIdBitsToInvisMasks__OFFSET: usize = 1020;
pub const CWorldFrameWar3__4__m_localPlayerIsObserver__OFFSET: usize = 1120;
pub const CWorldFrameWar3__4__m_isReplayFogDisabled__OFFSET: usize = 1121;
pub const CWorldFrameWar3__4__m_lastUpdateElapsedSec__OFFSET: usize = 1124;
pub const CWorldFrameWar3__4__m_lastAsyncElapsedSec__OFFSET: usize = 1128;
pub const CWorldFrameWar3__4__m_lastTrackSprite__OFFSET: usize = 1136;
pub const CWorldFrameWar3__4__m_lastTrackAgent__OFFSET: usize = 1144;
pub const CWorldFrameWar3__4__m_renderPlacement__OFFSET: usize = 1152;
pub const CWorldFrameWar3__4__m_buildingPlacementRender__OFFSET: usize = 1153;
pub const CWorldFrameWar3__4__m_pBuildFrame__OFFSET: usize = 1160;
pub const CWorldFrameWar3__4__m_cineFilter__OFFSET: usize = 1168;
pub const CWorldFrameWar3__4__m_targetingSprite__OFFSET: usize = 1696;
pub const CWorldFrameWar3__4__m_renderTargetSprite__OFFSET: usize = 1704;
pub const CWorldFrameWar3__4__m_targetingImage__OFFSET: usize = 1708;
pub const CWorldFrameWar3__4__m_targetingRadius__OFFSET: usize = 1712;
pub const CWorldFrameWar3__4__m_targetingSqMag__OFFSET: usize = 1716;
pub const CWorldFrameWar3__4__m_targetImageColor__OFFSET: usize = 1720;
pub const CWorldFrameWar3__4__m_currentWorldPos__OFFSET: usize = 1724;
pub const CWorldFrameWar3__4__m_suspended__OFFSET: usize = 1736;
pub const CWorldFrameWar3__4__m_noHitTests__OFFSET: usize = 1737;
pub const CWorldFrameWar3__4__m_doOcclusion__OFFSET: usize = 1738;
pub const CWorldFrameWar3__4__m_mouseMoveProcessed__OFFSET: usize = 1739;
pub const CWorldFrameWar3__4__m_doFogOfWar__OFFSET: usize = 1740;
pub const CWorldFrameWar3__4__anticheat_dummy_array_27__OFFSET: usize = 1741;
pub const CWorldFrameWar3__4__m_useDarkMask__OFFSET: usize = 1744;
pub const CWorldFrameWar3__4__m_renderUnits__OFFSET: usize = 1745;
pub const CWorldFrameWar3__4__m_terrainFog__OFFSET: usize = 1752;
pub const CWorldFrameWar3__4__m_terrainLight__OFFSET: usize = 1760;
pub const CWorldFrameWar3__4__m_unitLight__OFFSET: usize = 1768;
pub const CWorldFrameWar3__4__m_suspendedAmbientLight__OFFSET: usize = 1776;
pub const CWorldFrameWar3__4__m_targetLight__OFFSET: usize = 1784;
pub const CWorldFrameWar3__4__m_skyModelPath__OFFSET: usize = 1792;
pub const CWorldFrameWar3__4__m_sky__OFFSET: usize = 1816;
pub const CWorldFrameWar3__4__m_showSky__OFFSET: usize = 1824;
pub const CWorldFrameWar3__4__m_renderSky__OFFSET: usize = 1825;
pub const CWorldFrameWar3__4__m_indicators__OFFSET: usize = 1832;
pub const CWorldFrameWar3__4__m_targetIndicators__OFFSET: usize = 1856;
pub const CWorldFrameWar3__4__m_targetIndicatorIndex__OFFSET: usize = 1880;
pub const CWorldFrameWar3__4__m_enumElapsed__OFFSET: usize = 1884;
pub const CWorldFrameWar3__4__m_asyncElapsed__OFFSET: usize = 1888;
pub const CWorldFrameWar3__4__m_doEnumUpdate__OFFSET: usize = 1892;
pub const CWorldFrameWar3__4__m_doFogEnum__OFFSET: usize = 1893;
pub const CWorldFrameWar3__4__m_trackingElapsed__OFFSET: usize = 1896;
pub const CWorldFrameWar3__4__m_terrainMinZ__OFFSET: usize = 1900;
pub const CWorldFrameWar3__4__m_scaledAnimTime__OFFSET: usize = 1904;
pub const CWorldFrameWar3__4__m_hoursPerDay__OFFSET: usize = 1908;
pub const CWorldFrameWar3__4__m_selCircColorFriend__OFFSET: usize = 1912;
pub const CWorldFrameWar3__4__anticheat_dummy_array_28__OFFSET: usize = 1916;
pub const CWorldFrameWar3__4__m_selCircColorNeutral__OFFSET: usize = 1921;
pub const CWorldFrameWar3__4__m_selCircColorEnemy__OFFSET: usize = 1925;
pub const CWorldFrameWar3__4__m_enableTargetIndicator__OFFSET: usize = 1929;
pub const CWorldFrameWar3__4__m_rallyFlags__OFFSET: usize = 1932;
pub const CWorldFrameWar3__4__m_rallyDst__OFFSET: usize = 1936;
pub const CWorldFrameWar3__4__m_rallyDstReuse__OFFSET: usize = 2708;
pub const CWorldFrameWar3__4__m_rallySrc__OFFSET: usize = 2776;
pub const CWorldFrameWar3__4__m_indicatorTerrain__OFFSET: usize = 2784;
pub const CWorldFrameWar3__4__m_waypointIndicators__OFFSET: usize = 2792;
pub const CWorldFrameWar3__4__m_waypointIndicatorIndex__OFFSET: usize = 2816;
pub const CWorldFrameWar3__4__m_unitUpdateQueue__OFFSET: usize = 2824;
pub const CWorldFrameWar3__4__m_destUpdateQueue__OFFSET: usize = 2848;
pub const CWorldFrameWar3__4__m_itemUpdateQueue__OFFSET: usize = 2872;
pub const CWorldFrameWar3__4__m_captUpdateQueue__OFFSET: usize = 2896;
pub const CWorldFrameWar3__4__m_imagUpdateQueue__OFFSET: usize = 2920;
pub const CWorldFrameWar3__4__m_ghstUpdateQueue__OFFSET: usize = 2944;
pub const CWorldFrameWar3__4__anticheat_dummy_array_29__OFFSET: usize = 2968;
pub const CWorldFrameWar3__4__m_enumeratedUnits__OFFSET: usize = 2976;
pub const CWorldFrameWar3__4__m_enumeratedItems__OFFSET: usize = 3000;
pub const CWorldFrameWar3__4__m_visibleUnits__OFFSET: usize = 3024;
pub const CWorldFrameWar3__4__m_visibleSelectables__OFFSET: usize = 3048;
pub const CWorldFrameWar3__4__m_fogUpdateList__OFFSET: usize = 3072;
pub const CWorldFrameWar3__4__m_ghostImages__OFFSET: usize = 3096;
pub const CWorldFrameWar3__4__m_lastRenderLight__OFFSET: usize = 3120;
pub const CWorldFrameWar3__4__m_lastRenderFog__OFFSET: usize = 3124;
pub const CWorldFrameWar3__4__enableDSQDenialMessage__OFFSET: usize = 0;

#[repr(C)]
pub struct CGameWar3EventHandler {
    pub _opaque: [u8; 352],
}
pub const CGameWar3EventHandler__SIZE: usize = 352;
pub const CGameWar3EventHandler__NAME: &str = "CGameWar3EventHandler";
pub const CGameWar3EventHandler__m_localPlayerName__OFFSET: usize = 24;
pub const CGameWar3EventHandler__m_levelToTransitionTo__OFFSET: usize = 48;
pub const CGameWar3EventHandler__m_oldGameSpeed__OFFSET: usize = 72;
pub const CGameWar3EventHandler__m_oldMapFlags__OFFSET: usize = 76;
pub const CGameWar3EventHandler__m_oldGameType__OFFSET: usize = 80;
pub const CGameWar3EventHandler__m_doScoreScreen__OFFSET: usize = 84;
pub const CGameWar3EventHandler__m_isLoadGame__OFFSET: usize = 85;
pub const CGameWar3EventHandler__m_isScriptLoad__OFFSET: usize = 86;
pub const CGameWar3EventHandler__m_isReload__OFFSET: usize = 87;
pub const CGameWar3EventHandler__m_gameSetup__OFFSET: usize = 88;

#[repr(C)]
pub struct CTimerExpireEventData {
    pub _opaque: [u8; 64],
}
pub const CTimerExpireEventData__SIZE: usize = 64;
pub const CTimerExpireEventData__NAME: &str = "CTimerExpireEventData";
pub const CTimerExpireEventData__s_CTimerExpireEventData_pool__OFFSET: usize = 0;
pub const CTimerExpireEventData__m_timer__OFFSET: usize = 56;

#[repr(C)]
pub struct CUnitPool {
    pub _opaque: [u8; 80],
}
pub const CUnitPool__SIZE: usize = 80;
pub const CUnitPool__NAME: &str = "CUnitPool";
pub const CUnitPool__s_CUnitPool_pool__OFFSET: usize = 0;
pub const CUnitPool__m_pool__OFFSET: usize = 56;

#[repr(C)]
pub struct CGameSaveEventData {
    pub _opaque: [u8; 80],
}
pub const CGameSaveEventData__SIZE: usize = 80;
pub const CGameSaveEventData__NAME: &str = "CGameSaveEventData";
pub const CGameSaveEventData__s_CGameSaveEventData_pool__OFFSET: usize = 0;
pub const CGameSaveEventData__m_fileNameWithoutExtOrPath__OFFSET: usize = 56;

#[repr(C)]
pub struct CPlayerWar3__5 {
    pub _opaque: [u8; 1960],
}
pub const CPlayerWar3__5__SIZE: usize = 1960;
pub const CPlayerWar3__5__NAME: &str = "CPlayerWar3";
pub const CPlayerWar3__5__s_CPlayerWar3_pool__OFFSET: usize = 0;
pub const CPlayerWar3__5__anticheat_dummy_array_15__OFFSET: usize = 88;
pub const CPlayerWar3__5__m_playerId__OFFSET: usize = 103;
pub const CPlayerWar3__5__m_playerName__OFFSET: usize = 104;
pub const CPlayerWar3__5__m_toonName__OFFSET: usize = 128;
pub const CPlayerWar3__5__m_protobufPlayerData__OFFSET: usize = 152;
pub const CPlayerWar3__5__m_slotNetPlayerId__OFFSET: usize = 336;
pub const CPlayerWar3__5__m_playerNamePrefix__OFFSET: usize = 344;
pub const CPlayerWar3__5__m_pAlliances__OFFSET: usize = 368;
pub const CPlayerWar3__5__m_pSelection__OFFSET: usize = 376;
pub const CPlayerWar3__5__m_playerStates__OFFSET: usize = 384;
pub const CPlayerWar3__5__m_lumberDivertTax__OFFSET: usize = 1016;
pub const CPlayerWar3__5__m_goldDivertTax__OFFSET: usize = 1116;
pub const CPlayerWar3__5__m_highestFood__OFFSET: usize = 1216;
pub const CPlayerWar3__5__m_highestFoodUsed__OFFSET: usize = 1220;
pub const CPlayerWar3__5__anticheat_dummy_array_16__OFFSET: usize = 1224;
pub const CPlayerWar3__5__m_heroKills__OFFSET: usize = 1240;
pub const CPlayerWar3__5__m_itemsObtained__OFFSET: usize = 1244;
pub const CPlayerWar3__5__m_mercsHired__OFFSET: usize = 1248;
pub const CPlayerWar3__5__m_teamColorIndex__OFFSET: usize = 1252;
pub const CPlayerWar3__5__m_controller__OFFSET: usize = 1256;
pub const CPlayerWar3__5__m_startLocIndex__OFFSET: usize = 1260;
pub const CPlayerWar3__5__m_difficulty__OFFSET: usize = 1264;
pub const CPlayerWar3__5__m_slotState__OFFSET: usize = 1268;
pub const CPlayerWar3__5__m_teamIndex__OFFSET: usize = 1272;
pub const CPlayerWar3__5__m_numActions__OFFSET: usize = 1276;
pub const CPlayerWar3__5__m_racePref__OFFSET: usize = 1280;
pub const CPlayerWar3__5__m_race__OFFSET: usize = 1284;
pub const CPlayerWar3__5__m_realTimeAPM__OFFSET: usize = 1288;
pub const CPlayerWar3__5__m_capChangedListener__OFFSET: usize = 1384;
pub const CPlayerWar3__5__m_usedChangedListener__OFFSET: usize = 1392;
pub const CPlayerWar3__5__m_foodUsedUpkeepListener__OFFSET: usize = 1400;
pub const CPlayerWar3__5__m_lumberChangedListener__OFFSET: usize = 1408;
pub const CPlayerWar3__5__m_goldChangedListener__OFFSET: usize = 1416;
pub const CPlayerWar3__5__m_goldUpkeepChanged__OFFSET: usize = 1424;
pub const CPlayerWar3__5__m_lumberUpkeepChanged__OFFSET: usize = 1432;
pub const CPlayerWar3__5__m_handicap__OFFSET: usize = 1440;
pub const CPlayerWar3__5__m_referee__OFFSET: usize = 1512;
pub const CPlayerWar3__5__m_deadHeroAge__OFFSET: usize = 1520;
pub const CPlayerWar3__5__m_deadHeroes__OFFSET: usize = 1544;
pub const CPlayerWar3__5__m_revivableFlags__OFFSET: usize = 1568;
pub const CPlayerWar3__5__anticheat_dummy_array_17__OFFSET: usize = 1572;
pub const CPlayerWar3__5__m_pTechTree__OFFSET: usize = 1584;
pub const CPlayerWar3__5__m_fogMask__OFFSET: usize = 1592;
pub const CPlayerWar3__5__m_ai__OFFSET: usize = 1596;
pub const CPlayerWar3__5__m_bestHeroes__OFFSET: usize = 1608;
pub const CPlayerWar3__5__m_leaderboard__OFFSET: usize = 1632;
pub const CPlayerWar3__5__m_leaveGameTimer__OFFSET: usize = 1648;
pub const CPlayerWar3__5__anticheat_dummy_array_18__OFFSET: usize = 1680;
pub const CPlayerWar3__5__m_doScoreScreen__OFFSET: usize = 1708;
pub const CPlayerWar3__5__m_allowContinue__OFFSET: usize = 1709;
pub const CPlayerWar3__5__m_crippled__OFFSET: usize = 1712;
pub const CPlayerWar3__5__m_gameOverMessage__OFFSET: usize = 1720;
pub const CPlayerWar3__5__m_showInScoreScreen__OFFSET: usize = 1744;
pub const CPlayerWar3__5__m_cachedScore__OFFSET: usize = 1748;
pub const CPlayerWar3__5__m_cachedScoreIsCached__OFFSET: usize = 1752;
pub const CPlayerWar3__5__m_heroCount__OFFSET: usize = 1756;
pub const CPlayerWar3__5__m_bnetPlayerId__OFFSET: usize = 1760;
pub const CPlayerWar3__5__m_heroPickOrder__OFFSET: usize = 1768;
pub const CPlayerWar3__5__m_unitKills__OFFSET: usize = 1792;
pub const CPlayerWar3__5__m_unitsCreated__OFFSET: usize = 1824;
pub const CPlayerWar3__5__m_abilityInfo__OFFSET: usize = 1856;
pub const CPlayerWar3__5__m_upkeepTime__OFFSET: usize = 1888;
pub const CPlayerWar3__5__m_lastUpkeepTime__OFFSET: usize = 1920;
pub const CPlayerWar3__5__m_playerItemUseMap__OFFSET: usize = 1928;

#[repr(C)]
pub struct CGameWar3__5 {
    pub _opaque: [u8; 12872],
}
pub const CGameWar3__5__SIZE: usize = 12872;
pub const CGameWar3__5__NAME: &str = "CGameWar3";
pub const CGameWar3__5__s_campaignArchiveFile__OFFSET: usize = 0;
pub const CGameWar3__5__anticheat_dummy_array_19__OFFSET: usize = 9640;
pub const CGameWar3__5__s_pGameWar3__OFFSET: usize = 0;
pub const CGameWar3__5__anticheat_dummy_array_20__OFFSET: usize = 9695;
pub const CGameWar3__5__m_mapScriptInstance__OFFSET: usize = 9784;
pub const CGameWar3__5__m_mapScriptInstanceId__OFFSET: usize = 9792;
pub const CGameWar3__5__m_mapScript__OFFSET: usize = 9800;
pub const CGameWar3__5__m_mapScriptId__OFFSET: usize = 9808;
pub const CGameWar3__5__m_executions__OFFSET: usize = 9816;
pub const CGameWar3__5__anticheat_dummy_array_21__OFFSET: usize = 9840;
pub const CGameWar3__5__m_gameIdMap__OFFSET: usize = 9864;
pub const CGameWar3__5__m_gameState__OFFSET: usize = 9872;
pub const CGameWar3__5__m_buildId__OFFSET: usize = 9880;
pub const CGameWar3__5__m_localPlayerId__OFFSET: usize = 9884;
pub const CGameWar3__5__m_replayLocalPlayerId__OFFSET: usize = 9886;
pub const CGameWar3__5__m_playerIdBitsToGhost__OFFSET: usize = 9888;
pub const CGameWar3__5__m_playerIdBitsToGhostSet__OFFSET: usize = 9892;
pub const CGameWar3__5__m_pMapSetup__OFFSET: usize = 9896;
pub const CGameWar3__5__m_pFog__OFFSET: usize = 9904;
pub const CGameWar3__5__m_gameName__OFFSET: usize = 9912;
pub const CGameWar3__5__anticheat_dummy_array_22__OFFSET: usize = 9936;
pub const CGameWar3__5__m_maxPlayerCount__OFFSET: usize = 9952;
pub const CGameWar3__5__m_playerCount__OFFSET: usize = 9956;
pub const CGameWar3__5__m_gameMaxPlayerCount__OFFSET: usize = 9960;
pub const CGameWar3__5__m_maxTeamCount__OFFSET: usize = 9964;
pub const CGameWar3__5__m_teams__OFFSET: usize = 9968;
pub const CGameWar3__5__m_players__OFFSET: usize = 9992;
pub const CGameWar3__5__m_startLocations__OFFSET: usize = 10224;
pub const CGameWar3__5__m_slotToPlayerId__OFFSET: usize = 11960;
pub const CGameWar3__5__m_levelToTransitionTo__OFFSET: usize = 12064;
pub const CGameWar3__5__m_modelCine__OFFSET: usize = 12088;
pub const CGameWar3__5__m_cine__OFFSET: usize = 12112;
pub const CGameWar3__5__m_gameOverMessage__OFFSET: usize = 12376;
pub const CGameWar3__5__m_switchingLevels__OFFSET: usize = 12400;
pub const CGameWar3__5__m_doScoreScreen__OFFSET: usize = 12401;
pub const CGameWar3__5__m_isReload__OFFSET: usize = 12402;
pub const CGameWar3__5__anticheat_dummy_array_23__OFFSET: usize = 12403;
pub const CGameWar3__5__m_wasScriptLoad__OFFSET: usize = 12410;
pub const CGameWar3__5__m_wasLoadGame__OFFSET: usize = 12411;
pub const CGameWar3__5__m_isCampaignMission__OFFSET: usize = 12412;
pub const CGameWar3__5__m_campaignIndex__OFFSET: usize = 12416;
pub const CGameWar3__5__m_gameSetup__OFFSET: usize = 12420;
pub const CGameWar3__5__m_usingDarkMask__OFFSET: usize = 12678;
pub const CGameWar3__5__m_unseenQuests__OFFSET: usize = 12679;
pub const CGameWar3__5__m_showAllUnitsCmdBar__OFFSET: usize = 12680;
pub const CGameWar3__5__m_isReplay__OFFSET: usize = 12681;
pub const CGameWar3__5__m_replayFile__OFFSET: usize = 12688;
pub const CGameWar3__5__m_isTournament__OFFSET: usize = 12712;
pub const CGameWar3__5__m_copySaveCount__OFFSET: usize = 12716;
pub const CGameWar3__5__m_finishedInit__OFFSET: usize = 12720;
pub const CGameWar3__5__m_didSaveGameFromScript__OFFSET: usize = 12721;
pub const CGameWar3__5__m_playingGame__OFFSET: usize = 12722;
pub const CGameWar3__5__m_FacialAnimationSetScopeID__OFFSET: usize = 12724;
pub const CGameWar3__5__m_pTriggerStrings__OFFSET: usize = 12728;
pub const CGameWar3__5__anticheat_dummy_array_24__OFFSET: usize = 12736;
pub const CGameWar3__5__m_messageLogContents__OFFSET: usize = 12776;
pub const CGameWar3__5__m_pGameHashTableManager__OFFSET: usize = 12800;
pub const CGameWar3__5__m_gameCacheManager__OFFSET: usize = 12808;
pub const CGameWar3__5__m_gameOverCallback__OFFSET: usize = 12816;
pub const CGameWar3__5__m_maxCheckpointSaves__OFFSET: usize = 12824;
pub const CGameWar3__5__m_pSaveThread__OFFSET: usize = 12832;
pub const CGameWar3__5__m_pCheckpointPopup__OFFSET: usize = 12840;
pub const CGameWar3__5__m_gamePlayStartTime__OFFSET: usize = 12848;
pub const CGameWar3__5__m_latency__OFFSET: usize = 12856;

#[repr(C)]
pub struct CWorldFrameWar3__5 {
    pub _opaque: [u8; 3128],
}
pub const CWorldFrameWar3__5__SIZE: usize = 3128;
pub const CWorldFrameWar3__5__NAME: &str = "CWorldFrameWar3";
pub const CWorldFrameWar3__5__m_trackUpdating__OFFSET: usize = 656;
pub const CWorldFrameWar3__5__m_sprites__OFFSET: usize = 664;
pub const CWorldFrameWar3__5__m_targetedSprites__OFFSET: usize = 672;
pub const CWorldFrameWar3__5__m_walkableSprites__OFFSET: usize = 680;
pub const CWorldFrameWar3__5__m_pGame__OFFSET: usize = 688;
pub const CWorldFrameWar3__5__m_pDoHpBarsEnemies__OFFSET: usize = 696;
pub const CWorldFrameWar3__5__m_pDoHpBarsAllies__OFFSET: usize = 697;
pub const CWorldFrameWar3__5__m_pLocalPlayer__OFFSET: usize = 704;
pub const CWorldFrameWar3__5__m_pFog__OFFSET: usize = 712;
pub const CWorldFrameWar3__5__m_cachedCameraPos__OFFSET: usize = 720;
pub const CWorldFrameWar3__5__m_localPlayerId__OFFSET: usize = 732;
pub const CWorldFrameWar3__5__m_pCameraWar3__OFFSET: usize = 736;
pub const CWorldFrameWar3__5__m_pMinimap__OFFSET: usize = 744;
pub const CWorldFrameWar3__5__m_spriteButtons__OFFSET: usize = 752;
pub const CWorldFrameWar3__5__m_terrainButtons__OFFSET: usize = 756;
pub const CWorldFrameWar3__5__m_cursorMode__OFFSET: usize = 760;
pub const CWorldFrameWar3__5__anticheat_dummy_array_12__OFFSET: usize = 764;
pub const CWorldFrameWar3__5__m_cursorModeStack__OFFSET: usize = 808;
pub const CWorldFrameWar3__5__m_cursorModeLocked__OFFSET: usize = 832;
pub const CWorldFrameWar3__5__m_forceCursorUpdate__OFFSET: usize = 833;
pub const CWorldFrameWar3__5__m_cursorArt__OFFSET: usize = 840;
pub const CWorldFrameWar3__5__m_aspectScaledProjection__OFFSET: usize = 864;
pub const CWorldFrameWar3__5__m_fovScale__OFFSET: usize = 880;
pub const CWorldFrameWar3__5__m_frustumBounds__OFFSET: usize = 884;
pub const CWorldFrameWar3__5__m_frustumBoundsInt__OFFSET: usize = 900;
pub const CWorldFrameWar3__5__m_localPlayerIdBit__OFFSET: usize = 916;
pub const CWorldFrameWar3__5__m_playerIdBitsToGhostMasks__OFFSET: usize = 920;
pub const CWorldFrameWar3__5__m_playerIdBitsToInvisMasks__OFFSET: usize = 1020;
pub const CWorldFrameWar3__5__m_localPlayerIsObserver__OFFSET: usize = 1120;
pub const CWorldFrameWar3__5__m_isReplayFogDisabled__OFFSET: usize = 1121;
pub const CWorldFrameWar3__5__m_lastUpdateElapsedSec__OFFSET: usize = 1124;
pub const CWorldFrameWar3__5__m_lastAsyncElapsedSec__OFFSET: usize = 1128;
pub const CWorldFrameWar3__5__m_lastTrackSprite__OFFSET: usize = 1136;
pub const CWorldFrameWar3__5__m_lastTrackAgent__OFFSET: usize = 1144;
pub const CWorldFrameWar3__5__m_renderPlacement__OFFSET: usize = 1152;
pub const CWorldFrameWar3__5__m_buildingPlacementRender__OFFSET: usize = 1153;
pub const CWorldFrameWar3__5__m_pBuildFrame__OFFSET: usize = 1160;
pub const CWorldFrameWar3__5__m_cineFilter__OFFSET: usize = 1168;
pub const CWorldFrameWar3__5__m_targetingSprite__OFFSET: usize = 1696;
pub const CWorldFrameWar3__5__m_renderTargetSprite__OFFSET: usize = 1704;
pub const CWorldFrameWar3__5__m_targetingImage__OFFSET: usize = 1708;
pub const CWorldFrameWar3__5__m_targetingRadius__OFFSET: usize = 1712;
pub const CWorldFrameWar3__5__m_targetingSqMag__OFFSET: usize = 1716;
pub const CWorldFrameWar3__5__m_targetImageColor__OFFSET: usize = 1720;
pub const CWorldFrameWar3__5__m_currentWorldPos__OFFSET: usize = 1724;
pub const CWorldFrameWar3__5__m_suspended__OFFSET: usize = 1736;
pub const CWorldFrameWar3__5__m_noHitTests__OFFSET: usize = 1737;
pub const CWorldFrameWar3__5__m_doOcclusion__OFFSET: usize = 1738;
pub const CWorldFrameWar3__5__m_mouseMoveProcessed__OFFSET: usize = 1739;
pub const CWorldFrameWar3__5__m_doFogOfWar__OFFSET: usize = 1740;
pub const CWorldFrameWar3__5__anticheat_dummy_array_13__OFFSET: usize = 1741;
pub const CWorldFrameWar3__5__m_useDarkMask__OFFSET: usize = 1744;
pub const CWorldFrameWar3__5__m_renderUnits__OFFSET: usize = 1745;
pub const CWorldFrameWar3__5__m_terrainFog__OFFSET: usize = 1752;
pub const CWorldFrameWar3__5__m_terrainLight__OFFSET: usize = 1760;
pub const CWorldFrameWar3__5__m_unitLight__OFFSET: usize = 1768;
pub const CWorldFrameWar3__5__m_suspendedAmbientLight__OFFSET: usize = 1776;
pub const CWorldFrameWar3__5__m_targetLight__OFFSET: usize = 1784;
pub const CWorldFrameWar3__5__m_skyModelPath__OFFSET: usize = 1792;
pub const CWorldFrameWar3__5__m_sky__OFFSET: usize = 1816;
pub const CWorldFrameWar3__5__m_showSky__OFFSET: usize = 1824;
pub const CWorldFrameWar3__5__m_renderSky__OFFSET: usize = 1825;
pub const CWorldFrameWar3__5__m_indicators__OFFSET: usize = 1832;
pub const CWorldFrameWar3__5__m_targetIndicators__OFFSET: usize = 1856;
pub const CWorldFrameWar3__5__m_targetIndicatorIndex__OFFSET: usize = 1880;
pub const CWorldFrameWar3__5__m_enumElapsed__OFFSET: usize = 1884;
pub const CWorldFrameWar3__5__m_asyncElapsed__OFFSET: usize = 1888;
pub const CWorldFrameWar3__5__m_doEnumUpdate__OFFSET: usize = 1892;
pub const CWorldFrameWar3__5__m_doFogEnum__OFFSET: usize = 1893;
pub const CWorldFrameWar3__5__m_trackingElapsed__OFFSET: usize = 1896;
pub const CWorldFrameWar3__5__m_terrainMinZ__OFFSET: usize = 1900;
pub const CWorldFrameWar3__5__m_scaledAnimTime__OFFSET: usize = 1904;
pub const CWorldFrameWar3__5__m_hoursPerDay__OFFSET: usize = 1908;
pub const CWorldFrameWar3__5__m_selCircColorFriend__OFFSET: usize = 1912;
pub const CWorldFrameWar3__5__anticheat_dummy_array_14__OFFSET: usize = 1916;
pub const CWorldFrameWar3__5__m_selCircColorNeutral__OFFSET: usize = 1921;
pub const CWorldFrameWar3__5__m_selCircColorEnemy__OFFSET: usize = 1925;
pub const CWorldFrameWar3__5__m_enableTargetIndicator__OFFSET: usize = 1929;
pub const CWorldFrameWar3__5__m_rallyFlags__OFFSET: usize = 1932;
pub const CWorldFrameWar3__5__m_rallyDst__OFFSET: usize = 1936;
pub const CWorldFrameWar3__5__m_rallyDstReuse__OFFSET: usize = 2708;
pub const CWorldFrameWar3__5__m_rallySrc__OFFSET: usize = 2776;
pub const CWorldFrameWar3__5__m_indicatorTerrain__OFFSET: usize = 2784;
pub const CWorldFrameWar3__5__m_waypointIndicators__OFFSET: usize = 2792;
pub const CWorldFrameWar3__5__m_waypointIndicatorIndex__OFFSET: usize = 2816;
pub const CWorldFrameWar3__5__m_unitUpdateQueue__OFFSET: usize = 2824;
pub const CWorldFrameWar3__5__m_destUpdateQueue__OFFSET: usize = 2848;
pub const CWorldFrameWar3__5__m_itemUpdateQueue__OFFSET: usize = 2872;
pub const CWorldFrameWar3__5__m_captUpdateQueue__OFFSET: usize = 2896;
pub const CWorldFrameWar3__5__m_imagUpdateQueue__OFFSET: usize = 2920;
pub const CWorldFrameWar3__5__m_ghstUpdateQueue__OFFSET: usize = 2944;
pub const CWorldFrameWar3__5__anticheat_dummy_array_15__OFFSET: usize = 2968;
pub const CWorldFrameWar3__5__m_enumeratedUnits__OFFSET: usize = 2976;
pub const CWorldFrameWar3__5__m_enumeratedItems__OFFSET: usize = 3000;
pub const CWorldFrameWar3__5__m_visibleUnits__OFFSET: usize = 3024;
pub const CWorldFrameWar3__5__m_visibleSelectables__OFFSET: usize = 3048;
pub const CWorldFrameWar3__5__m_fogUpdateList__OFFSET: usize = 3072;
pub const CWorldFrameWar3__5__m_ghostImages__OFFSET: usize = 3096;
pub const CWorldFrameWar3__5__m_lastRenderLight__OFFSET: usize = 3120;
pub const CWorldFrameWar3__5__m_lastRenderFog__OFFSET: usize = 3124;
pub const CWorldFrameWar3__5__enableDSQDenialMessage__OFFSET: usize = 0;

#[repr(C)]
pub struct CPlayerWar3__6 {
    pub _opaque: [u8; 1960],
}
pub const CPlayerWar3__6__SIZE: usize = 1960;
pub const CPlayerWar3__6__NAME: &str = "CPlayerWar3";
pub const CPlayerWar3__6__s_CPlayerWar3_pool__OFFSET: usize = 0;
pub const CPlayerWar3__6__anticheat_dummy_array_8__OFFSET: usize = 88;
pub const CPlayerWar3__6__m_playerId__OFFSET: usize = 103;
pub const CPlayerWar3__6__m_playerName__OFFSET: usize = 104;
pub const CPlayerWar3__6__m_toonName__OFFSET: usize = 128;
pub const CPlayerWar3__6__m_protobufPlayerData__OFFSET: usize = 152;
pub const CPlayerWar3__6__m_slotNetPlayerId__OFFSET: usize = 336;
pub const CPlayerWar3__6__m_playerNamePrefix__OFFSET: usize = 344;
pub const CPlayerWar3__6__m_pAlliances__OFFSET: usize = 368;
pub const CPlayerWar3__6__m_pSelection__OFFSET: usize = 376;
pub const CPlayerWar3__6__m_playerStates__OFFSET: usize = 384;
pub const CPlayerWar3__6__m_lumberDivertTax__OFFSET: usize = 1016;
pub const CPlayerWar3__6__m_goldDivertTax__OFFSET: usize = 1116;
pub const CPlayerWar3__6__m_highestFood__OFFSET: usize = 1216;
pub const CPlayerWar3__6__m_highestFoodUsed__OFFSET: usize = 1220;
pub const CPlayerWar3__6__anticheat_dummy_array_9__OFFSET: usize = 1224;
pub const CPlayerWar3__6__m_heroKills__OFFSET: usize = 1240;
pub const CPlayerWar3__6__m_itemsObtained__OFFSET: usize = 1244;
pub const CPlayerWar3__6__m_mercsHired__OFFSET: usize = 1248;
pub const CPlayerWar3__6__m_teamColorIndex__OFFSET: usize = 1252;
pub const CPlayerWar3__6__m_controller__OFFSET: usize = 1256;
pub const CPlayerWar3__6__m_startLocIndex__OFFSET: usize = 1260;
pub const CPlayerWar3__6__m_difficulty__OFFSET: usize = 1264;
pub const CPlayerWar3__6__m_slotState__OFFSET: usize = 1268;
pub const CPlayerWar3__6__m_teamIndex__OFFSET: usize = 1272;
pub const CPlayerWar3__6__m_numActions__OFFSET: usize = 1276;
pub const CPlayerWar3__6__m_racePref__OFFSET: usize = 1280;
pub const CPlayerWar3__6__m_race__OFFSET: usize = 1284;
pub const CPlayerWar3__6__m_realTimeAPM__OFFSET: usize = 1288;
pub const CPlayerWar3__6__m_capChangedListener__OFFSET: usize = 1384;
pub const CPlayerWar3__6__m_usedChangedListener__OFFSET: usize = 1392;
pub const CPlayerWar3__6__m_foodUsedUpkeepListener__OFFSET: usize = 1400;
pub const CPlayerWar3__6__m_lumberChangedListener__OFFSET: usize = 1408;
pub const CPlayerWar3__6__m_goldChangedListener__OFFSET: usize = 1416;
pub const CPlayerWar3__6__m_goldUpkeepChanged__OFFSET: usize = 1424;
pub const CPlayerWar3__6__m_lumberUpkeepChanged__OFFSET: usize = 1432;
pub const CPlayerWar3__6__m_handicap__OFFSET: usize = 1440;
pub const CPlayerWar3__6__m_referee__OFFSET: usize = 1512;
pub const CPlayerWar3__6__m_deadHeroAge__OFFSET: usize = 1520;
pub const CPlayerWar3__6__m_deadHeroes__OFFSET: usize = 1544;
pub const CPlayerWar3__6__m_revivableFlags__OFFSET: usize = 1568;
pub const CPlayerWar3__6__anticheat_dummy_array_10__OFFSET: usize = 1572;
pub const CPlayerWar3__6__m_pTechTree__OFFSET: usize = 1584;
pub const CPlayerWar3__6__m_fogMask__OFFSET: usize = 1592;
pub const CPlayerWar3__6__m_ai__OFFSET: usize = 1596;
pub const CPlayerWar3__6__m_bestHeroes__OFFSET: usize = 1608;
pub const CPlayerWar3__6__m_leaderboard__OFFSET: usize = 1632;
pub const CPlayerWar3__6__m_leaveGameTimer__OFFSET: usize = 1648;
pub const CPlayerWar3__6__anticheat_dummy_array_11__OFFSET: usize = 1680;
pub const CPlayerWar3__6__m_doScoreScreen__OFFSET: usize = 1708;
pub const CPlayerWar3__6__m_allowContinue__OFFSET: usize = 1709;
pub const CPlayerWar3__6__m_crippled__OFFSET: usize = 1712;
pub const CPlayerWar3__6__m_gameOverMessage__OFFSET: usize = 1720;
pub const CPlayerWar3__6__m_showInScoreScreen__OFFSET: usize = 1744;
pub const CPlayerWar3__6__m_cachedScore__OFFSET: usize = 1748;
pub const CPlayerWar3__6__m_cachedScoreIsCached__OFFSET: usize = 1752;
pub const CPlayerWar3__6__m_heroCount__OFFSET: usize = 1756;
pub const CPlayerWar3__6__m_bnetPlayerId__OFFSET: usize = 1760;
pub const CPlayerWar3__6__m_heroPickOrder__OFFSET: usize = 1768;
pub const CPlayerWar3__6__m_unitKills__OFFSET: usize = 1792;
pub const CPlayerWar3__6__m_unitsCreated__OFFSET: usize = 1824;
pub const CPlayerWar3__6__m_abilityInfo__OFFSET: usize = 1856;
pub const CPlayerWar3__6__m_upkeepTime__OFFSET: usize = 1888;
pub const CPlayerWar3__6__m_lastUpkeepTime__OFFSET: usize = 1920;
pub const CPlayerWar3__6__m_playerItemUseMap__OFFSET: usize = 1928;

#[repr(C)]
pub struct CGameWar3__6 {
    pub _opaque: [u8; 12872],
}
pub const CGameWar3__6__SIZE: usize = 12872;
pub const CGameWar3__6__NAME: &str = "CGameWar3";
pub const CGameWar3__6__s_campaignArchiveFile__OFFSET: usize = 0;
pub const CGameWar3__6__anticheat_dummy_array_12__OFFSET: usize = 9640;
pub const CGameWar3__6__s_pGameWar3__OFFSET: usize = 0;
pub const CGameWar3__6__anticheat_dummy_array_13__OFFSET: usize = 9695;
pub const CGameWar3__6__m_mapScriptInstance__OFFSET: usize = 9784;
pub const CGameWar3__6__m_mapScriptInstanceId__OFFSET: usize = 9792;
pub const CGameWar3__6__m_mapScript__OFFSET: usize = 9800;
pub const CGameWar3__6__m_mapScriptId__OFFSET: usize = 9808;
pub const CGameWar3__6__m_executions__OFFSET: usize = 9816;
pub const CGameWar3__6__anticheat_dummy_array_14__OFFSET: usize = 9840;
pub const CGameWar3__6__m_gameIdMap__OFFSET: usize = 9864;
pub const CGameWar3__6__m_gameState__OFFSET: usize = 9872;
pub const CGameWar3__6__m_buildId__OFFSET: usize = 9880;
pub const CGameWar3__6__m_localPlayerId__OFFSET: usize = 9884;
pub const CGameWar3__6__m_replayLocalPlayerId__OFFSET: usize = 9886;
pub const CGameWar3__6__m_playerIdBitsToGhost__OFFSET: usize = 9888;
pub const CGameWar3__6__m_playerIdBitsToGhostSet__OFFSET: usize = 9892;
pub const CGameWar3__6__m_pMapSetup__OFFSET: usize = 9896;
pub const CGameWar3__6__m_pFog__OFFSET: usize = 9904;
pub const CGameWar3__6__m_gameName__OFFSET: usize = 9912;
pub const CGameWar3__6__anticheat_dummy_array_15__OFFSET: usize = 9936;
pub const CGameWar3__6__m_maxPlayerCount__OFFSET: usize = 9952;
pub const CGameWar3__6__m_playerCount__OFFSET: usize = 9956;
pub const CGameWar3__6__m_gameMaxPlayerCount__OFFSET: usize = 9960;
pub const CGameWar3__6__m_maxTeamCount__OFFSET: usize = 9964;
pub const CGameWar3__6__m_teams__OFFSET: usize = 9968;
pub const CGameWar3__6__m_players__OFFSET: usize = 9992;
pub const CGameWar3__6__m_startLocations__OFFSET: usize = 10224;
pub const CGameWar3__6__m_slotToPlayerId__OFFSET: usize = 11960;
pub const CGameWar3__6__m_levelToTransitionTo__OFFSET: usize = 12064;
pub const CGameWar3__6__m_modelCine__OFFSET: usize = 12088;
pub const CGameWar3__6__m_cine__OFFSET: usize = 12112;
pub const CGameWar3__6__m_gameOverMessage__OFFSET: usize = 12376;
pub const CGameWar3__6__m_switchingLevels__OFFSET: usize = 12400;
pub const CGameWar3__6__m_doScoreScreen__OFFSET: usize = 12401;
pub const CGameWar3__6__m_isReload__OFFSET: usize = 12402;
pub const CGameWar3__6__anticheat_dummy_array_16__OFFSET: usize = 12403;
pub const CGameWar3__6__m_wasScriptLoad__OFFSET: usize = 12410;
pub const CGameWar3__6__m_wasLoadGame__OFFSET: usize = 12411;
pub const CGameWar3__6__m_isCampaignMission__OFFSET: usize = 12412;
pub const CGameWar3__6__m_campaignIndex__OFFSET: usize = 12416;
pub const CGameWar3__6__m_gameSetup__OFFSET: usize = 12420;
pub const CGameWar3__6__m_usingDarkMask__OFFSET: usize = 12678;
pub const CGameWar3__6__m_unseenQuests__OFFSET: usize = 12679;
pub const CGameWar3__6__m_showAllUnitsCmdBar__OFFSET: usize = 12680;
pub const CGameWar3__6__m_isReplay__OFFSET: usize = 12681;
pub const CGameWar3__6__m_replayFile__OFFSET: usize = 12688;
pub const CGameWar3__6__m_isTournament__OFFSET: usize = 12712;
pub const CGameWar3__6__m_copySaveCount__OFFSET: usize = 12716;
pub const CGameWar3__6__m_finishedInit__OFFSET: usize = 12720;
pub const CGameWar3__6__m_didSaveGameFromScript__OFFSET: usize = 12721;
pub const CGameWar3__6__m_playingGame__OFFSET: usize = 12722;
pub const CGameWar3__6__m_FacialAnimationSetScopeID__OFFSET: usize = 12724;
pub const CGameWar3__6__m_pTriggerStrings__OFFSET: usize = 12728;
pub const CGameWar3__6__anticheat_dummy_array_17__OFFSET: usize = 12736;
pub const CGameWar3__6__m_messageLogContents__OFFSET: usize = 12776;
pub const CGameWar3__6__m_pGameHashTableManager__OFFSET: usize = 12800;
pub const CGameWar3__6__m_gameCacheManager__OFFSET: usize = 12808;
pub const CGameWar3__6__m_gameOverCallback__OFFSET: usize = 12816;
pub const CGameWar3__6__m_maxCheckpointSaves__OFFSET: usize = 12824;
pub const CGameWar3__6__m_pSaveThread__OFFSET: usize = 12832;
pub const CGameWar3__6__m_pCheckpointPopup__OFFSET: usize = 12840;
pub const CGameWar3__6__m_gamePlayStartTime__OFFSET: usize = 12848;
pub const CGameWar3__6__m_latency__OFFSET: usize = 12856;

#[repr(C)]
pub struct CGameUI__9 {
    pub _opaque: [u8; 2384],
}
pub const CGameUI__9__SIZE: usize = 2384;
pub const CGameUI__9__NAME: &str = "CGameUI";
pub const CGameUI__9__m_transitionState__OFFSET: usize = 704;
pub const CGameUI__9__m_cachedFonts__OFFSET: usize = 712;
pub const CGameUI__9__NUMBER_OF_PLAYER_ROWS__OFFSET: usize = 0;
pub const CGameUI__9__FIRST_ROW_Y_OFFSET__OFFSET: usize = 0;
pub const CGameUI__9__ROW_SPACING__OFFSET: usize = 0;
pub const CGameUI__9__m_userControl__OFFSET: usize = 784;
pub const CGameUI__9__m_userUI__OFFSET: usize = 785;
pub const CGameUI__9__m_currentInputMode__OFFSET: usize = 792;
pub const CGameUI__9__m_inputModeStack__OFFSET: usize = 800;
pub const CGameUI__9__m_sharedTooltip__OFFSET: usize = 824;
pub const CGameUI__9__m_uberTooltip__OFFSET: usize = 832;
pub const CGameUI__9__m_showUberTips__OFFSET: usize = 840;
pub const CGameUI__9__m_autosaveReplay__OFFSET: usize = 841;
pub const CGameUI__9__m_healthBarsMode__OFFSET: usize = 844;
pub const CGameUI__9__m_forceShowHealthBars__OFFSET: usize = 848;
pub const CGameUI__9__m_minimapSignals__OFFSET: usize = 849;
pub const CGameUI__9__m_useColoredHealthBars__OFFSET: usize = 850;
pub const CGameUI__9__m_useHeroFrames__OFFSET: usize = 851;
pub const CGameUI__9__m_showHeroLevelFrame__OFFSET: usize = 852;
pub const CGameUI__9__m_showNumericCooldown__OFFSET: usize = 853;
pub const CGameUI__9__m_showDenyIcon__OFFSET: usize = 854;
pub const CGameUI__9__m_showGoldmineUnitCounter__OFFSET: usize = 855;
pub const CGameUI__9__m_enableObserverUiV2__OFFSET: usize = 856;
pub const CGameUI__9__m_asyncPlayerData__OFFSET: usize = 860;
pub const CGameUI__9__m_targetMode__OFFSET: usize = 960;
pub const CGameUI__9__m_selectMode__OFFSET: usize = 968;
pub const CGameUI__9__m_dragSelectMode__OFFSET: usize = 976;
pub const CGameUI__9__m_dragScrollMode__OFFSET: usize = 984;
pub const CGameUI__9__m_buildMode__OFFSET: usize = 992;
pub const CGameUI__9__m_signalMode__OFFSET: usize = 1000;
pub const CGameUI__9__m_escapeMenu__OFFSET: usize = 1008;
pub const CGameUI__9__m_gameOverMenu__OFFSET: usize = 1016;
pub const CGameUI__9__m_suspendIndicator__OFFSET: usize = 1024;
pub const CGameUI__9__m_unresponsiveIndicator__OFFSET: usize = 1032;
pub const CGameUI__9__m_allianceMode__OFFSET: usize = 1040;
pub const CGameUI__9__m_chatMode__OFFSET: usize = 1048;
pub const CGameUI__9__m_questMode__OFFSET: usize = 1056;
pub const CGameUI__9__anticheat_dummy_array_8__OFFSET: usize = 1064;
pub const CGameUI__9__m_scriptDialogMode__OFFSET: usize = 1152;
pub const CGameUI__9__m_unresponsiveDelayTimerId__OFFSET: usize = 1160;
pub const CGameUI__9__m_selectionManager__OFFSET: usize = 1168;
pub const CGameUI__9__m_dragScrollManager__OFFSET: usize = 1176;
pub const CGameUI__9__m_camera__OFFSET: usize = 1184;
pub const CGameUI__9__m_paused__OFFSET: usize = 1192;
pub const CGameUI__9__m_uiPaused__OFFSET: usize = 1193;
pub const CGameUI__9__m_scriptPaused__OFFSET: usize = 1194;
pub const CGameUI__9__m_disableUpkeepMsg__OFFSET: usize = 1195;
pub const CGameUI__9__m_upkeepLvlSave__OFFSET: usize = 1196;
pub const CGameUI__9__m_refresh__OFFSET: usize = 1200;
pub const CGameUI__9__m_refreshWaypoints__OFFSET: usize = 1201;
pub const CGameUI__9__m_deferCleanSelUI__OFFSET: usize = 1202;
pub const CGameUI__9__m_refreshInfoBar__OFFSET: usize = 1203;
pub const CGameUI__9__m_refreshCommandBar__OFFSET: usize = 1204;
pub const CGameUI__9__m_refreshHeroBar__OFFSET: usize = 1205;
pub const CGameUI__9__m_popSubMenu__OFFSET: usize = 1206;
pub const CGameUI__9__m_activeValidOrder__OFFSET: usize = 1208;
pub const CGameUI__9__m_cineForceSubtitles__OFFSET: usize = 1212;
pub const CGameUI__9__m_cinematicPanelsHidden__OFFSET: usize = 1213;
pub const CGameUI__9__m_dragSelectEnabled__OFFSET: usize = 1214;
pub const CGameUI__9__m_dragSelectUIEnabled__OFFSET: usize = 1215;
pub const CGameUI__9__m_preSelectEnabled__OFFSET: usize = 1216;
pub const CGameUI__9__m_preSelectUIEnabled__OFFSET: usize = 1217;
pub const CGameUI__9__m_selectEnabled__OFFSET: usize = 1218;
pub const CGameUI__9__m_selectUIEnabled__OFFSET: usize = 1219;
pub const CGameUI__9__m_observerModeEnabled__OFFSET: usize = 1220;
pub const CGameUI__9__m_observerModeV2Active__OFFSET: usize = 1221;
pub const CGameUI__9__m_wideScreenAutoPosition__OFFSET: usize = 1222;
pub const CGameUI__9__m_fadeTimer__OFFSET: usize = 1224;
pub const CGameUI__9__m_cinematicFadeTimer__OFFSET: usize = 1328;
pub const CGameUI__9__anticheat_dummy_array_9__OFFSET: usize = 1432;
pub const CGameUI__9__m_frameParentTimer__OFFSET: usize = 1440;
pub const CGameUI__9__m_simpleFadeTimer__OFFSET: usize = 1544;
pub const CGameUI__9__m_attackNotify__OFFSET: usize = 1552;
pub const CGameUI__9__m_warpNotify__OFFSET: usize = 1576;
pub const CGameUI__9__m_timerDialogs__OFFSET: usize = 1608;
pub const CGameUI__9__m_leaderboard__OFFSET: usize = 1632;
pub const CGameUI__9__anticheat_dummy_array_10__OFFSET: usize = 1640;
pub const CGameUI__9__m_multiboard__OFFSET: usize = 1648;
pub const CGameUI__9__m_suspendMultiboardDisplay__OFFSET: usize = 1656;
pub const CGameUI__9__anticheat_dummy_array_11__OFFSET: usize = 1657;
pub const CGameUI__9__m_scriptDialog__OFFSET: usize = 1728;
pub const CGameUI__9__m_replaySelection__OFFSET: usize = 1736;
pub const CGameUI__9__m_unitStatusIconUIFactory__OFFSET: usize = 1744;
pub const CGameUI__9__m_unitStatusTexts__OFFSET: usize = 1752;
pub const CGameUI__9__m_worldFrame__OFFSET: usize = 1776;
pub const CGameUI__9__m_infoBar__OFFSET: usize = 1784;
pub const CGameUI__9__m_commandBar__OFFSET: usize = 1792;
pub const CGameUI__9__m_miniMap__OFFSET: usize = 1800;
pub const CGameUI__9__m_resourceBar__OFFSET: usize = 1808;
pub const CGameUI__9__m_upperButtonBar__OFFSET: usize = 1816;
pub const CGameUI__9__m_observerButtonBar__OFFSET: usize = 1824;
pub const CGameUI__9__m_playerStatisticsPanel__OFFSET: usize = 1832;
pub const CGameUI__9__m_heroPanel__OFFSET: usize = 1840;
pub const CGameUI__9__m_productionPanel__OFFSET: usize = 1848;
pub const CGameUI__9__m_minimapButtonBar__OFFSET: usize = 1856;
pub const CGameUI__9__m_trackingStub__OFFSET: usize = 1864;
pub const CGameUI__9__m_heroBar__OFFSET: usize = 1872;
pub const CGameUI__9__m_peonBar__OFFSET: usize = 1880;
pub const CGameUI__9__m_messageBox__OFFSET: usize = 1888;
pub const CGameUI__9__m_unitMessageBox__OFFSET: usize = 1896;
pub const CGameUI__9__m_chatMessageBox__OFFSET: usize = 1904;
pub const CGameUI__9__m_topMessageBox__OFFSET: usize = 1912;
pub const CGameUI__9__m_currentPortraitButton__OFFSET: usize = 1920;
pub const CGameUI__9__m_timeOfDayIndicator__OFFSET: usize = 1928;
pub const CGameUI__9__m_chatEditBar__OFFSET: usize = 1936;
pub const CGameUI__9__m_cinematicPanel__OFFSET: usize = 1944;
pub const CGameUI__9__m_lastBuffBar__OFFSET: usize = 1952;
pub const CGameUI__9__m_observerBar__OFFSET: usize = 1960;
pub const CGameUI__9__m_miniMapSignalButton__OFFSET: usize = 1968;
pub const CGameUI__9__m_miniMapTerrainButton__OFFSET: usize = 1976;
pub const CGameUI__9__m_miniMapAllyButton__OFFSET: usize = 1984;
pub const CGameUI__9__m_miniMapCreepButton__OFFSET: usize = 1992;
pub const CGameUI__9__m_formationButton__OFFSET: usize = 2000;
pub const CGameUI__9__m_consoleFrame__OFFSET: usize = 2008;
pub const CGameUI__9__m_simpleConsoleFrame__OFFSET: usize = 2016;
pub const CGameUI__9__m_simpleConsoleBottomBarFrame__OFFSET: usize = 2024;
pub const CGameUI__9__m_simpleConsoleBottomBarFrameOverlay__OFFSET: usize = 2032;
pub const CGameUI__9__m_frameUIParent__OFFSET: usize = 2040;
pub const CGameUI__9__top__OFFSET: usize = 2048;
pub const CGameUI__9__topInGame__OFFSET: usize = 2056;
pub const CGameUI__9__m_simpleUIParent__OFFSET: usize = 2064;
pub const CGameUI__9__m_observerUIParent__OFFSET: usize = 2072;
pub const CGameUI__9__m_observerInfoPanelBackground__OFFSET: usize = 2080;
pub const CGameUI__9__m_observerInfoPanelBackpack__OFFSET: usize = 2088;
pub const CGameUI__9__m_consoleBackdropFrame__OFFSET: usize = 2096;
pub const CGameUI__9__m_minimapTextureBackground__OFFSET: usize = 2104;
pub const CGameUI__9__m_infoPanelTexture__OFFSET: usize = 2112;
pub const CGameUI__9__m_quickSaveKey__OFFSET: usize = 2120;
pub const CGameUI__9__m_quickLoadKey__OFFSET: usize = 2124;
pub const CGameUI__9__m_quickHelpKey__OFFSET: usize = 2128;
pub const CGameUI__9__m_quickOptionsKey__OFFSET: usize = 2132;
pub const CGameUI__9__m_quickQuitKey__OFFSET: usize = 2136;
pub const CGameUI__9__m_minimapSignalKey__OFFSET: usize = 2140;
pub const CGameUI__9__m_minimapTerrainKey__OFFSET: usize = 2144;
pub const CGameUI__9__m_minimapColorsKey__OFFSET: usize = 2148;
pub const CGameUI__9__m_minimapCreepKey__OFFSET: usize = 2152;
pub const CGameUI__9__m_formationToggleKey__OFFSET: usize = 2156;
pub const CGameUI__9__m_observerToggleAllPanelsKey__OFFSET: usize = 2160;
pub const CGameUI__9__m_observerProductionModeKey__OFFSET: usize = 2164;
pub const CGameUI__9__m_observerUnitsModeKey__OFFSET: usize = 2168;
pub const CGameUI__9__m_mouseDownHandler__OFFSET: usize = 2176;
pub const CGameUI__9__m_mouseWheelHandler__OFFSET: usize = 2184;
pub const CGameUI__9__m_mouseUpHandler__OFFSET: usize = 2192;
pub const CGameUI__9__m_mouseMoveHandler__OFFSET: usize = 2200;
pub const CGameUI__9__m_observerToggleablePanels__OFFSET: usize = 2208;
pub const CGameUI__9__m_consoleBackdropHeight__OFFSET: usize = 2240;
pub const CGameUI__9__m_auroraChatHandler__OFFSET: usize = 2248;
pub const CGameUI__9__m_lobbyEventsHandler__OFFSET: usize = 2328;
pub const CGameUI__9__s_indexedCustomHotkeyMappingMaxCount__OFFSET: usize = 0;
pub const CGameUI__9__s_customHotkeyEnabled__OFFSET: usize = 0;
pub const CGameUI__9__s_customHotkeyMappingTable__OFFSET: usize = 0;
pub const CGameUI__9__s_customHotkeyMappingTableNonHero__OFFSET: usize = 0;
pub const CGameUI__9__m_observerUnitToFollow__OFFSET: usize = 2376;

#[repr(C)]
pub struct CMapListBoxItem {
    pub _opaque: [u8; 904],
}
pub const CMapListBoxItem__SIZE: usize = 904;
pub const CMapListBoxItem__NAME: &str = "CMapListBoxItem";
pub const CMapListBoxItem__m_itemType__OFFSET: usize = 872;
pub const CMapListBoxItem__m_icon__OFFSET: usize = 880;
pub const CMapListBoxItem__m_maxPlayersText__OFFSET: usize = 888;
pub const CMapListBoxItem__m_textColor__OFFSET: usize = 896;

#[repr(C)]
pub struct CMapList {
    pub _opaque: [u8; 1488],
}
pub const CMapList__SIZE: usize = 1488;
pub const CMapList__NAME: &str = "CMapList";
pub const CMapList__m_needsListUpdate__OFFSET: usize = 952;
pub const CMapList__m_selectOnUpdate__OFFSET: usize = 953;
pub const CMapList__mCurDir__OFFSET: usize = 954;
pub const CMapList__mDirLevel__OFFSET: usize = 1216;
pub const CMapList__mMapScript__OFFSET: usize = 1220;
pub const CMapList__mListType__OFFSET: usize = 1480;
pub const CMapList__mPrefIndex__OFFSET: usize = 1484;

#[repr(C)]
pub struct CMapListBoxData {
    pub _opaque: [u8; 688],
}
pub const CMapListBoxData__SIZE: usize = 688;
pub const CMapListBoxData__NAME: &str = "CMapListBoxData";
pub const CMapListBoxData__m_itemType__OFFSET: usize = 144;
pub const CMapListBoxData__m_fileName__OFFSET: usize = 148;
pub const CMapListBoxData__m_mapName__OFFSET: usize = 408;
pub const CMapListBoxData__m_numPlayers__OFFSET: usize = 668;
pub const CMapListBoxData__m_lastWriteTime__OFFSET: usize = 672;
pub const CMapListBoxData__m_netVersionId__OFFSET: usize = 680;
pub const CMapListBoxData__m_netProgramId__OFFSET: usize = 684;

#[repr(C)]
pub struct CMapInfoPane {
    pub _opaque: [u8; 1360],
}
pub const CMapInfoPane__SIZE: usize = 1360;
pub const CMapInfoPane__NAME: &str = "CMapInfoPane";
pub const CMapInfoPane__m_hiddenMinimap__OFFSET: usize = 656;
pub const CMapInfoPane__m_cachedMaxWidths__OFFSET: usize = 657;
pub const CMapInfoPane__m_maxMapNameWidth__OFFSET: usize = 660;
pub const CMapInfoPane__m_maxSuggestedWidth__OFFSET: usize = 664;
pub const CMapInfoPane__m_maxMapSizeWidth__OFFSET: usize = 668;
pub const CMapInfoPane__m_maxTilesetWidth__OFFSET: usize = 672;
pub const CMapInfoPane__m_maxDescHeight__OFFSET: usize = 676;
pub const CMapInfoPane__m_mapInfoLayout__OFFSET: usize = 680;
pub const CMapInfoPane__m_iconType__OFFSET: usize = 684;
pub const CMapInfoPane__m_authIconType__OFFSET: usize = 688;
pub const CMapInfoPane__m_displayedAuthIconType__OFFSET: usize = 692;
pub const CMapInfoPane__m_mapInfoFields__OFFSET: usize = 696;
pub const CMapInfoPane__m_mapFile__OFFSET: usize = 700;
pub const CMapInfoPane__m_campaignArchiveFile__OFFSET: usize = 960;
pub const CMapInfoPane__m_needsRefresh__OFFSET: usize = 1220;
pub const CMapInfoPane__m_needsPopulate__OFFSET: usize = 1221;
pub const CMapInfoPane__m_borderInset__OFFSET: usize = 1224;
pub const CMapInfoPane__m_fieldGap__OFFSET: usize = 1228;
pub const CMapInfoPane__m_numPlayers__OFFSET: usize = 1232;
pub const CMapInfoPane__m_elapsedTime__OFFSET: usize = 1236;
pub const CMapInfoPane__m_lastPopulateRequest__OFFSET: usize = 1240;
pub const CMapInfoPane__m_authIcon__OFFSET: usize = 1248;
pub const CMapInfoPane__m_maxPlayersIcon__OFFSET: usize = 1256;
pub const CMapInfoPane__m_maxPlayersValue__OFFSET: usize = 1264;
pub const CMapInfoPane__m_mapNameValue__OFFSET: usize = 1272;
pub const CMapInfoPane__m_mapImage__OFFSET: usize = 1280;
pub const CMapInfoPane__m_mapImageBackdrop__OFFSET: usize = 1288;
pub const CMapInfoPane__m_mapSuggestedPlayersLabel__OFFSET: usize = 1296;
pub const CMapInfoPane__m_mapSizeLabel__OFFSET: usize = 1304;
pub const CMapInfoPane__m_mapTilesetLabel__OFFSET: usize = 1312;
pub const CMapInfoPane__m_mapDescLabel__OFFSET: usize = 1320;
pub const CMapInfoPane__m_mapSuggestedPlayersValue__OFFSET: usize = 1328;
pub const CMapInfoPane__m_mapSizeValue__OFFSET: usize = 1336;
pub const CMapInfoPane__m_mapTilesetValue__OFFSET: usize = 1344;
pub const CMapInfoPane__m_mapDescValue__OFFSET: usize = 1352;

#[repr(C)]
pub struct CMapPreferenceBox {
    pub _opaque: [u8; 968],
}
pub const CMapPreferenceBox__SIZE: usize = 968;
pub const CMapPreferenceBox__NAME: &str = "CMapPreferenceBox";
pub const CMapPreferenceBox__m_maxVetoes__OFFSET: usize = 952;
pub const CMapPreferenceBox__m_currentVetoes__OFFSET: usize = 956;
pub const CMapPreferenceBox__m_readOnly__OFFSET: usize = 960;
pub const CMapPreferenceBox__m_thumbsVisible__OFFSET: usize = 961;
pub const CMapPreferenceBox__m_thumbsUpEnabled__OFFSET: usize = 962;

#[repr(C)]
pub struct CMapPreferenceBoxItem {
    pub _opaque: [u8; 936],
}
pub const CMapPreferenceBoxItem__SIZE: usize = 936;
pub const CMapPreferenceBoxItem__NAME: &str = "CMapPreferenceBoxItem";
pub const CMapPreferenceBoxItem__m_itemType__OFFSET: usize = 872;
pub const CMapPreferenceBoxItem__m_preferredOnButton__OFFSET: usize = 880;
pub const CMapPreferenceBoxItem__m_preferredOffButton__OFFSET: usize = 888;
pub const CMapPreferenceBoxItem__m_icon__OFFSET: usize = 896;
pub const CMapPreferenceBoxItem__m_maxPlayersValue__OFFSET: usize = 904;
pub const CMapPreferenceBoxItem__m_textColor__OFFSET: usize = 912;
pub const CMapPreferenceBoxItem__m_itemData__OFFSET: usize = 920;
pub const CMapPreferenceBoxItem__m_parentBox__OFFSET: usize = 928;

#[repr(C)]
pub struct CMapPreferenceBoxData {
    pub _opaque: [u8; 640],
}
pub const CMapPreferenceBoxData__SIZE: usize = 640;
pub const CMapPreferenceBoxData__NAME: &str = "CMapPreferenceBoxData";
pub const CMapPreferenceBoxData__m_mapFile__OFFSET: usize = 144;
pub const CMapPreferenceBoxData__m_mapName__OFFSET: usize = 404;
pub const CMapPreferenceBoxData__m_numPlayers__OFFSET: usize = 620;
pub const CMapPreferenceBoxData__m_preferred__OFFSET: usize = 624;
pub const CMapPreferenceBoxData__m_prefState__OFFSET: usize = 628;
pub const CMapPreferenceBoxData__m_itemType__OFFSET: usize = 632;

#[repr(C)]
pub struct CPlayerSlot {
    pub _opaque: [u8; 1048],
}
pub const CPlayerSlot__SIZE: usize = 1048;
pub const CPlayerSlot__NAME: &str = "CPlayerSlot";
pub const CPlayerSlot__m_maxTeams__OFFSET: usize = 840;
pub const CPlayerSlot__m_allowObservers__OFFSET: usize = 844;
pub const CPlayerSlot__m_observersAreReferees__OFFSET: usize = 845;
pub const CPlayerSlot__m_playerName__OFFSET: usize = 846;
pub const CPlayerSlot__m_playerNameIndex__OFFSET: usize = 912;
pub const CPlayerSlot__m_playerRacePref__OFFSET: usize = 916;
pub const CPlayerSlot__m_playerColor__OFFSET: usize = 920;
pub const CPlayerSlot__m_playerTeam__OFFSET: usize = 924;
pub const CPlayerSlot__m_playerHandicap__OFFSET: usize = 928;
pub const CPlayerSlot__m_playerProgress__OFFSET: usize = 932;
pub const CPlayerSlot__m_progressLabel__OFFSET: usize = 936;
pub const CPlayerSlot__m_nameMenu__OFFSET: usize = 944;
pub const CPlayerSlot__m_raceMenu__OFFSET: usize = 952;
pub const CPlayerSlot__m_teamButton__OFFSET: usize = 960;
pub const CPlayerSlot__m_teamButtonArrow__OFFSET: usize = 968;
pub const CPlayerSlot__m_teamButtonText__OFFSET: usize = 976;
pub const CPlayerSlot__m_colorButton__OFFSET: usize = 984;
pub const CPlayerSlot__m_colorButtonBackdrop__OFFSET: usize = 992;
pub const CPlayerSlot__m_colorButtonArrow__OFFSET: usize = 1000;
pub const CPlayerSlot__m_handicapMenu__OFFSET: usize = 1008;
pub const CPlayerSlot__m_teamSetup__OFFSET: usize = 1016;
pub const CPlayerSlot__m_teamColorMenu__OFFSET: usize = 1024;
pub const CPlayerSlot__m_pingLabel__OFFSET: usize = 1032;
pub const CPlayerSlot__m_createContext__OFFSET: usize = 1040;

#[repr(C)]
pub struct CGameUI__10 {
    pub _opaque: [u8; 2384],
}
pub const CGameUI__10__SIZE: usize = 2384;
pub const CGameUI__10__NAME: &str = "CGameUI";
pub const CGameUI__10__m_transitionState__OFFSET: usize = 704;
pub const CGameUI__10__m_cachedFonts__OFFSET: usize = 712;
pub const CGameUI__10__NUMBER_OF_PLAYER_ROWS__OFFSET: usize = 0;
pub const CGameUI__10__FIRST_ROW_Y_OFFSET__OFFSET: usize = 0;
pub const CGameUI__10__ROW_SPACING__OFFSET: usize = 0;
pub const CGameUI__10__m_userControl__OFFSET: usize = 784;
pub const CGameUI__10__m_userUI__OFFSET: usize = 785;
pub const CGameUI__10__m_currentInputMode__OFFSET: usize = 792;
pub const CGameUI__10__m_inputModeStack__OFFSET: usize = 800;
pub const CGameUI__10__m_sharedTooltip__OFFSET: usize = 824;
pub const CGameUI__10__m_uberTooltip__OFFSET: usize = 832;
pub const CGameUI__10__m_showUberTips__OFFSET: usize = 840;
pub const CGameUI__10__m_autosaveReplay__OFFSET: usize = 841;
pub const CGameUI__10__m_healthBarsMode__OFFSET: usize = 844;
pub const CGameUI__10__m_forceShowHealthBars__OFFSET: usize = 848;
pub const CGameUI__10__m_minimapSignals__OFFSET: usize = 849;
pub const CGameUI__10__m_useColoredHealthBars__OFFSET: usize = 850;
pub const CGameUI__10__m_useHeroFrames__OFFSET: usize = 851;
pub const CGameUI__10__m_showHeroLevelFrame__OFFSET: usize = 852;
pub const CGameUI__10__m_showNumericCooldown__OFFSET: usize = 853;
pub const CGameUI__10__m_showDenyIcon__OFFSET: usize = 854;
pub const CGameUI__10__m_showGoldmineUnitCounter__OFFSET: usize = 855;
pub const CGameUI__10__m_enableObserverUiV2__OFFSET: usize = 856;
pub const CGameUI__10__m_asyncPlayerData__OFFSET: usize = 860;
pub const CGameUI__10__m_targetMode__OFFSET: usize = 960;
pub const CGameUI__10__m_selectMode__OFFSET: usize = 968;
pub const CGameUI__10__m_dragSelectMode__OFFSET: usize = 976;
pub const CGameUI__10__m_dragScrollMode__OFFSET: usize = 984;
pub const CGameUI__10__m_buildMode__OFFSET: usize = 992;
pub const CGameUI__10__m_signalMode__OFFSET: usize = 1000;
pub const CGameUI__10__m_escapeMenu__OFFSET: usize = 1008;
pub const CGameUI__10__m_gameOverMenu__OFFSET: usize = 1016;
pub const CGameUI__10__m_suspendIndicator__OFFSET: usize = 1024;
pub const CGameUI__10__m_unresponsiveIndicator__OFFSET: usize = 1032;
pub const CGameUI__10__m_allianceMode__OFFSET: usize = 1040;
pub const CGameUI__10__m_chatMode__OFFSET: usize = 1048;
pub const CGameUI__10__m_questMode__OFFSET: usize = 1056;
pub const CGameUI__10__anticheat_dummy_array_16__OFFSET: usize = 1064;
pub const CGameUI__10__m_scriptDialogMode__OFFSET: usize = 1152;
pub const CGameUI__10__m_unresponsiveDelayTimerId__OFFSET: usize = 1160;
pub const CGameUI__10__m_selectionManager__OFFSET: usize = 1168;
pub const CGameUI__10__m_dragScrollManager__OFFSET: usize = 1176;
pub const CGameUI__10__m_camera__OFFSET: usize = 1184;
pub const CGameUI__10__m_paused__OFFSET: usize = 1192;
pub const CGameUI__10__m_uiPaused__OFFSET: usize = 1193;
pub const CGameUI__10__m_scriptPaused__OFFSET: usize = 1194;
pub const CGameUI__10__m_disableUpkeepMsg__OFFSET: usize = 1195;
pub const CGameUI__10__m_upkeepLvlSave__OFFSET: usize = 1196;
pub const CGameUI__10__m_refresh__OFFSET: usize = 1200;
pub const CGameUI__10__m_refreshWaypoints__OFFSET: usize = 1201;
pub const CGameUI__10__m_deferCleanSelUI__OFFSET: usize = 1202;
pub const CGameUI__10__m_refreshInfoBar__OFFSET: usize = 1203;
pub const CGameUI__10__m_refreshCommandBar__OFFSET: usize = 1204;
pub const CGameUI__10__m_refreshHeroBar__OFFSET: usize = 1205;
pub const CGameUI__10__m_popSubMenu__OFFSET: usize = 1206;
pub const CGameUI__10__m_activeValidOrder__OFFSET: usize = 1208;
pub const CGameUI__10__m_cineForceSubtitles__OFFSET: usize = 1212;
pub const CGameUI__10__m_cinematicPanelsHidden__OFFSET: usize = 1213;
pub const CGameUI__10__m_dragSelectEnabled__OFFSET: usize = 1214;
pub const CGameUI__10__m_dragSelectUIEnabled__OFFSET: usize = 1215;
pub const CGameUI__10__m_preSelectEnabled__OFFSET: usize = 1216;
pub const CGameUI__10__m_preSelectUIEnabled__OFFSET: usize = 1217;
pub const CGameUI__10__m_selectEnabled__OFFSET: usize = 1218;
pub const CGameUI__10__m_selectUIEnabled__OFFSET: usize = 1219;
pub const CGameUI__10__m_observerModeEnabled__OFFSET: usize = 1220;
pub const CGameUI__10__m_observerModeV2Active__OFFSET: usize = 1221;
pub const CGameUI__10__m_wideScreenAutoPosition__OFFSET: usize = 1222;
pub const CGameUI__10__m_fadeTimer__OFFSET: usize = 1224;
pub const CGameUI__10__m_cinematicFadeTimer__OFFSET: usize = 1328;
pub const CGameUI__10__anticheat_dummy_array_17__OFFSET: usize = 1432;
pub const CGameUI__10__m_frameParentTimer__OFFSET: usize = 1440;
pub const CGameUI__10__m_simpleFadeTimer__OFFSET: usize = 1544;
pub const CGameUI__10__m_attackNotify__OFFSET: usize = 1552;
pub const CGameUI__10__m_warpNotify__OFFSET: usize = 1576;
pub const CGameUI__10__m_timerDialogs__OFFSET: usize = 1608;
pub const CGameUI__10__m_leaderboard__OFFSET: usize = 1632;
pub const CGameUI__10__anticheat_dummy_array_18__OFFSET: usize = 1640;
pub const CGameUI__10__m_multiboard__OFFSET: usize = 1648;
pub const CGameUI__10__m_suspendMultiboardDisplay__OFFSET: usize = 1656;
pub const CGameUI__10__anticheat_dummy_array_19__OFFSET: usize = 1657;
pub const CGameUI__10__m_scriptDialog__OFFSET: usize = 1728;
pub const CGameUI__10__m_replaySelection__OFFSET: usize = 1736;
pub const CGameUI__10__m_unitStatusIconUIFactory__OFFSET: usize = 1744;
pub const CGameUI__10__m_unitStatusTexts__OFFSET: usize = 1752;
pub const CGameUI__10__m_worldFrame__OFFSET: usize = 1776;
pub const CGameUI__10__m_infoBar__OFFSET: usize = 1784;
pub const CGameUI__10__m_commandBar__OFFSET: usize = 1792;
pub const CGameUI__10__m_miniMap__OFFSET: usize = 1800;
pub const CGameUI__10__m_resourceBar__OFFSET: usize = 1808;
pub const CGameUI__10__m_upperButtonBar__OFFSET: usize = 1816;
pub const CGameUI__10__m_observerButtonBar__OFFSET: usize = 1824;
pub const CGameUI__10__m_playerStatisticsPanel__OFFSET: usize = 1832;
pub const CGameUI__10__m_heroPanel__OFFSET: usize = 1840;
pub const CGameUI__10__m_productionPanel__OFFSET: usize = 1848;
pub const CGameUI__10__m_minimapButtonBar__OFFSET: usize = 1856;
pub const CGameUI__10__m_trackingStub__OFFSET: usize = 1864;
pub const CGameUI__10__m_heroBar__OFFSET: usize = 1872;
pub const CGameUI__10__m_peonBar__OFFSET: usize = 1880;
pub const CGameUI__10__m_messageBox__OFFSET: usize = 1888;
pub const CGameUI__10__m_unitMessageBox__OFFSET: usize = 1896;
pub const CGameUI__10__m_chatMessageBox__OFFSET: usize = 1904;
pub const CGameUI__10__m_topMessageBox__OFFSET: usize = 1912;
pub const CGameUI__10__m_currentPortraitButton__OFFSET: usize = 1920;
pub const CGameUI__10__m_timeOfDayIndicator__OFFSET: usize = 1928;
pub const CGameUI__10__m_chatEditBar__OFFSET: usize = 1936;
pub const CGameUI__10__m_cinematicPanel__OFFSET: usize = 1944;
pub const CGameUI__10__m_lastBuffBar__OFFSET: usize = 1952;
pub const CGameUI__10__m_observerBar__OFFSET: usize = 1960;
pub const CGameUI__10__m_miniMapSignalButton__OFFSET: usize = 1968;
pub const CGameUI__10__m_miniMapTerrainButton__OFFSET: usize = 1976;
pub const CGameUI__10__m_miniMapAllyButton__OFFSET: usize = 1984;
pub const CGameUI__10__m_miniMapCreepButton__OFFSET: usize = 1992;
pub const CGameUI__10__m_formationButton__OFFSET: usize = 2000;
pub const CGameUI__10__m_consoleFrame__OFFSET: usize = 2008;
pub const CGameUI__10__m_simpleConsoleFrame__OFFSET: usize = 2016;
pub const CGameUI__10__m_simpleConsoleBottomBarFrame__OFFSET: usize = 2024;
pub const CGameUI__10__m_simpleConsoleBottomBarFrameOverlay__OFFSET: usize = 2032;
pub const CGameUI__10__m_frameUIParent__OFFSET: usize = 2040;
pub const CGameUI__10__top__OFFSET: usize = 2048;
pub const CGameUI__10__topInGame__OFFSET: usize = 2056;
pub const CGameUI__10__m_simpleUIParent__OFFSET: usize = 2064;
pub const CGameUI__10__m_observerUIParent__OFFSET: usize = 2072;
pub const CGameUI__10__m_observerInfoPanelBackground__OFFSET: usize = 2080;
pub const CGameUI__10__m_observerInfoPanelBackpack__OFFSET: usize = 2088;
pub const CGameUI__10__m_consoleBackdropFrame__OFFSET: usize = 2096;
pub const CGameUI__10__m_minimapTextureBackground__OFFSET: usize = 2104;
pub const CGameUI__10__m_infoPanelTexture__OFFSET: usize = 2112;
pub const CGameUI__10__m_quickSaveKey__OFFSET: usize = 2120;
pub const CGameUI__10__m_quickLoadKey__OFFSET: usize = 2124;
pub const CGameUI__10__m_quickHelpKey__OFFSET: usize = 2128;
pub const CGameUI__10__m_quickOptionsKey__OFFSET: usize = 2132;
pub const CGameUI__10__m_quickQuitKey__OFFSET: usize = 2136;
pub const CGameUI__10__m_minimapSignalKey__OFFSET: usize = 2140;
pub const CGameUI__10__m_minimapTerrainKey__OFFSET: usize = 2144;
pub const CGameUI__10__m_minimapColorsKey__OFFSET: usize = 2148;
pub const CGameUI__10__m_minimapCreepKey__OFFSET: usize = 2152;
pub const CGameUI__10__m_formationToggleKey__OFFSET: usize = 2156;
pub const CGameUI__10__m_observerToggleAllPanelsKey__OFFSET: usize = 2160;
pub const CGameUI__10__m_observerProductionModeKey__OFFSET: usize = 2164;
pub const CGameUI__10__m_observerUnitsModeKey__OFFSET: usize = 2168;
pub const CGameUI__10__m_mouseDownHandler__OFFSET: usize = 2176;
pub const CGameUI__10__m_mouseWheelHandler__OFFSET: usize = 2184;
pub const CGameUI__10__m_mouseUpHandler__OFFSET: usize = 2192;
pub const CGameUI__10__m_mouseMoveHandler__OFFSET: usize = 2200;
pub const CGameUI__10__m_observerToggleablePanels__OFFSET: usize = 2208;
pub const CGameUI__10__m_consoleBackdropHeight__OFFSET: usize = 2240;
pub const CGameUI__10__m_auroraChatHandler__OFFSET: usize = 2248;
pub const CGameUI__10__m_lobbyEventsHandler__OFFSET: usize = 2328;
pub const CGameUI__10__s_indexedCustomHotkeyMappingMaxCount__OFFSET: usize = 0;
pub const CGameUI__10__s_customHotkeyEnabled__OFFSET: usize = 0;
pub const CGameUI__10__s_customHotkeyMappingTable__OFFSET: usize = 0;
pub const CGameUI__10__s_customHotkeyMappingTableNonHero__OFFSET: usize = 0;
pub const CGameUI__10__m_observerUnitToFollow__OFFSET: usize = 2376;

#[repr(C)]
pub struct UnitUIDefOld {
    pub _opaque: [u8; 3792],
}
pub const UnitUIDefOld__SIZE: usize = 3792;
pub const UnitUIDefOld__NAME: &str = "UnitUIDefOld";
pub const UnitUIDefOld__idStr__OFFSET: usize = 0;
pub const UnitUIDefOld__modelPath__OFFSET: usize = 8;
pub const UnitUIDefOld__modelPathTable__OFFSET: usize = 48;
pub const UnitUIDefOld__portraitModelPath__OFFSET: usize = 80;
pub const UnitUIDefOld__portraitModelPathTable__OFFSET: usize = 120;
pub const UnitUIDefOld__uberSplat__OFFSET: usize = 152;
pub const UnitUIDefOld__uberSplatTable__OFFSET: usize = 192;
pub const UnitUIDefOld__shadow__OFFSET: usize = 224;
pub const UnitUIDefOld__shadowTable__OFFSET: usize = 264;
pub const UnitUIDefOld__structureShadow__OFFSET: usize = 296;
pub const UnitUIDefOld__structureShadowTable__OFFSET: usize = 336;
pub const UnitUIDefOld__scoreScreenIcon__OFFSET: usize = 368;
pub const UnitUIDefOld__scoreScreenIconTable__OFFSET: usize = 408;
pub const UnitUIDefOld__description__OFFSET: usize = 440;
pub const UnitUIDefOld__descriptionTable__OFFSET: usize = 480;
pub const UnitUIDefOld__movementSoundLabel__OFFSET: usize = 512;
pub const UnitUIDefOld__movementSoundLabelTable__OFFSET: usize = 552;
pub const UnitUIDefOld__buildingSoundLabel__OFFSET: usize = 584;
pub const UnitUIDefOld__buildingSoundLabelTable__OFFSET: usize = 624;
pub const UnitUIDefOld__randomSoundLabel__OFFSET: usize = 656;
pub const UnitUIDefOld__randomSoundLabelTable__OFFSET: usize = 696;
pub const UnitUIDefOld__scaleFactor__OFFSET: usize = 728;
pub const UnitUIDefOld__scaleFactorTable__OFFSET: usize = 736;
pub const UnitUIDefOld__zOffset__OFFSET: usize = 768;
pub const UnitUIDefOld__zOffsetTable__OFFSET: usize = 776;
pub const UnitUIDefOld__shadowOffset__OFFSET: usize = 808;
pub const UnitUIDefOld__shadowOffsetTable__OFFSET: usize = 816;
pub const UnitUIDefOld__shadowSize__OFFSET: usize = 848;
pub const UnitUIDefOld__shadowSizeTable__OFFSET: usize = 856;
pub const UnitUIDefOld__shadowOnWater__OFFSET: usize = 888;
pub const UnitUIDefOld__shadowOnWaterTable__OFFSET: usize = 896;
pub const UnitUIDefOld__selCircOnWater__OFFSET: usize = 928;
pub const UnitUIDefOld__selCircOnWaterTable__OFFSET: usize = 936;
pub const UnitUIDefOld__maxPitch__OFFSET: usize = 968;
pub const UnitUIDefOld__maxPitchTable__OFFSET: usize = 976;
pub const UnitUIDefOld__maxRoll__OFFSET: usize = 1008;
pub const UnitUIDefOld__maxRollTable__OFFSET: usize = 1016;
pub const UnitUIDefOld__elevPoints__OFFSET: usize = 1048;
pub const UnitUIDefOld__elevPointsTable__OFFSET: usize = 1056;
pub const UnitUIDefOld__elevRadius__OFFSET: usize = 1088;
pub const UnitUIDefOld__elevRadiusTable__OFFSET: usize = 1096;
pub const UnitUIDefOld__armorType__OFFSET: usize = 1128;
pub const UnitUIDefOld__armorTypeTable__OFFSET: usize = 1136;
pub const UnitUIDefOld__modelColor__OFFSET: usize = 1168;
pub const UnitUIDefOld__modelColorTable__OFFSET: usize = 1176;
pub const UnitUIDefOld__modelScale__OFFSET: usize = 1208;
pub const UnitUIDefOld__modelScaleTable__OFFSET: usize = 1216;
pub const UnitUIDefOld__isStructure__OFFSET: usize = 1248;
pub const UnitUIDefOld__isStructureTable__OFFSET: usize = 1256;
pub const UnitUIDefOld__useNBMinimapIcon__OFFSET: usize = 1288;
pub const UnitUIDefOld__useNBMinimapIconTable__OFFSET: usize = 1296;
pub const UnitUIDefOld__hideHeroBar__OFFSET: usize = 1328;
pub const UnitUIDefOld__hideHeroBarTable__OFFSET: usize = 1336;
pub const UnitUIDefOld__hideHeroMinimap__OFFSET: usize = 1368;
pub const UnitUIDefOld__hideHeroMinimapTable__OFFSET: usize = 1376;
pub const UnitUIDefOld__hideOnMinimap__OFFSET: usize = 1408;
pub const UnitUIDefOld__hideOnMinimapTable__OFFSET: usize = 1416;
pub const UnitUIDefOld__hideHeroDeathMsg__OFFSET: usize = 1448;
pub const UnitUIDefOld__hideHeroDeathMsgTable__OFFSET: usize = 1456;
pub const UnitUIDefOld__preventPlacementMask__OFFSET: usize = 1488;
pub const UnitUIDefOld__preventPlacementMaskTable__OFFSET: usize = 1496;
pub const UnitUIDefOld__requirePlacementMask__OFFSET: usize = 1528;
pub const UnitUIDefOld__requirePlacementMaskTable__OFFSET: usize = 1536;
pub const UnitUIDefOld__fadeInRate__OFFSET: usize = 1568;
pub const UnitUIDefOld__fadeInRateTable__OFFSET: usize = 1576;
pub const UnitUIDefOld__fadeOutRate__OFFSET: usize = 1608;
pub const UnitUIDefOld__fadeOutRateTable__OFFSET: usize = 1616;
pub const UnitUIDefOld__revive__OFFSET: usize = 1648;
pub const UnitUIDefOld__reviveTable__OFFSET: usize = 1656;
pub const UnitUIDefOld__buttonPos__OFFSET: usize = 1688;
pub const UnitUIDefOld__buttonPosTable__OFFSET: usize = 1696;
pub const UnitUIDefOld__hotkey__OFFSET: usize = 1728;
pub const UnitUIDefOld__hotkeyTable__OFFSET: usize = 1752;
pub const UnitUIDefOld__xpFactor__OFFSET: usize = 1784;
pub const UnitUIDefOld__xpFactorTable__OFFSET: usize = 1808;
pub const UnitUIDefOld__rangedMissileSpeed__OFFSET: usize = 1840;
pub const UnitUIDefOld__rangedMissileSpeedTable__OFFSET: usize = 1864;
pub const UnitUIDefOld__rangedMissileArc__OFFSET: usize = 1896;
pub const UnitUIDefOld__rangedMissileArcTable__OFFSET: usize = 1920;
pub const UnitUIDefOld__rangedMissileHoming__OFFSET: usize = 1952;
pub const UnitUIDefOld__rangedMissileHomingTable__OFFSET: usize = 1976;
pub const UnitUIDefOld__dependencyOr__OFFSET: usize = 2008;
pub const UnitUIDefOld__dependencyOrTable__OFFSET: usize = 2032;
pub const UnitUIDefOld__awakenTip__OFFSET: usize = 2064;
pub const UnitUIDefOld__awakenTipTable__OFFSET: usize = 2088;
pub const UnitUIDefOld__reviveTip__OFFSET: usize = 2120;
pub const UnitUIDefOld__reviveTipTable__OFFSET: usize = 2144;
pub const UnitUIDefOld__properNames__OFFSET: usize = 2176;
pub const UnitUIDefOld__properNamesTable__OFFSET: usize = 2200;
pub const UnitUIDefOld__missileModelPath__OFFSET: usize = 2232;
pub const UnitUIDefOld__missileModelPathTable__OFFSET: usize = 2256;
pub const UnitUIDefOld__name__OFFSET: usize = 2288;
pub const UnitUIDefOld__nameTable__OFFSET: usize = 2312;
pub const UnitUIDefOld__casterUpgradeArt__OFFSET: usize = 2344;
pub const UnitUIDefOld__casterUpgradeArtTable__OFFSET: usize = 2368;
pub const UnitUIDefOld__casterUpgradeName__OFFSET: usize = 2400;
pub const UnitUIDefOld__casterUpgradeNameTable__OFFSET: usize = 2424;
pub const UnitUIDefOld__casterUpgradeTip__OFFSET: usize = 2456;
pub const UnitUIDefOld__casterUpgradeTipTable__OFFSET: usize = 2480;
pub const UnitUIDefOld__specialArt__OFFSET: usize = 2512;
pub const UnitUIDefOld__specialArtTable__OFFSET: usize = 2536;
pub const UnitUIDefOld__buttonArt__OFFSET: usize = 2568;
pub const UnitUIDefOld__buttonArtTable__OFFSET: usize = 2592;
pub const UnitUIDefOld__tooltip__OFFSET: usize = 2624;
pub const UnitUIDefOld__tooltipTable__OFFSET: usize = 2648;
pub const UnitUIDefOld__uberTooltip__OFFSET: usize = 2680;
pub const UnitUIDefOld__uberTooltipTable__OFFSET: usize = 2704;
pub const UnitUIDefOld__animProps__OFFSET: usize = 2736;
pub const UnitUIDefOld__animPropsTable__OFFSET: usize = 2760;
pub const UnitUIDefOld__attachmentAnimProps__OFFSET: usize = 2792;
pub const UnitUIDefOld__attachmentAnimPropsTable__OFFSET: usize = 2816;
pub const UnitUIDefOld__attachmentLinkProps__OFFSET: usize = 2848;
pub const UnitUIDefOld__attachmentLinkPropsTable__OFFSET: usize = 2872;
pub const UnitUIDefOld__boneProps__OFFSET: usize = 2904;
pub const UnitUIDefOld__bonePropsTable__OFFSET: usize = 2928;
pub const UnitUIDefOld__upgradesIds__OFFSET: usize = 2960;
pub const UnitUIDefOld__upgradesIdsTable__OFFSET: usize = 2992;
pub const UnitUIDefOld__buildsIds__OFFSET: usize = 3024;
pub const UnitUIDefOld__buildsIdsTable__OFFSET: usize = 3056;
pub const UnitUIDefOld__trainsIds__OFFSET: usize = 3088;
pub const UnitUIDefOld__trainsIdsTable__OFFSET: usize = 3120;
pub const UnitUIDefOld__researchesIds__OFFSET: usize = 3152;
pub const UnitUIDefOld__researchesIdsTable__OFFSET: usize = 3184;
pub const UnitUIDefOld__sellsUnitIds__OFFSET: usize = 3216;
pub const UnitUIDefOld__sellsUnitIdsTable__OFFSET: usize = 3248;
pub const UnitUIDefOld__sellsItemIds__OFFSET: usize = 3280;
pub const UnitUIDefOld__sellsItemIdsTable__OFFSET: usize = 3312;
pub const UnitUIDefOld__makesItemIds__OFFSET: usize = 3344;
pub const UnitUIDefOld__makesItemIdsTable__OFFSET: usize = 3376;
pub const UnitUIDefOld__reviveAtIds__OFFSET: usize = 3408;
pub const UnitUIDefOld__reviveAtIdsTable__OFFSET: usize = 3440;
pub const UnitUIDefOld__specialAttachProps__OFFSET: usize = 3472;
pub const UnitUIDefOld__specialAttachPropsTable__OFFSET: usize = 3504;
pub const UnitUIDefOld__requirementIds__OFFSET: usize = 3536;
pub const UnitUIDefOld__requirementIdsTable__OFFSET: usize = 3568;
pub const UnitUIDefOld__requirementAmounts__OFFSET: usize = 3600;
pub const UnitUIDefOld__requirementAmountsTable__OFFSET: usize = 3632;
pub const UnitUIDefOld__requirementHadIds__OFFSET: usize = 3664;
pub const UnitUIDefOld__requirementHadIdsTable__OFFSET: usize = 3696;
pub const UnitUIDefOld__requirementHadAmounts__OFFSET: usize = 3728;
pub const UnitUIDefOld__requirementHadAmountsTable__OFFSET: usize = 3760;

#[repr(C)]
pub struct CUnitStatusTextUI {
    pub _opaque: [u8; 512],
}
pub const CUnitStatusTextUI__SIZE: usize = 512;
pub const CUnitStatusTextUI__NAME: &str = "CUnitStatusTextUI";
pub const CUnitStatusTextUI__m_unit__OFFSET: usize = 480;
pub const CUnitStatusTextUI__m_shouldShow__OFFSET: usize = 488;
pub const CUnitStatusTextUI__m_backdrop__OFFSET: usize = 496;
pub const CUnitStatusTextUI__m_descText__OFFSET: usize = 504;

#[repr(C)]
pub struct CGameWar3__7 {
    pub _opaque: [u8; 12872],
}
pub const CGameWar3__7__SIZE: usize = 12872;
pub const CGameWar3__7__NAME: &str = "CGameWar3";
pub const CGameWar3__7__s_campaignArchiveFile__OFFSET: usize = 0;
pub const CGameWar3__7__anticheat_dummy_array_28__OFFSET: usize = 9640;
pub const CGameWar3__7__s_pGameWar3__OFFSET: usize = 0;
pub const CGameWar3__7__anticheat_dummy_array_29__OFFSET: usize = 9695;
pub const CGameWar3__7__m_mapScriptInstance__OFFSET: usize = 9784;
pub const CGameWar3__7__m_mapScriptInstanceId__OFFSET: usize = 9792;
pub const CGameWar3__7__m_mapScript__OFFSET: usize = 9800;
pub const CGameWar3__7__m_mapScriptId__OFFSET: usize = 9808;
pub const CGameWar3__7__m_executions__OFFSET: usize = 9816;
pub const CGameWar3__7__anticheat_dummy_array_30__OFFSET: usize = 9840;
pub const CGameWar3__7__m_gameIdMap__OFFSET: usize = 9864;
pub const CGameWar3__7__m_gameState__OFFSET: usize = 9872;
pub const CGameWar3__7__m_buildId__OFFSET: usize = 9880;
pub const CGameWar3__7__m_localPlayerId__OFFSET: usize = 9884;
pub const CGameWar3__7__m_replayLocalPlayerId__OFFSET: usize = 9886;
pub const CGameWar3__7__m_playerIdBitsToGhost__OFFSET: usize = 9888;
pub const CGameWar3__7__m_playerIdBitsToGhostSet__OFFSET: usize = 9892;
pub const CGameWar3__7__m_pMapSetup__OFFSET: usize = 9896;
pub const CGameWar3__7__m_pFog__OFFSET: usize = 9904;
pub const CGameWar3__7__m_gameName__OFFSET: usize = 9912;
pub const CGameWar3__7__anticheat_dummy_array_31__OFFSET: usize = 9936;
pub const CGameWar3__7__m_maxPlayerCount__OFFSET: usize = 9952;
pub const CGameWar3__7__m_playerCount__OFFSET: usize = 9956;
pub const CGameWar3__7__m_gameMaxPlayerCount__OFFSET: usize = 9960;
pub const CGameWar3__7__m_maxTeamCount__OFFSET: usize = 9964;
pub const CGameWar3__7__m_teams__OFFSET: usize = 9968;
pub const CGameWar3__7__m_players__OFFSET: usize = 9992;
pub const CGameWar3__7__m_startLocations__OFFSET: usize = 10224;
pub const CGameWar3__7__m_slotToPlayerId__OFFSET: usize = 11960;
pub const CGameWar3__7__m_levelToTransitionTo__OFFSET: usize = 12064;
pub const CGameWar3__7__m_modelCine__OFFSET: usize = 12088;
pub const CGameWar3__7__m_cine__OFFSET: usize = 12112;
pub const CGameWar3__7__m_gameOverMessage__OFFSET: usize = 12376;
pub const CGameWar3__7__m_switchingLevels__OFFSET: usize = 12400;
pub const CGameWar3__7__m_doScoreScreen__OFFSET: usize = 12401;
pub const CGameWar3__7__m_isReload__OFFSET: usize = 12402;
pub const CGameWar3__7__anticheat_dummy_array_32__OFFSET: usize = 12403;
pub const CGameWar3__7__m_wasScriptLoad__OFFSET: usize = 12410;
pub const CGameWar3__7__m_wasLoadGame__OFFSET: usize = 12411;
pub const CGameWar3__7__m_isCampaignMission__OFFSET: usize = 12412;
pub const CGameWar3__7__m_campaignIndex__OFFSET: usize = 12416;
pub const CGameWar3__7__m_gameSetup__OFFSET: usize = 12420;
pub const CGameWar3__7__m_usingDarkMask__OFFSET: usize = 12678;
pub const CGameWar3__7__m_unseenQuests__OFFSET: usize = 12679;
pub const CGameWar3__7__m_showAllUnitsCmdBar__OFFSET: usize = 12680;
pub const CGameWar3__7__m_isReplay__OFFSET: usize = 12681;
pub const CGameWar3__7__m_replayFile__OFFSET: usize = 12688;
pub const CGameWar3__7__m_isTournament__OFFSET: usize = 12712;
pub const CGameWar3__7__m_copySaveCount__OFFSET: usize = 12716;
pub const CGameWar3__7__m_finishedInit__OFFSET: usize = 12720;
pub const CGameWar3__7__m_didSaveGameFromScript__OFFSET: usize = 12721;
pub const CGameWar3__7__m_playingGame__OFFSET: usize = 12722;
pub const CGameWar3__7__m_FacialAnimationSetScopeID__OFFSET: usize = 12724;
pub const CGameWar3__7__m_pTriggerStrings__OFFSET: usize = 12728;
pub const CGameWar3__7__anticheat_dummy_array_33__OFFSET: usize = 12736;
pub const CGameWar3__7__m_messageLogContents__OFFSET: usize = 12776;
pub const CGameWar3__7__m_pGameHashTableManager__OFFSET: usize = 12800;
pub const CGameWar3__7__m_gameCacheManager__OFFSET: usize = 12808;
pub const CGameWar3__7__m_gameOverCallback__OFFSET: usize = 12816;
pub const CGameWar3__7__m_maxCheckpointSaves__OFFSET: usize = 12824;
pub const CGameWar3__7__m_pSaveThread__OFFSET: usize = 12832;
pub const CGameWar3__7__m_pCheckpointPopup__OFFSET: usize = 12840;
pub const CGameWar3__7__m_gamePlayStartTime__OFFSET: usize = 12848;
pub const CGameWar3__7__m_latency__OFFSET: usize = 12856;

#[repr(C)]
pub struct CGameSaveSplashDialog {
    pub _opaque: [u8; 672],
}
pub const CGameSaveSplashDialog__SIZE: usize = 672;
pub const CGameSaveSplashDialog__NAME: &str = "CGameSaveSplashDialog";
pub const CGameSaveSplashDialog__m_message__OFFSET: usize = 664;

#[repr(C)]
pub struct CGameResultDialog {
    pub _opaque: [u8; 696],
}
pub const CGameResultDialog__SIZE: usize = 696;
pub const CGameResultDialog__NAME: &str = "CGameResultDialog";
pub const CGameResultDialog__m_gameResultText__OFFSET: usize = 664;
pub const CGameResultDialog__m_continueButton__OFFSET: usize = 672;
pub const CGameResultDialog__m_restartButton__OFFSET: usize = 680;
pub const CGameResultDialog__m_quitButton__OFFSET: usize = 688;

#[repr(C)]
pub struct CGameOverMenu {
    pub _opaque: [u8; 40],
}
pub const CGameOverMenu__SIZE: usize = 40;
pub const CGameOverMenu__NAME: &str = "CGameOverMenu";
pub const CGameOverMenu__m_gameResultDialog__OFFSET: usize = 24;
pub const CGameOverMenu__m_doScoreScreen__OFFSET: usize = 32;
pub const CGameOverMenu__m_allowContinue__OFFSET: usize = 33;

#[repr(C)]
pub struct CPlayerStatisticsPanel {
    pub _opaque: [u8; 552],
}
pub const CPlayerStatisticsPanel__SIZE: usize = 552;
pub const CPlayerStatisticsPanel__NAME: &str = "CPlayerStatisticsPanel";
pub const CPlayerStatisticsPanel__m_enabled__OFFSET: usize = 504;
pub const CPlayerStatisticsPanel__m_playerRows__OFFSET: usize = 512;
pub const CPlayerStatisticsPanel__m_backdropFrame__OFFSET: usize = 536;
pub const CPlayerStatisticsPanel__m_backdropBottomFrame__OFFSET: usize = 544;

#[repr(C)]
pub struct CUnitStatusIconUIFactory {
    pub _opaque: [u8; 64],
}
pub const CUnitStatusIconUIFactory__SIZE: usize = 64;
pub const CUnitStatusIconUIFactory__NAME: &str = "CUnitStatusIconUIFactory";
pub const CUnitStatusIconUIFactory__m_instances__OFFSET: usize = 16;
pub const CUnitStatusIconUIFactory__m_pool__OFFSET: usize = 40;

#[repr(C)]
pub struct CPlayerWar3__7 {
    pub _opaque: [u8; 1960],
}
pub const CPlayerWar3__7__SIZE: usize = 1960;
pub const CPlayerWar3__7__NAME: &str = "CPlayerWar3";
pub const CPlayerWar3__7__s_CPlayerWar3_pool__OFFSET: usize = 0;
pub const CPlayerWar3__7__anticheat_dummy_array_24__OFFSET: usize = 88;
pub const CPlayerWar3__7__m_playerId__OFFSET: usize = 103;
pub const CPlayerWar3__7__m_playerName__OFFSET: usize = 104;
pub const CPlayerWar3__7__m_toonName__OFFSET: usize = 128;
pub const CPlayerWar3__7__m_protobufPlayerData__OFFSET: usize = 152;
pub const CPlayerWar3__7__m_slotNetPlayerId__OFFSET: usize = 336;
pub const CPlayerWar3__7__m_playerNamePrefix__OFFSET: usize = 344;
pub const CPlayerWar3__7__m_pAlliances__OFFSET: usize = 368;
pub const CPlayerWar3__7__m_pSelection__OFFSET: usize = 376;
pub const CPlayerWar3__7__m_playerStates__OFFSET: usize = 384;
pub const CPlayerWar3__7__m_lumberDivertTax__OFFSET: usize = 1016;
pub const CPlayerWar3__7__m_goldDivertTax__OFFSET: usize = 1116;
pub const CPlayerWar3__7__m_highestFood__OFFSET: usize = 1216;
pub const CPlayerWar3__7__m_highestFoodUsed__OFFSET: usize = 1220;
pub const CPlayerWar3__7__anticheat_dummy_array_25__OFFSET: usize = 1224;
pub const CPlayerWar3__7__m_heroKills__OFFSET: usize = 1240;
pub const CPlayerWar3__7__m_itemsObtained__OFFSET: usize = 1244;
pub const CPlayerWar3__7__m_mercsHired__OFFSET: usize = 1248;
pub const CPlayerWar3__7__m_teamColorIndex__OFFSET: usize = 1252;
pub const CPlayerWar3__7__m_controller__OFFSET: usize = 1256;
pub const CPlayerWar3__7__m_startLocIndex__OFFSET: usize = 1260;
pub const CPlayerWar3__7__m_difficulty__OFFSET: usize = 1264;
pub const CPlayerWar3__7__m_slotState__OFFSET: usize = 1268;
pub const CPlayerWar3__7__m_teamIndex__OFFSET: usize = 1272;
pub const CPlayerWar3__7__m_numActions__OFFSET: usize = 1276;
pub const CPlayerWar3__7__m_racePref__OFFSET: usize = 1280;
pub const CPlayerWar3__7__m_race__OFFSET: usize = 1284;
pub const CPlayerWar3__7__m_realTimeAPM__OFFSET: usize = 1288;
pub const CPlayerWar3__7__m_capChangedListener__OFFSET: usize = 1384;
pub const CPlayerWar3__7__m_usedChangedListener__OFFSET: usize = 1392;
pub const CPlayerWar3__7__m_foodUsedUpkeepListener__OFFSET: usize = 1400;
pub const CPlayerWar3__7__m_lumberChangedListener__OFFSET: usize = 1408;
pub const CPlayerWar3__7__m_goldChangedListener__OFFSET: usize = 1416;
pub const CPlayerWar3__7__m_goldUpkeepChanged__OFFSET: usize = 1424;
pub const CPlayerWar3__7__m_lumberUpkeepChanged__OFFSET: usize = 1432;
pub const CPlayerWar3__7__m_handicap__OFFSET: usize = 1440;
pub const CPlayerWar3__7__m_referee__OFFSET: usize = 1512;
pub const CPlayerWar3__7__m_deadHeroAge__OFFSET: usize = 1520;
pub const CPlayerWar3__7__m_deadHeroes__OFFSET: usize = 1544;
pub const CPlayerWar3__7__m_revivableFlags__OFFSET: usize = 1568;
pub const CPlayerWar3__7__anticheat_dummy_array_26__OFFSET: usize = 1572;
pub const CPlayerWar3__7__m_pTechTree__OFFSET: usize = 1584;
pub const CPlayerWar3__7__m_fogMask__OFFSET: usize = 1592;
pub const CPlayerWar3__7__m_ai__OFFSET: usize = 1596;
pub const CPlayerWar3__7__m_bestHeroes__OFFSET: usize = 1608;
pub const CPlayerWar3__7__m_leaderboard__OFFSET: usize = 1632;
pub const CPlayerWar3__7__m_leaveGameTimer__OFFSET: usize = 1648;
pub const CPlayerWar3__7__anticheat_dummy_array_27__OFFSET: usize = 1680;
pub const CPlayerWar3__7__m_doScoreScreen__OFFSET: usize = 1708;
pub const CPlayerWar3__7__m_allowContinue__OFFSET: usize = 1709;
pub const CPlayerWar3__7__m_crippled__OFFSET: usize = 1712;
pub const CPlayerWar3__7__m_gameOverMessage__OFFSET: usize = 1720;
pub const CPlayerWar3__7__m_showInScoreScreen__OFFSET: usize = 1744;
pub const CPlayerWar3__7__m_cachedScore__OFFSET: usize = 1748;
pub const CPlayerWar3__7__m_cachedScoreIsCached__OFFSET: usize = 1752;
pub const CPlayerWar3__7__m_heroCount__OFFSET: usize = 1756;
pub const CPlayerWar3__7__m_bnetPlayerId__OFFSET: usize = 1760;
pub const CPlayerWar3__7__m_heroPickOrder__OFFSET: usize = 1768;
pub const CPlayerWar3__7__m_unitKills__OFFSET: usize = 1792;
pub const CPlayerWar3__7__m_unitsCreated__OFFSET: usize = 1824;
pub const CPlayerWar3__7__m_abilityInfo__OFFSET: usize = 1856;
pub const CPlayerWar3__7__m_upkeepTime__OFFSET: usize = 1888;
pub const CPlayerWar3__7__m_lastUpkeepTime__OFFSET: usize = 1920;
pub const CPlayerWar3__7__m_playerItemUseMap__OFFSET: usize = 1928;

#[repr(C)]
pub struct CSelectionCircleTokenizer {
    pub _opaque: [u8; 8],
}
pub const CSelectionCircleTokenizer__SIZE: usize = 8;
pub const CSelectionCircleTokenizer__NAME: &str = "CSelectionCircleTokenizer";
pub const CSelectionCircleTokenizer__s_toker__OFFSET: usize = 0;

#[repr(C)]
pub struct CUnitTip {
    pub _opaque: [u8; 544],
}
pub const CUnitTip__SIZE: usize = 544;
pub const CUnitTip__NAME: &str = "CUnitTip";
pub const CUnitTip__m_pSelectable__OFFSET: usize = 480;
pub const CUnitTip__m_backdrop__OFFSET: usize = 488;
pub const CUnitTip__m_name__OFFSET: usize = 496;
pub const CUnitTip__m_descText__OFFSET: usize = 504;
pub const CUnitTip__m_playerText__OFFSET: usize = 512;
pub const CUnitTip__m_hasNameText__OFFSET: usize = 520;
pub const CUnitTip__m_hasDescText__OFFSET: usize = 521;
pub const CUnitTip__m_hasPlayerText__OFFSET: usize = 522;
pub const CUnitTip__m_needsReposition__OFFSET: usize = 523;
pub const CUnitTip__m_autoPosition__OFFSET: usize = 524;
pub const CUnitTip__m_showDescText__OFFSET: usize = 525;
pub const CUnitTip__m_DescUpdateFunc__OFFSET: usize = 528;

#[repr(C)]
pub struct CPlayerStatisticsRow {
    pub _opaque: [u8; 600],
}
pub const CPlayerStatisticsRow__SIZE: usize = 600;
pub const CPlayerStatisticsRow__NAME: &str = "CPlayerStatisticsRow";
pub const CPlayerStatisticsRow__m_playerNameText__OFFSET: usize = 504;
pub const CPlayerStatisticsRow__m_teamNumberText__OFFSET: usize = 512;
pub const CPlayerStatisticsRow__m_goldText__OFFSET: usize = 520;
pub const CPlayerStatisticsRow__m_lumberText__OFFSET: usize = 528;
pub const CPlayerStatisticsRow__m_supplyText__OFFSET: usize = 536;
pub const CPlayerStatisticsRow__m_APMText__OFFSET: usize = 544;
pub const CPlayerStatisticsRow__m_playerColourBackdrop__OFFSET: usize = 552;
pub const CPlayerStatisticsRow__m_playerRaceBanner__OFFSET: usize = 560;
pub const CPlayerStatisticsRow__m_rowBorder__OFFSET: usize = 568;
pub const CPlayerStatisticsRow__m_backdropMiddleFrame__OFFSET: usize = 576;
pub const CPlayerStatisticsRow__m_buttonEventObserver__OFFSET: usize = 584;
pub const CPlayerStatisticsRow__m_enabled__OFFSET: usize = 592;
pub const CPlayerStatisticsRow__m_playerIndex__OFFSET: usize = 596;

#[repr(C)]
pub struct CGameUI__11 {
    pub _opaque: [u8; 2384],
}
pub const CGameUI__11__SIZE: usize = 2384;
pub const CGameUI__11__NAME: &str = "CGameUI";
pub const CGameUI__11__m_transitionState__OFFSET: usize = 704;
pub const CGameUI__11__m_cachedFonts__OFFSET: usize = 712;
pub const CGameUI__11__NUMBER_OF_PLAYER_ROWS__OFFSET: usize = 0;
pub const CGameUI__11__FIRST_ROW_Y_OFFSET__OFFSET: usize = 0;
pub const CGameUI__11__ROW_SPACING__OFFSET: usize = 0;
pub const CGameUI__11__m_userControl__OFFSET: usize = 784;
pub const CGameUI__11__m_userUI__OFFSET: usize = 785;
pub const CGameUI__11__m_currentInputMode__OFFSET: usize = 792;
pub const CGameUI__11__m_inputModeStack__OFFSET: usize = 800;
pub const CGameUI__11__m_sharedTooltip__OFFSET: usize = 824;
pub const CGameUI__11__m_uberTooltip__OFFSET: usize = 832;
pub const CGameUI__11__m_showUberTips__OFFSET: usize = 840;
pub const CGameUI__11__m_autosaveReplay__OFFSET: usize = 841;
pub const CGameUI__11__m_healthBarsMode__OFFSET: usize = 844;
pub const CGameUI__11__m_forceShowHealthBars__OFFSET: usize = 848;
pub const CGameUI__11__m_minimapSignals__OFFSET: usize = 849;
pub const CGameUI__11__m_useColoredHealthBars__OFFSET: usize = 850;
pub const CGameUI__11__m_useHeroFrames__OFFSET: usize = 851;
pub const CGameUI__11__m_showHeroLevelFrame__OFFSET: usize = 852;
pub const CGameUI__11__m_showNumericCooldown__OFFSET: usize = 853;
pub const CGameUI__11__m_showDenyIcon__OFFSET: usize = 854;
pub const CGameUI__11__m_showGoldmineUnitCounter__OFFSET: usize = 855;
pub const CGameUI__11__m_enableObserverUiV2__OFFSET: usize = 856;
pub const CGameUI__11__m_asyncPlayerData__OFFSET: usize = 860;
pub const CGameUI__11__m_targetMode__OFFSET: usize = 960;
pub const CGameUI__11__m_selectMode__OFFSET: usize = 968;
pub const CGameUI__11__m_dragSelectMode__OFFSET: usize = 976;
pub const CGameUI__11__m_dragScrollMode__OFFSET: usize = 984;
pub const CGameUI__11__m_buildMode__OFFSET: usize = 992;
pub const CGameUI__11__m_signalMode__OFFSET: usize = 1000;
pub const CGameUI__11__m_escapeMenu__OFFSET: usize = 1008;
pub const CGameUI__11__m_gameOverMenu__OFFSET: usize = 1016;
pub const CGameUI__11__m_suspendIndicator__OFFSET: usize = 1024;
pub const CGameUI__11__m_unresponsiveIndicator__OFFSET: usize = 1032;
pub const CGameUI__11__m_allianceMode__OFFSET: usize = 1040;
pub const CGameUI__11__m_chatMode__OFFSET: usize = 1048;
pub const CGameUI__11__m_questMode__OFFSET: usize = 1056;
pub const CGameUI__11__anticheat_dummy_array_30__OFFSET: usize = 1064;
pub const CGameUI__11__m_scriptDialogMode__OFFSET: usize = 1152;
pub const CGameUI__11__m_unresponsiveDelayTimerId__OFFSET: usize = 1160;
pub const CGameUI__11__m_selectionManager__OFFSET: usize = 1168;
pub const CGameUI__11__m_dragScrollManager__OFFSET: usize = 1176;
pub const CGameUI__11__m_camera__OFFSET: usize = 1184;
pub const CGameUI__11__m_paused__OFFSET: usize = 1192;
pub const CGameUI__11__m_uiPaused__OFFSET: usize = 1193;
pub const CGameUI__11__m_scriptPaused__OFFSET: usize = 1194;
pub const CGameUI__11__m_disableUpkeepMsg__OFFSET: usize = 1195;
pub const CGameUI__11__m_upkeepLvlSave__OFFSET: usize = 1196;
pub const CGameUI__11__m_refresh__OFFSET: usize = 1200;
pub const CGameUI__11__m_refreshWaypoints__OFFSET: usize = 1201;
pub const CGameUI__11__m_deferCleanSelUI__OFFSET: usize = 1202;
pub const CGameUI__11__m_refreshInfoBar__OFFSET: usize = 1203;
pub const CGameUI__11__m_refreshCommandBar__OFFSET: usize = 1204;
pub const CGameUI__11__m_refreshHeroBar__OFFSET: usize = 1205;
pub const CGameUI__11__m_popSubMenu__OFFSET: usize = 1206;
pub const CGameUI__11__m_activeValidOrder__OFFSET: usize = 1208;
pub const CGameUI__11__m_cineForceSubtitles__OFFSET: usize = 1212;
pub const CGameUI__11__m_cinematicPanelsHidden__OFFSET: usize = 1213;
pub const CGameUI__11__m_dragSelectEnabled__OFFSET: usize = 1214;
pub const CGameUI__11__m_dragSelectUIEnabled__OFFSET: usize = 1215;
pub const CGameUI__11__m_preSelectEnabled__OFFSET: usize = 1216;
pub const CGameUI__11__m_preSelectUIEnabled__OFFSET: usize = 1217;
pub const CGameUI__11__m_selectEnabled__OFFSET: usize = 1218;
pub const CGameUI__11__m_selectUIEnabled__OFFSET: usize = 1219;
pub const CGameUI__11__m_observerModeEnabled__OFFSET: usize = 1220;
pub const CGameUI__11__m_observerModeV2Active__OFFSET: usize = 1221;
pub const CGameUI__11__m_wideScreenAutoPosition__OFFSET: usize = 1222;
pub const CGameUI__11__m_fadeTimer__OFFSET: usize = 1224;
pub const CGameUI__11__m_cinematicFadeTimer__OFFSET: usize = 1328;
pub const CGameUI__11__anticheat_dummy_array_31__OFFSET: usize = 1432;
pub const CGameUI__11__m_frameParentTimer__OFFSET: usize = 1440;
pub const CGameUI__11__m_simpleFadeTimer__OFFSET: usize = 1544;
pub const CGameUI__11__m_attackNotify__OFFSET: usize = 1552;
pub const CGameUI__11__m_warpNotify__OFFSET: usize = 1576;
pub const CGameUI__11__m_timerDialogs__OFFSET: usize = 1608;
pub const CGameUI__11__m_leaderboard__OFFSET: usize = 1632;
pub const CGameUI__11__anticheat_dummy_array_32__OFFSET: usize = 1640;
pub const CGameUI__11__m_multiboard__OFFSET: usize = 1648;
pub const CGameUI__11__m_suspendMultiboardDisplay__OFFSET: usize = 1656;
pub const CGameUI__11__anticheat_dummy_array_33__OFFSET: usize = 1657;
pub const CGameUI__11__m_scriptDialog__OFFSET: usize = 1728;
pub const CGameUI__11__m_replaySelection__OFFSET: usize = 1736;
pub const CGameUI__11__m_unitStatusIconUIFactory__OFFSET: usize = 1744;
pub const CGameUI__11__m_unitStatusTexts__OFFSET: usize = 1752;
pub const CGameUI__11__m_worldFrame__OFFSET: usize = 1776;
pub const CGameUI__11__m_infoBar__OFFSET: usize = 1784;
pub const CGameUI__11__m_commandBar__OFFSET: usize = 1792;
pub const CGameUI__11__m_miniMap__OFFSET: usize = 1800;
pub const CGameUI__11__m_resourceBar__OFFSET: usize = 1808;
pub const CGameUI__11__m_upperButtonBar__OFFSET: usize = 1816;
pub const CGameUI__11__m_observerButtonBar__OFFSET: usize = 1824;
pub const CGameUI__11__m_playerStatisticsPanel__OFFSET: usize = 1832;
pub const CGameUI__11__m_heroPanel__OFFSET: usize = 1840;
pub const CGameUI__11__m_productionPanel__OFFSET: usize = 1848;
pub const CGameUI__11__m_minimapButtonBar__OFFSET: usize = 1856;
pub const CGameUI__11__m_trackingStub__OFFSET: usize = 1864;
pub const CGameUI__11__m_heroBar__OFFSET: usize = 1872;
pub const CGameUI__11__m_peonBar__OFFSET: usize = 1880;
pub const CGameUI__11__m_messageBox__OFFSET: usize = 1888;
pub const CGameUI__11__m_unitMessageBox__OFFSET: usize = 1896;
pub const CGameUI__11__m_chatMessageBox__OFFSET: usize = 1904;
pub const CGameUI__11__m_topMessageBox__OFFSET: usize = 1912;
pub const CGameUI__11__m_currentPortraitButton__OFFSET: usize = 1920;
pub const CGameUI__11__m_timeOfDayIndicator__OFFSET: usize = 1928;
pub const CGameUI__11__m_chatEditBar__OFFSET: usize = 1936;
pub const CGameUI__11__m_cinematicPanel__OFFSET: usize = 1944;
pub const CGameUI__11__m_lastBuffBar__OFFSET: usize = 1952;
pub const CGameUI__11__m_observerBar__OFFSET: usize = 1960;
pub const CGameUI__11__m_miniMapSignalButton__OFFSET: usize = 1968;
pub const CGameUI__11__m_miniMapTerrainButton__OFFSET: usize = 1976;
pub const CGameUI__11__m_miniMapAllyButton__OFFSET: usize = 1984;
pub const CGameUI__11__m_miniMapCreepButton__OFFSET: usize = 1992;
pub const CGameUI__11__m_formationButton__OFFSET: usize = 2000;
pub const CGameUI__11__m_consoleFrame__OFFSET: usize = 2008;
pub const CGameUI__11__m_simpleConsoleFrame__OFFSET: usize = 2016;
pub const CGameUI__11__m_simpleConsoleBottomBarFrame__OFFSET: usize = 2024;
pub const CGameUI__11__m_simpleConsoleBottomBarFrameOverlay__OFFSET: usize = 2032;
pub const CGameUI__11__m_frameUIParent__OFFSET: usize = 2040;
pub const CGameUI__11__top__OFFSET: usize = 2048;
pub const CGameUI__11__topInGame__OFFSET: usize = 2056;
pub const CGameUI__11__m_simpleUIParent__OFFSET: usize = 2064;
pub const CGameUI__11__m_observerUIParent__OFFSET: usize = 2072;
pub const CGameUI__11__m_observerInfoPanelBackground__OFFSET: usize = 2080;
pub const CGameUI__11__m_observerInfoPanelBackpack__OFFSET: usize = 2088;
pub const CGameUI__11__m_consoleBackdropFrame__OFFSET: usize = 2096;
pub const CGameUI__11__m_minimapTextureBackground__OFFSET: usize = 2104;
pub const CGameUI__11__m_infoPanelTexture__OFFSET: usize = 2112;
pub const CGameUI__11__m_quickSaveKey__OFFSET: usize = 2120;
pub const CGameUI__11__m_quickLoadKey__OFFSET: usize = 2124;
pub const CGameUI__11__m_quickHelpKey__OFFSET: usize = 2128;
pub const CGameUI__11__m_quickOptionsKey__OFFSET: usize = 2132;
pub const CGameUI__11__m_quickQuitKey__OFFSET: usize = 2136;
pub const CGameUI__11__m_minimapSignalKey__OFFSET: usize = 2140;
pub const CGameUI__11__m_minimapTerrainKey__OFFSET: usize = 2144;
pub const CGameUI__11__m_minimapColorsKey__OFFSET: usize = 2148;
pub const CGameUI__11__m_minimapCreepKey__OFFSET: usize = 2152;
pub const CGameUI__11__m_formationToggleKey__OFFSET: usize = 2156;
pub const CGameUI__11__m_observerToggleAllPanelsKey__OFFSET: usize = 2160;
pub const CGameUI__11__m_observerProductionModeKey__OFFSET: usize = 2164;
pub const CGameUI__11__m_observerUnitsModeKey__OFFSET: usize = 2168;
pub const CGameUI__11__m_mouseDownHandler__OFFSET: usize = 2176;
pub const CGameUI__11__m_mouseWheelHandler__OFFSET: usize = 2184;
pub const CGameUI__11__m_mouseUpHandler__OFFSET: usize = 2192;
pub const CGameUI__11__m_mouseMoveHandler__OFFSET: usize = 2200;
pub const CGameUI__11__m_observerToggleablePanels__OFFSET: usize = 2208;
pub const CGameUI__11__m_consoleBackdropHeight__OFFSET: usize = 2240;
pub const CGameUI__11__m_auroraChatHandler__OFFSET: usize = 2248;
pub const CGameUI__11__m_lobbyEventsHandler__OFFSET: usize = 2328;
pub const CGameUI__11__s_indexedCustomHotkeyMappingMaxCount__OFFSET: usize = 0;
pub const CGameUI__11__s_customHotkeyEnabled__OFFSET: usize = 0;
pub const CGameUI__11__s_customHotkeyMappingTable__OFFSET: usize = 0;
pub const CGameUI__11__s_customHotkeyMappingTableNonHero__OFFSET: usize = 0;
pub const CGameUI__11__m_observerUnitToFollow__OFFSET: usize = 2376;

#[repr(C)]
pub struct CWorldObjects {
    pub _opaque: [u8; 296],
}
pub const CWorldObjects__SIZE: usize = 296;
pub const CWorldObjects__NAME: &str = "CWorldObjects";
pub const CWorldObjects__mModels__OFFSET: usize = 16;
pub const CWorldObjects__mWalkables__OFFSET: usize = 40;
pub const CWorldObjects__mCollisionSets__OFFSET: usize = 64;
pub const CWorldObjects__mVisibleModels__OFFSET: usize = 88;
pub const CWorldObjects__mBuckets__OFFSET: usize = 112;
pub const CWorldObjects__mBucketsSize__OFFSET: usize = 136;
pub const CWorldObjects__mBucketVisible__OFFSET: usize = 144;
pub const CWorldObjects__mToBeDeleted__OFFSET: usize = 160;
pub const CWorldObjects__mShadowMapList__OFFSET: usize = 184;
pub const CWorldObjects__mTerrain__OFFSET: usize = 208;
pub const CWorldObjects__mLevel__OFFSET: usize = 216;
pub const CWorldObjects__mLoading__OFFSET: usize = 224;
pub const CWorldObjects__mNeedsSaved__OFFSET: usize = 225;
pub const CWorldObjects__mCurrentFrame__OFFSET: usize = 228;
pub const CWorldObjects__mFreeModels__OFFSET: usize = 232;
pub const CWorldObjects__mAnimating__OFFSET: usize = 233;
pub const CWorldObjects__mShowPaste__OFFSET: usize = 234;
pub const CWorldObjects__mPasteAlign__OFFSET: usize = 236;
pub const CWorldObjects__mPasteCenter__OFFSET: usize = 240;
pub const CWorldObjects__mLastAddObjectUndo__OFFSET: usize = 256;
pub const CWorldObjects__mEnableUberSplats__OFFSET: usize = 264;
pub const CWorldObjects__mExtraInfoDisplay__OFFSET: usize = 265;
pub const CWorldObjects__mPrevObjectID__OFFSET: usize = 268;
pub const CWorldObjects__mLastCameraPos__OFFSET: usize = 272;
pub const CWorldObjects__mLastCameraTarg__OFFSET: usize = 284;

#[repr(C)]
pub struct CUnit__5 {
    pub _opaque: [u8; 1864],
}
pub const CUnit__5__SIZE: usize = 1864;
pub const CUnit__5__NAME: &str = "CUnit";
pub const CUnit__5__s_CUnit_pool__OFFSET: usize = 0;
pub const CUnit__5__m_unit_paused_count__OFFSET: usize = 440;
pub const CUnit__5__m_PlayingDialogSound__OFFSET: usize = 444;
pub const CUnit__5__m_owner__OFFSET: usize = 448;
pub const CUnit__5__m_flags0__OFFSET: usize = 452;
pub const CUnit__5__m_flags1__OFFSET: usize = 456;
pub const CUnit__5__m_foodAndTechTreeFlags__OFFSET: usize = 460;
pub const CUnit__5__m_minimapFlashTimer__OFFSET: usize = 464;
pub const CUnit__5__m_damage_dealt__OFFSET: usize = 496;
pub const CUnit__5__m_self_damage__OFFSET: usize = 500;
pub const CUnit__5__m_healing_done__OFFSET: usize = 504;
pub const CUnit__5__m_time_stopped__OFFSET: usize = 508;
pub const CUnit__5__m_total_kills__OFFSET: usize = 512;
pub const CUnit__5__m_self_kills__OFFSET: usize = 516;
pub const CUnit__5__m_hero_kills__OFFSET: usize = 520;
pub const CUnit__5__m_building_kills__OFFSET: usize = 524;
pub const CUnit__5__m_override_ability_id_stats_credit__OFFSET: usize = 528;
pub const CUnit__5__m_override_item_id_stats_credit__OFFSET: usize = 532;
pub const CUnit__5__m_unit_stats_override__OFFSET: usize = 536;
pub const CUnit__5__m_caller__OFFSET: usize = 544;
pub const CUnit__5__m_lastAttackNotificationTime__OFFSET: usize = 576;
pub const CUnit__5__m_lastAttackNotificationSoundTime__OFFSET: usize = 580;
pub const CUnit__5__m_life__OFFSET: usize = 584;
pub const CUnit__5__m_death__OFFSET: usize = 608;
pub const CUnit__5__m_total_life_lost__OFFSET: usize = 616;
pub const CUnit__5__m_stored_time_when_last_alive__OFFSET: usize = 620;
pub const CUnit__5__m_time_alive_ms__OFFSET: usize = 624;
pub const CUnit__5__m_number_of_deaths__OFFSET: usize = 628;
pub const CUnit__5__m_life_regen_rate__OFFSET: usize = 632;
pub const CUnit__5__m_life_regen__OFFSET: usize = 648;
pub const CUnit__5__m_life_regen_type__OFFSET: usize = 656;
pub const CUnit__5__m_mana__OFFSET: usize = 664;
pub const CUnit__5__m_ai_mana__OFFSET: usize = 688;
pub const CUnit__5__m_mana_regen_rate__OFFSET: usize = 704;
pub const CUnit__5__m_mana_regen__OFFSET: usize = 720;
pub const CUnit__5__m_total_mana_spent__OFFSET: usize = 728;
pub const CUnit__5__m_defense__OFFSET: usize = 736;
pub const CUnit__5__m_battle_type__OFFSET: usize = 752;
pub const CUnit__5__m_invulnerable_count__OFFSET: usize = 756;
pub const CUnit__5__m_auto_attack_ignore_count__OFFSET: usize = 760;
pub const CUnit__5__m_fogFlash__OFFSET: usize = 764;
pub const CUnit__5__m_sight__OFFSET: usize = 776;
pub const CUnit__5__m_sightRangeCached__OFFSET: usize = 800;
pub const CUnit__5__m_sightMod__OFFSET: usize = 808;
pub const CUnit__5__m_invisibility_count__OFFSET: usize = 816;
pub const CUnit__5__m_invisibility_percent__OFFSET: usize = 824;
pub const CUnit__5__m_invis_modify__OFFSET: usize = 848;
pub const CUnit__5__m_invis_listen__OFFSET: usize = 856;
pub const CUnit__5__m_detectedData__OFFSET: usize = 864;
pub const CUnit__5__m_sharedVisionData__OFFSET: usize = 876;
pub const CUnit__5__m_sharedVisionDataCached__OFFSET: usize = 888;
pub const CUnit__5__m_sharedVisionTransitiveDataCached__OFFSET: usize = 892;
pub const CUnit__5__m_frost_count__OFFSET: usize = 896;
pub const CUnit__5__m_stone_count__OFFSET: usize = 900;
pub const CUnit__5__m_ethereal_count__OFFSET: usize = 904;
pub const CUnit__5__m_magic_immunity_count__OFFSET: usize = 908;
pub const CUnit__5__m_dispel_immunity_count__OFFSET: usize = 912;
pub const CUnit__5__m_fog_radius__OFFSET: usize = 920;
pub const CUnit__5__m_pos__OFFSET: usize = 936;
pub const CUnit__5__m_exp_level__OFFSET: usize = 960;
pub const CUnit__5__MaxNameLength__OFFSET: usize = 0;
pub const CUnit__5__m_unit_proper_name__OFFSET: usize = 964;
pub const CUnit__5__m_pick_order__OFFSET: usize = 1224;
pub const CUnit__5__m_task_stack__OFFSET: usize = 1228;
pub const CUnit__5__m_task_timer__OFFSET: usize = 1240;
pub const CUnit__5__m_action__OFFSET: usize = 1272;
pub const CUnit__5__m_uninterruptable_count__OFFSET: usize = 1276;
pub const CUnit__5__m_order_head__OFFSET: usize = 1280;
pub const CUnit__5__m_order_tail__OFFSET: usize = 1292;
pub const CUnit__5__order_count__OFFSET: usize = 1304;
pub const CUnit__5__m_gold_bounty_dice__OFFSET: usize = 1308;
pub const CUnit__5__m_gold_bounty_sides__OFFSET: usize = 1312;
pub const CUnit__5__m_gold_bounty_plus__OFFSET: usize = 1316;
pub const CUnit__5__m_wood_bounty_dice__OFFSET: usize = 1320;
pub const CUnit__5__m_wood_bounty_sides__OFFSET: usize = 1324;
pub const CUnit__5__m_wood_bounty_plus__OFFSET: usize = 1328;
pub const CUnit__5__m_cast_point__OFFSET: usize = 1336;
pub const CUnit__5__m_cast_back_swing__OFFSET: usize = 1352;
pub const CUnit__5__m_abilities__OFFSET: usize = 1368;
pub const CUnit__5__m_disabled_count__OFFSET: usize = 1380;
pub const CUnit__5__m_disabled_hidden_magic_count__OFFSET: usize = 1384;
pub const CUnit__5__m_silence_count__OFFSET: usize = 1388;
pub const CUnit__5__anticheat_dummy_array_22__OFFSET: usize = 1392;
pub const CUnit__5__m_disabled_magic_count__OFFSET: usize = 1404;
pub const CUnit__5__m_disabled_hidden_count__OFFSET: usize = 1408;
pub const CUnit__5__m_abil_paused_count__OFFSET: usize = 1412;
pub const CUnit__5__m_disabled_hidden_physical_count__OFFSET: usize = 1416;
pub const CUnit__5__m_disabled_physical_count__OFFSET: usize = 1420;
pub const CUnit__5__anticheat_dummy_array_23__OFFSET: usize = 1424;
pub const CUnit__5__m_buff_paused_count__OFFSET: usize = 1432;
pub const CUnit__5__m_abil_inv__OFFSET: usize = 1440;
pub const CUnit__5__m_abil_hero__OFFSET: usize = 1448;
pub const CUnit__5__m_abil_move__OFFSET: usize = 1456;
pub const CUnit__5__anticheat_dummy_array_24__OFFSET: usize = 1464;
pub const CUnit__5__m_abil_attack__OFFSET: usize = 1472;
pub const CUnit__5__m_abil_build__OFFSET: usize = 1480;
pub const CUnit__5__m_base_priority__OFFSET: usize = 1488;
pub const CUnit__5__m_death_time__OFFSET: usize = 1504;
pub const CUnit__5__s_minUnitSpeed__OFFSET: usize = 0;
pub const CUnit__5__s_maxUnitSpeed__OFFSET: usize = 0;
pub const CUnit__5__s_minBldgSpeed__OFFSET: usize = 0;
pub const CUnit__5__s_maxBldgSpeed__OFFSET: usize = 0;
pub const CUnit__5__s_buildingKillsGiveExp__OFFSET: usize = 0;
pub const CUnit__5__s_globalExperience__OFFSET: usize = 0;
pub const CUnit__5__s_maxLevelHeroesDrainExp__OFFSET: usize = 0;
pub const CUnit__5__m_display_height__OFFSET: usize = 1520;
pub const CUnit__5__m_display_height_modifier__OFFSET: usize = 1544;
pub const CUnit__5__anticheat_dummy_array_25__OFFSET: usize = 1552;
pub const CUnit__5__m_last_cluster_birth_tag__OFFSET: usize = 1560;
pub const CUnit__5__m_last_cluster_presence_tag__OFFSET: usize = 1564;
pub const CUnit__5__m_move_type__OFFSET: usize = 1568;
pub const CUnit__5__m_ground_count__OFFSET: usize = 1572;
pub const CUnit__5__m_launch_offset__OFFSET: usize = 1576;
pub const CUnit__5__m_fly_height__OFFSET: usize = 1592;
pub const CUnit__5__m_impact_swim__OFFSET: usize = 1616;
pub const CUnit__5__anticheat_dummy_array_26__OFFSET: usize = 1620;
pub const CUnit__5__m_launch_swim_z__OFFSET: usize = 1624;
pub const CUnit__5__m_impact_offset__OFFSET: usize = 1628;
pub const CUnit__5__m_category__OFFSET: usize = 1632;
pub const CUnit__5__m_target_type_flags__OFFSET: usize = 1636;
pub const CUnit__5__m_cargo_size__OFFSET: usize = 1640;
pub const CUnit__5__m_killedBy__OFFSET: usize = 1644;
pub const CUnit__5__m_shadow__OFFSET: usize = 1656;
pub const CUnit__5__m_shadowWater__OFFSET: usize = 1660;
pub const CUnit__5__anticheat_dummy_array_27__OFFSET: usize = 1664;
pub const CUnit__5__m_shadowAboveWater__OFFSET: usize = 1671;
pub const CUnit__5__m_shadowShow__OFFSET: usize = 1672;
pub const CUnit__5__m_showOccMark__OFFSET: usize = 1673;
pub const CUnit__5__m_allySelectionImage__OFFSET: usize = 1676;
pub const CUnit__5__m_occluded__OFFSET: usize = 1680;
pub const CUnit__5__anticheat_dummy_array_28__OFFSET: usize = 1681;
pub const CUnit__5__m_occlusionMark__OFFSET: usize = 1684;
pub const CUnit__5__m_jassUserData__OFFSET: usize = 1688;
pub const CUnit__5__m_foliageExclusionId__OFFSET: usize = 1692;
pub const CUnit__5__m_uiFlags__OFFSET: usize = 1696;
pub const CUnit__5__m_cachedPos__OFFSET: usize = 1700;
pub const CUnit__5__m_cachedFacing__OFFSET: usize = 1712;
pub const CUnit__5__m_cachedOrientation__OFFSET: usize = 1716;
pub const CUnit__5__m_lookAtTarget__OFFSET: usize = 1752;
pub const CUnit__5__m_lookAtOffset__OFFSET: usize = 1760;
pub const CUnit__5__m_overheadOffset__OFFSET: usize = 1772;
pub const CUnit__5__m_occupUI__OFFSET: usize = 1776;
pub const CUnit__5__m_unitStatusIconUI__OFFSET: usize = 1784;
pub const CUnit__5__m_unitStatusTextUI__OFFSET: usize = 1792;
pub const CUnit__5__m_uberSplatId__OFFSET: usize = 1800;
pub const CUnit__5__m_vertexColor__OFFSET: usize = 1804;
pub const CUnit__5__m_anim_size__OFFSET: usize = 1808;
pub const CUnit__5__m_anims__OFFSET: usize = 1812;
pub const CUnit__5__m_forcedAnimScale__OFFSET: usize = 1840;
pub const CUnit__5__m_walk_anim_speed__OFFSET: usize = 1844;
pub const CUnit__5__m_run_anim_speed__OFFSET: usize = 1848;
pub const CUnit__5__m_unit_ai__OFFSET: usize = 1852;

#[repr(C)]
pub struct CGameWar3__8 {
    pub _opaque: [u8; 12872],
}
pub const CGameWar3__8__SIZE: usize = 12872;
pub const CGameWar3__8__NAME: &str = "CGameWar3";
pub const CGameWar3__8__s_campaignArchiveFile__OFFSET: usize = 0;
pub const CGameWar3__8__anticheat_dummy_array_20__OFFSET: usize = 9640;
pub const CGameWar3__8__s_pGameWar3__OFFSET: usize = 0;
pub const CGameWar3__8__anticheat_dummy_array_21__OFFSET: usize = 9695;
pub const CGameWar3__8__m_mapScriptInstance__OFFSET: usize = 9784;
pub const CGameWar3__8__m_mapScriptInstanceId__OFFSET: usize = 9792;
pub const CGameWar3__8__m_mapScript__OFFSET: usize = 9800;
pub const CGameWar3__8__m_mapScriptId__OFFSET: usize = 9808;
pub const CGameWar3__8__m_executions__OFFSET: usize = 9816;
pub const CGameWar3__8__anticheat_dummy_array_22__OFFSET: usize = 9840;
pub const CGameWar3__8__m_gameIdMap__OFFSET: usize = 9864;
pub const CGameWar3__8__m_gameState__OFFSET: usize = 9872;
pub const CGameWar3__8__m_buildId__OFFSET: usize = 9880;
pub const CGameWar3__8__m_localPlayerId__OFFSET: usize = 9884;
pub const CGameWar3__8__m_replayLocalPlayerId__OFFSET: usize = 9886;
pub const CGameWar3__8__m_playerIdBitsToGhost__OFFSET: usize = 9888;
pub const CGameWar3__8__m_playerIdBitsToGhostSet__OFFSET: usize = 9892;
pub const CGameWar3__8__m_pMapSetup__OFFSET: usize = 9896;
pub const CGameWar3__8__m_pFog__OFFSET: usize = 9904;
pub const CGameWar3__8__m_gameName__OFFSET: usize = 9912;
pub const CGameWar3__8__anticheat_dummy_array_23__OFFSET: usize = 9936;
pub const CGameWar3__8__m_maxPlayerCount__OFFSET: usize = 9952;
pub const CGameWar3__8__m_playerCount__OFFSET: usize = 9956;
pub const CGameWar3__8__m_gameMaxPlayerCount__OFFSET: usize = 9960;
pub const CGameWar3__8__m_maxTeamCount__OFFSET: usize = 9964;
pub const CGameWar3__8__m_teams__OFFSET: usize = 9968;
pub const CGameWar3__8__m_players__OFFSET: usize = 9992;
pub const CGameWar3__8__m_startLocations__OFFSET: usize = 10224;
pub const CGameWar3__8__m_slotToPlayerId__OFFSET: usize = 11960;
pub const CGameWar3__8__m_levelToTransitionTo__OFFSET: usize = 12064;
pub const CGameWar3__8__m_modelCine__OFFSET: usize = 12088;
pub const CGameWar3__8__m_cine__OFFSET: usize = 12112;
pub const CGameWar3__8__m_gameOverMessage__OFFSET: usize = 12376;
pub const CGameWar3__8__m_switchingLevels__OFFSET: usize = 12400;
pub const CGameWar3__8__m_doScoreScreen__OFFSET: usize = 12401;
pub const CGameWar3__8__m_isReload__OFFSET: usize = 12402;
pub const CGameWar3__8__anticheat_dummy_array_24__OFFSET: usize = 12403;
pub const CGameWar3__8__m_wasScriptLoad__OFFSET: usize = 12410;
pub const CGameWar3__8__m_wasLoadGame__OFFSET: usize = 12411;
pub const CGameWar3__8__m_isCampaignMission__OFFSET: usize = 12412;
pub const CGameWar3__8__m_campaignIndex__OFFSET: usize = 12416;
pub const CGameWar3__8__m_gameSetup__OFFSET: usize = 12420;
pub const CGameWar3__8__m_usingDarkMask__OFFSET: usize = 12678;
pub const CGameWar3__8__m_unseenQuests__OFFSET: usize = 12679;
pub const CGameWar3__8__m_showAllUnitsCmdBar__OFFSET: usize = 12680;
pub const CGameWar3__8__m_isReplay__OFFSET: usize = 12681;
pub const CGameWar3__8__m_replayFile__OFFSET: usize = 12688;
pub const CGameWar3__8__m_isTournament__OFFSET: usize = 12712;
pub const CGameWar3__8__m_copySaveCount__OFFSET: usize = 12716;
pub const CGameWar3__8__m_finishedInit__OFFSET: usize = 12720;
pub const CGameWar3__8__m_didSaveGameFromScript__OFFSET: usize = 12721;
pub const CGameWar3__8__m_playingGame__OFFSET: usize = 12722;
pub const CGameWar3__8__m_FacialAnimationSetScopeID__OFFSET: usize = 12724;
pub const CGameWar3__8__m_pTriggerStrings__OFFSET: usize = 12728;
pub const CGameWar3__8__anticheat_dummy_array_25__OFFSET: usize = 12736;
pub const CGameWar3__8__m_messageLogContents__OFFSET: usize = 12776;
pub const CGameWar3__8__m_pGameHashTableManager__OFFSET: usize = 12800;
pub const CGameWar3__8__m_gameCacheManager__OFFSET: usize = 12808;
pub const CGameWar3__8__m_gameOverCallback__OFFSET: usize = 12816;
pub const CGameWar3__8__m_maxCheckpointSaves__OFFSET: usize = 12824;
pub const CGameWar3__8__m_pSaveThread__OFFSET: usize = 12832;
pub const CGameWar3__8__m_pCheckpointPopup__OFFSET: usize = 12840;
pub const CGameWar3__8__m_gamePlayStartTime__OFFSET: usize = 12848;
pub const CGameWar3__8__m_latency__OFFSET: usize = 12856;

#[repr(C)]
pub struct CPlayerWar3__8 {
    pub _opaque: [u8; 1960],
}
pub const CPlayerWar3__8__SIZE: usize = 1960;
pub const CPlayerWar3__8__NAME: &str = "CPlayerWar3";
pub const CPlayerWar3__8__s_CPlayerWar3_pool__OFFSET: usize = 0;
pub const CPlayerWar3__8__anticheat_dummy_array_16__OFFSET: usize = 88;
pub const CPlayerWar3__8__m_playerId__OFFSET: usize = 103;
pub const CPlayerWar3__8__m_playerName__OFFSET: usize = 104;
pub const CPlayerWar3__8__m_toonName__OFFSET: usize = 128;
pub const CPlayerWar3__8__m_protobufPlayerData__OFFSET: usize = 152;
pub const CPlayerWar3__8__m_slotNetPlayerId__OFFSET: usize = 336;
pub const CPlayerWar3__8__m_playerNamePrefix__OFFSET: usize = 344;
pub const CPlayerWar3__8__m_pAlliances__OFFSET: usize = 368;
pub const CPlayerWar3__8__m_pSelection__OFFSET: usize = 376;
pub const CPlayerWar3__8__m_playerStates__OFFSET: usize = 384;
pub const CPlayerWar3__8__m_lumberDivertTax__OFFSET: usize = 1016;
pub const CPlayerWar3__8__m_goldDivertTax__OFFSET: usize = 1116;
pub const CPlayerWar3__8__m_highestFood__OFFSET: usize = 1216;
pub const CPlayerWar3__8__m_highestFoodUsed__OFFSET: usize = 1220;
pub const CPlayerWar3__8__anticheat_dummy_array_17__OFFSET: usize = 1224;
pub const CPlayerWar3__8__m_heroKills__OFFSET: usize = 1240;
pub const CPlayerWar3__8__m_itemsObtained__OFFSET: usize = 1244;
pub const CPlayerWar3__8__m_mercsHired__OFFSET: usize = 1248;
pub const CPlayerWar3__8__m_teamColorIndex__OFFSET: usize = 1252;
pub const CPlayerWar3__8__m_controller__OFFSET: usize = 1256;
pub const CPlayerWar3__8__m_startLocIndex__OFFSET: usize = 1260;
pub const CPlayerWar3__8__m_difficulty__OFFSET: usize = 1264;
pub const CPlayerWar3__8__m_slotState__OFFSET: usize = 1268;
pub const CPlayerWar3__8__m_teamIndex__OFFSET: usize = 1272;
pub const CPlayerWar3__8__m_numActions__OFFSET: usize = 1276;
pub const CPlayerWar3__8__m_racePref__OFFSET: usize = 1280;
pub const CPlayerWar3__8__m_race__OFFSET: usize = 1284;
pub const CPlayerWar3__8__m_realTimeAPM__OFFSET: usize = 1288;
pub const CPlayerWar3__8__m_capChangedListener__OFFSET: usize = 1384;
pub const CPlayerWar3__8__m_usedChangedListener__OFFSET: usize = 1392;
pub const CPlayerWar3__8__m_foodUsedUpkeepListener__OFFSET: usize = 1400;
pub const CPlayerWar3__8__m_lumberChangedListener__OFFSET: usize = 1408;
pub const CPlayerWar3__8__m_goldChangedListener__OFFSET: usize = 1416;
pub const CPlayerWar3__8__m_goldUpkeepChanged__OFFSET: usize = 1424;
pub const CPlayerWar3__8__m_lumberUpkeepChanged__OFFSET: usize = 1432;
pub const CPlayerWar3__8__m_handicap__OFFSET: usize = 1440;
pub const CPlayerWar3__8__m_referee__OFFSET: usize = 1512;
pub const CPlayerWar3__8__m_deadHeroAge__OFFSET: usize = 1520;
pub const CPlayerWar3__8__m_deadHeroes__OFFSET: usize = 1544;
pub const CPlayerWar3__8__m_revivableFlags__OFFSET: usize = 1568;
pub const CPlayerWar3__8__anticheat_dummy_array_18__OFFSET: usize = 1572;
pub const CPlayerWar3__8__m_pTechTree__OFFSET: usize = 1584;
pub const CPlayerWar3__8__m_fogMask__OFFSET: usize = 1592;
pub const CPlayerWar3__8__m_ai__OFFSET: usize = 1596;
pub const CPlayerWar3__8__m_bestHeroes__OFFSET: usize = 1608;
pub const CPlayerWar3__8__m_leaderboard__OFFSET: usize = 1632;
pub const CPlayerWar3__8__m_leaveGameTimer__OFFSET: usize = 1648;
pub const CPlayerWar3__8__anticheat_dummy_array_19__OFFSET: usize = 1680;
pub const CPlayerWar3__8__m_doScoreScreen__OFFSET: usize = 1708;
pub const CPlayerWar3__8__m_allowContinue__OFFSET: usize = 1709;
pub const CPlayerWar3__8__m_crippled__OFFSET: usize = 1712;
pub const CPlayerWar3__8__m_gameOverMessage__OFFSET: usize = 1720;
pub const CPlayerWar3__8__m_showInScoreScreen__OFFSET: usize = 1744;
pub const CPlayerWar3__8__m_cachedScore__OFFSET: usize = 1748;
pub const CPlayerWar3__8__m_cachedScoreIsCached__OFFSET: usize = 1752;
pub const CPlayerWar3__8__m_heroCount__OFFSET: usize = 1756;
pub const CPlayerWar3__8__m_bnetPlayerId__OFFSET: usize = 1760;
pub const CPlayerWar3__8__m_heroPickOrder__OFFSET: usize = 1768;
pub const CPlayerWar3__8__m_unitKills__OFFSET: usize = 1792;
pub const CPlayerWar3__8__m_unitsCreated__OFFSET: usize = 1824;
pub const CPlayerWar3__8__m_abilityInfo__OFFSET: usize = 1856;
pub const CPlayerWar3__8__m_upkeepTime__OFFSET: usize = 1888;
pub const CPlayerWar3__8__m_lastUpkeepTime__OFFSET: usize = 1920;
pub const CPlayerWar3__8__m_playerItemUseMap__OFFSET: usize = 1928;

#[repr(C)]
pub struct CGameUI__12 {
    pub _opaque: [u8; 2384],
}
pub const CGameUI__12__SIZE: usize = 2384;
pub const CGameUI__12__NAME: &str = "CGameUI";
pub const CGameUI__12__m_transitionState__OFFSET: usize = 704;
pub const CGameUI__12__m_cachedFonts__OFFSET: usize = 712;
pub const CGameUI__12__NUMBER_OF_PLAYER_ROWS__OFFSET: usize = 0;
pub const CGameUI__12__FIRST_ROW_Y_OFFSET__OFFSET: usize = 0;
pub const CGameUI__12__ROW_SPACING__OFFSET: usize = 0;
pub const CGameUI__12__m_userControl__OFFSET: usize = 784;
pub const CGameUI__12__m_userUI__OFFSET: usize = 785;
pub const CGameUI__12__m_currentInputMode__OFFSET: usize = 792;
pub const CGameUI__12__m_inputModeStack__OFFSET: usize = 800;
pub const CGameUI__12__m_sharedTooltip__OFFSET: usize = 824;
pub const CGameUI__12__m_uberTooltip__OFFSET: usize = 832;
pub const CGameUI__12__m_showUberTips__OFFSET: usize = 840;
pub const CGameUI__12__m_autosaveReplay__OFFSET: usize = 841;
pub const CGameUI__12__m_healthBarsMode__OFFSET: usize = 844;
pub const CGameUI__12__m_forceShowHealthBars__OFFSET: usize = 848;
pub const CGameUI__12__m_minimapSignals__OFFSET: usize = 849;
pub const CGameUI__12__m_useColoredHealthBars__OFFSET: usize = 850;
pub const CGameUI__12__m_useHeroFrames__OFFSET: usize = 851;
pub const CGameUI__12__m_showHeroLevelFrame__OFFSET: usize = 852;
pub const CGameUI__12__m_showNumericCooldown__OFFSET: usize = 853;
pub const CGameUI__12__m_showDenyIcon__OFFSET: usize = 854;
pub const CGameUI__12__m_showGoldmineUnitCounter__OFFSET: usize = 855;
pub const CGameUI__12__m_enableObserverUiV2__OFFSET: usize = 856;
pub const CGameUI__12__m_asyncPlayerData__OFFSET: usize = 860;
pub const CGameUI__12__m_targetMode__OFFSET: usize = 960;
pub const CGameUI__12__m_selectMode__OFFSET: usize = 968;
pub const CGameUI__12__m_dragSelectMode__OFFSET: usize = 976;
pub const CGameUI__12__m_dragScrollMode__OFFSET: usize = 984;
pub const CGameUI__12__m_buildMode__OFFSET: usize = 992;
pub const CGameUI__12__m_signalMode__OFFSET: usize = 1000;
pub const CGameUI__12__m_escapeMenu__OFFSET: usize = 1008;
pub const CGameUI__12__m_gameOverMenu__OFFSET: usize = 1016;
pub const CGameUI__12__m_suspendIndicator__OFFSET: usize = 1024;
pub const CGameUI__12__m_unresponsiveIndicator__OFFSET: usize = 1032;
pub const CGameUI__12__m_allianceMode__OFFSET: usize = 1040;
pub const CGameUI__12__m_chatMode__OFFSET: usize = 1048;
pub const CGameUI__12__m_questMode__OFFSET: usize = 1056;
pub const CGameUI__12__anticheat_dummy_array_4__OFFSET: usize = 1064;
pub const CGameUI__12__m_scriptDialogMode__OFFSET: usize = 1152;
pub const CGameUI__12__m_unresponsiveDelayTimerId__OFFSET: usize = 1160;
pub const CGameUI__12__m_selectionManager__OFFSET: usize = 1168;
pub const CGameUI__12__m_dragScrollManager__OFFSET: usize = 1176;
pub const CGameUI__12__m_camera__OFFSET: usize = 1184;
pub const CGameUI__12__m_paused__OFFSET: usize = 1192;
pub const CGameUI__12__m_uiPaused__OFFSET: usize = 1193;
pub const CGameUI__12__m_scriptPaused__OFFSET: usize = 1194;
pub const CGameUI__12__m_disableUpkeepMsg__OFFSET: usize = 1195;
pub const CGameUI__12__m_upkeepLvlSave__OFFSET: usize = 1196;
pub const CGameUI__12__m_refresh__OFFSET: usize = 1200;
pub const CGameUI__12__m_refreshWaypoints__OFFSET: usize = 1201;
pub const CGameUI__12__m_deferCleanSelUI__OFFSET: usize = 1202;
pub const CGameUI__12__m_refreshInfoBar__OFFSET: usize = 1203;
pub const CGameUI__12__m_refreshCommandBar__OFFSET: usize = 1204;
pub const CGameUI__12__m_refreshHeroBar__OFFSET: usize = 1205;
pub const CGameUI__12__m_popSubMenu__OFFSET: usize = 1206;
pub const CGameUI__12__m_activeValidOrder__OFFSET: usize = 1208;
pub const CGameUI__12__m_cineForceSubtitles__OFFSET: usize = 1212;
pub const CGameUI__12__m_cinematicPanelsHidden__OFFSET: usize = 1213;
pub const CGameUI__12__m_dragSelectEnabled__OFFSET: usize = 1214;
pub const CGameUI__12__m_dragSelectUIEnabled__OFFSET: usize = 1215;
pub const CGameUI__12__m_preSelectEnabled__OFFSET: usize = 1216;
pub const CGameUI__12__m_preSelectUIEnabled__OFFSET: usize = 1217;
pub const CGameUI__12__m_selectEnabled__OFFSET: usize = 1218;
pub const CGameUI__12__m_selectUIEnabled__OFFSET: usize = 1219;
pub const CGameUI__12__m_observerModeEnabled__OFFSET: usize = 1220;
pub const CGameUI__12__m_observerModeV2Active__OFFSET: usize = 1221;
pub const CGameUI__12__m_wideScreenAutoPosition__OFFSET: usize = 1222;
pub const CGameUI__12__m_fadeTimer__OFFSET: usize = 1224;
pub const CGameUI__12__m_cinematicFadeTimer__OFFSET: usize = 1328;
pub const CGameUI__12__anticheat_dummy_array_5__OFFSET: usize = 1432;
pub const CGameUI__12__m_frameParentTimer__OFFSET: usize = 1440;
pub const CGameUI__12__m_simpleFadeTimer__OFFSET: usize = 1544;
pub const CGameUI__12__m_attackNotify__OFFSET: usize = 1552;
pub const CGameUI__12__m_warpNotify__OFFSET: usize = 1576;
pub const CGameUI__12__m_timerDialogs__OFFSET: usize = 1608;
pub const CGameUI__12__m_leaderboard__OFFSET: usize = 1632;
pub const CGameUI__12__anticheat_dummy_array_6__OFFSET: usize = 1640;
pub const CGameUI__12__m_multiboard__OFFSET: usize = 1648;
pub const CGameUI__12__m_suspendMultiboardDisplay__OFFSET: usize = 1656;
pub const CGameUI__12__anticheat_dummy_array_7__OFFSET: usize = 1657;
pub const CGameUI__12__m_scriptDialog__OFFSET: usize = 1728;
pub const CGameUI__12__m_replaySelection__OFFSET: usize = 1736;
pub const CGameUI__12__m_unitStatusIconUIFactory__OFFSET: usize = 1744;
pub const CGameUI__12__m_unitStatusTexts__OFFSET: usize = 1752;
pub const CGameUI__12__m_worldFrame__OFFSET: usize = 1776;
pub const CGameUI__12__m_infoBar__OFFSET: usize = 1784;
pub const CGameUI__12__m_commandBar__OFFSET: usize = 1792;
pub const CGameUI__12__m_miniMap__OFFSET: usize = 1800;
pub const CGameUI__12__m_resourceBar__OFFSET: usize = 1808;
pub const CGameUI__12__m_upperButtonBar__OFFSET: usize = 1816;
pub const CGameUI__12__m_observerButtonBar__OFFSET: usize = 1824;
pub const CGameUI__12__m_playerStatisticsPanel__OFFSET: usize = 1832;
pub const CGameUI__12__m_heroPanel__OFFSET: usize = 1840;
pub const CGameUI__12__m_productionPanel__OFFSET: usize = 1848;
pub const CGameUI__12__m_minimapButtonBar__OFFSET: usize = 1856;
pub const CGameUI__12__m_trackingStub__OFFSET: usize = 1864;
pub const CGameUI__12__m_heroBar__OFFSET: usize = 1872;
pub const CGameUI__12__m_peonBar__OFFSET: usize = 1880;
pub const CGameUI__12__m_messageBox__OFFSET: usize = 1888;
pub const CGameUI__12__m_unitMessageBox__OFFSET: usize = 1896;
pub const CGameUI__12__m_chatMessageBox__OFFSET: usize = 1904;
pub const CGameUI__12__m_topMessageBox__OFFSET: usize = 1912;
pub const CGameUI__12__m_currentPortraitButton__OFFSET: usize = 1920;
pub const CGameUI__12__m_timeOfDayIndicator__OFFSET: usize = 1928;
pub const CGameUI__12__m_chatEditBar__OFFSET: usize = 1936;
pub const CGameUI__12__m_cinematicPanel__OFFSET: usize = 1944;
pub const CGameUI__12__m_lastBuffBar__OFFSET: usize = 1952;
pub const CGameUI__12__m_observerBar__OFFSET: usize = 1960;
pub const CGameUI__12__m_miniMapSignalButton__OFFSET: usize = 1968;
pub const CGameUI__12__m_miniMapTerrainButton__OFFSET: usize = 1976;
pub const CGameUI__12__m_miniMapAllyButton__OFFSET: usize = 1984;
pub const CGameUI__12__m_miniMapCreepButton__OFFSET: usize = 1992;
pub const CGameUI__12__m_formationButton__OFFSET: usize = 2000;
pub const CGameUI__12__m_consoleFrame__OFFSET: usize = 2008;
pub const CGameUI__12__m_simpleConsoleFrame__OFFSET: usize = 2016;
pub const CGameUI__12__m_simpleConsoleBottomBarFrame__OFFSET: usize = 2024;
pub const CGameUI__12__m_simpleConsoleBottomBarFrameOverlay__OFFSET: usize = 2032;
pub const CGameUI__12__m_frameUIParent__OFFSET: usize = 2040;
pub const CGameUI__12__top__OFFSET: usize = 2048;
pub const CGameUI__12__topInGame__OFFSET: usize = 2056;
pub const CGameUI__12__m_simpleUIParent__OFFSET: usize = 2064;
pub const CGameUI__12__m_observerUIParent__OFFSET: usize = 2072;
pub const CGameUI__12__m_observerInfoPanelBackground__OFFSET: usize = 2080;
pub const CGameUI__12__m_observerInfoPanelBackpack__OFFSET: usize = 2088;
pub const CGameUI__12__m_consoleBackdropFrame__OFFSET: usize = 2096;
pub const CGameUI__12__m_minimapTextureBackground__OFFSET: usize = 2104;
pub const CGameUI__12__m_infoPanelTexture__OFFSET: usize = 2112;
pub const CGameUI__12__m_quickSaveKey__OFFSET: usize = 2120;
pub const CGameUI__12__m_quickLoadKey__OFFSET: usize = 2124;
pub const CGameUI__12__m_quickHelpKey__OFFSET: usize = 2128;
pub const CGameUI__12__m_quickOptionsKey__OFFSET: usize = 2132;
pub const CGameUI__12__m_quickQuitKey__OFFSET: usize = 2136;
pub const CGameUI__12__m_minimapSignalKey__OFFSET: usize = 2140;
pub const CGameUI__12__m_minimapTerrainKey__OFFSET: usize = 2144;
pub const CGameUI__12__m_minimapColorsKey__OFFSET: usize = 2148;
pub const CGameUI__12__m_minimapCreepKey__OFFSET: usize = 2152;
pub const CGameUI__12__m_formationToggleKey__OFFSET: usize = 2156;
pub const CGameUI__12__m_observerToggleAllPanelsKey__OFFSET: usize = 2160;
pub const CGameUI__12__m_observerProductionModeKey__OFFSET: usize = 2164;
pub const CGameUI__12__m_observerUnitsModeKey__OFFSET: usize = 2168;
pub const CGameUI__12__m_mouseDownHandler__OFFSET: usize = 2176;
pub const CGameUI__12__m_mouseWheelHandler__OFFSET: usize = 2184;
pub const CGameUI__12__m_mouseUpHandler__OFFSET: usize = 2192;
pub const CGameUI__12__m_mouseMoveHandler__OFFSET: usize = 2200;
pub const CGameUI__12__m_observerToggleablePanels__OFFSET: usize = 2208;
pub const CGameUI__12__m_consoleBackdropHeight__OFFSET: usize = 2240;
pub const CGameUI__12__m_auroraChatHandler__OFFSET: usize = 2248;
pub const CGameUI__12__m_lobbyEventsHandler__OFFSET: usize = 2328;
pub const CGameUI__12__s_indexedCustomHotkeyMappingMaxCount__OFFSET: usize = 0;
pub const CGameUI__12__s_customHotkeyEnabled__OFFSET: usize = 0;
pub const CGameUI__12__s_customHotkeyMappingTable__OFFSET: usize = 0;
pub const CGameUI__12__s_customHotkeyMappingTableNonHero__OFFSET: usize = 0;
pub const CGameUI__12__m_observerUnitToFollow__OFFSET: usize = 2376;

#[repr(C)]
pub struct CCameraWar3__1 {
    pub _opaque: [u8; 2216],
}
pub const CCameraWar3__1__SIZE: usize = 2216;
pub const CCameraWar3__1__NAME: &str = "CCameraWar3";
pub const CCameraWar3__1__s_CCameraWar3_pool__OFFSET: usize = 0;
pub const CCameraWar3__1__m_cameraNdx__OFFSET: usize = 56;
pub const CCameraWar3__1__m_defaultCameraDifferencePercentage__OFFSET: usize = 60;
pub const CCameraWar3__1__m_defaultCameraDistanceDifference__OFFSET: usize = 64;
pub const CCameraWar3__1__m_defaultCameraNdx__OFFSET: usize = 68;
pub const CCameraWar3__1__m_defaultPrefCameraDist__OFFSET: usize = 72;
pub const CCameraWar3__1__m_maxCameraDist__OFFSET: usize = 76;
pub const CCameraWar3__1__m_rotationNdx__OFFSET: usize = 80;
pub const CCameraWar3__1__m_rollNdx__OFFSET: usize = 84;
pub const CCameraWar3__1__m_localPitchNdx__OFFSET: usize = 88;
pub const CCameraWar3__1__m_localYawNdx__OFFSET: usize = 92;
pub const CCameraWar3__1__m_localRollNdx__OFFSET: usize = 96;
pub const CCameraWar3__1__m_wheelPos__OFFSET: usize = 100;
pub const CCameraWar3__1__m_lock__OFFSET: usize = 104;
pub const CCameraWar3__1__m_disableMouseScroll__OFFSET: usize = 105;
pub const CCameraWar3__1__m_enableMouseWheel__OFFSET: usize = 106;
pub const CCameraWar3__1__m_currentCamera__OFFSET: usize = 112;
pub const CCameraWar3__1__m_currentMode__OFFSET: usize = 120;
pub const CCameraWar3__1__m_isCinematicAudio__OFFSET: usize = 124;
pub const CCameraWar3__1__m_listener__OFFSET: usize = 128;
pub const CCameraWar3__1__m_camera__OFFSET: usize = 136;
pub const CCameraWar3__1__m_targetController__OFFSET: usize = 144;
pub const CCameraWar3__1__m_targetControllerInheritOrientation__OFFSET: usize = 152;
pub const CCameraWar3__1__m_targetControllerOrientationOnly__OFFSET: usize = 153;
pub const CCameraWar3__1__m_targetControllerOffset__OFFSET: usize = 156;
pub const CCameraWar3__1__m_cinematicCameraSprite__OFFSET: usize = 168;
pub const CCameraWar3__1__m_cinematicOrientation__OFFSET: usize = 176;
pub const CCameraWar3__1__m_timer__OFFSET: usize = 216;
pub const CCameraWar3__1__m_lastQT__OFFSET: usize = 248;
pub const CCameraWar3__1__m_lastQTTime__OFFSET: usize = 252;
pub const CCameraWar3__1__m_QTHead__OFFSET: usize = 256;
pub const CCameraWar3__1__m_activeQTCount__OFFSET: usize = 260;
pub const CCameraWar3__1__m_quickTargets__OFFSET: usize = 264;
pub const CCameraWar3__1__m_lastTH__OFFSET: usize = 288;
pub const CCameraWar3__1__m_lastTHTime__OFFSET: usize = 292;
pub const CCameraWar3__1__m_townhalls__OFFSET: usize = 296;
pub const CCameraWar3__1__m_listenerDistance__OFFSET: usize = 320;
pub const CCameraWar3__1__m_listenerDistanceMod__OFFSET: usize = 344;
pub const CCameraWar3__1__m_listenerDistanceSmoothMod__OFFSET: usize = 360;
pub const CCameraWar3__1__m_listenerAngleCorrection__OFFSET: usize = 376;
pub const CCameraWar3__1__m_listenerAngleCorrectionMod__OFFSET: usize = 400;
pub const CCameraWar3__1__m_listenerAngleCorrectionSmoothMod__OFFSET: usize = 416;
pub const CCameraWar3__1__m_listenerAOA__OFFSET: usize = 432;
pub const CCameraWar3__1__m_listenerAOAMod__OFFSET: usize = 456;
pub const CCameraWar3__1__m_listenerAOASmoothMod__OFFSET: usize = 472;
pub const CCameraWar3__1__m_distance__OFFSET: usize = 488;
pub const CCameraWar3__1__m_distanceMod__OFFSET: usize = 512;
pub const CCameraWar3__1__m_distanceFactor__OFFSET: usize = 528;
pub const CCameraWar3__1__m_distanceSmoothMod__OFFSET: usize = 536;
pub const CCameraWar3__1__m_farZ__OFFSET: usize = 552;
pub const CCameraWar3__1__m_farZMod__OFFSET: usize = 576;
pub const CCameraWar3__1__m_farZSmoothMod__OFFSET: usize = 592;
pub const CCameraWar3__1__m_nearZ__OFFSET: usize = 608;
pub const CCameraWar3__1__m_nearZMod__OFFSET: usize = 632;
pub const CCameraWar3__1__m_nearZSmoothMod__OFFSET: usize = 648;
pub const CCameraWar3__1__m_fov__OFFSET: usize = 664;
pub const CCameraWar3__1__m_fovMod__OFFSET: usize = 688;
pub const CCameraWar3__1__m_fovSmoothMod__OFFSET: usize = 704;
pub const CCameraWar3__1__m_rotation__OFFSET: usize = 720;
pub const CCameraWar3__1__m_rotationMod__OFFSET: usize = 744;
pub const CCameraWar3__1__m_rotationSmoothMod__OFFSET: usize = 760;
pub const CCameraWar3__1__m_aoa__OFFSET: usize = 776;
pub const CCameraWar3__1__m_aoaMod__OFFSET: usize = 800;
pub const CCameraWar3__1__m_aoaSmoothMod__OFFSET: usize = 816;
pub const CCameraWar3__1__m_roll__OFFSET: usize = 832;
pub const CCameraWar3__1__m_rollMod__OFFSET: usize = 856;
pub const CCameraWar3__1__m_rollSmoothMod__OFFSET: usize = 872;
pub const CCameraWar3__1__m_localPitch__OFFSET: usize = 888;
pub const CCameraWar3__1__m_localPitchMod__OFFSET: usize = 912;
pub const CCameraWar3__1__m_localPitchSmoothMod__OFFSET: usize = 928;
pub const CCameraWar3__1__m_localYaw__OFFSET: usize = 944;
pub const CCameraWar3__1__m_localYawMod__OFFSET: usize = 968;
pub const CCameraWar3__1__m_localYawSmoothMod__OFFSET: usize = 984;
pub const CCameraWar3__1__m_localRoll__OFFSET: usize = 1000;
pub const CCameraWar3__1__m_localRollMod__OFFSET: usize = 1024;
pub const CCameraWar3__1__m_localRollSmoothMod__OFFSET: usize = 1040;
pub const CCameraWar3__1__m_target__OFFSET: usize = 1056;
pub const CCameraWar3__1__m_targetZOffset__OFFSET: usize = 1080;
pub const CCameraWar3__1__m_targetZOffsetMod__OFFSET: usize = 1104;
pub const CCameraWar3__1__m_targetZOffsetSmoothMod__OFFSET: usize = 1112;
pub const CCameraWar3__1__m_panning__OFFSET: usize = 1120;
pub const CCameraWar3__1__m_panInterpZ__OFFSET: usize = 1121;
pub const CCameraWar3__1__m_panZ__OFFSET: usize = 1128;
pub const CCameraWar3__1__m_panZMod__OFFSET: usize = 1152;
pub const CCameraWar3__1__m_panZSmoothMod__OFFSET: usize = 1160;
pub const CCameraWar3__1__m_doTargetNoise__OFFSET: usize = 1168;
pub const CCameraWar3__1__m_doTargetNoiseVertOnly__OFFSET: usize = 1169;
pub const CCameraWar3__1__m_targetNoiseDir__OFFSET: usize = 1172;
pub const CCameraWar3__1__m_targetNoiseMag__OFFSET: usize = 1320;
pub const CCameraWar3__1__m_doSourceNoise__OFFSET: usize = 1468;
pub const CCameraWar3__1__m_doSourceNoiseVertOnly__OFFSET: usize = 1469;
pub const CCameraWar3__1__m_sourceNoiseDir__OFFSET: usize = 1472;
pub const CCameraWar3__1__m_sourceNoiseMag__OFFSET: usize = 1620;
pub const CCameraWar3__1__m_horMove__OFFSET: usize = 1768;
pub const CCameraWar3__1__m_verMove__OFFSET: usize = 1784;
pub const CCameraWar3__1__m_panMove__OFFSET: usize = 1800;
pub const CCameraWar3__1__m_panSmoothMove__OFFSET: usize = 1808;
pub const CCameraWar3__1__m_position__OFFSET: usize = 1816;
pub const CCameraWar3__1__m_lastListenerPos__OFFSET: usize = 1840;
pub const CCameraWar3__1__m_centerOnUnitKey__OFFSET: usize = 1852;
pub const CCameraWar3__1__m_resetZoomKey__OFFSET: usize = 1856;
pub const CCameraWar3__1__m_rotModeCenter__OFFSET: usize = 1860;
pub const CCameraWar3__1__m_rotModeTargOffset__OFFSET: usize = 1872;
pub const CCameraWar3__1__m_rotModeEyeOffset__OFFSET: usize = 1884;
pub const CCameraWar3__1__m_rotModeAngle__OFFSET: usize = 1896;
pub const CCameraWar3__1__m_rotModeInitialRot__OFFSET: usize = 1920;
pub const CCameraWar3__1__m_rotModeRate__OFFSET: usize = 1928;
pub const CCameraWar3__1__m_startPos__OFFSET: usize = 1936;
pub const CCameraWar3__1__m_boundRect__OFFSET: usize = 1944;
pub const CCameraWar3__1__m_boundPoints__OFFSET: usize = 1960;
pub const CCameraWar3__1__m_cameraGuardBand__OFFSET: usize = 1992;
pub const CCameraWar3__1__m_mouseSpeedFactor__OFFSET: usize = 2008;
pub const CCameraWar3__1__m_keyboardSpeedFactor__OFFSET: usize = 2012;
pub const CCameraWar3__1__m_worldToCamBasis__OFFSET: usize = 2016;
pub const CCameraWar3__1__m_worldToCam__OFFSET: usize = 2052;
pub const CCameraWar3__1__m_camToWorld__OFFSET: usize = 2088;
pub const CCameraWar3__1__m_filter__OFFSET: usize = 2124;
pub const CCameraWar3__1__m_heights__OFFSET: usize = 2160;
pub const CCameraWar3__1__m_elapsed__OFFSET: usize = 2172;
pub const CCameraWar3__1__m_focalDistance__OFFSET: usize = 2176;
pub const CCameraWar3__1__m_dofScale__OFFSET: usize = 2180;
pub const CCameraWar3__1__m_dofEnabled__OFFSET: usize = 2184;
pub const CCameraWar3__1__s_listenerNeedsUpdate__OFFSET: usize = 0;
pub const CCameraWar3__1__savedTarget__OFFSET: usize = 2188;
pub const CCameraWar3__1__savedDistance__OFFSET: usize = 2200;
pub const CCameraWar3__1__savedAOA__OFFSET: usize = 2204;
pub const CCameraWar3__1__savedRotation__OFFSET: usize = 2208;
pub const CCameraWar3__1__m_smoothingCoef__OFFSET: usize = 2212;

#[repr(C)]
pub struct CWidget__1 {
    pub _opaque: [u8; 408],
}
pub const CWidget__1__SIZE: usize = 408;
pub const CWidget__1__NAME: &str = "CWidget";
pub const CWidget__1__s_CWidget_pool__OFFSET: usize = 0;
pub const CWidget__1__s_updateHintMaps__OFFSET: usize = 0;
pub const CWidget__1__m_everSeen__OFFSET: usize = 104;
pub const CWidget__1__m_isFogged__OFFSET: usize = 108;
pub const CWidget__1__m_widget_id__OFFSET: usize = 112;
pub const CWidget__1__m_widget_name__OFFSET: usize = 116;
pub const CWidget__1__m_skin_id__OFFSET: usize = 376;
pub const CWidget__1__m_pPathingRegion__OFFSET: usize = 384;
pub const CWidget__1__m_occluder__OFFSET: usize = 392;
pub const CWidget__1__m_wasOccluding__OFFSET: usize = 396;
pub const CWidget__1__m_occluderHeight__OFFSET: usize = 400;

#[repr(C)]
pub struct CUnit__6 {
    pub _opaque: [u8; 1864],
}
pub const CUnit__6__SIZE: usize = 1864;
pub const CUnit__6__NAME: &str = "CUnit";
pub const CUnit__6__s_CUnit_pool__OFFSET: usize = 0;
pub const CUnit__6__m_unit_paused_count__OFFSET: usize = 440;
pub const CUnit__6__m_PlayingDialogSound__OFFSET: usize = 444;
pub const CUnit__6__m_owner__OFFSET: usize = 448;
pub const CUnit__6__m_flags0__OFFSET: usize = 452;
pub const CUnit__6__m_flags1__OFFSET: usize = 456;
pub const CUnit__6__m_foodAndTechTreeFlags__OFFSET: usize = 460;
pub const CUnit__6__m_minimapFlashTimer__OFFSET: usize = 464;
pub const CUnit__6__m_damage_dealt__OFFSET: usize = 496;
pub const CUnit__6__m_self_damage__OFFSET: usize = 500;
pub const CUnit__6__m_healing_done__OFFSET: usize = 504;
pub const CUnit__6__m_time_stopped__OFFSET: usize = 508;
pub const CUnit__6__m_total_kills__OFFSET: usize = 512;
pub const CUnit__6__m_self_kills__OFFSET: usize = 516;
pub const CUnit__6__m_hero_kills__OFFSET: usize = 520;
pub const CUnit__6__m_building_kills__OFFSET: usize = 524;
pub const CUnit__6__m_override_ability_id_stats_credit__OFFSET: usize = 528;
pub const CUnit__6__m_override_item_id_stats_credit__OFFSET: usize = 532;
pub const CUnit__6__m_unit_stats_override__OFFSET: usize = 536;
pub const CUnit__6__m_caller__OFFSET: usize = 544;
pub const CUnit__6__m_lastAttackNotificationTime__OFFSET: usize = 576;
pub const CUnit__6__m_lastAttackNotificationSoundTime__OFFSET: usize = 580;
pub const CUnit__6__m_life__OFFSET: usize = 584;
pub const CUnit__6__m_death__OFFSET: usize = 608;
pub const CUnit__6__m_total_life_lost__OFFSET: usize = 616;
pub const CUnit__6__m_stored_time_when_last_alive__OFFSET: usize = 620;
pub const CUnit__6__m_time_alive_ms__OFFSET: usize = 624;
pub const CUnit__6__m_number_of_deaths__OFFSET: usize = 628;
pub const CUnit__6__m_life_regen_rate__OFFSET: usize = 632;
pub const CUnit__6__m_life_regen__OFFSET: usize = 648;
pub const CUnit__6__m_life_regen_type__OFFSET: usize = 656;
pub const CUnit__6__m_mana__OFFSET: usize = 664;
pub const CUnit__6__m_ai_mana__OFFSET: usize = 688;
pub const CUnit__6__m_mana_regen_rate__OFFSET: usize = 704;
pub const CUnit__6__m_mana_regen__OFFSET: usize = 720;
pub const CUnit__6__m_total_mana_spent__OFFSET: usize = 728;
pub const CUnit__6__m_defense__OFFSET: usize = 736;
pub const CUnit__6__m_battle_type__OFFSET: usize = 752;
pub const CUnit__6__m_invulnerable_count__OFFSET: usize = 756;
pub const CUnit__6__m_auto_attack_ignore_count__OFFSET: usize = 760;
pub const CUnit__6__m_fogFlash__OFFSET: usize = 764;
pub const CUnit__6__m_sight__OFFSET: usize = 776;
pub const CUnit__6__m_sightRangeCached__OFFSET: usize = 800;
pub const CUnit__6__m_sightMod__OFFSET: usize = 808;
pub const CUnit__6__m_invisibility_count__OFFSET: usize = 816;
pub const CUnit__6__m_invisibility_percent__OFFSET: usize = 824;
pub const CUnit__6__m_invis_modify__OFFSET: usize = 848;
pub const CUnit__6__m_invis_listen__OFFSET: usize = 856;
pub const CUnit__6__m_detectedData__OFFSET: usize = 864;
pub const CUnit__6__m_sharedVisionData__OFFSET: usize = 876;
pub const CUnit__6__m_sharedVisionDataCached__OFFSET: usize = 888;
pub const CUnit__6__m_sharedVisionTransitiveDataCached__OFFSET: usize = 892;
pub const CUnit__6__m_frost_count__OFFSET: usize = 896;
pub const CUnit__6__m_stone_count__OFFSET: usize = 900;
pub const CUnit__6__m_ethereal_count__OFFSET: usize = 904;
pub const CUnit__6__m_magic_immunity_count__OFFSET: usize = 908;
pub const CUnit__6__m_dispel_immunity_count__OFFSET: usize = 912;
pub const CUnit__6__m_fog_radius__OFFSET: usize = 920;
pub const CUnit__6__m_pos__OFFSET: usize = 936;
pub const CUnit__6__m_exp_level__OFFSET: usize = 960;
pub const CUnit__6__MaxNameLength__OFFSET: usize = 0;
pub const CUnit__6__m_unit_proper_name__OFFSET: usize = 964;
pub const CUnit__6__m_pick_order__OFFSET: usize = 1224;
pub const CUnit__6__m_task_stack__OFFSET: usize = 1228;
pub const CUnit__6__m_task_timer__OFFSET: usize = 1240;
pub const CUnit__6__m_action__OFFSET: usize = 1272;
pub const CUnit__6__m_uninterruptable_count__OFFSET: usize = 1276;
pub const CUnit__6__m_order_head__OFFSET: usize = 1280;
pub const CUnit__6__m_order_tail__OFFSET: usize = 1292;
pub const CUnit__6__order_count__OFFSET: usize = 1304;
pub const CUnit__6__m_gold_bounty_dice__OFFSET: usize = 1308;
pub const CUnit__6__m_gold_bounty_sides__OFFSET: usize = 1312;
pub const CUnit__6__m_gold_bounty_plus__OFFSET: usize = 1316;
pub const CUnit__6__m_wood_bounty_dice__OFFSET: usize = 1320;
pub const CUnit__6__m_wood_bounty_sides__OFFSET: usize = 1324;
pub const CUnit__6__m_wood_bounty_plus__OFFSET: usize = 1328;
pub const CUnit__6__m_cast_point__OFFSET: usize = 1336;
pub const CUnit__6__m_cast_back_swing__OFFSET: usize = 1352;
pub const CUnit__6__m_abilities__OFFSET: usize = 1368;
pub const CUnit__6__m_disabled_count__OFFSET: usize = 1380;
pub const CUnit__6__m_disabled_hidden_magic_count__OFFSET: usize = 1384;
pub const CUnit__6__m_silence_count__OFFSET: usize = 1388;
pub const CUnit__6__anticheat_dummy_array_12__OFFSET: usize = 1392;
pub const CUnit__6__m_disabled_magic_count__OFFSET: usize = 1404;
pub const CUnit__6__m_disabled_hidden_count__OFFSET: usize = 1408;
pub const CUnit__6__m_abil_paused_count__OFFSET: usize = 1412;
pub const CUnit__6__m_disabled_hidden_physical_count__OFFSET: usize = 1416;
pub const CUnit__6__m_disabled_physical_count__OFFSET: usize = 1420;
pub const CUnit__6__anticheat_dummy_array_13__OFFSET: usize = 1424;
pub const CUnit__6__m_buff_paused_count__OFFSET: usize = 1432;
pub const CUnit__6__m_abil_inv__OFFSET: usize = 1440;
pub const CUnit__6__m_abil_hero__OFFSET: usize = 1448;
pub const CUnit__6__m_abil_move__OFFSET: usize = 1456;
pub const CUnit__6__anticheat_dummy_array_14__OFFSET: usize = 1464;
pub const CUnit__6__m_abil_attack__OFFSET: usize = 1472;
pub const CUnit__6__m_abil_build__OFFSET: usize = 1480;
pub const CUnit__6__m_base_priority__OFFSET: usize = 1488;
pub const CUnit__6__m_death_time__OFFSET: usize = 1504;
pub const CUnit__6__s_minUnitSpeed__OFFSET: usize = 0;
pub const CUnit__6__s_maxUnitSpeed__OFFSET: usize = 0;
pub const CUnit__6__s_minBldgSpeed__OFFSET: usize = 0;
pub const CUnit__6__s_maxBldgSpeed__OFFSET: usize = 0;
pub const CUnit__6__s_buildingKillsGiveExp__OFFSET: usize = 0;
pub const CUnit__6__s_globalExperience__OFFSET: usize = 0;
pub const CUnit__6__s_maxLevelHeroesDrainExp__OFFSET: usize = 0;
pub const CUnit__6__m_display_height__OFFSET: usize = 1520;
pub const CUnit__6__m_display_height_modifier__OFFSET: usize = 1544;
pub const CUnit__6__anticheat_dummy_array_15__OFFSET: usize = 1552;
pub const CUnit__6__m_last_cluster_birth_tag__OFFSET: usize = 1560;
pub const CUnit__6__m_last_cluster_presence_tag__OFFSET: usize = 1564;
pub const CUnit__6__m_move_type__OFFSET: usize = 1568;
pub const CUnit__6__m_ground_count__OFFSET: usize = 1572;
pub const CUnit__6__m_launch_offset__OFFSET: usize = 1576;
pub const CUnit__6__m_fly_height__OFFSET: usize = 1592;
pub const CUnit__6__m_impact_swim__OFFSET: usize = 1616;
pub const CUnit__6__anticheat_dummy_array_16__OFFSET: usize = 1620;
pub const CUnit__6__m_launch_swim_z__OFFSET: usize = 1624;
pub const CUnit__6__m_impact_offset__OFFSET: usize = 1628;
pub const CUnit__6__m_category__OFFSET: usize = 1632;
pub const CUnit__6__m_target_type_flags__OFFSET: usize = 1636;
pub const CUnit__6__m_cargo_size__OFFSET: usize = 1640;
pub const CUnit__6__m_killedBy__OFFSET: usize = 1644;
pub const CUnit__6__m_shadow__OFFSET: usize = 1656;
pub const CUnit__6__m_shadowWater__OFFSET: usize = 1660;
pub const CUnit__6__anticheat_dummy_array_17__OFFSET: usize = 1664;
pub const CUnit__6__m_shadowAboveWater__OFFSET: usize = 1671;
pub const CUnit__6__m_shadowShow__OFFSET: usize = 1672;
pub const CUnit__6__m_showOccMark__OFFSET: usize = 1673;
pub const CUnit__6__m_allySelectionImage__OFFSET: usize = 1676;
pub const CUnit__6__m_occluded__OFFSET: usize = 1680;
pub const CUnit__6__anticheat_dummy_array_18__OFFSET: usize = 1681;
pub const CUnit__6__m_occlusionMark__OFFSET: usize = 1684;
pub const CUnit__6__m_jassUserData__OFFSET: usize = 1688;
pub const CUnit__6__m_foliageExclusionId__OFFSET: usize = 1692;
pub const CUnit__6__m_uiFlags__OFFSET: usize = 1696;
pub const CUnit__6__m_cachedPos__OFFSET: usize = 1700;
pub const CUnit__6__m_cachedFacing__OFFSET: usize = 1712;
pub const CUnit__6__m_cachedOrientation__OFFSET: usize = 1716;
pub const CUnit__6__m_lookAtTarget__OFFSET: usize = 1752;
pub const CUnit__6__m_lookAtOffset__OFFSET: usize = 1760;
pub const CUnit__6__m_overheadOffset__OFFSET: usize = 1772;
pub const CUnit__6__m_occupUI__OFFSET: usize = 1776;
pub const CUnit__6__m_unitStatusIconUI__OFFSET: usize = 1784;
pub const CUnit__6__m_unitStatusTextUI__OFFSET: usize = 1792;
pub const CUnit__6__m_uberSplatId__OFFSET: usize = 1800;
pub const CUnit__6__m_vertexColor__OFFSET: usize = 1804;
pub const CUnit__6__m_anim_size__OFFSET: usize = 1808;
pub const CUnit__6__m_anims__OFFSET: usize = 1812;
pub const CUnit__6__m_forcedAnimScale__OFFSET: usize = 1840;
pub const CUnit__6__m_walk_anim_speed__OFFSET: usize = 1844;
pub const CUnit__6__m_run_anim_speed__OFFSET: usize = 1848;
pub const CUnit__6__m_unit_ai__OFFSET: usize = 1852;

#[repr(C)]
pub struct COrder__1 {
    pub _opaque: [u8; 120],
}
pub const COrder__1__SIZE: usize = 120;
pub const COrder__1__NAME: &str = "COrder";
pub const COrder__1__s_COrder_pool__OFFSET: usize = 0;
pub const COrder__1__m_id__OFFSET: usize = 88;
pub const COrder__1__m_playerId__OFFSET: usize = 92;
pub const COrder__1__m_next__OFFSET: usize = 96;
pub const COrder__1__m_agent__OFFSET: usize = 108;

#[repr(C)]
pub struct COrderPoint__1 {
    pub _opaque: [u8; 160],
}
pub const COrderPoint__1__SIZE: usize = 160;
pub const COrderPoint__1__NAME: &str = "COrderPoint";
pub const COrderPoint__1__s_COrderPoint_pool__OFFSET: usize = 0;
pub const COrderPoint__1__m_x__OFFSET: usize = 120;
pub const COrderPoint__1__m_y__OFFSET: usize = 136;
pub const COrderPoint__1__m_pPointMoveRequest__OFFSET: usize = 152;

#[repr(C)]
pub struct COrderTarget__1 {
    pub _opaque: [u8; 232],
}
pub const COrderTarget__1__SIZE: usize = 232;
pub const COrderTarget__1__NAME: &str = "COrderTarget";
pub const COrderTarget__1__s_COrderTarget_pool__OFFSET: usize = 0;
pub const COrderTarget__1__m_target__OFFSET: usize = 160;
pub const COrderTarget__1__m_pTargetMoveRequest__OFFSET: usize = 176;
pub const COrderTarget__1__m_ghostImageId__OFFSET: usize = 184;
pub const COrderTarget__1__m_ghostTargetFlags__OFFSET: usize = 188;
pub const COrderTarget__1__m_ghostCategory__OFFSET: usize = 192;
pub const COrderTarget__1__m_ghostImageOwner__OFFSET: usize = 196;
pub const COrderTarget__1__m_ghostX__OFFSET: usize = 200;
pub const COrderTarget__1__m_ghostY__OFFSET: usize = 216;

#[repr(C)]
pub struct CUnitAI__3 {
    pub _opaque: [u8; 152],
}
pub const CUnitAI__3__SIZE: usize = 152;
pub const CUnitAI__3__NAME: &str = "CUnitAI";
pub const CUnitAI__3__s_CUnitAI_pool__OFFSET: usize = 0;
pub const CUnitAI__3__m_captain__OFFSET: usize = 88;
pub const CUnitAI__3__anticheat_dummy_array_24__OFFSET: usize = 100;
pub const CUnitAI__3__m_mode__OFFSET: usize = 108;
pub const CUnitAI__3__m_town__OFFSET: usize = 112;
pub const CUnitAI__3__m_flags__OFFSET: usize = 124;
pub const CUnitAI__3__m_post__OFFSET: usize = 128;
pub const CUnitAI__3__m_data__OFFSET: usize = 140;
pub const CUnitAI__3__m_thinker__OFFSET: usize = 144;

#[repr(C)]
pub struct COrderPoint2__1 {
    pub _opaque: [u8; 192],
}
pub const COrderPoint2__1__SIZE: usize = 192;
pub const COrderPoint2__1__NAME: &str = "COrderPoint2";
pub const COrderPoint2__1__s_COrderPoint2_pool__OFFSET: usize = 0;
pub const COrderPoint2__1__m_x2__OFFSET: usize = 160;
pub const COrderPoint2__1__m_y2__OFFSET: usize = 176;

#[repr(C)]
pub struct COrderTarget2__1 {
    pub _opaque: [u8; 248],
}
pub const COrderTarget2__1__SIZE: usize = 248;
pub const COrderTarget2__1__NAME: &str = "COrderTarget2";
pub const COrderTarget2__1__s_COrderTarget2_pool__OFFSET: usize = 0;
pub const COrderTarget2__1__m_target2__OFFSET: usize = 232;

#[repr(C)]
pub struct COrderTrainCancel__1 {
    pub _opaque: [u8; 128],
}
pub const COrderTrainCancel__1__SIZE: usize = 128;
pub const COrderTrainCancel__1__NAME: &str = "COrderTrainCancel";
pub const COrderTrainCancel__1__s_COrderTrainCancel_pool__OFFSET: usize = 0;
pub const COrderTrainCancel__1__m_slot__OFFSET: usize = 120;
pub const COrderTrainCancel__1__m_trainableId__OFFSET: usize = 124;

#[repr(C)]
pub struct COrderReviveCancel__1 {
    pub _opaque: [u8; 136],
}
pub const COrderReviveCancel__1__SIZE: usize = 136;
pub const COrderReviveCancel__1__NAME: &str = "COrderReviveCancel";
pub const COrderReviveCancel__1__s_COrderReviveCancel_pool__OFFSET: usize = 0;
pub const COrderReviveCancel__1__m_hero__OFFSET: usize = 120;

#[repr(C)]
pub struct CGameWar3__9 {
    pub _opaque: [u8; 12872],
}
pub const CGameWar3__9__SIZE: usize = 12872;
pub const CGameWar3__9__NAME: &str = "CGameWar3";
pub const CGameWar3__9__s_campaignArchiveFile__OFFSET: usize = 0;
pub const CGameWar3__9__anticheat_dummy_array_29__OFFSET: usize = 9640;
pub const CGameWar3__9__s_pGameWar3__OFFSET: usize = 0;
pub const CGameWar3__9__anticheat_dummy_array_30__OFFSET: usize = 9695;
pub const CGameWar3__9__m_mapScriptInstance__OFFSET: usize = 9784;
pub const CGameWar3__9__m_mapScriptInstanceId__OFFSET: usize = 9792;
pub const CGameWar3__9__m_mapScript__OFFSET: usize = 9800;
pub const CGameWar3__9__m_mapScriptId__OFFSET: usize = 9808;
pub const CGameWar3__9__m_executions__OFFSET: usize = 9816;
pub const CGameWar3__9__anticheat_dummy_array_31__OFFSET: usize = 9840;
pub const CGameWar3__9__m_gameIdMap__OFFSET: usize = 9864;
pub const CGameWar3__9__m_gameState__OFFSET: usize = 9872;
pub const CGameWar3__9__m_buildId__OFFSET: usize = 9880;
pub const CGameWar3__9__m_localPlayerId__OFFSET: usize = 9884;
pub const CGameWar3__9__m_replayLocalPlayerId__OFFSET: usize = 9886;
pub const CGameWar3__9__m_playerIdBitsToGhost__OFFSET: usize = 9888;
pub const CGameWar3__9__m_playerIdBitsToGhostSet__OFFSET: usize = 9892;
pub const CGameWar3__9__m_pMapSetup__OFFSET: usize = 9896;
pub const CGameWar3__9__m_pFog__OFFSET: usize = 9904;
pub const CGameWar3__9__m_gameName__OFFSET: usize = 9912;
pub const CGameWar3__9__anticheat_dummy_array_32__OFFSET: usize = 9936;
pub const CGameWar3__9__m_maxPlayerCount__OFFSET: usize = 9952;
pub const CGameWar3__9__m_playerCount__OFFSET: usize = 9956;
pub const CGameWar3__9__m_gameMaxPlayerCount__OFFSET: usize = 9960;
pub const CGameWar3__9__m_maxTeamCount__OFFSET: usize = 9964;
pub const CGameWar3__9__m_teams__OFFSET: usize = 9968;
pub const CGameWar3__9__m_players__OFFSET: usize = 9992;
pub const CGameWar3__9__m_startLocations__OFFSET: usize = 10224;
pub const CGameWar3__9__m_slotToPlayerId__OFFSET: usize = 11960;
pub const CGameWar3__9__m_levelToTransitionTo__OFFSET: usize = 12064;
pub const CGameWar3__9__m_modelCine__OFFSET: usize = 12088;
pub const CGameWar3__9__m_cine__OFFSET: usize = 12112;
pub const CGameWar3__9__m_gameOverMessage__OFFSET: usize = 12376;
pub const CGameWar3__9__m_switchingLevels__OFFSET: usize = 12400;
pub const CGameWar3__9__m_doScoreScreen__OFFSET: usize = 12401;
pub const CGameWar3__9__m_isReload__OFFSET: usize = 12402;
pub const CGameWar3__9__anticheat_dummy_array_33__OFFSET: usize = 12403;
pub const CGameWar3__9__m_wasScriptLoad__OFFSET: usize = 12410;
pub const CGameWar3__9__m_wasLoadGame__OFFSET: usize = 12411;
pub const CGameWar3__9__m_isCampaignMission__OFFSET: usize = 12412;
pub const CGameWar3__9__m_campaignIndex__OFFSET: usize = 12416;
pub const CGameWar3__9__m_gameSetup__OFFSET: usize = 12420;
pub const CGameWar3__9__m_usingDarkMask__OFFSET: usize = 12678;
pub const CGameWar3__9__m_unseenQuests__OFFSET: usize = 12679;
pub const CGameWar3__9__m_showAllUnitsCmdBar__OFFSET: usize = 12680;
pub const CGameWar3__9__m_isReplay__OFFSET: usize = 12681;
pub const CGameWar3__9__m_replayFile__OFFSET: usize = 12688;
pub const CGameWar3__9__m_isTournament__OFFSET: usize = 12712;
pub const CGameWar3__9__m_copySaveCount__OFFSET: usize = 12716;
pub const CGameWar3__9__m_finishedInit__OFFSET: usize = 12720;
pub const CGameWar3__9__m_didSaveGameFromScript__OFFSET: usize = 12721;
pub const CGameWar3__9__m_playingGame__OFFSET: usize = 12722;
pub const CGameWar3__9__m_FacialAnimationSetScopeID__OFFSET: usize = 12724;
pub const CGameWar3__9__m_pTriggerStrings__OFFSET: usize = 12728;
pub const CGameWar3__9__anticheat_dummy_array_34__OFFSET: usize = 12736;
pub const CGameWar3__9__m_messageLogContents__OFFSET: usize = 12776;
pub const CGameWar3__9__m_pGameHashTableManager__OFFSET: usize = 12800;
pub const CGameWar3__9__m_gameCacheManager__OFFSET: usize = 12808;
pub const CGameWar3__9__m_gameOverCallback__OFFSET: usize = 12816;
pub const CGameWar3__9__m_maxCheckpointSaves__OFFSET: usize = 12824;
pub const CGameWar3__9__m_pSaveThread__OFFSET: usize = 12832;
pub const CGameWar3__9__m_pCheckpointPopup__OFFSET: usize = 12840;
pub const CGameWar3__9__m_gamePlayStartTime__OFFSET: usize = 12848;
pub const CGameWar3__9__m_latency__OFFSET: usize = 12856;

#[repr(C)]
pub struct CPlayerEventReg__1 {
    pub _opaque: [u8; 136],
}
pub const CPlayerEventReg__1__SIZE: usize = 136;
pub const CPlayerEventReg__1__NAME: &str = "CPlayerEventReg";
pub const CPlayerEventReg__1__s_CPlayerEventReg_pool__OFFSET: usize = 0;
pub const CPlayerEventReg__1__m_eventId__OFFSET: usize = 104;
pub const CPlayerEventReg__1__m_player__OFFSET: usize = 108;
pub const CPlayerEventReg__1__m_state__OFFSET: usize = 120;
pub const CPlayerEventReg__1__m_unitFilter__OFFSET: usize = 128;

#[repr(C)]
pub struct CPlayerWar3FrameEventReg__1 {
    pub _opaque: [u8; 112],
}
pub const CPlayerWar3FrameEventReg__1__SIZE: usize = 112;
pub const CPlayerWar3FrameEventReg__1__NAME: &str = "CPlayerWar3FrameEventReg";
pub const CPlayerWar3FrameEventReg__1__s_CPlayerWar3FrameEventReg_pool__OFFSET: usize = 0;
pub const CPlayerWar3FrameEventReg__1__m_frame__OFFSET: usize = 104;

#[repr(C)]
pub struct CPlayerWar3FrameEventData__1 {
    pub _opaque: [u8; 104],
}
pub const CPlayerWar3FrameEventData__1__SIZE: usize = 104;
pub const CPlayerWar3FrameEventData__1__NAME: &str = "CPlayerWar3FrameEventData";
pub const CPlayerWar3FrameEventData__1__s_CPlayerWar3FrameEventData_pool__OFFSET: usize = 0;
pub const CPlayerWar3FrameEventData__1__m_frame__OFFSET: usize = 56;
pub const CPlayerWar3FrameEventData__1__m_eventId__OFFSET: usize = 68;
pub const CPlayerWar3FrameEventData__1__m_val__OFFSET: usize = 72;
pub const CPlayerWar3FrameEventData__1__m_text__OFFSET: usize = 80;

#[repr(C)]
pub struct CGameState__1 {
    pub _opaque: [u8; 1416],
}
pub const CGameState__1__SIZE: usize = 1416;
pub const CGameState__1__NAME: &str = "CGameState";
pub const CGameState__1__s_CGameState_pool__OFFSET: usize = 0;
pub const CGameState__1__m_gameStatesFloat__OFFSET: usize = 88;
pub const CGameState__1__m_gameStatesInt__OFFSET: usize = 120;
pub const CGameState__1__m_timeScale__OFFSET: usize = 176;
pub const CGameState__1__m_todHaltFlags__OFFSET: usize = 200;
pub const CGameState__1__m_todMod__OFFSET: usize = 208;
pub const CGameState__1__m_todLis__OFFSET: usize = 216;
pub const CGameState__1__m_dawnLis__OFFSET: usize = 224;
pub const CGameState__1__m_duskLis__OFFSET: usize = 232;
pub const CGameState__1__m_isDayTime__OFFSET: usize = 240;
pub const CGameState__1__m_agentStacks__OFFSET: usize = 264;
pub const CGameState__1__m_unusedHandles__OFFSET: usize = 632;
pub const CGameState__1__m_handle2AgentMap__OFFSET: usize = 656;
pub const CGameState__1__m_agent2HandleMap__OFFSET: usize = 680;
pub const CGameState__1__m_func2AgentMap__OFFSET: usize = 752;
pub const CGameState__1__m_filter2AgentMap__OFFSET: usize = 824;
pub const CGameState__1__m_frame2AgentMap__OFFSET: usize = 896;
pub const CGameState__1__m_variableEvents__OFFSET: usize = 928;
pub const CGameState__1__m_varChangedEvent__OFFSET: usize = 1000;
pub const CGameState__1__m_heroNamesUsed__OFFSET: usize = 1024;
pub const CGameState__1__m_fogTimer__OFFSET: usize = 1096;
pub const CGameState__1__m_pathingTimer__OFFSET: usize = 1128;
pub const CGameState__1__m_levelChangeTimer__OFFSET: usize = 1160;
pub const CGameState__1__m_scriptGCTimer__OFFSET: usize = 1192;
pub const CGameState__1__m_questManager__OFFSET: usize = 1224;
pub const CGameState__1__m_textTagManager__OFFSET: usize = 1240;

#[repr(C)]
pub struct CGameCache__1 {
    pub _opaque: [u8; 112],
}
pub const CGameCache__1__SIZE: usize = 112;
pub const CGameCache__1__NAME: &str = "CGameCache";
pub const CGameCache__1__s_CGameCache_pool__OFFSET: usize = 0;
pub const CGameCache__1__m_campaignKey__OFFSET: usize = 88;

#[repr(C)]
pub struct CGameCacheManager__1 {
    pub _opaque: [u8; 216],
}
pub const CGameCacheManager__1__SIZE: usize = 216;
pub const CGameCacheManager__1__NAME: &str = "CGameCacheManager";
pub const CGameCacheManager__1__s_CGameCacheManager_pool__OFFSET: usize = 0;
pub const CGameCacheManager__1__m_campaignFilePath__OFFSET: usize = 88;
pub const CGameCacheManager__1__m_campaignKey__OFFSET: usize = 112;
pub const CGameCacheManager__1__m_campaigns__OFFSET: usize = 136;
pub const CGameCacheManager__1__m_campaignCount__OFFSET: usize = 208;

#[repr(C)]
pub struct CGameHashTable__1 {
    pub _opaque: [u8; 96],
}
pub const CGameHashTable__1__SIZE: usize = 96;
pub const CGameHashTable__1__NAME: &str = "CGameHashTable";
pub const CGameHashTable__1__s_CGameHashTable_pool__OFFSET: usize = 0;
pub const CGameHashTable__1__m_parentKey__OFFSET: usize = 88;

#[repr(C)]
pub struct CPlayerWar3__9 {
    pub _opaque: [u8; 1960],
}
pub const CPlayerWar3__9__SIZE: usize = 1960;
pub const CPlayerWar3__9__NAME: &str = "CPlayerWar3";
pub const CPlayerWar3__9__s_CPlayerWar3_pool__OFFSET: usize = 0;
pub const CPlayerWar3__9__anticheat_dummy_array_25__OFFSET: usize = 88;
pub const CPlayerWar3__9__m_playerId__OFFSET: usize = 103;
pub const CPlayerWar3__9__m_playerName__OFFSET: usize = 104;
pub const CPlayerWar3__9__m_toonName__OFFSET: usize = 128;
pub const CPlayerWar3__9__m_protobufPlayerData__OFFSET: usize = 152;
pub const CPlayerWar3__9__m_slotNetPlayerId__OFFSET: usize = 336;
pub const CPlayerWar3__9__m_playerNamePrefix__OFFSET: usize = 344;
pub const CPlayerWar3__9__m_pAlliances__OFFSET: usize = 368;
pub const CPlayerWar3__9__m_pSelection__OFFSET: usize = 376;
pub const CPlayerWar3__9__m_playerStates__OFFSET: usize = 384;
pub const CPlayerWar3__9__m_lumberDivertTax__OFFSET: usize = 1016;
pub const CPlayerWar3__9__m_goldDivertTax__OFFSET: usize = 1116;
pub const CPlayerWar3__9__m_highestFood__OFFSET: usize = 1216;
pub const CPlayerWar3__9__m_highestFoodUsed__OFFSET: usize = 1220;
pub const CPlayerWar3__9__anticheat_dummy_array_26__OFFSET: usize = 1224;
pub const CPlayerWar3__9__m_heroKills__OFFSET: usize = 1240;
pub const CPlayerWar3__9__m_itemsObtained__OFFSET: usize = 1244;
pub const CPlayerWar3__9__m_mercsHired__OFFSET: usize = 1248;
pub const CPlayerWar3__9__m_teamColorIndex__OFFSET: usize = 1252;
pub const CPlayerWar3__9__m_controller__OFFSET: usize = 1256;
pub const CPlayerWar3__9__m_startLocIndex__OFFSET: usize = 1260;
pub const CPlayerWar3__9__m_difficulty__OFFSET: usize = 1264;
pub const CPlayerWar3__9__m_slotState__OFFSET: usize = 1268;
pub const CPlayerWar3__9__m_teamIndex__OFFSET: usize = 1272;
pub const CPlayerWar3__9__m_numActions__OFFSET: usize = 1276;
pub const CPlayerWar3__9__m_racePref__OFFSET: usize = 1280;
pub const CPlayerWar3__9__m_race__OFFSET: usize = 1284;
pub const CPlayerWar3__9__m_realTimeAPM__OFFSET: usize = 1288;
pub const CPlayerWar3__9__m_capChangedListener__OFFSET: usize = 1384;
pub const CPlayerWar3__9__m_usedChangedListener__OFFSET: usize = 1392;
pub const CPlayerWar3__9__m_foodUsedUpkeepListener__OFFSET: usize = 1400;
pub const CPlayerWar3__9__m_lumberChangedListener__OFFSET: usize = 1408;
pub const CPlayerWar3__9__m_goldChangedListener__OFFSET: usize = 1416;
pub const CPlayerWar3__9__m_goldUpkeepChanged__OFFSET: usize = 1424;
pub const CPlayerWar3__9__m_lumberUpkeepChanged__OFFSET: usize = 1432;
pub const CPlayerWar3__9__m_handicap__OFFSET: usize = 1440;
pub const CPlayerWar3__9__m_referee__OFFSET: usize = 1512;
pub const CPlayerWar3__9__m_deadHeroAge__OFFSET: usize = 1520;
pub const CPlayerWar3__9__m_deadHeroes__OFFSET: usize = 1544;
pub const CPlayerWar3__9__m_revivableFlags__OFFSET: usize = 1568;
pub const CPlayerWar3__9__anticheat_dummy_array_27__OFFSET: usize = 1572;
pub const CPlayerWar3__9__m_pTechTree__OFFSET: usize = 1584;
pub const CPlayerWar3__9__m_fogMask__OFFSET: usize = 1592;
pub const CPlayerWar3__9__m_ai__OFFSET: usize = 1596;
pub const CPlayerWar3__9__m_bestHeroes__OFFSET: usize = 1608;
pub const CPlayerWar3__9__m_leaderboard__OFFSET: usize = 1632;
pub const CPlayerWar3__9__m_leaveGameTimer__OFFSET: usize = 1648;
pub const CPlayerWar3__9__anticheat_dummy_array_28__OFFSET: usize = 1680;
pub const CPlayerWar3__9__m_doScoreScreen__OFFSET: usize = 1708;
pub const CPlayerWar3__9__m_allowContinue__OFFSET: usize = 1709;
pub const CPlayerWar3__9__m_crippled__OFFSET: usize = 1712;
pub const CPlayerWar3__9__m_gameOverMessage__OFFSET: usize = 1720;
pub const CPlayerWar3__9__m_showInScoreScreen__OFFSET: usize = 1744;
pub const CPlayerWar3__9__m_cachedScore__OFFSET: usize = 1748;
pub const CPlayerWar3__9__m_cachedScoreIsCached__OFFSET: usize = 1752;
pub const CPlayerWar3__9__m_heroCount__OFFSET: usize = 1756;
pub const CPlayerWar3__9__m_bnetPlayerId__OFFSET: usize = 1760;
pub const CPlayerWar3__9__m_heroPickOrder__OFFSET: usize = 1768;
pub const CPlayerWar3__9__m_unitKills__OFFSET: usize = 1792;
pub const CPlayerWar3__9__m_unitsCreated__OFFSET: usize = 1824;
pub const CPlayerWar3__9__m_abilityInfo__OFFSET: usize = 1856;
pub const CPlayerWar3__9__m_upkeepTime__OFFSET: usize = 1888;
pub const CPlayerWar3__9__m_lastUpkeepTime__OFFSET: usize = 1920;
pub const CPlayerWar3__9__m_playerItemUseMap__OFFSET: usize = 1928;

#[repr(C)]
pub struct MapInformation {
    pub _opaque: [u8; 16],
}
pub const MapInformation__SIZE: usize = 16;
pub const MapInformation__NAME: &str = "MapInformation";
pub const MapInformation__maps__OFFSET: usize = 0;
pub const MapInformation__document__OFFSET: usize = 8;

#[repr(C)]
pub struct MapDownloadTimer {
    pub _opaque: [u8; 72],
}
pub const MapDownloadTimer__SIZE: usize = 72;
pub const MapDownloadTimer__NAME: &str = "MapDownloadTimer";
pub const MapDownloadTimer__MINMUM_DOWNLOAD_TIMER__OFFSET: usize = 16;
pub const MapDownloadTimer__m_eventId__OFFSET: usize = 20;
pub const MapDownloadTimer__timerExpired__OFFSET: usize = 24;
pub const MapDownloadTimer__m_mapDownloadCheckTimer__OFFSET: usize = 32;

#[repr(C)]
pub struct WidgetReplacementParms {
    pub _opaque: [u8; 40],
}
pub const WidgetReplacementParms__SIZE: usize = 40;
pub const WidgetReplacementParms__NAME: &str = "WidgetReplacementParms";
pub const WidgetReplacementParms__ourCliffLevel__OFFSET: usize = 0;
pub const WidgetReplacementParms__extents__OFFSET: usize = 4;
pub const WidgetReplacementParms__enumerator__OFFSET: usize = 24;
pub const WidgetReplacementParms__placementPos__OFFSET: usize = 32;

#[repr(C)]
pub struct CUnitAI__4 {
    pub _opaque: [u8; 152],
}
pub const CUnitAI__4__SIZE: usize = 152;
pub const CUnitAI__4__NAME: &str = "CUnitAI";
pub const CUnitAI__4__s_CUnitAI_pool__OFFSET: usize = 0;
pub const CUnitAI__4__m_captain__OFFSET: usize = 88;
pub const CUnitAI__4__anticheat_dummy_array_11__OFFSET: usize = 100;
pub const CUnitAI__4__m_mode__OFFSET: usize = 108;
pub const CUnitAI__4__m_town__OFFSET: usize = 112;
pub const CUnitAI__4__m_flags__OFFSET: usize = 124;
pub const CUnitAI__4__m_post__OFFSET: usize = 128;
pub const CUnitAI__4__m_data__OFFSET: usize = 140;
pub const CUnitAI__4__m_thinker__OFFSET: usize = 144;

#[repr(C)]
pub struct CGameUI__13 {
    pub _opaque: [u8; 2384],
}
pub const CGameUI__13__SIZE: usize = 2384;
pub const CGameUI__13__NAME: &str = "CGameUI";
pub const CGameUI__13__m_transitionState__OFFSET: usize = 704;
pub const CGameUI__13__m_cachedFonts__OFFSET: usize = 712;
pub const CGameUI__13__NUMBER_OF_PLAYER_ROWS__OFFSET: usize = 0;
pub const CGameUI__13__FIRST_ROW_Y_OFFSET__OFFSET: usize = 0;
pub const CGameUI__13__ROW_SPACING__OFFSET: usize = 0;
pub const CGameUI__13__m_userControl__OFFSET: usize = 784;
pub const CGameUI__13__m_userUI__OFFSET: usize = 785;
pub const CGameUI__13__m_currentInputMode__OFFSET: usize = 792;
pub const CGameUI__13__m_inputModeStack__OFFSET: usize = 800;
pub const CGameUI__13__m_sharedTooltip__OFFSET: usize = 824;
pub const CGameUI__13__m_uberTooltip__OFFSET: usize = 832;
pub const CGameUI__13__m_showUberTips__OFFSET: usize = 840;
pub const CGameUI__13__m_autosaveReplay__OFFSET: usize = 841;
pub const CGameUI__13__m_healthBarsMode__OFFSET: usize = 844;
pub const CGameUI__13__m_forceShowHealthBars__OFFSET: usize = 848;
pub const CGameUI__13__m_minimapSignals__OFFSET: usize = 849;
pub const CGameUI__13__m_useColoredHealthBars__OFFSET: usize = 850;
pub const CGameUI__13__m_useHeroFrames__OFFSET: usize = 851;
pub const CGameUI__13__m_showHeroLevelFrame__OFFSET: usize = 852;
pub const CGameUI__13__m_showNumericCooldown__OFFSET: usize = 853;
pub const CGameUI__13__m_showDenyIcon__OFFSET: usize = 854;
pub const CGameUI__13__m_showGoldmineUnitCounter__OFFSET: usize = 855;
pub const CGameUI__13__m_enableObserverUiV2__OFFSET: usize = 856;
pub const CGameUI__13__m_asyncPlayerData__OFFSET: usize = 860;
pub const CGameUI__13__m_targetMode__OFFSET: usize = 960;
pub const CGameUI__13__m_selectMode__OFFSET: usize = 968;
pub const CGameUI__13__m_dragSelectMode__OFFSET: usize = 976;
pub const CGameUI__13__m_dragScrollMode__OFFSET: usize = 984;
pub const CGameUI__13__m_buildMode__OFFSET: usize = 992;
pub const CGameUI__13__m_signalMode__OFFSET: usize = 1000;
pub const CGameUI__13__m_escapeMenu__OFFSET: usize = 1008;
pub const CGameUI__13__m_gameOverMenu__OFFSET: usize = 1016;
pub const CGameUI__13__m_suspendIndicator__OFFSET: usize = 1024;
pub const CGameUI__13__m_unresponsiveIndicator__OFFSET: usize = 1032;
pub const CGameUI__13__m_allianceMode__OFFSET: usize = 1040;
pub const CGameUI__13__m_chatMode__OFFSET: usize = 1048;
pub const CGameUI__13__m_questMode__OFFSET: usize = 1056;
pub const CGameUI__13__anticheat_dummy_array_22__OFFSET: usize = 1064;
pub const CGameUI__13__m_scriptDialogMode__OFFSET: usize = 1152;
pub const CGameUI__13__m_unresponsiveDelayTimerId__OFFSET: usize = 1160;
pub const CGameUI__13__m_selectionManager__OFFSET: usize = 1168;
pub const CGameUI__13__m_dragScrollManager__OFFSET: usize = 1176;
pub const CGameUI__13__m_camera__OFFSET: usize = 1184;
pub const CGameUI__13__m_paused__OFFSET: usize = 1192;
pub const CGameUI__13__m_uiPaused__OFFSET: usize = 1193;
pub const CGameUI__13__m_scriptPaused__OFFSET: usize = 1194;
pub const CGameUI__13__m_disableUpkeepMsg__OFFSET: usize = 1195;
pub const CGameUI__13__m_upkeepLvlSave__OFFSET: usize = 1196;
pub const CGameUI__13__m_refresh__OFFSET: usize = 1200;
pub const CGameUI__13__m_refreshWaypoints__OFFSET: usize = 1201;
pub const CGameUI__13__m_deferCleanSelUI__OFFSET: usize = 1202;
pub const CGameUI__13__m_refreshInfoBar__OFFSET: usize = 1203;
pub const CGameUI__13__m_refreshCommandBar__OFFSET: usize = 1204;
pub const CGameUI__13__m_refreshHeroBar__OFFSET: usize = 1205;
pub const CGameUI__13__m_popSubMenu__OFFSET: usize = 1206;
pub const CGameUI__13__m_activeValidOrder__OFFSET: usize = 1208;
pub const CGameUI__13__m_cineForceSubtitles__OFFSET: usize = 1212;
pub const CGameUI__13__m_cinematicPanelsHidden__OFFSET: usize = 1213;
pub const CGameUI__13__m_dragSelectEnabled__OFFSET: usize = 1214;
pub const CGameUI__13__m_dragSelectUIEnabled__OFFSET: usize = 1215;
pub const CGameUI__13__m_preSelectEnabled__OFFSET: usize = 1216;
pub const CGameUI__13__m_preSelectUIEnabled__OFFSET: usize = 1217;
pub const CGameUI__13__m_selectEnabled__OFFSET: usize = 1218;
pub const CGameUI__13__m_selectUIEnabled__OFFSET: usize = 1219;
pub const CGameUI__13__m_observerModeEnabled__OFFSET: usize = 1220;
pub const CGameUI__13__m_observerModeV2Active__OFFSET: usize = 1221;
pub const CGameUI__13__m_wideScreenAutoPosition__OFFSET: usize = 1222;
pub const CGameUI__13__m_fadeTimer__OFFSET: usize = 1224;
pub const CGameUI__13__m_cinematicFadeTimer__OFFSET: usize = 1328;
pub const CGameUI__13__anticheat_dummy_array_23__OFFSET: usize = 1432;
pub const CGameUI__13__m_frameParentTimer__OFFSET: usize = 1440;
pub const CGameUI__13__m_simpleFadeTimer__OFFSET: usize = 1544;
pub const CGameUI__13__m_attackNotify__OFFSET: usize = 1552;
pub const CGameUI__13__m_warpNotify__OFFSET: usize = 1576;
pub const CGameUI__13__m_timerDialogs__OFFSET: usize = 1608;
pub const CGameUI__13__m_leaderboard__OFFSET: usize = 1632;
pub const CGameUI__13__anticheat_dummy_array_24__OFFSET: usize = 1640;
pub const CGameUI__13__m_multiboard__OFFSET: usize = 1648;
pub const CGameUI__13__m_suspendMultiboardDisplay__OFFSET: usize = 1656;
pub const CGameUI__13__anticheat_dummy_array_25__OFFSET: usize = 1657;
pub const CGameUI__13__m_scriptDialog__OFFSET: usize = 1728;
pub const CGameUI__13__m_replaySelection__OFFSET: usize = 1736;
pub const CGameUI__13__m_unitStatusIconUIFactory__OFFSET: usize = 1744;
pub const CGameUI__13__m_unitStatusTexts__OFFSET: usize = 1752;
pub const CGameUI__13__m_worldFrame__OFFSET: usize = 1776;
pub const CGameUI__13__m_infoBar__OFFSET: usize = 1784;
pub const CGameUI__13__m_commandBar__OFFSET: usize = 1792;
pub const CGameUI__13__m_miniMap__OFFSET: usize = 1800;
pub const CGameUI__13__m_resourceBar__OFFSET: usize = 1808;
pub const CGameUI__13__m_upperButtonBar__OFFSET: usize = 1816;
pub const CGameUI__13__m_observerButtonBar__OFFSET: usize = 1824;
pub const CGameUI__13__m_playerStatisticsPanel__OFFSET: usize = 1832;
pub const CGameUI__13__m_heroPanel__OFFSET: usize = 1840;
pub const CGameUI__13__m_productionPanel__OFFSET: usize = 1848;
pub const CGameUI__13__m_minimapButtonBar__OFFSET: usize = 1856;
pub const CGameUI__13__m_trackingStub__OFFSET: usize = 1864;
pub const CGameUI__13__m_heroBar__OFFSET: usize = 1872;
pub const CGameUI__13__m_peonBar__OFFSET: usize = 1880;
pub const CGameUI__13__m_messageBox__OFFSET: usize = 1888;
pub const CGameUI__13__m_unitMessageBox__OFFSET: usize = 1896;
pub const CGameUI__13__m_chatMessageBox__OFFSET: usize = 1904;
pub const CGameUI__13__m_topMessageBox__OFFSET: usize = 1912;
pub const CGameUI__13__m_currentPortraitButton__OFFSET: usize = 1920;
pub const CGameUI__13__m_timeOfDayIndicator__OFFSET: usize = 1928;
pub const CGameUI__13__m_chatEditBar__OFFSET: usize = 1936;
pub const CGameUI__13__m_cinematicPanel__OFFSET: usize = 1944;
pub const CGameUI__13__m_lastBuffBar__OFFSET: usize = 1952;
pub const CGameUI__13__m_observerBar__OFFSET: usize = 1960;
pub const CGameUI__13__m_miniMapSignalButton__OFFSET: usize = 1968;
pub const CGameUI__13__m_miniMapTerrainButton__OFFSET: usize = 1976;
pub const CGameUI__13__m_miniMapAllyButton__OFFSET: usize = 1984;
pub const CGameUI__13__m_miniMapCreepButton__OFFSET: usize = 1992;
pub const CGameUI__13__m_formationButton__OFFSET: usize = 2000;
pub const CGameUI__13__m_consoleFrame__OFFSET: usize = 2008;
pub const CGameUI__13__m_simpleConsoleFrame__OFFSET: usize = 2016;
pub const CGameUI__13__m_simpleConsoleBottomBarFrame__OFFSET: usize = 2024;
pub const CGameUI__13__m_simpleConsoleBottomBarFrameOverlay__OFFSET: usize = 2032;
pub const CGameUI__13__m_frameUIParent__OFFSET: usize = 2040;
pub const CGameUI__13__top__OFFSET: usize = 2048;
pub const CGameUI__13__topInGame__OFFSET: usize = 2056;
pub const CGameUI__13__m_simpleUIParent__OFFSET: usize = 2064;
pub const CGameUI__13__m_observerUIParent__OFFSET: usize = 2072;
pub const CGameUI__13__m_observerInfoPanelBackground__OFFSET: usize = 2080;
pub const CGameUI__13__m_observerInfoPanelBackpack__OFFSET: usize = 2088;
pub const CGameUI__13__m_consoleBackdropFrame__OFFSET: usize = 2096;
pub const CGameUI__13__m_minimapTextureBackground__OFFSET: usize = 2104;
pub const CGameUI__13__m_infoPanelTexture__OFFSET: usize = 2112;
pub const CGameUI__13__m_quickSaveKey__OFFSET: usize = 2120;
pub const CGameUI__13__m_quickLoadKey__OFFSET: usize = 2124;
pub const CGameUI__13__m_quickHelpKey__OFFSET: usize = 2128;
pub const CGameUI__13__m_quickOptionsKey__OFFSET: usize = 2132;
pub const CGameUI__13__m_quickQuitKey__OFFSET: usize = 2136;
pub const CGameUI__13__m_minimapSignalKey__OFFSET: usize = 2140;
pub const CGameUI__13__m_minimapTerrainKey__OFFSET: usize = 2144;
pub const CGameUI__13__m_minimapColorsKey__OFFSET: usize = 2148;
pub const CGameUI__13__m_minimapCreepKey__OFFSET: usize = 2152;
pub const CGameUI__13__m_formationToggleKey__OFFSET: usize = 2156;
pub const CGameUI__13__m_observerToggleAllPanelsKey__OFFSET: usize = 2160;
pub const CGameUI__13__m_observerProductionModeKey__OFFSET: usize = 2164;
pub const CGameUI__13__m_observerUnitsModeKey__OFFSET: usize = 2168;
pub const CGameUI__13__m_mouseDownHandler__OFFSET: usize = 2176;
pub const CGameUI__13__m_mouseWheelHandler__OFFSET: usize = 2184;
pub const CGameUI__13__m_mouseUpHandler__OFFSET: usize = 2192;
pub const CGameUI__13__m_mouseMoveHandler__OFFSET: usize = 2200;
pub const CGameUI__13__m_observerToggleablePanels__OFFSET: usize = 2208;
pub const CGameUI__13__m_consoleBackdropHeight__OFFSET: usize = 2240;
pub const CGameUI__13__m_auroraChatHandler__OFFSET: usize = 2248;
pub const CGameUI__13__m_lobbyEventsHandler__OFFSET: usize = 2328;
pub const CGameUI__13__s_indexedCustomHotkeyMappingMaxCount__OFFSET: usize = 0;
pub const CGameUI__13__s_customHotkeyEnabled__OFFSET: usize = 0;
pub const CGameUI__13__s_customHotkeyMappingTable__OFFSET: usize = 0;
pub const CGameUI__13__s_customHotkeyMappingTableNonHero__OFFSET: usize = 0;
pub const CGameUI__13__m_observerUnitToFollow__OFFSET: usize = 2376;

#[repr(C)]
pub struct WidgetDamageData {
    pub _opaque: [u8; 16],
}
pub const WidgetDamageData__SIZE: usize = 16;
pub const WidgetDamageData__NAME: &str = "WidgetDamageData";
pub const WidgetDamageData__widget__OFFSET: usize = 0;
pub const WidgetDamageData__damage__OFFSET: usize = 8;
pub const WidgetDamageData__victimType__OFFSET: usize = 12;

#[repr(C)]
pub struct UnitDataNode {
    pub _opaque: [u8; 728],
}
pub const UnitDataNode__SIZE: usize = 728;
pub const UnitDataNode__NAME: &str = "UnitDataNode";
pub const UnitDataNode__buildTime__OFFSET: usize = 48;
pub const UnitDataNode__repairTime__OFFSET: usize = 52;
pub const UnitDataNode__goldCost__OFFSET: usize = 56;
pub const UnitDataNode__lumberCost__OFFSET: usize = 60;
pub const UnitDataNode__goldRepairCost__OFFSET: usize = 64;
pub const UnitDataNode__lumberRepairCost__OFFSET: usize = 68;
pub const UnitDataNode__goldBountyDice__OFFSET: usize = 72;
pub const UnitDataNode__goldBountySides__OFFSET: usize = 76;
pub const UnitDataNode__goldBountyPlus__OFFSET: usize = 80;
pub const UnitDataNode__woodBountyDice__OFFSET: usize = 84;
pub const UnitDataNode__woodBountySides__OFFSET: usize = 88;
pub const UnitDataNode__woodBountyPlus__OFFSET: usize = 92;
pub const UnitDataNode__stockMax__OFFSET: usize = 96;
pub const UnitDataNode__stockRegenRate__OFFSET: usize = 100;
pub const UnitDataNode__stockStartDelay__OFFSET: usize = 104;
pub const UnitDataNode__stockInitial__OFFSET: usize = 108;
pub const UnitDataNode__canSleep__OFFSET: usize = 112;
pub const UnitDataNode__canFlee__OFFSET: usize = 113;
pub const UnitDataNode__foodUsed__OFFSET: usize = 116;
pub const UnitDataNode__foodMade__OFFSET: usize = 120;
pub const UnitDataNode__cargoSize__OFFSET: usize = 124;
pub const UnitDataNode__expLevel__OFFSET: usize = 128;
pub const UnitDataNode__castPoint__OFFSET: usize = 132;
pub const UnitDataNode__castBackSwing__OFFSET: usize = 136;
pub const UnitDataNode__deathTime__OFFSET: usize = 140;
pub const UnitDataNode__regenType__OFFSET: usize = 144;
pub const UnitDataNode__regenRate__OFFSET: usize = 148;
pub const UnitDataNode__maxLife__OFFSET: usize = 152;
pub const UnitDataNode__startMana__OFFSET: usize = 156;
pub const UnitDataNode__maxMana__OFFSET: usize = 160;
pub const UnitDataNode__regenMana__OFFSET: usize = 164;
pub const UnitDataNode__baseDefense__OFFSET: usize = 168;
pub const UnitDataNode__defenseUpgrade__OFFSET: usize = 172;
pub const UnitDataNode__battleDefense__OFFSET: usize = 176;
pub const UnitDataNode__weaponsEnabled__OFFSET: usize = 180;
pub const UnitDataNode__attackTargetFlags__OFFSET: usize = 184;
pub const UnitDataNode__attackUpgrade__OFFSET: usize = 192;
pub const UnitDataNode__damageDice__OFFSET: usize = 200;
pub const UnitDataNode__damageSides__OFFSET: usize = 208;
pub const UnitDataNode__damageMod__OFFSET: usize = 216;
pub const UnitDataNode__targetCount__OFFSET: usize = 224;
pub const UnitDataNode__damageLoss__OFFSET: usize = 232;
pub const UnitDataNode__spillDistance__OFFSET: usize = 240;
pub const UnitDataNode__spillRadius__OFFSET: usize = 248;
pub const UnitDataNode__battleAttack__OFFSET: usize = 256;
pub const UnitDataNode__attackType__OFFSET: usize = 264;
pub const UnitDataNode__attackRange__OFFSET: usize = 272;
pub const UnitDataNode__attackRangeBuff__OFFSET: usize = 280;
pub const UnitDataNode__attackCooldown__OFFSET: usize = 288;
pub const UnitDataNode__damagePoint__OFFSET: usize = 296;
pub const UnitDataNode__backswing__OFFSET: usize = 304;
pub const UnitDataNode__splashTargetFlags__OFFSET: usize = 312;
pub const UnitDataNode__splashFullArea__OFFSET: usize = 320;
pub const UnitDataNode__splashHalfArea__OFFSET: usize = 328;
pub const UnitDataNode__splashQuarArea__OFFSET: usize = 336;
pub const UnitDataNode__splashHalfFactor__OFFSET: usize = 344;
pub const UnitDataNode__splashQuarFactor__OFFSET: usize = 352;
pub const UnitDataNode__damageMin__OFFSET: usize = 360;
pub const UnitDataNode__damageMax__OFFSET: usize = 368;
pub const UnitDataNode__strength__OFFSET: usize = 376;
pub const UnitDataNode__agility__OFFSET: usize = 380;
pub const UnitDataNode__intellect__OFFSET: usize = 384;
pub const UnitDataNode__primaryAttribute__OFFSET: usize = 388;
pub const UnitDataNode__strBonus__OFFSET: usize = 392;
pub const UnitDataNode__agiBonus__OFFSET: usize = 396;
pub const UnitDataNode__intBonus__OFFSET: usize = 400;
pub const UnitDataNode__sightRadius__OFFSET: usize = 404;
pub const UnitDataNode__nightSightRadius__OFFSET: usize = 408;
pub const UnitDataNode__acquireRadius__OFFSET: usize = 412;
pub const UnitDataNode__minAttackRange__OFFSET: usize = 416;
pub const UnitDataNode__collisionRadius__OFFSET: usize = 420;
pub const UnitDataNode__fogRadius__OFFSET: usize = 424;
pub const UnitDataNode__scaleBullets__OFFSET: usize = 428;
pub const UnitDataNode__moveType__OFFSET: usize = 432;
pub const UnitDataNode__pathingMaskUs__OFFSET: usize = 436;
pub const UnitDataNode__pathingMaskThem__OFFSET: usize = 440;
pub const UnitDataNode__targetTypeFlags__OFFSET: usize = 444;
pub const UnitDataNode__bufferType__OFFSET: usize = 448;
pub const UnitDataNode__bufferRadius__OFFSET: usize = 452;
pub const UnitDataNode__race__OFFSET: usize = 456;
pub const UnitDataNode__deathType__OFFSET: usize = 460;
pub const UnitDataNode__categories__OFFSET: usize = 464;
pub const UnitDataNode__priority__OFFSET: usize = 468;
pub const UnitDataNode__points__OFFSET: usize = 472;
pub const UnitDataNode__baseMoveSpeed__OFFSET: usize = 476;
pub const UnitDataNode__minSpeed__OFFSET: usize = 480;
pub const UnitDataNode__maxSpeed__OFFSET: usize = 484;
pub const UnitDataNode__steeringTurnRate__OFFSET: usize = 488;
pub const UnitDataNode__propWindow__OFFSET: usize = 492;
pub const UnitDataNode__orientInterp__OFFSET: usize = 496;
pub const UnitDataNode__formationPrio__OFFSET: usize = 500;
pub const UnitDataNode__occluderHeight__OFFSET: usize = 504;
pub const UnitDataNode__fatLOS__OFFSET: usize = 508;
pub const UnitDataNode__moveHeight__OFFSET: usize = 512;
pub const UnitDataNode__moveFloor__OFFSET: usize = 516;
pub const UnitDataNode__animBlendTime__OFFSET: usize = 520;
pub const UnitDataNode__walkAnimSpeed__OFFSET: usize = 524;
pub const UnitDataNode__runAnimSpeed__OFFSET: usize = 528;
pub const UnitDataNode__canRepulse__OFFSET: usize = 532;
pub const UnitDataNode__repulseParam__OFFSET: usize = 536;
pub const UnitDataNode__repulseGroup__OFFSET: usize = 540;
pub const UnitDataNode__repulsePrio__OFFSET: usize = 544;
pub const UnitDataNode__upgradeIds__OFFSET: usize = 552;
pub const UnitDataNode__upgradeTypes__OFFSET: usize = 576;
pub const UnitDataNode__meleeUpgradeId__OFFSET: usize = 580;
pub const UnitDataNode__armorUpgradeId__OFFSET: usize = 584;
pub const UnitDataNode__rangedUpgradeId__OFFSET: usize = 588;
pub const UnitDataNode__artilleryUpgradeId__OFFSET: usize = 592;
pub const UnitDataNode__casterUpgradeId__OFFSET: usize = 596;
pub const UnitDataNode__isBuildOn__OFFSET: usize = 600;
pub const UnitDataNode__canBuildOn__OFFSET: usize = 601;
pub const UnitDataNode__acceptsGold__OFFSET: usize = 602;
pub const UnitDataNode__mob__OFFSET: usize = 603;
pub const UnitDataNode__properNameCount__OFFSET: usize = 604;
pub const UnitDataNode__activeAutoTargetAbil__OFFSET: usize = 608;
pub const UnitDataNode__unitAbils__OFFSET: usize = 616;
pub const UnitDataNode__heroAbils__OFFSET: usize = 640;
pub const UnitDataNode__addon__OFFSET: usize = 664;
pub const UnitDataNode__isStructure__OFFSET: usize = 668;
pub const UnitDataNode__isSpecial__OFFSET: usize = 669;
pub const UnitDataNode__isHostile__OFFSET: usize = 670;
pub const UnitDataNode__canBeRandomNB__OFFSET: usize = 671;
pub const UnitDataNode__tilesetList__OFFSET: usize = 672;
pub const UnitDataNode__name__OFFSET: usize = 696;
pub const UnitDataNode__version__OFFSET: usize = 720;
pub const UnitDataNode__requireWaterRadius__OFFSET: usize = 724;

#[repr(C)]
pub struct CUnitSkinDB {
    pub _opaque: [u8; 680],
}
pub const CUnitSkinDB__SIZE: usize = 680;
pub const CUnitSkinDB__NAME: &str = "CUnitSkinDB";
pub const CUnitSkinDB__mUnitDB__OFFSET: usize = 672;

#[repr(C)]
pub struct UnitPrio {
    pub _opaque: [u8; 32],
}
pub const UnitPrio__SIZE: usize = 32;
pub const UnitPrio__NAME: &str = "UnitPrio";
pub const UnitPrio__unit__OFFSET: usize = 0;
pub const UnitPrio__count__OFFSET: usize = 8;
pub const UnitPrio__total__OFFSET: usize = 12;
pub const UnitPrio__prio__OFFSET: usize = 16;
pub const UnitPrio__ot__OFFSET: usize = 20;
pub const UnitPrio__subgroup__OFFSET: usize = 24;
pub const UnitPrio__autoQueue__OFFSET: usize = 25;

#[repr(C)]
pub struct UnitPrioMoveReq {
    pub _opaque: [u8; 40],
}
pub const UnitPrioMoveReq__SIZE: usize = 40;
pub const UnitPrioMoveReq__NAME: &str = "UnitPrioMoveReq";
pub const UnitPrioMoveReq__moveReq__OFFSET: usize = 32;

#[repr(C)]
pub struct UnitPrioMoveReq2 {
    pub _opaque: [u8; 48],
}
pub const UnitPrioMoveReq2__SIZE: usize = 48;
pub const UnitPrioMoveReq2__NAME: &str = "UnitPrioMoveReq2";
pub const UnitPrioMoveReq2__pointReq__OFFSET: usize = 32;
pub const UnitPrioMoveReq2__targReq__OFFSET: usize = 40;

#[repr(C)]
pub struct UnitPrioMoveReqSubPt {
    pub _opaque: [u8; 48],
}
pub const UnitPrioMoveReqSubPt__SIZE: usize = 48;
pub const UnitPrioMoveReqSubPt__NAME: &str = "UnitPrioMoveReqSubPt";
pub const UnitPrioMoveReqSubPt__subPtOrder__OFFSET: usize = 40;

#[repr(C)]
pub struct WorldData {
    pub _opaque: [u8; 496],
}
pub const WorldData__SIZE: usize = 496;
pub const WorldData__NAME: &str = "WorldData";
pub const WorldData__terrain__OFFSET: usize = 0;
pub const WorldData__doodads__OFFSET: usize = 8;
pub const WorldData__doodadCustomData__OFFSET: usize = 16;
pub const WorldData__walkables__OFFSET: usize = 24;
pub const WorldData__splatEmitter__OFFSET: usize = 32;
pub const WorldData__spawn__OFFSET: usize = 72;
pub const WorldData__tags__OFFSET: usize = 80;
pub const WorldData__bolts__OFFSET: usize = 88;
pub const WorldData__mapFile__OFFSET: usize = 96;
pub const WorldData__mapVersion__OFFSET: usize = 356;
pub const WorldData__mapFlags__OFFSET: usize = 360;
pub const WorldData__deprecatedFileLoadCallback__OFFSET: usize = 368;
pub const WorldData__mapSignature__OFFSET: usize = 376;
pub const WorldData__mapName__OFFSET: usize = 380;

#[repr(C)]
pub struct MapInfoData {
    pub _opaque: [u8; 14536],
}
pub const MapInfoData__SIZE: usize = 14536;
pub const MapInfoData__NAME: &str = "MapInfoData";
pub const MapInfoData__mapProps__OFFSET: usize = 0;
pub const MapInfoData__mapPlayers__OFFSET: usize = 14480;
pub const MapInfoData__mapForces__OFFSET: usize = 14504;
pub const MapInfoData__mapAuth__OFFSET: usize = 14528;

#[repr(C)]
pub struct MapInfo {
    pub _opaque: [u8; 14624],
}
pub const MapInfo__SIZE: usize = 14624;
pub const MapInfo__NAME: &str = "MapInfo";
pub const MapInfo__mapProps__OFFSET: usize = 0;
pub const MapInfo__players__OFFSET: usize = 14480;
pub const MapInfo__forces__OFFSET: usize = 14504;
pub const MapInfo__upgrades__OFFSET: usize = 14528;
pub const MapInfo__techtree__OFFSET: usize = 14552;
pub const MapInfo__randomGroups__OFFSET: usize = 14576;
pub const MapInfo__itemTables__OFFSET: usize = 14600;

#[repr(C)]
pub struct CSelectionRect {
    pub _opaque: [u8; 56],
}
pub const CSelectionRect__SIZE: usize = 56;
pub const CSelectionRect__NAME: &str = "CSelectionRect";
pub const CSelectionRect__mTerrain__OFFSET: usize = 0;
pub const CSelectionRect__mBounds__OFFSET: usize = 8;
pub const CSelectionRect__mVisible__OFFSET: usize = 24;
pub const CSelectionRect__mColor__OFFSET: usize = 28;
pub const CSelectionRect__mModels__OFFSET: usize = 32;

#[repr(C)]
pub struct MapInfoSaveLoadParams {
    pub _opaque: [u8; 24],
}
pub const MapInfoSaveLoadParams__SIZE: usize = 24;
pub const MapInfoSaveLoadParams__NAME: &str = "MapInfoSaveLoadParams";
pub const MapInfoSaveLoadParams__outInfo__OFFSET: usize = 0;
pub const MapInfoSaveLoadParams__inInfo__OFFSET: usize = 8;
pub const MapInfoSaveLoadParams__loadLevel__OFFSET: usize = 16;
pub const MapInfoSaveLoadParams__expansion__OFFSET: usize = 20;
pub const MapInfoSaveLoadParams__silent__OFFSET: usize = 21;

#[repr(C)]
pub struct CTriggerDB {
    pub _opaque: [u8; 88],
}
pub const CTriggerDB__SIZE: usize = 88;
pub const CTriggerDB__NAME: &str = "CTriggerDB";
pub const CTriggerDB__mAIOnly__OFFSET: usize = 8;
pub const CTriggerDB__mCategories__OFFSET: usize = 16;
pub const CTriggerDB__mTypes__OFFSET: usize = 40;
pub const CTriggerDB__mFunctions__OFFSET: usize = 64;

#[repr(C)]
pub struct CWorldObjectUndoable {
    pub _opaque: [u8; 968],
}
pub const CWorldObjectUndoable__SIZE: usize = 968;
pub const CWorldObjectUndoable__NAME: &str = "CWorldObjectUndoable";
pub const CWorldObjectUndoable__mObjects__OFFSET: usize = 8;
pub const CWorldObjectUndoable__mUndoType__OFFSET: usize = 16;
pub const CWorldObjectUndoable__mOldObject__OFFSET: usize = 24;
pub const CWorldObjectUndoable__mNewObject__OFFSET: usize = 496;

#[repr(C)]
pub struct LuaScriptInstance_t {
    pub _opaque: [u8; 56],
}
pub const LuaScriptInstance_t__SIZE: usize = 56;
pub const LuaScriptInstance_t__NAME: &str = "LuaScriptInstance_t";
pub const LuaScriptInstance_t__vm__OFFSET: usize = 8;
pub const LuaScriptInstance_t__engine__OFFSET: usize = 24;
pub const LuaScriptInstance_t__id__OFFSET: usize = 32;
pub const LuaScriptInstance_t__syncing__OFFSET: usize = 36;
pub const LuaScriptInstance_t__sleeping__OFFSET: usize = 37;
pub const LuaScriptInstance_t__sleepTime__OFFSET: usize = 40;
pub const LuaScriptInstance_t__sleepData__OFFSET: usize = 48;

#[repr(C)]
pub struct LuaScriptEngine_t {
    pub _opaque: [u8; 16],
}
pub const LuaScriptEngine_t__SIZE: usize = 16;
pub const LuaScriptEngine_t__NAME: &str = "LuaScriptEngine_t";
pub const LuaScriptEngine_t__impl___OFFSET: usize = 8;

#[repr(C)]
pub struct Jass2ScriptingEngine_t {
    pub _opaque: [u8; 16],
}
pub const Jass2ScriptingEngine_t__SIZE: usize = 16;
pub const Jass2ScriptingEngine_t__NAME: &str = "Jass2ScriptingEngine_t";
pub const Jass2ScriptingEngine_t__callbacks_ctx__OFFSET: usize = 8;

#[repr(C)]
pub struct LuaScriptEngineImpl {
    pub _opaque: [u8; 168],
}
pub const LuaScriptEngineImpl__SIZE: usize = 168;
pub const LuaScriptEngineImpl__NAME: &str = "LuaScriptEngineImpl";
pub const LuaScriptEngineImpl__vmShared__OFFSET: usize = 0;
pub const LuaScriptEngineImpl__nativeFuncs__OFFSET: usize = 8;
pub const LuaScriptEngineImpl__scripts__OFFSET: usize = 32;
pub const LuaScriptEngineImpl__instances__OFFSET: usize = 72;
pub const LuaScriptEngineImpl__vms__OFFSET: usize = 112;
pub const LuaScriptEngineImpl__contexts__OFFSET: usize = 144;

#[repr(C)]
pub struct Jass2ScriptInstance_t {
    pub _opaque: [u8; 48],
}
pub const Jass2ScriptInstance_t__SIZE: usize = 48;
pub const Jass2ScriptInstance_t__NAME: &str = "Jass2ScriptInstance_t";
pub const Jass2ScriptInstance_t__m_hinst__OFFSET: usize = 8;
pub const Jass2ScriptInstance_t__gchg_ctx__OFFSET: usize = 16;
pub const Jass2ScriptInstance_t__hcb_ctx__OFFSET: usize = 32;

#[repr(C)]
pub struct Jass2Script_t {
    pub _opaque: [u8; 16],
}
pub const Jass2Script_t__SIZE: usize = 16;
pub const Jass2Script_t__NAME: &str = "Jass2Script_t";
pub const Jass2Script_t__m_hscript__OFFSET: usize = 8;
pub const Jass2Script_t__debug_mode__OFFSET: usize = 12;

#[repr(C)]
pub struct LuaScript_t {
    pub _opaque: [u8; 40],
}
pub const LuaScript_t__SIZE: usize = 40;
pub const LuaScript_t__NAME: &str = "LuaScript_t";
pub const LuaScript_t__vm__OFFSET: usize = 8;
pub const LuaScript_t__engine__OFFSET: usize = 24;
pub const LuaScript_t__id__OFFSET: usize = 32;

#[repr(C)]
pub struct Jass2LuaScriptingEngine_t {
    pub _opaque: [u8; 24],
}
pub const Jass2LuaScriptingEngine_t__SIZE: usize = 24;
pub const Jass2LuaScriptingEngine_t__NAME: &str = "Jass2LuaScriptingEngine_t";
pub const Jass2LuaScriptingEngine_t__lua_engine__OFFSET: usize = 8;
pub const Jass2LuaScriptingEngine_t__jass2_engine__OFFSET: usize = 16;

#[repr(C)]
pub struct GameAdvertisement {
    pub _opaque: [u8; 144],
}
pub const GameAdvertisement__SIZE: usize = 144;
pub const GameAdvertisement__NAME: &str = "GameAdvertisement";
pub const GameAdvertisement__m_name__OFFSET: usize = 8;
pub const GameAdvertisement__m_programId__OFFSET: usize = 48;
pub const GameAdvertisement__m_versionId__OFFSET: usize = 52;
pub const GameAdvertisement__m_private__OFFSET: usize = 56;
pub const GameAdvertisement__m_attributes__OFFSET: usize = 64;

#[repr(C)]
pub struct GameInfoImpl {
    pub _opaque: [u8; 64],
}
pub const GameInfoImpl__SIZE: usize = 64;
pub const GameInfoImpl__NAME: &str = "GameInfoImpl";
pub const GameInfoImpl__m_gameId__OFFSET: usize = 8;
pub const GameInfoImpl__m_gatewayId__OFFSET: usize = 16;
pub const GameInfoImpl__m_serverPublicKey__OFFSET: usize = 24;

#[repr(C)]
pub struct GameSearchInfo {
    pub _opaque: [u8; 96],
}
pub const GameSearchInfo__SIZE: usize = 96;
pub const GameSearchInfo__NAME: &str = "GameSearchInfo";
pub const GameSearchInfo__m_id__OFFSET: usize = 8;
pub const GameSearchInfo__m_version__OFFSET: usize = 12;
pub const GameSearchInfo__m_name__OFFSET: usize = 16;
pub const GameSearchInfo__m_password__OFFSET: usize = 56;

#[repr(C)]
pub struct PlayerHeroInfo__1 {
    pub _opaque: [u8; 52],
}
pub const PlayerHeroInfo__1__SIZE: usize = 52;
pub const PlayerHeroInfo__1__NAME: &str = "PlayerHeroInfo";
pub const PlayerHeroInfo__1__level__OFFSET: usize = 0;
pub const PlayerHeroInfo__1__number_of_deaths__OFFSET: usize = 4;
pub const PlayerHeroInfo__1__experience__OFFSET: usize = 8;
pub const PlayerHeroInfo__1__time_alive_ms__OFFSET: usize = 12;
pub const PlayerHeroInfo__1__xp_per_minute__OFFSET: usize = 16;
pub const PlayerHeroInfo__1__play_time__OFFSET: usize = 20;
pub const PlayerHeroInfo__1__total_kills__OFFSET: usize = 24;
pub const PlayerHeroInfo__1__hero_kills__OFFSET: usize = 28;
pub const PlayerHeroInfo__1__building_kills__OFFSET: usize = 32;
pub const PlayerHeroInfo__1__damage_dealt__OFFSET: usize = 36;
pub const PlayerHeroInfo__1__healing_done__OFFSET: usize = 40;
pub const PlayerHeroInfo__1__damage_received__OFFSET: usize = 44;
pub const PlayerHeroInfo__1__pick_order__OFFSET: usize = 48;

#[repr(C)]
pub struct CJassCompile {
    pub _opaque: [u8; 368],
}
pub const CJassCompile__SIZE: usize = 368;
pub const CJassCompile__NAME: &str = "CJassCompile";
pub const CJassCompile__m_scanner__OFFSET: usize = 0;
pub const CJassCompile__m_bufferState__OFFSET: usize = 8;
pub const CJassCompile__m_nextInput__OFFSET: usize = 16;
pub const CJassCompile__m_errors__OFFSET: usize = 24;
pub const CJassCompile__m_errdata__OFFSET: usize = 32;
pub const CJassCompile__m_errstr__OFFSET: usize = 40;
pub const CJassCompile__m_code__OFFSET: usize = 48;
pub const CJassCompile__m_filename__OFFSET: usize = 56;
pub const CJassCompile__m_strings__OFFSET: usize = 64;
pub const CJassCompile__m_symbols__OFFSET: usize = 72;
pub const CJassCompile__m_compileType__OFFSET: usize = 224;
pub const CJassCompile__m_ast__OFFSET: usize = 232;
pub const CJassCompile__m_constFunc__OFFSET: usize = 240;
pub const CJassCompile__m_syntaxErrors__OFFSET: usize = 244;
pub const CJassCompile__m_ifThen__OFFSET: usize = 248;
pub const CJassCompile__m_loops__OFFSET: usize = 252;
pub const CJassCompile__m_ifWasLatest__OFFSET: usize = 256;
pub const CJassCompile__m_doEof__OFFSET: usize = 257;
pub const CJassCompile__m_inFunction__OFFSET: usize = 258;
pub const CJassCompile__m_inTakes__OFFSET: usize = 259;
pub const CJassCompile__m_hadError__OFFSET: usize = 260;
pub const CJassCompile__m_currFuncRetType__OFFSET: usize = 264;
pub const CJassCompile__m_bPrettyPrintMode__OFFSET: usize = 272;
pub const CJassCompile__m_fileNodes__OFFSET: usize = 280;
pub const CJassCompile__m_lastFileNode__OFFSET: usize = 304;
pub const CJassCompile__m_current_callables__OFFSET: usize = 312;
pub const CJassCompile__m_callables__OFFSET: usize = 336;

#[repr(C)]
pub struct JassFrame {
    pub _opaque: [u8; 368],
}
pub const JassFrame__SIZE: usize = 368;
pub const JassFrame__NAME: &str = "JassFrame";
pub const JassFrame__m_stack__OFFSET: usize = 16;
pub const JassFrame__m_top__OFFSET: usize = 280;
pub const JassFrame__m_data__OFFSET: usize = 288;
pub const JassFrame__m_calledFromNative__OFFSET: usize = 360;

#[repr(C)]
pub struct JassHandle {
    pub _opaque: [u8; 72],
}
pub const JassHandle__SIZE: usize = 72;
pub const JassHandle__NAME: &str = "JassHandle";
pub const JassHandle__m_parent__OFFSET: usize = 48;
pub const JassHandle__m_sibling__OFFSET: usize = 56;
pub const JassHandle__m_child__OFFSET: usize = 64;

#[repr(C)]
pub struct JassScript {
    pub _opaque: [u8; 48],
}
pub const JassScript__SIZE: usize = 48;
pub const JassScript__NAME: &str = "JassScript";
pub const JassScript__m_code__OFFSET: usize = 0;
pub const JassScript__m_codeSize__OFFSET: usize = 8;
pub const JassScript__m_strings__OFFSET: usize = 16;
pub const JassScript__m_compileType__OFFSET: usize = 24;
pub const JassScript__m_user_data__OFFSET: usize = 32;

#[repr(C)]
pub struct JassThreadLocal {
    pub _opaque: [u8; 288],
}
pub const JassThreadLocal__SIZE: usize = 288;
pub const JassThreadLocal__NAME: &str = "JassThreadLocal";
pub const JassThreadLocal__activeStack__OFFSET: usize = 8;
pub const JassThreadLocal__activeStackIndex__OFFSET: usize = 32;
pub const JassThreadLocal__nativeFuncs__OFFSET: usize = 40;
pub const JassThreadLocal__scriptTable__OFFSET: usize = 112;
pub const JassThreadLocal__unusedScriptHandles__OFFSET: usize = 136;
pub const JassThreadLocal__script2HandleMap__OFFSET: usize = 160;
pub const JassThreadLocal__instanceTable__OFFSET: usize = 232;
pub const JassThreadLocal__unusedInstanceHandles__OFFSET: usize = 256;
pub const JassThreadLocal__trace__OFFSET: usize = 280;
pub const JassThreadLocal__saveFileBuildId__OFFSET: usize = 284;

#[repr(C)]
pub struct JassString {
    pub _opaque: [u8; 32],
}
pub const JassString__SIZE: usize = 32;
pub const JassString__NAME: &str = "JassString";
pub const JassString__string__OFFSET: usize = 0;
pub const JassString__refCount__OFFSET: usize = 24;

#[repr(C)]
pub struct JassInstance {
    pub _opaque: [u8; 18704],
}
pub const JassInstance__SIZE: usize = 18704;
pub const JassInstance__NAME: &str = "JassInstance";
pub const JassInstance__m_duplicate__OFFSET: usize = 0;
pub const JassInstance__m_dupCount__OFFSET: usize = 4;
pub const JassInstance__m_ip__OFFSET: usize = 8;
pub const JassInstance__m_ipStack__OFFSET: usize = 16;
pub const JassInstance__m_sleep__OFFSET: usize = 40;
pub const JassInstance__m_sleepData__OFFSET: usize = 48;
pub const JassInstance__m_sync__OFFSET: usize = 56;
pub const JassInstance__m_sleepTime__OFFSET: usize = 60;
pub const JassInstance__m_execLimit__OFFSET: usize = 64;
pub const JassInstance__m_compileType__OFFSET: usize = 68;
pub const JassInstance__m_debug_mode__OFFSET: usize = 72;
pub const JassInstance__m_regs__OFFSET: usize = 80;
pub const JassInstance__m_instanceHandle__OFFSET: usize = 18520;
pub const JassInstance__m_masterInstanceHandle__OFFSET: usize = 18524;
pub const JassInstance__m_script__OFFSET: usize = 18528;
pub const JassInstance__m_userdata__OFFSET: usize = 18536;
pub const JassInstance__m_global__OFFSET: usize = 18552;
pub const JassInstance__m_frames__OFFSET: usize = 18560;
pub const JassInstance__m_handles__OFFSET: usize = 18584;
pub const JassInstance__m_labels__OFFSET: usize = 18592;
pub const JassInstance__m_stringTable__OFFSET: usize = 18600;
pub const JassInstance__m_unusedStringHandles__OFFSET: usize = 18608;
pub const JassInstance__m_string2HandleMap__OFFSET: usize = 18616;
pub const JassInstance__m_initFuncs__OFFSET: usize = 18624;
pub const JassInstance__m_funcs__OFFSET: usize = 18632;
pub const JassInstance__m_funcTable__OFFSET: usize = 18640;
pub const JassInstance__m_unusedFuncHandles__OFFSET: usize = 18648;
pub const JassInstance__m_func2HandleMap__OFFSET: usize = 18656;
pub const JassInstance__m_func2NameMap__OFFSET: usize = 18664;
pub const JassInstance__m_globalSetFunc__OFFSET: usize = 18672;
pub const JassInstance__m_globalSetFuncOpaqueData__OFFSET: usize = 18680;
pub const JassInstance__m_handleFunc__OFFSET: usize = 18688;
pub const JassInstance__m_handleFuncOpaqueData__OFFSET: usize = 18696;

#[repr(C)]
pub struct JassArray {
    pub _opaque: [u8; 32],
}
pub const JassArray__SIZE: usize = 32;
pub const JassArray__NAME: &str = "JassArray";

#[repr(C)]
pub struct RegionPrec {
    pub _opaque: [u8; 24],
}
pub const RegionPrec__SIZE: usize = 24;
pub const RegionPrec__NAME: &str = "RegionPrec";
pub const RegionPrec__endpt_a_prec__OFFSET: usize = 0;
pub const RegionPrec__endpt_b_prec__OFFSET: usize = 12;

#[repr(C)]
pub struct RegionPrec__1 {
    pub _opaque: [u8; 32],
}
pub const RegionPrec__1__SIZE: usize = 32;
pub const RegionPrec__1__NAME: &str = "RegionPrec";
pub const RegionPrec__1__endpt_a_prec__OFFSET: usize = 0;
pub const RegionPrec__1__endpt_b_prec__OFFSET: usize = 16;

#[repr(C)]
pub struct CUnitAI__5 {
    pub _opaque: [u8; 152],
}
pub const CUnitAI__5__SIZE: usize = 152;
pub const CUnitAI__5__NAME: &str = "CUnitAI";
pub const CUnitAI__5__s_CUnitAI_pool__OFFSET: usize = 0;
pub const CUnitAI__5__m_captain__OFFSET: usize = 88;
pub const CUnitAI__5__anticheat_dummy_array_26__OFFSET: usize = 100;
pub const CUnitAI__5__m_mode__OFFSET: usize = 108;
pub const CUnitAI__5__m_town__OFFSET: usize = 112;
pub const CUnitAI__5__m_flags__OFFSET: usize = 124;
pub const CUnitAI__5__m_post__OFFSET: usize = 128;
pub const CUnitAI__5__m_data__OFFSET: usize = 140;
pub const CUnitAI__5__m_thinker__OFFSET: usize = 144;

