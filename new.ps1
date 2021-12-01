param (
    [Parameter(Mandatory = $true)]
    [string]$day
)
Push-Location ./crates/
cargo generate -v --git ..\..\day_template\ --name $day
rm -r -force $day/.git
rm -r -force $day/.cargo-ok
Pop-Location