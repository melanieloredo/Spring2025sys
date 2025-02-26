use std::arch::asm;

fn main() {
    let message = b"Hello, direct syscall!\n"; //printing in terms of bytes 'b'

    unsafe {
        // write syscall
        asm!(
            "mov rax, 1",  // syscall number for write //trap table values
            "mov rdi, 1",  // file descriptor: 1 is stdout //rdi explicitly 
            "syscall", //transfer control 
            in("rsi") message.as_ptr(), //provide message
            in("rdx") message.len(), //provide size
            //output
            out("rax") _,
            out("rcx") _,
            out("r11") _,
            clobber_abi("system")
        );

        // exit syscall
        asm!(
            "mov rax, 60", // syscall number for exit ;; 60 for sys_exit
            "xor rdi, rdi", // status code 0
            "syscall",
            options(noreturn)
        );
    }
}