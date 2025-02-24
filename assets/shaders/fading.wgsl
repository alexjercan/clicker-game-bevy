#import bevy_pbr::forward_io::VertexOutput


@group(2) @binding(0) var<uniform> color: vec4<f32>;

@fragment
fn fragment(
    @location(0) local_pos: vec3<f32>
) -> @location(0) vec4<f32> {
    let alpha = smoothstep(-0.5, 0.5, local_pos.y);
    let color = vec4<f32>(color.xyz, alpha);
    return color;
}
