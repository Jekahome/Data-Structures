.PHONY: all commit clean

all: commit

commit:
	@if ! git diff-index --quiet HEAD --; then \
		echo ">>> Обнаружены изменения, делаю коммит..."; \
		git add .; \
		git commit --allow-empty-message -m ""; \
		git push; \
	else \
		echo ">>> Нет изменений для коммита."; \
	fi

clean:
	git clean -fd