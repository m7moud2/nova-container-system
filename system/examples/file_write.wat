(module
    (import "wasi_snapshot_preview1" "path_open" (func $path_open (param i32 i32 i32 i32 i32 i64 i64 i32 i32) (result i32)))
    (import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
    (import "wasi_snapshot_preview1" "fd_close" (func $fd_close (param i32) (result i32)))

    (memory (export "memory") 1)
    
    (data (i32.const 100) "hello.txt")
    (data (i32.const 200) "Hello from Wasm Disk! 💾\n")

    (func $main (export "_start")
        (local $fd i32)
        
        ;; Open file "/data/hello.txt" (relative to preopened dir)
        ;; We assume FD 3 is the preopened directory if it's the first one.
        
        (call $path_open
            (i32.const 3)    ;; dirfd (preopened dir usually starts at 3)
            (i32.const 0)    ;; dirflags
            (i32.const 100)  ;; path ptr ("hello.txt")
            (i32.const 9)    ;; path len
            (i32.const 9)    ;; oflags (1=CREAT | 8=TRUNC)
            (i64.const 64)   ;; fs_rights_base (write = 64)
            (i64.const 0)    ;; fs_rights_inheriting
            (i32.const 0)    ;; fdflags
            (i32.const 300)  ;; result ptr (place to store FD)
        )
        
        ;; Check error code (on stack). 0 = success.
        (if (i32.ne (i32.const 0))
            (then
                (unreachable) ;; Trap if failed
            )
        )
        
        (i32.load (i32.const 300))
        (local.set $fd)

        ;; Construct iovec at address 400
        (i32.store (i32.const 400) (i32.const 200)) ;; iov.buf = address of string "Hello..."
        (i32.store (i32.const 404) (i32.const 24))  ;; iov.len = length of string "Hello from Wasm Disk! 💾\n" (approx 24 bytes)

        ;; Write to file
        (call $fd_write
            (local.get $fd)
            (i32.const 400) ;; iovec array pointer
            (i32.const 1)   ;; iov array length (1 iovec)
            (i32.const 500) ;; nwritten ptr
        )
        drop 
        
        ;; Close
        (call $fd_close (local.get $fd))
        drop
    )
)
