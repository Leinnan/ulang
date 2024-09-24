to_asm:
    gcc -S -O -fno-asynchronous-unwind-tables -fcf-protection=none samples/return_2.c

run: to_asm
    gcc return_2.s -o return_2
    ./return_2 || echo $?

preprocess:
    gcc -E -P samples/return_2.c -o return_2.i

all_files:
    for filename in `ls ../writing-a-c-compiler-tests/tests/chapter_1/valid`; do \
      cargo run --release --  ../writing-a-c-compiler-tests/tests/chapter_1/valid/$filename; \
    done
