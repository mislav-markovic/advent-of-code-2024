# Advent Of Code 2024

Solutions for advent of code.

Project setup without any solutions is tagged with `starter-template`

## Project structure

Each days solutions are seperate executable placed in `src/bin/dayXX` folders.
Common code is organized in `src/lib/` folder.
Each days inputs should be placed in `inputs/` and named `dayXX.txt`

## Workflow

`just` tasks runner is used and several helpful recepies are provided.

### Prepare

Downloads input for specific day and saves it to file. Also creates new `main.rs` file where solution will be coded.

To work env variable with `session` token values needs to be set as `aoc_session_cookie`.
To change for which year inputs are downloaded, change variable `aoc-year` in justfile.

Example:

This downloads file for day 5, creates `inputs/day05.txt` file as well as `src/bin/day05/main.rs` file

```sh
just prepare 5
```

### Execute

Runs executable for specific day. If no argument is provided runs last solved days executable.

Example:

```sh
just exec
```

If we have solutions: `src/bin/day{01, 02, 03}` it will run `day03` solution.

```sh
just exec day02
```

Will run executable for day02 solution.
