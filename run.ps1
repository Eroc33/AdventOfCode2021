param (
    [Parameter(Mandatory = $true)]
    [string]$day,
    [Parameter(Mandatory = $true)]
    [string]$part
)
cargo run -p $day --bin $part -- "./inputs/$day.txt"