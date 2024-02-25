param (
		[Parameter(Mandatory=$true)][string]$uri
)

$url = "https://0ae1001c040e47fb80c83ff2003c0062.web-security-academy.net/filter?category=" + $uri;


Write-host $url

$response = Invoke-WebRequest -Uri $url

if ($response.StatusCode -ne 200) {
	Write-Host "Failed get data from URL"
	exit 0
}

if ($response.Content -match "cat") {
	Write-Host "Success"
} else {
	Write-Host "Failed"
}