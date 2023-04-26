FROM rust:1.67

COPY . /server
WORKDIR /server
RUN 
RUN cargo install --path .
CMD cargo run
EXPOSE 8081