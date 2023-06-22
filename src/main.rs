fn main(){
    let i:i16=0x7fff;
    println!("0x{:x}", i);
    let i:i16=-32767;
    println!("0x{:x}", i);
    let i:i16=32767;
    println!("0x{:x}", i);

    let i:i16=-32768;
    println!("0x{:x}", i);
    
}