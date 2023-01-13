// TODO: We shouldn't have to import this, at least not like this
use crate::net::ConnectionState::*;
use crate::net::PacketDirection::*;
use crate::packet_ids;

packet_ids! {
    version 47,
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
            0x00 => KeepAlive_47,
            0x01 => Login_47,
            0x02 => Chat_47,
            0x03 => UpdateTime_5,
            0x04 => EntityEquipment_47,
            0x05 => SpawnPosition_47,
            0x06 => UpdateHealth_47,
            0x07 => Respawn_5,
            0x08 => Position_47,
            0x09 => HeldItemSlot_5,
            0x0a => Bed_47,
            0x0b => Animation_5,
            0x0c => NamedEntitySpawn_47,
            0x0d => Collect_47,
            0x0e => SpawnEntity_5,
            0x0f => SpawnEntityLiving_5,
            0x10 => SpawnEntityPainting_47,
            0x11 => SpawnEntityExperienceOrb_5,
            0x12 => EntityVelocity_47,
            0x13 => EntityDestroy_47,
            0x14 => Entity_47,
            0x15 => RelEntityMove_47,
            0x16 => EntityLook_47,
            0x17 => EntityMoveLook_47,
            0x18 => EntityTeleport_47,
            0x19 => EntityHeadRotation_47,
            0x1a => EntityStatus_5,
            0x1b => AttachEntity_5,
            0x1c => EntityMetadata_47,
            0x1d => EntityEffect_47,
            0x1e => RemoveEntityEffect_47,
            0x1f => Experience_47,
            0x20 => UpdateAttributes_47,
            0x21 => MapChunk_47,
            0x22 => MultiBlockChange_47,
            0x23 => BlockChange_47,
            0x24 => BlockAction_47,
            0x25 => BlockBreakAnimation_47,
            0x26 => MapChunkBulk_47,
            0x27 => Explosion_5,
            0x28 => WorldEvent_47,
            0x29 => NamedSoundEffect_5,
            0x2a => WorldParticles_47,
            0x2b => GameStateChange_5,
            0x2c => SpawnEntityWeather_5,
            0x2d => OpenWindow_47,
            0x2e => CloseWindow_5,
            0x2f => SetSlot_5,
            0x30 => WindowItems_5,
            0x31 => CraftProgressBar_5,
            0x32 => Transaction_47,
            0x33 => UpdateSign_47,
            0x34 => Map_47,
            0x35 => TileEntityData_47,
            0x36 => OpenSignEntity_47,
            0x37 => Statistics_5,
            0x38 => PlayerInfo_47,
            0x39 => Abilities_5,
            0x3a => TabComplete_5,
            0x3b => ScoreboardObjective_47,
            0x3c => ScoreboardScore_47,
            0x3d => ScoreboardDisplayObjective_5,
            0x3e => ScoreboardTeam_47,
            0x3f => CustomPayload_47,
            0x40 => KickDisconnect_5,
            0x41 => Difficulty_47,
            0x42 => CombatEvent_47,
            0x43 => Camera_47,
            0x44 => WorldBorder_47,
            0x45 => Title_47,
            0x46 => SetCompression_47,
            0x47 => PlayerlistHeader_47,
            0x48 => ResourcePackSend_47,
            0x49 => UpdateEntityNbt_47,
        }
        serverbound Server {
            0x00 => KeepAlive_47,
            0x01 => ChatServerbound_5,
            0x02 => UseEntity_47,
            0x03 => Flying_5,
            0x04 => Position_47,
            0x05 => Look_5,
            0x06 => PositionLook_47,
            0x07 => BlockDig_47,
            0x08 => BlockPlace_47,
            0x09 => HeldItemSlot_5,
            0x0a => ArmAnimation_47,
            0x0b => EntityAction_47,
            0x0c => SteerVehicle_47,
            0x0d => CloseWindow_5,
            0x0e => WindowClick_47,
            0x0f => Transaction_5,
            0x10 => SetCreativeSlot_5,
            0x11 => EnchantItem_5,
            0x12 => UpdateSign_47,
            0x13 => Abilities_5,
            0x14 => TabCompleteServerbound_47,
            0x15 => Settings_47,
            0x16 => ClientCommand_47,
            0x17 => CustomPayload_47,
            0x18 => Spectate_47,
            0x19 => ResourcePackReceive_47,
        }
    }
}
