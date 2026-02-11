FROM rust:1.85.0-bookworm

WORKDIR /work
ENTRYPOINT ["/bin/bash", "-lc"]
CMD ["scripts/release_build.sh"]
