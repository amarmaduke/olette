all:
	cargo install wasm-pack 2> /dev/null
	wasm-pack build
	cd www && npm install
	npm link pkg
	cd www && npm run build
