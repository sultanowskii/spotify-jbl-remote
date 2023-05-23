# spotify-jbl-remote

Turn your JBL speaker into a remote for Spotify!

Developed and tested with JBL GO nearby. Can't gurantee all JBL speakers support.

Sooner or later will be able to test it on JBL Clip 2.

## What the hell is this?

This is a userspace "driver" for JBL to properly interact with Spotify Desktop on linux.

## Requirements

Please note this is linux-only.

At some point I'll publish pre-built executables, but for now you have to build it yourself, therefore you need `cargo` installed on your system.

## Build

```bash
$ make build
$ ls target/release/spotify-jbl-remote
```

## Install (after build)

1. Puts executable at /usr/bin/ so it is available all across system
2. Adds udev rule for JBL (so you don't have to have asspain with privileges and groups).

```bash
$ sudo make install
```

## Usage

1. Run Spotfiy Desktop
2. Run `spotify-jbl-remote`:
   ```bash
   $ ./spotify-jbl-remote
   ```
3. And now you can control spotify by **Action** button on your jbl speaker!

Default event handle 'driver' mapping:
- Press #1: Play
- Press #2: Pause
- Double press: Next track

There are other drivers available. Personally, I like `play-pause` most.

## Background process/daemon when???

For now I can't find an ultimate solution that fits everyone. `supervisor` seems like a good option, but it is not installed by default.

## Why?

1. For some reason JBL GO's signals have no effect on spotify client on linux.
2. Actually useful pet project.
3. To learn Rust
4. To learn more about input events in linux

## TODO:
- [x] Autosearch for JBL input event dev file
- [x] Comments
- [x] Basic spotify support (pause/unpause/next track)
- [x] Option to use PlayPause interface instead of Play+Pause
- [x] Poll mode
- [ ] Way to run it in the background
- [x] Build guide
- [x] Privileges setup guide
- [ ] Invalid file proper handling
- [x] Usage guide
- [x] Other JBL models support
