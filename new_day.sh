# Usage ./newday.sh <DAY>

set -e

DAY=$(echo $1 | sed 's/^0*//') # Remove leading zeros

# Create the input and puzzle files
mkdir -p inputs puzzles
aoc download -d $DAY --input-file inputs/day$DAY.txt --puzzle-file puzzles/day$DAY.md --overwrite

# Create the day file from the template
cp -n "src/day_template.rs" src/day$DAY.rs

# Add the day as a module to the lib
echo pub mod day$DAY\; >>src/lib.rs

# Add the day to the benches template
sed -i "s/\(benches!(.*\));/\1, $DAY);/" ./benches/bench_days.rs

# Format everything
cargo fmt

echo "Bootstrap for day ${1} complete!"
