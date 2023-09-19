### Expose your server publicly through Ngrok run
```bash
cargo run -- --NGROK_AUTHTOKEN <AUTH_TOKEN_FROM_NGROK_DASHBOARD>
```
#### Alternative for windows / powershell
```powershell
#set session long env variable
$env:NGROK_AUTHTOKEN = "AUTH_TOKEN"
cargo run 
```
#### Docs
[https://dashboard.ngrok](https://dashboard.ngrok.com/get-started/your-authtoken)

[Ngrok -rs DOCS](https://ngrok.com/docs/using-ngrok-with/rust/)

---
#### When Running this initially on windows i encountered this error

[Initial error](https://stackoverflow.com/questions/72461117/error-failed-to-run-custom-build-command-for-ring-v0-16-20)

[Github issue](https://github.com/japaric/rust-cross/issues/33)

#### Fix for windows 'musl-gcc' missing
To fix this on windows machine I had to nightly toolchain 

### 1. Show installed/active toolchains
```bash
rustup show
```
### 2. If not already present
```bash
rustup toolchain install nightly
```
### 3. Set default
```bash
rustup default nightly-x86_64-pc-windows-msvc
```
