#!/bin/bash
# Nova - Quick Deploy Script
# This script helps you deploy Nova to GitHub quickly

set -e

echo "🚀 Nova - Quick Deploy to GitHub"
echo "================================"
echo ""

# Check if GitHub username is set
if [ -z "$GITHUB_USER" ]; then
    echo "❓ Enter your GitHub username:"
    read GITHUB_USER
fi

echo "📋 GitHub User: $GITHUB_USER"
echo ""

# Confirm
echo "⚠️  This will:"
echo "   1. Update install.sh with your username"
echo "   2. Add GitHub remote"
echo "   3. Push code to GitHub"
echo ""
echo "Continue? (y/n)"
read -r response

if [[ ! "$response" =~ ^[Yy]$ ]]; then
    echo "❌ Cancelled"
    exit 1
fi

echo ""
echo "🔧 Step 1: Updating install.sh..."
sed -i.bak "s/YOUR_USERNAME/$GITHUB_USER/g" install.sh
rm -f install.sh.bak
git add install.sh
git commit -m "Update install.sh with GitHub username" || true

echo ""
echo "🔧 Step 2: Updating RELEASE_NOTES.md..."
sed -i.bak "s/YOUR_USERNAME/$GITHUB_USER/g" RELEASE_NOTES.md
rm -f RELEASE_NOTES.md.bak
git add RELEASE_NOTES.md
git commit -m "Update release notes with GitHub username" || true

echo ""
echo "🔧 Step 3: Adding GitHub remote..."
git remote add origin "https://github.com/$GITHUB_USER/nova-container-system.git" 2>/dev/null || \
git remote set-url origin "https://github.com/$GITHUB_USER/nova-container-system.git"

echo ""
echo "🔧 Step 4: Pushing to GitHub..."
git branch -M main
git push -u origin main

echo ""
echo "✅ Code pushed to GitHub!"
echo ""
echo "📋 Next steps:"
echo ""
echo "1. Create a release on GitHub:"
echo "   https://github.com/$GITHUB_USER/nova-container-system/releases/new"
echo ""
echo "2. Build and upload binary:"
echo "   cargo build --release"
echo "   # Upload: target/release/nova_container_system"
echo ""
echo "3. Deploy website:"
echo "   cd ../nova_website_bootstrap"
echo "   ./deploy_website.sh"
echo ""
echo "🎉 Good luck with your launch!"
