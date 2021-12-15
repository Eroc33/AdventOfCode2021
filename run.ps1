param (
    [Parameter(Mandatory = $true)]
    [string]$day,
    [Parameter(Mandatory = $true)]
    [string]$part,
    [Parameter(Mandatory=$false)]
    [switch]$release
)
$releaseFlag = if ($release){"--release"}else{""}
cargo run -p $day --bin $part $releaseFlag -- "./inputs/$day.txt"