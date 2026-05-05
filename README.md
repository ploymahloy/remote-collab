# remote-collab (TODO: Name project 😆)

**Low-latency remote collaboration for audio engineers ranging from bedroom dorm rooms to award-winning recording studios.**

The goal is to deliver something in the spirit of [Evercast](https://www.evercast.us/)—real-time review and collaboration—but tuned for home studios and small rooms: reliable audio routing, video that stays in sync with what you hear, and an experience that feels like you are in the same room.

## North star

| Target                             | Why it matters                                                                                                                            |
| ---------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| **End-to-end latency under 40 ms** | Musicians and engineers perceive delay quickly; staying under ~40 ms keeps talkback, playback, and reactions feeling natural.             |
| **Seamless AV**                    | Video and audio must stay locked together—no distracting drift, stutter, or “TV delay” lip sync when critiquing a mix or tracking a take. |

Everything in this repo—transport choices, buffering, clocking, and UI—should be judged against those two constraints.

## What this is (today)

The project is early. The current binary uses [cpal](https://github.com/RustAudio/cpal) to enumerate audio hosts and input/output devices on your machine—a foundation for building capture, monitoring, and streaming paths.

## Stack

- **Tauri** — desktop shell and native layer for the client in `[client/](client/)` (`[client/src-tauri/](client/src-tauri/)`).
- **Rust** — Tauri host process and commands, plus the audio/server crate in `[server/](server/)` for device enumeration.
- **React + Vite + TypeScript** - client UI that runs in the Tauri webview.
