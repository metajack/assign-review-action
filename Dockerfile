FROM rust:latest

LABEL com.github.actions.name="Assign Review"
LABEL com.github.actions.description="Assign reviewers when PRs are opened or when requested with r?"
LABEL com.github.actions.icon="crosshair"
LABEL com.github.actions.color="purple"

COPY assign_review /tmp

RUN cd /tmp/assign_review && cargo install

ENTRYPOINT ["assign_review"]
