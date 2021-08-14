all: push

# Make the `dof` remote, if it doesn't exist.
.git/refs/remotes/dof:
	ssh localhost mkdir -p ./$(PWD)
	ssh localhost git init ./$(PWD)
	git remote add dof --fetch localhost:./$(PWD)

# push changes to `dof` remote, and check them out
.git/refs/remotes/dof/incoming: .git/refs/remotes/dof .git/refs/heads/*
	git push dof HEAD:incoming
	ssh localhost "cd ./$(PWD) && git merge --ff-only incoming"

# push changes to `dof` remote
.PHONY: push
push: .git/refs/remotes/dof/incoming
