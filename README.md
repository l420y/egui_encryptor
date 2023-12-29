## Egui-rs Encryptor
**Egui_encryptor is a Rust project that provides a simple and secure way to encrypt and decrypt file-data using the egui library.**

![alt text](https://cdn.discordapp.com/attachments/774181902336000013/1190259699526938684/crop-image-online.com_1703850399_imagepng_q3fiKa5C.png?ex=65a126ba&is=658eb1ba&hm=1f98821f1cdf298b8faa4872c2fa1d45ed2238018e6b8bc3c79275b66495e254&)
##

### About
My native app was created for educational purposes (school project), but has all the necessary functionality to change (encrypt\decrypt) any file data.

At the moment, there are such cipher modes as:
- Xor 
- Binary
- Base64

File utilities:
- save file as new
- make it readonly (win)

### How to use
There is a pre-release 0.1.5 version build - [here](https://github.com/l420y/egui_encryptor/releases/tag/version-0.1.5) 

Also you can run it from source:
1) Clone repo to your computer
2) in cmd: ```cd ../egui_encryptor```
3) in cmd: ```cargo run --release```

### Dependencies and libraries
```rust
[dependencies]
eframe = "0.24.1"
rfd = "0.12.1"
ascii_converter = "0.3.0"
base64 = "0.21.5"
```



