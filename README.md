# bob
bob (the builder) (no affiliation) is an extremely dumb "CI."  he has one job:
pulling (primarily rust) repositories down and building docker images from
them.

bob the builder (the show)'s target audience is children and so it only talks
in terms a child can understand.  likeways, please expect bob (the CI) to only
be capable of what a 5 year old with a laptop and a very patient parent can do.

**worth noting:  though bob is async, it only processes one repo at a time.**

## why
my macbook is an M1.  generally it can build things for amd64 just fine.  rust
is an exception, due to cargo being a memory hog and docker buildx being
experimental and sometimes outright bad.

i could just use github actions to build docker images for amd64 but this
sounded miles more fun to make.

## config (env)
| name                  | what is it                   | default      |
|-----------------------|------------------------------|--------------|
| WATCH_USERS           | users to watch               | None (panic) |
| DOCKER_USERNAME       | docker registry username     | None (panic) |
| DOCKER_TOKEN          | docker registry token        | None (panic) |
| GITHUB_TOKEN          | github personal access token | None (panic) |
| DISCORD_WEBHOOK_ID    | id of discord webhook        | None (panic) |
| DISCORD_WEBHOOK_TOKEN | token of discord webhook     | None (panic) |

## license
see [LICENSE](LICENSE)

## support
there is none
