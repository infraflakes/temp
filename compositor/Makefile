.PHONY: build clean

build: clean
	@mkdir -p bin
	dagger call build --source=. export --path=./bin/srcom

clean:
	rm -rf bin/
	rm -rf build/
