#!/bin/bash

set -e

cargo doc
echo '<meta http-equiv=refresh content=0;url=geo/index.html>' > target/doc/index.html
sudo pip install ghp-import
ghp-import -n target/doc
git push -qf https://${TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages
