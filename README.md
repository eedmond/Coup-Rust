# Coup Simulator
This project is for learning Rust.

## Getting Started
1. Open the coup.code-workspace in VS Code
1. Install the recommended extensions
1. Should be able to F5 and set breakpoints to debug the code
1. To run the game from command line, navigate to ./cargo-game and run
```shell
cargo run {num players}
```

## Project Structure
`coup-engine`
- Library containing the engine implementation along with the interface definition for the players.

`coup-game`
- Thin binary over coup-engine that parses command line args and calls into the engine library to run the game.

## Running Tests
### From UI (with debugging):
Debugging tests is done via the overlay UI (debug/run tests) next to the tests themselves

### From command line:
To run tests from the command line, navigate to the proper target (e.g. ./coup-engine) and run
```shell
cargo test
```

## Project Goals
- Create an engine that executes the rules of Coup
- Define "players" who can decide actions on their turns
    - Engine should enforce rules -- players shouldn't be able to cheat
- Players should run in separate threads (to learn concurrency)
- Players should have read-access to the state of the world (to learn ownership and mutability)

## Learnings
- Doc tests don't run in binaries
- Doc tests can't access private members -- use unit tests
- Intellisense only worked after setting up project "correctly"
    - Project would still build and run, but no Intellisense
- Folder structure seems to be very important
    - Defining a module seems to require a file/folder of that name.
- Nesting a library inside the binary meant:
    1. The Debug UI for tests doesn't work when opening the binary (parent)
    1. debugging the whole game isn't possible when opening the library (child)

    Note: Everything worked after creating a separate (sibling) folder for the binary and library and using a wrapper code-workspace that holds both
- Traits == interfaces
    - Supports dynamic and static dispatch
        - We'll require dynamic for player controllers