# version 330 core

in vec2 TexCoord;

out vec4 FragColor;

// LEARN: Read up on uniforms again...
uniform sampler2D ourTexture;
uniform bool isText;
uniform vec3 textColor;

uniform vec4 bounding_box;

void main()
{
    if (!isText) {
        vec2 textureDimensions = textureSize(ourTexture, 0);

        float x_scale = (bounding_box.x / textureDimensions.x) + (TexCoord.x * (bounding_box.z / textureDimensions.x));
        float y_scale = (bounding_box.y / textureDimensions.y) + TexCoord.y * (bounding_box.w / textureDimensions.y);

        vec2 new_scale = vec2(x_scale, y_scale);

        FragColor = texture(ourTexture, new_scale);
    } else {
        vec4 sampled = vec4(1.0, 1.0, 1.0, texture(ourTexture, TexCoord).r);
        FragColor = vec4(textColor, 1.0) * sampled;
    }
}