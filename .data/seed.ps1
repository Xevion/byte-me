# Download all files from the Big Buck Bunny movies directory

$baseUrl = "https://download.blender.org/peach/bigbuckbunny_movies/"
$outputDir = "$PSScriptRoot"

# Get the HTML content of the directory listing
$html = Invoke-WebRequest -Uri $baseUrl

# Extract all file links (ignore parent directory links)
$fileLinks = ($html.Links | Where-Object {
        $_.href -notmatch '^\.\./' -and $_.href -notmatch '/$'
    }).href

# Download each file
foreach ($file in $fileLinks) {
    $fileUrl = "$baseUrl$file"
    $outFile = Join-Path $outputDir $file
    if (Test-Path $outFile) {
        Write-Host "Skipping $outFile (already exists)"
        continue
    }
    Write-Host "Downloading $fileUrl to $outFile"
    Invoke-WebRequest -Uri $fileUrl -OutFile $outFile
}

Write-Host "All files downloaded."
