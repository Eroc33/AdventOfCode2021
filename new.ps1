param (
    [string]$day,
    [string]$part
)
Push-Location ./crates/
cargo generate -v --git ..\..\day_template\ --name $day
rm -r -force $day/.git
rm -r -force $day/.cargo-ok
Pop-Location