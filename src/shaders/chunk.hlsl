#include "include/colormap.hlsl"

struct VertexOutput {
  float4 clip_position : SV_Position;
  float2 uv;
  float3 color;
  float dist;
};

struct PushConstants {
  int3 chunk_coordinates;
  uint render_distance;
  float3 camera_pos;
};
[[vk::push_constant]] ConstantBuffer<PushConstants> pc;

cbuffer CameraUniform { row_major float4x4 view_proj; };

[[vk::binding(0, 1)]] Texture2D<float4> atlas_texture;
[[vk::binding(1, 1)]] SamplerState atlas_sampler;

static float2 UV_INDEXMAP[4] = {
    float2(1, 1),
    float2(1, 0),
    float2(0, 0),
    float2(0, 1),
};

VertexOutput vs_main(uint vertex_index : SV_VertexID, uint data) : SV_Position {
  float3 vertex_position =
      float3(float((data >> 0u) & 0x1fu), float((data >> 5u) & 0x1fu),
             float((data >> 10u) & 0x1fu));

  data >>= 15;

  uint colormap_offset = data & 0xffu;
  uint side = (data >> 8u) & 0x7u;
  uint ao = (data >> 12u) & 0x3u;
  uint light = (data >> 14u) & 0xfu;

  float3 pcc = float3(pc.chunk_coordinates * 16);

  VertexOutput output;
  float3 pos = vertex_position + pcc;
  output.clip_position = view_proj * float4(pos, 1);
  output.color = float3(1, 1, 1);
  output.dist = distance(pc.camera_pos, pos);

  if (side == 1u) {
    output.color *= 0.3;
  }
  if (side == 2u) {
    output.color *= 0.7;
  }
  if (side == 3u) {
    output.color *= 0.8;
  }
  if (side == 5u) {
    output.color *= 0.5;
  }

  // output.color *= 0.5; // Nether/end ambient light?

  float ao_mul = max(0.0, min(1.0 - float(3u - ao) * 0.25, 1));
  output.color *= ao_mul;

  float2 uv_offset =
      float2(colormap_offset % 16, floor(colormap_offset / 16)) / 16;
  output.uv = UV_INDEXMAP[vertex_index % 4] / 16 + uv_offset;

  return output;
}

float linearFog(float z, float start, float end) {
  return 1.0 - clamp((end - z) / (end - start), 0.0, 1.0);
}

float4 fs_main(VertexOutput input) : SV_Target0 {
  float4 c = float4(input.color * atlas_texture.Sample(atlas_sampler, input.uv).rgb, 1);

  float distance = 16*max((pc.render_distance), 2);

  // Nether
  // float4 fogc = float4(0.20, 0.031, 0.031, 1);
  // float fog_amount = linearFog(input.dist, 0, distance);

  // Overworld
  float4 fogc = float4(0.529, 0.667, 1, 1);
  float fog_amount = linearFog(input.dist, distance * 0.9, distance);

  return lerp(c, fogc, fog_amount);
}
