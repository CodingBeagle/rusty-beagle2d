# version 330 core
out vec4 FragColor;

in vec2 TexCoord;

// Read up on uniforms again...
uniform sampler2D ourTexture;

void main()
{
    FragColor = texture(ourTexture, TexCoord);
} 