$inputFolder = Get-Location
$outputFolder = Get-Location
$svgFiles = Get-ChildItem -Path $inputFolder -Filter *.svg

foreach ($svgFile in $svgFiles) {
    $outputFile = Join-Path -Path $outputFolder -ChildPath "$($svgFile.BaseName).png"
    $convertCommand = "& magick convert -gravity center -background none -density 300 -bordercolor none -border 40x40 '$($svgFile.FullName)' '$outputFile'"
    Invoke-Expression $convertCommand
    Write-Host "Converted: $($svgFile.Name) to $($outputFile)"
}

Write-Host "Conversion completed."