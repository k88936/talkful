# Talkful

a lightweight offline voice input method, made out of hobby and the target of put rust into practice.

## Quick Start
```shell
# build
pnpm install
tauri build --no-bundle
# install
cp src-tauri/target/release/talkful ~/.cargo/bin/

```

## Tech stack
* UI/UX: tauri + react + ringui
* voice record: cpal
* asr: sherpa-onnx + paraformer (model)
* text inject: enigo



##
```shell
ln -s pathcch.lib ~/.cache/cargo-xwin/xwin/sdk/lib/um/x86_64/PathCch.lib
```