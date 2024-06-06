(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func (result i32)))
  (type (;2;) (func))
  (import "env" "drive" (func $drive (type 0)))
  (import "env" "damage" (func $damage (type 1)))
  (import "env" "speed" (func $speed (type 1)))
  (func $__wasm_call_ctors (type 2))
  (func $botinit (type 1) (result i32)
    (local i32 i32)
    i32.const 90
    local.set 0
    i32.const 10
    local.set 1
    local.get 0
    local.get 1
    call $warsdk::drive::h5027af460a94d5a1
    drop
    loop (result i32)  ;; label = @1
      call $warsdk::damage::hf4a2552f8d6655b4
      drop
      call $warsdk::speed::h85f2004cb004fe6d
      drop
      br 0 (;@1;)
    end)
  (func $warsdk::drive::h5027af460a94d5a1 (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    local.set 2
    i32.const 16
    local.set 3
    local.get 2
    local.get 3
    i32.sub
    local.set 4
    local.get 4
    global.set 0
    local.get 4
    local.get 0
    i32.store offset=8
    local.get 4
    local.get 1
    i32.store offset=12
    local.get 4
    i32.load offset=8
    local.set 5
    local.get 4
    i32.load offset=12
    local.set 6
    local.get 5
    local.get 6
    call $drive
    local.set 7
    i32.const 16
    local.set 8
    local.get 4
    local.get 8
    i32.add
    local.set 9
    local.get 9
    global.set 0
    local.get 7
    return)
  (func $warsdk::damage::hf4a2552f8d6655b4 (type 1) (result i32)
    (local i32)
    call $damage
    local.set 0
    local.get 0
    return)
  (func $warsdk::speed::h85f2004cb004fe6d (type 1) (result i32)
    (local i32)
    call $speed
    local.set 0
    local.get 0
    return)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048610))
  (global (;2;) i32 (i32.const 1048610))
  (global (;3;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "__indirect_function_table" (table 0))
  (export "__heap_base" (global 1))
  (export "__data_end" (global 2))
  (export "botinit" (func $botinit))
  (export "__rustc_debug_gdb_scripts_section__" (global 3))
  (data (;0;) (i32.const 1048576) "\01gdb_load_rust_pretty_printers.py\00"))
