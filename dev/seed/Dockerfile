FROM oven/bun:1.4.4 AS bun
FROM postgres:18
COPY --from=bun /usr/local/bin/bun /usr/local/bin/bun
WORKDIR /app
COPY web/src/lib/fakeData.ts web/src/lib/fakeData.ts
COPY dev/seed/export.ts dev/seed/export.ts
COPY dev/seed/run.sh /usr/local/bin/seed-dev
ENTRYPOINT ["sh", "/usr/local/bin/seed-dev"]
