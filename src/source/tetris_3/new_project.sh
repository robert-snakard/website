#!/usr/bin/env bash

mkdir tetris
cd tetris

cargo init
git init

git add .gitignore
git add *

git commit -m "first commit"

git remote add origin $REMOTE_URL
git remote -v

git push -u origin master
