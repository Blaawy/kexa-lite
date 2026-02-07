param(
  [string]$Rpc = "http://127.0.0.1:8030",
  [string]$Seed = "193.123.75.158:9030"
)

function Get-Raw([string]$Url) {
  try { (Invoke-WebRequest -UseBasicParsing -TimeoutSec 3 $Url).Content }
  catch { $null }
}

Write-Host "== seed tcp check =="

try {
  $seedHost, $seedPort = $Seed.Split(":")
  $c = New-Object System.Net.Sockets.TcpClient
  $iar = $c.BeginConnect($seedHost, [int]$seedPort, $null, $null)
  if (-not $iar.AsyncWaitHandle.WaitOne(1500)) { throw "timeout" }
  $c.EndConnect($iar)
  $c.Close()
  Write-Host "OK: $Seed reachable"
} catch {
  Write-Host "FAIL: $Seed not reachable ($_)"
}

Write-Host ""
Write-Host "== rpc $Rpc =="

Write-Host "-- /health (raw)"
$h = Get-Raw "$Rpc/health"
if ($h) { $h } else { Write-Host "FAIL: /health" }

Write-Host "-- /tip (raw)"
$t = Get-Raw "$Rpc/tip"
if ($t) { $t } else { Write-Host "FAIL: /tip" }

Write-Host "-- /peers/live (raw)"
$p = Get-Raw "$Rpc/peers/live"
if ($p) { $p } else { Write-Host "FAIL: /peers/live" }
