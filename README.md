# volume

Console programme to adjust the default output and input volumes on Windows.

Written in Rust using [windows-rs](https://github.com/microsoft/windows-rs).

## Usage

`volume.exe {out|in} {inc|dec|0.NN}`

## Examples

`volume.exe`  
show current volumes

`volume.exe out inc`  
increment output volume by 1%

`volume.exe in dec`  
decrement input volume by 1%

`volume.exe out 0.50`  
set default output volume to 50%.