use crate::packet_ids;

// TODO: We shouldn't have to import this, at least not like this
use crate::net::ConnectionState::*;
use crate::net::PacketDirection::*;

packet_ids! {
    version 5,
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
            0x01 => EncryptionBegin_5,
            0x02 => Success_5,
        }
        serverbound Server {
            0x00 => LoginStart_5,
            0x01 => EncryptionBegin_5,
        }
    }
    play Play {
        clientbound Client {
            0x00 => KeepAlive_5,
            0x01 => Login_5,
            0x02 => Chat_5,
            0x03 => UpdateTime_5,
            0x04 => EntityEquipment_5,
            0x05 => SpawnPosition_5,
            0x06 => UpdateHealth_5,
            0x07 => Respawn_5,
            0x08 => Position_5,
            0x09 => HeldItemSlot_5,
            0x0a => Bed_5,
            0x0b => Animation_5,
            0x0c => NamedEntitySpawn_5,
            0x0d => Collect_5,
            0x0e => SpawnEntity_5,
            0x0f => SpawnEntityLiving_5,
            0x10 => SpawnEntityPainting_5,
            0x11 => SpawnEntityExperienceOrb_5,
            0x12 => EntityVelocity_5,
            0x13 => EntityDestroy_5,
            0x14 => Entity_5,
            0x15 => RelEntityMove_5,
            0x16 => EntityLook_5,
            0x17 => EntityMoveLook_5,
            0x18 => EntityTeleport_5,
            0x19 => EntityHeadRotation_5,
            0x1a => EntityStatus_5,
            0x1b => AttachEntity_5,
            0x1c => EntityMetadata_5,
            0x1d => EntityEffect_5,
            0x1e => RemoveEntityEffect_5,
            0x1f => Experience_5,
            0x20 => UpdateAttributes_5,
            0x21 => MapChunk_5,
            0x22 => MultiBlockChange_5,
            0x23 => BlockChange_5,
            0x24 => BlockAction_5,
            0x25 => BlockBreakAnimation_5,
            0x26 => MapChunkBulk_5,
            0x27 => Explosion_5,
            0x28 => WorldEvent_5,
            0x29 => NamedSoundEffect_5,
            0x2a => WorldParticles_5,
            0x2b => GameStateChange_5,
            0x2c => SpawnEntityWeather_5,
            0x2d => OpenWindow_5,
            0x2e => CloseWindow_5,
            0x2f => SetSlot_5,
            0x30 => WindowItems_5,
            0x31 => CraftProgressBar_5,
            0x32 => Transaction_5,
            0x33 => UpdateSign_5,
            0x34 => Map_5,
            0x35 => TileEntityData_5,
            0x36 => OpenSignEntity_5,
            0x37 => Statistics_5,
            0x38 => PlayerInfo_5,
            0x39 => Abilities_5,
            0x3a => TabComplete_5,
            0x3b => ScoreboardObjective_5,
            0x3c => ScoreboardScore_5,
            0x3d => ScoreboardDisplayObjective_5,
            0x3e => ScoreboardTeam_5,
            0x3f => CustomPayload_5,
            0x40 => KickDisconnect_5,
        }
        serverbound Server {
            0x00 => KeepAlive_5,
            0x01 => ChatServerbound_5,
            0x02 => UseEntity_5,
            0x03 => Flying_5,
            0x04 => Position_5,
            0x05 => Look_5,
            0x06 => PositionLook_5,
            0x07 => BlockDig_5,
            0x08 => BlockPlace_5,
            0x09 => HeldItemSlot_5,
            0x0a => ArmAnimation_5,
            0x0b => EntityAction_5,
            0x0c => SteerVehicle_5,
            0x0d => CloseWindow_5,
            0x0e => WindowClick_5,
            0x0f => Transaction_5,
            0x10 => SetCreativeSlot_5,
            0x11 => EnchantItem_5,
            0x12 => UpdateSign_5,
            0x13 => Abilities_5,
            0x14 => TabComplete_5,
            0x15 => Settings_5,
            0x16 => ClientCommand_5,
            0x17 => CustomPayload_5,
        }
    }
}
