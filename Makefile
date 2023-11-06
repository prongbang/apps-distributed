dev:
	vercel dev

deploy:
	vercel deploy

pull_env:
	vercel env pull .env.development.local

get:
	curl -X POST -d '{"platform": "android", "version": "2.9.1", "build": "1", "action": "get"}' \
	https://localhost:3000/api/distributed?key=API_KEY

set:
	curl -X POST -d '{"platform": "android", "version": "2.9.1", "build": "1", "action": "set"}' \
	http://localhost:3000/api/distributed?key=API_KEY
