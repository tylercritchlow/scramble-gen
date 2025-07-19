# scramble-gen

Library with accompanying cli utility that generates practice scrambles for use in a future cube timer.

## Usage
### Library
```shell
cargo add scramble-gen
```
refer to docs at https://docs.rs/scramble-gen/latest/scramble_gen/

### CLI Utility

```shell
cargo install scramble-gen
```

```
scramble-gen - Generate practice scrambles for Rubik's cubes

USAGE:
    scramble-gen [OPTIONS]

OPTIONS:
    --2x2                Generate 2x2 scrambles
    --3x3                Generate 3x3 scrambles (default)
    --4x4                Generate 4x4 scrambles
    --5x5                Generate 5x5 scrambles
    --6x6                Generate 6x6 scrambles
    --7x7                Generate 7x7 scrambles
    --length=<num>       Set scramble length (default varies by cube)
    --amount=<num>       Generate multiple scrambles (default: 1)
    --help, -h           Show this help message

EXAMPLES:
    scramble-gen --3x3
    scramble-gen --5x5 --amount=5
    scramble-gen --7x7 --length=120
```