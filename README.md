# multiplayer-api
Monolithic API in Rust for multiplayer lobby / matchmaking


docker build -t api .

docker run -it -p 3000:3000 --name api --network=environment_hdr api:latest