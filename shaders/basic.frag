#version 330 core

out vec4 FragColor;
uniform float mixValue;

void main() {
    vec4 red = vec4(1.0, 0.0, 0.0, 1.0);
    vec4 green = vec4(0.0, 1.0, 0.0, 1.0);
    FragColor = mix(red, green, mixValue);
}
