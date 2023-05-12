# spotify-jbl-remote

Turn your JBL speaker into a remote for Spotify!

Developed and tested with JBL GO nearby. Can't gurantee all JBL speakers support.

### What the hell is this?

This is a userspace "driver" for JBL GO to properly interact with Spotify Desktop on linux.

### Requirements

Please note this is linux-only.

At some point I'll publish pre-built executables, but for now you have to build it yourself, therefore you need `cargo` installed on your system.

### Build

```bash
$ cargo build --release
$ ls target/release/spotify-jbl-remote
```

### Why?

1. The fact that for some reason JBL GO's signals have no effect on spotify is kinda frustrating.
2. Actually useful pet project.
3. To learn Rust
4. To learn more about input events in linux

### TODO:
- [x] Autosearch for JBL input event dev file
- [x] Comments
- [x] Basic spotify support (pause/unpause/next track)
- [x] Option to use PlayPause interface instead of Play+Pause
- [x] Poll mode
- [ ] Way to run it in the background
- [x] Build guide
- [ ] Privileges setup guide
- [ ] Usage guide
- [ ] Other JBL models support
