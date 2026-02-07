param(
  [string]$Seed = "193.123.75.158:9030,141.145.159.171:9030",
  [string]$Rpc = "http://127.0.0.1:8030",
  [int]$WaitSec = 8
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
$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
function Get-Raw([string]$Url) {
  try { (Invoke-WebRequest -UseBasicParsing -TimeoutSec 3 $Url).Content }
  catch { $null }
}

Write-Host "== Step 1: start node (docker compose remote1) =="

$join = "docker-compose.remote1.joinseed.yml"
$cmd  = @("compose","-f","docker-compose.yml","-f","docker-compose.remote1.root.yml")
if (Test-Path $join) { $cmd += @("-f",$join) }

# run quietly; docker prints build progress to stderr (can look like errors in PS)
$oldEap = $ErrorActionPreference
$ErrorActionPreference = "SilentlyContinue"

& docker @cmd up -d --build --quiet-pull *> $null
$code = $LASTEXITCODE

$ErrorActionPreference = $oldEap

if ($code -ne 0) {
  Write-Host "compose failed; re-running verbose for logs..."
  & docker @cmd up -d --build
  exit $code
}

Write-Host ""
Write-Host "== Step 2: wait $WaitSec sec =="
Start-Sleep -Seconds $WaitSec

Write-Host ""
Write-Host "== Step 3: verify (seed reachability + rpc outputs) =="

# seed tcp
try {
  $seedHost, $seedPort = $Seed.Split(":")
  $c = New-Object System.Net.Sockets.TcpClient
  $iar = $c.BeginConnect($seedHost, [int]$seedPort, $null, $null)
  if (-not $iar.AsyncWaitHandle.WaitOne(1500)) { throw "timeout" }
  $c.EndConnect($iar); $c.Close()
  Write-Host "OK: seed reachable: $Seed"
} catch {
  Write-Host "FAIL: seed not reachable: $Seed ($_)"
  exit 1
}

$h = Get-Raw "$Rpc/health"
$t = Get-Raw "$Rpc/tip"
$p = Get-Raw "$Rpc/peers/live"

if (-not $h) { Write-Host "FAIL: rpc /health"; exit 1 }
if (-not $t) { Write-Host "FAIL: rpc /tip"; exit 1 }
if (-not $p) { Write-Host "FAIL: rpc /peers/live"; exit 1 }

Write-Host "-- /health"; $h
Write-Host "-- /tip";    $t
Write-Host "-- /peers/live"; $p

if ($p -notmatch [regex]::Escape($Seed)) {
  Write-Host "FAIL: peers/live does not include seed $Seed"
  exit 1
}

Write-Host ""
Write-Host "PASS  joined + verified"
