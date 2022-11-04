#include "include/colormap.hlsl"

struct VertexOutput {
  float4 clip_position : SV_Position;
  float3 color;
};

struct PushConstants {
  int3 chunk_coordinates;
};
[[vk::push_constant]] ConstantBuffer<PushConstants> pc;

cbuffer CameraUniform { row_major float4x4 view_proj; };

VertexOutput vs_main(float3 position, float3 color) : SV_Position {
  VertexOutput output;

  float3 pcc =
      float3(pc.chunk_coordinates.x * 16, 0, pc.chunk_coordinates.y * 16);
  output.clip_position = view_proj * float4(position + pcc, 1);
  output.color = GammaToLinear(color);

  return output;
}

float4 fs_main(VertexOutput input) : SV_Target0 {
  return float4(input.color, 1);
}