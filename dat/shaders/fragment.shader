# version 330 core

in vec2 TexCoord;

out vec4 FragColor;

// LEARN: Read up on uniforms again...
uniform sampler2D ourTexture;
uniform bool isText;
uniform vec3 textColor;

void main()
{
    if (!isText) {
        FragColor = texture(ourTexture, TexCoord);
    } else {
        vec4 sampled = vec4(1.0, 1.0, 1.0, texture(ourTexture, TexCoord).r);
        FragColor = vec4(textColor, 1.0) * sampled;
    }
} 