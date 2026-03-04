(module
    (memory 1) ;; Start with 1 page (64KB)
    
    (func $main (export "_start")
        ;; Try to grow by 200 pages (12.8 MB)
        ;; If limit is 1MB, this returns -1.
        (if (i32.eq (memory.grow (i32.const 200)) (i32.const -1))
            (then
                ;; Success! Limit worked. Trap to signal "OOM prevented".
                (unreachable)
            )
        )
        
        ;; If we are still here, try to access memory at offset that would be valid if grow succeeded
        ;; but invalid if it failed/was limited.
        ;; Actually, let's just assert that we allocated too much if we want to crash.
        ;; But Wasmtime might verify resource limits at instantiation or grow time.
        
        ;; Let's define specific behavior: we want to FAIL instantiating if we ask for too much,
        ;; OR memory.grow fails.
    )
)
