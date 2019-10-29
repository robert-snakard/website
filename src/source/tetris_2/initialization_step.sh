#1/usr/bin/env bash

mkdir $PROJECT_FOLDER
Cd project_folder

cargo init --lib
rustup target add wasm32-unknown-unknown

git init
git add .gitignore
git add *
git commit -m "First Commit"

ls -a

git remote add origin $SSH_GIT_URL
git remote -v
git push --set-upstream origin master
