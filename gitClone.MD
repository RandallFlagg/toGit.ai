# Git Repository Types and Commands

## 1. Regular Repository
# A standard Git repository with a working directory.
git init my-regular-repo
cd my-regular-repo

## 2. Bare Repository
# A repository without a working directory, suitable for use as a remote repository.
git init --bare my-bare-repo.git

## 3. Shallow Repository
# A repository with limited commit history to reduce data size.
git clone --depth=1 <repository-url> my-shallow-repo

## 4. Mirror Repository
# A bare repository that mirrors another repository, fetching and pushing all refs.
git clone --mirror <repository-url> my-mirror-repo.git

## 5. Submodule Repository
# Adding another Git repository as a subdirectory in your main repository.
git submodule add <repository-url> path/to/submodule

## 6. Worktree Repository
# Creating additional working directories attached to a single repository.
git worktree add <path> <branch>
