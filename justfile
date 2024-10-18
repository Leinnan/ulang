main:
    cargo run --features="build-binary" -- samples/return_2.c

to_asm:
    cargo run --features="build-binary" -- samples/return_2.c return_2.s

to_asm_gcc:
    gcc -arch x86_64 -S -O -fno-asynchronous-unwind-tables -fcf-protection=none samples/return_2.c

run:
    gcc -arch x86_64 -masm=intel return_2.s -o return_2
    ./return_2 || echo $?

run_other: main
    clang -arch x86_64 samples/return_2.s -o return_2
    ./return_2 || echo $?

preprocess:
    gcc -E -P samples/return_2.c -o return_2.i

all_files:
    for filename in `ls ../writing-a-c-compiler-tests/tests/chapter_3/valid`; do \
      cargo run --release --features="build-binary" --  ../writing-a-c-compiler-tests/tests/chapter_3/valid/$filename; \
    done
