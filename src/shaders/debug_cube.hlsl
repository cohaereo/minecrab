#include "include/colormap.hlsl"

struct VertexInput {
  uint vertex_index : SV_VertexID;
  float3 position : SV_Position;
};

struct VertexOutput {
  float4 clip_position : SV_Position;
  float2 uv;
  float3 color;
};

struct PushConstants {
  float3 position;
};
[[vk::push_constant]] ConstantBuffer<PushConstants> pc;

cbuffer CameraUniform { row_major float4x4 view_proj; };

[[vk::binding(0, 1)]] Texture2D d_texture;
[[vk::binding(1, 1)]] SamplerState d_sampler;

// FIXME: Why do we need to flip the UVs?
static float2 UV_INDEXMAP[4] = {
    float2(1 - 0, 1),
    float2(1 - 1, 1),
    float2(1 - 0, 0),
    float2(1 - 1, 0),
};

VertexOutput vs_main(VertexInput input) : SV_Position {
  VertexOutput output;
  output.clip_position = view_proj * float4(input.position + pc.position, 1);
  output.color = float3(1, 0, 1);

  output.uv = UV_INDEXMAP[input.vertex_index % 4];

  return output;
}

float4 fs_main(VertexOutput input) : SV_Target0 {
  return float4(input.color * d_texture.Sample(d_sampler, input.uv).rgb, 1);
}
