(module
  (memory (import "js" "mem") 1)
  (func (export "upper") (param $ptr i32) (param $len i32) (result i32)
    (local $end i32)
    (local $upper i32)
    (local.set $end
      (i32.add
        (local.get $ptr)
        (i32.mul
          (local.get $len)
          (i32.const 1))))

    (block $break
      (loop $top
        (br_if $break
          (i32.eq
            (local.get $ptr)
            (local.get $end)))  

        local.get $ptr
        i32.load8_u
        i32.const 32
        i32.sub

        local.set $upper  

        local.get $upper  
        i32.const 64

        i32.gt_u

        (if
          (then
             local.get $ptr
             local.get $upper
     
            i32.store8
          )
        )
        
        (local.set $ptr
          (i32.add
            (local.get $ptr)
            (i32.const 1)))
        (br $top)
      )
    )
    (local.get $end)
  )
)