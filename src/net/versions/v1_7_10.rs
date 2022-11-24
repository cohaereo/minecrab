use crate::packet_ids;

// TODO: We shouldn't have to import this, at least not like this
use crate::net::ConnectionState::*;
use crate::net::PacketDirection::*;

packet_ids! {
    handshaking Handshaking {
        clientbound Client {}
        serverbound Server {
                0x00 => SetProtocol,
                0xfe => LegacyServerListPing,
        }
    }
    status Status {
        clientbound Client {
                0x00 => ServerInfo,
                0x01 => PingClientbound,
        }
        serverbound Server {
                0x00 => PingStart,
                0x01 => PingServerbound,
        }
    }
    login Login {
        clientbound Client {
                0x00 => Disconnect,
                0x01 => EncryptionBeginClientbound,
                0x02 => LoginSuccess,
        }
        serverbound Server {
                0x00 => LoginStart,
                0x01 => EncryptionBeginServerbound,
        }
    }
    play Play {
        clientbound Client {
                0x00 => KeepAliveClientbound,
                0x01 => Login,
                0x02 => ChatClientbound,
                0x03 => UpdateTime,
                0x04 => EntityEquipment,
                0x05 => SpawnPosition,
                0x06 => UpdateHealth,
                0x07 => Respawn,
                0x08 => PositionClientbound,
                0x09 => HeldItemSlot,
                0x0a => Bed,
                0x0b => Animation,
                0x0c => NamedEntitySpawn,
                0x0d => Collect,
                0x0e => SpawnEntity,
                0x0f => SpawnEntityLiving,
                0x10 => SpawnEntityPainting,
                0x11 => SpawnEntityExperienceOrb,
                0x12 => EntityVelocity,
                0x13 => EntityDestroy,
                0x14 => Entity,
                0x15 => RelEntityMove,
                0x16 => EntityLook,
                0x17 => EntityMoveLook,
                0x18 => EntityTeleport,
                0x19 => EntityHeadRotation,
                0x1a => EntityStatus,
                0x1b => AttachEntity,
                0x1c => EntityMetadata,
                0x1d => EntityEffect,
                0x1e => RemoveEntityEffect,
                0x1f => Experience,
                0x20 => UpdateAttributes,
                0x21 => MapChunk,
                0x22 => MultiBlockChange,
                0x23 => BlockChange,
                0x24 => BlockAction,
                0x25 => BlockBreakAnimation,
                0x26 => MapChunkBulk,
                0x27 => Explosion,
                0x28 => WorldEvent,
                0x29 => NamedSoundEffect,
                0x2a => WorldParticles,
                0x2b => GameStateChange,
                0x2c => SpawnEntityWeather,
                0x2d => OpenWindow,
                0x2e => CloseWindow,
                0x2f => SetSlot,
                0x30 => WindowItems,
                0x31 => CraftProgressBar,
                0x32 => ConfirmTransactionClientbound,
                0x33 => UpdateSignClientBound,
                0x34 => Map,
                0x35 => TileEntityData,
                0x36 => OpenSignEntity,
                0x37 => Statistics,
                0x38 => PlayerInfo,
                0x39 => AbilitiesClientbound,
                0x3a => TabCompleteClientbound,
                0x3b => ScoreboardObjective,
                0x3c => ScoreboardScore,
                0x3d => ScoreboardDisplayObjective,
                0x3e => ScoreboardTeam,
                0x3f => CustomPayloadClientbound,
                0x40 => KickDisconnect,
        }
        serverbound Server {
                0x00 => KeepAliveServerbound,
                0x01 => ChatServerbound,
                0x02 => UseEntity,
                0x03 => Flying,
                0x04 => PositionServerbound,
                0x05 => Look,
                0x06 => PositionLook,
                0x07 => BlockDig,
                0x08 => BlockPlace,
                0x09 => HeldItemSlot,
                0x0a => ArmAnimation,
                0x0b => EntityAction,
                0x0c => SteerVehicle,
                0x0d => CloseWindow,
                0x0e => WindowClick,
                0x0f => CompleteTransactionServerbound,
                0x10 => SetCreativeSlot,
                0x11 => EnchantItem,
                0x12 => UpdateSignServerbound,
                0x13 => AbilitiesServerbound,
                0x14 => TabCompleteServerbound,
                0x15 => Settings,
                0x16 => ClientCommand,
                0x17 => CustomPayloadServerbound,
        }
    }
}
