use crate::String;
use crate::format;
use alloc::string::ToString;
use cortex_m::asm::delay;
use kampela_system::in_free;

//           ____ _     ___  ____    _    _     ____
//          / ___| |   / _ \| __ )  / \  | |   / ___|
//         | |  _| |  | | | |  _ \ / _ \ | |   \___ \
//         | |_| | |__| |_| | |_) / ___ \| |___ ___) |
//          \____|_____\___/|____/_/   \_\_____|____/
//

pub static mut LOOP_COUNTER: u32 = 0;

const PRNBUF_SIZE: u16 = 1024;
pub static mut PRNBUF: &'static mut [u8] = &mut [0; PRNBUF_SIZE as usize];
pub static mut PRNBUF_I: u16 = 0;
pub static mut PRNBUF_R: u16 = 0;

// put a String to output uart-buffer
pub fn prn(s: String) {
    for c in s.as_bytes() { prnl_in(*c); }
}

// put a String + \n to output uart-buffer
pub fn prnn(s: String) {
    for c in s.as_bytes() { write_debug_eusart(*c); }
}

// print a String directly to uart
pub fn prnln(s: String) {
    prn(s);
    prn("\n".to_string()); // write_debug_eusart(b'\n');
}

// print a Char directly to uart
pub fn write_debug_eusart(c: u8) {
    unsafe {
     in_free(|peripherals| {
        while peripherals.EUSART1_S.status.read().txfl().bit_is_clear() {
            delay(10);
        }
        peripherals.EUSART1_S.txdata.write(|w_reg| w_reg.txdata().bits(c.into()));
     });
    }
}

// print all chars in uart-buffer (if exist) to uart
pub fn uartloop() {
    unsafe {
        let i = PRNBUF_I;
        let mut r = PRNBUF_R;
        while r != i {
            write_debug_eusart(PRNBUF[r as usize]);
            r += 1; if r >= PRNBUF_SIZE { r = 0; }
            PRNBUF_R = r;
        }
    }
}

// add 1 byte to PRNBUF '±'(177) in the end means buffer is overflooded, some chars was lost
pub fn prnl_in(c:u8) {
    unsafe {
        let mut i:u16 = PRNBUF_I;
        i+=1; if i >= PRNBUF_SIZE { i = 0; }
        if i != PRNBUF_R {
             PRNBUF[i as usize] = c;
             PRNBUF_I = i;
        } else {
             i = PRNBUF_I;
             if i > 0 { i-=1; } else { i = PRNBUF_SIZE-1; }
             PRNBUF[i as usize] = 177;
        }
    }
}
