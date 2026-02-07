param(
  [string]$Rpc = "http://127.0.0.1:8030",
  [string]$Seed = "193.123.75.158:9030,141.145.159.171:9030"
)


# --- multi-seed shim (comma-separated "A:9030,B:9030") ---
$SeedList = $Seed -split ',' | ForEach-Object { $_.Trim() } | Where-Object { $_ }

$picked = $null
foreach ($hp in $SeedList) {
  $i = $hp.LastIndexOf(':'); if ($i -lt 1) { continue }
  $h = $hp.Substring(0,$i)
  $port = [int]$hp.Substring($i+1)
  if (Test-NetConnection -ComputerName $h -Port $port -InformationLevel Quiet) { $picked = $hp; break }
}
if ($picked) { $Seed = $picked } else { $Seed = $SeedList[0] }
# --- end multi-seed shim ---
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
