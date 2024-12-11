{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:
{
  # https://devenv.sh/packages/
  packages = with pkgs; [
    aoc-cli
    cargo-codspeed
  ];

  scripts = {
    day.exec = ''
      # Usage newday.sh <DAY>

      set -e

      if [ -z "$1" ]; then
        DAY=$(date +%-d) # Get the current day of the month (without leading zero)
      else
        DAY=$(echo $1 | sed 's/^0*//') # Remove leading zeros
      fi

      # Create the input and puzzle files
      mkdir -p inputs puzzles
      aoc download -d $DAY --input-file inputs/day$DAY.txt --puzzle-file puzzles/day$DAY.md --overwrite

      # Create the day file from the template
      cp -n "src/day_template.rs" src/day$DAY.rs

      # Add the day as a module to the lib
      echo pub mod day$DAY\; >>src/lib.rs

      # Add the day to the benches template
      sed -i "s/\(benches!(.*\));/\1, $DAY);/" ./benches/bench_days.rs

      # Uncomment the day in the main file
      sed -i "s/\/\/ $DAY =>/$DAY =>/" ./src/main.rs

      # Format everything
      cargo fmt

      echo "Bootstrap for day $1 complete!"
    '';

    leaderboards.exec = ''
      # Hardcoded list of IDs
      ids=(1813799 207429 1893392)

      # Loop through each ID and execute the command
      for id in "''${ids[@]}"; do
          aoc private-leaderboard "''${id}"
      done
    '';
  };

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [ "x86_64-unknown-linux-gnu" ];
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
      "rust-src"
    ];
  };

  env.RUST_LOG = "info";
}
