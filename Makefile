watch:
	docker compose -f docker-compose.test.yml up -d && docker compose -f docker-compose.test.yml exec store_auth_test bash

watch-build:
	docker compose -f docker-compose.test.yml build

watch-done:
	docker compose -f docker-compose.test.yml down

pg-test:
	docker-compose -f docker-compose.test.yml exec postgres psql -U docker
