# version 330 core

in vec2 TexCoord;

out vec4 FragColor;

// LEARN: Read up on uniforms again...
uniform sampler2D ourTexture;
uniform bool isText;
uniform vec3 textColor;

uniform vec4 bounding_box;

const float width = 0.49;
const float edge = 0.041;

void main()
{
    if (!isText) {
        vec2 textureDimensions = textureSize(ourTexture, 0);

        float x_scale = (bounding_box.x / textureDimensions.x) + (TexCoord.x * (bounding_box.z / textureDimensions.x));
        float y_scale = (bounding_box.y / textureDimensions.y) + TexCoord.y * (bounding_box.w / textureDimensions.y);

        vec2 new_scale = vec2(x_scale, y_scale);

        FragColor = texture(ourTexture, new_scale);
    } else {
        vec2 textureDimensions = textureSize(ourTexture, 0);

        float x_scale = (bounding_box.x / textureDimensions.x) + (TexCoord.x * (bounding_box.z / textureDimensions.x));
        float y_scale = (bounding_box.y / textureDimensions.y) + TexCoord.y * (bounding_box.w / textureDimensions.y);

        vec2 new_scale = vec2(x_scale, y_scale);

        float distance = 1.0 - texture(ourTexture, new_scale).a;

        float alpha_v = 1.0 - smoothstep(width, width + edge, distance);

        FragColor = vec4(0.0, 0.0, 0.0, alpha_v);
    }
}