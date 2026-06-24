pub(crate) const HEAT_VERTEX: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying lowp vec2 uv;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
}
"#;

pub(crate) const HEAT_FRAGMENT: &str = r#"#version 100
precision mediump float;
varying vec2 uv;
uniform sampler2D Texture;
uniform float time;
uniform float intensity;
void main() {
    float w1 = sin(uv.y * 50.0 + time * 2.0) * sin(uv.x * 30.0 + time * 1.5);
    float w2 = sin(uv.y * 25.0 - time * 3.0) * cos(uv.x * 40.0 + time * 1.0);
    float dx = (w1 * 0.003 + w2 * 0.0025) * intensity;
    float dy = (w1 * 0.0025 + w2 * 0.002) * intensity;
    vec2 d = uv + vec2(dx, dy);
    gl_FragColor = texture2D(Texture, d);
}
"#;
