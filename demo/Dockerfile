FROM debian:trixie-slim

RUN apt-get update \
    && apt-get install --no-install-recommends -y ca-certificates curl libgcc-s1 \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system --gid 10001 app \
    && useradd --system --uid 10001 --gid app --home-dir /app --shell /usr/sbin/nologin app

WORKDIR /app
COPY --chown=app:app .docker/app /app/app
COPY --chown=app:app scripts/start.sh /app/start.sh
COPY --chown=app:app healthcheck /app/healthcheck
COPY --chown=app:app public/ /app/public/
RUN chmod 0555 /app/app /app/start.sh /app/healthcheck

USER app:app
ENV PORT=3187 ADDRESS=0.0.0.0
EXPOSE 3187

HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
    CMD ["/app/healthcheck"]
ENTRYPOINT ["/app/start.sh"]
