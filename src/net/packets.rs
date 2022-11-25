use crate::packet_structs;

packet_structs! {
    handshaking {
        clientbound {
        }
        serverbound {
            packet SetProtocol {
                protocol_version: VarInt,
                server_host: String,
                server_port: u16,
                next_state: VarInt,
            }
            packet LegacyServerListPing {
                payload: u8,
            }
        }
    }
    status {
        clientbound {
            packet ServerInfo {
                response: String,
            }
            packet PingClientbound {
                time: i64,
            }
        }
        serverbound {
            packet PingStart {
            }
            packet PingServerbound {
                time: i64,
            }
        }
    }
    login {
        clientbound {
            packet Disconnect {
                reason: String,
            }
            packet EncryptionBeginClientbound {
                server_id: String,
                public_key: PrefixedVec<u8, i16>,
                verify_token: PrefixedVec<u8, i16>,
            }
            packet LoginSuccess {
                uuid: String,
                username: String,
            }
        }
        serverbound {
            packet LoginStart {
                username: String,
            }
            packet EncryptionBeginServerbound {
                shared_secret: PrefixedVec<u8, i16>,
                verify_token: PrefixedVec<u8, i16>,
            }
        }
    }
    play {
        clientbound {
            packet KeepAliveClientbound {
                keep_alive_id: i32,
            }
            packet Login {
                entity_id: i32,
                game_mode: u8,
                dimension: i8,
                difficulty: u8,
                max_players: u8,
                level_type: String,
            }
            packet ChatClientbound {
                message: String,
            }
            packet UpdateTime {
                age: i64,
                time: i64,
            }
            packet EntityEquipment {
                entity_id: i32,
                slot: i16,
                item: Slot,
            }
            packet SpawnPosition {
                location: PositionIII,
            }
            packet UpdateHealth {
                health: f32,
                food: i16,
                food_saturation: f32,
            }
            packet Respawn {
                dimension: i32,
                difficulty: u8,
                gamemode: u8,
                level_type: String,
            }
            packet PositionClientbound {
                x: f64,
                y: f64,
                z: f64,
                yaw: f32,
                pitch: f32,
                on_ground: bool,
            }
            packet HeldItemSlotClientbound {
                slot: i8,
            }
            packet Bed {
                entity_id: i32,
                location: PositionIBI,
            }
            packet Animation {
                entity_id: VarInt,
                animation: u8,
            }
            packet NamedEntitySpawn {
                entity_id: VarInt,
                player_u_u_i_d: String,
                player_name: String,
                data: PrefixedVec<EntitySpawnProperty, VarInt>,
                x: FixedPoint32,
                y: FixedPoint32,
                z: FixedPoint32,
                yaw: i8,
                pitch: i8,
                current_item: i16,
                metadata: EntityMeta,
            }
            packet Collect {
                collected_entity_id: i32,
                collector_entity_id: i32,
            }
            packet SpawnEntity {
                entity_id: VarInt,
                kind: i8,
                x: FixedPoint32,
                y: FixedPoint32,
                z: FixedPoint32,
                pitch: i8,
                yaw: i8,
                data: ObjectData,
            }
            packet SpawnEntityLiving {
                entity_id: VarInt,
                kind: u8,
                x: FixedPoint32,
                y: FixedPoint32,
                z: FixedPoint32,
                yaw: i8,
                pitch: i8,
                head_pitch: i8,
                velocity_x: i16,
                velocity_y: i16,
                velocity_z: i16,
                metadata: EntityMeta,
            }
            packet SpawnEntityPainting {
                entity_id: VarInt,
                title: String,
                location: PositionIII,
                direction: i32,
            }
            packet SpawnEntityExperienceOrb {
                entity_id: VarInt,
                x: FixedPoint32,
                y: FixedPoint32,
                z: FixedPoint32,
                count: i16,
            }
            packet EntityVelocity {
                entity_id: i32,
                velocity_x: i16,
                velocity_y: i16,
                velocity_z: i16,
            }
            packet EntityDestroy {
                entity_ids: PrefixedVec<i32, i8>,
            }
            packet Entity {
                entity_id: i32,
            }
            packet RelEntityMove {
                entity_id: i32,
                d_x: FixedPoint8,
                d_y: FixedPoint8,
                d_z: FixedPoint8,
            }
            packet EntityLook {
                entity_id: i32,
                yaw: i8,
                pitch: i8,
            }
            packet EntityMoveLook {
                entity_id: i32,
                d_x: FixedPoint8,
                d_y: FixedPoint8,
                d_z: FixedPoint8,
                yaw: i8,
                pitch: i8,
            }
            packet EntityTeleport {
                entity_id: i32,
                x: FixedPoint32,
                y: FixedPoint32,
                z: FixedPoint32,
                yaw: i8,
                pitch: i8,
            }
            packet EntityHeadRotation {
                entity_id: i32,
                head_yaw: i8,
            }
            packet EntityStatus {
                entity_id: i32,
                entity_status: i8,
            }
            packet AttachEntity {
                entity_id: i32,
                vehicle_id: i32,
                leash: bool,
            }
            packet EntityMetadata {
                entity_id: i32,
                metadata: EntityMeta,
            }
            packet EntityEffect {
                entity_id: i32,
                effect_id: i8,
                amplifier: i8,
                duration: i16,
            }
            packet RemoveEntityEffect {
                entity_id: i32,
                effect_id: i8,
            }
            packet Experience {
                experience_bar: f32,
                level: i16,
                total_experience: i16,
            }
            packet UpdateAttributes {
                entity_id: i32,
                properties: PrefixedVec<EntityProperty, i32>,
            }
            packet MapChunk {
                x: i32,
                z: i32,
                ground_up: bool,
                bit_map: u16,
                add_bit_map: u16,
                compressed_chunk_data: PrefixedVec<u8, i32>,
            }
            packet MultiBlockChange {
                chunk_x: i32,
                chunk_z: i32,
                record_count: i16,
                data_length: i32,
                records: Vec<BlockChangeRecord> > vec(record_count),
            }
            packet BlockChange {
                location: PositionIBI,
                kind: VarInt,
                metadata: u8,
            }
            packet BlockAction {
                location: PositionISI,
                byte1: u8,
                byte2: u8,
                block_id: VarInt,
            }
            packet BlockBreakAnimation {
                entity_id: VarInt,
                location: PositionIII,
                destroy_stage: i8,
            }
            packet MapChunkBulk {
                column_count: i16,
                data_length: i32,
                sky_light_sent: bool,
                data: Vec<u8> > vec(data_length),
                meta: Vec<ChunkMetadata> > vec(column_count),
            }
            packet Explosion {
                x: f32,
                y: f32,
                z: f32,
                radius: f32,
                affected_block_offsets: PrefixedVec<ExplosionRecord, i32>,
                player_motion_x: f32,
                player_motion_y: f32,
                player_motion_z: f32,
            }
            packet WorldEvent {
                effect_id: i32,
                location: PositionIBI,
                data: i32,
                global: bool,
            }
            packet NamedSoundEffect {
                sound_name: String,
                x: i32,
                y: i32,
                z: i32,
                volume: f32,
                pitch: u8,
            }
            packet WorldParticles {
                particle_name: String,
                x: f32,
                y: f32,
                z: f32,
                offset_x: f32,
                offset_y: f32,
                offset_z: f32,
                particle_data: f32,
                particles: i32,
            }
            packet GameStateChange {
                reason: u8,
                game_mode: f32,
            }
            packet SpawnEntityWeather {
                entity_id: VarInt,
                kind: i8,
                x: FixedPoint32,
                y: FixedPoint32,
                z: FixedPoint32,
            }
            packet OpenWindow {
                window_id: u8,
                inventory_type: u8,
                window_title: String,
                slot_count: u8,
                use_provided_title: bool,
                entity_id: Option<i32> > when(|p: &OpenWindow| p.inventory_type == 11),
            }
            packet CloseWindowClientbound {
                window_id: u8,
            }
            packet SetSlot {
                window_id: i8,
                slot: i16,
                item: Slot,
            }
            packet WindowItems {
                window_id: u8,
                items: PrefixedVec<Slot, i16>,
            }
            packet CraftProgressBar {
                window_id: u8,
                property: i16,
                value: i16,
            }
            packet ConfirmTransactionClientbound {
                window_id: u8,
                action: i16,
                accepted: bool,
            }
            packet UpdateSignClientBound {
                location: PositionISI,
                text1: String,
                text2: String,
                text3: String,
                text4: String,
            }
            packet Map {
                item_damage: VarInt,
                data: PrefixedVec<u8, i16>,
            }
            packet TileEntityData {
                location: PositionISI,
                action: u8,
                data_length: i16,
                nbt_data: CompressedGzData<nbt::Blob>,
            }
            packet OpenSignEntity {
                location: PositionIII,
            }
            packet Statistics {
                entries: PrefixedVec<StatisticsEntry, VarInt>,
            }
            packet PlayerInfo {
                player_name: String,
                online: bool,
                ping: i16,
            }
            packet AbilitiesClientbound {
                flags: i8,
                flying_speed: f32,
                walking_speed: f32,
            }
            packet TabCompleteClientbound {
                matches: PrefixedVec<String, VarInt>,
            }
            packet ScoreboardObjective {
                name: String,
                display_text: String,
                action: i8,
            }
            packet ScoreboardScore {
                item_name: String,
                action: i8,
                score_name: Option<String> > when(|p: &ScoreboardScore| p.action != 1),
                value: Option<i32> > when(|p: &ScoreboardScore| p.action != 1),
            }
            packet ScoreboardDisplayObjective {
                position: i8,
                name: String,
            }
            packet ScoreboardTeam {
                team: String,
                action: i8,
                name: Option<String> > when(|p: &ScoreboardTeam| p.action == 0 || p.action == 2),
                prefix: Option<String> > when(|p: &ScoreboardTeam| p.action == 0 || p.action == 2),
                suffix: Option<String> > when(|p: &ScoreboardTeam| p.action == 0 || p.action == 2),
                friendly_fire: Option<i8> > when(|p: &ScoreboardTeam| p.action == 0 || p.action == 2),
                players: Option<PrefixedVec<String, i16>> > when(|p: &ScoreboardTeam| p.action == 0 || p.action == 3 || p.action == 4),
            }
            packet CustomPayloadClientbound {
                channel: String,
                data: PrefixedVec<u8, i16>,
            }
            packet KickDisconnect {
                reason: String,
            }
        }
        serverbound {
            packet KeepAliveServerbound {
                keep_alive_id: i32,
            }
            packet ChatServerbound {
                message: String,
            }
            packet UseEntity {
                target: i32,
                mouse: i8,
                x: Option<f32> > when(|p: &UseEntity| p.mouse == 2),
                y: Option<f32> > when(|p: &UseEntity| p.mouse == 2),
                z: Option<f32> > when(|p: &UseEntity| p.mouse == 2),
            }
            packet Flying {
                on_ground: bool,
            }
            packet PositionServerbound {
                x: f64,
                stance: f64,
                y: f64,
                z: f64,
                on_ground: bool,
            }
            packet Look {
                yaw: f32,
                pitch: f32,
                on_ground: bool,
            }
            packet PositionLook {
                x: f64,
                stance: f64,
                y: f64,
                z: f64,
                yaw: f32,
                pitch: f32,
                on_ground: bool,
            }
            packet BlockDig {
                status: i8,
                location: PositionIBI,
                face: i8,
            }
            packet BlockPlace {
                location: PositionIBI,
                direction: i8,
                held_item: Slot,
                cursor_x: i8,
                cursor_y: i8,
                cursor_z: i8,
            }
            packet HeldItemSlot {
                slot_id: i8,
            }
            packet ArmAnimation {
                entity_id: i32,
                animation: i8,
            }
            packet EntityAction {
                entity_id: i32,
                action_id: i8,
                jump_boost: i32,
            }
            packet SteerVehicle {
                sideways: f32,
                forward: f32,
                jump: bool,
                unmount: bool,
            }
            packet CloseWindow {
                window_id: u8,
            }
            packet WindowClick {
                window_id: i8,
                slot: i16,
                mouse_button: i8,
                action: i16,
                mode: i8,
                item: Slot,
            }
            packet CompleteTransactionServerbound {
                window_id: i8,
                action: i16,
                accepted: bool,
            }
            packet SetCreativeSlot {
                slot: i16,
                item: Slot,
            }
            packet EnchantItem {
                window_id: i8,
                enchantment: i8,
            }
            packet UpdateSignServerbound {
                location: PositionISI,
                text1: String,
                text2: String,
                text3: String,
                text4: String,
            }
            packet AbilitiesServerbound {
                flags: i8,
                flying_speed: f32,
                walking_speed: f32,
            }
            packet TabCompleteServerbound {
                text: String,
            }
            packet Settings {
                locale: String,
                view_distance: i8,
                chat_flags: i8,
                chat_colors: bool,
                difficulty: u8,
                show_cape: bool,
            }
            packet ClientCommand {
                payload: i8,
            }
            packet CustomPayloadServerbound {
                channel: String,
                data: PrefixedVec<u8, i16>,
            }
        }
    }
}
