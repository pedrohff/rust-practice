# Helps reloading a file when changed, compiling it, executing it's result and deleting it after used.
rust() {
	name=$(basename $1 .rs)
	echo $1 | entr -cp sh -c "rustc $@ && ./$name && rm $name"
}

