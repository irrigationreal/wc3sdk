// Generated types. Do not edit by hand.
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub mod _shared_ptr_base_bnl {
    #[repr(C)]
    pub struct TimerService_0_ {
        pub _opaque: [u8; 8],
    }
    pub const TimerService_0___SIZE: usize = 8;
    pub const TimerService_0___NAME: &str = "blz::_shared_ptr_base<bnl::TimerService,0>";
    pub const TimerService_0___m_pointer__OFFSET: usize = 0;

}

pub mod chained_hash_node_blz {
    pub mod pair_int_const__CMultiplayerCallbacks {
        #[repr(C)]
        pub struct GameAd___ {
            pub _opaque: [u8; 768],
        }
        pub const GameAd_____SIZE: usize = 768;
        pub const GameAd_____NAME: &str = "blz::chained_hash_node<blz::pair<int const ,CMultiplayerCallbacks::GameAd> >";
        pub const GameAd_____next__OFFSET: usize = 0;
        pub const GameAd_____val__OFFSET: usize = 8;

    }

    pub mod pair_unsigned_char_const__CMultiplayerManager {
        #[repr(C)]
        pub struct PlayerData___ {
            pub _opaque: [u8; 248],
        }
        pub const PlayerData_____SIZE: usize = 248;
        pub const PlayerData_____NAME: &str = "blz::chained_hash_node<blz::pair<unsigned char const ,CMultiplayerManager::PlayerData> >";
        pub const PlayerData_____next__OFFSET: usize = 0;
        pub const PlayerData_____val__OFFSET: usize = 8;

    }

    pub mod pair_unsigned_int_const__Net {
        pub mod NetProviderBNET {
            #[repr(C)]
            pub struct GameData___ {
                pub _opaque: [u8; 72],
            }
            pub const GameData_____SIZE: usize = 72;
            pub const GameData_____NAME: &str = "blz::chained_hash_node<blz::pair<unsigned int const ,Net::NetProviderBNET::GameData> >";
            pub const GameData_____next__OFFSET: usize = 0;
            pub const GameData_____val__OFFSET: usize = 8;

        }

    }

}

pub mod chained_hash_table_blz {
    pub mod unordered_map_traits_int_CMultiplayerCallbacks {
        pub mod GameAd__blz {
            pub mod hash_int__blz {
                pub mod equal_to_int__blz {
                    pub mod allocator_blz {
                        pub mod pair_int_const__CMultiplayerCallbacks {
                            #[repr(C)]
                            pub struct GameAd_____ {
                                pub _opaque: [u8; 32],
                            }
                            pub const GameAd_______SIZE: usize = 32;
                            pub const GameAd_______NAME: &str = "blz::chained_hash_table<blz::unordered_map_traits<int,CMultiplayerCallbacks::GameAd>,blz::hash<int>,blz::equal_to<int>,blz::allocator<blz::pair<int const ,CMultiplayerCallbacks::GameAd> > >";
                            pub const GameAd_______m_bucket_count__OFFSET: usize = 0;
                            pub const GameAd_______m_buckets__OFFSET: usize = 8;
                            pub const GameAd_______m_entry_count__OFFSET: usize = 16;
                            pub const GameAd_______m_max_load_factor__OFFSET: usize = 24;

                        }

                    }

                }

            }

        }

    }

    pub mod unordered_map_traits_unsigned_char_CMultiplayerManager {
        pub mod PlayerData__blz {
            pub mod hash_unsigned_char__blz {
                pub mod equal_to_unsigned_char__blz {
                    pub mod allocator_blz {
                        pub mod pair_unsigned_char_const__CMultiplayerManager {
                            #[repr(C)]
                            pub struct PlayerData_____ {
                                pub _opaque: [u8; 32],
                            }
                            pub const PlayerData_______SIZE: usize = 32;
                            pub const PlayerData_______NAME: &str = "blz::chained_hash_table<blz::unordered_map_traits<unsigned char,CMultiplayerManager::PlayerData>,blz::hash<unsigned char>,blz::equal_to<unsigned char>,blz::allocator<blz::pair<unsigned char const ,CMultiplayerManager::PlayerData> > >";
                            pub const PlayerData_______m_bucket_count__OFFSET: usize = 0;
                            pub const PlayerData_______m_buckets__OFFSET: usize = 8;
                            pub const PlayerData_______m_entry_count__OFFSET: usize = 16;
                            pub const PlayerData_______m_max_load_factor__OFFSET: usize = 24;

                        }

                    }

                }

            }

        }

    }

    pub mod unordered_map_traits_unsigned_int_Net {
        pub mod NetProviderBNET {
            pub mod GameData__blz {
                pub mod hash_unsigned_int__blz {
                    pub mod equal_to_unsigned_int__blz {
                        pub mod allocator_blz {
                            pub mod pair_unsigned_int_const__Net {
                                pub mod NetProviderBNET {
                                    #[repr(C)]
                                    pub struct GameData_____ {
                                        pub _opaque: [u8; 32],
                                    }
                                    pub const GameData_______SIZE: usize = 32;
                                    pub const GameData_______NAME: &str = "blz::chained_hash_table<blz::unordered_map_traits<unsigned int,Net::NetProviderBNET::GameData>,blz::hash<unsigned int>,blz::equal_to<unsigned int>,blz::allocator<blz::pair<unsigned int const ,Net::NetProviderBNET::GameData> > >";
                                    pub const GameData_______m_bucket_count__OFFSET: usize = 0;
                                    pub const GameData_______m_buckets__OFFSET: usize = 8;
                                    pub const GameData_______m_entry_count__OFFSET: usize = 16;
                                    pub const GameData_______m_max_load_factor__OFFSET: usize = 24;

                                }

                            }

                        }

                    }

                }

            }

        }

    }

}

pub mod chained_hash_table_const_iterator_blz {
    pub mod pair_int_const__CMultiplayerCallbacks {
        #[repr(C)]
        pub struct GameAd___ {
            pub _opaque: [u8; 24],
        }
        pub const GameAd_____SIZE: usize = 24;
        pub const GameAd_____NAME: &str = "blz::chained_hash_table_const_iterator<blz::pair<int const ,CMultiplayerCallbacks::GameAd> >";
        pub const GameAd_____node__OFFSET: usize = 0;
        pub const GameAd_____bucket__OFFSET: usize = 8;
        pub const GameAd_____end__OFFSET: usize = 16;

    }

    pub mod pair_unsigned_char_const__CMultiplayerManager {
        #[repr(C)]
        pub struct PlayerData___ {
            pub _opaque: [u8; 24],
        }
        pub const PlayerData_____SIZE: usize = 24;
        pub const PlayerData_____NAME: &str = "blz::chained_hash_table_const_iterator<blz::pair<unsigned char const ,CMultiplayerManager::PlayerData> >";
        pub const PlayerData_____node__OFFSET: usize = 0;
        pub const PlayerData_____bucket__OFFSET: usize = 8;
        pub const PlayerData_____end__OFFSET: usize = 16;

    }

    pub mod pair_unsigned_int_const__Net {
        pub mod NetProviderBNET {
            #[repr(C)]
            pub struct GameData___ {
                pub _opaque: [u8; 24],
            }
            pub const GameData_____SIZE: usize = 24;
            pub const GameData_____NAME: &str = "blz::chained_hash_table_const_iterator<blz::pair<unsigned int const ,Net::NetProviderBNET::GameData> >";
            pub const GameData_____node__OFFSET: usize = 0;
            pub const GameData_____bucket__OFFSET: usize = 8;
            pub const GameData_____end__OFFSET: usize = 16;

        }

    }

}

pub mod chained_hash_table_iterator_blz {
    pub mod pair_int_const__CMultiplayerCallbacks {
        #[repr(C)]
        pub struct GameAd___ {
            pub _opaque: [u8; 24],
        }
        pub const GameAd_____SIZE: usize = 24;
        pub const GameAd_____NAME: &str = "blz::chained_hash_table_iterator<blz::pair<int const ,CMultiplayerCallbacks::GameAd> >";

    }

    pub mod pair_unsigned_char_const__CMultiplayerManager {
        #[repr(C)]
        pub struct PlayerData___ {
            pub _opaque: [u8; 24],
        }
        pub const PlayerData_____SIZE: usize = 24;
        pub const PlayerData_____NAME: &str = "blz::chained_hash_table_iterator<blz::pair<unsigned char const ,CMultiplayerManager::PlayerData> >";

    }

    pub mod pair_unsigned_int_const__Net {
        pub mod NetProviderBNET {
            #[repr(C)]
            pub struct GameData___ {
                pub _opaque: [u8; 24],
            }
            pub const GameData_____SIZE: usize = 24;
            pub const GameData_____NAME: &str = "blz::chained_hash_table_iterator<blz::pair<unsigned int const ,Net::NetProviderBNET::GameData> >";

        }

    }

}

pub mod enable_shared_from_this_bnl {
    #[repr(C)]
    pub struct TimerService_ {
        pub _opaque: [u8; 16],
    }
    pub const TimerService___SIZE: usize = 16;
    pub const TimerService___NAME: &str = "blz::enable_shared_from_this<bnl::TimerService>";
    pub const TimerService___m_weak_this__OFFSET: usize = 0;

}

pub mod map_unsigned_int_CProductionPanelUnit {
    pub mod UnitData_blz {
        pub mod less_unsigned_int__blz {
            pub mod allocator_blz {
                pub mod pair_unsigned_int_const__CProductionPanelUnit {
                    #[repr(C)]
                    pub struct UnitData_____ {
                        pub _opaque: [u8; 32],
                    }
                    pub const UnitData_______SIZE: usize = 32;
                    pub const UnitData_______NAME: &str = "blz::map<unsigned int,CProductionPanelUnit::UnitData,blz::less<unsigned int>,blz::allocator<blz::pair<unsigned int const ,CProductionPanelUnit::UnitData> > >";

                }

            }

        }

    }

}

pub mod move_iterator_CAchievements {
    #[repr(C)]
    pub struct MapRequirement___ {
        pub _opaque: [u8; 8],
    }
    pub const MapRequirement_____SIZE: usize = 8;
    pub const MapRequirement_____NAME: &str = "blz::move_iterator<CAchievements::MapRequirement *>";
    pub const MapRequirement_____current__OFFSET: usize = 0;

}

pub mod pair_blz {
    pub mod chained_hash_table_iterator_blz {
        pub mod pair_int_const__CMultiplayerCallbacks {
            #[repr(C)]
            pub struct GameAd____bool_ {
                pub _opaque: [u8; 32],
            }
            pub const GameAd____bool___SIZE: usize = 32;
            pub const GameAd____bool___NAME: &str = "blz::pair<blz::chained_hash_table_iterator<blz::pair<int const ,CMultiplayerCallbacks::GameAd> >,bool>";
            pub const GameAd____bool___first__OFFSET: usize = 0;
            pub const GameAd____bool___second__OFFSET: usize = 24;

        }

        pub mod pair_unsigned_char_const__CMultiplayerManager {
            pub mod PlayerData____blz {
                pub mod chained_hash_table_iterator_blz {
                    pub mod pair_unsigned_char_const__CMultiplayerManager {
                        #[repr(C)]
                        pub struct PlayerData_____ {
                            pub _opaque: [u8; 48],
                        }
                        pub const PlayerData_______SIZE: usize = 48;
                        pub const PlayerData_______NAME: &str = "blz::pair<blz::chained_hash_table_iterator<blz::pair<unsigned char const ,CMultiplayerManager::PlayerData> >,blz::chained_hash_table_iterator<blz::pair<unsigned char const ,CMultiplayerManager::PlayerData> > >";
                        pub const PlayerData_______first__OFFSET: usize = 0;
                        pub const PlayerData_______second__OFFSET: usize = 24;

                    }

                }

            }

        }

    }

    pub mod rb_tree_iterator_blz {
        pub mod pair_unsigned_int_const__CProductionPanelUnit {
            #[repr(C)]
            pub struct UnitData____bool_ {
                pub _opaque: [u8; 16],
            }
            pub const UnitData____bool___SIZE: usize = 16;
            pub const UnitData____bool___NAME: &str = "blz::pair<blz::rb_tree_iterator<blz::pair<unsigned int const ,CProductionPanelUnit::UnitData> >,bool>";
            pub const UnitData____bool___first__OFFSET: usize = 0;
            pub const UnitData____bool___second__OFFSET: usize = 8;

        }

    }

}

pub mod pair_int_const__CMultiplayerCallbacks {
    #[repr(C)]
    pub struct GameAd_ {
        pub _opaque: [u8; 760],
    }
    pub const GameAd___SIZE: usize = 760;
    pub const GameAd___NAME: &str = "blz::pair<int const ,CMultiplayerCallbacks::GameAd>";
    pub const GameAd___first__OFFSET: usize = 0;
    pub const GameAd___second__OFFSET: usize = 8;

}

pub mod pair_int_CMultiplayerCallbacks {
    #[repr(C)]
    pub struct GameAd_ {
        pub _opaque: [u8; 760],
    }
    pub const GameAd___SIZE: usize = 760;
    pub const GameAd___NAME: &str = "blz::pair<int,CMultiplayerCallbacks::GameAd>";
    pub const GameAd___first__OFFSET: usize = 0;
    pub const GameAd___second__OFFSET: usize = 8;

}

pub mod pair_unsigned_char_const__CMultiplayerManager {
    #[repr(C)]
    pub struct PlayerData_ {
        pub _opaque: [u8; 240],
    }
    pub const PlayerData___SIZE: usize = 240;
    pub const PlayerData___NAME: &str = "blz::pair<unsigned char const ,CMultiplayerManager::PlayerData>";
    pub const PlayerData___first__OFFSET: usize = 0;
    pub const PlayerData___second__OFFSET: usize = 8;

}

pub mod pair_unsigned_int_const__CProductionPanelUnit {
    #[repr(C)]
    pub struct UnitData_ {
        pub _opaque: [u8; 40],
    }
    pub const UnitData___SIZE: usize = 40;
    pub const UnitData___NAME: &str = "blz::pair<unsigned int const ,CProductionPanelUnit::UnitData>";
    pub const UnitData___first__OFFSET: usize = 0;
    pub const UnitData___second__OFFSET: usize = 8;

}

pub mod pair_unsigned_int_const__Net {
    pub mod NetProviderBNET {
        #[repr(C)]
        pub struct GameData_ {
            pub _opaque: [u8; 64],
        }
        pub const GameData___SIZE: usize = 64;
        pub const GameData___NAME: &str = "blz::pair<unsigned int const ,Net::NetProviderBNET::GameData>";
        pub const GameData___first__OFFSET: usize = 0;
        pub const GameData___second__OFFSET: usize = 8;

    }

}

pub mod rb_node_blz {
    pub mod pair_unsigned_int_const__CProductionPanelUnit {
        #[repr(C)]
        pub struct UnitData___ {
            pub _opaque: [u8; 64],
        }
        pub const UnitData_____SIZE: usize = 64;
        pub const UnitData_____NAME: &str = "blz::rb_node<blz::pair<unsigned int const ,CProductionPanelUnit::UnitData> >";
        pub const UnitData_____val__OFFSET: usize = 24;

    }

}

pub mod rb_tree_blz {
    pub mod rb_map_traits_unsigned_int_CProductionPanelUnit {
        pub mod UnitData__blz {
            pub mod less_unsigned_int__blz {
                pub mod allocator_blz {
                    pub mod pair_unsigned_int_const__CProductionPanelUnit {
                        #[repr(C)]
                        pub struct UnitData_____ {
                            pub _opaque: [u8; 32],
                        }
                        pub const UnitData_______SIZE: usize = 32;
                        pub const UnitData_______NAME: &str = "blz::rb_tree<blz::rb_map_traits<unsigned int,CProductionPanelUnit::UnitData>,blz::less<unsigned int>,blz::allocator<blz::pair<unsigned int const ,CProductionPanelUnit::UnitData> > >";
                        pub const UnitData_______m_header__OFFSET: usize = 0;
                        pub const UnitData_______m_size__OFFSET: usize = 24;

                        pub mod UnitData_______mod {
                            #[repr(C)]
                            pub struct insert_location {
                                pub _opaque: [u8; 16],
                            }
                            pub const insert_location__SIZE: usize = 16;
                            pub const insert_location__NAME: &str = "blz::rb_tree<blz::rb_map_traits<unsigned int,CProductionPanelUnit::UnitData>,blz::less<unsigned int>,blz::allocator<blz::pair<unsigned int const ,CProductionPanelUnit::UnitData> > >::insert_location";
                            pub const insert_location__node__OFFSET: usize = 0;
                            pub const insert_location__child__OFFSET: usize = 8;

                        }

                    }

                }

            }

        }

    }

}

pub mod rb_tree_const_iterator_blz {
    pub mod pair_unsigned_int_const__CProductionPanelUnit {
        #[repr(C)]
        pub struct UnitData___ {
            pub _opaque: [u8; 8],
        }
        pub const UnitData_____SIZE: usize = 8;
        pub const UnitData_____NAME: &str = "blz::rb_tree_const_iterator<blz::pair<unsigned int const ,CProductionPanelUnit::UnitData> >";
        pub const UnitData_____n__OFFSET: usize = 0;

    }

}

pub mod rb_tree_iterator_blz {
    pub mod pair_unsigned_int_const__CProductionPanelUnit {
        #[repr(C)]
        pub struct UnitData___ {
            pub _opaque: [u8; 8],
        }
        pub const UnitData_____SIZE: usize = 8;
        pub const UnitData_____NAME: &str = "blz::rb_tree_iterator<blz::pair<unsigned int const ,CProductionPanelUnit::UnitData> >";

    }

}

pub mod shared_ptr_bnl {
    #[repr(C)]
    pub struct TimerService_ {
        pub _opaque: [u8; 16],
    }
    pub const TimerService___SIZE: usize = 16;
    pub const TimerService___NAME: &str = "blz::shared_ptr<bnl::TimerService>";
    pub const TimerService___m_manager__OFFSET: usize = 8;

}

pub mod unique_ptr_SoundManager {
    pub mod Events {
        pub mod UnitDialogueCallbackData_blz {
            pub mod default_delete_SoundManager {
                pub mod Events {
                    #[repr(C)]
                    pub struct UnitDialogueCallbackData___ {
                        pub _opaque: [u8; 8],
                    }
                    pub const UnitDialogueCallbackData_____SIZE: usize = 8;
                    pub const UnitDialogueCallbackData_____NAME: &str = "blz::unique_ptr<SoundManager::Events::UnitDialogueCallbackData,blz::default_delete<SoundManager::Events::UnitDialogueCallbackData> >";
                    pub const UnitDialogueCallbackData_____m_pointer__OFFSET: usize = 0;

                }

            }

        }

    }

}

pub mod unordered_map_int_CMultiplayerCallbacks {
    pub mod GameAd_blz {
        pub mod hash_int__blz {
            pub mod equal_to_int__blz {
                pub mod allocator_blz {
                    pub mod pair_int_const__CMultiplayerCallbacks {
                        #[repr(C)]
                        pub struct GameAd_____ {
                            pub _opaque: [u8; 32],
                        }
                        pub const GameAd_______SIZE: usize = 32;
                        pub const GameAd_______NAME: &str = "blz::unordered_map<int,CMultiplayerCallbacks::GameAd,blz::hash<int>,blz::equal_to<int>,blz::allocator<blz::pair<int const ,CMultiplayerCallbacks::GameAd> > >";

                    }

                }

            }

        }

    }

}

pub mod unordered_map_unsigned_char_CMultiplayerManager {
    pub mod PlayerData_blz {
        pub mod hash_unsigned_char__blz {
            pub mod equal_to_unsigned_char__blz {
                pub mod allocator_blz {
                    pub mod pair_unsigned_char_const__CMultiplayerManager {
                        #[repr(C)]
                        pub struct PlayerData_____ {
                            pub _opaque: [u8; 32],
                        }
                        pub const PlayerData_______SIZE: usize = 32;
                        pub const PlayerData_______NAME: &str = "blz::unordered_map<unsigned char,CMultiplayerManager::PlayerData,blz::hash<unsigned char>,blz::equal_to<unsigned char>,blz::allocator<blz::pair<unsigned char const ,CMultiplayerManager::PlayerData> > >";

                    }

                }

            }

        }

    }

}

pub mod unordered_map_unsigned_int_Net {
    pub mod NetProviderBNET {
        pub mod GameData_blz {
            pub mod hash_unsigned_int__blz {
                pub mod equal_to_unsigned_int__blz {
                    pub mod allocator_blz {
                        pub mod pair_unsigned_int_const__Net {
                            pub mod NetProviderBNET {
                                #[repr(C)]
                                pub struct GameData_____ {
                                    pub _opaque: [u8; 32],
                                }
                                pub const GameData_______SIZE: usize = 32;
                                pub const GameData_______NAME: &str = "blz::unordered_map<unsigned int,Net::NetProviderBNET::GameData,blz::hash<unsigned int>,blz::equal_to<unsigned int>,blz::allocator<blz::pair<unsigned int const ,Net::NetProviderBNET::GameData> > >";

                            }

                        }

                    }

                }

            }

        }

    }

}

pub mod vector_CAchievements {
    pub mod MapRequirement_blz {
        pub mod allocator_CAchievements {
            #[repr(C)]
            pub struct MapRequirement___ {
                pub _opaque: [u8; 24],
            }
            pub const MapRequirement_____SIZE: usize = 24;
            pub const MapRequirement_____NAME: &str = "blz::vector<CAchievements::MapRequirement,blz::allocator<CAchievements::MapRequirement> >";
            pub const MapRequirement_____m_elements__OFFSET: usize = 0;
            pub const MapRequirement_____m_size__OFFSET: usize = 8;
            pub const MapRequirement_____m_capacity__OFFSET: usize = 16;
            pub const MapRequirement_____m_capacity_is_embedded__OFFSET: usize = 16;

        }

    }

}

pub mod vector_CMultiplayerManager {
    pub mod PlayerData_blz {
        pub mod allocator_CMultiplayerManager {
            #[repr(C)]
            pub struct PlayerData___ {
                pub _opaque: [u8; 24],
            }
            pub const PlayerData_____SIZE: usize = 24;
            pub const PlayerData_____NAME: &str = "blz::vector<CMultiplayerManager::PlayerData,blz::allocator<CMultiplayerManager::PlayerData> >";
            pub const PlayerData_____m_elements__OFFSET: usize = 0;
            pub const PlayerData_____m_size__OFFSET: usize = 8;
            pub const PlayerData_____m_capacity__OFFSET: usize = 16;
            pub const PlayerData_____m_capacity_is_embedded__OFFSET: usize = 16;

            pub mod PlayerData_____mod {
                #[repr(C)]
                pub struct insert_raw_ranges {
                    pub _opaque: [u8; 24],
                }
                pub const insert_raw_ranges__SIZE: usize = 24;
                pub const insert_raw_ranges__NAME: &str = "blz::vector<CMultiplayerManager::PlayerData,blz::allocator<CMultiplayerManager::PlayerData> >::insert_raw_ranges";
                pub const insert_raw_ranges__begin__OFFSET: usize = 0;
                pub const insert_raw_ranges__mid__OFFSET: usize = 8;
                pub const insert_raw_ranges__end__OFFSET: usize = 16;

            }

        }

    }

}

pub mod vector_CTriggerStrings {
    pub mod TriggerString_blz {
        pub mod allocator_CTriggerStrings {
            #[repr(C)]
            pub struct TriggerString___ {
                pub _opaque: [u8; 24],
            }
            pub const TriggerString_____SIZE: usize = 24;
            pub const TriggerString_____NAME: &str = "blz::vector<CTriggerStrings::TriggerString,blz::allocator<CTriggerStrings::TriggerString> >";
            pub const TriggerString_____m_elements__OFFSET: usize = 0;
            pub const TriggerString_____m_size__OFFSET: usize = 8;
            pub const TriggerString_____m_capacity__OFFSET: usize = 16;
            pub const TriggerString_____m_capacity_is_embedded__OFFSET: usize = 16;

        }

    }

}

pub mod weak_ptr_bnl {
    #[repr(C)]
    pub struct TimerService_ {
        pub _opaque: [u8; 16],
    }
    pub const TimerService___SIZE: usize = 16;
    pub const TimerService___NAME: &str = "blz::weak_ptr<bnl::TimerService>";
    pub const TimerService___m_pointer__OFFSET: usize = 0;
    pub const TimerService___m_manager__OFFSET: usize = 8;

}

