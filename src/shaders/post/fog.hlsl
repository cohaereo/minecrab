#include "include/colormap.hlsl"

// nether: float3(0.145, 0.024, 0.024)
// overworld: float3(0.529, 0.667, 1)
struct VertexInput {
    uint vertex_index : SV_VertexID;
};

struct VertexOutput {
    float4 clip_position : SV_Position;
    float2 uv;
};

// struct PushConstants {
//   float3 position;
// };
// [[vk::push_constant]] ConstantBuffer<PushConstants> pc;

[[vk::binding(0, 0)]] Texture2D<float>    depth_texture;
[[vk::binding(1, 0)]] SamplerState depth_sampler;
// [[vk::binding(2, 0)]] SamplerState depth_sampler_;

const static float2 VERTICES[] = {
    float2(0, 0),
    float2(0, 1),
    float2(1, 0),
    float2(1, 1),
};

VertexOutput vs_main(uint vi : SV_VertexID) : SV_Position {
    VertexOutput output;
    // float x = float(1 - (vi >> 1u));
    // float y = float((vi & 1u));
    float2 pos = VERTICES[vi];
    output.clip_position = float4(pos * 2 - float2(1), 0.0, 1.0);
    output.uv = float2(pos.x, 1-pos.y);

    return output;
}

#define NEAR 0.1f
#define FAR 1000.0f

float linearizeDepth(float d) {
    return (2 * NEAR * FAR) / (FAR + NEAR - (d * 2 - 1) * (FAR - NEAR));
}

float logisticDepth(float depth, float steepness=0.5, float offset=5) {
    float z = linearizeDepth(depth);
    return (1 / (1  + exp(-steepness * (z - offset))));
}


float4 fs_main(VertexOutput input) : SV_Target0 {
    // float near = 0.1;
    // float far = 1000.0;
    float depth = depth_texture.Sample(depth_sampler, input.uv);
    // float dn = (depth - 0.98) * 50;
    // float dist = 4000;
    // // uint4 tdims;
    // // depth_texture.GetDimensions(tdims.x, tdims.y);
    // // float depth = depth_texture.Load(int3(0, 0, 0));
    // float depth = input.clip_position.x;
    // float dn = (depth - 0.98) * 50;
    // float dn = depth;
    // depth = 2.0 * depth - 1.0;
    // // float r = (2.0 * near) / (far + near - depth * (far - near));
    // float r = depth * far;
    // // float dn = (depth - 0.8061) * 5.15;
    // // float r = (depth * far);
    // if(r < dist)
    //     return float4(float3(dn), 1);
    // else
    //     return float4(dn, 0, 0, 1);
    // return float4(float3(0.529, 0.667, 1), logisticDepth(depth, 0.1, 16*8));
    return float4(GammaToLinear(float3(0.145, 0.024, 0.024)), logisticDepth(depth, 0.1, 16*8));
    // discard;
    // return float4(0);

    // return float4(input.clip_position);
}
