# build.ps1 - Build and (optionally) flash the NDIR firmware for Arduino Mega 2560
# Usage:
#   .\build.ps1          -> compile only, produce firmware.hex
#   .\build.ps1 -Flash   -> compile + flash via avrdude on COM14
#   .\build.ps1 -Port COM7 -Flash  -> compile + flash on a different COM port

param(
    [switch]$Flash,
    [string]$Port = "COM14"
)

# avr-gcc toolchain (Arduino IDE's bundled copy)
$AVR_BIN     = "$env:LOCALAPPDATA\Arduino15\packages\arduino\tools\avr-gcc\7.3.0-atmel3.6.1-arduino7\bin"
$AVRDUDE     = "$env:LOCALAPPDATA\Arduino15\packages\arduino\tools\avrdude\6.3.0-arduino17\bin\avrdude.exe"
$AVRDUDE_CONF = "$env:LOCALAPPDATA\Arduino15\packages\arduino\tools\avrdude\6.3.0-arduino17\etc\avrdude.conf"

if (-not (Test-Path "$AVR_BIN\avr-gcc.exe")) {
    Write-Error "avr-gcc not found at $AVR_BIN - is the Arduino IDE installed?"
    exit 1
}

$env:PATH = "$AVR_BIN;$env:PATH"
$gccVer = (avr-gcc --version)[0]
Write-Host "avr-gcc: $gccVer" -ForegroundColor Cyan

# Build
Write-Host "`nBuilding release firmware..." -ForegroundColor Yellow
cargo +nightly build -Zjson-target-spec --release
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "`nConverting ELF -> HEX..." -ForegroundColor Yellow
cargo +nightly objcopy -Zjson-target-spec --release -- -O ihex firmware.hex
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$size = (Get-Item firmware.hex).Length
Write-Host "firmware.hex ready ($size bytes)" -ForegroundColor Green

# Flash (optional)
if ($Flash) {
    Write-Host "`nFlashing to Arduino Mega on $Port..." -ForegroundColor Yellow
    & $AVRDUDE `
        -C $AVRDUDE_CONF `
        -p atmega2560 -c wiring -P $Port -b 115200 -D `
        -U "flash:w:firmware.hex:i"
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Flash complete!" -ForegroundColor Green
    } else {
        Write-Error "avrdude failed (exit $LASTEXITCODE)"
    }
}
