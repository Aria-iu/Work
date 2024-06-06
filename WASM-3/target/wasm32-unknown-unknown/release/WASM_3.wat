(module $WASM_3.wasm
  (type (;0;) (func (param i32)))
  (type (;1;) (func (param i32) (result i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func))
  (type (;4;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32)))
  (type (;6;) (func (param i32 i32 i32)))
  (type (;7;) (func (param i32 i32 i32 i32)))
  (type (;8;) (func (result i32)))
  (type (;9;) (func (param i32 i32 i32) (result i32)))
  (import "__wbindgen_placeholder__" "__wbindgen_describe" (func $_ZN12wasm_bindgen19__wbindgen_describe17h972d0852984605d7E (type 0)))
  (import "__wbindgen_externref_xform__" "__wbindgen_externref_table_grow" (func $_ZN12wasm_bindgen9externref31__wbindgen_externref_table_grow17h55902b3cc6f95e75E (type 1)))
  (import "__wbindgen_externref_xform__" "__wbindgen_externref_table_set_null" (func $_ZN12wasm_bindgen9externref35__wbindgen_externref_table_set_null17h01aaad0fb9a4ac4cE (type 0)))
  (func $add2num (type 2) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.add)
  (func $__wbindgen_describe_add2num (type 3)
    i32.const 11
    call $_ZN12wasm_bindgen19__wbindgen_describe17h972d0852984605d7E
    i32.const 0
    call $_ZN12wasm_bindgen19__wbindgen_describe17h972d0852984605d7E
    i32.const 2
    call $_ZN12wasm_bindgen19__wbindgen_describe17h972d0852984605d7E
    i32.const 4
    call $_ZN12wasm_bindgen19__wbindgen_describe17h972d0852984605d7E
    i32.const 4
    call $_ZN12wasm_bindgen19__wbindgen_describe17h972d0852984605d7E
    i32.const 4
    call $_ZN12wasm_bindgen19__wbindgen_describe17h972d0852984605d7E
    i32.const 4
    call $_ZN12wasm_bindgen19__wbindgen_describe17h972d0852984605d7E)
  (func $__rust_realloc (type 4) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.const -4
              i32.add
              local.tee 4
              i32.load
              local.tee 5
              i32.const -8
              i32.and
              local.tee 6
              i32.const 4
              i32.const 8
              local.get 5
              i32.const 3
              i32.and
              local.tee 7
              select
              local.get 1
              i32.add
              i32.lt_u
              br_if 0 (;@5;)
              local.get 1
              i32.const 39
              i32.add
              local.set 8
              block  ;; label = @6
                local.get 7
                i32.eqz
                br_if 0 (;@6;)
                local.get 6
                local.get 8
                i32.gt_u
                br_if 2 (;@4;)
              end
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    i32.const 9
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 2
                    local.get 3
                    call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8memalign17h234873f561ba0f37E
                    local.tee 2
                    br_if 1 (;@7;)
                    i32.const 0
                    return
                  end
                  i32.const 0
                  local.set 2
                  local.get 3
                  i32.const -65588
                  i32.gt_u
                  br_if 1 (;@6;)
                  i32.const 16
                  local.get 3
                  i32.const 11
                  i32.add
                  i32.const -8
                  i32.and
                  local.get 3
                  i32.const 11
                  i32.lt_u
                  select
                  local.set 1
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 7
                      br_if 0 (;@9;)
                      local.get 1
                      i32.const 256
                      i32.lt_u
                      br_if 1 (;@8;)
                      local.get 6
                      local.get 1
                      i32.const 4
                      i32.or
                      i32.lt_u
                      br_if 1 (;@8;)
                      local.get 6
                      local.get 1
                      i32.sub
                      i32.const 131073
                      i32.ge_u
                      br_if 1 (;@8;)
                      local.get 0
                      return
                    end
                    local.get 0
                    i32.const -8
                    i32.add
                    local.tee 8
                    local.get 6
                    i32.add
                    local.set 7
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 6
                              local.get 1
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 7
                              i32.const 0
                              i32.load offset=1049284
                              i32.eq
                              br_if 4 (;@9;)
                              local.get 7
                              i32.const 0
                              i32.load offset=1049280
                              i32.eq
                              br_if 2 (;@11;)
                              local.get 7
                              i32.load offset=4
                              local.tee 5
                              i32.const 2
                              i32.and
                              br_if 5 (;@8;)
                              local.get 5
                              i32.const -8
                              i32.and
                              local.tee 9
                              local.get 6
                              i32.add
                              local.tee 5
                              local.get 1
                              i32.lt_u
                              br_if 5 (;@8;)
                              local.get 7
                              local.get 9
                              call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$12unlink_chunk17h22188baaa299953bE
                              local.get 5
                              local.get 1
                              i32.sub
                              local.tee 3
                              i32.const 16
                              i32.lt_u
                              br_if 1 (;@12;)
                              local.get 4
                              local.get 1
                              local.get 4
                              i32.load
                              i32.const 1
                              i32.and
                              i32.or
                              i32.const 2
                              i32.or
                              i32.store
                              local.get 8
                              local.get 1
                              i32.add
                              local.tee 1
                              local.get 3
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 8
                              local.get 5
                              i32.add
                              local.tee 2
                              local.get 2
                              i32.load offset=4
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              local.get 1
                              local.get 3
                              call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17hecc39783b7c7ad40E
                              local.get 0
                              return
                            end
                            local.get 6
                            local.get 1
                            i32.sub
                            local.tee 3
                            i32.const 15
                            i32.gt_u
                            br_if 2 (;@10;)
                            local.get 0
                            return
                          end
                          local.get 4
                          local.get 5
                          local.get 4
                          i32.load
                          i32.const 1
                          i32.and
                          i32.or
                          i32.const 2
                          i32.or
                          i32.store
                          local.get 8
                          local.get 5
                          i32.add
                          local.tee 1
                          local.get 1
                          i32.load offset=4
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 0
                          return
                        end
                        i32.const 0
                        i32.load offset=1049272
                        local.get 6
                        i32.add
                        local.tee 7
                        local.get 1
                        i32.lt_u
                        br_if 2 (;@8;)
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 7
                            local.get 1
                            i32.sub
                            local.tee 3
                            i32.const 15
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 4
                            local.get 5
                            i32.const 1
                            i32.and
                            local.get 7
                            i32.or
                            i32.const 2
                            i32.or
                            i32.store
                            local.get 8
                            local.get 7
                            i32.add
                            local.tee 1
                            local.get 1
                            i32.load offset=4
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            i32.const 0
                            local.set 3
                            i32.const 0
                            local.set 1
                            br 1 (;@11;)
                          end
                          local.get 4
                          local.get 1
                          local.get 5
                          i32.const 1
                          i32.and
                          i32.or
                          i32.const 2
                          i32.or
                          i32.store
                          local.get 8
                          local.get 1
                          i32.add
                          local.tee 1
                          local.get 3
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 8
                          local.get 7
                          i32.add
                          local.tee 2
                          local.get 3
                          i32.store
                          local.get 2
                          local.get 2
                          i32.load offset=4
                          i32.const -2
                          i32.and
                          i32.store offset=4
                        end
                        i32.const 0
                        local.get 1
                        i32.store offset=1049280
                        i32.const 0
                        local.get 3
                        i32.store offset=1049272
                        local.get 0
                        return
                      end
                      local.get 4
                      local.get 1
                      local.get 5
                      i32.const 1
                      i32.and
                      i32.or
                      i32.const 2
                      i32.or
                      i32.store
                      local.get 8
                      local.get 1
                      i32.add
                      local.tee 1
                      local.get 3
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 7
                      local.get 7
                      i32.load offset=4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 1
                      local.get 3
                      call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17hecc39783b7c7ad40E
                      local.get 0
                      return
                    end
                    i32.const 0
                    i32.load offset=1049276
                    local.get 6
                    i32.add
                    local.tee 7
                    local.get 1
                    i32.gt_u
                    br_if 7 (;@1;)
                  end
                  local.get 3
                  call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17h9867234b8d2950f1E
                  local.tee 1
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 1
                  local.get 0
                  i32.const -4
                  i32.const -8
                  local.get 4
                  i32.load
                  local.tee 2
                  i32.const 3
                  i32.and
                  select
                  local.get 2
                  i32.const -8
                  i32.and
                  i32.add
                  local.tee 2
                  local.get 3
                  local.get 2
                  local.get 3
                  i32.lt_u
                  select
                  call $memcpy
                  local.set 1
                  local.get 0
                  call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17h1ca6b53be59cc4abE
                  local.get 1
                  return
                end
                local.get 2
                local.get 0
                local.get 1
                local.get 3
                local.get 1
                local.get 3
                i32.lt_u
                select
                call $memcpy
                drop
                local.get 4
                i32.load
                local.tee 3
                i32.const -8
                i32.and
                local.tee 7
                i32.const 4
                i32.const 8
                local.get 3
                i32.const 3
                i32.and
                local.tee 3
                select
                local.get 1
                i32.add
                i32.lt_u
                br_if 3 (;@3;)
                block  ;; label = @7
                  local.get 3
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 7
                  local.get 8
                  i32.gt_u
                  br_if 5 (;@2;)
                end
                local.get 0
                call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17h1ca6b53be59cc4abE
              end
              local.get 2
              return
            end
            i32.const 1048677
            i32.const 46
            i32.const 1048724
            call $_ZN4core9panicking5panic17h7587eba6ad042231E
            unreachable
          end
          i32.const 1048740
          i32.const 46
          i32.const 1048788
          call $_ZN4core9panicking5panic17h7587eba6ad042231E
          unreachable
        end
        i32.const 1048677
        i32.const 46
        i32.const 1048724
        call $_ZN4core9panicking5panic17h7587eba6ad042231E
        unreachable
      end
      i32.const 1048740
      i32.const 46
      i32.const 1048788
      call $_ZN4core9panicking5panic17h7587eba6ad042231E
      unreachable
    end
    local.get 4
    local.get 1
    local.get 5
    i32.const 1
    i32.and
    i32.or
    i32.const 2
    i32.or
    i32.store
    local.get 8
    local.get 1
    i32.add
    local.tee 3
    local.get 7
    local.get 1
    i32.sub
    local.tee 1
    i32.const 1
    i32.or
    i32.store offset=4
    i32.const 0
    local.get 1
    i32.store offset=1049276
    i32.const 0
    local.get 3
    i32.store offset=1049284
    local.get 0)
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8memalign17h234873f561ba0f37E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    i32.const 0
    local.set 2
    block  ;; label = @1
      i32.const -65587
      local.get 0
      i32.const 16
      local.get 0
      i32.const 16
      i32.gt_u
      select
      local.tee 0
      i32.sub
      local.get 1
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 16
      local.get 1
      i32.const 11
      i32.add
      i32.const -8
      i32.and
      local.get 1
      i32.const 11
      i32.lt_u
      select
      local.tee 3
      i32.add
      i32.const 12
      i32.add
      call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17h9867234b8d2950f1E
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const -8
      i32.add
      local.set 2
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.const -1
          i32.add
          local.tee 4
          local.get 1
          i32.and
          br_if 0 (;@3;)
          local.get 2
          local.set 0
          br 1 (;@2;)
        end
        local.get 1
        i32.const -4
        i32.add
        local.tee 5
        i32.load
        local.tee 6
        i32.const -8
        i32.and
        local.get 4
        local.get 1
        i32.add
        i32.const 0
        local.get 0
        i32.sub
        i32.and
        i32.const -8
        i32.add
        local.tee 1
        i32.const 0
        local.get 0
        local.get 1
        local.get 2
        i32.sub
        i32.const 16
        i32.gt_u
        select
        i32.add
        local.tee 0
        local.get 2
        i32.sub
        local.tee 1
        i32.sub
        local.set 4
        block  ;; label = @3
          local.get 6
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          local.get 4
          local.get 0
          i32.load offset=4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store offset=4
          local.get 0
          local.get 4
          i32.add
          local.tee 4
          local.get 4
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 5
          local.get 1
          local.get 5
          i32.load
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 2
          local.get 1
          i32.add
          local.tee 4
          local.get 4
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 2
          local.get 1
          call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17hecc39783b7c7ad40E
          br 1 (;@2;)
        end
        local.get 2
        i32.load
        local.set 2
        local.get 0
        local.get 4
        i32.store offset=4
        local.get 0
        local.get 2
        local.get 1
        i32.add
        i32.store
      end
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.const -8
        i32.and
        local.tee 2
        local.get 3
        i32.const 16
        i32.add
        i32.le_u
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        local.get 1
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get 0
        local.get 3
        i32.add
        local.tee 1
        local.get 2
        local.get 3
        i32.sub
        local.tee 3
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 0
        local.get 2
        i32.add
        local.tee 2
        local.get 2
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 1
        local.get 3
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17hecc39783b7c7ad40E
      end
      local.get 0
      i32.const 8
      i32.add
      local.set 2
    end
    local.get 2)
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$12unlink_chunk17h22188baaa299953bE (type 5) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 0
    i32.load offset=12
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 256
          i32.lt_u
          br_if 0 (;@3;)
          local.get 0
          i32.load offset=24
          local.set 3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                local.get 0
                i32.ne
                br_if 0 (;@6;)
                local.get 0
                i32.const 20
                i32.const 16
                local.get 0
                i32.load offset=20
                local.tee 2
                select
                i32.add
                i32.load
                local.tee 1
                br_if 1 (;@5;)
                i32.const 0
                local.set 2
                br 2 (;@4;)
              end
              local.get 0
              i32.load offset=8
              local.tee 1
              local.get 2
              i32.store offset=12
              local.get 2
              local.get 1
              i32.store offset=8
              br 1 (;@4;)
            end
            local.get 0
            i32.const 20
            i32.add
            local.get 0
            i32.const 16
            i32.add
            local.get 2
            select
            local.set 4
            loop  ;; label = @5
              local.get 4
              local.set 5
              local.get 1
              local.tee 2
              i32.const 20
              i32.add
              local.get 2
              i32.const 16
              i32.add
              local.get 2
              i32.load offset=20
              local.tee 1
              select
              local.set 4
              local.get 2
              i32.const 20
              i32.const 16
              local.get 1
              select
              i32.add
              i32.load
              local.tee 1
              br_if 0 (;@5;)
            end
            local.get 5
            i32.const 0
            i32.store
          end
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          block  ;; label = @4
            local.get 0
            i32.load offset=28
            i32.const 2
            i32.shl
            i32.const 1048856
            i32.add
            local.tee 1
            i32.load
            local.get 0
            i32.eq
            br_if 0 (;@4;)
            local.get 3
            i32.const 16
            i32.const 20
            local.get 3
            i32.load offset=16
            local.get 0
            i32.eq
            select
            i32.add
            local.get 2
            i32.store
            local.get 2
            i32.eqz
            br_if 3 (;@1;)
            br 2 (;@2;)
          end
          local.get 1
          local.get 2
          i32.store
          local.get 2
          br_if 1 (;@2;)
          i32.const 0
          i32.const 0
          i32.load offset=1049268
          i32.const -2
          local.get 0
          i32.load offset=28
          i32.rotl
          i32.and
          i32.store offset=1049268
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 2
          local.get 0
          i32.load offset=8
          local.tee 4
          i32.eq
          br_if 0 (;@3;)
          local.get 4
          local.get 2
          i32.store offset=12
          local.get 2
          local.get 4
          i32.store offset=8
          return
        end
        i32.const 0
        i32.const 0
        i32.load offset=1049264
        i32.const -2
        local.get 1
        i32.const 3
        i32.shr_u
        i32.rotl
        i32.and
        i32.store offset=1049264
        return
      end
      local.get 2
      local.get 3
      i32.store offset=24
      block  ;; label = @2
        local.get 0
        i32.load offset=16
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.store offset=16
        local.get 1
        local.get 2
        i32.store offset=24
      end
      local.get 0
      i32.load offset=20
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 1
      i32.store offset=20
      local.get 1
      local.get 2
      i32.store offset=24
      return
    end)
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17hecc39783b7c7ad40E (type 5) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 0
    local.get 1
    i32.add
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 3
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 3
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.tee 3
        local.get 1
        i32.add
        local.set 1
        block  ;; label = @3
          local.get 0
          local.get 3
          i32.sub
          local.tee 0
          i32.const 0
          i32.load offset=1049280
          i32.ne
          br_if 0 (;@3;)
          local.get 2
          i32.load offset=4
          i32.const 3
          i32.and
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          i32.const 0
          local.get 1
          i32.store offset=1049272
          local.get 2
          local.get 2
          i32.load offset=4
          i32.const -2
          i32.and
          i32.store offset=4
          local.get 0
          local.get 1
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 2
          local.get 1
          i32.store
          br 2 (;@1;)
        end
        local.get 0
        local.get 3
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$12unlink_chunk17h22188baaa299953bE
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.load offset=4
              local.tee 3
              i32.const 2
              i32.and
              br_if 0 (;@5;)
              local.get 2
              i32.const 0
              i32.load offset=1049284
              i32.eq
              br_if 2 (;@3;)
              local.get 2
              i32.const 0
              i32.load offset=1049280
              i32.eq
              br_if 3 (;@2;)
              local.get 2
              local.get 3
              i32.const -8
              i32.and
              local.tee 3
              call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$12unlink_chunk17h22188baaa299953bE
              local.get 0
              local.get 3
              local.get 1
              i32.add
              local.tee 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 1
              i32.add
              local.get 1
              i32.store
              local.get 0
              i32.const 0
              i32.load offset=1049280
              i32.ne
              br_if 1 (;@4;)
              i32.const 0
              local.get 1
              i32.store offset=1049272
              return
            end
            local.get 2
            local.get 3
            i32.const -2
            i32.and
            i32.store offset=4
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            local.get 1
            i32.add
            local.get 1
            i32.store
          end
          block  ;; label = @4
            local.get 1
            i32.const 256
            i32.lt_u
            br_if 0 (;@4;)
            i32.const 31
            local.set 2
            block  ;; label = @5
              local.get 1
              i32.const 16777215
              i32.gt_u
              br_if 0 (;@5;)
              local.get 1
              i32.const 6
              local.get 1
              i32.const 8
              i32.shr_u
              i32.clz
              local.tee 2
              i32.sub
              i32.shr_u
              i32.const 1
              i32.and
              local.get 2
              i32.const 1
              i32.shl
              i32.sub
              i32.const 62
              i32.add
              local.set 2
            end
            local.get 0
            i64.const 0
            i64.store offset=16 align=4
            local.get 0
            local.get 2
            i32.store offset=28
            local.get 2
            i32.const 2
            i32.shl
            i32.const 1048856
            i32.add
            local.set 3
            block  ;; label = @5
              i32.const 0
              i32.load offset=1049268
              i32.const 1
              local.get 2
              i32.shl
              local.tee 4
              i32.and
              br_if 0 (;@5;)
              local.get 3
              local.get 0
              i32.store
              local.get 0
              local.get 3
              i32.store offset=24
              local.get 0
              local.get 0
              i32.store offset=12
              local.get 0
              local.get 0
              i32.store offset=8
              i32.const 0
              i32.const 0
              i32.load offset=1049268
              local.get 4
              i32.or
              i32.store offset=1049268
              return
            end
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 3
                  i32.load
                  local.tee 4
                  i32.load offset=4
                  i32.const -8
                  i32.and
                  local.get 1
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 4
                  local.set 2
                  br 1 (;@6;)
                end
                local.get 1
                i32.const 0
                i32.const 25
                local.get 2
                i32.const 1
                i32.shr_u
                i32.sub
                local.get 2
                i32.const 31
                i32.eq
                select
                i32.shl
                local.set 3
                loop  ;; label = @7
                  local.get 4
                  local.get 3
                  i32.const 29
                  i32.shr_u
                  i32.const 4
                  i32.and
                  i32.add
                  i32.const 16
                  i32.add
                  local.tee 5
                  i32.load
                  local.tee 2
                  i32.eqz
                  br_if 2 (;@5;)
                  local.get 3
                  i32.const 1
                  i32.shl
                  local.set 3
                  local.get 2
                  local.set 4
                  local.get 2
                  i32.load offset=4
                  i32.const -8
                  i32.and
                  local.get 1
                  i32.ne
                  br_if 0 (;@7;)
                end
              end
              local.get 2
              i32.load offset=8
              local.tee 1
              local.get 0
              i32.store offset=12
              local.get 2
              local.get 0
              i32.store offset=8
              local.get 0
              i32.const 0
              i32.store offset=24
              local.get 0
              local.get 2
              i32.store offset=12
              local.get 0
              local.get 1
              i32.store offset=8
              return
            end
            local.get 5
            local.get 0
            i32.store
            local.get 0
            local.get 4
            i32.store offset=24
            local.get 0
            local.get 0
            i32.store offset=12
            local.get 0
            local.get 0
            i32.store offset=8
            return
          end
          local.get 1
          i32.const -8
          i32.and
          i32.const 1049000
          i32.add
          local.set 2
          block  ;; label = @4
            block  ;; label = @5
              i32.const 0
              i32.load offset=1049264
              local.tee 3
              i32.const 1
              local.get 1
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 1
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              local.get 3
              local.get 1
              i32.or
              i32.store offset=1049264
              local.get 2
              local.set 1
              br 1 (;@4;)
            end
            local.get 2
            i32.load offset=8
            local.set 1
          end
          local.get 2
          local.get 0
          i32.store offset=8
          local.get 1
          local.get 0
          i32.store offset=12
          local.get 0
          local.get 2
          i32.store offset=12
          local.get 0
          local.get 1
          i32.store offset=8
          return
        end
        i32.const 0
        local.get 0
        i32.store offset=1049284
        i32.const 0
        i32.const 0
        i32.load offset=1049276
        local.get 1
        i32.add
        local.tee 1
        i32.store offset=1049276
        local.get 0
        local.get 1
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        i32.const 0
        i32.load offset=1049280
        i32.ne
        br_if 1 (;@1;)
        i32.const 0
        i32.const 0
        i32.store offset=1049272
        i32.const 0
        i32.const 0
        i32.store offset=1049280
        return
      end
      i32.const 0
      local.get 0
      i32.store offset=1049280
      i32.const 0
      i32.const 0
      i32.load offset=1049272
      local.get 1
      i32.add
      local.tee 1
      i32.store offset=1049272
      local.get 0
      local.get 1
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 0
      local.get 1
      i32.add
      local.get 1
      i32.store
      return
    end)
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17h9867234b8d2950f1E (type 1) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i64)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    i32.const 245
                    i32.lt_u
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 1
                    local.get 0
                    i32.const -65587
                    i32.ge_u
                    br_if 5 (;@3;)
                    local.get 0
                    i32.const 11
                    i32.add
                    local.tee 0
                    i32.const -8
                    i32.and
                    local.set 2
                    i32.const 0
                    i32.load offset=1049268
                    local.tee 3
                    i32.eqz
                    br_if 4 (;@4;)
                    i32.const 0
                    local.set 4
                    block  ;; label = @9
                      local.get 2
                      i32.const 256
                      i32.lt_u
                      br_if 0 (;@9;)
                      i32.const 31
                      local.set 4
                      local.get 2
                      i32.const 16777215
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 2
                      i32.const 6
                      local.get 0
                      i32.const 8
                      i32.shr_u
                      i32.clz
                      local.tee 0
                      i32.sub
                      i32.shr_u
                      i32.const 1
                      i32.and
                      local.get 0
                      i32.const 1
                      i32.shl
                      i32.sub
                      i32.const 62
                      i32.add
                      local.set 4
                    end
                    i32.const 0
                    local.get 2
                    i32.sub
                    local.set 1
                    block  ;; label = @9
                      local.get 4
                      i32.const 2
                      i32.shl
                      i32.const 1048856
                      i32.add
                      i32.load
                      local.tee 5
                      br_if 0 (;@9;)
                      i32.const 0
                      local.set 0
                      i32.const 0
                      local.set 6
                      br 2 (;@7;)
                    end
                    i32.const 0
                    local.set 0
                    local.get 2
                    i32.const 0
                    i32.const 25
                    local.get 4
                    i32.const 1
                    i32.shr_u
                    i32.sub
                    local.get 4
                    i32.const 31
                    i32.eq
                    select
                    i32.shl
                    local.set 7
                    i32.const 0
                    local.set 6
                    loop  ;; label = @9
                      block  ;; label = @10
                        local.get 5
                        local.tee 5
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.tee 8
                        local.get 2
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 8
                        local.get 2
                        i32.sub
                        local.tee 8
                        local.get 1
                        i32.ge_u
                        br_if 0 (;@10;)
                        local.get 8
                        local.set 1
                        local.get 5
                        local.set 6
                        local.get 8
                        br_if 0 (;@10;)
                        i32.const 0
                        local.set 1
                        local.get 5
                        local.set 6
                        local.get 5
                        local.set 0
                        br 4 (;@6;)
                      end
                      local.get 5
                      i32.load offset=20
                      local.tee 8
                      local.get 0
                      local.get 8
                      local.get 5
                      local.get 7
                      i32.const 29
                      i32.shr_u
                      i32.const 4
                      i32.and
                      i32.add
                      i32.const 16
                      i32.add
                      i32.load
                      local.tee 5
                      i32.ne
                      select
                      local.get 0
                      local.get 8
                      select
                      local.set 0
                      local.get 7
                      i32.const 1
                      i32.shl
                      local.set 7
                      local.get 5
                      i32.eqz
                      br_if 2 (;@7;)
                      br 0 (;@9;)
                    end
                  end
                  block  ;; label = @8
                    i32.const 0
                    i32.load offset=1049264
                    local.tee 5
                    i32.const 16
                    local.get 0
                    i32.const 11
                    i32.add
                    i32.const 504
                    i32.and
                    local.get 0
                    i32.const 11
                    i32.lt_u
                    select
                    local.tee 2
                    i32.const 3
                    i32.shr_u
                    local.tee 1
                    i32.shr_u
                    local.tee 0
                    i32.const 3
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        i32.const -1
                        i32.xor
                        i32.const 1
                        i32.and
                        local.get 1
                        i32.add
                        local.tee 2
                        i32.const 3
                        i32.shl
                        local.tee 0
                        i32.const 1049000
                        i32.add
                        local.tee 1
                        local.get 0
                        i32.const 1049008
                        i32.add
                        i32.load
                        local.tee 0
                        i32.load offset=8
                        local.tee 6
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 6
                        local.get 1
                        i32.store offset=12
                        local.get 1
                        local.get 6
                        i32.store offset=8
                        br 1 (;@9;)
                      end
                      i32.const 0
                      local.get 5
                      i32.const -2
                      local.get 2
                      i32.rotl
                      i32.and
                      i32.store offset=1049264
                    end
                    local.get 0
                    local.get 2
                    i32.const 3
                    i32.shl
                    local.tee 2
                    i32.const 3
                    i32.or
                    i32.store offset=4
                    local.get 0
                    local.get 2
                    i32.add
                    local.tee 2
                    local.get 2
                    i32.load offset=4
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 0
                    i32.const 8
                    i32.add
                    return
                  end
                  local.get 2
                  i32.const 0
                  i32.load offset=1049272
                  i32.le_u
                  br_if 3 (;@4;)
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        br_if 0 (;@10;)
                        i32.const 0
                        i32.load offset=1049268
                        local.tee 0
                        i32.eqz
                        br_if 6 (;@4;)
                        local.get 0
                        i32.ctz
                        i32.const 2
                        i32.shl
                        i32.const 1048856
                        i32.add
                        i32.load
                        local.tee 6
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.get 2
                        i32.sub
                        local.set 1
                        local.get 6
                        local.set 5
                        loop  ;; label = @11
                          block  ;; label = @12
                            local.get 6
                            i32.load offset=16
                            local.tee 0
                            br_if 0 (;@12;)
                            local.get 6
                            i32.load offset=20
                            local.tee 0
                            br_if 0 (;@12;)
                            local.get 5
                            i32.load offset=24
                            local.set 4
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 5
                                  i32.load offset=12
                                  local.tee 0
                                  local.get 5
                                  i32.ne
                                  br_if 0 (;@15;)
                                  local.get 5
                                  i32.const 20
                                  i32.const 16
                                  local.get 5
                                  i32.load offset=20
                                  local.tee 0
                                  select
                                  i32.add
                                  i32.load
                                  local.tee 6
                                  br_if 1 (;@14;)
                                  i32.const 0
                                  local.set 0
                                  br 2 (;@13;)
                                end
                                local.get 5
                                i32.load offset=8
                                local.tee 6
                                local.get 0
                                i32.store offset=12
                                local.get 0
                                local.get 6
                                i32.store offset=8
                                br 1 (;@13;)
                              end
                              local.get 5
                              i32.const 20
                              i32.add
                              local.get 5
                              i32.const 16
                              i32.add
                              local.get 0
                              select
                              local.set 7
                              loop  ;; label = @14
                                local.get 7
                                local.set 8
                                local.get 6
                                local.tee 0
                                i32.const 20
                                i32.add
                                local.get 0
                                i32.const 16
                                i32.add
                                local.get 0
                                i32.load offset=20
                                local.tee 6
                                select
                                local.set 7
                                local.get 0
                                i32.const 20
                                i32.const 16
                                local.get 6
                                select
                                i32.add
                                i32.load
                                local.tee 6
                                br_if 0 (;@14;)
                              end
                              local.get 8
                              i32.const 0
                              i32.store
                            end
                            local.get 4
                            i32.eqz
                            br_if 4 (;@8;)
                            block  ;; label = @13
                              local.get 5
                              i32.load offset=28
                              i32.const 2
                              i32.shl
                              i32.const 1048856
                              i32.add
                              local.tee 6
                              i32.load
                              local.get 5
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 4
                              i32.const 16
                              i32.const 20
                              local.get 4
                              i32.load offset=16
                              local.get 5
                              i32.eq
                              select
                              i32.add
                              local.get 0
                              i32.store
                              local.get 0
                              i32.eqz
                              br_if 5 (;@8;)
                              br 4 (;@9;)
                            end
                            local.get 6
                            local.get 0
                            i32.store
                            local.get 0
                            br_if 3 (;@9;)
                            i32.const 0
                            i32.const 0
                            i32.load offset=1049268
                            i32.const -2
                            local.get 5
                            i32.load offset=28
                            i32.rotl
                            i32.and
                            i32.store offset=1049268
                            br 4 (;@8;)
                          end
                          local.get 0
                          i32.load offset=4
                          i32.const -8
                          i32.and
                          local.get 2
                          i32.sub
                          local.tee 6
                          local.get 1
                          local.get 6
                          local.get 1
                          i32.lt_u
                          local.tee 6
                          select
                          local.set 1
                          local.get 0
                          local.get 5
                          local.get 6
                          select
                          local.set 5
                          local.get 0
                          local.set 6
                          br 0 (;@11;)
                        end
                      end
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 0
                          local.get 1
                          i32.shl
                          i32.const 2
                          local.get 1
                          i32.shl
                          local.tee 0
                          i32.const 0
                          local.get 0
                          i32.sub
                          i32.or
                          i32.and
                          i32.ctz
                          local.tee 1
                          i32.const 3
                          i32.shl
                          local.tee 0
                          i32.const 1049000
                          i32.add
                          local.tee 6
                          local.get 0
                          i32.const 1049008
                          i32.add
                          i32.load
                          local.tee 0
                          i32.load offset=8
                          local.tee 7
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 7
                          local.get 6
                          i32.store offset=12
                          local.get 6
                          local.get 7
                          i32.store offset=8
                          br 1 (;@10;)
                        end
                        i32.const 0
                        local.get 5
                        i32.const -2
                        local.get 1
                        i32.rotl
                        i32.and
                        i32.store offset=1049264
                      end
                      local.get 0
                      local.get 2
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 2
                      i32.add
                      local.tee 7
                      local.get 1
                      i32.const 3
                      i32.shl
                      local.tee 6
                      local.get 2
                      i32.sub
                      local.tee 1
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 6
                      i32.add
                      local.get 1
                      i32.store
                      block  ;; label = @10
                        i32.const 0
                        i32.load offset=1049272
                        local.tee 5
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 5
                        i32.const -8
                        i32.and
                        i32.const 1049000
                        i32.add
                        local.set 6
                        i32.const 0
                        i32.load offset=1049280
                        local.set 2
                        block  ;; label = @11
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=1049264
                            local.tee 8
                            i32.const 1
                            local.get 5
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 5
                            i32.and
                            br_if 0 (;@12;)
                            i32.const 0
                            local.get 8
                            local.get 5
                            i32.or
                            i32.store offset=1049264
                            local.get 6
                            local.set 5
                            br 1 (;@11;)
                          end
                          local.get 6
                          i32.load offset=8
                          local.set 5
                        end
                        local.get 6
                        local.get 2
                        i32.store offset=8
                        local.get 5
                        local.get 2
                        i32.store offset=12
                        local.get 2
                        local.get 6
                        i32.store offset=12
                        local.get 2
                        local.get 5
                        i32.store offset=8
                      end
                      i32.const 0
                      local.get 7
                      i32.store offset=1049280
                      i32.const 0
                      local.get 1
                      i32.store offset=1049272
                      local.get 0
                      i32.const 8
                      i32.add
                      return
                    end
                    local.get 0
                    local.get 4
                    i32.store offset=24
                    block  ;; label = @9
                      local.get 5
                      i32.load offset=16
                      local.tee 6
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 6
                      i32.store offset=16
                      local.get 6
                      local.get 0
                      i32.store offset=24
                    end
                    local.get 5
                    i32.load offset=20
                    local.tee 6
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 6
                    i32.store offset=20
                    local.get 6
                    local.get 0
                    i32.store offset=24
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 1
                        i32.const 16
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 5
                        local.get 2
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 5
                        local.get 2
                        i32.add
                        local.tee 2
                        local.get 1
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 2
                        local.get 1
                        i32.add
                        local.get 1
                        i32.store
                        i32.const 0
                        i32.load offset=1049272
                        local.tee 7
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 7
                        i32.const -8
                        i32.and
                        i32.const 1049000
                        i32.add
                        local.set 6
                        i32.const 0
                        i32.load offset=1049280
                        local.set 0
                        block  ;; label = @11
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=1049264
                            local.tee 8
                            i32.const 1
                            local.get 7
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 7
                            i32.and
                            br_if 0 (;@12;)
                            i32.const 0
                            local.get 8
                            local.get 7
                            i32.or
                            i32.store offset=1049264
                            local.get 6
                            local.set 7
                            br 1 (;@11;)
                          end
                          local.get 6
                          i32.load offset=8
                          local.set 7
                        end
                        local.get 6
                        local.get 0
                        i32.store offset=8
                        local.get 7
                        local.get 0
                        i32.store offset=12
                        local.get 0
                        local.get 6
                        i32.store offset=12
                        local.get 0
                        local.get 7
                        i32.store offset=8
                        br 1 (;@9;)
                      end
                      local.get 5
                      local.get 1
                      local.get 2
                      i32.add
                      local.tee 0
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 5
                      local.get 0
                      i32.add
                      local.tee 0
                      local.get 0
                      i32.load offset=4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      br 1 (;@8;)
                    end
                    i32.const 0
                    local.get 2
                    i32.store offset=1049280
                    i32.const 0
                    local.get 1
                    i32.store offset=1049272
                  end
                  local.get 5
                  i32.const 8
                  i32.add
                  return
                end
                block  ;; label = @7
                  local.get 0
                  local.get 6
                  i32.or
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 6
                  i32.const 2
                  local.get 4
                  i32.shl
                  local.tee 0
                  i32.const 0
                  local.get 0
                  i32.sub
                  i32.or
                  local.get 3
                  i32.and
                  local.tee 0
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 0
                  i32.ctz
                  i32.const 2
                  i32.shl
                  i32.const 1048856
                  i32.add
                  i32.load
                  local.set 0
                end
                local.get 0
                i32.eqz
                br_if 1 (;@5;)
              end
              loop  ;; label = @6
                local.get 0
                local.get 6
                local.get 0
                i32.load offset=4
                i32.const -8
                i32.and
                local.tee 5
                local.get 2
                i32.sub
                local.tee 8
                local.get 1
                i32.lt_u
                local.tee 4
                select
                local.set 3
                local.get 5
                local.get 2
                i32.lt_u
                local.set 7
                local.get 8
                local.get 1
                local.get 4
                select
                local.set 8
                block  ;; label = @7
                  local.get 0
                  i32.load offset=16
                  local.tee 5
                  br_if 0 (;@7;)
                  local.get 0
                  i32.load offset=20
                  local.set 5
                end
                local.get 6
                local.get 3
                local.get 7
                select
                local.set 6
                local.get 1
                local.get 8
                local.get 7
                select
                local.set 1
                local.get 5
                local.set 0
                local.get 5
                br_if 0 (;@6;)
              end
            end
            local.get 6
            i32.eqz
            br_if 0 (;@4;)
            block  ;; label = @5
              i32.const 0
              i32.load offset=1049272
              local.tee 0
              local.get 2
              i32.lt_u
              br_if 0 (;@5;)
              local.get 1
              local.get 0
              local.get 2
              i32.sub
              i32.ge_u
              br_if 1 (;@4;)
            end
            local.get 6
            i32.load offset=24
            local.set 4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 6
                  i32.load offset=12
                  local.tee 0
                  local.get 6
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 6
                  i32.const 20
                  i32.const 16
                  local.get 6
                  i32.load offset=20
                  local.tee 0
                  select
                  i32.add
                  i32.load
                  local.tee 5
                  br_if 1 (;@6;)
                  i32.const 0
                  local.set 0
                  br 2 (;@5;)
                end
                local.get 6
                i32.load offset=8
                local.tee 5
                local.get 0
                i32.store offset=12
                local.get 0
                local.get 5
                i32.store offset=8
                br 1 (;@5;)
              end
              local.get 6
              i32.const 20
              i32.add
              local.get 6
              i32.const 16
              i32.add
              local.get 0
              select
              local.set 7
              loop  ;; label = @6
                local.get 7
                local.set 8
                local.get 5
                local.tee 0
                i32.const 20
                i32.add
                local.get 0
                i32.const 16
                i32.add
                local.get 0
                i32.load offset=20
                local.tee 5
                select
                local.set 7
                local.get 0
                i32.const 20
                i32.const 16
                local.get 5
                select
                i32.add
                i32.load
                local.tee 5
                br_if 0 (;@6;)
              end
              local.get 8
              i32.const 0
              i32.store
            end
            local.get 4
            i32.eqz
            br_if 3 (;@1;)
            block  ;; label = @5
              local.get 6
              i32.load offset=28
              i32.const 2
              i32.shl
              i32.const 1048856
              i32.add
              local.tee 5
              i32.load
              local.get 6
              i32.eq
              br_if 0 (;@5;)
              local.get 4
              i32.const 16
              i32.const 20
              local.get 4
              i32.load offset=16
              local.get 6
              i32.eq
              select
              i32.add
              local.get 0
              i32.store
              local.get 0
              i32.eqz
              br_if 4 (;@1;)
              br 3 (;@2;)
            end
            local.get 5
            local.get 0
            i32.store
            local.get 0
            br_if 2 (;@2;)
            i32.const 0
            i32.const 0
            i32.load offset=1049268
            i32.const -2
            local.get 6
            i32.load offset=28
            i32.rotl
            i32.and
            i32.store offset=1049268
            br 3 (;@1;)
          end
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      i32.const 0
                      i32.load offset=1049272
                      local.tee 0
                      local.get 2
                      i32.ge_u
                      br_if 0 (;@9;)
                      block  ;; label = @10
                        i32.const 0
                        i32.load offset=1049276
                        local.tee 0
                        local.get 2
                        i32.gt_u
                        br_if 0 (;@10;)
                        i32.const 0
                        local.set 1
                        local.get 2
                        i32.const 65583
                        i32.add
                        local.tee 6
                        i32.const 16
                        i32.shr_u
                        memory.grow
                        local.tee 0
                        i32.const -1
                        i32.eq
                        local.tee 7
                        br_if 7 (;@3;)
                        local.get 0
                        i32.const 16
                        i32.shl
                        local.tee 5
                        i32.eqz
                        br_if 7 (;@3;)
                        i32.const 0
                        i32.const 0
                        i32.load offset=1049288
                        i32.const 0
                        local.get 6
                        i32.const -65536
                        i32.and
                        local.get 7
                        select
                        local.tee 8
                        i32.add
                        local.tee 0
                        i32.store offset=1049288
                        i32.const 0
                        i32.const 0
                        i32.load offset=1049292
                        local.tee 1
                        local.get 0
                        local.get 1
                        local.get 0
                        i32.gt_u
                        select
                        i32.store offset=1049292
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=1049284
                              local.tee 1
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 1048984
                              local.set 0
                              loop  ;; label = @14
                                local.get 0
                                i32.load
                                local.tee 6
                                local.get 0
                                i32.load offset=4
                                local.tee 7
                                i32.add
                                local.get 5
                                i32.eq
                                br_if 2 (;@12;)
                                local.get 0
                                i32.load offset=8
                                local.tee 0
                                br_if 0 (;@14;)
                                br 3 (;@11;)
                              end
                            end
                            block  ;; label = @13
                              block  ;; label = @14
                                i32.const 0
                                i32.load offset=1049300
                                local.tee 0
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 0
                                local.get 5
                                i32.le_u
                                br_if 1 (;@13;)
                              end
                              i32.const 0
                              local.get 5
                              i32.store offset=1049300
                            end
                            i32.const 0
                            i32.const 4095
                            i32.store offset=1049304
                            i32.const 0
                            local.get 8
                            i32.store offset=1048988
                            i32.const 0
                            local.get 5
                            i32.store offset=1048984
                            i32.const 0
                            i32.const 1049000
                            i32.store offset=1049012
                            i32.const 0
                            i32.const 1049008
                            i32.store offset=1049020
                            i32.const 0
                            i32.const 1049000
                            i32.store offset=1049008
                            i32.const 0
                            i32.const 1049016
                            i32.store offset=1049028
                            i32.const 0
                            i32.const 1049008
                            i32.store offset=1049016
                            i32.const 0
                            i32.const 1049024
                            i32.store offset=1049036
                            i32.const 0
                            i32.const 1049016
                            i32.store offset=1049024
                            i32.const 0
                            i32.const 1049032
                            i32.store offset=1049044
                            i32.const 0
                            i32.const 1049024
                            i32.store offset=1049032
                            i32.const 0
                            i32.const 1049040
                            i32.store offset=1049052
                            i32.const 0
                            i32.const 1049032
                            i32.store offset=1049040
                            i32.const 0
                            i32.const 1049048
                            i32.store offset=1049060
                            i32.const 0
                            i32.const 1049040
                            i32.store offset=1049048
                            i32.const 0
                            i32.const 1049056
                            i32.store offset=1049068
                            i32.const 0
                            i32.const 1049048
                            i32.store offset=1049056
                            i32.const 0
                            i32.const 0
                            i32.store offset=1048996
                            i32.const 0
                            i32.const 1049064
                            i32.store offset=1049076
                            i32.const 0
                            i32.const 1049056
                            i32.store offset=1049064
                            i32.const 0
                            i32.const 1049064
                            i32.store offset=1049072
                            i32.const 0
                            i32.const 1049072
                            i32.store offset=1049084
                            i32.const 0
                            i32.const 1049072
                            i32.store offset=1049080
                            i32.const 0
                            i32.const 1049080
                            i32.store offset=1049092
                            i32.const 0
                            i32.const 1049080
                            i32.store offset=1049088
                            i32.const 0
                            i32.const 1049088
                            i32.store offset=1049100
                            i32.const 0
                            i32.const 1049088
                            i32.store offset=1049096
                            i32.const 0
                            i32.const 1049096
                            i32.store offset=1049108
                            i32.const 0
                            i32.const 1049096
                            i32.store offset=1049104
                            i32.const 0
                            i32.const 1049104
                            i32.store offset=1049116
                            i32.const 0
                            i32.const 1049104
                            i32.store offset=1049112
                            i32.const 0
                            i32.const 1049112
                            i32.store offset=1049124
                            i32.const 0
                            i32.const 1049112
                            i32.store offset=1049120
                            i32.const 0
                            i32.const 1049120
                            i32.store offset=1049132
                            i32.const 0
                            i32.const 1049120
                            i32.store offset=1049128
                            i32.const 0
                            i32.const 1049128
                            i32.store offset=1049140
                            i32.const 0
                            i32.const 1049136
                            i32.store offset=1049148
                            i32.const 0
                            i32.const 1049128
                            i32.store offset=1049136
                            i32.const 0
                            i32.const 1049144
                            i32.store offset=1049156
                            i32.const 0
                            i32.const 1049136
                            i32.store offset=1049144
                            i32.const 0
                            i32.const 1049152
                            i32.store offset=1049164
                            i32.const 0
                            i32.const 1049144
                            i32.store offset=1049152
                            i32.const 0
                            i32.const 1049160
                            i32.store offset=1049172
                            i32.const 0
                            i32.const 1049152
                            i32.store offset=1049160
                            i32.const 0
                            i32.const 1049168
                            i32.store offset=1049180
                            i32.const 0
                            i32.const 1049160
                            i32.store offset=1049168
                            i32.const 0
                            i32.const 1049176
                            i32.store offset=1049188
                            i32.const 0
                            i32.const 1049168
                            i32.store offset=1049176
                            i32.const 0
                            i32.const 1049184
                            i32.store offset=1049196
                            i32.const 0
                            i32.const 1049176
                            i32.store offset=1049184
                            i32.const 0
                            i32.const 1049192
                            i32.store offset=1049204
                            i32.const 0
                            i32.const 1049184
                            i32.store offset=1049192
                            i32.const 0
                            i32.const 1049200
                            i32.store offset=1049212
                            i32.const 0
                            i32.const 1049192
                            i32.store offset=1049200
                            i32.const 0
                            i32.const 1049208
                            i32.store offset=1049220
                            i32.const 0
                            i32.const 1049200
                            i32.store offset=1049208
                            i32.const 0
                            i32.const 1049216
                            i32.store offset=1049228
                            i32.const 0
                            i32.const 1049208
                            i32.store offset=1049216
                            i32.const 0
                            i32.const 1049224
                            i32.store offset=1049236
                            i32.const 0
                            i32.const 1049216
                            i32.store offset=1049224
                            i32.const 0
                            i32.const 1049232
                            i32.store offset=1049244
                            i32.const 0
                            i32.const 1049224
                            i32.store offset=1049232
                            i32.const 0
                            i32.const 1049240
                            i32.store offset=1049252
                            i32.const 0
                            i32.const 1049232
                            i32.store offset=1049240
                            i32.const 0
                            i32.const 1049248
                            i32.store offset=1049260
                            i32.const 0
                            i32.const 1049240
                            i32.store offset=1049248
                            i32.const 0
                            local.get 5
                            i32.store offset=1049284
                            i32.const 0
                            i32.const 1049248
                            i32.store offset=1049256
                            i32.const 0
                            local.get 8
                            i32.const -40
                            i32.add
                            local.tee 0
                            i32.store offset=1049276
                            local.get 5
                            local.get 0
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            local.get 5
                            local.get 0
                            i32.add
                            i32.const 40
                            i32.store offset=4
                            i32.const 0
                            i32.const 2097152
                            i32.store offset=1049296
                            br 8 (;@4;)
                          end
                          local.get 1
                          local.get 5
                          i32.ge_u
                          br_if 0 (;@11;)
                          local.get 6
                          local.get 1
                          i32.gt_u
                          br_if 0 (;@11;)
                          local.get 0
                          i32.load offset=12
                          i32.eqz
                          br_if 3 (;@8;)
                        end
                        i32.const 0
                        i32.const 0
                        i32.load offset=1049300
                        local.tee 0
                        local.get 5
                        local.get 0
                        local.get 5
                        i32.lt_u
                        select
                        i32.store offset=1049300
                        local.get 5
                        local.get 8
                        i32.add
                        local.set 6
                        i32.const 1048984
                        local.set 0
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              loop  ;; label = @14
                                local.get 0
                                i32.load
                                local.get 6
                                i32.eq
                                br_if 1 (;@13;)
                                local.get 0
                                i32.load offset=8
                                local.tee 0
                                br_if 0 (;@14;)
                                br 2 (;@12;)
                              end
                            end
                            local.get 0
                            i32.load offset=12
                            i32.eqz
                            br_if 1 (;@11;)
                          end
                          i32.const 1048984
                          local.set 0
                          block  ;; label = @12
                            loop  ;; label = @13
                              block  ;; label = @14
                                local.get 0
                                i32.load
                                local.tee 6
                                local.get 1
                                i32.gt_u
                                br_if 0 (;@14;)
                                local.get 6
                                local.get 0
                                i32.load offset=4
                                i32.add
                                local.tee 6
                                local.get 1
                                i32.gt_u
                                br_if 2 (;@12;)
                              end
                              local.get 0
                              i32.load offset=8
                              local.set 0
                              br 0 (;@13;)
                            end
                          end
                          i32.const 0
                          local.get 5
                          i32.store offset=1049284
                          i32.const 0
                          local.get 8
                          i32.const -40
                          i32.add
                          local.tee 0
                          i32.store offset=1049276
                          local.get 5
                          local.get 0
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 5
                          local.get 0
                          i32.add
                          i32.const 40
                          i32.store offset=4
                          i32.const 0
                          i32.const 2097152
                          i32.store offset=1049296
                          local.get 1
                          local.get 6
                          i32.const -32
                          i32.add
                          i32.const -8
                          i32.and
                          i32.const -8
                          i32.add
                          local.tee 0
                          local.get 0
                          local.get 1
                          i32.const 16
                          i32.add
                          i32.lt_u
                          select
                          local.tee 7
                          i32.const 27
                          i32.store offset=4
                          i32.const 0
                          i64.load offset=1048984 align=4
                          local.set 9
                          local.get 7
                          i32.const 16
                          i32.add
                          i32.const 0
                          i64.load offset=1048992 align=4
                          i64.store align=4
                          local.get 7
                          local.get 9
                          i64.store offset=8 align=4
                          i32.const 0
                          local.get 8
                          i32.store offset=1048988
                          i32.const 0
                          local.get 5
                          i32.store offset=1048984
                          i32.const 0
                          local.get 7
                          i32.const 8
                          i32.add
                          i32.store offset=1048992
                          i32.const 0
                          i32.const 0
                          i32.store offset=1048996
                          local.get 7
                          i32.const 28
                          i32.add
                          local.set 0
                          loop  ;; label = @12
                            local.get 0
                            i32.const 7
                            i32.store
                            local.get 0
                            i32.const 4
                            i32.add
                            local.tee 0
                            local.get 6
                            i32.lt_u
                            br_if 0 (;@12;)
                          end
                          local.get 7
                          local.get 1
                          i32.eq
                          br_if 7 (;@4;)
                          local.get 7
                          local.get 7
                          i32.load offset=4
                          i32.const -2
                          i32.and
                          i32.store offset=4
                          local.get 1
                          local.get 7
                          local.get 1
                          i32.sub
                          local.tee 0
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 7
                          local.get 0
                          i32.store
                          block  ;; label = @12
                            local.get 0
                            i32.const 256
                            i32.lt_u
                            br_if 0 (;@12;)
                            local.get 1
                            local.get 0
                            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17hd539d96635297a05E
                            br 8 (;@4;)
                          end
                          local.get 0
                          i32.const -8
                          i32.and
                          i32.const 1049000
                          i32.add
                          local.set 6
                          block  ;; label = @12
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=1049264
                              local.tee 5
                              i32.const 1
                              local.get 0
                              i32.const 3
                              i32.shr_u
                              i32.shl
                              local.tee 0
                              i32.and
                              br_if 0 (;@13;)
                              i32.const 0
                              local.get 5
                              local.get 0
                              i32.or
                              i32.store offset=1049264
                              local.get 6
                              local.set 0
                              br 1 (;@12;)
                            end
                            local.get 6
                            i32.load offset=8
                            local.set 0
                          end
                          local.get 6
                          local.get 1
                          i32.store offset=8
                          local.get 0
                          local.get 1
                          i32.store offset=12
                          local.get 1
                          local.get 6
                          i32.store offset=12
                          local.get 1
                          local.get 0
                          i32.store offset=8
                          br 7 (;@4;)
                        end
                        local.get 0
                        local.get 5
                        i32.store
                        local.get 0
                        local.get 0
                        i32.load offset=4
                        local.get 8
                        i32.add
                        i32.store offset=4
                        local.get 5
                        local.get 2
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 6
                        local.get 5
                        local.get 2
                        i32.add
                        local.tee 0
                        i32.sub
                        local.set 2
                        local.get 6
                        i32.const 0
                        i32.load offset=1049284
                        i32.eq
                        br_if 3 (;@7;)
                        local.get 6
                        i32.const 0
                        i32.load offset=1049280
                        i32.eq
                        br_if 4 (;@6;)
                        block  ;; label = @11
                          local.get 6
                          i32.load offset=4
                          local.tee 1
                          i32.const 3
                          i32.and
                          i32.const 1
                          i32.ne
                          br_if 0 (;@11;)
                          local.get 6
                          local.get 1
                          i32.const -8
                          i32.and
                          local.tee 1
                          call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$12unlink_chunk17h22188baaa299953bE
                          local.get 1
                          local.get 2
                          i32.add
                          local.set 2
                          local.get 6
                          local.get 1
                          i32.add
                          local.tee 6
                          i32.load offset=4
                          local.set 1
                        end
                        local.get 6
                        local.get 1
                        i32.const -2
                        i32.and
                        i32.store offset=4
                        local.get 0
                        local.get 2
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 0
                        local.get 2
                        i32.add
                        local.get 2
                        i32.store
                        block  ;; label = @11
                          local.get 2
                          i32.const 256
                          i32.lt_u
                          br_if 0 (;@11;)
                          local.get 0
                          local.get 2
                          call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17hd539d96635297a05E
                          br 6 (;@5;)
                        end
                        local.get 2
                        i32.const -8
                        i32.and
                        i32.const 1049000
                        i32.add
                        local.set 1
                        block  ;; label = @11
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=1049264
                            local.tee 6
                            i32.const 1
                            local.get 2
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 2
                            i32.and
                            br_if 0 (;@12;)
                            i32.const 0
                            local.get 6
                            local.get 2
                            i32.or
                            i32.store offset=1049264
                            local.get 1
                            local.set 2
                            br 1 (;@11;)
                          end
                          local.get 1
                          i32.load offset=8
                          local.set 2
                        end
                        local.get 1
                        local.get 0
                        i32.store offset=8
                        local.get 2
                        local.get 0
                        i32.store offset=12
                        local.get 0
                        local.get 1
                        i32.store offset=12
                        local.get 0
                        local.get 2
                        i32.store offset=8
                        br 5 (;@5;)
                      end
                      i32.const 0
                      local.get 0
                      local.get 2
                      i32.sub
                      local.tee 1
                      i32.store offset=1049276
                      i32.const 0
                      i32.const 0
                      i32.load offset=1049284
                      local.tee 0
                      local.get 2
                      i32.add
                      local.tee 6
                      i32.store offset=1049284
                      local.get 6
                      local.get 1
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 2
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 0
                      i32.const 8
                      i32.add
                      local.set 1
                      br 6 (;@3;)
                    end
                    i32.const 0
                    i32.load offset=1049280
                    local.set 1
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        local.get 2
                        i32.sub
                        local.tee 6
                        i32.const 15
                        i32.gt_u
                        br_if 0 (;@10;)
                        i32.const 0
                        i32.const 0
                        i32.store offset=1049280
                        i32.const 0
                        i32.const 0
                        i32.store offset=1049272
                        local.get 1
                        local.get 0
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 1
                        local.get 0
                        i32.add
                        local.tee 0
                        local.get 0
                        i32.load offset=4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        br 1 (;@9;)
                      end
                      i32.const 0
                      local.get 6
                      i32.store offset=1049272
                      i32.const 0
                      local.get 1
                      local.get 2
                      i32.add
                      local.tee 5
                      i32.store offset=1049280
                      local.get 5
                      local.get 6
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 1
                      local.get 0
                      i32.add
                      local.get 6
                      i32.store
                      local.get 1
                      local.get 2
                      i32.const 3
                      i32.or
                      i32.store offset=4
                    end
                    local.get 1
                    i32.const 8
                    i32.add
                    return
                  end
                  local.get 0
                  local.get 7
                  local.get 8
                  i32.add
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=1049284
                  local.tee 0
                  i32.const 15
                  i32.add
                  i32.const -8
                  i32.and
                  local.tee 1
                  i32.const -8
                  i32.add
                  local.tee 6
                  i32.store offset=1049284
                  i32.const 0
                  local.get 0
                  local.get 1
                  i32.sub
                  i32.const 0
                  i32.load offset=1049276
                  local.get 8
                  i32.add
                  local.tee 1
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 5
                  i32.store offset=1049276
                  local.get 6
                  local.get 5
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  local.get 1
                  i32.add
                  i32.const 40
                  i32.store offset=4
                  i32.const 0
                  i32.const 2097152
                  i32.store offset=1049296
                  br 3 (;@4;)
                end
                i32.const 0
                local.get 0
                i32.store offset=1049284
                i32.const 0
                i32.const 0
                i32.load offset=1049276
                local.get 2
                i32.add
                local.tee 2
                i32.store offset=1049276
                local.get 0
                local.get 2
                i32.const 1
                i32.or
                i32.store offset=4
                br 1 (;@5;)
              end
              i32.const 0
              local.get 0
              i32.store offset=1049280
              i32.const 0
              i32.const 0
              i32.load offset=1049272
              local.get 2
              i32.add
              local.tee 2
              i32.store offset=1049272
              local.get 0
              local.get 2
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 2
              i32.add
              local.get 2
              i32.store
            end
            local.get 5
            i32.const 8
            i32.add
            return
          end
          i32.const 0
          local.set 1
          i32.const 0
          i32.load offset=1049276
          local.tee 0
          local.get 2
          i32.le_u
          br_if 0 (;@3;)
          i32.const 0
          local.get 0
          local.get 2
          i32.sub
          local.tee 1
          i32.store offset=1049276
          i32.const 0
          i32.const 0
          i32.load offset=1049284
          local.tee 0
          local.get 2
          i32.add
          local.tee 6
          i32.store offset=1049284
          local.get 6
          local.get 1
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          local.get 2
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          i32.const 8
          i32.add
          return
        end
        local.get 1
        return
      end
      local.get 0
      local.get 4
      i32.store offset=24
      block  ;; label = @2
        local.get 6
        i32.load offset=16
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.get 5
        i32.store offset=16
        local.get 5
        local.get 0
        i32.store offset=24
      end
      local.get 6
      i32.load offset=20
      local.tee 5
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 5
      i32.store offset=20
      local.get 5
      local.get 0
      i32.store offset=24
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 16
        i32.lt_u
        br_if 0 (;@2;)
        local.get 6
        local.get 2
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 6
        local.get 2
        i32.add
        local.tee 0
        local.get 1
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        local.get 1
        i32.add
        local.get 1
        i32.store
        block  ;; label = @3
          local.get 1
          i32.const 256
          i32.lt_u
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17hd539d96635297a05E
          br 2 (;@1;)
        end
        local.get 1
        i32.const -8
        i32.and
        i32.const 1049000
        i32.add
        local.set 2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=1049264
            local.tee 5
            i32.const 1
            local.get 1
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 1
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            local.get 5
            local.get 1
            i32.or
            i32.store offset=1049264
            local.get 2
            local.set 1
            br 1 (;@3;)
          end
          local.get 2
          i32.load offset=8
          local.set 1
        end
        local.get 2
        local.get 0
        i32.store offset=8
        local.get 1
        local.get 0
        i32.store offset=12
        local.get 0
        local.get 2
        i32.store offset=12
        local.get 0
        local.get 1
        i32.store offset=8
        br 1 (;@1;)
      end
      local.get 6
      local.get 1
      local.get 2
      i32.add
      local.tee 0
      i32.const 3
      i32.or
      i32.store offset=4
      local.get 6
      local.get 0
      i32.add
      local.tee 0
      local.get 0
      i32.load offset=4
      i32.const 1
      i32.or
      i32.store offset=4
    end
    local.get 6
    i32.const 8
    i32.add)
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17h1ca6b53be59cc4abE (type 0) (param i32)
    (local i32 i32 i32 i32 i32)
    local.get 0
    i32.const -8
    i32.add
    local.tee 1
    local.get 0
    i32.const -4
    i32.add
    i32.load
    local.tee 2
    i32.const -8
    i32.and
    local.tee 0
    i32.add
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 2
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        i32.load
        local.tee 2
        local.get 0
        i32.add
        local.set 0
        block  ;; label = @3
          local.get 1
          local.get 2
          i32.sub
          local.tee 1
          i32.const 0
          i32.load offset=1049280
          i32.ne
          br_if 0 (;@3;)
          local.get 3
          i32.load offset=4
          i32.const 3
          i32.and
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          i32.const 0
          local.get 0
          i32.store offset=1049272
          local.get 3
          local.get 3
          i32.load offset=4
          i32.const -2
          i32.and
          i32.store offset=4
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 3
          local.get 0
          i32.store
          return
        end
        local.get 1
        local.get 2
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$12unlink_chunk17h22188baaa299953bE
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 3
                    i32.load offset=4
                    local.tee 2
                    i32.const 2
                    i32.and
                    br_if 0 (;@8;)
                    local.get 3
                    i32.const 0
                    i32.load offset=1049284
                    i32.eq
                    br_if 2 (;@6;)
                    local.get 3
                    i32.const 0
                    i32.load offset=1049280
                    i32.eq
                    br_if 6 (;@2;)
                    local.get 3
                    local.get 2
                    i32.const -8
                    i32.and
                    local.tee 2
                    call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$12unlink_chunk17h22188baaa299953bE
                    local.get 1
                    local.get 2
                    local.get 0
                    i32.add
                    local.tee 0
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 1
                    local.get 0
                    i32.add
                    local.get 0
                    i32.store
                    local.get 1
                    i32.const 0
                    i32.load offset=1049280
                    i32.ne
                    br_if 1 (;@7;)
                    i32.const 0
                    local.get 0
                    i32.store offset=1049272
                    return
                  end
                  local.get 3
                  local.get 2
                  i32.const -2
                  i32.and
                  i32.store offset=4
                  local.get 1
                  local.get 0
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 1
                  local.get 0
                  i32.add
                  local.get 0
                  i32.store
                end
                local.get 0
                i32.const 256
                i32.lt_u
                br_if 3 (;@3;)
                i32.const 31
                local.set 3
                block  ;; label = @7
                  local.get 0
                  i32.const 16777215
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 0
                  i32.const 6
                  local.get 0
                  i32.const 8
                  i32.shr_u
                  i32.clz
                  local.tee 3
                  i32.sub
                  i32.shr_u
                  i32.const 1
                  i32.and
                  local.get 3
                  i32.const 1
                  i32.shl
                  i32.sub
                  i32.const 62
                  i32.add
                  local.set 3
                end
                local.get 1
                i64.const 0
                i64.store offset=16 align=4
                local.get 1
                local.get 3
                i32.store offset=28
                local.get 3
                i32.const 2
                i32.shl
                i32.const 1048856
                i32.add
                local.set 2
                i32.const 0
                i32.load offset=1049268
                i32.const 1
                local.get 3
                i32.shl
                local.tee 4
                i32.and
                br_if 1 (;@5;)
                local.get 2
                local.get 1
                i32.store
                local.get 1
                local.get 2
                i32.store offset=24
                local.get 1
                local.get 1
                i32.store offset=12
                local.get 1
                local.get 1
                i32.store offset=8
                i32.const 0
                i32.const 0
                i32.load offset=1049268
                local.get 4
                i32.or
                i32.store offset=1049268
                br 2 (;@4;)
              end
              i32.const 0
              local.get 1
              i32.store offset=1049284
              i32.const 0
              i32.const 0
              i32.load offset=1049276
              local.get 0
              i32.add
              local.tee 0
              i32.store offset=1049276
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              block  ;; label = @6
                local.get 1
                i32.const 0
                i32.load offset=1049280
                i32.ne
                br_if 0 (;@6;)
                i32.const 0
                i32.const 0
                i32.store offset=1049272
                i32.const 0
                i32.const 0
                i32.store offset=1049280
              end
              local.get 0
              i32.const 0
              i32.load offset=1049296
              local.tee 4
              i32.le_u
              br_if 4 (;@1;)
              i32.const 0
              i32.load offset=1049284
              local.tee 3
              i32.eqz
              br_if 4 (;@1;)
              i32.const 0
              local.set 1
              block  ;; label = @6
                i32.const 0
                i32.load offset=1049276
                local.tee 5
                i32.const 41
                i32.lt_u
                br_if 0 (;@6;)
                i32.const 1048984
                local.set 0
                loop  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    i32.load
                    local.tee 2
                    local.get 3
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 2
                    local.get 0
                    i32.load offset=4
                    i32.add
                    local.get 3
                    i32.gt_u
                    br_if 2 (;@6;)
                  end
                  local.get 0
                  i32.load offset=8
                  local.tee 0
                  br_if 0 (;@7;)
                end
              end
              block  ;; label = @6
                i32.const 0
                i32.load offset=1048992
                local.tee 0
                i32.eqz
                br_if 0 (;@6;)
                i32.const 0
                local.set 1
                loop  ;; label = @7
                  local.get 1
                  i32.const 1
                  i32.add
                  local.set 1
                  local.get 0
                  i32.load offset=8
                  local.tee 0
                  br_if 0 (;@7;)
                end
              end
              i32.const 0
              local.get 1
              i32.const 4095
              local.get 1
              i32.const 4095
              i32.gt_u
              select
              i32.store offset=1049304
              local.get 5
              local.get 4
              i32.le_u
              br_if 4 (;@1;)
              i32.const 0
              i32.const -1
              i32.store offset=1049296
              br 4 (;@1;)
            end
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 2
                  i32.load
                  local.tee 4
                  i32.load offset=4
                  i32.const -8
                  i32.and
                  local.get 0
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 4
                  local.set 3
                  br 1 (;@6;)
                end
                local.get 0
                i32.const 0
                i32.const 25
                local.get 3
                i32.const 1
                i32.shr_u
                i32.sub
                local.get 3
                i32.const 31
                i32.eq
                select
                i32.shl
                local.set 2
                loop  ;; label = @7
                  local.get 4
                  local.get 2
                  i32.const 29
                  i32.shr_u
                  i32.const 4
                  i32.and
                  i32.add
                  i32.const 16
                  i32.add
                  local.tee 5
                  i32.load
                  local.tee 3
                  i32.eqz
                  br_if 2 (;@5;)
                  local.get 2
                  i32.const 1
                  i32.shl
                  local.set 2
                  local.get 3
                  local.set 4
                  local.get 3
                  i32.load offset=4
                  i32.const -8
                  i32.and
                  local.get 0
                  i32.ne
                  br_if 0 (;@7;)
                end
              end
              local.get 3
              i32.load offset=8
              local.tee 0
              local.get 1
              i32.store offset=12
              local.get 3
              local.get 1
              i32.store offset=8
              local.get 1
              i32.const 0
              i32.store offset=24
              local.get 1
              local.get 3
              i32.store offset=12
              local.get 1
              local.get 0
              i32.store offset=8
              br 1 (;@4;)
            end
            local.get 5
            local.get 1
            i32.store
            local.get 1
            local.get 4
            i32.store offset=24
            local.get 1
            local.get 1
            i32.store offset=12
            local.get 1
            local.get 1
            i32.store offset=8
          end
          i32.const 0
          local.set 1
          i32.const 0
          i32.const 0
          i32.load offset=1049304
          i32.const -1
          i32.add
          local.tee 0
          i32.store offset=1049304
          local.get 0
          br_if 2 (;@1;)
          block  ;; label = @4
            i32.const 0
            i32.load offset=1048992
            local.tee 0
            i32.eqz
            br_if 0 (;@4;)
            i32.const 0
            local.set 1
            loop  ;; label = @5
              local.get 1
              i32.const 1
              i32.add
              local.set 1
              local.get 0
              i32.load offset=8
              local.tee 0
              br_if 0 (;@5;)
            end
          end
          i32.const 0
          local.get 1
          i32.const 4095
          local.get 1
          i32.const 4095
          i32.gt_u
          select
          i32.store offset=1049304
          return
        end
        local.get 0
        i32.const -8
        i32.and
        i32.const 1049000
        i32.add
        local.set 3
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=1049264
            local.tee 2
            i32.const 1
            local.get 0
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 0
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            local.get 2
            local.get 0
            i32.or
            i32.store offset=1049264
            local.get 3
            local.set 0
            br 1 (;@3;)
          end
          local.get 3
          i32.load offset=8
          local.set 0
        end
        local.get 3
        local.get 1
        i32.store offset=8
        local.get 0
        local.get 1
        i32.store offset=12
        local.get 1
        local.get 3
        i32.store offset=12
        local.get 1
        local.get 0
        i32.store offset=8
        return
      end
      i32.const 0
      local.get 1
      i32.store offset=1049280
      i32.const 0
      i32.const 0
      i32.load offset=1049272
      local.get 0
      i32.add
      local.tee 0
      i32.store offset=1049272
      local.get 1
      local.get 0
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 1
      local.get 0
      i32.add
      local.get 0
      i32.store
      return
    end)
  (func $_ZN4core9panicking5panic17h7587eba6ad042231E (type 6) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 1
    i32.store offset=4
    local.get 3
    i64.const 0
    i64.store offset=12 align=4
    local.get 3
    i32.const 1048636
    i32.store offset=8
    local.get 3
    local.get 1
    i32.store offset=28
    local.get 3
    local.get 0
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 24
    i32.add
    i32.store
    local.get 3
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h962c0a269d1457efE
    unreachable)
  (func $_ZN4core9panicking9panic_fmt17h962c0a269d1457efE (type 5) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 1
    i32.store16 offset=28
    local.get 2
    local.get 1
    i32.store offset=24
    local.get 2
    local.get 0
    i32.store offset=20
    local.get 2
    i32.const 1048620
    i32.store offset=16
    local.get 2
    i32.const 1048636
    i32.store offset=12
    local.get 2
    i32.const 12
    i32.add
    call $rust_begin_unwind
    unreachable)
  (func $rust_begin_unwind (type 0) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=8
      local.tee 1
      br_if 0 (;@1;)
      call $_ZN4core6option13unwrap_failed17hd65fb1cd095b72e1E
      unreachable
    end
    local.get 1
    local.get 0
    call $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hbe35c3dc61cbc47fE
    unreachable)
  (func $_ZN4core3ptr37drop_in_place$LT$core..fmt..Error$GT$17h36b1182ddd7549f3E (type 0) (param i32))
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h764d0138e2733e60E (type 5) (param i32 i32)
    local.get 0
    i64.const -3022690870250374772
    i64.store offset=8
    local.get 0
    i64.const 9025657056811852811
    i64.store)
  (func $_ZN4core6option13unwrap_failed17hd65fb1cd095b72e1E (type 3)
    i32.const 1048576
    i32.const 43
    i32.const 1048832
    call $_ZN4core9panicking5panic17h7587eba6ad042231E
    unreachable)
  (func $rust_panic (type 3)
    unreachable
    unreachable)
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17hd539d96635297a05E (type 5) (param i32 i32)
    (local i32 i32 i32 i32)
    i32.const 31
    local.set 2
    block  ;; label = @1
      local.get 1
      i32.const 16777215
      i32.gt_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 6
      local.get 1
      i32.const 8
      i32.shr_u
      i32.clz
      local.tee 2
      i32.sub
      i32.shr_u
      i32.const 1
      i32.and
      local.get 2
      i32.const 1
      i32.shl
      i32.sub
      i32.const 62
      i32.add
      local.set 2
    end
    local.get 0
    i64.const 0
    i64.store offset=16 align=4
    local.get 0
    local.get 2
    i32.store offset=28
    local.get 2
    i32.const 2
    i32.shl
    i32.const 1048856
    i32.add
    local.set 3
    block  ;; label = @1
      i32.const 0
      i32.load offset=1049268
      i32.const 1
      local.get 2
      i32.shl
      local.tee 4
      i32.and
      br_if 0 (;@1;)
      local.get 3
      local.get 0
      i32.store
      local.get 0
      local.get 3
      i32.store offset=24
      local.get 0
      local.get 0
      i32.store offset=12
      local.get 0
      local.get 0
      i32.store offset=8
      i32.const 0
      i32.const 0
      i32.load offset=1049268
      local.get 4
      i32.or
      i32.store offset=1049268
      return
    end
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.load
          local.tee 4
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 1
          i32.ne
          br_if 0 (;@3;)
          local.get 4
          local.set 2
          br 1 (;@2;)
        end
        local.get 1
        i32.const 0
        i32.const 25
        local.get 2
        i32.const 1
        i32.shr_u
        i32.sub
        local.get 2
        i32.const 31
        i32.eq
        select
        i32.shl
        local.set 3
        loop  ;; label = @3
          local.get 4
          local.get 3
          i32.const 29
          i32.shr_u
          i32.const 4
          i32.and
          i32.add
          i32.const 16
          i32.add
          local.tee 5
          i32.load
          local.tee 2
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.const 1
          i32.shl
          local.set 3
          local.get 2
          local.set 4
          local.get 2
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 1
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 2
      i32.load offset=8
      local.tee 3
      local.get 0
      i32.store offset=12
      local.get 2
      local.get 0
      i32.store offset=8
      local.get 0
      i32.const 0
      i32.store offset=24
      local.get 0
      local.get 2
      i32.store offset=12
      local.get 0
      local.get 3
      i32.store offset=8
      return
    end
    local.get 5
    local.get 0
    i32.store
    local.get 0
    local.get 4
    i32.store offset=24
    local.get 0
    local.get 0
    i32.store offset=12
    local.get 0
    local.get 0
    i32.store offset=8)
  (func $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hbe35c3dc61cbc47fE (type 5) (param i32 i32)
    local.get 0
    local.get 1
    call $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h05184e899f59c350E
    unreachable)
  (func $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h05184e899f59c350E (type 5) (param i32 i32)
    (local i32)
    local.get 0
    i32.load offset=12
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.load offset=4
            br_table 0 (;@4;) 1 (;@3;) 2 (;@2;)
          end
          local.get 2
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 1
      i32.load8_u offset=16
      local.get 1
      i32.load8_u offset=17
      call $_ZN3std9panicking20rust_panic_with_hook17h0284e938a621504dE
      unreachable
    end
    local.get 1
    i32.load8_u offset=16
    local.get 1
    i32.load8_u offset=17
    call $_ZN3std9panicking20rust_panic_with_hook17h0284e938a621504dE
    unreachable)
  (func $_ZN3std9panicking20rust_panic_with_hook17h0284e938a621504dE (type 5) (param i32 i32)
    (local i32)
    i32.const 0
    i32.const 0
    i32.load offset=1048852
    local.tee 2
    i32.const 1
    i32.add
    i32.store offset=1048852
    block  ;; label = @1
      local.get 2
      i32.const 0
      i32.lt_s
      br_if 0 (;@1;)
      i32.const 0
      i32.load8_u offset=1049312
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      i32.load offset=1049308
      i32.const 1
      i32.add
      i32.store offset=1049308
      i32.const 0
      i32.load offset=1048848
      i32.const -1
      i32.le_s
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      i32.store8 offset=1049312
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      call $rust_panic
      unreachable
    end
    unreachable
    unreachable)
  (func $_ZN5alloc7raw_vec11finish_grow17h4c3ee3312076eda1E (type 7) (param i32 i32 i32 i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          i32.const 0
          i32.lt_s
          br_if 1 (;@2;)
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 3
                i32.load offset=4
                i32.eqz
                br_if 0 (;@6;)
                block  ;; label = @7
                  local.get 3
                  i32.load offset=8
                  local.tee 4
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    local.get 2
                    br_if 0 (;@8;)
                    local.get 1
                    local.set 3
                    br 4 (;@4;)
                  end
                  i32.const 0
                  i32.load8_u offset=1049313
                  drop
                  local.get 1
                  i32.const 9
                  i32.lt_u
                  br_if 2 (;@5;)
                  local.get 1
                  local.get 2
                  call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8memalign17h234873f561ba0f37E
                  local.set 3
                  br 3 (;@4;)
                end
                local.get 3
                i32.load
                local.get 4
                local.get 1
                local.get 2
                call $__rust_realloc
                local.set 3
                br 2 (;@4;)
              end
              block  ;; label = @6
                local.get 2
                br_if 0 (;@6;)
                local.get 1
                local.set 3
                br 2 (;@4;)
              end
              i32.const 0
              i32.load8_u offset=1049313
              drop
              local.get 1
              i32.const 9
              i32.lt_u
              br_if 0 (;@5;)
              local.get 1
              local.get 2
              call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8memalign17h234873f561ba0f37E
              local.set 3
              br 1 (;@4;)
            end
            local.get 2
            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17h9867234b8d2950f1E
            local.set 3
          end
          block  ;; label = @4
            local.get 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            local.get 2
            i32.store offset=8
            local.get 0
            local.get 3
            i32.store offset=4
            local.get 0
            i32.const 0
            i32.store
            return
          end
          local.get 0
          local.get 2
          i32.store offset=8
          local.get 0
          local.get 1
          i32.store offset=4
          br 2 (;@1;)
        end
        local.get 0
        i32.const 0
        i32.store offset=4
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      i32.store offset=4
    end
    local.get 0
    i32.const 1
    i32.store)
  (func $_ZN3std3sys12thread_local12static_local12Key$LT$T$GT$3get17h56ba68706616eab7E (type 3)
    block  ;; label = @1
      i32.const 0
      i32.load offset=1049316
      br_if 0 (;@1;)
      i32.const 0
      i64.const 0
      i64.store offset=1049332 align=4
      i32.const 0
      i64.const 4
      i64.store offset=1049324 align=4
      i32.const 0
      i64.const 1
      i64.store offset=1049316 align=4
    end)
  (func $__wbindgen_exn_store (type 0) (param i32))
  (func $__wbindgen_malloc (type 2) (param i32 i32) (result i32)
    block  ;; label = @1
      local.get 1
      i32.popcnt
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      i32.const -2147483648
      local.get 1
      i32.sub
      local.get 0
      i32.lt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 0
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        i32.load8_u offset=1049313
        drop
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 9
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            local.get 0
            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8memalign17h234873f561ba0f37E
            local.set 1
            br 1 (;@3;)
          end
          local.get 0
          call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17h9867234b8d2950f1E
          local.set 1
        end
        local.get 1
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 1
      return
    end
    unreachable
    unreachable)
  (func $__wbindgen_realloc (type 4) (param i32 i32 i32 i32) (result i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.popcnt
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        i32.const -2147483648
        local.get 3
        i32.sub
        local.get 1
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        local.get 3
        local.get 2
        call $__rust_realloc
        local.tee 3
        br_if 1 (;@1;)
      end
      unreachable
      unreachable
    end
    local.get 3)
  (func $__wbindgen_free (type 6) (param i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.const -4
          i32.add
          i32.load
          local.tee 3
          i32.const -8
          i32.and
          local.tee 4
          i32.const 4
          i32.const 8
          local.get 3
          i32.const 3
          i32.and
          local.tee 3
          select
          local.get 1
          i32.add
          i32.lt_u
          br_if 1 (;@2;)
          block  ;; label = @4
            local.get 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            local.get 1
            i32.const 39
            i32.add
            i32.gt_u
            br_if 3 (;@1;)
          end
          local.get 0
          call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17h1ca6b53be59cc4abE
        end
        return
      end
      i32.const 1048677
      i32.const 46
      i32.const 1048724
      call $_ZN4core9panicking5panic17h7587eba6ad042231E
      unreachable
    end
    i32.const 1048740
    i32.const 46
    i32.const 1048788
    call $_ZN4core9panicking5panic17h7587eba6ad042231E
    unreachable)
  (func $__externref_table_alloc (type 8) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    call $_ZN3std3sys12thread_local12static_local12Key$LT$T$GT$3get17h56ba68706616eab7E
    i32.const 0
    i32.load offset=1049336
    local.set 1
    i32.const 0
    i32.load offset=1049332
    local.set 2
    i32.const 0
    i64.const 0
    i64.store offset=1049332 align=4
    i32.const 0
    i32.load offset=1049324
    local.set 3
    i32.const 0
    i32.load offset=1049328
    local.set 4
    i32.const 0
    i64.const 4
    i64.store offset=1049324 align=4
    i32.const 0
    i32.load offset=1049320
    local.set 5
    i32.const 0
    i32.const 0
    i32.store offset=1049320
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              local.get 4
              i32.eq
              br_if 0 (;@5;)
              local.get 5
              local.set 6
              br 1 (;@4;)
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 4
                local.get 5
                i32.eq
                br_if 0 (;@6;)
                local.get 5
                local.set 6
                local.get 4
                local.set 5
                br 1 (;@5;)
              end
              local.get 5
              i32.const 128
              local.get 5
              i32.const 128
              i32.gt_u
              select
              local.tee 7
              call $_ZN12wasm_bindgen9externref31__wbindgen_externref_table_grow17h55902b3cc6f95e75E
              local.tee 6
              i32.const -1
              i32.eq
              br_if 2 (;@3;)
              block  ;; label = @6
                block  ;; label = @7
                  local.get 1
                  br_if 0 (;@7;)
                  local.get 6
                  local.set 1
                  br 1 (;@6;)
                end
                local.get 1
                local.get 5
                i32.add
                local.get 6
                i32.ne
                br_if 3 (;@3;)
              end
              local.get 5
              local.get 7
              i32.add
              local.tee 6
              local.get 5
              i32.lt_u
              br_if 2 (;@3;)
              local.get 6
              i32.const 2
              i32.shl
              local.set 7
              local.get 6
              i32.const 536870912
              i32.lt_u
              i32.const 2
              i32.shl
              local.set 8
              block  ;; label = @6
                block  ;; label = @7
                  local.get 5
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 3
                  br 1 (;@6;)
                end
                local.get 0
                local.get 3
                i32.store offset=20
                local.get 0
                local.get 5
                i32.const 2
                i32.shl
                i32.store offset=28
                i32.const 4
                local.set 3
              end
              local.get 0
              local.get 3
              i32.store offset=24
              local.get 0
              i32.const 8
              i32.add
              local.get 8
              local.get 7
              local.get 0
              i32.const 20
              i32.add
              call $_ZN5alloc7raw_vec11finish_grow17h4c3ee3312076eda1E
              local.get 0
              i32.load offset=8
              br_if 2 (;@3;)
              local.get 0
              i32.load offset=12
              local.set 3
            end
            local.get 5
            local.get 6
            i32.ge_u
            br_if 1 (;@3;)
            local.get 3
            local.get 5
            i32.const 2
            i32.shl
            i32.add
            local.get 4
            i32.const 1
            i32.add
            local.tee 4
            i32.store
          end
          local.get 2
          local.get 4
          i32.ge_u
          br_if 0 (;@3;)
          local.get 3
          local.get 2
          i32.const 2
          i32.shl
          i32.add
          i32.load
          local.set 5
          i32.const 0
          local.get 1
          i32.store offset=1049336
          i32.const 0
          local.get 5
          i32.store offset=1049332
          i32.const 0
          local.get 4
          i32.store offset=1049328
          i32.const 0
          i32.load offset=1049324
          local.set 5
          i32.const 0
          local.get 3
          i32.store offset=1049324
          i32.const 0
          i32.load offset=1049320
          local.set 4
          i32.const 0
          local.get 6
          i32.store offset=1049320
          block  ;; label = @4
            local.get 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            i32.const -4
            i32.add
            i32.load
            local.tee 3
            i32.const -8
            i32.and
            local.tee 6
            i32.const 4
            i32.const 8
            local.get 3
            i32.const 3
            i32.and
            local.tee 3
            select
            local.get 4
            i32.const 2
            i32.shl
            local.tee 4
            i32.add
            i32.lt_u
            br_if 2 (;@2;)
            block  ;; label = @5
              local.get 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 6
              local.get 4
              i32.const 39
              i32.add
              i32.gt_u
              br_if 4 (;@1;)
            end
            local.get 5
            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17h1ca6b53be59cc4abE
          end
          local.get 0
          i32.const 32
          i32.add
          global.set $__stack_pointer
          local.get 1
          local.get 2
          i32.add
          return
        end
        unreachable
        unreachable
      end
      i32.const 1048677
      i32.const 46
      i32.const 1048724
      call $_ZN4core9panicking5panic17h7587eba6ad042231E
      unreachable
    end
    i32.const 1048740
    i32.const 46
    i32.const 1048788
    call $_ZN4core9panicking5panic17h7587eba6ad042231E
    unreachable)
  (func $__externref_table_dealloc (type 0) (param i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 132
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        call $_ZN12wasm_bindgen9externref35__wbindgen_externref_table_set_null17h01aaad0fb9a4ac4cE
        call $_ZN3std3sys12thread_local12static_local12Key$LT$T$GT$3get17h56ba68706616eab7E
        local.get 0
        i32.const 0
        i32.load offset=1049336
        local.tee 1
        i32.lt_u
        br_if 1 (;@1;)
        local.get 0
        local.get 1
        i32.sub
        local.tee 0
        i32.const 0
        i32.load offset=1049328
        i32.ge_u
        br_if 1 (;@1;)
        i32.const 0
        i32.load offset=1049324
        local.get 0
        i32.const 2
        i32.shl
        i32.add
        i32.const 0
        i32.load offset=1049332
        i32.store
        i32.const 0
        local.get 0
        i32.store offset=1049332
      end
      return
    end
    unreachable
    unreachable)
  (func $__externref_drop_slice (type 5) (param i32 i32)
    block  ;; label = @1
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 2
      i32.shl
      local.set 1
      loop  ;; label = @2
        local.get 0
        i32.load
        call $__externref_table_dealloc
        local.get 0
        i32.const 4
        i32.add
        local.set 0
        local.get 1
        i32.const -4
        i32.add
        local.tee 1
        br_if 0 (;@2;)
      end
    end)
  (func $__externref_heap_live_count (type 8) (result i32)
    (local i32 i32 i32 i32)
    call $_ZN3std3sys12thread_local12static_local12Key$LT$T$GT$3get17h56ba68706616eab7E
    i32.const 0
    local.set 0
    block  ;; label = @1
      i32.const 0
      i32.load offset=1049332
      local.tee 1
      i32.const 0
      i32.load offset=1049328
      local.tee 2
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 0
      i32.load offset=1049324
      local.set 3
      i32.const 0
      local.set 0
      loop  ;; label = @2
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 3
        local.get 1
        i32.const 2
        i32.shl
        i32.add
        i32.load
        local.tee 1
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    local.get 2
    local.get 0
    i32.sub)
  (func $_ZN17compiler_builtins3mem6memcpy17h429fa537573077b3E (type 9) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 16
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      local.get 0
      i32.sub
      i32.const 3
      i32.and
      local.tee 4
      i32.add
      local.set 5
      block  ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        local.get 1
        local.set 6
        loop  ;; label = @3
          local.get 3
          local.get 6
          i32.load8_u
          i32.store8
          local.get 6
          i32.const 1
          i32.add
          local.set 6
          local.get 3
          i32.const 1
          i32.add
          local.tee 3
          local.get 5
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 5
      local.get 2
      local.get 4
      i32.sub
      local.tee 7
      i32.const -4
      i32.and
      local.tee 8
      i32.add
      local.set 3
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          local.get 4
          i32.add
          local.tee 9
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          i32.const 1
          i32.lt_s
          br_if 1 (;@2;)
          local.get 9
          i32.const 3
          i32.shl
          local.tee 6
          i32.const 24
          i32.and
          local.set 2
          local.get 9
          i32.const -4
          i32.and
          local.tee 10
          i32.const 4
          i32.add
          local.set 1
          i32.const 0
          local.get 6
          i32.sub
          i32.const 24
          i32.and
          local.set 4
          local.get 10
          i32.load
          local.set 6
          loop  ;; label = @4
            local.get 5
            local.get 6
            local.get 2
            i32.shr_u
            local.get 1
            i32.load
            local.tee 6
            local.get 4
            i32.shl
            i32.or
            i32.store
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 5
            i32.const 4
            i32.add
            local.tee 5
            local.get 3
            i32.lt_u
            br_if 0 (;@4;)
            br 2 (;@2;)
          end
        end
        local.get 8
        i32.const 1
        i32.lt_s
        br_if 0 (;@2;)
        local.get 9
        local.set 1
        loop  ;; label = @3
          local.get 5
          local.get 1
          i32.load
          i32.store
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 5
          i32.const 4
          i32.add
          local.tee 5
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 7
      i32.const 3
      i32.and
      local.set 2
      local.get 9
      local.get 8
      i32.add
      local.set 1
    end
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      local.get 2
      i32.add
      local.set 5
      loop  ;; label = @2
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 3
        i32.const 1
        i32.add
        local.tee 3
        local.get 5
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func $memcpy (type 9) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call $_ZN17compiler_builtins3mem6memcpy17h429fa537573077b3E)
  (table (;0;) 3 3 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1049340))
  (global (;2;) i32 (i32.const 1049344))
  (export "memory" (memory 0))
  (export "add2num" (func $add2num))
  (export "__wbindgen_describe_add2num" (func $__wbindgen_describe_add2num))
  (export "__wbindgen_exn_store" (func $__wbindgen_exn_store))
  (export "__wbindgen_malloc" (func $__wbindgen_malloc))
  (export "__wbindgen_realloc" (func $__wbindgen_realloc))
  (export "__wbindgen_free" (func $__wbindgen_free))
  (export "__externref_table_alloc" (func $__externref_table_alloc))
  (export "__externref_table_dealloc" (func $__externref_table_dealloc))
  (export "__externref_drop_slice" (func $__externref_drop_slice))
  (export "__externref_heap_live_count" (func $__externref_heap_live_count))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func $_ZN4core3ptr37drop_in_place$LT$core..fmt..Error$GT$17h36b1182ddd7549f3E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h764d0138e2733e60E)
  (data $.rodata (i32.const 1048576) "called `Option::unwrap()` on a `None` value\00\01\00\00\00\00\00\00\00\01\00\00\00\02\00\00\00/rust/deps/dlmalloc-0.2.6/src/dlmalloc.rsassertion failed: psize >= size + min_overhead\00<\00\10\00)\00\00\00\a8\04\00\00\09\00\00\00assertion failed: psize <= size + max_overhead\00\00<\00\10\00)\00\00\00\ae\04\00\00\0d\00\00\00library/std/src/panicking.rs\e4\00\10\00\1c\00\00\00\84\02\00\00\1e\00\00\00"))
