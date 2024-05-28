use std::collections::HashMap;

pub struct MemSet{
    begin: u32,
    end: u32,
}
pub struct MemoryManager{
    total_begin: u32,
    total_end: u32,
    memset: HashMap<u32,MemSet>,
    can_alloc_mem: HashMap<u32,MemSet>,
    mem_set_descriptor: u32,
    identifier: u32,
}
impl MemoryManager{
    pub fn new(begin: u32,end: u32) -> Self{
        Self{
            total_begin: begin,
            total_end: end,
            memset: HashMap::new(),
            can_alloc_mem: HashMap::new(),
            mem_set_descriptor: 0,
            identifier: 0,
        }
    }

    pub fn init(&mut self){
        self.identifier+=1;
        let temp_memset = MemSet{
            begin: self.total_begin,
            end: self.total_end,
        };
        self.can_alloc_mem.insert(self.identifier,temp_memset);
    }

    pub fn alloc(&mut self,len: u32) -> u32{
        if len > (self.total_end-self.total_begin){
            return 0;
        }

        self.can_alloc_mem.sort_by(|a,b|a.begin.cmp(&b.begin));

        for (identifier,mem) in self.can_alloc_mem.iter(){
            if (mem.end-mem.begin+1)>=len{
                let new_begin = mem.begin+len;
                let temp_memset = MemSet{
                    begin: mem.begin,
                    end: new_begin-1,
                };
                // 将temp_memset加入memset，从can_alloc_mem移除，若
                self.mem_set_descriptor+=1;
                self.memset.insert(self.mem_set_descriptor,temp_memset);
                // 若将内存非配完了
                if mem.end-mem.begin+1 == len{
                    self.can_alloc_mem.remove(identifier);
                }else{
                    // 若该mem内存没有被分配完
                    let temp_memset = self.can_alloc_mem.get_mut(identifier);
                    temp_memset.unwrap().begin = new_begin;
                }
                return self.mem_set_descriptor
            }
        }
        // 返回0表示这次分配失败，不会输出
        return 0;
    }

    pub fn erase(&mut self,fd: u32){
        let mut find_fd = false;
        for (alloc_fd,mem) in self.memset.iter(){
            if (*alloc_fd==fd){
                find_fd = true;
                // 将需要擦除的块，加入can_alloc_mem
                let temp_mem = MemSet{
                    begin: mem.begin,
                    end: mem.end,
                };
                self.identifier+=1;
                self.can_alloc_mem.insert(self.identifier,temp_mem);
                self.memset.remove(alloc_fd);
            }
        }
    }
}

fn main() {

}