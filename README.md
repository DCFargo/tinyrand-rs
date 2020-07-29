<p align="center">
    <img src="https://raw.githubusercontent.com/DCFargo/tinyrand-rs/master/logo.png">
</p>
<br/>
<p align="center">
    <img src="https://img.shields.io/github/languages/code-size/DCFargo/tinyrand-rs">
    <img src="https://img.shields.io/appveyor/build/DCFargo/tinyrand-rs">
    <img src="https://img.shields.io/codacy/grade/c576297e6d07469784aa809ae93286d8">
    <img src="https://img.shields.io/github/downloads/DCFargo/tinyrand-rs/total">
</p>
<br/>
  
tinyrand is a lightweight command-line randomizer built for a user-friendly experience. Originally built in Rust simply as a tool for building in Minecraft, tinyrand's general syntax and small size is capable of a much wider variety of applications.

## How to use
In the directory that `tinyrand.exe` is located, type:

    tinyrand <option> <weight> <option> <weight> etc.
where `<option>` is any name, and `<weight>` is the previous options weighted value.

## Examples

    tinyrand cobblestone 60 gravel 20 stone 20
In this example, cobblestone, gravel, and stone will be selected 60%, 20%, and 20% respectively.
Note that weighted values do not always have to add up to 100:

    tinyrand pizza 40 tacos 20 burgers 90
Nor do the weights have to be in order.
Here are some more examples:

    tinyrand up 1 down 1 left 1 right 1
 
    tinyrand go_to_bed 99 stay_up 1

    tinyrand english 90 math 84 science 91
