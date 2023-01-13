# 🦀 MineCrab/Nautilus

MineCrab is a Minecraft client written in Rust+WGPU, capable of connecting to official servers, or [any other compatible server](https://wiki.vg/Server_List)

```diff
- ! Code quality is abysmal as of right now, restructuring is in progress
```

![Screenshot](screenshot.png)

## Supported versions

MineCrab currently only supports 1.7.10, but I'm aiming to support every major version from 1.7 to the latest version (which is 1.19.2 as of writing this).

We use an abstraction system that makes it easy to implement new protocol versions (or even adapt to other protocols, such as the bedrock protocol)

Every version has 3 degrees of support:

- Parse: all packets are able to be decoded/encoded
- Basic: enough of the abstraction layer is implemented for movement, chunk loading and (basic) entities
- Mapped: all abstraction layer packets are covered by the implemented packets
- Full: every packet for this version is covered by the abstraction layer

| Version | Parse | Basic | Mapped | Full |
| ------- | ----- | ----- | ------ | ---- |
| 1.7     | ✅    | ✅    | 🚧     | ❌   |
| 1.8     | 🚧    | ✅    | 🚧     | ❌   |
| 1.9     | 🚧    | ❌    | ❌     | ❌   |
| 1.10    | 🚧    | ❌    | ❌     | ❌   |
| 1.11    | 🚧    | ❌    | ❌     | ❌   |
| 1.12    | 🚧    | ❌    | ❌     | ❌   |
| 1.13    | ❌    | ❌    | ❌     | ❌   |
| 1.15    | ❌    | ❌    | ❌     | ❌   |
| 1.16    | ❌    | ❌    | ❌     | ❌   |
| 1.16    | ❌    | ❌    | ❌     | ❌   |
| 1.17    | ❌    | ❌    | ❌     | ❌   |
| 1.18    | ❌    | ❌    | ❌     | ❌   |
| 1.19    | ❌    | ❌    | ❌     | ❌   |

## Features

- [x] Networking
  - [x] Multi-protocol abstraction system
  - [ ] Compression
  - [ ] Encryption (online-mode)
- [x] Rendering
  - [x] Basic rendering
  - [ ] Occlusion culling
  - [x] Frustum culling
  - [ ] Anti-aliasing
  - [ ] Biomes
  - [ ] Chunk animations
  - [ ] Non-cube models (torches, plants, etc)
  - [ ] Entity models
  - [ ] Texture animations
  - [ ] GUI
  - [ ] Optifine/Sodium shader support
  - [ ] Resource pack support
- [ ] GUI
  - [ ] Survival inventory
  - [ ] Creative inventory
  - [ ] Furnace
  - [ ] Crafting table
  - [ ] Enchanting table
  - [ ] Anvil
  - [ ] Sign editor
  - [ ] Chat
- [x] Sound
  - [x] Serverside sound
  - [ ] Clientside sound
- [ ] Plugin system
  - [ ] Basics
  - [ ] Example plugins
    - [ ] Nether ceiling tool
    - [ ] X-ray
- [ ] Input
  - [ ] Mouse input
  - [ ] Input settings

## Credits

- Mojang for making an amazing game
- [wiki.vg](https://wiki.vg/Main_Page) for the protocol documentation
