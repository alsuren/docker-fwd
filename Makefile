

.PHONY: push
push: .git/refs/remotes/dof/incoming

.git/refs/remotes/dof:
	ssh localhost mkdir -p ./$(PWD)
	ssh localhost git init ./$(PWD)
	git remote add dof --fetch localhost:./$(PWD)

.git/refs/remotes/dof/incoming: .git/refs/remotes/dof .git/refs/heads/*
	git push dof HEAD:incoming
	ssh localhost "cd ./$(PWD) && git merge --ff-only incoming"
