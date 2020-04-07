# nes_emulator

NES Emulator written in Rust  

<img src="https://user-images.githubusercontent.com/22634362/77064099-2d51f680-6a22-11ea-801d-fb1feee6ad46.gif" width="400">
Super Mario Bros.

<img src="https://user-images.githubusercontent.com/22634362/77730078-6a7e4000-7043-11ea-934f-c129c873f105.gif" width="300">
Dragon Quest

<img src="https://user-images.githubusercontent.com/22634362/78538239-a7a8b600-782b-11ea-97a8-a9edfa51ea73.gif" width="300">
Final FantasyIII

# RUN
## wasm
```
$ make
$ npm install
$ npm start
$ open http://localhost:10080
```

## test
```
$cargo test
```

# Refereneces
## main code & copyright 
from https://github.com/bokuweb/rustynes
Copyright (c) 2018 @bokuweb
## code
- sample code from: 
  - https://github.com/kamiyaowl/rust-nes-emulator
- `apu` sample code from:
  - https://github.com/fogleman/nes/blob/b2c994703237bdbe006c52809fecbbbc9ac35dd6/nes/apu.go
  
## website
- http://wiki.nesdev.com/w/index.php/Programming_guide
- http://pgate1.at-ninja.jp/NES_on_FPGA/nes_apu.htm
- http://hp.vector.co.jp/authors/VA042397/nes/apu.html

## books
- https://booth.pm/ja/items/1748859


# ROMS
- hello
  - http://hp.vector.co.jp/authors/VA042397/nes/sample.html
- nestest
  - https://wiki.nesdev.com/w/index.php/Emulator_tests
- other test roms
  - https://wiki.nesdev.com/w/index.php/Emulator_tests
  
# TODO
- Mappers
  - [x] Mapper3
  - [x] Mapper4
- Sound
  - [x] DMC
  - [ ] sweep not correct work
  - [x] irq
  
  
