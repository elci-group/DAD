#!/usr/bin/env bash
# DAD Bash Hook - Captures command errors and sends to daemon

# Only activate if DAD socket exists
DAD_SOCKET="/tmp/dad-events.sock"

# Function to capture command exit status and send to DAD
dad_capture() {
    local exit_code=$?
    local cmd="$BASH_COMMAND"
    
    # Only capture failures
    if [ $exit_code -ne 0 ] && [ -S "$DAD_SOCKET" ]; then
        # Capture stderr (stored in temp file by trap)
        local stderr_file="/tmp/dad-stderr-$$"
        if [ -f "$stderr_file" ]; then
            local stderr_content=$(cat "$stderr_file" 2>/dev/null)
            rm -f "$stderr_file"
        else
            local stderr_content=""
        fi
        
        # Get environment signature
        local env_sig="$(uname -s)-$(basename $SHELL)"
        
        # Create JSON event
        local event_json=$(cat <<EOF
{
    "command": "$cmd",
    "working_dir": "$PWD",
    "exit_code": $exit_code,
    "stderr": "$stderr_content",
    "env_signature": "$env_sig",
    "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
}
EOF
)
        
        # Send to DAD daemon via socket (async, non-blocking)
        echo "$event_json" | nc -U -w1 "$DAD_SOCKET" 2>/dev/null &
    fi
    
    return $exit_code
}

# Install trap on DEBUG to capture commands
trap 'dad_capture' DEBUG

# Optional: Add prompt indicator when DAD is active
if [ -S "$DAD_SOCKET" ]; then
    export PS1="[DAD] $PS1"
fi

echo "DAD shell hook activated"
