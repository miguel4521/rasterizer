struct PixelColorBuffer {
    data: array<u32>,
};

struct PixelDepthBuffer {
    depth: array<f32>,
};

struct Vertex { x: f32, y: f32, z: f32 }

struct VertexBuffer {
  values: array<Vertex>,
}

struct Uniform {
  width: f32,
  height: f32,
}

struct Camera {
  view_pos: vec4<f32>,
  view_proj: mat4x4<f32>,
}

@group(0) @binding(0) var<storage, read_write> pixel_color_buffer: PixelColorBuffer;
@group(1) @binding(0) var<storage, read_write> pixel_depth_buffer: PixelDepthBuffer;
@group(2) @binding(0) var<uniform> screen_dims : Uniform;
@group(3) @binding(0) var<storage, read> vertex_buffer : VertexBuffer;
@group(4) @binding(0) var<uniform> camera : Camera;

@compute @workgroup_size(256, 1)
fn raster(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let triangleIndex = global_id.x; // Each thread works on a single triangle
    let numTriangles = arrayLength(&vertex_buffer.values) / 3u;

    if (triangleIndex >= numTriangles) {
        return;
    }

    let i = triangleIndex * 3u;
    let v0 = vertex_buffer.values[i];
    let v1 = vertex_buffer.values[i + 1u];
    let v2 = vertex_buffer.values[i + 2u];

    // Transform vertices to clip space
    let clipV0 = camera.view_proj * vec4(v0.x, v0.y, v0.z, 1.0);
    let clipV1 = camera.view_proj * vec4(v1.x, v1.y, v1.z, 1.0);
    let clipV2 = camera.view_proj * vec4(v2.x, v2.y, v2.z, 1.0);

    // Transform to screen space
    let screenV0 = vec3((clipV0.x * 0.5 + 0.5) * screen_dims.width, (clipV0.y * -0.5 + 0.5) * screen_dims.height, clipV0.z);
    let screenV1 = vec3((clipV1.x * 0.5 + 0.5) * screen_dims.width, (clipV1.y * -0.5 + 0.5) * screen_dims.height, clipV1.z);
    let screenV2 = vec3((clipV2.x * 0.5 + 0.5) * screen_dims.width, (clipV2.y * -0.5 + 0.5) * screen_dims.height, clipV2.z);

    // Calculate bounding box for the triangle
    let bboxMin = min(min(screenV0.xy, screenV1.xy), screenV2.xy);
    let bboxMax = max(max(screenV0.xy, screenV1.xy), screenV2.xy);

    // Iterate over pixels within the bounding box
    for (var px = max(floor(bboxMin.x), 0.0); px <= min(ceil(bboxMax.x), screen_dims.width - 1.0); px += 1.0) {
        for (var py = max(floor(bboxMin.y), 0.0); py <= min(ceil(bboxMax.y), screen_dims.height - 1.0); py += 1.0) {
            let bcScreen = barycentric(screenV0, screenV1, screenV2, vec2(px, py));
            if (bcScreen.x >= 0.0 && bcScreen.y >= 0.0 && bcScreen.z >= 0.0) {
                let pixelIndex = u32(px) + u32(py) * u32(screen_dims.width);
                let z = bcScreen.x * screenV0.z + bcScreen.y * screenV1.z + bcScreen.z * screenV2.z;

                // Depth test
                if (pixel_depth_buffer.depth[pixelIndex] > z) {
                    pixel_depth_buffer.depth[pixelIndex] = z;
                    let color = rgb(u32(255.0 * bcScreen.x), u32(255.0 * bcScreen.y), u32(255.0 * bcScreen.z));
                    pixel_color_buffer.data[pixelIndex] = color;
                }
            }
        }
    }
}

fn rgb(r: u32, g: u32, b: u32) -> u32 {
    return (r << 16) | (g << 8) | b;
}

fn barycentric(p0: vec3<f32>, p1: vec3<f32>, p2: vec3<f32>, p: vec2<f32>) -> vec3<f32> {
    let v0 = p1.xy - p0.xy;
    let v1 = p2.xy - p0.xy;
    let v2 = p - p0.xy;
    let d00 = dot(v0, v0);
    let d01 = dot(v0, v1);
    let d11 = dot(v1, v1);
    let d20 = dot(v2, v0);
    let d21 = dot(v2, v1);
    let denom = d00 * d11 - d01 * d01;
    if (denom == 0.0) {
        return vec3<f32>(-1.0, -1.0, -1.0); // Degenerate triangle
    }
    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;
    return vec3<f32>(u, v, w);
}


@compute @workgroup_size(256, 1)
fn clear(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    let width = u32(screen_dims.width);
    let height = u32(screen_dims.height);
    let total_pixels = width * height;

    if (idx >= total_pixels) {
        return;
    }

    let x = idx % width;
    let y = idx / width;

    // Set color to a default value (e.g., black)
    pixel_color_buffer.data[idx] = rgb(255u, 255u, 255u);

    // Set depth to maximum (1.0)
    pixel_depth_buffer.depth[idx] = 1.0;
}