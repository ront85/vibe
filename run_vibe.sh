#!/bin/bash

# Vibe App Runner Script
# This script starts the Vibe transcription desktop application

echo "🚀 Starting Vibe Transcription App..."

# Check if we're in the right directory
if [ ! -d "desktop" ]; then
    echo "❌ Error: Please run this script from the project root directory (/Users/rontiso/Development/vibe)"
    exit 1
fi

# Navigate to desktop directory
cd desktop

# Check if mise is available and use it, otherwise fall back to direct commands
if command -v mise &> /dev/null; then
    echo "📦 Using mise to run the app..."
    mise exec -- bunx tauri dev
else
    echo "📦 Running directly with bunx..."
    bunx tauri dev
fi
