(module
    (func $square (param i32) (result i32)
        local.get 0
        local.get 0
        i32.mul
    )
    (export "square" (func $square))

    (func $echo (param i32) (result i32)
        local.get 0
    )
    (export "echo" (func $echo))

    (func $substract (param $p1 i32) (param $p2 i32) (result i32)
        local.get $p1
        local.get $p2
        i32.sub
    )
    (export "echo" (func $substract))
)