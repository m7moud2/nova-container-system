(module
    (import "env" "nova_get_pid" (func $get_pid (result i32)))
    (import "env" "nova_send" (func $send (param i32 i32 i32))) 
    (import "env" "nova_recv" (func $recv (param i32 i32) (result i32))) 
    (import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
    
    (memory (export "memory") 1)
    
    (data (i32.const 100) "Hello from PID 1\n") 
    (data (i32.const 200) "Received: ")         
    
    (func $main (export "_start")
        (local $pid i32)
        (local $bytes i32)
        
        (call $get_pid)
        (local.set $pid)
        
        (local.get $pid)
        (i32.const 1)
        (i32.eq)
        (if
            (then
                ;; I am PID 1. Send to PID 0.
                (call $send (i32.const 0) (i32.const 100) (i32.const 17))
            )
        )
        
        (local.get $pid)
        (i32.const 0)
        (i32.eq)
        (if
            (then
                ;; I am PID 0. Wait for message.
                (call $recv (i32.const 300) (i32.const 50))
                (local.set $bytes)
                
                ;; Print "Received: "
                (i32.store (i32.const 0) (i32.const 200)) ;; iov.base
                (i32.store (i32.const 4) (i32.const 10))  ;; iov.len
                (call $fd_write (i32.const 1) (i32.const 0) (i32.const 1) (i32.const 20))
                drop
                
                ;; Print received message
                (i32.store (i32.const 0) (i32.const 300)) ;; iov.base
                (i32.store (i32.const 4) (local.get $bytes)) ;; iov.len
                (call $fd_write (i32.const 1) (i32.const 0) (i32.const 1) (i32.const 20))
                drop
            )
        )
    )
)
