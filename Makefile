hotel_db = tests/fixtures/hotel-db.yml
fixture = tests/fixtures/input.txt
bench_fixture = tests/fixtures/big-input.txt
docker_image = hotel_docker_developer_environment

help:
	$(info -Targets -----------------------------------------------------------------------------)
	$(info answers                      | produce answers expected by the challenge)
	$(info answers-in-docker            | as above, but uses docker for those without Rust)
	$(info -- Use docker for all dependencies - run make interactively from there ----------------)
	$(info interactive-developer-environment-in-docker | gives you everything you need to run all targets)
	$(info -Development Targets -----------------------------------------------------------------)
	$(info lint                         | run lints with clippy)
	$(info benchmark                    | just for fun, really)
	$(info profile                      | only on linux - run callgrind and annotate it)
	$(info journey-tests                | run all stateless journey test)
	$(info continuous-journey-tests     | run all stateless journey test whenever something changes)

always:

interactive-developer-environment-in-docker:
	docker build -t $(docker_image) - < etc/developer.Dockerfile
	docker run -v $$PWD:/volume -w /volume -it $(docker_image)

target/debug/hotel: always
	cargo build

target/release/hotel: always
	cargo build --release

lint:
	cargo clippy

profile: target/release/hotel
	valgrind --callgrind-out-file=callgrind.profile --tool=callgrind  $< $(hotel_db) $(bench_fixture) >/dev/null
	callgrind_annotate --auto=yes callgrind.profile

benchmark: target/release/hotel
	hyperfine '$< $(hotel_db) $(bench_fixture)'

journey-tests: target/debug/hotel
	./tests/stateless-journey.sh $<

answers: target/debug/hotel
	$< $(hotel_db) $(fixture)

answers-in-docker:
	docker run -v $$PWD:/volume -w /volume rust make answers

continuous-journey-tests:
	watchexec $(MAKE) journey-tests

