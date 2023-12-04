#! /bin/sh
YEAR="$1"
DAY="$2"

echo "Preparing repository for YEAR=$YEAR and DAY=$DAY"
read -p "Press ENTER to continue" _

SESSION="53616c7465645f5fa302226dc7de369ab28c3f60c0ea294706f39ef32e98afa35e7ecfda51d801db9ee0123b116ec650780428ae47e1105aa7bb777556b8e5f6"

echo "Fetching input"
#wget --header "Cookie: session=$SESSION" "https://adventofcode.com/$YEAR/day/$DAY/input" -O inputs/input-$YEAR-day$(printf '%02d' $DAY)

echo "Creating branch"
git checkout -B "$YEAR-$DAY"

echo "Getting example"
EXAMPLE="$(wget https://adventofcode.com/$YEAR/day/$DAY -O - | sed -n '
/<pre><code>/,/<\/code><\/pre>/ {
    s#<pre><code>\|</code></pre>##
    /^$/d
    p
}'
)"
echo "Example from webpage:\n$EXAMPLE"
echo "Setting up solution block"
sed -i "/^        _ => {/i \
\ \ \ \ \ \ \ \ ($YEAR, $DAY, 1) => None,\n        ($YEAR, $DAY, 2) => None," src/lib.rs

echo "Setting up example"
sed -i "/Add next example above this line\./i \
\ \ \ \ \ \ \ \ \ \ \ \ ($YEAR, $DAY, _) => {\n\
\ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \"\\\\\n\
$(echo "$EXAMPLE" | sed 's/$/\\/')\"\n\
\ \ \ \ \ \ \ \ \ \ \ \ }" src/lib.rs

echo "Setting up example check"
sed -i "/fn check_examples(/i \
\ \ \ \ #[test_case($YEAR, $DAY, 1, 0)]\n\
\ \ \ \ #[test_case($YEAR, $DAY, 2, 0)]" src/lib.rs

echo "Setting up solution check"
sed -i "/fn check_solutions(/i \
\ \ \ \ #[test_case($YEAR, $DAY, 1, 0)]\n\
\ \ \ \ #[test_case($YEAR, $DAY, 2, 0)]" src/lib.rs

echo "Run with 'cargo run $DAY -y $YEAR'"
