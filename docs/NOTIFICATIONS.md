# DAD Notification System

DAD includes a dunst-compatible notification system that alerts you to training episodes, error patterns, and predictions in real-time.

## Features

- 🧠 **Training Episode Notifications**: Get notified when DAD learns from a new error
- 🔍 **Pattern Detection Alerts**: See when similar errors are detected
- ⚠️ **Prediction Warnings**: Receive warnings before running commands likely to fail
- 🎨 **Customizable**: Full dunst configuration support

## Installation

### 1. Install notify-send

```bash
# On Pop!_OS/Ubuntu/Debian
sudo apt install libnotify-bin

# On Arch
sudo pacman -S libnotify

# On Fedora
sudo dnf install libnotify
```

### 2. Install dunst (if not already installed)

```bash
# On Pop!_OS/Ubuntu/Debian
sudo apt install dunst

# On Arch
sudo pacman -S dunst

# On Fedora
sudo dnf install dunst
```

### 3. Configure dunst for DAD

Option A: Include DAD config in main dunst configuration:

```bash
# Backup existing config
cp ~/.config/dunst/dunstrc ~/.config/dunst/dunstrc.backup

# Append DAD configuration
cat config/dunstrc-dad >> ~/.config/dunst/dunstrc

# Restart dunst
killall dunst
```

Option B: Use dunst config directory (dunst 1.6.0+):

```bash
# Create config directory
mkdir -p ~/.config/dunst/dunstrc.d

# Copy DAD config
cp config/dunstrc-dad ~/.config/dunst/dunstrc.d/dad.conf

# Restart dunst
killall dunst
```

### 4. Configure notification preferences

Edit `config/notifications.toml` to customize:

```toml
[notifications]
enabled = true
notify_on_training = true
notify_on_similarity = true
notify_on_prediction = false  # Enable when predictions are ready

[filtering]
exclude_commands = [
    "^ls ",
    "^cd ",
]
```

## Notification Types

### Training Episode Notifications

Triggered when DAD processes a new error and updates its model.

**Example:**
```
🧠 DAD Learning Episode
Command: python script.py
Error Type: ModuleNotFoundError
Status: Training Complete
```

**Urgency Levels:**
- **Low**: Recoverable errors (exit codes 1, 2, 126, 127)
- **Normal**: Other errors
- **Critical**: System-level failures

### Pattern Detection Notifications

Triggered when DAD detects similar errors in history.

**Example:**
```
🔍 DAD Pattern Detected
Similar error detected!
5 related errors in history
Hash: a3f8bc9d
```

### Prediction Warnings

Triggered before running commands with high failure probability.

**Example:**
```
⚠️ DAD Prediction Warning
Command likely to fail: rm -rf /important
Failure probability: 85%
```

## Testing Notifications

```bash
# Test training notification
cargo run --bin dad-daemon &

# In another terminal, trigger an error
python3 -c "import nonexistent"

# You should see a notification appear
```

## Dunst Configuration Reference

### Color Schemes

The default DAD dunst config uses Catppuccin Mocha colors:

- **Background**: `#1e1e2e` (dark)
- **Foreground**: `#cdd6f4` (text)
- **Low urgency**: Blue frame (`#89b4fa`)
- **Normal urgency**: Yellow frame (`#f9e2af`)
- **Critical urgency**: Red frame (`#f38ba8`)
- **Pattern detection**: Green frame (`#a6e3a1`)
- **Predictions**: Orange frame (`#fab387`)

### Customizing Colors

Edit `~/.config/dunst/dunstrc.d/dad.conf`:

```ini
[dad_training]
    appname = "DAD"
    background = "#your_background_color"
    foreground = "#your_text_color"
    frame_color = "#your_border_color"
    timeout = 5
```

### Custom Icons

DAD looks for icons in standard locations. To use custom icons:

```ini
[dad_training]
    icon = /path/to/your/icon.png
```

Recommended icon packs:
- Papirus
- Breeze
- Adwaita

## Filtering & Quiet Hours

### Filtering by Command

Edit `config/notifications.toml`:

```toml
[filtering]
exclude_commands = [
    "^git ",      # Ignore git commands
    "^npm ",      # Ignore npm commands
    "^test$",     # Ignore 'test' command
]
```

### Quiet Hours

Disable notifications during specific times:

```toml
[filtering]
quiet_start = "22:00"
quiet_end = "08:00"
```

### Using dunst Scripts

For advanced filtering, use dunst script rules:

```ini
[dad_conditional]
    appname = "DAD"
    script = ~/.config/dunst/scripts/dad-filter.sh
```

Example filter script:

```bash
#!/bin/bash
# Only show notifications during work hours
current_hour=$(date +%H)
if [ $current_hour -ge 9 ] && [ $current_hour -le 17 ]; then
    exit 0  # Show
else
    exit 1  # Hide
fi
```

## Troubleshooting

### Notifications not appearing

1. **Check if notify-send works:**
   ```bash
   notify-send "Test" "This is a test notification"
   ```

2. **Check if dunst is running:**
   ```bash
   ps aux | grep dunst
   ```

3. **Check DAD logs:**
   ```bash
   RUST_LOG=dad_core=debug cargo run --bin dad-daemon
   ```

4. **Verify dunst config:**
   ```bash
   dunst -print
   ```

### Notifications appearing in wrong style

1. **Restart dunst:**
   ```bash
   killall dunst
   # Dunst will auto-restart on next notification
   ```

2. **Check config priority:**
   Dunst uses the first matching rule. Ensure DAD rules are near the top of your config.

### Too many notifications

Adjust thresholds in `config/notifications.toml`:

```toml
[notifications]
notify_on_training = false  # Disable training notifications
similarity_threshold = 10   # Only notify for frequent patterns
```

## Integration with Other Tools

### Using with i3/Sway

Dunst works automatically with i3 and Sway.

### Using with GNOME

GNOME uses its own notification daemon. To use dunst instead:

```bash
# Disable GNOME notification daemon
gsettings set org.gnome.desktop.notifications show-banners false

# Start dunst
dunst &
```

### Using with KDE Plasma

Install and configure dunst as usual. It will integrate with Plasma notifications.

## API for Custom Integrations

The notification module exports these functions:

```rust
// Notify about training episode
send_training_notification(event, "Training Complete").await?;

// Notify about pattern detection
send_similarity_notification(hash, similar_count).await?;

// Notify about prediction
send_prediction_notification(command, probability).await?;
```

## Examples

### Example 1: Custom notification for specific errors

```rust
if event.stderr.contains("Permission denied") {
    send_notification(
        "🔒 DAD Permission Issue",
        "Try running with sudo or checking file permissions",
        NotifyUrgency::Normal,
        Some(8000)
    ).await?;
}
```

### Example 2: Batch notification summary

```rust
// At end of training batch
let summary = format!("Learned from {} errors in the last hour", count);
send_notification(
    "📊 DAD Training Summary",
    &summary,
    NotifyUrgency::Low,
    Some(10000)
).await?;
```

## See Also

- [dunst documentation](https://dunst-project.org/documentation/)
- [notify-send man page](https://man.archlinux.org/man/notify-send.1)
- [Desktop Notifications Specification](https://specifications.freedesktop.org/notification-spec/latest/)
