#!/bin/bash

# LOC Counter Script
# Counts lines of code in Rust projects, excluding build artifacts, 
# working directories, and generated files

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_color() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Function to count LOC for a directory
count_loc() {
    local dir=$1
    local label=$2
    
    if [[ ! -d "$dir" ]]; then
        print_color $RED "Directory '$dir' not found"
        return 1
    fi
    
    # Find all .rs files, excluding:
    # - target/ directory (build artifacts)
    # - _wb/ directory (working/backup files)
    # - _dev/ directory (development files)
    # - Any file with "generated" in the name
    # - Any file in directories containing "target", "_wb", or "_dev"
    local count=$(find "$dir" -name "*.rs" \
        -not -path "*/target/*" \
        -not -path "*/_wb/*" \
        -not -path "*/_dev/*" \
        -not -name "*generated*" \
        -not -path "*/build/*" \
        -not -path "*/out/*" \
        | xargs wc -l 2>/dev/null | tail -1 | awk '{print $1}')
    
    # Handle case where no files are found
    if [[ "$count" == "" ]]; then
        count=0
    fi
    
    printf "%-25s %8s lines\n" "$label:" "$count" >&2
    echo "$count"
}

# Function to show file breakdown
show_breakdown() {
    local dir=$1
    local label=$2
    
    print_color $BLUE "\n=== File breakdown for $label ==="
    find "$dir" -name "*.rs" \
        -not -path "*/target/*" \
        -not -path "*/_wb/*" \
        -not -path "*/_dev/*" \
        -not -name "*generated*" \
        -not -path "*/build/*" \
        -not -path "*/out/*" \
        -exec wc -l {} + 2>/dev/null | sort -nr | head -10
}

# Main execution
main() {
    local current_dir=${1:-"."}
    local original_dir="$current_dir/ferrumfix-original"
    local show_details=${2:-"false"}
    
    print_color $YELLOW "=== Rust LOC Comparison ==="
    echo ""
    
    # Count core crates only (most accurate comparison)
    print_color $GREEN "Core Crates Comparison:"
    if [[ -d "$current_dir/crates" ]]; then
        current_crates=$(count_loc "$current_dir/crates" "Current (crates)")
    else
        print_color $RED "No crates/ directory found in current directory"
        current_crates=0
    fi
    
    if [[ -d "$original_dir/crates" ]]; then
        original_crates=$(count_loc "$original_dir/crates" "Original (crates)")
    else
        print_color $RED "No ferrumfix-original/crates/ directory found"
        original_crates=0
    fi
    
    # Calculate difference
    if [[ ${current_crates} -gt 0 && ${original_crates} -gt 0 ]]; then
        local diff=$((current_crates - original_crates))
        local percent=$(echo "scale=1; ($diff * 100.0) / $original_crates" | bc -l 2>/dev/null || echo "N/A")
        
        echo ""
        if [[ $diff -gt 0 ]]; then
            print_color $RED "Difference: +$diff lines (+${percent}%)"
        elif [[ $diff -lt 0 ]]; then
            print_color $GREEN "Difference: $diff lines (${percent}%)"
        else
            print_color $BLUE "Difference: No change"
        fi
    fi
    
    echo ""
    print_color $GREEN "Full Project Comparison (excluding artifacts):"
    current_total=$(count_loc "$current_dir" "Current (total)")
    original_total=$(count_loc "$original_dir" "Original (total)")
    
    # Calculate total difference
    if [[ ${current_total} -gt 0 && ${original_total} -gt 0 ]]; then
        local total_diff=$((current_total - original_total))
        local total_percent=$(echo "scale=1; ($total_diff * 100.0) / $original_total" | bc -l 2>/dev/null || echo "N/A")
        
        echo ""
        if [[ $total_diff -gt 0 ]]; then
            print_color $RED "Total Difference: +$total_diff lines (+${total_percent}%)"
        elif [[ $total_diff -lt 0 ]]; then
            print_color $GREEN "Total Difference: $total_diff lines (${total_percent}%)"
        else
            print_color $BLUE "Total Difference: No change"
        fi
    fi
    
    # Show file breakdowns if requested
    if [[ "$show_details" == "true" || "$show_details" == "--details" ]]; then
        if [[ -d "$current_dir/crates" ]]; then
            show_breakdown "$current_dir/crates" "Current crates"
        fi
        if [[ -d "$original_dir/crates" ]]; then
            show_breakdown "$original_dir/crates" "Original crates"
        fi
    fi
    
    echo ""
    print_color $BLUE "Excluded from count:"
    echo "  - target/ directories (build artifacts)"
    echo "  - _wb/ directories (working files)" 
    echo "  - _dev/ directories (development files)"
    echo "  - Files with 'generated' in name"
    echo "  - build/ and out/ directories"
    echo ""
    print_color $YELLOW "Usage: $0 [directory] [--details]"
    print_color $YELLOW "  directory: Root directory to analyze (default: current)"
    print_color $YELLOW "  --details: Show file breakdown for largest files"
}

# Check if bc is available for percentage calculations
if ! command -v bc &> /dev/null; then
    print_color $YELLOW "Note: 'bc' not found. Percentage calculations will show 'N/A'"
fi

# Run main function
main "$@"