main:
    cargo run --features="build-binary" -- samples/return_2.c

to_asm:
    gcc -S -O -fno-asynchronous-unwind-tables -fcf-protection=none samples/return_2.c

run:
    gcc return_2.s -o return_2
    ./return_2 || echo $?

preprocess:
    gcc -E -P samples/return_2.c -o return_2.i

all_files:
    for filename in `ls ../writing-a-c-compiler-tests/tests/chapter_3/valid`; do \
      cargo run --release --features="build-binary" --  ../writing-a-c-compiler-tests/tests/chapter_3/valid/$filename; \
    done
