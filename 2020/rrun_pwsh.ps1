# This is a PowerShell script. To run this file use ./rrun.ps1.

$binName=$args[0]
$extName=[System.IO.Path]::GetExtension($binName)
if ($extName -eq ".rs") {
	$binName=[System.IO.Path]::GetFileNameWithoutExtension($binName)
	$testPath = Test-Path -Path ".\rrun-bin" -PathType Container
	if (-not $testPath) {
		New-Item -ItemType Directory -Path ".\rrun-bin"
	}
	rustc --out-dir="rrun-bin" "$binName.rs"
	$fullExpression = ".\rrun-bin\$binName $args"
	Invoke-Expression $fullExpression
} else {
	Write-Output "Not a Rust source file."
}
