(module
  (import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  (data (i32.const 8) "{\"status\":\"ok\"}\n")
  (func (export "run")
    (i32.store (i32.const 0) (i32.const 1))
    (i32.store (i32.const 4) (i32.const 8))
    (i32.store (i32.const 12) (i32.const 16))
    (drop (call $fd_write (i32.const 0) (i32.const 4) (i32.const 1) (i32.const 20)))))
