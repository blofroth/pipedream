# Pipedream - an experiment in composeability

This project implements simplistic REST service equivalents to popular UNIX commands
that lends themselves to composability by piping (head, grep, cut, tr, sed, sort, ...).

## Goals 

The main goal is for me to learn Rust better :). Apart from that some directions that can be explored are:
* Can web services achieve a similar level of composeability as UNIX commands?
  * Is that even desirable from a performance point of view?
* Be a test bed for cloud orchestration tools (OpenShift/Kubernetes), scaling, rolling deploys

Limitations: 
* UTF-8 input and output

Non-goals:
* Achieve any sort of feature completeness of the UNIX command equivalents
* Aim to solve any particular problem in the best way
* Be appropriate to deploy in any environment where malicious users are present

## Approach

Each operation is exposed as a REST service, and accepts text data in the HTTP request body. Arguments are provided as query string parameters. Output is returned in the HTTP response body.

The `/pipe` operation allows triggering a chain of operations that are pipelined together. It expects one command per line. The format of providing arguments is likely to change. See `file/commands.txt`.

The `files` operation serves files statically from the `files` directory. This is useful as a source
to the `wget` command in case you have a slow internet connection or a lack of imagination of what data to process.

## Try it out

Start the server:

    cargo run 

Post some commands:

    curl -X POST http://localhost:8000/pipe --data-binary @files/commands.txt

Try an individual operation:

    curl -X POST http://localhost:8000/head?n=2 --data-binary @files/lines.txt