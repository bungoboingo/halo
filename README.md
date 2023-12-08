# Halo
A program for editing `.wgsl` shaders, written in [Iced](https://github.com/iced-rs/iced).

Halo is currently in very early WIP. Basic functionality is complete, e.g. you can edit shaders and immediately see the 
results, but it is missing a lot of QOL features.

# 0.1 Roadmap
### Easy
- [ ] Support normal editor hotkeys e.g. undo/redo, open, tab, etc.
- [ ] Custom Halo iced theme
- [ ] Editor show/hide toggle
### Medium
- [ ] WGSL syntax highlighting
- [ ] Inline error messages w/ tooltip
- [ ] Create a more Halo-like default shader

# 0.2
### Easy
- [ ] Export to other shader languages
- [ ] Minimizer support for those who crave min char count
- [ ] Editor popout to new window
- [ ] GLSL support
- [ ] Shadertoy Integration

### Medium
- [ ] More advanced editor actions e.g. go-to, search, etc.
- [ ] Support custom texture sampling in uniforms (e.g. ShaderToy's "channel"s)

### Hard
- [ ] Basic WGSL formatter w/ hotkey

# 0.3.. and beyond?
- [ ] Upload/download somewhere, maybe Nostr
- [ ] Compute shader support
- [ ] Built in video export to webm or mp4
