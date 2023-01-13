#![allow(non_camel_case_types)]
use crate::packet_structs;

packet_structs! {
    handshaking {
        clientbound {
        }
        serverbound {
            packet LegacyServerListPing_5 {
                payload: u8,
            }
            packet SetProtocol_5 {
                protocol_version: VarInt,
                server_host: String,
                server_port: u16,
                next_state: VarInt,
            }
        }
    }
    login {
        clientbound {
            packet Compress_47 {
                threshold: VarInt,
            }
            packet Disconnect_5 {
                reason: String,
            }
            packet EncryptionBegin_5 {
                server_id: String,
                public_key: PrefixedVec<u8, i16>,
                verify_token: PrefixedVec<u8, i16>,
            }
            packet EncryptionBegin_47 {
                server_id: String,
                public_key: PrefixedVec<u8, VarInt>,
                verify_token: PrefixedVec<u8, VarInt>,
            }
            packet Success_5 {
                uuid: String,
                username: String,
            }
        }
        serverbound {
            packet EncryptionBeginServerbound_5 {
                shared_secret: PrefixedVec<u8, i16>,
                verify_token: PrefixedVec<u8, i16>,
            }
            packet EncryptionBeginServerbound_47 {
                shared_secret: PrefixedVec<u8, VarInt>,
                verify_token: PrefixedVec<u8, VarInt>,
            }
            packet LoginStart_5 {
                username: String,
            }
        }
    }
    status {
        clientbound {
            packet Ping_5 {
                time: i64,
            }
            packet ServerInfo_5 {
                response: String,
            }
        }
        serverbound {
            packet PingStart_5 {
            }
            packet PingServerbound_5 {
                time: i64,
            }
        }
    }
    play {
        clientbound {
            packet Abilities_5 {
                flags: i8,
                flying_speed: f32,
                walking_speed: f32,
            }
            // TODO: Unfinished
            packet Advancements_335 {
                reset: bool,
                // advancement_mapping: ['array', {'countType': 'varint', 'type': ['container', [{'name': 'key', 'type': 'string'}, {'name': 'value', 'type': ['container', [{'name': 'parentId', 'type': ['option', 'string']}, {'name': 'displayData', 'type': ['option', ['container', [{'name': 'title', 'type': 'string'}, {'name': 'description', 'type': 'string'}, {'name': 'icon', 'type': 'slot'}, {'name': 'frameType', 'type': 'varint'}, {'name': 'flags', 'type': ['bitfield', [{'name': '_unused', 'size': 29, 'signed': False}, {'name': 'hidden', 'size': 1, 'signed': False}, {'name': 'show_toast', 'size': 1, 'signed': False}, {'name': 'has_background_texture', 'size': 1, 'signed': False}]]}, {'name': 'backgroundTexture', 'type': ['switch', {'compareTo': 'flags/has_background_texture', 'fields': {'1': 'string'}, 'default': 'void'}]}, {'name': 'xCord', 'type': 'f32'}, {'name': 'yCord', 'type': 'f32'}]]]}, {'name': 'criteria', 'type': ['array', {'countType': 'varint', 'type': ['container', [{'name': 'key', 'type': 'string'}, {'name': 'value', 'type': 'void'}]]}]}, {'name': 'requirements', 'type': ['array', {'countType': 'varint', 'type': ['array', {'countType': 'varint', 'type': 'string'}]}]}]]}]]}],
                // identifiers: PrefixedVec<String, VarInt>,
                // progress_mapping: ['array', {'countType': 'varint', 'type': ['container', [{'name': 'key', 'type': 'string'}, {'name': 'value', 'type': ['array', {'countType': 'varint', 'type': ['container', [{'name': 'criterionIdentifier', 'type': 'string'}, {'name': 'criterionProgress', 'type': ['option', 'i64']}]]}]}]]}],
            }
            packet Animation_5 {
                entity_id: VarInt,
                animation: u8,
            }
            packet AttachEntity_5 {
                entity_id: i32,
                vehicle_id: i32,
                leash: bool,
            }
            packet AttachEntity_107 {
                entity_id: i32,
                vehicle_id: i32,
            }
            packet Bed_5 {
                entity_id: i32,
                location: PositionIBI,
            }
            packet Bed_47 {
                entity_id: VarInt,
                location: Position,
            }
            packet BlockAction_5 {
                location: PositionISI,
                byte1: u8,
                byte2: u8,
                block_id: VarInt,
            }
            packet BlockAction_47 {
                location: Position,
                byte1: u8,
                byte2: u8,
                block_id: VarInt,
            }
            packet BlockBreakAnimation_5 {
                entity_id: VarInt,
                location: PositionIII,
                destroy_stage: i8,
            }
            packet BlockBreakAnimation_47 {
                entity_id: VarInt,
                location: Position,
                destroy_stage: i8,
            }
            packet BlockChange_5 {
                location: PositionIBI,
                kind: VarInt,
                metadata: u8,
            }
            packet BlockChange_47 {
                location: Position,
                kind: VarInt,
            }
            packet BossBar_107 {
                entity_u_u_i_d: uuid::Uuid,
                action: VarInt,
                title: Option<String> > when(|p: &BossBar_107| p.action == 0 || p.action == 3),
                health: Option<f32> > when(|p: &BossBar_107| p.action == 0 || p.action == 2),
                color: Option<VarInt> > when(|p: &BossBar_107| p.action == 0 || p.action == 4),
                dividers: Option<VarInt> > when(|p: &BossBar_107| p.action == 0 || p.action == 4),
                flags: Option<u8> > when(|p: &BossBar_107| p.action == 0 || p.action == 5),
            }
            packet Camera_47 {
                camera_id: VarInt,
            }
            packet Chat_5 {
                message: String,
            }
            packet Chat_47 {
                message: String,
                position: i8,
            }
            packet CloseWindow_5 {
                window_id: u8,
            }
            packet Collect_5 {
                collected_entity_id: i32,
                collector_entity_id: i32,
            }
            packet Collect_47 {
                collected_entity_id: VarInt,
                collector_entity_id: VarInt,
            }
            packet Collect_315 {
                collected_entity_id: VarInt,
                collector_entity_id: VarInt,
                pickup_item_count: VarInt,
            }
            packet CombatEvent_47 {
                event: VarInt,
                duration: Option<VarInt> > when(|p: &CombatEvent_47| p.event == 1),
                player_id: Option<VarInt> > when(|p: &CombatEvent_47| p.event == 2),
                entity_id: Option<i32> > when(|p: &CombatEvent_47| p.event == 1 || p.event == 2),
                message: Option<String> > when(|p: &CombatEvent_47| p.event == 2),
            }
            packet CraftProgressBar_5 {
                window_id: u8,
                property: i16,
                value: i16,
            }
            packet CraftRecipeResponse_338 {
                window_id: i8,
                recipe: VarInt,
            }
            packet CustomPayload_5 {
                channel: String,
                data: PrefixedVec<u8, i16>,
            }
            packet CustomPayload_47 {
                channel: String,
                data: Vec<u8>,
            }
            packet Difficulty_47 {
                difficulty: u8,
            }
            packet EntityDestroy_5 {
                entity_ids: PrefixedVec<i32, i8>,
            }
            packet EntityDestroy_47 {
                entity_ids: PrefixedVec<VarInt, VarInt>,
            }
            packet EntityEffect_5 {
                entity_id: i32,
                effect_id: i8,
                amplifier: i8,
                duration: i16,
            }
            packet EntityEffect_47 {
                entity_id: VarInt,
                effect_id: i8,
                amplifier: i8,
                duration: VarInt,
                hide_particles: bool,
            }
            packet EntityEffect_107 {
                entity_id: VarInt,
                effect_id: i8,
                amplifier: i8,
                duration: VarInt,
                hide_particles: i8,
            }
            packet EntityEquipment_5 {
                entity_id: i32,
                slot: i16,
                item: Slot,
            }
            packet EntityEquipment_47 {
                entity_id: VarInt,
                slot: i16,
                item: Slot,
            }
            packet EntityEquipment_107 {
                entity_id: VarInt,
                slot: VarInt,
                item: Slot,
            }
            packet EntityHeadRotation_5 {
                entity_id: i32,
                head_yaw: i8,
            }
            packet EntityHeadRotation_47 {
                entity_id: VarInt,
                head_yaw: i8,
            }
            packet EntityLook_5 {
                entity_id: i32,
                yaw: i8,
                pitch: i8,
            }
            packet EntityLook_47 {
                entity_id: VarInt,
                yaw: i8,
                pitch: i8,
                on_ground: bool,
            }
            packet EntityMetadata_5 {
                entity_id: i32,
                metadata: EntityMeta,
            }
            packet EntityMetadata_47 {
                entity_id: VarInt,
                metadata: EntityMeta,
            }
            packet EntityMoveLook_5 {
                entity_id: i32,
                d_x: FixedPoint8,
                d_y: FixedPoint8,
                d_z: FixedPoint8,
                yaw: i8,
                pitch: i8,
            }
            packet EntityMoveLook_47 {
                entity_id: VarInt,
                d_x: FixedPoint8,
                d_y: FixedPoint8,
                d_z: FixedPoint8,
                yaw: i8,
                pitch: i8,
                on_ground: bool,
            }
            packet EntityMoveLook_107 {
                entity_id: VarInt,
                d_x: FixedPoint16,
                d_y: FixedPoint16,
                d_z: FixedPoint16,
                yaw: i8,
                pitch: i8,
                on_ground: bool,
            }
            packet EntityStatus_5 {
                entity_id: i32,
                entity_status: i8,
            }
            packet EntityTeleport_5 {
                entity_id: i32,
                x: FixedPoint32,
                y: FixedPoint32,
                z: FixedPoint32,
                yaw: i8,
                pitch: i8,
            }
            packet EntityTeleport_47 {
                entity_id: VarInt,
                x: FixedPoint32,
                y: FixedPoint32,
                z: FixedPoint32,
                yaw: i8,
                pitch: i8,
                on_ground: bool,
            }
            packet EntityTeleport_107 {
                entity_id: VarInt,
                x: f64,
                y: f64,
                z: f64,
                yaw: i8,
                pitch: i8,
                on_ground: bool,
            }
            // TODO: Unfinished
            packet EntityUpdateAttributes_107 {
                entity_id: VarInt,
                // properties: ['array', {'countType': 'i32', 'type': ['container', [{'name': 'key', 'type': 'string'}, {'name': 'value', 'type': 'f64'}, {'name': 'modifiers', 'type': ['array', {'countType': 'varint', 'type': ['container', [{'name': 'uuid', 'type': 'UUID'}, {'name': 'amount', 'type': 'f64'}, {'name': 'operation', 'type': 'i8'}]]}]}]]}],
            }
            packet EntityVelocity_5 {
                entity_id: i32,
                velocity_x: i16,
                velocity_y: i16,
                velocity_z: i16,
            }
            packet EntityVelocity_47 {
                entity_id: VarInt,
                velocity_x: i16,
                velocity_y: i16,
                velocity_z: i16,
            }
            packet Entity_5 {
                entity_id: i32,
            }
            packet Entity_47 {
                entity_id: VarInt,
            }
            packet Experience_5 {
                experience_bar: f32,
                level: i16,
                total_experience: i16,
            }
            packet Experience_47 {
                experience_bar: f32,
                level: VarInt,
                total_experience: VarInt,
            }
            packet Explosion_5 {
                x: f32,
                y: f32,
                z: f32,
                radius: f32,
                affected_block_offsets: PrefixedVec<ExplosionRecord_5, i32>,
                player_motion_x: f32,
                player_motion_y: f32,
                player_motion_z: f32,
            }
            packet GameStateChange_5 {
                reason: u8,
                game_mode: f32,
            }
            packet HeldItemSlot_5 {
                slot: i8,
            }
            packet KeepAlive_5 {
                keep_alive_id: i32,
            }
            packet KeepAlive_47 {
                keep_alive_id: VarInt,
            }
            packet KeepAlive_340 {
                keep_alive_id: i64,
            }
            packet KickDisconnect_5 {
                reason: String,
            }
            packet Login_5 {
                entity_id: i32,
                game_mode: u8,
                dimension: i8,
                difficulty: u8,
                max_players: u8,
                level_kind: String,
            }
            packet Login_47 {
                entity_id: i32,
                game_mode: u8,
                dimension: i8,
                difficulty: u8,
                max_players: u8,
                level_kind: String,
                reduced_debug_info: bool,
            }
            packet Login_109 {
                entity_id: i32,
                game_mode: u8,
                dimension: i32,
                difficulty: u8,
                max_players: u8,
                level_kind: String,
                reduced_debug_info: bool,
            }
            packet MapChunkBulk_5 {
                column_count: i16,
                data_length: i32,
                sky_light_sent: bool,
                data: Vec<u8> > vec(data_length),
                meta: Vec<ChunkMetadata> > vec(column_count),
            }
            packet MapChunkBulk_47 {
                sky_light_sent: bool,
                meta: PrefixedVec<ChunkMetadata_47, VarInt>,
                data: Vec<u8>,
            }
            packet MapChunk_5 {
                x: i32,
                z: i32,
                ground_up: bool,
                bit_map: u16,
                add_bit_map: u16,
                compressed_chunk_data: PrefixedVec<u8, i32>,
            }
            packet MapChunk_47 {
                x: i32,
                z: i32,
                ground_up: bool,
                bit_map: u16,
                chunk_data: PrefixedVec<u8, VarInt>,
            }
            packet MapChunk_107 {
                x: i32,
                z: i32,
                ground_up: bool,
                bit_map: VarInt,
                chunk_data: PrefixedVec<u8, VarInt>,
            }
            packet MapChunk_110 {
                x: i32,
                z: i32,
                ground_up: bool,
                bit_map: VarInt,
                chunk_data: PrefixedVec<u8, VarInt>,
                block_entities: PrefixedVec<nbt::Blob, VarInt>,
            }
            packet Map_5 {
                item_damage: VarInt,
                data: PrefixedVec<u8, i16>,
            }
            // TODO: Unfinished
            packet Map_47 {
                item_damage: VarInt,
                scale: i8,
                // icons: ['array', {'countType': 'varint', 'type': ['container', [{'name': 'directionAndType', 'type': 'i8'}, {'name': 'x', 'type': 'i8'}, {'name': 'z', 'type': 'i8'}]]}],
                // columns: i8,
                // rows: Option<i8> > when(|p: &Map_47| p.columns != 0),
                // x: Option<i8> > when(|p: &Map_47| p.columns != 0),
                // y: Option<i8> > when(|p: &Map_47| p.columns != 0),
                // data: Option<PrefixedVec<u8, VarInt>> > when(|p: &Map_47| p.columns != 0),
            }
            // TODO: Unfinished
            packet Map_107 {
                item_damage: VarInt,
                scale: i8,
                tracking_position: bool,
                // icons: ['array', {'countType': 'varint', 'type': ['container', [{'name': 'directionAndType', 'type': 'i8'}, {'name': 'x', 'type': 'i8'}, {'name': 'z', 'type': 'i8'}]]}],
                // columns: i8,
                // rows: Option<i8> > when(|p: &Map_107| p.columns != 0),
                // x: Option<i8> > when(|p: &Map_107| p.columns != 0),
                // y: Option<i8> > when(|p: &Map_107| p.columns != 0),
                // data: Option<PrefixedVec<u8, VarInt>> > when(|p: &Map_107| p.columns != 0),
            }
            packet MultiBlockChange_5 {
                chunk_x: i32,
                chunk_z: i32,
                record_count: i16,
                data_length: i32,
                records: Vec<BlockChangeRecord_5> > vec(record_count),
            }
            packet MultiBlockChange_47 {
                chunk_x: i32,
                chunk_z: i32,
                records: PrefixedVec<BlockChangeRecord_47, VarInt>,
            }
            packet NamedEntitySpawn_5 {
                entity_id: VarInt,
                player_u_u_i_d: String,
                player_name: String,
                data: PrefixedVec<EntitySpawnProperty_5, VarInt>,
                x: FixedPoint32,
                y: FixedPoint32,
                z: FixedPoint32,
                yaw: i8,
                pitch: i8,
                current_item: i16,
                metadata: EntityMeta,
            }
            packet NamedEntitySpawn_47 {
                entity_id: VarInt,
                player_u_u_i_d: uuid::Uuid,
                x: FixedPoint32,
                y: FixedPoint32,
                z: FixedPoint32,
                yaw: i8,
                pitch: i8,
                current_item: i16,
                metadata: EntityMeta,
            }
            packet NamedEntitySpawn_107 {
                entity_id: VarInt,
                player_u_u_i_d: uuid::Uuid,
                x: f64,
                y: f64,
                z: f64,
                yaw: i8,
                pitch: i8,
                metadata: EntityMeta,
            }
            packet NamedSoundEffect_5 {
                sound_name: String,
                x: i32,
                y: i32,
                z: i32,
                volume: f32,
                pitch: u8,
            }
            packet NamedSoundEffect_107 {
                sound_name: String,
                sound_category: VarInt,
                x: i32,
                y: i32,
                z: i32,
                volume: f32,
                pitch: u8,
            }
            packet NamedSoundEffect_210 {
                sound_name: String,
                sound_category: VarInt,
                x: i32,
                y: i32,
                z: i32,
                volume: f32,
                pitch: f32,
            }
            packet OpenSignEntity_5 {
                location: PositionIII,
            }
            packet OpenSignEntity_47 {
                location: Position,
            }
            packet OpenWindow_5 {
                window_id: u8,
                inventory_type: u8,
                window_title: String,
                slot_count: u8,
                use_provided_title: bool,
                entity_id: Option<i32> > when(|p: &OpenWindow_5| p.inventory_type == 11),
            }
            packet OpenWindow_47 {
                window_id: u8,
                inventory_type: String,
                window_title: String,
                slot_count: u8,
                entity_id: Option<i32> > when(|p: &OpenWindow_47| p.inventory_type == "EntityHorse"),
            }
            packet PlayerInfo_5 {
                player_name: String,
                online: bool,
                ping: i16,
            }
            // TODO: Unfinished
            packet PlayerInfo_47 {
                action: VarInt,
                // data: ['array', {'countType': 'varint', 'type': ['container', [{'name': 'UUID', 'type': 'UUID'}, {'name': 'name', 'type': ['switch', {'compareTo': '../action', 'fields': {'0': 'string'}, 'default': 'void'}]}, {'name': 'properties', 'type': ['switch', {'compareTo': '../action', 'fields': {'0': ['array', {'countType': 'varint', 'type': ['container', [{'name': 'name', 'type': 'string'}, {'name': 'value', 'type': 'string'}, {'name': 'signature', 'type': ['option', 'string']}]]}]}, 'default': 'void'}]}, {'name': 'gamemode', 'type': ['switch', {'compareTo': '../action', 'fields': {'0': 'varint', '1': 'varint'}, 'default': 'void'}]}, {'name': 'ping', 'type': ['switch', {'compareTo': '../action', 'fields': {'0': 'varint', '2': 'varint'}, 'default': 'void'}]}, {'name': 'displayName', 'type': ['switch', {'compareTo': '../action', 'fields': {'0': ['option', 'string'], '3': ['option', 'string']}, 'default': 'void'}]}]]}],
            }
            packet PlayerlistHeader_47 {
                header: String,
                footer: String,
            }
            packet Position_5 {
                x: f64,
                y: f64,
                z: f64,
                yaw: f32,
                pitch: f32,
                on_ground: bool,
            }
            packet Position_47 {
                x: f64,
                y: f64,
                z: f64,
                yaw: f32,
                pitch: f32,
                flags: i8,
            }
            packet Position_107 {
                x: f64,
                y: f64,
                z: f64,
                yaw: f32,
                pitch: f32,
                flags: i8,
                teleport_id: VarInt,
            }
            packet RelEntityMove_5 {
                entity_id: i32,
                d_x: FixedPoint8,
                d_y: FixedPoint8,
                d_z: FixedPoint8,
            }
            packet RelEntityMove_47 {
                entity_id: VarInt,
                d_x: FixedPoint8,
                d_y: FixedPoint8,
                d_z: FixedPoint8,
                on_ground: bool,
            }
            packet RelEntityMove_107 {
                entity_id: VarInt,
                d_x: i16,
                d_y: i16,
                d_z: i16,
                on_ground: bool,
            }
            packet RemoveEntityEffect_5 {
                entity_id: i32,
                effect_id: i8,
            }
            packet RemoveEntityEffect_47 {
                entity_id: VarInt,
                effect_id: i8,
            }
            packet ResourcePackSend_47 {
                url: String,
                hash: String,
            }
            packet Respawn_5 {
                dimension: i32,
                difficulty: u8,
                gamemode: u8,
                level_kind: String,
            }
            packet ScoreboardDisplayObjective_5 {
                position: i8,
                name: String,
            }
            packet ScoreboardObjective_5 {
                name: String,
                display_text: String,
                action: i8,
            }
            packet ScoreboardObjective_47 {
                name: String,
                action: i8,
                display_text: Option<String> > when(|p: &ScoreboardObjective_47| p.action == 0 || p.action == 2),
                kind: Option<String> > when(|p: &ScoreboardObjective_47| p.action == 0 || p.action == 2),
            }
            packet ScoreboardScore_5 {
                item_name: String,
                action: i8,
                score_name: Option<String> > when(|p: &ScoreboardScore_5| p.action != 1),
                value: Option<i32> > when(|p: &ScoreboardScore_5| p.action != 1),
            }
            packet ScoreboardScore_47 {
                item_name: String,
                action: VarInt,
                score_name: String,
                value: Option<VarInt> > when(|p: &ScoreboardScore_47| p.action != 1),
            }
            // TODO: Unfinished
            packet ScoreboardTeam_5 {
                team: String,
                mode: i8,
                name: Option<String> > when(|p: &ScoreboardTeam_5| p.mode == 0 || p.mode == 2),
                prefix: Option<String> > when(|p: &ScoreboardTeam_5| p.mode == 0 || p.mode == 2),
                suffix: Option<String> > when(|p: &ScoreboardTeam_5| p.mode == 0 || p.mode == 2),
                friendly_fire: Option<i8> > when(|p: &ScoreboardTeam_5| p.mode == 0 || p.mode == 2),
                // players: Option<['array', {'countType': 'i16', 'type': 'string'}]> > when(|p: &ScoreboardTeam_5| p.mode == 0 || p.mode == 3 || p.mode == 4),
            }
            // TODO: Unfinished
            packet ScoreboardTeam_47 {
                team: String,
                mode: i8,
                name: Option<String> > when(|p: &ScoreboardTeam_47| p.mode == 0 || p.mode == 2),
                prefix: Option<String> > when(|p: &ScoreboardTeam_47| p.mode == 0 || p.mode == 2),
                suffix: Option<String> > when(|p: &ScoreboardTeam_47| p.mode == 0 || p.mode == 2),
                friendly_fire: Option<i8> > when(|p: &ScoreboardTeam_47| p.mode == 0 || p.mode == 2),
                name_tag_visibility: Option<String> > when(|p: &ScoreboardTeam_47| p.mode == 0 || p.mode == 2),
                color: Option<i8> > when(|p: &ScoreboardTeam_47| p.mode == 0 || p.mode == 2),
                // players: Option<['array', {'countType': 'varint', 'type': 'string'}]> > when(|p: &ScoreboardTeam_47| p.mode == 0 || p.mode == 3 || p.mode == 4),
            }
            packet SelectAdvancementTab_335 {
                id: Option<String>,
            }
            packet SetCompression_47 {
                threshold: VarInt,
            }
            packet SetCooldown_107 {
                item_i_d: VarInt,
                cooldown_ticks: VarInt,
            }
            packet SetPassengers_107 {
                entity_id: VarInt,
                passengers: PrefixedVec<VarInt, VarInt>,
            }
            packet SetSlot_5 {
                window_id: i8,
                slot: i16,
                item: Slot,
            }
            packet SoundEffect_107 {
                sound_id: VarInt,
                sound_category: VarInt,
                x: i32,
                y: i32,
                z: i32,
                volume: f32,
                pitch: u8,
            }
            packet SoundEffect_210 {
                sound_id: VarInt,
                sound_category: VarInt,
                x: i32,
                y: i32,
                z: i32,
                volume: f32,
                pitch: f32,
            }
            packet SpawnEntityExperienceOrb_5 {
                entity_id: VarInt,
                x: FixedPoint32,
                y: FixedPoint32,
                z: FixedPoint32,
                count: i16,
            }
            packet SpawnEntityExperienceOrb_107 {
                entity_id: VarInt,
                x: f64,
                y: f64,
                z: f64,
                count: i16,
            }
            packet SpawnEntityLiving_5 {
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
            packet SpawnEntityLiving_107 {
                entity_id: VarInt,
                entity_u_u_i_d: uuid::Uuid,
                kind: u8,
                x: f64,
                y: f64,
                z: f64,
                yaw: i8,
                pitch: i8,
                head_pitch: i8,
                velocity_x: i16,
                velocity_y: i16,
                velocity_z: i16,
                metadata: EntityMeta,
            }
            packet SpawnEntityLiving_315 {
                entity_id: VarInt,
                entity_u_u_i_d: uuid::Uuid,
                kind: VarInt,
                x: f64,
                y: f64,
                z: f64,
                yaw: i8,
                pitch: i8,
                head_pitch: i8,
                velocity_x: i16,
                velocity_y: i16,
                velocity_z: i16,
                metadata: EntityMeta,
            }
            packet SpawnEntityPainting_5 {
                entity_id: VarInt,
                title: String,
                location: PositionIII,
                direction: i32,
            }
            packet SpawnEntityPainting_47 {
                entity_id: VarInt,
                title: String,
                location: Position,
                direction: u8,
            }
            packet SpawnEntityPainting_107 {
                entity_id: VarInt,
                entity_u_u_i_d: uuid::Uuid,
                title: String,
                location: Position,
                direction: u8,
            }
            packet SpawnEntityWeather_5 {
                entity_id: VarInt,
                kind: i8,
                x: i32,
                y: i32,
                z: i32,
            }
            packet SpawnEntityWeather_107 {
                entity_id: VarInt,
                kind: i8,
                x: f64,
                y: f64,
                z: f64,
            }
            // TODO: Unfinished
            packet SpawnEntity_5 {
                entity_id: VarInt,
                kind: i8,
                x: FixedPoint32,
                y: FixedPoint32,
                z: FixedPoint32,
                pitch: i8,
                yaw: i8,
                // object_data: ['container', [{'name': 'intField', 'type': 'i32'}, {'name': 'velocityX', 'type': ['switch', {'compareTo': 'intField', 'fields': {'0': 'void'}, 'default': 'i16'}]}, {'name': 'velocityY', 'type': ['switch', {'compareTo': 'intField', 'fields': {'0': 'void'}, 'default': 'i16'}]}, {'name': 'velocityZ', 'type': ['switch', {'compareTo': 'intField', 'fields': {'0': 'void'}, 'default': 'i16'}]}]],
            }
            packet SpawnEntity_107 {
                entity_id: VarInt,
                object_u_u_i_d: uuid::Uuid,
                kind: i8,
                x: f64,
                y: f64,
                z: f64,
                pitch: i8,
                yaw: i8,
                object_data: i32,
                velocity_x: i16,
                velocity_y: i16,
                velocity_z: i16,
            }
            packet SpawnPosition_5 {
                location: PositionIII,
            }
            packet SpawnPosition_47 {
                location: Position,
            }
            packet Statistics_5 {
                entries: PrefixedVec<StatisticsEntry, VarInt>,
            }
            packet TabComplete_5 {
                matches: PrefixedVec<String, VarInt>,
            }
            packet Teams_107 {
                team: String,
                mode: i8,
                name: Option<String> > when(|p: &Teams_107| p.mode == 0 || p.mode == 2),
                prefix: Option<String> > when(|p: &Teams_107| p.mode == 0 || p.mode == 2),
                suffix: Option<String> > when(|p: &Teams_107| p.mode == 0 || p.mode == 2),
                friendly_fire: Option<i8> > when(|p: &Teams_107| p.mode == 0 || p.mode == 2),
                name_tag_visibility: Option<String> > when(|p: &Teams_107| p.mode == 0 || p.mode == 2),
                collision_rule: Option<String> > when(|p: &Teams_107| p.mode == 0 || p.mode == 2),
                color: Option<i8> > when(|p: &Teams_107| p.mode == 0 || p.mode == 2),
                players: Option<PrefixedVec<String, VarInt>> > when(|p: &Teams_107| p.mode == 0 || p.mode == 3 || p.mode == 4),
            }
            packet TileEntityData_5 {
                location: PositionISI,
                action: u8,
                nbt_data: CompressedGzData<nbt::Blob>,
            }
            packet TileEntityData_47 {
                location: Position,
                action: u8,
                nbt_data: nbt::Blob,
            }
            packet Title_47 {
                action: VarInt,
                text: Option<String> > when(|p: &Title_47| p.action == 0 || p.action == 1),
                fade_in: Option<i32> > when(|p: &Title_47| p.action == 2),
                stay: Option<i32> > when(|p: &Title_47| p.action == 2),
                fade_out: Option<i32> > when(|p: &Title_47| p.action == 2),
            }
            packet Title_315 {
                action: VarInt,
                text: Option<String> > when(|p: &Title_315| p.action == 0 || p.action == 1 || p.action == 2),
                fade_in: Option<i32> > when(|p: &Title_315| p.action == 3),
                stay: Option<i32> > when(|p: &Title_315| p.action == 3),
                fade_out: Option<i32> > when(|p: &Title_315| p.action == 3),
            }
            packet Transaction_5 {
                window_id: u8,
                action: i16,
                accepted: bool,
            }
            packet Transaction_47 {
                window_id: i8,
                action: i16,
                accepted: bool,
            }
            packet UnloadChunk_107 {
                chunk_x: i32,
                chunk_z: i32,
            }
            // TODO: Unfinished
            packet UnlockRecipes_335 {
                action: VarInt,
                crafting_book_open: bool,
                filtering_craftable: bool,
                recipes1: PrefixedVec<VarInt, VarInt>,
                // recipes2: Option<['array', {'countType': 'varint', 'type': 'varint'}]> > when(|p: &UnlockRecipes_335| p.action == 0),
            }
            // TODO: Unfinished
            packet UpdateAttributes_5 {
                entity_id: i32,
                // properties: ['array', {'countType': 'i32', 'type': ['container', [{'name': 'key', 'type': 'string'}, {'name': 'value', 'type': 'f64'}, {'name': 'modifiers', 'type': ['array', {'countType': 'i16', 'type': ['container', [{'name': 'uuid', 'type': 'UUID'}, {'name': 'amount', 'type': 'f64'}, {'name': 'operation', 'type': 'i8'}]]}]}]]}],
            }
            // TODO: Unfinished
            packet UpdateAttributes_47 {
                entity_id: VarInt,
                // properties: ['array', {'countType': 'i32', 'type': ['container', [{'name': 'key', 'type': 'string'}, {'name': 'value', 'type': 'f64'}, {'name': 'modifiers', 'type': ['array', {'countType': 'varint', 'type': ['container', [{'name': 'uuid', 'type': 'UUID'}, {'name': 'amount', 'type': 'f64'}, {'name': 'operation', 'type': 'i8'}]]}]}]]}],
            }
            packet UpdateEntityNbt_47 {
                entity_id: VarInt,
                tag: nbt::Blob,
            }
            packet UpdateHealth_5 {
                health: f32,
                food: i16,
                food_saturation: f32,
            }
            packet UpdateHealth_47 {
                health: f32,
                food: VarInt,
                food_saturation: f32,
            }
            packet UpdateSign_5 {
                location: PositionISI,
                text1: String,
                text2: String,
                text3: String,
                text4: String,
            }
            packet UpdateSign_47 {
                location: Position,
                text1: String,
                text2: String,
                text3: String,
                text4: String,
            }
            packet UpdateTime_5 {
                age: i64,
                time: i64,
            }
            packet VehicleMove_107 {
                x: f64,
                y: f64,
                z: f64,
                yaw: f32,
                pitch: f32,
            }
            packet WindowItems_5 {
                window_id: u8,
                items: PrefixedVec<Slot, i16>,
            }
            packet WorldBorder_47 {
                action: VarInt,
                radius: Option<f64> > when(|p: &WorldBorder_47| p.action == 0),
                x: Option<f64> > when(|p: &WorldBorder_47| p.action == 2 || p.action == 3),
                z: Option<f64> > when(|p: &WorldBorder_47| p.action == 2 || p.action == 3),
                old_radius: Option<f64> > when(|p: &WorldBorder_47| p.action == 1 || p.action == 3),
                new_radius: Option<f64> > when(|p: &WorldBorder_47| p.action == 1 || p.action == 3),
                speed: Option<VarLong> > when(|p: &WorldBorder_47| p.action == 1 || p.action == 3),
                portal_boundary: Option<VarInt> > when(|p: &WorldBorder_47| p.action == 3),
                warning_time: Option<VarInt> > when(|p: &WorldBorder_47| p.action == 3 || p.action == 4),
                warning_blocks: Option<VarInt> > when(|p: &WorldBorder_47| p.action == 3 || p.action == 5),
            }
            packet WorldEvent_5 {
                effect_id: i32,
                location: PositionIBI,
                data: i32,
                global: bool,
            }
            packet WorldEvent_47 {
                effect_id: i32,
                location: Position,
                data: i32,
                global: bool,
            }
            packet WorldParticles_5 {
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
            // TODO: Unfinished
            packet WorldParticles_47 {
                particle_id: i32,
                long_distance: bool,
                x: f32,
                y: f32,
                z: f32,
                offset_x: f32,
                offset_y: f32,
                offset_z: f32,
                particle_data: f32,
                particles: i32,
                // data: ['switch', {'compareTo': 'particleId', 'fields': {'36': ['array', {'count': 2, 'type': 'varint'}], '37': ['array', {'count': 1, 'type': 'varint'}], '38': ['array', {'count': 1, 'type': 'varint'}]}, 'default': 'void'}],
            }
        }
        serverbound {
            packet AbilitiesServerbound_5 {
                flags: i8,
                flying_speed: f32,
                walking_speed: f32,
            }
            packet AdvancementTab_335 {
                action: VarInt,
                tab_id: Option<String> > when(|p: &AdvancementTab_335| p.action == 0),
            }
            packet ArmAnimation_5 {
                entity_id: i32,
                animation: i8,
            }
            packet ArmAnimation_47 {
            }
            packet ArmAnimation_107 {
                hand: VarInt,
            }
            packet BlockDig_5 {
                status: i8,
                location: PositionIBI,
                face: i8,
            }
            packet BlockDig_47 {
                status: VarInt,
                location: Position,
                face: i8,
            }
            packet BlockPlace_5 {
                location: PositionIBI,
                direction: i8,
                held_item: Slot,
                cursor_x: i8,
                cursor_y: i8,
                cursor_z: i8,
            }
            packet BlockPlace_47 {
                location: Position,
                direction: i8,
                held_item: Slot,
                cursor_x: i8,
                cursor_y: i8,
                cursor_z: i8,
            }
            packet BlockPlace_107 {
                location: Position,
                direction: VarInt,
                hand: VarInt,
                cursor_x: i8,
                cursor_y: i8,
                cursor_z: i8,
            }
            packet BlockPlace_315 {
                location: Position,
                direction: VarInt,
                hand: VarInt,
                cursor_x: f32,
                cursor_y: f32,
                cursor_z: f32,
            }
            packet ChatServerbound_5 {
                message: String,
            }
            packet ClientCommand_5 {
                payload: i8,
            }
            packet ClientCommand_47 {
                payload: VarInt,
            }
            packet ClientCommand_107 {
                action_id: VarInt,
            }
            packet CloseWindowServerbound_5 {
                window_id: u8,
            }
            packet CraftRecipeRequest_338 {
                window_id: i8,
                recipe: VarInt,
                make_all: bool,
            }
            // TODO: We can probably merge all of these? I don't see any differences?
            // TODO: Unfinished
            packet CraftingBookData_335 {
                kind: VarInt,
                // _anon0: ['switch', {'compareTo': 'type', 'fields': {'0': ['container', [{'name': 'displayedRecipe', 'type': 'i32'}]], '1': ['container', [{'name': 'craftingBookOpen', 'type': 'bool'}, {'name': 'craftingFilter', 'type': 'bool'}]]}}],
            }
            // TODO: Unfinished
            packet CraftingBookData_338 {
                kind: VarInt,
                // _anon0: ['switch', {'compareTo': 'type', 'fields': {'0': ['container', [{'name': 'displayedRecipe', 'type': 'i32'}]], '1': ['container', [{'name': 'craftingBookOpen', 'type': 'bool'}, {'name': 'craftingFilter', 'type': 'bool'}]]}}],
            }
            // TODO: Unfinished
            packet CraftingBookData_340 {
                kind: VarInt,
                // _anon0: ['switch', {'compareTo': 'type', 'fields': {'0': ['container', [{'name': 'displayedRecipe', 'type': 'i32'}]], '1': ['container', [{'name': 'craftingBookOpen', 'type': 'bool'}, {'name': 'craftingFilter', 'type': 'bool'}]]}}],
            }
            packet CustomPayloadServerbound_5 {
                channel: String,
                data: PrefixedVec<u8, i16>,
            }
            packet CustomPayloadServerbound_47 {
                channel: String,
                data: Vec<u8>,
            }
            packet EnchantItem_5 {
                window_id: i8,
                enchantment: i8,
            }
            packet EntityAction_5 {
                entity_id: i32,
                action_id: i8,
                jump_boost: i32,
            }
            packet EntityAction_47 {
                entity_id: VarInt,
                action_id: VarInt,
                jump_boost: VarInt,
            }
            packet Flying_5 {
                on_ground: bool,
            }
            packet HeldItemSlotServerbound_5 {
                slot_id: i16,
            }
            packet KeepAliveServerbound_5 {
                keep_alive_id: i32,
            }
            packet KeepAliveServerbound_47 {
                keep_alive_id: VarInt,
            }
            packet KeepAliveServerbound_340 {
                keep_alive_id: i64,
            }
            packet Look_5 {
                yaw: f32,
                pitch: f32,
                on_ground: bool,
            }
            packet PositionLook_5 {
                x: f64,
                stance: f64,
                y: f64,
                z: f64,
                yaw: f32,
                pitch: f32,
                on_ground: bool,
            }
            packet PositionLook_47 {
                x: f64,
                y: f64,
                z: f64,
                yaw: f32,
                pitch: f32,
                on_ground: bool,
            }
            packet PositionServerbound_5 {
                x: f64,
                stance: f64,
                y: f64,
                z: f64,
                on_ground: bool,
            }
            packet PositionServerbound_47 {
                x: f64,
                y: f64,
                z: f64,
                on_ground: bool,
            }
            // TODO: Unfinished
            packet PrepareCraftingGrid_335 {
                window_id: u8,
                action_number: u16,
                // return_entry: ['array', {'countType': 'u16', 'type': ['container', [{'name': 'item', 'type': 'slot'}, {'name': 'craftingSlot', 'type': 'u8'}, {'name': 'playerSlot', 'type': 'u8'}]]}],
                // prepare_entry: ['array', {'countType': 'u16', 'type': ['container', [{'name': 'item', 'type': 'slot'}, {'name': 'craftingSlot', 'type': 'u8'}, {'name': 'playerSlot', 'type': 'u8'}]]}],
            }
            packet ResourcePackReceive_47 {
                hash: String,
                result: VarInt,
            }
            packet ResourcePackReceive_210 {
                result: VarInt,
            }
            packet SetCreativeSlot_5 {
                slot: i16,
                item: Slot,
            }
            packet Settings_5 {
                locale: String,
                view_distance: i8,
                chat_flags: i8,
                chat_colors: bool,
                difficulty: u8,
                show_cape: bool,
            }
            packet Settings_47 {
                locale: String,
                view_distance: i8,
                chat_flags: i8,
                chat_colors: bool,
                skin_parts: u8,
            }
            packet Settings_107 {
                locale: String,
                view_distance: i8,
                chat_flags: VarInt,
                chat_colors: bool,
                skin_parts: u8,
                main_hand: VarInt,
            }
            packet Spectate_47 {
                target: uuid::Uuid,
            }
            packet SteerBoat_107 {
                left_paddle: bool,
                right_paddle: bool,
            }
            packet SteerVehicle_5 {
                sideways: f32,
                forward: f32,
                jump: bool,
                unmount: bool,
            }
            packet SteerVehicle_47 {
                sideways: f32,
                forward: f32,
                jump: u8,
            }
            packet TabCompleteServerbound_5 {
                text: String,
            }
            packet TabCompleteServerbound_47 {
                text: String,
                block: Option<Position>,
            }
            packet TabComplete_107 {
                text: String,
                assume_command: bool,
                looked_at_block: Option<Position>,
            }
            packet TeleportConfirm_107 {
                teleport_id: VarInt,
            }
            packet TransactionServerbound_5 {
                window_id: i8,
                action: i16,
                accepted: bool,
            }
            packet UpdateSignServerbound_5 {
                location: PositionISI,
                text1: String,
                text2: String,
                text3: String,
                text4: String,
            }
            packet UpdateSignServerbound_47 {
                location: Position,
                text1: String,
                text2: String,
                text3: String,
                text4: String,
            }
            packet UseEntity_5 {
                target: i32,
                mouse: i8,
                x: Option<f32> > when(|p: &UseEntity_5| p.mouse == 2),
                y: Option<f32> > when(|p: &UseEntity_5| p.mouse == 2),
                z: Option<f32> > when(|p: &UseEntity_5| p.mouse == 2),
            }
            packet UseEntity_47 {
                target: VarInt,
                mouse: VarInt,
                x: Option<f32> > when(|p: &UseEntity_47| p.mouse == 2),
                y: Option<f32> > when(|p: &UseEntity_47| p.mouse == 2),
                z: Option<f32> > when(|p: &UseEntity_47| p.mouse == 2),
            }
            packet UseEntity_107 {
                target: VarInt,
                mouse: VarInt,
                x: Option<f32> > when(|p: &UseEntity_107| p.mouse == 2),
                y: Option<f32> > when(|p: &UseEntity_107| p.mouse == 2),
                z: Option<f32> > when(|p: &UseEntity_107| p.mouse == 2),
                hand: Option<VarInt> > when(|p: &UseEntity_107| p.mouse == 0 || p.mouse == 2),
            }
            packet UseItem_107 {
                hand: VarInt,
            }
            packet VehicleMoveServerbound_107 {
                x: f64,
                y: f64,
                z: f64,
                yaw: f32,
                pitch: f32,
            }
            packet WindowClick_5 {
                window_id: i8,
                slot: i16,
                mouse_button: i8,
                action: i16,
                mode: i8,
                item: Slot,
            }
            packet WindowClick_47 {
                window_id: u8,
                slot: i16,
                mouse_button: i8,
                action: i16,
                mode: i8,
                item: Slot,
            }
        }
    }
}
