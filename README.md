# swQolSuite
Mod for [Stormworks: Build and Rescue](https://store.steampowered.com/app/573090/Stormworks_Build_and_Rescue/) that adds some QOL settings.<br/>
Only supports 64 bit Windows version (stormworks64.exe). It might work through wine/proton, but untested.

![image](https://github.com/user-attachments/assets/52329669-8bc5-4dce-a0b9-e4526c37d3b8)

## READ THIS FIRST
swQolSuite uses low level techniques (mainly ASM injection) to patch the game since there's no other way to do it.<br/>
Due to the nature of these methods, I won't make any guarantees about stability.<br/>
**If something goes wrong it will most likely crash your game, so please save your work often.**<br/>
Injecting or ejecting the mod are the most likely to crash, but it might also be possible when changing settings while in use (eg. changing editor settings while in editor)<br/>

swQolSuite may be flagged by antiviruses as a side effect of how it works. The way swQolSuite injects into the game is similar to how some viruses hook other processes, and some antiviruses will detect this.
Obviously I will say this repo (PieKing1215/swQolSuite) and official builds do not actually contain malware to my knowledge, but if you want to be sure, look over the code and build from source.

swQolSuite is not a cheating/griefing tool, please do not request features that give you an advantage over other players in multiplayer.

## Download
For "stable" releases (there are none right now), see [Releases](../../releases).<br/>
For dev builds: sign in to GitHub, go [here](https://github.com/PieKing1215/swQolSuite/actions/workflows/autobuild.yml?query=branch%3Amain+is%3Asuccess), click the latest one, scroll down to "Artifacts" and download it.<br/>
Or download the latest at https://nightly.link/PieKing1215/swQolSuite/workflows/autobuild/main<br/>
Unzip and run swqols-inject.exe to run.

## Basic Usage
Have Stormworks open and run the injector exe.<br/>
A couple menus should appear ingame:
- swQolSuite: shows version number & commit SHA, and has an "Eject" button which removes the mod.<br/>
- Tweaks: shows the settings for all of the mod's features, see below for details.<br/>
- Errors: if there were any errors loading features, they will be shown in this window (hidden if no errors).<br/>

*Note: swQolSuite does not modify any Stormworks files on disk, so if you close & reopen the game you have to re-run the injector exe*

You can press the grave/backtick/tilde key [`` `~ ``] to toggle the visibility of the menus.

## Game Updates
The patches are set up so they should usually continue to work after game updates unless related parts of code were touched.<br/>
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

### Fast Main Menu Fade
Speeds up the main menu fade out/in when loading a save or returning to main menu (speeds up loading since it waits for the transition to end before starting)

### Skip Loading Finish Animation
Normally when loading into or exiting a world, once the loading finishes, the progress bar animates from its current percentage to 100%.<br/>
The animation is purely visual but it waits for it before spawning you in.<br/>
This tweak skips the animation so you start spawning immediately once ready.

### Show Hidden Components
If enabled, the editor component picker will include components marked as hidden (mainly deprecated ones).<br/>
Changing this setting requires reloading the save/world to apply.

### Force Borderless Fullscreen
Changes fullscreen to open as borderless instead of exclusive.<br/>
You need to toggle fullscreen off and on for it to update.

### Disable Minimize on Focus Lost
Disables the window automatically minimizing when it loses focus in fullscreen.<br/>
Turning fullscreen off and back on while this is enabled also fixes the window being forced on top of all other windows.

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
I am not personally affiliated with Stormworks: Build and Rescue or Geometa, nor has Stormworks: Build and Rescue or Geometa endorsed this product.<br/>
Stormworks: Build and Rescue and any of its content or materials are the property of their respective owners.
