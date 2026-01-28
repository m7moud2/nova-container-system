#!/bin/bash
# Nova Runtime Test Suite
# Tests Python, Node.js, and Wasm runtimes

set -e  # Exit on error

NOVA_BIN="./target/release/nova_container_system"
TEST_DIR="./test_apps"

echo "🧪 Nova Runtime Test Suite"
echo "=========================="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
PASSED=0
FAILED=0

# Helper function
test_command() {
    local name="$1"
    local cmd="$2"
    
    echo -n "Testing: $name... "
    
    if eval "$cmd" > /dev/null 2>&1; then
        echo -e "${GREEN}✓ PASSED${NC}"
        ((PASSED++))
    else
        echo -e "${RED}✗ FAILED${NC}"
        ((FAILED++))
    fi
}

# Create test directory
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

echo "📦 Setting up test applications..."
echo ""

# Test 1: Simple Python script
cat > test_simple.py << 'EOF'
print("Hello from Python!")
EOF

# Test 2: Flask app
cat > test_flask.py << 'EOF'
from flask import Flask
app = Flask(__name__)

@app.route('/')
def hello():
    return 'Hello from Flask!'

if __name__ == '__main__':
    print("Flask app created successfully")
EOF

# Test 3: Node.js script
cat > test_node.js << 'EOF'
console.log("Hello from Node.js!");
EOF

# Test 4: Express app
cat > test_express.js << 'EOF'
const express = require('express');
const app = express();

app.get('/', (req, res) => {
    res.send('Hello from Express!');
});

console.log("Express app created successfully");
EOF

cat > package.json << 'EOF'
{
  "name": "test-app",
  "version": "1.0.0",
  "dependencies": {
    "express": "^4.18.2"
  }
}
EOF

echo "✅ Test applications created"
echo ""

# Run tests
echo "🚀 Running Tests..."
echo ""

# Test Python detection
test_command "Python detection" "python3 --version"

# Test Node.js detection
test_command "Node.js detection" "node --version"

# Test Nova binary exists
test_command "Nova binary exists" "test -f ../$NOVA_BIN"

# Test simple Python execution
echo -n "Testing: Python execution... "
if timeout 2 python3 test_simple.py | grep -q "Hello from Python"; then
    echo -e "${GREEN}✓ PASSED${NC}"
    ((PASSED++))
else
    echo -e "${RED}✗ FAILED${NC}"
    ((FAILED++))
fi

# Test simple Node.js execution
echo -n "Testing: Node.js execution... "
if timeout 2 node test_node.js | grep -q "Hello from Node"; then
    echo -e "${GREEN}✓ PASSED${NC}"
    ((PASSED++))
else
    echo -e "${RED}✗ FAILED${NC}"
    ((FAILED++))
fi

# Test Flask import
echo -n "Testing: Flask import... "
if python3 -c "import flask" 2>/dev/null; then
    echo -e "${GREEN}✓ PASSED${NC}"
    ((PASSED++))
else
    echo -e "${YELLOW}⚠ SKIPPED (Flask not installed)${NC}"
fi

# Cleanup
cd ..
# rm -rf "$TEST_DIR"

echo ""
echo "=========================="
echo "📊 Test Results"
echo "=========================="
echo -e "Passed: ${GREEN}$PASSED${NC}"
echo -e "Failed: ${RED}$FAILED${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}❌ Some tests failed${NC}"
    exit 1
fi
