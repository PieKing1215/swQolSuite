# swMod
Mod for [Stormworks: Build and Rescue](https://store.steampowered.com/app/573090/Stormworks_Build_and_Rescue/) that adds some QOL settings.<br>
Only supports 64 bit Windows version (stormworks64.exe). It might work through wine/proton, but untested.

![image](https://github.com/user-attachments/assets/d3c0978b-7897-4b04-9c6d-71e28dfbe1de)

## READ THIS FIRST
The mod uses low level techniques (mainly ASM injection) to patch the game since there's no other way to do it.<br>
Due to the nature of these methods, I won't make any guarantees about stability.<br>
If something goes wrong it will most likely crash your game, so please save your work often (especially before injecting or ejecting the mod)<br>

swMod may be flagged by antiviruses as a side effect of how it works. The way swMod injects into the game is similar to how some viruses hook other processes, and some antiviruses will detect this.
Obviously I will say this repo (PieKing1215/swMod) and official builds do not actually contain malware to my knowledge, but if you want to be sure, look over the code and build from source.

## Download
For "stable" releases (there are none right now), see [Releases](../../releases).<br>
For dev builds: sign in to GitHub, go [here](https://github.com/PieKing1215/swMod/actions/workflows/autobuild.yml?query=branch%3Amain+is%3Asuccess), click the latest one, scroll down to "Artifacts" and download it.<br>
Or download the latest at https://nightly.link/PieKing1215/swMod/workflows/autobuild/main<br>
Unzip and run swmod-inject.exe to run.

## Basic Usage
Have Stormworks open and run the injector exe.<br>
A couple menus should appear ingame:
- SWMod: shows version number & commit SHA, and has an "Eject" button which removes the mod.<br>
- Tweaks: shows the settings for all of the mod's features, see below for details.<br>
- Errors: if there were any errors loading features, they will be shown in this window (hidden if no errors).<br>

*Note: swMod does not modify any Stormworks files on disk, so if you close & reopen the game you have to re-run the injector exe*

You can press the grave/backtick/tilde key [`` `~ ``] to toggle the visibility of the menus.

## Game Updates
The patches are set up so they should usually continue to work after game updates unless related parts of code were touched.<br>
However if after an update any patches fail, you should just get an error in the error window and the tweak will be disabled until it is updated to work again.

# Tweaks/Settings

### Map Sleep
There's normally a 10 millisecond sleep in the map render code, which makes it very laggy. This setting lets you change or remove that.

### Editor Camera
Settings to adjust the speed of the editor free camera. Separate settings for base speed, shift speed, and control speed.

### Disable Placement Support Check
If enabled, disables the check for a supporting surface when placing parts (eg. pipes, wedges don't need to connect).

### Disable Merge Check
If enabled, disables the check for a connecting surface when merging two subgrids (ie. lets you merge grids even if they're not connected)

## Licenses

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Disclaimer
I am not personally affiliated with Stormworks: Build and Rescue or Geometa, nor has Stormworks: Build and Rescue or Geometa endorsed this product.<br>
Stormworks: Build and Rescue and any of its content or materials are the property of their respective owners.
