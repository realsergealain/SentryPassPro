@echo off
title SentryPass Pro
cd /d "%~dp0"
echo ========================================================
echo   Demarrage de SentryPass Pro (Sentinelle Cryptographique)
echo ========================================================
start "" "target\release\sentrypass.exe"
exit
