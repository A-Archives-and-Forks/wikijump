FROM valkey/valkey:alpine

EXPOSE 6379
COPY ./valkey.conf /usr/local/etc/valkey.conf
CMD ["valkey-server", "/usr/local/etc/valkey.conf"]
