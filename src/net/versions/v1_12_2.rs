use crate::packet_ids;

// TODO: We shouldn't have to import this, at least not like this
use crate::net::ConnectionState::*;
use crate::net::PacketDirection::*;

packet_ids! {
    version 340,
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
            0x1f => KeepAlive_340,
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
            0x2b => CraftRecipeResponse_338,
            0x2c => Abilities_5,
            0x2d => CombatEvent_47,
            0x2e => PlayerInfo_47,
            0x2f => Position_107,
            0x30 => Bed_47,
            0x31 => UnlockRecipes_335,
            0x32 => EntityDestroy_47,
            0x33 => RemoveEntityEffect_47,
            0x34 => ResourcePackSend_47,
            0x35 => Respawn_5,
            0x36 => EntityHeadRotation_47,
            0x37 => SelectAdvancementTab_335,
            0x38 => WorldBorder_47,
            0x39 => Camera_47,
            0x3a => HeldItemSlot_5,
            0x3b => ScoreboardDisplayObjective_5,
            0x3c => EntityMetadata_47,
            0x3d => AttachEntity_107,
            0x3e => EntityVelocity_47,
            0x3f => EntityEquipment_107,
            0x40 => Experience_47,
            0x41 => UpdateHealth_47,
            0x42 => ScoreboardObjective_47,
            0x43 => SetPassengers_107,
            0x44 => Teams_107,
            0x45 => ScoreboardScore_47,
            0x46 => SpawnPosition_47,
            0x47 => UpdateTime_5,
            0x48 => Title_315,
            0x49 => SoundEffect_210,
            0x4a => PlayerlistHeader_47,
            0x4b => Collect_315,
            0x4c => EntityTeleport_107,
            0x4d => Advancements_335,
            0x4e => EntityUpdateAttributes_107,
            0x4f => EntityEffect_107,
        }
        serverbound Server {
            0x00 => TeleportConfirm_107,
            0x01 => TabComplete_107,
            0x02 => Chat_5,
            0x03 => ClientCommand_107,
            0x04 => Settings_107,
            0x05 => Transaction_5,
            0x06 => EnchantItem_5,
            0x07 => WindowClick_47,
            0x08 => CloseWindow_5,
            0x09 => CustomPayload_47,
            0x0a => UseEntity_107,
            0x0b => KeepAlive_340,
            0x0c => Flying_5,
            0x0d => Position_47,
            0x0e => PositionLook_47,
            0x0f => Look_5,
            0x10 => VehicleMove_107,
            0x11 => SteerBoat_107,
            0x12 => CraftRecipeRequest_338,
            0x13 => Abilities_5,
            0x14 => BlockDig_47,
            0x15 => EntityAction_47,
            0x16 => SteerVehicle_47,
            0x17 => CraftingBookData_340,
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
