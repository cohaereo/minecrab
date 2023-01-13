use crate::packet_ids;

// TODO: We shouldn't have to import this, at least not like this
use crate::net::ConnectionState::*;
use crate::net::PacketDirection::*;

packet_ids! {
    version 335,
    handshaking Handshaking {
        clientbound Client {
        }
        serverbound Server {
            0x00 => SetProtocol_5,
            0xfe => LegacyServerListPing_5,
        }
    }
    status Status {
        clientbound Client {
            0x00 => ServerInfo_5,
            0x01 => Ping_5,
        }
        serverbound Server {
            0x00 => PingStart_5,
            0x01 => Ping_5,
        }
    }
    login Login {
        clientbound Client {
            0x00 => Disconnect_5,
            0x01 => EncryptionBegin_47,
            0x02 => Success_5,
            0x03 => Compress_47,
        }
        serverbound Server {
            0x00 => LoginStart_5,
            0x01 => EncryptionBegin_47,
        }
    }
    play Play {
        clientbound Client {
            0x00 => SpawnEntity_107,
            0x01 => SpawnEntityExperienceOrb_107,
            0x02 => SpawnEntityWeather_107,
            0x03 => SpawnEntityLiving_315,
            0x04 => SpawnEntityPainting_107,
            0x05 => NamedEntitySpawn_107,
            0x06 => Animation_5,
            0x07 => Statistics_5,
            0x08 => BlockBreakAnimation_47,
            0x09 => TileEntityData_47,
            0x0a => BlockAction_47,
            0x0b => BlockChange_47,
            0x0c => BossBar_107,
            0x0d => Difficulty_47,
            0x0e => TabComplete_5,
            0x0f => Chat_47,
            0x10 => MultiBlockChange_47,
            0x11 => Transaction_47,
            0x12 => CloseWindow_5,
            0x13 => OpenWindow_47,
            0x14 => WindowItems_5,
            0x15 => CraftProgressBar_5,
            0x16 => SetSlot_5,
            0x17 => SetCooldown_107,
            0x18 => CustomPayload_47,
            0x19 => NamedSoundEffect_210,
            0x1a => KickDisconnect_5,
            0x1b => EntityStatus_5,
            0x1c => Explosion_5,
            0x1d => UnloadChunk_107,
            0x1e => GameStateChange_5,
            0x1f => KeepAlive_47,
            0x20 => MapChunk_110,
            0x21 => WorldEvent_47,
            0x22 => WorldParticles_47,
            0x23 => Login_109,
            0x24 => Map_107,
            0x25 => Entity_47,
            0x26 => RelEntityMove_107,
            0x27 => EntityMoveLook_107,
            0x28 => EntityLook_47,
            0x29 => VehicleMove_107,
            0x2a => OpenSignEntity_47,
            0x2b => Abilities_5,
            0x2c => CombatEvent_47,
            0x2d => PlayerInfo_47,
            0x2e => Position_107,
            0x2f => Bed_47,
            0x30 => UnlockRecipes_335,
            0x31 => EntityDestroy_47,
            0x32 => RemoveEntityEffect_47,
            0x33 => ResourcePackSend_47,
            0x34 => Respawn_5,
            0x35 => EntityHeadRotation_47,
            0x36 => SelectAdvancementTab_335,
            0x37 => WorldBorder_47,
            0x38 => Camera_47,
            0x39 => HeldItemSlot_5,
            0x3a => ScoreboardDisplayObjective_5,
            0x3b => EntityMetadata_47,
            0x3c => AttachEntity_107,
            0x3d => EntityVelocity_47,
            0x3e => EntityEquipment_107,
            0x3f => Experience_47,
            0x40 => UpdateHealth_47,
            0x41 => ScoreboardObjective_47,
            0x42 => SetPassengers_107,
            0x43 => Teams_107,
            0x44 => ScoreboardScore_47,
            0x45 => SpawnPosition_47,
            0x46 => UpdateTime_5,
            0x47 => Title_315,
            0x48 => SoundEffect_210,
            0x49 => PlayerlistHeader_47,
            0x4a => Collect_315,
            0x4b => EntityTeleport_107,
            0x4c => Advancements_335,
            0x4d => EntityUpdateAttributes_107,
            0x4e => EntityEffect_107,
        }
        serverbound Server {
            0x00 => TeleportConfirm_107,
            0x01 => PrepareCraftingGrid_335,
            0x02 => TabComplete_107,
            0x03 => Chat_5,
            0x04 => ClientCommand_107,
            0x05 => Settings_107,
            0x06 => Transaction_5,
            0x07 => EnchantItem_5,
            0x08 => WindowClick_47,
            0x09 => CloseWindow_5,
            0x0a => CustomPayload_47,
            0x0b => UseEntity_107,
            0x0c => KeepAlive_47,
            0x0d => Flying_5,
            0x0e => Position_47,
            0x0f => PositionLook_47,
            0x10 => Look_5,
            0x11 => VehicleMove_107,
            0x12 => SteerBoat_107,
            0x13 => Abilities_5,
            0x14 => BlockDig_47,
            0x15 => EntityAction_47,
            0x16 => SteerVehicle_47,
            0x17 => CraftingBookData_335,
            0x18 => ResourcePackReceive_210,
            0x19 => AdvancementTab_335,
            0x1a => HeldItemSlot_5,
            0x1b => SetCreativeSlot_5,
            0x1c => UpdateSign_47,
            0x1d => ArmAnimation_107,
            0x1e => Spectate_47,
            0x1f => BlockPlace_315,
            0x20 => UseItem_107,
        }
    }
}
