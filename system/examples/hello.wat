(module
    (import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
    (memory (export "memory") 1)
    (data (i32.const 8) "Hello from Nova! 🚀\n")

    (func $main (export "_start")
        (i32.store (i32.const 0) (i32.const 8))  ;; iov.base
        (i32.store (i32.const 4) (i32.const 22)) ;; iov.len (length of string)

        (call $fd_write
            (i32.const 1) ;; file_descriptor: 1 (stdout)
            (i32.const 0) ;; *iovs
            (i32.const 1) ;; iovs_len
            (i32.const 20) ;; *nwritten (place to store result)
        )
        drop ;; discard result of fd_write
    )
)
