.PHONY: run, test

run:
	@echo "Running code for task $(TASK_NUM)..."
	@cargo run --bin t$(TASK_NUM)

test:
	@echo "Running tests for task $(TASK_NUM)..."
	@cargo test --test $(TASK_NUM)/tests.rs
