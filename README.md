## Egui-rs Encryptor
Egui_encryptor is a Rust project that provides a simple and secure way to encrypt and decrypt file-data using the egui library.

![alt text](https://cdn.discordapp.com/attachments/1176201554211119186/1190243807233593404/image.png?ex=65a117ed&is=658ea2ed&hm=f66187ddda9da1f20dcddc92243e85be6f21594d9543a75baf450db1472bef1b&)
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
There is a pre-release 0.1.5 version build - https://github.com/l420y/egui_encryptor/releases/tag/version-0.1.5

Also you can run it from source:
1) Clone repo to your computer
2) in cmd: **cd ../egui_encryptor**
3) in cmd: **cargo run --release**

### Dependencies and libraries
```rust
[dependencies]
eframe = "0.24.1"
rfd = "0.12.1"
ascii_converter = "0.3.0"
base64 = "0.21.5"
```



