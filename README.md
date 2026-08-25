# petfetch
```
  .------.     Blobbio @ readme
 /        \    -------------
|   ._.    |   OS:      Totally Real
 \        /    Uptime:  225m
  '------'     Memory:  10507MB / 30713MB
               Food:    89% / 100%
               Joy:     63% / 100%
```

Rust pets. Now you feel a tinge of responsibility whenever you open a terminal.

## Installation
Clone the repository and build it with Cargo.
```
git clone https://github.com/jordanbrotherton/petfetch.git
cd petfetch
cargo build --release
```
Then you can move it to your `PATH`.

## Usage
Get your friend first by running `petfetch` and following the adoption process.
Afterwards, you can always check up on your pet with `petfetch`, getting a nice view of your system and pet.
It is meant to act similarly to other `fetch` programs, so feel free to put it in your rc!

### Commands
Your pet will need to be taken care of eventually! Run `petfetch` with these commands to care for your pet!
* `feed`: Feed your pet when it gets hungry.
* `play [left/right]`: Play a guessing game with your pet to make it happy.
* `toilet`: Let your pet relieve its bladder.
* `medicate`: Heal your pet if it gets sick.

## Saving
petfetch saves your pet's status in your OS' configuration directory, for example, `xdg-config/petfetch/pet.json`.
