# old Way
# $InformationPreference = "Continue"

# Get-ChildItem ./ | Where-Object { $_.PsIsContainer } | ForEach-Object {
#     Push-Location $_;
#     cargo test 2>&1 > $null;
#     if (!$?) {
#         $problem = $_.Name.Split("\")[-1]
#         Write-Information "Gotta do $problem"
#     }
#     Pop-Location;
# }

# This is The Way
Push-Location "./tester";
cargo run
Pop-Location;