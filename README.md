### Exponse your server publicly throuhg Ngrok run
```bash
cargo run -- --NGROK_AUTHTOKEN <TOKEN_FROM_NGROK_DASHBOARD>
```
[https://dashboard.ngrok](https://dashboard.ngrok.com/get-started/your-authtoken)

[Ngrok -rs DOCS](https://ngrok.com/docs/using-ngrok-with/rust/)

---
#### Running this initially on windows i encountered this error

[Initial error](https://stackoverflow.com/questions/72461117/error-failed-to-run-custom-build-command-for-ring-v0-16-20)

[Github issue](https://github.com/japaric/rust-cross/issues/33)

#### Fix for windows 'musl-gcc' missing
To fix this on windows machine i had to nightly toolchain 
```bash
# Show installed/active toolchains
1. rustup show 
# If not already present
2. rustup toolchain install nightly 
# Set default
3. rustup default nightly-x86_64-pc-windows-msvc
```