start:
				cargo web start --auto-reload
build:
				rm -rf docs; mkdir docs; cargo web deploy --release -o ./docs