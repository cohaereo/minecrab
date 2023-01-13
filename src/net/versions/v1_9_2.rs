use crate::packet_ids;

// TODO: We shouldn't have to import this, at least not like this
use crate::net::ConnectionState::*;
use crate::net::PacketDirection::*;

packet_ids! {
    version 109,
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
            0x03 => SpawnEntityLiving_107,
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
            0x19 => NamedSoundEffect_107,
            0x1a => KickDisconnect_5,
            0x1b => EntityStatus_5,
            0x1c => Explosion_5,
            0x1d => UnloadChunk_107,
            0x1e => GameStateChange_5,
            0x1f => KeepAlive_47,
            0x20 => MapChunk_107,
            0x21 => WorldEvent_47,
            0x22 => WorldParticles_47,
            0x23 => Login_109,
            0x24 => Map_107,
            0x25 => RelEntityMove_107,
            0x26 => EntityMoveLook_107,
            0x27 => EntityLook_47,
            0x28 => Entity_47,
            0x29 => VehicleMove_107,
            0x2a => OpenSignEntity_47,
            0x2b => Abilities_5,
            0x2c => CombatEvent_47,
            0x2d => PlayerInfo_47,
            0x2e => Position_107,
            0x2f => Bed_47,
            0x30 => EntityDestroy_47,
            0x31 => RemoveEntityEffect_47,
            0x32 => ResourcePackSend_47,
            0x33 => Respawn_5,
            0x34 => EntityHeadRotation_47,
            0x35 => WorldBorder_47,
            0x36 => Camera_47,
            0x37 => HeldItemSlot_5,
            0x38 => ScoreboardDisplayObjective_5,
            0x39 => EntityMetadata_47,
            0x3a => AttachEntity_107,
            0x3b => EntityVelocity_47,
            0x3c => EntityEquipment_107,
            0x3d => Experience_47,
            0x3e => UpdateHealth_47,
            0x3f => ScoreboardObjective_47,
            0x40 => SetPassengers_107,
            0x41 => Teams_107,
            0x42 => ScoreboardScore_47,
            0x43 => SpawnPosition_47,
            0x44 => UpdateTime_5,
            0x45 => Title_47,
            0x46 => UpdateSign_47,
            0x47 => SoundEffect_107,
            0x48 => PlayerlistHeader_47,
            0x49 => Collect_47,
            0x4a => EntityTeleport_107,
            0x4b => EntityUpdateAttributes_107,
            0x4c => EntityEffect_107,
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
            0x0b => KeepAlive_47,
            0x0c => Position_47,
            0x0d => PositionLook_47,
            0x0e => Look_5,
            0x0f => Flying_5,
            0x10 => VehicleMove_107,
            0x11 => SteerBoat_107,
            0x12 => Abilities_5,
            0x13 => BlockDig_47,
            0x14 => EntityAction_47,
            0x15 => SteerVehicle_47,
            0x16 => ResourcePackReceive_47,
            0x17 => HeldItemSlot_5,
            0x18 => SetCreativeSlot_5,
            0x19 => UpdateSign_47,
            0x1a => ArmAnimation_107,
            0x1b => Spectate_47,
            0x1c => BlockPlace_107,
            0x1d => UseItem_107,
        }
    }
}
