FROM rust:1.83.0-bookworm
WORKDIR /app
COPY target/release /app/server

##### default envs
ENV APP_RUN_SERVER yes
ENV APP_ENV prod
ENV APP_PORT 8080
ENV APP_DEBUG no

###### entrypoint
COPY devops/build/api/entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/entrypoint.sh

#####
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
