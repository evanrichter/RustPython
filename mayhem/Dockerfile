# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd fuzz && cargo +nightly -Z sparse-registry fuzz build && popd" && \
    mv fuzz/target/x86_64-unknown-linux-gnu/release/vm /vm && \
    echo done

RUN echo building non-instrumented harnesses && \
    bash -c "pushd fuzz && cargo +nightly -Z sparse-registry fuzz build --disable-instrumentation && popd" && \
    mv fuzz/target/x86_64-unknown-linux-gnu/release/vm /vm_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /vm  /vm_no_inst /
RUN mkdir /testsuite
COPY --from=builder /src/extra_tests/snippets/ /testsuite/
