param (
    [string]$day,
    [string]$part
)
cargo run -p $day --bin $part -- "./inputs/$day.txt"