IN_DIRECTORY_VERSION=in-directory-1.0.1-x86_64-unknown-linux-gnu
IN_DIRECTORY_URL=https://github.com/alsuren/cargo-quickinstall/releases/download/$(IN_DIRECTORY_VERSION)/$(IN_DIRECTORY_VERSION).tar.gz

all: push

# Make the `dof` remote, if it doesn't exist.
.git/refs/remotes/dof:
	ssh localhost mkdir -p ./$(PWD)
	ssh localhost git init ./$(PWD)
	git remote add dof --fetch localhost:./$(PWD)

# push changes to `dof` remote, and check them out.
.git/refs/remotes/dof/incoming: .git/refs/remotes/dof .git/refs/heads/* .done-install-in-directory
	git push dof HEAD:incoming
	ssh localhost ./bin/in ./$(PWD) git merge --ff-only incoming
	touch -c $@

# push changes to `dof` remote
push: .git/refs/remotes/dof/incoming

.git/hooks/post-commit:
	echo "make push" > .git/hooks/post-commit
	chmod u+x .git/hooks/post-commit

# Install post-commit hook
post-commit: .git/hooks/post-commit

# TODO: think of a better name for this marker file
install-in-directory: .done-install-in-directory
.done-install-in-directory:
	# gather statistics
	curl https://warehouse-clerk-tmp.vercel.app/api/crate/$(IN_DIRECTORY_VERSION).tar.gz > /dev/null
	ssh localhost mkdir -p ./bin
	curl -fsSL $(IN_DIRECTORY_URL) \
		| ssh localhost tar -xzvvf - -C ./bin
	touch .done-install-in-directory
