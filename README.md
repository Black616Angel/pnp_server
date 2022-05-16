# Pens and Crustaceans

Pens and Crustaceans is a FOSS, self-hostable virtual tabletop.
It aims at being a complete package with frontend and backend.

You can host it on your own server and play online with your friends.

One of the maini goals is the use of files for all data that has to be saved or transmitted between players.
Thus the server can be backed up easily and the files can be used in other games etc.

Frontend and backend are written mainly in rust.

The frontend uses [macroquad](https://github.com/not-fl3/macroquad/) for most of the visual stuff and the backend uses [tide](https://github.com/http-rs/tide/) to privide the server.


## Goals and milestones
- [ ] Scenes
    - [X] Simple Background and usage of tokens
    - [ ] Layers
- [ ] Multiplayer
    - [X] hacky, barebones
    - [ ] login-stuff
    - [ ] rights
- [ ] Chat and dice rolls
    - [ ] simple textchat
    - [ ] roll-shortcuts
    - [ ] Statsheet
- [ ] Ruleset and scripts for SRD

### Far future (no priority)
- lua-Scripting
- Fog of War
- Drawing on the Scene