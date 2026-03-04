#!/bin/bash
# Deploy Nova Website to GitHub Pages

set -e

echo "🌐 Deploying Nova Website to GitHub Pages"
echo "=========================================="
echo ""

# Check if GitHub username is set
if [ -z "$GITHUB_USER" ]; then
    echo "❓ Enter your GitHub username:"
    read GITHUB_USER
fi

echo "📋 GitHub User: $GITHUB_USER"
echo ""

# Initialize git if needed
if [ ! -d ".git" ]; then
    echo "🔧 Initializing git..."
    git init
fi

# Add all files
echo "📦 Adding files..."
git add .

# Commit
echo "💾 Committing..."
git commit -m "Nova website - Production ready" || true

# Add remote
echo "🔗 Adding GitHub remote..."
git remote add origin "https://github.com/$GITHUB_USER/nova-container-system.git" 2>/dev/null || \
git remote set-url origin "https://github.com/$GITHUB_USER/nova-container-system.git"

# Push to gh-pages
echo "🚀 Pushing to gh-pages..."
git branch -M gh-pages
git push -u origin gh-pages --force

echo ""
echo "✅ Website deployed!"
echo ""
echo "🌐 Your website will be available at:"
echo "   https://$GITHUB_USER.github.io/nova-container-system"
echo ""
echo "⏳ Note: It may take a few minutes for GitHub Pages to build"
echo ""
echo "📋 Enable GitHub Pages:"
echo "   1. Go to: https://github.com/$GITHUB_USER/nova-container-system/settings/pages"
echo "   2. Source: gh-pages branch"
echo "   3. Save"
echo ""
