latest_day := `ls src/bin/ | sort -r -g | head -n 1`
aoc-year := '2024'
aoc-session-cookie := env_var('aoc_session_cookie')

default:
  @just --list

# Builds specified configuration of project
_build configuration:
  cargo build --{{configuration}}

# Build release version of project
build: (_build "release")

# Format source code
fmt:
  cargo fmt --all

# Run linter against source code
check:
  cargo clippy

# Runs solution for specific provided day, of no input given defaults to running last days solution
exec day=latest_day: build
  RUST_LOG=info cargo run --release --bin {{day}}

# Prepares new day solution. `day_num` param should be given without leading zeroes
prepare day_num:
  mkdir -p 'src/bin/day{{shell('printf "%02d" $1', day_num)}}'
  touch 'src/bin/day{{shell('printf "%02d" $1', day_num)}}/main.rs'
  touch 'inputs/day{{shell('printf "%02d" $1', day_num)}}.txt'
  @curl --cookie session={{aoc-session-cookie}} -X GET https://adventofcode.com/{{aoc-year}}/day/{{day_num}}/input > 'inputs/day{{shell('printf "%02d" $1', day_num)}}.txt'
