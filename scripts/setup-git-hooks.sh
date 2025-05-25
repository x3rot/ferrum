#!/bin/bash
# This script sets up Git configuration for the project

# Set the commit message template
git config --local commit.template .gitmessage

# Install pre-commit hooks if pre-commit is available
if command -v pre-commit >/dev/null 2>&1; then
  echo "Setting up pre-commit hooks..."
  pre-commit install
else
  echo "pre-commit is not installed. To install, run: pip install pre-commit"
  echo "After installation, run: pre-commit install"
fi

echo "Git configuration complete!"
echo "For better commit messages, use the template with: git commit"
echo "Pre-commit hooks will run automatically on each commit."
