run:
	cargo run

run-release:
	cargo run --release

bench:
	cargo bench

statistics:
	cargo run --release --features print_statistics

clean:
	rm -f flamegraph.svg
	rm -f perf.data*
	rm -rf target

flamegraph:
ifeq (,$(shell cargo flamegraph -V))
 	$(error "Flamegraph is not installed, consider installing it via `cargo install flamegraph`")
else
	CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph
endif