#!/usr/bin/env bash
# Test DAD notification system

echo "🧪 Testing DAD Notification System"
echo "===================================="
echo ""

# Check if notify-send is installed
if ! command -v notify-send &> /dev/null; then
    echo "❌ notify-send not found. Install with:"
    echo "   sudo apt install libnotify-bin"
    exit 1
fi

echo "✅ notify-send found"
echo ""

# Test 1: Training notification (low urgency)
echo "📤 Test 1: Training notification (low urgency)..."
notify-send \
    --app-name="DAD" \
    --urgency=low \
    --icon=dialog-information \
    --expire-time=5000 \
    --category=dad.training \
    "🧠 DAD Learning Episode" \
    "Command: python script.py
Error Type: ModuleNotFoundError
Status: Training Complete"

sleep 2

# Test 2: Training notification (normal urgency)
echo "📤 Test 2: Training notification (normal urgency)..."
notify-send \
    --app-name="DAD" \
    --urgency=normal \
    --icon=dialog-information \
    --expire-time=5000 \
    --category=dad.training \
    "🧠 DAD Learning Episode" \
    "Command: cargo build --release
Error Type: CompilationError
Status: Training Complete"

sleep 2

# Test 3: Pattern detection notification
echo "📤 Test 3: Pattern detection notification..."
notify-send \
    --app-name="DAD" \
    --urgency=normal \
    --icon=dialog-information \
    --expire-time=4000 \
    --category=dad.training \
    "🔍 DAD Pattern Detected" \
    "Similar error detected!
7 related errors in history
Hash: a3f8bc9d"

sleep 2

# Test 4: Prediction warning (critical urgency)
echo "📤 Test 4: Prediction warning (critical)..."
notify-send \
    --app-name="DAD" \
    --urgency=critical \
    --icon=dialog-warning \
    --expire-time=8000 \
    --category=dad.training \
    "⚠️  DAD Prediction Warning" \
    "Command likely to fail: rm -rf /nonexistent
Failure probability: 89%"

sleep 2

# Test 5: Training failure notification
echo "📤 Test 5: Training failure notification..."
notify-send \
    --app-name="DAD" \
    --urgency=normal \
    --icon=dialog-error \
    --expire-time=5000 \
    --category=dad.training \
    "🧠 DAD Learning Episode" \
    "Command: ./broken_script.sh
Error Type: SyntaxError
Status: Training Failed"

echo ""
echo "✅ All test notifications sent!"
echo ""
echo "If you don't see notifications:"
echo "1. Check if dunst is running: ps aux | grep dunst"
echo "2. Verify dunst config: dunst -print"
echo "3. See docs/NOTIFICATIONS.md for troubleshooting"
