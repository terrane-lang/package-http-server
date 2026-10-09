# Annotated catalog demo

A real HTTP service built with class-based annotated APIs and ordinary typed Terrane models.
The catalog API provides item lookup, search, and stateless order quote pricing; a separate
site serves HTML pages, Stoplight Elements documentation, and static assets.

From the Terrane repository root:

```console
sh packages/http-server/build-consumer.sh
(cd packages/http-server/demo && ../../../target/debug/terrane run .)
```

The default address is `127.0.0.1:3187`. Pass a different address as the application's first
argument when launching its built executable.

```console
curl http://127.0.0.1:3187/
curl http://127.0.0.1:3187/about
curl http://127.0.0.1:3187/api/health
curl 'http://127.0.0.1:3187/api/items/1?detailed=true'
curl 'http://127.0.0.1:3187/api/search?term=Terrane+notebook&limit=2'
curl -H 'Content-Type: application/json' -d '{"item-id":1,"quantity":3}' http://127.0.0.1:3187/api/orders/quote
curl http://127.0.0.1:3187/api/openapi.json
curl http://127.0.0.1:3187/docs/
curl http://127.0.0.1:3187/assets/site.css
```

The notebook costs 1200 cents, so three cost 3600 cents. The order body is decoded into an
`order-input` model; quote responses use `quote-output`. Invalid model bodies and malformed
scalar path/query parameters return 422 before a handler runs. Quotes are calculated from the
catalog and are not persisted.
Missing model fields retain their Terrane defaults; omitted `quantity` is zero. The codecs
do not invent domain constraints such as positive quantities. `src/models.trn` defines the
models, `src/api.trn` defines the API class, `src/site.trn` defines pages/static annotations,
and `src/main.trn` only starts the configured application.

The explicit `[consumers.http-server]` manifest selects every model class and field, route
group, API/site class, endpoint method, and annotated parameter. `mount-annotated` mounts the
selected classes, and the `catalog-api` group prefix is `/api`; OpenAPI is at
`/api/openapi.json` and its schema paths include that prefix. The `site` group disables
OpenAPI documentation for its HTML methods. It serves `public/docs` at `/docs` and
`public/assets` at `/assets`, relative to the process working directory. Run the demo with
`demo` as cwd (the Docker image uses `/app`) so these paths resolve correctly. Generated
files and the consumer executable live beneath `.trn` and must not be edited by hand.

The runtime uses Debian Trixie: the locally prebuilt binary requires glibc 2.38 or newer,
which Debian Bookworm does not provide. Build the binary for the container's Linux architecture.

## Behavioral tests

```console
sh packages/http-server/build-consumer.sh
artifact=$(target/debug/terrane build packages/http-server/demo)
python packages/http-server/tests/integration/test_demo.py "$artifact"
python packages/http-server/tests/integration/test_consumer.py
target/debug/terrane test packages/http-server
```

The HTTP suite starts the real built application with `demo` as its working directory and
checks typed bodies and outputs, defaults, HTML pages, static documentation and assets, and
the mounted OpenAPI schema. The consumer suite checks malformed declarations through the
real compiler/consumer pipeline.

## Docker deployment

From the repository root, build the Terrane consumer and demo executable, stage the binary
inside the ignored Docker context, then build and start the Compose service:

```console
sh packages/http-server/demo/scripts/build-image.sh
```

The script uses the `TERRANE` environment variable when set, otherwise
`target/debug/terrane`. It copies the artifact path printed by `terrane build` into
`demo/.docker/app`; Docker receives only that prebuilt executable and the demo's `public/`
assets. The image does not contain or run the Terrane compiler. Compose publishes the service
on host port `3187` by default; override it with `HTTP_SERVER_PORT`, for example:

```console
HTTP_SERVER_PORT=8080 sh packages/http-server/demo/scripts/build-image.sh
```

To stop the service, run `docker compose -f packages/http-server/demo/compose.yaml down`
from the repository root.
